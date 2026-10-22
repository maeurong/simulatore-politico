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

    /// Uniforme in `(0, 1]`: mai zero, cosi' `ln` e' sempre finito.
    pub fn uniforme(&mut self, flusso: Flusso) -> f64 {
        // 53 bit: esatti in un f64.
        let alti = self.prossimo(flusso) >> 11;
        (alti as f64 + 1.0) / 9_007_199_254_740_992.0
    }

    /// Normale standard, Box-Muller su due uniformi, senza cache del secondo valore.
    pub fn normale(&mut self, flusso: Flusso) -> f64 {
        let u1 = self.uniforme(flusso);
        let u2 = self.uniforme(flusso);
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}
