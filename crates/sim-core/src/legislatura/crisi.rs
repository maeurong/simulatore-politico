//! La crisi: la sfiducia a blocchi (iter-legislativo-italiano:43, appello nominale) e il valore
//! della caduta per un alleato in ultimatum (scelta 12 del piano 7b).

use super::coalizione::{Dichiarazione, Quadro, coalizioni_al_voto, distanza};
use super::spartizione::{maggioranza_contigua, primo_per_seggi};
use crate::parlamento::CausaIntesa;
use crate::partiti::Governo;

/// La linea della sfiducia, sì per partito, senza estrazioni. Votano sì i partiti fuori dal
/// `governo`; degli alleati in `ultimatum`, sulla sfiducia al ministro del partito
/// `partito_del_ministro` quelli di un altro partito, su quella al governo quelli con `cade(j)`.
/// Gli altri no.
pub(crate) fn linea_sfiducia(
    n: usize,
    governo: &Governo,
    ultimatum: &[Option<CausaIntesa>],
    partito_del_ministro: Option<usize>,
    cade: impl Fn(usize) -> bool,
) -> Vec<bool> {
    (0..n)
        .map(|j| {
            if !governo.al_governo(j) {
                return true;
            }
            let in_ultimatum =
                j != governo.presidente && ultimatum.get(j).is_some_and(Option::is_some);
            in_ultimatum && partito_del_ministro.map_or_else(|| cade(j), |m| m != j)
        })
        .collect()
}

/// `j`, alleato del governo `attuale` presieduto da `presidente`, vota la caduta se gli vale più
/// che restare, sui termini di `Quadro::termini` e coi seggi veri che `q` porta. Restare: seggi,
/// governo certo, distanza da `attuale`, intesa sotto la base. Cadere: la maggioranza contigua
/// (formazione-di-coalizioni:13) senza `presidente` se è il giocatore, altrimenti con tutti come
/// il Capo dello Stato (scelta 11 del piano 7b), se contiene `j`: seggi, governo certo,
/// distanza da quella, guida se `j` è il primo per seggi (a parità l'indice più basso);
/// altrimenti i termini di `j` al voto proiettato sul sondaggio, con le coalizioni dell'ultimo
/// voto e `j` fuori da quella di `presidente`.
pub(crate) fn vota_la_caduta(q: &Quadro, j: usize, attuale: &[usize], presidente: usize) -> bool {
    let m = |nome: &str| q.dati.manopola(nome);
    let (camera, senato) = q.seggi(&Dichiarazione::default());
    let di = |v: &[u32], i: usize| v.get(i).copied().unwrap_or(0);
    let seggi = |i: usize| f64::from(di(&camera, i)) + f64::from(di(&senato, i));
    let totale =
        f64::from(camera.iter().sum::<u32>().max(1)) + f64::from(senato.iter().sum::<u32>().max(1));
    let quota = seggi(j) / totale;
    let intesa = q.intesa.get(j).copied().unwrap_or(0.0);
    let restare = m("peso_seggi_coalizione") * quota + m("peso_governare")
        - m("peso_distanza_coalizione") * distanza(q.partiti, attuale, j)
        - m("peso_intesa_coalizione") * (m("intesa_base") - intesa).max(0.0) / 100.0;
    let escluso = (presidente == q.giocatore).then_some(presidente);
    let contigua = maggioranza_contigua(q.partiti, &camera, &senato, escluso);
    let dopo = match contigua.filter(|k| k.contains(&j)) {
        Some(k) => {
            let guida = if primo_per_seggi(&k, &camera, &senato) == Some(j) {
                m("valore_guida")
            } else {
                0.0
            };
            m("peso_seggi_coalizione") * quota + m("peso_governare")
                - m("peso_distanza_coalizione") * distanza(q.partiti, &k, j)
                + guida
        }
        None => {
            let ultime: Vec<Vec<usize>> = (q.ultime.iter())
                .map(|k| {
                    let senza = k.iter().copied().filter(|&i| i != j);
                    if k.contains(&presidente) {
                        senza.collect()
                    } else {
                        k.clone()
                    }
                })
                .collect();
            let p = Quadro {
                ultime: &ultime,
                seggi_veri: None,
                ..q.clone()
            };
            let sua = coalizioni_al_voto(&ultime, q.giocatore, None);
            let d = Dichiarazione {
                partiti: (sua.into_iter().next().unwrap_or_default().into_iter())
                    .filter(|&i| i != q.giocatore)
                    .collect(),
                cessioni: Vec::new(),
            };
            p.valore(&d, j)
        }
    };
    dopo > restare
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Sette partiti, governo di 4 e 5 con 5 presidente.
    #[test]
    fn la_sfiducia_e_deterministica_a_blocchi() {
        let g = Governo {
            presidente: 5,
            dicasteri: vec![5, 5, 4],
            coalizione: vec![4, 5],
        };
        let opposizione = [true, true, true, true, false, false, true];
        let nessuno = vec![None; 7];
        for cade in [true, false] {
            assert_eq!(linea_sfiducia(7, &g, &nessuno, None, |_| cade), opposizione);
            assert_eq!(
                linea_sfiducia(7, &g, &nessuno, Some(5), |_| cade),
                opposizione
            );
        }
        let mut ultimatum = nessuno;
        ultimatum[4] = Some(CausaIntesa::Spartizione);
        let al_governo = |cade| linea_sfiducia(7, &g, &ultimatum, None, |j| j == 4 && cade)[4];
        assert!(al_governo(true));
        assert!(!al_governo(false));
        assert!(linea_sfiducia(7, &g, &ultimatum, Some(5), |_| false)[4]);
        assert!(!linea_sfiducia(7, &g, &ultimatum, Some(4), |_| true)[4]);
    }
}
