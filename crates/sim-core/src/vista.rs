use crate::grandezze::Verso;
use crate::informazione::{Revisione, Sondaggio, Stima};
use crate::ministri::{Causa, StatoAgenda, Variazione};
use crate::partita::Trimestre;
use serde::Serialize;
#[cfg(test)]
use ts_rs::TS;

/// Ciò che la UI può leggere della partita.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Vista {
    pub turno: u32,
    pub data: Trimestre,
    pub dicasteri: Vec<VoceDicastero>,
    /// Valori alla chiusura precedente: le grandezze arrivano con un turno di ritardo.
    pub grandezze: Vec<VoceGrandezza>,
    /// Spread BTP-Bund in punti base, arrotondato.
    pub spread_pb: i32,
    pub partiti: Vec<VocePartito>,
    /// Un ministero per dicastero, nell'ordine dei dicasteri.
    pub ministeri: Vec<VoceMinistero>,
    pub sondaggio: Sondaggio,
    /// Seggi sul sondaggio pubblico; `None` solo se il riparto rifiuta i voti.
    pub proiezione: Option<Vec<SeggiPartito>>,
    pub statistica: VistaStatistica,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct SeggiPartito {
    pub partito: String,
    pub camera: u32,
    pub senato: u32,
}

/// Valence esatta: è un ingresso dello scenario, non stato nascosto.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePartito {
    pub id: String,
    pub nome: String,
    pub posizione_dichiarata: f64,
    pub posizione_percepita: f64,
    pub valence: f64,
    pub al_governo: bool,
}

/// La statistica senza `vera`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaStatistica {
    pub trimestre: Trimestre,
    pub prima_stima: Stima,
    pub revisione: Option<Revisione>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDicastero {
    pub id: String,
    pub nome: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceGrandezza {
    pub id: String,
    pub nome: String,
    pub verso: Verso,
    /// Una voce, o una per regione.
    pub valori: Vec<f64>,
}

/// Il ministro (o l'interim con la sua rosa) di un dicastero.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMinistero {
    pub dicastero: String,
    /// `None` = interim.
    pub ministro: Option<VoceMinistro>,
    pub rosa: Vec<VoceMinistro>,
    /// Perché il ministro è uscito; `Some` solo in interim.
    pub causa_dimissioni: Option<Causa>,
    /// L'atto annunciato per il turno; `None` in interim o a margine esaurito.
    pub agenda: Option<VoceAgenda>,
    /// Perché manca l'agenda: `interim` o `margine esaurito`.
    pub motivo_senza_agenda: Option<String>,
    /// I 4 atti del dicastero.
    pub atti: Vec<VoceAtto>,
    pub cantieri: Vec<VoceCantiere>,
    /// Euro, prima delle rate dei cantieri.
    pub margine_residuo: i64,
}

/// `atto` e `regione` sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAgenda {
    pub atto: String,
    pub nome: String,
    pub regione: Option<String>,
    /// Euro per turno di lavori.
    pub costo: i64,
    pub motivo: String,
    pub stato: StatoAgenda,
    pub in_evidenza: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAtto {
    pub id: String,
    pub nome: String,
    pub motto: String,
    /// Euro per turno di lavori.
    pub costo: i64,
    pub turni: u32,
    pub pagabile: bool,
    pub effetti: Vec<VoceEffetto>,
}

/// `grandezza` è un id; `ritardo` quello scritto, in turni.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceEffetto {
    pub grandezza: String,
    pub migliora: bool,
    pub ritardo: u32,
}

/// `atto` e `regione` sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceCantiere {
    pub atto: String,
    pub regione: Option<String>,
    pub turni_mancanti: u32,
    pub rata: i64,
    pub speso: i64,
}

/// Il ministro com'è visibile al giocatore: soddisfazione, agende e capacità, non lo stato nascosto.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMinistro {
    pub nome: String,
    /// Id del partito.
    pub partito: String,
    pub capacita: u8,
    pub posizione: f64,
    pub obiettivo: f64,
    pub soddisfazione: f64,
    pub rodaggio: bool,
    pub minaccia: bool,
    pub causa_principale: Option<Causa>,
    /// Variazioni di soddisfazione degli ultimi due turni, con la loro causa.
    pub variazioni: Vec<Variazione>,
}
