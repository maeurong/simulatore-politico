//! La fondazione del partito del giocatore: nome, posizione sull'asse, anteprima sul sondaggio.

use super::VoceDisponibilita;
use crate::dati::DatiGioco;
use crate::elettorato::{PUNTI, media_pesata};
use crate::errore::Errore;
use crate::informazione::Sondaggio;
use crate::partita::Partita;
use crate::partiti::{ID_GIOCATORE, Partito, Vocazione};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

/// Il nome del partito e la posizione scelta, una di `posizioni`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fondazione {
    pub nome: String,
    pub posizione: f64,
}

/// Una posizione della fondazione: `quota_attesa` è la quota del giocatore fra i votanti, media sugli
/// istituti del sondaggio del turno zero; `disponibilita` quella dei partiti IA con la coalizione
/// vuota.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePosizione {
    pub posizione: f64,
    pub quota_attesa: f64,
    pub disponibilita: Vec<VoceDisponibilita>,
}

/// `istogramma`: autocollocazione dell'elettorato sui 7 punti, dalle regioni del sondaggio.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct AnteprimaFondazione {
    pub istogramma: [f64; PUNTI],
    pub posizioni: Vec<VocePosizione>,
}

const LUNGHEZZA_NOME: usize = 40;
const SEGNAPOSTO: &str = "Partito nuovo";

/// I punti medi fra due partiti consecutivi sull'asse, in ordine crescente: nessuno coincide con un
/// partito.
pub fn posizioni(dati: &DatiGioco) -> Vec<f64> {
    let mut posizioni: Vec<f64> = (dati.scenario().partiti.iter())
        .map(|p| p.posizione)
        .collect();
    posizioni.sort_by(f64::total_cmp);
    posizioni.dedup();
    posizioni.windows(2).map(|w| (w[0] + w[1]) / 2.0).collect()
}

/// Il partito del giocatore: la luna di miele del leader al primo mandato sta in `valence` e non
/// in `valence_base`, così si spegne col ritorno della valence.
fn partito_nuovo(dati: &DatiGioco, nome: &str, posizione: f64) -> Partito {
    let base = dati.manopola("valence_partito_nuovo");
    Partito {
        id: ID_GIOCATORE.into(),
        nome: nome.into(),
        posizione_dichiarata: posizione,
        posizione_percepita: posizione,
        valence: base + dati.manopola("luna_di_miele"),
        valence_base: base,
        vocazione: Vocazione::Mista,
        leader: String::new(),
    }
}

fn ko<T>(m: String) -> Result<T, Errore> {
    Err(Errore::Fondazione(m))
}

/// Media sugli istituti della quota del partito `j` fra i votanti; l'astensione è l'ultima voce.
pub(crate) fn quota_fra_i_votanti(s: &Sondaggio, j: usize) -> f64 {
    let quote = s.istituti.iter().map(|r| {
        let astensione = r.quote.last().copied().unwrap_or(0.0);
        r.quote.get(j).copied().unwrap_or(0.0) / (1.0 - astensione)
    });
    quote.sum::<f64>() / s.istituti.len() as f64
}

/// Media delle posizioni degli incroci regionali, pesata con gli elettori della regione.
fn istogramma(dati: &DatiGioco, s: &Sondaggio) -> [f64; PUNTI] {
    let pesi: Vec<f64> = (dati.elettorato().regioni.iter())
        .map(|r| r.elettori as f64)
        .collect();
    let media = media_pesata(&pesi, s.regioni.iter().map(|i| &i.posizione[..]), PUNTI);
    let mut h = [0.0; PUNTI];
    h.iter_mut().zip(media).for_each(|(a, x)| *a = x);
    h
}

impl Partita {
    /// La partita vera: il partito del giocatore in coda ai partiti dello scenario, il governo
    /// uscente dello scenario, Camere sciolte e voto nella chiusura del turno zero.
    ///
    /// # Errors
    /// `Fondazione` col nome vuoto o più lungo di 40 caratteri, con la posizione non valida (non
    /// finita), o con la posizione che non è una di `posizioni`.
    pub fn fonda(dati: DatiGioco, seme: u64, f: &Fondazione) -> Result<Partita, Errore> {
        let nome = f.nome.trim();
        if nome.is_empty() {
            return ko("nome del partito vuoto".into());
        }
        if nome.chars().count() > LUNGHEZZA_NOME {
            return ko(format!(
                "nome del partito più lungo di {LUNGHEZZA_NOME} caratteri"
            ));
        }
        if !f.posizione.is_finite() {
            return ko("posizione non valida".into());
        }
        let ammessi = posizioni(&dati);
        let Some(&posizione) = ammessi.iter().find(|x| (*x - f.posizione).abs() <= 1e-9) else {
            let virgola = |x: &f64| x.to_string().replace('.', ",");
            let elenco: Vec<String> = ammessi.iter().map(virgola).collect();
            return ko(format!(
                "posizione {} non ammessa: scegli fra {}",
                virgola(&f.posizione),
                elenco.join("; ")
            ));
        };
        let partito = partito_nuovo(&dati, nome, posizione);
        Ok(Partita::costruisci(dati, seme, Some(partito)))
    }

    /// Per ogni posizione, la quota attesa sul sondaggio del turno zero della partita che `fonda`
    /// farebbe con `seme`: stesso seme, stesso sondaggio. Non tocca nessuna partita.
    pub fn anteprima_fondazione(dati: &DatiGioco, seme: u64) -> AnteprimaFondazione {
        let mut anteprima = AnteprimaFondazione {
            istogramma: [0.0; PUNTI],
            posizioni: Vec::new(),
        };
        for posizione in posizioni(dati) {
            // ponytail: una partita intera per posizione invece del solo sondaggio, così è per
            // costruzione quello di `fonda`; se pesa nella schermata, costruire solo elettorato e
            // informazione.
            let nuovo = partito_nuovo(dati, SEGNAPOSTO, posizione);
            let p = Partita::costruisci(dati.clone(), seme, Some(nuovo));
            let s = &p.informazione().sondaggio;
            // Le autocollocazioni non dipendono dalla posizione: stesse estrazioni, stesso istogramma.
            anteprima.istogramma = istogramma(dati, s);
            anteprima.posizioni.push(VocePosizione {
                posizione,
                quota_attesa: quota_fra_i_votanti(s, p.giocatore()),
                disponibilita: p.disponibilita(),
            });
        }
        anteprima
    }
}
