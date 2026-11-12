Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Conti? Il bilancio in vigore con la perdita reale di ogni voce (storia 41) e il fondo di riserva residuo; le grandezze con la distanza dalla soglia e le situazioni accese (storie 121 e 132, `VoceSituazione`); prima stima e revisione (storia 119); il reddito reale per fascia con la crescita attesa (storia 116); spread (storia 52), PDE e conto di controllo (storia 50), stato del MES e della ristrutturazione; il centro studi comprato con la cassa (storia 123, `VistaCentroStudi`) con la previsione a quattro turni. PDE, conto di controllo e MES arrivano dal piano di core «esito e Vista completa»: qui se ne fissa il posto.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in due giri su un prototipo navigabile: [21-conti.prototype.html](../prototipi/21-conti.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [viste/21-conti.json](../viste/21-conti.json), che `genera/` aggiunge giocando: il centro studi comprato davvero in sei stati (il 03, il 06 all'opposizione, due situazioni accese insieme al seme 4 turno 15, la recessione al primo trimestre sotto soglia al seme 1 turno 13, lo spread a 205 punti al seme 25 turno 16, il bilancio non presentato da due anni al seme 11 turno 25), con la cassa prima e dopo, il rifiuto del secondo acquisto e la `serie` dei turni.

### Forma: A · Listino, con le grandezze a lato

- **Tre colonne di pagina economica e il dettaglio a lato.**
  - **Il bilancio** dell'anno, «in vigore» o «esercizio provvisorio»: i dieci dicasteri con la spesa annua e la perdita reale («−0,1%» in rosso, con una barretta); l'Irpef con aliquota e soglia reale di ogni scaglione con soglia; Iva, Ires e contributi in una riga; il fondo di riserva rimasto sul totale, con una barra.
  - **L'economia** del trimestre: la prima stima in quattro numeri grandi (PIL annualizzato sul trimestre prima, disoccupazione, inflazione sull'anno prima, disavanzo); sotto, la revisione del trimestre prima, prima stima → riveduta con lo scarto; lo spread col grafico della `serie` e la linea delle aste scoperte; poi il posto di debito sul PIL, regole UE (procedura, conto di controllo, proiezione, coesione, margine per l'energia), MES e ristrutturazione, redditi per fascia, che arrivano in 11g.
  - **Le soglie**: le sei situazioni, ciascuna su un righello con la soglia di accensione rossa, lo spegnimento tratteggiato, l'isteresi a retino e il valore ufficiale come pallino; sotto, la frase della distanza («si accende sotto 85: mancano 14,7», «accesa · si spegne sotto 5: mancano 7,5»). In fondo la riga «Le 21 grandezze ›», che apre l'elenco nel dettaglio a lato.
- **Il dettaglio a lato** è il centro studi finché non si sceglie una riga; una riga lo sostituisce col suo dettaglio e «‹ Torna al centro studi». Dicastero: spesa, salari, acquisti, margine e la frase della storia 41 («Sanità: −0,1% reale dal bilancio 2027»), con «Indicizzalo nella legge di bilancio ›» verso Bilancio. Scaglione Irpef: quanto la soglia è erosa. Situazione: il righello grande, le due soglie, la distanza, cosa fa accesa. Grandezza: valore, unità, verso, la soglia se ne ha una. Numero della stima: prima stima, centro studi, revisione. Spread: la serie grande. Fondo di riserva: gli eventi aperti col loro costo.
- **Il centro studi**: prima dell'acquisto dice cosa vende, «Compra · 150.000 €» e la cassa del partito; dopo, la stima del trimestre accanto alla prima stima e quattro ventagli a quattro trimestri (PIL e inflazione sull'anno prima, disoccupazione, debito sul PIL) con la fascia di una e di due sigma. Il secondo acquisto del turno è un rifiuto sul posto col testo del core («già comprato questo turno: centro studi»). La cassa sta anche nella cornice.
- **All'opposizione** la pagina è la stessa: il bilancio è quello del governo, il centro studi si compra con la propria cassa.
- **Tono grave**: le regole delle fondamenta.

### Decisioni sui casi

- **Le grandezze regionali** (infrastrutture, investimenti privati, domanda della regione) si leggono come campo «da–a» delle 20 regioni, col rimando a Paese; nessuna media calcolata nella UI. Le nazionali col valore e lo scarto dal turno zero, colorato secondo il verso.
- **La perdita reale negativa** (deflazione: +1,0% nel 06) si scrive in verde col più.
- **La soglia delle aste scoperte** (550 punti, `soglia_crisi_debito` in `dati/base/manopole.toml:1542`) è una linea sul grafico dello spread. Se sia un numero della UI o un rimando al core lo decide [Scale dei tracciati](31-scale-dei-tracciati.md).

### Rimandi

- **Trimestri oltre la soglia — al core.** La recessione si accende dopo due trimestri di fila sotto il −1%, le altre situazioni dopo uno (`crates/sim-core/src/eventi/situazioni.rs:12-14`); il conto sta in `Eventi.contatori`, ma `VoceSituazione` non lo porta. Al seme 1, turno 13, la crescita è a −4,2% e la recessione è spenta senza che la pagina possa dire perché. Mario sceglie il rimando: `VoceSituazione` porta `trimestri` e `oltre`, e la riga dice «oltre la soglia da un trimestre: si accende al secondo di fila». Fino ad allora: «oltre la soglia».
- **Debito sul PIL — al core.** La Vista non porta il debito: oggi lo si legge solo comprando il centro studi, che lo prevede. Mario sceglie il rimando: `Vista.debito.debito_pil`, esatto dopo l'ultima chiusura, lo stesso numero con cui si calcola lo spread (`crates/sim-core/src/economia/spread.rs:21-27`).
- Tutti e due scritti il 12/11 nella spec di core «esito e Vista completa» (sezioni «Situazioni» e «Debito») e nel suo piano (Task 16c e Task 15), PR delle esposizioni, in 11g. PDE, conto di controllo, MES, ristrutturazione e redditi per fascia c'erano già (Task 15 e 17).

### Guscio e dati

Nessun passacarte nuovo: tutto viene da `Vista` (`bilancio`, `statistica`, `situazioni`, `grandezze`, `spread_pb`, `acquisti`); l'acquisto passa da `comanda` con `Compra { CentroStudi }`; il grafico dello spread legge `serie`, già assegnato a 11a dal [Contratto del guscio](12-contratto-del-guscio.md).

### Stranezze notate

- La perdita reale è la stessa per tutti i dicasteri e cresce anche presentando la bozza di `bozza_bilancio()` a ogni bilancio: 0,15% nel 03, 4,1% al seme 4 turno 15. La bozza non indicizza da sé.
- Il centro studi vede il debito al 137% del PIL nel 03, in calo nei quattro trimestri; la prima stima del PIL dice +3,1% e il centro studi +2,8%.
- Al seme 1 turno 13, al governo, la cornice legge dalla `serie` un consenso del 12% e margini di −109 e −58 seggi: la coalizione della seconda legislatura non è quella del primo voto. Da guardare quando si fanno i tracciati.
- Al seme 1 turno 13 la sicurezza è a 111 e la qualità del servizio sanitario a 126, contro accensioni a 85 e a 90: le situazioni dei servizi sono lontanissime dalla soglia, la sola vicina è il sovraffollamento delle carceri.

### Scartato

B · Strumenti (le situazioni come termometri in testa, la pagina scorre di più); C · Schede (niente scorre, ma le soglie stanno dietro una scheda); le grandezze in pagina sotto le situazioni (la colonna lunga quasi due schermi a 1280).
