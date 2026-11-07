//! La clausola nazionale per l'energia (shock energetico): vale solo dopo `clausola_nazionale_fino_a`.
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

#[test]
fn dentro_la_clausola_nazionale_l_energia_non_conta_due_volte() {
    let senza = deviazione_2027(2028, 0);
    let con = deviazione_2027(2028, 2027);
    assert_eq!(senza, con);
}

#[test]
fn l_opzione_clausola_ricorda_l_anno_della_chiusura() {
    let p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let dati = comune::dati_base();
    let evento = dati.evento("shock_energia").unwrap();
    let mut p = edita(&p, |j| {
        j["eventi"]["aperti"] = json!([{
            "evento": evento,
            "turno": 0,
            "causa": { "tipo": "shock", "serie": "prezzi dell'energia", "sigma": 3.0 },
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
    let anno = p.vista().data.anno;
    p.chiudi_turno();
    assert_eq!(p.economia().regole_ue.clausola_energia_fino_a, anno);
}
