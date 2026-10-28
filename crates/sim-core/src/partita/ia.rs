//! I partiti IA: il registro delle mosse, il cambio di leader dopo una sconfitta e il governo
//! avversario che governa in apertura.

use super::Partita;
use crate::caso::Flusso;
use crate::comando::Comando;
use crate::economia::{self, Bilancio};
use crate::informazione::indice;
use crate::legislatura::Fase;
use crate::parlamento::{self, StatoEmendamento, ValoreProposto};
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

    /// In coda all'apertura del turno, a partita non finita e in fase ordinaria: le mosse dei
    /// partiti IA (scelta 8 del piano). Il governo IA governa se il Presidente del Consiglio non è
    /// il giocatore.
    pub(super) fn apertura_ia(&mut self) {
        if self.finita() || self.legislatura.fase != Fase::Ordinaria {
            return;
        }
        if self.governo.presidente != self.giocatore {
            self.governo_ia();
        }
    }

    /// Il governo IA, fuori dalla crisi, coi comandi del giocatore: bilancio, programma, proposte
    /// dei ministri, emendamenti, fiducia (scelte 9-11 del piano). Una mossa rifiutata dalle
    /// regole non cambia nulla e non lascia righe.
    pub(super) fn governo_ia(&mut self) {
        if self.in_crisi() {
            return;
        }
        let pc = self.governo.presidente;
        if (self.data.trimestre == 4 || self.economia.esercizio_provvisorio(self.data))
            && self.economia.bilancio_presentato.is_none()
            && let Some((bilancio, testo)) = self.bilancio_ia()
            && self.applica(Comando::PresentaBilancio { bilancio }).is_ok()
        {
            self.mossa(pc, testo);
        }
        // Col calendario pieno ogni voce che resta è rifiutata: provarle tutte è fermarsi lì.
        for (k, g) in self.governo.programma.clone() {
            let (Some(x), Some(v)) = (
                self.dati.disposizioni().get(k),
                self.disposizioni.voci.get(k),
            ) else {
                continue;
            };
            // Non ancora in vigore: né vigente, né in arrivo da una legge già approvata.
            let Some(gradino) = x.gradini.get(g) else {
                continue;
            };
            if v.in_arrivo.last().map_or(v.in_vigore, |a| a.gradino) == g {
                continue;
            }
            let valori = vec![ValoreProposto {
                disposizione: x.id.clone(),
                gradino: gradino.id.clone(),
            }];
            if self.applica(Comando::PresentaLegge { valori }).is_ok()
                && let Some(m) = self.parlamento.misure.last()
            {
                let testo = format!("Porta in aula: {}", parlamento::nome_misura(&self.dati, m));
                self.mossa(pc, testo);
            }
        }
        for i in 0..self.ministri.dicasteri.len() {
            let Some(d) = self.dati.dicasteri().get(i) else {
                continue;
            };
            if self.ministri.dicasteri[i].proposta.is_none() {
                continue;
            }
            let (id, nome) = (d.id.clone(), d.nome.clone());
            if self.applica(Comando::Accogli { dicastero: id }).is_ok() {
                self.mossa(pc, format!("Accoglie la proposta di {nome}"));
            }
        }
        let soglia = self.dati.manopola("soglia_ultimatum");
        let aperti: Vec<(u32, usize)> = (self.parlamento.misure.iter())
            .filter(|m| m.promotore.is_none() && m.voto == self.turno)
            .flat_map(|m| {
                (m.emendamenti.iter())
                    .filter(|e| e.stato == StatoEmendamento::Aperto)
                    .map(|e| (m.id, e.partito))
            })
            .collect();
        for (misura, j) in aperti {
            let Some(p) = self.partiti.get(j) else {
                continue;
            };
            let (partito, nome) = (p.id.clone(), p.nome.clone());
            let motivo = if self.intesa.valori.get(j).is_some_and(|&x| x < soglia) {
                Some(format!("l'intesa di {nome} è sotto {soglia:.0}"))
            } else if self.in_bilico(misura, Some(j)) {
                Some(format!("senza i voti di {nome} la legge rischia"))
            } else {
                None
            };
            let accogli = motivo.is_some();
            let c = Comando::Emendamento {
                misura,
                partito,
                accogli,
            };
            if self.applica(c).is_ok() {
                let testo = match motivo {
                    Some(m) => format!("Accoglie l'emendamento di {nome}: {m}"),
                    None => format!("Respinge l'emendamento di {nome}"),
                };
                self.mossa(pc, testo);
            }
        }
        if self.alleati().iter().any(|&j| self.in_ultimatum(j)) {
            return;
        }
        let al_voto: Vec<u32> = (self.parlamento.misure.iter())
            .filter(|m| m.promotore.is_none() && m.voto == self.turno && !m.fiducia)
            .map(|m| m.id)
            .collect();
        for misura in al_voto {
            let nome = (self.parlamento.misure.iter())
                .find(|m| m.id == misura)
                .map(|m| parlamento::nome_misura(&self.dati, m));
            if let Some(nome) = nome
                && self.in_bilico(misura, None)
                && self.applica(Comando::PoniFiducia { misura }).is_ok()
            {
                let testo = format!("Pone la fiducia su {nome}: la forbice include la sconfitta");
                self.mossa(pc, testo);
            }
        }
    }

    /// L'alleato `j` ha un ultimatum in corso.
    fn in_ultimatum(&self, j: usize) -> bool {
        self.intesa.ultimatum.get(j).is_some_and(Option::is_some)
    }

    /// La forbice della misura `misura` include la sconfitta, con la linea di `senza` a no.
    fn in_bilico(&self, misura: u32, senza: Option<usize>) -> bool {
        let Some(m) = self.parlamento.misure.iter().find(|m| m.id == misura) else {
            return false;
        };
        let mut p = self.stato_aula().previsione(m);
        if let Some(si) = senza.and_then(|j| p.linea.get_mut(j)) {
            *si = false;
        }
        self.parlamento.forbice(&p).in_bilico
    }

    /// Il bilancio del governo IA e la sua riga (scelta 10 del piano): la bozza coi budget dei
    /// dicasteri indicizzati della quota più alta che tiene la crescita della spesa netta entro
    /// il tetto UE (bisezione in 20 passi; quota 0 se nemmeno la bozza ci sta), poi i dicasteri
    /// degli alleati in ultimatum indicizzati per intero, anche oltre il tetto. `None` senza bozza.
    fn bilancio_ia(&self) -> Option<(Bilancio, String)> {
        let bozza = self.bozza_bilancio()?;
        let indice = self.indice_prezzi();
        let con = |q: f64| economia::indicizza_in_parte(&bozza, q, indice).ok();
        // Senza spesa netta di riferimento il tetto non si misura: nessun limite.
        let entro = |q: f64| {
            con(q).is_some_and(|b| {
                self.riepilogo_ammesso(&b)
                    .map_or(true, |r| r.crescita_spesa_netta <= r.tetto)
            })
        };
        let quota = if entro(1.0) {
            1.0
        } else {
            let (mut sotto, mut sopra) = (0.0, 1.0);
            for _ in 0..20 {
                let q = f64::midpoint(sotto, sopra);
                if entro(q) {
                    sotto = q;
                } else {
                    sopra = q;
                }
            }
            sotto
        };
        let mut b = con(quota)?;
        let richiedenti: Vec<usize> = (self.alleati().into_iter())
            .filter(|&j| self.in_ultimatum(j) && self.governo.dicasteri.contains(&j))
            .collect();
        let k: Vec<usize> = (self.governo.dicasteri.iter().enumerate())
            .filter(|&(_, j)| richiedenti.contains(j))
            .map(|(k, _)| k)
            .collect();
        if !k.is_empty() {
            b = self.indicizza_dicasteri(&b, &k).ok()?;
        }
        let mut testo = format!(
            "Bilancio {}: budget indicizzati al {:.0}% dell'inflazione, entro il tetto UE",
            b.anno,
            (quota * 100.0).round()
        );
        if !richiedenti.is_empty() {
            let nomi: Vec<&str> = (richiedenti.iter())
                .filter_map(|&j| self.partiti.get(j).map(|p| p.nome.as_str()))
                .collect();
            testo.push_str(&format!(", con le richieste di {}", nomi.join(", ")));
        }
        Some((b, testo))
    }
}
