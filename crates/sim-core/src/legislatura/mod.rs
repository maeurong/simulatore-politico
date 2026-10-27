//! La legislatura: fase, scadenza e coalizioni dell'ultimo voto.

use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

mod fondazione;
pub use fondazione::{AnteprimaFondazione, Fondazione, Posto, posti};

/// `Sciolta`: Camere sciolte, campagna, voto nella chiusura. `Trattativa`: il turno dopo il voto.
/// `Ordinaria`: governo in carica.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Fase {
    Sciolta,
    Trattativa,
    Ordinaria,
}

/// `coalizioni`: quelle dell'ultimo voto, indici di partito; alla nascita quelle dello scenario.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Legislatura {
    pub inizio: u32,
    pub scadenza: u32,
    pub fase: Fase,
    pub coalizioni: Vec<Vec<usize>>,
}

impl Legislatura {
    /// Si vota nella chiusura del `turno`: Camere sciolte o legislatura alla scadenza.
    pub fn turno_di_voto(&self, turno: u32) -> bool {
        self.fase == Fase::Sciolta || turno == self.scadenza
    }

    /// Il `turno` cade negli ultimi quattro della legislatura.
    pub fn ultimo_anno(&self, turno: u32) -> bool {
        turno.saturating_add(4) > self.scadenza
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Fine {
    SenzaSeggi,
}
