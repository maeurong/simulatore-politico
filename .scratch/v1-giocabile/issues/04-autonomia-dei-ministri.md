Type: grilling
Status: resolved
Blocked by: 03
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Fin dove arriva l'autonomia di un ministro: come sceglie gli atti del ministro (capacità, principi, obiettivi), cosa può bloccare o revocare il Presidente del Consiglio e a quale costo, quando un ministro propone un decreto o una legge, quando si dimette o va sostituito?

## Answer

Grilling con Mario del 16/10/2026, tre round, tutte le raccomandazioni accettate. Termini nuovi in `CONTEXT.md`: capacità, posizione (sostituisce «principi»), obiettivo, soddisfazione, agenda, interim, rodaggio, ministro proponente.

**Il ministro** ha quattro grandezze:
- **capacità** 1-5: moltiplica l'effetto dei suoi atti e ne divide il ritardo; fissa per ogni personaggio nella v1, salvo il rodaggio;
- **posizione** sull'asse a sette punti: quella del partito, più uno scarto personale (i tecnici hanno solo la propria);
- **obiettivo**: un indicatore del suo dicastero da portare a una soglia;
- **soddisfazione** 0-100.

**Agenda e scelta degli atti.** A inizio turno ogni ministro annuncia un atto. Fra i suoi 4 che il margine discrezionale può pagare, sceglie quello con il punteggio più alto: quanto avvicina l'obiettivo, più quanto è vicino alla sua posizione, più un piccolo rumore. Il motivo si mostra in una riga. A margine esaurito non fa nessun atto.

**Leve del Presidente del Consiglio sull'agenda.** Può *fermare* un atto (l'atto salta e il margine resta) oppure *dirottarlo* su un altro dei 4. Entrambe le leve costano soddisfazione, in proporzione a quanto l'atto scartato era vicino alla posizione e all'obiettivo del ministro; dirottare costa meno se l'atto imposto è compatibile.

**Proposte normative.** Ogni ministro ha al massimo una proposta in sospeso. Ci sono tre inneschi:
1. il margine non basta per l'obiettivo: il ministro chiede fondi con un decreto di variazione;
2. arriva un evento nel suo dicastero: propone un decreto;
3. la posizione lo spinge: propone una legge di riforma dal catalogo del dicastero.

Il Presidente del Consiglio accoglie (si passa all'iter del ticket 08), rinvia una volta senza costo o respinge, e respingere costa soddisfazione. Il Presidente del Consiglio può anche lanciare norme proprie, ma ogni norma ha un **ministro proponente**, quello del dicastero competente: firma sempre e perde soddisfazione in proporzione alla distanza dalla sua posizione.

**Soddisfazione.** Scende con sei cause: atto fermato o dirottato, proposta respinta, norma firmata contro la propria posizione, budget del dicastero tagliato nella legge di bilancio, indicatore di servizio in calo. Sale quando il ministro raggiunge l'obiettivo. Ogni turno torna piano verso un valore di base, e ogni variazione si mostra con la sua causa.

**Dimissioni.** Le regole sono deterministiche e visibili:
- sotto 40 il ministro minaccia di dimettersi e indica la causa principale;
- se a fine turno è sotto 25, si dimette.

Non esiste la revoca: il Presidente del Consiglio **chiede le dimissioni**. A un tecnico non costa nulla; con un ministro di partito è un colpo diretto allo scontento di quel partito. La sfiducia individuale del parlamento (Corte cost. 7/1996) arriva come evento.

**Successione.** Se il dicastero è nella quota di un partito, il partito propone una rosa di 3 nomi; se era di un tecnico, si sceglie fra 3 tecnici oppure lo si passa a un partito. Per un turno il dicastero resta in **interim** (nessun atto, solo spesa di funzionamento). Segue un turno di **rodaggio** a capacità ridotta.

**Raccordo con il ticket 05.** La soddisfazione dei ministri di un partito è uno degli ingressi del suo scontento; i pesi li decide il ticket 05. I tecnici non toccano nessun partito.

Soglie, pesi e costi sono provvisori e vanno al bilanciamento. Effetti sulla mappa:
- si sblocca il ticket 09;
- la nebbia del catalogo si allarga ai punteggi dei 40 atti e alle leggi di riforma di ogni dicastero;
- la sfiducia individuale va fra crisi ed eventi.
