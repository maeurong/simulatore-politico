//! Stato dell'economia: parte dall'ingresso dello scenario, i blocchi del motore lo fanno avanzare.

pub mod bce;
pub mod congiuntura;
mod finanza;
pub mod lavoro;
mod legge_bilancio;
pub mod leontief;
mod previdenza;
mod regole_ue;
mod sfc;
mod spread;

pub use bce::AreaEuro;
pub use finanza::{Bilancio, BudgetDicastero, ConsuntivoTrimestre, ContiTrimestre, Entrate, Spese};
pub use legge_bilancio::{Aliquote, PerditaReale, RiepilogoBilancio, Scaglione, Voce, irpef};
pub(crate) use legge_bilancio::{
    FUORI_STAGIONE, bozza, eti, fattore, indicizza, livello_ok, normalizza, perdita_reale,
    perdite_scaglioni, riepilogo,
};
pub use previdenza::Annata;
pub use regole_ue::RegoleUe;
pub use sfc::{Settori, Sfc};

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::dati::Dicastero;
use crate::misure::{CAPACITA_NEUTRA, Misure};
use crate::partita::Trimestre;
use crate::struttura::Struttura;
use finanza::MLD;
pub(crate) use finanza::euro;
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
    /// Punti di PIL potenziale, del trimestre in corso.
    pub impulso_fiscale: f64,
    /// Tasso reale meno naturale del turno prima di quello in corso.
    pub tasso_reale_precedente: f64,
    /// Annuo, nominale, dell'anno in corso.
    pub bilancio: Bilancio,
    /// Bilancio dell'anno dopo, se approvato.
    pub bilancio_approvato: Option<Bilancio>,
    /// Legge di bilancio presentata nel turno, in attesa dell'aula.
    pub bilancio_presentato: Option<Bilancio>,
    /// Euro dei decreti (fondi accolti) del turno: spesi per intero nel trimestre che si chiude,
    /// fuori dal bilancio annuo.
    pub decreti: i64,
    /// Euro.
    pub debito: i64,
    /// % annuo.
    pub tasso_medio_debito: f64,
    /// Entrate e spese del trimestre appena chiuso.
    pub conti: ContiTrimestre,
    /// Spese meno entrate del trimestre appena chiuso, euro.
    pub disavanzo_trimestre: i64,
    /// Ultimi 8 trimestri, il più recente in coda.
    pub storico: Vec<ConsuntivoTrimestre>,
    /// Pensionati per anno di pensionamento.
    pub pensioni: Vec<Annata>,
    /// Euro annui, fermi in nominale.
    pub sussidio_per_disoccupato: i64,
    /// Saldi e stock dei quattro settori.
    pub sfc: Sfc,
    /// Punti base sul Bund, a fine turno.
    pub spread: f64,
    /// Punti base dentro `spread`; il TPI lo azzera.
    pub premio_sfiducia: f64,
    /// Spesa netta, conto di controllo, PDE.
    pub regole_ue: RegoleUe,
}

/// Obiettivo di inflazione della BCE; riempie anche la storia prima del T2 2026, di cui lo scenario non dà dati (assunzione).
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
        let bilancio = finanza::Bilancio::iniziale(dati);
        let debito = euro(s.debito_pil.valore / 100.0 * 4.0 * s.pil_trimestrale.valore * MLD);
        let mut e = Economia {
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
            bilancio_approvato: Some(finanza::bilancio_approvato_iniziale(dati, &bilancio)),
            bilancio,
            bilancio_presentato: None,
            decreti: 0,
            debito,
            tasso_medio_debito: s.costo_medio_debito.valore,
            conti: ContiTrimestre::default(),
            disavanzo_trimestre: 0,
            storico: Vec::new(),
            pensioni: Vec::new(),
            sussidio_per_disoccupato: 0,
            sfc: Sfc::iniziale(debito, 1.0),
            spread: 0.0,
            premio_sfiducia: s.premio_sfiducia.valore,
            regole_ue: RegoleUe::nuove(s.pde, s.conto_controllo.valore),
        };
        e.avvia_previdenza(dati);
        e.conti = finanza::conti_del_trimestre(&e, dati, &Misure::neutre(dati));
        // L'ammenda della PDE la paga il primo passo, non il turno zero.
        e.conti.spese.ammende = 0;
        e.disavanzo_trimestre = e.conti.disavanzo();
        e.avvia_regole_ue(dati);
        // Quattro trimestri prima del turno zero, tutti uguali al conto di partenza.
        e.storico = std::iter::successors(Some(Trimestre::TURNO_ZERO.precedente()), |t| {
            Some(t.precedente())
        })
        .take(4)
        .map(|data| finanza::consuntivo(&e, data))
        .collect();
        e.storico.reverse();
        // Il TPI vale anche al turno zero, come nel passo.
        if !e.regole_ue.pde.e_inadempiente() {
            e.premio_sfiducia = 0.0;
        }
        e.spread = e.spread_dallo_stato(dati, CAPACITA_NEUTRA);
        e
    }

    // Ordine: misure (lette da `Misure`, costruita da `Partita`) → finanza pubblica e spesa
    // obbligatoria → domanda finale per settore → Leontief → IS/Phillips → Okun per regione →
    // tasso BCE → SFC → storico → regole UE → spread.
    // `Elettorato::aggiorna_redditi` è chiamato da `Partita` subito dopo il passo: legge solo
    // grandezze che BCE, SFC e spread non toccano.
    // `data` è il trimestre che il turno chiude.
    pub(crate) fn passo(
        &mut self,
        dati: &DatiGioco,
        caso: &mut Caso,
        data: Trimestre,
        misure: &Misure,
    ) {
        let spesa_netta = self.finanza(dati, data, misure);
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
        let mut d: Vec<f64> = d0
            .iter()
            .zip(&st.esportazioni)
            .map(|(d, e)| d + e * x_estera)
            .collect();
        // Scostamento reale della spesa trimestrale dei dicasteri dal turno zero, in miliardi.
        let pil_annuo_0 = 4.0 * dati.scenario().pil_trimestrale.valore;
        for (dic, b) in dati.dicasteri().iter().zip(&self.bilancio.dicasteri) {
            let spesa_0 = dic.spesa_pil / 100.0 * pil_annuo_0;
            let scostamento = (b.spesa as f64 / MLD / self.livello_prezzi - spesa_0) / 4.0;
            for (i, di) in d.iter_mut().enumerate() {
                *di += quota_dicastero(st, dic, i) * scostamento;
            }
        }
        let x = leontief::produzione(&st.a, &d);
        let x0 = produzione_riferimento(dati);
        let (tot, tot0) = (x.iter().sum::<f64>(), x0.iter().sum::<f64>());

        let prima = prezzo_consumi(dati, &self.prezzi_settori);
        self.prezzi_settori = prezzi_settori(dati, self.energia + misure.prezzo_energia);
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

        let crescita =
            dati.manopola("crescita_potenziale") + misure.crescita_potenziale + shock_produttivita;
        self.pil_potenziale_reale *= 1.0 + crescita / 400.0;
        self.livello_prezzi *= 1.0 + pi / 400.0;
        self.pil_trimestrale = self.pil_potenziale_reale * (1.0 + y / 100.0) * self.livello_prezzi;

        self.gap_settori = x
            .iter()
            .zip(&x0)
            .map(|(x, x0)| y + 100.0 * (x / x0 - tot / tot0))
            .collect();

        self.okun(dati, misure);
        self.bce(dati, shock_bce);
        self.sfc(dati);
        self.storico.push(finanza::consuntivo(self, data));
        if self.storico.len() > finanza::TRIMESTRI_DI_STORIA {
            self.storico.remove(0);
        }
        self.aggiorna_regole_ue(dati, data, spesa_netta);
        self.aggiorna_spread(dati, misure.capacita_economia);
        debug_assert!(self.e_finita(), "stato non finito: {self:?}");
        debug_assert_eq!(self.residuo_sfc(), 0, "SFC non chiusa: {:?}", self.sfc);
    }

    fn e_finita(&self) -> bool {
        let scalari = [
            self.pil_trimestrale,
            self.pil_potenziale_reale,
            self.livello_prezzi,
            self.tasso_bce,
            self.energia,
            self.domanda_estera,
            self.inflazione_importata,
            self.impulso_fiscale,
            self.tasso_medio_debito,
            self.tasso_reale_precedente,
            self.area_euro.inflazione_resto,
            self.area_euro.gap_resto,
            self.sfc.livello_prezzi_area,
            self.spread,
            self.premio_sfiducia,
            self.regole_ue.conto_controllo,
            self.regole_ue.ultima_deviazione,
        ];
        scalari
            .iter()
            .chain(&self.output_gap)
            .chain(&self.inflazione)
            .chain(&self.prezzi_settori)
            .chain(&self.gap_settori)
            .chain(
                self.regioni
                    .iter()
                    .flat_map(|r| [&r.disoccupazione, &r.disoccupazione_strutturale, &r.gap]),
            )
            .all(|x| x.is_finite())
    }
}

/// Produzione di riferimento `x0 = (I−A)⁻¹ d0`, a esportazioni e prezzi del turno zero.
pub(crate) fn produzione_riferimento(dati: &DatiGioco) -> Vec<f64> {
    leontief::produzione(&dati.struttura().a, &domanda_riferimento(dati))
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
        for (i, di) in d.iter_mut().enumerate() {
            *di += quota_dicastero(st, dic, i) * spesa;
        }
    }
    d
}

/// Quota della spesa del dicastero che va al settore `i`.
fn quota_dicastero(st: &Struttura, dic: &Dicastero, i: usize) -> f64 {
    match &dic.settori {
        Some(m) => m.get(&st.settori[i].id).copied().unwrap_or(0.0),
        None => st.spesa_pubblica[i],
    }
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
