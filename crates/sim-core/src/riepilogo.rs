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
    /// Le situazioni già accese al turno zero, nell'ordine dei dati.
    SituazioniAccese { nomi: Vec<String> },
    /// Lo spread del turno zero è già alla soglia della crisi del debito.
    CrisiDelDebito { spread_pb: i32, soglia_pb: i32 },
}

impl Avviso {
    /// Il testo per il giocatore.
    pub fn testo(&self) -> String {
        match self {
            Avviso::SenzaMaggioranza { posizioni } => format!(
                "Da {}, nessuna coalizione con te ha la maggioranza attesa in entrambe le Camere, \
                 prima della campagna.",
                elenco(
                    posizioni
                        .iter()
                        .map(|x| x.to_string().replace('.', ","))
                        .collect()
                )
            ),
            Avviso::SituazioniAccese { nomi } => format!(
                "Già accese al turno zero: {}. Pesano finché non tornano sotto la soglia di \
                 spegnimento.",
                elenco(nomi.clone())
            ),
            Avviso::CrisiDelDebito {
                spread_pb,
                soglia_pb,
            } => format!(
                "Spread a {spread_pb} punti base, oltre la soglia di {soglia_pb}: alla prima \
                 chiusura le aste vanno scoperte e dovrai scegliere fra MES e ristrutturazione."
            ),
        }
    }
}

/// «1,5», «1,5 e 2,5», «1,5; 2,5 e 3,5»: virgola decimale, quindi punto e virgola fra le voci.
fn elenco(voci: Vec<String>) -> String {
    match voci.split_last() {
        Some((ultima, prima)) if !prima.is_empty() => format!("{} e {ultima}", prima.join("; ")),
        _ => voci.concat(),
    }
}

/// Il riepilogo dei dati, deterministico e senza seme: una partita fondata per posizione, sulle
/// intenzioni vere prima delle campagne IA del turno zero. I conti vengono dalla prima.
pub fn riepilogo(dati: &DatiGioco) -> RiepilogoScenario {
    let mut r = None;
    let posizioni: Vec<PosizioneAttesa> = (posizioni(dati).into_iter())
        .map(|x| {
            // ponytail: una partita intera per posizione, come `anteprima_fondazione`; se pesa
            // nella schermata, la via leggera di cursori-dello-scenario.md:171-175 (un'economia
            // per stato, per posizione solo elettorato e voto).
            let nuovo = partito_nuovo(dati, SEGNAPOSTO, x);
            let p = Partita::costruisci(dati.clone(), SEME, Some(nuovo));
            r.get_or_insert_with(|| (conti(dati, &p), di_stato(dati, &p)));
            attesa(dati, &p, x)
        })
        .collect();
    // Nessuna posizione se i partiti dello scenario stanno tutti in un punto: i conti allora
    // vengono dalla partita del governo uscente.
    let (mut r, di_stato) = r.unwrap_or_else(|| {
        let p = Partita::nuova(dati.clone(), SEME);
        (conti(dati, &p), di_stato(dati, &p))
    });
    let senza: Vec<f64> = (posizioni.iter())
        .filter(|p| !p.maggioranza)
        .map(|p| p.posizione)
        .collect();
    if !senza.is_empty() {
        r.avvisi.push(Avviso::SenzaMaggioranza { posizioni: senza });
    }
    r.avvisi.extend(di_stato);
    r.posizioni = posizioni;
    r
}

/// Gli avvisi sullo stato del paese al turno zero, che non dipendono dalla posizione: le
/// situazioni già accese e lo spread già alla soglia di `crisi_del_debito`.
fn di_stato(dati: &DatiGioco, p: &Partita) -> Vec<Avviso> {
    let mut avvisi = Vec::new();
    let nomi: Vec<String> = (p.situazioni().into_iter())
        .filter(|s| s.accesa)
        .map(|s| s.nome)
        .collect();
    if !nomi.is_empty() {
        avvisi.push(Avviso::SituazioniAccese { nomi });
    }
    let soglia = dati.manopola("soglia_crisi_debito");
    if p.economia().spread >= soglia {
        avvisi.push(Avviso::CrisiDelDebito {
            spread_pb: p.economia().spread.round() as i32,
            soglia_pb: soglia.round() as i32,
        });
    }
    avvisi
}

/// I conti del turno zero, senza posizioni né avvisi. Non dipendono dal partito fondato:
/// `Partita::costruisci` li prende da `Economia::iniziale` e da `Disposizioni::iniziali` con le
/// leggi dello scenario, che non leggono i partiti.
fn conti(dati: &DatiGioco, p: &Partita) -> RiepilogoScenario {
    let e = p.economia();
    let sul_pil = |euro: i64| 100.0 * euro as f64 / (e.pil_trimestrale * 1e9);
    RiepilogoScenario {
        spread_pb: e.spread.round() as i32,
        interessi_pil: sul_pil(e.conti.spese.interessi),
        disavanzo_pil: sul_pil(e.disavanzo_trimestre),
        saldo_disposizioni_pil: p.disposizioni().saldo(dati),
        posizioni: Vec::new(),
        avvisi: Vec::new(),
    }
}

/// Il blocco migliore col giocatore `g`: al più `BLOCCO` partiti contigui sull'asse (posizione
/// dichiarata, a parità l'indice), quello con la quota più alta; a parità il primo per inizio,
/// poi per fine. Gli indici in ordine.
#[expect(
    clippy::expect_used,
    reason = "`g` è il giocatore della partita, quindi un indice di `partiti`"
)]
fn blocco(partiti: &[Partito], quote: &[f64], g: usize) -> Vec<usize> {
    let mut fila: Vec<usize> = (0..partiti.len()).collect();
    fila.sort_by(|&a, &b| {
        (partiti[a].posizione_dichiarata)
            .total_cmp(&partiti[b].posizione_dichiarata)
            .then(a.cmp(&b))
    });
    let at = (fila.iter().position(|&j| j == g)).expect("il giocatore è sempre fra i partiti");
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

fn attesa(dati: &DatiGioco, p: &Partita, posizione: f64) -> PosizioneAttesa {
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
                    prima della campagna.";
        assert_eq!(a(vec![1.5]), format!("Da 1,5, {coda}"));
        assert_eq!(a(vec![1.5, 6.5]), format!("Da 1,5 e 6,5, {coda}"));
        assert_eq!(
            a(vec![1.5, 2.5, 6.5]),
            "Da 1,5; 2,5 e 6,5, nessuna coalizione con te ha la maggioranza attesa in entrambe le \
             Camere, prima della campagna."
        );
    }

    #[test]
    fn i_testi_degli_avvisi_di_stato() {
        let coda = "Pesano finché non tornano sotto la soglia di spegnimento.";
        let a = |nomi: &[&str]| {
            let nomi = nomi.iter().map(|n| n.to_string()).collect();
            Avviso::SituazioniAccese { nomi }.testo()
        };
        assert_eq!(
            a(&["Caro energia"]),
            format!("Già accese al turno zero: Caro energia. {coda}")
        );
        assert_eq!(
            a(&["Caro energia", "Criminalità"]),
            format!("Già accese al turno zero: Caro energia e Criminalità. {coda}")
        );
        assert_eq!(
            Avviso::CrisiDelDebito {
                spread_pb: 585,
                soglia_pb: 550
            }
            .testo(),
            "Spread a 585 punti base, oltre la soglia di 550: alla prima chiusura le aste vanno \
             scoperte e dovrai scegliere fra MES e ristrutturazione."
        );
    }
}
