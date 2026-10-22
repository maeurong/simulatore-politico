//! Dati elettorali (`elezioni.toml`): circoscrizioni, menu delle leggi e impostazioni predefinite.

mod riparto;
mod seggi;

pub use riparto::{Esito, SeggiLista, Soggetto, Voti, riparto};
pub use seggi::{SeggiCircoscrizione, seggi_da_quota, seggi_per_circoscrizione};

use crate::errore::Errore;
use crate::struttura::Regione;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Circoscrizione {
    pub id: String,
    pub regione: String,
    pub popolazione: u64,
    /// `> 0`: circoscrizione speciale, fuori dalla quota di premio e dal riparto nazionale.
    #[serde(default)]
    pub uninominali: u32,
    #[serde(default)]
    pub seggi_fissi: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ramo {
    pub seggi: u32,
    pub estero: u32,
    pub minimo: u32,
    #[serde(rename = "circoscrizione")]
    pub circoscrizioni: Vec<Circoscrizione>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuoteTetto {
    pub camera_pb: u16,
    pub senato_pb: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Premio {
    Nessuno,
    Lista,
    Coalizione,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Formula {
    Hare,
    Dhondt,
}

/// I nove parametri del menu; quote in punti base.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LeggeElettorale {
    pub soglia_lista_pb: u16,
    pub coalizioni: bool,
    pub soglia_coalizione_pb: u16,
    pub ripescaggio: bool,
    pub premio: Premio,
    pub soglia_premio_pb: u16,
    pub taglia_premio_pb: u16,
    /// Assente = nessun tetto.
    #[serde(default)]
    pub tetto: Option<QuoteTetto>,
    pub formula: Formula,
}

/// Gradini ammessi per ogni parametro numerico.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Menu {
    pub soglia_lista_pb: Vec<u16>,
    pub soglia_coalizione_pb: Vec<u16>,
    pub soglia_premio_pb: Vec<u16>,
    pub taglia_premio_pb: Vec<u16>,
    pub tetto: Vec<QuoteTetto>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatiElezioni {
    pub soglia_regionale_senato_pb: u16,
    pub camera: Ramo,
    pub senato: Ramo,
    pub menu: Menu,
    pub impostazioni: BTreeMap<String, LeggeElettorale>,
}

fn incoerente(m: String) -> Errore {
    Errore::DatiElettorali(m)
}

fn gradini(nome: &str, v: &[u16]) -> Result<(), Errore> {
    if v.is_empty() || !v.windows(2).all(|w| w[0] < w[1]) || v.iter().any(|&x| x > 10_000) {
        return Err(incoerente(format!(
            "menu: {nome} deve avere gradini crescenti, non vuoti e al più 10000: {v:?}"
        )));
    }
    Ok(())
}

fn nel_menu(nome: &str, valore: u16, g: &[u16]) -> Result<(), Errore> {
    if g.contains(&valore) {
        Ok(())
    } else {
        Err(Errore::LeggeElettorale(format!(
            "{nome} = {valore} non è un gradino del menu {g:?}"
        )))
    }
}

impl LeggeElettorale {
    /// Ogni valore numerico è un gradino del menu; il premio è coerente con le coalizioni.
    ///
    /// # Errors
    /// `LeggeElettorale` col nome del parametro fuori menu, o dell'incoerenza premio/coalizioni.
    pub fn valida(&self, menu: &Menu) -> Result<(), Errore> {
        nel_menu(
            "soglia_lista_pb",
            self.soglia_lista_pb,
            &menu.soglia_lista_pb,
        )?;
        nel_menu(
            "soglia_coalizione_pb",
            self.soglia_coalizione_pb,
            &menu.soglia_coalizione_pb,
        )?;
        nel_menu(
            "soglia_premio_pb",
            self.soglia_premio_pb,
            &menu.soglia_premio_pb,
        )?;
        nel_menu(
            "taglia_premio_pb",
            self.taglia_premio_pb,
            &menu.taglia_premio_pb,
        )?;
        if let Some(t) = self.tetto
            && !menu.tetto.contains(&t)
        {
            return Err(Errore::LeggeElettorale(format!(
                "tetto = {t:?} non è un gradino del menu"
            )));
        }
        // assunzione A7 del piano 2: premio alla coalizione solo con le coalizioni, alla lista
        // solo senza.
        match (self.premio, self.coalizioni) {
            (Premio::Coalizione, false) => Err(Errore::LeggeElettorale(
                "premio alla coalizione senza coalizioni".into(),
            )),
            (Premio::Lista, true) => Err(Errore::LeggeElettorale(
                "premio alla lista con le coalizioni attive".into(),
            )),
            _ => Ok(()),
        }
    }
}

impl Menu {
    fn valida(&self) -> Result<(), Errore> {
        gradini("soglia_lista_pb", &self.soglia_lista_pb)?;
        gradini("soglia_coalizione_pb", &self.soglia_coalizione_pb)?;
        gradini("soglia_premio_pb", &self.soglia_premio_pb)?;
        gradini("taglia_premio_pb", &self.taglia_premio_pb)?;
        if self.tetto.is_empty()
            || self
                .tetto
                .iter()
                .any(|t| t.camera_pb > 10_000 || t.senato_pb > 10_000)
            || !self
                .tetto
                .windows(2)
                .all(|w| w[0].camera_pb < w[1].camera_pb && w[0].senato_pb < w[1].senato_pb)
        {
            return Err(incoerente(
                "menu: tetto deve avere gradini crescenti, non vuoti e al più 10000".into(),
            ));
        }
        if let Some(&p) = self.soglia_premio_pb.iter().find(|&&p| p < 4000) {
            return Err(Errore::LeggeElettorale(format!(
                "soglia_premio_pb = {p}: il premio richiede almeno il 40% (4000)"
            )));
        }
        Ok(())
    }
}

impl Ramo {
    fn valida(&self, nome: &str, regioni: &[Regione]) -> Result<(), Errore> {
        let mut visti = BTreeSet::new();
        for c in &self.circoscrizioni {
            if !visti.insert(c.id.as_str()) {
                return Err(incoerente(format!(
                    "{nome}: circoscrizione ripetuta: {}",
                    c.id
                )));
            }
            if c.popolazione == 0 {
                return Err(incoerente(format!(
                    "{nome}: circoscrizione {} senza popolazione",
                    c.id
                )));
            }
            if !regioni.iter().any(|r| r.id == c.regione) {
                return Err(incoerente(format!(
                    "{nome}: circoscrizione {} in regione sconosciuta: {}",
                    c.id, c.regione
                )));
            }
        }
        if let Some(r) = regioni
            .iter()
            .find(|r| !self.circoscrizioni.iter().any(|c| c.regione == r.id))
        {
            return Err(incoerente(format!(
                "{nome}: regione senza circoscrizioni: {}",
                r.id
            )));
        }
        if self.seggi > 10_000 {
            return Err(incoerente(format!(
                "{nome}: seggi = {} oltre il limite di 10000",
                self.seggi
            )));
        }
        let impegnati = self
            .circoscrizioni
            .iter()
            .map(|c| c.seggi_fissi.unwrap_or(self.minimo))
            .try_fold(u64::from(self.estero), |a, s| a.checked_add(u64::from(s)));
        if impegnati.is_none_or(|t| t > u64::from(self.seggi)) {
            return Err(incoerente(format!(
                "{nome}: estero, seggi fissi e minimi superano i seggi ({})",
                self.seggi
            )));
        }
        Ok(())
    }

    fn per_regione(&self, nome: &str) -> Result<BTreeMap<&str, u64>, Errore> {
        let mut m = BTreeMap::new();
        for c in &self.circoscrizioni {
            let p = m.entry(c.regione.as_str()).or_insert(0u64);
            *p = p.checked_add(c.popolazione).ok_or_else(|| {
                incoerente(format!("{nome}: popolazione di {} fuori scala", c.regione))
            })?;
        }
        Ok(m)
    }
}

impl DatiElezioni {
    /// Menu, impostazioni, rami e accordo fra le popolazioni dei due rami.
    ///
    /// # Errors
    /// `LeggeElettorale` per menu e impostazioni, `DatiElettorali` per i rami.
    pub(crate) fn valida(&self, regioni: &[Regione]) -> Result<(), Errore> {
        self.menu.valida()?;
        for (nome, l) in &self.impostazioni {
            l.valida(&self.menu).map_err(|e| match e {
                Errore::LeggeElettorale(m) => {
                    Errore::LeggeElettorale(format!("impostazione {nome}: {m}"))
                }
                e => e,
            })?;
        }
        self.camera.valida("camera", regioni)?;
        self.senato.valida("senato", regioni)?;
        let c = self.camera.per_regione("camera")?;
        let s = self.senato.per_regione("senato")?;
        if let Some(r) = c.keys().chain(s.keys()).find(|r| c.get(*r) != s.get(*r)) {
            let pop = |m: &BTreeMap<&str, u64>| m.get(r).map_or("assente".into(), u64::to_string);
            return Err(incoerente(format!(
                "regione {r}: popolazione {} alla camera, {} al senato",
                pop(&c),
                pop(&s)
            )));
        }
        Ok(())
    }
}
