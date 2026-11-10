//! Sondaggio pubblico: intenzioni vere + bias comune + effetto istituto + campionamento con
//! effetto del disegno, per quattro istituti e per incroci (condizione, fascia, regione).

use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::elettorato::{
    Condizione, Elettorato, Fascia, Intenzioni, PER_REGIONE, PUNTI, condizione_di, fascia_di,
    media_pesata,
};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Quote: una per partito, poi l'astensione.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Rilevazione {
    pub istituto: String,
    pub campione: u32,
    pub quote: Vec<f64>,
}

/// `posizione`: autocollocazione stimata sui 7 punti. `approvazione`: giudizio stimato del
/// sottocampione sul partito del presidente del consiglio, in unità d'utilità.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Incrocio {
    pub id: String,
    pub campione: u32,
    pub quote: Vec<f64>,
    pub posizione: Vec<f64>,
    pub approvazione: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Sondaggio {
    pub istituti: Vec<Rilevazione>,
    pub condizioni: Vec<Incrocio>,
    pub fasce: Vec<Incrocio>,
    pub regioni: Vec<Incrocio>,
}

/// Rinormalizza a somma 1, o `ripiego` se tutto è zero.
fn normalizza(v: Vec<f64>, ripiego: &[f64]) -> Vec<f64> {
    let somma: f64 = v.iter().sum();
    if somma > 0.0 {
        v.into_iter().map(|x| x / somma).collect()
    } else {
        ripiego.to_vec()
    }
}

/// `max(0, v_j + spostamento_j + √(d·v_j(1 − v_j)/n)·z)`, rinormalizzato; zero dove il vero è
/// zero (lo sciolto), senza spostamento.
fn campiona(
    vero: &[f64],
    spostamento: &[f64],
    n: f64,
    disegno: f64,
    z: &mut impl FnMut() -> f64,
) -> Vec<f64> {
    let q = vero
        .iter()
        .zip(spostamento)
        .map(|(p, s)| {
            let sd = (disegno * p * (1.0 - p) / n).max(0.0).sqrt();
            let q = (p + s + sd * z()).max(0.0);
            if *p > 0.0 { q } else { 0.0 }
        })
        .collect();
    normalizza(q, vero)
}

/// Una rilevazione completa. Ordine delle estrazioni fisso: istituti × alternative; poi
/// condizioni, fasce, regioni, ciascuno alternative, 7 punti, approvazione.
pub(super) fn rileva(
    dati: &DatiGioco,
    caso: &mut Caso,
    el: &Elettorato,
    vero: &Intenzioni,
    presidente: usize,
    bias: &[f64],
    effetti: &[Vec<f64>],
) -> Sondaggio {
    let mut z = || caso.normale(Flusso::Sondaggi);
    rileva_con(dati, el, vero, presidente, bias, effetti, 1.0, &mut z)
}

/// Il sondaggio privato: il primo istituto, `moltiplicatore_campione_privato` volte il campione
/// e `riduzione_istituto_privato` volte il suo effetto; il bias è quello comune. Le normali
/// vengono da `z`, mai dai flussi.
pub(super) fn rileva_privato(
    dati: &DatiGioco,
    el: &Elettorato,
    vero: &Intenzioni,
    presidente: usize,
    bias: &[f64],
    effetti: &[Vec<f64>],
    z: &mut impl FnMut() -> f64,
) -> Sondaggio {
    let riduzione = dati.manopola("riduzione_istituto_privato");
    let ridotti: Vec<Vec<f64>> = (effetti.iter().take(1))
        .map(|h| h.iter().map(|x| x * riduzione).collect())
        .collect();
    let campione = dati.manopola("moltiplicatore_campione_privato");
    rileva_con(dati, el, vero, presidente, bias, &ridotti, campione, z)
}

impl Sondaggio {
    /// Il sondaggio senza rumore: un istituto, quote e incroci uguali alle intenzioni vere,
    /// nessun bias, nessuna estrazione (approvazione compresa).
    pub(crate) fn atteso(
        dati: &DatiGioco,
        el: &Elettorato,
        vero: &Intenzioni,
        presidente: usize,
    ) -> Sondaggio {
        let nulli = vec![0.0; vero.nazionali.len()];
        let effetti = [nulli.clone()];
        rileva_con(
            dati,
            el,
            vero,
            presidente,
            &nulli,
            &effetti,
            1.0,
            &mut || 0.0,
        )
    }
}

/// `rileva` con le normali di `z`: un istituto per riga di `effetti`, ciascuno col campione
/// moltiplicato per `campione` (1 per il pubblico).
#[expect(
    clippy::too_many_arguments,
    reason = "tre ingressi del modello più il moltiplicatore; un tipo solo per questo non serve"
)]
fn rileva_con(
    dati: &DatiGioco,
    el: &Elettorato,
    vero: &Intenzioni,
    presidente: usize,
    bias: &[f64],
    effetti: &[Vec<f64>],
    campione: f64,
    z: &mut impl FnMut() -> f64,
) -> Sondaggio {
    let d = dati.elettorato();
    let (disegno, gamma, err_appr) = (
        dati.manopola("effetto_disegno"),
        dati.manopola("peso_giudizio"),
        dati.manopola("errore_approvazione"),
    );
    let alt = bias.len();

    let istituti: Vec<Rilevazione> = d
        .istituti
        .iter()
        .zip(effetti)
        .map(|(ist, h)| {
            let sposta: Vec<f64> = bias.iter().zip(h).map(|(b, x)| b + x).collect();
            let n = f64::from(ist.campione) * campione;
            Rilevazione {
                istituto: ist.nome.clone(),
                campione: n as u32,
                quote: campiona(&vero.nazionali, &sposta, n, disegno, z),
            }
        })
        .collect();

    let n_tot: f64 = campione
        * d.istituti
            .iter()
            .map(|i| f64::from(i.campione))
            .sum::<f64>();
    let elettori: Vec<f64> = (0..el.gruppi.len()).map(|i| el.elettori(d, i)).collect();
    let totale: f64 = elettori.iter().sum();
    // Effetto medio degli istituti, più il bias comune.
    let sposta: Vec<f64> = (0..alt)
        .map(|j| bias[j] + effetti.iter().map(|h| h[j]).sum::<f64>() / effetti.len() as f64)
        .collect();
    let mut incrocio = |id: String, indici: Vec<usize>| {
        let mut pesi: Vec<f64> = indici.iter().map(|&i| elettori[i]).collect();
        let n_s = (n_tot * pesi.iter().sum::<f64>() / totale).round().max(1.0);
        if pesi.iter().sum::<f64>() <= 0.0 {
            pesi.fill(1.0);
        }
        let p_s = media_pesata(&pesi, indici.iter().map(|&i| &vero.per_gruppo[i][..]), alt);
        let h_s = media_pesata(
            &pesi,
            indici.iter().map(|&i| &el.gruppi[i].posizione[..]),
            PUNTI,
        );
        let quote = campiona(&p_s, &sposta, n_s, disegno, z);
        let nulli = vec![0.0; h_s.len()];
        let posizione = campiona(&h_s, &nulli, n_s, disegno, z);
        let giudizio = indici
            .iter()
            .zip(&pesi)
            .map(|(&i, w)| w * el.gruppi[i].giudizio[presidente])
            .sum::<f64>()
            / pesi.iter().sum::<f64>();
        Incrocio {
            id,
            campione: n_s as u32,
            quote,
            posizione,
            approvazione: gamma * giudizio + err_appr / n_s.sqrt() * z(),
        }
    };
    let gruppi = 0..el.gruppi.len();
    let condizioni = Condizione::TUTTE
        .iter()
        .map(|&c| {
            let idx = gruppi.clone().filter(|&i| condizione_di(i) == c);
            incrocio(c.to_string(), idx.collect())
        })
        .collect();
    let fasce = Fascia::TUTTE
        .iter()
        .map(|&f| {
            let idx = gruppi.clone().filter(|&i| fascia_di(i) == f);
            incrocio(f.to_string(), idx.collect())
        })
        .collect();
    let regioni = d
        .regioni
        .iter()
        .enumerate()
        .map(|(r, reg)| {
            incrocio(
                reg.id.clone(),
                (r * PER_REGIONE..(r + 1) * PER_REGIONE).collect(),
            )
        })
        .collect();
    Sondaggio {
        istituti,
        condizioni,
        fasce,
        regioni,
    }
}
