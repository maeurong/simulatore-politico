Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatto il cruscotto quando il giocatore è all'opposizione? Il programma pubblico del governo avversario nell'ordine in cui va in aula (storia 104), con la linea del giocatore misura per misura e ciò che costa (storia 91); le leve: sfiducia con l'anteprima di chi la voterebbe (storia 92, `anteprima_sfiducia`), posto riservato (storia 94), referendum abrogativo e confermativo (storie 95 e 97); l'intesa degli alleati avversari in tre fasce (storia 90); «Avanti fino a novità» sopra «Chiudi il turno» e come si legge la fermata (storia 89, dal piano di core). Le agende dei ministri avversari non si vedono: i loro atti arrivano in prima pagina.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

### Premesse dall'11/11/2026

[Fondamenta visive](08-fondamenta-visive.md) ha fissato la cornice e i tracciati, ma non cosa mostrano i tre indicatori sintetici quando il giocatore non governa: il consenso è quello del suo partito o della sua coalizione, la maggioranza è quella del governo avversario? Il prototipo delle fondamenta mette valori di comodo. Si decide qui, e vale anche per il turno zero.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [28-opposizione.prototype.html](../prototipi/28-opposizione.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: la Vista 06 e [viste/28-opposizione.json](../viste/28-opposizione.json), che `genera/` (`genera/src/opposizione.rs`) aggiunge giocando col seme 1 a 1,5 sotto un governo IA: la sfiducia al governo presentata e respinta (60 sì contro 340), la sfiducia che passerebbe al turno 4 (217-183, con Sinistra e Centrosinistra in ultimatum; ai ministri, su sei dicasteri su dieci), il referendum abrogativo sul tetto al contante dalle firme all'esito (quorum mancato), il posto riservato del giocatore al turno 3 prima e dopo la proposta sul salario minimo, una linea contraria al default, il tono grave al turno 11 e l'ultimo turno prima del voto. Le anteprime della sfiducia e i rifiuti del prototipo sono quelli veri del core. Le firme che mancano e il referendum confermativo non si raggiungono giocando: nel prototipo sono stati di prova, dichiarati.

### Forma: A · Ordine del giorno, con le leve in testa

Gemella del [Cruscotto al governo](17-cruscotto-al-governo.md): sotto la cornice, le dodici colonne col dettaglio a lato.

- **In testa** l'evento del governo, in sola lettura: famiglia, regione, costo, opzioni e default, «decide il governo».
- **Le tue leve**, primo gruppo di righe, perché con cinque-nove misure in aula scenderebbero sotto la piega: «Sfiducia al governo» (le firme contro un decimo della Camera, chi la voterebbe), «Sfiducia a un ministro» («passerebbe su N dicasteri su 10»), «Posto riservato all'opposizione» (tuo e libero, di un altro, occupato), «Referendum abrogativo», e «Referendum confermativo» quando c'è una costituzionale approvata senza i due terzi.
- **Si vota a fine turno** e **In aula · al voto in …**: le misure del governo nell'ordine dell'aula, ciascuna con Sì · No · «Come da posizione (sì/no)» sulla riga, il default scritto e il costo della linea. Il gruppo dice quante vanno come da posizione.
- **Programma del governo** (storia 104): le disposizioni nell'ordine in cui andranno in aula, «in aula» o «arriva in quest'ordine», e una riga che lo dice: le agende dei ministri avversari non si vedono, i loro atti arrivano in prima pagina (storia 90).
- **La maggioranza**: chi guida il governo e gli alleati, coi seggi di Camera e Senato, la fascia d'intesa e la causa dell'ultimatum.
- **Dettaglio a lato** della riga scelta: per una misura l'emiciclo e le linee dell'[Aula](19-aula-e-calendario.md); per la sfiducia l'anteprima partito per partito, le firme, se passa e cosa costa se è respinta (storia 93); per la sfiducia a un ministro l'anteprima dicastero per dicastero; per il referendum le disposizioni abrogabili coi limiti (storie 95-96); per il posto riservato il rimando all'Aula, dove si scrive la proposta.

### I tracciati della cornice all'opposizione

La premessa del ticket. Lettura 2 del prototipo:

- **Consenso**: quello del tuo partito.
- **Per farlo cadere**, al posto della maggioranza: i sì che mancano alla sfiducia al governo, dall'anteprima vera (`anteprima_sfiducia`), il minimo fra le due Camere; si riempie quando il governo è fragile. Il margine in seggi inganna proprio quando conta: al turno 4 del seme 1 dice +140 alla Camera a tracciato pieno, mentre la sfiducia passerebbe 217-183.
- **Al turno zero**, dove non c'è un governo da far cadere: i seggi che la proiezione pubblica ti darebbe se si votasse oggi, contro la metà più uno della coalizione dichiarata.
- **Conti**: come al governo.

Scale e soglie restano a [Scale dei tracciati](31-scale-dei-tracciati.md). Sulla scala di prova del consenso (dal 25 al 75%) un partito d'opposizione fra il 5 e l'11% resta sempre a zero caselle: quel ticket deve tenerne conto.

### Decisioni sui casi

- **Niente conto in cornice** all'opposizione: il default «come da posizione» quasi sempre va bene ed è scritto su ogni riga.
- **«Avanti fino a novità» sopra «Chiudi il turno»**; dopo una fermata, la banda col perché e il rimando al registro, come in [Prima pagina](16-prima-pagina.md).
- **Conferma sul posto per la sfiducia**: «Conferma: presenta la sfiducia» e «Annulla». Il core non la ritira, una per turno, e respinta costa valence: al seme 1, da 0,2658 a 0,1492 (0,2392 senza sfiducia).
- **Conferma sul posto anche per il referendum abrogativo**: non si ritira e blocca ogni altro quesito («un referendum è già in corso»).
- **Nessuna conferma per il confermativo**: il comando si ridà fino alla chiusura (`crates/sim-core/src/partita.rs:1895`).
- **I rifiuti veri sul posto**, sotto la riga: seconda sfiducia, secondo quesito, «niente da abrogare», disposizione non referendabile, ministro in interim.
- **Tono grave**: le regole delle fondamenta.

### Rimandi

Mario sceglie: tutti e quattro nella PR delle esposizioni del piano di core «esito e Vista completa», in 11g. Fino ad allora il cruscotto mostra il rifiuto sul posto e una frase.

- **La sfiducia presentata**: oggi vive solo in `Parlamento.sfiducia` (`crates/sim-core/src/parlamento/mod.rs:135`); la Vista dopo il comando è uguale a prima.
- **Di chi è il posto riservato** del turno: la Vista porta un booleano solo per il giocatore (`crates/sim-core/src/vista.rs:61`).
- **L'ammissibilità del referendum** per disposizione: `referendabile` è statico, mentre ultimo anno, finestra, «niente da abrogare» e referendum già in corso si sanno solo dal rifiuto (`crates/sim-core/src/partita.rs:1850-1873`, `crates/sim-core/src/parlamento/mod.rs:493-541`).
- **Il costo della linea**: la percepita dopo il voto per sì e per no. Misurato giocando: 1,9363 contro 1,9149 sulla misura 0 del seme 1.

### Guscio e dati

Le scelte passano da `comanda` (`Linea`, `Sfiducia`, `PresentaProposta`, `PromuoviReferendum`, `Confermativo`). Un passacarte nuovo, `anteprima_sfiducia` (`crates/sim-core/src/partita.rs:3805`), nella fetta 11e: serve alla riga, al dettaglio e al secondo tracciato. Nient'altro si calcola nella UI: le firme sono nell'anteprima (`firme`), i seggi attesi nella `proiezione`.

### Stranezze notate

- Le firme bastano sempre: servirebbe un governo con almeno 361 seggi su 400, mai visto in 6 posizioni × 20 semi × 40 turni.
- Il referendum confermativo non si raggiunge giocando sotto un governo IA.
- La linea «No» scelta dove il default è già no costa sei volte i franchi tiratori del default: le due regole misurano la distanza da posizioni diverse (`crates/sim-core/src/parlamento/mod.rs:385-390` contro `crates/sim-core/src/parlamento/voto.rs:75-103`). Mario: difetto del core, nell'Out of scope della mappa.
- Il quesito sul tetto al contante si chiama «Tetto al contante: 1.000 €» in `referendum` e «… 5.000 €» in `esiti_referendum`.
- La banda di prova della fermata nel prototipo della [Prima pagina](16-prima-pagina.md) dice «ultimo trimestre per chiedere un referendum» al turno 19, che è nell'ultimo anno della legislatura, dove il core rifiuta ogni richiesta (`crates/sim-core/src/partita.rs:1873`): il testo vero della fermata va scritto con questa regola.

### Scartato

B · A fronte (il controprogramma su due colonne: senza dettaglio a lato, sfiducia ai ministri e referendum si aprono sotto la fascia delle leve); C · Spoglio (la maggioranza partito per partito al centro: misure e leve diventano elenchi compatti); le letture 1 (margine in seggi del governo avversario) e 3 (consenso dell'opposizione intera); il tracciato barrato al turno zero; il conto delle linee in cornice; le misure sopra le leve.
