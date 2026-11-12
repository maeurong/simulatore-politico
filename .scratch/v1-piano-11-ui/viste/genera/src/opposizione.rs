//! Il cruscotto all'opposizione (ticket 28): sfiducia, referendum, posto riservato, linea, intesa
//! del governo IA, confermativo, tono grave e fine legislatura, giocati sotto un governo IA.

use std::collections::BTreeMap;

use serde_json::{Map, Value};
use sim_core::eventi::Tono;
use sim_core::legislatura::Fase;
use sim_core::parlamento::FormaMisura;
use sim_core::{Comando, DatiGioco, Partita, Vista};

use super::{Esito, TURNI, Uscita, comanda, fonda, gradino_lontano};

type Prova = fn(&Partita, &Vista) -> bool;

struct Trovato {
    seme: u64,
    posizione: f64,
    p: Partita,
}

impl Trovato {
    fn come(&self) -> String {
        format!(
            "seme {}, fonda a {}, nessuna coalizione, nessun comando, {} chiusure",
            self.seme,
            self.posizione.to_string().replace('.', ","),
            self.p.vista().turno
        )
    }
}

fn all_opposizione(p: &Partita) -> bool {
    !p.finita() && p.legislatura().fase == Fase::Ordinaria && !p.governo().al_governo(p.giocatore())
}

fn il_06(d: &DatiGioco) -> Esito<Partita> {
    let mut p = fonda(d, 1, 1.5)?;
    p.chiudi_turno();
    p.chiudi_turno();
    Ok(p)
}

fn del_giocatore(v: &Vista) -> Option<&sim_core::vista::VocePartito> {
    v.partiti.iter().find(|x| x.id == v.giocatore)
}

fn primo_referendabile(v: &Vista, salta: usize) -> Option<String> {
    (v.disposizioni.iter())
        .filter(|x| x.referendabile)
        .nth(salta)
        .map(|x| x.id.clone())
}

fn promuovi(p: &mut Partita, disposizione: Option<String>) -> Result<(), String> {
    let disposizione = disposizione.ok_or("nessuna disposizione referendabile")?;
    comanda(p, Comando::PromuoviReferendum { disposizione })
}

/// Chi la voterebbe (id) e se passa, per la nota su stderr.
fn sintesi(a: &sim_core::vista::AnteprimaSfiducia) -> String {
    let si: Vec<&str> = (a.linee.iter())
        .filter(|l| l.si)
        .map(|l| l.partito.as_str())
        .collect();
    format!("firme {}, sì {si:?}, passa {}", a.firme, a.passa)
}

/// `anteprima_sfiducia` al governo e a ogni dicastero: `{"ok": anteprima}` o `{"rifiuto": testo}`.
fn anteprime_sfiducia(p: &Partita) -> Esito<Value> {
    let mut ministri = Map::new();
    for x in &p.vista().dicasteri {
        ministri.insert(x.id.clone(), esito!(p.anteprima_sfiducia(Some(&x.id))));
    }
    Ok(serde_json::json!({ "governo": esito!(p.anteprima_sfiducia(None)), "ministri": ministri }))
}

/// Una sfiducia che passa sullo stato: al governo prima, poi il primo ministro.
fn sfiducia_che_passa(p: &Partita, v: &Vista) -> Option<Option<String>> {
    if p.anteprima_sfiducia(None).is_ok_and(|a| a.passa) {
        return Some(None);
    }
    (v.dicasteri.iter())
        .find(|x| p.anteprima_sfiducia(Some(&x.id)).is_ok_and(|a| a.passa))
        .map(|x| Some(x.id.clone()))
}

/// Primo turno all'opposizione, per posizione e seme in fila, dove ogni prova vale; turni chiusi
/// senza comandi. ponytail: budget di 20 semi × `TURNI` per posizione, il confermativo che non
/// arriva lo consuma tutto.
fn cerca(d: &DatiGioco, prove: &[(&'static str, Prova)]) -> Esito<BTreeMap<&'static str, Trovato>> {
    let mut trovati = BTreeMap::new();
    for posizione in [1.5, 2.5, 3.5, 4.5, 5.5, 6.5] {
        for seme in 1..=20 {
            let mut p = fonda(d, seme, posizione)?;
            for _ in 0..TURNI {
                if p.finita() {
                    break;
                }
                if all_opposizione(&p) {
                    let v = p.vista();
                    for (k, prova) in prove {
                        if !trovati.contains_key(k) && prova(&p, &v) {
                            let p = p.clone();
                            trovati.insert(*k, Trovato { seme, posizione, p });
                        }
                    }
                    if trovati.len() == prove.len() {
                        return Ok(trovati);
                    }
                }
                p.chiudi_turno();
            }
        }
    }
    Ok(trovati)
}

pub(super) fn opposizione(d: &DatiGioco) -> Esito<Uscita> {
    let mut out = Map::new();
    let mut note = Vec::new();
    let prove: [(&str, Prova); 8] = [
        ("sfiducia_che_passa", |p, v| {
            sfiducia_che_passa(p, v).is_some()
        }),
        ("firme_mancanti", |p, _| {
            p.anteprima_sfiducia(None).is_ok_and(|a| !a.firme)
        }),
        ("referendum_ammesso", |p, v| {
            promuovi(&mut p.clone(), primo_referendabile(v, 0)).is_ok()
        }),
        ("posto_mio", |_, v| v.posto_riservato),
        ("intesa_varia", |_, v| {
            let a = &v.alleati_avversari;
            a.iter().any(|x| x.causa_ultimatum.is_some())
                || a.first()
                    .is_some_and(|f| a.iter().any(|x| x.fascia != f.fascia))
        }),
        ("confermativo", |_, v| v.referendum.iter().any(|r| r.scelta)),
        ("grave", |_, v| {
            v.eventi.iter().any(|e| e.tono == Tono::Grave)
        }),
        ("fine", |_, v| v.turni_al_voto <= 1),
    ];
    let trovati = cerca(d, &prove)?;
    let p06 = il_06(d)?;
    let v06 = p06.vista();
    let c06 = "il 06: seme 1, fonda a 1,5, nessuna coalizione, chiudi ×2";
    let valence = |p: &Partita| del_giocatore(&p.vista()).map_or(f64::NAN, |x| x.valence);
    let percepita =
        |p: &Partita| del_giocatore(&p.vista()).map_or(f64::NAN, |x| x.posizione_percepita);
    let fasce = |p: &Partita| {
        (p.vista().alleati_avversari.into_iter())
            .map(|x| format!("{} {:?}", x.partito, x.fascia))
            .collect::<Vec<_>>()
    };
    let mut senza = p06.clone();
    senza.chiudi_turno();

    // 1. Sfiducia al governo sul 06.
    let anteprime = anteprime_sfiducia(&p06)?;
    let mut q = p06.clone();
    comanda(&mut q, Comando::Sfiducia { dicastero: None })?;
    let dopo_comando = q.vista();
    let primo = v06.dicasteri.first().map(|x| x.id.clone());
    let ripetuta = q
        .comanda(Comando::Sfiducia { dicastero: primo })
        .err()
        .map(|e| e.to_string());
    q.chiudi_turno();
    let v = q.vista();
    let voto = v.esiti.iter().find(|e| e.forma == FormaMisura::Sfiducia);
    let come = format!(
        "{c06}; Sfiducia al governo, poi chiudi. Esito: {}. Valence del giocatore {:.4} prima, {:.4} \
         dopo (senza sfiducia {:.4}); fasce prima {:?}, dopo {:?} (senza sfiducia {:?})",
        voto.map_or("nessun voto".into(), |e| format!(
            "approvata {}, Camera {}-{}, Senato {}-{}",
            e.approvata, e.camera.si, e.camera.no, e.senato.si, e.senato.no
        )),
        valence(&p06),
        valence(&q),
        valence(&senza),
        fasce(&p06),
        fasce(&q),
        fasce(&senza),
    );
    if let Some(a) = p06.anteprima_sfiducia(None).ok().as_ref() {
        note.push(format!("sfiducia: {}", sintesi(a)));
    }
    out.insert(
        "sfiducia".into(),
        serde_json::json!({ "come": come, "anteprime": anteprime, "rifiuto_seconda": ripetuta,
                "prima": v06, "dopo_comando": dopo_comando, "vista": v }),
    );

    // 2. Una sfiducia che passa.
    let passa = match trovati.get("sfiducia_che_passa") {
        Some(t) => {
            let v = t.p.vista();
            let bersaglio = sfiducia_che_passa(&t.p, &v).flatten();
            note.push(format!(
                "sfiducia_che_passa: {}, su {bersaglio:?}",
                t.come()
            ));
            serde_json::json!({ "come": t.come(), "bersaglio": bersaglio, "anteprime": anteprime_sfiducia(&t.p)?, "vista": v })
        }
        None => Value::Null,
    };
    out.insert("sfiducia_che_passa".into(), passa);

    // 3. Firme che non bastano.
    let firme = match trovati.get("firme_mancanti") {
        Some(t) => {
            let rifiuto =
                t.p.clone()
                    .comanda(Comando::Sfiducia { dicastero: None })
                    .err()
                    .map(|e| e.to_string());
            note.push(format!("firme_mancanti: {}, rifiuto {rifiuto:?}", t.come()));
            serde_json::json!({ "come": t.come(), "anteprima": esito!(t.p.anteprima_sfiducia(None)), "rifiuto": rifiuto, "vista": t.p.vista() })
        }
        None => Value::Null,
    };
    out.insert("firme_mancanti".into(), firme);

    // 4. Referendum abrogativo: sul 06 se il core lo ammette, altrimenti al primo turno che sì.
    let rifiuto_06 = promuovi(&mut p06.clone(), primo_referendabile(&v06, 0)).err();
    let (mut r, base) = match (&rifiuto_06, trovati.get("referendum_ammesso")) {
        (None, _) => (p06.clone(), c06.to_string()),
        (Some(_), Some(t)) => (t.p.clone(), t.come()),
        (Some(_), None) => (p06.clone(), c06.to_string()),
    };
    let vr = r.vista();
    let quesito = primo_referendabile(&vr, 0);
    let non_ref = (vr.disposizioni.iter())
        .find(|x| !x.referendabile && x.dicastero.as_deref() == Some("economia_finanze"))
        .map(|x| x.id.clone());
    let rifiuto_fiscale = promuovi(&mut r.clone(), non_ref.clone()).err();
    let prima = r.clone();
    let referendum = if promuovi(&mut r, quesito.clone()).is_ok() {
        // Il primo altro quesito che da solo sarebbe ammesso: il rifiuto è quello del secondo.
        let secondo = (1..vr.disposizioni.len())
            .map(|i| primo_referendabile(&vr, i))
            .find(|x| promuovi(&mut prima.clone(), x.clone()).is_ok())
            .flatten();
        let rifiuto_secondo = promuovi(&mut r.clone(), secondo.clone()).err();
        let dopo_comando = r.vista();
        let mut chiusure = Vec::new();
        for i in 1..=8 {
            r.chiudi_turno();
            let v = r.vista();
            let fatto = !v.esiti_referendum.is_empty() || v.referendum.is_empty();
            if i <= 2 || fatto {
                chiusure.push(serde_json::json!({ "chiusure": i, "vista": v }));
            }
            if fatto || r.finita() {
                break;
            }
        }
        let come =
            format!("{base}; PromuoviReferendum su {quesito:?}, poi chiusure fino all'esito");
        note.push(format!("referendum: {come}; {} stati dopo", chiusure.len()));
        serde_json::json!({ "come": come, "rifiuto_06": rifiuto_06, "prima": vr, "dopo_comando": dopo_comando,
                "rifiuto_secondo": { "disposizione": secondo, "rifiuto": rifiuto_secondo },
                "rifiuto_non_referendabile": { "disposizione": non_ref, "rifiuto": rifiuto_fiscale },
                "chiusure": chiusure })
    } else {
        note.push("referendum: NON RAGGIUNTO".into());
        serde_json::json!({ "rifiuto_06": rifiuto_06, "rifiuto_non_referendabile": { "disposizione": non_ref, "rifiuto": rifiuto_fiscale } })
    };
    out.insert("referendum".into(), referendum);

    // 5. Il posto riservato del giocatore: una legge vicina a lui, come `aula`.
    let posto = match trovati.get("posto_mio") {
        Some(t) => {
            let mut q = t.p.clone();
            let prima = q.vista();
            let proposta = gradino_lontano(&prima, 7.0).ok_or("nessuna proposta")?;
            comanda(
                &mut q,
                Comando::PresentaProposta {
                    valori: vec![proposta.clone()],
                },
            )?;
            let come = format!(
                "{}; PresentaProposta {}={}",
                t.come(),
                proposta.disposizione,
                proposta.gradino
            );
            note.push(format!("posto_mio: {come}"));
            serde_json::json!({ "come": come, "prima": prima, "vista": q.vista() })
        }
        None => Value::Null,
    };
    out.insert("posto_mio".into(), posto);

    // 6. Linea contraria al default sul 06.
    let misura = (v06.calendario.iter())
        .filter(|m| m.promotore.is_none())
        .find_map(|m| {
            Some((
                m.id,
                m.nome.clone(),
                m.linee.iter().find(|l| l.partito == v06.giocatore)?.si,
            ))
        })
        .ok_or("nessuna misura del governo con la linea del giocatore")?;
    let mut q = p06.clone();
    comanda(
        &mut q,
        Comando::Linea {
            misura: misura.0,
            si: Some(!misura.2),
        },
    )?;
    let dopo_comando = q.vista();
    q.chiudi_turno();
    let dopo_chiusura = q.vista();
    let (p1, s1) = (percepita(&q), percepita(&senza));
    // La misura si vota nella chiusura del suo trimestre: una chiusura in più se non è arrivata.
    if !dopo_chiusura.esiti.iter().any(|e| e.misura == misura.0) {
        q.chiudi_turno();
        senza.chiudi_turno();
    }
    let voto = (q.vista().esiti.into_iter()).find(|e| e.misura == misura.0);
    let come = format!(
        "{c06}; Linea {} sulla misura {} «{}» (default {}), poi chiudi fino al voto. Percepita del \
         giocatore {:.4} prima, {p1:.4} dopo una chiusura (senza linea {s1:.4}), {:.4} dopo il voto \
         (senza linea {:.4}); voto: {}",
        if misura.2 { "no" } else { "sì" },
        misura.0,
        misura.1,
        if misura.2 { "sì" } else { "no" },
        percepita(&p06),
        percepita(&q),
        percepita(&senza),
        voto.map_or("nessuno".into(), |e| format!(
            "approvata {}, Camera {}-{}",
            e.approvata, e.camera.si, e.camera.no
        )),
    );
    note.push(format!("linea: {come}"));
    out.insert(
        "linea".into(),
        serde_json::json!({ "come": come, "dopo_comando": dopo_comando,
                            "dopo_chiusura": dopo_chiusura, "vista": q.vista() }),
    );

    // 7-10. Stati solo da guardare.
    for k in ["intesa_varia", "confermativo", "grave", "fine"] {
        let val = match trovati.get(k) {
            Some(t) => {
                let v = t.p.vista();
                let mut come = t.come();
                if k == "intesa_varia" {
                    come = format!("{come}; fasce {:?}", fasce(&t.p));
                }
                let mut j = serde_json::json!({ "come": come, "vista": v });
                if k == "confermativo" {
                    let costituzionale = (v.esiti.iter())
                        .find(|e| e.forma == FormaMisura::Costituzionale && e.approvata)
                        .map(|e| e.misura);
                    let r = costituzionale.map(|misura| {
                        t.p.clone()
                            .comanda(Comando::Confermativo {
                                misura,
                                chiedi: true,
                            })
                            .err()
                            .map(|e| e.to_string())
                    });
                    j = serde_json::json!({ "come": come, "misura": costituzionale, "rifiuto_chiedi": r, "vista": v });
                }
                note.push(format!("{k}: {come}"));
                j
            }
            None => {
                note.push(format!("{k}: NON RAGGIUNTO"));
                Value::Null
            }
        };
        out.insert(k.into(), val);
    }
    for k in ["sfiducia_che_passa", "firme_mancanti", "posto_mio"] {
        if !trovati.contains_key(k) {
            note.push(format!("{k}: NON RAGGIUNTO"));
        }
    }
    Ok(Uscita {
        nome: "28-opposizione.json",
        json: serde_json::to_string(&out).map_err(|e| e.to_string())?,
        nota: note.join("; "),
    })
}
