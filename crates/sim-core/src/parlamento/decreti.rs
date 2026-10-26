//! Decreti: valgono subito, a fine turno sono convertiti o decadono; materie escluse e niente
//! reiterazione di un decreto decaduto (iter-legislativo-italiano:12-15, 20-23, 67).

use super::{ValoreProposto, ko};
use crate::dati::DatiGioco;
use crate::disposizioni::Norma;
use crate::errore::Errore;
use crate::partita::LIMITE_EURO;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Il contenuto di un decreto. Il bilancio non è fra i contenuti possibili: il divieto vale per
/// costruzione.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Decreto {
    /// Sposta `importo` euro del margine di `da` al dicastero `a` (id).
    Fondi {
        da: String,
        a: String,
        importo: i64,
    },
    Disposizioni {
        valori: Vec<ValoreProposto>,
    },
}

/// I fondi di un decreto in conversione, con indici di dicastero e di atto del catalogo. La firma
/// è dicastero e atto, o i due dicasteri: l'importo non conta.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FondiDecreto {
    Accolti {
        dicastero: usize,
        atto: usize,
        importo: i64,
    },
    Spostati {
        da: usize,
        a: usize,
        importo: i64,
    },
}

impl FondiDecreto {
    pub(crate) fn stessa_firma(&self, altro: &FondiDecreto) -> bool {
        use FondiDecreto::{Accolti, Spostati};
        match (*self, *altro) {
            (
                Accolti {
                    dicastero, atto, ..
                },
                Accolti {
                    dicastero: d,
                    atto: k,
                    ..
                },
            ) => dicastero == d && atto == k,
            (Spostati { da, a, .. }, Spostati { da: x, a: y, .. }) => da == x && a == y,
            _ => false,
        }
    }

    /// Il dicastero che riceve i fondi.
    pub(crate) fn proponente(&self) -> usize {
        match *self {
            FondiDecreto::Accolti { dicastero, .. } => dicastero,
            FondiDecreto::Spostati { a, .. } => a,
        }
    }

    /// `Decreto: fondi a Sanità per …` o `Decreto: fondi da Giustizia a Sanità`.
    pub(crate) fn nome(&self, dati: &DatiGioco) -> String {
        let dicastero = |i: usize| dati.dicasteri().get(i).map_or("", |d| d.nome.as_str());
        match *self {
            FondiDecreto::Accolti {
                dicastero: i, atto, ..
            } => {
                let atto = dati.atti().get(atto).map_or("", |a| a.nome.as_str());
                format!("Decreto: fondi a {} per {atto}", dicastero(i))
            }
            FondiDecreto::Spostati { da, a, .. } => {
                format!("Decreto: fondi da {} a {}", dicastero(da), dicastero(a))
            }
        }
    }

    /// Indici dentro i dati e importo entro 0-1e15, per un salvataggio.
    pub(crate) fn valido(&self, dati: &DatiGioco) -> bool {
        let n = dati.dicasteri().len();
        match *self {
            FondiDecreto::Accolti {
                dicastero,
                atto,
                importo,
            } => dicastero < n && atto < dati.atti().len() && (0..=LIMITE_EURO).contains(&importo),
            FondiDecreto::Spostati { da, a, importo } => {
                da < n && a < n && (0..=LIMITE_EURO).contains(&importo)
            }
        }
    }
}

/// `EmanaDecreto`: rende `(da, a, importo)` dei fondi spostati, con `spostabili` il massimo
/// spostabile da ogni dicastero. Un decreto su disposizioni non ha mai un innesco: non ci sono eventi aperti.
///
/// # Errors
/// `Comando`, nell'ordine: disposizione sconosciuta, materia esclusa (elettorale e
/// costituzionale), decreto senza innesco; dicastero sconosciuto, stesso dicastero, importo non
/// positivo, margine insufficiente, decreto decaduto con la stessa firma.
pub(crate) fn ammetti(
    dati: &DatiGioco,
    decaduti: &[FondiDecreto],
    spostabili: &[i64],
    decreto: &Decreto,
) -> Result<(usize, usize, i64), Errore> {
    let (da, a, importo) = match decreto {
        Decreto::Disposizioni { valori } => {
            for v in valori {
                let Some(k) = dati.disposizione(&v.disposizione) else {
                    return ko(format!("disposizione sconosciuta: {}", v.disposizione));
                };
                let x = &dati.disposizioni()[k];
                if x.norma != Norma::Ordinaria {
                    return ko(format!("materia esclusa dal decreto: {}", x.nome));
                }
            }
            return ko("decreto senza innesco: serve un evento aperto".into());
        }
        Decreto::Fondi { da, a, importo } => (da, a, *importo),
    };
    let indice = |id: &str| {
        (dati.dicasteri().iter().position(|d| d.id == id))
            .ok_or_else(|| Errore::Comando(format!("dicastero sconosciuto: {id}")))
    };
    let (i, j) = (indice(da)?, indice(a)?);
    if i == j {
        return ko(format!("stesso dicastero: {da}"));
    }
    if importo <= 0 {
        return ko(format!("importo non ammesso: {importo}"));
    }
    if spostabili.get(i).is_none_or(|&m| importo > m) {
        return ko(format!("margine insufficiente in {da}"));
    }
    let fondi = FondiDecreto::Spostati {
        da: i,
        a: j,
        importo,
    };
    if decaduti.iter().any(|x| x.stessa_firma(&fondi)) {
        return ko(format!("decreto decaduto non si ripete: {da} → {a}"));
    }
    Ok((i, j, importo))
}
