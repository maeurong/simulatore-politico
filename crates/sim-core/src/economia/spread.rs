//! Spread BTP-Bund: quadratica nel debito (economia-italiana-parametri:12-17), 2,4 pb per punto
//! di disavanzo oltre il 3% (:21), premio di sfiducia che il TPI azzera (:37-44, :117-118).

use super::Economia;
use crate::dati::DatiGioco;
use crate::misure::CAPACITA_NEUTRA;

/// Trimestri dell'anno mobile su cui si misurano debito e disavanzo.
const ANNO_MOBILE: usize = 4;

/// Premio della ristrutturazione, in pb, oltre il quale il TPI non vale [assunzione del piano 9,
/// nessuna fonte in docs/ricerca].
const SOGLIA_TPI: f64 = 50.0;

/// Turni in cui il premio della ristrutturazione si dimezza [assunzione del piano 9, nessuna
/// fonte in docs/ricerca].
const EMIVITA_RISTRUTTURAZIONE: f64 = 8.0;

impl Economia {
    /// Debito e disavanzo in % del PIL degli ultimi 4 trimestri.
    pub(crate) fn debito_e_disavanzo_pil(&self) -> (f64, f64) {
        let da = self.storico.len().saturating_sub(ANNO_MOBILE);
        let ultimi = &self.storico[da..];
        let pil = ultimi.iter().map(|t| t.pil as f64).sum::<f64>();
        let disavanzo = ultimi.iter().map(|t| t.disavanzo as f64).sum::<f64>();
        (self.debito as f64 / pil * 100.0, disavanzo / pil * 100.0)
    }

    /// La formula dello spread sullo stato com'è ora, premio compreso. `capacita` è quella del
    /// ministro dell'Economia: `spread_capacita` pb per punto sotto `CAPACITA_NEUTRA`, dopo il
    /// tetto minimo dei fondamentali e fuori dal premio di sfiducia, che il TPI non azzera.
    pub(super) fn spread_dallo_stato(&self, dati: &DatiGioco, capacita: f64) -> f64 {
        let m = |n: &str| dati.manopola(n);
        let (debito, disavanzo) = self.debito_e_disavanzo_pil();
        let b = debito.max(0.0);
        let fondamentali =
            m("spread_s0") + 100.0 * (m("spread_beta_1") * b + m("spread_beta_2") * b * b);
        (fondamentali.max(m("spread_minimo"))
            + m("spread_disavanzo") * (disavanzo - 3.0).max(0.0)
            + m("spread_capacita") * (CAPACITA_NEUTRA - capacita)
            + self.premio_sfiducia
            + self.ristrutturazione.map_or(0.0, |r| r.premio))
        .min(m("spread_massimo"))
    }

    /// Fine turno, dopo le regole UE: il TPI vale con la PDE `Nessuna`, `Aperta` o `Sospesa`
    /// e azzera il premio. Finché dura il programma del MES lo azzera sempre (OMT), anche fuori
    /// dai criteri del TPI. Altrimenti il premio decade con la sua emivita. Solo il criterio 1
    /// del TPI è modellato [assunzione del piano 3: gli altri non hanno grandezze nella v1].
    /// `chiudi_turno` toglie il programma scaduto prima del passo: qui `programma_mes` c'è solo
    /// in corso.
    pub(super) fn aggiorna_spread(&mut self, dati: &DatiGioco, capacita: f64) {
        // Sopra `SOGLIA_TPI` il premio della ristrutturazione tiene il TPI fuori, come una PDE
        // inadempiente.
        let fuori_dal_tpi = self.regole_ue.pde.e_inadempiente()
            || self.ristrutturazione.is_some_and(|r| r.premio > SOGLIA_TPI);
        self.premio_sfiducia = if fuori_dal_tpi && self.programma_mes.is_none() {
            self.premio_sfiducia * 0.5_f64.powf(1.0 / dati.manopola("emivita_premio_sfiducia"))
        } else {
            0.0
        };
        self.spread = self.spread_dallo_stato(dati, capacita);
        // Il premio di questo turno è già nello spread; da qui si dimezza con la sua emivita.
        if let Some(r) = &mut self.ristrutturazione {
            r.premio *= 0.5_f64.powf(1.0 / EMIVITA_RISTRUTTURAZIONE);
        }
    }
}
