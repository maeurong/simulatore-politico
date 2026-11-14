//! Eventi puntuali (emergenze, scandali, shock): una uniforme per candidato, sempre, e al massimo
//! un puntuale a turno, il più grave.

use super::{Bersaglio, Causa, Tono};
use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::euro;
use crate::grandezze::Grandezze;
use crate::ministri::Ministri;
use crate::partita::LIMITE_EURO;
use crate::partiti::Partito;
use crate::vista::Fascia3;

/// Gli eventi che nascono dallo scandalo: sul ministro, sul leader.
pub(crate) const SCANDALI: [&str; 2] = ["scandalo_ministro", "scandalo_leader"];

/// Uno shock: l'evento, il nome della serie nella causa, il segno in cui l'innovazione è
/// sfavorevole e la manopola della sua sigma.
pub(crate) struct Shock {
    pub evento: &'static str,
    serie: &'static str,
    segno: f64,
    sigma: &'static str,
}

/// Gli shock, nell'ordine delle innovazioni di `Economia::passo`: domanda, produttività, energia,
/// domanda estera, BCE.
pub(crate) const SHOCK: [Shock; 5] = [
    Shock {
        evento: "shock_domanda",
        serie: "domanda",
        segno: -1.0,
        sigma: "sigma_domanda",
    },
    Shock {
        evento: "shock_produttivita",
        serie: "produttività",
        segno: -1.0,
        sigma: "sigma_produttivita",
    },
    Shock {
        evento: "shock_energia",
        serie: "prezzi dell'energia",
        segno: 1.0,
        sigma: "sigma_energia",
    },
    Shock {
        evento: "shock_domanda_estera",
        serie: "domanda estera",
        segno: -1.0,
        sigma: "sigma_domanda_estera",
    },
    Shock {
        evento: "shock_bce",
        serie: "tasso BCE",
        segno: 1.0,
        sigma: "sigma_bce",
    },
];

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
    /// Un'emergenza lieve: non apre un evento, la paga il fondo alla chiusura dopo (`Eventi.lievi`).
    pub lieve: bool,
}

/// Lo scandalo del turno: `p = scandalo_base × corruzione / 100`. Il bersaglio si estrae
/// sempre, anche a `p` zero o senza bersagli, fra i ministri in carica (in ordine di dicastero) e
/// i leader dei partiti IA non sciolti (in ordine d'indice): il giocatore non è mai fra loro.
pub(crate) fn candidato_scandalo(
    dati: &DatiGioco,
    grandezze: &Grandezze,
    ministri: &Ministri,
    partiti: &[Partito],
    giocatore: usize,
    caso: &mut Caso,
) -> Option<Candidato> {
    let corruzione = grandezze.nazionale(dati, dati.grandezza("corruzione")?);
    let (del_ministro, del_leader) = (dati.evento(SCANDALI[0])?, dati.evento(SCANDALI[1])?);
    let bersagli: Vec<Bersaglio> = (ministri.dicasteri.iter().enumerate())
        .filter(|(_, st)| st.ministro.is_some())
        .map(|(dicastero, _)| Bersaglio::Ministro { dicastero })
        .chain(
            (partiti.iter().enumerate())
                .filter(|&(j, p)| j != giocatore && !p.sciolto)
                .map(|(partito, _)| Bersaglio::Leader { partito }),
        )
        .collect();
    let u = caso.uniforme(Flusso::Eventi);
    let bersaglio = bersagli.get((u * bersagli.len() as f64) as usize).copied();
    let p = if bersaglio.is_some() {
        (dati.manopola("scandalo_base") * corruzione / 100.0).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let evento = match bersaglio {
        Some(Bersaglio::Leader { .. }) => del_leader,
        _ => del_ministro,
    };
    Some(Candidato {
        evento,
        p,
        gravita: (dati.eventi().get(evento)).map_or(0, |e| e.gravita),
        regione: None,
        bersaglio,
        causa: Causa::Rischio {
            livello: livello(p),
        },
        costo: 0,
        tono: Tono::Ordinario,
        lieve: false,
    })
}

/// Un candidato per serie, sempre cinque: `p = 1` se l'innovazione sfavorevole supera
/// `soglia_shock` sigma, altrimenti 0 (lo shock favorevole non fa nascere nulla). Il costo degli
/// aiuti è `aiuti_shock` del PIL annuo `pil_annuo`, in euro. Senza gli eventi nel catalogo, nessuno.
pub(crate) fn candidati_shock(
    dati: &DatiGioco,
    innovazioni: &[f64; 5],
    pil_annuo: f64,
) -> Vec<Candidato> {
    // Un salvataggio può allargare min e max della manopola: il costo non scende sotto zero.
    let costo = euro(dati.manopola("aiuti_shock") * pil_annuo).clamp(0, LIMITE_EURO);
    let soglia = dati.manopola("soglia_shock");
    let mut candidati = Vec::new();
    for (s, z) in SHOCK.iter().zip(innovazioni) {
        let Some(evento) = dati.evento(s.evento) else {
            return Vec::new();
        };
        let sfavore = z * s.segno;
        candidati.push(Candidato {
            evento,
            // Con la sigma a zero la serie non si muove: niente evento (l'uniforme si consuma lo stesso).
            p: if sfavore > soglia && dati.manopola(s.sigma) > 0.0 {
                1.0
            } else {
                0.0
            },
            gravita: (dati.eventi().get(evento)).map_or(0, |e| e.gravita),
            regione: None,
            bersaglio: None,
            causa: Causa::Shock {
                serie: s.serie.to_string(),
                sigma: sfavore.max(0.0),
                soglia,
            },
            costo,
            tono: Tono::Ordinario,
            lieve: false,
        });
    }
    candidati
}

/// Il livello di rischio per la causa: la vista non mostra mai la probabilità in cifre.
pub(crate) fn livello(p: f64) -> Fascia3 {
    if p < 0.05 {
        Fascia3::Poco
    } else if p < 0.2 {
        Fascia3::Medio
    } else {
        Fascia3::Molto
    }
}

/// Gli indici dei candidati usciti: il vincitore del tetto e gli altri, esclusi, nell'ordine.
pub(crate) struct Estratti {
    pub vincitore: Option<usize>,
    pub esclusi: Vec<usize>,
}

/// Una uniforme da `Flusso::Eventi` per candidato, nell'ordine, a prescindere dagli esiti: esce chi
/// ha `u < p`. Vince fra gli usciti la gravità più alta, a parità il primo. Una `p` non
/// finita non è difesa qui: con NaN `u < p` è falso e il candidato non esce mai, `p >= 1` esce
/// sempre, `p <= 0` mai; i candidati nascono dai dati validati, con `p` in 0-1.
pub(crate) fn estrai(caso: &mut Caso, candidati: &[Candidato]) -> Estratti {
    let mut usciti = Vec::new();
    let mut vincitore: Option<usize> = None;
    for (i, c) in candidati.iter().enumerate() {
        if caso.uniforme(Flusso::Eventi) < c.p {
            if vincitore.is_none_or(|v| c.gravita > candidati[v].gravita) {
                vincitore = Some(i);
            }
            usciti.push(i);
        }
    }
    usciti.retain(|&i| Some(i) != vincitore);
    Estratti {
        vincitore,
        esclusi: usciti,
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce è il fallimento voluto"
)]
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
            lieve: false,
        }
    }

    #[test]
    fn vince_il_piu_grave() {
        let mut caso = Caso::nuovo(1);
        let v = estrai(&mut caso, &[cand(1.0, 2), cand(1.0, 5), cand(1.0, 3)]);
        assert_eq!(v.vincitore, Some(1));
    }

    #[test]
    fn a_parita_vince_il_primo() {
        let mut caso = Caso::nuovo(1);
        let v = estrai(&mut caso, &[cand(1.0, 4), cand(1.0, 4)]);
        assert_eq!(v.vincitore, Some(0));
    }

    #[test]
    fn estrai_rende_gli_esclusi() {
        let mut caso = Caso::nuovo(1);
        let v = estrai(&mut caso, &[cand(1.0, 8), cand(1.0, 12)]);
        assert_eq!((v.vincitore, v.esclusi), (Some(1), vec![0]));
        // Chi non esce non è escluso.
        let v = estrai(&mut caso, &[cand(0.0, 8), cand(1.0, 12), cand(1.0, 3)]);
        assert_eq!((v.vincitore, v.esclusi), (Some(1), vec![2]));
    }

    #[test]
    fn estrae_sempre_tutti() {
        let mut a = Caso::nuovo(7);
        let mut b = Caso::nuovo(7);
        let v = estrai(&mut a, &[cand(1.0, 1), cand(0.0, 9), cand(1.0, 1)]);
        assert_eq!(v.vincitore, Some(0));
        for _ in 0..3 {
            b.uniforme(Flusso::Eventi);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn lo_scandalo_non_ha_mai_il_giocatore_per_bersaglio() {
        let dati = crate::dati::dati_base();
        let p = crate::partita::Partita::nuova(crate::dati::dati_base(), 1);
        let grandezze = Grandezze::iniziali(&dati);
        let (mut ministri, mut leader) = (0, 0);
        for seme in 1..=1000 {
            let mut caso = Caso::nuovo(seme);
            let c = candidato_scandalo(
                &dati,
                &grandezze,
                p.ministri(),
                p.partiti(),
                p.giocatore(),
                &mut caso,
            )
            .unwrap();
            match c.bersaglio {
                Some(Bersaglio::Leader { partito }) => {
                    assert_ne!(partito, p.giocatore(), "seme {seme}");
                    assert!(!p.partiti()[partito].sciolto);
                    leader += 1;
                }
                Some(Bersaglio::Ministro { .. }) => ministri += 1,
                None => unreachable!("seme {seme}: nessun bersaglio"),
            }
        }
        assert!(ministri > 0 && leader > 0, "{ministri} e {leader}");
    }

    fn con_z(z: [f64; 5]) -> Vec<Candidato> {
        candidati_shock(&crate::dati::dati_base(), &z, 1e12)
    }

    #[test]
    fn shock_oltre_soglia_apre_un_evento() {
        let c = con_z([-3.0, 0.0, 0.0, 0.0, 0.0]);
        let mut caso = Caso::nuovo(1);
        assert_eq!(estrai(&mut caso, &c).vincitore, Some(0));
        assert_eq!(
            c[0].causa,
            Causa::Shock {
                serie: "domanda".into(),
                sigma: 3.0,
                soglia: 2.5
            }
        );
        assert_eq!(c[0].costo, 3_000_000_000);
    }

    #[test]
    fn con_sigma_zero_niente_shock() {
        let mut dati = crate::dati::dati_base();
        dati.imposta_manopola("sigma_domanda", 0.0);
        let z = [-3.0, 0.0, 0.0, 0.0, 0.0];
        let c = candidati_shock(&dati, &z, 1e12);
        assert_eq!(c.len(), 5);
        assert!(c.iter().all(|x| x.p == 0.0));
        assert_eq!(estrai(&mut Caso::nuovo(1), &c).vincitore, None);
    }

    #[test]
    fn shock_sotto_soglia_niente() {
        let c = con_z([-2.4, -2.4, 2.4, -2.4, 2.4]);
        assert_eq!(c.len(), 5);
        assert_eq!(estrai(&mut Caso::nuovo(1), &c).vincitore, None);
    }

    #[test]
    fn la_soglia_dello_shock_e_stretta() {
        assert_eq!(con_z([-2.5, 0.0, 0.0, 0.0, 0.0])[0].p, 0.0);
        assert_eq!(con_z([-2.500_001, 0.0, 0.0, 0.0, 0.0])[0].p, 1.0);
    }

    #[test]
    fn shock_favorevole_niente_evento() {
        // Innovazioni favorevoli oltre soglia: produttività su, energia giù, BCE giù.
        let c = con_z([3.0, 3.0, -3.0, 3.0, -3.0]);
        assert!(c.iter().all(|x| x.p == 0.0));
        assert_eq!(estrai(&mut Caso::nuovo(1), &c).vincitore, None);
    }

    /// Cinque normali da `Flusso::Economia` per chiusura, come nel passo; vale un evento per
    /// chiusura in cui almeno una serie esce (il tetto ne lascia uno).
    #[test]
    fn frequenza_vicina_alla_storia() {
        let dati = crate::dati::dati_base();
        let mut eventi = 0;
        for seme in 1..=500 {
            let mut caso = Caso::nuovo(seme);
            for _ in 0..20 {
                let z = std::array::from_fn(|_| caso.normale(Flusso::Economia));
                eventi += usize::from(candidati_shock(&dati, &z, 1e12).iter().any(|c| c.p > 0.0));
            }
        }
        let media = eventi as f64 / 500.0;
        assert!((0.3..1.0).contains(&media), "{media}");
    }

    #[test]
    fn livello_alle_soglie() {
        assert_eq!(livello(0.049), Fascia3::Poco);
        assert_eq!(livello(0.05), Fascia3::Medio);
        assert_eq!(livello(0.2), Fascia3::Molto);
    }
}
