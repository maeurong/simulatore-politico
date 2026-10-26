use std::collections::BTreeMap;

use rand_chacha::ChaCha8Rng;
use rand_core::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

/// Id dei flussi ChaCha: mai riordinare, solo aggiungere in coda.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Flusso {
    Economia = 0,
    Elettorato = 1,
    Parlamento = 2,
    Eventi = 3,
    Sondaggi = 4,
    PartitiIa = 5,
    ElettoriCampione = 6,
    Governo = 7,
}

/// 53 bit: esatti in un f64. Uniforme in `(0, 1]`.
fn uniforme_da(grezzo: u64) -> f64 {
    ((grezzo >> 11) as f64 + 1.0) / 9_007_199_254_740_992.0
}

/// Box-Muller su due uniformi in `(0, 1]`.
fn normale_da(u1: f64, u2: f64) -> f64 {
    (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
}

/// Generatore estemporaneo: legge solo il seme del `Caso`, non tocca i suoi flussi.
pub struct Estrattore(ChaCha8Rng);

impl Estrattore {
    pub fn uniforme(&mut self) -> f64 {
        uniforme_da(self.0.next_u64())
    }

    pub fn normale(&mut self) -> f64 {
        let u1 = self.uniforme();
        let u2 = self.uniforme();
        normale_da(u1, u2)
    }
}

/// Un `ChaCha8Rng` per flusso, creato alla prima estrazione dal seme.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Caso {
    seme: u64,
    flussi: BTreeMap<Flusso, ChaCha8Rng>,
}

impl Caso {
    pub fn nuovo(seme: u64) -> Caso {
        Caso {
            seme,
            flussi: BTreeMap::new(),
        }
    }

    pub fn seme(&self) -> u64 {
        self.seme
    }

    pub fn prossimo(&mut self, flusso: Flusso) -> u64 {
        let seme = self.seme;
        self.flussi
            .entry(flusso)
            .or_insert_with(|| {
                let mut rng = ChaCha8Rng::seed_from_u64(seme);
                rng.set_stream(flusso as u64);
                rng
            })
            .next_u64()
    }

    /// Estrattore per `chiave` sul flusso `ElettoriCampione`: stessa chiave, stessi numeri.
    /// La chiave entra nella posizione della parola di ChaCha spostata di 32 bit, che ne ha 68
    /// utili: chiavi da 2^36 in su si sovrappongono a chiavi minori (con `turno·1024 + gruppo`,
    /// oltre 67 milioni di turni).
    pub fn estemporaneo(&self, chiave: u64) -> Estrattore {
        let mut rng = ChaCha8Rng::seed_from_u64(self.seme);
        rng.set_stream(Flusso::ElettoriCampione as u64);
        rng.set_word_pos(u128::from(chiave) << 32);
        Estrattore(rng)
    }

    /// Uniforme in `(0, 1]`: mai zero, cosi' `ln` e' sempre finito.
    pub fn uniforme(&mut self, flusso: Flusso) -> f64 {
        uniforme_da(self.prossimo(flusso))
    }

    /// Normale standard, Box-Muller su due uniformi, senza cache del secondo valore.
    pub fn normale(&mut self, flusso: Flusso) -> f64 {
        let u1 = self.uniforme(flusso);
        let u2 = self.uniforme(flusso);
        normale_da(u1, u2)
    }
}
