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
    assert_eq!(giocata(42, 8).salva(), giocata(42, 8).salva());
}

#[test]
fn salvare_e_ricaricare_non_cambia_il_seguito() {
    let mut diretta = giocata(42, 4);
    let mut ricaricata = Partita::carica(&diretta.salva()).unwrap();
    assert_eq!(ricaricata, diretta);
    for _ in 0..4 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.salva(), diretta.salva());
}

#[test]
fn i_dati_viaggiano_nel_salvataggio() {
    let p = Partita::carica(&giocata(3, 1).salva()).unwrap();
    assert!(p.vista().dicasteri.iter().any(|d| d.nome == "Sanità"));
}

#[test]
fn schema_diverso_rifiutato() {
    let mut json: serde_json::Value = serde_json::from_str(&giocata(1, 1).salva()).unwrap();
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
    let s = giocata(1, 2).salva();
    let e = Partita::carica(&s[..s.floor_char_boundary(s.len() / 2)]).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIllegibile(_)));
}
