mod comune;

use comune::{dati_base, file_base, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::elezioni::{
    Esito, Formula, LeggeElettorale, Premio, QuoteTetto, Ramo, SeggiCircoscrizione, SeggiLista,
    Voti, riparto, seggi_da_quota, seggi_per_circoscrizione,
};
use sim_core::errore::Errore;
use sim_core::struttura::Macroarea;
use std::collections::BTreeMap;

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

// --- riparto (Task 3): numeri attesi da strumenti/riparto/riferimento.py ---

fn impostazione(nome: &str) -> LeggeElettorale {
    dati_base().elezioni().impostazioni[nome].clone()
}

/// Popolazione di ogni regione: somma delle circoscrizioni della Camera.
fn popolazioni() -> BTreeMap<String, u64> {
    let mut m = BTreeMap::new();
    for c in &dati_base().elezioni().camera.circoscrizioni {
        *m.entry(c.regione.clone()).or_insert(0) += c.popolazione;
    }
    m
}

const SARDEGNA: [u64; 11] = [280, 90, 25, 10, 240, 60, 20, 0, 50, 225, 0];

/// Voti `(p_r / 2) * millesimi / 1000`, righe per macroarea (nord, centro, mezzogiorno).
fn voti_per_macroarea(righe: [[u64; 11]; 3]) -> Voti {
    let d = dati_base();
    let per_regione = popolazioni()
        .into_iter()
        .map(|(r, p)| {
            let area = d.regioni().iter().find(|x| x.id == r).unwrap().macroarea;
            let riga = match (r.as_str(), area) {
                ("sardegna", _) => SARDEGNA,
                (_, Macroarea::Nord) => righe[0],
                (_, Macroarea::Centro) => righe[1],
                (_, Macroarea::Mezzogiorno) => righe[2],
            };
            (r, riga.iter().map(|&m| (p / 2) * m / 1000).collect())
        })
        .collect();
    Voti {
        coalizioni: vec![vec![0, 1, 2, 3], vec![4, 5, 6]],
        per_regione,
    }
}

fn voti_g1() -> Voti {
    voti_per_macroarea([
        [320, 90, 25, 10, 200, 60, 20, 140, 50, 20, 65],
        [300, 90, 25, 10, 220, 60, 20, 140, 50, 20, 65],
        [280, 90, 25, 10, 240, 60, 20, 140, 50, 20, 65],
    ])
}

fn voti_g2() -> Voti {
    voti_per_macroarea([
        [400, 90, 25, 10, 150, 50, 20, 120, 50, 20, 65],
        [380, 90, 25, 10, 170, 50, 20, 120, 50, 20, 65],
        [360, 90, 25, 10, 190, 50, 20, 120, 50, 20, 65],
    ])
}

/// Voti `p_r * millesimi[l]`: stesse quote in ogni regione.
fn uniformi(millesimi: &[u64], coalizioni: Vec<Vec<usize>>) -> Voti {
    Voti {
        coalizioni,
        per_regione: popolazioni()
            .into_iter()
            .map(|(r, p)| (r, millesimi.iter().map(|&m| p * m).collect()))
            .collect(),
    }
}

fn esito(legge: &LeggeElettorale, voti: &Voti) -> Esito {
    riparto(dati_base().elezioni(), legge, voti).unwrap()
}

fn prop(v: &[SeggiLista]) -> Vec<u32> {
    v.iter().map(|s| s.proporzionali).collect()
}

#[test]
fn proporzionale_3_come_il_riferimento() {
    let e = esito(&impostazione("proporzionale_3"), &voti_g1());
    assert_eq!(prop(&e.camera), [126, 38, 0, 0, 91, 25, 0, 57, 21, 0, 26]);
    assert_eq!(prop(&e.senato), [64, 20, 0, 0, 45, 10, 0, 30, 9, 1, 10]);
}

#[test]
fn proporzionale_puro_come_il_riferimento() {
    let e = esito(&impostazione("proporzionale_puro"), &voti_g1());
    assert_eq!(prop(&e.camera), [116, 34, 10, 4, 84, 23, 8, 52, 19, 10, 24]);
    assert_eq!(prop(&e.senato), [60, 20, 5, 0, 43, 10, 1, 29, 9, 2, 10]);
}

#[test]
fn senato_ammette_il_20_per_cento_regionale() {
    let e = esito(&impostazione("proporzionale_3"), &voti_g1());
    assert_eq!(e.camera[9].proporzionali, 0);
    assert!(e.senato[9].proporzionali >= 1);
}

#[test]
fn soglia_di_lista_esatta() {
    let e = esito(
        &impostazione("proporzionale_3"),
        &uniformi(&[30, 29, 941], vec![]),
    );
    for ramo in [&e.camera, &e.senato] {
        assert!(ramo[0].proporzionali > 0);
        assert_eq!(ramo[1].proporzionali, 0);
    }
}

#[test]
fn coalizione_sotto_il_10_lascia_le_liste_sole() {
    let e = esito(
        &legge_180(),
        &uniformi(&[50, 25, 300, 300, 325], vec![vec![0, 1]]),
    );
    assert_eq!(e.camera[0].proporzionali, 20);
    assert_eq!(e.camera[1].proporzionali, 0);
}

#[test]
fn tutti_i_voti_della_coalizione_contano() {
    let e = esito(
        &legge_180(),
        &uniformi(&[95, 6, 300, 300, 299], vec![vec![0, 1]]),
    );
    assert_eq!(e.camera[1].proporzionali, 2);
    let e = esito(
        &legge_180(),
        &uniformi(&[93, 6, 300, 300, 301], vec![vec![0, 1]]),
    );
    assert_eq!(e.camera[1].proporzionali, 0);
}

#[test]
fn ripescaggio_della_piu_votata_sotto_soglia() {
    let voti = uniformi(&[300, 25, 15, 300, 360], vec![vec![0, 1, 2]]);
    let e = esito(&legge_180(), &voti);
    assert_eq!(e.camera[1].proporzionali, 10);
    assert_eq!(e.camera[2].proporzionali, 0);
    let senza = LeggeElettorale {
        ripescaggio: false,
        ..legge_180()
    };
    assert_eq!(esito(&senza, &voti).camera[1].proporzionali, 0);
}

#[test]
fn dhondt_favorisce_i_grandi() {
    let hare = impostazione("proporzionale_3");
    let dhondt = LeggeElettorale {
        formula: Formula::Dhondt,
        ..hare.clone()
    };
    let v = voti_g1();
    assert!(esito(&dhondt, &v).camera[0].proporzionali >= esito(&hare, &v).camera[0].proporzionali);
}

#[test]
fn senato_per_regione_somma_ai_seggi() {
    let v = voti_g1();
    for (nome, legge) in &dati_base().elezioni().impostazioni {
        let e = esito(legge, &v);
        for (l, s) in e.senato.iter().enumerate() {
            let somma: u32 = e.senato_per_regione.values().map(|r| r[l]).sum();
            assert_eq!(somma, s.proporzionali + s.speciali, "{nome} lista {l}");
        }
    }
}

#[test]
fn voti_incoerenti_rifiutati() {
    let base = voti_g1();
    let mut casi = Vec::new();
    let mut v = base.clone();
    v.per_regione.remove("molise");
    casi.push(v);
    let mut v = base.clone();
    v.per_regione.insert("atlantide".into(), vec![1; 11]);
    casi.push(v);
    let mut v = base.clone();
    v.per_regione.get_mut("lazio").unwrap().pop();
    casi.push(v);
    let mut v = base.clone();
    v.coalizioni.push(vec![10, 11]);
    casi.push(v);
    let mut v = base.clone();
    v.coalizioni[1].push(0);
    casi.push(v);
    let mut v = base;
    for x in v.per_regione.values_mut() {
        x.clear();
    }
    v.coalizioni.clear();
    casi.push(v);
    for (i, v) in casi.iter().enumerate() {
        let r = riparto(dati_base().elezioni(), &legge_180(), v);
        assert!(
            matches!(r, Err(Errore::VotiIncoerenti(_))),
            "caso {i}: {r:?}"
        );
    }
}

#[test]
fn nessun_soggetto_ammesso_non_va_in_panico() {
    let e = esito(&legge_180(), &uniformi(&[25; 40], vec![]));
    assert!(
        e.camera
            .iter()
            .chain(&e.senato)
            .all(|s| s.proporzionali == 0)
    );
}

#[test]
fn pochi_voti_conservano_i_seggi() {
    let mut v = uniformi(&[], vec![]);
    for x in v.per_regione.values_mut() {
        *x = vec![1, 1, 1];
    }
    let e = esito(&impostazione("proporzionale_puro"), &v);
    assert_eq!(e.camera.iter().map(|s| s.proporzionali).sum::<u32>(), 384);
    assert_eq!(e.senato.iter().map(|s| s.proporzionali).sum::<u32>(), 189);
}

#[test]
fn parita_risolta_per_ordine() {
    let legge = impostazione("proporzionale_3");
    let v = uniformi(&[400, 400, 200], vec![]);
    let e = esito(&legge, &v);
    assert_eq!(e, esito(&legge, &v));
    let (a, b) = (e.camera[0].proporzionali, e.camera[1].proporzionali);
    assert!(a >= b && a - b <= 1, "{a} {b}");
    assert_eq!(e.camera.iter().map(|s| s.proporzionali).sum::<u32>(), 384);
}

#[test]
fn legge_180_senza_premio_come_il_riferimento() {
    let legge = LeggeElettorale {
        premio: Premio::Nessuno,
        ..legge_180()
    };
    let e = esito(&legge, &voti_g1());
    assert_eq!(prop(&e.camera), [122, 36, 10, 0, 86, 23, 8, 54, 20, 0, 25]);
    assert_eq!(prop(&e.senato), [61, 17, 2, 0, 48, 10, 1, 29, 9, 2, 10]);
}

#[test]
fn seggi_della_coalizione_conservati_senza_voti_alle_ammesse() {
    // Nel Lazio la coalizione [0, 1, 2] ha solo i voti della lista 2: sotto il 3% nazionale,
    // sotto il 20% regionale, non ripescata. Le ammesse 0 e 1 lì hanno cifra 0.
    let legge = LeggeElettorale {
        premio: Premio::Nessuno,
        ripescaggio: false,
        ..legge_180()
    };
    let mut v = uniformi(&[300, 200, 0, 500], vec![vec![0, 1, 2]]);
    let lazio_pop = popolazioni()["lazio"];
    v.per_regione.insert(
        "lazio".into(),
        [0, 0, 190, 810].iter().map(|&m| lazio_pop * m).collect(),
    );
    let e = esito(&legge, &v);
    let lazio = &e.senato_per_regione["lazio"];
    let seggi_lazio = seggi_per_circoscrizione(&dati_base().elezioni().senato, 0)
        .into_iter()
        .find(|s| s.id == "lazio")
        .unwrap()
        .seggi;
    assert!(lazio[0] + lazio[1] > 0, "{lazio:?}");
    assert_eq!(lazio.iter().sum::<u32>(), seggi_lazio, "{lazio:?}");
}

// --- speciali ed estero (Task 4) ---

fn speciali(v: &[SeggiLista]) -> Vec<u32> {
    v.iter().map(|s| s.speciali).collect()
}

fn estero(v: &[SeggiLista]) -> Vec<u32> {
    v.iter().map(|s| s.estero).collect()
}

#[test]
fn speciali_ed_estero_come_il_riferimento() {
    let v = voti_g1();
    let e = esito(&impostazione("proporzionale_3"), &v);
    assert_eq!(speciali(&e.camera), [4, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0]);
    assert_eq!(estero(&e.camera), [3, 1, 0, 0, 2, 0, 0, 1, 0, 0, 1]);
    assert_eq!(speciali(&e.senato), [5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0]);
    assert_eq!(estero(&e.senato), [1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0]);
    let e = esito(&impostazione("proporzionale_puro"), &v);
    assert_eq!(speciali(&e.camera), [4, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0]);
    assert_eq!(estero(&e.camera), [2, 1, 0, 0, 2, 1, 0, 1, 0, 0, 1]);
    assert_eq!(speciali(&e.senato), [5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0]);
    assert_eq!(estero(&e.senato), [1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0]);
}

#[test]
fn ogni_ramo_somma_al_suo_totale() {
    for v in [voti_g1(), voti_g2()] {
        for nome in ["proporzionale_3", "proporzionale_puro"] {
            let e = esito(&impostazione(nome), &v);
            let tot = |s: &[SeggiLista]| s.iter().map(SeggiLista::totale).sum::<u32>();
            assert_eq!(tot(&e.camera), 400, "{nome}");
            assert_eq!(tot(&e.senato), 200, "{nome}");
        }
    }
}

#[test]
fn valle_d_aosta_al_piu_votato() {
    let mut v = uniformi(&[], vec![]);
    for x in v.per_regione.values_mut() {
        *x = vec![100, 600, 300];
    }
    let e = esito(&impostazione("proporzionale_puro"), &v);
    assert_eq!(e.senato_per_regione["valle_d_aosta"], [0, 1, 0]);
}

#[test]
fn uninominali_senza_soglia() {
    let mut v = uniformi(&[25; 40], vec![]);
    let vda = popolazioni()["valle_d_aosta"];
    let mut riga = vec![10; 40];
    riga[0] = 600;
    v.per_regione.insert(
        "valle_d_aosta".into(),
        riga.iter().map(|m| vda * m).collect(),
    );
    let e = esito(&legge_180(), &v);
    assert_eq!(e.camera[0].proporzionali, 0);
    assert!(e.camera[0].speciali >= 1);
    assert_eq!(e.senato[0].proporzionali, 189);
}
