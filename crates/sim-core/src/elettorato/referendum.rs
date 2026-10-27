//! Il voto degli elettori in un referendum: sì, no o astensione per gruppo e punto dell'asse.

use super::voto::{astensione, intenzioni, softmax};
use super::{Elettorato, PER_REGIONE, condizione_di};
use crate::dati::DatiGioco;
use crate::partiti::{Governo, Partito};

/// Elettori che votano sì, no o si astengono.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Affluenza {
    pub si: f64,
    pub no: f64,
    pub astenuti: f64,
}

/// Il quesito: posizione del sì e del no (`None` = nessuna posizione), peso del giudizio sul
/// governo nel sì, invito all'astensione, astensione di base del tipo di referendum, quota
/// toccata per gruppo in 0-1 (vuoto = nessuno), guadagno del sì per partito (vuoto = nessuno:
/// con un guadagno le posizioni non contano).
#[derive(Clone, Debug, PartialEq)]
pub struct Quesito {
    pub pos_si: Option<f64>,
    pub pos_no: Option<f64>,
    pub peso_governo: f64,
    pub invito: f64,
    pub astensione: f64,
    pub toccati: Vec<f64>,
    pub guadagno: Vec<f64>,
}

/// Per gruppo, quanto la disposizione `d` lo tocca: 1 se la sua condizione è fra i `gruppi`, per
/// il peso della regione nel `perimetro` (vuoto = 1); 0 altrimenti.
pub(crate) fn toccati(dati: &DatiGioco, d: usize) -> Vec<f64> {
    let x = &dati.disposizioni()[d];
    (0..dati.elettorato().regioni.len() * PER_REGIONE)
        .map(|i| {
            if !x.gruppi.contains(&condizione_di(i)) {
                return 0.0;
            }
            let regione = &dati.elettorato().regioni[i / PER_REGIONE].id;
            if x.perimetro.is_empty() {
                1.0
            } else {
                x.perimetro.get(regione).copied().unwrap_or(0.0)
            }
        })
        .collect()
}

impl Elettorato {
    /// Per gruppo e punto `x`: `u_si = −β(x − pos_si)² + peso_governo·γ·g`, `u_no = −β(x −
    /// pos_no)²` (senza posizione, distanza zero), `u_ast` = astensione del gruppo +
    /// astensione del tipo − `mobilitazione_toccati`·toccati + `invito`·quota di governo nelle
    /// sue intenzioni; `g` = giudizio
    /// sul partito del Presidente del Consiglio. Quote col softmax, pesate per `h_x` e elettori.
    pub fn referendum(
        &self,
        dati: &DatiGioco,
        partiti: &[Partito],
        governo: &Governo,
        quesito: Quesito,
    ) -> Affluenza {
        let Quesito {
            pos_si,
            pos_no,
            peso_governo,
            invito,
            astensione: base,
            toccati,
            guadagno,
        } = quesito;
        let (beta, gamma) = (
            dati.manopola("beta_distanza"),
            dati.manopola("peso_giudizio"),
        );
        let mobilitazione = dati.manopola("mobilitazione_toccati");
        let distanza = |x: f64, p: Option<f64>| p.map_or(0.0, |p| -beta * (x - p).powi(2));
        let intenzioni = intenzioni(dati, self, partiti, governo);
        let mut a = Affluenza {
            si: 0.0,
            no: 0.0,
            astenuti: 0.0,
        };
        for (i, (g, v)) in self.gruppi.iter().zip(&intenzioni).enumerate() {
            let quota_governo: f64 = (v.iter().enumerate())
                .filter(|&(j, _)| governo.al_governo(j))
                .map(|(_, q)| q)
                .sum();
            let giudizio = g.giudizio.get(governo.presidente).copied().unwrap_or(0.0);
            let toccato = toccati.get(i).copied().unwrap_or(0.0);
            let u_ast =
                astensione(dati, i) + base - mobilitazione * toccato + invito * quota_governo;
            let elettori = self.elettori(dati.elettorato(), i);
            let vantaggio: f64 = (v.iter().zip(&guadagno)).map(|(q, g)| q * g).sum();
            for (k, h) in g.posizione.iter().enumerate() {
                let x = (k + 1) as f64;
                let (u_si, u_no) = if guadagno.is_empty() {
                    (
                        distanza(x, pos_si) + peso_governo * gamma * giudizio,
                        distanza(x, pos_no),
                    )
                } else {
                    (vantaggio, 0.0)
                };
                let q = softmax(&[u_si, u_no, u_ast]);
                let w = h * elettori;
                a.si += w * q[0];
                a.no += w * q[1];
                a.astenuti += w * q[2];
            }
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::economia::Economia;
    use crate::partiti;

    fn affluenza(dati: &DatiGioco, pos: (f64, f64), peso: f64, invito: f64) -> Affluenza {
        let (partiti, governo) = partiti::iniziali(dati);
        let mut el = Elettorato::iniziale(dati, partiti.len(), &Economia::iniziale(dati));
        el.avvia_voto(dati, &partiti, &governo);
        el.referendum(
            dati,
            &partiti,
            &governo,
            Quesito {
                pos_si: Some(pos.0),
                pos_no: Some(pos.1),
                peso_governo: peso,
                invito,
                astensione: dati.manopola("astensione_abrogativo"),
                toccati: Vec::new(),
                guadagno: Vec::new(),
            },
        )
    }

    #[test]
    fn posizioni_uguali_dividono_a_meta() {
        let a = affluenza(&dati_base(), (4.0, 4.0), 0.0, 0.0);
        assert!(a.si > 0.0, "{a:?}");
        assert!((a.si - a.no).abs() < 1e-6 * a.si, "{a:?}");
    }

    #[test]
    fn invito_all_astensione_abbassa_i_votanti() {
        let dati = dati_base();
        let senza = affluenza(&dati, (5.0, 3.0), 1.0, 0.0);
        let con = affluenza(&dati, (5.0, 3.0), 1.0, 2.0);
        let totale = |a: &Affluenza| a.si + a.no + a.astenuti;
        assert!(con.si + con.no < senza.si + senza.no, "{con:?} {senza:?}");
        assert!((totale(&con) - totale(&senza)).abs() < 1e-6 * totale(&senza));
    }
}
