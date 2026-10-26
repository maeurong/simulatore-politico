mod comune;
use comune::{con_ingresso, dati_base, senza_shock};
use sim_core::{DatiGioco, FileDati, Partita};

fn giocata(f: &FileDati, seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(f).unwrap(), seme);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

#[test]
fn senza_shock_i_prezzi_dei_settori_restano_a_uno() {
    let p = giocata(&senza_shock(), 1, 4);
    for prezzo in &p.economia().prezzi_settori {
        assert!((prezzo - 1.0).abs() < 1e-9);
    }
    assert_eq!(p.economia().inflazione_importata, 0.0);
}

#[test]
fn uno_shock_energetico_alza_di_piu_i_prezzi_dell_industria() {
    let p = giocata(&con_ingresso(senza_shock(), "energia", 20.0), 1, 1);
    let e = p.economia();
    let (bte, l) = (e.prezzi_settori[1], e.prezzi_settori[6]);
    assert!(bte > l && l >= 1.0, "BTE {bte}, L {l}");
    assert!(e.inflazione_importata > 0.0);
}

#[test]
fn senza_shock_l_inflazione_torna_verso_il_due_per_cento() {
    let pi = giocata(&senza_shock(), 1, 40).economia().inflazione[0];
    assert!((pi - 2.0).abs() < 0.5, "{pi}");
}

#[test]
fn inflazione_sopra_l_obiettivo_fa_salire_il_tasso_bce() {
    assert!(giocata(&senza_shock(), 1, 1).economia().tasso_bce > 2.5);
}

#[test]
fn il_gap_si_riassorbe_senza_shock() {
    let p = giocata(&con_ingresso(senza_shock(), "output_gap", -3.0), 1, 60);
    // Con il bilancio fermo in nominale l'erosione reale è un freno permanente: il gap si
    // assesta intorno a -0,6 punti di PIL potenziale, non a zero (impulso fiscale negativo, blocco finanza pubblica).
    assert!(
        (-1.0..0.0).contains(&p.economia().output_gap[0]),
        "{:?}",
        p.economia().output_gap
    );
}

#[test]
fn il_pil_nominale_cresce_con_prezzi_e_potenziale() {
    let f = senza_shock();
    let partenza = giocata(&f, 1, 0).economia().pil_trimestrale;
    assert!(giocata(&f, 1, 4).economia().pil_trimestrale > partenza);
}

#[test]
fn semi_diversi_economie_diverse() {
    let f = comune::file_base();
    assert_ne!(giocata(&f, 1, 4).economia(), giocata(&f, 2, 4).economia());
}

#[test]
fn disoccupazione_nazionale_del_turno_zero_e_5_6() {
    let p = Partita::nuova(dati_base(), 1);
    assert!((p.economia().disoccupazione_nazionale(&dati_base()) - 5.6).abs() < 1e-9);
}

#[test]
fn una_recessione_alza_la_disoccupazione_in_ogni_regione() {
    let f = con_ingresso(senza_shock(), "output_gap", -4.0);
    let prima = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let dopo = giocata(&f, 1, 1);
    for (d, p) in dopo
        .economia()
        .regioni
        .iter()
        .zip(&prima.economia().regioni)
    {
        assert!(d.disoccupazione > p.disoccupazione, "{}", d.id);
    }
}

#[test]
fn il_gap_regionale_e_la_media_dei_settori_pesata_col_mix() {
    let d = dati_base();
    let p = giocata(&comune::file_base(), 5, 3);
    let e = p.economia();
    for (i, (r, g)) in e.regioni.iter().zip(d.regioni()).enumerate() {
        let atteso: f64 = d
            .struttura()
            .settori
            .iter()
            .zip(&e.gap_settori)
            .map(|(s, gap)| g.mix[&s.id] * gap)
            .sum::<f64>()
            + p.misure().domanda_regionale[i];
        assert!((r.gap - atteso).abs() < 1e-9, "{}", r.id);
    }
}

#[test]
fn un_settore_omesso_dal_mix_non_manda_in_panico() {
    let f = comune::sostituisci(
        comune::file_base(),
        "regioni",
        "A = 0.014219, BTE = 0.257791,",
        "BTE = 0.272010,",
    );
    giocata(&f, 1, 2);
}

fn media_pesata(d: &DatiGioco, v: impl Fn(usize) -> f64) -> f64 {
    let fl = d.regioni().iter().map(|r| r.forze_lavoro);
    let tot: f64 = fl.clone().sum();
    fl.enumerate().map(|(i, w)| w * v(i)).sum::<f64>() / tot
}

#[test]
fn i_coefficienti_riscalati_hanno_media_okun_nazionale() {
    let d = dati_base();
    let f = con_ingresso(senza_shock(), "output_gap", 2.0);
    let prima = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let dopo = giocata(&f, 1, 1);
    let (p, e) = (prima.economia(), dopo.economia());
    let dif = media_pesata(&d, |i| {
        e.regioni[i].disoccupazione - p.regioni[i].disoccupazione_strutturale
    });
    let gap = media_pesata(&d, |i| e.regioni[i].gap);
    assert!(
        (dif - d.manopola("okun_nazionale") * gap).abs() < 1e-9,
        "{dif} {gap}"
    );
}

#[test]
fn l_isteresi_sposta_la_disoccupazione_strutturale() {
    let d = dati_base();
    let f = con_ingresso(senza_shock(), "output_gap", -3.0);
    let prima = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let dopo = giocata(&f, 1, 1);
    let h = d.manopola("isteresi_disoccupazione");
    for (e, p) in dopo
        .economia()
        .regioni
        .iter()
        .zip(&prima.economia().regioni)
    {
        let atteso =
            p.disoccupazione_strutturale + h * (e.disoccupazione - p.disoccupazione_strutturale);
        assert!(
            (e.disoccupazione_strutturale - atteso).abs() < 1e-9,
            "{}",
            e.id
        );
    }
}

#[test]
fn la_disoccupazione_non_scende_sotto_lo_0_5() {
    let f = comune::sostituisci(
        con_ingresso(senza_shock(), "output_gap", 5.0),
        "regioni",
        "okun = -0.22\n",
        "okun = -5.0\n",
    );
    let p = giocata(&f, 1, 1);
    let u = p.economia().regioni.iter().map(|r| r.disoccupazione);
    assert!(u.clone().all(|u| u >= 0.5));
    assert!(u.clone().any(|u| u == 0.5), "il pavimento non scatta");
}

#[test]
fn piu_esportazioni_aiutano_di_piu_le_regioni_industriali() {
    let p = giocata(&con_ingresso(senza_shock(), "domanda_estera", 10.0), 1, 1);
    let gap = |id: &str| {
        p.economia()
            .regioni
            .iter()
            .find(|r| r.id == id)
            .unwrap()
            .gap
    };
    assert!(
        gap("veneto") > gap("lazio"),
        "veneto {} lazio {}",
        gap("veneto"),
        gap("lazio")
    );
}
