//! La campagna elettorale: bersagli, leve e spinte sul voto (scelte 13-15 del piano 8b,
//! effetti-della-campagna-e-promesse:51-58).

use crate::dati::DatiGioco;
use crate::elettorato::{Condizione, Elettorato, PER_REGIONE, PUNTI, condizione_di};
use crate::errore::Errore;
use crate::partiti::Partito;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Al più tanti bersagli per campagna.
const MASSIMO_BERSAGLI: usize = 3;

/// Una condizione (i suoi 60 gruppi) o una regione (i suoi 18), per indice nei dati.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Bersaglio {
    Condizione(Condizione),
    Regione(usize),
}

impl Bersaglio {
    pub(crate) fn contiene(self, i: usize) -> bool {
        match self {
            Bersaglio::Condizione(c) => condizione_di(i) == c,
            Bersaglio::Regione(r) => i / PER_REGIONE == r,
        }
    }

    /// s_b: la quota degli elettori nei gruppi del bersaglio.
    pub(crate) fn quota_elettori(self, dati: &DatiGioco, el: &Elettorato) -> f64 {
        let d = dati.elettorato();
        let (mut dentro, mut tutti) = (0.0, 0.0);
        for i in 0..el.gruppi.len() {
            let w = el.elettori(d, i);
            tutti += w;
            if self.contiene(i) {
                dentro += w;
            }
        }
        dentro / tutti
    }

    /// Il nome per il giocatore; `None` per una regione che non c'è.
    fn nome(self, dati: &DatiGioco) -> Option<String> {
        match self {
            Bersaglio::Condizione(c) => Some(
                match c {
                    Condizione::DipendentePrivato => "dipendenti privati",
                    Condizione::DipendentePubblico => "dipendenti pubblici",
                    Condizione::Autonomo => "autonomi",
                    Condizione::Pensionato => "pensionati",
                    Condizione::DisoccupatoInattivo => "disoccupati e inattivi",
                    Condizione::Studente => "studenti",
                }
                .into(),
            ),
            Bersaglio::Regione(r) => dati.regioni().get(r).map(|x| x.nome.clone()),
        }
    }
}

/// Mobilitare abbassa l'astensione dei vicini, persuadere alza la valence, convincere sposta le
/// posizioni (effetti-della-campagna-e-promesse:53-57).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Leva {
    Mobilitare,
    Persuadere,
    Convincere,
}

impl Leva {
    pub(crate) fn forza(self, dati: &DatiGioco) -> f64 {
        dati.manopola(match self {
            Leva::Mobilitare => "forza_mobilitare",
            Leva::Persuadere => "forza_persuadere",
            Leva::Convincere => "forza_convincere",
        })
    }

    fn verbo(self) -> &'static str {
        match self {
            Leva::Mobilitare => "mobilita",
            Leva::Persuadere => "persuade",
            Leva::Convincere => "convince",
        }
    }
}

/// Euro spesi con una leva su un bersaglio.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Spesa {
    pub bersaglio: Bersaglio,
    pub leva: Leva,
    pub euro: i64,
}

/// √(euro / `spesa_riferimento`) sotto 1/3, sotto 2/3 o oltre (scelta 15).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum Intensita {
    Poco,
    Medio,
    Molto,
}

/// Una spesa in anteprima: costo esatto, intensità e la sua causa in una riga.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAnteprimaCampagna {
    pub bersaglio: Bersaglio,
    pub leva: Leva,
    pub euro: i64,
    pub intensita: Intensita,
    pub causa: String,
}

/// Spinte della campagna sul voto, gruppo × partito: valence in più (persuadere) e utilità
/// dell'astensione in meno ai punti entro 1 dalla dichiarata del partito (mobilitare).
#[derive(Clone, Debug, PartialEq)]
pub struct Spinte {
    pub valence: Vec<Vec<f64>>,
    pub mobilita: Vec<Vec<f64>>,
}

/// √(euro / (`spesa_riferimento` · s_b)): la radice della spesa per elettore del bersaglio
/// (effetti-della-campagna-e-promesse:52); 1 è la spesa di riferimento su tutto l'elettorato.
fn radice(dati: &DatiGioco, el: &Elettorato, s: &Spesa) -> f64 {
    let s_b = s.bersaglio.quota_elettori(dati, el);
    (s.euro.max(0) as f64 / (dati.manopola("spesa_riferimento") * s_b)).sqrt()
}

fn forza(dati: &DatiGioco, el: &Elettorato, s: &Spesa) -> f64 {
    s.leva.forza(dati) * radice(dati, el, s)
}

/// Le spinte delle campagne dei partiti attivi (scelta 13): `forza_<leva>` × √(euro /
/// (`spesa_riferimento` · s_b)) nei gruppi del bersaglio. Una voce per partito, sciolti compresi.
pub(crate) fn spinte(dati: &DatiGioco, el: &Elettorato, partiti: &[Partito]) -> Spinte {
    let n_gruppi = el.gruppi.len();
    let mut s = Spinte {
        valence: vec![vec![0.0; partiti.len()]; n_gruppi],
        mobilita: vec![vec![0.0; partiti.len()]; n_gruppi],
    };
    for (j, p) in partiti.iter().enumerate().filter(|(_, p)| !p.sciolto) {
        for spesa in &p.campagna {
            let tabella = match spesa.leva {
                Leva::Persuadere => &mut s.valence,
                Leva::Mobilitare => &mut s.mobilita,
                Leva::Convincere => continue,
            };
            let f = forza(dati, el, spesa);
            for (i, riga) in tabella.iter_mut().enumerate() {
                if spesa.bersaglio.contiene(i) {
                    riga[j] += f;
                }
            }
        }
    }
    s
}

/// Convincere (scelta 13): in ogni gruppo del bersaglio la quota `forza_convincere` × √(euro /
/// (`spesa_riferimento` · s_b)), al più tutto, dell'istogramma passa al punto più vicino alla
/// dichiarata del partito. Si chiama prima dell'ancora, che poi la riassorbe.
pub(crate) fn convinci(dati: &DatiGioco, partiti: &[Partito], el: &mut Elettorato) {
    for p in partiti.iter().filter(|p| !p.sciolto) {
        // A metà fra due punti va su quello più a destra (`round` allontana da zero).
        let k = (p.posizione_dichiarata.round() as usize).clamp(1, PUNTI) - 1;
        for spesa in p.campagna.iter().filter(|s| s.leva == Leva::Convincere) {
            let quota = forza(dati, el, spesa).min(1.0);
            for (i, g) in el.gruppi.iter_mut().enumerate() {
                if !spesa.bersaglio.contiene(i) {
                    continue;
                }
                let mut spostata = 0.0;
                for h in &mut g.posizione {
                    spostata += quota * *h;
                    *h *= 1.0 - quota;
                }
                g.posizione[k] += spostata;
            }
        }
    }
}

/// I rifiuti del comando e dell'anteprima, a parte la finestra: più di tre bersagli, una regione
/// che non c'è, una spesa non positiva, un bersaglio ripetuto, il totale oltre la `cassa`.
pub(crate) fn valida(dati: &DatiGioco, spese: &[Spesa], cassa: i64) -> Result<(), Errore> {
    let ko = |m: String| Err(Errore::Comando(m));
    if spese.len() > MASSIMO_BERSAGLI {
        return ko("Al massimo 3 bersagli".into());
    }
    for (n, s) in spese.iter().enumerate() {
        let Some(nome) = s.bersaglio.nome(dati) else {
            return ko("Regione sconosciuta".into());
        };
        if s.euro <= 0 {
            return ko("Spesa non positiva".into());
        }
        if spese[..n].iter().any(|x| x.bersaglio == s.bersaglio) {
            return ko(format!("Bersaglio ripetuto: {nome}"));
        }
    }
    let totale = spese.iter().try_fold(0_i64, |t, s| t.checked_add(s.euro));
    if totale.is_none_or(|t| t > cassa) {
        return ko(format!("Spesa oltre la cassa: {} euro", migliaia(cassa)));
    }
    Ok(())
}

/// L'anteprima di spese già valide (scelta 15): legge i dati e gli elettori per gruppo, pubblici.
pub(crate) fn anteprima(
    dati: &DatiGioco,
    el: &Elettorato,
    spese: &[Spesa],
) -> Vec<VoceAnteprimaCampagna> {
    let riferimento = dati.manopola("spesa_riferimento").round() as i64;
    (spese.iter())
        .map(|s| {
            let r = radice(dati, el, s);
            let intensita = if r < 1.0 / 3.0 {
                Intensita::Poco
            } else if r < 2.0 / 3.0 {
                Intensita::Medio
            } else {
                Intensita::Molto
            };
            let causa = format!(
                "{} {}: {} euro, il {}% della forza di {} euro spesi su tutto l'elettorato \
                 (cresce con la radice della spesa per elettore)",
                maiuscola(s.leva.verbo()),
                s.bersaglio.nome(dati).unwrap_or_default(),
                migliaia(s.euro),
                (r * 100.0).round() as i64,
                migliaia(riferimento),
            );
            VoceAnteprimaCampagna {
                bersaglio: s.bersaglio,
                leva: s.leva,
                euro: s.euro,
                intensita,
                causa,
            }
        })
        .collect()
}

/// «Spende {euro} euro: persuade {bersagli}, mobilita {bersagli}, convince {bersagli}», senza le
/// leve non usate.
pub(crate) fn riga(dati: &DatiGioco, spese: &[Spesa], euro: i64) -> String {
    let parti: Vec<String> = [Leva::Persuadere, Leva::Mobilitare, Leva::Convincere]
        .into_iter()
        .filter_map(|leva| {
            let nomi: Vec<String> = (spese.iter())
                .filter(|s| s.leva == leva)
                .filter_map(|s| s.bersaglio.nome(dati))
                .collect();
            (!nomi.is_empty()).then(|| format!("{} {}", leva.verbo(), elenco(&nomi)))
        })
        .collect();
    format!("Spende {} euro: {}", migliaia(euro), parti.join(", "))
}

/// «a», «a e b», «a, b e c».
fn elenco(nomi: &[String]) -> String {
    match nomi.split_last() {
        Some((ultimo, primi)) if !primi.is_empty() => format!("{} e {ultimo}", primi.join(", ")),
        _ => nomi.join(""),
    }
}

fn maiuscola(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|p| p.to_uppercase().chain(c).collect())
        .unwrap_or_default()
}

/// Euro col punto delle migliaia: 3.000.000.
fn migliaia(euro: i64) -> String {
    let cifre = euro.unsigned_abs().to_string();
    let mut s = String::new();
    for (n, c) in cifre.chars().enumerate() {
        if n > 0 && (cifre.len() - n).is_multiple_of(3) {
            s.push('.');
        }
        s.push(c);
    }
    if euro < 0 { format!("-{s}") } else { s }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dati::dati_base;
    use crate::economia::Economia;
    use crate::partiti::iniziali;

    /// La spesa di riferimento sulla quota s_b degli elettori dei pensionati vale l'intensità 1
    /// (R23: la spinta per elettore del bersaglio).
    #[test]
    fn persuadere_una_condizione_tocca_solo_i_suoi_gruppi_e_il_suo_partito() {
        let dati = dati_base();
        let (mut partiti, _) = iniziali(&dati);
        let el = Elettorato::iniziale(&dati, partiti.len(), &Economia::iniziale(&dati));
        let d = dati.elettorato();
        let pesi: Vec<f64> = (0..el.gruppi.len()).map(|i| el.elettori(d, i)).collect();
        let pensionati: f64 = (pesi.iter().enumerate())
            .filter(|(i, _)| condizione_di(*i) == Condizione::Pensionato)
            .map(|(_, w)| w)
            .sum();
        let s_b = pensionati / pesi.iter().sum::<f64>();
        let euro = (dati.manopola("spesa_riferimento") * s_b).round() as i64;
        partiti[2].campagna.push(Spesa {
            bersaglio: Bersaglio::Condizione(Condizione::Pensionato),
            leva: Leva::Persuadere,
            euro,
        });
        let s = spinte(&dati, &el, &partiti);
        let f = dati.manopola("forza_persuadere");
        let mut toccati = 0;
        for (i, riga) in s.valence.iter().enumerate() {
            for (j, &v) in riga.iter().enumerate() {
                if j == 2 && condizione_di(i) == Condizione::Pensionato {
                    assert!((v - f).abs() < 1e-6, "{i} {j} {v}");
                    toccati += 1;
                } else {
                    assert_eq!(v, 0.0, "{i} {j}");
                }
            }
        }
        assert_eq!(toccati, 60);
        assert!(s.mobilita.iter().flatten().all(|&m| m == 0.0));
        // Lo sciolto non fa campagna.
        partiti[2].sciolto = true;
        let s = spinte(&dati, &el, &partiti);
        assert!(s.valence.iter().flatten().all(|&v| v == 0.0));
        assert_eq!(s.valence[0].len(), partiti.len());
    }

    #[test]
    fn migliaia_col_punto() {
        assert_eq!(migliaia(0), "0");
        assert_eq!(migliaia(999), "999");
        assert_eq!(migliaia(1000), "1.000");
        assert_eq!(migliaia(3_000_000), "3.000.000");
        assert_eq!(migliaia(-187_500), "-187.500");
    }

    #[test]
    fn la_riga_della_spesa() {
        let dati = dati_base();
        let s = |bersaglio, leva| Spesa {
            bersaglio,
            leva,
            euro: 1,
        };
        let spese = [
            s(Bersaglio::Regione(0), Leva::Mobilitare),
            s(
                Bersaglio::Condizione(Condizione::Studente),
                Leva::Persuadere,
            ),
            s(
                Bersaglio::Condizione(Condizione::Pensionato),
                Leva::Mobilitare,
            ),
        ];
        let nome = dati.regioni()[0].nome.clone();
        assert_eq!(
            riga(&dati, &spese, 1_500_000),
            format!("Spende 1.500.000 euro: persuade studenti, mobilita {nome} e pensionati")
        );
        assert!(valida(&dati, &spese, 3).is_ok());
        assert!(valida(&dati, &spese, 2).is_err());
        let enorme = [
            s(Bersaglio::Regione(0), Leva::Mobilitare),
            s(Bersaglio::Regione(1), Leva::Mobilitare),
        ]
        .map(|x| Spesa {
            euro: i64::MAX,
            ..x
        });
        assert!(valida(&dati, &enorme, i64::MAX).is_err());
    }
}
