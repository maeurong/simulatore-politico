//! I partiti IA: il registro delle mosse, il cambio di leader dopo una sconfitta, il governo
//! avversario che governa in apertura, affronta la crisi e va al voto anticipato, e l'opposizione
//! che presenta la sfiducia, porta la proposta nel posto riservato e chiede il referendum.

use super::Partita;
use crate::caso::Flusso;
use crate::comando::Comando;
use crate::disposizioni::Norma;
use crate::economia::{self, Bilancio};
use crate::elettorato::{Affluenza, stima};
use crate::informazione::indice;
use crate::legislatura::{self, Fase};
use crate::parlamento::{self, StatoEmendamento, ValoreProposto, voto};
use crate::vista::MossaIa;
use std::cmp::Reverse;

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
    /// il giocatore; nel turno di crisi, fuori dal turno del voto, affronta la crisi. Poi
    /// l'opposizione IA, chiunque governi.
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
        self.opposizione_ia();
    }

    /// L'opposizione IA in apertura, anche contro un governo IA (decisione 3 di Mario).
    fn opposizione_ia(&mut self) {
        self.sfiducia_ia();
        self.proposta_ia();
    }

    /// La proposta-imbarazzo (scelta 16 del piano, iter-legislativo-italiano:125-127): col posto
    /// riservato del turno assegnato a un partito IA e ancora libero, fra le riforme ordinarie a
    /// una disposizione ammesse da `Parlamento::bozza_proposta` col guadagno del promotore
    /// positivo, quella che massimizza i seggi (Camera più Senato) dei partiti di governo con la
    /// linea sì, senza il giocatore se presiede lui: il suo voto lo decide lui. A parità il
    /// guadagno del promotore, poi l'indice della disposizione e del gradino. Nessun candidato,
    /// nessuna mossa; niente nel turno del voto, dove la proposta decadrebbe senza voto.
    fn proposta_ia(&mut self) {
        let Some(j) = self.posto_riservato().filter(|&j| j != self.giocatore) else {
            return;
        };
        if !self.parlamento.posto_riservato_libero(self.turno)
            || self.legislatura.turno_di_voto(self.turno)
        {
            return;
        }
        let aula = self.stato_aula();
        let pl = &self.parlamento;
        let seggi = self.seggi_per_partito();
        let presiede = self.governo.presidente == self.giocatore;
        let conta = |k: usize| self.governo.al_governo(k) && !(presiede && k == self.giocatore);
        let mut migliore: Option<(u32, f64, ValoreProposto)> = None;
        let ordinarie = (self.dati.disposizioni().iter()).filter(|x| x.norma == Norma::Ordinaria);
        for x in ordinarie {
            for g in &x.gradini {
                let v = ValoreProposto {
                    disposizione: x.id.clone(),
                    gradino: g.id.clone(),
                };
                let valori = std::slice::from_ref(&v);
                let (dati, disposizioni) = (&self.dati, &self.disposizioni);
                let Ok(m) = pl.bozza_proposta(dati, disposizioni, self.turno, j, valori) else {
                    continue;
                };
                let p = aula.previsione(&m);
                // Col NaN il confronto è falso: la disposizione resta fuori.
                let Some(guadagno) = (voto::guadagni(&self.partiti, p.posizioni).get(j))
                    .copied()
                    .filter(|&g| g > 0.0)
                else {
                    continue;
                };
                let n = (p.linea.iter().enumerate())
                    .filter(|&(k, &si)| si && conta(k))
                    .filter_map(|(k, _)| seggi.get(k))
                    .sum();
                if migliore
                    .as_ref()
                    .is_none_or(|&(s, gd, _)| n > s || (n == s && guadagno > gd))
                {
                    migliore = Some((n, guadagno, v));
                }
            }
        }
        let Some((n, _, v)) = migliore else {
            return;
        };
        let (dati, disposizioni, turno) = (&self.dati, &self.disposizioni, self.turno);
        if (self.parlamento)
            .presenta_proposta(dati, disposizioni, turno, j, &[v])
            .is_ok()
            && let Some(m) = self.parlamento.misure.last()
        {
            let nome = parlamento::nome_misura(&self.dati, m);
            let di = if presiede {
                "fra Camera e Senato dei partiti alleati"
            } else {
                "fra Camera e Senato dei partiti di governo"
            };
            let voterebbero = match n {
                0 => format!("nessun seggio {di} la voterebbe"),
                1 => format!("1 seggio {di} la voterebbe"),
                _ => format!("{n} seggi {di} la voterebbero"),
            };
            self.mossa(
                j,
                format!("Porta nel posto riservato {nome}; {voterebbero}"),
            );
        }
    }

    /// La sfiducia dell'opposizione IA (scelta 14 del piano): al governo se passerebbe sullo stato
    /// dell'apertura e la caduta non ridarebbe lo stesso governo (`rinascerebbe_uguale`);
    /// altrimenti, se passerebbe, al ministro del partito col giudizio medio peggiore fra i gruppi
    /// (pesato per elettori), a parità il dicastero di peso maggiore, mai di un alleato in
    /// ultimatum né in interim. Firme: i seggi alla Camera dei partiti IA fuori dal governo, almeno
    /// un decimo (iter-legislativo-italiano:43, :141-142). Promotore: il più grande di loro alla
    /// Camera, a parità l'indice più basso. Niente con una mozione già presentata, né nel turno di
    /// crisi, dove la sfiducia automatica è già sul tavolo, né nel turno del voto, che chiude
    /// comunque il governo.
    fn sfiducia_ia(&mut self) {
        if self.parlamento.sfiducia.is_some()
            || self.in_crisi()
            || self.legislatura.turno_di_voto(self.turno)
        {
            return;
        }
        let ia_fuori = |j: usize| j != self.giocatore && !self.governo.al_governo(j);
        let (firme, servono) = self.firme_sfiducia(ia_fuori);
        let promotore = (self.parlamento.seggi_camera.iter().enumerate())
            .filter(|&(j, _)| ia_fuori(j))
            .max_by_key(|&(j, &n)| (n, Reverse(j)))
            .map(|(j, _)| j);
        let Some(promotore) = promotore.filter(|_| firme >= servono) else {
            return;
        };
        let in_ultimatum = self.alleati_in_ultimatum();
        // Il motivo della riga: i partiti di governo che nella linea votano sì. Oggi sono solo gli
        // alleati in ultimatum (`linea_sfiducia`, legislatura/crisi.rs:22-27): il nome senza «in
        // ultimatum» non si raggiunge.
        let motivo = |dicastero: Option<usize>| {
            let (linea, _, _, approvata) = self.voto_sfiducia(dicastero)?;
            if !approvata {
                return None;
            }
            let nomi: Vec<String> = (0..self.partiti.len())
                .filter(|&j| self.governo.al_governo(j) && linea.get(j) == Some(&true))
                .filter_map(|j| {
                    let nome = &self.partiti.get(j)?.nome;
                    Some(if in_ultimatum.contains(&j) {
                        format!("{nome} in ultimatum")
                    } else {
                        nome.clone()
                    })
                })
                .collect();
            Some(match nomi.len() {
                0 => "passerebbe coi voti dell'opposizione".to_string(),
                1 => format!("{} la voterebbe", elenco(&nomi)),
                _ => format!("{} la voterebbero", elenco(&nomi)),
            })
        };
        let el = &self.elettorato;
        // ponytail: il giudizio è per partito, non per dicastero: i ministri di un partito
        // pareggiano e decide il peso. Via d'uscita: la colpa per dicastero registrata da
        // `Elettorato::giudizio`. Somma pesata, non media: il denominatore è lo stesso per tutti.
        let giudizio = |j: usize| -> f64 {
            (el.gruppi.iter().enumerate())
                .map(|(i, g)| {
                    el.elettori(self.dati.elettorato(), i)
                        * g.giudizio.get(j).copied().unwrap_or(0.0)
                })
                .sum()
        };
        let peso = |i: usize| {
            self.dati
                .dicasteri()
                .get(i)
                .map_or(0, |d| d.peso_spartizione)
        };
        let individuale = || {
            let (i, m) = (self.ministri.dicasteri.iter().enumerate())
                .filter_map(|(i, st)| Some((i, st.ministro.as_ref()?)))
                .filter(|(_, m)| !in_ultimatum.contains(&m.partito))
                .min_by(|&(a, x), &(b, y)| {
                    (giudizio(x.partito).total_cmp(&giudizio(y.partito)))
                        .then(peso(b).cmp(&peso(a)))
                })?;
            let nome = &self.dati.dicasteri().get(i)?.nome;
            let testo = format!(
                "Presenta la sfiducia a {}, {nome}: {}",
                m.nome,
                motivo(Some(i))?
            );
            Some((Some(i), testo))
        };
        let scelta = match motivo(None).filter(|_| !self.rinascerebbe_uguale()) {
            Some(m) => Some((None, format!("Presenta la sfiducia al governo: {m}"))),
            None => individuale(),
        };
        if let Some((dicastero, testo)) = scelta {
            self.parlamento.sfiducia = Some((dicastero, promotore));
            self.mossa(promotore, testo);
        }
    }

    /// La caduta ridarebbe lo stesso governo (formazione-di-coalizioni:13): la maggioranza contigua
    /// di `capo_dello_stato` è la coalizione di adesso, senza il giocatore, e il primo per seggi,
    /// che riceve l'incarico IA, è il Presidente del Consiglio. Col giocatore dentro l'incarico è
    /// suo o una `Guida`, sempre un governo diverso; senza maggioranza contigua si vota.
    fn rinascerebbe_uguale(&self) -> bool {
        let ordinata = |c: &[usize]| {
            let mut c = c.to_vec();
            c.sort_unstable();
            c
        };
        self.maggioranza_del_capo(false).is_some_and(|c| {
            !c.contains(&self.giocatore)
                && ordinata(&c) == ordinata(&self.governo.coalizione)
                && self.primo_per_seggi(&c) == Some(self.governo.presidente)
        })
    }

    /// L'abrogativo dell'opposizione IA (scelta 17 del piano), in aula dopo il voto dell'abrogativo
    /// in corso e prima delle firme del turno, solo se all'apertura lo slot era libero (scelta 8);
    /// solo in fase ordinaria e dentro le finestre di `promuovi_referendum`, controllate prima di
    /// ogni stima. Sulla `stima` del sondaggio, fra le disposizioni non elettorali che
    /// `Parlamento::abrogativo` ammette, quelle dove il sì vince e i votanti superano la metà degli
    /// aventi diritto; vince il margine sì − no più ampio, a parità l'indice minore. Promotore: il
    /// partito IA fuori dal governo, con seggi, che guadagna di più dall'abrogazione, a parità
    /// l'indice minore; nessuno ci guadagna, nessun referendum. La richiesta passa da
    /// `promuovi_referendum` (iter-legislativo-italiano:131-135).
    pub(super) fn referendum_ia(&mut self) {
        if self.legislatura.fase != Fase::Ordinaria || self.finestra_referendum().is_err() {
            return;
        }
        let pl = &self.parlamento;
        let seggi = self.seggi_per_partito();
        let ia_fuori: Vec<usize> = (0..self.partiti.len())
            .filter(|&j| {
                j != self.giocatore
                    && !self.governo.al_governo(j)
                    && seggi.get(j).is_some_and(|&n| n > 0)
            })
            .collect();
        let Some(sonda) = ia_fuori.first().and_then(|&j| self.partiti.get(j)) else {
            return;
        };
        let aula = self.stato_aula();
        let sondaggio = &self.informazione.sondaggio;
        let mut migliore: Option<(f64, Affluenza, usize, usize)> = None;
        let non_elettorali =
            (self.dati.disposizioni().iter()).filter(|x| x.norma != Norma::Elettorale);
        for x in non_elettorali {
            // Le regole non dipendono da quale partito fuori dal governo promuove.
            let Ok(r) = pl.abrogativo(&aula, self.turno, &sonda.id, &x.id) else {
                continue;
            };
            let Some(&(d, dopo)) = r.valori.first() else {
                continue;
            };
            let a = stima(&self.dati, sondaggio, &self.quesito_abrogativo(d, dopo));
            let margine = a.si - a.no;
            let quorum = (a.si + a.no) * 2.0 > a.si + a.no + a.astenuti;
            if margine > 0.0 && quorum && migliore.as_ref().is_none_or(|m| margine > m.0) {
                migliore = Some((margine, a, d, dopo));
            }
        }
        let Some((_, a, d, dopo)) = migliore else {
            return;
        };
        let posizioni = parlamento::posizioni(&self.dati, &self.disposizioni, &[(d, dopo)]);
        let guadagni = voto::guadagni(&self.partiti, posizioni);
        let guadagno = |j: usize| guadagni.get(j).copied().unwrap_or(0.0);
        let Some(promotore) = (ia_fuori.into_iter())
            .filter(|&j| guadagno(j) > 0.0)
            .max_by(|&i, &j| guadagno(i).total_cmp(&guadagno(j)).then(j.cmp(&i)))
        else {
            return;
        };
        let x = &self.dati.disposizioni()[d];
        let (nome, disposizione) = (x.nome.clone(), x.id.clone());
        let id = self.partiti[promotore].id.clone();
        if self.promuovi_referendum(&id, &disposizione).is_ok() {
            let percento = format!("{:.1}", a.si / (a.si + a.no) * 100.0).replace('.', ",");
            self.mossa(
                promotore,
                format!(
                    "Chiede il referendum su {nome}: secondo il sondaggio il sì vincerebbe col \
                     {percento}% dei voti e il quorum sarebbe raggiunto"
                ),
            );
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
                    (self.costo_norma_lontana(j, nuova)).is_some_and(|c| self.sotto_ultimatum(j, c))
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
            } else if self.sotto_ultimatum(j, self.costo_respinto(j)) {
                Some(format!(
                    "respingerlo porterebbe la sua intesa sotto {soglia:.0}"
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
            // Scelta 9: niente fiducia che porti un alleato sotto `soglia_ultimatum`.
            let regge =
                (self.costi_fiducia(m).into_iter()).all(|(j, c)| !self.sotto_ultimatum(j, c));
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

    /// Gli alleati con un ultimatum in corso.
    fn alleati_in_ultimatum(&self) -> Vec<usize> {
        (self.alleati().into_iter())
            .filter(|&j| self.in_ultimatum(j))
            .collect()
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
        let richiedenti = self.alleati_in_ultimatum();
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

/// I nomi in elenco: «A», «A e B», «A, B e C».
fn elenco(nomi: &[String]) -> String {
    match nomi {
        [primi @ .., ultimo] if !primi.is_empty() => format!("{} e {ultimo}", primi.join(", ")),
        _ => nomi.join(""),
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
    use crate::parlamento::CausaIntesa;
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

    /// Governo IA 1-5 (220 su 400 alla Camera), il centrodestra (4, 40 seggi) in ultimatum: con
    /// lui l'opposizione passa a 220, ma la caduta del governo non gli conviene. Giudizi azzerati.
    /// Ministri: 0 e 9 destra, 1 e 5 centrodestra, 2 e 6 centrosinistra, 3 e 8 centro, 4 e 7
    /// sinistra (pesi 1 e 2).
    fn sfiducia_al_limite() -> Partita {
        let mut p = all_opposizione();
        p.parlamento.seggi_camera = vec![40, 80, 20, 20, 40, 60, 60, 80];
        p.parlamento.seggi_senato = vec![10, 40, 10, 10, 20, 30, 30, 50];
        p.intesa.valori[4] = 38.0;
        p.intesa.ultimatum[4] = Some(CausaIntesa::Spartizione);
        for g in &mut p.elettorato.gruppi {
            g.giudizio.fill(0.0);
        }
        p.parlamento.sfiducia = None;
        p.mosse.clear();
        p
    }

    /// Il giudizio di `j` a `x` nel gruppo `i` (tutti con `None`).
    fn giudica(p: &mut Partita, i: Option<usize>, j: usize, x: f64) {
        for (k, g) in p.elettorato.gruppi.iter_mut().enumerate() {
            if i.is_none_or(|i| i == k) {
                g.giudizio[j] = x;
            }
        }
    }

    /// Pesato per elettori: la sinistra a −0,5 nel gruppo più grande è peggio del centrosinistra
    /// a −1 nel più piccolo. Dei due ministri della sinistra, quello di peso 2 (Interno, 7).
    #[test]
    fn la_sfiducia_individuale_va_al_giudizio_pesato_peggiore_e_al_peso_maggiore() {
        let mut p = sfiducia_al_limite();
        let el: Vec<f64> = (0..p.elettorato.gruppi.len())
            .map(|i| p.elettorato.elettori(p.dati.elettorato(), i))
            .collect();
        let grande = (0..el.len())
            .max_by(|&a, &b| el[a].total_cmp(&el[b]))
            .unwrap();
        let piccolo = (0..el.len())
            .min_by(|&a, &b| el[a].total_cmp(&el[b]))
            .unwrap();
        assert!(el[grande] * 0.5 > el[piccolo]);
        giudica(&mut p, Some(grande), 1, -0.5);
        giudica(&mut p, Some(piccolo), 2, -1.0);
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia, Some((Some(7), 6)));
    }

    /// Il centrodestra in ultimatum ha il giudizio peggiore: i suoi ministri restano fuori, la
    /// sfiducia va al centro, il successivo; a parità di peso (1 e 1) il dicastero d'indice minore.
    #[test]
    fn la_sfiducia_individuale_salta_l_alleato_in_ultimatum() {
        let mut p = sfiducia_al_limite();
        giudica(&mut p, None, 4, -10.0);
        giudica(&mut p, None, 3, -1.0);
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia, Some((Some(3), 6)));
    }

    /// Firme: l'estrema sinistra e l'estrema destra (35) restano sotto un decimo della Camera (40)
    /// anche se col giocatore lo superano. Con 40 la sfiducia al governo, che passa coi soli voti
    /// dell'opposizione (220 su 400), si presenta.
    #[test]
    fn firmano_solo_i_partiti_ia() {
        let mut p = sfiducia_al_limite();
        p.intesa.ultimatum[4] = None;
        p.parlamento.seggi_camera = vec![10, 60, 20, 20, 40, 40, 25, 185];
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia, None);
        p.parlamento.seggi_camera = vec![15, 60, 20, 20, 40, 40, 25, 180];
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia, Some((None, 6)));
    }

    /// Centro (3) e centrodestra (4) in ultimatum votano la sfiducia ai ministri degli altri: la
    /// riga li elenca entrambi, col plurale.
    #[test]
    fn la_riga_della_sfiducia_coi_due_alleati_in_ultimatum() {
        let mut p = sfiducia_al_limite();
        p.intesa.valori[3] = 38.0;
        p.intesa.ultimatum[3] = Some(CausaIntesa::Spartizione);
        p.sfiducia_ia();
        let Some((Some(i), _)) = p.parlamento.sfiducia else {
            panic!(
                "attesa la sfiducia individuale: {:?}",
                p.parlamento.sfiducia
            );
        };
        let ministro = &p.ministri.dicasteri[i].ministro.as_ref().unwrap().nome;
        let attesa = format!(
            "Presenta la sfiducia a {ministro}, {}: Centro in ultimatum e Centrodestra in \
             ultimatum la voterebbero",
            p.dati.dicasteri()[i].nome
        );
        assert_eq!(
            p.mosse.last().map(|m| m.testo.as_str()),
            Some(attesa.as_str())
        );
    }

    #[test]
    fn a_parita_di_seggi_promuove_l_indice_minore() {
        let mut p = sfiducia_al_limite();
        p.parlamento.seggi_camera = vec![50, 80, 20, 20, 40, 60, 50, 80];
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia.map(|(_, j)| j), Some(0));
    }

    /// Governo IA 3-5 presieduto dalla destra (5), la maggioranza contigua minima: 210 su 400 e
    /// 105 su 200, la destra prima per seggi. Il centrodestra (4) in ultimatum vota la caduta, che
    /// col Capo dello Stato ridarebbe lo stesso governo. I dicasteri delle sinistre alla destra.
    fn sfiducia_allo_stesso_governo() -> Partita {
        let mut p = sfiducia_al_limite();
        p.parlamento.seggi_camera = vec![30, 50, 30, 70, 40, 100, 50, 30];
        p.parlamento.seggi_senato = vec![15, 25, 15, 35, 20, 50, 25, 15];
        p.governo.presidente = 5;
        p.governo.coalizione = vec![3, 4, 5];
        for d in &mut p.governo.dicasteri {
            if *d < 3 {
                *d = 5;
            }
        }
        for st in &mut p.ministri.dicasteri {
            if let Some(m) = &mut st.ministro
                && m.partito < 3
            {
                m.partito = 5;
            }
        }
        p
    }

    /// Correzione A2: la sfiducia al governo solo se la caduta darebbe un governo diverso; con la
    /// guida al centro (3) il Capo dello Stato la darebbe alla destra, prima per seggi.
    #[test]
    fn niente_sfiducia_al_governo_che_rinascerebbe_uguale() {
        let mut p = sfiducia_allo_stesso_governo();
        assert!(p.voto_sfiducia(None).is_some_and(|v| v.3));
        p.sfiducia_ia();
        assert!(
            matches!(p.parlamento.sfiducia, Some((Some(_), 1))),
            "{:?}",
            p.parlamento.sfiducia
        );
        let mut p = sfiducia_allo_stesso_governo();
        p.governo.presidente = 3;
        p.sfiducia_ia();
        assert_eq!(p.parlamento.sfiducia, Some((None, 1)));
    }

    #[test]
    fn senza_partiti_ia_all_opposizione_ne_ministri_nessuna_sfiducia() {
        let mut p = sfiducia_al_limite();
        p.governo.coalizione.extend([0, 6]);
        p.sfiducia_ia();
        assert!(p.parlamento.sfiducia.is_none() && p.mosse.is_empty());
        let mut p = sfiducia_al_limite();
        for st in &mut p.ministri.dicasteri {
            st.ministro = None;
        }
        p.sfiducia_ia();
        assert!(p.parlamento.sfiducia.is_none() && p.mosse.is_empty());
    }

    /// Scelta 16 del piano: il promotore porta solo una proposta che gli conviene, anche se
    /// un'altra raccoglierebbe più seggi di governo (senza il filtro vince una a −0,5).
    #[test]
    fn la_proposta_ia_conviene_al_promotore() {
        let mut p = Partita::nuova(crate::dati::dati_base(), 1);
        p.parlamento.misure.retain(|m| m.promotore.is_none());
        p.proposta_ia();
        let m = (p.parlamento.misure.iter())
            .find(|m| m.promotore.is_some())
            .expect("la partita di prova dà il posto a un partito IA");
        let j = m.promotore.expect("cercata col promotore");
        let posizioni = parlamento::posizioni(&p.dati, &p.disposizioni, &m.valori);
        let guadagno = voto::guadagni(&p.partiti, posizioni);
        assert!(guadagno[j] > 0.0, "{}: {}", j, guadagno[j]);
    }

    /// Lo spareggio della proposta: coi partiti di governo senza seggi ogni candidata ne raccoglie
    /// zero, e vince quella col guadagno del promotore più alto.
    #[test]
    fn a_parita_di_seggi_vince_il_guadagno_del_promotore() {
        let mut p = Partita::nuova(crate::dati::dati_base(), 1);
        p.parlamento.misure.retain(|m| m.promotore.is_none());
        for k in 0..p.partiti.len() {
            if p.governo.al_governo(k) {
                p.parlamento.seggi_camera[k] = 0;
                p.parlamento.seggi_senato[k] = 0;
            }
        }
        let j = (p.posto_riservato()).expect("la partita di prova dà il posto a un partito IA");
        let guadagno = |p: &Partita, valori: &[(usize, usize)]| {
            let posizioni = parlamento::posizioni(&p.dati, &p.disposizioni, valori);
            voto::guadagni(&p.partiti, posizioni)[j]
        };
        // Prima della mossa: dopo, la disposizione scelta è in calendario e la bozza è rifiutata.
        let mut massimo = f64::NEG_INFINITY;
        for x in (p.dati.disposizioni().iter()).filter(|x| x.norma == Norma::Ordinaria) {
            for g in &x.gradini {
                let v = [ValoreProposto {
                    disposizione: x.id.clone(),
                    gradino: g.id.clone(),
                }];
                if let Ok(b) =
                    (p.parlamento).bozza_proposta(&p.dati, &p.disposizioni, p.turno, j, &v)
                {
                    massimo = massimo.max(guadagno(&p, &b.valori));
                }
            }
        }
        p.proposta_ia();
        let m = (p.parlamento.misure.iter())
            .find(|m| m.promotore == Some(j))
            .expect("una candidata col guadagno positivo c'è");
        assert_eq!(guadagno(&p, &m.valori), massimo);
    }

    #[test]
    fn l_elenco_dei_partiti() {
        let n = |v: &[&str]| elenco(&v.iter().map(|s| (*s).to_string()).collect::<Vec<_>>());
        assert_eq!(n(&["A"]), "A");
        assert_eq!(n(&["A", "B"]), "A e B");
        assert_eq!(n(&["A", "B", "C"]), "A, B e C");
    }
}
