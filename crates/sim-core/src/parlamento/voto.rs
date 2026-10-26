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

/// Guadagno di distanza di ogni partito: quanto la media nuova `posizioni.1` gli è più vicina
/// della media in vigore `posizioni.0`; zero se la misura non ha posizione.
pub fn guadagni(partiti: &[Partito], posizioni: Option<(f64, f64)>) -> Vec<f64> {
    (partiti.iter())
        .map(|p| {
            let x = p.posizione_dichiarata;
            posizioni.map_or(0.0, |(vigore, nuova)| {
                (x - vigore).abs() - (x - nuova).abs()
            })
        })
        .collect()
}

/// La linea di ogni partito, nell'ordine dei partiti. Il partito del Presidente del Consiglio
/// vota sì; gli altri se `s > 0`, con `s` = `guadagno`, più l'intesa per gli alleati, meno
/// `malus_opposizione` per chi è fuori dal governo.
pub fn linea(dati: &DatiGioco, governo: &Governo, intesa: &[f64], guadagno: &[f64]) -> Vec<bool> {
    let peso = dati.manopola("peso_intesa_linea");
    let malus = dati.manopola("malus_opposizione");
    (intesa.iter().zip(guadagno).enumerate())
        .map(|(j, (&i, &g))| {
            if j == governo.presidente {
                return true;
            }
            let s = if governo.al_governo(j) {
                g + peso * (i - 50.0) / 50.0
            } else {
                g - malus
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
    use crate::parlamento::StatoEmendamento;
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
            sondaggio: &crate::informazione::Sondaggio {
                istituti: Vec::new(),
                condizioni: Vec::new(),
                fasce: Vec::new(),
                regioni: Vec::new(),
            },
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
    fn posizioni_mediano_solo_le_coppie_con_entrambe_le_posizioni() {
        use crate::disposizioni::Disposizioni;
        use crate::parlamento::posizioni;
        let dati = dati_base();
        let disposizioni = Disposizioni::iniziali(&dati);
        let g = |id: &str| (dati.disposizione(id).unwrap_or_default(), 0);
        // Età 62 (2.0) contro 67 (4.5); regime retributivo (2.0) contro contributivo (5.0);
        // la soglia di lista, elettorale, non ha posizioni.
        let (eta, regime, soglia) = (
            g("eta_pensionabile"),
            g("regime_pensioni"),
            g("soglia_lista"),
        );
        let p = |v: &[(usize, usize)]| posizioni(&dati, &disposizioni, v);
        assert_eq!(p(&[eta]), Some((4.5, 2.0)));
        assert_eq!(p(&[eta, regime, soglia]), Some((4.75, 2.0)));
        assert_eq!(p(&[soglia]), None);
    }

    #[test]
    fn la_fiducia_sull_elettorale_costa_il_doppio() {
        use crate::parlamento::{FormaMisura, costo_fiducia};
        let dati = dati_base();
        let attesa = dati.manopola("costo_fiducia") + dati.manopola("costo_fiducia_distanza") * 0.5;
        let costo = |forma| costo_fiducia(&dati, forma, 5.0, Some(2.0));
        assert!((costo(FormaMisura::Riforma) - attesa).abs() < 1e-12);
        assert!((costo(FormaMisura::Elettorale) - 2.0 * attesa).abs() < 1e-12);
    }

    /// I franchi tiratori del centrodestra su una riforma a 62 anni, con l'emendamento nello
    /// `stato` dato (`None` = nessun emendamento).
    fn franco_con_emendamento(dati: &DatiGioco, stato: Option<StatoEmendamento>) -> f64 {
        use crate::disposizioni::Disposizioni;
        use crate::parlamento::{FormaMisura, Misura, RichiestaEmendamento, StatoAula};
        let (partiti, governo) = partiti::iniziali(dati);
        let disposizioni = Disposizioni::iniziali(dati);
        let intesa = vec![30.0; partiti.len()];
        let aula = StatoAula {
            dati,
            disposizioni: &disposizioni,
            sondaggio: &crate::informazione::Sondaggio {
                istituti: Vec::new(),
                condizioni: Vec::new(),
                fasce: Vec::new(),
                regioni: Vec::new(),
            },
            partiti: &partiti,
            governo: &governo,
            intesa: &intesa,
        };
        let j = (partiti.iter())
            .position(|p| p.id == "centrodestra")
            .unwrap_or_default();
        let eta = dati.disposizione("eta_pensionabile").unwrap_or_default();
        let misura = Misura {
            id: 0,
            forma: FormaMisura::Riforma,
            valori: vec![(eta, 0)],
            proponente: None,
            presentata: 0,
            voto: 0,
            deliberazione: 0,
            fiducia: false,
            emendamenti: (stato.into_iter())
                .map(|stato| RichiestaEmendamento {
                    partito: j,
                    proposta: Vec::new(),
                    stato,
                    motivo: String::new(),
                })
                .collect(),
            emendamenti_aperti: true,
        };
        aula.previsione(&misura).franchi[j]
    }

    #[test]
    fn emendamento_accolto_da_ft_base_respinto_li_moltiplica_entro_ft_max() {
        use StatoEmendamento::{Accolto, Respinto};
        let mut dati = dati_base();
        dati.imposta_manopola("ft_emendamento_respinto", 3.0);
        let q = franco_con_emendamento(&dati, None);
        assert!(q > 0.0 && q * 4.0 < dati.manopola("ft_max"), "{q}");
        assert!(
            (franco_con_emendamento(&dati, Some(Accolto)) - dati.manopola("ft_base")).abs() < 1e-12
        );
        assert!((franco_con_emendamento(&dati, Some(Respinto)) - 4.0 * q).abs() < 1e-12);
        dati.imposta_manopola("ft_max", 2.0 * q);
        assert!((franco_con_emendamento(&dati, Some(Respinto)) - 2.0 * q).abs() < 1e-12);
    }

    #[test]
    fn ft_max_limita_la_probabilita() {
        let mut dati = dati_base();
        dati.imposta_manopola("ft_max", 0.05);
        assert!((centrodestra(&dati, 0.0, 1.0) - 0.05).abs() < 1e-12);
        assert!((centrodestra(&dati, 60.0, 5.0) - 0.02).abs() < 1e-12);
    }
}
