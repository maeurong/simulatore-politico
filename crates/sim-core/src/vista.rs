use crate::disposizioni::{Norma, TipoDisposizione};
use crate::economia::{Aliquote, Bilancio};
use crate::elettorato::{Condizione, Genere};
use crate::esito::VoceEsito;
use crate::eventi::{Famiglia, Tono};
use crate::grandezze::Verso;
use crate::informazione::{Revisione, Sondaggio, Stima, TipoAcquisto, VistaCentroStudi, VistaTest};
use crate::legislatura::{Eredita, EsitoElezioni, Fase, Spesa, VoceDisponibilita, VoceRichiesta};
use crate::ministri::{Causa, StatoAgenda, Variazione};
use crate::parlamento::{
    CausaIntesa, EsitoReferendum, EsitoVoto, FormaMisura, StatoEmendamento, TipoReferendum,
    ValoreProposto, VariazioneIntesa, VoceLinea,
};
use crate::partita::Trimestre;
use crate::partiti::{Regola, Vocazione};
use crate::scenario::StatoPde;
use crate::struttura::Macroarea;
use serde::{Deserialize, Serialize};
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
    /// Il partito (id) a cui la rotazione dà il posto riservato del turno, in fase ordinaria;
    /// `None` altrove o senza partiti fuori dal governo con seggi alla Camera.
    pub posto_riservato_di: Option<String>,
    /// La mozione di sfiducia presentata nel turno, votata in chiusura; `None` = nessuna.
    pub sfiducia: Option<VoceSfiducia>,
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
    /// In trattativa con l'incarico al giocatore, una voce per dicastero nell'ordine dei dati: la
    /// spartizione proposta o, senza proposta, quella di default; vuota altrove.
    pub spartizione: Vec<VoceSpartizione>,
    /// In trattativa con `Guida`, la coalizione del ribaltone e il primo per seggi; `None` altrove.
    pub ribaltone: Option<VistaRibaltone>,
    /// Gli esiti dei voti, il più recente per primo.
    pub elezioni: Vec<EsitoElezioni>,
    /// In trattativa a chi va l'incarico; `None` altrove.
    pub incarico: Option<VoceIncarico>,
    /// Il governo cade alla chiusura del turno.
    pub dimissioni: bool,
    /// Turno di crisi: solo ordinaria amministrazione; `Allarga` ammesso col giocatore al governo,
    /// fuori dal turno del voto (che chiude la crisi).
    pub crisi: bool,
    /// L'esito dell'ultima chiusura: tutte le voci, in evidenza e no, nell'ordine della chiusura;
    /// vuoto al turno zero.
    pub esito: Vec<VoceEsito>,
    /// Il programma pubblico del governo IA, nell'ordine in cui lo porta in aula; vuoto col
    /// giocatore Presidente del Consiglio.
    pub programma: Vec<VoceProgramma>,
    /// La campagna del giocatore: spese scelte e se il comando è ammesso.
    pub campagna: VistaCampagna,
    /// Gli eventi aperti: si risolvono alla chiusura, con la `scelta` o il default.
    pub eventi: Vec<VoceEvento>,
    /// Le situazioni a soglia, accese o no.
    pub situazioni: Vec<VoceSituazione>,
    /// L'informazione comprata nel turno e i prezzi di quella comprabile.
    pub acquisti: VistaAcquisti,
    /// Perché «avanti fino a novità» non concatena un'altra chiusura da questo turno; vuota = può.
    pub fermate: Vec<Fermata>,
    /// Una voce per regione, nell'ordine dei dati.
    pub regioni: Vec<VoceRegione>,
    /// Le opere finite dall'ultimo voto: atti regionali eseguiti e cantieri finiti.
    pub opere: Vec<VoceOpera>,
    pub regole_ue: VistaRegoleUe,
    pub debito: VistaDebito,
    pub posti: VistaPosti,
    pub soglie: VistaSoglie,
}

/// Le soglie che le regole leggono dalle manopole: soddisfazione del ministro (`minaccia`,
/// `dimissioni`), intesa dell'alleato (`ultimatum`, `uscita`) e spread della crisi del debito, in
/// punti base.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaSoglie {
    pub minaccia: f64,
    pub dimissioni: f64,
    pub ultimatum: f64,
    pub uscita: f64,
    pub crisi_debito_pb: f64,
}

/// Un dicastero (id) nella spartizione in corso: `peso` è `peso_spartizione`, `partito` l'id di chi
/// lo prende (`None` = un tecnico), `di_default` quello della spartizione di default.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceSpartizione {
    pub dicastero: String,
    pub peso: u8,
    pub partito: Option<String>,
    pub di_default: String,
}

/// Il ribaltone: la coalizione (id) e il primo per seggi, che cede o no la guida.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaRibaltone {
    pub coalizione: Vec<String>,
    pub primo: Option<String>,
}

/// I posti del calendario e quanti ne sono occupati, contati come li conta `PresentaLegge`: i
/// posti ordinari per riforme ed elettorali, quello costituzionale per le norme costituzionali.
/// Le proposte dell'opposizione hanno il loro posto, fuori da questi.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaPosti {
    pub ordinari: u32,
    pub ordinari_occupati: u32,
    pub costituzionali: u32,
    pub costituzionali_occupati: u32,
}

/// Un gradino di una disposizione della legge elettorale: se la bozza con quel gradino al posto
/// del suo valore farebbe una legge elettorale applicabile, e altrimenti perché. `disposizione` e
/// `gradino` sono id.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMenu {
    pub disposizione: String,
    pub gradino: String,
    pub ammesso: bool,
    pub motivo: Option<String>,
}

/// Le regole fiscali UE. Tutto tranne `proiezione` è il verdetto su un anno chiuso e riveduto;
/// `conto_controllo` e `ultima_deviazione` sono in % del PIL, dopo l'ultimo controllo.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaRegoleUe {
    pub procedura: StatoPde,
    pub turni_in_stato: u32,
    pub conto_controllo: f64,
    pub ultima_deviazione: f64,
    pub coesione_sospesa: bool,
    /// L'ultimo anno per cui il giocatore ha chiesto il margine UE per l'energia, solo se viene
    /// dopo `clausola_nazionale_fino_a` dello scenario (prima la clausola nazionale copre già
    /// l'energia); `None` se mai chiesto o se non conta.
    pub margine_energia: Option<u16>,
    /// `None` se al conto manca un anno.
    pub proiezione: Option<ProiezioneUe>,
}

/// Ciò che rileverebbe il prossimo controllo: l'anno giudicato, la deviazione e il conto di
/// controllo che ne uscirebbe, in % del PIL. Con l'anno non ancora chiuso, spesa netta e PIL sono
/// annualizzati sui trimestri chiusi.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct ProiezioneUe {
    pub anno: u16,
    pub deviazione: f64,
    pub conto: f64,
}

/// `debito_pil`: il debito in % del PIL degli ultimi quattro trimestri, quello dello spread.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaDebito {
    pub debito_pil: f64,
    pub mes: Option<VistaMes>,
    pub ristrutturazione: Option<VistaRistrutturazione>,
}

/// Il programma del MES in corso: `fine` è il primo trimestre senza vincoli.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaMes {
    pub fine: Trimestre,
    pub vincoli: Vec<VoceVincolo>,
}

/// Una disposizione vincolata dal memorandum e il gradino a cui la tiene, coi nomi del catalogo.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceVincolo {
    pub disposizione: String,
    pub gradino: String,
}

/// La ristrutturazione del debito: `taglio` è il trimestre in cui il debito è stato tagliato,
/// `premio_pb` il premio di oggi sullo spread, `fine_perdita` il primo trimestre in cui i redditi
/// dei detentori non calano più, `None` se la perdita è già finita.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaRistrutturazione {
    pub taglio: Trimestre,
    pub premio_pb: i32,
    pub fine_perdita: Option<Trimestre>,
}

/// Un'opera finita in una regione. `dicastero` e `regione` sono id, `atto` il nome dell'atto,
/// `turno` quello della chiusura che l'ha finita, `euro` la spesa dell'atto o quanto il cantiere
/// ha speso in tutto.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceOpera {
    pub dicastero: String,
    pub atto: String,
    pub regione: String,
    pub turno: u32,
    pub cantiere: bool,
    pub euro: i64,
}

/// Una ragione per fermarsi sul turno appena aperto. `misura` è l'id della misura in calendario,
/// `partito` l'id del partito.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "tipo", rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum Fermata {
    /// Il giocatore governa: ogni turno chiede le sue scelte.
    Governo,
    /// La partita è finita.
    Fine,
    /// Una misura col voto in questo turno tocca almeno `fermata_quota_elettorato` degli elettori
    /// stimati del giocatore.
    Voto { misura: u32 },
    /// Nell'ultima chiusura un alleato del governo ha posto un ultimatum.
    Ultimatum { partito: String },
    /// Nell'ultima chiusura il governo è entrato in crisi.
    Crisi,
    /// C'è un confermativo da scegliere, o a luglio-settembre il giocatore può ancora chiedere un
    /// abrogativo.
    Referendum,
    /// È il turno del voto.
    Campagna,
    /// Trattativa con l'incarico, o la guida, al giocatore.
    Incarico,
    /// Il posto riservato del turno è del giocatore, ed è libero.
    PostoRiservato,
}

/// L'informazione comprata: un campo vuoto è un tipo non comprato in questo turno. Prezzi in
/// euro.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaAcquisti {
    pub sondaggio_privato: Option<Sondaggio>,
    pub test_misura: Option<VistaTest>,
    pub centro_studi: Option<VistaCentroStudi>,
    pub prezzi: Vec<PrezzoAcquisto>,
}

/// Euro.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct PrezzoAcquisto {
    pub acquisto: TipoAcquisto,
    pub euro: i64,
}

/// Una situazione: `misura` è l'id della grandezza o della misura del motore; `valore` quello
/// ufficiale (grandezze alla chiusura precedente, prima stima della crescita); `accensione` e
/// `spegnimento` le due soglie dell'isteresi; `trimestri` quanti trimestri di fila dalla parte del
/// cambio lo fanno scattare, `oltre` quanti ne sono già passati.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceSituazione {
    pub nome: String,
    pub misura: String,
    pub valore: f64,
    pub accensione: f64,
    pub spegnimento: f64,
    pub accesa: bool,
    pub trimestri: u8,
    pub oltre: u8,
}

/// Un evento aperto: `causa` è il testo con i valori ufficiali, `scelta` l'indice in `opzioni`
/// (`None` = vale il default), `regione` e `bersaglio` i nomi, `costo` in euro.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceEvento {
    pub nome: String,
    pub famiglia: Famiglia,
    pub causa: String,
    pub tono: Tono,
    pub regione: Option<String>,
    pub bersaglio: Option<String>,
    pub costo: i64,
    pub opzioni: Vec<VoceOpzione>,
    pub scelta: Option<usize>,
    /// Le disposizioni (id) che il decreto innescato da questo evento ammette, nell'ordine del
    /// catalogo: vuota se l'evento non ammette decreto.
    pub norme: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceOpzione {
    pub id: String,
    pub nome: String,
    pub default: bool,
}

/// La campagna del giocatore: `attiva` nel turno del voto, quando `Comando::Campagna` è ammesso;
/// `spese` quella scelta, che la chiusura paga. `riposizionato`: `Riposiziona` è già fatto in
/// questa campagna; `passo` è `passo_movimento`, il massimo spostamento della dichiarata;
/// `minaccia_scissione` e `soglia_scissione` le distanze fra percepita e dichiarata a cui il
/// partito minaccia la scissione e si scinde.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaCampagna {
    pub attiva: bool,
    pub spese: Vec<Spesa>,
    pub riposizionato: bool,
    pub passo: f64,
    pub minaccia_scissione: f64,
    pub soglia_scissione: f64,
}

/// L'anteprima di `Riposiziona`: la dichiarata nuova, la percepita che lo spostamento darebbe e
/// la loro distanza, quella che le soglie di scissione misurano.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct AnteprimaRiposizionamento {
    pub dichiarata: f64,
    pub percepita: f64,
    pub distanza: f64,
}

/// L'intesa che un alleato perderebbe con `Prometti`: `partito` (id), `intesa` la somma delle
/// variazioni, negativa, e `disposizioni` gli id delle promesse su cui è contrario.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceCostoPromessa {
    pub partito: String,
    pub intesa: f64,
    pub disposizioni: Vec<String>,
}

/// Una voce del programma del governo IA: disposizione e gradino, con id e nomi dai dati.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceProgramma {
    pub disposizione: String,
    pub nome: String,
    pub gradino: String,
    pub nome_gradino: String,
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
/// = abrogativo con le firme in raccolta; il confermativo sempre `true`). `richiesto` = si vota
/// (il confermativo non richiesto entra in vigore senza voto); `scelta` = il giocatore
/// all'opposizione può ancora chiedere il confermativo o rinunciarvi (`Comando::Confermativo`).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VistaReferendum {
    pub tipo: TipoReferendum,
    pub nome: String,
    pub voto: Trimestre,
    pub promotore: Option<String>,
    pub firme: bool,
    pub invito_astensione: bool,
    pub richiesto: bool,
    pub scelta: bool,
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
/// giocatore all'opposizione, o al governo sulle proposte dell'opposizione; `None` = quella di
/// default.
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
    /// Come in `Anteprima`, stimati dal ministro che oggi guida il dicastero proponente (neutra
    /// senza ministro e per una proposta d'opposizione), con la chiave del turno di presentazione:
    /// il rumore della stima non si ricampiona, ma costo, leve ed effetti partono dal gradino in
    /// vigore, e il costo dal PIL potenziale dell'anno, quindi cambiano quando cambiano quelli.
    pub costo_annuo: i64,
    pub effetti: Vec<EffettoStimato>,
    pub leve: Vec<VoceLeva>,
    pub gruppi: Vec<Condizione>,
    /// Dove il giocatore sceglie la linea (la guardia di `Linea`), ciò che ogni linea costa al
    /// suo partito; `None` altrove.
    pub costo_linea: Option<CostiLinea>,
}

/// La sfiducia del turno: `dicastero` è l'id (`None` = al governo), `partito` l'id di chi l'ha
/// presentata.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceSfiducia {
    pub dicastero: Option<String>,
    pub partito: String,
}

/// Una linea del giocatore su una misura: la sua percepita dopo il voto, calcolata su quella di
/// oggi, e i franchi tiratori attesi del suo partito in ogni ramo (probabilità per seggi).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct CostoLinea {
    pub percepita: f64,
    pub franchi_camera: f64,
    pub franchi_senato: f64,
}

/// Il costo della linea sì, no e di default.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct CostiLinea {
    pub si: CostoLinea,
    pub no: CostoLinea,
    pub default: CostoLinea,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
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
    /// Gli id dei dicasteri indicizzati dalla proposta, in ordine; vuoto sugli emendamenti delle
    /// leggi.
    pub dicasteri: Vec<String>,
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
    /// Il rifiuto che `PromuoviReferendum` del giocatore darebbe ora; `None` = ammesso.
    pub rifiuto_referendum: Option<String>,
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
    /// Euro annui: il fondo di riserva non ancora speso, mai sotto zero.
    pub fondo_residuo: i64,
    /// C'è una bozza: si può presentare la legge di bilancio.
    pub presentabile: bool,
    /// Una legge di bilancio è già presentata nel turno: l'aula la vota in chiusura.
    pub presentato: bool,
    /// Il bilancio presentato com'è in aula, emendamenti accolti compresi; `None` senza legge
    /// presentata.
    pub in_aula: Option<Bilancio>,
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
    pub vocazione: Vocazione,
    /// Vuoto per il partito del giocatore.
    pub leader: String,
    /// `None` per il partito del giocatore, che non ha leader.
    pub genere_leader: Option<Genere>,
    pub regola: Regola,
    pub sciolto: bool,
    /// Euro: pubblica.
    pub cassa: i64,
    /// Pubbliche: prima quelle in sospeso dal voto prima, poi quelle della campagna in corso.
    pub promesse: Vec<VocePromessa>,
    /// Ha minacciato la scissione e la distanza fra le posizioni non è ancora tornata sotto la
    /// soglia di minaccia.
    pub minaccia: bool,
}

/// Una promessa di campagna coi nomi del catalogo.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VocePromessa {
    pub disposizione: String,
    pub gradino: String,
    /// Fatta al voto prima, si verifica al prossimo.
    pub in_sospeso: bool,
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
    /// L'indicatore di servizio: l'id di una voce di `Vista.grandezze`.
    pub indicatore: String,
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

/// Una regione, nell'ordine dei dati (lo stesso di `VoceGrandezza.valori`). `id` è la chiave.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceRegione {
    pub id: String,
    pub nome: String,
    pub macroarea: Macroarea,
    pub pericolosita: IndiciPericolosita,
    /// %, esatta, della chiusura precedente: in media, pesata sulle forze di lavoro, è la
    /// disoccupazione riveduta della `statistica`.
    pub disoccupazione: f64,
    /// Euro annui reali per elettore, sui redditi della chiusura precedente pesati con le quote
    /// di oggi.
    pub reddito: f64,
    /// Le 18 quote dei gruppi, che sommano a 1, nell'ordine condizione × fascia.
    pub gruppi: Vec<f64>,
}

/// Da 0 a 1: emergenze attese l'anno nella regione diviso quelle della regione più esposta.
/// `totale` fa lo stesso sulla somma dei tre tipi; la siccità non entra.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct IndiciPericolosita {
    pub sisma: f64,
    pub idro: f64,
    pub incendi: f64,
    pub totale: f64,
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
    /// L'intesa che l'alleato perderebbe con `ChiediDimissioni`; 0 col ministro del giocatore,
    /// col tecnico e in interim.
    pub prezzo_dimissioni: f64,
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
    /// La variazione di soddisfazione del ministro con `Ferma`, prima del limite a 0-100:
    /// negativa se tiene all'atto.
    pub costo_ferma: f64,
    /// Perché l'atto è in evidenza, nell'ordine della regola; vuota = non lo è.
    pub evidenza: Vec<MotivoEvidenza>,
    /// `!evidenza.is_empty()`.
    pub in_evidenza: bool,
}

/// Una ragione dell'evidenza di un atto: il ministro in minaccia, l'atto a `distanza_evidenza` o
/// più dalla posizione del Presidente del Consiglio, un evento aperto nel dicastero.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(test, derive(TS), ts(export))]
pub enum MotivoEvidenza {
    Minaccia,
    Distanza,
    Evento,
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
    /// La variazione di soddisfazione del ministro in carica con `FermaCantiere`, prima del
    /// limite a 0-100: negativa se tiene all'atto; 0 in interim.
    pub costo_ferma: f64,
}

/// Il ministro com'è visibile al giocatore: soddisfazione, agende e capacità, non lo stato nascosto.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct VoceMinistro {
    pub nome: String,
    pub genere: Genere,
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
