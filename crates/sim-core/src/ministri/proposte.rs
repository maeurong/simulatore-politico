//! Proposte di fondi: il ministro il cui atto preferito non è pagabile chiede la differenza.
//! Il giocatore accoglie, rinvia (una volta) o respinge; ignorata un turno vale respinta.

use super::agenda::{
    Agenda, StatoAgenda, costo, disponibile, in_cantiere, margini_rif, parti, penalizza, posizione,
};
use super::{Causa, Contesto, Ministri};
use crate::dati::DatiGioco;
use crate::errore::Errore;
use crate::vista::VoceProposta;
use serde::{Deserialize, Serialize};

/// Una richiesta di fondi per un atto del catalogo; al più una per ministro.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Proposta {
    pub atto: usize,
    /// Euro che mancano al margine perché l'atto sia pagabile.
    pub importo: i64,
    pub turni_in_sospeso: u32,
    pub rinviata: bool,
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
            let mancano = |k: usize| {
                let spesa = costo(
                    &dati.atti()[k],
                    rif.get(i).copied().unwrap_or(0),
                    cx.economia.livello_prezzi,
                );
                spesa - disponibile(st)
            };
            let nuova = if st.agenda.is_some() {
                None
            } else if let Some(p) = &st.proposta {
                Some(Proposta {
                    importo: mancano(p.atto),
                    ..p.clone()
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
                    importo: mancano(k),
                    turni_in_sospeso: 0,
                    rinviata: false,
                })
            };
            self.dicasteri[i].proposta = nuova.filter(|p| p.importo > 0);
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

    /// `Accogli`: rende l'importo del decreto, che entra anche nel margine residuo.
    pub(super) fn accogli(&mut self, cx: &Contesto, i: usize) -> Result<i64, Errore> {
        let (dati, id) = (cx.dati, &cx.dati.dicasteri()[i].id);
        let st = &self.dicasteri[i];
        let nessuna = || Errore::Comando(format!("nessuna proposta: {id}"));
        let (Some(p), Some(m)) = (st.proposta.as_ref(), st.ministro.as_ref()) else {
            return Err(nessuna());
        };
        if in_cantiere(st, p.atto) {
            let atto = &dati.atti()[p.atto].id;
            return Err(Errore::Comando(format!("cantiere già in corso: {atto}")));
        }
        let agenda = match &st.agenda {
            Some(old) => Agenda {
                atto: p.atto,
                stato: StatoAgenda::Annunciata,
                motivo: "fondi accolti".to_string(),
                ..old.clone()
            },
            None => {
                let atti: Vec<_> = dati.atti_del_dicastero(i).collect();
                let parti: Vec<_> = atti.iter().map(|(_, a)| parti(cx, i, m, a)).collect();
                let att = parti.iter().map(|(o, v)| o + v).collect();
                let pos = posizione(dati, i, p.atto).ok_or_else(nessuna)?;
                Self::agenda_di(cx, i, m, &atti, &parti, att, pos)
            }
        };
        let importo = p.importo;
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
            if !(suo && st.ministro.is_some() && (1..=1_000_000_000_000_000).contains(&p.importo)) {
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
