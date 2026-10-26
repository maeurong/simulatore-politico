//! Comandi del giocatore alla partita.

use crate::errore::Errore;
use crate::partita::Partita;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Comando {
    ChiediDimissioni { dicastero: String },
    ScegliSuccessore { dicastero: String, indice: u8 },
}

impl Partita {
    /// Applica un comando del giocatore.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero in causa; la partita resta identica.
    pub fn comanda(&mut self, c: Comando) -> Result<(), Errore> {
        self.ministri.comanda(&self.dati, c)
    }
}
