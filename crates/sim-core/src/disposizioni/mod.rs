//! Catalogo delle disposizioni (`disposizioni.toml`) e le nove disposizioni elettorali del menu.

use crate::dati::Dicastero;
use crate::elettorato::Condizione;
use crate::elezioni::{Formula, LeggeElettorale, Menu, Premio};
use crate::errore::Errore;
use crate::grandezze::DatoGrandezza;
use crate::partita::Trimestre;
use crate::struttura::Regione;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
#[cfg(test)]
use ts_rs::TS;

mod effetti;
mod stato;

pub use effetti::Leve;
pub(crate) use effetti::effetti_del_cambio;
pub(crate) use stato::data_di;
pub use stato::{Disposizioni, InArrivo, StatoDisposizione};

/// Le leve che il motore legge, una per disposizione: ogni leva sta in una sola disposizione.
/// Con l'intervallo ammesso, largo ma finito, nelle unità della leva.
pub const LEVE: [(&str, f64, f64); 15] = [
    ("forfettario", 0.0, 3.0),
    ("cuneo", 0.0, 20.0),
    ("accise", -20.0, 20.0),
    ("decontribuzione", 0.0, 2.0),
    ("regime_pensioni", 0.0, 2.0),
    ("eta_pensione", 55.0, 75.0),
    ("indicizzazione_pensioni", 0.0, 1.5),
    ("pensione_minima", 0.0, 3.0),
    ("sussidi", 0.0, 3.0),
    ("reddito_minimo", 0.0, 20000.0),
    ("salario_minimo", 0.0, 30.0),
    ("autonomia", 0.0, 1.0),
    ("seggi_camera", 100.0, 1000.0),
    ("seggi_senato", 50.0, 500.0),
    ("durata_legislatura", 3.0, 7.0),
];

/// Leva che vale un regime per indice: un valore fra due interi non è nessun regime.
const LEVA_INTERA: &str = "regime_pensioni";

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
    /// Indice di un gradino per id.
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
    let id_si_no = |b: bool| if b { "si" } else { "no" }.to_string();
    [
        l.soglia_lista_pb.to_string(),
        id_si_no(l.coalizioni),
        l.soglia_coalizione_pb.to_string(),
        id_si_no(l.ripescaggio),
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
    let nomi_coalizioni = [("no", "non ammesse"), ("si", "ammesse")];
    let nomi_ripescaggio = [("no", "non previsto"), ("si", "previsto")];
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
            fisso(&nomi_coalizioni),
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
            fisso(&nomi_ripescaggio),
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
        if d.legge.trim().is_empty() {
            return ko(format!("{id}: legge mancante"));
        }
        let citati = [("partenza", &d.partenza), ("precedente", &d.precedente)]
            .into_iter()
            .chain(d.dopo_abrogazione.iter().map(|g| ("dopo_abrogazione", g)))
            .chain(d.in_arrivo.iter().map(|a| ("in_arrivo", &a.gradino)));
        for (campo, g) in citati {
            if d.gradino(g).is_none() {
                return ko(format!("{id}: gradino sconosciuto {g} in {campo}"));
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
                return ko(format!("{id}: saldo di {gid} fuori da -2…2"));
            }
            for (leva, v) in &g.leve {
                let Some(&(_, min, max)) = LEVE.iter().find(|(n, _, _)| n == leva) else {
                    return ko(format!("{id}: leva sconosciuta {leva}"));
                };
                if !(min..=max).contains(v) {
                    return ko(format!("{id}: leva {leva} di {gid} fuori da {min}-{max}"));
                }
                if leva == LEVA_INTERA && v.fract() != 0.0 {
                    return ko(format!("{id}: leva {leva} di {gid} non intera"));
                }
            }
            for e in &g.effetti {
                let Some(gr) = grandezze.iter().find(|x| x.id == e.grandezza) else {
                    return ko(format!("{id}: grandezza sconosciuta {}", e.grandezza));
                };
                let gr_id = &gr.id;
                // Metà intervallo: il Δ fra due gradini resta nel tetto del salvataggio.
                let tetto = (gr.max - gr.min) / 2.0;
                if !(e.intensita.is_finite() && e.intensita.abs() <= tetto) {
                    return ko(format!(
                        "{id}: intensità dell'effetto su {gr_id} oltre ±{tetto}"
                    ));
                }
                if !(1..=40).contains(&e.ritardo) {
                    return ko(format!(
                        "{id}: ritardo dell'effetto su {gr_id} fuori da 1-40"
                    ));
                }
                if !d.perimetro.is_empty() && !gr.regionale {
                    return ko(format!(
                        "{id}: ha un perimetro ma l'effetto tocca {gr_id}, che è nazionale"
                    ));
                }
                // Le regionali non hanno limite di dicastero.
                if let Some(c) = &d.dicastero
                    && !gr.regionale
                    && !gr.dicasteri.contains(c)
                {
                    return ko(format!(
                        "{id}: la grandezza {gr_id} non appartiene al dicastero {c}"
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
        // L'autonomia sposta domanda per regione intera: un peso parziale non entra nel conto.
        let autonomia = d.gradini.iter().any(|g| g.leve.contains_key("autonomia"));
        for (r, p) in &d.perimetro {
            if !regioni.iter().any(|x| x.id == *r) {
                return ko(format!("{id}: regione sconosciuta {r}"));
            }
            if !(p.is_finite() && *p > 0.0 && *p <= 1.0) {
                return ko(format!("{id}: peso di {r} fuori da (0, 1]"));
            }
            if autonomia && *p != 1.0 {
                return ko(format!("{id}: peso di {r} deve essere 1"));
            }
        }
        let zero = Trimestre::TURNO_ZERO;
        let mut ultimo = (zero.anno, zero.trimestre);
        for a in &d.in_arrivo {
            let t = a.trimestre;
            if !(1..=4).contains(&t) {
                return ko(format!("{id}: trimestre {t} fuori da 1-4"));
            }
            let quando = (a.anno, t);
            if quando <= ultimo {
                return ko(format!("{id}: valori in arrivo non in ordine di turno"));
            }
            ultimo = quando;
        }
        if d.norma == Norma::Costituzionale && (d.referendabile || d.dicastero.is_some()) {
            return ko(format!(
                "{id}: una norma costituzionale non è referendabile e non ha dicastero"
            ));
        }
    }
    for (leva, _, _) in LEVE {
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

    fn indice(catalogo: &[DatoDisposizione], id: &str) -> usize {
        catalogo
            .iter()
            .position(|d| d.id == id)
            .unwrap_or(usize::MAX)
    }

    #[test]
    fn leva_fuori_dal_suo_intervallo_rifiutata() {
        let base = dati_base().disposizioni().to_vec();
        let k = indice(&base, "regime_pensioni");
        for (v, atteso) in [
            (
                3.0,
                "regime_pensioni: leva regime_pensioni di contributivo fuori da 0-2",
            ),
            (
                0.5,
                "regime_pensioni: leva regime_pensioni di contributivo non intera",
            ),
            (
                f64::NAN,
                "regime_pensioni: leva regime_pensioni di contributivo fuori da 0-2",
            ),
        ] {
            let mut catalogo = base.clone();
            catalogo[k].gradini[1]
                .leve
                .insert("regime_pensioni".into(), v);
            assert_eq!(
                rifiuto(&catalogo),
                format!("disposizioni non valide: {atteso}")
            );
        }
        let mut catalogo = base.clone();
        let a = indice(&catalogo, "accise");
        catalogo[a].gradini[0].leve.insert("accise".into(), -21.0);
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: accise: leva accise di basse fuori da -20-20"
        );
    }

    #[test]
    fn autonomia_vuole_pesi_pieni() {
        let mut catalogo = dati_base().disposizioni().to_vec();
        let k = indice(&catalogo, "autonomia_differenziata");
        catalogo[k].perimetro.insert("lombardia".into(), 0.5);
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: autonomia_differenziata: peso di lombardia deve essere 1"
        );
    }

    #[test]
    fn ritardo_e_arrivi_rifiutati_ciascuno_col_suo_messaggio() {
        let base = dati_base().disposizioni().to_vec();
        let k = indice(&base, "privato_accreditato");
        let mut catalogo = base.clone();
        for g in &mut catalogo[k].gradini {
            for e in &mut g.effetti {
                e.ritardo = 41;
            }
        }
        assert!(
            rifiuto(&catalogo).ends_with(
                "privato_accreditato: ritardo dell'effetto su qualita_servizio_sanitario fuori da 1-40"
            ),
            "{}",
            rifiuto(&catalogo)
        );
        let z = indice(&base, "credito_zes");
        let mut catalogo = base.clone();
        catalogo[z].in_arrivo[0].trimestre = 5;
        assert!(rifiuto(&catalogo).ends_with("credito_zes: trimestre 5 fuori da 1-4"));
        let mut catalogo = base.clone();
        catalogo[z].in_arrivo.swap(0, 1);
        assert!(
            rifiuto(&catalogo).ends_with("credito_zes: valori in arrivo non in ordine di turno")
        );
    }

    #[test]
    fn legge_vuota_rifiutata() {
        let mut catalogo = dati_base().disposizioni().to_vec();
        let k = indice(&catalogo, "cuneo_fiscale");
        catalogo[k].legge.clear();
        assert_eq!(
            rifiuto(&catalogo),
            "disposizioni non valide: cuneo_fiscale: legge mancante"
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
