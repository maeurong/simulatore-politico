//! Prime stime dei dati del trimestre appena chiuso, con la revisione della precedente.

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Percentuali; crescita e inflazione annualizzate, disavanzo in % del PIL del trimestre.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Stima {
    pub crescita_pil: f64,
    pub disoccupazione: f64,
    pub inflazione: f64,
    pub disavanzo: f64,
}

impl Stima {
    fn vera(dati: &DatiGioco, e: &Economia, crescita_pil: f64) -> Stima {
        Stima {
            crescita_pil,
            disoccupazione: e.disoccupazione_nazionale(dati),
            inflazione: e.inflazione[0],
            disavanzo: 100.0 * e.disavanzo_trimestre as f64 / (e.pil_trimestrale * 1e9),
        }
    }

    pub(super) fn e_finita(&self) -> bool {
        [
            self.crescita_pil,
            self.disoccupazione,
            self.inflazione,
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
}

impl Statistica {
    /// Turno zero: il trimestre prima di `TURNO_ZERO`; lo scenario non dà la crescita, vale 0.
    pub(super) fn iniziale(dati: &DatiGioco, e: &Economia) -> Statistica {
        let vera = Stima::vera(dati, e, 0.0);
        Statistica {
            trimestre: Trimestre::TURNO_ZERO.precedente(),
            prima_stima: vera.clone(),
            vera,
            revisione: None,
        }
    }

    /// Quattro estrazioni, anche con errori zero. `reale_prima` = `pil_trimestrale /
    /// livello_prezzi` letto prima del passo dell'economia.
    pub fn aggiorna(
        &mut self,
        dati: &DatiGioco,
        caso: &mut Caso,
        e: &Economia,
        reale_prima: f64,
        data: Trimestre,
    ) {
        let reale = e.pil_trimestrale / e.livello_prezzi;
        let vera = Stima::vera(dati, e, 400.0 * (reale / reale_prima - 1.0));
        let mut errore =
            |nome: &str, v: f64| v + dati.manopola(nome) * caso.normale(Flusso::Sondaggi);
        let prima_stima = Stima {
            crescita_pil: errore("errore_stima_pil", vera.crescita_pil),
            disoccupazione: errore("errore_stima_disoccupazione", vera.disoccupazione),
            inflazione: errore("errore_stima_inflazione", vera.inflazione),
            disavanzo: errore("errore_stima_disavanzo", vera.disavanzo),
        };
        let vecchia = std::mem::replace(
            self,
            Statistica {
                trimestre: data,
                prima_stima,
                vera,
                revisione: None,
            },
        );
        self.revisione = Some(Revisione {
            trimestre: vecchia.trimestre,
            prima_stima: vecchia.prima_stima,
            riveduta: vecchia.vera,
        });
    }
}
