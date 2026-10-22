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
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDicastero {
    pub id: String,
    pub nome: String,
}
