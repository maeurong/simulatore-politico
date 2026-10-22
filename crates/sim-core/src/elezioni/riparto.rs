//! Riparto dei seggi della legge 180/2026, parte proporzionale: soglie, metodo, Camera
//! nazionale (art. 83 DPR 361) e Senato regione per regione (art. 16-bis d.lgs. 533).
//! Puro: stessi argomenti, stesso esito; le parità si rompono per regola, mai a caso.

use super::{DatiElezioni, Formula, LeggeElettorale, seggi_per_circoscrizione};
use crate::errore::Errore;
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

/// Voti validi per regione, uno per lista; coalizioni dichiarate come indici di lista.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Voti {
    pub coalizioni: Vec<Vec<usize>>,
    pub per_regione: BTreeMap<String, Vec<u64>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeggiLista {
    pub proporzionali: u32,
    pub premio: u32,
    /// Seggi vinti nelle circoscrizioni speciali, uninominali e proporzionali.
    pub speciali: u32,
    pub estero: u32,
}

impl SeggiLista {
    #[must_use]
    pub fn totale(&self) -> u32 {
        self.proporzionali + self.premio + self.speciali + self.estero
    }
}

/// `Coalizione(i)`: indice in `Voti::coalizioni`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Soggetto {
    Lista(usize),
    Coalizione(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Esito {
    pub camera: Vec<SeggiLista>,
    pub senato: Vec<SeggiLista>,
    /// Seggi per lista in ogni regione, premio ed estero esclusi.
    pub senato_per_regione: BTreeMap<String, Vec<u32>>,
    pub premio: Option<Soggetto>,
    pub tetto_camera: bool,
    pub tetto_senato: bool,
}

/// Esito del metodo per un soggetto; resto e seggio dal resto servono al tetto del Senato.
#[derive(Clone, Copy, Default)]
struct Quota {
    seggi: u32,
    /// Solo con Hare.
    resto: Option<u128>,
    da_resto: bool,
}

fn quote(formula: Formula, cifre: &[u64], seggi: u32) -> Vec<Quota> {
    let totale: u128 = cifre.iter().map(|&c| u128::from(c)).sum();
    let mut q = vec![Quota::default(); cifre.len()];
    if seggi == 0 || totale == 0 {
        return q;
    }
    let s = u128::from(seggi);
    let c = |i: usize| u128::from(cifre[i]);
    match formula {
        Formula::Dhondt => {
            for _ in 0..seggi {
                // Quoziente `c / (s + 1)` più alto per prodotto incrociato.
                // assunzione A4 del piano 2: a parità la cifra più alta, poi l'indice più basso.
                let quoz = |i: usize| (c(i), u128::from(q[i].seggi) + 1);
                let Some(i) = (0..cifre.len()).max_by(|&a, &b| {
                    let ((ca, da), (cb, db)) = (quoz(a), quoz(b));
                    (ca * db).cmp(&(cb * da)).then(ca.cmp(&cb)).then(b.cmp(&a))
                }) else {
                    return q;
                };
                q[i].seggi += 1;
            }
        }
        Formula::Hare => {
            // Quoziente intero sulla somma delle cifre di questo riparto.
            let quoziente = totale / s;
            let parti: Vec<(u128, u128)> = (quoziente > 0)
                .then(|| {
                    (0..cifre.len())
                        .map(|i| (c(i) / quoziente, c(i) % quoziente))
                        .collect::<Vec<_>>()
                })
                .filter(|p| p.iter().map(|x| x.0).sum::<u128>() <= s)
                // assunzione A10 del piano 2: con pochissimi voti, quoziente naturale esatto.
                .unwrap_or_else(|| {
                    (0..cifre.len())
                        .map(|i| (c(i) * s / totale, c(i) * s % totale))
                        .collect()
                });
            for (qi, &(intera, resto)) in q.iter_mut().zip(&parti) {
                qi.seggi = u32::try_from(intera).unwrap_or(u32::MAX);
                qi.resto = Some(resto);
            }
            let dati: u32 = q.iter().map(|x| x.seggi).sum();
            let mut ordine: Vec<usize> = (0..cifre.len()).collect();
            // assunzione A4 del piano 2: a parità di resto la cifra più alta, poi l'indice
            // (ordinamento stabile).
            ordine.sort_by_key(|&i| (Reverse(parti[i].1), Reverse(cifre[i])));
            for &i in ordine.iter().take(seggi.saturating_sub(dati) as usize) {
                q[i].seggi += 1;
                q[i].da_resto = true;
            }
        }
    }
    q
}

/// Coalizione ammessa o lista sola: tutte le liste fanno la cifra, le ammesse prendono i seggi.
struct InGara {
    liste: Vec<usize>,
    ammesse: Vec<usize>,
}

fn sopra(voti: u64, totale: u64, soglia_pb: u16) -> bool {
    u128::from(voti) * 10_000 >= u128::from(soglia_pb) * u128::from(totale)
}

/// Soggetti in gara nel ramo, coalizioni prima e liste poi: l'ordine rompe le parità (A4).
fn in_gara(
    dati: &DatiElezioni,
    legge: &LeggeElettorale,
    voti: &Voti,
    naz: &[u64],
    senato: bool,
) -> Vec<InGara> {
    let totale: u64 = naz.iter().sum();
    let sopra_soglia = |l: usize| {
        sopra(naz[l], totale, legge.soglia_lista_pb)
            || senato
                && voti.per_regione.values().any(|v| {
                    let t: u64 = v.iter().sum();
                    t > 0 && sopra(v[l], t, dati.soglia_regionale_senato_pb)
                })
    };
    let coalizioni: Vec<&Vec<usize>> = if legge.coalizioni {
        // assunzione A9 del piano 2: coalizione di una lista sola = lista non coalizzata.
        voti.coalizioni.iter().filter(|c| c.len() >= 2).collect()
    } else {
        Vec::new()
    };
    let mut gara = Vec::new();
    let mut coperte = BTreeSet::new();
    for c in coalizioni {
        let cifra: u64 = c.iter().map(|&l| naz[l]).sum();
        let (mut ammesse, altre): (Vec<usize>, Vec<usize>) =
            c.iter().partition(|&&l| sopra_soglia(l));
        if ammesse.is_empty() || !sopra(cifra, totale, legge.soglia_coalizione_pb) {
            continue;
        }
        // assunzione A4 del piano 2: fra le più votate a pari voti, l'indice più basso.
        if legge.ripescaggio
            && let Some(&r) = altre.iter().max_by_key(|&&l| (naz[l], Reverse(l)))
        {
            ammesse.push(r);
        }
        ammesse.sort_unstable();
        coperte.extend(c.iter().copied());
        gara.push(InGara {
            liste: c.clone(),
            ammesse,
        });
    }
    for l in (0..naz.len()).filter(|l| !coperte.contains(l) && sopra_soglia(*l)) {
        gara.push(InGara {
            liste: vec![l],
            ammesse: vec![l],
        });
    }
    gara
}

/// Seggi per lista: `quote` fra i soggetti, poi ogni soggetto alle sue liste ammesse.
fn alle_liste(
    formula: Formula,
    gara: &[InGara],
    per_soggetto: &[Quota],
    cifre: &[u64],
) -> Vec<u32> {
    let mut seggi = vec![0; cifre.len()];
    for (g, q) in gara.iter().zip(per_soggetto) {
        let c: Vec<u64> = g.ammesse.iter().map(|&l| cifre[l]).collect();
        for (&l, ql) in g.ammesse.iter().zip(quote(formula, &c, q.seggi)) {
            seggi[l] += ql.seggi;
        }
    }
    seggi
}

fn cifre_soggetti(gara: &[InGara], cifre: &[u64]) -> Vec<u64> {
    gara.iter()
        .map(|g| g.liste.iter().map(|&l| cifre[l]).sum())
        .collect()
}

/// Voti per lista sommati sulle regioni; il totale è già controllato da `valida`.
fn somma<'a>(regioni: impl Iterator<Item = &'a Vec<u64>>, n: usize) -> Vec<u64> {
    regioni.fold(vec![0; n], |mut a, v| {
        a.iter_mut().zip(v).for_each(|(x, y)| *x += y);
        a
    })
}

fn incoerenti(m: String) -> Errore {
    Errore::VotiIncoerenti(m)
}

/// Numero di liste, o l'incoerenza dei voti con i dati.
fn valida(dati: &DatiElezioni, voti: &Voti) -> Result<usize, Errore> {
    let regioni: BTreeSet<&str> = dati
        .camera
        .circoscrizioni
        .iter()
        .chain(&dati.senato.circoscrizioni)
        .map(|c| c.regione.as_str())
        .collect();
    let chiavi: BTreeSet<&str> = voti.per_regione.keys().map(String::as_str).collect();
    if chiavi != regioni {
        return Err(incoerenti(format!(
            "regioni dei voti {chiavi:?} diverse da quelle delle circoscrizioni {regioni:?}"
        )));
    }
    let n = voti.per_regione.values().next().map_or(0, Vec::len);
    if n == 0 {
        return Err(incoerenti("nessuna lista".into()));
    }
    if let Some((r, v)) = voti.per_regione.iter().find(|(_, v)| v.len() != n) {
        return Err(incoerenti(format!(
            "regione {r}: {} liste invece di {n}",
            v.len()
        )));
    }
    let mut viste = BTreeSet::new();
    for l in voti.coalizioni.iter().flatten() {
        if *l >= n {
            return Err(incoerenti(format!(
                "coalizione con la lista {l}, ultima {}",
                n - 1
            )));
        }
        if !viste.insert(*l) {
            return Err(incoerenti(format!("lista {l} in due coalizioni")));
        }
    }
    voti.per_regione
        .values()
        .flatten()
        .try_fold(0u64, |a, &v| a.checked_add(v))
        .ok_or_else(|| incoerenti("totale dei voti fuori scala".into()))?;
    Ok(n)
}

/// Seggi di Camera e Senato per lista.
///
/// # Errors
/// `VotiIncoerenti` se i voti non combaciano con le regioni, le liste o le coalizioni.
pub fn riparto(dati: &DatiElezioni, legge: &LeggeElettorale, voti: &Voti) -> Result<Esito, Errore> {
    let n = valida(dati, voti)?;
    // Cifre nazionali con le circoscrizioni speciali: decidono le soglie.
    let naz = somma(voti.per_regione.values(), n);

    let camera_cs = &dati.camera.circoscrizioni;
    let speciali: BTreeSet<&str> = camera_cs
        .iter()
        .filter(|c| c.uninominali > 0)
        .map(|c| c.regione.as_str())
        .collect();
    let seggi_camera: u32 = camera_cs
        .iter()
        .zip(seggi_per_circoscrizione(&dati.camera, 0))
        .filter(|(c, _)| c.uninominali == 0)
        .map(|(_, s)| s.seggi.saturating_sub(s.premio))
        .sum();
    let senza_speciali = somma(
        voti.per_regione
            .iter()
            .filter(|(r, _)| !speciali.contains(r.as_str()))
            .map(|(_, v)| v),
        n,
    );
    let gara = in_gara(dati, legge, voti, &naz, false);
    let per_soggetto = quote(
        legge.formula,
        &cifre_soggetti(&gara, &senza_speciali),
        seggi_camera,
    );
    // assunzione A11 del piano 2: senza soggetti ammessi i seggi restano non assegnati.
    let camera = alle_liste(legge.formula, &gara, &per_soggetto, &senza_speciali)
        .into_iter()
        .map(|proporzionali| SeggiLista {
            proporzionali,
            ..SeggiLista::default()
        })
        .collect();

    let gara = in_gara(dati, legge, voti, &naz, true);
    let mut regionali = Vec::new();
    for (c, s) in dati
        .senato
        .circoscrizioni
        .iter()
        .zip(seggi_per_circoscrizione(&dati.senato, 0))
        .filter(|(c, _)| c.uninominali == 0)
    {
        let v = voti
            .per_regione
            .get(&c.regione)
            .ok_or_else(|| incoerenti(format!("regione {} senza voti", c.regione)))?;
        let q = quote(
            legge.formula,
            &cifre_soggetti(&gara, v),
            s.seggi.saturating_sub(s.premio),
        );
        regionali.push((c.regione.as_str(), v, q));
    }
    let mut senato = vec![SeggiLista::default(); n];
    let mut senato_per_regione: BTreeMap<String, Vec<u32>> = voti
        .per_regione
        .keys()
        .map(|r| (r.clone(), vec![0; n]))
        .collect();
    for (r, v, q) in &regionali {
        let liste = alle_liste(legge.formula, &gara, q, v);
        if let Some(pr) = senato_per_regione.get_mut(*r) {
            pr.iter_mut().zip(&liste).for_each(|(a, b)| *a += b);
        }
        senato
            .iter_mut()
            .zip(&liste)
            .for_each(|(a, b)| a.proporzionali += b);
    }

    Ok(Esito {
        camera,
        senato,
        senato_per_regione,
        premio: None,
        tetto_camera: false,
        tetto_senato: false,
    })
}
