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

    /// L'ultimatum sul valore corrente: l'alleato sotto `soglia_ultimatum` e senza ultimatum lo
    /// pone, con la causa peggiore delle sue variazioni (`NormaLontana` se non ce ne sono); alla
    /// soglia o sopra lo ritira; chi non è alleato non ne ha.
    pub(crate) fn ultimatum(&mut self, dati: &DatiGioco, alleati: &[usize]) {
        let soglia = dati.manopola("soglia_ultimatum");
        for (j, (&x, u)) in self.valori.iter().zip(&mut self.ultimatum).enumerate() {
            if !alleati.contains(&j) || x >= soglia {
                *u = None;
            } else if u.is_none() {
                let mie = (self.variazioni.iter()).filter(|v| v.partito == j);
                *u = Some(
                    causa_peggiore(mie.map(|v| (v.causa, v.delta)))
                        .unwrap_or(CausaIntesa::NormaLontana),
                );
            }
        }
    }

    /// Soglie di fine turno, sul valore prima del ritorno (scelta 8 del piano 7b): l'ultimatum,
    /// con le variazioni di questo turno e del precedente, poi le variazioni del turno precedente
    /// escono (restano due turni, come quelle dei ministri). Rende gli alleati sotto
    /// `soglia_uscita`, in ordine d'indice. Poi ritorno verso `intesa_base`, limitato a 0-100.
    pub(crate) fn soglie(&mut self, dati: &DatiGioco, turno: u32, alleati: &[usize]) -> Vec<usize> {
        self.ultimatum(dati, alleati);
        self.variazioni.retain(|v| v.turno >= turno);
        let uscita = dati.manopola("soglia_uscita");
        let escono = (self.valori.iter().enumerate())
            .filter(|&(j, &x)| x < uscita && alleati.contains(&j))
            .map(|(j, _)| j)
            .collect();
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::dati::dati_base as dati;

    #[test]
    fn escono_gli_alleati_sotto_la_soglia_di_uscita() {
        let d = dati();
        let mut intesa = Intesa::iniziale(&d, 3);
        intesa.valori[1] = 24.9;
        intesa.valori[2] = 10.0;
        assert_eq!(intesa.soglie(&d, 1, &[1]), vec![1]);
        // Il non alleato a 10 non esce e non ha ultimatum.
        assert_eq!(intesa.ultimatum[2], None);
        intesa.valori[1] = 25.0;
        intesa.valori[2] = 10.0;
        assert_eq!(intesa.soglie(&d, 2, &[1]), Vec::<usize>::new());
    }

    /// La causa dell'ultimatum legge anche le variazioni del turno prima, quelle della formazione.
    #[test]
    fn la_causa_dell_ultimatum_legge_il_turno_prima() {
        let d = dati();
        let mut intesa = Intesa::iniziale(&d, 2);
        intesa.varia(1, CausaIntesa::Spartizione, -30.0, 4);
        intesa.soglie(&d, 5, &[1]);
        assert_eq!(intesa.ultimatum[1], Some(CausaIntesa::Spartizione));
    }
}
