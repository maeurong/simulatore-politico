//! Il voto: voti per regione dalle intenzioni vere, esito e incarico (premio e riparto di Camera e
//! Senato, sistemi-elettorali:39-46).

use crate::dati::DatiGioco;
use crate::elettorato::{Elettorato, Intenzioni, PER_REGIONE};
use crate::partita::Trimestre;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(test)]
use ts_rs::TS;

/// Il risultato di un partito (id): `quota` fra i votanti, seggi dopo le cessioni.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VotiPartito {
    pub partito: String,
    pub quota: f64,
    pub camera: u32,
    pub senato: u32,
}

/// L'esito di un voto: `affluenza` fra gli aventi diritto, una voce per partito, `premio` = gli
/// id dei membri di chi l'ha preso uniti da `+`, `coalizioni` = quelle al voto (id).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct EsitoElezioni {
    pub data: Trimestre,
    pub affluenza: f64,
    pub voti: Vec<VotiPartito>,
    pub premio: Option<String>,
    pub coalizioni: Vec<Vec<String>>,
}

/// A chi va l'incarico dopo il voto: la coalizione con la maggioranza assoluta in entrambe le
/// Camere, del giocatore o IA (indici di partito).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Incarico {
    Giocatore,
    Ia { coalizione: Vec<usize> },
}

/// Per regione e partito `Σ_gruppi della regione round(elettori_i · v_ij)`, senza l'astensione.
pub(crate) fn voti_per_regione(
    dati: &DatiGioco,
    el: &Elettorato,
    v: &Intenzioni,
) -> BTreeMap<String, Vec<u64>> {
    let d = dati.elettorato();
    (d.regioni.iter().enumerate())
        .map(|(r, regione)| {
            let n = v.nazionali.len().saturating_sub(1);
            let mut voti = vec![0u64; n];
            for i in r * PER_REGIONE..(r + 1) * PER_REGIONE {
                let w = el.elettori(d, i);
                let quote = v.per_gruppo.get(i).map_or(&[][..], Vec::as_slice);
                for (x, q) in voti.iter_mut().zip(quote) {
                    *x += (w * q).round() as u64;
                }
            }
            (regione.id.clone(), voti)
        })
        .collect()
}

/// La prima di `coalizioni` (la prima è del giocatore) con più della metà dei seggi in entrambe le
/// Camere; i partiti fuori da ogni coalizione contano da soli.
pub(crate) fn incarico(
    coalizioni: &[Vec<usize>],
    giocatore: usize,
    camera: &[u32],
    senato: &[u32],
) -> Option<Incarico> {
    let soli = (0..camera.len())
        .filter(|j| !coalizioni.iter().flatten().any(|i| i == j))
        .map(|j| vec![j]);
    let maggioranza = |c: &[usize], seggi: &[u32]| {
        let s: u64 = (c.iter().filter_map(|&j| seggi.get(j)))
            .map(|&x| u64::from(x))
            .sum();
        2 * s > seggi.iter().map(|&x| u64::from(x)).sum::<u64>()
    };
    (coalizioni.iter().cloned().chain(soli))
        .find(|c| maggioranza(c, camera) && maggioranza(c, senato))
        .map(|c| {
            if c.contains(&giocatore) {
                Incarico::Giocatore
            } else {
                Incarico::Ia { coalizione: c }
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn incarico_a_chi_ha_la_maggioranza_in_entrambe_le_camere() {
        let c = [vec![0, 1], vec![2, 3]];
        let giocatore = Some(Incarico::Giocatore);
        assert_eq!(
            incarico(&c, 0, &[3, 3, 2, 1, 0], &[3, 2, 2, 2, 0]),
            giocatore
        );
        let ia = Some(Incarico::Ia {
            coalizione: vec![2, 3],
        });
        assert_eq!(incarico(&c, 0, &[1, 1, 3, 3, 0], &[1, 1, 4, 2, 0]), ia);
        // Metà esatta in un ramo: nessuna maggioranza assoluta.
        assert_eq!(incarico(&c, 0, &[3, 3, 2, 1, 0], &[2, 2, 2, 2, 0]), None);
        // Un partito fuori dalle coalizioni conta da solo.
        let solo = Some(Incarico::Ia {
            coalizione: vec![4],
        });
        assert_eq!(incarico(&c, 0, &[1, 0, 1, 0, 3], &[0, 1, 0, 1, 3]), solo);
    }
}
