//! Comandi del giocatore alla partita.

use crate::economia::Bilancio;
use crate::errore::Errore;
use crate::parlamento::ValoreProposto;
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
    /// approva in chiusura. La `spesa` di ogni voce è il totale voluto: chi cambia salari,
    /// acquisti o margine la ricalcola, altrimenti la differenza finisce nel resto.
    PresentaBilancio {
        bilancio: Bilancio,
    },
    /// Presenta una legge: la mette in calendario fino al voto.
    PresentaLegge {
        valori: Vec<ValoreProposto>,
    },
    /// Accoglie o respinge l'emendamento del partito `partito` (id) sulla misura.
    Emendamento {
        misura: u32,
        partito: String,
        accogli: bool,
    },
    /// Pone la questione di fiducia: la misura si vota a fine turno.
    PoniFiducia {
        misura: u32,
    },
}

impl Partita {
    /// Applica un comando del giocatore.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero o della disposizione in causa, `Bilancio` col guasto
    /// della legge di bilancio; la partita resta identica.
    pub fn comanda(&mut self, c: Comando) -> Result<(), Errore> {
        match c {
            Comando::PresentaBilancio { bilancio } => return self.presenta_bilancio(&bilancio),
            Comando::PresentaLegge { valori } => return self.presenta_legge(&valori),
            Comando::Emendamento {
                misura,
                partito,
                accogli,
            } => return self.emendamento(misura, &partito, accogli),
            Comando::PoniFiducia { misura } => return self.poni_fiducia(misura),
            _ => {}
        }
        let accolta = self.con_contesto(|ministri, cx| ministri.comanda(cx, c))?;
        if let Some(importo) = accolta {
            self.aggiungi_decreto(importo);
        }
        Ok(())
    }
}
