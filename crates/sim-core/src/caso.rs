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

const NUMERO_FLUSSI: usize = 6;

/// Un `ChaCha8Rng` per flusso, indicizzato dal discriminante di `Flusso`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Caso {
    flussi: Vec<ChaCha8Rng>,
}

impl Caso {
    pub fn nuovo(seme: u64) -> Caso {
        let flussi = (0..NUMERO_FLUSSI as u64)
            .map(|id| {
                let mut rng = ChaCha8Rng::seed_from_u64(seme);
                rng.set_stream(id);
                rng
            })
            .collect();
        Caso { flussi }
    }

    pub fn prossimo(&mut self, flusso: Flusso) -> u64 {
        self.flussi[flusso as usize].next_u64()
    }
}
