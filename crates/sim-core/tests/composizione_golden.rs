//! Golden bit a bit di lavoro ed età dell'elettorato, nato su c56787d (prima dell'estrazione di
//! `spostamenti_lavoro` e del refactor di `Elettorato::eta`): nessuno dei due valori di oggi viene
//! più da lì. Il valore a 14 turni è stato rimisurato su 5e7e867 (piano 9: deriva, situazioni,
//! eventi consumano estrazioni e muovono le grandezze) e di nuovo con gli scandali (due uniformi di
//! `Flusso::Eventi` a chiusura, poi valence e dimissioni degli scandali risolti) e con gli shock
//! (cinque uniformi di `Flusso::Eventi` in più a chiusura, che spostano le estrazioni dopo); quello
//! a 2 turni era rimasto uguale in tutti i casi. Poi tutti e due sono stati rimisurati con
//! l'inflazione di fondo (`Economia::inflazione_fondo`): i prezzi importati entrano nei ritardi
//! della Phillips solo per `secondo_giro_importata`, e l'inflazione pubblicata cambia dal secondo
//! turno; con quella manopola a 1 tornano i due valori di prima (9863557225151097556 e
//! 5723325121864969021).

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
        (10_001_238_181_506_179_722, 14_008_427_151_038_580_301)
    );
}
