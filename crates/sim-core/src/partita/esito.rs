//! Le voci dell'esito scritte dalla partita, nel punto del fatto.

use super::Partita;
use crate::esito::{TipoVoce, VoceEsito, grado};

impl Partita {
    /// La voce nasce col grado del catalogo per chi governa ora; `None` se il catalogo dice «–».
    /// «Al governo» è `al_governo`: Presidente del Consiglio, anche da uscente in trattativa,
    /// perché i suoi ministri eseguono ancora (Scelta 22 della spec); la fermata sul governo
    /// guarda invece `governa`.
    pub(super) fn voce(
        &mut self,
        tipo: TipoVoce,
        fatto: String,
        causa: String,
    ) -> Option<&mut VoceEsito> {
        let g = grado(tipo, self.al_governo())?;
        self.esito.push(VoceEsito::nuova(tipo, g, fatto, causa));
        self.esito.last_mut()
    }

    /// Come `voce`, col partito di indice `j` nel campo `partito`: l'ingresso delle 25 ex mosse.
    /// Nessuna voce se `j` non è un partito.
    pub(super) fn voce_di(
        &mut self,
        j: usize,
        tipo: TipoVoce,
        fatto: String,
        causa: String,
    ) -> Option<&mut VoceEsito> {
        let id = self.partiti.get(j)?.id.clone();
        let v = self.voce(tipo, fatto, causa)?;
        v.partito = Some(id);
        Some(v)
    }

    /// Il giocatore è Presidente del Consiglio (Scelta 7).
    pub(super) fn al_governo(&self) -> bool {
        self.governo.presidente == self.giocatore
    }
}
