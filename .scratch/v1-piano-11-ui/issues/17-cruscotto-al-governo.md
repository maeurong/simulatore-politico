Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatto il cruscotto quando il giocatore governa? In testa l'evento aperto con le opzioni e il default (storie 130-131); poi ciò che chiede una scelta nel turno: proposte dei ministri con accogli, rinvia, respingi (storie 30-31, `anteprima_accogli`), ultimatum (storia 83), emendamenti (storia 63), voti a fine turno (storia 62), bilancio da presentare, campagna da fare, ciascuno col rimando alla sua sezione; i ministri in evidenza (storia 27) col rimando a Governo; il conto delle scelte che vanno al default; il corsivo. Varianti: crisi con la banda nera e «allarga o dimettiti» (storia 84, `Vista.crisi`, `disponibilita`), dimissioni annunciate (`Vista.dimissioni`), esercizio provvisorio, turno zero con le sole scelte di coalizione e campagna, tono grave.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario l'11/11/2026 in un giro su un prototipo navigabile: [17-cruscotto.prototype.html](../prototipi/17-cruscotto.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: le Viste 01, 03, 04, 05, 08 e [viste/17-cruscotto.json](../viste/17-cruscotto.json), che `genera/` aggiunge giocando gli stati che mancavano: ultimatum ed esercizio provvisorio (seme 1, turno 13), crisi in esercizio provvisorio (seme 14, turno 37), dimissioni annunciate e scelte già fatte (la 03 con `Dimettiti`, e con `Rinvia` e `RispondiEvento`), più `anteprima_accogli` vera sulla proposta di Infrastrutture della 03.

### Forma: A · Ordine del giorno

Sotto la cornice, le dodici colonne col **dettaglio a lato**.

- **In testa**, a tutta colonna: l'evento aperto (famiglia e regione, nome e costo, la causa, accanto il fondo di riserva rimasto, le opzioni come bottoni col default segnato, e sotto «Senza scelta, alla chiusura: …»). In crisi, prima dell'evento, il riquadro «Allarga la maggioranza o dimettiti». Con le dimissioni annunciate, un riquadro che lo dice. Un evento con una sola opzione (lo scandalo della 05) non chiede niente e lo dice.
- **Sotto, una colonna di righe per gruppo**: «Da decidere» (bilancio da presentare, successori dei ministri in interim), «Proposte dei ministri», «Emendamenti degli alleati», «Si vota a fine turno», «Da tenere d'occhio» (gli ultimatum). Ogni gruppo dice quante scelte vanno al default. Ogni riga ha un segno (vuoto, pieno se scelta, a retino se urgente), il titolo, sotto il ministro o il fatto e **il suo default**, e a destra le azioni sulla riga: Accogli · Rinvia · Respingi, Accogli · Respingi, il nome dei candidati della rosa, «Poni la fiducia», oppure il rimando alla sezione («Bilancio ›», «Partiti ›»).
- **Ministri in evidenza** (storia 27) in fondo, col rimando a Governo: quelli con `agenda.in_evidenza` (minaccia, distanza o evento, `crates/sim-core/src/ministri/agenda.rs:570`), con l'atto e, se in preavviso, la soddisfazione in rosso.
- **Dettaglio a lato** della riga scelta: per una proposta di fondi il ministro, il motivo e l'anteprima del decreto (costo, effetti in fasce col verso e il ritardo, forbice di Camera e Senato sulla conversione) con le tre azioni; per una riforma, che entra in calendario come legge, e il rimando all'anteprima della misura ([Anteprima della misura](20-anteprima-della-misura.md)); per un voto, la forbice e «in bilico»; per il resto, motivo e causa.
- **Il conto** delle scelte al default sta nella cornice («4 scelte vanno al default ›», «nessuna scelta al default») e si aggiorna a ogni scelta.

### Decisioni sui casi

- **«Dimettiti» con la conferma sul posto**: il bottone diventa «Conferma: dimettiti» e «Annulla». Il core non lo ritira (`Partita::dimettiti`, `crates/sim-core/src/partita.rs:3535`), quindi si aggiunge alle tre conferme di [Ossatura e navigazione](07-ossatura-e-navigazione.md) (riavvolgimento, abbandono, eredità).
- **Il default su ogni riga**, oltre al conto in cornice: chiudere senza clic è una scelta informata (storia 18).
- **Allarga**: i partiti disponibili si spuntano, gli altri restano a tratteggio con la causa; «Allarga a X e Y» compare quando ce n'è almeno uno. In crisi in esercizio provvisorio la caduta è certa e il riquadro lo dice senza bottoni; restano successori ed evento.
- **Una proposta già rinviata** ha «Rinvia» spento con «già rinviata una volta»; il suo default è «resta in sospeso, poi vale respinta» (`crates/sim-core/src/ministri/proposte.rs:277`).
- **Fondo di riserva**: paga fino al residuo dell'anno, e l'effetto vale in proporzione (`crates/sim-core/src/eventi/mod.rs:67`). Nelle partite giocate è quasi sempre 0 € mentre il default dell'evento è «attingere al fondo»: per questo il residuo sta accanto al costo, in rosso se non basta. La UI non calcola la quota coperta.
- **Turno zero**: due righe, «Dichiara la coalizione» coi disponibili e «Fai la campagna», tutte e due col rimando a Campagna. Le proposte dei ministri del governo uscente non si mostrano.
- **Tono grave**: lo stesso cruscotto con le regole delle fondamenta (iniziali, niente testata né colori d'accento).
- **Nessun corsivo**: tagliato dalla v1 ([Testi del Pungolo](09-testi-del-pungolo.md)).

### Guscio e dati

Nessun dato nuovo, nessun tocco al core. Le scelte passano da `comanda` (`RispondiEvento`, `Accogli`, `Rinvia`, `Respingi`, `Emendamento`, `ScegliSuccessore`, `PoniFiducia`, `Allarga`, `Dimettiti`); l'anteprima del decreto da `anteprima_accogli`, che il guscio ha già (`app/src-tauri/src/lib.rs:392`). Il conto delle scelte al default lo fa la UI contando le voci aperte della Vista, senza simulare niente.

### Scartato

B · Tre colonne (per scadenza, senza dettaglio a lato: col bilancio la colonna centrale scorre); C · Pratica per pratica (indice numerato e una pratica alla volta: un turno tranquillo costa troppi clic, storia 18); «Dimettiti» diretto; il default solo nel conto della cornice.

