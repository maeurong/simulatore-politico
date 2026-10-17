Type: grilling
Status: resolved
Blocked by: 08, 09
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa fa il giocatore quando il suo partito è all'opposizione: quali leve ha (mozioni, sfiducia, voto sulle leggi del governo, alleanze per il voto successivo), cosa vede e governa ancora (niente dicasteri né budget?), come torna al governo (elezioni, crisi del governo avversario, ribaltone a suo favore), quanto dura un turno all'opposizione, e quando finisce la partita, se finisce (partito fuori dal parlamento? abbandono)?

Ingresso da Coalizione e crisi (05): ogni perdita del governo, cioè sfiducia, sconfitta elettorale e ribaltone, porta all'opposizione e non al game over. Il governo avversario è guidato dai partiti IA con le stesse regole di coalizione, intesa e crisi.

Ingresso da Iter parlamentare (08): l'opposizione vota con la regola della linea più un malus fisso, non presenta emendamenti al governo; può chiedere il referendum confermativo con 1/5 dei seggi. Restano a questo ticket il referendum abrogativo (art. 75) e le leggi proposte dall'opposizione.

Ingresso da Giro del turno (09): il turno di governo ha tre fasi (apertura con l'esito, decisioni in ordine libero, chiusura in ordine fisso atti → aula → motore → soglie → eventi), si chiude anche senza clic e ha tre indicatori sintetici (consenso, maggioranza, spread). Da decidere quanto di questo scheletro vale all'opposizione. Dopo le elezioni la trattativa occupa le decisioni del turno successivo, col governo uscente in interim.

## Answer

Grilling con Mario del 17/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Fatti istituzionali in `docs/ricerca/modelli/iter-legislativo-italiano.md`. Termini nuovi in `CONTEXT.md`: governo avversario, sfiducia, referendum abrogativo, eredità; precisati opposizione, linea e calendario. Il difetto da evitare è l'opposizione finta di Democracy 4, opposta al 100% su tutto (`docs/ricerca/giochi/democracy-4.md:39`).

**Turno.** Stesso turno trimestrale, perché l'economia non cambia passo, e stesso scheletro a tre fasi del ticket 09.
- **Apertura:** esito, eventi, misure del governo avversario in calendario, sondaggio.
- **Decisioni:** le leve sotto.
- **Chiusura:** identica (atti dei ministri del governo avversario → aula → motore → soglie → eventi).

Obiettivo 2-4 minuti. Il comando **«avanti fino a novità»** concatena chiusure senza clic e si ferma quando serve una scelta: voto su una misura che tocca l'elettorato del giocatore, ultimatum nella maggioranza avversaria, crisi, finestra del referendum, turno prima del voto. Il turno della trattativa dopo una sconfitta passa da solo, e l'esito mostra il nuovo governo (coalizione, spartizione, listino dei ministri). Il primo turno vero all'opposizione è quello dopo.

**Cosa vede.** Tutto ciò che è pubblico:
- indicatori del paese, sondaggio del trimestre, registro;
- misure del governo avversario in calendario, con l'anteprima del ticket 09 e le linee dei partiti;
- intesa degli alleati del governo avversario in tre fasce (solida, incrinata, in ultimatum), con gli ultimatum e la loro causa.

Non vede le agende dei ministri prima che vengano eseguite (le scopre nell'esito) né i numeri esatti dell'intesa. Nessun dicastero, budget o fondo di riserva da gestire.

**Indicatori sintetici**, negli stessi tre posti:
- **consenso:** intenzione di voto del partito del giocatore, più la coalizione se già dichiarata;
- **maggioranza:** margine del governo avversario, con gli alleati in ultimatum segnati;
- **conti:** spread.

Restano data e turni al voto.

**Leve.**
- **Linea del proprio partito.** La decide il giocatore, misura per misura; il default è la regola del ticket 08 col malus d'opposizione. Votare lontano dalla propria posizione produce franchi tiratori nel suo partito, crescenti con la distanza. Ogni voto sposta la posizione percepita del partito (06).
- **Sfiducia** al governo o a un singolo ministro.
  - *Chi la presenta:* il giocatore, se ha 1/10 dei seggi, anche contando le firme di altri partiti d'opposizione (art. 94, `docs/ricerca/modelli/iter-legislativo-italiano.md:43`).
  - *Voto:* palese a blocchi come nel ticket 05. Un alleato del governo avversario la vota solo se è in ultimatum e se la caduta gli rende di più che restare, con lo stesso valore atteso del ticket 05 (ribaltone con la sua richiesta, oppure elezioni con i suoi sondaggi). Lo si vede prima del voto.
  - *Se è respinta:* la maggioranza si ricompatta (intesa degli alleati in salita) e il giocatore perde un po' di valence. Nessun limite di frequenza.
  - *Sfiducia individuale:* stesse firme della sfiducia al governo (Camera art. 115, Corte cost. 7/1996, `docs/ricerca/modelli/iter-legislativo-italiano.md:130`). Se passa, il ministro si dimette e il suo partito subisce il colpo secco d'intesa del ticket 05.
- **Proposta di legge.** Una in esame alla volta, dal catalogo delle misure (16), esclusi bilancio e decreti. Va nel posto del calendario riservato alle opposizioni, uno al trimestre, a rotazione fra i partiti d'opposizione, e si vota con la regola della linea. Il posto riproduce le regole vere: alla Camera le opposizioni hanno 1/5 degli argomenti (art. 24, `iter-legislativo-italiano.md:107`); al Senato una proposta firmata da 1/3 dei senatori entra «uno ogni tre mesi» (art. 53, `:110`). Passa di rado: diventa legge l'1,1% delle proposte dei parlamentari (XVIII legislatura, `:114`). Serve a costringere la maggioranza a votare contro misure care ai suoi gruppi, e così si muovono posizione percepita e giudizio (06).
- **Referendum abrogativo** (art. 75, `docs/ricerca/fonti/normattiva-costituzione-art75.md`).
  - *Su cosa:* una legge in vigore, anche di un governo precedente o del giocatore stesso; mai imposte, bilancio o amnistia.
  - *Firme:* raccolte in un turno, riuscite sempre. Un referendum pendente alla volta.
  - *Calendario:* una finestra all'anno, nel turno aprile-giugno. È un dettaglio aggiunto dopo il grilling, preso dalla l. 352/1970 e non ridiscusso con Mario (`iter-legislativo-italiano.md:119-125`):
    - la richiesta si deposita entro il turno luglio-settembre e si vota nel turno aprile-giugno dell'anno dopo;
    - niente richieste nell'ultimo anno di legislatura;
    - lo scioglimento anticipato sospende un referendum già indetto;
    - nessun vaglio di ammissibilità, perché la Corte non c'è (08).
  - *Voto:* votano i gruppi come nel confermativo (distanza dalla misura più giudizio sul governo), e il quorum degli aventi diritto si calcola con l'astensione endogena (06). Il governo può invitare all'astensione.
  - *Esito:* se la legge è abrogata, gli effetti cessano come per un decreto decaduto e il governo perde valence. Se manca il quorum, perde un po' il proponente.
- **Referendum confermativo.** Se il partito del giocatore ha 1/5 dei seggi, da solo o coi partiti contrari, il gioco lo propone e il giocatore sceglie. Se non sceglie, vale la regola del ticket 08.
- Le leve sono simmetriche: i partiti IA all'opposizione le usano contro il giocatore quando governa (ticket 18).

**Ritorno al governo.**
- **Elezioni:** a scadenza o anticipate, con la coalizione pre-elettorale del ticket 05.
- **Ribaltone a suo favore.** Quando il governo avversario cade, il Capo dello Stato cerca una maggioranza di partiti contigui (05). Se il partito del giocatore ne è il primo, il giocatore diventa Presidente del Consiglio e passa alla trattativa del ticket 05, senza la fase pre-elettorale. Se non è il primo, può chiedere la guida come condizione: il partito più grande accetta se il suo valore atteso supera quello delle elezioni. Il giocatore può anche rifiutare e scommettere sul voto.
- **Mai alleato minore** in un governo altrui nella v1 (punti di aggancio).

**Fine della partita.** Nessun termine fisso. Finisce quando il partito del giocatore resta senza seggi in entrambe le Camere, perché senza seggi non ci sono più leve; con seggi in una sola Camera, o ripescato in coalizione, si continua. Finisce anche per abbandono, in qualunque turno. In entrambi i casi chiude l'**eredità**: anni al governo, leggi ancora in vigore, indicatori confrontati col turno zero. Il giocatore resta sempre leader del suo partito: niente congressi né sfide interne nella v1.

**Fuori da questo ticket.** Piazza, media, lobbying e riposizionamento del partito sull'asse (Power & Revolution, `docs/ricerca/giochi/power-and-revolution.md:23`) vanno alla nebbia Campagna elettorale. Il giocatore come alleato minore e i congressi di partito vanno ai punti di aggancio.

**Ne escono:**
- Partiti IA (18), ticket nuovo dalla nebbia: governo avversario, leve dei partiti IA contro il giocatore, posizione nel tempo.
- Crisi ed eventi (15): la sfiducia individuale è anche una leva, non solo un evento.
- Costo del governare (nebbia): perché non convenga perdere apposta, costi di sfiducia respinta e quorum mancato, franchi tiratori del giocatore, 2-4 minuti per turno d'opposizione.
- Informazione imperfetta (nebbia): quanto si legge dell'intesa avversaria oltre le tre fasce.
- Salvataggi (nebbia): l'eredità.
