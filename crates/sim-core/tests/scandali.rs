//! Scandali su ministri e leader IA (piano 9, task 7): costo in valence, dimissioni, bersaglio che
//! sparisce, leader senza scelta.

mod comune;

use comune::{
    al_governo, chiudi_col_bilancio, dati_base, edita, file_base, indice, partito, senza_emergenze,
    sostituisci,
};
use serde_json::{Value, json};
use sim_core::dati::DatiGioco;
use sim_core::eventi::Famiglia;
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Partita};

const MINISTRO: &str = "scandalo_ministro";
const LEADER: &str = "scandalo_leader";

/// `p` con uno scandalo `evento` aperto in testa, sul `bersaglio`, con la `scelta` data.
fn con_scandalo(p: &Partita, evento: &str, bersaglio: Value, scelta: Option<usize>) -> Partita {
    let k = dati_base().evento(evento).unwrap();
    edita(p, |j| {
        let prova = json!({
            "evento": k,
            "turno": p.vista().turno,
            "causa": { "tipo": "rischio", "livello": "medio" },
            "regione": null,
            "bersaglio": bersaglio,
            "tono": "ordinario",
            "costo": 0,
            "scelta": scelta,
        });
        j["eventi"]["aperti"]
            .as_array_mut()
            .unwrap()
            .insert(0, prova);
    })
    .unwrap()
}

fn copia(p: &Partita) -> Partita {
    edita(p, |_| {}).unwrap()
}

fn ministro(dicastero: usize) -> Value {
    json!({ "tipo": "ministro", "dicastero": dicastero })
}

fn leader(partito: usize) -> Value {
    json!({ "tipo": "leader", "partito": partito })
}

fn variazioni(p: &Partita, j: usize) -> usize {
    (p.intesa().variazioni.iter())
        .filter(|v| v.partito == j && v.causa == CausaIntesa::DimissioniChieste)
        .count()
}

#[test]
fn difenderlo_costa_valence() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    let mut senza = copia(&p);
    let mut con = con_scandalo(&p, MINISTRO, ministro(d), None);
    senza.chiudi_turno();
    con.chiudi_turno();
    // Il costo cade prima del ritorno della valence verso la base, che ne riprende un decimo.
    let costo =
        dati_base().manopola("valence_scandalo") * (1.0 - dati_base().manopola("ritorno_valence"));
    let (v0, v1) = (senza.partiti()[j].valence, con.partiti()[j].valence);
    assert!((v0 - v1 - costo).abs() < 1e-9, "{v0} contro {v1}");
    let m = con.ministri().dicasteri[d].ministro.as_ref().unwrap();
    assert_eq!(
        f64::from(m.sotto_scandalo),
        dati_base().manopola("turni_scandalo")
    );
    // Cala a ogni chiusura.
    con.chiudi_turno();
    let m = con.ministri().dicasteri[d].ministro.as_ref().unwrap();
    assert_eq!(m.sotto_scandalo, 3);
}

#[test]
fn le_dimissioni_seguono_la_regola_del_piano_5() {
    let cd = partito("centrodestra");
    let d = indice("ambiente_energia");
    let chiuso = |tecnico: bool| {
        let p = Partita::nuova(dati_base(), 1);
        let p = edita(&p, |j| {
            j["ministri"]["dicasteri"][d]["ministro"]["tecnico"] = json!(tecnico);
        })
        .unwrap();
        let nome = p.ministri().dicasteri[d]
            .ministro
            .as_ref()
            .unwrap()
            .nome
            .clone();
        let mut p = con_scandalo(&p, MINISTRO, ministro(d), Some(1));
        p.chiudi_turno();
        let ancora = p.ministri().dicasteri[d]
            .ministro
            .as_ref()
            .map(|m| m.nome.clone());
        assert_ne!(
            ancora,
            Some(nome),
            "tecnico {tecnico}: il ministro è ancora in carica"
        );
        p
    };
    assert_eq!(variazioni(&chiuso(false), cd), 1);
    assert_eq!(variazioni(&chiuso(true), cd), 0);
}

#[test]
fn mai_il_giocatore() {
    // Corruzione a 200 e scandalo_base a 1: a ogni chiusura nasce uno scandalo.
    let f = sostituisci(
        senza_emergenze(file_base()),
        "grandezze",
        "nome = \"Corruzione\"\ndinamica = \"fissa\"\nverso = \"scende\"\nregionale = false\niniziale = 100.0",
        "nome = \"Corruzione\"\ndinamica = \"fissa\"\nverso = \"scende\"\nregionale = false\niniziale = 200.0",
    );
    let f = sostituisci(
        f,
        "manopole",
        "[scandalo_base]\nvalore = 0.0",
        "[scandalo_base]\nvalore = 1.0",
    );
    let (mut ministri, mut leader_visti) = (0, 0);
    for seme in 1..=500 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        chiudi_col_bilancio(&mut p);
        let j: Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        let nati: Vec<&Value> = j["eventi"]["aperti"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["bersaglio"].is_object())
            .collect();
        assert_eq!(nati.len(), 1, "seme {seme}: uno scandalo a turno");
        match nati[0]["bersaglio"]["tipo"].as_str().unwrap() {
            "leader" => {
                leader_visti += 1;
                assert_ne!(
                    nati[0]["bersaglio"]["partito"],
                    p.giocatore(),
                    "seme {seme}"
                );
            }
            _ => ministri += 1,
        }
    }
    assert!(
        ministri > 0 && leader_visti > 0,
        "{ministri} e {leader_visti}"
    );
}

#[test]
fn bersaglio_sparito_nessun_panic() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    // Il ministro non c'è più, le dimissioni sono già state chieste,
    // nessuna chiusura fallisce, nessun effetto e nessuna riga nelle mosse.
    let sparito = edita(&p, |x| {
        x["ministri"]["dicasteri"][d]["ministro"] = Value::Null;
        x["ministri"]["dicasteri"][d]["agenda"] = Value::Null;
        x["ministri"]["dicasteri"][d]["proposta"] = Value::Null;
    })
    .unwrap();
    let chieste = edita(&p, |x| {
        x["ministri"]["dicasteri"][d]["dimissioni_chieste"] = json!(true);
    })
    .unwrap();
    let casi = [
        (sparito, MINISTRO, ministro(d), None),
        (chieste, MINISTRO, ministro(d), Some(1)),
    ];
    for (base, evento, bersaglio, scelta) in casi {
        let mut senza = copia(&base);
        let mut con = con_scandalo(&base, evento, bersaglio, scelta);
        senza.chiudi_turno();
        con.chiudi_turno();
        assert_eq!(con.vista().mosse, senza.vista().mosse, "{evento}");
        assert!((con.partiti()[j].valence - senza.partiti()[j].valence).abs() < 1e-9);
    }
}

#[test]
fn scandalo_del_leader_ia_senza_scelta() {
    let p = al_governo();
    let j = (0..p.partiti().len())
        .find(|&k| k != p.giocatore())
        .unwrap();
    let mut p = con_scandalo(&p, LEADER, leader(j), None);
    let voce = &p.vista().eventi[0];
    assert_eq!(voce.nome, "Scandalo su un leader");
    assert_eq!(voce.opzioni.len(), 1);
    let rispondi = |p: &mut Partita, opzione: &str| {
        p.comanda(Comando::RispondiEvento {
            evento: 0,
            opzione: opzione.into(),
        })
    };
    assert!(rispondi(&mut p, "difenderlo").is_err());
    rispondi(&mut p, &voce.opzioni[0].id.clone()).unwrap();
}

#[test]
fn senza_scandalo_base_nessuno_scandalo() {
    let f = senza_emergenze(file_base());
    for seme in 1..=50 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        chiudi_col_bilancio(&mut p);
        let eventi = p.vista().eventi;
        let scandali = eventi.iter().filter(|e| e.famiglia == Famiglia::Scandalo);
        assert_eq!(scandali.count(), 0, "seme {seme}");
    }
}
