//! Riparto dei seggi della legge 180/2026: soglie, metodo, premio e tetti, Camera nazionale
//! (art. 83 DPR 361) e Senato regione per regione (art. 16-bis d.lgs. 533).
//! Puro: stessi argomenti, stesso esito; le parità si rompono per regola, mai a caso.

use super::{
    DatiElezioni, Formula, LeggeElettorale, Premio, SeggiCircoscrizione, seggi_da_quota,
    seggi_per_circoscrizione,
};
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

/// Esito del metodo per un soggetto; resto e suo uso servono al tetto del Senato.
#[derive(Clone, Copy, Default)]
struct Quota {
    seggi: u32,
    /// Resto della divisione per il quoziente; `None` con D'Hondt.
    resto: Option<u128>,
    /// Il resto ha già dato un seggio.
    resto_usato: bool,
}

fn quote(formula: Formula, cifre: &[u64], seggi: u32) -> Vec<Quota> {
    let totale: u128 = cifre.iter().map(|&c| u128::from(c)).sum();
    let mut q = vec![Quota::default(); cifre.len()];
    if seggi == 0 || cifre.is_empty() {
        return q;
    }
    if totale == 0 {
        // assunzione A13 del piano 2: tutte le cifre a 0 valgono come pari, i seggi si
        // conservano in parti uguali e l'avanzo va agli indici più bassi.
        return quote(formula, &vec![1; cifre.len()], seggi);
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
                    // Irraggiungibile: cifre non vuota, controllato sopra.
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
                // ≤ seggi per costruzione: il filtro sopra o il quoziente naturale.
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
                q[i].resto_usato = true;
            }
        }
    }
    q
}

/// Coalizione ammessa o lista sola: tutte le liste fanno la cifra, le ammesse prendono i seggi.
struct InGara {
    soggetto: Soggetto,
    liste: Vec<usize>,
    ammesse: Vec<usize>,
}

fn sopra(voti: u64, totale: u64, soglia_pb: u16) -> bool {
    u128::from(voti) * 10_000 >= u128::from(soglia_pb) * u128::from(totale)
}

/// Soggetti in gara nel ramo, coalizioni prima e liste poi: l'ordine rompe le parità (A4).
fn in_gara(
    legge: &LeggeElettorale,
    soglia_regionale_pb: u16,
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
                    t > 0 && sopra(v[l], t, soglia_regionale_pb)
                })
    };
    // `enumerate` prima del filtro: `Soggetto::Coalizione(i)` è l'indice dichiarato.
    let coalizioni: Vec<(usize, &Vec<usize>)> = if legge.coalizioni {
        // assunzione A9 del piano 2: coalizione di una lista sola = lista non coalizzata.
        voti.coalizioni
            .iter()
            .enumerate()
            .filter(|(_, c)| c.len() >= 2)
            .collect()
    } else {
        Vec::new()
    };
    let mut gara = Vec::new();
    let mut coperte = BTreeSet::new();
    for (i, c) in coalizioni {
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
            soggetto: Soggetto::Coalizione(i),
            liste: c.clone(),
            ammesse,
        });
    }
    for l in (0..naz.len()).filter(|l| !coperte.contains(l) && sopra_soglia(*l)) {
        gara.push(InGara {
            soggetto: Soggetto::Lista(l),
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

/// Soggetto più votato del ramo sulle cifre nazionali con le speciali, se ha la soglia del premio.
fn primo(legge: &LeggeElettorale, gara: &[InGara], naz: &[u64]) -> Option<Soggetto> {
    // assunzione A7 del piano 2: premio alla lista o alla coalizione, stessa regola.
    if matches!(legge.premio, Premio::Nessuno) {
        return None;
    }
    let cifre = cifre_soggetti(gara, naz);
    // assunzione A4 del piano 2: a parità il primo in gara, coalizioni prima delle liste.
    let (i, &cifra) = cifre
        .iter()
        .enumerate()
        .max_by_key(|&(i, &c)| (c, Reverse(i)))?;
    sopra(cifra, naz.iter().sum(), legge.soglia_premio_pb).then_some(gara[i].soggetto)
}

/// Vincitore del premio in un ramo: indice in gara e seggi di premio.
type Vincitore = Option<(usize, u32)>;

fn assegna_premio(
    seggi: &mut [SeggiLista],
    formula: Formula,
    gara: &[InGara],
    vincitore: Vincitore,
    naz: &[u64],
) {
    let Some((w, premio)) = vincitore else {
        return;
    };
    // assunzione A3 del piano 2: premio alle liste ammesse col metodo, sulle cifre nazionali.
    let q = Quota {
        seggi: premio,
        ..Quota::default()
    };
    let liste = alle_liste(formula, std::slice::from_ref(&gara[w]), &[q], naz);
    for (l, p) in seggi.iter_mut().zip(liste) {
        l.premio = p;
    }
}

/// Seggi del soggetto nelle circoscrizioni speciali, su tutte le liste dichiarate.
fn speciali_di(g: &InGara, seggi: &[SeggiLista]) -> u32 {
    g.liste.iter().map(|&l| seggi[l].speciali).sum()
}

/// Regione ordinaria del Senato: cifre dei soggetti e loro quote.
struct Regionale<'a> {
    nome: &'a str,
    voti: &'a [u64],
    cifre: Vec<u64>,
    quote: Vec<Quota>,
}

/// Un seggio dal vincitore `w` all'altro soggetto col resto più alto non usato, o in mancanza
/// con la cifra regionale più alta (art. 16-bis c. 1-ter lett. b). `false` se non si sposta.
fn sposta(r: &mut Regionale<'_>, w: usize) -> bool {
    let Some(resta) = r.quote[w].seggi.checked_sub(1) else {
        return false;
    };
    let altri: Vec<usize> = (0..r.quote.len()).filter(|&i| i != w).collect();
    // assunzione A4 del piano 2: a parità la cifra regionale più alta, poi l'indice.
    let dal_resto = altri
        .iter()
        .filter(|&&i| !r.quote[i].resto_usato)
        .filter_map(|&i| Some((r.quote[i].resto?, r.cifre[i], Reverse(i))))
        .max()
        .map(|(_, _, Reverse(i))| i);
    let j = if let Some(j) = dal_resto {
        r.quote[j].resto_usato = true;
        j
    } else if let Some(j) = altri
        .iter()
        .copied()
        .max_by_key(|&i| (r.cifre[i], Reverse(i)))
    {
        j
    } else {
        return false;
    };
    r.quote[w].seggi = resta;
    r.quote[j].seggi += 1;
    true
}

/// Art. 16-bis c. 1-ter lett. a: toglie al vincitore `w` fino a `eccesso` seggi regionali.
fn togli_eccesso(regioni: &mut [Regionale<'_>], w: usize, mut eccesso: u32) {
    // Prima dove ha preso il seggio col resto, resto crescente su un'unica graduatoria
    // nazionale, uno per regione; ordinamento stabile: a parità l'ordine dei dati.
    let mut col_resto: Vec<usize> = (0..regioni.len())
        .filter(|&r| regioni[r].quote[w].resto_usato)
        .collect();
    // ponytail: col ripiego A10 il resto è in voti × seggi, non in voti, e la graduatoria fra
    // regioni si distorce; succede solo con una manciata di voti per regione. Se servisse:
    // resto in un'unità comune.
    col_resto.sort_by_key(|&r| regioni[r].quote[w].resto);
    let mut toccate = BTreeSet::new();
    for r in col_resto {
        if eccesso == 0 {
            break;
        }
        if sposta(&mut regioni[r], w) {
            eccesso -= 1;
            toccate.insert(r);
        }
    }
    // Poi per percentuale regionale crescente del vincitore, a parità l'ordine dei dati.
    let quota = |r: &Regionale<'_>| {
        let t: u128 = r.voti.iter().map(|&v| u128::from(v)).sum();
        if t == 0 {
            (0, 1)
        } else {
            (u128::from(r.cifre[w]), t)
        }
    };
    let mut ordine: Vec<usize> = (0..regioni.len()).collect();
    ordine.sort_by(|&a, &b| {
        let ((ca, ta), (cb, tb)) = (quota(&regioni[a]), quota(&regioni[b]));
        (ca * tb).cmp(&(cb * ta))
    });
    // Il primo giro salta le regioni toccate sopra, i successivi no; tutti saltano quelle dove
    // il vincitore ha un solo seggio. Un giro successivo senza sottrazioni chiude: il tetto
    // può restare irraggiungibile.
    let mut primo_giro = true;
    while eccesso > 0 {
        let mut tolto = false;
        for &r in &ordine {
            if eccesso == 0 {
                break;
            }
            if (primo_giro && toccate.contains(&r)) || regioni[r].quote[w].seggi <= 1 {
                continue;
            }
            if sposta(&mut regioni[r], w) {
                eccesso -= 1;
                tolto = true;
            }
        }
        if !tolto && !primo_giro {
            break;
        }
        primo_giro = false;
    }
}

fn voti_di<'a>(voti: &'a Voti, regione: &str) -> Result<&'a [u64], Errore> {
    voti.per_regione
        .get(regione)
        .map(Vec::as_slice)
        .ok_or_else(|| Errore::VotiIncoerenti(format!("regione {regione} senza voti")))
}

/// Seggi uninominali di una regione: D'Hondt senza soglia fra coalizioni dichiarate e liste
/// sole, poi dentro la coalizione fra tutte le sue liste, ammesse o no.
fn uninominali(legge: &LeggeElettorale, voti: &Voti, v: &[u64], seggi: u32) -> Vec<u32> {
    // assunzione A2 del piano 2: D'Hondt qualunque sia la formula, nessuna soglia.
    let mut gara = Vec::new();
    let mut coperte = BTreeSet::new();
    if legge.coalizioni {
        // assunzione A9 del piano 2: coalizione di una lista sola = lista non coalizzata.
        for (i, c) in voti
            .coalizioni
            .iter()
            .enumerate()
            .filter(|(_, c)| c.len() >= 2)
        {
            coperte.extend(c.iter().copied());
            gara.push(InGara {
                soggetto: Soggetto::Coalizione(i),
                liste: c.clone(),
                ammesse: c.clone(),
            });
        }
    }
    for l in (0..v.len()).filter(|l| !coperte.contains(l)) {
        gara.push(InGara {
            soggetto: Soggetto::Lista(l),
            liste: vec![l],
            ammesse: vec![l],
        });
    }
    let q = quote(Formula::Dhondt, &cifre_soggetti(&gara, v), seggi);
    alle_liste(Formula::Dhondt, &gara, &q, v)
}

/// Seggi per lista di una circoscrizione speciale: gli uninominali, poi la parte proporzionale
/// col metodo della legge fra i soggetti ammessi del ramo, sui voti della regione.
fn speciale(
    legge: &LeggeElettorale,
    voti: &Voti,
    gara: &[InGara],
    v: &[u64],
    s: &SeggiCircoscrizione,
) -> Vec<u32> {
    let unin = uninominali(legge, voti, v, s.uninominali);
    let q = quote(
        legge.formula,
        &cifre_soggetti(gara, v),
        s.seggi.saturating_sub(s.uninominali),
    );
    let prop = alle_liste(legge.formula, gara, &q, v);
    unin.iter().zip(prop).map(|(u, p)| u + p).collect()
}

/// Seggi del ramo per lista con solo l'estero: liste ammesse in ordine crescente, sulle cifre
/// nazionali.
fn estero(formula: Formula, gara: &[InGara], naz: &[u64], seggi: u32) -> Vec<SeggiLista> {
    // assunzione A1 del piano 2: estero col metodo della legge fra le liste ammesse.
    let mut ammesse: Vec<usize> = gara
        .iter()
        .flat_map(|g| g.ammesse.iter().copied())
        .collect();
    ammesse.sort_unstable();
    let cifre: Vec<u64> = ammesse.iter().map(|&l| naz[l]).collect();
    let mut out = vec![SeggiLista::default(); naz.len()];
    for (&l, q) in ammesse.iter().zip(quote(formula, &cifre, seggi)) {
        out[l].estero = q.seggi;
    }
    out
}

/// Voti per lista sommati sulle regioni; il totale è già controllato da `valida`.
fn somma<'a>(regioni: impl Iterator<Item = &'a Vec<u64>>, n: usize) -> Vec<u64> {
    regioni.fold(vec![0; n], |mut a, v| {
        a.iter_mut().zip(v).for_each(|(x, y)| *x += y);
        a
    })
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
        return Err(Errore::VotiIncoerenti(format!(
            "regioni dei voti {chiavi:?} diverse da quelle delle circoscrizioni {regioni:?}"
        )));
    }
    let n = voti.per_regione.values().next().map_or(0, Vec::len);
    if n == 0 {
        return Err(Errore::VotiIncoerenti("nessuna lista".into()));
    }
    if let Some((r, v)) = voti.per_regione.iter().find(|(_, v)| v.len() != n) {
        return Err(Errore::VotiIncoerenti(format!(
            "regione {r}: {} liste invece di {n}",
            v.len()
        )));
    }
    let mut viste = BTreeSet::new();
    for l in voti.coalizioni.iter().flatten() {
        if *l >= n {
            return Err(Errore::VotiIncoerenti(format!(
                "coalizione con la lista {l}, ultima {}",
                n - 1
            )));
        }
        if !viste.insert(*l) {
            return Err(Errore::VotiIncoerenti(format!(
                "lista {l} in due coalizioni"
            )));
        }
    }
    voti.per_regione
        .values()
        .flatten()
        .try_fold(0u64, |a, &v| a.checked_add(v))
        .ok_or_else(|| Errore::VotiIncoerenti("totale dei voti fuori scala".into()))?;
    Ok(n)
}

/// Camera: speciali, riparto nazionale col tetto (art. 83 c. 1-ter), premio ed estero.
/// Seggi per lista e se il tetto è scattato.
fn camera(
    dati: &DatiElezioni,
    legge: &LeggeElettorale,
    voti: &Voti,
    naz: &[u64],
    gara: &[InGara],
    vincitore: Vincitore,
) -> Result<(Vec<SeggiLista>, bool), Errore> {
    let cs = &dati.camera.circoscrizioni;
    let premio = vincitore.map_or(0, |(_, p)| p);
    let sc = seggi_per_circoscrizione(&dati.camera, premio);
    let speciali: BTreeSet<&str> = cs
        .iter()
        .filter(|c| c.uninominali > 0)
        .map(|c| c.regione.as_str())
        .collect();
    let seggi_nazionali: u32 = cs
        .iter()
        .zip(&sc)
        .filter(|(c, _)| c.uninominali == 0)
        .map(|(_, s)| s.seggi.saturating_sub(s.premio))
        .sum();
    let senza_speciali = somma(
        voti.per_regione
            .iter()
            .filter(|(r, _)| !speciali.contains(r.as_str()))
            .map(|(_, v)| v),
        naz.len(),
    );
    let mut seggi = estero(legge.formula, gara, naz, dati.camera.estero);
    for (c, s) in cs.iter().zip(&sc).filter(|(c, _)| c.uninominali > 0) {
        let liste = speciale(legge, voti, gara, voti_di(voti, &c.regione)?, s);
        for (l, n) in seggi.iter_mut().zip(liste) {
            l.speciali += n;
        }
    }
    let cifre = cifre_soggetti(gara, &senza_speciali);
    let mut per_soggetto = quote(legge.formula, &cifre, seggi_nazionali);
    let mut scattato = false;
    // assunzione A12 del piano 2: vincitore unico soggetto ammesso, il tetto non si applica.
    if let (Some((w, _)), Some(t)) = (vincitore, legge.tetto)
        && gara.len() > 1
    {
        let tetto = seggi_da_quota(t.camera_pb, dati.camera.seggi);
        let spec = speciali_di(&gara[w], &seggi);
        if premio
            .saturating_add(per_soggetto[w].seggi)
            .saturating_add(spec)
            > tetto
        {
            // Al vincitore il resto del tetto, agli altri gli altri seggi sulle loro cifre
            // senza speciali.
            scattato = true;
            let al_vincitore = tetto.saturating_sub(premio).saturating_sub(spec);
            let altri: Vec<usize> = (0..gara.len()).filter(|&i| i != w).collect();
            let c: Vec<u64> = altri.iter().map(|&i| cifre[i]).collect();
            let q = quote(
                legge.formula,
                &c,
                seggi_nazionali.saturating_sub(al_vincitore),
            );
            for (&i, qi) in altri.iter().zip(q) {
                per_soggetto[i] = qi;
            }
            per_soggetto[w].seggi = al_vincitore;
        }
    }
    // assunzione A11 del piano 2: senza soggetti ammessi i seggi restano non assegnati.
    let prop = alle_liste(legge.formula, gara, &per_soggetto, &senza_speciali);
    for (l, p) in seggi.iter_mut().zip(prop) {
        l.proporzionali = p;
    }
    assegna_premio(&mut seggi, legge.formula, gara, vincitore, naz);
    Ok((seggi, scattato))
}

/// Seggi del Senato per lista, per regione (premio ed estero esclusi) e se il tetto è scattato.
type EsitoSenato = (Vec<SeggiLista>, BTreeMap<String, Vec<u32>>, bool);

/// Senato: speciali, riparto regionale col tetto (art. 16-bis c. 1-ter), premio ed estero.
fn senato(
    dati: &DatiElezioni,
    legge: &LeggeElettorale,
    voti: &Voti,
    naz: &[u64],
    gara: &[InGara],
    vincitore: Vincitore,
) -> Result<EsitoSenato, Errore> {
    let cs = &dati.senato.circoscrizioni;
    let premio = vincitore.map_or(0, |(_, p)| p);
    let sc = seggi_per_circoscrizione(&dati.senato, premio);
    let mut seggi = estero(legge.formula, gara, naz, dati.senato.estero);
    let mut per_regione: BTreeMap<String, Vec<u32>> = voti
        .per_regione
        .keys()
        .map(|r| (r.clone(), vec![0; naz.len()]))
        .collect();
    for (c, s) in cs.iter().zip(&sc).filter(|(c, _)| c.uninominali > 0) {
        let liste = speciale(legge, voti, gara, voti_di(voti, &c.regione)?, s);
        if let Some(pr) = per_regione.get_mut(&c.regione) {
            pr.iter_mut().zip(&liste).for_each(|(a, b)| *a += b);
        }
        seggi
            .iter_mut()
            .zip(&liste)
            .for_each(|(a, b)| a.speciali += b);
    }
    let mut regionali = Vec::new();
    for (c, s) in cs.iter().zip(&sc).filter(|(c, _)| c.uninominali == 0) {
        let v = voti_di(voti, &c.regione)?;
        let cifre = cifre_soggetti(gara, v);
        let quote = quote(legge.formula, &cifre, s.seggi.saturating_sub(s.premio));
        regionali.push(Regionale {
            nome: c.regione.as_str(),
            voti: v,
            cifre,
            quote,
        });
    }
    let mut scattato = false;
    // assunzione A12 del piano 2: vincitore unico soggetto ammesso, il tetto non si applica.
    if let (Some((w, _)), Some(t)) = (vincitore, legge.tetto)
        && gara.len() > 1
    {
        let tetto = seggi_da_quota(t.senato_pb, dati.senato.seggi);
        let totale = regionali.iter().map(|r| r.quote[w].seggi).fold(
            premio.saturating_add(speciali_di(&gara[w], &seggi)),
            u32::saturating_add,
        );
        if let Some(eccesso) = totale.checked_sub(tetto).filter(|&e| e > 0) {
            scattato = true;
            togli_eccesso(&mut regionali, w, eccesso);
        }
    }
    for r in &regionali {
        let liste = alle_liste(legge.formula, gara, &r.quote, r.voti);
        if let Some(pr) = per_regione.get_mut(r.nome) {
            pr.iter_mut().zip(&liste).for_each(|(a, b)| *a += b);
        }
        seggi
            .iter_mut()
            .zip(&liste)
            .for_each(|(a, b)| a.proporzionali += b);
    }
    assegna_premio(&mut seggi, legge.formula, gara, vincitore, naz);
    Ok((seggi, per_regione, scattato))
}

/// Seggi di Camera e Senato per lista.
///
/// # Errors
/// `VotiIncoerenti` se i voti non combaciano con le regioni, le liste o le coalizioni;
/// `LeggeElettorale` se la legge esce dal menu dei dati o il premio contraddice le coalizioni.
pub fn riparto(dati: &DatiElezioni, legge: &LeggeElettorale, voti: &Voti) -> Result<Esito, Errore> {
    let n = valida(dati, voti)?;
    legge.valida(&dati.menu)?;
    // Cifre nazionali con le circoscrizioni speciali: decidono soglie e premio.
    // assunzione A5 del piano 2: stessi `Voti` per Camera e Senato.
    let naz = somma(voti.per_regione.values(), n);
    let soglia_regionale = dati.soglia_regionale_senato_pb;
    let gara_camera = in_gara(legge, soglia_regionale, voti, &naz, false);
    let gara_senato = in_gara(legge, soglia_regionale, voti, &naz, true);
    // art. 83 c. 1 e art. 16-bis (ricerca sistemi-elettorali.md:39): premio solo se lo vince
    // lo stesso soggetto nei due rami.
    let premio =
        primo(legge, &gara_camera, &naz).filter(|&s| primo(legge, &gara_senato, &naz) == Some(s));
    let vincitore = |gara: &[InGara], seggi: u32| -> Vincitore {
        let w = gara.iter().position(|g| Some(g.soggetto) == premio)?;
        Some((w, seggi_da_quota(legge.taglia_premio_pb, seggi)))
    };
    let (camera, tetto_camera) = camera(
        dati,
        legge,
        voti,
        &naz,
        &gara_camera,
        vincitore(&gara_camera, dati.camera.seggi),
    )?;
    let (senato, senato_per_regione, tetto_senato) = senato(
        dati,
        legge,
        voti,
        &naz,
        &gara_senato,
        vincitore(&gara_senato, dati.senato.seggi),
    )?;
    Ok(Esito {
        camera,
        senato,
        senato_per_regione,
        premio,
        tetto_camera,
        tetto_senato,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elezioni::Premio;

    #[test]
    fn coalizione_tiene_l_indice_dichiarato() {
        let legge = LeggeElettorale {
            soglia_lista_pb: 300,
            coalizioni: true,
            soglia_coalizione_pb: 1000,
            ripescaggio: true,
            premio: Premio::Nessuno,
            soglia_premio_pb: 4200,
            taglia_premio_pb: 1750,
            tetto: None,
            formula: Formula::Hare,
        };
        let voti = Voti {
            coalizioni: vec![vec![2], vec![0, 1]],
            per_regione: BTreeMap::from([("r".into(), vec![400, 300, 300])]),
        };
        let gara = in_gara(&legge, 2000, &voti, &[400, 300, 300], false);
        let soggetti: Vec<Soggetto> = gara.iter().map(|g| g.soggetto).collect();
        assert_eq!(soggetti, [Soggetto::Coalizione(1), Soggetto::Lista(2)]);
    }
}
