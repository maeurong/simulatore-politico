//! Le Viste campione dei prototipi del piano 11, giocate sui dati veri di `dati/base`: scrive un
//! JSON per stato nella cartella sopra questa e stampa su stderr seme, turno e mosse di ognuno.

use std::path::Path;

use sim_core::eventi::Tono;
use sim_core::legislatura::{Fase, Fondazione, Incarico};
use sim_core::parlamento::ValoreProposto;
use sim_core::riepilogo::riepilogo;
use sim_core::salvataggi::{FORMATO_INDICE, IndicePartita, VocePartita};
use sim_core::{Comando, DatiGioco, FileDati, Partita, SuMisura, VERSIONE_SCHEMA};

type Esito<T> = Result<T, String>;

const SEMI: std::ops::RangeInclusive<u64> = 1..=60;
const TURNI: u32 = 40;
/// Turni al governo prima dell'abbandono (07): due anni.
const PRIMA_DI_ABBANDONARE: u32 = 8;
const NOMI: [&str; 9] = [
    "01-turno-zero.json",
    "02-trattativa-incarico.json",
    "03-governo-ordinario.json",
    "04-governo-bilancio.json",
    "04-governo-bilancio.bozza.json",
    "05-crisi.json",
    "06-opposizione-governo-ia.json",
    "07-partita-finita-eredita.json",
    "08-tono-grave.json",
];
const DESTRA: [&str; 3] = ["destra", "centrodestra", "estrema_destra"];

struct Uscita {
    nome: &'static str,
    json: String,
    nota: String,
}

fn dati() -> Esito<DatiGioco> {
    let cartella = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../dati/base");
    let file = FileDati::leggi(|nome| {
        std::fs::read_to_string(cartella.join(nome)).map_err(|e| format!("{nome}: {e}"))
    })?;
    DatiGioco::da_testi(&file).map_err(|e| e.to_string())
}

fn fonda(dati: &DatiGioco, seme: u64, posizione: f64) -> Esito<Partita> {
    let f = Fondazione {
        nome: "Partito del giocatore".into(),
        posizione,
    };
    Partita::fonda(dati.clone(), seme, &f).map_err(|e| e.to_string())
}

fn uscita(nome: &'static str, p: &Partita, seme: u64, come: &str) -> Esito<Uscita> {
    let d = p.vista().data;
    Ok(Uscita {
        nome,
        json: serde_json::to_string_pretty(&p.vista()).map_err(|e| e.to_string())?,
        nota: format!(
            "seme {seme}, turno {}, {}T{}: {come}",
            p.vista().turno,
            d.anno,
            d.trimestre
        ),
    })
}

fn manca(uscite: &[Uscita], nome: &str) -> bool {
    !uscite.iter().any(|u| u.nome == nome)
}

fn comanda(p: &mut Partita, c: Comando) -> Esito<()> {
    p.comanda(c).map_err(|e| e.to_string())
}

/// Dichiara la coalizione coi partiti della destra disponibili, come `vinta()` dei test.
fn con_la_destra(p: &mut Partita) -> Esito<()> {
    let partiti = (p.disponibilita().into_iter())
        .filter(|v| v.disponibile && DESTRA.contains(&v.partito.as_str()))
        .map(|v| v.partito)
        .collect();
    comanda(
        p,
        Comando::DichiaraCoalizione {
            partiti,
            cessioni: vec![],
        },
    )
}

/// Il giocatore fonda a 4,5, vota con la destra, prende l'incarico e governa fino a `TURNI`
/// turni, presentando la bozza a ogni turno del bilancio. Riempie gli stati che incontra.
fn al_governo(
    d: &DatiGioco,
    seme: u64,
    uscite: &mut Vec<Uscita>,
    ripieghi: &mut Vec<Uscita>,
) -> Esito<()> {
    let zero = fonda(d, seme, 4.5)?;
    let mut p = zero.clone();
    con_la_destra(&mut p)?;
    p.chiudi_turno();
    let l = p.legislatura();
    if l.fase != Fase::Trattativa || l.incarico != Some(Incarico::Giocatore) {
        return Ok(());
    }
    if manca(uscite, "02-trattativa-incarico.json") {
        uscite.push(uscita(
            "01-turno-zero.json",
            &zero,
            seme,
            "fonda a 4,5, nessun comando",
        )?);
        uscite.push(uscita(
            "02-trattativa-incarico.json",
            &p,
            seme,
            "fonda a 4,5, DichiaraCoalizione con la destra disponibile, chiudi",
        )?);
    }
    let base = "fonda a 4,5, coalizione con la destra, chiudi (voto), chiudi (trattativa senza \
                proposta), poi turni chiusi senza decisioni salvo PresentaBilancio con la bozza";
    p.chiudi_turno();
    let mut turni_al_governo = 0;
    for _ in 0..TURNI {
        if p.finita() {
            break;
        }
        if p.governa() {
            turni_al_governo += 1;
            let v = p.vista();
            let bozza = p.bozza_bilancio();
            let ordinario = !v.crisi && bozza.is_none() && !p.turno_del_bilancio();
            // Crisi senza esercizio provvisorio: con, la caduta è certa e `Allarga` non si offre.
            if v.crisi && !v.bilancio.esercizio_provvisorio && manca(uscite, "05-crisi.json") {
                uscite.push(uscita("05-crisi.json", &p, seme, base)?);
            }
            if v.crisi && manca(ripieghi, "05-crisi.json") {
                let come = format!("{base}; in esercizio provvisorio, caduta certa");
                ripieghi.push(uscita("05-crisi.json", &p, seme, &come)?);
            }
            if ordinario {
                // `Vista.richieste` è delle trattative (partita.rs:3263): quelle dei ministri
                // sono `ministeri[].proposta`.
                if !v.eventi.is_empty()
                    && v.ministeri.iter().any(|m| m.proposta.is_some())
                    && manca(uscite, "03-governo-ordinario.json")
                {
                    let come = format!("{base}; evento aperto e proposte dei ministri");
                    uscite.push(uscita("03-governo-ordinario.json", &p, seme, &come)?);
                }
                if manca(ripieghi, "03-governo-ordinario.json") {
                    ripieghi.push(uscita("03-governo-ordinario.json", &p, seme, base)?);
                }
            }
            if v.eventi.iter().any(|e| e.tono == Tono::Grave) && manca(uscite, "08-tono-grave.json")
            {
                uscite.push(uscita("08-tono-grave.json", &p, seme, base)?);
            }
            if turni_al_governo == PRIMA_DI_ABBANDONARE
                && manca(uscite, "07-partita-finita-eredita.json")
            {
                let mut q = p.clone();
                q.abbandona().map_err(|e| e.to_string())?;
                let come = format!("{base}; abbandona al turno {PRIMA_DI_ABBANDONARE} al governo");
                uscite.push(uscita("07-partita-finita-eredita.json", &q, seme, &come)?);
            }
            if let Some(bilancio) = bozza.filter(|_| !v.crisi) {
                comanda(
                    &mut p,
                    Comando::PresentaBilancio {
                        bilancio: bilancio.clone(),
                    },
                )?;
                if p.turno_del_bilancio() && manca(uscite, "04-governo-bilancio.json") {
                    let come = format!("{base}; PresentaBilancio con la bozza, prima di chiudere");
                    uscite.push(uscita("04-governo-bilancio.json", &p, seme, &come)?);
                    uscite.push(Uscita {
                        nome: "04-governo-bilancio.bozza.json",
                        json: serde_json::to_string_pretty(&bilancio).map_err(|e| e.to_string())?,
                        nota: format!("seme {seme}: bozza_bilancio() dello stesso turno"),
                    });
                }
            }
        }
        p.chiudi_turno();
    }
    Ok(())
}

/// Una partita giocata come in `al_governo` (coalizione con la destra se `destra`, la bozza a
/// ogni bilancio fuori dalla crisi), fino al turno `fino` o alla fine.
fn giocata(
    d: &DatiGioco,
    seme: u64,
    posizione: f64,
    nome: &str,
    destra: bool,
    fino: u32,
) -> Esito<Partita> {
    let f = Fondazione {
        nome: nome.into(),
        posizione,
    };
    let mut p = Partita::fonda(d.clone(), seme, &f).map_err(|e| e.to_string())?;
    if destra {
        con_la_destra(&mut p)?;
    }
    while p.vista().turno < fino && !p.finita() {
        if p.governa() && !p.vista().crisi {
            if let Some(bilancio) = p.bozza_bilancio() {
                comanda(&mut p, Comando::PresentaBilancio { bilancio })?;
            }
        }
        p.chiudi_turno();
    }
    Ok(p)
}

fn voce(p: &Partita, id: &str, ultimo_salvataggio: u64) -> VocePartita {
    VocePartita {
        indice: IndicePartita {
            formato_indice: FORMATO_INDICE,
            id: id.into(),
            testata: p.testata(),
            ultimo_salvataggio,
            riavvolgimenti: p.riavvolgimenti(),
        },
        apribile: true,
        motivo: None,
    }
}

/// L'edicola (ticket 13): `partite()` e `albo()` del guscio, ricostruiti da `testata()` e
/// `voce_albo()` di partite giocate. Nomi dei partiti diversi, perché l'elenco li distingua.
fn edicola(d: &DatiGioco) -> Esito<Uscita> {
    // ponytail: orari fissi (10/11/2026 sera e prima), il file resta deterministico.
    let ore = 1_794_000_000;
    let governo = giocata(d, 4, 4.5, "Patto Civico", true, 6)?;
    let opposizione = giocata(d, 1, 1.5, "Fronte dei Comuni", false, 5)?;
    let crisi = giocata(d, 9, 4.5, "Nuova Concordia", true, 34)?;
    let mut vecchia = voce(
        &giocata(d, 2, 5.5, "Unione dei Pendolari", false, 3)?,
        "unione-dei-pendolari",
        ore - 9 * 86_400,
    );
    // Forzata: lo schema 16 di un salvataggio di prima delle #51-#52 (archivio.rs:98-103).
    vecchia.indice.testata.versione_schema = VERSIONE_SCHEMA - 1;
    vecchia.apribile = false;
    vecchia.motivo = Some(
        sim_core::Errore::SchemaDiverso {
            trovata: VERSIONE_SCHEMA - 1,
            attesa: VERSIONE_SCHEMA,
        }
        .to_string(),
    );
    // Forzata: senza seggi in tutte e due le Camere (partita.rs:2891-2893) non si raggiunge coi
    // semi 1-60 a 6,5; la partita finita con l'eredità in bozza resta fra le partite in corso.
    let mut senza_seggi = voce(
        &giocata(d, 3, 6.5, "Lista del Faro", false, 1)?,
        "lista-del-faro",
        ore - 86_400,
    );
    senza_seggi.indice.testata.finita = true;
    let partite = vec![
        voce(&governo, "patto-civico", ore),
        senza_seggi,
        voce(&crisi, "nuova-concordia", ore - 2 * 86_400 - 3_600),
        voce(&opposizione, "fronte-dei-comuni", ore - 5 * 86_400),
        vecchia,
    ];
    let mut albo = Vec::new();
    for (seme, pos, nome, destra, fino) in [
        (4, 4.5, "Partito del giocatore", true, 10),
        (9, 4.5, "Concordia", true, 34),
        (1, 1.5, "Fronte Popolare dei Borghi", false, 12),
    ] {
        let mut p = giocata(d, seme, pos, nome, destra, fino)?;
        if !p.finita() {
            p.abbandona().map_err(|e| e.to_string())?;
        }
        albo.extend(p.voce_albo());
    }
    let json = serde_json::json!({ "partite": partite, "albo": albo });
    Ok(Uscita {
        nome: "13-edicola.json",
        json: serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?,
        nota: format!(
            "partite: Patto Civico s4 t6, Nuova Concordia s9 t34, Fronte dei Comuni s1 t5, \
             Unione dei Pendolari s2 t3 con schema forzato a {}, Lista del Faro s3 t1 con \
             `finita` forzata; albo: abbandoni s4 t10, s9 t34, s1 t12",
            VERSIONE_SCHEMA - 1
        ),
    })
}

/// Riepilogo di uno scenario su misura, o l'errore di `con_su_misura` (rifiuto sul posto).
fn riepilogo_di(d: &DatiGioco, etichetta: &str, s: &SuMisura) -> serde_json::Value {
    let t = std::time::Instant::now();
    let r = d.con_su_misura(s).map(|x| riepilogo(&x));
    let ms = t.elapsed().as_millis();
    eprintln!("14 «{etichetta}»: {ms} ms");
    match r {
        Ok(r) => serde_json::json!({ "etichetta": etichetta, "scenario": s, "riepilogo": r }),
        Err(e) => {
            serde_json::json!({ "etichetta": etichetta, "scenario": s, "errore": e.to_string() })
        }
    }
}

/// Ciò che l'editor deve sapere prima di ogni riepilogo e che nessun comando del guscio porta:
/// intervalli dei sei cursori, impostazioni elettorali, disposizioni coi gradini.
fn editor(d: &DatiGioco) -> Esito<Uscita> {
    let sc = d.scenario();
    let c = &sc.cursori;
    let ingresso = |nome: &str, unita: &str, i: &sim_core::scenario::Ingresso| serde_json::json!({ "nome": nome, "unita": unita, "valore": i.valore, "min": i.min, "max": i.max });
    let cursori = vec![
        ingresso("debito_pil", "% del PIL", &sc.debito_pil),
        ingresso("disoccupazione", "%", &sc.disoccupazione),
        ingresso("inflazione", "% annua", &sc.inflazione),
        ingresso("ricchezza", "± % PIL pro capite", &c.ricchezza),
        ingresso("eta_mediana", "± anni", &c.eta_mediana),
        ingresso("orientamento", "± punti 1-7", &c.orientamento),
    ];
    let dicastero = |id: &Option<String>| {
        id.as_ref()
            .and_then(|i| d.dicasteri().iter().find(|x| &x.id == i))
            .map(|x| x.nome.clone())
    };
    let disposizioni: Vec<_> = (d.disposizioni().iter())
        .map(|x| {
            serde_json::json!({
                "id": x.id, "nome": x.nome, "dicastero": dicastero(&x.dicastero), "norma": x.norma,
                "partenza": x.partenza,
                "gradini": x.gradini.iter().map(|g| serde_json::json!({
                    "id": g.id, "nome": g.nome, "saldo": g.saldo })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let base = SuMisura::base(d);
    // Per legge: i gradini delle nove disposizioni elettorali, letti dai dati che il core ne fa.
    let mut leggi = serde_json::Map::new();
    for nome in d.elezioni().impostazioni.keys() {
        let mut s = base.clone();
        s.cursori.legge_elettorale.clone_from(nome);
        let x = d.con_su_misura(&s).map_err(|e| e.to_string())?;
        let g: serde_json::Map<_, _> = (x.disposizioni().iter())
            .filter(|x| x.norma == sim_core::disposizioni::Norma::Elettorale)
            .map(|x| (x.id.clone(), x.partenza.clone().into()))
            .collect();
        leggi.insert(nome.clone(), g.into());
    }
    let con = |nome: &str, f: &dyn Fn(&mut SuMisura)| {
        let mut s = base.clone();
        s.nome = nome.into();
        f(&mut s);
        s
    };
    let prove = [
        ("base", base.clone()),
        (
            "debito al tetto",
            con("Debito al tetto", &|s| s.cursori.debito_pil = 160.0),
        ),
        (
            "crisi",
            con("Crisi", &|s| {
                s.cursori.debito_pil = 160.0;
                s.cursori.inflazione = 8.0;
                s.cursori.disoccupazione = 11.6;
                s.cursori.ricchezza = -20.0;
            }),
        ),
        (
            "elettorato a sinistra",
            con("Elettorato a sinistra", &|s| s.cursori.orientamento = -1.0),
        ),
        (
            "elettorato a destra",
            con("Elettorato a destra", &|s| s.cursori.orientamento = 1.0),
        ),
        (
            "proporzionale puro",
            con("Proporzionale puro", &|s| {
                s.cursori.legge_elettorale = "proporzionale_puro".into();
            }),
        ),
        (
            "detrazione al 110",
            con("Superbonus", &|s| {
                s.disposizioni
                    .insert("detrazione_edilizia".into(), "110".into());
            }),
        ),
        (
            "legge elettorale incoerente",
            con("Incoerente", &|s| {
                // Premio alla coalizione senza coalizioni.
                s.disposizioni.insert("coalizioni".into(), "no".into());
            }),
        ),
    ];
    let riepiloghi: Vec<_> = prove.iter().map(|(e, s)| riepilogo_di(d, e, s)).collect();
    // ponytail: nove passi per cursore, uno alla volta dalla base; i cursori mossi insieme il
    // prototipo li approssima col passo più vicino e lo dice.
    let mut griglia = serde_json::Map::new();
    for c in &cursori {
        let nome = c["nome"].as_str().unwrap_or_default().to_string();
        let (lo, hi) = (
            c["min"].as_f64().unwrap_or(0.0),
            c["max"].as_f64().unwrap_or(0.0),
        );
        let passi: Vec<_> = (0..9)
            .map(|k| {
                let v = lo + (hi - lo) * f64::from(k) / 8.0;
                let mut s = base.clone();
                let x = &mut s.cursori;
                match nome.as_str() {
                    "debito_pil" => x.debito_pil = v,
                    "disoccupazione" => x.disoccupazione = v,
                    "inflazione" => x.inflazione = v,
                    "ricchezza" => x.ricchezza = v,
                    "eta_mediana" => x.eta_mediana = v,
                    _ => x.orientamento = v,
                }
                let r = d
                    .con_su_misura(&s)
                    .map(|x| riepilogo(&x))
                    .map_err(|e| e.to_string());
                r.map(|r| serde_json::json!({ "valore": v, "riepilogo": r }))
            })
            .collect::<Esito<_>>()?;
        griglia.insert(nome, passi.into());
    }
    let json = serde_json::json!({
        "cursori": cursori, "leggi_elettorali": leggi, "legge_di_base": sc.legge_elettorale,
        "disposizioni": disposizioni,
        "scenari_salvati": ["Debito al tetto", "Elettorato a destra", "Proporzionale puro", "Superbonus"],
        "riepiloghi": riepiloghi, "griglia": griglia,
    });
    Ok(Uscita {
        nome: "14-editor.json",
        json: serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?,
        nota: format!(
            "{} riepiloghi di prova e nove passi per cursore; scenari salvati: quattro prove",
            prove.len()
        ),
    })
}

/// `anteprima_fondazione` vera: tre semi sull'Italia di oggi (quanto rumore fa il sondaggio) e una
/// sull'elettorato a destra; accanto, i partiti dello scenario coi loro posti sull'asse.
fn fondazione(d: &DatiGioco) -> Esito<Uscita> {
    let partiti: Vec<_> = (d.scenario().partiti.iter())
        .map(|p| serde_json::json!({ "id": p.id, "nome": p.nome, "posizione": p.posizione }))
        .collect();
    let mut destra = SuMisura::base(d);
    destra.nome = "Elettorato a destra".into();
    destra.cursori.orientamento = 1.0;
    let destra = d.con_su_misura(&destra).map_err(|e| e.to_string())?;
    let mut anteprime = serde_json::Map::new();
    let mut ms = Vec::new();
    for (etichetta, dati, seme) in [
        ("oggi, seme 4", d, 4),
        ("oggi, seme 1", d, 1),
        ("oggi, seme 9", d, 9),
        ("elettorato a destra, seme 4", &destra, 4),
    ] {
        let t = std::time::Instant::now();
        let a = Partita::anteprima_fondazione(dati, seme);
        ms.push(t.elapsed().as_millis());
        anteprime.insert(
            etichetta.into(),
            serde_json::to_value(&a).map_err(|e| e.to_string())?,
        );
    }
    let json = serde_json::json!({ "partiti": partiti, "anteprime": anteprime });
    Ok(Uscita {
        nome: "15-fondazione.json",
        json: serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?,
        nota: format!("quattro anteprime, ms {ms:?}"),
    })
}

/// Il governo del 03 (seme 4 a 4,5, coalizione con la destra, due chiusure).
fn governo_03(d: &DatiGioco) -> Esito<Partita> {
    let mut p = fonda(d, 4, 4.5)?;
    con_la_destra(&mut p)?;
    p.chiudi_turno();
    p.chiudi_turno();
    Ok(p)
}

/// Primo turno al governo, sui semi in fila, dove `cerca` vale; `bilancio` presenta la bozza a
/// ogni turno che la offre, altrimenti la salta (esercizio provvisorio).
fn cerca_al_governo(
    d: &DatiGioco,
    bilancio: bool,
    cerca: impl Fn(&sim_core::Vista) -> bool,
) -> Esito<Option<(u64, Partita)>> {
    for seme in SEMI {
        let mut p = fonda(d, seme, 4.5)?;
        con_la_destra(&mut p)?;
        p.chiudi_turno();
        p.chiudi_turno();
        for _ in 0..TURNI {
            if p.finita() {
                break;
            }
            if p.governa() {
                let v = p.vista();
                if cerca(&v) {
                    return Ok(Some((seme, p)));
                }
                if let Some(b) = p.bozza_bilancio().filter(|_| bilancio && !v.crisi) {
                    comanda(&mut p, Comando::PresentaBilancio { bilancio: b })?;
                }
            }
            p.chiudi_turno();
        }
    }
    Ok(None)
}

/// Gli stati del cruscotto al governo che le Viste 01-08 non hanno: ultimatum, esercizio
/// provvisorio (anche in crisi), dimissioni annunciate, proposta rinviata, e l'anteprima di
/// `Accogli` sulla proposta di fondi del 03.
fn cruscotto(d: &DatiGioco) -> Esito<Uscita> {
    let mut out = serde_json::Map::new();
    let note = std::cell::RefCell::new(Vec::new());
    let metti_nota = |chiave: &str| note.borrow_mut().push(format!("{chiave}: NON RAGGIUNTO"));
    let mut metti = |chiave: &str, p: &Partita, come: String| -> Esito<()> {
        let v = p.vista();
        note.borrow_mut().push(format!(
            "{chiave}: {come}, turno {} {}T{}",
            v.turno, v.data.anno, v.data.trimestre
        ));
        let val = serde_json::json!({ "come": come, "vista": v });
        out.insert(chiave.into(), val);
        Ok(())
    };
    let p03 = governo_03(d)?;
    let anteprima =
        (p03.anteprima_accogli("infrastrutture_trasporti")).map_err(|e| e.to_string())?;
    let mut q = p03.clone();
    comanda(&mut q, Comando::Dimettiti)?;
    metti("dimissioni", &q, "il 03, poi Dimettiti".into())?;
    let mut q = p03.clone();
    comanda(
        &mut q,
        Comando::Rinvia {
            dicastero: "ambiente_energia".into(),
        },
    )?;
    comanda(
        &mut q,
        Comando::RispondiEvento {
            evento: 0,
            opzione: "debito".into(),
        },
    )?;
    metti(
        "scelte_fatte",
        &q,
        "il 03, poi Rinvia su Ambiente e RispondiEvento «debito»".into(),
    )?;
    type Cerca = fn(&sim_core::Vista) -> bool;
    let cerche: [(&str, bool, Cerca); 3] = [
        ("ultimatum", true, |v| {
            !v.crisi && v.alleati.iter().any(|a| a.ultimatum)
        }),
        ("esercizio_provvisorio", false, |v| {
            !v.crisi && v.bilancio.esercizio_provvisorio
        }),
        ("crisi_provvisorio", false, |v| {
            v.crisi && v.bilancio.esercizio_provvisorio
        }),
    ];
    for (chiave, bilancio, cerca) in cerche {
        match cerca_al_governo(d, bilancio, cerca)? {
            Some((s, p)) => {
                let come = if bilancio {
                    "la bozza a ogni bilancio"
                } else {
                    "bilancio mai presentato"
                };
                metti(chiave, &p, format!("seme {s}, come il 05, {come}"))?;
            }
            None => metti_nota(chiave),
        }
    }
    out.insert(
        "anteprima_accogli_03".into(),
        serde_json::to_value(&anteprima).map_err(|e| e.to_string())?,
    );
    Ok(Uscita {
        nome: "17-cruscotto.json",
        json: serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?,
        nota: note.into_inner().join("; "),
    })
}

/// Il gradino più lontano da `verso` fra quelli non in vigore delle disposizioni ordinarie.
fn gradino_lontano(v: &sim_core::Vista, verso: f64) -> Option<ValoreProposto> {
    (v.disposizioni.iter())
        .filter(|x| matches!(x.norma, sim_core::disposizioni::Norma::Ordinaria))
        .filter(|x| !(v.calendario.iter()).any(|m| m.valori.iter().any(|w| w.disposizione == x.id)))
        .flat_map(|x| {
            (x.gradini.iter())
                .filter(|g| g.id != x.in_vigore && !g.bloccato && x.in_arrivo.is_empty())
                .filter_map(|g| g.posizione.map(|p| (x.id.clone(), g.id.clone(), p)))
        })
        .max_by(|a, b| (a.2 - verso).abs().total_cmp(&(b.2 - verso).abs()))
        .map(|(disposizione, gradino, _)| ValoreProposto {
            disposizione,
            gradino,
        })
}

fn valori(coppie: &[(&str, &str)]) -> Vec<ValoreProposto> {
    (coppie.iter())
        .map(|(d, g)| ValoreProposto {
            disposizione: (*d).into(),
            gradino: (*g).into(),
        })
        .collect()
}

/// Gli stati dell'Aula che le Viste 01-08 non hanno: calendario pieno con legge elettorale,
/// norma costituzionale e riforma con emendamenti degli alleati; le deliberazioni e i voti dopo;
/// all'opposizione il posto riservato libero, poi occupato dalla proposta del giocatore, e una
/// linea scelta. Più le anteprime della legge elettorale e della riforma.
fn aula(d: &DatiGioco) -> Esito<Uscita> {
    let mut out = serde_json::Map::new();
    let mut note = Vec::new();
    let mut metti = |out: &mut serde_json::Map<String, serde_json::Value>,
                     chiave: &str,
                     p: &Partita,
                     come: String| {
        let v = p.vista();
        note.push(format!(
            "{chiave}: {come}, turno {} {}T{}",
            v.turno, v.data.anno, v.data.trimestre
        ));
        out.insert(
            chiave.into(),
            serde_json::json!({ "come": come, "vista": v }),
        );
    };
    // L'impostazione pronta «proporzionale al 3%» (`leggi_elettorali` dei dati), meno ciò che è già in vigore.
    let elettorale = valori(&[
        ("coalizioni", "no"),
        ("soglia_coalizione", "0"),
        ("ripescaggio", "no"),
        ("premio", "nessuno"),
        ("soglia_premio", "4000"),
        ("taglia_premio", "875"),
        ("tetto", "nessuno"),
    ]);
    let p03 = governo_03(d)?;
    // La norma costituzionale con più sì attesi alla Camera, perché arrivi alla seconda.
    let costituzionale = [
        ("durata_legislatura", "4"),
        ("durata_legislatura", "6"),
        ("numero_parlamentari", "300_150"),
        ("numero_parlamentari", "630_315"),
    ]
    .into_iter()
    .map(|c| valori(&[c]))
    .max_by_key(|v| p03.anteprima(v).map(|a| a.forbice.camera[0]).unwrap_or(0))
    .ok_or("nessuna costituzionale")?;
    let riforma = gradino_lontano(&p03.vista(), 5.5).ok_or("nessun gradino lontano")?;
    let ant_el = p03.anteprima(&elettorale).map_err(|e| e.to_string())?;
    let ant_ri = (p03.anteprima(std::slice::from_ref(&riforma))).map_err(|e| e.to_string())?;
    let mut p = p03.clone();
    for (nome, v) in [
        ("elettorale", &elettorale),
        ("costituzionale", &costituzionale),
        ("riforma", &vec![riforma.clone()]),
    ] {
        comanda(&mut p, Comando::PresentaLegge { valori: v.clone() })
            .map_err(|e| format!("PresentaLegge {nome}: {e}"))?;
    }
    let rifiuto = p
        .comanda(Comando::PresentaLegge {
            valori: valori(&[("durata_legislatura", "6")]),
        })
        .err()
        .map(|e| e.to_string());
    let base = format!(
        "il 03, poi PresentaLegge proporzionale al 3%, {}={}, {}={}",
        costituzionale[0].disposizione,
        costituzionale[0].gradino,
        riforma.disposizione,
        riforma.gradino
    );
    metti(&mut out, "piena", &p, base.clone());
    // Turni dopo: bilancio presentato quando c'è, emendamenti lasciati al default.
    let mut q = p.clone();
    let (mut dopo_voto, mut seconda) = (false, false);
    for i in 1..=8 {
        q.chiudi_turno();
        if q.finita() || !q.governa() {
            break;
        }
        let v = q.vista();
        if !dopo_voto && !v.esiti.is_empty() {
            dopo_voto = true;
            metti(
                &mut out,
                "dopo_voto",
                &q,
                format!("{base}, poi {i} chiusure"),
            );
        }
        let c = sim_core::parlamento::FormaMisura::Costituzionale;
        if !seconda
            && v.calendario
                .iter()
                .any(|m| m.forma == c && m.deliberazione == 2)
        {
            seconda = true;
            metti(
                &mut out,
                "seconda_deliberazione",
                &q,
                format!("{base}, poi {i} chiusure"),
            );
        }
        let el = sim_core::parlamento::FormaMisura::Elettorale;
        if !out.contains_key("dopo_elettorale")
            && v.esiti.iter().any(|e| e.forma == el && e.approvata)
        {
            metti(
                &mut out,
                "dopo_elettorale",
                &q,
                format!("{base}, poi {i} chiusure"),
            );
        }
        if !v.referendum.is_empty() {
            metti(
                &mut out,
                "referendum",
                &q,
                format!("{base}, poi {i} chiusure"),
            );
            break;
        }
        if let Some(b) = q.bozza_bilancio() {
            comanda(&mut q, Comando::PresentaBilancio { bilancio: b })?;
        }
    }
    // All'opposizione: un seme a 1,5 col posto riservato libero.
    for seme in SEMI {
        let mut p = fonda(d, seme, 1.5)?;
        p.chiudi_turno();
        p.chiudi_turno();
        let v = p.vista();
        if p.legislatura().fase != Fase::Ordinaria || p.governa() || !v.posto_riservato {
            continue;
        }
        let come = format!("seme {seme}, fonda a 1,5, nessuna coalizione, chiudi ×2");
        metti(&mut out, "opposizione_libero", &p, come.clone());
        let proposta = gradino_lontano(&v, 7.0).ok_or("nessuna proposta")?;
        let ant = p.anteprima(std::slice::from_ref(&proposta));
        comanda(
            &mut p,
            Comando::PresentaProposta {
                valori: vec![proposta.clone()],
            },
        )?;
        if let Some(m) = p.vista().calendario.iter().find(|m| m.promotore.is_none()) {
            comanda(
                &mut p,
                Comando::Linea {
                    misura: m.id,
                    si: Some(false),
                },
            )?;
        }
        let come = format!(
            "{come}, poi PresentaProposta {}={} e Linea no sulla prima del governo",
            proposta.disposizione, proposta.gradino
        );
        metti(&mut out, "opposizione_proposta", &p, come);
        match ant {
            Ok(a) => out.insert(
                "anteprima_proposta".into(),
                serde_json::to_value(&a).map_err(|e| e.to_string())?,
            ),
            Err(e) => out.insert("anteprima_proposta_rifiuto".into(), e.to_string().into()),
        };
        break;
    }
    for (chiave, a) in [
        ("anteprima_elettorale", &ant_el),
        ("anteprima_riforma", &ant_ri),
    ] {
        out.insert(
            chiave.into(),
            serde_json::to_value(a).map_err(|e| e.to_string())?,
        );
    }
    out.insert(
        "rifiuto_calendario_costituzionale".into(),
        serde_json::json!(rifiuto),
    );
    for k in [
        "dopo_voto",
        "seconda_deliberazione",
        "dopo_elettorale",
        "referendum",
        "opposizione_libero",
    ] {
        if !out.contains_key(k) {
            note.push(format!("{k}: NON RAGGIUNTO"));
        }
    }
    Ok(Uscita {
        nome: "19-aula.json",
        json: serde_json::to_string_pretty(&out).map_err(|e| e.to_string())?,
        nota: note.join("; "),
    })
}

/// Perché «avanti fino a novità» si fermerebbe sul turno appena aperto, con le regole della spec
/// di core «esito e Vista completa» (sezione «Fermate»). ponytail: senza la fermata `voto`, che
/// chiede la quota degli elettori toccati sul sondaggio; la porterà il core.
fn fermate(v: &sim_core::Vista, prima: &sim_core::Vista) -> Vec<String> {
    let mut f = Vec::new();
    if v.governa {
        f.push("governo".into());
    }
    if v.finita {
        f.push("fine".into());
    }
    for a in &v.alleati_avversari {
        let gia = (prima.alleati_avversari.iter())
            .any(|b| b.partito == a.partito && b.causa_ultimatum.is_some());
        if a.causa_ultimatum.is_some() && !gia {
            f.push(format!("ultimatum:{}", a.partito));
        }
    }
    if v.crisi && !prima.crisi {
        f.push("crisi".into());
    }
    if v.data.trimestre == 3 && v.referendum.is_empty() {
        f.push("referendum".into());
    }
    if v.campagna.attiva {
        f.push("campagna".into());
    }
    if v.fase == Fase::Trattativa && v.incarico.is_some() {
        f.push("incarico".into());
    }
    if v.posto_riservato {
        f.push("posto_riservato".into());
    }
    f
}

/// Gli stati della prima pagina che le Viste 01-08 non hanno: il turno 3 del seme 4 (fra 03 e 04),
/// il voto col giocatore all'opposizione (seme 1 a 1,5), il voto del seme 9 a 4,5, e le corse di
/// «avanti fino a novità» all'opposizione dal 06 in poi, con la Vista prima e dopo ogni fermata.
fn prima_pagina(d: &DatiGioco) -> Esito<Uscita> {
    let mut out = serde_json::Map::new();
    let mut note = Vec::new();
    let mut metti = |k: &str, come: String, v: &sim_core::Vista| -> Esito<()> {
        let j = serde_json::to_value(v).map_err(|e| e.to_string())?;
        out.insert(k.into(), serde_json::json!({"come": come, "vista": j}));
        Ok(())
    };
    let mut p = governo_03(d)?;
    p.chiudi_turno();
    metti("t3", "seme 4: come 03, poi chiudi senza decisioni".into(), &p.vista())?;

    let mut p = fonda(d, 9, 4.5)?;
    con_la_destra(&mut p)?;
    metti("zero_9", "seme 9, 4,5: coalizione con la destra, prima di chiudere".into(), &p.vista())?;
    p.chiudi_turno();
    metti("voto_9", "seme 9, 4,5: coalizione con la destra, chiudi (voto)".into(), &p.vista())?;

    let mut p = fonda(d, 1, 1.5)?;
    p.chiudi_turno();
    metti("voto_opp", "seme 1, 1,5: nessuna coalizione, chiudi (voto)".into(), &p.vista())?;
    p.chiudi_turno();
    // Dal 06: «avanti fino a novità» premuto di nuovo a ogni fermata, per 24 turni.
    let mut corse = Vec::new();
    let mut da = p.vista().turno;
    for _ in 0..24 {
        if p.finita() {
            break;
        }
        let ultima = p.vista();
        p.chiudi_turno();
        let v = p.vista();
        let f = fermate(&v, &ultima);
        if !f.is_empty() {
            let k = format!("fermata_{}", v.turno);
            corse.push(serde_json::json!({"da": da, "a": v.turno, "fermate": f, "chiave": k}));
            note.push(format!("{}→{} {:?}", da, v.turno, f));
            // ponytail: tiene solo la corsa più lunga (≥ 3 turni), il primo ultimatum e la campagna.
            let tieni = v.turno - da >= 3
                || f.iter().any(|x| x == "campagna")
                || (f.iter().any(|x| x.starts_with("ultimatum")) && !out.keys().any(|k| k.starts_with("fermata")));
            if tieni {
                let j = serde_json::json!({"prima": serde_json::to_value(&ultima).map_err(|e| e.to_string())?,
                    "vista": serde_json::to_value(&v).map_err(|e| e.to_string())?});
                out.insert(k, j);
            }
            da = v.turno;
        }
    }
    out.insert("corse".into(), serde_json::Value::Array(corse));
    Ok(Uscita {
        nome: "16-prima-pagina.json",
        json: serde_json::to_string(&out).map_err(|e| e.to_string())?,
        nota: format!("t3, voto_9, voto_opp, corse dal 06: {}", note.join("; ")),
    })
}

fn esegui() -> Esito<()> {
    let d = dati()?;
    let mut uscite = Vec::new();
    let mut ripieghi = Vec::new();
    // ponytail: semi in fila finché ogni stato ha il suo; ogni file resta riproducibile dal suo seme.
    for seme in SEMI {
        al_governo(&d, seme, &mut uscite, &mut ripieghi)?;
        // Il 06 ha la sua partita, sotto.
        if !(NOMI.iter()).any(|n| !n.starts_with("06") && manca(&uscite, n)) {
            break;
        }
    }
    for r in ripieghi {
        if manca(&uscite, r.nome) {
            uscite.push(r);
        }
    }
    for seme in SEMI {
        let mut p = fonda(&d, seme, 1.5)?;
        p.chiudi_turno();
        p.chiudi_turno();
        if p.legislatura().fase == Fase::Ordinaria && !p.governo().al_governo(p.giocatore()) {
            let come = "fonda a 1,5, nessuna coalizione, chiudi (voto), chiudi (trattativa IA)";
            uscite.push(uscita("06-opposizione-governo-ia.json", &p, seme, come)?);
            break;
        }
    }
    uscite.push(edicola(&d)?);
    uscite.push(editor(&d)?);
    uscite.push(fondazione(&d)?);
    uscite.push(cruscotto(&d)?);
    uscite.push(aula(&d)?);
    uscite.push(prima_pagina(&d)?);
    uscite.sort_by_key(|u| u.nome);
    let cartella = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for u in &uscite {
        std::fs::write(cartella.join(u.nome), &u.json).map_err(|e| format!("{}: {e}", u.nome))?;
        eprintln!("{}: {} ({} byte)", u.nome, u.nota, u.json.len());
    }
    eprintln!("{} file scritti", uscite.len());
    for nome in NOMI.iter().filter(|n| manca(&uscite, n)) {
        eprintln!("NON RAGGIUNTO: {nome}");
    }
    Ok(())
}

fn main() -> std::process::ExitCode {
    match esegui() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("errore: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
