use crate::partita::Trimestre;
use serde::Serialize;

/// Ciò che la UI può leggere della partita.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Vista {
    pub turno: u32,
    pub data: Trimestre,
    pub dicasteri: Vec<VoceDicastero>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct VoceDicastero {
    pub id: String,
    pub nome: String,
}
