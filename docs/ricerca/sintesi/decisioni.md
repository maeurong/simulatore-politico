---
tipo: sintesi
fonti: []
aggiornato: 2026-10-18
---
# Decisioni di Mario

Le scelte di prodotto prese da Mario, con la data. Ogni pagina della wiki che le contraddice va corretta.

## 2026-10-16: gioco completamente gratuito
Mario: «assolutamente gratuito». Il gioco non si vende, nemmeno in parte.

Conseguenze sui dati:
- Le licenze non commerciali (NC) si possono usare per distribuire il gioco, senza contraddizioni: ESS ([[ess]]), OMS ([[who-gho]]).
- Restano escluse le fonti che vietano la ridistribuzione indipendentemente dal prezzo: Manifesto ([[manifesto-project]]), UNODC ([[unodc]]), IMF WEO ([[imf-weo]]), Comtrade grezzo ([[un-comtrade]]), ACLED ([[acled]]), IISS ([[iiss-military-balance]]).
  - In tensione con [[imf-weo]]: i termini IMF sui dati statistici permettono di distribuire con attribuzione e chiedono il permesso solo per il riuso commerciale, quindi il WEO starebbe con le fonti NC; richiamano però anche i termini generali IMF (uso personale e non commerciale). La fonte non scioglie il punto.
- Un repo open source (MIT, GPL) permette a chiunque l'uso commerciale, e questo non è compatibile con dati NC messi sotto la stessa licenza. La questione è chiusa dalla decisione successiva.

## 2026-10-16: codice open source, dati in una cartella separata
Mario ha approvato. Il codice ha una licenza open source, ancora da scegliere. I dati stanno in una cartella separata e ogni dataset tiene la propria licenza. Questo lascia entrare i dati NC (ESS, OMS) e quelli CC BY-SA (V-Dem) senza estendere le loro condizioni al codice. Restano esclusi i dati che vietano la ridistribuzione.

## 2026-10-16: paese reale, personaggi inventati, turno di 3 mesi
Mario, nel brainstorming: «sono il presidente del consiglio di un paese reale, ma tutti i personaggi sono inventati». Il turno «dura 3 mesi».

Conseguenze:
- Scioglie in parte la tensione di [[soggetti-reali-vs-fittizi]]: il paese è reale, quindi si usano i suoi dati macro e istituzionali aperti; nessun volto, nome o simbolo di persona reale. Restano aperti i partiti: reali o fittizi.
- Scioglie la tensione sul passo del tempo in [[tempo-e-ritmo]]: il turno è trimestrale, lo stesso passo dei modelli in [[macro-stock-flow-consistent]] e [[macro-neokeynesiano-piccolo]].
- Il giocatore è il capo del governo. I ministri sono personaggi con descrizione, capacità, principi e obiettivi. Il giocatore li sceglie e assegna il budget per settore; i ministri attuano le politiche in autonomia e propongono misure nel proprio settore e in quelli vicini.
- Il raggio è quello di uno stato intero: ricerca, specializzazione produttiva, costruzioni, diplomazia (accordi, alleanze, unioni, federazioni), commercio (embarghi), forza militare (invasioni, blocchi navali), intelligence, finanziamento di gruppi e stati esteri, imprevisti da prevedere, rapporto debito/PIL.

## 2026-10-16: solo Italia, parlamento ed elezioni, partiti inventati
Risposte di Mario nel brainstorming:
- **Paese giocabile: solo l'Italia.** Gli altri paesi sono attori più semplici: con loro si fanno lo stesso diplomazia, commercio e guerra.
- **Potere: parlamento ed elezioni.** Si perde con la sconfitta elettorale o con la sfiducia. In più: «posso anche decidere di cambiare forma di governo con forme più autoritarie, posso anche essere vittima di attentati da gruppi in particolare a cui ho fatto dei torti». L'attentato va legato a cause dentro il modello, non al caso: è la critica a Democracy 4 in [[democracy-4]] e il criterio di [[crisi-ed-eventi]]. Il colpo di stato e la svolta autoritaria hanno formule esplicite in [[personaggi-fazioni-e-stabilita-interna]] e [[controinsurrezione-e-stabilita]].
- **Partiti: tutti inventati**, ridotti a sette posizioni: estrema sinistra, sinistra, centrosinistra, centro, centrodestra, destra, estrema destra. Chiude la tensione fra [[soggetti-reali-vs-fittizi]] e [[ches]]: i dati CHES servono solo a tarare le posizioni.
- **Durata della legislatura: si parte da 5 anni**, ma la durata è una politica che il governo può cambiare. La partita non ha una fine fissa.

## 2026-10-16: partenza, partito nuovo, ministri, mappa
- **Partenza:** l'Italia di oggi, coi dati reali più recenti, oppure un'Italia su misura con i parametri cambiati (più debito, più ricca, ecc.). Niente scenari storici.
- **Partito del giocatore:** «crei un partito oltre a quelli già definiti». È un ottavo partito che si aggiunge ai sette inventati.
- **Ministri:** quote per partito più tecnici. Ogni partito della coalizione chiede ministeri in proporzione al suo peso, e il giocatore può nominare alcuni tecnici. Scontentare un alleato porta alla crisi. Il modello è in [[formazione-di-coalizioni]].
- **Mappa:** per regioni, cioè 20 regioni con indicatori e grandi opere visibili. Niente province e niente city builder.

## 2026-10-16: prima la v1, dicasteri, quattro forme di misura
- **Obiettivo della pianificazione:** la spec di una v1 giocabile dall'inizio alla fine. Il resto della visione resta per dopo, ma la v1 non deve chiudergli la strada.
- **Il budget va ai dicasteri**, non ai settori produttivi. Glossario in `CONTEXT.md` alla radice del repo.
- **Quattro forme di misura:** atto del ministro (amministrativo, dentro il budget del dicastero, autonomo), decreto, legge, norma costituzionale. L'autonomia dei ministri vale solo per il primo livello; le altre forme le propongono al giocatore. Raccordo con [[leggi-decreti-ed-editti]], che ne proponeva tre.

## 2026-10-16: taglio della v1
- **Inizio:** il partito del giocatore è appena entrato in parlamento con un buon risultato; la partita parte dalla trattativa di coalizione. Niente fase da outsider.
- **Dentro la v1:** coalizione con quote e tecnici, 8-10 dicasteri accorpati con ministri autonomi e budget, economia aggregata (nell'euro: tassi della BCE fuori dal controllo del giocatore), elettorato per gruppi con sondaggi rumorosi, parlamento con voto probabilistico, crisi di governo e sfiducia, elezioni con la legge elettorale vigente, mappa delle 20 regioni, crisi ed eventi con cause leggibili, informazione imperfetta. Gli esteri entrano solo come vincoli (regole UE) e come shock esterni.
- **Fuori dalla v1:** guerra, intelligence, finanziamento di gruppi esteri, alleanze/unioni/federazioni, embarghi, svolta autoritaria, attentati, ricerca tecnologica, diplomazia attiva. La v1 deve lasciare loro i punti di aggancio.
- **Pubblico:** solo Mario. Niente store, marchio o traduzioni; solo italiano, ma con i testi esternalizzati.
- **Partenza:** l'Italia di oggi più un editor dei parametri iniziali sullo stesso scenario.

## 2026-10-16: dieci dicasteri, legge di bilancio, due quote di budget
Grilling sul ticket wayfinder «Dicasteri della v1» (`.scratch/v1-giocabile/issues/03-dicasteri-della-v1.md`, con la tabella completa delle grandezze).
- **Dieci dicasteri:** Economia e Finanze; Sviluppo economico (Imprese, Agricoltura, Turismo); Lavoro e Welfare; Sanità; Istruzione e Ricerca (con Università e Cultura); Infrastrutture e Trasporti; Ambiente ed Energia; Interno; Giustizia; Esteri e Difesa. Esteri e Difesa si divide quando arriva la guerra.
- **Legge di bilancio annuale:** aliquote e budget per dicastero si fissano una volta l'anno e passano dal parlamento; durante l'anno i fondi si spostano solo per decreto.
- **Due quote di budget per dicastero:** spesa di funzionamento, che tiene in piedi un indicatore di servizio, e margine discrezionale, che il ministro spende su 4 atti propri (40 nella v1). Risponde alla domanda aperta «sotto-gioco o slider» di [[raccomandazioni-convergenti]] e [[ministri-e-gabinetto]]: entrambi, in due quote.
- **Regioni passive:** nessuna giunta; solo gli atti di Infrastrutture e di Sviluppo economico scelgono una Regione.
- **Fondo di riserva** del Presidente del Consiglio per le emergenze.
- Nella trattativa di coalizione si spartiscono solo i dieci dicasteri.

## 2026-10-16: autonomia dei ministri
Grilling sul ticket wayfinder «Autonomia dei ministri» (`.scratch/v1-giocabile/issues/04-autonomia-dei-ministri.md`, con il dettaglio). Tutte le raccomandazioni accettate.
- **Quattro grandezze per ministro:** capacità 1-5, che moltiplica l'effetto degli atti e ne divide il ritardo; posizione sull'asse a sette punti, quella del partito più uno scarto personale; un obiettivo, cioè un indicatore del dicastero da portare a soglia; soddisfazione 0-100. Prende da [[ministri-e-gabinetto]] il ministro come moltiplicatore; al posto della lealtà c'è la soddisfazione con le sue cause.
- **Agenda annunciata a inizio turno:** il ministro sceglie fra i suoi 4 atti con un punteggio leggibile (obiettivo, posizione, piccolo rumore) e spiega la scelta in una riga. Il Presidente del Consiglio può fermare l'atto o dirottarlo su un altro, e paga in soddisfazione.
- **Iniziativa normativa a entrambi:** ogni ministro ha al massimo una proposta in sospeso, con tre inneschi (fondi, evento, riforma); il Presidente del Consiglio accoglie, rinvia una volta o respinge. Le norme del Presidente del Consiglio hanno comunque un ministro proponente, che firma e perde soddisfazione se la norma va contro la sua posizione.
- **Nessuna revoca:** il Presidente del Consiglio chiede le dimissioni, come nella prassi costituzionale. La sfiducia individuale del parlamento è un evento (Corte cost. sent. 7/1996, https://www.cortecostituzionale.it/actionSchedaPronuncia.do?anno=1996&numero=7).
- **Dimissioni deterministiche e visibili:** sotto 40 di soddisfazione il ministro minaccia di dimettersi e ne dice la causa; sotto 25 a fine turno si dimette. È il criterio di [[personaggi-fazioni-e-stabilita-interna]]: ogni soglia si vede prima di essere superata.
- **Successione:** rosa di 3 nomi proposta dal partito titolare della quota, un turno di interim e un turno di rodaggio.
- **Raccordo con la coalizione:** la soddisfazione dei ministri di un partito è uno degli ingressi del suo scontento ([[formazione-di-coalizioni]]); i tecnici non toccano nessun partito.

## 2026-10-17: coalizione e crisi, opposizione invece del game over
Grilling sul ticket wayfinder «Coalizione e crisi» (`.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md`, con il dettaglio).
- **Perdere il governo non chiude la partita.** Sfiducia, sconfitta elettorale e ribaltone portano il giocatore all'opposizione. Mario: «il giocatore torna all'opposizione», e alle sue dinamiche va dedicata una sessione a parte (ticket «Opposizione»). Supera il «si perde con la sconfitta elettorale o con la sfiducia» del 16/10.
- **Coalizione pre-elettorale scelta sui sondaggi.** Mario: il punto è «guardare i sondaggi politici e capire come muoversi». Ogni partito confronta il valore atteso delle coalizioni possibili (seggi attesi, probabilità di governare, distanza di posizione) e mostra la sua disponibilità con la causa. Anche un alleato di governo può cambiare campo. Con la 180/2026 i seggi premio si dividono nell'accordo pre-voto ([[sistemi-elettorali]]); con una legge senza premio la coalizione dichiarata vale come impegno politico.
- **Spartizione con Gamson come richiesta, non come esito.** Il giocatore propone; ogni alleato chiede `0,07 + 0,79 × quota di seggi` in punti di peso dei dicasteri ([[formazione-di-coalizioni]]). Sotto la richiesta accetta ma con meno intesa. I tecnici escono dalla quota del giocatore.
- **Intesa** 0-100 per partito alleato, con le cause visibili come la soddisfazione dei ministri; ultimatum sotto 40, uscita sotto 25.
- **Uscita di un alleato:** rimpasto se la maggioranza regge, altrimenti un turno di crisi per allargare o dimettersi. Poi la sfiducia, votata a blocchi perché l'art. 94 Cost. vuole l'appello nominale. Niente appoggio esterno nella v1.
- **Elezioni anticipate** solo con le dimissioni del giocatore. Il Capo dello Stato, istituzione a regole, verifica prima se esiste un'altra maggioranza (ribaltone). È la leva dello scioglimento di [[cicli-politico-economici-e-luna-di-miele]].
- **La legge elettorale è una misura.** Si parte dalla 180/2026; Mario ha valutato una proporzionale di partenza ed è tornato alla 180. Il giocatore la cambia con una legge ordinaria, da un menu di parametri, e la nuova legge vale dal voto successivo (ticket «Legge elettorale come misura»).

## 2026-10-17: elettorato a gruppi, voto logit
Grilling sul ticket wayfinder «Elettorato» (`.scratch/v1-giocabile/issues/06-elettorato.md`, con il dettaglio). Raccomandazioni accettate; Mario ha chiesto almeno tre fasce di reddito.
- **Gruppi come motore, individui come finestra.** Scioglie la tensione fra [[cittadini-simulati]] e [[elettorato-e-gruppi-di-voto]] a favore dell'aggregato. 360 gruppi: 6 condizioni (dipendente privato, dipendente pubblico, autonomo, pensionato, disoccupato o inattivo, studente) × 3 fasce di reddito × 20 regioni. Le regioni servono perché con la 180 il Senato riparte i seggi regione per regione ([[sistemi-elettorali]]).
- **Posizione come istogramma** sui 7 punti dell'asse dentro ogni gruppo; un solo asse, lo stesso dei partiti. Le issue entrano come interessi materiali di ogni condizione, in una tabella fissa, più il reddito della fascia.
- **Voto logit** ([[voto-spaziale]]): distanza dalla posizione percepita del partito, valence, giudizio retrospettivo sugli interessi per i partiti di governo, asimmetrico ([[voto-economico-e-retrospettivo]]), fedeltà; l'astensione è un'alternativa legata alla fiducia.
- **Colpa ripartita:** una quota fissa al partito del Presidente del Consiglio, il resto ai partiti dei dicasteri che toccano gli interessi del gruppo.
- **Posizione percepita:** quella di un partito di governo scivola verso le misure che firma o vota; quella dichiarata resta fissa (superato il 17/10 da «partiti IA»: la dichiarata si muove di mezzo punto a legislatura).
- **Dinamica:** posizioni ancorate con Friedkin-Johnsen più spinte di eventi e campagna ([[opinion-dynamics]]); niente confidenza limitata fra gruppi nella v1. Le quote dei gruppi si muovono ogni turno con lavoro e reddito, ogni anno con l'età.
- **Elettori campione** senza stato, ricavati dal sondaggio e non dallo stato vero; il giocatore vede solo sondaggi con incroci ([[sondaggi-ed-errore]]).

## 2026-10-17: modello economico della v1
Grilling sul ticket wayfinder «Modello economico della v1» (`.scratch/v1-giocabile/issues/07-modello-economico-v1.md`, con il dettaglio). Raccomandazioni accettate; Mario ha precisato che la spesa obbligatoria deve poter cambiare regime, non solo parametri.
- **Produzione a 10 settori** con Leontief nazionale; ogni regione ha il suo mix, da cui prende il suo gap ([[input-output-leontief]]). SFC a 4 settori senza banche; niente gravità, l'estero è domanda esogena ([[motore-economico-raccomandato]]).
- **Congiuntura** Rudebusch-Svensson ricalibrata, shock stocastici a seme fisso ([[macro-neokeynesiano-piccolo]]). **BCE** con regola di Taylor sull'area euro, Italia pesata per la sua quota: niente leva monetaria né governatore da nominare ([[banca-centrale-e-inflazione]]).
- **Spread non lineare** con il TPI come tetto condizionato al rispetto delle regole UE (una PDE con azione efficace non lo esclude, un giudizio di «nessuna azione efficace» sì); oltre la soglia, scelta fra MES con OMT e ristrutturazione del debito; uscita dall'euro fuori dalla v1 ([[finanza-pubblica]], [[regole-fiscali-ue]]). Regole UE nella forma del ticket 02.
- **Okun per regione** con u* che ha isteresi; reddito disponibile per i 360 gruppi, deflazionato con tre panieri, uno per fascia; niente decili ([[mercato-del-lavoro-e-disuguaglianza]]).
- **Fisco**: IRPEF a scaglioni, IVA, IRES, contributi, con elasticità del reddito imponibile. **Spesa dei dicasteri** divisa fra salari (turnover e rinnovi) e acquisti.
- **Spesa obbligatoria** fuori dai dicasteri (pensioni, sussidi, interessi): regime e parametri cambiabili per legge, voci nuove istituibili per legge, cambio di regime pro rata sulle coorti future; Mario: nel gioco si può tenere il contributivo e variarne i parametri, «oppure ci deve essere un modo per poter passare ad altri sistemi come quello retributivo». La colpa di pensioni e sussidi va a Lavoro e Welfare.
- **Piramide nazionale per coorti** per pensioni e passaggi fra gruppi; **PIL potenziale** mosso da infrastrutture, capitale umano, giustizia, energia e demografia.
- Numeri per l'Italia in [[economia-italiana-parametri]].

## 2026-10-17: iter parlamentare
Grilling sul ticket wayfinder «Iter parlamentare» (`.scratch/v1-giocabile/issues/08-iter-parlamentare.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate; fatti in [[iter-legislativo-italiano]].
- **Voto per linea con franchi tiratori.** Ogni partito vota sì o no in base alla distanza dalla misura, all'intesa (per gli alleati) e a un malus fisso (per l'opposizione); una parte dei suoi parlamentari defeziona, con un'estrazione a seme fisso. Il giocatore vede prima la forbice e dopo il conteggio. Risponde al voto deterministico criticato in [[parlamento-e-legislazione]]. Col voto segreto sulla legge elettorale, che esiste solo alla Camera, i franchi tiratori raddoppiano.
- **Decreto solo con un innesco** (evento aperto o variazione di fondi), mai su legge elettorale, costituzione o bilancio. Vale subito e si converte o decade nello stesso turno; se decade, il denaro speso resta speso e non si riemana.
- **Legge in 2 turni** (i 3-4 reali sono un parametro di bilanciamento). Gli alleati lontani dalla misura chiedono un emendamento che la attenua; calendario a 3 posti più 1 per una costituzionale. Una legge respinta si ripresenta dopo 2 turni.
- **Questione di fiducia:** niente emendamenti né franchi tiratori, un turno in meno, intesa che cala; se respinta, il governo si dimette.
- **Legge di bilancio** nel turno ottobre-dicembre; gli emendamenti degli alleati sono richieste di budget; un turno di esercizio provvisorio, poi le dimissioni.
- **Norma costituzionale** (durata della legislatura, numero dei parlamentari) con l'art. 138 e il referendum confermativo votato dai gruppi dell'elettorato. Niente rinvio presidenziale né Corte costituzionale nella v1.

## 2026-10-17: giro del turno
Grilling sul ticket wayfinder «Giro del turno» (`.scratch/v1-giocabile/issues/09-giro-del-turno.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate.
- **Turno discreto in tre fasi.** Apertura (esito del turno precedente, eventi, agende, proposte, sondaggio), decisioni in ordine libero, chiusura avviata dal giocatore in ordine fisso: atti dei ministri → voti d'aula → economia ed elettorato → soglie → eventi nuovi. Supera il tick giornaliero con pausa di [[tempo-e-ritmo]].
- **Nessuna moneta delle azioni.** Niente capitale politico: i limiti sono calendario, inneschi, budget, soddisfazione, intesa e valence.
- **Il turno si chiude anche senza clic.** Le decisioni obbligate hanno un esito di default; una proposta di ministro ignorata per un turno vale come respinta.
- **Eventi solo in apertura**, affrontabili nello stesso turno; **un sondaggio pubblico** gratuito a trimestre.
- **Durata obiettivo:** 5-10 minuti un turno ordinario, 20-30 bilancio, elezioni e crisi, circa 2-3 ore a legislatura.
- **Esito del turno** rivelato a ritmo ([[movimento-e-feedback]]), al massimo 7 voci in evidenza, il resto nel registro. **Tre indicatori sintetici** sempre a schermo: consenso, maggioranza, spread ([[feedback-visivo-dello-stato]]).
- **Anteprima di una misura:** direzione, ritardo, gruppi colpiti e forbice del voto, senza cifre esatte.
- **Bilancio, crisi e voto** sono varianti dello stesso scheletro; le elezioni si tengono nella chiusura e la trattativa occupa il turno dopo, col governo uscente in interim.

## 2026-10-17: direzione artistica (verdetto parziale)
Prototipo del ticket wayfinder «Direzione artistica» (`.scratch/v1-giocabile/issues/10-direzione-artistica.md`). Su tre varianti dello stesso cruscotto d'apertura del turno:
- **Direzione B · Dossier con rete** (carta stampata più la rete di D), come raccomandato in [[direzioni-artistiche-candidate]].
- **Prese dalle altre varianti:** l'esito del turno come prima pagina di giornale, in una schermata propria; il listino dell'agenda con tutti e 10 i ministri; lo schema dello Stato come vista secondaria.
- **Mappa delle regioni a tinte piatte**, cinque classi di un inchiostro; il retino solo per ritratti e illustrazioni.
- **Aperto, il tono:** il prototipo è «troppo serio, rimane comunque un gioco». Deve restare semplice e comprensibile. Si decide su un secondo prototipo con stili giocosi, da [[tono-giocoso-su-temi-seri]] e dalle altre pagine nuove di `estetica/`.

## 2026-10-17: direzione artistica (verdetto)
Secondo prototipo del ticket (`.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html`), quattro stili giocosi sulle scelte qui sopra:
- **Stile Il Settimanale** ([[tono-giocoso-su-temi-seri]]): il dossier con rete diventa un settimanale satirico inventato. Occhiello serio col fatto e il numero, titolo spiritoso sotto, vignetta, note a biro sulla rete e sullo schema dello Stato.
- **Prese dagli altri stili:**
  - il borsino dei ministri con caricatura, voto e motto;
  - la carta del ministro che si gira su Ferma e Dirotta ([[succo-per-interfacce-di-carte-e-numeri]]);
  - gli indicatori sintetici come tracciati a caselle ([[tabellone-e-carta-ritagliata]]);
  - l'anteprima della misura come un mobile appeso, che oscilla solo a un gesto e mai in loop ([[grafica-italiana-giocosa]]).
- **Tono grave:** la cornice giocosa tace. Restano una pagina neutra col fatto e l'inchiostro, senza battute né illustrazioni.
- **Aperta, la regola dei 9 px sul Deck:** si misura il corpo o la x-height? Il prototipo sta a 12 px di corpo.

## 2026-10-17: opposizione
Grilling sul ticket wayfinder «Opposizione» (`.scratch/v1-giocabile/issues/12-opposizione.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate; fatti in [[iter-legislativo-italiano]]. Si evita l'opposizione finta di [[democracy-4]]; da [[power-and-revolution]] si prende solo il fatto che all'opposizione si gioca.
- **Stesso turno trimestrale a tre fasi**, 2-4 minuti; un comando fa scorrere i turni finché non serve una scelta.
- **Vista pubblica:** indicatori, sondaggio, misure del governo avversario con anteprima, intesa dei suoi alleati in tre fasce. Niente dicasteri, budget o agende in anticipo.
- **Leve d'aula:**
  - la linea del proprio partito, scelta dal giocatore, che mette in gioco franchi tiratori e posizione percepita;
  - la sfiducia al governo o a un ministro, che passa solo se un alleato in ultimatum ci guadagna;
  - una proposta di legge alla volta, nel posto riservato alle opposizioni;
  - il referendum abrogativo con quorum endogeno, uno pendente alla volta;
  - il referendum confermativo, a scelta.
- **Ritorno al governo** col voto o con un ribaltone guidato dal giocatore: il suo partito è il primo della nuova maggioranza, oppure il primo accetta la sua guida. Mai alleato minore nella v1.
- **Fine partita** solo senza seggi in entrambe le Camere o per abbandono, con l'eredità come resoconto. Il giocatore resta sempre leader del suo partito.
- **Fuori:** piazza, media e riposizionamento sull'asse restano a campagna elettorale, informazione imperfetta e partiti IA.

## 2026-10-17: catalogo degli atti e delle leggi di riforma
Grilling sul ticket wayfinder «Catalogo atti e riforme» (`.scratch/v1-giocabile/issues/16-catalogo-atti-e-riforme.md`, con il dettaglio; schema ed elenco in `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md`). Tre giri, tutte le raccomandazioni accettate. Si evitano le migliaia di formule scritte a mano di [[policy-e-effetti]]; i ritardi seguono [[ritardi-e-inerzia-delle-politiche]].
- **40 atti del ministro**, quattro per dicastero in una griglia 2×2: rapido o lento × sinistra o destra. Puntuali, o cantieri che impegnano il margine per più turni e rendono a lavori finiti. Effetti solo sulle 1-2 grandezze del dicastero, con ritardo come impulso che decade o stock con vita utile; mai effetti diretti sull'elettorato.
- **Leggi di riforma = disposizioni:** 30 regole del paese con un valore in vigore (età pensionabile, regime dei sussidi, forfettario, prescrizione, nucleare…), ognuna di un dicastero. Una legge ne cambia il valore; linea contro lo status quo, emendamento a metà strada, abrogazione al valore precedente.
- **Turno zero:** valori reali 2026 con la legge che li ha fissati, quindi abrogabili per referendum anche le leggi vere.
- **Il ministro propone** la disposizione più lontana dalla sua posizione, oppure lo sblocco del prerequisito di un atto.
- **Contenuto:** Sanità scritta per intero come modello; gli altri 36 atti e le tabelle degli effetti si scrivono dopo la spec.

## 2026-10-17: partiti IA
Grilling sul ticket wayfinder «Partiti IA» (`.scratch/v1-giocabile/issues/18-partiti-ia.md`, con il dettaglio). Tre giri, raccomandazioni accettate tranne una: Mario ha voluto anche le scissioni, non sette partiti fissi. Regole da [[abm-elettorali]]; si evita «all political parties play identically» di [[partiti-e-coalizioni]].
- **Regole a punteggio, leggibili:** i partiti IA usano le stesse formule del giocatore (valore atteso, punteggio dell'agenda, linea) con un piccolo rumore a seme fisso, e ogni mossa è spiegata in una riga. Niente ricerca in avanti.
- **Due tratti visibili:**
  - la vocazione, identitaria, mista o governista (α di Lehrer-Schumacher);
  - la regola di movimento: estremi Sticker, sinistra e destra Aggregator, centrosinistra e centrodestra Hunter, centro Governator satisficing.
- **La posizione dichiarata si muove** una volta a legislatura, nel turno prima del voto, di mezzo punto al massimo e senza scavalcare il vicino. Supera il «dichiarata fissa» del 17/10 (elettorato) e le «posizioni fisse» del 16/10; la percepita scivola come prima.
- **Leader** per ogni partito: porta la valence, guida il governo avversario se il partito è il più grande, cambia dopo una sconfitta.
- **Governo avversario:**
  - guida il partito più grande, spartizione = Gamson esatta;
  - programma pubblico di disposizioni lontane dalla posizione della coalizione;
  - bilancio dell'anno prima nel tetto UE, eventi con l'opzione di default;
  - condotta che minimizza il rischio di cadere (emendamenti, fiducia, crisi);
  - voto anticipato quando rende più che restare.
- **Contro il giocatore:**
  - sfiducia solo quando passerebbe, con un turno per reagire;
  - proposta-imbarazzo che spacca la maggioranza;
  - referendum abrogativo sulla legge che vince;
  - malus d'opposizione che cresce nell'ultimo anno di legislatura.
- **Vita dei partiti:**
  - partito di protesta che nasce prima del voto in un tratto dell'asse scoperto e con astensione alta;
  - scissione immediata quando percepita e dichiarata distano 1 punto, che vale anche per il partito del giocatore;
  - scioglimento dopo due elezioni senza seggi;
  - tetto di 10 partiti, niente fusioni.

## 2026-10-17: stack e linguaggio del core
Grilling sul ticket wayfinder «Stack e linguaggio del core» (`.scratch/v1-giocabile/issues/11-stack-e-linguaggio-del-core.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate. Supera la raccomandazione Godot + gdext di [[confronto-stack]]: dopo la direzione artistica la UI è un settimanale da impaginare, la mappa è a tinte piatte e il pubblico della v1 è Mario sul Mac.
- **Stack:** Tauri 2.12 + core di simulazione in Rust ([[core-rust-e-ui-separata]], [[tauri]]). UI in HTML/CSS/TypeScript (Svelte 5); mappa in SVG, senza WebGL. Il core non fa I/O, quindi resta compilabile in WASM.
- **Numeri:** interi per euro, seggi, voti ed elettori; `f64` per Leontief, congiuntura e logit, arrotondati quando toccano i conti pubblici ([[architettura-simulazione]]). Partita riproducibile sullo stesso binario; salvataggi come snapshot.
- **Modding v1:** solo dati in file di testo; lo scripting ([[modding-e-dati-esterni]]) resta un punto di aggancio.
- **Testo:** minimo 12 px a scala 1×, scala della UI regolabile; niente regola dei 9 px del Deck.
- Le pagine di `estetica/` che parlano di Godot Compatibility (StyleBoxFlat, Tween, cutout) sono superate come via d'implementazione; le controparti CSS dei prototipi diventano la via diretta. Da correggere al prossimo lint.

## 2026-10-17: legge elettorale come misura
Grilling sul ticket wayfinder «Legge elettorale come misura» (`.scratch/v1-giocabile/issues/13-legge-elettorale-come-misura.md`, con il dettaglio). Tre giri, tutte le raccomandazioni accettate. Prassi su fiducia e voto segreto verificata in [[iter-legislativo-italiano]] §5.
- **Menu a gradini** ([[sistemi-elettorali]]):
  - parametri: soglia di lista 0/3/4/5%; coalizioni sì/no, con soglia 0/10% e ripescaggio; premio nessuno, alla lista o alla coalizione, con soglia 40/42/45% e taglia circa 9 o 18% dei seggi; tetto 55/60% o nessuno; formula Hare o D'Hondt;
  - vincoli fissi: Senato regionale, premio unico per le due Camere, soglia del premio mai sotto il 40% (Corte cost. 1/2014 e 35/2017, scritti nel menu perché in v1 la Corte non c'è);
  - niente uninominali, ballottaggio, preferenze.
  Impostazioni pronte: 180, proporzionale con soglia 3%, proporzionale puro. Premio e tetto in quota dei seggi.
- **Forma:** disposizioni fuori da ogni dicastero, proposte solo dal Presidente del Consiglio. Si abrogano una alla volta, anche per referendum, e tornano al valore precedente (al turno zero: Rosatellum). Valgono dal voto successivo, senza finestra di blocco.
- **Linea sui seggi attesi:** ogni partito proietta i seggi sul sondaggio pubblico e vota sì se ci guadagna. Un alleato che perde seggi paga intesa. Nessun costo diretto di consenso. Il giocatore vede prima la tabella dei seggi guadagnati e persi.
- **Emendamento:** gradino di mezzo sui gradini; sui parametri a scelta, rinvio di un anno (precedente: l'Italicum).
- **Fiducia:** copre gli articoli, non il voto finale. Alla Camera il voto finale resta segreto, con franchi tiratori raddoppiati; se boccia la legge, cade la legge, non il governo.
- **Governo avversario:** la cambia nel programma o più tardi, quando la proiezione gli rende abbastanza seggi.

## 2026-10-17: crisi ed eventi
Grilling sul ticket wayfinder «Crisi ed eventi» (`.scratch/v1-giocabile/issues/15-crisi-ed-eventi.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate. Supera in parte la proposta di [[crisi-ed-eventi]]: il regista adattivo resta fuori.
- **Origine:** situazioni a soglia con isteresi, più eventi puntuali con un rischio per turno calcolato dallo stato ed estratto col seme fisso. Niente regista adattivo, perché sarebbe una causa fuori dal modello; al massimo un evento puntuale nuovo per turno. Situazioni e crisi del debito sono fuori dal tetto.
- **Famiglie:**
  - situazioni dei dicasteri: recessione, criminalità, liste d'attesa, carceri, energia, sbarchi;
  - shock economici oltre 2σ;
  - crisi del debito;
  - emergenze naturali per regione;
  - scandali.

  Le scadenze a data fissa sono calendario, non eventi. L'epidemia va fra i punti di aggancio.
- **Forma:** ogni evento cita la causa e ha 2-3 opzioni chiuse. Il default è l'opzione meno irreversibile (MES per il debito). Il decreto tocca solo i dicasteri coinvolti, tranne nella crisi del debito.
- **Voto:** i danni passano dalle grandezze e quindi dagli interessi. La valence si muove solo per gli scandali e per la gestione delle emergenze. Spinte di posizione piccole, solo per criminalità e sbarchi.
- **Situazioni:** aprono evento e decreto solo all'accensione, poi restano come stato. Le soglie sono visibili.
- **Scandali:** il rischio viene dalla corruzione. Colpiscono un ministro o un leader IA, mai il giocatore. Un ministro si difende (default) o gli si chiedono le dimissioni; se lo si difende, l'opposizione può presentare la sfiducia individuale.
- **Emergenze:** rischio regionale dai dati reali, gravità ridotta dalla prevenzione. Le opzioni sono fondo di riserva (default), decreto a debito o niente; con vittime vale il tono grave.

## 2026-10-17: tabella degli interessi
Grilling sul ticket wayfinder «Tabella degli interessi» (`.scratch/v1-giocabile/issues/17-tabella-degli-interessi.md`, con la tabella e il dettaglio). Due giri; Mario ha aggiunto la sicurezza agli interessi di dipendenti privati e autonomi e ha dato la disoccupazione a Lavoro e Welfare.
- **Denaro da una voce sola:** il reddito disponibile reale della fascia, col suo paniere, vale per tutti e contiene salario, pensione, sussidio, imposte e prezzi. Gli altri indicatori sono solo cose che non sono denaro: disoccupazione regionale, indicatori di servizio, occupazione pubblica.
- **Giudizio:** cumulato con memoria (emivita intorno a 4 turni), asimmetrico indicatore per indicatore come in [[voto-economico-e-retrospettivo]]; contano le variazioni, non i livelli; un partito che entra al governo parte da zero.
- **Tabella:** 6 righe, una per condizione; la fascia agisce già da reddito, paniere e imposte. Pesi provvisori.
- **Disposizioni:** mai dirette negli interessi; effetti materiali dagli indicatori, parte simbolica dalla posizione della misura. Il modello economico deve ripartire l'evasione per condizione e portare il salario minimo nel reddito della fascia bassa dei dipendenti privati.
- **Colpa:** reddito primario e prezzi al solo Presidente del Consiglio; disoccupazione, pensioni, sussidi e reddito minimo a Lavoro e Welfare; imposte, salario e occupazione pubblica a Economia e Finanze; ogni servizio al suo dicastero.

## 2026-10-17: partenza della partita
Grilling sul ticket wayfinder «Partenza della partita» (`.scratch/v1-giocabile/issues/20-partenza-della-partita.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate.
- **Un turno prima del voto.** Con la 180/2026 la coalizione si dichiara prima del voto ([[sistemi-elettorali]]), quindi la partita comincia lì: il giocatore legge i sondaggi, dichiara la coalizione, il voto si tiene in chiusura. **Supera** l'«appena entrato in parlamento con un buon risultato» del 16/10: al turno zero il partito è fuori dal parlamento e il buon risultato è atteso, non scritto. Perdere il primo voto porta all'opposizione.
- **Fondazione:** nome e uno dei sei mezzi punti fra due partiti vicini (1,5 … 6,5), con risultato atteso e coalizioni possibili per ogni posto. Il peso viene dalla luna di miele del leader nuovo, la stessa del partito di protesta, tarata perché il posto medio dia circa il 15%.
- **Calendario:** turno zero a ottobre-dicembre 2026, il primo con la 180; bilancio 2027 già approvato dal governo uscente (2026 indicizzato nel tetto UE); spartizione a gennaio-marzo 2027.
- **Parlamento** dal voto simulato dal motore con seme fisso; **governo uscente** di centrodestra e destra IA, in interim durante la trattativa; giudizio a zero per tutti.
- **Taratura:** valence dei sette partiti sulle intenzioni di voto reali di ottobre 2026 sommate per posto (CHES, [[ches]]), corrispondenza offline e fuori dai dati. Lo scenario di partenza ha sempre una coalizione vincibile; l'editor avvisa senza impedire.
- La fase da outsider resta fuori, ma fondazione e prima campagna entrano, ridotte a una schermata e a un turno.
- **Dati del turno zero** (ticket 23, [[stato-del-paese-al-turno-zero]]): stato base al 2026T2, l'ultimo trimestre coerente, portato a inizio ottobre con gli indicatori mensili di luglio-settembre trattati come dati osservati. Il calendario di gioco resta ottobre-dicembre 2026.

## 2026-10-18: editor dello scenario
Grilling sul ticket wayfinder «Editor dello scenario» (`.scratch/v1-giocabile/issues/21-editor-dello-scenario.md`, con la tabella dei cursori e il dettaglio). Due giri, tutte le raccomandazioni accettate. Precisa la «partenza su misura» del 16/10.
- **Struttura fissa, stato modificabile.** Lo scenario su misura non cambia data del turno zero, istituzioni, dicasteri, regioni, gruppi, i sette posti dei partiti IA né il governo uscente; cambia stato economico, demografia, elettorato, legge elettorale di partenza e disposizioni.
- **Solo ingressi.** Lo scenario contiene grandezze indipendenti; interessi, spread, disavanzo, bilancio 2027, sondaggio e parlamento si ricalcolano al caricamento con le formule del gioco, quindi uno stato incoerente non si può scrivere. La valence resta l'ingresso tarato sullo scenario di base; il sondaggio di partenza è derivato.
- **Sei cursori** (debito, ricchezza con le soglie ferme in euro, disoccupazione insieme a u*, inflazione di partenza, età, orientamento dell'elettorato) più i menu della legge elettorale e delle disposizioni; il resto valore per valore solo nel file di testo, come vuole il modding solo dati.
- **Limiti e avvisi:** fuori dall'intervallo di taratura il valore è rifiutato; uno scenario senza coalizione vincibile o già in crisi al turno zero è lecito, con un avviso.
- **Sequenza:** scelta dello scenario prima della fondazione; file di differenze dalla base, seme nella partita; riepilogo dei derivati dal vivo, a cifre esatte.
- **Disposizioni cambiate** attribuite a una legge del governo uscente del 2026, col valore di base come precedente.

## 2026-10-18: abrogazione e gradini di partenza
Grilling sul ticket wayfinder «Abrogazione e gradini di partenza» (`.scratch/v1-giocabile/issues/24-abrogazione-e-gradini-di-partenza.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate. Sostituisce «abrogazione al valore precedente» del catalogo (17/10) e della legge elettorale (17/10), alla luce di [[disposizioni-al-turno-zero]].
- **Abroga solo il referendum.** Niente abrogazione per legge: il parlamento porta una disposizione a qualunque gradino, compreso quello di prima.
- **Referendum su una disposizione per quesito**, cioè la legge che ne ha fissato il valore in vigore; cancella anche un valore in arrivo. Esito: il **valore dopo l'abrogazione**, di default il precedente (semplificazione dichiarata per le novelle testuali), fisso solo per il reddito minimo («non istituito»). Al turno zero, precedente al gradino più vicino; se coincide col vigente, nessun quesito.
- **Non referendabili** le cinque disposizioni fiscali (forfettario, cuneo, detrazione edilizia, accise, incentivo agli investimenti); referendabili 27 su 32, legge elettorale compresa.
- **Vincolo referendario:** il gradino abrogato è bloccato fino alla legislatura successiva, per tutti.
- **Valore in arrivo:** valore già deciso per legge con il turno da cui vale. Nucleare «vietato» con «consentito» in arrivo da ottobre 2027; incentivo agli investimenti 0 / 10 / 20 / 40* in % del costo, con 0 in arrivo da ottobre 2028.
- **Gradini:** salario minimo «nessuno*» e concorrenza «licenze chiuse*» restano; rinnovabili «ordinarie*» / «accelerate».

## 2026-10-18: informazione imperfetta
Grilling sul ticket wayfinder «Informazione imperfetta» (`.scratch/v1-giocabile/issues/25-informazione-imperfetta.md`, con il dettaglio). Quattro giri; due scelte di Mario contro la raccomandazione: la cassa del partito e le previsioni del centro studi. Precisa l'«informazione imperfetta» del 16/10 e la proposta a tre livelli di [[consiglieri-e-informazione-imperfetta]], senza il terzo livello.
- **Due livelli, nessun bias:** prima stima economica rumorosa e riveduta il turno dopo, indicatori di servizio con un turno di ritardo, spread, seggi e voti esatti; nessun ministro piega i numeri, il giornale dell'esito non è manipolabile.
- **Sondaggio pubblico** a tre strati d'errore ([[sondaggi-ed-errore]]): 3-4 istituti, bias comune per legislatura rivelato dal voto, campionamento. Silenzio elettorale irrilevante a turni trimestrali.
- **Attori:** propri in cifre, intesa avversaria in tre fasce.
- **Anteprima:** costo esatto, effetti a fasce con errore che cala con la capacità del ministro proponente; tabella dei seggi = proiezione sul sondaggio pubblico.
- **Rischio:** vicinanza alla soglia delle situazioni visibile, pericolosità naturale nota, probabilità per turno nascosta.
- **Cassa del partito**, risorsa nuova: 2 per mille sui voti, donazioni sull'intenzione di voto, dote alla fondazione, mai sotto zero. Compra, una volta per tipo a turno, sondaggio privato, test di una misura, centro studi (stima del trimestre e previsione a 4 turni sul percorso in corso). I partiti IA hanno la stessa cassa ma la spendono solo in campagna. **Supera** in parte «nessuna moneta delle azioni» del giro del turno: la cassa non paga le azioni di governo, solo informazione e campagna.

## 2026-10-18: campagna elettorale
Grilling sul ticket wayfinder «Campagna elettorale» (`.scratch/v1-giocabile/issues/26-campagna-elettorale.md`, con il dettaglio). Tre giri; una scelta di Mario contro la raccomandazione: la campagna convince anche, cioè sposta le posizioni degli elettori in modo duraturo. Segue la raccomandazione di [[campagna-elettorale]]: voti al margine, il governo resta il fattore principale.
- **Quando:** solo nel turno prima del voto (scadenza, scioglimento, turno zero), stesse leve al governo e all'opposizione; niente campagna permanente né referendaria.
- **Ordine:** in apertura si muovono i partiti IA e nasce la protesta; nelle decisioni il giocatore si riposiziona, dichiara la coalizione, promette e spende; in chiusura spendono tutti e si vota. Al turno zero niente mosse IA né protesta.
- **Riposizionamento** gratuito di mezzo punto; la percepita fa metà strada, anche per i partiti IA.
- **Bersagli:** fino a 3, ciascuno una condizione o una regione, con una leva: mobilitare e persuadere per quel voto, convincere in modo duraturo con l'ancora di Friedkin-Johnsen (conferma le spinte di campagna del 06).
- **Scala:** rendimenti decrescenti, 1-3 punti a cassa piena.
- **Promesse:** fino a 2 disposizioni a un valore; costano intesa agli alleati contrari; al voto dopo mantenute o mancate muovono la valence, se il giocatore ha governato almeno 4 turni.
- **Partiti IA:** stesse leve, spesa dove un euro rende più voti attesi, promesse dal programma. Protesta con una dote, scissione con la quota di cassa.
- **Vista:** anteprima a fasce, casse rivali pubbliche, spesa altrui nell'esito.

## 2026-10-20: mappa delle 20 regioni
Grilling sul ticket wayfinder «Mappa delle 20 regioni» (`.scratch/v1-giocabile/issues/27-mappa-delle-20-regioni.md`, con il dettaglio). Cinque giri, l'ultimo dopo la ricerca [[disposizioni-territoriali-al-turno-zero]], che ha corretto tre premesse; tre scelte di Mario contro la raccomandazione: disposizioni con perimetro regionale, regione bersaglio per tutti gli atti di Infrastrutture e Trasporti e di Sviluppo economico, autonomia differenziata nella v1. Allarga la regione bersaglio dei dicasteri (03).
- **Nazionali o fermi:** indicatori di servizio e situazioni dei dicasteri; popolazione delle regioni (niente migrazioni interne, seggi da censimento); divario Nord-Sud solo nel file dello scenario.
- **Regione bersaglio** per gli 8 atti di Infrastrutture e Trasporti e di Sviluppo economico: i rapidi danno un impulso alla domanda della regione, i cantieri alzano lo stock della regione (prevenzione, u* regionale). La sceglie il ministro per bisogno e voti del suo partito, pesati dalla vocazione; il Presidente del Consiglio la dirotta finché l'atto è in agenda. Nella regione il merito va al partito del dicastero.
- **Disposizioni territoriali**, valide nel loro perimetro (macroaree ISTAT Nord, Centro, Mezzogiorno): decontribuzione per il Mezzogiorno sui lavoratori in forza (Lavoro e Welfare, 20% al turno zero, in calo per legge fino al 2029); credito d'imposta ZES unica (Sviluppo economico) su 10 regioni con Marche e Umbria, leva lo stanziamento perché le aliquote sono della Carta UE; autonomia differenziata fuori dicastero, non attuata al turno zero, con i gradini delle due intese reali (prime materie, intese complete) su Liguria, Lombardia, Piemonte e Veneto, che agisce sul gettito trattenuto. Le disposizioni sono 35, le non referendabili sette.
- **Mappa:** un livello alla volta a cinque classi (consenso, disoccupazione, reddito, affluenza, pericolosità), più il risultato del voto col colore del primo partito; segni per atti regionali ed emergenze; scheda della regione; selettore per le scelte che chiedono una regione.

## 2026-10-20: salvataggi
Grilling sul ticket wayfinder «Salvataggi» (`.scratch/v1-giocabile/issues/28-salvataggi.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate. Precisa «salvataggi come snapshot» del 17/10 secondo [[architettura-simulazione]].
- **Riavvolgimento:** uno snapshot automatico all'apertura di ogni turno, tenuto per tutta la partita; «torna al turno N» cancella i turni dopo. Niente ironman, niente slot manuali né rami.
- **Stesso caso:** lo stato del generatore è nello snapshot, quindi riavvolgere compra conoscenza, non fortuna; l'eredità conta i riavvolgimenti, senza penalità.
- **Chiudere e abbandonare:** chiudere salva la bozza del turno; abbandonare è esplicito e chiude con l'eredità.
- **Versioni:** il salvataggio copia i dati con cui la partita è cominciata; uno schema diverso si rifiuta, senza migrazioni nella v1.
- **Albo delle partite:** eredità, serie dei tre indicatori sintetici, risultati delle elezioni, riavvolgimenti; alla fine per seggi l'eredità è una bozza ancora riavvolgibile. Più partite in corso insieme.

## 2026-10-21: costo del governare
Grilling sul ticket wayfinder «Costo del governare» (`.scratch/v1-giocabile/issues/29-costo-del-governare.md`, con il dettaglio). Due giri, tutte le raccomandazioni accettate. Precisa la «deriva negativa» di [[voto-economico-e-retrospettivo]]: niente funzione del tempo, il costo emerge dal modello; la forma a U di [[cicli-politico-economici-e-luna-di-miele]] da luna di miele e campagna.
- **Bersagli di bilanciamento**, test di accettazione su partite simulate: strategia media −2,25 punti a legislatura (σ ≈ 4,5), inerte peggio e rieletta meno, riformatrice meglio dell'inerte alla seconda elezione.
- **Aspettative:** il reddito reale si giudica contro la crescita media degli ultimi ~8 turni, alzata dalle promesse.
- **Erosione nominale:** budget, fondo di riserva, scaglioni e soglie, salario e pensione minimi restano nominali finché una legge non li cambia; indicizzate solo le pensioni in essere. Bozza del bilancio sulle cifre dell'anno prima, con «indicizza all'inflazione» e costo in anteprima.
- **Nessun κ** di calendario; si aggiunge come manopola solo se la taratura manca i bersagli.
- **Luna di miele** solo al primo governo del giocatore.
- **Margine discrezionale** vero all'1,6%, allargabile nella legge di bilancio a spese dei servizi o del disavanzo.
- **Visibilità:** crescita attesa, voce nell'esito, perdita reale di budget e scaglioni, sempre con la causa.
- **Manopole** nel file dei dati con valore, intervallo e ticket d'origine; taratura dopo la costruzione, senza spike.

## Collegamenti
- [[soggetti-reali-vs-fittizi]]: la raccomandazione di un paese fittizio non dipende dal prezzo, perché gli art. 96-97 L. 633/1941 vietano anche gli usi che ledono il decoro, non solo quelli a scopo di lucro.
- [[distribuzione-e-aggiornamenti]]: anche un gioco gratuito su Steam paga l'iscrizione da 100 $, e su macOS serve comunque la notarizzazione Apple.
