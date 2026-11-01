//! Il riepilogo dal vivo dello scenario: i conti del turno zero e, per ogni posizione della
//! fondazione, consenso e maggioranza attesi senza rumore, prima della campagna.

use crate::dati::DatiGioco;
use crate::elettorato::Intenzioni;
use crate::informazione::Sondaggio;
use crate::legislatura::{Dichiarazione, SEGNAPOSTO, maggioranza, partito_nuovo, posizioni};
use crate::partita::{Partita, quote_fra_i_votanti};
use crate::partiti::Partito;
use serde::Serialize;
#[cfg(test)]
use ts_rs::TS;

/// Nessuna lettura del riepilogo passa dal caso: il seme serve solo a costruire la partita.
const SEME: u64 = 0;

/// Partiti al più nel blocco dell'avviso, giocatore compreso.
const BLOCCO: usize = 3;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct RiepilogoScenario {
    /// Punti base sul Bund, arrotondato.
    pub spread_pb: i32,
    /// % del PIL: interessi del trimestre del turno zero sul PIL del trimestre.
    pub interessi_pil: f64,
    /// % del PIL: disavanzo del trimestre del turno zero sul PIL del trimestre.
    pub disavanzo_pil: f64,
    /// % del PIL potenziale annuo: saldo delle disposizioni dell'editor rispetto alla partenza,
    /// che il disavanzo del turno zero non contiene ancora.
    pub saldo_disposizioni_pil: f64,
    /// Una voce per posizione della fondazione, in ordine.
    pub posizioni: Vec<PosizioneAttesa>,
    pub avvisi: Vec<Avviso>,
}

/// La partita fondata a `posizione`, sul sondaggio atteso. `coalizione` è il blocco migliore
/// (`blocco`), in id e in ordine d'indice; i seggi sono i suoi col riparto atteso, e `maggioranza`
/// vale se supera la metà dei seggi in entrambe le Camere.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PosizioneAttesa {
    /// Punti sull'asse 1-7.
    pub posizione: f64,
    /// Quota del giocatore fra i votanti, 0-1.
    pub quota_attesa: f64,
    pub coalizione: Vec<String>,
    /// Quota del blocco fra i votanti, 0-1.
    pub quota_coalizione: f64,
    pub seggi_camera: u32,
    pub seggi_senato: u32,
    pub maggioranza: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(tag = "tipo", rename_all = "snake_case")]
pub enum Avviso {
    /// Le posizioni da cui il blocco del giocatore non ha la maggioranza attesa.
    SenzaMaggioranza { posizioni: Vec<f64> },
}

impl Avviso {
    /// Il testo per il giocatore.
    pub fn testo(&self) -> String {
        match self {
            Avviso::SenzaMaggioranza { posizioni } => format!(
                "Da {} nessuna coalizione con te ha la maggioranza attesa in entrambe le Camere, \
                 prima della campagna",
                elenco(posizioni)
            ),
        }
    }
}

/// «1,5», «1,5 e 2,5», «1,5; 2,5 e 3,5»: virgola decimale, quindi punto e virgola fra le voci.
fn elenco(xs: &[f64]) -> String {
    let voci: Vec<String> = xs.iter().map(|x| x.to_string().replace('.', ",")).collect();
    match voci.split_last() {
        Some((ultima, prima)) if !prima.is_empty() => format!("{} e {ultima}", prima.join("; ")),
        _ => voci.concat(),
    }
}

/// Il riepilogo dei dati, deterministico e senza seme. I conti vengono dalla partita di prova
/// al turno zero; consenso e seggi da una partita fondata per posizione, sulle intenzioni vere
/// prima delle campagne IA del turno zero.
pub fn riepilogo(dati: &DatiGioco) -> RiepilogoScenario {
    let prova = Partita::nuova(dati.clone(), SEME);
    let e = prova.economia();
    let sul_pil = |euro: i64| 100.0 * euro as f64 / (e.pil_trimestrale * 1e9);
    let posizioni: Vec<PosizioneAttesa> = (posizioni(dati).into_iter())
        .map(|x| attesa(dati, x))
        .collect();
    let senza: Vec<f64> = (posizioni.iter())
        .filter(|p| !p.maggioranza)
        .map(|p| p.posizione)
        .collect();
    RiepilogoScenario {
        spread_pb: e.spread.round() as i32,
        interessi_pil: sul_pil(e.conti.spese.interessi),
        disavanzo_pil: sul_pil(e.disavanzo_trimestre),
        saldo_disposizioni_pil: prova.disposizioni().saldo(dati),
        posizioni,
        avvisi: if senza.is_empty() {
            Vec::new()
        } else {
            vec![Avviso::SenzaMaggioranza { posizioni: senza }]
        },
    }
}

/// Il blocco migliore col giocatore `g` (spec righe 282 e 285): al più `BLOCCO` partiti contigui
/// sull'asse (posizione dichiarata, a parità l'indice), quello con la quota più alta; a parità il
/// primo per inizio, poi per fine. Gli indici in ordine.
fn blocco(partiti: &[Partito], quote: &[f64], g: usize) -> Vec<usize> {
    let mut fila: Vec<usize> = (0..partiti.len()).collect();
    fila.sort_by(|&a, &b| {
        (partiti[a].posizione_dichiarata)
            .total_cmp(&partiti[b].posizione_dichiarata)
            .then(a.cmp(&b))
    });
    let at = fila.iter().position(|&j| j == g).unwrap_or(0);
    let quota = |f: &[usize]| f.iter().filter_map(|&j| quote.get(j)).sum::<f64>();
    let mut migliore = &fila[at..=at];
    for i in at.saturating_sub(BLOCCO - 1)..=at {
        for k in at..fila.len().min(i + BLOCCO) {
            if quota(&fila[i..=k]) > quota(migliore) {
                migliore = &fila[i..=k];
            }
        }
    }
    let mut b = migliore.to_vec();
    b.sort_unstable();
    b
}

fn attesa(dati: &DatiGioco, posizione: f64) -> PosizioneAttesa {
    // ponytail: una partita intera per posizione, come `anteprima_fondazione`; se pesa nella
    // schermata, la via leggera di cursori-dello-scenario.md:171-175 (un'economia per stato, per
    // posizione solo elettorato e voto).
    let nuovo = partito_nuovo(dati, SEGNAPOSTO, posizione);
    let p = Partita::costruisci(dati.clone(), SEME, Some(nuovo));
    let (g, partiti, el) = (p.giocatore(), p.partiti(), p.elettorato());
    let vero = Intenzioni::calcola(dati, el, partiti, p.governo(), None);
    let sondaggio = Sondaggio::atteso(dati, el, &vero, p.governo().presidente);
    // Le quote del sondaggio atteso sono quelle vere: si leggono dalle intenzioni.
    let quote = quote_fra_i_votanti(&vero);
    let membri = blocco(partiti, &quote, g);
    let d = Dichiarazione {
        partiti: membri.iter().copied().filter(|&j| j != g).collect(),
        cessioni: Vec::new(),
    };
    let (camera, senato) = p.quadro_su(&sondaggio).seggi(&d);
    let seggi = |v: &[u32]| membri.iter().filter_map(|&j| v.get(j)).sum();
    PosizioneAttesa {
        posizione,
        quota_attesa: quote.get(g).copied().unwrap_or(0.0),
        coalizione: (membri.iter())
            .filter_map(|&j| partiti.get(j).map(|x| x.id.clone()))
            .collect(),
        quota_coalizione: membri.iter().filter_map(|&j| quote.get(j)).sum(),
        seggi_camera: seggi(&camera),
        seggi_senato: seggi(&senato),
        maggioranza: maggioranza(&camera, &senato, &membri),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn il_testo_dell_avviso_elenca_le_posizioni() {
        let a = |posizioni: Vec<f64>| Avviso::SenzaMaggioranza { posizioni }.testo();
        let coda = "nessuna coalizione con te ha la maggioranza attesa in entrambe le Camere, \
                    prima della campagna";
        assert_eq!(a(vec![1.5]), format!("Da 1,5 {coda}"));
        assert_eq!(a(vec![1.5, 6.5]), format!("Da 1,5 e 6,5 {coda}"));
        assert_eq!(a(vec![1.5, 2.5, 6.5]), format!("Da 1,5; 2,5 e 6,5 {coda}"));
    }
}
