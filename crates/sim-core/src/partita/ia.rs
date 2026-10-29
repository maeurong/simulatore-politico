//! I partiti IA: il registro delle mosse e il cambio di leader dopo una sconfitta.

use super::Partita;
use crate::caso::Flusso;
use crate::informazione::indice;
use crate::vista::MossaIa;

impl Partita {
    /// Una riga nel registro del turno, col partito `j`.
    fn mossa(&mut self, j: usize, testo: String) {
        if let Some(p) = self.partiti.get(j) {
            self.mosse.push(MossaIa {
                partito: p.id.clone(),
                testo,
            });
        }
    }

    /// Il partito IA `j` cambia leader: nome nuovo estratto da `Flusso::PartitiIa`; la valence
    /// riparte da `valence_base` più la luna di miele. Mai sul partito del giocatore. Se in 100
    /// estrazioni il nome esce sempre uguale all'uscente, niente cambio e niente mossa.
    pub(super) fn cambia_leader(&mut self, j: usize, causa: &str) {
        if j == self.giocatore {
            return;
        }
        let el = self.dati.elettorato();
        let uscente = self.partiti.get(j).map(|p| p.leader.clone());
        let mut nuovo = None;
        for _ in 0..100 {
            let n = &el.nomi[indice(self.caso.uniforme(Flusso::PartitiIa), el.nomi.len())];
            let c = &el.cognomi[indice(self.caso.uniforme(Flusso::PartitiIa), el.cognomi.len())];
            let candidato = format!("{n} {c}");
            if uscente.as_ref() != Some(&candidato) {
                nuovo = Some(candidato);
                break;
            }
        }
        let Some(nuovo) = nuovo else {
            return;
        };
        let luna = self.dati.manopola("luna_di_miele_leader");
        let Some(p) = self.partiti.get_mut(j) else {
            return;
        };
        p.leader.clone_from(&nuovo);
        p.valence = p.valence_base + luna;
        self.mossa(j, format!("Nuovo leader: {nuovo}, dopo {causa}"));
    }

    /// Il calo di quota fra i votanti del partito `j` sull'elezione precedente, se arriva a
    /// `calo_cambio_leader`. Alla prima elezione non c'è un'elezione precedente.
    pub(super) fn calo(&self, j: usize) -> Option<f64> {
        let [esito, prima, ..] = self.legislatura.elezioni.as_slice() else {
            return None;
        };
        // ponytail: i voti si abbinano per posizione, valido finché i partiti non cambiano fra
        // due voti (l'8b li fa cambiare: lì si abbina per id).
        let calo = prima.voti.get(j)?.quota - esito.voti.get(j)?.quota;
        (calo >= self.dati.manopola("calo_cambio_leader")).then_some(calo)
    }

    /// Nella chiusura del voto, dopo il ritorno della valence: ogni partito IA con un `calo`
    /// cambia leader.
    pub(super) fn leader_dopo_il_voto(&mut self) {
        for j in 0..self.partiti.len() {
            if let Some(calo) = self.calo(j) {
                let punti = format!("{:.1}", calo * 100.0).replace('.', ",");
                self.cambia_leader(j, &format!("il calo di {punti} punti percentuali al voto"));
            }
        }
    }
}
