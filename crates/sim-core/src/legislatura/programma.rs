//! Il programma pubblico del governo IA (scelta 7 del piano 8a), calcolato alla sua nascita.

use crate::disposizioni::Norma;
use crate::parlamento::StatoAula;

/// Posizione della coalizione: la media delle posizioni dichiarate pesata coi `seggi` di Camera
/// più Senato. Per ogni disposizione ordinaria con posizione, il gradino non bloccato più vicino a
/// quella posizione, se avvicina il vigente di almeno `soglia_programma`; in ordine di distanza
/// decrescente del vigente. In coda, al più una voce elettorale (`elettorale`). Le costituzionali
/// restano fuori. Vuoto se la coalizione non ha seggi.
// ponytail: «non bloccato» = fuori da `bloccati` del referendum; le respinte da poco e le misure in
// esame dipendono dal calendario del governo uscente e non entrano nel programma.
pub(crate) fn programma(
    aula: &StatoAula,
    coalizione: &[usize],
    seggi: &[u32],
) -> Vec<(usize, usize)> {
    let peso = |j: usize| f64::from(seggi.get(j).copied().unwrap_or(0));
    let totale: f64 = coalizione.iter().map(|&j| peso(j)).sum();
    if totale <= 0.0 {
        return Vec::new();
    }
    let pos = (coalizione.iter())
        .filter_map(|&j| Some(peso(j) * aula.partiti.get(j)?.posizione_dichiarata))
        .sum::<f64>()
        / totale;
    let dati = aula.dati;
    let soglia = dati.manopola("soglia_programma");
    let mut voci: Vec<(f64, (usize, usize))> = Vec::new();
    for (d, dato) in dati.disposizioni().iter().enumerate() {
        let stato = &aula.disposizioni.voci[d];
        let Some(vigente) = dato.posizione(stato.in_vigore) else {
            continue;
        };
        if dato.norma != Norma::Ordinaria {
            continue;
        }
        let distanza = (vigente - pos).abs();
        let migliore = (0..dato.gradini.len())
            .filter(|g| !stato.bloccati.contains(g))
            .filter_map(|g| Some((g, (dato.posizione(g)? - pos).abs())))
            .min_by(|a, b| a.1.total_cmp(&b.1));
        if let Some((g, _)) = migliore.filter(|&(_, nuova)| distanza - nuova >= soglia) {
            voci.push((distanza, (d, g)));
        }
    }
    voci.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut programma: Vec<(usize, usize)> = voci.into_iter().map(|v| v.1).collect();
    let minimo = dati.manopola("seggi_programma_elettorale") as i32;
    programma.extend(elettorale(aula, coalizione, minimo));
    programma
}

/// La disposizione elettorale e il gradino non bloccato che rendono più seggi attesi (Camera più
/// Senato) alla coalizione, se almeno `minimo`.
fn elettorale(aula: &StatoAula, coalizione: &[usize], minimo: i32) -> Option<(usize, usize)> {
    let mut migliore: Option<((usize, usize), i32)> = None;
    for (d, dato) in aula.dati.disposizioni().iter().enumerate() {
        if dato.norma != Norma::Elettorale {
            continue;
        }
        let stato = &aula.disposizioni.voci[d];
        for g in (0..dato.gradini.len()).filter(|&g| g != stato.in_vigore) {
            if stato.bloccati.contains(&g) {
                continue;
            }
            let n: i32 = (aula.seggi_attesi(&[(d, g)]).iter().enumerate())
                .filter(|(j, _)| coalizione.contains(j))
                .map(|(_, &(c, s))| c + s)
                .sum();
            if n >= minimo && migliore.is_none_or(|m| n > m.1) {
                migliore = Some(((d, g), n));
            }
        }
    }
    migliore.map(|m| m.0)
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::partita::Partita;

    /// Il gradino migliore dà `n` seggi attesi alla coalizione: entra con soglia `n`, non con
    /// `n + 1`.
    #[test]
    fn l_elettorale_entra_solo_oltre_n_seggi() {
        let p = Partita::nuova(dati_base(), 1);
        let aula = p.stato_aula();
        let coalizione = &p.governo().coalizione;
        let voce = elettorale(&aula, coalizione, i32::MIN).unwrap();
        let n: i32 = (aula.seggi_attesi(&[voce]).iter().enumerate())
            .filter(|(j, _)| coalizione.contains(j))
            .map(|(_, &(c, s))| c + s)
            .sum();
        assert_eq!(elettorale(&aula, coalizione, n), Some(voce));
        assert_eq!(elettorale(&aula, coalizione, n + 1), None);
    }
}
