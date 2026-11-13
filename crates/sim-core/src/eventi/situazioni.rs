//! Le sei situazioni a soglia con isteresi (piano 9): si accendono oltre `accensione`
//! e si spengono a `spegnimento` o oltre, dall'altra parte. La recessione chiede due trimestri
//! di fila da quella parte, le altre uno.

use super::{Causa, Eventi, EventoAperto, Tono};
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::grandezze::Grandezze;
use crate::informazione::Statistica;

/// Trimestri di fila dalla parte giusta della soglia che cambiano lo stato.
pub(crate) fn trimestri(misura: &str) -> u8 {
    if misura == "crescita_pil" { 2 } else { 1 }
}

/// Una misura per situazione, nell'ordine dei dati: la grandezza nazionale (pesata sulle forze di
/// lavoro se regionale), la crescita del PIL o l'energia. `ufficiale`: i valori che la vista
/// mostra (le grandezze alla chiusura precedente, la prima stima della crescita), non il vero.
pub(crate) fn misure(
    dati: &DatiGioco,
    grandezze: &Grandezze,
    economia: &Economia,
    statistica: &Statistica,
    ufficiale: bool,
) -> Vec<f64> {
    (dati.situazioni().iter())
        .map(|s| match (s.misura.as_str(), dati.grandezza(&s.misura)) {
            (_, Some(k)) if ufficiale => grandezze.nazionale_prima(dati, k),
            (_, Some(k)) => grandezze.nazionale(dati, k),
            ("crescita_pil", _) if ufficiale => statistica.prima_stima.crescita_pil,
            ("crescita_pil", _) => statistica.vera.crescita_pil,
            _ => economia.energia,
        })
        .collect()
}

/// Aggiorna accese e contatori con `misure`; restituisce le situazioni appena accese.
pub(crate) fn aggiorna(dati: &DatiGioco, eventi: &mut Eventi, misure: &[f64]) -> Vec<usize> {
    let mut nuove = Vec::new();
    let righe = (dati.situazioni().iter().zip(misure)).zip(eventi.accese.iter_mut());
    for (i, (((s, &m), acceso), conta)) in righe.zip(eventi.contatori.iter_mut()).enumerate() {
        // Oltre `soglia` nel verso in cui la situazione si accende.
        let oltre = |soglia: f64| s.verso.segno() * (m - soglia) > 0.0;
        let verso_il_cambio = if *acceso {
            s.verso.segno() * (s.spegnimento - m) >= 0.0
        } else {
            oltre(s.accensione)
        };
        *conta = if verso_il_cambio {
            conta.saturating_add(1)
        } else {
            0
        };
        if *conta >= trimestri(&s.misura) {
            *acceso = !*acceso;
            *conta = 0;
            if *acceso {
                nuove.push(i);
            }
        }
    }
    nuove
}

/// Apre, fuori dal tetto, l'evento di ogni situazione appena accesa.
pub(crate) fn apri(
    dati: &DatiGioco,
    eventi: &mut Eventi,
    nuove: &[usize],
    misure: &[f64],
    turno: u32,
) {
    for &i in nuove {
        let (Some(s), Some(&valore)) = (dati.situazioni().get(i), misure.get(i)) else {
            continue;
        };
        let Some(evento) = dati.evento(&s.evento) else {
            continue;
        };
        eventi.aperti.push(EventoAperto {
            evento,
            turno,
            causa: Causa::Soglia {
                misura: s.misura.clone(),
                valore,
                soglia: s.accensione,
            },
            regione: None,
            bersaglio: None,
            persona: None,
            tono: Tono::Ordinario,
            costo: 0,
            scelta: None,
        });
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce è il fallimento voluto"
)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    /// Misure a riposo: né accesa né vicina a una soglia, tranne la recessione e l'energia.
    fn misure_di(dati: &DatiGioco, crescita: f64) -> Vec<f64> {
        (dati.situazioni().iter())
            .map(|s| match s.misura.as_str() {
                "crescita_pil" => crescita,
                _ => s.spegnimento,
            })
            .collect()
    }

    #[test]
    fn due_trimestri_in_calo_accendono_la_recessione() {
        let dati = dati_base();
        let mut e = Eventi::iniziali(&dati);
        let r = (dati.situazioni().iter())
            .position(|s| s.id == "recessione")
            .unwrap();
        // Due trimestri a -0,5 non bastano: la soglia è -1,0.
        for _ in 0..2 {
            assert!(aggiorna(&dati, &mut e, &misure_di(&dati, -0.5)).is_empty());
        }
        assert!(!e.accese[r]);
        // Due a -1,5 sì, e solo al secondo.
        assert!(aggiorna(&dati, &mut e, &misure_di(&dati, -1.5)).is_empty());
        assert!(!e.accese[r]);
        assert_eq!(aggiorna(&dati, &mut e, &misure_di(&dati, -1.5)), [r]);
        assert!(e.accese[r]);
        // A -0,5 resta accesa, per quanto duri.
        for _ in 0..3 {
            aggiorna(&dati, &mut e, &misure_di(&dati, -0.5));
        }
        assert!(e.accese[r]);
        // A 0,0 un trimestre non la spegne, il secondo sì.
        aggiorna(&dati, &mut e, &misure_di(&dati, 0.0));
        assert!(e.accese[r]);
        aggiorna(&dati, &mut e, &misure_di(&dati, 0.0));
        assert!(!e.accese[r]);
    }

    #[test]
    fn un_trimestre_in_calo_isolato_non_conta() {
        let dati = dati_base();
        let mut e = Eventi::iniziali(&dati);
        for crescita in [-1.5, 0.3, -1.5, 0.3] {
            aggiorna(&dati, &mut e, &misure_di(&dati, crescita));
        }
        assert!(e.accese.iter().all(|a| !a));
    }

    /// `m` sulla misura della situazione `id`, tutte le altre a riposo: se `acceso`, parte accesa.
    fn con(dati: &DatiGioco, id: &str, acceso: bool, m: f64) -> bool {
        let mut e = Eventi::iniziali(dati);
        let i = (dati.situazioni().iter()).position(|s| s.id == id).unwrap();
        e.accese[i] = acceso;
        let misure: Vec<f64> = (misure_di(dati, 0.0).into_iter())
            .enumerate()
            .map(|(k, x)| if k == i { m } else { x })
            .collect();
        aggiorna(dati, &mut e, &misure);
        e.accese[i]
    }

    #[test]
    fn le_soglie_sono_strette_all_accensione_larghe_allo_spegnimento() {
        let dati = dati_base();
        // Verso `sale`: carceri, accensione 117, spegnimento 110.
        assert!(!con(&dati, "carceri", false, 117.0));
        assert!(con(&dati, "carceri", false, 117.0001));
        assert!(!con(&dati, "carceri", true, 110.0));
        assert!(con(&dati, "carceri", true, 110.0001));
        // Verso `scende`: criminalità, accensione 85, spegnimento 92.
        assert!(!con(&dati, "criminalita", false, 85.0));
        assert!(con(&dati, "criminalita", false, 84.9999));
        assert!(!con(&dati, "criminalita", true, 92.0));
        assert!(con(&dati, "criminalita", true, 91.9999));
    }
}
