Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Aula? Il calendario con tre posti ordinari, uno costituzionale e quello riservato all'opposizione, i decreti in precedenza, il bilancio che lo occupa nel suo turno (storia 61); le misure in esame con linee dei partiti e forbice prima del voto, conteggio dopo (storia 62); emendamenti (storia 63); la fiducia con chi voterebbe no (storie 64-65); l'elenco delle disposizioni con valore in vigore, legge d'origine, precedente e in arrivo (storia 58), da cui si presenta una legge di riforma (storia 57) o una norma costituzionale (storia 68); il menu a gradini della legge elettorale con le impostazioni pronte e la tabella dei seggi (storie 71-73); referendum in attesa ed esiti. Variante all'opposizione: il posto riservato e la proposta (storia 94). L'anteprima che si apre a lato è di [Anteprima della misura](20-anteprima-della-misura.md).

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario l'11/11/2026 in due giri su un prototipo navigabile: [19-aula.prototype.html](../prototipi/19-aula.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: le Viste 03, 04, 05, 08, lo stato `ultimatum` di [viste/17-cruscotto.json](../viste/17-cruscotto.json) e [viste/19-aula.json](../viste/19-aula.json), che `genera/` aggiunge giocando: dal 03 presenta la legge elettorale «proporzionale al 3%», la norma costituzionale sul numero dei parlamentari (300 e 150) e il salario minimo a 11 €/h, e riempie il calendario (`piena`); al turno dopo gli emendamenti di Centrodestra e Destra e la seconda deliberazione (`dopo_voto`, `seconda_deliberazione`); al turno 4 la legge elettorale approvata, con «L. 1/2027» sulle disposizioni (`dopo_elettorale`); all'opposizione (seme 2, a 1,5) il posto riservato libero, poi la proposta sulla cittadinanza e una linea scelta. Più le anteprime vere della legge elettorale e della riforma, e due rifiuti veri del core.

### Forma: D · Misto

Nata dal primo giro: Mario ha scelto l'ossatura di A, voleva l'emiciclo di B («mi piace moltissimo») e trovava C più professionale e completa, ma complicata per un giocatore medio.

- **Ossatura di A · Ordine dei lavori.** Sotto la cornice, le dodici colonne col dettaglio a lato. In testa **il calendario a posti**: Decreti (in precedenza), Ordinaria 1/3, 2/3, 3/3, Costituzionale, Opposizione. Ogni posto dice la misura e quando si vota; un posto libero lo dice («libero · scrivi una legge»; all'opposizione «libero: porta una proposta», bordato di rosso). Nel turno del bilancio la fila diventa un posto solo, «Legge di bilancio · occupa il calendario del turno»; le misure già in esame restano nell'elenco. Sotto, **quattro schede**: In aula (con il numero), Disposizioni, Legge elettorale, Referendum.
- **In aula:** una riga per misura, col segno (a retino se in bilico), nome, forma (legge ordinaria, legge elettorale, costituzionale con la deliberazione, conversione, bilancio), chi la porta e quando si vota; a destra le linee dei partiti in caselle da sinistra a destra (pieno = sì, il giocatore in rosso al suo posto sull'asse), la barra della forbice alla Camera con la riga della maggioranza, «in bilico», gli emendamenti aperti, «fiducia». Sotto, **Ultimi voti**: il conteggio esatto di Camera e Senato, approvata, respinta o decaduta col motivo.
- **Dettaglio a lato di una misura, con l'emiciclo di B:** Camera e Senato affiancati, i seggi pieni per i sì sicuri, a retino per la forbice dei franchi tiratori, vuoti per i no, la riga rossa sulla maggioranza; sotto «Camera · 179–193 sì · ne servono 201». Poi la linea di ogni partito, la tabella dei seggi per una legge elettorale, la regola delle due deliberazioni per una costituzionale, gli emendamenti degli alleati con Accogli · Respingi e il motivo, la questione di fiducia con chi voterebbe no, la linea del giocatore, «Anteprima della misura ›» ([Anteprima della misura](20-anteprima-della-misura.md)). Le barre restano nelle righe dell'elenco.
- **Disposizioni, a righe come C ma chiuse:** raggruppate per dicastero, la Costituzione in fondo; ogni riga dice nome, legge, valore in vigore, valore precedente se diverso, valore in arrivo con la data, «in esame» se una misura la tocca, e «Cambia ›» (all'opposizione «Proponi ›»). I gradini si aprono **solo sulla riga scelta** (o con un valore già in bozza): in vigore a fondo pieno, in bozza in giallo, in arrivo a tratteggio blu, bloccato da un referendum a retino. Il dettaglio a lato tiene i dati della disposizione e la bozza.
- **La bozza** nel dettaglio a lato: i valori scelti («ora …» accanto), l'anteprima (costo annuo, effetti a fasce col ritardo, linee, forbice, seggi per l'elettorale) e il bottone che cambia nome: «Presenta la legge di riforma», «Presenta la legge elettorale», «Proponi la norma costituzionale», all'opposizione «Porta nel posto riservato». Una legge può toccare più disposizioni (storia 57); una norma costituzionale non si mescola alle altre.
- **Legge elettorale:** le nove disposizioni coi gradini tutti aperti, sono poche; in testa le impostazioni pronte (Legge 180/2026, Proporzionale al 3%, Proporzionale puro), che riempiono la bozza con ciò che cambia. La soglia del premio parte dal 40%: i dati non hanno gradini sotto.
- **Referendum:** quelli in attesa (tipo, promotore, firme o indetto, quando), al governo «Invita all'astensione» sull'abrogativo, la scelta sul confermativo quando c'è; gli esiti. All'opposizione «Promuovi un referendum abrogativo» sta nel dettaglio di una disposizione abrogabile.

### Decisioni sui casi

- **Questione di fiducia con la conferma sul posto:** «Poni la fiducia» diventa «Conferma: poni la fiducia» e «Annulla». Il core non la ritira: l'intesa cala subito e la misura si vota nel turno (`crates/sim-core/src/partita.rs:1539-1546`). È la quinta conferma, dopo riavvolgimento, abbandono, eredità ([Ossatura e navigazione](07-ossatura-e-navigazione.md)) e dimissioni ([Cruscotto al governo](17-cruscotto-al-governo.md)); **vale anche per «Poni la fiducia» sulla riga del cruscotto**. Si pone solo sulle misure del governo (`crates/sim-core/src/parlamento/emendamenti.rs:251`): nessun bottone sulle proposte dell'opposizione, anche se la Vista porta chi voterebbe no.
- **Emendamento lasciato aperto:** resta aperto e si vota il testo com'è, senza il costo del respinto (`crates/sim-core/src/parlamento/mod.rs:377-381`). È il default scritto sotto ogni emendamento («Senza scelta: resta aperto, si vota il testo com'è»), e conta nelle scelte al default della cornice.
- **La linea del giocatore** (Sì · No · «Come da posizione (sì)», che è il default) all'opposizione su ogni misura, al governo sulle proposte dell'opposizione (`VoceMisura.linea_giocatore`).
- **Calendario pieno è un rifiuto sul posto**, come lo dice il core («calendario pieno»): il bottone della bozza resta, con la causa accanto; vale anche per il posto riservato già occupato.
- **Tono grave:** la stessa Aula con le regole delle fondamenta.

### Rimandi

- **Anteprima della proposta d'opposizione — al core.** All'opposizione `Partita::anteprima` rifiuta («non governi: sei all'opposizione»): la proposta per il posto riservato si scriverebbe senza costo, linee né forbice, che arrivano solo quando è già in calendario. Mario sceglie il rimando: l'anteprima vale anche per la proposta d'opposizione, come quinta voce minore della PR delle esposizioni del piano di core «esito e Vista completa»; l'Aula la mostra in 11g. Fino ad allora, nella bozza all'opposizione il rifiuto si legge al posto dell'anteprima.
- **Posti in calendario e combinazioni spente — a 11g**, già decisi ([Dati senza piano](11-dati-senza-piano.md), [Contratto del guscio](12-contratto-del-guscio.md)): il prototipo conta i posti da sé solo per mostrarli, segnati «11g»; i gradini impossibili della legge elettorale li spegne `menu_elettorale`.

### Guscio e dati

Nessun dato nuovo oltre ai rimandi, nessun passacarte nuovo. Le scelte passano da `comanda` (`PresentaLegge`, `PresentaProposta`, `Emendamento`, `PoniFiducia`, `Linea`, `PromuoviReferendum`, `InvitaAstensione`, `Confermativo`); l'anteprima della bozza da `anteprima`, che il guscio ha già (`app/src-tauri/src/lib.rs:379`).

### Stranezze notate

- La norma costituzionale entra con la prima deliberazione già nel turno della presentazione (`deliberazione` 1, si vota a fine turno). La durata della legislatura a 4 anni, nella prima prova, è caduta 78 a 322; i 300 e 150 sono passati alla prima (219 sì, sotto i due terzi) e caduti alla seconda, quindi nessun referendum confermativo è stato raggiunto giocando.
- Lo stato `ultimatum` del 17 ha `alleati_contro_fiducia` pieno su due proposte dell'opposizione, dove la questione di fiducia non si può porre.
- In ogni Vista campione al governo il posto riservato è occupato da una proposta dell'IA, spesso due (una per turno di voto).

### Scartato

A da sola (manca l'emiciclo); B · Emiciclo (il voto al centro e il codice a parte: l'emiciclo è passato nel dettaglio); C · Codice (le 46 disposizioni coi gradini tutti aperti: completo ma troppo per un giocatore medio, la riga chiusa con «Cambia ›» ne tiene la struttura); la forbice solo a barra nel dettaglio; «Poni la fiducia» a clic diretto; la proposta d'opposizione senza anteprima.

