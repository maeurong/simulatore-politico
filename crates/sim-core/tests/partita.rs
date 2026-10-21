#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::dati_base;
use sim_core::{Errore, Partita, Trimestre, VERSIONE_SCHEMA};

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
        "salvataggio di una versione diversa del gioco: schema 99, atteso 1"
    );
}

#[test]
fn salvataggio_troncato_non_va_in_panico() {
    let s = giocata(1, 2).salva().unwrap();
    let e = Partita::carica(&s[..s.floor_char_boundary(s.len() / 2)]).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIllegibile(_)));
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
    assert!(matches!(&e, Errore::SalvataggioIllegibile(m) if m.contains("255")));
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
fn dati_incoerenti_nel_salvataggio_rifiutati() {
    let s = salvataggio_modificato(|j| {
        for d in j["dati"]["dicasteri"].as_array_mut().unwrap() {
            d["ruoli"] = serde_json::json!([]);
        }
    });
    let e = Partita::carica(&s).unwrap_err();
    assert!(matches!(e, Errore::RuoloOrfano { .. }));
}
