use sim_core::dati::{DatiGioco, FileDati};

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn file_base() -> FileDati {
    FileDati {
        dicasteri: include_str!("../../../../dati/base/dicasteri.toml").into(),
        manopole: include_str!("../../../../dati/base/manopole.toml").into(),
        economia: include_str!("../../../../dati/base/economia.toml").into(),
        regioni: include_str!("../../../../dati/base/regioni.toml").into(),
        scenario: include_str!("../../../../dati/base/scenario.toml").into(),
    }
}

#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn dati_base() -> DatiGioco {
    DatiGioco::da_testi(&file_base()).expect("i dati di base devono essere validi")
}

/// Come `file_base()`, con `valore = 0.0` in ogni sezione `sigma_*` delle manopole.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn senza_shock() -> FileDati {
    let mut f = file_base();
    let mut dentro = false;
    let mut azzerate = 0;
    f.manopole = f
        .manopole
        .lines()
        .map(|riga| {
            if riga.starts_with('[') {
                dentro = riga.starts_with("[sigma_");
            }
            if dentro && riga.starts_with("valore = ") {
                azzerate += 1;
                "valore = 0.0".to_string()
            } else {
                riga.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    assert!(azzerate > 0, "nessuna manopola sigma_ azzerata");
    f
}

/// Sostituisce la riga `valore = ...` subito sotto `[nome]` nello scenario.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn con_ingresso(mut f: FileDati, nome: &str, valore: f64) -> FileDati {
    let testa = format!("[{nome}]\n");
    let inizio = f
        .scenario
        .find(&testa)
        .unwrap_or_else(|| panic!("sezione {nome} assente nello scenario"))
        + testa.len();
    let fine = inizio + f.scenario[inizio..].find('\n').expect("riga valore");
    f.scenario
        .replace_range(inizio..fine, &format!("valore = {valore:?}"));
    f
}

/// `replacen(da, a, 1)` nel file di nome `file`; panico se `da` non c'e'.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn sostituisci(mut f: FileDati, file: &str, da: &str, a: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    assert!(testo.contains(da), "{da:?} assente in {file}");
    *testo = testo.replacen(da, a, 1);
    f
}

fn testo_di<'a>(f: &'a mut FileDati, file: &str) -> &'a mut String {
    match file {
        "dicasteri" => &mut f.dicasteri,
        "manopole" => &mut f.manopole,
        "economia" => &mut f.economia,
        "regioni" => &mut f.regioni,
        "scenario" => &mut f.scenario,
        _ => panic!("file di dati sconosciuto: {file}"),
    }
}

/// Nel file `file`, il primo valore dopo `prefisso` (fino a `,` `]` `}` spazio o fine riga)
/// diventa `nuovo`. Non dipende dai numeri dei dati, solo dalla forma del file.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn imposta(mut f: FileDati, file: &str, prefisso: &str, nuovo: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    let inizio = testo
        .find(prefisso)
        .unwrap_or_else(|| panic!("{prefisso:?} assente in {file}"))
        + prefisso.len();
    let fine = inizio
        + testo[inizio..]
            .find([',', ']', '}', ' ', '\n'])
            .expect("fine del valore");
    testo.replace_range(inizio..fine, nuovo);
    f
}

/// Ogni riga di `file` che inizia con `prefisso` diventa `nuova`.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn riscrivi(mut f: FileDati, file: &str, prefisso: &str, nuova: &str) -> FileDati {
    let testo = testo_di(&mut f, file);
    *testo = testo
        .lines()
        .map(|r| if r.starts_with(prefisso) { nuova } else { r })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    f
}
