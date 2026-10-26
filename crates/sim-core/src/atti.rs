use crate::dati::DatiGioco;
use crate::errore::Errore;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Velocita {
    Rapido,
    Lento,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Lato {
    Sinistra,
    Destra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Forma {
    Impulso,
    Stock,
}

impl fmt::Display for Velocita {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Rapido => "rapido",
            Self::Lento => "lento",
        })
    }
}

impl fmt::Display for Lato {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Sinistra => "sinistra",
            Self::Destra => "destra",
        })
    }
}

impl fmt::Display for Forma {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Impulso => "impulso",
            Self::Stock => "stock",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffettoAtto {
    pub grandezza: String,
    pub intensita: f64,
    pub ritardo: u32,
    pub forma: Forma,
    pub durata: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Atto {
    pub id: String,
    pub nome: String,
    pub motto: String,
    pub dicastero: String,
    pub velocita: Velocita,
    pub lato: Lato,
    pub posizione: f64,
    pub turni: u32,
    pub costo: f64,
    pub effetti: Vec<EffettoAtto>,
}

/// Coerenza del catalogo con i dicasteri e le grandezze.
pub(crate) fn valida(dati: &DatiGioco) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Atti(m));
    let atti = dati.atti();
    for (i, a) in atti.iter().enumerate() {
        if atti[..i].iter().any(|p| p.id == a.id) {
            return ko(format!("atto ripetuto: {}", a.id));
        }
        if !dati.dicasteri().iter().any(|d| d.id == a.dicastero) {
            return ko(format!("{}: dicastero sconosciuto {}", a.id, a.dicastero));
        }
    }
    for (i, d) in dati.dicasteri().iter().enumerate() {
        let suoi: Vec<&Atto> = dati.atti_del_dicastero(i).map(|(_, a)| a).collect();
        if suoi.len() != 4 {
            return ko(format!("{} ha {} atti, ne servono 4", d.id, suoi.len()));
        }
        for (k, a) in suoi.iter().enumerate() {
            if suoi[..k]
                .iter()
                .any(|p| p.velocita == a.velocita && p.lato == a.lato)
            {
                return ko(format!(
                    "{}: due atti nella casella {} {}",
                    d.id, a.velocita, a.lato
                ));
            }
        }
    }
    for a in atti {
        let p = a.posizione;
        if !(p.is_finite() && (1.0..=7.0).contains(&p)) {
            return ko(format!("{}: posizione fuori intervallo", a.id));
        }
        let giusto = match a.lato {
            Lato::Sinistra => p <= 4.0,
            Lato::Destra => p >= 4.0,
        };
        if !giusto {
            return ko(format!(
                "{}: posizione {p} dal lato sbagliato ({})",
                a.id, a.lato
            ));
        }
        if !(1..=20).contains(&a.turni) {
            return ko(format!("{}: turni fuori intervallo", a.id));
        }
        if !(a.costo.is_finite() && a.costo > 0.0 && a.costo <= 10.0) {
            return ko(format!("{}: costo fuori intervallo", a.id));
        }
        if !(1..=2).contains(&a.effetti.len()) {
            return ko(format!("{}: effetti fuori intervallo", a.id));
        }
        let mut regionale = false;
        for e in &a.effetti {
            let Some(g) = dati.grandezze().iter().find(|g| g.id == e.grandezza) else {
                return ko(format!("{}: grandezza sconosciuta {}", a.id, e.grandezza));
            };
            if !g.dicasteri.contains(&a.dicastero) {
                return ko(format!(
                    "{}: {} non è nella riga del dicastero {}",
                    a.id, g.id, a.dicastero
                ));
            }
            // Con capacità 5 l'effetto vale intensità · 5/3: deve restare nel tetto del salvataggio.
            let tetto = (g.max - g.min) * 3.0 / 5.0;
            if !(e.intensita.is_finite()
                && e.intensita != 0.0
                && e.intensita.abs() <= tetto.min(1e3))
            {
                return ko(format!("{}: intensita fuori intervallo", a.id));
            }
            if e.ritardo > 40 {
                return ko(format!("{}: ritardo fuori intervallo", a.id));
            }
            if !(e.durata.is_finite() && e.durata > 0.0 && e.durata <= 1000.0) {
                return ko(format!("{}: durata fuori intervallo", a.id));
            }
            regionale |= g.regionale;
        }
        let bersaglio = dati
            .dicasteri()
            .iter()
            .any(|d| d.id == a.dicastero && d.regione_bersaglio);
        if bersaglio && !regionale {
            return ko(format!(
                "{}: nessun effetto su una grandezza regionale",
                a.id
            ));
        }
    }
    Ok(())
}
