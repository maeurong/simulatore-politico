//! La regola di movimento dei partiti IA nell'apertura della campagna (scelta 4 del piano 8b,
//! abm-elettorali:14-18).

use crate::dati::DatiGioco;
use crate::elettorato::{PUNTI, softmax};
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
/// Sticker fermo; Aggregator verso il baricentro; Hunter, a passo pieno, ripete la direzione
/// dell'ultimo passo dopo un guadagno e la inverte dopo una perdita (abm-elettorali:16), senza
/// direzione resta fermo, senza due voti va verso il centro; Governator fermo al governo,
/// altrimenti verso la posizione del governo senza superarla.
pub(crate) fn passo(r: Regola, x: f64, c: &Contesto, massimo: f64) -> f64 {
    let limite = |d: f64| d.clamp(-massimo, massimo);
    let verso = |y: f64| limite(y - x);
    let direzione = if c.ultimo_passo == 0.0 {
        0.0
    } else {
        massimo.copysign(c.ultimo_passo)
    };
    match r {
        Regola::Sticker => 0.0,
        Regola::Aggregator => c.baricentro.map_or(0.0, verso),
        Regola::Hunter => match c.guadagno {
            Some(true) => direzione,
            Some(false) => -direzione,
            None => verso(c.centro),
        },
        Regola::Governator if c.al_governo => 0.0,
        Regola::Governator => verso(c.posizione_governo),
    }
}

/// La posizione nuova del partito `j` da `x` col `passo`, dentro 1-7, senza cambiare l'ordine della
/// fila per (dichiarata, indice): si ferma sul primo degli `altri` (indice, dichiarata) che lo
/// segue nella direzione del passo, anche alla stessa posizione; arriva sulla sua posizione solo
/// se gli indici tengono l'ordine, altrimenti un soffio prima.
pub(crate) fn limita(j: usize, x: f64, passo: f64, altri: &[(usize, f64)]) -> f64 {
    let y = (x + passo).clamp(1.0, 7.0);
    let dopo = |k: usize, a: f64| a > x || (a == x && k > j);
    // ponytail: «un soffio prima» = il float accanto; a video resta la stessa cifra del vicino.
    if passo > 0.0 {
        (altri.iter().filter(|&&(k, a)| dopo(k, a)))
            .map(|&(k, a)| if k > j { a } else { a.next_down() })
            .fold(y, f64::min)
    } else if passo < 0.0 {
        (altri.iter().filter(|&&(k, a)| !dopo(k, a)))
            .map(|&(k, a)| if k < j { a } else { a.next_up() })
            .fold(y, f64::max)
    } else {
        x
    }
}

/// L'ultimo passo che Hunter ricorda dopo lo spostamento da `x` a `y`: il passo `richiesto`, anche
/// se `limita` l'ha accorciato; rovesciato se l'ha annullato, così il partito fermo sul vicino o
/// sul bordo torna indietro.
pub(crate) fn ultimo_passo(x: f64, y: f64, richiesto: f64) -> f64 {
    if y == x { -richiesto } else { richiesto }
}

/// Gli elettori a ogni punto dell'asse sul sondaggio pubblico, come la stima del referendum IA
/// (`elettorato/referendum.rs:127-150`): Σ_r w_r·h_rk, con h_rk l'istogramma `posizione`
/// dell'incrocio regionale e w_r gli elettori della regione nei dati.
pub(crate) fn istogramma(dati: &DatiGioco, sondaggio: &Sondaggio) -> [f64; PUNTI] {
    let mut h = [0.0; PUNTI];
    for (regione, incrocio) in dati.elettorato().regioni.iter().zip(&sondaggio.regioni) {
        for (h, x) in h.iter_mut().zip(&incrocio.posizione) {
            *h += regione.elettori as f64 * x;
        }
    }
    h
}

/// Il baricentro degli elettori del partito `j`, coi soli dati pubblici. A ogni punto
/// k dell'asse P(j|k), il softmax di −`beta`·(k − x_i)² sulle `dichiarate` x_i (`None` = sciolto,
/// escluso), stima chi vota il partito a quel punto. Baricentro = Σ k·h_k·P(j|k) / Σ h_k·P(j|k)
/// sull'`istogramma`; `None` se il peso è zero.
pub(crate) fn baricentro(
    beta: f64,
    istogramma: &[f64],
    dichiarate: &[Option<f64>],
    j: usize,
) -> Option<f64> {
    let (mut somma, mut peso) = (0.0, 0.0);
    for (k, h) in istogramma.iter().enumerate() {
        let x = (k + 1) as f64;
        let u: Vec<f64> = (dichiarate.iter())
            .map(|d| d.map_or(f64::NEG_INFINITY, |d| -beta * (x - d).powi(2)))
            .collect();
        let w = h * softmax(&u).get(j).copied().unwrap_or(0.0);
        somma += w * x;
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
        // Un ultimo passo accorciato (−0,1) dà la direzione, non l'ampiezza.
        let c = |guadagno| Contesto {
            guadagno,
            ultimo_passo: -0.1,
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
        let fermo = Contesto {
            guadagno: Some(true),
            ..contesto()
        };
        assert_eq!(passo(Regola::Hunter, 4.0, &fermo, 0.5), 0.0);
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
    fn baricentro_sui_partiti_vicini() {
        let dichiarate = [Some(2.0), Some(4.0), Some(6.0)];
        let uniforme = [1.0; 7];
        let b = |j| baricentro(0.186, &uniforme, &dichiarate, j).unwrap();
        assert!(b(0) < 3.0, "{}", b(0));
        assert!(b(2) > 5.0, "{}", b(2));
        assert!(vicino(b(1), 4.0));
        // Lo sciolto non vota: senza il partito a 2, quello a 4 prende anche la sinistra.
        let senza = [None, Some(4.0), Some(6.0)];
        assert!(baricentro(0.186, &uniforme, &senza, 1).unwrap() < b(1));
        assert_eq!(baricentro(0.186, &[0.0; 7], &dichiarate, 0), None);
    }

    #[test]
    fn limita_al_vicino_e_all_asse() {
        // Il partito 3 a 3,0.
        let l = |passo: f64, altri: &[(usize, f64)]| limita(3, 3.0, passo, altri);
        assert_eq!(l(0.5, &[(5, 3.2), (0, 1.0)]), 3.2);
        assert_eq!(l(-0.5, &[(5, 3.2), (0, 2.8)]), 2.8);
        assert_eq!(l(0.5, &[(5, 2.8), (0, 4.0)]), 3.5);
        assert_eq!(limita(3, 6.8, 0.5, &[]), 7.0);
        assert_eq!(limita(3, 1.2, -0.5, &[]), 1.0);
        assert_eq!(l(0.0, &[(5, 3.0)]), 3.0);
    }

    /// L'ordine della fila per (dichiarata, indice) tiene: alla stessa posizione ferma chi lo segue
    /// nella direzione del passo, non chi lo precede; sulla posizione di un vicino con l'indice
    /// che invertirebbe l'ordine si ferma un soffio prima.
    #[test]
    fn limita_non_scavalca_nemmeno_dal_pareggio() {
        let l = |passo: f64, altri: &[(usize, f64)]| limita(3, 3.0, passo, altri);
        assert_eq!(l(0.5, &[(5, 3.0)]), 3.0);
        assert_eq!(l(-0.5, &[(5, 3.0)]), 2.5);
        assert_eq!(l(-0.5, &[(1, 3.0)]), 3.0);
        assert_eq!(l(0.5, &[(1, 3.0)]), 3.5);
        let y = l(0.5, &[(1, 3.2)]);
        assert!(y < 3.2 && vicino(y, 3.2), "{y}");
        let y = l(-0.5, &[(5, 2.8)]);
        assert!(y > 2.8 && vicino(y, 2.8), "{y}");
    }

    /// Hunter al bordo 7 dopo un guadagno: il passo richiesto (+0,5) si annulla, l'ultimo passo
    /// diventa −0,5, e al guadagno dopo rimbalza verso il centro.
    #[test]
    fn hunter_al_bordo_rimbalza() {
        let c = |ultimo_passo| Contesto {
            guadagno: Some(true),
            ultimo_passo,
            ..contesto()
        };
        let richiesto = passo(Regola::Hunter, 7.0, &c(0.5), 0.5);
        let y = limita(6, 7.0, richiesto, &[]);
        assert_eq!(y, 7.0);
        let ultimo = ultimo_passo(7.0, y, richiesto);
        assert_eq!(ultimo, -0.5);
        let richiesto = passo(Regola::Hunter, 7.0, &c(ultimo), 0.5);
        assert_eq!(limita(6, 7.0, richiesto, &[]), 6.5);
        assert_eq!(ultimo_passo(7.0, 6.5, richiesto), -0.5);
    }
}
