//! Eventi puntuali (emergenze, scandali, shock): una uniforme per candidato, sempre, e al massimo
//! un puntuale a turno, il più grave.

use super::{Bersaglio, Causa, Tono};
use crate::caso::{Caso, Flusso};
use crate::dati::DatiGioco;
use crate::grandezze::Grandezze;
use crate::ministri::Ministri;
use crate::partiti::Partito;
use crate::vista::Fascia3;

/// Gli eventi che nascono dallo scandalo: sul ministro, sul leader.
pub(crate) const SCANDALI: [&str; 2] = ["scandalo_ministro", "scandalo_leader"];

/// Un evento che può nascere in questo turno con probabilità `p`.
pub(crate) struct Candidato {
    pub evento: usize,
    pub p: f64,
    pub gravita: u8,
    pub regione: Option<usize>,
    pub bersaglio: Option<Bersaglio>,
    pub causa: Causa,
    pub costo: i64,
    pub tono: Tono,
    /// Un'emergenza lieve: non apre un evento, la paga il fondo alla nascita.
    pub lieve: bool,
}

/// Lo scandalo del turno (scelta 18): `p = scandalo_base × corruzione / 100`. Il bersaglio si estrae
/// sempre, anche a `p` zero o senza bersagli, fra i ministri in carica (in ordine di dicastero) e
/// i leader dei partiti IA non sciolti (in ordine d'indice): il giocatore non è mai fra loro.
pub(crate) fn candidato_scandalo(
    dati: &DatiGioco,
    grandezze: &Grandezze,
    ministri: &Ministri,
    partiti: &[Partito],
    giocatore: usize,
    caso: &mut Caso,
) -> Option<Candidato> {
    let corruzione = grandezze.nazionale(dati, dati.grandezza("corruzione")?);
    let (del_ministro, del_leader) = (dati.evento(SCANDALI[0])?, dati.evento(SCANDALI[1])?);
    let bersagli: Vec<Bersaglio> = (ministri.dicasteri.iter().enumerate())
        .filter(|(_, st)| st.ministro.is_some())
        .map(|(dicastero, _)| Bersaglio::Ministro { dicastero })
        .chain(
            (partiti.iter().enumerate())
                .filter(|&(j, p)| j != giocatore && !p.sciolto)
                .map(|(partito, _)| Bersaglio::Leader { partito }),
        )
        .collect();
    let u = caso.uniforme(Flusso::Eventi);
    let bersaglio = bersagli.get((u * bersagli.len() as f64) as usize).copied();
    let p = if bersaglio.is_some() {
        (dati.manopola("scandalo_base") * corruzione / 100.0).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let evento = match bersaglio {
        Some(Bersaglio::Leader { .. }) => del_leader,
        _ => del_ministro,
    };
    Some(Candidato {
        evento,
        p,
        gravita: (dati.eventi().get(evento)).map_or(0, |e| e.gravita),
        regione: None,
        bersaglio,
        causa: Causa::Rischio {
            livello: livello(p),
        },
        costo: 0,
        tono: Tono::Ordinario,
        lieve: false,
    })
}

/// Il livello di rischio per la causa: la vista non mostra mai la probabilità in cifre.
pub(crate) fn livello(p: f64) -> Fascia3 {
    if p < 0.05 {
        Fascia3::Poco
    } else if p < 0.2 {
        Fascia3::Medio
    } else {
        Fascia3::Molto
    }
}

/// Una uniforme da `Flusso::Eventi` per candidato, nell'ordine, a prescindere dagli esiti: esce chi
/// ha `u < p`. Restituisce l'indice del vincitore (gravità più alta, a parità il primo) e quelli
/// degli altri usciti. Una `p` non finita non è difesa qui: con NaN `u < p` è falso e il candidato
/// non esce mai, `p >= 1` esce sempre, `p <= 0` mai; i candidati nascono dai dati validati, con `p` in 0-1.
pub(crate) fn estrai(caso: &mut Caso, candidati: &[Candidato]) -> (Option<usize>, Vec<usize>) {
    let usciti: Vec<usize> = (candidati.iter().enumerate())
        .filter_map(|(i, c)| (caso.uniforme(Flusso::Eventi) < c.p).then_some(i))
        .collect();
    let vincitore = (usciti.iter().copied()).reduce(|v, i| {
        if candidati[i].gravita > candidati[v].gravita {
            i
        } else {
            v
        }
    });
    let altri = usciti
        .into_iter()
        .filter(|&i| Some(i) != vincitore)
        .collect();
    (vincitore, altri)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cand(p: f64, gravita: u8) -> Candidato {
        Candidato {
            evento: 0,
            p,
            gravita,
            regione: None,
            bersaglio: None,
            causa: Causa::Rischio {
                livello: livello(p),
            },
            costo: 0,
            tono: Tono::Ordinario,
            lieve: false,
        }
    }

    #[test]
    fn vince_il_piu_grave() {
        let mut caso = Caso::nuovo(1);
        let (v, altri) = estrai(&mut caso, &[cand(1.0, 2), cand(1.0, 5), cand(1.0, 3)]);
        assert_eq!((v, altri), (Some(1), vec![0, 2]));
    }

    #[test]
    fn a_parita_vince_il_primo() {
        let mut caso = Caso::nuovo(1);
        let (v, altri) = estrai(&mut caso, &[cand(1.0, 4), cand(1.0, 4)]);
        assert_eq!((v, altri), (Some(0), vec![1]));
    }

    #[test]
    fn estrae_sempre_tutti() {
        let mut a = Caso::nuovo(7);
        let mut b = Caso::nuovo(7);
        let (v, _) = estrai(&mut a, &[cand(1.0, 1), cand(0.0, 9), cand(1.0, 1)]);
        assert_eq!(v, Some(0));
        for _ in 0..3 {
            b.uniforme(Flusso::Eventi);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn livello_alle_soglie() {
        assert_eq!(livello(0.049), Fascia3::Poco);
        assert_eq!(livello(0.05), Fascia3::Medio);
        assert_eq!(livello(0.2), Fascia3::Molto);
    }
}
