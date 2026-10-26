//! Proposte di fondi: il ministro il cui atto preferito non è pagabile chiede la differenza.
//! Il giocatore accoglie, rinvia (una volta) o respinge; ignorata un turno vale respinta.

use super::agenda::{
    costo, disponibile, in_cantiere, margini_rif, parti, penalizza, posizione, senza_regione,
};
use super::{Causa, Contesto, Ministri, StatoDicastero};
use crate::dati::DatiGioco;
use crate::errore::Errore;
use crate::partita::LIMITE_EURO;
use crate::vista::VoceProposta;
use serde::{Deserialize, Serialize};

/// Una richiesta di fondi per un atto del catalogo; al più una per ministro.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposta {
    pub atto: usize,
    /// Euro che mancano al margine perché l'atto sia pagabile.
    pub importo: i64,
    pub turni_in_sospeso: u32,
    pub rinviata: bool,
}

/// Euro che mancano al margine perché l'atto `k` del dicastero `i` sia pagabile.
fn mancano(cx: &Contesto, rif: &[i64], i: usize, st: &StatoDicastero, k: usize) -> i64 {
    let spesa = costo(&cx.dati.atti()[k], rif[i], cx.economia.livello_prezzi);
    spesa.saturating_sub(disponibile(st))
}

impl Ministri {
    /// Apertura, dopo le agende. Chi è in carica, senza agenda (nessun atto pagabile), e ha il
    /// suo atto più sentito (`o > 0`, a parità il primo, mai uno già in cantiere) non pagabile,
    /// chiede la differenza.
    /// Una proposta già in corso si ricalcola sul suo atto: con un'agenda, o se l'atto è ormai
    /// pagabile, decade senza costo; altrimenti l'`importo` si aggiorna (`rinviata` e
    /// `turni_in_sospeso` restano).
    pub(super) fn proponi(&mut self, cx: &Contesto) {
        let (dati, rif) = (cx.dati, margini_rif(cx.dati));
        for i in 0..self.dicasteri.len() {
            let st = &self.dicasteri[i];
            let Some(m) = st.ministro.as_ref() else {
                continue;
            };
            let nuova = if st.agenda.is_some() {
                None
            } else if let Some(p) = &st.proposta {
                Some(Proposta {
                    importo: mancano(cx, &rif, i, st, p.atto),
                    ..*p
                })
            } else {
                let mut migliore: Option<(usize, f64)> = None;
                for (k, a) in dati
                    .atti_del_dicastero(i)
                    .filter(|(k, _)| !in_cantiere(st, *k))
                {
                    let o = parti(cx, i, m, a).0;
                    if o > 0.0 && migliore.is_none_or(|(_, b)| o > b) {
                        migliore = Some((k, o));
                    }
                }
                migliore.map(|(k, _)| Proposta {
                    atto: k,
                    importo: mancano(cx, &rif, i, st, k),
                    turni_in_sospeso: 0,
                    rinviata: false,
                })
            };
            self.dicasteri[i].proposta = nuova.filter(|p| (1..=LIMITE_EURO).contains(&p.importo));
        }
    }

    /// Default delle decisioni: una proposta non decisa resta in sospeso un turno (due se
    /// rinviata), poi vale respinta.
    pub(crate) fn decidi_proposte(&mut self, cx: &Contesto) {
        for st in &mut self.dicasteri {
            let Some(p) = st.proposta.as_mut() else {
                continue;
            };
            if p.turni_in_sospeso < 1 + u32::from(p.rinviata) {
                p.turni_in_sospeso += 1;
            } else {
                st.proposta = None;
                let delta = -cx.dati.manopola("costo_respingere");
                penalizza(st, Causa::PropostaRespinta, delta, cx.turno);
            }
        }
    }

    /// `Accogli`: ricalcola quanto manca e rende l'importo del decreto, che entra anche nel
    /// margine residuo. Una proposta esiste solo senza agenda (la decade `proponi`, la rifiuta il
    /// salvataggio).
    pub(super) fn accogli(&mut self, cx: &Contesto, i: usize) -> Result<i64, Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let st = &self.dicasteri[i];
        let nessuna = || Errore::Comando(format!("nessuna proposta: {id}"));
        let (Some(p), Some(m)) = (st.proposta.as_ref(), st.ministro.as_ref()) else {
            return Err(nessuna());
        };
        let atto = &dati.atti()[p.atto].id;
        if in_cantiere(st, p.atto) {
            return Err(Errore::Comando(format!("cantiere già in corso: {atto}")));
        }
        let importo = mancano(cx, &margini_rif(dati), i, st, p.atto);
        if importo <= 0 {
            return Err(Errore::Comando(format!("fondi già sufficienti per {atto}")));
        }
        let atti: Vec<_> = dati.atti_del_dicastero(i).collect();
        let parti: Vec<_> = atti.iter().map(|(_, a)| parti(cx, i, m, a)).collect();
        let att = parti.iter().map(|(o, v)| o + v).collect();
        let pos = posizione(dati, i, p.atto).ok_or_else(nessuna)?;
        let mut agenda = Self::agenda_di(cx, i, m, &atti, &parti, att, pos);
        let coda = &agenda.motivo[senza_regione(&agenda.motivo).len()..];
        agenda.motivo = format!("fondi accolti dal Presidente del Consiglio{coda}");
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
            let suo = dati.atti().get(p.atto).is_some_and(|a| a.dicastero == d.id);
            let buona = suo
                && st.ministro.is_some()
                && st.agenda.is_none()
                && (1..=LIMITE_EURO).contains(&p.importo);
            if !buona {
                return Err(format!("{}: proposta non valida", d.id));
            }
        }
        Ok(())
    }

    /// La proposta del dicastero `i` per la `Vista`.
    pub(super) fn voce_proposta(&self, cx: &Contesto, i: usize) -> Option<VoceProposta> {
        let p = self.dicasteri[i].proposta.as_ref()?;
        let a = cx.dati.atti().get(p.atto)?;
        Some(VoceProposta {
            atto: a.id.clone(),
            nome: a.nome.clone(),
            importo: p.importo,
            motivo: format!("fondi che non bastano per {}", a.nome),
            rinviata: p.rinviata,
        })
    }
}
