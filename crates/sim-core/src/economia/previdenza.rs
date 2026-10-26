//! Spesa obbligatoria: pensioni per annate di pensionati e sussidi ai disoccupati.

use super::Economia;
use super::finanza::{MLD, euro};
use crate::dati::DatiGioco;
use crate::disposizioni::Leve;
use crate::partita::Trimestre;
use crate::scenario::RegimePensioni;
use serde::{Deserialize, Serialize};

/// Pensionati andati in pensione nello stesso anno, col loro importo annuo medio in euro.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Annata {
    pub anno: u16,
    pub regime: RegimePensioni,
    pub pensionati: i64,
    pub importo: i64,
}

/// Forze di lavoro in persone.
fn forze_lavoro(dati: &DatiGioco) -> f64 {
    dati.regioni().iter().map(|r| r.forze_lavoro).sum::<f64>() * 1e3
}

impl Economia {
    fn disoccupati(&self, dati: &DatiGioco) -> f64 {
        forze_lavoro(dati) * self.disoccupazione_nazionale(dati) / 100.0
    }

    /// Una sola annata (2026) col regime di partenza e tutti i pensionati; il sussidio per
    /// disoccupato deriva dalla spesa.
    pub(super) fn avvia_previdenza(&mut self, dati: &DatiGioco) {
        let s = dati.scenario();
        let pensionati = s.pensionati.valore;
        self.pensioni = vec![Annata {
            anno: Trimestre::TURNO_ZERO.anno,
            regime: RegimePensioni::da_leva(Leve::partenza(dati).regime_pensioni),
            pensionati: euro(pensionati),
            importo: euro(s.spesa_pensioni.valore * MLD / pensionati),
        }];
        self.sussidio_per_disoccupato = euro(s.spesa_sussidi.valore * MLD / self.disoccupati(dati));
    }

    /// A gennaio: uscite e indicizzazione delle annate in essere, poi entra l'annata nuova col
    /// regime, l'età e la pensione minima delle leve.
    pub(super) fn previdenza(&mut self, dati: &DatiGioco, data: Trimestre, leve: &Leve) {
        if data.trimestre != 1 {
            return;
        }
        let uscite = dati.manopola("uscite_pensionati");
        let media = self.inflazione[..4].iter().sum::<f64>() / 4.0;
        let fattore = 1.0 + leve.indicizzazione_pensioni * media / 100.0;
        for a in &mut self.pensioni {
            a.pensionati = euro(a.pensionati as f64 * (1.0 - uscite));
            a.importo = euro(a.importo as f64 * fattore);
        }
        // Annate quasi estinte: fuori, per non trascinare righe di pochi pensionati.
        self.pensioni.retain(|a| a.pensionati >= 10);

        let eta = leve.eta_pensione as usize;
        let indice = eta.saturating_sub(usize::from(
            data.anno.saturating_sub(Trimestre::TURNO_ZERO.anno),
        ));
        let coorte = f64::from(dati.struttura().piramide[indice.min(100)]);
        let salario = self.salario_medio(dati);
        let minima = dati.scenario().pensione_minima.valore * leve.pensione_minima;
        let sost = |m: &str| dati.manopola(m) * salario;
        let regime = RegimePensioni::da_leva(leve.regime_pensioni);
        let importo = match regime {
            RegimePensioni::Retributivo => minima.max(sost("sostituzione_retributivo")),
            RegimePensioni::Contributivo => minima.max(sost("sostituzione_contributivo")),
            RegimePensioni::QuotaBaseUniversale => {
                dati.manopola("quota_base_universale") + sost("sostituzione_contributivo")
            }
        };
        self.pensioni.push(Annata {
            anno: data.anno,
            regime,
            pensionati: euro(dati.manopola("copertura_pensionamento") * coorte),
            importo: euro(importo),
        });
    }

    /// Euro annui per occupato: quota dei redditi da lavoro sul PIL annuo.
    pub(crate) fn salario_medio(&self, dati: &DatiGioco) -> f64 {
        let occupati = forze_lavoro(dati) * (1.0 - self.disoccupazione_nazionale(dati) / 100.0);
        dati.manopola("quota_redditi_lavoro") * 4.0 * self.pil_trimestrale * MLD / occupati
    }

    /// Euro annui per pensionato.
    pub(crate) fn pensione_media(&self) -> f64 {
        let pensionati: f64 = self.pensioni.iter().map(|a| a.pensionati as f64).sum();
        let spesa: f64 = self
            .pensioni
            .iter()
            .map(|a| a.pensionati as f64 * a.importo as f64)
            .sum();
        spesa / pensionati.max(1.0)
    }

    /// Euro annui per disoccupato, col coefficiente dei sussidi.
    pub(crate) fn sussidio_effettivo(&self, leve: &Leve) -> f64 {
        self.sussidio_per_disoccupato as f64 * leve.sussidi
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
    pub(super) fn spesa_sussidi(&self, dati: &DatiGioco, leve: &Leve) -> i64 {
        euro(self.disoccupati(dati) * self.sussidio_effettivo(leve) / 4.0)
    }
}
