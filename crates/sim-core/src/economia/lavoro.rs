//! Okun per regione con isteresi: dal gap dei settori alla disoccupazione.

use super::Economia;
use crate::dati::DatiGioco;

/// Piano 3 (assunzione): la disoccupazione non scende sotto questo livello, in %.
const DISOCCUPAZIONE_MINIMA: f64 = 0.5;

impl Economia {
    /// Media della disoccupazione regionale pesata sulle forze di lavoro.
    pub fn disoccupazione_nazionale(&self, dati: &DatiGioco) -> f64 {
        let (somma, pesi) = self
            .regioni
            .iter()
            .zip(dati.regioni())
            .fold((0.0, 0.0), |(s, p), (e, r)| {
                (s + r.forze_lavoro * e.disoccupazione, p + r.forze_lavoro)
            });
        somma / pesi
    }

    /// `gap = Σ mix·gap_settori`; `u = u* + β·gap`; `u* += isteresi·(u − u*)`.
    /// I `β` regionali sono riscalati perché la loro media pesata valga `okun_nazionale`
    /// (economia-italiana-parametri:54-69, :120).
    pub(super) fn okun(&mut self, dati: &DatiGioco) {
        let regioni = dati.regioni();
        let (somma, pesi) = regioni.iter().fold((0.0, 0.0), |(s, p), r| {
            (s + r.forze_lavoro * r.okun, p + r.forze_lavoro)
        });
        let scala = dati.manopola("okun_nazionale") / (somma / pesi);
        let isteresi = dati.manopola("isteresi_disoccupazione");
        for (e, r) in self.regioni.iter_mut().zip(regioni) {
            e.gap = dati
                .struttura()
                .settori
                .iter()
                .zip(&self.gap_settori)
                .map(|(s, gap)| r.mix.get(&s.id).copied().unwrap_or(0.0) * gap)
                .sum();
            let u =
                (e.disoccupazione_strutturale + r.okun * scala * e.gap).max(DISOCCUPAZIONE_MINIMA);
            e.disoccupazione = u;
            e.disoccupazione_strutturale += isteresi * (u - e.disoccupazione_strutturale);
        }
    }
}
