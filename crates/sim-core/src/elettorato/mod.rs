//! Elettorato: i 360 gruppi (regione x condizione x fascia di reddito), i loro dati
//! (`elettorato.toml`) e il loro stato.

use crate::dati::{DatiGioco, Dicastero};
use crate::economia::{Economia, Scaglione, irpef};
use crate::errore::Errore;
use crate::misure::Misure;
use crate::struttura::Regione;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

mod composizione;
mod giudizio;
mod redditi;
mod voto;
pub use redditi::{Indici, Reddito};
pub use voto::{Intenzioni, intenzioni};

/// Punti dell'istogramma di posizione.
pub const PUNTI: usize = 7;

const TOLLERANZA: f64 = 1e-6;

/// Crescite ricordate al massimo: il `max` della manopola `finestra_aspettative`.
pub(crate) const FINESTRA_MASSIMA: usize = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Condizione {
    DipendentePrivato,
    DipendentePubblico,
    Autonomo,
    Pensionato,
    DisoccupatoInattivo,
    Studente,
}

impl Condizione {
    pub const TUTTE: [Condizione; 6] = [
        Condizione::DipendentePrivato,
        Condizione::DipendentePubblico,
        Condizione::Autonomo,
        Condizione::Pensionato,
        Condizione::DisoccupatoInattivo,
        Condizione::Studente,
    ];
}

impl fmt::Display for Condizione {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Condizione::DipendentePrivato => "dipendente_privato",
            Condizione::DipendentePubblico => "dipendente_pubblico",
            Condizione::Autonomo => "autonomo",
            Condizione::Pensionato => "pensionato",
            Condizione::DisoccupatoInattivo => "disoccupato_inattivo",
            Condizione::Studente => "studente",
        })
    }
}

/// Fascia di reddito.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fascia {
    Bassa,
    Media,
    Alta,
}

impl Fascia {
    pub const TUTTE: [Fascia; 3] = [Fascia::Bassa, Fascia::Media, Fascia::Alta];
}

impl fmt::Display for Fascia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Fascia::Bassa => "bassa",
            Fascia::Media => "media",
            Fascia::Alta => "alta",
        })
    }
}

/// Gruppi per regione: condizioni x fasce.
pub(crate) const PER_REGIONE: usize = 18;

/// Indice canonico di un gruppo: `regione·18 + condizione·3 + fascia`.
pub fn indice_gruppo(regione: usize, c: Condizione, f: Fascia) -> usize {
    regione * PER_REGIONE + c as usize * Fascia::TUTTE.len() + f as usize
}

/// Condizione del gruppo di indice canonico `i`.
pub fn condizione_di(i: usize) -> Condizione {
    Condizione::TUTTE[i % PER_REGIONE / Fascia::TUTTE.len()]
}

/// Fascia del gruppo di indice canonico `i`.
pub fn fascia_di(i: usize) -> Fascia {
    Fascia::TUTTE[i % Fascia::TUTTE.len()]
}

/// Media di `righe` pesata con `pesi`, lunga `lung`; somma dei pesi positiva.
pub(crate) fn media_pesata<'a>(
    pesi: &[f64],
    righe: impl IntoIterator<Item = &'a [f64]>,
    lung: usize,
) -> Vec<f64> {
    let tot: f64 = pesi.iter().sum();
    let mut m = vec![0.0; lung];
    for (w, r) in pesi.iter().zip(righe) {
        m.iter_mut().zip(r).for_each(|(a, x)| *a += w * x);
    }
    m.iter_mut().for_each(|a| *a /= tot);
    m
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElettoriRegione {
    pub id: String,
    pub elettori: u64,
    pub astensione: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoGruppo {
    pub regione: String,
    pub condizione: Condizione,
    pub fascia: Fascia,
    /// Quota della regione.
    pub quota: f64,
    pub posizione: [f64; PUNTI],
    pub primario: f64,
    pub trasferimenti: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Istituto {
    pub nome: String,
    pub campione: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatiElettorato {
    #[serde(rename = "regione")]
    pub regioni: Vec<ElettoriRegione>,
    #[serde(rename = "gruppo")]
    pub gruppi: Vec<DatoGruppo>,
    pub interessi: BTreeMap<Condizione, BTreeMap<String, f64>>,
    pub aliquote: BTreeMap<Fascia, f64>,
    pub paniere: BTreeMap<Fascia, Vec<f64>>,
    pub astensione_condizione: BTreeMap<Condizione, f64>,
    pub astensione_fascia: BTreeMap<Fascia, f64>,
    #[serde(rename = "istituto")]
    pub istituti: Vec<Istituto>,
    pub nomi: Vec<String>,
    pub cognomi: Vec<String>,
}

fn ko(m: String) -> Errore {
    Errore::Elettorato(m)
}

fn finiti_non_negativi(v: &[f64]) -> bool {
    v.iter().all(|x| x.is_finite() && *x >= 0.0)
}

fn somma_uno(v: &[f64]) -> bool {
    finiti_non_negativi(v) && (v.iter().sum::<f64>() - 1.0).abs() <= TOLLERANZA
}

impl DatiElettorato {
    /// Coerenza dei dati con regioni, dicasteri e settori.
    ///
    /// # Errors
    /// `Elettorato` col nome del colpevole.
    pub fn valida(
        &self,
        regioni: &[Regione],
        dicasteri: &[Dicastero],
        n_settori: usize,
    ) -> Result<(), Errore> {
        let stesse = self
            .regioni
            .iter()
            .map(|r| &r.id)
            .eq(regioni.iter().map(|r| &r.id));
        if !stesse {
            return Err(ko(
                "le regioni non combaciano con quelle di regioni.toml (stesso elenco, stesso ordine)"
                    .into(),
            ));
        }
        for r in &self.regioni {
            if r.elettori == 0 || !r.astensione.is_finite() {
                return Err(ko(format!(
                    "regione {}: elettori nulli o astensione non finita",
                    r.id
                )));
            }
        }
        self.valida_gruppi()?;
        self.valida_interessi(dicasteri)?;
        self.valida_tabelle(n_settori)?;
        if !(3..=4).contains(&self.istituti.len()) || self.istituti.iter().any(|i| i.campione < 100)
        {
            return Err(ko(format!(
                "servono da 3 a 4 istituti, ciascuno con campione di almeno 100: {} istituti, campione minimo {}",
                self.istituti.len(),
                self.istituti.iter().map(|i| i.campione).min().unwrap_or(0)
            )));
        }
        if self.nomi.is_empty() || self.cognomi.is_empty() {
            return Err(ko("nomi e cognomi non possono essere vuoti".into()));
        }
        Ok(())
    }

    fn valida_gruppi(&self) -> Result<(), Errore> {
        if self.gruppi.len() != self.regioni.len() * PER_REGIONE {
            return Err(ko(format!(
                "{} gruppi, ne servono {} (18 per regione)",
                self.gruppi.len(),
                self.regioni.len() * PER_REGIONE
            )));
        }
        for (i, g) in self.gruppi.iter().enumerate() {
            let regione = &self.regioni[i / PER_REGIONE].id;
            let (condizione, fascia) = (condizione_di(i), fascia_di(i));
            if (&g.regione, g.condizione, g.fascia) != (regione, condizione, fascia) {
                return Err(ko(format!(
                    "gruppo {i} fuori ordine: atteso {regione} {condizione} {fascia}, trovato {} {} {}",
                    g.regione, g.condizione, g.fascia
                )));
            }
            if !(g.quota.is_finite() && g.quota >= 0.0) {
                return Err(ko(format!(
                    "gruppo {regione} {condizione} {fascia}: quota non finita o negativa"
                )));
            }
            if !somma_uno(&g.posizione) {
                return Err(ko(format!(
                    "gruppo {regione} {condizione} {fascia}: la posizione somma a {}, non a 1",
                    g.posizione.iter().sum::<f64>()
                )));
            }
            let ok_flussi = finiti_non_negativi(&[g.primario, g.trasferimenti])
                && g.primario + g.trasferimenti > 0.0;
            if !ok_flussi {
                return Err(ko(format!(
                    "gruppo {regione} {condizione} {fascia}: primario e trasferimenti devono essere finiti, non negativi e con somma positiva"
                )));
            }
        }
        for (r, reg) in self.regioni.iter().enumerate() {
            let quote: Vec<f64> = self.gruppi[r * PER_REGIONE..(r + 1) * PER_REGIONE]
                .iter()
                .map(|g| g.quota)
                .collect();
            if !somma_uno(&quote) {
                return Err(ko(format!(
                    "regione {}: le quote dei gruppi sommano a {}, non a 1",
                    reg.id,
                    quote.iter().sum::<f64>()
                )));
            }
        }
        Ok(())
    }

    fn valida_interessi(&self, dicasteri: &[Dicastero]) -> Result<(), Errore> {
        let ammessi: BTreeSet<&str> = ["reddito", "disoccupazione", "occupazione_pubblica"]
            .into_iter()
            .chain(dicasteri.iter().map(|d| d.indicatore_servizio.as_str()))
            .collect();
        for c in Condizione::TUTTE {
            let Some(pesi) = self.interessi.get(&c) else {
                return Err(ko(format!("interessi mancanti per la condizione {c}")));
            };
            if let Some(nome) = pesi.keys().find(|k| !ammessi.contains(k.as_str())) {
                return Err(ko(format!(
                    "interessi di {c}: indicatore sconosciuto {nome}"
                )));
            }
            let v: Vec<f64> = pesi.values().copied().collect();
            if !somma_uno(&v) {
                return Err(ko(format!(
                    "interessi di {c}: i pesi sommano a {}, non a 1",
                    v.iter().sum::<f64>()
                )));
            }
        }
        Ok(())
    }

    fn valida_tabelle(&self, n_settori: usize) -> Result<(), Errore> {
        for f in Fascia::TUTTE {
            let aliquota = self.aliquote.get(&f).copied();
            if !aliquota.is_some_and(|a| (0.0..=0.9).contains(&a)) {
                return Err(ko(format!(
                    "aliquota della fascia {f} mancante o fuori da 0-0,9"
                )));
            }
            match self.paniere.get(&f) {
                Some(p) if p.len() == n_settori && somma_uno(p) => {}
                _ => {
                    return Err(ko(format!(
                        "paniere della fascia {f}: servono {n_settori} quote che sommano a 1"
                    )));
                }
            }
            if !self
                .astensione_fascia
                .get(&f)
                .is_some_and(|x| x.is_finite())
            {
                return Err(ko(format!(
                    "astensione della fascia {f} mancante o non finita"
                )));
            }
        }
        for c in Condizione::TUTTE {
            if !self
                .astensione_condizione
                .get(&c)
                .is_some_and(|x| x.is_finite())
            {
                return Err(ko(format!(
                    "astensione della condizione {c} mancante o non finita"
                )));
            }
        }
        Ok(())
    }
}

/// Stato di un gruppo. `giudizio` ha una voce per partito.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Gruppo {
    pub quota: f64,
    pub posizione: [f64; PUNTI],
    pub ancora: [f64; PUNTI],
    pub giudizio: Vec<f64>,
    /// Voto dell'ultima elezione (fino al piano 7: la stazionaria del turno zero): una quota
    /// per partito, l'ultima è l'astensione.
    pub voto_precedente: Vec<f64>,
    pub reddito: Reddito,
    pub reddito_prima: Reddito,
    /// Ultime crescite reali trimestrali in punti, la più recente in coda.
    pub crescite: Vec<f64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Elettorato {
    pub gruppi: Vec<Gruppo>,
    /// Indici dell'economia al turno zero.
    pub base: Indici,
    /// Disoccupazione per regione letta al turno prima, in %.
    pub ultima_disoccupazione: Vec<f64>,
    /// Aliquota media IRPEF al turno zero, con gli scaglioni dello scenario.
    pub aliquota_irpef_0: f64,
}

impl Elettorato {
    /// Elettori di un gruppo: la sua quota viva x gli elettori della sua regione.
    pub fn elettori(&self, d: &DatiElettorato, i: usize) -> f64 {
        self.gruppi[i].quota * d.regioni[i / PER_REGIONE].elettori as f64
    }

    pub fn iniziale(dati: &DatiGioco, n_partiti: usize, e: &Economia) -> Elettorato {
        let mut el = Elettorato {
            base: Indici::da(e, dati),
            aliquota_irpef_0: 0.0,
            ultima_disoccupazione: e.regioni.iter().map(|r| r.disoccupazione).collect(),
            gruppi: dati
                .elettorato()
                .gruppi
                .iter()
                .map(|g| Gruppo {
                    quota: g.quota,
                    posizione: g.posizione,
                    ancora: g.posizione,
                    giudizio: vec![0.0; n_partiti],
                    voto_precedente: vec![0.0; n_partiti + 1],
                    reddito: Reddito::default(),
                    reddito_prima: Reddito::default(),
                    crescite: Vec::new(),
                })
                .collect(),
        };
        // Due volte, così `reddito_prima == reddito` al turno zero.
        let neutre = Misure::neutre(dati);
        el.aggiorna_redditi(dati, e, &neutre);
        el.aggiorna_redditi(dati, e, &neutre);
        el.aliquota_irpef_0 = el.aliquota_media_irpef(dati, &dati.scenario().aliquote.irpef);
        el
    }

    /// `Σ elettori·irpef(lordo) / Σ elettori·lordo` sui redditi correnti, con gli scaglioni `s`.
    pub(crate) fn aliquota_media_irpef(&self, dati: &DatiGioco, s: &[Scaglione]) -> f64 {
        let d = dati.elettorato();
        let (mut imposta, mut lordo) = (0.0, 0.0);
        for (k, g) in self.gruppi.iter().enumerate() {
            let (n, l) = (
                self.elettori(d, k),
                g.reddito.primario + g.reddito.trasferimenti,
            );
            imposta += n * irpef(s, l);
            lordo += n * l;
        }
        if lordo > 0.0 { imposta / lordo } else { 0.0 }
    }

    /// Lunghezze e finitezza, per uno stato letto da un salvataggio.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` col campo nel messaggio.
    pub fn controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore> {
        let attesi = dati.elettorato().gruppi.len();
        let ko = |m: String| Err(Errore::SalvataggioIlleggibile(m));
        if self.gruppi.len() != attesi {
            return ko(format!(
                "elettorato.gruppi ha {} voci, i gruppi sono {attesi}",
                self.gruppi.len()
            ));
        }
        for (i, g) in self.gruppi.iter().enumerate() {
            if g.giudizio.len() != n_partiti {
                return ko(format!(
                    "elettorato.gruppi[{i}].giudizio ha {} voci, i partiti sono {n_partiti}",
                    g.giudizio.len()
                ));
            }
            if g.voto_precedente.len() != n_partiti + 1 || !somma_uno(&g.voto_precedente) {
                return ko(format!(
                    "elettorato.gruppi[{i}].voto_precedente: servono {} quote che sommano a 1",
                    n_partiti + 1
                ));
            }
            let ok = g.quota.is_finite()
                && g.quota >= 0.0
                && somma_uno(&g.posizione)
                && somma_uno(&g.ancora);
            if !ok {
                return ko(format!(
                    "elettorato.gruppi[{i}]: quota negativa, posizione o ancora che non sommano a 1"
                ));
            }
            if g.giudizio.iter().any(|x| !x.is_finite()) {
                return ko(format!("elettorato.gruppi[{i}].giudizio non finito"));
            }
            if g.crescite.len() > FINESTRA_MASSIMA || g.crescite.iter().any(|x| !x.is_finite()) {
                return ko(format!(
                    "elettorato.gruppi[{i}].crescite: al massimo {FINESTRA_MASSIMA} valori finiti"
                ));
            }
            if !(g.reddito.e_valido() && g.reddito_prima.e_valido()) {
                return ko(format!(
                    "elettorato.gruppi[{i}]: reddito non finito o paniere non positivo"
                ));
            }
        }
        let regioni = dati.elettorato().regioni.len();
        for (r, gruppi) in self.gruppi.chunks(PER_REGIONE).enumerate() {
            let quote: Vec<f64> = gruppi.iter().map(|g| g.quota).collect();
            if !somma_uno(&quote) {
                return ko(format!(
                    "elettorato.gruppi: le quote della regione {r} sommano a {}, non a 1",
                    quote.iter().sum::<f64>()
                ));
            }
        }
        if self.ultima_disoccupazione.len() != regioni
            || self.ultima_disoccupazione.iter().any(|x| !x.is_finite())
        {
            return ko(format!(
                "elettorato.ultima_disoccupazione: servono {regioni} valori finiti"
            ));
        }
        if !self.base.e_finito() {
            return ko("elettorato.base non finita".into());
        }
        if !(self.aliquota_irpef_0.is_finite()
            && self.aliquota_irpef_0 > 0.0
            && self.aliquota_irpef_0 < 1.0)
        {
            return ko("elettorato.aliquota_irpef_0 fuori da (0, 1)".into());
        }
        Ok(())
    }
}
