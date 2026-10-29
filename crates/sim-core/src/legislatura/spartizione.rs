//! La spartizione dei dicasteri dopo il voto: richiesta di Gamson e spartizione di default
//! (legge di Gamson, formazione-di-coalizioni:16; nessun premio al formateur oltre la presidenza,
//! :20-22; intercetta e pendenza, :29-30).

use crate::dati::DatiGioco;
use crate::partiti::Partito;
#[cfg(test)]
use crate::partiti::{Regola, Vocazione};
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
/// partito per dicastero, nell'ordine dei dati; `seggi` per partito, Camera più Senato. I
/// dicasteri `Some` di `tenuti` restano a chi li tiene, al netto della sua richiesta (rimpasto,
/// Gamson ricalcolata, formazione-di-coalizioni:16); si assegnano solo i `None`.
pub(crate) fn spartizione_di_default(
    dati: &DatiGioco,
    coalizione: &[usize],
    guida: usize,
    seggi: &[u32],
    tenuti: &[Option<usize>],
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
    for (i, &j) in tenuti.iter().enumerate() {
        let Some(j) = j else { continue };
        if let Some(x) = v.get_mut(i) {
            *x = j;
            if let Some(s) = scoperte.iter_mut().find(|s| s.0 == j) {
                s.1 -= f64::from(d[i].peso_spartizione);
            }
        }
    }
    for i in ordine
        .into_iter()
        .filter(|&i| tenuti.get(i).is_none_or(Option::is_none))
    {
        let k =
            (0..scoperte.len()).fold(0, |b, k| if scoperte[k].1 > scoperte[b].1 { k } else { b });
        v[i] = scoperte[k].0;
        scoperte[k].1 -= f64::from(d[i].peso_spartizione);
    }
    v
}

/// Il primo della `coalizione` per seggi (Camera più Senato), a parità l'indice più basso.
pub(crate) fn primo_per_seggi(
    coalizione: &[usize],
    seggi_c: &[u32],
    seggi_s: &[u32],
) -> Option<usize> {
    let di = |v: &[u32], j: usize| u64::from(v.get(j).copied().unwrap_or(0));
    (coalizione.iter().copied())
        .max_by_key(|&j| (di(seggi_c, j) + di(seggi_s, j), std::cmp::Reverse(j)))
}

/// La `coalizione` ha più della metà dei seggi in entrambe le Camere.
pub(crate) fn maggioranza(seggi_c: &[u32], seggi_s: &[u32], coalizione: &[usize]) -> bool {
    let oltre_meta = |seggi: &[u32]| {
        let s: u64 = (coalizione.iter().filter_map(|&j| seggi.get(j)))
            .map(|&x| u64::from(x))
            .sum();
        2 * s > seggi.iter().map(|&x| u64::from(x)).sum::<u64>()
    };
    oltre_meta(seggi_c) && oltre_meta(seggi_s)
}

/// La maggioranza minima connessa vincente (formazione-di-coalizioni:13): fra le finestre di
/// partiti consecutivi per posizione dichiarata (a parità l'indice), senza quelli privi di seggi
/// in entrambe le Camere e senza `escluso` (la finestra può scavalcarlo), quelle con la
/// maggioranza; la più piccola per numero di partiti, poi la più grande per seggi (Camera più
/// Senato), poi la più a sinistra. I partiti nell'ordine della fila.
pub(crate) fn maggioranza_contigua(
    partiti: &[Partito],
    seggi_c: &[u32],
    seggi_s: &[u32],
    escluso: Option<usize>,
) -> Option<Vec<usize>> {
    let di = |v: &[u32], j: usize| v.get(j).copied().unwrap_or(0);
    let seggi = |j: usize| u64::from(di(seggi_c, j)) + u64::from(di(seggi_s, j));
    let mut fila: Vec<usize> = (0..partiti.len())
        .filter(|&j| seggi(j) > 0 && Some(j) != escluso)
        .collect();
    fila.sort_by(|&a, &b| {
        (partiti[a].posizione_dichiarata)
            .total_cmp(&partiti[b].posizione_dichiarata)
            .then(a.cmp(&b))
    });
    // ponytail: O(P³) su sette partiti; il 7b la riusa così com'è.
    (1..=fila.len()).find_map(|n| {
        (fila.windows(n))
            .filter(|w| maggioranza(seggi_c, seggi_s, w))
            .fold(None::<&[usize]>, |meglio, w| {
                let tot = |w: &[usize]| w.iter().map(|&j| seggi(j)).sum::<u64>();
                match meglio {
                    Some(m) if tot(m) >= tot(w) => Some(m),
                    _ => Some(w),
                }
            })
            .map(<[usize]>::to_vec)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    fn sette(posizioni: [f64; 7]) -> Vec<Partito> {
        (posizioni.iter().enumerate())
            .map(|(j, &x)| Partito {
                id: format!("p{j}"),
                nome: format!("P{j}"),
                posizione_dichiarata: x,
                posizione_percepita: x,
                valence: 0.0,
                valence_base: 0.0,
                vocazione: Vocazione::Mista,
                leader: String::new(),
                regola: Regola::Sticker,
                sciolto: false,
                cassa: 0,
                senza_seggi: 0,
                ultimo_passo: 0.0,
            })
            .collect()
    }

    /// Camera 400, Senato 200: maggioranza oltre 200 e oltre 100.
    #[test]
    fn maggioranza_contigua_minima() {
        let in_fila = sette([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        let m = |p: &[Partito], c: [u32; 7], s: [u32; 7], escluso| {
            maggioranza_contigua(p, &c, &s, escluso)
        };
        // Due partiti (1, 2) battono i tre (0, 1, 2); (0, 1) è alla metà esatta.
        let (c, s) = ([50, 150, 100, 30, 20, 30, 20], [25, 75, 50, 15, 10, 15, 10]);
        assert_eq!(m(&in_fila, c, s, None), Some(vec![1, 2]));
        // Ogni finestra con la maggioranza contiene il partito 3.
        let (c, s) = ([10, 10, 10, 300, 10, 50, 10], [5, 5, 5, 150, 5, 25, 5]);
        assert_eq!(m(&in_fila, c, s, None), Some(vec![3]));
        assert_eq!(m(&in_fila, c, s, Some(3)), None);
        // Con 3 al centro escluso, la finestra (2, 4, 5) lo scavalca; con lui vince (2, 3, 4).
        let (c, s) = ([20, 20, 80, 100, 80, 80, 20], [10, 10, 40, 50, 40, 40, 10]);
        assert_eq!(m(&in_fila, c, s, Some(3)), Some(vec![2, 4, 5]));
        assert_eq!(m(&in_fila, c, s, None), Some(vec![2, 3, 4]));
        // A parità di partiti vince (1, 2), con più seggi, anche se (0, 1) è più a sinistra.
        let (c, s) = ([95, 110, 100, 10, 10, 40, 35], [50, 55, 50, 5, 5, 20, 15]);
        assert_eq!(m(&in_fila, c, s, None), Some(vec![1, 2]));
        // A parità anche di seggi vince la più a sinistra.
        let (c, s) = ([100, 105, 100, 10, 10, 40, 35], [50, 55, 50, 5, 5, 20, 15]);
        assert_eq!(m(&in_fila, c, s, None), Some(vec![0, 1]));
        // Contigui per posizione, non per indice: 0 e 2 sono vicini.
        let scambiati = sette([1.0, 3.0, 2.0, 4.0, 5.0, 6.0, 7.0]);
        let (c, s) = ([110, 10, 100, 60, 40, 40, 40], [55, 5, 50, 30, 20, 20, 20]);
        assert_eq!(m(&scambiati, c, s, None), Some(vec![0, 2]));
        // Un partito senza seggi non sta in parlamento: non interrompe la fila né la allunga.
        let (c, s) = ([150, 0, 60, 50, 50, 50, 40], [75, 0, 30, 25, 25, 25, 20]);
        assert_eq!(m(&in_fila, c, s, None), Some(vec![0, 2]));
    }

    /// Pesi dei dati: Economia 3, Interno ed Esteri 2, gli altri 1 (14). Seggi 100, 50, 150:
    /// richieste 4,67 e 2,82, alla guida il resto, 6,51. Seggi 50, 50, 200: due richieste
    /// uguali, 2,82, e la parità va all'indice più basso.
    #[test]
    fn la_spartizione_di_default_copre_le_richieste_in_ordine_di_peso() {
        let v = spartizione_di_default(&dati_base(), &[0, 1, 2], 2, &[100, 50, 150], &[None; 10]);
        assert_eq!(v, [2, 1, 0, 1, 0, 2, 1, 0, 0, 2]);
        let v = spartizione_di_default(&dati_base(), &[0, 1, 2], 2, &[50, 50, 200], &[None; 10]);
        assert_eq!(v, [2, 0, 1, 0, 1, 2, 0, 2, 1, 2]);
    }

    /// Un `tenuti` più lungo dei dicasteri: l'eccesso si ignora, senza panico.
    #[test]
    fn tenuti_oltre_i_dicasteri_si_ignorano() {
        let v = spartizione_di_default(&dati_base(), &[0, 1], 1, &[100, 100], &[Some(0); 12]);
        assert_eq!(v, [0; 10]);
    }

    /// Seggi 100, 50, 150 come sopra, col partito 1 che tiene Esteri (9, peso 2): gli resta, e
    /// la sua richiesta scoperta scende da 2,82 a 0,82. In più prende solo Ambiente (6), dove le
    /// scoperte sono 0,67, 0,82 e 0,51; il suo peso totale resta 3, come senza il tenuto.
    #[test]
    fn il_tenuto_resta_e_sconta_la_richiesta() {
        let mut tenuti = [None; 10];
        tenuti[9] = Some(1);
        let v = spartizione_di_default(&dati_base(), &[0, 1, 2], 2, &[100, 50, 150], &tenuti);
        assert_eq!(v, [2, 2, 0, 2, 0, 2, 1, 0, 0, 1]);
    }
}
