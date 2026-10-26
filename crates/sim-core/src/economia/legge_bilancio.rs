//! Legge di bilancio: aliquote e scaglioni IRPEF (economia-italiana-parametri:98-106,
//! finanza-pubblica:19-21), bozza, indicizzazione, presentazione e riepilogo.

use super::Economia;
use super::finanza::{Bilancio, BudgetDicastero, ContiTrimestre, conti_del_trimestre};
use crate::dati::DatiGioco;
use crate::elettorato::Elettorato;
use crate::misure::Misure;
use crate::partita::{LIMITE_EURO, Trimestre};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Scaglione IRPEF: `aliquota` marginale sul reddito fino a `fino_a` euro annui; l'ultimo
/// scaglione non ha tetto.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Scaglione {
    pub fino_a: Option<i64>,
    pub aliquota: f64,
}

/// Le aliquote che il giocatore può cambiare.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Aliquote {
    pub irpef: Vec<Scaglione>,
    pub iva: f64,
    pub ires: f64,
    pub contributi: f64,
}

/// Aliquota massima ammessa.
const MASSIMA: f64 = 0.9;

impl Aliquote {
    /// # Errors
    /// Un messaggio che nomina la voce colpevole: `scaglioni`, `irpef`, `iva`, `ires` o `contributi`.
    pub fn valida(&self) -> Result<(), String> {
        let s = &self.irpef;
        let tetti_ok = s.split_last().is_some_and(|(ultimo, prima)| {
            ultimo.fino_a.is_none()
                && prima.iter().all(|x| x.fino_a.is_some_and(|t| t > 0))
                && prima.windows(2).all(|w| w[0].fino_a < w[1].fino_a)
        });
        if !tetti_ok {
            return Err(
                "scaglioni: servono tetti positivi e crescenti, e l'ultimo scaglione senza tetto"
                    .into(),
            );
        }
        let in_range = |a: f64| (0.0..=MASSIMA).contains(&a);
        if !s.iter().all(|x| in_range(x.aliquota)) {
            return Err(format!("irpef: aliquota fuori da 0-{MASSIMA}"));
        }
        for (nome, a) in [
            ("iva", self.iva),
            ("ires", self.ires),
            ("contributi", self.contributi),
        ] {
            if !in_range(a) {
                return Err(format!("{nome}: aliquota fuori da 0-{MASSIMA}"));
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

/// Perdita reale di una voce in %; `voce` è l'id del dicastero o `"scaglioni"`.
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
    /// Euro annui di spesa dei dicasteri e fondo in più della bozza.
    pub costo_spesa: i64,
    /// Euro annui di gettito persi rispetto alla bozza.
    pub costo_entrate: i64,
    /// % sulla spesa netta dell'ultimo trimestre chiuso, annualizzata.
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
        let lp = b.livello_prezzi_scaglioni;
        for s in &mut b.aliquote.irpef {
            s.fino_a = s.fino_a.map(|t| scala(t, lp, "scaglioni")).transpose()?;
        }
        b.livello_prezzi_scaglioni = attuale;
    }
    Ok(b)
}

/// Il bilancio `b` come lo approverebbe l'aula, o il messaggio che ne nomina il guasto. La
/// `spesa` si ricalcola dalla bozza; il livello dei prezzi di una voce (e degli scaglioni)
/// resta quello della bozza se le cifre non cambiano, altrimenti diventa `attuale`.
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
    let cifre = |d: &BudgetDicastero| [d.salari, d.acquisti, d.margine];
    let mut dicasteri = Vec::with_capacity(b.dicasteri.len());
    for (d, z) in b.dicasteri.iter().zip(&bozza.dicasteri) {
        if !cifre(d).into_iter().all(ammesso) {
            return Err(format!("voce negativa o enorme: {}", d.id));
        }
        // Voci della bozza e presentate entro 1e15: la somma non trabocca.
        let spesa = z.spesa - cifre(z).iter().sum::<i64>() + cifre(d).iter().sum::<i64>();
        if !ammesso(spesa) {
            return Err(format!("voce negativa o enorme: {}", d.id));
        }
        dicasteri.push(BudgetDicastero {
            spesa,
            livello_prezzi: if cifre(d) == cifre(z) {
                z.livello_prezzi
            } else {
                attuale
            },
            ..d.clone()
        });
    }
    if !ammesso(b.fondo_riserva) {
        return Err("voce negativa o enorme: fondo_riserva".into());
    }
    b.aliquote.valida()?;
    let soglie = |x: &Bilancio| {
        x.aliquote
            .irpef
            .iter()
            .map(|s| s.fino_a)
            .collect::<Vec<_>>()
    };
    Ok(Bilancio {
        dicasteri,
        livello_prezzi_scaglioni: if soglie(b) == soglie(bozza) {
            bozza.livello_prezzi_scaglioni
        } else {
            attuale
        },
        ..b.clone()
    })
}

/// I conti dell'ultimo trimestre chiuso rifatti con le aliquote e gli scaglioni di `b`.
fn conti_con(
    e: &Economia,
    dati: &DatiGioco,
    el: &Elettorato,
    misure: &Misure,
    b: &Bilancio,
) -> ContiTrimestre {
    // ponytail: copia intera dell'economia per una chiamata della UI; basta finché non pesa.
    let mut e = e.clone();
    e.bilancio = b.clone();
    let misure = misure.con_scaglioni(dati, el, &b.aliquote.irpef);
    conti_del_trimestre(&e, dati, &misure)
}

/// Il riepilogo di `b`, già passato da `normalizza`, contro la `bozza`.
pub(crate) fn riepilogo(
    e: &Economia,
    dati: &DatiGioco,
    el: &Elettorato,
    misure: &Misure,
    b: &Bilancio,
    bozza: &Bilancio,
) -> RiepilogoBilancio {
    let annuo = |x: &Bilancio| x.spesa_totale() + x.fondo_riserva;
    let imposte = |c: &ContiTrimestre| {
        let x = &c.entrate;
        x.irpef + x.ires + x.iva + x.contributi
    };
    let (con_b, con_bozza) = (
        conti_con(e, dati, el, misure, b),
        conti_con(e, dati, el, misure, bozza),
    );
    let attuale = 4 * e.spesa_netta_trimestre(dati);
    let proposta = attuale - 4 * e.conti.spese.dicasteri
        + annuo(b)
        + 4 * (e.conti.entrate_discrezionali - con_b.entrate_discrezionali);
    let lp = e.livello_prezzi;
    let perdita_reale = (b.dicasteri.iter())
        .map(|d| PerditaReale {
            voce: d.id.clone(),
            perdita: perdita_reale(d.livello_prezzi, lp),
        })
        .chain(std::iter::once(PerditaReale {
            voce: "scaglioni".into(),
            perdita: perdita_reale(b.livello_prezzi_scaglioni, lp),
        }))
        .collect();
    RiepilogoBilancio {
        costo_spesa: annuo(b) - annuo(bozza),
        costo_entrate: 4 * (imposte(&con_bozza) - imposte(&con_b)),
        crescita_spesa_netta: 100.0 * (proposta as f64 / attuale as f64 - 1.0),
        tetto: dati.scenario().tetto_spesa_netta(b.anno),
        perdita_reale,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn scaglioni() -> Vec<Scaglione> {
        [(Some(28_000), 0.23), (Some(50_000), 0.33), (None, 0.43)]
            .map(|(fino_a, aliquota)| Scaglione { fino_a, aliquota })
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
        a.irpef[0].aliquota = 0.95;
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
