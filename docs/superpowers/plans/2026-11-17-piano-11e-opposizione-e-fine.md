# Piano 11e «Opposizione e fine»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** il giocatore all'opposizione ha un cruscotto suo, con le leve in testa; a partita finita legge l'ultima edizione e decide se riprovare; in ogni stato apre lo schema dello Stato e vede chi muove cosa. La fetta porta tre schermate (Cruscotto all'opposizione, Eredità, Schema dello Stato), il passacarte `anteprima_sfiducia`, e riempie subito, coi dati dello schema 19, i quattro posti dell'opposizione che la spec lasciava alla 11g.

**Architettura:**
- Ogni schermata è una cartella in `app/src/lib/<sezione>/`: un modulo `.ts` puro, che legge la Vista e decide cosa mostrare, e i componenti Svelte che lo disegnano. Il modulo puro è provato con Vitest sulle Viste campione.
- Il guscio guadagna un passacarte verso un metodo già pubblico del core, `anteprima_sfiducia`. `sim-core` non cambia.
- La Vista dello schema 19 porta già la sfiducia presentata (`Vista.sfiducia`), il posto riservato altrui (`posto_riservato_di`), la referendabilità (`VoceDisposizione.referendabile`) e il costo della linea (`VoceMisura.costo_linea`): la fetta li usa (decisione 3).
- L'Eredità è una composizione con due sorgenti: la Vista nella partita, la voce d'albo dall'edicola. Un adattatore della UI le porta alla stessa forma e non calcola niente (spec:401).
- Il guscio finto impara a rispondere dai campioni 28 e 30, così ogni schermata si prova nel browser.

**Tecnologie:** quelle della 11d (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2, Rust). Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 131-143 (righe 180-195); i vincoli alle righe 248-371, la sezione della fetta alle righe 398-402, le conferme alla riga 453 e i test alle righe 455-476. La PR di questo piano corregge la spec dove le decisioni del 17/11 la cambiano (righe 355, 400, 401, 402, 420, 495). Chi esegue legge la spec e il ticket citato nel task.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `28-opposizione.prototype.html`: forma A · Ordine del giorno con le leve in testa, tracciati con la lettura 2; stati `ordinario`, `sfiducia`, `passa`, `referendum`, `posto`, `linea`, `intesa`, `grave`, `fine`. I segni `11g` a tratteggio ora vengono dalla Vista (decisione 3). Gli stati `firme` e `confermativo` e la banda della fermata sono DI PROVA: non vanno nella fetta;
- `30-eredita.prototype.html`: forma A · Ultima edizione col riavvolgimento di B; stati `bozza` e `albo`;
- `37-scelte-11e-fine.prototype.html`: i sedici titoli (decisione 2) e la colonna delle leggi con l'interruttore `leggi` su «schema 19»;
- `29-schema.prototype.html`, forma B, per tutto ciò che il 37 non tocca (colonne, ordine delle righe, tipi di filo, dettaglio a lato, stati `ordinario`, `opposizione`, `grave`, `zero`);
- `37-scelte-11e-schema.prototype.html`, aperto su riga · pettine · punta: le tre note di Mario (decisione 1). Il codice di riferimento è `svgB` nel file.

I ticket che ogni task cita stanno in `.scratch/v1-piano-11-ui/issues/` (28, 29, 30). I testi dell'interfaccia si copiano dai prototipi. Un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR. Un numero che un prototipo segna «11g» ora viene dalla Vista o da `anteprima_sfiducia`.

**Da dove si parte:** main a `e680875`, con la 11d fusa (#73, #74, #75). Oggi l'opposizione apre il cruscotto con i soli eventi e un segnaposto (`app/src/routes/partita/[sezione]/+page.svelte:64-70`); Eredità e Schema sono segnaposto della 11e (`app/src/lib/ossatura.ts:61-67`, `fettaDi`); «Leggi l'eredità ›» nell'albo apre un segnaposto (`app/src/lib/edicola/DettaglioAlbo.svelte:44-45`); il tracciato «Per farlo cadere» c'è già, vuoto, col rimando alla 11e (`app/src/lib/tracciati.ts:103`). Le Viste campione 28 e 30 sono allo schema 19 (rigenerate il 15/11/2026, `viste/README.md:5`): non servono campi nuovi né una rigenerazione.

## Vincoli globali

Valgono tutti i vincoli globali del piano 11d (`docs/superpowers/plans/2026-11-16-piano-11d-i-conti.md:28-40`), che richiamano quelli delle fette prima. Qui quelli che la fetta tocca di più, e quelli nuovi.

- **La UI legge, il core decide** (spec:250). Lecito: sommare i seggi alla Camera dei partiti che l'anteprima dà per il sì (voto a blocchi, come il prototipo 28 e il core a `partita.rs:4437`), contare le voci della Vista, scegliere un testo con l'hash. Ogni esito, firma, costo e ammissibilità viene dal core.
- **Il guscio aggiunge un passacarte** verso un metodo già pubblico della partita, senza logica né stato (spec:253). `sim-core` non cambia.
- **Conferme** (spec:453 e storia 133): sfiducia (al governo e a un ministro) e referendum abrogativo con la conferma sul posto; confermativo senza conferma, perché il comando si ridà fino alla chiusura; la linea Sì · No · come da posizione diretta. «Torna al turno N · prima del voto ›» e «Conferma l'eredità» confermati sul posto, con ciò che si cancella (storia 139).
- **Rifiuto del core accanto alla scelta**, col testo passato da `testoRifiuto`. Una regola che la Vista non dice (finestra del referendum, ultimo anno della legislatura) non si scrive nella UI: si mostra il rifiuto del core dopo il clic.
- **Agende avversarie**: all'opposizione le agende dei ministri non si vedono, e una riga lo dice (storia 134). La Vista non le porta: la UI non le cerca.
- **Tono grave**: le regole delle fondamenta; lo schema non anima i fili.
- **Cargo da un worktree**: `--manifest-path <wt>/Cargo.toml`, `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi` e `CARGO_TARGET_DIR=<wt>/target`, tutti assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md:105`).

## Decisioni di Mario del 17/11/2026

Prese sui prototipi `37-scelte-11e-schema` e `37-scelte-11e-fine`, in una sola domanda.

1. **Schema dello Stato, le tre note dentro B: riga · pettine · punta.**
   - *Fili muti*: l'etichetta corta del filo sta nella riga di chi lo muove, allineata a destra sulla seconda riga del riquadro; più d'un filo in uscita dà «+N». Sui riquadri scuri (Presidente, Camera, Senato, Mercati) l'etichetta è gialla.
   - *Groviglio sul Presidente*: due sbarre ai lati della colonna del governo. Ogni filo verso il Presidente, e ogni filo fra il Presidente e un suo ministro, arriva dritto sulla sbarra del suo lato, all'altezza della riga da cui parte; una punta sola per sbarra entra nel Presidente.
   - *Verso*: punta dove il filo arriva.
2. **Titoli della fine senza seggi: tutti e sedici** del prototipo 37-fine, nel file dei testi del Pungolo accanto ai motti.
3. **Schema 19, tutti i posti.** La 11e riempie anche i quattro posti dell'opposizione che la spec dava alla 11g: sfiducia presentata, posto riservato altrui, referendabilità, costo della linea, nella forma del 28. Nessun campo nuovo nelle Viste campione. Riletta dall'albo, la colonna delle leggi ha solo i nomi: `VoceAlbo.leggi_in_vigore` non porta l'etichetta. Rimando al core.
4. **Eredità di una partita abbandonata, riletta dall'albo: testa seria.** Occhiello «Partita lasciata · <trimestre della fine>», titolo serio col nome del partito, niente scherzo, bollo «Abbandono».

## Letture che la spec non fissa

Le ho decise scrivendo il piano. Mario le conferma col sì all'esecuzione, o le cambia prima.

1. **«Sposta la tua posizione di 4,5 punti» in Aula è un difetto, ed entra qui.** `CostoLinea.percepita` è la posizione percepita *dopo* il voto (`crates/sim-core/src/vista.rs:543-551`), non lo spostamento; `aula.costo_linea` la scrive come uno spostamento (`app/src/lib/aula/Misura.svelte:53`), quindi tutte e tre le linee dicono la posizione di oggi. Il cruscotto all'opposizione mostra lo stesso costo: la riga della linea diventa un componente condiviso e il testo diventa «la tua posizione percepita va da {da} a {a} · franchi attesi: {c} alla Camera, {s} al Senato», con due decimali (gli scarti sono di centesimi: sul «Cuneo fiscale» del 28 la linea sì porta da 1,94 a 2,01, il default a 2,06). Una linea che lascia la posizione dov'è dice «resta a {da}». `da` è `partiti[giocatore].posizione_percepita`.
2. **Firme della sfiducia a parole, senza numeri**: «le firme bastano» o «le firme non bastano», da `AnteprimaSfiducia.firme`. La soglia (un decimo della Camera) è una regola del core che la Vista non porta (`partita.rs:4422`), e le firme bastano sempre nelle partite giocate (spec:496).
3. **«Cosa costa se è respinta» senza cifra.** Né la Vista né l'anteprima portano il costo della sfiducia respinta. La riga dice la conseguenza che il ticket 28 ha misurato, a parole: «se è respinta, perdi credibilità e gli alleati del governo si stringono». Rimando al core per la cifra.
4. **«Per farlo cadere»**: i sì che mancano alla metà più uno della Camera, sommando i seggi alla Camera dei partiti con la linea sì in `anteprima_sfiducia(null)`; zero e pieno quando `passa`. Quindici seggi per casella (storia 136), mai rosso. La cornice chiede l'anteprima a ogni Vista nuova all'opposizione; finché non arriva, il tracciato resta com'è oggi, vuoto.
5. **Sfiducia a un ministro**: «Scegli il ministro ›» apre nel dettaglio a lato le dieci anteprime (una chiamata per dicastero con un ministro, in sequenza, alla prima apertura del turno); la riga dice su quanti passerebbe. Un dicastero in interim non si offre (`sfiducia_ammessa`, `partita.rs:4391-4416`).
6. **Referendum dal cruscotto**: «Scegli la disposizione ›» apre nel dettaglio a lato le disposizioni in vigore, quelle non referendabili spente col motivo «non referendabile» (`VoceDisposizione.referendabile`); la scelta si conferma sul posto e comanda `PromuoviReferendum`. Il bottone che l'Aula ha già (`aula/Disposizioni.svelte:37`) resta com'è.
7. **Turno da riavvolgere**: «Torna al turno N · prima del voto» con N = `vista.turno − 1`, il turno della chiusura che ha finito la partita (30: turno 21, si torna al 20). Dopo `riavvolgi` la UI rifà `riprendi` se il guscio dà un guasto: un `torna_a` fallito lascia la memoria allo stato del disco (debito della 11a).
8. **Dopo «Conferma l'eredità»** l'app apre l'edicola con la voce nuova dell'albo scelta, con lo stesso `voceNuova` dell'abbandono (`app/src/lib/edicola/edicola.ts:48`), passato con un parametro della rotta: `/?albo=nuova&nome=<nome>`.
9. **Eredità dall'albo nella stessa pagina dell'edicola**: «Leggi l'eredità ›» sostituisce il banco con l'ultima edizione, senza cornice, con «‹ Edicola» e nessuna azione. Nessuna rotta nuova.
10. **Titolo scelto con l'hash di nome, scenario e trimestre d'inizio**, che la bozza e l'albo hanno tutti e due: la stessa partita ha lo stesso titolo nella bozza e riletta dall'albo.
11. **Schema: i fili dello schema 19.** Alla tabella della spec (dodici tipi, spec:402) si aggiungono la sfiducia presentata (partito → Presidente, o → ministro) e il posto riservato altrui (partito → Camera, «posto riservato»). Il referendum in attesa resta fuori: non tocca nessun soggetto delle quattro colonne.
12. **Schema senza tondi, filtro e battute**: arrivano con la 11f (spec:415). Il dettaglio a lato, senza scelta, ha «Come si legge» e il numero dei fili.
13. **Barrata del Bilancio a partita finita**: «Partita finita» al posto di «Non governi» (oggi `ossatura.ts:54` vince perché a partita finita `governa` è falso). Il debito rinviato dalla #75 parlava di «fuori stagione»: il codice di main dà «Non governi»; il motivo nuovo copre tutti e due.

## Punti da sorvegliare

I casi che la spec implica e che più facilmente rompono il gioco. Ognuno ha il suo test nel task indicato.

1. **Doppia mossa.** Sfiducia e abrogativo sono una mossa per turno: dopo il comando la riga dice «presentata» dalla Vista (`Vista.sfiducia`, `referendum[]`), il bottone sparisce, e un secondo clic veloce non parte (una chiamata in volo). Il secondo comando è un rifiuto sul posto col testo del core (`rifiuto_seconda` e `rifiuto_secondo` del 28). Task 4.
2. **Sfiducia presentata da un altro partito.** `Vista.sfiducia.partito` può non essere il giocatore: la riga dice chi l'ha presentata e il bottone è spento con quel motivo. Task 2, sullo stato `fine` o costruendo la Vista dal 28.
3. **Anteprima che arriva tardi.** La Vista cambia (chiusura, comando) mentre `anteprima_sfiducia` è in volo: la risposta vecchia non scrive sulla Vista nuova (`ultimoVince`). Task 2 e Task 3.
4. **Riavvolgimento fallito.** `riavvolgi` dà un guasto: la pagina non resta sulla bozza vecchia, la UI rifà `riprendi` e mostra il guasto nella cornice. Task 7.
5. **Fili che puntano a un soggetto assente.** Un filo il cui capo non ha una riga (un partito sciolto, una regione senza nodo) non si disegna e non rompe la tavola; il conteggio dei fili conta solo quelli disegnati. Task 10.

## Struttura dei file

```
app/src-tauri/src/lib.rs                     + anteprima_sfiducia
app/src/lib/guscio/chiamate.ts               + anteprimaSfiducia
app/src/lib/guscio/finto.ts                  + risposte dai campioni 28 e 30; abbandona, conferma_eredita, riavvolgi
app/src/lib/opposizione/                     opposizione.ts, Opposizione.svelte, Leve.svelte, DettaglioSfiducia.svelte, DettaglioReferendum.svelte, Programma.svelte
app/src/lib/aula/Linea.svelte                la linea Sì · No · come da posizione col costo, da Misura.svelte
app/src/lib/aula/Misura.svelte               usa Linea.svelte
app/src/lib/tracciati.ts                     «Per farlo cadere» dall'anteprima
app/src/lib/eredita/                         eredita.ts (adattatore e titolo), Eredita.svelte, Testa.svelte, Colonne.svelte, Bozza.svelte
app/src/lib/edicola/DettaglioAlbo.svelte     «Leggi l'eredità ›» apre l'ultima edizione
app/src/routes/+page.svelte                  ultima edizione al posto del banco; ?albo=nuova
app/src/lib/schema/                          schema.ts (nodi e fili), Tavola.svelte, DettaglioSchema.svelte, Schema.svelte
app/src/lib/testi/pungolo.json               + titoli_fine
app/src/lib/testi/pungolo.ts                 + titoloFine
app/src/lib/testi/interfaccia.json           testi nuovi; aula.costo_linea riscritto
app/src/lib/ossatura.ts                      fettaDi dà null per opposizione, eredità e schema; Bilancio barrato a partita finita
app/src/routes/partita/[sezione]/+page.svelte   monta Opposizione, Eredità, Schema
docs/superpowers/verifiche/11e/              screenshot della fetta
```

I test Vitest stanno accanto al modulo che provano. Un file `.svelte` che supera le 250 righe si divide per riquadro.

## PR e ordine

- **PR 1, Opposizione** (`feat/11e-opposizione`): Task 1-5.
- **PR 2, Eredità** (`feat/11e-eredita`): Task 6-9.
- **PR 3, Schema dello Stato** (`feat/11e-schema`): Task 10-12.

Le PR sono impilate: la 2 parte dalla 1, la 3 dalla 2 (memoria `pr-impilate-con-squash`: dopo lo squash della base, `merge -s ours origin/main` sul branch impilato). Dentro una PR i task vanno in sequenza, con queste eccezioni: 1 e 2 in parallelo (guscio Rust e moduli puri, cartelle diverse); 6 e 10 possono partire appena la PR 1 è chiusa (cartelle diverse), ma la 10 si fa nel branch della PR 3.

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo; `rust-reviewer` nella PR 1 (guscio). `craft-reviewer` riceve gli screenshot del task di verifica della PR e li confronta col prototipo e coi token.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta.

- **Dove ricerca e piano dicono cose diverse, vale il piano**: i prototipi e le decisioni del 17/11 sono venuti dopo le ricerche.
- **In un parallelo ogni subagente mette nel commit solo i file del proprio task**, nominandoli uno per uno in `git add`.

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `rust-engineer` | primo della PR 1, in parallelo col 2 | `rust-skills` |
| 2 | `frontend-engineer` | in parallelo con l'1 | salto dichiarato: moduli puri e guscio finto, nessuna interfaccia |
| 3 | `frontend-engineer` | dopo l'1 e il 2 | `impeccable`, `apple-design` |
| 4 | `frontend-engineer` | dopo il 3 | `impeccable`, `apple-design` |
| 5 | sessione principale | dopo il 4 | `scrittura-italiana` |
| 6 | `frontend-engineer` | primo della PR 2 | salto dichiarato: moduli puri e testi |
| 7 | `frontend-engineer` | dopo il 6 | `impeccable`, `dataviz`, `apple-design` |
| 8 | `frontend-engineer` | dopo il 7 | `impeccable`, `apple-design` |
| 9 | sessione principale | dopo l'8 | `scrittura-italiana` |
| 10 | `frontend-engineer` | primo della PR 3 | salto dichiarato: modulo puro |
| 11 | `frontend-engineer` | dopo il 10 | `impeccable`, `dataviz`, `apple-design` |
| 12 | sessione principale | dopo l'11 | `scrittura-italiana` |

«Almeno una fra» le skill elencate. Ogni brief nomina anche `skill-gate` e `caveman`; quelli dei subagenti che scrivono codice (tutti, fuori dalla sessione principale) anche `ponytail`.

## Dove va ogni storia

| Storia | Task |
|---|---|
| 131, 134, 135 | 2, 3 |
| 132, 133 | 4 |
| 136, 137 | 2 |
| 138 | 6, 7 |
| 139, 140 | 7 |
| 141, 142 | 10, 11 |
| 143 | 10 |

---

### Task 1: passacarte della sfiducia

**Subagente:** `rust-engineer`: il task tocca `.rs`.

**Sequenza:** primo della PR 1, in parallelo col Task 2.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:33`: `anteprima_sfiducia(dicastero?)` è il passacarte dell'opposizione; i numeri di riga di allora sono cambiati, valgono quelli sotto.

**Files:**
- Modify: `app/src-tauri/src/lib.rs` (comando e `generate_handler!`, riga 627); `app/src/lib/guscio/chiamate.ts` (riga 52-87)
- Test: `app/src-tauri/src/lib.rs` `mod tests`; `app/src/lib/guscio/chiamate.test.ts`

**Interfaces:**
- Consumes: `Partita::anteprima_sfiducia(&self, dicastero: Option<&str>) -> Result<AnteprimaSfiducia, Errore>` (`crates/sim-core/src/partita.rs:4437`); `sulla_partita` (`app/src-tauri/src/lib.rs:435`).
- Produces:
  - Rust: `anteprima_sfiducia(dicastero: Option<String>) -> Result<AnteprimaSfiducia, ErroreGuscio>`, involucro di una riga su `sulla_partita` come `anteprima_accogli` (`lib.rs:505`).
  - TS: `guscio.anteprimaSfiducia(dicastero: string | null) => Promise<AnteprimaSfiducia>`, che chiama `"anteprima_sfiducia"` con `{ dicastero }`.

- [ ] **Step 1: scrivi i test che falliscono.** In `lib.rs`: `anteprima_sfiducia_senza_partita_e_un_guasto` (`Err(guasto(NESSUNA))`); `anteprima_sfiducia_e_quella_del_core` (uguale al metodo del core su una partita all'opposizione); `anteprima_sfiducia_al_governo_e_un_rifiuto` (`Specie::Rifiuto` col testo del core). In `chiamate.test.ts`, la riga della tabella: `["anteprima_sfiducia", () => guscio.anteprimaSfiducia("sanita"), { dicastero: "sanita" }]`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml -p app`. Expected: errore di compilazione sul comando che manca.
- [ ] **Step 3: implementa** il comando, registralo, aggiungi la chiamata TS.
- [ ] **Step 4: verifica.** Run, uno per chiamata:
  - `CARGO_TARGET_DIR=<wt>/target TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`
  - `cargo clippy --manifest-path <wt>/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
  - `npm --prefix <wt>/app test -- chiamate`
  - `git -C <wt> status --short`

  Expected: verdi, clippy pulito, nessun file in `app/src/lib/tipi/` cambiato.
- [ ] **Step 5: commit.** `feat(app): passacarte dell'anteprima della sfiducia`

---

### Task 2: moduli puri dell'opposizione, linea condivisa, tracciato e guscio finto

**Subagente:** `frontend-engineer`.

**Sequenza:** in parallelo col Task 1: lavora in `app/src/lib/opposizione/`, `aula/`, `tracciati.ts`, `guscio/finto.ts`, `testi/`. Non tocca `chiamate.ts`: nei test usa `rispondi` del finto.

**Skill-gate:** salto dichiarato (moduli puri e guscio finto, nessuna interfaccia); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:64`: cosa porta la Vista all'opposizione (fasce, programma, mosse, agende nascoste per costruzione).

**Files:**
- Create: `app/src/lib/opposizione/opposizione.ts`, `opposizione.test.ts`; `app/src/lib/aula/Linea.svelte`
- Modify: `app/src/lib/aula/Misura.svelte` (righe 53 e 161-175); `app/src/lib/tracciati.ts` (righe 76-111) e il suo test; `app/src/lib/guscio/finto.ts` (righe 300-345) e il suo test; `app/src/lib/testi/interfaccia.json` (riga 936)

**Interfaces:**
- Consumes: `Vista`, `AnteprimaSfiducia`, `VoceMisura.costo_linea`, `VocePartito.posizione_percepita`.
- Produces, in `opposizione.ts`:
  - `leve(v: Vista): Leva[]`, dove `Leva = { tipo: "sfiducia_governo" | "sfiducia_ministro" | "posto" | "abrogativo" | "confermativo"; stato: "libera" | "presentata" | "altrui" | "spenta"; motivo: Chiave | null; chi: string | null }`. Il confermativo c'è solo se `referendum[]` ne ha uno `richiesto` o richiedibile; la sfiducia è `presentata` se `v.sfiducia?.partito === v.giocatore`, `altrui` se è di un altro (con `chi`); il posto è `libera` se `posto_riservato`, `altrui` con `chi = posto_riservato_di` se è di un altro, `presentata` se una misura in calendario ha `promotore === v.giocatore`.
  - `misureDelGoverno(v: Vista): VoceMisura[]`: il calendario senza le misure del giocatore, nell'ordine del voto.
  - `programma(v: Vista): VoceProgramma[]`: `v.programma` com'è, nell'ordine della Vista.
  - `maggioranzaAvversaria(v: Vista): { partito: string; seggi: [number, number]; fascia: FasciaIntesa | null }[]`: i partiti al governo coi seggi e la fascia da `alleati_avversari` (`null` per chi guida).
  - `siMancanti(v: Vista, a: AnteprimaSfiducia): number`: metà più uno della Camera meno i seggi alla Camera dei partiti con `si`; 0 se `a.passa`.
- Produces, in `tracciati.ts`: `tracciatiDi(v, serie, sfiducia?: AnteprimaSfiducia | null)`; con la sfiducia, il secondo tracciato è `tracciato.per_farlo_cadere` con valore `siMancanti`, scala di 15 seggi per casella, `rosso: false`, `rimando: null`.
- Produces, `Linea.svelte`: props `{ vista: Vista; misura: VoceMisura; onlinea: (si: boolean | null) => void; occupato?: string }`; disegna i tre bottoni, il default e il costo della linea scelta col testo della lettura 1. `Misura.svelte` lo usa al posto delle righe 161-175.
- Produces, `finto.ts`: carica `28-` e `30-` (il campo `opposizione` e `eredita` di `Campioni`); risponde `anteprima_sfiducia` dalle `anteprime` del 28 (`governo` per `dicastero: null`, `ministri[d]` altrimenti; `{ rifiuto }` come rifiuto), `comanda` con `Sfiducia` dal `dopo_comando` dello stato `sfiducia`, `abbandona`, `conferma_eredita` (null) e `riavvolgi` dalla `vista` del 30.

- [ ] **Step 1: scrivi i test che falliscono**, sulle Viste del 28 caricate dal file:
  - `leve` sullo stato `sfiducia` (`prima`): sfiducia al governo `libera`; dopo il comando (`dopo_comando`): `presentata`; posto con `posto_riservato_di` altrui: `altrui` col nome giusto; sullo stato `posto_mio`: `libera`, poi `presentata`.
  - `leve` con una Vista del 28 in cui `sfiducia = { dicastero: null, partito: "sinistra" }`: `altrui`, `chi = "sinistra"`.
  - `misureDelGoverno` sullo stato `linea`: nessuna misura col giocatore promotore.
  - `siMancanti` sulle anteprime del 28: al turno 2 governo `passa: false` → 201 − (seggi di estrema sinistra, estrema destra, giocatore); sullo stato `sfiducia_che_passa` → 0.
  - `tracciatiDi` con la sfiducia: secondo tracciato `per_farlo_cadere`, mai rosso; senza: com'è oggi.
  - `Linea`, sulla Vista dello stato `linea` (percepita del giocatore 1,9363): sul «Cuneo fiscale» il costo di `si` dice «va da 1,94 a 2,01» e quello di `default` «va da 1,94 a 2,06»; sulla «Legge di bilancio» (1,9363 in ogni ramo) dice «resta a 1,94».
  - finto: `rispondi("anteprima_sfiducia", { dicastero: null }, c)` dà l'anteprima del 28; il test che usava `anteprima_sfiducia` come comando ignoto (`finto.test.ts:73-77`) passa a un comando che il finto non ha.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- opposizione tracciati finto`. Expected: FAIL sui moduli e sulle chiavi che mancano.
- [ ] **Step 3: implementa** i moduli, `Linea.svelte`, il finto e il testo nuovo di `aula.costo_linea` (più `aula.costo_linea_resta`).
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): moduli dell'opposizione e costo della linea corretto`

---

### Task 3: Cruscotto all'opposizione, ordine del giorno

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo l'1 e il 2.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:64`: dati della Vista all'opposizione.

**Files:**
- Create: `app/src/lib/opposizione/Opposizione.svelte`, `Leve.svelte`, `Programma.svelte`
- Modify: `app/src/lib/ossatura.ts` (`CRUSCOTTO`, righe 65-67: `opposizione` a `null`); `app/src/routes/partita/[sezione]/+page.svelte` (righe 45 e 64-70); `app/src/lib/cornice/Cornice.svelte` (chiede l'anteprima per il tracciato); `app/src/lib/testi/interfaccia.json`
- Test: `app/src/lib/ossatura.test.ts`

**Interfaces:**
- Consumes: `leve`, `misureDelGoverno`, `programma`, `maggioranzaAvversaria`, `Linea.svelte` (Task 2); `guscio.anteprimaSfiducia` (Task 1); `Evento` con `puoRispondere=false`; `ultimoVince`.
- Produces: `Opposizione.svelte`, props `{ vista: Vista }`, con un dettaglio a lato che i Task 4 riempie: `let scelta = $state<{ tipo: "sfiducia" | "ministro" | "referendum" | "misura"; id: string | null } | null>(null)`.

Forma: prototipo 28, forma A, stato `ordinario`. Dall'alto: l'evento del governo in sola lettura; «Le tue leve», una riga per `Leva` col suo stato (sfiducia al governo, sfiducia a un ministro, posto riservato, referendum abrogativo, confermativo quando c'è); «Si vota a fine turno» e «In aula» con le misure del governo, ciascuna con `Linea` sulla riga; «Il programma del governo» nell'ordine della Vista, con la riga «Le agende dei ministri avversari non si vedono»; «La maggioranza» coi partiti al governo, i seggi e la fascia. Niente conto in cornice (resta com'è, `Cornice.svelte:70-76`).

- [ ] **Step 1: scrivi il test che fallisce**: `fettaDi("cruscotto", "opposizione")` è `null`.
- [ ] **Step 2: esegui e verifica che fallisca.** Run: `npm --prefix <wt>/app test -- ossatura`. Expected: FAIL.
- [ ] **Step 3: implementa** le tre righe di `ossatura.ts` e della rotta, `Opposizione.svelte` coi suoi riquadri, e nella cornice `anteprimaSfiducia(null)` a ogni Vista all'opposizione, con `ultimoVince`, passata a `tracciatiDi`.
- [ ] **Step 4: guarda la pagina** nel browser sulle Viste del 28: `/partita/cruscotto?vista=28-&stato=sfiducia`, `linea`, `intesa_varia`, `grave`, `fine`, `posto_mio`; e sul `06-`. Expected: la forma del prototipo; il tracciato «Per farlo cadere» pieno sul `sfiducia_che_passa`.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): cruscotto all'opposizione`

---

### Task 4: sfiducia, referendum e confermativo nel dettaglio a lato

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 3.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/modelli/iter-legislativo-italiano.md:77`: chi chiede il referendum confermativo e quando, per il testo della riga.
- `docs/ricerca/modelli/iter-legislativo-italiano.md:115`: cosa può fare l'opposizione con le regole in vigore.

**Files:**
- Create: `app/src/lib/opposizione/DettaglioSfiducia.svelte`, `DettaglioReferendum.svelte`
- Modify: `Opposizione.svelte`, `Leve.svelte`, `interfaccia.json`

**Interfaces:**
- Consumes: `guscio.anteprimaSfiducia`; `partita.comanda` con `{ tipo: "sfiducia", dicastero }`, `{ tipo: "promuovi_referendum", disposizione }`, `{ tipo: "confermativo", misura, chiedi }` (`app/src/lib/tipi/Comando.ts:10`); `Conferma.svelte`; `testoRifiuto`.

Forma: prototipo 28, stati `sfiducia`, `passa`, `referendum`. Sfiducia al governo: nel dettaglio, la linea di ogni partito (sì/no), l'esito «se nulla cambia» (`passa`), le firme a parole (lettura 2), cosa succede se è respinta (lettura 3), «Presenta la sfiducia» con la conferma sul posto. Sfiducia a un ministro: le dieci anteprime (lettura 5), un ministro scelto apre la stessa scheda. Abrogativo: le disposizioni (lettura 6) con la conferma sul posto. Confermativo: «Chiedi il referendum» · «Non chiederlo», senza conferma.

- [ ] **Step 1: prova nel browser** i tre stati del 28 e i rifiuti del finto (`rifiuto_seconda`, `rifiuto_secondo`, `rifiuto_non_referendabile`): dopo «Presenta», la riga in testa dice «presentata» e il bottone sparisce; il secondo comando mostra il rifiuto sotto la riga.
- [ ] **Step 2: un clic doppio fa una chiamata sola** (punto da sorvegliare 1): prova a mano col guscio finto, contando le chiamate a `comanda` in console.
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 4: commit.** `feat(app): sfiducia e referendum dal cruscotto all'opposizione`

---

### Task 5: verifica della PR 1

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 4. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** (`npm --prefix <wt>/app run verifica:chrome` e `verifica:webkit`, `app/verifica/README.md`): `/partita/cruscotto` sugli stati del Task 3, Step 4, e `/partita/aula?vista=19-&stato=opposizione_libero` per la linea condivisa, alle due finestre e alle due scale. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le leve, le conferme sul posto, la linea, il dettaglio e il ritorno.
- [ ] **Step 3: test e tipi.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`. Expected: verdi, pulito.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11e/`: il cruscotto all'opposizione nei suoi stati alle due finestre; la sfiducia che passa; l'abrogativo; l'Aula con la linea corretta; il tono grave.
- [ ] **Step 5: corpo della PR**: cosa cambia, decisioni prese in esecuzione, «Per Mario», testi nuovi, verifiche.

**Fine della PR 1.**

---

### Task 6: adattatore dell'eredità, titoli della fine e Bilancio barrato

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 2.

**Skill-gate:** salto dichiarato (moduli puri e testi); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:66`: cosa porta la Vista per l'eredità.

**Files:**
- Create: `app/src/lib/eredita/eredita.ts`, `eredita.test.ts`
- Modify: `app/src/lib/testi/pungolo.json`, `pungolo.ts`, `pungolo.test.ts`; `app/src/lib/ossatura.ts` (riga 54) e `ossatura.test.ts`; `interfaccia.json`

**Interfaces:**
- Consumes: `Vista.eredita`, `Vista.elezioni` (`EsitoElezioni`, coi seggi in `VotiPartito.camera` e `senato`), `Vista.disposizioni[].legge`, `PuntoSerie[]`, `VoceAlbo`, `variante` (`testi/t.ts:64-66`).
- Produces, in `eredita.ts`:
  - `type Edizione = { nome: string; scenario: string; inizio: Trimestre; fine: Trimestre; turni: number; esito: "senza_seggi" | "abbandono"; anni_al_governo: number; riavvolgimenti: number; leggi: { nome: string; legge: string | null }[]; indicatori: IndicatoreAlbo[]; serie: PuntoSerie[]; elezioni: ElezioneAlbo[]; bozza: boolean }`;
  - `daVista(v: Vista, serie: PuntoSerie[], nome: string, scenario: string, inizio: Trimestre): Edizione`, che copia i campi e porta `EsitoElezioni` alla forma di `ElezioneAlbo` (nomi dei partiti da `v.partiti`, seggi del giocatore da `VotiPartito`), senza calcolare;
  - `daAlbo(a: VoceAlbo): Edizione` con `bozza: false` e `legge: null` per ogni legge (decisione 3);
  - `occhiello(e: Edizione): string`: senza seggi «Voto di {trimestre} · {quota}, nessun seggio» dall'ultima elezione; abbandono «Partita lasciata · {trimestre della fine}» (decisione 4);
  - `titolo(e: Edizione): string`: senza seggi `titoloFine(e.nome, e.scenario, e.inizio)`; abbandono il titolo serio `eredita.titolo_abbandono` («{nome} lascia la partita»).
- Produces, in `pungolo.ts`: `titoloFine(nome: string, scenario: string, inizio: Trimestre): string`, `variante` sui sedici `titoli_fine` (lettura 10).
- Produces, in `ossatura.ts`: `barrata("bilancio", v)` dà `barrata.bilancio_finita` («Partita finita») prima di ogni altro motivo quando `v.finita`.

I sedici titoli, da copiare in `pungolo.json` sotto `titoli_fine`, nell'ordine:
1. Sfratto esecutivo: si spengono le luci in Parlamento
2. Gli elettori cambiano la serratura
3. Montecitorio restituisce le chiavi
4. Dalla stanza dei bottoni al corridoio
5. Le urne chiudono, il partito pure
6. Applausi, sipario, nessun bis
7. Arrivederci, e grazie per tutti gli emendamenti
8. Il seggio era in affitto: contratto scaduto
9. Nessun seggio, molti ricordi
10. Il popolo ha parlato, e non ha fatto il vostro nome
11. L'opposizione più dura: quella degli elettori
12. Si cercano parlamentari, astenersi perditempo
13. Il partito entra nella storia, dalla porta di servizio
14. Prossima fermata: i libri di storia
15. Gli elettori votano compatti: altrove
16. Fine delle trasmissioni

- [ ] **Step 1: scrivi i test che falliscono**:
  - `daVista` sulla Vista del 30: `esito "senza_seggi"`, `bozza: true`, due leggi con l'etichetta (`"L. 79/2031"` per la concorrenza nei servizi), elezioni nell'ordine della Vista con `seggi_giocatore [0, 0]` per l'ultima;
  - `daAlbo` sulla `voce_albo` del 30: stesse leggi coi nomi, `legge: null`; stessi indicatori di `daVista`;
  - `titolo(daVista(...))` uguale a `titolo(daAlbo(...))` sulla stessa partita;
  - `occhiello` senza seggi: «Voto di ottobre–dicembre 2031 · 2,4%, nessun seggio»; abbandono su una voce del `13-edicola.json` con esito «Abbandono»: «Partita lasciata · …»;
  - `pungolo.test.ts`: sedici titoli, nessuna cifra, nessun nome di partito dei dati;
  - `barrata("bilancio", v)` sulla `07-` e sul 30: `barrata.bilancio_finita`.
- [ ] **Step 2: esegui e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- eredita pungolo ossatura`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): adattatore dell'eredità e titoli della fine`

---

### Task 7: Eredità nella partita, con la bozza

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 6.

**Skill-gate:** `impeccable`, `dataviz` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Create: `app/src/lib/eredita/Eredita.svelte`, `Testa.svelte`, `Colonne.svelte`, `Bozza.svelte`
- Modify: `ossatura.ts` (`FETTA` e `CRUSCOTTO`, righe 61-67: `eredita` e `finita` a `null`); la rotta `[sezione]`; `app/src/lib/partita.svelte.ts` (riavvolgi e conferma); `app/src/routes/+page.svelte` (`?albo=nuova`)

**Interfaces:**
- Consumes: `Edizione`, `daVista`, `occhiello`, `titolo` (Task 6); `Grafico.svelte` (`app/src/lib/edicola/Grafico.svelte:7-19`) per i tre grafici; `guscio.riavvolgi(turno)`, `guscio.confermaEredita()`, `guscio.riprendi`; `voceNuova` (`edicola.ts:48`); `Conferma.svelte`.
- Produces: `Eredita.svelte`, props `{ edizione: Edizione; azioni: boolean }`, che il Task 8 monta dall'albo con `azioni: false`.

Forma: prototipo 30, forma A, stato `bozza`; la colonna delle leggi come il 37-fine con `leggi` su «schema 19». Testa con occhiello, titolo e bollo; riga con scenario, date, turni, anni al governo e riavvolgimenti; tre colonne (Le elezioni, Il Paese che lasci, Le leggi che restano) e i tre grafici. Dettaglio a lato: «Non è ancora scritta», «Torna al turno N · prima del voto ›» (lettura 7) e «Conferma l'eredità», confermati sul posto con ciò che si cancella; nessun «Un altro turno, nel Registro ›» (arriva con la 11f). Dopo la conferma: lettura 8.

- [ ] **Step 1: guarda la pagina** su `/partita/eredita?vista=30-` alle due finestre. Expected: la forma del 30, cornice «Partita finita · Senza seggi», tracciati grigi, «Nessun turno da chiudere».
- [ ] **Step 2: riavvolgimento.** Col finto, «Torna al turno 20» porta alla Vista del 30 al turno 20; con un guasto del finto la UI rifà `riprendi` e mostra il guasto (punto da sorvegliare 4). Un test sul modulo del negozio (`partita.svelte.ts`) copre il ramo del guasto.
- [ ] **Step 3: conferma.** «Conferma l'eredità» chiama `conferma_eredita`, poi apre l'edicola con la voce nuova scelta.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): eredità, ultima edizione con la bozza`

---

### Task 8: Eredità riletta dall'albo

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 7.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/edicola/DettaglioAlbo.svelte` (righe 43-46); `app/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `Eredita.svelte` con `azioni: false`; `daAlbo` (Task 6).

Forma: prototipo 30, stato `albo`: senza cornice, «‹ Edicola» in alto, nessuna azione (lettura 9). Una voce «Abbandono» ha la testa seria (decisione 4).

- [ ] **Step 1: guarda la pagina** dall'edicola (`/?vista=13-`): «Leggi l'eredità ›» su una voce senza seggi e su una d'abbandono; «‹ Edicola» torna al banco con la stessa voce scelta.
- [ ] **Step 2: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 3: commit.** `feat(app): eredità riletta dall'albo`

---

### Task 9: verifica della PR 2

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 8. Chiude la PR 2.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** su `/partita/eredita?vista=30-`, `/partita/eredita?vista=07-` e l'edicola con l'eredità aperta, alle due finestre e alle due scale. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le due conferme sul posto, «‹ Edicola».
- [ ] **Step 3: test e tipi**, come al Task 5, Step 3.
- [ ] **Step 4: giro nell'app vera** col telecomando (`app/verifica/README.md:63-98`), con `verifica:giro` lanciato per primo e in background. Il giro chiede una partita finita senza seggi: si gioca il seme 43 a 4,5 come il 30 (`viste/README.md:49`) solo se il guscio permette di fissare il seme; altrimenti si prova con l'abbandono (un'eredità senza bozza, dall'albo) e la bozza resta provata col finto, scritto nella PR.
- [ ] **Step 5: screenshot e corpo della PR**: l'Eredità in bozza e dall'albo (senza seggi e abbandono), alle due finestre; «Per Mario» con ciò che il giro non ha provato.

**Fine della PR 2.**

---

### Task 10: modello dello schema dello Stato

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 3.

**Skill-gate:** salto dichiarato (modulo puro); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:62`: lo schema si compone da campi che la Vista ha già, nessun campo nuovo.

**Files:**
- Create: `app/src/lib/schema/schema.ts`, `schema.test.ts`

**Interfaces:**
- Consumes: `Vista`.
- Produces, in `schema.ts`:
  - `type Colonna = 0 | 1 | 2 | 3` (Partiti, Parlamento, Governo, Paese e mercati);
  - `type Nodo = { id: string; colonna: Colonna; tipo: "partito" | "camera" | "presidente" | "ministero" | "paese" | "situazione" | "mercati"; nome: Chiave | string; sotto: string; spento: boolean; tuo: boolean; alGoverno: boolean }`;
  - `type Filo = { id: string; da: string; a: string; tipo: TipoFilo; etichetta: string; frase: string; contrario: boolean; ritardo: boolean; cantiere: boolean; sezione: Sezione; fonte: string }`;
  - `type TipoFilo = "maggioranza" | "agenda" | "cantiere" | "proposta" | "minaccia" | "dimissioni" | "intesa" | "misura" | "legge" | "posto" | "sfiducia" | "evento" | "situazione" | "spread"`;
  - `schema(v: Vista): { nodi: Nodo[]; fili: Filo[] }`, la tabella campo → filo della spec (spec:402) più le due righe della lettura 11. Il riferimento per ogni filo (capi, etichetta, frase, contrario, ritardo) è `modello()` nel prototipo 29; il verso è quello del prototipo (chi muove → chi riceve). I nodi in ordine: Presidente primo nel governo, Mercati primo nel Paese, situazioni in fondo.
  - Al turno zero (`fase === "sciolta"`): Presidente «uscente · {chi guida}», Camera e Senato `spento`, il filo «si vota a fine turno»; i partiti del governo uscente sono quelli con `al_governo` (01: centrodestra e destra). All'opposizione nessun filo di agenda: la Vista non le porta.

- [ ] **Step 1: scrivi i test che falliscono**, sulle Viste `04-`, `08-`, `01-` e sullo stato `fine` del 28:
  - 04: un filo `proposta` per ogni ministero con `proposta` (sei), verso il Presidente; due `maggioranza`; nessun filo con un capo senza nodo (punto da sorvegliare 5);
  - 01: Camera e Senato spenti, il filo «si vota a fine turno», governo uscente centrodestra e destra;
  - 28 `fine`: nessun filo `agenda`; un filo `intesa` per ogni voce di `alleati_avversari`;
  - una Vista con `sfiducia` e `posto_riservato_di` altrui (dal 28): un filo `sfiducia` e un filo `posto`.
- [ ] **Step 2: esegui e verifica che falliscano.** Run: `npm --prefix <wt>/app test -- schema`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): modello dello schema dello Stato`

---

### Task 11: la tavola per poteri

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il 10.

**Skill-gate:** `impeccable`, `dataviz` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Create: `app/src/lib/schema/Schema.svelte`, `Tavola.svelte`, `DettaglioSchema.svelte`
- Modify: `ossatura.ts` (`FETTA`, riga 63: `schema` a `null`); la rotta `[sezione]`; `interfaccia.json`

**Interfaces:**
- Consumes: `schema(v)` (Task 10).

Forma: prototipo 37-schema aperto su riga · pettine · punta (decisione 1), il resto dal 29 forma B. Testa «Tavola n. {turno} · Come funziona lo Stato» e «Chi tocca chi in {trimestre}»; quattro colonne fisse; un riquadro per riga; fili blu, rossi se contrari o a rischio, tratteggiati se arrivano più avanti, a puntini per un cantiere. Le etichette nella riga di chi muove; il pettine verso il Presidente; la punta dove il filo arriva. Clic su un filo o una riga: nel dettaglio a lato la frase intera, da chi a chi, e il rimando alla sezione («Governo ›»); per una riga, l'elenco dei suoi fili con il verso. Tutti i fili all'apertura; niente tondi, filtro e battute (lettura 12). Ogni filo e ogni riga è raggiungibile col Tab e ha un nome accessibile con la frase intera. In tono grave i colori diventano inchiostro, come nel 29.

- [ ] **Step 1: guarda la pagina** su `/partita/schema` con le Viste `04-`, `08-`, `01-`, `28-&stato=fine` alle due finestre. Expected: la forma del 37 e del 29; nessun testo fuori dalla tavola.
- [ ] **Step 2: tastiera, a mano**: Tab sulle righe e sui fili, Invio apre il dettaglio, Esc lo chiude.
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 4: commit.** `feat(app): schema dello Stato, tavola per poteri`

---

### Task 12: verifica della fetta

**Subagente:** sessione principale.

**Sequenza:** dopo l'11. Chiude la PR 3 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** sullo schema nei quattro stati del Task 11 e, di nuovo, sul cruscotto all'opposizione e sull'eredità. Expected: uscita 0, tagli 0.
- [ ] **Step 2: test e tipi**, come al Task 5, Step 3, più `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`.
- [ ] **Step 3: giro nell'app vera** col telecomando: una partita dell'Italia di oggi a 1,5 senza coalizione; al voto, poi all'opposizione: lo schema; il cruscotto con le leve; l'anteprima della sfiducia al governo e ai ministri; «Presenta la sfiducia» confermato; la linea su una misura del governo col costo; «Chiudi il turno» e l'esito della sfiducia; lo schema di nuovo. A fine giro si rimettono le impostazioni, si cancella la cartella di verifica e si ripristina `src-tauri/Cargo.toml` (`README.md:94-98`).
- [ ] **Step 4: screenshot** dello schema nei suoi stati in `docs/superpowers/verifiche/11e/`.
- [ ] **Step 5: corpo della PR 3.** Oltre alle sezioni solite: l'esito del giro; ciò che resta da provare a mano a Mario (Tab reale nella tavola, resa a occhio delle sbarre a 1,25×).
- [ ] **Step 6: commit dei ritocchi della verifica**, se ce ne sono. `fix(app): ritocchi dalla verifica della 11e`

**Fine della PR 3 e della fetta.**

## Rimandi al core

Nati scrivendo il piano. Vanno nei corpi delle PR e, se Mario vuole, nel prossimo piano di core.

- **Costo della sfiducia respinta**: né la Vista né `AnteprimaSfiducia` lo portano (lettura 3).
- **Etichetta delle leggi nella voce d'albo**: `VoceAlbo.leggi_in_vigore` ha solo i nomi (decisione 3).
- **Soglia delle firme**: la Vista non dice quante ne servono (lettura 2).
