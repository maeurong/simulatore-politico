//! La regola di movimento dei partiti IA nell'apertura della campagna (scelta 4 del piano 8b,
//! abm-elettorali:14-18).

use crate::dati::DatiGioco;
use crate::informazione::Sondaggio;
use crate::partiti::Regola;

/// Quello che la regola legge. `baricentro`: degli elettori del partito, `None` senza elettori.
/// `guadagno`: quota fra i votanti dell'ultimo voto contro quella del voto prima, `None` senza due
/// voti alle spalle. `posizione_governo`: media delle dichiarate della coalizione di governo
/// pesata coi seggi.
pub(crate) struct Contesto {
    pub centro: f64,
    pub baricentro: Option<f64>,
    pub guadagno: Option<bool>,
    pub ultimo_passo: f64,
    pub al_governo: bool,
    pub posizione_governo: f64,
}

/// Lo spostamento della dichiarata `x` secondo la regola `r`, al più `massimo` in valore assoluto.
/// Sticker fermo; Aggregator verso il baricentro; Hunter ripete l'ultimo passo dopo un guadagno,
/// lo inverte dopo una perdita, senza due voti va verso il centro; Governator fermo al governo,
/// altrimenti verso la posizione del governo senza superarla.
pub(crate) fn passo(r: Regola, x: f64, c: &Contesto, massimo: f64) -> f64 {
    let limite = |d: f64| d.clamp(-massimo, massimo);
    let verso = |y: f64| limite(y - x);
    match r {
        Regola::Sticker => 0.0,
        Regola::Aggregator => c.baricentro.map_or(0.0, verso),
        Regola::Hunter => match c.guadagno {
            Some(true) => limite(c.ultimo_passo),
            Some(false) => limite(-c.ultimo_passo),
            None => verso(c.centro),
        },
        Regola::Governator if c.al_governo => 0.0,
        Regola::Governator => verso(c.posizione_governo),
    }
}

/// La posizione nuova da `x` col `passo`: mai oltre il primo degli `altri` nella direzione del
/// passo (uguale ammessa), mai fuori da 1-7.
pub(crate) fn limita(x: f64, passo: f64, altri: &[f64]) -> f64 {
    let y = x + passo;
    let fermo = if passo > 0.0 {
        (altri.iter().copied()).filter(|&a| a > x).fold(y, f64::min)
    } else {
        (altri.iter().copied()).filter(|&a| a < x).fold(y, f64::max)
    };
    fermo.clamp(1.0, 7.0)
}

/// Il baricentro degli elettori del partito `j` sul sondaggio pubblico, come la stima del
/// referendum IA (`elettorato/referendum.rs:127-150`): per regione la posizione media
/// dell'incrocio, pesata con la quota del partito nell'incrocio e con gli elettori della regione
/// nei dati. `None` se il partito non ha quota in nessuna regione.
// ponytail: l'incrocio regionale non incrocia posizione e voto; la posizione media è di tutta
// la regione. Via d'uscita: un istogramma per partito negli incroci del sondaggio.
pub(crate) fn baricentro(dati: &DatiGioco, sondaggio: &Sondaggio, j: usize) -> Option<f64> {
    let (mut somma, mut peso) = (0.0, 0.0);
    for (regione, incrocio) in dati.elettorato().regioni.iter().zip(&sondaggio.regioni) {
        let h: f64 = incrocio.posizione.iter().sum();
        if h <= 0.0 {
            continue;
        }
        let punti = incrocio.posizione.iter().enumerate();
        let media = punti.map(|(k, h)| (k + 1) as f64 * h).sum::<f64>() / h;
        let w = incrocio.quote.get(j).copied().unwrap_or(0.0) * regione.elettori as f64;
        somma += w * media;
        peso += w;
    }
    (peso > 0.0).then(|| somma / peso)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn contesto() -> Contesto {
        Contesto {
            centro: 4.0,
            baricentro: None,
            guadagno: None,
            ultimo_passo: 0.0,
            al_governo: false,
            posizione_governo: 4.0,
        }
    }

    fn vicino(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn sticker_fermo() {
        let c = Contesto {
            baricentro: Some(5.0),
            ..contesto()
        };
        assert_eq!(passo(Regola::Sticker, 2.0, &c, 0.5), 0.0);
    }

    #[test]
    fn aggregator_verso_il_baricentro() {
        let c = Contesto {
            baricentro: Some(2.3),
            ..contesto()
        };
        assert!(vicino(2.0 + passo(Regola::Aggregator, 2.0, &c, 0.5), 2.3));
        let c = Contesto {
            baricentro: Some(4.0),
            ..contesto()
        };
        assert!(vicino(2.0 + passo(Regola::Aggregator, 2.0, &c, 0.5), 2.5));
        assert_eq!(passo(Regola::Aggregator, 2.0, &contesto(), 0.5), 0.0);
    }

    #[test]
    fn hunter_ripete_inverte_o_va_al_centro() {
        let c = |guadagno| Contesto {
            guadagno,
            ultimo_passo: -0.5,
            ..contesto()
        };
        assert!(vicino(
            passo(Regola::Hunter, 3.0, &c(Some(true)), 0.5),
            -0.5
        ));
        assert!(vicino(
            passo(Regola::Hunter, 3.0, &c(Some(false)), 0.5),
            0.5
        ));
        assert!(vicino(passo(Regola::Hunter, 3.0, &c(None), 0.5), 0.5));
        assert!(vicino(passo(Regola::Hunter, 5.0, &c(None), 0.5), -0.5));
    }

    #[test]
    fn governator_fermo_al_governo_altrimenti_verso_il_governo() {
        let c = |al_governo| Contesto {
            al_governo,
            posizione_governo: 4.8,
            ..contesto()
        };
        assert_eq!(passo(Regola::Governator, 5.0, &c(true), 0.5), 0.0);
        assert!(vicino(
            5.0 + passo(Regola::Governator, 5.0, &c(false), 0.5),
            4.8
        ));
    }

    #[test]
    fn limita_al_vicino_e_all_asse() {
        assert!(vicino(limita(3.0, 0.5, &[3.2, 1.0]), 3.2));
        assert!(vicino(limita(3.0, -0.5, &[3.2, 2.8]), 2.8));
        assert!(vicino(limita(3.0, 0.5, &[2.8, 4.0]), 3.5));
        assert!(vicino(limita(6.8, 0.5, &[]), 7.0));
        assert!(vicino(limita(1.2, -0.5, &[]), 1.0));
        assert!(vicino(limita(3.0, 0.0, &[3.0]), 3.0));
    }
}
