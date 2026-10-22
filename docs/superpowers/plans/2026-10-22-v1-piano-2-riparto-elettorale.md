# V1 · Piano 2: riparto e legge elettorale — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Il riparto voti → seggi come funzione pura di `sim-core` (seam 2 della spec), con la legge 180/2026 riprodotta esattamente, il menu a gradini della legge elettorale con le sue tre impostazioni pronte, e i seggi di ogni circoscrizione calcolati dalla popolazione censita (cucitura 3).

**Architecture:** Un modulo nuovo `crates/sim-core/src/elezioni/` senza stato e senza `Caso`: `mod.rs` (tipi, dati, validazione del menu), `seggi.rs` (seggi per circoscrizione dalla popolazione), `riparto.rs` (la seam). I dati stanno in un sesto file `dati/base/elezioni.toml`: popolazione del censimento 2021 per circoscrizione, seggi costituzionali, menu e impostazioni pronte. `DatiGioco` lo carica e lo valida come gli altri; lo scenario nomina l'impostazione in vigore al turno zero. La partita non vota ancora: il voto arriva col piano 7, l'elettorato col piano 4.

**Tech Stack:** Rust 1.98, workspace esistente, nessuna dipendenza nuova. Python 3 sola libreria standard per lo script di riferimento già scritto (`strumenti/riparto/riferimento.py`).

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 70-72 (righe 110-112), «Le quattro cuciture» punto 3 (riga 260), «Elezioni e riparto» (righe 316-321), «Testing Decisions» seam 2 (riga 418) ed «Elezioni» (riga 426). Scope: tabella dei piani in [`2026-10-21-v1-piano-1-fondamenta.md:43`](2026-10-21-v1-piano-1-fondamenta.md). Ticket: `.scratch/v1-giocabile/issues/01-legge-elettorale-vigente.md`, `13-legge-elettorale-come-misura.md`, `20-partenza-della-partita.md`.

## Confini con gli altri piani

Qui solo la seam pura, le disposizioni del menu come dati e le impostazioni pronte. Restano fuori, col piano che porta ciò da cui dipendono: anteprima con la tabella dei seggi guadagnati e persi, linea dei partiti sui seggi attesi, intesa pagata dagli alleati che perdono seggi, fiducia e voto segreto, validità dal voto successivo (storie 73-76, piano 6, insieme a valore precedente, abrogazione e referendum delle disposizioni); la proiezione sul sondaggio pubblico (piani 4 e 6); il voto del turno zero e la spartizione dei seggi premio per accordo pre-voto con la cessione di posti (piano 7); il cambio del numero dei parlamentari per norma costituzionale (piano 6: qui `seggi_per_circoscrizione` regge già qualunque totale); il menu come schermata, compresi i gradini da mostrare spenti (piano 11).

## Assunzioni del piano 2

Punti che la legge non decide per un elettorato a 20 regioni, o che la spec lascia al piano (riga 488). Ognuna va scritta nel codice come commento `// assunzione A<n> del piano 2: …` accanto alla riga che la applica.

- **A1 · Estero.** Gli 8 deputati e 4 senatori esteri si ripartiscono con la formula della legge sui voti nazionali delle liste ammesse al riparto di quel ramo; esclusi dai tetti, come nella legge (`docs/ricerca/modelli/sistemi-elettorali.md:65`). L'elettorato a 20 regioni non vota all'estero e la ricerca non ha la loro ripartizione (spec riga 488).
- **A2 · Uninominali di Valle d'Aosta e Trentino-Alto Adige.** Il gioco non ha geografia sotto la regione: i seggi uninominali di una circoscrizione si danno col D'Hondt fra le coalizioni dichiarate (se la legge le ammette) e le liste non coalizzate, sui voti della regione e senza soglie; i seggi di una coalizione passano alle sue liste col D'Hondt sui voti della regione. Con un seggio solo (Valle d'Aosta) è il più votato, come nella legge.
- **A3 · Premio fra le liste della coalizione.** I seggi premio vanno alle liste ammesse della coalizione con la formula della legge sui voti nazionali; il piano 7 li ripartirà per accordo pre-voto (spec riga 320).
- **A4 · Sorteggio.** Dove la legge sorteggia (parità di resti e di cifra), vince la lista o coalizione con l'indice più basso; fra circoscrizioni, quella che viene prima nei dati.
- **A5 · Stessi voti nei due rami.** Dal 2021 Camera e Senato hanno lo stesso elettorato: `Voti` è uno solo. La verifica del premio si fa comunque ramo per ramo, perché al Senato l'ammissione col 20% regionale può cambiare i soggetti in gara.
- **A6 · Minoranze linguistiche.** Nessun partito del gioco le rappresenta: le soglie speciali per le loro liste non esistono.
- **A7 · Combinazioni impossibili.** «Premio alla coalizione» è la regola della 180: va al soggetto più votato, coalizione o lista singola, e richiede le coalizioni ammesse. «Premio alla lista» va alla lista più votata e le esclude (nessun precedente reale di premio alla lista con coalizioni; l'Italicum non le aveva). Nel riparto le due regole sono la stessa (il soggetto più votato); il menu le distingue solo per la combinazione con `coalizioni`. Coalizioni no ⇒ soglia di coalizione e ripescaggio ignorati; premio nessuno ⇒ soglia, taglia e tetto ignorati.
- **A8 · Gradini di taglia e tetto.** Ticket 13 dà «circa 9 · 18%» e «55 · 60% · nessuno». In quota dei seggi la 180 esatta è 17,5% (70/400, 35/200) e il tetto vale 55% alla Camera (220/400) e 56,5% al Senato (113/200): taglia 8,75 · 17,5%; tetto «come la 180» (55% / 56,5%), 60%, nessuno.
- **A9 · Coalizione di una lista sola** = lista non coalizzata.
- **A10 · Quoziente intero con pochi voti.** Il metodo della legge usa il quoziente troncato (`totale / seggi` intero). Se i voti sono così pochi che il quoziente è 0 o le parti intere superano i seggi (servono meno di `seggi × (seggi + 1)` voti: mai in una partita, solo nei test), si usa il quoziente naturale esatto.
- **A11 · Nessun soggetto ammesso.** I seggi proporzionali che non trovano un soggetto ammesso restano non assegnati: la somma del ramo scende sotto il totale, nessun panic.

## Global Constraints

Valgono tutti i vincoli del piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`): italiano coi termini di `CONTEXT.md`; `sim-core` senza I/O e senza thread; solo `Vec` e `BTreeMap`; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova.
- Il modulo `elezioni` non usa `f64`: voti `u64`, seggi `u32`, quote in punti base `u16` (10 000 = 100%). Ogni confronto con una soglia è in interi: `voti * 10_000 >= soglia_pb * totale` in `u64`, i confronti fra percentuali di regioni diverse per prodotto incrociato in `u128`.
- Seggi da una quota: `(quota_pb as u64 * seggi as u64 + 5_000) / 10_000`, arrotondamento a metà per eccesso.
- Il riparto è puro: stessi argomenti, stesso esito; nessuna estrazione dal `Caso`. Le parità si rompono per regola (A4), mai a caso.
- Nessun panic su un ingresso malformato: voti incoerenti col territorio → `Errore::VotiIncoerenti`.
- `VERSIONE_SCHEMA` passa a `3` nel Task 1 (lo snapshot copia `DatiGioco`, che cambia forma) e resta `3` per tutto il piano.
- Una lista è il simbolo di un partito: il riparto lavora per indice di lista; quale partito stia dietro un indice lo decide il piano 7.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.

## Review Focus

1. **Voti con una regione mancante, liste di lunghezza diversa, una lista in due coalizioni**: `Errore::VotiIncoerenti` col motivo, mai un panic né un indice fuori dai limiti. Test `voti_incoerenti_rifiutati` nel Task 3.
2. **Nessuna lista sopra soglia** (tanti partiti piccoli, 2,5% ciascuno): nessun panic, proporzionali tutti a zero (A11). Test `nessun_soggetto_ammesso_non_va_in_panico` nel Task 3.
3. **Pochissimi voti** (una partita di prova, un test): niente divisione per zero, seggi conservati (A10). Test `pochi_voti_conservano_i_seggi` nel Task 3.
4. **Tetto del Senato che non si può raggiungere** (tetto sotto premio più uninominali): l'algoritmo di sottrazione termina e la somma resta 200. Test `tetto_irraggiungibile_termina` nel Task 5.
5. **Due liste con voti identici ovunque**: esito deterministico, vince l'indice più basso (A4), mai seggi persi o doppi. Test `parita_risolta_per_ordine` nel Task 3.

## Dispatch

Tutto sequenziale, 1 → 2 → 3 → 4 → 5: ogni task tocca `crates/sim-core/src/elezioni/` e `crates/sim-core/tests/elezioni.rs`, e consuma i tipi o le funzioni del precedente. Una PR sola, `feat/v1-riparto-elettorale` da `main`.

Tutti i task a `rust-engineer`: solo Rust e il TOML che il Rust valida. Le copie integrali delle fonti citate come `docs/ricerca/fonti/*.md` sono ignorate da git (`.gitignore:3`) e non esistono nel worktree: si leggono in sola lettura dal checkout principale, `/Users/mario/GitHub/simulatore-politico/docs/ricerca/fonti/`. Le pagine wiki (`docs/ricerca/modelli/…`) ci sono.

---

### Task 1: Dati elettorali, legge elettorale e menu

**Files:**
- Create: `dati/base/elezioni.toml`
- Create: `crates/sim-core/src/elezioni/mod.rs`
- Modify: `crates/sim-core/src/lib.rs` (`pub mod elezioni;`)
- Modify: `crates/sim-core/src/dati.rs` (sesto file, campo e accessori, validazione)
- Modify: `crates/sim-core/src/scenario.rs`, `dati/base/scenario.toml` (campo `legge_elettorale`)
- Modify: `crates/sim-core/src/errore.rs` (tre varianti)
- Modify: `crates/sim-core/src/partita.rs:10` (`VERSIONE_SCHEMA = 3`)
- Modify: `crates/sim-core/tests/comune/mod.rs` (file `elezioni` in `file_base` e `testo_di`), `crates/sim-core/tests/scenario.rs:111` (test rinominato `leggi_chiede_i_sei_file_per_nome`)
- Test: `crates/sim-core/tests/elezioni.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:31` (400/200, Estero 8/4, VdA 1, TAA 7 e 6), `:48` (seggi dal censimento, premio escluse VdA/TAA), `:33-39` (valori di `legge_180`: soglie 3% e 10%, ripescaggio, 42%, 70/35); spec righe 110-112 (storie 70-72: nove parametri, tre impostazioni, premio almeno al 40%), 260 (cucitura 3), 319 (menu); gradini da `.scratch/v1-giocabile/issues/13-legge-elettorale-come-misura.md:33`.

**Dispatch:** `rust-engineer` · primo, nessuna dipendenza · skill-gate: true. Tocca `dati.rs`, `partita.rs`, `scenario.rs`, `errore.rs` e i test comuni, che nessun altro task riapre.

**Interfaces:**
- Consumes: `DatiGioco::da_testi`/`valida` (`crates/sim-core/src/dati.rs:213-307`), `Regione` (`crates/sim-core/src/struttura.rs:27`), `FileDati::NOMI` (`dati.rs:20`).
- Produces, in `sim_core::elezioni` (tutti `Clone, Debug, PartialEq, Serialize, Deserialize`; gli enum anche `Copy, Eq`, `#[serde(rename_all = "snake_case")]`):
  - `struct Circoscrizione { id: String, regione: String, popolazione: u64, #[serde(default)] uninominali: u32, #[serde(default)] seggi_fissi: Option<u32> }` — `regione` è un id di `regioni.toml`; `uninominali > 0` segna una circoscrizione speciale (Valle d'Aosta, Trentino-Alto Adige, province di Bolzano e Trento): esclusa dalla quota di premio e dal riparto nazionale.
  - `struct Ramo { seggi: u32, estero: u32, minimo: u32, #[serde(rename = "circoscrizione")] circoscrizioni: Vec<Circoscrizione> }`.
  - `struct QuoteTetto { camera_pb: u16, senato_pb: u16 }`.
  - `enum Premio { Nessuno, Lista, Coalizione }`, `enum Formula { Hare, Dhondt }`.
  - `struct LeggeElettorale { soglia_lista_pb: u16, coalizioni: bool, soglia_coalizione_pb: u16, ripescaggio: bool, premio: Premio, soglia_premio_pb: u16, taglia_premio_pb: u16, #[serde(default)] tetto: Option<QuoteTetto>, formula: Formula }` — i nove parametri del menu, nell'ordine della storia 71; `tetto` assente = nessun tetto.
  - `struct Menu { soglia_lista_pb: Vec<u16>, soglia_coalizione_pb: Vec<u16>, soglia_premio_pb: Vec<u16>, taglia_premio_pb: Vec<u16>, tetto: Vec<QuoteTetto> }` — i gradini; booleani ed enum ammettono tutti i valori, `tetto` ammette anche l'assenza.
  - `struct DatiElezioni { soglia_regionale_senato_pb: u16, camera: Ramo, senato: Ramo, menu: Menu, impostazioni: BTreeMap<String, LeggeElettorale> }`.
  - `impl LeggeElettorale { pub fn valida(&self, menu: &Menu) -> Result<(), Errore> }` — ogni valore numerico è un gradino del menu (`tetto` presente è uno dei `menu.tetto`); premio `Coalizione` con `coalizioni == false` e premio `Lista` con `coalizioni == true` rifiutati (A7). La UI e il piano 6 la chiamano per sapere se una legge è ammessa.
  - `impl DatiElezioni { pub(crate) fn valida(&self, regioni: &[Regione]) -> Result<(), Errore> }`.
  - `DatiGioco::elezioni(&self) -> &DatiElezioni`; `DatiGioco::legge_elettorale_iniziale(&self) -> &LeggeElettorale` (con `expect("impostazione validata al caricamento")`, come `manopola`, `dati.rs:198-207`).
  - `Scenario.legge_elettorale: String`, nome di un'impostazione.
  - `Errore::LeggeElettorale(String)` («legge elettorale non ammessa: {m}»), `Errore::DatiElettorali(String)` («dati elettorali incoerenti: {m}»), `Errore::VotiIncoerenti(String)` («voti incoerenti: {m}», la usa il Task 3).
  - `FileDati.elezioni: String`, `FileDati::NOMI` a sei voci con `"elezioni.toml"` in coda.

Contenuto di `dati/base/elezioni.toml` (intestazione con le fonti: Costituzione artt. 56-57; legge 9 ottobre 2026, n. 180; popolazione del censimento permanente 2021, DPR 20 gennaio 2023, GU n. 53 del 3/3/2023, come riportata nel dossier della Camera AC 2822-B, tabelle 1 e 6; quote in punti base):

- `soglia_regionale_senato_pb = 2000`.
- `[camera]` `seggi = 400`, `estero = 8`, `minimo = 0`; 28 `[[camera.circoscrizione]]` in quest'ordine (id, regione, popolazione): `piemonte_1` piemonte 2208370 · `piemonte_2` piemonte 2047980 · `lombardia_1` lombardia 4010096 · `lombardia_2` 2099602 · `lombardia_3` 2205257 · `lombardia_4` 1628049 · `veneto_1` veneto 1913482 · `veneto_2` 2934263 · `friuli_venezia_giulia` 1194647 · `liguria` 1509227 · `emilia_romagna` 4425366 · `toscana` 3663191 · `umbria` 858812 · `marche` 1487150 · `lazio_1` lazio 3822459 · `lazio_2` 1892423 · `abruzzo` 1275950 · `molise` 292150 · `campania_1` campania 2988376 · `campania_2` 2636044 · `puglia` 3922941 · `basilicata` 541168 · `calabria` 1855454 · `sicilia_1` sicilia 2268634 · `sicilia_2` 2564695 · `sardegna` 1587413 · `valle_d_aosta` 123360 `uninominali = 1` · `trentino_alto_adige` 1073574 `uninominali = 4`. Dove la regione non è scritta, coincide con l'id.
- `[senato]` `seggi = 200`, `estero = 4`, `minimo = 3`; 21 `[[senato.circoscrizione]]` in quest'ordine: `piemonte` 4256350 · `valle_d_aosta` 123360 `uninominali = 1` `seggi_fissi = 1` · `lombardia` 9943004 · `bolzano` regione `trentino_alto_adige` 532616 `uninominali = 3` · `trento` regione `trentino_alto_adige` 540958 `uninominali = 3` · `veneto` 4847745 · `friuli_venezia_giulia` 1194647 · `liguria` 1509227 · `emilia_romagna` 4425366 · `toscana` 3663191 · `umbria` 858812 · `marche` 1487150 · `lazio` 5714882 · `abruzzo` 1275950 · `molise` 292150 `seggi_fissi = 2` · `campania` 5624420 · `puglia` 3922941 · `basilicata` 541168 · `calabria` 1855454 · `sicilia` 4833329 · `sardegna` 1587413. Commento: art. 57 Cost., «nessuna regione o provincia autonoma ha meno di tre senatori; il Molise ne ha due, la Valle d'Aosta uno».
- `[menu]` `soglia_lista_pb = [0, 300, 400, 500]`, `soglia_coalizione_pb = [0, 1000]`, `soglia_premio_pb = [4000, 4200, 4500]`, `taglia_premio_pb = [875, 1750]`, `tetto = [{ camera_pb = 5500, senato_pb = 5650 }, { camera_pb = 6000, senato_pb = 6000 }]`; commento con A8 e la sentenza 1/2014 e 35/2017 per il 40%.
- `[impostazioni.legge_180]`: 300, `true`, 1000, `true`, `"coalizione"`, 4200, 1750, `tetto = { camera_pb = 5500, senato_pb = 5650 }`, `"hare"`.
- `[impostazioni.proporzionale_3]`: 300, `false`, 0, `false`, `"nessuno"`, 4000, 875, nessun tetto, `"hare"`.
- `[impostazioni.proporzionale_puro]`: come `proporzionale_3` con `soglia_lista_pb = 0`.

`scenario.toml`: `legge_elettorale = "legge_180"` fra le chiavi di primo livello.

Validazione (`DatiElezioni::valida`, poi in `DatiGioco::valida` il nome dello scenario), ogni errore col nome dell'elemento:
- menu: ogni lista di gradini non vuota, crescente, ogni valore ≤ 10 000; ogni `soglia_premio_pb` ≥ 4000, altrimenti `LeggeElettorale` col messaggio che contiene `soglia_premio_pb` e il valore (storia 72);
- ogni impostazione passa `LeggeElettorale::valida`, e il messaggio nomina impostazione e parametro;
- per ramo: id unici, `popolazione > 0`, `regione` presente fra le regioni di `regioni.toml`, ogni regione coperta da almeno una circoscrizione, `estero + Σ seggi_fissi ≤ seggi`;
- per regione, la popolazione sommata della Camera uguale a quella del Senato (due copie dello stesso censimento: un refuso in una delle due si vede qui);
- `scenario.legge_elettorale` è una chiave di `impostazioni`, altrimenti `LeggeElettorale` col nome.

- [ ] **Step 1: Scrivi i test in `crates/sim-core/tests/elezioni.rs`**

Modulo `mod comune;` come gli altri crate di test. Test:
- `elezioni_base_si_caricano`: `dati_base().elezioni()` ha 28 circoscrizioni della Camera e 21 del Senato, `impostazioni` con chiavi `legge_180`, `proporzionale_3`, `proporzionale_puro`; `legge_elettorale_iniziale()` è uguale a `LeggeElettorale { soglia_lista_pb: 300, coalizioni: true, soglia_coalizione_pb: 1000, ripescaggio: true, premio: Premio::Coalizione, soglia_premio_pb: 4200, taglia_premio_pb: 1750, tetto: Some(QuoteTetto { camera_pb: 5500, senato_pb: 5650 }), formula: Formula::Hare }`.
- `soglia_premio_sotto_il_40_rifiutata`: `sostituisci(file_base(), "elezioni", "soglia_premio_pb = [4000", "soglia_premio_pb = [3900")` → `Errore::LeggeElettorale(m)` con `m` che contiene `"soglia_premio_pb"` e `"3900"`.
- `impostazione_fuori_dal_menu_rifiutata_col_nome`: `soglia_lista_pb = 300` della `legge_180` portato a 250 → messaggio con `"legge_180"` e `"soglia_lista_pb"`.
- `premio_alla_coalizione_senza_coalizioni_rifiutato` e `premio_alla_lista_con_coalizioni_rifiutato`: su una `LeggeElettorale` costruita in Rust da `legge_180` cambiando un campo, `valida(&menu)` è `Err(Errore::LeggeElettorale(_))`; `legge_180` stessa passa, e passa la variante `premio: Lista, coalizioni: false`.
- `circoscrizione_con_regione_sconosciuta_rifiutata`: `regione = "piemonte"` della prima circoscrizione → `"piemontese"`; `Errore::DatiElettorali(m)` con `"piemontese"` in `m`.
- `popolazioni_discordi_fra_i_rami_rifiutate`: popolazione del `molise` del Senato 292150 → 292151; messaggio con `"molise"`.
- `scenario_con_legge_inesistente_rifiutato`: `legge_elettorale = "legge_180"` → `"legge_2030"`; `Errore::LeggeElettorale` con `"legge_2030"`.

In `tests/scenario.rs` il test `leggi_chiede_i_cinque_file_per_nome` diventa `leggi_chiede_i_sei_file_per_nome`, invariato nel corpo.

- [ ] **Step 2: Lancia i test e verifica che falliscano**

Run: `cargo test -p sim-core --test elezioni`
Expected: FAIL di compilazione (`elezioni` non esiste in `sim_core`).

- [ ] **Step 3: Implementa tipi, dati, validazione e schema 3** come da Interfaces; `FileDati::leggi` legge il sesto file per nome, così CLI (`crates/sim-cli/src/main.rs:55`) e guscio Tauri (`app/src-tauri/src/lib.rs:21`) lo leggono senza modifiche.

- [ ] **Step 4: Lancia le verifiche**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: tutto verde.

- [ ] **Step 5: Commit**

```bash
git add dati/base/elezioni.toml dati/base/scenario.toml crates/sim-core
git commit -m "feat(core): electoral data, law menu and presets"
```

---

### Task 2: Seggi per circoscrizione dalla popolazione censita

**Files:**
- Create: `crates/sim-core/src/elezioni/seggi.rs`
- Modify: `crates/sim-core/src/elezioni/mod.rs` (`mod seggi; pub use seggi::…`)
- Test: `crates/sim-core/tests/elezioni.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:48-61` (seggi e premio per circoscrizione sul censimento 2021: l'oracolo dei test); spec riga 260 (cucitura 3: seggi dalla popolazione, premio e tetto in quota).

**Dispatch:** `rust-engineer` · dopo il Task 1: legge `Ramo` e `dati_base().elezioni()` · skill-gate: true.

**Interfaces:**
- Consumes: `Ramo`, `Circoscrizione` (Task 1).
- Produces:
  - `pub struct SeggiCircoscrizione { pub id: String, pub regione: String, pub seggi: u32, pub premio: u32, pub uninominali: u32 }` (`Clone, Debug, PartialEq, Eq`).
  - `pub fn seggi_per_circoscrizione(ramo: &Ramo, premio: u32) -> Vec<SeggiCircoscrizione>`, stesso ordine di `ramo.circoscrizioni`.
  - `pub fn seggi_da_quota(quota_pb: u16, seggi: u32) -> u32` (formula nei Global Constraints).

Algoritmo, verificato contro le quattro tabelle del dossier (`docs/ricerca/fonti/camera-dossier-ac2822-legge-elettorale-2026.md:1505-1520`, `:6440-6490`, `:6910-6930`):
1. Da ripartire: `ramo.seggi − ramo.estero`. Le circoscrizioni con `seggi_fissi` li prendono subito.
2. Il resto si divide fra le altre col **quoziente naturale esatto** (art. 56-57 Cost.): parte intera `p × S / P`, resto `p × S % P` in `u64`, i seggi rimasti ai resti più alti; a parità di resto la popolazione più alta, poi l'ordine dei dati.
3. Se una circoscrizione riceve meno di `ramo.minimo`, la si fissa a `minimo` e si rifà il passo 2 sulle rimanenti, finché nessuna resta sotto.
4. `uninominali = min(c.uninominali, seggi)`.
5. `premio` (se > 0) si divide col passo 2, senza fissi né minimi, fra le sole circoscrizioni con `uninominali == 0`; le speciali hanno `premio = 0`.

- [ ] **Step 1: Scrivi i test**

Valori del dossier, nell'ordine dei dati:
- `camera_dal_censimento_2021_come_il_dossier`: `seggi_per_circoscrizione(&e.camera, 70)` → `seggi` = `[15, 14, 27, 14, 15, 11, 13, 19, 8, 10, 29, 24, 6, 10, 25, 13, 8, 2, 20, 17, 26, 4, 12, 15, 17, 10, 1, 7]`.
- `premio_della_camera_come_il_dossier`: stesso esito → `premio` = `[3, 2, 5, 2, 3, 2, 2, 4, 1, 2, 5, 4, 1, 2, 5, 2, 2, 0, 4, 3, 5, 1, 2, 3, 3, 2, 0, 0]`; Σ(`seggi − premio`) sulle circoscrizioni con `uninominali == 0` = 314; con `premio = 0` la stessa somma = 384.
- `senato_dal_censimento_2021_come_il_dossier`: `seggi_per_circoscrizione(&e.senato, 35)` → `seggi` = `[14, 1, 32, 3, 3, 16, 4, 5, 14, 12, 3, 5, 18, 4, 2, 18, 13, 3, 6, 15, 5]`.
- `premio_del_senato_come_il_dossier`: → `premio` = `[3, 0, 6, 0, 0, 3, 1, 1, 3, 2, 1, 1, 3, 1, 0, 3, 2, 0, 1, 3, 1]`; Σ plurinominali = 154, senza premio 189.
- `uninominali_delle_circoscrizioni_speciali`: Camera `valle_d_aosta` 1 e `trentino_alto_adige` 4; Senato `valle_d_aosta` 1, `bolzano` 3, `trento` 3; tutte le altre 0.
- `premio_e_tetto_in_quota_dei_seggi`: `seggi_da_quota(1750, 400) == 70`, `(1750, 200) == 35`, `(5500, 400) == 220`, `(5650, 200) == 113`, `(875, 200) == 18`, `(0, 400) == 0`.
- `ramo_ridotto_rispetta_i_minimi` (storia 69, cucitura 3): un clone del Senato con `seggi = 100` → somma 96, `molise` 2, `valle_d_aosta` 1, ogni altra ≥ 3; un clone della Camera con `seggi = 300` → somma 292.

- [ ] **Step 2: Verifica che falliscano** — `cargo test -p sim-core --test elezioni` → FAIL di compilazione.

- [ ] **Step 3: Implementa `seggi.rs`.**

- [ ] **Step 4: Verifiche** come nel Task 1, tutto verde.

- [ ] **Step 5: Commit** — `git add crates/sim-core && git commit -m "feat(core): seats per constituency from census population"`.

---

### Task 3: La seam del riparto, parte proporzionale

**Files:**
- Create: `crates/sim-core/src/elezioni/riparto.rs`
- Modify: `crates/sim-core/src/elezioni/mod.rs`
- Test: `crates/sim-core/tests/elezioni.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:33-37` (soglie, 20% regionale al Senato, ripescaggio, fine della regola dell'1%), `:42` (riparto nazionale della Camera senza VdA/TAA), `:46` (riparto regionale del Senato), `:12-13` (D'Hondt, Hare); spec righe 316-317 (riparto puro, 180 come impostazione), 418 (seam 2).

**Dispatch:** `rust-engineer` · dopo il Task 2: chiama `seggi_per_circoscrizione` · skill-gate: true. Il task più denso del piano (regole 1-6, tredici test): il controller verifica i numeri attesi contro `python3 -I strumenti/riparto/riferimento.py` prima di accettare una divergenza.

**Interfaces:**
- Consumes: `DatiElezioni`, `LeggeElettorale` (Task 1); `seggi_per_circoscrizione`, `seggi_da_quota` (Task 2); `Errore::VotiIncoerenti` (Task 1).
- Produces (tipi `Clone, Debug, PartialEq, Eq, Serialize, Deserialize`):
  - `pub struct Voti { pub coalizioni: Vec<Vec<usize>>, pub per_regione: BTreeMap<String, Vec<u64>> }` — coalizioni dichiarate come indici di lista; voti validi per regione, uno per lista.
  - `#[derive(Default, Copy)] pub struct SeggiLista { pub proporzionali: u32, pub premio: u32, pub speciali: u32, pub estero: u32 }` con `pub fn totale(&self) -> u32`. `speciali` = seggi vinti nelle circoscrizioni speciali, uninominali e proporzionali.
  - `pub enum Soggetto { Lista(usize), Coalizione(usize) }` (`Copy`), indice in `Voti.coalizioni` per la coalizione.
  - `pub struct Esito { pub camera: Vec<SeggiLista>, pub senato: Vec<SeggiLista>, pub senato_per_regione: BTreeMap<String, Vec<u32>>, pub premio: Option<Soggetto>, pub tetto_camera: bool, pub tetto_senato: bool }` — `senato_per_regione`: seggi per lista in ogni regione, premio ed estero esclusi (per la mappa, storia 138).
  - `pub fn riparto(dati: &DatiElezioni, legge: &LeggeElettorale, voti: &Voti) -> Result<Esito, Errore>`.

Regole di questo task (testo: `docs/ricerca/modelli/sistemi-elettorali.md:33-46`; soglie del Senato `docs/ricerca/fonti/camera-dossier-ac2822-legge-elettorale-2026.md:8435-8460`, della Camera `:4318-4340`). Qui premio, tetti, speciali ed estero restano a zero: li aggiungono i Task 4 e 5.

1. **Validazione dei voti** → `VotiIncoerenti`: chiavi di `per_regione` = l'insieme delle regioni delle circoscrizioni; almeno una lista; vettori tutti della stessa lunghezza; indici di coalizione nei limiti; nessuna lista in due coalizioni.
2. **Cifre nazionali** (tutte le regioni, speciali comprese) e totale `T`.
3. **Soggetti in gara**, ramo per ramo. Coalizioni = quelle dichiarate con almeno due liste se `legge.coalizioni` (A9), altrimenti nessuna. Lista *sopra soglia* = cifra nazionale ≥ `soglia_lista_pb`; al Senato anche ≥ `soglia_regionale_senato_pb` dei voti validi di almeno una regione. Coalizione *ammessa* = somma delle cifre di **tutte** le sue liste (niente più regola dell'1%) ≥ `soglia_coalizione_pb` **e** almeno una sua lista sopra soglia. Liste ammesse di una coalizione ammessa: quelle sopra soglia, più, se `legge.ripescaggio`, la più votata fra le altre. Liste non coalizzate, o di coalizioni non ammesse: soggetto a sé se sopra soglia.
4. **Metodo** (`legge.formula`), per ogni riparto fra soggetti o fra liste:
   - *Hare della legge*: quoziente `q = T / S` intero; parte intera `c / q`, resto `c % q`; seggi rimasti ai resti più alti, a parità la cifra più alta, poi l'indice (A4); ricaduta sul quoziente naturale quando `q == 0` o le parti intere superano `S` (A10).
   - *D'Hondt*: `S` volte il seggio al quoziente `c / (s + 1)` più alto, confrontato per prodotto incrociato; a parità la cifra più alta, poi l'indice. Nessun resto.
   - Il metodo restituisce, per ogni soggetto, i seggi e (solo con Hare) il resto e se un seggio è venuto dal resto: servono al tetto del Senato (Task 5).
5. **Camera**: seggi da ripartire `N` = Σ (`seggi − premio`) delle circoscrizioni con `uninominali == 0` (qui `premio = 0`, quindi 384); cifre nazionali **senza** i voti delle regioni delle circoscrizioni speciali; riparto fra i soggetti; i seggi di una coalizione passano alle sue liste ammesse col metodo, sulle stesse cifre senza speciali.
6. **Senato**: per ogni circoscrizione con `uninominali == 0`, `seggi − premio` ripartiti fra i soggetti sulle cifre della sua regione; i seggi di una coalizione alle sue liste ammesse sulle cifre regionali (art. 17 d.lgs. 533). `senato_per_regione` somma per regione.

- [ ] **Step 1: Scrivi i test**

Helper nel file di test: `voti_g1()` e `voti_g2()`. Per ogni regione `r`, `p_r` = somma delle popolazioni delle circoscrizioni della Camera di `r`; voti della lista `l` = `(p_r / 2) * millesimi[l] / 1000` in `u64`, divisioni intere in quest'ordine. Righe di millesimi per macroarea della regione (`dati.regioni()`), undici liste, `coalizioni = [[0, 1, 2, 3], [4, 5, 6]]`:
- G1: nord `[320, 90, 25, 10, 200, 60, 20, 140, 50, 20, 65]`, centro `[300, 90, 25, 10, 220, 60, 20, 140, 50, 20, 65]`, mezzogiorno `[280, 90, 25, 10, 240, 60, 20, 140, 50, 20, 65]`; Sardegna sostituita da `[280, 90, 25, 10, 240, 60, 20, 0, 50, 225, 0]`.
- G2: nord `[400, 90, 25, 10, 150, 50, 20, 120, 50, 20, 65]`, centro `[380, 90, 25, 10, 170, 50, 20, 120, 50, 20, 65]`, mezzogiorno `[360, 90, 25, 10, 190, 50, 20, 120, 50, 20, 65]`; stessa riga per la Sardegna.

Numeri attesi da `strumenti/riparto/riferimento.py` (seconda lettura della legge; se diverge, si torna al testo della legge e il controller decide):
- `proporzionale_3_come_il_riferimento`: impostazione `proporzionale_3`, G1 → `proporzionali` Camera `[126, 38, 0, 0, 91, 25, 0, 57, 21, 0, 26]`, Senato `[64, 20, 0, 0, 45, 10, 0, 30, 9, 1, 10]`.
- `proporzionale_puro_come_il_riferimento`: `proporzionale_puro`, G1 → Camera `[116, 34, 10, 4, 84, 23, 8, 52, 19, 10, 24]`, Senato `[60, 20, 5, 0, 43, 10, 1, 29, 9, 2, 10]`.
- `senato_ammette_il_20_per_cento_regionale`: nel caso precedente con `proporzionale_3`, la lista 9 (2,55% nazionale, 22,5% in Sardegna) ha 0 seggi alla Camera e almeno 1 al Senato.
- `soglia_di_lista_esatta`: voti uniformi `p_r * millesimi[l]` (stesse quote in ogni regione), `proporzionale_3`, millesimi `[30, 29, 941]` → lista 0 > 0 seggi, lista 1 = 0 in entrambi i rami.
- `coalizione_sotto_il_10_lascia_le_liste_sole`: `legge_180`, voti uniformi `[50, 25, 300, 300, 325]`, coalizione `[0, 1]` (7,5%) → Camera lista 0 = 20 (da sola, al 5%), lista 1 = 0 (2,5%, niente ripescaggio fuori da una coalizione ammessa).
- `tutti_i_voti_della_coalizione_contano`: `legge_180`, voti uniformi `[95, 6, 300, 300, 299]`, coalizione `[0, 1]` (10,1%: ammessa, la 1 ripescata) → Camera lista 1 = 2; con `[93, 6, 300, 300, 301]` (9,9%) → lista 1 = 0. Nessun soggetto arriva al 42%: il premio del Task 5 non cambia questi numeri.
- `ripescaggio_della_piu_votata_sotto_soglia`: `legge_180`, voti uniformi `[300, 25, 15, 300, 360]`, coalizione `[0, 1, 2]` → Camera lista 1 = 10, lista 2 = 0; con `ripescaggio: false` → lista 1 = 0.
- `dhondt_favorisce_i_grandi`: G1 con `proporzionale_3` e `formula: Dhondt` → la lista 0 ha alla Camera almeno i seggi che ha con Hare.
- `senato_per_regione_somma_ai_seggi`: per G1 e ogni impostazione, Σ su regioni di `senato_per_regione[r][l]` = `senato[l].proporzionali + senato[l].speciali`.
- `voti_incoerenti_rifiutati`: regione mancante, regione in più, vettore più corto, coalizione con indice 11, lista in due coalizioni, zero liste → ciascuno `Err(Errore::VotiIncoerenti(_))`.
- `nessun_soggetto_ammesso_non_va_in_panico`: `legge_180`, 40 liste da 25 millesimi uniformi, nessuna coalizione → `Ok`, tutti i `proporzionali` a 0.
- `pochi_voti_conservano_i_seggi`: un voto per lista per regione, tre liste, `proporzionale_puro` → Camera Σ `proporzionali` = 384, Senato = 189.
- `parita_risolta_per_ordine`: voti uniformi `[400, 400, 200]`, `proporzionale_3` → due chiamate danno lo stesso `Esito`; Camera lista 0 ≥ lista 1, differenza ≤ 1; Σ = 384.

- [ ] **Step 2: Verifica che falliscano** — FAIL di compilazione.

- [ ] **Step 3: Implementa `riparto.rs`** con le regole 1-6.

- [ ] **Step 4: Verifiche** come nel Task 1, tutto verde.

- [ ] **Step 5: Commit** — `git add crates/sim-core && git commit -m "feat(core): proportional seat allocation with 2026 thresholds"`.

---

### Task 4: Circoscrizioni speciali ed estero

**Files:**
- Modify: `crates/sim-core/src/elezioni/riparto.rs`
- Test: `crates/sim-core/tests/elezioni.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:63` (VdA e TAA: uninominali, 3 proporzionali TAA alla Camera, voti che contano per soglie e 42%), `:65` (Estero, esclusi dai tetti); spec riga 488 (Estero come punto aperto, chiuso da A1).

**Dispatch:** `rust-engineer` · dopo il Task 3: stesso `riparto.rs`, riusa metodo e soggetti · skill-gate: true.

**Interfaces:**
- Consumes: `riparto`, metodo e soggetti del Task 3.
- Produces: `SeggiLista.speciali` ed `estero` popolati; `senato_per_regione` con gli speciali. Dopo questo task, senza premio, ogni ramo somma al suo totale (400 e 200) quando c'è almeno un soggetto ammesso.

Regole (`docs/ricerca/modelli/sistemi-elettorali.md:63-65`):
1. **Uninominali** di ogni circoscrizione speciale, in entrambi i rami: assunzione A2. Nessuna soglia.
2. **Camera, parte proporzionale delle speciali** (Trentino-Alto Adige, `seggi − uninominali` = 3): col metodo della legge fra i soggetti ammessi a livello nazionale, sui voti della regione; i seggi di una coalizione alle sue liste ammesse sui voti della regione. Contano in `speciali`.
3. **Estero**: assunzione A1, `dati.camera.estero` e `dati.senato.estero`.

- [ ] **Step 1: Scrivi i test**
- `speciali_ed_estero_come_il_riferimento`: G1, `proporzionale_3` → Camera `speciali` `[4, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0]`, `estero` `[3, 1, 0, 0, 2, 0, 0, 1, 0, 0, 1]`; Senato `speciali` `[5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0]`, `estero` `[1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0]`. Con `proporzionale_puro`: Camera `speciali` uguali, `estero` `[2, 1, 0, 0, 2, 1, 0, 1, 0, 0, 1]`; Senato uguali al caso al 3%.
- `ogni_ramo_somma_al_suo_totale`: G1 e G2 con `proporzionale_3` e `proporzionale_puro` → Σ `totale()` = 400 alla Camera e 200 al Senato.
- `valle_d_aosta_al_piu_votato`: voti uniformi `[100, 600, 300]`, `proporzionale_puro` → in `senato_per_regione["valle_d_aosta"]` l'unico seggio è della lista 1.
- `uninominali_senza_soglia`: `legge_180`, 40 liste da 25 millesimi uniformi, nessuna coalizione, con la riga della Valle d'Aosta sostituita da lista 0 = 600 e le altre 10 (lista 0 al 2,6% nazionale) → alla Camera la lista 0 ha `proporzionali` = 0 e `speciali` ≥ 1 (il seggio della Valle d'Aosta). Al Senato la stessa lista passa per il 20% regionale: è l'unico soggetto ammesso e prende tutti i 189 regionali, come vuole la legge.

- [ ] **Step 2: Verifica che falliscano** — i test nuovi falliscono sui valori attesi (speciali ed estero a zero).

- [ ] **Step 3: Implementa le regole 1-3.**

- [ ] **Step 4: Verifiche** come nel Task 1, tutto verde.

- [ ] **Step 5: Commit** — `git add crates/sim-core && git commit -m "feat(core): special constituencies and abroad seats"`.

---

### Task 5: Premio di governabilità e tetti

**Files:**
- Modify: `crates/sim-core/src/elezioni/riparto.rs`
- Test: `crates/sim-core/tests/elezioni.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:39` (premio col 42% nelle due Camere allo stesso soggetto, 70/35), `:43` (tetto 220: `150 − seggi VdA/TAA`), `:46` (tetto 113 e sottrazione dei seggi); spec righe 317 e 426 (premio solo col 42% in entrambe, tetti).

**Dispatch:** `rust-engineer` · dopo il Task 4: il tetto conta gli `speciali` del vincitore · skill-gate: true.

**Interfaces:**
- Consumes: Task 2-4.
- Produces: `Esito.premio`, `SeggiLista.premio`, `tetto_camera`, `tetto_senato`; la 180 completa (storia 70).

Regole (testo: `docs/ricerca/fonti/gu-legge-180-2026-art1.md:573-700` per la Camera, `gu-legge-180-2026-art2.md:273-420` per il Senato; sintesi `docs/ricerca/modelli/sistemi-elettorali.md:39-46`):
1. **Premio** se `legge.premio != Nessuno`: in ciascun ramo il soggetto ammesso con la cifra nazionale più alta (cifre con le speciali; parità A4) deve avere almeno `soglia_premio_pb` del totale nazionale; si assegna solo se lo è in entrambi i rami **ed è lo stesso soggetto** (A5). Seggi premio `P = seggi_da_quota(taglia_premio_pb, ramo.seggi)`; `seggi_per_circoscrizione(ramo, P)` dà le quote per circoscrizione, quindi `N` scende a 314 e i seggi regionali del Senato a 154. Il premio passa alle liste ammesse del vincitore con A3.
2. **Tetto della Camera** se c'è premio e `legge.tetto`: `C = seggi_da_quota(tetto.camera_pb, camera.seggi)`. Se premio + proporzionali + speciali del vincitore (estero escluso) superano `C`, il vincitore riceve nel riparto nazionale `C − P − speciali` (saturando a 0) e gli altri soggetti si dividono `N` meno quel numero col metodo, sulle loro cifre senza speciali (art. 83 c. 1-ter). `tetto_camera = true`.
3. **Tetto del Senato** con `C = seggi_da_quota(tetto.senato_pb, senato.seggi)`, eccesso `E` = totale del vincitore (premio + regionali + speciali) − `C`. Se `E > 0` (art. 16-bis c. 1-ter), `tetto_senato = true`, e si tolgono `E` seggi al vincitore così:
   a. nelle regioni dove ha preso un seggio col resto, in ordine di resto crescente su un'unica graduatoria nazionale (parità: ordine dei dati), uno per regione;
   b. se ne restano, nelle regioni in ordine di percentuale regionale del vincitore crescente (prodotto incrociato in `u128`; parità: ordine dei dati), uno per regione, saltando quelle già toccate in (a) e quelle dove il vincitore ha un solo seggio; si ripete (b), ora senza escludere quelle di (a) ma sempre saltando quelle con un solo seggio, finché `E` è a zero **o un giro intero non toglie nulla** (Review Focus 4);
   c. ogni seggio tolto va, nella stessa regione, all'altro soggetto col resto più alto non usato (quel resto diventa usato); se non ce n'è (D'Hondt, o resti tutti usati), all'altro soggetto con la cifra regionale più alta; senza altri soggetti in regione, la regione si salta.
   I seggi regionali così corretti passano alle liste come nel Task 3.

- [ ] **Step 1: Scrivi i test** (numeri da `strumenti/riparto/riferimento.py`)
- `legge_180_con_premio_come_il_riferimento`: G1, `legge_180` → `premio == Some(Soggetto::Coalizione(0))`, nessun tetto; Camera `proporzionali` `[100, 30, 8, 0, 70, 19, 7, 44, 16, 0, 20]`, `premio` `[51, 15, 4, 0, 0, 0, 0, 0, 0, 0, 0]`, `speciali` `[5, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0]`, `estero` `[3, 1, 0, 0, 2, 0, 0, 1, 0, 0, 1]`; Senato `proporzionali` `[51, 12, 3, 0, 36, 10, 0, 23, 8, 1, 10]`, `premio` `[25, 8, 2, 0, 0, 0, 0, 0, 0, 0, 0]`, `speciali` `[5, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0]`, `estero` `[1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0]`.
- `tetti_220_e_113`: G2, `legge_180` → `tetto_camera` e `tetto_senato` veri; liste 0-3 senza estero: Camera 220, Senato 113. Camera `proporzionali` `[111, 26, 7, 0, 62, 18, 7, 42, 18, 0, 23]`, `premio` `[54, 13, 3, 0, …0]`, `speciali` `[6, 0, 0, 0, 2, 0, …0]`, `estero` `[3, 1, 0, 0, 1, 1, 0, 1, 0, 0, 1]`; Senato `proporzionali` `[61, 10, 2, 0, 30, 8, 1, 20, 9, 2, 11]`, `premio` `[27, 6, 2, 0, …0]`, `speciali` `[5, 0, 0, 0, 2, 0, …0]`, `estero` `[2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0]`.
- `senza_tetto_il_vincitore_supera_220`: G2, `legge_180` con `tetto: None` → flag falsi; Camera `proporzionali` `[125, 30, 8, 0, 54, 16, 7, 38, 16, 0, 20]`; Senato `proporzionali` `[66, 12, 2, 0, 29, 8, 1, 16, 9, 1, 10]`.
- `tetti_con_dhondt`: G2, `legge_180` con `formula: Dhondt` → liste 0-3 senza estero 220 e 113; Senato `proporzionali` `[63, 10, 0, 0, 53, 9, 0, 13, 1, 1, 4]`, `estero` `[3, 0, 0, 0, 1, 0, …0]`.
- `premio_solo_col_42_per_cento`: voti uniformi `p_r * millesimi[l]`, coalizione `[0, 1]`, millesimi `[400, 20, 300, 160, 120]` → `premio == Some(Soggetto::Coalizione(0))`, Camera Σ `premio` = 70, Senato 35; con `[399, 20, 300, 160, 121]` (41,9%) → `None` e Σ `premio` = 0 (storia 70, spec riga 426).
- `premio_alla_lista_senza_coalizioni`: stessi voti con `[420, 0, 300, 160, 120]`, `legge_180` con `coalizioni: false` e `premio: Lista` → `premio == Some(Soggetto::Lista(0))`, Camera `premio[0]` = 70.
- `tetto_irraggiungibile_termina`: G2, `legge_180` con `tetto: Some(QuoteTetto { camera_pb: 1000, senato_pb: 1000 })` → `Ok`, Camera Σ `totale()` = 400, Senato = 200; Camera Σ `proporzionali` delle liste 0-3 = 0.
- `ogni_ramo_somma_al_suo_totale` del Task 4 esteso a `legge_180` su G1 e G2.

- [ ] **Step 2: Verifica che falliscano** — falliscono sui valori attesi (premio a zero, nessun tetto).

- [ ] **Step 3: Implementa le regole 1-3.**

- [ ] **Step 4: Verifiche** come nel Task 1, tutto verde.

- [ ] **Step 5: Commit** — `git add crates/sim-core && git commit -m "feat(core): majority bonus and seat caps of law 180/2026"`.
