//! La coalizione pre-elettorale: dichiarazione, coalizioni al voto e disponibilità dei partiti
//! (formazione-di-coalizioni:34, :48; premio alle liste della coalizione, sistemi-elettorali:39).

use super::fondazione::quota_fra_i_votanti;
use crate::dati::DatiGioco;
use crate::disposizioni::Leve;
use crate::elezioni::{LeggeElettorale, SeggiLista};
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::parlamento::proietta;
use crate::partiti::Partito;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// `seggi` della Camera, e la metà per difetto al Senato, che il giocatore cede dopo il riparto
/// al partito `partito` (id) della sua coalizione.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Cessione {
    pub partito: String,
    pub seggi: u32,
}

/// La coalizione dichiarata: `partiti` = i membri senza il giocatore, `cessioni` = (membro, seggi
/// della Camera ceduti).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dichiarazione {
    pub partiti: Vec<usize>,
    pub cessioni: Vec<(usize, u32)>,
}

/// I cinque termini del valore atteso di una coalizione, nell'ordine di `Quadro::termini`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CausaDisponibilita {
    Seggi,
    Governare,
    Distanza,
    Guida,
    Intesa,
}

impl CausaDisponibilita {
    const TUTTE: [Self; 5] = [
        Self::Seggi,
        Self::Governare,
        Self::Distanza,
        Self::Guida,
        Self::Intesa,
    ];

    /// La causa in una riga; `con_te` = il termine spinge verso la coalizione del giocatore.
    pub fn testo(self, con_te: bool) -> &'static str {
        match (self, con_te) {
            (Self::Seggi, false) => "più seggi da solo",
            (Self::Seggi, true) => "più seggi con te",
            (Self::Governare, false) => "più probabilità di governare altrove",
            (Self::Governare, true) => "più probabilità di governare con te",
            (Self::Distanza, false) => "troppo lontano dalla tua posizione",
            (Self::Distanza, true) => "vicino alla tua posizione",
            (Self::Guida, _) => "guiderebbe la propria coalizione",
            (Self::Intesa, _) => "intesa bassa con te",
        }
    }
}

/// La disponibilità di un partito IA (id) a entrare nella coalizione del giocatore: la regola e
/// la causa, mai i valori attesi.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDisponibilita {
    pub partito: String,
    pub disponibile: bool,
    pub causa: String,
}

/// Le coalizioni al voto: prima quella del giocatore (lui più i membri di `d`; senza
/// dichiarazione la sua dell'ultimo voto, o lui da solo), poi quelle di `ultime` senza i suoi
/// membri, tolte quelle rimaste di un partito solo (scelta 4 del piano 7).
pub(crate) fn coalizioni_al_voto(
    ultime: &[Vec<usize>],
    giocatore: usize,
    d: Option<&Dichiarazione>,
) -> Vec<Vec<usize>> {
    let sua: Vec<usize> = match d {
        Some(d) => std::iter::once(giocatore)
            .chain(d.partiti.iter().copied())
            .collect(),
        None => (ultime.iter().find(|c| c.contains(&giocatore)))
            .cloned()
            .unwrap_or_else(|| vec![giocatore]),
    };
    let altre: Vec<Vec<usize>> = (ultime.iter())
        .map(|c| c.iter().copied().filter(|j| !sua.contains(j)).collect())
        .filter(|c: &Vec<usize>| c.len() >= 2)
        .collect();
    let mut tutte = vec![sua];
    tutte.extend(altre);
    tutte
}

/// `n` seggi da `da` ad `a`, mai oltre quelli di `da`.
fn passa(seggi: &mut [u32], da: usize, a: usize, n: u32) {
    if da < seggi.len() && a < seggi.len() {
        let n = n.min(seggi[da]);
        seggi[da] -= n;
        seggi[a] += n;
    }
}

/// Le cessioni dopo il riparto (scelta 5 del piano 7): `n` seggi della Camera e `n / 2` del
/// Senato dal giocatore al membro, mai oltre i seggi del giocatore.
pub(crate) fn cedi(
    camera: &mut [u32],
    senato: &mut [u32],
    giocatore: usize,
    cessioni: &[(usize, u32)],
) {
    for &(j, n) in cessioni {
        passa(camera, giocatore, j, n);
        passa(senato, giocatore, j, n / 2);
    }
}

/// Ciò che la disponibilità legge della partita: sondaggio pubblico, legge e seggi dei rami in
/// vigore, coalizioni dell'ultimo voto.
pub(crate) struct Quadro<'a> {
    pub dati: &'a DatiGioco,
    pub partiti: &'a [Partito],
    pub sondaggio: &'a Sondaggio,
    pub legge: LeggeElettorale,
    pub leve: Leve,
    pub ultime: &'a [Vec<usize>],
    pub giocatore: usize,
    pub intesa: &'a [f64],
    /// Gli alleati del giocatore in carica; vuoto se non è al governo.
    pub alleati: Vec<usize>,
}

impl Quadro<'_> {
    /// Seggi di Camera e Senato per partito al voto con `d`: riparto sul sondaggio pubblico, poi
    /// le cessioni; zero se il riparto rifiuta i voti.
    fn seggi(&self, d: &Dichiarazione) -> (Vec<u32>, Vec<u32>) {
        let n = self.partiti.len();
        let c = coalizioni_al_voto(self.ultime, self.giocatore, Some(d));
        let esito = proietta(
            self.dati,
            self.partiti,
            self.sondaggio,
            &self.legge,
            &self.leve,
            &c,
        );
        let Ok(e) = esito else {
            return (vec![0; n], vec![0; n]);
        };
        let totali = |v: &[SeggiLista]| v.iter().map(SeggiLista::totale).collect::<Vec<u32>>();
        let (mut camera, mut senato) = (totali(&e.camera), totali(&e.senato));
        cedi(&mut camera, &mut senato, self.giocatore, &d.cessioni);
        (camera, senato)
    }

    /// I termini, già pesati, del valore atteso per `j` al voto con `d`, nella sua coalizione `k`,
    /// nell'ordine di `CausaDisponibilita`: `peso_seggi_coalizione · s_j / S`, `peso_governare ·
    /// P_k`, `− peso_distanza_coalizione · d_jk`, `valore_guida · [j guida k]`,
    /// `− peso_intesa_coalizione · max(0, intesa_base − intesa_j) / 100 · [j alleato in carica e
    /// k del giocatore]`.
    fn termini(&self, d: &Dichiarazione, j: usize) -> [f64; 5] {
        let m = |nome: &str| self.dati.manopola(nome);
        let (camera, senato) = self.seggi(d);
        let k = (coalizioni_al_voto(self.ultime, self.giocatore, Some(d)).into_iter())
            .find(|k| k.contains(&j))
            .unwrap_or_else(|| vec![j]);
        let del_giocatore = k.contains(&self.giocatore);
        let di = |v: &[u32], i: usize| f64::from(v.get(i).copied().unwrap_or(0));
        let totale = |v: &[u32]| v.iter().sum::<u32>();
        let s = f64::from(totale(&camera).max(1)) + f64::from(totale(&senato).max(1));
        let quota = (di(&camera, j) + di(&senato, j)) / s;
        // Margine sulla maggioranza del ramo (metà più uno), in quota dei seggi del ramo.
        let margine = |v: &[u32]| {
            let t = totale(v);
            let seggi_k: f64 = k.iter().map(|&i| di(v, i)).sum();
            (seggi_k - f64::from(t / 2 + 1)) / f64::from(t.max(1))
        };
        let x = margine(&camera).min(margine(&senato)) / m("scala_maggioranza");
        let governare = 1.0 / (1.0 + (-x).exp());
        let pos = |i: usize| self.partiti.get(i).map_or(0.0, |p| p.posizione_dichiarata);
        let altri: Vec<f64> = (k.iter().filter(|&&i| i != j))
            .map(|&i| (pos(i) - pos(j)).abs())
            .collect();
        let distanza = if altri.is_empty() {
            0.0
        } else {
            altri.iter().sum::<f64>() / altri.len() as f64
        };
        // Nella coalizione del giocatore guida sempre il giocatore; chi corre da solo non guida
        // una coalizione (A9 del piano 2: coalizione di una lista sola = lista non coalizzata).
        let q = |i: &&usize| quota_fra_i_votanti(self.sondaggio, **i);
        let guida = !del_giocatore
            && k.len() >= 2
            && k.iter().max_by(|a, b| q(a).total_cmp(&q(b))) == Some(&j);
        let intesa = if del_giocatore && self.alleati.contains(&j) {
            let x = self.intesa.get(j).copied().unwrap_or(0.0);
            (m("intesa_base") - x).max(0.0) / 100.0
        } else {
            0.0
        };
        [
            m("peso_seggi_coalizione") * quota,
            m("peso_governare") * governare,
            -m("peso_distanza_coalizione") * distanza,
            if guida { m("valore_guida") } else { 0.0 },
            -m("peso_intesa_coalizione") * intesa,
        ]
    }

    /// Una voce per partito IA: disponibile se il valore atteso nella coalizione del giocatore con
    /// lui aggiunto (e le cessioni di `d`) supera quello nella disposizione senza di lui; la causa
    /// è il termine con lo scarto più grande nel verso che decide.
    pub(crate) fn disponibilita(&self, d: &Dichiarazione) -> Vec<VoceDisponibilita> {
        // ponytail: due riparti per partito IA, senza cache; si memorizza se il profilo lo chiede.
        (self.partiti.iter().enumerate())
            .filter(|&(j, _)| j != self.giocatore)
            .map(|(j, p)| {
                let mut con = d.clone();
                if !con.partiti.contains(&j) {
                    con.partiti.push(j);
                }
                let senza = Dichiarazione {
                    partiti: d.partiti.iter().copied().filter(|&i| i != j).collect(),
                    cessioni: d.cessioni.iter().copied().filter(|c| c.0 != j).collect(),
                };
                let (a, b) = (self.termini(&con, j), self.termini(&senza, j));
                let disponibile = a.iter().sum::<f64>() > b.iter().sum::<f64>();
                let verso = if disponibile { 1.0 } else { -1.0 };
                let causa = (a.iter().zip(&b).map(|(x, y)| verso * (x - y)))
                    .zip(CausaDisponibilita::TUTTE)
                    .max_by(|x, y| x.0.total_cmp(&y.0))
                    .map_or(CausaDisponibilita::Seggi, |(_, c)| c);
                VoceDisponibilita {
                    partito: p.id.clone(),
                    disponibile,
                    causa: causa.testo(disponibile).into(),
                }
            })
            .collect()
    }
}

/// La dichiarazione del comando `DichiaraCoalizione`, validata su `q`.
///
/// # Errors
/// `Comando` col nome del partito in causa: sconosciuto, ripetuto, il giocatore stesso, cessione a
/// un partito fuori dalla coalizione, cessioni oltre i seggi attesi del giocatore, ogni membro non
/// disponibile con la coalizione intera.
pub(crate) fn dichiarazione(
    q: &Quadro,
    partiti: &[String],
    cessioni: &[Cessione],
) -> Result<Dichiarazione, Errore> {
    let ko = |m: String| Err(Errore::Comando(m));
    let indice = |id: &str| {
        (q.partiti.iter().position(|p| p.id == id))
            .ok_or_else(|| Errore::Comando(format!("Partito sconosciuto: {id}")))
    };
    let nome = |j: usize| q.partiti.get(j).map_or("", |p| p.nome.as_str());
    let mut d = Dichiarazione::default();
    for id in partiti {
        let j = indice(id)?;
        if j == q.giocatore {
            return ko(format!("{} è il tuo partito: guida la coalizione", nome(j)));
        }
        if d.partiti.contains(&j) {
            return ko(format!("{} ripetuto nella coalizione", nome(j)));
        }
        d.partiti.push(j);
    }
    for c in cessioni {
        let j = indice(&c.partito)?;
        if !d.partiti.contains(&j) {
            return ko(format!("Cessione a {}, fuori dalla coalizione", nome(j)));
        }
        d.cessioni.push((j, c.seggi));
    }
    let g = q.giocatore;
    let attesi = q.seggi(&Dichiarazione {
        partiti: d.partiti.clone(),
        cessioni: Vec::new(),
    });
    let somma = |f: fn(u32) -> u32| (d.cessioni.iter()).fold(0u32, |a, c| a.saturating_add(f(c.1)));
    let (camera, senato) = (somma(|n| n), somma(|n| n / 2));
    let (ha_camera, ha_senato) = (
        attesi.0.get(g).copied().unwrap_or(0),
        attesi.1.get(g).copied().unwrap_or(0),
    );
    if camera > ha_camera || senato > ha_senato {
        return ko(format!(
            "Cessioni oltre i seggi attesi di {}: {camera} alla Camera su {ha_camera}, {senato} al \
             Senato su {ha_senato}",
            nome(g)
        ));
    }
    let fuori: Vec<String> = (q.disponibilita(&d).into_iter())
        .filter(|v| !v.disponibile)
        .filter_map(|v| {
            let j = q.partiti.iter().position(|p| p.id == v.partito)?;
            d.partiti
                .contains(&j)
                .then(|| format!("{} non entra: {}", nome(j), v.causa))
        })
        .collect();
    if !fuori.is_empty() {
        return ko(fuori.join("; "));
    }
    Ok(d)
}
