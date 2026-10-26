//! Legge di bilancio: aliquote e scaglioni IRPEF (economia-italiana-parametri:98-106,
//! finanza-pubblica:19-21), bozza, indicizzazione, presentazione e riepilogo.

use super::Economia;
use super::finanza::{Bilancio, BudgetDicastero, ContiTrimestre, conti_del_trimestre};
use crate::dati::DatiGioco;
use crate::elettorato::Elettorato;
use crate::informazione::Statistica;
use crate::misure::Misure;
use crate::partita::{LIMITE_EURO, Trimestre};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Scaglione IRPEF: `aliquota` marginale (frazione 0-1, 0,22 = 22%) sul reddito fino a
/// `fino_a` euro annui; l'ultimo scaglione non ha tetto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Scaglione {
    pub fino_a: Option<i64>,
    pub aliquota: f64,
    /// Il livello dei prezzi a cui la soglia ha preso la sua cifra nominale: 1 al turno zero.
    #[serde(default = "uno")]
    pub livello_prezzi: f64,
}

fn uno() -> f64 {
    1.0
}

/// Livello dei prezzi ammesso per una voce: finito, in (0, 1e6].
pub(crate) fn livello_ok(x: f64) -> bool {
    x.is_finite() && x > 0.0 && x <= 1e6
}

/// Le aliquote che il giocatore può cambiare, in frazione 0-1 (0,22 = 22%).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Aliquote {
    pub irpef: Vec<Scaglione>,
    pub iva: f64,
    pub ires: f64,
    pub contributi: f64,
}

/// Aliquota massima ammessa, e come la dice un messaggio.
const MASSIMA: f64 = 0.9;
const FUORI: &str = "aliquota fuori da 0-0,9 (frazione, 0,22 = 22%)";

/// Scaglioni IRPEF ammessi.
const MAX_SCAGLIONI: usize = 12;

impl Aliquote {
    /// # Errors
    /// Un messaggio che nomina la voce colpevole: `scaglioni`, `irpef[i]`, `iva`, `ires` o
    /// `contributi`.
    pub fn valida(&self) -> Result<(), String> {
        let s = &self.irpef;
        let Some((ultimo, prima)) = s.split_last() else {
            return Err("scaglioni: ne serve almeno uno".into());
        };
        if s.len() > MAX_SCAGLIONI {
            return Err(format!(
                "scaglioni: al più {MAX_SCAGLIONI}, sono {}",
                s.len()
            ));
        }
        if ultimo.fino_a.is_some() {
            return Err(format!(
                "scaglioni: l'ultimo, irpef[{}], non deve avere tetto",
                prima.len()
            ));
        }
        let mut sotto = 0;
        for (i, x) in prima.iter().enumerate() {
            let Some(t) = x.fino_a else {
                return Err(format!(
                    "scaglioni: irpef[{i}] senza tetto, ma non è l'ultimo"
                ));
            };
            if t <= sotto {
                return Err(format!(
                    "scaglioni: il tetto di irpef[{i}] ({t}) deve superare {sotto}"
                ));
            }
            if t > LIMITE_EURO {
                return Err(format!(
                    "scaglioni: il tetto di irpef[{i}] supera 1e15 euro"
                ));
            }
            sotto = t;
        }
        let in_range = |a: f64| (0.0..=MASSIMA).contains(&a);
        for (i, x) in s.iter().enumerate() {
            if !in_range(x.aliquota) {
                return Err(format!("irpef[{i}]: {FUORI}"));
            }
            if !livello_ok(x.livello_prezzi) {
                return Err(format!("irpef[{i}]: livello_prezzi fuori da (0, 1e6]"));
            }
        }
        for (nome, a) in [
            ("iva", self.iva),
            ("ires", self.ires),
            ("contributi", self.contributi),
        ] {
            if !in_range(a) {
                return Err(format!("{nome}: {FUORI}"));
            }
        }
        Ok(())
    }
}

/// IRPEF annua in euro su `reddito`, a scaglioni marginali; 0 per redditi non positivi.
pub fn irpef(scaglioni: &[Scaglione], reddito: f64) -> f64 {
    let mut sotto = 0.0;
    let mut dovuta = 0.0;
    for s in scaglioni {
        let sopra = s.fino_a.map_or(f64::INFINITY, |t| t as f64);
        dovuta += (reddito.min(sopra) - sotto).max(0.0) * s.aliquota;
        sotto = sopra;
    }
    dovuta
}

/// Elasticità della base imponibile: `((1 − τ)/(1 − τ₀))^e` (finanza-pubblica:21).
pub(crate) fn eti(tau: f64, tau0: f64, elasticita: f64) -> f64 {
    ((1.0 - tau) / (1.0 - tau0)).powf(elasticita)
}

/// Gettito di una imposta rispetto a `τ₀`: `(τ/τ₀) · eti`. Con `τ₀ = 0` non c'è rapporto.
pub(crate) fn fattore(tau: f64, tau0: f64, elasticita: f64) -> f64 {
    let rapporto = if tau0 > 0.0 { tau / tau0 } else { 1.0 };
    rapporto * eti(tau, tau0, elasticita)
}

/// Le voci che si indicizzano: `Tutto` è tutti i dicasteri e gli scaglioni, non il fondo.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Voce {
    Tutto,
    Dicastero { id: String },
    Scaglioni,
}

/// Perdita reale di una voce in %; `voce` è l'id del dicastero o `scaglione N` (dal primo).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PerditaReale {
    pub voce: String,
    pub perdita: f64,
}

/// Cosa costa una legge di bilancio rispetto alla bozza, e il suo peso sul tetto UE.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct RiepilogoBilancio {
    /// Euro annui di spesa dei dicasteri e fondo in più della bozza; negativo se la legge spende
    /// meno della bozza.
    pub costo_spesa: i64,
    /// Euro annui di gettito persi rispetto alla bozza; negativo se la legge incassa più della
    /// bozza.
    pub costo_entrate: i64,
    /// Crescita % della spesa netta sull'anno prima di quello del bilancio, come la misura il
    /// controllo UE.
    pub crescita_spesa_netta: f64,
    /// % del percorso concordato per l'anno del bilancio.
    pub tetto: f64,
    pub perdita_reale: Vec<PerditaReale>,
}

/// `100 · (1 − livello della voce / livello attuale)`: il potere d'acquisto perso dalla cifra
/// nominale (ticket 29).
pub(crate) fn perdita_reale(livello_voce: f64, attuale: f64) -> f64 {
    100.0 * (1.0 - livello_voce / attuale)
}

/// La perdita reale di ogni scaglione con soglia, in ordine.
pub(crate) fn perdite_scaglioni(a: &Aliquote, attuale: f64) -> impl Iterator<Item = f64> + '_ {
    (a.irpef.iter())
        .filter(|s| s.fino_a.is_some())
        .map(move |s| perdita_reale(s.livello_prezzi, attuale))
}

pub(crate) const FUORI_STAGIONE: &str =
    "fuori stagione: si presenta in ottobre-dicembre o nell'esercizio provvisorio";

/// In ottobre-dicembre senza bilancio approvato, il bilancio in vigore con l'anno dopo;
/// nell'esercizio provvisorio, lo stesso con l'anno in corso; altrimenti nessuna bozza.
pub(crate) fn bozza(e: &Economia, data: Trimestre) -> Option<Bilancio> {
    let anno = if data.trimestre == 4 && e.bilancio_approvato.is_none() {
        data.anno.saturating_add(1)
    } else if e.esercizio_provvisorio(data) {
        data.anno
    } else {
        return None;
    };
    Some(Bilancio {
        anno,
        ..e.bilancio_in_vigore(data).clone()
    })
}

/// Le voci scelte × `attuale / livello della voce`, all'euro, col livello aggiornato; degli
/// scaglioni si muovono le soglie.
pub(crate) fn indicizza(b: &Bilancio, voce: &Voce, attuale: f64) -> Result<Bilancio, String> {
    if let Voce::Dicastero { id } = voce
        && !b.dicasteri.iter().any(|d| &d.id == id)
    {
        return Err(format!("dicastero sconosciuto: {id}"));
    }
    let scala = |x: i64, livello: f64, nome: &str| {
        let y = (x as f64 * attuale / livello).round();
        if (0.0..=LIMITE_EURO as f64).contains(&y) {
            Ok(y as i64)
        } else {
            Err(format!("voce negativa o enorme: {nome}"))
        }
    };
    let tutto = *voce == Voce::Tutto;
    let mut b = b.clone();
    for d in &mut b.dicasteri {
        if tutto || matches!(voce, Voce::Dicastero { id } if *id == d.id) {
            let (lp, id) = (d.livello_prezzi, d.id.as_str());
            d.spesa = scala(d.spesa, lp, id)?;
            d.salari = scala(d.salari, lp, id)?;
            d.acquisti = scala(d.acquisti, lp, id)?;
            d.margine = scala(d.margine, lp, id)?;
            d.livello_prezzi = attuale;
        }
    }
    if tutto || *voce == Voce::Scaglioni {
        for s in &mut b.aliquote.irpef {
            s.fino_a = (s.fino_a)
                .map(|t| scala(t, s.livello_prezzi, "scaglioni"))
                .transpose()?;
            s.livello_prezzi = attuale;
        }
    }
    Ok(b)
}

/// Il bilancio `b` come lo approverebbe l'aula, o il messaggio che ne nomina il guasto. La
/// `spesa` è salari + acquisti + margine più il resto presentato, limitato fra quello della
/// bozza e quello indicizzato per intero; il livello dei prezzi di una voce (e degli scaglioni)
/// resta quello della bozza se la cifra non cambia, altrimenti la segue (`livello_pesato`).
pub(crate) fn normalizza(b: &Bilancio, bozza: &Bilancio, attuale: f64) -> Result<Bilancio, String> {
    if b.anno != bozza.anno {
        return Err(format!("anno {}, atteso {}", b.anno, bozza.anno));
    }
    if !b
        .dicasteri
        .iter()
        .map(|d| &d.id)
        .eq(bozza.dicasteri.iter().map(|d| &d.id))
    {
        return Err("dicasteri diversi da quelli dei dati".into());
    }
    let ammesso = |x: i64| (0..=LIMITE_EURO).contains(&x);
    let cifre = |d: &BudgetDicastero| d.salari + d.acquisti + d.margine;
    let mut dicasteri = Vec::with_capacity(b.dicasteri.len());
    for (d, z) in b.dicasteri.iter().zip(&bozza.dicasteri) {
        if ![d.spesa, d.salari, d.acquisti, d.margine]
            .into_iter()
            .all(ammesso)
        {
            return Err(format!("voce negativa o enorme: {}", d.id));
        }
        // Il resto della spesa, fuori da salari, acquisti e margine, sta fra quello della bozza
        // e quello indicizzato per intero. Voci entro 1e15: le somme non traboccano.
        let resto_bozza = z.spesa - cifre(z);
        // `as` satura: un livello quasi nullo da un salvataggio non fa traboccare.
        let indicizzato = (resto_bozza as f64 * attuale / z.livello_prezzi).round() as i64;
        let resto =
            (d.spesa - cifre(d)).clamp(resto_bozza.min(indicizzato), resto_bozza.max(indicizzato));
        let spesa = resto + cifre(d);
        if !ammesso(spesa) {
            return Err(format!("voce negativa o enorme: {}", d.id));
        }
        dicasteri.push(BudgetDicastero {
            id: d.id.clone(),
            spesa,
            salari: d.salari,
            acquisti: d.acquisti,
            margine: d.margine,
            livello_prezzi: livello_pesato(z.livello_prezzi, z.spesa, spesa, attuale),
        });
    }
    if !ammesso(b.fondo_riserva) {
        return Err("voce negativa o enorme: fondo_riserva".into());
    }
    let a = &b.aliquote;
    a.valida()?;
    // La cifra di uno scaglione è la sua soglia, contro quella dello scaglione con lo stesso
    // posto nella bozza; senza soglia nella bozza vale `attuale`.
    let irpef = (a.irpef.iter().enumerate())
        .map(|(i, s)| {
            let z = bozza.aliquote.irpef.get(i);
            Scaglione {
                fino_a: s.fino_a,
                aliquota: s.aliquota,
                livello_prezzi: livello_pesato(
                    z.map_or(attuale, |z| z.livello_prezzi),
                    z.and_then(|z| z.fino_a).unwrap_or(0),
                    s.fino_a.unwrap_or(0),
                    attuale,
                ),
            }
        })
        .collect();
    Ok(Bilancio {
        anno: b.anno,
        dicasteri,
        aliquote: Aliquote {
            irpef,
            iva: a.iva,
            ires: a.ires,
            contributi: a.contributi,
        },
        fondo_riserva: b.fondo_riserva,
    })
}

/// Livello dei prezzi di una voce la cui cifra passa da `prima` (nella bozza, al `livello`) a
/// `dopo`: `min(attuale, livello · dopo/prima)`, così un taglio nominale alza la perdita reale e
/// un aumento oltre l'indicizzazione la porta a 0, mai sotto; `prima ≤ 0` → `attuale`.
fn livello_pesato(livello: f64, prima: i64, dopo: i64, attuale: f64) -> f64 {
    if dopo == prima {
        livello
    } else if prima <= 0 {
        attuale
    } else if dopo as f64 + 2.0 >= prima as f64 * attuale / livello {
        // Indicizzata: la spesa è la somma di quattro cifre arrotondate all'euro, al più 2 € in
        // meno.
        attuale
    } else {
        // ponytail: pavimento a un milionesimo dell'indice, perché una voce azzerata con
        // livello 0 non sarebbe più salvabile né indicizzabile.
        (livello * dopo as f64 / prima as f64).clamp(attuale * 1e-6, attuale)
    }
}

/// I conti dell'ultimo trimestre chiuso rifatti con le aliquote e gli scaglioni di `b`.
fn conti_con(
    e: &Economia,
    dati: &DatiGioco,
    el: &Elettorato,
    misure: &Misure,
    b: &Bilancio,
) -> ContiTrimestre {
    let misure = misure.clone().con_scaglioni(dati, el, &b.aliquote.irpef);
    conti_del_trimestre(e, b, dati, &misure)
}

/// Il riepilogo di `b`, già passato da `normalizza`, contro la `bozza`, con l'indice dei prezzi
/// e l'inflazione pubblicati da `stat`.
pub(crate) fn riepilogo(
    e: &Economia,
    dati: &DatiGioco,
    el: &Elettorato,
    misure: &Misure,
    b: &Bilancio,
    bozza: &Bilancio,
    stat: &Statistica,
) -> Result<RiepilogoBilancio, String> {
    let annuo = |x: &Bilancio| x.spesa_totale() + x.fondo_riserva;
    let imposte = |c: &ContiTrimestre| {
        let x = &c.entrate;
        x.irpef + x.ires + x.iva + x.contributi
    };
    let (con_b, con_bozza) = (
        conti_con(e, dati, el, misure, b),
        conti_con(e, dati, el, misure, bozza),
    );
    // La stessa grandezza del controllo UE: la spesa netta dell'anno prima di quello del
    // bilancio, annualizzata sui trimestri già chiusi (tre in ottobre-dicembre).
    let rif = b.anno.saturating_sub(1);
    // In `f64`: la spesa netta per anno di un salvataggio non ha tetto, i prodotti in `i64`
    // potrebbero traboccare.
    let chiusi = e.storico.iter().filter(|t| t.data.anno == rif).count() as f64;
    let attuale = match e.regole_ue.spesa_netta.get(&rif) {
        Some(&sn) if chiusi > 0.0 => sn as f64 * 4.0 / chiusi,
        _ => 0.0,
    };
    if attuale <= 0.0 {
        return Err("peso sul tetto UE non calcolabile: spesa netta di riferimento nulla".into());
    }
    // Spesa dei dicasteri di `b` al posto di quella dell'anno; il resto cresce con l'inflazione
    // pubblicata, meno le entrate discrezionali di `b` rispetto a quelle in vigore (le stesse
    // aliquote della bozza). Il fondo di riserva non è spesa finché nessuno lo spende.
    let resto =
        (attuale - e.bilancio.spesa_totale() as f64) * (1.0 + stat.prima_stima.inflazione / 100.0);
    let proposta = b.spesa_totale() as f64 + resto
        - 4.0 * (con_b.entrate_discrezionali - con_bozza.entrate_discrezionali) as f64;
    let indice = stat.indice_prezzi;
    let perdite = (b.dicasteri.iter())
        .map(|d| PerditaReale {
            voce: d.id.clone(),
            perdita: perdita_reale(d.livello_prezzi, indice),
        })
        .chain(
            perdite_scaglioni(&b.aliquote, indice)
                .enumerate()
                .map(|(i, perdita)| PerditaReale {
                    voce: format!("scaglione {}", i + 1),
                    perdita,
                }),
        )
        .collect();
    Ok(RiepilogoBilancio {
        costo_spesa: annuo(b) - annuo(bozza),
        costo_entrate: 4 * (imposte(&con_bozza) - imposte(&con_b)),
        crescita_spesa_netta: 100.0 * (proposta / attuale - 1.0),
        tetto: dati.scenario().tetto_spesa_netta(b.anno),
        perdita_reale: perdite,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn scaglioni() -> Vec<Scaglione> {
        [(Some(28_000), 0.23), (Some(50_000), 0.33), (None, 0.43)]
            .map(|(fino_a, aliquota)| Scaglione {
                fino_a,
                aliquota,
                livello_prezzi: 1.0,
            })
            .to_vec()
    }

    fn con(irpef: Vec<Scaglione>) -> Aliquote {
        Aliquote {
            irpef,
            iva: 0.22,
            ires: 0.24,
            contributi: 0.33,
        }
    }

    #[test]
    fn aliquota_piu_alta_piu_gettito_ma_meno_che_in_proporzione() {
        let f = fattore(0.24, 0.22, 0.25);
        assert!(f > 1.0 && f < 0.24 / 0.22, "{f}");
        assert_eq!(fattore(0.22, 0.22, 0.25), 1.0);
    }

    #[test]
    fn la_validazione_nomina_la_voce() {
        assert!(con(scaglioni()).valida().is_ok());
        let mut a = con(scaglioni());
        a.irpef[1].fino_a = Some(20_000);
        assert!(a.valida().err().unwrap_or_default().contains("scaglioni"));
        assert!(
            con(vec![])
                .valida()
                .err()
                .unwrap_or_default()
                .contains("scaglioni")
        );
        let mut a = con(scaglioni());
        a.irpef[2].fino_a = Some(90_000);
        assert!(a.valida().err().unwrap_or_default().contains("scaglioni"));
        let mut a = con(scaglioni());
        a.irpef[1].fino_a = Some(LIMITE_EURO + 1);
        assert!(a.valida().err().unwrap_or_default().contains("scaglioni"));
        let tredici = (1..=13)
            .map(|k| Scaglione {
                fino_a: (k < 13).then_some(k * 10_000),
                aliquota: 0.2,
                livello_prezzi: 1.0,
            })
            .collect();
        assert!(
            con(tredici)
                .valida()
                .err()
                .unwrap_or_default()
                .contains("scaglioni")
        );
        let mut a = con(scaglioni());
        a.irpef[0].aliquota = 0.95;
        let e = a.valida().err().unwrap_or_default();
        assert!(
            e.starts_with("irpef[0]: ") && e.contains("0-0,9 (frazione, 0,22 = 22%)"),
            "{e}"
        );
        let mut a = con(scaglioni());
        a.irpef[0].livello_prezzi = 1e7;
        assert!(a.valida().err().unwrap_or_default().contains("irpef"));
        for (nome, ritocco) in [
            (
                "iva",
                (|a: &mut Aliquote| a.iva = f64::NAN) as fn(&mut Aliquote),
            ),
            ("ires", |a| a.ires = -0.1),
            ("contributi", |a| a.contributi = 1.0),
        ] {
            let mut a = con(scaglioni());
            ritocco(&mut a);
            assert!(
                a.valida().err().unwrap_or_default().starts_with(nome),
                "{nome}"
            );
        }
    }
}
