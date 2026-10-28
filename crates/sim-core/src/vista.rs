use crate::disposizioni::{Norma, TipoDisposizione};
use crate::economia::Aliquote;
use crate::elettorato::Condizione;
use crate::grandezze::Verso;
use crate::informazione::{Revisione, Sondaggio, Stima};
use crate::legislatura::{Eredita, EsitoElezioni, Fase, VoceDisponibilita, VoceRichiesta};
use crate::ministri::{Causa, StatoAgenda, Variazione};
use crate::parlamento::{
    CausaIntesa, EsitoReferendum, EsitoVoto, FormaMisura, StatoEmendamento, TipoReferendum,
    ValoreProposto, VariazioneIntesa, VoceLinea,
};
use crate::partita::Trimestre;
use serde::Serialize;
#[cfg(test)]
use ts_rs::TS;

/// Ciò che la UI può leggere della partita.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Vista {
    pub turno: u32,
    pub data: Trimestre,
    /// L'id del partito del giocatore.
    pub giocatore: String,
    /// La fase della legislatura: Camere sciolte, trattativa dopo il voto, governo in carica.
    pub fase: Fase,
    /// Il trimestre del prossimo voto: si vota nella chiusura di quel turno.
    pub voto: Trimestre,
    /// Turni che mancano al voto; 0 nel turno del voto.
    pub turni_al_voto: u32,
    /// Il giocatore guida il governo: i comandi di governo sono ammessi.
    pub governa: bool,
    /// La partita è finita.
    pub finita: bool,
    pub dicasteri: Vec<VoceDicastero>,
    /// Valori alla chiusura precedente: le grandezze arrivano con un turno di ritardo.
    pub grandezze: Vec<VoceGrandezza>,
    /// Spread BTP-Bund in punti base, arrotondato.
    pub spread_pb: i32,
    pub partiti: Vec<VocePartito>,
    /// Una voce per dicastero, nell'ordine dei dicasteri. Senza `governa` nessuna agenda: chi
    /// non governa non la vede prima dell'esecuzione (storia 90).
    pub ministeri: Vec<VoceMinistero>,
    pub sondaggio: Sondaggio,
    /// Seggi sul sondaggio pubblico; `None` solo se il riparto rifiuta i voti.
    pub proiezione: Option<Vec<SeggiPartito>>,
    pub statistica: VistaStatistica,
    pub bilancio: VistaBilancio,
    /// Nell'ordine dei dati.
    pub disposizioni: Vec<VoceDisposizione>,
    /// Il parlamento in carica.
    pub seggi: Vec<SeggiPartito>,
    pub alleati: Vec<VoceAlleato>,
    /// In fase ordinaria sotto un governo IA, una voce per suo alleato: l'intesa in fasce, mai in
    /// cifre. Vuota altrove.
    pub alleati_avversari: Vec<VoceFascia>,
    /// Il posto riservato all'opposizione di questo turno è del giocatore, e libero: nessuna
    /// proposta dell'opposizione è già in calendario.
    pub posto_riservato: bool,
    pub calendario: Vec<VoceMisura>,
    pub esiti: Vec<EsitoVoto>,
    /// I referendum in attesa del voto.
    pub referendum: Vec<VistaReferendum>,
    pub esiti_referendum: Vec<EsitoReferendum>,
    /// A partita finita, ciò che il giocatore lascia.
    pub eredita: Option<Eredita>,
    /// Nel turno del voto, una voce per partito IA: se è disponibile e perché. In trattativa con
    /// l'incarico al giocatore, o in crisi col giocatore al governo, una voce per partito fuori
    /// dalla sua coalizione: se entrerebbe coi seggi veri. Vuota altrove.
    pub disponibilita: Vec<VoceDisponibilita>,
    /// Nel turno del voto i membri della coalizione del giocatore con cui si vota (id), senza di
    /// lui: dichiarati o, senza dichiarazione, quelli dell'ultimo voto; vuota altrove.
    pub coalizione: Vec<String>,
    /// In trattativa con l'incarico al giocatore, una voce per alleato: la sua richiesta e
    /// l'offerta della spartizione proposta o, senza proposta, di quella di default; vuota
    /// altrove.
    pub richieste: Vec<VoceRichiesta>,
    /// Gli esiti dei voti, il più recente per primo.
    pub elezioni: Vec<EsitoElezioni>,
    /// In trattativa a chi va l'incarico; `None` altrove.
    pub incarico: Option<VoceIncarico>,
    /// Il governo cade alla chiusura del turno.
    pub dimissioni: bool,
    /// Turno di crisi: solo ordinaria amministrazione; `Allarga` ammesso col giocatore al governo,
    /// fuori dal turno del voto (che chiude la crisi).
    pub crisi: bool,
}

/// A chi va l'incarico in trattativa: al giocatore o al Capo dello Stato. `Guida`: dopo una
/// caduta, il giocatore sta nella maggioranza del Capo dello Stato ma non è il primo per seggi.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum VoceIncarico {
    Giocatore,
    CapoDelloStato,
    Guida,
}

/// Un referendum in attesa del voto; `promotore` è l'id di un partito; `firme` = indetto (`false`
/// = abrogativo con le firme in raccolta; il confermativo sempre `true`).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaReferendum {
    pub tipo: TipoReferendum,
    pub nome: String,
    pub voto: Trimestre,
    pub promotore: Option<String>,
    pub firme: bool,
    pub invito_astensione: bool,
}

/// Seggi attesi in più (o in meno) di un partito con la legge elettorale; `partito` è un id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VariazioneSeggi {
    pub partito: String,
    pub camera: i32,
    pub senato: i32,
}

/// L'intesa di un alleato in cifre: il giocatore governa con lui.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAlleato {
    pub partito: String,
    pub intesa: f64,
    pub ultimatum: bool,
    /// La causa con cui l'ultimatum è stato posto; resta finché dura.
    pub causa_ultimatum: Option<CausaIntesa>,
    pub causa_principale: Option<CausaIntesa>,
    /// Variazioni degli ultimi due turni.
    pub variazioni: Vec<VariazioneIntesa>,
}

/// L'intesa di un alleato del governo IA vista dall'opposizione (ticket 12, riga 19): la causa
/// solo in ultimatum.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceFascia {
    pub partito: String,
    pub fascia: FasciaIntesa,
    pub causa_ultimatum: Option<CausaIntesa>,
}

/// `Ultimatum` con l'ultimatum posto, `Incrinata` sotto `soglia_incrinata`, `Solida` altrimenti.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum FasciaIntesa {
    Solida,
    Incrinata,
    Ultimatum,
}

/// La sfiducia prima della presentazione: le firme bastano, la linea di ogni partito e l'esito
/// sullo stato attuale, che il voto in chiusura ripete se nulla cambia prima.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct AnteprimaSfiducia {
    pub firme: bool,
    pub linee: Vec<VoceLinea>,
    pub passa: bool,
}

/// Una misura in calendario; `proponente` è l'id del dicastero, `promotore` quello del partito
/// d'opposizione che l'ha portata nel posto riservato; `linea_giocatore` la linea scelta dal
/// giocatore all'opposizione, `None` = quella di default.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMisura {
    pub id: u32,
    pub nome: String,
    pub forma: FormaMisura,
    pub proponente: Option<String>,
    pub promotore: Option<String>,
    pub linea_giocatore: Option<bool>,
    pub valori: Vec<ValoreProposto>,
    pub voto: Trimestre,
    pub deliberazione: u8,
    pub fiducia: bool,
    pub linee: Vec<VoceLinea>,
    pub forbice: Forbice,
    pub emendamenti: Vec<VoceEmendamento>,
    /// Gli alleati che con la fiducia voterebbero no (id): sullo stato attuale, più il costo
    /// della fiducia se non è ancora posta.
    pub alleati_contro_fiducia: Vec<String>,
    /// Solo per le leggi elettorali: una voce per partito.
    pub seggi: Vec<VariazioneSeggi>,
}

/// Una legge prima della presentazione: costo annuo esatto in euro, effetti nominali a fasce
/// stimate dal ministro proponente, leve che cambiano, gruppi toccati; linee, forbice e seggi
/// come in calendario. `proponente` è l'id del dicastero.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Anteprima {
    pub nome: String,
    pub forma: FormaMisura,
    pub proponente: Option<String>,
    pub costo_annuo: i64,
    pub effetti: Vec<EffettoStimato>,
    pub leve: Vec<VoceLeva>,
    pub gruppi: Vec<Condizione>,
    pub linee: Vec<VoceLinea>,
    pub forbice: Forbice,
    pub seggi: Vec<VariazioneSeggi>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum Fascia3 {
    Poco,
    Medio,
    Molto,
}

/// `grandezza` è un id; `fascia` quella stimata dal ministro; `ritardo` quello scritto, in turni.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct EffettoStimato {
    pub grandezza: String,
    pub migliora: bool,
    pub fascia: Fascia3,
    pub ritardo: u32,
}

/// Una leva che cambia: `da` in vigore, `a` con la legge.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceLeva {
    pub leva: String,
    pub da: f64,
    pub a: f64,
}

/// L'emendamento di un alleato; `partito` è un id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceEmendamento {
    pub partito: String,
    pub testo: String,
    pub motivo: String,
    pub stato: StatoEmendamento,
}

/// Sì attesi ± 2σ per ramo, prima del voto; `maggioranza_*` = sì necessari; `in_bilico` se una
/// forbice scende sotto la sua maggioranza.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct Forbice {
    pub camera: [u32; 2],
    pub senato: [u32; 2],
    pub maggioranza_camera: u32,
    pub maggioranza_senato: u32,
    pub in_bilico: bool,
}

/// Una disposizione com'è in vigore; gradini, dicastero e perimetro sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDisposizione {
    pub id: String,
    pub nome: String,
    pub dicastero: Option<String>,
    pub norma: Norma,
    pub tipo: TipoDisposizione,
    pub in_vigore: String,
    pub legge: String,
    pub precedente: String,
    pub in_arrivo: Vec<VoceArrivo>,
    pub referendabile: bool,
    /// Le regioni; vuoto = tutto il Paese.
    pub perimetro: Vec<String>,
    pub gradini: Vec<VoceGradino>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceArrivo {
    pub gradino: String,
    pub data: Trimestre,
}

/// `bloccato` = abrogato da un referendum.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceGradino {
    pub id: String,
    pub nome: String,
    pub posizione: Option<f64>,
    pub bloccato: bool,
}

/// Il bilancio in vigore nel trimestre: l'approvato se è dell'anno in corso, altrimenti il
/// corrente.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaBilancio {
    /// Anno del bilancio in vigore.
    pub anno: u16,
    pub dicasteri: Vec<VoceBudget>,
    pub aliquote: Aliquote,
    /// %, come `VoceBudget.perdita_reale`: una per scaglione con soglia, in ordine.
    pub perdita_reale_scaglioni: Vec<f64>,
    /// Euro annui.
    pub fondo_riserva: i64,
    /// C'è una bozza: si può presentare la legge di bilancio.
    pub presentabile: bool,
    /// Una legge di bilancio è già presentata nel turno: l'aula la vota in chiusura.
    pub presentato: bool,
    /// Gennaio-marzo senza bilancio approvato per l'anno: niente agende né proposte.
    pub esercizio_provvisorio: bool,
}

/// Euro annui; `perdita_reale` in %, il potere d'acquisto perso dalla cifra nominale.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceBudget {
    pub id: String,
    pub spesa: i64,
    pub salari: i64,
    pub acquisti: i64,
    pub margine: i64,
    pub perdita_reale: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct SeggiPartito {
    pub partito: String,
    pub camera: u32,
    pub senato: u32,
}

/// Valence esatta: è un ingresso dello scenario, non stato nascosto.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePartito {
    pub id: String,
    pub nome: String,
    pub posizione_dichiarata: f64,
    pub posizione_percepita: f64,
    pub valence: f64,
    pub al_governo: bool,
}

/// La statistica senza `vera`.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaStatistica {
    pub trimestre: Trimestre,
    pub prima_stima: Stima,
    pub revisione: Option<Revisione>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceDicastero {
    pub id: String,
    pub nome: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceGrandezza {
    pub id: String,
    pub nome: String,
    pub verso: Verso,
    /// Una voce, o una per regione.
    pub valori: Vec<f64>,
}

/// Il ministro (o l'interim con la sua rosa) di un dicastero.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMinistero {
    pub dicastero: String,
    /// `None` = interim.
    pub ministro: Option<VoceMinistro>,
    pub rosa: Vec<VoceMinistro>,
    /// Il ministro uscirà a fine turno: nessun comando sul dicastero fino al successore.
    pub dimissioni_chieste: bool,
    /// Indice nella rosa del successore scelto; `None` = il primo.
    pub scelta: Option<u8>,
    /// Perché il ministro è uscito; `Some` solo in interim.
    pub causa_dimissioni: Option<Causa>,
    /// L'atto annunciato per il turno.
    pub agenda: Option<VoceAgenda>,
    /// Perché manca l'agenda: `esercizio provvisorio`, `interim` o `margine esaurito`.
    pub motivo_senza_agenda: Option<String>,
    /// I 4 atti del dicastero.
    pub atti: Vec<VoceAtto>,
    pub cantieri: Vec<VoceCantiere>,
    /// Euro, prima delle rate dei cantieri.
    pub margine_residuo: i64,
    /// La proposta (di fondi o di riforma) in attesa di Accogli, Rinvia o Respingi.
    pub proposta: Option<VoceProposta>,
}

/// Proposta di fondi: `atto` è un id, `importo` gli euro che mancano al margine. Proposta di
/// riforma: `disposizione` e `gradino` sono id, nessun `atto` e `importo` 0.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceProposta {
    pub atto: Option<String>,
    pub nome: String,
    pub importo: i64,
    pub motivo: String,
    pub rinviata: bool,
    pub disposizione: Option<String>,
    pub gradino: Option<String>,
}

/// `atto` e `regione` sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAgenda {
    pub atto: String,
    pub nome: String,
    pub regione: Option<String>,
    /// Euro per turno di lavori.
    pub costo: i64,
    pub motivo: String,
    pub stato: StatoAgenda,
    pub in_evidenza: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceAtto {
    pub id: String,
    pub nome: String,
    pub motto: String,
    /// Euro per turno di lavori.
    pub costo: i64,
    pub turni: u32,
    pub pagabile: bool,
    /// `serve {disposizione}: {gradino} o {gradino}`; l'atto non è pagabile.
    pub prerequisito_mancante: Option<String>,
    pub effetti: Vec<VoceEffetto>,
}

/// `grandezza` è un id; `ritardo` quello scritto, in turni.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceEffetto {
    pub grandezza: String,
    pub migliora: bool,
    pub ritardo: u32,
}

/// `atto` e `regione` sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceCantiere {
    pub atto: String,
    pub regione: Option<String>,
    pub turni_mancanti: u32,
    pub rata: i64,
    pub speso: i64,
}

/// Il ministro com'è visibile al giocatore: soddisfazione, agende e capacità, non lo stato nascosto.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMinistro {
    pub nome: String,
    /// Id del partito.
    pub partito: String,
    pub capacita: u8,
    pub posizione: f64,
    pub obiettivo: f64,
    pub soddisfazione: f64,
    pub rodaggio: bool,
    /// Un tecnico, senza partito: `partito` è quello del Presidente del Consiglio.
    pub tecnico: bool,
    pub minaccia: bool,
    pub causa_principale: Option<Causa>,
    /// Variazioni di soddisfazione degli ultimi due turni, con la loro causa.
    pub variazioni: Vec<Variazione>,
}
