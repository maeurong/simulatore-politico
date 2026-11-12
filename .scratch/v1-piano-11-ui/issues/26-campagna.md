Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Campagna? La coalizione da dichiarare, con la disponibilità di ogni partito e la sua causa e i posti delle liste comuni da cedere (storie 77-78); fino a tre bersagli fra condizioni e regioni, ciascuno con la sua leva e l'anteprima a fasce (storia 127, `anteprima_campagna`); fino a due promesse su disposizioni (storia 128); il riposizionamento di mezzo punto col rischio di scissione (storia 126); le casse dei rivali (storia 129); il sondaggio privato; l'avviso della campagna tagliata quando la cassa non basta (dal piano di core). Vale al turno zero e nell'ultimo turno della legislatura, al governo e all'opposizione; negli altri turni è barrata.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [26-campagna.prototype.html](../prototipi/26-campagna.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [viste/26-campagna.json](../viste/26-campagna.json), che `genera/` aggiunge giocando: il turno zero del seme 4 a 4,5 con ogni coalizione dichiarabile provata col comando (128 sottoinsiemi), le cessioni, le anteprime di 26 bersagli a sette importi, i riposizionamenti a passi di un quarto, le promesse, il sondaggio privato e la campagna tagliata; l'ultimo turno della legislatura al governo (seme 4, turno 20) e all'opposizione (seme 1 a 1,5, turno 20); la cessione che convince (seme 2 a 2,5: la Sinistra entra con 30 seggi). Ogni disponibilità, fascia e rifiuto del prototipo è quello vero del core.

### Forma: A · Tavolo

- **Tre colonne a pagina:** Coalizione, Spesa, Posizione e promesse. Nel **dettaglio a lato** la cassa: il conto (cassa, sondaggio, spese, resto) con l'avviso della campagna tagliata, il sondaggio privato e le casse dei rivali. Quando si sceglie una regione bersaglio il lato diventa il selettore deciso nel [Paese](23-paese.md): mappa piccola tinta con la tua quota, lista da tastiera, «Spendi in …».
- **Coalizione (storie 77-78):** una riga per partito, disponibile o no con la causa. «Aggiungi» e «Togli» mandano subito `DichiaraCoalizione`, che si sostituisce: nessuna conferma, il rifiuto del core sta sulla riga (aggiungere il Centrosinistra a Centrodestra e Destra: «Destra non entra: posizione troppo lontana dalla tua»). Sotto ogni partito non disponibile, i seggi delle liste comuni da cedere a passi: ogni passo è una dichiarazione, col rifiuto o l'ingresso del core; accanto, i tuoi seggi attesi sul sondaggio.
- **Spesa (storia 127):** fino a tre riquadri. Bersaglio fra le sei condizioni (pulsanti) e le venti regioni (selettore), leva a tre (mobilita, persuade, convince, col suo effetto al passaggio), importo, la fascia dell'anteprima in tre caselle con la causa del core, e la tua quota fra i votanti del bersaglio, pubblica e privata se comprata. Oltre la cassa, il rifiuto del core sul posto. Nel gioco l'importo è libero e `anteprima_campagna` gira a ogni modifica.
- **Posizione (storia 126):** l'asse da 1 a 7 con dichiarata, percepita e gli altri partiti; quattro passi da −0,5 a +0,5; lo spostamento in blu dall'anteprima del core, con la distanza contro le soglie di minaccia e di scissione. Conferma sul posto: si fa una volta.
- **Promesse (storia 128):** fino a due, disposizione e gradino, il vigente segnato e rifiutato col testo del core. Al governo, prima del sì, l'intesa che ogni alleato contrario perde, dall'anteprima del core. Conferma sul posto: si fa una volta.
- **Rivali (storia 129):** le casse in cifre a barre, coi colori dei partiti del Paese, e le promesse in sospeso. Quanto e dove hanno speso lo dice l'esito del voto (`spesa_di_campagna`).
- **Sondaggio privato:** si compra dal lato; le quote private stanno accanto alle pubbliche in ogni bersaglio.
- **Barrata** fuori dal turno del voto, col motivo, come nell'ossatura.

### Decisioni sui casi

- **Riposizionamento con l'anteprima del core** (scelta di Mario): dichiarata, percepita e distanza dopo lo spostamento, e le soglie nella Vista. All'opposizione (seme 1, turno 20) la distanza è 0,61, minaccia accesa: spostarsi a 2 la porta a 0,36, a 1 a 0,86.
- **Promesse al governo con l'anteprima del core** (scelta di Mario): quali alleati sono contrari e quanta intesa perdono. Al seme 4, turno 20, reddito minimo universale ed età pensionabile a 64 anni costano 10 punti al Centrodestra, già in ultimatum, e 10 alla Destra.
- **L'avviso della campagna tagliata lo scrive la UI** (scelta di Mario), come il funzionamento tagliato del [Bilancio](22-bilancio.md): `Campagna` si valida contro la cassa del momento, poi un acquisto la abbassa e la chiusura taglia le spese in ordine. Confrontando spese e cassa la UI dice quale spesa scende o cade e avverte che una scissione in chiusura porterebbe via altra cassa; nell'esito resta la voce `campagna_tagliata` del core.
- **Dichiarazione al clic** (scelta mia): il comando si sostituisce, quindi niente bozza in memoria e niente conferma; la disponibilità che la Vista rende dopo il comando è quella della coalizione appena dichiarata.
- **Cessioni senza tetto nella Vista** (scelta mia): il massimo sono i tuoi seggi attesi, che il core calcola diversi dalla proiezione pubblica (72 contro 80 al seme 4); il rifiuto sul posto lo dice, e basta.
- **Promesse su tutto il catalogo** (scelta mia): `prometti` accetta ogni disposizione, anche territoriale o costituzionale.

### Rimandi

Due al core, PR 2 della spec «esito e Vista completa», Task 16e del piano (scritti in spec e piano il 12/11/2026):

- **`anteprima_riposizionamento(posizione)`** e, in `VistaCampagna`, `riposizionato`, `passo` e le due soglie di scissione.
- **`anteprima_promesse(promesse)`**: una voce per alleato contrario, con l'intesa persa e le disposizioni.

### Guscio e dati

`anteprima_campagna` è già assegnato a 11b ([Contratto del guscio](12-contratto-del-guscio.md)). I due passacarte nuovi, `anteprima_riposizionamento` e `anteprima_promesse`, arrivano con la PR delle esposizioni, quindi in 11g come la minaccia di scissione ([Fette](10-fette.md)); fino ad allora la sezione mostra dichiarata e percepita di oggi e conferma sul posto senza anteprima.

### Stranezze notate

- Al seme 4, turno 20, al governo, il giocatore non può riposizionarsi: il Centro sta a 4,5 e il Centrodestra a 4,500000000000001, e ogni passo ne scavalca uno.
- Allo stesso turno Centrodestra e Destra sono in coalizione dall'ultimo voto ma «non disponibili»: senza dichiarazione la coalizione resta, ma tolti non tornano.
- I partiti IA spendono tutta la cassa: al turno zero del seme 4, 22,1 mln € in sette.
- La causa di un'anteprima non dipende dalla leva: la forza della leva entra nelle spinte, non nella fascia.

### Lessico

Nessun termine nuovo in `CONTEXT.md`: bersaglio e promessa ci sono già; riposizionamento e liste comuni sono le parole della spec v1 (storie 78 e 126) e dei comandi, senza ambiguità da sciogliere.

### Scartato

B · Cassa ripartita (la cassa contro i rivali in testa e i 26 bersagli in tabella: la spesa diventa la sola cosa a pagina, e coalizione, posizione e promesse finiscono in linguette); C · Ordine del giorno (le righe col default come nel cruscotto: una scelta alla volta, senza vedere insieme alleanza, spesa e rischio).
