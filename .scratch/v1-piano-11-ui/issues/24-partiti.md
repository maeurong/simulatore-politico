Type: prototype
Status: resolved
Blocked by: 06, 08, 32
Assignee: maeurong
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Partiti? L'asse con i sette partiti IA e quello del giocatore, posizione dichiarata e percepita (storia 117), leader con nome e caricatura (storia 103), vocazione e regola di movimento in una riga (storia 100), minaccia di scissione (storia 108, dal piano di core), nascite e scioglimenti (storie 108-109); gli alleati con intesa, cause, ultimatum e soglie (storia 83); il parlamento in carica; le casse dei partiti (storie 122 e 129). Variante all'opposizione: gli alleati sono quelli avversari, in fasce.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 su un prototipo navigabile: [24-partiti.prototype.html](../prototipi/24-partiti.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: le Viste campione, senza aggiunte a `genera/`. Otto stati: il 03 al governo; l'ultimatum del [Cruscotto al governo](17-cruscotto-al-governo.md) (seme 1, turno 13: la Sinistra in ultimatum a 29, due partiti in minaccia di scissione, la Lista dei Territori nata al 27,7%); la crisi in esercizio provvisorio dello stesso ticket (seme 14, turno 37: dieci partiti, il tetto, e due leader nuovi); il 06 e la fine legislatura all'opposizione; l'[Eredità](30-eredita.md) con l'Estrema sinistra sciolta; il tono grave dell'08; il turno zero.

### Forma: A · Schieramento

Sotto la cornice, l'asse a pagina e le schede, col **dettaglio a lato**.

- **L'asse da 1 a 7 a tutta larghezza**: ogni partito ha un quadrato del suo colore sulla posizione dichiarata e un anello su quella percepita, con la freccia fra i due (storia 117). Chi è a mezzo punto o più ha la freccia rossa e i due fili, a mezzo punto la minaccia, a un punto la scissione, solo dal lato dove scivola. I nomi stanno sopra, su più file quando si toccano, dentro i bordi.
- **Una scheda per partito**, nell'ordine dell'asse e su due file (quattro per fila con otto partiti, cinque con dieci): la faccia del leader a 46 px a sinistra, il nome del partito sotto il filo del suo colore, il leader, il sondaggio, i seggi di Camera e Senato, la cassa, vocazione e regola in breve («mista · insegue i voti»). Per un alleato al governo, l'intesa in dieci caselle col filo dell'ultimatum; all'opposizione, la fascia dell'alleato del governo in tre caselle. Sotto, gli avvisi: ultimatum, minaccia di scissione, sciolto, governo uscente, guida il governo.
- **Il tuo partito** ha la casella «Tu» al posto della faccia: il giocatore resta sempre leader del suo.
- **Il dettaglio a lato, senza scelta, mostra le Camere**: Camera e Senato in carica e se si votasse oggi, come barre coi colori dei partiti e il filo della maggioranza, i seggi di ogni partito e «N partiti su 10». A Camere sciolte la riga «in carica» dice solo «sciolte, fino al voto».
- **La scheda del partito** nel dettaglio a lato: faccia a 84 px, leader («al posto di Luca Benedetti, dopo la sconfitta» quando è cambiato), gli avvisi; la posizione sull'asse con la tua accanto e una frase («Gli elettori lo vedono a 2,76, verso destra: 0,76 punti dalla dichiarata. Oltre mezzo punto: minaccia la scissione…»); l'indole in una riga, vocazione e regola per esteso (storia 100); l'intesa con te; i seggi in carica e se si votasse oggi, il sondaggio, l'ultimo voto, il carisma del leader, la cassa (storie 122 e 129); le promesse; le mosse del turno.
- **L'intesa** (storia 83): la barra da 0 a 100 coi fili a 40 («ultimatum») e a 25 («esce»), l'ultimatum con la causa, la causa che pesa di più, le variazioni degli ultimi due turni col segno; in ultimatum, «Rispondi nel Cruscotto ›». All'opposizione la fascia (solida, incrinata, in ultimatum) con la causa dell'ultimatum, e la riga «un alleato incrinato vota meno compatto».

### Decisioni sui casi

- **Carisma del leader come numero con segno** («carisma +0,61, del leader, nel voto»): è la valence del partito.
- **I partiti nati in partita a retino grigio**, tutti uguali, distinti dal nome, anche nelle barre delle Camere. Gli otto colori del [Paese](23-paese.md) restano agli otto partiti dello scenario.
- **Nati, sciolti e leader nuovi li dice il registro**: le voci `partito_di_protesta`, `scissione` e `cambio_di_leader` della spec di core «esito e Vista completa» portano origine e turno. La scheda di un nato dice «nato in partita» e l'origine quando il registro la porta. Lo sciolto resta nell'elenco, barrato, a pallido, con «Sciolto dopo due elezioni senza seggi»; un partito senza seggi all'ultimo voto ha l'avviso «alla seconda elezione di fila si scioglie», contato da `Vista.elezioni`.
- **Dieci partiti** stanno in due file da cinque; il tetto è scritto nel lato («al massimo dieci: oltre, nessuno nasce»).
- **Turno zero**: i partiti del governo uscente portano «governo uscente», nessuno guida il governo.
- **Tono grave**: le iniziali al posto delle facce, i colori dei partiti spenti in grigio, la classe unica delle [Fondamenta visive](08-fondamenta-visive.md).
- La sezione è di sola lettura: nessun comando, nessuna conferma sul posto. Riposizionarsi sta nella [Campagna](26-campagna.md), rispondere all'ultimatum nel Cruscotto.

### Guscio e dati

Tutto dalla Vista: `Vista.partiti` (`crates/sim-core/src/vista.rs:446-462`), `seggi`, `proiezione`, `sondaggio`, `elezioni`, `alleati` (`vista.rs:228-237`), `alleati_avversari`, `mosse`. La minaccia è già nella spec di core, PR delle esposizioni: `VocePartito.minaccia` (`docs/superpowers/specs/2026-11-11-esito-e-vista-completa.md:442`) e le due soglie in `VistaCampagna` (`:488`). Nessun passacarte nuovo. Un **rimando al core, PR delle esposizioni, in 11g**:

- **Le soglie dell'intesa**: 40 per l'ultimatum e 25 per l'uscita sono manopole (`dati/base/manopole.toml:807-811`, `:1136-1139`) che la Vista non porta, e la barra dell'intesa ne ha bisogno per i suoi fili. Stesso caso delle soglie della soddisfazione nel [Governo](18-governo.md).

Le frasi dell'indole vengono da come il core muove i partiti (`crates/sim-core/src/legislatura/movimento.rs:22-26`): Sticker non si sposta mai; Aggregator va verso i suoi elettori; Hunter, dopo un guadagno, rifà il passo e, dopo una perdita, torna indietro; Governator resta fermo al governo e fuori si avvicina al governo.

### Scartato

B · Schedario (tabella densa, una riga per partito, con le Camere a barre sotto); C · Campi (tre colonne per rapporto con te: maggioranza, opposizione, fuori dalle Camere); il carisma a pallini o nascosto; un colore in più per ogni nato, o quello del partito d'origine; il lato aperto sul tuo partito.
