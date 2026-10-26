//! Aliquote e scaglioni IRPEF: ciò che la legge di bilancio può cambiare del prelievo.
//! Fonti: economia-italiana-parametri:98-106, finanza-pubblica:19-21.

use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Scaglione IRPEF: `aliquota` marginale sul reddito fino a `fino_a` euro annui; l'ultimo
/// scaglione non ha tetto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Scaglione {
    pub fino_a: Option<i64>,
    pub aliquota: f64,
}

/// Le aliquote che il giocatore può cambiare.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Aliquote {
    pub irpef: Vec<Scaglione>,
    pub iva: f64,
    pub ires: f64,
    pub contributi: f64,
}

/// Aliquota massima ammessa.
const MASSIMA: f64 = 0.9;

impl Aliquote {
    /// # Errors
    /// Un messaggio che nomina la voce colpevole: `scaglioni`, `irpef`, `iva`, `ires` o `contributi`.
    pub fn valida(&self) -> Result<(), String> {
        let s = &self.irpef;
        let tetti_ok = s.split_last().is_some_and(|(ultimo, prima)| {
            ultimo.fino_a.is_none()
                && prima.iter().all(|x| x.fino_a.is_some_and(|t| t > 0))
                && prima.windows(2).all(|w| w[0].fino_a < w[1].fino_a)
        });
        if !tetti_ok {
            return Err(
                "scaglioni: servono tetti positivi e crescenti, e l'ultimo scaglione senza tetto"
                    .into(),
            );
        }
        let in_range = |a: f64| (0.0..=MASSIMA).contains(&a);
        if !s.iter().all(|x| in_range(x.aliquota)) {
            return Err(format!("irpef: aliquota fuori da 0-{MASSIMA}"));
        }
        for (nome, a) in [
            ("iva", self.iva),
            ("ires", self.ires),
            ("contributi", self.contributi),
        ] {
            if !in_range(a) {
                return Err(format!("{nome}: aliquota fuori da 0-{MASSIMA}"));
            }
        }
        Ok(())
    }
}

/// IRPEF annua in euro su `reddito`, a scaglioni marginali; 0 per redditi non positivi.
pub fn irpef(scaglioni: &[Scaglione], reddito: f64) -> f64 {
    let mut sotto = 0.0;
    let mut dovuta = 0.0;
    for s in scaglioni {
        let sopra = s.fino_a.map_or(f64::INFINITY, |t| t as f64);
        dovuta += (reddito.min(sopra) - sotto).max(0.0) * s.aliquota;
        sotto = sopra;
    }
    dovuta
}

/// Elasticità della base imponibile: `((1 − τ)/(1 − τ₀))^e` (finanza-pubblica:21).
pub(crate) fn eti(tau: f64, tau0: f64, elasticita: f64) -> f64 {
    ((1.0 - tau) / (1.0 - tau0)).powf(elasticita)
}

/// Gettito di una imposta rispetto a `τ₀`: `(τ/τ₀) · eti`. Con `τ₀ = 0` non c'è rapporto.
pub(crate) fn fattore(tau: f64, tau0: f64, elasticita: f64) -> f64 {
    let rapporto = if tau0 > 0.0 { tau / tau0 } else { 1.0 };
    rapporto * eti(tau, tau0, elasticita)
}

#[cfg(test)]
mod test {
    use super::*;

    fn scaglioni() -> Vec<Scaglione> {
        [(Some(28_000), 0.23), (Some(50_000), 0.33), (None, 0.43)]
            .map(|(fino_a, aliquota)| Scaglione { fino_a, aliquota })
            .to_vec()
    }

    fn con(irpef: Vec<Scaglione>) -> Aliquote {
        Aliquote {
            irpef,
            iva: 0.22,
            ires: 0.24,
            contributi: 0.33,
        }
    }

    #[test]
    fn aliquota_piu_alta_piu_gettito_ma_meno_che_in_proporzione() {
        let f = fattore(0.24, 0.22, 0.25);
        assert!(f > 1.0 && f < 0.24 / 0.22, "{f}");
        assert_eq!(fattore(0.22, 0.22, 0.25), 1.0);
    }

    #[test]
    fn la_validazione_nomina_la_voce() {
        assert!(con(scaglioni()).valida().is_ok());
        let mut a = con(scaglioni());
        a.irpef[1].fino_a = Some(20_000);
        assert!(a.valida().err().unwrap_or_default().contains("scaglioni"));
        assert!(
            con(vec![])
                .valida()
                .err()
                .unwrap_or_default()
                .contains("scaglioni")
        );
        let mut a = con(scaglioni());
        a.irpef[2].fino_a = Some(90_000);
        assert!(a.valida().err().unwrap_or_default().contains("scaglioni"));
        let mut a = con(scaglioni());
        a.irpef[0].aliquota = 0.95;
        assert!(a.valida().err().unwrap_or_default().contains("irpef"));
        for (nome, ritocco) in [
            (
                "iva",
                (|a: &mut Aliquote| a.iva = f64::NAN) as fn(&mut Aliquote),
            ),
            ("ires", |a| a.ires = -0.1),
            ("contributi", |a| a.contributi = 1.0),
        ] {
            let mut a = con(scaglioni());
            ritocco(&mut a);
            assert!(
                a.valida().err().unwrap_or_default().starts_with(nome),
                "{nome}"
            );
        }
    }
}
