//! Partiti e governo uscente: dati dello scenario e stato della partita.

use crate::dati::{DatiGioco, Dicastero, Ruolo};
use crate::errore::Errore;
use crate::scenario::{Ingresso, Scenario};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use ts_rs::TS;

/// L'id del partito del giocatore, riservato: nessun partito dello scenario può averlo.
pub const ID_GIOCATORE: &str = "giocatore";

/// Tetto ai partiti attivi (vita-dei-partiti:121: liste con seggi 9-14, NEV 3,5-6,4).
pub const MASSIMO_PARTITI: usize = 10;

/// Regola di movimento di un partito sull'asse (abm-elettorali:14-18).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Regola {
    Sticker,
    Aggregator,
    Hunter,
    Governator,
}

/// Quanto un partito pesa le politiche rispetto alle poltrone.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Vocazione {
    Identitaria,
    Mista,
    Governista,
}

impl Vocazione {
    fn alfa(self, dati: &DatiGioco) -> f64 {
        dati.manopola(match self {
            Self::Identitaria => "alfa_identitaria",
            Self::Mista => "alfa_mista",
            Self::Governista => "alfa_governista",
        })
    }

    /// 2α: peso delle politiche nell'utilità del partito (α = 0,5 pesa 1).
    pub fn peso_politiche(self, dati: &DatiGioco) -> f64 {
        2.0 * self.alfa(dati)
    }

    /// 2(1 − α): peso dell'ufficio (poltrone) nell'utilità del partito.
    pub fn peso_poltrone(self, dati: &DatiGioco) -> f64 {
        2.0 * (1.0 - self.alfa(dati))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoPartito {
    pub id: String,
    pub nome: String,
    /// Da 1 (estrema sinistra) a 7 (estrema destra).
    pub posizione: f64,
    pub valence: Ingresso,
    pub vocazione: Vocazione,
    pub leader: String,
    pub regola: Regola,
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
    pub vocazione: Vocazione,
    pub leader: String,
    pub regola: Regola,
    /// Resta al suo indice, senza voti.
    pub sciolto: bool,
    /// Euro, mai sotto zero (scelta 10 del piano 8b).
    pub cassa: i64,
}

/// Indici nei partiti; `dicasteri` è allineato a `DatiGioco::dicasteri()`; `coalizione` = i
/// partiti della maggioranza, presidente compreso, anche senza dicasteri. `programma`: il programma
/// pubblico del governo IA, (disposizione, gradino) alla sua nascita; vuoto col giocatore
/// Presidente del Consiglio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Governo {
    pub presidente: usize,
    pub dicasteri: Vec<usize>,
    pub coalizione: Vec<usize>,
    pub programma: Vec<(usize, usize)>,
}

impl Governo {
    pub fn al_governo(&self, p: usize) -> bool {
        self.presidente == p || self.dicasteri.contains(&p) || self.coalizione.contains(&p)
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
            vocazione: p.vocazione,
            leader: p.leader.clone(),
            regola: p.regola,
            sciolto: false,
            cassa: 0,
        })
        .collect();
    let presidente = indice(&s.governo.presidente);
    let dicasteri: Vec<usize> = (dati.dicasteri().iter())
        .map(|d| s.governo.dicasteri.get(&d.id).map_or(0, indice))
        .collect();
    let coalizione = (std::iter::once(presidente).chain(dicasteri.iter().copied()))
        .collect::<BTreeSet<usize>>()
        .into_iter()
        .collect();
    let governo = Governo {
        presidente,
        dicasteri,
        coalizione,
        programma: Vec::new(),
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
    if !(2..=MASSIMO_PARTITI).contains(&partiti.len()) {
        return Err(ko(format!(
            "{} partiti, servono da 2 a {MASSIMO_PARTITI}",
            partiti.len()
        )));
    }
    let mut visti = BTreeSet::new();
    for p in partiti {
        if !visti.insert(p.id.as_str()) {
            return Err(ko(format!("partito ripetuto: {}", p.id)));
        }
        if p.id == ID_GIOCATORE {
            return Err(ko(format!(
                "id riservato al partito del giocatore: {ID_GIOCATORE}"
            )));
        }
        if p.leader.trim().is_empty() {
            return Err(ko(format!("leader vuoto: {}", p.id)));
        }
        if !(p.posizione.is_finite() && (1.0..=7.0).contains(&p.posizione)) {
            return Err(ko(format!("posizione di {} fuori da 1-7", p.id)));
        }
    }
    if scenario.nomi_partiti.is_empty() {
        return Err(ko("nomi_partiti: nessun nome per i partiti nuovi".into()));
    }
    let usato = |n: &&String| partiti.iter().any(|p| &p.nome == *n);
    if let Some(nome) = scenario.nomi_partiti.iter().find(usato) {
        return Err(ko(format!("nome di partito nuovo già usato: {nome}")));
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
    if let Some(c) = scenario.coalizioni.iter().find(|c| c.len() < 2) {
        return Err(ko(format!(
            "coalizione di meno di due partiti: {}",
            c.join("+")
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    #[test]
    fn i_pesi_della_vocazione() {
        let d = dati_base();
        let casi = [
            (Vocazione::Identitaria, 1.5, 0.5),
            (Vocazione::Mista, 1.0, 1.0),
            (Vocazione::Governista, 0.5, 1.5),
        ];
        for (v, politiche, poltrone) in casi {
            assert!((v.peso_politiche(&d) - politiche).abs() < 1e-12, "{v:?}");
            assert!((v.peso_poltrone(&d) - poltrone).abs() < 1e-12, "{v:?}");
        }
    }

    #[test]
    fn nomi_partiti_vuota_o_gia_usata_rifiutata() {
        let d = dati_base();
        let mut s = d.scenario().clone();
        assert!(s.nomi_partiti.len() >= 8);
        assert!(valida(&s, d.dicasteri()).is_ok());
        s.nomi_partiti.push("Centro".into());
        let e = valida(&s, d.dicasteri()).unwrap_err();
        assert!(
            e.to_string()
                .contains("nome di partito nuovo già usato: Centro"),
            "{e}"
        );
        s.nomi_partiti.clear();
        assert!(valida(&s, d.dicasteri()).is_err());
    }
}
