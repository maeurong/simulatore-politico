//! Emergenze naturali per regione (piano 9): sisma, idrogeologica, incendi e siccità. Il rischio
//! viene dai dati della regione, il livello (lieve, grave, con vittime) da una estrazione, la
//! prevenzione dallo stock di infrastrutture. Fonti: `docs/ricerca/dati/pericolosita-naturale-regioni.md`.

use super::rischio::{Candidato, livello};
use super::{Causa, EventoAperto, Famiglia, Tono};
use crate::atti::Forma;
use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::economia::euro;
use crate::grandezze::{EffettoAttivo, Grandezze};
use crate::partita::Trimestre;

/// Gli eventi del catalogo che questo modulo fa nascere: sisma, idro, incendi, siccità.
pub(crate) const EVENTI: [&str; 4] = [
    "emergenza_sismica",
    "emergenza_idro",
    "emergenza_incendi",
    "emergenza_siccita",
];

/// Costo per livello: lieve, grave, con vittime.
const COSTI: [&str; 3] = [
    "costo_emergenza_lieve",
    "costo_emergenza_grave",
    "costo_emergenza_vittime",
];

/// Estrazione sotto la prima soglia = lieve, sotto la seconda = grave, oltre = con vittime.
// ponytail: assunzione del piano 9, fissa; una manopola se il bilanciamento la vuole.
const SOGLIE_LIVELLO: [f64; 2] = [0.7, 0.95];

/// Nel terzo trimestre cade l'87% dei grandi incendi: p = min(1, 3,5 × h)
/// (`pericolosita-naturale-regioni.md:124`).
const FATTORE_INCENDI: f64 = 3.5;

const MLD: f64 = 1e9;

/// Probabilità del trimestre `data` per il tipo `tipo` (indice in `EVENTI`) nella regione `r`.
fn probabilita(dati: &DatiGioco, data: Trimestre, tipo: usize, r: usize) -> f64 {
    let Some(h) = dati.regioni().get(r).map(|x| &x.pericolosita) else {
        return 0.0;
    };
    let p = match tipo {
        // Il minimo vale per ogni regione, Sardegna compresa (la ricerca la dà a 0, :122): i dati
        // non distinguono uno zero vero da uno zero «solo il minimo».
        0 => h.sisma.max(dati.manopola("sisma_minimo")),
        1 => 1.0 - (-dati.manopola("emergenze_idro_anno") * h.idro / 4.0).exp(),
        2 if data.trimestre == 3 => FATTORE_INCENDI * h.incendi,
        // La siccità: il rischio dell'anno cade sul secondo e sul terzo trimestre, uguale per regione.
        3 if matches!(data.trimestre, 2 | 3) => {
            let regioni = dati.regioni().len().max(1) as f64;
            1.0 - (-dati.manopola("siccita_anno") / 2.0 / regioni).exp()
        }
        _ => 0.0,
    };
    // Il clamp tiene `p` in 0-1 per `estrai`. La Sicilia a p = 1 nel terzo trimestre (3,5 × 0,35) è
    // voluta: è la formula della ricerca, `pericolosita-naturale-regioni.md:124`.
    p.clamp(0.0, 1.0)
}

/// Un candidato per regione e tipo (poi la siccità per ogni regione), sempre con la sua estrazione
/// del livello da `Flusso::Eventi`, anche a `p` zero: le estrazioni non dipendono dagli esiti.
/// `data` è il trimestre del turno che si apre.
pub(crate) fn candidati(
    dati: &DatiGioco,
    data: Trimestre,
    grandezze: &Grandezze,
    caso: &mut Caso,
) -> Vec<Candidato> {
    let Some(stock) = dati.grandezza("stock_infrastrutture") else {
        return Vec::new();
    };
    let Some(eventi) = EVENTI
        .iter()
        .map(|id| dati.evento(id))
        .collect::<Option<Vec<usize>>>()
    else {
        return Vec::new();
    };
    let g = &dati.grandezze()[stock];
    let n = dati.regioni().len();
    let ordine = (0..n)
        .flat_map(|r| (0..3).map(move |t| (t, r)))
        .chain((0..n).map(|r| (3, r)));
    ordine
        .map(|(tipo, r)| {
            let p = probabilita(dati, data, tipo, r);
            let u = caso.uniforme(Flusso::Eventi);
            let liv = SOGLIE_LIVELLO.iter().filter(|&&s| u >= s).count();
            // Lo stock iniziale è nessuna prevenzione, il massimo è prevenzione piena.
            let stock_r = (grandezze.valori.get(stock))
                .and_then(|v| v.get(r))
                .copied()
                .unwrap_or(g.iniziale);
            let prevenzione =
                ((stock_r - g.iniziale) / (g.max - g.iniziale).max(f64::EPSILON)).clamp(0.0, 1.0);
            let k = dati.manopola(if tipo == 0 {
                "k_prevenzione_sisma"
            } else {
                "k_prevenzione_altre"
            });
            let costo = euro(dati.manopola(COSTI[liv]) * (1.0 - k * prevenzione));
            let evento = eventi[tipo];
            let gravita = (dati.eventi().get(evento)).map_or(0, |e| e.gravita);
            Candidato {
                evento,
                p,
                // Prima il livello, a parità il tipo.
                gravita: (liv as u8).saturating_mul(10).saturating_add(gravita),
                regione: Some(r),
                bersaglio: None,
                causa: Causa::Rischio {
                    livello: livello(p),
                },
                costo,
                tono: if liv == 2 {
                    Tono::Grave
                } else {
                    Tono::Ordinario
                },
                lieve: liv == 0,
            }
        })
        .collect()
}

/// Il danno dell'emergenza sullo stock di infrastrutture della sua regione, moltiplicato per
/// `quota` (1 alla nascita, meno la quota pagata al ripristino, così i due effetti si elidono).
/// Un effetto d'impulso dal turno `inizio`, che dimezza ogni `emivita_danno` turni. `None` se
/// l'evento non è un'emergenza regionale o il danno è nullo.
pub(crate) fn danno(
    dati: &DatiGioco,
    aperto: &EventoAperto,
    inizio: u32,
    quota: f64,
) -> Option<EffettoAttivo> {
    let regione = aperto.regione?;
    let e = dati.eventi().get(aperto.evento)?;
    if e.famiglia != Famiglia::Emergenza {
        return None;
    }
    let grandezza = dati.grandezza("stock_infrastrutture")?;
    let intensita = -quota * aperto.costo as f64 / MLD * dati.manopola("danno_stock_per_miliardo");
    (intensita != 0.0).then_some(EffettoAttivo {
        grandezza,
        regione: Some(regione),
        dicastero: None,
        disposizione: None,
        intensita,
        inizio,
        forma: Forma::Impulso,
        durata: dati.manopola("emivita_danno"),
    })
}
