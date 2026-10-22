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
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDicastero {
    pub id: String,
    pub nome: String,
}
