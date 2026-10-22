pub mod caso;
pub mod dati;
pub mod errore;
pub mod partita;
pub mod vista;

pub use dati::{DatiGioco, FileDati};
pub use errore::Errore;
pub use partita::{Partita, Trimestre, VERSIONE_SCHEMA};
pub use vista::Vista;
