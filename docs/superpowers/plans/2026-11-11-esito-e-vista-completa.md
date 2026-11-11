# Esito del turno e Vista completa: piano di implementazione

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `sim-core` produce l'esito del turno come elenco di voci con causa e criticità, il registro per turno, le fermate di «avanti fino a novità», e porta nella Vista lo stato che le schermate del piano 11 chiedono.

**Architecture:** Un modulo nuovo `esito.rs` tiene i tipi (`VoceEsito`, `TipoVoce`, `Criticita`) e le regole pure: criticità per tipo, fusione delle gemelle, scelta delle sette. La partita raccoglie le voci in un campo `esito` che sostituisce `mosse`: ogni passo di `chiudi_turno` scrive la voce nel punto del fatto, e la composizione gira una volta, alla fine della chiusura. I moduli sotto la partita (`ministri`, `parlamento`, `economia`, `eventi`, `elettorato`) non scrivono nell'esito: rendono il fatto al passo che li chiama, al più etichettato col suo `TipoVoce`. Il registro del turno N si rilegge dal salvataggio del turno N+1 con una funzione pura; il guscio aggiunge un solo comando. La seconda PR aggiunge campi alla Vista e lo stato che oggi si calcola e si butta.

**Tech Stack:** Rust (workspace `sim-core`, `sim-cli`, `app/src-tauri`), serde, ts-rs per i tipi in `app/src/lib/tipi/`. Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-11-esito-e-vista-completa.md`](../specs/2026-11-11-esito-e-vista-completa.md). Il catalogo dei 92 tipi di voce (righe 117-273) è l'oracolo dei Task 1-7: il piano non lo ricopia.

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md` (i 135 fatti nell'ordine di `chiudi_turno`, con funzione e riga) e `docs/ricerca/sintesi/stato-per-le-esposizioni.md` (dove vive ogni dato da esporre), scritte per questo piano sul codice a `8952a06`.

**Quando si esegue:** solo col via di Mario. Base `origin/main`; se nel frattempo `main` è avanzato, i numeri di riga di questo piano e delle ricerche vanno ricontrollati col nome della funzione, che è sempre citato accanto.

| PR | Branch | Task | Schema |
|---|---|---|---|
| 1. Logica nuova | `feat/esito-del-turno` | 1-10 | 15 → 16 |
| 2. Esposizioni | `feat/vista-completa`, da `main` dopo il merge della 1 | 11-17 | 16 → 17 |

## Decisioni di Mario

Stanno nella spec, righe 39-60, e non si riaprono: la famiglia dell'evento nelle voci `evento_risolto` ed `evento_nato` (ticket «Testi del Pungolo», 11/11/2026); criticità per tipo a tre livelli; le sette (prima le critiche, poi le notevoli, gemelle fuse, ordine della chiusura); fermata sul voto alla quota del 30% dei suoi elettori; le quattro voci minori nella PR 2; redditi per fascia subito ed esatti; forme delle esposizioni del 10/11; l'anteprima della proposta d'opposizione nella PR 2, Task 16 (ticket «Aula e calendario», 11/11/2026, spec «Anteprima della proposta d'opposizione»); una voce sola per una situazione accesa col suo evento nato, Task 7 (ticket «Prima pagina», 11/11/2026, spec «Situazione accesa ed evento nato»).

## Scelte del piano

Oltre alle 22 «Scelte mie» della spec (righe 61-86), prese scrivendo il piano:

1. **`precedenza` nella voce.** La scelta delle sette gira a fine chiusura su voci già raccolte nello stato: la precedenza (3, 2, 1, oppure 0) è un campo della voce, esportato.
2. **Criticità condizionata.** `grado(tipo, al_governo)` rende il caso base del catalogo; nei sette tipi con una condizione («C2 se…») il punto del fatto la alza o la abbassa con `VoceEsito::con_grado`. L'elenco dei sette sta nel Task 1.
3. **«–» nel catalogo = la voce non nasce.** `grado` rende `None` e `Partita::voce` non scrive nulla: la regola di visibilità (all'opposizione niente soddisfazione dei ministri né intesa) sta in un posto solo.
4. **`al_governo` si legge quando nasce la voce**, non a fine chiusura: una voce nata prima della caduta del governo resta giudicata «al governo».
5. **Il riassunto di una voce fusa usa le cifre**: «10 atti eseguiti», non «Dieci».
6. **Le fermate `ultimatum` e `crisi` si leggono dall'esito**: c'è la fermata se l'ultimo esito contiene `ultimatum_posto` o `crisi_aperta`. Nessuno stato nuovo.
7. **«Al governo» vuol dire Presidente del Consiglio**: `governo.presidente == giocatore`, in un metodo solo che usano `Partita::voce` e i punti con `con_grado`. Vale anche per l'uscente in trattativa, i cui ministri eseguono ancora. La fermata `Governo` segue invece `governa()`.
8. **`Ministri::esegui` rende i fatti** (`Vec<FattoAtto>`): servono alle voci degli atti nel Task 3 e alle opere finite nel Task 14.
9. **Un test per tipo di voce, di preferenza dentro il test che già fa accadere il fatto**: si aggiunge l'asserzione sull'esito al test esistente, e se ne scrive uno nuovo solo dove non c'è.

## Global Constraints

Valgono i vincoli del piano 9 (`2026-11-01-v1-piano-9-eventi-e-informazione.md:79-93`) e, attraverso quelli, dei piani 1-8b: italiano coi termini di `CONTEXT.md`; `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; niente `serde(default)`. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Nessuna estrazione casuale nuova: nessun task consuma un flusso di `Caso`. Stesso seme e stessi comandi danno le stesse grandezze di prima del piano.
- Termini: «voce» e «criticità» (`CONTEXT.md`), mai «livello», «gravità» o «priorità» per la criticità; «registro», «fermata», «tono grave» come nel glossario.
- Una voce non ha mai la causa vuota e non dice ciò che la Vista nasconde: mai i valori attesi in cifre dei partiti IA, mai le probabilità, mai l'intesa avversaria in cifre.
- Le voci in tono grave non hanno battute: il testo del core è sempre l'occhiello serio; il titolo spiritoso è del ticket 09 della mappa.
- `app/src/` non si tocca, salvo i tipi generati in `app/src/lib/tipi/`.
- Ogni manopola nuova ha `valore`, `min`, `max`, `ticket` ed entra in `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:70`) nel task che la introduce, col commento della fonte.
- `VERSIONE_SCHEMA` (`crates/sim-core/src/partita.rs:52`) sale a 16 nel Task 2 e a 17 nel Task 11; i campi nuovi degli altri task entrano senza alzarla.
- Ogni cargo gira con `TS_RS_EXPORT_DIR=<worktree assoluto>/app/src/lib/tipi` e `--manifest-path <worktree assoluto>/Cargo.toml`: senza, i tipi finiscono nel checkout sbagliato.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Testi di `fatto`, `causa` e dei riassunti elencati nella descrizione della PR per la revisione di Mario.
- Partita di prova e helper come nel piano 9: `al_governo()`, `all_opposizione()`, `vinta()`, `caduto()`, `edita` (`crates/sim-core/tests/comune/mod.rs:324`, `:482`, `:584`).

## Review Focus

1. **Registro fuori dai turni che ci sono:** `registro(turno)` per il turno in corso, per un turno cancellato da un riavvolgimento, per `u32::MAX`, e nella partita di prova rende un errore con il turno nel testo, mai un panic né l'esito di un altro turno. Test `registro_fuori_dai_turni_e_un_errore` nel Task 9.
2. **Salvare a metà e ricaricare:** una partita salvata e ricaricata mostra lo stesso esito, voce per voce, e la chiusura successiva dà lo stesso esito che senza salvataggio. Test `l_esito_sopravvive_al_salvataggio` nel Task 2.
3. **Due cause di caduta nella stessa chiusura:** una fiducia respinta seguita dall'esercizio provvisorio, o le dimissioni del governo avversario seguite dall'esercizio provvisorio, lasciano una sola voce `governo_caduto` con la prima causa, non l'ultima. Test `due_origini_vince_la_prima` nel Task 6.
4. **Fermata senza elettori:** un giocatore con quota zero nel sondaggio, o un incrocio con campione zero, non divide per zero e non ha la fermata `voto`. Test `senza_elettori_nessuna_fermata_sul_voto` nel Task 8.
5. **Stato che non c'è ancora:** prima del primo voto nessun voto per regione, senza programma `mes` vuoto, con un anno che manca al conto nessuna proiezione; la Vista si costruisce lo stesso. Test `vista_completa_al_turno_zero` nel Task 15.

## Dispatch

PR 1: 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8 → 9 → 10. I task 3-7 scrivono tutti dentro `chiudi_turno` e i suoi passi in `partita.rs`.

PR 2: dopo il merge della 1, 11 → 12 → 13 → 14 → 15 → 16 → 17. Tutti aggiungono campi a `Vista` e alla funzione `vista()`.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task. A fine PR, review in parallelo: `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer` (legge i testi delle voci elencati nella PR), `rust-reviewer` e `spec-reviewer` (questo piano e la spec), con i findings di quest'ultimo in un report a parte. Poi un solo giro di correzioni. Le decisioni prese in esecuzione si elencano nella PR. Il merge lo decide Mario.

**Architect (11/11), sequenza e parallelo.** Sui file reggono due paralleli, tutti e due piccoli. **PR 1: il 9 accanto alla catena 3 → 8**, in un secondo worktree, appena fuso il 2: gli serve solo `Partita.esito` salvato. Tocca `partita.rs` in un punto solo, accanto a `carica` (`:4487-4502`, il controllo di versione da estrarre), poi `tests/salvataggi.rs`, `app/src-tauri/src/archivio.rs` e `lib.rs`; nessuno dei task 3-8 entra in quelle righe né nel guscio. L'unico file in comune è `tests/salvataggi.rs`, che il Task 6 elenca per `partito_sciolto_e_salvataggio_ricaricabile`: quel test va in `tests/esito.rs` e il conflitto sparisce. Per tenerla corta basta **8 ∥ 9**: l'8 tocca `vista.rs`, `partita/esito.rs`, `vista()` (`partita.rs:3943-4104`), `dati.rs`, `manopole.toml`, i tipi e `tests/fermate.rs`, a quattrocento righe dal punto del 9. **PR 2: 12 ∥ 13** dopo l'11. Il 12 tocca `informazione/sondaggio.rs`, `tests/sondaggi.rs` e i tipi `Incrocio.ts` e `Rilevazione.ts`, che nessun altro task della PR tocca; il 13 tocca `vota_parlamento` (`partita.rs:2810-2847`), `legislatura/voto.rs`, `tests/elezioni.rs` e `EsitoElezioni.ts`. Il resto resta in sequenza. I task 3-7 scrivono nel corpo di `chiudi_turno` (`:546-725`) e si contendono `tests/debito.rs` (3, 4, 5), `tests/eventi.rs`, `shock.rs` ed `emendamenti.rs` (3, 7), `partita/ia.rs` e `tests/governo_ia.rs` (2, 6, 7); il 6 legge le variazioni nate nei passi del 3 e del 4. L'8 viene dopo il 6, perché le fermate `ultimatum` e `crisi` leggono voci che il 6 scrive. L'11, il 14, il 15, il 16 e il 17 aggiungono tutti un campo al letterale `Vista { … }` di `vista()`; il 13 e il 14 scrivono tutti e due in `vota_parlamento`; il 15 ha un test che guarda le regioni del 13 (`vista_completa_al_turno_zero`). Dipendenze fra PR: `indice_pericolosita` e `FattoAtto` nascono nel Task 3 e li usano l'11 e il 14; `RedditoFascia` nasce nel 5 e lo usa il 17.

**Architect (11/11), task troppo grandi per una review sola** (proposta, il piano non è riscritto). **Task 4** → 4a `Conteggio.franchi` e il voto di leggi, decreti e bilancio (`vota`, `approva_bilancio`, `ricondotto_al_tetto_mes`, `decade`): 13 tipi, da `leggi_slittate` a `confermativo_in_attesa` più `misura_decaduta`, col tipo esportato che cambia e uno dei due avvisi del piano 9; 4b referendum e sfiducia (`vota_confermativi`, `vota_abrogativo`, `referendum_ia`, `sfiducia`): 8 tipi. **Task 5** → 5a «Partiti e campagna» (8 tipi, passi F-H, con `campagna_tagliata`); 5b «Motore» senza il reddito (13 tipi, passo I); 5c `giudizio` che rende `RedditoFascia`, la manopola e `reddito_deluso`: è il solo pezzo che cambia un'interfaccia fuori dalla partita, la PR 2 ci poggia sopra, e un reviewer può accettare le voci e respingere l'aggregato. **Task 6** → 6a `dimissioni: Option<OrigineCaduta>` e `governo_caduto` (un tipo solo, ma sette punti di scrittura, sette di lettura e i test che scrivono `true` nel JSON); 6b «Voto» e il resto di «Leader, uscite, crisi, governo» (15 tipi, passi J e L-Q); 6c «Soglie» (6 tipi, passo K), il solo pezzo che legge le variazioni e chiede un segno in testa alla chiusura (riga Architect del Task 6). Il **Task 2** è lungo ma non si spezza: tolta `mossa`, niente compila finché le 25 chiamate e i test non sono convertiti. Al reviewer serve la tabella «mossa → tipo» nella PR; il resto del diff è conversione di asserzioni.

**Architect (11/11), assegnazione e skill-gate.** `rust-engineer` confermato per tutti e diciassette, compreso il 9: nel guscio tocca solo `archivio.rs` e `lib.rs`, il crate è un membro del workspace (`Cargo.toml:2`), un comando nuovo non chiede permessi (`app/src-tauri/capabilities/default.json:6` ha solo `core:default`) e `app/src/` resta fuori. Skill-gate: true per tutti, false solo per il Task 10 (un elenco tenuto a mano e un commento, nessuna regola nuova). Anche il 12, il più piccolo, fissa una formula e il caso del campione zero.

**Architect (11/11), premesse che valgono per più task.** (1) **`al_governo` non è definito in termini di codice**, e il codice ha due test diversi: `governa()` (`partita.rs:2724-2728`), falso in trattativa e a Camere sciolte, e `governo.presidente == giocatore`, quello con cui la Vista mostra l'intesa (`voci_alleati`, `:4431`). In trattativa i ministri dell'uscente eseguono ancora (`:556-557`, `:564-571`): con `governa()` le sue `proposta_respinta`, `minaccia_di_dimissioni` e `soddisfazione_variata` non nascerebbero («–»). Proposta: `presidente == giocatore`, in un metodo solo che usano `Partita::voce` e i punti con `con_grado`; la fermata `Governo` resta su `governa()`. Si decide nel Task 2 e si scrive nel commento di `voce`. (2) **Ogni campo di stato nuovo entra in `controlla_coerenza`** (`partita.rs:4998`) nel task che lo crea, con lunghezze, indici dentro i dati e numeri finiti: `esito` (Task 2), `dimissioni` contro `voto_anticipato` (6), la disoccupazione di prima (11), `EsitoElezioni.regioni` (13), `opere` (14), `redditi_fasce` (17). Il piano lo dice solo per il Task 2, e la carica è la classe di difetti che in questo repo si ripete (`91e56ca`, `8868a38`, `5c423b2`). (3) **Molte voci non hanno un fatto da leggere ma un prima e un dopo**: `minaccia_rientrata` (`partita/vita.rs:122-127`), `situazione_spenta` (`eventi.accese`), `ultimatum_posto` e `ultimatum_ritirato` (`intesa.ultimatum`), `minaccia_di_dimissioni`, `procedura_ue`, `ristrutturazione_finita`. Bastano copie locali prese in `chiudi_turno` prima del passo: nessuno stato nuovo e nessuna firma da cambiare nei moduli sotto.

**Decisioni d'esecuzione (11/11, thread principale)**, sulle righe Architect. Dove una riga Architect e il testo di un task dicono cose diverse, vale questo elenco.

1. **Paralleli accettati:** il 9 accanto alla catena 3 → 8 in un secondo worktree, dal Task 2 fuso (o almeno 8 ∥ 9); 12 ∥ 13 dopo l'11.
2. **Task spezzati per il dispatch e per la review**, senza riscrivere il piano: 4a/4b, 5a/5b/5c, 6a/6b/6c, come nella riga Architect. Il brief di ogni pezzo copia la riga Architect del suo task.
3. **«Si dimette» in apertura:** tipo nuovo `governo_dimissionario`, il novantaduesimo (Scelta 21 della spec). Confermato da Mario l'11/11/2026.
4. **`al_governo`:** Presidente del Consiglio (Scelta 7).
5. **Due origini di caduta nella stessa chiusura:** vince la prima; il punto che arriva dopo non sovrascrive. `voto_anticipato` (`crates/sim-core/src/legislatura/mod.rs:77`) si toglie se `Some(VotoAnticipato)` lo copre in tutte le letture; altrimenti la coerenza fra i due entra nel controllo al caricamento.
6. **Variazioni nate dentro la chiusura:** in testa a `chiudi_turno` si conta quante variazioni del turno corrente ci sono già, per ministro e per alleato. Le voci si scrivono per le altre, alla fine di ogni passo che può variare soddisfazione o intesa, prima di ogni `nuovo_governo` (che azzera, e il conto torna a zero) e a fine chiusura.
7. **Le voci del motore nascono solo nella chiusura vera** (`con_caso`), mai nelle copie di `avanza_copia`.
8. **`FattoAtto.tipo: TipoVoce` resta**: un'enum senza dati non crea cicli, e la riga Architecture lo dice.
9. **Voci «prima e dopo»** (`minaccia_rientrata`, `situazione_spenta`, `ultimatum_posto`, `ultimatum_ritirato`, `minaccia_di_dimissioni`, `procedura_ue`, `ristrutturazione_finita`): copie locali prese in `chiudi_turno` prima del passo, nessuno stato nuovo.
10. **Ogni campo di stato nuovo entra in `controlla_coerenza`** (`crates/sim-core/src/partita.rs:4998`) nel task che lo crea.
11. **Le due conversioni provvisorie del Task 2** (`crates/sim-core/src/partita/ia.rs:865`, `:830`) si fondono nel Task 6 con la voce scritta nel punto del fatto: una voce, non due.
12. **Campagna tagliata:** la somma degli acquisti si rifà dal prezzo dei tipi comprati. **Reddito per fascia:** nell'aggregato entra la crescita ripulita, mai quella grezza.
13. **Scelta ritirata.** La «Scelta 7» di cui parla la riga Architect del Task 2 (validazione senza controllo del partito) non c'è più: un partito sciolto resta in `partiti`, il controllo si tiene. La Scelta 7 di oggi è `al_governo`. Per lo stesso motivo il terzo punto del Review Focus è cambiato.
14. **Nomi:** il figlio resta `partita/esito.rs`, e in `partita.rs` si scrive `crate::esito::componi`.

---

# PR 1: logica nuova

### Task 1: tipi, criticità e le sette

**Files:**
- Create: `crates/sim-core/src/esito.rs`
- Modify: `crates/sim-core/src/lib.rs` (`pub mod esito;`)
- Test: unit test in fondo a `esito.rs`

**Ricerca:** `docs/ricerca/sistemi/feedback-visivo-dello-stato.md:36`, `:46` (gerarchia per impatto, niente notifica per singolo oggetto); spec righe 90-120 (la voce), 122-278 (catalogo), 280-288 (le sette).

**Interfaces:**
- Consumes: `crate::eventi::{Famiglia, Tono}`.
- Produces:

```rust
#[serde(rename_all = "snake_case")]
pub enum TipoVoce { EmergenzaLieve, EventoRisolto, /* … le 92 righe del catalogo, in ordine */ GovernoDimissionario }
#[serde(rename_all = "snake_case")]
pub enum Criticita { Critica, Notevole, Ordinaria }
pub struct VoceEsito {
    pub tipo: TipoVoce, pub criticita: Criticita, pub precedenza: u8, pub in_evidenza: bool,
    pub tono: Tono, pub fatto: String, pub causa: String,
    pub partito: Option<String>, pub dicastero: Option<String>, pub regione: Option<String>,
    pub misura: Option<u32>, pub famiglia: Option<Famiglia>, pub dettagli: Vec<String>,
}
pub const MASSIMO_IN_EVIDENZA: usize = 7;
pub(crate) enum Grado { C3, C2, C1, N, O }
pub(crate) fn grado(tipo: TipoVoce, al_governo: bool) -> Option<Grado>;
pub(crate) fn fondibile(tipo: TipoVoce) -> bool;
pub(crate) fn riassunto(tipo: TipoVoce, n: usize) -> String;
impl VoceEsito {
    pub(crate) fn nuova(tipo: TipoVoce, g: Grado, fatto: String, causa: String) -> VoceEsito;
    pub(crate) fn con_grado(&mut self, g: Grado) -> &mut VoceEsito;
}
pub(crate) fn componi(voci: &mut Vec<VoceEsito>, giocatore: &str);
```

`VoceEsito`, `TipoVoce` e `Criticita` derivano `Serialize, Deserialize` e si esportano in TypeScript come gli altri tipi della Vista.

`grado` e `fondibile` sono `match` esaustivi sulle colonne G, O e F del catalogo. Per i sette tipi condizionati `grado` rende il caso base, e il punto del fatto usa `con_grado`:

| Tipo | Base (G / O) | Si cambia in |
|---|---|---|
| `evento_risolto` | N / N | C1 al governo, se risolto per default |
| `legge_respinta` | O / N | C2 al governo, se la legge è del governo |
| `referendum_votato` | N / N | C2 al governo, se il governo perde |
| `scissione` | N / N | C2 se è il partito del giocatore o un partito di governo |
| `minaccia_di_scissione` | O / O | C2 se è del giocatore, N se è di governo |
| `procedura_ue` | N / O | C2 al governo e N all'opposizione, se lo stato peggiora |
| `ministro_esce` | C2 / O | N al governo, se le dimissioni le ha chieste il giocatore o per rimpasto |

`componi`, nell'ordine: (1) alza a notevole ogni voce ordinaria con `partito == giocatore`, tranne `Sondaggio`, `Revisione`, `EntrateDiCassa`; (2) fonde, per ogni tipo fondibile, le voci con lo stesso tipo e la stessa criticità al posto della prima: `dettagli` = una riga «fatto: causa» per voce, `fatto` = `riassunto(tipo, n)`, `causa` = la più frequente o «più cause» se nessuna prevale, `partito`/`dicastero`/`regione`/`misura` solo se uguali in tutte, `tono` grave se una lo è; (3) segna `in_evidenza`: le critiche, e se sono più di sette le sette con `precedenza` più alta e a parità le prime; se sono meno, le prime notevoli fino a sette. Non riordina mai.

- [ ] **Step 1: test che falliscono**, in `esito.rs`:
  - `ogni_tipo_ha_un_grado`: `grado(FiduciaRespinta, true) == Some(C3)`, `grado(SoddisfazioneVariata, false) == None`, `grado(GuidaRifiutata, true) == None`, `grado(AttoEseguito, true) == Some(O)`; `TipoVoce` ha 92 varianti (conteggio da un array di tutte).
  - `nove_critiche_ne_restano_sette`: 3 voci C3, 3 C2, 3 C1 mescolate; in evidenza 7, fuori le ultime due C1 nell'ordine.
  - `due_critiche_e_otto_notevoli`: in evidenza le 2 critiche e le prime 5 notevoli.
  - `le_ordinarie_non_entrano_mai`: 3 ordinarie → nessuna in evidenza.
  - `dieci_atti_una_voce`: 10 `AttoEseguito` → 1 voce, `dettagli.len() == 10`, `fatto == "10 atti eseguiti"`, nel posto della prima.
  - `una_fondibile_sola_resta_com_e`: `dettagli` vuoto, `fatto` invariato.
  - `criticita_diverse_non_si_fondono`: due `MinistroEsce`, una C2 e una N → restano due.
  - `il_soggetto_e_il_giocatore`: `PromessaVerificata` con `partito = Some("x")` e giocatore `"x"` → notevole, e non si fonde con quelle degli altri partiti; `EntrateDiCassa` del giocatore resta ordinaria e si fonde.
  - `causa_della_fusa`: tre voci con cause «a», «a», «b» → «a»; con «a», «b» → «più cause».
  - `l_ordine_non_cambia`: i tipi non fusi escono nell'ordine d'ingresso.
- [ ] **Step 2:** `cargo test -p sim-core esito::` → falliscono (modulo assente).
- [ ] **Step 3:** implementa `esito.rs` come da Interfaces. `riassunto`: una frase per tipo fondibile, «N <fatto al plurale>».
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat(core): tipi dell'esito, criticità e scelta delle sette`.

### Task 2: l'esito nello stato e nella Vista, le mosse diventano voci

**Files:**
- Create: `crates/sim-core/src/partita/esito.rs`, `crates/sim-core/tests/esito.rs`
- Modify: `crates/sim-core/src/partita.rs` (`VERSIONE_SCHEMA` `:52`; campo `mosse` `:126-127`, `:320`; `chiudi_turno` `:546-725`; `vista()` `:4062`; validazione `:4810-4815`; unit test `:5436-6150`), `crates/sim-core/src/partita/ia.rs` (`mossa` `:24-31` e le sue 17 chiamate), `crates/sim-core/src/partita/vita.rs` (5 chiamate), `crates/sim-core/src/vista.rs` (`mosse` `:89-90`, `MossaIa` `:180-186`), `crates/sim-core/tests/comune/mod.rs`, i 10 file di `crates/sim-core/tests/` che leggono `.mosse`, `app/src/lib/tipi/` (rigenerati; `MossaIa.ts` cancellato)

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:26-33` (premesse: cosa porta `mosse` oggi), `:313-337` (cause già strutturate e cause solo testuali).

**Architect (11/11):** premesse verificate: 25 chiamate di `mossa` (3 in `partita.rs:2320`, `:2534`, `:3387`; 17 in `partita/ia.rs`; 5 in `partita/vita.rs`), dieci file di `tests/` che leggono `.mosse`, `mosse.clear()` a `:550`, validazione a `:4810-4815`. `voce` si può chiamare da tutti e 25 i punti: `mossa` prende già `&mut self` senza prestiti aperti, e un metodo `pub(super)` di `partita/esito.rs` si vede anche da `ia.rs` e `vita.rs`. Cinque cose che il piano non vede. (1) **Due mosse su 25 non hanno un tipo nel catalogo:** «Si dimette: …» del governo IA in crisi, scritta in apertura (`ia.rs:710`, `:769`). Il catalogo dà al fatto 127 solo `coalizione_allargata`, e `governo_caduto` nasce alla chiusura dopo (`partita.rs:667`). O la riga sparisce e il giocatore lo sa da `Vista.dimissioni`, o `governo_caduto` nasce due volte in due esiti, o serve un tipo in più. È una scelta di prodotto, da chiedere a Mario prima del dispatch. I tre motivi di quella riga sono provati uno per uno (`tests/governo_ia.rs:724-725`, `:743-744`; `tests/legislatura.rs:208`) e sono il frutto di un fix (`5ec430c`): l'origine unica `CrisiSenzaUscita` del Task 6 li schiaccia, se la causa della voce non li ripete. (2) **Due conversioni sono provvisorie:** `ia.rs:865` (voto anticipato → `governo_caduto`) e `ia.rs:830` (formatore → `governo_formato`) scrivono voci che il Task 6 scrive di nuovo nel punto del fatto (`partita.rs:667`, `forma_governo`): lì si fondono, non si raddoppiano. (3) **La Scelta 7 poggia su una premessa falsa:** un partito non esce mai da `self.partiti`, lo scioglimento alza `sciolto` (`vita.rs:275`) e l'unico `partiti.pop()` del crate è in un test (`partiti.rs:343`). L'id di una voce esiste sempre: il controllo di `:4810-4815` si tiene, portato su `voce.partito`, e gli si aggiungono causa non vuota e al massimo sette in evidenza. Il Review Focus 3 resta come test, ma prova un caso che non può rompersi. (4) **Nomi:** `partita/esito.rs` è un figlio di `partita`, quindi dentro `partita.rs` `esito::componi` cerca nel figlio e non compila. Va scritto `crate::esito::componi`, oppure il figlio prende un altro nome (`partita/voci.rs`); `Esito` in `partita.rs:13` è già il riparto. (5) **Il partito:** `voce` non lo prende, e `mossa` lo ricavava da sola dall'indice (`ia.rs:25-29`); ognuno dei 25 punti dovrebbe clonare l'id prima della chiamata. Un secondo ingresso con l'indice del partito evita 25 copie. In più: i test unitari che leggono `p.mosse` stanno anche in `partita/ia.rs:1176-1865` e in `partita/vita.rs:417-462`, fuori dai «10 file»; `VoceEsito` deriva `PartialEq`, perché `salva` confronta lo stato riletto (`:4465`).

**Interfaces:**
- Consumes: Task 1.
- Produces:

```rust
// partita/esito.rs
impl Partita {
    /// La voce nasce col grado del catalogo per chi governa ora; `None` se il catalogo dice «–».
    pub(super) fn voce(&mut self, tipo: TipoVoce, fatto: String, causa: String) -> Option<&mut VoceEsito>;
    /// Come `voce`, col partito di indice `j` nel campo `partito`: l'ingresso delle 25 ex mosse.
    pub(super) fn voce_di(&mut self, j: usize, tipo: TipoVoce, fatto: String, causa: String) -> Option<&mut VoceEsito>;
    /// Il giocatore è Presidente del Consiglio (Scelta 7).
    pub(super) fn al_governo(&self) -> bool;
}
// vista.rs
pub struct Vista { /* … */ pub esito: Vec<VoceEsito>, /* al posto di `mosse` */ }
// tests/comune/mod.rs
pub fn voci(p: &Partita, tipo: TipoVoce) -> Vec<VoceEsito>;
```

Il campo `Partita.esito: Vec<VoceEsito>` prende il posto di `mosse`: svuotato in testa a `chiudi_turno`, composto con `crate::esito::componi` come ultima istruzione della chiusura, dopo le mosse dell'apertura. `VoceEsito` deriva `PartialEq`. La validazione al caricamento tiene il controllo del partito di oggi, portato su `voce.partito` (un partito sciolto resta in `partiti`), e aggiunge causa non vuota e al massimo sette in evidenza; l'esito entra in `controlla_coerenza`. I test unitari che leggono `p.mosse` stanno anche in `partita/ia.rs` e `partita/vita.rs`.

Ognuna delle 25 chiamate di `mossa` diventa una chiamata di `voce` col tipo del catalogo che copre quel fatto (colonna «Fatti», contro la colonna della traccia in `fatti-della-chiusura.md`). Il testo di oggi si divide: ciò che è successo in `fatto`, il perché in `causa`. `partito` = l'id che `mossa` scriveva.

- [ ] **Step 1: test che falliscono**, in `tests/esito.rs`:
  - `al_turno_zero_l_esito_e_vuoto`: `al_governo().vista().esito.is_empty()`.
  - `nessuna_causa_vuota_mai_piu_di_sette`: dopo 8 chiusure di `al_governo()` e di `all_opposizione()`, a ogni turno nessuna voce ha `causa` vuota, nessuna ordinaria è in evidenza, le voci in evidenza sono al massimo 7.
  - `stesso_seme_stesso_esito`: due partite uguali, stessi comandi, 6 chiusure → `vista().esito` uguale a ogni turno.
  - `l_esito_sopravvive_al_salvataggio`: `Partita::carica(&p.salva()?)` ha lo stesso `vista().esito`; chiudere l'originale e la ricaricata dà lo stesso esito.
  - `schema_15_rifiutato`: un salvataggio con `versione_schema` 15 dà `Errore::SchemaDiverso { trovata: 15, attesa: 16 }`.
  - I test che oggi leggono `.mosse` passano a `voci(&p, tipo)` e confrontano `fatto` e `causa`.
- [ ] **Step 2:** `cargo test -p sim-core --test esito` → non compila (`esito` assente dalla Vista).
- [ ] **Step 3:** implementa: campo, `voce`, `voce_di`, `al_governo`, schema 16, Vista, validazione, le 25 conversioni, `componi` a fine chiusura. Nella PR la tabella «mossa → tipo».
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati; `npm --prefix app run check` verde.
- [ ] **Step 5:** commit `feat(core)!: l'esito del turno sostituisce le mosse (schema 16)`.

### Task 3: voci di eventi risolti, default e atti

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (`risolvi_eventi` `:4157-4246`; default delle decisioni `:564-569`; atti `:570-571`), `crates/sim-core/src/ministri/agenda.rs` (`esegui` `:327-370`), `crates/sim-core/src/ministri/proposte.rs`, `crates/sim-core/src/parlamento/emendamenti.rs`, `crates/sim-core/src/eventi/emergenze.rs` (`probabilita` `:40-60`)
- Test: `crates/sim-core/tests/eventi.rs`, `emergenze.rs`, `scandali.rs`, `debito.rs`, `shock.rs`, `proposte.rs`, `emendamenti.rs`, `agenda.rs`, `atti.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:44-79` (passi B, C, D: fatti 2-20), `:280-311` (quelli senza traccia).

**Architect (11/11):** `esegui` può rendere i fatti senza rompere nessuno: ha un chiamante solo (`partita.rs:570`), `con_contesto` è generico sul ritorno (`:765`) e i cinque fatti stanno già nei rami di `ministri/agenda.rs:336-367`. La firma diventa `-> (Vec<EffettoAttivo>, Vec<FattoAtto>)`. Tre correzioni. (1) **Altre due firme cambiano e non sono in Interfaces:** `decidi_proposte` (`ministri/proposte.rs:278`, oggi rende `()`) deve dire respinte e decadute; `respingi_aperti` (`parlamento/emendamenti.rs:183`, oggi `Vec<usize>` di soli alleati) deve dire anche la misura, che la voce `emendamento_respinto` cita. (2) **`FattoAtto.tipo: TipoVoce` porta `crate::esito` dentro `ministri`**, contro la riga Architecture («i moduli sotto la partita non conoscono l'esito»). È un'enum senza dati e non crea cicli: si può tenere, ma allora la riga Architecture si corregge nello stesso commit; l'alternativa è un'enum locale di cinque varianti che la partita traduce. (3) **`indice_pericolosita`:** `probabilita` vuole un `Trimestre` e ne legge solo il numero (`eventi/emergenze.rs:40-60`); se il massimo fra le regioni è zero l'indice vale 0, non NaN, che nella Vista diventerebbe `null`. Il Task 11 chiede anche `totale`, che questa firma non sa rendere: l'atteso non normalizzato va in una funzione sua, così l'11 somma e divide senza riaprire il modulo. `atto_saltato` nasce solo per margine insufficiente o dimissioni chieste: l'atto fermato ha già lasciato la sua variazione al comando (spec riga 116) ed `esegui` lo scarta a `:348`. Riferimento in più, per la pericolosità a parole: `docs/ricerca/dati/pericolosita-naturale-regioni.md:121-124`.

**Interfaces:**
- Consumes: `Partita::voce`, `VoceEsito::con_grado`.
- Produces:

```rust
// ministri/agenda.rs
pub(crate) struct FattoAtto {
    pub tipo: TipoVoce,           // AttoEseguito, AttoSaltato, CantiereAvviato, CantiereFermo, CantiereFinito
    pub dicastero: usize, pub atto: String, pub regione: Option<usize>,
    pub euro: i64, pub turni: u32, pub motivo: String,
}
// `Ministri::esegui` rende `(Vec<EffettoAttivo>, Vec<FattoAtto>)`, i fatti nell'ordine d'esecuzione.
// `decidi_proposte` (ministri/proposte.rs:278) rende le proposte respinte e quelle decadute;
// `respingi_aperti` (parlamento/emendamenti.rs:183) rende, con l'alleato, anche la misura.

// eventi/emergenze.rs
/// Emergenze attese l'anno nella regione `r` per il tipo `tipo` (0 sisma, 1 idro, 2 incendi):
/// `probabilita` sommata sui quattro trimestri, non normalizzata.
pub(crate) fn attese_pericolosita(dati: &DatiGioco, tipo: usize, r: usize) -> f64;
/// `attese_pericolosita` diviso il massimo fra le regioni: 0-1; 0 se il massimo è zero.
pub(crate) fn indice_pericolosita(dati: &DatiGioco, tipo: usize, r: usize) -> f64;
/// «alta» da 2/3, «media» da 1/3, «bassa» sotto.
pub(crate) fn a_parole(indice: f64) -> &'static str;
```

Tipi di questo task (13): le tabelle «Eventi risolti», «Default delle decisioni», «Atti dei ministri» del catalogo. `evento_risolto` per default al governo prende `con_grado(C1)`. Una voce nata da un evento di tono grave porta `tono = Tono::Grave`. `evento_risolto` porta `famiglia = Some(..)`, quella dell'evento; gli altri tipi `None`. La causa di `emergenza_lieve` dice la pericolosità con `a_parole(indice_pericolosita(..))`; per la siccità dice la stagione (spec riga 365).

- [ ] **Step 1: test che falliscono.** Un'asserzione sull'esito per ciascuno dei 13 tipi, nel test che già fa accadere il fatto (Scelta 9). I primi, perché oggi non lasciano traccia: `evento_risolto_per_default_e_critico` (voce `EventoRisolto`, `criticita == Critica`, `fatto` contiene «per default», `famiglia` è quella dell'evento), `atto_eseguito_lascia_la_voce` (dicastero, regione ed euro giusti), `cantiere_finito_lascia_la_voce`, `proposta_respinta_per_silenzio`. In più `indice_di_pericolosita_fra_zero_e_uno`: per ogni tipo una regione vale 1, tutte stanno in 0-1.
- [ ] **Step 2:** `cargo test -p sim-core --test eventi --test agenda --test proposte` → falliscono sulle voci mancanti.
- [ ] **Step 3:** implementa le voci nei tre passi; `esegui` rende i `FattoAtto`.
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat(core): voci di eventi risolti, default e atti`.

### Task 4: voci dell'aula, franchi tiratori, bilancio ricondotto

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (`aula` `:788-819` e i passi che chiama; `oltre_il_tetto_mes` `:1741`, `ricondotto_al_tetto_mes` `:1765`), `crates/sim-core/src/parlamento/mod.rs` (`Conteggio`), `crates/sim-core/src/parlamento/voto.rs` (`conta` `:126-139`); fuori da `aula`: `decadi_tutto` (chiamata a `partita.rs:2803`), `decadi_calendario` (`:3551-3553`), la legge elettorale che decade in apertura (`:681-685`)
- Test: `crates/sim-core/tests/voto.rs`, `decreti.rs`, `bilancio_in_aula.rs`, `referendum.rs`, `norme.rs`, `opposizione.rs`, `debito.rs`, `calendario.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:80-112` (passo E: fatti 21-48), `:350-360` (bilancio ricondotto).

**Architect (11/11):** `conta` è a `parlamento/voto.rs:126-139`, non `:84-139` (da `:75` c'è `probabilita_franco`). Il franco tiratore è già la variabile `contro` (`:130`): `franchi` è una riga. I letterali `Conteggio { si: 0, no: 0 }` da aggiornare sono otto: `partita.rs:1038-1039`, `:3738`, `parlamento/mod.rs:997-998`, `voto.rs:127`, `tests/norme.rs:209-210`. `voto_sfiducia` (`partita.rs:3720-3750`) costruisce il conteggio a blocchi con `..c`: lì `franchi` resta zero da solo. **Quattro punti di nascita dei tipi di questo task stanno fuori da `aula`:** `misura_decaduta` per la fine della legislatura (`decadi_tutto`, chiamata a `partita.rs:2803`, fatto 75) e per la caduta del governo (`decadi_calendario`, `:3551-3553`, fatto 101), `referendum_decaduto` per lo scioglimento (fatto 76, lo stesso `decadi_tutto`) e la legge elettorale che decade in apertura (`:681-685`, fatto 38). Si aggiungono a **Files**, e alla riga **Ricerca** `fatti-della-chiusura.md:165-166` e `:225`.

**Interfaces:**
- Consumes: `Partita::voce`.
- Produces: `Conteggio { pub si: u32, pub no: u32, pub franchi: u32 }`: i seggi che hanno votato contro la linea del loro partito. Zero dove il voto non li estrae (spec riga 343).

Tipi di questo task (21): la tabella «Aula» del catalogo. Le voci dei voti si scrivono leggendo `EsitoVoto` ed `EsitoReferendum` nello stesso passo che li produce. `legge_respinta` del governo e `referendum_votato` perso dal governo prendono `con_grado(C2)` al governo. `bilancio_ricondotto` porta crescita e limite di `oltre_il_tetto_mes` e la quota tenuta.

- [ ] **Step 1: test che falliscono.** Un'asserzione per ciascuno dei 21 tipi. In più: `i_franchi_tiratori_si_contano` (un voto con franchi tiratori: `franchi` uguale ai seggi fuori linea, e la voce `LeggeApprovata` li dice per ramo); `sotto_fiducia_zero_franchi`; `bilancio_ricondotto_lascia_la_voce` (sul caso di `il_bilancio_presentato_prima_del_mes_entra_ricondotto`, `tests/debito.rs:597-631`: la voce porta crescita e limite); `valore_non_applicato_per_il_memorandum`.
- [ ] **Step 2:** `cargo test -p sim-core --test voto --test debito` → falliscono.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati (`Conteggio` è esportato).
- [ ] **Step 5:** commit `feat(core): voci dell'aula e franchi tiratori contati`.

### Task 5: voci di partiti, campagna e motore; reddito sotto l'atteso

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (passi F-I `:580-637`; `motore` `:1056-1097`; `spendi_campagne` `:2302`), `crates/sim-core/src/partita/vita.rs` (`scissioni` `:114-137`), `crates/sim-core/src/elettorato/giudizio.rs` (`:82-87`), `crates/sim-core/src/economia/regole_ue.rs`, `crates/sim-core/src/eventi/situazioni.rs`, `crates/sim-core/src/dati.rs` (`MANOPOLE_RICHIESTE`), `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/vita_dei_partiti.rs`, `promesse.rs`, `campagna.rs`, `acquisti.rs`, `conti.rs`, `debito.rs`, `situazioni.rs`, `sondaggi.rs`, `cassa.rs`, `elettorato.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:113-160` (passi F-I: fatti 49-74), `:340-349` (campagna tagliata), `:362-385` (storia 116).

**Architect (11/11):** `giudizio` ha i dati per l'aggregato: crescita e aspettativa per gruppo sono già calcolate (`elettorato/giudizio.rs:82-87`), la fascia è `fascia_di(i)` (`elettorato/mod.rs:112`), il peso è `Elettorato::elettori` (`:388`). Due attenzioni: i pesi si prendono prima del ciclo, che tiene `self.gruppi` in prestito mutabile (`giudizio.rs:75`); e nell'aggregato entra la crescita ripulita, quella accodata a `crescite` (`:88-92`), non quella grezza, che per un gruppo degenere è NaN e finirebbe nel testo della voce e, dal Task 17, nello stato salvato. I chiamanti non si rompono: uno in produzione (`partita.rs:1086`), due nei test, che ignorano il ritorno (`tests/elettorato.rs:687`, `:783`). `RedditoFascia` nasce già con `Serialize`, `Deserialize` e `PartialEq`: il Task 17 lo mette nello stato. **`motore` gira anche sulle copie** di `avanza_copia` (`partita.rs:1105-1122`, `motore(false)`): le voci scritte lì dentro finirebbero nell'esito delle copie. O si scrivono solo con `con_caso`, o `motore` rende i fatti e le voci nascono in `chiudi_turno` dopo `:597`. **Le voci 60-62 non chiedono firme nuove** in `regole_ue.rs`: si leggono `pde`, `conto_controllo` e le ammende dei conti prima e dopo `passo`. Manca solo sapere se il controllo è girato, perché esce senza toccare nulla se al conto manca un anno (`economia/regole_ue.rs:147-154`). **`Acquistati` non porta gli euro spesi** (`informazione/acquisti.rs:243-247`): la somma da mettere da parte prima di `:551` si rifà dal prezzo dei tipi comprati (`tipo.prezzo(&self.dati)`, `:2344`).

**Interfaces:**
- Consumes: `Partita::voce`.
- Produces:

```rust
// elettorato/giudizio.rs: `giudizio` rende anche questo, una voce per fascia, pesato sugli elettori.
pub struct RedditoFascia { pub fascia: Fascia, pub reddito: f64, pub crescita: f64, pub attesa: f64 }
```

Manopola nuova `scarto_reddito_deluso` = 0,25 (min 0, max 2, ticket `09`, `# assunzione del piano «esito e Vista completa», nessuna fonte in docs/ricerca`).

Tipi di questo task (22): le tabelle «Partiti e campagna» e «Motore». `campagna_tagliata`: in `spendi_campagne` il chiesto si somma prima della sovrascrittura; la somma degli acquisti del turno si mette da parte in testa a `chiudi_turno`, prima dell'azzeramento; la voce nasce solo per il partito del giocatore e solo se lo speso è minore del chiesto. `scissione`, `minaccia_di_scissione`, `procedura_ue` usano `con_grado` (tabella del Task 1). `reddito_deluso`: una voce per fascia con `attesa − crescita ≥ scarto_reddito_deluso`; non cita le promesse.

- [ ] **Step 1: test che falliscono.** Un'asserzione per ciascuno dei 22 tipi. In più: `campagna_tagliata_dagli_acquisti` (campagna pari alla cassa, poi un sondaggio privato: la voce porta chiesto, speso e la somma degli acquisti; `criticita == Critica`); `campagna_intera_nessuna_voce`; `reddito_deluso_sopra_lo_scarto` (scarto 0,25: a 0,25 la voce c'è, a 0,24 no); `situazione_spenta_lascia_la_voce`; `turno_quieto_ha_prima_stima_e_sondaggio`.
- [ ] **Step 2:** `cargo test -p sim-core --test campagna --test elettorato` → falliscono.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat(core): voci di partiti, campagna e motore`.

### Task 6: voci di voto, soglie, uscite, crisi e governo; perché cade un governo

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (passi J-Q `:638-672`; `vota_parlamento` `:2802-2893`; `forma_governo` `:3293-3404`; `fine_della_crisi` `:3501-3529`; `capo_dello_stato` `:3550-3571`; i punti `dimissioni = true` a `:587`, `:973`, `:3533`, `:3694` e gli altri che `grep "dimissioni = "` trova), `crates/sim-core/src/legislatura/mod.rs` (campo `dimissioni`), `crates/sim-core/src/partita/ia.rs`
- Test: `crates/sim-core/tests/elezioni.rs`, `ministri.rs`, `intesa.rs`, `crisi.rs`, `caduta.rs`, `ribaltone.rs`, `coalizione.rs`, `governo_ia.rs`, `legislatura.rs`, `esito.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:161-242` (passi J-Q: fatti 78-112), `:280-311` (alleato che esce e governo caduto non lasciano traccia).

**Architect (11/11):** i punti `dimissioni = true` sono sette e stanno tutti in file di questo task: `partita.rs:587`, `:973`, `:3533`, `:3694`, `partita/ia.rs:708`, `:762`, `:861`. Sette punti, sei origini: `ia.rs:708` e `:762` sono tutti e due `CrisiSenzaUscita`. Da aggiungere. (1) **Files incompleti.** Il tipo cambia anche dove il flag si legge (`partita.rs:657`, `:667`, `:814`, `:3505`, `:4060`, `:4925`; `ia.rs:844`), dove nasce (`partita.rs:240`, `:267`) e nei test che scrivono `true` nel JSON: `tests/comune/mod.rs:568` (l'helper `caduto`, usato da molti file), `tests/opposizione.rs:507`, `tests/vita_dei_partiti.rs:390`, `tests/caduta.rs:96`, `tests/governo_ia.rs:802`; più i test unitari di `partita.rs:5832`, `partita/ia.rs:1270-1298` e `comando.rs:345-347`. (2) **Due origini nella stessa chiusura.** La fiducia respinta alza il flag in aula (`:973`) e l'esercizio provvisorio lo rialza poco dopo (`:587`); il governo IA che si dimette in apertura (`ia.rs:708`) incontra lo stesso `:587` alla chiusura. Con un'assegnazione vince l'ultima, e la voce dice la causa sbagliata. Regola da scrivere e da provare: vince la prima. È la classe di `5ec430c` («motivo vero delle dimissioni in crisi»). (3) **`voto_anticipato`** (`legislatura/mod.rs:77`) diventa un doppione di `Some(VotoAnticipato)`: o si toglie, o la coerenza fra i due entra nel controllo di `partita.rs:4925`. (4) **Variazioni «nate dentro la chiusura».** `Variazione` e `VariazioneIntesa` portano solo `turno` (`ministri/mod.rs:104`, `parlamento/intesa.rs:57`), e un comando del turno N, la chiusura del turno N e l'apertura fatta dalla chiusura prima scrivono lo stesso numero. Serve un segno in testa a `chiudi_turno`: quante variazioni col turno corrente ci sono già, per ministro e per alleato. Quelle sopravvivono in ordine al `retain` di `intesa.rs:140` e di `ministri/mod.rs:292-293`; `nuovo_governo` azzera quelle dei ministri (`ministri/mod.rs:207-214`) e lì il segno torna a zero. La lettura non può stare al passo K: spartizione e rottura dell'impegno nascono dopo (`forma_governo`, `partita.rs:3347-3360`), il taglio dell'esercizio provvisorio in apertura (`:717-720`, fatto 126 a `fatti-della-chiusura.md:267`). (5) **Voci doppie.** `governo_caduto` e `governo_formato` hanno già una conversione del Task 2 (`ia.rs:865`, `:830`): qui si fondono nella voce unica; i tre motivi di «Si dimette» aspettano la decisione scritta nella riga Architect del Task 2. (6) `partito_sciolto_e_salvataggio_ricaricabile`: il partito sciolto resta in `self.partiti` (`vita.rs:275`), quindi «l'id del partito sparito» è l'id di un partito con `sciolto = true`; il test sta in `tests/esito.rs`, per non incrociare il Task 9. I fatti 75-77 del passo J sono del Task 4 (riga Architect lì).

**Interfaces:**
- Consumes: `Partita::voce`.
- Produces:

```rust
// legislatura/mod.rs: `dimissioni: Option<OrigineCaduta>` al posto del booleano.
#[serde(rename_all = "snake_case")]
pub enum OrigineCaduta { EsercizioProvvisorio, FiduciaRespinta, Sfiducia, DimissioniDelPresidente, CrisiSenzaUscita, VotoAnticipato }
```

`Vista.dimissioni` resta `bool` (`is_some()`).

Tipi di questo task (22): le tabelle «Voto», «Soglie», «Leader, uscite, crisi, governo». `soddisfazione_variata` e `intesa_variata` si scrivono dalle `Variazione` e `VariazioneIntesa` nate dentro la chiusura, non da quelle dei comandi del turno. `ministro_esce` per dimissioni chieste o rimpasto prende `con_grado(N)`. La causa di `governo_caduto` è l'`OrigineCaduta`.

- [ ] **Step 1: test che falliscono.** Un'asserzione per ciascuno dei 22 tipi. In più: `governo_caduto_dice_l_origine`, un caso per ognuna delle sei origini; `alleato_che_esce_lascia_la_voce`; `all_opposizione_niente_soddisfazione_ne_intesa` (dopo 6 chiusure di `all_opposizione()` nessuna voce `SoddisfazioneVariata` o `IntesaVariata`); `due_origini_vince_la_prima` (fiducia respinta in un turno che entra in esercizio provvisorio: una sola voce `GovernoCaduto`, con causa la fiducia respinta); `la_variazione_del_comando_non_e_una_voce` (un `Ferma` nel turno non lascia `SoddisfazioneVariata` alla chiusura; la variazione nata in aula sì).
- [ ] **Step 2:** `cargo test -p sim-core --test caduta --test crisi --test elezioni` → falliscono.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat(core): voci di voto, soglie, crisi e governo`.

### Task 7: voci degli eventi nuovi e dell'apertura; shock escluso

**Files:**
- Modify: `crates/sim-core/src/eventi/rischio.rs` (`estrai` `:179-188` e i suoi test), `crates/sim-core/src/partita.rs` (`nascono_i_puntuali` `:341-391`; apertura `:676-724`), `crates/sim-core/src/partita/ia.rs`
- Test: `crates/sim-core/tests/shock.rs`, `eventi.rs`, `stato_disposizioni.rs`, `partiti_ia.rs`, `governo_ia.rs`, `opposizione_ia.rs`, `emendamenti.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:243-279` (passi R, S: fatti 113-134).

**Architect (11/11):** `estrai` ha un chiamante solo in produzione (`partita.rs:361`) e sette nei test di `eventi/rischio.rs:217-319`, che confrontano con `Some(i)` e `None`: passano a `.vincitore`. Il test `shock_escluso_voce_senza_effetti` non può ottenere «la stessa chiusura senza lo shock» cambiando le innovazioni, che il passo riscrive (la stessa trappola del piano 9, Task 8): si alza `soglia_shock`, come fa `tests/shock.rs:109`. L'uniforme si consuma lo stesso e lo stato coincide, tolti l'esito e la manopola dentro `dati`. La legge elettorale che decade in apertura (`partita.rs:681-685`) è una `misura_decaduta` del Task 4, non di questo.

**Interfaces:**
- Consumes: `Partita::voce`.
- Produces:

```rust
// eventi/rischio.rs
pub(crate) struct Estratti { pub vincitore: Option<usize>, pub esclusi: Vec<usize> }
pub(crate) fn estrai(caso: &mut Caso, candidati: &[Candidato]) -> Estratti;
```

`esclusi` = i candidati usciti e non vincitori, nell'ordine. Le uniformi estratte restano una per candidato, nello stesso ordine. La voce `shock_assorbito` nasce solo per gli esclusi con `Causa::Shock`; nessun effetto si applica (spec riga 76).

Tipi di questo task (14): la tabella «Eventi nuovi e apertura», compreso `governo_dimissionario`. `evento_nato` porta la `famiglia` dell'evento, e il suo test la controlla. Quando l'evento nasce da una situazione accesa nella stessa chiusura, il passo toglie dall'esito la voce `situazione_accesa` di quella situazione (spec, «Situazione accesa ed evento nato»). Quelli già convertiti dal Task 2 (le mosse del governo avversario e dell'opposizione IA) qui prendono solo il test che manca.

- [ ] **Step 1: test che falliscono.** Un'asserzione per ciascuno dei 14 tipi. In più: `estrai_rende_gli_esclusi` (unit, due candidati a p = 1 di gravità 8 e 12: vincitore il secondo, escluso il primo); `shock_escluso_voce_senza_effetti` (uno shock e un'emergenza più grave nello stesso turno: la voce `ShockAssorbito` c'è, e lo stato, tolti l'esito e la manopola, è uguale a quello della stessa chiusura con `soglia_shock` alzata perché lo shock non esca, come fa `tests/shock.rs:109`); `il_governo_avversario_si_dimette_col_motivo` (i tre motivi di `tests/governo_ia.rs:724-744` e `tests/legislatura.rs:208` stanno nella causa di `GovernoDimissionario`). `situazione_accesa_tace_se_nasce_il_suo_evento` (una recessione che si accende e il suo evento che nasce nella stessa chiusura: una voce sola, `EventoNato`; con l'evento che non nasce, resta `SituazioneAccesa`).
- [ ] **Step 2:** `cargo test -p sim-core rischio:: --test shock` → falliscono.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat(core): voci degli eventi nuovi e dell'apertura`.

### Task 8: fermate di «avanti fino a novità»

**Files:**
- Create: `crates/sim-core/tests/fermate.rs`
- Modify: `crates/sim-core/src/vista.rs` (`Fermata`, `Vista.fermate`), `crates/sim-core/src/partita/esito.rs` (`fermate`), `crates/sim-core/src/partita.rs` (`vista()`), `crates/sim-core/src/dati.rs`, `dati/base/manopole.toml`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sistemi/tempo-e-ritmo.md:59` (niente tempi morti senza decisioni); `docs/ricerca/sintesi/stato-per-le-esposizioni.md:60` (il campione di un incrocio è proporzionale agli elettori veri); `.scratch/v1-giocabile/issues/12-opposizione.md:27`; spec, sezione «Fermate».

**Architect (11/11):** riferimento da correggere. `fatti-della-chiusura.md:387-407` è la chiusura dell'inventario e non parla di fermate (la sezione finisce a `:406`). Governano questo task `docs/ricerca/sistemi/tempo-e-ritmo.md:59` (evitare i tempi morti senza decisioni) e `docs/ricerca/sintesi/stato-per-le-esposizioni.md:60` (il campione di un incrocio è proporzionale agli elettori veri del sottoinsieme: è ciò che fa di `campione × quota` una stima degli elettori del giocatore). Premesse verificate: `Misura.valori` è `Vec<(usize, usize)>` (`parlamento/mod.rs:64`); `Incrocio` ha `campione` e `quote` per partito (`informazione/sondaggio.rs:27-33`); `gruppi` e `perimetro` si leggono dal dato della disposizione come fa `toccati` (`elettorato/referendum.rs:34-49`), che però rende un vettore per gruppo e qui non si chiama. Va dopo il Task 6: `Ultimatum` e `Crisi` leggono voci che nascono lì, e `ultimatum_posto` deve restare non fondibile, o il `partito` si perde nella fusione. `Governo` resta su `governa()` (`partita.rs:2724`): in trattativa l'uscente non ha la fermata `Governo` ma può avere `Incarico`.

**Interfaces:**
- Consumes: `Partita.esito`; `Sondaggio.condizioni`, `Sondaggio.regioni` (`crates/sim-core/src/informazione/sondaggio.rs`); `toccati` (`crates/sim-core/src/elettorato/referendum.rs:32-49`).
- Produces:

```rust
// vista.rs — serializzata come le altre enum con dati della Vista (`eventi::Causa`).
pub enum Fermata {
    Governo, Fine, Voto { misura: u32 }, Ultimatum { partito: String },
    Crisi, Referendum, Campagna, Incarico, PostoRiservato,
}
pub struct Vista { /* … */ pub fermate: Vec<Fermata> }
// partita/esito.rs
impl Partita {
    fn fermate(&self) -> Vec<Fermata>;
    /// Quota degli elettori stimati del giocatore toccati dalla misura, 0-1, dal sondaggio pubblico.
    fn quota_toccata(&self, m: &parlamento::Misura) -> f64;
}
```

Manopola nuova `fermata_quota_elettorato` = 0,30 (min 0, max 1, ticket `12`, `# decisione di Mario dell'11/11/2026`).

Quando scatta ciascuna, in quest'ordine nella lista: `Fine` se la partita è finita (e nient'altro); `Governo` se `governa` (e nient'altro); poi `Campagna` se `turni_al_voto == 0`; `Incarico` se in trattativa `incarico` è del giocatore o è la guida; `PostoRiservato` se `posto_riservato`; `Crisi` e `Ultimatum { partito }` se l'esito ha `CrisiAperta` o `UltimatumPosto` (Scelta 6); `Referendum` secondo la spec riga 310; `Voto { misura }` per ogni misura del calendario col voto in questo turno e `quota_toccata ≥ fermata_quota_elettorato`. La formula di `quota_toccata` è nella spec, righe 317-324.

- [ ] **Step 1: test che falliscono**, in `tests/fermate.rs`:
  - `al_governo_solo_governo`, `a_partita_finita_solo_fine`.
  - `turno_quieto_lista_vuota`: `all_opposizione()` senza misure al voto → `fermate.is_empty()`.
  - `il_trenta_per_cento_ferma`: con `edita` sul sondaggio, quota toccata 0,30 → `Voto { misura }`; a 0,29 no.
  - `l_ultimatum_ferma_una_volta`: nel turno in cui è posto c'è `Ultimatum { partito }`, nel successivo no.
  - `nel_turno_del_voto_campagna`, `incarico_al_giocatore_ferma`, `posto_riservato_ferma`, `luglio_settembre_referendum`.
  - `senza_elettori_nessuna_fermata_sul_voto`: quota del giocatore zero in ogni incrocio, o campione zero → nessun `Voto`, nessun panic, `quota_toccata == 0.0`.
- [ ] **Step 2:** `cargo test -p sim-core --test fermate` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): fermate di «avanti fino a novità»`.

### Task 9: registro per turno

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (accanto a `carica` `:4487`), `crates/sim-core/tests/salvataggi.rs`, `app/src-tauri/src/archivio.rs` (accanto a `riavvolgi` `:359`), `app/src-tauri/src/lib.rs` (comando e `generate_handler!` `:412`)

**Ricerca:** `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:78-85` (uno snapshot per apertura, peso); spec righe 290-297.

**Architect (11/11):** `rust-engineer` confermato: solo file `.rs`, e il guscio è nel workspace, quindi `cargo test -p app registro` vale quanto il `--manifest-path` dello Step 2 e rispetta il vincolo globale. Righe da correggere: col merge di `a02f6ad` (#49) nel branch (`c840da3`) quelle di `lib.rs` sono scese. `torna_a` è a `:148-164` (l'errore della partita di prova a `:151-153`), `generate_handler!` a `:412`, `chiudi` scrive lo snapshot a `:118-131`. Tre casi limite, presi dai difetti di questo repo. (1) **Disco un turno avanti.** Dopo una chiusura guasta a metà lo snapshot del turno N+1 può esistere mentre la partita in memoria è ancora al turno N (`lib.rs:114-117`, test `chiudi_guasta_a_meta_si_ripete` a `:581`). `registro(N)` leggerebbe l'esito di una chiusura che per il giocatore non è avvenuta. La spec riveduta lo chiede già (sezione «Registro»: errore per un turno che la partita in memoria non ha ancora chiuso), il piano no. `Archivio::registro(id, turno)` non vede la partita in memoria: il rifiuto sta nel comando di `lib.rs`, prima di toccare il disco, e il caso «turno in corso» di `registro_fuori_dai_turni_e_un_errore` si prova lì, sul disco lasciato un turno avanti, non in `archivio.rs`. (2) **Il messaggio.** Uno snapshot che manca esce da `apri` come errore di lettura col percorso (`archivio.rs:135`): il testo «nessun registro per il turno N» chiede di guardare prima `turni(cartella)` (`:257`). (3) **Schema e validazione.** La versione si legge per prima, come in `carica` (`partita.rs:4488-4502`): un salvataggio dello schema 15 non ha il campo `esito`, e letto in un colpo solo darebbe `SalvataggioIlleggibile` invece di `SchemaDiverso`. `esito_da_salvataggio` non passa da `controlla_coerenza`: i controlli dell'esito (causa non vuota, al massimo sette in evidenza) stanno in una funzione che usano sia lei sia `carica`.

**Interfaces:**
- Consumes: `Partita.esito` salvato (Task 2); `nome_turno`, `leggi_da`, `apri` (`app/src-tauri/src/archivio.rs:25`, `:124`, `:135`).
- Produces:

```rust
// sim-core, partita.rs
impl Partita {
    /// Legge dal testo di un salvataggio solo la versione dello schema e l'esito.
    pub fn esito_da_salvataggio(json: &str) -> Result<Vec<VoceEsito>, Errore>;
}
// src-tauri, archivio.rs
impl Archivio { pub(crate) fn registro(&self, id: &str, turno: u32) -> Result<Vec<VoceEsito>, String>; }
// src-tauri, lib.rs
#[tauri::command] async fn registro(turno: u32, /* stato, archivio */) -> Result<Vec<VoceEsito>, String>;
```

`esito_da_salvataggio` usa lo stesso controllo di versione di `carica` (estratto in una funzione comune) e gli stessi errori. `Archivio::registro` legge lo snapshot del turno `turno + 1` (`checked_add`), guardando prima `turni(cartella)`; se non c'è: «nessun registro per il turno N: il turno non è ancora chiuso». Il comando in `lib.rs` rifiuta, prima di toccare il disco, un turno che la partita in memoria non ha ancora chiuso: dopo un guasto a metà il disco può stare un turno avanti (`app/src-tauri/src/lib.rs:114-117`). La partita di prova non ha salvataggi: errore, come `torna_a` (`app/src-tauri/src/lib.rs:148-153`). I controlli dell'esito (causa non vuota, al massimo sette) stanno in una funzione che usano `carica` ed `esito_da_salvataggio`.

- [ ] **Step 1: test che falliscono.**
  - In `tests/salvataggi.rs`: `esito_da_salvataggio_rende_l_esito` (dopo una chiusura, sul testo di `salva()` rende `p.vista().esito`); `esito_da_salvataggio_rifiuta_un_altro_schema` (`SchemaDiverso`); `esito_da_salvataggio_rifiuta_il_json_rotto` (`SalvataggioIlleggibile`).
  - In `archivio.rs`: `registro_del_turno_chiuso` (tre chiusure: `registro(id, 1)` è l'esito che la Vista mostrava all'apertura del turno 2); `registro_fuori_dai_turni_e_un_errore` (`u32::MAX`, e il turno 2 dopo `riavvolgi(id, 2)`: errore col numero del turno, mai panic).
  - In `lib.rs`: `registro_del_turno_in_corso_rifiutato` (sul disco lasciato un turno avanti come in `chiudi_guasta_a_meta_si_ripete`, `registro` del turno in corso è un errore); `registro_della_partita_di_prova_rifiutato`.
- [ ] **Step 2:** `cargo test -p sim-core --test salvataggi` e `cargo test --manifest-path app/src-tauri/Cargo.toml registro` → falliscono.
- [ ] **Step 3:** implementa funzione, metodo e comando.
- [ ] **Step 4:** verifiche globali verdi.
- [ ] **Step 5:** commit `feat: registro del turno riletto dal salvataggio successivo`.

### Task 10: chiusura della PR 1

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (commento d'ordine di `chiudi_turno` `:520-545`), `crates/sim-core/tests/esito.rs`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:28` (il commento d'ordine è indietro sul codice in due punti).

- [ ] **Step 1:** test `ogni_tipo_e_coperto` in `tests/esito.rs`: un array con i 92 `TipoVoce` e, accanto a ciascuno, il nome del test che lo fa nascere; il test fallisce se un tipo ha il nome vuoto. È un elenco tenuto a mano: serve alla review, non prova da solo la copertura.
- [ ] **Step 2:** riscrivi il commento d'ordine di `chiudi_turno` sull'ordine vero dei passi, coi punti in cui nascono le voci.
- [ ] **Step 3:** verifiche globali verdi, più `npm --prefix app run check`.
- [ ] **Step 4:** apri la PR `feat/esito-del-turno`: nella descrizione l'elenco dei testi di `fatto`, `causa` e dei riassunti per tipo, e le decisioni prese in esecuzione.
- [ ] **Step 5:** commit `docs(core): ordine della chiusura e copertura dei tipi di voce`.

---

# PR 2: esposizioni

### Task 11: regioni nella Vista

**Files:**
- Create: `crates/sim-core/tests/vista_completa.rs`
- Modify: `crates/sim-core/src/partita.rs` (`VERSIONE_SCHEMA` → 17; `vista()`; il punto del motore in cui la disoccupazione regionale si aggiorna), `crates/sim-core/src/economia/mod.rs` (stato della regione), `crates/sim-core/src/vista.rs`, `crates/sim-core/src/elettorato/mod.rs` (`Fascia` esportata), `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:23-44` (disoccupazione e reddito per regione), `:56-65` (gruppi), `:147-171` (tabella dello stato).

**Architect (11/11):** `totale` non esce da `indice_pericolosita(dati, tipo, r)` com'è firmata nel Task 3: serve l'atteso non normalizzato, e la riga Architect del Task 3 chiede di farlo nascere lì. La copia della disoccupazione si fa dentro il passo dell'economia, prima di `okun` (`economia/mod.rs:329`, `economia/lavoro.rs:45`), così la fanno anche le copie di `avanza_copia`. La Vista oggi nomina le regioni in tre modi (indice in `VoceGrandezza.valori`, id nel sondaggio, nome in `VoceEvento.regione`: `stato-per-le-esposizioni.md:20`): `VoceRegione.id` è la chiave, e l'ordine è quello dei dati, lo stesso di `VoceGrandezza.valori`. Riferimento in più, per le formule dietro i tre indici: `docs/ricerca/dati/pericolosita-naturale-regioni.md:121-124`.

**Interfaces:**
- Consumes: `eventi::emergenze::indice_pericolosita` (Task 3).
- Produces:

```rust
pub struct VoceRegione {
    pub id: String, pub nome: String, pub macroarea: Macroarea,
    pub pericolosita: IndiciPericolosita,
    pub disoccupazione: f64,   // %, esatta, della chiusura precedente
    pub reddito: f64,          // euro annui reali per elettore, sui redditi della chiusura precedente
    pub gruppi: Vec<f64>,      // 18 quote che sommano a 1, condizione × fascia
}
pub struct IndiciPericolosita { pub sisma: f64, pub idro: f64, pub incendi: f64, pub totale: f64 }
pub struct Vista { /* … */ pub regioni: Vec<VoceRegione> }
```

Stato nuovo: la disoccupazione regionale della chiusura precedente, copiata prima che il motore la aggiorni, come `grandezze.prima`. Al turno zero è uguale alla corrente. Le definizioni di `reddito` e degli indici stanno nella spec, righe 351-365; `totale` è l'indice sulla somma dei tre tipi.

- [ ] **Step 1: test che falliscono**, in `tests/vista_completa.rs`:
  - `venti_regioni_nell_ordine_dei_dati`.
  - `la_disoccupazione_regionale_e_in_ritardo`: dopo due chiusure, la media delle `disoccupazione` pesata sulle forze di lavoro è uguale (entro 1e-9) alla disoccupazione riveduta della Vista, e diversa da quella corrente dello stato.
  - `le_quote_dei_gruppi_sommano_a_uno`: 18 quote per regione, somma 1 entro 1e-9.
  - `gli_indici_di_pericolosita`: tutti in 0-1; per `sisma`, `idro`, `incendi` e `totale` una regione vale 1.
  - `schema_16_rifiutato`.
- [ ] **Step 2:** `cargo test -p sim-core --test vista_completa` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core)!: regioni nella Vista (schema 17)`.

### Task 12: margine dichiarato del sondaggio

**Files:**
- Modify: `crates/sim-core/src/informazione/sondaggio.rs` (`Incrocio`, `Rilevazione`, `campiona`), `crates/sim-core/tests/sondaggi.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:127-146` (voce 9a: campione e `effetto_disegno` ci sono già).

**Architect (11/11):** la voce 9a è la riga `stato-per-le-esposizioni.md:129`. Il perché del margine «dichiarato» sta in `docs/ricerca/modelli/sondaggi-ed-errore.md:23` e `:38`: l'errore vero è circa il doppio del dichiarato, e il commento del campo lo dice, perché la UI non lo presenti come forbice dell'errore. **Files** incompleto: c'è un letterale `Incrocio { … }` nei test di `elettorato/referendum.rs:225`; in produzione i costruttori sono solo `sondaggio.rs:165` e `:206`. Regge in parallelo col 13.

**Interfaces:**
- Produces: `Incrocio.margine: f64` e `Rilevazione.margine: f64` = `2·√(effetto_disegno·0,25/campione)`; 0 se il campione è 0.

- [ ] **Step 1:** test `il_margine_e_quello_dichiarato` (campione 1000, `effetto_disegno` 1 → 0,0316 entro 1e-4; il margine di una regione piccola è più largo di quello nazionale) e `campione_zero_margine_zero`.
- [ ] **Step 2:** `cargo test -p sim-core --test sondaggi` → falliscono.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): margine dichiarato del sondaggio`.

### Task 13: voto per regione

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (`vota_parlamento` `:2802-2893`, dove voti e seggi per regione si calcolano e si buttano `:2810-2832`), il tipo `EsitoElezioni`, `crates/sim-core/tests/elezioni.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:45-55` (voti per regione ceduti al riparto, `senato_per_regione` scartato, Camera nazionale).

**Architect (11/11):** **Files** incompleto: `EsitoElezioni` è in `legislatura/voto.rs:27-33` e ha due letterali nei test unitari da aggiornare (`partita.rs:6216`, `partita/ia.rs:1670`). In `vota_parlamento` la mappa dei voti è ceduta a `riparto_con` a `:2816` e il riparto è consumato a `:2829`: le regioni si prendono prima. Col riparto rifiutato (`:2848-2851`) i seggi per regione non esistono: vettori di zeri lunghi quanto i partiti, non regioni mancanti. `voti_per_regione` rende una `BTreeMap`, quindi in ordine d'id: `regioni` si rimette nell'ordine dei dati, come `Vista.regioni` del Task 11.

**Interfaces:**
- Produces:

```rust
pub struct VotoRegione {
    pub regione: String,
    pub voti: Vec<f64>,          // stesso ordine e stessa unità di `EsitoElezioni.voti`
    pub seggi_senato: Vec<u32>,  // prima del premio, dell'estero e delle cessioni in coalizione
    pub primo: String,           // id del primo partito nella regione
}
// EsitoElezioni prende `pub regioni: Vec<VotoRegione>`.
```

L'albo non cambia: `ElezioneAlbo` copia in un tipo suo (`crates/sim-core/src/salvataggi.rs:73-80`).

- [ ] **Step 1:** test `i_voti_per_regione_sommano_al_nazionale` (per ogni partito, entro 1e-9), `i_seggi_del_senato_per_regione` (sommano ai seggi proporzionali più speciali del riparto), `il_primo_partito_ha_piu_voti`, `l_albo_non_porta_le_regioni`.
- [ ] **Step 2:** `cargo test -p sim-core --test elezioni` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): voto e seggi del Senato per regione`.

### Task 14: opere finite

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (campo `opere`; passo degli atti `:570-571`; `vota_parlamento`; `vista()`), `crates/sim-core/src/vista.rs`, `crates/sim-core/tests/agenda.rs`, `crates/sim-core/tests/ribaltone.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:66-77` (cantieri tolti in `esegui`; `nuovo_governo` ricostruisce `StatoDicastero` da zero, `crates/sim-core/src/ministri/mod.rs:207-214`).

**Interfaces:**
- Consumes: `FattoAtto` (Task 3).
- Produces:

```rust
pub struct VoceOpera { pub dicastero: String, pub atto: String, pub regione: String, pub turno: u32, pub cantiere: bool, pub euro: i64 }
pub struct Vista { /* … */ pub opere: Vec<VoceOpera> }
```

Lo stato `Partita.opere` sta sulla partita, non in `StatoDicastero`. Si riempie dai `FattoAtto` di tipo `AttoEseguito` e `CantiereFinito` con una regione; si svuota in `vota_parlamento`.

- [ ] **Step 1:** test `il_cantiere_finito_resta_fino_al_voto` (c'è dopo la fine dei lavori, c'è ancora dopo un ribaltone, non c'è più dopo il voto), `un_atto_senza_regione_non_e_un_opera`.
- [ ] **Step 2:** `cargo test -p sim-core --test agenda --test ribaltone` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): opere finite fino al voto successivo`.

### Task 15: regole UE, debito, minaccia di scissione

**Files:**
- Modify: `crates/sim-core/src/economia/regole_ue.rs` (`proiezione`; `controllo_annuale` `:143-166` da cui estrarre la formula), `crates/sim-core/src/scenario.rs` (`StatoPde` esportato), `crates/sim-core/src/vista.rs`, `crates/sim-core/src/partita.rs` (`vista()`), `crates/sim-core/tests/conti.rs`, `crates/sim-core/tests/debito.rs`, `crates/sim-core/tests/vista_completa.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:78-92` (PDE e conto), `:93-101` (minaccia), `:102-118` (MES e ristrutturazione), `:119-126` (clausola dell'energia: nazionale).

**Architect (11/11):** premesse verificate: `controllo_annuale` è a `economia/regole_ue.rs:143-191` e la formula a `:155-166`; il 2026 entra intero al turno zero (`:94-107`) e la chiusura del turno zero non accumula (`:117-125`); `ultima_deviazione` è già nello stato (`:19`). La funzione pura rende un `Option`: il controllo esce senza toccare nulla se al conto manca un anno (`:147-154`), ed è lo stesso `None` della proiezione. Dentro ci va anche la clausola (`:155-161`), o proiezione e controllo divergono proprio negli anni coperti.

**Interfaces:**
- Produces:

```rust
pub struct VistaRegoleUe {
    pub procedura: StatoPde, pub turni_in_stato: u32,
    pub conto_controllo: f64, pub ultima_deviazione: f64, pub coesione_sospesa: bool,
    pub margine_energia: Option<u16>,        // l'anno coperto, se chiesto e se conta
    pub proiezione: Option<ProiezioneUe>,
}
pub struct ProiezioneUe { pub anno: u16, pub deviazione: f64, pub conto: f64 }
pub struct VistaDebito { pub mes: Option<VistaMes>, pub ristrutturazione: Option<VistaRistrutturazione> }
pub struct VistaMes { pub fine: Trimestre, pub vincoli: Vec<VoceVincolo> }
pub struct VoceVincolo { pub disposizione: String, pub gradino: String }
pub struct VistaRistrutturazione { pub taglio: Trimestre, pub premio_pb: i32, pub fine_perdita: Option<Trimestre> }
pub struct Vista { /* … */ pub regole_ue: VistaRegoleUe, pub debito: VistaDebito }
// VocePartito prende `pub minaccia: bool`.
```

La deviazione di `controllo_annuale` diventa una funzione pura che controllo e proiezione usano entrambi. La regola della proiezione è nella spec, riga 398: anno già intero nel conto (anno chiuso, oppure il 2026) → numeri esatti; altrimenti annualizzato sui trimestri chiusi; anno mancante al conto → `None`.

- [ ] **Step 1: test che falliscono:**
  - `la_proiezione_di_gennaio_coincide_col_controllo`: in gennaio-marzo, `proiezione.deviazione` è la `ultima_deviazione` dopo la chiusura di aprile-giugno, se nel mezzo non cambia nulla (entro 1e-9).
  - `al_turno_zero_la_proiezione_non_annualizza`: l'anno è 2026 e la deviazione è quella dei numeri del turno zero, senza il fattore 4/3.
  - `in_luglio_settembre_annualizza_su_due_trimestri`.
  - `il_mes_dice_fine_e_vincoli`, `la_ristrutturazione_dice_premio_e_fine_della_perdita`, `senza_mes_ne_ristrutturazione_tutto_vuoto`.
  - `la_minaccia_sta_nel_partito`.
  - `vista_completa_al_turno_zero`: su `al_governo()` e `all_opposizione()` appena nate la Vista si costruisce, `elezioni` senza regioni dove non si è votato, `debito.mes` e `debito.ristrutturazione` vuoti; togliendo con `edita` l'anno dal conto della spesa netta, `proiezione == None`.
- [ ] **Step 2:** `cargo test -p sim-core --test conti --test vista_completa` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): regole UE, debito e minaccia di scissione nella Vista`.

### Task 16: posti in calendario, menu elettorale, anteprima della proposta, costo di Ferma e Dirotta

**Files:**
- Modify: `crates/sim-core/src/parlamento/mod.rs` (conteggio dei posti di `presenta` `:585-602`, costanti `:33-40`), `crates/sim-core/src/parlamento/regole.rs` (`legge_non_valida` `:139-150`), `crates/sim-core/src/ministri/agenda.rs` (`ferma` `:383-389`, `dirotta` `:447-458`, `ferma_cantiere` `:492-497`), `crates/sim-core/src/vista.rs`, `crates/sim-core/src/partita.rs` (`anteprima` `:1929-1946`, `presenta_proposta` `:3868`), `crates/sim-core/tests/calendario.rs`, `crates/sim-core/tests/agenda.rs`, `crates/sim-core/tests/norme.rs`, `crates/sim-core/tests/opposizione.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/stato-per-le-esposizioni.md:127-146` (voci 9b, 9c, 9d).

**Architect (11/11):** `costo_dirotta(dicastero, atto, regione)` con tre `&str` non può avere «gli stessi rifiuti del comando». `Dirotta` porta `atto: Option<String>` e `regione: Option<String>` (`comando.rs:28-32`), `dirotta` li prende come `Option<&str>` (`ministri/agenda.rs:394-400`) e rifiuta una regione nei dicasteri senza regione bersaglio (`:419-421`): con la regione obbligatoria il costo non si può chiedere per otto dicasteri su dieci. La firma prende due `Option<&str>`. Le righe citate in **Files** sono le formule; le funzioni cominciano prima: `ferma` a `agenda.rs:374`, `dirotta` a `:394`, `ferma_cantiere` a `:479`, `presenta` a `parlamento/mod.rs:577`. Le voci 9b, 9c e 9d stanno a `stato-per-le-esposizioni.md:131-145`.

**Interfaces:**
- Produces:

```rust
pub struct VistaPosti { pub ordinari: u32, pub ordinari_occupati: u32, pub costituzionali: u32, pub costituzionali_occupati: u32 }
pub struct VoceMenu { pub disposizione: String, pub gradino: String, pub ammesso: bool, pub motivo: Option<String> }
pub struct Vista { /* … */ pub posti: VistaPosti }
// VoceAgenda e VoceCantiere prendono `pub costo_ferma: f64`.
impl Partita {
    pub fn menu_elettorale(&self, valori: &[ValoreProposto]) -> Vec<VoceMenu>;
    pub fn costo_dirotta(&self, dicastero: &str, atto: Option<&str>, regione: Option<&str>) -> Result<f64, Errore>;
    pub fn anteprima_proposta(&self, valori: &[ValoreProposto]) -> Result<Anteprima, Errore>;
}
```

Il conteggio dei posti e le tre formule di costo si estraggono in funzioni che il comando e la Vista usano entrambi: una regola sola, non una copia. I costi sono il `delta` prima del limite a 0-100. `costo_dirotta` ha gli stessi rifiuti del comando `Dirotta`. `anteprima_proposta` è `presenta_proposta` senza il controllo del posto riservato: `all_opposizione`, poi `Parlamento::bozza_proposta` (`parlamento/mod.rs:649-666`) e `parlamento::anteprima` con `CAPACITA_NEUTRA`, come fa `anteprima`; nessuna regola copiata. I comandi passacarte del guscio restano al ticket 12 della mappa.

- [ ] **Step 1: test che falliscono:**
  - `i_posti_contati_come_li_conta_presenta`: con i posti ordinari tutti occupati, `ordinari_occupati == ordinari` e `presenta` rifiuta; con uno libero accetta.
  - `il_costo_di_ferma_e_quello_vero`: `costo_ferma` è la variazione di soddisfazione che `Ferma` applica (soddisfazione lontana da 0 e da 100).
  - `il_costo_di_dirotta_e_quello_vero`, `costo_dirotta_rifiuta_come_il_comando`.
  - `il_menu_spegne_il_premio_senza_coalizioni`: con le coalizioni spente il gradino del premio alla coalizione ha `ammesso == false` e un motivo; `ammesso` coincide con `legge_non_valida` per ogni gradino.
  - `l_anteprima_della_proposta_e_quella_in_calendario`: all'opposizione, col posto riservato libero, linee e forbice di `anteprima_proposta` coincidono con quelle della misura che `PresentaProposta` mette in calendario nello stesso turno; la partita prima e dopo l'anteprima è la stessa.
  - `anteprima_proposta_rifiuta_come_il_comando`: rifiuta al governo e con una norma costituzionale; col posto riservato di un altro partito non rifiuta.
- [ ] **Step 2:** `cargo test -p sim-core --test calendario --test agenda --test norme --test opposizione` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi; tipi rigenerati.
- [ ] **Step 5:** commit `feat(core): posti, menu elettorale, anteprima della proposta e costo di Ferma e Dirotta`.

### Task 17: redditi per fascia e chiusura della PR 2

**Files:**
- Modify: `crates/sim-core/src/elettorato/mod.rs` (stato `redditi_fasce`), `crates/sim-core/src/vista.rs`, `crates/sim-core/src/partita.rs`, `crates/sim-core/tests/elettorato.rs`, `app/src/lib/tipi/`

**Ricerca:** `docs/ricerca/sintesi/fatti-della-chiusura.md:362-385` (storia 116; `:377`: l'attesa non si rifà dallo stato a finestra piena).

**Architect (11/11):** `VoceReddito` ha gli stessi quattro campi di `RedditoFascia`: basta un tipo solo, esportato, in `Vista.redditi` e nello stato.

**Interfaces:**
- Consumes: `RedditoFascia` (Task 5).
- Produces: `Vista.redditi: Vec<RedditoFascia>`, tre voci: lo stesso tipo del Task 5, esportato, senza un doppione per la Vista. Stato nuovo: l'ultimo `Vec<RedditoFascia>` reso da `giudizio`, conservato nell'elettorato; al turno zero crescita e attesa a zero.

- [ ] **Step 1:** test `i_redditi_della_vista_sono_quelli_della_voce` (in un turno con `RedditoDeluso`, crescita e attesa della fascia nella Vista sono i numeri della voce) e `tre_fasce_al_turno_zero`.
- [ ] **Step 2:** `cargo test -p sim-core --test elettorato` → non compila.
- [ ] **Step 3:** implementa.
- [ ] **Step 4:** verifiche globali verdi, tipi rigenerati, `npm --prefix app run check`.
- [ ] **Step 5:** commit `feat(core): redditi per fascia nella Vista`; apri la PR `feat/vista-completa` con le decisioni prese in esecuzione.

---

## Rimandato

- **Piano 11 (UI):** prima pagina del Pungolo, registro con «torna al turno N», mappa a cinque livelli, «Conti», «Avanti fino a novità». I posti dei dati sono quelli del ticket 07 della mappa.
- **Ticket 09 della mappa:** titoli spiritosi e varianti per ciascuno dei 92 tipi.
- **Ticket 12 della mappa:** comandi passacarte del guscio per `elettori_campione`, `menu_elettorale`, `costo_dirotta`, `anteprima_proposta`.
- **Piano strutturale riveduto** (storia 51): piano di core suo.
- **Aspettativa «alzata dalle promesse»:** debito aperto fra glossario e motore (spec riga 30).
