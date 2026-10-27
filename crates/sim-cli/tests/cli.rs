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
fn posto_fonda_la_partita_vera() {
    let out = sim(&[
        "--dati", DATI, "--seme", "7", "--turni", "1", "--posto", "4.5",
    ]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let vista: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(vista["vista"]["giocatore"], "giocatore");
}

#[test]
fn posto_non_ammesso_esce_con_errore() {
    rifiuta(
        &[
            "--dati", DATI, "--seme", "7", "--turni", "1", "--posto", "4",
        ],
        "posto 4",
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

fn tara_in(cartella: &std::path::Path, bersagli: &str) -> std::process::Output {
    sim(&[
        "tara",
        "--dati",
        cartella.to_str().unwrap(),
        "--bersagli",
        bersagli,
    ])
}

/// Oracolo indipendente: ricarica i file riscritti e rifa' i conti dalle intenzioni per gruppo.
fn controlla_bersagli(cartella: &std::path::Path) {
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
    // Raggruppa per il campo `regione` del toml, non per posizione.
    let elettorato: toml::Table = std::fs::read_to_string(cartella.join("elettorato.toml"))
        .unwrap()
        .parse()
        .unwrap();
    let mut per_regione: std::collections::BTreeMap<&str, (f64, f64)> = Default::default();
    for (g, riga) in elettorato["gruppo"].as_array().unwrap().iter().enumerate() {
        let id = riga["regione"].as_str().unwrap();
        let q = partita.elettorato().gruppi[g].quota;
        let e = per_regione.entry(id).or_default();
        e.0 += q;
        e.1 += q * int.per_gruppo[g].last().unwrap();
    }
    assert_eq!(per_regione.len(), ids.len());
    for (id, (peso, astenuti)) in per_regione {
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
}

fn leggi(cartella: &std::path::Path, file: &str) -> String {
    std::fs::read_to_string(cartella.join(file)).unwrap()
}

/// Parte fuori dal punto fisso: valence a 0 e astensione a 0 in ogni regione.
fn scosta(cartella: &std::path::Path) {
    let mut dentro = false;
    let scenario: String = leggi(cartella, "scenario.toml")
        .split_inclusive('\n')
        .map(|r| {
            if r.starts_with('[') {
                dentro = r.trim_end() == "[partito.valence]";
            }
            if dentro && r.starts_with("valore = ") {
                "valore = 0.0\n".to_string()
            } else {
                r.to_string()
            }
        })
        .collect();
    std::fs::write(cartella.join("scenario.toml"), scenario).unwrap();
    let elettorato: String = leggi(cartella, "elettorato.toml")
        .split_inclusive('\n')
        .map(|r| {
            if r.starts_with("astensione = ") {
                "astensione = 0.0\n".to_string()
            } else {
                r.to_string()
            }
        })
        .collect();
    std::fs::write(cartella.join("elettorato.toml"), elettorato).unwrap();
}

#[test]
fn dati_committati_sono_gia_tarati() {
    let cartella = copia_dati("ok");
    let out = tara_in(&cartella, BERSAGLI);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["giri"], 0, "rilancia `sim-cli tara` su dati/base");
    assert!(v["scarto"].as_f64().unwrap() < 1e-4);
    controlla_bersagli(&cartella);
    std::fs::remove_dir_all(cartella).unwrap();
}

#[test]
fn tara_converge_da_dati_scostati_e_tocca_solo_valence_e_astensione() {
    let cartella = copia_dati("scostati");
    scosta(&cartella);
    let prima = [
        leggi(&cartella, "scenario.toml"),
        leggi(&cartella, "elettorato.toml"),
    ];
    let out = tara_in(&cartella, BERSAGLI);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    controlla_bersagli(&cartella);

    // Solo le righe `valore =` (7) e `astensione =` (20) cambiano; nessun .tmp resta.
    for (file, prefisso, massimo, i) in [
        ("scenario.toml", "valore = ", 7, 0),
        ("elettorato.toml", "astensione = ", 20, 1),
    ] {
        let dopo = leggi(&cartella, file);
        assert_eq!(dopo.lines().count(), prima[i].lines().count(), "{file}");
        let diverse: Vec<_> = prima[i]
            .lines()
            .zip(dopo.lines())
            .filter(|(a, b)| a != b)
            .collect();
        assert!(!diverse.is_empty() && diverse.len() <= massimo, "{file}");
        assert!(
            diverse
                .iter()
                .all(|(a, b)| a.starts_with(prefisso) && b.starts_with(prefisso)),
            "{diverse:?}"
        );
    }
    assert!(
        std::fs::read_dir(&cartella).unwrap().all(|e| !e
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp"))
    );

    // Uscita JSON.
    let v: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert!(v["giri"].as_u64().unwrap() > 0);
    let quote: Vec<f64> = v["quote"]
        .as_array()
        .unwrap()
        .iter()
        .map(|q| q.as_f64().unwrap())
        .collect();
    assert_eq!(quote.len(), 7);
    assert!((quote.iter().sum::<f64>() - 1.0).abs() < 1e-9);
    let valence = v["valence"].as_array().unwrap();
    let ids: Vec<&str> = valence.iter().map(|x| x[0].as_str().unwrap()).collect();
    assert!(
        valence
            .iter()
            .all(|x| (-3.0..=3.0).contains(&x[1].as_f64().unwrap()))
    );
    let coalizioni = v["coalizioni"].as_array().unwrap();
    assert_eq!(coalizioni.len(), 2);
    for c in coalizioni {
        let attesa: f64 = c["partiti"]
            .as_array()
            .unwrap()
            .iter()
            .map(|id| quote[ids.iter().position(|i| *i == id.as_str().unwrap()).unwrap()])
            .sum();
        assert!((c["quota"].as_f64().unwrap() - attesa).abs() < 1e-12);
    }
    let affluenza = v["affluenza_nazionale"].as_f64().unwrap();
    assert!((0.5..0.8).contains(&affluenza), "{affluenza}");
    std::fs::remove_dir_all(cartella).unwrap();
}

fn tara_con(nome: &str, da: &str, a: &str) -> (std::path::PathBuf, std::process::Output) {
    let cartella = copia_dati(nome);
    let bersagli = cartella.join("bersagli.toml");
    std::fs::write(
        &bersagli,
        std::fs::read_to_string(BERSAGLI).unwrap().replace(da, a),
    )
    .unwrap();
    let out = sim(&[
        "tara",
        "--dati",
        cartella.to_str().unwrap(),
        "--bersagli",
        bersagli.to_str().unwrap(),
    ]);
    (cartella, out)
}

#[test]
fn tara_con_valence_fuori_intervallo_esce_senza_scrivere() {
    let (cartella, out) = tara_con(
        "fuori",
        "posti = [3.165, 36.015, 7.12, 5.02, 8.42, 33.0, 7.26]",
        "posti = [0.0001, 36.0, 7.12, 5.02, 8.42, 33.0, 10.4399]",
    );
    assert_eq!(out.status.code(), Some(1));
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains("valence di estrema_sinistra fuori da [-3, 3]"),
        "{err}"
    );
    for file in ["scenario.toml", "elettorato.toml"] {
        let prima = std::fs::read(format!("{DATI}/{file}")).unwrap();
        assert_eq!(std::fs::read(cartella.join(file)).unwrap(), prima, "{file}");
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
        "posto 1 = -3.165, serve una quota positiva",
    );
    std::fs::remove_dir_all(cartella).unwrap();
}

#[test]
fn tara_rifiuta_un_flag_sconosciuto_con_la_riga_d_uso() {
    rifiuta(
        &["tara", "--boh", "x"],
        "argomento sconosciuto: --boh (uso: sim-cli tara --dati <cartella> --bersagli <file>)",
    );
}

#[test]
fn tara_accetta_un_intero_come_affluenza() {
    let cartella = copia_dati("intero");
    let bersagli = cartella.join("bersagli.toml");
    std::fs::write(
        &bersagli,
        std::fs::read_to_string(BERSAGLI)
            .unwrap()
            .replace("piemonte = 66.1", "piemonte = 66"),
    )
    .unwrap();
    let out = tara_in(&cartella, bersagli.to_str().unwrap());
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    std::fs::remove_dir_all(cartella).unwrap();
}

#[test]
fn tara_rifiuta_una_coalizione_con_un_partito_sconosciuto() {
    let cartella = copia_dati("coalizione");
    let scenario = leggi(&cartella, "scenario.toml").replace(
        r#"["centrodestra", "destra"]]"#,
        r#"["centrodestra", "boh"]]"#,
    );
    std::fs::write(cartella.join("scenario.toml"), scenario).unwrap();
    rifiuta(
        &[
            "tara",
            "--dati",
            cartella.to_str().unwrap(),
            "--bersagli",
            BERSAGLI,
        ],
        "boh",
    );
    std::fs::remove_dir_all(cartella).unwrap();
}

#[test]
fn tara_conta_le_righe_astensione_scritte_senza_spazi() {
    let cartella = copia_dati("senza-spazi");
    let elettorato = leggi(&cartella, "elettorato.toml");
    let riga = elettorato
        .lines()
        .find(|r| r.starts_with("astensione = "))
        .unwrap();
    std::fs::write(
        cartella.join("elettorato.toml"),
        elettorato.replacen(riga, "astensione=0.3", 1),
    )
    .unwrap();
    rifiuta(
        &[
            "tara",
            "--dati",
            cartella.to_str().unwrap(),
            "--bersagli",
            BERSAGLI,
        ],
        "righe",
    );
    std::fs::remove_dir_all(cartella).unwrap();
}
