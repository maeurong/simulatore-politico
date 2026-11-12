//! L'eredità (ticket 30): la partita finita senza seggi, con l'eredità in bozza, giocata.
//!
//! La fine senza seggi (`partita.rs:2891-2893`) arriva giocando: senza leggi, 17 partite su 720
//! (semi 1-60, sei posti, con e senza la destra, la bozza a ogni bilancio) finiscono al turno 21,
//! col secondo voto; la nota del 13 provava solo il posto 6,5. Con al più due leggi che passano
//! (`con_leggi`) ne finiscono 6 su 360 senza la destra: questa è quella con più leggi in vigore.

use sim_core::parlamento::ValoreProposto;
use sim_core::{Comando, DatiGioco, Partita};

use super::{Esito, Uscita, comanda, con_la_destra, fonda};

/// Il gradino ordinario più vicino a `verso` che l'anteprima dà approvato anche col minimo della
/// forbice in tutte e due le Camere: fuori dal calendario, non in vigore, non in arrivo.
fn gradino_che_passa(p: &Partita, verso: f64) -> Option<ValoreProposto> {
    let v = p.vista();
    let mut c: Vec<(ValoreProposto, f64)> = (v.disposizioni.iter())
        .filter(|x| matches!(x.norma, sim_core::disposizioni::Norma::Ordinaria))
        .filter(|x| !(v.calendario.iter()).any(|m| m.valori.iter().any(|w| w.disposizione == x.id)))
        .filter(|x| x.in_arrivo.is_empty())
        .flat_map(|x| {
            (x.gradini.iter())
                .filter(|g| g.id != x.in_vigore && !g.bloccato)
                .filter_map(|g| {
                    g.posizione.map(|q| {
                        let w = ValoreProposto {
                            disposizione: x.id.clone(),
                            gradino: g.id.clone(),
                        };
                        (w, (q - verso).abs())
                    })
                })
        })
        .collect();
    c.sort_by(|a, b| a.1.total_cmp(&b.1));
    c.into_iter().map(|(w, _)| w).find(|w| {
        p.anteprima(std::slice::from_ref(w)).is_ok_and(|a| {
            a.forbice.camera[0] >= a.forbice.maggioranza_camera
                && a.forbice.senato[0] >= a.forbice.maggioranza_senato
        })
    })
}

/// Come `giocata`, più una legge (la più vicina al proprio posto che l'anteprima dà approvata) a
/// ogni turno al governo fuori dalla crisi, fino a `MAX_LEGGI`. Le mosse per turno, per riprodurla, e le disposizioni che
/// cambiano legge in ogni chiusura (la Vista dice solo la legge in vigore, non quando è entrata).
fn con_leggi(
    d: &DatiGioco,
    seme: u64,
    posizione: f64,
    destra: bool,
) -> Esito<(Partita, Vec<String>, Vec<serde_json::Value>)> {
    let mut p = fonda(d, seme, posizione)?;
    let mut mosse = Vec::new();
    let mut cambi = Vec::new();
    if destra {
        con_la_destra(&mut p)?;
        mosse.push("turno 0: DichiaraCoalizione con la destra disponibile".to_string());
    }
    while p.vista().turno < 40 && !p.finita() {
        let leggi = mosse.iter().filter(|m| m.contains("PresentaLegge")).count();
        if p.governa() && !p.vista().crisi {
            let t = p.vista().turno;
            if let Some(bilancio) = p.bozza_bilancio() {
                comanda(&mut p, Comando::PresentaBilancio { bilancio })?;
                mosse.push(format!("turno {t}: PresentaBilancio con la bozza"));
            }
            if leggi < MAX_LEGGI {
                if let Some(w) = gradino_che_passa(&p, posizione) {
                    match p.comanda(Comando::PresentaLegge {
                        valori: vec![w.clone()],
                    }) {
                        Ok(()) => mosse.push(format!(
                            "turno {t}: PresentaLegge {}={}",
                            w.disposizione, w.gradino
                        )),
                        Err(e) => mosse.push(format!(
                            "RIFIUTO turno {t}: {}={}: {e}",
                            w.disposizione, w.gradino
                        )),
                    }
                }
            }
        }
        let prima = p.vista();
        p.chiudi_turno();
        let dopo = p.vista();
        for (a, b) in prima.disposizioni.iter().zip(&dopo.disposizioni) {
            if a.legge != b.legge || a.in_vigore != b.in_vigore {
                cambi.push(serde_json::json!({
                    "chiusura": prima.turno, "data": dopo.data, "id": b.id, "nome": b.nome,
                    "legge": b.legge, "da": a.in_vigore, "a": b.in_vigore,
                }));
            }
        }
    }
    Ok((p, mosse, cambi))
}

/// Leggi presentate al più in una partita: con tante leggi approvate il giocatore non perde.
const MAX_LEGGI: usize = 2;

pub(super) fn eredita(d: &DatiGioco) -> Esito<Uscita> {
    let mut prove = Vec::new();
    // ponytail: la ricerca su semi 1-60 e sei posti costa 60 s; il seme trovato è fisso qui.
    for (seme, posizione, destra) in [(43, 4.5, false)] {
        let (p, mosse, cambi) = con_leggi(d, seme, posizione, destra)?;
        let Some(e) = p.eredita() else {
            continue;
        };
        prove.push(format!(
            "seme {seme} a {posizione}: turno {}, {} anni, {} leggi",
            p.vista().turno,
            e.anni_al_governo,
            e.leggi_in_vigore.len(),
        ));
        if e.leggi_in_vigore.is_empty() {
            continue;
        }
        let come = format!(
            "seme {seme}, fonda a {}, {}; la bozza a ogni bilancio e, al governo fuori dalla \
             crisi, la legge più vicina al proprio posto che l'anteprima dà approvata, al più \
             {MAX_LEGGI}; finita senza seggi alla chiusura del turno 20",
            posizione.to_string().replace('.', ","),
            if destra {
                "coalizione con la destra"
            } else {
                "nessuna coalizione"
            }
        );
        let json = serde_json::json!({
            "come": come,
            "mosse": mosse,
            "cambi": cambi,
            "vista": p.vista(),
            "eredita": e,
            "voce_albo": p.voce_albo(),
            "serie": p.serie(),
            "prove": prove,
        });
        return Ok(Uscita {
            nome: "30-eredita.json",
            json: serde_json::to_string_pretty(&json).map_err(|e| e.to_string())?,
            nota: come,
        });
    }
    Err(format!(
        "30-eredita.json: la partita del seme 43 non finisce più senza seggi con leggi: {prove:?}"
    ))
}
