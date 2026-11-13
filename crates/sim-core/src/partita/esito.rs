//! Le voci dell'esito scritte dalla partita, nel punto del fatto.

use super::Partita;
use super::vita::virgola;
use crate::disposizioni::data_di;
use crate::esito::{Grado, TipoVoce, VoceEsito, grado};
use crate::eventi::{Bersaglio, DatoOpzione, EventoAperto, emergenze};
use crate::legislatura::Spesa;
use crate::legislatura::campagna::{self, migliaia};
use crate::ministri::{FattoAtto, Proposta};
use crate::parlamento::{
    self, Attenuazione, Conteggio, EsitoReferendum, EsitoVoto, FormaMisura, Misura, Referendum,
    StatoEmendamento, TipoReferendum,
};

/// «N euro dal fondo di riserva (Q% del costo)» o «N euro a debito».
pub(super) fn pagamento(costo: i64, quota: f64, dal_fondo: bool) -> String {
    let pagato = (quota * costo as f64).round() as i64;
    if dal_fondo {
        format!(
            "{} euro dal fondo di riserva ({:.0}% del costo)",
            migliaia(pagato),
            quota * 100.0
        )
    } else {
        format!("{} euro a debito", migliaia(pagato))
    }
}

/// «valence -0,05 a {nome}», come nella voce dell'evento.
pub(super) fn valence(delta: f64, nome: &str) -> String {
    let segno = format!("{delta:+.2}").replace('.', ",");
    format!("valence {segno} a {nome}")
}

/// «Camera 230 sì e 170 no, 12 franchi tiratori; Senato …»: i franchi solo dove ce ne sono.
fn conteggi(e: &EsitoVoto) -> String {
    let ramo = |nome: &str, c: &Conteggio| {
        let franchi = match c.franchi {
            0 => String::new(),
            1 => ", 1 franco tiratore".to_string(),
            n => format!(", {n} franchi tiratori"),
        };
        format!("{nome} {} sì e {} no{franchi}", c.si, c.no)
    };
    format!(
        "{}; {}",
        ramo("Camera", &e.camera),
        ramo("Senato", &e.senato)
    )
}

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

    /// « in {regione}», o niente.
    fn in_regione(&self, r: Option<usize>) -> String {
        (r.and_then(|r| self.dati.regioni().get(r)))
            .map_or(String::new(), |r| format!(" in {}", r.nome))
    }

    fn id_regione(&self, r: Option<usize>) -> Option<String> {
        r.and_then(|r| self.dati.regioni().get(r))
            .map(|r| r.id.clone())
    }

    /// Fatto 2: l'emergenza lieve del turno prima, pagata dal fondo per la `quota`.
    pub(super) fn voce_lieve(&mut self, aperto: &EventoAperto, quota: f64) {
        let Some(e) = self.dati.eventi().get(aperto.evento) else {
            return;
        };
        let fatto = format!(
            "{} lieve{}, {}",
            e.nome,
            self.in_regione(aperto.regione),
            pagamento(aperto.costo, quota, true)
        );
        let tipo = emergenze::EVENTI.iter().position(|id| *id == e.id);
        let causa = match (tipo, aperto.regione) {
            (Some(3), _) => match data_di(aperto.turno).trimestre {
                2 => "la stagione secca di primavera".to_string(),
                3 => "la stagione secca d'estate".to_string(),
                _ => "la stagione secca".to_string(),
            },
            (Some(t), Some(r)) => {
                let quale = ["sismica", "idrogeologica", "d'incendio"].get(t).copied();
                let indice = emergenze::indice_pericolosita(&self.dati, t, r);
                format!(
                    "pericolosità {} {}",
                    quale.unwrap_or(""),
                    emergenze::a_parole(indice)
                )
            }
            _ => "emergenza naturale".to_string(),
        };
        let (regione, tono) = (self.id_regione(aperto.regione), aperto.tono);
        if let Some(v) = self.voce(TipoVoce::EmergenzaLieve, fatto, causa) {
            v.regione = regione;
            v.tono = tono;
        }
    }

    /// Fatti 3, 4, 5, 9, 10, 11: l'evento risolto con l'`opzione`. `coda` dice il seguito (costo,
    /// esito dello scandalo); la valence di `colpito` era `prima` all'inizio della risoluzione.
    pub(super) fn voce_evento(
        &mut self,
        aperto: &EventoAperto,
        opzione: &DatoOpzione,
        mut coda: Vec<String>,
        colpito: usize,
        prima: f64,
    ) {
        let Some(e) = self.dati.eventi().get(aperto.evento) else {
            return;
        };
        let (nome, famiglia) = (e.nome.clone(), e.famiglia);
        let persona = (aperto.persona.clone())
            .or_else(|| aperto.bersaglio.and_then(|b| self.nome_del(b)))
            .map_or(String::new(), |p| format!(" ({p})"));
        let per_default = aperto.scelta.is_none();
        let mut fatto = format!(
            "{nome}{}{persona}, «{}»{}",
            self.in_regione(aperto.regione),
            opzione.nome,
            if per_default { " per default" } else { "" }
        );
        let partito = self.partiti.get(colpito);
        let delta = partito.map_or(0.0, |p| p.valence - prima);
        if let Some(p) = partito.filter(|_| delta != 0.0) {
            let segno = format!("{delta:+.2}").replace('.', ",");
            coda.push(format!("valence {segno} a {}", p.nome));
        }
        for parte in coda.iter().filter(|c| !c.is_empty()) {
            fatto.push_str("; ");
            fatto.push_str(parte);
        }
        let causa = aperto.causa.testo(&self.dati);
        let dicastero = match aperto.bersaglio {
            Some(Bersaglio::Ministro { dicastero }) => {
                self.dati.dicasteri().get(dicastero).map(|d| d.id.clone())
            }
            _ => None,
        };
        // Solo col bersaglio ancora al suo posto: il ripiego di `partito_evento` sul Presidente
        // del Consiglio vale per la valence, non per il soggetto della voce.
        let presente = (aperto.bersaglio.and_then(|b| self.nome_del(b)))
            .is_some_and(|n| aperto.persona.as_ref().is_none_or(|p| *p == n));
        let id_partito = presente
            .then(|| self.partiti.get(colpito).map(|p| p.id.clone()))
            .flatten();
        let (regione, al_governo) = (self.id_regione(aperto.regione), self.al_governo());
        let Some(v) = self.voce(TipoVoce::EventoRisolto, fatto, causa) else {
            return;
        };
        if per_default && al_governo {
            v.con_grado(Grado::C1);
        }
        v.tono = aperto.tono;
        v.famiglia = Some(famiglia);
        v.regione = regione;
        v.dicastero = dicastero;
        v.partito = id_partito;
    }

    /// Fatti 6, 7, 8: ciò che l'evento ha avviato (programma del MES, ristrutturazione col
    /// `taglio` in euro, margine UE per l'energia).
    pub(super) fn voce_seguito(&mut self, tipo: TipoVoce, aperto: &EventoAperto, taglio: i64) {
        let evento = (self.dati.eventi().get(aperto.evento)).map_or("", |e| e.nome.as_str());
        let scelta = format!("la scelta nelle «{evento}»");
        let m = |nome: &str| self.dati.manopola(nome);
        let (fatto, causa) = match tipo {
            TipoVoce::MesAvviato => {
                let fine = self.economia.programma_mes.unwrap_or(self.turno);
                let vincoli = parlamento::nome(&self.dati, &self.condizionalita());
                let mut fatto = format!(
                    "Programma del MES di {} trimestri, finisce a {}; vincolati {vincoli}",
                    m("durata_mes").round(),
                    parlamento::mese(data_di(fine))
                );
                // All'opposizione l'intesa degli alleati del governo avversario non si vede.
                if self.al_governo() {
                    let intesa = virgola(m("intesa_mes"));
                    fatto.push_str(&format!("; ogni alleato perde {intesa} punti d'intesa"));
                }
                let causa = format!("{scelta}, {}", aperto.causa.testo(&self.dati));
                (fatto, causa)
            }
            TipoVoce::Ristrutturazione => (
                format!(
                    "Debito tagliato del {:.0}%, {} euro; premio di {:.0} pb sullo spread; \
                     redditi primari delle fasce media e alta giù del {:.0}% per {:.0} turni",
                    m("taglio_ristrutturazione") * 100.0,
                    migliaia(taglio),
                    m("premio_ristrutturazione"),
                    m("perdita_ristrutturazione") * 100.0,
                    m("turni_ristrutturazione").round()
                ),
                scelta,
            ),
            TipoVoce::MargineEnergia => (
                format!("Margine UE per l'energia chiesto per il {}", self.data.anno),
                format!("la scelta nel «{evento}»"),
            ),
            _ => return,
        };
        if let Some(v) = self.voce(tipo, fatto, causa) {
            v.tono = aperto.tono;
        }
    }

    /// Fatti 12 e 13: le proposte tolte da `decidi_proposte`, respinte per silenzio o decadute.
    pub(super) fn voci_proposte(&mut self, tolte: Vec<(usize, Proposta, Option<&'static str>)>) {
        let costo = virgola(self.dati.manopola("costo_respingere"));
        for (i, p, decaduta) in tolte {
            let Some(d) = self.dati.dicasteri().get(i) else {
                continue;
            };
            let ministro = (self.ministri.dicasteri.get(i))
                .and_then(|st| st.ministro.as_ref())
                .map_or("", |m| m.nome.as_str());
            let chi = format!("{ministro} ({})", d.nome);
            let (id, testo) = (d.id.clone(), p.tipo.testo(&self.dati));
            let (tipo, fatto, causa) = match decaduta {
                None => (
                    TipoVoce::PropostaRespinta,
                    format!(
                        "Respinta la proposta di {chi}, {testo}; perde {costo} punti di soddisfazione"
                    ),
                    format!(
                        "in sospeso da {} turni senza risposta",
                        p.turni_in_sospeso + 1
                    ),
                ),
                Some(motivo) => (
                    TipoVoce::PropostaDecaduta,
                    format!("Decade senza costo la proposta di {chi}, {testo}"),
                    motivo.to_string(),
                ),
            };
            if let Some(v) = self.voce(tipo, fatto, causa) {
                v.dicastero = Some(id);
            }
        }
    }

    /// Fatto 14: l'emendamento dell'alleato `j` alla `misura` respinto per default, con l'intesa
    /// persa (`costo`).
    pub(super) fn voce_emendamento_respinto(&mut self, j: usize, misura: u32, costo: f64) {
        let Some(m) = self.parlamento.misure.iter().find(|m| m.id == misura) else {
            return;
        };
        let legge = parlamento::nome_misura(&self.dati, m);
        let disposizioni: Vec<String> = (m.valori.iter())
            .filter_map(|&(k, _)| self.dati.disposizioni().get(k).map(|x| x.id.clone()))
            .collect();
        let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
        let fatto = format!(
            "Respinto l'emendamento di {nome} su {legge}; perde {} punti d'intesa",
            virgola(costo)
        );
        let causa = "senza risposta prima del voto".to_string();
        if let Some(v) = self.voce_di(j, TipoVoce::EmendamentoRespinto, fatto, causa) {
            v.misura = Some(misura);
            v.disposizioni = disposizioni;
        }
    }

    /// Fatti 16-20: un atto dei ministri eseguito, saltato, o un cantiere avviato, fermo, finito.
    pub(super) fn voce_atto(&mut self, f: FattoAtto) {
        let Some(d) = self.dati.dicasteri().get(f.dicastero) else {
            return;
        };
        let (dicastero, atto, euro) = (&d.nome, &f.atto, migliaia(f.euro));
        let dove = self.in_regione(f.regione);
        let fatto = match f.tipo {
            TipoVoce::AttoEseguito => {
                format!("Il dicastero {dicastero} esegue «{atto}»{dove} con {euro} euro")
            }
            TipoVoce::AttoSaltato => {
                format!("Il dicastero {dicastero} non esegue «{atto}»{dove}, annunciato in agenda")
            }
            TipoVoce::CantiereAvviato => format!(
                "Il dicastero {dicastero} avvia il cantiere «{atto}»{dove}, {euro} euro a turno \
                 per {} turni",
                f.turni
            ),
            TipoVoce::CantiereFermo => {
                format!("Il cantiere «{atto}»{dove} del dicastero {dicastero} resta fermo un turno")
            }
            TipoVoce::CantiereFinito => format!(
                "Finito il cantiere «{atto}»{dove} del dicastero {dicastero}, {euro} euro spesi \
                 in tutto; gli effetti partono"
            ),
            _ => return,
        };
        let id = d.id.clone();
        // Il motivo dell'agenda porta «: », che nel dettaglio di una voce fusa separa il fatto
        // dalla causa.
        let causa = f.motivo.replace(": ", ", ");
        let regione = self.id_regione(f.regione);
        if let Some(v) = self.voce(f.tipo, fatto, causa) {
            v.dicastero = Some(id);
            v.regione = regione;
        }
    }

    /// Fatto 56: la campagna del giocatore tagliata da `chiesto` a `speso` euro, con le spese
    /// `cadute`; la causa dice gli `acquistato` euro d'informazione del turno e la scissione del
    /// suo partito in questa chiusura.
    pub(super) fn voce_tagliata(
        &mut self,
        chiesto: i64,
        speso: i64,
        cadute: &[Spesa],
        acquistato: i64,
    ) {
        let id = self.id_di(Some(self.giocatore));
        let scissa = (self.esito.iter())
            .any(|v| v.tipo == TipoVoce::Scissione && v.partito.is_some() && v.partito == id);
        let fatto = format!(
            "Campagna tagliata: chiesti {} euro, spesi {}{}",
            migliaia(chiesto),
            migliaia(speso),
            campagna::cadute(&self.dati, cadute)
        );
        let mut perche = Vec::new();
        if acquistato > 0 {
            perche.push(format!(
                "acquisti d'informazione per {} euro",
                migliaia(acquistato)
            ));
        }
        if scissa {
            perche.push("la scissione".to_string());
        }
        let mut causa = "la cassa è scesa dopo la decisione".to_string();
        if !perche.is_empty() {
            causa.push_str(&format!(", con {}", perche.join(" e ")));
        }
        self.voce_di(self.giocatore, TipoVoce::CampagnaTagliata, fatto, causa);
    }

    /// Il promotore (id) e le disposizioni della misura, nell'ordine dei valori.
    fn soggetti(&self, m: &Misura) -> (Option<String>, Vec<String>) {
        (self.id_di(m.promotore), self.disposizioni_di(&m.valori))
    }

    fn id_di(&self, j: Option<usize>) -> Option<String> {
        j.and_then(|j| self.partiti.get(j)).map(|p| p.id.clone())
    }

    /// Gli id delle disposizioni dei `valori`, nell'ordine.
    fn disposizioni_di(&self, valori: &[(usize, usize)]) -> Vec<String> {
        (valori.iter())
            .filter_map(|&(k, _)| self.dati.disposizioni().get(k).map(|x| x.id.clone()))
            .collect()
    }

    /// Come `voce`, coi soggetti della misura `m`.
    fn voce_misura(&mut self, tipo: TipoVoce, m: &Misura, fatto: String, causa: String) {
        let (partito, disposizioni) = self.soggetti(m);
        if let Some(v) = self.voce(tipo, fatto, causa) {
            v.misura = Some(m.id);
            v.partito = partito;
            v.disposizioni = disposizioni;
        }
    }

    /// «per il sì A, B; per il no C»: le linee dei partiti non sciolti.
    fn linee(&self, e: &EsitoVoto) -> String {
        let nomi = |si: bool| {
            let nomi: Vec<&str> = (e.linee.iter())
                .filter(|l| l.si == si)
                .filter_map(|l| (self.partiti.iter()).find(|p| p.id == l.partito && !p.sciolto))
                .map(|p| p.nome.as_str())
                .collect();
            if nomi.is_empty() {
                "nessuno".to_string()
            } else {
                nomi.join(", ")
            }
        };
        format!("per il sì {}; per il no {}", nomi(true), nomi(false))
    }

    /// Fatto 21: le leggi passate al turno dopo nel turno del bilancio o di crisi.
    pub(super) fn voce_slittate(&mut self, slittate: &[Misura]) {
        let k = slittate.iter().filter(|m| m.fiducia).count();
        let mut fatto = match slittate.len() {
            0 => return,
            1 => "Slitta al turno dopo una legge".to_string(),
            n => format!("Slittano al turno dopo {n} leggi"),
        };
        if k > 0 {
            let quante = if k == 1 { "una".into() } else { k.to_string() };
            fatto.push_str(&format!("; cade la fiducia su {quante}"));
            // All'opposizione l'intesa degli alleati del governo avversario non si vede.
            if self.al_governo() {
                fatto.push_str(", e l'intesa che costava torna agli alleati");
            }
        }
        let causa = match (self.turno_del_bilancio(), self.in_crisi()) {
            (true, true) => "il turno della legge di bilancio e la crisi di governo",
            (true, false) => "il turno della legge di bilancio",
            _ => "la crisi di governo",
        };
        self.voce(TipoVoce::LeggiSlittate, fatto, causa.into());
    }

    /// Fatti 22-24, 27, 28, 33-36: il voto `e` della misura `m`. `coda` segue i conteggi nel
    /// fatto; `in_attesa` dice, per la costituzionale che aspetta il confermativo, se è già
    /// richiesto.
    pub(super) fn voce_voto(
        &mut self,
        e: &EsitoVoto,
        m: &Misura,
        coda: Vec<String>,
        in_attesa: Option<bool>,
    ) {
        use TipoVoce::*;
        let tipo = match (e.forma, e.approvata) {
            _ if e.fiducia_respinta => FiduciaRespinta,
            (FormaMisura::Conversione, true) => DecretoConvertito,
            (FormaMisura::Conversione, false) => DecretoDecaduto,
            (FormaMisura::Bilancio, true) => BilancioApprovato,
            (FormaMisura::Bilancio, false) => BilancioRespinto,
            _ if in_attesa.is_some() => ConfermativoInAttesa,
            (_, true) => LeggeApprovata,
            (_, false) => LeggeRespinta,
        };
        let nome = &e.nome;
        let mut fatto = match tipo {
            FiduciaRespinta => format!("Respinta la fiducia su «{nome}»"),
            DecretoConvertito => format!("Convertito in legge «{nome}»"),
            DecretoDecaduto => format!("Decade «{nome}»"),
            BilancioApprovato => "Approvata la legge di bilancio".into(),
            BilancioRespinto => "Respinta la legge di bilancio".into(),
            ConfermativoInAttesa => format!("Approvata senza i due terzi «{nome}»"),
            LeggeApprovata => format!("Approvata «{nome}»"),
            _ => format!("Respinta «{nome}»"),
        };
        if e.fiducia && tipo != FiduciaRespinta {
            fatto.push_str(" con la fiducia");
        }
        if e.forma == FormaMisura::Costituzionale && tipo != ConfermativoInAttesa {
            fatto.push_str(match e.deliberazione {
                1 => " in prima deliberazione",
                _ => " in seconda deliberazione",
            });
        }
        let (causa, parti) = match in_attesa {
            Some(richiesto) => {
                let voto = parlamento::mese(data_di(self.turno.saturating_add(1)));
                let chi = if richiesto {
                    "già richiesto"
                } else {
                    "non ancora richiesto"
                };
                let causa = format!("seconda deliberazione, {}", conteggi(e));
                (
                    causa,
                    vec![format!("referendum confermativo a {voto}, {chi}")],
                )
            }
            None => (
                self.linee(e),
                [conteggi(e)].into_iter().chain(coda).collect(),
            ),
        };
        for parte in parti {
            fatto.push_str("; ");
            fatto.push_str(&parte);
        }
        let del_governo = m.promotore.is_none() && self.al_governo();
        let (partito, disposizioni) = self.soggetti(m);
        let Some(v) = self.voce(tipo, fatto, causa) else {
            return;
        };
        if tipo == LeggeRespinta && del_governo {
            v.con_grado(Grado::C2);
        }
        v.misura = Some(m.id);
        v.partito = partito;
        v.disposizioni = disposizioni;
    }

    /// «budget tagliato in termini reali a Sanità (-1,2%), …»: i dicasteri con ministro e perdita
    /// reale, quelli che il taglio costa.
    pub(super) fn tagli_reali(&self, perdite: &[f64]) -> Option<String> {
        let tagliati: Vec<String> = (self.dati.dicasteri().iter())
            .zip(&self.ministri.dicasteri)
            .zip(perdite)
            .filter(|((_, st), p)| **p > 0.0 && st.ministro.is_some())
            .map(|((d, _), &p)| format!("{} (-{}%)", d.nome, virgola(p)))
            .collect();
        (!tagliati.is_empty())
            .then(|| format!("budget tagliato in termini reali a {}", tagliati.join(", ")))
    }

    /// Fatto 25: la legge di bilancio `misura` approvata e ricondotta al tetto del MES, con
    /// crescita della spesa netta presentata, limite e quota tenuta.
    pub(super) fn voce_ricondotto(
        &mut self,
        misura: u32,
        (crescita, limite, resta): (f64, f64, f64),
    ) {
        let fine =
            (self.economia.programma_mes).map_or(String::new(), |f| parlamento::mese(data_di(f)));
        let fatto = format!(
            "Legge di bilancio ricondotta al tetto del MES: la spesa netta presentata cresceva \
             del {}%, il limite è del {}%; dei budget presentati resta il {:.0}% dello scarto \
             dalla bozza",
            virgola(crescita),
            virgola(limite),
            resta * 100.0
        );
        let causa =
            format!("il programma del MES, cominciato dopo la presentazione, finisce a {fine}");
        if let Some(v) = self.voce(TipoVoce::BilancioRicondotto, fatto, causa) {
            v.misura = Some(misura);
        }
    }

    /// Fatti 29 e 30: i valori della legge `m` che non entrano ora (`entra_in_vigore`).
    pub(super) fn voci_fuori(&mut self, m: &Misura, fuori: Vec<(usize, usize, Option<u32>)>) {
        let rinvio = self.dati.manopola("turni_rinvio_regime").round();
        let fine = (self.economia.mes_in_corso(self.turno))
            .map_or(String::new(), |f| parlamento::mese(data_di(f)));
        for (d, g, arrivo) in fuori {
            let Some(x) = self.dati.disposizioni().get(d) else {
                continue;
            };
            let nuovo = x.gradini.get(g).map_or("", |g| g.nome.as_str());
            let (tipo, fatto, causa) = match arrivo {
                Some(t) => {
                    let chi = (m.emendamenti.iter())
                        .find(|e| {
                            e.stato == StatoEmendamento::Accolto
                                && e.proposta.contains(&(d, Attenuazione::Rinvio))
                        })
                        .and_then(|e| self.partiti.get(e.partito))
                        .map_or("", |p| p.nome.as_str());
                    (
                        TipoVoce::ValoreRinviato,
                        format!(
                            "{} passa a {nuovo} solo da {}",
                            x.nome,
                            parlamento::mese(data_di(t))
                        ),
                        format!(
                            "l'emendamento accolto di {chi}, che rinvia il valore di {rinvio} turni"
                        ),
                    )
                }
                None => {
                    let vigente = (self.disposizioni.voci.get(d))
                        .and_then(|v| x.gradini.get(v.in_vigore))
                        .map_or("", |g| g.nome.as_str());
                    (
                        TipoVoce::ValoreNonApplicato,
                        format!(
                            "{} resta a {vigente}, non a {nuovo}, nonostante la legge approvata",
                            x.nome
                        ),
                        format!("il vincolo del memorandum del MES, fino a {fine}"),
                    )
                }
            };
            self.voce_misura(tipo, m, fatto, causa);
        }
    }

    /// Fatti 38, 75, 101: la misura `m` decaduta senza voto, con la `causa`.
    pub(super) fn voce_decaduta(&mut self, m: &Misura, causa: String) {
        let nome = parlamento::nome_misura(&self.dati, m);
        let fatto = format!("Decade senza voto «{nome}»");
        self.voce_misura(TipoVoce::MisuraDecaduta, m, fatto, causa);
    }

    /// Fatti 39, 41, 42: l'esito `e` del referendum `r`, con la valence persa (`persa`). Al governo
    /// è critico se il governo perde: abrogativo approvato, confermativo respinto.
    pub(super) fn voce_referendum_votato(
        &mut self,
        e: &EsitoReferendum,
        r: &Referendum,
        persa: Vec<String>,
    ) {
        let affluenza = virgola(e.votanti as f64 / e.aventi_diritto.max(1) as f64 * 100.0);
        let confermativo = e.tipo == TipoReferendum::Confermativo;
        let (quale, esito, causa) = if confermativo {
            (
                "confermativo",
                if e.approvato { "approvato" } else { "respinto" },
                "il confermativo non ha quorum, decide la maggioranza dei votanti".to_string(),
            )
        } else {
            let esito = match (e.quorum, e.approvato) {
                (false, _) => "fallito senza quorum",
                (true, true) => "approvato",
                (true, false) => "respinto",
            };
            let mut causa = format!("affluenza del {affluenza}% contro il quorum del 50%");
            if r.invito_astensione {
                causa.push_str(", con l'invito del governo all'astensione");
            }
            ("abrogativo", esito, causa)
        };
        let mut fatto = format!(
            "Referendum {quale} su «{}» {esito}; {} sì e {} no, affluenza del {affluenza}%",
            e.nome,
            migliaia(e.si as i64),
            migliaia(e.no as i64)
        );
        for parte in persa {
            fatto.push_str("; ");
            fatto.push_str(&parte);
        }
        let perso = e.approvato != confermativo;
        let (al_governo, partito) = (self.al_governo(), self.id_di(r.promotore));
        let disposizioni = self.disposizioni_di(&r.valori);
        let Some(v) = self.voce(TipoVoce::ReferendumVotato, fatto, causa) else {
            return;
        };
        if perso && al_governo {
            v.con_grado(Grado::C2);
        }
        v.partito = partito;
        // Solo il confermativo ha la sua legge; una voce senza misura non porta disposizioni.
        if r.misura.is_some() {
            v.misura = r.misura;
            v.disposizioni = disposizioni;
        }
    }

    /// Fatti 43 e 76: l'abrogativo `e` di `promotore` decaduto, o sospeso, senza voto.
    pub(super) fn voce_referendum_decaduto(
        &mut self,
        e: &EsitoReferendum,
        promotore: Option<usize>,
        sospeso: bool,
        causa: String,
    ) {
        let verbo = if sospeso { "Sospeso" } else { "Decade" };
        let fatto = format!(
            "{verbo} senza voto il referendum abrogativo su «{}»",
            e.nome
        );
        let partito = self.id_di(promotore);
        if let Some(v) = self.voce(TipoVoce::ReferendumDecaduto, fatto, causa) {
            v.partito = partito;
        }
    }

    /// Fatto 40: la costituzionale del confermativo `r`, che nessuno ha chiesto, in vigore.
    pub(super) fn voce_senza_voto(&mut self, r: &Referendum) {
        let nome = parlamento::nome(&self.dati, &r.valori);
        let fatto = format!("Entra in vigore senza referendum «{nome}»");
        let causa = "nessuno ha chiesto il referendum confermativo".to_string();
        let disposizioni = self.disposizioni_di(&r.valori);
        if let Some(v) = self.voce(TipoVoce::ConfermativoSenzaVoto, fatto, causa) {
            v.misura = r.misura;
            v.disposizioni = disposizioni;
        }
    }

    /// Fatto 45: le firme dell'abrogativo `r`, chiesto nel turno, raccolte.
    pub(super) fn voce_firme(&mut self, r: &Referendum) {
        let nome = (r.valori.first())
            .and_then(|&(d, _)| self.dati.disposizioni().get(d))
            .map_or(String::new(), |x| x.nome.clone());
        let fatto = format!(
            "Indetto il referendum abrogativo su {nome}, si vota in {}",
            parlamento::mese(data_di(r.voto))
        );
        let causa = "firme depositate nei termini".to_string();
        let partito = self.id_di(r.promotore);
        if let Some(v) = self.voce(TipoVoce::FirmeRaccolte, fatto, causa) {
            v.partito = partito;
        }
    }

    /// «al governo», «individuale a Nome (Sanità)», o «individuale su Sanità» senza ministro.
    fn bersaglio(&self, dicastero: Option<usize>, ministro: Option<&str>) -> String {
        match (
            dicastero.and_then(|i| self.dati.dicasteri().get(i)),
            ministro,
        ) {
            (Some(d), Some(m)) => format!("individuale a {m} ({})", d.nome),
            (Some(d), None) => format!("individuale su {}", d.nome),
            (None, _) => "al governo".to_string(),
        }
    }

    /// Fatti 46, 47, 98: la sfiducia `e` votata con la `linea`, al governo o al `ministro` del
    /// `dicastero`; `coda` segue i conteggi nel fatto.
    pub(super) fn voce_sfiducia(
        &mut self,
        e: &EsitoVoto,
        (dicastero, ministro): (Option<usize>, Option<&str>),
        promotore: Option<usize>,
        linea: &[bool],
        coda: Vec<String>,
    ) {
        let verbo = if e.approvata { "Approvata" } else { "Respinta" };
        let mut fatto = format!(
            "{verbo} la sfiducia {}; {}",
            self.bersaglio(dicastero, ministro),
            conteggi(e)
        );
        for parte in coda {
            fatto.push_str("; ");
            fatto.push_str(&parte);
        }
        let mut causa = self.linee(e);
        let in_ultimatum: Vec<&str> = (self.alleati_in_ultimatum().into_iter())
            .filter(|&j| linea.get(j) == Some(&true))
            .filter_map(|j| self.partiti.get(j).map(|p| p.nome.as_str()))
            .collect();
        if e.approvata && !in_ultimatum.is_empty() {
            causa.push_str(&format!(
                "; in ultimatum votano sì {}",
                in_ultimatum.join(", ")
            ));
        }
        let tipo = if e.approvata {
            TipoVoce::SfiduciaApprovata
        } else {
            TipoVoce::SfiduciaRespinta
        };
        let id = dicastero.and_then(|i| self.dati.dicasteri().get(i).map(|d| d.id.clone()));
        let partito = self.id_di(promotore);
        if let Some(v) = self.voce(tipo, fatto, causa) {
            v.dicastero = id;
            v.partito = partito;
        }
    }

    /// Fatto 48: la mozione di `promotore` sul `dicastero` (o sul governo) tolta senza voto.
    pub(super) fn voce_sfiducia_scartata(
        &mut self,
        dicastero: Option<usize>,
        promotore: Option<usize>,
        causa: &str,
    ) {
        let fatto = format!(
            "Tolta senza voto la mozione di sfiducia {}",
            self.bersaglio(dicastero, None)
        );
        let id = dicastero.and_then(|i| self.dati.dicasteri().get(i).map(|d| d.id.clone()));
        let partito = self.id_di(promotore);
        if let Some(v) = self.voce(TipoVoce::SfiduciaScartata, fatto, causa.into()) {
            v.dicastero = id;
            v.partito = partito;
        }
    }
}
