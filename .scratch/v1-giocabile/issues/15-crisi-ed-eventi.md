Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come nascono gli eventi nella v1: trigger su soglie del modello, regista adattivo che dosa la tensione, o entrambi? Quali famiglie di eventi entrano (shock economici già stocastici nel motore, crisi del debito con la scelta MES/ristrutturazione, emergenze pagate dal fondo di riserva, sfiducia individuale a un ministro, scandali che toccano la valence) e come ogni evento cita la sua causa dentro il modello (`docs/ricerca/sistemi/crisi-ed-eventi.md`)?

Ingressi: Modello economico (07) dà gli shock con seme fisso e la soglia delle aste deserte; Dicasteri (03) il fondo di riserva; Autonomia dei ministri (04) la sfiducia individuale come evento; Elettorato (06) valence e spinte degli eventi sulle posizioni dei gruppi; Giro del turno (09) i tempi: gli eventi nascono nell'ultimo passo della chiusura, si mostrano solo in apertura, si affrontano nello stesso turno (decreto, fondo di riserva) e, se ignorati, producono i loro effetti nella chiusura.

Ingresso da Opposizione (12): la sfiducia individuale a un ministro è anche una leva dell'opposizione, non solo un evento. Passa se la vota un alleato del governo in ultimatum che dalla caduta del ministro guadagna; i partiti IA all'opposizione la usano contro i ministri del giocatore. Resta da decidere qui se esiste anche come evento indipendente (scandalo che la innesca).

Ingresso da Catalogo atti e riforme (16): la condizionalità del MES si esprime come valori imposti a disposizioni (per esempio età pensionabile, indicizzazione). Un atto può avere una disposizione come prerequisito.

Ingresso da Partiti IA (18): il governo avversario risponde a ogni evento con la sua opzione di default (09), per decreto solo se l'evento lo consente; ogni evento deve quindi avere un default sensato per chiunque governi. All'opposizione i partiti IA presentano la sfiducia, anche individuale, solo quando passerebbe; il bersaglio di quella individuale è il ministro del dicastero col giudizio peggiore fra i gruppi. Una scissione a metà legislatura può aprire una crisi (05).

## Answer

Grilling con Mario del 17/10/2026: due giri, tutte le raccomandazioni accettate, quadro finale confermato. Termini nuovi in `CONTEXT.md`: evento, situazione. La parola «crisi» resta quella di governo (05).

Scelte fatte senza chiedere, perché di dettaglio, confermate col quadro:
- se nello stesso turno escono più eventi puntuali, vince il più grave. Uno shock escluso succede comunque nel motore e va nell'esito col suo default; uno scandalo o un'emergenza esclusi non succedono;
- una situazione non aggiunge effetti materiali dove il motore li produce già (recessione, energia): è un'etichetta con isteresi sopra grandezze che si muovono da sole, più le spinte di posizione dove previste;
- il catalogo degli eventi è contenuto: la spec scrive per intero una famiglia come modello, come la Sanità nel catalogo degli atti (16);
- gli eventi stanno nei file di dati, come il resto del modding (11), con la causa espressa sulle grandezze per nome.

**Origine: soglie e rischio, niente regista.** Due meccanismi (`docs/ricerca/sistemi/crisi-ed-eventi.md`, §1 e §3):
- *situazioni*: si accendono quando una grandezza supera una soglia e si spengono solo sotto una soglia più bassa, così non lampeggiano attorno al valore critico (Democracy 4);
- *eventi puntuali*: ogni turno hanno un rischio calcolato dallo stato (per esempio la corruzione per gli scandali), estratto col seme fisso della partita (07), come il tempo mediano di Paradox.

Niente regista adattivo alla RimWorld: alzare la frequenza perché il giocatore va bene sarebbe una causa fuori dal modello, e la regola è che ogni evento cita la sua (`docs/ricerca/sintesi/raccomandazioni-convergenti.md:77`). Al suo posto, al massimo **un evento puntuale nuovo per turno**, per tenere il turno in 5-10 minuti (09). Situazioni e crisi del debito non contano nel tetto. Il regista va fra i punti di aggancio.

**Famiglie della v1.**
- *Situazioni dei dicasteri:* recessione, ondata di criminalità, collasso delle liste d'attesa, emergenza carceri, crisi energetica, ondata di sbarchi.
- *Shock economici:* quelli del motore (domanda estera, energia, tasso BCE, produttività; 07) diventano evento solo oltre due deviazioni standard dei residui storici; sotto restano una voce dell'esito.
- *Crisi del debito:* le aste deserte (07), con MES o ristrutturazione.
- *Emergenze naturali:* terremoti, alluvioni e frane, incendi e siccità, per regione.
- *Scandali.*

Le scadenze a data fissa (controllo UE di primavera, legge di bilancio, voto) non sono eventi: sono il calendario del turno. L'epidemia è esclusa, perché un modello SIR è un sottosistema intero: va fra i punti di aggancio.

**Forma di un evento.**
- Cita la sua causa: la grandezza, il suo valore e la soglia, oppure il livello di rischio.
- Ha 2-3 opzioni chiuse. Una è il **default**: l'opzione meno irreversibile, che vale se il giocatore non sceglie (09) e che sceglie sempre il governo avversario (18).
- Se l'evento ammette il decreto, questo tocca solo disposizioni e fondi dei dicasteri coinvolti. Fa eccezione la crisi del debito, che apre anche le riforme (08).

**Effetti sul voto.** I danni materiali passano solo dalle grandezze, quindi dagli interessi dei gruppi e dal giudizio retrospettivo (06): niente doppio conteggio. La valence si muove solo per gli scandali e per la gestione delle emergenze: un piccolo premio se il governo risponde, un malus se le ignora. Spinte sulla posizione dei gruppi solo per criminalità e sbarchi, verso destra nella regione colpita, piccole e scritte a mano.

**Situazioni.** Aprono un evento, con la finestra del decreto, solo nel turno in cui si accendono. Poi restano accese come stato finché non scendono sotto la soglia di spegnimento, e si combattono con atti e leggi ordinari. Le soglie di accensione si vedono in chiaro accanto alla grandezza: ogni crisi lenta si vede arrivare (`raccomandazioni-convergenti.md:77`).

**Scandali.**
- Il rischio per turno cresce con la corruzione (grandezza di Giustizia, 03) ed è uguale per tutti i partiti.
- Il bersaglio è un ministro o un leader dei partiti IA, mai il giocatore, che resta sempre leader del suo partito (18).
- Su un ministro del governo le opzioni sono due:
  - *difenderlo* (default): la valence del suo partito cala e l'opposizione può presentare la sfiducia individuale, che passa solo se la vota un alleato in ultimatum (05, 12);
  - *chiedergli le dimissioni*, col costo del 04 (a un tecnico non costa nulla).
- Su un leader IA lo scandalo pesa solo sulla valence del suo partito.

Risposta all'ingresso di 12: la sfiducia individuale non è un evento a sé. È una leva dell'opposizione, e lo scandalo è l'evento che le dà un bersaglio.

**Emergenze naturali.** Il rischio per regione viene dai dati reali di pericolosità (sismica, idrogeologica, incendi): li cerca il ticket research «Pericolosità naturale per regione». La gravità è ridotta dalla prevenzione, cioè dallo stock di infrastrutture e dalla sicurezza ambientale della regione. Le opzioni sono tre:
- fondo di riserva fino a capienza (default);
- decreto con fondi in più a debito;
- nessun intervento, con malus di valence.

Quando ci sono vittime, l'evento è in tono grave (10).

**Shock economici.** Le opzioni sono aiuti per decreto ai settori colpiti, a debito e dentro il conto del tetto UE (02), oppure niente (default). Lo shock energetico ha in più l'opzione di attivare la clausola nazionale per l'energia (02).

**Crisi del debito.** Default: MES (07), perché è l'opzione meno irreversibile; la ristrutturazione resta una scelta attiva.

Effetti sulla mappa:
- nuovo ticket research «Pericolosità naturale per regione»;
- nebbia «Costo del governare e bilanciamento»: soglie di accensione e spegnimento delle sei situazioni, rischi per turno di scandali ed emergenze, soglia di 2σ degli shock, premio e malus di valence della gestione, ampiezza delle spinte di posizione, gravità che fa scattare il tono grave;
- nebbia «Informazione imperfetta»: quanto si vede del rischio di un evento puntuale prima che accada;
- nebbia «Mappa delle 20 regioni»: le emergenze sono regionali, mentre le situazioni dei dicasteri restano nazionali finché quella nebbia non decide altrimenti;
- punti di aggancio: regista adattivo, epidemia (SIR).
