mod comune;
use comune::{con_ingresso, dati_base, file_base, senza_shock};
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

fn dati(f: sim_core::FileDati) -> DatiGioco {
    DatiGioco::da_testi(&f).unwrap()
}
fn inadempiente(f: sim_core::FileDati) -> sim_core::FileDati {
    comune::sostituisci(f, "scenario", "pde = \"sospesa\"", "pde = \"inadempiente\"")
}

#[test]
fn spread_del_turno_zero_fra_79_e_89() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert!((79..=89).contains(&v.spread_pb), "{}", v.spread_pb);
}

#[test]
fn spread_piatto_sotto_la_soglia() {
    let p = Partita::nuova(dati(con_ingresso(senza_shock(), "debito_pil", 110.0)), 1);
    assert_eq!(p.vista().spread_pb, 60);
}

#[test]
fn spread_convesso_sopra_la_soglia() {
    let s = |b: f64| {
        Partita::nuova(dati(con_ingresso(senza_shock(), "debito_pil", b)), 1)
            .economia()
            .spread
    };
    let (a, b, c) = (s(140.0), s(150.0), s(160.0));
    assert!(c - b > b - a && b > a, "{a} {b} {c}");
}

#[test]
fn il_tpi_azzera_il_premio_con_la_pde_sospesa() {
    let p = giocata(
        dati(con_ingresso(senza_shock(), "premio_sfiducia", 200.0)),
        1,
    );
    assert_eq!(p.economia().premio_sfiducia, 0.0);
}

#[test]
fn senza_tpi_il_premio_decade_con_la_sua_emivita() {
    let f = inadempiente(con_ingresso(senza_shock(), "premio_sfiducia", 200.0));
    let premio = giocata(dati(f), 2).economia().premio_sfiducia;
    assert!(
        (premio - 200.0 * 0.5_f64.powf(2.0 / 4.0)).abs() < 1e-6,
        "{premio}"
    );
}

#[test]
fn conto_di_controllo_si_muove_solo_ad_aprile_giugno() {
    let mut p = Partita::nuova(dati_base(), 3);
    let mut prima = p.economia().regole_ue.conto_controllo;
    for _ in 0..12 {
        p.chiudi_turno();
        let ora = p.economia().regole_ue.conto_controllo;
        if ora != prima {
            assert_eq!(
                p.vista().data.trimestre,
                3,
                "il controllo gira nel turno aprile-giugno appena chiuso"
            );
        }
        prima = ora;
    }
}

#[test]
fn nessuna_inadempienza_spuria_al_primo_controllo() {
    let p = giocata(base_senza_shock(), 3);
    assert!(!p.economia().regole_ue.pde.e_inadempiente());
}

#[test]
fn salva_carica_con_pde_inadempiente() {
    let mut diretta = giocata(dati(inadempiente(file_base())), 5);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    for _ in 0..6 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
}

#[test]
fn la_catena_delle_sanzioni_si_attraversa() {
    use sim_core::scenario::StatoPde::{Inadempiente, Intimata, Sanzionata};
    // Conto al massimo e nessuna clausola: ogni controllo annuale e' rilevante.
    let forzata = || {
        con_ingresso(
            con_ingresso(senza_shock(), "conto_controllo", 3.0),
            "clausola_nazionale_margine",
            0.0,
        )
    };
    let mut p = Partita::nuova(dati(forzata()), 1);
    let mut riferimento = Partita::nuova(base_senza_shock(), 1);
    let (mut sequenza, mut ammende_sanzionata) = (vec![p.economia().regole_ue.pde], vec![]);
    let mut ammenda_ingresso = false;
    let mut conto_mosso = false;
    for _ in 0..16 {
        let prima = p.economia().regole_ue.clone();
        p.chiudi_turno();
        riferimento.chiudi_turno();
        let e = p.economia();
        let trimestre = p.vista().data.trimestre;
        assert_eq!(e.residuo_sfc(), 0);
        assert!(e.conti.entrate.fondi_coesione >= 0);
        if e.regole_ue.conto_controllo != prima.conto_controllo {
            conto_mosso = true;
            assert_eq!(trimestre, 3);
        }
        if prima.pde == Inadempiente && prima.turni_in_stato == 0 {
            ammenda_ingresso = e.conti.spese.ammende > 0;
        }
        if prima.pde == Sanzionata {
            ammende_sanzionata.push(e.conti.spese.ammende > 0);
        }
        if prima.pde.e_inadempiente() {
            assert!(
                e.conti.entrate.fondi_coesione
                    < riferimento.economia().conti.entrate.fondi_coesione
            );
        }
        if e.regole_ue.pde != prima.pde {
            if e.regole_ue.pde == Intimata {
                assert_eq!(trimestre, 1, "l'intimazione arriva chiudendo un T4");
            }
            sequenza.push(e.regole_ue.pde);
        }
    }
    assert_eq!(
        sequenza[1..],
        [Inadempiente, Intimata, Sanzionata],
        "{sequenza:?}"
    );
    assert!(ammenda_ingresso, "nessuna ammenda al turno d'ingresso");
    assert!(ammende_sanzionata.len() >= 4, "{ammende_sanzionata:?}");
    assert!(
        ammende_sanzionata.windows(2).all(|w| w[0] != w[1]),
        "ogni 2 turni: {ammende_sanzionata:?}"
    );
    assert!(conto_mosso);
}

#[test]
fn scenario_estremo_resta_finito() {
    let f = con_ingresso(
        con_ingresso(
            con_ingresso(file_base(), "debito_pil", 200.0),
            "inflazione",
            12.0,
        ),
        "disoccupazione",
        15.0,
    );
    let mut p = Partita::nuova(dati(f), 9);
    for _ in 0..200 {
        p.chiudi_turno();
        assert_eq!(p.economia().residuo_sfc(), 0);
    }
    assert!(p.salva().is_ok());
}
