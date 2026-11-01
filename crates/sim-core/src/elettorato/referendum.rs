//! Il voto degli elettori in un referendum: sì, no o astensione per gruppo e punto dell'asse.

use super::voto::{astensione, intenzioni, softmax};
use super::{Elettorato, PER_REGIONE, condizione_di};
use crate::dati::DatiGioco;
use crate::informazione::Sondaggio;
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
    /// sue intenzioni; `g` = giudizio sul partito del Presidente del Consiglio. Con `guadagno`
    /// (quesito elettorale) `u_si` è il guadagno del sì pesato sulle intenzioni del gruppo, `u_no`
    /// = 0 e le posizioni sono ignorate. Quote col softmax, pesate per `h_x` e elettori.
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
        let intenzioni = intenzioni(dati, self, partiti, governo, None);
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
            // `v` ha un posto in più dei partiti: l'astensione, che non ha guadagno.
            debug_assert!(guadagno.is_empty() || guadagno.len() + 1 == v.len());
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
                a.aggiungi(h * elettori, [u_si, u_no, u_ast]);
            }
        }
        a
    }
}

impl Affluenza {
    /// `w` elettori divisi fra sì, no e astensione col softmax delle utilità `u`.
    fn aggiungi(&mut self, w: f64, u: [f64; 3]) {
        let q = softmax(&u);
        self.si += w * q[0];
        self.no += w * q[1];
        self.astenuti += w * q[2];
    }
}

/// `Elettorato::referendum` sul `sondaggio`, che sbaglia come quelli veri (sondaggi-ed-errore:55):
/// le regole IA non leggono lo stato vero. Per regione le posizioni dell'incrocio regionale al
/// posto dei gruppi, pesate per gli elettori della regione nei dati; astensione dei dati e toccati
/// sono la media dei gruppi della regione, pesata per le loro quote nei dati (la composizione del
/// turno zero, non quella viva). Senza giudizio sul governo, invito né guadagno: `peso_governo`,
/// `invito` e `guadagno` del quesito non contano.
pub(crate) fn stima(dati: &DatiGioco, sondaggio: &Sondaggio, quesito: &Quesito) -> Affluenza {
    let beta = dati.manopola("beta_distanza");
    let mobilitazione = dati.manopola("mobilitazione_toccati");
    let distanza = |x: f64, p: Option<f64>| p.map_or(0.0, |p| -beta * (x - p).powi(2));
    let mut a = Affluenza {
        si: 0.0,
        no: 0.0,
        astenuti: 0.0,
    };
    let regioni = dati.elettorato().regioni.iter().zip(&sondaggio.regioni);
    for (r, (regione, incrocio)) in regioni.enumerate() {
        let gruppi = r * PER_REGIONE..(r + 1) * PER_REGIONE;
        let quota = |i: usize| dati.elettorato().gruppi.get(i).map_or(0.0, |g| g.quota);
        let peso: f64 = gruppi.clone().map(quota).sum();
        let media =
            |f: &dyn Fn(usize) -> f64| gruppi.clone().map(|i| quota(i) * f(i)).sum::<f64>() / peso;
        let u_ast = media(&|i| astensione(dati, i)) + quesito.astensione
            - mobilitazione * media(&|i| quesito.toccati.get(i).copied().unwrap_or(0.0));
        for (k, h) in incrocio.posizione.iter().enumerate() {
            let x = (k + 1) as f64;
            let u = [
                distanza(x, quesito.pos_si),
                distanza(x, quesito.pos_no),
                u_ast,
            ];
            a.aggiungi(h * regione.elettori as f64, u);
        }
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::economia::Economia;
    use crate::elettorato::{PUNTI, media_pesata};
    use crate::informazione::Incrocio;
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

    /// Un sondaggio senza errore, con le posizioni vere delle regioni: la stima di sì e no resta
    /// entro 3 punti dal voto vero. Spostato di un punto verso il sì, la stima del sì sale (da 31,0
    /// a 35,6%): legge il sondaggio.
    #[test]
    fn la_stima_sul_sondaggio_vero_e_vicina() {
        let dati = dati_base();
        let (partiti, governo) = partiti::iniziali(&dati);
        let mut el = Elettorato::iniziale(&dati, partiti.len(), &Economia::iniziale(&dati));
        el.avvia_voto(&dati, &partiti, &governo);
        let regioni = (dati.elettorato().regioni.iter().enumerate())
            .map(|(r, reg)| {
                let gruppi = r * PER_REGIONE..(r + 1) * PER_REGIONE;
                let pesi: Vec<f64> = (gruppi.clone())
                    .map(|i| el.elettori(dati.elettorato(), i))
                    .collect();
                let posizioni = gruppi.map(|i| &el.gruppi[i].posizione[..]);
                Incrocio {
                    id: reg.id.clone(),
                    campione: 0,
                    quote: Vec::new(),
                    posizione: media_pesata(&pesi, posizioni, PUNTI),
                    approvazione: 0.0,
                }
            })
            .collect();
        let sondaggio = Sondaggio {
            istituti: Vec::new(),
            condizioni: Vec::new(),
            fasce: Vec::new(),
            regioni,
        };
        let d = dati.disposizione("regime_pensioni").unwrap();
        let quesito = Quesito {
            pos_si: Some(5.0),
            pos_no: Some(3.0),
            peso_governo: 0.0,
            invito: 0.0,
            astensione: dati.manopola("astensione_abrogativo"),
            toccati: toccati(&dati, d),
            guadagno: Vec::new(),
        };
        let quote = |a: Affluenza| {
            let totale = a.si + a.no + a.astenuti;
            (a.si / totale, a.no / totale)
        };
        let (si, no) = quote(stima(&dati, &sondaggio, &quesito));
        // Il sondaggio spostato di un punto a destra, verso il sì: la stima lo segue.
        let mut a_destra = sondaggio.clone();
        for r in &mut a_destra.regioni {
            let ultimo = r.posizione[PUNTI - 1];
            r.posizione.rotate_right(1);
            r.posizione[0] = 0.0;
            r.posizione[PUNTI - 1] += ultimo;
        }
        let (si_spostato, _) = quote(stima(&dati, &a_destra, &quesito));
        let (si_vero, no_vero) = quote(el.referendum(&dati, &partiti, &governo, quesito));
        assert!((si - si_vero).abs() < 0.03, "{si} {si_vero}");
        assert!((no - no_vero).abs() < 0.03, "{no} {no_vero}");
        assert!(si_spostato > si + 0.02, "{si_spostato} {si}");
    }
}
