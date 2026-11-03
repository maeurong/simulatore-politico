//! Eventi aperti: risposta, default alla chiusura, salvataggio.

mod comune;

use comune::{al_governo, all_opposizione, chiudi_col_bilancio, dati_base, edita};
use serde_json::json;
use sim_core::{Comando, Errore, Partita};

/// L'evento `criminalita` (indice nel catalogo) aperto al turno corrente, nella partita al governo:
/// default «ordinaria» (nessun effetto), «presidi» con un effetto su `sicurezza`.
fn con_evento_di_prova() -> Partita {
    con_evento(al_governo())
}

fn con_evento(p: Partita) -> Partita {
    let evento = dati_base().evento("criminalita").unwrap();
    edita(&p, |j| {
        // In testa (indice 0), davanti a quelli che il caso ha gia' aperto (le emergenze).
        let prova = json!({
            "evento": evento,
            "turno": p.vista().turno,
            "causa": { "tipo": "soglia", "misura": "sicurezza", "valore": 80.0, "soglia": 85.0 },
            "regione": null,
            "bersaglio": null,
            "tono": "ordinario",
            "costo": 0,
            "scelta": null,
        });
        j["eventi"]["aperti"]
            .as_array_mut()
            .unwrap()
            .insert(0, prova);
    })
    .unwrap()
}

type Guasto = fn(&mut serde_json::Value);

fn rispondi(p: &mut Partita, evento: usize, opzione: &str) -> Result<(), Errore> {
    p.comanda(Comando::RispondiEvento {
        evento,
        opzione: opzione.into(),
    })
}

/// Il valore di `sicurezza` nello stato vero (la vista lo mostra con un turno di ritardo).
fn sicurezza(p: &Partita) -> f64 {
    let k = dati_base().grandezza("sicurezza").unwrap();
    let j: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    j["grandezze"]["valori"][k][0].as_f64().unwrap()
}

#[test]
fn senza_scelta_vale_il_default() {
    let mut p = con_evento_di_prova();
    let mut q = al_governo();
    assert_eq!(p.vista().eventi[0].nome, "Ondata di criminalità");
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    // L'evento di prova e' chiuso; quelli nuovi li fa nascere il caso (le emergenze), uguali in p e q.
    assert!((p.vista().eventi.iter()).all(|e| e.nome != "Ondata di criminalità"));
    // Il default non ha effetti: la partita e' identica a quella senza evento.
    assert_eq!(p, q);
}

#[test]
fn la_scelta_applica_l_opzione() {
    let mut p = con_evento_di_prova();
    let mut q = con_evento_di_prova();
    rispondi(&mut p, 0, "presidi").unwrap();
    assert_eq!(p.vista().eventi[0].scelta, Some(1));
    for _ in 0..2 {
        chiudi_col_bilancio(&mut p);
        chiudi_col_bilancio(&mut q);
    }
    assert!(sicurezza(&p) > sicurezza(&q));
}

#[test]
fn la_scelta_si_puo_cambiare_fino_alla_chiusura() {
    let mut p = con_evento_di_prova();
    let mut q = con_evento_di_prova();
    rispondi(&mut p, 0, "presidi").unwrap();
    rispondi(&mut p, 0, "ordinaria").unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    assert_eq!(p, q);
}

#[test]
fn risposta_rifiutata_non_cambia_nulla() {
    let mut p = con_evento_di_prova();
    let prima = p.salva().unwrap();
    for (evento, opzione, messaggio) in [
        (9, "ordinaria", "Nessun evento aperto con numero 9"),
        (
            0,
            "boh",
            "Opzione sconosciuta per «Ondata di criminalità»: «boh»",
        ),
    ] {
        let e = rispondi(&mut p, evento, opzione).unwrap_err();
        assert!(e.to_string().contains(messaggio), "{e}");
        assert_eq!(p.salva().unwrap(), prima);
    }

    let mut o = con_evento(all_opposizione());
    let prima = o.salva().unwrap();
    let e = rispondi(&mut o, 0, "presidi").unwrap_err();
    assert!(
        e.to_string()
            .contains("Solo il governo risponde agli eventi"),
        "{e}"
    );
    assert_eq!(o.salva().unwrap(), prima);
}

#[test]
fn a_partita_finita_la_risposta_e_rifiutata() {
    let mut p = con_evento_di_prova();
    p.abbandona().unwrap();
    let prima = p.salva().unwrap();
    let e = rispondi(&mut p, 0, "presidi").unwrap_err();
    assert!(e.to_string().contains("La partita è finita"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn evento_aperto_sopravvive_al_salvataggio() {
    let mut p = con_evento_di_prova();
    rispondi(&mut p, 0, "presidi").unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    assert_eq!(p, q);
}

#[test]
fn salvataggio_con_eventi_incoerenti_rifiutato() {
    let p = con_evento_di_prova();
    let guasti: [(&str, Guasto); 6] = [
        ("evento", |j| j["eventi"]["aperti"][0]["evento"] = 99.into()),
        ("scelta", |j| j["eventi"]["aperti"][0]["scelta"] = 5.into()),
        ("regione", |j| {
            j["eventi"]["aperti"][0]["regione"] = 99.into();
        }),
        ("bersaglio", |j| {
            j["eventi"]["aperti"][0]["bersaglio"] = json!({ "tipo": "ministro", "dicastero": 99 });
        }),
        ("accese", |j| {
            j["eventi"]["accese"] = json!([]);
        }),
        ("contatori", |j| {
            j["eventi"]["contatori"] = json!([0]);
        }),
    ];
    for (nome, guasto) in guasti {
        let e = edita(&p, guasto).unwrap_err();
        assert!(
            matches!(e, Errore::SalvataggioIlleggibile(ref m) if m.contains("eventi")),
            "{nome}: {e:?}"
        );
    }
}

#[test]
fn la_scelta_non_sposta_le_estrazioni() {
    let mut p = con_evento_di_prova();
    let mut q = con_evento_di_prova();
    rispondi(&mut p, 0, "presidi").unwrap();
    rispondi(&mut q, 0, "ordinaria").unwrap();
    // «presidi» ha ritardo 1: serve piu' di una chiusura perche' la scelta cambi lo stato, e solo
    // allora un'estrazione che dipendesse dagli esiti si vedrebbe (i candidati delle emergenze
    // leggono lo stato).
    for _ in 0..3 {
        chiudi_col_bilancio(&mut p);
        chiudi_col_bilancio(&mut q);
    }
    assert!(sicurezza(&p) > sicurezza(&q));
    let (j, k): (serde_json::Value, serde_json::Value) = (
        serde_json::from_str(&p.salva().unwrap()).unwrap(),
        serde_json::from_str(&q.salva().unwrap()).unwrap(),
    );
    assert_eq!(j["caso"], k["caso"]);
    assert_eq!(j["eventi"], k["eventi"]);
}
