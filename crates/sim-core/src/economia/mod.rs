//! Stato dell'economia: parte dall'ingresso dello scenario, i blocchi del motore lo fanno avanzare.

pub mod bce;
pub mod congiuntura;
pub mod lavoro;
pub mod leontief;

pub use bce::AreaEuro;

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::partita::Trimestre;
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
    /// Scostamento % del prezzo degli input importati, AR(1).
    pub energia: f64,
    /// Scostamento % della domanda estera, AR(1).
    pub domanda_estera: f64,
    /// Indici, 1 al turno zero.
    pub prezzi_settori: Vec<f64>,
    /// Punti di output gap per settore.
    pub gap_settori: Vec<f64>,
    /// Punti annualizzati, l'ultima.
    pub inflazione_importata: f64,
    pub area_euro: AreaEuro,
    /// Punti di PIL potenziale; 0 fino al Task 7.
    pub impulso_fiscale: f64,
    /// Tasso reale meno naturale del turno prima di quello in corso.
    pub tasso_reale_precedente: f64,
}

/// Obiettivo di inflazione della BCE: i trimestri prima del T2 2026 ci stanno sopra.
const OBIETTIVO_INFLAZIONE: f64 = 2.0;

impl Economia {
    pub fn iniziale(dati: &DatiGioco) -> Economia {
        let s = dati.scenario();
        let gap = s.output_gap.valore;
        let (somma, pesi) = dati.regioni().iter().fold((0.0, 0.0), |(somma, pesi), r| {
            let w = r.forze_lavoro;
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
            energia: s.energia.valore,
            domanda_estera: s.domanda_estera.valore,
            prezzi_settori: prezzi_settori(dati, 0.0),
            gap_settori: vec![gap; dati.struttura().settori.len()],
            inflazione_importata: 0.0,
            area_euro: AreaEuro {
                inflazione_resto: s.inflazione.valore,
                gap_resto: 0.0,
            },
            impulso_fiscale: 0.0,
            tasso_reale_precedente: s.tasso_bce.valore
                - s.inflazione.valore
                - dati.manopola("tasso_naturale_reale"),
        }
    }

    // Ordine dei blocchi del trimestre (spec): misure (piano 5) → finanza pubblica e spesa
    // obbligatoria → domanda finale per settore → Leontief → IS/Phillips → Okun per regione →
    // redditi dei gruppi e panieri (piano 4) → tasso BCE → SFC → spread, regole UE.
    // `data` è il trimestre che il turno chiude. Qui: dalla domanda finale per settore a Okun e BCE.
    pub(crate) fn passo(&mut self, dati: &DatiGioco, caso: &mut Caso, _data: Trimestre) {
        // Cinque estrazioni fisse ogni turno, anche con sigma 0.
        let mut shock = |sigma: &str| caso.normale(Flusso::Economia) * dati.manopola(sigma);
        let shock_domanda = shock("sigma_domanda");
        let shock_produttivita = shock("sigma_produttivita");
        let shock_energia = shock("sigma_energia");
        let shock_domanda_estera = shock("sigma_domanda_estera");
        let shock_bce = shock("sigma_bce");
        let domanda_estera_prima = self.domanda_estera;
        self.energia = dati.manopola("persistenza_energia") * self.energia + shock_energia;
        self.domanda_estera = dati.manopola("persistenza_domanda_estera") * self.domanda_estera
            + shock_domanda_estera;

        let st = dati.struttura();
        let d0 = domanda_riferimento(dati);
        let x_estera = quote_pil(dati).2 * self.domanda_estera / 100.0;
        let d: Vec<f64> = d0
            .iter()
            .zip(&st.esportazioni)
            .map(|(d, e)| d + e * x_estera)
            .collect();
        let x = leontief::produzione(&st.a, &d);
        let x0 = leontief::produzione(&st.a, &d0);
        let (tot, tot0) = (x.iter().sum::<f64>(), x0.iter().sum::<f64>());

        let prima = prezzo_consumi(dati, &self.prezzi_settori);
        self.prezzi_settori = prezzi_settori(dati, self.energia);
        self.inflazione_importata =
            400.0 * (prezzo_consumi(dati, &self.prezzi_settori) / prima - 1.0);

        let r_1 = self.tasso_bce - self.inflazione[0] - dati.manopola("tasso_naturale_reale");
        let y = self.is(
            dati,
            (r_1, self.tasso_reale_precedente),
            self.domanda_estera - domanda_estera_prima,
            shock_domanda,
        );
        let pi = self.phillips(dati);
        self.tasso_reale_precedente = r_1;
        self.output_gap = [y, self.output_gap[0]];
        self.inflazione.rotate_right(1);
        self.inflazione[0] = pi;

        let crescita = dati.manopola("crescita_potenziale") + shock_produttivita;
        self.pil_potenziale_reale *= 1.0 + crescita / 400.0;
        self.livello_prezzi *= 1.0 + pi / 400.0;
        self.pil_trimestrale = self.pil_potenziale_reale * (1.0 + y / 100.0) * self.livello_prezzi;

        self.gap_settori = x
            .iter()
            .zip(&x0)
            .map(|(x, x0)| y + 100.0 * (x / x0 - tot / tot0))
            .collect();

        self.okun(dati);
        self.bce(dati, shock_bce);
    }
}

/// (C0, I0, X0): miliardi per trimestre a prezzi del turno zero.
fn quote_pil(dati: &DatiGioco) -> (f64, f64, f64) {
    let s = dati.scenario();
    let pil = s.pil_trimestrale.valore;
    let quota = |pct: f64| pct / 100.0 * pil;
    (
        quota(s.consumi_famiglie_pil.valore),
        quota(s.investimenti_pil.valore),
        quota(s.esportazioni_pil.valore),
    )
}

/// `d0 = consumi·C0 + investimenti·I0 + esportazioni·X0 + Σ_dicasteri vettore·spesa`.
fn domanda_riferimento(dati: &DatiGioco) -> Vec<f64> {
    let st = dati.struttura();
    let (c0, i0, x0) = quote_pil(dati);
    let pil = dati.scenario().pil_trimestrale.valore;
    let mut d: Vec<f64> = (0..st.settori.len())
        .map(|i| st.consumi[i] * c0 + st.investimenti[i] * i0 + st.esportazioni[i] * x0)
        .collect();
    for dic in dati.dicasteri() {
        let spesa = dic.spesa_pil / 100.0 * pil;
        for (i, settore) in st.settori.iter().enumerate() {
            let quota = match &dic.settori {
                Some(m) => m.get(&settore.id).copied().unwrap_or(0.0),
                None => st.spesa_pubblica[i],
            };
            d[i] += quota * spesa;
        }
    }
    d
}

/// `p = (I−Aᵀ)⁻¹ (valore aggiunto + importazioni·(1 + energia/100))`.
fn prezzi_settori(dati: &DatiGioco, energia: f64) -> Vec<f64> {
    let st = dati.struttura();
    let costo: Vec<f64> = st
        .valore_aggiunto
        .iter()
        .zip(&st.importazioni)
        .map(|(v, m)| v + m * (1.0 + energia / 100.0))
        .collect();
    leontief::prezzi(&st.a, &costo)
}

/// Media dei prezzi pesata col vettore `consumi`.
fn prezzo_consumi(dati: &DatiGioco, prezzi: &[f64]) -> f64 {
    dati.struttura()
        .consumi
        .iter()
        .zip(prezzi)
        .map(|(w, p)| w * p)
        .sum()
}
