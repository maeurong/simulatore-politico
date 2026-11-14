//! Scandali su ministri e leader IA: costo in valence, dimissioni, bersaglio che
//! sparisce, leader senza scelta.

mod comune;

use comune::{
    al_governo, chiudi_col_bilancio, dati_base, edita, file_base, indice, partito, scandalo_certo,
    senza_emergenze, voci,
};
use serde_json::{Value, json};
use sim_core::dati::DatiGioco;
use sim_core::esito::{TipoVoce, VoceEsito};
use sim_core::eventi::Famiglia;
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Errore, Partita};

const MINISTRO: &str = "scandalo_ministro";
const LEADER: &str = "scandalo_leader";

/// `p` con uno scandalo `evento` aperto in testa, sul `bersaglio`, con la `scelta` data.
fn con_scandalo(p: &Partita, evento: &str, bersaglio: Value, scelta: Option<usize>) -> Partita {
    prova_scandalo(p, evento, bersaglio, scelta).unwrap()
}

fn prova_scandalo(
    p: &Partita,
    evento: &str,
    bersaglio: Value,
    scelta: Option<usize>,
) -> Result<Partita, Errore> {
    let k = dati_base().evento(evento).unwrap();
    // Chi occupa il posto alla nascita, come lo scrive il motore.
    let persona = match (
        bersaglio["tipo"].as_str(),
        bersaglio["dicastero"].as_u64(),
        bersaglio["partito"].as_u64(),
    ) {
        (Some("ministro"), Some(d), _) => (p.ministri().dicasteri.get(d as usize))
            .and_then(|st| st.ministro.as_ref())
            .map(|m| m.nome.clone()),
        (Some("leader"), _, Some(j)) => p.partiti().get(j as usize).map(|x| x.leader.clone()),
        _ => None,
    };
    edita(p, |j| {
        let prova = json!({
            "evento": k,
            "turno": p.vista().turno,
            "causa": { "tipo": "rischio", "livello": "medio" },
            "regione": null,
            "bersaglio": bersaglio,
            "persona": persona,
            "tono": "ordinario",
            "costo": 0,
            "scelta": scelta,
        });
        j["eventi"]["aperti"]
            .as_array_mut()
            .unwrap()
            .insert(0, prova);
    })
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

fn voce_dello_scandalo(p: &Partita) -> VoceEsito {
    (voci(p, TipoVoce::EventoRisolto).into_iter())
        .find(|v| v.fatto.starts_with("Scandalo"))
        .unwrap()
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
    let v = voce_dello_scandalo(&con);
    assert_eq!(v.famiglia, Some(Famiglia::Scandalo));
    assert_eq!(v.dicastero.as_deref(), Some("ambiente_energia"));
    assert_eq!(v.partito.as_deref(), Some(con.partiti()[j].id.as_str()));
    assert!(
        v.fatto.contains("«Difenderlo» per default") && v.fatto.contains(&m.nome),
        "{}",
        v.fatto
    );
    // Cala a ogni chiusura.
    con.chiudi_turno();
    let m = con.ministri().dicasteri[d].ministro.as_ref().unwrap();
    assert_eq!(m.sotto_scandalo, 3);
}

#[test]
fn le_dimissioni_costano_meta_della_difesa() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    let valence = |scelta: Option<usize>| {
        let mut x = con_scandalo(&p, MINISTRO, ministro(d), scelta);
        x.chiudi_turno();
        x.partiti()[j].valence
    };
    let mut senza = copia(&p);
    senza.chiudi_turno();
    let base = senza.partiti()[j].valence;
    // Il costo cade prima del ritorno della valence verso la base.
    let r = 1.0 - dati_base().manopola("ritorno_valence");
    let mut dimesso = con_scandalo(&p, MINISTRO, ministro(d), Some(1));
    dimesso.chiudi_turno();
    let v = voce_dello_scandalo(&dimesso);
    assert!(
        v.fatto.contains("«Chiedere le dimissioni»") && v.fatto.contains("si dimette"),
        "{}",
        v.fatto
    );
    let (difesa, dimissioni) = (valence(None), valence(Some(1)));
    assert!((base - difesa - 0.05 * r).abs() < 1e-9, "{base} {difesa}");
    assert!(
        (base - dimissioni - 0.025 * r).abs() < 1e-9,
        "{base} {dimissioni}"
    );
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
fn bersaglio_sparito_nessun_panic() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    // Il ministro non c'è più: nessuna chiusura fallisce, nessun effetto, lo stesso esito.
    let sparito = edita(&p, |x| {
        x["ministri"]["dicasteri"][d]["ministro"] = Value::Null;
        x["ministri"]["dicasteri"][d]["agenda"] = Value::Null;
        x["ministri"]["dicasteri"][d]["proposta"] = Value::Null;
    })
    .unwrap();
    let mut senza = copia(&sparito);
    let mut con = con_scandalo(&sparito, MINISTRO, ministro(d), None);
    senza.chiudi_turno();
    con.chiudi_turno();
    // Lo stesso esito, più la voce dello scandalo chiuso senza seguito.
    let v = voce_dello_scandalo(&con);
    assert!(v.fatto.ends_with("chiuso senza seguito"), "{}", v.fatto);
    // Il ripiego sul Presidente del Consiglio vale solo per la valence: la voce non ha partito.
    assert!(v.partito.is_none(), "{:?}", v.partito);
    let mut esito = con.vista().esito;
    esito.retain(|x| x != &v);
    assert_eq!(esito, senza.vista().esito);
    assert!((con.partiti()[j].valence - senza.partiti()[j].valence).abs() < 1e-9);
}

#[test]
fn le_dimissioni_gia_chieste_costano_come_quelle_date() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    let chieste = edita(&p, |x| {
        x["ministri"]["dicasteri"][d]["dimissioni_chieste"] = json!(true);
    })
    .unwrap();
    let mut senza = copia(&chieste);
    senza.chiudi_turno();
    let r = 1.0 - dati_base().manopola("ritorno_valence");
    // Con l'opzione «dimissioni» e col default: in entrambi i casi il costo è quello delle
    // dimissioni, e il ministro non resta sotto scandalo.
    for scelta in [Some(1), None] {
        let mut con = con_scandalo(&chieste, MINISTRO, ministro(d), scelta);
        con.chiudi_turno();
        let (v0, v1) = (senza.partiti()[j].valence, con.partiti()[j].valence);
        assert!((v0 - v1 - 0.025 * r).abs() < 1e-9, "{scelta:?}: {v0} {v1}");
        let sotto = (con.ministri().dicasteri[d].ministro.iter()).any(|m| m.sotto_scandalo > 0);
        assert!(!sotto, "{scelta:?}");
    }
}

#[test]
fn lo_scandalo_del_leader_costa_valence() {
    let p = Partita::nuova(dati_base(), 1);
    let j = (0..p.partiti().len())
        .find(|&k| k != p.giocatore() && !p.partiti()[k].sciolto)
        .unwrap();
    let mut senza = copia(&p);
    let mut con = con_scandalo(&p, LEADER, leader(j), None);
    senza.chiudi_turno();
    con.chiudi_turno();
    // Il costo cade prima del ritorno della valence verso la base.
    let costo =
        dati_base().manopola("valence_scandalo") * (1.0 - dati_base().manopola("ritorno_valence"));
    let (v0, v1) = (senza.partiti()[j].valence, con.partiti()[j].valence);
    assert!((v0 - v1 - costo).abs() < 1e-9, "{v0} contro {v1}");
}

#[test]
fn lo_scandalo_nasce_e_porta_la_persona() {
    let f = scandalo_certo(file_base());
    let dati = DatiGioco::da_testi(&f).unwrap();
    let (del_ministro, del_leader) = (dati.evento(MINISTRO).unwrap(), dati.evento(LEADER).unwrap());
    let (mut ministri, mut leader) = (0, 0);
    for seme in 1..=10 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme);
        for turno in 0..10 {
            chiudi_col_bilancio(&mut p);
            let n = (p.vista().eventi.iter())
                .filter(|e| e.famiglia == Famiglia::Scandalo)
                .count();
            assert_eq!(n, 1, "seme {seme}, turno {turno}");
            let j: Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
            let aperto = (j["eventi"]["aperti"].as_array().unwrap().iter())
                .find(|a| a["evento"] == del_ministro || a["evento"] == del_leader)
                .unwrap();
            assert!(aperto["persona"].is_string(), "seme {seme}: {aperto}");
            if aperto["evento"] == del_ministro {
                ministri += 1;
            } else {
                leader += 1;
            }
        }
    }
    assert!(ministri > 0 && leader > 0, "{ministri} e {leader}");
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
    rispondi(&mut p, &voce.opzioni[0].id).unwrap();
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

#[test]
fn ministro_sostituito_fra_nascita_e_chiusura_nessun_effetto() {
    let p = Partita::nuova(dati_base(), 1);
    let d = indice("ambiente_energia");
    let j = p.ministri().dicasteri[d].ministro.as_ref().unwrap().partito;
    let cambia = |x: &mut Value| {
        x["ministri"]["dicasteri"][d]["ministro"]["nome"] = json!("Nuovo Arrivato");
    };
    let mut con = edita(&con_scandalo(&p, MINISTRO, ministro(d), None), cambia).unwrap();
    let mut senza = edita(&p, cambia).unwrap();
    con.chiudi_turno();
    senza.chiudi_turno();
    assert!((con.partiti()[j].valence - senza.partiti()[j].valence).abs() < 1e-9);
    let m = con.ministri().dicasteri[d].ministro.as_ref().unwrap();
    assert_eq!(m.sotto_scandalo, 0);
    // Il posto è di un altro: la voce non lo attribuisce al suo partito.
    assert!(voce_dello_scandalo(&con).partito.is_none());
}

#[test]
fn un_salvataggio_con_bersaglio_o_evento_fuori_dai_dati_e_rifiutato() {
    let p = Partita::nuova(dati_base(), 1);
    let troppo = 1000;
    assert!(prova_scandalo(&p, MINISTRO, ministro(troppo), None).is_err());
    assert!(prova_scandalo(&p, LEADER, leader(troppo), None).is_err());
    assert!(prova_scandalo(&p, MINISTRO, ministro(0), None).is_ok());
    for coda in ["aperti", "lievi"] {
        let rotto = edita(&p, |j| {
            j["eventi"][coda].as_array_mut().unwrap().push(json!({
                "evento": 10_000,
                "turno": 0,
                "causa": { "tipo": "rischio", "livello": "medio" },
                "regione": null,
                "bersaglio": null,
                "tono": "ordinario",
                "costo": 0,
                "scelta": null,
            }));
        });
        assert!(rotto.is_err(), "{coda}");
    }
}
