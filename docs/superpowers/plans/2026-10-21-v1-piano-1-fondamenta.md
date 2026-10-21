# V1 · Piano 1: fondamenta — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Workspace Rust + Tauri con il crate `sim-core` puro (caso a seme fisso, dati caricati e validati, partita che avanza a trimestri e si salva e ricarica identica), il binario `sim-cli` e un guscio Tauri/Svelte che mostra la vista: la seam principale della spec, pronta per i piani che ci mettono dentro le regole.

**Architecture:** `sim-core` è una libreria senza I/O né thread. Riceve i file di dati come testo, tiene lo stato vero e consegna una `Vista` serializzabile. `sim-cli` e il guscio Tauri leggono i file e chiamano la stessa API. I tipi TypeScript della UI sono generati dai tipi Rust con `ts-rs`.

**Tech Stack:** Rust 1.98 (workspace Cargo), `serde`, `serde_json`, `toml`, `rand_core` + `rand_chacha` (ChaCha8, versione esatta), `ts-rs`; Tauri 2.12; Svelte 5 + TypeScript + Vite (template `svelte-ts` di create-tauri-app), npm.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md), sezioni «Architettura», «Le quattro cuciture», «Modulo dati», «Turno e partita», «Testing Decisions».

## Global Constraints

- Identificatori, messaggi d'errore e testi in italiano, coi termini di `CONTEXT.md` (dicastero, mai «settore» da solo; turno, mai tick).
- `sim-core` non fa I/O e non usa thread: niente `std::fs`, `std::time`, `std::thread`, `std::net`, `std::env`. Resta compilabile in WASM.
- PRNG con algoritmo nominato, ChaCha8 di `rand_chacha`, versione esatta con `=` in `Cargo.toml`; mai `StdRng` né `SmallRng`; mai campionare `usize` (`docs/ricerca/tecnica/architettura-simulazione.md:19-21`).
- Un flusso casuale per sottosistema, così una modifica in uno non sposta gli altri (`docs/ricerca/tecnica/architettura-simulazione.md:22`).
- Nello stato solo `Vec` e `BTreeMap`, mai `HashMap`: ordine d'iterazione stabile (`docs/ricerca/tecnica/architettura-simulazione.md:28`).
- Interi per euro, seggi, voti ed elettori; `f64` solo per Leontief, IS-Phillips, logit e manopole.
- Stesso seme + stessi comandi = stessa partita sullo stesso binario.
- Dati di gioco in TOML (`docs/ricerca/tecnica/modding-e-dati-esterni.md:14`); errore di validazione leggibile, col nome dell'elemento colpevole (`:15`).
- Salvataggio = snapshot JSON con versione dello schema, seme, stato del generatore e copia dei dati (`docs/ricerca/tecnica/architettura-simulazione.md:32-34`); schema diverso rifiutato, nessuna migrazione.
- Solo macOS; Tauri 2.12; versioni bloccate da `Cargo.lock` e `app/package-lock.json`, entrambi committati.
- Testo minimo 12 px a scala 1×.
- Testi del gioco esternalizzati (storia 166): arrivano col piano 11. Qui i messaggi di validazione di `Errore` restano nel codice come diagnostica per l'autore dei dati, e le etichette della schermata provvisoria del Task 5 si buttano col piano 11.

## Review Focus

1. **Salvataggio troncato o corrotto** (disco pieno, file copiato a metà): `Partita::carica` restituisce `Errore::SalvataggioIllegibile`, mai un panic. Test `salvataggio_troncato_non_va_in_panico` nel Task 3.
2. **Nome di ruolo sbagliato nel file dei dicasteri** (refuso del modder): rifiuto al caricamento con il nome sbagliato nel messaggio. Test `ruolo_sconosciuto_rifiutato_col_nome` nel Task 2.
3. **Manopola con intervallo rovesciato o valore fuori intervallo**: rifiuto con il nome della manopola. Test `manopola_fuori_intervallo_rifiutata_col_nome` e `manopola_con_intervallo_rovesciato_rifiutata` nel Task 2.
4. **Aggiornamento di `rand_chacha` che cambia i numeri**: la riproducibilità si rompe in silenzio. Test `valore_fissato_dell_algoritmo` nel Task 1.
5. **Cartella dati mancante passata al CLI**: messaggio con il percorso e codice d'uscita 1, mai un panic. Test `cartella_dati_mancante_esce_con_errore` nel Task 4.

## Sequenza dei piani della v1

Ogni piano produce software che gira e si testa da solo. Si scrive appena chiuso il precedente, perché le sue interfacce dipendono da quelle che il precedente ha consegnato. Tutte le regole passano dalla seam 1 (API di `sim-core`); il riparto ha la sua seam pura; il CLI sta sopra la seam 1.

| # | Piano | Contenuto (sezioni della spec) | Dipende da |
|---|---|---|---|
| 1 | Fondamenta (questo) | workspace, caso, dati e ruoli, partita, snapshot, CLI, guscio Tauri | — |
| 2 | Riparto e legge elettorale | seam 2: riparto puro della 180/2026 contro la simulazione del dossier; menu a gradini; seggi per regione dalla popolazione censita (cucitura 3) | 1 |
| 3 | Motore economico | blocchi nell'ordine della spec (cucitura 1), SFC con controllo di chiusura, spread, regole UE, spesa obbligatoria; scenario di base con i valori di partenza | 1 |
| 4 | Elettorato e sondaggi | 360 gruppi, logit, interessi, giudizio, aspettative, Friedkin-Johnsen; sondaggio pubblico e prime stime; pipeline di taratura offline | 2, 3 |
| 5 | Governo | ministri, agenda, proposte, soddisfazione, legge di bilancio, fondo di riserva, atti e cantieri, regione bersaglio | 3, 4 |
| 6 | Parlamento e misure | disposizioni, leggi di riforma, decreti, linee, franchi tiratori, calendario, fiducia, norme costituzionali, referendum | 2, 5 |
| 7 | Coalizione, elezioni, opposizione | turno zero e fondazione, coalizione pre-elettorale, spartizione, intesa, crisi, sfiducia, ribaltone, opposizione, eredità | 6 |
| 8 | Partiti IA e campagna | governo avversario, opposizione IA, vita dei partiti, campagna, promesse, cassa del partito | 7 |
| 9 | Eventi e informazione | situazioni, eventi puntuali, pericolosità naturale, acquisti d'informazione, centro studi | 5, 8 |
| 10 | Scenario e salvataggi | editor coi cursori e il riepilogo dal vivo, riavvolgimento, bozza, più partite, albo | 3, 7 |
| 11 | UI «Il Settimanale» | tutte le schermate della spec sopra la vista; mappa SVG; testi dell'interfaccia in file esterni; scala della UI regolabile e interruttore del movimento | 1-10, a fette man mano che la vista cresce |
| 12 | Bilanciamento | tre strategie nel CLI, bersagli del costo del governare, taratura delle manopole | 1-10 |

Il piano 11 può partire a fette dopo il 5, quando la vista ha qualcosa da mostrare; il 12 gira dopo ogni piano dal 7 in poi come controllo.

## Dispatch

Tutto sequenziale, 1 → 2 → 3 → 4 → 5: ogni task tocca `Cargo.toml`/`Cargo.lock` (radice o `sim-core`) e consuma l'API del precedente. Ogni riga **Dispatch** sotto dà subagente, dipendenza e skill-gate; la riga **Ricerca** è il riferimento da copiare nel brief.

---

### Task 1: Workspace e caso a seme fisso

**Files:**
- Create: `Cargo.toml` (workspace, `members = ["crates/sim-core"]`, `resolver = "3"`)
- Create: `crates/sim-core/Cargo.toml`
- Create: `crates/sim-core/src/lib.rs`
- Create: `crates/sim-core/src/caso.rs`
- Test: `crates/sim-core/tests/caso.rs`
- Modify: `.gitignore` (aggiungere `target/`, `node_modules/`, `app/dist/`)

**Ricerca:** `docs/ricerca/tecnica/architettura-simulazione.md:19-22` (PRNG nominato e versionato, niente `usize`, un flusso per sottosistema); `docs/ricerca/tecnica/core-rust-e-ui-separata.md:14` (crate senza dipendenze dal motore, test senza finestra).

**Dispatch:** `rust-engineer` · primo, nessuna dipendenza · skill-gate: true.

**Interfaces:**
- Consumes: niente.
- Produces:
  - `sim_core::caso::Flusso` — `enum Flusso { Economia = 0, Elettorato = 1, Parlamento = 2, Eventi = 3, Sondaggi = 4, PartitiIa = 5 }`, `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize`. I discriminanti sono gli id dei flussi ChaCha: mai riordinarli, solo aggiungere in coda.
  - `sim_core::caso::Caso` — `Clone, Debug, PartialEq, Serialize, Deserialize`; `Caso::nuovo(seme: u64) -> Caso`; `Caso::prossimo(&mut self, flusso: Flusso) -> u64`.

- [ ] **Step 1: Crea il workspace e il crate**

`crates/sim-core/Cargo.toml`: `name = "sim-core"`, `edition = "2024"`; dipendenze `serde` (feature `derive`), `rand_core`, `rand_chacha` con versione esatta `=X.Y.Z` (l'ultima stabile al momento, con la feature che abilita serde: `serde` dalla 0.9, `serde1` nelle precedenti). `lib.rs` contiene solo `pub mod caso;`.

- [ ] **Step 2: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/caso.rs
use sim_core::caso::{Caso, Flusso};

#[test]
fn stesso_seme_stessa_sequenza() {
    let (mut a, mut b) = (Caso::nuovo(42), Caso::nuovo(42));
    for _ in 0..100 {
        assert_eq!(a.prossimo(Flusso::Eventi), b.prossimo(Flusso::Eventi));
    }
}

#[test]
fn flussi_indipendenti() {
    let (mut a, mut b) = (Caso::nuovo(42), Caso::nuovo(42));
    for _ in 0..10 {
        b.prossimo(Flusso::Eventi);
    }
    assert_eq!(a.prossimo(Flusso::Economia), b.prossimo(Flusso::Economia));
}

#[test]
fn flussi_diversi_danno_numeri_diversi() {
    let mut c = Caso::nuovo(42);
    assert_ne!(c.prossimo(Flusso::Economia), c.prossimo(Flusso::Eventi));
}

#[test]
fn caso_serializzato_riprende_dallo_stesso_punto() {
    let mut a = Caso::nuovo(7);
    for _ in 0..5 {
        a.prossimo(Flusso::Sondaggi);
    }
    let mut b: Caso = serde_json::from_str(&serde_json::to_string(&a).unwrap()).unwrap();
    assert_eq!(a.prossimo(Flusso::Sondaggi), b.prossimo(Flusso::Sondaggi));
}

#[test]
fn semi_estremi() {
    Caso::nuovo(0).prossimo(Flusso::Economia);
    Caso::nuovo(u64::MAX).prossimo(Flusso::PartitiIa);
}

// Fallisce se un aggiornamento di rand_chacha cambia i numeri
// (docs/ricerca/tecnica/architettura-simulazione.md:19).
#[test]
fn valore_fissato_dell_algoritmo() {
    assert_eq!(Caso::nuovo(42).prossimo(Flusso::Economia), VALORE_FISSATO);
}
```

`serde_json` va fra le `[dev-dependencies]`. `VALORE_FISSATO` è una `const u64` nel file di test: al primo giro mettila a `0`.

- [ ] **Step 3: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test caso`
Expected: errore di compilazione, `Caso` e `Flusso` non esistono.

- [ ] **Step 4: Implementa `Caso` in `crates/sim-core/src/caso.rs`**

Un `ChaCha8Rng` per flusso, creato con `ChaCha8Rng::seed_from_u64(seme)` seguito da `set_stream(flusso as u64)`; `prossimo` chiama `next_u64()` sul flusso richiesto. Struttura dei flussi a scelta (array indicizzato dal discriminante o `BTreeMap<Flusso, ChaCha8Rng>`), purché serializzabile e senza `HashMap`.

- [ ] **Step 5: Fissa il valore dell'algoritmo**

Run: `cargo test -p sim-core --test caso valore_fissato_dell_algoritmo`
Expected: FAIL con `left: <numero>, right: 0`. Copia `<numero>` in `VALORE_FISSATO`.

- [ ] **Step 6: Esegui tutti i test**

Run: `cargo test -p sim-core`
Expected: PASS, 6 test.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock .gitignore crates/sim-core
git commit -m "feat(core): seeded ChaCha8 streams per subsystem"
```

---

### Task 2: Dati di gioco, ruoli dei dicasteri e manopole

**Files:**
- Create: `dati/base/dicasteri.toml`
- Create: `dati/base/manopole.toml`
- Create: `crates/sim-core/src/dati.rs`
- Create: `crates/sim-core/src/errore.rs`
- Modify: `crates/sim-core/src/lib.rs` (aggiunge `pub mod dati; pub mod errore;`)
- Modify: `crates/sim-core/Cargo.toml` (aggiunge `toml`)
- Test: `crates/sim-core/tests/dati.rs`
- Test: `crates/sim-core/tests/comune/mod.rs`

**Ricerca:** `docs/ricerca/tecnica/modding-e-dati-esterni.md:14-15` (TOML per le entità strutturate, validazione all'avvio con errori leggibili); `docs/ricerca/sintesi/decisioni.md:315` (ruoli nei dati, rifiuto se orfano o doppio); spec, «Le quattro cuciture», cucitura 4.

**Dispatch:** `rust-engineer` · dopo Task 1: stessi `lib.rs`, `crates/sim-core/Cargo.toml`, `Cargo.lock` · skill-gate: true.

**Interfaces:**
- Consumes: niente dal Task 1.
- Produces:
  - `sim_core::dati::FileDati { pub dicasteri: String, pub manopole: String }`: i testi dei file, letti da chi fa I/O.
  - `sim_core::dati::DatiGioco { pub dicasteri: Vec<Dicastero>, pub manopole: BTreeMap<String, Manopola> }` — `Clone, Debug, PartialEq, Serialize, Deserialize`; `DatiGioco::da_testi(file: &FileDati) -> Result<DatiGioco, Errore>`.
  - `sim_core::dati::Dicastero { pub id: String, pub nome: String, pub indicatore_servizio: String, pub peso_spartizione: u8, pub regione_bersaglio: bool, pub ruoli: Vec<Ruolo> }`.
  - `sim_core::dati::Ruolo` — `enum { ColpaDisoccupazioneTrasferimenti, ColpaImposteImpiegoPubblico, BilancioECredibilita }`, serde `rename_all = "snake_case"`, `Display` che stampa lo stesso nome snake_case; `Ruolo::TUTTI: [Ruolo; 3]`.
  - `sim_core::dati::Manopola { pub valore: f64, pub min: f64, pub max: f64, pub ticket: String }`.
  - `sim_core::errore::Errore` — `Debug, PartialEq`, `Display`, `std::error::Error`, con le varianti e i messaggi sotto (testo esatto):
    - `RuoloOrfano { ruolo: Ruolo }` → `ruolo senza dicastero: {ruolo}`
    - `RuoloDoppio { ruolo: Ruolo, primo: String, secondo: String }` → `ruolo assegnato a due dicasteri: {ruolo} ({primo}, {secondo})`
    - `DicasteroDoppio { id: String }` → `dicastero ripetuto: {id}`
    - `FuoriIntervallo { nome: String, valore: f64, min: f64, max: f64 }` → `manopola fuori intervallo: {nome} = {valore} (ammesso da {min} a {max})`
    - `Toml { file: String, messaggio: String }` → `{file}: {messaggio}`
    - `SchemaDiverso { trovata: u32, attesa: u32 }` → `salvataggio di una versione diversa del gioco: schema {trovata}, atteso {attesa}` (usata nel Task 3)
    - `SalvataggioIllegibile(String)` → `salvataggio illeggibile: {0}` (usata nel Task 3)
  - Test helper `tests/comune/mod.rs`: `pub fn file_base() -> FileDati` (con `include_str!("../../../../dati/base/…")`) e `pub fn dati_base() -> DatiGioco`.

La regione bersaglio spetta a due dicasteri, quindi è il campo `regione_bersaglio` e non un `Ruolo`: i ruoli sono unici per definizione. La colpa di un indicatore di servizio va al dicastero che lo eroga, quindi basta `indicatore_servizio`.

- [ ] **Step 1: Scrivi i dati di base**

`dati/base/dicasteri.toml`, un `[[dicastero]]` per ciascuno dei dieci, con i valori della spec (indicatori da ticket 03, pesi di spartizione provvisori da ticket 05):

| id | nome | indicatore_servizio | peso | regione_bersaglio | ruoli |
|---|---|---|---|---|---|
| economia_finanze | Economia e Finanze | efficienza_riscossione | 3 | false | colpa_imposte_impiego_pubblico, bilancio_e_credibilita |
| sviluppo_economico | Sviluppo economico | sostegno_imprese | 1 | true | — |
| lavoro_welfare | Lavoro e Welfare | copertura_welfare | 1 | false | colpa_disoccupazione_trasferimenti |
| sanita | Sanità | qualita_servizio_sanitario | 1 | false | — |
| istruzione_ricerca | Istruzione e Ricerca | qualita_scuola | 1 | false | — |
| infrastrutture_trasporti | Infrastrutture e Trasporti | qualita_trasporti | 1 | true | — |
| ambiente_energia | Ambiente ed Energia | sicurezza_energetica | 1 | false | — |
| interno | Interno | sicurezza | 2 | false | — |
| giustizia | Giustizia | durata_processi | 1 | false | — |
| esteri_difesa | Esteri e Difesa | prontezza_difesa | 2 | false | — |

`dati/base/manopole.toml`, una tabella per manopola con `valore`, `min`, `max`, `ticket` (intervalli provvisori, al bilanciamento):

| nome | valore | min | max | ticket |
|---|---|---|---|---|
| soglia_minaccia | 40.0 | 30.0 | 50.0 | "04" |
| soglia_dimissioni | 25.0 | 15.0 | 35.0 | "04" |
| turni_legge_ordinaria | 2.0 | 2.0 | 4.0 | "08" |

- [ ] **Step 2: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/dati.rs
mod comune;
use comune::file_base;
use sim_core::dati::{DatiGioco, FileDati, Ruolo};
use sim_core::errore::Errore;

fn con_dicasteri(testo: &str) -> FileDati {
    FileDati { dicasteri: testo.into(), ..file_base() }
}

#[test]
fn dati_base_si_caricano() {
    let d = DatiGioco::da_testi(&file_base()).unwrap();
    assert_eq!(d.dicasteri.len(), 10);
    assert!(d.dicasteri.iter().any(|x| x.nome == "Sanità"));
    assert_eq!(d.dicasteri.iter().filter(|x| x.regione_bersaglio).count(), 2);
    assert_eq!(d.manopole["soglia_minaccia"].valore, 40.0);
}

#[test]
fn ruolo_orfano_rifiutato_col_nome() {
    let testo = file_base().dicasteri.replace("\"colpa_disoccupazione_trasferimenti\"", "");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(e, Errore::RuoloOrfano { ruolo: Ruolo::ColpaDisoccupazioneTrasferimenti });
    assert_eq!(e.to_string(), "ruolo senza dicastero: colpa_disoccupazione_trasferimenti");
}

#[test]
fn ruolo_doppio_rifiutato_col_nome() {
    let testo = format!(
        "{}\n[[dicastero]]\nid = \"bis\"\nnome = \"Bis\"\nindicatore_servizio = \"x\"\npeso_spartizione = 1\nregione_bersaglio = false\nruoli = [\"bilancio_e_credibilita\"]\n",
        file_base().dicasteri
    );
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(e.to_string().starts_with("ruolo assegnato a due dicasteri: bilancio_e_credibilita"));
}

#[test]
fn ruolo_sconosciuto_rifiutato_col_nome() {
    let testo = file_base().dicasteri.replace("\"bilancio_e_credibilita\"", "\"bilancio_e_credibilta\"");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert!(matches!(e, Errore::Toml { .. }));
    assert!(e.to_string().contains("bilancio_e_credibilta"));
}

#[test]
fn dicastero_ripetuto_rifiutato() {
    let base = file_base().dicasteri;
    let primo = base.split("[[dicastero]]").nth(1).unwrap();
    let testo = format!("{base}\n[[dicastero]]{primo}");
    let e = DatiGioco::da_testi(&con_dicasteri(&testo)).unwrap_err();
    assert_eq!(e, Errore::DicasteroDoppio { id: "economia_finanze".into() });
}

#[test]
fn manopola_fuori_intervallo_rifiutata_col_nome() {
    let manopole = file_base().manopole.replacen("valore = 40.0", "valore = 90.0", 1);
    let e = DatiGioco::da_testi(&FileDati { manopole, ..file_base() }).unwrap_err();
    assert!(e.to_string().starts_with("manopola fuori intervallo: soglia_minaccia = 90"));
}

#[test]
fn manopola_con_intervallo_rovesciato_rifiutata() {
    let manopole = file_base().manopole.replacen("min = 30.0", "min = 60.0", 1);
    let e = DatiGioco::da_testi(&FileDati { manopole, ..file_base() }).unwrap_err();
    assert!(matches!(e, Errore::FuoriIntervallo { ref nome, .. } if nome == "soglia_minaccia"));
}
```

I test assumono che nel file dei dicasteri Economia e Finanze venga per primo, e nel file delle manopole `soglia_minaccia`: è l'ordine della tabella dello Step 1.

- [ ] **Step 3: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test dati`
Expected: errore di compilazione, `sim_core::dati` non esiste.

- [ ] **Step 4: Implementa `DatiGioco::da_testi` in `crates/sim-core/src/dati.rs` ed `Errore` in `crates/sim-core/src/errore.rs`**

Ordine dei controlli: parse TOML (`Errore::Toml` col nome del file, `dicasteri.toml` o `manopole.toml`) → id ripetuti → per ogni `Ruolo::TUTTI` zero titolari (orfano) o più di uno (doppio, coi primi due id) → per ogni manopola, in ordine di nome, `min <= valore <= max`. Le manopole entrano nel `BTreeMap` direttamente dalle tabelle TOML di primo livello.

- [ ] **Step 5: Esegui i test**

Run: `cargo test -p sim-core`
Expected: PASS, tutti i test dei Task 1-2.

- [ ] **Step 6: Commit**

```bash
git add dati crates/sim-core
git commit -m "feat(core): load and validate ministries, roles and knobs"
```

---

### Task 3: Partita, vista e salvataggio (seam 1)

**Files:**
- Create: `crates/sim-core/src/partita.rs`
- Create: `crates/sim-core/src/vista.rs`
- Modify: `crates/sim-core/src/lib.rs` (aggiunge `pub mod partita; pub mod vista;` e riesporta `Partita`, `Vista`, `Trimestre`, `DatiGioco`, `FileDati`, `Errore` alla radice)
- Modify: `crates/sim-core/Cargo.toml` (`serde_json` passa da dev a dipendenza normale)
- Test: `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/tecnica/architettura-simulazione.md:32-34` (snapshot completo con seme, versione dello schema e dati; replay solo a parità di binario); `docs/ricerca/tecnica/architettura-simulazione.md:40` e `docs/ricerca/sintesi/decisioni.md:297` (decisione del 20/10: JSON, stato del generatore, copia dei dati, schema diverso rifiutato senza migrazioni); `docs/ricerca/tecnica/core-rust-e-ui-separata.md:15-16` (interfaccia stretta, nessuna logica nel frontend).

**Dispatch:** `rust-engineer` · dopo Task 2: consuma `DatiGioco`, `Errore`, `Caso` · skill-gate: true.

**Interfaces:**
- Consumes: `Caso::nuovo`, `DatiGioco`, `Errore::{SchemaDiverso, SalvataggioIllegibile}`.
- Produces:
  - `sim_core::VERSIONE_SCHEMA: u32 = 1`.
  - `sim_core::Trimestre { pub anno: u16, pub trimestre: u8 }` (trimestre 1-4) — `Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize`; `Trimestre::TURNO_ZERO = Trimestre { anno: 2026, trimestre: 4 }`; `Trimestre::successivo(self) -> Trimestre`.
  - `sim_core::Partita` — `Debug, PartialEq, Serialize, Deserialize`, campi privati: `versione_schema: u32`, `seme: u64`, `caso: Caso`, `dati: DatiGioco`, `turno: u32`, `data: Trimestre`.
    - `Partita::nuova(dati: DatiGioco, seme: u64) -> Partita`
    - `Partita::chiudi_turno(&mut self)`
    - `Partita::vista(&self) -> Vista`
    - `Partita::salva(&self) -> String` (JSON)
    - `Partita::carica(json: &str) -> Result<Partita, Errore>`
  - `sim_core::Vista { pub turno: u32, pub data: Trimestre, pub dicasteri: Vec<VoceDicastero> }`, `sim_core::vista::VoceDicastero { pub id: String, pub nome: String }`, entrambe `Clone, Debug, PartialEq, Serialize`.

`chiudi_turno` per ora porta il turno avanti di uno e la data al trimestre successivo. I piani seguenti inseriscono i passi della chiusura in quest'ordine fisso, da scrivere come commento di una riga sopra la funzione: atti dei ministri → aula → motore (economia, poi elettorato) → soglie → eventi del turno dopo; voto delle elezioni dopo il motore. Nessuna funzione vuota per i passi futuri.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/partita.rs
mod comune;
use comune::dati_base;
use sim_core::{Errore, Partita, Trimestre, VERSIONE_SCHEMA};

fn giocata(seme: u64, turni: u32) -> Partita {
    let mut p = Partita::nuova(dati_base(), seme);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

#[test]
fn turno_zero_e_ottobre_dicembre_2026() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.turno, 0);
    assert_eq!(v.data, Trimestre { anno: 2026, trimestre: 4 });
    assert_eq!(v.dicasteri.len(), 10);
}

#[test]
fn chiudere_il_turno_avanza_di_un_trimestre() {
    assert_eq!(giocata(1, 1).vista().data, Trimestre { anno: 2027, trimestre: 1 });
    let v = giocata(1, 8).vista();
    assert_eq!(v.turno, 8);
    assert_eq!(v.data, Trimestre { anno: 2028, trimestre: 4 });
}

#[test]
fn stesso_seme_stessa_partita() {
    assert_eq!(giocata(42, 8).salva(), giocata(42, 8).salva());
}

#[test]
fn salvare_e_ricaricare_non_cambia_il_seguito() {
    let mut diretta = giocata(42, 4);
    let mut ricaricata = Partita::carica(&diretta.salva()).unwrap();
    assert_eq!(ricaricata, diretta);
    for _ in 0..4 {
        diretta.chiudi_turno();
        ricaricata.chiudi_turno();
    }
    assert_eq!(ricaricata.salva(), diretta.salva());
}

#[test]
fn i_dati_viaggiano_nel_salvataggio() {
    let p = Partita::carica(&giocata(3, 1).salva()).unwrap();
    assert!(p.vista().dicasteri.iter().any(|d| d.nome == "Sanità"));
}

#[test]
fn schema_diverso_rifiutato() {
    let mut json: serde_json::Value = serde_json::from_str(&giocata(1, 1).salva()).unwrap();
    json["versione_schema"] = 99.into();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert_eq!(e, Errore::SchemaDiverso { trovata: 99, attesa: VERSIONE_SCHEMA });
    assert_eq!(
        e.to_string(),
        "salvataggio di una versione diversa del gioco: schema 99, atteso 1"
    );
}

#[test]
fn salvataggio_troncato_non_va_in_panico() {
    let s = giocata(1, 2).salva();
    let e = Partita::carica(&s[..s.len() / 2]).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIllegibile(_)));
}
```

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test partita`
Expected: errore di compilazione, `Partita` non esiste.

- [ ] **Step 3: Implementa `Partita`, `Trimestre` e `Vista` in `crates/sim-core/src/partita.rs` e `crates/sim-core/src/vista.rs`**

`carica` legge prima il JSON come `serde_json::Value` (`SalvataggioIllegibile` se non è JSON valido), confronta `versione_schema` con `VERSIONE_SCHEMA` (`SchemaDiverso` se diversa o assente, con `trovata = 0` se assente) e solo dopo deserializza in `Partita` (`SalvataggioIllegibile` col messaggio di serde). Così un salvataggio di uno schema vecchio dà il messaggio di versione e non un errore di campo.

- [ ] **Step 4: Esegui i test**

Run: `cargo test -p sim-core`
Expected: PASS, tutti i test dei Task 1-3.

- [ ] **Step 5: Verifica che il core resti puro**

Run: `grep -rnE "std::(fs|time|thread|net|env)|HashMap|StdRng|SmallRng" crates/sim-core/src`
Expected: nessuna riga.

- [ ] **Step 6: Commit**

```bash
git add crates/sim-core
git commit -m "feat(core): game state with quarterly turns and versioned snapshot"
```

---

### Task 4: Binario `sim-cli`

**Files:**
- Create: `crates/sim-cli/Cargo.toml` (`name = "sim-cli"`, dipendenze `sim-core` per path e `serde_json`)
- Create: `crates/sim-cli/src/main.rs`
- Modify: `Cargo.toml` (aggiunge `crates/sim-cli` ai membri)
- Test: `crates/sim-cli/tests/cli.rs`

**Ricerca:** `docs/ricerca/tecnica/architettura-simulazione.md:14` (simulazione senza finestra per test e bilanciamento); `docs/ricerca/tecnica/core-rust-e-ui-separata.md:14`.

**Dispatch:** `rust-engineer` · dopo Task 3: consuma `Partita`/`Vista` riesportate alla radice; tocca `Cargo.toml` radice come il Task 5 · skill-gate: true.

**Interfaces:**
- Consumes: `FileDati`, `DatiGioco::da_testi`, `Partita::{nuova, chiudi_turno, vista}`.
- Produces: comando `sim-cli --dati <cartella> --seme <u64> --turni <u32>`. Legge `dicasteri.toml` e `manopole.toml` dalla cartella, gioca i turni chiudendoli senza decisioni, stampa su stdout la `Vista` finale in JSON indentato. Con un errore stampa su stderr `errore: <messaggio>` ed esce con codice 1. Argomenti letti con `std::env::args`, senza crate di parsing. Il piano 12 vi aggiunge le strategie.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-cli/tests/cli.rs
use std::process::Command;

const DATI: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../dati/base");

fn sim(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_sim-cli")).args(args).output().unwrap()
}

#[test]
fn gioca_otto_turni() {
    let out = sim(&["--dati", DATI, "--seme", "42", "--turni", "8"]);
    assert!(out.status.success());
    let vista: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(vista["turno"], 8);
    assert_eq!(vista["data"]["anno"], 2028);
    assert_eq!(vista["data"]["trimestre"], 4);
}

#[test]
fn stesso_seme_stesso_output() {
    let args = ["--dati", DATI, "--seme", "7", "--turni", "5"];
    assert_eq!(sim(&args).stdout, sim(&args).stdout);
}

#[test]
fn cartella_dati_mancante_esce_con_errore() {
    let out = sim(&["--dati", "/non/esiste", "--seme", "1", "--turni", "1"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("/non/esiste"));
}

#[test]
fn argomento_mancante_esce_con_errore() {
    let out = sim(&["--dati", DATI]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).starts_with("errore: "));
}
```

`serde_json` va anche fra le `[dev-dependencies]` di `sim-cli`.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-cli`
Expected: errore, il pacchetto `sim-cli` non esiste.

- [ ] **Step 3: Implementa `main` in `crates/sim-cli/src/main.rs`**

`fn main() -> std::process::ExitCode`, con il lavoro in una `fn esegui() -> Result<String, String>`; i messaggi d'errore di lettura includono il percorso del file.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml Cargo.lock crates/sim-cli
git commit -m "feat(cli): headless sim-cli runs quarters from data dir"
```

---

### Task 5: Tipi TypeScript e guscio Tauri/Svelte

**Files:**
- Create: `.cargo/config.toml` (`[env] TS_RS_EXPORT_DIR = { value = "app/src/lib/tipi", relative = true }`)
- Modify: `crates/sim-core/Cargo.toml` (aggiunge `ts-rs`)
- Modify: `crates/sim-core/src/partita.rs`, `crates/sim-core/src/vista.rs` (`#[derive(TS)] #[ts(export)]` su `Vista`, `VoceDicastero`, `Trimestre`)
- Create: `app/` da template (`npm create tauri-app@latest app -- --template svelte-ts --manager npm --yes`), poi:
- Modify: `app/src-tauri/Cargo.toml` (dipendenza `sim-core` per path; nessun `[workspace]` proprio)
- Modify: `Cargo.toml` (aggiunge `app/src-tauri` ai membri)
- Modify: `app/src-tauri/tauri.conf.json` (`productName` «Simulatore politico», `identifier` `it.maeurong.simulatorepolitico`, `bundle.resources` `{ "../../dati/base/": "dati/base/" }`, `bundle.targets` solo macOS)
- Modify: `app/src-tauri/src/lib.rs`
- Modify: `app/src/routes/+page.svelte` (o `App.svelte`, secondo il template), togliendo la demo
- Create: `app/src/lib/tipi/*.ts` (generati, committati)
- Modify: `app/package.json` (versioni esatte di `@tauri-apps/cli` e `@tauri-apps/api` della serie 2.12)

**Ricerca:** `docs/ricerca/tecnica/core-rust-e-ui-separata.md:15-16` (comandi verso il core, snapshot verso la UI, niente logica nel frontend); `docs/ricerca/sintesi/decisioni.md:193` (Tauri 2.12 + core Rust, UI Svelte 5); `docs/ricerca/tecnica/tauri.md:12` (versioni esatte: `tauri` 2.12.3, `tauri-cli` 2.12.2); spec, «Architettura» (tipi TypeScript generati dal Rust, il guscio legge i file).

**Dispatch:** `rust-engineer`, intero, non spezzato · dopo Task 4: modifica `partita.rs`/`vista.rs` del Task 3 e `Cargo.toml` radice del Task 4 · skill-gate: true. Il lato Svelte sono un'intestazione, un elenco e un pulsante, provvisori fino al piano 11: spezzarlo verso `frontend-engineer` aggiunge un passaggio di consegne sui nomi dei comandi e sui tipi generati, più l'orchestrazione `impeccable` per una schermata da buttare. Lo Step 5 lo guarda Mario: finestra nativa, nessun agente la vede, e fuori da Tauri `invoke` non risponde.

**Interfaces:**
- Consumes: `FileDati`, `DatiGioco::da_testi`, `Partita::{nuova, chiudi_turno, vista}`, `Vista`, `Trimestre`.
- Produces:
  - comandi Tauri `nuova_partita() -> Result<Vista, String>` e `chiudi_turno() -> Result<Vista, String>`, con lo stato in `Mutex<Option<Partita>>` gestito da Tauri. Il seme lo sceglie il guscio dall'orologio di sistema in nanosecondi: il core non legge l'ora, e un `u64` passato da JavaScript perderebbe precisione oltre 2^53. `chiudi_turno` senza partita restituisce `Err("nessuna partita in corso")`.
  - i dati si leggono da `resource_dir()/dati/base/`.
  - tipi `Vista`, `VoceDicastero`, `Trimestre` in `app/src/lib/tipi/`.

- [ ] **Step 1: Genera i tipi TypeScript**

Run: `cargo test -p sim-core`
Expected: PASS, e compaiono `app/src/lib/tipi/Vista.ts`, `VoceDicastero.ts`, `Trimestre.ts`.

- [ ] **Step 2: Crea l'app dal template e collegala al workspace**

Run: `cargo build --workspace`
Expected: compila senza errori, con un solo `Cargo.lock` alla radice.

- [ ] **Step 3: Implementa i comandi in `app/src-tauri/src/lib.rs` e la schermata**

La schermata, unica e provvisoria finché non arriva il piano 11, mostra:
- l'intestazione `Turno {turno} · {nome del trimestre} {anno}`; i nomi dei trimestri sono, in quest'ordine, `gennaio-marzo`, `aprile-giugno`, `luglio-settembre`, `ottobre-dicembre`;
- l'elenco dei nomi dei dieci dicasteri;
- un pulsante `Chiudi turno`.

Testo minimo 12 px. Nessun calcolo di gioco nel frontend.

- [ ] **Step 4: Verifica tipi e build della UI**

Run: `npm --prefix app run check && npm --prefix app run build`
Expected: nessun errore di svelte-check; build di Vite completata.

- [ ] **Step 5: Verifica l'app a mano**

Run: `npm --prefix app run tauri dev`
Expected: finestra con «Turno 0 · ottobre-dicembre 2026» e i dieci dicasteri, «Sanità» compresa; dopo un clic su «Chiudi turno», «Turno 1 · gennaio-marzo 2027».

- [ ] **Step 6: Esegui tutta la suite**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add .cargo Cargo.toml Cargo.lock crates/sim-core app
git commit -m "feat(app): Tauri shell shows core view, generated TS types"
```
