//! Conti a quattro settori (famiglie, imprese, Stato, estero), senza banche. Ogni flusso compare
//! con segni opposti in due settori, perciò la somma dei saldi fa zero: è il controllo di chiusura.

use super::finanza::{MLD, euro};
use super::{Economia, Ristrutturazione};
use crate::dati::DatiGioco;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settori {
    pub famiglie: i64,
    pub imprese: i64,
    pub stato: i64,
    pub estero: i64,
}

impl Settori {
    fn somma(&self) -> i64 {
        self.famiglie + self.imprese + self.stato + self.estero
    }
}

/// Euro. `saldi` sono del trimestre, `stock` la posizione netta accumulata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Sfc {
    pub saldi: Settori,
    pub stock: Settori,
    /// Prezzi del resto dell'area, 1 al turno zero.
    pub livello_prezzi_area: f64,
}

impl Sfc {
    /// Turno zero: lo Stato deve il debito alle famiglie (assunzione del piano 3: nessuna
    /// posizione netta sull'estero), saldi nulli.
    pub(super) fn iniziale(debito: i64, livello_prezzi: f64) -> Sfc {
        Sfc {
            saldi: Settori::default(),
            stock: Settori {
                famiglie: debito,
                stato: -debito,
                ..Settori::default()
            },
            livello_prezzi_area: livello_prezzi,
        }
    }
}

impl Economia {
    /// Taglia il debito di `taglio` (quota): lo Stato guadagna lo stesso importo, che esce dagli
    /// stock positivi di famiglie, estero e imprese in proporzione; il resto dell'arrotondamento
    /// alle famiglie. Gli stock restano a somma zero e lo Stato a `−debito`.
    pub(crate) fn ristruttura(&mut self, turno: u32, taglio: f64, premio: f64) {
        let t = euro(self.debito as f64 * taglio).max(0);
        let s = &mut self.sfc.stock;
        let positivi = [s.famiglie, s.estero, s.imprese].map(|x| x.max(0));
        let totale = i128::from(positivi.iter().sum::<i64>());
        let quote = positivi.map(|x| {
            (i128::from(t) * i128::from(x))
                .checked_div(totale)
                .map_or(0, |q| q as i64)
        });
        s.famiglie -= quote[0] + (t - quote.iter().sum::<i64>());
        s.estero -= quote[1];
        s.imprese -= quote[2];
        s.stato += t;
        self.debito -= t;
        self.ristrutturazione = Some(Ristrutturazione { turno, premio });
    }

    /// Somma dei quattro saldi del turno: zero se la contabilità è chiusa.
    pub fn residuo_sfc(&self) -> i64 {
        self.sfc.saldi.somma()
    }

    /// Somma dei quattro stock: zero se la contabilità è chiusa.
    pub fn residuo_stock(&self) -> i64 {
        self.sfc.stock.somma()
    }

    /// Blocco SFC del trimestre, dopo PIL e prezzi aggiornati, sui `conti` già calcolati.
    pub(super) fn sfc(&mut self, dati: &DatiGioco) {
        let s = dati.scenario();
        let pil = self.pil_trimestrale * MLD;
        let y = euro(pil);
        let c = &self.conti;

        // Primari: arrotondati una volta. I residui sono in interi, così la somma chiude per identità.
        let rapporto_prezzi = self.sfc.livello_prezzi_area / self.livello_prezzi;
        let x = euro(
            s.esportazioni_pil.valore / 100.0
                * pil
                * (1.0 + self.domanda_estera / 100.0)
                * rapporto_prezzi.powf(dati.manopola("elasticita_export_prezzi")),
        );
        // L'export vede i prezzi dell'area di questo turno, che si aggiornano dopo.
        self.sfc.livello_prezzi_area *= 1.0 + self.area_euro.inflazione_resto / 400.0;
        let m = euro(s.importazioni_pil.valore / 100.0 * pil);
        let g = euro(
            self.bilancio
                .dicasteri
                .iter()
                .map(|d| d.salari + d.acquisti + d.margine)
                .sum::<i64>() as f64
                / 4.0,
        );
        // Assunzione del piano 3: la spesa dei dicasteri oltre G va alle imprese.
        let contributi_imprese = c.spese.dicasteri - g;
        debug_assert!(contributi_imprese >= 0, "G sopra la spesa dei dicasteri");
        let lavoro = euro(dati.manopola("quota_redditi_lavoro") * pil);
        let profitti = y - lavoro - c.entrate.iva;
        let distribuiti = euro(dati.manopola("quota_profitti_distribuiti") * profitti as f64);
        let (cq, iq) = (s.consumi_famiglie_pil.valore, s.investimenti_pil.valore);
        let consumi_investimenti = y - g - x + m;
        let consumi = euro(cq / (cq + iq) * consumi_investimenti as f64);
        let investimenti = consumi_investimenti - consumi;

        let (e, sp) = (&c.entrate, &c.spese);
        let saldi = Settori {
            famiglie: lavoro
                + distribuiti
                + sp.pensioni
                + sp.sussidi
                + sp.altre_prestazioni
                + sp.interessi
                // ponytail: tutto il saldo delle disposizioni alle famiglie, anche quello che
                // tocca le imprese (credito ZES, incentivi); un settore per disposizione se
                // gli stock SFC diventano un ingresso del motore.
                + sp.disposizioni
                - e.irpef
                - e.contributi
                - e.altre
                - consumi,
            imprese: profitti - distribuiti + contributi_imprese - e.ires - investimenti,
            stato: c.totale_entrate() - c.totale_spese(),
            estero: m - x - e.fondi_coesione + sp.ammende,
        };
        self.sfc.stock = Settori {
            famiglie: self.sfc.stock.famiglie + saldi.famiglie,
            imprese: self.sfc.stock.imprese + saldi.imprese,
            stato: self.sfc.stock.stato + saldi.stato,
            estero: self.sfc.stock.estero + saldi.estero,
        };
        self.sfc.saldi = saldi;
    }
}
