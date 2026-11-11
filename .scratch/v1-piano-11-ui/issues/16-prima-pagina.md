Type: prototype
Status: resolved
Blocked by: 06, 08, esterno: spec del piano di core «esito e Vista completa»
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la prima pagina di «Il Pungolo», cioè l'esito del turno? La striscia della cornice sopra la testata grande; la voce d'apertura e fino a sette voci in evidenza, prima le critiche (storia 15); la rivelazione una voce alla volta nell'ordine della chiusura (storia 14), con la velocità e il salto; occhiello serio e titolo spiritoso (storia 153), vignetta; i tracciati a caselle che avanzano mentre l'esito si rivela; la notizia dell'evento aperto, senza opzioni; «Vai al cruscotto ›» e «Altre N notizie nel registro ›». Varianti: tono grave, la pagina dopo una fermata di «avanti fino a novità» col motivo della fermata, la pagina del risultato del voto, la pagina già rivelata che apre «Riprendi».

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

Dall'11/11/2026 ([Testi del Pungolo](09-testi-del-pungolo.md)): il titolo è senza nomi e scelto per tipo di voce; una voce senza titolo mostra il solo occhiello, come nel tono grave; il corsivo è tagliato dalla v1. La didascalia della vignetta, se c'è, si decide qui con le stesse regole di scrittura.

## Answer

Deciso con Mario l'11/11/2026 in un giro su un prototipo navigabile: [16-prima-pagina.prototype.html](../prototipi/16-prima-pagina.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: le Viste 02, 03, 04 e [viste/16-prima-pagina.json](../viste/16-prima-pagina.json), che `genera/` aggiunge giocando gli stati che mancavano: il turno 3 del seme 4 (fra 03 e 04), il voto del seme 9 col suo turno zero, il voto col giocatore all'opposizione, e le corse di «avanti fino a novità» dal 06 in poi, con le fermate calcolate secondo la spec di core (senza la fermata `voto`). La Vista di oggi non porta l'esito: le voci del prototipo sono scritte a mano col catalogo della spec di core «esito e Vista completa», leggendo i fatti nelle Viste di prima e di dopo ogni chiusura. I titoli sono una bozza.

### Forma: A · Giornale composto

Sotto la cornice, che resta come striscia, la pagina delle [Fondamenta visive](08-fondamenta-visive.md).

- **Testata**: a sinistra «Esito del trimestre», il trimestre chiuso, quante notizie in pagina e quante nel registro; al centro «Il Pungolo»; a destra numero, turno e trimestre nuovo. Sotto, la riga col sottotitolo.
- **Voce di testa**: la voce in evidenza con la criticità e la precedenza più alte; a parità, la prima nell'ordine della chiusura. Occhiello col fatto, titolo a 44 px, causa, righe delle voci fuse, rimando alla sezione. A lato la vignetta. Nella pagina del voto, al posto della vignetta, il risultato a barre: affluenza, quota e seggi di Camera e Senato per partito, la coalizione col premio in nero, il giocatore in rosso.
- **Le altre voci in evidenza** in tre colonne, nell'ordine della chiusura, con le stesse parti. Una critica ha il quadratino rosso davanti all'occhiello. L'evento nato non mostra opzioni: «Si decide sul cruscotto ›», e all'opposizione «Lo decide il governo».
- **Piede**: «Altre N notizie nel registro ›», con «, K critiche» quando qualche critica resta fuori dalle sette (la UI conta le critiche non in evidenza, non sceglie); «Salta la rivelazione ››» mentre si rivela; «Vai al cruscotto ›».
- **Rivelazione** (storia 14): una voce ogni 1,1 s moltiplicati per il fattore `k` della velocità, nell'ordine della chiusura, ognuna al suo posto nella pagina, anche quando salta dalle colonne alla testa. Un clic sulla pagina salta tutto; a velocità istantanea la pagina è già composta. «Riprendi» la apre già rivelata e ferma.
- **Tracciati che scattano**: partono dai valori della Vista che la UI aveva prima di chiudere e scattano quando la rivelazione supera la voce che li muove (sondaggio, spread, risultato del voto), anche se quella voce non è in evidenza. Le caselle che cambiano lampeggiano in giallo; accanto al valore resta «era X». Con «Riprendi» non c'è una Vista di prima e i tracciati sono già fermi.
- **Dopo una fermata** di «avanti fino a novità», una banda sotto la cornice: quante chiusure di fila, perché si è fermata (una frase per fermata), e il rimando al registro dei turni le cui pagine sono state saltate.
- **Tono grave**: testata «Esito del trimestre», niente titoli (l'occhiello sale a titolo e il quadratino della critica passa al titolo), niente vignetta, come in [Testi del Pungolo](09-testi-del-pungolo.md).

### Decisioni sui casi

- **Vignetta da un kit per tipo** (scelta di Mario): un disegno per ciascuno dei tipi che più spesso vanno in testa e una didascalia per tipo, con le regole dei titoli di [Testi del Pungolo](09-testi-del-pungolo.md) (niente nomi né cifre, il bersaglio è il palazzo). Una voce di testa di un tipo senza disegno va a tutta larghezza. Disegni e didascalie si fanno nella fetta che porta la prima pagina, 11f di [Fette](10-fette.md): i disegni con lo stesso metodo del kit delle [Caricature](05-caricature.md), le didascalie dallo stesso agente che scrive i titoli e con lo stesso taglio di Mario. Quali sono i tipi frequenti lo dicono le partite automatiche della CLI, come per i titoli.
- **Voce di testa per criticità e precedenza** (scelta mia, mostrata nel giro): con la prima voce nell'ordine della chiusura, la pagina del voto si aprirebbe con le spese di campagna. È un ordinamento sui campi della voce, non una regola del gioco.
- **Fermate come sono** (scelta di Mario). All'opposizione «avanti fino a novità» si ferma quasi a ogni turno: nelle corse giocate dal 06 (seme 1), 2→3, 3→4, 4→5, 5→7, 7→9, 9→11, 11→13, 13→14, 14→15, 15→16, 16→19, 19→20, 20→21, 21→23. Fermano soprattutto il posto riservato del giocatore libero e la finestra del referendum di luglio-settembre: sono decisioni vere, e la banda dice perché.
- **Grandi cali senza notizia** (scelta di Mario): nella 04 il consenso cala di 7,5 punti e il PIL esce a −3,3%, ma sondaggio e prima stima sono voci sempre ordinarie (spec di core, scelta 20) e vanno solo nel registro. Basta il tracciato che scatta durante la rivelazione.

### Rimandi

- **Una voce sola per una situazione accesa col suo evento — al core.** Quando nella stessa chiusura una situazione si accende e nasce il suo evento, `situazione_accesa` ed `evento_nato` dicevano la stessa cosa, tutte e due in evidenza (la recessione della corsa all'opposizione, turno 18). Mario sceglie il rimando: tace `situazione_accesa`. Il rimando è nella spec e nel piano dall'11/11/2026 (branch `docs/piano-esito-vista-completa`, `d5982ff`): sezione «Situazione accesa ed evento nato», test nel Task 7.

### Glossario

In `CONTEXT.md`: voce di testa (non «apertura», che è la fase del turno), vignetta.

### Guscio e dati

Nessun dato nuovo oltre al rimando. L'esito, `in_evidenza`, la criticità, la precedenza e le fermate li porta la PR 1 del piano di core; il registro dei turni saltati lo legge `registro`, già assegnato a 11f dal [Contratto del guscio](12-contratto-del-guscio.md). Quale tracciato muove ciascun tipo di voce è una tabella della UI, scritta nella fetta: dice dove scatta, i valori vengono dalla Vista.

### Stranezze notate

- Nelle partite giocate il fondo di riserva è quasi sempre a zero, e l'evento risolto per default dice «coperti 0 € su 1 mld» in quasi ogni turno (già notato in [Cruscotto al governo](17-cruscotto-al-governo.md)).
- Nella corsa all'opposizione una proposta della Lista dei Territori passa alla Camera (203 a 197) e cade al Senato a 100 pari.
- Nel voto del seme 9 lo spread sale di 29 punti nel turno del voto (da 84 a 113).

### Scartato

B · Cronaca (una colonna nell'ordine della chiusura col dettaglio a lato: più ordine del giorno che giornale); C · Una alla volta, poi la pagina (una carta per voce prima della pagina: la storia 14 alla lettera, ma costa tempo a ogni turno); la vignetta per famiglia e la sola pagina del voto col risultato; le fermate spegnibili dalle impostazioni o rivedute dal core; due voci per la situazione accesa; sondaggio e prima stima che salgono in pagina oltre una soglia.
