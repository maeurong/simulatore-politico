//! Eventi puntuali (emergenze, scandali, shock): una uniforme per candidato, sempre, e al massimo
//! un puntuale a turno, il più grave.

use super::{Bersaglio, Causa, Tono};
use crate::caso::{Caso, Flusso};
use crate::vista::Fascia3;

/// Un evento che può nascere in questo turno con probabilità `p`.
pub(crate) struct Candidato {
    pub evento: usize,
    pub p: f64,
    pub gravita: u8,
    pub regione: Option<usize>,
    pub bersaglio: Option<Bersaglio>,
    pub causa: Causa,
    pub costo: i64,
    pub tono: Tono,
}

/// Il livello di rischio per la causa: la vista non mostra mai la probabilità in cifre.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "lo usano le famiglie dei task 6-8")
)]
pub(crate) fn livello(p: f64) -> Fascia3 {
    if p < 0.05 {
        Fascia3::Poco
    } else if p < 0.2 {
        Fascia3::Medio
    } else {
        Fascia3::Molto
    }
}

/// Una uniforme da `Flusso::Eventi` per candidato, nell'ordine, a prescindere dagli esiti: esce chi
/// ha `u < p`. Restituisce l'indice del vincitore (gravità più alta, a parità il primo) e quelli
/// degli altri usciti.
pub(crate) fn estrai(caso: &mut Caso, candidati: &[Candidato]) -> (Option<usize>, Vec<usize>) {
    let usciti: Vec<usize> = (candidati.iter().enumerate())
        .filter_map(|(i, c)| (caso.uniforme(Flusso::Eventi) < c.p).then_some(i))
        .collect();
    let vincitore = (usciti.iter().copied()).reduce(|v, i| {
        if candidati[i].gravita > candidati[v].gravita {
            i
        } else {
            v
        }
    });
    let altri = usciti
        .into_iter()
        .filter(|&i| Some(i) != vincitore)
        .collect();
    (vincitore, altri)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(p: f64, gravita: u8) -> Candidato {
        Candidato {
            evento: 0,
            p,
            gravita,
            regione: None,
            bersaglio: None,
            causa: Causa::Rischio {
                livello: livello(p),
            },
            costo: 0,
            tono: Tono::Ordinario,
        }
    }

    #[test]
    fn vince_il_piu_grave() {
        let mut caso = Caso::nuovo(1);
        let (v, altri) = estrai(&mut caso, &[cand(1.0, 2), cand(1.0, 5), cand(1.0, 3)]);
        assert_eq!((v, altri), (Some(1), vec![0, 2]));
    }

    #[test]
    fn a_parita_vince_il_primo() {
        let mut caso = Caso::nuovo(1);
        let (v, altri) = estrai(&mut caso, &[cand(1.0, 4), cand(1.0, 4)]);
        assert_eq!((v, altri), (Some(0), vec![1]));
    }

    #[test]
    fn estrae_sempre_tutti() {
        let mut a = Caso::nuovo(7);
        let mut b = Caso::nuovo(7);
        let (v, _) = estrai(&mut a, &[cand(1.0, 1), cand(0.0, 9), cand(1.0, 1)]);
        assert_eq!(v, Some(0));
        for _ in 0..3 {
            b.uniforme(Flusso::Eventi);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn livello_alle_soglie() {
        assert_eq!(livello(0.049), Fascia3::Poco);
        assert_eq!(livello(0.05), Fascia3::Medio);
        assert_eq!(livello(0.2), Fascia3::Molto);
    }
}
