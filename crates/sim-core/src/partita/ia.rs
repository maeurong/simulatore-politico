//! I partiti IA: il registro delle mosse e il cambio di leader dopo una sconfitta.

use super::Partita;
use crate::caso::Flusso;
use crate::informazione::indice;
use crate::vista::MossaIa;

impl Partita {
    /// Una riga nel registro del turno, col partito `j`.
    pub(super) fn mossa(&mut self, j: usize, testo: String) {
        if let Some(p) = self.partiti.get(j) {
            self.mosse.push(MossaIa {
                partito: p.id.clone(),
                testo,
            });
        }
    }

    /// Il partito IA `j` cambia leader: nome nuovo, diverso dall'uscente, estratto da
    /// `Flusso::PartitiIa`; la valence riparte da `valence_base` più la luna di miele. Mai sul
    /// partito del giocatore.
    pub(super) fn cambia_leader(&mut self, j: usize, causa: &str) {
        if j == self.giocatore {
            return;
        }
        let el = self.dati.elettorato();
        let uscente = self.partiti.get(j).map(|p| p.leader.clone());
        let mut nuovo = String::new();
        for _ in 0..100 {
            let n = &el.nomi[indice(self.caso.uniforme(Flusso::PartitiIa), el.nomi.len())];
            let c = &el.cognomi[indice(self.caso.uniforme(Flusso::PartitiIa), el.cognomi.len())];
            nuovo = format!("{n} {c}");
            if uscente.as_ref() != Some(&nuovo) {
                break;
            }
        }
        let luna = self.dati.manopola("luna_di_miele_leader");
        let Some(p) = self.partiti.get_mut(j) else {
            return;
        };
        p.leader.clone_from(&nuovo);
        p.valence = p.valence_base + luna;
        self.mossa(j, format!("Nuovo leader: {nuovo}, dopo {causa}"));
    }

    /// Nella chiusura del voto, dopo il ritorno della valence: ogni partito IA la cui quota fra i
    /// votanti è scesa di almeno `calo_cambio_leader` sull'elezione precedente cambia leader.
    /// Alla prima elezione non c'è un'elezione precedente.
    pub(super) fn leader_dopo_il_voto(&mut self) {
        let [esito, prima, ..] = self.legislatura.elezioni.as_slice() else {
            return;
        };
        let soglia = self.dati.manopola("calo_cambio_leader");
        let cali: Vec<(usize, f64)> = (prima.voti.iter().zip(&esito.voti))
            .enumerate()
            .map(|(j, (a, b))| (j, a.quota - b.quota))
            .filter(|&(_, calo)| calo >= soglia)
            .collect();
        for (j, calo) in cali {
            let punti = format!("{:.1}", calo * 100.0).replace('.', ",");
            self.cambia_leader(j, &format!("il calo di {punti} punti al voto"));
        }
    }
}
