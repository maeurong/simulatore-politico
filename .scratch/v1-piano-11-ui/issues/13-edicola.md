Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta l'edicola, la schermata fuori dalla partita che l'app apre sempre all'avvio? Partite in corso con nome, scenario, turno e data dell'ultimo salvataggio (storia 147), l'ultima giocata in testa con «Riprendi»; la partita che non si apre col suo motivo (storia 148, `VocePartita.apribile` e `motivo`); l'abbandono con la conferma sul posto (storia 152); l'ingresso a «Nuova partita»; l'albo con eredità, serie dei tre indicatori e risultati di ogni elezione (storia 151, `VoceAlbo`); l'ingresso alle impostazioni. Dati dai comandi del guscio `partite`, `riprendi`, `albo`.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario l'11/11/2026 in un giro su un prototipo navigabile: [13-edicola.prototype.html](../prototipi/13-edicola.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili con `?v=B|C`. Dati dalle partite giocate da `genera/` ([viste/13-edicola.json](../viste/13-edicola.json)): `testata()` e `voce_albo()` vere, `ultimo_salvataggio` a orari fissi. Forzati e dichiarati: lo schema 16 di una partita, e una partita finita senza seggi con l'eredità in bozza, che coi semi 1-60 non si raggiunge giocando (`crates/sim-core/src/partita.rs:2891-2893`).

### Forma: A · Banco

Pagina senza cornice: testatina «Il Pungolo · Edicola», a destra «Impostazioni». Sotto, le dodici colonne con il **dettaglio a lato**, come il borsino.

- **In corso**, in testa: le partite in ordine di `ultimo_salvataggio`, la più recente più grande con «l'ultima giocata», «Riprendi ›» pieno e «Abbandona» come link rosso. Riga: nome del partito, scenario, turno, trimestre, «salvata ieri alle 22:20» (oggi, ieri, giorno della settimana fino a sei giorni, poi la data); i riavvolgimenti solo se ce ne sono. «Nuova partita ›» sta sul filetto della sezione.
- **Albo**, sotto: nome, scenario, inizio e fine, anni al governo, bollo dell'esito («Abbandono», «Senza seggi»).
- **Dettaglio a lato** della riga scelta. Partita in corso: scenario, turno, salvataggio, riavvolgimenti, schema, azioni. Voce d'albo: esito, leggi ancora in vigore, riavvolgimenti; indicatori al turno zero e alla fine con la freccia (salire è male per debito, spread e disoccupazione; l'inflazione non ha colore); i tre indicatori turno per turno in **tre piccoli grafici**, uno per asse (consenso; margine sulla metà più uno con Camera piena e Senato tratteggiato; spread), i voti segnati, valore al passaggio del puntatore; ogni elezione come barra delle quote con la quota e i seggi del giocatore.

### Casi

- **Non si apre** (`apribile` falso, storia 148): riga a retino, nome in grigio, il `motivo` in rosso, nessun pulsante. Nel dettaglio: il salvataggio resta dov'è e non si cancella.
- **Finita senza seggi, eredità in bozza** (`testata.finita`, storia 150): resta fra le partite in corso, «Apri l'eredità ›» al posto di «Riprendi», niente «Abbandona» (il guscio lo rifiuta, `app/src-tauri/src/lib.rs:181`).
- **Abbandono** (storia 152): «Abbandona» diventa sul posto «Conferma: abbandona» e «Annulla», e il dettaglio dice cosa succede. Confermato, **resta nell'edicola**: la partita esce dall'elenco e la sua voce d'albo si apre nel dettaglio. Nessuna pagina Eredità per questo caso.
- **Primo avvio**: le due sezioni dicono di essere vuote; il dettaglio invita a «Nuova partita ›».

### Guscio e dati

- `abbandona` lavora sulla partita caricata (`app/src-tauri/src/lib.rs:176-190`): dall'edicola la UI chiama `riprendi(id)` e poi `abbandona`, poi `partite` e `albo` per ridisegnare. Nessun passacarte nuovo.
- Le partite in corso si presentano con ciò che `Testata` porta (`app/src/lib/tipi/Testata.ts`): niente indicatori né stato del giocatore, e nessun rimando al core per averli.
- Numeri del formato `Intl` di `t()` ([Testi esterni](03-testi-esterni.md)); il `-` del margine è il segno meno.

### Scartato

B · Copertina (l'ultima partita come apertura di prima pagina, l'albo su una pagina sua); C · Schedario (una tabella con filtri, dettaglio aperto sotto la riga); pagina Eredità dopo l'abbandono; indicatori nell'indice delle partite.
