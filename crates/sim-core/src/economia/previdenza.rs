//! Spesa obbligatoria: pensioni per annate di pensionati e sussidi ai disoccupati.

use super::Economia;
use crate::dati::DatiGioco;
use crate::partita::Trimestre;
use crate::scenario::{RegimePensioni, RegimeSussidi};
use serde::{Deserialize, Serialize};

const ANNO_ZERO: u16 = 2026;

/// Pensionati andati in pensione nello stesso anno, col loro importo annuo medio in euro.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Annata {
    pub anno: u16,
    pub regime: RegimePensioni,
    pub pensionati: i64,
    pub importo: i64,
}

/// Euro `f64` → euro interi.
fn euro(x: f64) -> i64 {
    x.round() as i64
}

/// Forze di lavoro in persone.
fn forze_lavoro(dati: &DatiGioco) -> f64 {
    dati.regioni().iter().map(|r| r.forze_lavoro).sum::<f64>() * 1e3
}

impl Economia {
    fn disoccupati(&self, dati: &DatiGioco) -> f64 {
        forze_lavoro(dati) * self.disoccupazione_nazionale(dati) / 100.0
    }

    /// Una sola annata (2026) con tutti i pensionati; il sussidio per disoccupato deriva dalla spesa.
    pub(super) fn avvia_previdenza(&mut self, dati: &DatiGioco) {
        let s = dati.scenario();
        let pensionati = s.pensionati.valore;
        self.pensioni = vec![Annata {
            anno: ANNO_ZERO,
            regime: s.regime_pensioni,
            pensionati: euro(pensionati),
            importo: euro(s.spesa_pensioni.valore * 1e9 / pensionati),
        }];
        self.sussidio_per_disoccupato = euro(s.spesa_sussidi.valore * 1e9 / self.disoccupati(dati));
    }

    /// A gennaio: uscite e indicizzazione delle annate in essere, poi entra l'annata nuova.
    pub(super) fn previdenza(&mut self, dati: &DatiGioco, data: Trimestre) {
        if data.trimestre != 1 {
            return;
        }
        let s = dati.scenario();
        let uscite = dati.manopola("uscite_pensionati");
        let media = self.inflazione[..4].iter().sum::<f64>() / 4.0;
        let fattore = 1.0 + s.indicizzazione_pensioni.valore * media / 100.0;
        for a in &mut self.pensioni {
            a.pensionati = euro(a.pensionati as f64 * (1.0 - uscite));
            a.importo = euro(a.importo as f64 * fattore);
        }

        let eta = s.eta_pensione.valore as usize;
        let indice = eta.saturating_sub(usize::from(data.anno.saturating_sub(ANNO_ZERO)));
        let coorte = f64::from(dati.struttura().piramide[indice.min(100)]);
        let occupati = forze_lavoro(dati) * (1.0 - self.disoccupazione_nazionale(dati) / 100.0);
        let salario =
            dati.manopola("quota_redditi_lavoro") * 4.0 * self.pil_trimestrale * 1e9 / occupati;
        let minima = s.pensione_minima.valore;
        let sost = |m: &str| dati.manopola(m) * salario;
        let importo = match s.regime_pensioni {
            RegimePensioni::Retributivo => minima.max(sost("sostituzione_retributivo")),
            RegimePensioni::Contributivo => minima.max(sost("sostituzione_contributivo")),
            RegimePensioni::QuotaBaseUniversale => {
                dati.manopola("quota_base_universale") + sost("sostituzione_contributivo")
            }
        };
        self.pensioni.push(Annata {
            anno: data.anno,
            regime: s.regime_pensioni,
            pensionati: euro(dati.manopola("copertura_pensionamento") * coorte),
            importo: euro(importo),
        });
    }

    /// Spesa pensionistica del trimestre, euro.
    pub(super) fn spesa_pensioni(&self) -> i64 {
        let annua: f64 = self
            .pensioni
            .iter()
            .map(|a| a.pensionati as f64 * a.importo as f64)
            .sum();
        euro(annua / 4.0)
    }

    /// Spesa per i sussidi del trimestre, euro.
    pub(super) fn spesa_sussidi(&self, dati: &DatiGioco) -> i64 {
        let k = match dati.scenario().regime_sussidi {
            RegimeSussidi::Assicurativo => 1.0,
            RegimeSussidi::Universale => dati.manopola("sussidio_universale"),
        };
        euro(self.disoccupati(dati) * self.sussidio_per_disoccupato as f64 * k / 4.0)
    }

    /// Pensionati totali, tutte le annate.
    pub fn pensionati(&self) -> i64 {
        self.pensioni.iter().map(|a| a.pensionati).sum()
    }
}
