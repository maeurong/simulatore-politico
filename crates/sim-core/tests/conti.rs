mod comune;
use comune::{con_ingresso, dati_base, file_base, imposta, senza_shock, sostituisci};
use sim_core::economia::Economia;
use sim_core::scenario::StatoPde;
use sim_core::{DatiGioco, FileDati, Partita};

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
    let f = sostituisci(
        senza_shock(),
        "disposizioni",
        "partenza = \"contributivo\"",
        &format!("partenza = \"{regime}\""),
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
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..40 {
        p.chiudi_turno();
        assert_eq!(p.economia().sfc.stock.stato, -p.economia().debito);
    }
    let r = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(r.economia().sfc.stock.stato, -r.economia().debito);
}

#[test]
fn il_saldo_dello_stato_e_meno_il_disavanzo() {
    let p = giocata(dati_base(), 3);
    assert_eq!(
        p.economia().sfc.saldi.stato,
        -p.economia().disavanzo_trimestre
    );
}

fn dati(f: FileDati) -> DatiGioco {
    DatiGioco::da_testi(&f).unwrap()
}
fn con_pde(f: FileDati, stato: &str) -> FileDati {
    sostituisci(
        f,
        "scenario",
        "pde = \"sospesa\"",
        &format!("pde = \"{stato}\""),
    )
}
fn inadempiente(f: FileDati) -> FileDati {
    con_pde(f, "inadempiente")
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
    // Conto al massimo e nessuna clausola: ogni controllo annuale è rilevante.
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

// Un controllo annuale gira chiudendo il terzo turno (T2 2027, sull'anno 2026).

/// Disavanzo del 2026 in % del PIL, come lo vede il controllo.
fn disavanzo_2026(p: &Partita) -> f64 {
    let r = &p.economia().regole_ue;
    r.disavanzo_annuo[&2026] as f64 / r.pil_annuo[&2026] as f64 * 100.0
}

/// Disavanzo degli ultimi 4 trimestri dello storico, in % del loro PIL.
fn disavanzo_mobile(e: &Economia) -> f64 {
    let ultimi = &e.storico[e.storico.len() - 4..];
    let pil: f64 = ultimi.iter().map(|t| t.pil as f64).sum();
    ultimi.iter().map(|t| t.disavanzo as f64).sum::<f64>() / pil * 100.0
}

/// Disavanzo sopra il 3%: meno altre entrate.
fn disavanzo_alto(f: FileDati) -> FileDati {
    con_ingresso(f, "altre_entrate_pil", 11.5)
}

#[test]
fn pde_nessuna_si_apre_col_disavanzo_sopra_il_3() {
    let alto = giocata(dati(con_pde(disavanzo_alto(senza_shock()), "nessuna")), 3);
    assert!(disavanzo_2026(&alto) > 3.0, "{}", disavanzo_2026(&alto));
    assert_eq!(alto.economia().regole_ue.pde, StatoPde::Aperta);
    let base = giocata(dati(con_pde(senza_shock(), "nessuna")), 3);
    assert!(disavanzo_2026(&base) < 3.0);
    assert_eq!(base.economia().regole_ue.pde, StatoPde::Nessuna);
}

#[test]
fn pde_nessuna_si_apre_con_debito_e_deviazione_rilevante() {
    // Senza clausola la deviazione e' nulla: a renderla rilevante e' il conto alto.
    let f = |conto: f64| {
        con_ingresso(
            con_ingresso(con_pde(senza_shock(), "nessuna"), "conto_controllo", conto),
            "clausola_nazionale_margine",
            0.0,
        )
    };
    let p = giocata(dati(f(3.0)), 3);
    let d = disavanzo_2026(&p);
    assert!((0.5..3.0).contains(&d), "{d}");
    assert_eq!(p.economia().regole_ue.pde, StatoPde::Aperta);
    let basso = giocata(dati(f(0.1)), 3);
    assert_eq!(basso.economia().regole_ue.pde, StatoPde::Nessuna);
}

#[test]
fn pde_aperta_senza_deviazione_va_sospesa() {
    let p = giocata(dati(con_pde(senza_shock(), "aperta")), 3);
    assert_eq!(p.economia().regole_ue.pde, StatoPde::Sospesa);
}

#[test]
fn pde_sospesa_rientra_a_nessuna_sotto_il_3() {
    // Il disavanzo mobile di base al primo controllo e' 3,04%: ci vuole un po' piu' di entrate.
    let p = giocata(
        dati(con_ingresso(senza_shock(), "altre_entrate_pil", 12.6)),
        3,
    );
    let mobile = disavanzo_mobile(p.economia());
    assert!(disavanzo_2026(&p) < 3.0 && mobile < 3.0, "{mobile}");
    assert_eq!(p.economia().regole_ue.pde, StatoPde::Nessuna);
    let alto = giocata(dati(disavanzo_alto(senza_shock())), 3);
    assert_eq!(alto.economia().regole_ue.pde, StatoPde::Sospesa);
}

#[test]
fn pde_sanzionata_rientra_a_sospesa_senza_deviazione() {
    let f = con_ingresso(
        con_ingresso(
            con_pde(senza_shock(), "sanzionata"),
            "clausola_nazionale_margine",
            1.5,
        ),
        "conto_controllo",
        0.0,
    );
    // Il controllo (turno 3) toglie la sanzione; il turno 4 ha già i conti senza.
    let p = giocata(dati(f), 4);
    let riferimento = giocata(base_senza_shock(), 4);
    let e = p.economia();
    assert_eq!(e.regole_ue.pde, StatoPde::Sospesa);
    assert_eq!(e.conti.spese.ammende, 0);
    assert_eq!(
        e.conti.entrate.fondi_coesione,
        riferimento.economia().conti.entrate.fondi_coesione
    );
}

#[test]
fn la_clausola_vale_solo_fino_all_anno_indicato() {
    // Il controllo sull'anno 2027 gira chiudendo il settimo turno.
    let deviazione = |f: FileDati| giocata(dati(f), 7).economia().regole_ue.ultima_deviazione;
    let con = deviazione(senza_shock());
    let senza = deviazione(sostituisci(
        senza_shock(),
        "scenario",
        "clausola_nazionale_fino_a = 2028",
        "clausola_nazionale_fino_a = 2026",
    ));
    assert!((senza - con - 0.6).abs() < 1e-9, "{senza} {con}");
}

#[test]
fn il_conto_di_controllo_non_scende_sotto_zero() {
    let f = con_ingresso(
        con_ingresso(senza_shock(), "clausola_nazionale_margine", 1.5),
        "conto_controllo",
        0.0,
    );
    let p = giocata(dati(f), 3);
    let r = &p.economia().regole_ue;
    assert_eq!(r.conto_controllo, 0.0);
    assert!(r.ultima_deviazione < 0.0, "{}", r.ultima_deviazione);
}

#[test]
fn il_bilancio_approvato_cresce_dell_inflazione_se_sotto_il_tetto() {
    let sanita = |p: &Partita| {
        p.economia()
            .bilancio
            .dicasteri
            .iter()
            .find(|d| d.id == "sanita")
            .unwrap()
            .spesa
    };
    let f = con_ingresso(senza_shock(), "inflazione", 1.0);
    let s26 = sanita(&Partita::nuova(dati(f.clone()), 1));
    let s27 = sanita(&giocata(dati(f), 2));
    assert!(
        (s27 - (s26 as f64 * 1.010).round() as i64).abs() <= 1,
        "{s26} {s27}"
    );
}

#[test]
fn oltre_il_3_di_disavanzo_lo_spread_sale_di_2_4_pb_per_punto() {
    let d = dati(disavanzo_alto(senza_shock()));
    let m = |n: &str| d.manopola(n);
    let mut p = Partita::nuova(d.clone(), 1);
    p.chiudi_turno();
    let e = p.economia();
    let disavanzo = disavanzo_mobile(e);
    assert!(disavanzo > 3.0, "{disavanzo}");
    let pil: f64 = e.storico[e.storico.len() - 4..]
        .iter()
        .map(|t| t.pil as f64)
        .sum();
    let b = e.debito as f64 / pil * 100.0;
    let fondamentali =
        m("spread_s0") + 100.0 * (m("spread_beta_1") * b + m("spread_beta_2") * b * b);
    let capacita = m("spread_capacita") * (3.0 - p.misure().capacita_economia);
    let atteso = fondamentali.max(m("spread_minimo")) + 2.4 * (disavanzo - 3.0) + capacita;
    assert!((e.spread - atteso).abs() < 0.05, "{} {atteso}", e.spread);
}

#[test]
fn lo_spread_non_supera_il_massimo() {
    let f = inadempiente(con_ingresso(
        con_ingresso(senza_shock(), "debito_pil", 200.0),
        "premio_sfiducia",
        400.0,
    ));
    let p = giocata(dati(f), 1);
    assert_eq!(p.economia().spread, 1000.0);
}

#[test]
fn il_tpi_vale_con_nessuna_e_aperta_ma_non_con_intimata_e_sanzionata() {
    for (stato, tpi) in [
        ("nessuna", true),
        ("aperta", true),
        ("intimata", false),
        ("sanzionata", false),
    ] {
        let f = con_pde(con_ingresso(senza_shock(), "premio_sfiducia", 200.0), stato);
        let premio = giocata(dati(f), 1).economia().premio_sfiducia;
        assert_eq!(premio == 0.0, tpi, "{stato}: {premio}");
    }
}

#[test]
fn il_tpi_vale_anche_al_turno_zero() {
    let turno_zero = |premio: f64| {
        let p = Partita::nuova(
            dati(con_ingresso(senza_shock(), "premio_sfiducia", premio)),
            1,
        );
        (p.economia().spread, p.economia().premio_sfiducia)
    };
    assert_eq!(turno_zero(200.0), turno_zero(0.0));
}

#[test]
fn al_turno_zero_nessuna_ammenda() {
    let d = dati(inadempiente(senza_shock()));
    let mut p = Partita::nuova(d.clone(), 1);
    assert_eq!(p.economia().conti.spese.ammende, 0);
    let pil_annuo = p.economia().pil_trimestrale * 4.0 * 1e9;
    p.chiudi_turno();
    let atteso = d.manopola("ammenda_pde") / 100.0 * pil_annuo;
    let ammende = p.economia().conti.spese.ammende;
    assert!(
        ammende > 0 && (ammende as f64 - atteso).abs() <= 1.0,
        "{ammende} {atteso}"
    );
}

#[test]
fn il_tasso_medio_converge_verso_il_rendimento() {
    let d = base_senza_shock();
    let iniziale = d.scenario().costo_medio_debito.valore;
    let mut p = Partita::nuova(d.clone(), 1);
    for _ in 0..40 {
        p.chiudi_turno();
    }
    let e = p.economia();
    let rendimento = e.tasso_bce + d.manopola("premio_termine_bund") + e.spread / 100.0;
    assert!(
        (e.tasso_medio_debito - rendimento).abs() < (e.tasso_medio_debito - iniziale).abs(),
        "{} {rendimento} {iniziale}",
        e.tasso_medio_debito
    );
}

#[test]
fn le_entrate_seguono_il_gap_con_la_loro_elasticita() {
    // IRPEF del turno 2 su un potenziale di quel turno, e gap con cui e' stata calcolata:
    // il rapporto vale 1 + ε·gap/100. Il gap iniziale di -4 si e' gia' riassorbito in parte.
    let rapporto = |gap: f64| {
        let d = dati(con_ingresso(senza_shock(), "output_gap", gap));
        let quota = d.scenario().irpef_pil.valore;
        let mut p = Partita::nuova(d, 1);
        p.chiudi_turno();
        let e = p.economia();
        let (potenziale, gap_turno) = (
            e.pil_potenziale_reale * e.livello_prezzi * 4.0 * 1e9,
            e.output_gap[0],
        );
        p.chiudi_turno();
        let irpef = p.economia().conti.entrate.irpef as f64;
        (irpef / (quota / 100.0 * potenziale / 4.0), gap_turno)
    };
    let ((r_crisi, g_crisi), (r_base, g_base)) = (rapporto(-4.0), rapporto(0.0));
    let calo = (1.0 - r_crisi / r_base) * 100.0;
    let elasticita = dati(senza_shock()).manopola("elasticita_irpef");
    assert!(g_crisi < -3.0 && g_base.abs() < 0.5, "{g_crisi} {g_base}");
    assert!(
        (calo - elasticita * (g_base - g_crisi)).abs() < 0.1,
        "{calo} {}",
        elasticita * (g_base - g_crisi)
    );
}

fn annata_2027(p: &Partita) -> sim_core::economia::Annata {
    let a = p.economia().pensioni.iter().find(|a| a.anno == 2027);
    a.expect("annata 2027 dopo il secondo turno").clone()
}

#[test]
fn regime_quota_base_universale_somma_la_quota() {
    let d = dati(sostituisci(
        senza_shock(),
        "disposizioni",
        "partenza = \"contributivo\"",
        "partenza = \"quota_base_universale\"",
    ));
    let mut p = Partita::nuova(d.clone(), 1);
    p.chiudi_turno();
    // Il salario medio che usa il turno 2 e' quello dello stato dopo il turno 1.
    let e = p.economia();
    let forze: f64 = d.regioni().iter().map(|r| r.forze_lavoro).sum::<f64>() * 1e3;
    let occupati = forze * (1.0 - e.disoccupazione_nazionale(&d) / 100.0);
    let salario = d.manopola("quota_redditi_lavoro") * 4.0 * e.pil_trimestrale * 1e9 / occupati;
    p.chiudi_turno();
    let atteso =
        d.manopola("quota_base_universale") + d.manopola("sostituzione_contributivo") * salario;
    let importo = annata_2027(&p).importo;
    assert!((importo as f64 - atteso).abs() <= 1.0, "{importo} {atteso}");
}

#[test]
fn sussidi_universali_valgono_1_6_volte() {
    let sussidi = |regime: &str| {
        let f = sostituisci(
            senza_shock(),
            "disposizioni",
            "partenza = \"assicurativo\"",
            &format!("partenza = \"{regime}\""),
        );
        giocata(dati(f), 1).economia().conti.spese.sussidi as f64
    };
    let rapporto = sussidi("universale") / sussidi("assicurativo");
    assert!((rapporto - 1.6).abs() < 1e-6, "{rapporto}");
}

#[test]
fn la_pensione_minima_fa_da_pavimento() {
    // Sostituzione al minimo, cosi' che 20.000 euro stiano sopra il 60% del salario.
    let f = imposta(
        imposta(
            senza_shock(),
            "scenario",
            "[pensione_minima]\nvalore = ",
            "20000.0",
        ),
        "manopole",
        "[sostituzione_contributivo]\nvalore = ",
        "0.3",
    );
    let p = giocata(dati(f), 2);
    assert_eq!(annata_2027(&p).importo, 20_000);
}
