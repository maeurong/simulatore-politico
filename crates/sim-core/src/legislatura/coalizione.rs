//! La coalizione pre-elettorale: dichiarazione, coalizioni al voto e disponibilità dei partiti
//! (formazione-di-coalizioni:34, :48; premio alle liste della coalizione, sistemi-elettorali:39).

use super::fondazione::quota_fra_i_votanti;
use super::spartizione::maggioranza;
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

/// Seggi che il giocatore cede, dopo il riparto, a `partito` (id) della sua coalizione: `seggi`
/// alla Camera e la metà, per difetto, al Senato.
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

/// I cinque termini del valore atteso di una coalizione (`Quadro::termini`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CausaDisponibilita {
    Seggi,
    Governare,
    Distanza,
    Guida,
    Intesa,
}

impl CausaDisponibilita {
    /// La causa come frammento in minuscolo, da comporre in una frase; `con_te` = il termine
    /// spinge verso la coalizione del giocatore.
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

/// La disponibilità di un partito IA (id) a entrare nella coalizione del giocatore: sì o no, con
/// la causa. I valori attesi restano interni.
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

/// `n` seggi dal giocatore `da` ad `a`: se ne aveva, gliene resta almeno uno.
fn passa(seggi: &mut [u32], da: usize, a: usize, n: u32) {
    if da < seggi.len() && a < seggi.len() {
        let n = n.min(seggi[da].saturating_sub(1));
        seggi[da] -= n;
        seggi[a] += n;
    }
}

/// Le cessioni dopo il riparto (scelta 5 del piano 7): `n` seggi della Camera e `n / 2` del
/// Senato dal giocatore al membro; in ogni ramo dove aveva seggi gliene resta almeno uno.
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
#[derive(Clone)]
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
    /// Dopo il voto, i seggi veri di Camera e Senato: valgono per ogni coalizione al posto del
    /// riparto sul sondaggio, e `P` vale 1 con la maggioranza in entrambe le Camere, 0 senza.
    pub seggi_veri: Option<(Vec<u32>, Vec<u32>)>,
}

impl Quadro<'_> {
    /// Seggi di Camera e Senato per partito al voto con `d`: riparto sul sondaggio pubblico, poi
    /// le cessioni; zero se il riparto rifiuta i voti.
    pub(super) fn seggi(&self, d: &Dichiarazione) -> (Vec<u32>, Vec<u32>) {
        if let Some(veri) = &self.seggi_veri {
            return veri.clone();
        }
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

    /// I termini, già pesati e con la loro causa, del valore atteso per `j` al voto con `d` e i
    /// `seggi` di `d`, nella sua coalizione `k`: `peso_seggi_coalizione · s_j / S`,
    /// `peso_governare · P_k` (coi seggi veri, 1 o 0), `− peso_distanza_coalizione · d_jk`, `valore_guida · [j guida k]`,
    /// `− peso_intesa_coalizione · max(0, intesa_base − intesa_j) / 100 · [j alleato in carica e
    /// k del giocatore]`.
    pub(super) fn termini(
        &self,
        d: &Dichiarazione,
        (camera, senato): &(Vec<u32>, Vec<u32>),
        j: usize,
    ) -> [(CausaDisponibilita, f64); 5] {
        let m = |nome: &str| self.dati.manopola(nome);
        let k = (coalizioni_al_voto(self.ultime, self.giocatore, Some(d)).into_iter())
            .find(|k| k.contains(&j))
            .unwrap_or_else(|| vec![j]);
        let del_giocatore = k.contains(&self.giocatore);
        let di = |v: &[u32], i: usize| f64::from(v.get(i).copied().unwrap_or(0));
        let totale = |v: &[u32]| v.iter().sum::<u32>();
        let s = f64::from(totale(camera).max(1)) + f64::from(totale(senato).max(1));
        let quota = (di(camera, j) + di(senato, j)) / s;
        // Margine sulla maggioranza del ramo (metà più uno), in quota dei seggi del ramo.
        let margine = |v: &[u32]| {
            let t = totale(v);
            let seggi_k: f64 = k.iter().map(|&i| di(v, i)).sum();
            (seggi_k - f64::from(t / 2 + 1)) / f64::from(t.max(1))
        };
        let x = margine(camera).min(margine(senato)) / m("scala_maggioranza");
        let governare = match self.seggi_veri {
            Some(_) if maggioranza(camera, senato, &k) => 1.0,
            Some(_) => 0.0,
            None => 1.0 / (1.0 + (-x).exp()),
        };
        let distanza = distanza(self.partiti, &k, j);
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
        use CausaDisponibilita as C;
        [
            (C::Seggi, m("peso_seggi_coalizione") * quota),
            (C::Governare, m("peso_governare") * governare),
            (C::Distanza, -m("peso_distanza_coalizione") * distanza),
            (C::Guida, if guida { m("valore_guida") } else { 0.0 }),
            (C::Intesa, -m("peso_intesa_coalizione") * intesa),
        ]
    }

    /// Una voce per partito IA: `voci` su tutti.
    pub(crate) fn disponibilita(&self, d: &Dichiarazione) -> Vec<VoceDisponibilita> {
        self.voci(d, (0..self.partiti.len()).filter(|&j| j != self.giocatore))
    }

    /// Una voce per partito di `quali`: disponibile se il valore atteso nella coalizione del
    /// giocatore con lui aggiunto (e le cessioni di `d`) supera quello nella disposizione senza di
    /// lui; la causa è il termine con lo scarto più grande nel verso che decide, `Distanza` se
    /// nessuno scarta in quel verso.
    pub(crate) fn voci(
        &self,
        d: &Dichiarazione,
        quali: impl Iterator<Item = usize>,
    ) -> Vec<VoceDisponibilita> {
        // ponytail: un riparto per partito oltre a quello di `d`, senza cache; si memorizza se il
        // profilo lo chiede.
        let di_d = self.seggi(d);
        let seggi = |x: &Dichiarazione| if x == d { di_d.clone() } else { self.seggi(x) };
        quali
            .map(|j| {
                let mut con = d.clone();
                if !con.partiti.contains(&j) {
                    con.partiti.push(j);
                }
                let senza = Dichiarazione {
                    partiti: d.partiti.iter().copied().filter(|&i| i != j).collect(),
                    cessioni: d.cessioni.iter().copied().filter(|c| c.0 != j).collect(),
                };
                let (disponibile, causa) = confronta(
                    &self.termini(&con, &seggi(&con), j),
                    &self.termini(&senza, &seggi(&senza), j),
                );
                VoceDisponibilita {
                    partito: self
                        .partiti
                        .get(j)
                        .map_or_else(String::new, |p| p.id.clone()),
                    disponibile,
                    causa: causa.testo(disponibile).into(),
                }
            })
            .collect()
    }

    /// Il valore atteso di `j` al voto con `d`: la somma dei suoi termini.
    pub(crate) fn valore(&self, d: &Dichiarazione, j: usize) -> f64 {
        somma(&self.termini(d, &self.seggi(d), j))
    }

    /// `j` cede la guida al giocatore se il suo valore con `d` su questo quadro supera quello con
    /// `d_prima` sul quadro `prima`; altrimenti la causa, come in `voci`.
    pub(crate) fn cede_la_guida(
        &self,
        d: &Dichiarazione,
        prima: &Quadro,
        d_prima: &Dichiarazione,
        j: usize,
    ) -> Result<(), CausaDisponibilita> {
        let (cede, causa) = confronta(
            &self.termini(d, &self.seggi(d), j),
            &prima.termini(d_prima, &prima.seggi(d_prima), j),
        );
        if cede { Ok(()) } else { Err(causa) }
    }
}

fn somma(t: &[(CausaDisponibilita, f64)]) -> f64 {
    t.iter().map(|x| x.1).sum()
}

/// `a` vince se il suo valore supera quello di `b`; la causa è il termine con lo scarto più grande
/// nel verso che decide, `Distanza` se nessuno scarta in quel verso.
fn confronta(
    a: &[(CausaDisponibilita, f64)],
    b: &[(CausaDisponibilita, f64)],
) -> (bool, CausaDisponibilita) {
    let vince = somma(a) > somma(b);
    let verso = if vince { 1.0 } else { -1.0 };
    let causa = (a.iter().zip(b))
        .map(|(&(c, x), &(_, y))| (c, verso * (x - y)))
        .filter(|&(_, scarto)| scarto > 0.0)
        .max_by(|x, y| x.1.total_cmp(&y.1))
        .map_or(CausaDisponibilita::Distanza, |(c, _)| c);
    (vince, causa)
}

/// La distanza media fra la posizione dichiarata di `j` e quelle degli altri membri di `k`; 0
/// se non ce ne sono.
pub(super) fn distanza(partiti: &[Partito], k: &[usize], j: usize) -> f64 {
    let pos = |i: usize| partiti.get(i).map_or(0.0, |p| p.posizione_dichiarata);
    let altri: Vec<f64> = (k.iter().filter(|&&i| i != j))
        .map(|&i| (pos(i) - pos(j)).abs())
        .collect();
    if altri.is_empty() {
        0.0
    } else {
        altri.iter().sum::<f64>() / altri.len() as f64
    }
}

/// La dichiarazione del comando `DichiaraCoalizione`, validata su `q`.
///
/// # Errors
/// `Comando` col nome del partito in causa (l'id se sconosciuto): sconosciuto, ripetuto, il
/// giocatore stesso, cessione a un partito fuori dalla coalizione o ripetuta, cessioni pari o
/// oltre i seggi attesi del giocatore in un ramo, ogni membro non disponibile con la coalizione
/// intera.
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
            return ko(format!(
                "non puoi inserire il tuo partito ({}) fra i membri: la coalizione la guidi tu",
                nome(j)
            ));
        }
        if d.partiti.contains(&j) {
            return ko(format!("{} ripetuto nella coalizione", nome(j)));
        }
        d.partiti.push(j);
    }
    for c in cessioni {
        let j = indice(&c.partito)?;
        if !d.partiti.contains(&j) {
            return ko(format!(
                "{} non è nella coalizione: non puoi cedergli seggi",
                nome(j)
            ));
        }
        if d.cessioni.iter().any(|c| c.0 == j) {
            return ko(format!("Cessione ripetuta a {}", nome(j)));
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
    // Pari ai seggi attesi è già troppo: il giocatore resterebbe senza seggi nel ramo.
    if (camera > 0 && camera >= ha_camera) || (senato > 0 && senato >= ha_senato) {
        return ko(format!(
            "Cessioni pari o oltre i seggi attesi di {}: {camera} seggi ceduti alla Camera su \
             {ha_camera} attesi, {senato} al Senato su {ha_senato} (metà dei ceduti alla Camera)",
            nome(g)
        ));
    }
    let fuori: Vec<String> = (d.partiti.iter().zip(q.voci(&d, d.partiti.iter().copied())))
        .filter(|(_, v)| !v.disponibile)
        .map(|(&j, v)| format!("{} non entra: {}", nome(j), v.causa))
        .collect();
    if !fuori.is_empty() {
        return ko(fuori.join("; "));
    }
    Ok(d)
}

/// Chi della `coalizione` di governo rompe l'impegno pre-elettorale, in ordine d'indice. Rompe se
/// sta fra gli `entranti`, ha corso in una coalizione `C` di almeno due partiti di `ultime` e un
/// compagno di `C` resta fuori dalla `coalizione`; o se un altro entrante ha corso in una
/// coalizione di almeno due partiti diversa dalla sua, anche se lui ha corso da solo
/// (formazione-di-coalizioni:48: cambiare campo costa). Governare con chi ha corso da solo non
/// fa rompere nessuno.
pub(crate) fn rotture(
    ultime: &[Vec<usize>],
    coalizione: &[usize],
    entranti: &[usize],
) -> Vec<usize> {
    let campo = |j: usize| (ultime.iter()).find(|c| c.len() >= 2 && c.contains(&j));
    let mut v: Vec<usize> = (coalizione.iter().copied())
        .filter(|&j| {
            let c = campo(j);
            let lascia = entranti.contains(&j)
                && c.is_some_and(|c| c.iter().any(|k| !coalizione.contains(k)));
            lascia || (entranti.iter()).any(|&k| k != j && campo(k).is_some_and(|ck| Some(ck) != c))
        })
        .collect();
    v.sort_unstable();
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cedi_non_oltre_i_seggi_del_giocatore() {
        let (mut camera, mut senato) = (vec![5, 0], vec![2, 0]);
        cedi(&mut camera, &mut senato, 0, &[(1, 9)]);
        assert_eq!((camera, senato), (vec![1, 4], vec![1, 1]));
        let (mut camera, mut senato) = (vec![5, 0], vec![2, 0]);
        cedi(&mut camera, &mut senato, 0, &[(9, 3)]);
        assert_eq!((camera, senato), (vec![5, 0], vec![2, 0]));
    }

    #[test]
    fn coalizioni_al_voto_senza_dichiarazione() {
        let ultime = [vec![0, 1], vec![2, 3, 7]];
        assert_eq!(
            coalizioni_al_voto(&ultime, 7, None),
            vec![vec![2, 3, 7], vec![0, 1]]
        );
        assert_eq!(
            coalizioni_al_voto(&ultime, 5, None),
            vec![vec![5], vec![0, 1], vec![2, 3, 7]]
        );
    }

    #[test]
    fn coalizione_rimasta_di_un_partito_scartata() {
        let d = Dichiarazione {
            partiti: vec![3],
            cessioni: Vec::new(),
        };
        assert_eq!(
            coalizioni_al_voto(&[vec![2, 3]], 0, Some(&d)),
            vec![vec![0, 3]]
        );
    }

    #[test]
    fn rompe_chi_lascia_un_compagno_o_governa_con_l_altro_campo() {
        let ultime = [vec![7], vec![1, 2], vec![4, 5]];
        // 2 lascia 1; 7, che ha corso da solo, governa con un entrante dell'altro campo.
        assert_eq!(rotture(&ultime, &[2, 7], &[2, 7]), vec![2, 7]);
        // 3, che ha corso da solo, governa con la coalizione rivale 4-5; 4 e 5 restano insieme
        // e governano con chi ha corso da solo: liberi.
        assert_eq!(
            rotture(&[vec![4, 5], vec![3]], &[3, 4, 5], &[3, 4, 5]),
            vec![3]
        );
        // 5 governa senza il suo compagno 4; 6 governa con un entrante della coalizione 4-5.
        assert_eq!(rotture(&[vec![4, 5]], &[5, 6], &[5, 6]), vec![5, 6]);
        // Un entrante che aveva corso da solo non fa rompere chi governa già.
        assert_eq!(
            rotture(&[vec![4, 5], vec![6]], &[5, 6], &[6]),
            Vec::<usize>::new()
        );
    }
}
