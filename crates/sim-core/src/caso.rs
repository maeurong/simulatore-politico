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

// Legato all'ultima variante: una variante in coda allunga l'array da sola.
const NUMERO_FLUSSI: usize = Flusso::PartitiIa as usize + 1;

/// Un `ChaCha8Rng` per flusso, indicizzato dal discriminante di `Flusso`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Caso {
    flussi: [ChaCha8Rng; NUMERO_FLUSSI],
}

impl Caso {
    pub fn nuovo(seme: u64) -> Caso {
        let flussi = std::array::from_fn(|id| {
            let mut rng = ChaCha8Rng::seed_from_u64(seme);
            rng.set_stream(id as u64);
            rng
        });
        Caso { flussi }
    }

    pub fn prossimo(&mut self, flusso: Flusso) -> u64 {
        self.flussi[flusso as usize].next_u64()
    }
}
