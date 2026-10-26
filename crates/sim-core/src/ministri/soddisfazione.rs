//! Soglie di fine turno di un dicastero con ministro.

use super::{Causa, Ministro, StatoDicastero, Variazione};
use crate::dati::DatiGioco;
use crate::grandezze::Verso;

/// Ogni variazione si applica subito alla soddisfazione e si accoda con il suo turno.
fn registra(m: &mut Ministro, v: &mut Vec<Variazione>, causa: Causa, delta: f64, turno: u32) {
    m.soddisfazione += delta;
    v.push(Variazione {
        causa,
        delta,
        turno,
    });
}

/// Calo del servizio, obiettivo, ritorno alla base, dimissioni. `variazione` è
/// `valore − prima` dell'indicatore, nel suo verso naturale.
pub(super) fn passo(
    st: &mut StatoDicastero,
    dati: &DatiGioco,
    verso: Verso,
    valore: f64,
    variazione: f64,
    turno: u32,
) {
    let Some(m) = st.ministro.as_mut() else {
        return;
    };
    let segno = verso.segno();
    let calo = (-segno * variazione).max(0.0);
    if calo > 0.0 {
        let costo = -dati.manopola("costo_calo_servizio") * calo / dati.manopola("scala_servizio");
        registra(m, &mut st.variazioni, Causa::IndicatoreInCalo, costo, turno);
    }
    if segno * (valore - m.obiettivo) >= 0.0 {
        let premio = dati.manopola("premio_obiettivo");
        registra(
            m,
            &mut st.variazioni,
            Causa::ObiettivoRaggiunto,
            premio,
            turno,
        );
        m.obiettivo += segno * dati.manopola("passo_obiettivo");
    }
    let base = dati.manopola("soddisfazione_base");
    m.soddisfazione += dati.manopola("ritorno_soddisfazione") * (base - m.soddisfazione);
    m.soddisfazione = m.soddisfazione.clamp(0.0, 100.0);
    m.turni_in_carica = m.turni_in_carica.saturating_add(1);
    if m.soddisfazione < dati.manopola("soglia_dimissioni") || st.dimissioni_chieste {
        st.ministro = None;
        st.dimissioni_chieste = false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[expect(
        clippy::unwrap_used,
        reason = "nei test un unwrap che fallisce e' il fallimento voluto"
    )]
    fn dati() -> DatiGioco {
        let file = crate::dati::FileDati {
            dicasteri: include_str!("../../../../dati/base/dicasteri.toml").into(),
            manopole: include_str!("../../../../dati/base/manopole.toml").into(),
            economia: include_str!("../../../../dati/base/economia.toml").into(),
            regioni: include_str!("../../../../dati/base/regioni.toml").into(),
            scenario: include_str!("../../../../dati/base/scenario.toml").into(),
            elezioni: include_str!("../../../../dati/base/elezioni.toml").into(),
            elettorato: include_str!("../../../../dati/base/elettorato.toml").into(),
            grandezze: include_str!("../../../../dati/base/grandezze.toml").into(),
            atti: include_str!("../../../../dati/base/atti.toml").into(),
        };
        DatiGioco::da_testi(&file).unwrap()
    }

    fn stato(obiettivo: f64) -> StatoDicastero {
        StatoDicastero {
            ministro: Some(Ministro {
                nome: "Test Prova".into(),
                partito: 0,
                capacita: 3,
                posizione: 4.0,
                obiettivo,
                soddisfazione: 60.0,
                turni_in_carica: 1,
            }),
            ..StatoDicastero::default()
        }
    }

    fn somma(st: &StatoDicastero, causa: Causa) -> Vec<f64> {
        (st.variazioni.iter())
            .filter(|v| v.causa == causa)
            .map(|v| v.delta)
            .collect()
    }

    #[test]
    fn calo_del_servizio_costa_in_proporzione() {
        let d = dati();
        let scala = d.manopola("scala_servizio");
        let (mut uno, mut due) = (stato(1000.0), stato(1000.0));
        passo(&mut uno, &d, Verso::Sale, 90.0, -scala, 1);
        passo(&mut due, &d, Verso::Sale, 90.0, -2.0 * scala, 1);
        let (a, b) = (
            somma(&uno, Causa::IndicatoreInCalo),
            somma(&due, Causa::IndicatoreInCalo),
        );
        assert!(a[0] < 0.0);
        assert!((b[0] - 2.0 * a[0]).abs() < 1e-12);
        let mut migliora = stato(1000.0);
        passo(&mut migliora, &d, Verso::Scende, 90.0, -scala, 1);
        assert!(somma(&migliora, Causa::IndicatoreInCalo).is_empty());
    }

    #[test]
    fn obiettivo_raggiunto_premia_e_si_sposta() {
        let d = dati();
        let (premio, passo_ob) = (
            d.manopola("premio_obiettivo"),
            d.manopola("passo_obiettivo"),
        );
        let obiettivo = |st: &StatoDicastero| st.ministro.as_ref().map(|m| m.obiettivo);
        let mut sale = stato(105.0);
        passo(&mut sale, &d, Verso::Sale, 106.0, 0.0, 1);
        assert_eq!(somma(&sale, Causa::ObiettivoRaggiunto), [premio]);
        assert_eq!(obiettivo(&sale), Some(105.0 + passo_ob));
        passo(&mut sale, &d, Verso::Sale, 106.0, 0.0, 2);
        assert_eq!(somma(&sale, Causa::ObiettivoRaggiunto), [premio]);
        let mut scende = stato(95.0);
        passo(&mut scende, &d, Verso::Scende, 94.0, 0.0, 1);
        assert_eq!(somma(&scende, Causa::ObiettivoRaggiunto), [premio]);
        assert_eq!(obiettivo(&scende), Some(95.0 - passo_ob));
    }
}
