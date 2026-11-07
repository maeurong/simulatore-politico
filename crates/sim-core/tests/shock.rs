//! Il margine UE per l'energia (shock energetico): vale solo dopo `clausola_nazionale_fino_a`.
//! Candidati e frequenza degli shock stanno in `eventi/rischio.rs`: `candidati_shock` è interna.

mod comune;

use comune::{edita, senza_shock, sostituisci};
use serde_json::json;
use sim_core::{Comando, DatiGioco, Partita};

fn deviazione_2027(fino_a_nazionale: u16, energia_fino_a: u16) -> f64 {
    let f = sostituisci(
        senza_shock(),
        "scenario",
        "clausola_nazionale_fino_a = 2028",
        &format!("clausola_nazionale_fino_a = {fino_a_nazionale}"),
    );
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..6 {
        p.chiudi_turno();
    }
    let mut p = edita(&p, |j| {
        j["economia"]["regole_ue"]["clausola_energia_fino_a"] = json!(energia_fino_a);
    })
    .unwrap();
    // Il controllo sull'anno 2027 gira chiudendo il settimo turno.
    p.chiudi_turno();
    p.economia().regole_ue.ultima_deviazione
}

#[test]
fn la_clausola_per_l_energia_allarga_il_margine() {
    let senza = deviazione_2027(2026, 0);
    let con = deviazione_2027(2026, 2027);
    assert!((senza - con - 0.3).abs() < 1e-9, "{senza} {con}");
}

/// Lo shock dell'energia aperto al turno zero (2026) con `clausola_nazionale_fino_a` a
/// `fino_a_nazionale`, la risposta «clausola» e una chiusura.
fn clausola_scelta(fino_a_nazionale: u16) -> Partita {
    let f = sostituisci(
        senza_shock(),
        "scenario",
        "clausola_nazionale_fino_a = 2028",
        &format!("clausola_nazionale_fino_a = {fino_a_nazionale}"),
    );
    let p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let evento = comune::dati_base().evento("shock_energia").unwrap();
    let mut p = edita(&p, |j| {
        j["eventi"]["aperti"] = json!([{
            "evento": evento,
            "turno": 0,
            "causa": { "tipo": "shock", "serie": "prezzi dell'energia", "sigma": 3.0, "soglia": 2.5 },
            "regione": null,
            "bersaglio": null,
            "tono": "ordinario",
            "costo": 0,
            "scelta": null,
        }]);
    })
    .unwrap();
    p.comanda(Comando::RispondiEvento {
        evento: 0,
        opzione: "clausola".into(),
    })
    .unwrap();
    p.chiudi_turno();
    p
}

#[test]
fn l_opzione_clausola_ricorda_l_anno_della_chiusura() {
    let p = clausola_scelta(2025);
    assert_eq!(p.economia().regole_ue.clausola_energia_fino_a, 2026);
}

/// Dentro la clausola nazionale l'energia è già coperta: l'opzione non segna nulla, e un
/// salvataggio che la segna è rifiutato.
#[test]
fn dentro_la_clausola_nazionale_l_energia_non_conta_due_volte() {
    let p = clausola_scelta(2028);
    assert_eq!(p.economia().regole_ue.clausola_energia_fino_a, 0);
    let m = rifiutato(|j| j["economia"]["regole_ue"]["clausola_energia_fino_a"] = json!(2026));
    assert!(m.contains("né 0 né dopo clausola_nazionale_fino_a"), "{m}");
}

fn rifiutato(guasto: impl FnOnce(&mut serde_json::Value)) -> String {
    let p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let mut j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    guasto(&mut j);
    Partita::carica(&j.to_string()).unwrap_err().to_string()
}

#[test]
fn un_salvataggio_con_la_clausola_nel_futuro_e_rifiutato() {
    let m = rifiutato(|j| j["economia"]["regole_ue"]["clausola_energia_fino_a"] = json!(65535));
    assert!(m.contains("clausola_energia_fino_a"), "{m}");
}
