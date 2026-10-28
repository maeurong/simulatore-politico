//! Emendamenti degli alleati e questione di fiducia (issue 08:20, 08:24;
//! iter-legislativo-italiano:38-44).

use super::{FormaMisura, Misura, Parlamento, StatoAula, ko, nome_misura, posizioni, slitta, voto};
use crate::dati::DatiGioco;
use crate::disposizioni::{Disposizioni, TipoDisposizione};
use crate::errore::Errore;
use crate::partiti::Partito;
use crate::vista::VoceEmendamento;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// L'attenuazione chiesta su una disposizione: un gradino più vicino al vigente, o lo stesso
/// valore in arrivo dopo `turni_rinvio_regime` turni. Sulla legge di bilancio l'indice della
/// coppia è il dicastero e l'attenuazione è `Indicizza`: il budget segue l'inflazione.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Attenuazione {
    Gradino(usize),
    Rinvio,
    Indicizza,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum StatoEmendamento {
    Aperto,
    Accolto,
    Respinto,
}

/// `partito` è un indice; `proposta` = disposizione e attenuazione.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RichiestaEmendamento {
    pub partito: usize,
    pub proposta: Vec<(usize, Attenuazione)>,
    pub stato: StatoEmendamento,
    pub motivo: String,
}

/// Per disposizione: una disposizione a gradini che salta almeno due gradini torna alla metà, arrotondata verso
/// il vigente; un regime, un salto di un gradino o una metà bloccata dal referendum si rinvia.
fn proposta(
    dati: &DatiGioco,
    disposizioni: &Disposizioni,
    valori: &[(usize, usize)],
) -> Vec<(usize, Attenuazione)> {
    (valori.iter())
        .filter_map(|&(d, b)| {
            let s = disposizioni.voci.get(d)?;
            let a = s.in_vigore;
            let parametro = dati.disposizioni().get(d)?.tipo == TipoDisposizione::Parametro;
            let meta = if a > b {
                (a + b).div_ceil(2)
            } else {
                (a + b) / 2
            };
            let x = if parametro && a.abs_diff(b) >= 2 && !s.bloccati.contains(&meta) {
                Attenuazione::Gradino(meta)
            } else {
                Attenuazione::Rinvio
            };
            Some((d, x))
        })
        .collect()
}

/// `"{nome}: {gradino}"`, `"{nome}: in vigore fra {n} turni"` o `"{dicastero}: budget indicizzato
/// all'inflazione"`, uniti da `"; "`.
fn testo(dati: &DatiGioco, proposta: &[(usize, Attenuazione)]) -> String {
    let turni = dati.manopola("turni_rinvio_regime");
    (proposta.iter())
        .filter_map(|&(d, x)| match x {
            Attenuazione::Indicizza => {
                let nome = &dati.dicasteri().get(d)?.nome;
                Some(format!("{nome}: budget indicizzato all'inflazione"))
            }
            Attenuazione::Gradino(g) => {
                let d = dati.disposizioni().get(d)?;
                Some(format!("{}: {}", d.nome, d.gradini.get(g)?.nome))
            }
            Attenuazione::Rinvio => {
                let d = dati.disposizioni().get(d)?;
                Some(format!("{}: in vigore fra {turni} turni", d.nome))
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

pub(crate) fn voci(
    dati: &DatiGioco,
    partiti: &[Partito],
    emendamenti: &[RichiestaEmendamento],
) -> Vec<VoceEmendamento> {
    (emendamenti.iter())
        .map(|e| VoceEmendamento {
            partito: partiti
                .get(e.partito)
                .map(|p| p.id.clone())
                .unwrap_or_default(),
            testo: testo(dati, &e.proposta),
            motivo: e.motivo.clone(),
            stato: e.stato,
        })
        .collect()
}

impl Parlamento {
    /// Apertura del turno `turno`: sulle riforme e le elettorali col voto nel turno, senza fiducia,
    /// senza emendamenti già aperti e fuori dal posto riservato all'opposizione (scelta 15 del
    /// piano 7b), chiede un emendamento ogni alleato distante almeno `distanza_alleato` dalla
    /// misura, che tiene il dicastero proponente o che con la legge elettorale perde seggi attesi;
    /// non chi ha già un emendamento deciso sulla misura.
    pub(crate) fn apri_emendamenti(&mut self, aula: &StatoAula, turno: u32) {
        let (dati, governo) = (aula.dati, aula.governo);
        let soglia = dati.manopola("distanza_alleato");
        for m in &mut self.misure {
            let forma = matches!(m.forma, FormaMisura::Riforma | FormaMisura::Elettorale);
            let opposizione = m.promotore.is_some();
            if !forma || m.voto != turno || m.emendamenti_aperti || m.fiducia || opposizione {
                continue;
            }
            let nuova = posizioni(dati, aula.disposizioni, &m.valori).map(|(_, n)| n);
            let testo = proposta(dati, aula.disposizioni, &m.valori);
            let seggi = if m.forma == FormaMisura::Elettorale {
                aula.seggi_attesi(&m.valori)
            } else {
                Vec::new()
            };
            for (j, p) in aula.partiti.iter().enumerate() {
                if !voto::alleato(governo, j) || m.emendamenti.iter().any(|e| e.partito == j) {
                    continue;
                }
                let distanza = nuova.map(|n| (p.posizione_dichiarata - n).abs());
                let dicastero = (m.proponente)
                    .filter(|&k| governo.dicasteri.get(k) == Some(&j))
                    .and_then(|k| dati.dicasteri().get(k));
                let perde = (seggi.get(j)).map(|&(c, s)| c + s).filter(|&x| x < 0);
                let motivo = match (distanza, dicastero, perde) {
                    (Some(d), _, _) if d >= soglia => {
                        let d = format!("{d:.1}").replace('.', ",");
                        format!("a {d} punti dalla sua posizione")
                    }
                    (_, Some(c), _) => format!("tocca il suo dicastero {}", c.nome),
                    (_, _, Some(x)) => seggi_persi(-x),
                    _ => continue,
                };
                m.emendamenti.push(RichiestaEmendamento {
                    partito: j,
                    proposta: testo.clone(),
                    stato: StatoEmendamento::Aperto,
                    motivo,
                });
            }
            m.emendamenti_aperti = true;
        }
    }

    /// Gli emendamenti della legge di bilancio del turno: restano i decisi, gli aperti diventano
    /// `nuovi`, tranne quelli di chi ha già un emendamento deciso.
    pub(crate) fn emendamenti_bilancio(&mut self, turno: u32, nuovi: Vec<RichiestaEmendamento>) {
        let bilancio = |m: &&mut Misura| m.forma == FormaMisura::Bilancio && m.voto == turno;
        if let Some(m) = self.misure.iter_mut().find(bilancio) {
            m.emendamenti
                .retain(|e| e.stato != StatoEmendamento::Aperto);
            for n in nuovi {
                if !m.emendamenti.iter().any(|e| e.partito == n.partito) {
                    m.emendamenti.push(n);
                }
            }
        }
    }

    /// Chiusura: ogni emendamento ancora aperto è respinto, tranne quelli delle leggi che
    /// slittano nel turno del bilancio o di crisi (`occupato`); rende gli alleati che pagano.
    pub(crate) fn respingi_aperti(&mut self, turno: u32, occupato: bool) -> Vec<usize> {
        let aperti = (self.misure.iter_mut())
            .filter(|m| !(occupato && slitta(m, turno)))
            .flat_map(|m| m.emendamenti.iter_mut())
            .filter(|e| e.stato == StatoEmendamento::Aperto);
        (aperti)
            .map(|e| {
                e.stato = StatoEmendamento::Respinto;
                e.partito
            })
            .collect()
    }

    /// `Emendamento`: accolto, la misura prende il gradino a metà (il rinvio pesa al voto);
    /// respinto, rende l'alleato che paga. Un rifiuto non tocca nulla.
    ///
    /// # Errors
    /// `Comando`: misura sconosciuta, nessun emendamento del partito, emendamento già deciso.
    pub(crate) fn decidi_emendamento(
        &mut self,
        dati: &DatiGioco,
        partiti: &[Partito],
        misura: u32,
        partito: &str,
        accogli: bool,
    ) -> Result<Option<usize>, Errore> {
        let Some(m) = self.misure.iter_mut().find(|m| m.id == misura) else {
            return ko(format!("misura sconosciuta: {misura}"));
        };
        let j = partiti.iter().position(|p| p.id == partito);
        let chi = j.map_or(partito, |j| partiti[j].nome.as_str());
        let legge = nome_misura(dati, m);
        let Some(e) = m.emendamenti.iter_mut().find(|e| Some(e.partito) == j) else {
            return ko(format!("nessun emendamento di {chi} su «{legge}»"));
        };
        if e.stato != StatoEmendamento::Aperto {
            return ko(format!("emendamento già deciso: {chi} su «{legge}»"));
        }
        if !accogli {
            e.stato = StatoEmendamento::Respinto;
            return Ok(Some(e.partito));
        }
        e.stato = StatoEmendamento::Accolto;
        for &(d, x) in &e.proposta {
            if let (Attenuazione::Gradino(g), Some(v)) = (x, m.valori.iter_mut().find(|v| v.0 == d))
            {
                v.1 = g;
            }
        }
        Ok(None)
    }

    /// `PoniFiducia`: la misura si vota nel turno e gli emendamenti aperti cadono senza costo.
    ///
    /// # Errors
    /// `Comando`: misura sconosciuta, norma costituzionale, questione di fiducia già posta,
    /// legge nel turno del bilancio.
    pub(crate) fn poni_fiducia(
        &mut self,
        dati: &DatiGioco,
        misura: u32,
        turno: u32,
    ) -> Result<&Misura, Errore> {
        let occupato = self.turno_del_bilancio(turno);
        let Some(m) = self.misure.iter_mut().find(|m| m.id == misura) else {
            return ko(format!("misura sconosciuta: {misura}"));
        };
        if m.forma == FormaMisura::Costituzionale {
            return ko(format!(
                "niente questione di fiducia su «{}»: la norma costituzionale passa da due \
                 deliberazioni e dal referendum",
                nome_misura(dati, m)
            ));
        }
        if m.fiducia {
            return ko(format!(
                "questione di fiducia già posta su «{}»",
                nome_misura(dati, m)
            ));
        }
        if occupato && matches!(m.forma, FormaMisura::Riforma | FormaMisura::Elettorale) {
            return ko(format!(
                "il bilancio occupa il calendario: niente questione di fiducia su «{}» in questo \
                 turno",
                nome_misura(dati, m)
            ));
        }
        m.fiducia = true;
        m.voto = turno;
        (m.emendamenti).retain(|e| e.stato != StatoEmendamento::Aperto);
        Ok(m)
    }
}

/// L'intesa che perde l'alleato in `posizione` se si pone la fiducia su una misura di forma
/// `forma` in posizione `nuova`: `costo_fiducia` più `costo_fiducia_distanza` per sesto di asse,
/// il doppio sull'elettorale.
pub(crate) fn costo_fiducia(
    dati: &DatiGioco,
    forma: FormaMisura,
    posizione: f64,
    nuova: Option<f64>,
) -> f64 {
    let d = nuova.map_or(0.0, |n| (posizione - n).abs());
    let doppio = if forma == FormaMisura::Elettorale {
        2.0
    } else {
        1.0
    };
    doppio * (dati.manopola("costo_fiducia") + dati.manopola("costo_fiducia_distanza") * d / 6.0)
}

/// Il motivo dell'alleato che perde `n` seggi attesi.
fn seggi_persi(n: i32) -> String {
    let seggi = if n == 1 { "seggio" } else { "seggi" };
    format!("perde {n} {seggi} con la legge elettorale")
}

/// La richiesta aperta di `partito` sul bilancio: indicizzare i `perdite` (dicastero, perdita
/// reale in punti).
pub(crate) fn richiesta_bilancio(
    dati: &DatiGioco,
    partito: usize,
    perdite: &[(usize, f64)],
) -> RichiestaEmendamento {
    let motivo = (perdite.iter())
        .filter_map(|&(k, x)| {
            let x = format!("{x:.1}").replace('.', ",");
            Some(format!("{}: −{x}% reale", dati.dicasteri().get(k)?.nome))
        })
        .collect::<Vec<_>>()
        .join("; ");
    RichiestaEmendamento {
        partito,
        proposta: (perdite.iter())
            .map(|&(k, _)| (k, Attenuazione::Indicizza))
            .collect(),
        stato: StatoEmendamento::Aperto,
        motivo,
    }
}

/// Un emendamento accolto rinvia la disposizione `d`.
pub(crate) fn rinviata(emendamenti: &[RichiestaEmendamento], d: usize) -> bool {
    (emendamenti.iter())
        .filter(|e| e.stato == StatoEmendamento::Accolto)
        .any(|e| e.proposta.contains(&(d, Attenuazione::Rinvio)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    #[test]
    fn un_seggio_perso_al_singolare() {
        assert_eq!(seggi_persi(1), "perde 1 seggio con la legge elettorale");
        assert_eq!(seggi_persi(3), "perde 3 seggi con la legge elettorale");
    }

    #[test]
    fn gradino_a_meta_bloccato_diventa_rinvio() {
        let dati = dati_base();
        let d = dati.disposizione("salario_minimo").expect("nei dati base");
        let g = |id: &str| {
            dati.disposizioni()[d]
                .gradino(id)
                .expect("gradino nei dati base")
        };
        let mut disposizioni = Disposizioni::iniziali(&dati);
        let valori = [(d, g("11"))];
        assert_eq!(
            proposta(&dati, &disposizioni, &valori),
            vec![(d, Attenuazione::Gradino(g("7")))]
        );
        disposizioni.voci[d].bloccati.push(g("7"));
        assert_eq!(
            proposta(&dati, &disposizioni, &valori),
            vec![(d, Attenuazione::Rinvio)]
        );
    }
}
