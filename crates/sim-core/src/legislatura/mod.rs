//! La legislatura: fase, scadenza e coalizioni dell'ultimo voto.

use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

mod coalizione;
mod fondazione;
mod voto;
pub use coalizione::{CausaDisponibilita, Cessione, Dichiarazione, VoceDisponibilita};
pub(crate) use coalizione::{Quadro, cedi, coalizioni_al_voto, dichiarazione};
pub use fondazione::{AnteprimaFondazione, Fondazione, Posto, posti};
pub use voto::{EsitoElezioni, Incarico, VotiPartito};
pub(crate) use voto::{incarico, voti_per_regione};

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
/// `dichiarazione`: la coalizione pre-elettorale del turno del voto, cancellata dal voto.
/// `elezioni`: gli esiti dei voti, il più recente per primo. `totali`: i seggi di Camera e Senato
/// del parlamento in carica. `incarico`: a chi va dopo l'ultimo voto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Legislatura {
    pub inizio: u32,
    pub scadenza: u32,
    pub fase: Fase,
    pub coalizioni: Vec<Vec<usize>>,
    pub dichiarazione: Option<Dichiarazione>,
    pub elezioni: Vec<EsitoElezioni>,
    pub totali: (u32, u32),
    pub incarico: Option<Incarico>,
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
