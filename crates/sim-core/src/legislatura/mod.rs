//! La legislatura: fase, scadenza e coalizioni dell'ultimo voto.

use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub(crate) mod cassa;
mod coalizione;
mod crisi;
mod eredita;
mod fondazione;
pub(crate) mod movimento;
mod programma;
mod spartizione;
mod voto;
#[cfg(test)]
pub(crate) use coalizione::CausaDisponibilita;
pub use coalizione::{Cessione, Dichiarazione, VoceDisponibilita};
pub(crate) use coalizione::{Quadro, cedi, coalizioni_al_voto, dichiarazione, rotture};
pub(crate) use crisi::{linea_sfiducia, valore_al_governo, vota_la_caduta};
pub use eredita::{ConfrontoIndicatore, Eredita};
pub use fondazione::{AnteprimaFondazione, Fondazione, VocePosizione, posizioni};
pub(crate) use fondazione::{partito_nuovo, quota_fra_i_votanti};
pub(crate) use programma::programma;
pub use spartizione::{Assegnazione, VoceRichiesta};
pub(crate) use spartizione::{
    maggioranza, maggioranza_contigua, primo_per_seggi, richiesta, spartizione_di_default,
};
pub use voto::{EsitoElezioni, Incarico, VotiPartito};
pub(crate) use voto::{incarico, voti_per_regione};

/// `Sciolta`: Camere sciolte, campagna, voto nella chiusura. `Trattativa`: il turno dopo il voto.
/// `Ordinaria`: governo in carica.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Fase {
    Sciolta,
    Trattativa,
    Ordinaria,
}

/// `coalizioni`: quelle dell'ultimo voto, indici di partito; alla nascita quelle dello scenario.
/// `dichiarazione`: la coalizione pre-elettorale del turno del voto, cancellata dal voto.
/// `elezioni`: gli esiti dei voti, il più recente per primo. `totali`: i seggi di Camera e Senato
/// del parlamento in carica. `incarico`: a chi va dopo l'ultimo voto. `proposta`: la spartizione
/// proposta in trattativa, un partito per dicastero nell'ordine dei dati (`None` = un tecnico).
/// `allargati`: i partiti entrati con `Allarga` nella coalizione del giocatore in trattativa o in
/// crisi, fuori da `coalizioni`; vuoto altrove. `dimissioni`: il governo cade alla chiusura del
/// turno. `maggioranza`: la coalizione del ribaltone col giocatore primo per seggi, in trattativa;
/// sostituisce la sua coalizione dell'ultimo voto. `crisi`: il turno di crisi in corso, dopo
/// l'uscita di un alleato senza maggioranza; si chiude nella sua chiusura. `voto_anticipato`: le
/// `dimissioni` sono del governo IA che va al voto, e il Capo dello Stato esclude il suo partito.
/// `riposizionato`: il giocatore ha già spostato la dichiarata in questa campagna; azzerato al voto.
/// `protesta`: in questa legislatura è già nato un partito di protesta; azzerata al voto.
/// `frange`: i partiti nati da una scissione in questa legislatura; azzerate al voto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Legislatura {
    pub inizio: u32,
    pub scadenza: u32,
    pub fase: Fase,
    pub coalizioni: Vec<Vec<usize>>,
    pub dichiarazione: Option<Dichiarazione>,
    pub elezioni: Vec<EsitoElezioni>,
    pub totali: (u32, u32),
    pub incarico: Option<Incarico>,
    pub proposta: Option<Vec<Option<usize>>>,
    pub allargati: Vec<usize>,
    pub dimissioni: bool,
    pub maggioranza: Option<Vec<usize>>,
    pub crisi: Option<u32>,
    pub voto_anticipato: bool,
    pub riposizionato: bool,
    pub protesta: bool,
    pub frange: Vec<usize>,
}

impl Legislatura {
    /// Si vota nella chiusura del `turno`: Camere sciolte o legislatura alla scadenza.
    pub fn turno_di_voto(&self, turno: u32) -> bool {
        self.fase == Fase::Sciolta || turno == self.scadenza
    }

    /// Il `turno` cade negli ultimi quattro della legislatura.
    pub fn ultimo_anno(&self, turno: u32) -> bool {
        turno.saturating_add(4) > self.scadenza
    }
}

/// Come finisce la partita: senza seggi dopo il voto, o per scelta del giocatore.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Fine {
    SenzaSeggi,
    Abbandono,
}
