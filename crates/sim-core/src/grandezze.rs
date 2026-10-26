//! Grandezze dei dicasteri: indicatori di servizio, occupazione pubblica e stock regionali.
//! La base segue il funzionamento reale (salari e acquisti a prezzi costanti) con un decadimento
//! verso l'obiettivo; la vista le mostra con un turno di ritardo.

use crate::atti::Forma;
use crate::dati::DatiGioco;
use crate::economia::{Bilancio, Economia};
use crate::errore::Errore;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dinamica {
    /// La base segue il funzionamento del dicastero.
    Servizio,
    /// La base segue i salari di tutti i dicasteri.
    Turnover,
    /// La base resta all'iniziale: la muovono solo gli effetti.
    Fissa,
}

/// Il verso in cui la grandezza migliora.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Verso {
    Sale,
    Scende,
}

impl Verso {
    pub fn segno(self) -> f64 {
        match self {
            Verso::Sale => 1.0,
            Verso::Scende => -1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoGrandezza {
    pub id: String,
    pub nome: String,
    pub dinamica: Dinamica,
    pub verso: Verso,
    pub regionale: bool,
    pub iniziale: f64,
    pub min: f64,
    pub max: f64,
    /// Dicasteri i cui atti possono muoverla.
    pub dicasteri: Vec<String>,
}

/// Effetto di un atto già registrato: `intensita` è quella corretta per la capacità del
/// ministro, `inizio` il turno da cui conta.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffettoAttivo {
    pub grandezza: usize,
    /// `None` = vale ovunque.
    pub regione: Option<usize>,
    pub dicastero: usize,
    pub intensita: f64,
    pub inizio: u32,
    pub forma: Forma,
    /// Emivita (impulso) o vita utile (stock), in turni.
    pub durata: f64,
}

/// Contributo al turno `turno`: 0 prima di `inizio`; impulso `I · 0.5^(età/durata)`,
/// stock `I · max(0, 1 − età/durata)`.
pub(crate) fn contributo(e: &EffettoAttivo, turno: u32) -> f64 {
    let Some(eta) = turno.checked_sub(e.inizio) else {
        return 0.0;
    };
    let x = f64::from(eta) / e.durata;
    match e.forma {
        Forma::Impulso => e.intensita * 0.5_f64.powf(x),
        Forma::Stock => e.intensita * (1.0 - x).max(0.0),
    }
}

/// Uno stock esce a fine vita utile, un impulso oltre dieci emivite.
fn esaurito(e: &EffettoAttivo, turno: u32) -> bool {
    let Some(eta) = turno.checked_sub(e.inizio) else {
        return false;
    };
    let eta = f64::from(eta);
    match e.forma {
        Forma::Stock => eta >= e.durata,
        Forma::Impulso => eta > 10.0 * e.durata,
    }
}

/// Una riga per grandezza, lunga 1 o quanto le regioni.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Grandezze {
    pub basi: Vec<Vec<f64>>,
    pub valori: Vec<Vec<f64>>,
    /// Valori alla chiusura precedente: variazione per il giudizio, vista in ritardo.
    pub prima: Vec<Vec<f64>>,
    pub effetti: Vec<EffettoAttivo>,
}

fn righe_di(dati: &DatiGioco, f: impl Fn(&DatoGrandezza) -> f64) -> Vec<Vec<f64>> {
    let n = dati.regioni().len();
    (dati.grandezze().iter())
        .map(|g| vec![f(g); if g.regionale { n } else { 1 }])
        .collect()
}

/// Spesa di funzionamento a prezzi costanti: `(salari + acquisti) / prezzi`.
fn funzionamento(b: &Bilancio, i: usize, prezzi: f64) -> f64 {
    let d = &b.dicasteri[i];
    (d.salari + d.acquisti) as f64 / prezzi
}

/// Salari di tutti i dicasteri a prezzi costanti.
fn salari(b: &Bilancio, prezzi: f64) -> f64 {
    b.dicasteri.iter().map(|d| d.salari as f64).sum::<f64>() / prezzi
}

impl Grandezze {
    pub fn iniziali(dati: &DatiGioco) -> Grandezze {
        let righe = righe_di(dati, |g| g.iniziale);
        Grandezze {
            basi: righe.clone(),
            valori: righe.clone(),
            prima: righe,
            effetti: Vec::new(),
        }
    }

    /// Un passo: `prima ← valori`, poi la base verso l'obiettivo, poi i valori.
    pub fn avanza(&mut self, dati: &DatiGioco, e: &Economia, turno: u32) {
        self.prima.clone_from(&self.valori);
        let (v_serv, v_turn, elast) = (
            dati.manopola("velocita_servizio"),
            dati.manopola("velocita_turnover"),
            dati.manopola("elasticita_servizio"),
        );
        let zero = Bilancio::iniziale(dati);
        let (s, s0) = (salari(&e.bilancio, e.livello_prezzi), salari(&zero, 1.0));
        for (k, g) in dati.grandezze().iter().enumerate() {
            let (rapporto, velocita) = match g.dinamica {
                Dinamica::Fissa => {
                    self.basi[k].fill(g.iniziale);
                    continue;
                }
                Dinamica::Turnover => (s / s0, v_turn),
                Dinamica::Servizio => {
                    let rapporto = (dati.dicasteri().iter())
                        .position(|d| d.indicatore_servizio == g.id)
                        .map_or(1.0, |i| {
                            funzionamento(&e.bilancio, i, e.livello_prezzi)
                                / funzionamento(&zero, i, 1.0)
                        });
                    (rapporto, v_serv)
                }
            };
            // Un riferimento nullo (0/0 o x/0): nessuna spinta.
            let rapporto = if rapporto.is_finite() { rapporto } else { 1.0 };
            let obiettivo =
                (g.iniziale * rapporto.powf(g.verso.segno() * elast)).clamp(g.min, g.max);
            for b in &mut self.basi[k] {
                *b += velocita * (obiettivo - *b);
            }
        }
        for (k, g) in dati.grandezze().iter().enumerate() {
            for (r, (v, b)) in self.valori[k].iter_mut().zip(&self.basi[k]).enumerate() {
                let somma: f64 = (self.effetti.iter())
                    .filter(|e| e.grandezza == k && (e.regione.is_none() || e.regione == Some(r)))
                    .map(|e| contributo(e, turno))
                    .sum();
                *v = (b + somma).clamp(g.min, g.max);
            }
        }
        self.effetti.retain(|e| !esaurito(e, turno));
    }

    /// Valore nazionale: la riga se è una, altrimenti la media pesata sulle forze di lavoro.
    pub fn nazionale(&self, dati: &DatiGioco, g: usize) -> f64 {
        media(dati, &self.valori[g])
    }

    /// Valore nazionale alla chiusura precedente, quello che la `Vista` mostra.
    pub(crate) fn nazionale_prima(&self, dati: &DatiGioco, g: usize) -> f64 {
        media(dati, &self.prima[g])
    }

    /// Variazione nazionale nell'ultimo turno.
    pub(crate) fn variazione(&self, dati: &DatiGioco, g: usize) -> f64 {
        media(dati, &self.valori[g]) - media(dati, &self.prima[g])
    }

    /// Forma e valori di un salvataggio contro i dati.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` se le righe non sono quante le grandezze o lunghe quanto devono,
    /// se un valore non è finito o esce da min-max, o se un effetto punta a una grandezza, una
    /// regione o un dicastero che non c'è, o ha intensità o durata non valide.
    pub fn controlla(&self, dati: &DatiGioco) -> Result<(), Errore> {
        let n = dati.regioni().len();
        let buoni = self.effetti.iter().all(|e| {
            let regione_giusta = |regionale: bool| e.regione.is_none_or(|r| regionale && r < n);
            dati.grandezze()
                .get(e.grandezza)
                .is_some_and(|g| regione_giusta(g.regionale) && e.intensita.abs() <= g.max - g.min)
                && e.dicastero < dati.dicasteri().len()
                && e.intensita.is_finite()
                && e.durata.is_finite()
                && e.durata > 0.0
        });
        if !buoni {
            return Err(Errore::SalvataggioIlleggibile(
                "grandezze.effetti non combacia con i dati".into(),
            ));
        }
        for (nome, righe) in [
            ("basi", &self.basi),
            ("valori", &self.valori),
            ("prima", &self.prima),
        ] {
            let giuste = righe.len() == dati.grandezze().len()
                && dati.grandezze().iter().zip(righe).all(|(g, r)| {
                    r.len() == if g.regionale { n } else { 1 }
                        && r.iter().all(|x| (g.min..=g.max).contains(x))
                });
            if !giuste {
                return Err(Errore::SalvataggioIlleggibile(format!(
                    "grandezze.{nome} non combacia con le grandezze dei dati"
                )));
            }
        }
        Ok(())
    }
}

fn media(dati: &DatiGioco, riga: &[f64]) -> f64 {
    if let [x] = riga {
        return *x;
    }
    let pesi = dati.regioni().iter().map(|r| r.forze_lavoro);
    let totale: f64 = pesi.clone().sum();
    pesi.zip(riga).map(|(w, x)| w * x).sum::<f64>() / totale
}

#[cfg(test)]
mod test {
    use super::*;

    fn effetto(forma: Forma, intensita: f64, inizio: u32, durata: f64) -> EffettoAttivo {
        EffettoAttivo {
            grandezza: 0,
            regione: None,
            dicastero: 0,
            intensita,
            inizio,
            forma,
            durata,
        }
    }

    #[test]
    fn impulso_si_dimezza_a_ogni_emivita() {
        let e = effetto(Forma::Impulso, 8.0, 2, 3.0);
        assert_eq!(contributo(&e, 2), 8.0);
        assert!((contributo(&e, 5) - 4.0).abs() < 1e-12);
        assert!((contributo(&e, 8) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn stock_si_consuma_lungo_la_vita_utile() {
        let e = effetto(Forma::Stock, 8.0, 1, 4.0);
        assert_eq!(contributo(&e, 1), 8.0);
        assert!((contributo(&e, 3) - 4.0).abs() < 1e-12);
        assert_eq!(contributo(&e, 5), 0.0);
        assert_eq!(contributo(&e, 50), 0.0);
        assert!(esaurito(&e, 5) && !esaurito(&e, 4));
    }

    #[test]
    fn niente_prima_del_ritardo() {
        for forma in [Forma::Impulso, Forma::Stock] {
            let e = effetto(forma, 8.0, 4, 3.0);
            assert_eq!(contributo(&e, 3), 0.0);
            assert!(!esaurito(&e, 3));
            assert!(contributo(&e, 4) > 0.0);
        }
    }

    #[test]
    fn impulso_esce_dopo_dieci_emivite() {
        let e = effetto(Forma::Impulso, 8.0, 2, 3.0);
        assert!(!esaurito(&e, 32));
        assert!(esaurito(&e, 33));
    }

    #[test]
    fn segno_del_verso() {
        assert_eq!(Verso::Sale.segno(), 1.0);
        assert_eq!(Verso::Scende.segno(), -1.0);
    }

    /// Con meno fondi i processi si allungano (`verso = scende`), la sanità peggiora (`sale`).
    #[test]
    #[expect(
        clippy::unwrap_used,
        reason = "nei test un unwrap che fallisce e' il fallimento voluto"
    )]
    fn meno_fondi_allungano_i_processi() {
        let dati = crate::dati::dati_base();
        let mut e = Economia::iniziale(&dati);
        for d in &mut e.bilancio.dicasteri {
            d.salari /= 2;
            d.acquisti /= 2;
        }
        let mut g = Grandezze::iniziali(&dati);
        g.avanza(&dati, &e, 0);
        let valore = |id: &str| g.nazionale(&dati, dati.grandezza(id).unwrap());
        assert!(valore("durata_processi") > 100.0);
        assert!(valore("qualita_servizio_sanitario") < 100.0);
    }
}
