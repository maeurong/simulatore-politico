Type: prototype
Status: resolved
Blocked by: 06, 08, esterno: spec del piano di core «esito e Vista completa»
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Registro? L'elenco dei turni passati; le voci di un turno, ciascuna con la sua causa (storia 16), revisioni delle stime comprese (storia 119); «torna al turno N» con la conferma sul posto che dice cosa si cancella (storia 144); il registro a partita finita, finché l'eredità è in bozza (storia 150). Barrata al turno zero.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

Dall'11/11/2026 ([Testi del Pungolo](09-testi-del-pungolo.md)): il registro è neutro, fatto e causa, senza titoli.

## Answer

Deciso con Mario il 12/11/2026 in due giri su un prototipo navigabile: [25-registro.prototype.html](../prototipi/25-registro.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [viste/25-registro.json](../viste/25-registro.json), che `genera/` aggiunge giocando le chiusure 0-8 del seme 4 a 4,5 sul percorso del 05, con la Vista prima e dopo ognuna. La Vista di oggi non porta l'esito: le voci sono scritte a mano col catalogo della spec di core «esito e Vista completa», riprendendo quelle della [Prima pagina](16-prima-pagina.md) per le chiusure 1-3; le sette le sceglie il codice con la regola della spec. Lo stato «partita finita con l'eredità in bozza» è forzato e dichiarato tale: la sconfitta per seggi non arriva giocando.

### Forma: D · Indice col filo (l'Indice di A coi filtri di B)

- **Colonna dei turni** a sinistra, il più recente in alto: turno, trimestre chiuso, quante voci, il quadratino rosso se ci sono critiche, i segni del tono grave e del voto. In fondo il turno zero, che non ha voci ma si può riaprire.
- **A pagina le voci del turno scelto**, tutte, nell'ordine della chiusura: fatto e causa, senza titoli; le critiche col quadratino, quelle che erano in prima pagina segnate, le fuse con le righe dei dettagli, la sezione del rimando a destra.
- **Filtri sopra la pagina**, che valgono su tutti i turni: criticità, sezione, soggetto (partito, dicastero, regione, misura, disposizione). Con un filtro attivo la colonna tiene solo i turni con voci che combaciano, col loro numero, e la pagina mostra solo quelle (storia 16).
- **Dettaglio a lato:** la voce scelta coi dettagli, i soggetti e il rimando alla sezione; sotto, le voci collegate negli altri turni (stesso soggetto, anche dentro le fuse); per una revisione la prima stima che corregge, nel turno prima, e viceversa (storia 119).
- **«Torna al turno N»** in testa al registro del turno N. La conferma sul posto (il pulsante diventa due) dice cosa si cancella: questa chiusura e le K dopo, fino a quale trimestre, il turno in corso o l'eredità in bozza, il voto o il governo nato in mezzo; che il riavvolgimento si conta, e che con le stesse scelte il caso dà lo stesso esito (storia 144). A partita finita resta acceso finché l'eredità è in bozza (storia 150).
- **Tono grave:** le voci di tono grave portano il loro segno; nel turno grave la pagina segue le regole delle fondamenta.
- **Barrata al turno zero** col motivo, come nell'ossatura.

### Decisioni sui casi

- **I ministri del giocatore restano ordinari** (scelta di Mario): le loro variazioni di soddisfazione e le minacce di dimissioni sono fatti del governo, non del partito, e restano fuse con le altre. La regola «il soggetto è il giocatore» guarda solo il campo `partito`.
- **Il ritorno della soddisfazione verso la base resta muto** (scelta di Mario): è un movimento continuo (`crates/sim-core/src/ministri/soddisfazione.rs:54-56`), come la rata di un cantiere in corso; nel registro una soddisfazione può salire senza voce.
- **Dopo «avanti fino a novità»** il rimando della banda apre il registro sul più vecchio dei turni saltati, senza segni nella colonna (scelta di Mario).
- **Lettura dei registri** (scelta mia): entrando nella sezione la UI chiede `registro(t)` per ogni turno chiuso, un file ciascuno, e li tiene in memoria fino al riavvolgimento successivo; la colonna e i filtri hanno bisogno di tutti.
- **Sezione di una voce** (scelta mia): una tabella della UI per tipo, come quella dei tracciati della prima pagina; la voce non la porta.
- **Revisione e prima stima** (scelta mia): la UI cerca la `prima_stima` nel registro del turno prima; nessun legame nel dato.

### Rimandi

Due al core, PR 1 della spec «esito e Vista completa», Task 1 del piano (scritti in spec e piano il 12/11/2026):

- **I soggetti delle voci fuse:** `dettagli` diventa `Vec<Dettaglio>`, ogni riga col suo testo e i suoi `partito`, `dicastero`, `regione`, `misura` e `disposizioni`. Oggi la fusione li butta quando non sono uguali in tutte, e il filo «Interno» passerebbe solo dal testo.
- **Le disposizioni nella voce:** `disposizioni: Vec<String>`, gli id delle disposizioni che la misura tocca. Una legge ripresentata prende un id di misura nuovo (pensioni 2 e 7, concorrenza 4 e 9 nel seme 4), e il filo per misura si spezzerebbe. Il bilancio si segue per misura.

### Guscio e dati

Nessun passacarte nuovo: `registro` è già assegnato a 11f ([Contratto del guscio](12-contratto-del-guscio.md)), il riavvolgimento passa da `riavvolgi`.

### Stranezze notate

- Con gli atti ricavati dalle Viste la chiusura 3 ha «Sei cantieri fermi», notevole, che entra fra le sette: la prima pagina del 16, scritta senza gli atti, ne mostrava cinque.
- Le emergenze lievi si vedono nelle Viste solo come 100 mln € in meno nel fondo di riserva (chiusure 5 e 6), senza regione né tipo: la voce `emergenza_lieve` del core le dirà.
- Due cantieri finiscono giocando (chiusure 5 e 6): il «NON RAGGIUNTO» del ticket Paese vale per le opere sulla mappa, non per la voce.
- All'apertura del turno 9 nessun ministro ha una proposta.

### Lessico

«Norma» nel glossario è il tipo della misura (ordinaria, costituzionale, elettorale); il soggetto del filtro è la **disposizione**. Nessun termine nuovo in `CONTEXT.md`.

### Scartato

A · Indice senza filtri; B · Cronologia (tutte le chiusure in una colonna: il filo entra nell'Indice, la lettura resta per turno); C · Tabellone turni × sezioni (vista d'insieme senza testo); «Rigioca questo turno» e il ritorno all'apertura del turno dopo; il filtro che cerca nel testo dei dettagli; il segno «non letto» sui turni saltati.
