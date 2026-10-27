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
    IndicatoreDoppio {
        indicatore: String,
        primo: String,
        secondo: String,
    },
    FuoriIntervallo {
        nome: String,
        valore: f64,
        min: f64,
        max: f64,
    },
    ScenarioFuoriIntervallo {
        nome: String,
        valore: f64,
        min: f64,
        max: f64,
    },
    ManopolaMancante {
        nome: String,
    },
    SettoreSconosciuto {
        id: String,
        dove: String,
    },
    SettoreNonProduttivo {
        id: String,
    },
    QuoteNonUnitarie {
        dove: String,
    },
    StrutturaIncoerente(String),
    LeggeElettorale(String),
    DatiElettorali(String),
    VotiIncoerenti(String),
    Elettorato(String),
    Partiti(String),
    Grandezze(String),
    Atti(String),
    Disposizioni(String),
    Comando(String),
    Bilancio(String),
    Fondazione(String),
    Toml {
        file: String,
        messaggio: String,
    },
    SchemaDiverso {
        trovata: u32,
        attesa: u32,
    },
    SalvataggioIlleggibile(String),
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
            Self::IndicatoreDoppio {
                indicatore,
                primo,
                secondo,
            } => write!(
                f,
                "indicatore di servizio assegnato a due dicasteri: {indicatore} ({primo}, {secondo})"
            ),
            Self::FuoriIntervallo {
                nome,
                valore,
                min,
                max,
            } => write!(
                f,
                "manopola fuori intervallo: {nome} = {valore} (ammesso da {min} a {max})"
            ),
            Self::ScenarioFuoriIntervallo {
                nome,
                valore,
                min,
                max,
            } => write!(
                f,
                "valore dello scenario fuori intervallo: {nome} = {valore} (ammesso da {min} a {max})"
            ),
            Self::ManopolaMancante { nome } => write!(f, "manopola mancante: {nome}"),
            Self::SettoreSconosciuto { id, dove } => {
                write!(f, "settore sconosciuto: {id} (in {dove})")
            }
            Self::SettoreNonProduttivo { id } => {
                write!(f, "settore che consuma più di quanto produce: {id}")
            }
            Self::QuoteNonUnitarie { dove } => write!(f, "quote che non sommano a 1: {dove}"),
            Self::StrutturaIncoerente(m) => write!(f, "struttura economica incoerente: {m}"),
            Self::LeggeElettorale(m) => write!(f, "legge elettorale non ammessa: {m}"),
            Self::DatiElettorali(m) => write!(f, "dati elettorali incoerenti: {m}"),
            Self::VotiIncoerenti(m) => write!(f, "voti incoerenti: {m}"),
            Self::Elettorato(m) => write!(f, "dati dell'elettorato non validi: {m}"),
            Self::Partiti(m) => write!(f, "partiti dello scenario non validi: {m}"),
            Self::Grandezze(m) => write!(f, "grandezze dei dicasteri non valide: {m}"),
            Self::Atti(m) => write!(f, "catalogo degli atti non valido: {m}"),
            Self::Disposizioni(m) => write!(f, "disposizioni non valide: {m}"),
            Self::Comando(m) => write!(f, "comando non ammesso: {m}"),
            Self::Bilancio(m) => write!(f, "legge di bilancio non ammessa: {m}"),
            Self::Fondazione(m) => write!(f, "fondazione del partito non valida: {m}"),
            Self::Toml { file, messaggio } => write!(f, "{file}: {messaggio}"),
            Self::SchemaDiverso { trovata, attesa } => write!(
                f,
                "salvataggio di una versione diversa del gioco: schema {trovata}, atteso {attesa}"
            ),
            Self::SalvataggioIlleggibile(m) => write!(f, "salvataggio illeggibile: {m}"),
            Self::SalvataggioNonRiuscito(m) => write!(f, "salvataggio non riuscito: {m}"),
        }
    }
}

impl std::error::Error for Errore {}
