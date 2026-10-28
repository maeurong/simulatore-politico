//! I partiti IA: il registro delle mosse, il cambio di leader dopo una sconfitta e il governo
//! avversario che governa in apertura, affronta la crisi e va al voto anticipato.

use super::Partita;
use crate::caso::Flusso;
use crate::comando::Comando;
use crate::economia::{self, Bilancio};
use crate::informazione::indice;
use crate::legislatura::{self, Fase};
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
    /// il giocatore; nel turno di crisi, fuori dal turno del voto, affronta la crisi.
    pub(super) fn apertura_ia(&mut self) {
        if self.finita() || self.legislatura.fase != Fase::Ordinaria {
            return;
        }
        if self.governo.presidente != self.giocatore {
            if self.in_crisi_ia() {
                self.crisi_ia();
            }
            self.governo_ia();
        }
    }

    /// Il governo IA nel turno di crisi (scelta 12 del piano, formazione-di-coalizioni:13). In
    /// esercizio provvisorio si dimette subito: il governo cade comunque a fine turno
    /// (iter-legislativo-italiano:86). Altrimenti il governo IA guarda la fila dei partiti con
    /// seggi per posizione dichiarata, senza il giocatore, e i due vicini della coalizione. Allarga
    /// a quello che le ridà la maggioranza in entrambe le Camere e ci guadagna, con l'intesa non
    /// sotto `soglia_ultimatum` (l'alleato appena uscito resta fuori); fra due, il più vicino di
    /// posizione al membro accanto, a parità il più grande. Senza nessuno si dimette
    /// (iter-legislativo-italiano:42).
    pub(super) fn crisi_ia(&mut self) {
        let pc = self.governo.presidente;
        if self.economia.esercizio_provvisorio(self.data) {
            self.legislatura.dimissioni = true;
            let testo = "Si dimette: in esercizio provvisorio non allarga la coalizione";
            self.mossa(pc, testo.into());
            return;
        }
        let scelto = {
            let pl = &self.parlamento;
            let tutti = self.seggi_per_partito();
            let seggi = |j: usize| tutti.get(j).copied().unwrap_or(0);
            let pos = |j: usize| self.partiti.get(j).map_or(0.0, |p| p.posizione_dichiarata);
            let mut fila: Vec<usize> = (0..self.partiti.len())
                .filter(|&j| seggi(j) > 0 && j != self.giocatore)
                .collect();
            fila.sort_by(|&a, &b| pos(a).total_cmp(&pos(b)).then(a.cmp(&b)));
            let coalizione = &self.governo.coalizione;
            let dentro = |i: &usize| fila.get(*i).is_some_and(|j| coalizione.contains(j));
            let primo = (0..fila.len()).find(dentro);
            let ultimo = (0..fila.len()).rfind(dentro);
            // (candidato, il membro accanto a lui nella fila)
            let vicini = [
                primo.and_then(|i| Some((*fila.get(i.checked_sub(1)?)?, *fila.get(i)?))),
                ultimo.and_then(|i| Some((*fila.get(i + 1)?, *fila.get(i)?))),
            ];
            let q = self.quadro_veri();
            // Sotto `soglia_ultimatum`: l'alleato uscito sotto `soglia_uscita` non ci torna col
            // ritorno dell'intesa di un turno.
            let soglia = self.dati.manopola("soglia_ultimatum");
            // ponytail: il valore fuori dal governo è 0, non il termine dei seggi che il partito
            // tiene comunque all'opposizione; il guadagno ne è sovrastimato. Via d'uscita: un
            // `valore_all_opposizione` in `legislatura/crisi.rs`.
            (vicini.into_iter().flatten())
                .filter(|&(j, _)| {
                    let mut con = coalizione.clone();
                    con.push(j);
                    !self.intesa.valori.get(j).is_some_and(|&x| x < soglia)
                        && legislatura::maggioranza(&pl.seggi_camera, &pl.seggi_senato, &con)
                        && legislatura::valore_al_governo(&q, j, &con, false) > 0.0
                })
                .min_by(|&(a, x), &(b, y)| {
                    ((pos(a) - pos(x)).abs())
                        .total_cmp(&(pos(b) - pos(y)).abs())
                        .then(seggi(b).cmp(&seggi(a)))
                })
                .map(|(j, _)| j)
        };
        match scelto.and_then(|j| Some((j, self.partiti.get(j)?.nome.clone()))) {
            Some((j, nome)) => {
                self.legislatura.allargati.push(j);
                self.mossa(pc, format!("Allarga la coalizione a {nome}"));
            }
            None => {
                self.legislatura.dimissioni = true;
                let testo = "Si dimette: nessun partito contiguo gli ridà la maggioranza con un \
                             guadagno";
                self.mossa(pc, testo.into());
            }
        }
    }

    /// Il voto anticipato si decide: governo IA in carica, fase ordinaria, dal quinto turno della
    /// legislatura (`inizio + 4`) a due turni dalla scadenza, fuori dalla crisi (anche quella
    /// aperta per il turno dopo) e senza dimissioni in corso.
    fn finestra_voto_anticipato(&self) -> bool {
        let l = &self.legislatura;
        !self.finita()
            && l.fase == Fase::Ordinaria
            && self.governo.presidente != self.giocatore
            && self.turno >= l.inizio.saturating_add(4)
            && l.scadenza.saturating_sub(self.turno) >= 2
            && l.crisi.is_none()
            && !l.dimissioni
    }

    /// In chiusura, dopo il sondaggio (scelta 13 del piano): nella finestra, il governo IA si
    /// dimette per andare al voto se il valore atteso del voto per il partito del Presidente del
    /// Consiglio (`Quadro::valore` sul sondaggio pubblico, coalizioni dell'ultimo voto) supera di
    /// `margine_voto_anticipato` quello del restare. Il restare è `valore_al_governo` coi seggi
    /// veri, con la guida: il termine `valore_guida` che spetta a chi presiede il governo.
    pub(super) fn voto_anticipato(&mut self) {
        if !self.finestra_voto_anticipato() {
            return;
        }
        let pc = self.governo.presidente;
        let voto = self.quadro().valore(&self.dichiarazione_corrente(), pc);
        let restare =
            legislatura::valore_al_governo(&self.quadro_veri(), pc, &self.governo.coalizione, true);
        if voto > restare + self.dati.manopola("margine_voto_anticipato") {
            self.legislatura.dimissioni = true;
            self.legislatura.voto_anticipato = true;
            let testo = "Si dimette puntando al voto: nei sondaggi il voto gli vale più che \
                         restare al governo";
            self.mossa(pc, testo.into());
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
            && let Some((bilancio, _, testo)) = self.bilancio_ia()
            && self.applica(Comando::PresentaBilancio { bilancio }).is_ok()
        {
            self.mossa(pc, testo);
        }
        // Col calendario pieno le voci che restano sono rifiutate una a una, senza righe.
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
            // Scelta 9: il costo della norma lontana si sa già, dalla posizione del gradino; lo
            // paga all'approvazione, e un emendamento accolto può solo cambiarlo.
            let nuova = parlamento::posizioni(&self.dati, &self.disposizioni, &[(k, g)]);
            if let Some((_, nuova)) = nuova
                && (self.alleati().into_iter()).any(|j| {
                    (self.costo_norma_lontana(j, nuova)).is_some_and(|c| self.sotto_uscita(j, c))
                })
            {
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
                self.mossa(pc, format!("Accoglie la proposta del dicastero {nome}"));
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
                Some(format!("la sua intesa è sotto {soglia:.0}"))
            } else if self.sotto_uscita(j, self.costo_respinto(j)) {
                let uscita = self.dati.manopola("soglia_uscita");
                Some(format!(
                    "respingerlo porterebbe la sua intesa sotto {uscita:.0}"
                ))
            } else if self.in_bilico(misura, Some(j)) {
                Some(format!(
                    "senza i voti di {nome} la legge rischia di non passare"
                ))
            } else {
                None
            };
            let accogli = motivo.is_some();
            let c = Comando::Emendamento {
                misura,
                partito,
                accogli,
            };
            // La legge col nome del calendario dopo la decisione (l'accolto ne cambia i gradini).
            if self.applica(c).is_ok()
                && let Some(m) = self.parlamento.misure.iter().find(|m| m.id == misura)
            {
                let legge = parlamento::nome_misura(&self.dati, m);
                let testo = match motivo {
                    Some(m) => format!("Accoglie l'emendamento di {nome} su {legge}: {m}"),
                    None => format!("Respinge l'emendamento di {nome} su {legge}"),
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
            let Some(m) = self.parlamento.misure.iter().find(|m| m.id == misura) else {
                continue;
            };
            let nome = parlamento::nome_misura(&self.dati, m);
            // Scelta 9: niente fiducia che porti un alleato sotto `soglia_uscita`.
            let regge = (self.costi_fiducia(m).into_iter()).all(|(j, c)| !self.sotto_uscita(j, c));
            if regge
                && self.in_bilico(misura, None)
                && self.applica(Comando::PoniFiducia { misura }).is_ok()
            {
                let testo = format!(
                    "Pone la questione di fiducia su {nome}: la forbice include la sconfitta"
                );
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

    /// Il bilancio del governo IA, la quota e la riga (scelta 10 del piano): la bozza coi budget
    /// dei dicasteri indicizzati della quota più alta che tiene la crescita della spesa netta
    /// entro il tetto UE (bisezione in 20 passi; quota 0 se nemmeno la bozza ci sta), poi i
    /// dicasteri degli alleati in ultimatum indicizzati per intero, anche oltre il tetto. La riga
    /// dice entro od oltre il tetto sul bilancio finale; niente, se il riepilogo non si calcola.
    /// La quota serve solo ai test. `None` senza bozza o se l'indicizzazione fallisce.
    pub(super) fn bilancio_ia(&self) -> Option<(Bilancio, f64, String)> {
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
            .filter(|&j| self.in_ultimatum(j))
            .collect();
        let dicasteri_di = |j: usize| {
            (self.governo.dicasteri.iter().enumerate())
                .filter(move |&(_, &x)| x == j)
                .map(|(k, _)| k)
        };
        let k: Vec<usize> = richiedenti.iter().flat_map(|&j| dicasteri_di(j)).collect();
        // Solo gli alleati le cui voci l'indicizzazione piena cambia davvero.
        let mut nomi = Vec::new();
        if !k.is_empty() {
            let pieno = self.indicizza_dicasteri(&b, &k).ok()?;
            let spesa = |x: &Bilancio, k: usize| x.dicasteri.get(k).map(|d| d.spesa);
            for &j in &richiedenti {
                if dicasteri_di(j).any(|k| spesa(&b, k) != spesa(&pieno, k))
                    && let Some(p) = self.partiti.get(j)
                {
                    nomi.push(p.nome.as_str());
                }
            }
            b = pieno;
        }
        let percento = (quota * 100.0).round() as u32;
        let mut testo = format!("Bilancio {}: {}", b.anno, indicizzati(percento));
        if let Ok(r) = self.riepilogo_ammesso(&b) {
            testo.push_str(if r.crescita_spesa_netta <= r.tetto {
                ", entro il tetto UE"
            } else {
                ", oltre il tetto UE"
            });
        }
        if !nomi.is_empty() {
            testo.push_str(&format!(", con le richieste di {}", nomi.join(", ")));
        }
        Some((b, quota, testo))
    }
}

/// «budget indicizzati per il {percento}% dell'inflazione», con l'articolo eliso davanti ai
/// numeri che si leggono con una vocale (l'1%, l'8%, l'11%, l'80-89%); a 0, «budget non
/// indicizzati».
pub(super) fn indicizzati(percento: u32) -> String {
    match percento {
        0 => "budget non indicizzati".into(),
        1 | 8 | 11 | 80..=89 => format!("budget indicizzati per l'{percento}% dell'inflazione"),
        _ => format!("budget indicizzati per il {percento}% dell'inflazione"),
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod tests {
    use super::super::tests::all_opposizione;
    use super::*;
    use crate::partiti::Vocazione;

    const AL_VOTO: &str =
        "Si dimette puntando al voto: nei sondaggi il voto gli vale più che restare al governo";

    /// Uscita la Destra (5), in crisi: la coalizione 1-4 non ha la maggioranza al Senato (99 su
    /// 200). Vicini nella fila: l'estrema sinistra (0, a 1 dal membro accanto, governista, con
    /// 10 e 5 seggi tolti al giocatore) che gliela ridà, e la Destra, tenuta fuori con l'intesa
    /// sotto `soglia_ultimatum`.
    fn in_crisi_senza_destra() -> Partita {
        let mut p = all_opposizione();
        p.parlamento.seggi_camera = vec![10, 125, 26, 18, 33, 125, 30, 33];
        p.parlamento.seggi_senato = vec![5, 69, 10, 7, 13, 65, 13, 18];
        p.governo.coalizione.retain(|&j| j != 5);
        p.partiti[0].vocazione = Vocazione::Governista;
        p.intesa.valori[5] = 30.0;
        p.legislatura.crisi = Some(p.turno);
        p.mosse.clear();
        p
    }

    #[test]
    fn in_crisi_allarga_solo_al_vicino_che_rida_la_maggioranza_e_ci_guadagna() {
        let mut p = in_crisi_senza_destra();
        p.crisi_ia();
        assert_eq!(p.legislatura.allargati, [0]);
        // Senza la maggioranza al Senato con lui (99 + 1).
        let mut p = in_crisi_senza_destra();
        p.parlamento.seggi_senato[0] = 1;
        p.parlamento.seggi_senato[7] += 4;
        p.crisi_ia();
        assert!(p.legislatura.allargati.is_empty() && p.legislatura.dimissioni);
        // Identitaria, la distanza dalla coalizione le toglie il guadagno.
        let mut p = in_crisi_senza_destra();
        p.partiti[0].vocazione = Vocazione::Identitaria;
        p.crisi_ia();
        assert!(p.legislatura.allargati.is_empty() && p.legislatura.dimissioni);
        let riga = "Si dimette: nessun partito contiguo gli ridà la maggioranza con un guadagno";
        assert!(p.mosse.iter().any(|m| m.testo == riga), "{:?}", p.mosse);
    }

    /// Con la Destra di nuovo ammessa, entrambi i vicini ridanno la maggioranza: a distanza 1 dal
    /// membro accanto tutti e due, vince la Destra, più grande; l'estrema sinistra a 0,5 vince.
    #[test]
    fn in_crisi_allarga_al_piu_vicino_e_a_parita_al_piu_grande() {
        let mut p = in_crisi_senza_destra();
        p.intesa.valori[5] = 50.0;
        p.crisi_ia();
        assert_eq!(p.legislatura.allargati, [5]);
        let mut p = in_crisi_senza_destra();
        p.intesa.valori[5] = 50.0;
        p.partiti[0].posizione_dichiarata = 1.5;
        p.crisi_ia();
        assert_eq!(p.legislatura.allargati, [0]);
    }

    /// Nel turno del voto la crisi la chiude il voto: `apertura_ia` non la affronta.
    #[test]
    fn nel_turno_del_voto_la_crisi_ia_non_agisce() {
        let mut p = in_crisi_senza_destra();
        p.turno = p.legislatura.scadenza;
        p.legislatura.crisi = Some(p.turno);
        assert!(p.legislatura.turno_di_voto(p.turno));
        p.apertura_ia();
        assert!(p.legislatura.allargati.is_empty() && !p.legislatura.dimissioni);
        let mut p = in_crisi_senza_destra();
        p.apertura_ia();
        assert_eq!(p.legislatura.allargati, [0]);
    }

    #[test]
    fn l_articolo_della_quota_si_elide_davanti_a_vocale() {
        assert_eq!(indicizzati(0), "budget non indicizzati");
        for (q, attesa) in [
            (8, "per l'8%"),
            (11, "per l'11%"),
            (85, "per l'85%"),
            (50, "per il 50%"),
        ] {
            assert_eq!(
                indicizzati(q),
                format!("budget indicizzati {attesa} dell'inflazione")
            );
        }
    }

    #[test]
    fn niente_voto_anticipato_fuori_finestra() {
        let mut p = all_opposizione();
        let (inizio, scadenza) = (p.legislatura.inizio, p.legislatura.scadenza);
        let mut a = |turno: u32, f: fn(&mut Partita)| {
            p.turno = turno;
            f(&mut p);
            let si = p.finestra_voto_anticipato();
            p.legislatura.crisi = None;
            p.legislatura.dimissioni = false;
            p.legislatura.fase = Fase::Ordinaria;
            si
        };
        assert!(!a(inizio + 3, |_| {}));
        assert!(a(inizio + 4, |_| {}));
        assert!(a(scadenza - 2, |_| {}));
        assert!(!a(scadenza - 1, |_| {}));
        assert!(!a(scadenza, |_| {}));
        assert!(!a(inizio + 4, |p| p.legislatura.crisi = Some(p.turno)));
        assert!(!a(inizio + 4, |p| p.legislatura.dimissioni = true));
        assert!(!a(inizio + 4, |p| p.legislatura.fase = Fase::Sciolta));
        assert!(!a(inizio + 4, |p| p.governo.presidente = p.giocatore));
    }

    #[test]
    fn in_crisi_riprende_l_uscente_solo_da_soglia_ultimatum() {
        let mut p = all_opposizione();
        let destra = p.partiti.iter().position(|x| x.id == "destra").unwrap();
        p.intesa.valori[destra] = 10.0;
        p.chiudi_turno();
        assert!(p.in_crisi() && p.legislatura.dimissioni);
        assert!(p.legislatura.allargati.is_empty());
        let soglia = p.dati.manopola("soglia_ultimatum");
        p.legislatura.dimissioni = false;
        p.intesa.valori[destra] = soglia - 1.0;
        p.crisi_ia();
        assert!(p.legislatura.dimissioni && p.legislatura.allargati.is_empty());
        p.legislatura.dimissioni = false;
        p.intesa.valori[destra] = soglia;
        p.crisi_ia();
        assert_eq!(p.legislatura.allargati, [destra]);
        assert!(
            p.mosse
                .iter()
                .any(|m| m.testo == "Allarga la coalizione a Destra")
        );
        // `carica` ammette gli allargati del governo IA in crisi.
        assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    }

    #[test]
    fn si_va_al_voto_quando_conviene() {
        let mut p = all_opposizione();
        let pc = p.governo.presidente;
        while p.turno < p.legislatura.inizio + 6 {
            p.chiudi_turno();
        }
        assert_eq!(p.governo.presidente, pc);
        assert_eq!(p.legislatura.fase, Fase::Ordinaria);
        let mut resta = Partita::carica(&p.salva().unwrap()).unwrap();
        resta.chiudi_turno();
        assert!(resta.mosse.iter().all(|m| m.testo != AL_VOTO));
        assert_eq!(resta.legislatura.fase, Fase::Ordinaria);
        assert_eq!(resta.governo.presidente, pc);
        for (j, x) in p.partiti.iter_mut().enumerate() {
            x.valence = if j == pc { 3.0 } else { -3.0 };
        }
        // Gli alleati crollano nei sondaggi: con l'intesa piena non escono, e la crisi non c'è.
        p.intesa.valori.fill(100.0);
        let salvata = p.salva().unwrap();
        // Voto − restare in questa chiusura: circa 4,2 (misurato col seme 7). Margini lontani da
        // entrambi i lati, perché il test regga ai ritocchi del motore.
        let chiudi = |margine: f64| {
            let mut q = Partita::carica(&salvata).unwrap();
            q.dati.imposta_manopola("margine_voto_anticipato", margine);
            q.chiudi_turno();
            q
        };
        let q = chiudi(8.0);
        assert!(q.mosse.iter().all(|m| m.testo != AL_VOTO));
        assert_eq!(q.legislatura.fase, Fase::Ordinaria);
        let p = chiudi(0.0);
        let id = &p.partiti[pc].id;
        assert!(
            p.mosse
                .iter()
                .any(|m| m.testo == AL_VOTO && &m.partito == id)
        );
        // Dimissioni e voto anticipato li consuma il Capo dello Stato nella stessa chiusura.
        assert!(!p.legislatura.dimissioni && !p.legislatura.voto_anticipato);
        assert_ne!(p.legislatura.fase, Fase::Ordinaria);
    }
}
