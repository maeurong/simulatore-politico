//! Informazione comprata con la cassa del partito del giocatore, al massimo una volta a turno
//! per tipo.

use super::{Sondaggio, Statistica, Stima, controlla_sondaggio};
use crate::caso::Estrattore;
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::elettorato::Condizione;
use crate::errore::Errore;
use crate::parlamento::ValoreProposto;
use crate::partita::Trimestre;
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
    CentroStudi,
}

impl Acquisto {
    pub fn tipo(&self) -> TipoAcquisto {
        match self {
            Acquisto::SondaggioPrivato => TipoAcquisto::SondaggioPrivato,
            Acquisto::TestMisura { .. } => TipoAcquisto::TestMisura,
            Acquisto::CentroStudi => TipoAcquisto::CentroStudi,
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
    CentroStudi,
}

impl TipoAcquisto {
    pub const TUTTI: [TipoAcquisto; 3] = [
        TipoAcquisto::SondaggioPrivato,
        TipoAcquisto::TestMisura,
        TipoAcquisto::CentroStudi,
    ];

    pub fn nome(self) -> &'static str {
        match self {
            TipoAcquisto::SondaggioPrivato => "sondaggio privato",
            TipoAcquisto::TestMisura => "test di una misura",
            TipoAcquisto::CentroStudi => "centro studi",
        }
    }

    /// Euro, dalla manopola del tipo.
    pub fn prezzo(self, dati: &DatiGioco) -> i64 {
        let nome = match self {
            TipoAcquisto::SondaggioPrivato => "prezzo_sondaggio_privato",
            TipoAcquisto::TestMisura => "prezzo_test_misura",
            TipoAcquisto::CentroStudi => "prezzo_centro_studi",
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

/// Il test di una misura: `misura` è il nome della bozza, poi una reazione per condizione.
// Le reazioni stanno nell'ordine di `Condizione::TUTTE`.
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

/// Simmetrica: `centro ± sigma` è l'intervallo di 1σ, in punti percentuali (il debito in punti
/// di PIL).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Forchetta {
    pub centro: f64,
    pub sigma: f64,
}

/// Le grandezze previste del trimestre `data`, in percentuali.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePrevisione {
    pub data: Trimestre,
    /// Crescita del PIL reale sullo stesso trimestre dell'anno prima.
    // Le RMSE sono sulla crescita annua, e `storico` tiene i PIL nominali di otto trimestri,
    // `inflazione` i prezzi. I quattro trimestri prima del turno zero sono il conto del turno zero
    // ai suoi prezzi, quindi si deflaziona solo con le inflazioni dei trimestri chiusi dopo.
    pub pil: Forchetta,
    /// Tasso nazionale.
    pub disoccupazione: Forchetta,
    /// Prezzi sullo stesso trimestre dell'anno prima.
    // Dalle quattro inflazioni trimestrali.
    pub inflazione: Forchetta,
    /// In % del PIL degli ultimi quattro trimestri.
    pub debito: Forchetta,
}

impl VocePrevisione {
    /// L'economia `e` dopo `h` chiusure (1-4) della copia, `data` il trimestre chiuso per ultimo.
    /// La sigma va da `rmse_<grandezza>_1` a `rmse_<grandezza>_4`, lineare in `h`.
    pub(crate) fn da(dati: &DatiGioco, e: &Economia, data: Trimestre, h: u32) -> VocePrevisione {
        let t = f64::from(h.saturating_sub(1)) / 3.0;
        let forchetta = |nome: &str, centro: f64| Forchetta {
            centro,
            sigma: dati.manopola(&format!("rmse_{nome}_1")) * (1.0 - t)
                + dati.manopola(&format!("rmse_{nome}_4")) * t,
        };
        let prezzi = |n: usize| -> f64 {
            (e.inflazione.iter().take(n))
                .map(|i| 1.0 + i / 400.0)
                .product()
        };
        // Le chiusure dal turno zero a `data` compresa: la partita parte sempre da `TURNO_ZERO`.
        let indice = |t: Trimestre| 4 * u32::from(t.anno) + u32::from(t.trimestre);
        let passi = (indice(data) + 1).saturating_sub(indice(Trimestre::TURNO_ZERO));
        let deflatore = prezzi(passi.min(4) as usize);
        // ponytail: con meno di cinque trimestri in `storico` (solo da un salvataggio scritto a
        // mano) la crescita vale 0, come al turno zero della statistica.
        let pil = match (e.storico.last(), e.storico.iter().rev().nth(4)) {
            (Some(ora), Some(prima)) => {
                100.0 * (ora.pil as f64 / prima.pil as f64 / deflatore - 1.0)
            }
            _ => 0.0,
        };
        VocePrevisione {
            data,
            pil: forchetta("pil", pil),
            disoccupazione: forchetta("disoccupazione", e.disoccupazione_nazionale(dati)),
            inflazione: forchetta("inflazione", e.inflazione_annua()),
            debito: forchetta("debito", e.debito_e_disavanzo_pil().0),
        }
    }
}

/// La stima del trimestre chiuso e la previsione dei quattro trimestri dal corrente. Con
/// l'inflazione della `previsione` si confronta l'`inflazione_annua` della `stima`: tutte e due
/// sullo stesso trimestre dell'anno prima. Il PIL e l'`inflazione` della `stima` no: sono
/// annualizzati sul trimestre prima, come nella prima stima.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaCentroStudi {
    pub stima: Stima,
    pub previsione: Vec<VocePrevisione>,
}

impl VistaCentroStudi {
    /// La stima è `vera + quota_errore_centro_studi · errore_stima_<campo> · z`, campo per campo,
    /// con le `z` di `rng` nell'ordine dei campi: un errore suo, così dalla prima
    /// stima e dalla sua non si ricava il vero. L'inflazione annua non ha una `z` sua
    /// (`Stima::con_errore`). `e` è l'economia del trimestre di `s`.
    pub(crate) fn stima(
        dati: &DatiGioco,
        s: &Statistica,
        e: &Economia,
        mut rng: Estrattore,
    ) -> Stima {
        // Al turno zero la prima stima è già il vero (`Statistica::iniziale`): un errore in più
        // renderebbe il centro studi peggiore del dato pubblico.
        if s.trimestre == Trimestre::TURNO_ZERO.precedente() {
            return s.vera.clone();
        }
        let q = dati.manopola("quota_errore_centro_studi");
        (s.vera).con_errore(e, |nome, vera| {
            vera + q * dati.manopola(nome) * rng.normale()
        })
    }
}

/// Gli acquisti del turno col loro risultato, al massimo uno per tipo.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Acquistati {
    pub sondaggio_privato: Option<Sondaggio>,
    pub test_misura: Option<VistaTest>,
    pub centro_studi: Option<VistaCentroStudi>,
}

impl Acquistati {
    /// Forma e finitezza dei risultati, per uno stato letto da un salvataggio: il privato ha
    /// un solo istituto, il test una reazione per condizione in ordine col segno fra −1 e 1, il
    /// centro studi quattro trimestri consecutivi da `data`, il corrente.
    ///
    /// # Errors
    /// `SalvataggioIlleggibile` col campo nel messaggio.
    pub(crate) fn controlla(
        &self,
        dati: &DatiGioco,
        n_partiti: usize,
        data: Trimestre,
    ) -> Result<(), Errore> {
        if let Some(s) = &self.sondaggio_privato {
            controlla_sondaggio(s, dati, n_partiti + 1, 1)
                .map_err(|m| Errore::SalvataggioIlleggibile(format!("acquisti.privato.{m}")))?;
        }
        if let Some(t) = &self.test_misura {
            let ko = |m: &str| Err(Errore::SalvataggioIlleggibile(format!("acquisti.test.{m}")));
            let condizioni = t.reazioni.iter().map(|r| r.condizione);
            if !condizioni.eq(Condizione::TUTTE) {
                return ko("reazioni: servono le 6 condizioni, nel loro ordine");
            }
            if t.reazioni.iter().any(|r| !(-1..=1).contains(&r.segno)) {
                return ko("segno: fuori da −1, 0, 1");
            }
        }
        if let Some(c) = &self.centro_studi {
            let ko = |m: &str| {
                Err(Errore::SalvataggioIlleggibile(format!(
                    "acquisti.centro_studi.{m}"
                )))
            };
            if c.previsione.len() != 4 {
                return ko("previsione: servono 4 voci");
            }
            let date = std::iter::successors(Some(data), |t| Some(t.successivo()));
            if !c.previsione.iter().map(|v| v.data).eq(date.take(4)) {
                return ko("date: servono i 4 trimestri consecutivi dal corrente");
            }
            let forchette = (c.previsione.iter())
                .flat_map(|v| [v.pil, v.disoccupazione, v.inflazione, v.debito]);
            for f in forchette {
                if !(f.centro.is_finite() && f.sigma.is_finite()) {
                    return ko("previsione: valore non finito");
                }
                if f.sigma < 0.0 {
                    return ko("sigma: negativa");
                }
            }
            if !c.stima.e_finita() {
                return ko("stima: valore non finito");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caso::Caso;
    use crate::dati::dati_base;

    /// Le `z` del centro studi sono le prime quattro dell'estrattore, nell'ordine dei campi:
    /// l'inflazione annua non ne prende una sua.
    #[test]
    fn il_centro_studi_estrae_quattro_normali() {
        let dati = dati_base();
        let e = Economia::iniziale(&dati);
        let mut caso = Caso::nuovo(1);
        let mut s = Statistica::iniziale(&dati, &e);
        let reale = e.pil_trimestrale / e.livello_prezzi;
        s.aggiorna(&dati, &mut caso, &e, reale, Trimestre::TURNO_ZERO);
        let stima = VistaCentroStudi::stima(&dati, &s, &e, caso.estemporaneo(7));
        let mut rng = caso.estemporaneo(7);
        let z: [f64; 4] = std::array::from_fn(|_| rng.normale());
        let q = dati.manopola("quota_errore_centro_studi");
        let attesa = |nome: &str, vera: f64, z: f64| vera + q * dati.manopola(nome) * z;
        let (v, inflazione, disavanzo) =
            (&s.vera, "errore_stima_inflazione", "errore_stima_disavanzo");
        assert_eq!(stima.inflazione, attesa(inflazione, v.inflazione, z[2]));
        assert_eq!(stima.disavanzo, attesa(disavanzo, v.disavanzo, z[3]));
    }

    /// `n` test con le stesse `reazioni`, chiavi `0..n`: la condizione `k` riceve la stessa `z`
    /// a parità di chiave, qualunque siano vere e quote.
    fn stime(dati: &DatiGioco, reazioni: [(f64, f64); 6], n: u64) -> Vec<VistaTest> {
        let caso = Caso::nuovo(1);
        (0..n)
            .map(|k| VistaTest::stima(dati, caso.estemporaneo(k), String::new(), reazioni))
            .collect()
    }

    /// Con `errore_test` 0,05, perché la vera 0,05 non stia a molte sigma dallo zero.
    #[test]
    fn i_segni_giusti_crescono_con_la_quota() {
        let quote = [0.04, 0.09, 0.16, 0.25, 0.36, 0.49];
        let reazioni = quote.map(|q| (0.05, q));
        let mut giusti = [0; 6];
        let mut dati = dati_base();
        dati.imposta_manopola("errore_test", 0.05);
        for t in stime(&dati, reazioni, 20_000) {
            for (g, r) in giusti.iter_mut().zip(&t.reazioni) {
                *g += u32::from(r.segno == 1);
            }
        }
        assert!(giusti.windows(2).all(|w| w[0] < w[1]), "{giusti:?}");
    }

    /// Con le stesse `z`, `|σ₁ z| < s` vale esattamente quando `|σ₂ z| < s · σ₂/σ₁`: con σ come
    /// `1/√quota` i `Poco` coincidono se la soglia scala di `√(q₁/q₂)`, con σ costante no.
    /// `errore_test` 0,05 tiene la soglia 0,1 a meno di una sigma.
    #[test]
    fn la_dispersione_scala_come_uno_su_radice_della_quota() {
        let poco = |quota: f64, soglia: f64| {
            let mut dati = dati_base();
            dati.imposta_manopola("errore_test", 0.05);
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

    /// Senza errore la stima è la vera: fascia dalle soglie, comprese, e segno anche negativo.
    #[test]
    fn senza_errore_fascia_e_segno_sono_quelli_della_vera() {
        let mut dati = dati_base();
        dati.imposta_manopola("errore_test", 0.0);
        let (poco, medio) = (
            dati.manopola("soglia_reazione_poco"),
            dati.manopola("soglia_reazione_medio"),
        );
        let vere = [poco / 2.0, poco, medio, -poco / 2.0, -medio, -poco];
        let t = &stime(&dati, vere.map(|x| (x, 0.5)), 1)[0];
        let viste: Vec<_> = t.reazioni.iter().map(|r| (r.segno, r.fascia)).collect();
        assert_eq!(
            viste,
            [
                (1, Fascia3::Poco),
                (1, Fascia3::Medio),
                (1, Fascia3::Molto),
                (-1, Fascia3::Poco),
                (-1, Fascia3::Molto),
                (-1, Fascia3::Medio),
            ]
        );
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
