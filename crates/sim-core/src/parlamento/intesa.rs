//! Intesa degli alleati: un valore 0-100 per partito, letto solo per gli alleati.

use crate::dati::DatiGioco;
use crate::errore::Errore;
use serde::{Deserialize, Serialize};
use std::fmt;
#[cfg(test)]
use ts_rs::TS;

/// Perché l'intesa di un alleato è cambiata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum CausaIntesa {
    NormaLontana,
    EmendamentoRespinto,
    Fiducia,
    SeggiPersi,
}

impl fmt::Display for CausaIntesa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NormaLontana => "norma lontana dalla sua posizione",
            Self::EmendamentoRespinto => "emendamento respinto",
            Self::Fiducia => "questione di fiducia",
            Self::SeggiPersi => "seggi persi con la legge elettorale",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VariazioneIntesa {
    pub partito: usize,
    pub causa: CausaIntesa,
    pub delta: f64,
    pub turno: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Intesa {
    /// Una voce per partito, nell'ordine dello scenario.
    pub valori: Vec<f64>,
    pub variazioni: Vec<VariazioneIntesa>,
}

impl Intesa {
    pub(crate) fn iniziale(dati: &DatiGioco, n_partiti: usize) -> Intesa {
        Intesa {
            valori: vec![dati.manopola("intesa_base"); n_partiti],
            variazioni: Vec::new(),
        }
    }

    /// Soglie di fine turno: ritorno verso `intesa_base`, limitato a 0-100; le variazioni restano
    /// due turni, come quelle dei ministri.
    pub(crate) fn soglie(&mut self, dati: &DatiGioco, turno: u32) {
        let (base, ritorno) = (
            dati.manopola("intesa_base"),
            dati.manopola("ritorno_intesa"),
        );
        for x in &mut self.valori {
            *x = (*x + ritorno * (base - *x)).clamp(0.0, 100.0);
        }
        self.variazioni.retain(|v| v.turno >= turno);
    }

    /// # Errors
    /// `SalvataggioIlleggibile` se le voci non sono una per partito, un valore non è finito in
    /// 0-100 o una variazione esce dai partiti o non è finita.
    pub(crate) fn controlla(&self, n_partiti: usize) -> Result<(), Errore> {
        let buona = self.valori.len() == n_partiti
            && self.valori.iter().all(|x| (0.0..=100.0).contains(x))
            && (self.variazioni.iter()).all(|v| v.partito < n_partiti && v.delta.is_finite());
        if buona {
            Ok(())
        } else {
            Err(Errore::SalvataggioIlleggibile(
                "intesa: non una voce per partito, o un valore fuori da 0-100".into(),
            ))
        }
    }
}
