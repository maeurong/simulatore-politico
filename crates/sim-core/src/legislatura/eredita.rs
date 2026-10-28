use super::Fine;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Un indicatore al turno zero e a fine partita.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ConfrontoIndicatore {
    pub id: String,
    pub nome: String,
    pub turno_zero: f64,
    pub fine: f64,
}

/// Ciò che il giocatore lascia a partita finita. `bozza` vale solo per `SenzaSeggi`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Eredita {
    pub fine: Fine,
    pub bozza: bool,
    pub anni_al_governo: f64,
    pub leggi_in_vigore: Vec<String>,
    pub indicatori: Vec<ConfrontoIndicatore>,
}
