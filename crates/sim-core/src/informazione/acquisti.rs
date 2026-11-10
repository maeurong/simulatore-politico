//! Informazione comprata con la cassa del partito del giocatore, al massimo una volta a turno
//! per tipo.

use super::Sondaggio;
use crate::dati::DatiGioco;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Acquisto {
    SondaggioPrivato,
}

impl Acquisto {
    pub const TUTTI: [Acquisto; 1] = [Acquisto::SondaggioPrivato];

    pub fn nome(self) -> &'static str {
        match self {
            Acquisto::SondaggioPrivato => "sondaggio privato",
        }
    }

    /// Euro, dalla manopola del tipo.
    pub fn prezzo(self, dati: &DatiGioco) -> i64 {
        match self {
            Acquisto::SondaggioPrivato => dati.manopola("prezzo_sondaggio_privato").round() as i64,
        }
    }
}

/// Un acquisto del turno col suo risultato.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Acquistato {
    SondaggioPrivato(Sondaggio),
}

impl Acquistato {
    pub fn tipo(&self) -> Acquisto {
        match self {
            Acquistato::SondaggioPrivato(_) => Acquisto::SondaggioPrivato,
        }
    }
}
