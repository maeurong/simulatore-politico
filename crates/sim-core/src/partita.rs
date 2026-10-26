use crate::caso::Caso;
use crate::dati::DatiGioco;
use crate::disposizioni::Disposizioni;
use crate::economia::{
    self, Annata, Bilancio, Economia, FUORI_STAGIONE, RiepilogoBilancio, Voce, livello_ok,
    perdita_reale, perdite_scaglioni,
};
use crate::elettorato::{Condizione, Elettorato, Fascia, Intenzioni, indice_gruppo};
use crate::elezioni::{Esito, LeggeElettorale, Voti, riparto};
use crate::errore::Errore;
use crate::grandezze::Grandezze;
use crate::informazione::{ElettoreCampione, Informazione, genera};
use crate::ministri::{Causa, Contesto, Ministri, causa_peggiore, penalizza};
use crate::misure::Misure;
use crate::parlamento::{
    self, CausaIntesa, EsitoVoto, Intesa, Parlamento, StatoAula, ValoreProposto, VariazioneIntesa,
    voto,
};
use crate::partiti::{self, Governo, Partito};
use crate::vista::{
    SeggiPartito, Vista, VistaBilancio, VistaStatistica, VoceAlleato, VoceBudget, VoceDicastero,
    VoceGrandezza, VocePartito,
};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use ts_rs::TS;

pub const VERSIONE_SCHEMA: u32 = 6;

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
}

impl Partita {
    pub fn nuova(dati: DatiGioco, seme: u64) -> Partita {
        let economia = Economia::iniziale(&dati);
        let grandezze = Grandezze::iniziali(&dati);
        let (partiti, governo) = partiti::iniziali(&dati);
        let mut elettorato = Elettorato::iniziale(&dati, partiti.len(), &economia);
        elettorato.avvia_voto(&dati, &partiti, &governo);
        let mut caso = Caso::nuovo(seme);
        let informazione =
            Informazione::nuova(&dati, &mut caso, &elettorato, &partiti, &governo, &economia);
        let disposizioni = Disposizioni::iniziali(&dati);
        let ministri = Ministri::iniziali(
            &Contesto {
                dati: &dati,
                governo: &governo,
                partiti: &partiti,
                grandezze: &grandezze,
                economia: &economia,
                sondaggio: &informazione.sondaggio,
                disposizioni: &disposizioni,
                indice_prezzi: informazione.statistica.indice_prezzi,
                turno: 0,
                data: Trimestre::TURNO_ZERO,
            },
            &mut caso,
        );
        Partita {
            parlamento: Parlamento::uscente(&dati, &partiti),
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
        }
    }

    // Ordine della chiusura:
    // default delle decisioni → atti dei ministri → aula (conversioni → bilancio → leggi →
    // referendum) → motore (misure, economia, grandezze,
    // redditi, giudizio [prima di composizione, che aggiorna ultima_disoccupazione],
    // composizione, posizioni, sondaggio, statistica) → soglie (ministri, ritorno dell'intesa) →
    // calendario → apertura (valori in arrivo, esiti dei due turni prima, rose, agende, proposte;
    // il costo del taglio se l'anno entra in esercizio provvisorio).
    pub fn chiudi_turno(&mut self) {
        self.con_contesto(|ministri, cx| ministri.decidi_proposte(cx));
        let effetti = self.con_contesto(|ministri, cx| ministri.esegui(cx));
        self.grandezze.effetti.extend(effetti);
        self.aula();
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
        self.con_contesto(|ministri, cx| ministri.soglie(cx));
        self.intesa.soglie(&self.dati, self.turno);
        self.turno = self.turno.saturating_add(1);
        self.data = self.data.successivo();
        (self.disposizioni).arrivi(&self.dati, self.turno, &mut self.grandezze.effetti);
        self.parlamento.apri(self.turno);
        let cx = Contesto {
            dati: &self.dati,
            governo: &self.governo,
            partiti: &self.partiti,
            grandezze: &self.grandezze,
            economia: &self.economia,
            sondaggio: &self.informazione.sondaggio,
            disposizioni: &self.disposizioni,
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
            indice_prezzi: self.informazione.statistica.indice_prezzi,
            turno: self.turno,
            data: self.data,
        };
        f(&mut self.ministri, &cx)
    }

    fn aula(&mut self) {
        self.approva_bilancio();
        self.vota_leggi();
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
    fn vota(&mut self, id: u32) {
        let Some(k) = self.parlamento.misure.iter().position(|m| m.id == id) else {
            return;
        };
        let m = self.parlamento.misure.remove(k);
        let aula = self.stato_aula();
        let p = aula.previsione(&m.valori);
        let linee = aula.voci_linea(&p.linea);
        let pl = &self.parlamento;
        let camera = voto::conta(&mut self.caso, &pl.seggi_camera, &p.linea, &p.franchi);
        let senato = voto::conta(&mut self.caso, &pl.seggi_senato, &p.linea, &p.franchi);
        let approvata = camera.si > camera.no && senato.si > senato.no;
        let turno = self.turno;
        let nuova = p.posizioni.map(|(_, n)| n);
        if approvata {
            let legge = format!("L. {id}/{}", self.data.anno);
            for &(d, g) in &m.valori {
                (self.disposizioni).cambia(
                    &self.dati,
                    d,
                    g,
                    turno,
                    &legge,
                    &mut self.grandezze.effetti,
                );
            }
            if let Some(nuova) = nuova {
                self.norma_lontana(nuova);
            }
        } else {
            let blocco = self.dati.manopola("turni_blocco_respinta") as u32;
            let da = turno.saturating_add(1).saturating_add(blocco);
            (self.parlamento.respinte).extend(m.valori.iter().map(|&(d, _)| (d, da)));
            if let Some(st) = m
                .proponente
                .and_then(|i| self.ministri.dicasteri.get_mut(i))
            {
                let costo = self.dati.manopola("costo_legge_respinta");
                penalizza(st, Causa::LeggeRespinta, -costo, turno);
            }
            if let Some(pdc) = self.partiti.get_mut(self.governo.presidente) {
                pdc.valence -= self.dati.manopola("valence_legge_respinta");
            }
        }
        if let Some(nuova) = nuova {
            let s = self.dati.manopola("scivolamento_percepita");
            for (x, _) in self.partiti.iter_mut().zip(&p.linea).filter(|(_, si)| **si) {
                x.posizione_percepita += s * (nuova - x.posizione_percepita);
            }
        }
        let esito = EsitoVoto {
            misura: id,
            nome: parlamento::nome(&self.dati, &m.valori),
            forma: m.forma,
            data: self.data,
            deliberazione: m.deliberazione,
            camera,
            senato,
            approvata,
            fiducia: m.fiducia,
            fiducia_respinta: false,
            linee,
        };
        self.parlamento.esiti.insert(0, esito);
    }

    /// Norma approvata in posizione `nuova`: ogni alleato distante almeno `distanza_alleato`
    /// perde intesa.
    fn norma_lontana(&mut self, nuova: f64) {
        let soglia = self.dati.manopola("distanza_alleato");
        let costo = self.dati.manopola("costo_norma_lontana");
        for j in self.alleati() {
            let (Some(p), Some(x)) = (self.partiti.get(j), self.intesa.valori.get_mut(j)) else {
                continue;
            };
            let distanza = (p.posizione_dichiarata - nuova).abs();
            if distanza < soglia {
                continue;
            }
            let delta = -costo * (distanza - soglia + 1.0);
            *x = (*x + delta).max(0.0);
            self.intesa.variazioni.push(VariazioneIntesa {
                partito: j,
                causa: CausaIntesa::NormaLontana,
                delta,
                turno: self.turno,
            });
        }
    }

    fn stato_aula(&self) -> StatoAula<'_> {
        StatoAula {
            dati: &self.dati,
            disposizioni: &self.disposizioni,
            partiti: &self.partiti,
            governo: &self.governo,
            intesa: &self.intesa.valori,
        }
    }

    /// Il bilancio presentato è approvato, senza voto per ora. Se l'anno è in esercizio
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

    /// `PresentaBilancio`: sostituisce quello già presentato; l'aula lo approva in chiusura.
    pub(crate) fn presenta_bilancio(&mut self, b: &Bilancio) -> Result<(), Errore> {
        let (_, b) = self.bilancio_ammesso(b)?;
        self.economia.bilancio_presentato = Some(b);
        Ok(())
    }

    /// `PresentaLegge`: la misura entra in calendario.
    pub(crate) fn presenta_legge(&mut self, valori: &[ValoreProposto]) -> Result<(), Errore> {
        (self.parlamento).presenta(
            &self.dati,
            &self.disposizioni,
            &mut self.ministri,
            self.turno,
            valori,
        )
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

    /// `Accogli`: l'importo è un decreto, speso per intero nel trimestre che si chiude.
    pub(crate) fn aggiungi_decreto(&mut self, importo: i64) {
        self.economia.decreti += importo;
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

    /// Riparto dei seggi sul sondaggio pubblico: voti regionali dalle quote degli incroci,
    /// coalizioni dell'ultimo voto, partiti senza coalizione da soli.
    ///
    /// # Errors
    /// Quelli di `elezioni::riparto`.
    pub fn proiezione_seggi(&self) -> Result<Esito, Errore> {
        let n = self.partiti.len();
        let per_regione = self
            .dati
            .elettorato()
            .regioni
            .iter()
            .zip(&self.informazione.sondaggio.regioni)
            .map(|(r, inc)| {
                let voti = inc.quote.iter().take(n);
                (
                    r.id.clone(),
                    voti.map(|q| (r.elettori as f64 * q).round() as u64)
                        .collect(),
                )
            })
            .collect();
        let posizione = |id: &String| self.partiti.iter().position(|p| &p.id == id);
        let coalizioni = (self.dati.scenario().coalizioni.iter())
            .map(|c| c.iter().filter_map(posizione).collect())
            .collect();
        riparto(
            self.dati.elezioni(),
            &self.legge_elettorale(),
            &Voti {
                coalizioni,
                per_regione,
            },
        )
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
        Vista {
            turno: self.turno,
            data: self.data,
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
    /// economia, grandezze, partiti e governo, elettorato, informazione, disposizioni.
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
        partita.disposizioni.controlla(&partita.dati)?;
        let n = partita.partiti.len();
        partita.parlamento.controlla(&partita.dati, n)?;
        partita.intesa.controlla(n)?;
        partita.controlla_partiti()?;
        partita
            .ministri
            .controlla(&partita.dati, partita.partiti.len())?;
        Ok(partita)
    }
}

impl Partita {
    /// Partiti, governo ed elettorato di un salvataggio contro i dati.
    fn controlla_partiti(&self) -> Result<(), Errore> {
        let n = self.partiti.len();
        let attesi = self.dati.scenario().partiti.len();
        ok(
            n == attesi,
            &format!("partiti ha {n} voci, lo scenario ne ha {attesi}"),
        )?;
        let g = &self.governo;
        ok(g.presidente < n, "governo.presidente fuori dai partiti")?;
        ok(
            g.dicasteri.len() == self.dati.dicasteri().len() && g.dicasteri.iter().all(|&p| p < n),
            "governo.dicasteri non combacia con i dicasteri o con i partiti",
        )?;
        ok(
            self.partiti
                .iter()
                .all(|p| p.posizione_percepita.is_finite() && p.valence.is_finite()),
            "partiti: posizione_percepita o valence non finita",
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
            (0..=LIMITE_EURO).contains(&self.decreti),
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
