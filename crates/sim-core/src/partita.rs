use crate::caso::Caso;
use crate::dati::DatiGioco;
use crate::disposizioni::{Disposizioni, Norma, data_di};
use crate::economia::{
    self, Annata, Bilancio, Economia, FUORI_STAGIONE, RiepilogoBilancio, Voce, livello_ok,
    perdita_reale, perdite_scaglioni,
};
use crate::elettorato::{
    Affluenza, Condizione, Elettorato, Fascia, Intenzioni, Quesito, indice_gruppo, toccati,
};
use crate::elezioni::{Esito, LeggeElettorale, SeggiLista, Soggetto};
use crate::errore::Errore;
use crate::grandezze::Grandezze;
use crate::informazione::{ElettoreCampione, Informazione, genera};
use crate::legislatura::{
    self, Assegnazione, Cessione, Dichiarazione, EsitoElezioni, Fase, Fine, Incarico, Legislatura,
    Quadro, VoceDisponibilita, VoceRichiesta, VotiPartito, coalizioni_al_voto,
};
use crate::ministri::{Causa, Contesto, Ministri, TipoProposta, causa_peggiore, penalizza};
use crate::misure::{CAPACITA_NEUTRA, Misure};
use crate::parlamento::{
    self, CausaIntesa, Conteggio, Decreto, EsitoReferendum, EsitoVoto, FondiDecreto, FormaMisura,
    Intesa, Misura, Parlamento, Referendum, StatoAula, TipoReferendum, ValoreProposto, voto,
};
use crate::partiti::{self, Governo, ID_GIOCATORE, Partito};
use crate::vista::{
    Anteprima, SeggiPartito, Vista, VistaBilancio, VistaReferendum, VistaStatistica, VoceAlleato,
    VoceBudget, VoceDicastero, VoceGrandezza, VocePartito,
};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub const VERSIONE_SCHEMA: u32 = 7;

/// Trimestre di calendario: `trimestre` va da 1 a 4.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Trimestre {
    pub anno: u16,
    pub trimestre: u8,
}

impl Trimestre {
    pub const TURNO_ZERO: Trimestre = Trimestre {
        anno: 2026,
        trimestre: 4,
    };

    pub fn precedente(self) -> Trimestre {
        if self.trimestre == 1 {
            Trimestre {
                anno: self.anno.saturating_sub(1),
                trimestre: 4,
            }
        } else {
            Trimestre {
                anno: self.anno,
                trimestre: self.trimestre.saturating_sub(1),
            }
        }
    }

    pub fn successivo(self) -> Trimestre {
        if self.trimestre == 4 {
            Trimestre {
                anno: self.anno.saturating_add(1),
                trimestre: 1,
            }
        } else {
            Trimestre {
                anno: self.anno,
                trimestre: self.trimestre.saturating_add(1),
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Partita {
    versione_schema: u32,
    caso: Caso,
    dati: DatiGioco,
    turno: u32,
    data: Trimestre,
    economia: Economia,
    partiti: Vec<Partito>,
    governo: Governo,
    elettorato: Elettorato,
    informazione: Informazione,
    grandezze: Grandezze,
    ministri: Ministri,
    misure: Misure,
    disposizioni: Disposizioni,
    parlamento: Parlamento,
    intesa: Intesa,
    /// Indice del partito del giocatore.
    giocatore: usize,
    legislatura: Legislatura,
    fine: Option<Fine>,
}

impl Partita {
    /// La partita di prova dal governo uscente: parlamento uscente dello scenario, il giocatore
    /// è il partito del Presidente del Consiglio e la legislatura dura `durata_legislatura`.
    pub fn nuova(dati: DatiGioco, seme: u64) -> Partita {
        Partita::costruisci(dati, seme, None)
    }

    /// Il turno zero col governo uscente dello scenario. Con `nuovo` (la partita vera) il partito
    /// del giocatore entra in coda prima di elettorato e informazione, così ogni vettore per
    /// partito nasce della misura giusta, e le Camere sono sciolte, senza seggi.
    pub(crate) fn costruisci(dati: DatiGioco, seme: u64, nuovo: Option<Partito>) -> Partita {
        let economia = Economia::iniziale(&dati);
        let grandezze = Grandezze::iniziali(&dati);
        let (mut partiti, governo) = partiti::iniziali(&dati);
        let coalizioni = (dati.scenario().coalizioni.iter())
            .map(|c| {
                (c.iter())
                    .filter_map(|id| partiti.iter().position(|p| &p.id == id))
                    .collect()
            })
            .collect();
        let fondata = nuovo.is_some();
        partiti.extend(nuovo);
        let mut elettorato = Elettorato::iniziale(&dati, partiti.len(), &economia);
        elettorato.avvia_voto(&dati, &partiti, &governo);
        let mut caso = Caso::nuovo(seme);
        let informazione =
            Informazione::nuova(&dati, &mut caso, &elettorato, &partiti, &governo, &economia);
        let disposizioni = Disposizioni::iniziali(&dati);
        let n = partiti.len();
        let (parlamento, giocatore, legislatura) = if fondata {
            let sciolte = Parlamento {
                seggi_camera: vec![0; n],
                seggi_senato: vec![0; n],
                ..Parlamento::default()
            };
            let legislatura = Legislatura {
                inizio: 0,
                scadenza: 0,
                fase: Fase::Sciolta,
                coalizioni,
                dichiarazione: None,
                elezioni: Vec::new(),
                totali: (0, 0),
                incarico: None,
                proposta: None,
            };
            (sciolte, n.saturating_sub(1), legislatura)
        } else {
            let durata = disposizioni.leve(&dati).durata_legislatura;
            let uscente = Parlamento::uscente(&dati, &partiti);
            let legislatura = Legislatura {
                inizio: 0,
                scadenza: (4.0 * durata - 1.0).round().max(0.0) as u32,
                fase: Fase::Ordinaria,
                coalizioni,
                dichiarazione: None,
                elezioni: Vec::new(),
                totali: (
                    uscente.seggi_camera.iter().sum(),
                    uscente.seggi_senato.iter().sum(),
                ),
                incarico: None,
                proposta: None,
            };
            (uscente, governo.presidente, legislatura)
        };
        let ministri = Ministri::iniziali(
            &Contesto {
                dati: &dati,
                governo: &governo,
                partiti: &partiti,
                grandezze: &grandezze,
                economia: &economia,
                sondaggio: &informazione.sondaggio,
                disposizioni: &disposizioni,
                parlamento: &parlamento,
                indice_prezzi: informazione.statistica.indice_prezzi,
                turno: 0,
                data: Trimestre::TURNO_ZERO,
            },
            &mut caso,
        );
        Partita {
            parlamento,
            intesa: Intesa::iniziale(&dati, partiti.len()),
            misure: Misure::neutre(&dati),
            disposizioni,
            versione_schema: VERSIONE_SCHEMA,
            caso,
            informazione,
            economia,
            elettorato,
            grandezze,
            ministri,
            partiti,
            governo,
            dati,
            turno: 0,
            data: Trimestre::TURNO_ZERO,
            giocatore,
            legislatura,
            fine: None,
        }
    }

    // Ordine della chiusura:
    // default delle decisioni (proposte, emendamenti aperti respinti tranne quelli delle leggi
    // che slittano) → atti dei ministri → aula (leggi slittate nel turno del bilancio →
    // conversioni → bilancio → leggi → referendum) → motore (misure, economia, grandezze,
    // redditi, giudizio [prima di composizione, che aggiorna ultima_disoccupazione],
    // composizione, posizioni, sondaggio, statistica) → voto, nel turno del voto → soglie
    // (ministri, ritorno dell'intesa) → governo nuovo, nel turno di trattativa → calendario →
    // apertura (valori in arrivo, esiti dei due turni prima, emendamenti, rose, agende, proposte;
    // il costo del taglio se l'anno entra in esercizio provvisorio).
    // A partita finita la chiusura non fa nulla; a Camere sciolte l'aula non vota: nessuno può
    // presentare, il calendario è vuoto.
    pub fn chiudi_turno(&mut self) {
        if self.finita() {
            return;
        }
        let trattativa = self.legislatura.fase == Fase::Trattativa;
        self.con_contesto(|ministri, cx| ministri.decidi_proposte(cx));
        // Prima del voto del bilancio: approvato, esce dal calendario.
        let occupato = self.turno_del_bilancio();
        let respinti = self.parlamento.respingi_aperti(self.turno, occupato);
        self.emendamenti_respinti(respinti);
        let effetti = self.con_contesto(|ministri, cx| ministri.esegui(cx));
        self.grandezze.effetti.extend(effetti);
        if self.legislatura.fase != Fase::Sciolta {
            self.aula(occupato);
        }
        let reale_prima = self.economia.pil_trimestrale / self.economia.livello_prezzi;
        let nuove = Misure::da(
            &self.dati,
            &self.grandezze,
            &self.ministri,
            &self.economia,
            &self.elettorato,
            self.turno,
            self.data,
            &self.disposizioni,
        );
        let prima = std::mem::replace(&mut self.misure, nuove);
        self.economia
            .passo(&self.dati, &mut self.caso, self.data, &self.misure);
        self.grandezze
            .avanza(&self.dati, &self.economia, self.turno);
        self.elettorato
            .aggiorna_redditi(&self.dati, &self.economia, &self.misure);
        self.elettorato.giudizio(
            &self.dati,
            &self.economia,
            &self.governo,
            &self.grandezze,
            &prima,
            &self.misure,
        );
        self.elettorato
            .composizione(&self.dati, &self.economia, self.data);
        self.elettorato.posizioni(&self.dati, &mut self.caso);
        let vero = self.intenzioni_vere();
        self.informazione.rileva(
            &self.dati,
            &mut self.caso,
            &self.elettorato,
            &vero,
            &self.governo,
        );
        self.informazione.statistica.aggiorna(
            &self.dati,
            &mut self.caso,
            &self.economia,
            reale_prima,
            self.data,
        );
        if self.legislatura.turno_di_voto(self.turno) {
            self.vota_parlamento(&vero);
        }
        self.con_contesto(|ministri, cx| ministri.soglie(cx));
        self.intesa.soglie(&self.dati, self.turno);
        let ritorno = self.dati.manopola("ritorno_valence");
        for p in &mut self.partiti {
            p.valence += ritorno * (p.valence_base - p.valence);
        }
        if trattativa {
            self.forma_governo();
        }
        self.turno = self.turno.saturating_add(1);
        self.data = self.data.successivo();
        (self.disposizioni).arrivi(&self.dati, self.turno, &mut self.grandezze.effetti);
        self.parlamento.apri(self.turno);
        // Le elettorali rese non valide dai voti e dagli arrivi decadono prima degli emendamenti.
        for m in std::mem::take(&mut self.parlamento.misure) {
            if !self.decade(&m) {
                self.parlamento.misure.push(m);
            }
        }
        let aula = StatoAula {
            dati: &self.dati,
            disposizioni: &self.disposizioni,
            sondaggio: &self.informazione.sondaggio,
            partiti: &self.partiti,
            governo: &self.governo,
            intesa: &self.intesa.valori,
            coalizioni: self.coalizioni_correnti(),
        };
        self.parlamento.apri_emendamenti(&aula, self.turno);
        let cx = Contesto {
            dati: &self.dati,
            governo: &self.governo,
            partiti: &self.partiti,
            grandezze: &self.grandezze,
            economia: &self.economia,
            sondaggio: &self.informazione.sondaggio,
            disposizioni: &self.disposizioni,
            parlamento: &self.parlamento,
            indice_prezzi: self.informazione.statistica.indice_prezzi,
            turno: self.turno,
            data: self.data,
        };
        self.ministri.apri(&cx, &mut self.caso);
        // Anno nuovo in esercizio provvisorio: costa la perdita reale del bilancio che continua.
        if self.economia.esercizio_provvisorio(self.data) {
            let perdite = self.perdite(self.economia.bilancio_in_vigore(self.data));
            self.costo_taglio(&perdite);
        }
    }

    /// Le perdite reali in % delle voci dei dicasteri di `b`, all'indice pubblicato.
    fn perdite(&self, b: &Bilancio) -> Vec<f64> {
        let indice = self.indice_prezzi();
        (b.dicasteri.iter())
            .map(|d| perdita_reale(d.livello_prezzi, indice))
            .collect()
    }

    /// `BudgetTagliato`: `−costo_taglio_budget · perdita` per ogni dicastero con ministro.
    fn costo_taglio(&mut self, perdite: &[f64]) {
        let costo = self.dati.manopola("costo_taglio_budget");
        self.ministri.costo_taglio(perdite, costo, self.turno);
    }

    /// L'indice dei prezzi pubblicato: la legge di bilancio non legge il livello vero.
    fn indice_prezzi(&self) -> f64 {
        self.informazione.statistica.indice_prezzi
    }

    fn contesto(&self) -> Contesto<'_> {
        Contesto {
            dati: &self.dati,
            governo: &self.governo,
            partiti: &self.partiti,
            grandezze: &self.grandezze,
            economia: &self.economia,
            sondaggio: &self.informazione.sondaggio,
            disposizioni: &self.disposizioni,
            parlamento: &self.parlamento,
            indice_prezzi: self.informazione.statistica.indice_prezzi,
            turno: self.turno,
            data: self.data,
        }
    }

    /// Esegue `f` sui ministri (in scrittura) con ciò che leggono del resto della partita.
    pub(crate) fn con_contesto<R>(&mut self, f: impl FnOnce(&mut Ministri, &Contesto) -> R) -> R {
        let cx = Contesto {
            dati: &self.dati,
            governo: &self.governo,
            partiti: &self.partiti,
            grandezze: &self.grandezze,
            economia: &self.economia,
            sondaggio: &self.informazione.sondaggio,
            disposizioni: &self.disposizioni,
            parlamento: &self.parlamento,
            indice_prezzi: self.informazione.statistica.indice_prezzi,
            turno: self.turno,
            data: self.data,
        };
        f(&mut self.ministri, &cx)
    }

    /// `occupato`: il turno del bilancio, letto prima di ogni voto.
    fn aula(&mut self, occupato: bool) {
        if occupato {
            for m in self.parlamento.slitta_leggi(self.turno) {
                self.rimborsa_fiducia(&m);
            }
        }
        self.vota_forma(FormaMisura::Conversione);
        self.vota_forma(FormaMisura::Bilancio);
        self.vota_leggi();
        self.vota_confermativi();
        self.vota_abrogativo();
        // Le firme dell'abrogativo chiesto nel turno sono raccolte.
        if let Some(r) = &mut self.parlamento.referendum {
            r.firme = true;
        }
    }

    /// In calendario c'è un bilancio presentato in questo turno, col voto a fine turno.
    pub fn turno_del_bilancio(&self) -> bool {
        self.parlamento.turno_del_bilancio(self.turno)
    }

    /// Le misure di forma `forma`, in ordine di id: prima le conversioni dei decreti del turno
    /// (scelta 2), poi il bilancio.
    fn vota_forma(&mut self, forma: FormaMisura) {
        let ids: Vec<u32> = (self.parlamento.misure.iter())
            .filter(|m| m.forma == forma)
            .map(|m| m.id)
            .collect();
        for id in ids {
            self.vota(id);
        }
    }

    /// Le leggi col voto nel turno, in ordine di id (iter-legislativo-italiano:166).
    fn vota_leggi(&mut self) {
        let mut ids: Vec<u32> = (self.parlamento.misure.iter())
            .filter(|m| m.voto == self.turno)
            .map(|m| m.id)
            .collect();
        ids.sort_unstable();
        for id in ids {
            self.vota(id);
        }
    }

    /// Il voto della misura `id` nei due rami; la misura esce dal calendario. Approvata: i valori
    /// entrano in vigore e gli alleati lontani perdono intesa. Respinta: blocco
    /// (iter-legislativo-italiano:171), costo al proponente e valence al Presidente del
    /// Consiglio. Chi vota sì sposta la sua posizione percepita verso la misura.
    ///
    /// Elettorale (iter-legislativo-italiano:48-61): con la fiducia, prima il voto a blocchi e
    /// poi il voto finale, che porta i conteggi; bocciata al voto finale cade la legge, non la
    /// fiducia; approvata, ogni alleato che perde seggi attesi paga intesa. Costituzionale
    /// (iter-legislativo-italiano:76-80): la prima deliberazione approvata torna in calendario per
    /// la seconda; approvata la seconda senza i due terzi e con un quinto contro in un ramo, il
    /// valore aspetta il referendum confermativo. Conversione respinta: il decreto decade
    /// (`decreto_decaduto`), col costo e la valence del decreto invece di quelli della legge.
    /// Bilancio approvato: `approva_bilancio`; respinto: il bilancio presentato cade.
    fn vota(&mut self, id: u32) {
        let Some(k) = self.parlamento.misure.iter().position(|m| m.id == id) else {
            return;
        };
        let mut m = self.parlamento.misure.remove(k);
        let elettorale = m.forma == FormaMisura::Elettorale;
        // Due elettorali votate nella stessa chiusura: la seconda può decadere qui.
        if self.decade(&m) {
            return;
        }
        let aula = self.stato_aula();
        let p = aula.previsione(&m);
        let linee = aula.voci_linea(&p.linea);
        let pl = &self.parlamento;
        let a_blocchi = m.fiducia && elettorale;
        let nessuno = vec![0.0; p.linea.len()];
        let (q_camera, q_senato) = if a_blocchi {
            (&nessuno, &nessuno)
        } else {
            (&p.franchi_camera, &p.franchi)
        };
        let mut camera = voto::conta(&mut self.caso, &pl.seggi_camera, &p.linea, q_camera);
        let mut senato = voto::conta(&mut self.caso, &pl.seggi_senato, &p.linea, q_senato);
        let passa = |c: &Conteggio| c.si > c.no;
        let fiducia_respinta = m.fiducia && !(passa(&camera) && passa(&senato));
        // Il voto finale si estrae anche dopo la fiducia respinta, e si scarta: l'esito non sposta
        // i franchi tiratori delle misure che seguono.
        if a_blocchi {
            let c = voto::conta(
                &mut self.caso,
                &pl.seggi_camera,
                &p.linea,
                &p.franchi_camera,
            );
            let s = voto::conta(&mut self.caso, &pl.seggi_senato, &p.linea, &p.franchi);
            if !fiducia_respinta {
                (camera, senato) = (c, s);
            }
        }
        // Tutti votano: `sì > no` è anche la maggioranza assoluta della seconda deliberazione.
        let approvata = passa(&camera) && passa(&senato);
        if m.forma == FormaMisura::Bilancio {
            if approvata {
                self.approva_bilancio();
            } else {
                self.economia.bilancio_presentato = None;
            }
        }
        let turno = self.turno;
        let nuova = p.posizioni.map(|(_, n)| n);
        let prima = m.forma == FormaMisura::Costituzionale && m.deliberazione == 1;
        let esito = EsitoVoto {
            misura: id,
            nome: parlamento::nome_misura(&self.dati, &m),
            forma: m.forma,
            data: self.data,
            deliberazione: m.deliberazione,
            approvata,
            fiducia: m.fiducia,
            fiducia_respinta,
            linee,
            camera,
            senato,
            decaduta: None,
        };
        if approvata && !prima {
            let (c, s) = (&esito.camera, &esito.senato);
            let pl = &self.parlamento;
            let quinto = parlamento::quinto_contro(&self.governo, &pl.seggi_camera, &p.linea)
                || parlamento::quinto_contro(&self.governo, &pl.seggi_senato, &p.linea);
            let due_terzi = parlamento::due_terzi(c, &pl.seggi_camera)
                && parlamento::due_terzi(s, &pl.seggi_senato);
            if m.forma == FormaMisura::Costituzionale && !due_terzi && quinto {
                let voto = turno.saturating_add(1);
                self.parlamento.confermativi.push(Referendum {
                    tipo: TipoReferendum::Confermativo,
                    misura: Some(id),
                    valori: m.valori.clone(),
                    promotore: None,
                    voto,
                    firme: true,
                    invito_astensione: false,
                });
            } else {
                self.entra_in_vigore(&m);
            }
            if let Some(nuova) = nuova {
                self.norma_lontana(nuova);
            }
            self.seggi_persi(&p.seggi);
        } else if !approvata {
            // La fiducia respinta fa cadere la misura senza blocco (le dimissioni: piano 7).
            if !fiducia_respinta {
                let blocco = self.dati.manopola("turni_blocco_respinta") as u32;
                let da = turno.saturating_add(1).saturating_add(blocco);
                (self.parlamento.respinte).extend(m.valori.iter().map(|&(d, _)| (d, da)));
            }
            let (causa, costo, valence) = if m.forma == FormaMisura::Conversione {
                let (c, v) = ("costo_decreto_decaduto", "valence_decreto_decaduto");
                (Causa::DecretoDecaduto, c, v)
            } else {
                let (c, v) = ("costo_legge_respinta", "valence_legge_respinta");
                (Causa::LeggeRespinta, c, v)
            };
            if let Some(st) = m
                .proponente
                .and_then(|i| self.ministri.dicasteri.get_mut(i))
            {
                penalizza(st, causa, -self.dati.manopola(costo), turno);
            }
            if let Some(pdc) = self.partiti.get_mut(self.governo.presidente) {
                pdc.valence -= self.dati.manopola(valence);
            }
            if let Some(f) = m.fondi {
                self.decreto_decaduto(f);
            }
        }
        if let Some(nuova) = nuova {
            let s = self.dati.manopola("scivolamento_percepita");
            for (x, _) in self.partiti.iter_mut().zip(&p.linea).filter(|(_, si)| **si) {
                x.posizione_percepita += s * (nuova - x.posizione_percepita);
            }
        }
        self.parlamento.esiti.insert(0, esito);
        if approvata && prima {
            m.deliberazione = 2;
            m.voto = turno.saturating_add(1);
            self.parlamento.misure.push(m);
        }
    }

    /// L'elettorale `m` che, dopo i voti e gli arrivi venuti dalla presentazione, darebbe una legge
    /// non valida decade senza voto: esito con `decaduta`, nessuna estrazione, nessun blocco,
    /// nessun costo. `true` se decade.
    fn decade(&mut self, m: &Misura) -> bool {
        if m.forma != FormaMisura::Elettorale {
            return false;
        }
        let Some(motivo) = parlamento::legge_non_valida(&self.dati, &self.disposizioni, &m.valori)
        else {
            return false;
        };
        let esito = EsitoVoto {
            misura: m.id,
            nome: parlamento::nome(&self.dati, &m.valori),
            forma: m.forma,
            data: self.data,
            deliberazione: m.deliberazione,
            camera: Conteggio { si: 0, no: 0 },
            senato: Conteggio { si: 0, no: 0 },
            approvata: false,
            fiducia: m.fiducia,
            fiducia_respinta: false,
            linee: Vec::new(),
            decaduta: Some(format!(
                "Decaduta senza voto: con le leggi già approvate non sarebbe più applicabile \
                 ({motivo})"
            )),
        };
        self.parlamento.esiti.insert(0, esito);
        true
    }

    /// I valori della misura approvata entrano in vigore, o arrivano fra `turni_rinvio_regime`
    /// turni se un emendamento accolto li rinvia.
    fn entra_in_vigore(&mut self, m: &Misura) {
        let (turno, legge) = (self.turno, format!("L. {}/{}", m.id, self.data.anno));
        let rinvio = self.dati.manopola("turni_rinvio_regime") as u32;
        for &(d, g) in &m.valori {
            if parlamento::rinviata(&m.emendamenti, d) {
                let arrivo = turno.saturating_add(rinvio);
                // `controlla` e `presenta` garantiscono `d` e `g`: non fallisce.
                let _ = (self.disposizioni).programma(&self.dati, d, g, arrivo, &legge);
                continue;
            }
            (self.disposizioni).cambia(
                &self.dati,
                d,
                g,
                turno,
                &legge,
                &mut self.grandezze.effetti,
            );
        }
    }

    /// `SeggiPersi`: ogni alleato con `Δseggi < 0` perde `costo_seggio_perso · |Δseggi|`.
    fn seggi_persi(&mut self, seggi: &[(i32, i32)]) {
        let costo = self.dati.manopola("costo_seggio_perso");
        for j in self.alleati() {
            let delta = seggi.get(j).map_or(0, |&(c, s)| c + s);
            if delta < 0 {
                let d = costo * f64::from(delta);
                (self.intesa).varia(j, CausaIntesa::SeggiPersi, d, self.turno);
            }
        }
    }

    /// I referendum confermativi del turno (art. 138, nessun quorum), uno per legge sulla
    /// posizione media dei suoi valori: approvato con `sì > no`, entrano in vigore tutti.
    fn vota_confermativi(&mut self) {
        let turno = self.turno;
        let (dovuti, restano): (Vec<Referendum>, Vec<Referendum>) =
            std::mem::take(&mut self.parlamento.confermativi)
                .into_iter()
                .partition(|r| r.voto <= turno);
        self.parlamento.confermativi = restano;
        for r in dovuti {
            let posizioni = parlamento::posizioni(&self.dati, &self.disposizioni, &r.valori);
            let (pos_no, pos_si) = (posizioni.map(|p| p.0), posizioni.map(|p| p.1));
            let quesito = Quesito {
                pos_si,
                pos_no,
                peso_governo: self.dati.manopola("peso_governo_confermativo"),
                invito: 0.0,
                astensione: self.dati.manopola("astensione_confermativo"),
                toccati: Vec::new(),
                guadagno: Vec::new(),
            };
            let a = (self.elettorato).referendum(&self.dati, &self.partiti, &self.governo, quesito);
            let nome = parlamento::nome(&self.dati, &r.valori);
            let esito = EsitoReferendum::da(r.tipo, nome, self.data, &a);
            if esito.approvato {
                let legge = format!("L. {}/{}", r.misura.unwrap_or_default(), self.data.anno);
                for &(d, g) in &r.valori {
                    (self.disposizioni).cambia(
                        &self.dati,
                        d,
                        g,
                        turno,
                        &legge,
                        &mut self.grandezze.effetti,
                    );
                }
            }
            self.parlamento.esiti_referendum.insert(0, esito);
        }
    }

    /// L'abrogativo del turno (art. 75): sì al valore dopo l'abrogazione, no al vigente, nessun
    /// peso del giudizio sul governo (scelta 9), `invito_astensione` se il governo l'ha chiesto.
    /// Quorum e `sì > no`: la disposizione va al valore dopo l'abrogazione (gli arrivi si
    /// cancellano), il gradino abrogato si blocca e ogni partito al governo perde
    /// `valence_referendum_perso`. Senza quorum il promotore perde `valence_referendum_fallito`.
    /// Se la disposizione ha già il valore dopo l'abrogazione e nessun arrivo, o l'abrogazione
    /// renderebbe non valida la legge elettorale con le leggi e gli arrivi venuti dopo la
    /// promozione, il referendum decade senza voto: esito con `decaduto`, nessun blocco, nessuna
    /// valence. Il titolo dell'esito è il gradino vigente che il quesito abroga: se una legge lo ha
    /// cambiato, il quesito si sposta su quello. Sull'elettorale gli elettori votano i seggi
    /// attesi (`Quesito::guadagno`).
    fn vota_abrogativo(&mut self) {
        let turno = self.turno;
        let Some(r) = self.parlamento.referendum.take_if(|r| r.voto <= turno) else {
            return;
        };
        let Some(&(d, dopo)) = r.valori.first() else {
            return;
        };
        let vigente = self.disposizioni.voci[d].in_vigore;
        let nome = parlamento::nome(&self.dati, &[(d, vigente)]);
        let decaduto = if vigente == dopo && self.disposizioni.voci[d].in_arrivo.is_empty() {
            Some(
                "Decaduto senza voto: la disposizione ha già il valore che il quesito le darebbe"
                    .to_string(),
            )
        } else {
            parlamento::legge_non_valida(&self.dati, &self.disposizioni, &[(d, dopo)]).map(|m| {
                format!(
                    "Decaduto senza voto: con le leggi già approvate l'abrogazione non sarebbe \
                     più applicabile ({m})"
                )
            })
        };
        if decaduto.is_some() {
            let nessuno = Affluenza {
                si: 0.0,
                no: 0.0,
                astenuti: 0.0,
            };
            let esito = EsitoReferendum {
                decaduto,
                ..EsitoReferendum::da(r.tipo, nome, self.data, &nessuno)
            };
            self.parlamento.esiti_referendum.insert(0, esito);
            return;
        }
        let x = &self.dati.disposizioni()[d];
        let invito = if r.invito_astensione {
            self.dati.manopola("invito_astensione")
        } else {
            0.0
        };
        let guadagno = if x.norma == Norma::Elettorale {
            let per_punto = self.dati.manopola("seggi_per_punto_linea");
            (self.stato_aula().seggi_attesi(&[(d, dopo)]).iter())
                .map(|&(c, s)| f64::from(c + s) / per_punto)
                .collect()
        } else {
            Vec::new()
        };
        let quesito = Quesito {
            pos_si: x.posizione(dopo),
            pos_no: x.posizione(vigente),
            peso_governo: 0.0,
            invito,
            astensione: self.dati.manopola("astensione_abrogativo"),
            toccati: toccati(&self.dati, d),
            guadagno,
        };
        let a = (self.elettorato).referendum(&self.dati, &self.partiti, &self.governo, quesito);
        let mut esito = EsitoReferendum::da(r.tipo, nome, self.data, &a);
        esito.approvato &= esito.quorum;
        let (quorum, approvato) = (esito.quorum, esito.approvato);
        if approvato {
            let legge = format!("referendum del {}", self.data.anno);
            (self.disposizioni).cambia(
                &self.dati,
                d,
                dopo,
                turno,
                &legge,
                &mut self.grandezze.effetti,
            );
            let bloccati = &mut self.disposizioni.voci[d].bloccati;
            if vigente != dopo && !bloccati.contains(&vigente) {
                bloccati.push(vigente);
            }
        }
        if approvato {
            let perso = self.dati.manopola("valence_referendum_perso");
            for (j, p) in self.partiti.iter_mut().enumerate() {
                if self.governo.al_governo(j) {
                    p.valence -= perso;
                }
            }
        } else if let Some(p) = r
            .promotore
            .filter(|_| !quorum)
            .and_then(|j| self.partiti.get_mut(j))
        {
            p.valence -= self.dati.manopola("valence_referendum_fallito");
        }
        self.parlamento.esiti_referendum.insert(0, esito);
    }

    /// Norma approvata in posizione `nuova`: ogni alleato distante almeno `distanza_alleato`
    /// perde intesa.
    fn norma_lontana(&mut self, nuova: f64) {
        let soglia = self.dati.manopola("distanza_alleato");
        let costo = self.dati.manopola("costo_norma_lontana");
        for j in self.alleati() {
            let distanza = (self.partiti[j].posizione_dichiarata - nuova).abs();
            if distanza < soglia {
                continue;
            }
            let delta = -costo * (distanza - soglia + 1.0);
            (self.intesa).varia(j, CausaIntesa::NormaLontana, delta, self.turno);
        }
    }

    /// `Emendamento`: deciso dal giocatore; respinto, costa intesa all'alleato.
    pub(crate) fn emendamento(
        &mut self,
        misura: u32,
        partito: &str,
        accogli: bool,
    ) -> Result<(), Errore> {
        let respinto = (self.parlamento).decidi_emendamento(
            &self.dati,
            &self.partiti,
            misura,
            partito,
            accogli,
        )?;
        self.emendamenti_respinti(respinto);
        Ok(())
    }

    /// `EmendamentoRespinto`: `−costo_emendamento_respinto` a ogni alleato.
    fn emendamenti_respinti(&mut self, alleati: impl IntoIterator<Item = usize>) {
        let delta = -self.dati.manopola("costo_emendamento_respinto");
        for j in alleati {
            (self.intesa).varia(j, CausaIntesa::EmendamentoRespinto, delta, self.turno);
        }
    }

    /// `PoniFiducia`: la misura si vota nel turno; ogni alleato perde `parlamento::costo_fiducia`.
    pub(crate) fn poni_fiducia(&mut self, misura: u32) -> Result<(), Errore> {
        let m = (self.parlamento).poni_fiducia(&self.dati, misura, self.turno)?;
        let nuova = parlamento::posizioni(&self.dati, &self.disposizioni, &m.valori).map(|p| p.1);
        let forma = m.forma;
        for j in self.alleati() {
            let x = self.partiti[j].posizione_dichiarata;
            let delta = -parlamento::costo_fiducia(&self.dati, forma, x, nuova);
            (self.intesa).varia(j, CausaIntesa::Fiducia, delta, self.turno);
        }
        Ok(())
    }

    fn stato_aula(&self) -> StatoAula<'_> {
        StatoAula {
            dati: &self.dati,
            disposizioni: &self.disposizioni,
            sondaggio: &self.informazione.sondaggio,
            partiti: &self.partiti,
            governo: &self.governo,
            intesa: &self.intesa.valori,
            coalizioni: self.coalizioni_correnti(),
        }
    }

    /// Il bilancio presentato è approvato dall'aula. Se l'anno è in esercizio
    /// provvisorio, il bilancio è dell'anno in corso, il passo di gennaio-marzo che segue lo mette
    /// in vigore e il margine residuo diventa 3/4 del margine del bilancio approvato. Ogni
    /// approvazione costa ai ministri la perdita reale delle loro voci.
    fn approva_bilancio(&mut self) {
        let Some(b) = self.economia.bilancio_presentato.take() else {
            return;
        };
        if self.economia.esercizio_provvisorio(self.data) {
            for (st, d) in self.ministri.dicasteri.iter_mut().zip(&b.dicasteri) {
                st.margine_residuo = d.margine / 4 * 3;
            }
        }
        let perdite = self.perdite(&b);
        self.costo_taglio(&perdite);
        self.economia.bilancio_approvato = Some(b);
    }

    /// La bozza della legge di bilancio: in ottobre-dicembre senza bilancio approvato, la copia
    /// nominale del bilancio in vigore con l'anno dopo; nell'esercizio provvisorio, la stessa con
    /// l'anno in corso; altrimenti `None`.
    pub fn bozza_bilancio(&self) -> Option<Bilancio> {
        economia::bozza(&self.economia, self.data)
    }

    /// Le voci scelte di `b` rivalutate all'indice dei prezzi pubblicato.
    ///
    /// # Errors
    /// `Bilancio` se il dicastero non c'è o una voce rivalutata esce da 0-1e15.
    pub fn indicizza(&self, b: &Bilancio, voce: &Voce) -> Result<Bilancio, Errore> {
        economia::indicizza(b, voce, self.indice_prezzi()).map_err(Errore::Bilancio)
    }

    /// Costo, peso sul tetto UE e perdita reale di `b` rispetto alla bozza, come lo approverebbe
    /// l'aula.
    ///
    /// # Errors
    /// `Bilancio` fuori stagione, con le stesse regole della presentazione, o con la spesa netta
    /// di riferimento nulla.
    pub fn riepilogo_bilancio(&self, b: &Bilancio) -> Result<RiepilogoBilancio, Errore> {
        let (bozza, b) = self.bilancio_ammesso(b)?;
        economia::riepilogo(
            &self.economia,
            &self.dati,
            &self.elettorato,
            &self.misure,
            &b,
            &bozza,
            &self.informazione.statistica,
        )
        .map_err(Errore::Bilancio)
    }

    /// `PresentaBilancio`: sostituisce quello già presentato, nella stessa misura; l'aula lo vota
    /// in chiusura.
    pub(crate) fn presenta_bilancio(&mut self, b: &Bilancio) -> Result<(), Errore> {
        let (_, b) = self.bilancio_ammesso(b)?;
        self.economia.bilancio_presentato = Some(b);
        self.parlamento.apri_bilancio(self.turno);
        Ok(())
    }

    /// `PresentaLegge`: la misura entra in calendario.
    pub(crate) fn presenta_legge(&mut self, valori: &[ValoreProposto]) -> Result<(), Errore> {
        (self.parlamento).presenta(
            &self.dati,
            &self.disposizioni,
            Some(&mut self.ministri),
            self.turno,
            valori,
        )
    }

    /// Il partito `promotore`, all'opposizione, chiede il referendum abrogativo su
    /// `disposizione`: firme a fine turno, voto ad aprile-giugno dell'anno dopo.
    ///
    /// # Errors
    /// `Comando`, e la partita resta identica: promotore sconosciuto o al governo, disposizione
    /// sconosciuta o non referendabile, niente da abrogare, quesito elettorale inammissibile, un
    /// referendum già in corso, fuori finestra (dopo luglio-settembre).
    pub fn promuovi_referendum(
        &mut self,
        promotore: &str,
        disposizione: &str,
    ) -> Result<(), Errore> {
        let r = (self.parlamento).abrogativo(
            &self.stato_aula(),
            self.turno,
            promotore,
            disposizione,
        )?;
        self.parlamento.referendum = Some(r);
        Ok(())
    }

    /// `InvitaAstensione`.
    pub(crate) fn invita_astensione(&mut self) -> Result<(), Errore> {
        self.parlamento.invita_astensione()
    }

    /// L'anteprima della legge `valori`: le regole di `PresentaLegge` tranne il
    /// posto in calendario; la partita non cambia.
    ///
    /// # Errors
    /// Quelli di `PresentaLegge` tranne il calendario pieno.
    pub fn anteprima(&self, valori: &[ValoreProposto]) -> Result<Anteprima, Errore> {
        let m = (self.parlamento).bozza(&self.dati, &self.disposizioni, self.turno, valori)?;
        let capacita = (m.proponente)
            .and_then(|i| self.ministri.dicasteri.get(i)?.ministro.as_ref())
            .map_or(CAPACITA_NEUTRA, |x| x.capacita_effettiva(&self.dati));
        Ok(parlamento::anteprima(
            &self.parlamento,
            &self.stato_aula(),
            &m,
            capacita,
            &self.caso,
            self.economia.pil_potenziale_annuo(),
        ))
    }

    /// Il massimo spostabile da ogni dicastero: il margine residuo senza il decreto accolto in
    /// agenda, entro il margine annuo.
    fn spostabili(&self) -> Vec<i64> {
        let b = self.economia.bilancio_in_vigore(self.data);
        (self.ministri.dicasteri.iter().zip(&b.dicasteri))
            .map(|(st, d)| {
                let decreto = st.agenda.as_ref().map_or(0, |ag| ag.decreto);
                (st.margine_residuo - decreto).min(d.margine)
            })
            .collect()
    }

    /// L'anteprima del `decreto`: la conversione che aprirebbe, a saldo zero e senza effetti;
    /// i rifiuti sono quelli di `EmanaDecreto` e la partita non cambia.
    ///
    /// # Errors
    /// `Comando`, come `EmanaDecreto`.
    pub fn anteprima_decreto(&self, decreto: &Decreto) -> Result<Anteprima, Errore> {
        let (da, a, importo) = parlamento::ammetti(
            &self.dati,
            &self.parlamento.decaduti,
            &self.spostabili(),
            decreto,
        )?;
        let fondi = FondiDecreto::Spostati { da, a, importo };
        Ok(self.anteprima_conversione(&self.parlamento.conversione(self.turno, fondi), a))
    }

    /// L'anteprima del decreto che `Accogli` produrrebbe per la proposta di fondi del
    /// `dicastero`: il costo è l'importo, gli effetti quelli dell'atto; la partita non cambia.
    ///
    /// # Errors
    /// `Comando` col nome del dicastero: sconosciuto, senza proposta di fondi, esercizio
    /// provvisorio, decreto decaduto, e i rifiuti di `Accogli` sull'atto.
    pub fn anteprima_accogli(&self, dicastero: &str) -> Result<Anteprima, Errore> {
        self.accogli_ripetuto(dicastero)?;
        let cx = self.contesto();
        // accogli = true (spende denaro), sul_ministro = false (non è un comando sul ministro)
        let i = self.ministri.ammesso(&cx, dicastero, true, false)?;
        let nome = &self.dati.dicasteri()[i].nome;
        let fondi = matches!(
            (self.ministri.dicasteri.get(i)).and_then(|st| st.proposta.map(|p| p.tipo)),
            Some(TipoProposta::Fondi { .. })
        );
        if !fondi {
            return Err(Errore::Comando(format!(
                "nessuna proposta di fondi: {nome}"
            )));
        }
        let (atto, importo) = self.ministri.fondi_da_accogliere(&cx, i)?;
        let fondi = FondiDecreto::Accolti {
            dicastero: i,
            atto,
            importo,
        };
        Ok(self.anteprima_conversione(&self.parlamento.conversione(self.turno, fondi), i))
    }

    /// L'anteprima della conversione `m`, con la capacità del ministro del dicastero `proponente`.
    fn anteprima_conversione(&self, m: &Misura, proponente: usize) -> Anteprima {
        let capacita = (self.ministri.dicasteri.get(proponente))
            .and_then(|st| st.ministro.as_ref())
            .map_or(CAPACITA_NEUTRA, |x| x.capacita_effettiva(&self.dati));
        parlamento::anteprima(
            &self.parlamento,
            &self.stato_aula(),
            m,
            capacita,
            &self.caso,
            self.economia.pil_potenziale_annuo(),
        )
    }

    /// `Accogli` di una proposta di riforma: la legge si presenta come con `PresentaLegge`, firma
    /// del ministro compresa (spec, storia 32), e la proposta esce solo se la presentazione passa.
    pub(crate) fn accogli_riforma(
        &mut self,
        dicastero: usize,
        disposizione: usize,
        gradino: usize,
    ) -> Result<(), Errore> {
        let x = &self.dati.disposizioni()[disposizione];
        let valori = [ValoreProposto {
            disposizione: x.id.clone(),
            gradino: x.gradini[gradino].id.clone(),
        }];
        (self.parlamento).presenta(
            &self.dati,
            &self.disposizioni,
            Some(&mut self.ministri),
            self.turno,
            &valori,
        )?;
        if let Some(st) = self.ministri.dicasteri.get_mut(dicastero) {
            st.proposta = None;
        }
        Ok(())
    }

    /// La bozza e `b` come lo approverebbe l'aula.
    fn bilancio_ammesso(&self, b: &Bilancio) -> Result<(Bilancio, Bilancio), Errore> {
        let bozza =
            (self.bozza_bilancio()).ok_or_else(|| Errore::Bilancio(FUORI_STAGIONE.into()))?;
        let b = economia::normalizza(b, &bozza, self.indice_prezzi()).map_err(Errore::Bilancio)?;
        Ok((bozza, b))
    }

    fn vista_bilancio(&self) -> VistaBilancio {
        let e = &self.economia;
        let b = e.bilancio_in_vigore(self.data);
        let indice = self.indice_prezzi();
        VistaBilancio {
            anno: b.anno,
            dicasteri: (b.dicasteri.iter())
                .map(|d| VoceBudget {
                    id: d.id.clone(),
                    spesa: d.spesa,
                    salari: d.salari,
                    acquisti: d.acquisti,
                    margine: d.margine,
                    perdita_reale: perdita_reale(d.livello_prezzi, indice),
                })
                .collect(),
            aliquote: b.aliquote.clone(),
            perdita_reale_scaglioni: perdite_scaglioni(&b.aliquote, indice).collect(),
            fondo_riserva: b.fondo_riserva,
            presentabile: self.bozza_bilancio().is_some(),
            presentato: e.bilancio_presentato.is_some(),
            esercizio_provvisorio: e.esercizio_provvisorio(self.data),
        }
    }

    /// `Accogli`: l'importo è un decreto del dicastero, speso per intero nel trimestre che si
    /// chiude; la conversione va in calendario sull'atto dell'agenda.
    pub(crate) fn aggiungi_decreto(&mut self, dicastero: usize, importo: i64) {
        let Some(atto) = (self.ministri.dicasteri.get(dicastero))
            .and_then(|st| st.agenda.as_ref())
            .map(|ag| ag.atto)
        else {
            return;
        };
        if let Some(x) = self.economia.decreti.get_mut(dicastero) {
            *x += importo;
        }
        let fondi = FondiDecreto::Accolti {
            dicastero,
            atto,
            importo,
        };
        self.parlamento.apri_conversione(self.turno, fondi);
    }

    /// Ritiro del decreto accolto (scelta 1): niente speso, nessuna conversione, e si paga solo
    /// il comando: con la questione di fiducia posta, gli alleati riavranno il suo costo d'intesa.
    pub(crate) fn ritira_decreto(&mut self, dicastero: usize, importo: i64) {
        if let Some(x) = self.economia.decreti.get_mut(dicastero) {
            *x -= importo;
        }
        if let Some(m) = self.parlamento.ritira(dicastero).filter(|m| m.fiducia) {
            self.rimborsa_fiducia(&m);
        }
    }

    /// Dopo `Dirotta` la conversione del decreto accolto segue l'atto dell'agenda: decade, se
    /// respinta, l'atto pagato davvero.
    pub(crate) fn conversione_su_agenda(&mut self, dicastero: usize) {
        let atto = (self.ministri.dicasteri.get(dicastero))
            .and_then(|st| st.agenda.as_ref())
            .map(|ag| ag.atto);
        if let Some(atto) = atto {
            self.parlamento.sposta_conversione(dicastero, atto);
        }
    }

    /// Gli alleati riavranno il costo d'intesa della questione di fiducia posta su `m`.
    fn rimborsa_fiducia(&mut self, m: &Misura) {
        let nuova = parlamento::posizioni(&self.dati, &self.disposizioni, &m.valori).map(|p| p.1);
        for j in self.alleati() {
            let x = self.partiti[j].posizione_dichiarata;
            let delta = parlamento::costo_fiducia(&self.dati, m.forma, x, nuova);
            (self.intesa).varia(j, CausaIntesa::Fiducia, delta, self.turno);
        }
    }

    /// `Accogli` sulla firma (dicastero e atto) di un decreto decaduto: rifiutato
    /// (iter-legislativo-italiano:20-23, 67).
    ///
    /// # Errors
    /// `Comando` col nome dell'atto.
    pub(crate) fn accogli_ripetuto(&self, dicastero: &str) -> Result<(), Errore> {
        let proposta = (self.dati.dicasteri().iter().position(|d| d.id == dicastero))
            .and_then(|i| Some((i, self.ministri.dicasteri.get(i)?.proposta?.tipo)));
        let Some((i, TipoProposta::Fondi { atto, .. })) = proposta else {
            return Ok(());
        };
        if self.parlamento.decaduto_accolto(i, atto) {
            let atto = self.dati.atti().get(atto).map_or("", |a| a.nome.as_str());
            return Err(Errore::Comando(format!(
                "decreto decaduto non si ripete: {atto}"
            )));
        }
        Ok(())
    }

    /// `EmanaDecreto` (storia 46): sposta `importo` da margine e spesa di `da` a quelli di `a`,
    /// nel bilancio in vigore e nei margini residui, anche nell'esercizio provvisorio (scelta 12).
    /// Il ministro di `da` paga `FondiTolti` come un taglio di `100 · importo / spesa` punti.
    /// Spostabile: vedi `spostabili`.
    pub(crate) fn emana_decreto(&mut self, decreto: &Decreto) -> Result<(), Errore> {
        let b = self.economia.bilancio_in_vigore(self.data);
        let spostabili = self.spostabili();
        let decaduti = &self.parlamento.decaduti;
        let (da, a, importo) = parlamento::ammetti(&self.dati, decaduti, &spostabili, decreto)?;
        let spesa = b.dicasteri.get(da).map_or(1, |d| d.spesa.max(1));
        self.sposta_fondi(da, a, importo);
        if let Some(st) = self.ministri.dicasteri.get_mut(da) {
            let costo = self.dati.manopola("costo_taglio_budget");
            let delta = -costo * 100.0 * importo as f64 / spesa as f64;
            penalizza(st, Causa::FondiTolti, delta, self.turno);
        }
        let fondi = FondiDecreto::Spostati { da, a, importo };
        self.parlamento.apri_conversione(self.turno, fondi);
        Ok(())
    }

    /// `importo` passa da margine e spesa di `da` a quelli di `a` nel bilancio in vigore, e fra i
    /// margini residui.
    fn sposta_fondi(&mut self, da: usize, a: usize, importo: i64) {
        let b = self.economia.bilancio_in_vigore_mut(self.data);
        for (i, x) in [(da, -importo), (a, importo)] {
            if let Some(d) = b.dicasteri.get_mut(i) {
                d.margine += x;
                d.spesa += x;
            }
            if let Some(st) = self.ministri.dicasteri.get_mut(i) {
                st.margine_residuo += x;
            }
        }
    }

    /// Il decreto non convertito: il denaro speso resta speso, uno spostamento torna indietro
    /// per quanto `a` ha ancora (iter-legislativo-italiano:163); la firma non si ripete.
    fn decreto_decaduto(&mut self, f: FondiDecreto) {
        if let FondiDecreto::Spostati { da, a, importo } = f {
            let residuo = self
                .ministri
                .dicasteri
                .get(a)
                .map_or(0, |st| st.margine_residuo);
            let annuo = (self.economia.bilancio_in_vigore(self.data).dicasteri.get(a))
                .map_or(0, |d| d.margine);
            self.sposta_fondi(a, da, importo.min(residuo).min(annuo).max(0));
        }
        self.parlamento.decaduti.push(f);
    }

    // Da qui a `informazione`: stato vero, per il CLI e i test, mai per la `Vista`.
    pub fn economia(&self) -> &Economia {
        &self.economia
    }

    pub fn grandezze(&self) -> &Grandezze {
        &self.grandezze
    }

    /// Valore nazionale vero di una grandezza.
    pub fn grandezza(&self, id: &str) -> Option<f64> {
        let g = self.dati.grandezza(id)?;
        Some(self.grandezze.nazionale(&self.dati, g))
    }

    /// Misure dell'ultima chiusura.
    pub fn misure(&self) -> &Misure {
        &self.misure
    }

    pub fn partiti(&self) -> &[Partito] {
        &self.partiti
    }

    pub fn ministri(&self) -> &Ministri {
        &self.ministri
    }

    pub fn governo(&self) -> &Governo {
        &self.governo
    }

    pub fn elettorato(&self) -> &Elettorato {
        &self.elettorato
    }

    /// Intenzioni di voto vere per gruppo e nazionali.
    pub fn intenzioni_vere(&self) -> Intenzioni {
        Intenzioni::calcola(&self.dati, &self.elettorato, &self.partiti, &self.governo)
    }

    pub fn informazione(&self) -> &Informazione {
        &self.informazione
    }

    /// Tre elettori del gruppo, estratti dal sondaggio con un estrattore estemporaneo:
    /// non avanzano nessun flusso del `Caso`.
    ///
    /// # Errors
    /// `Elettorato` se la regione non c'è.
    pub fn elettori_campione(
        &self,
        regione: &str,
        c: Condizione,
        f: Fascia,
    ) -> Result<Vec<ElettoreCampione>, Errore> {
        let r = self
            .dati
            .elettorato()
            .regioni
            .iter()
            .position(|x| x.id == regione)
            .ok_or_else(|| Errore::Elettorato(format!("regione sconosciuta: {regione}")))?;
        let chiave = u64::from(self.turno) * 1024 + indice_gruppo(r, c, f) as u64;
        let mut rng = self.caso.estemporaneo(chiave);
        Ok(genera(
            &self.informazione.sondaggio,
            &self.voci_partiti(),
            &self.dati,
            r,
            c,
            f,
            &mut rng,
        ))
    }

    pub fn disposizioni(&self) -> &Disposizioni {
        &self.disposizioni
    }

    pub fn parlamento(&self) -> &Parlamento {
        &self.parlamento
    }

    pub fn intesa(&self) -> &Intesa {
        &self.intesa
    }

    /// Indice del partito del giocatore.
    pub fn giocatore(&self) -> usize {
        self.giocatore
    }

    pub fn legislatura(&self) -> &Legislatura {
        &self.legislatura
    }

    pub fn finita(&self) -> bool {
        self.fine.is_some()
    }

    /// Il giocatore guida il governo in carica: i comandi di governo sono suoi.
    pub fn governa(&self) -> bool {
        !self.finita()
            && self.legislatura.fase == Fase::Ordinaria
            && self.governo.presidente == self.giocatore
    }

    /// Il turno del prossimo voto: questo a Camere sciolte, altrimenti la scadenza.
    fn turno_del_voto(&self) -> u32 {
        if self.legislatura.fase == Fase::Sciolta {
            self.turno
        } else {
            self.legislatura.scadenza
        }
    }

    /// I partiti al governo, escluso quello del Presidente del Consiglio.
    pub fn alleati(&self) -> Vec<usize> {
        (0..self.partiti.len())
            .filter(|&j| j != self.governo.presidente && self.governo.al_governo(j))
            .collect()
    }

    /// L'id del gradino in vigore della disposizione `id`.
    pub fn in_vigore(&self, id: &str) -> Option<&str> {
        let k = self.dati.disposizione(id)?;
        let g = self.disposizioni.voci.get(k)?.in_vigore;
        Some(self.dati.disposizioni()[k].gradini.get(g)?.id.as_str())
    }

    /// La legge elettorale delle nove disposizioni elettorali in vigore.
    pub fn legge_elettorale(&self) -> LeggeElettorale {
        self.disposizioni.legge_elettorale(&self.dati)
    }

    /// Riparto dei seggi sul sondaggio pubblico con la legge elettorale e il numero dei
    /// parlamentari in vigore: voti regionali dalle quote degli incroci; coalizioni correnti:
    /// quelle al voto nel turno del voto, altrove quelle dell'ultimo voto; partiti senza
    /// coalizione da soli.
    ///
    /// # Errors
    /// Quelli di `elezioni::riparto`.
    pub fn proiezione_seggi(&self) -> Result<Esito, Errore> {
        parlamento::proietta(
            &self.dati,
            &self.partiti,
            &self.informazione.sondaggio,
            &self.legge_elettorale(),
            &self.disposizioni.leve(&self.dati),
            &self.coalizioni_correnti(),
        )
    }

    /// Nel turno del voto le coalizioni al voto, altrove quelle dell'ultimo voto.
    fn coalizioni_correnti(&self) -> Vec<Vec<usize>> {
        let l = &self.legislatura;
        if l.turno_di_voto(self.turno) {
            coalizioni_al_voto(&l.coalizioni, self.giocatore, l.dichiarazione.as_ref())
        } else {
            l.coalizioni.clone()
        }
    }

    /// Il voto (premio e riparto, sistemi-elettorali:39; bias comune che si rivela,
    /// sondaggi-ed-errore:50, :55): riparto delle intenzioni vere `vero` con le coalizioni al voto,
    /// la legge e i seggi dei rami in vigore, poi le cessioni; voto precedente = intenzioni, bias
    /// comune nuovo, legislatura nuova in trattativa, incarico. Riparto rifiutato: seggi di prima,
    /// esito con le quote senza seggi. Il giocatore senza seggi in entrambe le Camere: fine.
    fn vota_parlamento(&mut self, vero: &Intenzioni) {
        let (n, g) = (self.partiti.len(), self.giocatore);
        let leve = self.disposizioni.leve(&self.dati);
        let coalizioni = self.coalizioni_correnti();
        let voti = legislatura::voti_per_regione(&self.dati, &self.elettorato, vero);
        let per_partito: Vec<u64> = (0..n)
            .map(|j| voti.values().filter_map(|v| v.get(j)).sum())
            .collect();
        let votanti = per_partito.iter().sum::<u64>().max(1) as f64;
        let legge = self.legge_elettorale();
        let riparto = parlamento::riparto_con(&self.dati, &legge, &leve, &coalizioni, voti);
        let eletti = riparto.as_ref().ok().map(|e| {
            let totali = |v: &[SeggiLista]| v.iter().map(SeggiLista::totale).collect::<Vec<_>>();
            let (mut camera, mut senato) = (totali(&e.camera), totali(&e.senato));
            let d = self.legislatura.dichiarazione.as_ref();
            legislatura::cedi(&mut camera, &mut senato, g, d.map_or(&[], |d| &d.cessioni));
            (camera, senato)
        });
        let ids = |c: &[usize]| -> Vec<String> {
            (c.iter().filter_map(|&j| self.partiti.get(j)))
                .map(|p| p.id.clone())
                .collect()
        };
        let premio = (riparto.ok().and_then(|e| e.premio)).map(|s| match s {
            Soggetto::Lista(j) => ids(&[j]).join("+"),
            Soggetto::Coalizione(k) => ids(coalizioni.get(k).map_or(&[], Vec::as_slice)).join("+"),
        });
        let seggi = |v: Option<&Vec<u32>>, j: usize| v.and_then(|v| v.get(j)).copied().unwrap_or(0);
        let esito = EsitoElezioni {
            data: self.data,
            affluenza: 1.0 - vero.nazionali.get(n).copied().unwrap_or(0.0),
            voti: (self.partiti.iter().zip(&per_partito).enumerate())
                .map(|(j, (p, &v))| VotiPartito {
                    partito: p.id.clone(),
                    quota: v as f64 / votanti,
                    camera: seggi(eletti.as_ref().map(|e| &e.0), j),
                    senato: seggi(eletti.as_ref().map(|e| &e.1), j),
                })
                .collect(),
            premio,
            coalizioni: coalizioni.iter().map(|c| ids(c)).collect(),
        };
        // ponytail: col riparto rifiutato restano i seggi di prima; al turno zero le Camere restano
        // vuote in Trattativa, la partita finisce senza seggi e `salva` fallisce. Non deve
        // accadere: la legge in vigore è sempre valida (piano 6 Task 9) e `carica` rifiuta le
        // coalizioni e le dichiarazioni che il riparto rifiuterebbe.
        if let Some((camera, senato)) = eletti {
            self.legislatura.totali = (camera.iter().sum(), senato.iter().sum());
            self.parlamento.seggi_camera = camera;
            self.parlamento.seggi_senato = senato;
        }
        for (gruppo, v) in self.elettorato.gruppi.iter_mut().zip(&vero.per_gruppo) {
            gruppo.voto_precedente.clone_from(v);
        }
        (self.informazione).riestrai_bias(&self.dati, &mut self.caso, n);
        let pl = &self.parlamento;
        let turno = self.turno;
        let l = &mut self.legislatura;
        l.incarico = legislatura::incarico(&coalizioni, g, &pl.seggi_camera, &pl.seggi_senato);
        l.elezioni.insert(0, esito);
        l.coalizioni = coalizioni;
        l.dichiarazione = None;
        l.proposta = None;
        l.inizio = turno.saturating_add(1);
        l.scadenza = turno.saturating_add((4.0 * leve.durata_legislatura).round() as u32);
        l.fase = Fase::Trattativa;
        let nessuno = |v: &[u32]| v.get(g).is_none_or(|&x| x == 0);
        if nessuno(&pl.seggi_camera) && nessuno(&pl.seggi_senato) {
            self.fine = Some(Fine::SenzaSeggi);
        }
    }

    fn quadro(&self) -> Quadro<'_> {
        Quadro {
            dati: &self.dati,
            partiti: &self.partiti,
            sondaggio: &self.informazione.sondaggio,
            legge: self.legge_elettorale(),
            leve: self.disposizioni.leve(&self.dati),
            ultime: &self.legislatura.coalizioni,
            giocatore: self.giocatore,
            intesa: &self.intesa.valori,
            alleati: if self.governo.presidente == self.giocatore {
                self.alleati()
            } else {
                Vec::new()
            },
        }
    }

    /// La disponibilità di ogni partito IA sulla dichiarazione in corso; senza dichiarazione,
    /// sulla coalizione del giocatore dell'ultimo voto (alla fondazione, lui da solo).
    pub fn disponibilita(&self) -> Vec<VoceDisponibilita> {
        self.quadro().disponibilita(&self.dichiarazione_corrente())
    }

    /// La dichiarazione in corso; senza, la coalizione del giocatore dell'ultimo voto, senza
    /// cessioni.
    fn dichiarazione_corrente(&self) -> Dichiarazione {
        let l = &self.legislatura;
        l.dichiarazione.clone().unwrap_or_else(|| {
            let sua = coalizioni_al_voto(&l.coalizioni, self.giocatore, None);
            Dichiarazione {
                partiti: (sua.into_iter().next().unwrap_or_default().into_iter())
                    .filter(|&j| j != self.giocatore)
                    .collect(),
                cessioni: Vec::new(),
            }
        })
    }

    /// `DichiaraCoalizione`: solo nel turno del voto; sostituisce la dichiarazione precedente.
    pub(crate) fn dichiara_coalizione(
        &mut self,
        partiti: &[String],
        cessioni: &[Cessione],
    ) -> Result<(), Errore> {
        if !self.legislatura.turno_di_voto(self.turno) {
            return Err(Errore::Comando(
                "Coalizione: si dichiara solo nel turno del voto".into(),
            ));
        }
        let d = legislatura::dichiarazione(&self.quadro(), partiti, cessioni)?;
        self.legislatura.dichiarazione = Some(d);
        Ok(())
    }

    /// `ProponiSpartizione`: solo in trattativa con l'incarico al giocatore; un'assegnazione per
    /// ogni dicastero, a un partito della sua coalizione o a un tecnico. Sostituisce la proposta
    /// precedente.
    ///
    /// # Errors
    /// `Comando` con la fase, col nome del dicastero sconosciuto, ripetuto o mancante, o col nome
    /// del partito sconosciuto o fuori dalla coalizione.
    pub(crate) fn proponi_spartizione(&mut self, dicasteri: &[Assegnazione]) -> Result<(), Errore> {
        let ko = |m: String| Err(Errore::Comando(m));
        let l = &self.legislatura;
        if l.fase != Fase::Trattativa || l.incarico != Some(Incarico::Giocatore) {
            return ko("Spartizione: si propone solo in trattativa, con l'incarico".into());
        }
        let d = self.dati.dicasteri();
        let coalizione = self.coalizione_del_giocatore();
        let mut proposta: Vec<Option<Option<usize>>> = vec![None; d.len()];
        for a in dicasteri {
            let Some(i) = d.iter().position(|x| x.id == a.dicastero) else {
                return ko(format!("Dicastero sconosciuto: {}", a.dicastero));
            };
            if proposta[i].is_some() {
                return ko(format!("{} ripetuto nella spartizione", d[i].nome));
            }
            let partito = match &a.partito {
                None => None,
                Some(id) => {
                    let Some(j) = self.partiti.iter().position(|p| &p.id == id) else {
                        return ko(format!("Partito sconosciuto: {id}"));
                    };
                    if !coalizione.contains(&j) {
                        return ko(format!(
                            "{} non è nella coalizione: niente dicasteri",
                            self.partiti[j].nome
                        ));
                    }
                    Some(j)
                }
            };
            proposta[i] = Some(partito);
        }
        if let Some(i) = proposta.iter().position(Option::is_none) {
            return ko(format!(
                "{} senza assegnazione: servono tutti i dicasteri",
                d[i].nome
            ));
        }
        self.legislatura.proposta = Some(proposta.into_iter().flatten().collect());
        Ok(())
    }

    /// La coalizione del giocatore all'ultimo voto; lui da solo se non ne ha.
    fn coalizione_del_giocatore(&self) -> Vec<usize> {
        let g = self.giocatore;
        (self.legislatura.coalizioni.iter())
            .find(|c| c.contains(&g))
            .cloned()
            .unwrap_or_else(|| vec![g])
    }

    /// Seggi di Camera più Senato per partito.
    fn seggi_per_partito(&self) -> Vec<u32> {
        let pl = &self.parlamento;
        (pl.seggi_camera.iter().zip(&pl.seggi_senato))
            .map(|(c, s)| c + s)
            .collect()
    }

    /// Per alleato della coalizione del giocatore: (indice, richiesta, offerta di `assegnati`, i
    /// punti di peso dei dicasteri che gli dà).
    fn quote(&self, assegnati: &[Option<usize>]) -> Vec<(usize, f64, f64)> {
        let (g, seggi) = (self.giocatore, self.seggi_per_partito());
        let coalizione = self.coalizione_del_giocatore();
        let di = |j: usize| seggi.get(j).copied().unwrap_or(0);
        let s = coalizione.iter().map(|&j| di(j)).sum();
        (coalizione.iter().filter(|&&j| j != g))
            .map(|&j| {
                let offerta = (self.dati.dicasteri().iter().zip(assegnati))
                    .filter(|&(_, &a)| a == Some(j))
                    .map(|(d, _)| f64::from(d.peso_spartizione))
                    .sum();
                (j, legislatura::richiesta(&self.dati, di(j), s), offerta)
            })
            .collect()
    }

    /// La spartizione del giocatore: la sua proposta o, senza, quella di default.
    fn assegnati_dal_giocatore(&self) -> Vec<Option<usize>> {
        self.legislatura.proposta.clone().unwrap_or_else(|| {
            let c = self.coalizione_del_giocatore();
            let seggi = self.seggi_per_partito();
            let v = legislatura::spartizione_di_default(&self.dati, &c, self.giocatore, &seggi);
            v.into_iter().map(Some).collect()
        })
    }

    /// In trattativa con l'incarico al giocatore, una voce per alleato: richiesta e offerta della
    /// proposta in corso o, senza, di quella di default che la chiusura applicherà; vuota altrove.
    pub fn richieste(&self) -> Vec<VoceRichiesta> {
        let l = &self.legislatura;
        if l.fase != Fase::Trattativa || l.incarico != Some(Incarico::Giocatore) {
            return Vec::new();
        }
        (self.quote(&self.assegnati_dal_giocatore()).into_iter())
            .map(|(j, richiesta, offerta)| VoceRichiesta {
                partito: self.partiti[j].id.clone(),
                richiesta,
                offerta,
            })
            .collect()
    }

    /// Il governo nuovo alla chiusura della trattativa, dopo le soglie. Con l'incarico al
    /// giocatore la sua proposta (o quella di default), i tecnici al suo partito, e ogni alleato
    /// riparte da `intesa_base + intesa_per_punto · (offerta − richiesta)`; con l'incarico IA il
    /// primo partito per seggi e la spartizione di default. Poi ministri tutti nuovi, giudizio da
    /// zero per chi entra al governo, fase ordinaria. Senza incarico la trattativa resta aperta
    /// (Task 10).
    fn forma_governo(&mut self) {
        let seggi = self.seggi_per_partito();
        let di = |j: &usize| seggi.get(*j).copied().unwrap_or(0);
        let (presidente, coalizione, assegnati) = match &self.legislatura.incarico {
            None => return,
            Some(Incarico::Giocatore) => (
                self.giocatore,
                self.coalizione_del_giocatore(),
                self.assegnati_dal_giocatore(),
            ),
            Some(Incarico::Ia { coalizione }) => {
                let guida = (coalizione.iter())
                    .max_by_key(|&j| (di(j), std::cmp::Reverse(*j)))
                    .copied()
                    .unwrap_or(self.giocatore);
                let v = legislatura::spartizione_di_default(&self.dati, coalizione, guida, &seggi);
                (guida, coalizione.clone(), v.into_iter().map(Some).collect())
            }
        };
        if presidente == self.giocatore {
            let (base, per_punto) = (
                self.dati.manopola("intesa_base"),
                self.dati.manopola("intesa_per_punto"),
            );
            for (j, richiesta, offerta) in self.quote(&assegnati) {
                if let Some(x) = self.intesa.valori.get_mut(j) {
                    *x = base;
                }
                let delta = per_punto * (offerta - richiesta);
                (self.intesa).varia(j, CausaIntesa::Spartizione, delta, self.turno);
            }
        }
        let entranti: Vec<usize> = (coalizione.iter().copied())
            .filter(|&j| !self.governo.al_governo(j))
            .collect();
        for gruppo in &mut self.elettorato.gruppi {
            for &j in &entranti {
                if let Some(x) = gruppo.giudizio.get_mut(j) {
                    *x = 0.0;
                }
            }
        }
        let tecnici: Vec<bool> = assegnati.iter().map(Option::is_none).collect();
        self.governo = Governo {
            presidente,
            dicasteri: assegnati.iter().map(|a| a.unwrap_or(presidente)).collect(),
            coalizione,
        };
        let cx = Contesto {
            dati: &self.dati,
            governo: &self.governo,
            partiti: &self.partiti,
            grandezze: &self.grandezze,
            economia: &self.economia,
            sondaggio: &self.informazione.sondaggio,
            disposizioni: &self.disposizioni,
            parlamento: &self.parlamento,
            indice_prezzi: self.informazione.statistica.indice_prezzi,
            turno: self.turno,
            data: self.data,
        };
        (self.ministri).nuovo_governo(&cx, &mut self.caso, &tecnici);
        let l = &mut self.legislatura;
        l.fase = Fase::Ordinaria;
        l.incarico = None;
        l.proposta = None;
    }

    fn voci_partiti(&self) -> Vec<VocePartito> {
        (self.partiti.iter().enumerate())
            .map(|(j, p)| VocePartito {
                id: p.id.clone(),
                nome: p.nome.clone(),
                posizione_dichiarata: p.posizione_dichiarata,
                posizione_percepita: p.posizione_percepita,
                valence: p.valence,
                al_governo: self.governo.al_governo(j),
            })
            .collect()
    }

    pub fn vista(&self) -> Vista {
        let s = &self.informazione.statistica;
        let proiezione = self.proiezione_seggi().ok().map(|e| {
            let seggi = e.camera.iter().zip(&e.senato);
            (self.partiti.iter().zip(seggi))
                .map(|(p, (c, s))| SeggiPartito {
                    partito: p.id.clone(),
                    camera: c.totale(),
                    senato: s.totale(),
                })
                .collect()
        });
        let voto = self.turno_del_voto();
        let l = &self.legislatura;
        Vista {
            turno: self.turno,
            data: self.data,
            giocatore: (self.partiti.get(self.giocatore))
                .map(|p| p.id.clone())
                .unwrap_or_default(),
            fase: self.legislatura.fase,
            voto: data_di(voto),
            turni_al_voto: voto.saturating_sub(self.turno),
            governa: self.governa(),
            finita: self.finita(),
            spread_pb: self.economia.spread.round() as i32,
            dicasteri: self
                .dati
                .dicasteri()
                .iter()
                .map(|d| VoceDicastero {
                    id: d.id.clone(),
                    nome: d.nome.clone(),
                })
                .collect(),
            grandezze: (self.dati.grandezze().iter().zip(&self.grandezze.prima))
                .map(|(g, valori)| VoceGrandezza {
                    id: g.id.clone(),
                    nome: g.nome.clone(),
                    verso: g.verso,
                    valori: valori.clone(),
                })
                .collect(),
            partiti: self.voci_partiti(),
            ministeri: self.ministri.voci(&self.contesto()),
            sondaggio: self.informazione.sondaggio.clone(),
            proiezione,
            statistica: VistaStatistica {
                trimestre: s.trimestre,
                prima_stima: s.prima_stima.clone(),
                revisione: s.revisione.clone(),
            },
            bilancio: self.vista_bilancio(),
            disposizioni: self.disposizioni.voci_vista(&self.dati),
            seggi: (self.partiti.iter())
                .zip(&self.parlamento.seggi_camera)
                .zip(&self.parlamento.seggi_senato)
                .map(|((p, &camera), &senato)| SeggiPartito {
                    partito: p.id.clone(),
                    camera,
                    senato,
                })
                .collect(),
            alleati: self.voci_alleati(),
            calendario: self.parlamento.voci_calendario(&self.stato_aula()),
            esiti: self.parlamento.esiti.clone(),
            referendum: (self.parlamento.referendum.iter())
                .chain(&self.parlamento.confermativi)
                .map(|r| VistaReferendum {
                    tipo: r.tipo,
                    nome: parlamento::nome(&self.dati, &r.valori),
                    voto: data_di(r.voto),
                    promotore: (r.promotore)
                        .and_then(|j| self.partiti.get(j))
                        .map(|p| p.id.clone()),
                    firme: r.firme,
                    invito_astensione: r.invito_astensione,
                })
                .collect(),
            esiti_referendum: self.parlamento.esiti_referendum.clone(),
            disponibilita: if l.turno_di_voto(self.turno) && !self.finita() {
                self.disponibilita()
            } else {
                Vec::new()
            },
            // Nel turno del voto la coalizione con cui si vota, anche ereditata; altrove nessuna.
            coalizione: if l.turno_di_voto(self.turno) {
                (self.dichiarazione_corrente().partiti.iter())
                    .filter_map(|&j| self.partiti.get(j))
                    .map(|p| p.id.clone())
                    .collect()
            } else {
                Vec::new()
            },
            richieste: self.richieste(),
            elezioni: l.elezioni.clone(),
        }
    }

    fn voci_alleati(&self) -> Vec<VoceAlleato> {
        let soglia = self.dati.manopola("soglia_ultimatum");
        (self.alleati().into_iter())
            .map(|j| {
                let variazioni: Vec<_> = (self.intesa.variazioni.iter())
                    .filter(|v| v.partito == j)
                    .cloned()
                    .collect();
                let intesa = self.intesa.valori.get(j).copied().unwrap_or_default();
                VoceAlleato {
                    partito: self.partiti[j].id.clone(),
                    intesa,
                    ultimatum: intesa < soglia,
                    causa_principale: causa_peggiore(variazioni.iter().map(|v| (v.causa, v.delta))),
                    variazioni,
                }
            })
            .collect()
    }

    /// Snapshot JSON completo: schema, stato del generatore (con il seme), dati, calendario,
    /// economia, grandezze, partiti e governo, elettorato, informazione, disposizioni, parlamento
    /// e intesa.
    ///
    /// # Errors
    /// Serializzazione fallita, o stato che riletto non torna identico
    /// (per esempio una manopola NaN, scritta come `null`): chi salva non
    /// deve scrivere un file illeggibile.
    pub fn salva(&self) -> Result<String, Errore> {
        let s = serde_json::to_string(self)
            .map_err(|e| Errore::SalvataggioNonRiuscito(e.to_string()))?;
        match Partita::carica(&s) {
            Ok(riletta) if riletta == *self => Ok(s),
            Ok(_) => Err(Errore::SalvataggioNonRiuscito(
                "lo stato riletto è diverso da quello salvato".into(),
            )),
            Err(e) => Err(Errore::SalvataggioNonRiuscito(format!(
                "lo stato salvato non si rilegge: {e}"
            ))),
        }
    }

    /// Ricostruisce una partita da uno snapshot JSON, validandolo.
    ///
    /// # Errors
    /// JSON illeggibile, schema diverso da `VERSIONE_SCHEMA`, trimestre fuori
    /// da 1-4, dati che non passano `DatiGioco::valida`, o `economia` che non combacia coi dati
    /// (regioni diverse o in altro ordine, `prezzi_settori`/`gap_settori` di lunghezza sbagliata,
    /// conti incoerenti, importi o pensioni oltre 1e15), o partiti, governo, elettorato e
    /// informazione che non combaciano coi dati (`Elettorato::controlla`, `Informazione::controlla`),
    /// o disposizioni, parlamento e intesa che non combaciano coi dati (`Disposizioni::controlla`,
    /// `Parlamento::controlla`, `Intesa::controlla`).
    pub fn carica(json: &str) -> Result<Partita, Errore> {
        let valore: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| Errore::SalvataggioIlleggibile(e.to_string()))?;
        let trovata = valore
            .get("versione_schema")
            .and_then(serde_json::Value::as_u64)
            .and_then(|v| u32::try_from(v).ok())
            .unwrap_or(0);
        if trovata != VERSIONE_SCHEMA {
            return Err(Errore::SchemaDiverso {
                trovata,
                attesa: VERSIONE_SCHEMA,
            });
        }
        let partita: Partita = serde_json::from_value(valore)
            .map_err(|e| Errore::SalvataggioIlleggibile(e.to_string()))?;
        if !(1..=4).contains(&partita.data.trimestre) {
            return Err(Errore::SalvataggioIlleggibile(format!(
                "trimestre {} fuori da 1-4",
                partita.data.trimestre
            )));
        }
        partita.dati.valida()?;
        let e = &partita.economia;
        let regioni_uguali = e.regioni.iter().map(|r| &r.id).eq(partita
            .dati
            .regioni()
            .iter()
            .map(|r| &r.id));
        if !regioni_uguali {
            return Err(Errore::SalvataggioIlleggibile(
                "economia.regioni non combacia con le regioni dei dati".into(),
            ));
        }
        let id_dicasteri = || partita.dati.dicasteri().iter().map(|d| &d.id);
        for b in e.bilanci() {
            if !b.dicasteri.iter().map(|d| &d.id).eq(id_dicasteri()) {
                return Err(Errore::SalvataggioIlleggibile(
                    "economia.bilancio non combacia con i dicasteri dei dati".into(),
                ));
            }
        }
        let n_dicasteri = partita.dati.dicasteri().len();
        if e.decreti.len() != n_dicasteri {
            return Err(Errore::SalvataggioIlleggibile(format!(
                "economia.decreti ha {} voci, i dicasteri sono {n_dicasteri}",
                e.decreti.len()
            )));
        }
        let n_settori = partita.dati.struttura().settori.len();
        for (campo, n) in [
            ("prezzi_settori", e.prezzi_settori.len()),
            ("gap_settori", e.gap_settori.len()),
        ] {
            if n != n_settori {
                return Err(Errore::SalvataggioIlleggibile(format!(
                    "economia.{campo} ha {n} voci, i settori sono {n_settori}"
                )));
            }
        }
        partita.economia.controlla_coerenza(partita.data)?;
        partita.grandezze.controlla(&partita.dati)?;
        partita.misure.controlla(&partita.dati)?;
        partita
            .disposizioni
            .controlla(&partita.dati, partita.turno)?;
        let n = partita.partiti.len();
        partita.parlamento.controlla(
            &partita.dati,
            n,
            partita.turno,
            partita.legislatura.totali,
        )?;
        partita.intesa.controlla(n)?;
        partita.controlla_partiti()?;
        partita
            .ministri
            .controlla(&partita.dati, partita.partiti.len())?;
        partita.controlla_decreti()?;
        let in_aula = (partita.parlamento.misure.iter())
            .filter(|m| m.forma == FormaMisura::Bilancio)
            .count();
        ok(
            in_aula == usize::from(partita.economia.bilancio_presentato.is_some())
                && (in_aula == 0 || partita.turno_del_bilancio()),
            "bilancio presentato senza la sua misura in calendario per il turno, o viceversa",
        )?;
        Ok(partita)
    }
}

impl Partita {
    /// Il decreto di ogni agenda è quello di `economia.decreti`, entro il margine residuo, e la
    /// conversione in calendario porta il suo atto e il suo importo; senza decreto, nessuna
    /// conversione.
    fn controlla_decreti(&self) -> Result<(), Errore> {
        let dicasteri = self.ministri.dicasteri.iter().zip(&self.economia.decreti);
        for (i, (st, &x)) in dicasteri.enumerate() {
            let decreto = st.agenda.as_ref().map_or(0, |ag| ag.decreto);
            let attesa =
                (st.agenda.as_ref().filter(|ag| ag.decreto > 0)).map(|ag| FondiDecreto::Accolti {
                    dicastero: i,
                    atto: ag.atto,
                    importo: ag.decreto,
                });
            let buono = decreto == x
                && decreto <= st.margine_residuo
                && self.parlamento.in_conversione(i) == attesa;
            ok(
                buono,
                "decreto in agenda diverso da economia.decreti o dalla conversione, o oltre il \
                 margine",
            )?;
        }
        Ok(())
    }

    /// Partiti, giocatore, legislatura, governo ed elettorato di un salvataggio contro i dati: il
    /// partito del giocatore, se fondato, è l'ultimo, in più di quelli dello scenario.
    fn controlla_partiti(&self) -> Result<(), Errore> {
        let n = self.partiti.len();
        let attesi = self.dati.scenario().partiti.len();
        let fondato = self.giocatore.saturating_add(1) == n
            && self.partiti.last().is_some_and(|p| p.id == ID_GIOCATORE);
        let (attesi, giocatore_ok) = if fondato {
            (attesi.saturating_add(1), true)
        } else {
            (attesi, self.giocatore < n)
        };
        ok(
            n == attesi,
            &format!("partiti ha {n} voci, ne servono {attesi}"),
        )?;
        ok(giocatore_ok, "giocatore fuori dai partiti")?;
        ok(
            self.legislatura.coalizioni.iter().flatten().all(|&j| j < n),
            "legislatura.coalizioni con un partito fuori dai partiti",
        )?;
        let mut dichiarati = (self.legislatura.dichiarazione.iter())
            .flat_map(|d| d.partiti.iter().chain(d.cessioni.iter().map(|c| &c.0)));
        ok(
            dichiarati.all(|&j| j < n && j != self.giocatore),
            "legislatura.dichiarazione con un partito fuori dai partiti o col giocatore",
        )?;
        let l = &self.legislatura;
        // Il riparto al voto rifiuta un partito in due liste: carica rifiuta lo stato che lo porta.
        fn unici(v: impl IntoIterator<Item = usize>) -> bool {
            let mut visti = std::collections::BTreeSet::new();
            v.into_iter().all(|j| visti.insert(j))
        }
        ok(
            unici(l.coalizioni.iter().flatten().copied()),
            "legislatura.coalizioni non disgiunte",
        )?;
        ok(
            l.dichiarazione.as_ref().is_none_or(|d| {
                unici(d.partiti.iter().copied())
                    && unici(d.cessioni.iter().map(|c| c.0))
                    && d.cessioni.iter().all(|c| d.partiti.contains(&c.0))
            }),
            "legislatura.dichiarazione con un partito ripetuto, una cessione ripetuta o una cessione fuori dai membri",
        )?;
        ok(
            l.scadenza >= l.inizio,
            "legislatura.scadenza prima dell'inizio",
        )?;
        ok(
            l.fase == Fase::Sciolta || self.turno <= l.scadenza,
            "legislatura: turno oltre la scadenza fuori dalla fase sciolta",
        )?;
        ok(
            match &l.incarico {
                Some(Incarico::Ia { coalizione }) => {
                    !coalizione.is_empty() && coalizione.iter().all(|&j| j < n)
                }
                _ => true,
            },
            "legislatura.incarico IA con una coalizione vuota o un partito fuori dai partiti",
        )?;
        let coalizione = self.coalizione_del_giocatore();
        let in_trattativa = l.fase == Fase::Trattativa && l.incarico == Some(Incarico::Giocatore);
        ok(
            l.proposta.as_ref().is_none_or(|v| {
                in_trattativa
                    && v.len() == self.dati.dicasteri().len()
                    && v.iter().flatten().all(|j| coalizione.contains(j))
            }),
            "legislatura.proposta fuori dalla trattativa col giocatore, senza una voce per \
             dicastero o con un partito fuori dalla sua coalizione",
        )?;
        let pl = &self.parlamento;
        let vuoto = |seggi: &[u32]| seggi.iter().all(|&x| x == 0);
        ok(
            self.legislatura.fase == Fase::Sciolta
                || !(vuoto(&pl.seggi_camera) || vuoto(&pl.seggi_senato)),
            "legislatura.fase non sciolta con Camere senza seggi",
        )?;
        let g = &self.governo;
        ok(g.presidente < n, "governo.presidente fuori dai partiti")?;
        ok(
            g.dicasteri.len() == self.dati.dicasteri().len() && g.dicasteri.iter().all(|&p| p < n),
            "governo.dicasteri non combacia con i dicasteri o con i partiti",
        )?;
        ok(
            g.coalizione.iter().all(|&p| p < n) && unici(g.coalizione.iter().copied()),
            "governo.coalizione con un partito fuori dai partiti o ripetuto",
        )?;
        ok(
            self.partiti.iter().all(|p| {
                p.posizione_percepita.is_finite()
                    && p.valence.is_finite()
                    && p.valence_base.is_finite()
            }),
            "partiti: posizione_percepita, valence o valence_base non finita",
        )?;
        self.elettorato.controlla(&self.dati, n)?;
        self.informazione.controlla(&self.dati, n)
    }
}

/// Tetto agli importi in euro di un salvataggio: oltre, le somme in `i64` rischiano di traboccare.
pub(crate) const LIMITE_EURO: i64 = 1_000_000_000_000_000;

fn ok(condizione: bool, messaggio: &str) -> Result<(), Errore> {
    if condizione {
        Ok(())
    } else {
        Err(Errore::SalvataggioIlleggibile(messaggio.into()))
    }
}

impl Economia {
    /// Il bilancio corrente, l'approvato e il presentato.
    fn bilanci(&self) -> impl Iterator<Item = &Bilancio> {
        std::iter::once(&self.bilancio)
            .chain(&self.bilancio_approvato)
            .chain(&self.bilancio_presentato)
    }

    /// Invarianti dei conti che un salvataggio manomesso potrebbe rompere. L'ordine conta:
    /// i limiti sugli importi vengono prima delle somme che li usano.
    fn controlla_coerenza(&self, data: Trimestre) -> Result<(), Errore> {
        let in_limite = |x: i64| x.abs() <= LIMITE_EURO;
        let (c, st) = (&self.conti, &self.sfc.stock);
        ok(data.anno <= 2200, "data.anno oltre il 2200")?;
        ok(
            (1..=8).contains(&self.storico.len()) && self.storico.iter().all(|t| t.pil > 0),
            "economia.storico: servono da 1 a 8 voci con pil > 0",
        )?;
        let somma = |f: fn(&Annata) -> f64| self.pensioni.iter().map(f).sum::<f64>();
        let limite = LIMITE_EURO as f64;
        ok(
            !self.pensioni.is_empty()
                && self
                    .pensioni
                    .iter()
                    .all(|a| a.pensionati >= 0 && a.importo >= 0)
                && somma(|a| a.pensionati as f64) <= limite
                && somma(|a| a.pensionati as f64 * a.importo as f64) <= limite,
            "economia.pensioni: vuote, negative, o con pensionati o spesa oltre 1e15",
        )?;
        for b in self.bilanci() {
            ok(
                b.aliquote.valida().is_ok()
                    && b.dicasteri.iter().all(|d| livello_ok(d.livello_prezzi)),
                "economia.bilancio: aliquote incoerenti o un livello_prezzi fuori da (0, 1e6]",
            )?;
            ok(
                (b.dicasteri.iter())
                    .flat_map(|d| [d.spesa, d.salari, d.acquisti, d.margine])
                    .chain([b.fondo_riserva])
                    .all(|x| (0..=LIMITE_EURO).contains(&x)),
                "economia.bilancio: una voce o il fondo_riserva fuori da 0-1e15",
            )?;
        }
        if let Some(b) = &self.bilancio_presentato {
            ok(
                economia::bozza(self, data).is_some_and(|z| z.anno == b.anno),
                "economia.bilancio_presentato fuori stagione o d'altro anno",
            )?;
        }
        ok(
            self.decreti.iter().all(|x| (0..=LIMITE_EURO).contains(x))
                && self.decreti.iter().sum::<i64>() <= LIMITE_EURO,
            "economia.decreti fuori da 0-1e15",
        )?;
        ok(
            self.sussidio_per_disoccupato >= 0,
            "economia.sussidio_per_disoccupato negativo",
        )?;
        ok(
            self.livello_prezzi.is_finite()
                && self.livello_prezzi > 0.0
                && self.livello_prezzi <= 1e6,
            "economia.livello_prezzi non finito o fuori da (0, 1e6]",
        )?;
        ok(
            self.tasso_medio_debito.is_finite(),
            "economia.tasso_medio_debito non finito",
        )?;
        ok(
            self.sfc.livello_prezzi_area.is_finite() && self.sfc.livello_prezzi_area > 0.0,
            "economia.sfc.livello_prezzi_area non finito o non positivo",
        )?;
        let importi = [
            self.debito,
            self.sussidio_per_disoccupato,
            c.entrate.irpef,
            c.entrate.ires,
            c.entrate.iva,
            c.entrate.contributi,
            c.entrate.altre,
            c.entrate.fondi_coesione,
            c.spese.dicasteri,
            c.spese.pensioni,
            c.spese.sussidi,
            c.spese.altre_prestazioni,
            c.spese.interessi,
            c.spese.ammende,
            c.spese.disposizioni,
            st.famiglie,
            st.imprese,
            st.stato,
            st.estero,
        ];
        ok(
            importi.into_iter().all(in_limite),
            "economia: un importo supera 1e15 euro",
        )?;
        ok(
            st.stato == -self.debito,
            "economia.sfc.stock.stato diverso dal debito cambiato di segno",
        )?;
        ok(
            self.residuo_stock() == 0,
            "economia.sfc.stock non somma a zero",
        )?;
        ok(
            self.disavanzo_trimestre == c.disavanzo(),
            "economia.disavanzo_trimestre diverso da quello dei conti",
        )
    }
}

#[cfg(test)]
#[expect(
    clippy::unwrap_used,
    reason = "nei test un unwrap che fallisce e' il fallimento voluto"
)]
mod tests {
    use super::*;
    use crate::caso::Flusso;
    use crate::dati::dati_base;
    use crate::legislatura::Fondazione;

    fn fondata() -> Partita {
        let f = Fondazione {
            nome: "Partito nuovo".into(),
            posto: 5.5,
        };
        Partita::fonda(dati_base(), 7, &f).unwrap()
    }

    #[test]
    fn il_voto_porta_il_voto_precedente_alle_intenzioni() {
        let mut p = fondata();
        // Alla fondazione voto precedente e intenzioni coincidono: la valence le separa.
        p.partiti[p.giocatore].valence += 1.0;
        let vero = p.intenzioni_vere();
        let precedente = |p: &Partita| -> Vec<Vec<f64>> {
            (p.elettorato.gruppi.iter())
                .map(|g| g.voto_precedente.clone())
                .collect()
        };
        assert_ne!(precedente(&p), vero.per_gruppo);
        p.vota_parlamento(&vero);
        assert_eq!(precedente(&p), vero.per_gruppo);
    }

    /// Bias nuovo: `n + 1` normali di `Flusso::Sondaggi`, nient'altro dal `Caso`.
    #[test]
    fn il_voto_riestrae_il_bias_comune() {
        let mut p = fondata();
        let vero = p.intenzioni_vere();
        let (bias, mut caso) = (p.informazione.bias.clone(), p.caso.clone());
        p.vota_parlamento(&vero);
        let s = p.dati.manopola("sigma_bias_comune");
        let attesi: Vec<f64> = (0..=p.partiti.len())
            .map(|_| s * caso.normale(Flusso::Sondaggi))
            .collect();
        assert_ne!(p.informazione.bias, bias);
        assert_eq!(p.informazione.bias, attesi);
        assert_eq!(p.caso, caso);
    }
}
