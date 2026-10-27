//! Intesa degli alleati: un valore 0-100 per partito, letto solo per gli alleati.

use crate::dati::DatiGioco;
use crate::errore::Errore;
use crate::ministri::causa_peggiore;
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
    Spartizione,
    MinistriScontenti,
    DimissioniChieste,
    QuotaScoperta,
    SondaggiInCalo,
    RotturaImpegno,
    SfiduciaRespinta,
}

impl fmt::Display for CausaIntesa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NormaLontana => "norma lontana dalla sua posizione",
            Self::EmendamentoRespinto => "emendamento respinto",
            Self::Fiducia => "questione di fiducia",
            Self::SeggiPersi => "seggi persi con la legge elettorale",
            Self::Spartizione => "quota dei dicasteri rispetto alla richiesta",
            Self::MinistriScontenti => "ministri del suo partito scontenti",
            Self::DimissioniChieste => "dimissioni chieste a un suo ministro",
            Self::QuotaScoperta => "dicasteri sotto la sua richiesta",
            Self::SondaggiInCalo => "calo nei sondaggi al governo",
            Self::RotturaImpegno => "rottura dell'impegno pre-elettorale",
            Self::SfiduciaRespinta => "sfiducia respinta",
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
    /// Una voce per partito: l'ultimatum in corso, con la sua causa.
    pub ultimatum: Vec<Option<CausaIntesa>>,
}

impl Intesa {
    pub(crate) fn iniziale(dati: &DatiGioco, n_partiti: usize) -> Intesa {
        Intesa {
            valori: vec![dati.manopola("intesa_base"); n_partiti],
            variazioni: Vec::new(),
            ultimatum: vec![None; n_partiti],
        }
    }

    /// Alla formazione del governo l'intesa del partito `j` riparte da `base`, senza ultimatum.
    pub(crate) fn riparti(&mut self, j: usize, base: f64) {
        if let (Some(x), Some(u)) = (self.valori.get_mut(j), self.ultimatum.get_mut(j)) {
            *x = base;
            *u = None;
        }
    }

    /// L'intesa del partito `j` cambia di `delta`, limitata a 0-100, e resta la variazione
    /// applicata, non quella chiesta.
    pub(crate) fn varia(&mut self, j: usize, causa: CausaIntesa, delta: f64, turno: u32) {
        let Some(x) = self.valori.get_mut(j) else {
            return;
        };
        let prima = *x;
        *x = (*x + delta).clamp(0.0, 100.0);
        self.variazioni.push(VariazioneIntesa {
            partito: j,
            causa,
            delta: *x - prima,
            turno,
        });
    }

    /// Soglie di fine turno, sul valore prima del ritorno (scelta 8 del piano 7b): l'alleato sotto
    /// `soglia_ultimatum` e senza ultimatum lo pone, con la causa peggiore delle variazioni rimaste
    /// (`NormaLontana` se non ce ne sono); alla soglia o sopra lo ritira; chi non è alleato non ne
    /// ha. Rende gli alleati sotto `soglia_uscita`, in ordine d'indice. Poi ritorno verso
    /// `intesa_base`, limitato a 0-100; le variazioni restano due turni, come quelle dei ministri.
    pub(crate) fn soglie(&mut self, dati: &DatiGioco, turno: u32, alleati: &[usize]) -> Vec<usize> {
        self.variazioni.retain(|v| v.turno >= turno);
        let (soglia, uscita) = (
            dati.manopola("soglia_ultimatum"),
            dati.manopola("soglia_uscita"),
        );
        let mut escono = Vec::new();
        for (j, (&x, u)) in self.valori.iter().zip(&mut self.ultimatum).enumerate() {
            let alleato = alleati.contains(&j);
            if !alleato || x >= soglia {
                *u = None;
            } else if u.is_none() {
                let mie = (self.variazioni.iter()).filter(|v| v.partito == j);
                *u = Some(
                    causa_peggiore(mie.map(|v| (v.causa, v.delta)))
                        .unwrap_or(CausaIntesa::NormaLontana),
                );
            }
            if alleato && x < uscita {
                escono.push(j);
            }
        }
        let (base, ritorno) = (
            dati.manopola("intesa_base"),
            dati.manopola("ritorno_intesa"),
        );
        for x in &mut self.valori {
            *x = (*x + ritorno * (base - *x)).clamp(0.0, 100.0);
        }
        escono
    }

    /// # Errors
    /// `SalvataggioIlleggibile` se le voci non sono una per partito, un valore non è finito in
    /// 0-100 o una variazione esce dai partiti o non è finita.
    pub(crate) fn controlla(&self, n_partiti: usize) -> Result<(), Errore> {
        let buona = self.valori.len() == n_partiti
            && self.ultimatum.len() == n_partiti
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
