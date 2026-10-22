mod comune;

use comune::{dati_base, file_base, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::elezioni::{
    Formula, LeggeElettorale, Premio, QuoteTetto, Ramo, SeggiCircoscrizione, seggi_da_quota,
    seggi_per_circoscrizione,
};
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

fn seggi(v: &[SeggiCircoscrizione]) -> Vec<u32> {
    v.iter().map(|s| s.seggi).collect()
}

fn premi(v: &[SeggiCircoscrizione]) -> Vec<u32> {
    v.iter().map(|s| s.premio).collect()
}

/// Somma dei plurinominali (`seggi - premio`) sulle circoscrizioni ordinarie dei dati.
fn plurinominali(r: &Ramo, v: &[SeggiCircoscrizione]) -> u32 {
    r.circoscrizioni
        .iter()
        .zip(v)
        .filter(|(c, _)| c.uninominali == 0)
        .map(|(_, s)| s.seggi - s.premio)
        .sum()
}

#[test]
fn camera_dal_censimento_2021_come_il_dossier() {
    let d = dati_base();
    let v = seggi_per_circoscrizione(&d.elezioni().camera, 70);
    assert_eq!(
        seggi(&v),
        [
            15, 14, 27, 14, 15, 11, 13, 19, 8, 10, 29, 24, 6, 10, 25, 13, 8, 2, 20, 17, 26, 4, 12,
            15, 17, 10, 1, 7
        ]
    );
}

#[test]
fn premio_della_camera_come_il_dossier() {
    let d = dati_base();
    let r = &d.elezioni().camera;
    let v = seggi_per_circoscrizione(r, 70);
    assert_eq!(
        premi(&v),
        [
            3, 2, 5, 2, 3, 2, 2, 4, 1, 2, 5, 4, 1, 2, 5, 2, 2, 0, 4, 3, 5, 1, 2, 3, 3, 2, 0, 0
        ]
    );
    assert_eq!(plurinominali(r, &v), 314);
    assert_eq!(plurinominali(r, &seggi_per_circoscrizione(r, 0)), 384);
}

#[test]
fn senato_dal_censimento_2021_come_il_dossier() {
    let d = dati_base();
    let v = seggi_per_circoscrizione(&d.elezioni().senato, 35);
    assert_eq!(
        seggi(&v),
        [
            14, 1, 32, 3, 3, 16, 4, 5, 14, 12, 3, 5, 18, 4, 2, 18, 13, 3, 6, 15, 5
        ]
    );
}

#[test]
fn premio_del_senato_come_il_dossier() {
    let d = dati_base();
    let r = &d.elezioni().senato;
    let v = seggi_per_circoscrizione(r, 35);
    assert_eq!(
        premi(&v),
        [
            3, 0, 6, 0, 0, 3, 1, 1, 3, 2, 1, 1, 3, 1, 0, 3, 2, 0, 1, 3, 1
        ]
    );
    assert_eq!(plurinominali(r, &v), 154);
    assert_eq!(plurinominali(r, &seggi_per_circoscrizione(r, 0)), 189);
}

#[test]
fn uninominali_delle_circoscrizioni_speciali() {
    let d = dati_base();
    let e = d.elezioni();
    let camera: &[(&str, u32)] = &[("valle_d_aosta", 1), ("trentino_alto_adige", 4)];
    let senato: &[(&str, u32)] = &[("valle_d_aosta", 1), ("bolzano", 3), ("trento", 3)];
    for (r, speciali) in [(&e.camera, camera), (&e.senato, senato)] {
        for s in seggi_per_circoscrizione(r, 0) {
            let atteso = speciali
                .iter()
                .find(|(id, _)| *id == s.id)
                .map_or(0, |&(_, n)| n);
            assert_eq!(s.uninominali, atteso, "{}", s.id);
        }
    }
}

#[test]
fn premio_e_tetto_in_quota_dei_seggi() {
    assert_eq!(seggi_da_quota(1750, 400), 70);
    assert_eq!(seggi_da_quota(1750, 200), 35);
    assert_eq!(seggi_da_quota(5500, 400), 220);
    assert_eq!(seggi_da_quota(5650, 200), 113);
    assert_eq!(seggi_da_quota(875, 200), 18);
    assert_eq!(seggi_da_quota(0, 400), 0);
}

#[test]
fn ramo_ridotto_rispetta_i_minimi() {
    let d = dati_base();
    let mut senato = d.elezioni().senato.clone();
    senato.seggi = 100;
    let v = seggi_per_circoscrizione(&senato, 0);
    assert_eq!(v.iter().map(|s| s.seggi).sum::<u32>(), 96);
    for s in &v {
        match s.id.as_str() {
            "molise" => assert_eq!(s.seggi, 2),
            "valle_d_aosta" => assert_eq!(s.seggi, 1),
            _ => assert!(s.seggi >= 3, "{} {}", s.id, s.seggi),
        }
    }
    let mut camera = d.elezioni().camera.clone();
    camera.seggi = 300;
    let v = seggi_per_circoscrizione(&camera, 0);
    assert_eq!(v.iter().map(|s| s.seggi).sum::<u32>(), 292);
}

#[test]
fn ingressi_degeneri_senza_panico() {
    let d = dati_base();
    let mut r = d.elezioni().camera.clone();
    r.seggi = 0;
    assert!(
        seggi_per_circoscrizione(&r, 70)
            .iter()
            .all(|s| s.seggi == 0)
    );
    r.circoscrizioni.clear();
    assert!(seggi_per_circoscrizione(&r, 70).is_empty());
}
