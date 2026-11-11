//! I tipi dei salvataggi: la serie degli indicatori, un punto per apertura di turno, e la testata
//! che l'elenco delle partite legge senza caricare la partita.

use crate::legislatura::ConfrontoIndicatore;
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Gli indicatori all'apertura del turno `turno`. `consenso`: quota fra i votanti della
/// coalizione del giocatore nel sondaggio pubblico (media degli istituti). `margine_*`: seggi
/// della coalizione meno la metà più uno del ramo, in seggi e negativo sotto la maggioranza. La
/// metà più uno è quella delle Camere in carica (`legislatura.totali`), anche sciolte e in
/// prorogatio (allora la coalizione non ha seggi); le Leve si usano solo coi totali a zero, cioè
/// alla fondazione, prima del primo voto. `spread_pb` in punti base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PuntoSerie {
    pub turno: u32,
    pub data: Trimestre,
    pub consenso: f64,
    pub margine_camera: i32,
    pub margine_senato: i32,
    pub spread_pb: i32,
}

/// Ciò che l'elenco delle partite mostra: `nome` è il nome del partito del giocatore, `scenario`
/// quello dello scenario. Regole del formato come per `FORMATO_ALBO`: i campi aggiunti dopo la v1
/// sono `Option<T>`, mai `serde(default)`; non si rinomina, non si toglie, non si cambia tipo
/// (ricerca salvataggi-e-riavvolgimento:65).
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
/// nuovi si aggiungono e chi legge ignora quelli che non conosce. Regola del formato: i campi
/// aggiunti dopo la v1 sono `Option<T>`, mai `serde(default)`; non si rinomina, non si toglie,
/// non si cambia tipo (ricerca persistenza:94). Dalla v2 l'indicatore `inflazione` a fine partita
/// è l'annua (prezzi sullo stesso trimestre dell'anno prima); nella v1 era quella del trimestre,
/// annualizzata.
pub const FORMATO_ALBO: u32 = 2;

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
/// (Camera, Senato) del giocatore. (0, 0) vale sia per un giocatore assente al voto sia per zero
/// seggi: se era presente lo si legge da `voti`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ElezioneAlbo {
    pub data: Trimestre,
    pub affluenza: f64,
    pub voti: Vec<(String, f64)>,
    pub seggi_giocatore: (u32, u32),
}

/// La partita finita, come l'albo la elenca da sola. `nome` e `scenario` come nella `Testata`;
/// `turni` = le chiusure fatte; `esito` = soltanto «Senza seggi» o «Abbandono», testo stabile del formato v1; `elezioni` nell'ordine della
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
    /// Il valore dello snapshot, modificabile a mano: il contatore vero lo tiene il guscio fuori
    /// dallo snapshot e lo reimposta all'apertura (ricerca persistenza:129).
    pub riavvolgimenti: u32,
}

/// Versione del formato dell'indice di una partita. Stesse regole di `FORMATO_ALBO`, che valgono
/// anche per la `Testata` che l'indice contiene: l'elenco mostra gli indici d'altro formato come non
/// apribili, le scritture li rifiutano.
pub const FORMATO_INDICE: u32 = 1;

/// L'indice di una partita in corso, in chiaro accanto agli snapshot: l'elenco lo legge senza
/// aprire la partita, anche con uno schema diverso. `ultimo_salvataggio` in secondi Unix.
/// `riavvolgimenti` è il contatore che fa testo: il guscio lo reimpone alla partita a ogni
/// apertura, perché lo snapshot di un turno passato ne porta il valore vecchio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct IndicePartita {
    pub formato_indice: u32,
    pub id: String,
    pub testata: Testata,
    /// `number` in TS: l'IPC lo consegna come numero JSON, e i secondi Unix stanno in 2^53.
    #[cfg_attr(test, ts(type = "number"))]
    pub ultimo_salvataggio: u64,
    pub riavvolgimenti: u32,
}

/// Una partita dell'elenco: `apribile` se il formato dell'indice e lo schema sono quelli di questa
/// versione, altrimenti `motivo` nomina il valore trovato e quello atteso.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePartita {
    pub indice: IndicePartita,
    pub apribile: bool,
    pub motivo: Option<String>,
}
