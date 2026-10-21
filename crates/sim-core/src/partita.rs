use crate::caso::Caso;
use crate::dati::DatiGioco;
use crate::errore::Errore;
use crate::vista::{Vista, VoceDicastero};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub const VERSIONE_SCHEMA: u32 = 1;

/// Trimestre di calendario: `trimestre` va da 1 a 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Trimestre {
    pub anno: u16,
    pub trimestre: u8,
}

impl Trimestre {
    pub const TURNO_ZERO: Trimestre = Trimestre {
        anno: 2026,
        trimestre: 4,
    };

    pub fn successivo(self) -> Trimestre {
        if self.trimestre == 4 {
            Trimestre {
                anno: self.anno.saturating_add(1),
                trimestre: 1,
            }
        } else {
            Trimestre {
                anno: self.anno,
                trimestre: self.trimestre.saturating_add(1),
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Partita {
    versione_schema: u32,
    seme: u64,
    caso: Caso,
    dati: DatiGioco,
    turno: u32,
    data: Trimestre,
}

impl Partita {
    pub fn nuova(dati: DatiGioco, seme: u64) -> Partita {
        Partita {
            versione_schema: VERSIONE_SCHEMA,
            seme,
            caso: Caso::nuovo(seme),
            dati,
            turno: 0,
            data: Trimestre::TURNO_ZERO,
        }
    }

    // Ordine di chiusura: atti dei ministri → aula → motore (economia, poi elettorato) → soglie → eventi del turno dopo; voto delle elezioni dopo il motore.
    pub fn chiudi_turno(&mut self) {
        self.turno = self.turno.saturating_add(1);
        self.data = self.data.successivo();
    }

    pub fn vista(&self) -> Vista {
        Vista {
            turno: self.turno,
            data: self.data,
            dicasteri: self
                .dati
                .dicasteri
                .iter()
                .map(|d| VoceDicastero {
                    id: d.id.clone(),
                    nome: d.nome.clone(),
                })
                .collect(),
        }
    }

    /// Snapshot JSON completo: schema, seme, stato del generatore, dati.
    ///
    /// # Errors
    /// Serializzazione fallita, o stato che riletto non torna identico
    /// (per esempio una manopola NaN, scritta come `null`): chi salva non
    /// deve scrivere un file illeggibile.
    pub fn salva(&self) -> Result<String, Errore> {
        let s = serde_json::to_string(self)
            .map_err(|e| Errore::SalvataggioNonRiuscito(e.to_string()))?;
        match Partita::carica(&s) {
            Ok(riletta) if riletta == *self => Ok(s),
            Ok(_) => Err(Errore::SalvataggioNonRiuscito(
                "lo stato riletto è diverso da quello salvato".into(),
            )),
            Err(e) => Err(Errore::SalvataggioNonRiuscito(format!(
                "lo stato salvato non si rilegge: {e}"
            ))),
        }
    }

    /// # Errors
    /// JSON illeggibile, o schema diverso da `VERSIONE_SCHEMA`.
    pub fn carica(json: &str) -> Result<Partita, Errore> {
        let valore: serde_json::Value =
            serde_json::from_str(json).map_err(|e| Errore::SalvataggioIllegibile(e.to_string()))?;
        let trovata = valore
            .get("versione_schema")
            .and_then(serde_json::Value::as_u64)
            .and_then(|v| u32::try_from(v).ok())
            .unwrap_or(0);
        if trovata != VERSIONE_SCHEMA {
            return Err(Errore::SchemaDiverso {
                trovata,
                attesa: VERSIONE_SCHEMA,
            });
        }
        let partita: Partita = serde_json::from_value(valore)
            .map_err(|e| Errore::SalvataggioIllegibile(e.to_string()))?;
        if !(1..=4).contains(&partita.data.trimestre) {
            return Err(Errore::SalvataggioIllegibile(format!(
                "trimestre {} fuori da 1-4",
                partita.data.trimestre
            )));
        }
        partita.dati.valida()?;
        Ok(partita)
    }
}
