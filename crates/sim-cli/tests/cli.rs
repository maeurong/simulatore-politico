#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

use std::process::Command;

const DATI: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dati/base");

fn sim(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn gioca_otto_turni() {
    let out = sim(&["--dati", DATI, "--seme", "42", "--turni", "8"]);
    assert!(out.status.success());
    let vista: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(vista["vista"]["turno"], 8);
    assert_eq!(vista["vista"]["data"]["anno"], 2028);
    assert_eq!(vista["vista"]["data"]["trimestre"], 4);
    assert!(vista["vista"]["spread_pb"].is_i64());
    assert_eq!(vista["economia"]["regioni"].as_array().unwrap().len(), 20);
}

#[test]
fn stesso_seme_stesso_output() {
    let args = ["--dati", DATI, "--seme", "7", "--turni", "5"];
    assert_eq!(sim(&args).stdout, sim(&args).stdout);
}

#[test]
fn cartella_dati_mancante_esce_con_errore() {
    let out = sim(&["--dati", "/non/esiste", "--seme", "1", "--turni", "1"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("/non/esiste"));
}

#[test]
fn argomento_mancante_esce_con_errore() {
    let out = sim(&["--dati", DATI]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("errore: "));
}

fn rifiuta(args: &[&str], frammento: &str) {
    let out = sim(args);
    assert_eq!(out.status.code(), Some(1), "{args:?}");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.starts_with("errore: "), "{err}");
    assert!(err.contains(frammento), "{err}");
    assert!(!err.contains("panicked"), "{err}");
}

#[test]
fn seme_non_numerico_esce_con_errore() {
    rifiuta(
        &["--dati", DATI, "--seme", "abc", "--turni", "1"],
        "seme non valido: abc",
    );
}

#[test]
fn turni_non_numerici_escono_con_errore() {
    rifiuta(
        &["--dati", DATI, "--seme", "1", "--turni", "-3"],
        "numero di turni non valido: -3",
    );
}

#[test]
fn flag_sconosciuto_esce_con_errore() {
    rifiuta(
        &["--dati", DATI, "--seme", "1", "--turni", "1", "--boh", "x"],
        "argomento sconosciuto: --boh",
    );
}

#[test]
fn flag_senza_valore_esce_con_errore() {
    rifiuta(
        &["--dati", DATI, "--seme", "1", "--turni"],
        "manca il valore per --turni",
    );
}

#[test]
fn l_uscita_contiene_le_intenzioni_vere() {
    let out = sim(&["--dati", DATI, "--seme", "42", "--turni", "2"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["elettorato"]["partiti"].as_array().unwrap().len(), 7);
    let n = v["elettorato"]["nazionali"].as_array().unwrap();
    assert_eq!(n.len(), 8);
    let somma: f64 = n.iter().map(|x| x.as_f64().unwrap()).sum();
    assert!((somma - 1.0).abs() < 1e-9, "{somma}");
}

const BERSAGLI: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../strumenti/taratura/bersagli.toml"
);

fn copia_dati(nome: &str) -> std::path::PathBuf {
    let cartella = std::env::temp_dir().join(format!("tara-{}-{nome}", std::process::id()));
    std::fs::create_dir_all(&cartella).unwrap();
    for file in sim_core::FileDati::NOMI {
        std::fs::copy(format!("{DATI}/{file}"), cartella.join(file)).unwrap();
    }
    cartella
}

#[test]
fn tara_porta_il_turno_zero_sui_bersagli() {
    let cartella = copia_dati("ok");
    let out = sim(&[
        "tara",
        "--dati",
        cartella.to_str().unwrap(),
        "--bersagli",
        BERSAGLI,
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );

    // Oracolo indipendente: ricarica i file riscritti e rifà i conti dalle intenzioni per gruppo.
    let file = sim_core::FileDati::leggi(|n| std::fs::read_to_string(cartella.join(n))).unwrap();
    let dati = sim_core::DatiGioco::da_testi(&file).unwrap();
    let ids: Vec<String> = dati
        .elettorato()
        .regioni
        .iter()
        .map(|r| r.id.clone())
        .collect();
    let partita = sim_core::Partita::nuova(dati, 1);
    let int = partita.intenzioni_vere();
    let bersagli: toml::Table = std::fs::read_to_string(BERSAGLI).unwrap().parse().unwrap();
    let affluenza = bersagli["affluenza"].as_table().unwrap();
    let per_regione = int.per_gruppo.len() / ids.len();
    for (r, id) in ids.iter().enumerate() {
        let (mut peso, mut astenuti) = (0.0, 0.0);
        for g in r * per_regione..(r + 1) * per_regione {
            let q = partita.elettorato().gruppi[g].quota;
            peso += q;
            astenuti += q * int.per_gruppo[g].last().unwrap();
        }
        let atteso = affluenza[id].as_float().unwrap() / 100.0;
        assert!((1.0 - astenuti / peso - atteso).abs() < 1e-3, "{id}");
    }
    let votanti = 1.0 - int.nazionali.last().unwrap();
    for (j, p) in bersagli["posti"].as_array().unwrap().iter().enumerate() {
        let atteso = p.as_float().unwrap() / 100.0;
        assert!(
            (int.nazionali[j] / votanti - atteso).abs() < 1e-3,
            "posto {}",
            j + 1
        );
    }
    std::fs::remove_dir_all(cartella).unwrap();
}

#[test]
fn tara_rifiuta_un_bersaglio_impossibile() {
    let cartella = copia_dati("neg");
    let testo = std::fs::read_to_string(BERSAGLI)
        .unwrap()
        .replace("posti = [3.165,", "posti = [-3.165,");
    let bersagli = cartella.join("bersagli.toml");
    std::fs::write(&bersagli, testo).unwrap();
    rifiuta(
        &[
            "tara",
            "--dati",
            cartella.to_str().unwrap(),
            "--bersagli",
            bersagli.to_str().unwrap(),
        ],
        "bersaglio",
    );
    std::fs::remove_dir_all(cartella).unwrap();
}
