//! Golden bit a bit di lavoro ed età dell'elettorato: i valori sono stati misurati su c56787d
//! (prima dell'estrazione di `spostamenti_lavoro` e del refactor di `Elettorato::eta`). Il valore a
//! 14 turni è stato rimisurato su 5e7e867 (piano 9: deriva, situazioni, eventi consumano estrazioni
//! e muovono le grandezze); quello a 2 turni è rimasto uguale.

mod comune;
use sim_core::Partita;

/// FNV-1a a 64 bit sui bit dei f64: basta un bit diverso per cambiare l'impronta.
fn impronta(p: &Partita) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut bit = |x: f64| {
        for b in x.to_bits().to_le_bytes() {
            h = (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    };
    for g in &p.elettorato().gruppi {
        bit(g.quota);
        g.posizione.iter().for_each(|&x| bit(x));
        g.ancora.iter().for_each(|&x| bit(x));
        g.giudizio.iter().for_each(|&x| bit(x));
    }
    h
}

fn dopo(turni: usize) -> Partita {
    let mut p = Partita::nuova(comune::dati_base(), 1);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

// 2 turni: il lavoro si è mosso, nessun gennaio ancora passato dopo il primo; 14 turni: almeno
// tre passaggi di gennaio, quindi anche l'età.
#[test]
fn lavoro_ed_eta_identici_a_prima_dell_estrazione() {
    assert_eq!(
        (impronta(&dopo(2)), impronta(&dopo(14))),
        (10_129_470_773_385_014_276, 4_452_738_675_040_110_957)
    );
}
