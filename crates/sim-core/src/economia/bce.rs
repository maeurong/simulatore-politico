//! Regola di Taylor della BCE sull'area euro, con l'Italia pesata per quota di PIL.

use super::{Economia, OBIETTIVO_INFLAZIONE};
use crate::dati::DatiGioco;
use serde::{Deserialize, Serialize};

/// Il resto dell'area euro: inflazione e gap tornano a 2 e 0 con la sola persistenza.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AreaEuro {
    pub inflazione_resto: f64,
    pub gap_resto: f64,
}

impl Economia {
    pub(super) fn bce(&mut self, dati: &DatiGioco, shock_bce: f64) {
        let m = |n: &str| dati.manopola(n);
        let rho = m("persistenza_resto_area");
        let ea = &mut self.area_euro;
        ea.inflazione_resto =
            OBIETTIVO_INFLAZIONE + rho * (ea.inflazione_resto - OBIETTIVO_INFLAZIONE);
        ea.gap_resto *= rho;
        let q = m("quota_italia_area_euro");
        let pi = q * self.inflazione[0] + (1.0 - q) * ea.inflazione_resto;
        let y = q * self.output_gap[0] + (1.0 - q) * ea.gap_resto;
        let obiettivo = m("tasso_naturale_reale")
            + pi
            + m("taylor_inflazione") * (pi - OBIETTIVO_INFLAZIONE)
            + m("taylor_gap") * y;
        let inerzia = m("taylor_inerzia");
        self.tasso_bce = inerzia * self.tasso_bce + (1.0 - inerzia) * obiettivo + shock_bce;
    }
}
