#![expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]

use sim_core::caso::{Caso, Flusso};

const VALORE_FISSATO: u64 = 12578764544318200737;

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
