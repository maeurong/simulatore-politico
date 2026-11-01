//! La cassa del partito: entrate a ogni chiusura (scelta 10 del piano 8b).

use crate::dati::DatiGioco;

/// Entrate di un turno per partito, in euro: `round((due_per_mille_turno · ultimo_voto[j] +
/// donazioni_turno · intenzione[j]) · quota_libera_entrate)`. Le quote sono fra i votanti, una
/// per partito; chi ha quota zero (lo sciolto, chi non c'era al voto) non incassa da quel lato.
pub(crate) fn entrate(dati: &DatiGioco, ultimo_voto: &[f64], intenzione: &[f64]) -> Vec<i64> {
    let due_per_mille = dati.manopola("due_per_mille_turno");
    let donazioni = dati.manopola("donazioni_turno");
    let libera = dati.manopola("quota_libera_entrate");
    (ultimo_voto.iter().zip(intenzione))
        .map(|(v, i)| ((due_per_mille * v + donazioni * i) * libera).round() as i64)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dati::dati_base;

    #[test]
    fn entrate_a_quota_zero_sono_zero() {
        assert_eq!(entrate(&dati_base(), &[0.0, 0.0], &[0.0, 0.0]), vec![0, 0]);
    }

    #[test]
    fn entrate_esatte() {
        assert_eq!(entrate(&dati_base(), &[0.5], &[0.5]), vec![682_303]);
    }
}
