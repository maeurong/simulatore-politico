//! Posizioni che tornano all'ancora e voto logit con astensione e fedeltà al voto precedente.

use super::{Condizione, Elettorato, Fascia, PER_REGIONE, PUNTI};
use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::partiti::{Governo, Partito};
use serde::Serialize;

/// Intenzioni di voto: una voce per partito, l'ultima è l'astensione. Somma 1.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Intenzioni {
    pub per_gruppo: Vec<Vec<f64>>,
    /// Pesate con gli elettori vivi di ogni gruppo.
    pub nazionali: Vec<f64>,
}

/// Softmax con lo shift del massimo.
fn softmax(u: &[f64]) -> Vec<f64> {
    let max = u.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let e: Vec<f64> = u.iter().map(|x| (x - max).exp()).collect();
    let somma: f64 = e.iter().sum();
    e.into_iter().map(|x| x / somma).collect()
}

/// Utilità del gruppo `i` al punto `x` (1..=7): un valore per partito, poi l'astensione.
fn utilita(
    dati: &DatiGioco,
    el: &Elettorato,
    partiti: &[Partito],
    governo: &Governo,
    i: usize,
    x: f64,
) -> Vec<f64> {
    let d = dati.elettorato();
    let (beta, gamma) = (
        dati.manopola("beta_distanza"),
        dati.manopola("peso_giudizio"),
    );
    let g = &el.gruppi[i];
    let mut u: Vec<f64> = partiti
        .iter()
        .enumerate()
        .map(|(j, p)| {
            let giudizio = if governo.al_governo(j) {
                g.giudizio[j]
            } else {
                0.0
            };
            -beta * (x - p.posizione_percepita).powi(2) + p.valence + gamma * giudizio
        })
        .collect();
    let c = Condizione::TUTTE[i % PER_REGIONE / Fascia::TUTTE.len()];
    let f = Fascia::TUTTE[i % Fascia::TUTTE.len()];
    u.push(
        d.regioni[i / PER_REGIONE].astensione
            + d.astensione_condizione.get(&c).copied().unwrap_or(0.0)
            + d.astensione_fascia.get(&f).copied().unwrap_or(0.0),
    );
    u
}

/// `Σ_x h_x T_x`, con `T_x[m][k] = softmax_k(U_x + φ·[k = m])`: da chi votava `m` a chi vota `k`.
fn transizione(
    dati: &DatiGioco,
    el: &Elettorato,
    partiti: &[Partito],
    governo: &Governo,
    i: usize,
) -> Vec<Vec<f64>> {
    let phi = dati.manopola("fedelta");
    let n = partiti.len() + 1;
    let mut t = vec![vec![0.0; n]; n];
    for (k, h) in el.gruppi[i].posizione.iter().enumerate().take(PUNTI) {
        let u = utilita(dati, el, partiti, governo, i, (k + 1) as f64);
        for (m, riga) in t.iter_mut().enumerate() {
            let mut um = u.clone();
            um[m] += phi;
            for (a, b) in riga.iter_mut().zip(softmax(&um)) {
                *a += h * b;
            }
        }
    }
    t
}

/// Intenzioni per gruppo: partiti e astensione, somma 1.
pub fn intenzioni(
    dati: &DatiGioco,
    el: &Elettorato,
    partiti: &[Partito],
    governo: &Governo,
) -> Vec<Vec<f64>> {
    (0..el.gruppi.len())
        .map(|i| {
            let t = transizione(dati, el, partiti, governo, i);
            (0..t.len())
                .map(|k| {
                    el.gruppi[i]
                        .voto_precedente
                        .iter()
                        .zip(&t)
                        .map(|(v, r)| v * r[k])
                        .sum()
                })
                .collect()
        })
        .collect()
}

/// Distribuzione stazionaria di `t`: `π ← π t` da uniforme.
fn stazionaria(t: &[Vec<f64>]) -> Vec<f64> {
    let n = t.len();
    let mut pi = vec![1.0 / n as f64; n];
    for _ in 0..1000 {
        let nuovo: Vec<f64> = (0..n)
            .map(|k| pi.iter().zip(t).map(|(p, r)| p * r[k]).sum())
            .collect();
        let delta = nuovo
            .iter()
            .zip(&pi)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        pi = nuovo;
        if delta < 1e-13 {
            break;
        }
    }
    pi
}

impl Elettorato {
    /// Ancora di Friedkin-Johnsen senza influenza fra gruppi, poi rumore: sette normali per
    /// gruppo anche con rumore zero, così il flusso avanza sempre dello stesso numero.
    pub fn posizioni(&mut self, dati: &DatiGioco, caso: &mut Caso) {
        let (a, sigma) = (
            dati.manopola("ancora_posizioni"),
            dati.manopola("rumore_posizioni"),
        );
        for g in &mut self.gruppi {
            let mut somma = 0.0;
            for (h, ancora) in g.posizione.iter_mut().zip(&g.ancora) {
                let z = caso.normale(Flusso::Elettorato);
                *h = ((1.0 - a) * *h + a * ancora) * (sigma * z).exp();
                somma += *h;
            }
            g.posizione.iter_mut().for_each(|h| *h /= somma);
        }
    }

    /// Voto precedente iniziale: la stazionaria di `T_i`, così al turno zero l'intenzione
    /// coincide col voto precedente.
    pub fn avvia_voto(&mut self, dati: &DatiGioco, partiti: &[Partito], governo: &Governo) {
        let iniziali: Vec<Vec<f64>> = (0..self.gruppi.len())
            .map(|i| stazionaria(&transizione(dati, self, partiti, governo, i)))
            .collect();
        for (g, pi) in self.gruppi.iter_mut().zip(iniziali) {
            g.voto_precedente = pi;
        }
    }
}
