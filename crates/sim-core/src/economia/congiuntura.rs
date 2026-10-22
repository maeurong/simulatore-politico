//! IS e Phillips trimestrali (Rudebusch-Svensson, coefficienti di Garnier-Wilhelmsen).

use super::{Economia, OBIETTIVO_INFLAZIONE};
use crate::dati::DatiGioco;

impl Economia {
    /// Nuovo output gap. `r_1`, `r_2`: tasso reale meno naturale, uno e due turni fa.
    pub(super) fn is(
        &self,
        dati: &DatiGioco,
        (r_1, r_2): (f64, f64),
        delta_domanda_estera: f64,
        shock_domanda: f64,
    ) -> f64 {
        let m = |n: &str| dati.manopola(n);
        m("is_gap_1") * self.output_gap[0]
            + m("is_gap_2") * self.output_gap[1]
            + m("is_tasso") * (r_1 + r_2)
            + m("moltiplicatore_fiscale") * self.impulso_fiscale
            + dati.scenario().esportazioni_pil.valore / 100.0 * delta_domanda_estera
            + shock_domanda
    }

    /// Nuova inflazione annualizzata, dal gap del turno prima e dall'inflazione importata.
    pub(super) fn phillips(&self, dati: &DatiGioco) -> f64 {
        let m = |n: &str| dati.manopola(n);
        let media = |p: &[f64]| p.iter().sum::<f64>() / p.len() as f64;
        let a = m("ancoraggio_aspettative");
        (1.0 - a)
            * (m("phillips_1") * self.inflazione[0]
                + m("phillips_2_4") * media(&self.inflazione[1..4])
                + m("phillips_5_8") * media(&self.inflazione[4..8]))
            + a * OBIETTIVO_INFLAZIONE
            + m("phillips_gap") * self.output_gap[0]
            + self.inflazione_importata
    }
}
