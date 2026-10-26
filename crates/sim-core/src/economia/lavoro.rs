//! Okun per regione con isteresi: dal gap dei settori alla disoccupazione.

use super::{Economia, EconomiaRegione};
use crate::dati::DatiGioco;
use crate::misure::Misure;

/// Piano 3 (assunzione): la disoccupazione non scende sotto questo livello, in %.
const DISOCCUPAZIONE_MINIMA: f64 = 0.5;

impl Economia {
    /// Media della disoccupazione regionale pesata sulle forze di lavoro.
    pub fn disoccupazione_nazionale(&self, dati: &DatiGioco) -> f64 {
        self.media_regionale(dati, |e| e.disoccupazione)
    }

    /// Media pesata sulle forze di lavoro di una grandezza regionale.
    pub(super) fn media_regionale(
        &self,
        dati: &DatiGioco,
        f: impl Fn(&EconomiaRegione) -> f64,
    ) -> f64 {
        let (somma, pesi) = self
            .regioni
            .iter()
            .zip(dati.regioni())
            .fold((0.0, 0.0), |(s, p), (e, r)| {
                (s + r.forze_lavoro * f(e), p + r.forze_lavoro)
            });
        somma / pesi
    }

    /// I `β` regionali, riscalati perché la loro media pesata valga `okun_nazionale`
    /// (economia-italiana-parametri:54-69, :120).
    pub fn beta_okun(&self, dati: &DatiGioco) -> Vec<f64> {
        let regioni = dati.regioni();
        let (somma, pesi) = regioni.iter().fold((0.0, 0.0), |(s, p), r| {
            (s + r.forze_lavoro * r.okun, p + r.forze_lavoro)
        });
        let scala = dati.manopola("okun_nazionale") / (somma / pesi);
        regioni.iter().map(|r| r.okun * scala).collect()
    }

    /// `gap = Σ mix·gap_settori + domanda_regionale`; `u = max(minimo, u* + ds + β·gap)`;
    /// `u* += isteresi·(u − ds − u*)`, con `ds` la disoccupazione strutturale delle misure.
    pub(super) fn okun(&mut self, dati: &DatiGioco, misure: &Misure) {
        let beta = self.beta_okun(dati);
        let isteresi = dati.manopola("isteresi_disoccupazione");
        for (i, (e, r)) in self.regioni.iter_mut().zip(dati.regioni()).enumerate() {
            e.gap = dati
                .struttura()
                .settori
                .iter()
                .zip(&self.gap_settori)
                .map(|(s, gap)| r.mix.get(&s.id).copied().unwrap_or(0.0) * gap)
                .sum::<f64>()
                + misure.domanda_regionale[i];
            let ds = misure.disoccupazione_strutturale[i];
            let u =
                (e.disoccupazione_strutturale + ds + beta[i] * e.gap).max(DISOCCUPAZIONE_MINIMA);
            e.disoccupazione = u;
            e.disoccupazione_strutturale += isteresi * (u - ds - e.disoccupazione_strutturale);
        }
    }
}
