# Piano 11b «Al voto»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** dal turno zero il giocatore dichiara la coalizione, fa la campagna, legge il Paese e i partiti, vota e compone il governo in trattativa. La fetta porta cinque schermate (Cruscotto del turno zero, Campagna, Paese, Partiti, Trattativa) e il kit delle caricature.

**Architettura:**
- Ogni schermata è una cartella in `app/src/lib/<sezione>/`: un modulo `.ts` puro, che legge la Vista e decide cosa mostrare, e i componenti Svelte che lo disegnano. Il modulo puro è provato con Vitest sulle Viste campione.
- Le scelte passano da `partita.comanda`, che la fetta aggiunge allo stato condiviso. Il rifiuto del core torna al chiamante e compare accanto alla scelta.
- Il guscio guadagna due passacarte, `anteprima_campagna` ed `elettori_campione`. Il core guadagna solo la riga che esporta `Fascia` in TypeScript.
- Il guscio finto impara a rispondere dai campioni 23, 26 e 27, così ogni schermata si prova nel browser.
- Dove la Vista tace, il posto resta vuoto e nomina la 11g.

**Tecnologie:** quelle della 11a (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2, Rust). Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 60-91 (righe 97-133), i vincoli alle righe 248-371, la sezione della fetta alle righe 373-381, le conferme alla riga 453 e i test alle righe 455-476. Chi esegue legge la spec e il ticket citato nel task.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `17-cruscotto.prototype.html`: forma A · Ordine del giorno, stato «turno zero»;
- `26-campagna.prototype.html`: forma A · Tavolo;
- `23-paese.prototype.html`: forma A · Atlante; «dove» mostra il selettore di regione dentro le sezioni ospiti;
- `24-partiti.prototype.html`: forma A · Schieramento;
- `27-trattativa.prototype.html`: forma A · Righe;
- `32-caricature.prototype.html?f=5`: la Linea rifinita.

I testi dell'interfaccia si copiano dai prototipi. Un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR. I numeri che un prototipo segna «11g», o che rifà dai file del core, restano fuori (spec:492).

**Da dove si parte:** main a `63c14d7`, con la 11a fusa (#55, #56, #57). Le 25 Viste campione sono state rigenerate su quel commit il 13/11/2026: stesse impronte, nessun file cambiato. Il core «esito e Vista completa» lavora in parallelo sul branch `feat/esito-del-turno` e non ha fuso nulla: la 11b usa solo ciò che c'è su main.

## Vincoli globali

Valgono tutti i vincoli globali del piano della 11a (`docs/superpowers/plans/2026-11-12-piano-11a-avvio.md:27-47`). Qui quelli che la fetta tocca di più, e quelli nuovi.

- **La UI legge, il core decide** (spec:250). Lecito: contare voci aperte della Vista, sommare quote pubbliche del sondaggio e seggi di una coalizione, ordinare per campi. Ogni altra cifra viene dal core o resta un posto vuoto.
- **Dato mancante = posto vuoto che nomina la 11g** (spec:251), col componente `Segnaposto` o con una riga a tratteggio che dice «arriva con la fetta 11g».
- **`sim-core` cambia in una riga sola**: l'esportazione di `Fascia` (spec:252, :381). La Vista resta com'è.
- **Il guscio aggiunge passacarte** verso metodi già pubblici della partita, senza logica né stato (spec:253).
- **Conferma sul posto** per riposizionamento, promesse, allargamento in trattativa e «Resta fuori». **Diretti**: dichiarare la coalizione, «Chiedi la guida», la risposta a un evento, l'acquisto del sondaggio (spec:453).
- **Rifiuto del core accanto alla scelta**, col testo del core senza il prefisso `comando non ammesso: ` (lettura 7).
- **Bozze solo in memoria**: spese della campagna, promesse in composizione, spartizione in composizione (spec:268). Al rientro nella pagina vale ciò che dice la Vista.
- **Colori dei partiti** solo nelle barre e nella mappa del voto, sempre col nome accanto (spec:335). Nel tono grave sono grigi.
- **Tono grave** finché un evento grave è aperto: iniziali al posto delle facce, nessun colore d'accento, nessun movimento oltre le dissolvenze (storie 169-170).
- **Cargo da un worktree**: `--manifest-path <wt>/Cargo.toml`, `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi` e `CARGO_TARGET_DIR=<wt>/target`, tutti assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md:105`).
- **Commit** in Conventional Commits, col trailer del modello che scrive il commit.

## Decisioni di Mario del 13/11/2026

1. **Governo in trattativa resta barrato.** La Vista non dice chi guida il governo uscente (spec:285). La 11b lascia la barrata com'è e scrive il campo mancante fra i rimandi della PR 3, per il piano di core.
2. **Prova di riconoscimento delle caricature: la guarda Mario da solo**, sulla pagina del campione senza nomi (Task 15). La prova con 3-5 persone resta un debito scritto nella PR 3, da chiudere prima del rilascio della v1.
3. **Nella PR 1 entrano i tre debiti tecnici del guscio** lasciati dalla 11a (Task 1). I testi d'errore del guscio restano come sono.
4. **«N scelte vanno al default» conta solo gli eventi aperti senza risposta.** Coalizione e campagna dicono il loro stato sulle due righe del cruscotto.

## Letture che la spec non fissa

Le ho decise scrivendo il piano. Mario le conferma col sì all'esecuzione, o le cambia prima.

1. **Il cruscotto della 11b è quello del turno zero.** Al governo e in crisi la pagina mostra il riquadro dell'evento in testa, che la fetta costruisce una volta, e sotto il segnaposto della 11c. All'opposizione resta il segnaposto della 11e. Il tono grave si prova sul riquadro dell'evento della Vista 08.
2. **I nomi delle venti regioni stanno nella UI**, accanto alla geometria della mappa: sono il join di venti righe della spec:332. La macroarea è un dato del core (`dati/base/regioni.toml`) e arriva con `Vista.regioni` in 11g.
3. **La trattativa parte senza dicasteri assegnati.** La Vista porta richiesta e offerta in punti per alleato, non la spartizione di default per dicastero. Il giocatore assegna i dieci dicasteri e la proposta parte quando sono tutti assegnati; da lì ogni clic la sostituisce. Il peso a pallini viene da una tabella di tre valori nella UI (spec:379), che la 11g sostituisce col dato. Restano alla 11g: il pallino blu sulla riga cambiata, l'intesa di partenza con «entra in ultimatum» e «può uscire», i nomi nel ribaltone.
4. **Partiti senza le soglie.** La freccia fra dichiarata e percepita c'è; il rosso e i due fili di minaccia e scissione arrivano in 11g, con le soglie. La barra dell'intesa mostra valore e ultimatum; i fili a 40 e 25 arrivano in 11g.
5. **La campagna si comanda all'uscita dal campo.** `anteprima_campagna` gira a ogni modifica dell'importo; `Campagna` parte quando il giocatore lascia il campo, preme Invio o cambia leva o bersaglio, e solo se l'ultima anteprima è riuscita. Così la Vista porta sempre l'ultima campagna valida.
6. **Le facce arrivano con la PR 3.** Nella PR 2 le schede dei partiti mostrano le iniziali nel cerchio, lo stesso disegno del tono grave; la PR 3 le sostituisce con le caricature.
7. **Il rifiuto perde il prefisso `comando non ammesso: `**, come nei prototipi e come la 11a ha fatto nell'editor (#57, decisione 8).
8. **La geometria della mappa si prende dal prototipo 23** (`23-paese.prototype.html:248`, costante `GEO`), che viene già dai confini ISTAT. Nessun download.

## Punti da sorvegliare

I casi che la spec implica e che più facilmente rompono il gioco. Ognuno ha il suo test nel task indicato.

1. **Corsa delle anteprime della campagna.** Chi scrive un importo in fretta fa partire molte richieste: ne resta in volo una per volta, l'ultima vince, una risposta vecchia non copre la nuova. Task 6.
2. **Doppio clic su una mossa che si fa una volta** (riposizionamento, promesse, allargamento, «Resta fuori»): parte un comando solo, e il pulsante resta spento mentre `partita.occupato`. Task 2 e Task 7.
3. **La cassa scende sotto la bozza.** Comprato il sondaggio, le spese già comandate possono superare la cassa: l'avviso della campagna tagliata si ricalcola a ogni Vista nuova e dice quale spesa scende e quale cade. Task 6.
4. **Dieci partiti, due nati in partita.** Le quote del sondaggio si leggono per posizione in `Vista.partiti`; un partito fuori dagli otto colori va a retino grigio; un partito senza riga nel sondaggio non azzera gli altri. Task 2 e Task 11.
5. **Bersaglio regione per indice.** `Bersaglio.regione` è un indice nei dati, la mappa lavora per id: la conversione passa dall'ordine di `Vista.sondaggio.regioni`, in un punto solo. Task 3.

## Struttura dei file

```
crates/sim-core/src/elettorato/mod.rs       + esportazione TS di Fascia
app/src-tauri/src/lib.rs                    + anteprima_campagna, elettori_campione; debiti del guscio
app/src/lib/tipi/Fascia.ts                  rigenerato
app/src/lib/guscio/chiamate.ts              + anteprimaCampagna, elettoriCampione, testoRifiuto
app/src/lib/guscio/finto.ts                 + risposte dai campioni 23, 26, 27
app/src/lib/partita.svelte.ts               + comanda()
app/src/lib/partiti.ts                      quote fra i votanti, coalizione del giocatore, colori, nomi
app/src/lib/stile/fondamenta.css            + otto colori dei partiti
app/src/lib/componenti/                     + Caselle, VoceCausa, BarraSeggi, Faccia
app/src/lib/mappa/                          regioni.ts (geometria, nomi, ancore), mappa.ts, Mappa.svelte, SelettoreRegione.svelte
app/src/lib/cruscotto/                      cruscotto.ts, Cruscotto.svelte, Evento.svelte, Riga.svelte
app/src/lib/campagna/                       campagna.ts, Campagna.svelte, Coalizione.svelte, Spese.svelte, Posizione.svelte, Promesse.svelte, Cassa.svelte
app/src/lib/paese/                          paese.ts, Paese.svelte, Sondaggio.svelte, SchedaRegione.svelte
app/src/lib/partiti/                        schieramento.ts, Partiti.svelte, AsseDeiPartiti.svelte, SchedaPartito.svelte, Camere.svelte
app/src/lib/trattativa/                     trattativa.ts, Trattativa.svelte, Risultato.svelte, Alleati.svelte, Ribaltone.svelte
app/src/lib/caricature/                     caricatura.ts, kit.ts, Caricatura.svelte
app/src/lib/ossatura.ts                     fettaDi dà null per ciò che la fetta costruisce
app/src/routes/partita/[sezione]/+page.svelte   monta le sezioni costruite
app/src/routes/sviluppo/caricature/+page.svelte il campione senza nomi (solo in sviluppo)
docs/superpowers/verifiche/11b/             screenshot della fetta
```

I test Vitest stanno accanto al modulo che provano. Un file `.svelte` che supera le 250 righe si divide per riquadro.

## PR e ordine

- **PR 1, cruscotto e campagna** (`feat/11b-cruscotto-campagna`): Task 1-8.
- **PR 2, Paese e Partiti** (`feat/11b-paese-partiti`): Task 9-13.
- **PR 3, trattativa con le caricature** (`feat/11b-trattativa-caricature`): Task 14-19.

Ogni PR parte da main dopo la fusione della precedente. Dentro una PR i task vanno in sequenza, con queste eccezioni: 3 e 4 in parallelo dopo il 2; 10 e 11 in parallelo dopo il 9; 14 e 16 in parallelo. Toccano cartelle diverse. Il 9 e l'11 restano in sequenza: tutti e due modificano `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo; `rust-reviewer` nella PR 1. `craft-reviewer` riceve gli screenshot del task di verifica della PR e li confronta col prototipo e coi token.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta.

- **Dove ricerca e piano dicono cose diverse, vale il piano**: i prototipi sono stati decisi dopo le ricerche. Gli scarti noti sono scritti accanto al riferimento.
- **In un parallelo ogni subagente mette nel commit solo i file del proprio task**, nominandoli uno per uno in `git add`. Test e `check` girano su tutta l'app: un rosso che nasce nella cartella dell'altro task si riporta a chi orchestra e resta com'è.
- **Il Task 9 e il Task 11 vanno in sequenza**, e il parallelo della PR 2 passa a 10 e 11 (col 12 in coda all'11).

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `rust-engineer` | primo della PR 1 | `rust-skills` |
| 2 | `frontend-engineer` | dopo l'1 | `impeccable`, `dataviz`, `apple-design` |
| 3 | `frontend-engineer` | dopo il 2; in parallelo col 4 | `impeccable`, `dataviz`, `apple-design` |
| 4 | `frontend-engineer` | dopo il 2; in parallelo col 3 | `impeccable`, `dataviz`, `apple-design` |
| 5 | `frontend-engineer` | dopo il 3 e il 4 | `impeccable`, `dataviz`, `apple-design` |
| 6 | `frontend-engineer` | dopo il 5 | `impeccable`, `dataviz`, `apple-design` |
| 7 | `frontend-engineer` | dopo il 6 | `impeccable`, `dataviz`, `apple-design` |
| 8 | sessione principale | dopo il 7 | `scrittura-italiana` |
| 9 | `frontend-engineer` | primo della PR 2; il parallelo con l'11 è tolto | `impeccable`, `dataviz`, `apple-design` |
| 10 | `frontend-engineer` | dopo il 9; in parallelo con 11 e 12 | `impeccable`, `dataviz`, `apple-design` |
| 11 | `frontend-engineer` | dopo il 9; in parallelo col 10 | `impeccable`, `dataviz`, `apple-design` |
| 12 | `frontend-engineer` | dopo l'11; in parallelo col 10 | `impeccable`, `dataviz`, `apple-design` |
| 13 | sessione principale | dopo il 10 e il 12 | `scrittura-italiana` |
| 14 | `frontend-engineer` | primo della PR 3; in parallelo col 16 | salto dichiarato: modulo puro |
| 15 | `frontend-engineer`, senza `graphic-designer` | dopo il 14; in parallelo col 16; Step 5 alla sessione principale | `impeccable`, `dataviz`, `apple-design` |
| 16 | `frontend-engineer` | in parallelo con 14 e 15 | `impeccable`, `dataviz`, `apple-design` |
| 17 | `frontend-engineer` | dopo il 15 e il 16 | `impeccable`, `dataviz`, `apple-design` |
| 18 | `frontend-engineer` | dopo il 17 | `impeccable`, `dataviz`, `apple-design` |
| 19 | sessione principale | dopo il 18 | `scrittura-italiana` |

Ogni brief nomina anche `caveman`; quelli dei subagenti che scrivono codice (tutti, fuori dalla sessione principale) anche `ponytail`.

## Dove va ogni storia

| Storia | Task | Resta alla 11g (o 11f) |
|---|---|---|
| 60, 61 | 4 | |
| 62, 63 | 5 | |
| 64, 66, 67, 69 | 6 | |
| 65 | 3, 6 | |
| 68 | 7 | anteprime di riposizionamento e promesse, soglie di scissione (storia 165) |
| 70, 71, 72, 73, 76 | 9 | livelli disoccupazione, reddito, pericolosità e voto; opere finite; margine del sondaggio (storia 158) |
| 74, 75 | 10 | macroarea, disoccupazione, reddito, pericolosità, quote dei diciotto gruppi, seggi del Senato, primo partito |
| 77 | 3 | |
| 78, 79, 80, 83 | 11 | rosso e fili di minaccia e scissione; origine dei nati e leader nuovi (11f, dal registro) |
| 81, 82 | 12 | fili dell'intesa a 40 e 25 (storia 168) |
| 84, 85, 87 | 16 | spartizione di default per dicastero, pallino blu, peso dal core (storia 167) |
| 86, 88 | 17 | intesa di partenza, «entra in ultimatum», «può uscire» (storia 167) |
| 89, 90 | 18 | coalizione del ribaltone e primo per seggi (storia 167) |
| 91 | 14, 15 | genere dei personaggi (storia 168) |

---

### Task 1: passacarte della fetta, `Fascia` e debiti del guscio

**Subagente:** `rust-engineer`: il task tocca `.rs`.

**Sequenza:** primo della PR 1, da solo. Il Task 2 aspetta il suo commit.

**Skill-gate:** `rust-skills`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:141-142`: i due metodi pubblici senza comando Tauri. Le righe di `partita.rs` scritte lì sono vecchie di tre: valgono `:2425` e `:2551` di questo task.
- `docs/ricerca/sintesi/vista-per-schermata.md:149`: `Fascia` non è esportato in TypeScript.
- `docs/ricerca/sintesi/vista-per-schermata.md:24`: la forma attesa del tipo: `"bassa" | "media" | "alta"`.

**Files:**
- Modify: `crates/sim-core/src/elettorato/mod.rs:72-74`; `app/src-tauri/src/lib.rs`
- Rigenerato: `app/src/lib/tipi/Fascia.ts`

**Interfaces:**
- Consumes: `Partita::anteprima_campagna(&self, spese: &[Spesa]) -> Result<Vec<VoceAnteprimaCampagna>, Errore>` (`crates/sim-core/src/partita.rs:2425`); `Partita::elettori_campione(&self, regione: &str, c: Condizione, f: Fascia) -> Result<Vec<ElettoreCampione>, Errore>` (`:2551`); `sulla_partita` (`app/src-tauri/src/lib.rs:414-421`).
- Produces, i comandi:
  - `anteprima_campagna(spese: Vec<Spesa>) -> Result<Vec<VoceAnteprimaCampagna>, ErroreGuscio>`
  - `elettori_campione(regione: String, condizione: Condizione, fascia: Fascia) -> Result<Vec<ElettoreCampione>, ErroreGuscio>`

  Tutti e due sono involucri di una riga su `sulla_partita`: l'errore del core è un rifiuto, la partita assente un guasto.
- Su `Fascia` va la stessa riga degli altri tipi esportati: `#[cfg_attr(test, derive(TS), ts(export))]`.

**Debiti del guscio** (decisione 3 di Mario):
- il commento di `SemeFondazione` (`lib.rs:31-32`) dice che la fondazione «consuma» il seme: dalla #55 il seme resta finché la fondazione non riesce. Il commento dice ciò che il codice fa;
- il lock del seme (`lib.rs:61`, `:75`) dà un guasto se è avvelenato, e blocca anteprime e fondazioni fino al riavvio. Un `Option<u64>` è valido anche dopo un panico: i due punti prendono il lock anche avvelenato, come `blocca_comunque` (`lib.rs:88-90`);
- `InCorso.id` è `Option<String>` (`lib.rs:23`) e i quattro `if let Some(id)` (`:164`, `:180`, `:224`, `:240`) non hanno più un ramo `None` in produzione. Diventa `String`; `avvia`, che vive solo nei test, prende un id di prova.

- [ ] **Step 1: scrivi i test che falliscono**, in `lib.rs` `mod tests`, sulle funzioni interne come fanno i test di `serie`:
  - `anteprima_di_campagna_senza_partita_e_un_guasto` ed `elettori_senza_partita_e_un_guasto`: `Err(guasto(NESSUNA))`.
  - `quattro_spese_sono_un_rifiuto`: su una partita al turno zero, quattro spese su quattro condizioni diverse danno `Specie::Rifiuto` e il testo contiene «Al massimo 3 bersagli».
  - `una_spesa_da_la_sua_voce`: una spesa `{ condizione: pensionato, mobilitare, 300.000 }` dà una voce con gli stessi `bersaglio`, `leva` ed `euro`.
  - `regione_sconosciuta_e_un_rifiuto`: `elettori_campione("atlantide", …)` dà `Specie::Rifiuto`.
  - `tre_elettori_per_gruppo`: `("piemonte", DipendentePrivato, Media)` dà tre elettori, e due chiamate uguali danno lo stesso risultato.
  - `il_seme_avvelenato_non_blocca_l_anteprima`: dopo un panico tenendo il lock di `SemeFondazione`, `anteprima_di_fondazione` riesce e `seme_per_fondare` dà il seme nuovo.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml -p app`. Expected: errore di compilazione sui comandi che mancano. Il nome del package si legge in `app/src-tauri/Cargo.toml`.
- [ ] **Step 3: implementa** i due comandi, registrali in `generate_handler!` (`lib.rs:471-491`), aggiungi la riga su `Fascia` e chiudi i tre debiti.
- [ ] **Step 4: esegui tutto e rigenera i tipi.** Run, uno per chiamata:
  - `CARGO_TARGET_DIR=<wt>/target TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`
  - `cargo clippy --manifest-path <wt>/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
  - `git -C <wt> status --short app/src/lib/tipi`

  Expected: tutti i test passano, clippy pulito, e fra i tipi compare solo `Fascia.ts` (`"bassa" | "media" | "alta"`).
- [ ] **Step 5: commit.** `feat(app): passacarte della campagna e degli elettori campione`

---

### Task 2: giunture della UI per la fetta

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 1. Da solo: i Task 3-7 consumano ciò che produce.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/modelli/sondaggi-ed-errore.md:55`: la quota letta è la media degli istituti: è ciò che `quotaVotanti` calcola.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:136`: le tinte dei partiti vanno sempre col nome del partito nel nome accessibile: vale per `BarraSeggi`.

**Files:**
- Modify: `app/src/lib/guscio/chiamate.ts`, `chiamate.test.ts`, `finto.ts`, `finto.test.ts`; `app/src/lib/partita.svelte.ts`, `partita.test.ts`; `app/src/lib/tracciati.ts`; `app/src/lib/stile/fondamenta.css`; `app/src/lib/sviluppo/ProvaComponenti.svelte`
- Create: `app/src/lib/partiti.ts`, `partiti.test.ts`; `app/src/lib/componenti/Caselle.svelte`, `VoceCausa.svelte`, `BarraSeggi.svelte`, `Faccia.svelte`

**Interfaces:**
- Consumes: i due comandi del Task 1; `ultimoVince` (`app/src/lib/editor/editor.svelte.ts`).
- Produces:

```ts
// chiamate.ts
guscio.anteprimaCampagna(spese: Spesa[]): Promise<VoceAnteprimaCampagna[]>   // invoke "anteprima_campagna", { spese }
guscio.elettoriCampione(regione: string, condizione: Condizione, fascia: Fascia): Promise<ElettoreCampione[]>
/** Il testo del core senza «comando non ammesso: » in testa. */
export function testoRifiuto(e: ErroreGuscio): string;

// partita.svelte.ts
/** Come fonda e riprendi: Vista nuova, o null davanti a un guasto; il rifiuto si rilancia al chiamante. */
partita.comanda(comando: Comando): Promise<Vista | null>;

// partiti.ts
export const COLORI: Record<string, string>;            // id → variabile CSS, per gli otto dello scenario
/** Il colore del partito, o null: un nato in partita va a retino grigio. */
export function coloreDi(id: string): string | null;
export function nomeDi(v: Vista, id: string): string;   // il giocatore: il nome del suo partito
/** Media sugli istituti di quote[j] / (1 − astensione); null se il partito non ha riga. */
export function quotaVotanti(s: Sondaggio, j: number): number | null;
/** La stessa quota dentro un incrocio (una condizione, una regione). */
export function quotaIncrocio(i: Incrocio, j: number): number | null;
/** {giocatore} ∪ coalizione al voto, ∪ richieste in trattativa, ∪ alleati al governo e in crisi. */
export function coalizioneDi(v: Vista): string[];
export function iniziali(nome: string): string;         // «Luca Benedetti» → «LB»
```

- `tracciati.ts` usa `quotaVotanti` e `coalizioneDi` al posto delle sue copie private (`tracciati.ts:40`): una formula in un posto solo. I test dei tracciati restano verdi senza modifiche.
- `fondamenta.css`: otto variabili `--p-<id>` coi valori di `23-paese.prototype.html:276`; sotto `:root.grave` valgono tutte `var(--grigio)`.
- `Caselle`: props `{ piene: number; su: number; filo?: number; rosso?: boolean; nome: string }`. Caselle piccole in fila, col nome della classe accanto e per i lettori di schermo. Serve la fascia a tre (anteprima della campagna, intesa degli alleati avversari) e la barra a dieci dell'intesa.
- `VoceCausa`: props `{ fatto: string; causa?: string }`. Fatto in neretto, «perché:» in grigio a 12 px (spec:299).
- `BarraSeggi`: props `{ righe: { id: string; nome: string; seggi: number }[]; totale: number; segno?: number }`. Barra dei seggi coi colori dei partiti, il filo rosso della metà più uno e, se c'è `segno`, il segno nero sotto i seggi della coalizione. Ogni tratto ha il nome nel `title` e nell'elenco accanto.
- `Faccia`: props `{ nome: string; misura: 34 | 46 | 62 | 84 }`. In questa PR disegna le iniziali nel cerchio d'inchiostro; il Task 15 le dà il disegno.

**Guscio finto.** `installaFinto` carica sempre anche `23-`, `26-` e `27-` (come oggi `13-`, `14-`, `15-`). `rispondi` impara:

| Comando | Risposta |
|---|---|
| `anteprima_campagna` | la voce di `26-campagna.json` (`anteprime`, `piano`) con le stesse spese; se manca, per ogni spesa la voce dello stesso bersaglio con l'importo più vicino; altrimenti rifiuto «non nel campione» |
| `elettori_campione` | `23-paese.json[stato].campione["regione\|condizione\|fascia"]`, con `stato` quello dell'URL se il campione è il 23, altrimenti `governo` |
| `comanda` con `dichiara_coalizione` | da `26-campagna.json`, `prove.<stato>.coalizioni`, con la chiave fatta dagli id uniti da `+` (`""` per nessuno): la Vista del campione con `coalizione` e `disponibilita` della prova, o il suo rifiuto |
| `comanda` con `compra` del sondaggio | la Vista del campione con `acquisti.sondaggio_privato` della prova (`prove.<stato>.privato`, o lo stato del 23) |
| `comanda` con gli altri tipi della fetta | la Vista «dopo» che il campione porta per quel comando (`27-trattativa.json`: `proposte`, `allarga_uno`, `allarga_tutti`, `chiedi`, `rifiuta`; `26-campagna.json`: `riposiziona`, `promesse`, `rifiuti_campagna`), o il suo rifiuto |
| il resto | come oggi: `dopo_comando`, poi rifiuto «non nel campione» |

La forma di ogni voce si legge nel campione e nella sua riga di `.scratch/v1-piano-11-ui/viste/README.md`. Un comando che il campione non ha giocato resta un rifiuto: il finto non inventa Viste.

- [ ] **Step 1: scrivi i test che falliscono.**
  - `chiamate.test.ts`: il test dei nomi dei comandi conta 21 nomi; `anteprimaCampagna([])` chiama `anteprima_campagna` con `{ spese: [] }`; `testoRifiuto({specie:"rifiuto", testo:"comando non ammesso: Al massimo 3 bersagli"})` è `"Al massimo 3 bersagli"`, e un testo senza prefisso resta com'è.
  - `partita.test.ts`: `comanda` riuscito aggiorna `vista`; un rifiuto si rilancia e lascia `vista` com'era; due `comanda` sovrapposti fanno una chiamata sola (punto da sorvegliare 2).
  - `partiti.test.ts`, sulle Viste campione:
    - su `01-turno-zero.json`, `quotaVotanti` del giocatore formattata è `"19,2%"` (lo stesso oracolo di `tracciati.test.ts`);
    - `coalizioneDi`: 01 → `["giocatore"]`; `26-campagna.json["dichiarata"]` → giocatore, centrodestra, destra; `02-trattativa-incarico.json` → gli stessi tre;
    - `coloreDi("destra")` non è `null`; `coloreDi("nuovo_8")` è `null` (l'id di un nato in partita nel campione `17-cruscotto.json`);
    - su `17-cruscotto.json["crisi_provvisorio"].vista` (dieci partiti): `quotaVotanti` è un numero per ogni indice di `partiti`, e `null` per un indice oltre l'ultimo (punto da sorvegliare 4);
    - `iniziali("Luca Benedetti")` è `"LB"`; `iniziali("")` è `""`.
  - `finto.test.ts`: `anteprima_campagna` con le spese di `piano` dà tre voci; `elettori_campione` su `abruzzo|autonomo|alta` dà tre elettori e il primo è «Fabio Caruso»; `dichiara_coalizione` con Centrodestra e Destra sullo stato `zero` dà una Vista con `coalizione` di due id; una coalizione che la prova rifiuta lancia un rifiuto col testo della prova.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa**, poi aggiungi i quattro componenti alla pagina di prova (`ProvaComponenti.svelte`), in ogni stato.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito. Nel browser, `/sviluppo/componenti` accanto a `08-fondamenta.prototype.html`: fascia e voce con causa come nel prototipo.
- [ ] **Step 5: commit.** `feat(app): giunture della fetta 11b, comanda e guscio finto`

---

### Task 3: mappa delle regioni e selettore

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 2, in parallelo col Task 4. Il parallelo regge: oltre ai file elencati questo task tocca `ProvaComponenti.svelte` (Step 3) e i token della rampa in `fondamenta.css`, e il Task 4 non tocca nessuno dei due.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:98`: ancore dei segni dal punto interno, con lo spostamento a mano per Liguria, Valle d'Aosta e Molise. Per più segni vale il piano (in fila), e il bollino col numero resta fuori.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:99`: i segni lasciano il clic alla regione (`pointer-events: none`) e il loro contenuto va nel nome accessibile.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:112`: `radiogroup` con `radio`, `aria-checked` e tabindex mobile; Invio e Spazio scelgono. Per le frecce vale il piano: seguono `ordine`.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:113`: la forma del nome accessibile: nome, valore, classe, segni.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:114`: l'anello del fuoco ricopiato in coda con `<use>`, a doppio tratto chiaro e scuro.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:115`: la lista delle venti regioni è il controllo equivalente per le tre regioni sotto i 24 px.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:133`: cinque classi a passi uguali di luminosità, la più chiara distinta dalla carta.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:135`: nel tono grave la rampa grigia segue gli stessi passi.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:140`: la riga di attribuzione ISTAT da mettere in testa a `regioni.ts`.

**Files:**
- Create: `app/src/lib/mappa/regioni.ts`, `mappa.ts`, `mappa.test.ts`, `Mappa.svelte`, `SelettoreRegione.svelte`
- Modify: `app/src/lib/stile/fondamenta.css` (i token della rampa), `app/src/lib/sviluppo/ProvaComponenti.svelte`

**Interfaces:**
- Consumes: `Vista.sondaggio.regioni` per l'ordine; i token.
- Produces:

```ts
// regioni.ts: generato una volta dal GEO del prototipo 23, con l'attribuzione ISTAT in testa al file
export const TELA: { w: number; h: number };
export const REGIONI: readonly { id: string; nome: string; d: string; ancora: [number, number] }[]; // venti, nell'ordine dei dati
// mappa.ts
/** L'id della regione di indice i nei dati, e il contrario: l'ordine è quello di Vista.sondaggio.regioni. */
export function idDi(v: Vista, indice: number): string | null;
export function indiceDi(v: Vista, id: string): number | null;
/** Cinque classi a passi uguali fra il minimo e il massimo dei valori; null resta null (senza dato). */
export function classi(valori: (number | null)[]): (0 | 1 | 2 | 3 | 4 | null)[];
export function estremi(valori: (number | null)[]): { min: number; max: number; passo: number } | null;
/** La regione accanto nell'ordine della lista, girando agli estremi. */
export function accanto(id: string, ordine: string[], verso: 1 | -1): string;
```

- `Mappa.svelte`: props `{ valori: Record<string, number | null>; segni?: Record<string, Segno[]>; scelta: string | null; ordine: string[]; nome: string; piccola?: boolean; solaLettura?: boolean; onscelta }`, con `type Segno = "cantiere" | "atto" | "opera" | "emergenza"`.
  - un `radiogroup` con una sola fermata di Tab; le frecce seguono `ordine`; Invio e Spazio scelgono;
  - gli anelli del fuoco e della scelta ricopiati in coda con `<use>`;
  - cinque tinte dalla carta al rosso scuro, all'inchiostro nel tono grave, coi valori di `rampa()` del prototipo 23 messi come token;
  - i segni sull'ancora della regione: triangolo, cerchio, quadrato, croce rossa; più segni in fila;
  - il nome accessibile di ogni regione dice nome, valore e segni;
  - una regione senza valore è a tratteggio.
- `SelettoreRegione.svelte`: props `{ valori; scelta; azione: string /* «Spendi in {regione}» */; onconferma; onannulla }`. La mappa piccola sopra, la lista a due colonne sotto, poi il bottone d'azione (spec:336). La lista è l'equivalente da tastiera per le regioni piccole.

**Geometria.** I tracciati e la tela vengono dalla costante `GEO` di `23-paese.prototype.html:248`; i nomi da `REGN` (`:269`) e dalle chiavi di `GEO`; le ancore dei segni dal prototipo, con lo spostamento a mano per Liguria, Valle d'Aosta e Molise. In testa a `regioni.ts`: «Confini: ISTAT, unità amministrative a fini statistici al 1° gennaio 2026, versione generalizzata, CC BY 4.0; semplificati con mapshaper».

- [ ] **Step 1: scrivi i test che falliscono**, in `mappa.test.ts`:
  - `REGIONI` ha venti voci con id distinti, e gli id sono quelli di `01-turno-zero.json`, `sondaggio.regioni`, nello stesso ordine;
  - `idDi(v, 14)` è `"campania"` e `indiceDi(v, "campania")` è 14 (l'indice che il campione `26-campagna.json`, `piano`, usa per la Campania); `idDi(v, 20)` è `null` (punto da sorvegliare 5);
  - `classi([0, 25, 50, 75, 100])` è `[0, 1, 2, 3, 4]`; `classi([10, 10, 10])` è tutta una classe; `classi([null, 5])` tiene il `null`; `classi([])` è `[]`;
  - `accanto("sardegna", ordine, 1)` è la prima dell'ordine.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa**, poi aggiungi mappa e selettore alla pagina di prova, coi valori del consenso della Vista 03.
- [ ] **Step 4: verifica nel browser** su `/sviluppo/componenti`: un solo Tab entra nella mappa, le frecce girano tutte e venti le regioni, Invio sceglie; il selettore si usa tutto da tastiera; nel tono grave la rampa è d'inchiostro. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): mappa delle regioni e selettore`

---

### Task 4: cruscotto del turno zero e riquadro dell'evento

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 2, in parallelo col Task 3. È il primo della fetta a modificare `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`: il Task 5 parte dal suo commit.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:47`: ciò che costa al giocatore ha peso visivo proporzionale: l'evento sta in testa.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza si legge prima della scelta: è la riga «Senza scelta, alla chiusura: …».

**Files:**
- Create: `app/src/lib/cruscotto/cruscotto.ts`, `cruscotto.test.ts`, `Cruscotto.svelte`, `Evento.svelte`, `Riga.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/lib/cornice/Cornice.svelte`; `app/src/routes/partita/[sezione]/+page.svelte`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`, `coalizioneDi`, `nomeDi` (Task 2); `VoceCausa`, `Pulsante`, `Rifiuto`, `Segnaposto`.
- Produces:

```ts
/** Gli eventi con più di un'opzione e senza scelta: le scelte che la chiusura lascia al default (decisione 4). */
export function scelteAlDefault(v: Vista): number;
export function defaultDi(e: VoceEvento): VoceOpzione | null;       // l'opzione col campo default
export function opzioneUnica(e: VoceEvento): boolean;
/** Chi può rispondere: il giocatore che governa, fuori dalla trattativa. */
export function risponde(v: Vista): boolean;
/** Le due righe del turno del voto: lo stato della coalizione e della campagna, in parole. */
export function righeDelVoto(v: Vista): { coalizione: { dichiarati: string[]; disponibili: string[] }; campagna: { spese: number; euro: number } };
```

- `fettaDi("cruscotto", "turno_zero")` dà `null`: la pagina monta `Cruscotto`. Per `governo` e `crisi` resta «11c», per `opposizione` «11e»: la pagina monta `Evento` in testa e sotto il `Segnaposto` (lettura 1).
- `Evento.svelte`: props `{ evento: VoceEvento; indice: number; puoRispondere: boolean }`. Famiglia e regione, nome e costo, la causa, le opzioni come bottoni con la scelta a fondo giallo e il default segnato, e sotto «Senza scelta, alla chiusura: …» (storia 61). Il clic manda `{ tipo: "rispondi_evento", evento: indice, opzione: id }`; il rifiuto compare sotto i bottoni. Con una sola opzione il riquadro lo dice e non ha bottoni. Senza `puoRispondere` le opzioni si leggono e basta. Nel tono grave, le regole delle fondamenta.
- `Cruscotto.svelte`, al turno zero: gli eventi in testa; poi le due righe «Dichiara la coalizione» e «Fai la campagna», ciascuna col suo stato («nessuna coalizione dichiarata», i nomi dei dichiarati, «nessuna spesa», «3 spese, 600.000 €») e il rimando «Campagna ›» (storia 60). Le proposte dei ministri del governo uscente restano fuori (spec:375).
- `Cornice.svelte`: sotto «Chiudi il turno» compare il conto, con le chiavi che la 11a ha già (`cornice.scelte_al_default`): «N scelte vanno al default ›», che porta al cruscotto, o «nessuna scelta al default». In trattativa, al suo posto, la riga del default (Task 16).

- [ ] **Step 1: scrivi i test che falliscono**, in `cruscotto.test.ts`:
  - `scelteAlDefault`: 0 su `01-turno-zero.json` (nessun evento); 1 su `03-governo-ordinario.json` («Emergenza idrogeologica», tre opzioni, nessuna scelta); 0 su `17-cruscotto.json["scelte_fatte"].vista`, dove l'evento ha `scelta` 1; 0 su `05-crisi.json`, dove lo scandalo ha un'opzione sola;
  - `defaultDi` dà l'opzione con `default` vero; `opzioneUnica` è vero sull'evento della 05;
  - `risponde`: vero sulla 03, falso sulla 02 (trattativa) e sulla 06 (opposizione);
  - `righeDelVoto(01)`: `dichiarati` vuoto, `disponibili` Centrodestra e Destra, `spese` 0; su `26-campagna.json["dichiarata"]`: due dichiarati.
  - `ossatura.test.ts`: `fettaDi("cruscotto", "turno_zero")` è `null`; al governo resta «11c».
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `17-cruscotto.prototype.html`:
  - `/partita/cruscotto?vista=01-turno-zero.json`: le due righe col rimando; nessun evento, e la cornice dice «nessuna scelta al default»;
  - `?vista=17-cruscotto.json&stato=scelte_fatte`: l'evento con la scelta a fondo giallo;
  - `?vista=08-tono-grave.json`: il riquadro dell'evento nel tono grave, e sotto il segnaposto della 11c;
  - `?vista=05-crisi.json`: l'evento a opzione unica lo dice;
  - tutto si raggiunge e si attiva da tastiera.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): cruscotto del turno zero ed evento in testa`

---

### Task 5: Campagna, coalizione e cessioni

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 3 e il Task 4. Col 4 condivide `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:60`: i campi della Vista che servono la coalizione (`disponibilita`, `coalizione`) e il comando `DichiaraCoalizione`.

**Files:**
- Create: `app/src/lib/campagna/campagna.ts`, `campagna.test.ts`, `Campagna.svelte`, `Coalizione.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`, `nomeDi` (Task 2).
- Produces:

```ts
/** Il comando che dichiara la coalizione di oggi con un partito in più o in meno; le cessioni degli usciti cadono. */
export function dichiara(v: Vista, cessioni: Cessione[], partito: string, entra: boolean): Comando;
/** Il comando con la cessione a `partito` portata a `seggi`; 0 la toglie. */
export function cedi(v: Vista, cessioni: Cessione[], partito: string, seggi: number): Comando;
export const PASSO_CESSIONE = 5;
/** I seggi attesi del giocatore alla Camera sulla proiezione pubblica; null senza proiezione. */
export function seggiAttesi(v: Vista): number | null;
/** Una riga per partito IA, nell'ordine di Vista.partiti: dichiarato, disponibile, causa. */
export function righeCoalizione(v: Vista): { id: string; nome: string; dichiarato: boolean; disponibile: boolean; causa: string }[];
```

- `Campagna.svelte` è la forma A · Tavolo: tre colonne (Coalizione, Spesa, Posizione e promesse) e la cassa nel dettaglio a lato. Questo task monta la prima colonna; le altre due e il lato portano il nome della colonna e arrivano coi Task 6 e 7. `fettaDi("campagna", …)` dà `null`.
- `Coalizione.svelte` (storie 62-63, ticket 26):
  - una riga per partito, disponibile o no, con la causa del core;
  - «Aggiungi» e «Togli» mandano subito `dichiara_coalizione`, senza conferma; il rifiuto del core sta sulla riga che l'ha causato;
  - sotto ogni partito non disponibile, i seggi delle liste comuni da cedere a passi di `PASSO_CESSIONE`: ogni passo è una dichiarazione, con l'ingresso o il rifiuto del core sulla riga;
  - accanto, i seggi attesi del giocatore, con la nota che vengono dal sondaggio pubblico. Il tetto delle cessioni non c'è nella Vista: lo dice il rifiuto (spec:376).
- Le cessioni fatte vivono nello stato del componente: la Vista non le riporta. Al rientro nella pagina ripartono da zero, e la riga lo dice.

- [ ] **Step 1: scrivi i test che falliscono**, in `campagna.test.ts`:
  - `dichiara(01, [], "centrodestra", true)` è `{ tipo: "dichiara_coalizione", partiti: ["centrodestra"], cessioni: [] }`;
  - su `26-campagna.json["dichiarata"]`, `dichiara(v, [{partito:"destra", seggi:10}], "destra", false)` ha `partiti` `["centrodestra"]` e `cessioni` vuote;
  - `cedi(v, [], "centro", 10)` aggiunge la cessione; `cedi(v, [{partito:"centro", seggi:10}], "centro", 0)` la toglie;
  - `righeCoalizione(01)`: sette righe; Centrodestra e Destra disponibili; il Centro non disponibile con «posizione troppo lontana dalla tua»;
  - `seggiAttesi(01)` è 80.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `26-campagna.prototype.html`, su `/partita/campagna?vista=26-campagna.json&stato=zero`:
  - «Aggiungi» su Centrodestra e Destra li dichiara;
  - aggiungere il Centrosinistra a Centrodestra e Destra mostra sulla riga «Destra non entra: posizione troppo lontana dalla tua»;
  - una cessione all'Estrema destra mostra il rifiuto del core sulla riga;
  - su `?vista=03-governo-ordinario.json` la sezione è barrata e `/partita/campagna` torna alla casa.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): campagna, coalizione e cessioni`

---

### Task 6: Campagna, spesa, cassa e rivali

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 5: stessi `campagna.ts`, `campagna.test.ts` e `Campagna.svelte`. Consuma il selettore del Task 3.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:65`: i campi della Vista e i comandi della campagna.
- `docs/ricerca/modelli/effetti-della-campagna-e-promesse.md:55`: l'ordine delle leve a pari spesa (persuadere, mobilitare, convincere): serve alla riga che spiega l'effetto di ogni leva, se il prototipo non la porta.
- `docs/ricerca/modelli/informazione-a-pagamento.md:120`: il prezzo del sondaggio sta sulla scala reale e lo decide il core: la UI lo legge da `acquisti.prezzi`.

**Files:**
- Create: `app/src/lib/campagna/Spese.svelte`, `Cassa.svelte`
- Modify: `app/src/lib/campagna/campagna.ts`, `campagna.test.ts`, `Campagna.svelte`

**Interfaces:**
- Consumes: `guscio.anteprimaCampagna`, `partita.comanda`, `quotaIncrocio`, `coloreDi` (Task 2); `SelettoreRegione`, `idDi`, `indiceDi` (Task 3); `Caselle`; `ultimoVince` (`app/src/lib/editor/editor.svelte.ts`).
- Produces:

```ts
export const LEVE: readonly Leva[];                       // mobilitare, persuadere, convincere
export const MASSIMO_SPESE = 3;
/** Cassa del giocatore, prezzo del sondaggio se comprato in questo turno, somma delle spese, resto. */
export function conto(v: Vista, spese: Spesa[]): { cassa: number; sondaggio: number; spese: number; resto: number };
/** Cosa farebbe la chiusura con questa cassa: le spese si pagano in ordine finché la cassa basta. */
export function taglio(spese: Spesa[], cassa: number): { indice: number; paga: number; esito: "intera" | "scende" | "cade" }[];
/** La quota del giocatore fra i votanti del bersaglio, sul sondaggio pubblico e sul privato se comprato. */
export function quotaNelBersaglio(v: Vista, b: Bersaglio): { pubblica: number | null; privata: number | null };
/** Le casse dei partiti IA, dalla più ricca, e le loro promesse in sospeso. */
export function rivali(v: Vista): { id: string; nome: string; cassa: number; promesse: VocePromessa[] }[];
```

- `Spese.svelte` (storie 64-65, 69): fino a tre riquadri.
  - Bersaglio fra le sei condizioni, a pulsanti, e le venti regioni, col `SelettoreRegione` che prende il posto della cassa nel lato e chiude con «Spendi in {regione}»; la mappa piccola è tinta con la quota del giocatore per regione.
  - Leva a tre, con l'effetto nel `title` e in una riga per i lettori di schermo.
  - Importo libero, in euro interi.
  - La fascia dell'anteprima (`Caselle` su tre: poco, medio, molto) con la causa del core.
  - La quota del giocatore fra i votanti del bersaglio, pubblica e, se comprata, privata accanto.
  - `anteprima_campagna` gira a ogni modifica, dentro `ultimoVince`; il comando `{ tipo: "campagna", spese }` parte all'uscita dal campo, e solo se l'ultima anteprima è riuscita (lettura 5). Un rifiuto («Spesa oltre la cassa…», «Bersaglio ripetuto…») sta sotto il riquadro che l'ha causato.
  - All'apertura la bozza è `Vista.campagna.spese`.
- `Cassa.svelte`, nel dettaglio a lato (storie 66, 67, 69):
  - il conto: cassa, sondaggio, spese, resto;
  - **l'avviso della campagna tagliata**, quando `taglio` dà una voce che scende o cade: dice quale spesa scende e a quanto, quale cade, e che una scissione in chiusura porterebbe via altra cassa (spec:376);
  - «Compra il sondaggio privato · 15.000 €», col prezzo di `acquisti.prezzi`; comprato, lo dice; il secondo acquisto mostra il rifiuto del core sul posto;
  - le casse dei rivali a barre coi colori dei partiti e le cifre accanto, e le promesse in sospeso. Quanto e dove hanno speso lo dirà l'esito del voto: una riga lo scrive.

- [ ] **Step 1: scrivi i test che falliscono**, in `campagna.test.ts`:
  - `taglio`, sulla regola del core (`crates/sim-core/src/partita.rs:2301-2303`: le spese si tagliano in ordine):
    - due spese da 500.000 con cassa 985.000 (il campione `tagliata_prima`, dopo il sondaggio) → la prima `intera`, la seconda `scende` a 485.000;
    - due da 500.000 con cassa 400.000 → la prima `scende` a 400.000, la seconda `cade`;
    - cassa che basta → tutte `intera`; nessuna spesa → `[]`; cassa 0 → tutte `cade`.
  - `conto` su `26-campagna.json["tagliata_prima"]`: `cassa` 985.000, `spese` 1.000.000, `resto` negativo.
  - `quotaNelBersaglio(v, { regione: 14 })` legge l'incrocio della Campania; con `acquisti.sondaggio_privato` `null`, `privata` è `null`.
  - `rivali(01)`: sette voci, la prima è la Sinistra con 8.135.300 €.
  - **Corsa delle anteprime** (punto da sorvegliare 1), con promesse risolte a mano come in `editor.test.ts:125`: tre importi scritti mentre la prima anteprima è in volo fanno due chiamate, e la fascia mostrata è quella dell'ultimo importo.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, su `/partita/campagna?vista=26-campagna.json` con `&stato=zero`, `piano_scelto` e `tagliata_prima`:
  - tre spese con le loro fasce; la quarta non si può aggiungere, e il motivo è scritto;
  - una regione si sceglie dalla mappa piccola e dalla lista, tutte e due da tastiera;
  - su `tagliata_prima` l'avviso dice che la seconda spesa scende a 485.000 € (punto da sorvegliare 3);
  - il rifiuto «Spesa oltre la cassa» compare sotto il riquadro.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): campagna, spese, cassa e rivali`

---

### Task 7: Campagna, posizione e promesse

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 6: stessi `campagna.ts`, `campagna.test.ts` e `Campagna.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:160`: la minaccia di scissione non è in `VocePartito`: è il motivo del posto vuoto che nomina la 11g.
- `docs/ricerca/modelli/effetti-della-campagna-e-promesse.md:61`: una promessa mancata pesa più di una mantenuta: serve al testo che spiega la conferma, se il prototipo non lo porta.

**Files:**
- Create: `app/src/lib/campagna/Posizione.svelte`, `Promesse.svelte`
- Modify: `app/src/lib/campagna/campagna.ts`, `campagna.test.ts`, `Campagna.svelte`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`; `Conferma`, `Pulsante`, `Rifiuto`.
- Produces:

```ts
export const PASSI = [-0.5, -0.25, 0.25, 0.5] as const;
/** Le quattro posizioni raggiungibili dalla dichiarata di oggi. */
export function posizioniVicine(v: Vista): number[];
/** Le promesse della campagna in corso: quelle del giocatore non in sospeso. */
export function promesseFatte(v: Vista): VocePromessa[];
export const MASSIMO_PROMESSE = 2;
/** Il comando con le coppie (id della disposizione, id del gradino). */
export function prometti(scelte: { disposizione: string; gradino: string }[]): Comando;
```

- `Posizione.svelte` (storia 68): l'asse da 1 a 7 con la dichiarata, la percepita e gli altri partiti; i quattro passi; «Spostati a {posizione} ›» dentro `Conferma`, con «Conferma lo spostamento» e la spiegazione che si fa una volta. Il rifiuto del core sta sotto. L'anteprima dello spostamento e le soglie di scissione sono un posto vuoto che nomina la 11g: fino ad allora si leggono dichiarata e percepita di oggi (spec:376).
- `Promesse.svelte` (storia 68): fino a due, disposizione e gradino scelti da `Vista.disposizioni`, tutte (spec:376). Il gradino in vigore è segnato; chi lo sceglie vede il rifiuto del core. «Prometti ›» dentro `Conferma`, con «Conferma la promessa» o «Conferma le promesse». Le promesse già fatte in questa campagna si leggono al posto del modulo. L'intesa che ogni alleato perderebbe è un posto vuoto che nomina la 11g.
- I due pulsanti sono spenti mentre `partita.occupato` (punto da sorvegliare 2).

- [ ] **Step 1: scrivi i test che falliscono**, in `campagna.test.ts`:
  - `posizioniVicine(01)` è `[4, 4.25, 4.75, 5]`;
  - `promesseFatte(01)` è vuota; su `26-campagna.json["piano_scelto"]` sono due («Reddito minimo: universale», «Età pensionabile: 64 anni»); su `["piano_dopo"]`, dopo il voto, è vuota: le stesse due sono passate in sospeso;
  - `prometti([{disposizione:"a", gradino:"b"}])` è `{ tipo: "prometti", promesse: [["a", "b"]] }`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, su `/partita/campagna?vista=26-campagna.json&stato=zero`:
  - lo spostamento a 4 si conferma sul posto, da tastiera; Esc annulla;
  - una terza promessa non si può aggiungere;
  - su `&stato=governo_fine` un passo che il core rifiuta mostra il rifiuto sotto l'asse;
  - due Invio veloci su «Conferma lo spostamento» fanno una chiamata sola.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): campagna, posizione e promesse`

---

### Task 8: verifica della PR 1

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 7. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

Lo esegue la sessione principale. Ogni esito va nella PR.

- [ ] **Step 1: batterie nei due motori**, col server di sviluppo acceso e la macchina scarica. Le pagine:
  - `zero=/partita/cruscotto?vista=01-turno-zero.json`
  - `evento=/partita/cruscotto?vista=17-cruscotto.json&stato=scelte_fatte`
  - `grave=/partita/cruscotto?vista=08-tono-grave.json`
  - `campagna-zero=/partita/campagna?vista=26-campagna.json&stato=zero`
  - `campagna-dichiarata=…&stato=dichiarata`, `campagna-piano=…&stato=piano_scelto`, `campagna-tagliata=…&stato=tagliata_prima`
  - `campagna-governo=…&stato=governo_fine`, `campagna-opposizione=…&stato=opposizione_fine`

  Run: `npm --prefix <wt>/app run verifica:chrome -- <pagine>`, poi `npm --prefix <wt>/app run verifica:webkit -- <pagine>`. Expected: uscita 0 in tutti e due; tagli 0 su ogni misura; nessun `← MOVIMENTO`, `← TASTIERA` o `← ALTROVE`.
- [ ] **Step 2: attivazione da tastiera, a mano nel browser**: la risposta a un evento, «Aggiungi» e «Togli», una cessione, il selettore di regione, le due conferme sul posto. La batteria conta il Tab ma non preme i tasti (`app/verifica/README.md:112`).
- [ ] **Step 3: test e tipi.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`, poi `grep -rlw "non nel campione" <wt>/app/build`. Expected: PASS, pulito, nessun file.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11b/`: cruscotto del turno zero e campagna, a 1440×900 e a 1280×800, più la campagna a 1280 con 1,25×. Vanno a `craft-reviewer` e nel corpo della PR.
- [ ] **Step 5: corpo della PR**, con le sezioni della 11a: «Cosa cambia», «Decisioni prese in esecuzione», «Per Mario» (scarti voluti dai prototipi, tabella dei testi nuovi), «Da sapere per le fette dopo», «Verifiche». Il testo passa da `scrittura-italiana` prima di aprire la PR.

**Fine della PR 1.**

---

### Task 9: Paese, mappa, livelli e sondaggio

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 2, da solo. Il piano lo dava in parallelo col Task 11, ma tutti e due modificano `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`: il Task 11 parte dal commit di questo.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:61`: consenso e affluenza vengono da `sondaggio.regioni`, i segni da agenda e cantieri.
- `docs/ricerca/sintesi/vista-per-schermata.md:84`: la mappa ha due livelli su sei: gli altri quattro restano barrati.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:115`: mappa e lista sono due viste dello stesso dato, e la lista è ordinabile.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:133`: cinque classi a passi uguali: la regola della legenda.
- `docs/ricerca/modelli/sondaggi-ed-errore.md:55`: il giocatore legge la media degli istituti.
- `docs/ricerca/modelli/sondaggi-ed-errore.md:59`: i seggi «se si votasse oggi» li calcola il core dal sondaggio: la UI legge `proiezione`.
- `docs/ricerca/modelli/informazione-a-pagamento.md:121`: il sondaggio privato ha incroci più fini: comprato, incroci e scheda passano ai suoi numeri.

**Files:**
- Create: `app/src/lib/paese/paese.ts`, `paese.test.ts`, `Paese.svelte`, `Sondaggio.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`

**Interfaces:**
- Consumes: `Mappa`, `classi`, `estremi`, `REGIONI` (Task 3); `quotaVotanti`, `quotaIncrocio`, `coalizioneDi`, `coloreDi` (Task 2); `BarraSeggi`; `partita.comanda`.
- Produces:

```ts
export const LIVELLI = ["consenso", "disoccupazione", "reddito", "affluenza", "pericolosita", "voto"] as const;
export type Livello = (typeof LIVELLI)[number];
/** null = il livello ha i dati; altrimenti la fetta che li porta («11g»). In 11b: consenso e affluenza. */
export function livelloManca(l: Livello): string | null;
/** Il sondaggio che la pagina legge: il privato se comprato, altrimenti il pubblico. */
export function sondaggioLetto(v: Vista): { sondaggio: Sondaggio; privato: boolean };
/** Consenso per regione di una coalizione o di un partito solo: somma delle quote fra i votanti. */
export function consensoPerRegione(v: Vista, chi: string[]): Record<string, number | null>;
/** Uno meno l'astensione del sondaggio della regione. */
export function affluenzaPerRegione(v: Vista): Record<string, number | null>;
/** Cantieri, atti in agenda ed emergenze aperte, per id di regione. Le opere finite arrivano in 11g. */
export function segniPerRegione(v: Vista): Record<string, Segno[]>;
/** La lista delle venti regioni, per nome o per valore. */
export function lista(valori: Record<string, number | null>, ordine: "nome" | "valore"): { id: string; nome: string; valore: number | null }[];
/** La media degli istituti: una quota per partito e l'astensione a parte. */
export function media(v: Vista, s: Sondaggio): { partiti: { id: string; nome: string; quota: number }[]; astensione: number };
```

- `Paese.svelte` è la forma A · Atlante (ticket 23):
  - in testa le sei linguette dei livelli; un livello senza dati è barrato col motivo e la fetta, come nel sommario. Accanto, la scelta che il livello chiede: di chi è il consenso, con la coalizione del giocatore di default e un menu per un partito solo (storia 71);
  - la mappa a tutta altezza e accanto la lista delle venti regioni col valore e i segni, ordinabile per nome o per valore; sotto, la legenda a cinque classi con gli estremi e la legenda dei segni;
  - il dettaglio a lato tiene il sondaggio finché non si sceglie una regione; la scheda della regione arriva col Task 10.
- `Sondaggio.svelte` (storie 69, 73):
  - la media degli istituti fra i votanti, a barre coi colori dei partiti, e l'astensione a parte;
  - il riquadro del sondaggio privato: «Compra · 15.000 €» con la cassa; comprato, diventa una colonna accanto alla media, e incroci e scheda passano ai suoi numeri; «Compralo ancora» mostra il rifiuto del core sul posto;
  - i seggi se si votasse oggi, Camera e Senato, con la coalizione contro la metà più uno (`BarraSeggi` sulla `proiezione`); senza proiezione, una riga lo dice;
  - gli incroci per condizione con la quota del giocatore; un incrocio scelto apre le sue quote, l'approvazione del governo e l'autocollocazione. Il margine del sondaggio è un posto vuoto che nomina la 11g.
- L'emergenza porta la regione come nome (`VoceEvento.regione`): `segniPerRegione` la riporta all'id coi nomi di `REGIONI`.

- [ ] **Step 1: scrivi i test che falliscono**, in `paese.test.ts`:
  - `livelloManca("consenso")` e `("affluenza")` sono `null`; gli altri quattro danno `"11g"`;
  - su `23-paese.json["governo"].vista`: `consensoPerRegione` della coalizione di governo ha il minimo in Basilicata (37,6%) e il massimo in Toscana (69,2%), arrotondati a un decimale (ticket 23:50);
  - `sondaggioLetto` sullo stesso campione ha `privato` vero; sulla 01 falso;
  - `affluenzaPerRegione(01)` ha venti valori fra 0 e 1;
  - `segniPerRegione` su `23-paese.json["grave"].vista` ha `"emergenza"` sul `lazio` («Emergenza incendi», regione «Lazio»); sulla 01 è vuoto;
  - `lista(valori, "valore")` è decrescente e mette in fondo i `null`;
  - `media(v, s)` somma a 1 con l'astensione, entro 1e-9.
  - `ossatura.test.ts`: `fettaDi("paese", …)` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `23-paese.prototype.html`, su `/partita/paese?vista=23-paese.json` con `&stato=governo`, `opposizione`, `zero` e `grave`:
  - le quattro linguette senza dati sono barrate e dicono «arriva con la fetta 11g»;
  - il menu del consenso passa dalla coalizione a un partito solo e la mappa cambia;
  - al governo il privato è una colonna accanto alla media; al turno zero «Compra» c'è;
  - nel tono grave la rampa è d'inchiostro e le barre sono grigie.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Paese, mappa a due livelli e sondaggio`

---

### Task 10: Paese, scheda della regione ed elettori campione

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 9, in parallelo col Task 11 e col Task 12: lavora solo in `app/src/lib/paese/`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:142`: `elettori_campione` serve la scheda della regione. La riga di `partita.rs` scritta lì è vecchia: vale `:2551`.
- `docs/ricerca/sintesi/vista-per-schermata.md:158`: disoccupazione, reddito e gruppi della scheda mancano dalla Vista: sono i posti vuoti che nominano la 11g.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:115`: la scheda si apre anche dalla lista, che è la via da tastiera per le regioni piccole.

**Files:**
- Create: `app/src/lib/paese/SchedaRegione.svelte`
- Modify: `app/src/lib/paese/paese.ts`, `paese.test.ts`, `Paese.svelte`

**Interfaces:**
- Consumes: `guscio.elettoriCampione`, `testoRifiuto`, `quotaIncrocio`, `nomeDi`; `Fascia`, `Condizione` (tipi).
- Produces:

```ts
export const CONDIZIONI: readonly Condizione[];   // le sei, nell'ordine del tipo
export const FASCE: readonly Fascia[];            // bassa, media, alta
/** Le disposizioni territoriali che valgono nella regione: quelle col perimetro che la contiene. */
export function disposizioniDi(v: Vista, regione: string): VoceDisposizione[];
/** Il sondaggio della regione: una quota per partito fra i votanti, dal sondaggio letto. */
export function sondaggioDi(v: Vista, regione: string): { id: string; nome: string; quota: number }[];
```

- `SchedaRegione.svelte` sostituisce il sondaggio nel lato al clic sulla mappa o sulla lista, con «‹ Sondaggio» per tornare (storie 74-75):
  - il nome e i segni scritti per esteso («cantiere in corso: …»);
  - il sondaggio della regione a barre coi colori dei partiti;
  - la griglia condizione × fascia, diciotto caselle. Le quote dei gruppi sono un posto vuoto che nomina la 11g; un clic su una casella mette sotto i suoi tre elettori (`elettori_campione`): età, nome, voto o astensione, motivo, collocazione, e la riga che dice che sono inventati dal sondaggio;
  - le disposizioni territoriali in vigore col gradino, quello in arrivo e il rimando «Aula ›»;
  - a tratteggio, con «arriva con la fetta 11g»: macroarea, disoccupazione, reddito, pericolosità, seggi del Senato e primo partito dell'ultimo voto.
- Il rifiuto di `elettori_campione` sta sotto la griglia; un guasto va nella cornice.
- Cambiare regione o casella mentre una richiesta è in volo mostra solo gli elettori dell'ultima casella scelta.

- [ ] **Step 1: scrivi i test che falliscono**, in `paese.test.ts`:
  - `disposizioniDi(v, "campania")` sulla 03 contiene le disposizioni il cui `perimetro` ha `"campania"` e nessuna col perimetro vuoto (leggi dal campione quante sono);
  - `sondaggioDi(v, "toscana")` ha una voce per partito e somma a 1 entro 1e-9;
  - `CONDIZIONI.length × FASCE.length` è 18.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, su `/partita/paese?vista=23-paese.json&stato=governo`:
  - clic sull'Abruzzo, poi sulla casella autonomo · alta: tre elettori, il primo «Fabio Caruso», 28 anni;
  - la scheda si apre dalla mappa e dalla lista, da tastiera;
  - i posti vuoti nominano la 11g; nessun numero dei prototipi segnato «11g» compare.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): scheda della regione ed elettori campione`

---

### Task 11: Partiti, asse, schede e Camere

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 9, da cui eredita `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`; in parallelo col Task 10.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:160`: la minaccia di scissione non è in `VocePartito`: sull'asse restano fuori il rosso e i due fili.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:136`: le tinte dei partiti vanno sempre col nome del partito.

**Files:**
- Create: `app/src/lib/partiti/schieramento.ts`, `schieramento.test.ts`, `Partiti.svelte`, `AsseDeiPartiti.svelte`, `Camere.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`

**Interfaces:**
- Consumes: `quotaVotanti`, `coloreDi`, `nomeDi` (Task 2); `BarraSeggi`, `Faccia`, `Caselle`.
- Produces:

```ts
export const TETTO_PARTITI = 10;
/** I partiti nell'ordine dell'asse: per posizione dichiarata, a parità l'ordine della Vista. */
export function inOrdine(v: Vista): VocePartito[];
/** Schede per fila: quattro fino a otto partiti, cinque oltre. */
export function perFila(n: number): 4 | 5;
/** L'indole in breve: vocazione · regola («mista · insegue i voti»). Chiavi di t. */
export function indoleBreve(p: VocePartito): [Chiave, Chiave];
/** Gli avvisi di una scheda, nell'ordine: ultimatum, sciolto, senza seggi all'ultimo voto, governo uscente, guida il governo, nato in partita. */
export function avvisi(v: Vista, p: VocePartito): Chiave[];
/** Seggi in carica e proiezione di un partito; null a Camere sciolte o senza proiezione. */
export function seggiDi(v: Vista, id: string): { carica: SeggiPartito | null; oggi: SeggiPartito | null };
```

- `Partiti.svelte` è la forma A · Schieramento (ticket 24), di sola lettura:
  - `AsseDeiPartiti`: l'asse da 1 a 7 a tutta larghezza; ogni partito ha un quadrato del suo colore sulla dichiarata, un anello sulla percepita e la freccia fra i due; i nomi sopra, su più file quando si toccano. Il rosso e i fili di minaccia e scissione sono della 11g (lettura 4);
  - una scheda per partito, nell'ordine dell'asse, su due file: `Faccia` a 46 px, il nome sotto il filo del colore, il leader, il sondaggio, i seggi di Camera e Senato, la cassa, l'indole in breve. Il partito del giocatore ha la casella «Tu». Al governo, per ogni alleato l'intesa a dieci caselle con l'ultimatum; all'opposizione, la fascia dell'alleato del governo a tre caselle (storia 82). Sotto, gli avvisi;
  - un nato in partita ha il retino grigio, anche nelle barre; lo sciolto resta in elenco, barrato e sbiadito (storia 83);
  - `Camere`, nel dettaglio a lato senza scelta (storia 80): Camera e Senato in carica e se si votasse oggi, a barre coi colori, il filo della maggioranza, i seggi di ogni partito, «N partiti su 10» e la riga del tetto. A Camere sciolte la riga «in carica» dice «sciolte, fino al voto».
- «Senza seggi all'ultimo voto» si legge da `Vista.elezioni[0].voti`: Camera e Senato a zero (spec:378). L'origine di un nato e il leader nuovo arrivano col registro, in 11f.

- [ ] **Step 1: scrivi i test che falliscono**, in `schieramento.test.ts`:
  - `inOrdine(01)` mette il giocatore (4,5) fra Centro e Centrodestra;
  - `perFila(8)` è 4; `perFila(10)` è 5; `perFila(9)` è 5;
  - `indoleBreve` dà una coppia di chiavi diversa per ognuna delle quattro regole;
  - `avvisi`: sulla 01, Centrodestra e Destra hanno «governo uscente»; su `17-cruscotto.json["ultimatum"].vista` la Sinistra ha l'ultimatum; su `30-eredita.json` (`vista`) l'Estrema sinistra ha «sciolto»;
  - su `17-cruscotto.json["crisi_provvisorio"].vista`: `inOrdine` dà dieci partiti, e quelli con `coloreDi` `null` hanno l'avviso «nato in partita» (punto da sorvegliare 4);
  - `seggiDi(01, "destra").carica` è `null` (Camere sciolte).
  - `ossatura.test.ts`: `fettaDi("partiti", …)` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `24-partiti.prototype.html`, su `/partita/partiti` con le Viste 01, 03, 06, 08, `17-cruscotto.json&stato=ultimatum`, `17-cruscotto.json&stato=crisi_provvisorio` e `30-eredita.json`:
  - con dieci partiti le schede stanno in due file da cinque a 1280×800, senza tagli;
  - i nomi sull'asse non si coprono;
  - nel tono grave i colori sono grigi.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Partiti, asse, schede e Camere`

---

### Task 12: Partiti, scheda del partito e intesa

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 11: stessi `schieramento.ts`, `schieramento.test.ts` e `Partiti.svelte`. In parallelo col Task 10.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:160`: la minaccia di scissione non è nella Vista: la coda della frase sulla distanza resta alla 11g.

**Files:**
- Create: `app/src/lib/partiti/SchedaPartito.svelte`
- Modify: `app/src/lib/partiti/schieramento.ts`, `schieramento.test.ts`, `Partiti.svelte`

**Interfaces:**
- Consumes: Task 11; `segnato`, `numero`, `percentuale` (`app/src/lib/testi/t.ts`).
- Produces:

```ts
/** La frase sulla distanza: verso (sinistra, destra, ferma) e punti fra percepita e dichiarata. */
export function distanza(p: VocePartito): { verso: "sinistra" | "destra" | "ferma"; punti: number };
/** L'intesa con il giocatore: in cifre se è un alleato, in fascia se è un alleato del governo avversario, null altrimenti. */
export function intesaDi(v: Vista, id: string): { alleato: VoceAlleato } | { fascia: VoceFascia } | null;
/** Le mosse del partito nell'ultima chiusura. */
export function mosseDi(v: Vista, id: string): string[];
/** L'ultimo voto del partito: quota e seggi; null se non c'era o non si è ancora votato. */
export function ultimoVoto(v: Vista, id: string): VotiPartito | null;
```

- `SchedaPartito.svelte`, nel dettaglio a lato al clic su una scheda o su un partito dell'asse, con «‹ Camere» (storie 81-82):
  - `Faccia` a 84 px, il leader, gli avvisi;
  - la posizione sull'asse con quella del giocatore accanto, e la frase: «Gli elettori lo vedono a 2,76, verso destra: 0,76 punti dalla dichiarata.» La coda sulla minaccia di scissione è della 11g;
  - l'indole per esteso: vocazione e regola, con le quattro frasi di come il core muove i partiti (spec:378);
  - l'intesa: al governo la barra da 0 a 100 a dieci caselle, l'ultimatum con la sua causa, la causa che pesa di più, le variazioni degli ultimi due turni col segno e, in ultimatum, «Rispondi nel Cruscotto ›». All'opposizione la fascia con la causa dell'ultimatum e la riga «un alleato incrinato vota meno compatto»;
  - i seggi in carica e se si votasse oggi, il sondaggio, l'ultimo voto;
  - «carisma +0,61, del leader, nel voto»: la valence con segno e due decimali;
  - la cassa, le promesse, le mosse del turno.
- Le cause dell'intesa (`CausaIntesa`) hanno una chiave di testo ciascuna, anche vuota: un valore nuovo del core fa fallire il controllo dei tipi (storia 180).

- [ ] **Step 1: scrivi i test che falliscono**, in `schieramento.test.ts`:
  - `distanza` di un partito con dichiarata 2 e percepita 2,76 è `{ verso: "destra", punti: 0.76 }` entro 1e-9; con le due uguali, `ferma` e 0;
  - `intesaDi`: sulla 03 un alleato dà `alleato` con l'intesa in cifre; sulla 06 un alleato del governo dà `fascia`; il partito del giocatore dà `null`;
  - `ultimoVoto(01, "destra")` è `null`; sulla 02 ha la quota del campione;
  - `mosseDi` sulla 03 dà le mosse del partito scelto, nell'ordine della Vista.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser** su `17-cruscotto.json&stato=ultimatum` (la Sinistra in ultimatum, con la causa e il rimando al cruscotto), sulla 06 (la fascia) e sulla 03. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): scheda del partito e intesa`

---

### Task 13: verifica della PR 2

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 10 e il Task 12. Chiude la PR 2.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:115`: la lista è il controllo equivalente della mappa: la prova da tastiera dello Step 2 la percorre tutta.

Lo esegue la sessione principale, come il Task 8, sugli stati di Paese e Partiti.

- [ ] **Step 1: batterie nei due motori.** Le pagine: `/partita/paese?vista=23-paese.json` nei quattro stati; `/partita/partiti` sulle sette Viste del Task 11. Expected: uscita 0, tagli 0, nessuna riga segnata.
- [ ] **Step 2: tastiera, a mano**: la mappa come gruppo di scelta (un Tab, le frecce, Invio), la lista, le linguette, il menu del consenso, l'acquisto del sondaggio, la griglia dei gruppi, le schede dei partiti.
- [ ] **Step 3: test e tipi**, come al Task 8, passo 3.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11b/`: Paese col sondaggio e con la scheda, Partiti con otto e con dieci partiti, alle due finestre; il tono grave di tutti e due.
- [ ] **Step 5: corpo della PR**, come al Task 8, passo 5.

**Fine della PR 2.**

---

### Task 14: caricature, identità

**Subagente:** `frontend-engineer`.

**Sequenza:** primo della PR 3, in parallelo col Task 16: lavora solo in `app/src/lib/caricature/`.

**Skill-gate:** nessuna skill del ruolo ha il trigger su un modulo puro senza interfaccia: il salto si dichiara nel report. `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:52`: un hash della stringa dà la stessa faccia su ogni macchina.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:94`: la chiave dell'hash è `ruolo:partito:nome`.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:97`: la regola contro le collisioni dentro la rosa era aperta: la chiude `rosa`.
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:256`: le leve del profilo al tratto: naso, mento, fronte, colletto. Per il numero di varianti vale `VARIANTI` del piano.
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:257`: lo stato si legge dal fondo del riquadro.
- `docs/ricerca/estetica/grafica-italiana-giocosa.md:109`: le soglie 40 e 25 della soddisfazione.

**Files:**
- Create: `app/src/lib/caricature/caricatura.ts`, `caricatura.test.ts`

**Interfaces:**
- Consumes: `hash` (`app/src/lib/testi/t.ts:58`).
- Produces:

```ts
export type Ruolo = "ministro" | "tecnico" | "leader";
export type Leve = { naso: number; mento: number; fronte: number; nuca: number; occhio: number; colletto: number };
export const VARIANTI: Leve;   // { naso: 6, mento: 3, fronte: 2, nuca: 2, occhio: 2, colletto: 6 }
/** Le leve dall'hash di `ruolo:partito:nome`; `salto` passa alla combinazione successiva. */
export function leveDi(ruolo: Ruolo, partito: string, nome: string, salto?: number): Leve;
/** La rosa: chi ripete naso e colletto di uno già dentro passa alla combinazione successiva, finché è libera. */
export function rosa(candidati: { partito: string; nome: string }[]): Leve[];
/** Il fondo del riquadro dalla soddisfazione: carta, giallo sotto 40, rosso sotto 25. */
export function fondoDi(soddisfazione: number | null): "carta" | "giallo" | "rosso";
```

- Le leve si ricavano dall'hash come nel prototipo (`32-caricature.prototype.html`): lo stesso nome dà la stessa faccia nel prototipo e nell'app.
- Fino alla 11g il genere non c'è: tutte le facce sono disegnate senza i segni femminili (spec:342).
- Le soglie 40 e 25 del fondo sono quelle della spec:341. Il fondo si usa dalla 11c, coi ministri: in 11b i leader hanno il fondo di carta.

- [ ] **Step 1: scrivi i test che falliscono**, in `caricatura.test.ts`:
  - `leveDi` è deterministica, e ogni leva sta sotto la sua variante;
  - cambiare ruolo, partito o nome cambia l'hash: su cento nomi di prova escono almeno cinque nasi e cinque colletti diversi;
  - `leveDi(…, 1)` differisce da `leveDi(…, 0)`;
  - `rosa` di tre candidati con lo stesso naso e lo stesso colletto dà tre coppie (naso, colletto) distinte; una rosa già distinta resta com'è;
  - `rosa([])` è `[]`;
  - `fondoDi(39.9)` è giallo, `fondoDi(40)` carta, `fondoDi(24.9)` rosso, `fondoDi(null)` carta;
  - sui leader della Vista 01 (sette nomi), nessuna coppia ha le sei leve uguali.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): identità delle caricature dall'hash`

---

### Task 15: caricature, kit, disegno e prova di riconoscimento

**Subagente:** `frontend-engineer`. `graphic-designer` resta fuori: il disegno è già scelto, il lavoro è convertire il kit del prototipo in un modulo `.ts` e in un componente Svelte da cablare e provare nel browser, e quel ruolo consegna file senza cablarli.

**Sequenza:** dopo il Task 14; può correre in parallelo col Task 16, che tocca altri file. Il subagente chiude gli Step 1-4 e si ferma; lo Step 5 lo fa la sessione principale con Mario, e il commit dello Step 6 arriva dopo l'esito.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/grafica-italiana-giocosa.md:106`: la lista nera dei tratti-firma, da rileggere a ogni parte del kit.
- `docs/ricerca/estetica/grafica-italiana-giocosa.md:107`: il metodo della prova: la faccia senza nome e la domanda «a chi somiglia?».
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:73`: il kit non contiene acconciature, occhiali né peli, e la prova si fa una volta sul campione.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:50`: il cartellino per il tecnico e la coccarda per il leader.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:53`: il tratto a pennello è un contorno disegnato come forma piena.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:80`: nel tono grave le iniziali nel cerchio d'inchiostro, nello stesso riquadro.
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:258`: il profilo ha il rischio di somiglianza più alto: è il motivo della prova dello Step 5.
- `docs/ricerca/estetica/riferimenti-per-le-facce.md:269`: le leve di posizione restano strette.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:81`: lì la caricatura è decorativa e nascosta ai lettori di schermo. Vale il piano: il nome sta nel `title` e nel nome accessibile.

**Files:**
- Create: `app/src/lib/caricature/kit.ts`, `Caricatura.svelte`; `app/src/routes/sviluppo/caricature/+page.svelte`
- Modify: `app/src/lib/componenti/Faccia.svelte`; `app/src/lib/sviluppo/ProvaComponenti.svelte`

**Interfaces:**
- Consumes: `leveDi`, `fondoDi` (Task 14).
- Produces:
  - `kit.ts`: un poligono precalcolato per ogni parte del kit (i sei nasi, i tre menti, le due fronti, le due nuche, i due occhi, i sei colletti, la coccarda del leader, il cartellino del tecnico), nella tela del prototipo. Il pennino del prototipo è fatto di quadrilateri misurati sul tracciato, circa 15 KB per faccia: qui ogni parte è un tracciato solo (spec:340).
  - `Caricatura.svelte`: props `{ ruolo: Ruolo; partito: string; nome: string; leve?: Leve; misura: 34 | 46 | 62 | 84; fondo?: "carta" | "giallo" | "rosso" }`. Profilo verso destra, tratto d'inchiostro, figura campita di carta, i due inchiostri sovrapposti del ticket 32:34 come token. Il nome sta nel `title` e nel nome accessibile.
  - `Faccia.svelte` guadagna `ruolo` e `partito`: fuori dal tono grave disegna la `Caricatura`, nel tono grave le iniziali. Le schede dei partiti (Task 11 e 12) passano `ruolo="leader"` e mostrano le facce.
- **Lista nera** (spec:343): niente capelli, occhiali, barbe, baffi, collane, fiocchi.

**Prima del dispatch** (chi orchestra): il disegno si porta dal prototipo `32-caricature.prototype.html?f=5`, che Mario ha scelto. Chi esegue converte le parti in poligoni con uno script di sviluppo, che resta fuori dal repo, e confronta a occhio il risultato col prototipo alle quattro misure.

- [ ] **Step 1: genera `kit.ts`** dalle parti del prototipo e misura il peso. Expected: il file sta sotto i 60 KB, e una faccia disegnata pesa meno di 3 KB di SVG.
- [ ] **Step 2: implementa `Caricatura.svelte`** e aggancia `Faccia.svelte`.
- [ ] **Step 3: fai la pagina del campione**, `/sviluppo/caricature`, caricata solo in sviluppo come `/sviluppo/componenti`: i sette leader della Vista 01 e i dieci ministri della Vista 03, alle quattro misure, **senza nomi**, con un numero sotto ciascuna; in fondo, ripiegata, la chiave numero → nome. Poi una rosa di tre e il tono grave.
- [ ] **Step 4: verifica.** Nel browser, accanto al prototipo: le facce si distinguono a 34 px. Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`, poi `grep -rl "sviluppo/caricature" <wt>/app/build`. Expected: PASS, pulito, nessun file.
- [ ] **Step 5: prova di riconoscimento, con Mario.** Chi orchestra si ferma qui: apre `/sviluppo/caricature` nel browser e chiede a Mario se una faccia gli ricorda un politico vero (storia 91, decisione 2). Una faccia riconosciuta cambia leva o si toglie dal kit, e la prova si rifà. L'esito va nella PR 3, col debito della prova a 3-5 persone.
- [ ] **Step 6: commit.** `feat(app): kit delle caricature`

---

### Task 16: Trattativa, risultato e spartizione

**Subagente:** `frontend-engineer`.

**Sequenza:** in parallelo col Task 14 e col Task 15: nessun file in comune.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:60`: i campi della trattativa (`richieste`, `incarico`, `elezioni`, `seggi`) e il comando `ProponiSpartizione`.
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:136`: le tinte dei partiti vanno sempre col nome del partito: vale per i bottoni «a chi».

**Files:**
- Create: `app/src/lib/trattativa/trattativa.ts`, `trattativa.test.ts`, `Trattativa.svelte`, `Risultato.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/lib/cornice/Cornice.svelte`; `app/src/routes/partita/[sezione]/+page.svelte`

**Interfaces:**
- Consumes: `partita.comanda`, `testoRifiuto`, `coalizioneDi`, `nomeDi`, `coloreDi`; `BarraSeggi`, `Evento` (Task 4); `percentuale`, `elenco`.
- Produces:

```ts
/** Peso a pallini di un dicastero: 3 Economia, 2 Interno ed Esteri, 1 gli altri (spec:379). La 11g lo legge dal core. */
export function pesoDi(dicastero: string): 1 | 2 | 3;
/** A chi si può dare un dicastero: il giocatore, il tecnico (null), poi gli alleati di Vista.richieste. */
export function aChi(v: Vista): (string | null)[];
/** La bozza: un'assegnazione per dicastero, `undefined` finché il giocatore non sceglie. */
export type Bozza = Record<string, string | null | undefined>;
export function completa(v: Vista, b: Bozza): boolean;
/** Il comando, solo a bozza completa: una voce per dicastero, nell'ordine di Vista.dicasteri. */
export function proponi(v: Vista, b: Bozza): Comando | null;
/** Il risultato dell'ultimo voto letto per la testa della pagina. */
export function risultato(v: Vista): { affluenza: number; premio: string[] | null; voti: (VotiPartito & { nome: string; miei: boolean })[]; camera: { seggi: number; soglia: number }; senato: { seggi: number; soglia: number } } | null;
/** Il default accanto a «Chiudi il turno», secondo l'incarico. */
export function defaultDellaChiusura(v: Vista): Chiave | null;
```

- `Trattativa.svelte` è la casa del turno in trattativa (ticket 27). Sceglie la pagina dall'incarico:
  - `giocatore`: risultato, spartizione e lato (questo task e il 17);
  - `guida`: il ribaltone; `capo_dello_stato`: la pagina corta (Task 18).

  `fettaDi("trattativa", …)` dà `null`.
- `Risultato.svelte`, in testa (storia 84): affluenza, premio (o «nessun premio»), le quote dei partiti con la coalizione del giocatore in neretto, Camera e Senato a barre col filo rosso della metà più uno e il segno nero sotto i seggi della coalizione, e la frase: «La tua coalizione ha 224 seggi alla Camera e 115 al Senato: la maggioranza in tutte e due», o quanti ne mancano e dove.
- La spartizione (storie 85, 87): un dicastero per riga, nell'ordine di `Vista.dicasteri`, col peso a pallini e i bottoni «a chi»: Tu, Tecnico, poi gli alleati; quello scelto è pieno del colore del partito. La pagina parte senza scelte (lettura 3) e lo dice, con la riga che nomina la 11g per la spartizione di default. A bozza completa parte `proponi_spartizione`; da lì ogni clic la sostituisce. Sotto, per ogni alleato i punti offerti e la richiesta, letti da `Vista.richieste`, e la nota: senza proposta, a fine turno vale quella di default. Il rifiuto del core sta sotto la riga cliccata.
- Un evento aperto sta in una riga in testa, con `Evento` in sola lettura e la frase che in trattativa nessuno risponde e vale il default (spec:379).
- `Cornice.svelte`: in trattativa, accanto a «Chiudi il turno», la riga di `defaultDellaChiusura`: «senza proposta: la spartizione di default», «senza scelta: chiedi la guida».

- [ ] **Step 1: scrivi i test che falliscono**, in `trattativa.test.ts`:
  - `pesoDi` sui dieci dicasteri della Vista 02, nell'ordine di `Vista.dicasteri`, dà `[3, 1, 1, 1, 1, 1, 1, 2, 1, 2]` (i `pesi` di `27-trattativa.json["premio"]`);
  - `aChi(02)` è `["giocatore", null, "centrodestra", "destra"]`;
  - `completa` è falso con nove dicasteri assegnati e vero con dieci, tecnico compreso;
  - `proponi` è `null` a bozza incompleta; a bozza completa ha dieci assegnazioni, e il tecnico è `partito: null`;
  - `risultato(02)`: affluenza 71,4%; premio di tre membri; 224 seggi alla Camera e 115 al Senato per la coalizione; soglie 201 e 101;
  - `risultato(01)` è `null`;
  - `defaultDellaChiusura`: la chiave della spartizione sulla 02, quella della guida su `27-trattativa.json["guida"].vista`, `null` su `["capo_dello_stato"].vista`.
  - `ossatura.test.ts`: `fettaDi("trattativa", "trattativa")` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `27-trattativa.prototype.html`, su `/partita/trattativa?vista=27-trattativa.json&stato=premio`:
  - il risultato in testa con la frase dei seggi;
  - dieci righe coi pesi; assegnati tutti, la proposta parte e i punti degli alleati si aggiornano dove il campione ha la proposta;
  - dare un dicastero a un partito fuori dalla coalizione non si può: il bottone non c'è;
  - l'evento in tono grave sta nella sua riga, senza bottoni.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): trattativa, risultato e spartizione`

---

### Task 17: Trattativa, alleati e allargamento

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 16 (stessi `trattativa.ts`, `trattativa.test.ts` e `Trattativa.svelte`) e dopo il Task 15, che dà il disegno a `Faccia`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:60`: `richieste` per alleato e il comando `Allarga`.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza si legge prima della conferma: `Conferma` dice che la spartizione torna al default.

**Files:**
- Create: `app/src/lib/trattativa/Alleati.svelte`
- Modify: `app/src/lib/trattativa/trattativa.ts`, `trattativa.test.ts`, `Trattativa.svelte`

**Interfaces:**
- Consumes: Task 16; `Faccia` (Task 15), `Caselle`, `Conferma`.
- Produces:

```ts
/** Per alleato: richiesta, offerta e scarto in punti, dalla Vista. */
export function richiesteDi(v: Vista): { id: string; nome: string; richiesta: number; offerta: number; scarto: number }[];
/** La coalizione ha la metà più uno in tutte e due le Camere. */
export function haLaMaggioranza(v: Vista): boolean;
/** I partiti fuori dalla coalizione: seggi, disponibilità e causa. */
export function fuori(v: Vista): { id: string; nome: string; camera: number; senato: number; disponibile: boolean; causa: string }[];
export function allarga(partiti: string[]): Comando;
```

- `Alleati.svelte`, nel dettaglio a lato:
  - in testa l'allargamento (storia 88). Con la maggioranza, una riga: «Puoi anche allargare: … sono disponibili. Chi entra chiede la sua parte». Senza, l'avviso «Senza maggioranza, a fine turno l'incarico passa», i partiti fuori coi seggi, la disponibilità e la causa, «Allarga ›» su ogni disponibile e «Allarga a tutti e due» quando sono più d'uno. Ogni allargamento passa da `Conferma`, che dice che la spartizione torna al default coi nuovi alleati dentro (spec:379);
  - per ogni alleato (storia 86): la faccia del leader a 34 px, «chiede 2,5 punti, ne offri 2», una fila di 14 caselle coi punti offerti e il segno della richiesta. L'intesa di partenza, «entra in ultimatum» e «può uscire» sono un posto vuoto che nomina la 11g;
  - poi il partito del giocatore coi suoi dicasteri e i tecnici, contati dalla bozza, e «Torna alla spartizione di default», che svuota la bozza: il comando che toglie una proposta non c'è, e la riga dice che la proposta fatta resta finché non ne parte un'altra.
- Dopo un allargamento la bozza si svuota: la Vista nuova ha alleati nuovi.

- [ ] **Step 1: scrivi i test che falliscono**, in `trattativa.test.ts`:
  - `richiesteDi(02)`: Centrodestra chiede 2,51 e ha 2; Destra chiede 6,79 e ha 7; gli scarti sono −0,51 e +0,21 a due decimali;
  - `haLaMaggioranza`: vero sulla 02, falso su `27-trattativa.json["senza_maggioranza"].vista`;
  - `fuori` sullo stesso stato: Centrosinistra e Centro disponibili, le estreme e la Sinistra no, con la causa del core;
  - `allarga(["centro"])` è `{ tipo: "allarga", partiti: ["centro"] }`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser** su `&stato=senza_maggioranza`: l'avviso, «Allarga ›» con la conferma sul posto da tastiera, «Allarga a tutti e due»; il tracciato della maggioranza in cornice segue la coalizione dopo l'allargamento. Su `&stato=premio`: la riga «Puoi anche allargare». Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): trattativa, alleati e allargamento`

---

### Task 18: ribaltone e incarico a un'altra coalizione

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 17: stesso `Trattativa.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:60`: `incarico` e i comandi `ChiediGuida` e `RifiutaRibaltone`.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza si legge prima della conferma: vale per «Resta fuori».

**Files:**
- Create: `app/src/lib/trattativa/Ribaltone.svelte`
- Modify: `app/src/lib/trattativa/Trattativa.svelte`

**Interfaces:**
- Consumes: `Risultato`, `Camere` (Task 11), `Conferma`, `partita.comanda`, `testoRifiuto`.
- Produces: le due pagine che `Trattativa.svelte` monta secondo `Vista.incarico`.

- `guida` (storia 89): la pagina dice che il governo è caduto e che il giocatore sta nella nuova maggioranza. «Chiedi la guida ›» è diretto (`{ tipo: "chiedi_guida" }`); «Resta fuori» passa da `Conferma` (`{ tipo: "rifiuta_ribaltone" }`). Ceduta la guida, la Vista nuova ha l'incarico al giocatore e la pagina diventa la spartizione; negata, il rifiuto del core sta sotto i bottoni. I nomi dei partiti del ribaltone e del primo per seggi sono un posto vuoto che nomina la 11g (lettura 3). Nel lato, le Camere.
- `capo_dello_stato` (storia 90): una pagina corta, «L'incarico va a un'altra coalizione», col risultato e le Camere a lato.

- [ ] **Step 1: implementa le due pagine.** La logica nuova è la scelta della pagina dall'incarico, già provata dal Task 16 con `defaultDellaChiusura`.
- [ ] **Step 2: verifica nel browser**, su `/partita/trattativa?vista=27-trattativa.json`:
  - `&stato=guida`: «Chiedi la guida ›» porta alla spartizione (il campione ha `dopo_chiedi`); «Resta fuori» si conferma sul posto, da tastiera; due Invio veloci fanno una chiamata sola;
  - `&stato=capo_dello_stato`: la pagina corta, senza bottoni d'azione;
  - `?vista=02-trattativa-incarico.json`: il tono grave, con le iniziali al posto delle facce.

  Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 3: commit.** `feat(app): ribaltone e incarico a un'altra coalizione`

---

### Task 19: verifica della fetta

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 18. Chiude la PR 3 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 6).

**Ricerca:**
- `docs/ricerca/estetica/mappa-svg-delle-regioni.md:145`: resta da provare che nella WKWebView il Tab entri nella mappa: è una voce del giro nell'app vera.
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md:73`: la prova di riconoscimento a 3-5 persone: è il debito da scrivere nella PR 3.

Lo esegue la sessione principale. Ogni esito va nella PR 3.

- [ ] **Step 1: batterie nei due motori** sugli stati della trattativa (`27-trattativa.json` nei quattro stati, più la 02) e, di nuovo, su tutte le pagine dei Task 8 e 13: ora le facce sono disegnate. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: i bottoni «a chi», l'allargamento con la conferma, «Chiedi la guida», «Resta fuori».
- [ ] **Step 3: test e tipi**, come al Task 8, passo 3, più `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`.
- [ ] **Step 4: giro nell'app vera**, col telecomando (`app/verifica/README.md:63-98`). `computer-use` non vede l'app di sviluppo: il giro la pilota da dentro, col guscio vero. `npm --prefix <wt>/app run verifica:giro` va lanciato per primo e in background: la prima compilazione dura una dozzina di minuti. Il percorso, dall'edicola:
  1. «Nuova partita ›», Italia di oggi, un nome e un posto, «Fonda il partito ›»;
  2. cruscotto del turno zero, poi «Campagna ›»;
  3. dichiarare due partiti disponibili; una cessione rifiutata, col rifiuto sulla riga;
  4. una spesa su una condizione e una su una regione, con le fasce dell'anteprima;
  5. comprare il sondaggio privato: la cassa scende, le quote private compaiono in Campagna e in Paese;
  6. un riposizionamento confermato; una promessa confermata;
  7. Paese: una regione, un gruppo, i tre elettori;
  8. Partiti: una scheda;
  9. «Chiudi il turno»: si vota e si apre la Trattativa col risultato;
  10. assegnare i dieci dicasteri, un tecnico compreso; «Chiudi il turno»: il governo nasce.

  Poi i tagli nell'app vera su campagna, paese, partiti e trattativa (`?tagli=1`), a scala 1 e 1,25. A fine giro si rimettono le impostazioni, si cancella la cartella di verifica e si ripristina `src-tauri/Cargo.toml` (`README.md:94-98`).
- [ ] **Step 5: screenshot** della trattativa nei suoi stati e del campione delle caricature, in `docs/superpowers/verifiche/11b/`.
- [ ] **Step 6: corpo della PR 3.** Oltre alle sezioni solite:
  - l'esito della prova di riconoscimento e il debito della prova a 3-5 persone;
  - **i rimandi al core trovati dalla fetta**: il campo che dice se il giocatore guida il governo uscente (decisione 1) e ogni altro dato mancante incontrato;
  - ciò che resta da provare a mano a Mario: il Tab reale, il puntatore sulla mappa, Cmd+Q a campagna in corso, la resa a occhio delle facce nell'app.
- [ ] **Step 7: commit dei ritocchi della verifica**, se ce ne sono. `fix(app): ritocchi dalla verifica della 11b`

**Fine della PR 3 e della fetta.**
