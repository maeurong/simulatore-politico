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
    self, Assegnazione, Cessione, ConfrontoIndicatore, Dichiarazione, Eredita, EsitoElezioni, Fase,
    Fine, Incarico, Legislatura, Quadro, VoceDisponibilita, VoceRichiesta, VotiPartito,
    coalizioni_al_voto,
};
use crate::ministri::{Causa, Contesto, Ministri, TipoProposta, causa_peggiore, penalizza};
use crate::misure::{CAPACITA_NEUTRA, Misure};
use crate::parlamento::{
    self, CausaIntesa, Conteggio, Decreto, EsitoReferendum, EsitoVoto, FondiDecreto, FormaMisura,
    Intesa, Misura, Parlamento, Referendum, RichiestaEmendamento, StatoAula, StatoEmendamento,
    TipoReferendum, ValoreProposto, voto,
};
use crate::partiti::{self, Governo, ID_GIOCATORE, Partito};
use crate::vista::{
    Anteprima, AnteprimaSfiducia, FasciaIntesa, SeggiPartito, Vista, VistaBilancio,
    VistaReferendum, VistaStatistica, VoceAlleato, VoceBudget, VoceDicastero, VoceFascia,
    VoceGrandezza, VoceIncarico, VoceMinistero, VocePartito,
};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub const VERSIONE_SCHEMA: u32 = 8;

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
    /// Chiusure iniziate col giocatore al governo.
    turni_al_governo: u32,
    /// Le stringhe `legge` delle sue leggi, scritte quando entrano in vigore.
    leggi_del_giocatore: Vec<String>,
    /// I quattro indicatori al turno zero (`indicatori`).
    partenza: [f64; 4],
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
                allargati: Vec::new(),
                dimissioni: false,
                maggioranza: None,
                crisi: None,
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
                allargati: Vec::new(),
                dimissioni: false,
                maggioranza: None,
                crisi: None,
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
        let mut p = Partita {
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
            turni_al_governo: 0,
            leggi_del_giocatore: Vec::new(),
            partenza: [0.0; 4],
        };
        p.partenza = p.indicatori();
        p
    }

    /// Debito in % del PIL nominale annuo, spread in punti base, disoccupazione nazionale e
    /// inflazione annualizzata dell'economia vera: stessi campi al turno zero e a fine partita.
    fn indicatori(&self) -> [f64; 4] {
        let e = &self.economia;
        [
            100.0 * e.debito as f64 / (4.0 * e.pil_trimestrale * 1e9),
            e.spread,
            e.disoccupazione_nazionale(&self.dati),
            e.inflazione[0],
        ]
    }

    // Ordine della chiusura:
    // bilancio del governo IA in ottobre-dicembre e nell'esercizio provvisorio → default delle
    // decisioni (proposte, emendamenti aperti respinti tranne quelli delle leggi che slittano) →
    // atti dei ministri → aula (leggi slittate nel turno del bilancio → conversioni → bilancio →
    // leggi, con le dimissioni dopo la fiducia respinta → referendum) → esercizio provvisorio
    // senza bilancio, dimissioni → motore (misure, economia, grandezze, redditi, giudizio [prima di
    // composizione, che aggiorna ultima_disoccupazione], composizione, posizioni, sondaggio,
    // statistica) → voto, nel turno del voto → soglie (ministri, ritorno dell'intesa) → uscite
    // degli alleati (rimpasto o crisi), fuori dal turno del voto e senza dimissioni → fine della
    // crisi (rimpasto o sfiducia) → Capo dello Stato se ci sono dimissioni, fuori dal turno del
    // voto (che le azzera), col calendario del governo caduto che decade → governo nuovo,
    // maggioranza contigua o scioglimento, nel turno di trattativa fuori dal turno del voto →
    // calendario →
    // apertura (valori in arrivo, esiti dei due turni prima, emendamenti, rose, agende, proposte;
    // il costo del taglio se l'anno entra in esercizio provvisorio).
    // A partita finita la chiusura non fa nulla; a Camere sciolte l'aula vota solo i
    // confermativi: il calendario si è svuotato alla caduta o al voto, e nessuno può presentare.
    pub fn chiudi_turno(&mut self) {
        if self.finita() {
            return;
        }
        if self.governa() {
            self.turni_al_governo = self.turni_al_governo.saturating_add(1);
        }
        let trattativa = self.legislatura.fase == Fase::Trattativa;
        // ponytail: governo IA minimo, il piano 8 lo sostituisce. Le proposte dei suoi ministri
        // restano ignorate; in ottobre-dicembre e nell'esercizio provvisorio presenta la bozza del
        // bilancio, se c'è, tranne in crisi (decisione 2 di Mario, per chiunque governi).
        if self.legislatura.fase == Fase::Ordinaria
            && !self.governa()
            && !self.in_crisi()
            && (self.data.trimestre == 4 || self.economia.esercizio_provvisorio(self.data))
            && self.economia.bilancio_presentato.is_none()
            && let Some(b) = self.bozza_bilancio()
        {
            // La bozza passa le regole della presentazione; se non le passasse, nessun bilancio.
            let _ = self.presenta_bilancio(&b);
        }
        self.con_contesto(|ministri, cx, _| ministri.decidi_proposte(cx));
        // Prima del voto del bilancio: approvato, esce dal calendario. In crisi le leggi restano
        // ferme un turno senza decadere, come nel turno del bilancio (ticket 09, riga 45).
        let occupato = self.turno_del_bilancio() || self.in_crisi();
        let respinti = self.parlamento.respingi_aperti(self.turno, occupato);
        self.emendamenti_respinti(respinti);
        let effetti = self.con_contesto(|ministri, cx, _| ministri.esegui(cx));
        self.grandezze.effetti.extend(effetti);
        if self.legislatura.fase == Fase::Sciolta {
            self.vota_confermativi();
        } else {
            self.aula(occupato);
        }
        // Esercizio provvisorio (iter-legislativo-italiano:86): un turno solo, poi il governo cade,
        // col bilancio respinto come con quello mai presentato.
        if self.legislatura.fase == Fase::Ordinaria
            && self.economia.esercizio_provvisorio(self.data)
        {
            self.legislatura.dimissioni = true;
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
        let di_voto = self.legislatura.turno_di_voto(self.turno);
        if di_voto {
            self.vota_parlamento(&vero);
        }
        // Prima delle soglie dei ministri: chi si dimette nel turno conta ancora nella media.
        self.pressioni_intesa();
        self.con_contesto(|ministri, cx, _| ministri.soglie(cx));
        let escono = (self.intesa).soglie(&self.dati, self.turno, &self.alleati());
        let ritorno = self.dati.manopola("ritorno_valence");
        for p in &mut self.partiti {
            p.valence += ritorno * (p.valence_base - p.valence);
        }
        // Uscite e crisi (scelta 10 del piano 7b); nel turno del voto nessuna uscita, e la crisi
        // l'ha già chiusa `vota_parlamento`.
        let l = &self.legislatura;
        if l.fase == Fase::Ordinaria && !l.dimissioni && !di_voto {
            self.uscite(&escono);
        }
        if self.legislatura.crisi == Some(self.turno) {
            self.fine_della_crisi();
        }
        // Nel turno del voto vince il voto: le dimissioni si azzerano senza effetto.
        if std::mem::take(&mut self.legislatura.dimissioni) && !di_voto {
            self.capo_dello_stato();
        }
        if trattativa && !di_voto {
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
            ultimatum: &self.intesa.ultimatum,
            coalizioni: self.coalizioni_correnti(),
            giocatore: self.giocatore,
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

    /// Esegue `f` sui ministri e sul generatore (in scrittura) con ciò che leggono del resto della
    /// partita.
    pub(crate) fn con_contesto<R>(
        &mut self,
        f: impl FnOnce(&mut Ministri, &Contesto, &mut Caso) -> R,
    ) -> R {
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
        f(&mut self.ministri, &cx, &mut self.caso)
    }

    /// `occupato`: il turno del bilancio o di crisi, letto prima di ogni voto.
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
        // Dopo `esegui`: il ministro sfiduciato non ha più un decreto in agenda.
        if let Some((dicastero, promotore)) = self.parlamento.sfiducia.take() {
            self.sfiducia(dicastero, Some(promotore));
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
    /// Consiglio, tranne sulla proposta dell'opposizione, che paga solo il blocco. Chi vota sì
    /// sposta la sua posizione percepita verso la misura; il giocatore con la linea scelta, verso
    /// la posizione che il suo voto sostiene.
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
                    del_giocatore: self.del_giocatore(&m),
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
            // La fiducia respinta fa cadere la misura senza blocco e il governo
            // (iter-legislativo-italiano:42: le dimissioni sono prassi, qui automatiche).
            if fiducia_respinta {
                self.legislatura.dimissioni = true;
            } else {
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
            // La proposta dell'opposizione respinta non costa nulla (scelta 15 del piano 7b).
            if let Some(pdc) =
                (self.partiti.get_mut(self.governo.presidente)).filter(|_| m.promotore.is_none())
            {
                pdc.valence -= self.dati.manopola(valence);
            }
            if let Some(f) = m.fondi {
                self.decreto_decaduto(f);
            }
        }
        // Il giocatore con la linea scelta scivola verso la posizione che il suo voto sostiene: la
        // nuova col sì, la vigente col no (scelta 14 del piano 7b).
        if let Some((vigore, nuova)) = p.posizioni {
            let s = self.dati.manopola("scivolamento_percepita");
            let scelta = m.linea_giocatore.map(|_| self.giocatore);
            let voti = self.partiti.iter_mut().zip(&p.linea).enumerate();
            for (_, (x, &si)) in voti.filter(|&(j, (_, &si))| si || scelta == Some(j)) {
                let sostenuta = if si { nuova } else { vigore };
                x.posizione_percepita += s * (sostenuta - x.posizione_percepita);
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
        if self.del_giocatore(m) {
            self.leggi_del_giocatore.push(legge.clone());
        }
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

    /// La misura è del giocatore: sua proposta d'opposizione, o del governo che presiede.
    fn del_giocatore(&self, m: &Misura) -> bool {
        match m.promotore {
            Some(j) => j == self.giocatore,
            None => self.governo.presidente == self.giocatore,
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
                if r.del_giocatore {
                    self.leggi_del_giocatore.push(legge.clone());
                }
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
        let indicizzato = self.bilancio_emendato(misura, partito, accogli)?;
        let respinto = (self.parlamento).decidi_emendamento(
            &self.dati,
            &self.partiti,
            misura,
            partito,
            accogli,
        )?;
        if indicizzato.is_some() {
            self.economia.bilancio_presentato = indicizzato;
        }
        self.emendamenti_respinti(respinto);
        Ok(())
    }

    /// Accogliere l'emendamento aperto di `partito` sul bilancio presentato: il bilancio con i
    /// suoi dicasteri indicizzati. `None` per ogni altro caso (gli errori li dà
    /// `decidi_emendamento`); un bilancio non ammesso è un rifiuto e la partita resta identica.
    fn bilancio_emendato(
        &self,
        misura: u32,
        partito: &str,
        accogli: bool,
    ) -> Result<Option<Bilancio>, Errore> {
        let j = self.partiti.iter().position(|p| p.id == partito);
        let proposta = (self.parlamento.misure.iter())
            .filter(|m| m.id == misura && m.forma == FormaMisura::Bilancio)
            .flat_map(|m| &m.emendamenti)
            .find(|e| Some(e.partito) == j && e.stato == StatoEmendamento::Aperto);
        let (true, Some(e), Some(b)) = (accogli, proposta, &self.economia.bilancio_presentato)
        else {
            return Ok(None);
        };
        let k: Vec<usize> = e.proposta.iter().map(|p| p.0).collect();
        self.indicizza_dicasteri(b, &k).map(Some)
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
            ultimatum: &self.intesa.ultimatum,
            coalizioni: self.coalizioni_correnti(),
            giocatore: self.giocatore,
        }
    }

    /// `ChiediDimissioni` accolto sul dicastero `id`: se il ministro è di un alleato e non è un
    /// tecnico, l'alleato perde `costo_dimissioni_chieste`.
    pub(crate) fn dimissioni_chieste(&mut self, id: &str) {
        let i = self.dati.dicasteri().iter().position(|d| d.id == id);
        let ministro = i.and_then(|i| self.ministri.dicasteri.get(i)?.ministro.as_ref());
        let Some(m) = ministro.filter(|m| !m.tecnico && voto::alleato(&self.governo, m.partito))
        else {
            return;
        };
        let (j, delta) = (m.partito, -self.dati.manopola("costo_dimissioni_chieste"));
        (self.intesa).varia(j, CausaIntesa::DimissioniChieste, delta, self.turno);
    }

    /// Pressioni di fine turno sull'intesa di ogni alleato del governo in carica: ministri
    /// scontenti, quota dei dicasteri sotto la richiesta di Gamson sui seggi veri, calo nei
    /// sondaggi rispetto all'ultimo voto (cicli-politico-economici-e-luna-di-miele:21). Una
    /// variazione si registra solo se il delta non è zero. Solo in fase ordinaria: in trattativa
    /// il governo uscente è in interim, e alla formazione l'intesa riparte comunque. Nel turno di
    /// crisi niente quota scoperta: i dicasteri dell'uscente sono in interim presso il presidente
    /// fino al rimpasto, e farli pagare agli alleati rimasti darebbe uscite a cascata.
    fn pressioni_intesa(&mut self) {
        if self.legislatura.fase != Fase::Ordinaria {
            return;
        }
        let d = |nome| self.dati.manopola(nome);
        let (minaccia, costo_ministri) = (d("soglia_minaccia"), d("costo_ministri_scontenti"));
        let (costo_quota, costo_calo) = (d("costo_quota_scoperta"), d("costo_calo_sondaggi"));
        let (g, crisi) = (&self.governo, self.in_crisi());
        let mut variazioni = Vec::new();
        for j in self.alleati() {
            let sodd: Vec<f64> = (self.ministri.dicasteri.iter())
                .filter_map(|st| st.ministro.as_ref())
                .filter(|m| m.partito == j)
                .map(|m| m.soddisfazione)
                .collect();
            if !sodd.is_empty() {
                let media = sodd.iter().sum::<f64>() / sodd.len() as f64;
                let delta = -costo_ministri * (minaccia - media).max(0.0) / 10.0;
                variazioni.push((j, CausaIntesa::MinistriScontenti, delta));
            }
            if !crisi {
                let assegnati = g.dicasteri.iter().copied().map(Some);
                let (richiesta, offerta) = self.richiesta_e_offerta(j, &g.coalizione, assegnati);
                let delta = -costo_quota * (richiesta - offerta).max(0.0);
                variazioni.push((j, CausaIntesa::QuotaScoperta, delta));
            }
            if let Some(voti) = self
                .legislatura
                .elezioni
                .first()
                .and_then(|e| e.voti.get(j))
            {
                let sondaggio = legislatura::quota_fra_i_votanti(&self.informazione.sondaggio, j);
                let delta = -costo_calo * (voti.quota - sondaggio).max(0.0) * 100.0;
                variazioni.push((j, CausaIntesa::SondaggiInCalo, delta));
            }
        }
        for (j, causa, delta) in variazioni {
            if delta != 0.0 {
                (self.intesa).varia(j, causa, delta, self.turno);
            }
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
    /// `Comando` come `PresentaBilancio` se il giocatore non governa; `Bilancio` fuori stagione,
    /// con le stesse regole della presentazione, o con la spesa netta di riferimento nulla.
    pub fn riepilogo_bilancio(&self, b: &Bilancio) -> Result<RiepilogoBilancio, Errore> {
        self.di_governo()?;
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
    /// Col giocatore al governo, ogni alleato chiede di indicizzare i suoi dicasteri in perdita
    /// reale (`emendamenti_bilancio`); gli emendamenti accolti si riapplicano al bilancio nuovo.
    pub(crate) fn presenta_bilancio(&mut self, b: &Bilancio) -> Result<(), Errore> {
        let (_, mut b) = self.bilancio_ammesso(b)?;
        let turno = self.turno;
        let accolti: Vec<usize> = (self.parlamento.misure.iter())
            .filter(|m| m.forma == FormaMisura::Bilancio && m.voto == turno)
            .flat_map(|m| &m.emendamenti)
            .filter(|e| e.stato == StatoEmendamento::Accolto)
            .flat_map(|e| e.proposta.iter().map(|p| p.0))
            .collect();
        if !accolti.is_empty() {
            b = self.indicizza_dicasteri(&b, &accolti)?;
        }
        self.economia.bilancio_presentato = Some(b);
        self.parlamento.apri_bilancio(turno);
        if self.governa() {
            let nuovi = self.richieste_di_indicizzazione();
            self.parlamento.emendamenti_bilancio(turno, nuovi);
        }
        Ok(())
    }

    /// `b` con i budget dei dicasteri `k` indicizzati, come lo approverebbe l'aula.
    fn indicizza_dicasteri(&self, b: &Bilancio, k: &[usize]) -> Result<Bilancio, Errore> {
        let mut b = b.clone();
        for d in k.iter().filter_map(|&k| self.dati.dicasteri().get(k)) {
            b = self.indicizza(&b, &Voce::Dicastero { id: d.id.clone() })?;
        }
        Ok(self.bilancio_ammesso(&b)?.1)
    }

    /// Una richiesta per ogni alleato che tiene dicasteri la cui voce perde più di
    /// `soglia_emendamento_bilancio` punti reali nel bilancio presentato.
    fn richieste_di_indicizzazione(&self) -> Vec<RichiestaEmendamento> {
        let Some(b) = &self.economia.bilancio_presentato else {
            return Vec::new();
        };
        let (indice, soglia) = (
            self.indice_prezzi(),
            self.dati.manopola("soglia_emendamento_bilancio"),
        );
        let perdite = |j: usize| -> Vec<(usize, f64)> {
            (b.dicasteri.iter().enumerate())
                .filter(|&(k, _)| self.governo.dicasteri.get(k) == Some(&j))
                .map(|(k, d)| (k, perdita_reale(d.livello_prezzi, indice)))
                .filter(|&(_, x)| x > soglia)
                .collect()
        };
        (self.alleati().into_iter())
            .map(|j| (j, perdite(j)))
            .filter(|(_, p)| !p.is_empty())
            .map(|(j, p)| parlamento::richiesta_bilancio(&self.dati, j, &p))
            .collect()
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
    /// referendum già in corso, fuori finestra (dopo luglio-settembre); prima di tutto, l'ultimo
    /// anno della legislatura (iter-legislativo-italiano:135), poi le Camere sciolte, poi i sei
    /// mesi dopo i comizi: il turno dopo un voto (stessa fonte).
    pub fn promuovi_referendum(
        &mut self,
        promotore: &str,
        disposizione: &str,
    ) -> Result<(), Errore> {
        if self.legislatura.ultimo_anno(self.turno) {
            return Err(Errore::Comando(
                "Nessuna richiesta di referendum nell'ultimo anno della legislatura".into(),
            ));
        }
        if self.legislatura.fase == Fase::Sciolta {
            return Err(Errore::Comando("Nessun referendum a Camere sciolte".into()));
        }
        let l = &self.legislatura;
        if !l.elezioni.is_empty() && self.turno <= l.inizio {
            return Err(Errore::Comando(
                "Nessuna richiesta di referendum nei sei mesi dopo la convocazione dei comizi"
                    .into(),
            ));
        }
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
        self.di_governo()?;
        self.fuori_dalla_crisi()?;
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
        self.di_governo()?;
        self.fuori_dalla_crisi()?;
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
        self.di_governo()?;
        self.fuori_dalla_crisi()?;
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

    /// Ciò che il giocatore lascia, a partita finita.
    pub fn eredita(&self) -> Option<Eredita> {
        let fine = self.fine?;
        let nomi = (self.dati.disposizioni().iter().zip(&self.disposizioni.voci))
            .filter(|(_, v)| self.leggi_del_giocatore.contains(&v.legge))
            .map(|(d, _)| d.nome.clone());
        let voci = [
            ("debito_pil", "Debito in % del PIL"),
            ("spread", "Spread"),
            ("disoccupazione", "Disoccupazione"),
            ("inflazione", "Inflazione"),
        ];
        let fine_partita = self.indicatori();
        Some(Eredita {
            fine,
            bozza: fine == Fine::SenzaSeggi,
            anni_al_governo: f64::from(self.turni_al_governo) / 4.0,
            leggi_in_vigore: nomi.collect(),
            indicatori: (voci.iter().zip(self.partenza).zip(fine_partita))
                .map(|((&(id, nome), turno_zero), fine)| ConfrontoIndicatore {
                    id: id.into(),
                    nome: nome.into(),
                    turno_zero,
                    fine,
                })
                .collect(),
        })
    }

    /// Il giocatore lascia la partita: l'eredità subito.
    ///
    /// # Errors
    /// `Comando` a partita finita.
    pub fn abbandona(&mut self) -> Result<Eredita, Errore> {
        if self.finita() {
            return Err(Errore::Comando("La partita è finita".into()));
        }
        self.fine = Some(Fine::Abbandono);
        self.eredita()
            .ok_or_else(|| Errore::Comando("La partita è finita".into()))
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
    /// sondaggi-ed-errore:50, :55): prima la fine della legislatura (`decadi_tutto`, gradini
    /// sbloccati, scelta 12), poi riparto delle intenzioni vere `vero` con le coalizioni al voto,
    /// la legge e i seggi dei rami in vigore, poi le cessioni; voto precedente = intenzioni, bias
    /// comune nuovo, legislatura nuova in trattativa, incarico. Riparto rifiutato: seggi di prima,
    /// esito con le quote senza seggi. Il giocatore senza seggi in entrambe le Camere: fine.
    fn vota_parlamento(&mut self, vero: &Intenzioni) {
        (self.parlamento).decadi_tutto(&self.dati, &self.disposizioni, self.data);
        for v in &mut self.disposizioni.voci {
            v.bloccati.clear();
        }
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
        // Il ribaltone e la crisi della vigilia del voto decadono col voto.
        l.maggioranza = None;
        l.allargati.clear();
        l.crisi = None;
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
            seggi_veri: None,
        }
    }

    /// Il `Quadro` coi seggi veri del parlamento.
    fn quadro_veri(&self) -> Quadro<'_> {
        let pl = &self.parlamento;
        Quadro {
            seggi_veri: Some((pl.seggi_camera.clone(), pl.seggi_senato.clone())),
            ..self.quadro()
        }
    }

    /// In trattativa con l'incarico al giocatore.
    fn con_incarico(&self) -> bool {
        let l = &self.legislatura;
        l.fase == Fase::Trattativa && l.incarico == Some(Incarico::Giocatore)
    }

    /// Nel turno di crisi.
    pub(crate) fn in_crisi(&self) -> bool {
        self.legislatura.crisi == Some(self.turno)
    }

    /// In crisi col giocatore al governo, fuori dal turno del voto (che chiude la crisi).
    fn in_crisi_al_governo(&self) -> bool {
        self.in_crisi() && self.governa() && !self.legislatura.turno_di_voto(self.turno)
    }

    /// In crisi al governo nell'esercizio provvisorio: in crisi il bilancio non si presenta, e il
    /// governo cade a fine turno (iter-legislativo-italiano:86).
    fn caduta_certa(&self) -> bool {
        self.in_crisi_al_governo() && self.economia.esercizio_provvisorio(self.data)
    }

    /// `Allarga` ammesso: in trattativa con l'incarico, o in crisi al governo se la caduta non è
    /// certa.
    fn allarga_ammesso(&self) -> bool {
        self.con_incarico() || (self.in_crisi_al_governo() && !self.caduta_certa())
    }

    /// La disponibilità di ogni partito IA sulla dichiarazione in corso; senza dichiarazione,
    /// sulla coalizione del giocatore dell'ultimo voto (alla fondazione, lui da solo). In
    /// trattativa con l'incarico o in crisi al governo, quella dei partiti fuori dalla coalizione
    /// a entrarvi; nessuna con la caduta certa.
    pub fn disponibilita(&self) -> Vec<VoceDisponibilita> {
        if self.caduta_certa() {
            return Vec::new();
        }
        if !self.allarga_ammesso() {
            return self.quadro().disponibilita(&self.dichiarazione_corrente());
        }
        let c = self.coalizione_del_giocatore();
        self.dopo_il_voto((0..self.partiti.len()).filter(|j| !c.contains(j)), &[])
    }

    /// La disponibilità dei partiti di `quali` a entrare nella coalizione del giocatore, allargata
    /// a `piu`, coi seggi del voto (scelta 9 del piano 7).
    fn dopo_il_voto(
        &self,
        quali: impl Iterator<Item = usize>,
        piu: &[usize],
    ) -> Vec<VoceDisponibilita> {
        let q = self.quadro_veri();
        let d = Dichiarazione {
            partiti: (self.coalizione_del_giocatore().into_iter())
                .filter(|&j| j != self.giocatore)
                .chain(piu.iter().copied())
                .collect(),
            cessioni: Vec::new(),
        };
        q.voci(&d, quali)
    }

    /// `Allarga`: in trattativa con l'incarico, o in crisi al governo, i partiti `ids` entrano
    /// insieme nella coalizione se ognuno è disponibile coi seggi veri e la lista intera
    /// (formazione-di-coalizioni:13).
    ///
    /// # Errors
    /// `Comando` col nome del partito (l'id se sconosciuto), e la partita resta identica: lista
    /// vuota, fuori dalla trattativa con l'incarico e dalla crisi, in crisi con la caduta certa,
    /// sconosciuto, ripetuto, già nella coalizione, o con la causa per ogni membro non
    /// disponibile.
    pub(crate) fn allarga(&mut self, ids: &[String]) -> Result<(), Errore> {
        let ko = |m: String| Err(Errore::Comando(m));
        if ids.is_empty() {
            return ko("allargamento senza partiti".into());
        }
        if !self.allarga_ammesso() {
            let nomi: Vec<&str> = (ids.iter())
                .map(|id| {
                    (self.partiti.iter().find(|p| &p.id == id))
                        .map_or(id.as_str(), |p| p.nome.as_str())
                })
                .collect();
            let motivo = if self.caduta_certa() {
                "allargamento inutile, senza bilancio in esercizio provvisorio il governo cade a \
                 fine turno"
            } else {
                "allargamento solo in trattativa con l'incarico, o in crisi mentre governi"
            };
            return ko(format!("{}: {motivo}", nomi.join(", ")));
        }
        let c = self.coalizione_del_giocatore();
        let mut lista = Vec::new();
        for id in ids {
            let Some(j) = self.partiti.iter().position(|p| &p.id == id) else {
                return ko(format!("Partito sconosciuto: {id}"));
            };
            let nome = &self.partiti[j].nome;
            if c.contains(&j) {
                return ko(format!("{nome} è già nella coalizione"));
            }
            if lista.contains(&j) {
                return ko(format!("{nome} ripetuto nell'allargamento"));
            }
            lista.push(j);
        }
        let voci = self.dopo_il_voto(lista.iter().copied(), &lista);
        let fuori: Vec<String> = (lista.iter().zip(voci))
            .filter(|(_, v)| !v.disponibile)
            .map(|(&j, v)| format!("{} non entra: {}", self.partiti[j].nome, v.causa))
            .collect();
        if !fuori.is_empty() {
            return ko(fuori.join("; "));
        }
        // La proposta fatta senza di loro decade: le richieste tornano alla spartizione di default.
        self.legislatura.allargati.extend(lista);
        self.legislatura.proposta = None;
        Ok(())
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
        if !self.con_incarico() {
            return ko("Spartizione: si propone solo in trattativa con l'incarico".into());
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

    /// La coalizione del giocatore: la maggioranza del ribaltone, se c'è, altrimenti quella
    /// dell'ultimo voto (lui da solo se non ne ha); in crisi al governo quella di governo; più gli
    /// allargati.
    fn coalizione_del_giocatore(&self) -> Vec<usize> {
        let (g, l) = (self.giocatore, &self.legislatura);
        let mut c = (l.maggioranza.clone())
            .or_else(|| {
                (self.in_crisi() && self.governa()).then(|| self.governo.coalizione.clone())
            })
            .or_else(|| l.coalizioni.iter().find(|c| c.contains(&g)).cloned())
            .unwrap_or_else(|| vec![g]);
        c.extend(&l.allargati);
        c
    }

    /// Seggi di Camera più Senato per partito.
    fn seggi_per_partito(&self) -> Vec<u32> {
        let pl = &self.parlamento;
        (pl.seggi_camera.iter().zip(&pl.seggi_senato))
            .map(|(c, s)| c + s)
            .collect()
    }

    /// (richiesta, offerta) del partito `j` nella `coalizione`: la richiesta di Gamson sui seggi
    /// veri, e i punti di peso dei dicasteri che `assegnati` gli dà.
    fn richiesta_e_offerta(
        &self,
        j: usize,
        coalizione: &[usize],
        assegnati: impl Iterator<Item = Option<usize>>,
    ) -> (f64, f64) {
        let seggi = self.seggi_per_partito();
        let di = |j: usize| seggi.get(j).copied().unwrap_or(0);
        let offerta = (self.dati.dicasteri().iter().zip(assegnati))
            .filter(|&(_, a)| a == Some(j))
            // Da 0,0: la `sum` di un f64 vuoto dà -0,0.
            .fold(0.0, |a, (d, _)| a + f64::from(d.peso_spartizione));
        let s = coalizione.iter().map(|&k| di(k)).sum();
        (legislatura::richiesta(&self.dati, di(j), s), offerta)
    }

    /// Per alleato della coalizione del giocatore: (indice, richiesta, offerta di `assegnati`, i
    /// punti di peso dei dicasteri che gli dà).
    fn quote(&self, assegnati: &[Option<usize>]) -> Vec<(usize, f64, f64)> {
        let coalizione = self.coalizione_del_giocatore();
        (coalizione.iter().filter(|&&j| j != self.giocatore))
            .map(|&j| {
                let (r, o) = self.richiesta_e_offerta(j, &coalizione, assegnati.iter().copied());
                (j, r, o)
            })
            .collect()
    }

    /// La spartizione del giocatore: la sua proposta o, senza, quella di default.
    fn assegnati_dal_giocatore(&self) -> Vec<Option<usize>> {
        self.legislatura.proposta.clone().unwrap_or_else(|| {
            let c = self.coalizione_del_giocatore();
            let seggi = self.seggi_per_partito();
            let nessuno = vec![None; self.dati.dicasteri().len()];
            let v = legislatura::spartizione_di_default(
                &self.dati,
                &c,
                self.giocatore,
                &seggi,
                &nessuno,
            );
            v.into_iter().map(Some).collect()
        })
    }

    /// In trattativa con l'incarico al giocatore, una voce per alleato: richiesta e offerta della
    /// proposta in corso o, senza, di quella di default che la chiusura applicherà; vuota altrove.
    pub fn richieste(&self) -> Vec<VoceRichiesta> {
        if !self.con_incarico() {
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

    /// Il governo nuovo alla chiusura della trattativa, dopo le soglie. Con `Guida` ancora aperta
    /// chiede la guida come `ChiediGuida`: ceduta, l'incarico è del giocatore. Con l'incarico al
    /// giocatore la sua proposta (o quella di default), i tecnici al suo partito, e ogni alleato
    /// riparte da `intesa_base + intesa_per_punto · (offerta − richiesta)`; con l'incarico IA il
    /// primo partito per seggi e la spartizione di default, e ogni alleato riparte da
    /// `intesa_base` senza variazione registrata. Senza maggioranza del giocatore (con
    /// gli allargati) né incarico IA, il Capo dello Stato cerca la maggioranza contigua senza il
    /// giocatore (formazione-di-coalizioni:13), governo IA come sopra; senza nemmeno quella
    /// scioglie le Camere: si vota nella chiusura del turno dopo. Poi ministri tutti nuovi,
    /// giudizio da zero per chi entra al governo, l'ultimatum sul valore di partenza (l'alleato
    /// che entra sotto la soglia lo pone subito, chi non è più alleato non ne ha), fase ordinaria.
    fn forma_governo(&mut self) {
        // Il ribaltone senza scelta chiede la guida (decisione 1 di Mario); rifiutata, o senza
        // `Guida` aperta, nulla cambia.
        let _ = self.chiedi_guida();
        let seggi = self.seggi_per_partito();
        let (camera, senato) = (&self.parlamento.seggi_camera, &self.parlamento.seggi_senato);
        let sua = self.coalizione_del_giocatore();
        let (presidente, coalizione, assegnati) = if self.legislatura.incarico
            == Some(Incarico::Giocatore)
            && legislatura::maggioranza(camera, senato, &sua)
        {
            (self.giocatore, sua, self.assegnati_dal_giocatore())
        } else {
            let ia = match &self.legislatura.incarico {
                Some(Incarico::Ia { coalizione }) => Some(coalizione.clone()),
                _ => legislatura::maggioranza_contigua(
                    &self.partiti,
                    camera,
                    senato,
                    Some(self.giocatore),
                ),
            };
            let Some(coalizione) = ia else {
                let l = &mut self.legislatura;
                l.fase = Fase::Sciolta;
                l.incarico = None;
                l.proposta = None;
                l.allargati.clear();
                l.maggioranza = None;
                return;
            };
            let guida = (self.primo_per_seggi(&coalizione)).unwrap_or(self.giocatore);
            let nessuno = vec![None; self.dati.dicasteri().len()];
            let v = legislatura::spartizione_di_default(
                &self.dati,
                &coalizione,
                guida,
                &seggi,
                &nessuno,
            );
            (guida, coalizione, v.into_iter().map(Some).collect())
        };
        if presidente == self.giocatore {
            let (base, per_punto) = (
                self.dati.manopola("intesa_base"),
                self.dati.manopola("intesa_per_punto"),
            );
            for (j, richiesta, offerta) in self.quote(&assegnati) {
                self.intesa.riparti(j, base);
                let delta = per_punto * (offerta - richiesta);
                (self.intesa).varia(j, CausaIntesa::Spartizione, delta, self.turno);
            }
        } else {
            let base = self.dati.manopola("intesa_base");
            for &j in coalizione.iter().filter(|&&j| j != presidente) {
                self.intesa.riparti(j, base);
            }
        }
        self.paga_rotture(&coalizione, &coalizione, presidente);
        let entranti: Vec<usize> = (coalizione.iter().copied())
            .filter(|&j| !self.governo.al_governo(j))
            .collect();
        self.giudizio_da_zero(&entranti);
        let tecnici: Vec<bool> = assegnati.iter().map(Option::is_none).collect();
        self.governo = Governo {
            presidente,
            dicasteri: assegnati.iter().map(|a| a.unwrap_or(presidente)).collect(),
            coalizione,
        };
        self.con_contesto(|ministri, cx, caso| ministri.nuovo_governo(cx, caso, &tecnici));
        let alleati = self.alleati();
        self.intesa.ultimatum(&self.dati, &alleati);
        let l = &mut self.legislatura;
        l.fase = Fase::Ordinaria;
        l.incarico = None;
        l.proposta = None;
        l.allargati.clear();
        l.maggioranza = None;
    }

    /// Il giudizio sul governo riparte da zero in ogni gruppo per chi entra al governo.
    fn giudizio_da_zero(&mut self, entranti: &[usize]) {
        for gruppo in &mut self.elettorato.gruppi {
            for &j in entranti {
                if let Some(x) = gruppo.giudizio.get_mut(j) {
                    *x = 0.0;
                }
            }
        }
    }

    /// Rottura dell'impegno, simmetrica (scelta 3 del piano 7b): chi della `coalizione` rompe con
    /// gli `entranti` perde `valence_rottura_impegno`; l'alleato del `presidente` anche
    /// `intesa_rottura_impegno`. Paga anche il giocatore.
    fn paga_rotture(&mut self, coalizione: &[usize], entranti: &[usize], presidente: usize) {
        let (valence, intesa) = (
            self.dati.manopola("valence_rottura_impegno"),
            self.dati.manopola("intesa_rottura_impegno"),
        );
        for j in legislatura::rotture(&self.legislatura.coalizioni, coalizione, entranti) {
            if let Some(p) = self.partiti.get_mut(j) {
                p.valence -= valence;
            }
            if j != presidente {
                (self.intesa).varia(j, CausaIntesa::RotturaImpegno, -intesa, self.turno);
            }
        }
    }

    /// Escono dal governo gli alleati `escono` (sotto `soglia_uscita`): i loro ministri si
    /// dimettono (`UscitaDalGoverno`), i dicasteri passano al presidente in interim, l'ultimatum
    /// cade. Se la coalizione rimasta ha la maggioranza, rimpasto; altrimenti il turno dopo è di
    /// crisi (una crisi già aperta non si allunga).
    fn uscite(&mut self, escono: &[usize]) {
        if escono.is_empty() {
            return;
        }
        let g = &mut self.governo;
        for &j in escono {
            g.coalizione.retain(|&k| k != j);
            if let Some(u) = self.intesa.ultimatum.get_mut(j) {
                *u = None;
            }
            for (i, d) in g.dicasteri.iter_mut().enumerate() {
                if *d == j {
                    *d = g.presidente;
                    self.ministri.dimetti(i, Causa::UscitaDalGoverno);
                }
            }
        }
        let pl = &self.parlamento;
        if legislatura::maggioranza(&pl.seggi_camera, &pl.seggi_senato, &g.coalizione) {
            self.rimpasto();
        } else if self.legislatura.crisi.is_none() {
            self.legislatura.crisi = Some(self.turno.saturating_add(1));
        }
    }

    /// Il rimpasto (formazione-di-coalizioni:16, Gamson ricalcolata): gli alleati tengono i loro
    /// dicasteri, quelli del presidente si ridistribuiscono con la spartizione di default al netto
    /// del tenuto. Il dicastero che cambia partito va in interim (`Rimpasto`).
    fn rimpasto(&mut self) {
        let g = &self.governo;
        let tenuti: Vec<Option<usize>> = (g.dicasteri.iter())
            .map(|&j| (j != g.presidente && g.coalizione.contains(&j)).then_some(j))
            .collect();
        let seggi = self.seggi_per_partito();
        let v = legislatura::spartizione_di_default(
            &self.dati,
            &g.coalizione,
            g.presidente,
            &seggi,
            &tenuti,
        );
        for (i, j) in v.into_iter().enumerate() {
            if let Some(d) = self.governo.dicasteri.get_mut(i)
                && *d != j
            {
                *d = j;
                self.ministri.dimetti(i, Causa::Rimpasto);
            }
        }
    }

    /// La fine della crisi (scelta 10 del piano 7b). Con la maggioranza, coalizione più allargati:
    /// gli allargati entrano al governo come alla formazione (giudizio da zero, intesa alla base
    /// senza ultimatum), chi rompe l'impegno paga, rimpasto, ultimatum degli allargati sul valore
    /// corrente. Senza, la sfiducia automatica; respinta, la crisi si riapre il turno dopo. Non si
    /// vota se il governo è già dimissionario.
    fn fine_della_crisi(&mut self) {
        let l = &mut self.legislatura;
        l.crisi = None;
        let allargati = std::mem::take(&mut l.allargati);
        if l.dimissioni {
            return;
        }
        let mut coalizione = self.governo.coalizione.clone();
        coalizione.extend(&allargati);
        let pl = &self.parlamento;
        if !legislatura::maggioranza(&pl.seggi_camera, &pl.seggi_senato, &coalizione) {
            // Respinta, niente governo di minoranza (spec riga 336): la crisi si riapre.
            if !self.sfiducia(None, None) {
                self.legislatura.crisi = Some(self.turno.saturating_add(1));
            }
            return;
        }
        let base = self.dati.manopola("intesa_base");
        for &j in &allargati {
            self.intesa.riparti(j, base);
        }
        self.giudizio_da_zero(&allargati);
        self.paga_rotture(&coalizione, &allargati, self.governo.presidente);
        self.governo.coalizione = coalizione;
        self.rimpasto();
        // Solo gli allargati: l'ultimatum posto dalle soglie agli altri resta.
        self.intesa.ultimatum_di(&self.dati, &allargati);
    }

    /// `Dimettiti`: il governo cade alla chiusura del turno.
    pub(crate) fn dimettiti(&mut self) {
        self.legislatura.dimissioni = true;
    }

    /// Il Capo dello Stato dopo le dimissioni: la maggioranza contigua
    /// (formazione-di-coalizioni:13), senza il giocatore se presiedeva lui, con tutti se il
    /// governo era IA (scelta 11 del piano 7b). Senza il giocatore, l'incarico IA; col giocatore
    /// primo per seggi (Camera più Senato, a parità l'indice più basso), l'incarico a lui con
    /// quella maggioranza; col giocatore non primo, `Guida`. Trovata, si va in trattativa; senza,
    /// le Camere si sciolgono. Nessuna luna di miele al governo che nasce. Prima, le misure in
    /// calendario del governo caduto decadono senza costi, come a fine legislatura
    /// (iter-legislativo-italiano:80): una sua legge non costa al successore. I referendum restano.
    fn capo_dello_stato(&mut self) {
        let decaduta = "Decaduta per la caduta del governo";
        (self.parlamento).decadi_calendario(&self.dati, self.data, decaduta);
        let g = self.giocatore;
        let escluso = (self.governo.presidente == g).then_some(g);
        let pl = &self.parlamento;
        let trovata = legislatura::maggioranza_contigua(
            &self.partiti,
            &pl.seggi_camera,
            &pl.seggi_senato,
            escluso,
        );
        let Some(coalizione) = trovata else {
            self.legislatura.fase = Fase::Sciolta;
            return;
        };
        let primo = self.primo_per_seggi(&coalizione);
        let l = &mut self.legislatura;
        l.fase = Fase::Trattativa;
        l.incarico = Some(if !coalizione.contains(&g) {
            Incarico::Ia { coalizione }
        } else if primo == Some(g) {
            l.maggioranza = Some(coalizione);
            Incarico::Giocatore
        } else {
            Incarico::Guida { coalizione }
        });
    }

    /// Il primo della `coalizione` per seggi (Camera più Senato), a parità l'indice più basso.
    fn primo_per_seggi(&self, coalizione: &[usize]) -> Option<usize> {
        let pl = &self.parlamento;
        legislatura::primo_per_seggi(coalizione, &pl.seggi_camera, &pl.seggi_senato)
    }

    /// La coalizione del ribaltone, in trattativa con `Guida`.
    ///
    /// # Errors
    /// `Comando` senza `Guida` aperta.
    fn guida_aperta(&self) -> Result<Vec<usize>, Errore> {
        match &self.legislatura.incarico {
            Some(Incarico::Guida { coalizione }) if self.legislatura.fase == Fase::Trattativa => {
                Ok(coalizione.clone())
            }
            _ => Err(Errore::Comando(
                "Ribaltone: nessuna guida da chiedere".into(),
            )),
        }
    }

    /// `ChiediGuida` (decisione 1 di Mario): la guida, cioè la presidenza
    /// (formazione-di-coalizioni:22), la cede il primo per seggi della coalizione del ribaltone.
    /// La cede se il ribaltone, coi seggi veri e il giocatore alla guida, gli vale più del voto
    /// proiettato sul sondaggio pubblico con le coalizioni dell'ultimo voto, dove pesa la guida
    /// della sua (formazione-di-coalizioni:48). Ceduta, l'incarico passa al giocatore con quella
    /// maggioranza, come dopo il voto.
    ///
    /// # Errors
    /// `Comando` senza `Guida` aperta, o «{nome} non ti cede la guida: {causa}»; la partita resta
    /// identica.
    pub(crate) fn chiedi_guida(&mut self) -> Result<(), Errore> {
        let coalizione = self.guida_aperta()?;
        let veri = self.quadro_veri();
        let ribaltone = Dichiarazione {
            partiti: (coalizione.iter().copied())
                .filter(|&j| j != self.giocatore)
                .collect(),
            cessioni: Vec::new(),
        };
        if let Some(l) = self.primo_per_seggi(&coalizione)
            && let Err(causa) = veri.cede_la_guida(
                &ribaltone,
                &self.quadro(),
                &self.dichiarazione_corrente(),
                l,
            )
        {
            return Err(Errore::Comando(format!(
                "{} non ti cede la guida: {}",
                self.partiti.get(l).map_or("", |p| p.nome.as_str()),
                causa.testo(false)
            )));
        }
        let l = &mut self.legislatura;
        l.incarico = Some(Incarico::Giocatore);
        l.maggioranza = Some(coalizione);
        Ok(())
    }

    /// `RifiutaRibaltone`: il giocatore resta fuori; la chiusura cerca la maggioranza contigua
    /// senza di lui, altrimenti scioglie.
    ///
    /// # Errors
    /// `Comando` senza `Guida` aperta.
    pub(crate) fn rifiuta_ribaltone(&mut self) -> Result<(), Errore> {
        self.guida_aperta()?;
        let l = &mut self.legislatura;
        l.incarico = None;
        l.maggioranza = None;
        Ok(())
    }

    /// La mozione di sfiducia al governo (`dicastero` = `None`) o al ministro del `dicastero`,
    /// votata a blocchi sui seggi dei partiti del sì e del no (iter-legislativo-italiano:43,
    /// appello nominale): passa se il sì supera il no in almeno una Camera. L'esito si registra.
    /// Passata al governo, dimissioni; al ministro, il ministro esce (`Sfiduciato`) e, se è di un
    /// alleato, l'alleato perde `costo_dimissioni_chieste`. Respinta, ogni alleato guadagna
    /// `intesa_sfiducia_respinta` e il `promotore` perde `valence_sfiducia_respinta`. `true` se
    /// è passata. Un `dicastero` fuori dai dati o in interim: nessun voto, `false`.
    pub(crate) fn sfiducia(&mut self, dicastero: Option<usize>, promotore: Option<usize>) -> bool {
        let tecnico = (dicastero.and_then(|i| self.ministri.dicasteri.get(i)?.ministro.as_ref()))
            .is_some_and(|m| m.tecnico);
        let Some((linea, camera, senato, approvata)) = self.voto_sfiducia(dicastero) else {
            return false;
        };
        let nome = match dicastero.and_then(|i| self.dati.dicasteri().get(i)) {
            Some(d) => format!("Mozione di sfiducia individuale: {}", d.nome),
            None => "Mozione di sfiducia al governo".into(),
        };
        let esito = EsitoVoto {
            misura: self.parlamento.prossima,
            nome,
            forma: FormaMisura::Sfiducia,
            data: self.data,
            deliberazione: 0,
            camera,
            senato,
            approvata,
            fiducia: false,
            fiducia_respinta: false,
            linee: self.stato_aula().voci_linea(&linea),
            decaduta: None,
        };
        self.parlamento.prossima = self.parlamento.prossima.saturating_add(1);
        self.parlamento.esiti.insert(0, esito);
        match (approvata, dicastero) {
            (true, None) => self.legislatura.dimissioni = true,
            (true, Some(i)) => {
                if let Some(id) = self.dati.dicasteri().get(i).map(|d| d.id.clone()) {
                    self.dimissioni_chieste(&id);
                }
                self.ministri.dimetti(i, Causa::Sfiduciato);
                if let Some(st) = self.ministri.dicasteri.get_mut(i) {
                    st.rosa_tecnica = tecnico;
                }
            }
            (false, _) => {
                let delta = self.dati.manopola("intesa_sfiducia_respinta");
                for j in self.alleati() {
                    (self.intesa).varia(j, CausaIntesa::SfiduciaRespinta, delta, self.turno);
                }
                let valence = self.dati.manopola("valence_sfiducia_respinta");
                if let Some(p) = promotore.and_then(|j| self.partiti.get_mut(j)) {
                    p.valence -= valence;
                }
            }
        }
        approvata
    }

    /// La linea della sfiducia e i conteggi a blocchi di Camera e Senato sullo stato attuale, con
    /// l'esito; `None` con un `dicastero` fuori dai dati o in interim.
    fn voto_sfiducia(
        &self,
        dicastero: Option<usize>,
    ) -> Option<(Vec<bool>, Conteggio, Conteggio, bool)> {
        let partito = match dicastero {
            Some(i) => Some(self.ministri.dicasteri.get(i)?.ministro.as_ref()?.partito),
            None => None,
        };
        let (g, pl) = (&self.governo, &self.parlamento);
        let q = self.quadro_veri();
        let linea = legislatura::linea_sfiducia(
            self.partiti.len(),
            g,
            &self.intesa.ultimatum,
            partito,
            |j| legislatura::vota_la_caduta(&q, j, &g.coalizione, g.presidente),
        );
        let blocchi = |seggi: &[u32]| {
            (seggi.iter().zip(&linea)).fold(Conteggio { si: 0, no: 0 }, |c, (&x, &si)| {
                if si {
                    Conteggio { si: c.si + x, ..c }
                } else {
                    Conteggio { no: c.no + x, ..c }
                }
            })
        };
        let (camera, senato) = (blocchi(&pl.seggi_camera), blocchi(&pl.seggi_senato));
        // Il pareggio respinge la mozione, anche quella automatica a fine crisi.
        let approvata = camera.si > camera.no || senato.si > senato.no;
        Some((linea, camera, senato, approvata))
    }

    /// Il dicastero della sfiducia del giocatore (`None` = al governo), le firme e la linea.
    ///
    /// # Errors
    /// `Comando` dalla guardia dell'opposizione, col dicastero sconosciuto o in interim.
    fn sfiducia_ammessa(
        &self,
        dicastero: Option<&str>,
    ) -> Result<(Option<usize>, AnteprimaSfiducia), Errore> {
        self.all_opposizione()?;
        let ko = |m: String| Err(Errore::Comando(m));
        let i = match dicastero {
            None => None,
            Some(id) => match self.dati.dicasteri().iter().position(|d| d.id == id) {
                Some(i) => Some(i),
                None => return ko(format!("dicastero sconosciuto: {id}")),
            },
        };
        let Some((linea, _, _, passa)) = self.voto_sfiducia(i) else {
            let nome = i
                .and_then(|i| self.dati.dicasteri().get(i))
                .map(|d| &d.nome);
            return ko(format!("{} è in interim", nome.map_or("", |n| n.as_str())));
        };
        let (firme, servono) = self.firme_sfiducia();
        let anteprima = AnteprimaSfiducia {
            firme: firme >= servono,
            linee: self.stato_aula().voci_linea(&linea),
            passa,
        };
        Ok((i, anteprima))
    }

    /// Le firme della sfiducia del giocatore: i seggi alla Camera dei partiti fuori dalla
    /// coalizione di governo, e quante ne servono, un decimo della Camera per eccesso
    /// (iter-legislativo-italiano:142).
    fn firme_sfiducia(&self) -> (u32, u32) {
        let seggi = &self.parlamento.seggi_camera;
        let firme = (seggi.iter().enumerate())
            .filter(|&(j, _)| !self.governo.al_governo(j))
            .map(|(_, &n)| n)
            .sum();
        (firme, seggi.iter().sum::<u32>().div_ceil(10))
    }

    /// L'anteprima della sfiducia del giocatore all'opposizione, al governo (`None`) o al ministro
    /// del `dicastero` (id): le firme bastano, chi la voterebbe e se passa, come il voto in
    /// chiusura se nulla cambia prima. Non tocca nulla.
    ///
    /// # Errors
    /// `Comando` come `Sfiducia`, tranne la sfiducia già presentata e le firme che mancano.
    pub fn anteprima_sfiducia(&self, dicastero: Option<&str>) -> Result<AnteprimaSfiducia, Errore> {
        self.sfiducia_ammessa(dicastero).map(|(_, a)| a)
    }

    /// `Sfiducia` (scelta 13 del piano 7b): una per turno, votata in chiusura dopo leggi e
    /// referendum.
    ///
    /// # Errors
    /// `Comando`, e la partita resta identica: guardia dell'opposizione, sfiducia già presentata
    /// nel turno, dicastero sconosciuto o in interim, firme sotto un decimo della Camera.
    pub(crate) fn presenta_sfiducia(&mut self, dicastero: Option<&str>) -> Result<(), Errore> {
        self.all_opposizione()?;
        if self.parlamento.sfiducia.is_some() {
            return Err(Errore::Comando(
                "una sfiducia è già presentata in questo turno".into(),
            ));
        }
        let (i, a) = self.sfiducia_ammessa(dicastero)?;
        if !a.firme {
            let (n, m) = self.firme_sfiducia();
            return Err(Errore::Comando(format!(
                "servono le firme di un decimo della Camera: {n} su {m}"
            )));
        }
        self.parlamento.sfiducia = Some((i, self.giocatore));
        Ok(())
    }

    /// Il partito del posto riservato all'opposizione nel turno (scelta 15 del piano 7b): la fila
    /// sono i partiti fuori dalla coalizione di governo con seggi alla Camera, in ordine d'indice,
    /// e nel turno `t` il posto va a `fila[t % fila.len()]`. `None` con la fila vuota.
    fn posto_riservato(&self) -> Option<usize> {
        let fila: Vec<usize> = (self.parlamento.seggi_camera.iter().enumerate())
            .filter(|&(j, &n)| n > 0 && !self.governo.al_governo(j))
            .map(|(j, _)| j)
            .collect();
        (self.turno as usize)
            .checked_rem(fila.len())
            .and_then(|k| fila.get(k).copied())
    }

    /// `Linea`: la linea del giocatore all'opposizione sulla misura.
    ///
    /// # Errors
    /// `Comando` dalla guardia dell'opposizione o con la misura sconosciuta.
    pub(crate) fn linea(&mut self, misura: u32, si: Option<bool>) -> Result<(), Errore> {
        self.all_opposizione()?;
        self.parlamento.linea(misura, si)
    }

    /// `PresentaProposta`: la legge del giocatore nel posto riservato, nel turno in cui è suo.
    ///
    /// # Errors
    /// `Comando`, e la partita resta identica: guardia dell'opposizione, posto riservato di un
    /// altro partito o di nessuno, poi quelli di `Parlamento::presenta_proposta`.
    pub(crate) fn presenta_proposta(&mut self, valori: &[ValoreProposto]) -> Result<(), Errore> {
        self.all_opposizione()?;
        let g = self.giocatore;
        match self.posto_riservato() {
            Some(j) if j == g => {}
            Some(j) => {
                let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
                return Err(Errore::Comando(format!(
                    "il posto riservato di questo turno è di {nome}"
                )));
            }
            None => {
                return Err(Errore::Comando(
                    "nessun posto riservato in questo turno".into(),
                ));
            }
        }
        (self.parlamento).presenta_proposta(&self.dati, &self.disposizioni, self.turno, g, valori)
    }

    /// In fase ordinaria sotto un governo IA, l'intesa dei suoi alleati in fasce (spec riga 370):
    /// la causa solo in ultimatum, mai le cifre. Vuota altrove.
    fn voci_avversari(&self) -> Vec<VoceFascia> {
        if self.legislatura.fase != Fase::Ordinaria || self.governo.presidente == self.giocatore {
            return Vec::new();
        }
        let soglia = self.dati.manopola("soglia_incrinata");
        (self.alleati().into_iter())
            .map(|j| {
                let causa_ultimatum = self.intesa.ultimatum.get(j).copied().flatten();
                let incrinata = self.intesa.valori.get(j).is_some_and(|&v| v < soglia);
                let fascia = match (causa_ultimatum, incrinata) {
                    (Some(_), _) => FasciaIntesa::Ultimatum,
                    (None, true) => FasciaIntesa::Incrinata,
                    (None, false) => FasciaIntesa::Solida,
                };
                VoceFascia {
                    partito: self.partiti[j].id.clone(),
                    fascia,
                    causa_ultimatum,
                }
            })
            .collect()
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
            ministeri: (self.ministri.voci(&self.contesto()).into_iter())
                .map(|m| VoceMinistero {
                    agenda: m.agenda.filter(|_| self.governa()),
                    ..m
                })
                .collect(),
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
            alleati_avversari: self.voci_avversari(),
            posto_riservato: self.all_opposizione().is_ok()
                && self.posto_riservato() == Some(self.giocatore)
                && self.parlamento.posto_riservato_libero(),
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
            eredita: self.eredita(),
            disponibilita: if (l.turno_di_voto(self.turno) || self.allarga_ammesso())
                && !self.finita()
            {
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
            incarico: match (l.fase, &l.incarico) {
                (Fase::Trattativa, Some(Incarico::Giocatore)) => Some(VoceIncarico::Giocatore),
                (Fase::Trattativa, Some(Incarico::Guida { .. })) => Some(VoceIncarico::Guida),
                (Fase::Trattativa, _) => Some(VoceIncarico::CapoDelloStato),
                _ => None,
            },
            dimissioni: l.dimissioni,
            crisi: self.in_crisi(),
        }
    }

    /// Sotto un governo IA nessuna: l'intesa è quella col giocatore, come in `quadro()`.
    fn voci_alleati(&self) -> Vec<VoceAlleato> {
        if self.governo.presidente != self.giocatore {
            return Vec::new();
        }
        (self.alleati().into_iter())
            .map(|j| {
                let variazioni: Vec<_> = (self.intesa.variazioni.iter())
                    .filter(|v| v.partito == j)
                    .cloned()
                    .collect();
                let causa_ultimatum = self.intesa.ultimatum.get(j).copied().flatten();
                VoceAlleato {
                    partito: self.partiti[j].id.clone(),
                    intesa: self.intesa.valori.get(j).copied().unwrap_or_default(),
                    ultimatum: causa_ultimatum.is_some(),
                    causa_ultimatum,
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
        partita.intesa.controlla(n, partita.turno)?;
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
        ok(
            self.turni_al_governo <= self.turno.saturating_add(1)
                && self.partenza.iter().all(|x| x.is_finite()),
            "turni_al_governo oltre il turno o partenza non finita",
        )?;
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
            (l.elezioni.iter().flat_map(|e| &e.voti)).all(|v| (0.0..=1.0).contains(&v.quota)),
            "legislatura.elezioni con una quota fuori da 0-1",
        )?;
        ok(
            l.fase == Fase::Sciolta || self.turno <= l.scadenza,
            "legislatura: turno oltre la scadenza fuori dalla fase sciolta",
        )?;
        ok(
            match &l.incarico {
                Some(Incarico::Ia { coalizione } | Incarico::Guida { coalizione }) => {
                    !coalizione.is_empty()
                        && coalizione.iter().all(|&j| j < n)
                        && unici(coalizione.iter().copied())
                }
                _ => true,
            },
            "legislatura.incarico IA o di guida con una coalizione vuota, un partito ripetuto o \
             fuori dai partiti",
        )?;
        let g = self.giocatore;
        ok(
            match &l.incarico {
                Some(Incarico::Guida { coalizione }) => {
                    coalizione.contains(&g) && self.primo_per_seggi(coalizione) != Some(g)
                }
                _ => true,
            },
            "legislatura.incarico di guida senza il giocatore o col giocatore primo per seggi",
        )?;
        ok(
            l.maggioranza.as_ref().is_none_or(|c| {
                l.fase == Fase::Trattativa
                    && l.incarico == Some(Incarico::Giocatore)
                    && c.contains(&self.giocatore)
                    && c.iter().all(|&j| j < n)
            }),
            "legislatura.maggioranza fuori dalla trattativa col giocatore, senza di lui o con un \
             partito fuori dai partiti",
        )?;
        ok(
            l.crisi
                .is_none_or(|t| t == self.turno && l.fase == Fase::Ordinaria),
            "legislatura.crisi fuori dal turno in corso o fuori dalla fase ordinaria",
        )?;
        let coalizione = self.coalizione_del_giocatore();
        let in_trattativa = self.con_incarico();
        ok(
            l.allargati.iter().all(|&j| j < n)
                && unici(coalizione.iter().copied())
                && (self.allarga_ammesso() || l.allargati.is_empty()),
            "legislatura.allargati con un partito fuori dai partiti, ripetuto, già nella \
             coalizione o fuori dalla trattativa e dalla crisi col giocatore",
        )?;
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
            posizione: 5.5,
        };
        Partita::fonda(dati_base(), 7, &f).unwrap()
    }

    /// Sotto un governo IA il segnaposto presenta la bozza e nessun alleato emenda il bilancio.
    #[test]
    fn il_bilancio_dell_ia_non_ha_emendamenti() {
        let f = Fondazione {
            nome: "Partito nuovo".into(),
            posizione: 1.5,
        };
        let mut p = Partita::fonda(dati_base(), 7, &f).unwrap();
        p.chiudi_turno(); // voto
        p.chiudi_turno(); // trattativa
        assert!(!p.governa());
        // Dal secondo ottobre-dicembre: il primo anno i prezzi non hanno ancora mangiato i budget.
        for _ in 0..2 {
            p.chiudi_turno();
            while p.data.trimestre != 4 {
                p.chiudi_turno();
            }
        }
        p.informazione.statistica.indice_prezzi = 1.2; // voci in perdita reale
        let b = p.bozza_bilancio().unwrap();
        p.presenta_bilancio(&b).unwrap();
        let m = (p.parlamento.misure.iter())
            .find(|m| m.forma == FormaMisura::Bilancio)
            .unwrap();
        assert!(m.emendamenti.is_empty());
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

    /// Sfiducia a un dicastero fuori dai dati o in interim: nessun voto, nessun effetto.
    #[test]
    fn sfiducia_senza_ministro_non_si_vota() {
        let mut p = Partita::nuova(dati_base(), 1);
        let st = &mut p.ministri.dicasteri[0];
        (st.ministro, st.agenda, st.proposta) = (None, None, None);
        let prima = p.salva().unwrap();
        assert!(!p.sfiducia(Some(0), Some(0)));
        assert!(!p.sfiducia(Some(99), Some(0)));
        assert_eq!(p.salva().unwrap(), prima);
    }

    /// Il centrodestra (4, posizione 5) vota la caduta del governo di 4 e 5 (destra, posizione 6,
    /// presidente) coi seggi veri `c`, `s` e la sua intesa `i`. Manopole dei dati: seggi 4,
    /// governare 2, distanza 1, guida 1, intesa 2 su base 60. Con `q` la sua quota dei seggi veri,
    /// restare vale `4q + 2 − |5 − 6| − 2 · (60 − i) / 100 = 4q + 1 − (60 − i) / 50`.
    fn cade(c: [u32; 7], s: [u32; 7], intesa: f64) -> bool {
        let mut p = Partita::nuova(dati_base(), 1);
        p.intesa.valori[4] = intesa;
        let q = Quadro {
            seggi_veri: Some((c.to_vec(), s.to_vec())),
            ..p.quadro()
        };
        legislatura::vota_la_caduta(&q, 4, &[4, 5], 5)
    }

    /// Come `cade`, col governo IA: il giocatore è l'estrema sinistra (0).
    fn cade_ia(c: [u32; 7], s: [u32; 7], intesa: f64) -> bool {
        let mut p = Partita::nuova(dati_base(), 1);
        p.intesa.valori[4] = intesa;
        let q = Quadro {
            seggi_veri: Some((c.to_vec(), s.to_vec())),
            giocatore: 0,
            ..p.quadro()
        };
        legislatura::vota_la_caduta(&q, 4, &[4, 5], 5)
    }

    /// La maggioranza contigua senza la destra, che sceglie il ramo.
    fn contigua(c: [u32; 7], s: [u32; 7]) -> Option<Vec<usize>> {
        let p = Partita::nuova(dati_base(), 1);
        legislatura::maggioranza_contigua(&p.partiti, &c, &s, Some(5))
    }

    #[test]
    fn la_caduta_vale_sui_tre_rami() {
        // Ramo 1: contigua (4, 6) con l'estrema destra prima per seggi (165 contro 152). Cadere
        // vale `4q + 2 − |5 − 7| = 4q`: cade se `(60 − i) / 50 > 1`, cioè sotto intesa 10.
        let (c, s) = ([10, 40, 29, 10, 101, 100, 110], [5, 20, 14, 5, 51, 50, 55]);
        assert_eq!(contigua(c, s), Some(vec![4, 6]));
        assert!(!cade(c, s, 60.0));
        assert!(!cade(c, s, 15.0));
        assert!(cade(c, s, 5.0));
        // Ramo 2: stessa contigua col centrodestra primo (165 contro 152): in più la guida,
        // cadere vale `4q + 1`; cade se `(60 − i) / 50 > 0`, cioè sotto la base.
        let (c, s) = ([10, 40, 29, 10, 110, 100, 101], [5, 20, 14, 5, 55, 50, 51]);
        assert_eq!(contigua(c, s), Some(vec![4, 6]));
        assert!(!cade(c, s, 60.0));
        assert!(cade(c, s, 59.0));
        // Col governo IA la contigua è con tutti (scelta 11): (4, 5), lo stesso governo col
        // centrodestra primo. Cadere vale restare più la guida: cade anche alla base.
        assert!(cade_ia(c, s, 60.0));
        // Ramo 3: contigua (0, 1, 2), senza il centrodestra. Cadere vale i suoi termini al voto
        // proiettato sul sondaggio, da solo: `4q' + 2P`, fra 0 e 0,4 su questo sondaggio, dove i
        // seggi veri non contano. Con 6 seggi veri su 600 (`q` = 0,01) restare vale
        // `1,04 − (60 − i) / 50`: 1,04 alla base, −0,16 a intesa zero.
        let (c, s) = ([10, 100, 100, 10, 4, 116, 60], [5, 50, 50, 5, 2, 58, 30]);
        assert_eq!(contigua(c, s), Some(vec![0, 1, 2]));
        assert!(!cade(c, s, 60.0));
        assert!(cade(c, s, 0.0));
    }

    /// Destra e centrodestra in minoranza (150 su 400, 75 su 200): la sfiducia al ministro del
    /// centrodestra all'Ambiente passa, il ministro esce e il centrodestra paga le dimissioni
    /// chieste.
    #[test]
    fn sfiducia_approvata_al_ministro_lo_toglie() {
        let mut p = Partita::nuova(dati_base(), 1);
        p.parlamento.seggi_camera = vec![40, 80, 60, 30, 50, 100, 40];
        p.parlamento.seggi_senato = vec![20, 40, 30, 15, 25, 50, 20];
        let k = (p.dati.dicasteri().iter())
            .position(|d| d.id == "ambiente_energia")
            .unwrap();
        assert_eq!(
            p.ministri.dicasteri[k].ministro.as_ref().unwrap().partito,
            4
        );
        let intesa = p.intesa.valori[4];
        assert!(p.sfiducia(Some(k), Some(0)));
        let e = &p.parlamento.esiti[0];
        let nome = &p.dati.dicasteri()[k].nome;
        assert_eq!(e.nome, format!("Mozione di sfiducia individuale: {nome}"));
        let st = &p.ministri.dicasteri[k];
        assert!(st.ministro.is_none());
        assert_eq!(st.causa_dimissioni, Some(Causa::Sfiduciato));
        let costo = p.dati.manopola("costo_dimissioni_chieste");
        assert_eq!(p.intesa.valori[4], intesa - costo);
        let chieste =
            (p.intesa.variazioni.iter()).any(|v| v.causa == CausaIntesa::DimissioniChieste);
        assert!(chieste);
        assert!(!p.legislatura.dimissioni);
    }

    /// Destra e centrodestra con 200 seggi su 400 alla Camera e 101 su 200 al Senato: niente
    /// maggioranza, ma la sfiducia pareggia alla Camera e perde al Senato. Il pareggio la respinge.
    #[test]
    fn il_pareggio_respinge_la_sfiducia() {
        let mut p = Partita::nuova(dati_base(), 1);
        p.parlamento.seggi_camera = vec![10, 40, 30, 20, 100, 100, 100];
        p.parlamento.seggi_senato = vec![5, 20, 14, 10, 51, 50, 50];
        let pl = &p.parlamento;
        assert!(!legislatura::maggioranza(
            &pl.seggi_camera,
            &pl.seggi_senato,
            &p.governo.coalizione
        ));
        let (intesa, valence) = (p.intesa.valori[4], p.partiti[0].valence);
        assert!(!p.sfiducia(None, Some(0)));
        let e = &p.parlamento.esiti[0];
        assert_eq!((e.camera.si, e.camera.no), (200, 200));
        assert!(e.senato.si < e.senato.no);
        assert!(!e.approvata && !p.legislatura.dimissioni);
        let d = |nome| p.dati.manopola(nome);
        assert_eq!(p.intesa.valori[4], intesa + d("intesa_sfiducia_respinta"));
        assert_eq!(
            p.partiti[0].valence,
            valence - d("valence_sfiducia_respinta")
        );
    }
}
