//! Elettori campione: persone di un gruppo, estratte dalla stima del sondaggio. Leggono solo
//! ciò che sta nella `Vista` (sondaggio e partiti) e i dati, mai lo stato vero.

use crate::caso::Estrattore;
use crate::dati::DatiGioco;
use crate::elettorato::{Condizione, Fascia, PUNTI, media_pesata};
use crate::informazione::Sondaggio;
use crate::vista::VocePartito;
use serde::Serialize;
#[cfg(test)]
use ts_rs::TS;

/// Elettori estratti per gruppo.
const NUMERO: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum Motivo {
    Vicinanza,
    Leader,
    Governo,
    Protesta,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ElettoreCampione {
    pub nome: String,
    pub eta: u8,
    pub posizione: u8,
    /// Id del partito votato; `None` per l'astensione.
    pub voto: Option<String>,
    pub motivo: Motivo,
}

/// Prodotto elemento per elemento `c·r·f / n²`, rinormalizzato; una voce con `n` zero vale zero.
fn stima(c: &[f64], r: &[f64], f: &[f64], n: &[f64]) -> Vec<f64> {
    let p: Vec<f64> = c
        .iter()
        .zip(r)
        .zip(f)
        .zip(n)
        .map(|(((c, r), f), n)| if *n > 0.0 { c * r * f / (n * n) } else { 0.0 })
        .collect();
    let somma: f64 = p.iter().sum();
    if somma > 0.0 {
        p.into_iter().map(|x| x / somma).collect()
    } else {
        p
    }
}

/// Indice scelto con probabilità proporzionale a `pesi`; senza pesi positivi, quello di mezzo.
fn scegli(pesi: &[f64], u: f64) -> usize {
    let mut resto = u * pesi.iter().sum::<f64>();
    pesi.iter()
        .position(|w| {
            resto -= w;
            *w > 0.0 && resto <= 0.0
        })
        .or_else(|| pesi.iter().rposition(|w| *w > 0.0))
        .unwrap_or(pesi.len() / 2)
}

/// Indice intero uniforme in `0..lung`, da `u` in `(0, 1]`.
fn indice(u: f64, lung: usize) -> usize {
    ((u * lung as f64) as usize).min(lung.saturating_sub(1))
}

/// Tre elettori del gruppo (`regione`, `c`, `f`) estratti dalla stima del sondaggio.
/// Stima: quote `∝ q_c·q_r·q_f / q_n²`, istogramma e approvazione allo stesso modo.
pub fn genera(
    s: &Sondaggio,
    partiti: &[VocePartito],
    dati: &DatiGioco,
    regione: usize,
    c: Condizione,
    f: Fascia,
    rng: &mut Estrattore,
) -> Vec<ElettoreCampione> {
    let d = dati.elettorato();
    let (ic, ir, i_f) = (
        &s.condizioni[c as usize],
        &s.regioni[regione],
        &s.fasce[f as usize],
    );
    let uno = vec![1.0; s.istituti.len()];
    let quote_n = media_pesata(
        &uno,
        s.istituti.iter().map(|r| &r.quote[..]),
        ic.quote.len(),
    );
    let mut p = stima(&ic.quote, &ir.quote, &i_f.quote, &quote_n);
    if p.iter().sum::<f64>() <= 0.0 {
        p.clone_from(&ir.quote);
    }
    let elettori: Vec<f64> = d.regioni.iter().map(|r| r.elettori as f64).collect();
    let regionali = s.regioni.iter().map(|i| &i.posizione[..]);
    let h = stima(
        &ic.posizione,
        &ir.posizione,
        &i_f.posizione,
        &media_pesata(&elettori, regionali, PUNTI),
    );
    let tot: f64 = elettori.iter().sum();
    let appr_n = s
        .regioni
        .iter()
        .zip(&elettori)
        .map(|(i, w)| i.approvazione * w / tot)
        .sum::<f64>();
    let appr = ic.approvazione + ir.approvazione + i_f.approvazione - 2.0 * appr_n;

    let beta = dati.manopola("beta_distanza");
    let n_p = partiti.len() as f64;
    let (eta_da, eta_a) = match c {
        Condizione::Studente => (18, 25),
        Condizione::Pensionato => (65, 90),
        _ => (25, 64),
    };
    (0..NUMERO)
        .map(|_| {
            let k = scegli(&p, rng.uniforme());
            let percepita = partiti.get(k).map(|v| v.posizione_percepita);
            let pesi: Vec<f64> = (0..PUNTI)
                .map(|x| {
                    let w = percepita.map_or(1.0, |m| (-beta * (x as f64 + 1.0 - m).powi(2)).exp());
                    h[x] * w
                })
                .collect();
            let x = scegli(&pesi, rng.uniforme()) + 1;
            let eta = eta_da + indice(rng.uniforme(), eta_a - eta_da + 1);
            let nome = format!(
                "{} {}",
                d.nomi[indice(rng.uniforme(), d.nomi.len())],
                d.cognomi[indice(rng.uniforme(), d.cognomi.len())]
            );
            let (voto, motivo) = match partiti.get(k) {
                None => (None, Motivo::Protesta),
                Some(v) => {
                    let x = x as f64;
                    let distanza = |m: f64| (x - m).powi(2);
                    let vicinanza = beta
                        * (partiti
                            .iter()
                            .map(|q| distanza(q.posizione_percepita))
                            .sum::<f64>()
                            / n_p
                            - distanza(v.posizione_percepita));
                    let leader = v.valence - partiti.iter().map(|q| q.valence).sum::<f64>() / n_p;
                    let governo = if v.al_governo { appr } else { -appr };
                    let (mut motivo, mut max) = (Motivo::Vicinanza, vicinanza);
                    for (m, g) in [(Motivo::Leader, leader), (Motivo::Governo, governo)] {
                        if g > max {
                            (motivo, max) = (m, g);
                        }
                    }
                    (Some(v.id.clone()), motivo)
                }
            };
            ElettoreCampione {
                nome,
                eta: eta as u8,
                posizione: x as u8,
                voto,
                motivo,
            }
        })
        .collect()
}
