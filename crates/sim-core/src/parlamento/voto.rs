//! Voto in aula: linee dei partiti, probabilità dei franchi tiratori, forbice.
//!
//! Distanza come base della scelta: voto-spaziale:41.

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::parlamento::Conteggio;
use crate::partiti::{Governo, Partito};

/// Fuori dal Presidente del Consiglio e al governo.
pub(crate) fn alleato(governo: &Governo, j: usize) -> bool {
    j != governo.presidente && governo.al_governo(j)
}

/// La linea sotto fiducia: il governo vota sì, tranne l'alleato in ultimatum (intesa sotto
/// `soglia_ultimatum`) distante almeno `distanza_alleato` da `nuova`; fuori dal governo no.
pub fn linea_fiducia(
    dati: &DatiGioco,
    partiti: &[Partito],
    governo: &Governo,
    intesa: &[f64],
    nuova: Option<f64>,
) -> Vec<bool> {
    let ultimatum = dati.manopola("soglia_ultimatum");
    let distanza = dati.manopola("distanza_alleato");
    (partiti.iter().zip(intesa).enumerate())
        .map(|(j, (p, &i))| {
            let lontana = nuova.is_some_and(|n| (p.posizione_dichiarata - n).abs() >= distanza);
            governo.al_governo(j) && !(alleato(governo, j) && i < ultimatum && lontana)
        })
        .collect()
}

/// La linea di ogni partito, nell'ordine dei partiti. `posizioni` = media in vigore e media
/// nuova della misura, `None` se non ha posizione. Il partito del Presidente del Consiglio vota
/// sì; gli altri se `s > 0`, con `s` = guadagno di distanza, più l'intesa per gli alleati, meno
/// `malus_opposizione` per chi è fuori dal governo.
pub fn linea(
    dati: &DatiGioco,
    partiti: &[Partito],
    governo: &Governo,
    intesa: &[f64],
    posizioni: Option<(f64, f64)>,
) -> Vec<bool> {
    let peso = dati.manopola("peso_intesa_linea");
    let malus = dati.manopola("malus_opposizione");
    (partiti.iter().zip(intesa).enumerate())
        .map(|(j, (p, &i))| {
            if j == governo.presidente {
                return true;
            }
            let x = p.posizione_dichiarata;
            let guadagno = posizioni.map_or(0.0, |(vigore, nuova)| {
                (x - vigore).abs() - (x - nuova).abs()
            });
            let s = if governo.al_governo(j) {
                guadagno + peso * (i - 50.0) / 50.0
            } else {
                guadagno - malus
            };
            s > 0.0
        })
        .collect()
}

/// Probabilità che un seggio di ogni partito voti contro la sua `linea`. `nuova` = posizione
/// media dei valori nuovi, `None` se la misura non ha posizione.
pub fn probabilita_franco(
    dati: &DatiGioco,
    partiti: &[Partito],
    governo: &Governo,
    intesa: &[f64],
    nuova: Option<f64>,
    linea: &[bool],
) -> Vec<f64> {
    let base = dati.manopola("ft_base");
    let per_distanza = dati.manopola("ft_distanza");
    let per_intesa = dati.manopola("ft_intesa");
    let tetto = dati.manopola("ft_max");
    (partiti.iter().zip(intesa).zip(linea).enumerate())
        .map(|(j, ((p, &i), &si))| {
            let d = nuova.map_or(0.0, |n| (p.posizione_dichiarata - n).abs() / 6.0);
            let q = if si {
                let intesa_bassa = if alleato(governo, j) {
                    per_intesa * (50.0 - i).max(0.0) / 50.0
                } else {
                    0.0
                };
                base + per_distanza * d + intesa_bassa
            } else {
                base * (1.0 - d)
            };
            q.min(tetto)
        })
        .collect()
}

/// Sì necessari in un ramo di `seggi` seggi: tutti votano, passa con `sì > no`.
pub fn maggioranza(seggi: u32) -> u32 {
    seggi / 2 + 1
}

/// Sì attesi ± 2σ, arrotondati e limitati ai seggi del ramo.
pub fn forbice(seggi: &[u32], linea: &[bool], q: &[f64]) -> [u32; 2] {
    let (mut media, mut varianza) = (0.0, 0.0);
    for ((&n, &si), &q) in seggi.iter().zip(linea).zip(q) {
        let n = f64::from(n);
        media += n * if si { 1.0 - q } else { q };
        varianza += n * q * (1.0 - q);
    }
    let totale = f64::from(seggi.iter().sum::<u32>());
    let ampiezza = 2.0 * f64::sqrt(varianza);
    let limita = |x: f64| x.round().clamp(0.0, totale) as u32;
    [limita(media - ampiezza), limita(media + ampiezza)]
}

/// Il voto di un ramo: un'uniforme per seggio, partiti in ordine; il seggio vota contro la
/// linea se l'uniforme sta sotto la probabilità del suo partito.
pub(crate) fn conta(caso: &mut Caso, seggi: &[u32], linea: &[bool], q: &[f64]) -> Conteggio {
    let mut c = Conteggio { si: 0, no: 0 };
    for ((&n, &si), &q) in seggi.iter().zip(linea).zip(q) {
        for _ in 0..n {
            let contro = caso.uniforme(Flusso::Parlamento) < q;
            if si != contro {
                c.si += 1;
            } else {
                c.no += 1;
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::partiti;

    fn centrodestra(dati: &DatiGioco, intesa: f64, nuova: f64) -> f64 {
        let (partiti, governo) = partiti::iniziali(dati);
        let j = (partiti.iter())
            .position(|p| p.id == "centrodestra")
            .unwrap_or_default();
        let mut valori = vec![60.0; partiti.len()];
        valori[j] = intesa;
        let linea = vec![true; partiti.len()];
        probabilita_franco(dati, &partiti, &governo, &valori, Some(nuova), &linea)[j]
    }

    #[test]
    fn franco_cresce_con_la_distanza_e_l_intesa_bassa() {
        let dati = dati_base();
        let vicina = centrodestra(&dati, 60.0, 5.0);
        let lontana = centrodestra(&dati, 60.0, 2.0);
        let intesa_bassa = centrodestra(&dati, 20.0, 5.0);
        assert!((vicina - 0.02).abs() < 1e-12, "{vicina}");
        assert!((lontana - (0.02 + 0.15 * 0.5)).abs() < 1e-12, "{lontana}");
        assert!(
            (intesa_bassa - (0.02 + 0.1 * 0.6)).abs() < 1e-12,
            "{intesa_bassa}"
        );
    }

    /// La prossima uniforme del flusso del parlamento dopo un voto su `seggi`.
    fn dopo_il_voto(seggi: &[u32], linea: &[bool], q: &[f64]) -> f64 {
        let mut caso = Caso::nuovo(7);
        conta(&mut caso, seggi, linea, q);
        caso.uniforme(Flusso::Parlamento)
    }

    #[test]
    fn conta_estrae_sempre_un_uniforme_per_seggio() {
        let seggi = [3, 0, 5];
        let attesa = dopo_il_voto(&seggi, &[true; 3], &[0.3; 3]);
        for linea in [[true; 3], [false; 3]] {
            for q in [[0.0; 3], [1.0; 3]] {
                assert_eq!(dopo_il_voto(&seggi, &linea, &q), attesa);
            }
        }
    }

    #[test]
    fn la_fiducia_estrae_quanto_un_voto_libero() {
        use crate::disposizioni::Disposizioni;
        use crate::parlamento::{FormaMisura, Misura, StatoAula};
        let dati = dati_base();
        let (partiti, governo) = partiti::iniziali(&dati);
        let disposizioni = Disposizioni::iniziali(&dati);
        let intesa = vec![30.0; partiti.len()];
        let aula = StatoAula {
            dati: &dati,
            disposizioni: &disposizioni,
            partiti: &partiti,
            governo: &governo,
            intesa: &intesa,
        };
        let eta = dati.disposizione("eta_pensionabile").unwrap_or_default();
        let misura = |fiducia| Misura {
            id: 0,
            forma: FormaMisura::Riforma,
            valori: vec![(eta, 0)],
            proponente: None,
            presentata: 0,
            voto: 0,
            deliberazione: 0,
            fiducia,
            emendamenti: Vec::new(),
            emendamenti_aperti: false,
        };
        let seggi = [9, 100, 20, 14, 48, 189, 20];
        let dopo = |fiducia| {
            let p = aula.previsione(&misura(fiducia));
            assert!(p.franchi.iter().all(|&q| q == 0.0) == fiducia);
            dopo_il_voto(&seggi, &p.linea, &p.franchi)
        };
        assert_eq!(dopo(true), dopo(false));
    }

    #[test]
    fn ft_max_limita_la_probabilita() {
        let mut dati = dati_base();
        dati.imposta_manopola("ft_max", 0.05);
        assert!((centrodestra(&dati, 0.0, 1.0) - 0.05).abs() < 1e-12);
        assert!((centrodestra(&dati, 60.0, 5.0) - 0.02).abs() < 1e-12);
    }
}
