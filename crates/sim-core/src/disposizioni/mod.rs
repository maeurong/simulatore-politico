//! Catalogo delle disposizioni (`disposizioni.toml`) e le nove disposizioni elettorali del menu.

use crate::dati::Dicastero;
use crate::elettorato::Condizione;
use crate::elezioni::{Formula, LeggeElettorale, Menu, Premio};
use crate::errore::Errore;
use crate::grandezze::DatoGrandezza;
use crate::struttura::Regione;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use ts_rs::TS;

mod stato;

pub use stato::{Disposizioni, InArrivo, StatoDisposizione};

/// Le leve che il motore legge, una per disposizione: ogni leva sta in una sola disposizione.
pub const LEVE: [&str; 15] = [
    "forfettario",
    "cuneo",
    "accise",
    "decontribuzione",
    "regime_pensioni",
    "eta_pensione",
    "indicizzazione_pensioni",
    "pensione_minima",
    "sussidi",
    "reddito_minimo",
    "salario_minimo",
    "autonomia",
    "seggi_camera",
    "seggi_senato",
    "durata_legislatura",
];

/// Ultimo turno prima dei valori in arrivo: ottobre-dicembre 2026.
const TURNO_ZERO: (u16, u8) = (2026, 4);

const LEGGE_ELETTORALE: &str = "L. 180/2026";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum TipoDisposizione {
    Parametro,
    Regime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum Norma {
    Ordinaria,
    Costituzionale,
    Elettorale,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct EffettoDisposizione {
    pub grandezza: String,
    pub intensita: f64,
    pub ritardo: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Gradino {
    pub id: String,
    pub nome: String,
    /// Posizione sull'asse 1-7; assente nelle disposizioni elettorali.
    #[serde(default)]
    pub posizione: Option<f64>,
    pub saldo: f64,
    #[serde(default)]
    pub leve: BTreeMap<String, f64>,
    #[serde(default)]
    pub effetti: Vec<EffettoDisposizione>,
}

/// Il gradino che entra in vigore in un trimestre dopo il turno zero.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Arrivo {
    pub gradino: String,
    pub anno: u16,
    pub trimestre: u8,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct DatoDisposizione {
    pub id: String,
    pub nome: String,
    /// Assente per le norme costituzionali e le elettorali.
    #[serde(default)]
    pub dicastero: Option<String>,
    pub tipo: TipoDisposizione,
    pub norma: Norma,
    pub referendabile: bool,
    #[serde(default)]
    pub gruppi: Vec<Condizione>,
    /// Regione -> peso in (0, 1]; vuoto = tutto il Paese.
    #[serde(default)]
    pub perimetro: BTreeMap<String, f64>,
    pub partenza: String,
    pub legge: String,
    pub precedente: String,
    #[serde(default)]
    pub dopo_abrogazione: Option<String>,
    pub in_arrivo: Vec<Arrivo>,
    pub gradini: Vec<Gradino>,
}

impl DatoDisposizione {
    /// Posizione di un gradino per id.
    pub fn gradino(&self, id: &str) -> Option<usize> {
        self.gradini.iter().position(|g| g.id == id)
    }

    /// Posizione sull'asse 1-7 del gradino `g` (per indice).
    pub fn posizione(&self, g: usize) -> Option<f64> {
        self.gradini.get(g).and_then(|g| g.posizione)
    }
}

/// Punti base come percentuale con la virgola: `5650` -> `56,5%`.
fn percento(pb: u16) -> String {
    let decimali = format!("{:02}", pb % 100);
    let decimali = decimali.trim_end_matches('0');
    if decimali.is_empty() {
        format!("{}%", pb / 100)
    } else {
        format!("{},{decimali}%", pb / 100)
    }
}

fn id_tetto(t: Option<crate::elezioni::QuoteTetto>) -> String {
    t.map_or_else(
        || "nessuno".into(),
        |t| format!("{}_{}", t.camera_pb, t.senato_pb),
    )
}

/// Gli id dei gradini di `l`, nell'ordine delle nove disposizioni elettorali.
fn id_valori(l: &LeggeElettorale) -> [String; 9] {
    let si_no = |b: bool| if b { "si" } else { "no" }.to_string();
    [
        l.soglia_lista_pb.to_string(),
        si_no(l.coalizioni),
        l.soglia_coalizione_pb.to_string(),
        si_no(l.ripescaggio),
        match l.premio {
            Premio::Nessuno => "nessuno",
            Premio::Lista => "lista",
            Premio::Coalizione => "coalizione",
        }
        .into(),
        l.soglia_premio_pb.to_string(),
        l.taglia_premio_pb.to_string(),
        id_tetto(l.tetto),
        match l.formula {
            Formula::Hare => "hare",
            Formula::Dhondt => "dhondt",
        }
        .into(),
    ]
}

/// Le nove disposizioni elettorali dal menu, con i valori dello scenario e quelli precedenti.
pub(crate) fn elettorali(
    menu: &Menu,
    partenza: &LeggeElettorale,
    precedente: &LeggeElettorale,
) -> Vec<DatoDisposizione> {
    let da_pb = |v: &[u16]| -> Vec<(String, String)> {
        v.iter().map(|&x| (x.to_string(), percento(x))).collect()
    };
    let fisso = |v: &[(&str, &str)]| -> Vec<(String, String)> {
        v.iter().map(|&(i, n)| (i.into(), n.into())).collect()
    };
    let mut tetto: Vec<(String, String)> = menu
        .tetto
        .iter()
        .map(|t| {
            (
                id_tetto(Some(*t)),
                format!(
                    "{} alla Camera, {} al Senato",
                    percento(t.camera_pb),
                    percento(t.senato_pb)
                ),
            )
        })
        .collect();
    tetto.push(("nessuno".into(), "nessuno".into()));
    let si_no = [("no", "non ammesse"), ("si", "ammesse")];
    let si_no_ripescaggio = [("no", "no"), ("si", "sì")];
    let schede = [
        (
            "soglia_lista",
            "Soglia di lista",
            TipoDisposizione::Parametro,
            da_pb(&menu.soglia_lista_pb),
        ),
        (
            "coalizioni",
            "Coalizioni",
            TipoDisposizione::Regime,
            fisso(&si_no),
        ),
        (
            "soglia_coalizione",
            "Soglia di coalizione",
            TipoDisposizione::Parametro,
            da_pb(&menu.soglia_coalizione_pb),
        ),
        (
            "ripescaggio",
            "Ripescaggio",
            TipoDisposizione::Regime,
            fisso(&si_no_ripescaggio),
        ),
        (
            "premio",
            "Premio di maggioranza",
            TipoDisposizione::Regime,
            fisso(&[
                ("nessuno", "nessuno"),
                ("lista", "alla lista"),
                ("coalizione", "alla coalizione"),
            ]),
        ),
        (
            "soglia_premio",
            "Soglia del premio",
            TipoDisposizione::Parametro,
            da_pb(&menu.soglia_premio_pb),
        ),
        (
            "taglia_premio",
            "Taglia del premio",
            TipoDisposizione::Parametro,
            da_pb(&menu.taglia_premio_pb),
        ),
        (
            "tetto",
            "Tetto ai seggi",
            TipoDisposizione::Parametro,
            tetto,
        ),
        (
            "formula",
            "Formula di riparto",
            TipoDisposizione::Regime,
            fisso(&[("hare", "quozienti Hare"), ("dhondt", "D'Hondt")]),
        ),
    ];
    let (ora, prima) = (id_valori(partenza), id_valori(precedente));
    schede
        .into_iter()
        .zip(ora.into_iter().zip(prima))
        .map(
            |((id, nome, tipo, gradini), (partenza, precedente))| DatoDisposizione {
                id: id.into(),
                nome: nome.into(),
                dicastero: None,
                tipo,
                norma: Norma::Elettorale,
                referendabile: true,
                gruppi: Vec::new(),
                perimetro: BTreeMap::new(),
                partenza,
                legge: LEGGE_ELETTORALE.into(),
                precedente,
                dopo_abrogazione: None,
                in_arrivo: Vec::new(),
                gradini: gradini
                    .into_iter()
                    .map(|(id, nome)| Gradino {
                        id,
                        nome,
                        posizione: None,
                        saldo: 0.0,
                        leve: BTreeMap::new(),
                        effetti: Vec::new(),
                    })
                    .collect(),
            },
        )
        .collect()
}

/// La legge elettorale dai gradini (per indice) delle nove disposizioni elettorali, con i
/// gradini nell'ordine in cui li genera [`elettorali`].
fn legge_da_gradini(menu: &Menu, g: [usize; 9]) -> LeggeElettorale {
    let [
        lista,
        coalizioni,
        coalizione,
        ripescaggio,
        premio,
        soglia_premio,
        taglia,
        tetto,
        formula,
    ] = g;
    let pb = |v: &[u16], k: usize| v.get(k).copied().unwrap_or_default();
    LeggeElettorale {
        soglia_lista_pb: pb(&menu.soglia_lista_pb, lista),
        coalizioni: coalizioni == 1,
        soglia_coalizione_pb: pb(&menu.soglia_coalizione_pb, coalizione),
        ripescaggio: ripescaggio == 1,
        premio: match premio {
            0 => Premio::Nessuno,
            1 => Premio::Lista,
            _ => Premio::Coalizione,
        },
        soglia_premio_pb: pb(&menu.soglia_premio_pb, soglia_premio),
        taglia_premio_pb: pb(&menu.taglia_premio_pb, taglia),
        // L'ultimo gradino, oltre il menu, è «nessuno».
        tetto: menu.tetto.get(tetto).copied(),
        formula: if formula == 0 {
            Formula::Hare
        } else {
            Formula::Dhondt
        },
    }
}

/// Coerenza del catalogo con i dicasteri, le grandezze e le regioni.
pub(crate) fn valida(
    catalogo: &[DatoDisposizione],
    dicasteri: &[Dicastero],
    grandezze: &[DatoGrandezza],
    regioni: &[Regione],
) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Disposizioni(m));
    for (i, d) in catalogo.iter().enumerate() {
        let id = &d.id;
        if catalogo[..i].iter().any(|p| p.id == *id) {
            return ko(format!("disposizione ripetuta: {id}"));
        }
        if let Some(c) = &d.dicastero
            && !dicasteri.iter().any(|x| x.id == *c)
        {
            return ko(format!("{id}: dicastero sconosciuto {c}"));
        }
        let distinti = d
            .gradini
            .iter()
            .enumerate()
            .all(|(k, g)| d.gradini[..k].iter().all(|p| p.id != g.id));
        if !(2..=10).contains(&d.gradini.len()) || !distinti {
            return ko(format!("{id}: servono da 2 a 10 gradini distinti"));
        }
        let citati = [&d.partenza, &d.precedente]
            .into_iter()
            .chain(d.dopo_abrogazione.as_ref())
            .chain(d.in_arrivo.iter().map(|a| &a.gradino));
        for g in citati {
            if d.gradino(g).is_none() {
                return ko(format!("{id}: gradino sconosciuto {g}"));
            }
        }
        for g in &d.gradini {
            let gid = &g.id;
            let elettorale = d.norma == Norma::Elettorale;
            let in_scala = |p: f64| p.is_finite() && (1.0..=7.0).contains(&p);
            if !(elettorale || g.posizione.is_some_and(in_scala)) {
                return ko(format!("{id}: posizione di {gid} fuori da 1-7"));
            }
            if !(g.saldo.is_finite() && (-2.0..=2.0).contains(&g.saldo)) {
                return ko(format!("{id}: saldo di {gid} fuori intervallo"));
            }
            for (leva, v) in &g.leve {
                if !LEVE.contains(&leva.as_str()) {
                    return ko(format!("{id}: leva sconosciuta {leva}"));
                }
                if !v.is_finite() {
                    return ko(format!("{id}: leva {leva} non finita"));
                }
            }
            for e in &g.effetti {
                let Some(gr) = grandezze.iter().find(|x| x.id == e.grandezza) else {
                    return ko(format!("{id}: grandezza sconosciuta {}", e.grandezza));
                };
                if !(e.intensita.is_finite() && (1..=40).contains(&e.ritardo)) {
                    return ko(format!("{id}: effetto su {} fuori intervallo", gr.id));
                }
                // Le regionali non hanno limite di dicastero.
                if let Some(c) = &d.dicastero
                    && !gr.regionale
                    && !gr.dicasteri.contains(c)
                {
                    return ko(format!(
                        "{id}: {} non è nella riga del dicastero {c}",
                        gr.id
                    ));
                }
            }
        }
        let tutte: BTreeSet<&String> = d.gradini.iter().flat_map(|g| g.leve.keys()).collect();
        for leva in tutte {
            if d.gradini.iter().any(|g| !g.leve.contains_key(leva)) {
                return ko(format!("{id}: leva {leva} non è in tutti i gradini"));
            }
        }
        for (r, p) in &d.perimetro {
            if !regioni.iter().any(|x| x.id == *r) {
                return ko(format!("{id}: regione sconosciuta {r}"));
            }
            if !(p.is_finite() && *p > 0.0 && *p <= 1.0) {
                return ko(format!("{id}: peso di {r} fuori da (0, 1]"));
            }
        }
        let mut ultimo = TURNO_ZERO;
        for a in &d.in_arrivo {
            let quando = (a.anno, a.trimestre);
            if !(1..=4).contains(&a.trimestre) || quando <= ultimo {
                return ko(format!("{id}: arrivi fuori ordine"));
            }
            ultimo = quando;
        }
        if d.norma == Norma::Costituzionale && (d.referendabile || d.dicastero.is_some()) {
            return ko(format!(
                "{id}: norma costituzionale referendabile o con dicastero"
            ));
        }
    }
    for leva in LEVE {
        let dove = catalogo
            .iter()
            .filter(|d| d.gradini.iter().any(|g| g.leve.contains_key(leva)))
            .count();
        match dove {
            0 => return ko(format!("leva richiesta mancante: {leva}")),
            1 => {}
            _ => return ko(format!("leva in due disposizioni: {leva}")),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;

    fn rifiuto(catalogo: &[DatoDisposizione]) -> String {
        let d = dati_base();
        valida(catalogo, d.dicasteri(), d.grandezze(), d.regioni())
            .unwrap_err()
            .to_string()
    }

    #[test]
    fn leve_richieste_una_volta_sola() {
        let mut catalogo = dati_base().disposizioni().to_vec();
        let senza = catalogo.remove(1);
        assert_eq!(senza.id, "cuneo_fiscale");
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: leva richiesta mancante: cuneo"
        );
        catalogo.insert(1, senza);
        for g in &mut catalogo[0].gradini {
            g.leve.insert("cuneo".into(), 0.0);
        }
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: leva in due disposizioni: cuneo"
        );
    }

    #[test]
    fn leva_in_un_gradino_solo_rifiutata() {
        let mut catalogo = dati_base().disposizioni().to_vec();
        catalogo[1].gradini[0].leve.clear();
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: cuneo_fiscale: leva cuneo non è in tutti i gradini"
        );
    }
}
