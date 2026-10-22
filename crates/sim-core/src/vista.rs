use crate::informazione::{Revisione, Sondaggio, Stima};
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
    /// Spread BTP-Bund in punti base, arrotondato.
    pub spread_pb: i32,
    pub partiti: Vec<VocePartito>,
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
