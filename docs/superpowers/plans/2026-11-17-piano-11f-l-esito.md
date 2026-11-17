# Piano 11f «L'esito»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** dopo la chiusura il giocatore legge le conseguenze come un giornale. La prima pagina rivela le voci una per volta, e i tracciati scattano mentre la rivelazione avanza. Il registro ritrova qualunque voce di qualunque turno e permette di tornare indietro. All'opposizione «Avanti fino a novità» salta i turni vuoti, e lo schema dello Stato numera i fili come la prima pagina. La fetta porta Prima pagina, Registro, «Avanti fino a novità», i testi del Pungolo, le vignette e i tondi dello schema. Chiude tutti i segnaposto «11f».

**Architettura:**
- Ogni schermata è una cartella in `app/src/lib/<sezione>/`, con dentro un modulo `.ts` puro, provato con Vitest sulle Viste campione, e i componenti Svelte. Il modulo puro legge la Vista e le voci dell'esito e decide cosa mostrare.
- Lo stato della rivelazione (la Vista di prima, il cursore, la corsa di «avanti») e la cache del registro vivono in due moduli `.svelte.ts` condivisi fra cornice e sezioni, accanto a `partita.svelte.ts`.
- Il guscio ha già `registro` (`app/src-tauri/src/lib.rs:346`) e `guscio.registro` (`app/src/lib/guscio/chiamate.ts:61`). `sim-core`, la Vista e il guscio Rust non cambiano.
- Il guscio finto impara `registro(t)` e la catena delle chiusure dai campioni 25 e 16, così rivelazione, registro e corse si provano nel browser.
- I testi del Pungolo sono già scritti e tagliati da Mario nel file accanto a questo piano (decisione 3): l'esecuzione li copia e non scrive prosa.

**Tecnologie:** quelle della 11e (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2). Nessuna dipendenza nuova. Il solo Rust toccato è il generatore delle Viste campione in `.scratch/` (Task 1).

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 144-157 (righe 196-211), i vincoli alle righe 248-371, la sezione della fetta alle righe 404-417, le conferme alla riga 454 e i test alle righe 456-477. Valgono anche i rimandi alla 11f delle fette prima: righe 351, 355, 400 e 401. La PR di questo piano corregge la spec dove le decisioni del 17/11 la cambiano (righe 356, 409, 415-417, 496). Chi esegue legge la spec e il ticket citato nel task.

**Testi:** [`2026-11-17-piano-11f-testi.md`](2026-11-17-piano-11f-testi.md), con 182 titoli per 79 chiavi, 14 didascalie e 14 battute a biro, tagliati da Mario nella PR di questo piano. Fa testo la versione fusa su main.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `16-prima-pagina.prototype.html`, forma A · Giornale composto, con gli stati `ordinario`, `quieto`, `voto`, `grave`, `fermata` e la `riv` a `chiusura` o `riprendi`. Le voci e i titoli del prototipo sono scritti a mano: le voci vere vengono dalla Vista, i titoli dal file dei testi. I disegni sono segnaposto e le vignette vere sono quelle della PR 4.
- `25-registro.prototype.html`, forma D · Indice col filo, con gli stati `governo`, `grave`, `finita` e `zero`. La tabella `RIGA` è la base della tabella delle sezioni (lettura 7), `testoConferma` quella del testo del riavvolgimento.
- `37-scelte-11e-schema.prototype.html`, su riga · pettine · punta: i tondi sui fili e sui nodi, l'elenco «Le notizie del trimestre», il filtro, la battuta nel dettaglio a lato (`:747`).
- `38-scelte-11f-esito.prototype.html`: le quattro decisioni del 17/11. Scheda 1 per le cinque scene e i tipi che servono, scheda 2 per il posto della battuta.

I ticket che ogni task cita stanno in `.scratch/v1-piano-11-ui/issues/` (09, 16, 25, 29). I testi neutri dell'interfaccia si copiano dai prototipi. Un testo neutro che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR. I testi del Pungolo vengono solo dal file dei testi.

**Da dove si parte:** main a `36db656`, con la 11e fusa (#76-#79) e la CI (#80). Oggi Prima pagina e Registro sono segnaposto della 11f (`app/src/lib/ossatura.ts:62-64`, `fettaDi`). «Chiudi il turno» apre la casa del turno nuovo (`app/src/lib/partita.svelte.ts:76-80`) e «Riprendi» fa lo stesso (`app/src/routes/+page.svelte:96`). La cornice disegna i tracciati dalla Vista corrente (`app/src/lib/cornice/Cornice.svelte:38`). Lo schema non ha tondi, filtro né battute (`app/src/lib/schema/Schema.svelte:2`). La scheda di un partito nato in partita ha il segnaposto dell'origine (`app/src/lib/partiti/SchedaPartito.svelte:43`). La bozza dell'eredità ha solo «Torna al turno N · prima del voto» (`app/src/lib/eredita/Bozza.svelte:13-48`). Le Viste campione 16 e 25 sono allo schema 19, rigenerate il 15/11/2026 (`viste/README.md:5`). Nella Vista non serve nessun campo nuovo (decisione 4).

## Vincoli globali

Valgono tutti i vincoli globali del piano 11e (`docs/superpowers/plans/2026-11-17-piano-11e-opposizione-e-fine.md:29-40`). Qui sotto quelli che la fetta tocca di più, e quelli nuovi.

- **La UI legge, il core decide** (spec:250). La UI può ordinare le voci per criticità, precedenza e ordine della chiusura (la voce di testa, spec:406), contare le voci aperte (le critiche fuori pagina) e scegliere un testo con l'hash. Non sceglie le sette notizie (`in_evidenza`), non deduce un fatto che la voce non dice, non decide da sola quando «avanti» si ferma: si ferma quando `Vista.fermate` non è vuota.
- **Variante del titolo** (spec:319): `(hash(id della partita, chiave) + turno + posto fra le voci dello stesso tipo nella pagina) mod n`. Non usa `variante` di `t.ts:64`, che fa l'hash di tutte le parti: serve una funzione sua. L'id della partita è `IndicePartita.id` (`partita.indice()`).
- **Una chiave per tipo, anche vuota** (storia 180): titoli e sezioni sono `Record` sul tipo esportato `TipoVoce`. Un tipo nuovo del core fa fallire `npm run check`.
- **Tono grave della pagina** (spec:413, storia 169): la prima pagina è grave se una voce dell'esito ha `tono: "grave"` o se un evento grave è aperto. Nel tono grave niente vignetta né titolo spiritoso: l'occhiello sale a titolo e la testata dice «Esito del trimestre». Lo schema usa la stessa regola per far tacere le battute.
- **Movimento** (spec:328): la rivelazione mette una voce ogni 1,1 s × `k`. Si animano solo `opacity` e `transform`, e le caselle che scattano lampeggiano in giallo con `steps()`. Col movimento spento (`k = 0`) la pagina arriva già composta e i tracciati sono già al valore nuovo.
- **Conferme** (spec:454): «Torna al turno N» si conferma sul posto, con ciò che si cancella e il numero del riavvolgimento. «Avanti fino a novità», «Chiudi il turno», «Salta la rivelazione» e «Vai al cruscotto» partono senza conferma.
- **Cargo da un worktree**: `--manifest-path`, `CARGO_TARGET_DIR=<wt>/target`, percorsi assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md:106`).

## Decisioni di Mario del 17/11/2026

Prese sul prototipo `38-scelte-11f-esito`, in una sola domanda.

1. **Vignette: cinque scene**, ciascuna con la didascalia del suo tipo. Le scene servono quattordici tipi di voce di testa e coprono l'85% delle prime pagine ordinarie (conteggio: 80 partite automatiche, 3.124 prime pagine):
   - Aula: `sfiducia_approvata`, `bilancio_approvato`, `legge_respinta`, `sfiducia_presentata`, `legge_approvata`;
   - Giuramento: `governo_formato`;
   - Emergenza: `evento_risolto` ed `evento_nato` della famiglia `emergenza`;
   - Lettera: `ultimatum_posto`, `minaccia_di_dimissioni`, `minaccia_di_scissione`, `ministro_esce`;
   - Cassa: `evento_risolto` della famiglia `situazione`, `reddito_deluso`.

   Un altro tipo va in testa a tutta larghezza. Il voto ha le barre (spec:408).
2. **Battuta a biro dello schema solo nel dettaglio a lato** del filo, come il 37 (`:747`), mai sulla tavola.
3. **Testi del Pungolo tagliati nel piano.** I titoli, le didascalie e le battute stanno nel file dei testi accanto a questo piano, tagliati da Mario nella PR del piano. L'esecuzione li copia in `pungolo.json`. Ciò che Mario ha tagliato non entra, e l'occhiello sale a titolo.
4. **Viste campione: lo schema 19 basta, nessun campo nuovo.** Il generatore cerca giocando l'avviso `bilancio_ricondotto`, cioè un bilancio presentato in disavanzo sotto il MES. Se non lo trova, l'avviso si prova in Vitest con una voce scritta a mano e dichiarata.

## Letture che la spec non fissa

Le ho decise scrivendo il piano. Mario le conferma col sì all'esecuzione, o le cambia prima.

1. **La Vista di prima di una corsa di «avanti»** è quella prima dell'ultima chiusura, non quella del clic. I tracciati che scattano spiegano le voci della pagina mostrata; i turni saltati stanno nel registro, e la banda lo dice.
2. **La chiusura che finisce la partita apre la prima pagina**, con `fine_della_partita`. «Vai al cruscotto ›» diventa «Vai all'eredità ›», cioè la casa dello stato (`casa`, `ossatura.ts:25`). In trattativa è «Vai alla trattativa ›».
3. **Dopo «Torna al turno N» dal Registro si apre la prima pagina del turno N, ferma**, come con «Riprendi». Al turno zero si apre la casa, perché la prima pagina è barrata.
4. **Riprendi e ritorno alla Prima pagina**: la pagina è ferma e composta. La rivelazione parte solo alla prima apertura dopo una chiusura. Uscire dalla prima pagina a metà rivelazione la chiude: i tracciati vanno al valore nuovo.
5. **Nomi dei soggetti nel Registro.** Il partito si legge da `Vista.partiti`, che tiene anche i partiti sciolti (`sciolto`). Il dicastero ha il nome della Vista, come `governo.ts:45`. La regione viene da `REGIONI` (`app/src/lib/mappa/regioni.ts:7`), la disposizione da `Vista.disposizioni` (46 voci, tutte). La Vista non porta il nome di una misura passata. Le leggi allora si filtrano per le disposizioni che toccano. Il bilancio (una `misura` senza `disposizioni`) si chiama «Legge di bilancio · <anno>», con l'anno del trimestre della voce.
6. **Quale tracciato muove ciascun tipo** (spec:407). La tabella `TRACCIATO_DI: Partial<Record<TipoVoce, "consenso" | "margine" | "spread">>` dice:
   - `sondaggio` e `risultato_del_voto` muovono il consenso;
   - `risultato_del_voto`, `governo_formato`, `alleato_esce`, `frangia_esce`, `coalizione_allargata`, `scissione`, `partito_sciolto`, `governo_caduto` e `crisi_aperta` muovono la maggioranza (o «per farlo cadere», che sta allo stesso posto);
   - `spread` muove lo spread.

   Vale la prima voce di quel tipo nell'ordine della chiusura, in evidenza o no. Un tracciato che nessuna voce muove scatta alla fine.
7. **La sezione di ogni tipo** (spec:407) sta in `SEZIONE_DI: Record<TipoVoce, Sezione>`, costruita dalla tabella `RIGA` del prototipo 25. Un tipo che `RIGA` non ha va dove si decide o si legge il suo soggetto, e la PR elenca le scelte. Il rimando in pagina è «<Sezione> ›». La voce dell'evento aperto dice «Si decide sul cruscotto ›» al governo e in trattativa, «Lo decide il governo» all'opposizione (storia 150). È aperto un evento con la stessa famiglia e la stessa regione in `Vista.eventi`.
8. **Tondi dello schema**: il numero è la posizione della voce fra quelle in evidenza, nell'ordine della pagina, con la voce di testa al numero 1. Ogni voce cerca un posto, in quest'ordine:
   - il filo `spread` per `spread`;
   - `sfiducia` per `sfiducia_presentata`;
   - `cal:<misura>` o `es:<misura>` se la voce ha `misura`;
   - `min:`, `prop:` o `dim:<dicastero>` secondo il tipo, se ha `dicastero`;
   - `al:<partito>` se ha `partito` ed esiste il filo;
   - `ev:<nome>` dell'evento della Vista con la stessa famiglia e la stessa regione, per gli eventi;
   - altrimenti il nodo del soggetto (`d:<dicastero>`, il partito).

   Senza posto la voce sta solo nell'elenco «Le notizie del trimestre», senza tondo.
9. **«Avanti fino a novità» senza tetto di chiusure.** Il core garantisce una fermata almeno al turno della campagna (`crates/sim-core/src/partita/esito.rs:1926`). Un guasto a metà corsa ferma la corsa. La banda allora dice le chiusure fatte, e la cornice il guasto. La corsa non chiede la serie a ogni giro: la chiede una volta, alla fine.
10. **Cache del registro**: alla prima apertura del Registro, o di Partiti con un partito nato in partita, la UI chiede `registro(t)` per ogni turno chiuso, uno alla volta. Le risposte restano in memoria fino a `riavvolgi`, `riprendi`, `fonda` o `esci`, che svuotano la cache. Un turno il cui registro fallisce mostra il guasto nella sua riga, e gli altri restano leggibili.
11. **Testo della conferma del riavvolgimento** dal prototipo 25 (`testoConferma`). Dice le chiusure che si cancellano, i voti e i governi nati nel mezzo (dal registro) e che sarà il riavvolgimento numero `IndicePartita.riavvolgimenti + 1`. Se l'indice non si legge, la frase sul numero cade e il resto resta.
12. **Partiti, l'origine.** La scheda di un partito nato in partita dice «Nato nel turno N», col `fatto` della voce `scissione` o `partito_di_protesta` che lo nomina. Se nel registro c'è un `cambio_di_leader` sul partito, aggiunge «leader nuovo dal turno N». Cerca per `partito`, anche nei `dettagli`.
13. **«Un altro turno, nel Registro ›»** nella bozza dell'eredità è un link a `/partita/registro`, sotto «Torna al turno N · prima del voto ›» (spec:401).
14. **Le vignette sono la PR 4, l'ultima**, e hanno una fermata per Mario. `graphic-designer` disegna tre varianti per scena e si ferma, Mario sceglie, poi rifinisce. Fino alla PR 4 ogni testa va a tutta larghezza (spec:409), ed è la stessa regola di un tipo senza scena. Le PR 1-3 non aspettano i disegni.
15. **Minori della 11e che la fetta tocca**, dentro la PR della loro schermata (memoria `minori-rinviati-11e`). Nella PR 2: `riavvolgi` con la finestra di doppio clic durante `await indice()` e il guasto originale perso se anche `riprendi` fallisce (`partita.svelte.ts:95-117`), che il Registro userà. Nella PR 3 i minori dello schema:
    - `fraTrimestri` che ricalcola `quando`/`votaOra` di `aula.ts`;
    - i test mancanti su dimissioni, posto libero o occupato, decaduta e nome del Presidente al turno zero;
    - `aggiungi` con 8 parametri;
    - la legenda di `DettaglioSchema` con gli spessori sbagliati;
    - la sbarra a 0,16 nel fuoco (il 37 dice 0,4);
    - `aria-pressed` al posto di `aria-expanded`/`aria-controls`;
    - `aria-label` che sostituisce il testo visibile (WCAG 2.5.3);
    - `uscenti` duplicato in `tavola.ts`.
16. **L'esito della sfiducia** (debito della #79) si legge in prima pagina, perché `sfiducia_approvata` e `sfiducia_respinta` sono voci dell'esito. Il Task 3 lo prova sulla chiusura dello stato `sfiducia` del 28.

## Punti da sorvegliare

I casi che la spec implica e che più facilmente rompono il gioco. Ognuno ha il suo test nel task indicato.

1. **Corsa che si guasta a metà.** La seconda chiusura di «avanti» dà un guasto. La corsa si ferma, la Vista resta quella della prima chiusura, la banda dice «1 chiusura» e la cornice mostra il guasto. Niente giro infinito, niente pagina vuota. Task 7.
2. **Navigare via a metà rivelazione.** Si apre il Cruscotto alla terza voce. I tracciati della cornice vanno al valore nuovo, e tornando alla Prima pagina la pagina è composta e ferma. Task 3.
3. **Risposte del registro dopo un riavvolgimento.** `registro(t)` è in volo quando si riavvolge: la risposta vecchia non entra nella cache nuova. Task 5.
4. **Pagina senza voci in evidenza.** Un esito con zero voci `in_evidenza` dà la testata, una riga «Nessuna notizia in evidenza» e il link al registro, non una pagina rotta. Task 2.
5. **Tipo del core senza testo.** Un tipo con l'elenco dei titoli vuoto, o tagliato da Mario, mostra l'occhiello come titolo. Un tipo nuovo che il file non ha fa fallire `npm run check`. Task 2.

## Struttura dei file

```
.scratch/v1-piano-11-ui/viste/genera/src/main.rs   + stato «ricondotto» (bilancio_ricondotto), se si trova giocando
.scratch/v1-piano-11-ui/viste/                     Viste rigenerate su main (16, 22 o 21 col ricondotto, README)
app/src/lib/testi/pungolo.json                     + titoli, didascalie, battute (dal file dei testi)
app/src/lib/testi/pungolo.ts                       + titoloDi, didascaliaDi, battutaDi
app/src/lib/esito/                                 esito.ts (testa, colonne, conti, grave, sezioni, tracciati per tipo),
                                                   rivelazione.svelte.ts, PrimaPagina.svelte, Testa.svelte, Voce.svelte,
                                                   Risultato.svelte, Banda.svelte, Vignetta.svelte (PR 4), scene/ (PR 4)
app/src/lib/registro/                              registro.ts (turni, filtri, collegate, nomi), cache.svelte.ts,
                                                   Registro.svelte, Turni.svelte, DettaglioVoce.svelte, Riavvolgi.svelte
app/src/lib/tracciati.ts                           + era (valore di prima) e scatto
app/src/lib/componenti/Tracciato.svelte            + «era X», lampeggio
app/src/lib/cornice/Cornice.svelte                 tracciati dalla rivelazione; «Avanti fino a novità ››» all'opposizione
app/src/lib/partita.svelte.ts                      chiudiTurno → prima pagina; avanti(); riavvolgi senza doppio clic
app/src/lib/guscio/finto.ts                        + registro(t), catena delle chiusure dal 25, corse dal 16
app/src/lib/ossatura.ts                            fettaDi: null per prima pagina e registro
app/src/routes/+page.svelte                        «Riprendi» apre la prima pagina ferma
app/src/routes/partita/[sezione]/+page.svelte      monta PrimaPagina e Registro
app/src/lib/schema/                                tondi, elenco, filtro, battuta nel dettaglio; minori della 11e
app/src/lib/partiti/SchedaPartito.svelte           origine e leader nuovo dal registro
app/src/lib/eredita/Bozza.svelte                   «Un altro turno, nel Registro ›»
docs/superpowers/verifiche/11f/                    screenshot della fetta
```

I test Vitest stanno accanto al modulo che provano. Un file `.svelte` che supera le 250 righe si divide per riquadro.

## PR e ordine

- **PR 1, Prima pagina** (`feat/11f-prima-pagina`): Task 1-4.
- **PR 2, Registro e «avanti»** (`feat/11f-registro`): Task 5-8.
- **PR 3, Schema** (`feat/11f-schema`): Task 9-10.
- **PR 4, Vignette** (`feat/11f-vignette`): Task 11-13, con la fermata per Mario dopo l'11.

Le PR sono impilate: la 2 parte dalla 1, la 3 dalla 2, la 4 dalla 3 (memoria `pr-impilate-con-squash`). Main è protetto, e il controllo `test` deve essere verde prima di ogni squash. Dentro una PR i task vanno in sequenza. Il Task 11, che produce solo disegni in `.scratch/`, può partire appena il piano è fuso, così la fermata per Mario cade mentre le PR 1-3 avanzano.

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo. `rust-reviewer` entra nella PR 1 solo se il Task 1 tocca il generatore. `craft-reviewer` riceve gli screenshot del task di verifica della PR e li confronta col prototipo e coi token.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta.

- **Dove ricerca e piano dicono cose diverse, vale il piano**: i prototipi e le decisioni del 17/11 sono venuti dopo le ricerche.
- **In un parallelo ogni subagente mette nel commit solo i file del proprio task**, nominandoli uno per uno in `git add`.

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `rust-engineer` | primo della PR 1 | `rust-skills` |
| 2 | `frontend-engineer` | dopo l'1 | salto dichiarato: moduli puri e testi copiati |
| 3 | `frontend-engineer` | dopo il 2 | `impeccable`, `apple-design` |
| 4 | sessione principale | dopo il 3 | `scrittura-italiana` |
| 5 | `frontend-engineer` | primo della PR 2 | salto dichiarato: moduli puri |
| 6 | `frontend-engineer` | dopo il 5 | `impeccable`, `apple-design` |
| 7 | `frontend-engineer` | dopo il 6 | `impeccable`, `apple-design` |
| 8 | sessione principale | dopo il 7 | `scrittura-italiana` |
| 9 | `frontend-engineer` | primo della PR 3 | `impeccable`, `dataviz` |
| 10 | sessione principale | dopo il 9 | `scrittura-italiana` |
| 11 | `graphic-designer` | appena il piano è fuso; si ferma per Mario | `canvas-design`, `algorithmic-art` |
| 12 | `graphic-designer`, poi `frontend-engineer` | dopo la scelta di Mario e la PR 3 | `canvas-design`; `impeccable` |
| 13 | sessione principale | dopo il 12 | `scrittura-italiana` |

«Almeno una fra» le skill elencate. Ogni brief nomina anche `skill-gate` e `caveman`, e quelli dei subagenti che scrivono codice anche `ponytail`.

## Dove va ogni storia

| Storia | Task |
|---|---|
| 144, 145, 149, 150 | 2, 3 |
| 146, 147, 151 | 2, 3 |
| 148 | 2 |
| 152 | 7 |
| 153, 154, 155 | 5, 6 |
| 156 | 6 |
| 157 | 9 |
| vignette (spec:409) | 11, 12 |
| testi (spec:416) | 2 |

---

### Task 1: Viste campione su main, col bilancio ricondotto

**Subagente:** `rust-engineer`: il generatore è un package Cargo.

**Sequenza:** primo della PR 1.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `.scratch/v1-piano-11-ui/viste/genera/src/main.rs` (`conti`, riga 1058, o `bilancio`, riga 1153: dove sta già `cerca_in_disavanzo`, riga 530); `.scratch/v1-piano-11-ui/viste/README.md`
- Output: le Viste rigenerate

**Interfaces:**
- Consumes: `cerca_in_disavanzo` (`genera/src/main.rs:530`), che raggiunge già il MES (`21-conti.json`, stato `mes`, seme 4 turno 15).
- Produces: nel file che ospita lo stato, la chiave `ricondotto` con `{ come, prima, vista }`. `vista.esito` contiene una voce `bilancio_ricondotto`. Se lo stato non si raggiunge, il generatore stampa `NON RAGGIUNTO: ricondotto` e la chiave manca.

- [ ] **Step 1: rigenera le Viste su main così come sono.** Usa il comando del README (`viste/README.md:13-15`) col percorso del worktree. Expected: stessi `shasum` dei file su main, salvo i campi che la 11e non toccava. Uno scarto non spiegato ferma il task e va scritto nel README.
- [ ] **Step 2: aggiungi la ricerca dello stato `ricondotto`.** Parti dalla partita in MES di `cerca_in_disavanzo` e, a ogni turno del bilancio, presenta la bozza con la spesa alzata oltre il tetto del MES, a passi (+1%, +3%, +5% della spesa di ogni dicastero). Fermati alla prima chiusura il cui esito ha `bilancio_ricondotto`. Semi 1-60, come gli altri stati.
- [ ] **Step 3: lancia il generatore due volte.** Expected: deterministico, stessi `shasum` ai due giri. Lo stato c'è, oppure `NON RAGGIUNTO: ricondotto` (decisione 4: in questo caso il Task 2 scrive la voce a mano).
- [ ] **Step 4: README.** Aggiungi la riga dello stato, o la nota «non raggiunto» coi semi provati.
- [ ] **Step 5: commit.** `chore(viste): rigenerate su main, stato del bilancio ricondotto`

---

### Task 2: testi del Pungolo, modulo puro della prima pagina, rivelazione e guscio finto

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo l'1.

**Skill-gate:** salto dichiarato (moduli puri e testi copiati, nessuna interfaccia); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/index.md:196`: `docs/ricerca/estetica/tono-giocoso-su-temi-seri.md`, cioè dove lo scherzo si rompe. Serve solo a capire perché nel tono grave i testi del Pungolo tacciono.

**Files:**
- Create: `app/src/lib/esito/esito.ts`, `esito.test.ts`; `app/src/lib/esito/rivelazione.svelte.ts`, `rivelazione.test.ts`
- Modify: `app/src/lib/testi/pungolo.json`, `pungolo.ts`, `pungolo.test.ts`; `app/src/lib/tracciati.ts` (`Tracciato`, righe 21-31; `tracciatiDi`, riga 80) e il suo test; `app/src/lib/guscio/finto.ts` (`rispondi`, righe 254-265; `installaFinto`, riga 331) e il suo test

**Interfaces:**
- Consumes: `VoceEsito`, `TipoVoce`, `Famiglia`, `Vista`, `PuntoSerie`; `hash` (`t.ts:58`); `tracciatiDi` (`tracciati.ts:80`).
- Produces, in `pungolo.ts`:
  - `type ChiaveTitolo = Exclude<TipoVoce, "evento_nato" | "evento_risolto"> | \`${"evento_nato" | "evento_risolto"}|${Famiglia}\``;
  - `chiaveDi(v: VoceEsito): ChiaveTitolo`;
  - `titoloDi(id: string, v: VoceEsito, turno: number, posto: number): string | null`, che dà null se la chiave non ha titoli. La formula è quella dei vincoli globali.
  - `didascaliaDi(v: VoceEsito): string | null`;
  - `battutaDi(filo: TipoFilo): string | null`.

  In `pungolo.json` vanno `titoli: Record<ChiaveTitolo, string[]>`, con tutte e 92 le chiavi più le famiglie e i 21 tipi sempre ordinari a `[]`, poi `didascalie` e `battute`.
- Produces, in `esito.ts`:
  - `testa(es: VoceEsito[]): number | null`: l'indice della voce di testa fra quelle `in_evidenza` (criticità `critica` > `notevole` > `ordinaria`, poi `precedenza` più alta, a parità la prima).
  - `pagina(es: VoceEsito[]): { testa: number | null; colonne: number[]; fuori: number; critFuori: number }`: le colonne nell'ordine della chiusura, senza la testa.
  - `grave(v: Vista): boolean`: regola dei vincoli globali.
  - `SEZIONE_DI: Record<TipoVoce, Sezione>` (lettura 7); `TRACCIATO_DI` (lettura 6).
  - `ordineRivelazione(es: VoceEsito[]): number[]`: gli indici `in_evidenza` nell'ordine della chiusura.
  - `scattaDopo(es: VoceEsito[], tr: "consenso" | "margine" | "spread"): number | null`: l'indice della prima voce che muove il tracciato, null se nessuna.
- Produces, in `rivelazione.svelte.ts`, l'oggetto condiviso `rivelazione`:
  - `prima: Vista | null`, la Vista di prima;
  - `serie_prima: PuntoSerie[]`;
  - `passo: number`, quante voci in evidenza sono rivelate;
  - `ferma: boolean`;
  - `corsa: { chiusure: number; da: number; fermate: Fermata[] } | null`;
  - `inizia(prima: Vista, serie: PuntoSerie[])`;
  - `avanza()`;
  - `salta()`;
  - `chiudi()`, che porta tutto al valore nuovo e azzera `prima`.

  Senza `prima`, la pagina è ferma (Riprendi).
- Produces, in `tracciati.ts`: `Tracciato.era: string | null`, il testo del valore di prima, e `Tracciato.scatta: boolean`; `tracciatiRivelati(v: Vista, serie: PuntoSerie[], r: typeof rivelazione, sfiducia?: AnteprimaSfiducia | null): [Tracciato, Tracciato, Tracciato]`. Prende ciascun tracciato da `prima` finché la sua voce non è passata, poi da `v` con `era`.
- Produces, in `finto.ts`: `?vista=25-&chiusura=N` serve `dopo` della chiusura N come Vista. `chiudi_turno` risponde con `dopo` della chiusura N+1. `registro` con `{ turno }` risponde con `dopo.esito` della chiusura `turno` della catena. Con `?vista=16-&stato=fermata_19` la Vista è `vista` e `prima` è disponibile per la corsa.

- [ ] **Step 1: copia i testi.** Copia dal file dei testi, versione fusa, titoli, didascalie e battute che Mario ha tenuto o riscritto (colonna «Mario» vuota o col testo nuovo). Una riga con `x` non entra. Non scrivere testi nuovi: una chiave senza titoli resta `[]`.
- [ ] **Step 2: scrivi i test che falliscono.**
  - `pungolo.test.ts`:
    - ogni `TipoVoce` ha la sua chiave;
    - `titoloDi` è stabile per id, turno e posto, e cambia da un turno al successivo per le chiavi con più di una variante;
    - una chiave vuota dà null;
    - nessun titolo supera 55 caratteri o contiene cifre;
    - `didascaliaDi` dà null per un tipo fuori dai quattordici.
  - `esito.test.ts`, sulle voci di `25-registro.json` (chiusure 3 e 8) e di `16-prima-pagina.json` (`voto_9`):
    - `testa` sceglie la critica con la precedenza più alta;
    - `pagina` conta `fuori` e `critFuori` come le voci non in evidenza;
    - con zero voci in evidenza `testa` è null e `colonne` vuoto;
    - `grave` è vero sulla Vista 08 e falso sul 03;
    - `SEZIONE_DI` copre ogni tipo;
    - `scattaDopo(..., "spread")` punta alla voce `spread`.

    Se il Task 1 ha dato `NON RAGGIUNTO: ricondotto`, un test costruisce una voce `bilancio_ricondotto` a mano (dichiarata nel test) e verifica che entri in pagina col suo titolo.
  - `rivelazione.test.ts`:
    - `inizia` poi `avanza` porta `passo` da 0 a 1;
    - `salta` porta `passo` alla fine e `ferma` a vero;
    - `chiudi` azzera `prima`.
  - `tracciati.test.ts`, sulla chiusura 3 del 25:
    - a `passo` 0 lo spread è quello di `prima`;
    - passata la voce `spread`, è quello di `dopo` con `era` uguale al testo di prima;
    - senza `prima`, `tracciatiRivelati` è uguale a `tracciatiDi`.
  - `finto.test.ts`:
    - `registro` sulla catena del 25 dà l'esito della chiusura;
    - `chiudi_turno` dà la chiusura successiva.
- [ ] **Step 3: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- pungolo esito rivelazione tracciati finto`. Expected: FAIL sui moduli e sulle chiavi che mancano.
- [ ] **Step 4: implementa** moduli, testi e finto.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): testi del Pungolo, modulo della prima pagina e rivelazione`

---

### Task 3: Prima pagina, rivelazione nella cornice, chiusura e Riprendi

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 2.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `.scratch/v1-piano-11-ui/issues/16-prima-pagina.md`: verdetto, fermate, grandi cali senza notizia.

**Files:**
- Create: `app/src/lib/esito/PrimaPagina.svelte`, `Testa.svelte`, `Voce.svelte`, `Risultato.svelte`
- Modify: `app/src/lib/componenti/Tracciato.svelte` («era X», lampeggio delle caselle che cambiano); `app/src/lib/cornice/Cornice.svelte` (riga 38: `tracciatiRivelati`); `app/src/lib/partita.svelte.ts` (righe 76-80); `app/src/routes/+page.svelte` (riga 96); `app/src/lib/ossatura.ts` (righe 62-64); `app/src/routes/partita/[sezione]/+page.svelte` (righe 46-73); `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 2 (`pagina`, `testa`, `grave`, `SEZIONE_DI`, `titoloDi`, `rivelazione`, `tracciatiRivelati`); `partita.indice()` per l'id.
- Produces: `PrimaPagina.svelte` con props `{ vista: Vista }`. La cornice usa `tracciatiRivelati`. `partita.chiudiTurno()` salva `rivelazione.inizia(vista di prima, serie di prima)` e apre `/partita/prima-pagina`. `riprendi` dall'edicola apre `/partita/prima-pagina` senza `inizia`, cioè ferma, e al turno zero apre la casa.

- [ ] **Step 1: componi la pagina sul prototipo 16, forma A.**
  - La testata dice «Esito del trimestre», il trimestre chiuso, quante notizie in pagina e quante nel registro.
  - La testa ha occhiello, titolo, causa, rimando e, per `risultato_del_voto`, il risultato a barre: affluenza, quote e seggi, coalizione col premio in nero, giocatore in rosso (spec:408). Fino alla PR 4 nessuna vignetta: la testa va a tutta larghezza.
  - Tre colonne, poi il quadratino rosso sulle critiche.
  - Il piede ha «Altre N notizie nel registro ›» (con «, K critiche»), «Salta la rivelazione ››» finché la rivelazione è in corso, e «Vai al cruscotto ›» (lettura 2).

  Nel tono grave il titolo spiritoso non c'è e l'occhiello sale a titolo.
- [ ] **Step 2: rivelazione.** Fai avanzare le voci al loro posto, una ogni 1,1 s × `k`; un clic ovunque sulla pagina salta tutto. I tracciati scattano al passaggio della loro voce, lampeggiano in giallo e tengono «era X». Uscendo dalla pagina chiama `rivelazione.chiudi()` (lettura 4). A `k = 0` la pagina è già composta.
- [ ] **Step 3: Chiusura e Riprendi.** «Chiudi il turno» apre la prima pagina. Riprendi la apre ferma. La voce dell'evento aperto dice «Si decide sul cruscotto ›» o «Lo decide il governo» (lettura 7). `fettaDi` dà null per prima pagina e registro (il registro resta segnaposto fino alla PR 2: `fettaDi` per `registro` torna null nel Task 6).
- [ ] **Step 4: prova nel browser** coi campioni:
  - `/partita/prima-pagina?vista=25-&chiusura=3`;
  - `?vista=16-&stato=voto_9` e `voto_opp`;
  - `?vista=08-` per il tono grave;
  - `?vista=28-&stato=sfiducia`, chiudendo il turno: la pagina dice l'esito della sfiducia (lettura 16).

  Prova anche il clic che salta, l'uscita a metà (punto 2 da sorvegliare) e il movimento spento.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): prima pagina con la rivelazione`

---

### Task 4: verifica della PR 1

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 3. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** (`npm --prefix <wt>/app run verifica:chrome` e `verifica:webkit`, `app/verifica/README.md`). Pagine: `/partita/prima-pagina` sugli stati del Task 3, Step 4, alle due finestre e alle due scale, con la rivelazione finita (`?tagli=1` dopo «Salta»). Expected: uscita 0, tagli 0.
- [ ] **Step 2: giro nell'app vera col telecomando** (memoria `verifica-ui-nell-app-vera`): dall'edicola, Riprendi, poi «Chiudi il turno» e la rivelazione. Prova anche il movimento spento.
- [ ] **Step 3: test e tipi.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`. Expected: verdi, pulito.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11f/`: la prima pagina ordinaria, del voto e grave, alle due finestre, e una a metà rivelazione con un tracciato che scatta.
- [ ] **Step 5: corpo della PR**: cosa cambia, decisioni prese in esecuzione, «Per Mario», testi neutri nuovi, verifiche.

**Fine della PR 1.**

---

### Task 5: modulo puro del registro e cache

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 2.

**Skill-gate:** salto dichiarato (moduli puri); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Create: `app/src/lib/registro/registro.ts`, `registro.test.ts`; `app/src/lib/registro/cache.svelte.ts`, `cache.test.ts`
- Modify: `app/src/lib/partita.svelte.ts` (`riavvolgi`, righe 95-117; `riprendi`, `fonda`, `esci` svuotano la cache)

**Interfaces:**
- Consumes: `guscio.registro` (`chiamate.ts:61`); `SEZIONE_DI` (Task 2); `REGIONI`; `Vista.partiti`, `Vista.disposizioni`.
- Produces, in `registro.ts`:
  - `type Turno = { turno: number; voci: VoceEsito[]; critiche: number; grave: boolean; voto: boolean }`;
  - `type Filtro = { criticita: "tutte" | "notevoli" | "critiche"; sezione: Sezione | null; soggetto: Soggetto | null }`, con `Soggetto = { tipo: "partito" | "dicastero" | "regione" | "disposizione" | "bilancio"; id: string }`;
  - `passa(v: VoceEsito, f: Filtro): boolean`, che guarda anche i `dettagli`;
  - `soggetti(turni: Turno[], vista: Vista): { soggetto: Soggetto; nome: string }[]`, coi nomi della lettura 5;
  - `collegate(v: VoceEsito, turno: number, turni: Turno[]): { stesse: [number, VoceEsito][]; nelleRighe: [number, VoceEsito][] }`;
  - `corregge(v: VoceEsito, turno: number, turni: Turno[]): VoceEsito | null`, la `prima_stima` del turno prima per una `revisione`;
  - `cancella(n: number, ultimo: number, turni: Turno[]): { chiusure: number; voti: number[]; governi: number[] }`, per il testo della conferma.
- Produces, in `cache.svelte.ts`: `registroCache` con `turni: (Turno | { turno: number; guasto: string })[]`, `carica(vista: Vista): Promise<void>` (uno alla volta; un giro nuovo scarta le risposte vecchie) e `svuota()`.

- [ ] **Step 1: scrivi i test che falliscono**, sulla catena del 25 (`dopo.esito` delle chiusure 0-8):
  - `passa` con `criticita: "critiche"` tiene solo le critiche;
  - un soggetto `partito: "centrodestra"` passa anche da una voce fusa che lo ha solo in un `dettaglio`;
  - `soggetti` dà il nome dei partiti, delle regioni, delle disposizioni e «Legge di bilancio · 2027» per la misura 3 (chiusura 4);
  - `corregge` sulla `revisione` del turno 4 dà la `prima_stima` del turno 3;
  - `collegate` su `legge_respinta` di `autonomia_differenziata` (chiusura 5) trova le voci della stessa disposizione negli altri turni;
  - `cancella(5, 8, …)` dà 4 chiusure;
  - la cache: `carica` poi `svuota` durante il volo, e la risposta in ritardo non entra (punto 3 da sorvegliare);
  - un turno che fallisce entra come `{ turno, guasto }`;
  - `partita.riavvolgi` chiamato due volte di fila parte una volta sola (minore della 11e);
  - se `riavvolgi` e `riprendi` falliscono tutti e due, il guasto mostrato è quello di `riavvolgi`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- registro cache partita`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): modulo del registro e cache dei turni`

---

### Task 6: Registro, D · Indice col filo, con «Torna al turno N»

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 5.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `.scratch/v1-piano-11-ui/issues/25-registro.md`: verdetto D, cosa legge, revisione e prima stima.

**Files:**
- Create: `app/src/lib/registro/Registro.svelte`, `Turni.svelte`, `DettaglioVoce.svelte`, `Riavvolgi.svelte`
- Modify: `app/src/lib/ossatura.ts` (`fettaDi` per `registro`); `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/eredita/Bozza.svelte` (lettura 13); `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 5; `Conferma.svelte` (`app/src/lib/componenti/`); `partita.riavvolgi`, `partita.indice()`.
- Produces: `Registro.svelte` con props `{ vista: Vista }`.

- [ ] **Step 1: componi sul prototipo 25, forma D.**
  - La colonna dei turni ha il più recente in alto, i conti delle critiche, il segno del tono grave e del voto, e il turno zero in fondo.
  - In pagina vanno le voci del turno scelto, con fatto e causa, senza titoli.
  - Sopra stanno i filtri (criticità, sezione, soggetto), che valgono su tutti i turni. Filtrando, la colonna tiene solo i turni che combaciano.
  - Il dettaglio a lato ha la voce, «Corregge» o «Riveduta da», e le collegate.

  Il Registro resta acceso a partita finita finché l'eredità è in bozza, e barrato al turno zero (`ossatura.ts:44`).
- [ ] **Step 2: «Torna al turno N ›»** in testa al turno N. La conferma sul posto ha il testo della lettura 11 e il bottone «Conferma: cancella K chiusure». Dopo il riavvolgimento si apre la prima pagina ferma (lettura 3). Durante il riavvolgimento il bottone è spento, perché c'è una chiamata in volo.
- [ ] **Step 3: «Un altro turno, nel Registro ›»** nella bozza dell'eredità (lettura 13).
- [ ] **Step 4: prova nel browser.**
  - `/partita/registro?vista=25-&chiusura=8`: i filtri e le collegate, la conferma, il riavvolgimento sul finto.
  - `?vista=30-`: la partita finita in bozza.
  - `?vista=08-`: il tono grave.
  - Il turno zero, barrato.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): registro con l'indice dei turni e il riavvolgimento`

---

### Task 7: «Avanti fino a novità», banda delle fermate, origine dei partiti

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 6.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Create: `app/src/lib/esito/Banda.svelte`
- Modify: `app/src/lib/partita.svelte.ts` (+ `avanti`); `app/src/lib/cornice/Cornice.svelte` (righe 75-90); `app/src/lib/esito/PrimaPagina.svelte`; `app/src/lib/partiti/SchedaPartito.svelte` (riga 43); `app/src/lib/guscio/finto.ts` (corse del 16); `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `Vista.fermate`, `Fermata`; `rivelazione.corsa` (Task 2); `registroCache` (Task 5).
- Produces: `partita.avanti(): Promise<void>`. Chiude il turno, e finché la Vista nuova ha `fermate` vuote chiude di nuovo. Alla fine scrive `rivelazione.corsa = { chiusure, da, fermate }`, fa `inizia` con la Vista prima dell'ultima chiusura (lettura 1), chiede la serie una volta e apre la prima pagina. Un guasto ferma la corsa (lettura 9).

- [ ] **Step 1: scrivi i test che falliscono** in `partita.test.ts`, col guscio finto:
  - la corsa dal 06 si ferma alla prima Vista con `fermate` non vuote;
  - `corsa.chiusure` conta le chiusure;
  - un guasto alla seconda chiusura lascia la Vista della prima, `corsa.chiusure = 1` e il guasto in `partita.guasto` (punto 1 da sorvegliare).

  In `esito.test.ts`, `fraseFermata(f: Fermata, v: Vista): string`:
  - una frase per tipo, come le `FERMA` del prototipo 16, con «governo» e «fine» comprese;
  - per `voto` il nome della misura dal calendario;
  - per `ultimatum` il nome del partito.

  La frase sul referendum non promette una finestra che il core chiude: dice «è l'ultimo trimestre per chiedere un referendum quest'anno» solo quando la fermata c'è.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- partita esito`. Expected: FAIL.
- [ ] **Step 3: implementa.**
  - Nella cornice, all'opposizione, «Avanti fino a novità ››» va sopra «Chiudi il turno» (prototipo 16, `cornice()`).
  - La banda in testa alla prima pagina dice le chiusure di fila, perché si è fermata (una frase per fermata) e «Le pagine saltate sono nel registro: turno N ›» sul più vecchio dei saltati. Il link apre il Registro su quel turno.
  - Nella scheda dei Partiti c'è l'origine e il leader nuovo (lettura 12). Il segnaposto della riga 43 sparisce.
- [ ] **Step 4: prova nel browser.** `/partita/cruscotto?vista=06-` con «avanti» sul finto: catena del 16, corse `fermata_19` e `fermata_20`. Prova la banda e il link, poi Partiti su `?vista=30-`, che ha un `partito_sciolto`, e sulla voce nata in partita se il campione ne ha una.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): avanti fino a novità con la banda delle fermate`

---

### Task 8: verifica della PR 2

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 7. Chiude la PR 2.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori.** Pagine: il Registro sugli stati del Task 6, Step 4, e la prima pagina con la banda (`?vista=16-&stato=fermata_19`), alle due finestre e alle due scale. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: colonna dei turni, filtri, voce e collegate, conferma del riavvolgimento, «avanti» e il link della banda.
- [ ] **Step 3: giro nell'app vera col telecomando**: una partita all'opposizione, «avanti» fino a una fermata, il Registro, «Torna al turno N». Controlla che l'albo conti il riavvolgimento.
- [ ] **Step 4: test, tipi, build e screenshot** in `docs/superpowers/verifiche/11f/`.
- [ ] **Step 5: corpo della PR.**

**Fine della PR 2.**

---

### Task 9: schema dello Stato coi tondi, l'elenco, il filtro e la battuta; minori della 11e

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 3.

**Skill-gate:** `impeccable` o `dataviz`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `.scratch/v1-piano-11-ui/issues/29-schema-dello-stato.md`: tondi e filtro.

**Files:**
- Modify: `app/src/lib/schema/schema.ts` (+ `tondiDi`), `schema.test.ts`, `tavola.ts`, `tavola.test.ts`, `Tavola.svelte`, `DettaglioSchema.svelte`, `Schema.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `pagina`, `grave` (Task 2); `battutaDi` (Task 2).
- Produces: `tondiDi(v: Vista, S: { nodi: Nodo[]; fili: Filo[] }): { fili: Map<string, number[]>; nodi: Map<string, number[]>; elenco: { n: number; voce: VoceEsito; posto: string | null }[] }`, con la regola della lettura 8.

- [ ] **Step 1: scrivi i test che falliscono** in `schema.test.ts`, sulla Vista 04 (`25-registro`, chiusura 3, `dopo`):
  - la voce `spread` ha il tondo sul filo `spread`;
  - una legge respinta ha il tondo su `es:<misura>`;
  - una voce senza posto sta nell'elenco con `posto: null`;
  - i numeri seguono l'ordine della pagina, testa al numero 1;
  - al turno zero nessun tondo.

  Aggiungi i test che mancavano: dimissioni, posto libero e occupato, decaduta, nome del Presidente al turno zero.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- schema tavola`. Expected: FAIL.
- [ ] **Step 3: implementa.**
  - I tondi vanno sui fili e sulle righe, come nel 37.
  - L'elenco «Le notizie del trimestre» sta nel dettaglio a lato senza scelta.
  - Il filtro «Solo quelli in prima pagina» mostra i fili e le righe che hanno un tondo.
  - La battuta del tipo di filo va solo nel dettaglio a lato del filo che ha un tondo, a biro (decisione 2), e tace nel tono grave.
  - Chiudi i minori della 11e della lettura 15.
- [ ] **Step 4: prova nel browser.** `/partita/schema` su `?vista=25-&chiusura=3`, `?vista=28-&stato=fine`, `?vista=08-` e `?vista=01-`.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): tondi, filtro e battute nello schema dello Stato`

---

### Task 10: verifica della PR 3

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 9. Chiude la PR 3.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** sugli stati del Task 9, Step 4, col filtro acceso e spento. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera**: il filtro, i tondi, l'elenco. Con un lettore di schermo, il testo visibile delle righe è letto (WCAG 2.5.3).
- [ ] **Step 3: test, tipi, build e screenshot.**
- [ ] **Step 4: corpo della PR.**

**Fine della PR 3.**

---

### Task 11: varianti delle cinque scene (fermata per Mario)

**Subagente:** `graphic-designer`, primo dei suoi due dispatch.

**Sequenza:** appena questo piano è fuso, in parallelo alle PR 1-3: scrive solo in `.scratch/`.

**Skill-gate:** `canvas-design` o `algorithmic-art`; `skill-gate`; `caveman`.

**Ricerca:**
- `docs/ricerca/index.md:197`: `docs/ricerca/estetica/grafica-italiana-giocosa.md`, la vignetta satirica e i tratti formali;
- `docs/ricerca/index.md:198`: `docs/ricerca/estetica/caricature-di-personaggi-inventati.md`, il metodo del kit (pennino piatto, figura campita di carta).

**Files:**
- Create: `.scratch/v1-piano-11-ui/prototipi/39-vignette-11f.prototype.html`, con tre varianti per ciascuna delle cinque scene, ciascuna con la didascalia di un suo tipo dal file dei testi, nel riquadro della testa del prototipo 16.

- [ ] **Step 1: disegna** le cinque scene (Aula, Giuramento, Emergenza, Lettera, Cassa) col metodo del kit delle caricature: profilo al tratto, pennino piatto, figura campita di carta, inchiostri `--ink` e `--carta`, retino solo come nel 16. Valgono la lista nera (spec:343) e il bersaglio del palazzo: nessuna vittima disegnata. Nessuna somiglianza con persone o luoghi reali.
- [ ] **Step 2: prove di resa** sul render: a 300×150 e a metà, chiaro, nessuna sovrapposizione col bordo della figura.
- [ ] **Step 3: fermati.** Riporta il prototipo a Mario, che sceglie una variante per scena. Il Task 12 non parte senza la sua scelta.

---

### Task 12: vignette nella prima pagina

**Subagente:** `graphic-designer` (rifinitura ed export, secondo dispatch), poi `frontend-engineer`.

**Sequenza:** dopo la scelta di Mario e la PR 3.

**Skill-gate:** `canvas-design` per il primo; `impeccable` per il secondo; `skill-gate`; `caveman`; `ponytail` per il secondo.

**Ricerca:**
- `docs/ricerca/index.md:198`: il peso misurato e il ripiego nel tono grave.

**Files:**
- Create: `app/src/lib/esito/scene/` (un SVG pulito per scena, consegnato dal `graphic-designer`); `app/src/lib/esito/Vignetta.svelte`
- Modify: `app/src/lib/esito/esito.ts` (+ `SCENA_DI`), `esito.test.ts`; `app/src/lib/esito/Testa.svelte`

**Interfaces:**
- Produces: `SCENA_DI: Partial<Record<ChiaveTitolo, "aula" | "giuramento" | "emergenza" | "lettera" | "cassa">>`, coi quattordici tipi della decisione 1; `Vignetta.svelte` con props `{ voce: VoceEsito }`, che non disegna niente senza scena, nel tono grave o per il voto.

- [ ] **Step 1: rifinisci ed esporta** le cinque varianti scelte in SVG puliti, ciascuno sotto i 15 KB.
- [ ] **Step 2: scrivi i test che falliscono** in `esito.test.ts`:
  - `SCENA_DI` ha esattamente i quattordici tipi;
  - `evento_risolto|emergenza` ed `evento_nato|emergenza` danno `emergenza`;
  - `evento_risolto|scandalo` non ha scena.
- [ ] **Step 3: implementa.** La vignetta va nella testa, al 30% della larghezza come nel 16, con la didascalia (`didascaliaDi`). Senza scena la testa resta a tutta larghezza.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): vignette della prima pagina`

---

### Task 13: verifica della PR 4

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 12. Chiude la PR 4 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 4).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** sulla prima pagina con una testa per scena: quelle che i campioni portano (25, 16, 28), più una per scena mancante costruita dal finto con `?testa=<tipo>`, dichiarata. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tono grave e movimento spento**: niente vignetta nel grave, e niente animazione della figura.
- [ ] **Step 3: test, tipi, build e screenshot**, con le cinque scene alle due finestre.
- [ ] **Step 4: corpo della PR.** Nella sezione «Per Mario» va la prova di riconoscimento sulle scene: nessuna ricorda un fatto o un luogo reale.

**Fine della PR 4 e della fetta.**
