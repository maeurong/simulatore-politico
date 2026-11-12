Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Paese? La mappa di [Mappa SVG delle regioni](02-mappa-svg-delle-regioni.md) con un livello alla volta (storie 137-138), i segni di atti, cantieri ed emergenze (storia 139), la lista delle venti regioni accanto; a lato la scheda della regione con i diciotto gruppi, gli elettori campione al clic (storie 113 e 140), seggi, forbice del sondaggio e disposizioni territoriali; il sondaggio del trimestre con gli istituti, gli incroci per condizione, fascia e regione e la proiezione dei seggi (storia 111); il sondaggio privato comprato, accanto a quello pubblico (storia 123). Qui si fissa anche come appare il selettore di regione dentro le altre sezioni, quando una decisione lo chiede (storia 141).

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 su un prototipo navigabile: [23-paese.prototype.html](../prototipi/23-paese.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso, e «dove» mostra il selettore dentro tre sezioni ospiti. Dati: [viste/23-paese.json](../viste/23-paese.json), che `genera/` aggiunge giocando: il governo del 03 e l'opposizione del 06 col sondaggio privato comprato davvero (e il secondo acquisto rifiutato), il turno zero, il tono grave dello 08, e per ognuno le venti regioni calcolate con le regole della spec di core «esito e Vista completa» (disoccupazione della chiusura precedente, reddito reale, pericolosità relativa, i diciotto gruppi) più i tre elettori campione di tutti i 360 gruppi. Il controllo della spec regge: la media delle disoccupazioni regionali pesata sulle forze di lavoro è la disoccupazione riveduta (5,693 contro 5,6925 nel 03).

### Forma: A · Atlante

- **A pagina la mappa e la lista.** In testa i livelli come linguette (Consenso, Disoccupazione, Reddito, Affluenza, Pericolosità, Risultato del voto), con accanto la scelta che il livello chiede: di chi è il consenso, che tipo di pericolosità. Sotto, la mappa a tutta altezza e accanto la lista delle venti regioni col valore del livello e i segni, ordinabile per nome o per valore; sotto la lista la legenda a cinque classi e quella dei segni. Un livello senza dati è barrato col motivo, come il sommario.
- **Il dettaglio a lato tiene il sondaggio** finché non si sceglie una regione: la media dei quattro istituti sui votanti con l'astensione a parte, il sondaggio privato, i seggi se si votasse oggi (Camera e Senato, la coalizione contro la maggioranza), gli incroci per condizione con la tua quota e il margine. Un incrocio scelto apre le sue quote con la forchetta, l'approvazione del governo e l'autocollocazione.
- **La scheda della regione** sostituisce il sondaggio al clic sulla mappa o sulla lista: macroarea, segni scritti per esteso, disoccupazione e reddito con un turno di ritardo, pericolosità a parole per tipo (alta da 2/3, media da 1/3), il sondaggio della regione a barre con la forchetta, i diciotto gruppi in una griglia condizione × fascia con la quota, i seggi del Senato e il primo partito dell'ultimo voto, le disposizioni territoriali in vigore con il gradino, quello in arrivo e il rimando all'Aula.
- **Elettori campione:** un clic su un gruppo della griglia mette sotto i suoi tre elettori (età, nome, voto o astensione, motivo, collocazione), con la riga che dice che sono inventati dal sondaggio.
- **Sondaggio privato:** il riquadro con «Compra · 15.000 €» e la cassa sta sotto la media; comprato, diventa una colonna accanto alla media, col suo margine, e gli incroci e la scheda della regione passano ai suoi numeri. «Compralo ancora» mostra il rifiuto del core sul posto.
- **Segni:** triangolo per il cantiere in corso, cerchio per l'atto in agenda, quadrato per l'opera finita, croce rossa per l'emergenza aperta; più segni nella stessa regione in fila.
- **Mappa come radiogroup:** una fermata di Tab, frecce per scorrere nell'ordine della lista, Invio o spazio per scegliere, gli anelli del fuoco e della scelta ricopiati in coda con `<use>`. La lista fa da equivalente per le regioni troppo piccole.
- **Tono grave:** le regole delle fondamenta; la rampa passa all'inchiostro.

### Decisioni sui casi

- **Consenso di default della coalizione del giocatore:** al governo il giocatore più gli alleati, all'opposizione il giocatore più la coalizione dichiarata; un menu sceglie un partito solo. Le quote sono sui votanti, sommate dalla UI dal sondaggio: è lettura di un dato pubblico, non stato nuovo.
- **Affluenza** per regione: uno meno l'astensione del sondaggio della regione.
- **Colori dei partiti:** otto, uno per partito, passati al validatore di `dataviz` sulla carta (adiacenti distinti anche per i daltonici). Mario sceglie di usarli nella mappa del voto e nelle barre (sondaggio, seggi, scheda della regione); il resto resta a inchiostro. Contrasto basso per tre di loro: c'è sempre il nome accanto.
- **Classi della mappa:** cinque, a passi uguali sul campo dei valori del turno (la pericolosità a passi fissi di 0,2), tinte interpolate in OKLab dalla carta a un rosso scuro, all'inchiostro nel tono grave.
- **Selettore di regione nelle altre sezioni (storia 141):** nel dettaglio a lato della sezione ospite, mappa piccola sopra e lista a due colonne sotto, poi il bottone d'azione; la sezione ospite resta dov'è. Dirottamento: la regione del ministro tinta e il motivo, il costo dal passacarte `costo_dirotta` (11g, [Contratto del guscio](12-contratto-del-guscio.md)). Bersaglio di campagna: la tua quota per regione come tinta, col campione e il margine della regione scelta. Perimetro di una disposizione: a tratteggio, da leggere e non da scegliere.

### Rimandi

Nessuno nuovo. Tutto ciò che la Vista di oggi non porta è già nella spec di core «esito e Vista completa», PR delle esposizioni, in 11g: `Vista.regioni` (disoccupazione, reddito, pericolosità, gruppi), `EsitoElezioni.regioni` (voti, seggi del Senato e primo partito per regione), `Vista.opere` e il margine del sondaggio. Il voto per regione `genera/` non lo rifà: `vota_parlamento` usa le intenzioni vere dentro la chiusura, e fuori si vedono solo il prima e il dopo; nel prototipo il livello è barrato col rimando. Le opere finite non arrivano per lo stesso motivo (gli atti a un turno della trattativa non si vedono). Gli elettori campione passano dal passacarte `elettori_campione` ([Contratto del guscio](12-contratto-del-guscio.md)).

### Guscio e dati

Nessun passacarte nuovo: `elettori_campione` e `costo_dirotta` sono già assegnati; l'acquisto passa da `comanda` (`Compra { SondaggioPrivato }`).

### Stranezze notate

- Nel totale della pericolosità il sisma pesa poco: l'Abruzzo ha terremoti a 1 e totale a 0,26. La più esposta è la Sicilia (incendi al tetto), poi la Sardegna senza terremoti.
- Il sondaggio privato ha 4.000 interviste in un istituto, il pubblico 4.501 sommate sulle regioni: per regione il privato può avere un campione più piccolo del pubblico (Piemonte 288 contro 324).
- Gli elettori campione leggono solo il sondaggio pubblico: dopo l'acquisto del privato restano gli stessi.
- Il consenso della coalizione di governo nel 03 va dal 37,6% (Basilicata) al 69,2% (Toscana).

### Scartato

B · Sondaggio a fronte (sondaggio a pagina e mappa più piccola: due cose da leggere insieme che si rubano lo spazio); C · Schede (mappa, sondaggio e incroci separati: la mappa non si legge accanto ai numeri); portare il giocatore nella sezione Paese per scegliere una regione.
