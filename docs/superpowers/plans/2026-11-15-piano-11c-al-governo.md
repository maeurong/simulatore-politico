# Piano 11c «Al governo»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** al governo il giocatore decide il turno dal cruscotto, guida i dieci ministri, scrive e porta in aula le leggi e legge l'anteprima di ogni misura. La fetta porta quattro schermate (Cruscotto al governo e in crisi, Governo, Aula, Anteprima della misura), i motti dei ministri e i tre passacarte che restavano al core.

**Architettura:**
- Ogni schermata è una cartella in `app/src/lib/<sezione>/`: un modulo `.ts` puro, che legge la Vista e decide cosa mostrare, e i componenti Svelte che lo disegnano. Il modulo puro è provato con Vitest sulle Viste campione.
- Le scelte passano da `partita.comanda`; le anteprime e i costi dalle chiamate del guscio, con una richiesta in volo per volta (`ultimoVince`). Il rifiuto del core compare accanto alla scelta.
- Il guscio guadagna tre passacarte verso metodi già pubblici del core: `costo_dirotta`, `menu_elettorale`, `anteprima_proposta`. `sim-core` non cambia.
- La Vista dello schema 19 porta già costi di Ferma, perché dell'evidenza, indicatore di servizio, soglie, prezzo delle dimissioni, posti del calendario, costo ed effetti delle misure in calendario e norme del decreto d'evento: la fetta li usa (decisione 1).
- Il guscio finto impara a rispondere dai campioni 17, 18, 19 e 20, così ogni schermata si prova nel browser.

**Tecnologie:** quelle della 11b (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2, Rust). Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 92-117 (righe 135-162), i vincoli alle righe 248-371, le caricature alle righe 340-345, la sezione della fetta alle righe 383-390, le conferme alla riga 453 e i test alle righe 455-476. La PR di questo piano corregge la spec dove le decisioni del 15/11 la cambiano (righe 271, 341, 353, 357, 387, 388, 420). Chi esegue legge la spec e il ticket citato nel task.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `17-cruscotto.prototype.html`: forma A · Ordine del giorno, stati `ordinario`, `scelte`, `bilancio`, `ultimatum`, `provvisorio`, `crisi`, `crisiprov`, `dimissioni`, `grave`;
- `18-governo.prototype.html`: forma B · Figurine, Ferma diretto;
- `19-aula.prototype.html`: forma D · Misto;
- `20-anteprima.prototype.html`: forma D · Cartellino col prima e dopo;
- `35-scelte-11c.prototype.html`: colletti a fantasie (`?c=C`), chiave fissa del giocatore, la tabella dei motti.

I ticket che ogni task cita stanno in `.scratch/v1-piano-11-ui/issues/` (17, 18, 19, 20 e 09). I testi dell'interfaccia si copiano dai prototipi. Un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR. Un numero che un prototipo segna «11g» ora viene dalla Vista o dai tre passacarte; se non viene da lì, resta un posto vuoto che nomina la 11g.

**Da dove si parte:** main a `62a6a4a`, con la 11b fusa (#61, #63, #64, #66) e il core «esito e Vista completa» fuso (#62, #65, schema 19), più la PR delle Viste campione rigenerate sullo schema 19 (maeurong/simulatore-politico#68): solo campi nuovi, nessun valore cambiato, due giri con le stesse impronte. La 11c parte da main dopo la fusione della #68.

## Vincoli globali

Valgono tutti i vincoli globali dei piani 11a (`docs/superpowers/plans/2026-11-12-piano-11a-avvio.md:27-47`) e 11b (`docs/superpowers/plans/2026-11-13-piano-11b-al-voto.md:30-45`). Qui quelli che la fetta tocca di più, e quelli nuovi.

- **La UI legge, il core decide** (spec:250). Lecito: contare voci aperte della Vista, sommare variazioni e seggi, ordinare per campi, confrontare un valore con una soglia di `Vista.soglie`. Ogni altra cifra viene dal core.
- **Soglie dalla Vista.** Le soglie della soddisfazione (40 e 25) e dell'intesa si leggono da `Vista.soglie`; nessun 40 o 25 scritto nel codice della fetta.
- **Il guscio aggiunge passacarte** verso metodi già pubblici della partita, senza logica né stato (spec:253). `sim-core` non cambia.
- **Conferma sul posto** per «Dimettiti», «Poni la fiducia» (Aula e Cruscotto), «Chiedi le dimissioni», «Ferma il cantiere», l'allargamento in crisi e il referendum abrogativo. **Diretti**: «Ferma» col costo sul bottone, «Dirotta» col costo sul bottone, Accogli · Rinvia · Respingi, Accogli · Respingi sugli emendamenti, la scelta del successore, la risposta a un evento, «Emana il decreto», presentare una legge, la linea, il confermativo, «Invita all'astensione», il test della misura (spec:453).
- **Rifiuto del core accanto alla scelta**, col testo passato da `testoRifiuto`.
- **Bozze solo in memoria**: la legge in composizione, l'atto e la regione di Dirotta scelti e non ancora mandati (spec:268). Al rientro nella pagina vale ciò che dice la Vista.
- **Tono grave** finché un evento grave è aperto: iniziali al posto delle facce, niente motti, la carta non gira, il cartellino non oscilla, nessun colore d'accento (storie 117, 169-170).
- **Movimento**: la carta che gira e il cartellino che oscilla partono solo a un gesto, mai in loop; la durata segue la velocità (`fattori`, `app/src/lib/impostazioni.svelte.ts:45`); col movimento spento non partono.
- **Cargo da un worktree**: `--manifest-path <wt>/Cargo.toml`, `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi` e `CARGO_TARGET_DIR=<wt>/target`, tutti assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md:105`).
- **Commit** in Conventional Commits, col trailer del modello che scrive il commit.

## Decisioni di Mario del 15/11/2026

Prese su `35-scelte-11c.prototype.html`, in una sola domanda.

1. **La 11c usa lo schema 19, in tre PR.** I posti che la spec lasciava alla 11g per Governo, Aula, Anteprima e Cruscotto (riga 353: costi di Ferma e Dirotta, posti in calendario, combinazioni spente, anteprime fuori dalla bozza; riga 420: indicatore di servizio, perché dell'evidenza, soglie, prezzo delle dimissioni, anteprima della proposta d'opposizione e di una misura in calendario, norme del decreto d'evento) li riempie la 11c. Il genere delle facce resta alla 11g (spec:342), anche se `VoceMinistro.genere` c'è già. PR: Governo e motti; Aula e Anteprima; Cruscotto.
2. **Colletti a fantasie d'inchiostro**: pieno, vuoto, righe, pois, quadretti, tratteggio, al posto delle sei tinte (quattro somigliavano al colore di un partito). Vale per tutte le facce, leader compresi; il tecnico tiene l'abito grigio.
3. **Chiave fissa per il partito del giocatore**: nell'hash delle facce il suo partito è `giocatore`, non il nome scelto alla fondazione. Gli altri partiti restano col nome visualizzato.
4. **Tutti e quaranta i motti** della tabella del Task 3.

## Letture che la spec non fissa

Le ho decise scrivendo il piano. Mario le conferma col sì all'esecuzione, o le cambia prima.

1. **Il conto delle scelte al default** (storia 112) conta, come il prototipo 17 (`17-cruscotto.prototype.html:333`): gli eventi con più di un'opzione senza scelta; i dicasteri in interim con la rosa e senza successore scelto; le proposte dei ministri non rinviate; gli emendamenti aperti; il bilancio presentabile e non presentato, fuori dalla crisi; la crisi fuori dall'esercizio provvisorio. Una proposta rinviata non conta: la Vista non distingue «rinviata in questo turno» da «rinviata prima», e la riga scrive il suo default. Tutto ciò che non è un evento conta solo se il giocatore governa: al turno zero la Vista porta le proposte del governo uscente (sei nella 01), che non si mostrano. Estende la decisione 4 della 11b, che contava solo gli eventi.
2. **«Ferma» col segno**: `costo_ferma` arrotondato col segno, «Ferma −18», «Ferma +3» se il ministro non tiene all'atto, «Ferma» senza numero a zero.
3. **La forma della bozza** si legge dalla `norma` delle sue disposizioni: costituzionale se ce n'è una costituzionale, elettorale se ce n'è una elettorale, altrimenti riforma. Le regole sulle miscele (un dicastero per legge, la costituzionale da sola) le dice il core col suo rifiuto: la UI non le ripete.
4. **Il decreto d'evento**: la riga dell'evento elenca le norme di `VoceEvento.norme`; scelta una norma, il dettaglio a lato ne mostra i gradini come una bozza di una sola disposizione, il cartellino da `anteprima_decreto` e «Emana il decreto», diretto.
5. **L'allargamento in crisi** riusa la logica della trattativa (`fuori`, `allarga` di `app/src/lib/trattativa/trattativa.ts:65-71`): i disponibili si spuntano, «Allarga a X e Y» passa dalla conferma sul posto, come l'allargamento in trattativa.

## Punti da sorvegliare

I casi che la spec implica e che più facilmente rompono il gioco. Ognuno ha il suo test nel task indicato.

1. **Doppio clic su una mossa che si fa una volta** («Dimettiti», «Poni la fiducia», «Chiedi le dimissioni», «Ferma il cantiere», «Ferma»): parte un comando solo, e il bottone resta spento mentre `partita.occupato`. Task 5 e Task 13.
2. **Corsa delle anteprime.** Chi cambia gradino in fretta fa partire molte `anteprima`: ne resta in volo una per volta, l'ultima vince, una risposta vecchia non copre la nuova; lo stesso per `menu_elettorale` e per i costi di Dirotta. Task 8 e Task 5.
3. **Il conto al default cambia a ogni scelta.** Rispondere a un evento, rinviare una proposta, decidere un emendamento o scegliere un successore abbassa il conto della cornice e del gruppo al rientro della Vista. Task 12.
4. **Interim senza ministro.** `VoceMinistero.ministro` è `null`: la carta, la riga del cruscotto e il dettaglio reggono senza nome, senza voto e senza agenda, e mostrano la rosa. Task 4 e Task 12, sulla Vista 05 (sei interim).
5. **Calendario pieno e posti.** Una bozza con il posto già occupato mostra il rifiuto del core sul posto, e il calendario conta i posti da `Vista.posti`, mai dalle misure; nel turno del bilancio la fila è un posto solo. Task 7 e Task 8.

## Struttura dei file

```
app/src-tauri/src/lib.rs                    + costo_dirotta, menu_elettorale, anteprima_proposta
.scratch/v1-piano-11-ui/viste/genera/       + costi di Dirotta, menu elettorale e anteprima della proposta nei campioni 18 e 19
app/src/lib/guscio/chiamate.ts              + costoDirotta, menuElettorale, anteprimaProposta
app/src/lib/guscio/finto.ts                 + risposte dai campioni 17, 18, 19, 20
app/src/lib/caricature/                     kit.ts (FANTASIE), Caricatura.svelte, Fantasie.svelte, caricatura.ts (chiaveDi, fondoDi con le soglie)
app/src/lib/componenti/                     Faccia.svelte (+ fondo), Emiciclo.svelte
app/src/lib/testi/                          pungolo.json, pungolo.ts
app/src/lib/governo/                        governo.ts, Governo.svelte, Figurina.svelte, CartaMinistro.svelte, Dirotta.svelte, Rosa.svelte, Cantieri.svelte
app/src/lib/aula/                           aula.ts, bozza.ts, elettorale.ts, Aula.svelte, Calendario.svelte, InAula.svelte, Misura.svelte, Disposizioni.svelte, Bozza.svelte, Elettorale.svelte, Referendum.svelte
app/src/lib/anteprima/                      anteprima.ts, oscilla.ts, Cartellino.svelte
app/src/lib/cruscotto/                      cruscotto.ts (+ ordine del giorno), Cruscotto.svelte, Evento.svelte (+ fondo, norme), Voce.svelte, Crisi.svelte, DettaglioVoce.svelte
app/src/lib/ossatura.ts                     fettaDi dà null per ciò che la fetta costruisce
app/src/routes/partita/[sezione]/+page.svelte   monta Governo, Aula e il cruscotto al governo
app/src/routes/+layout.svelte               monta Fantasie una volta
docs/superpowers/verifiche/11c/             screenshot della fetta
```

I test Vitest stanno accanto al modulo che provano. Un file `.svelte` che supera le 250 righe si divide per riquadro.

## PR e ordine

- **PR 1, Governo e motti** (`feat/11c-governo`): Task 1-6.
- **PR 2, Aula e Anteprima della misura** (`feat/11c-aula-anteprima`): Task 7-11.
- **PR 3, Cruscotto al governo e crisi** (`feat/11c-cruscotto`): Task 12-14.

Ogni PR parte da main dopo la fusione della precedente. Dentro una PR i task vanno in sequenza, con queste eccezioni: 1 e 3 in parallelo; 2 dopo l'1; 8 e 10 in parallelo dopo il 7. Toccano cartelle diverse.

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo; `rust-reviewer` nella PR 1. `craft-reviewer` riceve gli screenshot del task di verifica della PR e li confronta col prototipo e coi token.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta.

- **Dove ricerca e piano dicono cose diverse, vale il piano**: i prototipi e le decisioni del 15/11 sono venuti dopo le ricerche. Gli scarti noti sono scritti accanto al riferimento.
- **In un parallelo ogni subagente mette nel commit solo i file del proprio task**, nominandoli uno per uno in `git add`. Test e `check` girano su tutta l'app: un rosso che nasce nella cartella dell'altro task si riporta a chi orchestra e resta com'è.

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `rust-engineer` | primo della PR 1, in parallelo col 3 | `rust-skills` |
| 2 | `frontend-engineer` | dopo l'1 | salto dichiarato: guscio finto e chiamate, nessuna interfaccia |
| 3 | `frontend-engineer` | in parallelo con l'1 | `impeccable`, `apple-design` |
| 4 | `frontend-engineer` | dopo il 2 e il 3 | `impeccable`, `dataviz`, `apple-design` |
| 5 | `frontend-engineer` | dopo il 4 | `impeccable`, `dataviz`, `apple-design` |
| 6 | sessione principale | dopo il 5 | `scrittura-italiana` |
| 7 | `frontend-engineer` | primo della PR 2 | `impeccable`, `dataviz`, `apple-design` |
| 8 | `frontend-engineer` | dopo il 7, in parallelo col 10 | `impeccable`, `dataviz`, `apple-design` |
| 9 | `frontend-engineer` | dopo l'8 e il 10 | `impeccable`, `dataviz`, `apple-design` |
| 10 | `frontend-engineer` | dopo il 7, in parallelo con l'8 | `impeccable`, `dataviz`, `apple-design` |
| 11 | sessione principale | dopo il 9 | `scrittura-italiana` |
| 12 | `frontend-engineer` | primo della PR 3 | `impeccable`, `dataviz`, `apple-design` |
| 13 | `frontend-engineer` | dopo il 12 | `impeccable`, `dataviz`, `apple-design` |
| 14 | sessione principale | dopo il 13 | `scrittura-italiana` |

«Almeno una fra» le skill elencate. Ogni brief nomina anche `skill-gate` e `caveman`; quelli dei subagenti che scrivono codice (tutti, fuori dalla sessione principale) anche `ponytail`.

## Dove va ogni storia

| Storia | Task | Resta alla 11g |
|---|---|---|
| 92, 93, 94, 112 | 12 | |
| 95, 96 | 13 | |
| 97, 111 | 13 (cruscotto), 7 (fiducia in Aula) | |
| 98, 103 | 3, 4 | genere dei ministri (storia 168) |
| 99, 100 | 4, 5 | |
| 101, 102 | 5 | |
| 104, 105, 106 | 7 | |
| 107, 108 | 8 | |
| 109, 110 | 9 | |
| 113, 114, 115, 116, 117 | 10 | |

---

### Task 1: passacarte del governo e campioni

**Subagente:** `rust-engineer`: il task tocca `.rs`.

**Sequenza:** primo della PR 1, in parallelo col Task 3. Il Task 2 aspetta il suo commit.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:54`: Ferma e Dirotta senza anteprima del costo nella Vista di allora. Oggi `VoceAgenda.costo_ferma` c'è; manca solo il passacarte di `costo_dirotta`.
- `docs/ricerca/sintesi/vista-per-schermata.md:55`: i posti e le combinazioni della legge elettorale come dati mancanti; oggi `Vista.posti` c'è e `menu_elettorale` è un metodo pubblico.

**Files:**
- Modify: `app/src-tauri/src/lib.rs` (comandi e `generate_handler!`); `.scratch/v1-piano-11-ui/viste/genera/src/governo.rs`, `genera/src/main.rs`; `.scratch/v1-piano-11-ui/viste/README.md`
- Rigenerati: `.scratch/v1-piano-11-ui/viste/18-governo.json`, `19-aula.json`

**Interfaces:**
- Consumes: `Partita::costo_dirotta(&self, dicastero: &str, atto: Option<&str>, regione: Option<&str>) -> Result<f64, Errore>` (`crates/sim-core/src/partita.rs:2268`); `Partita::menu_elettorale(&self, valori: &[ValoreProposto]) -> Vec<VoceMenu>` (`:2232`); `Partita::anteprima_proposta(&self, valori: &[ValoreProposto]) -> Result<Anteprima, Errore>` (`:2208`); `sulla_partita` (`app/src-tauri/src/lib.rs:434-441`).
- Produces, i comandi, involucri di una riga su `sulla_partita` (la partita assente è un guasto, l'errore del core un rifiuto):
  - `costo_dirotta(dicastero: String, atto: Option<String>, regione: Option<String>) -> Result<f64, ErroreGuscio>`
  - `menu_elettorale(valori: Vec<ValoreProposto>) -> Result<Vec<VoceMenu>, ErroreGuscio>`
  - `anteprima_proposta(valori: Vec<ValoreProposto>) -> Result<Anteprima, ErroreGuscio>`
- Produces, nei campioni:
  - `18-governo.json["costo_dirotta_03"]`: per ogni dicastero della 03, `{ "atti": { <atto>: numero | { "rifiuto": testo } }, "regioni": { <regione>: numero | { "rifiuto": testo } } }`, dal metodo del core, ogni atto con `regione` nulla e ogni regione con `atto` nullo;
  - `19-aula.json["menu_03"]`: `menu_elettorale(&[])` sulla 03; `19-aula.json["menu_proporzionale"]`: lo stesso con i valori della legge elettorale «proporzionale al 3%» dello stato `piena`;
  - `19-aula.json["anteprima_proposta"]`: `{ "valori", "ok" | "rifiuto" }` sullo stato `opposizione_libero`, con `cittadinanza` a `ius_soli_temperato`.

- [ ] **Step 1: scrivi i test che falliscono**, in `lib.rs` `mod tests`, sulle funzioni interne come fanno i test di `anteprima_campagna`:
  - `costo_dirotta_senza_partita_e_un_guasto`, `menu_senza_partita_e_un_guasto`, `anteprima_proposta_senza_partita_e_un_guasto`: `Err(guasto(NESSUNA))`;
  - `costo_dirotta_al_turno_zero_e_un_rifiuto`: al turno zero il giocatore non governa; `Specie::Rifiuto`, col testo di `Partita::costo_dirotta` sulla stessa partita;
  - `il_menu_e_quello_del_core`: `menu_elettorale(vec![])` uguale a `Partita::menu_elettorale(&[])` sulla stessa partita, e non vuoto;
  - `l_anteprima_della_proposta_e_quella_del_core`: con una bozza su `cittadinanza`, il risultato del guscio (valore o testo del rifiuto) uguale a quello di `Partita::anteprima_proposta`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml -p app`. Expected: errore di compilazione sui comandi che mancano. Il nome del package si legge in `app/src-tauri/Cargo.toml`.
- [ ] **Step 3: implementa** i tre comandi e registrali.
- [ ] **Step 4: aggiungi le voci al generatore** e un controllo: dove `costi_03` ha la soddisfazione misurata comandando una copia della partita, `costo_dirotta_03` deve dare lo stesso numero entro 1e-9; uno scarto si stampa su stderr come `SCARTO`. Aggiungi le tre voci al README, nella riga del 18 e del 19.
- [ ] **Step 5: esegui tutto e rigenera.** Run, uno per chiamata:
  - `CARGO_TARGET_DIR=<wt>/target TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`
  - `cargo clippy --manifest-path <wt>/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
  - `CARGO_TARGET_DIR=<wt>/target TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo run --release --manifest-path <wt>/.scratch/v1-piano-11-ui/viste/genera/Cargo.toml`, due volte, con `shasum` dei 25 file dopo ciascuna
  - `git -C <wt> status --short`

  Expected: test verdi, clippy pulito, nessun `SCARTO`, impronte uguali nei due giri; cambiano solo `18-governo.json`, `19-aula.json`, il README e il generatore, e nessun file in `app/src/lib/tipi/`.
- [ ] **Step 6: commit.** `feat(app): passacarte di Dirotta, menu elettorale e proposta d'opposizione`

---

### Task 2: chiamate e guscio finto della fetta

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 1. I Task 4-13 consumano ciò che produce.

**Skill-gate:** nessuna skill del ruolo ha il trigger su chiamate e guscio finto senza interfaccia: il salto si dichiara nel report. `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/guscio/chiamate.ts`, `chiamate.test.ts`, `finto.ts`, `finto.test.ts`

**Interfaces:**
- Consumes: i comandi del Task 1; `VoceMenu`, `Anteprima`, `ValoreProposto` (`app/src/lib/tipi/`).
- Produces:

```ts
// chiamate.ts
guscio.costoDirotta(dicastero: string, atto: string | null, regione: string | null): Promise<number>   // invoke "costo_dirotta", { dicastero, atto, regione }
guscio.menuElettorale(valori: ValoreProposto[]): Promise<VoceMenu[]>
guscio.anteprimaProposta(valori: ValoreProposto[]): Promise<Anteprima>
```

**Guscio finto.** `installaFinto` carica sempre anche `17-`, `18-`, `19-` e `20-` (come oggi `23-`, `26-`, `27-`). `rispondi` impara:

| Comando | Risposta |
|---|---|
| `anteprima` | la voce di `20-anteprima.json["anteprime"]` o di `19-aula.json` (`anteprima_riforma`, `anteprima_elettorale`) con gli stessi `valori`: la sua `anteprima` o il suo `rifiuto` |
| `anteprima_accogli` | `17-cruscotto.json["anteprima_accogli_03"]` per `infrastrutture_trasporti`, `20-anteprima.json["anteprime"]["accogli"]` per il dicastero che porta |
| `anteprima_decreto` | `20-anteprima.json["decreto"]` o `["decreto_grave"]` con gli stessi valori |
| `anteprima_proposta` | `19-aula.json["anteprima_proposta"]` con gli stessi valori, altrimenti `anteprima_proposta_rifiuto` |
| `menu_elettorale` | `menu_03` con valori vuoti, `menu_proporzionale` con quelli della proporzionale |
| `costo_dirotta` | `18-governo.json["costo_dirotta_03"][dicastero]`, sotto `atti` o `regioni` |
| `comanda` con `ferma`, `dirotta`, `chiedi_dimissioni` | `18-governo.json["scelte"].vista`, se il comando è una delle quattro `mosse` di `scelte` |
| `comanda` con `scegli_successore` | `18-governo.json["successore_scelto"]` (`vista` o `rifiuto`) |
| `comanda` con `dimettiti` | `17-cruscotto.json["dimissioni"].vista` |
| `comanda` con `rinvia` su Ambiente, `rispondi_evento` «debito» | `17-cruscotto.json["scelte_fatte"].vista` |
| `comanda` con `presenta_legge` | `19-aula.json["piena"].vista` con i `valori` di una delle tre misure che il giocatore ha presentato in `piena.vista.calendario` (id 1, 2 e 3); la seconda costituzionale dà `rifiuto_calendario_costituzionale` |
| `comanda` con `presenta_proposta`, `linea` | `19-aula.json["opposizione_proposta"].vista` |
| `comanda` con `compra` del test della misura | la Vista del campione con `acquisti.test_misura` dalla voce di `20-anteprima.json["test"]` con gli stessi valori, o il suo rifiuto |
| il resto | come oggi |

La forma di ogni voce si legge nel campione e nella sua riga di `.scratch/v1-piano-11-ui/viste/README.md`. Un comando che il campione non ha giocato resta un rifiuto «non nel campione»: il finto non inventa Viste.

- [ ] **Step 1: scrivi i test che falliscono.**
  - `chiamate.test.ts`: il test dei nomi dei comandi conta tre nomi in più; `costoDirotta("sanita", "sanita_case_comunita", null)` chiama `costo_dirotta` con `{ dicastero: "sanita", atto: "sanita_case_comunita", regione: null }`.
  - `finto.test.ts`, una prova per riga della tabella, ciascuna con l'oracolo nel campione: `costo_dirotta` dà il numero di `costo_dirotta_03`; `menu_elettorale([])` ha tante voci quante `menu_03`; `ferma` su Lavoro sulla 03 dà una Vista con l'agenda di Lavoro `fermata`; `presenta_legge` della seconda costituzionale lancia un rifiuto col testo «calendario pieno»; `dimettiti` dà `dimissioni` vero.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): chiamate e guscio finto della fetta 11c`

---

### Task 3: facce dei ministri, colletti a fantasie e motti

**Subagente:** `frontend-engineer`.

**Sequenza:** in parallelo col Task 1: lavora solo in `caricature/`, `componenti/Faccia.svelte`, `testi/` e nel layout.

**Skill-gate:** almeno una fra `impeccable` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:263`: il colletto a motivo (pieno, righe, scacchi, pois) come leva d'identità: è la decisione 2. Vale il piano: inchiostro su carta, non i cinque inchiostri scuri.
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:43`: il colletto a motivo al posto dei capelli.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:79`: nel tono grave il motto sparisce insieme alla vignetta.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:22`: dieci ministri, figurine a 62 px.

**Files:**
- Modify: `app/src/lib/caricature/kit.ts`, `kit.test.ts`, `Caricatura.svelte`, `caricatura.ts`, `caricatura.test.ts`; `app/src/lib/componenti/Faccia.svelte`; `app/src/routes/+layout.svelte`; `app/src/routes/sviluppo/caricature/+page.svelte`
- Create: `app/src/lib/caricature/Fantasie.svelte`; `app/src/lib/testi/pungolo.json`, `pungolo.ts`, `pungolo.test.ts`

**Interfaces:**
- Consumes: `leveDi`, `salti`, `saltiDeiLeader` (`app/src/lib/caricature/caricatura.ts:21-61`); `variante` (`app/src/lib/testi/t.ts:64`); `nomeDi` (`app/src/lib/partiti.ts`); `VistaSoglie`.
- Produces:

```ts
// kit.ts: sostituisce COLLETTI; l'indice è la leva colletto, la rosa non cambia
export const FANTASIE = ["pieno", "vuoto", "righe", "pois", "quadretti", "tratteggio"] as const;
// caricatura.ts
/** La chiave del partito nell'hash: `giocatore` per il partito del giocatore (decisione 3), il nome visualizzato per gli altri. */
export function chiaveDi(v: Vista, partito: string): string;
/** Il fondo del riquadro: carta; giallo sotto soglie.minaccia; rosso sotto soglie.dimissioni. */
export function fondoDi(soddisfazione: number | null, soglie: VistaSoglie): "carta" | "giallo" | "rosso";
// pungolo.ts
/** Il motto del ministro: hash(nome, dicastero) mod n sui motti del dicastero (spec:319); null se il dicastero non ne ha. */
export function mottoDi(nome: string, dicastero: string): string | null;
```

- `Fantasie.svelte`: un `<svg>` nascosto con sei `<pattern>` (`id="fantasia-<nome>"`), montato una volta in `+layout.svelte`. I disegni vengono dalle definizioni `pat0`-`pat5` di `35-scelte-11c.prototype.html`, con `var(--ink)` e `var(--carta)`. Il colletto di `Caricatura.svelte` riempie con `url(#fantasia-<nome>)`; il tecnico tiene `var(--grigio)`.
- `Faccia.svelte` guadagna `fondo?: "carta" | "giallo" | "rosso"` e lo passa alla `Caricatura`. Chi disegna un ministro passa `partito={chiaveDi(vista, m.partito)}`; chi disegna una rosa passa i salti di `salti("ministro", …)` sulla stessa chiave.
- `pungolo.json`: `{ "motti": { "<id del dicastero>": [quattro motti] } }`, le dieci chiavi di `Vista.dicasteri`, nell'ordine della tabella. I componenti del tono grave non importano `pungolo.ts` (spec:315): il motto si nasconde, non si calcola.

**I quaranta motti** (decisione 4). Il primo di ogni riga viene da `18-governo.prototype.html:398`; gli apostrofi sono tipografici.

| Dicastero | Motti |
|---|---|
| `economia_finanze` | Il centesimo è sacro, il resto si vedrà · Ogni euro ha la sua copertura. Quasi ogni euro · Rinvio la spesa, anticipo il comunicato · Taglio dove non si vede, annuncio dove si vede |
| `sviluppo_economico` | Ne parliamo al prossimo tavolo · Ho convocato un tavolo sui tavoli · Incentivo tutto, anche gli incentivi · L’impresa viene prima. Dopo la riunione |
| `lavoro_welfare` | Il lavoro nobilita, la riforma pure · Ascolto le parti, a turno, a lungo · Rinnovo il contratto e il comunicato · Creo posti di lavoro: uno lo occupo io |
| `sanita` | Prima il paziente, poi il bilancio, poi la lettera · Le liste d’attesa le aspetto anch’io · Curo prima il comunicato, poi il reparto · Ho un piano per ogni reparto. Il personale arriva col prossimo |
| `istruzione_ricerca` | Chi non studia ripete. Anche il ministro · Riformo la scuola ogni anno, perché nessuno si annoi · La ricerca è il futuro: la finanzierò in futuro · Promuovo tutti, a cominciare dalla riforma |
| `infrastrutture_trasporti` | Ogni cantiere merita due inaugurazioni · Il ponte lo vedo già, sul plastico · Poso la prima pietra; la seconda può aspettare · Il treno arriva. Io arrivo prima, per la foto |
| `ambiente_energia` | Il futuro è verde, il bilancio un po’ meno · Ogni transizione ha la sua proroga · Spengo la luce quando esco dal convegno · Risparmio energia: firmo una volta sola |
| `interno` | Ordine, disciplina e un caffè · Ho tutto sotto controllo, tranne il controllo · La sicurezza è percepita. Io la percepisco benissimo · Presidio il territorio dalla scrivania |
| `giustizia` | Mi riservo di rispondere, in tre gradi · La riforma è pronta, in attesa di giudizio · I tempi della giustizia sono certi: lunghi · Le sentenze non le commento. Le aspetto |
| `esteri_difesa` | Dipende. In sei lingue · Esprimo preoccupazione con fermezza · Una stretta di mano costa meno di un trattato · Sto al fianco di tutti. Un po’ di lato, per prudenza |

- [ ] **Step 1: scrivi i test che falliscono.**
  - `caricatura.test.ts`: con `soglie` di `03-governo-ordinario.json`, `fondoDi(39.9)` giallo, `fondoDi(40)` carta, `fondoDi(24.9)` rosso, `fondoDi(null)` carta; `chiaveDi(v03, "giocatore")` è `"giocatore"`, `chiaveDi(v03, "destra")` è `"Destra"`; i test della rosa e dei leader restano verdi.
  - `kit.test.ts`: `FANTASIE` ha sei voci; il colletto di ogni faccia riempie con `url(#fantasia-…)`, quello del tecnico col grigio; il tetto di 3 KB per faccia resta.
  - `pungolo.test.ts`: quaranta motti, quattro per ciascuno dei dieci id di `Vista.dicasteri` della 03; `mottoDi("Simone Gallo", "economia_finanze")` è uno dei quattro di Economia e due chiamate danno lo stesso; `mottoDi("Simone Gallo", "nessuno")` è `null`; nessun motto contiene una cifra.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa**, poi aggiorna la pagina `/sviluppo/caricature`: i dieci ministri della 03 con la chiave fissa e il fondo, e la rosa dell'interim di `18-governo.json["interim"]`.
- [ ] **Step 4: verifica nel browser** su `/sviluppo/caricature` accanto a `35-scelte-11c.prototype.html?c=C`: le sei fantasie si distinguono a 34 px; le facce dei leader cambiano solo nel colletto. Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): colletti a fantasie, chiave fissa del giocatore e motti dei ministri`

---

### Task 4: Governo, album e carta del ministro

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 2 e il Task 3. È il primo della fetta a modificare `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md:173`: l'album di figurine come metafora del governo.
- `docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md:190`: la forma della figurina. Per bordi, ombre e colori valgono il prototipo 18 e i token delle fondamenta.
- `docs/ricerca/estetica/tono-giocoso-su-temi-seri.md:89`: il borsino dei ministri, voto e freccia. La forma A · Listino è scartata (ticket 18).

**Files:**
- Create: `app/src/lib/governo/governo.ts`, `governo.test.ts`, `Governo.svelte`, `Figurina.svelte`, `CartaMinistro.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `Faccia`, `chiaveDi`, `fondoDi`, `mottoDi` (Task 3); `nomeDi`; `Dettaglio`, `VoceCausa`, `Caselle`.
- Produces:

```ts
export type StatoCarta = "preavviso" | "rodaggio" | "tecnico" | "dimissioni_chieste" | "interim";
export type Figurina = {
  indice: number; dicastero: string; nomeDicastero: string;
  ministro: VoceMinistro | null; stati: StatoCarta[];
  voto: number | null;            // soddisfazione arrotondata
  freccia: number;                // somma delle variazioni del ministro (gli ultimi due turni, comandi compresi)
  evidenza: MotivoEvidenza[];     // agenda.evidenza; vuota = niente filo rosso
  agenda: VoceAgenda | null; senzaAgenda: string | null;
};
/** Le dieci carte nell'ordine di Vista.dicasteri. */
export function figurine(v: Vista): Figurina[];
/** L'indicatore di servizio del dicastero (VoceDicastero.indicatore in Vista.grandezze) contro l'obiettivo del ministro. */
export function obiettivo(v: Vista, dicastero: string): { nome: string; valore: number | null; obiettivo: number | null; verso: string };
/** La posizione del ministro accanto a quella del giocatore (posizione_percepita del suo partito). */
export function posizioni(v: Vista, m: VoceMinistro): { ministro: number; tua: number };
```

- `fettaDi("governo", …)` dà `null`: la pagina monta `Governo`.
- `Governo.svelte` (storia 98): l'album 5 × 2 sotto la cornice, col dettaglio a lato; l'album resta fermo, il dettaglio scorre da solo; a 1280×800 l'album sta nella pagina.
- `Figurina.svelte`: il numero, lo stato, il voto con la freccia (`NumeroFreccia`), la faccia a 62 px col fondo della soddisfazione, nome, dicastero e partito, il motto (fuori dal tono grave), l'atto in agenda con regione e costo, oppure il riquadro tratteggiato col motivo di `senzaAgenda`. Il filo rosso in testa quando `evidenza` non è vuota. I bottoni di Ferma e Dirotta li porta il Task 5. Senza ministro, la carta dice «interim» e il numero della rosa.
- `CartaMinistro.svelte`, nel dettaglio a lato (storia 100): faccia a 84 px; capacità a pallini (nel rodaggio l'ultimo a retino); motto; soddisfazione con la barra e i fili di `soglie.minaccia` e `soglie.dimissioni`, sotto le variazioni con la causa (un testo per ciascuna delle quindici `Causa`); la posizione sull'asse accanto alla tua; l'obiettivo; l'atto col perché dell'evidenza (un testo per `minaccia`, `distanza`, `evento`); il margine residuo; la proposta in sospeso col rimando «Cruscotto ›». Senza scelta il dettaglio mostra il primo ministro in evidenza, o il primo dell'album.

- [ ] **Step 1: scrivi i test che falliscono**, in `governo.test.ts`:
  - `figurine(03)`: dieci, nell'ordine di `dicasteri`; evidenza `["distanza"]` su Economia e Sviluppo economico, `["evento"]` su Ambiente, vuota sugli altri; `senzaAgenda` «margine esaurito» su Infrastrutture ed Esteri; `stati` contiene `rodaggio` su tutte e dieci in `18-governo.json["scelte"].vista` (primo turno di governo);
  - su `["scelte"].vista`: Lavoro ha la freccia −11,6 a un decimale (l'atto fermato) e voto 48; Ambiente ha `dimissioni_chieste`;
  - su `["scontenti"].vista`: sei carte con `preavviso`; su `["tecnico"].vista`: Giustizia con `tecnico`; su `["interim"].vista`: Ambiente con `interim`, `ministro` nullo e `voto` nullo;
  - `obiettivo(03, "sanita")`: nome «Qualità del servizio sanitario», valore 99,7 a un decimale, obiettivo 104,7;
  - su `05-crisi.json`: sei carte in interim (punto da sorvegliare 4);
  - `ossatura.test.ts`: `fettaDi("governo", "governo")` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `18-governo.prototype.html`, su `/partita/governo?vista=03-governo-ordinario.json` e su `?vista=18-governo.json&stato=` `scontenti`, `interim`, `tecnico`, `grave`: l'album a 1280×800 sta nella pagina; nel tono grave le iniziali e niente motti; la carta del ministro si raggiunge da tastiera. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Governo, album dei ministri e carta del ministro`

---

### Task 5: Governo, Ferma, Dirotta, cantieri, dimissioni e rosa

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 4: stessi `governo.ts`, `Figurina.svelte`, `CartaMinistro.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md:204`: fermare un atto gira la figurina sul retro. Vale il piano: Web Animations con la molla, non `scaleX`.
- `docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:45`: la molla come curva `linear()` su una sola animazione.
- `docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:58`: `finish()` sulle Web Animations, e la doppia animazione delle transizioni di Svelte.
- `docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md:21`: l'oscillazione continua dà nausea: la carta gira una volta, al gesto.

**Files:**
- Create: `app/src/lib/governo/Dirotta.svelte`, `Rosa.svelte`, `Cantieri.svelte`
- Modify: `app/src/lib/governo/governo.ts`, `governo.test.ts`, `Figurina.svelte`, `CartaMinistro.svelte`, `Governo.svelte`

**Interfaces:**
- Consumes: Task 4; `guscio.costoDirotta` (Task 2); `ultimoVince` (`app/src/lib/editor/editor.svelte.ts:54`); `Mappa`, `SelettoreRegione` (`app/src/lib/mappa/`); `Conferma`; `partita.comanda`, `testoRifiuto`; `salti`, `chiaveDi`; `fattori`.
- Produces:

```ts
/** «Ferma −18», «Ferma +3», «Ferma» a zero (lettura 2). */
export function etichettaFerma(costo: number): string;
/** Gli atti di Dirotta: gli altri tre del dicastero, con costo, turni, effetti e prerequisito. */
export function attiDirotta(m: VoceMinistero): VoceAtto[];
/** La regione bersaglio si sceglie solo per Sviluppo economico e Infrastrutture (spec:387). */
export const conRegione = (m: VoceMinistero): boolean => m.dicastero === "sviluppo_economico" || m.dicastero === "infrastrutture_trasporti";
/** I costi di Dirotta, uno per atto o per regione, in sequenza; un rifiuto resta come testo. */
export function costiDirotta(chiedi: (atto: string | null, regione: string | null) => Promise<number>, atti: string[], regioni: string[]): Promise<{ atti: Record<string, number | string>; regioni: Record<string, number | string> }>;
/** Il prezzo delle dimissioni chieste, in parole: gratis col tecnico, niente col tuo partito, N punti d'intesa all'alleato. */
export function prezzoDimissioni(v: Vista, m: VoceMinistero): string;
```

- **Ferma** (storia 99): `{ tipo: "ferma", dicastero }`, diretto, con `etichettaFerma(agenda.costo_ferma)` sul bottone.
- **Dirotta**: il bottone apre a lato `Dirotta.svelte`: i quattro atti del dicastero (l'atto in agenda segnato) con costo, turni, effetti e il costo in soddisfazione; un atto non pagabile mostra `prerequisito_mancante`. Con `conRegione`, sotto la mappa piccola in cinque classi di soddisfazione persa e la lista delle venti regioni col costo (`SelettoreRegione`). Atto e regione si scelgono insieme; «Dirotta · −N» manda `{ tipo: "dirotta", dicastero, atto, regione }`. I costi partono all'apertura con `costiDirotta`; una nuova apertura annulla la vecchia (punto da sorvegliare 2).
- **La carta gira** dopo Ferma e Dirotta riusciti: sul retro il bollo «Fermato» o «Dirottato», l'atto, il voto nuovo e «rigira». Web Animations con la molla del prototipo 18; durata per `fattori`; nel tono grave e col movimento spento il retro compare senza rotazione. Lo stato del retro si legge da `agenda.stato`: al rientro nella pagina la carta è già girata.
- **Cantieri** (storia 101), nella carta del ministro: atto, regione, turni mancanti, rata, speso; «Ferma il cantiere» con `Conferma`, che dice gli euro già spesi persi (`speso`) e la soddisfazione (`costo_ferma`).
- **Dimissioni**: «Chiedi le dimissioni» con `Conferma`, che dice `prezzoDimissioni`. Con `dimissioni_chieste` la carta dice che il ministro esce a fine turno e i comandi sul dicastero si spengono.
- **Rosa** (storia 102): in interim il dettaglio mostra i tre candidati con faccia (salti della rosa), capacità, posizione e partito; il default scritto («senza scelta entra il primo, il più capace»); il clic manda `{ tipo: "scegli_successore", dicastero, indice }`, diretto; il candidato scelto (`VoceMinistero.scelta`) è a fondo giallo.

- [ ] **Step 1: scrivi i test che falliscono**, in `governo.test.ts`:
  - `etichettaFerma(-18.4)` «Ferma −18», `etichettaFerma(2.6)` «Ferma +3», `etichettaFerma(0.2)` «Ferma»;
  - `attiDirotta` sulla Sanità della 03: tre atti, l'atto in agenda escluso; `conRegione`: vero su Sviluppo economico e Infrastrutture, falso sugli altri otto;
  - `costiDirotta` con un `chiedi` finto che rifiuta un atto: quel costo è il testo del rifiuto, gli altri numeri; le chiamate sono in sequenza (la seconda parte dopo la prima risposta);
  - `prezzoDimissioni`: 10 punti d'intesa alla Destra su Economia della 03, «niente» su Interno (il tuo partito), «gratis» su Giustizia di `18-governo.json["tecnico"].vista`;
  - la rosa di `["interim"].vista`: tre candidati, e i salti danno tre coppie (naso, colletto) distinte.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `18-governo.prototype.html`:
  - `?vista=03-governo-ordinario.json`: «Ferma» su Lavoro gira la carta (il finto risponde con la Vista di `scelte`); due clic veloci fanno una chiamata sola (punto da sorvegliare 1); Dirotta su Sviluppo economico apre atti, mappa e lista con i costi; «Chiedi le dimissioni» e «Ferma il cantiere» si confermano sul posto da tastiera, e l'Esc annulla;
  - `?vista=18-governo.json&stato=interim`: la rosa con le facce, il default scritto, la scelta;
  - col movimento spento e in `&stato=grave` la carta non ruota.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Ferma, Dirotta, cantieri, dimissioni e rosa dei successori`

---

### Task 6: verifica della PR 1

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 5. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:73`: la prova di riconoscimento si fa sul campione: con le fantasie le facce cambiano solo nel colletto, quindi la prova del 15/11 vale ancora; lo si scrive nella PR.

- [ ] **Step 1: batterie nei due motori** (`npm --prefix <wt>/app run verifica:chrome` e `verifica:webkit`, `app/verifica/README.md`): `/partita/governo` su 03, 05 e gli stati di `18-governo.json` (`scelte`, `interim`, `rodaggio`, `scontenti`, `grave`, `tecnico`, `successore_scelto`); di nuovo le pagine della 11b con le facce (Partiti con otto e con dieci partiti, Trattativa nei cinque stati): il colletto è cambiato. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le carte, Ferma, Dirotta con atto e regione, le due conferme, la rosa.
- [ ] **Step 3: test e tipi.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`, `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`. Expected: verdi, pulito.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11c/`: Governo nei suoi stati, alle due finestre; la carta girata; Dirotta con la regione; il tono grave; il campione delle caricature con le fantasie.
- [ ] **Step 5: corpo della PR**: cosa cambia, decisioni prese in esecuzione, «Per Mario», testi nuovi, verifiche, rimandi al core trovati.

**Fine della PR 1.**

---

### Task 7: Aula, calendario, In aula e dettaglio della misura

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 2. Modifica `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:55`: i campi dell'Aula (`calendario`, `esiti`, `referendum`, `disposizioni`, `posto_riservato`) e i comandi. I posti oggi stanno in `Vista.posti`.
- `docs/ricerca/estetica/tabellone-e-carta-ritagliata.md:113`: perdere il governo non è la fine: la stessa Aula all'opposizione.

**Files:**
- Create: `app/src/lib/aula/aula.ts`, `aula.test.ts`, `Aula.svelte`, `Calendario.svelte`, `InAula.svelte`, `Misura.svelte`; `app/src/lib/componenti/Emiciclo.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`, `nomeDi`, `coloreDi`; `Conferma`, `Caselle`, `Dettaglio`.
- Produces:

```ts
export type Posto =
  | { tipo: "bilancio"; misura: VoceMisura }
  | { tipo: "decreti" | "ordinaria" | "costituzionale" | "opposizione"; misura: VoceMisura | null; numero?: number; di?: number };
/** I posti del calendario: decreti in precedenza, poi le ordinarie da Vista.posti, la costituzionale, l'opposizione;
 *  nel turno del bilancio un posto solo. Le misure riempiono i posti nell'ordine di Vista.calendario. */
export function calendario(v: Vista): Posto[];
/** Si vota alla chiusura di questo turno. */
export const votaOra = (v: Vista, m: VoceMisura): boolean => m.voto.anno === v.data.anno && m.voto.trimestre === v.data.trimestre;
/** La fiducia si pone solo sulle misure del governo, fuori dal bilancio e fuori dalla crisi. */
export function fiduciaPonibile(v: Vista, m: VoceMisura): boolean;
/** La linea del giocatore si sceglie all'opposizione su ogni misura, al governo sulle proposte dell'opposizione. */
export function lineaScelta(v: Vista, m: VoceMisura): boolean;
/** Gli ultimi voti, dal più recente. */
export function ultimiVoti(v: Vista): EsitoVoto[];
```

- `Emiciclo.svelte`: props `{ totale: number; si: [number, number]; maggioranza: number; ramo: "Camera" | "Senato" }`. Seggi pieni per i sì sicuri (`si[0]`), a retino fino a `si[1]`, vuoti gli altri; la riga rossa sulla maggioranza; sotto «Camera · 179–193 sì · ne servono 201». Il nome accessibile dice la stessa frase.
- `Calendario.svelte` (storia 104): la fila dei posti in testa; ogni posto dice la misura e quando si vota, un posto libero lo dice («libero · scrivi una legge», all'opposizione «libero: porta una proposta» bordato di rosso quando `posto_riservato`).
- `Aula.svelte`: sotto il calendario le quattro schede (In aula col numero, Disposizioni, Legge elettorale, Referendum); in questo task le ultime tre mostrano un segnaposto interno che i Task 8 e 9 sostituiscono.
- `InAula.svelte` (storia 105): una riga per misura col segno (a retino se `in_bilico`), nome, forma (con la deliberazione per una costituzionale), chi la porta, quando si vota; le linee dei partiti in caselle da sinistra a destra, la barra della forbice alla Camera con la riga della maggioranza, «in bilico», gli emendamenti aperti, «fiducia». Sotto, «Ultimi voti» col conteggio esatto e il motivo della decadenza.
- `Misura.svelte`, nel dettaglio a lato (storia 106): i due emicicli; la linea di ogni partito; la tabella dei seggi per un'elettorale (`VoceMisura.seggi`); la regola delle due deliberazioni per una costituzionale; gli emendamenti con Accogli · Respingi (`{ tipo: "emendamento", misura, partito, accogli }`, diretti) e il default «Senza scelta: resta aperto, si vota il testo com'è»; «Poni la fiducia» con `Conferma`, che nomina gli alleati di `alleati_contro_fiducia` (storia 111); la linea del giocatore (Sì · No · «Come da posizione», il default) col costo di `costo_linea` (`{ tipo: "linea", misura, si }`); «Anteprima della misura ›», che il Task 10 collega.

- [ ] **Step 1: scrivi i test che falliscono**, in `aula.test.ts`:
  - `calendario` su `19-aula.json["piena"].vista`: tre ordinarie (due occupate, dalla legge elettorale e dal salario minimo), la costituzionale occupata, l'opposizione con lo sgravio del Centro; su `04-governo-bilancio.json`: un posto solo, `bilancio`; su `["opposizione_libero"].vista`: il posto dell'opposizione libero; i conti vengono da `Vista.posti` (punto da sorvegliare 5);
  - `votaOra`: vero sulla costituzionale di `piena` (deliberazione 1, voto 2027T2), falso sulle altre;
  - `fiduciaPonibile`: falso sullo sgravio del Centro, vero sul salario minimo, falso su ogni misura di `05-crisi.json`;
  - `lineaScelta`: vero sullo sgravio del Centro al governo; su `["opposizione_proposta"].vista` vero su ogni misura;
  - `ultimiVoti` su `["dopo_voto"].vista`: il primo è la costituzionale approvata, Camera 219 sì e 181 no;
  - `ossatura.test.ts`: `fettaDi("aula", "governo")` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `19-aula.prototype.html`, su `/partita/aula?vista=19-aula.json&stato=` `piena`, `dopo_voto`, `opposizione_proposta`, e su `?vista=04-governo-bilancio.json`, `?vista=08-tono-grave.json`: i posti, le righe, l'emiciclo nel dettaglio; un emendamento accolto; «Poni la fiducia» con la conferma sul posto da tastiera. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Aula, calendario, misure in esame ed emiciclo`

---

### Task 8: Aula, disposizioni e bozza

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 7, in parallelo col Task 10: questo lavora in `aula/` (`bozza.ts`, `Disposizioni.svelte`, `Bozza.svelte`, `Aula.svelte`), il 10 in `anteprima/` e `Misura.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:59`: l'anteprima di una legge prima della presentazione e i suoi campi.

**Files:**
- Create: `app/src/lib/aula/bozza.ts`, `bozza.test.ts`, `Disposizioni.svelte`, `Bozza.svelte`
- Modify: `app/src/lib/aula/Aula.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 7; `guscio.anteprima`, `guscio.anteprimaProposta` (Task 2); `ultimoVince`; `partita.comanda`, `testoRifiuto`.
- Produces:

```ts
export type Bozza = ValoreProposto[];
/** Mette o toglie un gradino: il gradino in vigore toglie la disposizione dalla bozza. */
export function cambia(v: Vista, b: Bozza, disposizione: string, gradino: string): Bozza;
/** La forma dalla norma delle disposizioni (lettura 3); null a bozza vuota. */
export function forma(v: Vista, b: Bozza): "riforma" | "elettorale" | "costituzionale" | null;
/** Il bottone: «Presenta la legge di riforma», «Presenta la legge elettorale», «Proponi la norma costituzionale»,
 *  all'opposizione «Porta nel posto riservato». */
export function etichettaBozza(v: Vista, b: Bozza): Chiave | null;
export function comandoBozza(v: Vista, b: Bozza): Comando;   // presenta_legge al governo, presenta_proposta all'opposizione
/** Lo stato di un gradino nella riga aperta. */
export function statoGradino(d: VoceDisposizione, b: Bozza, gradino: VoceGradino): "in_vigore" | "in_bozza" | "in_arrivo" | "bloccato" | "libero";
/** Le disposizioni ordinarie e costituzionali per dicastero, nell'ordine di Vista.dicasteri; la Costituzione in fondo. */
export function perDicastero(v: Vista): { dicastero: string | null; nome: string; voci: VoceDisposizione[] }[];
```

- `Disposizioni.svelte` (storia 107): le righe chiuse per dicastero, con nome, legge, valore in vigore, precedente se diverso, in arrivo con la data, «in esame» se una misura in calendario la tocca, e «Cambia ›» (all'opposizione «Proponi ›»). I gradini si aprono solo sulla riga scelta o con un valore già in bozza: in vigore a fondo pieno, in bozza in giallo, in arrivo a tratteggio blu, bloccato a retino.
- `Bozza.svelte`, nel dettaglio a lato (storia 108): i valori scelti con «ora …» accanto; l'anteprima (`guscio.anteprima` al governo, `guscio.anteprimaProposta` all'opposizione), una in volo per volta, l'ultima vince (punto da sorvegliare 2), con costo annuo, effetti, linee e forbice; il bottone di `etichettaBozza`; il rifiuto del core sul posto (calendario pieno compreso), col rimando alla misura in esame quando il testo la nomina. La scheda Disposizioni sostituisce il suo segnaposto in `Aula.svelte`.

- [ ] **Step 1: scrivi i test che falliscono**, in `bozza.test.ts`:
  - `cambia`: aggiungere salario minimo a 11 € e poi il gradino in vigore lascia la bozza vuota; due gradini della stessa disposizione tengono l'ultimo;
  - `forma`: riforma col salario minimo; costituzionale col numero dei parlamentari; elettorale con la soglia di sbarramento; `null` a bozza vuota;
  - `etichettaBozza` al governo sulla 03 e all'opposizione su `19-aula.json["opposizione_libero"].vista`;
  - `comandoBozza`: `presenta_legge` sulla 03, `presenta_proposta` all'opposizione, con gli stessi valori;
  - `statoGradino`: su `["dopo_elettorale"].vista` il gradino «no» di `coalizioni` (legge «L. 1/2027») è `in_vigore`; un gradino in `in_arrivo` è `in_arrivo`; un gradino messo in bozza è `in_bozza`;
  - `perDicastero` sulla 03: le disposizioni elettorali escluse, la Costituzione ultima.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser** su `/partita/aula?vista=03-governo-ordinario.json`: «Cambia ›» apre i gradini; una bozza sul salario minimo mostra l'anteprima del campione e «Presenta la legge di riforma»; la costituzionale su `&stato=piena` di `19-aula.json` mostra «calendario pieno» sul posto; cambiare gradino in fretta lascia l'ultima anteprima. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Aula, disposizioni e bozza della legge`

---

### Task 9: Aula, legge elettorale e referendum

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 8 e il Task 10: usa `Bozza.svelte` e vi collega il cartellino.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:55`: referendum e legge elettorale fra i dati dell'Aula.

**Files:**
- Create: `app/src/lib/aula/elettorale.ts`, `elettorale.test.ts`, `Elettorale.svelte`, `Referendum.svelte`
- Modify: `app/src/lib/aula/Aula.svelte`, `Bozza.svelte`, `Disposizioni.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 8; `Cartellino` (Task 10); `guscio.menuElettorale`, `guscio.catalogoScenario`; `ultimoVince`; `Conferma`.
- Produces:

```ts
/** Le leggi pronte del catalogo (CatalogoScenario.leggi_elettorali) come bozza: solo ciò che cambia rispetto al vigore. */
export function bozzaDiLegge(v: Vista, legge: LeggeElettoraleCatalogo): ValoreProposto[];
/** I gradini spenti dal menu, col motivo: chiave `disposizione|gradino`. */
export function spenti(menu: VoceMenu[]): Record<string, string>;
```

Il nome del tipo della legge del catalogo si legge in `app/src/lib/tipi/CatalogoScenario.ts`.

- `Elettorale.svelte` (storia 109): le nove disposizioni elettorali coi gradini tutti aperti; in testa le impostazioni pronte del catalogo, che riempiono la bozza; a ogni bozza `menu_elettorale`, una richiesta in volo per volta: un gradino che il core non ammette è spento col motivo. La soglia del premio parte dal 40%: i dati non hanno gradini sotto.
- `Referendum.svelte` (storia 110): i referendum in attesa (`Vista.referendum`: tipo, promotore, firme o indetto, quando) e gli esiti (`esiti_referendum`). Al governo «Invita all'astensione» sull'abrogativo (`{ tipo: "invita_astensione" }`, diretto) e la scelta sul confermativo (`{ tipo: "confermativo", misura, chiedi }`, diretto). All'opposizione «Promuovi un referendum abrogativo» sta nel dettaglio di una disposizione con `rifiuto_referendum` nullo, con `Conferma`; con `rifiuto_referendum` presente la riga mostra il testo senza bottone.
- In `Bozza.svelte`, «Anteprima della misura ›» apre il `Cartellino` al posto della bozza, con «‹ Torna alla bozza»; cambiare un gradino con l'anteprima aperta la fa oscillare.

- [ ] **Step 1: scrivi i test che falliscono**, in `elettorale.test.ts`:
  - `bozzaDiLegge` della legge in vigore sulla 03 è vuota; quella della «proporzionale al 3%» ha i sette `valori` della misura 1 di `19-aula.json["piena"].vista.calendario`;
  - `spenti` su `19-aula.json["menu_03"]`: ogni voce con `ammesso` falso ha il suo motivo, e nessuna voce ammessa compare.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**: la scheda elettorale sulla 03 con le impostazioni pronte e i gradini spenti; il referendum all'opposizione su `28-opposizione.json&stato=referendum` con la conferma; l'anteprima che si apre dalla bozza e torna indietro. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Aula, legge elettorale e referendum`

---

### Task 10: Anteprima della misura, il cartellino

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 7, in parallelo con l'8: lavora in `anteprima/` e collega il cartellino a `Misura.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/grafica-italiana-giocosa.md:211`: il mobile che oscilla con una molla smorzata, circa 1,5 Hz, fermo in circa 1,2 s. Vale il piano: oscilla solo a un gesto, mai a ogni modifica in loop.
- `docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:27`: le Web Animations nella WKWebView.
- `docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:65`: una regola globale sul movimento ridotto non tocca le animazioni: il controllo passa da `fattori`.

**Files:**
- Create: `app/src/lib/anteprima/anteprima.ts`, `anteprima.test.ts`, `oscilla.ts`, `Cartellino.svelte`
- Modify: `app/src/lib/aula/Misura.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `Emiciclo`, `fiduciaPonibile` (Task 7); `Mappa` (`app/src/lib/mappa/`); `partita.comanda`, `testoRifiuto`; `fattori`.
- Produces:

```ts
/** Ciò che il cartellino mostra: un'anteprima del core o una misura già in calendario (oggi porta costo, effetti e gruppi). */
export type Misurabile = Anteprima | VoceMisura;
/** Una riga per disposizione: il gradino in vigore e quello della misura. */
export function cosaCambia(v: Vista, valori: ValoreProposto[]): { disposizione: string; ora: string; con: string }[];
/** «subito», «dopo 4 turni», «dopo 40 turni · 10 anni». */
export function quando(ritardo: number): string;
/** Chi stima: il ministro del dicastero proponente con la sua capacità; null = stima neutra (storia 114). */
export function stimatore(v: Vista, proponente: string | null): { nome: string; dicastero: string; capacita: number } | null;
/** Le regioni dove valgono le disposizioni della misura; vuoto = tutto il Paese. */
export function perimetro(v: Vista, valori: ValoreProposto[]): string[];
// oscilla.ts
/** Un'oscillazione smorzata attorno allo spillo; null nel tono grave o col movimento spento. */
export function oscilla(el: HTMLElement, k: number): Animation | null;
```

- `Cartellino.svelte` (storie 113-117): props `{ misura: Misurabile; valori: ValoreProposto[]; forma: FormaMisura; indietro: string; ontorna }`. Un foglio appeso allo spillo; dentro, in ordine: forma e titolo; la tabella cosa cambia · ora · con la misura, poi la spesa all'anno («saldo zero» a costo nullo); gli effetti con quando e quanto (tre caselle, migliora o peggiora); chi stima e la capacità, o «stima neutra»; i gruppi toccati e «Compra il test · N €» con la cassa accanto (`{ tipo: "compra", acquisto: { test_misura: { valori } } }`, diretto); comprato, la tabella delle sei condizioni da `acquisti.test_misura` con i gruppi toccati segnati; il perimetro su una mappa piccola con l'elenco accanto; il voto con i due emicicli e le linee; i seggi per un'elettorale; una riga sulla forma (deliberazioni, conversione, «accolta, entra in calendario come legge»). Il prezzo del test si legge in `acquisti.prezzi`.
- Oscilla all'apertura, al cambio di gradino con l'anteprima aperta e all'acquisto del test; mai in loop.
- In `Misura.svelte`, «Anteprima della misura ›» apre il cartellino dalla `VoceMisura` in calendario, con «‹ Torna alla misura».

- [ ] **Step 1: scrivi i test che falliscono**, in `anteprima.test.ts`:
  - `quando(0)` «subito», `quando(4)` «dopo 4 turni», `quando(40)` «dopo 40 turni · 10 anni»;
  - `cosaCambia` sulla 03 per il salario minimo a 11 €: una riga col gradino in vigore e «11 €/h»;
  - `stimatore(03, "lavoro_welfare")` è Simone Ricci con la sua capacità; `stimatore(03, null)` è `null`;
  - `perimetro` della ZES: le regioni della disposizione; del salario minimo: vuoto;
  - la `VoceMisura` del salario minimo di `19-aula.json["piena"].vista` è un `Misurabile` con costo ed effetti.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `20-anteprima.prototype.html`: il cartellino di una misura in calendario dal dettaglio dell'Aula; il test comprato e il secondo acquisto rifiutato sul posto («già comprato questo turno»); col movimento spento e nel tono grave il foglio è fermo; alla velocità 2× l'oscillazione dura la metà. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): anteprima della misura, il cartellino`

---

### Task 11: verifica della PR 2

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 9. Chiude la PR 2.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori**: `/partita/aula` sugli stati di `19-aula.json`, su 03, 04, 05, 08 e su `17-cruscotto.json&stato=ultimatum`; il cartellino aperto sulle quattro forme. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le schede, «Cambia ›» e i gradini, la bozza, gli emendamenti, la fiducia con la conferma, la linea, il referendum, il cartellino e «‹ Torna».
- [ ] **Step 3: test e tipi**, come al Task 6, passo 3.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11c/`: l'Aula nelle quattro schede, al governo e all'opposizione, alle due finestre; il cartellino delle quattro forme; il tono grave.
- [ ] **Step 5: corpo della PR**, come al Task 6, passo 5.

**Fine della PR 2.**

---

### Task 12: Cruscotto al governo, l'ordine del giorno

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 3. Modifica `ossatura.ts`, `ossatura.test.ts`, `Cornice.svelte` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:47`: ciò che costa al giocatore ha peso visivo: l'evento e la crisi stanno in testa.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza si legge prima della scelta: il default su ogni riga.

**Files:**
- Create: `app/src/lib/cruscotto/Voce.svelte`
- Modify: `app/src/lib/cruscotto/cruscotto.ts`, `cruscotto.test.ts`, `Cruscotto.svelte`, `Evento.svelte`; `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/lib/cornice/Cornice.svelte`; `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`; `Evento`, `Riga` (11b); `Faccia`, `salti`, `chiaveDi` (Task 3); `fiduciaPonibile`, `votaOra` (Task 7); `Conferma`.
- Produces:

```ts
export type Gruppo = "da_decidere" | "proposte" | "emendamenti" | "si_vota" | "occhio";
export type VoceOdg = {
  chiave: string; gruppo: Gruppo; titolo: string; sotto: string;
  default: string;            // il default scritto sulla riga
  alDefault: boolean;         // conta fra le scelte al default (lettura 1)
  urgente: boolean; fatta: boolean;
  azioni: "proposta" | "emendamento" | "successore" | "fiducia" | { rimando: Sezione };
  dicastero?: string; misura?: number; partito?: string;
};
/** Le voci per gruppo, nell'ordine dei gruppi e, dentro, della Vista. */
export function ordineDelGiorno(v: Vista): VoceOdg[];
/** Gli eventi senza scelta più le voci al default dell'ordine del giorno (lettura 1). Sostituisce la versione della 11b. */
export function scelteAlDefault(v: Vista): number;
/** Il fondo di riserva rimasto basta a coprire il costo dell'evento. */
export const fondoBasta = (v: Vista, e: VoceEvento): boolean => v.bilancio.fondo_residuo >= e.costo;
```

- `fettaDi("cruscotto", "governo" | "crisi")` dà `null`: la pagina monta `Cruscotto` anche al governo.
- `Cruscotto.svelte`, al governo (storia 92): gli eventi in testa; sotto, i gruppi «Da decidere» (bilancio da presentare col rimando «Bilancio ›», successori), «Proposte dei ministri», «Emendamenti degli alleati», «Si vota a fine turno», «Da tenere d'occhio» (gli ultimatum, rimando «Partiti ›»), ciascuno col conto al default. Il turno zero resta com'è.
- `Voce.svelte` (storia 93): segno (vuoto, pieno se fatta, a retino se urgente), titolo, sotto il ministro o il fatto e il default, a destra le azioni: Accogli · Rinvia · Respingi (Rinvia spento con «già rinviata una volta» se `rinviata`), Accogli · Respingi, i nomi della rosa, «Poni la fiducia» con `Conferma` (storia 111), o il rimando. Il rifiuto del core sotto la riga.
- `Evento.svelte` (storia 94): al governo accanto al costo il fondo di riserva rimasto, in rosso se `fondoBasta` è falso; la UI non calcola la quota coperta. Con `norme` non vuote, la riga elenca le norme del decreto (lettura 4): il dettaglio arriva col Task 13.
- `Cornice.svelte`: il conto usa la nuova `scelteAlDefault`.

- [ ] **Step 1: scrivi i test che falliscono**, in `cruscotto.test.ts`:
  - `scelteAlDefault`: 4 sulla 03 (l'evento e le tre proposte); 2 su `17-cruscotto.json["scelte_fatte"].vista` (evento scelto, Ambiente rinviata); 1 su `["ultimatum"].vista` (la proposta di Economia); 1 su `["esercizio_provvisorio"].vista` (il bilancio); 7 su `05-crisi.json` (la crisi e sei interim; lo scandalo a opzione unica non conta); 6 su `["crisi_provvisorio"].vista` (cinque interim e l'evento; la crisi in esercizio provvisorio non conta); 0 su `01-turno-zero.json`, che porta sei proposte del governo uscente;
  - `ordineDelGiorno` sulla 03: tre voci in «Proposte dei ministri», nell'ordine dei dicasteri, ciascuna col suo default; su `19-aula.json["dopo_voto"].vista`: due voci in «Emendamenti degli alleati» col default «resta aperto, si vota il testo com'è»; su `["ultimatum"].vista`: la Sinistra in «Da tenere d'occhio»;
  - `fondoBasta`: falso sulla 03 (fondo 0, evento da un miliardo);
  - `ossatura.test.ts`: `fettaDi("cruscotto", "governo")` e `("cruscotto", "crisi")` sono `null`; all'opposizione resta «11e».
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `17-cruscotto.prototype.html`, su `/partita/cruscotto?vista=03-governo-ordinario.json` e su `17-cruscotto.json` negli stati `scelte_fatte`, `ultimatum`, `esercizio_provvisorio`: i gruppi coi loro conti; Rinvia sulla proposta di Ambiente porta alla Vista di `scelte_fatte` e il conto in cornice scende da 4 a 2 (punto da sorvegliare 3); il fondo in rosso. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): cruscotto al governo, l'ordine del giorno`

---

### Task 13: Cruscotto, crisi, dimissioni, ministri in evidenza e dettaglio a lato

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 12: stessi `cruscotto.ts` e `Cruscotto.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza si legge prima della conferma: «Dimettiti» e l'allargamento dicono cosa succede.
- `docs/ricerca/estetica/tabellone-e-carta-ritagliata.md:113`: perdere il governo porta all'opposizione, non alla fine: la crisi lo dice.

**Files:**
- Create: `app/src/lib/cruscotto/Crisi.svelte`, `DettaglioVoce.svelte`
- Modify: `app/src/lib/cruscotto/cruscotto.ts`, `cruscotto.test.ts`, `Cruscotto.svelte`, `Evento.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 12; `Cartellino` (Task 10); `guscio.anteprimaAccogli`, `guscio.anteprimaDecreto`; `fuori`, `allarga` (`app/src/lib/trattativa/trattativa.ts:65-71`); `figurine` (Task 4); `Conferma`; `ultimoVince`.
- Produces:

```ts
/** I ministri in evidenza, con l'atto e la soddisfazione in rosso se in preavviso. */
export function inEvidenza(v: Vista): { dicastero: string; ministro: VoceMinistro; agenda: VoceAgenda; preavviso: boolean }[];
/** Il riquadro di testa: crisi da allargare, crisi in esercizio provvisorio (caduta certa), dimissioni annunciate, o niente. */
export function testa(v: Vista): "crisi" | "crisi_provvisorio" | "dimissioni" | null;
```

- `Crisi.svelte` (storia 97): prima dell'evento. In crisi: «Allarga la maggioranza o dimettiti»; i partiti di `disponibilita` disponibili si spuntano, gli altri restano a tratteggio con la causa; «Allarga a X e Y» compare con almeno uno spuntato e passa da `Conferma` (lettura 5); «Dimettiti» con `Conferma` («Conferma: dimettiti» · «Annulla»). In esercizio provvisorio la caduta è certa e il riquadro lo dice senza bottoni. Con `dimissioni` vero, il riquadro dice che il governo cade alla chiusura. La banda nera la mette già la cornice.
- In fondo al cruscotto i ministri in evidenza col rimando «Governo ›» (storia 96).
- `DettaglioVoce.svelte`, nel dettaglio a lato della voce scelta: per una proposta di fondi il ministro, il motivo e il `Cartellino` da `anteprima_accogli`, con le tre azioni (storia 95); per una riforma, che accolta entra in calendario come legge, col rimando «Anteprima della misura ›» dall'`anteprima` dei suoi valori; per un voto i due emicicli e «in bilico»; per un decreto d'evento i gradini della norma scelta, il cartellino da `anteprima_decreto` e «Emana il decreto» (`{ tipo: "emana_decreto", decreto: { tipo: "disposizioni", valori } }`, diretto); per il resto motivo e causa.

- [ ] **Step 1: scrivi i test che falliscono**, in `cruscotto.test.ts`:
  - `testa`: `crisi` sulla 05, `crisi_provvisorio` su `17-cruscotto.json["crisi_provvisorio"].vista`, `dimissioni` su `["dimissioni"].vista`, `null` sulla 03;
  - `inEvidenza` sulla 03: Economia, Sviluppo economico e Ambiente, nessuno in preavviso; su `18-governo.json["scontenti"].vista`: i ministri con `minaccia` sono in preavviso;
  - i disponibili della 05: Sinistra e Centrodestra, gli altri con la causa del core.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `17-cruscotto.prototype.html`:
  - `?vista=05-crisi.json`: il riquadro della crisi, la spunta, «Allarga» e «Dimettiti» con la conferma da tastiera; due Invio veloci su «Dimettiti» fanno una chiamata sola (punto da sorvegliare 1);
  - `?vista=17-cruscotto.json&stato=crisi_provvisorio`: il riquadro senza bottoni, i successori;
  - `?vista=03-governo-ordinario.json`: la proposta di Infrastrutture col cartellino di `anteprima_accogli`; il decreto d'evento sul codice degli appalti col cartellino e «Emana il decreto»; i ministri in evidenza in fondo;
  - `?vista=08-tono-grave.json`: il cruscotto con le regole del tono grave.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): crisi, dimissioni, ministri in evidenza e dettaglio del cruscotto`

---

### Task 14: verifica della fetta

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 13. Chiude la PR 3 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 6).

**Ricerca:**
- nessun riferimento pertinente

Lo esegue la sessione principale. Ogni esito va nella PR 3.

- [ ] **Step 1: batterie nei due motori** sul cruscotto in tutti gli stati del Task 12 e del Task 13 e, di nuovo, su Governo e Aula: ora il conto in cornice è quello nuovo. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le azioni sulla riga, la rosa, la fiducia, «Dimettiti», l'allargamento in crisi, il decreto d'evento.
- [ ] **Step 3: test e tipi**, come al Task 6, passo 3.
- [ ] **Step 4: giro nell'app vera**, col telecomando (`app/verifica/README.md:63-98`). `npm --prefix <wt>/app run verifica:giro` va lanciato per primo e in background: la prima compilazione dura una dozzina di minuti. Il percorso, dall'edicola:
  1. **una partita che esce dal voto senza maggioranza**, per provare l'allargamento della trattativa, mai provato nell'app vera: nell'editor lo scenario con la legge «Proporzionale puro», fondazione a 5,5, nessuna coalizione, «Chiudi il turno». Se l'incarico arriva con la maggioranza, o non arriva, si abbandona e si rifonda: al più tre tentativi, ciascuno scritto nella PR. Con l'incarico senza maggioranza: «Allarga ›» con la conferma, la spartizione, «Chiudi il turno»;
  2. se nei tre tentativi l'incarico senza maggioranza non arriva, si continua con una partita dell'Italia di oggi a 4,5 con la destra, e la PR lo dice: l'allargamento resta provato solo in Chrome headless;
  3. al governo, il cruscotto: rispondere all'evento, accogliere o respingere una proposta, il conto in cornice che scende;
  4. Governo: Ferma su un ministro, la carta che gira; Dirotta con atto (e la regione, su Sviluppo economico o Infrastrutture); una dimissione chiesta con la conferma;
  5. Aula: una legge di riforma dalla bozza con l'anteprima e il cartellino; il test della misura comprato, il secondo rifiutato; un emendamento deciso al turno dopo; la fiducia posta;
  6. «Chiudi il turno» fino a una crisi o a un interim, se arriva entro dieci turni: la rosa, il successore scelto; in crisi «Allarga» o «Dimettiti».

  Poi i tagli nell'app vera su cruscotto, governo e aula (`?tagli=1`), a scala 1 e 1,25. A fine giro si rimettono le impostazioni, si cancella la cartella di verifica e si ripristina `src-tauri/Cargo.toml` (`README.md:94-98`).
- [ ] **Step 5: screenshot** del cruscotto nei suoi stati in `docs/superpowers/verifiche/11c/`.
- [ ] **Step 6: corpo della PR 3.** Oltre alle sezioni solite: l'esito del giro con l'allargamento (e i tentativi); i rimandi al core trovati dalla fetta; ciò che resta da provare a mano a Mario (il Tab reale, la carta che gira e il cartellino che oscilla nell'app, Cmd+Q con una bozza aperta).
- [ ] **Step 7: commit dei ritocchi della verifica**, se ce ne sono. `fix(app): ritocchi dalla verifica della 11c`

**Fine della PR 3 e della fetta.**
