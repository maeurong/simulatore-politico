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
}
