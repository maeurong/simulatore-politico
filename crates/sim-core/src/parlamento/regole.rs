//! Le regole della legge elettorale (seggi attesi) e della norma costituzionale (art. 138:
//! due terzi, un quinto, referendum confermativo), iter-legislativo-italiano:48-61, 76-80.

use super::{Conteggio, StatoAula};
use crate::dati::DatiGioco;
use crate::disposizioni::{Disposizioni, Leve};
use crate::elettorato::Affluenza;
use crate::elezioni::{Esito, LeggeElettorale, SeggiLista, Voti, riparto};
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::partita::Trimestre;
use crate::partiti::{Governo, Partito};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum TipoReferendum {
    Confermativo,
    Abrogativo,
}

/// Un referendum in attesa del voto, su una legge intera: `valori` = disposizione e gradino
/// (indici), `promotore` un partito, `voto` il turno nella cui chiusura si vota.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Referendum {
    pub tipo: TipoReferendum,
    pub misura: Option<u32>,
    pub valori: Vec<(usize, usize)>,
    pub promotore: Option<usize>,
    /// Il confermativo è di una legge del giocatore.
    pub del_giocatore: bool,
    pub voto: u32,
    pub firme: bool,
    pub invito_astensione: bool,
}

/// Elettori arrotondati; `quorum` = più della metà degli aventi diritto ha votato. `decaduto` =
/// il motivo per cui il referendum non si è votato (decaduto o sospeso), leggibile per il
/// giocatore.
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
    pub decaduto: Option<String>,
}

impl EsitoReferendum {
    /// L'esito dall'affluenza `a`: sì e no arrotondati, quorum, approvato con `sì > no`.
    pub(crate) fn da(tipo: TipoReferendum, nome: String, data: Trimestre, a: &Affluenza) -> Self {
        let (si, no) = (a.si.round() as u64, a.no.round() as u64);
        let aventi_diritto = (a.si + a.no + a.astenuti).round() as u64;
        let votanti = si.saturating_add(no);
        EsitoReferendum {
            tipo,
            nome,
            data,
            si,
            no,
            votanti,
            aventi_diritto,
            quorum: votanti.saturating_mul(2) > aventi_diritto,
            approvato: si > no,
            decaduto: None,
        }
    }
}

/// Riparto sul sondaggio pubblico con la legge `legge` e i seggi dei rami da `leve`: voti
/// regionali dalle quote degli incroci, `coalizioni` (indici di partito) da chi chiama, partiti
/// senza coalizione da soli.
///
/// # Errors
/// Quelli di `elezioni::riparto`.
pub(crate) fn proietta(
    dati: &DatiGioco,
    partiti: &[Partito],
    sondaggio: &Sondaggio,
    legge: &LeggeElettorale,
    leve: &Leve,
    coalizioni: &[Vec<usize>],
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
    riparto_con(dati, legge, leve, coalizioni, per_regione)
}

/// `elezioni::riparto` dei voti `per_regione` con la legge `legge`, i seggi dei rami da `leve` e
/// le `coalizioni` (indici di partito).
///
/// # Errors
/// Quelli di `elezioni::riparto`.
pub(crate) fn riparto_con(
    dati: &DatiGioco,
    legge: &LeggeElettorale,
    leve: &Leve,
    coalizioni: &[Vec<usize>],
    per_regione: BTreeMap<String, Vec<u64>>,
) -> Result<Esito, Errore> {
    // I totali dei rami da `numero_parlamentari`: `seggi_per_circoscrizione` regge ogni totale.
    let mut elezioni = dati.elezioni().clone();
    elezioni.camera.seggi = leve.seggi_camera.round() as u32;
    elezioni.senato.seggi = leve.seggi_senato.round() as u32;
    riparto(
        &elezioni,
        legge,
        &Voti {
            coalizioni: coalizioni.to_vec(),
            per_regione,
        },
    )
}

/// Perché la legge elettorale in vigore con `valori` non sarebbe valida: la stringa dentro
/// `Errore::LeggeElettorale`; `None` se è valida.
pub(crate) fn legge_non_valida(
    dati: &DatiGioco,
    disposizioni: &Disposizioni,
    valori: &[(usize, usize)],
) -> Option<String> {
    let e = (disposizioni.legge_con(dati, valori))
        .valida(&dati.elezioni().menu)
        .err()?;
    Some(match e {
        Errore::LeggeElettorale(m) => m,
        altro => altro.to_string(),
    })
}

/// Almeno due terzi dei `seggi` del ramo votano sì: `sì · 3 ≥ seggi · 2`.
pub(crate) fn due_terzi(c: &Conteggio, seggi: &[u32]) -> bool {
    c.si * 3 >= seggi.iter().sum::<u32>() * 2
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
        let proietta = |l: &LeggeElettorale| {
            proietta(
                dati,
                self.partiti,
                self.sondaggio,
                l,
                &leve,
                &self.coalizioni,
            )
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Un elettorato manomesso oltre `u64::MAX / 2` non fa traboccare votanti e quorum.
    #[test]
    fn esito_con_elettorato_enorme_non_trabocca() {
        let a = Affluenza {
            si: 1e19,
            no: 1e19,
            astenuti: 1e19,
        };
        let e = EsitoReferendum::da(
            TipoReferendum::Abrogativo,
            String::new(),
            Trimestre::TURNO_ZERO,
            &a,
        );
        assert_eq!(e.votanti, u64::MAX);
    }
}
