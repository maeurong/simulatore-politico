//! Le Viste campione dei prototipi del piano 11, giocate sui dati veri di `dati/base`: scrive un
//! JSON per stato nella cartella sopra questa e stampa su stderr seme, turno e mosse di ognuno.

use std::path::Path;

use sim_core::eventi::Tono;
use sim_core::legislatura::{Fase, Fondazione, Incarico};
use sim_core::salvataggi::{FORMATO_INDICE, IndicePartita, VocePartita};
use sim_core::{Comando, DatiGioco, FileDati, Partita, VERSIONE_SCHEMA};

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
