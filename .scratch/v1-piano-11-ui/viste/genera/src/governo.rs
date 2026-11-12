//! La sezione Governo (ticket 18): i costi di Ferma, Dirotta e FermaCantiere su ogni ministro
//! del 03, misurati comandando una copia della partita (la Vista non li porta: `costo_ferma` e
//! `costo_dirotta` sono della PR delle esposizioni); le scelte fatte, l'interim con la rosa, il
//! successore in rodaggio; un tecnico messo dalla spartizione.

use sim_core::legislatura::Assegnazione;
use sim_core::{Comando, DatiGioco, Partita};
use serde_json::Value;

use super::{Esito, Uscita, cerca_al_governo, comanda, con_la_destra, fonda, governo_03};

fn sodd(p: &Partita, dicastero: &str) -> Option<f64> {
    let v = p.vista();
    let m = v.ministeri.iter().find(|m| m.dicastero == dicastero)?;
    m.ministro.as_ref().map(|x| x.soddisfazione)
}

/// La soddisfazione che il comando costerebbe al ministro, o il rifiuto del core.
fn prova(p: &Partita, dicastero: &str, c: Comando) -> Value {
    let mut q = p.clone();
    let prima = sodd(p, dicastero).unwrap_or(0.0);
    match q.comanda(c) {
        Ok(()) => serde_json::json!(sodd(&q, dicastero).unwrap_or(0.0) - prima),
        Err(e) => serde_json::json!({ "rifiuto": e.to_string() }),
    }
}

fn costi(p: &Partita) -> Value {
    let v = p.vista();
    let regioni: Vec<String> = v.sondaggio.regioni.iter().map(|r| r.id.clone()).collect();
    let mut out = serde_json::Map::new();
    for m in &v.ministeri {
        let d = m.dicastero.clone();
        let mut x = serde_json::Map::new();
        x.insert("ferma".into(), prova(p, &d, Comando::Ferma { dicastero: d.clone() }));
        let atti: serde_json::Map<String, Value> = (m.atti.iter())
            .filter(|a| m.agenda.as_ref().is_none_or(|g| g.atto != a.id))
            .map(|a| {
                let c = Comando::Dirotta { dicastero: d.clone(), atto: Some(a.id.clone()), regione: None };
                (a.id.clone(), prova(p, &d, c))
            })
            .collect();
        x.insert("dirotta_atti".into(), Value::Object(atti));
        if m.agenda.as_ref().is_some_and(|g| g.regione.is_some()) {
            let reg: serde_json::Map<String, Value> = (regioni.iter())
                .map(|r| {
                    let c = Comando::Dirotta { dicastero: d.clone(), atto: None, regione: Some(r.clone()) };
                    (r.clone(), prova(p, &d, c))
                })
                .collect();
            x.insert("dirotta_regioni".into(), Value::Object(reg));
        }
        let cant: serde_json::Map<String, Value> = (m.cantieri.iter())
            .map(|c| {
                let k = Comando::FermaCantiere { dicastero: d.clone(), atto: c.atto.clone() };
                (c.atto.clone(), prova(p, &d, k))
            })
            .collect();
        x.insert("ferma_cantiere".into(), Value::Object(cant));
        out.insert(d, Value::Object(x));
    }
    Value::Object(out)
}

fn intese(p: &Partita) -> Value {
    p.vista().alleati.iter().map(|a| (a.partito.clone(), serde_json::json!(a.intesa))).collect::<serde_json::Map<_, _>>().into()
}

pub fn governo(d: &DatiGioco) -> Esito<Uscita> {
    let mut out = serde_json::Map::new();
    let mut note = Vec::new();
    let p03 = governo_03(d)?;
    out.insert("costi_03".into(), costi(&p03));
    out.insert("intese_03".into(), intese(&p03));

    // Le quattro mosse sul borsino, una per ministro.
    let mut p = p03.clone();
    let mosse = vec![
        Comando::Ferma { dicastero: "lavoro_welfare".into() },
        Comando::Dirotta { dicastero: "sanita".into(), atto: Some("sanita_case_comunita".into()), regione: None },
        Comando::Dirotta { dicastero: "sviluppo_economico".into(), atto: None, regione: Some("calabria".into()) },
        Comando::ChiediDimissioni { dicastero: "ambiente_energia".into() },
    ];
    let mut fatte = Vec::new();
    for c in mosse {
        let testo = format!("{c:?}");
        match p.comanda(c) {
            Ok(()) => fatte.push(serde_json::json!({ "mossa": testo })),
            Err(e) => fatte.push(serde_json::json!({ "mossa": testo, "rifiuto": e.to_string() })),
        }
    }
    note.push(format!("scelte: il 03 e {} mosse", fatte.len()));
    out.insert("scelte".into(), serde_json::json!({ "come": "il 03, poi Ferma, Dirotta l'atto, Dirotta la regione, ChiediDimissioni", "mosse": fatte, "intese": intese(&p), "vista": p.vista() }));

    p.chiudi_turno();
    note.push(format!("interim: turno {}", p.vista().turno));
    out.insert("interim".into(), serde_json::json!({ "come": "le scelte, poi chiudi", "costi": costi(&p), "vista": p.vista() }));
    let rifiuto = p.comanda(Comando::ScegliSuccessore { dicastero: "ambiente_energia".into(), indice: 0 }).err().map(|e| e.to_string());
    out.insert("successore_scelto".into(), serde_json::json!({ "come": "l'interim, poi ScegliSuccessore 0", "rifiuto": rifiuto, "vista": p.vista() }));
    p.chiudi_turno();
    out.insert("rodaggio".into(), serde_json::json!({ "come": "il successore scelto, poi chiudi", "costi": costi(&p), "vista": p.vista() }));

    // Ministri scontenti con l'agenda aperta: almeno tre in minaccia, fuori dalla crisi.
    let scontenti = |v: &sim_core::Vista| {
        !v.crisi
            && (v.ministeri.iter()).filter(|m| m.ministro.as_ref().is_some_and(|x| x.minaccia)).count() >= 3
            && (v.ministeri.iter()).filter(|m| m.agenda.is_some()).count() >= 3
    };
    match cerca_al_governo(d, true, scontenti)? {
        Some((s, q)) => {
            note.push(format!("scontenti: seme {s}, turno {}", q.vista().turno));
            out.insert("scontenti".into(), serde_json::json!({ "come": format!("seme {s}, come il 05, la bozza a ogni bilancio; almeno tre ministri in minaccia con l'agenda aperta"), "costi": costi(&q), "intese": intese(&q), "vista": q.vista() }));
        }
        None => note.push("scontenti: NON RAGGIUNTO".into()),
    }

    // Il tono grave dell'08, coi costi: stessa partita (seme 4, la bozza a ogni bilancio).
    let grave = |v: &sim_core::Vista| (v.eventi.iter()).any(|e| e.tono == sim_core::eventi::Tono::Grave);
    match cerca_al_governo(d, true, grave)? {
        Some((s, q)) => {
            note.push(format!("grave: seme {s}, turno {}", q.vista().turno));
            out.insert("grave".into(), serde_json::json!({ "come": format!("seme {s}, come l'08"), "costi": costi(&q), "intese": intese(&q), "vista": q.vista() }));
        }
        None => note.push("grave: NON RAGGIUNTO".into()),
    }

    // Un tecnico dalla spartizione: quella di default con Giustizia senza partito.
    let mut t = fonda(d, 4, 4.5)?;
    con_la_destra(&mut t)?;
    t.chiudi_turno();
    let mut q = t.clone();
    q.chiudi_turno();
    let dicasteri: Vec<Assegnazione> = (q.vista().ministeri.iter())
        .map(|m| Assegnazione {
            dicastero: m.dicastero.clone(),
            partito: (m.dicastero != "giustizia").then(|| m.ministro.as_ref().map(|x| x.partito.clone())).flatten(),
        })
        .collect();
    match comanda(&mut t, Comando::ProponiSpartizione { dicasteri }) {
        Ok(()) => {
            t.chiudi_turno();
            note.push(format!("tecnico: turno {}", t.vista().turno));
            out.insert("tecnico".into(), serde_json::json!({ "come": "seme 4 a 4,5 con la destra, spartizione di default con Giustizia a un tecnico", "costi": costi(&t), "vista": t.vista() }));
        }
        Err(e) => {
            note.push(format!("tecnico: NON RAGGIUNTO ({e})"));
        }
    }
    Ok(Uscita {
        nome: "18-governo.json",
        json: serde_json::to_string(&out).map_err(|e| e.to_string())?,
        nota: note.join("; "),
    })
}
