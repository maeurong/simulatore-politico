# Piano 11a «Avvio»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** l'app si apre sull'edicola. Dall'edicola il giocatore sceglie o compone lo scenario, fonda il partito ed entra nel turno zero, dentro la cornice coi tracciati e il sommario. Sotto ci sono le fondamenta che useranno le fette 11b-11g.

**Architettura:**
- La UI Svelte 5 legge la Vista generata da ts-rs attraverso un modulo di chiamate tipizzato sopra `invoke`, che è la giuntura di test principale.
- In sviluppo, nel browser, un modulo `mockIPC` serve le Viste campione al posto del guscio.
- Il guscio guadagna tre passacarte e gli errori in due specie.
- Il core guadagna solo `catalogo_scenario`, una funzione di sola lettura.
- La logica pura (scale dei tracciati, `t`, stato del giocatore, impostazioni, data relativa) sta in moduli `.ts` senza DOM, provati con Vitest.

**Tecnologie:** Svelte 5 (runes) e SvelteKit con `adapter-static` in modalità SPA, TypeScript, Vite 8, Tauri 2.12, Rust (`sim-core`, `app/src-tauri`), ts-rs. Una sola dipendenza nuova: `vitest`.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 1-59 e 174-181, le decisioni alle righe 248-371 e i test alle righe 455-476. Chi esegue legge la spec e i ticket citati nei task.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `08-fondamenta.prototype.html`: token, cornice e componenti, forma A;
- `07-ossatura.prototype.html`: sommario, barrate e motivi, forma A;
- `13-edicola.prototype.html`, `14-editor.prototype.html` e `15-fondazione.prototype.html`: forma A · Banco;
- `31-tracciati.prototype.html`: scale. Dove il prototipo ha `cons:'premio'`, vince il ticket 31: il rosso del consenso viene dalla proiezione;
- `33-impostazioni.prototype.html`: forma B · Lato.

I testi dell'interfaccia si copiano dai prototipi. Un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR.

## Vincoli globali

- **La UI non calcola regole** (spec:250). Può:
  - contare voci aperte della Vista;
  - sommare quote pubbliche del sondaggio;
  - ordinare per campi.
- **Dato mancante = posto vuoto che nomina la fetta che lo riempie** (spec:251).
- **`sim-core` e la Vista non si toccano, salvo `catalogo_scenario`** (spec:252).
- **Il guscio aggiunge solo passacarte**, salvo il seme della fondazione (spec:253).
- **Finestra minima ~1280×800**, progetto a ~1440×900 (spec:254).
- **Errori:** `{ specie: "rifiuto" | "guasto"; testo: string }` (spec:262). Il rifiuto si mostra accanto alla scelta che l'ha causato; il guasto sta nella cornice (fuori dalla partita, in testa alla pagina) e dice che la partita è rimasta com'era.
- **CSP:** `default-src 'self'; style-src 'self' 'unsafe-inline'` (spec:270).
- **Caratteri dentro l'app**: nessuna richiesta a Google Fonts (spec:296).
- **Nessun colore né misura scritti a mano** fuori dai token (spec:294).
- **Testo mai sotto 12 px a 1×** (spec:296).
- **Si animano solo `transform` e `opacity`** (spec:326).
- **Numeri** con `Intl.NumberFormat("it-IT", { useGrouping: "always" })`; il meno è il segno meno (U+2212); la UI arrotonda (spec:316).
- **Nessuna finestra modale**: ciò che distrugge diventa due pulsanti sul posto (spec:451-453).
- **Cargo da un worktree**: ogni comando cargo gira con `--manifest-path <worktree assoluto>/Cargo.toml` e con `TS_RS_EXPORT_DIR=<worktree assoluto>/app/src/lib/tipi`. Senza, i tipi finiscono nel checkout sbagliato.
- **I comandi `npm`** usano `--prefix <worktree assoluto>/app`.
- **Commit** in Conventional Commits, con `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Letture che la spec non fissa, confermate da Mario il 12/11/2026

1. **Tracciati in trattativa**: gli stessi del governo (consenso della coalizione, maggioranza sulle Camere elette, spread), con la coalizione letta da `Vista.coalizione` più il giocatore. La spec non li fissa per questo stato (spec:302-309).
2. **«N scelte al default» sotto «Chiudi il turno»** resta nascosto in 11a. Il conto nasce con le righe del cruscotto, in 11b per il turno zero e in 11c al governo.
3. **Tracciato «per farlo cadere»** all'opposizione: caselle tratteggiate col rimando a 11e, perché gli serve `anteprima_sfiducia`.
4. **Stacco e rosso dello spread** assenti fino a 11g, perché la Vista non porta la soglia delle Aste scoperte (spec:311). Il tracciato resta grigio-inchiostro, senza rosso.

## Punti da sorvegliare

I casi che nessun test dei task copre e che più facilmente rompono il gioco. Ogni riga ha il suo controllo nel task indicato.

1. **CSP nell'app vera.** SvelteKit mette script inline in `index.html`. Tauri ne aggiunge gli hash da sé solo in build, mentre in `tauri dev` la CSP vale sulla pagina di Vite. Se in dev la pagina resta bianca o l'HMR non si collega, si aggiunge `devCsp` in `tauri.conf.json` con `connect-src 'self' ws://localhost:1420`; la CSP di rilascio non cambia. Controllo nel Task 15, passo 5.
2. **Scala nell'app vera.** In WKWebView `zoom` sulla radice va verificato per tre cose:
   - le coordinate del puntatore sul trascinamento dei cursori dell'editor;
   - le media query;
   - `window.innerWidth`, che deve restare in pixel non scalati.

   Se `innerWidth` scala, la scala applicata si calcola su `document.documentElement.clientWidth * zoom`. Controllo nel Task 15, passo 5.
3. **Corsa dei riepiloghi nell'editor.** Durante un trascinamento partono molte richieste: ne resta in volo una per volta, l'ultima vince, e una risposta vecchia non sovrascrive mai una nuova. Test nel Task 13.
4. **Doppio clic su «Fonda il partito» o «Riprendi».** Il secondo clic non fonda una seconda partita: il pulsante resta spento mentre la chiamata è in volo. Controllo nel Task 14 e nel Task 12.
5. **Impostazioni illeggibili.** Se `localStorage` contiene JSON rotto, una versione ignota o un valore fuori dai gradini, valgono i valori di partenza senza eccezioni. Se `localStorage` stesso lancia un'eccezione (deposito bloccato), l'app parte lo stesso. Test nel Task 8.

## Struttura dei file

```
crates/sim-core/src/riepilogo.rs           + catalogo(), CatalogoScenario e tipi figli
crates/sim-core/tests/riepilogo.rs         + test del catalogo
.cargo/config.toml                         + TS_RS_LARGE_INT
app/src-tauri/src/errore.rs                nuovo: ErroreGuscio, rifiuto(), guasto()
app/src-tauri/src/lib.rs                   errori in due specie, passacarte, seme, via nuova_partita
app/src-tauri/src/archivio.rs              salva_scenario distingue rifiuto e guasto
app/src-tauri/tauri.conf.json              CSP, finestra 1440×900 con minimo 1280×800
app/package.json, app/vite.config.js       vitest, script "test", server.fs.allow
app/static/fonts/                          Fraunces e Atkinson Hyperlegible Next, con OFL.txt
app/src/lib/guscio/chiamate.ts             una funzione per comando, ErroreGuscio
app/src/lib/guscio/finto.ts                mockIPC di sviluppo sulle Viste campione
app/src/lib/testi/interfaccia.json         testi neutri
app/src/lib/testi/t.ts                     t(), numero(), percentuale(), dataRelativa()
app/src/lib/stile/fondamenta.css           token e regole di base
app/src/lib/impostazioni.svelte.ts         scala, velocità, movimento; localStorage
app/src/lib/tracciati.ts                   scale e valori dei tracciati dalla Vista
app/src/lib/ossatura.ts                    stato del giocatore, sezioni, barrate, casa
app/src/lib/partita.svelte.ts              Vista, serie e guasto della partita in corso; azioni
app/src/lib/sviluppo/tagli.ts              contatore dei tagli (?tagli=1, solo sviluppo)
app/src/lib/componenti/*.svelte            Pulsante, Conferma, NumeroFreccia, Tracciato, Dettaglio, Rifiuto, Segnaposto
app/src/lib/cornice/*.svelte               Cornice, Sommario, Impostazioni (lato)
app/src/lib/edicola/*.svelte               RigaPartita, DettaglioPartita, DettaglioAlbo, Grafico
app/src/lib/editor/*.svelte                Cursore, LeggeElettorale, Disposizioni, Riepilogo
app/src/lib/fondazione/*.svelte            Asse, TabellaPosti, DettaglioPosto
app/src/routes/+layout.ts / +layout.svelte scala prima del primo disegno, scorciatoie, mock
app/src/routes/+page.svelte                Edicola
app/src/routes/scenario/+page.svelte       Scenario ed editor
app/src/routes/fondazione/+page.svelte     Fondazione
app/src/routes/partita/[sezione]/+page.svelte  cornice, sommario, sezione o segnaposto
app/src/lib/*.test.ts                      Vitest accanto al modulo che provano
```

I test Vitest stanno accanto al modulo: `t.test.ts`, `tracciati.test.ts` e così via.

## PR e ordine

- **PR 1, core e guscio:** Task 1-3. Rust, `rust-engineer`.
- **PR 2, fondamenta:** Task 4-11, `frontend-engineer`.
- **PR 3, schermate:** Task 12-15, `frontend-engineer`.

Tutti i task sono in sequenza. Fanno eccezione il 6 e il 7, che dopo il 4 possono andare in parallelo: toccano file diversi.

Prima del Task 1 le Viste campione si rigenerano su main (spec:474), col comando in `.scratch/v1-piano-11-ui/viste/README.md:14` lanciato da un checkout di main aggiornato. Se un `shasum` cambia, i file nuovi entrano nella PR 1.

Il round pre-commit di ogni PR:
- `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer`, in parallelo;
- `rust-reviewer` nella PR 1;
- per le PR 2 e 3, `craft-reviewer` riceve gli screenshot del Task 15.

---

### Task 1: `catalogo_scenario` in `sim-core`

**Files:**
- Modify: `crates/sim-core/src/riepilogo.rs`, accanto a `riepilogo` (`:118`)
- Test: `crates/sim-core/tests/riepilogo.rs`
- Rigenerati: `app/src/lib/tipi/` (cinque tipi nuovi)

**Interfaces:**
- Produces: `pub fn catalogo(dati: &DatiGioco) -> Result<CatalogoScenario, Errore>`.

```rust
#[derive(Clone, Debug, PartialEq, Serialize)]
#[cfg_attr(test, derive(TS), ts(export))]
pub struct CatalogoScenario {
    /// I sei cursori, in quest'ordine: debito_pil, disoccupazione, inflazione, ricchezza, eta_mediana, orientamento.
    pub cursori: Vec<CursoreCatalogo>,
    /// Nome dell'impostazione di elezioni.toml -> (id della disposizione elettorale -> id del gradino).
    pub leggi_elettorali: BTreeMap<String, BTreeMap<String, String>>,
    pub legge_di_base: String,
    /// Tutte le disposizioni dei dati, nel loro ordine.
    pub disposizioni: Vec<VoceCatalogo>,
    /// I partiti dello scenario, nel loro ordine: la fondazione li mette sotto l'asse.
    pub partiti: Vec<PartitoCatalogo>,
}
pub struct CursoreCatalogo { pub nome: String, pub valore: f64, pub min: f64, pub max: f64 }
pub struct VoceCatalogo {
    pub id: String, pub nome: String,
    /// Nome del dicastero; None per costituzionali ed elettorali.
    pub dicastero: Option<String>,
    pub norma: Norma, pub partenza: String, pub gradini: Vec<GradinoCatalogo>,
}
pub struct GradinoCatalogo { pub id: String, pub nome: String, pub saldo: f64 }
pub struct PartitoCatalogo { pub id: String, pub nome: String, pub posizione: f64 }
```

Ogni tipo deriva gli stessi attributi di `CatalogoScenario`. La forma ricalca `editor()` del generatore (`.scratch/v1-piano-11-ui/viste/genera/src/main.rs:314-345`), così il mock del Task 5 passa `14-editor.json` com'è. Il campo `unita` del generatore è testo dell'interfaccia e resta fuori dal core.

- [ ] **Step 1: scrivi i test che falliscono**, in `tests/riepilogo.rs`. Gli oracoli sono i valori di `.scratch/v1-piano-11-ui/viste/14-editor.json`, generati su main con gli stessi dati.

```rust
#[test]
fn il_catalogo_della_base() {
    let c = catalogo(&dati_base()).unwrap();
    let nomi: Vec<_> = c.cursori.iter().map(|x| x.nome.as_str()).collect();
    assert_eq!(nomi, ["debito_pil", "disoccupazione", "inflazione", "ricchezza", "eta_mediana", "orientamento"]);
    assert_eq!((c.cursori[0].valore, c.cursori[0].min, c.cursori[0].max), (136.7, 110.0, 160.0));
    let leggi: Vec<_> = c.leggi_elettorali.keys().map(String::as_str).collect();
    assert_eq!(leggi, ["legge_180", "proporzionale_3", "proporzionale_puro"]);
    assert_eq!(c.leggi_elettorali["legge_180"].len(), 9);
    assert_eq!(c.legge_di_base, "legge_180");
    assert_eq!(c.disposizioni.len(), 46);
    assert_eq!(c.disposizioni.iter().filter(|d| d.norma == Norma::Ordinaria).count(), 35);
    assert_eq!(c.partiti.len(), 7);
    assert_eq!((c.partiti[0].id.as_str(), c.partiti[0].posizione), ("estrema_sinistra", 1.0));
}

#[test]
fn i_gradini_di_ogni_legge_coincidono_coi_dati_che_ne_fa_il_core() {
    // Per ogni legge: con_su_misura con quella legge, poi le partenze delle disposizioni elettorali.
    // Stesso calcolo di genera/src/main.rs:342-352, rifatto nel test come oracolo indipendente.
}
```

Prima di fissare un oracolo, confrontalo con `14-editor.json`: `legge_di_base`, il conteggio delle disposizioni e i primi valori. Se il file dice altro, vale il file.

- [ ] **Step 2: esegui i test e verifica che falliscano.**
  Run: `cargo test --manifest-path <wt>/Cargo.toml -p sim-core --test riepilogo catalogo`
  Expected: errore di compilazione, `catalogo` non c'è.

- [ ] **Step 3: implementa `catalogo`.** Legge da:
  - `dati.scenario()`: gli ingressi `debito_pil`, `disoccupazione` e `inflazione`, i cursori sintetici, la legge elettorale e i partiti;
  - `dati.elezioni().impostazioni`, per le chiavi in ordine;
  - `dati.disposizioni()`.

  Il nome del dicastero arriva da `dati.dicasteri()`. I gradini elettorali di ogni legge vengono da `dati.con_su_misura(&s)`, dove `s` è `SuMisura::base(dati)` con la legge cambiata; l'errore si propaga.

- [ ] **Step 4: esegui i test e verifica che passino**, poi rigenera i tipi.
  Run: `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml -p sim-core`
  Expected: PASS, e in `app/src/lib/tipi/` compaiono `CatalogoScenario.ts`, `CursoreCatalogo.ts`, `VoceCatalogo.ts`, `GradinoCatalogo.ts` e `PartitoCatalogo.ts`.

- [ ] **Step 5: commit.** `feat(core): catalogo dello scenario per l'editor`

---

### Task 2: errori in due specie e interi grandi a `number`

**Files:**
- Create: `app/src-tauri/src/errore.rs`
- Modify:
  - `app/src-tauri/src/lib.rs`: tutti i comandi `:230-397`, gli helper `:34-227` e i test `:444-823`;
  - `app/src-tauri/src/archivio.rs`: `salva_scenario` `:468-481`;
  - `.cargo/config.toml`;
  - `app/src-tauri/tauri.conf.json`.
- Rigenerati: `app/src/lib/tipi/`, i file che oggi hanno `bigint`.

**Interfaces:**
- Produces:

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErroreGuscio { pub specie: Specie, pub testo: String }
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Specie { Rifiuto, Guasto }
pub fn rifiuto(e: impl ToString) -> ErroreGuscio;
pub fn guasto(e: impl ToString) -> ErroreGuscio;
impl From<String> for ErroreGuscio  // -> guasto
impl From<&str> for ErroreGuscio    // -> guasto
impl From<ErroreGuscio> for String  // il testo, per i test che tornano Result<(), String>
```

Ogni comando torna `Result<T, ErroreGuscio>`. Con `From<String>`, gli errori dell'archivio (disco) e del lock diventano guasti con un `?`; ogni `sim_core::Errore` passa invece da `.map_err(rifiuto)` scritto nel punto.

**Classificazione** (ticket 12:26):
- **Rifiuti:**
  - ogni `sim_core::Errore`: da `comanda`, `fonda`, `con_su_misura` (in `riepilogo_scenario`, `fonda_partita` e `salva_scenario`), le anteprime e `abbandona` del core;
  - `SENZA_SEGGI` (`:31`) e «nessuna eredità in bozza da confermare» (`:198`);
  - in `salva_scenario`: il nome non valido (`percorso_scenario`) e lo scenario che differisce solo per le maiuscole (`archivio.rs:471-479`).
- **Guasti:** `ERRORE_STATO`, `NESSUNA`, ogni scrittura o lettura del disco, «nessuna voce d'albo» (`:170`) e `carica_scenario`.

`salva_scenario` dell'archivio torna `Result<(), ErroreGuscio>` e marca i tre casi da sé; `scrivi` resta un guasto.

- [ ] **Step 1: scrivi i test che falliscono**, in `lib.rs` `mod tests`:
  - `rifiuto_del_core_su_comanda`: `esegui` con `campagna(4)` (quattro spese, oltre le tre della storia 64 della spec) dà `Specie::Rifiuto`, e la partita in memoria resta com'era (`vista()` uguale a prima). Se il core accetta quattro spese, usa un comando che rifiuta di sicuro e scrivi nel test quale e perché.
  - `senza_partita_e_un_guasto`: `chiudi` su `Stato::default()` dà `Err(guasto(NESSUNA))`.
  - `lock_avvelenato_e_un_guasto`: adatta `lock_avvelenato_ripulito_da_una_partita_nuova` (`:491`). L'assert diventa `Err(guasto(ERRORE_STATO))`, e `avvia` lascia il posto a `fonda` con `SEME + 1` (vedi lo step 3).
  - `nome_di_scenario_non_valido_e_un_rifiuto`: `archivio.salva_scenario` con nome `""` dà `Specie::Rifiuto`.

  Gli altri test esistenti si adattano solo nel tipo d'errore.

- [ ] **Step 2: esegui i test e verifica che falliscano.**
  Run: `cargo test --manifest-path <wt>/Cargo.toml -p app`
  Expected: FAIL in compilazione su `ErroreGuscio`. Il nome del package si legge in `app/src-tauri/Cargo.toml`.

- [ ] **Step 3: implementa.**
  - `errore.rs`, poi i comandi.
  - **`nuova_partita` esce** dal codice e da `generate_handler!` (`:413`). Esce anche `avvia` (`:71-77`), che resta senza chiamanti. `InCorso.id` resta `Option`: la semplificazione non è di questa fetta.
  - `.cargo/config.toml`: `TS_RS_LARGE_INT = "number"` accanto a `TS_RS_EXPORT_DIR`.
  - `tauri.conf.json`:
    - `"csp": "default-src 'self'; style-src 'self' 'unsafe-inline'"`;
    - finestra `"width": 1440, "height": 900, "minWidth": 1280, "minHeight": 800`.

- [ ] **Step 4: esegui i test, rigenera i tipi e controlla.**
  Run, uno per chiamata:
  - `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml --workspace`
  - `grep -l bigint <wt>/app/src/lib/tipi/*.ts`
  - `npm --prefix <wt>/app run check`

  Expected: tutti i test passano, `grep` non trova niente e il check esce pulito. Se il check trova `invoke("nuova_partita")` nella pagina di oggi, la pagina si sostituisce nel Task 10: per ora basta che compili, e la chiamata a `nuova_partita` si toglie.

- [ ] **Step 5: commit.** `feat(app): errori del guscio in due specie, CSP accesa`

---

### Task 3: passacarte `serie`, `anteprima_fondazione`, `catalogo_scenario` e seme della fondazione

**Files:**
- Modify: `app/src-tauri/src/lib.rs`

**Interfaces:**
- Consumes: `catalogo` (Task 1); `Partita::serie` (`crates/sim-core/src/partita.rs:2603`); `Partita::anteprima_fondazione(dati, seme)` (`crates/sim-core/src/legislatura/fondazione.rs:142`).
- Produces, i comandi:
  - `serie() -> Result<Vec<PuntoSerie>, ErroreGuscio>`: della partita in corso; guasto senza partita.
  - `anteprima_fondazione(scenario: Option<SuMisura>) -> Result<AnteprimaFondazione, ErroreGuscio>`: sceglie il seme dall'orologio, lo tiene e lo restituisce coi dati. L'errore di `con_su_misura` è un rifiuto.
  - `catalogo_scenario() -> Result<CatalogoScenario, ErroreGuscio>`: sui dati di base. L'errore di `catalogo` è un rifiuto.
- Stato nuovo, gestito: `type SemeFondazione = Mutex<Option<u64>>`.
  - `anteprima_fondazione` lo sovrascrive.
  - `fonda_partita` lo consuma con `take()`; senza anteprima prima, lo prende dall'orologio (oggi `:264`).

- [ ] **Step 1: scrivi i test che falliscono.**
  - `la_fondazione_riusa_il_seme_dell_anteprima`: un `SemeFondazione` con `Some(SEME)`; la funzione interna che `fonda_partita` chiama lo consuma; la partita fondata ha `id` `format!("{SEME:016x}")` (cioè `ID`) e il seme torna `None`.
  - `l_anteprima_mostra_il_sondaggio_della_partita`: l'anteprima col seme `s` e la Vista della partita fondata con lo stesso `s` alla `posizione` scelta hanno la stessa `quota_attesa` della `VocePosizione` e la stessa quota del giocatore nel sondaggio della Vista, calcolata come `quota_fra_i_votanti` (media sugli istituti di `quote[j] / (1 − astensione)`, `crates/sim-core/src/legislatura/fondazione.rs:87-93`), entro 1e-12. È la storia 28. Se le due quote non coincidono, fermati e segnalalo: è un difetto del core, e non si aggiusta nel guscio.
  - `serie_senza_partita_e_un_guasto`.

  Fattorizza le funzioni interne `seme_per_fondare(&SemeFondazione) -> Result<u64, ErroreGuscio>` e `anteprima(&DatiGioco, Option<&SuMisura>, &SemeFondazione)`, come `fonda` e `chiudi`. I comandi restano involucri di una riga.

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `cargo test --manifest-path <wt>/Cargo.toml -p app`. Expected: FAIL.

- [ ] **Step 3: implementa i tre comandi** e registrali in `generate_handler!`.

- [ ] **Step 4: esegui i test e verifica che passino.** Run: come allo step 2. Expected: PASS.

- [ ] **Step 5: commit.** `feat(app): passacarte di serie, anteprima della fondazione e catalogo`

**Fine della PR 1.**

---

### Task 4: Vitest e modulo delle chiamate

**Files:**
- Modify: `app/package.json` (`"test": "TZ=Europe/Rome vitest run"`, devDependency `vitest`), `app/vite.config.js` (`test: { include: ["src/**/*.test.ts"] }`; `server.fs.allow: [".."]`, che serve al Task 5 per leggere `.scratch/` dalla radice del repo).
- Create: `app/src/lib/guscio/chiamate.ts`, `app/src/lib/guscio/chiamate.test.ts`

**Interfaces:**
- Produces:

```ts
export type ErroreGuscio = { specie: "rifiuto" | "guasto"; testo: string };
export function comeErrore(e: unknown): ErroreGuscio; // forma giusta → com'è; altro → guasto con String(e)
// Una funzione per comando del guscio, coi tipi di $lib/tipi; ognuna rilancia ErroreGuscio.
export const guscio: {
  chiudiTurno(): Promise<Vista>;
  partite(): Promise<VocePartita[]>;
  fondaPartita(scenario: SuMisura | null, fondazione: Fondazione): Promise<Vista>;
  riprendi(id: string): Promise<Vista>;
  comanda(comando: Comando): Promise<Vista>;
  riavvolgi(turno: number): Promise<Vista>;
  abbandona(): Promise<Eredita>;
  confermaEredita(): Promise<void>;
  albo(): Promise<VoceAlbo[]>;
  scenari(): Promise<string[]>;
  salvaScenario(scenario: SuMisura): Promise<void>;
  caricaScenario(nome: string): Promise<SuMisura>;
  riepilogoScenario(scenario: SuMisura | null): Promise<RiepilogoScenario>;
  anteprima(valori: ValoreProposto[]): Promise<Anteprima>;
  anteprimaDecreto(decreto: Decreto): Promise<Anteprima>;
  anteprimaAccogli(dicastero: string): Promise<Anteprima>;
  serie(): Promise<PuntoSerie[]>;
  anteprimaFondazione(scenario: SuMisura | null): Promise<AnteprimaFondazione>;
  catalogoScenario(): Promise<CatalogoScenario>;
};
```

Gli argomenti vanno a `invoke` coi nomi dei parametri Rust (`scenario`, `fondazione`, `id`, `comando`, `turno`, `nome`, `valori`, `decreto`, `dicastero`). Tauri li vuole in camelCase: qui coincidono.

- [ ] **Step 1: scrivi i test che falliscono**, in `chiamate.test.ts`, con `mockIPC` di `@tauri-apps/api/mocks` e `clearMocks` in `afterEach`:
  - `comeErrore` lascia com'è `{specie:"rifiuto",testo:"x"}`; trasforma `"disco pieno"` in `{specie:"guasto",testo:"disco pieno"}`; trasforma `new Error("y")` in un guasto con testo `"Error: y"`.
  - `guscio.riprendi("abc")` chiama `riprendi` con `{ id: "abc" }`: il gestore del mock registra comando e argomenti.
  - Un rifiuto lanciato dal mock arriva al chiamante come `ErroreGuscio` con `specie: "rifiuto"`.

- [ ] **Step 2: esegui i test e verifica che falliscano.**
  Run: `npm --prefix <wt>/app install -D vitest`, poi `npm --prefix <wt>/app test`.
  Expected: FAIL, il modulo non c'è.

- [ ] **Step 3: implementa `chiamate.ts`.** Una funzione privata `chiama<T>(comando, args?)` fa `invoke` e in `catch` rilancia `comeErrore(e)`.

- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, check pulito.

- [ ] **Step 5: commit.** `feat(app): modulo delle chiamate al guscio e Vitest`

---

### Task 5: guscio finto sulle Viste campione

**Files:**
- Create: `app/src/lib/guscio/finto.ts`, `app/src/lib/guscio/finto.test.ts`

**Interfaces:**
- Consumes: `ErroreGuscio` (Task 4).
- Produces:

```ts
/** La risposta finta di un comando, senza stato: `campione` è il JSON scelto con ?vista e ?stato. */
export function rispondi(
  comando: string, args: Record<string, unknown>,
  c: { campione: unknown; edicola: unknown; editor: unknown; fondazione: unknown },
): unknown; // lancia ErroreGuscio {specie:"rifiuto", testo:"non nel campione: <comando>"} se non sa rispondere
export function vistaDi(campione: unknown): Vista | null;
/** Solo se import.meta.env.DEV e fuori da Tauri: legge ?vista e ?stato, carica i JSON con
 *  import.meta.glob (lazy) da ../../../../.scratch/v1-piano-11-ui/viste/, installa mockIPC.
 *  Ritorna la Vista del campione, o null. */
export async function installaFinto(url: URL): Promise<Vista | null>;
```

**Regole di `rispondi`** (ticket 34:23):
- Il campione è il file; con `?stato=k` diventa `file[k]` (`k` numerico per i file che sono un elenco, come `25-registro.json`).
- `vistaDi`: il campione se ha `turno`; altrimenti `.vista`, poi `.prima`.
- `chiudi_turno` risponde con `.dopo`; `comanda` con `.dopo_comando`; `serie` con `.serie ?? []`; `riprendi` con la Vista del campione.
- `partite` e `albo` vengono da `13-edicola.json`; `scenari` da `14-editor.json` (`scenari_salvati`).
- `carica_scenario(nome)`: lo `scenario` di `riepiloghi[]` che ha quel nome.
- `catalogo_scenario`: `14-editor.json` coi campi del Task 1, più `partiti` presi da `15-fondazione.json`.
- `riepilogo_scenario(s)`:
  - `null` dà la base;
  - uno scenario uguale (confronto profondo) a uno di `riepiloghi[].scenario` dà il suo `riepilogo`, oppure il suo `errore` come rifiuto;
  - se differisce dalla base in un cursore solo, dà il passo più vicino della `griglia`;
  - altrimenti, rifiuto.
- `anteprima_fondazione(s)`: base → `anteprime["oggi, seme 4"]`; `orientamento` 1 → `anteprime["elettorato a destra, seme 4"]`.
- `fonda_partita` dà `vistaDi` di `01-turno-zero.json`.
- `salva_scenario` dà `null`.
- Il resto, rifiuto.

**Fuori da Tauri** vuol dire `!("__TAURI_INTERNALS__" in window)`. In `tauri dev` il mock non si installa mai.

- [ ] **Step 1: scrivi i test che falliscono**, sui JSON veri, letti con `fs` in Node:
  - `vistaDi(01)` ha `turno` 0.
  - `vistaDi(17-cruscotto.json["ultimatum"])` non è `null`.
  - `rispondi("chiudi_turno", {}, {campione: 25[0]})` è la Vista con `turno` 1.
  - `rispondi("riepilogo_scenario", {scenario: null}, …)` ha `spread_pb` uguale a quello di `riepiloghi[0]` («base»).
  - `rispondi("riepilogo_scenario", {scenario: <incoerente>}, …)` lancia un rifiuto il cui testo è l'`errore` del campione.
  - `rispondi("anteprima_sfiducia", {}, …)` lancia `{specie:"rifiuto"}` con «non nel campione».

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.

- [ ] **Step 3: implementa.** `installaFinto` si chiama da `+layout.ts` (Task 10), dentro `if (import.meta.env.DEV)`, con `import()` dinamico, così il modulo non entra nel rilascio.

- [ ] **Step 4: verifica.**
  - Run: `npm --prefix <wt>/app test`. Expected: PASS.
  - Run: `npm --prefix <wt>/app run build`, poi `grep -rl "non nel campione" <wt>/app/build`. Expected: nessun file. Il mock non è nel rilascio.

- [ ] **Step 5: commit.** `feat(app): guscio finto sulle Viste campione, solo in sviluppo`

---

### Task 6: testi, `t()` e numeri

**Files:**
- Create: `app/src/lib/testi/interfaccia.json`, `app/src/lib/testi/t.ts`, `app/src/lib/testi/t.test.ts`

**Interfaces:**
- Produces:

```ts
export type Chiave = keyof typeof interfaccia;
/** {nome} sostituiti; un valore oggetto {one, other} sceglie con Intl.PluralRules("it") su {n}; "many" ricade su "other". */
export function t(chiave: Chiave, valori?: Record<string, string | number>): string;
export function numero(x: number, decimali?: number): string;     // it-IT, useGrouping "always", meno U+2212
export function percentuale(quota: number, decimali?: number): string; // 0,192 → "19,2%"
export function segnato(x: number, decimali?: number): string;    // "+3", "−4", "0"
/** «oggi alle 22:20», «ieri alle …», giorno della settimana fino a sei giorni, poi «il 28 ottobre alle …» (13-edicola.prototype.html:209-215). */
export function dataRelativa(secondiUnix: number, ora: Date): string;
export function hash(testo: string): number; // FNV-1a a 32 bit, intero ≥ 0
export function variante(n: number, ...parti: (string | number)[]): number; // hash(parti.join("|")) % n
```

`interfaccia.json` parte con i testi delle schermate di 11a, copiati dai prototipi 07, 08, 13, 14, 15 e 33. I testi del Pungolo (`pungolo.json`) non servono in 11a.

- [ ] **Step 1: scrivi i test che falliscono.**
  - `numero(1234)` è `"1.234"`; `numero(-4.25, 1)` è `"−4,3"` (U+2212); `numero(0.5, 0)` segue l'arrotondamento di `Intl` (`"1"`).
  - `percentuale(0.192387, 1)` è `"19,2%"`.
  - `segnato(14)` è `"+14"`; `segnato(-121)` è `"−121"`.
  - `dataRelativa`, con `ora = new Date((1794000000 + 10*3600)*1000)`:
    - 1794000000 dà `"ieri alle 22:20"`;
    - 1793913600, `"giovedì alle 22:20"`;
    - 1793823600, `"mercoledì alle 21:20"`;
    - 1793568000, `"domenica alle 22:20"`;
    - 1793222400, `"il 28 ottobre alle 22:20"`;
    - 1794000000 + 9*3600, `"oggi alle 07:20"`.
  - `t` con segnaposto; `t` col plurale su `n` = 1 e 2; `variante(4, "a", 3)` è uguale fra due chiamate e sta fra 0 e 3.

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`. Expected: PASS.
- [ ] **Step 5: commit.** `feat(app): testi dell'interfaccia, t() e numeri all'italiana`

---

### Task 7: fondamenta visive e componenti di base

**Files:**
- Create:
  - `app/static/fonts/`: Fraunces variabile (assi `SOFT`, `WONK`, `opsz`, `wght`), Atkinson Hyperlegible Next variabile, diritta e corsiva, e `OFL.txt`;
  - `app/src/lib/stile/fondamenta.css`;
  - `app/src/lib/componenti/Pulsante.svelte`, `Conferma.svelte`, `NumeroFreccia.svelte`, `Dettaglio.svelte`, `Rifiuto.svelte`, `Segnaposto.svelte`.

**Prima del dispatch** (chi orchestra, non il subagente): i caratteri vengono da `github.com/google/fonts`, cartelle `ofl/fraunces/` e `ofl/atkinsonhyperlegiblenext/`. Elenca i file della cartella, chiedi a Mario il sì al download dicendo nomi, fonte e dimensioni, e scarica solo i `.ttf` variabili e `OFL.txt`.

**Interfaces:**
- Produces:
  - `fondamenta.css`, importato da `+layout.svelte`. Contiene:
    - `@font-face` sui file in `/fonts/`;
    - i token, tali e quali alla classe `.vS` di `08-fondamenta.prototype.html:47-71` ma su `:root`;
    - la variante grave `:root.grave` (`:73-77`);
    - `:focus-visible` e `::selection`.
  - `Pulsante`: props `{ forma: "chiude" | "bordo" | "piccolo" | "link"; scelto?: boolean; distruttivo?: boolean; spento?: string /* il motivo */; onclick }`. Da spento è tratteggiato col motivo scritto accanto e resta raggiungibile col Tab (`aria-disabled`, mai `disabled`, così il motivo si legge).
  - `Conferma`: props `{ etichetta: string; conferma: string; annulla?: string /* «Annulla» */; spiegazione?: string; onconferma }`. Al clic il pulsante diventa due pulsanti sul posto, col fuoco sul primo; Esc annulla.
  - `NumeroFreccia`: props `{ valore: number; prima?: number; formato: (x:number)=>string; male?: "sale" | "scende" | null; sottoSoglia?: boolean }`. La freccia è un triangolo SVG: il verso dice la direzione, il colore il giudizio; `null` vuol dire senza colore.
  - `Dettaglio`: il riquadro a lato, largo `var(--lato)`, che scorre in verticale.
  - `Rifiuto`: props `{ testo: string }`, in rosso, accanto a ciò che l'ha causato; `role="alert"`.
  - `Segnaposto`: props `{ sezione: string; fetta: string }`. Testo: «{Sezione}: arriva con la fetta {fetta}.» (`t`).

- [ ] **Step 1: fai la pagina di prova dei componenti**, `app/src/routes/sviluppo/componenti/+page.svelte`, aperta solo in DEV: fuori da DEV fa `goto("/")`. Mostra ogni componente in ogni stato, come il prototipo 08.
- [ ] **Step 2: guardala nel browser**, con `npm --prefix <wt>/app run dev`, a 1280×800 e 1440×900, accanto a `08-fondamenta.prototype.html`. Atteso:
  - stessi colori, filetti e caratteri;
  - nessuna richiesta a `fonts.googleapis.com` fra le richieste di rete;
  - `Conferma` si usa da tastiera (Tab, Invio, Esc).
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app run check`. Expected: pulito.
- [ ] **Step 4: commit.** `feat(app): token, caratteri e componenti di base`

---

### Task 8: impostazioni, scala e movimento

**Files:**
- Create: `app/src/lib/impostazioni.svelte.ts`, `app/src/lib/impostazioni.test.ts`, `app/src/lib/cornice/Impostazioni.svelte`

**Interfaces:**
- Produces, la parte pura (testata):

```ts
export const GRADINI = [1, 1.1, 1.25, 1.4, 1.6, 1.8, 2] as const;
export type Impostazioni = { scala: (typeof GRADINI)[number]; velocita: "1" | "2" | "istantaneo"; movimento: "sistema" | "acceso" | "spento" };
export const PARTENZA: Impostazioni = { scala: 1, velocita: "1", movimento: "sistema" };
export const CHIAVE = "impostazioni";               // valore: JSON { versione: 1, ...Impostazioni }
export function leggi(testo: string | null): Impostazioni; // illeggibile, versione ≠ 1, valore fuori elenco → PARTENZA
export function scrivi(i: Impostazioni): string;
export function scalaApplicata(scelta: number, larghezza: number): number; // il gradino più alto ≤ min(scelta, larghezza/1024), mai sotto 1
export function formaStretta(larghezza: number, applicata: number): boolean; // larghezza/applicata < 1150
export function fattori(i: Impostazioni, riduciSistema: boolean): { k: 1 | 0.5 | 0; muove: boolean };
export function passo(scelta: number, verso: 1 | -1): number; // gradino accanto, fermo agli estremi
```

- Produces, lo stato reattivo:
  - `impostazioni` (`$state`), letto **in modo sincrono** all'import del modulo, con `localStorage` dentro un `try`.
  - `applica()`, che scrive `document.documentElement.style.zoom`, `--k`, `data-movimento="spento"` quando `!muove` e la classe di forma stretta.
  - Un ascoltatore di `resize` e di `matchMedia("(prefers-reduced-motion: reduce)")`.
  - `scorciatoia(e: KeyboardEvent): boolean`, per Cmd+ (`=` o `+`), Cmd− e Cmd0.
  - `avvisoScala` (`$state`), il testo dell'avviso di 1,5 s.

  «Spento» lascia le dissolvenze: le regole CSS su `[data-movimento="spento"]` annullano solo le animazioni di `transform`.
- `Impostazioni.svelte`: il lato della forma B (`33-impostazioni.prototype.html`), con la prova dentro; Esc chiude; ha un `onchiudi`. Contiene:
  - i sette gradini; quelli oltre il tetto sono tratteggiati ma si possono scegliere;
  - l'avviso rosso «Applicata 1,25×: la finestra è stretta. Allargala e torna 1,4×.»;
  - accanto a «Come il sistema», la nota sullo stato di macOS;
  - «Torna ai valori di partenza», senza conferma (ticket 33:27-31).

- [ ] **Step 1: scrivi i test che falliscono**, in `impostazioni.test.ts`:
  - `leggi(null)`, `leggi("{")`, `leggi('{"versione":2,"scala":1.4}')` e `leggi('{"versione":1,"scala":3}')` danno tutti `PARTENZA`.
  - `leggi(scrivi({scala:1.4, velocita:"2", movimento:"spento"}))` dà lo stesso oggetto.
  - `scalaApplicata`:
    - `(2, 1440)` è 1,4;
    - `(1.4, 1280)` è 1,25;
    - `(1.25, 1280)` è 1,25;
    - `(1, 900)` è 1;
    - `(1.6, 1700)` è 1,6.
  - `formaStretta`: `(1280, 1.25)` vero; `(1440, 1.25)` falso (1152); `(1440, 1.4)` vero.
  - `fattori`:
    - con velocità `"2"` e movimento `"acceso"`: `{k:0.5, muove:true}`;
    - con movimento `"sistema"` e `riduciSistema` vero: `muove` falso;
    - con velocità `"istantaneo"`: `k` 0.
  - `passo(2, 1)` è 2; `passo(1, -1)` è 1; `passo(1.1, 1)` è 1,25.

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.** Il deposito separato fra sviluppo e rilascio viene gratis da Tauri (ticket 33:22): niente prefissi nella chiave.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): impostazioni dell'app, scala e movimento`

---

### Task 9: tracciati

**Files:**
- Create: `app/src/lib/tracciati.ts`, `app/src/lib/tracciati.test.ts`, `app/src/lib/componenti/Tracciato.svelte`

**Interfaces:**
- Consumes: `Vista`, `PuntoSerie` (tipi); `percentuale`, `segnato` (Task 6).
- Produces:

```ts
export type Scala = { stacco: number; passo: number; rovesciata: boolean };
/** Casella i copre [da+i·passo, da+(i+1)·passo), con da = stacco − 3·passo (rovesciata: da = stacco + 3·passo). */
export function caselle(v: number, s: Scala): { piene: number; sotto: boolean; sopra: boolean };
export type Tracciato = {
  nome: Chiave;                       // chiave di t
  valore: number | null;              // null → caselle tratteggiate
  testo: string;                      // il numero da mostrare ("19,2%", "+14", "86")
  ramo: "Camera" | "Senato" | null;   // solo per maggioranza e seggi attesi
  scala: Scala | null;                // null → senza stacco (spread fino a 11g)
  rosso: boolean;
  grigio: boolean;                    // partita finita
  rimando: string | null;             // fetta che riempie un tracciato senza dato
};
export function tracciatiDi(vista: Vista, serie: PuntoSerie[]): [Tracciato, Tracciato, Tracciato];
```

**Regole di `tracciatiDi`** (spec:300-311, ticket 31). La coalizione è `{giocatore} ∪ Vista.coalizione` al turno zero e in trattativa, `{giocatore} ∪ alleati[].partito` al governo e in crisi. La quota di un partito `j` è `quota_fra_i_votanti`: la media sugli istituti di `quote[j] / (1 − quote[ultimo])`, con l'indice di `j` in `Vista.partiti`.

Le soglie si leggono da `Vista.disposizioni`:
- `soglia_lista.in_vigore` e `soglia_premio.in_vigore`, in centesimi di punto (`"300"` = 3%, `"4200"` = 42%);
- `premio.in_vigore` vale `nessuno`, `lista` o `coalizione`: con `nessuno` lo stacco del consenso è 50.

Il margine di un ramo è `seggi della coalizione − (totale/2 + 1)`, con la divisione intera (`crates/sim-core/src/parlamento/voto.rs:106-107`). La Camera più debole è quella col margine minore.

| Stato | 1 | 2 | 3 |
|---|---|---|---|
| turno zero | consenso del partito | seggi attesi, da `proiezione` | spread |
| governo, crisi, trattativa | consenso della coalizione; rosso se un margine di `proiezione` < 0; non si accende se `proiezione` è `null` | maggioranza, da `Vista.seggi`; senza seggi (totali 0) `valore` è `null` | spread |
| opposizione | consenso del partito | per farlo cadere: `valore` `null`, `rimando` «11e» | spread |
| finita | i tre dell'ultimo stato, `grigio` vero e `rosso` falso | | |

Lo spread ha `scala` `{stacco: 550, passo: 75, rovesciata: true}` solo per le caselle: `rosso` è sempre falso e lo stacco non si disegna fino a 11g (lettura 4). La variazione sotto il numero viene dal punto della `serie` del turno prima, quando c'è.

- [ ] **Step 1: scrivi i test che falliscono.** Le Viste si leggono da `.scratch/v1-piano-11-ui/viste/` con `fs`, e gli oracoli sono calcolati con le formule del core.
  - `caselle`, sulla soglia:
    - `caselle(42, premio)` → `piene` 4 (fuori dal pericolo); `caselle(41.99, premio)` → 3;
    - `caselle(0, margine)` → 4; `caselle(-1, margine)` → 3;
    - `caselle(3, lista)` → 4;
    - `caselle(550, spread)` → 3 (dentro); `caselle(549, spread)` → 4;
    - `caselle(13.4, lista)` → `piene` 10 e `sopra` vero;
    - `caselle(-121, margine)` → `piene` 0 e `sotto` vero.
  - Su `01-turno-zero.json`:
    - consenso del partito: `testo` `"19,2%"`, `piene` 10, `sopra`, non rosso;
    - seggi attesi: valore −121, ramo «Camera», rosso;
    - spread 84, `piene` 10, non rosso.
  - Su `25-registro.json[0].prima` (coalizione dichiarata con Centrodestra e Destra): seggi attesi valore 15, ramo «Senato», `piene` 9, non rosso.
  - Su `03-governo-ordinario.json`:
    - consenso della coalizione 56,68% (`"56,7%"`), `piene` 7, non rosso (margini di proiezione 24 e 15);
    - maggioranza valore 14, ramo «Senato», `piene` 8.
  - Su `05-crisi.json`: consenso della coalizione 20,8%, rosso; maggioranza −97, ramo «Camera», rosso.
  - Su `07-partita-finita-eredita.json`: tre tracciati `grigio` e nessuno `rosso`.
  - Su `06-opposizione-governo-ia.json`: il secondo tracciato ha `valore` `null` e `rimando` «11e».

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa `tracciati.ts`, poi `Tracciato.svelte`.** Il componente ha:
  - dieci caselle a `steps()`;
  - il retino solo dove la casella è larga almeno 12 px;
  - ◂ e ▸ fuori scala (◂ rosso se `rosso`);
  - lo stacco dopo la terza casella, quando `scala` c'è e il tracciato non è lo spread;
  - il tratteggio quando `valore` è `null`, col rimando nel `title` e nel testo per i lettori di schermo.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): tracciati con le scale del ticket 31`

---

### Task 10: ossatura, cornice, sommario e partita in corso

**Files:**
- Create:
  - `app/src/lib/ossatura.ts`, `app/src/lib/ossatura.test.ts`;
  - `app/src/lib/partita.svelte.ts`;
  - `app/src/lib/cornice/Cornice.svelte`, `app/src/lib/cornice/Sommario.svelte`;
  - `app/src/routes/+layout.svelte`;
  - `app/src/routes/partita/[sezione]/+page.svelte`.
- Modify: `app/src/routes/+layout.ts`, `app/src/routes/+page.svelte` (sostituito; l'Edicola vera arriva nel Task 12, qui basta un collegamento a `/partita/cruscotto`).

**Interfaces:**
- Consumes: `guscio` (Task 4), `installaFinto` (Task 5), `t` (Task 6), i componenti (Task 7), `impostazioni` (Task 8), `tracciatiDi` (Task 9).
- Produces:

```ts
export type StatoGiocatore = "turno_zero" | "trattativa" | "governo" | "crisi" | "opposizione" | "finita";
export const SEZIONI = ["prima-pagina","cruscotto","governo","aula","conti","bilancio","paese","partiti","campagna","registro"] as const;
export type Sezione = (typeof SEZIONI)[number];
/** In quest'ordine: finita → turno === 0 → fase "trattativa" → crisi → governa → opposizione. */
export function statoDi(v: Vista): StatoGiocatore;
export function casa(v: Vista): Sezione | "trattativa" | "eredita";
/** null = accesa; altrimenti il motivo della barrata (07-ossatura.prototype.html:235). */
export function barrata(s: Sezione, v: Vista): Chiave | null;
/** La fetta che costruisce la sezione in questo stato; null se è costruita in 11a. */
export function fettaDi(s: Sezione | "trattativa" | "eredita" | "schema", stato: StatoGiocatore): string | null;
```

**Regole di `barrata`** (ticket 07:27-36, spec:278-289):
- **Prima pagina:** barrata al turno zero.
- **Conti, Paese, Partiti:** mai barrate.
- **Campagna:** accesa solo con `turni_al_voto === 0`.
- **Registro:** barrato al turno zero; a partita finita, acceso solo con `eredita.bozza`.
- **Aula:** barrata con `fase === "sciolta"`, in trattativa e a partita finita.
- **Governo:** barrato se `!governa`.
- **Bilancio:** acceso solo con `governa && data.trimestre === 4`.

**`fettaDi`** in 11a:
- tutte le sezioni sono segnaposto;
- il cruscotto è dell'11b al turno zero e in trattativa, dell'11c al governo e in crisi, dell'11e all'opposizione;
- Prima pagina e Registro vanno all'11f; Governo e Aula all'11c; Conti e Bilancio all'11d; Paese, Partiti e Campagna all'11b;
- la trattativa all'11b, l'eredità all'11e, lo schema all'11e.

**`partita.svelte.ts`**, lo stato reattivo condiviso:
- i campi `vista`, `serie`, `guasto: string | null` e `occupato: boolean`;
- le azioni `fonda(scenario, fondazione)`, `riprendi(id)`, `chiudiTurno()` e `esci()`.

Ogni azione:
1. mette `occupato` e lo toglie in `finally`;
2. chiama il guscio;
3. aggiorna `vista` e `serie` (`guscio.serie()` dopo ogni Vista nuova);
4. davanti a un guasto lascia `vista` com'era e scrive `guasto`;
5. davanti a un rifiuto lo rilancia al chiamante, che lo mostra sul posto.

`chiudiTurno` porta a `/partita/<casa>` del turno nuovo (spec:290).

**`+layout.ts`**:
- `ssr = false`, come oggi;
- import del modulo delle impostazioni, che applica la scala prima del primo disegno;
- in DEV, `installaFinto(new URL(location.href))`, che semina `partita.vista` se il campione ne ha una;
- `?tagli=1` avvia il contatore (Task 11).

**`+layout.svelte`**: importa `fondamenta.css`, aggancia `scorciatoia` a `keydown` e mostra `avvisoScala`.

**La cornice:**
- forma A · Gerenza, alta 104 px a 1× (`08-fondamenta.prototype.html`);
- testatina; lo stato su tre righe (testi da `07-ossatura.prototype.html:215`);
- i tre `Tracciato`;
- «Chiudi il turno ›» pieno, assente a partita finita («Nessun turno da chiudere»);
- la banda nera in crisi;
- il guasto in una riga sotto la gerenza, con «La partita è rimasta com'era.»;
- in forma stretta via la testatina, con «Schema» e le voci del sommario a 13 px.

**Il corpo della sezione** scorre in verticale quando non basta; la cornice resta ferma, e niente scorre di lato (storia 40).

**Il sommario:** le dieci sezioni con le barrate (motivo nel `title` e in un `aria-describedby`); a destra Schema dello Stato, Impostazioni (apre il lato del Task 8) ed Edicola. La pagina `/partita/[sezione]` senza `partita.vista` torna a `/`.

- [ ] **Step 1: scrivi i test che falliscono**, in `ossatura.test.ts`, sulle Viste campione:
  - `statoDi`:
    - 01 → `turno_zero`;
    - 02 → `trattativa`;
    - 03 → `governo`;
    - 05 → `crisi`;
    - 06 → `opposizione`;
    - 07 → `finita`.
  - Barrate al turno zero (01): Prima pagina, Governo, Aula e Registro; Campagna accesa. Aula col motivo «Camere sciolte: niente aula fino al voto».
  - Barrate all'opposizione (06): Governo, e Bilancio anche fuori stagione.
  - In trattativa (02): Aula e Governo barrati.
  - A partita finita (07): Governo, Aula, Bilancio e Campagna barrati; Registro barrato perché `eredita.bozza` è falso.
  - `casa`: 01 → `cruscotto`; 02 → `trattativa`; 07 → `eredita`.

- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa**, prima `ossatura.ts`, poi lo stato, la cornice, il sommario e le pagine.
- [ ] **Step 4: verifica nel browser**, con `npm --prefix <wt>/app run dev`. Apri `/partita/cruscotto?vista=` con ognuno di questi campioni:
  - `01-turno-zero.json`;
  - `02-trattativa-incarico.json`;
  - `03-governo-ordinario.json`;
  - `05-crisi.json`;
  - `06-opposizione-governo-ia.json`;
  - `07-partita-finita-eredita.json`;
  - `08-tono-grave.json`.

  Atteso:
  - stato, barrate e tracciati come nei test;
  - «Chiudi il turno» su `25-registro.json&stato=0` porta alla casa del turno 1;
  - a 1280 con 1,25× la cornice è in forma stretta.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 6: commit.** `feat(app): cornice, sommario e stato del giocatore`

---

### Task 11: contatore dei tagli

**Files:**
- Create: `app/src/lib/sviluppo/tagli.ts`

**Interfaces:**
- Produces: `export function tagli(radice: HTMLElement): number`. Lo stesso algoritmo di `tagli()` in `08-fondamenta.prototype.html:537-547`:
  1. per ogni discendente (saltando `svg` e ciò che porta `data-tagli-ignora`), è un taglio se esce dalla radice di più di 1 px a destra o in fondo, fuori da un contenitore che scorre;
  2. è un taglio anche lo `scrollWidth` oltre il `clientWidth` di più di 1 px, salvo gli elementi inline, quelli con `text-overflow: ellipsis` e quelli con `-webkit-line-clamp`;
  3. ogni taglio prende la classe `.taglio` (outline magenta).
- Con `?tagli=1`, solo in DEV, `+layout.ts` lo lancia dopo ogni navigazione e a ogni `resize`. Il numero compare in un badge fisso e in `console.info`.

- [ ] **Step 1: aggiungi il contatore**, poi provalo su un elemento troppo largo nella pagina dei componenti del Task 7: almeno un taglio.
- [ ] **Step 2: misura.** Lancia il contatore sui sette campioni del Task 10, a 1280×800 e 1440×900, a scala 1 e alla massima. Expected: 0.
- [ ] **Step 3: verifica il rilascio.** Run: `npm --prefix <wt>/app run build`, poi `grep -rl "taglio" <wt>/app/build`. Expected: nessun file.
- [ ] **Step 4: commit.** `feat(app): contatore dei tagli in sviluppo`

**Fine della PR 2.** Prima del round di review, fai la batteria del Task 15, passi 1-4, sugli stati della cornice.

---

### Task 12: Edicola

**Files:**
- Create: `app/src/lib/edicola/RigaPartita.svelte`, `DettaglioPartita.svelte`, `DettaglioAlbo.svelte`, `Grafico.svelte`; `app/src/lib/edicola/edicola.ts` e `edicola.test.ts`
- Modify: `app/src/routes/+page.svelte`

**Interfaces:**
- Consumes: `guscio.partite`, `albo`, `riprendi`, `abbandona` e `confermaEredita`; `partita.riprendi`; `dataRelativa`, `numero` e `t`; `Conferma`, `Dettaglio` e `Pulsante`; `Impostazioni.svelte`.
- Produces:

```ts
/** Per ultimo_salvataggio decrescente. */
export function inOrdine(p: VocePartita[]): VocePartita[];
/** "riprendi" | "eredita" (finita: «Apri l'eredità ›», senza «Abbandona») | "nessuna" (non apribile) */
export function azioneDi(p: VocePartita): "riprendi" | "eredita" | "nessuna";
/** Il colore della freccia di un indicatore d'albo: debito_pil, spread e disoccupazione salgono male; inflazione senza colore. */
export function maleSe(id: string): "sale" | null;
```

**Comportamento** (spec:365-366, ticket 13): la forma A · Banco di `13-edicola.prototype.html`.
- **Riga di una partita in corso:** solo ciò che porta la `Testata`. I riavvolgimenti compaiono solo se sono più di 0.
- **Partita non apribile:** riga a retino, `motivo` in rosso, nessun pulsante. Il dettaglio dice che il file resta dov'è.
- **Abbandono**, da `Conferma` con «Conferma: abbandona» e «Annulla»:
  1. `riprendi(id)`, poi `abbandona()`, poi `partite()` e `albo()`;
  2. la voce d'albo nuova (la prima per `fine`, con lo stesso `nome`) si apre nel dettaglio.
- **«Riprendi ›»:** `partita.riprendi(id)`, poi `/partita/<casa>` (fino a 11f non c'è la prima pagina). Il pulsante è spento mentre `partita.occupato`.
- **«Apri l'eredità ›» e «Leggi l'eredità ›»:** `Segnaposto` dell'11e.
- **Primo avvio:** sezioni vuote con l'invito a «Nuova partita ›», che porta a `/scenario`.
- **Grafici del dettaglio d'albo:** SVG semplici dalla `serie`:
  - consenso;
  - margine, con la Camera piena e il Senato tratteggiato;
  - spread;

  con i voti segnati e il valore al passaggio del puntatore. Ogni elezione ha una barra delle quote.
- **Impostazioni** apre il lato al posto del dettaglio.

- [ ] **Step 1: scrivi i test che falliscono**, su `13-edicola.json`:
  - `inOrdine` mette prima «Patto Civico» (1794000000) e ultima «Unione dei Pendolari».
  - `azioneDi`: «Lista del Faro» → `eredita`; «Unione dei Pendolari» → `nessuna`; le altre → `riprendi`.
  - `maleSe("inflazione")` è `null`; `maleSe("spread")` è `"sale"`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa i componenti e la pagina.**
- [ ] **Step 4: verifica nel browser.**
  - `/?vista=13-edicola.json`: cinque partite e tre voci d'albo; la più recente più grande; l'abbandono si conferma sul posto da tastiera.
  - Dopo l'abbandono, il mock rifiuta `abbandona` «non nel campione» e il rifiuto compare accanto alla scheda. È atteso: il percorso vero si prova nel Task 15.
  - Il primo avvio si prova con un campione vuoto: aggiungi a `finto.ts` il parametro `?edicola=vuota`, che dà `[]` a `partite` e `albo`.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 6: commit.** `feat(app): edicola`

---

### Task 13: Scenario ed editor

**Files:**
- Create: `app/src/lib/editor/Cursore.svelte`, `LeggeElettorale.svelte`, `Disposizioni.svelte`, `Riepilogo.svelte`; `app/src/lib/editor/editor.svelte.ts` e `editor.test.ts`
- Create: `app/src/routes/scenario/+page.svelte`

**Interfaces:**
- Consumes: `guscio.catalogoScenario`, `scenari`, `caricaScenario`, `salvaScenario` e `riepilogoScenario`; `CatalogoScenario` (Task 1).
- Produces:

```ts
/** Lo scenario di base dal catalogo: i valori dei cursori, legge_di_base, nessun ritocco. */
export function base(c: CatalogoScenario): SuMisura;
/** Un ritocco: sull'Italia di oggi (nome "") apre un su misura nuovo senza nome (storia 13). */
export function ritocca(s: SuMisura | null, c: CatalogoScenario, f: (s: SuMisura) => void): SuMisura;
/** Cambiare legge toglie i ritocchi delle nove disposizioni elettorali (ticket 14:24). */
export function cambiaLegge(s: SuMisura, c: CatalogoScenario, legge: string): SuMisura;
export function alLimite(v: number, cur: CursoreCatalogo): boolean;
/** Una richiesta in volo per volta; arrivata quella, parte l'ultima chiesta nel frattempo; l'esito vecchio non sovrascrive il nuovo. */
export function ultimoVince<A, R>(f: (a: A) => Promise<R>, esito: (r: R | ErroreGuscio, a: A) => void): (a: A) => void;
```

**Comportamento** (spec:367, ticket 14): la forma A · Banco di `14-editor.prototype.html`.
- **Bottoni dello scenario:** Italia di oggi, i salvati (da `scenari()`) e «+ Nuovo su misura».
- **Cursori:** valore grande, gli estremi, la tacca rossa sul valore di oggi e il filo giallo se è stato mosso. Al limite compare «fermo al limite della taratura». Sotto l'inflazione, la riga che spiega perché non cambia il riepilogo (ticket 14:34).
- **Legge elettorale:** un menu con i nove parametri a bottoni. Accanto a ogni parametro, il gradino che darebbe la legge scelta, da `leggi_elettorali`.
- **Costituzione:** durata della legislatura e numero dei parlamentari.
- **Disposizioni ordinarie:** raggruppate per `dicastero`; bottoni fino a 4 gradini, oltre un menu; un pallino sul gradino di partenza.
- **Riepilogo dal vivo** a lato: `riepilogoScenario` attraverso `ultimoVince`. Mostra avvisi, spread, interessi, disavanzo, il peso delle disposizioni cambiate (`saldo_disposizioni_pil`, con «il disavanzo del turno zero non le contiene ancora») e i sei posti con quote, seggi e maggioranza.
  - Gli avvisi «senza maggioranza» e «crisi del debito» sono in rosso, le situazioni accese col filo grigio. I testi degli avvisi li scrive la UI dai campi dell'`Avviso`, come `Avviso::testo` (`crates/sim-core/src/riepilogo.rs:70-96`).
  - **Davanti a un rifiuto:** testo del core sopra i parametri, riepilogo fermo all'ultimo valido e spento, «Fonda il partito» spento col motivo.
- **Salva:**
  - il nome in un campo, con `scenari()` per sapere se c'è già;
  - se c'è già, `Conferma` con «Conferma: sostituisci «…»» e «Annulla»;
  - il rifiuto del guscio (nome non valido) va sotto il campo.
- **«‹ Edicola»** con modifiche non salvate: `Conferma` con «Esci comunque» e «Resta».
- **«Fonda il partito ›»:** porta a `/fondazione`. Lo scenario in composizione (o `null` per l'Italia di oggi) resta in `editor.svelte.ts`, solo in memoria.

- [ ] **Step 1: scrivi i test che falliscono**, in `editor.test.ts`, su `14-editor.json` passato come catalogo:
  - `base(c).cursori.debito_pil` è 136,7 e `legge_elettorale` è `"legge_180"`.
  - `ritocca(null, c, s => s.cursori.inflazione = 8)` ha `nome` `""` e `inflazione` 8.
  - `cambiaLegge` dopo un ritocco di `coalizioni` lo toglie.
  - `alLimite(160, debito)` è vero; `alLimite(150, debito)` è falso.
  - `ultimoVince`, con promesse risolte a mano: chiamato con `a`, `b` e `c` mentre `a` è in volo, chiama `f` solo con `a` e poi con `c`. Se `a` si risolve dopo che `c` è stato chiesto, `esito` riceve il risultato di `a` e poi quello di `c`, e il chiamante tiene l'ultimo. Un errore di `a` non blocca `c`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser** su `/scenario`.
  - Le otto prove di `riepiloghi` si riproducono a mano:
    - «Incoerente» (`coalizioni` a «no») dà il rifiuto sul posto;
    - «Debito al tetto» dà la crisi del debito in rosso.
  - Muovere un cursore a passi della griglia aggiorna il riepilogo.
  - Tutto si raggiunge col Tab.
  - I cursori si muovono con le frecce.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 6: commit.** `feat(app): scenario ed editor`

---

### Task 14: Fondazione e ingresso nel turno zero

**Files:**
- Create: `app/src/lib/fondazione/Asse.svelte`, `TabellaPosti.svelte`, `DettaglioPosto.svelte`; `app/src/lib/fondazione/fondazione.ts` e `fondazione.test.ts`
- Create: `app/src/routes/fondazione/+page.svelte`

**Interfaces:**
- Consumes:
  - `guscio.anteprimaFondazione`, con lo stesso scenario che poi va a `fonda` (ticket 12:27);
  - `partita.fonda`;
  - lo scenario in composizione del Task 13;
  - `CatalogoScenario.partiti`, per i nomi e le posizioni sotto l'asse.
- Produces:

```ts
export const LUNGHEZZA_NOME = 40;
/** Solo spazi = vuoto. */
export function nomeValido(nome: string): boolean;
/** Il motivo per cui «Fonda il partito» è spento, o null: prima il nome, poi il posto. */
export function cosaManca(nome: string, posto: number | null): Chiave | null;
/** I partiti dei due lati del posto: «fra X e Y». */
export function fra(posto: number, partiti: PartitoCatalogo[]): [PartitoCatalogo, PartitoCatalogo];
/** «Con chi correre»: i sette, prima i disponibili, ciascuno con la causa del core; ordine dei dati a parità. */
export function conChiCorrere(v: VocePosizione, partiti: PartitoCatalogo[]): { partito: PartitoCatalogo; disponibile: boolean; causa: string }[];
```

**Comportamento** (spec:369-370, ticket 15): la forma A · Banco di `15-fondazione.prototype.html`.
- **Nome:** `maxlength` 40 col contatore.
- **Asse:** istogramma sui sette punti, partiti sotto, sei posti come `radiogroup` con le frecce. Nessun posto scelto all'apertura.
- **Tabella dei sei posti:** «fra X e Y», quota con la barra, alleati disponibili. Non riporta quote della coalizione né seggi.
- **Dettaglio del posto scelto:**
  - la quota attesa;
  - la riga «È un sondaggio: il voto può dire altro.»;
  - «Con chi correre».
- **«Fonda il partito ›»:**
  - diretto, senza conferma, spento col motivo finché manca qualcosa;
  - spento anche mentre `partita.occupato`;
  - il rifiuto compare sopra il bottone; il guasto in testa alla pagina;
  - riuscito, porta a `/partita/cruscotto`.
- **«‹ Scenario»** tiene nome e posto in memoria. Tornare e cambiare scenario rilancia l'anteprima, quindi con un seme nuovo.

- [ ] **Step 1: scrivi i test che falliscono**, su `15-fondazione.json`:
  - `nomeValido("   ")` è falso; `nomeValido("Patto")` è vero.
  - `cosaManca("", 4.5)` dà la chiave del nome; `cosaManca("Patto", null)` quella del posto; `cosaManca("Patto", 4.5)` è `null`.
  - `fra(4.5, partiti)` è Centro e Centrodestra (4 e 5), letti da `15-fondazione.json`, `partiti`.
  - `conChiCorrere` su `anteprime["oggi, seme 4"]` al posto 4,5: sette voci, i disponibili prima. Il Centro è non disponibile con la causa del campione (README:166-169).
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser.**
  - `/scenario` → «Fonda il partito ›» → `/fondazione`: l'anteprima di «oggi, seme 4».
  - Scegli un posto e scrivi un nome → «Fonda il partito ›» → `/partita/cruscotto` con la cornice del turno zero (Vista 01).
  - Due clic veloci su «Fonda il partito ›» fanno una chiamata sola: lo conta il gestore del mock, con un `console.count` temporaneo.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 6: commit.** `feat(app): fondazione e ingresso nel turno zero`

---

### Task 15: verifica della fetta

Chi esegue questo task è la sessione principale, non un subagente, perché il passo 5 usa computer-use sull'app vera. Ogni passo ha il suo esito scritto nella PR.

- [ ] **Step 1: batteria nel browser**, con `npm --prefix <wt>/app run dev` e `?tagli=1`, a 1280×800 e 1440×900, ciascuna a scala 1 e alla massima (1,25 e 1,4). Le pagine:
  - `/?vista=13-edicola.json` e la variante vuota;
  - `/scenario`;
  - `/fondazione`;
  - la cornice su 01, 02, 03, 05, 06, 07 e 08.

  Expected: il contatore dà 0 ovunque; la forma stretta compare a 1280 con 1,25×.
- [ ] **Step 2: movimento.**
  - Col movimento spento, nessuna animazione parte: `document.getAnimations().length` è 0 dopo ogni interazione, tono grave compreso.
  - Con «Come il sistema» e `prefers-reduced-motion: reduce` emulato, vale lo stesso.
- [ ] **Step 3: tastiera.** Su ogni pagina, dall'albero di accessibilità, ogni azione si raggiunge col Tab e si attiva con Invio o Spazio, conferme sul posto comprese. Le barrate dicono il motivo al fuoco.
- [ ] **Step 4: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`.** Expected: PASS, pulito.
- [ ] **Step 5: giro nell'app vera.** Lancia `npm --prefix <wt>/app run tauri dev`, poi gioca con computer-use:
  1. edicola vuota;
  2. «Nuova partita ›»;
  3. un ritocco e il salvataggio dello scenario con un nome;
  4. il ritorno all'Italia di oggi;
  5. «Fonda il partito ›» con un nome e un posto;
  6. il turno zero con la cornice;
  7. «Chiudi il turno»;
  8. Edicola, dove la partita compare in testa;
  9. «Riprendi»;
  10. Edicola, poi «Abbandona» confermato, con la voce d'albo aperta nel dettaglio.

  Poi prova Cmd+ fino a 2× (con l'avviso e il tetto a 1440), Cmd0 e il movimento spento. Chiudi con Cmd+Q, riapri e controlla che le impostazioni siano rimaste.

  Verifica in WKWebView:
  - `zoom`;
  - i due caratteri;
  - le media query;
  - le coordinate del puntatore sui cursori dell'editor sotto `zoom`;
  - `prefers-reduced-motion`;
  - il Tab dentro l'SVG dell'asse.

  Se qualcosa non va, controlla prima i punti da sorvegliare 1 e 2.
- [ ] **Step 6: screenshot nella PR**: edicola, editor, fondazione e turno zero, alle due finestre. Il `craft-reviewer` li confronta coi prototipi e coi token; gli scarti voluti si scrivono in PR. Nella PR vanno anche:
  - la tabella dei testi nuovi, quelli che non vengono dai prototipi;
  - un rimando alle quattro letture confermate, in testa al piano.
- [ ] **Step 7: commit dei ritocchi della verifica**, se ce ne sono. `fix(app): ritocchi dalla verifica della 11a`

**Fine della PR 3.**
