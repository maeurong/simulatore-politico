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
    pub forze_lavoro: f64,
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

/// `x` finito e ammesso da `ok`; altrimenti `StrutturaIncoerente` col nome del campo.
fn controlla(x: f64, ok: bool, regola: &str, nome: impl FnOnce() -> String) -> Result<(), Errore> {
    if x.is_finite() && ok {
        Ok(())
    } else {
        incoerente(&format!("{} = {x}: deve essere {regola}", nome()))
    }
}

fn non_negativo(x: f64, nome: impl FnOnce() -> String) -> Result<(), Errore> {
    controlla(x, x >= 0.0, "finito e non negativo", nome)
}

impl Struttura {
    /// Forma, produttività, quote e id di settore di struttura, regioni e dicasteri.
    ///
    /// # Errors
    /// Vedi `Errore`: `StrutturaIncoerente` (anche per numeri non finiti, negativi dove non
    /// ammesso, `disoccupazione_2025` fuori da [0, 100], `okun` non negativo),
    /// `SettoreNonProduttivo`, `QuoteNonUnitarie`, `SettoreSconosciuto`.
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
        if regioni.is_empty()
            || regioni
                .iter()
                .any(|r| !(r.forze_lavoro.is_finite() && r.forze_lavoro > 0.0))
        {
            return incoerente("regioni assenti o con forze di lavoro non positive");
        }
        let id = |i: usize| self.settori[i].id.as_str();
        for (i, riga) in self.a.iter().enumerate() {
            for (j, &x) in riga.iter().enumerate() {
                non_negativo(x, || format!("a[{}][{}]", id(i), id(j)))?;
            }
        }
        for (nome, v) in &vettori {
            for (j, &x) in v.iter().enumerate() {
                non_negativo(x, || format!("{nome} del settore {}", id(j)))?;
            }
        }
        for r in regioni {
            let u = r.disoccupazione_2025;
            controlla(u, (0.0..=100.0).contains(&u), "finito in [0, 100]", || {
                format!("disoccupazione_2025 di {}", r.id)
            })?;
            controlla(r.okun, r.okun < 0.0, "finito e negativo", || {
                format!("okun di {}", r.id)
            })?;
            for (s, &q) in &r.mix {
                non_negativo(q, || format!("mix di {} per il settore {s}", r.id))?;
            }
        }
        for d in dicasteri {
            let spese = [
                ("spesa_pil", d.spesa_pil),
                ("salari_pil", d.salari_pil),
                ("acquisti_pil", d.acquisti_pil),
            ];
            for (nome, x) in spese {
                non_negativo(x, || format!("{nome} del dicastero {}", d.id))?;
            }
            for (s, &q) in d.settori.iter().flatten() {
                non_negativo(q, || {
                    format!("quota del dicastero {} per il settore {s}", d.id)
                })?;
            }
        }
        for (j, settore) in self.settori.iter().enumerate() {
            let costi: f64 = self.a.iter().map(|riga| riga[j]).sum::<f64>() + self.importazioni[j];
            if costi >= 1.0 {
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

    /// Ultimo controllo della struttura: una piramide per età 0..=100, con qualcuno fra 15 e 66
    /// anni (la forza lavoro della demografia).
    ///
    /// # Errors
    /// `StrutturaIncoerente` se non ha 101 voci o nessuno fra 15 e 66 anni.
    pub fn valida_piramide(&self) -> Result<(), Errore> {
        if self.piramide.len() != ETA_PIRAMIDE {
            incoerente("la piramide deve avere 101 età")
        } else if self.piramide[15..=66].iter().all(|&x| x == 0) {
            incoerente("la piramide non ha nessuno fra 15 e 66 anni")
        } else {
            Ok(())
        }
    }
}
