use crate::dati::Ruolo;
use std::fmt;

/// Errori di caricamento e validazione dei dati e dei salvataggi.
#[derive(Debug, PartialEq)]
pub enum Errore {
    RuoloOrfano {
        ruolo: Ruolo,
    },
    RuoloDoppio {
        ruolo: Ruolo,
        primo: String,
        secondo: String,
    },
    DicasteroDoppio {
        id: String,
    },
    FuoriIntervallo {
        nome: String,
        valore: f64,
        min: f64,
        max: f64,
    },
    Toml {
        file: String,
        messaggio: String,
    },
    SchemaDiverso {
        trovata: u32,
        attesa: u32,
    },
    SalvataggioIllegibile(String),
    SalvataggioNonRiuscito(String),
}

impl fmt::Display for Errore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RuoloOrfano { ruolo } => write!(f, "ruolo senza dicastero: {ruolo}"),
            Self::RuoloDoppio {
                ruolo,
                primo,
                secondo,
            } => write!(
                f,
                "ruolo assegnato a due dicasteri: {ruolo} ({primo}, {secondo})"
            ),
            Self::DicasteroDoppio { id } => write!(f, "dicastero ripetuto: {id}"),
            Self::FuoriIntervallo {
                nome,
                valore,
                min,
                max,
            } => write!(
                f,
                "manopola fuori intervallo: {nome} = {valore} (ammesso da {min} a {max})"
            ),
            Self::Toml { file, messaggio } => write!(f, "{file}: {messaggio}"),
            Self::SchemaDiverso { trovata, attesa } => write!(
                f,
                "salvataggio di una versione diversa del gioco: schema {trovata}, atteso {attesa}"
            ),
            Self::SalvataggioIllegibile(m) => write!(f, "salvataggio illeggibile: {m}"),
            Self::SalvataggioNonRiuscito(m) => write!(f, "salvataggio non riuscito: {m}"),
        }
    }
}

impl std::error::Error for Errore {}
