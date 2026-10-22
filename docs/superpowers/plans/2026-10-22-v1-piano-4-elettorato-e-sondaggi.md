# V1 · Piano 4: elettorato e sondaggi — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** I 360 gruppi dell'elettorato dentro `sim-core`, con redditi e panieri per fascia, composizione che si muove, giudizio retrospettivo sugli interessi con aspettative e colpa per ruolo, posizioni con l'ancora di Friedkin-Johnsen e voto logit con astensione; un sondaggio pubblico a tre strati d'errore con incroci, le prime stime rumorose e rivedute, gli elettori campione e la proiezione dei seggi sul sondaggio; poi la pipeline offline che tara gruppi, istogrammi, β, fedeltà, valence e astensione sui dati reali.

**Architecture:** Un modulo nuovo `crates/sim-core/src/elettorato/` tiene i dati dei gruppi (settimo file `dati/base/elettorato.toml`) e lo stato `Elettorato` di `Partita`, un blocco per file (`redditi.rs`, `composizione.rs`, `giudizio.rs`, `voto.rs`). I partiti in gioco, il governo uscente e le coalizioni dell'ultimo voto entrano nello scenario (`partiti.rs`). `crates/sim-core/src/informazione/` genera sondaggio e prime stime dallo stato vero col flusso `Flusso::Sondaggi`, e la `Vista` riceve solo quelli. La pipeline offline estende `strumenti/taratura/` con `elettorato.py` e un sottocomando `tara` di `sim-cli`, che usa lo stesso logit del gioco.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Python 3.11+ con la sola libreria standard per la pipeline; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 110-113 (righe 162-165), «Caso» (riga 251), «Motore economico» (riga 298, redditi e panieri), «Elettorato» (righe 305-314), «Elezioni e riparto» (riga 322, proiezione), «Informazione e cassa del partito» (righe 366-372), «Pipeline di taratura, offline» (righe 403-409), «Testing Decisions» (righe 424 e 429). Scope dalla tabella dei piani: [`2026-10-21-v1-piano-1-fondamenta.md:46`](2026-10-21-v1-piano-1-fondamenta.md). Ticket: `.scratch/v1-giocabile/issues/06-elettorato.md`, `14-dati-per-tarare-i-gruppi.md`, `17-tabella-degli-interessi.md`, `25-informazione-imperfetta.md`, `29-costo-del-governare.md`.

## Due PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · motore dell'elettorato e sondaggi | `feat/v1-piano-4-elettorato` | `main` | 1-8 | dati provvisori dei gruppi, partiti e governo uscente, redditi e panieri, composizione, giudizio, posizioni e voto, sondaggio e prime stime, elettori campione e proiezione |
| B · pipeline di taratura | `feat/v1-piano-4-taratura` | `feat/v1-piano-4-elettorato` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 9-12 | quote con IPF sui margini ISTAT e MEF, istogrammi, redditi e panieri, β e fedeltà su ITANES, valence e astensione tarate col logit del gioco |

La PR A gira e si testa con dati provvisori scritti da uno script deterministico (Task 1); la PR B riscrive gli stessi file con i dati reali e non cambia una riga di `sim-core` fuori da `sim-cli`.

## Confini con gli altri piani (scelte di questo piano)

- **Partiti.** Il piano 4 ha bisogno di partiti da votare prima che il piano 7 porti la fondazione e il piano 8 i partiti IA: lo scenario dichiara i sette partiti con id, nome segnaposto, posizione e valence, il governo uscente (Presidente del Consiglio e partito di ogni dicastero) e le coalizioni dell'ultimo voto. Vocazione, regola di movimento, leader e cassa arrivano col piano 8; il partito del giocatore col piano 7. La posizione percepita esiste ed è uguale alla dichiarata: la spostano le misure del piano 6.
- **Voto precedente.** La fedeltà guarda al voto dell'ultima elezione. Il voto del turno zero è del piano 7: fino ad allora il voto precedente di un gruppo è la distribuzione stazionaria del suo logit al turno zero (Task 6), così una partita senza cambiamenti non deriva da sola.
- **Interessi che non esistono ancora.** Indicatori di servizio e occupazione pubblica arrivano col piano 5: la tabella degli interessi li porta già nei dati, il giudizio li legge a variazione zero finché il loro blocco non c'è.
- **Colpa e aspettative fuori da qui.** L'eccezione della colpa nella regione bersaglio di un atto e nel perimetro di una disposizione territoriale (spec riga 312) arriva coi piani 5 e 6, che portano atti e disposizioni; le promesse che alzano l'aspettativa col piano 8; gli indicatori di servizio esatti con un turno di ritardo (spec riga 368) col piano 5.
- **Spinte sulle posizioni.** Eventi (piano 9) e campagna (piano 8) aggiungono spinte; qui l'ancora di Friedkin-Johnsen e il rumore.
- **Proiezione.** Usa la seam `elezioni::riparto` del piano 2 sul sondaggio pubblico; il voto vero resta del piano 7.
- **Registro.** La revisione della prima stima va nella vista; il registro del turno nasce col piano che porta l'esito.
- **Blocco «redditi dei gruppi e panieri».** Nell'ordine della spec (riga 289) sta fra Okun e BCE. Legge solo grandezze che BCE, SFC e spread non toccano nel turno (PIL nominale, prezzi, disoccupazione regionale, pensioni, sussidi, conti), quindi `Partita` lo chiama subito dopo `Economia::passo` con gli stessi valori, e `Economia` non conosce i gruppi.

## Global Constraints

Valgono tutti i vincoli del piano 1 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`) e del piano 3 (`2026-10-22-v1-piano-3-motore-economico.md:20-31`): italiano coi termini di `CONTEXT.md` (gruppo, condizione, fascia di reddito, interessi, giudizio, aspettativa, sondaggio, elettore campione, posizione percepita; mai «pop», «agente», «gradimento»); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova in nessun crate; la pipeline usa solo la libreria standard di Python.
- `f64` per redditi per elettore, quote, istogrammi, utilità e logit (spec riga 250); i voti passati al riparto sono `u64`.
- Ogni estrazione dell'elettorato esce da `Flusso::Elettorato`, ogni estrazione di sondaggi e prime stime da `Flusso::Sondaggi`, sempre nello stesso numero e ordine per turno anche quando l'ampiezza è zero (spec riga 251). Gli elettori campione usano un estrattore estemporaneo che non avanza nessun flusso (Task 8).
- Ogni valore provvisorio è una manopola con `valore`, `min`, `max`, `ticket`, scritta nel Task 2; quelle senza fonte portano `# assunzione del piano 4, nessuna fonte in docs/ricerca`.
- Ordine canonico dei 360 gruppi: regioni nell'ordine di `regioni.toml`, poi `Condizione::TUTTE`, poi `Fascia::TUTTE`; indice `r·18 + c·3 + f`. Il file dei dati è in quest'ordine e la validazione lo esige.
- La `Vista` non contiene mai lo stato vero: niente intenzioni vere, bias comune, effetti d'istituto, valori veri della statistica del trimestre in corso, giudizi, voto precedente (spec riga 429).
- `VERSIONE_SCHEMA` passa a `4` nel Task 2 e resta `4` per tutto il piano.
- Verifiche di ogni task Rust: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`. Di ogni task Python: `python3 -I strumenti/taratura/elettorato.py --selftest`.

## Review Focus

1. **Salvataggio manomesso con un gruppo in meno o vettori di lunghezza sbagliata** (giudizio, voto precedente, istogramma): `Partita::carica` restituisce `SalvataggioIlleggibile`, mai un panic sugli indici. Test `salvataggio_con_un_gruppo_in_meno_illeggibile` e `salvataggio_con_giudizio_corto_illeggibile` nel Task 2, esteso nei Task 6 e 7 per i campi che aggiungono.
2. **Partita lunga** (200 turni con shock): quote che sommano a 1 per regione, istogrammi validi, redditi reali finiti e positivi, intenzioni che sommano a 1, salvataggio che riesce. Test `elettorato_resta_valido_dopo_200_turni` nel Task 6.
3. **Sottocampione minuscolo** (Valle d'Aosta, campione di pochi elettori): l'incrocio esce con quote valide che sommano a 1, mai un `NaN` da una varianza su `n = 0`. Test `incrocio_della_regione_piu_piccola_valido` nel Task 7.
4. **Elettori campione chiesti più volte** (il giocatore clicca avanti e indietro): stessi tre elettori, e la partita giocata dopo resta identica a quella senza richieste. Test `elettori_campione_non_toccano_la_partita` nel Task 8.
5. **Refuso in un id dei dati** (regione, condizione, partito o indicatore che non esiste): rifiuto al caricamento col nome sbagliato nel messaggio. Test `gruppo_di_regione_sconosciuta_rifiutato_col_nome`, `interessi_con_indicatore_sconosciuto_rifiutati_col_nome`, `partito_sconosciuto_nel_governo_rifiutato_col_nome` nel Task 2.

## Dispatch

PR A: tutto in sequenza, 1 → 2 → 3 → 4 → 5 → 6 → 7 → 8. Il Task 2 non parte prima del commit del Task 1, perché i test comuni includono `dati/base/elettorato.toml` con `include_str!`. Dal 2 all'8 ogni task tocca `crates/sim-core/src/partita.rs` (`nuova`, `chiudi_turno`, `carica` o `vista`). I task 2-6 toccano anche `elettorato/mod.rs` (campi di `Gruppo`, `controlla`) e `tests/elettorato.rs`, il 7 e l'8 `vista.rs` e `tests/sondaggi.rs`. Ognuno consuma i campi del precedente, nell'ordine reddito → composizione → giudizio → voto → sondaggio → elettori campione. L'unica parte disgiunta è l'estrattore estemporaneo del Task 8 (`caso.rs`, `tests/caso.rs`): resta nel Task 8, perché spezzarlo costerebbe un passaggio di consegne per una sola funzione.

PR B: dopo la review della PR A, sul branch impilato. 9 → 10 in sequenza, perché toccano lo stesso `elettorato.py` e rigenerano lo stesso `elettorato.toml`. Il Task 11 va in parallelo col 9 → 10: crea `stima_voto.py` e scrive solo due manopole, e con loro condivide soltanto `strumenti/taratura/README.md`. Se girano nello stesso worktree, sezione del README, `cargo test --workspace` e commit si fanno dopo il commit in corso dell'altro, perché due commit concorrenti si contendono `index.lock`. Il passo 1 del Task 12 (`researcher`, che scrive solo in `docs/ricerca/`) parte insieme al Task 9. I passi 2-5 vengono dopo 9, 10 e 11: `tara` aggiusta le costanti sopra quote, istogrammi, β e fedeltà definitivi.

Nessuna dipendenza nuova, quindi `Cargo.lock` non cambia. `manopole.toml` lo scrivono il Task 2 (manopole nuove) e il Task 11 (due valori), `scenario.toml` il Task 2 e il Task 12, sempre in PR diverse. Ogni brief nomina `skill-gate`, `caveman` e, per chi scrive codice (`rust-engineer`, `coder`, `data-scientist`), `ponytail`; copia la riga **Ricerca** del task come riferimento. Ogni riga **Dispatch** sotto dà subagente, dipendenza e skill-gate. Il round pre-commit di fine PR aggiunge `rust-reviewer` (ci sono diff `.rs` in entrambe le PR) e `spec-reviewer` (il piano esiste).

---

## PR A · Motore dell'elettorato e sondaggi (Task 1-8)

### Task 1: Dati provvisori dell'elettorato

Scrive `dati/base/elettorato.toml` con uno script deterministico che legge solo dati già nel repo e costanti citate. È lo scheletro che la PR B riempie con le fonti vere: le funzioni restano, cambiano i corpi.

**Files:**
- Create: `strumenti/taratura/elettorato.py`
- Create: `dati/base/elettorato.toml` (generato)
- Modify: `strumenti/taratura/README.md` (sezione «Elettorato (provvisorio)»)

**Ricerca:** `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:44-45` (occupati dipendenti e indipendenti, quinti per condizione), `:66` (fasce 3-4-3 sui decili), `:91` (taglio della scala 0-10 sui 7 punti); `docs/ricerca/dati/risultati-elettorali-italia.md:13-27` (elettori 2022 e affluenza per regione).

**Dispatch:** `coder` · primo della PR A, nessuna dipendenza · skill-gate: true. Script Python offline che legge i TOML del repo e scrive un file, nessun `.rs`: è una pipeline, non un'analisi che decide se un numero regge, quindi non `data-scientist`.

**Interfaces:**
- Consumes: `dati/base/regioni.toml` (`id`, `macroarea`, `forze_lavoro`, `disoccupazione_2025`), `dati/base/elezioni.toml` (`popolazione` e `regione` delle circoscrizioni della Camera), `dati/base/scenario.toml` (`pensionati`, `spesa_pensioni`, `spesa_sussidi`), `dati/base/economia.toml` (`consumi`, `settori`). Lettura con `tomllib`.
- Produces: `dati/base/elettorato.toml` nel formato sotto, che il Task 2 carica. Funzioni Python `elettori()`, `quote()`, `istogrammi()`, `redditi()`, `panieri()`, `astensione()`, ciascuna con un corpo provvisorio che la PR B sostituisce.

Formato del file (ordine delle sezioni libero, gruppi in ordine canonico):

```toml
# Generato da `python3 -I strumenti/taratura/elettorato.py` il <data>. Non modificare a mano.
# Valori provvisori del piano 4: vedi strumenti/taratura/README.md.

aliquote = { bassa = 0.15, media = 0.25, alta = 0.33 }
astensione_condizione = { dipendente_privato = 0.0, dipendente_pubblico = 0.0, autonomo = 0.0, pensionato = 0.0, disoccupato_inattivo = 0.0, studente = 0.0 }
astensione_fascia = { bassa = 0.3, media = 0.0, alta = -0.2 }
nomi = ["Giulia", "Marco", ...]
cognomi = ["Rossi", "Esposito", ...]

[paniere]
bassa = [10 quote]
media = [10 quote]
alta = [10 quote]

[interessi.dipendente_privato]
reddito = 0.45
disoccupazione = 0.25
qualita_trasporti = 0.10
qualita_servizio_sanitario = 0.10
sicurezza = 0.10
# ... una sezione per condizione, righe del ticket 17

[[istituto]]
nome = "Istituto A"
campione = 1000

[[regione]]
id = "piemonte"
elettori = 3412345
astensione = 0.24

[[gruppo]]
regione = "piemonte"
condizione = "dipendente_privato"
fascia = "bassa"
quota = 0.071234
posizione = [0.05, 0.12, 0.17, 0.25, 0.16, 0.17, 0.08]
primario = 13200.0
trasferimenti = 0.0
```

- [ ] **Step 1: Scrivi `--selftest` con le invarianti, prima del resto**

`python3 -I strumenti/taratura/elettorato.py --selftest` costruisce tutti i valori senza scrivere e controlla con `assert`: 20 `[[regione]]` con `elettori > 0`; 360 gruppi in ordine canonico; per ogni regione le 18 quote ≥ 0 sommano a 1 entro 1e-9 dopo l'arrotondamento a 6 decimali (correzione sull'ultima voce, come `economia.py`); ogni istogramma ha 7 voci ≥ 0 che sommano a 1; `primario + trasferimenti > 0` in ogni gruppo; ogni riga degli interessi somma a 1; i panieri hanno 10 quote che sommano a 1; la somma degli elettori è 46.350.000 ± 20. Senza `--selftest` lo script scrive il file solo se le stesse `assert` passano.

- [ ] **Step 2: Esegui il selftest e verifica che fallisca**

Run: `python3 -I strumenti/taratura/elettorato.py --selftest`
Expected: FAIL (funzioni non ancora scritte).

- [ ] **Step 3: Implementa le funzioni con queste regole provvisorie**

Condizioni, fasce e chiavi come in `Condizione`/`Fascia` del Task 2 (`dipendente_privato`, `dipendente_pubblico`, `autonomo`, `pensionato`, `disoccupato_inattivo`, `studente`; `bassa`, `media`, `alta`).

- `elettori()`: `E_r = round(46_350_000 · pop_r / Σ pop)`, `pop_r` = somma della `popolazione` delle circoscrizioni della Camera con quella `regione`; correzione dell'arrotondamento sull'ultima regione [assunzione: elettori proporzionali alla popolazione censita; 46,35 milioni ≈ 46.253.014 della Camera 2022 (`risultati-elettorali-italia.md:13`) più la Valle d'Aosta].
- `quote()`, persone per regione: `occ = FL·1000·(1 − u/100)`; dipendenti `occ·18,93/24,11`, autonomi `occ·5,18/24,11` (`dati-per-tarare-i-gruppi.md:45`); dipendenti pubblici `0,17` dei dipendenti [assunzione: RGS non aperto, `:51`], privati il resto; disoccupati `FL·1000·u/100`; pensionati `E_r · pensionati / 46_350_000` (scenario); studenti `0,06·E_r` [assunzione]; `disoccupato_inattivo` = disoccupati + il residuo `E_r − tutti gli altri` (`assert` residuo ≥ 0). Fasce per condizione, (bassa, media, alta) [assunzione dai quinti di `:44` e dalle fasce 3-4-3 di `:66`]: dipendente privato (0,25, 0,45, 0,30); pubblico (0,15, 0,45, 0,40); autonomo (0,30, 0,35, 0,35); pensionato (0,30, 0,42, 0,28); disoccupato o inattivo (0,60, 0,32, 0,08); studente (0,40, 0,40, 0,20); la quota bassa × 1,3 nel Mezzogiorno e × 0,85 al Nord, poi rinormalizzate. Quota del gruppo = persone del gruppo / `E_r`.
- `istogrammi()`: lo stesso istogramma per tutti, `[0.05, 0.12, 0.17, 0.25, 0.16, 0.17, 0.08]` [segnaposto: la PR B lo ricava da ESS].
- `redditi()`, euro annui per elettore, fascia media e Centro: `primario` dipendente privato 24.000, pubblico 30.000, autonomo 28.000, pensionato 0, disoccupato o inattivo 4.000, studente 3.000; `trasferimenti` del pensionato = `spesa_pensioni·1e9 / pensionati`, del disoccupato o inattivo = `spesa_sussidi·1e9 /` persone di quella condizione in Italia, 0 altrove. Moltiplicatori di fascia 0,55 / 1 / 1,9 su entrambi; di macroarea Nord 1,1, Centro 1, Mezzogiorno 0,8 sul solo `primario` [assunzione, tutto].
- `panieri()`: i tre panieri uguali al vettore `consumi` di `economia.toml` [segnaposto: la PR B li differenzia].
- `astensione()`: `ln((1 − a_r)/a_r) + 0,57`, con `a_r` l'affluenza 2022 della tabella `risultati-elettorali-italia.md:16-27` scritta come costante, Valle d'Aosta = 0,636 nazionale; 0,57 = `ln Σ_j e^{−(4−j)²}` per un elettore al centro con β = 1 [assunzione: la PR B la tara col logit vero].
- `aliquote`, `astensione_condizione`, `astensione_fascia` come nel formato sopra; quattro istituti `Istituto A`-`D` con campioni 1000, 800, 1200, 1500 [segnaposto]; dodici nomi e dodici cognomi italiani comuni [segnaposto]; interessi = tabella del ticket 17 (`17-tabella-degli-interessi.md:28-35`) con le chiavi `reddito`, `disoccupazione`, `occupazione_pubblica` e gli `indicatore_servizio` di `dicasteri.toml` (trasporti → `qualita_trasporti`, sanità e liste d'attesa → `qualita_servizio_sanitario`, scuola → `qualita_scuola`, sicurezza → `sicurezza`, durata dei processi → `durata_processi`, sostegno alle imprese → `sostegno_imprese`, copertura del welfare → `copertura_welfare`). I pesi degli interessi si scrivono con `f"{w:.2f}"`: il test del Task 2 cerca la stringa `qualita_trasporti = 0.10`.

- [ ] **Step 4: Esegui selftest e generazione**

Run: `python3 -I strumenti/taratura/elettorato.py --selftest && python3 -I strumenti/taratura/elettorato.py`
Expected: PASS, `dati/base/elettorato.toml` scritto con 20 regioni e 360 gruppi. Una seconda esecuzione lo riscrive identico salvo la data.

- [ ] **Step 5: README e commit**

Sezione «Elettorato (provvisorio)» in `strumenti/taratura/README.md`: comando, file scritto, l'elenco delle regole sopra marcate assunzione o segnaposto, e che la PR B le sostituisce.

```bash
git add strumenti/taratura/elettorato.py strumenti/taratura/README.md dati/base/elettorato.toml
git commit -m "feat(dati): dati provvisori dei 360 gruppi"
```

### Task 2: Gruppi, partiti e governo uscente nel core

**Files:**
- Create: `crates/sim-core/src/elettorato/mod.rs`
- Create: `crates/sim-core/src/partiti.rs`
- Modify: `crates/sim-core/src/dati.rs:10-45` (settimo file), `:47-101` (manopole richieste), `:156-164` e `:275-346` (campo e validazione)
- Modify: `crates/sim-core/src/scenario.rs:48-69` (campi non numerici dello scenario)
- Modify: `crates/sim-core/src/errore.rs` (due varianti)
- Modify: `crates/sim-core/src/partita.rs:10` (schema 4), `:55-75` (campi), `:135-191` (controlli di `carica`)
- Modify: `crates/sim-core/src/lib.rs`
- Modify: `crates/sim-core/tests/comune/mod.rs:7-16` e `:86-96` (file `elettorato`)
- Modify: `dati/base/scenario.toml`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/elettorato.rs` (nuovo), `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:12-14` (gruppi e dati); `docs/ricerca/modelli/voto-spaziale.md:25` (ordine di grandezza di β), `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:104` (β su 7 punti = β₁₀/0,36); `docs/ricerca/modelli/voto-economico-e-retrospettivo.md:13` (ρ ≈ 1/3); `docs/ricerca/modelli/sondaggi-ed-errore.md:38-41` (errori dei sondaggi).

**Dispatch:** `rust-engineer` · dopo il Task 1: i test comuni includono `elettorato.toml` · skill-gate: true. Tocca `dati.rs`, `scenario.rs`, `errore.rs`, `partita.rs`, `lib.rs`, i test comuni, `scenario.toml` e `manopole.toml`; crea `elettorato/mod.rs` e `partiti.rs`, che i task 3-8 riaprono.

**Interfaces:**
- Consumes: `dati/base/elettorato.toml` (Task 1); `DatiGioco`, `Errore`, `Ingresso` esistenti.
- Produces:
  - `sim_core::elettorato::Condizione` — `enum { DipendentePrivato, DipendentePubblico, Autonomo, Pensionato, DisoccupatoInattivo, Studente }`, `serde(rename_all = "snake_case")`, `Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize`, con `pub const TUTTE: [Condizione; 6]` in quest'ordine. `Fascia` — `enum { Bassa, Media, Alta }`, stessi derive, `TUTTE: [Fascia; 3]`. `pub const PUNTI: usize = 7`.
  - `DatiElettorato { pub regioni: Vec<ElettoriRegione>, pub gruppi: Vec<DatoGruppo>, pub interessi: BTreeMap<Condizione, BTreeMap<String, f64>>, pub aliquote: BTreeMap<Fascia, f64>, pub paniere: BTreeMap<Fascia, Vec<f64>>, pub astensione_condizione: BTreeMap<Condizione, f64>, pub astensione_fascia: BTreeMap<Fascia, f64>, pub istituti: Vec<Istituto>, pub nomi: Vec<String>, pub cognomi: Vec<String> }` (nel TOML `regione`, `gruppo`, `istituto` sono gli array di tabelle). `ElettoriRegione { pub id: String, pub elettori: u64, pub astensione: f64 }`. `DatoGruppo { pub regione: String, pub condizione: Condizione, pub fascia: Fascia, pub quota: f64, pub posizione: [f64; 7], pub primario: f64, pub trasferimenti: f64 }`. `Istituto { pub nome: String, pub campione: u32 }`.
  - `pub fn indice_gruppo(regione: usize, c: Condizione, f: Fascia) -> usize` = `regione·18 + c·3 + f`; `Elettorato::elettori(&self, dati: &DatiElettorato, i: usize) -> f64` = quota viva del gruppo · `elettori` della sua regione (la quota dei dati vale solo al turno zero).
  - `DatiGioco::elettorato(&self) -> &DatiElettorato`.
  - Scenario: `pub partiti: Vec<DatoPartito>`, `pub governo: DatoGoverno`, `pub coalizioni: Vec<Vec<String>>`. `DatoPartito { pub id: String, pub nome: String, pub posizione: f64, pub valence: Ingresso }`; `DatoGoverno { pub presidente: String, pub dicasteri: BTreeMap<String, String> }` (dicastero → partito). Tutti in `partiti.rs`.
  - Stato: `Partito { pub id: String, pub nome: String, pub posizione_dichiarata: f64, pub posizione_percepita: f64, pub valence: f64 }`; `Governo { pub presidente: usize, pub dicasteri: Vec<usize> }` (indici nei partiti, `dicasteri` allineato a `DatiGioco::dicasteri()`), con `Governo::al_governo(&self, p: usize) -> bool` (presidente o titolare di un dicastero) e `Governo::partito_del_ruolo(&self, dati: &DatiGioco, r: Ruolo) -> usize`. `partiti::iniziali(dati: &DatiGioco) -> (Vec<Partito>, Governo)`.
  - `Elettorato { pub gruppi: Vec<Gruppo> }`, `Gruppo { pub quota: f64, pub posizione: [f64; 7], pub ancora: [f64; 7], pub giudizio: Vec<f64> }` (giudizio lungo quanto i partiti). `Elettorato::iniziale(dati: &DatiGioco, n_partiti: usize) -> Elettorato`: quote e istogrammi dai dati, `ancora = posizione`, giudizio a zero. `Elettorato::controlla(&self, dati: &DatiGioco, n_partiti: usize) -> Result<(), Errore>`: lunghezze e finitezza, `SalvataggioIlleggibile` col campo nel messaggio; i task successivi lo estendono coi loro campi.
  - `Partita` con i campi `partiti: Vec<Partito>`, `governo: Governo`, `elettorato: Elettorato`; accessori di stato vero per CLI e test: `Partita::partiti(&self) -> &[Partito]`, `Partita::governo(&self) -> &Governo`, `Partita::elettorato(&self) -> &Elettorato`.
  - `Errore::Elettorato(String)` («dati dell'elettorato non validi: …») e `Errore::Partiti(String)` («partiti dello scenario non validi: …»).
  - `FileDati.elettorato` e `"elettorato.toml"` in coda a `FileDati::NOMI` (7 voci).

- [ ] **Step 1: Scrivi i test che falliscono**

In `crates/sim-core/tests/elettorato.rs`, con gli helper di `comune` (aggiungi `"elettorato"` a `testo_di` e il file a `file_base`):

```rust
#[test]
fn dati_base_hanno_360_gruppi_in_ordine() {
    let d = dati_base();
    let e = d.elettorato();
    assert_eq!(e.gruppi.len(), 360);
    for (r, reg) in d.regioni().iter().enumerate() {
        let somma: f64 = (0..18).map(|k| e.gruppi[r * 18 + k].quota).sum();
        assert!((somma - 1.0).abs() < 1e-6, "{}", reg.id);
        assert_eq!(e.gruppi[r * 18].regione, reg.id);
    }
    assert_eq!(e.gruppi[indice_gruppo(0, Condizione::Pensionato, Fascia::Alta)].fascia, Fascia::Alta);
}

#[test]
fn gruppo_di_regione_sconosciuta_rifiutato_col_nome() {
    let f = imposta(file_base(), "elettorato", "regione = \"", "molisse\"");
    let Err(Errore::Elettorato(m)) = DatiGioco::da_testi(&f) else { panic!() };
    assert!(m.contains("molisse"), "{m}");
}

#[test]
fn istogramma_che_non_somma_a_uno_rifiutato_col_gruppo() {
    let f = imposta(file_base(), "elettorato", "posizione = [", "0.9");
    let Err(Errore::Elettorato(m)) = DatiGioco::da_testi(&f) else { panic!() };
    assert!(m.contains("dipendente_privato") && m.contains("bassa"), "{m}");
}

#[test]
fn interessi_con_indicatore_sconosciuto_rifiutati_col_nome() {
    let f = sostituisci(file_base(), "elettorato", "qualita_trasporti = 0.10", "trasporto = 0.10");
    let Err(Errore::Elettorato(m)) = DatiGioco::da_testi(&f) else { panic!() };
    assert!(m.contains("trasporto"), "{m}");
}

#[test]
fn pesi_degli_interessi_che_non_sommano_a_uno_rifiutati() {
    let f = sostituisci(file_base(), "elettorato", "disoccupazione = 0.25", "disoccupazione = 0.35");
    let Err(Errore::Elettorato(m)) = DatiGioco::da_testi(&f) else { panic!() };
    assert!(m.contains("dipendente_privato"), "{m}");
}

#[test]
fn partito_sconosciuto_nel_governo_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "scenario", "presidente = \"destra\"", "presidente = \"destrra\"");
    let Err(Errore::Partiti(m)) = DatiGioco::da_testi(&f) else { panic!() };
    assert!(m.contains("destrra"), "{m}");
}

#[test]
fn dicastero_senza_partito_rifiutato_col_nome() { /* togli `sanita = "destra"` da [governo.dicasteri]; Partiti(m) con "sanita" */ }

#[test]
fn partito_in_due_coalizioni_rifiutato_col_nome() { /* aggiungi "destra" alla prima coalizione; Partiti(m) con "destra" */ }

#[test]
fn valence_fuori_intervallo_rifiutata_col_nome() {
    // valore = 9.0 nella valence di "centro": ScenarioFuoriIntervallo { nome: "valence di centro", .. }
}

#[test]
fn partita_nuova_ha_il_governo_uscente() {
    let p = Partita::nuova(dati_base(), 1);
    let pres = &p.partiti()[p.governo().presidente];
    assert_eq!(pres.id, "destra");
    assert_eq!(p.partiti().len(), 7);
    assert!(p.elettorato().gruppi.iter().all(|g| g.giudizio.iter().all(|&x| x == 0.0)));
}
```

In `crates/sim-core/tests/partita.rs`, accanto ai test di salvataggio esistenti e col loro helper `salvataggio_modificato`:

```rust
#[test]
fn salvataggio_con_un_gruppo_in_meno_illeggibile() {
    let s = salvataggio_modificato(|j| { j["elettorato"]["gruppi"].as_array_mut().unwrap().pop(); });
    assert!(matches!(Partita::carica(&s), Err(Errore::SalvataggioIlleggibile(_))));
}

#[test]
fn salvataggio_con_giudizio_corto_illeggibile() {
    let s = salvataggio_modificato(|j| { j["elettorato"]["gruppi"][5]["giudizio"].as_array_mut().unwrap().pop(); });
    assert!(matches!(Partita::carica(&s), Err(Errore::SalvataggioIlleggibile(_))));
}

#[test]
fn salvataggio_con_presidente_fuori_dai_partiti_illeggibile() {
    let s = salvataggio_modificato(|j| j["governo"]["presidente"] = 99.into());
    assert!(matches!(Partita::carica(&s), Err(Errore::SalvataggioIlleggibile(_))));
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test elettorato --test partita`
Expected: FAIL (tipi e campi inesistenti).

- [ ] **Step 3: Dati dello scenario e manopole**

In `dati/base/scenario.toml`, dopo `legge_elettorale`:

```toml
coalizioni = [["sinistra", "centrosinistra"], ["centrodestra", "destra"]]

[governo]
presidente = "destra"
dicasteri = { economia_finanze = "destra", sviluppo_economico = "destra", lavoro_welfare = "destra", sanita = "destra", istruzione_ricerca = "destra", infrastrutture_trasporti = "destra", ambiente_energia = "centrodestra", interno = "destra", giustizia = "destra", esteri_difesa = "centrodestra" }
```

e sette `[[partito]]` con id `estrema_sinistra`, `sinistra`, `centrosinistra`, `centro`, `centrodestra`, `destra`, `estrema_destra`, `posizione` 1.0 … 7.0, `nome` = l'id in parole con la maiuscola («Estrema sinistra») come segnaposto, `[partito.valence]` con `valore = 0.0`, `min = -3.0`, `max = 3.0`, `fonte = "assunzione del piano 4: tarata nella PR B (Task 12)"`. Commento sopra `[governo]`: governo uscente di centrodestra e destra (spec riga 277); partito per dicastero dai ministri del 2026 ricondotti ai posti [assunzione del piano 4; la corrispondenza fra partiti veri e posti resta fuori dai dati, spec riga 284]. Coalizioni del 2022 ricondotte ai posti, stessa assunzione.

Manopole nuove in `dati/base/manopole.toml`, tutte richieste in `MANOPOLE_RICHIESTE`:

| Manopola | valore | min | max | ticket | fonte nel commento |
|---|---|---|---|---|---|
| `beta_distanza` | 1.0 | 0.3 | 3.0 | 14 | voto-spaziale:25 e dati-per-tarare-i-gruppi:104 |
| `fedelta` | 1.5 | 0.0 | 4.0 | 14 | assunzione |
| `peso_giudizio` | 0.3 | 0.0 | 2.0 | 29 | assunzione |
| `quota_colpa_presidente` | 0.3 | 0.0 | 1.0 | 17 | assunzione |
| `asimmetria_giudizio` | 0.333 | 0.1 | 1.0 | 17 | voto-economico-e-retrospettivo:13 |
| `emivita_giudizio` | 4.0 | 1.0 | 12.0 | 17 | ticket 17 |
| `finestra_aspettative` | 8.0 | 4.0 | 16.0 | 29 | ticket 29 |
| `scala_reddito` | 0.8 | 0.2 | 3.0 | 17 | assunzione (punti di crescita reale trimestrale) |
| `scala_disoccupazione` | 0.3 | 0.1 | 1.0 | 17 | assunzione (punti di disoccupazione al trimestre) |
| `ancora_posizioni` | 0.1 | 0.0 | 1.0 | 06 | opinion-dynamics:38 (nessun valore generale) |
| `rumore_posizioni` | 0.02 | 0.0 | 0.2 | 06 | assunzione |
| `mobilita_fasce` | 0.5 | 0.0 | 2.0 | 06 | assunzione |
| `uscita_studenti` | 0.2 | 0.0 | 0.5 | 06 | assunzione (quota annua) |
| `sigma_bias_comune` | 0.015 | 0.0 | 0.04 | 25 | sondaggi-ed-errore:39 |
| `sigma_istituto` | 0.01 | 0.0 | 0.03 | 25 | sondaggi-ed-errore:41 |
| `effetto_disegno` | 2.0 | 1.0 | 4.0 | 25 | sondaggi-ed-errore:18, :38 |
| `errore_approvazione` | 1.0 | 0.0 | 3.0 | 25 | assunzione |
| `errore_stima_pil` | 1.0 | 0.0 | 3.0 | 25 | ticket 25:33 (circa 1 punto annuo) |
| `errore_stima_disoccupazione` | 0.2 | 0.0 | 1.0 | 25 | assunzione |
| `errore_stima_inflazione` | 0.2 | 0.0 | 1.0 | 25 | assunzione |
| `errore_stima_disavanzo` | 0.3 | 0.0 | 1.0 | 25 | assunzione |

- [ ] **Step 4: Tipi, caricamento, validazione, stato**

`DatiElettorato::valida(&self, regioni: &[Regione], dicasteri: &[Dicastero], n_settori: usize) -> Result<(), Errore>`, chiamata da `DatiGioco::valida` dopo la struttura; ogni rifiuto è `Errore::Elettorato` col nome del colpevole:
- una `[[regione]]` per ogni regione di `regioni.toml`, nello stesso ordine, `elettori > 0`, `astensione` finita;
- 360 gruppi in ordine canonico: il gruppo `i` ha la regione, la condizione e la fascia dell'indice `i` (messaggio con regione, condizione e fascia attese e trovate);
- `quota` finita ≥ 0 e somma per regione 1 ± 1e-6; `posizione` finita ≥ 0 e somma 1 ± 1e-6; `primario`, `trasferimenti` finiti ≥ 0 con somma > 0;
- `interessi`: tutte le 6 condizioni, pesi finiti ≥ 0 che sommano a 1 ± 1e-6, chiavi in `{reddito, disoccupazione, occupazione_pubblica}` ∪ gli `indicatore_servizio` dei dicasteri;
- `aliquote` per le 3 fasce in `[0, 0.9]`; `paniere` per le 3 fasce con `n_settori` quote ≥ 0 che sommano a 1 ± 1e-6; `astensione_condizione` e `astensione_fascia` complete e finite;
- da 3 a 4 istituti (spec riga 369) con `campione ≥ 100`; `nomi` e `cognomi` non vuoti.

`partiti::valida(scenario: &Scenario, dicasteri: &[Dicastero]) -> Result<(), Errore>` (`Errore::Partiti`): da 2 a 10 partiti con id distinti, `posizione` finita in `[1, 7]`; `presidente` è un partito; `governo.dicasteri` ha esattamente gli id dei dicasteri, ciascuno con un partito esistente; ogni partito delle coalizioni esiste e sta in una coalizione sola. La valence passa da `Scenario::valida` come ingresso, col nome `valence di <id>`.

`Partita::nuova` costruisce `partiti::iniziali` e `Elettorato::iniziale` dopo `Economia::iniziale`. `Partita::carica` aggiunge dopo i controlli dell'economia: `partiti.len()` uguale a quello dello scenario, `governo.presidente` e ogni `governo.dicasteri[i]` minori di `partiti.len()`, `governo.dicasteri.len()` uguale ai dicasteri, `posizione_percepita` e `valence` finite, poi `elettorato.controlla(dati, partiti.len())`. `VERSIONE_SCHEMA = 4`.

- [ ] **Step 5: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS, compresi i test esistenti di partita, CLI e guscio (il settimo file arriva in `app` con la cartella `dati/base/`, `tauri.conf.json:34`).

- [ ] **Step 6: Commit**

```bash
git add crates/sim-core dati/base/scenario.toml dati/base/manopole.toml
git commit -m "feat(core): gruppi, partiti e governo uscente"
```

### Task 3: Redditi dei gruppi, panieri e Gini

**Files:**
- Create: `crates/sim-core/src/elettorato/redditi.rs`
- Modify: `crates/sim-core/src/elettorato/mod.rs` (campi di `Gruppo` ed `Elettorato`, `controlla`)
- Modify: `crates/sim-core/src/economia/previdenza.rs:58-65` (salario medio estratto in una funzione)
- Modify: `crates/sim-core/src/economia/mod.rs:184-186` (commento sull'ordine)
- Modify: `crates/sim-core/src/partita.rs:65-84`
- Test: `crates/sim-core/tests/elettorato.rs`

**Ricerca:** spec riga 298 (reddito disponibile, panieri, Gini); `docs/ricerca/modelli/voto-economico-e-retrospettivo.md:40` (economia egotropica per gruppi se il motore produce redditi per gruppo); `docs/ricerca/modelli/mercato-del-lavoro-e-disuguaglianza.md:19` (Gini da gruppi discreti) e `:44` (Gini derivato, non uno stato); ticket 29 (`29-costo-del-governare.md:48`, erosione nominale).

**Dispatch:** `rust-engineer` · dopo il Task 2: `Gruppo`, `Elettorato`, `partita.rs` e `tests/elettorato.rs` del 2 · skill-gate: true. Tocca anche `economia/previdenza.rs` e `economia/mod.rs` (solo il commento d'ordine), che nessun altro task del piano riapre.

**Interfaces:**
- Consumes: `Economia` (`pil_trimestrale`, `livello_prezzi`, `prezzi_settori`, `bilancio`, `conti`, `pensioni`, `sussidio_per_disoccupato`), `DatiElettorato` (Task 2).
- Produces:
  - `pub(crate) fn Economia::salario_medio(&self, dati: &DatiGioco) -> f64` (euro annui per occupato, la formula di `previdenza.rs:63-65`, che la riusa), `pensione_media(&self) -> f64` (spesa pensionistica annua / pensionati), `sussidio_effettivo(&self, dati) -> f64` (`sussidio_per_disoccupato · k` del regime, come `previdenza.rs:95-99`).
  - `Reddito { pub primario: f64, pub trasferimenti: f64, pub imposte: f64, pub paniere: f64 }` (euro annui nominali per elettore; `paniere` indice di prezzo della fascia, 1 al turno zero), `Reddito::netto(&self) -> f64` = primario + trasferimenti − imposte, `Reddito::reale(&self) -> f64` = netto / paniere.
  - `Gruppo` acquista `pub reddito: Reddito`, `pub reddito_prima: Reddito`; `Elettorato` acquista `pub base: Indici` (gli indici del turno zero).
  - `Indici { pub salario: f64, pub salari_pubblici: f64, pub pil: f64, pub pensione: f64, pub sussidio: f64, pub pressione: f64 }` e `Indici::da(e: &Economia, dati: &DatiGioco) -> Indici`.
  - `Elettorato::aggiorna_redditi(&mut self, dati: &DatiGioco, e: &Economia)`: `reddito_prima ← reddito`, poi il reddito nuovo.
  - `Elettorato::gini(&self, dati: &DatiGioco) -> f64`, sul reddito reale pesato con gli elettori del gruppo.

Regole, con `I` gli indici di oggi e `B = base`:
- `salario` = `salario_medio`; `salari_pubblici` = somma dei `salari` del bilancio in corso; `pil` = `pil_trimestrale`; `pensione` = `pensione_media`; `sussidio` = `sussidio_effettivo`; `pressione` = `(irpef + contributi) / (pil_trimestrale·1e9)` dei conti del trimestre.
- `primario` = `primario₀ · I.x / B.x` con `x` = `salari_pubblici` per il dipendente pubblico, `pil` per l'autonomo, `salario` per tutti gli altri.
- `trasferimenti` = `trasferimenti₀ · I.pensione / B.pensione` per il pensionato, `· I.sussidio / B.sussidio` per il disoccupato o inattivo, `trasferimenti₀` altrove.
- `imposte` = `aliquota_fascia · (primario + trasferimenti) · I.pressione / B.pressione` (le aliquote a scaglioni arrivano col piano 5).
- `paniere` = `livello_prezzi · Σ_s w_fs p_s / Σ_s consumi_s p_s`, `p` = `prezzi_settori`, `w_f` = paniere della fascia.
- Gini pesato: `Σ_i Σ_j n_i n_j |y_i − y_j| / (2 N² ȳ)`, `n` = elettori del gruppo, `y` = reddito reale.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn redditi_al_turno_zero_dai_dati() {
    let d = dati_base();
    let p = Partita::nuova(d.clone(), 1);
    let i = indice_gruppo(3, Condizione::Autonomo, Fascia::Alta);
    let g = &d.elettorato().gruppi[i];
    let lordo = g.primario + g.trasferimenti;
    let atteso = lordo - d.elettorato().aliquote[&Fascia::Alta] * lordo;
    assert!((p.elettorato().gruppi[i].reddito.reale() - atteso).abs() < 1e-6);
}

#[test]
fn bilancio_fermo_erode_il_reddito_dei_dipendenti_pubblici() {
    // 4 turni senza shock, inflazione al 4,2%: salari pubblici fermi in nominale fino al bilancio di gennaio,
    // salari privati che seguono il PIL nominale.
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let (pubblico, privato) = (indice_gruppo(11, Condizione::DipendentePubblico, Fascia::Media), indice_gruppo(11, Condizione::DipendentePrivato, Fascia::Media));
    let prima = |p: &Partita, i: usize| p.elettorato().gruppi[i].reddito.reale();
    let (pu0, pr0) = (prima(&p, pubblico), prima(&p, privato));
    for _ in 0..4 { p.chiudi_turno(); }
    assert!(prima(&p, pubblico) / pu0 < prima(&p, privato) / pr0);
}

#[test]
fn gini_nullo_con_redditi_uguali() {
    let f = riscrivi(riscrivi(riscrivi(file_base(), "elettorato", "primario = ", "primario = 1000.0"),
        "elettorato", "trasferimenti = ", "trasferimenti = 0.0"),
        "elettorato", "aliquote = ", "aliquote = { bassa = 0.2, media = 0.2, alta = 0.2 }");
    let d = DatiGioco::da_testi(&f).unwrap();
    assert!(Partita::nuova(d.clone(), 1).elettorato().gini(&d).abs() < 1e-12);
    let base = dati_base();
    let g = Partita::nuova(base.clone(), 1).elettorato().gini(&base);
    assert!(g > 0.1 && g < 0.6, "{g}");
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test elettorato`
Expected: FAIL.

- [ ] **Step 3: Implementa `redditi.rs` e i tre metodi di `Economia`**

`Partita::nuova` chiama `Elettorato::iniziale`, poi `base = Indici::da(&economia, &dati)` e `aggiorna_redditi` due volte (così `reddito_prima == reddito` al turno zero). `chiudi_turno` chiama `aggiorna_redditi` subito dopo `economia.passo`. Il commento d'ordine di `Economia::passo` cita il blocco con la nota «chiamato da `Partita` subito dopo il passo: legge solo grandezze che BCE, SFC e spread non toccano». `controlla` esige redditi e indici finiti, `paniere > 0`.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(elettorato): redditi dei gruppi, panieri e Gini"
```

### Task 4: Composizione che si muove

**Files:**
- Create: `crates/sim-core/src/elettorato/composizione.rs`
- Modify: `crates/sim-core/src/elettorato/mod.rs` (`ultima_disoccupazione`, `controlla`)
- Modify: `crates/sim-core/src/partita.rs` (`chiudi_turno`)
- Test: `crates/sim-core/tests/elettorato.rs`

**Ricerca:** spec riga 307 (quote che si muovono con lavoro, reddito ed età; chi cambia gruppo porta con sé la sua distribuzione); ticket 06 (`06-elettorato.md:37`).

**Dispatch:** `rust-engineer` · dopo il Task 3: la crescita reale `γ_i` viene da `reddito`/`reddito_prima` del 3; `elettorato/mod.rs`, `partita.rs`, `tests/elettorato.rs` · skill-gate: true.

**Interfaces:**
- Consumes: `Gruppo` (Task 2-3), `Economia::regioni[].disoccupazione`, `Economia::pensioni` (annata dell'anno), manopole `mobilita_fasce`, `uscita_studenti`, `uscite_pensionati`.
- Produces:
  - `Elettorato` acquista `pub ultima_disoccupazione: Vec<f64>` (disoccupazione per regione letta al turno prima, dall'economia iniziale al turno zero).
  - `Elettorato::sposta(&mut self, da: usize, a: usize, massa: f64)`: massa fermata a `quota[da]`; `a` diventa la media pesata per massa di `posizione`, `ancora`, `giudizio` (e dei campi che i task successivi aggiungono: il Task 6 vi aggiunge `voto_precedente`); `quota[da] −= m`, `quota[a] += m`.
  - `Elettorato::rinnova(&mut self, da: usize, a: usize, massa: f64)`: sposta la quota senza mescolare (elettori nuovi al posto di chi esce).
  - `Elettorato::composizione(&mut self, dati: &DatiGioco, e: &Economia, data: Trimestre)`, che alla fine aggiorna `ultima_disoccupazione`.

Regole, in quest'ordine, per regione `r` (masse in quota della regione, ogni frazione fermata in `[0, 1]`):
1. **Lavoro.** `Δ = u_r − ultima_disoccupazione_r` (punti). Se `Δ > 0`: per ogni fascia, dipendente privato e autonomo spostano `Δ/100 · quota` ciascuno verso il disoccupato o inattivo della stessa fascia. Se `Δ < 0`: per ogni fascia il disoccupato o inattivo sposta `|Δ|/100 · (quota del dipendente privato + quota dell'autonomo)` verso il dipendente privato e l'autonomo della stessa fascia, ripartita in proporzione alle loro quote del turno zero (dati) [ruling del round pre-commit: flussi simmetrici, la prima stesura svuotava gli autonomi].
2. **Reddito.** Crescita reale del gruppo `γ_i = reale/reale_prima − 1`; per condizione `G_c` = media di `γ_i` pesata con gli elettori sui gruppi di `c` in tutta Italia, `G` la media su tutti. `x = mobilita_fasce · 100·|G_c − G|`, frazione `x/100`. Se `G_c < G`: media → bassa e alta → media, ciascuna `x/100` della propria quota (dalla quota d'inizio passo); se `G_c > G`: bassa → media e media → alta.
3. **Età, solo con `data.trimestre == 1`**, dopo che la previdenza ha aggiunto l'annata dell'anno: tasso di pensionamento `t = pensionati dell'annata con anno == data.anno / Σ elettori di dipendenti privati, pubblici e autonomi`; ogni gruppo di quelle tre condizioni sposta `t · quota` al pensionato della stessa fascia. Ogni pensionato `rinnova` `uscite_pensionati · quota` verso lo studente della stessa fascia (diciottenni che entrano). Ogni studente sposta `uscita_studenti · quota`: `(1 − u_r/100)` di essa al dipendente privato, al dipendente pubblico e all'autonomo in proporzione alle loro quote del turno zero (dati) nella stessa regione e fascia, `u_r/100` al disoccupato o inattivo, stessa fascia [ruling del round pre-commit: senza entrate i pubblici e gli autonomi si svuotavano a senso unico].

`chiudi_turno` chiama `composizione` dopo `aggiorna_redditi`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn quote_di_regione_sommano_a_uno_dopo_40_turni() {
    let mut p = Partita::nuova(dati_base(), 7);
    for _ in 0..40 { p.chiudi_turno(); }
    for r in 0..20 {
        let s: f64 = (0..18).map(|k| p.elettorato().gruppi[r * 18 + k].quota).sum();
        assert!((s - 1.0).abs() < 1e-9, "regione {r}: {s}");
    }
}

#[test]
fn la_disoccupazione_sposta_dipendenti_fra_i_disoccupati() {
    // Per ogni regione con |Δu| > 1e-9 nel primo turno, la quota dei disoccupati o inattivi
    // si muove nello stesso verso della disoccupazione.
}

#[test]
fn i_pensionati_cambiano_solo_a_gennaio() {
    let mut p = Partita::nuova(dati_base(), 3);
    let pensionati = |p: &Partita| -> f64 { /* Σ quota·elettori dei 60 gruppi di pensionati */ };
    let t0 = pensionati(&p);
    p.chiudi_turno(); // chiude il 2026T4
    assert!((pensionati(&p) - t0).abs() < 1e-6 * t0);
    p.chiudi_turno(); // chiude il 2027T1: annata nuova, uscite
    assert!((pensionati(&p) - t0).abs() > 1e-6 * t0);
}
```

E un test d'unità in `composizione.rs` (`#[cfg(test)]`) su un `Elettorato` costruito a mano: `sposta` di metà della quota da un gruppo con istogramma tutto sul punto 1 a uno vuoto con istogramma tutto sul punto 7 lascia il destinatario con l'istogramma del mittente e le quote che sommano come prima; `rinnova` lascia l'istogramma del destinatario com'era.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core composizione && cargo test -p sim-core --test elettorato`
Expected: FAIL.

- [ ] **Step 3: Implementa `composizione.rs`**

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(elettorato): composizione mobile dei gruppi"
```

### Task 5: Interessi, aspettative, giudizio e colpa

**Files:**
- Create: `crates/sim-core/src/elettorato/giudizio.rs`
- Modify: `crates/sim-core/src/elettorato/mod.rs` (`crescite`, `controlla`)
- Modify: `crates/sim-core/src/partita.rs` (`chiudi_turno`)
- Test: `crates/sim-core/tests/elettorato.rs`

**Ricerca:** `docs/ricerca/modelli/voto-economico-e-retrospettivo.md:13` (asimmetria del malcontento, `α_i = 2αρ/(1+ρ)`, `α_d = 2α/(1+ρ)`), `:18` e `:49` (costo del governare come bersaglio, niente deriva scritta); ticket 17 (`17-tabella-degli-interessi.md:20-24` e `:44-48`, misura in deviazioni standard, giudizio cumulato, chi paga); ticket 29 (`29-costo-del-governare.md:47`, aspettative).

**Dispatch:** `rust-engineer` · dopo il Task 4: legge `ultima_disoccupazione` del 4 e s'inserisce in `chiudi_turno` fra `aggiorna_redditi` e `composizione`; `elettorato/mod.rs`, `partita.rs`, `tests/elettorato.rs` · skill-gate: true.

**Interfaces:**
- Consumes: `Reddito`/`reddito_prima` (Task 3), `ultima_disoccupazione` (Task 4) letta prima che `composizione` la aggiorni: per questo `chiudi_turno` chiama `giudizio` prima di `composizione`; `Governo::al_governo`, `Governo::partito_del_ruolo` (Task 2).
- Produces:
  - `Gruppo` acquista `pub crescite: Vec<f64>`: le ultime crescite reali trimestrali in punti, la più recente in coda, al massimo `finestra_aspettative` arrotondata; vuota al turno zero [assunzione: aspettativa nulla finché non c'è storia]. `sposta` (Task 4) non mescola `crescite`: è la storia del reddito del gruppo, non delle persone che vi entrano.
  - `pub(crate) fn asimmetrica(x: f64, rho: f64) -> f64` = `x · 2ρ/(1+ρ)` se `x > 0`, `x · 2/(1+ρ)` altrimenti.
  - `Elettorato::giudizio(&mut self, dati: &DatiGioco, e: &Economia, partiti: &[Partito], governo: &Governo)`.

Regole per il gruppo `i` (regione `r`, condizione `c`), `σ_r = scala_reddito`, `σ_u = scala_disoccupazione`, `Y₀ = reddito_prima.netto()`:
- componenti in punti: `lav = 100·Δprimario/Y₀`, `tra = 100·Δtrasferimenti/Y₀`, `imp = −100·Δimposte/Y₀`, `prez = −100·(paniere/paniere_prima − 1)`; aspettativa `a` = media di `crescite` (0 se vuota); poi entra in coda `100·(reale/reale_prima − 1)`.
- interesse reddito, **una voce sola** (spec righe 309-311: il reddito disponibile reale contro la sua aspettativa): componenti `c = [lav, prez − a, tra, imp]` con responsabili `[Economia e Finanze se c è dipendente pubblico, altrimenti Presidente; Presidente; ruolo colpa_disoccupazione_trasferimenti; ruolo colpa_imposte_impiego_pubblico]` (spec riga 312); `Δ_y = Σ c / σ_r`; coefficiente `α = 2ρ/(1+ρ)` se `Δ_y > 0`, altrimenti `2/(1+ρ)`, scelto **dal segno del totale** e applicato a ogni componente, così `Σ_p` del contributo = `w_reddito · asimmetrica(Δ_y, ρ)` e un reddito reale fermo dà zero. Disoccupazione `(−(u_r − ultima_disoccupazione_r)/σ_u, w_disoccupazione, ruolo colpa_disoccupazione_trasferimenti)`, con la sua `asimmetrica`. Indicatori di servizio e occupazione pubblica non hanno voce finché il piano 5 non li porta: il loro peso resta nei dati e non si rinormalizza.
- per ogni partito `p` al governo: `g ← λ·g + w_reddito · α/σ_r · Σ_j c_j · colpa(resp_j, p) + w_disoccupazione · asimmetrica(Δ_u, ρ) · colpa(lavoro, p)`, `λ = 0,5^(1/emivita_giudizio)`, `colpa(resp, p) = q·[p = presidente] + (1 − q)·[p = resp]`, `q = quota_colpa_presidente`. Gli altri partiti restano a zero. [Ruling del round pre-commit: la prima stesura applicava `asimmetrica` a ogni componente nominale e puniva il governo a reddito reale fermo.]

- [ ] **Step 1: Scrivi i test che falliscono**

Test d'unità in `giudizio.rs`: `asimmetrica(1.0, 1.0/3.0) == 0.5`, `asimmetrica(-1.0, 1.0/3.0) == -1.5` (esempio degli autori, `voto-economico-e-retrospettivo.md:13`), e un miglioramento seguito da un peggioramento uguale lasciano `Σ < 0`.

In `tests/elettorato.rs`:

```rust
#[test]
fn solo_i_partiti_di_governo_hanno_un_giudizio() {
    let mut p = Partita::nuova(dati_base(), 5);
    for _ in 0..8 { p.chiudi_turno(); }
    let gov = p.governo().clone();
    for (j, _) in p.partiti().iter().enumerate().filter(|(j, _)| !gov.al_governo(*j)) {
        assert!(p.elettorato().gruppi.iter().all(|g| g.giudizio[j] == 0.0));
    }
    assert!(p.elettorato().gruppi.iter().any(|g| g.giudizio[gov.presidente] != 0.0));
}

#[test]
fn la_colpa_della_disoccupazione_va_al_partito_del_lavoro() {
    // lavoro_welfare = "centrodestra" nello scenario: dopo 6 turni con shock il centrodestra ha un giudizio ≠ 0
    // in qualche gruppo; con in più quota_colpa_presidente = 1.0 resta 0 ovunque.
}

#[test]
fn le_aspettative_ricordano_al_massimo_la_finestra() {
    let mut p = Partita::nuova(dati_base(), 2);
    for _ in 0..20 { p.chiudi_turno(); }
    assert!(p.elettorato().gruppi.iter().all(|g| g.crescite.len() == 8));
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core giudizio && cargo test -p sim-core --test elettorato`
Expected: FAIL.

- [ ] **Step 3: Implementa `giudizio.rs`**

`chiudi_turno` dopo il passo: `aggiorna_redditi` → `giudizio` → `composizione`. `controlla` esige `crescite` finite e non più lunghe di 16.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(elettorato): giudizio retrospettivo con aspettative e colpa"
```

### Task 6: Posizioni e voto logit con astensione

**Files:**
- Create: `crates/sim-core/src/elettorato/voto.rs`
- Modify: `crates/sim-core/src/elettorato/mod.rs` (`voto_precedente`, `controlla`), `composizione.rs` (`sposta` mescola anche il voto precedente)
- Modify: `crates/sim-core/src/partita.rs` (`nuova`, `chiudi_turno`, `intenzioni_vere`)
- Modify: `crates/sim-cli/src/main.rs:61` (stato vero dell'elettorato nell'uscita)
- Test: `crates/sim-core/tests/elettorato.rs`, `crates/sim-cli/tests/cli.rs`

**Ricerca:** `docs/ricerca/modelli/voto-spaziale.md:14` (logit con valence), `:41` (probabilità lisce); `docs/ricerca/modelli/opinion-dynamics.md:13` (Friedkin-Johnsen), `:27` e `:57` (niente attrazione fra gruppi, rumore); `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:117` (astensione per regione, condizione e fascia).

**Dispatch:** `rust-engineer` · dopo il Task 5: utilità col giudizio del 5, `sposta` di `composizione.rs` (Task 4) esteso a `voto_precedente`; `elettorato/mod.rs`, `partita.rs`, `tests/elettorato.rs`, più `crates/sim-cli` (`main.rs`, `tests/cli.rs`) · skill-gate: true.

**Interfaces:**
- Consumes: `Gruppo` (Task 2-5), `Partito`, `Governo`, `DatiElettorato` (astensione), manopole `beta_distanza`, `fedelta`, `peso_giudizio`, `ancora_posizioni`, `rumore_posizioni`; `Caso::normale(Flusso::Elettorato)`.
- Produces:
  - `Gruppo` acquista `pub voto_precedente: Vec<f64>` (lungo `partiti + 1`, l'ultima voce è l'astensione).
  - `Elettorato::posizioni(&mut self, dati: &DatiGioco, caso: &mut Caso)`.
  - `pub fn intenzioni(dati: &DatiGioco, el: &Elettorato, partiti: &[Partito], governo: &Governo) -> Vec<Vec<f64>>`: per gruppo, quote dell'elettorato del gruppo per partito più astensione, somma 1.
  - `Elettorato::avvia_voto(&mut self, dati, partiti, governo)`: il voto precedente iniziale.
  - `Partita::intenzioni_vere(&self) -> Intenzioni`, `Intenzioni { pub per_gruppo: Vec<Vec<f64>>, pub nazionali: Vec<f64> }` (`nazionali` pesate con gli elettori, somma 1), `Serialize`.

Regole:
- **Posizioni** (ancora di Friedkin-Johnsen senza influenza fra gruppi, spec riga 313): per ogni gruppo in ordine canonico `h ← (1 − a)·h + a·ancora`, `a = ancora_posizioni`; poi per ogni punto `h_x ← h_x · exp(σ·z)`, `z = caso.normale(Flusso::Elettorato)`, `σ = rumore_posizioni`, sette estrazioni per gruppo anche con `σ = 0`; rinormalizza.
- **Utilità** per il gruppo `i` (regione `r`, condizione `c`, fascia `f`) al punto `x ∈ 1..=7` (spec riga 308): partito `j`: `U_j = −β·(x − percepita_j)² + valence_j + γ·giudizio_ij` (il termine di giudizio è zero per chi non governa); astensione: `U_a = astensione_r + astensione_condizione_c + astensione_fascia_f`.
- **Fedeltà**: `T_x[m][k] = softmax_k(U + φ·[k = m])` (con lo shift del massimo), `P(k | i, x) = Σ_m voto_precedente_m · T_x[m][k]`; intenzione del gruppo `= Σ_x h_x · P(· | i, x)`.
- **Voto precedente iniziale**: distribuzione stazionaria di `T_i = Σ_x h_x T_x`: iterazione `π ← π T_i` da uniforme, fino a `max|Δ| < 1e-13` o 1000 giri. Così al turno zero l'intenzione coincide col voto precedente.
- Ordine di `chiudi_turno` dopo il passo: `aggiorna_redditi` → `giudizio` → `composizione` → `posizioni`. `Partita::nuova` chiama `avvia_voto` per ultimo.
- CLI: l'uscita JSON aggiunge `"elettorato": { "partiti": [id…], "nazionali": [...] }` dalle intenzioni vere.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn intenzioni_sommano_a_uno() {
    let p = Partita::nuova(dati_base(), 1);
    for v in &p.intenzioni_vere().per_gruppo {
        assert!((v.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    }
}

#[test]
fn al_turno_zero_l_intenzione_e_il_voto_precedente() {
    let p = Partita::nuova(dati_base(), 1);
    for (g, v) in p.elettorato().gruppi.iter().zip(&p.intenzioni_vere().per_gruppo) {
        for (a, b) in g.voto_precedente.iter().zip(v) { assert!((a - b).abs() < 1e-9); }
    }
}

#[test]
fn piu_valence_piu_voti() { /* valence di "centro" da 0.0 a 1.0: la sua quota nazionale sale, quella degli altri partiti scende */ }

#[test]
fn piu_astensione_in_una_regione_meno_affluenza() { /* astensione della prima [[regione]] +1.0: affluenza di quella regione più bassa, delle altre uguale */ }

#[test]
fn affluenza_del_turno_zero_plausibile() {
    let n = Partita::nuova(dati_base(), 1).intenzioni_vere().nazionali;
    let affluenza = 1.0 - n[n.len() - 1];
    assert!((0.5..0.8).contains(&affluenza), "{affluenza}");
}

#[test]
fn il_rumore_delle_posizioni_non_tocca_l_economia() {
    // Stesso seme, rumore_posizioni 0.0 contro 0.2: dopo 5 turni `economia()` identica (flussi separati, spec riga 251).
}

#[test]
fn elettorato_resta_valido_dopo_200_turni() {
    let mut p = Partita::nuova(dati_base(), 11);
    for _ in 0..200 { p.chiudi_turno(); }
    // quote per regione = 1, istogrammi ≥ 0 e somma 1, redditi reali finiti e > 0, intenzioni somma 1
    p.salva().expect("il salvataggio riesce");
}
```

Più `salvataggio_con_voto_precedente_corto_illeggibile` in `tests/partita.rs` e, in `tests/cli.rs`, che l'uscita contenga `elettorato.nazionali` lungo 8.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test --workspace`
Expected: FAIL.

- [ ] **Step 3: Implementa `voto.rs`**

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core crates/sim-cli
git commit -m "feat(elettorato): posizioni e voto logit con astensione"
```

### Task 7: Sondaggio pubblico, prime stime e vista

**Files:**
- Create: `crates/sim-core/src/informazione/mod.rs`, `crates/sim-core/src/informazione/sondaggio.rs`, `crates/sim-core/src/informazione/statistica.rs`
- Modify: `crates/sim-core/src/vista.rs`, `crates/sim-core/src/partita.rs` (campo `informazione`, `nuova`, `chiudi_turno`, `vista`, `carica`), `crates/sim-core/src/lib.rs`
- Test: `crates/sim-core/tests/sondaggi.rs` (nuovo), `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/modelli/sondaggi-ed-errore.md:14-19` (modello generativo a tre strati), `:23` e `:38-41` (valori), `:55` (la media degli istituti non toglie il bias comune); `docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:10` (revisione media della prima stima del PIL, poco più di 1 punto annuo); ticket 25 (`25-informazione-imperfetta.md:33-38`).

**Dispatch:** `rust-engineer` · dopo il Task 6: `intenzioni` e `voto_precedente` del 6; `partita.rs`, `vista.rs`, `lib.rs`; rigenera e committa i tipi in `app/src/lib/tipi` · skill-gate: true.

**Interfaces:**
- Consumes: `intenzioni` (Task 6), `DatiElettorato::istituti`, `Elettorato::elettori` (quota viva), `Economia`, manopole del sondaggio e delle stime (Task 2), `Caso::normale(Flusso::Sondaggi)`.
- Produces:
  - `Informazione { pub bias: Vec<f64>, pub effetti_istituto: Vec<Vec<f64>>, pub sondaggio: Sondaggio, pub statistica: Statistica }` (stato vero; `Partita::informazione(&self) -> &Informazione` per CLI e test).
  - Tipi pubblici, `Clone, Debug, PartialEq, Serialize, Deserialize` e `TS` come `Vista`: `Sondaggio { pub istituti: Vec<Rilevazione>, pub condizioni: Vec<Incrocio>, pub fasce: Vec<Incrocio>, pub regioni: Vec<Incrocio> }`; `Rilevazione { pub istituto: String, pub campione: u32, pub quote: Vec<f64> }`; `Incrocio { pub id: String, pub campione: u32, pub quote: Vec<f64>, pub posizione: Vec<f64>, pub approvazione: f64 }`. `quote` = partiti nell'ordine dello scenario più l'astensione in coda; `posizione` = autocollocazione stimata sui 7 punti; `approvazione` = giudizio stimato del sottocampione sul partito del Presidente del Consiglio, in unità d'utilità.
  - `Stima { pub crescita_pil: f64, pub disoccupazione: f64, pub inflazione: f64, pub disavanzo: f64 }` (percentuali; crescita e inflazione annualizzate, disavanzo in % del PIL del trimestre); `Statistica { pub trimestre: Trimestre, pub prima_stima: Stima, pub vera: Stima, pub revisione: Option<Revisione> }`; `Revisione { pub trimestre: Trimestre, pub prima_stima: Stima, pub riveduta: Stima }`.
  - `Vista` acquista `pub partiti: Vec<VocePartito>` (`id`, `nome`, `posizione_dichiarata`, `posizione_percepita`, `valence`, `al_governo: bool`), `pub sondaggio: Sondaggio`, `pub statistica: VistaStatistica { pub trimestre: Trimestre, pub prima_stima: Stima, pub revisione: Option<Revisione> }` (senza `vera`).

Regole:
- **Alla nascita** (dopo `avvia_voto`), da `Flusso::Sondaggi`: `bias_j = sigma_bias_comune · z` per ogni alternativa (bias comune, una volta per legislatura: lo riestrae il piano 7 al voto), poi `effetti_istituto[k][j] = sigma_istituto · z` per istituto e alternativa; poi il primo sondaggio e la statistica del turno zero: `trimestre` = 2026T3, `vera` dall'economia iniziale con `crescita_pil = 0` [assunzione: lo scenario non dà la crescita del T3], `prima_stima = vera`, nessuna revisione.
- **Rilevazione dell'istituto `k`** con campione `n_k`: `q̂_j = max(0, p_j + bias_j + h_kj + √(d·p_j(1 − p_j)/n_k)·z)`, `p` = intenzioni nazionali vere, `d = effetto_disegno`; rinormalizza (se tutte zero, `q̂ = p`).
- **Incroci** sul campione unito `n = Σ n_k`: per ogni condizione, fascia e regione, `n_S = max(1, round(n · elettori_S / elettori))`; `q̂` come sopra con `p_S` del sottoinsieme e `h̄_j` media degli effetti d'istituto; `posizione`: `ĥ_x = max(0, h_S,x + √(d·h_S,x(1 − h_S,x)/n_S)·z)` rinormalizzato (se tutte zero, `ĥ = h_S`); `approvazione = γ·Ḡ_S + errore_approvazione/√n_S · z`, `Ḡ_S` media pesata del giudizio sul presidente.
- **Ordine delle estrazioni per turno**, fisso: istituti in ordine × alternative; poi condizioni, fasce, regioni, ciascuno: alternative, 7 punti, approvazione; poi le quattro stime.
- **Prime stime** del trimestre `data` appena chiuso: `vera.crescita_pil = 400·(R_t/R_{t−1} − 1)` con `R = pil_trimestrale/livello_prezzi` letto prima e dopo il passo; `disoccupazione` = `disoccupazione_nazionale`; `inflazione = inflazione[0]`; `disavanzo = 100·disavanzo_trimestre/(pil_trimestrale·1e9)`. `prima_stima = vera + errore_stima_<grandezza> · z`. Prima di sovrascrivere, `revisione = Some(trimestre vecchio, prima_stima vecchia, vera vecchia)`: la stima riveduta è il valore vero.
- `chiudi_turno` dopo `posizioni`: sondaggio, poi statistica.
- `carica` controlla lunghezze (`bias` = partiti + 1, `effetti_istituto` = istituti × (partiti + 1), quote e incroci della forma giusta) e finitezza.

- [ ] **Step 1: Scrivi i test che falliscono**

In `crates/sim-core/tests/sondaggi.rs`:

```rust
#[test]
fn la_vista_non_contiene_lo_stato_vero() {
    let mut p = Partita::nuova(dati_base(), 4);
    p.chiudi_turno();
    let json = serde_json::to_string(&p.vista()).unwrap();
    for chiave in ["\"bias\"", "\"effetti_istituto\"", "\"vera\"", "\"voto_precedente\"", "\"giudizio\"", "\"per_gruppo\""] {
        assert!(!json.contains(chiave), "{chiave}");
    }
}

#[test]
fn la_prima_stima_sbaglia_e_la_revisione_e_esatta() {
    let mut p = Partita::nuova(dati_base(), 4);
    p.chiudi_turno();
    let s = p.vista().statistica;
    let vera = p.informazione().statistica.vera.clone();
    assert_ne!(s.prima_stima.crescita_pil, vera.crescita_pil);
    p.chiudi_turno();
    let r = p.vista().statistica.revisione.unwrap();
    assert_eq!(r.riveduta, vera);
    assert_eq!(r.prima_stima, s.prima_stima);
}

#[test]
fn senza_errore_la_prima_stima_e_esatta() { /* errore_stima_* = 0.0: prima_stima == vera */ }

#[test]
fn la_media_degli_istituti_non_toglie_il_bias_comune() {
    // sigma_istituto = 0, effetto_disegno = 1, campioni dei dati: su 40 turni la media di (media degli istituti − vero)
    // per ogni partito sta entro 0,005 dal bias del partito, e almeno un bias supera 0,005 in valore assoluto.
}

#[test]
fn l_errore_cresce_nei_sottocampioni_piccoli() {
    // sigma_bias_comune = 0, sigma_istituto = 0: su 30 turni l'errore assoluto medio dell'incrocio della Valle d'Aosta
    // supera quello della Lombardia.
}

#[test]
fn incrocio_della_regione_piu_piccola_valido() {
    let p = Partita::nuova(dati_base(), 4);
    let vda = p.vista().sondaggio.regioni.into_iter().find(|i| i.id == "valle_d_aosta").unwrap();
    assert!(vda.campione >= 1);
    assert!((vda.quote.iter().sum::<f64>() - 1.0).abs() < 1e-12 && vda.quote.iter().all(|q| q.is_finite() && *q >= 0.0));
    assert!((vda.posizione.iter().sum::<f64>() - 1.0).abs() < 1e-12 && vda.posizione.iter().all(|h| h.is_finite() && *h >= 0.0));
}

#[test]
fn quattro_istituti_e_incroci_completi() {
    let s = Partita::nuova(dati_base(), 4).vista().sondaggio;
    assert_eq!((s.istituti.len(), s.condizioni.len(), s.fasce.len(), s.regioni.len()), (4, 6, 3, 20));
}
```

Più `salvataggio_con_effetti_istituto_corti_illeggibile` in `tests/partita.rs`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test sondaggi --test partita`
Expected: FAIL.

- [ ] **Step 3: Implementa `informazione/` e i campi della vista**

I tipi TypeScript si rigenerano in `app/src/lib/tipi` con `cargo test -p sim-core` (`.cargo/config.toml:2`): committali.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(informazione): sondaggio pubblico e prime stime"
```

### Task 8: Elettori campione e proiezione dei seggi

**Files:**
- Create: `crates/sim-core/src/informazione/campione.rs`
- Modify: `crates/sim-core/src/caso.rs` (flusso in coda, estrattore estemporaneo)
- Modify: `crates/sim-core/src/vista.rs` (`proiezione`), `crates/sim-core/src/partita.rs` (`elettori_campione`, `proiezione_seggi`, `vista`)
- Test: `crates/sim-core/tests/sondaggi.rs`, `crates/sim-core/tests/caso.rs`

**Ricerca:** ticket 06 (`06-elettorato.md:18`, elettori campione dal sondaggio, motivo); `docs/ricerca/modelli/sondaggi-ed-errore.md:59` (proiezione in seggi col sistema elettorale); spec riga 322 (sondaggio del trimestre, coalizioni dell'ultimo voto, partiti nati dopo da soli).

**Dispatch:** `rust-engineer` · dopo il Task 7: legge solo la `Vista` del 7 (sondaggio, partiti); `partita.rs`, `vista.rs`, `tests/sondaggi.rs`, più `caso.rs` e `tests/caso.rs`, che nessun altro task tocca; tipi TypeScript rigenerati. Chiude la PR A, poi il round di review · skill-gate: true.

**Interfaces:**
- Consumes: `Vista` (Task 7), `elezioni::riparto(dati: &DatiElezioni, legge: &LeggeElettorale, voti: &Voti) -> Result<Esito, Errore>` (`crates/sim-core/src/elezioni/riparto.rs:665`), `Voti { coalizioni: Vec<Vec<usize>>, per_regione: BTreeMap<String, Vec<u64>> }` (`riparto.rs:16-19`), `DatiGioco::legge_elettorale_iniziale` (`dati.rs:211`).
- Produces:
  - `Flusso::ElettoriCampione = 6` in coda all'enum (`caso.rs:7`: mai riordinare). `Caso::estemporaneo(&self, chiave: u64) -> Estrattore`: `ChaCha8Rng::seed_from_u64(seme)`, `set_stream(6)`, `set_word_pos(u128::from(chiave) << 32)`; non tocca i flussi del `Caso`. `Estrattore` ha `uniforme(&mut self) -> f64` e `normale(&mut self) -> f64` con le stesse formule di `Caso` (`caso.rs:49-61`), estratte in due funzioni libere condivise.
  - `ElettoreCampione { pub nome: String, pub eta: u8, pub posizione: u8, pub voto: Option<String>, pub motivo: Motivo }`, `Motivo { Vicinanza, Leader, Governo, Protesta }` (`Serialize`, `TS`).
  - `pub fn genera(vista: &Vista, dati: &DatiGioco, regione: usize, c: Condizione, f: Fascia, rng: &mut Estrattore) -> Vec<ElettoreCampione>`: legge solo la vista e i dati, mai lo stato vero.
  - `Partita::elettori_campione(&self, regione: &str, c: Condizione, f: Fascia) -> Result<Vec<ElettoreCampione>, Errore>`: tre elettori; regione sconosciuta → `Errore::Elettorato` col nome; chiave `turno·1024 + indice_gruppo`.
  - `Partita::proiezione_seggi(&self) -> Result<Esito, Errore>`; `Vista` acquista `pub proiezione: Option<Vec<SeggiPartito>>`, `SeggiPartito { pub partito: String, pub camera: u32, pub senato: u32 }` (`None` solo se il riparto rifiuta i voti).

Regole:
- **Stima del gruppo dal sondaggio**: quote `p̂ ∝ q̂_c ⊙ q̂_r ⊙ q̂_f / q̂_n²` (prodotto elemento per elemento, `q̂_n` = media delle rilevazioni degli istituti), rinormalizzate; istogramma `ĥ ∝ ĥ_c ⊙ ĥ_r ⊙ ĥ_f / ĥ_n²`, `ĥ_n` = media degli istogrammi regionali pesata con gli elettori; approvazione `Â = Â_c + Â_r + Â_f − 2·Â_n`, `Â_n` media regionale pesata. Una voce con denominatore zero vale zero.
- **Un elettore**: voto `k ~ p̂`; posizione `x ~ ĥ_x · w_x`, `w_x = exp(−β(x − percepita_k)²)` se `k` è un partito, `1` per l'astensione; età uniforme intera in 18-25 per lo studente, 65-90 per il pensionato, 25-64 altrimenti; nome `nomi[⌊u·len⌋] + " " + cognomi[⌊u·len⌋]`.
- **Motivo** (spec riga 314): astensione → `Protesta`. Altrimenti, con le grandezze pubbliche della vista: `vicinanza = β·(media_j (x − percepita_j)² − (x − percepita_k)²)`, `leader = valence_k − media_j valence_j`, `governo = Â` se `k` è al governo, `−Â` altrimenti; vince il più grande, a pari merito nell'ordine vicinanza, leader, governo. La fedeltà non è osservabile dal sondaggio e non è un motivo [assunzione del piano 4].
- **Proiezione**: per ogni regione `voti_j = round(elettori_r · q̂_r,j)` per i partiti (astensione esclusa); una lista per partito, nell'ordine dello scenario; coalizioni = quelle dello scenario come indici, gli altri partiti da soli; legge = `legge_elettorale_iniziale` (il piano 6 la renderà quella in vigore).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn elettori_campione_non_toccano_la_partita() {
    let mut a = Partita::nuova(dati_base(), 9);
    let mut b = Partita::nuova(dati_base(), 9);
    let primi = a.elettori_campione("campania", Condizione::Pensionato, Fascia::Bassa).unwrap();
    assert_eq!(primi, a.elettori_campione("campania", Condizione::Pensionato, Fascia::Bassa).unwrap());
    assert_eq!(primi.len(), 3);
    a.chiudi_turno(); b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}

#[test]
fn nessun_elettore_vota_un_partito_che_il_sondaggio_da_a_zero() {
    // Vista della base con quote del partito "centro" azzerate (e rinormalizzate) in ogni incrocio e rilevazione:
    // 50 chiamate a `genera` con chiavi diverse, nessun voto "centro".
}

#[test]
fn chi_si_astiene_ha_motivo_protesta() { /* su 200 elettori generati, ogni voto None ha Motivo::Protesta e nessun voto Some lo ha */ }

#[test]
fn regione_sconosciuta_rifiutata_col_nome() {
    let p = Partita::nuova(dati_base(), 1);
    let Err(Errore::Elettorato(m)) = p.elettori_campione("padania", Condizione::Studente, Fascia::Media) else { panic!() };
    assert!(m.contains("padania"));
}

#[test]
fn la_proiezione_assegna_tutti_i_seggi() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let s = v.proiezione.expect("riparto riuscito");
    assert_eq!(s.iter().map(|x| x.camera).sum::<u32>(), 400);
    assert_eq!(s.iter().map(|x| x.senato).sum::<u32>(), 200);
}
```

In `tests/caso.rs`: `estemporaneo_non_avanza_i_flussi` (dopo `estemporaneo(5)` il prossimo valore di ogni flusso è lo stesso di un `Caso` gemello) e `estemporaneo_dipende_dalla_chiave` (chiavi 1 e 2 danno uniformi diverse, la stessa chiave le stesse).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test sondaggi --test caso`
Expected: FAIL.

- [ ] **Step 3: Implementa `campione.rs`, l'estrattore e la proiezione**

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS; tipi TypeScript rigenerati.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(informazione): elettori campione e proiezione dei seggi"
```

---

## PR B · Pipeline di taratura offline (Task 9-12)

Gira fuori dal binario del gioco (spec riga 405): `strumenti/taratura/elettorato.py` scarica le fonti aperte, legge da una cartella locale quelle che non si ridistribuiscono, e riscrive `dati/base/elettorato.toml`; `sim-cli tara` usa lo stesso logit del gioco per le costanti che dipendono da esso. Nel repo stanno solo i risultati aggregati, i parametri e lo script (`strumenti/taratura/README.md:3`). La cache dei download sta fuori dal repo (`--cache <cartella>`, default `~/.cache/simulatore-politico`), mai committata.

Ogni task comincia verificando le sue premesse sulle fonti: se un endpoint non risponde o un file richiede un accesso che la sessione non ha, il task tiene il corpo provvisorio della PR A per quella funzione, lo dichiara nel README e nel report, e prosegue.

### Task 9: Quote dei gruppi con IPF sui margini

**Files:**
- Modify: `strumenti/taratura/elettorato.py` (`elettori()`, `quote()`)
- Modify: `strumenti/taratura/README.md`, `dati/base/elettorato.toml` (rigenerato)

**Ricerca:** `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:20-35` (tabella dei dati e licenze), `:56-62` (metodo IPF), `:64-66` (fasce sui decili), `:119-131` (cosa entra nella cartella dati); `docs/ricerca/dati/banca-italia-ibf.md:12-14` (archivio 2022 scaricabile, variabili); `docs/ricerca/dati/risultati-elettorali-italia.md:13` (CSV della Camera 2022).

**Dispatch:** `coder` · primo della PR B, dopo la review della PR A, sul branch `feat/v1-piano-4-taratura` dalla punta della PR A; in parallelo col Task 11 e col passo 1 del Task 12 · skill-gate: true. Pipeline che scarica e trasforma, come il Task 2 del piano 3: non `data-scientist`. Le risposte delle API sono dati non fidati, si leggono e non si eseguono; i microdati IBF restano nella cache, mai nel repo.

**Interfaces:**
- Consumes: SDMX ISTAT `150_938_DF_DCCV_OCCUPATIT1_23`, `151_929_DF_DCCV_DISOCCUPT1_6`, `152_928_DF_DCCV_INATTIV1_9`, `32_38_DF_DCCV_QUINTOREDD_5` (`dati-per-tarare-i-gruppi.md:152-163`); MEF open data pensionati e dichiarazioni (`:164-169`); IBF 2022 `ind22_ascii.zip` (`banca-italia-ibf.md:14`); CSV Camera 2022 (`risultati-elettorali-italia.md:13`). Gli helper di download con attesa e tentativi di `economia.py` (copiati o importati, a scelta dell'implementatore).
- Produces: `elettori()` dal CSV della Camera per regione (Valle d'Aosta dal suo collegio, se presente nel file; altrimenti dalla popolazione, dichiarato); `quote()` = IPF 20 × 6 × 3 con seme IBF pesato `pesofit2` (celle vuote al prodotto dei margini), margini (a) elettori per regione, (b) regione × condizione da ISTAT e MEF con la quota pubblica dei dipendenti come assunzione dichiarata, (c) regione × fascia dalle classi MEF tradotte in fasce, (d) condizione × fascia nazionale dai quinti ISTAT; convergenza entro lo 0,1% di ogni margine. `--senza-ibf` usa il seme d'indipendenza. Il selftest sostituisce l'invariante «elettori 46.350.000 ± 20» con «somma degli elettori = elettori del CSV + Valle d'Aosta», esatta.

- [ ] **Step 1: Premesse** — verifica che ogni fonte risponda e che le colonne/dimensioni citate esistano; annota nel report cosa non risponde.
- [ ] **Step 2: Selftest d'IPF** — `--selftest` aggiunge un caso sintetico 2 × 2 × 2 con margini coerenti noti: l'IPF li ritrova entro 1e-9, e un seme con una cella zero resta zero solo se il margine lo consente.
- [ ] **Step 3: Esegui il selftest e verifica che fallisca**, poi implementa `elettori()` e `quote()`.
- [ ] **Step 4: Rigenera** — `python3 -I strumenti/taratura/elettorato.py`; `cargo test --workspace` resta verde coi dati nuovi (i test del core non dipendono dai numeri dei dati, solo dalla forma).
- [ ] **Step 5: README** (fonti con interrogazione, licenze, modifiche ai dati, ripieghi) **e commit** `feat(taratura): quote dei gruppi con IPF sui margini`.

### Task 10: Istogrammi, redditi e panieri

**Files:**
- Modify: `strumenti/taratura/elettorato.py` (`istogrammi()`, `redditi()`, `panieri()`), `strumenti/taratura/README.md`, `dati/base/elettorato.toml`

**Ricerca:** `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:80-95` (IPF con la posizione come quarta dimensione, taglio 0-10 → 7 punti, ESS per la cartella dati e ITANES solo offline); `docs/ricerca/dati/ess.md:21` (condizioni d'uso ESS).

**Dispatch:** `coder` · dopo il Task 9: stesso `elettorato.py` ed `elettorato.toml`, IPF del 9 estesa a quattro dimensioni sui margini dei gruppi del 9 · skill-gate: true. Se il Data Portal ESS chiede un accesso che la sessione non ha, il file lo passa Mario con `--ess`; fino ad allora resta l'istogramma provvisorio, dichiarato.

**Interfaces:**
- Consumes: file ESS round 11 per l'Italia da `--ess <file>` (CSV del Data Portal: autocollocazione, attività principale, dipendente/autonomo, tipo di organizzazione, reddito in 10 categorie, regione); margini dei redditi MEF per tipo e regione; spesa delle famiglie per quinto ISTAT per i panieri (interrogazione da individuare al passo 1).
- Produces: `istogrammi()` = IPF a 4 dimensioni 20 × 6 × 3 × 7 coi margini del gruppo (Task 9), condizione × fascia × posizione e regione × posizione da ESS (regioni sotto 100 casi accorpate alla macroarea), taglio `{0,1}→1, {2,3}→2, 4→3, 5→4, 6→5, {7,8}→6, {9,10}→7`, chi non si colloca escluso dai margini della posizione; `redditi()` dai margini MEF; `panieri()` dalle quote di spesa per divisione dei quinti ricondotte ai 10 settori. Senza `--ess` l'istogramma resta quello provvisorio, dichiarato.

- [ ] **Step 1: Premesse** — scaricabilità di ESS (accesso del Data Portal), presenza della regione nel file, interrogazioni ISTAT e MEF dei redditi e dei consumi per quinto.
- [ ] **Step 2: Selftest** — l'IPF a 4 dimensioni su un caso sintetico ritrova i margini; il taglio 0-10 → 7 punti su tutti gli 11 valori.
- [ ] **Step 3: Implementa, rigenera, `cargo test --workspace` verde.**
- [ ] **Step 4: README (licenza CC BY-NC-SA dei derivati ESS dichiarata accanto al file) e commit** `feat(taratura): istogrammi, redditi e panieri dei gruppi`.

### Task 11: β e fedeltà stimati offline su ITANES

**Files:**
- Create: `strumenti/taratura/stima_voto.py`
- Modify: `strumenti/taratura/README.md`, `dati/base/manopole.toml` (solo `valore` e commento di `beta_distanza` e `fedelta`)

**Ricerca:** `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:99-111` (logit condizionale su ITANES post 2022, cambio di scala β₇ = β₁₀/0,36, matrice 2018 → 2022, fedeltà come tetto); `docs/ricerca/dati/itanes.md:20-31` (termini: solo parametri nel repo, mai i microdati).

**Dispatch:** `data-scientist` · in parallelo col 9 → 10: file disgiunti salvo `strumenti/taratura/README.md` (sezione e commit dopo il commit in corso dell'altro) · skill-gate: true. È una stima per massima verosimiglianza con errore standard, cambio di scala e fedeltà da leggere come tetto: la domanda è se il numero regge, quindi `data-scientist` e non `coder`. Il passo 1 si ferma se i termini ITANES vanno accettati da Mario; i microdati stanno in una cartella locale fuori dal repo.

**Interfaces:**
- Consumes: ITANES 2022 pre e post da `--itanes <cartella>` (Dataverse UNIMI, `doi:10.13130/RD_UNIMI/JV77WR`); pacchetto `nopp` (dati CSES Italia 2006) da `--nopp <file>` come banco di prova del codice.
- Produces: `stima_voto.py` con logit condizionale per massima verosimiglianza (Newton-Raphson, libreria standard) su `U = −β·d² + λ_partito + φ·[voto 2018 = j]`, distanze sulla scala 0-10; il voto 2018 si riconduce alle alternative del 2022 per famiglia politica con una tabella nello script (partito senza erede: nessun termine di fedeltà), e φ vale anche per astensione → astensione, come `voto_precedente` nel gioco; scrive in `manopole.toml` `beta_distanza = β₁₀/0,36` e `fedelta = φ`, con errore standard nel commento; `--selftest` ritrova β e φ da dati sintetici generati con valori noti (errore < 5%).

- [ ] **Step 1: Premesse** — scaricabilità dei file ITANES (termini accettati da Mario: se servono, il task si ferma qui e lo dice), variabili `:683`, `:702`, `:573` del questionario.
- [ ] **Step 2: Selftest su dati sintetici, falliscono; implementa; passano.**
- [ ] **Step 3: Stima sui dati veri, scrivi le due manopole (se fuori dall'intervallo, allarga l'intervallo e dichiaralo), `cargo test --workspace` verde.**
- [ ] **Step 4: Commit** `feat(taratura): beta e fedelta stimati su ITANES`.

### Task 12: Valence e astensione tarate col logit del gioco

**Files:**
- Create: `docs/ricerca/dati/sondaggi-ottobre-2026.md` (ricerca, `researcher`)
- Create: `crates/sim-cli/src/tara.rs`; Modify: `crates/sim-cli/src/main.rs` (sottocomando `tara`)
- Create: `strumenti/taratura/bersagli.toml` (affluenza 2022 per regione e intenzioni di voto per posto, con fonti)
- Modify: `dati/base/scenario.toml` (`valore` delle valence), `dati/base/elettorato.toml` (`astensione` delle regioni), `strumenti/taratura/README.md`
- Test: `crates/sim-cli/tests/cli.rs`

**Ricerca:** spec righe 284-285 (valence tarata sulle intenzioni reali di ottobre 2026 sommate per posto, corrispondenza offline con le collocazioni CHES; vincibilità), `docs/ricerca/dati/ches.md:18-29` (CHES come fonte delle posizioni, licenza da chiarire: si usa solo offline); `docs/ricerca/dati/risultati-elettorali-italia.md:16-27` (affluenza 2022); `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:117` (costante per regione sull'affluenza ufficiale).

**Dispatch:** passo 1 a `researcher`, che scrive solo `docs/ricerca/dati/sondaggi-ottobre-2026.md` e la sua riga in `docs/ricerca/index.md`, in parallelo col Task 9. Passi 2-5 a `rust-engineer`, dopo 9, 10, 11 e il passo 1. Scrive anche `strumenti/taratura/bersagli.toml` dai numeri del researcher, perché il researcher scrive solo in `docs/ricerca/`; tocca `crates/sim-cli` (`tara.rs`, `main.rs`, `tests/cli.rs`) e riscrive righe di `scenario.toml` ed `elettorato.toml`. Chiude la PR B · skill-gate: true per entrambi.

**Interfaces:**
- Consumes: `Partita::nuova`, `Partita::intenzioni_vere` (Task 6), `FileDati`, `DatiGioco::da_testi`.
- Produces:
  - `docs/ricerca/dati/sondaggi-ottobre-2026.md`: media dei sondaggi nazionali di ottobre 2026 per partito reale, con fonte, data e tre campi per riferimento; corrispondenza partito reale → posto dalla collocazione CHES 2024 sinistra-destra (1-7 col taglio di Task 10); somma per posto. Riga in `docs/ricerca/index.md`.
  - `sim-cli tara --dati <cartella> --bersagli <file>`: contrazione alla BLP sulle costanti, `astensione_r ← astensione_r + ln((1 − a_r)/a_r) − ln((1 − â_r)/â_r)` e `valence_j ← valence_j + ln(s_j/ŝ_j)` (quote fra i votanti), con `â`, `ŝ` dalle intenzioni del turno zero, fino a scarti < 1e-4 o 200 giri; riscrive solo le righe `astensione = ` delle regioni e `valore = ` delle valence (riconosciute per sezione), lasciando il resto dei file com'è; esce con codice 1 se non converge o se una valence esce da `[min, max]` del suo `Ingresso`, nominando il partito. Si tara sulle intenzioni vere e non su un sondaggio estratto: il valore atteso del sondaggio è vero + bias, e il bias comune ha media zero (spec riga 284). Il README fissa l'ordine: `elettorato.py`, poi `sim-cli tara`; rilanciare `elettorato.py` cancella la taratura dell'astensione e va seguito da `tara`.
- [ ] **Step 1: Ricerca** — dispatch `researcher` per `sondaggi-ottobre-2026.md`; `bersagli.toml` dai suoi numeri e dalla tabella dell'affluenza.
- [ ] **Step 2: Test che falliscono** in `tests/cli.rs`: `tara` su una copia dei dati in una cartella temporanea porta affluenza per regione e quote per posto del turno zero entro 1e-3 dai bersagli; `tara` con un bersaglio impossibile (quota negativa) esce con 1 e messaggio.
- [ ] **Step 3: Implementa `tara.rs`, esegui sui dati di base, `cargo test --workspace` verde.**
- [ ] **Step 4: Controllo di vincibilità** — riporta nel report, per ciascuna delle due coalizioni dello scenario, la quota attesa del turno zero; la vincibilità vera (spec riga 285) si verifica col partito del giocatore nel piano 7.
- [ ] **Step 5: README e commit** `feat(taratura): valence e astensione tarate sul turno zero`.
