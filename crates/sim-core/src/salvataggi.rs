//! I tipi dei salvataggi: la serie degli indicatori, un punto per apertura di turno, e la testata
//! che l'elenco delle partite legge senza caricare la partita.

use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Gli indicatori all'apertura del turno `turno`. `consenso`: quota fra i votanti della
/// coalizione del giocatore nel sondaggio pubblico (media degli istituti). `maggioranza_*`: seggi
/// della coalizione meno la metà più uno del ramo nella legge in vigore, negativi sotto la
/// maggioranza (a Camere sciolte la coalizione non ha seggi). `spread_pb` in punti base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PuntoSerie {
    pub turno: u32,
    pub data: Trimestre,
    pub consenso: f64,
    pub maggioranza_camera: i32,
    pub maggioranza_senato: i32,
    pub spread_pb: i32,
}

/// Ciò che l'elenco delle partite mostra: `nome` è il nome del partito del giocatore, `scenario`
/// quello dello scenario.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Testata {
    pub nome: String,
    pub scenario: String,
    pub turno: u32,
    pub data: Trimestre,
    pub versione_schema: u32,
    pub finita: bool,
}
