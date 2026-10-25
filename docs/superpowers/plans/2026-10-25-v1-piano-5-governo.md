# V1 · Piano 5: governo — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** I dieci ministri dentro `sim-core`: grandezze dei dicasteri tenute in piedi dalla spesa di funzionamento, catalogo dei 40 atti, agenda annunciata in apertura con fermare e dirottare, atti puntuali e cantieri con ritardo e capacità, regione bersaglio col merito al partito del dicastero, soddisfazione con le sue cause, minacce e dimissioni, successione con interim e rodaggio, proposte di fondi; poi la legge di bilancio con aliquote e scaglioni, budget in funzionamento e margine, fondo di riserva, indicizzazione e perdita reale, esercizio provvisorio.

**Architecture:** Tre moduli nuovi in `sim-core`: `grandezze.rs` (stato delle grandezze dei dicasteri ed effetti attivi degli atti, nono file di dati `dati/base/grandezze.toml`), `atti.rs` (catalogo, decimo file `dati/base/atti.toml`), `ministri/` (ministri, agenda, soddisfazione, proposte). I comandi del giocatore entrano da un `enum Comando` e da `Partita::comanda`, e mutano lo stato della partita, così un salvataggio a metà turno li conserva. Gli atti arrivano al motore economico attraverso una `Misure` costruita da `Partita` prima di `Economia::passo`, nel posto «misure» in testa all'ordine dei blocchi. La legge di bilancio vive in `economia/legge_bilancio.rs` accanto a `Bilancio`.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Python 3 con la sola libreria standard per il controllo del catalogo; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 24-38 (righe 55-69) e 39-43, 45 (righe 73-79), «Le quattro cuciture» (riga 261), «Modulo dati» (riga 268, schema dell'atto), «Motore economico» (righe 289, 295, 299-302), «Elettorato» (riga 312, colpa nella regione bersaglio), «Governo, ministri e coalizione» (righe 327-330), «Informazione» (riga 368), «Turno e partita» (righe 385-386), «Testing Decisions» (riga 428). Scope dalla tabella dei piani: [`2026-10-21-v1-piano-1-fondamenta.md:47`](2026-10-21-v1-piano-1-fondamenta.md). Ticket: `.scratch/v1-giocabile/issues/03-dicasteri-della-v1.md`, `04-autonomia-dei-ministri.md`, `16-catalogo-atti-e-riforme.md` con l'asset `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md`, `27-mappa-delle-20-regioni.md:31-36`, `29-costo-del-governare.md`. Debiti lasciati dai piani precedenti: `2026-10-22-v1-piano-3-motore-economico.md:24`, `2026-10-22-v1-piano-4-elettorato-e-sondaggi.md:26-27`, `:389`.

## Due PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · ministri e atti | `feat/v1-piano-5-ministri` | `main` | 1-7 | grandezze dei dicasteri, catalogo dei 40 atti, ministri e successione, agenda e cantieri, effetti sul motore e merito regionale, proposte |
| B · legge di bilancio | `feat/v1-piano-5-bilancio` | `feat/v1-piano-5-ministri` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 8-9 | aliquote e scaglioni nel gettito e nei redditi, legge di bilancio con bozza, indicizzazione, riepilogo, approvazione ed esercizio provvisorio |

La PR A gira col bilancio del piano 3 (2026 al turno zero, 2027 già approvato, poi cifre nominali ferme). La PR B mette in mano al giocatore il bilancio degli anni dopo.

## Confini con gli altri piani (scelte di questo piano)

- **Giocatore.** Finché il piano 7 non porta fondazione e voto, il giocatore è il Presidente del Consiglio del governo uscente dello scenario: i comandi agiscono su quel governo.
- **Aula.** Il bilancio presentato si approva in chiusura senza voto, nel posto «aula» dell'ordine; il piano 6 vi mette il voto, e un voto perso diventa un bilancio non approvato, che qui esiste già (esercizio provvisorio). Il secondo fallimento che porta alle dimissioni (storia 45) arriva col piano 7: qui, chiuso l'esercizio provvisorio senza bilancio, l'anno prosegue con le cifre di quello prima.
- **Decreti.** Accogliere una proposta di fondi applica subito la variazione, che è l'effetto del decreto (spec riga 342: vale subito); il piano 6 aggiunge la conversione in chiusura. Degli inneschi delle proposte qui c'è solo il primo, i fondi: l'evento arriva col piano 9, la disposizione lontana col 6. Le norme proprie del Presidente del Consiglio, col ministro proponente che firma, sono del piano 6.
- **Coalizione.** Intesa, emendamenti degli alleati al bilancio (storia 44) e il colpo all'intesa per le dimissioni chieste sono del piano 7. Lo stesso vale per i tecnici, che entrano con la spartizione: qui nessun tecnico esiste, quindi nessuna rosa di tecnici e nessun «passa a un partito».
- **Eventi.** Il fondo di riserva qui è uno stanziamento della legge di bilancio con il suo peso sul tetto UE; lo spendono le emergenze del piano 9, che porta anche il residuo e il decreto a debito quando finisce.
- **Anteprima.** La vista mostra degli atti costo esatto, forma, grandezze toccate con la direzione e ritardo scritto. Gli effetti a fasce con l'errore che cala con la capacità, insieme a linee e forbice, arrivano con l'anteprima delle misure del piano 6.
- **Vocazione.** La scelta della regione bersaglio pesa i voti del partito con una manopola unica; il peso per vocazione arriva col piano 8.
- **Disposizioni.** Il prerequisito facoltativo di un atto (una disposizione a un valore) arriva col piano 6, che porta le disposizioni. L'unico atto con prerequisito della Sanità, le convenzioni col privato accreditato, lo soddisfa al turno zero.
- **Esito e registro.** La tabella dei piani non li assegna. Qui ogni ministro porta nella vista le variazioni di soddisfazione con la loro causa degli ultimi due turni, l'unica parte dell'esito che questo piano produce; la scelta delle 7 voci in evidenza resta aperta.
- **Strategie del CLI.** Del piano 12; il CLI esistente gioca senza comandi e non cambia.

## Scelte di prodotto prese dal piano, da confermare in review

1. **Ministri generati dal seme.** La spec (riga 484) mette «rose di ministri e tecnici» fra i contenuti da scrivere a mano. Il piano le genera col flusso `Flusso::Governo`: nome dalle liste `nomi` e `cognomi` di `elettorato.toml`, capacità ed escursione dalla posizione del partito estratte. Motivo: la destra tiene 8 dicasteri al turno zero e ogni partito può arrivare a tenerli tutti; rose scritte a mano vorrebbero più di 13 personaggi per partito.
2. **Venti grandezze, solo quelle con chi le legge.** Restano i 10 indicatori di servizio, l'occupazione pubblica, gli stock regionali di infrastrutture e investimenti privati, la domanda regionale, il prezzo dell'energia, le politiche attive, il capitale umano, e tre grandezze che leggeranno le situazioni e gli scandali del piano 9 (sovraffollamento delle carceri, immigrazione irregolare, corruzione). Escono speranza di vita, emissioni, integrazione, credito con l'UE, export, povertà, spesa pensionistica, abbandono scolastico e spesa per la difesa sul PIL. Della Sanità scritta per intero cadono quindi l'effetto sulla speranza di vita delle case di comunità e il −2% di spesa di funzionamento dell'accorpamento delle ASL.
3. **Indici a 100 al turno zero.** Gli indicatori di servizio e gli stock sono indici, non unità vere: la ricerca non ha i livelli di partenza (liste d'attesa in giorni, durata dei processi in mesi), e il giudizio legge solo variazioni.
4. **Credibilità del ministro dell'Economia fuori dal premio di sfiducia.** Entra nello spread come termine additivo per punto di capacità sotto o sopra 3, che il TPI non azzera; in interim vale capacità 1.

## Global Constraints

Valgono tutti i vincoli del piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`), del piano 3 (`2026-10-22-v1-piano-3-motore-economico.md:26-31`) e del piano 4 (`2026-10-22-v1-piano-4-elettorato-e-sondaggi.md:33-44`): italiano coi termini di `CONTEXT.md` (dicastero, ministro, capacità, posizione, obiettivo, soddisfazione, agenda, interim, rodaggio, atto del ministro, cantiere, regione bersaglio, spesa di funzionamento, margine discrezionale, indicatore di servizio, legge di bilancio, fondo di riserva, esercizio provvisorio; mai «ministero» per l'area, «settore» da solo, «lealtà», «manovra», «progetto»); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Ogni estrazione di questo piano esce da `Flusso::Governo = 7`, in coda all'enum di `caso.rs` (oggi l'ultimo è `ElettoriCampione = 6`, `crates/sim-core/src/caso.rs:9-17`). Il rumore dell'agenda è sempre di 40 estrazioni per apertura (una per atto, nell'ordine del catalogo), anche per i dicasteri in interim o senza margine.
- Un comando rifiutato lascia la partita identica: `salva()` prima e dopo uguali. Ogni rifiuto è `Errore::Comando` col nome del dicastero, dell'atto o della regione in causa.
- Ogni valore provvisorio è una manopola con `valore`, `min`, `max`, `ticket`, aggiunta a `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:53`) dal task che la introduce; quelle senza fonte portano `# assunzione del piano 5, nessuna fonte in docs/ricerca`.
- La `Vista` mostra le grandezze con un turno di ritardo (spec riga 368) e nient'altro dello stato vero: soddisfazione, agende e capacità dei ministri del proprio governo sì, rumore dell'agenda e attaccamento no.
- `VERSIONE_SCHEMA` passa a `5` nel Task 1 e resta `5` per tutto il piano.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task Rust: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.

## Review Focus

1. **Dimissioni chieste a un ministro con una proposta in sospeso e un cantiere in corso**: l'agenda del turno non si esegue, la proposta decade, il cantiere resta fermo per tutto l'interim senza perdere quanto speso, nessun panic. Test `dimissioni_annullano_agenda_e_proposta` nel Task 7.
2. **Bilancio con margine zero in tutti i dicasteri**: nessuna agenda, motivo «margine esaurito», nessuna divisione per zero nei costi e nei punteggi. Test `margine_zero_nessuna_agenda` nel Task 9.
3. **Comando su un dicastero, un atto, una regione o un successore che non esistono, o fuori momento** (dirottare la regione della Sanità, scegliere il successore di un ministro in carica): rifiuto col nome, partita invariata. Test `comandi_non_ammessi_rifiutati_col_nome` nel Task 4, esteso nei Task 5, 7 e 9 coi comandi che aggiungono.
4. **Salvataggio a metà turno dopo aver fermato e dirottato** (storia 146): la partita ricaricata chiude il turno identica a quella mai salvata. Test `salvare_a_meta_turno_conserva_le_decisioni` nel Task 5.
5. **Bilancio presentato malformato** (scaglioni non crescenti, aliquota oltre 0,9, voce negativa, dicasteri in altro ordine, anno sbagliato, fuori stagione): rifiuto col nome della voce. Test `bilancio_malformato_rifiutato_col_nome` nel Task 9.

## Dispatch

PR A: il Task 2 (`coder`, scrive solo `dati/base/atti.toml`) parte in parallelo col Task 1; i nomi delle grandezze che usa sono fissati nella tabella del Task 1, non nel codice. Dal Task 3 in poi tutto in sequenza, 3 → 4 → 5 → 6 → 7: ognuno tocca `partita.rs` (`nuova`, `chiudi_turno`, `carica` o `vista`), `vista.rs` e `manopole.toml`, e consuma i tipi del precedente nell'ordine grandezze → catalogo → ministri → agenda → misure → proposte. Il Task 3 aspetta il commit del 2, perché i test comuni includono `atti.toml` con `include_str!`.

PR B: dopo la review della PR A, sul branch impilato, 8 → 9 in sequenza: il 9 consuma `Aliquote` e il gettito dell'8 e tocca gli stessi `finanza.rs`, `legge_bilancio.rs` e `partita.rs`.

Ogni brief nomina `skill-gate`, `caveman` e, per chi scrive codice (`rust-engineer`, `coder`), `ponytail`; copia la riga **Ricerca** del task come riferimento. Il round pre-commit di fine PR aggiunge `rust-reviewer` (diff `.rs` in entrambe) e `spec-reviewer` (il piano esiste), in parallelo a `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer`. Il `craft-reviewer` della PR A legge anche nomi e motti del catalogo. La PR A chiede la revisione di Mario sul contenuto di `dati/base/atti.toml` (36 atti nuovi, spec riga 484) prima del merge.

---

## PR A · Ministri e atti (Task 1-7)

### Task 1: Grandezze dei dicasteri

**Files:**
- Create: `dati/base/grandezze.toml`
- Create: `crates/sim-core/src/grandezze.rs`
- Modify: `crates/sim-core/src/dati.rs` (`FileDati.grandezze`, `NOMI` a 8 voci, `DatiGioco::grandezze()`, validazione, manopole richieste)
- Modify: `crates/sim-core/src/errore.rs` (variante `Grandezze`)
- Modify: `crates/sim-core/src/elettorato/giudizio.rs` (indicatori di servizio e occupazione pubblica nel giudizio)
- Modify: `crates/sim-core/src/partita.rs` (campo `grandezze`, `VERSIONE_SCHEMA = 5`, `chiudi_turno`, `carica`, `vista`, `grandezze()`, `grandezza()`)
- Modify: `crates/sim-core/src/vista.rs` (`VoceGrandezza`)
- Modify: `crates/sim-core/src/lib.rs`, `crates/sim-core/tests/comune/mod.rs` (`include_str!` del file nuovo e ramo `"grandezze"` in `testo_di`)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/grandezze.rs`

**Ricerca:** `docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:17-18` (decadimento e stock con vita utile); `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:104-106` (funzionamento = salari + acquisti); `docs/ricerca/modelli/voto-economico-e-retrospettivo.md:13` (asimmetria indicatore per indicatore).

**Dispatch:** `rust-engineer` · primo della PR A, in parallelo col Task 2 (file disgiunti) · skill-gate: true.

**Interfaces:**
- Consumes: `DatiGioco::dicasteri()` con `indicatore_servizio` e `regione_bersaglio`; `Economia.bilancio` (`salari`, `acquisti` per dicastero), `Economia.livello_prezzi`; `Elettorato::giudizio` (`crates/sim-core/src/elettorato/giudizio.rs:22`).
- Produces:
  - `sim_core::grandezze::DatoGrandezza { pub id: String, pub nome: String, pub dinamica: Dinamica, pub verso: Verso, pub regionale: bool, pub iniziale: f64, pub min: f64, pub max: f64, pub dicasteri: Vec<String> }`; `Dinamica { Servizio, Turnover, Fissa }`, `Verso { Sale, Scende }` (il verso in cui la grandezza migliora), entrambe serde `snake_case`; `Verso::segno(self) -> f64` (`+1`/`−1`).
  - `sim_core::grandezze::Grandezze` — `Clone, Debug, PartialEq, Serialize, Deserialize`, campi `pub basi: Vec<Vec<f64>>`, `pub valori: Vec<Vec<f64>>`, `pub prima: Vec<Vec<f64>>` (una riga per grandezza, lunga 1 o quanto le regioni), `pub effetti: Vec<EffettoAttivo>` (vuoto fino al Task 5). `prima` sono i valori alla chiusura precedente: servono al giudizio (variazione) e alla vista (un turno di ritardo).
  - `Grandezze::iniziali(dati: &DatiGioco) -> Grandezze`; `Grandezze::avanza(&mut self, dati: &DatiGioco, e: &Economia, turno: u32)`; `Grandezze::nazionale(&self, dati: &DatiGioco, g: usize) -> f64` (media pesata sulle `forze_lavoro` per le regionali); `Grandezze::controlla(&self, dati: &DatiGioco) -> Result<(), Errore>`.
  - `DatiGioco::grandezze() -> &[DatoGrandezza]`, `DatiGioco::grandezza(id: &str) -> Option<usize>`.
  - `Errore::Grandezze(String)` → `grandezze dei dicasteri non valide: {0}`.
  - `Partita::grandezze() -> &Grandezze` (stato vero, per CLI e test), `Partita::grandezza(id: &str) -> Option<f64>` (valore nazionale vero).
  - `sim_core::vista::VoceGrandezza { pub id: String, pub nome: String, pub verso: Verso, pub valori: Vec<f64> }` con i valori `prima`; `Vista.grandezze: Vec<VoceGrandezza>`.
  - `Elettorato::giudizio(&mut self, dati: &DatiGioco, e: &Economia, governo: &Governo, g: &Grandezze)`.

Il file, una `[[grandezza]]` per riga e nell'ordine della tabella (i test lo assumono: la prima con `regionale = true` è `stock_infrastrutture`), campi nell'ordine `id, nome, dinamica, verso, regionale, iniziale, min, max, dicasteri`, una riga vuota fra un blocco e l'altro. `dicasteri` sono i dicasteri i cui atti possono muoverla (riga del ticket 03):

| id | nome | dinamica | verso | regionale | iniziale | min | max | dicasteri |
|---|---|---|---|---|---|---|---|---|
| efficienza_riscossione | Efficienza della riscossione | servizio | sale | no | 100 | 0 | 200 | economia_finanze |
| sostegno_imprese | Sostegno alle imprese | servizio | sale | no | 100 | 0 | 200 | sviluppo_economico |
| copertura_welfare | Copertura del welfare | servizio | sale | no | 100 | 0 | 200 | lavoro_welfare |
| qualita_servizio_sanitario | Qualità del servizio sanitario | servizio | sale | no | 100 | 0 | 200 | sanita |
| qualita_scuola | Qualità della scuola | servizio | sale | no | 100 | 0 | 200 | istruzione_ricerca |
| qualita_trasporti | Qualità dei trasporti | servizio | sale | no | 100 | 0 | 200 | infrastrutture_trasporti |
| sicurezza_energetica | Sicurezza energetica | servizio | sale | no | 100 | 0 | 200 | ambiente_energia |
| sicurezza | Sicurezza | servizio | sale | no | 100 | 0 | 200 | interno |
| durata_processi | Durata dei processi | servizio | scende | no | 100 | 0 | 200 | giustizia |
| prontezza_difesa | Prontezza della difesa | servizio | sale | no | 100 | 0 | 200 | esteri_difesa |
| occupazione_pubblica | Occupazione pubblica | turnover | sale | no | 100 | 0 | 200 | — |
| stock_infrastrutture | Stock di infrastrutture | fissa | sale | sì | 100 | 0 | 200 | infrastrutture_trasporti |
| investimenti_privati | Investimenti privati | fissa | sale | sì | 100 | 0 | 200 | sviluppo_economico |
| domanda_regionale | Domanda della regione (punti di gap) | fissa | sale | sì | 0 | −5 | 5 | infrastrutture_trasporti, sviluppo_economico |
| prezzo_energia | Prezzo dell'energia (% sugli input importati) | fissa | scende | no | 0 | −50 | 50 | ambiente_energia |
| politiche_attive | Politiche attive (punti di disoccupazione strutturale tolti) | fissa | sale | no | 0 | 0 | 5 | lavoro_welfare |
| capitale_umano | Capitale umano | fissa | sale | no | 100 | 0 | 200 | istruzione_ricerca |
| sovraffollamento_carceri | Sovraffollamento delle carceri | fissa | scende | no | 100 | 0 | 200 | giustizia |
| immigrazione_irregolare | Immigrazione irregolare | fissa | scende | no | 100 | 0 | 200 | interno |
| corruzione | Corruzione | fissa | scende | no | 100 | 0 | 200 | giustizia |

Dinamica della base, a ogni `avanza`, dopo `prima ← valori`:
- `servizio`: `base += velocita_servizio · (obiettivo − base)`, con `obiettivo = iniziale · (F/F₀)^elasticita_servizio`, dove `F = (salari + acquisti)/livello_prezzi` del dicastero il cui `indicatore_servizio` è questa grandezza, nel `bilancio` in vigore, e `F₀` lo stesso al turno zero (da `Bilancio::iniziale`, prezzi 1);
- `turnover`: come `servizio`, con `S = Σ salari dei dicasteri / livello_prezzi` al posto di `F` e `velocita_turnover`;
- `fissa`: `base = iniziale`.

Poi `valore = clamp(base + Σ contributi degli effetti attivi, min, max)`, per ogni regione se regionale; il Task 5 porta gli effetti. `iniziali` mette `basi = valori = prima = iniziale`.

Validazione, in `DatiGioco::valida` dopo i dicasteri, messaggi esatti dentro `Errore::Grandezze`: id ripetuto (`grandezza ripetuta: {id}`), `min ≤ iniziale ≤ max` finiti (`{id}: iniziale fuori da min-max`), dicastero sconosciuto in `dicasteri` (`{id}: dicastero sconosciuto {d}`); ogni `indicatore_servizio` è una grandezza `servizio` nazionale (`indicatore di servizio senza grandezza: {indicatore}`); una grandezza regionale ha in `dicasteri` solo dicasteri con `regione_bersaglio` (`{id}: regionale, ma {d} non ha regione bersaglio`); `GRANDEZZE_RICHIESTE`, quelle che il motore legge per nome, esistono col `regionale` atteso (`grandezza richiesta mancante o di forma sbagliata: {id}`): `occupazione_pubblica` nazionale, `stock_infrastrutture`, `investimenti_privati`, `domanda_regionale` regionali, `prezzo_energia`, `politiche_attive`, `capitale_umano` nazionali. Le voci degli interessi restano controllate da `valida_interessi` (`crates/sim-core/src/elettorato/mod.rs:271-290`), che ammette già `occupazione_pubblica` e gli indicatori di servizio, ora tutti grandezze.

Giudizio: per ogni voce degli interessi della condizione diversa da `reddito` e `disoccupazione`, `Δ = verso.segno() · (valore − prima) / scala_servizio` sulla grandezza nazionale omonima; contributo `w · asimmetrica(Δ, ρ) · colpa(responsabile, p)`, con responsabile il partito del dicastero di cui è l'indicatore di servizio, il partito del ruolo `ColpaImposteImpiegoPubblico` per `occupazione_pubblica` (spec riga 312). Chiusura: `Grandezze::avanza` subito dopo `Economia::passo`, prima di `aggiorna_redditi`, con il turno che si chiude.

Manopole nuove (`ticket` fra parentesi), tutte `# assunzione del piano 5, nessuna fonte in docs/ricerca`:

| nome | valore | min | max | ticket |
|---|---|---|---|---|
| scala_servizio | 1.0 | 0.2 | 5.0 | 17 |
| elasticita_servizio | 1.0 | 0.2 | 2.0 | 03 |
| velocita_servizio | 0.25 | 0.0 | 1.0 | 03 |
| velocita_turnover | 0.1 | 0.0 | 1.0 | 07 |

- [ ] **Step 1: Scrivi `dati/base/grandezze.toml` e i test che falliscono**

```rust
// crates/sim-core/tests/grandezze.rs
mod comune;
use comune::{dati_base, file_base, imposta, senza_shock, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::elettorato::{Condizione, condizione_di};
use sim_core::{Errore, Partita};

#[test]
fn grandezze_base_si_caricano() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    assert_eq!(v.grandezze.len(), 20);
    let stock = v.grandezze.iter().find(|g| g.id == "stock_infrastrutture").unwrap();
    assert_eq!(stock.valori, vec![100.0; 20]);
    assert_eq!(p.grandezza("qualita_servizio_sanitario"), Some(100.0));
}

#[test]
fn indicatore_di_servizio_senza_grandezza_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "grandezze", "id = \"qualita_scuola\"", "id = \"qualita_scuole\"");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(matches!(e, Errore::Grandezze(_)));
    assert!(e.to_string().contains("qualita_scuola"), "{e}");
}

#[test]
fn grandezza_richiesta_di_forma_sbagliata_rifiutata() {
    let f = sostituisci(file_base(), "grandezze", "regionale = true", "regionale = false");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("stock_infrastrutture"), "{e}");
}

#[test]
fn grandezza_regionale_di_un_dicastero_senza_bersaglio_rifiutata() {
    let f = sostituisci(file_base(), "grandezze", "dicasteri = [\"infrastrutture_trasporti\"]\n\n[[grandezza]]\nid = \"investimenti_privati\"", "dicasteri = [\"sanita\"]\n\n[[grandezza]]\nid = \"investimenti_privati\"");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("sanita"), "{e}");
}

#[test]
fn servizio_segue_il_funzionamento_reale() {
    let fermo = imposta(senza_shock(), "manopole", "[velocita_servizio]\nvalore = ", "0.0");
    let mut a = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let mut b = Partita::nuova(DatiGioco::da_testi(&fermo).unwrap(), 1);
    let mut serie = Vec::new();
    for _ in 0..10 {
        a.chiudi_turno();
        b.chiudi_turno();
        serie.push(a.grandezza("qualita_servizio_sanitario").unwrap());
        assert_eq!(b.grandezza("qualita_servizio_sanitario"), Some(100.0));
    }
    // Dal 2028 il bilancio resta quello nominale del 2027: l'inflazione lo erode.
    assert!(serie[9] < serie[5], "{serie:?}");
}

#[test]
fn servizio_in_calo_pesa_sul_partito_del_dicastero() {
    let base = imposta(senza_shock(), "scenario", "sanita = ", "\"centrodestra\"");
    let fermo = imposta(base.clone(), "manopole", "[velocita_servizio]\nvalore = ", "0.0");
    let giudizio = |f| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        for _ in 0..10 {
            p.chiudi_turno();
        }
        let j = p.partiti().iter().position(|x| x.id == "centrodestra").unwrap();
        let g = &p.elettorato().gruppi;
        let pensionati: Vec<f64> = (0..g.len())
            .filter(|&i| condizione_di(i) == Condizione::Pensionato)
            .map(|i| g[i].giudizio[j])
            .collect();
        pensionati.iter().sum::<f64>() / pensionati.len() as f64
    };
    assert!(giudizio(base) < giudizio(fermo));
}

#[test]
fn indicatori_pubblicati_con_un_turno_di_ritardo() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..3 {
        p.chiudi_turno();
    }
    let g = p.grandezze();
    let v = p.vista();
    for (i, voce) in v.grandezze.iter().enumerate() {
        assert_eq!(voce.valori, g.prima[i]);
    }
    assert!(g.valori != g.prima, "nessuna grandezza si è mossa nell'ultimo turno");
}

#[test]
fn salvataggio_con_grandezze_corte_illeggibile() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.chiudi_turno();
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["grandezze"]["valori"][11] = serde_json::json!([100.0]);
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}

#[test]
fn grandezze_restano_nei_limiti_dopo_200_turni() {
    let dati = dati_base();
    let mut p = Partita::nuova(dati.clone(), 9);
    for _ in 0..200 {
        p.chiudi_turno();
    }
    for (d, riga) in dati.grandezze().iter().zip(&p.grandezze().valori) {
        assert!(riga.iter().all(|x| x.is_finite() && (d.min..=d.max).contains(x)), "{}", d.id);
    }
    p.salva().unwrap();
}
```

`imposta` col prefisso `"sanita = "` tocca la riga `[governo]` dello scenario (`dati/base/scenario.toml:17`), l'unica con quella chiave.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test grandezze`
Expected: errore di compilazione, `Vista.grandezze` e `Partita::grandezze` non esistono.

- [ ] **Step 3: Implementa `grandezze.rs`, la validazione, il giudizio e i campi di `Partita`**

`carica` chiama `Grandezze::controlla`: righe in numero e lunghezza giusti, tutto finito, `effetti` con indici di grandezza, regione e dicastero validi (il Task 5 aggiunge i campi). Nel test `salvataggio_con_grandezze_corte_illeggibile` la riga 11 è `stock_infrastrutture`, lunga 20.

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS; `app/src/lib/tipi/VoceGrandezza.ts` e `Verso.ts` nuovi.

- [ ] **Step 5: Commit**

```bash
git add dati/base crates/sim-core app/src/lib/tipi
git commit -m "feat(core): ministry quantities with service decay and delayed view"
```

---

### Task 2: Catalogo dei 40 atti

Contenuto, non codice: 36 atti nuovi accanto ai 4 della Sanità dell'asset, sotto la revisione di Mario nella PR.

**Files:**
- Create: `dati/base/atti.toml`

**Ricerca:** `docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:17-18` (impulso che decade, stock con vita utile), `:36` (il dilemma fra ritardo e consenso che la griglia 2×2 porta in ogni dicastero); `docs/ricerca/sistemi/ministri-e-gabinetto.md:31` (niente volti né nomi di politici reali).

**Dispatch:** `coder` · in parallelo col Task 1, scrive solo `dati/base/atti.toml` · skill-gate: true. Legge `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md` (schema e Sanità) e la tabella delle grandezze del Task 1. Tono dei motti: `.scratch/v1-giocabile/issues/10-direzione-artistica.md`.

**Interfaces:**
- Consumes: gli id delle grandezze e la colonna `dicasteri` della tabella del Task 1; gli id dei dicasteri di `dati/base/dicasteri.toml`.
- Produces: `dati/base/atti.toml`, 40 `[[atto]]`, campi nell'ordine `id, nome, motto, dicastero, velocita, lato, posizione, turni, costo, effetti`, che il Task 3 carica. Ogni effetto è `{ grandezza, intensita, ritardo, forma, durata }`, con `forma` `"impulso"` (`durata` = emivita in turni) o `"stock"` (`durata` = vita utile in turni). `turni = 1` è un atto puntuale, `turni > 1` un cantiere che costa `costo` a ogni turno di lavori e rende a lavori finiti. `costo` in quote trimestrali del margine discrezionale di riferimento. Intensità e ritardi sono quelli scritti per un ministro di capacità 3.

La Sanità, verbatim (i test del Task 3 vi si appoggiano):

```toml
[[atto]]
id = "sanita_liste_straordinario"
nome = "Piano straordinario sulle liste d'attesa"
motto = "<motto>"
dicastero = "sanita"
velocita = "rapido"
lato = "sinistra"
posizione = 3.0
turni = 1
costo = 1.0
effetti = [
  { grandezza = "qualita_servizio_sanitario", intensita = 8.0, ritardo = 1, forma = "impulso", durata = 2.0 },
]

[[atto]]
id = "sanita_convenzioni_privato"
nome = "Convenzioni col privato accreditato"
motto = "<motto>"
dicastero = "sanita"
velocita = "rapido"
lato = "destra"
posizione = 5.0
turni = 1
costo = 1.0
effetti = [
  { grandezza = "qualita_servizio_sanitario", intensita = 8.0, ritardo = 1, forma = "impulso", durata = 1.5 },
]

[[atto]]
id = "sanita_case_comunita"
nome = "Case di comunità e medicina territoriale"
motto = "<motto>"
dicastero = "sanita"
velocita = "lento"
lato = "sinistra"
posizione = 3.0
turni = 4
costo = 1.0
effetti = [
  { grandezza = "qualita_servizio_sanitario", intensita = 4.0, ritardo = 0, forma = "stock", durata = 40.0 },
]

[[atto]]
id = "sanita_accorpamento_asl"
nome = "Accorpamento delle ASL e premi ai primari"
motto = "<motto>"
dicastero = "sanita"
velocita = "lento"
lato = "destra"
posizione = 5.0
turni = 1
costo = 0.5
effetti = [
  { grandezza = "qualita_servizio_sanitario", intensita = 2.0, ritardo = 4, forma = "stock", durata = 20.0 },
]
```

`<motto>` lo scrive il task, uno per atto. Regole per gli altri 36, che il Task 3 valida:
- per dicastero esattamente 4 atti, uno per casella: rapido-sinistra, rapido-destra, lento-sinistra, lento-destra; ordine nel file: dicasteri nell'ordine di `dicasteri.toml`, caselle in quest'ordine;
- `id` = `<id del dicastero>_<parola>`; nomi in italiano, generici, mai una legge vera col suo nome né un politico reale;
- `posizione` da 2.0 a 3.5 a sinistra, da 4.5 a 6.0 a destra;
- rapidi: `turni = 1`, `ritardo` 0-2, `forma = "impulso"` con `durata` 1-4; lenti: cantiere con `turni` 3-8, o puntuale con `ritardo` ≥ 3, `forma = "stock"` con `durata` 12-60;
- `costo` da 0.5 a 2.0, e in ogni dicastero almeno un atto con `costo ≤ 1.0`: al turno zero resta un trimestre di margine, una quota, e ogni ministro deve poter annunciare un atto;
- 1-2 effetti, solo su grandezze che hanno il dicastero nella colonna `dicasteri`;
- intensità per grandezza, lieve / moderato / forte: indici 2 / 4 / 8; `domanda_regionale` 0.3 / 0.6 / 1.0; `prezzo_energia` −1 / −2 / −4; `politiche_attive` 0.05 / 0.1 / 0.2. Il segno porta il verso: un atto che peggiora la sua grandezza ha l'intensità col segno contrario al verso;
- Infrastrutture e Trasporti e Sviluppo economico (spec riga 268, ticket 27:31-33): ogni atto ha almeno un effetto regionale; i due rapidi un impulso su `domanda_regionale`, i due lenti uno stock su `stock_infrastrutture` (Infrastrutture e Trasporti) o `investimenti_privati` (Sviluppo economico); ciascuno dei due ha almeno un cantiere, e quello di Infrastrutture e Trasporti con `costo ≤ 1.0` (i test del Task 7 ne avviano uno e vogliono margine per l'agenda dopo);
- Economia e Finanze: la riga del ticket 03 dà riscossione, gestione del debito e dismissioni; nella v1 tutti e quattro muovono `efficienza_riscossione` (un condono, a destra, la muove con intensità negativa a ritardo lungo e positiva subito).

- [ ] **Step 1: Scrivi `dati/base/atti.toml`**

Intestazione: `# Catalogo degli atti del ministro (ticket 16). Sanità dall'asset 16; gli altri 36 sono una bozza del piano 5 sotto revisione di Mario. Intensità e ritardi per un ministro di capacità 3.`

- [ ] **Step 2: Controlla la forma del file**

Run: `python3 -I -c 'import tomllib,collections as c;a=tomllib.load(open("dati/base/atti.toml","rb"))["atto"];k=c.Counter((x["dicastero"],x["velocita"],x["lato"]) for x in a);print(len(a),len({x["id"] for x in a}),max(k.values()),len(k))'`
Expected: `40 40 1 40`.

- [ ] **Step 3: Commit**

```bash
git add dati/base/atti.toml
git commit -m "feat(dati): draft catalogue of the 40 minister acts"
```

---

### Task 3: Catalogo caricato e validato

**Files:**
- Create: `crates/sim-core/src/atti.rs`
- Modify: `crates/sim-core/src/dati.rs` (`FileDati.atti`, `NOMI` a 9 voci, `DatiGioco::atti()`, validazione)
- Modify: `crates/sim-core/src/errore.rs` (variante `Atti`)
- Modify: `crates/sim-core/src/lib.rs`, `crates/sim-core/tests/comune/mod.rs` (`include_str!` e ramo `"atti"`)
- Test: `crates/sim-core/tests/atti.rs`

**Ricerca:** `docs/ricerca/tecnica/modding-e-dati-esterni.md:14-15` (TOML, validazione all'avvio col nome); `docs/ricerca/sistemi/policy-e-effetti.md:51` (il catalogo come grammatica, per tenere basso il bilanciamento manuale).

**Dispatch:** `rust-engineer` · dopo i commit dei Task 1 e 2: valida contro `DatiGioco::grandezze()` del 1 e include `atti.toml` del 2 · skill-gate: true.

**Interfaces:**
- Consumes: `DatoGrandezza.dicasteri` e `regionale`, `Dicastero.regione_bersaglio`.
- Produces:
  - `sim_core::atti::Atto { pub id: String, pub nome: String, pub motto: String, pub dicastero: String, pub velocita: Velocita, pub lato: Lato, pub posizione: f64, pub turni: u32, pub costo: f64, pub effetti: Vec<EffettoAtto> }`; `EffettoAtto { pub grandezza: String, pub intensita: f64, pub ritardo: u32, pub forma: Forma, pub durata: f64 }`; `Velocita { Rapido, Lento }`, `Lato { Sinistra, Destra }`, `Forma { Impulso, Stock }`, serde `snake_case`, `Display` col nome minuscolo. Tutti `Clone, Debug, PartialEq, Serialize, Deserialize`.
  - `DatiGioco::atti() -> &[Atto]`; `DatiGioco::atti_del_dicastero(d: usize) -> impl Iterator<Item = (usize, &Atto)>` (indice nel catalogo, nell'ordine del file).
  - `Errore::Atti(String)` → `catalogo degli atti non valido: {0}`.

Controlli in quest'ordine, messaggi esatti: id ripetuto (`atto ripetuto: {id}`); dicastero sconosciuto (`{id}: dicastero sconosciuto {d}`); per ogni dicastero, nell'ordine dei dati, 4 atti (`{d} ha {n} atti, ne servono 4`) e caselle distinte (`{d}: due atti nella casella {velocita} {lato}`); per atto: `posizione` finita in 1-7 e dal lato giusto, sinistra ≤ 4, destra ≥ 4 (`{id}: posizione {p} dal lato sbagliato ({lato})`); `turni` in 1-20, `costo` finito in (0, 10], 1-2 effetti (`{id}: {campo} fuori intervallo`); per effetto: grandezza esistente (`{id}: grandezza sconosciuta {g}`), col dicastero nella sua colonna `dicasteri` (`{id}: {g} non è nella riga del dicastero {d}`), `intensita` finita e diversa da zero, `ritardo` ≤ 40, `durata` finita e positiva (`{id}: {campo} fuori intervallo`); un atto di un dicastero con regione bersaglio ha almeno un effetto regionale (`{id}: nessun effetto su una grandezza regionale`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/atti.rs
mod comune;
use comune::{dati_base, file_base, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::Errore;

fn rifiuto(file: &str, da: &str, a: &str) -> String {
    let e = DatiGioco::da_testi(&sostituisci(file_base(), file, da, a)).unwrap_err();
    assert!(matches!(e, Errore::Atti(_)), "{e}");
    e.to_string()
}

#[test]
fn catalogo_base_si_carica() {
    let d = dati_base();
    assert_eq!(d.atti().len(), 40);
    for i in 0..d.dicasteri().len() {
        assert_eq!(d.atti_del_dicastero(i).count(), 4);
    }
    let c = d.atti().iter().find(|a| a.id == "sanita_case_comunita").unwrap();
    assert_eq!(c.turni, 4);
}

#[test]
fn dicastero_con_tre_atti_rifiutato_col_nome() {
    let m = rifiuto("atti", "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"destra\"", "dicastero = \"giustizia\"\nvelocita = \"lento\"\nlato = \"destra\"");
    assert!(m.contains("sanita ha 3 atti"), "{m}");
}

#[test]
fn casella_doppia_rifiutata_col_nome() {
    let m = rifiuto("atti", "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"sinistra\"", "dicastero = \"sanita\"\nvelocita = \"lento\"\nlato = \"destra\"");
    assert!(m.contains("sanita: due atti nella casella lento destra"), "{m}");
}

#[test]
fn posizione_dal_lato_sbagliato_rifiutata() {
    let m = rifiuto(
        "atti",
        "lato = \"sinistra\"\nposizione = 3.0\nturni = 1\ncosto = 1.0\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
        "lato = \"sinistra\"\nposizione = 5.0\nturni = 1\ncosto = 1.0\neffetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = 8.0",
    );
    assert!(m.contains("sanita_liste_straordinario: posizione 5"), "{m}");
}

#[test]
fn effetto_su_grandezza_sconosciuta_rifiutato_col_nome() {
    let m = rifiuto("atti", "grandezza = \"qualita_servizio_sanitario\", intensita = 8.0", "grandezza = \"qualita_servizio_sanitaro\", intensita = 8.0");
    assert!(m.contains("qualita_servizio_sanitaro"), "{m}");
}

#[test]
fn effetto_fuori_dalla_riga_del_dicastero_rifiutato() {
    let m = rifiuto("atti", "grandezza = \"qualita_servizio_sanitario\", intensita = 8.0", "grandezza = \"qualita_scuola\", intensita = 8.0");
    assert!(m.contains("qualita_scuola non è nella riga del dicastero sanita"), "{m}");
}
```

Le sostituzioni si appoggiano al testo verbatim della Sanità del Task 2: `qualita_servizio_sanitario` appare solo lì, e la combinazione `posizione = 3.0 … intensita = 8.0` solo in `sanita_liste_straordinario`.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test atti`
Expected: errore di compilazione, `DatiGioco::atti` non esiste.

- [ ] **Step 3: Implementa `atti.rs` e la validazione in `DatiGioco::valida`, dopo le grandezze**

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(core): load and validate the minister acts catalogue"
```

---

### Task 4: Ministri, soddisfazione, dimissioni e successione

**Files:**
- Create: `crates/sim-core/src/ministri/mod.rs`
- Create: `crates/sim-core/src/ministri/soddisfazione.rs`
- Create: `crates/sim-core/src/comando.rs`
- Modify: `crates/sim-core/src/caso.rs` (`Flusso::Governo = 7`)
- Modify: `crates/sim-core/src/errore.rs` (variante `Comando`)
- Modify: `crates/sim-core/src/partita.rs`, `crates/sim-core/src/vista.rs`, `crates/sim-core/src/lib.rs` (riesporta `Comando`)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/ministri.rs`

**Ricerca:** `docs/ricerca/sistemi/ministri-e-gabinetto.md:13` (la competenza corregge costi e tempi), `:28-29` (moltiplicatore, e dire sempre perché un ministro è scontento o si dimette), `:31` (niente stati senza uscita, niente volti reali).

**Dispatch:** `rust-engineer` · dopo il Task 3 · skill-gate: true.

**Interfaces:**
- Consumes: `Governo.dicasteri` (partito per dicastero, `crates/sim-core/src/partiti.rs:36-39`), `Partito.posizione_dichiarata`, `DatiElettorato.nomi`/`cognomi`, `Grandezze` del Task 1, `Caso::uniforme`/`normale` sul flusso nuovo.
- Produces:
  - `sim_core::ministri::Ministro { pub nome: String, pub partito: usize, pub capacita: u8, pub posizione: f64, pub obiettivo: f64, pub soddisfazione: f64, pub turni_in_carica: u32 }`; `Ministro::capacita_effettiva(&self, dati: &DatiGioco) -> f64` (`max(1, capacita − riduzione_rodaggio)` se `turni_in_carica == 0`, altrimenti `capacita`).
  - `sim_core::ministri::StatoDicastero { pub ministro: Option<Ministro>, pub rosa: Vec<Ministro>, pub scelta: Option<u8>, pub dimissioni_chieste: bool, pub variazioni: Vec<Variazione> }` (`None` = interim; i Task 5 e 7 aggiungono campi); `sim_core::ministri::Ministri { pub dicasteri: Vec<StatoDicastero> }`, tutti `Clone, Debug, PartialEq, Serialize, Deserialize`.
  - `sim_core::ministri::Contesto<'a> { pub dati: &'a DatiGioco, pub governo: &'a Governo, pub partiti: &'a [Partito], pub grandezze: &'a Grandezze, pub economia: &'a Economia, pub sondaggio: &'a Sondaggio, pub turno: u32, pub data: Trimestre }`: ciò che i ministri leggono della partita, costruito da `Partita` coi prestiti dei suoi campi (economia e sondaggio servono dal Task 5).
  - `Ministri::iniziali(cx: &Contesto, caso: &mut Caso) -> Ministri`, chiamata in `Partita::nuova` dopo `Informazione::nuova`; `Ministri::soglie(&mut self, cx: &Contesto)`; `Ministri::apri(&mut self, cx: &Contesto, caso: &mut Caso)`; `Ministri::controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore>`.
  - `sim_core::ministri::Causa` — `enum { IndicatoreInCalo, ObiettivoRaggiunto }` in questo task (i Task 5, 7 e 9 aggiungono `AttoFermato`, `AttoDirottato`, `CantiereFermato`, `PropostaRespinta`, `BudgetTagliato`), serde `snake_case`, `Display` in italiano («indicatore di servizio in calo», «obiettivo raggiunto», …); `Variazione { pub causa: Causa, pub delta: f64, pub turno: u32 }`.
  - `sim_core::Comando` — `Clone, Debug, PartialEq, Serialize, Deserialize`, TS esportato, `#[serde(tag = "tipo", rename_all = "snake_case")]`; in questo task `ChiediDimissioni { dicastero: String }` e `ScegliSuccessore { dicastero: String, indice: u8 }`.
  - `Partita::comanda(&mut self, c: Comando) -> Result<(), Errore>`, in `comando.rs`; `Partita::ministri() -> &Ministri`.
  - `Errore::Comando(String)` → `comando non ammesso: {0}`.
  - `sim_core::vista::VoceMinistero { pub dicastero: String, pub ministro: Option<VoceMinistro>, pub rosa: Vec<VoceMinistro> }` e `VoceMinistro { pub nome: String, pub partito: String, pub capacita: u8, pub posizione: f64, pub obiettivo: f64, pub soddisfazione: f64, pub rodaggio: bool, pub minaccia: bool, pub causa_principale: Option<Causa>, pub variazioni: Vec<Variazione> }`; `Vista.ministeri: Vec<VoceMinistero>`, nell'ordine dei dicasteri.

Regole:
- **Generazione** (al turno zero per i 10 del governo uscente, poi per ogni rosa), tutto da `Flusso::Governo`: nome `nomi[⌊u·n⌋] + " " + cognomi[⌊u·n⌋]`, riestratto finché non coincide con un ministro in carica o in rosa (al più 100 volte); `capacita = min(5, 1 + ⌊5u⌋)`; `posizione = clamp(posizione_dichiarata del partito + scarto_ministri · normale, 1, 7)`. `obiettivo` = valore vero dell'indicatore di servizio del dicastero + `verso.segno() · passo_obiettivo`; `soddisfazione = soddisfazione_base`. Il governo uscente parte con `turni_in_carica = 1` (niente rodaggio), un successore con `0`.
- **Soglie**, in chiusura dopo il motore, per ogni dicastero con ministro: variazione `IndicatoreInCalo` di `−costo_calo_servizio · max(0, −Δ)/scala_servizio` con `Δ = verso.segno() · (valore − prima)` del suo indicatore; se `verso.segno() · (valore − obiettivo) ≥ 0`, `ObiettivoRaggiunto` di `+premio_obiettivo` e `obiettivo += verso.segno() · passo_obiettivo`; poi `soddisfazione += ritorno_soddisfazione · (soddisfazione_base − soddisfazione)`, limitata a 0-100; `turni_in_carica += 1`. Dimissioni se `soddisfazione < soglia_dimissioni` o `dimissioni_chieste`: `ministro = None`, `dimissioni_chieste = false`. Ogni variazione si applica subito alla soddisfazione e si accoda con il suo `turno`.
- **Apertura**, dopo `turno += 1`: le variazioni con `turno < turno_corrente − 1` si scartano; un dicastero entrato in interim nella chiusura riceve una rosa di 3 dal partito che lo tiene, in ordine di capacità decrescente e poi di nome; un dicastero che ha passato il suo turno di interim prende `rosa[scelta.unwrap_or(0)]`, che entra in carica col rodaggio, e la rosa si svuota.
- **Vista**: `minaccia = soddisfazione < soglia_minaccia`; `causa_principale` = la causa con la somma più negativa fra le variazioni tenute, `None` se nessuna è negativa; `rodaggio = turni_in_carica == 0`; `partito` è l'id.
- **Comandi**, col dicastero nominato dall'id dei dati, rifiuti esatti: id sconosciuto `dicastero sconosciuto: {id}`; `ChiediDimissioni` su un interim `{dicastero} è in interim`, ripetuta `dimissioni già chieste: {dicastero}`; `ScegliSuccessore` fuori dall'interim `{dicastero} non è in interim`, con `indice ≥ rosa.len()` `successore fuori dalla rosa di {dicastero}: {indice}`.

Manopole nuove, tutte del ticket 04 e `# assunzione del piano 5, nessuna fonte in docs/ricerca`: `soddisfazione_base` 60 (30-80), `ritorno_soddisfazione` 0.1 (0-0.5), `costo_calo_servizio` 3 (0-10), `premio_obiettivo` 10 (0-30), `passo_obiettivo` 5 (1-20), `riduzione_rodaggio` 1 (0-2), `scarto_ministri` 0.5 (0-1.5).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/ministri.rs
mod comune;
use comune::{dati_base, file_base, imposta};
use sim_core::dati::DatiGioco;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

fn dimissioni(dicastero: &str) -> Comando {
    Comando::ChiediDimissioni { dicastero: dicastero.into() }
}

#[test]
fn dieci_ministri_al_turno_zero() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.ministeri.len(), 10);
    for m in &v.ministeri {
        let x = m.ministro.as_ref().unwrap();
        assert!((1..=5).contains(&x.capacita));
        assert!((1.0..=7.0).contains(&x.posizione));
        assert_eq!(x.soddisfazione, 60.0);
        assert!(!x.rodaggio && !x.minaccia);
    }
    assert_eq!(ministero(&v, "esteri_difesa").ministro.as_ref().unwrap().partito, "centrodestra");
    assert_eq!(ministero(&v, "sanita").ministro.as_ref().unwrap().obiettivo, 105.0);
    assert_eq!(ministero(&v, "giustizia").ministro.as_ref().unwrap().obiettivo, 95.0);
}

#[test]
fn stesso_seme_stessi_ministri() {
    let a = Partita::nuova(dati_base(), 4).vista().ministeri;
    assert_eq!(a, Partita::nuova(dati_base(), 4).vista().ministeri);
    assert_ne!(a, Partita::nuova(dati_base(), 5).vista().ministeri);
}

#[test]
fn dimissioni_chieste_portano_interim_rosa_e_rodaggio() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert!(m.ministro.is_none());
    assert_eq!(m.rosa.len(), 3);
    assert!(m.rosa.iter().all(|x| x.partito == "destra"));
    assert!(m.rosa.windows(2).all(|w| w[0].capacita >= w[1].capacita));
    let scelto = m.rosa[1].nome.clone();
    p.comanda(Comando::ScegliSuccessore { dicastero: "sanita".into(), indice: 1 }).unwrap();
    p.chiudi_turno();
    let x = ministero(&p.vista(), "sanita").ministro.clone().unwrap();
    assert_eq!(x.nome, scelto);
    assert!(x.rodaggio);
    p.chiudi_turno();
    assert!(!ministero(&p.vista(), "sanita").ministro.as_ref().unwrap().rodaggio);
}

#[test]
fn successore_non_scelto_e_il_primo_della_rosa() {
    let mut p = Partita::nuova(dati_base(), 2);
    p.comanda(dimissioni("interno")).unwrap();
    p.chiudi_turno();
    let primo = ministero(&p.vista(), "interno").rosa[0].nome.clone();
    p.chiudi_turno();
    assert_eq!(ministero(&p.vista(), "interno").ministro.as_ref().unwrap().nome, primo);
}

#[test]
fn ministri_sotto_soglia_minacciano_e_si_dimettono_a_fine_turno() {
    let f = imposta(file_base(), "manopole", "[soddisfazione_base]\nvalore = ", "30.0");
    let f = imposta(f, "manopole", "[soglia_dimissioni]\nvalore = ", "35.0");
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    assert!(p.vista().ministeri.iter().all(|m| m.ministro.as_ref().unwrap().minaccia));
    p.chiudi_turno();
    assert!(p.vista().ministeri.iter().all(|m| m.ministro.is_none() && m.rosa.len() == 3));
}

#[test]
fn comandi_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (dimissioni("sanitá"), "sanitá"),
        (Comando::ScegliSuccessore { dicastero: "sanita".into(), indice: 0 }, "sanita"),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    p.comanda(dimissioni("sanita")).unwrap();
    p.chiudi_turno();
    let e = p.comanda(Comando::ScegliSuccessore { dicastero: "sanita".into(), indice: 3 }).unwrap_err();
    assert!(e.to_string().contains("sanita"), "{e}");
    let mut q = Partita::carica(&prima).unwrap();
    assert!(q.comanda(dimissioni("sanitá")).is_err());
    assert_eq!(q.salva().unwrap(), prima);
}

#[test]
fn salvataggio_con_ministri_corti_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    let mut json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    json["ministri"]["dicasteri"].as_array_mut().unwrap().pop();
    let e = Partita::carica(&json.to_string()).unwrap_err();
    assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
}
```

Più due test d'unità in `ministri/soddisfazione.rs` sulla funzione delle soglie: `calo_del_servizio_costa_in_proporzione` (un calo di 2 `scala_servizio` costa il doppio di uno di 1) e `obiettivo_raggiunto_premia_e_si_sposta` (premio una volta, obiettivo spostato di `passo_obiettivo` nel verso giusto, anche per `durata_processi`).

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test ministri`
Expected: errore di compilazione, `Comando` e `Vista.ministeri` non esistono.

- [ ] **Step 3: Implementa `ministri/`, `comando.rs`, il flusso nuovo e i passi della chiusura**

`chiudi_turno` prende sopra di sé il commento con l'ordine completo della chiusura, che i task dopo riempiono: default delle decisioni → atti dei ministri → aula → motore (misure, economia, grandezze, redditi, giudizio, composizione, posizioni, sondaggio, statistica) → soglie → calendario → apertura (anno nuovo, rose, agende, proposte). Qui `soglie` dopo la statistica e `apri` dopo il calendario. `Ministri::controlla` in `carica`: 10 dicasteri, partiti nei limiti, capacità 1-5, soddisfazione finita in 0-100, rosa di 0 o 3, `scelta` dentro la rosa.

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): seeded ministers, satisfaction, resignation and succession"
```

---

### Task 5: Agenda, atti eseguiti e cantieri

**Files:**
- Create: `crates/sim-core/src/ministri/agenda.rs`
- Modify: `crates/sim-core/src/ministri/mod.rs` (campi `agenda`, `cantieri`, `margine_residuo` di `StatoDicastero`; cause nuove)
- Modify: `crates/sim-core/src/grandezze.rs` (`EffettoAttivo`, contributi)
- Modify: `crates/sim-core/src/comando.rs` (`Ferma`, `Dirotta`, `FermaCantiere`)
- Modify: `crates/sim-core/src/partita.rs`, `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`
- Modify: `crates/sim-core/tests/comune/mod.rs` (`ferma_tranne`, `ferma_tutti`), `crates/sim-core/tests/grandezze.rs` (i due test che confrontano partite fermano le agende)
- Test: `crates/sim-core/tests/agenda.rs`

**Ricerca:** `docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:17-19` (impulso, stock, tempo di attuazione corretto per la competenza); `docs/ricerca/sistemi/ministri-e-gabinetto.md:13`.

**Dispatch:** `rust-engineer` · dopo il Task 4: `Ministro`, `StatoDicastero`, `Comando` e i passi della chiusura del 4 · skill-gate: true.

**Interfaces:**
- Consumes: `DatiGioco::atti_del_dicastero`, `Ministro::capacita_effettiva`, `Grandezze`, `Economia.bilancio` (margine), `Economia.livello_prezzi`, `Economia.regioni` (disoccupazione), `Informazione.sondaggio.regioni[r].quote` (voti pubblici del partito), `Bilancio::iniziale` (margine di riferimento).
- Produces:
  - `sim_core::grandezze::EffettoAttivo { pub grandezza: usize, pub regione: Option<usize>, pub dicastero: usize, pub intensita: f64, pub inizio: u32, pub forma: Forma, pub durata: f64 }`; `fn contributo(e: &EffettoAttivo, turno: u32) -> f64`: 0 prima di `inizio`; impulso `I · 0.5^(età/durata)`; stock `I · max(0, 1 − età/durata)`; con `età = turno − inizio`. Un effetto esce quando è uno stock con `età ≥ durata` o un impulso con `età > 10 · durata`. Un effetto regionale vale solo nella sua regione; uno nazionale di un atto regionale vale ovunque.
  - `StatoDicastero` in più: `pub agenda: Option<Agenda>`, `pub cantieri: Vec<Cantiere>`, `pub margine_residuo: i64`; `Agenda { pub atto: usize, pub regione: Option<usize>, pub stato: StatoAgenda, pub motivo: String, pub attaccamenti: Vec<f64>, pub bersaglio: Vec<f64> }` (attaccamento dei 4 atti del dicastero, punteggio di ogni regione; vuoto se senza regione bersaglio); `StatoAgenda { Annunciata, Fermata, Dirottata }` (`PartialEq`, esportato); `Cantiere { pub atto: usize, pub regione: Option<usize>, pub turni_mancanti: u32, pub rata: i64, pub speso: i64, pub attaccamento: f64 }`, con `rata` il costo per turno fissato all'avvio.
  - `Comando::Ferma { dicastero: String }`, `Comando::Dirotta { dicastero: String, atto: Option<String>, regione: Option<String> }`, `Comando::FermaCantiere { dicastero: String, atto: String }`.
  - `Causa::AttoFermato`, `Causa::AttoDirottato`, `Causa::CantiereFermato`.
  - `VoceMinistero` in più: `pub agenda: Option<VoceAgenda>`, `pub motivo_senza_agenda: Option<String>`, `pub atti: Vec<VoceAtto>`, `pub cantieri: Vec<VoceCantiere>`, `pub margine_residuo: i64`; `VoceAgenda { pub atto: String, pub nome: String, pub regione: Option<String>, pub costo: i64, pub motivo: String, pub stato: StatoAgenda, pub in_evidenza: bool }` (`atto` e `regione` sono id); `VoceAtto { pub id: String, pub nome: String, pub motto: String, pub costo: i64, pub turni: u32, pub pagabile: bool, pub effetti: Vec<VoceEffetto> }`; `VoceEffetto { pub grandezza: String, pub migliora: bool, pub ritardo: u32 }`; `VoceCantiere { pub atto: String, pub regione: Option<String>, pub turni_mancanti: u32, pub rata: i64, pub speso: i64 }`.

Regole:
- **Costo** di un atto per turno di lavori, in euro: `round(costo · margine_rif/4 · livello_prezzi)`, con `margine_rif` il margine del dicastero in `Bilancio::iniziale`. **Disponibile** = `margine_residuo − Σ rata dei cantieri in corso del dicastero`.
- **Margine residuo**: al turno zero `round(margine 2026 / 4)`, con lo stesso arrotondamento del costo, così un atto da una quota è pagabile esatto (un trimestre dell'anno resta); all'apertura di un turno di gennaio-marzo il margine annuo del bilancio che vale nell'anno (`bilancio_approvato` se è di quell'anno, altrimenti `bilancio`). Il Task 9 lo ritocca per l'esercizio provvisorio.
- **Punteggio** dell'atto `k` del ministro: `peso_obiettivo · o + peso_posizione · v + rumore_agenda · z`, con `z` normale estratta, `v = 1 − |posizione_atto − posizione_ministro|/6`, `o = min(I, d)/d` se `d > 0` altrimenti 0, dove `d = max(0, verso.segno() · (obiettivo − valore))` sull'indicatore obiettivo e `I = verso.segno() · Σ intensità degli effetti dell'atto su quell'indicatore` (effetto nominale scritto, spec riga 328). **Attaccamento** = lo stesso senza il rumore.
- **Agenda**: fra gli atti con costo ≤ disponibile, il punteggio più alto (a parità, il primo nel catalogo). Nessun atto pagabile: niente agenda, `motivo_senza_agenda = "margine esaurito"`; interim: `"interim"`. Motivo: `"{nome dell'indicatore}: avvicina l'obiettivo"` se `peso_obiettivo · o ≥ peso_posizione · v`, altrimenti `"vicino alla sua posizione"`.
- **Regione bersaglio** (solo dicasteri con `regione_bersaglio`), scelta a parte dall'atto, perché pesa uguale su tutti e quattro: `punteggio_r = peso_bisogno_bersaglio · b_r + peso_voti_bersaglio · w_r`, con `b_r = (u_r − u_nazionale) + (100 − (stock_infrastrutture_r + investimenti_privati_r)/2)/10` e `w_r = 100 · (quota del partito del ministro nel sondaggio pubblico della regione − quota nazionale)`; la regione col punteggio più alto, a parità la prima. Al motivo si aggiunge `"; {nome della regione}: ci votano"` se `peso_voti_bersaglio · w_r ≥ peso_bisogno_bersaglio · b_r`, altrimenti `"; {nome della regione}: ne ha bisogno"`.
- **In evidenza** (storia 27, ticket 09: la regola sta nel core): il ministro minaccia le dimissioni, oppure `|posizione dell'atto − posizione dichiarata del partito del Presidente del Consiglio| ≥ distanza_evidenza`. L'evento aperto nel dicastero si aggiunge col piano 9.
- **Ferma**: solo su un'agenda `Annunciata`; variazione `AttoFermato` di `−costo_fermare · attaccamento(atto)`. **Dirotta**: solo su `Annunciata`, `atto` (un altro dei 4, pagabile) e/o `regione` (solo dicasteri con regione bersaglio); variazione `AttoDirottato` di `−costo_fermare · max(0, att(vecchio) − att(nuovo)) − costo_dirottare_regione · max(0, bersaglio(vecchia) − bersaglio(nuova))`; stato `Dirottata`. Rifiuti: `nessuna agenda: {dicastero}`, `agenda già modificata: {dicastero}`, `atto sconosciuto o di un altro dicastero: {atto}`, `margine insufficiente per {atto}`, `{dicastero} non ha regione bersaglio`, `regione sconosciuta: {regione}`, `niente da dirottare: {dicastero}` (né atto né regione, o entrambi uguali a quelli annunciati).
- **Esecuzione**, primo passo della chiusura dopo i default: per ogni dicastero con ministro, nell'ordine, prima i cantieri in corso (se `rata ≤ margine_residuo` la pagano, `speso += rata`, `turni_mancanti −= 1`; altrimenti restano fermi un turno), poi l'agenda se `Annunciata` o `Dirottata` e senza dimissioni chieste: si paga il primo turno; un puntuale registra subito i suoi effetti, un cantiere entra fra i cantieri con `rata` = costo di oggi e `turni_mancanti = turni − 1` (e registra se 0). A lavori finiti gli effetti si registrano col turno che si chiude. Effetto registrato: `intensita · c/3`, `inizio = turno + round(ritardo · 3/c)`, con `c` la capacità effettiva al momento in cui si registra; `regione` quella dell'agenda per gli effetti su grandezze regionali. In interim i cantieri restano fermi. L'agenda si svuota dopo l'esecuzione.
- **FermaCantiere**: rimuove il cantiere, quanto speso resta speso, variazione `CantiereFermato` di `−costo_fermare · attaccamento` (salvato all'avvio). Rifiuto: `nessun cantiere {atto} in {dicastero}`.
- **Apertura**: 40 normali estratte nell'ordine del catalogo, poi le agende.

Manopole nuove, `# assunzione del piano 5, nessuna fonte in docs/ricerca`: `peso_obiettivo` 1.0 (0-3, "04"), `peso_posizione` 1.0 (0-3, "04"), `rumore_agenda` 0.1 (0-0.5, "04"), `costo_fermare` 15 (0-40, "04"), `peso_bisogno_bersaglio` 0.5 (0-3, "27"), `peso_voti_bersaglio` 0.5 (0-3, "27"), `costo_dirottare_regione` 2 (0-10, "27"), `distanza_evidenza` 2.0 (1-4, "09").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/comune/mod.rs, in aggiunta (con l'`#[allow(dead_code, …)]` degli altri helper)
use sim_core::ministri::StatoAgenda;
use sim_core::{Comando, Partita};

/// Ferma le agende ancora annunciate, tranne quella di `tranne`, così due partite divergono
/// solo per ciò che il test fa.
pub fn ferma_tranne(p: &mut Partita, tranne: &str) {
    for m in p.vista().ministeri {
        let annunciata = m.agenda.as_ref().is_some_and(|a| a.stato == StatoAgenda::Annunciata);
        if annunciata && m.dicastero != tranne {
            p.comanda(Comando::Ferma { dicastero: m.dicastero }).unwrap();
        }
    }
}

pub fn ferma_tutti(p: &mut Partita) {
    ferma_tranne(p, "");
}
```

In `tests/grandezze.rs`, `servizio_segue_il_funzionamento_reale` e `servizio_in_calo_pesa_sul_partito_del_dicastero` chiamano `ferma_tutti` su ogni partita prima di ogni `chiudi_turno`: dal Task 5 i ministri agiscono, e le due partite divergerebbero anche per gli atti.

```rust
// crates/sim-core/tests/agenda.rs
mod comune;
use comune::{dati_base, ferma_tranne, ferma_tutti, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

fn ferma(d: &str) -> Comando {
    Comando::Ferma { dicastero: d.into() }
}

fn coppia(seme: u64) -> (Partita, Partita) {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    (Partita::nuova(d.clone(), seme), Partita::nuova(d, seme))
}

#[test]
fn ogni_ministro_annuncia_un_atto_che_il_margine_paga() {
    let v = Partita::nuova(dati_base(), 1).vista();
    for m in &v.ministeri {
        let a = m.agenda.as_ref().unwrap();
        assert!(a.costo <= m.margine_residuo && !a.motivo.is_empty());
        let bersaglio = ["infrastrutture_trasporti", "sviluppo_economico"].contains(&m.dicastero.as_str());
        assert_eq!(a.regione.is_some(), bersaglio, "{}", m.dicastero);
        assert_eq!(m.atti.len(), 4);
    }
}

#[test]
fn atto_eseguito_muove_la_grandezza_e_spende_il_margine() {
    let (mut a, mut b) = coppia(3);
    a.chiudi_turno();
    b.chiudi_turno();
    let costo = ministero(&a.vista(), "sanita").agenda.as_ref().unwrap().costo;
    ferma_tranne(&mut a, "sanita");
    ferma_tutti(&mut b);
    a.chiudi_turno();
    b.chiudi_turno();
    let residuo = |p: &Partita| ministero(&p.vista(), "sanita").margine_residuo;
    assert_eq!(residuo(&a), residuo(&b) - costo);
    let (mut sa, mut sb) = (0.0, 0.0);
    for _ in 0..12 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        sa += a.grandezza("qualita_servizio_sanitario").unwrap();
        sb += b.grandezza("qualita_servizio_sanitario").unwrap();
    }
    assert!(sa > sb);
}

#[test]
fn fermare_costa_soddisfazione_e_dirottare_su_un_altro_atto_costa_meno() {
    let (mut a, mut b) = coppia(5);
    a.chiudi_turno();
    b.chiudi_turno();
    let v = a.vista();
    let m = ministero(&v, "sanita");
    let annunciato = m.agenda.as_ref().unwrap().atto.clone();
    let altro = m.atti.iter().find(|x| x.id != annunciato && x.pagabile).unwrap().id.clone();
    let prima = m.ministro.as_ref().unwrap().soddisfazione;
    a.comanda(ferma("sanita")).unwrap();
    b.comanda(Comando::Dirotta { dicastero: "sanita".into(), atto: Some(altro), regione: None }).unwrap();
    let s = |p: &Partita| ministero(&p.vista(), "sanita").ministro.as_ref().unwrap().soddisfazione;
    assert!(s(&a) < prima);
    assert!(s(&b) >= s(&a));
    assert!(a.comanda(ferma("sanita")).is_err());
}

#[test]
fn cantiere_fermato_a_meta_perde_quanto_speso() {
    let (mut a, mut b) = coppia(7);
    a.chiudi_turno();
    b.chiudi_turno();
    a.comanda(Comando::Dirotta { dicastero: "sanita".into(), atto: Some("sanita_case_comunita".into()), regione: None }).unwrap();
    b.comanda(ferma("sanita")).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    let c = &ministero(&a.vista(), "sanita").cantieri;
    assert_eq!((c.len(), c[0].turni_mancanti), (1, 3));
    let speso = c[0].speso;
    a.comanda(Comando::FermaCantiere { dicastero: "sanita".into(), atto: "sanita_case_comunita".into() }).unwrap();
    let residuo = |p: &Partita| ministero(&p.vista(), "sanita").margine_residuo;
    assert_eq!(residuo(&a), residuo(&b) - speso);
    for _ in 0..10 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        assert_eq!(a.grandezza("qualita_servizio_sanitario"), b.grandezza("qualita_servizio_sanitario"));
    }
}

#[test]
fn salvare_a_meta_turno_conserva_le_decisioni() {
    let mut p = Partita::nuova(dati_base(), 11);
    p.comanda(ferma("sanita")).unwrap();
    let scelta = ministero(&p.vista(), "infrastrutture_trasporti").agenda.as_ref().unwrap().regione.clone();
    let altra = if scelta.as_deref() == Some("calabria") { "lombardia" } else { "calabria" };
    p.comanda(Comando::Dirotta { dicastero: "infrastrutture_trasporti".into(), atto: None, regione: Some(altra.into()) }).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(p.salva().unwrap(), q.salva().unwrap());
}

#[test]
fn comandi_dell_agenda_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (Comando::Dirotta { dicastero: "sanita".into(), atto: None, regione: Some("calabria".into()) }, "sanita"),
        (Comando::Dirotta { dicastero: "sanita".into(), atto: Some("giustizia_x".into()), regione: None }, "giustizia_x"),
        (Comando::Dirotta { dicastero: "sviluppo_economico".into(), atto: None, regione: Some("padania".into()) }, "padania"),
        (Comando::FermaCantiere { dicastero: "sanita".into(), atto: "sanita_case_comunita".into() }, "sanita_case_comunita"),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
}
```

Più test d'unità in `grandezze.rs` (`impulso_si_dimezza_a_ogni_emivita`, `stock_si_consuma_lungo_la_vita_utile`, `niente_prima_del_ritardo`) e in `ministri/agenda.rs` (`capacita_5_quintuplica_l_intensita_di_capacita_1_e_divide_il_ritardo`).

Al turno 1 (gennaio-marzo 2027) il margine residuo è un anno intero, quindi ogni atto della Sanità (costo ≤ 1 quota) è pagabile: i test che dirottano partono da lì.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test agenda`
Expected: errore di compilazione, `Comando::Ferma` e `VoceMinistero.agenda` non esistono.

- [ ] **Step 3: Implementa agenda, esecuzione, cantieri e i tre comandi**

`Grandezze::controlla` e `Ministri::controlla` estendono il controllo del salvataggio ai campi nuovi (indici di atto del dicastero giusto, regioni, effetti con durata positiva).

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): minister agendas, stop and redirect, acts and works"
```

---

### Task 6: Gli atti nel motore e il merito nella regione bersaglio

**Files:**
- Create: `crates/sim-core/src/misure.rs`
- Modify: `crates/sim-core/src/economia/mod.rs` (`passo` con `&Misure`, energia, PIL potenziale), `economia/finanza.rs` (riscossione), `economia/lavoro.rs` (domanda regionale, disoccupazione strutturale), `economia/spread.rs` (capacità)
- Modify: `crates/sim-core/src/elettorato/giudizio.rs` (merito regionale), `elettorato/redditi.rs` (imposte degli autonomi)
- Modify: `crates/sim-core/src/partita.rs` (campo `misure`, `misure()`)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/misure.rs`

**Ricerca:** `docs/ricerca/modelli/economia-italiana-parametri.md:106` (gettito di un'imposta come base per un fattore, canale separato dall'elasticità al ciclo); `docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:18` (stock che spostano il potenziale con la loro lentezza); ticket 27:33-36 (stock regionale su u*, merito al partito del dicastero).

**Dispatch:** `rust-engineer` · dopo il Task 5: legge gli effetti attivi e le agende del 5 · skill-gate: true. Cambia la firma di `Economia::passo`: aggiorna ogni chiamante (`partita.rs`, test d'economia, `sim-cli tara`).

**Interfaces:**
- Consumes: `Grandezze`, `Ministri` (capacità del ministro col ruolo `BilancioECredibilita`), `Economia.regioni`, `DatiGioco::struttura().piramide`.
- Produces:
  - `sim_core::misure::Misure` — `Clone, Debug, PartialEq, Serialize, Deserialize`: `pub riscossione: f64`, `pub prezzo_energia: f64`, `pub domanda_regionale: Vec<f64>`, `pub disoccupazione_strutturale: Vec<f64>`, `pub crescita_potenziale: f64`, `pub capacita_economia: f64`, `pub merito: Vec<Vec<f64>>` (regione × dicastero); `Misure::neutre(dati: &DatiGioco) -> Misure` (1, 0, zeri, 0, 3, zeri); `Misure::da(dati: &DatiGioco, g: &Grandezze, m: &Ministri, e: &Economia, turno: u32, data: Trimestre) -> Misure`.
  - `Economia::passo(&mut self, dati: &DatiGioco, caso: &mut Caso, data: Trimestre, misure: &Misure)`; `Economia::beta_okun(&self, dati: &DatiGioco) -> Vec<f64>` (i `β` regionali già riscalati, come in `crates/sim-core/src/economia/lavoro.rs:35-39`).
  - `Elettorato::giudizio(&mut self, dati, e, governo, g, misure_prima: &Misure, misure: &Misure)`; `Elettorato::aggiorna_redditi(&mut self, dati, e, misure: &Misure)`.
  - `Partita::misure() -> &Misure`.

Formule (`n` = valore nazionale, `·_r` = della regione):
- `riscossione = 1 + elasticita_riscossione · (efficienza_riscossione/100 − 1)`; moltiplica le entrate IRPEF, IRES e IVA di `conti_del_trimestre` e le imposte dei gruppi autonomi in `aggiorna_redditi` (ticket 17: la lotta all'evasione abbassa il reddito degli autonomi).
- `prezzo_energia` = la grandezza; i prezzi dei settori usano `energia + prezzo_energia` al posto di `energia` (`crates/sim-core/src/economia/mod.rs:225`).
- `domanda_regionale_r` = la grandezza; Okun: `gap_r = Σ mix·gap_settori + domanda_regionale_r`.
- `disoccupazione_strutturale_r = −stock_disoccupazione · ((stock_infrastrutture_r − 100) + (investimenti_privati_r − 100))/10 − politiche_attive`; Okun: `u_r = max(minimo, u*_r + ds_r + β_r·gap_r)`, `u*_r += isteresi · (u_r − ds_r − u*_r)`.
- `crescita_potenziale = crescita_infrastrutture · (infrastrutture_n − 100)/10 + crescita_capitale_umano · (capitale_umano − 100)/10 − crescita_giustizia · (durata_processi − 100)/10 − crescita_energia · prezzo_energia/10 + crescita_demografia · (g(anno) − g(2027))`, con `g(y) = 100 · (L(y)/L(y−1) − 1)` e `L(y) = Σ_{a=15}^{66} piramide[max(0, a − (y − 2026))]`: la piramide del 1/1/2026 invecchiata senza nascite né morti, i più giovani come la coorte 0. Si somma alla crescita del potenziale del passo (`crates/sim-core/src/economia/mod.rs:242`).
- `capacita_economia` = capacità effettiva del ministro col ruolo `BilancioECredibilita`, 1 in interim; lo spread somma `spread_capacita · (3 − capacita_economia)` dopo il tetto minimo dei fondamentali e fuori dal premio di sfiducia. `Economia::iniziale` usa `Misure::neutre`, quindi lo spread del turno zero non cambia.
- `merito[r][d]`, solo per i dicasteri con regione bersaglio: i punti di disoccupazione della regione dovuti ai loro effetti attivi, `β_r · (contributi di d su domanda_regionale_r) − stock_disoccupazione · (contributi di d su stock_infrastrutture_r e investimenti_privati_r)/10`.
- Giudizio, disoccupazione: `Δu = −(u_r − ultima_r)/σ_u`; `Δ_d = −(merito[r][d] − merito_prima[r][d])/σ_u`; `Δ_resto = Δu − Σ_d Δ_d`; contributo `w_u · (asimmetrica(Δ_resto, ρ) · colpa(partito_lavoro, p) + Σ_d asimmetrica(Δ_d, ρ) · [p == partito di d])`: nella regione bersaglio il merito è tutto del partito del dicastero (spec riga 312), senza quota del Presidente del Consiglio.
- Chiusura: `let nuove = Misure::da(...)` prima di `passo`, `let prima = std::mem::replace(&mut self.misure, nuove)`, poi `passo(&self.misure)` e il giudizio con `(&prima, &self.misure)`.

Manopole nuove, `# assunzione del piano 5, nessuna fonte in docs/ricerca`: `elasticita_riscossione` 0.3 (0-1, "03"), `spread_capacita` 10 pb (0-50, "07"), `stock_disoccupazione` 0.2 (0-1, "27"), `crescita_infrastrutture` 0.05 (0-0.3, "07"), `crescita_capitale_umano` 0.05 (0-0.3, "07"), `crescita_giustizia` 0.02 (0-0.2, "07"), `crescita_energia` 0.02 (0-0.2, "07"), `crescita_demografia` 1.0 (0-2, "07").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/misure.rs
mod comune;
use comune::{dati_base, ferma_tranne, ferma_tutti, imposta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::{Comando, Partita};

/// 6 condizioni × 3 fasce: i gruppi di una regione sono contigui (`indice_gruppo`).
const PER_REGIONE: usize = 18;

/// Due partite uguali al turno 1: nella prima `sviluppo_economico` fa un atto rapido sulla
/// domanda della Calabria, nella seconda nessun atto; tutte le altre agende fermate in entrambe.
fn prepara(f: FileDati) -> (Partita, Partita) {
    let d = DatiGioco::da_testi(&f).unwrap();
    let (mut a, mut b) = (Partita::nuova(d.clone(), 3), Partita::nuova(d, 3));
    a.chiudi_turno();
    b.chiudi_turno();
    let v = a.vista();
    let se = v.ministeri.iter().find(|m| m.dicastero == "sviluppo_economico").unwrap();
    let annunciata = se.agenda.as_ref().unwrap();
    let rapido = se.atti.iter()
        .find(|x| x.turni == 1 && x.pagabile && x.effetti.iter().any(|e| e.grandezza == "domanda_regionale"))
        .unwrap().id.clone();
    let atto = (annunciata.atto != rapido).then_some(rapido);
    let regione = (annunciata.regione.as_deref() != Some("calabria")).then(|| "calabria".to_string());
    if atto.is_some() || regione.is_some() {
        a.comanda(Comando::Dirotta { dicastero: "sviluppo_economico".into(), atto, regione }).unwrap();
    }
    ferma_tranne(&mut a, "sviluppo_economico");
    ferma_tutti(&mut b);
    (a, b)
}

fn u(p: &Partita, id: &str) -> f64 {
    p.economia().regioni.iter().find(|r| r.id == id).unwrap().disoccupazione
}

#[test]
fn atto_regionale_abbassa_la_disoccupazione_della_regione_bersaglio() {
    let (mut a, mut b) = prepara(senza_shock());
    let (mut calabria, mut lombardia) = (0.0, 0.0);
    for _ in 0..10 {
        a.chiudi_turno();
        b.chiudi_turno();
        calabria += u(&a, "calabria") - u(&b, "calabria");
        lombardia += u(&a, "lombardia") - u(&b, "lombardia");
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
    }
    assert!(calabria < 0.0);
    assert!(calabria.abs() > 10.0 * lombardia.abs());
}

#[test]
fn merito_dell_atto_al_partito_del_dicastero() {
    let f = imposta(senza_shock(), "scenario", "sviluppo_economico = ", "\"centrodestra\"");
    let f = imposta(f, "manopole", "[quota_colpa_presidente]\nvalore = ", "0.0");
    let (mut a, mut b) = prepara(f);
    for _ in 0..6 {
        a.chiudi_turno();
        b.chiudi_turno();
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
    }
    let j = a.partiti().iter().position(|x| x.id == "centrodestra").unwrap();
    let regioni = dati_base();
    let r = |id: &str| regioni.regioni().iter().position(|x| x.id == id).unwrap();
    let media = |p: &Partita, reg: usize| {
        let g = &p.elettorato().gruppi[reg * PER_REGIONE..(reg + 1) * PER_REGIONE];
        g.iter().map(|x| x.giudizio[j]).sum::<f64>() / g.len() as f64
    };
    let calabria = media(&a, r("calabria")) - media(&b, r("calabria"));
    let lombardia = media(&a, r("lombardia")) - media(&b, r("lombardia"));
    assert!(calabria > 0.0);
    assert!(calabria > 10.0 * lombardia.abs());
}

#[test]
fn riscossione_in_calo_toglie_gettito() {
    let ferma = imposta(senza_shock(), "manopole", "[velocita_servizio]\nvalore = ", "0.0");
    let gettito = |f: FileDati| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
        for _ in 0..10 {
            ferma_tutti(&mut p);
            p.chiudi_turno();
        }
        (p.misure().riscossione, p.economia().conti.entrate.iva)
    };
    let (ra, ia) = gettito(senza_shock());
    let (rb, ib) = gettito(ferma);
    assert_eq!(rb, 1.0);
    assert!(ra < 1.0 && ia < ib);
}

#[test]
fn senza_ministro_dell_economia_lo_spread_sale() {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    let (mut a, mut b) = (Partita::nuova(d.clone(), 2), Partita::nuova(d, 2));
    a.comanda(Comando::ChiediDimissioni { dicastero: "economia_finanze".into() }).unwrap();
    for _ in 0..2 {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
    }
    assert_eq!(a.misure().capacita_economia, 1.0);
    assert!(a.economia().spread >= b.economia().spread);
}
```

`imposta` col prefisso `"sviluppo_economico = "` tocca la riga `[governo]` dello scenario. Più test d'unità in `misure.rs`: `demografia_frena_il_potenziale_rispetto_al_2027` (`g(2027) − g(2027) = 0`, e con la piramide di base `g(2035) < g(2027)`), `stock_sopra_100_alzano_il_potenziale`, `misure_neutre_al_turno_zero` (`Misure::da` sulla partita nuova, prima di ogni atto, dà riscossione 1, energia 0, domanda e disoccupazione strutturale zero, crescita zero).

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test misure`
Expected: errore di compilazione, `Partita::misure` non esiste.

- [ ] **Step 3: Implementa `Misure` e i punti d'aggancio nel motore**

Il commento d'ordine sopra `Economia::passo` (`crates/sim-core/src/economia/mod.rs:184-188`) si apre con «misure (lette da `Misure`, costruita da `Partita`)». `carica` controlla `Misure`: lunghezze per regione e dicastero, tutto finito.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS, compresi i test d'economia e d'elettorato dei piani 3 e 4 con la firma nuova.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base/manopole.toml
git commit -m "feat(core): acts reach the engine, regional credit to the ministry's party"
```

---

### Task 7: Proposte di fondi

**Files:**
- Create: `crates/sim-core/src/ministri/proposte.rs`
- Modify: `crates/sim-core/src/ministri/mod.rs` (campo `proposta`, causa `PropostaRespinta`)
- Modify: `crates/sim-core/src/comando.rs` (`Accogli`, `Rinvia`, `Respingi`)
- Modify: `crates/sim-core/src/partita.rs`, `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/proposte.rs`

**Ricerca:** - nessun riferimento pertinente (regole dei ticket 04 e 09: una proposta per ministro, rinvio una volta, ignorata che vale respinta dopo un turno).

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true. Chiude la PR A, poi il round di review.

**Interfaces:**
- Consumes: punteggi e disponibile del Task 5, `Economia.bilancio` (margine e spesa del dicastero).
- Produces:
  - `StatoDicastero.proposta: Option<Proposta>`; `Proposta { pub atto: usize, pub importo: i64, pub turni_in_sospeso: u32, pub rinviata: bool }`.
  - `Comando::Accogli { dicastero: String }`, `Comando::Rinvia { dicastero: String }`, `Comando::Respingi { dicastero: String }`.
  - `Causa::PropostaRespinta`.
  - `VoceMinistero.proposta: Option<VoceProposta>`; `VoceProposta { pub atto: String, pub nome: String, pub importo: i64, pub motivo: String, pub rinviata: bool }`, con `motivo = "fondi che non bastano per {nome dell'atto}"`.

Regole:
- **Innesco** (all'apertura, dopo le agende): un ministro in carica, senza proposta, il cui atto con l'`o` più alto (`o > 0`, a parità il primo) non è pagabile, propone `importo = costo − disponibile` per quell'atto.
- **Accogli**: `margine` e `spesa` del dicastero nel `bilancio` in vigore crescono di `importo` (il denaro va in disavanzo), `margine_residuo += importo`, l'agenda diventa quell'atto, `Annunciata`, con la regione già scelta per il dicastero; la proposta esce.
- **Rinvia**: una volta, senza costo: `rinviata = true`, `turni_in_sospeso = 0`. Una seconda volta: `proposta già rinviata: {dicastero}`.
- **Respingi**: variazione `PropostaRespinta` di `−costo_respingere`; la proposta esce.
- **Default**, primo passo della chiusura: una proposta non decisa con `turni_in_sospeso == 0` passa a 1; con `≥ 1` vale come respinta, col suo costo.
- **Dimissioni**: chi esce dal dicastero porta via la sua proposta.
- Rifiuti: `nessuna proposta: {dicastero}`.

Manopola nuova: `costo_respingere` 8 (0-30, "04"), `# assunzione del piano 5, nessuna fonte in docs/ricerca`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/proposte.rs
mod comune;
use comune::{file_base, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::vista::{Vista, VoceMinistero};
use sim_core::{Comando, Errore, Partita};

fn ministero<'a>(v: &'a Vista, id: &str) -> &'a VoceMinistero {
    v.ministeri.iter().find(|m| m.dicastero == id).unwrap()
}

/// La Sanità con i quattro atti a 9 quote: nessuno è pagabile, il ministro chiede fondi.
fn sanita_cara() -> FileDati {
    let effetto = "effetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = ";
    let mut f = file_base();
    for coda in ["8.0, ritardo = 1, forma = \"impulso\", durata = 2.0", "8.0, ritardo = 1, forma = \"impulso\", durata = 1.5", "4.0", "2.0"] {
        let costo = if coda == "2.0" { "costo = 0.5\n" } else { "costo = 1.0\n" };
        let da = format!("{costo}{effetto}{coda}");
        f = sostituisci(f, "atti", &da, &da.replacen(costo, "costo = 9.0\n", 1));
    }
    f
}

fn con_proposta(seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), seme)
}

#[test]
fn fondi_che_non_bastano_portano_una_proposta() {
    let v = con_proposta(1).vista();
    let m = ministero(&v, "sanita");
    assert!(m.agenda.is_none());
    let p = m.proposta.as_ref().unwrap();
    assert!(p.importo > 0 && p.motivo.starts_with("fondi che non bastano"));
}

#[test]
fn accogliere_alza_margine_e_spesa_e_mette_l_atto_in_agenda() {
    let mut p = con_proposta(1);
    let v = p.vista();
    let m = ministero(&v, "sanita");
    let (importo, atto) = (m.proposta.as_ref().unwrap().importo, m.proposta.as_ref().unwrap().atto.clone());
    let spesa = p.economia().bilancio.dicasteri[3].spesa;
    p.comanda(Comando::Accogli { dicastero: "sanita".into() }).unwrap();
    let v = p.vista();
    let m = ministero(&v, "sanita");
    assert_eq!(m.agenda.as_ref().unwrap().atto, atto);
    assert!(m.proposta.is_none());
    assert_eq!(p.economia().bilancio.dicasteri[3].spesa, spesa + importo);
}

#[test]
fn rinviare_una_volta_senza_costo() {
    let mut p = con_proposta(1);
    let s = |p: &Partita| ministero(&p.vista(), "sanita").ministro.as_ref().unwrap().soddisfazione;
    let prima = s(&p);
    p.comanda(Comando::Rinvia { dicastero: "sanita".into() }).unwrap();
    assert_eq!(s(&p), prima);
    assert!(p.comanda(Comando::Rinvia { dicastero: "sanita".into() }).is_err());
}

#[test]
fn proposta_ignorata_vale_respinta_dopo_un_turno() {
    let mut p = con_proposta(1);
    p.chiudi_turno();
    assert!(ministero(&p.vista(), "sanita").proposta.is_some());
    p.chiudi_turno();
    let v = p.vista();
    let x = ministero(&v, "sanita").ministro.as_ref().unwrap();
    assert!(x.variazioni.iter().any(|v| v.causa == Causa::PropostaRespinta));
}

#[test]
fn dimissioni_annullano_agenda_e_proposta() {
    const IT: &str = "infrastrutture_trasporti";
    let mut p = con_proposta(1);
    p.chiudi_turno();
    // Turno 1: margine di un anno intero, Infrastrutture e Trasporti avvia un cantiere.
    let v = p.vista();
    let it = ministero(&v, IT);
    let cantiere = it.atti.iter().filter(|a| a.turni > 1 && a.pagabile).min_by_key(|a| a.costo).unwrap().id.clone();
    if it.agenda.as_ref().unwrap().atto != cantiere {
        p.comanda(Comando::Dirotta { dicastero: IT.into(), atto: Some(cantiere), regione: None }).unwrap();
    }
    p.chiudi_turno();
    // Turno 2: cantiere in corso, agenda annunciata, la Sanità ha di nuovo una proposta.
    let v = p.vista();
    assert!(ministero(&v, "sanita").proposta.is_some() && ministero(&v, IT).agenda.is_some());
    let (residuo, rata) = (ministero(&v, IT).margine_residuo, ministero(&v, IT).cantieri[0].rata);
    p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap();
    p.comanda(Comando::ChiediDimissioni { dicastero: IT.into() }).unwrap();
    p.chiudi_turno();
    // Turno 3, interim: pagata solo la rata del cantiere, non l'agenda; la proposta è decaduta.
    let v = p.vista();
    assert!(ministero(&v, "sanita").ministro.is_none() && ministero(&v, "sanita").proposta.is_none());
    assert_eq!(ministero(&v, IT).margine_residuo, residuo - rata);
    let mancanti = ministero(&v, IT).cantieri[0].turni_mancanti;
    p.chiudi_turno();
    // Turno 4: nell'interim il cantiere è rimasto fermo, senza perdere quanto speso.
    let v = p.vista();
    assert_eq!(ministero(&v, IT).cantieri[0].turni_mancanti, mancanti);
    assert_eq!(ministero(&v, IT).margine_residuo, residuo - rata);
}

#[test]
fn comandi_delle_proposte_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&file_base()).unwrap(), 1);
    let e = p.comanda(Comando::Accogli { dicastero: "giustizia".into() }).unwrap_err();
    assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains("giustizia"), "{e}");
}
```

In `dimissioni_annullano_agenda_e_proposta` la proposta della Sanità del turno zero, ignorata due volte, vale respinta in chiusura del turno 1 e rinasce all'apertura del 2, perché gli atti restano impagabili. L'indice 3 di `bilancio.dicasteri` è la Sanità, nell'ordine di `dicasteri.toml`.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test proposte`
Expected: errore di compilazione, `VoceMinistero.proposta` non esiste.

- [ ] **Step 3: Implementa le proposte, i tre comandi e il default**

- [ ] **Step 4: Esegui i test, rigenera i tipi, verifica che il core resti puro**

Run: `cargo test --workspace && npm --prefix app run check && grep -rnE "std::(fs|time|thread|net|env)|HashMap|StdRng|SmallRng" crates/sim-core/src`
Expected: PASS; nessuna riga dal `grep`.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): minister funding requests, accept, defer or reject"
```

---

## PR B · Legge di bilancio (Task 8-9)

### Task 8: Aliquote e scaglioni nel gettito e nei redditi

**Files:**
- Create: `crates/sim-core/src/economia/legge_bilancio.rs`
- Modify: `crates/sim-core/src/economia/finanza.rs` (`Bilancio.aliquote`, entrate, `ContiTrimestre.entrate_discrezionali`), `economia/regole_ue.rs` (spesa netta), `economia/mod.rs` (riesporta)
- Modify: `crates/sim-core/src/scenario.rs`, `dati/base/scenario.toml` (`[aliquote]`)
- Modify: `crates/sim-core/src/elettorato/redditi.rs` (imposte per gruppo), `elettorato/mod.rs` (`aliquota_media_irpef`, campo `aliquota_irpef_0`), `crates/sim-core/src/misure.rs` (fattori IRPEF)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/fisco.rs`

**Ricerca:** `docs/ricerca/modelli/economia-italiana-parametri.md:98` (IRPEF 2026: 23% fino a 28.000 €, 33% fino a 50.000 €, 43% oltre), `:100` (IVA 22%), `:102` (IRES 24%), `:106` (elasticità al ciclo e all'aliquota come canali separati); `docs/ricerca/modelli/finanza-pubblica.md:19` (elasticità del reddito imponibile fra 0,12 e 0,4), `:21` (base imponibile `Z(τ) = Z₀((1−τ)/(1−τ₀))^e`).

**Dispatch:** `rust-engineer` · primo della PR B, sul branch `feat/v1-piano-5-bilancio` dalla punta della PR A, dopo la sua review · skill-gate: true.

**Interfaces:**
- Consumes: `conti_del_trimestre` (`crates/sim-core/src/economia/finanza.rs:153-188`), `Elettorato::aggiorna_redditi` (`crates/sim-core/src/elettorato/redditi.rs:78-114`), `DatoGruppo.primario`/`trasferimenti` (valori del turno zero), `Elettorato::elettori`, `Misure`.
- Produces:
  - `sim_core::economia::Scaglione { pub fino_a: Option<i64>, pub aliquota: f64 }`, `Aliquote { pub irpef: Vec<Scaglione>, pub iva: f64, pub ires: f64, pub contributi: f64 }`, `Clone, Debug, PartialEq, Serialize, Deserialize`, TS esportato; `Aliquote::valida(&self) -> Result<(), String>` (il messaggio nomina la voce: `scaglioni` se vuoti, non crescenti, non positivi o con `fino_a` assente prima dell'ultimo e presente nell'ultimo; `irpef`, `iva`, `ires`, `contributi` per un'aliquota fuori da 0-0,9).
  - `pub fn irpef(scaglioni: &[Scaglione], reddito: f64) -> f64` (euro annui, a scaglioni marginali, 0 per redditi ≤ 0).
  - `Scenario.aliquote: Aliquote`, validata in `Scenario::valida` (`Errore::StrutturaIncoerente` col messaggio di `valida`); `Bilancio.aliquote: Aliquote` e `Bilancio.livello_prezzi_scaglioni: f64` (1 al turno zero); `Bilancio::iniziale` copia quelle dello scenario, `scalato` le porta invariate.
  - `Elettorato::aliquota_media_irpef(&self, dati: &DatiGioco, s: &[Scaglione]) -> f64` = `Σ elettori·irpef(lordo) / Σ elettori·lordo` sui redditi correnti, con `lordo = primario + trasferimenti` per elettore; `Elettorato.aliquota_irpef_0: f64`, fissata in `Partita::nuova` con gli scaglioni dello scenario.
  - `Misure` in più: `pub fattore_irpef: f64` = `t(S, redditi correnti)/aliquota_irpef_0`, e `pub irpef_discrezionale: f64` = `1 − t(S₀, redditi correnti)/t(S, redditi correnti)` (la parte del gettito dovuta agli scaglioni cambiati, senza il drenaggio); neutre 1 e 0.
  - `ContiTrimestre.entrate_discrezionali: i64`.

Formule:
- `eti(τ, τ₀) = ((1 − τ)/(1 − τ₀))^elasticita_imponibile`.
- IRPEF: la formula di oggi × `fattore_irpef · eti(t, t₀) · riscossione`, con `t = t(S, correnti)` e `t₀ = aliquota_irpef_0`. IVA, IRES e contributi: × `(τ/τ₀) · eti(τ, τ₀)` (e × `riscossione` per IVA e IRES), con `τ₀` dello scenario.
- `entrate_discrezionali` = `irpef · irpef_discrezionale` + Σ su IVA, IRES e contributi di `entrata − entrata con τ = τ₀`: il gettito dovuto alle aliquote cambiate. La spesa netta del trimestre (`crates/sim-core/src/economia/regole_ue.rs:64-80`) la sottrae, come le misure discrezionali sulle entrate nella regola UE.
- Imposte del gruppo: `aliquota_fascia · lordo · r` al posto di `· pressione/pressione_base` (`crates/sim-core/src/elettorato/redditi.rs:110`), con `r = (irpef(S, lordo)/lordo + τc) / (irpef(S₀, lordo₀)/lordo₀ + τc₀)`, `lordo₀ = primario + trasferimenti` del `DatoGruppo`, `r = 1` se un lordo non è positivo; `Indici.pressione` esce. Il drenaggio fiscale nasce da qui: lordo nominale che cresce, scaglioni fermi, `r` che sale.

Scenario, in `dati/base/scenario.toml` con la fonte nel commento:

```toml
# economia-italiana-parametri:98-102; contributi 33%: assunzione del piano 5, nessuna fonte in docs/ricerca
[aliquote]
iva = 0.22
ires = 0.24
contributi = 0.33
irpef = [{ fino_a = 28000, aliquota = 0.23 }, { fino_a = 50000, aliquota = 0.33 }, { aliquota = 0.43 }]
```

Manopola nuova: `elasticita_imponibile` 0.25 (0-0.6, "07"), `# finanza-pubblica:19 (0,12-0,4)`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/fisco.rs
mod comune;
use comune::{dati_base, file_base, sostituisci, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::economia::{Scaglione, irpef};
use sim_core::Partita;

fn scaglioni_2026() -> Vec<Scaglione> {
    vec![
        Scaglione { fino_a: Some(28_000), aliquota: 0.23 },
        Scaglione { fino_a: Some(50_000), aliquota: 0.33 },
        Scaglione { fino_a: None, aliquota: 0.43 },
    ]
}

#[test]
fn irpef_2026_sui_tre_scaglioni() {
    let s = scaglioni_2026();
    assert!((irpef(&s, 28_000.0) - 6_440.0).abs() < 1e-6);
    assert!((irpef(&s, 50_000.0) - 13_700.0).abs() < 1e-6);
    assert!((irpef(&s, 60_000.0) - 18_000.0).abs() < 1e-6);
    assert_eq!(irpef(&s, -5.0), 0.0);
}

#[test]
fn aliquote_del_turno_zero_lasciano_i_fattori_neutri() {
    let p = Partita::nuova(dati_base(), 1);
    assert_eq!(p.economia().bilancio.aliquote, p.economia().bilancio_approvato.as_ref().unwrap().aliquote);
    assert_eq!(p.economia().conti.entrate_discrezionali, 0);
}

#[test]
fn scaglioni_fermi_e_inflazione_danno_drenaggio_fiscale() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    for _ in 0..8 {
        p.chiudi_turno();
    }
    assert!(p.misure().fattore_irpef > 1.0);
    assert_eq!(p.misure().irpef_discrezionale, 0.0);
    assert_eq!(p.economia().conti.entrate_discrezionali, 0);
}

#[test]
fn scaglioni_non_crescenti_nello_scenario_rifiutati_col_nome() {
    let f = sostituisci(file_base(), "scenario", "{ fino_a = 50000, aliquota = 0.33 }", "{ fino_a = 20000, aliquota = 0.33 }");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("scaglioni"), "{e}");
}
```

Più test d'unità in `legge_bilancio.rs`: `aliquota_piu_alta_piu_gettito_ma_meno_che_in_proporzione` (con `elasticita_imponibile` 0,25, `τ/τ₀ · eti` da 0,22 a 0,24 sta fra 1 e 0,24/0,22).

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test fisco`
Expected: errore di compilazione, `sim_core::economia::irpef` non esiste.

- [ ] **Step 3: Implementa aliquote, gettito, imposte dei gruppi e spesa netta**

`Economia::controlla_coerenza` aggiunge le aliquote dei bilanci a ciò che un salvataggio deve rispettare (`Aliquote::valida`, `livello_prezzi_scaglioni` finito e positivo).

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base
git commit -m "feat(core): tax rates and IRPEF brackets drive revenue and group taxes"
```

---

### Task 9: Legge di bilancio ed esercizio provvisorio

**Files:**
- Modify: `crates/sim-core/src/economia/legge_bilancio.rs` (bozza, indicizzazione, riepilogo, validazione della presentazione)
- Modify: `crates/sim-core/src/economia/finanza.rs` (`BudgetDicastero.livello_prezzi`, `Bilancio.fondo_riserva`)
- Modify: `crates/sim-core/src/comando.rs` (`PresentaBilancio`)
- Modify: `crates/sim-core/src/ministri/mod.rs`, `ministri/agenda.rs` (`Causa::BudgetTagliato`, agende e accogli nell'esercizio provvisorio)
- Modify: `crates/sim-core/src/errore.rs` (variante `Bilancio`)
- Modify: `crates/sim-core/src/partita.rs` (campo `bilancio_presentato`, passo «aula», anno nuovo), `crates/sim-core/src/vista.rs` (`VistaBilancio`)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/bilancio.rs`

**Ricerca:** `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:104` (il margine che si sposta in un anno, 1,6% della manovra), `:110` (fondo di riserva [NON TROVATO], manopola sotto i 5,3 miliardi del programma 33.2); `docs/ricerca/dati/pericolosita-naturale-regioni.md:134` (fondo per le emergenze 0,2-0,4 miliardi l'anno); ticket 29 (bozza fotocopia come taglio reale, perdita reale dall'ultimo aggiornamento).

**Dispatch:** `rust-engineer` · dopo il Task 8: `Aliquote`, gettito ed `entrate_discrezionali` dell'8 · skill-gate: true. Chiude la PR B, poi il round di review.

**Interfaces:**
- Consumes: `Bilancio`, `Economia.bilancio`/`bilancio_approvato` (scambio al primo passo di gennaio-marzo, `crates/sim-core/src/economia/finanza.rs:203-208`), `Scenario::tetto_spesa_netta`, `Ministri` (margine residuo, cause), `Aliquote::valida`.
- Produces:
  - `BudgetDicastero.livello_prezzi: f64` (il livello dei prezzi a cui la voce ha preso la cifra nominale: 1 nel 2026, `1 + min(inflazione, tetto 2027)/100` nel 2027 approvato dal governo uscente); `Bilancio.fondo_riserva: i64` (euro annui; `fondo_riserva` miliardi nel 2026, scalato con le altre voci nel 2027).
  - `sim_core::economia::Voce` — `enum { Tutto, Dicastero { id: String }, Scaglioni }`, serde, TS esportato.
  - `Partita::bozza_bilancio(&self) -> Option<Bilancio>`: in ottobre-dicembre senza `bilancio_approvato`, una copia nominale del bilancio in vigore con `anno + 1`; nell'esercizio provvisorio la stessa con l'anno in corso; altrimenti `None`.
  - `Partita::indicizza(&self, b: &Bilancio, voce: &Voce) -> Result<Bilancio, Errore>`: le voci scelte × `livello_prezzi_attuale / livello_prezzi della voce` (gli scaglioni: le soglie `fino_a`), arrotondate all'euro, col livello aggiornato; `Tutto` = tutti i dicasteri e gli scaglioni, non il fondo.
  - `Partita::riepilogo_bilancio(&self, b: &Bilancio) -> Result<RiepilogoBilancio, Errore>`; `RiepilogoBilancio { pub costo_spesa: i64, pub costo_entrate: i64, pub crescita_spesa_netta: f64, pub tetto: f64, pub perdita_reale: Vec<PerditaReale> }`, `PerditaReale { pub voce: String, pub perdita: f64 }` (una per dicastero e una `"scaglioni"`), TS esportati.
  - `Comando::PresentaBilancio { bilancio: Bilancio }`, quindi `Bilancio` e `BudgetDicastero` prendono `#[cfg_attr(test, derive(TS), ts(export))]` come gli altri tipi esportati; `Errore::Bilancio(String)` → `legge di bilancio non ammessa: {0}`.
  - `Causa::BudgetTagliato`.
  - `Vista.bilancio: VistaBilancio { pub anno: u16, pub dicasteri: Vec<VoceBudget>, pub aliquote: Aliquote, pub perdita_reale_scaglioni: f64, pub fondo_riserva: i64, pub presentabile: bool, pub presentato: bool, pub esercizio_provvisorio: bool }`, `VoceBudget { pub id: String, pub spesa: i64, pub salari: i64, pub acquisti: i64, pub margine: i64, pub perdita_reale: f64 }`, sul bilancio in vigore nel trimestre (quello approvato se è dell'anno in corso, altrimenti quello corrente).

Regole:
- **Perdita reale** di una voce, in %: `100 · (1 − livello_prezzi della voce / livello_prezzi attuale)` (ticket 29: «Sanità: −4% reale in un anno»).
- **Presentazione**: ammessa con una bozza (`fuori stagione: si presenta in ottobre-dicembre o nell'esercizio provvisorio` altrimenti). `anno` uguale a quello della bozza (`anno {a}, atteso {b}`); dicasteri con gli stessi id nello stesso ordine (`dicasteri diversi da quelli dei dati`); `salari`, `acquisti`, `margine`, `fondo_riserva` ≥ 0 e ≤ 1e15 (`voce negativa o enorme: {id}`, `fondo_riserva` per il fondo); `Aliquote::valida`. La `spesa` del bilancio presentato si ignora e si ricalcola: `spesa = (spesa − salari − acquisti − margine della bozza) + salari + acquisti + margine`. Il `livello_prezzi` di una voce la cui cifra nominale cambia rispetto alla bozza diventa quello attuale; `livello_prezzi_scaglioni` lo stesso, se cambiano gli scaglioni. Ripresentare sostituisce.
- **Riepilogo**: `costo_spesa = Σ spesa + fondo` del bilancio meno quella della bozza; `costo_entrate` = gettito annuo perso rispetto alla bozza, ai redditi e alla base dell'ultimo trimestre chiuso (4 × la differenza di `entrate` delle quattro imposte calcolate con le aliquote dell'una e dell'altra); `crescita_spesa_netta = 100 · (P/A − 1)`, con `A = 4 ×` la spesa netta dell'ultimo trimestre chiuso e `P` la stessa formula con `Σ spesa dei dicasteri + fondo` del bilancio al posto di 4 × la spesa dei dicasteri del trimestre e le `entrate_discrezionali` alle aliquote del bilancio; `tetto = tetto_spesa_netta(anno del bilancio)`.
- **Aula**, dopo l'esecuzione degli atti: il bilancio presentato diventa `bilancio_approvato` e `bilancio_presentato` torna `None`. Nell'esercizio provvisorio è dell'anno in corso, quindi il passo di gennaio-marzo che segue lo mette subito in vigore; il margine residuo del dicastero diventa allora i 3/4 del margine nuovo.
- **Esercizio provvisorio** = turno di gennaio-marzo senza `bilancio_approvato` dell'anno: nessuna agenda (`motivo_senza_agenda = "esercizio provvisorio"`), `Accogli` rifiutato (`esercizio provvisorio: nessuna misura che costi denaro`), i cantieri in corso proseguono. Senza bilancio approvato a fine turno, l'anno prosegue col bilancio vecchio (il piano 7 porta le dimissioni).
- **Anno nuovo** (apertura di gennaio-marzo, dove il Task 5 rimette il margine residuo): per ogni dicastero con ministro, `taglio = 1 − spesa nuova / (spesa vecchia · (1 + media delle ultime 4 inflazioni/100))`; se positivo, variazione `BudgetTagliato` di `−costo_taglio_budget · 100 · taglio`. Spesa nuova = quella del bilancio che vale nell'anno (anche le cifre vecchie dell'esercizio provvisorio, che sono un taglio reale).
- **Fondo di riserva**: stanziato, nel riepilogo e nella vista; nessuno lo spende fino al piano 9.

Manopole nuove: `costo_taglio_budget` 2 (0-10, "04", `# assunzione del piano 5, nessuna fonte in docs/ricerca`), `fondo_riserva` 0.3 miliardi (0-2, "19", `# pericolosita-naturale-regioni:134 (0,2-0,4 l'anno); stato-del-paese:110`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/bilancio.rs
mod comune;
use comune::{dati_base, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::economia::{Bilancio, Voce};
use sim_core::elettorato::{Fascia, fascia_di};
use sim_core::{Comando, Errore, Partita};

/// Al turno 4 (ottobre-dicembre 2027) la bozza del 2028 è aperta.
fn al_bilancio_2028(seme: u64) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), seme);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    p
}

fn presenta(p: &mut Partita, b: Bilancio) -> Result<(), Errore> {
    p.comanda(Comando::PresentaBilancio { bilancio: b })
}

#[test]
fn bozza_solo_in_ottobre_dicembre_senza_bilancio_approvato() {
    assert!(Partita::nuova(dati_base(), 1).bozza_bilancio().is_none());
    let p = al_bilancio_2028(1);
    let b = p.bozza_bilancio().unwrap();
    assert_eq!(b.anno, 2028);
    assert_eq!(b.dicasteri, p.economia().bilancio.dicasteri);
}

#[test]
fn bilancio_fotocopia_e_un_taglio_reale_e_indicizzare_lo_annulla() {
    let p = al_bilancio_2028(1);
    let bozza = p.bozza_bilancio().unwrap();
    let r = p.riepilogo_bilancio(&bozza).unwrap();
    assert_eq!((r.costo_spesa, r.costo_entrate), (0, 0));
    assert!(r.perdita_reale.iter().all(|x| x.perdita > 0.0));
    let tutto = p.indicizza(&bozza, &Voce::Tutto).unwrap();
    let r = p.riepilogo_bilancio(&tutto).unwrap();
    assert!(r.costo_spesa > 0);
    assert!(r.perdita_reale.iter().all(|x| x.perdita.abs() < 1e-9));
}

#[test]
fn bilancio_presentato_vale_dall_anno_dopo() {
    let mut p = al_bilancio_2028(2);
    let mut b = p.bozza_bilancio().unwrap();
    b.dicasteri[3].margine *= 2;
    let margine = b.dicasteri[3].margine;
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(!v.bilancio.esercizio_provvisorio);
    assert_eq!(v.bilancio.anno, 2028);
    assert_eq!(v.ministeri[3].margine_residuo, margine);
}

#[test]
fn bilancio_non_presentato_porta_l_esercizio_provvisorio() {
    let mut p = al_bilancio_2028(3);
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.bilancio.esercizio_provvisorio && v.bilancio.presentabile);
    assert!(v.ministeri.iter().all(|m| m.agenda.is_none() && m.motivo_senza_agenda.as_deref() == Some("esercizio provvisorio")));
    let b = p.bozza_bilancio().unwrap();
    assert_eq!(b.anno, 2028);
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    assert!(!p.vista().bilancio.esercizio_provvisorio);
    assert_eq!(p.economia().bilancio.anno, 2028);
}

#[test]
fn taglio_al_dicastero_costa_soddisfazione() {
    let soddisfazione = |taglio: bool| {
        let mut p = al_bilancio_2028(4);
        let mut b = p.bozza_bilancio().unwrap();
        if taglio {
            b.dicasteri[3].salari = b.dicasteri[3].salari * 9 / 10;
        }
        presenta(&mut p, b).unwrap();
        p.chiudi_turno();
        p.vista().ministeri[3].ministro.clone().unwrap().soddisfazione
    };
    assert!(soddisfazione(true) < soddisfazione(false));
}

#[test]
fn scaglione_di_mezzo_piu_basso_aiuta_la_fascia_alta() {
    let imposte = |aliquota: f64| {
        let mut p = al_bilancio_2028(5);
        let mut b = p.bozza_bilancio().unwrap();
        b.aliquote.irpef[1].aliquota = aliquota;
        presenta(&mut p, b).unwrap();
        p.chiudi_turno();
        p.chiudi_turno();
        let g = &p.elettorato().gruppi;
        let media = |f: Fascia| {
            let x: Vec<f64> = (0..g.len()).filter(|&i| fascia_di(i) == f).map(|i| g[i].reddito.imposte).collect();
            x.iter().sum::<f64>() / x.len() as f64
        };
        (media(Fascia::Alta), media(Fascia::Bassa))
    };
    let (alta_33, bassa_33) = imposte(0.33);
    let (alta_28, bassa_28) = imposte(0.28);
    assert!(alta_28 / alta_33 < bassa_28 / bassa_33);
}

#[test]
fn margine_zero_nessuna_agenda() {
    let mut p = al_bilancio_2028(6);
    let mut b = p.bozza_bilancio().unwrap();
    for d in &mut b.dicasteri {
        d.margine = 0;
    }
    presenta(&mut p, b).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.ministeri.iter().all(|m| m.agenda.is_none() && m.motivo_senza_agenda.as_deref() == Some("margine esaurito")));
    p.chiudi_turno();
}

#[test]
fn iva_tagliata_pesa_sul_tetto_ue() {
    let p = al_bilancio_2028(7);
    let bozza = p.bozza_bilancio().unwrap();
    let mut b = bozza.clone();
    b.aliquote.iva = 0.20;
    let r = p.riepilogo_bilancio(&b).unwrap();
    assert!(r.costo_entrate > 0);
    assert!(r.crescita_spesa_netta > p.riepilogo_bilancio(&bozza).unwrap().crescita_spesa_netta);
}

#[test]
fn bilancio_malformato_rifiutato_col_nome() {
    let mut p = al_bilancio_2028(8);
    let prima = p.salva().unwrap();
    let bozza = p.bozza_bilancio().unwrap();
    let guasti: Vec<(Box<dyn Fn(&mut Bilancio)>, &str)> = vec![
        (Box::new(|b| b.aliquote.irpef[1].fino_a = Some(10_000)), "scaglioni"),
        (Box::new(|b| b.aliquote.iva = 1.2), "iva"),
        (Box::new(|b| b.dicasteri[8].acquisti = -1), "giustizia"),
        (Box::new(|b| b.dicasteri.swap(0, 1)), "dicasteri"),
        (Box::new(|b| b.anno = 2030), "2030"),
    ];
    for (guasta, nome) in guasti {
        let mut b = bozza.clone();
        guasta(&mut b);
        let e = presenta(&mut p, b).unwrap_err();
        assert!(matches!(e, Errore::Bilancio(_)) && e.to_string().contains(nome), "{e}");
    }
    let mut fuori = Partita::nuova(dati_base(), 8);
    let e = presenta(&mut fuori, bozza).unwrap_err();
    assert!(e.to_string().contains("fuori stagione"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}
```

`fascia_di`, `Fascia` e `Gruppo.reddito` sono già pubblici (`crates/sim-core/src/elettorato/mod.rs:99`, `:347`). L'indice 3 dei dicasteri è la Sanità, l'8 la Giustizia. Il test distributivo tocca lo scaglione 28.000-50.000 € perché in `dati/base/elettorato.toml` il lordo per elettore della fascia bassa sta tutto sotto 12.000 € e quello della fascia alta ha la mediana a 39.600 €: lo scaglione oltre 50.000 € ne toccherebbe pochi gruppi.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test bilancio`
Expected: errore di compilazione, `Partita::bozza_bilancio` non esiste.

- [ ] **Step 3: Implementa bozza, indicizzazione, riepilogo, presentazione, aula, anno nuovo ed esercizio provvisorio**

Il commento d'ordine di `chiudi_turno` riceve il passo «aula» fra gli atti e il motore.

- [ ] **Step 4: Esegui tutta la suite, rigenera i tipi, verifica l'app**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check && npm --prefix app run build`
Expected: PASS; build di Vite completata coi tipi nuovi.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): budget law with indexing, real loss and provisional budget"
```
