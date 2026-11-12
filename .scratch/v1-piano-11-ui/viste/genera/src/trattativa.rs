//! La trattativa (ticket 27): l'incarico col premio (la 02) con proposte di spartizione, i loro
//! rifiuti e l'intesa di partenza dopo la chiusura; l'incarico senza maggioranza con
//! l'allargamento; il ribaltone con la guida; l'incarico al Capo dello Stato. Tutti giocati. Il rimpasto
//! non ha comandi (`partita.rs:3474`): niente stato qui.

use serde_json::Value;
use sim_core::legislatura::{Assegnazione, Fase, Incarico};
use sim_core::{Comando, DatiGioco, Partita};

use super::{Esito, Uscita, con_la_destra, fonda};

macro_rules! js {
    ($x:expr) => {
        serde_json::to_value($x).unwrap_or(Value::Null)
    };
}

fn prova(p: &mut Partita, c: Comando) -> Value {
    match p.comanda(c) {
        Ok(()) => serde_json::json!({"ok": true}),
        Err(e) => serde_json::json!({"rifiuto": e.to_string()}),
    }
}

/// La spartizione che la chiusura applica: per dicastero il partito del ministro, `None` se tecnico.
fn applicata(p: &Partita) -> Vec<(String, Option<String>)> {
    let mut q = p.clone();
    q.chiudi_turno();
    (q.vista().ministeri.iter())
        .map(|m| {
            let partito = (m.ministro.as_ref()).and_then(|x| (!x.tecnico).then(|| x.partito.clone()));
            (m.dicastero.clone(), partito)
        })
        .collect()
}

fn assegnazioni(s: &[(String, Option<String>)]) -> Vec<Assegnazione> {
    (s.iter())
        .map(|(d, p)| Assegnazione {
            dicastero: d.clone(),
            partito: p.clone(),
        })
        .collect()
}

/// La proposta `s` sul clone di `p`: richieste dopo il comando, poi alleati e ministri dopo la chiusura.
fn proposta(p: &Partita, nome: &str, s: &[(String, Option<String>)]) -> Value {
    let mut q = p.clone();
    let esito = prova(
        &mut q,
        Comando::ProponiSpartizione {
            dicasteri: assegnazioni(s),
        },
    );
    let richieste = js!(&q.vista().richieste);
    q.chiudi_turno();
    let v = q.vista();
    serde_json::json!({
        "nome": nome,
        "spartizione": s.iter().map(|(d, p)| serde_json::json!({"dicastero": d, "partito": p})).collect::<Vec<_>>(),
        "esito": esito,
        "richieste": richieste,
        "dopo_chiusura": {"alleati": js!(&v.alleati), "ministeri": js!(&v.ministeri), "fase": js!(&v.fase)},
    })
}

/// Seggi della coalizione del giocatore (con gli allargati) contro la metà delle Camere.
fn maggioranza(p: &Partita) -> bool {
    let l = p.legislatura();
    let g = p.giocatore();
    let mut c = (l.maggioranza.clone())
        .or_else(|| l.coalizioni.iter().find(|c| c.contains(&g)).cloned())
        .unwrap_or_else(|| vec![g]);
    c.extend(&l.allargati);
    let v = p.vista();
    let somma = |f: fn(&sim_core::vista::SeggiPartito) -> u32| -> u32 {
        c.iter().filter_map(|&j| v.seggi.get(j)).map(f).sum()
    };
    somma(|s| s.camera) * 2 > l.totali.0 && somma(|s| s.senato) * 2 > l.totali.1
}

fn incarico_col_premio(d: &DatiGioco) -> Esito<Value> {
    let mut p = fonda(d, 4, 4.5)?;
    con_la_destra(&mut p)?;
    p.chiudi_turno();
    let default = applicata(&p);
    let di = |pred: &dyn Fn(&Option<String>) -> bool| -> Vec<usize> {
        (0..default.len()).filter(|&i| pred(&default[i].1)).collect()
    };
    let miei = di(&|x| x.as_deref() == Some("giocatore"));
    let della_destra = di(&|x| x.as_deref() == Some("destra"));
    let mut proposte = vec![proposta(&p, "default", &default)];
    let pesi = [3, 1, 1, 1, 1, 1, 1, 2, 1, 2]; // ponytail: da dati/base/dicasteri.toml, la Vista non li porta
    // Generosa: un dicastero mio da un punto al Centrodestra.
    if let Some(&i) = miei.iter().find(|&&i| pesi[i] == 1) {
        let mut s = default.clone();
        s[i].1 = Some("centrodestra".into());
        proposte.push(proposta(&p, "generosa_centrodestra", &s));
    }
    // Avara: il dicastero più pesante della Destra a me.
    if let Some(&i) = della_destra.iter().max_by_key(|&&i| pesi[i]) {
        let mut s = default.clone();
        s[i].1 = Some("giocatore".into());
        proposte.push(proposta(&p, "avara_destra", &s));
    }
    // Un tecnico nella mia quota, poi uno al posto di un dicastero della Destra.
    if let Some(&i) = miei.first() {
        let mut s = default.clone();
        s[i].1 = None;
        proposte.push(proposta(&p, "tecnico_mio", &s));
    }
    if let Some(&i) = della_destra.iter().find(|&&i| pesi[i] == 1) {
        let mut s = default.clone();
        s[i].1 = None;
        proposte.push(proposta(&p, "tecnico_della_destra", &s));
    }
    let mut fuori = default.clone();
    fuori[0].1 = Some("centro".into());
    let mut q = p.clone();
    let rifiuto_fuori = prova(&mut q, Comando::ProponiSpartizione { dicasteri: assegnazioni(&fuori) });
    let rifiuto_mancante = prova(
        &mut q,
        Comando::ProponiSpartizione { dicasteri: assegnazioni(&default[1..]) },
    );
    Ok(serde_json::json!({
        "come": "seme 4, fonda a 4,5, coalizione con la destra, chiudi (voto): la 02",
        "pesi": pesi,
        "vista": js!(&p.vista()),
        "proposte": proposte,
        "rifiuto_fuori_coalizione": rifiuto_fuori,
        "rifiuto_mancante": rifiuto_mancante,
    }))
}

fn senza_maggioranza(base: &DatiGioco) -> Esito<Value> {
    let mut scenari = vec![("in vigore".to_string(), base.clone())];
    for nome in base.elezioni().impostazioni.keys() {
        let mut s = sim_core::SuMisura::base(base);
        s.cursori.legge_elettorale.clone_from(nome);
        if let Ok(x) = base.con_su_misura(&s) {
            scenari.push((nome.clone(), x));
        }
    }
    for (legge, d) in &scenari {
    let d = d;
    for seme in 1..=30 {
        for pos in [4.5, 3.5, 5.5, 2.5] {
            for destra in [false, true] {
                let mut p = fonda(d, seme, pos)?;
                if destra && con_la_destra(&mut p).is_err() {
                    continue;
                }
                p.chiudi_turno();
                let l = p.legislatura();
                if l.fase != Fase::Trattativa || l.incarico != Some(Incarico::Giocatore) || maggioranza(&p) {
                    continue;
                }
                let q = &p;
                let default_base = serde_json::json!(applicata(&q).iter().map(|(d, p)| serde_json::json!({"dicastero": d, "partito": p})).collect::<Vec<_>>());
                let disp = p.vista().disponibilita;
                let mut allarga = Vec::new();
                for v in &disp {
                    let mut q = p.clone();
                    let esito = prova(&mut q, Comando::Allarga { partiti: vec![v.partito.clone()] });
                    let dopo = esito.get("ok").map(|_| {
                        serde_json::json!({"maggioranza": maggioranza(&q), "richieste": js!(&q.vista().richieste), "default": serde_json::json!(applicata(&q).iter().map(|(d, p)| serde_json::json!({"dicastero": d, "partito": p})).collect::<Vec<_>>()),
                               "disponibilita": js!(&q.vista().disponibilita)})
                    });
                    allarga.push(serde_json::json!({"partito": v.partito, "esito": esito, "dopo": dopo}));
                }
                let disponibili: Vec<String> =
                    disp.iter().filter(|v| v.disponibile).map(|v| v.partito.clone()).collect();
                let mut q = p.clone();
                let tutti = prova(&mut q, Comando::Allarga { partiti: disponibili.clone() });
                let tutti_vista = js!(&q.vista());
                let tutti_maggioranza = maggioranza(&q);
                let tutti_default = serde_json::json!(applicata(&q).iter().map(|(d, p)| serde_json::json!({"dicastero": d, "partito": p})).collect::<Vec<_>>());
                let mut r = p.clone();
                r.chiudi_turno();
                let rv = r.vista();
                let senza = serde_json::json!({"fase": js!(&rv.fase), "governa": rv.governa,
                    "presidente": js!(&r.governo().presidente), "ministeri": js!(&rv.ministeri)});
                return Ok(serde_json::json!({
                    "come": format!("legge {legge}, seme {seme}, fonda a {pos}, {}chiudi (voto): incarico al giocatore senza maggioranza",
                        if destra { "coalizione con la destra, " } else { "nessuna coalizione, " }),
                    "vista": js!(&p.vista()),
                    "default": default_base,
                    "allarga_uno": allarga,
                    "allarga_tutti": {"partiti": disponibili, "esito": tutti, "maggioranza": tutti_maggioranza, "default": tutti_default, "vista": tutti_vista},
                    "chiusa_senza_allargare": senza,
                }));
            }
        }
    }
    }
    Ok(Value::Null)
}

fn guida(d: &DatiGioco) -> Esito<Value> {
    for pos in [1.5, 2.5, 5.5, 6.5, 3.5] {
        for seme in 1..=40 {
            let mut p = fonda(d, seme, pos)?;
            for _ in 0..40 {
                if p.finita() {
                    break;
                }
                if matches!(p.legislatura().incarico, Some(Incarico::Guida { .. }))
                    && p.legislatura().fase == Fase::Trattativa
                {
                    let mut a = p.clone();
                    let chiedi = prova(&mut a, Comando::ChiediGuida);
                    let dopo_chiedi = js!(&a.vista());
                    let q = &a;
                    let default_chiedi = serde_json::json!(applicata(&q).iter().map(|(d, p)| serde_json::json!({"dicastero": d, "partito": p})).collect::<Vec<_>>());
                    let mut b = p.clone();
                    let rifiuta = prova(&mut b, Comando::RifiutaRibaltone);
                    let dopo_rifiuta = js!(&b.vista());
                    b.chiudi_turno();
                    let bv = b.vista();
                    let mut c = p.clone();
                    c.chiudi_turno();
                    let cv = c.vista();
                    return Ok(serde_json::json!({
                        "come": format!("seme {seme}, fonda a {pos}, nessuna coalizione, turni chiusi senza comandi fino alla guida"),
                        "vista": js!(&p.vista()),
                        "chiedi": chiedi, "dopo_chiedi": dopo_chiedi, "default_chiedi": default_chiedi,
                        "rifiuta": rifiuta, "dopo_rifiuta": dopo_rifiuta,
                        "rifiuta_chiusa": {"fase": js!(&bv.fase), "governa": bv.governa, "incarico": js!(&bv.incarico)},
                        "senza_scelta_chiusa": {"fase": js!(&cv.fase), "governa": cv.governa, "alleati": js!(&cv.alleati),
                            "presidente": js!(&c.governo().presidente)},
                    }));
                }
                p.chiudi_turno();
            }
        }
    }
    Ok(Value::Null)
}

fn capo_dello_stato(d: &DatiGioco) -> Esito<Value> {
    let mut p = fonda(d, 1, 1.5)?;
    p.chiudi_turno();
    Ok(serde_json::json!({"come": "seme 1, fonda a 1,5, nessuna coalizione, chiudi (voto): il 06 un turno prima",
              "vista": js!(&p.vista())}))
}

pub fn trattativa(d: &DatiGioco) -> Esito<Uscita> {
    let out = serde_json::json!({
        "premio": incarico_col_premio(d)?,
        "senza_maggioranza": senza_maggioranza(d)?,
        "guida": guida(d)?,
        "capo_dello_stato": capo_dello_stato(d)?,
    });
    let nota = ["senza_maggioranza", "guida"]
        .iter()
        .map(|k| format!("{k}: {}", if out[k].is_null() { "NON RAGGIUNTO" } else { "ok" }))
        .collect::<Vec<_>>()
        .join("; ");
    Ok(Uscita {
        nome: "27-trattativa.json",
        json: serde_json::to_string(&out).map_err(|e| e.to_string())?,
        nota,
    })
}
