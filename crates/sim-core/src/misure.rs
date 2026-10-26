//! Misure: ciò che gli atti dei ministri e gli stock fanno arrivare al motore economico.
//! Le costruisce `Partita` a ogni chiusura, dalle grandezze com'erano all'apertura del turno.

use crate::dati::{DatiGioco, Ruolo};
use crate::economia::Economia;
use crate::errore::Errore;
use crate::grandezze::{Grandezze, contributo};
use crate::ministri::Ministri;
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};

/// Capacità del ministro dell'Economia che non sposta lo spread.
pub(crate) const CAPACITA_NEUTRA: f64 = 3.0;

/// Anno in cui è calibrata `crescita_potenziale`: la demografia conta rispetto a quello.
const ANNO_BASE: u16 = 2027;

/// Anno della piramide dei dati (1/1/2026).
const ANNO_PIRAMIDE: u16 = 2026;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misure {
    /// Moltiplica IRPEF, IRES, IVA e le imposte degli autonomi; 1 = riscossione di riferimento.
    pub riscossione: f64,
    /// Punti percentuali sul prezzo degli input importati.
    pub prezzo_energia: f64,
    /// Punti di output gap, per regione.
    pub domanda_regionale: Vec<f64>,
    /// Punti di disoccupazione, per regione.
    pub disoccupazione_strutturale: Vec<f64>,
    /// Punti annui, si sommano alla crescita del potenziale.
    pub crescita_potenziale: f64,
    /// Capacità effettiva del ministro dell'Economia, 1 in interim.
    pub capacita_economia: f64,
    /// Punti di disoccupazione dovuti agli effetti regionali attivi: regione × dicastero.
    pub merito: Vec<Vec<f64>>,
}

/// `g(y) = 100 · (L(y)/L(y−1) − 1)`, con `L(y)` le persone di 15-66 anni nell'anno `y`: la
/// piramide del 1/1/2026 invecchiata senza nascite né morti, i più giovani come la coorte 0.
/// Vale per `y > 2026`.
fn crescita_forza_lavoro(piramide: &[u32], anno: u16) -> f64 {
    let l = |y: u16| -> f64 {
        let passo = usize::from(y.saturating_sub(ANNO_PIRAMIDE));
        (15..=66usize)
            .map(|a| f64::from(piramide.get(a.saturating_sub(passo)).copied().unwrap_or(0)))
            .sum()
    };
    100.0 * (l(anno) / l(anno.saturating_sub(1)) - 1.0)
}

impl Misure {
    /// Le misure che non cambiano nulla: riscossione 1, il resto a zero, capacità 3.
    pub fn neutre(dati: &DatiGioco) -> Misure {
        let n = dati.regioni().len();
        Misure {
            riscossione: 1.0,
            prezzo_energia: 0.0,
            domanda_regionale: vec![0.0; n],
            disoccupazione_strutturale: vec![0.0; n],
            crescita_potenziale: 0.0,
            capacita_economia: CAPACITA_NEUTRA,
            merito: vec![vec![0.0; dati.dicasteri().len()]; n],
        }
    }

    /// Le misure del turno `turno` (trimestre `data`), dalle grandezze, dai ministri e
    /// dall'economia.
    pub fn da(
        dati: &DatiGioco,
        g: &Grandezze,
        m: &Ministri,
        e: &Economia,
        turno: u32,
        data: Trimestre,
    ) -> Misure {
        let n = dati.regioni().len();
        let manopola = |id: &str| dati.manopola(id);
        let nazionale =
            |id: &str, vuoto: f64| dati.grandezza(id).map_or(vuoto, |k| g.nazionale(dati, k));
        let regionale = |id: &str, vuoto: f64| {
            dati.grandezza(id)
                .map_or_else(|| vec![vuoto; n], |k| g.valori[k].clone())
        };

        let riscossione = 1.0
            + manopola("elasticita_riscossione")
                * (nazionale("efficienza_riscossione", 100.0) / 100.0 - 1.0);

        let infrastrutture = regionale("stock_infrastrutture", 100.0);
        let investimenti = regionale("investimenti_privati", 100.0);
        let stock = manopola("stock_disoccupazione");
        let politiche_attive = nazionale("politiche_attive", 0.0);
        let disoccupazione_strutturale = infrastrutture
            .iter()
            .zip(&investimenti)
            .map(|(i, v)| -stock * ((i - 100.0) + (v - 100.0)) / 10.0 - politiche_attive)
            .collect();

        let prezzo_energia = nazionale("prezzo_energia", 0.0);
        // Prima del 2027 la demografia non sposta il potenziale dalla base calibrata.
        let anno = data.anno.max(ANNO_BASE);
        let piramide = &dati.struttura().piramide;
        let crescita_potenziale = manopola("crescita_infrastrutture")
            * (nazionale("stock_infrastrutture", 100.0) - 100.0)
            / 10.0
            + manopola("crescita_capitale_umano") * (nazionale("capitale_umano", 100.0) - 100.0)
                / 10.0
            - manopola("crescita_giustizia") * (nazionale("durata_processi", 100.0) - 100.0) / 10.0
            - manopola("crescita_energia") * prezzo_energia / 10.0
            + manopola("crescita_demografia")
                * (crescita_forza_lavoro(piramide, anno)
                    - crescita_forza_lavoro(piramide, ANNO_BASE));

        let capacita_economia = dati
            .dicasteri()
            .iter()
            .position(|d| d.ruoli.contains(&Ruolo::BilancioECredibilita))
            .and_then(|i| m.dicasteri.get(i)?.ministro.as_ref())
            .map_or(1.0, |x| x.capacita_effettiva(dati));

        Misure {
            riscossione,
            prezzo_energia,
            domanda_regionale: regionale("domanda_regionale", 0.0),
            disoccupazione_strutturale,
            crescita_potenziale,
            capacita_economia,
            merito: Self::merito(dati, g, e, turno),
        }
    }

    /// Per regione e dicastero, i punti di disoccupazione dovuti ai suoi effetti regionali
    /// attivi: `β_r · domanda_regionale − stock_disoccupazione · (infrastrutture + investimenti)/10`.
    /// I valori delle grandezze sono quelli dell'ultimo `avanza`, cioè del turno prima: il merito
    /// legge i contributi allo stesso turno, così coincide con ciò che la disoccupazione ha
    /// davvero ricevuto.
    fn merito(dati: &DatiGioco, g: &Grandezze, e: &Economia, turno: u32) -> Vec<Vec<f64>> {
        let n = dati.regioni().len();
        let mut merito = vec![vec![0.0; dati.dicasteri().len()]; n];
        let beta = e.beta_okun(dati);
        let stock = dati.manopola("stock_disoccupazione");
        let (domanda, infrastrutture, investimenti) = (
            dati.grandezza("domanda_regionale"),
            dati.grandezza("stock_infrastrutture"),
            dati.grandezza("investimenti_privati"),
        );
        // Al turno 0 nessuna grandezza è ancora passata da `avanza`: niente da attribuire.
        let Some(visto) = turno.checked_sub(1) else {
            return merito;
        };
        for ef in &g.effetti {
            let Some(r) = ef.regione else { continue };
            let c = contributo(ef, visto);
            let k = Some(ef.grandezza);
            let punti = if k == domanda {
                beta[r] * c
            } else if k == infrastrutture || k == investimenti {
                -stock * c / 10.0
            } else {
                0.0
            };
            merito[r][ef.dicastero] += punti;
        }
        merito
    }

    /// Lunghezze per regione e dicastero, tutto finito.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` se una lunghezza non combacia coi dati o un valore non è finito.
    pub fn controlla(&self, dati: &DatiGioco) -> Result<(), Errore> {
        let (n, nd) = (dati.regioni().len(), dati.dicasteri().len());
        let ko = |m: String| Err(Errore::SalvataggioIlleggibile(format!("misure: {m}")));
        for (campo, len) in [
            ("domanda_regionale", self.domanda_regionale.len()),
            (
                "disoccupazione_strutturale",
                self.disoccupazione_strutturale.len(),
            ),
            ("merito", self.merito.len()),
        ] {
            if len != n {
                return ko(format!("{campo} ha {len} voci, le regioni sono {n}"));
            }
        }
        if self.merito.iter().any(|r| r.len() != nd) {
            return ko(format!("merito: una regione non ha {nd} dicasteri"));
        }
        let scalari = [
            ("riscossione", self.riscossione),
            ("prezzo_energia", self.prezzo_energia),
            ("crescita_potenziale", self.crescita_potenziale),
            ("capacita_economia", self.capacita_economia),
        ];
        if let Some((campo, _)) = scalari.iter().find(|(_, x)| !x.is_finite()) {
            return ko(format!("{campo} non finito"));
        }
        if self.riscossione <= 0.0 {
            return ko("riscossione non positiva".into());
        }
        let vettori = [
            ("domanda_regionale", &self.domanda_regionale),
            (
                "disoccupazione_strutturale",
                &self.disoccupazione_strutturale,
            ),
        ];
        if let Some((campo, _)) = vettori
            .iter()
            .find(|(_, v)| v.iter().any(|x| !x.is_finite()))
        {
            return ko(format!("{campo} ha un valore non finito"));
        }
        if self.merito.iter().flatten().any(|x| !x.is_finite()) {
            return ko("merito ha un valore non finito".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dati::dati_base as dati;
    use crate::partita::Partita;

    fn misure_di(d: &DatiGioco, p: &Partita, grandezze: &Grandezze, data: Trimestre) -> Misure {
        Misure::da(d, grandezze, p.ministri(), p.economia(), 0, data)
    }

    #[test]
    fn demografia_frena_il_potenziale_rispetto_al_2027() {
        let d = dati();
        let p = &d.struttura().piramide;
        assert_eq!(
            crescita_forza_lavoro(p, 2027) - crescita_forza_lavoro(p, 2027),
            0.0
        );
        assert!(crescita_forza_lavoro(p, 2035) < crescita_forza_lavoro(p, 2027));
    }

    #[test]
    #[expect(
        clippy::unwrap_used,
        reason = "nei test un unwrap che fallisce e' il fallimento voluto"
    )]
    fn stock_sopra_100_alzano_il_potenziale() {
        let d = dati();
        let p = Partita::nuova(d.clone(), 1);
        let mut g = p.grandezze().clone();
        let data = Trimestre::TURNO_ZERO.successivo();
        let base = misure_di(&d, &p, &g, data).crescita_potenziale;
        for nome in ["stock_infrastrutture", "capitale_umano"] {
            let k = d.grandezza(nome).unwrap();
            g.valori[k].fill(120.0);
        }
        assert!(misure_di(&d, &p, &g, data).crescita_potenziale > base);
    }

    #[test]
    fn misure_neutre_al_turno_zero() {
        let d = dati();
        let p = Partita::nuova(d.clone(), 1);
        let m = misure_di(&d, &p, p.grandezze(), Trimestre::TURNO_ZERO);
        let neutre = Misure::neutre(&d);
        assert_eq!(m.riscossione, 1.0);
        assert_eq!(m.prezzo_energia, 0.0);
        assert!(m.domanda_regionale.iter().all(|x| *x == 0.0));
        assert!(m.disoccupazione_strutturale.iter().all(|x| *x == 0.0));
        assert!(m.crescita_potenziale.abs() < 1e-12, "{m:?}");
        assert_eq!(m.merito, neutre.merito);
        assert_eq!(
            (m.domanda_regionale.len(), m.merito.len()),
            (d.regioni().len(), d.regioni().len())
        );
    }
}
