//! Regole fiscali UE: spesa netta per anno solare, conto di controllo e macchina della PDE
//! (regole-fiscali-ue:14, :20, :33-38, :63-78). Il controllo gira una volta l'anno, nel turno
//! aprile-giugno, sull'anno solare prima.

use super::Economia;
use super::finanza::{MLD, euro};
use crate::dati::DatiGioco;
use crate::partita::Trimestre;
use crate::scenario::StatoPde;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegoleUe {
    pub pde: StatoPde,
    /// % del PIL, mai sotto zero.
    pub conto_controllo: f64,
    /// % del PIL, dell'ultimo controllo (già al netto della clausola).
    pub ultima_deviazione: f64,
    /// Turni chiusi nello stato corrente; torna a 0 a ogni cambio.
    pub turni_in_stato: u32,
    /// Euro, per anno solare.
    pub spesa_netta: BTreeMap<u16, i64>,
    /// Euro, per anno solare.
    pub pil_annuo: BTreeMap<u16, i64>,
    /// Euro, per anno solare.
    pub disavanzo_annuo: BTreeMap<u16, i64>,
}

impl RegoleUe {
    pub(super) fn nuove(pde: StatoPde, conto_controllo: f64) -> RegoleUe {
        RegoleUe {
            pde,
            conto_controllo,
            ultima_deviazione: 0.0,
            turni_in_stato: 0,
            spesa_netta: BTreeMap::new(),
            pil_annuo: BTreeMap::new(),
            disavanzo_annuo: BTreeMap::new(),
        }
    }

    /// Ammende del trimestre che comincia, euro: quella della 126.8 nel primo turno da
    /// `Inadempiente`, poi quella semestrale ogni 2 turni da `Sanzionata`.
    pub(super) fn ammende(&self, pil_annuo: f64, dati: &DatiGioco) -> i64 {
        let pct = match self.pde {
            StatoPde::Inadempiente if self.turni_in_stato == 0 => dati.manopola("ammenda_pde"),
            StatoPde::Sanzionata if self.turni_in_stato.is_multiple_of(2) => {
                dati.manopola("ammenda_semestrale")
            }
            _ => 0.0,
        };
        euro(pct / 100.0 * pil_annuo)
    }

    fn cambia(&mut self, pde: StatoPde) {
        if pde != self.pde {
            self.pde = pde;
            self.turni_in_stato = 0;
        }
    }
}

impl Economia {
    /// Spesa netta del trimestre appena calcolato in `conti`: spesa primaria meno fondi di
    /// coesione meno la parte dei sussidi sopra quella a disoccupazione strutturale.
    pub(crate) fn spesa_netta_trimestre(&self, dati: &DatiGioco) -> i64 {
        let u = self.disoccupazione_nazionale(dati);
        let u_strutturale = self.media_regionale(dati, |e| e.disoccupazione_strutturale);
        let s = &self.conti.spese;
        let ciclica = if u > 0.0 {
            s.sussidi as f64 * (1.0 - u_strutturale / u).max(0.0)
        } else {
            0.0
        };
        // Con la coesione sospesa `fondi_coesione` scende e la spesa netta sale: quella spesa
        // non è più coperta dai fondi UE. Effetto voluto.
        s.dicasteri + s.pensioni + s.sussidi + s.altre_prestazioni
            - self.conti.entrate.fondi_coesione
            // Le entrate dovute ad aliquote cambiate sono misure discrezionali: abbassano la
            // spesa netta.
            - self.conti.entrate_discrezionali
            - euro(ciclica)
    }

    /// Turno zero: il 2026 è il trimestre simulato ×4, il 2025 sta esattamente sul tetto del
    /// 2026 [assunzione del piano 3, coerente con la PDE sospesa].
    pub(super) fn avvia_regole_ue(&mut self, dati: &DatiGioco) {
        let anno = Trimestre::TURNO_ZERO.anno;
        let sn = 4 * self.spesa_netta_trimestre(dati);
        let tetto = dati.scenario().tetto_spesa_netta(anno);
        let r = &mut self.regole_ue;
        r.spesa_netta.insert(anno, sn);
        r.spesa_netta.insert(
            anno.saturating_sub(1),
            euro(sn as f64 / (1.0 + tetto / 100.0)),
        );
        r.pil_annuo
            .insert(anno, 4 * euro(self.pil_trimestrale * MLD));
        r.disavanzo_annuo.insert(anno, 4 * self.disavanzo_trimestre);
    }

    /// Fine del turno `data`, dopo lo storico: accumula, controlla (T2), fa avanzare la PDE.
    /// `spesa_netta` è quella del trimestre, calcolata in `finanza` (prima di Okun).
    pub(super) fn aggiorna_regole_ue(
        &mut self,
        dati: &DatiGioco,
        data: Trimestre,
        spesa_netta: i64,
    ) {
        // L'anno del turno zero è già completo.
        if data.anno > Trimestre::TURNO_ZERO.anno {
            let pil = euro(self.pil_trimestrale * MLD);
            let disavanzo = self.disavanzo_trimestre;
            let r = &mut self.regole_ue;
            *r.spesa_netta.entry(data.anno).or_default() += spesa_netta;
            *r.pil_annuo.entry(data.anno).or_default() += pil;
            *r.disavanzo_annuo.entry(data.anno).or_default() += disavanzo;
        }
        self.regole_ue.turni_in_stato = self.regole_ue.turni_in_stato.saturating_add(1);
        let mut pde = self.regole_ue.pde;
        if data.trimestre == 2 {
            pde = self.controllo_annuale(dati, data.anno);
        }
        if pde == self.regole_ue.pde {
            // 126.9 al primo T4 dopo l'inadempienza; 126.11 dopo 2 turni da intimata.
            pde = match pde {
                StatoPde::Inadempiente if data.trimestre == 4 => StatoPde::Intimata,
                StatoPde::Intimata if self.regole_ue.turni_in_stato >= 2 => StatoPde::Sanzionata,
                p => p,
            };
        }
        self.regole_ue.cambia(pde);
    }

    /// Controllo dell'anno `anno - 1`; aggiorna il conto e dice lo stato che ne segue.
    fn controllo_annuale(&mut self, dati: &DatiGioco, anno: u16) -> StatoPde {
        let r = &self.regole_ue;
        let s = dati.scenario();
        let (a1, a2) = (anno.saturating_sub(1), anno.saturating_sub(2));
        let (Some(&sn1), Some(&sn2), Some(&pil1), Some(&dis1)) = (
            r.spesa_netta.get(&a1),
            r.spesa_netta.get(&a2),
            r.pil_annuo.get(&a1),
            r.disavanzo_annuo.get(&a1),
        ) else {
            return r.pde;
        };
        let clausola = if a1 <= s.clausola_nazionale_fino_a {
            s.clausola_nazionale_margine.valore
        } else {
            0.0
        };
        let tetto = s.tetto_spesa_netta(a1);
        let pil1 = pil1 as f64;
        let deviazione =
            (sn1 as f64 - sn2 as f64 * (1.0 + tetto / 100.0)) / pil1 * 100.0 - clausola;
        let conto = (r.conto_controllo + deviazione).max(0.0);
        let rilevante = deviazione > dati.manopola("deviazione_annua_rilevante")
            || conto > dati.manopola("deviazione_cumulata_rilevante");
        let disavanzo_anno = dis1 as f64 / pil1 * 100.0;
        let (debito, disavanzo_mobile) = self.debito_e_disavanzo_pil();
        let pde = match r.pde {
            StatoPde::Nessuna
                if disavanzo_anno > 3.0 || (debito > 60.0 && disavanzo_anno > 0.5 && rilevante) =>
            {
                StatoPde::Aperta
            }
            StatoPde::Aperta | StatoPde::Sospesa if rilevante => StatoPde::Inadempiente,
            StatoPde::Aperta => StatoPde::Sospesa,
            StatoPde::Sospesa if disavanzo_anno < 3.0 && disavanzo_mobile < 3.0 => {
                StatoPde::Nessuna
            }
            StatoPde::Inadempiente | StatoPde::Intimata | StatoPde::Sanzionata if !rilevante => {
                StatoPde::Sospesa
            }
            p => p,
        };
        let r = &mut self.regole_ue;
        r.ultima_deviazione = deviazione;
        r.conto_controllo = conto;
        pde
    }
}
