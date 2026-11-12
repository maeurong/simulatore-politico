Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatto lo schema dello Stato, la vista secondaria a pagina intera (storia 157)? Chi tocca cosa nel turno, come rete con le note a biro; i tondi numerati che rimandano alle voci della prima pagina; da quali campi della Vista si compone (`ministeri`, `calendario`, `referendum`, `alleati`, `partiti`) senza chiederne di nuovi; cosa mostra all'opposizione; il tono grave. Si apre dalla riga del sommario, fuori dalle dieci sezioni.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [29-schema.prototype.html](../prototipi/29-schema.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: quattro Viste già giocate, ridotte ai campi che lo schema legge: la 04 (al governo, turno 4), la 08 (tono grave), la 01 (turno zero) e lo stato «fine» di [viste/28-opposizione.json](../viste/28-opposizione.json) (all'opposizione, turno 19). Nessuno stato nuovo in `genera/`. Le voci della prima pagina sono quelle scritte a mano nel prototipo della [Prima pagina](16-prima-pagina.md), che ha giocato gli stessi turni; quelle del tono grave le ho scritte allo stesso modo, dalla Vista prima e dopo la chiusura del turno 6 (`viste/25-registro.json`).

### A cosa serve

Mario: lo schema risponde prima di tutto a **chi muove cosa ora**, cioè alle pressioni del trimestre: chi chiede soldi, chi minaccia, cosa va in aula, cosa arriva dal Paese. Non serve a spiegare com'è fatto lo Stato, e nemmeno a risalire da una notizia alla sua causa.

### Forma: B · Tavola per poteri

Sotto la cornice, col dettaglio a lato.

- **Quattro colonne fisse**: Partiti, Parlamento, Governo (prima il Presidente del Consiglio, poi i dieci dicasteri), Paese e mercati (mercati, regioni, situazioni accese, grandezze). Un soggetto per riga, col nome e una riga sotto: ministro e partito, seggi, «regione», «grandezza».
- **Un filo** va da riga a riga e dice chi tocca chi nel trimestre. È blu, rosso quando l'effetto è contrario o a rischio, tratteggiato quando arriva più avanti, a puntini quando è un cantiere in corso. Porta solo il tondo; cosa dice si legge nel dettaglio a lato, al clic sul filo o sulla riga.
- **I tondi** portano il numero della voce nell'elenco «Le notizie del trimestre» del dettaglio a lato. Un clic sulla voce accende i suoi fili.
- **In testa**: «Tavola n. N · Come funziona lo Stato», il trimestre, e i due bottoni «Tutti i fili · N» e «Solo quelli in prima pagina».

### Decisioni sui casi

- **All'apertura si vedono tutti i fili** (27 al governo nella Vista 04, 20 all'opposizione al turno 19), e c'è il filtro «Solo quelli in prima pagina».
- **Battute a biro sì**, una per voce, solo sui fili in prima pagina, con le regole dei [Testi del Pungolo](09-testi-del-pungolo.md); in B stanno nel dettaglio del filo. Nel prototipo ce n'è una bozza per tipo di filo: le scrive la fetta, con lo stesso agente e lo stesso taglio di Mario dei titoli. Nel tono grave tacciono.
- **Al turno zero lo schema si apre**: governo uscente coi suoi ministri, Camere sciolte a tratteggio, «si vota a fine turno». Niente tondi, perché non c'è una prima pagina.
- **All'opposizione** le stesse colonne: il Presidente del Consiglio porta «guida: …», gli alleati del governo vanno al Presidente con la fascia d'intesa, il posto riservato libero è un filo dal tuo partito alla Camera. Le agende dei ministri avversari non si vedono, come nel [Cruscotto all'opposizione](28-cruscotto-all-opposizione.md).
- **Tono grave**: le regole delle fondamenta, tutto a inchiostro, senza battute.

### Note di Mario ancora aperte

Su B, Mario ha detto che «non si capisce e non ci si riesce bene ad orientare», per tre motivi: i **fili muti**, che per capire cosa sono vanno cliccati; il **groviglio sul Presidente**, dove finiscono quasi tutti i fili e quelli interni alla colonna Governo si accavallano; il **verso non chiaro**, perché non si vede chi agisce e chi subisce. La forma D, nata per rispondere a queste note, non lo ha convinto: le tre note vanno risolte **dentro B**, nella fetta che porta lo schema.

### I fili e da dove vengono

Nessun campo nuovo: ogni filo viene da un campo che la Vista porta già. Quale campo diventa quale filo lo decide una tabella della UI, scritta nella fetta.

| Filo | Da → a | Campo della Vista |
|---|---|---|
| maggioranza | Presidente → Camera, Senato | `seggi`, `partiti[].al_governo` |
| agenda | dicastero → grandezza, o regione | `ministeri[].agenda`, `atti[].effetti` |
| cantiere | dicastero → grandezza, o regione | `ministeri[].cantieri` |
| proposta in sospeso | dicastero → Presidente | `ministeri[].proposta` |
| minaccia di dimissioni | dicastero → Presidente | `ministeri[].ministro.minaccia` |
| intesa | alleato → Presidente | `alleati`, `alleati_avversari` |
| misura in aula | promotore, dicastero o governo → Camera | `calendario` |
| legge del trimestre chiuso | Camera → Senato | `esiti` col trimestre precedente |
| posto riservato | il tuo partito → Camera | `posto_riservato` |
| evento | regione o situazione → Presidente | `eventi` |
| situazione accesa | situazione → Presidente | `situazioni` |
| spread | mercati → Economia | `spread_pb` |

- **I tondi vogliono l'esito**: senza la PR 1 del piano di core «esito e Vista completa» non ci sono voci da numerare. Lo schema entra in 11e coi fili, come da [Fette](10-fette.md); i tondi e l'elenco delle notizie arrivano in 11f, insieme alla prima pagina.
- **Chi guida il governo** fuori dal governo è il partito di governo senza fascia d'intesa in `alleati_avversari`. Al turno zero la Vista non lo dice; il prototipo prende il partito con più ministri. Nessun rimando: la fetta scrive i partiti del governo uscente.
- **Nessun passacarte nuovo.**

### Glossario

In `CONTEXT.md`: schema dello Stato, filo.

### Stranezze notate

- Nella Vista 04 sei ministri su dieci hanno una proposta in sospeso perché il margine è esaurito: sono 1,8 mld di richieste al Presidente nello stesso turno.
- «Caro energia» resta accesa a 6,97 con la soglia di accensione a 10: è l'isteresi (si spegne sotto 5), non un errore. La Vista però non dice quale dicastero muove la situazione: il filo va al Presidente.

### Scartato

A · Tavola a rete (la rete del prototipo vincolante, troppo affollata con 20-27 fili, anche sfoltita); C · Indice dei fili (elenco con la rete piccola a lato); D · Mappa delle forze (sette blocchi con i fili raggruppati in frecce scritte e col verso, nata dal giro: non ha convinto); l'apertura sui soli fili in prima pagina; lo schema barrato al turno zero.
