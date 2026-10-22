mod comune;
use comune::{con_ingresso, dati_base, file_base, senza_shock};
use sim_core::caso::{Caso, Flusso};
use sim_core::{DatiGioco, FileDati, Partita};

fn giocata(f: &FileDati, seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(f).unwrap(), seme);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

/// IS e Phillips a mano (economia-italiana-parametri:73-79), senza shock, con energia e
/// domanda estera a zero: un turno da `output_gap` 1,0 e inflazioni 4,2 / 3,4 / 3,0.
#[test]
fn is_e_phillips_a_valori_noti() {
    let f = con_ingresso(senza_shock(), "output_gap", 1.0);
    let d = DatiGioco::da_testi(&f).unwrap();
    let s = d.scenario();
    assert_eq!((s.energia.valore, s.domanda_estera.valore), (0.0, 0.0));
    let m = |n: &str| d.manopola(n);

    let (gap, pi, t3, t2) = (1.0, 4.2, 3.4, 3.0);
    let r = s.tasso_bce.valore - pi - m("tasso_naturale_reale");
    // Bilancio fermo in nominale: l'impulso del primo turno è l'erosione reale della spesa
    // discrezionale (dicasteri + altre prestazioni), in punti di PIL potenziale nominale.
    let pil = s.pil_trimestrale.valore;
    let gp = d.dicasteri().iter().map(|x| x.spesa_pil).sum::<f64>() / 100.0 * pil
        + s.altre_prestazioni.valore / 4.0;
    let impulso = -100.0 * (pi / 400.0) * gp / (pil / (1.0 + gap / 100.0));
    let y = (m("is_gap_1") + m("is_gap_2")) * gap
        + m("is_tasso") * 2.0 * r
        + m("moltiplicatore_fiscale") * impulso;
    let a = m("ancoraggio_aspettative");
    let attesa_pi = (1.0 - a)
        * (m("phillips_1") * pi
            + m("phillips_2_4") * (t3 + t2 + 2.0) / 3.0
            + m("phillips_5_8") * 2.0)
        + a * 2.0
        + m("phillips_gap") * gap;

    let e = giocata(&f, 1, 1).economia().clone();
    assert!(
        (e.output_gap[0] - y).abs() < 1e-9,
        "{} {y}",
        e.output_gap[0]
    );
    assert!(
        (e.inflazione[0] - attesa_pi).abs() < 1e-9,
        "{} {attesa_pi}",
        e.inflazione[0]
    );
}

#[test]
fn cinque_estrazioni_fisse_per_turno() {
    for (nome, f) in [
        ("senza shock", senza_shock()),
        ("dati di base", file_base()),
    ] {
        let turni = 3;
        let p = giocata(&f, 7, turni);
        let mut atteso = Caso::nuovo(7);
        for _ in 0..5 * turni {
            atteso.normale(Flusso::Economia);
        }
        let salvato: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
        assert_eq!(
            salvato["caso"],
            serde_json::to_value(&atteso).unwrap(),
            "{nome}"
        );
    }
}

#[test]
fn ar1_energia_e_domanda_estera() {
    let d = dati_base();
    let (pe, pd) = (
        d.manopola("persistenza_energia"),
        d.manopola("persistenza_domanda_estera"),
    );
    let f = con_ingresso(
        con_ingresso(senza_shock(), "energia", 20.0),
        "domanda_estera",
        10.0,
    );
    for (turni, k) in [(1, 1), (2, 2)] {
        let e = giocata(&f, 1, turni).economia().clone();
        assert!(
            (e.energia - pe.powi(k) * 20.0).abs() < 1e-9,
            "energia dopo {turni}"
        );
        assert!(
            (e.domanda_estera - pd.powi(k) * 10.0).abs() < 1e-9,
            "estera dopo {turni}"
        );
    }
}
