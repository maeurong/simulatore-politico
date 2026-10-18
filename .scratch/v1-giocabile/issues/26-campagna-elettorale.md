Type: grilling
Status: resolved
Blocked by: 25
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa fa il giocatore nel turno prima del voto, oltre a dichiarare la coalizione (05, 20)? Quali leve di campagna ha (temi, piazza, media, promesse, riposizionamento del partito sull'asse), cosa costano in cassa del partito (25) e in valence, visto che le azioni di governo restano senza moneta (09), e come muovono valence e posizioni dei gruppi (06)? Vale lo stesso al governo, all'opposizione (12) e al turno zero, quando il partito è ancora fuori dal parlamento (20)?

Vincoli già fissati:
- il riposizionamento del giocatore è simmetrico alla regola di movimento dei partiti IA: mezzo punto al massimo, senza scavalcare il vicino (18);
- nel turno prima del voto si muovono anche i partiti IA e nascono i partiti di protesta (18): va deciso cosa ne vede il giocatore e in che ordine;
- la cassa del partito esiste (25): entra da 2 per mille, donazioni e dote, non va sotto zero, e la campagna è il suo secondo uso dopo l'informazione; i partiti IA hanno la stessa cassa e la spendono solo qui;
- il sondaggio privato resta disponibile nel turno del voto, e il silenzio elettorale non conta a turni trimestrali (25).

Resta da decidere anche la campagna dei partiti IA: se fanno qualcosa oltre a muoversi sull'asse.

## Answer

Grilling con Mario del 18/10/2026, tre giri. Raccomandazioni accettate tranne una scelta di Mario: la campagna **convince anche**, cioè sposta in modo permanente le posizioni degli elettori, oltre a mobilitare e persuadere. Quadro finale confermato. Il difetto da evitare è quello di Lawgivers II, dove la campagna sostituisce il governo invece di affiancarlo (`docs/ricerca/sistemi/campagna-elettorale.md:23`): la campagna sposta voti al margine e il governo resta il fattore principale (`:28`). Termini nuovi in `CONTEXT.md`: campagna elettorale, bersaglio, promessa; precisata posizione percepita.

**Quando.** Solo nel turno prima del voto: a scadenza, dopo uno scioglimento anticipato e al turno zero, con le stesse leve al governo e all'opposizione. Niente campagna permanente negli altri turni, niente campagna per i referendum (resta l'invito all'astensione, 12). La cassa si accumula per tutta la legislatura, quindi ogni acquisto di informazione (25) toglie qualcosa alla campagna.

**Ordine nel turno prima del voto**, dentro lo scheletro a tre fasi (09):
- *Apertura:* i partiti IA mostrano lo spostamento della loro posizione dichiarata (18) e nasce l'eventuale partito di protesta, ciascuno con la sua causa. Il sondaggio privato resta acquistabile (25).
- *Decisioni:* il giocatore si riposiziona, dichiara la coalizione con la disponibilità ricalcolata (05), sceglie fino a 2 promesse e spende in campagna su fino a 3 bersagli.
- *Chiusura:* si spendono le casse di tutti, gira il motore, si vota (09).
- *Turno zero:* niente spostamenti dei partiti IA e niente partito di protesta, perché lo scenario è la fotografia tarata sui sondaggi veri (20). Il giocatore spende la dote e può promettere; i partiti IA spendono la loro cassa.

**Riposizionamento.** È gratuito: mezzo punto al massimo, senza scavalcare il vicino (18). La posizione percepita fa **metà strada**, cioè si sposta di un quarto di punto nella stessa direzione. Spostarsi verso la percepita riduce il divario che porta alla scissione; spostarsi lontano lo allarga, e la scissione, che nasce subito (18), può scattare prima del voto. La stessa regola vale per i partiti IA e chiude un vuoto del 18.

**Bersagli e leve.** Fino a 3 bersagli per campagna. Ognuno è una condizione (6) oppure una regione (20), mai il singolo gruppo, e riceve una sola leva fra queste tre:
- **mobilitare:** l'astensione rende meno fra gli elettori del bersaglio vicini alla posizione del partito; vale per quel voto;
- **persuadere:** valence in più verso il bersaglio; vale per quel voto;
- **convincere:** le posizioni degli elettori del bersaglio si spostano verso la posizione dichiarata del partito. L'effetto è permanente, ma l'ancora di Friedkin-Johnsen (06) lo riassorbe piano lungo la legislatura. Conferma le «spinte di campagna» del 06 (`docs/ricerca/sintesi/decisioni.md:92`).

**Scala.** Rendimenti decrescenti per bersaglio (radice della spesa). Una campagna a cassa piena vale 1-3 punti di voto nazionali, meno di quanto sposti il giudizio sul governo (17). Convincere costa di più per punto, ma dura. I valori esatti vanno al bilanciamento.

**Promesse.** Fino a 2, ciascuna una disposizione portata a un valore del catalogo (16).
- Alla dichiarazione danno persuasione verso i gruppi che ne guadagnano, e costano intesa agli alleati la cui linea sarebbe no, come una misura lontana (05).
- Al voto successivo una promessa è mantenuta se la disposizione sta al valore promesso, chiunque l'abbia portata lì. Mantenuta dà poca valence, mancata ne toglie di più.
- Conta solo se il giocatore è stato Presidente del Consiglio per almeno 4 turni in quella legislatura.

**Partiti IA.** Stesse leve, regola in una riga nell'esito: ogni partito spende tutta la cassa sui 3 bersagli dove un euro gli rende più voti attesi, letti sul sondaggio pubblico (18). Di solito mobilita un bersaglio astenuto vicino e persuade uno conteso; solo i partiti a vocazione identitaria convincono. Come promesse prende le prime 2 disposizioni del suo programma, con la stessa verifica.

**Partiti nuovi.** Il partito di protesta riceve una dote come il partito del giocatore alla fondazione (20) e la spende subito. Una scissione porta via la quota della cassa del partito d'origine pari alla sua quota di seggi.

**Cosa vede il giocatore.** L'anteprima della campagna funziona come quella di una misura (25): costo esatto ed effetto in fasce (poco, medio, molto) per bersaglio, con la causa. Le casse dei rivali sono pubbliche in cifre. Quanto e dove hanno speso si vede nell'esito, una riga per partito; il dettaglio va nel registro. Tra la spesa e il voto non esce nessun sondaggio, quindi l'effetto della propria campagna si scopre solo dal risultato.

Effetti sulla mappa:
- Costo del governare (29): rendimento della campagna per leva, scala di 1-3 punti, premio e malus delle promesse, costo d'intesa delle promesse, taglia della dote del partito di protesta. Le promesse sono una prima forma di aspettative che crescono.
- Salvataggi (28): le promesse in sospeso e le spinte di posizione entrano nello stato della partita.
- Punti di aggancio (30): campagna per i referendum; salienza mobile dei temi come canale della campagna (06).
- Partiti IA (18): la regola della percepita che fa metà strada vale anche per loro.
