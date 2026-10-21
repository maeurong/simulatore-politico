use crate::errore::Errore;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt;

/// Testi dei file di dati, letti da chi fa I/O.
#[derive(Clone, Debug)]
pub struct FileDati {
    pub dicasteri: String,
    pub manopole: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Ruolo {
    ColpaDisoccupazioneTrasferimenti,
    ColpaImposteImpiegoPubblico,
    BilancioECredibilita,
}

impl Ruolo {
    pub const TUTTI: [Ruolo; 3] = [
        Ruolo::ColpaDisoccupazioneTrasferimenti,
        Ruolo::ColpaImposteImpiegoPubblico,
        Ruolo::BilancioECredibilita,
    ];
}

impl fmt::Display for Ruolo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Ruolo::ColpaDisoccupazioneTrasferimenti => "colpa_disoccupazione_trasferimenti",
            Ruolo::ColpaImposteImpiegoPubblico => "colpa_imposte_impiego_pubblico",
            Ruolo::BilancioECredibilita => "bilancio_e_credibilita",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dicastero {
    pub id: String,
    pub nome: String,
    pub indicatore_servizio: String,
    pub peso_spartizione: u8,
    pub regione_bersaglio: bool,
    pub ruoli: Vec<Ruolo>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Manopola {
    pub valore: f64,
    pub min: f64,
    pub max: f64,
    pub ticket: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatiGioco {
    pub dicasteri: Vec<Dicastero>,
    pub manopole: BTreeMap<String, Manopola>,
}

#[derive(Deserialize)]
struct FileDicasteri {
    dicastero: Vec<Dicastero>,
}

fn errore_toml(file: &str, e: toml::de::Error) -> Errore {
    Errore::Toml {
        file: file.into(),
        messaggio: e.to_string(),
    }
}

impl DatiGioco {
    /// Parsa e valida i testi dei file di dati.
    ///
    /// # Errors
    /// TOML malformato, dicasteri ripetuti, ruoli orfani o doppi,
    /// manopole con valore fuori dall'intervallo.
    pub fn da_testi(file: &FileDati) -> Result<DatiGioco, Errore> {
        let dicasteri = toml::from_str::<FileDicasteri>(&file.dicasteri)
            .map_err(|e| errore_toml("dicasteri.toml", e))?
            .dicastero;
        let manopole = toml::from_str::<BTreeMap<String, Manopola>>(&file.manopole)
            .map_err(|e| errore_toml("manopole.toml", e))?;

        let dati = DatiGioco {
            dicasteri,
            manopole,
        };
        dati.valida()?;
        Ok(dati)
    }

    /// Coerenza dei dati, anche per quelli letti da un salvataggio.
    ///
    /// # Errors
    /// Dicasteri ripetuti, ruoli orfani o doppi, manopole non finite
    /// o con valore fuori dall'intervallo.
    pub fn valida(&self) -> Result<(), Errore> {
        let dicasteri = &self.dicasteri;
        for (i, d) in dicasteri.iter().enumerate() {
            if dicasteri[..i].iter().any(|p| p.id == d.id) {
                return Err(Errore::DicasteroDoppio { id: d.id.clone() });
            }
        }
        for ruolo in Ruolo::TUTTI {
            let mut titolari = dicasteri.iter().filter(|d| d.ruoli.contains(&ruolo));
            match (titolari.next(), titolari.next()) {
                (None, _) => return Err(Errore::RuoloOrfano { ruolo }),
                (Some(a), Some(b)) => {
                    return Err(Errore::RuoloDoppio {
                        ruolo,
                        primo: a.id.clone(),
                        secondo: b.id.clone(),
                    });
                }
                _ => {}
            }
        }
        for (nome, m) in &self.manopole {
            let finite = m.valore.is_finite() && m.min.is_finite() && m.max.is_finite();
            if !(finite && m.min <= m.valore && m.valore <= m.max) {
                return Err(Errore::FuoriIntervallo {
                    nome: nome.clone(),
                    valore: m.valore,
                    min: m.min,
                    max: m.max,
                });
            }
        }
        Ok(())
    }
}
