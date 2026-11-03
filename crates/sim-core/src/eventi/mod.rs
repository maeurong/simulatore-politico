//! Eventi e situazioni (piano 9): il catalogo dei dati, gli eventi aperti nella partita e la
//! risposta del giocatore. Un evento aperto vale un turno: alla chiusura si risolve con l'opzione
//! scelta o, senza scelta, con il default.

use crate::atti::Forma;
use crate::dati::DatiGioco;
use crate::errore::Errore;
use crate::grandezze::Verso;
use crate::partita::{LIMITE_EURO, vita::virgola};
use crate::vista::Fascia3;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub(crate) mod emergenze;
pub(crate) mod rischio;
pub(crate) mod situazioni;

/// Le misure che non sono grandezze ma che il motore calcola: crescita annualizzata vera del PIL
/// reale ed `Economia.energia`.
const MISURE_DEL_MOTORE: [&str; 2] = ["crescita_pil", "energia"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Famiglia {
    Situazione,
    Emergenza,
    Scandalo,
    Shock,
    Debito,
}

/// Gli eventi gravi non hanno battute (storia 136).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Tono {
    Ordinario,
    Grave,
}

/// Cosa fa un'opzione. `Grandezza`, `Valence`, `FondoRiserva` e `Debito` sono gli unici che
/// `valida` ammette finora: gli altri li applicano i task che li introducono.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum EffettoOpzione {
    /// Come un effetto di atto, ma senza capacità del ministro: `intensita` vale tale e quale.
    /// Su una grandezza regionale cade nella regione dell'evento, se c'è.
    Grandezza {
        grandezza: String,
        intensita: f64,
        ritardo: u32,
        forma: Forma,
        durata: f64,
    },
    /// Al partito del bersaglio dell'evento se c'è (il leader, o il partito del ministro),
    /// altrimenti a quello del Presidente del Consiglio.
    Valence {
        delta: f64,
    },
    /// Spende il costo dell'evento dal fondo di riserva (fino al residuo dell'anno) sul primo
    /// dicastero dell'evento, nel turno. Le `Grandezza` che seguono in elenco valgono per la quota
    /// pagata; l'emergenza ripristina in proporzione il suo danno.
    FondoRiserva,
    /// Come `FondoRiserva`, ma per intero e a disavanzo.
    Debito,
    Dimissioni,
    ClausolaEnergia,
    Mes,
    Ristrutturazione,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoOpzione {
    pub id: String,
    pub nome: String,
    /// Vale se il giocatore non sceglie, o se governa l'IA.
    pub default: bool,
    pub effetti: Vec<EffettoOpzione>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoEvento {
    pub id: String,
    pub famiglia: Famiglia,
    pub nome: String,
    /// Nel tetto di un evento a turno vince la gravità più alta.
    pub gravita: u8,
    /// Dicasteri (id) le cui disposizioni ordinarie il decreto innescato ammette.
    pub dicasteri: Vec<String>,
    pub decreto: bool,
    /// Fra le opzioni c'è il decreto a debito.
    pub debito: bool,
    pub opzioni: Vec<DatoOpzione>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DatoSituazione {
    pub id: String,
    pub nome: String,
    /// Una grandezza (id) o una misura del motore (`crescita_pil`, `energia`).
    pub misura: String,
    /// Il verso in cui la situazione si ACCENDE: `Sale` accende sopra `accensione` e spegne sotto
    /// `spegnimento`, `Scende` il contrario. Non è il verso in cui la grandezza migliora: per le
    /// carceri quello è `Scende`, questo `Sale`.
    pub verso: Verso,
    pub accensione: f64,
    pub spegnimento: f64,
    /// L'evento (id) che si apre quando la situazione si accende.
    pub evento: String,
    /// Spinge di un punto verso destra le posizioni dei gruppi finché è accesa.
    pub spinta: bool,
}

/// Perché l'evento è nato, con i valori letti: mai la probabilità in cifre.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Causa {
    Soglia {
        misura: String,
        valore: f64,
        soglia: f64,
    },
    Rischio {
        livello: Fascia3,
    },
    Shock {
        serie: String,
        sigma: f64,
    },
    Spread {
        pb: i32,
        soglia: i32,
    },
}

impl Causa {
    /// La causa per la vista, col nome della grandezza se `misura` lo è.
    pub fn testo(&self, dati: &DatiGioco) -> String {
        match self {
            Causa::Soglia {
                misura,
                valore,
                soglia,
            } => {
                let (v, s) = (virgola(*valore), virgola(*soglia));
                match misura.as_str() {
                    "crescita_pil" => format!("crescita del PIL: {v}% annuo (soglia {s}%)"),
                    "energia" => format!("scarto dei prezzi dell'energia: {v} (soglia {s})"),
                    _ => {
                        let nome = (dati.grandezza(misura))
                            .and_then(|k| dati.grandezze().get(k))
                            .map_or(misura.as_str(), |g| g.nome.as_str());
                        format!("{nome}: {v} (soglia {s})")
                    }
                }
            }
            Causa::Rischio { livello } => format!(
                "rischio {}",
                match livello {
                    Fascia3::Poco => "basso",
                    Fascia3::Medio => "medio",
                    Fascia3::Molto => "alto",
                }
            ),
            Causa::Shock { serie, sigma } => format!("{serie}: shock di {} sigma", virgola(*sigma)),
            Causa::Spread { pb, soglia } => format!("spread a {pb} pb (soglia {soglia} pb)"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Bersaglio {
    Ministro { dicastero: usize },
    Leader { partito: usize },
}

/// Un evento nato e non ancora risolto; `evento` è l'indice nel catalogo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventoAperto {
    pub evento: usize,
    pub turno: u32,
    pub causa: Causa,
    pub regione: Option<usize>,
    pub bersaglio: Option<Bersaglio>,
    pub tono: Tono,
    /// Euro.
    pub costo: i64,
    /// Indice dell'opzione scelta; `None` = vale il default.
    pub scelta: Option<usize>,
}

impl EventoAperto {
    /// Indice dell'opzione di default (0 se il catalogo non lo dice: `valida` lo esclude).
    pub fn default_di(&self, dati: &DatiGioco) -> usize {
        (dati.eventi().get(self.evento))
            .and_then(|e| e.opzioni.iter().position(|o| o.default))
            .unwrap_or(0)
    }
}

/// Lo stato degli eventi nella partita: `accese` e `contatori` hanno una voce per situazione.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Eventi {
    pub aperti: Vec<EventoAperto>,
    /// Emergenze lievi nate in questa chiusura: non sono eventi (il giocatore non le vede), le
    /// paga il fondo alla chiusura dopo, nel trimestre in cui la spesa cade.
    pub lievi: Vec<EventoAperto>,
    pub accese: Vec<bool>,
    /// Trimestri consecutivi dalla parte giusta della soglia (la recessione ne conta due).
    pub contatori: Vec<u8>,
}

/// Cosa innesca il decreto su disposizioni: i dicasteri (indici) degli eventi aperti che lo
/// ammettono.
pub(crate) struct Inneschi {
    pub dicasteri: Vec<usize>,
}

impl Eventi {
    /// Gli eventi aperti con un dicastero `id` fra i loro, come indici del catalogo.
    pub(crate) fn nel_dicastero<'a>(
        &'a self,
        dati: &'a DatiGioco,
        id: &'a str,
    ) -> impl Iterator<Item = usize> + 'a {
        (self.aperti.iter().map(|a| a.evento)).filter(move |&e| {
            (dati.eventi().get(e)).is_some_and(|x| x.dicasteri.iter().any(|d| d == id))
        })
    }

    pub(crate) fn inneschi(&self, dati: &DatiGioco) -> Inneschi {
        let dicasteri = (self.aperti.iter())
            .filter_map(|a| dati.eventi().get(a.evento))
            .filter(|e| e.decreto)
            .flat_map(|e| &e.dicasteri)
            .filter_map(|id| dati.dicasteri().iter().position(|d| &d.id == id))
            .collect();
        Inneschi { dicasteri }
    }

    pub fn iniziali(dati: &DatiGioco) -> Eventi {
        let n = dati.situazioni().len();
        Eventi {
            aperti: Vec::new(),
            lievi: Vec::new(),
            accese: vec![false; n],
            contatori: vec![0; n],
        }
    }

    /// Chiude gli eventi aperti: per ciascuno l'opzione scelta o il default, da applicare a chi
    /// chiama (le grandezze e i partiti non sono qui). Non estrae nulla.
    pub(crate) fn risolvi<'d>(
        &mut self,
        dati: &'d DatiGioco,
    ) -> Vec<(EventoAperto, &'d DatoOpzione)> {
        std::mem::take(&mut self.aperti)
            .into_iter()
            .filter_map(|a| {
                let k = a.scelta.unwrap_or_else(|| a.default_di(dati));
                let opzione = dati.eventi().get(a.evento)?.opzioni.get(k)?;
                Some((a, opzione))
            })
            .collect()
    }

    /// Indici e lunghezze di un salvataggio contro i dati: evento, opzione, regione, bersaglio;
    /// `accese` e `contatori` lunghi quanto le situazioni.
    pub(crate) fn controlla(
        &self,
        dati: &DatiGioco,
        partiti: usize,
        turno: u32,
    ) -> Result<(), Errore> {
        let ko = |m: &str| Err(Errore::SalvataggioIlleggibile(format!("eventi: {m}")));
        let n = dati.situazioni().len();
        if self.accese.len() != n || self.contatori.len() != n {
            return ko("accese e contatori non combaciano con le situazioni dei dati");
        }
        for a in self.aperti.iter().chain(&self.lievi) {
            let Some(e) = dati.eventi().get(a.evento) else {
                return ko("evento aperto fuori dal catalogo");
            };
            if a.scelta.is_some_and(|k| k >= e.opzioni.len()) {
                return ko("scelta fuori dalle opzioni dell'evento");
            }
            if a.regione.is_some_and(|r| r >= dati.regioni().len()) {
                return ko("regione fuori dai dati");
            }
            let bersaglio_ok = match a.bersaglio {
                None => true,
                Some(Bersaglio::Ministro { dicastero }) => dicastero < dati.dicasteri().len(),
                Some(Bersaglio::Leader { partito }) => partito < partiti,
            };
            if !bersaglio_ok {
                return ko("bersaglio fuori dai dati");
            }
            let causa_ok = match &a.causa {
                Causa::Soglia { valore, soglia, .. } => valore.is_finite() && soglia.is_finite(),
                Causa::Shock { sigma, .. } => sigma.is_finite(),
                Causa::Rischio { .. } | Causa::Spread { .. } => true,
            };
            if !causa_ok || a.turno > turno || !(0..=LIMITE_EURO).contains(&a.costo) {
                return ko("causa, turno o costo di un evento aperto fuori misura");
            }
        }
        Ok(())
    }
}

/// Coerenza del catalogo con i dicasteri e le grandezze; i messaggi portano il nome dell'evento o
/// della situazione.
pub(crate) fn valida(dati: &DatiGioco) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Eventi(m));
    let eventi = dati.eventi();
    for (i, e) in eventi.iter().enumerate() {
        let nome = &e.nome;
        if eventi[..i].iter().any(|p| p.id == e.id) {
            return ko(format!("evento ripetuto: {}", e.id));
        }
        if !(1..=3).contains(&e.opzioni.len()) {
            return ko(format!(
                "{nome}: servono da 1 a 3 opzioni, ce ne sono {}",
                e.opzioni.len()
            ));
        }
        let default = e.opzioni.iter().filter(|o| o.default).count();
        if default != 1 {
            return ko(format!(
                "{nome}: serve esattamente un'opzione di default, ce ne sono {default}"
            ));
        }
        if let Some(id) = e
            .dicasteri
            .iter()
            .find(|id| !dati.dicasteri().iter().any(|d| &d.id == *id))
        {
            return ko(format!("{nome}: dicastero sconosciuto {id}"));
        }
        for (k, o) in e.opzioni.iter().enumerate() {
            if e.opzioni[..k].iter().any(|p| p.id == o.id) {
                return ko(format!("{nome}: opzione ripetuta {}", o.id));
            }
            let spende = |x: &EffettoOpzione| {
                matches!(x, EffettoOpzione::FondoRiserva | EffettoOpzione::Debito)
            };
            if o.effetti.iter().any(spende) && e.dicasteri.is_empty() {
                return ko(format!(
                    "{nome}: l'opzione {} spende dal primo dicastero e l'evento non ne ha",
                    o.id
                ));
            }
            for effetto in &o.effetti {
                valida_effetto(dati, nome, effetto)?;
            }
        }
    }
    let richiesti = (emergenze::EVENTI.iter().map(|id| (id, Famiglia::Emergenza)))
        .chain(rischio::SCANDALI.iter().map(|id| (id, Famiglia::Scandalo)));
    for (id, famiglia) in richiesti {
        match dati.evento(id).and_then(|k| eventi.get(k)) {
            None => return ko(format!("evento richiesto dal motore mancante: {id}")),
            Some(e) if e.famiglia != famiglia => {
                return ko(format!("{}: serve un'altra famiglia", e.nome));
            }
            Some(_) => {}
        }
    }
    for (i, s) in dati.situazioni().iter().enumerate() {
        let nome = &s.nome;
        if dati.situazioni()[..i].iter().any(|p| p.id == s.id) {
            return ko(format!("situazione ripetuta: {}", s.id));
        }
        if dati.grandezza(&s.misura).is_none() && !MISURE_DEL_MOTORE.contains(&s.misura.as_str()) {
            return ko(format!("{nome}: misura sconosciuta {}", s.misura));
        }
        let giuste = match s.verso {
            Verso::Sale => s.accensione > s.spegnimento,
            Verso::Scende => s.accensione < s.spegnimento,
        };
        if !(s.accensione.is_finite() && s.spegnimento.is_finite() && giuste) {
            return ko(format!(
                "{nome}: l'accensione {} deve stare oltre lo spegnimento {}, nel verso",
                s.accensione, s.spegnimento
            ));
        }
        if dati.evento(&s.evento).is_none() {
            return ko(format!("{nome}: evento sconosciuto {}", s.evento));
        }
    }
    Ok(())
}

fn valida_effetto(dati: &DatiGioco, nome: &str, effetto: &EffettoOpzione) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Eventi(format!("{nome}: {m}")));
    match effetto {
        EffettoOpzione::Grandezza {
            grandezza,
            intensita,
            ritardo,
            forma,
            durata,
        } => {
            let Some(g) = (dati.grandezza(grandezza)).and_then(|k| dati.grandezze().get(k)) else {
                return ko(format!("grandezza sconosciuta {grandezza}"));
            };
            if !(intensita.is_finite() && *intensita != 0.0 && intensita.abs() <= g.max - g.min) {
                return ko(format!("{grandezza}: intensita fuori intervallo"));
            }
            if *forma == Forma::Livello {
                return ko(format!(
                    "{grandezza}: forma livello solo per le disposizioni"
                ));
            }
            if *ritardo > 40 {
                return ko(format!("{grandezza}: ritardo fuori intervallo"));
            }
            if !(durata.is_finite() && *durata > 0.0 && *durata <= 1000.0) {
                return ko(format!("{grandezza}: durata fuori intervallo"));
            }
            Ok(())
        }
        EffettoOpzione::Valence { delta } if delta.is_finite() => Ok(()),
        EffettoOpzione::Valence { .. } => ko("valence non finita".into()),
        EffettoOpzione::FondoRiserva | EffettoOpzione::Debito | EffettoOpzione::Dimissioni => {
            Ok(())
        }
        _ => ko("effetto non ancora ammesso nei dati".into()),
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce è il fallimento voluto"
)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    fn aperto(dati: &DatiGioco, scelta: Option<usize>) -> EventoAperto {
        EventoAperto {
            evento: dati.evento("criminalita").unwrap(),
            turno: 0,
            causa: Causa::Rischio {
                livello: Fascia3::Poco,
            },
            regione: None,
            bersaglio: None,
            tono: Tono::Ordinario,
            costo: 0,
            scelta,
        }
    }

    #[test]
    fn risolvi_svuota_e_senza_scelta_da_il_default() {
        let dati = dati_base();
        let mut e = Eventi::iniziali(&dati);
        e.aperti = vec![aperto(&dati, None), aperto(&dati, Some(1))];
        assert_eq!(e.aperti[0].default_di(&dati), 0);
        let risolti = e.risolvi(&dati);
        assert!(e.aperti.is_empty());
        let id: Vec<&str> = risolti.iter().map(|(_, o)| o.id.as_str()).collect();
        assert_eq!(id, ["ordinaria", "presidi"]);
    }

    #[test]
    fn la_causa_nomina_anche_le_misure_del_motore() {
        let dati = dati_base();
        let soglia = |misura: &str| Causa::Soglia {
            misura: misura.into(),
            valore: -0.2,
            soglia: 0.0,
        };
        assert_eq!(
            soglia("crescita_pil").testo(&dati),
            "crescita del PIL: -0,2% annuo (soglia 0,0%)"
        );
        assert_eq!(
            soglia("energia").testo(&dati),
            "scarto dei prezzi dell'energia: -0,2 (soglia 0,0)"
        );
    }

    #[test]
    fn il_catalogo_di_base_ha_sei_situazioni_con_il_loro_evento() {
        let dati = dati_base();
        assert_eq!(dati.situazioni().len(), 6);
        assert_eq!(Eventi::iniziali(&dati).accese, vec![false; 6]);
        assert!(
            dati.situazioni()
                .iter()
                .all(|s| dati.evento(&s.evento).is_some())
        );
    }
}
