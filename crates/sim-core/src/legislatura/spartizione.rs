//! La spartizione dei dicasteri dopo il voto: richiesta di Gamson e spartizione di default
//! (legge di Gamson, formazione-di-coalizioni:16; nessun premio al formateur oltre la presidenza,
//! :20-22; intercetta e pendenza, :29-30).

use crate::dati::DatiGioco;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Un dicastero (id) a un partito (id) della coalizione; `None` = un tecnico.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Assegnazione {
    pub dicastero: String,
    pub partito: Option<String>,
}

/// Un alleato (id) in trattativa: la sua richiesta e l'offerta della spartizione proposta, in
/// punti di peso dei dicasteri.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceRichiesta {
    pub partito: String,
    pub richiesta: f64,
    pub offerta: f64,
}

/// La richiesta di un partito con `seggi_j` seggi (Camera più Senato) in una maggioranza di
/// `seggi_maggioranza`: `(gamson_intercetta + gamson_pendenza · seggi_j / seggi_maggioranza) ·
/// Σ peso_spartizione`; 0 se la maggioranza non ha seggi.
pub(crate) fn richiesta(dati: &DatiGioco, seggi_j: u32, seggi_maggioranza: u32) -> f64 {
    if seggi_maggioranza == 0 {
        return 0.0;
    }
    let quota = f64::from(seggi_j) / f64::from(seggi_maggioranza);
    (dati.manopola("gamson_intercetta") + dati.manopola("gamson_pendenza") * quota) * pesi(dati)
}

/// La somma dei pesi di spartizione dei dicasteri.
fn pesi(dati: &DatiGioco) -> f64 {
    (dati.dicasteri().iter())
        .map(|d| f64::from(d.peso_spartizione))
        .sum()
}

/// La spartizione di default (scelta 6 del piano 7): i dicasteri in ordine di peso decrescente
/// (a parità quello dei dati), ciascuno al partito della `coalizione` con la richiesta scoperta
/// più grande (a parità l'indice più basso); `guida` ha come richiesta il resto dei pesi. Un
/// partito per dicastero, nell'ordine dei dati; `seggi` per partito, Camera più Senato.
pub(crate) fn spartizione_di_default(
    dati: &DatiGioco,
    coalizione: &[usize],
    guida: usize,
    seggi: &[u32],
) -> Vec<usize> {
    let di = |j: usize| seggi.get(j).copied().unwrap_or(0);
    let s: u32 = coalizione.iter().map(|&j| di(j)).sum();
    let mut scoperte: Vec<(usize, f64)> = (coalizione.iter())
        .filter(|&&j| j != guida)
        .map(|&j| (j, richiesta(dati, di(j), s)))
        .collect();
    let resto = pesi(dati) - scoperte.iter().map(|x| x.1).sum::<f64>();
    scoperte.push((guida, resto));
    scoperte.sort_by_key(|x| x.0);
    let d = dati.dicasteri();
    let mut ordine: Vec<usize> = (0..d.len()).collect();
    ordine.sort_by_key(|&i| std::cmp::Reverse(d[i].peso_spartizione));
    let mut v = vec![guida; d.len()];
    for i in ordine {
        let k =
            (0..scoperte.len()).fold(0, |b, k| if scoperte[k].1 > scoperte[b].1 { k } else { b });
        v[i] = scoperte[k].0;
        scoperte[k].1 -= f64::from(d[i].peso_spartizione);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    /// Pesi dei dati: Economia 3, Interno ed Esteri 2, gli altri 1 (14). Seggi 100, 50, 150:
    /// richieste 4,67 e 2,82, alla guida il resto, 6,51. Seggi 50, 50, 200: due richieste
    /// uguali, 2,82, e la parità va all'indice più basso.
    #[test]
    fn la_spartizione_di_default_copre_le_richieste_in_ordine_di_peso() {
        let v = spartizione_di_default(&dati_base(), &[0, 1, 2], 2, &[100, 50, 150]);
        assert_eq!(v, [2, 1, 0, 1, 0, 2, 1, 0, 0, 2]);
        let v = spartizione_di_default(&dati_base(), &[0, 1, 2], 2, &[50, 50, 200]);
        assert_eq!(v, [2, 0, 1, 0, 1, 2, 0, 2, 1, 2]);
    }
}
