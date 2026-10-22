//! Seggi per circoscrizione dalla popolazione censita (art. 56-57 Cost.).

use super::Ramo;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeggiCircoscrizione {
    pub id: String,
    pub regione: String,
    pub seggi: u32,
    pub premio: u32,
    pub uninominali: u32,
}

/// Quota in punti base di `seggi`, arrotondata all'intero più vicino (metà in su).
#[must_use]
pub fn seggi_da_quota(quota_pb: u16, seggi: u32) -> u32 {
    let n = (u64::from(quota_pb) * u64::from(seggi) + 5_000) / 10_000;
    // ≤ seggi per costruzione con `quota_pb` ≤ 10000, come vuole il menu.
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// Hare sulla popolazione: quoziente naturale esatto `p*S/P`, seggi rimasti ai resti più alti;
/// a parità di resto la popolazione più alta, poi l'ordine. Senza popolazione: tutti a 0.
fn hare(pop: &[u64], seggi: u32) -> Vec<u32> {
    let tot: u128 = pop.iter().map(|&p| u128::from(p)).sum();
    if tot == 0 {
        return vec![0; pop.len()];
    }
    let s = u128::from(seggi);
    let mut parti: Vec<u32> = pop
        .iter()
        // ≤ seggi per costruzione: `p ≤ tot`.
        .map(|&p| u32::try_from(u128::from(p) * s / tot).unwrap_or(u32::MAX))
        .collect();
    let mut ordine: Vec<usize> = (0..pop.len()).collect();
    ordine.sort_by_key(|&i| {
        (
            std::cmp::Reverse(u128::from(pop[i]) * s % tot),
            std::cmp::Reverse(pop[i]),
        )
    });
    let resto = seggi.saturating_sub(parti.iter().sum());
    for &i in ordine.iter().take(resto as usize) {
        parti[i] += 1;
    }
    parti
}

/// Seggi (e quota di premio) di ogni circoscrizione, nell'ordine di `ramo.circoscrizioni`.
///
/// Passi: fissi e estero tolti; il resto per Hare sulla popolazione; chi resta sotto `minimo`
/// è fissato a `minimo` e si rifà il riparto sulle altre; il `premio` si divide con Hare fra le
/// sole circoscrizioni ordinarie (`uninominali == 0` nei dati).
#[must_use]
pub fn seggi_per_circoscrizione(ramo: &Ramo, premio: u32) -> Vec<SeggiCircoscrizione> {
    let cs = &ramo.circoscrizioni;
    let mut seggi: Vec<Option<u32>> = cs.iter().map(|c| c.seggi_fissi).collect();
    loop {
        let liberi: Vec<usize> = (0..cs.len()).filter(|&i| seggi[i].is_none()).collect();
        let da_dare = ramo
            .seggi
            .saturating_sub(ramo.estero)
            .saturating_sub(seggi.iter().flatten().sum());
        let pop: Vec<u64> = liberi.iter().map(|&i| cs[i].popolazione).collect();
        let parti = hare(&pop, da_dare);
        let sotto: Vec<usize> = liberi
            .iter()
            .zip(&parti)
            .filter(|&(_, &p)| p < ramo.minimo)
            .map(|(&i, _)| i)
            .collect();
        if sotto.is_empty() {
            for (&i, &p) in liberi.iter().zip(&parti) {
                seggi[i] = Some(p);
            }
            break;
        }
        for i in sotto {
            seggi[i] = Some(ramo.minimo);
        }
    }
    let ordinarie: Vec<usize> = (0..cs.len()).filter(|&i| cs[i].uninominali == 0).collect();
    let pop: Vec<u64> = ordinarie.iter().map(|&i| cs[i].popolazione).collect();
    let mut premi = vec![0; cs.len()];
    for (&i, p) in ordinarie.iter().zip(hare(&pop, premio)) {
        premi[i] = p;
    }
    cs.iter()
        .zip(seggi.iter().zip(premi))
        .map(|(c, (s, premio))| {
            let seggi = s.unwrap_or(0);
            SeggiCircoscrizione {
                id: c.id.clone(),
                regione: c.regione.clone(),
                seggi,
                premio,
                uninominali: c.uninominali.min(seggi),
            }
        })
        .collect()
}
