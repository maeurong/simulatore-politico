//! Agenda dei ministri: punteggio degli atti, regione bersaglio, esecuzione e cantieri, e i
//! comandi che la fermano o la dirottano.

use super::soddisfazione::registra;
use super::{Causa, Contesto, Ministri, Ministro, StatoDicastero, indicatore};
use crate::atti::Atto;
use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::{Bilancio, euro};
use crate::errore::Errore;
use crate::grandezze::EffettoAttivo;
use crate::partita::LIMITE_EURO;
use crate::vista::{VoceAgenda, VoceAtto, VoceCantiere, VoceEffetto};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum StatoAgenda {
    Annunciata,
    Fermata,
    Dirottata,
}

/// L'atto che il ministro annuncia in apertura. Gli indici di atto sono del catalogo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agenda {
    pub atto: usize,
    /// Solo per i dicasteri con regione bersaglio.
    pub regione: Option<usize>,
    pub stato: StatoAgenda,
    pub motivo: String,
    /// Dei 4 atti del dicastero, nell'ordine del catalogo; mai nella `Vista`.
    pub attaccamenti: Vec<f64>,
    /// Punteggio di ogni regione; vuoto se il dicastero non ha regione bersaglio.
    pub bersaglio: Vec<f64>,
    /// L'importo del decreto di `Accogli` che l'ha messa in agenda; 0 altrimenti.
    pub decreto: i64,
}

/// Un atto a più turni in corso. `rata` è il costo per turno fissato all'avvio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cantiere {
    pub atto: usize,
    pub regione: Option<usize>,
    pub turni_mancanti: u32,
    pub rata: i64,
    pub speso: i64,
}

/// Cantieri ammessi per dicastero in un salvataggio.
const MAX_CANTIERI: usize = 80;

/// Margine annuo di ogni dicastero nel bilancio del turno zero.
pub(super) fn margini_rif(dati: &DatiGioco) -> Vec<i64> {
    let b = Bilancio::iniziale(dati);
    b.dicasteri.iter().map(|d| d.margine).collect()
}

/// Euro per turno di lavori dell'atto nel dicastero `i`:
/// `round(costo · margine_rif/4 · indice_prezzi)`, con `rif` i `margini_rif` e l'indice
/// pubblicato, così la `Vista` non rivela il livello vero.
pub(super) fn costo_atto(cx: &Contesto, rif: &[i64], i: usize, a: &Atto) -> i64 {
    euro(a.costo * rif[i] as f64 / 4.0 * cx.indice_prezzi)
}

/// Posizione di un atto (indice di catalogo) fra i 4 del dicastero.
pub(super) fn posizione(dati: &DatiGioco, d: usize, atto: usize) -> Option<usize> {
    dati.atti_del_dicastero(d).position(|(k, _)| k == atto)
}

/// L'atto (indice di catalogo) ha già un cantiere in corso nel dicastero.
pub(super) fn in_cantiere(st: &StatoDicastero, atto: usize) -> bool {
    st.cantieri.iter().any(|c| c.atto == atto)
}

pub(super) fn disponibile(st: &StatoDicastero) -> i64 {
    st.margine_residuo - st.cantieri.iter().map(|c| c.rata).sum::<i64>()
}

/// `(peso_obiettivo · o, peso_posizione · v)` di un atto per il ministro del dicastero `i`.
pub(super) fn parti(cx: &Contesto, i: usize, m: &Ministro, a: &Atto) -> (f64, f64) {
    let dati = cx.dati;
    let v = 1.0 - (a.posizione - m.posizione).abs() / 6.0;
    let o = indicatore(cx, i).map_or(0.0, |k| {
        let g = &dati.grandezze()[k];
        let segno = g.verso.segno();
        let d = (segno * (m.obiettivo - cx.grandezze.nazionale(dati, k))).max(0.0);
        let spinta = (a.effetti.iter())
            .filter(|e| e.grandezza == g.id)
            .map(|e| e.intensita)
            .sum::<f64>();
        if d > 0.0 {
            ((segno * spinta).min(d) / d).clamp(-1.0, 1.0)
        } else {
            0.0
        }
    });
    (
        dati.manopola("peso_obiettivo") * o,
        dati.manopola("peso_posizione") * v,
    )
}

/// Per ogni regione `(peso_bisogno · b_r, peso_voti · w_r)`: bisogno (disoccupazione sopra la
/// media, infrastrutture e investimenti sotto 100) e voti del partito sopra la sua quota
/// nazionale nel sondaggio pubblico.
fn termini_regioni(cx: &Contesto, partito: usize) -> Vec<(f64, f64)> {
    let dati = cx.dati;
    let al = |v: &[f64], r: usize| v.get(r).copied().unwrap_or(0.0);
    let n = dati.regioni().len();
    let media = |valori: &[f64], pesi: &[f64]| {
        let tot: f64 = pesi.iter().sum();
        pesi.iter().zip(valori).map(|(w, x)| w * x).sum::<f64>() / tot
    };
    let forze: Vec<f64> = dati.regioni().iter().map(|r| r.forze_lavoro).collect();
    let u: Vec<f64> = cx
        .economia
        .regioni
        .iter()
        .map(|r| r.disoccupazione)
        .collect();
    let u_nazionale = media(&u, &forze);
    let elettori: Vec<f64> = (dati.elettorato().regioni.iter())
        .map(|r| r.elettori as f64)
        .collect();
    let quote: Vec<f64> = (cx.sondaggio.regioni.iter())
        .map(|inc| inc.quote.get(partito).copied().unwrap_or(0.0))
        .collect();
    let q_nazionale = media(&quote, &elettori);
    let riga = |id: &str| dati.grandezza(id).map(|k| &cx.grandezze.valori[k]);
    let (infrastrutture, investimenti) =
        (riga("stock_infrastrutture"), riga("investimenti_privati"));
    let (peso_bisogno, peso_voti) = (
        dati.manopola("peso_bisogno_bersaglio"),
        dati.manopola("peso_voti_bersaglio"),
    );
    (0..n)
        .map(|r| {
            let stock = |riga: Option<&Vec<f64>>| riga.map_or(100.0, |v| al(v, r));
            let dotazione = (stock(infrastrutture) + stock(investimenti)) / 2.0;
            let bisogno = (al(&u, r) - u_nazionale) + (100.0 - dotazione) / 10.0;
            let voti = 100.0 * (al(&quote, r) - q_nazionale);
            (peso_bisogno * bisogno, peso_voti * voti)
        })
        .collect()
}

/// `"; {regione}: ci votano"` o `"; {regione}: ne ha bisogno"`, secondo il termine che pesa di
/// più nella regione `r`.
fn motivo_regione(cx: &Contesto, termini: &[(f64, f64)], r: usize) -> String {
    let (bisogno, voti) = termini[r];
    let perche = if voti >= bisogno {
        "ci votano"
    } else {
        "ne ha bisogno"
    };
    format!("; {}: {perche}", cx.dati.regioni()[r].nome)
}

/// Il motivo senza la coda della regione bersaglio.
pub(super) fn senza_regione(motivo: &str) -> &str {
    motivo.split_once("; ").map_or(motivo, |(base, _)| base)
}

/// Effetti di un atto al momento in cui si registrano: intensità `· c/3`, inizio dopo
/// `round(ritardo · 3/c)` turni, con `c` la capacità effettiva del ministro.
fn effetti_di(
    dati: &DatiGioco,
    atto: &Atto,
    dicastero: usize,
    regione: Option<usize>,
    c: f64,
    turno: u32,
) -> Vec<EffettoAttivo> {
    (atto.effetti.iter())
        .filter_map(|e| {
            let k = dati.grandezza(&e.grandezza)?;
            let ritardo = (f64::from(e.ritardo) * 3.0 / c).round() as u32;
            Some(EffettoAttivo {
                grandezza: k,
                regione: if dati.grandezze()[k].regionale {
                    regione
                } else {
                    None
                },
                dicastero: Some(dicastero),
                disposizione: None,
                intensita: e.intensita * c / 3.0,
                inizio: turno.saturating_add(ritardo),
                forma: e.forma,
                durata: e.durata,
            })
        })
        .collect()
}

/// Applica una variazione di soddisfazione al ministro, se c'è e se non è nulla.
pub(crate) fn penalizza(st: &mut StatoDicastero, causa: Causa, delta: f64, turno: u32) {
    if delta == 0.0 {
        return;
    }
    if let Some(m) = st.ministro.as_mut() {
        registra(m, &mut st.variazioni, causa, delta, turno);
        m.soddisfazione = m.soddisfazione.clamp(0.0, 100.0);
    }
}

impl Ministri {
    /// Apertura dei lavori: il margine (turno zero: un trimestre del bilancio iniziale; ogni
    /// gennaio-marzo: l'anno intero del bilancio che vale), le 40 estrazioni di rumore nell'ordine
    /// del catalogo (sempre, anche per interim e margine esaurito), poi le agende.
    /// Nell'esercizio provvisorio né agende né proposte.
    pub(super) fn apri_agende(&mut self, cx: &Contesto, caso: &mut Caso) {
        let rumore: Vec<f64> = (0..cx.dati.atti().len())
            .map(|_| caso.normale(Flusso::Governo))
            .collect();
        let rif = margini_rif(cx.dati);
        let annuo = cx.economia.bilancio_in_vigore(cx.data);
        let provvisorio = cx.economia.esercizio_provvisorio(cx.data);
        for i in 0..self.dicasteri.len() {
            if cx.turno == 0 {
                self.dicasteri[i].margine_residuo = euro(rif[i] as f64 / 4.0);
            } else if cx.data.trimestre == 1 {
                self.dicasteri[i].margine_residuo = annuo.dicasteri[i].margine;
            }
            let agenda = if provvisorio {
                None
            } else {
                self.scegli(cx, i, &rumore, &rif)
            };
            self.dicasteri[i].agenda = agenda;
        }
        if provvisorio {
            // Senza agende ogni ministro chiederebbe fondi che nessuno può accogliere.
            self.dicasteri.iter_mut().for_each(|st| st.proposta = None);
        } else {
            self.proponi(cx);
        }
    }

    /// `BudgetTagliato`: `−costo · perdita` per ogni dicastero con ministro, con `perdite` le
    /// perdite reali in % delle voci, nell'ordine dei dicasteri; nulla dove la perdita non è
    /// positiva.
    pub(crate) fn costo_taglio(&mut self, perdite: &[f64], costo: f64, turno: u32) {
        for (st, p) in self.dicasteri.iter_mut().zip(perdite) {
            penalizza(st, Causa::BudgetTagliato, -costo * p.max(0.0), turno);
        }
    }

    /// L'atto pagabile, senza cantiere in corso e col prerequisito soddisfatto col punteggio più
    /// alto (a parità il primo), e la regione bersaglio.
    fn scegli(&self, cx: &Contesto, i: usize, rumore: &[f64], rif: &[i64]) -> Option<Agenda> {
        let dati = cx.dati;
        let st = &self.dicasteri[i];
        let m = st.ministro.as_ref()?;
        let atti: Vec<(usize, &Atto)> = dati.atti_del_dicastero(i).collect();
        let parti: Vec<(f64, f64)> = atti.iter().map(|(_, a)| parti(cx, i, m, a)).collect();
        let (libero, sigma) = (disponibile(st), dati.manopola("rumore_agenda"));
        let mut scelta: Option<(usize, f64)> = None;
        for (p, (k, a)) in atti.iter().enumerate() {
            if costo_atto(cx, rif, i, a) > libero
                || in_cantiere(st, *k)
                || a.prerequisito_mancante(dati, cx.disposizioni).is_some()
            {
                continue;
            }
            let punteggio = parti[p].0 + parti[p].1 + sigma * rumore[*k];
            if scelta.is_none_or(|(_, s)| punteggio > s) {
                scelta = Some((p, punteggio));
            }
        }
        let (p, _) = scelta?;
        Some(Self::agenda_di(cx, i, m, &atti, &parti, p))
    }

    /// L'agenda sull'atto in posizione `p` fra i 4 del dicastero, con la regione bersaglio.
    pub(super) fn agenda_di(
        cx: &Contesto,
        i: usize,
        m: &Ministro,
        atti: &[(usize, &Atto)],
        parti: &[(f64, f64)],
        p: usize,
    ) -> Agenda {
        let dati = cx.dati;
        let nome = indicatore(cx, i).map_or("", |k| dati.grandezze()[k].nome.as_str());
        let (peso_o, peso_v) = parti[p];
        let mut motivo = if peso_o >= peso_v {
            format!("{nome}: avvicina l'obiettivo")
        } else {
            "vicino alla sua posizione".to_string()
        };
        let (mut regione, mut bersaglio) = (None, Vec::new());
        if dati.dicasteri()[i].regione_bersaglio {
            let termini = termini_regioni(cx, m.partito);
            bersaglio = termini.iter().map(|(b, w)| b + w).collect();
            let migliore = bersaglio.iter().enumerate().fold(None, |p, (r, &s)| {
                if p.is_none_or(|(_, t)| s > t) {
                    Some((r, s))
                } else {
                    p
                }
            });
            if let Some((r, _)) = migliore {
                regione = Some(r);
                motivo.push_str(&motivo_regione(cx, &termini, r));
            }
        }
        Agenda {
            atto: atti[p].0,
            regione,
            stato: StatoAgenda::Annunciata,
            motivo,
            attaccamenti: parti.iter().map(|(o, v)| o + v).collect(),
            bersaglio,
            decreto: 0,
        }
    }

    /// Primo passo della chiusura: per ogni dicastero con ministro prima i cantieri in corso,
    /// poi l'agenda se non fermata e senza dimissioni chieste. Rende gli effetti registrati;
    /// l'agenda si svuota. In interim i cantieri restano fermi.
    pub(crate) fn esegui(&mut self, cx: &Contesto) -> Vec<EffettoAttivo> {
        let dati = cx.dati;
        let rif = margini_rif(dati);
        let mut nuovi = Vec::new();
        for (i, st) in self.dicasteri.iter_mut().enumerate() {
            let agenda = st.agenda.take();
            let Some(c) = st.ministro.as_ref().map(|m| m.capacita_effettiva(dati)) else {
                continue;
            };
            for cantiere in &mut st.cantieri {
                if cantiere.rata <= st.margine_residuo {
                    st.margine_residuo -= cantiere.rata;
                    cantiere.speso += cantiere.rata;
                    cantiere.turni_mancanti = cantiere.turni_mancanti.saturating_sub(1);
                }
            }
            for f in st.cantieri.iter().filter(|f| f.turni_mancanti == 0) {
                let a = &dati.atti()[f.atto];
                nuovi.extend(effetti_di(dati, a, i, f.regione, c, cx.turno));
            }
            st.cantieri.retain(|f| f.turni_mancanti > 0);
            let Some(ag) = agenda.filter(|a| a.stato != StatoAgenda::Fermata) else {
                continue;
            };
            let atto = &dati.atti()[ag.atto];
            let spesa = costo_atto(cx, &rif, i, atto);
            if st.dimissioni_chieste || spesa > st.margine_residuo {
                continue;
            }
            st.margine_residuo -= spesa;
            if atto.turni <= 1 {
                nuovi.extend(effetti_di(dati, atto, i, ag.regione, c, cx.turno));
            } else {
                st.cantieri.push(Cantiere {
                    atto: ag.atto,
                    regione: ag.regione,
                    turni_mancanti: atto.turni - 1,
                    rata: spesa,
                    speso: spesa,
                });
            }
        }
        nuovi
    }

    /// `Ferma`: l'agenda annunciata non si esegue; costa soddisfazione in proporzione
    /// all'attaccamento del ministro all'atto.
    pub(super) fn ferma(&mut self, cx: &Contesto, i: usize) -> Result<(), Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let st = &mut self.dicasteri[i];
        let Some(ag) = st.agenda.as_mut() else {
            return Err(Errore::Comando(format!("nessuna agenda: {id}")));
        };
        if ag.stato != StatoAgenda::Annunciata {
            return Err(Errore::Comando(format!("agenda già modificata: {id}")));
        }
        let att = posizione(dati, i, ag.atto)
            .and_then(|p| ag.attaccamenti.get(p))
            .copied()
            .unwrap_or(0.0);
        ag.stato = StatoAgenda::Fermata;
        let delta = -dati.manopola("costo_fermare") * att;
        penalizza(st, Causa::AttoFermato, delta, cx.turno);
        Ok(())
    }

    /// `Dirotta`: un altro atto pagabile del dicastero e/o un'altra regione bersaglio.
    pub(super) fn dirotta(
        &mut self,
        cx: &Contesto,
        i: usize,
        atto: Option<&str>,
        regione: Option<&str>,
    ) -> Result<(), Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let ko = |m: String| Err(Errore::Comando(m));
        let st = &self.dicasteri[i];
        let Some(ag) = st.agenda.as_ref() else {
            return ko(format!("nessuna agenda: {id}"));
        };
        if ag.stato != StatoAgenda::Annunciata {
            return ko(format!("agenda già modificata: {id}"));
        }
        let nuovo_atto = match atto {
            None => None,
            Some(a) => match dati.atti_del_dicastero(i).find(|(_, x)| x.id == a) {
                Some((k, _)) => Some(k),
                None => return ko(format!("atto sconosciuto o di un altro dicastero: {a}")),
            },
        };
        let nuova_regione = match regione {
            None => None,
            Some(_) if !dati.dicasteri()[i].regione_bersaglio => {
                return ko(format!("{id} non ha regione bersaglio"));
            }
            Some(r) => match dati.regioni().iter().position(|x| x.id == r) {
                Some(k) => Some(k),
                None => return ko(format!("regione sconosciuta: {r}")),
            },
        };
        let cambia_atto = nuovo_atto.filter(|&k| k != ag.atto);
        let cambia_regione = nuova_regione.filter(|&r| Some(r) != ag.regione);
        if cambia_atto.is_none() && cambia_regione.is_none() {
            return ko(format!("niente da dirottare: {id}"));
        }
        if let Some(k) = cambia_atto {
            if ag.decreto > 0 && cx.parlamento.decaduto_accolto(i, k) {
                let nome = &dati.atti()[k].nome;
                return ko(format!("decreto decaduto non si ripete: {nome}"));
            }
            if in_cantiere(st, k) {
                return ko(format!("cantiere già in corso: {}", dati.atti()[k].id));
            }
            if costo_atto(cx, &margini_rif(dati), i, &dati.atti()[k]) > disponibile(st) {
                return ko(format!("margine insufficiente per {}", dati.atti()[k].id));
            }
            if let Some(m) = dati.atti()[k].prerequisito_mancante(dati, cx.disposizioni) {
                return ko(format!("{}: {m}", dati.atti()[k].id));
            }
        }
        let att = |k: usize| {
            posizione(dati, i, k)
                .and_then(|p| ag.attaccamenti.get(p))
                .copied()
                .unwrap_or(0.0)
        };
        let ber = |r: Option<usize>| r.and_then(|r| ag.bersaglio.get(r)).copied().unwrap_or(0.0);
        let perdita_atto = cambia_atto.map_or(0.0, |k| (att(ag.atto) - att(k)).max(0.0));
        let perdita_regione =
            cambia_regione.map_or(0.0, |r| (ber(ag.regione) - ber(Some(r))).max(0.0));
        let delta = -dati.manopola("costo_fermare") * perdita_atto
            - dati.manopola("costo_dirottare_regione") * perdita_regione;
        let motivo = match (cambia_regione, st.ministro.as_ref()) {
            (Some(r), Some(m)) => {
                let coda = motivo_regione(cx, &termini_regioni(cx, m.partito), r);
                Some(format!("{}{coda}", senza_regione(&ag.motivo)))
            }
            _ => None,
        };
        let st = &mut self.dicasteri[i];
        if let Some(ag) = st.agenda.as_mut() {
            ag.atto = cambia_atto.unwrap_or(ag.atto);
            ag.regione = cambia_regione.or(ag.regione);
            ag.stato = StatoAgenda::Dirottata;
            ag.motivo = motivo.unwrap_or(std::mem::take(&mut ag.motivo));
        }
        penalizza(st, Causa::AttoDirottato, delta, cx.turno);
        Ok(())
    }

    /// `FermaCantiere`: quanto speso resta speso; costa l'attaccamento all'atto del ministro in
    /// carica.
    pub(super) fn ferma_cantiere(
        &mut self,
        cx: &Contesto,
        i: usize,
        atto: &str,
    ) -> Result<(), Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let st = &mut self.dicasteri[i];
        let trovato = (st.cantieri.iter()).position(|f| dati.atti()[f.atto].id == atto);
        let Some(p) = trovato else {
            return Err(Errore::Comando(format!("nessun cantiere {atto} in {id}")));
        };
        let f = st.cantieri.remove(p);
        let attaccamento = st.ministro.as_ref().map_or(0.0, |m| {
            let (o, v) = parti(cx, i, m, &dati.atti()[f.atto]);
            o + v
        });
        let delta = -dati.manopola("costo_fermare") * attaccamento;
        penalizza(st, Causa::CantiereFermato, delta, cx.turno);
        Ok(())
    }

    /// Forma e valori di agende e cantieri di un salvataggio contro i dati.
    pub(super) fn controlla_agende(&self, dati: &DatiGioco) -> Result<(), String> {
        let n_regioni = dati.regioni().len();
        for (i, (d, st)) in dati.dicasteri().iter().zip(&self.dicasteri).enumerate() {
            let suo = |k: usize| dati.atti().get(k).is_some_and(|a| a.dicastero == d.id);
            let regione_giusta = |r: Option<usize>| {
                r.is_some() == d.regione_bersaglio && r.is_none_or(|r| r < n_regioni)
            };
            let in_limite = |x: i64| (0..=LIMITE_EURO).contains(&x);
            if !in_limite(st.margine_residuo) {
                return Err(format!("{}: margine residuo fuori dai limiti", d.id));
            }
            if st.cantieri.len() > MAX_CANTIERI {
                return Err(format!("{}: più di {MAX_CANTIERI} cantieri", d.id));
            }
            let rate = (st.cantieri.iter()).try_fold(0_i64, |s, f| s.checked_add(f.rata));
            if !rate.is_some_and(in_limite) {
                return Err(format!("{}: rate dei cantieri oltre il tetto", d.id));
            }
            if let Some(ag) = &st.agenda {
                let n_bersaglio = if d.regione_bersaglio { n_regioni } else { 0 };
                let buona = st.ministro.is_some()
                    && in_limite(ag.decreto)
                    && suo(ag.atto)
                    && regione_giusta(ag.regione)
                    && ag.attaccamenti.len() == dati.atti_del_dicastero(i).count()
                    && ag.bersaglio.len() == n_bersaglio
                    && ag
                        .attaccamenti
                        .iter()
                        .chain(&ag.bersaglio)
                        .all(|x| x.is_finite());
                if !buona {
                    return Err(format!("{}: agenda non valida", d.id));
                }
            }
            for f in &st.cantieri {
                let buono = suo(f.atto)
                    && regione_giusta(f.regione)
                    && f.turni_mancanti >= 1
                    && in_limite(f.rata)
                    && in_limite(f.speso);
                if !buono {
                    return Err(format!("{}: cantiere non valido", d.id));
                }
            }
        }
        Ok(())
    }

    /// Agenda, atti e cantieri del dicastero `i` per la `Vista`. Mai rumore né attaccamenti.
    pub(super) fn voci_governo(&self, cx: &Contesto, rif: &[i64], i: usize) -> VociGoverno {
        let (dati, st) = (cx.dati, &self.dicasteri[i]);
        let libero = disponibile(st);
        let regione = |r: Option<usize>| r.map(|r| dati.regioni()[r].id.clone());
        let agenda = st.agenda.as_ref().map(|ag| {
            let atto = &dati.atti()[ag.atto];
            let presidente = cx.partiti[cx.governo.presidente].posizione_dichiarata;
            let minaccia = st
                .ministro
                .as_ref()
                .is_some_and(|m| m.soddisfazione < dati.manopola("soglia_minaccia"));
            VoceAgenda {
                atto: atto.id.clone(),
                nome: atto.nome.clone(),
                regione: regione(ag.regione),
                costo: costo_atto(cx, rif, i, atto),
                motivo: ag.motivo.clone(),
                stato: ag.stato,
                in_evidenza: minaccia
                    || (atto.posizione - presidente).abs() >= dati.manopola("distanza_evidenza"),
            }
        });
        let provvisorio = cx.economia.esercizio_provvisorio(cx.data);
        let motivo_senza_agenda = match (&agenda, &st.ministro) {
            (Some(_), _) => None,
            (None, _) if provvisorio => Some("esercizio provvisorio".to_string()),
            (None, None) => Some("interim".to_string()),
            (None, Some(_)) => Some("margine esaurito".to_string()),
        };
        let atti = dati
            .atti_del_dicastero(i)
            .map(|(_, a)| {
                let spesa = costo_atto(cx, rif, i, a);
                let prerequisito_mancante = a.prerequisito_mancante(dati, cx.disposizioni);
                VoceAtto {
                    id: a.id.clone(),
                    nome: a.nome.clone(),
                    motto: a.motto.clone(),
                    costo: spesa,
                    turni: a.turni,
                    pagabile: spesa <= libero && prerequisito_mancante.is_none(),
                    prerequisito_mancante,
                    effetti: (a.effetti.iter())
                        .map(|e| VoceEffetto {
                            migliora: dati.grandezza(&e.grandezza).is_some_and(|k| {
                                e.intensita * dati.grandezze()[k].verso.segno() > 0.0
                            }),
                            grandezza: e.grandezza.clone(),
                            ritardo: e.ritardo,
                        })
                        .collect(),
                }
            })
            .collect();
        let cantieri = (st.cantieri.iter())
            .map(|f| VoceCantiere {
                atto: dati.atti()[f.atto].id.clone(),
                regione: regione(f.regione),
                turni_mancanti: f.turni_mancanti,
                rata: f.rata,
                speso: f.speso,
            })
            .collect();
        VociGoverno {
            agenda,
            motivo_senza_agenda,
            atti,
            cantieri,
        }
    }
}

/// Le parti della `VoceMinistero` che vengono dall'agenda.
pub(super) struct VociGoverno {
    pub agenda: Option<VoceAgenda>,
    pub motivo_senza_agenda: Option<String>,
    pub atti: Vec<VoceAtto>,
    pub cantieri: Vec<VoceCantiere>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::atti::{EffettoAtto, Forma, Lato, Velocita};
    use crate::dati::dati_base as dati;

    #[test]
    fn capacita_5_quintuplica_l_intensita_di_capacita_1_e_divide_il_ritardo() {
        let d = dati();
        let atto = Atto {
            id: "prova".into(),
            nome: "Prova".into(),
            motto: String::new(),
            dicastero: "sanita".into(),
            velocita: Velocita::Rapido,
            lato: Lato::Sinistra,
            posizione: 3.0,
            turni: 1,
            costo: 1.0,
            effetti: vec![EffettoAtto {
                grandezza: "qualita_servizio_sanitario".into(),
                intensita: 3.0,
                ritardo: 5,
                forma: Forma::Impulso,
                durata: 2.0,
            }],
            prerequisito: None,
        };
        let uno = effetti_di(&d, &atto, 3, None, 1.0, 10);
        let cinque = effetti_di(&d, &atto, 3, None, 5.0, 10);
        assert_eq!((uno.len(), cinque.len()), (1, 1));
        assert!((cinque[0].intensita - 5.0 * uno[0].intensita).abs() < 1e-12);
        assert_eq!(uno[0].inizio - 10, 5 * (cinque[0].inizio - 10));
        assert_eq!(cinque[0].inizio, 13);
    }

    #[test]
    fn effetti_di_scarta_la_regione_sulle_grandezze_nazionali() {
        let d = dati();
        let effetto = |grandezza: &str| EffettoAtto {
            grandezza: grandezza.into(),
            intensita: 2.0,
            ritardo: 0,
            forma: Forma::Stock,
            durata: 10.0,
        };
        let atto = Atto {
            id: "prova".into(),
            nome: "Prova".into(),
            motto: String::new(),
            dicastero: "infrastrutture_trasporti".into(),
            velocita: Velocita::Lento,
            lato: Lato::Sinistra,
            posizione: 3.0,
            turni: 1,
            costo: 1.0,
            effetti: vec![
                effetto("stock_infrastrutture"),
                effetto("qualita_trasporti"),
            ],
            prerequisito: None,
        };
        let e = effetti_di(&d, &atto, 5, Some(3), 3.0, 0);
        assert_eq!(
            e.iter().map(|x| x.regione).collect::<Vec<_>>(),
            [Some(3), None]
        );
    }

    /// Un atto che peggiora molto l'indicatore non va sotto `o = −1`.
    #[test]
    #[expect(
        clippy::unwrap_used,
        reason = "nei test un unwrap che fallisce e' il fallimento voluto"
    )]
    fn o_resta_fra_meno_uno_e_uno() {
        let d = dati();
        let peso = d.manopola("peso_obiettivo");
        let atto = |intensita: f64| Atto {
            id: "prova".into(),
            nome: "Prova".into(),
            motto: String::new(),
            dicastero: "sanita".into(),
            velocita: Velocita::Rapido,
            lato: Lato::Sinistra,
            posizione: 3.0,
            turni: 1,
            costo: 1.0,
            effetti: vec![EffettoAtto {
                grandezza: "qualita_servizio_sanitario".into(),
                intensita,
                ritardo: 1,
                forma: Forma::Impulso,
                durata: 2.0,
            }],
            prerequisito: None,
        };
        let mut p = crate::partita::Partita::nuova(d, 1);
        p.con_contesto(|ministri, cx| {
            let m = ministri.dicasteri[3].ministro.as_ref().unwrap();
            for (intensita, atteso) in [(-1000.0, -1.0), (1000.0, 1.0)] {
                let (o, _) = parti(cx, 3, m, &atto(intensita));
                assert!((o - peso * atteso).abs() < 1e-12, "{intensita}: {o}");
            }
        });
    }
}
