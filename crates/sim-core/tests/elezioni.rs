mod comune;

use comune::{dati_base, file_base, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::elezioni::{Formula, LeggeElettorale, Premio, QuoteTetto};
use sim_core::errore::Errore;

fn errore(f: FileDati) -> Errore {
    DatiGioco::da_testi(&f).unwrap_err()
}

fn legge_180() -> LeggeElettorale {
    dati_base().elezioni().impostazioni["legge_180"].clone()
}

#[test]
fn elezioni_base_si_caricano() {
    let d = dati_base();
    let e = d.elezioni();
    assert_eq!(e.camera.circoscrizioni.len(), 28);
    assert_eq!(e.senato.circoscrizioni.len(), 21);
    let chiavi: Vec<_> = e.impostazioni.keys().map(String::as_str).collect();
    assert_eq!(
        chiavi,
        ["legge_180", "proporzionale_3", "proporzionale_puro"]
    );
    assert_eq!(
        *d.legge_elettorale_iniziale(),
        LeggeElettorale {
            soglia_lista_pb: 300,
            coalizioni: true,
            soglia_coalizione_pb: 1000,
            ripescaggio: true,
            premio: Premio::Coalizione,
            soglia_premio_pb: 4200,
            taglia_premio_pb: 1750,
            tetto: Some(QuoteTetto {
                camera_pb: 5500,
                senato_pb: 5650
            }),
            formula: Formula::Hare,
        }
    );
}

#[test]
fn soglia_premio_sotto_il_40_rifiutata() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "soglia_premio_pb = [4000",
        "soglia_premio_pb = [3900",
    );
    let Errore::LeggeElettorale(m) = errore(f) else {
        panic!("atteso LeggeElettorale")
    };
    assert!(m.contains("soglia_premio_pb") && m.contains("3900"), "{m}");
}

#[test]
fn impostazione_fuori_dal_menu_rifiutata_col_nome() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "soglia_lista_pb = 300",
        "soglia_lista_pb = 250",
    );
    let Errore::LeggeElettorale(m) = errore(f) else {
        panic!("atteso LeggeElettorale")
    };
    assert!(
        m.contains("legge_180") && m.contains("soglia_lista_pb"),
        "{m}"
    );
}

#[test]
fn premio_alla_coalizione_senza_coalizioni_rifiutato() {
    let menu = dati_base().elezioni().menu.clone();
    let l = LeggeElettorale {
        coalizioni: false,
        ..legge_180()
    };
    assert!(matches!(l.valida(&menu), Err(Errore::LeggeElettorale(_))));
}

#[test]
fn premio_alla_lista_con_coalizioni_rifiutato() {
    let menu = dati_base().elezioni().menu.clone();
    assert_eq!(legge_180().valida(&menu), Ok(()));
    let ok = LeggeElettorale {
        premio: Premio::Lista,
        coalizioni: false,
        ..legge_180()
    };
    assert_eq!(ok.valida(&menu), Ok(()));
    let l = LeggeElettorale {
        premio: Premio::Lista,
        ..legge_180()
    };
    assert!(matches!(l.valida(&menu), Err(Errore::LeggeElettorale(_))));
}

#[test]
fn circoscrizione_con_regione_sconosciuta_rifiutata() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "regione = \"piemonte\"",
        "regione = \"piemontese\"",
    );
    let Errore::DatiElettorali(m) = errore(f) else {
        panic!("atteso DatiElettorali")
    };
    assert!(m.contains("piemontese"), "{m}");
}

#[test]
fn popolazioni_discordi_fra_i_rami_rifiutate() {
    let senato = file_base().elezioni;
    let i = senato.find("[senato]").unwrap();
    let (testa, coda) = senato.split_at(i);
    let coda = coda.replacen("292150", "292151", 1);
    let f = FileDati {
        elezioni: format!("{testa}{coda}"),
        ..file_base()
    };
    let Errore::DatiElettorali(m) = errore(f) else {
        panic!("atteso DatiElettorali")
    };
    assert!(m.contains("molise"), "{m}");
}

#[test]
fn scenario_con_legge_inesistente_rifiutato() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "legge_elettorale = \"legge_180\"",
        "legge_elettorale = \"legge_2030\"",
    );
    let Errore::LeggeElettorale(m) = errore(f) else {
        panic!("atteso LeggeElettorale")
    };
    assert!(m.contains("legge_2030"), "{m}");
}

#[test]
fn seggi_fissi_enormi_rifiutati_senza_panico() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "seggi_fissi = 2",
        "seggi_fissi = 4294967295",
    );
    assert!(matches!(errore(f), Errore::DatiElettorali(_)));
}

#[test]
fn tetto_del_menu_non_crescente_rifiutato() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "{ camera_pb = 6000, senato_pb = 6000 }",
        "{ camera_pb = 5500, senato_pb = 6000 }",
    );
    let Errore::DatiElettorali(m) = errore(f) else {
        panic!("atteso DatiElettorali")
    };
    assert!(m.contains("tetto"), "{m}");
}

#[test]
fn tetto_fuori_dal_menu_rifiutato_col_nome() {
    let f = sostituisci(
        file_base(),
        "elezioni",
        "tetto = { camera_pb = 5500, senato_pb = 5650 }",
        "tetto = { camera_pb = 5600, senato_pb = 5650 }",
    );
    let Errore::LeggeElettorale(m) = errore(f) else {
        panic!("atteso LeggeElettorale")
    };
    assert!(m.contains("legge_180") && m.contains("tetto"), "{m}");
}
