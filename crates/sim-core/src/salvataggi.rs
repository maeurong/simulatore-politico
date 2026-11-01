//! I tipi dei salvataggi: la serie degli indicatori, un punto per apertura di turno, e la testata
//! che l'elenco delle partite legge senza caricare la partita.

use crate::legislatura::ConfrontoIndicatore;
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

/// Versione del formato della voce dell'albo: sale solo se un campo cambia significato; i campi
/// nuovi si aggiungono e chi legge ignora quelli che non conosce.
pub const FORMATO_ALBO: u32 = 1;

/// Un indicatore al turno zero e a fine partita, copia propria dell'albo: il formato dell'albo
/// non segue il tipo dell'eredità.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct IndicatoreAlbo {
    pub id: String,
    pub nome: String,
    pub turno_zero: f64,
    pub fine: f64,
}

impl From<ConfrontoIndicatore> for IndicatoreAlbo {
    fn from(c: ConfrontoIndicatore) -> Self {
        Self {
            id: c.id,
            nome: c.nome,
            turno_zero: c.turno_zero,
            fine: c.fine,
        }
    }
}

/// Un voto nell'albo: `voti` = (nome del partito, quota fra i votanti), `seggi_giocatore` =
/// (Camera, Senato) del giocatore, (0, 0) se non c'era a quel voto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ElezioneAlbo {
    pub data: Trimestre,
    pub affluenza: f64,
    pub voti: Vec<(String, f64)>,
    pub seggi_giocatore: (u32, u32),
}

/// La partita finita, come l'albo la elenca da sola. `nome` e `scenario` come nella `Testata`;
/// `turni` = le chiusure fatte; `esito` = «Senza seggi» o «Abbandono»; `elezioni` nell'ordine della
/// legislatura, la più recente per prima. Con «Senza seggi» la serie ha `turni` punti (la chiusura
/// che finisce la partita non apre un turno), con «Abbandono» ne ha `turni + 1`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAlbo {
    pub formato_albo: u32,
    pub nome: String,
    pub scenario: String,
    pub inizio: Trimestre,
    pub fine: Trimestre,
    pub turni: u32,
    pub esito: String,
    pub anni_al_governo: f64,
    pub leggi_in_vigore: Vec<String>,
    pub indicatori: Vec<IndicatoreAlbo>,
    pub serie: Vec<PuntoSerie>,
    pub elezioni: Vec<ElezioneAlbo>,
    pub riavvolgimenti: u32,
}
