mod comune;
use comune::{con_ingresso, dati_base, senza_shock};
use sim_core::{DatiGioco, Partita};

fn giocata(dati: DatiGioco, turni: u32) -> Partita {
    let mut p = Partita::nuova(dati, 1);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}
fn base_senza_shock() -> DatiGioco {
    DatiGioco::da_testi(&senza_shock()).unwrap()
}

#[test]
fn disavanzo_del_turno_zero_e_intorno_al_2_9_per_cento() {
    let p = giocata(base_senza_shock(), 1);
    let e = p.economia();
    let quota = e.disavanzo_trimestre as f64 / (e.pil_trimestrale * 1e9) * 100.0;
    assert!((2.5..=3.3).contains(&quota), "{quota}");
}

#[test]
fn il_debito_cresce_esattamente_del_disavanzo() {
    let mut p = Partita::nuova(base_senza_shock(), 1);
    for _ in 0..8 {
        let prima = p.economia().debito;
        p.chiudi_turno();
        assert_eq!(
            p.economia().debito,
            prima + p.economia().disavanzo_trimestre
        );
    }
}

#[test]
fn bilancio_2027_applicato_una_volta() {
    let sanita = |p: &Partita| {
        p.economia()
            .bilancio
            .dicasteri
            .iter()
            .find(|d| d.id == "sanita")
            .unwrap()
            .spesa
    };
    let s26 = sanita(&Partita::nuova(base_senza_shock(), 1));
    let dopo_ottobre = giocata(base_senza_shock(), 1);
    assert_eq!(dopo_ottobre.economia().bilancio.anno, 2026);
    let dopo_gennaio = giocata(base_senza_shock(), 2);
    assert_eq!(dopo_gennaio.economia().bilancio.anno, 2027);
    assert!((sanita(&dopo_gennaio) - (s26 as f64 * 1.019).round() as i64).abs() <= 1);
    let gennaio_2028 = giocata(base_senza_shock(), 6);
    assert_eq!(
        gennaio_2028.economia().bilancio.anno,
        2027,
        "senza legge di bilancio resta il 2027"
    );
    assert_eq!(sanita(&gennaio_2028), sanita(&dopo_gennaio));
}

#[test]
fn il_disavanzo_si_deriva_al_caricamento() {
    let e = Partita::nuova(base_senza_shock(), 1);
    assert!(e.economia().disavanzo_trimestre > 0);
}

#[test]
fn margine_al_1_6_per_cento() {
    let p = Partita::nuova(dati_base(), 1);
    let b = &p.economia().bilancio;
    let (m, s): (i64, i64) = b
        .dicasteri
        .iter()
        .fold((0, 0), |(m, s), d| (m + d.margine, s + d.spesa));
    assert!(((m as f64 / s as f64) * 100.0 - 1.6).abs() < 0.01);
}

#[test]
fn bilancio_fermo_in_nominale_e_un_impulso_negativo() {
    let p = giocata(base_senza_shock(), 6);
    assert!(p.economia().impulso_fiscale < 0.0);
}

#[test]
fn interessi_del_turno_zero_al_costo_medio() {
    let p = giocata(base_senza_shock(), 1);
    let e = p.economia();
    let annui = e.conti.spese.interessi as f64 * 4.0 / 1e9;
    assert!(
        (annui - 0.0281 * 1.367 * 4.0 * 576.9).abs() < 1.0,
        "{annui}"
    );
}

#[test]
fn storico_di_otto_trimestri_col_piu_recente_in_coda() {
    let p = giocata(base_senza_shock(), 10);
    let s = &p.economia().storico;
    assert_eq!(s.len(), 8);
    assert_eq!(s[7].data.anno, 2029);
    assert_eq!(s[7].data.trimestre, 1);
    let zero = Partita::nuova(base_senza_shock(), 1);
    let iniziale = &zero.economia().storico;
    assert_eq!(iniziale.len(), 4);
    assert_eq!(
        (iniziale[3].data.anno, iniziale[3].data.trimestre),
        (2026, 3)
    );
}

fn con_regime_pensioni(regime: &str) -> DatiGioco {
    let mut f = senza_shock();
    f.scenario = f.scenario.replacen(
        "regime_pensioni = \"contributivo\"",
        &format!("regime_pensioni = \"{regime}\""),
        1,
    );
    DatiGioco::da_testi(&f).unwrap()
}

#[test]
fn spesa_pensioni_del_turno_zero_e_quella_dello_scenario() {
    let p = giocata(base_senza_shock(), 1);
    let annua = p.economia().conti.spese.pensioni as f64 * 4.0 / 1e9;
    assert!((annua - 351.8).abs() < 0.5, "{annua}");
}

#[test]
fn il_regime_tocca_solo_le_annate_nuove() {
    let (r, c) = (
        con_regime_pensioni("retributivo"),
        con_regime_pensioni("contributivo"),
    );
    assert_eq!(
        giocata(r.clone(), 1).economia().conti.spese.pensioni,
        giocata(c.clone(), 1).economia().conti.spese.pensioni
    );
    let (pr, pc) = (giocata(r, 9), giocata(c, 9));
    assert!(pr.economia().conti.spese.pensioni > pc.economia().conti.spese.pensioni);
    assert_eq!(
        pr.economia().pensioni[0].importo,
        pc.economia().pensioni[0].importo,
        "l'annata del 2026 non cambia col regime"
    );
}

#[test]
fn le_pensioni_in_essere_seguono_l_inflazione() {
    let p = giocata(base_senza_shock(), 2);
    let a = &p.economia().pensioni[0];
    let iniziale = (351.8e9_f64 / 16_388_149.0).round() as i64;
    assert!(a.importo > iniziale);
}

#[test]
fn i_sussidi_salgono_in_recessione() {
    let crisi = giocata(
        DatiGioco::da_testi(&con_ingresso(senza_shock(), "output_gap", -4.0)).unwrap(),
        2,
    );
    let normale = giocata(base_senza_shock(), 2);
    assert!(crisi.economia().conti.spese.sussidi > normale.economia().conti.spese.sussidi);
}

#[test]
fn sussidi_del_turno_zero_alla_naspi() {
    let p = giocata(base_senza_shock(), 1);
    let annui = p.economia().conti.spese.sussidi as f64 * 4.0 / 1e9;
    assert!((annui - 17.93).abs() < 0.5, "{annui}");
}

#[test]
fn la_somma_dei_saldi_fa_zero_a_ogni_turno() {
    for seme in 0..20 {
        let mut p = Partita::nuova(dati_base(), seme);
        for _ in 0..40 {
            p.chiudi_turno();
            assert_eq!(p.economia().residuo_sfc(), 0, "seme {seme}");
            assert_eq!(p.economia().residuo_stock(), 0, "seme {seme}");
        }
    }
}

#[test]
fn lo_stock_dello_stato_e_il_debito() {
    let p = giocata(dati_base(), 12);
    assert_eq!(p.economia().sfc.stock.stato, -p.economia().debito);
}

#[test]
fn il_saldo_dello_stato_e_meno_il_disavanzo() {
    let p = giocata(dati_base(), 3);
    assert_eq!(
        p.economia().sfc.saldi.stato,
        -p.economia().disavanzo_trimestre
    );
}
