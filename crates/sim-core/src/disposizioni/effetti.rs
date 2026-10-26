//! Ciò che le disposizioni fanno al motore: le leve dei gradini in vigore e gli effetti di
//! livello di un cambio di gradino.

use super::{DatoDisposizione, Disposizioni};
use crate::atti::Forma;
use crate::dati::DatiGioco;
use crate::grandezze::EffettoAttivo;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Le leve che il motore legge, coi valori dei gradini in vigore.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Leve {
    pub regime_pensioni: f64,
    pub eta_pensione: f64,
    pub indicizzazione_pensioni: f64,
    pub pensione_minima: f64,
    pub sussidi: f64,
    pub reddito_minimo: f64,
    pub salario_minimo: f64,
    pub cuneo: f64,
    pub forfettario: f64,
    pub accise: f64,
    pub decontribuzione: f64,
    pub autonomia: f64,
    pub durata_legislatura: f64,
    pub seggi_camera: f64,
    pub seggi_senato: f64,
}

impl Leve {
    /// Le leve dei gradini `gradino(k, d)` delle disposizioni; `valida` garantisce che ogni
    /// leva stia in una disposizione sola.
    fn dai_gradini(dati: &DatiGioco, gradino: impl Fn(usize, &DatoDisposizione) -> usize) -> Leve {
        let mut v: BTreeMap<&str, f64> = BTreeMap::new();
        for (k, d) in dati.disposizioni().iter().enumerate() {
            if let Some(g) = d.gradini.get(gradino(k, d)) {
                v.extend(g.leve.iter().map(|(n, x)| (n.as_str(), *x)));
            }
        }
        let l = |n: &str| v.get(n).copied().unwrap_or_default();
        Leve {
            regime_pensioni: l("regime_pensioni"),
            eta_pensione: l("eta_pensione"),
            indicizzazione_pensioni: l("indicizzazione_pensioni"),
            pensione_minima: l("pensione_minima"),
            sussidi: l("sussidi"),
            reddito_minimo: l("reddito_minimo"),
            salario_minimo: l("salario_minimo"),
            cuneo: l("cuneo"),
            forfettario: l("forfettario"),
            accise: l("accise"),
            decontribuzione: l("decontribuzione"),
            autonomia: l("autonomia"),
            durata_legislatura: l("durata_legislatura"),
            seggi_camera: l("seggi_camera"),
            seggi_senato: l("seggi_senato"),
        }
    }

    /// Le leve dei gradini di partenza.
    pub fn partenza(dati: &DatiGioco) -> Leve {
        Leve::dai_gradini(dati, |_, d| d.gradino(&d.partenza).unwrap_or(0))
    }
}

impl Disposizioni {
    /// Le leve dei gradini in vigore.
    pub fn leve(&self, dati: &DatiGioco) -> Leve {
        Leve::dai_gradini(dati, |k, _| self.voci.get(k).map_or(0, |v| v.in_vigore))
    }

    /// `Σ (saldo in vigore − saldo di partenza)`, % del PIL potenziale annuo.
    pub fn saldo(&self, dati: &DatiGioco) -> f64 {
        let saldo = |d: &DatoDisposizione, g: Option<usize>| {
            g.and_then(|g| d.gradini.get(g)).map_or(0.0, |g| g.saldo)
        };
        (dati.disposizioni().iter().zip(&self.voci))
            .map(|(d, v)| saldo(d, Some(v.in_vigore)) - saldo(d, d.gradino(&d.partenza)))
            .sum()
    }
}

/// Gli effetti di livello del passaggio della disposizione `d` dal gradino `da` al gradino `a`
/// al turno `turno`: per ogni grandezza toccata, `Δ = I_a − I_da` dal ritardo di `a` (di `da` se
/// `a` non la tocca), uno per regione del perimetro con `Δ · peso`, uno solo senza perimetro.
/// Nessun fattore di capacità: le disposizioni non le esegue un ministro.
pub(crate) fn effetti_del_cambio(
    dati: &DatiGioco,
    d: usize,
    da: usize,
    a: usize,
    turno: u32,
) -> Vec<EffettoAttivo> {
    let Some(disp) = dati.disposizioni().get(d) else {
        return Vec::new();
    };
    let (Some(prima), Some(dopo)) = (disp.gradini.get(da), disp.gradini.get(a)) else {
        return Vec::new();
    };
    let dicastero =
        (disp.dicastero.as_ref()).and_then(|c| dati.dicasteri().iter().position(|x| &x.id == c));
    let regioni: Vec<(Option<usize>, f64)> = if disp.perimetro.is_empty() {
        vec![(None, 1.0)]
    } else {
        (disp.perimetro.iter())
            .filter_map(|(r, p)| Some((Some(dati.regioni().iter().position(|x| &x.id == r)?), *p)))
            .collect()
    };
    let mut toccate: Vec<&str> = Vec::new();
    for e in dopo.effetti.iter().chain(&prima.effetti) {
        if !toccate.contains(&e.grandezza.as_str()) {
            toccate.push(&e.grandezza);
        }
    }
    let mut effetti = Vec::new();
    for id in toccate {
        let ea = prima.effetti.iter().find(|e| e.grandezza == id);
        let eb = dopo.effetti.iter().find(|e| e.grandezza == id);
        let delta = eb.map_or(0.0, |e| e.intensita) - ea.map_or(0.0, |e| e.intensita);
        let (Some(k), Some(e)) = (dati.grandezza(id), eb.or(ea)) else {
            continue;
        };
        if delta == 0.0 {
            continue;
        }
        effetti.extend(regioni.iter().map(|&(regione, peso)| EffettoAttivo {
            grandezza: k,
            regione,
            dicastero,
            intensita: delta * peso,
            inizio: turno.saturating_add(e.ritardo),
            forma: Forma::Livello,
            durata: 0.0,
        }));
    }
    effetti
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod test {
    use super::*;
    use crate::dati::dati_base;

    #[test]
    fn cambio_rende_la_differenza_dei_livelli() {
        let dati = dati_base();
        let d = dati.disposizione("privato_accreditato").unwrap();
        let g = |id: &str| dati.disposizioni()[d].gradino(id).unwrap();
        let e = effetti_del_cambio(&dati, d, g("ridotto"), g("ampio"), 3);
        assert_eq!(e.len(), 1);
        let sanita = dati.dicasteri().iter().position(|x| x.id == "sanita");
        assert_eq!(
            e[0].grandezza,
            dati.grandezza("qualita_servizio_sanitario").unwrap()
        );
        assert_eq!((e[0].intensita, e[0].inizio), (5.0, 4));
        assert_eq!(
            (e[0].forma, e[0].regione, e[0].dicastero),
            (Forma::Livello, None, sanita)
        );
    }

    #[test]
    fn perimetro_divide_per_regione_col_peso() {
        let dati = dati_base();
        let d = dati.disposizione("credito_zes").unwrap();
        let disp = &dati.disposizioni()[d];
        let g = |id: &str| disp.gradino(id).unwrap();
        let e = effetti_del_cambio(&dati, d, g("2_3"), g("1"), 1);
        assert_eq!(e.len(), disp.perimetro.len());
        for (r, peso) in &disp.perimetro {
            let k = dati.regioni().iter().position(|x| &x.id == r);
            let x = e.iter().find(|x| x.regione == k).unwrap();
            assert_eq!((x.intensita, x.inizio), (-4.0 * peso, 3));
        }
    }
}
