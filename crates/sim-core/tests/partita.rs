#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base};
use sim_core::{DatiGioco, Errore, FileDati, Partita, Trimestre, VERSIONE_SCHEMA};

fn giocata(seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(dati_base(), seme);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

#[test]
fn turno_zero_e_ottobre_dicembre_2026() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.turno, 0);
    assert_eq!(
        v.data,
        Trimestre {
            anno: 2026,
            trimestre: 4
        }
    );
    assert_eq!(v.dicasteri.len(), 10);
}

#[test]
fn chiudere_il_turno_avanza_di_un_trimestre() {
    assert_eq!(
        giocata(1, 1).vista().data,
        Trimestre {
            anno: 2027,
            trimestre: 1
        }
    );
    let v = giocata(1, 8).vista();
    assert_eq!(v.turno, 8);
    assert_eq!(
        v.data,
        Trimestre {
            anno: 2028,
            trimestre: 4
        }
    );
}

#[test]
fn stesso_seme_stessa_partita() {
    assert_eq!(
        giocata(42, 8).salva().unwrap(),
        giocata(42, 8).salva().unwrap()
    );
}

#[test]
fn salvare_e_ricaricare_non_cambia_il_seguito() {
    let mut diretta = giocata(42, 4);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    assert_eq!(ricaricata, diretta);
    for _ in 0..4 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
}

// Primo test che estrae dal Caso prima e dopo salva/carica (rimando della PR #15).
#[test]
fn estrazioni_identiche_dopo_salva_carica() {
    let mut diretta = giocata(42, 3);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    for _ in 0..5 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.economia(), diretta.economia());
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
    assert_ne!(
        giocata(42, 8).economia(),
        giocata(43, 8).economia(),
        "il caso deve essere stato estratto"
    );
}

#[test]
fn i_dati_viaggiano_nel_salvataggio() {
    let p = Partita::carica(&giocata(3, 1).salva().unwrap()).unwrap();
    assert!(p.vista().dicasteri.iter().any(|d| d.nome == "Sanità"));
}

#[test]
fn schema_diverso_rifiutato() {
    let mut json: serde_json::Value =
        serde_json::from_str(&giocata(1, 1).salva().unwrap()).unwrap();
    json["versione_schema"] = 99.into();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert_eq!(
        e,
        Errore::SchemaDiverso {
            trovata: 99,
            attesa: VERSIONE_SCHEMA
        }
    );
    assert_eq!(
        e.to_string(),
        "salvataggio di una versione diversa del gioco: schema 99, atteso 2"
    );
}

#[test]
fn salvataggio_troncato_non_va_in_panico() {
    let s = giocata(1, 2).salva().unwrap();
    let e = Partita::carica(&s[..s.floor_char_boundary(s.len() / 2)]).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)));
}

fn salvataggio_modificato(f: impl FnOnce(&mut serde_json::Value)) -> String {
    let mut json: serde_json::Value =
        serde_json::from_str(&giocata(1, 1).salva().unwrap()).unwrap();
    f(&mut json);
    json.to_string()
}

#[test]
fn trimestre_fuori_intervallo_rifiutato() {
    let s = salvataggio_modificato(|j| j["data"]["trimestre"] = 255.into());
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("255")));
}

#[test]
fn contatori_al_limite_non_vanno_in_panico() {
    let s = salvataggio_modificato(|j| {
        j["turno"] = u32::MAX.into();
        j["data"] = serde_json::json!({ "anno": u16::MAX, "trimestre": 4 });
    });
    let mut p = Partita::carica(&s).unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().turno, u32::MAX);
}

#[test]
fn f64_identici_attraverso_salva_carica() {
    let file = file_base();
    let manopole = file
        .manopole
        .replacen("valore = 40.0", "valore = 94.52706955539223", 1)
        .replacen("min = 30.0", "min = 0.0", 1)
        .replacen("max = 50.0", "max = 100.0", 1);
    let dati = DatiGioco::da_testi(&FileDati { manopole, ..file }).unwrap();
    let p = Partita::nuova(dati, 1);
    let r = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(r, p);
}

#[test]
fn manopola_nan_rifiutata_al_caricamento() {
    let file = file_base();
    let manopole = file.manopole.replacen("valore = 40.0", "valore = nan", 1);
    let e = DatiGioco::da_testi(&FileDati { manopole, ..file }).unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}

#[test]
fn il_salvataggio_non_ripete_il_seme() {
    let json: serde_json::Value = serde_json::from_str(&giocata(5, 1).salva().unwrap()).unwrap();
    assert!(json.get("seme").is_none());
    assert_eq!(json["caso"]["seme"], 5);
}

#[test]
fn dati_incoerenti_nel_salvataggio_rifiutati() {
    let s = salvataggio_modificato(|j| {
        for d in j["dati"]["dicasteri"].as_array_mut().unwrap() {
            d["ruoli"] = serde_json::json!([]);
        }
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(e, Errore::RuoloOrfano { .. }));
}

#[test]
fn versione_assente_vale_zero() {
    let s = salvataggio_modificato(|j| {
        j.as_object_mut().unwrap().remove("versione_schema");
    });
    let e = Partita::carica(&s).unwrap_err();
    assert_eq!(
        e,
        Errore::SchemaDiverso {
            trovata: 0,
            attesa: VERSIONE_SCHEMA
        }
    );
}

#[test]
fn versione_non_intera_rifiutata() {
    for sbagliata in [serde_json::json!("1"), serde_json::json!(1.5)] {
        let s = salvataggio_modificato(|j| j["versione_schema"] = sbagliata.clone());
        let e = Partita::carica(&s).unwrap_err();
        assert!(
            matches!(e, Errore::SchemaDiverso { trovata: 0, .. }),
            "{sbagliata}"
        );
    }
}

#[test]
fn economia_che_non_combacia_coi_dati_rifiutata() {
    let rifiutata = |campo: &str, f: fn(&mut Vec<serde_json::Value>)| {
        let s = salvataggio_modificato(|j| f(j["economia"][campo].as_array_mut().unwrap()));
        Partita::carica(&s).unwrap_err()
    };
    let e = rifiutata("regioni", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("economia.regioni")));
    let e = rifiutata("regioni", |v| v.swap(0, 1));
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("economia.regioni")));
    let e = rifiutata("gap_settori", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("gap_settori")));
    let e = rifiutata("prezzi_settori", |v| {
        v.pop();
    });
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("prezzi_settori")));
}

#[test]
fn campo_obbligatorio_mancante_rifiutato() {
    let s = salvataggio_modificato(|j| {
        j.as_object_mut().unwrap().remove("turno");
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(&e, Errore::SalvataggioIlleggibile(m) if m.contains("turno")));
}
