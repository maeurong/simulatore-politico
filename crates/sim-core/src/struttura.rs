//! Struttura economica (`economia.toml`) e regioni (`regioni.toml`): tipi e controlli di coerenza.

use crate::dati::Dicastero;
use crate::errore::Errore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const N_SETTORI: usize = 10;
const TOLLERANZA_QUOTE: f64 = 1e-6;
const ETA_PIRAMIDE: usize = 101;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Settore {
    pub id: String,
    pub nome: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Macroarea {
    Nord,
    Centro,
    Mezzogiorno,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Regione {
    pub id: String,
    pub nome: String,
    pub macroarea: Macroarea,
    /// Migliaia di persone.
    pub forze_lavoro: f32,
    pub disoccupazione_2025: f64,
    pub okun: f64,
    /// Quota di ogni settore nel valore aggiunto della regione.
    pub mix: BTreeMap<String, f64>,
}

#[derive(Deserialize)]
pub(crate) struct FileRegioni {
    pub regione: Vec<Regione>,
}

/// Tavola intersettoriale: `a[i][j]` è l'input del settore `i` per unità prodotta dal settore `j`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Struttura {
    pub anno_tavola: u16,
    pub settori: Vec<Settore>,
    pub a: Vec<Vec<f64>>,
    pub importazioni: Vec<f64>,
    pub valore_aggiunto: Vec<f64>,
    pub consumi: Vec<f64>,
    pub investimenti: Vec<f64>,
    pub esportazioni: Vec<f64>,
    pub spesa_pubblica: Vec<f64>,
    /// Residenti per età 0..=99, poi 100 e oltre.
    pub piramide: Vec<u32>,
}

fn quote_unitarie(dove: &str, quote: impl Iterator<Item = f64>) -> Result<(), Errore> {
    if (quote.sum::<f64>() - 1.0).abs() <= TOLLERANZA_QUOTE {
        Ok(())
    } else {
        Err(Errore::QuoteNonUnitarie { dove: dove.into() })
    }
}

fn incoerente<T>(m: &str) -> Result<T, Errore> {
    Err(Errore::StrutturaIncoerente(m.into()))
}

impl Struttura {
    /// Forma, produttività, quote e id di settore di struttura, regioni e dicasteri.
    ///
    /// # Errors
    /// Vedi `Errore`: `StrutturaIncoerente`, `SettoreNonProduttivo`, `QuoteNonUnitarie`,
    /// `SettoreSconosciuto`.
    pub fn valida(&self, regioni: &[Regione], dicasteri: &[Dicastero]) -> Result<(), Errore> {
        if self.settori.len() != N_SETTORI {
            return incoerente("i settori devono essere 10");
        }
        if self.a.len() != N_SETTORI || self.a.iter().any(|r| r.len() != N_SETTORI) {
            return incoerente("la matrice a deve essere 10x10");
        }
        let vettori = [
            ("importazioni", &self.importazioni),
            ("valore_aggiunto", &self.valore_aggiunto),
            ("consumi", &self.consumi),
            ("investimenti", &self.investimenti),
            ("esportazioni", &self.esportazioni),
            ("spesa_pubblica", &self.spesa_pubblica),
        ];
        for (nome, v) in vettori {
            if v.len() != N_SETTORI {
                return incoerente(&format!("{nome} deve avere 10 voci"));
            }
        }
        if self.piramide.len() != ETA_PIRAMIDE {
            return incoerente("la piramide deve avere 101 età");
        }
        for (j, settore) in self.settori.iter().enumerate() {
            let costi: f64 = self.a.iter().map(|riga| riga[j]).sum::<f64>() + self.importazioni[j];
            if !costi.is_finite() || costi >= 1.0 {
                return Err(Errore::SettoreNonProduttivo {
                    id: settore.id.clone(),
                });
            }
        }
        for (nome, v) in &vettori[2..] {
            quote_unitarie(nome, v.iter().copied())?;
        }
        for r in regioni {
            quote_unitarie(&r.id, r.mix.values().copied())?;
        }
        for d in dicasteri {
            if let Some(s) = &d.settori {
                quote_unitarie(&d.id, s.values().copied())?;
            }
        }
        let sconosciuto = |id: &String, dove: &str| {
            if self.settori.iter().any(|s| &s.id == id) {
                Ok(())
            } else {
                Err(Errore::SettoreSconosciuto {
                    id: id.clone(),
                    dove: dove.into(),
                })
            }
        };
        for r in regioni {
            r.mix.keys().try_for_each(|id| sconosciuto(id, &r.id))?;
        }
        for d in dicasteri {
            if let Some(s) = &d.settori {
                s.keys().try_for_each(|id| sconosciuto(id, &d.id))?;
            }
        }
        Ok(())
    }
}
