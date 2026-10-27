//! Partiti e governo uscente: dati dello scenario e stato della partita.

use crate::dati::{DatiGioco, Dicastero, Ruolo};
use crate::errore::Errore;
use crate::scenario::{Ingresso, Scenario};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoPartito {
    pub id: String,
    pub nome: String,
    /// Da 1 (estrema sinistra) a 7 (estrema destra).
    pub posizione: f64,
    pub valence: Ingresso,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoGoverno {
    pub presidente: String,
    /// Dicastero -> partito.
    pub dicasteri: BTreeMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Partito {
    pub id: String,
    pub nome: String,
    pub posizione_dichiarata: f64,
    pub posizione_percepita: f64,
    pub valence: f64,
    /// Valence del turno zero: verso questa torna `valence` a ogni turno.
    pub valence_base: f64,
}

/// Indici nei partiti; `dicasteri` è allineato a `DatiGioco::dicasteri()`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Governo {
    pub presidente: usize,
    pub dicasteri: Vec<usize>,
}

impl Governo {
    pub fn al_governo(&self, p: usize) -> bool {
        self.presidente == p || self.dicasteri.contains(&p)
    }

    /// Partito che tiene il dicastero col ruolo `r`.
    ///
    /// Se il dicastero non c'è (impossibile nei dati validati) risponde il presidente.
    pub fn partito_del_ruolo(&self, dati: &DatiGioco, r: Ruolo) -> usize {
        dati.dicasteri()
            .iter()
            .position(|d| d.ruoli.contains(&r))
            .and_then(|i| self.dicasteri.get(i))
            .copied()
            .unwrap_or(self.presidente)
    }
}

/// Partiti e governo del turno zero. I dati sono validati: un id sconosciuto non c'è.
pub fn iniziali(dati: &DatiGioco) -> (Vec<Partito>, Governo) {
    let s = dati.scenario();
    let indice = |id: &String| s.partiti.iter().position(|p| &p.id == id).unwrap_or(0);
    let partiti = s
        .partiti
        .iter()
        .map(|p| Partito {
            id: p.id.clone(),
            nome: p.nome.clone(),
            posizione_dichiarata: p.posizione,
            posizione_percepita: p.posizione,
            valence: p.valence.valore,
            valence_base: p.valence.valore,
        })
        .collect();
    let governo = Governo {
        presidente: indice(&s.governo.presidente),
        dicasteri: dati
            .dicasteri()
            .iter()
            .map(|d| s.governo.dicasteri.get(&d.id).map_or(0, indice))
            .collect(),
    };
    (partiti, governo)
}

fn ko(m: String) -> Errore {
    Errore::Partiti(m)
}

/// Coerenza di partiti, governo e coalizioni (la valence passa da `Scenario::valida`).
///
/// # Errors
/// `Partiti` col nome del colpevole.
pub fn valida(scenario: &Scenario, dicasteri: &[Dicastero]) -> Result<(), Errore> {
    let partiti = &scenario.partiti;
    if !(2..=10).contains(&partiti.len()) {
        return Err(ko(format!("{} partiti, servono da 2 a 10", partiti.len())));
    }
    let mut visti = BTreeSet::new();
    for p in partiti {
        if !visti.insert(p.id.as_str()) {
            return Err(ko(format!("partito ripetuto: {}", p.id)));
        }
        if !(p.posizione.is_finite() && (1.0..=7.0).contains(&p.posizione)) {
            return Err(ko(format!("posizione di {} fuori da 1-7", p.id)));
        }
    }
    let esiste = |id: &str| visti.contains(id);
    let g = &scenario.governo;
    if !esiste(&g.presidente) {
        return Err(ko(format!("presidente sconosciuto: {}", g.presidente)));
    }
    if let Some(d) = dicasteri.iter().find(|d| !g.dicasteri.contains_key(&d.id)) {
        return Err(ko(format!("dicastero senza partito: {}", d.id)));
    }
    if let Some(id) = g
        .dicasteri
        .keys()
        .find(|k| !dicasteri.iter().any(|d| &d.id == *k))
    {
        return Err(ko(format!(
            "il governo assegna un dicastero sconosciuto: {id}"
        )));
    }
    if let Some((d, p)) = g.dicasteri.iter().find(|(_, p)| !esiste(p)) {
        return Err(ko(format!(
            "dicastero {d} assegnato a un partito sconosciuto: {p}"
        )));
    }
    let mut in_coalizione = BTreeSet::new();
    for id in scenario.coalizioni.iter().flatten() {
        if !esiste(id) {
            return Err(ko(format!("coalizione con un partito sconosciuto: {id}")));
        }
        if !in_coalizione.insert(id.as_str()) {
            return Err(ko(format!("partito in due coalizioni: {id}")));
        }
    }
    Ok(())
}
