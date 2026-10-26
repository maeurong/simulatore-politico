//! Giudizio retrospettivo: ogni gruppo giudica i partiti di governo sul reddito reale (lavoro,
//! prezzi, trasferimenti e imposte, una voce sola), sulla disoccupazione e sugli indicatori di
//! servizio e l'occupazione pubblica, col responsabile per ruolo o per dicastero.

use super::{Condizione, Elettorato, FINESTRA_MASSIMA, PER_REGIONE, condizione_di};
use crate::dati::{DatiGioco, Ruolo};
use crate::economia::Economia;
use crate::grandezze::Grandezze;
use crate::partiti::Governo;

/// Coefficiente dell'asimmetria per il segno di `x`: `2ρ/(1+ρ)` se `x > 0`, altrimenti `2/(1+ρ)`.
fn coefficiente(x: f64, rho: f64) -> f64 {
    (if x > 0.0 { 2.0 * rho } else { 2.0 }) / (1.0 + rho)
}

/// Asimmetria del malcontento: il peggioramento pesa `1/ρ` volte il miglioramento.
pub(crate) fn asimmetrica(x: f64, rho: f64) -> f64 {
    x * coefficiente(x, rho)
}

impl Elettorato {
    /// Aggiorna il giudizio sui partiti di governo dalla variazione `reddito_prima → reddito` e
    /// dalla disoccupazione contro `ultima_disoccupazione` e dalla variazione delle grandezze
    /// contro `prima`, poi accoda la crescita alle aspettative.
    pub fn giudizio(
        &mut self,
        dati: &DatiGioco,
        e: &Economia,
        governo: &Governo,
        grandezze: &Grandezze,
    ) {
        let d = dati.elettorato();
        let rho = dati.manopola("asimmetria_giudizio");
        let q = dati.manopola("quota_colpa_presidente");
        let (sigma_r, sigma_u) = (
            dati.manopola("scala_reddito"),
            dati.manopola("scala_disoccupazione"),
        );
        let lambda = 0.5_f64.powf(1.0 / dati.manopola("emivita_giudizio"));
        let finestra =
            (dati.manopola("finestra_aspettative").round() as usize).min(FINESTRA_MASSIMA);
        let pres = governo.presidente;
        let partito_imposte = governo.partito_del_ruolo(dati, Ruolo::ColpaImposteImpiegoPubblico);
        let partito_lavoro =
            governo.partito_del_ruolo(dati, Ruolo::ColpaDisoccupazioneTrasferimenti);
        let colpa =
            |resp: usize, p: usize| q * f64::from(p == pres) + (1.0 - q) * f64::from(p == resp);
        let scala = dati.manopola("scala_servizio");
        // Per ogni grandezza con un responsabile: voce, variazione scalata col verso, partito.
        let servizi: Vec<(&str, f64, usize)> = (dati.grandezze().iter().enumerate())
            .filter_map(|(k, gr)| {
                let resp = if gr.id == "occupazione_pubblica" {
                    partito_imposte
                } else {
                    let i =
                        (dati.dicasteri().iter()).position(|d| d.indicatore_servizio == gr.id)?;
                    governo.dicasteri.get(i).copied()?
                };
                let delta = gr.verso.segno() * grandezze.variazione(dati, k) / scala;
                Some((gr.id.as_str(), delta, resp))
            })
            .collect();
        for (i, g) in self.gruppi.iter_mut().enumerate() {
            let (r, c) = (i / PER_REGIONE, condizione_di(i));
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
                partito_imposte
            } else {
                pres
            };
            let prezzi = if sensato {
                -100.0 * (ora.paniere / prima.paniere - 1.0)
            } else {
                0.0
            };
            // Reddito reale contro l'aspettativa: una voce sola, con l'asimmetria scelta dal
            // segno del totale; ogni componente porta la colpa al suo responsabile.
            let componenti = [
                (punti(ora.primario - prima.primario), responsabile_lavoro),
                (prezzi - aspettativa, pres),
                (
                    punti(ora.trasferimenti - prima.trasferimenti),
                    partito_lavoro,
                ),
                (-punti(ora.imposte - prima.imposte), partito_imposte),
            ];
            let dy = componenti.iter().map(|c| c.0).sum::<f64>() / sigma_r;
            let fattore = w_reddito * coefficiente(dy, rho) / sigma_r;
            let du = -(e.regioni[r].disoccupazione - self.ultima_disoccupazione[r]) / sigma_u;
            let disoccupazione = w_disoccupazione * asimmetrica(du, rho);
            for (p, gp) in g.giudizio.iter_mut().enumerate() {
                if governo.al_governo(p) {
                    let reddito: f64 = componenti.iter().map(|&(x, resp)| x * colpa(resp, p)).sum();
                    let servizio: f64 = (servizi.iter())
                        .map(|&(voce, dx, resp)| peso(voce) * asimmetrica(dx, rho) * colpa(resp, p))
                        .sum();
                    *gp = lambda * *gp
                        + fattore * reddito
                        + disoccupazione * colpa(partito_lavoro, p)
                        + servizio;
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
