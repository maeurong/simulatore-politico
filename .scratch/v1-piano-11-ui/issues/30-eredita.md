Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: maeurong
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta l'eredità, la casa della partita finita? Anni al governo, leggi ancora in vigore, indicatori contro il turno zero, numero di riavvolgimenti (storia 149); la serie dei tre indicatori e i risultati di ogni elezione; la bozza dopo la sconfitta per seggi, con «torna a un turno» verso il registro (storia 150) e la conferma sul posto che la manda nell'albo; l'eredità immediata dopo un abbandono (storia 152). Da vedere se la stessa composizione si rilegge dall'albo nell'[Edicola](13-edicola.md).

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [30-eredita.prototype.html](../prototipi/30-eredita.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [viste/30-eredita.json](../viste/30-eredita.json), una partita **giocata** fino alla fine senza seggi, niente di forzato. `genera/` gioca il seme 43 a 4,5: un anno al governo, due leggi approvate, poi al voto di ottobre-dicembre 2031 il 2,4% e nessun seggio. La fine senza seggi si raggiunge giocando, contro quanto notava l'[Edicola](13-edicola.md), che provava solo il posto 6,5: 17 partite su 720 (vedi le stranezze del 30 in `viste/README.md`).

### Forma: A · Ultima edizione

La pagina di giornale della fine, dentro la cornice; nel sommario la sezione di casa si chiama «Eredità».

- **Testa**: occhiello serio col fatto e il numero («Voto di ottobre-dicembre 2031 · 2,4%, nessun seggio»), titolo spiritoso, bollo dell'esito; sotto, una riga con scenario, inizio e fine, turni, anni al governo e riavvolgimenti (storia 149).
- **Tre colonne**: le elezioni, la più recente in testa, come barre delle quote con la quota e i seggi del giocatore; il Paese che lasci, cioè i quattro indicatori al turno zero e alla fine con la freccia (salire è male per debito, spread e disoccupazione; l'inflazione non ha colore); le leggi che restano, col nome e l'etichetta della legge.
- **I tre grafici in fondo**, uno per indicatore (consenso della coalizione; margine sulla metà più uno, Camera piena e Senato tratteggiato; spread), i voti segnati e il valore al passaggio del puntatore. Sono quelli della voce d'albo dell'Edicola, più larghi.
- **Il dettaglio a lato** tiene la bozza (storia 150), col riavvolgimento preso da B: «Torna al turno N · prima del voto ›», diretto, riapre il trimestre prima delle urne; «Un altro turno, nel Registro ›» porta al Registro, dove «Torna al turno N» sta già in testa a ogni turno ([Registro](25-registro.md)); poi «Conferma l'eredità». Tutte e due le azioni hanno la conferma sul posto: il pulsante diventa due pulsanti. Quella del riavvolgimento dice cosa si cancella: le chiusure dal turno N all'ultimo, il voto perso e l'eredità in bozza; che il riavvolgimento si conta, e che con le stesse scelte il caso dà lo stesso esito. Quella dell'eredità dice che non si torna più a nessun turno e che la partita esce da quelle in corso. Confermata, l'app apre l'Edicola con la voce d'albo aperta, come dopo l'abbandono.
- **Cornice a partita finita**: «Partita finita · Senza seggi» al posto di chi governa, i tracciati fermi in grigio all'ultimo voto, «Nessun turno da chiudere» al posto dei pulsanti di chiusura. Barrate Governo, Aula, Bilancio e Campagna, col motivo; il Registro resta acceso finché l'eredità è in bozza ([Ossatura e navigazione](07-ossatura-e-navigazione.md)).

### Riletta dall'albo

La stessa composizione si rilegge dall'albo. Nell'[Edicola](13-edicola.md) il dettaglio della voce d'albo prende «Leggi l'eredità ›», che apre questa pagina senza cornice, con «‹ Edicola» e senza azioni; nel dettaglio a lato resta il bollo con la frase sull'albo. Vale anche per le partite abbandonate: l'abbandono resta nell'Edicola e non apre la pagina ([Edicola](13-edicola.md)), ma la voce d'albo che lascia si rilegge qui.

### Dati e guscio

- Nella partita la pagina si compone da `Vista.eredita`, `Vista.elezioni` e `serie` (passacarte di 11a); dall'albo da `VoceAlbo`, che porta gli stessi campi più nome, scenario, inizio e fine. Una composizione, due sorgenti: l'adattatore sta nella UI e non calcola niente.
- Comandi già nel guscio: `riavvolgi` (`app/src-tauri/src/lib.rs:294`), `conferma_eredita` (`lib.rs:311`), `albo` (`lib.rs:319`). Nessun passacarte nuovo, nessun rimando al core.
- La Vista dice la legge in vigore ma non quando è entrata: la colonna mostra il nome e l'etichetta della legge («L. 77/2031»), che porta già l'anno. La data d'entrata del prototipo viene da `genera/` e non va nella fetta.
- Il titolo spiritoso della fine senza seggi è un tipo di voce nuovo per i [Testi del Pungolo](09-testi-del-pungolo.md): bozza degli agenti e taglio di Mario nella fetta 11e, con le stesse regole (niente nomi). L'occhiello si scrive coi dati dell'ultima elezione.

### Scartato

B · Consuntivo (quattro numeri in testa, righelli contro il turno zero, la bozza in una banda nera sotto la cornice), da cui viene il pulsante diretto a prima del voto; C · Cronologia (la partita su un asse solo con le legislature a fasce e ogni turno riavvolgibile dal dettaglio a lato); il riavvolgimento solo dal Registro; la voce d'albo letta solo nel dettaglio dell'Edicola.
