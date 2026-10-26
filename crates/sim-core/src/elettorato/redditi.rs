//! Redditi dei gruppi, panieri e Gini: derivati dall'economia, mai stato indipendente.

use super::{Condizione, Elettorato, Fascia};
use crate::dati::DatiGioco;
use crate::disposizioni::Leve;
use crate::economia::{Economia, Scaglione, irpef};
use crate::misure::Misure;
use serde::{Deserialize, Serialize};

/// Euro annui nominali per elettore. `paniere` è l'indice di prezzo della fascia:
/// `livello_prezzi · Σ paniere_f·prezzi / Σ consumi·prezzi`. Al turno zero vale esattamente 1
/// finché il paniere della fascia è il vettore `consumi` (PR A); con panieri diversi (PR B) vale 1
/// a meno dello scarto dei prezzi di Leontief iniziali da 1, cioè degli arrotondamenti della tavola.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Reddito {
    pub primario: f64,
    pub trasferimenti: f64,
    pub imposte: f64,
    pub paniere: f64,
}

impl Reddito {
    pub fn netto(&self) -> f64 {
        self.primario + self.trasferimenti - self.imposte
    }

    pub fn reale(&self) -> f64 {
        self.netto() / self.paniere
    }

    pub(super) fn e_valido(&self) -> bool {
        [self.primario, self.trasferimenti, self.imposte]
            .iter()
            .all(|x| x.is_finite())
            && self.paniere.is_finite()
            && self.paniere > 0.0
    }
}

/// Grandezze dell'economia che muovono i redditi.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Indici {
    pub salario: f64,
    pub salari_pubblici: f64,
    pub pil: f64,
    pub pensione: f64,
    pub sussidio: f64,
}

impl Indici {
    pub fn da(e: &Economia, dati: &DatiGioco, leve: &Leve) -> Indici {
        Indici {
            salario: e.salario_medio(dati),
            salari_pubblici: e.bilancio.dicasteri.iter().map(|d| d.salari as f64).sum(),
            pil: e.pil_trimestrale,
            pensione: e.pensione_media(),
            sussidio: e.sussidio_effettivo(leve),
        }
    }

    pub(super) fn e_finito(&self) -> bool {
        [
            self.salario,
            self.salari_pubblici,
            self.pil,
            self.pensione,
            self.sussidio,
        ]
        .iter()
        .all(|x| x.is_finite())
    }
}

impl Elettorato {
    /// `reddito_prima ← reddito`, poi il reddito nuovo dallo stato dell'economia. La riscossione
    /// delle misure pesa anche sulle imposte degli autonomi; le leve delle disposizioni, rispetto
    /// a quelle di partenza, muovono salario minimo, reddito minimo, cuneo e forfettario.
    pub fn aggiorna_redditi(&mut self, dati: &DatiGioco, e: &Economia, misure: &Misure) {
        let (l, l0) = (&misure.leve, &Leve::partenza(dati));
        let (i, b) = (Indici::da(e, dati, l), self.base);
        let (ore, copertura) = (
            dati.manopola("ore_salario_minimo"),
            dati.manopola("copertura_salario_minimo"),
        );
        let d = dati.elettorato();
        let consumi = &dati.struttura().consumi;
        let media =
            |w: &[f64]| -> f64 { w.iter().zip(&e.prezzi_settori).map(|(w, p)| w * p).sum() };
        let costo_base = media(consumi);
        let s0 = &dati.scenario().aliquote;
        // ponytail: base nulla (per esempio tutti i salari_pil a 0) = nessun termine di
        // confronto, il reddito resta quello dei dati invece di diventare NaN.
        let rapporto = |oggi: f64, base: f64| if base > 0.0 { oggi / base } else { 1.0 };
        for (g, dato) in self.gruppi.iter_mut().zip(&d.gruppi) {
            g.reddito_prima = g.reddito;
            let (c, bassa) = (dato.condizione, dato.fascia == Fascia::Bassa);
            let mut primario = dato.primario
                * match dato.condizione {
                    Condizione::DipendentePubblico => {
                        rapporto(i.salari_pubblici, b.salari_pubblici)
                    }
                    Condizione::Autonomo => rapporto(i.pil, b.pil),
                    _ => rapporto(i.salario, b.salario),
                };
            let mut trasferimenti = dato.trasferimenti
                * match dato.condizione {
                    Condizione::Pensionato => rapporto(i.pensione, b.pensione),
                    Condizione::DisoccupatoInattivo => rapporto(i.sussidio, b.sussidio),
                    _ => 1.0,
                };
            g.lordo_irpef = primario + trasferimenti;
            if c == Condizione::DipendentePrivato && bassa {
                primario += copertura * (l.salario_minimo * ore - primario).max(0.0);
            }
            if c == Condizione::DisoccupatoInattivo && bassa {
                trasferimenti = (trasferimenti + (l.reddito_minimo - l0.reddito_minimo)).max(0.0);
            }
            let lordo = primario + trasferimenti;
            let aliquota = d.aliquote.get(&dato.fascia).copied().unwrap_or_default();
            let paniere = d.paniere.get(&dato.fascia).map_or(0.0, |w| media(w));
            // Aliquota effettiva del gruppo (IRPEF + contributi) oggi su quella del turno zero:
            // il drenaggio nasce qui, col lordo nominale che cresce e gli scaglioni fermi.
            let r = {
                let tasso = |s: &[Scaglione], l: f64, contributi: f64| irpef(s, l) / l + contributi;
                let lordo_0 = dato.primario + dato.trasferimenti;
                let den = if lordo_0 > 0.0 {
                    tasso(&s0.irpef, lordo_0, s0.contributi)
                } else {
                    0.0
                };
                if lordo > 0.0 && den > 0.0 {
                    tasso(
                        &e.bilancio.aliquote.irpef,
                        lordo,
                        e.bilancio.aliquote.contributi,
                    ) / den
                } else {
                    1.0
                }
            };
            let evasione = if dato.condizione == Condizione::Autonomo {
                misure.riscossione
            } else {
                1.0
            };
            let mut imposte = aliquota * lordo * r * evasione;
            match c {
                Condizione::DipendentePrivato | Condizione::DipendentePubblico
                    if dato.fascia != Fascia::Alta =>
                {
                    imposte -= (l.cuneo - l0.cuneo) / 100.0 * primario;
                }
                Condizione::Autonomo => imposte *= rapporto(l.forfettario, l0.forfettario),
                _ => {}
            }
            g.reddito = Reddito {
                primario,
                trasferimenti,
                imposte: imposte.min(lordo).max(0.0),
                paniere: e.livello_prezzi * paniere / costo_base,
            };
        }
    }

    /// Gini del reddito reale, pesato con gli elettori dei gruppi.
    pub fn gini(&self, dati: &DatiGioco) -> f64 {
        let d = dati.elettorato();
        let punti: Vec<(f64, f64)> = self
            .gruppi
            .iter()
            .enumerate()
            .map(|(k, g)| (self.elettori(d, k), g.reddito.reale()))
            .collect();
        let n: f64 = punti.iter().map(|p| p.0).sum();
        let media = punti.iter().map(|p| p.0 * p.1).sum::<f64>() / n;
        let somma: f64 = punti
            .iter()
            .flat_map(|a| punti.iter().map(move |b| a.0 * b.0 * (a.1 - b.1).abs()))
            .sum();
        // ponytail: O(gruppi²) = 130k coppie, una volta a turno; ordinare e cumulare se pesa.
        if n > 0.0 && media > 0.0 {
            somma / (2.0 * n * n * media)
        } else {
            0.0
        }
    }
}
