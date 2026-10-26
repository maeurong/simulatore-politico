//! Ministri dei dicasteri: generati dal seme, con soddisfazione, dimissioni e successione.
//! Un dicastero senza ministro è in interim; la rosa dei successori dura un turno.

use crate::caso::{Caso, Flusso};
use crate::comando::Comando;
use crate::dati::DatiGioco;
use crate::disposizioni::Disposizioni;
use crate::economia::Economia;
use crate::errore::Errore;
use crate::grandezze::Grandezze;
use crate::informazione::{Sondaggio, indice};
use crate::partita::Trimestre;
use crate::partiti::{Governo, Partito};
use crate::vista::{VoceMinistero, VoceMinistro};
use serde::{Deserialize, Serialize};
use std::fmt;
#[cfg(test)]
use ts_rs::TS;

mod agenda;
mod proposte;
mod soddisfazione;

pub use agenda::{Agenda, Cantiere, StatoAgenda};
pub use proposte::Proposta;

/// Perché la soddisfazione di un ministro è cambiata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Causa {
    IndicatoreInCalo,
    ObiettivoRaggiunto,
    DimissioniChieste,
    AttoFermato,
    AttoDirottato,
    CantiereFermato,
    PropostaRespinta,
    BudgetTagliato,
}

impl fmt::Display for Causa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::IndicatoreInCalo => "indicatore di servizio in calo",
            Self::ObiettivoRaggiunto => "obiettivo raggiunto",
            Self::DimissioniChieste => "dimissioni chieste dal Presidente del Consiglio",
            Self::AttoFermato => "atto fermato dal Presidente del Consiglio",
            Self::AttoDirottato => "atto dirottato dal Presidente del Consiglio",
            Self::CantiereFermato => "cantiere fermato dal Presidente del Consiglio",
            Self::PropostaRespinta => "proposta di fondi respinta",
            Self::BudgetTagliato => "spesa del dicastero tagliata in termini reali",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Variazione {
    pub causa: Causa,
    pub delta: f64,
    pub turno: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ministro {
    pub nome: String,
    pub partito: usize,
    pub capacita: u8,
    pub posizione: f64,
    pub obiettivo: f64,
    pub soddisfazione: f64,
    pub turni_in_carica: u32,
}

impl Ministro {
    /// Nel rodaggio (prima del primo turno in carica) la capacità cala di `riduzione_rodaggio`,
    /// minimo 1.
    pub fn capacita_effettiva(&self, dati: &DatiGioco) -> f64 {
        let c = f64::from(self.capacita);
        if self.turni_in_carica == 0 {
            (c - dati.manopola("riduzione_rodaggio")).max(1.0)
        } else {
            c
        }
    }
}

/// `ministro = None` è l'interim.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StatoDicastero {
    pub ministro: Option<Ministro>,
    pub rosa: Vec<Ministro>,
    pub scelta: Option<u8>,
    pub dimissioni_chieste: bool,
    pub variazioni: Vec<Variazione>,
    /// Perché il ministro è uscito; `Some` solo in interim.
    pub causa_dimissioni: Option<Causa>,
    /// L'atto annunciato per il turno in corso; `None` in interim o a margine esaurito.
    pub agenda: Option<Agenda>,
    pub cantieri: Vec<Cantiere>,
    /// Euro ancora spendibili nell'anno, prima delle rate dei cantieri.
    pub margine_residuo: i64,
    /// La richiesta di fondi in attesa di una decisione; `None` in interim.
    pub proposta: Option<Proposta>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ministri {
    pub dicasteri: Vec<StatoDicastero>,
}

/// Ciò che i ministri leggono della partita.
pub(crate) struct Contesto<'a> {
    pub dati: &'a DatiGioco,
    pub governo: &'a Governo,
    pub partiti: &'a [Partito],
    pub grandezze: &'a Grandezze,
    pub economia: &'a Economia,
    pub sondaggio: &'a Sondaggio,
    pub disposizioni: &'a Disposizioni,
    /// L'indice dei prezzi pubblicato, non il livello vero.
    pub indice_prezzi: f64,
    pub turno: u32,
    pub data: Trimestre,
}

const NELLA_ROSA: usize = 3;

/// Indice nei dati della grandezza che è l'indicatore di servizio del dicastero `i`.
fn indicatore(cx: &Contesto, i: usize) -> Option<usize> {
    cx.dati
        .grandezza(&cx.dati.dicasteri()[i].indicatore_servizio)
}

impl Ministri {
    /// I ministri del governo uscente, già oltre il rodaggio.
    pub(crate) fn iniziali(cx: &Contesto, caso: &mut Caso) -> Ministri {
        let n = cx.dati.dicasteri().len();
        let mut m = Ministri {
            dicasteri: vec![StatoDicastero::default(); n],
        };
        for i in 0..n {
            let mut x = m.genera(cx, caso, i);
            x.turni_in_carica = 1;
            m.dicasteri[i].ministro = Some(x);
        }
        m.apri_agende(cx, caso);
        m
    }

    /// Un ministro nuovo per il dicastero `i`, del partito che lo tiene. Il nome non coincide
    /// con quello di un ministro in carica o in rosa (al più 100 riestrazioni). L'obiettivo parte
    /// dall'indicatore com'è visto, con un turno di ritardo.
    fn genera(&self, cx: &Contesto, caso: &mut Caso, i: usize) -> Ministro {
        let el = cx.dati.elettorato();
        let preso = |nome: &str| {
            (self.dicasteri.iter())
                .flat_map(|s| s.ministro.iter().chain(&s.rosa))
                .any(|x| x.nome == nome)
        };
        let mut nome = String::new();
        for _ in 0..100 {
            let n = &el.nomi[indice(caso.uniforme(Flusso::Governo), el.nomi.len())];
            let c = &el.cognomi[indice(caso.uniforme(Flusso::Governo), el.cognomi.len())];
            nome = format!("{n} {c}");
            if !preso(&nome) {
                break;
            }
        }
        let partito = cx.governo.dicasteri[i];
        let capacita = 5.min(1 + (5.0 * caso.uniforme(Flusso::Governo)) as u8);
        let scarto = cx.dati.manopola("scarto_ministri") * caso.normale(Flusso::Governo);
        let posizione = (cx.partiti[partito].posizione_dichiarata + scarto).clamp(1.0, 7.0);
        let obiettivo = indicatore(cx, i).map_or(0.0, |k| {
            cx.grandezze.nazionale_prima(cx.dati, k)
                + cx.dati.grandezze()[k].verso.segno() * cx.dati.manopola("passo_obiettivo")
        });
        Ministro {
            nome,
            partito,
            capacita,
            posizione,
            obiettivo,
            soddisfazione: cx.dati.manopola("soddisfazione_base"),
            turni_in_carica: 0,
        }
    }

    /// Soglie di fine turno: servizio, obiettivo, ritorno alla base, dimissioni.
    pub(crate) fn soglie(&mut self, cx: &Contesto) {
        for (i, st) in self.dicasteri.iter_mut().enumerate() {
            // I dati validati hanno l'indicatore; senza, restano turni e dimissioni.
            let servizio = indicatore(cx, i).map(|k| {
                (
                    cx.dati.grandezze()[k].verso,
                    cx.grandezze.nazionale(cx.dati, k),
                    cx.grandezze.variazione(cx.dati, k),
                )
            });
            soddisfazione::passo(st, cx.dati, servizio, cx.turno);
            if st.ministro.is_none() {
                st.proposta = None;
            }
        }
    }

    /// Apertura del turno (`cx.turno` è già quello nuovo): variazioni vecchie, rose per gli
    /// interim appena aperti, ingresso del successore dopo il turno di interim.
    pub(crate) fn apri(&mut self, cx: &Contesto, caso: &mut Caso) {
        for i in 0..self.dicasteri.len() {
            let st = &mut self.dicasteri[i];
            st.variazioni
                .retain(|v| v.turno.saturating_add(1) >= cx.turno);
            if st.ministro.is_some() {
                continue;
            }
            if st.rosa.is_empty() {
                for _ in 0..NELLA_ROSA {
                    let x = self.genera(cx, caso, i);
                    self.dicasteri[i].rosa.push(x);
                }
                self.dicasteri[i].rosa.sort_by(|a, b| {
                    (b.capacita.cmp(&a.capacita)).then_with(|| a.nome.cmp(&b.nome))
                });
            } else {
                let k = usize::from(st.scelta.unwrap_or(0));
                st.ministro = st.rosa.drain(..).nth(k);
                if let Some(x) = st.ministro.as_mut() {
                    x.turni_in_carica = 0;
                }
                st.scelta = None;
                st.causa_dimissioni = None;
            }
        }
        self.apri_agende(cx, caso);
    }

    /// Forma e valori di un salvataggio contro i dati.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` se i dicasteri non sono quelli dei dati, un partito esce dai
    /// limiti, una capacità non è 1-5, una soddisfazione non è finita in 0-100, la rosa non è di
    /// 0 o 3 o la scelta esce dalla rosa.
    pub(crate) fn controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore> {
        let ko = |m: String| Err(Errore::SalvataggioIlleggibile(format!("ministri: {m}")));
        let n = dati.dicasteri().len();
        if self.dicasteri.len() != n {
            return ko(format!(
                "{} dicasteri, i dati ne hanno {n}",
                self.dicasteri.len()
            ));
        }
        for (d, st) in dati.dicasteri().iter().zip(&self.dicasteri) {
            if !matches!(st.rosa.len(), 0 | NELLA_ROSA) {
                return ko(format!(
                    "{}: rosa di {} invece di 0 o 3",
                    d.id,
                    st.rosa.len()
                ));
            }
            if st.causa_dimissioni.is_some() && st.ministro.is_some() {
                return ko(format!("{}: causa delle dimissioni con un ministro", d.id));
            }
            if st.dimissioni_chieste && st.ministro.is_none() {
                return ko(format!("{}: dimissioni chieste in interim", d.id));
            }
            if !st.rosa.is_empty() && st.ministro.is_some() {
                return ko(format!("{}: rosa con un ministro in carica", d.id));
            }
            if st.scelta.is_some_and(|s| usize::from(s) >= st.rosa.len()) {
                return ko(format!("{}: scelta fuori dalla rosa", d.id));
            }
            for x in st.ministro.iter().chain(&st.rosa) {
                let buono = x.partito < n_partiti
                    && (1..=5).contains(&x.capacita)
                    && (0.0..=100.0).contains(&x.soddisfazione)
                    && x.posizione.is_finite()
                    && x.obiettivo.is_finite();
                if !buono {
                    return ko(format!("{}: ministro {} fuori dai limiti", d.id, x.nome));
                }
            }
        }
        (self.controlla_agende(dati))
            .and_then(|()| self.controlla_proposte(dati))
            .or_else(ko)
    }

    /// Esegue un comando del giocatore; un rifiuto non tocca nulla. `Accogli` rende l'importo
    /// del decreto: lo spende chi possiede l'economia. Invariante: `PresentaBilancio` non arriva
    /// mai qui (`Partita::comanda` lo intercetta), quindi «comando senza dicastero» non si
    /// raggiunge.
    pub(crate) fn comanda(&mut self, cx: &Contesto, c: Comando) -> Result<Option<i64>, Errore> {
        let (dati, ko) = (cx.dati, |m: String| Err(Errore::Comando(m)));
        let (Comando::ChiediDimissioni { dicastero }
        | Comando::ScegliSuccessore { dicastero, .. }
        | Comando::Ferma { dicastero }
        | Comando::Dirotta { dicastero, .. }
        | Comando::FermaCantiere { dicastero, .. }
        | Comando::Accogli { dicastero }
        | Comando::Rinvia { dicastero }
        | Comando::Respingi { dicastero }) = &c
        else {
            return ko("comando senza dicastero".into());
        };
        let Some(i) = dati.dicasteri().iter().position(|d| &d.id == dicastero) else {
            return ko(format!("dicastero sconosciuto: {dicastero}"));
        };
        if matches!(c, Comando::Accogli { .. }) && cx.economia.esercizio_provvisorio(cx.data) {
            return ko(format!(
                "esercizio provvisorio: nessuna misura che costi denaro: {dicastero}"
            ));
        }
        let st = &mut self.dicasteri[i];
        let sul_ministro = matches!(
            c,
            Comando::ChiediDimissioni { .. } | Comando::ScegliSuccessore { .. }
        );
        if st.dimissioni_chieste && !sul_ministro {
            return ko(format!("dimissioni chieste: {dicastero}"));
        }
        match c {
            Comando::Accogli { .. } => return self.accogli(cx, i).map(Some),
            Comando::Rinvia { .. } => self.rinvia(cx, i)?,
            Comando::Respingi { .. } => self.respingi(cx, i)?,
            Comando::Ferma { .. } => self.ferma(cx, i)?,
            Comando::Dirotta { atto, regione, .. } => {
                self.dirotta(cx, i, atto.as_deref(), regione.as_deref())?;
            }
            Comando::FermaCantiere { atto, .. } => self.ferma_cantiere(cx, i, &atto)?,
            Comando::ChiediDimissioni { dicastero } => {
                if st.ministro.is_none() {
                    return ko(format!("{dicastero} è in interim"));
                }
                if st.dimissioni_chieste {
                    return ko(format!("dimissioni già chieste: {dicastero}"));
                }
                st.dimissioni_chieste = true;
            }
            Comando::ScegliSuccessore { dicastero, indice } => {
                if st.ministro.is_some() {
                    return ko(format!("{dicastero} non è in interim"));
                }
                if usize::from(indice) >= st.rosa.len() {
                    return ko(format!(
                        "successore fuori dalla rosa di {dicastero}: {indice}"
                    ));
                }
                st.scelta = Some(indice);
            }
            // Escluso dal `let ... else` in testa; lo intercetta `Partita::comanda`.
            Comando::PresentaBilancio { .. } => {}
        }
        Ok(None)
    }

    /// Le voci per la `Vista`, nell'ordine dei dicasteri.
    pub(crate) fn voci(&self, cx: &Contesto) -> Vec<VoceMinistero> {
        let (dati, partiti) = (cx.dati, cx.partiti);
        let soglia = dati.manopola("soglia_minaccia");
        let voce = |x: &Ministro, variazioni: &[Variazione]| VoceMinistro {
            nome: x.nome.clone(),
            partito: partiti[x.partito].id.clone(),
            capacita: x.capacita,
            posizione: x.posizione,
            obiettivo: x.obiettivo,
            soddisfazione: x.soddisfazione,
            rodaggio: x.turni_in_carica == 0,
            minaccia: x.soddisfazione < soglia,
            causa_principale: causa_principale(variazioni),
            variazioni: variazioni.to_vec(),
        };
        let rif = agenda::margini_rif(dati);
        (dati.dicasteri().iter().zip(&self.dicasteri))
            .enumerate()
            .map(|(i, (d, st))| {
                let g = self.voci_governo(cx, &rif, i);
                VoceMinistero {
                    dicastero: d.id.clone(),
                    ministro: st.ministro.as_ref().map(|x| voce(x, &st.variazioni)),
                    rosa: st.rosa.iter().map(|x| voce(x, &[])).collect(),
                    dimissioni_chieste: st.dimissioni_chieste,
                    scelta: st.scelta,
                    causa_dimissioni: st.causa_dimissioni,
                    agenda: g.agenda,
                    motivo_senza_agenda: g.motivo_senza_agenda,
                    atti: g.atti,
                    cantieri: g.cantieri,
                    margine_residuo: st.margine_residuo,
                    proposta: self.voce_proposta(cx, i),
                }
            })
            .collect()
    }
}

/// La causa con la somma più negativa; `None` se nessuna somma è negativa.
pub(super) fn causa_principale(variazioni: &[Variazione]) -> Option<Causa> {
    let mut somme: Vec<(Causa, f64)> = Vec::new();
    for v in variazioni {
        match somme.iter_mut().find(|(c, _)| *c == v.causa) {
            Some((_, s)) => *s += v.delta,
            None => somme.push((v.causa, v.delta)),
        }
    }
    let peggiore = somme.into_iter().fold(None, |p: Option<(Causa, f64)>, x| {
        if x.1 < p.map_or(0.0, |q| q.1) {
            Some(x)
        } else {
            p
        }
    });
    peggiore.map(|(c, _)| c)
}
