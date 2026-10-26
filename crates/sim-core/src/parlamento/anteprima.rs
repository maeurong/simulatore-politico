//! L'anteprima di una legge sugli effetti nominali scritti nei dati, mai su una simulazione dello
//! stato vero (scelta 8). La fascia passa per la stima del ministro proponente, che sbaglia meno
//! quanto più è capace (consiglieri-e-informazione-imperfetta:21, 24).

use super::{Misura, Parlamento, StatoAula, nome_misura};
use crate::caso::Caso;
use crate::dati::DatiGioco;
use crate::disposizioni::{Gradino, variazioni};
use crate::economia::euro;
use crate::vista::{Anteprima, EffettoStimato, Fascia3, VoceLeva};

/// `Poco` sotto `soglia_fascia_poco` di `|Δ| / (max − min)` della grandezza `k`, `Medio` sotto
/// `soglia_fascia_medio`, altrimenti `Molto`.
fn fascia_vera(dati: &DatiGioco, k: usize, delta: f64) -> Fascia3 {
    let g = &dati.grandezze()[k];
    let r = delta.abs() / (g.max - g.min);
    if r < dati.manopola("soglia_fascia_poco") {
        Fascia3::Poco
    } else if r < dati.manopola("soglia_fascia_medio") {
        Fascia3::Medio
    } else {
        Fascia3::Molto
    }
}

/// Con `u1 < errore · (5 − capacita) / 4` la fascia si sposta di una, in giù con `u2 < 0,5` e in
/// su altrimenti, restando fra le tre; altrimenti resta la vera.
pub(crate) fn stima_fascia(vera: Fascia3, capacita: f64, u1: f64, u2: f64, errore: f64) -> Fascia3 {
    if u1 >= errore * (5.0 - capacita) / 4.0 {
        return vera;
    }
    match (vera, u2 >= 0.5) {
        (Fascia3::Poco | Fascia3::Medio, false) => Fascia3::Poco,
        (Fascia3::Poco, true) | (Fascia3::Molto, false) => Fascia3::Medio,
        (Fascia3::Medio | Fascia3::Molto, true) => Fascia3::Molto,
    }
}

/// L'anteprima della misura `m` non ancora in calendario: costo annuo sul PIL potenziale annuo
/// nominale `pil_annuo` (euro), fasce stimate con la `capacita` del proponente da un estrattore
/// estemporaneo, che non avanza nessun flusso; linee, forbice e seggi come in calendario.
pub(crate) fn anteprima(
    parlamento: &Parlamento,
    aula: &StatoAula,
    m: &Misura,
    capacita: f64,
    caso: &Caso,
    pil_annuo: f64,
) -> Anteprima {
    let dati = aula.dati;
    let chiave = (m.valori.iter().enumerate())
        .map(|(k, &(d, g))| (k as u64 + 1) * (64 * d as u64 + g as u64))
        .sum::<u64>()
        + u64::from(m.presentata) * 1_000_003;
    let mut rng = caso.estemporaneo(chiave);
    let errore = dati.manopola("errore_fascia");
    let (mut saldo, mut effetti, mut leve, mut gruppi) = (0.0, Vec::new(), Vec::new(), Vec::new());
    for &(d, g) in &m.valori {
        let x = &dati.disposizioni()[d];
        let (prima, dopo) = (
            &x.gradini[aula.disposizioni.voci[d].in_vigore],
            &x.gradini[g],
        );
        saldo += dopo.saldo - prima.saldo;
        let nuove = (dopo.leve.keys()).filter(|n| !prima.leve.contains_key(*n));
        for n in prima.leve.keys().chain(nuove) {
            let valore = |g: &Gradino| g.leve.get(n).copied().unwrap_or(0.0);
            let (da, a) = (valore(prima), valore(dopo));
            if da != a {
                leve.push(VoceLeva {
                    leva: n.clone(),
                    da,
                    a,
                });
            }
        }
        for (id, delta, ritardo) in variazioni(prima, dopo) {
            let Some(k) = dati.grandezza(id) else {
                continue;
            };
            let (u1, u2) = (rng.uniforme(), rng.uniforme());
            effetti.push(EffettoStimato {
                grandezza: id.to_owned(),
                migliora: delta * dati.grandezze()[k].verso.segno() > 0.0,
                fascia: stima_fascia(fascia_vera(dati, k, delta), capacita, u1, u2, errore),
                ritardo,
            });
        }
        for c in &x.gruppi {
            if !gruppi.contains(c) {
                gruppi.push(*c);
            }
        }
    }
    let p = aula.previsione(m);
    Anteprima {
        nome: nome_misura(dati, m),
        forma: m.forma,
        proponente: (m.proponente)
            .and_then(|i| dati.dicasteri().get(i))
            .map(|d| d.id.clone()),
        costo_annuo: euro(saldo / 100.0 * pil_annuo),
        effetti,
        leve,
        gruppi,
        linee: aula.voci_linea(&p.linea),
        forbice: parlamento.forbice(&p),
        seggi: aula.voci_seggi(&p.seggi),
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod test {
    use super::*;
    use crate::dati::dati_base;
    use Fascia3::{Medio, Molto, Poco};

    const ERRORE: f64 = 0.4;

    #[test]
    fn capacita_5_non_sbaglia_mai() {
        for vera in [Poco, Medio, Molto] {
            for u1 in [1e-12, 0.1, 0.4, 1.0] {
                for u2 in [0.1, 0.9] {
                    assert_eq!(stima_fascia(vera, 5.0, u1, u2, ERRORE), vera);
                }
            }
        }
    }

    #[test]
    fn capacita_1_sbaglia_quando_u1_sta_sotto_l_errore() {
        assert_eq!(stima_fascia(Medio, 1.0, ERRORE - 1e-9, 0.9, ERRORE), Molto);
        assert_eq!(stima_fascia(Medio, 1.0, ERRORE + 1e-9, 0.9, ERRORE), Medio);
    }

    #[test]
    fn fascia_resta_fra_le_tre() {
        assert_eq!(stima_fascia(Poco, 1.0, 0.01, 0.1, ERRORE), Poco);
        assert_eq!(stima_fascia(Molto, 1.0, 0.01, 0.9, ERRORE), Molto);
    }

    #[test]
    fn u2_sceglie_il_verso() {
        assert_eq!(stima_fascia(Medio, 1.0, 0.01, 0.49, ERRORE), Poco);
        assert_eq!(stima_fascia(Medio, 1.0, 0.01, 0.5, ERRORE), Molto);
    }

    #[test]
    fn fascia_vera_sulle_soglie_di_qualita_sanitaria() {
        let dati = dati_base();
        let k = dati.grandezza("qualita_servizio_sanitario").unwrap();
        // `ampio`: 3 su 200 fa 0,015.
        assert_eq!(fascia_vera(&dati, k, 3.0), Poco);
        assert_eq!(fascia_vera(&dati, k, -6.0), Medio);
        assert_eq!(fascia_vera(&dati, k, 10.0), Molto);
    }
}
