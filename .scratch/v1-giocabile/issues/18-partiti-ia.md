Type: grilling
Status: resolved
Blocked by: 16
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come decidono i sette partiti IA nella v1? Quando guidano il governo avversario: quali misure del catalogo portano e in che forma, come il loro Presidente del Consiglio gestisce agende, emendamenti e fiducia, quando si dimette per scommettere sul voto anticipato. Quando stanno all'opposizione contro il giocatore: quando presentano sfiducia (al governo o a un ministro), proposte di legge, referendum abrogativo e confermativo, e quale linea scelgono oltre la regola della distanza. Nel tempo: la loro posizione sull'asse si muove, e con quale regola? Hanno un leader che conta?

Ingresso da Coalizione e crisi (05): il governo avversario segue le stesse regole di coalizione, intesa e crisi; ogni partito sceglie la coalizione pre-elettorale sul valore atteso.

Ingresso da Elettorato (06): la posizione percepita di un partito scivola con le misure votate.

Ingresso da Iter parlamentare (08): in aula seguono la regola della linea.

Ingresso da Opposizione (12): le leve dell'opposizione sono simmetriche, quindi i partiti IA all'opposizione hanno quelle del giocatore (linea, sfiducia, una proposta nel posto riservato, referendum). Un alleato del governo avversario in ultimatum vota la sfiducia se la caduta gli rende di più. Nel ribaltone il partito più grande della nuova maggioranza accetta la guida del giocatore se il suo valore atteso supera quello delle elezioni. La campagna dei partiti IA resta alla nebbia Campagna elettorale.

Ingresso da Catalogo atti e riforme (16): le leggi di riforma sono cambi di valore delle 30 disposizioni; un partito vota sì se il valore nuovo è più vicino alla sua posizione di quello in vigore. Il governo avversario porta disposizioni al valore più vicino alla sua posizione; i suoi ministri scelgono gli atti con lo stesso punteggio di quelli del giocatore (04). Schema in `../assets/16-catalogo-schema-e-sanita.md`.

## Answer

Grilling con Mario del 17/10/2026, tre giri. Raccomandazioni accettate tranne una: sulla nascita e morte dei partiti Mario ha scelto anche le scissioni, non i sette partiti fissi. Quadro finale confermato. Regole di comportamento da `docs/ricerca/modelli/abm-elettorali.md` (Laver-Sergenti, Lehrer-Schumacher). Il difetto da evitare è «all political parties play identically» (`docs/ricerca/sistemi/partiti-e-coalizioni.md:17`). Termini nuovi in `CONTEXT.md`: leader, vocazione, regola di movimento, posizione percepita, programma, scissione, partito di protesta; precisato partito; «leader» tolto dagli _Evita_ del Presidente del Consiglio.

**Come decidono.** Regole a punteggio che riusano le formule del giocatore: valore atteso (05), punteggio dell'agenda (04), linea (08, 16). C'è un piccolo rumore a seme fisso, e ogni mossa è spiegata in una riga nell'esito. Niente ricerca in avanti: l'avversario deve essere leggibile, come le soglie del 04 e del 05.

**Tratti visibili, per partito.**
- **Vocazione:** quanto pesa le politiche rispetto alle poltrone (α di Lehrer-Schumacher, `abm-elettorali.md:35`), scritta in parole. Entra nel valore atteso delle coalizioni (peso della distanza contro quello dei seggi), nel voto sulla sfiducia e nella velocità con cui cala l'intesa per misure lontane. Alla partenza è identitaria agli estremi, governista al centro, mista per gli altri; l'editor dello scenario la può cambiare.
- **Regola di movimento** (`abm-elettorali.md:14-18`), anch'essa modificabile dall'editor:

| Partito | Regola | Nel turno prima del voto |
|---|---|---|
| estrema sinistra, estrema destra | Sticker | resta fermo |
| sinistra, destra | Aggregator | mezzo punto verso il baricentro di chi l'ha votato alle ultime elezioni |
| centrosinistra, centrodestra | Hunter | ripete lo spostamento se fra le ultime due elezioni ha guadagnato voti, altrimenti inverte; alla prima legislatura va verso il centro |
| centro | Governator, versione satisficing | fermo se è al governo, altrimenti verso la posizione del governo, senza superarla |

**Posizione.** La dichiarata si muove una volta a legislatura, nel turno prima del voto (il programma elettorale): mezzo punto al massimo, senza mai scavalcare il partito vicino, così i nomi sull'asse restano veri. La percepita scivola sempre con le misure votate (06). **Supera** il «dichiarata fissa» del 06 e le «posizioni fisse» del glossario. Per simmetria, la nebbia Campagna elettorale deve dare la stessa mossa al partito del giocatore.

**Leader.** Ogni partito IA ha un personaggio inventato, con nome e caricatura per il borsino (10), che porta la valence del partito (06). Il leader del partito che guida il governo avversario ne è il Presidente del Consiglio. Il partito cambia leader dopo una sconfitta: quando perde il governo, o quando cala di almeno X punti al voto. Il nuovo leader riparte dalla valence base, con una breve luna di miele. Niente congressi né sfide giocate.

**Governo avversario.**
- *Nascita:* coalizione col valore atteso (05); la guida il partito più grande. Spartizione = richiesta di Gamson esatta, perché fra IA non c'è trattativa da giocare; niente tecnici.
- *Programma:* pubblico, dichiarato alla nascita. Contiene le disposizioni (16) più lontane dalla posizione della coalizione (media dei partiti pesata per seggi), ognuna portata al valore più vicino a quella posizione e ordinata per distanza. Riempie i 3 posti del calendario in quell'ordine. Gli atti li scelgono i ministri, col punteggio del 04.
- *Legge di bilancio:* i budget dell'anno prima, indicizzati e ricondotti al tetto della spesa netta UE (02), più le richieste di budget degli alleati in ultimatum. Le aliquote cambiano solo se una disposizione fiscale è nel programma.
- *Eventi:* l'opzione di default dell'evento (09), per decreto solo se l'evento lo consente.
- *Condotta del Presidente del Consiglio avversario,* con una regola sola, ridurre il rischio di cadere:
  - accoglie un emendamento se, senza i voti di quell'alleato, la forbice include la sconfitta, o se l'alleato è sotto 40 di intesa;
  - pone la questione di fiducia quando la forbice include la sconfitta e nessun alleato è in ultimatum;
  - non ferma né dirotta mai le agende dei ministri;
  - in crisi allarga a un partito contiguo che ci guadagna, altrimenti si dimette.
- *Voto anticipato:* si dimette se il valore atteso del voto (05, sugli stessi sondaggi rumorosi) supera quello del restare, oltre un margine fisso. Mai nel primo anno di legislatura né a meno di 2 turni dalla scadenza; poi decide il Capo dello Stato, e il ribaltone può essere del giocatore (12).

**Partiti IA all'opposizione contro il giocatore** (leve simmetriche, 12).
- *Sfiducia:* la presentano solo quando passerebbe, cioè quando un alleato in ultimatum ci guadagna. La presentano in apertura e la votano in chiusura, quindi il giocatore ha un turno per alzare l'intesa sopra 40 o cedere. Niente sfiducie di bandiera. La sfiducia individuale, con la stessa condizione, colpisce il ministro del dicastero col giudizio peggiore fra i gruppi, mai un ministro dell'alleato in ultimatum.
- *Proposta nel posto riservato* (a rotazione): la disposizione e il valore nuovo che massimizzano i seggi della maggioranza la cui linea sarebbe sì. È una proposta-imbarazzo: il giocatore accetta i franchi tiratori o paga in intesa.
- *Referendum abrogativo:* nella finestra annuale, sulla legge in vigore la cui abrogazione vince nel sondaggio del trimestre e raggiunge il quorum stimato; se sono più d'una, quella col margine più ampio. Il confermativo resta la regola dell'08.
- *Linea:* quella dell'08, ma il malus d'opposizione cresce nell'ultimo anno di legislatura.

**Vita dei partiti.**
- *Partito di protesta:* si valuta nel turno prima del voto, al massimo uno a legislatura. Nasce in un tratto dell'asse con almeno il X% degli elettori (istogrammi del 06), lontano almeno 1 punto dal partito più vicino e con l'astensione sopra la media. Prende il centro di quella zona, un leader nuovo in luna di miele, regola Hunter e vocazione identitaria; non ha seggi fino al voto.
- *Scissione:* quando la posizione percepita dista almeno 1 punto dalla dichiarata; a mezzo punto scatta la minaccia, visibile con la causa. Nasce subito, anche a metà legislatura, un partito sulla vecchia posizione dichiarata, con una quota fissa dei seggi del partito d'origine. Se quel partito era al governo, il nuovo esce dalla coalizione, la maggioranza si ricalcola e può aprirsi una crisi (05). Vale anche per il partito del giocatore, che resta leader della parte che rimane.
- *Scioglimento:* un partito IA senza seggi per due elezioni di fila si scioglie, e i suoi elettori si ridistribuiscono col logit del 06. Il partito del giocatore no: senza seggi la partita finisce già (12).
- *Tetto:* 10 partiti. Oltre il tetto non nasce nulla e la scissione resta una minaccia. Niente fusioni.

**Ne escono:**
- Crisi ed eventi (15): ogni evento deve avere un default sensato, perché il governo avversario sceglie quello.
- Legge elettorale come misura (13): se soglie e premio diventano disposizioni, il programma del governo avversario può cambiarli; da decidere se ammetterlo.
- Campagna elettorale (nebbia): il riposizionamento del giocatore, simmetrico alla regola di movimento; il turno prima del voto è anche quello delle nascite e dei movimenti dei partiti IA.
- Costo del governare (nebbia): soglia X% della nascita, quota di seggi della scissione, soglia del cambio di leader, margine del voto anticipato, crescita del malus nell'ultimo anno, peso dei livelli di vocazione.
- Scenario iniziale (nebbia): leader, vocazioni e regole di partenza dei sette partiti.
- Punti di aggancio (nebbia): fusioni fra partiti.
