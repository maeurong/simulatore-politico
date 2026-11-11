//! Prime stime dei dati del trimestre appena chiuso, con la revisione della precedente.

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Percentuali; crescita e `inflazione` annualizzate sul trimestre prima, disavanzo in % del PIL
/// del trimestre.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Stima {
    pub crescita_pil: f64,
    pub disoccupazione: f64,
    pub inflazione: f64,
    /// Prezzi sullo stesso trimestre dell'anno prima.
    pub inflazione_annua: f64,
    pub disavanzo: f64,
}

impl Stima {
    fn vera(dati: &DatiGioco, e: &Economia, crescita_pil: f64) -> Stima {
        Stima {
            crescita_pil,
            disoccupazione: e.disoccupazione_nazionale(dati),
            inflazione: e.inflazione[0],
            inflazione_annua: e.inflazione_annua(),
            disavanzo: 100.0 * e.disavanzo_trimestre as f64 / (e.pil_trimestrale * 1e9),
        }
    }

    /// `self`, il vero, con `errore(manopola, valore)` campo per campo, nell'ordine `pil`,
    /// `disoccupazione`, `inflazione`, `disavanzo`. L'inflazione annua non ha un errore suo: i tre
    /// trimestri prima sono già riveduti, quindi è la loro composizione in `e` con la stima
    /// dell'ultimo. Con un errore suo, o con una frazione di quello del trimestre, dalle due stime
    /// si ricaverebbe il vero.
    pub(super) fn con_errore(
        &self,
        e: &Economia,
        mut errore: impl FnMut(&str, f64) -> f64,
    ) -> Stima {
        let crescita_pil = errore("errore_stima_pil", self.crescita_pil);
        let disoccupazione = errore("errore_stima_disoccupazione", self.disoccupazione);
        let inflazione = errore("errore_stima_inflazione", self.inflazione);
        Stima {
            crescita_pil,
            disoccupazione,
            inflazione,
            inflazione_annua: e.inflazione_annua_con(inflazione),
            disavanzo: errore("errore_stima_disavanzo", self.disavanzo),
        }
    }

    pub(super) fn e_finita(&self) -> bool {
        [
            self.crescita_pil,
            self.disoccupazione,
            self.inflazione,
            self.inflazione_annua,
            self.disavanzo,
        ]
        .iter()
        .all(|x| x.is_finite())
    }
}

/// La stima riveduta è il valore vero.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Revisione {
    pub trimestre: Trimestre,
    pub prima_stima: Stima,
    pub riveduta: Stima,
}

/// Stato vero: `vera` non passa mai dalla `Vista`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Statistica {
    pub trimestre: Trimestre,
    pub prima_stima: Stima,
    pub vera: Stima,
    pub revisione: Option<Revisione>,
    /// Indice dei prezzi pubblicato, 1 al turno zero: cresce con le prime stime
    /// dell'inflazione. La legge di bilancio legge questo, non il livello vero.
    pub indice_prezzi: f64,
}

impl Statistica {
    /// Turno zero: il trimestre prima di `TURNO_ZERO`; lo scenario non dà la crescita, vale 0.
    /// L'inflazione annua è il dato di scenario, non la composizione: i valori di scenario sono
    /// già variazioni sull'anno prima.
    pub(super) fn iniziale(dati: &DatiGioco, e: &Economia) -> Statistica {
        let vera = Stima {
            inflazione_annua: e.inflazione[0],
            ..Stima::vera(dati, e, 0.0)
        };
        Statistica {
            trimestre: Trimestre::TURNO_ZERO.precedente(),
            prima_stima: vera.clone(),
            vera,
            revisione: None,
            indice_prezzi: 1.0,
        }
    }

    /// Quattro estrazioni, anche con errori zero. `reale_prima` = `pil_trimestrale /
    /// livello_prezzi` letto prima del passo dell'economia.
    pub(crate) fn aggiorna(
        &mut self,
        dati: &DatiGioco,
        caso: &mut Caso,
        e: &Economia,
        reale_prima: f64,
        data: Trimestre,
    ) {
        let reale = e.pil_trimestrale / e.livello_prezzi;
        let vera = Stima::vera(dati, e, 400.0 * (reale / reale_prima - 1.0));
        let prima_stima = vera.con_errore(e, |nome, v| {
            v + dati.manopola(nome) * caso.normale(Flusso::Sondaggi)
        });
        // Inflazione annualizzata: un trimestre ne vale la radice quarta.
        // Pavimento a −99%: sotto −100% la radice non esiste e l'indice diventerebbe NaN.
        let indice_prezzi =
            self.indice_prezzi * (1.0 + (prima_stima.inflazione / 100.0).max(-0.99)).powf(0.25);
        let vecchia = std::mem::replace(
            self,
            Statistica {
                trimestre: data,
                prima_stima,
                vera,
                revisione: None,
                indice_prezzi,
            },
        );
        self.revisione = Some(Revisione {
            trimestre: vecchia.trimestre,
            prima_stima: vecchia.prima_stima,
            riveduta: vecchia.vera,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    /// `aggiorna` prende dal `Caso` quattro normali di `Flusso::Sondaggi` e nient'altro, la quarta
    /// per il disavanzo: l'inflazione annua non ne ha una sua, e ciò che si estrae dopo non si
    /// sposta.
    #[test]
    fn aggiorna_estrae_quattro_normali_dei_sondaggi() {
        let dati = dati_base();
        let e = Economia::iniziale(&dati);
        let mut caso = Caso::nuovo(1);
        let mut atteso = caso.clone();
        let z: [f64; 4] = std::array::from_fn(|_| atteso.normale(Flusso::Sondaggi));
        let mut s = Statistica::iniziale(&dati, &e);
        let reale = e.pil_trimestrale / e.livello_prezzi;
        s.aggiorna(&dati, &mut caso, &e, reale, Trimestre::TURNO_ZERO);
        assert_eq!(caso, atteso);
        let disavanzo = s.vera.disavanzo + dati.manopola("errore_stima_disavanzo") * z[3];
        assert_eq!(s.prima_stima.disavanzo, disavanzo);
    }
}
