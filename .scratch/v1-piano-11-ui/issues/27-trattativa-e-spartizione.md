Type: prototype
Status: resolved
Blocked by: 06, 08, 32
Assignee: maeurong
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la casa del turno dopo il voto? Il risultato in testa; la spartizione dei dieci dicasteri proposta dal giocatore, con la richiesta di ogni alleato in vista e l'intesa di partenza che sale o scende (storie 79-80); i tecnici nella quota del proprio partito (storia 81); l'allargamento ad altri partiti quando manca il premio (storia 82). Variante con l'incarico a un altro: chiedere la guida o rifiutare il ribaltone (storia 98). Da vedere se il rimpasto dopo l'uscita di un alleato (storia 84) riusa la stessa composizione.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 su un prototipo navigabile: [27-trattativa.prototype.html](../prototipi/27-trattativa.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [27-trattativa.json](../viste/27-trattativa.json), nuovo in `genera/` (`src/trattativa.rs`), tutto giocato. Cinque stati: la 02 col premio, in tono ordinario e grave; l'incarico senza maggioranza (proporzionale al 3%, seme 4 a 5,5: 188 seggi su 400, Centrosinistra e Centro disponibili); il ribaltone (seme 2 a 1,5, turno 5: Sinistra, Centrosinistra, Centro e il giocatore dopo la caduta del governo di destra, la Sinistra prima per seggi); l'incarico al Capo dello Stato (seme 1 a 1,5, il turno prima del 06).

### Forma: A · Righe

La casa del turno in trattativa, sotto la cornice, col **dettaglio a lato**.

- **Il risultato in testa**: il voto con l'affluenza e il premio (o «nessun premio»), le quote dei partiti con la tua coalizione in neretto, Camera e Senato a barre coi colori dei partiti, il filo rosso della metà e un segno nero sotto i seggi della coalizione; accanto una frase: «La tua coalizione ha 224 seggi alla Camera e 115 al Senato: la maggioranza in tutte e due», o quanti ne mancano.
- **Un dicastero per riga**: nome, peso a pallini (3 Economia, 2 Interno ed Esteri, 1 gli altri) e i bottoni «a chi»: Tu, Tecnico, poi gli alleati, quello scelto pieno del colore del partito. La riga cambiata rispetto alla spartizione di default ha un pallino blu. Sotto, i punti di ognuno su 14 con la richiesta accanto, e la nota: senza proposta, a fine turno vale quella di default.
- **Gli alleati nel dettaglio a lato** (storie 79-80): faccia del leader, «chiede 2,5 punti, ne offri 2», una fila di 14 caselle coi punti dati e il segno della richiesta, l'intesa di partenza in dieci caselle col filo dell'ultimatum, «parte da 55 · 0,5 punti sotto la richiesta». Sotto 40 «entra in ultimatum: spartizione», sotto 25 «può uscire alla prima chiusura». Poi il tuo partito, coi tecnici, la regola dell'intesa in una frase e «Torna alla spartizione di default».
- **L'allargamento** (storia 82) in testa al lato: con la maggioranza, una riga («Puoi anche allargare: Centrosinistra e Centro sono disponibili. Chi entra chiede la sua parte»); senza, l'avviso «Senza maggioranza, a fine turno l'incarico passa», i partiti fuori coi seggi, la disponibilità e la causa, «Allarga ›» su ognuno e «Allarga a tutti e due» quando sono più d'uno, con la conferma sul posto. Il tracciato della maggioranza in cornice segue la coalizione.
- **Il ribaltone** (storia 98) prende la pagina: «Il governo è caduto. Sinistra, Centrosinistra, Centro e il tuo partito hanno la maggioranza», chi è il primo per seggi, il risultato col ribaltone segnato, «Chiedi la guida a Luca Benedetti ›» e «Resta fuori». Ceduta la guida, la pagina diventa la spartizione; negata, il rifiuto sul posto con la causa del core. Nel lato i partiti del ribaltone coi seggi.
- **L'incarico a un'altra coalizione**: una pagina corta, «L'incarico va a un'altra coalizione», il risultato e le Camere a lato.
- **Cornice**: stato «Trattativa», la sezione di casa si chiama Trattativa; barrate Governo, Aula, Bilancio e Campagna col motivo; accanto a «Chiudi il turno» il default («senza proposta: la spartizione di default», «senza scelta: chiedi la guida»). Un evento aperto sta in una riga in testa: in trattativa nessuno governa e nessuno risponde (`crates/sim-core/src/partita.rs:4415`), vale il default.

### Decisioni sui casi

- **Allargare riporta la spartizione al default**, coi nuovi alleati dentro: lo fa già il core (`partita.rs:3061-3063`), e la conferma sul posto lo dice.
- **Il tecnico si nomina su qualunque dicastero** e conta sempre nella tua quota: messo su un dicastero del default di un alleato gli toglie il punto, e l'intesa di partenza scende. Verificato giocando: un tecnico su Sanità porta la Destra da 7 a 6 punti e da 62,1 a 52,1.
- **«Chiedi la guida» è diretto**: chiedere non costa niente. «Resta fuori» ha la conferma sul posto. Senza scelta, la chiusura chiede la guida.
- **Il rimpasto (storia 84) non riusa questa pagina**: il core lo fa da solo, senza comandi (`partita.rs:3474`); lo dicono Governo (l'interim) e Registro.
- **Tono grave**: le iniziali al posto delle facce e i colori dei partiti spenti, come nelle [Fondamenta visive](08-fondamenta-visive.md). La 02 è grave per l'emergenza idrogeologica.

### Guscio e dati

Dalla Vista: `incarico`, `richieste` (`crates/sim-core/src/vista.rs:79-83`), `disponibilita`, `seggi`, `elezioni`, `partiti`, `eventi`. Comandi già nel core: `ProponiSpartizione`, `Allarga`, `ChiediGuida`, `RifiutaRibaltone`. Nessun passacarte nuovo. Verificato giocando sulla 02: cinque proposte (default, generosa, avara, due col tecnico) danno dopo la chiusura l'intesa di `partita.rs:3284`; l'avara toglie Economia alla Destra, che parte da 32,1 in ultimatum per spartizione. Tre **rimandi al core, PR delle esposizioni, in 11g**:

- **La spartizione in corso, per dicastero**: a chi va (partito o tecnico) e il peso. La Vista porta solo richiesta e offerta in punti per alleato; il default lo calcola `assegnati_dal_giocatore` (`partita.rs:3250`) e i pesi stanno in `dati/base/dicasteri.toml`. Senza, la pagina non sa da dove parte.
- **L'intesa di partenza di ogni alleato** in `VoceRichiesta`: `intesa_base` e `intesa_per_punto` sono manopole (`dati/base/manopole.toml:793`, `:1129`) che la Vista non porta.
- **Il ribaltone**: la coalizione e il primo per seggi. `VoceIncarico::Guida` non porta niente (`vista.rs:193`), mentre il core li tiene in `Incarico::Guida`.

### Scartato

B · Cesti (un cesto per partito coi dicasteri come tessere alte quanto il peso e la richiesta tratteggiata); C · Incroci (tabella dicasteri × partiti a pallini coi totali sotto); il tecnico solo sui tuoi dicasteri; la conferma sul posto anche per «Chiedi la guida».
