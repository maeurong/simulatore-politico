//! Giudizio retrospettivo: ogni gruppo giudica i partiti di governo su lavoro, prezzi,
//! trasferimenti, imposte e disoccupazione, ciascuna voce col suo responsabile per ruolo.

use super::{Condizione, Elettorato, Fascia, PER_REGIONE};
use crate::dati::{DatiGioco, Ruolo};
use crate::economia::Economia;
use crate::partiti::Governo;

/// Asimmetria del malcontento: il peggioramento pesa `1/ρ` volte il miglioramento.
pub(crate) fn asimmetrica(x: f64, rho: f64) -> f64 {
    if x > 0.0 {
        x * 2.0 * rho / (1.0 + rho)
    } else {
        x * 2.0 / (1.0 + rho)
    }
}

impl Elettorato {
    /// Aggiorna il giudizio sui partiti di governo dalla variazione `reddito_prima → reddito` e
    /// dalla disoccupazione contro `ultima_disoccupazione`, poi accoda la crescita alle aspettative.
    pub fn giudizio(&mut self, dati: &DatiGioco, e: &Economia, governo: &Governo) {
        let d = dati.elettorato();
        let rho = dati.manopola("asimmetria_giudizio");
        let q = dati.manopola("quota_colpa_presidente");
        let (sigma_r, sigma_u) = (
            dati.manopola("scala_reddito"),
            dati.manopola("scala_disoccupazione"),
        );
        let lambda = 0.5_f64.powf(1.0 / dati.manopola("emivita_giudizio"));
        let finestra = dati.manopola("finestra_aspettative").round() as usize;
        let pres = governo.presidente;
        let imposte = governo.partito_del_ruolo(dati, Ruolo::ColpaImposteImpiegoPubblico);
        let lavoro = governo.partito_del_ruolo(dati, Ruolo::ColpaDisoccupazioneTrasferimenti);
        let colpa =
            |resp: usize, p: usize| q * f64::from(p == pres) + (1.0 - q) * f64::from(p == resp);
        for (i, g) in self.gruppi.iter_mut().enumerate() {
            let (r, c) = (
                i / PER_REGIONE,
                Condizione::TUTTE[i % PER_REGIONE / Fascia::TUTTE.len()],
            );
            let (ora, prima) = (g.reddito, g.reddito_prima);
            let y0 = prima.netto();
            // Base degenere (netto o paniere non positivi): nessun termine di confronto.
            let sensato = y0 > 0.0 && prima.paniere > 0.0;
            let punti = |x: f64| if sensato { 100.0 * x / y0 } else { 0.0 };
            let aspettativa = if g.crescite.is_empty() {
                0.0
            } else {
                g.crescite.iter().sum::<f64>() / g.crescite.len() as f64
            };
            let crescita = 100.0 * (ora.reale() / prima.reale() - 1.0);
            g.crescite.push(if sensato && crescita.is_finite() {
                crescita
            } else {
                0.0
            });
            g.crescite
                .drain(..g.crescite.len().saturating_sub(finestra));
            let peso = |voce: &str| {
                d.interessi
                    .get(&c)
                    .and_then(|m| m.get(voce))
                    .copied()
                    .unwrap_or(0.0)
            };
            let (w_reddito, w_disoccupazione) = (peso("reddito"), peso("disoccupazione"));
            let responsabile_lavoro = if c == Condizione::DipendentePubblico {
                imposte
            } else {
                pres
            };
            let prezzi = if sensato {
                -100.0 * (ora.paniere / prima.paniere - 1.0)
            } else {
                0.0
            };
            let du = e.regioni[r].disoccupazione - self.ultima_disoccupazione[r];
            let voci = [
                (
                    punti(ora.primario - prima.primario) / sigma_r,
                    w_reddito,
                    responsabile_lavoro,
                ),
                ((prezzi - aspettativa) / sigma_r, w_reddito, pres),
                (
                    punti(ora.trasferimenti - prima.trasferimenti) / sigma_r,
                    w_reddito,
                    lavoro,
                ),
                (
                    -punti(ora.imposte - prima.imposte) / sigma_r,
                    w_reddito,
                    imposte,
                ),
                (-du / sigma_u, w_disoccupazione, lavoro),
            ];
            for (p, gp) in g.giudizio.iter_mut().enumerate() {
                if governo.al_governo(p) {
                    let nuovo: f64 = voci
                        .iter()
                        .map(|&(x, w, resp)| w * asimmetrica(x, rho) * colpa(resp, p))
                        .sum();
                    *gp = lambda * *gp + nuovo;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn asimmetria_degli_autori() {
        assert!((asimmetrica(1.0, 1.0 / 3.0) - 0.5).abs() < 1e-12);
        assert!((asimmetrica(-1.0, 1.0 / 3.0) + 1.5).abs() < 1e-12);
    }

    #[test]
    fn miglioramento_e_peggioramento_uguali_lasciano_un_saldo_negativo() {
        let rho = 1.0 / 3.0;
        assert!(asimmetrica(1.0, rho) + asimmetrica(-1.0, rho) < 0.0);
    }
}
