//! Composizione che si muove: lavoro, reddito ed età spostano quota fra i gruppi di una regione.

use super::{Condizione, Elettorato, Fascia, PER_REGIONE, indice_gruppo};
use crate::dati::DatiGioco;
use crate::economia::Economia;
use crate::partita::Trimestre;

/// Media pesata di due grandezze: `a` pesa `qa`, `b` pesa `m`.
fn mescola(a: f64, qa: f64, b: f64, m: f64) -> f64 {
    (qa * a + m * b) / (qa + m)
}

impl Elettorato {
    /// Sposta `massa` di quota (fermata a `quota[da]`) da `da` a `a`: il destinatario diventa la
    /// media pesata per massa di posizione, ancora e giudizio.
    pub fn sposta(&mut self, da: usize, a: usize, massa: f64) {
        let m = if massa > 0.0 {
            massa.min(self.gruppi[da].quota)
        } else {
            0.0
        };
        if m <= 0.0 || da == a {
            return;
        }
        let (mittente, qa) = (self.gruppi[da].clone(), self.gruppi[a].quota);
        let dest = &mut self.gruppi[a];
        for k in 0..mittente.posizione.len() {
            dest.posizione[k] = mescola(dest.posizione[k], qa, mittente.posizione[k], m);
            dest.ancora[k] = mescola(dest.ancora[k], qa, mittente.ancora[k], m);
        }
        for (x, y) in dest.giudizio.iter_mut().zip(&mittente.giudizio) {
            *x = mescola(*x, qa, *y, m);
        }
        self.rinnova(da, a, m);
    }

    /// Sposta la quota senza mescolare: elettori nuovi al posto di chi esce.
    pub fn rinnova(&mut self, da: usize, a: usize, massa: f64) {
        let m = if massa > 0.0 {
            massa.min(self.gruppi[da].quota)
        } else {
            0.0
        };
        self.gruppi[da].quota -= m;
        self.gruppi[a].quota += m;
    }

    /// Lavoro, reddito ed età (a gennaio). Alla fine `ultima_disoccupazione` è quella di `e`.
    pub fn composizione(&mut self, dati: &DatiGioco, e: &Economia, data: Trimestre) {
        let n_regioni = dati.elettorato().regioni.len();
        self.lavoro(e, n_regioni);
        self.mobilita(dati);
        if data.trimestre == 1 {
            self.eta(dati, e, data);
        }
        self.ultima_disoccupazione = e.regioni.iter().map(|r| r.disoccupazione).collect();
    }

    fn lavoro(&mut self, e: &Economia, n_regioni: usize) {
        use Condizione::{Autonomo, DipendentePrivato as Priv, DisoccupatoInattivo as Dis};
        for r in 0..n_regioni {
            let delta = e.regioni[r].disoccupazione - self.ultima_disoccupazione[r];
            for f in Fascia::TUTTE {
                let (p, a, d) = (
                    indice_gruppo(r, Priv, f),
                    indice_gruppo(r, Autonomo, f),
                    indice_gruppo(r, Dis, f),
                );
                if delta > 0.0 {
                    let frazione = (delta / 100.0).min(1.0);
                    let (mp, ma) = (
                        frazione * self.gruppi[p].quota,
                        frazione * self.gruppi[a].quota,
                    );
                    self.sposta(p, d, mp);
                    self.sposta(a, d, ma);
                } else if delta < 0.0 {
                    let occupati = self.gruppi[p].quota + self.gruppi[a].quota;
                    self.sposta(d, p, -delta / 100.0 * occupati);
                }
            }
        }
    }

    fn mobilita(&mut self, dati: &DatiGioco) {
        let d = dati.elettorato();
        let crescita = |i: usize| -> f64 {
            let g = &self.gruppi[i];
            let c = g.reddito.reale() / g.reddito_prima.reale() - 1.0;
            if c.is_finite() { c } else { 0.0 }
        };
        // Per condizione: (elettori, crescita pesata) in tutta Italia.
        let mut per_condizione = [(0.0, 0.0); 6];
        for i in 0..self.gruppi.len() {
            let w = self.gruppi[i].quota * d.regioni[i / PER_REGIONE].elettori as f64;
            let c = i % PER_REGIONE / Fascia::TUTTE.len();
            per_condizione[c].0 += w;
            per_condizione[c].1 += w * crescita(i);
        }
        let n: f64 = per_condizione.iter().map(|p| p.0).sum();
        if n <= 0.0 {
            return;
        }
        let g_italia = per_condizione.iter().map(|p| p.1).sum::<f64>() / n;
        let mobilita = dati.manopola("mobilita_fasce");
        for (c, cond) in Condizione::TUTTE.into_iter().enumerate() {
            let (w, somma) = per_condizione[c];
            if w <= 0.0 {
                continue;
            }
            let scarto = somma / w - g_italia;
            let x = mobilita * 100.0 * scarto.abs();
            let frazione = (x / 100.0).min(1.0);
            for r in 0..d.regioni.len() {
                let [b, m, a] = Fascia::TUTTE.map(|f| indice_gruppo(r, cond, f));
                let (qb, qm, qa) = (
                    self.gruppi[b].quota,
                    self.gruppi[m].quota,
                    self.gruppi[a].quota,
                );
                if scarto < 0.0 {
                    self.sposta(m, b, frazione * qm);
                    self.sposta(a, m, frazione * qa);
                } else if scarto > 0.0 {
                    self.sposta(m, a, frazione * qm);
                    self.sposta(b, m, frazione * qb);
                }
            }
        }
    }

    fn eta(&mut self, dati: &DatiGioco, e: &Economia, data: Trimestre) {
        use Condizione::{
            Autonomo, DipendentePrivato as Priv, DipendentePubblico as Pub,
            DisoccupatoInattivo as Dis, Pensionato, Studente,
        };
        let d = dati.elettorato();
        let nuovi: i64 = e
            .pensioni
            .iter()
            .filter(|a| a.anno == data.anno)
            .map(|a| a.pensionati)
            .sum();
        let lavoratori: f64 = (0..self.gruppi.len())
            .filter(|i| {
                matches!(
                    Condizione::TUTTE[i % PER_REGIONE / Fascia::TUTTE.len()],
                    Priv | Pub | Autonomo
                )
            })
            .map(|i| self.gruppi[i].quota * d.regioni[i / PER_REGIONE].elettori as f64)
            .sum();
        let tasso = if lavoratori > 0.0 {
            (nuovi as f64 / lavoratori).clamp(0.0, 1.0)
        } else {
            0.0
        };
        let uscite = dati.manopola("uscite_pensionati");
        let studenti = dati.manopola("uscita_studenti");
        for r in 0..d.regioni.len() {
            let u = e.regioni[r].disoccupazione / 100.0;
            for f in Fascia::TUTTE {
                let pens = indice_gruppo(r, Pensionato, f);
                for c in [Priv, Pub, Autonomo] {
                    let da = indice_gruppo(r, c, f);
                    self.sposta(da, pens, tasso * self.gruppi[da].quota);
                }
                let stud = indice_gruppo(r, Studente, f);
                self.rinnova(pens, stud, uscite * self.gruppi[pens].quota);
                let massa = studenti * self.gruppi[stud].quota;
                self.sposta(stud, indice_gruppo(r, Priv, f), (1.0 - u) * massa);
                self.sposta(stud, indice_gruppo(r, Dis, f), u * massa);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::elettorato::{PUNTI, Reddito};

    fn gruppo(quota: f64, punto: usize) -> crate::elettorato::Gruppo {
        let mut posizione = [0.0; PUNTI];
        posizione[punto] = 1.0;
        crate::elettorato::Gruppo {
            quota,
            posizione,
            ancora: posizione,
            giudizio: vec![0.5],
            reddito: Reddito::default(),
            reddito_prima: Reddito::default(),
        }
    }

    fn el() -> Elettorato {
        Elettorato {
            gruppi: vec![gruppo(0.6, 1), gruppo(0.0, 6)],
            base: crate::elettorato::Indici {
                salario: 1.0,
                salari_pubblici: 1.0,
                pil: 1.0,
                pensione: 1.0,
                sussidio: 1.0,
                pressione: 1.0,
            },
            ultima_disoccupazione: vec![],
        }
    }

    #[test]
    fn sposta_porta_con_se_la_distribuzione() {
        let mut e = el();
        e.sposta(0, 1, 0.3);
        assert_eq!(e.gruppi[1].posizione, e.gruppi[0].posizione);
        assert_eq!(e.gruppi[1].ancora, e.gruppi[0].ancora);
        assert!((e.gruppi[0].quota + e.gruppi[1].quota - 0.6).abs() < 1e-12);
        assert!((e.gruppi[1].quota - 0.3).abs() < 1e-12);
    }

    #[test]
    fn sposta_mescola_in_media_pesata() {
        let mut e = el();
        e.gruppi[1] = gruppo(0.3, 5);
        e.sposta(0, 1, 0.3);
        assert!((e.gruppi[1].posizione[1] - 0.5).abs() < 1e-12);
        assert!((e.gruppi[1].posizione[5] - 0.5).abs() < 1e-12);
    }

    #[test]
    fn rinnova_lascia_l_istogramma_del_destinatario() {
        let mut e = el();
        e.gruppi[1] = gruppo(0.1, 5);
        let prima = e.gruppi[1].posizione;
        e.rinnova(0, 1, 0.3);
        assert_eq!(e.gruppi[1].posizione, prima);
        assert!((e.gruppi[1].quota - 0.4).abs() < 1e-12);
        assert!((e.gruppi[0].quota - 0.3).abs() < 1e-12);
    }

    #[test]
    fn massa_oltre_la_quota_si_ferma_alla_quota() {
        let mut e = el();
        e.sposta(0, 1, 9.0);
        assert!(e.gruppi[0].quota.abs() < 1e-12);
        assert!((e.gruppi[1].quota - 0.6).abs() < 1e-12);
    }
}
