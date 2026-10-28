//! Comandi del giocatore alla partita.

use crate::economia::Bilancio;
use crate::errore::Errore;
use crate::legislatura::{Assegnazione, Cessione, Fase};
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

    /// Applica un comando del giocatore.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero o della disposizione in causa, `Bilancio` col guasto
    /// della legge di bilancio; la partita resta identica. A partita finita, `Comando`. Ogni
    /// comando tranne `DichiaraCoalizione`, `ProponiSpartizione` e `Allarga` è di governo: se il
    /// giocatore non governa, `Comando` col motivo.
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
        self.di_governo()?;
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
