# V1 · Piano 3: motore economico — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Un motore economico trimestrale in blocchi separati dentro `sim-core`, che parte dallo scenario di base dell'Italia di ottobre 2026 e a ogni chiusura del turno muove produzione per settore, prezzi, output gap, inflazione, tasso BCE, disoccupazione per regione, conti pubblici, spesa obbligatoria, stock dei quattro settori, spread e regole UE.

**Architecture:** `Economia` è un campo di `Partita`, serializzato nello snapshot. `Economia::passo` chiama i blocchi in un ordine fisso scritto una volta sola (cucitura 1); ogni blocco è un file sotto `crates/sim-core/src/economia/`. Struttura (matrice A, mix regionali, piramide) e scenario di base sono file TOML in `dati/base/`, prodotti in parte da uno script offline in `strumenti/taratura/`. Gli shock passano tutti dal flusso `Flusso::Economia` del `Caso`. Il CLI legge lo stato vero; la `Vista` riceve solo lo spread, l'unico indicatore economico esatto e sempre a schermo.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova a runtime; Python 3 con la sola libreria standard per lo script di taratura; Tauri 2.12.3, Svelte 5.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md), sezioni «Architettura», «Le quattro cuciture», «Modulo scenario», «Motore economico», «Testing Decisions». Piano precedente: [`2026-10-21-v1-piano-1-fondamenta.md`](2026-10-21-v1-piano-1-fondamenta.md) (riga 44 per lo scope di questo piano).

## Due PR

Il piano non entra in una sessione sola: si divide in due PR, ciascuna verde da sola.

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · economia reale | `feat/v1-motore-economico` | `main` | 1-6 | debiti del piano 1, dati strutturali, scenario di base, Leontief, IS-Phillips e BCE, Okun per regione |
| B · conti pubblici | `feat/v1-conti-pubblici` | `feat/v1-motore-economico` (stacked, da ribasare su `main` dopo il merge di A) | 7-10 | finanza pubblica e impulso fiscale, spesa obbligatoria, SFC a 4 settori, spread, TPI e regole UE, spread nella vista |

## Confini con gli altri piani (scelte di questo piano)

Il passo «redditi dei gruppi e panieri» dell'ordine della spec resta un posto vuoto nell'ordine: i 360 gruppi e le tre fasce nascono col piano 4, che vi aggiunge redditi, panieri e Gini leggendo prezzi dei settori, gap e disoccupazione per regione, conti dello Stato. Per lo stesso motivo IRPEF a scaglioni, elasticità del reddito imponibile e drenaggio fiscale arrivano con la legge di bilancio (piano 5), che cambia le aliquote: qui le quattro imposte sono aggregate, con la loro elasticità al ciclo. Restano fuori anche, ciascuno col piano che porta ciò da cui dipende: le misure, che entrano col piano 5 nel posto «misure» in testa all'ordine; i cambi di regime per legge (piano 6, qui il regime è un dato dello scenario); il fondo di riserva (piano 5); gli effetti di un taglio a salari e acquisti dei dicasteri, cioè turnover bloccato e servizio che peggiora (piano 5, con la legge di bilancio che li taglia: qui le due quote esistono e finiscono in domanda e nei conti); le disposizioni territoriali (piano 6); il piano strutturale riveduto (piano 7, serve un governo nuovo); aste deserte, MES, OMT, ristrutturazione e rating (piano 9, sono eventi); la credibilità del ministro dell'Economia nello spread (piano 5, serve il ministro); le prime stime rumorose (piano 4); lo spostamento del PIL potenziale da infrastrutture, scuola, giustizia, energia e demografia (piano 5, quando quelle grandezze esistono).

## Global Constraints

Valgono tutti i vincoli del piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`), in particolare: italiano coi termini di `CONTEXT.md`; `sim-core` senza I/O e senza thread; solo `Vec` e `BTreeMap`; ChaCha8 versionato; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova in `sim-core` né in `sim-cli`: algebra lineare 10×10 e Box-Muller scritti a mano.
- Euro in `i64`, arrotondati con `f64::round() as i64` nel punto in cui un flusso tocca i conti pubblici o gli stock SFC; tassi, gap, indici di prezzo e coefficienti in `f64` (spec, «Numeri»).
- Tutti gli shock economici escono da `Flusso::Economia`, sempre nello stesso numero e ordine per turno, anche quando la loro ampiezza è zero: il flusso non deve disallinearsi se una manopola cambia.
- Ogni valore numerico non dichiarato reale dalla spec è una manopola in `dati/base/manopole.toml` con `valore`, `min`, `max`, `ticket`; quelli senza fonte nella ricerca portano nel file il commento `# assunzione del piano 3, nessuna fonte in docs/ricerca` (spec, «Manopole»).
- Ogni ingresso dello scenario ha `valore`, `min`, `max`, `fonte`; un valore fuori intervallo si rifiuta col nome (spec, «Modulo dati», «Limiti di taratura»).
- Lo stato dell'economia resta finito: nessun `NaN` né infinito, nemmeno nello scenario estremo; `Partita::salva` li rifiuta già (`crates/sim-core/src/partita.rs:92-104`).
- `VERSIONE_SCHEMA` passa a `2` nel Task 1 e resta `2` per tutto il piano.
- Nomi dei blocchi e delle grandezze dalla spec: settore produttivo, spesa obbligatoria, regime, spread, PIL potenziale, spesa di funzionamento, margine discrezionale.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.

## Review Focus

1. **Matrice A modificata a mano che rende un settore non produttivo** (colonna con input ≥ 1): rifiuto al caricamento col nome del settore, mai una produzione negativa a runtime. Test `colonna_non_produttiva_rifiutata_col_nome` nel Task 3.
2. **Scenario ai limiti di taratura giocato a lungo** (debito 200%, inflazione 12%, disoccupazione 15%): lo stato resta finito, il salvataggio riesce e la somma SFC fa zero dopo 200 turni. Test `scenario_estremo_resta_finito` nel Task 10.
3. **Settore o regione sconosciuti nei dati** (refuso in `settori` di un dicastero o nel `mix` di una regione): rifiuto col nome. Test `settore_sconosciuto_nel_dicastero_rifiutato_col_nome` nel Task 3.
4. **Scadenze annuali al cambio d'anno** (bilancio 2027 applicato una volta sola al turno gennaio-marzo, indicizzazione delle pensioni a gennaio, controllo UE solo ad aprile-giugno): niente doppie applicazioni né salti. Test `bilancio_2027_applicato_una_volta` nel Task 7 e `conto_di_controllo_si_muove_solo_ad_aprile_giugno` nel Task 10.
5. **Salvataggio a metà di una procedura** (PDE inadempiente, annate di pensionati diverse): ricaricato, prosegue identico. Test `salva_carica_con_pde_inadempiente` nel Task 10.

## Dispatch

PR A: Task 1 e Task 2 in parallelo (file disgiunti, il 2 non consuma nulla dal 1), poi 3 → 4 → 5 → 6 in sequenza: toccano tutti `crates/sim-core/src/economia/mod.rs` (dal 3, che lo crea) o `dati.rs`/`partita.rs`, e ognuno consuma i campi di `Economia` del precedente. PR B: 7 → 8 → 9 → 10 in sequenza, su `economia/mod.rs`, `finanza.rs` e `tests/conti.rs` condivisi. Nessuna dipendenza nuova, quindi `Cargo.lock` non cambia; `manopole.toml` lo scrivono il Task 3 e il Task 8, già in sequenza. Ogni riga **Dispatch** sotto dà subagente, dipendenza e skill-gate; la riga **Ricerca** è il riferimento da copiare nel brief.

---

## PR A · Economia reale (Task 1-6)

### Task 1: Debiti del piano 1

Chiude i rimandi della PR #15 che questo piano tocca: dati costruibili solo validati, seme in un posto solo, comandi Tauri async, normale estratta dal `Caso`.

**Files:**
- Modify: `crates/sim-core/src/dati.rs` (campi di `DatiGioco` privati, accessori)
- Modify: `crates/sim-core/src/partita.rs` (tolto il campo `seme`; `VERSIONE_SCHEMA = 2`)
- Modify: `crates/sim-core/src/caso.rs` (`seme()`, `uniforme`, `normale`)
- Modify: `crates/sim-core/tests/dati.rs`, `crates/sim-core/tests/partita.rs`, `crates/sim-core/tests/caso.rs`
- Modify: `app/src-tauri/src/lib.rs` (comandi `async`)

**Ricerca:** `docs/ricerca/tecnica/architettura-simulazione.md:19-22` (PRNG nominato, niente `usize`, un flusso per sottosistema); spec, «Architettura», riga 251 (ogni estrazione passa dal caso, lo stato del generatore sta nello snapshot).

**Dispatch:** `rust-engineer`, intero · primo della PR A, in parallelo col Task 2: file disgiunti (`crates/`, `app/src-tauri/src/lib.rs` contro `strumenti/`, `dati/base/`) · skill-gate: true. Le due righe `async` del guscio sono Rust e stanno accanto ad `avvia`/`chiudi`, che il task non cambia ma i cui test rilancia: spezzarle verso un altro ruolo aggiunge un passaggio di consegne per due parole.

**Interfaces:**
- Consumes: `Caso`, `DatiGioco`, `Partita` del piano 1.
- Produces:
  - `DatiGioco` con campi privati; accessori `dicasteri(&self) -> &[Dicastero]`, `manopole(&self) -> &BTreeMap<String, Manopola>`, `manopola(&self, nome: &str) -> f64`. Un `DatiGioco` esiste solo validato (da `da_testi` o da `Partita::carica`, che chiama `valida`), quindi `Partita::nuova` resta infallibile. `manopola` usa `expect` con il messaggio `manopola validata al caricamento`: la presenza la garantisce il controllo `ManopolaMancante` che il Task 3 aggiunge a `valida`.
  - `Partita` senza il campo `seme`: lo tiene solo `Caso`. `Caso::seme(&self) -> u64`.
  - `Caso::uniforme(&mut self, flusso: Flusso) -> f64` in `(0, 1]`: `((prossimo >> 11) + 1) as f64 / 2^53`.
  - `Caso::normale(&mut self, flusso: Flusso) -> f64`: Box-Muller su due `uniforme` consecutive, `sqrt(-2 ln u1) · cos(2π u2)`, senza cache del secondo valore.
  - `VERSIONE_SCHEMA = 2`.
  - Comandi Tauri `async fn nuova_partita` e `async fn chiudi_turno`, stesse firme di ritorno. Il corpo resta sincrono, nessun `await` mentre il `Mutex` è preso: `async` basta a toglierli dal thread principale della finestra. Il pulsante «Chiudi turno» è già disabilitato durante la chiamata (`app/src/routes/+page.svelte:36`), nessuna modifica alla UI.

- [ ] **Step 1: Riscrivi i test che toccavano i campi `pub`**

I test di `partita.rs` che fanno `dati.manopole.get_mut(...)` (`crates/sim-core/tests/partita.rs:129-147`) passano dal testo: `f64_identici_attraverso_salva_carica` riscrive `soglia_minaccia` in `manopole.toml` con `min = 0.0`, `max = 100.0`, `valore = 94.52706955539223` e carica con `DatiGioco::da_testi`; `salva_rifiuta_manopola_non_finita` diventa `manopola_nan_rifiutata_al_caricamento` (TOML ammette `nan`: `valore = nan` dà `Errore::FuoriIntervallo` col nome). In `dati.rs` i test leggono con gli accessori (`d.dicasteri().len()`, `d.manopole()["soglia_minaccia"]`). `schema_diverso_rifiutato` attende `atteso 2`.

- [ ] **Step 2: Scrivi i test nuovi del caso**

```rust
// crates/sim-core/tests/caso.rs
#[test]
fn normale_ha_media_zero_e_varianza_uno() {
    let mut c = Caso::nuovo(42);
    let n = 20_000;
    let xs: Vec<f64> = (0..n).map(|_| c.normale(Flusso::Economia)).collect();
    let media = xs.iter().sum::<f64>() / n as f64;
    let varianza = xs.iter().map(|x| (x - media).powi(2)).sum::<f64>() / n as f64;
    assert!(media.abs() < 0.03, "{media}");
    assert!((varianza - 1.0).abs() < 0.05, "{varianza}");
}

#[test]
fn uniforme_mai_zero_e_mai_sopra_uno() {
    let mut c = Caso::nuovo(0);
    for _ in 0..10_000 {
        let u = c.uniforme(Flusso::Economia);
        assert!(u > 0.0 && u <= 1.0);
    }
}

#[test]
fn normale_valore_fissato() {
    assert_eq!(Caso::nuovo(42).normale(Flusso::Economia).to_bits(), NORMALE_FISSATA);
}

#[test]
fn il_seme_sta_nel_caso() {
    assert_eq!(Caso::nuovo(7).seme(), 7);
}
```

`NORMALE_FISSATA` è una `const u64` messa a `0` al primo giro e poi copiata dal fallimento, come `VALORE_FISSATO` nel piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:151-154`). In `partita.rs` aggiungi:

```rust
#[test]
fn il_salvataggio_non_ripete_il_seme() {
    let json: serde_json::Value =
        serde_json::from_str(&giocata(5, 1).salva().unwrap()).unwrap();
    assert!(json.get("seme").is_none());
    assert_eq!(json["caso"]["seme"], 5);
}
```

e togli `campo_obbligatorio_mancante_rifiutato` su `seme` sostituendolo con lo stesso controllo sul campo `turno`.

- [ ] **Step 3: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core`
Expected: errori di compilazione (`uniforme`, `normale`, `seme`, accessori non esistono).

- [ ] **Step 4: Implementa**

Accessori e campi privati in `dati.rs`; campo `seme` tolto da `Partita`; `VERSIONE_SCHEMA = 2`; `uniforme`, `normale`, `seme` in `caso.rs`. Nel guscio, `async` sui due comandi; i test di `app/src-tauri/src/lib.rs:75-109` chiamano `avvia` e `chiudi`, che restano sincroni.

- [ ] **Step 5: Fissa il valore della normale ed esegui tutto**

Run: `cargo test --workspace`
Expected: PASS dopo aver copiato `NORMALE_FISSATA`.

- [ ] **Step 6: Commit**

```bash
git add crates app/src-tauri/src/lib.rs
git commit -m "refactor(core): validated-only game data, seed in Caso, normal draws"
```

---

### Task 2: Pipeline di taratura per i dati strutturali

Script offline che scarica le tavole e scrive i file di dati del motore. È la «pipeline di taratura, offline» della spec: fuori dal binario, con le licenze accanto.

**Files:**
- Create: `strumenti/taratura/economia.py`
- Create: `strumenti/taratura/README.md` (fonti, licenze, comando, data dell'interrogazione, modifiche fatte ai dati come chiede la licenza Eurostat)
- Create: `dati/base/economia.toml` (generato)
- Create: `dati/base/regioni.toml` (generato)
- Modify: `dati/base/dicasteri.toml` (aggiunge a ogni dicastero `spesa_pil`, `salari_pil`, `acquisti_pil` e, per `infrastrutture_trasporti`, `settori`)

**Ricerca:** `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:134-141` (FIGARO 2024 per la matrice A, tavola nazionale 2022 di controllo, mix A*10 2023); `docs/ricerca/modelli/economia-italiana-parametri.md:87-93` (flussi SDMX ISTAT `93_1227_DF_DCCN_TNA1_1`, codici A*10, A*10 regionale fino al 2023) e `:54-67` (Okun regionale, tab. 2); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:45-59` (disoccupazione regionale 2025, `lfst_r_lfu3rt`; Trentino-Alto Adige da due province, media pesata sulle forze di lavoro) e `:86-102` (COFOG 2024 per dicastero, `gov_10a_exp`, salari D1, acquisti P2 + D632) e `:130` (piramide ISTAT `22_289_DF_DCIS_POPRES1_1`, chiave `A.IT.JAN.9..99`); `docs/ricerca/dati/input-output-tables.md:24` (nel repo solo la tavola aggregata e lo script); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:171-179` (licenze: ISTAT CC BY 4.0, Eurostat riuso con citazione e dichiarazione delle modifiche).

**Dispatch:** `coder` · in parallelo col Task 1 (nessuna interfaccia consumata, file disgiunti); se i due girano nello stesso worktree, lo Step 3 (`cargo test --workspace`) e il commit vanno dopo il commit del Task 1: la suite vedrebbe il Task 1 a metà, e due commit concorrenti si contendono `index.lock` · skill-gate: true. Script Python offline, nessun `.rs`: non è `rust-engineer`; è una pipeline che scarica e trasforma, non un'analisi che decide se un numero regge, quindi non `data-scientist`. Le risposte delle API sono dati non fidati: si leggono, non si eseguono.

**Interfaces:**
- Consumes: niente dal Task 1.
- Produces (file che il Task 3 legge; nomi dei campi vincolanti):
  - `economia.toml`:
    - `settori`: array di 10 tabelle `{ id, nome }` con gli id A*10 in quest'ordine: `A`, `BTE`, `F`, `GTI`, `J`, `K`, `L`, `MN`, `OTQ`, `RTU`;
    - `a`: array di 10 righe da 10 `f64`, una riga per linea, `a[i][j]` = input interno del settore `i` per unità di produzione del settore `j` (colonne = settore che compra, come in `docs/ricerca/modelli/input-output-leontief.md:12`);
    - `importazioni`: 10 `f64` su una sola riga che comincia con `importazioni = [`, input intermedi importati per unità di produzione di ogni settore;
    - `valore_aggiunto`: 10 `f64`, definito come `1 − Σ_i a[i][j] − importazioni[j]` (le imposte nette sui prodotti finiscono qui);
    - `consumi`, `investimenti`, `esportazioni`, `spesa_pubblica`: 10 `f64` ciascuno, la composizione per settore delle colonne di domanda finale P3_S14+S15, P51G, P6, P3_S13 della stessa tavola; ogni vettore somma a 1;
    - `piramide`: 101 interi, residenti per età da 0 a 99 più la classe 100 e oltre, al 1° gennaio 2026;
    - `anno_tavola`: l'anno della tavola usata.
  - `regioni.toml`: 20 `[[regione]]` con `id` (snake_case senza accenti, Trentino-Alto Adige = `trentino_alto_adige`), `nome`, `macroarea` (`nord` | `centro` | `mezzogiorno`), `forze_lavoro` (migliaia, 2025), `disoccupazione_2025` (%), `okun` (il `b` della tab. 2 di `economia-italiana-parametri.md:56-65`; per il Trentino-Alto Adige -0,141), `mix` (tabella `{ A = …, BTE = …, … }` con le quote di valore aggiunto 2023, somma 1).
  - `dicasteri.toml`: `spesa_pil`, `salari_pil`, `acquisti_pil` = colonne «Spesa 2024», «Salari D1», «Acquisti P2 + D632» della tabella di `stato-del-paese-al-turno-zero.md:88-99` divise per il PIL 2024 e moltiplicate per 100 (percentuali di PIL); `infrastrutture_trasporti` riceve `settori = { … }` uguale al vettore `investimenti` [assunzione del piano 3: le opere vanno in costruzioni e beni d'investimento]. Gli altri dicasteri usano il vettore `spesa_pubblica`.

Scelte di metodo dello script: tavola FIGARO 2024 (blocco Italia → Italia per gli input interni, righe estere sommate per `importazioni`) se l'API la dà filtrata per paese; altrimenti la simmetrica nazionale `naio_10_cp1700` 2022, dichiarandolo nel README e in `anno_tavola`. Aggregazione 64 → A*10 per somma dei codici CPA. Le quote sono arrotondate a 6 decimali e poi corrette sull'ultima voce perché ogni vettore sommi esattamente a 1.

- [ ] **Step 1: Scrivi lo script**

`python3 -I strumenti/taratura/economia.py` legge con `urllib.request` dalle API citate, senza librerie esterne, e scrive i tre file. Ogni file generato comincia con un commento: comando, data dell'interrogazione, fonti, licenza.

- [ ] **Step 2: Esegui lo script e controlla le invarianti**

Run: `python3 -I strumenti/taratura/economia.py`
Expected: i tre file scritti; lo script stampa e controlla con `assert`: 10 settori, ogni colonna di `a` con somma + importazioni < 1, ogni vettore e ogni `mix` con somma 1 entro 1e-9, 20 regioni, disoccupazione nazionale 2025 pesata sulle forze di lavoro fra 6,0 e 6,2 (il 6,1 di `stato-del-paese-al-turno-zero.md:57`), somma di `spesa_pil` dei dieci dicasteri pari a 575,6 miliardi sul PIL 2024 (entro 0,1 punti).

- [ ] **Step 3: Verifica che i dati di base si carichino ancora**

Run: `cargo test --workspace`
Expected: PASS. I campi nuovi di `dicasteri.toml` sono ignorati dal parser attuale finché il Task 3 non li legge.

- [ ] **Step 4: Commit**

```bash
git add strumenti dati/base
git commit -m "feat(data): offline calibration pipeline for I-O, regions, budgets"
```

---

### Task 3: Struttura, scenario di base e stato iniziale dell'economia

**Files:**
- Create: `crates/sim-core/src/struttura.rs` (tipi e validazione di `economia.toml` e `regioni.toml`)
- Create: `crates/sim-core/src/scenario.rs` (tipi e validazione di `scenario.toml`)
- Create: `crates/sim-core/src/economia/mod.rs` (`Economia`, `Economia::iniziale`, `Economia::passo` con l'ordine dei blocchi)
- Create: `dati/base/scenario.toml`
- Modify: `dati/base/manopole.toml` (tutte le manopole del piano, tabella sotto)
- Modify: `crates/sim-core/src/dati.rs` (`FileDati` a cinque file, `FileDati::NOMI`, `FileDati::leggi`, `Dicastero` coi campi di bilancio, accessori `struttura`, `regioni`, `scenario`, controllo delle manopole richieste)
- Modify: `crates/sim-core/src/errore.rs` (varianti nuove)
- Modify: `crates/sim-core/src/partita.rs` (campo `economia`, `Partita::economia`, `chiudi_turno` chiama `economia.passo`)
- Modify: `crates/sim-core/src/lib.rs`
- Modify: `crates/sim-cli/src/main.rs`, `app/src-tauri/src/lib.rs` (leggono con `FileDati::leggi`)
- Modify: `crates/sim-core/tests/comune/mod.rs` (cinque file; `senza_shock`)
- Test: `crates/sim-core/tests/scenario.rs`

**Ricerca:** spec, «Modulo scenario» (righe 274-285: solo ingressi, derivati ricalcolati, valori di partenza); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:14-41` (livelli del turno zero) e `:167` (stato base al 2026T2 portato a ottobre); `docs/ricerca/sintesi/decisioni.md:250` (solo ingressi); `docs/ricerca/modelli/regole-fiscali-ue.md:46-49` (tetti, PDE sospesa, conto, clausola); `docs/ricerca/modelli/economia-italiana-parametri.md:117-121` (forme per il gioco); `docs/ricerca/tecnica/modding-e-dati-esterni.md:14-15`.

**Dispatch:** `rust-engineer` · dopo Task 1 e Task 2: consuma `DatiGioco` coi campi privati (Task 1) e i file generati (Task 2); tocca `dati.rs`, `partita.rs`, `app/src-tauri/src/lib.rs` già toccati dal Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `DatiGioco` coi campi privati (Task 1); i file del Task 2.
- Produces:
  - `FileDati { pub dicasteri, pub manopole, pub economia, pub regioni, pub scenario: String }`; `FileDati::NOMI: [&str; 5] = ["dicasteri.toml", "manopole.toml", "economia.toml", "regioni.toml", "scenario.toml"]`; `FileDati::leggi<E>(leggi: impl FnMut(&str) -> Result<String, E>) -> Result<FileDati, E>`. Il CLI e il guscio passano la loro lettura da disco: è l'estrazione rimandata dal piano 1 all'arrivo del terzo file.
  - `Dicastero` aggiunge `pub spesa_pil: f64`, `pub salari_pil: f64`, `pub acquisti_pil: f64`, `#[serde(default)] pub settori: Option<BTreeMap<String, f64>>`.
  - `sim_core::struttura::{Struttura, Settore, Regione, Macroarea}` coi campi del Task 2, tutti `pub` (raggiungibili solo in lettura da `&DatiGioco`). `DatiGioco::struttura(&self) -> &Struttura`, `DatiGioco::regioni(&self) -> &[Regione]`.
  - `sim_core::scenario::{Scenario, Ingresso, StatoPde, RegimePensioni, RegimeSussidi}`. `Ingresso { pub valore: f64, pub min: f64, pub max: f64, pub fonte: String }`; in `scenario.toml` ogni ingresso è una tabella `[nome]` la cui prima riga è `valore = …`, poi `min`, `max`, `fonte`. `StatoPde` — `enum { Nessuna, Aperta, Sospesa, Inadempiente, Intimata, Sanzionata }`, serde `snake_case`. `Scenario` ha i campi della tabella sotto, ognuno un `Ingresso` salvo dove indicato. `DatiGioco::scenario(&self) -> &Scenario`.
  - Varianti nuove di `Errore`, testo esatto:
    - `ScenarioFuoriIntervallo { nome: String, valore: f64, min: f64, max: f64 }` → `valore dello scenario fuori intervallo: {nome} = {valore} (ammesso da {min} a {max})`
    - `ManopolaMancante { nome: String }` → `manopola mancante: {nome}`
    - `SettoreSconosciuto { id: String, dove: String }` → `settore sconosciuto: {id} (in {dove})`
    - `SettoreNonProduttivo { id: String }` → `settore che consuma più di quanto produce: {id}`
    - `QuoteNonUnitarie { dove: String }` → `quote che non sommano a 1: {dove}`
    - `StrutturaIncoerente(String)` → `struttura economica incoerente: {0}`
  - `sim_core::economia::Economia` — `Clone, Debug, PartialEq, Serialize, Deserialize`, campi `pub`; `Economia::iniziale(dati: &DatiGioco) -> Economia`. `pub(crate) fn passo(&mut self, dati: &DatiGioco, caso: &mut Caso, data: Trimestre)` nasce nel Task 4 col primo blocco (nessuna funzione vuota, come nel piano 1), e porta sopra di sé il commento con l'ordine dei blocchi della spec (riga 289), che i task successivi riempiono: misure (piano 5) → finanza pubblica e spesa obbligatoria → domanda finale per settore → Leontief → IS/Phillips → Okun per regione → redditi dei gruppi e panieri (piano 4) → tasso BCE → SFC → spread, regole UE. `passo(data)` simula il trimestre del turno che si chiude.
  - Campi di `Economia` creati qui (gli altri task ne aggiungono): `pil_trimestrale: f64` (miliardi, nominale), `pil_potenziale_reale: f64` (miliardi a prezzi del turno zero), `livello_prezzi: f64` (1 al turno zero), `output_gap: [f64; 2]` (corrente, precedente), `inflazione: [f64; 8]` (annualizzata, dalla più recente), `tasso_bce: f64`, `regioni: Vec<EconomiaRegione>` con `EconomiaRegione { pub id: String, pub disoccupazione: f64, pub disoccupazione_strutturale: f64, pub gap: f64 }`.
  - `Partita` aggiunge il campo `economia: Economia`; `Partita::economia(&self) -> &Economia` (stato vero: per il CLI e i test, mai per la `Vista`).
  - Helper di test in `tests/comune/mod.rs`, riscrittura riga per riga del testo, senza dipendenze:
    - `senza_shock() -> FileDati`: come `file_base()`, con `valore = 0.0` in ogni sezione di `manopole.toml` il cui nome comincia con `sigma_`;
    - `con_ingresso(f: FileDati, nome: &str, valore: f64) -> FileDati`: sostituisce la riga `valore = …` subito sotto `[nome]` in `f.scenario`; va in panico se la sezione non c'è;
    - `sostituisci(f: FileDati, file: &str, da: &str, a: &str) -> FileDati`: `replacen(da, a, 1)` nel campo di `FileDati` di nome `file` (`"dicasteri"`, `"manopole"`, `"economia"`, `"regioni"`, `"scenario"`); va in panico se `da` non c'è.

**Validazione al caricamento**, in quest'ordine dopo quella del piano 1: manopole richieste presenti (`ManopolaMancante`, elenco = tabella sotto); `settori` esattamente 10, `a` 10×10, ogni vettore lungo 10 (`StrutturaIncoerente`); per ogni colonna `j`, `Σ_i a[i][j] + importazioni[j] < 1` (`SettoreNonProduttivo` con l'id); ogni vettore di domanda, ogni `mix` di regione e ogni `settori` di dicastero somma a 1 entro 1e-6 (`QuoteNonUnitarie` con `dove` = nome del vettore, della regione o del dicastero); ogni chiave di `mix` e di `settori` è un id di settore (`SettoreSconosciuto`); ogni ingresso dello scenario nel suo intervallo (`ScenarioFuoriIntervallo`); `piramide` lunga 101.

**Scenario di base** (`dati/base/scenario.toml`), intervalli di taratura provvisori al bilanciamento:

| campo | valore | min | max | fonte |
|---|---|---|---|---|
| `pil_trimestrale` (mld €) | 576.9 | 400 | 800 | stato-del-paese:18, PIL 2026T2 destagionalizzato |
| `debito_pil` (%) | 136.7 | 60 | 200 | stato-del-paese:23 |
| `costo_medio_debito` (%) | 2.81 | 0.5 | 6 | stato-del-paese:31 |
| `vita_media_debito` (anni) | 7.8 | 4 | 12 | stato-del-paese:32 |
| `tasso_bce` (%) | 2.5 | -0.5 | 6 | stato-del-paese:35 |
| `inflazione` (%) | 4.2 | -2 | 12 | stato-del-paese:36, settembre |
| `inflazione_t3` (%) | 3.4 | -2 | 12 | stato-del-paese:36, media T3 |
| `inflazione_t2` (%) | 3.0 | -2 | 12 | stato-del-paese:36, media T2 |
| `output_gap` (%) | 0.0 | -5 | 5 | assunzione del piano 3, nessuna stima in docs/ricerca |
| `disoccupazione` (%) | 5.6 | 3 | 15 | stato-del-paese:37, T2 2026 |
| `consumi_famiglie_pil` (%) | 58.76 | 40 | 75 | stato-del-paese:19, 339,0 / 576,9 |
| `investimenti_pil` (%) | 22.62 | 10 | 35 | stato-del-paese:21 |
| `esportazioni_pil` (%) | 32.95 | 15 | 50 | stato-del-paese:22 |
| `importazioni_pil` (%) | 32.19 | 15 | 50 | stato-del-paese:22 |
| `irpef_pil` (%) | 12.06 | 6 | 20 | stato-del-paese:39, 273,1 / 2.265,0 |
| `ires_pil` (%) | 2.68 | 1 | 6 | stato-del-paese:39 |
| `iva_pil` (%) | 6.64 | 3 | 12 | stato-del-paese:39 |
| `contributi_pil` (%) | 13.31 | 8 | 20 | stato-del-paese:40 |
| `altre_entrate_pil` (%) | residuo di taratura | 0 | 25 | calcolato una volta perché il disavanzo annuo del turno zero sia 2,9% (stato-del-paese:27, previsione 2026), scritto come numero |
| `fondi_coesione_pil` (%) | 0.26 | 0 | 1 | assunzione del piano 3, nessuna cifra in docs/ricerca |
| `pensionati` (persone) | 16388149 | 10e6 | 25e6 | stato-del-paese:119 |
| `spesa_pensioni` (mld € annui) | 351.8 | 200 | 500 | stato-del-paese:116, previsione 2026 |
| `eta_pensione` (anni) | 67 | 60 | 72 | regime vigente; `piramide` dà gli ingressi |
| `indicizzazione_pensioni` (quota dell'inflazione) | 1.0 | 0 | 1 | regime vigente |
| `pensione_minima` (€ annui) | 7800 | 0 | 20000 | assunzione del piano 3, nessuna cifra in docs/ricerca |
| `spesa_sussidi` (mld € annui) | 17.93 | 5 | 50 | stato-del-paese:123, NASpI 2025 |
| `altre_prestazioni` (mld € annui) | 98.4 | 50 | 200 | stato-del-paese:117, 116,3 meno la NASpI |
| `conto_controllo` (% PIL) | 0.1 | -3 | 3 | regole-fiscali-ue:47 |
| `clausola_nazionale_margine` (% PIL) | 0.6 | 0 | 1.5 | regole-fiscali-ue:49, 0,3 difesa + 0,3 energia |
| `premio_sfiducia` (pb) | 0 | 0 | 400 | economia-italiana-parametri:117, nessun evento al turno zero |
| `energia` (% del prezzo degli input importati) | 0 | -50 | 100 | nessuno scostamento al turno zero: lo shock in corso sta già nell'inflazione |
| `domanda_estera` (%) | 0 | -30 | 30 | nessuno scostamento al turno zero |

Campi non `Ingresso`: `regime_pensioni = "contributivo"` (`Retributivo | Contributivo | QuotaBaseUniversale`, serde `snake_case`); `regime_sussidi = "assicurativo"` (`Assicurativo | Universale`); `pde = "sospesa"`; `tetti_spesa_netta = { 2026 = 1.6, 2027 = 1.9, 2028 = 1.7, 2029 = 1.5 }` (regole-fiscali-ue:46; dopo il 2029 vale l'ultimo); `clausola_nazionale_fino_a = 2028`.

**Manopole** aggiunte a `dati/base/manopole.toml` (tutte provvisorie, al bilanciamento; «A» = commento `# assunzione del piano 3, nessuna fonte in docs/ricerca`):

| nome | valore | min | max | ticket | fonte |
|---|---|---|---|---|---|
| `is_gap_1` | 0.70 | 0.5 | 0.95 | 07 | economia-italiana-parametri:75 |
| `is_gap_2` | 0.14 | 0.0 | 0.3 | 07 | idem |
| `is_tasso` | -0.056 | -0.2 | 0.0 | 07 | idem |
| `phillips_1` | 1.18 | 1.0 | 1.4 | 07 | economia-italiana-parametri:76 |
| `phillips_2_4` | -0.28 | -0.5 | 0.0 | 07 | idem |
| `phillips_5_8` | 0.10 | 0.0 | 0.3 | 07 | idem |
| `phillips_gap` | 0.051 | 0.0 | 0.2 | 07 | idem |
| `ancoraggio_aspettative` | 0.10 | 0.0 | 0.5 | 07 | A (peso fisso dell'obiettivo del 2%, spec riga 293) |
| `tasso_naturale_reale` | 0.25 | -1.0 | 2.0 | 07 | A |
| `crescita_potenziale` (% annuo) | 0.6 | 0.0 | 2.0 | 07 | A |
| `sigma_domanda` (punti di gap) | 0.4 | 0.0 | 2.0 | 07 | A (residui storici [NON TROVATO]) |
| `sigma_produttivita` (punti di crescita annua) | 0.2 | 0.0 | 1.0 | 07 | A |
| `sigma_energia` (% del prezzo degli input importati) | 3.0 | 0.0 | 15.0 | 07 | A |
| `persistenza_energia` | 0.7 | 0.0 | 0.95 | 07 | A |
| `sigma_domanda_estera` (%) | 1.0 | 0.0 | 5.0 | 07 | A |
| `persistenza_domanda_estera` | 0.7 | 0.0 | 0.95 | 07 | A |
| `sigma_bce` (punti) | 0.1 | 0.0 | 0.5 | 07 | A |
| `quota_italia_area_euro` | 0.142 | 0.10 | 0.18 | 07 | economia-italiana-parametri:110 |
| `taylor_inerzia` | 0.85 | 0.5 | 0.95 | 07 | banca-centrale-e-inflazione:13 |
| `taylor_inflazione` | 0.5 | 0.0 | 1.5 | 07 | idem |
| `taylor_gap` | 0.5 | 0.0 | 1.5 | 07 | idem |
| `persistenza_resto_area` | 0.8 | 0.0 | 0.95 | 07 | A |
| `okun_nazionale` | -0.254 | -0.5 | -0.1 | 07 | economia-italiana-parametri:52 |
| `isteresi_disoccupazione` (per turno) | 0.05 | 0.0 | 0.2 | 07 | A (spec riga 297, nessun numero) |
| `moltiplicatore_fiscale` | 0.8 | 0.3 | 1.5 | 07 | A ([NON TROVATO] in docs/ricerca) |
| `elasticita_irpef` | 1.46 | 0.5 | 3.0 | 07 | economia-italiana-parametri:104 |
| `elasticita_ires` | 3.07 | 1.0 | 5.0 | 07 | idem |
| `elasticita_iva` | 1.10 | 0.5 | 2.0 | 07 | idem |
| `elasticita_contributi` | 0.58 | 0.2 | 1.5 | 07 | idem |
| `quota_margine` (% del budget) | 1.6 | 0.5 | 5.0 | 23 | stato-del-paese:104; spec riga 278 |
| `quota_redditi_lavoro` (quota del PIL) | 0.40 | 0.30 | 0.55 | 07 | A |
| `quota_profitti_distribuiti` | 0.6 | 0.0 | 1.0 | 07 | A |
| `elasticita_export_prezzi` | 1.0 | 0.0 | 3.0 | 07 | A (spec riga 294, nessun numero) |
| `sostituzione_retributivo` | 0.75 | 0.4 | 1.0 | 07 | A |
| `sostituzione_contributivo` | 0.60 | 0.3 | 0.9 | 07 | A |
| `quota_base_universale` (€ annui) | 6000 | 0 | 15000 | 07 | A |
| `uscite_pensionati` (quota annua) | 0.055 | 0.03 | 0.09 | 07 | A |
| `copertura_pensionamento` | calcolata nel Task 8 | 0.5 | 3.0 | 07 | taratura: stock dei pensionati stabile nel primo anno |
| `sussidio_universale` (moltiplicatore della spesa) | 1.6 | 1.0 | 3.0 | 07 | A |
| `spread_s0` (pb) | -883 | -1000 | -700 | 07 | taratura: 84 pb al 136,7% (centro di 79-89, stato-del-paese:34) |
| `spread_beta_1` | -0.0523 | -0.1 | 0.0 | 07 | economia-italiana-parametri:15 |
| `spread_beta_2` | 0.0009 | 0.0 | 0.002 | 07 | idem |
| `spread_minimo` (pb) | 60 | 0 | 150 | 07 | A (tratto piatto, spec riga 295) |
| `spread_disavanzo` (pb per punto oltre il 3%) | 2.4 | 0.0 | 20.0 | 07 | economia-italiana-parametri:21 |
| `emivita_premio_sfiducia` (turni) | 4 | 1 | 12 | 07 | A |
| `premio_termine_bund` (punti) | 0.97 | 0.0 | 2.0 | 07 | stato-del-paese:33-35: 4,36 − 0,89 − 2,50 |
| `deviazione_annua_rilevante` (% PIL) | 0.3 | 0.1 | 1.0 | 02 | regole-fiscali-ue:20 |
| `deviazione_cumulata_rilevante` (% PIL) | 0.6 | 0.2 | 2.0 | 02 | idem |
| `ammenda_pde` (% PIL) | 0.2 | 0.0 | 0.5 | 02 | regole-fiscali-ue:35 |
| `ammenda_semestrale` (% PIL) | 0.05 | 0.0 | 0.2 | 02 | regole-fiscali-ue:37 |
| `sospensione_coesione` (% PIL) | 0.25 | 0.0 | 0.5 | 02 | regole-fiscali-ue:35 |

Le manopole dei Task 7-10 entrano qui, così il file cambia una volta sola; i task che le usano le leggono con `DatiGioco::manopola`.

**Stato iniziale** (`Economia::iniziale`, derivati dagli ingressi):
- `output_gap = [output_gap, output_gap]`; `pil_potenziale_reale = pil_trimestrale / (1 + output_gap/100)`; `livello_prezzi = 1`.
- `inflazione = [inflazione, inflazione_t3, inflazione_t2, 2, 2, 2, 2, 2]` [assunzione del piano 3: i trimestri prima del T2 2026 all'obiettivo della BCE].
- Disoccupazione di ogni regione = `disoccupazione_2025` della regione + (`disoccupazione` − media 2025 pesata sulle forze di lavoro): stesso spostamento in punti per tutte, divario invariato. `disoccupazione_strutturale` = disoccupazione: il cursore della disoccupazione sposta anche u* (spec, tabella del ticket 21). `gap` della regione = `output_gap`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/scenario.rs
mod comune;
use comune::{con_ingresso, dati_base, file_base, sostituisci};
use sim_core::{DatiGioco, Errore, FileDati, Partita};

fn errore(f: FileDati) -> Errore {
    DatiGioco::da_testi(&f).unwrap_err()
}

#[test]
fn dati_base_hanno_dieci_settori_e_venti_regioni() {
    let d = dati_base();
    assert_eq!(d.struttura().settori.len(), 10);
    assert_eq!(d.regioni().len(), 20);
}

#[test]
fn turno_zero_riproduce_i_valori_di_partenza() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let e = p.economia();
    assert_eq!(e.inflazione[0], 4.2);
    assert_eq!(e.tasso_bce, 2.5);
    let pesata: f64 = e.regioni.iter().zip(d.regioni())
        .map(|(r, g)| r.disoccupazione * f64::from(g.forze_lavoro)).sum();
    let tot: f64 = d.regioni().iter().map(|g| f64::from(g.forze_lavoro)).sum();
    assert!((pesata / tot - 5.6).abs() < 1e-9);
}

#[test]
fn colonna_non_produttiva_rifiutata_col_nome() {
    let testo = file_base().economia;
    let riga = testo.lines().find(|l| l.starts_with("importazioni = [")).unwrap().to_string();
    let primo = riga["importazioni = [".len()..].split(',').next().unwrap().trim().to_string();
    let nuova = riga.replacen(&primo, "0.99", 1);
    let e = errore(sostituisci(file_base(), "economia", &riga, &nuova));
    assert_eq!(e, Errore::SettoreNonProduttivo { id: "A".into() });
}

#[test]
fn settore_sconosciuto_nel_dicastero_rifiutato_col_nome() {
    let e = errore(sostituisci(file_base(), "dicasteri", "BTE", "BTX"));
    assert_eq!(e.to_string(), "settore sconosciuto: BTX (in infrastrutture_trasporti)");
}

#[test]
fn mix_regionale_che_non_somma_a_uno_rifiutato() {
    let e = errore(sostituisci(file_base(), "regioni", "A = 0.", "A = 9."));
    assert!(matches!(e, Errore::QuoteNonUnitarie { .. }), "{e}");
}

#[test]
fn ingresso_dello_scenario_fuori_intervallo_rifiutato_col_nome() {
    let e = errore(con_ingresso(file_base(), "debito_pil", 250.0));
    assert!(e.to_string().starts_with("valore dello scenario fuori intervallo: debito_pil = 250"), "{e}");
}

#[test]
fn manopola_mancante_rifiutata_col_nome() {
    let e = errore(sostituisci(file_base(), "manopole", "[okun_nazionale]", "[okun_nazionalx]"));
    assert_eq!(e, Errore::ManopolaMancante { nome: "okun_nazionale".into() });
}

#[test]
fn leggi_chiede_i_cinque_file_per_nome() {
    let mut chiesti = Vec::new();
    let _ = FileDati::leggi(|nome| {
        chiesti.push(nome.to_string());
        Ok::<_, ()>(String::new())
    });
    assert_eq!(chiesti, FileDati::NOMI);
}
```

`"BTE"` compare in `dicasteri.toml` solo nei `settori` di `infrastrutture_trasporti`; `"A = 0."` è la quota del settore A nel `mix` della prima regione. Le manopole in più rispetto all'elenco non sono un errore.

Nei test del CLI (`crates/sim-cli/tests/cli.rs`) e del guscio, la cartella dei dati ora contiene cinque file: i test esistenti restano invariati e devono passare.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test scenario`
Expected: errore di compilazione, `struttura`, `scenario`, `economia` non esistono.

- [ ] **Step 3: Implementa tipi, validazione, stato iniziale e lettura nel CLI e nel guscio**

`Economia` entra in `Partita::nuova` con `Economia::iniziale(&dati)`; dal Task 4 `chiudi_turno` chiama `self.economia.passo(&self.dati, &mut self.caso, self.data)` prima di avanzare il calendario. `carica` non ricalcola nulla: lo stato arriva dal salvataggio.

- [ ] **Step 4: Esegui tutti i test e il controllo di purezza**

Run: `cargo test --workspace && grep -rnE "std::(fs|time|thread|net|env)|HashMap|StdRng|SmallRng" crates/sim-core/src`
Expected: PASS; il grep non stampa righe.

- [ ] **Step 5: Commit**

```bash
git add dati crates app/src-tauri/src/lib.rs
git commit -m "feat(core): load economic structure, base scenario and initial state"
```

---

### Task 4: Leontief, produzione e prezzi per settore

**Files:**
- Create: `crates/sim-core/src/economia/leontief.rs`
- Modify: `crates/sim-core/src/economia/mod.rs` (nasce `passo`, coi passi «domanda finale per settore» e «Leontief»; campi nuovi)
- Modify: `crates/sim-core/src/partita.rs` (`chiudi_turno` chiama `passo`)
- Test: `crates/sim-core/tests/leontief.rs`, `crates/sim-core/tests/economia.rs`

**Ricerca:** `docs/ricerca/modelli/input-output-leontief.md:12-15` (quantità `x = (I−A)⁻¹ d`, prezzi `p = (I−Aᵀ)⁻¹ v`, Hawkins-Simon) e `:39-42` (A fissa, nessun razionamento); oracolo dell'esempio a due settori carbone-acciaio della fonte `docs/ricerca/fonti/wikipedia-input-output-model.md:41-74` (gitignorata: c'è solo nel checkout principale, non nel worktree; i numeri che servono sono già qui) (A = [[0,5, 0,2], [0,4, 0,1]], d = (7, 4), (I−A)⁻¹ = (1/0,37)·[[0,9, 0,2], [0,4, 0,5]]); spec riga 291.

**Dispatch:** `rust-engineer` · dopo Task 3: `economia/mod.rs` e `partita.rs` del Task 3, manopole `sigma_` scritte dal Task 3 · skill-gate: true.

**Interfaces:**
- Consumes: `Struttura` (Task 3), `Caso::normale` (Task 1).
- Produces:
  - `pub fn leontief::produzione(a: &[Vec<f64>], domanda: &[f64]) -> Vec<f64>`: risolve `(I − A) x = d` per eliminazione di Gauss con pivot parziale. È la seconda funzione pura esposta dopo il riparto, per la stessa ragione: ha un oracolo esterno esatto.
  - `pub fn leontief::prezzi(a: &[Vec<f64>], costo_unitario: &[f64]) -> Vec<f64>`: risolve `(I − Aᵀ) p = c`.
  - Campi nuovi di `Economia`: `energia: f64` (scostamento % del prezzo degli input importati, AR(1)), `domanda_estera: f64` (scostamento %, AR(1)), entrambi iniziati dagli ingressi omonimi dello scenario; `prezzi_settori: Vec<f64>` (indici, 1 al turno zero), `gap_settori: Vec<f64>` (punti), `inflazione_importata: f64` (punti annualizzati, l'ultima).

**Il passo, in quest'ordine:**
1. Shock del turno, estratti tutti e sempre in quest'ordine da `Flusso::Economia`: `domanda`, `produttivita`, `energia`, `domanda_estera`, `bce`. Ognuno è `caso.normale(Flusso::Economia) * sigma_…`. Il Task 4 li estrae tutti e cinque e usa `energia` e `domanda_estera`; il Task 5 usa gli altri tre.
2. `energia ← persistenza_energia · energia + shock_energia`; `domanda_estera ← persistenza_domanda_estera · domanda_estera + shock_domanda_estera`.
3. Domanda finale di riferimento, in miliardi per trimestre a prezzi del turno zero: `d0 = consumi·C0 + investimenti·I0 + esportazioni·X0 + Σ_dicasteri vettore_d·spesa_d`, con `C0 = consumi_famiglie_pil/100·pil_trimestrale` (iniziale), `I0` e `X0` allo stesso modo, `spesa_d = spesa_pil/100·pil_trimestrale`; `vettore_d` è `settori` del dicastero se c'è, altrimenti `spesa_pubblica`. Domanda del turno `d = d0 + esportazioni·X0·domanda_estera/100` (il Task 7 aggiunge lo scostamento reale della spesa dei dicasteri).
4. `x = produzione(a, d)`, `x0 = produzione(a, d0)`; `gap_settori[s] = output_gap_corrente + 100·(x[s]/x0[s] − Σx/Σx0)`. Il Task 5 ricalcola questo campo dopo la IS: qui serve la scomposizione.
5. `prezzi_settori = prezzi(a, valore_aggiunto + importazioni·(1 + energia/100))`; `inflazione_importata = 400·(P_c(t)/P_c(t−1) − 1)` con `P_c` = media dei prezzi pesata col vettore `consumi`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/leontief.rs
use sim_core::economia::leontief::{prezzi, produzione};

fn vicino(a: f64, b: f64) -> bool { (a - b).abs() < 1e-9 }

#[test]
fn esempio_carbone_acciaio() {
    let a = vec![vec![0.5, 0.2], vec![0.4, 0.1]];
    let x = produzione(&a, &[7.0, 4.0]);
    assert!(vicino(x[0], 7.1 / 0.37) && vicino(x[1], 4.8 / 0.37), "{x:?}");
}

#[test]
fn prezzi_unitari_quando_il_costo_chiude_ogni_colonna() {
    let a = vec![vec![0.5, 0.2], vec![0.4, 0.1]];
    let p = prezzi(&a, &[1.0 - 0.9, 1.0 - 0.3]);
    assert!(vicino(p[0], 1.0) && vicino(p[1], 1.0), "{p:?}");
}

#[test]
fn matrice_nulla_restituisce_la_domanda() {
    let a = vec![vec![0.0; 3]; 3];
    assert_eq!(produzione(&a, &[1.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
}
```

```rust
// crates/sim-core/tests/economia.rs
mod comune;
use comune::{con_ingresso, dati_base, senza_shock};
use sim_core::{DatiGioco, FileDati, Partita};

fn giocata(f: &FileDati, seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(f).unwrap(), seme);
    for _ in 0..turni { p.chiudi_turno(); }
    p
}

#[test]
fn senza_shock_i_prezzi_dei_settori_restano_a_uno() {
    let p = giocata(&senza_shock(), 1, 4);
    for prezzo in &p.economia().prezzi_settori { assert!((prezzo - 1.0).abs() < 1e-9); }
    assert_eq!(p.economia().inflazione_importata, 0.0);
}

#[test]
fn uno_shock_energetico_alza_di_piu_i_prezzi_dell_industria() {
    let p = giocata(&con_ingresso(senza_shock(), "energia", 20.0), 1, 1);
    let e = p.economia();
    let (bte, l) = (e.prezzi_settori[1], e.prezzi_settori[6]);
    assert!(bte > l && l >= 1.0, "BTE {bte}, L {l}");
    assert!(e.inflazione_importata > 0.0);
}
```

Gli indici 1 e 6 sono `BTE` e `L` nell'ordine dei settori del Task 2. Il test usa `dati_base` solo nei task successivi: togli l'import se il compilatore lo segnala inutilizzato.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test leontief --test economia`
Expected: errore di compilazione.

- [ ] **Step 3: Implementa `leontief.rs` e i due passi in `mod.rs`**

- [ ] **Step 4: Esegui tutti i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati
git commit -m "feat(core): Leontief production and price blocks"
```

---

### Task 5: Congiuntura, PIL potenziale e BCE

**Files:**
- Create: `crates/sim-core/src/economia/congiuntura.rs` (IS, Phillips, PIL potenziale)
- Create: `crates/sim-core/src/economia/bce.rs` (Taylor sull'area euro)
- Modify: `crates/sim-core/src/economia/mod.rs`
- Test: `crates/sim-core/tests/economia.rs`, `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/modelli/economia-italiana-parametri.md:73-79` (IS e Phillips di Garnier-Wilhelmsen per l'area euro, forma e coefficienti) e `:121` (primo default per l'Italia); `docs/ricerca/modelli/macro-neokeynesiano-piccolo.md:17-22` (RS, verticalità della Phillips, Taylor); `docs/ricerca/modelli/banca-centrale-e-inflazione.md:13` (Taylor della Fed con inerzia 0,85) e `:39` (BCE sull'area euro, Italia pesata per quota di PIL); `docs/ricerca/modelli/economia-italiana-parametri.md:110-113` (14,2%, quota di PIL e non chiave di capitale); spec righe 292-293.

**Dispatch:** `rust-engineer` · dopo Task 4: `economia/mod.rs`, `tests/economia.rs`, shock estratti nel Task 4 · skill-gate: true.

**Interfaces:**
- Consumes: shock `domanda`, `produttivita`, `bce` estratti nel Task 4; `inflazione_importata`, `domanda_estera` del Task 4.
- Produces: campi nuovi di `Economia`: `area_euro: AreaEuro { pub inflazione_resto: f64, pub gap_resto: f64 }` (iniziali: inflazione del turno zero, gap 0 [assunzione del piano 3]); `impulso_fiscale: f64` (punti di PIL potenziale, 0 fino al Task 7). Equazioni, con `y` = output gap, `π` = inflazione annualizzata, `i` = tasso BCE, `r̃_t = i_t − π_t − tasso_naturale_reale`:
  - IS: `y_t = is_gap_1·y_{t−1} + is_gap_2·y_{t−2} + is_tasso·(r̃_{t−1} + r̃_{t−2}) + moltiplicatore_fiscale·impulso_fiscale + esportazioni_pil/100·(domanda_estera_t − domanda_estera_{t−1}) + shock_domanda`. `r̃_{t−2}` usa il tasso e l'inflazione salvati del turno prima: aggiungi `tasso_reale_precedente: f64` allo stato.
  - Phillips: `π_t = (1 − ancoraggio)·[phillips_1·π_{t−1} + phillips_2_4·media(π_{t−2..t−4}) + phillips_5_8·media(π_{t−5..t−8})] + ancoraggio·2 + phillips_gap·y_{t−1} + inflazione_importata`.
  - PIL potenziale: `pil_potenziale_reale ← pil_potenziale_reale·(1 + (crescita_potenziale + shock_produttivita)/400)`. `livello_prezzi ← livello_prezzi·(1 + π_t/400)`. `pil_trimestrale = pil_potenziale_reale·(1 + y_t/100)·livello_prezzi`.
  - `gap_settori` ricalcolato con lo `y_t` nuovo (Task 4, punto 4).
  - Resto dell'area: `inflazione_resto ← 2 + persistenza_resto_area·(inflazione_resto − 2)`; `gap_resto ← persistenza_resto_area·gap_resto`.
  - BCE, dopo Okun nell'ordine della spec: `π_ea = q·π_t + (1 − q)·inflazione_resto`, `y_ea = q·y_t + (1 − q)·gap_resto` con `q = quota_italia_area_euro`; `i* = tasso_naturale_reale + π_ea + taylor_inflazione·(π_ea − 2) + taylor_gap·y_ea`; `i_t = taylor_inerzia·i_{t−1} + (1 − taylor_inerzia)·i* + shock_bce`. Nessun limite inferiore: col tasso di partenza al 2,5% non serve [assunzione del piano 3].

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// in crates/sim-core/tests/economia.rs
#[test]
fn senza_shock_l_inflazione_torna_verso_il_due_per_cento() {
    let pi = giocata(&senza_shock(), 1, 40).economia().inflazione[0];
    assert!((pi - 2.0).abs() < 0.5, "{pi}");
}

#[test]
fn inflazione_sopra_l_obiettivo_fa_salire_il_tasso_bce() {
    assert!(giocata(&senza_shock(), 1, 1).economia().tasso_bce > 2.5);
}

#[test]
fn il_gap_si_riassorbe_senza_shock() {
    let p = giocata(&con_ingresso(senza_shock(), "output_gap", -3.0), 1, 60);
    assert!(p.economia().output_gap[0].abs() < 0.5, "{:?}", p.economia().output_gap);
}

#[test]
fn il_pil_nominale_cresce_con_prezzi_e_potenziale() {
    assert!(giocata(&senza_shock(), 1, 4).economia().pil_trimestrale > 576.9);
}

#[test]
fn semi_diversi_economie_diverse() {
    let f = comune::file_base();
    assert_ne!(giocata(&f, 1, 4).economia(), giocata(&f, 2, 4).economia());
}
```

```rust
// in crates/sim-core/tests/partita.rs
// Primo test che estrae dal Caso prima e dopo salva/carica (rimando della PR #15).
#[test]
fn estrazioni_identiche_dopo_salva_carica() {
    let mut diretta = giocata(42, 3);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    for _ in 0..5 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.economia(), diretta.economia());
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
    assert_ne!(giocata(42, 8).economia(), giocata(43, 8).economia(), "il caso deve essere stato estratto");
}
```

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test economia --test partita`
Expected: FAIL (i campi non si muovono ancora, `tasso_bce` resta 2,5).

- [ ] **Step 3: Implementa i due blocchi e il loro posto nell'ordine**

- [ ] **Step 4: Esegui tutti i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): IS-Phillips cycle, potential output and ECB Taylor rule"
```

---

### Task 6: Okun per regione e stato vero nel CLI

**Files:**
- Create: `crates/sim-core/src/economia/lavoro.rs`
- Modify: `crates/sim-core/src/economia/mod.rs`
- Modify: `crates/sim-cli/src/main.rs` (stampa `{ "vista": …, "economia": … }`)
- Modify: `crates/sim-cli/tests/cli.rs`
- Test: `crates/sim-core/tests/economia.rs`

**Ricerca:** `docs/ricerca/modelli/economia-italiana-parametri.md:52` (Okun nazionale in gap, −0,254), `:54-69` (coefficienti regionali in differenze, non intercambiabili col nazionale) e `:120` (ricalibrare la media sul nazionale, u* regionale con isteresi); `docs/ricerca/modelli/mercato-del-lavoro-e-disuguaglianza.md:12` e `:23`; spec righe 291 (gap della regione = media dei gap di settore pesata col mix) e 297.

**Dispatch:** `rust-engineer` · dopo Task 5: `economia/mod.rs`, `tests/economia.rs`, `gap_settori` ricalcolato nel Task 5; chiude la PR A, poi il round di review · skill-gate: true.

**Interfaces:**
- Consumes: `gap_settori` (Task 4-5), `Regione::{mix, okun, forze_lavoro}`.
- Produces:
  - `β_r = okun_r · okun_nazionale / (Σ_r fl_r·okun_r / Σ_r fl_r)`: i coefficienti regionali, riscalati perché la loro media pesata sulle forze di lavoro valga `okun_nazionale`.
  - Per ogni regione: `gap = Σ_s mix[s]·gap_settori[s]`; `disoccupazione = disoccupazione_strutturale + β_r·gap`; poi `disoccupazione_strutturale ← disoccupazione_strutturale + isteresi_disoccupazione·(disoccupazione − disoccupazione_strutturale)`. La disoccupazione non scende sotto 0,5% [assunzione del piano 3].
  - `Economia::disoccupazione_nazionale(&self, dati: &DatiGioco) -> f64`: media pesata sulle forze di lavoro.
  - Output del CLI: un oggetto JSON con `vista` (la `Vista` di prima) ed `economia` (lo stato vero). I test esistenti leggono `["vista"]["turno"]` e così via.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn disoccupazione_nazionale_del_turno_zero_e_5_6() {
    let p = Partita::nuova(dati_base(), 1);
    assert!((p.economia().disoccupazione_nazionale(&dati_base()) - 5.6).abs() < 1e-9);
}

#[test]
fn una_recessione_alza_la_disoccupazione_in_ogni_regione() {
    let f = con_ingresso(senza_shock(), "output_gap", -4.0);
    let prima = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let dopo = giocata(&f, 1, 1);
    for (d, p) in dopo.economia().regioni.iter().zip(&prima.economia().regioni) {
        assert!(d.disoccupazione > p.disoccupazione, "{}", d.id);
    }
}

#[test]
fn il_gap_regionale_e_la_media_dei_settori_pesata_col_mix() {
    let d = dati_base();
    let p = giocata(&comune::file_base(), 5, 3);
    let e = p.economia();
    for (r, g) in e.regioni.iter().zip(d.regioni()) {
        let atteso: f64 = d.struttura().settori.iter().zip(&e.gap_settori)
            .map(|(s, gap)| g.mix[&s.id] * gap).sum();
        assert!((r.gap - atteso).abs() < 1e-9, "{}", r.id);
    }
}

#[test]
fn piu_esportazioni_aiutano_di_piu_le_regioni_industriali() {
    let p = giocata(&con_ingresso(senza_shock(), "domanda_estera", 10.0), 1, 1);
    let gap = |id: &str| p.economia().regioni.iter().find(|r| r.id == id).unwrap().gap;
    assert!(gap("veneto") > gap("lazio"), "veneto {} lazio {}", gap("veneto"), gap("lazio"));
}
```

Il quarto test vale se l'export pesa più nell'industria e l'industria pesa più nel mix veneto che in quello laziale: lo dicono la tavola e il mix del Task 2.

Nei test del CLI: `gioca_otto_turni` legge `vista["vista"]["turno"]` ecc. e controlla che `vista["economia"]["regioni"]` abbia 20 elementi.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test --workspace`
Expected: FAIL nei test nuovi.

- [ ] **Step 3: Implementa `lavoro.rs`, il passo e l'output del CLI**

- [ ] **Step 4: Esegui la suite, clippy e fmt**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: tutto pulito.

- [ ] **Step 5: Commit e PR A**

```bash
git add crates
git commit -m "feat(core): regional Okun with hysteresis; CLI prints true state"
```

PR A su `main` dal branch `feat/v1-motore-economico`, dopo il round di review pre-commit.

---

## PR B · Conti pubblici (Task 7-10)

Branch `feat/v1-conti-pubblici` creato dalla punta di `feat/v1-motore-economico`.

### Task 7: Finanza pubblica, interessi e impulso fiscale

**Files:**
- Create: `crates/sim-core/src/economia/finanza.rs`
- Modify: `crates/sim-core/src/economia/mod.rs`, `congiuntura.rs` (impulso nella IS), `leontief.rs` (scostamento reale della spesa dei dicasteri nella domanda)
- Test: `crates/sim-core/tests/conti.rs`

**Ricerca:** `docs/ricerca/modelli/finanza-pubblica.md:12-13` (dinamica del debito); `docs/ricerca/modelli/economia-italiana-parametri.md:104-106` (elasticità per imposta, gettito sopra la base strutturale); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:86-106` (budget da COFOG, funzionamento = salari + acquisti, margine 1,6%) e `:27-32` (disavanzo, interessi, costo medio, vita media); spec righe 289, 299-300 (erosione nominale), 276 (bilancio 2027 ricondotto al tetto UE); ticket 20 (bilancio 2026 indicizzato e ricondotto al tetto del 2027: `.scratch/v1-giocabile/issues/20-partenza-della-partita.md`, sezione «Calendario»).

**Dispatch:** `rust-engineer` · dopo Task 6, sul branch `feat/v1-conti-pubblici` creato dalla punta della PR A: `economia/mod.rs`, `congiuntura.rs`, `leontief.rs` dei Task 4-5; fissa `altre_entrate_pil` in `dati/base/scenario.toml` del Task 3 · skill-gate: true.

**Interfaces:**
- Consumes: `pil_trimestrale`, `output_gap`, `livello_prezzi`, `tasso_bce` (Task 3-5); `Dicastero::{spesa_pil, salari_pil, acquisti_pil}`.
- Produces: campi nuovi di `Economia`, in euro `i64` salvo dove detto:
  - `bilancio: Bilancio { pub anno: u16, pub dicasteri: Vec<BudgetDicastero> }`, `BudgetDicastero { pub id: String, pub spesa: i64, pub salari: i64, pub acquisti: i64, pub margine: i64 }` (annui, nominali); `bilancio_approvato: Option<Bilancio>` per l'anno dopo;
  - `debito: i64`; `tasso_medio_debito: f64`; `pil_potenziale_nominale_iniziale: f64` (riferimento delle entrate);
  - `conti: ContiTrimestre { pub entrate: Entrate, pub spese: Spese }` del turno appena chiuso, con `Entrate { pub irpef, pub ires, pub iva, pub contributi, pub altre, pub fondi_coesione: i64 }` e `Spese { pub dicasteri, pub pensioni, pub sussidi, pub altre_prestazioni, pub interessi: i64 }`; `disavanzo_trimestre: i64` (spese − entrate);
  - `storico: Vec<ConsuntivoTrimestre>` degli ultimi 8 trimestri `{ pub data: Trimestre, pub pil: i64, pub disavanzo: i64, pub spesa_primaria: i64 }`, il più recente in coda.
- Regole (`passo(data)` simula il trimestre `data`, cioè il turno che si chiude: la prima chiusura simula ottobre-dicembre 2026, la seconda gennaio-marzo 2027):
  - `pub(crate) fn conti_del_trimestre(e: &Economia, dati: &DatiGioco) -> ContiTrimestre` è l'unica formula di entrate e spese. `Economia::iniziale` la chiama una volta per derivare i conti e il disavanzo del turno zero senza toccare il debito (spec riga 276: il disavanzo si ricalcola al caricamento), e `storico` parte con quel trimestre ripetuto 4 volte; `passo` la chiama a ogni turno.
  - Bilancio 2026 al turno zero: per ogni dicastero `spesa = spesa_pil/100 · 4·pil_trimestrale`, `salari`, `acquisti` uguali, `margine = quota_margine/100 · spesa`. Bilancio 2027 approvato dal governo uscente, calcolato in `iniziale`: ogni voce × `(1 + min(inflazione, tetto_2027)/100)`. Al primo `passo` con `data.trimestre == 1` e un `bilancio_approvato` di `data.anno`, il bilancio diventa quello approvato e `bilancio_approvato` torna `None`. Senza bilancio approvato le cifre restano nominali (esercizio provvisorio; la legge di bilancio arriva col piano 5).
  - Spesa del trimestre: ogni voce annua / 4. Pensioni e sussidi qui sono le cifre annue dello scenario / 4; il Task 8 le sostituisce coi suoi blocchi. `altre_prestazioni` / 4, ferma in nominale.
  - Entrate: `R_k = quota_k/100 · PIL*_nom/4 · (1 + y_{t−1}/100)^elasticita_k` per IRPEF, IRES, IVA, contributi, con `PIL*_nom = pil_potenziale_reale · livello_prezzi · 4` e il gap del turno prima (la finanza pubblica viene prima della IS nell'ordine); `altre` con elasticità 1; `fondi_coesione = fondi_coesione_pil/100 · PIL*_nom/4` (il Task 10 li sospende).
  - Interessi: `interessi = tasso_medio_debito/100 · debito / 4`; poi `tasso_medio_debito ← (1 − q)·tasso_medio_debito + q·(tasso_bce + premio_termine_bund + spread/100)` con `q = 1/(4·vita_media_debito)` e lo spread del turno prima (fino al Task 10 lo spread è la costante `SPREAD_PROVVISORIO_PB: f64 = 84.0` in `finanza.rs`, che il Task 10 toglie).
  - `debito ← debito + disavanzo_trimestre`. Debito iniziale `debito_pil/100 · 4·pil_trimestrale`.
  - Impulso fiscale (nella IS, Task 5): `100·[Gp_t − Gp_{t−1}·(1 + π_{t−1}/400)] / PIL*_nom_trimestrale`, con `Gp` = spesa dei dicasteri + altre prestazioni del trimestre. Il bilancio fermo in nominale dà così un impulso negativo pari all'erosione reale (spec, «Erosione nominale»).
  - Domanda finale (Task 4): a `d` si aggiunge `Σ_d vettore_d · (spesa_d,t / livello_prezzi − spesa_d,0) / 4 / 1e9`, cioè lo scostamento reale della spesa trimestrale del dicastero dal turno zero, in miliardi.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/conti.rs
mod comune;
use comune::{con_ingresso, dati_base, file_base, senza_shock};
use sim_core::scenario::StatoPde;
use sim_core::{DatiGioco, Partita};

fn giocata(dati: DatiGioco, turni: u32) -> Partita {
    let mut p = Partita::nuova(dati, 1);
    for _ in 0..turni { p.chiudi_turno(); }
    p
}
fn base_senza_shock() -> DatiGioco { DatiGioco::da_testi(&senza_shock()).unwrap() }

#[test]
fn disavanzo_del_turno_zero_e_intorno_al_2_9_per_cento() {
    let p = giocata(base_senza_shock(), 1);
    let e = p.economia();
    let quota = e.disavanzo_trimestre as f64 / (e.pil_trimestrale * 1e9) * 100.0;
    assert!((2.5..=3.3).contains(&quota), "{quota}");
}

#[test]
fn il_debito_cresce_esattamente_del_disavanzo() {
    let mut p = Partita::nuova(base_senza_shock(), 1);
    for _ in 0..8 {
        let prima = p.economia().debito;
        p.chiudi_turno();
        assert_eq!(p.economia().debito, prima + p.economia().disavanzo_trimestre);
    }
}

#[test]
fn bilancio_2027_applicato_una_volta() {
    let sanita = |p: &Partita| p.economia().bilancio.dicasteri.iter().find(|d| d.id == "sanita").unwrap().spesa;
    let s26 = sanita(&Partita::nuova(base_senza_shock(), 1));
    let dopo_ottobre = giocata(base_senza_shock(), 1);
    assert_eq!(dopo_ottobre.economia().bilancio.anno, 2026);
    let dopo_gennaio = giocata(base_senza_shock(), 2);
    assert_eq!(dopo_gennaio.economia().bilancio.anno, 2027);
    assert!((sanita(&dopo_gennaio) - (s26 as f64 * 1.019).round() as i64).abs() <= 1);
    let gennaio_2028 = giocata(base_senza_shock(), 6);
    assert_eq!(gennaio_2028.economia().bilancio.anno, 2027, "senza legge di bilancio resta il 2027");
    assert_eq!(sanita(&gennaio_2028), sanita(&dopo_gennaio));
}

#[test]
fn il_disavanzo_si_deriva_al_caricamento() {
    let e = Partita::nuova(base_senza_shock(), 1);
    assert!(e.economia().disavanzo_trimestre > 0);
}

#[test]
fn margine_al_1_6_per_cento() {
    let p = Partita::nuova(dati_base(), 1);
    let b = &p.economia().bilancio;
    let (m, s): (i64, i64) = b.dicasteri.iter().fold((0, 0), |(m, s), d| (m + d.margine, s + d.spesa));
    assert!(((m as f64 / s as f64) * 100.0 - 1.6).abs() < 0.01);
}

#[test]
fn bilancio_fermo_in_nominale_e_un_impulso_negativo() {
    let p = giocata(base_senza_shock(), 6);
    assert!(p.economia().impulso_fiscale < 0.0);
}

#[test]
fn interessi_del_turno_zero_al_costo_medio() {
    let p = giocata(base_senza_shock(), 1);
    let e = p.economia();
    let annui = e.conti.spese.interessi as f64 * 4.0 / 1e9;
    assert!((annui - 0.0281 * 1.367 * 4.0 * 576.9).abs() < 1.0, "{annui}");
}
```

Il fattore 1,019 del test del bilancio è `1 + min(4,2; 1,9)/100`. `altre_entrate_pil` dello scenario si fissa in questo step perché `disavanzo_del_turno_zero_e_intorno_al_2_9_per_cento` passi al centro (2,9), e il numero scritto nel file porta il commento che lo dice.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test conti`
Expected: errore di compilazione.

- [ ] **Step 3: Implementa `finanza.rs`, l'impulso nella IS e la spesa nella domanda finale**

- [ ] **Step 4: Esegui tutti i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati
git commit -m "feat(core): public finance block, debt dynamics and fiscal impulse"
```

---

### Task 8: Spesa obbligatoria: pensioni per annate e sussidi

**Files:**
- Create: `crates/sim-core/src/economia/previdenza.rs`
- Modify: `crates/sim-core/src/economia/mod.rs`, `finanza.rs` (pensioni e sussidi dai blocchi nuovi)
- Modify: `dati/base/manopole.toml` (`copertura_pensionamento` calcolata)
- Test: `crates/sim-core/tests/conti.rs`

**Ricerca:** `docs/ricerca/modelli/finanza-pubblica.md:34` (indicizzazione delle pensioni) e `:44` (la formula di calcolo cambia la pensione media solo per le coorti future); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:116-130` (spesa, pensionati, nuove pensioni, NASpI, piramide); spec righe 301 (regimi, coorti future pro rata, indicizzazione unica leva retroattiva) e 300 (pensione minima e soglie ferme in nominale); `CONTEXT.md:28-34` (spesa obbligatoria, regime).

**Dispatch:** `rust-engineer` · dopo Task 7: `finanza.rs`, `tests/conti.rs`, `manopole.toml` (`copertura_pensionamento`) del Task 3 · skill-gate: true.

**Interfaces:**
- Consumes: `Scenario::{pensionati, spesa_pensioni, eta_pensione, indicizzazione_pensioni, pensione_minima, regime_pensioni, spesa_sussidi, regime_sussidi}`; `Struttura::piramide`; `pil_trimestrale`, `inflazione`, disoccupazione nazionale e forze di lavoro (Task 6).
- Produces: campi nuovi di `Economia`: `pensioni: Vec<Annata>` con `Annata { pub anno: u16, pub regime: RegimePensioni, pub pensionati: i64, pub importo: i64 }` (importo annuo medio in euro); `sussidio_per_disoccupato: i64` (euro annui, derivato al turno zero = `spesa_sussidi / disoccupati`). `RegimePensioni` e `RegimeSussidi` stanno in `scenario.rs` (Task 3). Il regime in vigore resta quello dello scenario finché il piano 6 non porta le leggi.
- Regole:
  - Turno zero: una sola annata `{ anno: 2026, regime: regime dello scenario, pensionati, importo: spesa_pensioni / pensionati }`.
  - A ogni turno con `trimestre == 1`, anno `A`, in quest'ordine: (1) ogni annata esistente perde `uscite_pensionati` dei pensionati e indicizza l'importo per `1 + indicizzazione_pensioni·π̄/100`, con `π̄` = media delle ultime 4 `inflazione`; (2) entra l'annata `A`: `pensionati = copertura_pensionamento · piramide[eta_pensione − (A − 2026)]` (indice fermato a 0; nessuna mortalità prima della pensione e nessuna nascita [assunzione del piano 3, il limite pesa dopo una ventina d'anni]); `importo` secondo il regime in vigore, con `salario = quota_redditi_lavoro · 4·pil_trimestrale·1e9 / occupati` e `occupati = forze di lavoro · (1 − u/100)`:
    - `Retributivo`: `max(pensione_minima, sostituzione_retributivo · salario)`;
    - `Contributivo`: `max(pensione_minima, sostituzione_contributivo · salario)`;
    - `QuotaBaseUniversale`: `quota_base_universale + sostituzione_contributivo · salario`.
  - Spesa del trimestre: `Σ pensionati·importo / 4`.
  - Sussidi: `disoccupati · sussidio_per_disoccupato / 4`, per `sussidio_universale` se il regime è `Universale`. L'importo resta fermo in nominale.
  - `copertura_pensionamento` = il valore per cui gli ingressi del 2027 uguagliano le uscite del 2027 dello stock iniziale, calcolato una volta dall'implementatore con la piramide del Task 2 e scritto nel file col commento che lo spiega.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
fn con_regime_pensioni(regime: &str) -> DatiGioco {
    let mut f = senza_shock();
    f.scenario = f.scenario.replacen("regime_pensioni = \"contributivo\"", &format!("regime_pensioni = \"{regime}\""), 1);
    DatiGioco::da_testi(&f).unwrap()
}

#[test]
fn spesa_pensioni_del_turno_zero_e_quella_dello_scenario() {
    let p = giocata(base_senza_shock(), 1);
    let annua = p.economia().conti.spese.pensioni as f64 * 4.0 / 1e9;
    assert!((annua - 351.8).abs() < 0.5, "{annua}");
}

#[test]
fn il_regime_tocca_solo_le_annate_nuove() {
    let (r, c) = (con_regime_pensioni("retributivo"), con_regime_pensioni("contributivo"));
    assert_eq!(giocata(r.clone(), 1).economia().conti.spese.pensioni,
               giocata(c.clone(), 1).economia().conti.spese.pensioni);
    let (pr, pc) = (giocata(r, 9), giocata(c, 9));
    assert!(pr.economia().conti.spese.pensioni > pc.economia().conti.spese.pensioni);
    assert_eq!(pr.economia().pensioni[0].importo, pc.economia().pensioni[0].importo,
               "l'annata del 2026 non cambia col regime");
}

#[test]
fn le_pensioni_in_essere_seguono_l_inflazione() {
    let p = giocata(base_senza_shock(), 2);
    let a = &p.economia().pensioni[0];
    let iniziale = (351.8e9 / 16_388_149.0).round() as i64;
    assert!(a.importo > iniziale);
}

#[test]
fn i_sussidi_salgono_in_recessione() {
    let crisi = giocata(DatiGioco::da_testi(&con_ingresso(senza_shock(), "output_gap", -4.0)).unwrap(), 2);
    let normale = giocata(base_senza_shock(), 2);
    assert!(crisi.economia().conti.spese.sussidi > normale.economia().conti.spese.sussidi);
}

#[test]
fn sussidi_del_turno_zero_alla_naspi() {
    let p = giocata(base_senza_shock(), 1);
    let annui = p.economia().conti.spese.sussidi as f64 * 4.0 / 1e9;
    assert!((annui - 17.93).abs() < 0.5, "{annui}");
}
```

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test conti`
Expected: FAIL o errore di compilazione sui campi nuovi.

- [ ] **Step 3: Implementa `previdenza.rs` e calcola `copertura_pensionamento`**

- [ ] **Step 4: Esegui tutti i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati
git commit -m "feat(core): pensions by vintage with regimes, unemployment benefits"
```

---

### Task 9: SFC a quattro settori con il controllo di chiusura

**Files:**
- Create: `crates/sim-core/src/economia/sfc.rs`
- Modify: `crates/sim-core/src/economia/mod.rs`
- Test: `crates/sim-core/tests/conti.rs`

**Ricerca:** `docs/ricerca/modelli/macro-stock-flow-consistent.md:35` (la somma dei saldi fa zero, controllo gratuito) e `:52` (ogni azione passa da una riga della matrice); `docs/ricerca/sintesi/motore-economico-raccomandato.md:35` (4 settori, senza banche); spec riga 294 (estero come domanda esogena con elasticità ai prezzi relativi; fondi di coesione come entrata esogena).

**Dispatch:** `rust-engineer` · dopo Task 8: `economia/mod.rs`, `tests/conti.rs`, conti dei Task 7-8 · skill-gate: true.

**Interfaces:**
- Consumes: `conti`, `pil_trimestrale`, `domanda_estera`, `livello_prezzi`, `area_euro.inflazione_resto` (Task 5-8).
- Produces: campi nuovi di `Economia`: `sfc: Sfc { pub saldi: Settori, pub stock: Settori, pub livello_prezzi_area: f64 }`, `Settori { pub famiglie: i64, pub imprese: i64, pub stato: i64, pub estero: i64 }`; `Economia::residuo_sfc(&self) -> i64` = somma dei quattro saldi del turno; `Economia::residuo_stock(&self) -> i64` = somma dei quattro stock.
- Flussi del trimestre, in euro, arrotondati una volta ciascuno (`Y` = `pil_trimestrale`):
  - `X = esportazioni_pil/100 · Y · (1 + domanda_estera/100) · (livello_prezzi_area / livello_prezzi)^elasticita_export_prezzi`; `M = importazioni_pil/100 · Y`; `livello_prezzi_area ← livello_prezzi_area·(1 + inflazione_area/400)`.
  - `G` = salari + acquisti + margine dei dicasteri del trimestre; `contributi_imprese` = resto dei dicasteri (`spesa − salari − acquisti − margine`) [assunzione del piano 3: la parte non di funzionamento va alle imprese come contributi e investimenti].
  - Domanda privata `C + I = Y − G − X + M`, divisa fra `C` e `I` con le quote dello scenario.
  - `W = quota_redditi_lavoro · Y`; imposte indirette `T_ind = IVA`; profitti `Π = Y − W − T_ind` (residuo, come l'eccedenza operativa dei conti nazionali); distribuiti `quota_profitti_distribuiti · Π`.
  - Saldi: famiglie `W + Π_dist + pensioni + sussidi + altre_prestazioni + interessi − IRPEF − contributi − altre_entrate − C`; imprese `Π − Π_dist + contributi_imprese − IRES − I`; Stato `entrate − spese` (= `−disavanzo_trimestre`); estero `M − X − fondi_coesione + ammende` (le ammende arrivano col Task 10, qui 0). Gli interessi vanno tutti alle famiglie [assunzione del piano 3: quota estera del debito non nella ricerca].
  - Stock: `stock ← stock + saldo`. Iniziali: Stato `−debito`, famiglie `+debito`, imprese e estero 0 [assunzione del piano 3: posizione netta sull'estero non nella ricerca]. Stato e `debito` restano legati: `stock.stato == −debito` a ogni turno.
- La somma fa zero per costruzione contabile solo se ogni flusso compare con segni opposti in due settori: `residuo_sfc` è il controllo dei bug di contabilità, e `passo` ha un `debug_assert_eq!(self.residuo_sfc(), 0)`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn la_somma_dei_saldi_fa_zero_a_ogni_turno() {
    for seme in 0..20 {
        let mut p = Partita::nuova(dati_base(), seme);
        for _ in 0..40 {
            p.chiudi_turno();
            assert_eq!(p.economia().residuo_sfc(), 0, "seme {seme}");
            assert_eq!(p.economia().residuo_stock(), 0, "seme {seme}");
        }
    }
}

#[test]
fn lo_stock_dello_stato_e_il_debito() {
    let p = giocata(dati_base(), 12);
    assert_eq!(p.economia().sfc.stock.stato, -p.economia().debito);
}

#[test]
fn il_saldo_dello_stato_e_meno_il_disavanzo() {
    let p = giocata(dati_base(), 3);
    assert_eq!(p.economia().sfc.saldi.stato, -p.economia().disavanzo_trimestre);
}
```

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test conti`
Expected: errore di compilazione.

- [ ] **Step 3: Implementa `sfc.rs`**

- [ ] **Step 4: Esegui tutti i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): four-sector SFC accounts with closing check"
```

---

### Task 10: Spread, TPI, regole UE e spread nella vista

**Files:**
- Create: `crates/sim-core/src/economia/spread.rs`
- Create: `crates/sim-core/src/economia/regole_ue.rs`
- Modify: `crates/sim-core/src/economia/mod.rs`, `finanza.rs` (spread vero nel tasso di mercato, fondi di coesione sospesi, ammende), `sfc.rs` (ammende all'estero)
- Modify: `crates/sim-core/src/vista.rs` (`spread_pb: i32`), `crates/sim-core/src/partita.rs` (`vista` lo riempie)
- Modify: `app/src/lib/tipi/Vista.ts` (rigenerato), `app/src/routes/+page.svelte` (una riga `Spread {spread_pb} pb`)
- Test: `crates/sim-core/tests/conti.rs`, `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/modelli/economia-italiana-parametri.md:12-17` (quadratica di De Grauwe-Ji, coefficienti), `:21` (2,4 pb per punto di disavanzo), `:37-44` (criteri del TPI, il primo alternativo: PDE con azione efficace li rispetta) e `:117-118` (forma per il gioco e tetto del TPI sul solo premio di sfiducia); `docs/ricerca/modelli/regole-fiscali-ue.md:14` (spesa netta), `:20` (conto di controllo e soglie), `:33-38` (PDE passo per passo), `:63-78` (forma minima per il gioco); spec righe 295-296; `CONTEXT.md:338` (lo spread è l'indicatore sintetico dei conti, sempre a schermo); `docs/ricerca/sintesi/decisioni.md:267` (spread esatto nella vista).

**Dispatch:** `rust-engineer`, intero, non spezzato · dopo Task 9: `finanza.rs`, `sfc.rs`, `partita.rs`, `tests/conti.rs` · skill-gate: true. Il lato Svelte è una riga `<p>` provvisoria fino al piano 11 più i tipi rigenerati da `cargo test`: stessa scelta del Task 5 del piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:560`). Lo Step 5 lo guarda Mario: finestra nativa, nessun agente la vede.

**Interfaces:**
- Consumes: `debito`, `storico`, `conti` (Task 7-9); `Scenario::{pde, conto_controllo, tetti_spesa_netta, clausola_nazionale_margine, clausola_nazionale_fino_a, premio_sfiducia}`.
- Produces: campi nuovi di `Economia`: `spread: f64` (pb), `premio_sfiducia: f64` (pb), `regole_ue: RegoleUe { pub pde: StatoPde, pub conto_controllo: f64, pub ultima_deviazione: f64, pub turni_in_stato: u32, pub spesa_netta: BTreeMap<u16, i64>, pub pil_annuo: BTreeMap<u16, i64>, pub disavanzo_annuo: BTreeMap<u16, i64> }` (accumulatori per anno solare, solo `BTreeMap`); `Vista::spread_pb: i32` (spread arrotondato).
- Spread, a fine turno: con `b` = debito / PIL degli ultimi 4 trimestri × 100 e `d` = disavanzo degli ultimi 4 trimestri / stesso PIL × 100,
  `spread = max(spread_minimo, spread_s0 + 100·(spread_beta_1·b + spread_beta_2·b²)) + spread_disavanzo·max(0, d − 3) + premio_sfiducia`.
  Al turno zero, prima di ogni passo, `spread` = la stessa formula sullo stato iniziale (il riepilogo dello scenario lo deriva così). Per gli ultimi 4 trimestri al turno zero vale il trimestre del turno zero ripetuto.
- Premio di sfiducia: se la PDE è `Nessuna`, `Aperta` o `Sospesa` il TPI vale e il premio va a 0 nel turno; altrimenti decade con `premio ← premio · 0,5^(1/emivita_premio_sfiducia)`. Solo il criterio 1 del TPI è modellato [assunzione del piano 3: squilibri, sostenibilità e PNRR non hanno grandezze nella v1].
- Regole UE:
  - Accumulatori: a ogni turno si sommano all'anno solare della `data` la spesa netta del trimestre (spesa primaria − fondi di coesione − componente ciclica dei sussidi, cioè la parte sopra la spesa a disoccupazione strutturale), il PIL e il disavanzo. Il 2026 si completa al turno zero moltiplicando per 4 il trimestre simulato; il 2025 si ricava dal 2026 come `spesa_netta_2026 / (1 + tetto_2026/100)` [assunzione del piano 3: il 2026 esattamente sul tetto, coerente con la PDE sospesa].
  - Controllo annuale nel turno con `trimestre == 2` dell'anno `A`, sull'anno `A − 1`: `deviazione = (SN[A−1] − SN[A−2]·(1 + tetto[A−1]/100)) / PIL[A−1] · 100 − clausola` (clausola = `clausola_nazionale_margine` fino a `clausola_nazionale_fino_a`, poi 0), mai sotto zero nel conto; `conto_controllo += deviazione`. Rilevante se `deviazione > deviazione_annua_rilevante` o `conto_controllo > deviazione_cumulata_rilevante`.
  - Macchina della PDE nello stesso turno: `Nessuna` → `Aperta` se il disavanzo di `A − 1` > 3% o (debito > 60%, disavanzo > 0,5% e deviazione rilevante). `Aperta` o `Sospesa` → `Inadempiente` se rilevante, altrimenti `Sospesa`; `Sospesa` → `Nessuna` se il disavanzo di `A − 1` e quello degli ultimi 4 trimestri sono sotto il 3%. `Inadempiente` comporta l'ammenda `ammenda_pde` % del PIL annuo (spesa dello Stato verso l'estero, a debito) e la sospensione dei fondi di coesione fino a `sospensione_coesione` % del PIL; al primo `passo` con `data.trimestre == 4` dopo almeno un turno in `Inadempiente` diventa `Intimata` (126.9); dopo 2 turni in `Intimata` diventa `Sanzionata`, e da lì ogni 2 turni paga `ammenda_semestrale` % del PIL annuo. `turni_in_stato` conta i turni passati nello stato corrente e torna a 0 a ogni cambio. Da `Inadempiente`, `Intimata` o `Sanzionata` torna `Sospesa` al controllo annuale senza deviazione rilevante.
  - Fuori dal turno aprile-giugno nulla cambia nel conto né nella PDE.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
fn dati(f: sim_core::FileDati) -> DatiGioco { DatiGioco::da_testi(&f).unwrap() }
fn inadempiente(f: sim_core::FileDati) -> sim_core::FileDati {
    comune::sostituisci(f, "scenario", "pde = \"sospesa\"", "pde = \"inadempiente\"")
}

#[test]
fn spread_del_turno_zero_fra_79_e_89() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert!((79..=89).contains(&v.spread_pb), "{}", v.spread_pb);
}

#[test]
fn spread_piatto_sotto_la_soglia() {
    let p = Partita::nuova(dati(con_ingresso(senza_shock(), "debito_pil", 110.0)), 1);
    assert_eq!(p.vista().spread_pb, 60);
}

#[test]
fn spread_convesso_sopra_la_soglia() {
    let s = |b: f64| Partita::nuova(dati(con_ingresso(senza_shock(), "debito_pil", b)), 1).economia().spread;
    let (a, b, c) = (s(140.0), s(150.0), s(160.0));
    assert!(c - b > b - a && b > a, "{a} {b} {c}");
}

#[test]
fn il_tpi_azzera_il_premio_con_la_pde_sospesa() {
    let p = giocata(dati(con_ingresso(senza_shock(), "premio_sfiducia", 200.0)), 1);
    assert_eq!(p.economia().premio_sfiducia, 0.0);
}

#[test]
fn senza_tpi_il_premio_decade_con_la_sua_emivita() {
    let f = inadempiente(con_ingresso(senza_shock(), "premio_sfiducia", 200.0));
    let premio = giocata(dati(f), 2).economia().premio_sfiducia;
    assert!((premio - 200.0 * 0.5_f64.powf(2.0 / 4.0)).abs() < 1e-6, "{premio}");
}

#[test]
fn conto_di_controllo_si_muove_solo_ad_aprile_giugno() {
    let mut p = Partita::nuova(dati_base(), 3);
    let mut prima = p.economia().regole_ue.conto_controllo;
    for _ in 0..12 {
        p.chiudi_turno();
        let ora = p.economia().regole_ue.conto_controllo;
        if ora != prima { assert_eq!(p.vista().data.trimestre, 3, "il controllo gira nel turno aprile-giugno appena chiuso"); }
        prima = ora;
    }
}

#[test]
fn nessuna_inadempienza_spuria_al_primo_controllo() {
    let p = giocata(base_senza_shock(), 3);
    assert!(!matches!(p.economia().regole_ue.pde, StatoPde::Inadempiente | StatoPde::Intimata | StatoPde::Sanzionata));
}

#[test]
fn salva_carica_con_pde_inadempiente() {
    let mut diretta = giocata(dati(inadempiente(file_base())), 5);
    let mut ricaricata = Partita::carica(&diretta.salva().unwrap()).unwrap();
    for _ in 0..6 { diretta.chiudi_turno(); ricaricata.chiudi_turno(); }
    assert_eq!(ricaricata.salva().unwrap(), diretta.salva().unwrap());
}

#[test]
fn scenario_estremo_resta_finito() {
    let f = con_ingresso(con_ingresso(con_ingresso(file_base(), "debito_pil", 200.0), "inflazione", 12.0), "disoccupazione", 15.0);
    let mut p = Partita::nuova(dati(f), 9);
    for _ in 0..200 {
        p.chiudi_turno();
        assert_eq!(p.economia().residuo_sfc(), 0);
    }
    assert!(p.salva().is_ok());
}
```

Il turno aprile-giugno è il trimestre 2: dopo averlo chiuso la data della vista è il trimestre 3, da cui l'asserzione. Nel test dell'emivita i due turni (ottobre-dicembre 2026, gennaio-marzo 2027) non contengono un controllo annuale, quindi la PDE resta fuori dal TPI per tutti e due.
Aggiorna `turno_zero_e_ottobre_dicembre_2026` in `partita.rs` controllando anche `v.spread_pb` fra 79 e 89, e i test del CLI controllando che `vista.spread_pb` esista.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core`
Expected: errore di compilazione.

- [ ] **Step 3: Implementa `spread.rs`, `regole_ue.rs`, i collegamenti e la vista**

Rigenera i tipi TypeScript con `cargo test -p sim-core` e aggiungi nella schermata, sotto l'intestazione, `<p>Spread {vista.spread_pb} pb</p>` (testo minimo 12 px, nessun calcolo nel frontend).

- [ ] **Step 4: Verifica tutto**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check && npm --prefix app run build`
Expected: tutto pulito.

- [ ] **Step 5: Verifica l'app a mano (Mario)**

Run: `npm --prefix app run tauri dev`
Expected: «Turno 0 · ottobre-dicembre 2026», «Spread N pb» con N fra 79 e 89; dopo «Chiudi turno» la finestra resta reattiva, il pulsante è disabilitato durante la chiamata, «Turno 1 · gennaio-marzo 2027» e uno spread nuovo.

- [ ] **Step 6: Commit e PR B**

```bash
git add crates app
git commit -m "feat(core): spread with TPI, EU fiscal rules; spread in the view"
```

PR B sul branch `feat/v1-motore-economico` (o su `main` se A è già unita), dopo il round di review pre-commit.
