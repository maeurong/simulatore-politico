mod comune;

use comune::{al_governo, all_opposizione, chiudi_col_bilancio, dati_base, edita, fondata};
use sim_core::esito::{Criticita, MASSIMO_IN_EVIDENZA};
use sim_core::{Errore, Partita};

#[test]
fn al_turno_zero_l_esito_e_vuoto() {
    assert!(Partita::nuova(dati_base(), 7).vista().esito.is_empty());
    assert!(fondata(4.5).vista().esito.is_empty());
}

#[test]
fn nessuna_causa_vuota_mai_piu_di_sette() {
    for mut p in [al_governo(), all_opposizione()] {
        for turno in 0..8 {
            chiudi_col_bilancio(&mut p);
            let esito = p.vista().esito;
            assert!(
                esito.iter().all(|v| !v.causa.is_empty()),
                "turno {turno}: {esito:?}"
            );
            assert!(
                (esito.iter()).all(|v| !(v.in_evidenza && v.criticita == Criticita::Ordinaria)),
                "turno {turno}: {esito:?}"
            );
            let in_evidenza = esito.iter().filter(|v| v.in_evidenza).count();
            assert!(
                in_evidenza <= MASSIMO_IN_EVIDENZA,
                "turno {turno}: {esito:?}"
            );
        }
    }
}

#[test]
fn stesso_seme_stesso_esito() {
    let (mut a, mut b) = (all_opposizione(), all_opposizione());
    let mut voci = 0;
    for _ in 0..6 {
        chiudi_col_bilancio(&mut a);
        chiudi_col_bilancio(&mut b);
        assert_eq!(a.vista().esito, b.vista().esito);
        voci += a.vista().esito.len();
    }
    assert!(
        voci > 0,
        "sei chiusure senza una voce: il confronto non prova nulla"
    );
}

#[test]
fn l_esito_sopravvive_al_salvataggio() {
    let mut p = all_opposizione();
    // Una chiusura con voci, per salvare un esito non vuoto.
    for _ in 0..8 {
        if !p.vista().esito.is_empty() {
            break;
        }
        chiudi_col_bilancio(&mut p);
    }
    assert!(!p.vista().esito.is_empty());
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(q.vista().esito, p.vista().esito);
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    assert_eq!(q.vista().esito, p.vista().esito);
}

#[test]
fn l_esito_manomesso_e_rifiutato() {
    let p = all_opposizione();
    assert!(!p.vista().esito.is_empty());
    let voce = serde_json::to_value(&p.vista().esito[0]).unwrap();
    let errore =
        |esito: serde_json::Value| edita(&p, |j| j["esito"] = esito).unwrap_err().to_string();
    let mut senza_causa = voce.clone();
    senza_causa["causa"] = "".into();
    assert!(errore(serde_json::json!([senza_causa])).contains("senza causa"));
    let mut in_evidenza = voce.clone();
    in_evidenza["in_evidenza"] = true.into();
    let otto = vec![in_evidenza; MASSIMO_IN_EVIDENZA + 1];
    assert!(errore(serde_json::json!(otto)).contains("in evidenza"));
    let mut sconosciuto = voce;
    sconosciuto["partito"] = "fantasma".into();
    assert!(errore(serde_json::json!([sconosciuto])).contains("fantasma"));
}

#[test]
fn schema_17_rifiutato() {
    let p = Partita::nuova(dati_base(), 7);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["versione_schema"] = 17.into();
    assert_eq!(
        Partita::carica(&json.to_string()),
        Err(Errore::SchemaDiverso {
            trovata: 17,
            attesa: 18
        })
    );
}
