Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come comincia la partita? In quale trimestre cade il turno zero, e chi ha approvato la legge di bilancio in corso? Il parlamento di partenza nasce da un voto simulato dal motore con la 180/2026 sull'elettorato tarato, oppure da una composizione scritta? Come si crea il partito del giocatore (nome, posizione sull'asse, rapporto coi sette partiti vicini) e come si garantisce il «buon risultato» che la v1 promette (`docs/ricerca/sintesi/decisioni.md:50`)? Chi governava prima del voto, e cosa eredita il giocatore (governo uscente in interim, disposizioni ai valori reali, giudizi dei gruppi)? Con quali quote di voto, valence e leader partono i sette partiti IA?

Ingressi:
- Partiti IA (18) dà vocazioni e regole di movimento di partenza, già decise; restano leader e valence.
- Coalizione e crisi (05) e Giro del turno (09): il voto si tiene in chiusura e la trattativa occupa il turno dopo, col governo uscente in interim.
- Dati per tarare i gruppi (14) dà il metodo di taratura dell'elettorato.
- Catalogo atti e riforme (16): al turno zero le disposizioni stanno ai valori reali 2026.
- Tabella degli interessi (17): il giudizio parte da zero per chi entra al governo.

## Answer

Grilling con Mario del 17/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Termine nuovo in `CONTEXT.md`: turno zero.

**Si parte un turno prima del voto, non dopo.** Con la 180/2026 la coalizione si dichiara prima del voto, con un candidato unico (05), quindi la trattativa vera è quella sui sondaggi. Il giocatore dichiara la coalizione, il voto si tiene in chiusura e la spartizione dei dicasteri occupa il turno dopo. Perdere il primo voto porta all'opposizione (12). **Supera** in parte il «partito appena entrato in parlamento con un buon risultato» del 16/10 (`docs/ricerca/sintesi/decisioni.md:50`): al turno zero il partito è fuori dal parlamento e il buon risultato è atteso, non scritto. La fase da outsider resta fuori: la fondazione è una schermata e la prima campagna dura un turno.

**Fondazione**, prima del turno zero. Il giocatore sceglie il nome del partito e uno dei sei mezzi punti fra due partiti vicini (1,5 … 6,5), mai sovrapposto a un partito IA: i nomi dei sette posti restano veri, e la regola «senza scavalcare il vicino» (18) vale subito anche per lui. La schermata mostra i sei posti con l'istogramma dell'elettorato (06) e, per ogni posto, il risultato atteso e le coalizioni possibili con la loro disponibilità, lette sullo stesso sondaggio rumoroso del gioco. Il leader è il giocatore; la caricatura è contenuto.

**Peso del partito nuovo.** Viene dalla luna di miele del leader nuovo, la stessa del partito di protesta (18), uguale per ogni posto e tarata una volta perché il posto medio dia circa il 15%. Scegliere il centro o un'ala è una scelta vera, con più elettori o meno concorrenza. Durata e decadimento vanno al bilanciamento.

**Calendario.**
- Turno zero: ottobre-dicembre 2026, il primo trimestre in cui vale la 180 (dal 24/10/2026, ticket 01), con la legislatura sciolta un anno prima della scadenza reale. L'aula è chiusa: si fanno campagna e voto.
- La legge di bilancio 2027 risulta approvata dal governo uscente prima dello scioglimento, senza voto in gioco: il bilancio 2026 indicizzato e ricondotto al tetto UE del 2027 (+1,9%, ticket 07), la stessa regola del governo avversario (18).
- Gennaio-marzo 2027: spartizione col governo uscente in interim (09). La prima legge di bilancio del giocatore arriva a fine 2027.

**Parlamento di partenza** prodotto dal voto simulato dal motore: elettorato tarato (14) → logit (06) → riparto con la 180, seme fisso. Seggi, sondaggi e giudizi sono coerenti dal primo turno; cambiando seme cambia la partita.

**Governo uscente:** una coalizione IA di centrodestra e destra, quella che ha fissato i valori reali 2026 delle disposizioni (16). Non ha storia: il giudizio dei gruppi parte da zero per tutti i partiti. Regge l'interim della trattativa, poi è opposizione o alleato possibile come gli altri.

**Taratura dei sette partiti IA.** La valence si tara perché il sondaggio del turno zero riproduca le intenzioni di voto reali di ottobre 2026, sommate per posto sull'asse con le collocazioni CHES. La corrispondenza fra partiti veri e posti si fa una volta, offline, e non entra nei dati del gioco, che restano di partiti inventati (`docs/ricerca/sintesi/decisioni.md:31-35`). Vocazioni e regole di movimento sono i default di Partiti IA (18). Leader, nomi e caricature sono contenuto da scrivere dopo la spec.

**Vincibilità.** Lo scenario di partenza è tarato perché almeno una coalizione col giocatore superi il 42% nei sondaggi attesi, da ogni posto: favorita, non certa. Il seme muove il rumore, non i valori attesi, quindi nessun ritiro del seme. Nell'editor (21) uno scenario senza coalizione vincibile resta lecito, con un avviso.

Effetti sulla mappa:
- la patch di nebbia «Scenario iniziale» è graduata in questo ticket, in Editor dello scenario (21), in Valori di partenza delle disposizioni (22, research) e in Stato del paese al turno zero (23, research);
- Out of scope: la riga «Fase da outsider» è precisata, perché fondazione e prima campagna, ridotte a una schermata e un turno, entrano nella v1;
- Campagna elettorale (nebbia): il turno zero è anche un turno di campagna, il primo della partita;
- Costo del governare (nebbia): durata e decadimento della luna di miele del giocatore;
- Editor dello scenario (21): l'avviso di vincibilità.
