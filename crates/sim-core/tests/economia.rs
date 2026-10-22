mod comune;
use comune::{con_ingresso, senza_shock};
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
    assert!(
        p.economia().output_gap[0].abs() < 0.5,
        "{:?}",
        p.economia().output_gap
    );
}

#[test]
fn il_pil_nominale_cresce_con_prezzi_e_potenziale() {
    assert!(giocata(&senza_shock(), 1, 4).economia().pil_trimestrale > 576.9);
}

#[test]
fn semi_diversi_economie_diverse() {
    let f = comune::file_base();
    assert_ne!(giocata(&f, 1, 4).economia(), giocata(&f, 2, 4).economia());
}
