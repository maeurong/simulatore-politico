//! La vita dei partiti IA: lo scioglimento dopo due elezioni senza seggi (scelta 9 del piano 8b,
//! vita-dei-partiti:120).

use super::Partita;
use crate::legislatura::Incarico;

/// Elezioni di fila senza seggi dopo le quali un partito IA si scioglie.
pub(super) const ELEZIONI_SENZA_SEGGI: u8 = 2;

impl Partita {
    /// In coda al voto: un partito IA attivo senza seggi in entrambe le Camere conta un'elezione in
    /// più, altrimenti il conto riparte; a `ELEZIONI_SENZA_SEGGI` si scioglie, con la sua riga. Lo
    /// sciolto esce dalle coalizioni dell'ultimo voto (quella che resta di un partito solo cade) e
    /// dall'incarico IA. Il giocatore no: senza seggi la partita finisce già.
    pub(super) fn scioglimenti(&mut self) {
        let (camera, senato) = (&self.parlamento.seggi_camera, &self.parlamento.seggi_senato);
        let vuoto = |v: &[u32], j: usize| v.get(j).is_none_or(|&x| x == 0);
        let senza: Vec<bool> = (0..self.partiti.len())
            .map(|j| vuoto(camera, j) && vuoto(senato, j))
            .collect();
        let mut sciolti = Vec::new();
        for (j, p) in self.partiti.iter_mut().enumerate() {
            if p.sciolto || j == self.giocatore {
                continue;
            }
            p.senza_seggi = if senza[j] {
                p.senza_seggi.saturating_add(1)
            } else {
                0
            };
            if p.senza_seggi >= ELEZIONI_SENZA_SEGGI {
                p.sciolto = true;
                sciolti.push(j);
            }
        }
        for &j in &sciolti {
            let testo = format!("Si scioglie dopo {ELEZIONI_SENZA_SEGGI} elezioni senza seggi");
            self.mossa(j, testo);
        }
        let l = &mut self.legislatura;
        // Una coalizione ridotta a un solo membro dallo scioglimento cade.
        l.coalizioni.retain_mut(|c| {
            let prima = c.len();
            c.retain(|j| !sciolti.contains(j));
            c.len() == prima || c.len() >= 2
        });
        if let Some(Incarico::Ia { coalizione }) = &mut l.incarico {
            coalizione.retain(|j| !sciolti.contains(j));
        }
    }
}
