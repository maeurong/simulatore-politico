//! Comandi del giocatore alla partita.

use crate::economia::Bilancio;
use crate::errore::Errore;
use crate::partita::Partita;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Comando {
    ChiediDimissioni {
        dicastero: String,
    },
    ScegliSuccessore {
        dicastero: String,
        indice: u8,
    },
    Ferma {
        dicastero: String,
    },
    Dirotta {
        dicastero: String,
        atto: Option<String>,
        regione: Option<String>,
    },
    FermaCantiere {
        dicastero: String,
        atto: String,
    },
    Accogli {
        dicastero: String,
    },
    Rinvia {
        dicastero: String,
    },
    Respingi {
        dicastero: String,
    },
    /// Presenta la legge di bilancio del turno, sostituendo quella già presentata; l'aula la
    /// approva in chiusura.
    PresentaBilancio {
        bilancio: Bilancio,
    },
}

impl Partita {
    /// Applica un comando del giocatore.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero in causa, `Bilancio` col guasto della legge di
    /// bilancio; la partita resta identica.
    pub fn comanda(&mut self, c: Comando) -> Result<(), Errore> {
        if let Comando::PresentaBilancio { bilancio } = c {
            return self.presenta_bilancio(&bilancio);
        }
        let accolta = self.con_contesto(|ministri, cx| ministri.comanda(cx, c))?;
        if let Some(importo) = accolta {
            self.aggiungi_decreto(importo);
        }
        Ok(())
    }
}
