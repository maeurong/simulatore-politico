//! Il riepilogo dal vivo dello scenario: i conti del turno zero e, per ogni posizione della
//! fondazione, consenso e maggioranza attesi senza rumore, prima della campagna.

use crate::dati::DatiGioco;
use crate::disposizioni::Norma;
use crate::elettorato::Intenzioni;
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::legislatura::{Dichiarazione, SEGNAPOSTO, maggioranza, partito_nuovo, posizioni};
use crate::partita::{Partita, quote_fra_i_votanti};
use crate::partiti::Partito;
use crate::scenario::Ingresso;
use crate::su_misura::SuMisura;
use serde::Serialize;
use std::collections::BTreeMap;
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
    /// Lo spread del turno zero è alla soglia della crisi del debito: è lo stato del turno zero,
    /// la prima chiusura ricalcola lo spread (e col ministro vero).
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
                    &posizioni
                        .iter()
                        .map(|x| x.to_string().replace('.', ","))
                        .collect::<Vec<_>>(),
                    "; "
                )
            ),
            Avviso::SituazioniAccese { nomi } => format!(
                "Già accese al turno zero: {}. Pesano finché non rientrano.",
                elenco(nomi, ", ")
            ),
            Avviso::CrisiDelDebito {
                spread_pb,
                soglia_pb,
            } => format!(
                "Spread a {spread_pb} punti base, alla soglia della crisi del debito ({soglia_pb}): \
                 se ci resta alla prima chiusura si apre «Aste scoperte», con la scelta fra MES e \
                 ristrutturazione."
            ),
        }
    }
}

/// «A», «A e B», «A, B e C». Il separatore è «; » solo per i numeri, che hanno la virgola
/// decimale: «1,5; 2,5 e 3,5».
fn elenco(voci: &[String], separatore: &str) -> String {
    match voci.split_last() {
        Some((ultima, prima)) if !prima.is_empty() => {
            format!("{} e {ultima}", prima.join(separatore))
        }
        _ => voci.concat(),
    }
}

/// La condizione di `Partita::crisi_del_debito` e dell'avviso: lo spread alla manopola
/// `soglia_crisi_debito`. Una sola, così le due non divergono.
pub(crate) fn alla_soglia_della_crisi(soglia: f64, spread: f64) -> bool {
    spread >= soglia
}

/// Ciò che l'editor deve sapere prima di ogni riepilogo: intervalli dei sei cursori, leggi
/// elettorali, disposizioni coi gradini, partiti dello scenario.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct CatalogoScenario {
    /// I sei cursori, in quest'ordine: debito_pil, disoccupazione, inflazione, ricchezza,
    /// eta_mediana, orientamento.
    pub cursori: Vec<CursoreCatalogo>,
    /// Nome dell'impostazione di elezioni.toml -> (id della disposizione elettorale -> id del
    /// gradino).
    pub leggi_elettorali: BTreeMap<String, BTreeMap<String, String>>,
    pub legge_di_base: String,
    /// Tutte le disposizioni dei dati, nel loro ordine.
    pub disposizioni: Vec<VoceCatalogo>,
    /// I partiti dello scenario, nel loro ordine: la fondazione li mette sotto l'asse.
    pub partiti: Vec<PartitoCatalogo>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct CursoreCatalogo {
    pub nome: String,
    pub valore: f64,
    pub min: f64,
    pub max: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceCatalogo {
    pub id: String,
    pub nome: String,
    /// Nome del dicastero; None per costituzionali ed elettorali.
    pub dicastero: Option<String>,
    pub norma: Norma,
    pub partenza: String,
    pub gradini: Vec<GradinoCatalogo>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct GradinoCatalogo {
    pub id: String,
    pub nome: String,
    pub saldo: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PartitoCatalogo {
    pub id: String,
    pub nome: String,
    pub posizione: f64,
}

/// Il catalogo dei dati, di sola lettura. I gradini elettorali di ogni legge sono quelli che
/// `con_su_misura` ne fa; il suo errore si propaga.
///
/// # Errors
/// Se `con_su_misura` rifiuta una delle leggi elettorali dei dati.
pub fn catalogo(dati: &DatiGioco) -> Result<CatalogoScenario, Errore> {
    let sc = dati.scenario();
    let c = &sc.cursori;
    let cursore = |nome: &str, i: &Ingresso| CursoreCatalogo {
        nome: nome.into(),
        valore: i.valore,
        min: i.min,
        max: i.max,
    };
    let base = SuMisura::base(dati);
    let mut leggi_elettorali = BTreeMap::new();
    for nome in dati.elezioni().impostazioni.keys() {
        let mut s = base.clone();
        s.cursori.legge_elettorale.clone_from(nome);
        let gradini = (dati.con_su_misura(&s)?.disposizioni().iter())
            .filter(|x| x.norma == Norma::Elettorale)
            .map(|x| (x.id.clone(), x.partenza.clone()))
            .collect();
        leggi_elettorali.insert(nome.clone(), gradini);
    }
    Ok(CatalogoScenario {
        cursori: vec![
            cursore("debito_pil", &sc.debito_pil),
            cursore("disoccupazione", &sc.disoccupazione),
            cursore("inflazione", &sc.inflazione),
            cursore("ricchezza", &c.ricchezza),
            cursore("eta_mediana", &c.eta_mediana),
            cursore("orientamento", &c.orientamento),
        ],
        leggi_elettorali,
        legge_di_base: sc.legge_elettorale.clone(),
        disposizioni: (dati.disposizioni().iter())
            .map(|x| VoceCatalogo {
                id: x.id.clone(),
                nome: x.nome.clone(),
                dicastero: (x.dicastero.as_ref())
                    .and_then(|i| dati.dicasteri().iter().find(|d| &d.id == i))
                    .map(|d| d.nome.clone()),
                norma: x.norma,
                partenza: x.partenza.clone(),
                gradini: (x.gradini.iter())
                    .map(|g| GradinoCatalogo {
                        id: g.id.clone(),
                        nome: g.nome.clone(),
                        saldo: g.saldo,
                    })
                    .collect(),
            })
            .collect(),
        partiti: (sc.partiti.iter())
            .map(|p| PartitoCatalogo {
                id: p.id.clone(),
                nome: p.nome.clone(),
                posizione: p.posizione,
            })
            .collect(),
    })
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
/// situazioni già accese e lo spread alla manopola `soglia_crisi_debito`. Come i conti, non
/// leggono il partito fondato.
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
    if alla_soglia_della_crisi(soglia, p.economia().spread) {
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
        let coda = "Pesano finché non rientrano.";
        let a = |nomi: &[&str]| {
            let nomi = nomi.iter().map(|n| n.to_string()).collect();
            Avviso::SituazioniAccese { nomi }.testo()
        };
        assert_eq!(
            a(&["Caro energia"]),
            format!("Già accese al turno zero: Caro energia. {coda}")
        );
        assert_eq!(
            a(&["Criminalità", "Caro energia"]),
            format!("Già accese al turno zero: Criminalità e Caro energia. {coda}")
        );
        assert_eq!(
            a(&["Criminalità", "Liste d'attesa", "Caro energia"]),
            format!("Già accese al turno zero: Criminalità, Liste d'attesa e Caro energia. {coda}")
        );
        assert_eq!(
            Avviso::CrisiDelDebito {
                spread_pb: 586,
                soglia_pb: 550
            }
            .testo(),
            "Spread a 586 punti base, alla soglia della crisi del debito (550): se ci resta alla \
             prima chiusura si apre «Aste scoperte», con la scelta fra MES e ristrutturazione."
        );
    }

    #[test]
    fn elenco_con_tre_voci() {
        let v = |xs: &[&str]| xs.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(elenco(&v(&["A", "B", "C"]), ", "), "A, B e C");
        assert_eq!(elenco(&v(&["1,5", "2,5", "3,5"]), "; "), "1,5; 2,5 e 3,5");
        assert_eq!(elenco(&v(&["A", "B"]), ", "), "A e B");
        assert_eq!(elenco(&v(&["A"]), ", "), "A");
    }

    #[test]
    fn la_soglia_della_crisi_e_inclusiva() {
        assert!(alla_soglia_della_crisi(550.0, 550.0));
        assert!(alla_soglia_della_crisi(550.0, 551.0));
        assert!(!alla_soglia_della_crisi(550.0, 549.999));
    }
}
