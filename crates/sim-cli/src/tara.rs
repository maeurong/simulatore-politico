//! `tara`: porta il turno zero sui bersagli ritoccando le costanti di astensione delle regioni e le valence dei partiti.
//!
//! Contrazione alla BLP: `astensione_r += ln((1-a)/a) - ln((1-â)/â)` con `a` l'affluenza bersaglio
//! e `â` quella del turno zero, `valence_j += ln(s_j/ŝ_j)` con `s`, `ŝ` quote fra i votanti.
//! Le intenzioni sono quelle vere: il sondaggio ha media vera + bias, e il bias comune ha media zero.

use std::path::PathBuf;

use sim_core::elettorato::{Condizione, Fascia, indice_gruppo};
use sim_core::{DatiGioco, FileDati, Partita};

const TOLLERANZA: f64 = 1e-4;
const GIRI: u32 = 200;
const USO: &str = "uso: sim-cli tara --dati <cartella> --bersagli <file>";

struct Bersagli {
    /// Quote fra i votanti per posto, somma 1.
    quote: Vec<f64>,
    /// Affluenza per regione, nell'ordine di `elettorato.toml`.
    affluenza: Vec<f64>,
}

/// Un numero TOML, scritto con o senza la virgola.
fn numero(v: &toml::Value) -> Option<f64> {
    v.as_float().or_else(|| v.as_integer().map(|i| i as f64))
}

fn leggi_bersagli(testo: &str, n_posti: usize, regioni: &[String]) -> Result<Bersagli, String> {
    let t: toml::Table = testo
        .parse()
        .map_err(|e| format!("bersagli illeggibili: {e}"))?;
    let posti = t
        .get("posti")
        .and_then(toml::Value::as_array)
        .ok_or("bersagli: manca `posti`")?;
    let posti: Vec<f64> = posti
        .iter()
        .map(|p| numero(p).ok_or("bersagli: `posti` vuole numeri"))
        .collect::<Result<_, _>>()?;
    if posti.len() != n_posti {
        return Err(format!(
            "bersaglio non valido: {} posti, i partiti sono {n_posti}",
            posti.len()
        ));
    }
    if let Some(i) = posti.iter().position(|p| !p.is_finite() || *p <= 0.0) {
        return Err(format!(
            "bersaglio non valido: posto {} = {}, serve una quota positiva",
            i + 1,
            posti[i]
        ));
    }
    let somma: f64 = posti.iter().sum();
    if (somma - 100.0).abs() > 1e-6 {
        return Err(format!(
            "bersaglio non valido: i posti sommano {somma}, non 100"
        ));
    }
    let tabella = t
        .get("affluenza")
        .and_then(toml::Value::as_table)
        .ok_or("bersagli: manca [affluenza]")?;
    let affluenza = regioni
        .iter()
        .map(|id| {
            let a = tabella
                .get(id)
                .and_then(numero)
                .ok_or_else(|| format!("bersaglio: manca l'affluenza di {id}"))?;
            if a > 0.0 && a < 100.0 {
                Ok(a / 100.0)
            } else {
                Err(format!(
                    "bersaglio non valido: affluenza di {id} = {a}, serve fra 0 e 100"
                ))
            }
        })
        .collect::<Result<_, String>>()?;
    Ok(Bersagli {
        quote: posti.iter().map(|p| p / 100.0).collect(),
        affluenza,
    })
}

/// Riscrive riga per riga: `nuova` vede ogni riga e può sostituirla, il resto resta com'è.
/// Restituisce anche quante righe ha sostituito.
fn riscrivi(testo: &str, mut nuova: impl FnMut(&str) -> Option<String>) -> (String, usize) {
    let mut sostituite = 0;
    let nuovo = testo
        .split_inclusive('\n')
        .map(|riga| match nuova(riga.trim_end()) {
            Some(r) => {
                sostituite += 1;
                format!("{r}\n")
            }
            None => riga.to_string(),
        })
        .collect();
    (nuovo, sostituite)
}

fn riscrivi_valence(testo: &str, valence: &[f64]) -> (String, usize) {
    let (mut dentro, mut k) = (false, 0);
    riscrivi(testo, |riga| {
        if riga.starts_with('[') {
            dentro = riga == "[partito.valence]";
            k += usize::from(dentro);
        }
        (dentro && riga.starts_with("valore = "))
            .then(|| valence.get(k - 1).map(|v| format!("valore = {v:.6}")))
            .flatten()
    })
}

fn riscrivi_astensione(testo: &str, ids: &[String], astensione: &[f64]) -> (String, usize) {
    let (mut dentro, mut i) = (false, None);
    riscrivi(testo, |riga| {
        if riga.starts_with('[') {
            dentro = riga == "[[regione]]";
            i = None;
        }
        if dentro && riga.starts_with("id = ") {
            i = ids.iter().position(|id| riga == format!("id = \"{id}\""));
        }
        (dentro && riga.starts_with("astensione = "))
            .then(|| {
                i.and_then(|i| astensione.get(i))
                    .map(|a| format!("astensione = {a:.6}"))
            })
            .flatten()
    })
}

pub fn esegui(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    let (mut dati, mut bersagli) = (None, None);
    while let Some(nome) = args.next() {
        let valore = args
            .next()
            .ok_or_else(|| format!("manca il valore per {nome}"))?;
        match nome.as_str() {
            "--dati" => dati = Some(valore),
            "--bersagli" => bersagli = Some(valore),
            _ => return Err(format!("argomento sconosciuto: {nome} ({USO})")),
        }
    }
    let cartella = PathBuf::from(dati.ok_or_else(|| format!("manca --dati ({USO})"))?);
    let percorso = bersagli.ok_or_else(|| format!("manca --bersagli ({USO})"))?;
    let testo = std::fs::read_to_string(&percorso)
        .map_err(|e| format!("impossibile leggere {percorso}: {e}"))?;

    let mut file = FileDati::leggi(|nome| crate::leggi(&cartella, nome))?;
    let iniziali = DatiGioco::da_testi(&file).map_err(|e| e.to_string())?;
    let partiti = &iniziali.scenario().partiti;
    let ids_partiti: Vec<String> = partiti.iter().map(|p| p.id.clone()).collect();
    let limiti: Vec<(f64, f64)> = partiti
        .iter()
        .map(|p| (p.valence.min, p.valence.max))
        .collect();
    let coalizioni = &iniziali.scenario().coalizioni;
    let regioni = &iniziali.elettorato().regioni;
    let ids_regioni: Vec<String> = regioni.iter().map(|r| r.id.clone()).collect();
    let mut astensione: Vec<f64> = regioni.iter().map(|r| r.astensione).collect();
    let mut valence: Vec<f64> = partiti.iter().map(|p| p.valence.valore).collect();
    let bersagli = leggi_bersagli(&testo, partiti.len(), &ids_regioni)?;

    let (_, n) = riscrivi_valence(&file.scenario, &valence);
    if n != valence.len() {
        return Err(format!(
            "scenario.toml: {n} righe `valore =` in [partito.valence], attese {}",
            valence.len()
        ));
    }
    let (_, n) = riscrivi_astensione(&file.elettorato, &ids_regioni, &astensione);
    if n != ids_regioni.len() {
        return Err(format!(
            "elettorato.toml: {n} righe `astensione =` in [[regione]] riconosciute per id, attese {}",
            ids_regioni.len()
        ));
    }
    // Gruppi di ogni regione dalla mappa canonica del motore, controllata sul campo `regione`.
    let gruppi_dati = &iniziali.elettorato().gruppi;
    if gruppi_dati.len() != ids_regioni.len() * Condizione::TUTTE.len() * Fascia::TUTTE.len() {
        return Err(format!(
            "elettorato.toml: {} gruppi, attesi {} per regione",
            gruppi_dati.len(),
            Condizione::TUTTE.len() * Fascia::TUTTE.len()
        ));
    }
    let mut gruppi_di: Vec<Vec<usize>> = vec![Vec::new(); ids_regioni.len()];
    for (r, id) in ids_regioni.iter().enumerate() {
        for c in Condizione::TUTTE {
            for f in Fascia::TUTTE {
                let i = indice_gruppo(r, c, f);
                let g = &gruppi_dati[i];
                if g.regione != *id || g.condizione != c || g.fascia != f {
                    return Err(format!(
                        "elettorato.toml: il gruppo {i} e' ({}, {}, {}), la mappa canonica vuole ({id}, {c}, {f})",
                        g.regione, g.condizione, g.fascia
                    ));
                }
                gruppi_di[r].push(i);
            }
        }
    }

    let mut giro = 0;
    loop {
        let partita = Partita::nuova(DatiGioco::da_testi(&file).map_err(|e| e.to_string())?, 0);
        let int = partita.intenzioni_vere();
        let votanti = 1.0 - int.nazionali[valence.len()];
        let quote: Vec<f64> = int.nazionali[..valence.len()]
            .iter()
            .map(|n| n / votanti)
            .collect();
        let affluenza: Vec<f64> = gruppi_di
            .iter()
            .map(|gruppi| {
                let (mut peso, mut astenuti) = (0.0, 0.0);
                for &g in gruppi {
                    let q = partita.elettorato().gruppi[g].quota;
                    peso += q;
                    astenuti += q * int.per_gruppo[g][valence.len()];
                }
                1.0 - astenuti / peso
            })
            .collect();
        let scarto = affluenza
            .iter()
            .zip(&bersagli.affluenza)
            .chain(quote.iter().zip(&bersagli.quote))
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, f64::max);
        if !scarto.is_finite() {
            return Err("la taratura diverge: scarto non finito".into());
        }
        if scarto < TOLLERANZA {
            let coalizioni = coalizioni
                .iter()
                .map(|c| {
                    let quota = c
                        .iter()
                        .map(|id| {
                            let j = ids_partiti.iter().position(|p| p == id);
                            j.map(|j| quote[j]).ok_or_else(|| {
                                format!("coalizione con un partito sconosciuto: {id}")
                            })
                        })
                        .sum::<Result<f64, String>>()?;
                    Ok(serde_json::json!({"partiti": c, "quota": quota}))
                })
                .collect::<Result<Vec<_>, String>>()?;
            // Prima si scrivono entrambi i .tmp, poi si rinominano: un errore di scrittura lascia gli originali intatti
            // (al più un .tmp orfano). ponytail: se fallisce il secondo rename lo scenario resta tarato e l'elettorato
            // no; si rilancia `tara`, che riparte dai file su disco.
            let nuovi = [
                ("scenario.toml", &file.scenario),
                ("elettorato.toml", &file.elettorato),
            ];
            for (nome, testo) in nuovi {
                let p = cartella.join(format!("{nome}.tmp"));
                std::fs::write(&p, testo)
                    .map_err(|e| format!("impossibile scrivere {}: {e}", p.display()))?;
            }
            for (nome, _) in nuovi {
                let (da, a) = (cartella.join(format!("{nome}.tmp")), cartella.join(nome));
                std::fs::rename(&da, &a)
                    .map_err(|e| format!("impossibile scrivere {}: {e}", a.display()))?;
            }
            let uscita = serde_json::json!({
                "giri": giro,
                "scarto": scarto,
                "affluenza_nazionale": 1.0 - int.nazionali[valence.len()],
                "valence": ids_partiti.iter().zip(&valence).collect::<Vec<_>>(),
                "quote": quote,
                "coalizioni": coalizioni,
            });
            return serde_json::to_string_pretty(&uscita).map_err(|e| e.to_string());
        }
        if giro == GIRI {
            return Err(format!(
                "la taratura non converge in {GIRI} giri: scarto {scarto}"
            ));
        }
        for ((a, ah), b) in astensione
            .iter_mut()
            .zip(&affluenza)
            .zip(&bersagli.affluenza)
        {
            *a += ((1.0 - b) / b).ln() - ((1.0 - ah) / ah).ln();
        }
        for (j, v) in valence.iter_mut().enumerate() {
            *v += (bersagli.quote[j] / quote[j]).ln();
            let (min, max) = limiti[j];
            if !(min..=max).contains(v) {
                return Err(format!(
                    "valence di {} fuori da [{min}, {max}]: {v}",
                    ids_partiti[j]
                ));
            }
        }
        file.scenario = riscrivi_valence(&file.scenario, &valence).0;
        file.elettorato = riscrivi_astensione(&file.elettorato, &ids_regioni, &astensione).0;
        giro += 1;
    }
}
