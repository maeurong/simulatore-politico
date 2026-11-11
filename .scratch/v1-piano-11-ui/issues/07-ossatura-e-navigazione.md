Type: grilling
Status: resolved
Blocked by: 01
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Com'è fatta l'ossatura dell'app? Schermate come pagine separate o pannelli dentro il cruscotto del turno; cosa resta sempre a schermo (tre indicatori sintetici, data, turni al voto, chiudi turno); dove si apre l'esito come prima pagina e come si passa alle decisioni; come si raggiungono mappa, registro e schema dello Stato; cosa cambia fra governo, opposizione, trattativa, crisi e partita finita; come entra il tono grave nella cornice; nessuna finestra modale (storia 19). Fissa anche l'elenco definitivo delle schermate da prototipare.

## Answer

Deciso con Mario fra il 10 e l'11/11/2026, in tre giri su un prototipo navigabile: [07-ossatura.prototype.html](../prototipi/07-ossatura.prototype.html). Il prototipo si apre sulla forma decisa; le alternative scartate restano raggiungibili dai parametri scritti in testa al file.

### Ossatura

- **Cornice fissa, sommario di sezioni a pagina intera, dettaglio a lato.** Scartati il cruscotto snodo del prototipo vincolante (lì le schermate erano tre: con dieci il cruscotto diventa una pulsantiera e ogni passaggio costa due clic) e il cruscotto unico (a 1280×800 non entra, e schema dello Stato e mappa chiedono comunque una pagina loro).
- **Cornice**, sempre a schermo dentro una partita: testata piccola, stato del giocatore (al governo, all'opposizione, crisi, trattativa, turno zero, partita finita), data, turno, turni al voto, i tre indicatori sintetici a caselle, «Chiudi il turno». Sulla prima pagina resta, come striscia sopra la testata grande: la storia 21 non ha eccezioni.
- **Sommario**, dieci sezioni sempre nello stesso posto: Prima pagina · Cruscotto · Governo · Aula · Conti · Bilancio · Paese · Partiti · Campagna · Registro. Una sezione che in quello stato non ha niente da mostrare è **barrata e dice perché** («Camere sciolte: niente aula fino al voto»); non sparisce.
- Fuori dal sommario, a destra della stessa riga: Schema dello Stato (vista secondaria, a pagina intera), Impostazioni, Edicola.

### Cosa sta dove

| Sezione | Contenuto | Accesa |
|---|---|---|
| Prima pagina | L'esito del turno. Solo lettura: annuncia l'evento, non lo fa scegliere. «Vai al cruscotto ›» e «Altre N notizie nel registro ›». | sempre, tranne al turno zero |
| Cruscotto | La casa del turno, corta: l'evento aperto con le sue opzioni in testa; ciò che chiede una scelta nel turno (proposte dei ministri, ultimatum, emendamenti, voti a fine turno, bilancio da presentare, campagna da fare), ogni riga col rimando alla sezione; i ministri in evidenza col rimando a Governo. | sempre; cambia nome in trattativa e a partita finita |
| Governo | Borsino dei dieci ministri, carta del ministro a lato, dicasteri con indicatore di servizio e grandezze. | solo quando il giocatore governa |
| Aula | Calendario, misure in esame con linee e forbici, disposizioni (legge elettorale compresa), referendum; a lato l'anteprima della misura. | con le Camere aperte |
| Conti | Bilancio in vigore con la perdita reale, grandezze e situazioni con le soglie, spread, regole UE, MES. | sempre |
| Bilancio | Il passaggio del bilancio: entrate, spesa dei dieci dicasteri, saldo contro il tetto UE, emendamenti degli alleati. Qui il posto del piano strutturale riveduto. | in ottobre-dicembre, al governo |
| Paese | Mappa con livelli, segni e scheda della regione a lato; sondaggio del trimestre con gli incroci. | sempre |
| Partiti | Asse coi partiti e i leader, alleati con intesa e cause, parlamento in carica, casse. | sempre |
| Campagna | Coalizione da dichiarare, bersagli, promesse, riposizionamento, casse dei rivali. | nel turno prima del voto, turno zero compreso |
| Registro | Turni passati, voci di ciascuno, «torna al turno N». | dal turno 1 |

Dati che la spec (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:398`) lasciava senza schermata e che ora hanno un posto: sondaggio con incroci (storia 111) in Paese; grandezze vicine alle soglie e situazioni (121, 132) in Conti; partiti sull'asse, casse, intesa con le cause (100, 129, 83) in Partiti; elenco delle disposizioni (58) in Aula.

**Acquisti d'informazione** (`Vista.acquisti`): ciascuno dove si legge il suo risultato. Test di una misura nell'anteprima, in Aula; centro studi in Conti; sondaggio privato in Paese e in Campagna. Nessuna pagina «Informazione».

**Evento e situazioni**: l'evento non ha una pagina sua, ha il riquadro in testa al cruscotto e la notizia in prima pagina. Le situazioni stanno in Conti.

### Cosa cambia con lo stato

L'ossatura è la stessa; cambiano la sezione di casa, le sezioni barrate e i pulsanti di chiusura.

| Stato | Casa | Barrate, oltre a Bilancio e Campagna fuori dai loro turni | Chiusura |
|---|---|---|---|
| Al governo | Cruscotto | nessuna | «Chiudi il turno», con accanto quante scelte vanno al default |
| Crisi | Cruscotto, con «allarga o dimettiti» al posto dell'evento; banda nera sotto la cornice («solo ordinaria amministrazione»); borsino senza atti | nessuna | «Chiudi il turno» |
| Opposizione | Cruscotto: programma del governo avversario con la linea del giocatore misura per misura, le leve (sfiducia, posto riservato, referendum), intesa degli alleati avversari in fasce | Governo | «Avanti fino a novità» sopra «Chiudi il turno» |
| Trattativa | «Trattativa»: spartizione, richieste degli alleati, incarico | Aula; Governo se il giocatore non guida il governo uscente | «Chiudi il turno» |
| Turno zero | Cruscotto con le scelte aperte (coalizione, campagna) | Prima pagina, Governo, Aula, Registro | «Chiudi il turno» |
| Partita finita | «Eredità»; il Registro resta acceso per «torna a un turno» finché l'eredità è in bozza | Governo, Aula | nessuna |

### Tono grave

Finché un evento in tono grave è aperto, **tutto il turno è grave, in ogni schermata**: spariscono testata, corsivo, motti, vignette e colori d'accento, nessun rimbalzo. Una regola sola, quella dell'interruttore del prototipo vincolante. Il segnale è `VoceEvento.tono` (`app/src/lib/tipi/VoceEvento.ts`). Rimando al piano di core «esito e Vista completa»: se una voce dell'esito porta il suo tono, la prima pagina di quel turno è grave anche a evento chiuso; la UI non lo deduce.

### Nessuna finestra modale (storia 19)

- «Chiudi il turno» parte subito, senza conferma (storia 18); il conto delle scelte al default sta accanto al pulsante e porta al cruscotto. Dopo la chiusura si apre la prima pagina del turno nuovo.
- Conferme solo per ciò che distrugge (riavvolgimento, abbandono, conferma dell'eredità), **sul posto**: il pulsante diventa due pulsanti.
- I dettagli si aprono a lato dentro la sezione, mai sopra: carta del ministro, anteprima della misura, scheda della regione. Il selettore di regione sta dove c'è la decisione che lo chiede.

### Fuori dalla partita

- All'avvio l'app apre sempre l'**edicola**: partite in corso con l'ultima giocata in testa, nuova partita, albo.
- Nuova partita → scenario di base, oppure editor → fondazione → turno zero. Sono pagine senza cornice, con «‹ Edicola».
- «Riprendi» apre la prima pagina del turno, già tutta rivelata.
- L'abbandono sta sulla scheda della partita nell'edicola, non dentro il turno.

### Elenco definitivo delle schermate da prototipare

Diciotto, un ticket ciascuna, tutti dietro [Viste campione](06-viste-campione.md) e [Fondamenta visive](08-fondamenta-visive.md). La cornice col sommario non ha un prototipo suo: è nelle fondamenta visive.

1. [Edicola](13-edicola.md)
2. [Scenario ed editor](14-scenario-ed-editor.md)
3. [Fondazione](15-fondazione.md)
4. [Prima pagina](16-prima-pagina.md)
5. [Cruscotto al governo](17-cruscotto-al-governo.md)
6. [Governo](18-governo.md)
7. [Aula e calendario](19-aula-e-calendario.md)
8. [Anteprima della misura](20-anteprima-della-misura.md)
9. [Conti](21-conti.md)
10. [Bilancio](22-bilancio.md)
11. [Paese](23-paese.md)
12. [Partiti](24-partiti.md)
13. [Registro](25-registro.md)
14. [Campagna](26-campagna.md)
15. [Trattativa e spartizione](27-trattativa-e-spartizione.md)
16. [Cruscotto all'opposizione](28-cruscotto-all-opposizione.md)
17. [Schema dello Stato](29-schema-dello-stato.md)
18. [Eredità](30-eredita.md)

Rispetto all'elenco della spec: «proposte» ed «eventi» entrano nel cruscotto; il «listino-borsino» è Governo; la «vista dell'opposizione» è il cruscotto all'opposizione; «partite in corso e albo» è l'edicola; Conti e Partiti sono nuove.

### Scartato

Oltre alle due ossature: prima pagina con la sola testata o con gli indicatori dentro la testata; tono grave solo dov'è l'evento, o solo su cornice e prima pagina; riapertura dell'ultima partita all'avvio; sette sezioni col borsino sul cruscotto, o sei con Partiti dentro Paese; sezioni tolte invece che barrate; opzioni dell'evento in prima pagina; pagina unica degli acquisti; Governo che mostra il governo avversario; campagna dentro Partiti; passaggio del bilancio dentro Conti; casa sempre «Cruscotto».

Termini nuovi nel glossario (`CONTEXT.md`, «Interfaccia»): cornice, sommario, sezione, cruscotto, prima pagina, edicola.
