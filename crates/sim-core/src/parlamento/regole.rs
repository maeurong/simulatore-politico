//! Le regole della legge elettorale (seggi attesi) e della norma costituzionale (art. 138:
//! due terzi, un quinto, referendum confermativo), iter-legislativo-italiano:48-61, 76-80.

use super::{Conteggio, StatoAula};
use crate::dati::DatiGioco;
use crate::disposizioni::Leve;
use crate::elezioni::{Esito, LeggeElettorale, SeggiLista, Voti, riparto};
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::partita::Trimestre;
use crate::partiti::{Governo, Partito};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum TipoReferendum {
    Confermativo,
    Abrogativo,
}

/// Un referendum in attesa del voto, su un valore: `disposizione` e `gradino` sono indici,
/// `promotore` un partito, `voto` il turno nella cui chiusura si vota.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Referendum {
    pub tipo: TipoReferendum,
    pub misura: Option<u32>,
    pub disposizione: usize,
    pub gradino: usize,
    pub promotore: Option<usize>,
    pub voto: u32,
    pub firme: bool,
    pub invito_astensione: bool,
}

/// Elettori arrotondati; `quorum` = più della metà degli aventi diritto ha votato.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct EsitoReferendum {
    pub tipo: TipoReferendum,
    pub nome: String,
    pub data: Trimestre,
    pub si: u64,
    pub no: u64,
    pub votanti: u64,
    pub aventi_diritto: u64,
    pub quorum: bool,
    pub approvato: bool,
}

/// Riparto sul sondaggio pubblico con la legge `legge` e i seggi dei rami da `leve`: voti
/// regionali dalle quote degli incroci, coalizioni dell'ultimo voto, partiti senza coalizione
/// da soli.
///
/// # Errors
/// Quelli di `elezioni::riparto`.
pub(crate) fn proietta(
    dati: &DatiGioco,
    partiti: &[Partito],
    sondaggio: &Sondaggio,
    legge: &LeggeElettorale,
    leve: &Leve,
) -> Result<Esito, Errore> {
    let n = partiti.len();
    let per_regione = (dati.elettorato().regioni.iter())
        .zip(&sondaggio.regioni)
        .map(|(r, inc)| {
            let voti = inc.quote.iter().take(n);
            (
                r.id.clone(),
                voti.map(|q| (r.elettori as f64 * q).round() as u64)
                    .collect(),
            )
        })
        .collect();
    let posizione = |id: &String| partiti.iter().position(|p| &p.id == id);
    let coalizioni = (dati.scenario().coalizioni.iter())
        .map(|c| c.iter().filter_map(posizione).collect())
        .collect();
    // I totali dei rami da `numero_parlamentari`: `seggi_per_circoscrizione` regge ogni totale.
    let mut elezioni = dati.elezioni().clone();
    elezioni.camera.seggi = leve.seggi_camera.round() as u32;
    elezioni.senato.seggi = leve.seggi_senato.round() as u32;
    riparto(
        &elezioni,
        legge,
        &Voti {
            coalizioni,
            per_regione,
        },
    )
}

/// Almeno due terzi di sì nel ramo: `sì · 3 ≥ seggi · 2`.
pub(crate) fn due_terzi(c: &Conteggio) -> bool {
    c.si * 3 >= (c.si + c.no) * 2
}

/// I partiti fuori dal governo con linea no hanno almeno un quinto dei `seggi` del ramo.
pub(crate) fn quinto_contro(governo: &Governo, seggi: &[u32], linea: &[bool]) -> bool {
    let contro: u32 = (seggi.iter().zip(linea).enumerate())
        .filter(|&(j, (_, &si))| !si && !governo.al_governo(j))
        .map(|(_, (&n, _))| n)
        .sum();
    contro * 5 >= seggi.iter().sum::<u32>()
}

impl StatoAula<'_> {
    /// Seggi di Camera e Senato per partito con la legge in vigore più `valori`, meno quelli con
    /// la legge in vigore; zero se il riparto rifiuta i voti.
    pub(crate) fn seggi_attesi(&self, valori: &[(usize, usize)]) -> Vec<(i32, i32)> {
        let (dati, d) = (self.dati, self.disposizioni);
        let leve = d.leve(dati);
        let proietta = |l: &LeggeElettorale| proietta(dati, self.partiti, self.sondaggio, l, &leve);
        let (Ok(prima), Ok(dopo)) = (
            proietta(&d.legge_elettorale(dati)),
            proietta(&d.legge_con(dati, valori)),
        ) else {
            return vec![(0, 0); self.partiti.len()];
        };
        let s = |x: &[SeggiLista], j: usize| x.get(j).map_or(0, |s| s.totale() as i32);
        (0..self.partiti.len())
            .map(|j| {
                (
                    s(&dopo.camera, j) - s(&prima.camera, j),
                    s(&dopo.senato, j) - s(&prima.senato, j),
                )
            })
            .collect()
    }
}
