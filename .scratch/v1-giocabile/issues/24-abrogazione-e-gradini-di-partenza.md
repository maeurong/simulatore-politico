Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

La regola del catalogo «l'abrogazione riporta al valore precedente» (`.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:37`) non corrisponde al diritto reale: per Corte cost. 13/2012 il referendum abrogativo non fa rivivere le norme abrogate (`docs/ricerca/sintesi/disposizioni-al-turno-zero.md:62-80`). La si tiene come semplificazione dichiarata, oppure ogni disposizione ha un campo «valore dopo abrogazione» (regola generale, nessun valore, non istituita)? E vale allo stesso modo per l'abrogazione per legge e per quella per referendum?

Con la stessa ricerca (`:51-60`), come si rappresentano al turno zero:
- il nucleare, la cui delega (L. 169/2026) entra in vigore il 15/10/2026, dentro il turno zero: gradino nuovo «delega in corso», oppure «vietato» con un evento a scadenza per i decreti attuativi?
- il credito d'imposta per gli investimenti, che dal 2026 è un iperammortamento al 180%: stessa disposizione al gradino 40%, o disposizione rinominata con gradini sulla maggiorazione?
- i gradini facoltativi: «minimo da CCNL» per il salario minimo (DL 62/2026), le gare dei balneari già previste dal 2027, «lente*» per le rinnovabili, senza riscontro di legge.

Ingresso da Editor dello scenario (21): una disposizione cambiata dall'editor si attribuisce a una legge del governo uscente del 2026, con il valore dello scenario di base come precedente; la regola scelta qui vale anche per lei.

## Answer

Grilling con Mario del 18/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Fatti in `docs/ricerca/sintesi/disposizioni-al-turno-zero.md`. Termini nuovi in `CONTEXT.md`: valore in arrivo, valore dopo l'abrogazione; precisati legge di riforma e referendum abrogativo.

**Abroga solo il referendum.** L'abrogazione per legge sparisce come operazione: una legge di riforma porta una disposizione a qualunque gradino, compreso quello di prima. È anche il diritto reale: dopo un'abrogazione il valore precedente torna solo se il Parlamento lo riscrive (`disposizioni-al-turno-zero.md:66`). Vale anche per la legge elettorale (13), che si cambia gradino per gradino con la linea sui seggi attesi.

**Referendum abrogativo** (12), regole nuove:
- *Bersaglio:* una disposizione per quesito, cioè la legge che ne ha fissato il valore in vigore. È il quesito parziale reale, ed è già la regola della legge elettorale. Se la disposizione ha un valore in arrivo, l'abrogazione lo cancella: un referendum su L. 169/2026 prima di ottobre 2027 lascia il nucleare «vietato».
- *Esito:* la disposizione prende il suo **valore dopo l'abrogazione**, un campo dello schema (`.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:28`) di due tipi:
  - *precedente* (default): il valore che c'era prima della legge abrogata. È fedele dove la legge precedente resta in vigore (età pensionabile, privato accreditato, licenziamento → reintegro, prescrizione → bloccata dopo il primo grado). Per le novelle testuali è una semplificazione dichiarata, perché il gioco non ha un «comma senza cifra» (Corte cost. 13/2012, `:64-68`);
  - *fisso*: un gradino scritto a mano. Nella v1 riguarda solo il **reddito minimo** → «non istituito», perché abrogare la legge che l'ha istituito non fa tornare il regime di prima, come il Reddito di cittadinanza dopo l'Assegno di inclusione.

  Il campo è statico e vale per qualunque legge colpisca la disposizione, anche quelle fatte durante la partita. Per le disposizioni cambiate dall'editor (21) il precedente è il valore dello scenario di base.
- *Turno zero:* se il valore reale precedente non è un gradino o non è noto, si prende il gradino più vicino. Se coincide col valore in vigore, la disposizione non è un bersaglio finché una legge della partita non la cambia (obbligo scolastico; indicizzazione, perché senza il c. 478 resta l'art. 34 della L. 448/1998, già per fasce [INF]).
- *Esclusioni (art. 75):* non referendabili le cinque disposizioni fiscali: forfettario, cuneo fiscale, detrazione edilizia, accise, incentivo agli investimenti. Restano referendabili, per scelta di gioco dichiarata, il ticket, le tasse universitarie e il decreto flussi (nella realtà un DPCM). Le referendabili sono 27 su 32.
- *Vincolo referendario:* il gradino abrogato non si può riproporre fino alla legislatura successiva, né dal giocatore né dai partiti IA; il menu lo mostra bloccato, col motivo. È un paletto della Corte scritto nel menu, come quello del premio (13): Corte cost. 199/2012 [INF, fonte da catturare].
- *Legge elettorale:* tutte le disposizioni di tipo precedente, referendabili (i referendum elettorali reali del 1991, 1993 e 2009), con i valori del Rosatellum come precedente al turno zero. Ogni gradino del menu dà una legge subito applicabile, quindi il requisito di autoapplicatività è rispettato.

**Valore in arrivo.** Si riusa lo stato del regime emendato con entrata in vigore rinviata (16): un valore già deciso per legge, con il turno da cui vale; finché non arriva, una legge di riforma può cambiarlo e i partiti lo leggono nel programma.

**Gradini di partenza:**
- *Nucleare:* «vietato» in vigore, con valore in arrivo «consentito» dal turno ottobre-dicembre 2027 (i dodici mesi della delega, L. 169/2026). Semplificazione dichiarata: la delega vale come legge con effetto differito; nella realtà, se il governo non scrive i decreti, la delega scade gratis. Niente gradino «delega in corso», niente evento programmato, che il sistema degli eventi (15) non prevede.
- *Incentivo fiscale agli investimenti* (rinominato da «credito d'imposta per gli investimenti»): gradini 0 / 10 / 20 / 40* in % del costo equivalente. Il 40% vale l'iperammortamento al 180% con l'IRES al 24% [INF]. Valore in arrivo 0 dal turno ottobre-dicembre 2028, perché la L. 199/2025 copre gli investimenti fino al 30/9/2028: il rinnovo diventa una scelta del bilancio 2028.
- *Salario minimo:* resta «nessuno*», con legge d'origine DL 62/2026 citata nella carta. La disposizione misura una soglia oraria di legge, che non c'è.
- *Concorrenza nei servizi:* resta «licenze chiuse*». Le gare dei balneari dal 2027 sono una semplificazione dichiarata: dividere la disposizione, o darle un valore in arrivo, aprirebbe anche i taxi.
- *Autorizzazioni per le rinnovabili:* gradini rinominati «ordinarie*» / «accelerate». «Lente» era un giudizio, non un valore di legge; i tempi reali restano negli effetti.
- Le altre correzioni di Valori di partenza delle disposizioni (22) restano come lì: intercettazioni «limitate*», concessioni autostradali «a gara*», cuneo con precedente 0.

Effetti sulla mappa:
- nessun ticket nuovo;
- Costo del governare (nebbia): intensità dell'incentivo agli investimenti in % del costo, posizioni dei gradini rinominati;
- fonte da catturare: Corte cost. 199/2012 (vincolo referendario).
