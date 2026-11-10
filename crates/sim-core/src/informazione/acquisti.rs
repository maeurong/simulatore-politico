//! Informazione comprata con la cassa del partito del giocatore, al massimo una volta a turno
//! per tipo.

use super::{Sondaggio, controlla_sondaggio};
use crate::caso::Estrattore;
use crate::dati::DatiGioco;
use crate::elettorato::Condizione;
use crate::errore::Errore;
use crate::parlamento::ValoreProposto;
use crate::vista::Fascia3;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Acquisto {
    SondaggioPrivato,
    /// Il test della bozza `valori`, la stessa dell'anteprima.
    TestMisura {
        valori: Vec<ValoreProposto>,
    },
}

impl Acquisto {
    pub fn tipo(&self) -> TipoAcquisto {
        match self {
            Acquisto::SondaggioPrivato => TipoAcquisto::SondaggioPrivato,
            Acquisto::TestMisura { .. } => TipoAcquisto::TestMisura,
        }
    }
}

/// Il tipo di un acquisto, senza i suoi campi: la chiave del prezzo e del limite di uno a turno.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum TipoAcquisto {
    SondaggioPrivato,
    TestMisura,
}

impl TipoAcquisto {
    pub const TUTTI: [TipoAcquisto; 2] = [TipoAcquisto::SondaggioPrivato, TipoAcquisto::TestMisura];

    pub fn nome(self) -> &'static str {
        match self {
            TipoAcquisto::SondaggioPrivato => "sondaggio privato",
            TipoAcquisto::TestMisura => "test di una misura",
        }
    }

    /// Euro, dalla manopola del tipo.
    pub fn prezzo(self, dati: &DatiGioco) -> i64 {
        let nome = match self {
            TipoAcquisto::SondaggioPrivato => "prezzo_sondaggio_privato",
            TipoAcquisto::TestMisura => "prezzo_test_misura",
        };
        dati.manopola(nome).round() as i64
    }
}

/// La reazione stimata di una condizione verso il governo: segno e fascia, mai la cifra.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ReazioneStimata {
    pub condizione: Condizione,
    /// −1, 0 o 1.
    pub segno: i8,
    pub fascia: Fascia3,
}

/// Il test di una misura: `misura` è il nome della bozza, una reazione per condizione
/// nell'ordine di `Condizione::TUTTE`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaTest {
    pub misura: String,
    pub reazioni: Vec<ReazioneStimata>,
}

impl VistaTest {
    /// Da `(vera, quota)` per condizione: la stima è `vera + σ·z`, con
    /// `σ = errore_test / √quota` (informazione-a-pagamento.md:54-59) e una `z` di `rng` per
    /// condizione, in ordine. Una condizione senza elettori (quota non positiva o NaN) dà segno 0
    /// e `Poco`.
    pub(crate) fn stima(
        dati: &DatiGioco,
        mut rng: Estrattore,
        misura: String,
        reazioni: [(f64, f64); 6],
    ) -> VistaTest {
        let errore = dati.manopola("errore_test");
        let (poco, medio) = (
            dati.manopola("soglia_reazione_poco"),
            dati.manopola("soglia_reazione_medio"),
        );
        let reazioni = (Condizione::TUTTE.into_iter().zip(reazioni))
            .map(|(condizione, (vera, quota))| {
                let z = rng.normale();
                if quota.is_nan() || quota <= 0.0 {
                    return ReazioneStimata {
                        condizione,
                        segno: 0,
                        fascia: Fascia3::Poco,
                    };
                }
                let stima = vera + errore / quota.sqrt() * z;
                let fascia = match stima.abs() {
                    x if x < poco => Fascia3::Poco,
                    x if x < medio => Fascia3::Medio,
                    _ => Fascia3::Molto,
                };
                ReazioneStimata {
                    condizione,
                    segno: if stima > 0.0 {
                        1
                    } else if stima < 0.0 {
                        -1
                    } else {
                        0
                    },
                    fascia,
                }
            })
            .collect();
        VistaTest { misura, reazioni }
    }
}

/// Un acquisto del turno col suo risultato.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Acquistato {
    SondaggioPrivato(Sondaggio),
    TestMisura(VistaTest),
}

impl Acquistato {
    pub fn tipo(&self) -> TipoAcquisto {
        match self {
            Acquistato::SondaggioPrivato(_) => TipoAcquisto::SondaggioPrivato,
            Acquistato::TestMisura(_) => TipoAcquisto::TestMisura,
        }
    }

    /// Forma e finitezza del risultato, per uno stato letto da un salvataggio: il privato ha
    /// un solo istituto, il test una reazione per condizione in ordine col segno fra −1 e 1.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` col campo nel messaggio.
    pub(crate) fn controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore> {
        match self {
            Acquistato::SondaggioPrivato(s) => controlla_sondaggio(s, dati, n_partiti + 1, 1)
                .map_err(|m| Errore::SalvataggioIlleggibile(format!("acquisti.privato.{m}"))),
            Acquistato::TestMisura(t) => {
                let ko =
                    |m: &str| Err(Errore::SalvataggioIlleggibile(format!("acquisti.test.{m}")));
                let condizioni = t.reazioni.iter().map(|r| r.condizione);
                if !condizioni.eq(Condizione::TUTTE) {
                    return ko("reazioni: servono le 6 condizioni, nel loro ordine");
                }
                if t.reazioni.iter().any(|r| !(-1..=1).contains(&r.segno)) {
                    return ko("segno: fuori da −1, 0, 1");
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caso::Caso;
    use crate::dati::dati_base;

    /// `n` test con le stesse `reazioni`, chiavi `0..n`: la condizione `k` riceve la stessa `z`
    /// a parità di chiave, qualunque siano vere e quote.
    fn stime(dati: &DatiGioco, reazioni: [(f64, f64); 6], n: u64) -> Vec<VistaTest> {
        let caso = Caso::nuovo(1);
        (0..n)
            .map(|k| VistaTest::stima(dati, caso.estemporaneo(k), String::new(), reazioni))
            .collect()
    }

    #[test]
    fn i_segni_giusti_crescono_con_la_quota() {
        let quote = [0.04, 0.09, 0.16, 0.25, 0.36, 0.49];
        let reazioni = quote.map(|q| (0.05, q));
        let mut giusti = [0; 6];
        for t in stime(&dati_base(), reazioni, 20_000) {
            for (g, r) in giusti.iter_mut().zip(&t.reazioni) {
                *g += u32::from(r.segno == 1);
            }
        }
        assert!(giusti.windows(2).all(|w| w[0] < w[1]), "{giusti:?}");
    }

    /// Con le stesse `z`, `|σ₁ z| < s` vale esattamente quando `|σ₂ z| < s · σ₂/σ₁`: con σ come
    /// `1/√quota` i `Poco` coincidono se la soglia scala di `√(q₁/q₂)`, con σ costante no.
    #[test]
    fn la_dispersione_scala_come_uno_su_radice_della_quota() {
        let poco = |quota: f64, soglia: f64| {
            let mut dati = dati_base();
            dati.imposta_manopola("soglia_reazione_poco", soglia);
            (stime(&dati, [(0.0, quota); 6], 5_000).iter())
                .flat_map(|t| &t.reazioni)
                .filter(|r| r.fascia == Fascia3::Poco)
                .count()
        };
        let (q1, q2, s) = (0.04, 0.36, 0.1);
        let piccola = poco(q1, s);
        assert!((5_000..25_000).contains(&piccola), "{piccola}");
        assert!(piccola.abs_diff(poco(q2, s * (q1 / q2).sqrt())) <= 2);
        assert!(poco(q2, s) > piccola + 5_000);
    }

    #[test]
    fn una_condizione_senza_elettori_da_segno_zero_e_poco() {
        let reazioni = [
            (0.3, 0.0),
            (0.3, f64::NAN),
            (0.3, 0.5),
            (0.3, -1.0),
            (0.3, 0.5),
            (0.3, 0.5),
        ];
        let t = &stime(&dati_base(), reazioni, 1)[0];
        for r in [&t.reazioni[0], &t.reazioni[1], &t.reazioni[3]] {
            assert_eq!((r.segno, r.fascia), (0, Fascia3::Poco));
        }
        assert_eq!(t.reazioni[2].segno, 1);
    }
}
