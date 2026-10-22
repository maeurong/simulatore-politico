#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

use sim_core::caso::{Caso, Flusso};

const VALORE_FISSATO: u64 = 12578764544318200737;
const NORMALE_FISSATA: u64 = 4605675623877093340;

#[test]
fn stesso_seme_stessa_sequenza() {
    let (mut a, mut b) = (Caso::nuovo(42), Caso::nuovo(42));
    for _ in 0..100 {
        assert_eq!(a.prossimo(Flusso::Eventi), b.prossimo(Flusso::Eventi));
    }
}

#[test]
fn flussi_indipendenti() {
    let (mut a, mut b) = (Caso::nuovo(42), Caso::nuovo(42));
    for _ in 0..10 {
        b.prossimo(Flusso::Eventi);
    }
    assert_eq!(a.prossimo(Flusso::Economia), b.prossimo(Flusso::Economia));
}

#[test]
fn flussi_diversi_danno_numeri_diversi() {
    let mut c = Caso::nuovo(42);
    assert_ne!(c.prossimo(Flusso::Economia), c.prossimo(Flusso::Eventi));
}

#[test]
fn caso_serializzato_riprende_dallo_stesso_punto() {
    let mut a = Caso::nuovo(7);
    for _ in 0..5 {
        a.prossimo(Flusso::Sondaggi);
    }
    let mut b: Caso = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
    assert_eq!(a.prossimo(Flusso::Sondaggi), b.prossimo(Flusso::Sondaggi));
}

#[test]
fn caso_passa_da_value_e_riprende_dallo_stesso_punto() {
    let mut a = Caso::nuovo(7);
    for _ in 0..5 {
        a.prossimo(Flusso::Sondaggi);
    }
    let v = serde_json::to_value(&a).unwrap();
    let mut b: Caso = serde_json::from_value(v).unwrap();
    assert_eq!(a.prossimo(Flusso::Sondaggi), b.prossimo(Flusso::Sondaggi));
}

#[test]
fn flusso_mai_estratto_riparte_dallo_stato_iniziale() {
    let mut a = Caso::nuovo(9);
    a.prossimo(Flusso::Economia);
    let mut b: Caso = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
    assert_eq!(
        b.prossimo(Flusso::Sondaggi),
        Caso::nuovo(9).prossimo(Flusso::Sondaggi)
    );
}

#[test]
fn semi_estremi() {
    Caso::nuovo(0).prossimo(Flusso::Economia);
    Caso::nuovo(u64::MAX).prossimo(Flusso::PartitiIa);
}

// Fallisce se un aggiornamento di rand_chacha cambia i numeri
// (docs/ricerca/tecnica/architettura-simulazione.md:19).
#[test]
fn valore_fissato_dell_algoritmo() {
    assert_eq!(Caso::nuovo(42).prossimo(Flusso::Economia), VALORE_FISSATO);
}

#[test]
fn normale_ha_media_zero_e_varianza_uno() {
    let mut c = Caso::nuovo(42);
    let n = 20_000;
    let xs: Vec<f64> = (0..n).map(|_| c.normale(Flusso::Economia)).collect();
    let media = xs.iter().sum::<f64>() / f64::from(n);
    let varianza = xs.iter().map(|x| (x - media).powi(2)).sum::<f64>() / f64::from(n);
    assert!(media.abs() < 0.03, "{media}");
    assert!((varianza - 1.0).abs() < 0.05, "{varianza}");
}

#[test]
fn uniforme_mai_zero_e_mai_sopra_uno() {
    let mut c = Caso::nuovo(0);
    for _ in 0..10_000 {
        let u = c.uniforme(Flusso::Economia);
        assert!(u > 0.0 && u <= 1.0);
    }
}

#[test]
fn normale_valore_fissato() {
    assert_eq!(
        Caso::nuovo(42).normale(Flusso::Economia).to_bits(),
        NORMALE_FISSATA
    );
}

#[test]
fn il_seme_sta_nel_caso() {
    assert_eq!(Caso::nuovo(7).seme(), 7);
}
