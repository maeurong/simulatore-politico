//! Comandi del giocatore alla partita.

use crate::economia::Bilancio;
use crate::errore::Errore;
use crate::legislatura::{Assegnazione, Cessione, Fase, Spesa};
use crate::ministri::EffettoComando;
use crate::parlamento::{Decreto, ValoreProposto};
use crate::partita::Partita;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Comando {
    ChiediDimissioni {
        dicastero: String,
    },
    ScegliSuccessore {
        dicastero: String,
        indice: u8,
    },
    Ferma {
        dicastero: String,
    },
    Dirotta {
        dicastero: String,
        atto: Option<String>,
        regione: Option<String>,
    },
    FermaCantiere {
        dicastero: String,
        atto: String,
    },
    Accogli {
        dicastero: String,
    },
    Rinvia {
        dicastero: String,
    },
    Respingi {
        dicastero: String,
    },
    /// Presenta la legge di bilancio del turno, sostituendo quella già presentata; l'aula la
    /// vota in chiusura. La `spesa` di ogni voce è il totale voluto: chi cambia salari,
    /// acquisti o margine la ricalcola, altrimenti la differenza finisce nel resto.
    PresentaBilancio {
        bilancio: Bilancio,
    },
    /// Presenta una legge: la mette in calendario fino al voto.
    PresentaLegge {
        valori: Vec<ValoreProposto>,
    },
    /// Accoglie o respinge l'emendamento del partito `partito` (id) sulla misura.
    Emendamento {
        misura: u32,
        partito: String,
        accogli: bool,
    },
    /// Pone la questione di fiducia: la misura si vota a fine turno.
    PoniFiducia {
        misura: u32,
    },
    /// Emana un decreto: vale subito, a fine turno è convertito o decade.
    EmanaDecreto {
        decreto: Decreto,
    },
    /// Invita a disertare il referendum abrogativo indetto, una volta.
    InvitaAstensione,
    /// Dichiara la coalizione pre-elettorale nel turno del voto: `partiti` (id) senza il
    /// giocatore, vuoto = da solo. Sostituisce la dichiarazione precedente.
    DichiaraCoalizione {
        partiti: Vec<String>,
        cessioni: Vec<Cessione>,
    },
    /// Propone la spartizione in trattativa, con l'incarico: un'assegnazione per dicastero.
    /// Sostituisce la proposta precedente.
    ProponiSpartizione {
        dicasteri: Vec<Assegnazione>,
    },
    /// Allarga la coalizione in trattativa, con l'incarico, ai `partiti` (id): entrano tutti se
    /// ognuno è disponibile coi seggi del voto e la lista intera, o nessuno. Restano fino alla
    /// formazione del governo.
    Allarga {
        partiti: Vec<String>,
    },
    /// Il Presidente del Consiglio si dimette: il governo cade alla chiusura del turno.
    Dimettiti,
    /// Nel ribaltone col giocatore non primo per seggi, chiede la guida al primo: se la cede,
    /// l'incarico passa al giocatore. Senza scelta, la chiusura la chiede.
    ChiediGuida,
    /// Nel ribaltone col giocatore non primo per seggi, ne resta fuori: il Capo dello Stato cerca
    /// la maggioranza senza di lui.
    RifiutaRibaltone,
    /// All'opposizione, la linea del partito del giocatore sulla misura; al governo, solo sulle
    /// proposte dell'opposizione. `None` torna a quella di default.
    Linea {
        misura: u32,
        si: Option<bool>,
    },
    /// All'opposizione, la mozione di sfiducia al governo (`None`) o al ministro del `dicastero`
    /// (id): si vota in chiusura, dopo leggi e referendum.
    Sfiducia {
        dicastero: Option<String>,
    },
    /// All'opposizione, una legge nel posto riservato, nel turno in cui è del giocatore: si vota
    /// al turno dopo.
    PresentaProposta {
        valori: Vec<ValoreProposto>,
    },
    /// All'opposizione, il referendum abrogativo su `disposizione` (id).
    PromuoviReferendum {
        disposizione: String,
    },
    /// All'opposizione, nel turno dopo la seconda deliberazione della costituzionale `misura`
    /// approvata senza i due terzi: chiede il referendum confermativo (`chiedi`) o vi rinuncia.
    Confermativo {
        misura: u32,
        chiedi: bool,
    },
    /// In campagna elettorale, una volta, sposta la dichiarata del suo partito su `posizione`:
    /// al più `passo_movimento` di distanza, senza cambiare il suo rango nella fila per (dichiarata,
    /// indice).
    Riposiziona {
        posizione: f64,
    },
    /// Nel turno del voto, la campagna del suo partito: fino a 3 bersagli distinti, una leva
    /// ciascuno, euro positivi entro la cassa. Sostituisce quella già scelta; si paga in
    /// chiusura, prima del voto.
    Campagna {
        spese: Vec<Spesa>,
    },
}

impl Partita {
    /// La guardia dei comandi di governo e delle loro anteprime.
    ///
    /// # Errors
    /// `Comando` a partita finita o, se il giocatore non governa, col motivo.
    pub(crate) fn di_governo(&self) -> Result<(), Errore> {
        if self.finita() {
            return Err(Errore::Comando("La partita è finita".into()));
        }
        if self.governa() {
            return Ok(());
        }
        let motivo = match self.legislatura().fase {
            Fase::Sciolta => "non governi: Camere sciolte",
            Fase::Trattativa => "non governi: trattativa in corso",
            Fase::Ordinaria => "non governi: sei all'opposizione",
        };
        Err(Errore::Comando(motivo.into()))
    }

    /// La guardia dei comandi dell'opposizione e della loro anteprima: in fase ordinaria sotto un
    /// governo IA, anche in crisi.
    ///
    /// # Errors
    /// `Comando` a partita finita, quando il giocatore governa, a Camere sciolte e in trattativa.
    pub(crate) fn all_opposizione(&self) -> Result<(), Errore> {
        if self.finita() {
            return Err(Errore::Comando("La partita è finita".into()));
        }
        let motivo = match self.legislatura().fase {
            Fase::Sciolta => "solo all'opposizione: Camere sciolte",
            Fase::Trattativa => "solo all'opposizione: trattativa in corso",
            Fase::Ordinaria if self.governa() => "solo all'opposizione: governi tu",
            Fase::Ordinaria => return Ok(()),
        };
        Err(Errore::Comando(motivo.into()))
    }

    /// La guardia dell'ordinaria amministrazione (scelta 2 del piano 7b): nel turno di crisi niente
    /// leggi, bilancio, decreti, fiducia, `Accogli`, né le anteprime di legge e decreto.
    ///
    /// # Errors
    /// `Comando` nel turno di crisi.
    pub(crate) fn fuori_dalla_crisi(&self) -> Result<(), Errore> {
        if self.in_crisi() {
            return Err(Errore::Comando(
                "Crisi di governo: solo ordinaria amministrazione".into(),
            ));
        }
        Ok(())
    }

    /// Applica un comando del giocatore.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero o della disposizione in causa, `Bilancio` col guasto
    /// della legge di bilancio; la partita resta identica. A partita finita, `Comando`. Ogni
    /// comando tranne `DichiaraCoalizione`, `ProponiSpartizione`, `Allarga`, `ChiediGuida`,
    /// `RifiutaRibaltone`, `Riposiziona`, `Campagna` e quelli dell'opposizione (`Linea`, `Sfiducia`, `PresentaProposta`,
    /// `PromuoviReferendum`, `Confermativo`, con la guardia `all_opposizione`; `Linea` anche a chi
    /// governa, sulle proposte dell'opposizione) è di governo: se il giocatore non
    /// governa, `Comando` col motivo. Nel
    /// turno di crisi `PresentaBilancio`, `PresentaLegge`, `EmanaDecreto`, `PoniFiducia` e
    /// `Accogli`: `Comando`.
    pub fn comanda(&mut self, c: Comando) -> Result<(), Errore> {
        if self.finita() {
            return Err(Errore::Comando("La partita è finita".into()));
        }
        if let Comando::DichiaraCoalizione { partiti, cessioni } = &c {
            return self.dichiara_coalizione(partiti, cessioni);
        }
        if let Comando::ProponiSpartizione { dicasteri } = &c {
            return self.proponi_spartizione(dicasteri);
        }
        if let Comando::Allarga { partiti } = &c {
            return self.allarga(partiti);
        }
        if c == Comando::ChiediGuida {
            return self.chiedi_guida();
        }
        if c == Comando::RifiutaRibaltone {
            return self.rifiuta_ribaltone();
        }
        match &c {
            Comando::Linea { misura, si } => return self.linea(*misura, *si),
            Comando::Sfiducia { dicastero } => return self.presenta_sfiducia(dicastero.as_deref()),
            Comando::PresentaProposta { valori } => return self.presenta_proposta(valori),
            Comando::PromuoviReferendum { disposizione } => {
                self.all_opposizione()?;
                let id = self.partiti()[self.giocatore()].id.clone();
                return self.promuovi_referendum(&id, disposizione);
            }
            Comando::Confermativo { misura, chiedi } => {
                self.all_opposizione()?;
                return self.confermativo(*misura, *chiedi);
            }
            Comando::Riposiziona { posizione } => return self.riposiziona(*posizione),
            Comando::Campagna { spese } => return self.campagna(spese.clone()),
            _ => {}
        }
        self.di_governo()?;
        self.applica(c)
    }

    /// Il corpo dei comandi di governo, senza guardia su chi governa: li usa anche il governo IA.
    /// Nel turno di crisi `PresentaBilancio`, `PresentaLegge`, `EmanaDecreto`, `PoniFiducia` e
    /// `Accogli`: `Comando`. Gli altri comandi non passano da qui.
    ///
    /// # Errors
    /// Come `comanda`, tranne le guardie del giocatore.
    pub(crate) fn applica(&mut self, c: Comando) -> Result<(), Errore> {
        if matches!(
            c,
            Comando::PresentaBilancio { .. }
                | Comando::PresentaLegge { .. }
                | Comando::EmanaDecreto { .. }
                | Comando::PoniFiducia { .. }
                | Comando::Accogli { .. }
        ) {
            self.fuori_dalla_crisi()?;
        }
        match c {
            Comando::PresentaBilancio { bilancio } => return self.presenta_bilancio(&bilancio),
            Comando::PresentaLegge { valori } => return self.presenta_legge(&valori),
            Comando::Emendamento {
                misura,
                partito,
                accogli,
            } => return self.emendamento(misura, &partito, accogli),
            Comando::PoniFiducia { misura } => return self.poni_fiducia(misura),
            Comando::EmanaDecreto { decreto } => return self.emana_decreto(&decreto),
            Comando::InvitaAstensione => return self.invita_astensione(),
            Comando::Dimettiti => {
                self.dimettiti();
                return Ok(());
            }
            Comando::Accogli { ref dicastero } => self.accogli_ripetuto(dicastero)?,
            _ => {}
        }
        let dimissioni = match &c {
            Comando::ChiediDimissioni { dicastero } => Some(dicastero.clone()),
            _ => None,
        };
        match self.con_contesto(|ministri, cx, _| ministri.comanda(cx, c))? {
            EffettoComando::Nessuno => {}
            EffettoComando::Dirottata { dicastero } => self.conversione_su_agenda(dicastero),
            EffettoComando::Decreto { dicastero, importo } => {
                self.aggiungi_decreto(dicastero, importo);
            }
            EffettoComando::Ritiro { dicastero, importo } => {
                self.ritira_decreto(dicastero, importo);
            }
            EffettoComando::Riforma {
                dicastero,
                disposizione,
                gradino,
            } => self.accogli_riforma(dicastero, disposizione, gradino)?,
        }
        if let Some(dicastero) = dimissioni {
            self.dimissioni_chieste(&dicastero);
        }
        Ok(())
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::legislatura::Fondazione;

    /// `applica` non guarda chi governa: l'opposizione che lo chiama con `Dimettiti` mette le
    /// dimissioni al governo IA, mentre `comanda` la respinge.
    #[test]
    fn applica_non_guarda_chi_governa() {
        let f = Fondazione {
            nome: "Partito nuovo".into(),
            posizione: 1.5,
        };
        let mut p = Partita::fonda(dati_base(), 7, &f).unwrap();
        p.chiudi_turno(); // voto
        p.chiudi_turno(); // trattativa
        assert!(!p.governa());
        assert!(p.comanda(Comando::Dimettiti).is_err());
        assert!(!p.legislatura().dimissioni);
        p.applica(Comando::Dimettiti).unwrap();
        assert!(p.legislatura().dimissioni);
    }
}
