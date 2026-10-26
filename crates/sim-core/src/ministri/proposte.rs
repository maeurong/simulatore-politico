//! Proposte dei ministri: fondi per l'atto preferito non pagabile, o una legge di riforma che
//! sblocca il prerequisito di un atto o avvicina una disposizione alla posizione del ministro.
//! Il giocatore accoglie, rinvia (una volta) o respinge; ignorata un turno vale respinta.

use super::agenda::{
    costo_atto, disponibile, in_cantiere, margini_rif, parti, penalizza, posizione, senza_regione,
};
use super::{Causa, Contesto, Ministri, Ministro, StatoDicastero};
use crate::dati::DatiGioco;
use crate::disposizioni::Norma;
use crate::errore::Errore;
use crate::parlamento;
use crate::partita::LIMITE_EURO;
use crate::vista::VoceProposta;
use serde::{Deserialize, Serialize};

/// La proposta di un ministro; al più una per ministro.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposta {
    pub tipo: TipoProposta,
    pub turni_in_sospeso: u32,
    pub rinviata: bool,
}

/// Indici del catalogo.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TipoProposta {
    /// `importo`: euro che mancano al margine perché l'atto sia pagabile.
    Fondi { atto: usize, importo: i64 },
    /// La legge che porta la disposizione al gradino; `sblocca`: l'atto il cui prerequisito
    /// soddisfa (innesco 2), `None` per l'innesco 3.
    Riforma {
        disposizione: usize,
        gradino: usize,
        sblocca: Option<usize>,
    },
}

/// Euro che mancano al margine perché l'atto `k` del dicastero `i` sia pagabile.
fn mancano(cx: &Contesto, rif: &[i64], i: usize, st: &StatoDicastero, k: usize) -> i64 {
    costo_atto(cx, rif, i, &cx.dati.atti()[k]).saturating_sub(disponibile(st))
}

/// La disposizione si può presentare: non è in esame né in una legge respinta da poco.
fn libera(cx: &Contesto, d: usize) -> bool {
    !cx.parlamento.in_esame(d) && cx.parlamento.respinta_da_poco(d, cx.turno).is_none()
}

/// Accogli non la rifiuterebbe: la disposizione è [`libera`] e il gradino non è né in
/// vigore né bloccato dal referendum.
fn aperta(cx: &Contesto, d: usize, g: usize) -> bool {
    let s = &cx.disposizioni.voci[d];
    libera(cx, d) && s.in_vigore != g && !s.bloccati.contains(&g)
}

/// Una riforma in corso tiene: è [`aperta`] e il suo innesco vale ancora (il prerequisito
/// dell'atto manca, o la disposizione in vigore dista dal ministro almeno `distanza_alleato`).
fn tiene(cx: &Contesto, m: &Ministro, tipo: TipoProposta) -> bool {
    let TipoProposta::Riforma {
        disposizione: d,
        gradino,
        sblocca,
    } = tipo
    else {
        return true;
    };
    let dati = cx.dati;
    aperta(cx, d, gradino)
        && match sblocca {
            Some(k) => (dati.atti()[k].prerequisito_mancante(dati, cx.disposizioni)).is_some(),
            None => (dati.disposizioni()[d].posizione(cx.disposizioni.voci[d].in_vigore))
                .is_some_and(|v| (v - m.posizione).abs() >= dati.manopola("distanza_alleato")),
        }
}

/// L'atto del dicastero `i` più sentito (`o > 0`, a parità il primo), fra quelli senza cantiere
/// in corso che passano `filtro`.
fn piu_sentito(
    cx: &Contesto,
    i: usize,
    m: &Ministro,
    st: &StatoDicastero,
    filtro: impl Fn(usize) -> bool,
) -> Option<usize> {
    let mut migliore: Option<(usize, f64)> = None;
    for (k, a) in
        (cx.dati.atti_del_dicastero(i)).filter(|(k, _)| !in_cantiere(st, *k) && filtro(*k))
    {
        let o = parti(cx, i, m, a).0;
        if o > 0.0 && migliore.is_none_or(|(_, b)| o > b) {
            migliore = Some((k, o));
        }
    }
    migliore.map(|(k, _)| k)
}

/// Innesco 2: fra gli atti col prerequisito mancante il più sentito (ticket 16: «un atto che
/// serve al suo obiettivo») → la sua disposizione al primo gradino ammesso ancora [`aperta`].
fn sblocco(cx: &Contesto, i: usize, m: &Ministro, st: &StatoDicastero) -> Option<TipoProposta> {
    let dati = cx.dati;
    let mancante = |k: usize| {
        dati.atti()[k]
            .prerequisito_mancante(dati, cx.disposizioni)
            .is_some()
    };
    let k = piu_sentito(cx, i, m, st, mancante)?;
    let p = dati.atti()[k].prerequisito.as_ref()?;
    let d = dati.disposizione(&p.disposizione)?;
    let g = (p.gradini.iter())
        .filter_map(|id| dati.disposizioni()[d].gradino(id))
        .find(|&g| aperta(cx, d, g))?;
    Some(TipoProposta::Riforma {
        disposizione: d,
        gradino: g,
        sblocca: Some(k),
    })
}

/// Innesco 3: fra le disposizioni ordinarie del dicastero non in esame e senza gradini
/// bloccati, la più lontana dalla posizione del ministro (a parità la prima), se almeno
/// `distanza_alleato`, al gradino più vicino alla sua posizione (a parità il più vicino al
/// vigente). Una disposizione il cui gradino più vicino è già in vigore non si propone.
fn distanza(cx: &Contesto, i: usize, m: &Ministro) -> Option<TipoProposta> {
    let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
    let mut migliore: Option<(usize, usize, f64)> = None;
    for (d, x) in dati.disposizioni().iter().enumerate() {
        let s = &cx.disposizioni.voci[d];
        let candidata = x.dicastero.as_ref() == Some(id)
            && x.norma == Norma::Ordinaria
            && s.bloccati.is_empty()
            && libera(cx, d);
        let Some(vigore) = x.posizione(s.in_vigore).filter(|_| candidata) else {
            continue;
        };
        let vicino = (0..x.gradini.len())
            .filter_map(|g| Some((g, (x.posizione(g)? - m.posizione).abs())))
            .min_by(|a, b| {
                (a.1.total_cmp(&b.1))
                    .then_with(|| a.0.abs_diff(s.in_vigore).cmp(&b.0.abs_diff(s.in_vigore)))
            });
        let Some((g, _)) = vicino.filter(|&(g, _)| g != s.in_vigore) else {
            continue;
        };
        let lontananza = (vigore - m.posizione).abs();
        if migliore.is_none_or(|(_, _, l)| lontananza > l) {
            migliore = Some((d, g, lontananza));
        }
    }
    let (d, g, lontananza) = migliore?;
    (lontananza >= dati.manopola("distanza_alleato")).then_some(TipoProposta::Riforma {
        disposizione: d,
        gradino: g,
        sblocca: None,
    })
}

impl Ministri {
    /// Apertura, dopo le agende, per chi è in carica senza proposta, nell'ordine:
    /// 1. fondi: senza agenda (nessun atto pagabile), il suo atto più sentito (`o > 0`, a parità
    ///    il primo, mai uno già in cantiere) non pagabile: chiede la differenza;
    /// 2. [`sblocco`] e 3. [`distanza`], anche con un'agenda.
    ///
    /// Una proposta di fondi già in corso si ricalcola sul suo atto: con un'agenda, o se l'atto
    /// è ormai pagabile, decade senza costo; altrimenti l'`importo` si aggiorna (`rinviata` e
    /// `turni_in_sospeso` restano). Un atto col prerequisito mancante non si propone, e la sua
    /// proposta decade. Una proposta di riforma già in corso resta finché [`tiene`], poi decade
    /// senza costo.
    pub(super) fn proponi(&mut self, cx: &Contesto) {
        let (dati, rif) = (cx.dati, margini_rif(cx.dati));
        let ammesso = |k: usize| {
            dati.atti()[k]
                .prerequisito_mancante(dati, cx.disposizioni)
                .is_none()
        };
        let fondi = |atto: usize, importo: i64| {
            ((1..=LIMITE_EURO).contains(&importo) && ammesso(atto))
                .then_some(TipoProposta::Fondi { atto, importo })
        };
        for i in 0..self.dicasteri.len() {
            let st = &self.dicasteri[i];
            let Some(m) = st.ministro.as_ref() else {
                continue;
            };
            let senza_agenda = st.agenda.is_none();
            let nuova = match st.proposta {
                Some(p) => match p.tipo {
                    TipoProposta::Fondi { atto, .. } => senza_agenda
                        .then(|| fondi(atto, mancano(cx, &rif, i, st, atto)))
                        .flatten(),
                    TipoProposta::Riforma { .. } => tiene(cx, m, p.tipo).then_some(p.tipo),
                }
                .map(|tipo| Proposta { tipo, ..p }),
                None => senza_agenda
                    .then(|| piu_sentito(cx, i, m, st, ammesso))
                    .flatten()
                    .and_then(|k| fondi(k, mancano(cx, &rif, i, st, k)))
                    .or_else(|| sblocco(cx, i, m, st))
                    .or_else(|| distanza(cx, i, m))
                    .map(|tipo| Proposta {
                        tipo,
                        turni_in_sospeso: 0,
                        rinviata: false,
                    }),
            };
            self.dicasteri[i].proposta = nuova;
        }
    }

    /// Default delle decisioni: una proposta non decisa resta in sospeso un turno (due se
    /// rinviata), poi vale respinta. Una riforma che non [`tiene`] più decade senza costo.
    pub(crate) fn decidi_proposte(&mut self, cx: &Contesto) {
        for st in &mut self.dicasteri {
            let (Some(p), Some(m)) = (st.proposta.as_mut(), st.ministro.as_ref()) else {
                continue;
            };
            if !tiene(cx, m, p.tipo) {
                st.proposta = None;
            } else if p.turni_in_sospeso < 1 + u32::from(p.rinviata) {
                p.turni_in_sospeso += 1;
            } else {
                st.proposta = None;
                let delta = -cx.dati.manopola("costo_respingere");
                penalizza(st, Causa::PropostaRespinta, delta, cx.turno);
            }
        }
    }

    /// `Accogli` di una proposta di fondi: ricalcola quanto manca e rende l'importo del decreto,
    /// che entra anche nel margine residuo e resta segnato nell'agenda per il ritiro. Una
    /// proposta di fondi esiste solo senza agenda (la decade `proponi`, la rifiuta il
    /// salvataggio).
    pub(super) fn accogli(&mut self, cx: &Contesto, i: usize) -> Result<i64, Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let st = &self.dicasteri[i];
        let nessuna = || Errore::Comando(format!("nessuna proposta: {id}"));
        let (Some(TipoProposta::Fondi { atto: k, .. }), Some(m)) =
            (st.proposta.map(|p| p.tipo), st.ministro.as_ref())
        else {
            return Err(nessuna());
        };
        let atto = &dati.atti()[k].nome;
        if in_cantiere(st, k) {
            return Err(Errore::Comando(format!("cantiere già in corso: {atto}")));
        }
        let importo = mancano(cx, &margini_rif(dati), i, st, k);
        if importo <= 0 {
            return Err(Errore::Comando(format!("fondi già sufficienti per {atto}")));
        }
        let sotto = |x: i64| x.checked_add(importo).is_some_and(|s| s <= LIMITE_EURO);
        if !(sotto(st.margine_residuo) && sotto(cx.economia.decreti.iter().sum())) {
            return Err(Errore::Comando(format!("importo troppo alto per {atto}")));
        }
        let atti: Vec<_> = dati.atti_del_dicastero(i).collect();
        let parti: Vec<_> = atti.iter().map(|(_, a)| parti(cx, i, m, a)).collect();
        let pos = posizione(dati, i, k).ok_or_else(nessuna)?;
        let mut agenda = Self::agenda_di(cx, i, m, &atti, &parti, pos);
        let coda = &agenda.motivo[senza_regione(&agenda.motivo).len()..];
        agenda.motivo = format!("fondi accolti dal Presidente del Consiglio{coda}");
        agenda.decreto = importo;
        let st = &mut self.dicasteri[i];
        st.margine_residuo += importo;
        st.agenda = Some(agenda);
        st.proposta = None;
        Ok(importo)
    }

    /// `Rinvia`: una volta, senza costo; la proposta resta un turno in più.
    pub(super) fn rinvia(&mut self, cx: &Contesto, i: usize) -> Result<(), Errore> {
        let id = &cx.dati.dicasteri()[i].id;
        let Some(p) = self.dicasteri[i].proposta.as_mut() else {
            return Err(Errore::Comando(format!("nessuna proposta: {id}")));
        };
        if p.rinviata {
            return Err(Errore::Comando(format!("proposta già rinviata: {id}")));
        }
        p.rinviata = true;
        Ok(())
    }

    /// `Respingi`: costa soddisfazione.
    pub(super) fn respingi(&mut self, cx: &Contesto, i: usize) -> Result<(), Errore> {
        let st = &mut self.dicasteri[i];
        if st.proposta.take().is_none() {
            let id = &cx.dati.dicasteri()[i].id;
            return Err(Errore::Comando(format!("nessuna proposta: {id}")));
        }
        let delta = -cx.dati.manopola("costo_respingere");
        penalizza(st, Causa::PropostaRespinta, delta, cx.turno);
        Ok(())
    }

    /// Forma delle proposte di un salvataggio contro i dati.
    pub(super) fn controlla_proposte(&self, dati: &DatiGioco) -> Result<(), String> {
        for (d, st) in dati.dicasteri().iter().zip(&self.dicasteri) {
            let Some(p) = &st.proposta else { continue };
            let buona = st.ministro.is_some()
                && match p.tipo {
                    TipoProposta::Fondi { atto, importo } => {
                        dati.atti().get(atto).is_some_and(|a| a.dicastero == d.id)
                            && st.agenda.is_none()
                            && (1..=LIMITE_EURO).contains(&importo)
                    }
                    TipoProposta::Riforma {
                        disposizione,
                        gradino,
                        sblocca,
                    } => {
                        (dati.disposizioni().get(disposizione))
                            .is_some_and(|x| gradino < x.gradini.len())
                            && sblocca.is_none_or(|k| {
                                dati.atti().get(k).is_some_and(|a| a.dicastero == d.id)
                            })
                    }
                };
            if !buona {
                return Err(format!("{}: proposta non valida", d.id));
            }
        }
        Ok(())
    }

    /// La proposta del dicastero `i` per la `Vista`.
    pub(super) fn voce_proposta(&self, cx: &Contesto, i: usize) -> Option<VoceProposta> {
        let p = self.dicasteri[i].proposta.as_ref()?;
        let dati = cx.dati;
        Some(match p.tipo {
            TipoProposta::Fondi { atto, importo } => {
                let a = &dati.atti()[atto];
                VoceProposta {
                    atto: Some(a.id.clone()),
                    nome: a.nome.clone(),
                    importo,
                    motivo: format!("fondi che non bastano per {}", a.nome),
                    rinviata: p.rinviata,
                    disposizione: None,
                    gradino: None,
                }
            }
            TipoProposta::Riforma {
                disposizione,
                gradino,
                sblocca,
            } => {
                let x = &dati.disposizioni()[disposizione];
                VoceProposta {
                    atto: None,
                    nome: parlamento::nome(dati, &[(disposizione, gradino)]),
                    importo: 0,
                    motivo: sblocca.map_or_else(
                        || format!("{}: lontana dalla sua posizione", x.nome),
                        |k| format!("sblocca {}", dati.atti()[k].nome),
                    ),
                    rinviata: p.rinviata,
                    disposizione: Some(x.id.clone()),
                    gradino: Some(x.gradini[gradino].id.clone()),
                }
            }
        })
    }
}
