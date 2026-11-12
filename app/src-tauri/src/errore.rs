//! L'errore che i comandi mandano alla UI: il rifiuto sta accanto alla scelta che l'ha causato, il
//! guasto nella cornice.

use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Specie {
    Rifiuto,
    Guasto,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErroreGuscio {
    pub specie: Specie,
    pub testo: String,
}

/// La regola del gioco o la richiesta dell'utente non lo permette.
pub fn rifiuto(e: impl ToString) -> ErroreGuscio {
    ErroreGuscio {
        specie: Specie::Rifiuto,
        testo: e.to_string(),
    }
}

/// Disco, lock o stato: la partita è rimasta com'era.
pub fn guasto(e: impl ToString) -> ErroreGuscio {
    ErroreGuscio {
        specie: Specie::Guasto,
        testo: e.to_string(),
    }
}

impl From<String> for ErroreGuscio {
    fn from(testo: String) -> Self {
        guasto(testo)
    }
}

impl From<&str> for ErroreGuscio {
    fn from(testo: &str) -> Self {
        guasto(testo)
    }
}

impl From<ErroreGuscio> for String {
    fn from(e: ErroreGuscio) -> Self {
        e.testo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_forma_serializzata_ha_la_specie_in_snake_case() {
        assert_eq!(
            serde_json::to_string(&rifiuto("no")).ok().as_deref(),
            Some(r#"{"specie":"rifiuto","testo":"no"}"#)
        );
        assert_eq!(
            serde_json::to_string(&guasto("rotto")).ok().as_deref(),
            Some(r#"{"specie":"guasto","testo":"rotto"}"#)
        );
    }
}
