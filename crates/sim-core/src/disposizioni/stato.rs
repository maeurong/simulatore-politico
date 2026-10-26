//! Le disposizioni nella partita: gradino in vigore, legge, precedente e valori in arrivo.

use super::{DatoDisposizione, effetti_del_cambio, legge_da_gradini};
use crate::dati::DatiGioco;
use crate::elezioni::LeggeElettorale;
use crate::errore::Errore;
use crate::grandezze::EffettoAttivo;
use crate::partita::Trimestre;
use crate::vista::{VoceArrivo, VoceDisposizione, VoceGradino};
use serde::{Deserialize, Serialize};

/// Un valore in arrivo: il gradino, il turno d'apertura in cui arriva, la legge con cui arriva.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InArrivo {
    pub gradino: usize,
    pub turno: u32,
    pub legge: String,
}

/// Gradini per indice; `bloccati` = gradini abrogati da un referendum.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatoDisposizione {
    pub in_vigore: usize,
    pub precedente: usize,
    pub legge: String,
    pub in_arrivo: Vec<InArrivo>,
    pub bloccati: Vec<usize>,
}

/// Una voce per disposizione, nell'ordine dei dati.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disposizioni {
    pub voci: Vec<StatoDisposizione>,
}

/// Le nove elettorali stanno in coda al catalogo.
const ELETTORALI: usize = 9;

/// Trimestri dal turno zero a `(anno, trimestre)`.
fn turno_di(anno: u16, trimestre: u8) -> u32 {
    let t = |anno: u16, trimestre: u8| u32::from(anno) * 4 + u32::from(trimestre);
    let zero = Trimestre::TURNO_ZERO;
    t(anno, trimestre).saturating_sub(t(zero.anno, zero.trimestre))
}

/// Il trimestre di calendario del turno `turno`.
pub(crate) fn data_di(turno: u32) -> Trimestre {
    let zero = Trimestre::TURNO_ZERO;
    let q = (u32::from(zero.anno) * 4 + u32::from(zero.trimestre) - 1).saturating_add(turno);
    Trimestre {
        anno: u16::try_from(q / 4).unwrap_or(u16::MAX),
        trimestre: (q % 4) as u8 + 1,
    }
}

/// Indice del gradino `id`; `valida` garantisce che ci sia.
fn indice(d: &DatoDisposizione, id: &str) -> usize {
    d.gradino(id).unwrap_or(0)
}

impl Disposizioni {
    /// Dai dati: partenza, precedente, legge e arrivi, col turno di ogni arrivo contato dal
    /// turno zero e la legge della disposizione.
    pub fn iniziali(dati: &DatiGioco) -> Disposizioni {
        let voci = (dati.disposizioni().iter())
            .map(|d| StatoDisposizione {
                in_vigore: indice(d, &d.partenza),
                precedente: indice(d, &d.precedente),
                legge: d.legge.clone(),
                in_arrivo: (d.in_arrivo.iter())
                    .map(|a| InArrivo {
                        gradino: indice(d, &a.gradino),
                        turno: turno_di(a.anno, a.trimestre),
                        legge: d.legge.clone(),
                    })
                    .collect(),
                bloccati: Vec::new(),
            })
            .collect();
        Disposizioni { voci }
    }

    /// Il gradino `g` entra in vigore nella disposizione `d` al turno `turno` con la legge
    /// `legge`, coi suoi effetti aggiunti a `effetti`; gli arrivi restano. Con `g` già in vigore
    /// cambia solo la legge.
    ///
    /// # Panics
    /// Se `d` non è l'indice di una voce.
    fn applica(
        &mut self,
        dati: &DatiGioco,
        d: usize,
        g: usize,
        turno: u32,
        legge: &str,
        effetti: &mut Vec<EffettoAttivo>,
    ) {
        let v = &mut self.voci[d];
        v.legge = legge.into();
        if g == v.in_vigore {
            return;
        }
        v.precedente = v.in_vigore;
        v.in_vigore = g;
        effetti_del_cambio(dati, d, v.precedente, g, turno, effetti);
    }

    /// Una legge porta la disposizione `d` al gradino `g` e cancella i valori in arrivo.
    ///
    /// # Panics
    /// Se `d` non è l'indice di una voce.
    pub(crate) fn cambia(
        &mut self,
        dati: &DatiGioco,
        d: usize,
        g: usize,
        turno: u32,
        legge: &str,
        effetti: &mut Vec<EffettoAttivo>,
    ) {
        self.applica(dati, d, g, turno, legge, effetti);
        self.voci[d].in_arrivo.clear();
    }

    /// Gli arrivi di `d` diventano uno solo: `g` all'apertura del turno `turno`, con `legge`.
    ///
    /// # Errors
    /// `Comando` se `d` non è una disposizione dei dati o `g` non è un suo gradino.
    pub fn programma(
        &mut self,
        dati: &DatiGioco,
        d: usize,
        g: usize,
        turno: u32,
        legge: &str,
    ) -> Result<(), Errore> {
        let (Some(disp), Some(v)) = (dati.disposizioni().get(d), self.voci.get_mut(d)) else {
            return Err(Errore::Comando(format!("disposizione {d} assente")));
        };
        if g >= disp.gradini.len() {
            return Err(Errore::Comando(format!("{}: gradino {g} assente", disp.id)));
        }
        v.in_arrivo = vec![InArrivo {
            gradino: g,
            turno,
            legge: legge.into(),
        }];
        Ok(())
    }

    /// Apertura del turno `turno`: ogni arrivo dovuto entra in vigore, in ordine, con la sua
    /// legge, ed esce dalla lista; gli effetti vanno in `effetti`.
    pub fn arrivi(&mut self, dati: &DatiGioco, turno: u32, effetti: &mut Vec<EffettoAttivo>) {
        for d in 0..self.voci.len() {
            let dovuti = (self.voci[d].in_arrivo.iter())
                .take_while(|a| a.turno <= turno)
                .count();
            let arrivati: Vec<InArrivo> = self.voci[d].in_arrivo.drain(..dovuti).collect();
            for a in arrivati {
                self.applica(dati, d, a.gradino, turno, &a.legge, effetti);
            }
        }
    }

    /// La legge elettorale dei gradini in vigore delle nove disposizioni elettorali.
    pub fn legge_elettorale(&self, dati: &DatiGioco) -> LeggeElettorale {
        let mut g = [0; ELETTORALI];
        let coda = &self.voci[self.voci.len().saturating_sub(ELETTORALI)..];
        for (x, v) in g.iter_mut().zip(coda) {
            *x = v.in_vigore;
        }
        legge_da_gradini(&dati.elezioni().menu, g)
    }

    /// Posizione sull'asse 1-7 del gradino in vigore della disposizione `d`.
    pub fn posizione(&self, dati: &DatiGioco, d: usize) -> Option<f64> {
        dati.disposizioni()
            .get(d)?
            .posizione(self.voci.get(d)?.in_vigore)
    }

    /// Le voci per la `Vista`, nell'ordine dei dati.
    pub(crate) fn voci_vista(&self, dati: &DatiGioco) -> Vec<VoceDisposizione> {
        (dati.disposizioni().iter().zip(&self.voci))
            .map(|(d, v)| {
                let id = |g: usize| d.gradini.get(g).map(|x| x.id.clone()).unwrap_or_default();
                VoceDisposizione {
                    id: d.id.clone(),
                    nome: d.nome.clone(),
                    dicastero: d.dicastero.clone(),
                    norma: d.norma,
                    tipo: d.tipo,
                    in_vigore: id(v.in_vigore),
                    legge: v.legge.clone(),
                    precedente: id(v.precedente),
                    in_arrivo: (v.in_arrivo.iter())
                        .map(|a| VoceArrivo {
                            gradino: id(a.gradino),
                            data: data_di(a.turno),
                        })
                        .collect(),
                    referendabile: d.referendabile,
                    perimetro: d.perimetro.keys().cloned().collect(),
                    gradini: (d.gradini.iter().enumerate())
                        .map(|(k, g)| VoceGradino {
                            id: g.id.clone(),
                            nome: g.nome.clone(),
                            posizione: g.posizione,
                            bloccato: v.bloccati.contains(&k),
                        })
                        .collect(),
                }
            })
            .collect()
    }

    /// Lo stato di un salvataggio contro i dati.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` se le voci non sono quante le disposizioni, un gradino (in
    /// vigore, precedente, in arrivo, bloccato) è fuori dai gradini, gli arrivi non sono in
    /// ordine di turno o la legge elettorale in vigore non è valida.
    pub fn controlla(&self, dati: &DatiGioco) -> Result<(), Errore> {
        let ko = |m: String| Err(Errore::SalvataggioIlleggibile(format!("disposizioni: {m}")));
        let catalogo = dati.disposizioni();
        if self.voci.len() != catalogo.len() {
            return ko(format!(
                "{} voci, le disposizioni sono {}",
                self.voci.len(),
                catalogo.len()
            ));
        }
        for (d, v) in catalogo.iter().zip(&self.voci) {
            let n = d.gradini.len();
            let gradini = [v.in_vigore, v.precedente]
                .into_iter()
                .chain(v.in_arrivo.iter().map(|a| a.gradino))
                .chain(v.bloccati.iter().copied());
            for g in gradini {
                if g >= n {
                    return ko(format!("{}: gradino {g} fuori dai {n}", d.id));
                }
            }
            if !v.in_arrivo.windows(2).all(|w| w[0].turno < w[1].turno) {
                return ko(format!("{}: valori in arrivo non in ordine di turno", d.id));
            }
        }
        (self.legge_elettorale(dati))
            .valida(&dati.elezioni().menu)
            .or_else(|e| ko(format!("legge elettorale in vigore: {e}")))
    }
}
