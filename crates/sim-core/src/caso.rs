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

// Il match e' senza `_`: una nuova variante di `Flusso` non compila finche'
// non viene elencata qui, dove va aggiornato anche il numero.
const fn numero_flussi(f: Flusso) -> usize {
    match f {
        Flusso::Economia
        | Flusso::Elettorato
        | Flusso::Parlamento
        | Flusso::Eventi
        | Flusso::Sondaggi
        | Flusso::PartitiIa => 6,
    }
}

const NUMERO_FLUSSI: usize = numero_flussi(Flusso::Economia);

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
