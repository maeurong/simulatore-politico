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
    assert_eq!(vista["turno"], 8);
    assert_eq!(vista["data"]["anno"], 2028);
    assert_eq!(vista["data"]["trimestre"], 4);
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
