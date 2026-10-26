//! Ciò che il giocatore vede dell'elettorato e dell'economia: sondaggio e prime stime, con
//! errori. Lo stato vero (bias, effetti d'istituto, valori veri) non passa mai dalla `Vista`.

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::elettorato::{Condizione, Elettorato, Fascia, Intenzioni, PUNTI};
use crate::errore::Errore;
use crate::partiti::{Governo, Partito};
use serde::{Deserialize, Serialize};

mod campione;
pub(crate) use campione::indice;
mod sondaggio;
mod statistica;
pub use campione::{ElettoreCampione, Motivo, genera};
pub use sondaggio::{Incrocio, Rilevazione, Sondaggio};
pub use statistica::{Revisione, Statistica, Stima};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Informazione {
    /// Bias comune per alternativa (partiti, poi astensione), una volta per legislatura.
    pub bias: Vec<f64>,
    /// Effetto di ogni istituto per alternativa.
    pub effetti_istituto: Vec<Vec<f64>>,
    pub sondaggio: Sondaggio,
    pub statistica: Statistica,
}

impl Informazione {
    /// Bias e effetti d'istituto, poi il primo sondaggio e la statistica del turno zero.
    pub fn nuova(
        dati: &DatiGioco,
        caso: &mut Caso,
        el: &Elettorato,
        partiti: &[Partito],
        governo: &Governo,
        economia: &Economia,
    ) -> Informazione {
        let alt = partiti.len() + 1;
        let (s_bias, s_ist) = (
            dati.manopola("sigma_bias_comune"),
            dati.manopola("sigma_istituto"),
        );
        let bias: Vec<f64> = (0..alt)
            .map(|_| s_bias * caso.normale(Flusso::Sondaggi))
            .collect();
        let effetti_istituto: Vec<Vec<f64>> = dati
            .elettorato()
            .istituti
            .iter()
            .map(|_| {
                (0..alt)
                    .map(|_| s_ist * caso.normale(Flusso::Sondaggi))
                    .collect()
            })
            .collect();
        let vero = Intenzioni::calcola(dati, el, partiti, governo);
        let sondaggio = sondaggio::rileva(
            dati,
            caso,
            el,
            &vero,
            governo.presidente,
            &bias,
            &effetti_istituto,
        );
        Informazione {
            bias,
            effetti_istituto,
            sondaggio,
            statistica: Statistica::iniziale(dati, economia),
        }
    }

    /// Nuovo sondaggio dalle intenzioni vere.
    pub(crate) fn rileva(
        &mut self,
        dati: &DatiGioco,
        caso: &mut Caso,
        el: &Elettorato,
        vero: &Intenzioni,
        governo: &Governo,
    ) {
        self.sondaggio = sondaggio::rileva(
            dati,
            caso,
            el,
            vero,
            governo.presidente,
            &self.bias,
            &self.effetti_istituto,
        );
    }

    /// Lunghezze e finitezza, per uno stato letto da un salvataggio.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` col campo nel messaggio.
    pub fn controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore> {
        let (d, alt) = (dati.elettorato(), n_partiti + 1);
        let ko = |m: &str| Err(Errore::SalvataggioIlleggibile(format!("informazione.{m}")));
        let giusta = |v: &[f64], n: usize| v.len() == n && v.iter().all(|x| x.is_finite());
        if !giusta(&self.bias, alt) {
            return ko(&format!("bias: servono {alt} valori finiti"));
        }
        if self.effetti_istituto.len() != d.istituti.len()
            || !self.effetti_istituto.iter().all(|h| giusta(h, alt))
        {
            return ko(&format!(
                "effetti_istituto: servono {} righe di {alt} valori finiti",
                d.istituti.len()
            ));
        }
        let s = &self.sondaggio;
        if s.istituti.len() != d.istituti.len() || !s.istituti.iter().all(|r| giusta(&r.quote, alt))
        {
            return ko("sondaggio.istituti non combacia con gli istituti dei dati");
        }
        let incroci = [
            (&s.condizioni, Condizione::TUTTE.len(), "condizioni"),
            (&s.fasce, Fascia::TUTTE.len(), "fasce"),
            (&s.regioni, d.regioni.len(), "regioni"),
        ];
        for (v, n, nome) in incroci {
            let ok = v.len() == n
                && v.iter().all(|i| {
                    giusta(&i.quote, alt)
                        && giusta(&i.posizione, PUNTI)
                        && i.approvazione.is_finite()
                });
            if !ok {
                return ko(&format!("sondaggio.{nome}: forma o valori non validi"));
            }
        }
        let t = &self.statistica;
        let trimestri_ok = (1..=4).contains(&t.trimestre.trimestre)
            && t.revisione
                .as_ref()
                .is_none_or(|r| (1..=4).contains(&r.trimestre.trimestre));
        let stime = [&t.prima_stima, &t.vera].into_iter().chain(
            t.revisione
                .iter()
                .flat_map(|r| [&r.prima_stima, &r.riveduta]),
        );
        if !trimestri_ok || !stime.into_iter().all(Stima::e_finita) {
            return ko("statistica: trimestre fuori da 1-4 o stima non finita");
        }
        Ok(())
    }
}
