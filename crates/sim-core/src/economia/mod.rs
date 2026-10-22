//! Stato dell'economia: parte dall'ingresso dello scenario, i blocchi del motore lo fanno avanzare.

use crate::dati::DatiGioco;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EconomiaRegione {
    pub id: String,
    pub disoccupazione: f64,
    pub disoccupazione_strutturale: f64,
    pub gap: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Economia {
    /// Miliardi, nominale.
    pub pil_trimestrale: f64,
    /// Miliardi a prezzi del turno zero.
    pub pil_potenziale_reale: f64,
    /// 1 al turno zero.
    pub livello_prezzi: f64,
    /// Corrente, precedente.
    pub output_gap: [f64; 2],
    /// Annualizzata, dalla più recente.
    pub inflazione: [f64; 8],
    pub tasso_bce: f64,
    pub regioni: Vec<EconomiaRegione>,
}

/// Obiettivo di inflazione della BCE: i trimestri prima del T2 2026 ci stanno sopra.
const OBIETTIVO_INFLAZIONE: f64 = 2.0;

impl Economia {
    pub fn iniziale(dati: &DatiGioco) -> Economia {
        let s = dati.scenario();
        let gap = s.output_gap.valore;
        let (somma, pesi) = dati.regioni().iter().fold((0.0, 0.0), |(somma, pesi), r| {
            let w = f64::from(r.forze_lavoro);
            (somma + w * r.disoccupazione_2025, pesi + w)
        });
        let spostamento = s.disoccupazione.valore - somma / pesi;
        Economia {
            pil_trimestrale: s.pil_trimestrale.valore,
            pil_potenziale_reale: s.pil_trimestrale.valore / (1.0 + gap / 100.0),
            livello_prezzi: 1.0,
            output_gap: [gap, gap],
            inflazione: [
                s.inflazione.valore,
                s.inflazione_t3.valore,
                s.inflazione_t2.valore,
                OBIETTIVO_INFLAZIONE,
                OBIETTIVO_INFLAZIONE,
                OBIETTIVO_INFLAZIONE,
                OBIETTIVO_INFLAZIONE,
                OBIETTIVO_INFLAZIONE,
            ],
            tasso_bce: s.tasso_bce.valore,
            regioni: dati
                .regioni()
                .iter()
                .map(|r| {
                    let u = r.disoccupazione_2025 + spostamento;
                    EconomiaRegione {
                        id: r.id.clone(),
                        disoccupazione: u,
                        disoccupazione_strutturale: u,
                        gap,
                    }
                })
                .collect(),
        }
    }
}
