//! Le voci dell'esito scritte dalla partita, nel punto del fatto.

use super::ia::elenco;
use super::vita::virgola;
use super::{Partita, Trimestre};
use crate::disposizioni::Arrivati;
use crate::disposizioni::data_di;
use crate::economia::Economia;
use crate::elettorato::RedditoFascia;
use crate::esito::{Grado, TipoVoce, VoceEsito, grado};
use crate::eventi::rischio::Candidato;
use crate::eventi::situazioni::trimestri;
use crate::eventi::{Bersaglio, Causa as EventoCausa, DatoOpzione, EventoAperto, Tono, emergenze};
use crate::informazione::{Incrocio, Sondaggio};
use crate::legislatura::campagna::{self, migliaia};
use crate::legislatura::{self, Fase, Incarico, OrigineCaduta, Spesa, quota_fra_i_votanti};
use crate::ministri::{Causa, FattoAtto, Ministro, Proposta, causa_peggiore};
use crate::parlamento::{
    self, Attenuazione, CausaIntesa, Conteggio, EsitoReferendum, EsitoVoto, FormaMisura, Misura,
    Referendum, StatoEmendamento, TipoReferendum,
};
use crate::scenario::StatoPde;
use crate::vista::Fermata;

/// «+1,2» o «-0,3»: una cifra decimale, col segno.
fn segnato(x: f64) -> String {
    format!("{x:+.1}").replace('.', ",")
}

/// «12 seggi», «1 seggio».
fn seggi(n: u32) -> String {
    if n == 1 {
        "1 seggio".to_string()
    } else {
        format!("{n} seggi")
    }
}

/// «luglio-settembre 2026».
fn trimestre(t: Trimestre) -> String {
    let mesi = match t.trimestre {
        1 => "gennaio-marzo",
        2 => "aprile-giugno",
        3 => "luglio-settembre",
        _ => "ottobre-dicembre",
    };
    format!("{mesi} {}", t.anno)
}

fn stato_pde(s: StatoPde) -> &'static str {
    match s {
        StatoPde::Nessuna => "nessuna",
        StatoPde::Aperta => "aperta",
        StatoPde::Sospesa => "sospesa",
        StatoPde::Inadempiente => "inadempiente",
        StatoPde::Intimata => "intimata",
        StatoPde::Sanzionata => "sanzionata",
    }
}

/// Il rango dello stato della procedura, più alto se più avanti verso la sanzione: la sospesa sta
/// fra nessuna procedura e l'aperta, perché la sospensione è il premio di chi corregge.
fn rango_pde(s: StatoPde) -> u8 {
    match s {
        StatoPde::Nessuna => 0,
        StatoPde::Sospesa => 1,
        StatoPde::Aperta => 2,
        StatoPde::Inadempiente => 3,
        StatoPde::Intimata => 4,
        StatoPde::Sanzionata => 5,
    }
}

/// `s` con la prima lettera minuscola: il nome di una grandezza dentro una frase.
fn minuscola(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map_or(String::new(), |p| p.to_lowercase().chain(c).collect())
}

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

/// «perde 1,25» o «guadagna 0,40»: una variazione di soddisfazione o d'intesa.
fn perde_o_guadagna(delta: f64) -> String {
    let verbo = if delta < 0.0 { "perde" } else { "guadagna" };
    format!(
        "{verbo} {}",
        format!("{:.2}", delta.abs()).replace('.', ",")
    )
}

/// Quante variazioni del turno corrente la chiusura ha già letto: di soddisfazione per dicastero,
/// d'intesa in tutto. Le altre del turno sono nate dopo, e diventano voci. Basta il conto: le
/// variazioni si accodano in ordine di turno, e i `retain` tolgono solo quelle di turni passati.
pub(super) struct Lette {
    ministri: Vec<usize>,
    intesa: usize,
}

/// «{cosa} è scesa sotto la soglia {quale} di 40,0; la causa peggiore è «…»», o la sola soglia
/// senza una causa.
fn sotto_la_soglia(cosa: &str, quale: &str, soglia: f64, peggiore: Option<String>) -> String {
    let mut causa = format!(
        "{cosa} è scesa sotto la soglia {quale} di {}",
        virgola(soglia)
    );
    if let Some(c) = peggiore {
        causa.push_str(&format!("; la causa peggiore è «{c}»"));
    }
    causa
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
    /// `cadute`; la causa dice la scissione del suo partito in questa chiusura e, senza
    /// attribuirgli il taglio, gli `acquistato` euro d'informazione del turno: comprati prima del
    /// comando «Campagna» non l'hanno tagliata.
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
        let mut causa = "la cassa è scesa dopo la decisione".to_string();
        if scissa {
            causa.push_str(", con la scissione");
        }
        if acquistato > 0 {
            causa.push_str(&format!(
                "; nel turno gli acquisti d'informazione sono costati {} euro",
                migliaia(acquistato)
            ));
        }
        self.voce_di(self.giocatore, TipoVoce::CampagnaTagliata, fatto, causa);
    }

    /// Fatti 57, 59-62, 66: le voci del motore, dal confronto con l'economia `prima` del passo.
    /// Solo dalla chiusura vera: `motore` gira anche sulle copie di `avanza_copia`.
    pub(super) fn voci_del_motore(&mut self, prima: &Economia) {
        let m = |nome: &str| self.dati.manopola(nome).round();
        let (durata, turni) = (m("durata_mes"), m("turni_ristrutturazione"));
        // Nella chiusura che apre il turno `fine`: già in quel turno i vincoli non valgono.
        let dopo = self.turno.saturating_add(1);
        if prima.mes_in_corso(self.turno).is_some() && self.economia.mes_in_corso(dopo).is_none() {
            self.voce(
                TipoVoce::MesFinito,
                "Finito il programma del MES: cadono i vincoli del memorandum".into(),
                format!("la durata del programma, {durata} trimestri"),
            );
        }
        // Cast: lo spread è limitato da `spread_massimo` e non è mai NaN.
        let (ora, pb) = (
            self.economia.spread.round() as i32,
            prima.spread.round() as i32,
        );
        let fatto = format!("Spread a {ora} pb, {:+} pb sul trimestre prima", ora - pb);
        self.voce(TipoVoce::Spread, fatto, "chiusura del trimestre".into());
        self.voci_regole_ue(prima);
        if prima.ristrutturazione.is_some() && self.economia.ristrutturazione.is_none() {
            self.voce(
                TipoVoce::RistrutturazioneFinita,
                "Finita la ristrutturazione del debito: il premio sullo spread è rientrato e i \
                 redditi delle fasce media e alta non calano più"
                    .into(),
                format!(
                    "premio sotto 1 punto base e {turni} turni di perdita dei redditi trascorsi"
                ),
            );
        }
    }

    /// Fatto 67: una voce per fascia con la crescita reale sotto l'attesa di almeno
    /// `scarto_reddito_deluso` punti; l'attesa è la media delle `ricordate` crescite passate.
    pub(super) fn voci_reddito(&mut self, redditi: &[RedditoFascia], ricordate: usize) {
        let scarto = self.dati.manopola("scarto_reddito_deluso");
        // Due decimali: con una sola cifra uno scarto di 0,25 punti si leggerebbe 0,2 o 0,3.
        let punti = |x: f64| format!("{x:+.2}").replace('.', ",");
        let causa = match ricordate {
            0 => "nessun trimestre passato da ricordare, l'attesa parte da zero".to_string(),
            1 => "l'attesa è la crescita reale del trimestre prima".to_string(),
            n => format!("l'attesa è la media delle crescite reali degli ultimi {n} trimestri"),
        };
        for r in redditi.iter().filter(|r| r.attesa - r.crescita >= scarto) {
            let fatto = format!(
                "Reddito reale della fascia {} a {}% nel trimestre contro il {}% atteso, giudizio \
                 sul governo in calo",
                r.fascia,
                punti(r.crescita),
                punti(r.attesa)
            );
            self.voce(TipoVoce::RedditoDeluso, fatto, causa.clone());
        }
    }

    /// Fatti 60-62: controllo annuale, procedura per disavanzo eccessivo e ammenda, dalle regole
    /// UE `prima` del passo e da quelle di ora.
    fn voci_regole_ue(&mut self, prima: &Economia) {
        let m = |nome: &str| self.dati.manopola(nome);
        let r = &self.economia.regole_ue;
        let a1 = self.data.anno.saturating_sub(1);
        // Le condizioni di `controllo_annuale`: senza un anno nei conti esce senza toccare nulla.
        let conti = (
            r.spesa_netta.get(&a1),
            r.spesa_netta.get(&a1.saturating_sub(1)),
            r.pil_annuo.get(&a1),
            r.disavanzo_annuo.get(&a1),
        );
        let controllo = match conti {
            (Some(&sn1), Some(&sn2), Some(&pil1), Some(&dis1)) if self.data.trimestre == 2 => {
                Some((sn1 as f64, sn2 as f64, pil1 as f64, dis1 as f64))
            }
            _ => None,
        };
        let (deviazione, conto, pde) = (r.ultima_deviazione, r.conto_controllo, r.pde);
        let rilevante = deviazione > m("deviazione_annua_rilevante")
            || conto > m("deviazione_cumulata_rilevante");
        let (pde_prima, ammende) = (prima.regole_ue.pde, self.economia.conti.spese.ammende);
        if let Some((sn1, sn2, pil1, _)) = controllo {
            let tetto = self.dati.scenario().tetto_spesa_netta(a1);
            // `deviazione` è già al netto della clausola: la differenza dalla lorda è la clausola.
            let clausola = (sn1 - sn2 * (1.0 + tetto / 100.0)) / pil1 * 100.0 - deviazione;
            let netto = if clausola.abs() < 1e-9 {
                ", senza clausola".to_string()
            } else {
                format!(", al netto della clausola di {} punti", virgola(clausola))
            };
            let verso = if deviazione > 0.0 { "oltre" } else { "sotto" };
            let fatto = format!(
                "Controllo UE sul {a1}: spesa netta {verso} il percorso di {} punti di PIL{netto}; \
                 conto di controllo a {} punti di PIL",
                virgola(deviazione.abs()),
                virgola(conto)
            );
            let causa = format!(
                "la spesa netta del {a1} cresce del {}% contro il tetto del {}%",
                virgola((sn1 / sn2 - 1.0) * 100.0),
                virgola(tetto)
            );
            self.voce(TipoVoce::ControlloUe, fatto, causa);
        }
        if pde != pde_prima {
            let fatto = match (pde_prima, pde) {
                (StatoPde::Nessuna, _) => "Aperta la procedura per disavanzo eccessivo".to_string(),
                (a, StatoPde::Nessuna) => format!(
                    "Chiusa la procedura per disavanzo eccessivo, che era {}",
                    stato_pde(a)
                ),
                (a, b) => format!(
                    "La procedura per disavanzo eccessivo passa da {} a {}",
                    stato_pde(a),
                    stato_pde(b)
                ),
            };
            let deviazione = if rilevante {
                "deviazione rilevante della spesa netta"
            } else {
                "deviazione non rilevante della spesa netta"
            };
            let causa = match (pde, controllo) {
                (StatoPde::Intimata, _) => {
                    "nessuna correzione entro l'anno dall'inadempienza (articolo 126.9)".to_string()
                }
                (StatoPde::Sanzionata, _) => {
                    "nessuna correzione due turni dopo l'intimazione (articolo 126.11)".to_string()
                }
                (_, Some((_, _, pil1, dis1))) => format!(
                    "{deviazione}, disavanzo del {a1} al {}% del PIL",
                    virgola(dis1 / pil1 * 100.0)
                ),
                (_, None) => deviazione.to_string(),
            };
            let grado = if self.al_governo() {
                Grado::C2
            } else {
                Grado::N
            };
            if let Some(v) = self.voce(TipoVoce::ProceduraUe, fatto, causa)
                && rango_pde(pde) > rango_pde(pde_prima)
            {
                v.con_grado(grado);
            }
        }
        if ammende > 0 {
            // L'ammenda si calcola sul PIL di prima del passo, in `finanza`.
            let quota = ammende as f64 / (prima.pil_trimestrale * 4.0 * 1e9) * 100.0;
            let fatto = format!(
                "Pagata un'ammenda UE di {} euro, pari a {}% del PIL",
                migliaia(ammende),
                format!("{quota:.2}").replace('.', ",")
            );
            let causa = match pde_prima {
                StatoPde::Inadempiente => {
                    "procedura per disavanzo eccessivo inadempiente, ammenda del primo turno"
                }
                _ => "procedura per disavanzo eccessivo sanzionata, ammenda semestrale",
            };
            self.voce(TipoVoce::AmmendaUe, fatto, causa.into());
        }
    }

    /// Fatto 69: il sondaggio pubblico appena rilevato contro quello `prima`, per la coalizione
    /// del giocatore o, se corre da solo, per il suo partito.
    pub(super) fn voce_sondaggio(&mut self, prima: &Sondaggio) {
        let g = self.giocatore;
        let coalizione = (self.coalizioni_correnti().into_iter())
            .find(|c| c.contains(&g))
            .unwrap_or_else(|| vec![g]);
        let quota = |s: &Sondaggio| {
            100.0
                * (coalizione.iter())
                    .map(|&j| quota_fra_i_votanti(s, j))
                    .sum::<f64>()
        };
        let (ora, prima) = (quota(&self.informazione.sondaggio), quota(prima));
        let nomi: Vec<&str> = (coalizione.iter())
            .filter_map(|&j| self.partiti.get(j).map(|p| p.nome.as_str()))
            .collect();
        let chi = match nomi.as_slice() {
            [nome] => nome.to_string(),
            _ => format!("la coalizione di {}", nomi.join(", ")),
        };
        let fatto = format!(
            "Sondaggio: {chi} al {}% fra i votanti, {} punti sul trimestre prima",
            virgola(ora),
            segnato(ora - prima)
        );
        let causa = format!(
            "sondaggio pubblico del trimestre, media di {} istituti",
            self.informazione.sondaggio.istituti.len()
        );
        self.voce_di(g, TipoVoce::Sondaggio, fatto, causa);
    }

    /// Fatto 70: la prima stima del trimestre chiuso e la revisione di quella prima.
    pub(super) fn voci_statistica(&mut self) {
        let s = &self.informazione.statistica;
        let p = &s.prima_stima;
        let fatto = format!(
            "Prima stima di {}: PIL {}% annualizzato, inflazione {}% annualizzata e {}% \
             sull'anno, disoccupazione {}%, disavanzo {}% del PIL",
            trimestre(s.trimestre),
            segnato(p.crescita_pil),
            virgola(p.inflazione),
            virgola(p.inflazione_annua),
            virgola(p.disoccupazione),
            virgola(p.disavanzo)
        );
        let revisione = s.revisione.as_ref().map(|r| {
            let (a, b) = (&r.prima_stima, &r.riveduta);
            format!(
                "Revisione di {}: PIL da {}% a {}%, inflazione da {}% a {}%, disoccupazione da \
                 {}% a {}%, disavanzo da {}% a {}% del PIL",
                trimestre(r.trimestre),
                segnato(a.crescita_pil),
                segnato(b.crescita_pil),
                virgola(a.inflazione),
                virgola(b.inflazione),
                virgola(a.disoccupazione),
                virgola(b.disoccupazione),
                virgola(a.disavanzo),
                virgola(b.disavanzo)
            )
        });
        let causa = "prima stima, riveduta fra un turno".to_string();
        self.voce(TipoVoce::PrimaStima, fatto, causa);
        if let Some(fatto) = revisione {
            self.voce(
                TipoVoce::Revisione,
                fatto,
                "revisione della prima stima".into(),
            );
        }
    }

    /// Il nome della misura di una situazione dentro una frase, con l'unità dei suoi valori.
    fn misura_a_parole(&self, misura: &str) -> (String, &'static str) {
        match misura {
            "crescita_pil" => ("crescita del PIL".into(), "% annuo"),
            "energia" => ("scarto dei prezzi dell'energia".into(), ""),
            id => {
                let nome = (self.dati.grandezza(id))
                    .and_then(|k| self.dati.grandezze().get(k))
                    .map_or(id, |g| g.nome.as_str());
                (minuscola(nome), "")
            }
        }
    }

    /// Fatti 71 e 72: le situazioni accese o spente in questa chiusura, `accese_prima` contro le
    /// accese di ora, col valore `ufficiali` che la Vista mostra.
    pub(super) fn voci_situazioni(&mut self, accese_prima: &[bool], ufficiali: &[f64]) {
        let righe = (self.dati.situazioni().iter().enumerate()).zip(&self.eventi.accese);
        let cambi: Vec<(usize, TipoVoce, String, String)> = righe
            .filter_map(|((i, s), &ora)| {
                let (tipo, verbo, quale, soglia) = match (accese_prima.get(i)?, ora) {
                    (false, true) => (
                        TipoVoce::SituazioneAccesa,
                        "accende",
                        "oltrepassata la soglia di accensione",
                        s.accensione,
                    ),
                    (true, false) => (
                        TipoVoce::SituazioneSpenta,
                        "spegne",
                        "raggiunta la soglia di spegnimento",
                        s.spegnimento,
                    ),
                    _ => return None,
                };
                let (misura, unita) = self.misura_a_parole(&s.misura);
                let fila = match trimestri(&s.misura) {
                    1 => String::new(),
                    n => format!(", per {n} trimestri di fila"),
                };
                let fatto = format!(
                    "Si {verbo} la situazione «{}»: {misura} a {}{unita}",
                    s.nome,
                    virgola(*ufficiali.get(i)?)
                );
                let causa = format!("{quale} di {}{unita}{fila}", virgola(soglia));
                Some((i, tipo, fatto, causa))
            })
            .collect();
        for (i, tipo, fatto, causa) in cambi {
            // Spec, «Situazione accesa ed evento nato»: se l'evento della situazione è nato ora,
            // la sua voce porta già valore ufficiale e soglia, e la situazione tace.
            match (tipo == TipoVoce::SituazioneAccesa)
                .then(|| self.nato_dalla_situazione(i))
                .flatten()
            {
                Some(aperto) => self.voce_nato(&aperto),
                None => {
                    self.voce(tipo, fatto, causa);
                }
            }
        }
    }

    /// L'evento della situazione `i` aperto in questa chiusura (`situazioni::apri`), se c'è.
    fn nato_dalla_situazione(&self, i: usize) -> Option<EventoAperto> {
        let evento = self.dati.evento(&self.dati.situazioni().get(i)?.evento)?;
        let turno = self.turno.saturating_add(1);
        (self.eventi.aperti.iter())
            .find(|a| a.evento == evento && a.turno == turno)
            .cloned()
    }

    /// Fatto 113: l'evento `a` nato in questa chiusura, con la causa della `VoceEvento`.
    pub(super) fn voce_nato(&mut self, a: &EventoAperto) {
        let Some(e) = self.dati.eventi().get(a.evento) else {
            return;
        };
        let (nome, famiglia) = (e.nome.clone(), e.famiglia);
        let (mut dicastero, mut partito, mut di) = (None, None, None);
        match a.bersaglio {
            Some(Bersaglio::Ministro { dicastero: k }) => {
                let d = self.dati.dicasteri().get(k);
                dicastero = d.map(|d| d.id.clone());
                di = d.map(|d| d.nome.clone());
            }
            Some(Bersaglio::Leader { partito: j }) => {
                let p = self.partiti.get(j);
                partito = p.map(|p| p.id.clone());
                di = p.map(|p| p.nome.clone());
            }
            None => {}
        }
        let chi = match (&a.persona, di) {
            (Some(persona), Some(di)) => format!(" ({persona}, {di})"),
            (Some(persona), None) => format!(" ({persona})"),
            _ => String::new(),
        };
        let mut fatto = format!("{nome}{}{chi}", self.in_regione(a.regione));
        if a.costo > 0 {
            fatto.push_str(&format!(
                ", costo dell'intervento {} euro",
                migliaia(a.costo)
            ));
        }
        let causa = a.causa.testo(&self.dati);
        let regione = self.id_regione(a.regione);
        if let Some(v) = self.voce(TipoVoce::EventoNato, fatto, causa) {
            v.tono = a.tono;
            v.famiglia = Some(famiglia);
            v.regione = regione;
            v.dicastero = dicastero;
            v.partito = partito;
        }
    }

    /// Fatto 115: lo shock `c` uscito ma escluso dal tetto di un evento a turno. Gli altri esclusi
    /// (emergenze, scandali) non sono nati e non sono voci.
    pub(super) fn voce_assorbito(&mut self, c: &Candidato) {
        let EventoCausa::Shock {
            serie,
            sigma,
            soglia,
        } = &c.causa
        else {
            return;
        };
        let Some(e) = self.dati.eventi().get(c.evento) else {
            return;
        };
        let default = (e.opzioni.iter().find(|o| o.default)).map_or("", |o| o.nome.as_str());
        let fatto = format!(
            "{} senza evento: ne nasce uno solo a turno, la serie si muove lo stesso e vale \
             «{default}»",
            e.nome
        );
        let causa = format!(
            "serie «{serie}», shock di {} sigma oltre la soglia di {} sigma",
            virgola(*sigma),
            virgola(*soglia)
        );
        if let Some(v) = self.voce(TipoVoce::ShockAssorbito, fatto, causa) {
            v.tono = c.tono;
        }
    }

    /// Fatto 73: le «Aste scoperte» (`evento`) aperte con lo spread a `pb` e la `soglia`.
    pub(super) fn voce_crisi(&mut self, evento: usize, pb: i32, soglia: i32) {
        let nome = (self.dati.eventi().get(evento)).map_or("", |e| e.nome.as_str());
        let fatto = format!("Spread a {pb} pb: si aprono le «{nome}»");
        let causa = format!(
            "lo spread ha raggiunto la soglia di crisi di {soglia} pb, senza programma del MES né \
             ristrutturazione in corso"
        );
        if let Some(v) = self.voce(TipoVoce::CrisiDelDebito, fatto, causa) {
            v.tono = Tono::Grave;
        }
    }

    /// Fatto 74: le `entrate` del partito `j`, col 2 per mille sulla `quota` fra i votanti
    /// dell'ultimo voto (sull'intenzione, prima del primo voto) e il resto in donazioni.
    pub(super) fn voce_entrate(&mut self, j: usize, entrate: i64, quota: f64, primo_voto: bool) {
        let m = |nome: &str| self.dati.manopola(nome);
        let dpm = (m("due_per_mille_turno") * quota * m("quota_libera_entrate")).round() as i64;
        let Some(p) = self.partiti.get(j).filter(|_| entrate > 0) else {
            return;
        };
        let fatto = format!(
            "{} incassa {} euro, {} dal 2 per mille e {} di donazioni",
            p.nome,
            migliaia(entrate),
            migliaia(dpm),
            migliaia(entrate - dpm)
        );
        let causa = if primo_voto {
            "prima del primo voto, 2 per mille e donazioni sulle intenzioni di voto".to_string()
        } else {
            format!(
                "2 per mille sul {}% dell'ultimo voto, donazioni sulle intenzioni di voto",
                virgola(quota * 100.0)
            )
        };
        self.voce_di(j, TipoVoce::EntrateDiCassa, fatto, causa);
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

    /// Fatti 116 e 117: all'apertura, i valori in arrivo entrati in vigore e gli elettorali scartati.
    pub(super) fn voci_arrivi(&mut self, a: Arrivati) {
        // Il motivo viene dagli errori di `valida`: niente «: », perché la voce si fonde.
        let motivo = a.motivo.replace(": ", ", ");
        let righe = (a.entrati.into_iter().map(|x| (x, true)))
            .chain(a.scartati.into_iter().map(|x| (x, false)));
        for ((d, arrivo), entrato) in righe {
            let Some(x) = self.dati.disposizioni().get(d) else {
                continue;
            };
            let nuovo = x
                .gradini
                .get(arrivo.gradino)
                .map_or("", |g| g.nome.as_str());
            let (tipo, fatto, causa) = if entrato {
                (
                    TipoVoce::ValoreInVigore,
                    format!(
                        "{} passa a {nuovo}: entra in vigore il valore in arrivo",
                        x.nome
                    ),
                    format!("valore fissato da «{}»", arrivo.legge),
                )
            } else {
                let vigente = (self.disposizioni.voci.get(d))
                    .and_then(|v| x.gradini.get(v.in_vigore))
                    .map_or("", |g| g.nome.as_str());
                (
                    TipoVoce::ArrivoScartato,
                    format!(
                        "{} resta a {vigente}, non a {nuovo}: il valore in arrivo è scartato",
                        x.nome
                    ),
                    format!("la legge elettorale che ne uscirebbe non sarebbe valida, {motivo}"),
                )
            };
            self.voce(tipo, fatto, causa);
        }
    }

    /// Fatto 122: gli emendamenti `chiesti` all'apertura, come (id della misura, alleato).
    pub(super) fn voci_emendamenti_chiesti(&mut self, chiesti: &[(u32, usize)]) {
        for &(id, j) in chiesti {
            let Some(m) = self.parlamento.misure.iter().find(|m| m.id == id) else {
                continue;
            };
            let Some(e) = m.emendamenti.iter().find(|e| e.partito == j) else {
                continue;
            };
            let chi = self.partiti.get(j).map_or("", |p| p.nome.as_str());
            let fatto = format!(
                "{chi} chiede di emendare «{}»: {}",
                parlamento::nome_misura(&self.dati, m),
                parlamento::testo(&self.dati, &e.proposta)
            );
            let causa = e.motivo.clone();
            let disposizioni = self.disposizioni_di(&m.valori);
            if let Some(v) = self.voce_di(j, TipoVoce::EmendamentoChiesto, fatto, causa) {
                v.misura = Some(id);
                v.disposizioni = disposizioni;
            }
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

    /// Fatti 51, 96, 100: il governo cade alla chiusura, per l'`origine` delle dimissioni.
    pub(super) fn voce_caduto(&mut self, origine: OrigineCaduta) {
        let pc = self.governo.presidente;
        let nome = self.partiti.get(pc).map_or("", |p| p.nome.as_str());
        let fatto = format!("Cade il governo di {nome}");
        let causa = match origine {
            OrigineCaduta::EsercizioProvvisorio => {
                "l'esercizio provvisorio senza legge di bilancio approvata"
            }
            OrigineCaduta::FiduciaRespinta => "la fiducia respinta in aula",
            OrigineCaduta::Sfiducia => "la mozione di sfiducia approvata",
            OrigineCaduta::DimissioniDelPresidente => "le dimissioni del Presidente del Consiglio",
            OrigineCaduta::CrisiSenzaUscita => "la crisi di governo senza uscita",
            OrigineCaduta::VotoAnticipato => {
                "le dimissioni per il voto anticipato: nei sondaggi il voto gli vale più che \
                 restare al governo"
            }
        };
        self.voce_di(pc, TipoVoce::GovernoCaduto, fatto, causa.into());
    }

    /// I nomi dei partiti `js`, «A, B e C».
    pub(super) fn nomi(&self, js: &[usize]) -> String {
        let nomi: Vec<String> = (js.iter().filter_map(|&j| self.partiti.get(j)))
            .map(|p| p.nome.clone())
            .collect();
        elenco(&nomi)
    }

    /// «N seggi su C alla Camera e M su S al Senato»: i seggi della `coalizione`.
    fn seggi_di(&self, coalizione: &[usize]) -> String {
        let pl = &self.parlamento;
        let somma = |v: &[u32]| coalizione.iter().filter_map(|&j| v.get(j)).sum::<u32>();
        let (c, s) = self.legislatura.totali;
        format!(
            "{} su {c} alla Camera e {} su {s} al Senato",
            seggi(somma(&pl.seggi_camera)),
            somma(&pl.seggi_senato)
        )
    }

    /// Fatti 78-80, in coda al voto: il risultato, poi l'incarico o, col giocatore senza seggi,
    /// la fine della partita.
    pub(super) fn voci_del_voto(&mut self) {
        let Some(e) = self.legislatura.elezioni.first() else {
            return;
        };
        let pct = |x: f64| virgola(x * 100.0);
        let nome = |id: &str| {
            (self.partiti.iter().find(|p| p.id == id)).map_or(id.to_string(), |p| p.nome.clone())
        };
        let mio = self.partiti.get(self.giocatore);
        let suo = mio.and_then(|g| e.voti.iter().find(|v| v.partito == g.id));
        let (quota, camera, senato) = suo.map_or((0.0, 0, 0), |v| (v.quota, v.camera, v.senato));
        let mio = mio.map_or(String::new(), |g| g.nome.clone());
        let primo = (e.voti.iter())
            .max_by(|a, b| a.quota.total_cmp(&b.quota))
            .map_or(String::new(), |v| {
                format!("primo partito {} al {}%", nome(&v.partito), pct(v.quota))
            });
        let premio = match &e.premio {
            Some(ids) => {
                let nomi: Vec<String> = ids.split('+').map(nome).collect();
                format!("premio di maggioranza a {}", elenco(&nomi))
            }
            None => "nessun premio di maggioranza".to_string(),
        };
        let fatto = format!(
            "Elezioni politiche: affluenza del {}%; {mio} al {}% con {} alla Camera e {senato} al \
             Senato; {primo}; {premio}",
            pct(e.affluenza),
            pct(quota),
            seggi(camera)
        );
        let fine = format!("al voto il {}% fra i votanti, nessun seggio", pct(quota));
        self.voce(
            TipoVoce::RisultatoDelVoto,
            fatto,
            "elezioni politiche".into(),
        );
        if self.fine.is_some() {
            let fatto =
                format!("{mio} resta senza seggi alla Camera e al Senato: la partita finisce");
            self.voce_di(self.giocatore, TipoVoce::FineDellaPartita, fatto, fine);
        } else {
            self.voce_incarico(false);
        }
    }

    /// Fatti 79 e 103: a chi va l'incarico, dopo il voto o `dopo_la_caduta`. La coalizione
    /// dell'incarico IA non si dice: la Vista mostra solo il Capo dello Stato.
    pub(super) fn voce_incarico(&mut self, dopo_la_caduta: bool) {
        let g = self.giocatore;
        let mio = self.partiti.get(g).map_or("", |p| p.nome.as_str());
        let a_me = format!("Incarico di formare il governo a {mio}");
        let capo = "L'incarico passa al Capo dello Stato".to_string();
        let (fatto, causa, suo) = match (&self.legislatura.incarico, dopo_la_caduta) {
            (Some(Incarico::Giocatore), true) => (
                a_me,
                "sei il primo per seggi della maggioranza contigua trovata dopo la caduta".into(),
                true,
            ),
            (Some(Incarico::Giocatore), false) => {
                let sua = self.coalizione_del_giocatore();
                let pl = &self.parlamento;
                let quale = if legislatura::maggioranza(&pl.seggi_camera, &pl.seggi_senato, &sua) {
                    "la tua coalizione ha la maggioranza in entrambe le Camere"
                } else {
                    "senza maggioranza, la tua coalizione è la prima alla Camera"
                };
                (a_me, format!("{quale}, con {}", self.seggi_di(&sua)), true)
            }
            (Some(Incarico::Guida { coalizione }), _) => {
                let primo: Vec<usize> = self.primo_per_seggi(coalizione).into_iter().collect();
                let primo = self.nomi(&primo);
                (
                    format!("Ribaltone possibile, con la guida a {primo}"),
                    format!(
                        "nella maggioranza contigua trovata dopo la caduta {primo} ha più seggi di te"
                    ),
                    false,
                )
            }
            (Some(Incarico::Ia { .. }), true) => (
                capo,
                "la maggioranza contigua trovata dopo la caduta non ti comprende".into(),
                false,
            ),
            (Some(Incarico::Ia { .. }), false) => (
                capo,
                "un'altra coalizione ha la maggioranza in entrambe le Camere".into(),
                false,
            ),
            (None, _) => (
                capo,
                "nessuna coalizione ha la maggioranza e la tua non è la prima alla Camera".into(),
                false,
            ),
        };
        if suo {
            self.voce_di(g, TipoVoce::Incarico, fatto, causa);
        } else {
            self.voce(TipoVoce::Incarico, fatto, causa);
        }
    }

    /// Fatto 93: gli alleati `escono` dal governo, coi dicasteri che lasciano in interim. La causa
    /// dice la soglia e l'ultimatum, mai l'intesa in cifre.
    pub(super) fn voci_alleati_usciti(&mut self, escono: &[usize]) {
        let soglia = virgola(self.dati.manopola("soglia_uscita"));
        for &j in escono {
            let dicasteri: Vec<&str> = (self.dati.dicasteri().iter())
                .zip(&self.governo.dicasteri)
                .filter(|&(_, &k)| k == j)
                .map(|(d, _)| d.nome.as_str())
                .collect();
            let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
            let fatto = if dicasteri.is_empty() {
                format!("{nome} lascia il governo, senza dicasteri")
            } else {
                format!(
                    "{nome} lascia il governo; dicasteri in interim: {}",
                    dicasteri.join(", ")
                )
            };
            let mut causa = format!("l'intesa è scesa sotto la soglia d'uscita di {soglia}");
            if let Some(Some(c)) = self.intesa.ultimatum.get(j) {
                causa.push_str(&format!(", dopo l'ultimatum per {c}"));
            }
            self.voce_di(j, TipoVoce::AlleatoEsce, fatto, causa);
        }
    }

    /// Fatto 94: i dicasteri che il rimpasto passa di partito, `(dicastero, partito)`.
    pub(super) fn voce_rimpasto(&mut self, cambi: &[(usize, usize)], causa: String) {
        let parti: Vec<String> = (cambi.iter())
            .filter_map(|&(i, j)| {
                let d = self.dati.dicasteri().get(i)?;
                Some(format!("{} a {}", d.nome, self.partiti.get(j)?.nome))
            })
            .collect();
        if !parti.is_empty() {
            let fatto = format!("Rimpasto: {}", parti.join(", "));
            self.voce(TipoVoce::Rimpasto, fatto, causa);
        }
    }

    /// Fatti 95 e 99: la crisi aperta dall'uscita degli `usciti`, o riaperta (`None`) dalla
    /// sfiducia respinta, con i seggi della `coalizione`.
    pub(super) fn voce_crisi_di_governo(&mut self, usciti: Option<&[usize]>, coalizione: &[usize]) {
        let seggi = self.seggi_di(coalizione);
        let (fatto, causa) = match usciti {
            Some(u) => (
                format!(
                    "Crisi di governo: senza {} la coalizione non ha la maggioranza",
                    self.nomi(u)
                ),
                format!("la coalizione ha {seggi}"),
            ),
            None => (
                "Si riapre la crisi di governo: la coalizione non ha ancora la maggioranza".into(),
                format!("la sfiducia respinta, con la coalizione a {seggi}"),
            ),
        };
        self.voce(TipoVoce::CrisiAperta, fatto, causa);
    }

    /// Fatto 97: gli `allargati` entrano al governo alla fine della crisi.
    pub(super) fn voce_allargata(&mut self, allargati: &[usize]) {
        let verbo = if allargati.len() == 1 {
            "Entra"
        } else {
            "Entrano"
        };
        let fatto = format!("{verbo} al governo {}", self.nomi(allargati));
        let causa = "la crisi aperta dalla coalizione senza maggioranza".to_string();
        self.voce_di(
            self.governo.presidente,
            TipoVoce::CoalizioneAllargata,
            fatto,
            causa,
        );
    }

    /// Fatti 102 e 112: le Camere sciolte, col voto nel turno dopo.
    pub(super) fn voce_sciolte(&mut self, causa: &str) {
        let voto = parlamento::mese(data_di(self.turno.saturating_add(1)));
        let fatto = format!("Camere sciolte, si vota a {voto}");
        self.voce(TipoVoce::CamereSciolte, fatto, causa.into());
    }

    /// Fatto 105: il governo del `presidente` con la `coalizione` e la spartizione `assegnati`
    /// (`None` = tecnico), dopo il voto o per ribaltone; `perche` dice la maggioranza.
    pub(super) fn voce_formato(
        &mut self,
        presidente: usize,
        coalizione: &[usize],
        assegnati: &[Option<usize>],
        perche: &str,
    ) {
        let quando = if self.turno <= self.legislatura.inizio {
            "dopo il voto"
        } else {
            "per ribaltone"
        };
        let altri: Vec<usize> = (coalizione.iter().copied())
            .filter(|&j| j != presidente)
            .collect();
        let con = if altri.is_empty() {
            "da solo".to_string()
        } else {
            format!("con {}", self.nomi(&altri))
        };
        let quanti = |a: Option<usize>| assegnati.iter().filter(|&&x| x == a).count();
        let mut parti: Vec<String> = (std::iter::once(presidente).chain(altri))
            .filter_map(|j| Some((quanti(Some(j)), &self.partiti.get(j)?.nome)))
            .filter(|&(n, _)| n > 0)
            .map(|(n, nome)| format!("{n} a {nome}"))
            .collect();
        if quanti(None) > 0 {
            parti.push(format!("{} a tecnici", quanti(None)));
        }
        let pc = self.partiti.get(presidente).map_or("", |p| p.nome.as_str());
        let fatto = format!(
            "Nasce il governo di {pc} {quando}, {con}; dicasteri: {}",
            parti.join(", ")
        );
        self.voce_di(presidente, TipoVoce::GovernoFormato, fatto, perche.into());
    }

    /// Fatto 108: `j` della `coalizione` rompe l'impegno pre-elettorale con gli `entranti`, come
    /// lo decide `legislatura::rotture`. L'intesa persa si dice solo per gli alleati del giocatore.
    pub(super) fn voce_rottura(
        &mut self,
        j: usize,
        coalizione: &[usize],
        entranti: &[usize],
        presidente: usize,
    ) {
        let ultime = &self.legislatura.coalizioni;
        let campo = |k: usize| (ultime.iter()).find(|c| c.len() >= 2 && c.contains(&k));
        let mio = campo(j);
        let fuori: Vec<usize> = (mio.into_iter().flatten().copied())
            .filter(|k| entranti.contains(&j) && !coalizione.contains(k))
            .collect();
        let altri: Vec<usize> = (entranti.iter().copied())
            .filter(|&k| k != j && campo(k).is_some_and(|c| Some(c) != mio))
            .collect();
        let compagni: Vec<usize> = (mio.into_iter().flatten().copied())
            .filter(|&k| k != j)
            .collect();
        let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
        let mut fatto = if fuori.is_empty() {
            format!(
                "{nome} rompe l'impegno pre-elettorale governando con {}",
                self.nomi(&altri)
            )
        } else {
            let resta = if fuori.len() == 1 { "resta" } else { "restano" };
            format!(
                "{nome} rompe l'impegno pre-elettorale con {}, che {resta} fuori dal governo",
                self.nomi(&fuori)
            )
        };
        let m = |nome: &str| self.dati.manopola(nome);
        fatto.push_str("; ");
        fatto.push_str(&valence(-m("valence_rottura_impegno"), nome));
        if j != presidente && presidente == self.giocatore {
            let intesa = virgola(m("intesa_rottura_impegno"));
            fatto.push_str(&format!("; perde {intesa} punti d'intesa"));
        }
        let causa = if compagni.is_empty() {
            "ha corso da solo al voto".to_string()
        } else {
            format!(
                "la coalizione dichiarata prima del voto con {}",
                self.nomi(&compagni)
            )
        };
        self.voce_di(j, TipoVoce::RotturaDellImpegno, fatto, causa);
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

    /// I ministri in carica ora, per il confronto dopo un passo.
    pub(super) fn in_carica(&self) -> Vec<Option<Ministro>> {
        (self.ministri.dicasteri.iter())
            .map(|st| st.ministro.clone())
            .collect()
    }

    /// Il conto delle variazioni del turno corrente già nello stato.
    pub(super) fn lette(&self) -> Lette {
        let t = self.turno;
        Lette {
            ministri: (self.ministri.dicasteri.iter())
                .map(|st| st.variazioni.iter().filter(|v| v.turno == t).count())
                .collect(),
            intesa: (self.intesa.variazioni.iter())
                .filter(|v| v.turno == t)
                .count(),
        }
    }

    /// Fatti 6, 21, 23, 24, 31, 32, 35, 46, 84-86, 107, 108, 126: una voce per ogni variazione di
    /// soddisfazione o d'intesa del turno oltre le `lette`, che poi le contano tutte. Solo i
    /// ministri in carica: in interim la Vista non mostra le variazioni. Proposta ed emendamento
    /// respinti per default (fatti 12 e 14) hanno già la loro voce, con la perdita nel fatto.
    pub(super) fn voci_variate(&mut self, lette: &mut Lette) {
        let t = self.turno;
        let mut nuove = Vec::new();
        let righe = (self.dati.dicasteri().iter())
            .zip(&self.ministri.dicasteri)
            .zip(&lette.ministri);
        for ((d, st), &gia) in righe {
            let Some(m) = &st.ministro else {
                continue;
            };
            let variazioni = (st.variazioni.iter()).filter(|v| v.turno == t).skip(gia);
            for v in variazioni.filter(|v| v.delta != 0.0 && v.causa != Causa::PropostaRespinta) {
                let fatto = format!(
                    "{} ({}) {} punti di soddisfazione",
                    m.nome,
                    d.nome,
                    perde_o_guadagna(v.delta)
                );
                let tipo = TipoVoce::SoddisfazioneVariata;
                nuove.push((tipo, None, Some(d.id.clone()), fatto, v.causa.to_string()));
            }
        }
        let variazioni = (self.intesa.variazioni.iter())
            .filter(|v| v.turno == t)
            .skip(lette.intesa);
        for v in
            variazioni.filter(|v| v.delta != 0.0 && v.causa != CausaIntesa::EmendamentoRespinto)
        {
            let nome = self.partiti.get(v.partito).map_or("", |p| p.nome.as_str());
            let fatto = format!("{nome} {} punti d'intesa", perde_o_guadagna(v.delta));
            let tipo = TipoVoce::IntesaVariata;
            nuove.push((tipo, Some(v.partito), None, fatto, v.causa.to_string()));
        }
        *lette = self.lette();
        for (tipo, j, dicastero, fatto, causa) in nuove {
            let v = match j {
                Some(j) => self.voce_di(j, tipo, fatto, causa),
                None => self.voce(tipo, fatto, causa),
            };
            if let Some(v) = v {
                v.dicastero = dicastero;
            }
        }
    }

    /// Fatti 9, 47, 87, 93, 94: il ministro `m` lascia il dicastero `i` in interim; `causa` è la
    /// sua `causa_dimissioni`. Al governo è notevole se le dimissioni le ha chieste il giocatore
    /// o per il rimpasto.
    pub(super) fn voce_ministro_esce(&mut self, i: usize, m: &Ministro, causa: Option<Causa>) {
        let Some(d) = self.dati.dicasteri().get(i) else {
            return;
        };
        let nome_di = |j: usize| self.partiti.get(j).map_or("", |p| p.nome.as_str());
        let testo = match causa {
            Some(Causa::DimissioniChieste) => {
                "le dimissioni chieste dal Presidente del Consiglio".to_string()
            }
            Some(Causa::Sfiduciato) => "la sfiducia individuale approvata dalle Camere".to_string(),
            Some(Causa::UscitaDalGoverno) => {
                format!("l'uscita dal governo di {}", nome_di(m.partito))
            }
            Some(Causa::Rimpasto) => {
                let a = self.governo.dicasteri.get(i).map_or("", |&j| nome_di(j));
                format!("il rimpasto, che passa il dicastero a {a}")
            }
            peggiore => sotto_la_soglia(
                "la soddisfazione",
                "di dimissioni",
                self.dati.manopola("soglia_dimissioni"),
                peggiore.map(|c| c.to_string()),
            ),
        };
        let fatto = format!(
            "{} lascia il dicastero {}, che va in interim",
            m.nome, d.nome
        );
        let id = d.id.clone();
        let chieste = matches!(causa, Some(Causa::DimissioniChieste | Causa::Rimpasto));
        let notevole = chieste && self.al_governo();
        if let Some(v) = self.voce(TipoVoce::MinistroEsce, fatto, testo) {
            if notevole {
                v.con_grado(Grado::N);
            }
            v.dicastero = Some(id);
        }
    }

    /// Le uscite del passo delle soglie: i ministri `prima` in carica che ora non ci sono più.
    pub(super) fn voci_dimessi(&mut self, prima: &[Option<Ministro>]) {
        for (i, m) in prima.iter().enumerate() {
            let st = self.ministri.dicasteri.get(i);
            if let (Some(m), Some(st)) = (m, st)
                && st.ministro.is_none()
            {
                let causa = st.causa_dimissioni;
                self.voce_ministro_esce(i, m, causa);
            }
        }
    }

    /// Fatto 88: i ministri `prima` (in testa alla chiusura) sopra la soglia di minaccia, ancora in
    /// carica e ora sotto. Lo stesso ministro ha lo stesso nome ed è passato dalle soglie, che gli
    /// contano un turno in carica: chi entra (governo nuovo, successore) riparte da zero, e un
    /// omonimo non eredita la minaccia del vecchio.
    pub(super) fn voci_minaccia(&mut self, prima: &[Option<Ministro>]) {
        let soglia = self.dati.manopola("soglia_minaccia");
        let mut nuove = Vec::new();
        let righe = (self.dati.dicasteri().iter())
            .zip(&self.ministri.dicasteri)
            .zip(prima);
        for ((d, st), a) in righe {
            let (Some(a), Some(b)) = (a, &st.ministro) else {
                continue;
            };
            let stesso = a.nome == b.nome && b.turni_in_carica > a.turni_in_carica;
            if stesso && a.soddisfazione >= soglia && b.soddisfazione < soglia {
                let peggiore = causa_peggiore(st.variazioni.iter().map(|v| (v.causa, v.delta)));
                let fatto = format!("{} ({}) minaccia le dimissioni", b.nome, d.nome);
                let causa = sotto_la_soglia(
                    "la soddisfazione",
                    "di minaccia",
                    soglia,
                    peggiore.map(|c| c.to_string()),
                );
                nuove.push((d.id.clone(), fatto, causa));
            }
        }
        for (id, fatto, causa) in nuove {
            if let Some(v) = self.voce(TipoVoce::MinacciaDiDimissioni, fatto, causa) {
                v.dicastero = Some(id);
            }
        }
    }

    /// Fatti 89, 90, 111: gli ultimatum posti contro quelli `prima`; con `ritiri`, anche quelli
    /// ritirati da chi resta alleato, perché l'intesa è tornata alla soglia. Mai l'intesa in cifre.
    pub(super) fn voci_ultimatum(&mut self, prima: &[Option<CausaIntesa>], ritiri: bool) {
        let soglia = self.dati.manopola("soglia_ultimatum");
        let alleati = self.alleati();
        let cambi: Vec<(usize, TipoVoce, &str, String)> = (self.intesa.ultimatum.iter())
            .zip(prima)
            .enumerate()
            .filter_map(|(j, (ora, prima))| match (prima, ora) {
                (None, Some(c)) => Some((
                    j,
                    TipoVoce::UltimatumPosto,
                    "pone",
                    sotto_la_soglia("l'intesa", "d'ultimatum", soglia, Some(c.to_string())),
                )),
                (Some(_), None) if ritiri && alleati.contains(&j) => Some((
                    j,
                    TipoVoce::UltimatumRitirato,
                    "ritira",
                    format!(
                        "l'intesa è tornata alla soglia d'ultimatum di {} o sopra",
                        virgola(soglia)
                    ),
                )),
                _ => None,
            })
            .collect();
        for (j, tipo, verbo, causa) in cambi {
            let nome = self.partiti.get(j).map_or("", |p| p.nome.as_str());
            let fatto = format!("{nome} {verbo} l'ultimatum al governo");
            self.voce_di(j, tipo, fatto, causa);
        }
    }

    /// Le fermate del turno appena aperto, nell'ordine della spec: `Fine` e `Governo` da sole.
    /// Lettura pura: solo ciò che la Vista mostra, il voto stimato sul sondaggio pubblico.
    pub(super) fn fermate(&self) -> Vec<Fermata> {
        if self.finita() {
            return vec![Fermata::Fine];
        }
        if self.governa() {
            return vec![Fermata::Governo];
        }
        let (l, pl) = (&self.legislatura, &self.parlamento);
        let mut fermate = Vec::new();
        if self.turno_del_voto() <= self.turno {
            fermate.push(Fermata::Campagna);
        }
        if l.fase == Fase::Trattativa
            && matches!(
                l.incarico,
                Some(Incarico::Giocatore | Incarico::Guida { .. })
            )
        {
            fermate.push(Fermata::Incarico);
        }
        if self.posto_riservato_mio() {
            fermate.push(Fermata::PostoRiservato);
        }
        if self.esito.iter().any(|v| v.tipo == TipoVoce::CrisiAperta) {
            fermate.push(Fermata::Crisi);
        }
        // `UltimatumPosto` non è fondibile: ogni voce porta il suo `partito`.
        fermate.extend(
            (self.esito.iter())
                .filter(|v| v.tipo == TipoVoce::UltimatumPosto)
                .filter_map(|v| v.partito.clone())
                .map(|partito| Fermata::Ultimatum { partito }),
        );
        let opposizione = self.all_opposizione().is_ok();
        let confermativo = opposizione
            && (pl.confermativi.iter())
                .any(|r| (r.misura).is_some_and(|id| self.esito_del_confermativo(id).is_some()));
        // Luglio-settembre è l'ultimo trimestre in cui l'abrogativo si chiede per l'anno.
        let abrogativo = opposizione
            && self.data.trimestre == 3
            && pl.referendum.is_none()
            && !self.governo.al_governo(self.giocatore)
            && self.finestra_referendum().is_ok();
        if confermativo || abrogativo {
            fermate.push(Fermata::Referendum);
        }
        let soglia = self.dati.manopola("fermata_quota_elettorato");
        fermate.extend(
            (pl.misure.iter())
                .filter(|m| m.voto == self.turno)
                .filter(|m| {
                    let q = self.quota_toccata(m);
                    q > 0.0 && q >= soglia
                })
                .map(|m| Fermata::Voto { misura: m.id }),
        );
        fermate
    }

    /// Quota degli elettori stimati del giocatore toccati dalla misura, 0-1, dal sondaggio
    /// pubblico: la più alta fra le sue disposizioni. Per disposizione, la quota nelle condizioni
    /// dei suoi `gruppi` per quella nelle regioni del `perimetro` pesate (1 se nazionale): il
    /// sondaggio dà solo i margini. 0 senza elettori stimati.
    pub fn quota_toccata(&self, m: &Misura) -> f64 {
        let (s, g) = (&self.informazione.sondaggio, self.giocatore);
        (m.valori.iter())
            .filter_map(|&(d, _)| self.dati.disposizioni().get(d))
            .map(|x| {
                let condizioni = quota_pesata(&s.condizioni, g, |id| {
                    if x.gruppi.iter().any(|c| c.to_string() == id) {
                        1.0
                    } else {
                        0.0
                    }
                });
                let regioni = if x.perimetro.is_empty() {
                    1.0
                } else {
                    quota_pesata(&s.regioni, g, |id| {
                        x.perimetro.get(id).copied().unwrap_or(0.0)
                    })
                };
                condizioni * regioni
            })
            .fold(0.0, f64::max)
    }
}

/// Gli elettori stimati del partito `g` negli `incroci` (campione × quota), pesati per `peso`
/// dell'id, sul totale; 0 se il totale è zero.
fn quota_pesata(incroci: &[Incrocio], g: usize, peso: impl Fn(&str) -> f64) -> f64 {
    let elettori = |i: &Incrocio| f64::from(i.campione) * i.quote.get(g).copied().unwrap_or(0.0);
    let tutti: f64 = incroci.iter().map(elettori).sum();
    if tutti > 0.0 {
        incroci
            .iter()
            .map(|i| peso(&i.id) * elettori(i))
            .sum::<f64>()
            / tutti
    } else {
        0.0
    }
}
