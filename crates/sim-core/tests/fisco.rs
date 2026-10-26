#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

mod comune;
use comune::{dati_base, file_base, senza_shock, sostituisci};
use serde_json::{Value, json};
use sim_core::dati::DatiGioco;
use sim_core::economia::{Scaglione, irpef};
use sim_core::{Errore, Partita};

fn scaglioni_2026() -> Vec<Scaglione> {
    vec![
        Scaglione {
            fino_a: Some(28_000),
            aliquota: 0.23,
            livello_prezzi: 1.0,
        },
        Scaglione {
            fino_a: Some(50_000),
            aliquota: 0.33,
            livello_prezzi: 1.0,
        },
        Scaglione {
            fino_a: None,
            aliquota: 0.43,
            livello_prezzi: 1.0,
        },
    ]
}

/// La partita dopo `turni` chiusure, con l'economia salvata modificata da `modifica`.
fn con_bilancio(turni: u32, modifica: impl FnOnce(&mut Value)) -> Result<Partita, Errore> {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    let mut j: Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    modifica(&mut j["economia"]);
    Partita::carica(&j.to_string())
}

#[test]
fn irpef_2026_sui_tre_scaglioni() {
    let s = scaglioni_2026();
    assert!((irpef(&s, 28_000.0) - 6_440.0).abs() < 1e-6);
    assert!((irpef(&s, 50_000.0) - 13_700.0).abs() < 1e-6);
    assert!((irpef(&s, 60_000.0) - 18_000.0).abs() < 1e-6);
    assert_eq!(irpef(&s, -5.0), 0.0);
}

#[test]
fn aliquote_del_turno_zero_lasciano_i_fattori_neutri() {
    let mut p = Partita::nuova(dati_base(), 1);
    assert_eq!(
        p.economia().bilancio.aliquote,
        p.economia().bilancio_approvato.as_ref().unwrap().aliquote
    );
    let s = &p.economia().bilancio.aliquote.irpef;
    assert!(s.iter().all(|x| x.livello_prezzi == 1.0));
    assert_eq!(p.economia().conti.entrate_discrezionali, 0);
    p.chiudi_turno();
    assert_eq!(p.misure().fattore_irpef, 1.0);
    assert_eq!(p.misure().irpef_discrezionale, 0.0);
    assert_eq!(p.economia().conti.entrate_discrezionali, 0);
}

#[test]
fn scaglioni_fermi_e_inflazione_danno_drenaggio_fiscale() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..8 {
        p.chiudi_turno();
    }
    assert!(p.misure().fattore_irpef > 1.0, "{:?}", p.misure());
    assert_eq!(p.misure().irpef_discrezionale, 0.0);
    assert_eq!(p.economia().conti.entrate_discrezionali, 0);
}

#[test]
fn scaglioni_non_crescenti_nello_scenario_rifiutati_col_nome() {
    let f = sostituisci(
        file_base(),
        "scenario",
        "{ fino_a = 50000, aliquota = 0.33 }",
        "{ fino_a = 20000, aliquota = 0.33 }",
    );
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("scaglioni"), "{e}");
}

#[test]
fn un_salvataggio_con_aliquote_incoerenti_e_rifiutato() {
    let e = con_bilancio(0, |e| e["bilancio"]["aliquote"]["iva"] = json!(2.0)).err();
    assert!(
        matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
        "{e:?}"
    );
    let e = con_bilancio(0, |e| {
        e["bilancio"]["aliquote"]["irpef"][0]["livello_prezzi"] = json!(0.0)
    })
    .err();
    assert!(
        matches!(e, Some(Errore::SalvataggioIlleggibile(_))),
        "{e:?}"
    );
}

#[test]
fn iva_piu_alta_da_gettito_discrezionale_che_la_spesa_netta_sottrae() {
    let mut a = con_bilancio(0, |_| ()).unwrap();
    let mut b = con_bilancio(0, |e| e["bilancio"]["aliquote"]["iva"] = json!(0.24)).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    let (ca, cb) = (&a.economia().conti, &b.economia().conti);
    assert!(cb.entrate.iva > ca.entrate.iva);
    // Meno che in proporzione (elasticità dell'imponibile), ma oltre il valore di partenza.
    assert!((cb.entrate.iva as f64) < ca.entrate.iva as f64 * 0.24 / 0.22);
    assert_eq!(ca.entrate_discrezionali, 0);
    assert!(cb.entrate_discrezionali > 0, "{cb:?}");
    assert!(
        (cb.entrate_discrezionali - (cb.entrate.iva - ca.entrate.iva)).abs() <= 1,
        "solo l'IVA è cambiata"
    );
}

#[test]
fn scaglione_piu_alto_alza_irpef_e_imposte_dei_gruppi() {
    let mut a = con_bilancio(0, |_| ()).unwrap();
    let mut b = con_bilancio(0, |e| {
        e["bilancio"]["aliquote"]["irpef"][2]["aliquota"] = json!(0.6)
    })
    .unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert!(b.misure().irpef_discrezionale > 0.0);
    assert!(b.misure().fattore_irpef > a.misure().fattore_irpef);
    assert!(b.economia().conti.entrate.irpef > a.economia().conti.entrate.irpef);
    assert!(b.economia().conti.entrate_discrezionali > 0);
    let imposte = |p: &Partita| -> f64 {
        p.elettorato()
            .gruppi
            .iter()
            .map(|g| g.reddito.imposte)
            .sum()
    };
    assert!(imposte(&b) > imposte(&a));
}

#[test]
fn il_bilancio_approvato_vale_dal_primo_trimestre_anche_per_le_misure() {
    let mut p = con_bilancio(0, |e| {
        e["bilancio_approvato"]["aliquote"]["irpef"][2]["aliquota"] = json!(0.6);
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.misure().irpef_discrezionale, 0.0, "ancora il 2026");
    p.chiudi_turno();
    assert!(p.misure().irpef_discrezionale > 0.0, "{:?}", p.misure());
    assert!(p.economia().conti.entrate_discrezionali > 0);
}

#[test]
fn scenario_con_aliquote_nulle_rifiutato_col_nome() {
    for (da, a, voce) in [
        ("iva = 0.22", "iva = 0.0", "iva"),
        ("ires = 0.24", "ires = 0.0", "ires"),
        ("contributi = 0.33", "contributi = 0.0", "contributi"),
        ("aliquota = 0.23", "aliquota = 0.0", "irpef"),
    ] {
        let f = sostituisci(file_base(), "scenario", da, a);
        let e = DatiGioco::da_testi(&f).unwrap_err().to_string();
        assert!(e.contains(voce), "{voce}: {e}");
    }
}

#[test]
fn un_salvataggio_con_aliquota_irpef_0_nulla_e_rifiutato() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    let mut j: Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    j["elettorato"]["aliquota_irpef_0"] = json!(0.0);
    assert!(matches!(
        Partita::carica(&j.to_string()),
        Err(Errore::SalvataggioIlleggibile(_))
    ));
}

#[test]
fn ultimo_scaglione_piu_basso_da_discrezionale_negativo() {
    let mut a = con_bilancio(0, |_| ()).unwrap();
    let mut b = con_bilancio(0, |e| {
        e["bilancio"]["aliquote"]["irpef"][2]["aliquota"] = json!(0.3)
    })
    .unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert!(b.misure().irpef_discrezionale < 0.0);
    assert!(b.economia().conti.entrate_discrezionali < 0);
    let d = b.economia().conti.entrate.irpef - a.economia().conti.entrate.irpef;
    assert!(d < 0, "{d}");
}

#[test]
fn scaglioni_a_zero_azzerano_l_irpef_e_il_discrezionale_e_negativo() {
    let mut p = con_bilancio(0, |e| {
        for s in e["bilancio"]["aliquote"]["irpef"].as_array_mut().unwrap() {
            s["aliquota"] = json!(0.0);
        }
    })
    .unwrap();
    p.chiudi_turno();
    assert_eq!(p.misure().fattore_irpef, 0.0);
    assert!(p.economia().conti.entrate.irpef.abs() <= 1);
    assert!(p.economia().conti.entrate_discrezionali < 0);
}
