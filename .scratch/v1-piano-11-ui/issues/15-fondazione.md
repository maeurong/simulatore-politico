Type: prototype
Status: resolved
Blocked by: 06, 08, 12
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la fondazione del partito? Nome e posto su uno dei sei mezzi punti dell'asse (storia 8); per ogni posto l'istogramma dell'elettorato, il risultato atteso e le coalizioni possibili con la loro disponibilità (storia 9). Si nutre di `riepilogo_scenario` o di `anteprima_fondazione`: lo decide [Contratto del guscio](12-contratto-del-guscio.md). Pagina senza cornice, fra l'editor e il turno zero.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

Dall'11/11/2026 ([Contratto del guscio](12-contratto-del-guscio.md)): la fondazione si nutre di `anteprima_fondazione` (istogramma, risultato atteso, disponibilità), sullo stesso seme della partita che nasce; `riepilogo_scenario` resta all'editor.

## Answer

Deciso con Mario l'11/11/2026 in un giro su un prototipo navigabile: [15-fondazione.prototype.html](../prototipi/15-fondazione.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati da `genera/` ([viste/15-fondazione.json](../viste/15-fondazione.json)): `Partita::anteprima_fondazione` vera (`crates/sim-core/src/legislatura/fondazione.rs:142`) sull'Italia di oggi coi semi 4, 1 e 9 e sull'elettorato a destra, circa 43 ms ciascuna in release.

### Forma: A · Banco

Pagina senza cornice, come edicola ed editor: «‹ Scenario», «Il Pungolo · Nuova partita · il partito» e, a destra, lo scenario («Italia di oggi» o il nome del su misura). Sotto, le dodici colonne col **dettaglio a lato**.

- **Il nome**: un campo solo, al massimo 40 caratteri col contatore (il limite di `fonda`, `fondazione.rs:42`).
- **Il posto sull'asse** (storie 8 e 9): l'istogramma dell'elettorato sui sette punti, coi partiti dello scenario sotto, e sei bottoni sui mezzi punti con quota attesa e posto; il posto scelto è in rosso, i due punti accanto in nero. Sotto, una tabella dei sei posti: «fra X e Y», quota attesa con una barra, alleati disponibili. Asse e tabella scelgono la stessa cosa.
- **Dettaglio a lato** del posto scelto: la quota attesa in grande, «dei votanti, atteso al turno zero: media degli istituti del sondaggio», e una riga fissa: «È un sondaggio: il voto può dire altro.» Poi la quota di elettori sui due punti accanto e «Con chi correre»: i sette partiti, prima i disponibili, ciascuno con la causa del core.
- **Fonda** in fondo al dettaglio: nome e posto, «Si vota alla chiusura del turno zero: il primo turno è la campagna», «Fonda il partito ›». Diretto, senza conferma: non si perde nulla, lo scenario resta e l'abbandono sta nell'edicola.

### Casi e assunzioni dichiarate

- **Nessun posto scelto all'apertura**: il posto è la prima decisione strategica (storia 8). Finché mancano nome o posto, il bottone è spento e dice cosa manca.
- **Nome**: `maxlength` del campo a 40, quindi il rifiuto del core sulla lunghezza non si vede; il nome di soli spazi conta come vuoto, come in `fonda`.
- **Rifiuto di `fonda_partita`** (posto non ammesso, scenario cambiato fra anteprima e fondazione): sul posto sopra il bottone; guasto nella cornice ([Contratto del guscio](12-contratto-del-guscio.md)).
- **«‹ Scenario»** torna all'editor con lo scenario com'era; nome e posto restano in memoria finché la UI è aperta. Tornare e cambiare scenario chiama di nuovo `anteprima_fondazione`, quindi un seme nuovo.
- **Radiogroup** con le frecce del sistema sui sei posti; i numeri col formato `Intl` di `t()` ([Testi esterni](03-testi-esterni.md)).

### Guscio e dati

- Solo `anteprima_fondazione` e `fonda_partita`, già assegnati a 11a ([Contratto del guscio](12-contratto-del-guscio.md)), con lo stesso seme tenuto dal guscio. Nessun dato nuovo, nessun tocco al core.
- **Coalizioni possibili = l'elenco dei disponibili con la causa**: niente quota della coalizione né seggi, che l'anteprima non porta. Si scoprono al turno zero.
- **Rumore**: l'anteprima porta solo la media degli istituti; da seme a seme la quota a 1,5 va dal 10,9% al 14,5% e al posto medio dal 19,2% al 23,3%. La nota lo dice senza inventare un margine.

### Scartato

B · Manifesto (asse a tutta pagina, sei colonne sopra, cause solo al passaggio del mouse); C · Tabella (sei righe con mini-istogramma e sette pallini); la quota della coalizione o un margine del sondaggio chiesti al core; una conferma sul posto prima di fondare.

### Fuori mappa

La causa del Centro da 3,5 e da 4,5 dice «posizione troppo lontana dalla tua» a mezzo punto di distanza: `Distanza` è il ripiego quando nessun termine spinge contro (`crates/sim-core/src/legislatura/coalizione.rs:326`). È un difetto del testo del core, non della UI.
