//! `tara`: porta il turno zero sui bersagli ritoccando le costanti di astensione delle regioni e le valence dei partiti.
//!
//! Contrazione alla BLP: `astensione_r += ln((1-a)/a) - ln((1-â)/â)` con `a` l'affluenza bersaglio
//! e `â` quella del turno zero, `valence_j += ln(s_j/ŝ_j)` con `s`, `ŝ` quote fra i votanti.
//! Le intenzioni sono quelle vere: il sondaggio ha media vera + bias, e il bias comune ha media zero.

use std::path::Path;

use sim_core::{DatiGioco, FileDati, Partita};

const TOLLERANZA: f64 = 1e-4;
const GIRI: u32 = 200;

struct Bersagli {
    /// Quote fra i votanti per posto, somma 1.
    quote: Vec<f64>,
    /// Affluenza per regione, nell'ordine di `elettorato.toml`.
    affluenza: Vec<f64>,
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
        .map(|p| {
            p.as_float()
                .ok_or("bersagli: `posti` vuole numeri decimali")
        })
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
                .and_then(toml::Value::as_float)
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
fn riscrivi(testo: &str, mut nuova: impl FnMut(&str) -> Option<String>) -> String {
    testo
        .split_inclusive('\n')
        .map(|riga| match nuova(riga.trim_end()) {
            Some(r) => format!("{r}\n"),
            None => riga.to_string(),
        })
        .collect()
}

fn riscrivi_valence(testo: &str, valence: &[f64]) -> String {
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

fn riscrivi_astensione(testo: &str, ids: &[String], astensione: &[f64]) -> String {
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
            _ => return Err(format!("argomento sconosciuto: {nome}")),
        }
    }
    let cartella = Path::new(&dati.ok_or("manca --dati <cartella>")?).to_path_buf();
    let percorso = bersagli.ok_or("manca --bersagli <file>")?;
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
    let coalizioni = iniziali.scenario().coalizioni.clone();
    let regioni = &iniziali.elettorato().regioni;
    let ids_regioni: Vec<String> = regioni.iter().map(|r| r.id.clone()).collect();
    let mut astensione: Vec<f64> = regioni.iter().map(|r| r.astensione).collect();
    let mut valence: Vec<f64> = partiti.iter().map(|p| p.valence.valore).collect();
    let bersagli = leggi_bersagli(&testo, partiti.len(), &ids_regioni)?;

    for giro in 0..=GIRI {
        let partita = Partita::nuova(DatiGioco::da_testi(&file).map_err(|e| e.to_string())?, 0);
        let int = partita.intenzioni_vere();
        let votanti = 1.0 - int.nazionali[valence.len()];
        let quote: Vec<f64> = int.nazionali[..valence.len()]
            .iter()
            .map(|n| n / votanti)
            .collect();
        let per_regione = int.per_gruppo.len() / ids_regioni.len();
        let affluenza: Vec<f64> = (0..ids_regioni.len())
            .map(|r| {
                let gruppi = r * per_regione..(r + 1) * per_regione;
                let (mut peso, mut astenuti) = (0.0, 0.0);
                for g in gruppi {
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
            for (nome, testo) in [(4, &file.scenario), (6, &file.elettorato)] {
                let p = cartella.join(FileDati::NOMI[nome]);
                std::fs::write(&p, testo)
                    .map_err(|e| format!("impossibile scrivere {}: {e}", p.display()))?;
            }
            let quota_di = |c: &Vec<String>| -> f64 {
                c.iter()
                    .filter_map(|id| ids_partiti.iter().position(|p| p == id))
                    .map(|j| quote[j])
                    .sum()
            };
            let uscita = serde_json::json!({
                "giri": giro,
                "scarto": scarto,
                "affluenza_nazionale": 1.0 - int.nazionali[valence.len()],
                "valence": ids_partiti.iter().zip(&valence).collect::<Vec<_>>(),
                "quote": quote,
                "coalizioni": coalizioni.iter().map(|c| serde_json::json!({"partiti": c, "quota": quota_di(c)})).collect::<Vec<_>>(),
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
        file.scenario = riscrivi_valence(&file.scenario, &valence);
        file.elettorato = riscrivi_astensione(&file.elettorato, &ids_regioni, &astensione);
    }
    Err("giri esauriti".into())
}
