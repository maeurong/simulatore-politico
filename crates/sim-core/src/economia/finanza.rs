//! Finanza pubblica: bilancio dei dicasteri, entrate e spese del trimestre, debito, interessi,
//! impulso fiscale. Euro in `i64` (arrotondati dove un flusso tocca i conti), tassi in `f64`.

use super::Economia;
use super::legge_bilancio::{Aliquote, fattore};
use crate::dati::DatiGioco;
use crate::misure::Misure;
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub(super) const MLD: f64 = 1e9;

/// Euro annui.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct BudgetDicastero {
    pub id: String,
    pub spesa: i64,
    pub salari: i64,
    pub acquisti: i64,
    pub margine: i64,
    /// Il livello dei prezzi a cui la voce ha preso la sua cifra nominale.
    pub livello_prezzi: f64,
}

/// Cifre annue nominali, in euro.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Bilancio {
    pub anno: u16,
    pub dicasteri: Vec<BudgetDicastero>,
    pub aliquote: Aliquote,
    /// Euro annui stanziati per le emergenze; non ancora spesi da alcun evento.
    pub fondo_riserva: i64,
}

/// Euro del trimestre.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Entrate {
    pub irpef: i64,
    pub ires: i64,
    pub iva: i64,
    pub contributi: i64,
    pub altre: i64,
    pub fondi_coesione: i64,
}

/// Euro del trimestre.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Spese {
    pub dicasteri: i64,
    pub pensioni: i64,
    pub sussidi: i64,
    pub altre_prestazioni: i64,
    pub interessi: i64,
    /// Ammende della PDE, versate all'estero.
    pub ammende: i64,
    /// Saldo delle disposizioni rispetto alla partenza, con segno: sotto zero è gettito.
    pub disposizioni: i64,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ContiTrimestre {
    pub entrate: Entrate,
    pub spese: Spese,
    /// Euro del trimestre dovuti alle aliquote e agli scaglioni cambiati rispetto allo scenario.
    pub entrate_discrezionali: i64,
}

impl ContiTrimestre {
    pub fn totale_entrate(&self) -> i64 {
        let e = &self.entrate;
        e.irpef + e.ires + e.iva + e.contributi + e.altre + e.fondi_coesione
    }

    pub fn totale_spese(&self) -> i64 {
        let s = &self.spese;
        s.dicasteri
            + s.pensioni
            + s.sussidi
            + s.altre_prestazioni
            + s.interessi
            + s.ammende
            + s.disposizioni
    }

    pub fn disavanzo(&self) -> i64 {
        self.totale_spese() - self.totale_entrate()
    }

    /// Spesa che entra nell'impulso fiscale: dicasteri e altre prestazioni.
    fn spesa_discrezionale(&self) -> i64 {
        self.spese.dicasteri + self.spese.altre_prestazioni
    }
}

/// Euro del trimestre.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConsuntivoTrimestre {
    pub data: Trimestre,
    pub pil: i64,
    pub disavanzo: i64,
    pub spesa_primaria: i64,
}

pub(crate) const TRIMESTRI_DI_STORIA: usize = 8;

/// Arrotonda euro `f64` a euro interi (nessuna conversione di unità).
pub(crate) fn euro(x: f64) -> i64 {
    debug_assert!(x.is_finite(), "euro({x})");
    x.round() as i64
}

impl Bilancio {
    /// Bilancio dell'anno del turno zero: `spesa_pil` % di 4 trimestri di PIL.
    pub(crate) fn iniziale(dati: &DatiGioco) -> Bilancio {
        let pil_annuo = 4.0 * dati.scenario().pil_trimestrale.valore * MLD;
        let margine = dati.manopola("quota_margine") / 100.0;
        let quota = |pct: f64| euro(pct / 100.0 * pil_annuo);
        Bilancio {
            anno: Trimestre::TURNO_ZERO.anno,
            dicasteri: dati
                .dicasteri()
                .iter()
                .map(|d| {
                    let spesa = quota(d.spesa_pil);
                    BudgetDicastero {
                        id: d.id.clone(),
                        spesa,
                        salari: quota(d.salari_pil),
                        acquisti: quota(d.acquisti_pil),
                        margine: euro(margine * spesa as f64),
                        livello_prezzi: 1.0,
                    }
                })
                .collect(),
            aliquote: dati.scenario().aliquote.clone(),
            fondo_riserva: euro(dati.manopola("fondo_riserva") * MLD),
        }
    }

    /// Il bilancio dell'anno dopo: ogni voce dei dicasteri, col suo livello dei prezzi, e il
    /// fondo × `k`.
    pub(crate) fn scalato(&self, k: f64) -> Bilancio {
        let s = |x: i64| euro(x as f64 * k);
        Bilancio {
            anno: self.anno.saturating_add(1),
            dicasteri: self
                .dicasteri
                .iter()
                .map(|d| BudgetDicastero {
                    id: d.id.clone(),
                    spesa: s(d.spesa),
                    salari: s(d.salari),
                    acquisti: s(d.acquisti),
                    margine: s(d.margine),
                    livello_prezzi: d.livello_prezzi * k,
                })
                .collect(),
            aliquote: self.aliquote.clone(),
            fondo_riserva: s(self.fondo_riserva),
        }
    }

    pub(crate) fn spesa_totale(&self) -> i64 {
        self.dicasteri.iter().map(|d| d.spesa).sum()
    }
}

/// Bilancio dell'anno dopo il turno zero, approvato dal governo uscente: crescita nominale
/// pari all'inflazione, ma non oltre il tetto di spesa netta di quell'anno.
pub(crate) fn bilancio_approvato_iniziale(dati: &DatiGioco, base: &Bilancio) -> Bilancio {
    let s = dati.scenario();
    let tetto = s.tetto_spesa_netta(base.anno.saturating_add(1));
    base.scalato(1.0 + s.inflazione.valore.min(tetto) / 100.0)
}

/// L'unica formula di entrate e spese del trimestre, sullo stato `e` com'è ora e col
/// `bilancio` dato (di solito `e.bilancio`). `misure.riscossione` moltiplica IRPEF, IRES e IVA;
/// gli scaglioni e le aliquote del bilancio, rispetto a quelli dello scenario, muovono il gettito.
pub(crate) fn conti_del_trimestre(
    e: &Economia,
    bilancio: &Bilancio,
    dati: &DatiGioco,
    misure: &Misure,
) -> ContiTrimestre {
    let s = dati.scenario();
    let riscossione = misure.riscossione;
    let pil_potenziale = e.pil_potenziale_reale * e.livello_prezzi * 4.0 * MLD;
    // Elasticità lineare sul gap del turno prima (economia-italiana-parametri:109).
    let base = |quota: f64, elasticita: f64, risc: f64| {
        quota / 100.0 * pil_potenziale / 4.0 * (1.0 + elasticita * e.output_gap[0] / 100.0) * risc
    };
    let el = |n: &str| dati.manopola(n);
    let (a, a0) = (&bilancio.aliquote, &s.aliquote);
    let irpef_base = base(s.irpef_pil.valore, el("elasticita_irpef"), riscossione);
    let irpef = irpef_base * misure.fattore_irpef * misure.eti_irpef;
    // (entrata con `τ = τ₀`, fattore delle aliquote): l'entrata vera è il prodotto.
    let ires = (
        base(s.ires_pil.valore, el("elasticita_ires"), riscossione),
        fattore(a.ires, a0.ires, el("elasticita_imponibile_ires")),
    );
    let iva = (
        base(s.iva_pil.valore, el("elasticita_iva"), riscossione),
        fattore(a.iva, a0.iva, el("elasticita_imponibile_iva")),
    );
    let contributi = (
        base(s.contributi_pil.valore, el("elasticita_contributi"), 1.0),
        fattore(
            a.contributi,
            a0.contributi,
            el("elasticita_imponibile_contributi"),
        ),
    );
    // Il gettito dovuto alle aliquote cambiate: l'entrata meno quella con `τ = τ₀`.
    let discrezionali = irpef_base * misure.irpef_discrezionale
        + [ires, iva, contributi]
            .iter()
            .map(|(x, f)| x * (f - 1.0))
            .sum::<f64>();
    let trimestre = |mld_annui: f64| euro(mld_annui * MLD / 4.0);
    // Coesione sospesa finché la PDE è in uno stato di inadempienza.
    let sospesa = if e.regole_ue.pde.e_inadempiente() {
        el("sospensione_coesione")
    } else {
        0.0
    };
    ContiTrimestre {
        entrate: Entrate {
            irpef: euro(irpef),
            ires: euro(ires.0 * ires.1),
            iva: euro(iva.0 * iva.1),
            contributi: euro(contributi.0 * contributi.1),
            altre: euro(base(s.altre_entrate_pil.valore, 1.0, 1.0)),
            fondi_coesione: euro(
                (s.fondi_coesione_pil.valore - sospesa).max(0.0) / 100.0 * pil_potenziale / 4.0,
            ),
        },
        spese: Spese {
            dicasteri: euro(bilancio.spesa_totale() as f64 / 4.0) + e.decreti,
            pensioni: e.spesa_pensioni(),
            sussidi: e.spesa_sussidi(dati, &misure.leve),
            altre_prestazioni: trimestre(s.altre_prestazioni.valore),
            interessi: euro(e.tasso_medio_debito / 100.0 * e.debito as f64 / 4.0),
            ammende: e.regole_ue.ammende(e.pil_trimestrale * 4.0 * MLD, dati),
            // Il saldo è in % del PIL potenziale annuo, quello del modello è trimestrale.
            disposizioni: euro(
                misure.saldo_disposizioni / 100.0 * e.pil_potenziale_reale * e.livello_prezzi * MLD,
            ),
        },
        entrate_discrezionali: euro(discrezionali),
    }
}

pub(crate) fn consuntivo(e: &Economia, data: Trimestre) -> ConsuntivoTrimestre {
    ConsuntivoTrimestre {
        data,
        pil: euro(e.pil_trimestrale * MLD),
        disavanzo: e.disavanzo_trimestre,
        spesa_primaria: e.conti.totale_spese() - e.conti.spese.interessi - e.conti.spese.ammende,
    }
}

impl Economia {
    /// Il bilancio che vale nel trimestre `data`: l'approvato dell'anno, se già in attesa.
    pub(crate) fn bilancio_in_vigore(&self, data: Trimestre) -> &Bilancio {
        match &self.bilancio_approvato {
            Some(b) if b.anno == data.anno => b,
            _ => &self.bilancio,
        }
    }

    /// Gennaio-marzo senza un bilancio approvato per l'anno.
    pub(crate) fn esercizio_provvisorio(&self, data: Trimestre) -> bool {
        data.trimestre == 1 && self.bilancio_in_vigore(data).anno != data.anno
    }

    /// Blocco finanza pubblica del trimestre `data`: prima della IS, sullo stato del turno prima.
    /// Rende la spesa netta del trimestre, con la disoccupazione che ha dato i sussidi.
    pub(super) fn finanza(&mut self, dati: &DatiGioco, data: Trimestre, misure: &Misure) -> i64 {
        if let Some(b) = self
            .bilancio_approvato
            .take_if(|b| data.trimestre == 1 && b.anno == data.anno)
        {
            self.bilancio = b;
        }
        self.previdenza(dati, data, &misure.leve);
        let conti = conti_del_trimestre(self, &self.bilancio, dati, misure);
        self.decreti = 0;

        // Impulso: la spesa discrezionale oltre quella del turno prima rivalutata all'inflazione.
        let prima = self.conti.spesa_discrezionale() as f64 * (1.0 + self.inflazione[0] / 400.0);
        let pil_potenziale_trimestrale = self.pil_potenziale_reale * self.livello_prezzi * MLD;
        self.impulso_fiscale =
            100.0 * (conti.spesa_discrezionale() as f64 - prima) / pil_potenziale_trimestrale;

        self.conti = conti;
        self.disavanzo_trimestre = self.conti.disavanzo();
        let spesa_netta = self.spesa_netta_trimestre(dati);

        let q = 1.0 / (4.0 * dati.scenario().vita_media_debito.valore);
        let rendimento =
            self.tasso_bce + dati.manopola("premio_termine_bund") + self.spread / 100.0;
        self.tasso_medio_debito = (1.0 - q) * self.tasso_medio_debito + q * rendimento;
        self.debito += self.disavanzo_trimestre;
        spesa_netta
    }
}
