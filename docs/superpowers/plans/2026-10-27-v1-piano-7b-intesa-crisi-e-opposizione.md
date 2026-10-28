# V1 · Piano 7b: intesa, crisi e opposizione — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Il governo può cadere e il giocatore può tornarci. L'intesa degli alleati ha tutte le sue cause, l'ultimatum diventa uno stato, sotto 25 l'alleato esce: rimpasto se la maggioranza regge, altrimenti un turno di crisi e la sfiducia. Il governo cade anche per dimissioni (volontarie, dopo una fiducia respinta, dopo un bilancio mancato in esercizio provvisorio), e il Capo dello Stato fa un ribaltone o scioglie. All'opposizione il giocatore decide la linea, presenta la sfiducia, porta proposte nel posto riservato e chiede referendum; torna al governo col voto o con un ribaltone, chiedendo la guida se non è il primo. Gli alleati emendano il bilancio. La partita si chiude con l'eredità o con l'abbandono.

**Architecture:** Niente moduli nuovi oltre a `legislatura/crisi.rs`, che raccoglie la sfiducia a blocchi e il valore della caduta. Lo stato nuovo sta dove c'è già il suo simile: ultimatum in `Intesa`, crisi, dimissioni e maggioranza del ribaltone in `Legislatura`, sfiducia in calendario in `Parlamento`. Tre passi si aggiungono alla chiusura, dopo le soglie e prima della formazione del governo: uscite (rimpasto o crisi), fine della crisi, Capo dello Stato. Le regole valgono per chi governa, giocatore o IA; i comandi nuovi entrano dall'`enum Comando` con due guardie, quella di governo che esiste già e una per l'opposizione.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 34 (riga 65), 44-45 (righe 78-79), 83-86 (righe 126-129), 88-98 (righe 134-144), 149-152 (righe 216-219); «Governo, ministri e coalizione» (righe 331-336, coalizione, intesa, uscita, sfiducia, dimissioni); «Parlamento» (righe 340, 342-344, linea, fiducia, bilancio, calendario, referendum); «Partiti IA» (riga 353, governo avversario, solo dove serve al 7b); «Turno e partita» (righe 386-389, default delle decisioni, crisi, fine). Scope: [`2026-10-27-v1-piano-7a-turno-zero-voto-e-governo.md:18`](2026-10-27-v1-piano-7a-turno-zero-voto-e-governo.md) («Perché due piani») e `:1287` («Rimandato»). Ticket: `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md` (righe 26-37: intesa, uscita, sfiducia, dimissioni), `12-opposizione.md` (righe 30-56: leve, ritorno, fine), `28-salvataggi.md:29`, `:35` (abbandono, eredità in bozza), `29-costo-del-governare.md:50` (luna di miele solo al primo governo). Debiti del 7a: memoria `decisioni-piano-7`, sezioni «Decisioni da confermare» delle PR #28 e #29.

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · intesa | `feat/v1-piano-7b-intesa` | `main` | 1-3 | rinomino «posizione», schema 8, debiti del 7a, intesa completa, ultimatum come stato, rottura dell'impegno |
| B · crisi | `feat/v1-piano-7b-crisi` | `feat/v1-piano-7b-intesa` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 4-6 | sfiducia, dimissioni e Capo dello Stato; uscita, rimpasto e crisi; ribaltone a favore del giocatore e guida |
| C · opposizione | `feat/v1-piano-7b-opposizione` | `feat/v1-piano-7b-crisi` (impilata, stessa regola) | 7-9 | comandi dell'opposizione, emendamenti al bilancio, eredità e abbandono |

Questo file entra nel primo commit della PR A.

## Decisioni di Mario

Del 27/10/2026, sulle domande di questo piano:

1. **Ribaltone in cui il giocatore non è il primo.** Se il giocatore non sceglie, il default è chiedere la guida. Il partito più grande accetta se gli conviene più del voto; se rifiuta, il Capo dello Stato cerca una maggioranza senza il giocatore, altrimenti scioglie (Task 6).
2. **Crisi = solo controllo.** Nel turno di crisi sono rifiutati leggi nuove, bilancio, decreti, fiducia e `Accogli`; restano `Ferma`, `Dirotta`, `FermaCantiere`, `Rinvia`, `Respingi`, `ChiediDimissioni`, `ScegliSuccessore`, `Emendamento`, `InvitaAstensione`, più `Allarga` e `Dimettiti`. Un bilancio non presentato in crisi porta all'esercizio provvisorio (Task 5).
3. **Rottura dell'impegno simmetrica.** Paga anche il partito del giocatore quando governa con chi ha corso contro di lui (Task 3).
4. **Tabella `[parlamento_uscente]`.** Confermata la scelta 1 del piano 7a: resta per la partita di prova (`Partita::nuova`). Nessun task.

Restano valide le decisioni del 7a (memorie `decisioni-piano-7` e `decisioni-piano-6-aperte`), compresa quella delle larghe intese: `maggioranza_contigua` toglie dalla fila il partito escluso.

## Scelte del piano, da confermare in review

1. **`VERSIONE_SCHEMA` passa a 8** (Task 1). Nessuna release è uscita con lo schema 7, ma i salvataggi di prova di Mario esistono: con lo schema nuovo sono rifiutati con il messaggio della cucitura 2, invece che con un errore di forma.
2. **Rinomino solo per l'asse.** `Fondazione.posto` → `posizione`, `posti()` → `posizioni()`, `Posto` → `VocePosizione`, `AnteprimaFondazione.posti` → `posizioni`, `--posto` → `--posizione`. I posti del calendario (`POSTI_ORDINARI`, «posto riservato») restano: sono posti, non posizioni.
3. **`Allarga` prende una lista** (`partiti: Vec<String>`). Ogni partito della lista entra solo se è disponibile con la lista intera. La probabilità di governare dopo il voto resta a 0 o 1, perché i seggi sono noti. Così due partner che bastano solo insieme entrano insieme (debito del 7a).
4. **Successore di un tecnico**: la rosa è di tre tecnici. Quello di un ministro di partito è di tre nomi dello stesso partito, come oggi.
5. **Blocco dei referendum dopo i comizi** (`iter-legislativo-italiano.md:135`): i comizi si convocano nel turno del voto, quindi sei mesi coprono il turno dopo, cioè quello di trattativa (`turno <= legislatura.inizio`, solo dopo almeno un voto).
6. **`valida` delle coalizioni dello scenario**: la memoria non dice quale controllo manchi. Il piano aggiunge l'unico che `coalizioni_al_voto` presuppone: una coalizione di meno di due partiti è rifiutata col nome.
7. **Governo IA**: alla formazione l'intesa dei suoi alleati riparte da `intesa_base`, perché la sua spartizione è la richiesta di Gamson esatta (spec riga 353).
8. **Ordine delle soglie dell'intesa**: ultimatum e uscita si leggono sul valore dopo le variazioni del turno e **prima** del ritorno verso la base. L'ultimatum nasce sotto `soglia_ultimatum` con la causa peggiore delle variazioni recenti, e si spegne quando l'intesa torna alla soglia.
9. **Rimpasto**: si ridistribuiscono i dicasteri liberati e quelli del partito del Presidente del Consiglio, che ha come richiesta il resto (scelta 6 del 7a). Chi perde un dicastero lascia, e il dicastero va in interim con la rosa del partito nuovo. `spartizione_di_default` diventa il caso con tutti i dicasteri liberi.
10. **Crisi**: l'alleato esce alla chiusura del turno `t`, la crisi occupa il turno `t + 1` e si chiude nella sua chiusura. Con la maggioranza tornata (allargati compresi) c'è il rimpasto, altrimenti la sfiducia automatica, che passa per costruzione e si registra come esito leggibile.
11. **Caduta di un governo IA**: il Capo dello Stato cerca la maggioranza contigua **senza escludere nessuno**, quindi può tornare il partito che è caduto (un reincarico, come Conte I → II). Con la caduta del governo del giocatore si esclude il suo partito (spec riga 336).
12. **Valore della caduta per un alleato in ultimatum**: sui termini del valore atteso del 7a (`Quadro::termini`). Restare vale seggi, governo certo, distanza dai membri e intesa bassa. Cadere vale come la maggioranza contigua che lo contiene, coi seggi veri; se nessuna lo contiene, vale come il voto proiettato sul sondaggio pubblico, con lui fuori dalla coalizione del Presidente del Consiglio.
13. **Sfiducia del giocatore**: votata nella chiusura dello stesso turno, dopo le leggi, una per turno. Le firme sono di tutti i partiti fuori dalla coalizione di governo e servono almeno un decimo dei seggi della Camera (`iter-legislativo-italiano.md:43`).
14. **Linea del giocatore**: la scelta vale su ogni misura in calendario. I franchi tiratori del suo partito crescono con la distanza fra la sua posizione e la posizione che il voto sostiene: la nuova se vota sì, la vigente se vota no. La posizione percepita scivola verso quella sostenuta.
15. **Posto riservato a rotazione**: la fila sono i partiti fuori dalla coalizione di governo con seggi alla Camera, in ordine d'indice; nel turno `t` il posto va al partito `fila[t % fila.len()]`. Ci vanno solo riforme ed elettorali, e sulle proposte dell'opposizione gli alleati non chiedono emendamenti. Respinta, la proposta non costa nulla a nessuno: blocca solo la ripresentazione.
16. **Emendamenti al bilancio**: alla presentazione, ogni alleato con un dicastero la cui voce perde più di `soglia_emendamento_bilancio` punti reali chiede di indicizzarlo. Accolto, la voce si indicizza come con `Partita::indicizza`; respinto, costa come un emendamento respinto su una legge. Il governo IA non riceve emendamenti fino al piano 8.
17. **Eredità**: anni al governo come turni da Presidente del Consiglio diviso 4; leggi ancora in vigore = disposizioni il cui valore viene da una legge portata dal giocatore (al governo, dall'opposizione o confermata dal referendum); indicatori = debito in % del PIL, spread, disoccupazione e inflazione, alla fine e al turno zero. I riavvolgimenti li conta il guscio (piano 10).

## Global Constraints

Valgono tutti i vincoli del piano 7a (`2026-10-27-v1-piano-7a-turno-zero-voto-e-governo.md:62-76`) e, attraverso quelli, dei piani 1-6: italiano coi termini di `CONTEXT.md` (intesa, ultimatum, crisi, rimpasto, sfiducia, opposizione, governo avversario, eredità; «posizione» sull'asse, mai «posto»); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano il colpevole; niente `serde(default)`. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Nessun flusso casuale nuovo. La sfiducia è deterministica, a blocchi e senza estrazioni. Uscite, rimpasto, Capo dello Stato e guida non estraggono nulla. Le rose nuove estraggono da `Flusso::Governo`, come oggi.
- Un comando rifiutato lascia la partita identica (`salva()` prima e dopo uguali). Ogni rifiuto è un `Errore::Comando` col nome del partito, del dicastero o della fase in causa.
- Ogni manopola nuova ha `valore`, `min`, `max`, `ticket`, entra in `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:65`) nel task che la introduce e porta `# assunzione del piano 7b, nessuna fonte in docs/ricerca` quando non ha fonte.
- La `Vista` non porta stato vero nuovo. All'opposizione l'intesa avversaria si vede in tre fasce (spec riga 370), mai in cifre; la sfiducia mostra chi la voterebbe, mai i valori attesi.
- `VERSIONE_SCHEMA` passa a `8` nel Task 1 e resta `8` per tutto il piano.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Partita di prova (`Partita::nuova`): sette partiti nell'ordine dello scenario (0 estrema sinistra … 3 centro, 4 centrodestra, 5 destra, 6 estrema destra), il giocatore è la destra (5) al governo col centrodestra (4), che tiene Ambiente ed Energia (dicastero 6) ed Esteri e Difesa (9); scadenza 19; turno 0 = ottobre-dicembre 2026. Partita vera: il giocatore è l'indice 7, come nel 7a.
- Testi per il giocatore elencati nella PR, come nella #26, per la revisione di Mario.

## Review Focus

1. **Due cadute nella stessa chiusura** (dimissioni chieste nel turno di crisi e sfiducia automatica; fiducia respinta e bilancio mancato in esercizio provvisorio): una sola caduta, un solo passaggio dal Capo dello Stato, nessun panic. Test `due_cadute_nello_stesso_turno` nel Task 5.
2. **Dimissioni o uscita nel turno del voto** (scadenza o Camere sciolte): vince il voto, nessun Capo dello Stato e nessuna crisi. Test `dimissioni_nel_turno_del_voto` nel Task 4.
3. **Alleato che esce mentre un suo dicastero è già in interim, con la rosa aperta o con le dimissioni chieste**: rimpasto coerente, salvataggio che si ricarica. Test `rimpasto_con_interim_si_ricarica` nel Task 5.
4. **Comandi dell'opposizione dati da chi governa, in trattativa, a Camere sciolte o a partita finita, e `Dimettiti` dato all'opposizione**: rifiuto col motivo, partita identica. Test `comandi_dell_opposizione_rifiutati_a_chi_governa` nel Task 7.
5. **Eredità di chi non ha mai governato, abbandono ripetuto, abbandono dopo la fine per seggi**: zero anni e nessuna legge senza divisioni per zero, secondo abbandono rifiutato, eredità in bozza solo per la fine per seggi. Test `eredita_di_chi_non_ha_mai_governato` nel Task 9.

## Dispatch

PR A: 1 → 2 → 3 in sequenza; tutti toccano `partita.rs`, il 2 e il 3 anche `legislatura/`.

PR B: dopo la review della A, sul branch impilato, 4 → 5 → 6 in sequenza: il 5 usa la sfiducia e il Capo dello Stato del 4, il 6 li estende al governo IA.

PR C: dopo la review della B, 7 → 8 → 9 in sequenza: tutti toccano `comando.rs`, `partita.rs` e `vista.rs`.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task. A fine PR, review a sei in parallelo: `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer` (legge i testi per il giocatore elencati nella PR), `rust-reviewer` (il diff tocca `.rs`) e `spec-reviewer` (questo piano), con i findings di quest'ultimo in un report a parte. Poi un solo giro di correzioni. Il merge lo decide Mario.

---

## PR A · Intesa (Task 1-3)

### Task 1: «Posizione», messaggi dei ministri e schema 8

**Files:**
- Modify: `crates/sim-core/src/legislatura/fondazione.rs:14-52` (`Fondazione.posto`, `Posto`, `posti`), `crates/sim-core/src/legislatura/mod.rs:13` (ri-esportazioni)
- Modify: `crates/sim-core/src/partita.rs:34` (`VERSIONE_SCHEMA = 8`) e ogni uso dei nomi rinominati
- Modify: `crates/sim-core/src/ministri/mod.rs:442-458` e ogni altro messaggio di `Errore::Comando` in `crates/sim-core/src/ministri/` che stampa l'id del dicastero dove il dicastero è noto
- Modify: `crates/sim-cli/src/main.rs` (`--posto` → `--posizione`), `crates/sim-cli/src/tara.rs`
- Modify: `crates/sim-core/tests/comune/mod.rs:319` (`fondata(posizione: f64)`), i test che usano i nomi vecchi, `crates/sim-cli/tests/cli.rs`
- Test: `crates/sim-core/tests/ministri.rs`

**Ricerca:** - nessun riferimento pertinente (rinomino e testi; debito del 7a, PR #28 «Posizione o posto»).

**Dispatch:** `rust-engineer` · primo della PR A · skill-gate: true.

**Interfaces:**
- Consumes: nulla di nuovo.
- Produces:
  - `pub struct Fondazione { pub nome: String, pub posizione: f64 }`; `pub fn posizioni(dati: &DatiGioco) -> Vec<f64>`; `pub struct VocePosizione { pub posizione: f64, pub quota_attesa: f64, pub disponibilita: Vec<VoceDisponibilita> }`; `AnteprimaFondazione { istogramma, posizioni: Vec<VocePosizione> }`. I testi del rifiuto restano quelli della PR #28 («posizione non valida», «posizione 0,5 non ammessa: …»).
  - CLI `--posizione <x>`.
  - Nei messaggi dei ministri il nome del dicastero dai dati (`Sanità`), mai l'id; «dicastero sconosciuto: {id}» resta con l'id, perché un nome non c'è.
  - `VERSIONE_SCHEMA = 8`.

- [ ] **Step 1: Scrivi il test che fallisce**

```rust
// crates/sim-core/tests/ministri.rs (in coda)
#[test]
fn i_messaggi_dei_ministri_nominano_il_dicastero() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap();
    let e = p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap_err();
    assert!(e.to_string().contains("Sanità"), "{e}");
    let e = p.comanda(Comando::ScegliSuccessore { dicastero: "giustizia".into(), indice: 0 }).unwrap_err();
    assert!(e.to_string().contains("Giustizia") && !e.to_string().contains("giustizia"), "{e}");
}
```

Il rinomino non ha un test proprio: i test esistenti di `fondazione.rs`, `coalizione.rs` e `cli.rs`, riscritti coi nomi nuovi, sono la verifica.

- [ ] **Step 2: Esegui e verifica che fallisca**

Run: `cargo test -p sim-core --test ministri i_messaggi_dei_ministri_nominano_il_dicastero`
Expected: FAIL, il messaggio contiene `sanita`.

- [ ] **Step 3: Rinomina, sistema i messaggi, alza lo schema**

Il rinomino tocca anche i commenti e i nomi dei test che dicono «posto» per l'asse (`sei_posti_fra_i_sette_partiti` → `sei_posizioni_fra_i_sette_partiti`). Restano i «posti» del calendario.

- [ ] **Step 4: Esegui e verifica che passi**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/plans crates app/src/lib/tipi
git commit -m "refactor: posizione al posto di posto, nomi dei dicasteri nei messaggi, schema 8"
```

### Task 2: Debiti del 7a

**Files:**
- Modify: `crates/sim-core/src/ministri/mod.rs:126-142` (`StatoDicastero.rosa_tecnica`), `:273-300` (`apri`), `crates/sim-core/src/ministri/soddisfazione.rs:58-66`
- Modify: `crates/sim-core/src/partita.rs:958-979` (`promuovi_referendum`), `:1570-1595` (`allarga`), `:1747-1817` (`forma_governo`, intesa del governo IA)
- Modify: `crates/sim-core/src/comando.rs:82-86` (`Allarga { partiti }`)
- Modify: `crates/sim-core/src/partiti.rs:147-155` (`valida`)
- Modify: `crates/sim-core/tests/comune/mod.rs` (helper `con_seggi`, `in_trattativa`)
- Test: `crates/sim-core/tests/debiti_7a.rs`, `crates/sim-core/tests/governo_ia.rs`, `crates/sim-core/tests/spartizione.rs` (usi di `Allarga`)

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:135` (nessuna richiesta nei sei mesi dopo la convocazione dei comizi); `docs/ricerca/modelli/formazione-di-coalizioni.md:13` (coalizione minima connessa vincente: i partner entrano insieme).

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `Quadro::voci` (`crates/sim-core/src/legislatura/coalizione.rs:235`), `Partita::dopo_il_voto` (`crates/sim-core/src/partita.rs:1549`), `Ministri::genera` (`crates/sim-core/src/ministri/mod.rs:211`).
- Produces:
  - `StatoDicastero.rosa_tecnica: bool`: vale `true` quando esce un ministro tecnico (dimissioni in `soddisfazione::passo`), e da qui in avanti in ogni uscita. `apri` genera la rosa con `genera(cx, caso, i, st.rosa_tecnica)` e lo azzera quando entra il successore.
  - `promuovi_referendum`, dopo il controllo delle Camere sciolte: se `!legislatura.elezioni.is_empty() && turno <= legislatura.inizio`, rifiuta con «Nessuna richiesta di referendum nei sei mesi dopo la convocazione dei comizi».
  - `Comando::Allarga { partiti: Vec<String> }`: rifiuta con lista vuota («Allarga: nessun partito»), partito sconosciuto, ripetuto o già nella coalizione; disponibilità di ciascuno calcolata con la coalizione più tutta la lista (`Quadro::voci` con `d.partiti` esteso); rifiuto con «{nome} non entra: {causa}» per ogni membro non disponibile, uniti da «; ». Entrano tutti o nessuno.
  - `partiti::valida`: «coalizione di meno di due partiti: {ids uniti da +}» per una coalizione dello scenario con 0 o 1 partiti.
  - `forma_governo` con l'incarico IA o il Capo dello Stato: ogni alleato del governo nuovo riparte da `intesa_base`, senza variazione registrata.
  - Helper di test in `tests/comune/mod.rs`:
    ```rust
    /// Seggi scritti a mano nell'ordine dei partiti; le somme devono restare i `totali`.
    pub fn con_seggi(p: &Partita, camera: &[u32], senato: &[u32]) -> Partita
    /// La partita vera (posizione 4,5) dopo il voto, in trattativa con l'incarico al giocatore,
    /// con `coalizioni` (JSON di indici) e seggi scritti a mano.
    pub fn in_trattativa(coalizioni: serde_json::Value, camera: &[u32], senato: &[u32]) -> Partita
    ```
    `in_trattativa` fonda, chiude il turno del voto e con `edita` scrive `legislatura.incarico = "Giocatore"`, `legislatura.coalizioni` e i seggi.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/debiti_7a.rs
mod comune;
use comune::{dati_base, edita, fondata, in_trattativa};
use sim_core::{Comando, Partita};

#[test]
fn il_successore_di_un_tecnico_e_un_tecnico() {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| j["ministri"]["dicasteri"][3]["ministro"]["tecnico"] = serde_json::json!(true)).unwrap();
    p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap();
    p.chiudi_turno();
    let st = &p.ministri().dicasteri[3];
    assert!(st.ministro.is_none());
    assert_eq!(st.rosa.len(), 3);
    assert!(st.rosa.iter().all(|x| x.tecnico));
}

#[test]
fn nessun_referendum_nei_sei_mesi_dopo_i_comizi() {
    let mut p = fondata(1.5);
    p.chiudi_turno(); // voto: turno 1 = trattativa, inizio = 1
    let e = p.promuovi_referendum("sinistra", "licenziamento").unwrap_err();
    assert!(e.to_string().contains("sei mesi"), "{e}");
}

/// Partiti 3 (centro, 4,0) e 4 (centrodestra, 5,0) accanto al giocatore (4,5): da soli non
/// bastano, insieme sì (Camera 210 su 400, Senato 105 su 200).
const CAMERA: [u32; 8] = [10, 80, 50, 30, 30, 40, 10, 150];
const SENATO: [u32; 8] = [5, 40, 25, 15, 15, 20, 5, 75];

#[test]
fn due_partner_che_bastano_insieme_entrano_insieme() {
    let mut p = in_trattativa(serde_json::json!([[7]]), &CAMERA, &SENATO);
    let prima = p.salva().unwrap();
    let e = p.comanda(Comando::Allarga { partiti: vec!["centro".into()] }).unwrap_err();
    assert!(e.to_string().contains("Centro"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    p.comanda(Comando::Allarga { partiti: vec!["centro".into(), "centrodestra".into()] }).unwrap();
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(3) && p.governo().al_governo(4));
}
```

```rust
// crates/sim-core/tests/dati.rs (in coda)
#[test]
fn coalizione_di_un_partito_solo_rifiutata() {
    let f = comune::sostituisci(comune::file_base(), "scenario",
        "[\"sinistra\", \"centrosinistra\"]", "[\"sinistra\"]");
    let e = sim_core::dati::DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("sinistra"), "{e}");
}
```

```rust
// crates/sim-core/tests/governo_ia.rs (in coda)
#[test]
fn gli_alleati_del_governo_ia_partono_dalla_base() {
    let mut p = fondata(1.5);
    p.chiudi_turno(); // voto
    let n = p.partiti().len();
    let mut p = comune::edita(&p, |j| j["intesa"]["valori"] = serde_json::json!(vec![30.0; n])).unwrap();
    p.chiudi_turno(); // trattativa: governo IA
    let base = comune::dati_base().manopola("intesa_base");
    for j in p.alleati() {
        assert!((p.intesa().valori[j] - base).abs() < 1e-9, "{j}");
    }
}
```

In `due_partner_che_bastano_insieme_entrano_insieme` le somme dei seggi (400 e 200) devono coincidere con i `totali` del voto; se la legge in vigore al voto non dà 400 e 200, il test scala i vettori sui `legislatura().totali`. Se il centro risultasse disponibile da solo con le manopole dei dati, il test lo segnala e sceglie un partner più lontano: il caso che conta è che la lista entri.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test debiti_7a --test dati --test governo_ia`
Expected: FAIL; `Allarga { partiti }` non compila finché il comando non cambia.

- [ ] **Step 3: Implementa i cinque debiti**

Gli usi esistenti di `Allarga { partito }` nei test (`governo_ia.rs`, `spartizione.rs`) diventano `Allarga { partiti: vec![…] }`.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "fix(core): debiti del piano 7a (tecnici, comizi, allargamento in gruppo, coalizioni, intesa IA)"
```

### Task 3: Intesa completa, ultimatum come stato, rottura dell'impegno

**Files:**
- Modify: `crates/sim-core/src/parlamento/intesa.rs` (cause nuove, `ultimatum`, `soglie` che rende chi esce)
- Modify: `crates/sim-core/src/parlamento/voto.rs:15-32` (`linea_fiducia` legge lo stato)
- Modify: `crates/sim-core/src/legislatura/coalizione.rs` (`rotture`)
- Modify: `crates/sim-core/src/partita.rs:298-303` (pressioni sull'intesa nelle soglie), `:1747-1817` (rotture alla formazione), `:1938-1959` (`voci_alleati`), `crates/sim-core/src/comando.rs:141-158` (`ChiediDimissioni` a un alleato)
- Modify: `crates/sim-core/src/vista.rs:107-115` (`VoceAlleato.causa_ultimatum`), `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Modify: `crates/sim-core/tests/comune/mod.rs` (helper `al_governo`)
- Test: `crates/sim-core/tests/intesa.rs`

**Ricerca:** `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:21` (i partiti di governo perdono nei sondaggi nella prima metà: la pressione sugli alleati); `docs/ricerca/modelli/formazione-di-coalizioni.md:48` (il punteggio di una coalizione premia l'incumbent: cambiare campo costa).

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true. Legge `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md:26-29`.

**Interfaces:**
- Consumes: `Intesa::varia` (`crates/sim-core/src/parlamento/intesa.rs:60`), `richiesta` (`crates/sim-core/src/legislatura/spartizione.rs:32`), `causa_peggiore` (usata in `crates/sim-core/src/partita.rs:1954`), `EsitoElezioni.voti` (`crates/sim-core/src/legislatura/voto.rs:27`).
- Produces:
  - `CausaIntesa` guadagna, con `Display`: `MinistriScontenti` «ministri del suo partito scontenti», `DimissioniChieste` «dimissioni chieste a un suo ministro», `QuotaScoperta` «dicasteri sotto la sua richiesta», `SondaggiInCalo` «in calo nei sondaggi al governo», `RotturaImpegno` «rottura dell'impegno pre-elettorale», `SfiduciaRespinta` «sfiducia respinta: la maggioranza si ricompatta» (quest'ultima la usa il Task 4).
  - `Intesa.ultimatum: Vec<Option<CausaIntesa>>`, una voce per partito. `Intesa::soglie(&mut self, dati, turno, alleati: &[usize]) -> Vec<usize>`: per ogni alleato, sul valore prima del ritorno, sotto `soglia_ultimatum` e senza ultimatum, `ultimatum[j] = Some(causa peggiore delle variazioni di j rimaste, o NormaLontana se non ce ne sono)`; alla soglia o sopra, `None`. Rende gli alleati sotto `soglia_uscita`, in ordine d'indice. Poi il ritorno, come oggi. Chi non è alleato ha `ultimatum = None`. `controlla` verifica una voce per partito.
  - `linea_fiducia` e `voci_alleati` leggono `ultimatum[j].is_some()` invece di confrontare l'intesa con la soglia; `VoceAlleato.causa_ultimatum: Option<CausaIntesa>`.
  - Pressioni nelle soglie di `chiudi_turno`, dopo `ministri.soglie` e prima di `intesa.soglie`, per ogni alleato `j` del governo in carica (giocatore o IA):
    - `MinistriScontenti`: con almeno un ministro di `j` in carica, `−costo_ministri_scontenti · max(0, soglia_minaccia − media) / 10`, `media` = soddisfazione media dei suoi ministri.
    - `QuotaScoperta`: `−costo_quota_scoperta · max(0, richiesta − offerta)`, con `richiesta` di Gamson sui seggi veri della coalizione di governo e `offerta` = punti di peso dei dicasteri che `j` tiene.
    - `SondaggiInCalo`: dopo almeno un voto, `−costo_calo_sondaggi · max(0, quota al voto − quota nel sondaggio) · 100`, con le quote fra i votanti e quella del sondaggio come media degli istituti (`quota_fra_i_votanti`, `crates/sim-core/src/legislatura/fondazione.rs`).
    - Una variazione si registra solo se il delta non è zero.
  - `ChiediDimissioni` accolto su un dicastero il cui ministro è di un alleato e non è tecnico: `DimissioniChieste` di `−costo_dimissioni_chieste` a quell'alleato, nello stesso comando.
  - `pub(crate) fn rotture(ultime: &[Vec<usize>], coalizione: &[usize], entranti: &[usize]) -> Vec<usize>` in `legislatura/coalizione.rs`, in ordine d'indice. Rompe `j ∈ coalizione` se (a) `j ∈ entranti`, ha corso in una coalizione di almeno due partiti e un suo compagno di quella coalizione non sta in `coalizione`; oppure (b) un entrante diverso da `j` ha corso in una coalizione di almeno due partiti diversa da quella di `j`. Chi ha corso da solo è libero.
  - In `forma_governo`, con `entranti` = tutta la coalizione nuova: ogni partito che rompe perde `valence_rottura_impegno` di valence (anche il giocatore, decisione 3), e ogni alleato che rompe ha `RotturaImpegno` di `−intesa_rottura_impegno`, applicata dopo il ripristino dell'intesa di partenza.
  - Manopole (tutte `ticket = "05"`, assunzione del piano 7b): `soglia_uscita` 25 (spec riga 333; min 0, max 40), `costo_ministri_scontenti` 2,0 (0-10), `costo_quota_scoperta` 2,0 (0-10), `costo_calo_sondaggi` 1,0 (0-5), `costo_dimissioni_chieste` 10,0 (0-30), `valence_rottura_impegno` 0,3 (0-2), `intesa_rottura_impegno` 10,0 (0-30).
  - Helper di test: `pub fn al_governo() -> Partita` in `tests/comune/mod.rs`, ossia il `vinta()` di `tests/spartizione.rs:14` più la chiusura della trattativa; `spartizione.rs` lo riusa.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/intesa.rs
mod comune;
use comune::{dati_base, edita, indice};
use sim_core::parlamento::CausaIntesa;
use sim_core::{Comando, Partita};

fn variazioni(p: &Partita, j: usize, c: CausaIntesa) -> Vec<f64> {
    (p.intesa().variazioni.iter()).filter(|v| v.partito == j && v.causa == c).map(|v| v.delta).collect()
}

#[test]
fn dimissioni_chieste_a_un_alleato_costano_intesa() {
    let mut p = Partita::nuova(dati_base(), 1);
    let cd = indice("centrodestra");
    p.comanda(Comando::ChiediDimissioni { dicastero: "ambiente_energia".into() }).unwrap();
    let costo = dati_base().manopola("costo_dimissioni_chieste");
    assert_eq!(variazioni(&p, cd, CausaIntesa::DimissioniChieste), vec![-costo]);
    // Un ministro del partito del Presidente del Consiglio: nessun alleato paga.
    p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap();
    assert_eq!(variazioni(&p, cd, CausaIntesa::DimissioniChieste).len(), 1);
}

#[test]
fn ministri_scontenti_costano_intesa() {
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        for k in [6, 9] {
            j["ministri"]["dicasteri"][k]["ministro"]["soddisfazione"] = serde_json::json!(30.0);
        }
    }).unwrap();
    p.chiudi_turno();
    let v = variazioni(&p, indice("centrodestra"), CausaIntesa::MinistriScontenti);
    assert!(v.len() == 1 && v[0] < 0.0, "{v:?}");
}

#[test]
fn la_quota_sotto_la_richiesta_pesa_ogni_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    let cd = indice("centrodestra");
    let pl = p.parlamento();
    let s = |j: usize| f64::from(pl.seggi_camera[j] + pl.seggi_senato[j]);
    let dati = dati_base();
    let richiesta = (dati.manopola("gamson_intercetta")
        + dati.manopola("gamson_pendenza") * s(cd) / (s(cd) + s(indice("destra")))) * 14.0;
    let offerta = 3.0; // Ambiente ed Energia 1, Esteri e Difesa 2
    p.chiudi_turno();
    let v = variazioni(&p, cd, CausaIntesa::QuotaScoperta);
    if richiesta > offerta {
        let atteso = -dati.manopola("costo_quota_scoperta") * (richiesta - offerta);
        assert!(v.len() == 1 && (v[0] - atteso).abs() < 1e-9, "{v:?} {atteso}");
    } else {
        assert!(v.is_empty());
    }
}

#[test]
fn il_calo_nei_sondaggi_al_governo_costa_intesa() {
    let p = comune::al_governo();
    let j = p.alleati()[0];
    // Una quota al voto più alta di cinque punti: l'alleato risulta in calo.
    let mut p = edita(&p, |x| {
        let q = x["legislatura"]["elezioni"][0]["voti"][j]["quota"].as_f64().unwrap();
        x["legislatura"]["elezioni"][0]["voti"][j]["quota"] = serde_json::json!(q + 0.05);
    }).unwrap();
    p.chiudi_turno();
    let v = variazioni(&p, j, CausaIntesa::SondaggiInCalo);
    assert!(v.len() == 1 && v[0] < 0.0, "{v:?}");
}

#[test]
fn l_ultimatum_resta_con_la_sua_causa() {
    let p = Partita::nuova(dati_base(), 1);
    let cd = indice("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(45.0)).unwrap();
    p.comanda(Comando::ChiediDimissioni { dicastero: "ambiente_energia".into() }).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    let a = v.alleati.iter().find(|a| a.partito == "centrodestra").unwrap();
    assert!(a.ultimatum);
    assert_eq!(a.causa_ultimatum, Some(CausaIntesa::DimissioniChieste));
    assert!(p.governo().al_governo(cd)); // sopra 25: non esce
}
```

```rust
// crates/sim-core/src/legislatura/coalizione.rs, mod tests (in coda)
#[test]
fn rompe_chi_lascia_un_compagno_o_governa_con_l_altro_campo() {
    let ultime = [vec![7], vec![1, 2], vec![4, 5]];
    // 2 lascia 1; 7 governa con un partito dell'altro campo; 7 aveva corso da solo ma paga (b).
    assert_eq!(rotture(&ultime, &[2, 7], &[2, 7]), vec![2, 7]);
    // Allargare a chi ha corso da solo non rompe nulla.
    assert_eq!(rotture(&[vec![4, 5], vec![3]], &[3, 4, 5], &[3, 4, 5]), Vec::<usize>::new());
    // 5 governa senza il suo compagno 4.
    assert_eq!(rotture(&[vec![4, 5]], &[5, 6], &[5, 6]), vec![5]);
    // Un entrante che aveva corso da solo non fa rompere chi governa già.
    assert_eq!(rotture(&[vec![4, 5], vec![6]], &[5, 6], &[6]), Vec::<usize>::new());
}
```

Test di integrazione della rottura, in `tests/intesa.rs`: `in_trattativa(json!([[7], [1, 2], [4, 5]]), …)` con seggi scritti a mano perché il giocatore più il centrosinistra (2) abbiano la maggioranza. `Allarga { partiti: ["centrosinistra"] }` e chiusura: il centrosinistra ha una variazione `RotturaImpegno`, e la valence del giocatore, confrontata con la stessa partita chiusa senza allargamento, scende di `valence_rottura_impegno · (1 − ritorno_valence)`. Se il centrosinistra non è disponibile coi seggi scelti, il test gli dà più seggi.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test intesa && cargo test -p sim-core --lib rompe_chi_lascia`
Expected: FAIL, cause e funzioni assenti.

- [ ] **Step 3: Implementa cause, ultimatum, rotture e manopole**

La causa peggiore dell'ultimatum si sceglie con la stessa regola di `causa_peggiore`. Il test `gli_alleati_del_governo_ia_partono_dalla_base` del Task 2 si aggiorna: un alleato che rompe l'impegno parte da `intesa_base − intesa_rottura_impegno`. I test esistenti che leggono `VoceAlleato.ultimatum` o la linea sotto fiducia con l'intesa scritta a mano (`tests/emendamenti.rs`, `tests/calendario.rs`) scrivono anche `intesa.ultimatum`, senza cambiare le asserzioni.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): intesa con tutte le cause, ultimatum come stato, rottura dell'impegno"
```

---

## PR B · Crisi (Task 4-6)

### Task 4: Sfiducia, dimissioni e Capo dello Stato

**Files:**
- Create: `crates/sim-core/src/legislatura/crisi.rs` (`linea_sfiducia`, valore della caduta)
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.dimissioni`, `Legislatura.maggioranza`, `Incarico::Guida`)
- Modify: `crates/sim-core/src/parlamento/mod.rs:42-48` (`FormaMisura::Sfiducia`)
- Modify: `crates/sim-core/src/partita.rs:223-346` (passi nuovi della chiusura), `:465-594` (fiducia respinta), `:1747-1817` (`forma_governo` con `maggioranza`), `:1678-1686` (`coalizione_del_giocatore`)
- Modify: `crates/sim-core/src/comando.rs` (`Dimettiti`)
- Modify: `crates/sim-core/src/vista.rs` (`dimissioni`, `VoceIncarico::Guida`), `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/caduta.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:42` (dopo la fiducia respinta le dimissioni sono prassi: la spec le rende automatiche), `:43` (sfiducia motivata, un decimo delle firme, appello nominale), `:86` (esercizio provvisorio per legge e al più quattro mesi: un turno); `docs/ricerca/modelli/formazione-di-coalizioni.md:13` (maggioranza minima connessa del Capo dello Stato); `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:47` (il voto anticipato come scommessa).

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true. Legge `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md:35-37`.

**Interfaces:**
- Consumes: `maggioranza_contigua` (`crates/sim-core/src/legislatura/spartizione.rs:95`), `Quadro::termini` (`crates/sim-core/src/legislatura/coalizione.rs:168`), `Intesa.ultimatum` (Task 3), `esercizio_provvisorio` (`crates/sim-core/src/economia/finanza.rs:283`).
- Produces:
  - `Legislatura.dimissioni: bool` (il governo cade alla chiusura del turno) e `Legislatura.maggioranza: Option<Vec<usize>>` (la coalizione del ribaltone col giocatore, in trattativa). `coalizione_del_giocatore` rende `maggioranza` quando c'è.
  - `Incarico::Guida { coalizione: Vec<usize> }`: il giocatore sta nella maggioranza del ribaltone ma non è il primo. Fino al Task 6, `forma_governo` lo tratta come l'incarico al Capo dello Stato (maggioranza contigua senza il giocatore). `VoceIncarico::Guida`.
  - `FormaMisura::Sfiducia`. L'esito porta nome «Mozione di sfiducia al governo» o «Mozione di sfiducia al ministro di {dicastero}», conteggi a blocchi (i seggi dei partiti del sì e del no), `approvata` se il sì supera il no in almeno una Camera, linee per partito.
  - `pub(crate) fn linea_sfiducia(n: usize, governo: &Governo, ultimatum: &[Option<CausaIntesa>], partito_del_ministro: Option<usize>, cade: impl Fn(usize) -> bool) -> Vec<bool>` in `legislatura/crisi.rs`, pura. Votano sì i partiti fuori dalla coalizione di governo. Fra gli alleati in ultimatum, sulla sfiducia individuale votano sì quelli diversi dal partito del ministro; su quella al governo vota sì chi ha `cade(j)`. Tutti gli altri no. `Partita` le passa `|j| vota_la_caduta(&self.quadro(), j, …)`.
  - `pub(crate) fn vota_la_caduta(q: &Quadro, j: usize, attuale: &[usize], presidente: usize) -> bool` (scelta 12): `valore(dopo) > valore(restare)`, con i termini e le manopole del 7a. `restare` = `peso_seggi_coalizione · s_j/S + peso_governare − peso_distanza_coalizione · d(j, attuale) − peso_intesa_coalizione · max(0, intesa_base − intesa_j)/100`. `dopo` = con `maggioranza_contigua(…, Some(presidente))` che contiene `j`: `peso_seggi_coalizione · s_j/S + peso_governare − peso_distanza_coalizione · d(j, quella) + valore_guida · [j primo per seggi in quella]`; altrimenti i termini di `j` al voto proiettato sul sondaggio con le coalizioni dell'ultimo voto, `j` tolto dalla coalizione di `presidente`.
  - `fn sfiducia(&mut self, dicastero: Option<usize>, promotore: Option<usize>) -> bool` su `Partita`: registra l'esito e rende se è passata. Passata al governo: `dimissioni = true`. Passata al ministro: il ministro esce (`Causa::Sfiduciato`, «sfiduciato dalle Camere») e, se è di un alleato, `DimissioniChieste` di `−costo_dimissioni_chieste`. Respinta: ogni alleato `SfiduciaRespinta` di `+intesa_sfiducia_respinta`, il promotore perde `valence_sfiducia_respinta`. La chiamano il Task 5 (fine della crisi) e il Task 7 (sfiducia del giocatore); qui si testa la sua linea.
  - Cadute: `Comando::Dimettiti` (guardia di governo, ammesso anche in crisi) mette `dimissioni = true`. Una fiducia respinta in `vota` fa lo stesso (il commento a `partita.rs:556` diventa la regola). Dopo l'aula, in fase ordinaria, se `esercizio_provvisorio(data)` vale e non c'è un bilancio approvato per l'anno in corso, `dimissioni = true`: vale per il bilancio respinto come per quello mai presentato. Il segnaposto del governo IA (`partita.rs:228-238`) presenta la bozza anche nell'esercizio provvisorio.
  - Dopo le soglie, se `dimissioni` e il turno non è di voto: `fn capo_dello_stato(&mut self)`. Con `escluso` = il giocatore se presiedeva lui, altrimenti nessuno (scelta 11), cerca `maggioranza_contigua`. Se la trova senza il giocatore: `fase = Trattativa`, `Incarico::Ia { coalizione }`. Se il giocatore ci sta ed è il primo per seggi (Camera più Senato): `Incarico::Giocatore` e `maggioranza = Some(coalizione)`. Se ci sta ma non è il primo: `Incarico::Guida { coalizione }`. Se non la trova: `fase = Sciolta`. Poi `dimissioni = false`. Nel turno di voto `dimissioni` si azzera senza effetto: vince il voto.
  - `forma_governo` azzera `maggioranza`. Nessuna luna di miele al governo nuovo (ticket 29, riga 50).
  - `Vista.dimissioni: bool`.
  - Manopole (`ticket = "12"`, assunzione del piano 7b): `intesa_sfiducia_respinta` 5,0 (0-20), `valence_sfiducia_respinta` 0,1 (0-1).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/caduta.rs
mod comune;
use comune::{con_seggi, dati_base, presenta};
use sim_core::legislatura::{Fase, Incarico};
use sim_core::{Comando, Partita};

/// La destra e il centrodestra in minoranza: 150 su 400, 75 su 200.
const MINORANZA_C: [u32; 7] = [40, 80, 60, 30, 50, 100, 40];
const MINORANZA_S: [u32; 7] = [20, 40, 30, 15, 25, 50, 20];

#[test]
fn dimettersi_porta_al_ribaltone_o_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.comanda(Comando::Dimettiti).unwrap();
    assert!(p.vista().dimissioni);
    p.chiudi_turno();
    let l = p.legislatura();
    match l.fase {
        Fase::Trattativa => {
            let Some(Incarico::Ia { coalizione }) = &l.incarico else { panic!("{:?}", l.incarico) };
            assert!(!coalizione.contains(&p.giocatore()));
            p.chiudi_turno();
            assert!(!p.governa() && p.legislatura().fase == Fase::Ordinaria);
        }
        Fase::Sciolta => assert!(p.legislatura().turno_di_voto(p.vista().turno)),
        Fase::Ordinaria => panic!("il governo non è caduto"),
    }
}

#[test]
fn fiducia_respinta_fa_dimettere() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.fiducia_respinta));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
}

#[test]
fn bilancio_mancato_anche_in_esercizio_provvisorio() {
    let mut p = con_seggi(&Partita::nuova(dati_base(), 1), &MINORANZA_C, &MINORANZA_S);
    for _ in 0..4 {
        p.chiudi_turno(); // fino a ottobre-dicembre 2027
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
    p.chiudi_turno(); // respinto: esercizio provvisorio nel 2028
    assert!(p.governa());
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
    p.chiudi_turno(); // respinto di nuovo: dimissioni
    assert!(!p.governa());
}

#[test]
fn dimissioni_nel_turno_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    while p.vista().turno < 19 {
        p.chiudi_turno();
    }
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno(); // si vota
    let l = p.legislatura();
    assert_eq!(l.elezioni.len(), 1);
    assert_eq!(l.fase, Fase::Trattativa);
    assert!(!l.dimissioni);
}
```

Test d'unità in `legislatura/crisi.rs`, `la_sfiducia_e_deterministica_a_blocchi`: sette partiti, governo di 4 e 5 con 5 presidente. Senza ultimatum la linea è sì per 0-3 e 6, no per 4 e 5, qualunque sia `cade`. Con 4 in ultimatum: sulla sfiducia al governo 4 vota sì solo con `cade(4) = true`; su quella individuale a un ministro di 5 vota sì, a un ministro di 4 no.

Se i totali del parlamento uscente non sono 400 e 200, `con_seggi` riceve vettori scalati sui `legislatura().totali`, e il commento delle costanti lo dice. Se con la minoranza scritta a mano la fiducia passa lo stesso, il test controlla la linea nell'anteprima del calendario (`alleati_contro_fiducia`, linea dell'opposizione no) prima di toccare i seggi.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test caduta && cargo test -p sim-core --lib sfiducia`
Expected: FAIL, comando e campi assenti.

- [ ] **Step 3: Implementa cadute, sfiducia e Capo dello Stato**

L'ordine della chiusura nel commento di `chiudi_turno` (`partita.rs:211-222`) si aggiorna: aula (con la fiducia respinta) → esercizio provvisorio mancato → motore → voto → soglie → Capo dello Stato se ci sono dimissioni → governo nuovo nel turno di trattativa.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): dimissioni, sfiducia a blocchi e Capo dello Stato"
```

### Task 5: Uscita, rimpasto e crisi

**Files:**
- Modify: `crates/sim-core/src/legislatura/spartizione.rs:47-77` (`spartizione_di_default` con i dicasteri già tenuti)
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.crisi`)
- Modify: `crates/sim-core/src/ministri/mod.rs` (`dimetti`, cause `UscitaDalGoverno` e `Rimpasto`)
- Modify: `crates/sim-core/src/partita.rs:298-306` (uscite e fine della crisi), `:1570-1595` (`allarga` in crisi), `crates/sim-core/src/comando.rs:94-160` (guardia dell'ordinaria amministrazione)
- Modify: `crates/sim-core/src/vista.rs` (`crisi`)
- Test: `crates/sim-core/tests/crisi.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:16` (Gamson ricalcolata nel rimpasto), `:21` (pendenza e intercetta); `docs/ricerca/modelli/iter-legislativo-italiano.md:43` (sfiducia per appello nominale, palese).

**Dispatch:** `rust-engineer` · dopo il Task 4 · skill-gate: true. Legge `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md:31-33`.

**Interfaces:**
- Consumes: `Intesa::soglie` che rende gli uscenti e `rotture` (Task 3); `sfiducia`, `capo_dello_stato` e `dimissioni` (Task 4); `maggioranza` (`crates/sim-core/src/legislatura/spartizione.rs:80`).
- Produces:
  - `spartizione_di_default(dati, coalizione, guida, seggi, tenuti: &[Option<usize>]) -> Vec<usize>`: assegna solo i dicasteri `None` di `tenuti`, con le richieste scoperte al netto del peso già tenuto; con tutti `None` è la funzione di oggi. I chiamanti esistenti passano `&vec![None; n]`, e il test d'unità esistente resta invariato.
  - `Ministri::dimetti(&mut self, i: usize, causa: Causa, turno: u32)`: interim con `causa_dimissioni`, rosa vuota (la riapre `apri`), proposta e agenda cancellate, decreto in agenda ritirato come `ritira`. Le nuove `Causa` hanno `Display`: `UscitaDalGoverno` «il suo partito ha lasciato la maggioranza», `Rimpasto` «il dicastero passa a un altro partito».
  - `Legislatura.crisi: Option<u32>`: il turno di crisi in corso.
  - Uscite, dopo `intesa.soglie`, in fase ordinaria, senza dimissioni e fuori dal turno di voto. Per ogni uscente `j`: i suoi ministri escono con `UscitaDalGoverno`, `j` lascia `governo.coalizione` e `intesa.ultimatum[j]` torna `None`. Se la coalizione rimasta ha la maggioranza, `fn rimpasto(&mut self)`: `tenuti` = i dicasteri di partiti ancora in coalizione diversi dal presidente, `None` gli altri; i dicasteri che cambiano partito vanno in interim con `Rimpasto` (quelli già in interim restano in interim col partito nuovo). Altrimenti i dicasteri di `j` passano al presidente, in interim, e `crisi = Some(turno + 1)`.
  - Fine della crisi, dopo le uscite, quando `crisi == Some(turno)`. Con la maggioranza (coalizione più `allargati`): gli allargati entrano in `governo.coalizione`, `rotture` con `entranti` = allargati, `rimpasto`, `crisi = None`. Senza: `sfiducia(None, None)`, `crisi = None`. Con `dimissioni` già vere la sfiducia automatica non si vota (Review Focus 1).
  - Nel turno di voto la crisi si chiude senza sfiducia: vince il voto, che azzera `crisi` in `vota_parlamento`.
  - Ordinaria amministrazione: con `crisi == Some(turno)`, `PresentaLegge`, `PresentaBilancio`, `EmanaDecreto`, `PoniFiducia`, `Accogli` e le anteprime di legge e decreto rifiutano con «Crisi di governo: solo ordinaria amministrazione».
  - `Allarga` ammesso anche al giocatore presidente in crisi, con la disponibilità sui seggi veri della coalizione di governo più la lista. Gli entrati stanno in `allargati` fino alla fine della crisi.
  - `Vista.crisi: bool`; `Vista.disponibilita` anche in crisi, per i partiti fuori dalla coalizione.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/crisi.rs
mod comune;
use comune::{con_seggi, dati_base, edita, indice, presenta};
use sim_core::legislatura::Fase;
use sim_core::parlamento::FormaMisura;
use sim_core::{Comando, Partita};

/// Destra 100 e centrodestra 101 alla Camera (201 su 400), 50 e 51 al Senato: senza il
/// centrodestra la destra resta sola; l'estrema destra (110, 55) basta a rifare la maggioranza.
const CRISI_C: [u32; 7] = [10, 40, 29, 10, 101, 100, 110];
const CRISI_S: [u32; 7] = [5, 20, 14, 5, 51, 50, 55];

fn in_crisi() -> Partita {
    let p = con_seggi(&Partita::nuova(dati_base(), 1), &CRISI_C, &CRISI_S);
    let cd = indice("centrodestra");
    let mut p = edita(&p, |j| j["intesa"]["valori"][cd] = serde_json::json!(10.0)).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().crisi, Some(1));
    p
}

#[test]
fn un_alleato_sotto_25_esce_e_apre_la_crisi() {
    let p = in_crisi();
    let cd = indice("centrodestra");
    assert!(!p.governo().al_governo(cd));
    for k in [6, 9] {
        assert_eq!(p.governo().dicasteri[k], p.governo().presidente);
        assert!(p.ministri().dicasteri[k].ministro.is_none());
    }
    assert!(p.vista().crisi);
}

#[test]
fn in_crisi_solo_ordinaria_amministrazione() {
    let mut p = in_crisi();
    let prima = p.salva().unwrap();
    let e = presenta(&mut p, &[("tetto_contante", "1000")]).unwrap_err();
    assert!(e.to_string().contains("ordinaria amministrazione"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
    p.comanda(Comando::Ferma { dicastero: "sanita".into() }).unwrap();
}

#[test]
fn crisi_senza_allargamento_finisce_in_sfiducia() {
    let mut p = in_crisi();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.forma == FormaMisura::Sfiducia && e.approvata));
    assert!(!p.governa());
    assert_ne!(p.legislatura().fase, Fase::Ordinaria);
}

#[test]
fn allargare_in_crisi_salva_il_governo() {
    let mut p = in_crisi();
    p.comanda(Comando::Allarga { partiti: vec!["estrema_destra".into()] }).unwrap();
    p.chiudi_turno();
    assert!(p.governa());
    assert!(p.governo().al_governo(indice("estrema_destra")));
    assert_eq!(p.legislatura().crisi, None);
    // Il rimpasto dà dicasteri al nuovo alleato.
    assert!(p.governo().dicasteri.contains(&indice("estrema_destra")));
}

#[test]
fn rimpasto_se_la_maggioranza_regge() {
    // Il centro (3) entra con la Sanità (3), poi esce: destra e centrodestra reggono.
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        j["governo"]["dicasteri"][3] = serde_json::json!(3);
        j["governo"]["coalizione"].as_array_mut().unwrap().push(serde_json::json!(3));
        j["ministri"]["dicasteri"][3]["ministro"]["partito"] = serde_json::json!(3);
        j["intesa"]["valori"][3] = serde_json::json!(10.0);
    }).unwrap();
    p.chiudi_turno();
    assert!(!p.governo().al_governo(3));
    assert_ne!(p.governo().dicasteri[3], 3);
    assert!(p.ministri().dicasteri[3].ministro.is_none());
    assert_eq!(p.legislatura().crisi, None);
    assert!(p.governa());
}

#[test]
fn rimpasto_con_interim_si_ricarica() {
    // Come sopra, con le dimissioni chieste a un ministro della destra nello stesso turno.
    let p = Partita::nuova(dati_base(), 1);
    let mut p = edita(&p, |j| {
        j["governo"]["dicasteri"][3] = serde_json::json!(3);
        j["governo"]["coalizione"].as_array_mut().unwrap().push(serde_json::json!(3));
        j["ministri"]["dicasteri"][3]["ministro"]["partito"] = serde_json::json!(3);
        j["intesa"]["valori"][3] = serde_json::json!(10.0);
    }).unwrap();
    p.comanda(Comando::ChiediDimissioni { dicastero: "giustizia".into() }).unwrap();
    p.chiudi_turno();
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
}

#[test]
fn due_cadute_nello_stesso_turno() {
    let mut p = in_crisi();
    p.comanda(Comando::Dimettiti).unwrap();
    p.chiudi_turno();
    assert!(!p.vista().esiti.iter().any(|e| e.forma == FormaMisura::Sfiducia));
    assert!(!p.governa());
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

#[test]
fn salvare_a_meta_crisi() {
    let mut a = in_crisi();
    a.comanda(Comando::Allarga { partiti: vec!["estrema_destra".into()] }).unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}
```

`rimpasto_se_la_maggioranza_regge` e `rimpasto_con_interim_si_ricarica` presuppongono che destra e centrodestra abbiano la maggioranza nel parlamento uscente: se non ce l'hanno, usano `con_seggi` con `CRISI_C` e `CRISI_S` più dieci seggi spostati dall'estrema destra al centro. Se nel `in_crisi` l'estrema destra non fosse disponibile, il test lo dice nel messaggio: è un esito delle manopole del 7a, da riportare, non da forzare.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test crisi`
Expected: FAIL, nessuno esce.

- [ ] **Step 3: Implementa uscite, rimpasto, crisi e la sua fine**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): uscita degli alleati, rimpasto e turno di crisi"
```

### Task 6: Ribaltone a favore del giocatore e guida

**Files:**
- Modify: `crates/sim-core/src/legislatura/coalizione.rs` (`Quadro::valore`)
- Modify: `crates/sim-core/src/partita.rs` (`chiedi_guida`, `rifiuta_ribaltone`, `forma_governo` con `Incarico::Guida`)
- Modify: `crates/sim-core/src/comando.rs` (`ChiediGuida`, `RifiutaRibaltone`)
- Test: `crates/sim-core/tests/ribaltone.rs`, `crates/sim-core/tests/comune/mod.rs` (helper `caduto`)

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:22` (il formateur prende la presidenza: la guida è la posta), `:48` (partito più grande e incumbent nel punteggio); `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:22` (niente luna di miele per chi torna).

**Dispatch:** `rust-engineer` · dopo il Task 5 · skill-gate: true. Legge `.scratch/v1-giocabile/issues/12-opposizione.md:51-54`.

**Interfaces:**
- Consumes: `capo_dello_stato`, `Incarico::Guida`, `Legislatura.maggioranza` (Task 4); `Quadro::termini`.
- Produces:
  - `pub(crate) fn Quadro::valore(&self, d: &Dichiarazione, j: usize) -> f64`: la somma dei termini di `j` con `d`.
  - `Comando::ChiediGuida`: ammesso in trattativa con `Incarico::Guida { coalizione }`. Il primo per seggi `l` della coalizione accetta se `valore` coi seggi veri e la coalizione del ribaltone (guida al giocatore) supera `valore` al voto proiettato sul sondaggio pubblico con le coalizioni dell'ultimo voto. Se accetta: `Incarico::Giocatore` e `maggioranza = Some(coalizione)`, poi la spartizione si propone come dopo il voto. Se rifiuta: `Errore::Comando` «{nome} non ti cede la guida: {causa}», partita identica; la causa è il termine con lo scarto più grande, come in `Quadro::voci`.
  - `Comando::RifiutaRibaltone`: ammesso con `Incarico::Guida`; `incarico = None`, `maggioranza = None`. La chiusura cerca allora la maggioranza contigua senza il giocatore, altrimenti scioglie.
  - `forma_governo` con `Incarico::Guida` ancora aperto applica il default (decisione 1): valuta la guida come `ChiediGuida`. Se `l` accetta, governa il giocatore con la spartizione di default; se no, vale il ramo del Capo dello Stato. Guardia: chi non ha l'incarico giusto riceve «Ribaltone: nessuna guida da chiedere».
  - Helper di test in `tests/comune/mod.rs`:
    ```rust
    /// La partita vera (posizione 4,5) nel turno dopo il voto, riportata con `edita` in fase
    /// ordinaria sotto il governo uscente (destra e centrodestra), con `incarico = null`, seggi
    /// scritti a mano e `dimissioni = true`; poi la chiusura, che chiama il Capo dello Stato.
    pub fn caduto(camera: &[u32], senato: &[u32]) -> Partita
    ```

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/ribaltone.rs
mod comune;
use comune::caduto;
use sim_core::legislatura::{Fase, Incarico};
use sim_core::Comando;

// Ordine sull'asse: 0 (1), 1 (2), 2 (3), 3 (4), giocatore 7 (4,5), 4 (5), 5 (6), 6 (7).
/// Nessuna coppia ha la maggioranza; fra le terne vince (7, 4, 5), la più grande (240 e 120):
/// il giocatore è il primo.
const PRIMO_C: [u32; 8] = [10, 40, 40, 50, 40, 80, 20, 120];
const PRIMO_S: [u32; 8] = [5, 20, 20, 25, 20, 40, 10, 60];
/// Fra le terne pareggiano (2, 3, 7) e (3, 7, 4), 315 seggi: vince la più a sinistra, (2, 3, 7),
/// dove il centro (3) ha più seggi del giocatore.
const SECONDO_C: [u32; 8] = [10, 40, 40, 120, 40, 80, 20, 50];
const SECONDO_S: [u32; 8] = [5, 20, 20, 60, 20, 40, 10, 25];

#[test]
fn primo_nel_ribaltone_ottieni_l_incarico() {
    let mut p = caduto(&PRIMO_C, &PRIMO_S);
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
    assert_eq!(p.legislatura().incarico, Some(Incarico::Giocatore));
    assert!(p.richieste().iter().any(|r| r.partito == "centrodestra"));
    p.chiudi_turno();
    assert!(p.governa());
}

#[test]
fn secondo_nel_ribaltone_chiedi_la_guida() {
    let mut p = caduto(&SECONDO_C, &SECONDO_S);
    assert!(matches!(p.legislatura().incarico, Some(Incarico::Guida { .. })));
    match p.comanda(Comando::ChiediGuida) {
        Ok(()) => {
            assert_eq!(p.legislatura().incarico, Some(Incarico::Giocatore));
            p.chiudi_turno();
            assert!(p.governa());
        }
        Err(e) => assert!(e.to_string().contains("Centro"), "{e}"),
    }
}

#[test]
fn senza_scelta_il_default_chiede_la_guida() {
    let mut a = caduto(&SECONDO_C, &SECONDO_S);
    let mut b = caduto(&SECONDO_C, &SECONDO_S);
    let accetta = b.comanda(Comando::ChiediGuida).is_ok();
    a.chiudi_turno();
    assert_eq!(a.governa(), accetta);
}

#[test]
fn rifiutare_il_ribaltone_lascia_il_giocatore_fuori() {
    let mut p = caduto(&SECONDO_C, &SECONDO_S);
    p.comanda(Comando::RifiutaRibaltone).unwrap();
    p.chiudi_turno();
    assert!(!p.governa());
    assert!(!p.governo().al_governo(p.giocatore()));
}
```

Se con i seggi di `SECONDO_C` il centro non accetta, `secondo_nel_ribaltone_chiedi_la_guida` verifica il rifiuto col nome, e un secondo test con i seggi del centro ridotti a 60 (e 30 al Senato, i 60 e 30 di differenza alla sinistra) verifica che accetti. Il report riporta quale dei due casi si è visto.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test ribaltone`
Expected: FAIL, comandi assenti.

- [ ] **Step 3: Implementa guida, rifiuto e default**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): ribaltone a favore del giocatore, guida chiesta come condizione"
```

---

## PR C · Opposizione (Task 7-9)

### Task 7: Comandi dell'opposizione

**Files:**
- Modify: `crates/sim-core/src/comando.rs` (`Linea`, `Sfiducia`, `PresentaProposta`, `PromuoviReferendum`, guardia `all_opposizione`)
- Modify: `crates/sim-core/src/parlamento/mod.rs:54-66` (`Misura.promotore`, `Misura.linea_giocatore`), `:113-125` (`Parlamento.sfiducia`), `:276-351` (`previsione` con la linea del giocatore), `:508-549` (`presenta` nel posto riservato)
- Modify: `crates/sim-core/src/parlamento/emendamenti.rs:108-140` (niente emendamenti sulle proposte dell'opposizione)
- Modify: `crates/sim-core/src/partita.rs:406-421` (sfiducia del giocatore dopo le leggi), `:465-594` (costi della respinta solo per il governo, percepita del giocatore)
- Modify: `crates/sim-core/src/vista.rs` (`posto_opposizione`, `alleati_avversari`, `VoceMisura.promotore`, `VoceMisura.linea_giocatore`), `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/opposizione.rs`, `crates/sim-core/tests/comune/mod.rs` (helper `all_opposizione`)

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:119` (un quinto degli argomenti dell'aula alle opposizioni), `:126` (le proposte dell'opposizione passano di rado), `:135` (blocchi del referendum), `:142` (sfiducia individuale: un decimo, appello nominale), `:154` (la sfiducia individuale serve a dividere la maggioranza).

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true. Legge `.scratch/v1-giocabile/issues/12-opposizione.md:16-49`.

**Interfaces:**
- Consumes: `sfiducia` e `linea_sfiducia` (Task 4), `Partita::promuovi_referendum` (`crates/sim-core/src/partita.rs:958`), `Parlamento::bozza` (`crates/sim-core/src/parlamento/mod.rs:559`), `probabilita_franco` (`crates/sim-core/src/parlamento/voto.rs:70`).
- Produces:
  - `fn all_opposizione(&self) -> Result<(), Errore>`: rifiuta a partita finita («La partita è finita»), quando il giocatore governa («Comando dell'opposizione: governi tu»), a Camere sciolte («Comando dell'opposizione: Camere sciolte») e in trattativa («Comando dell'opposizione: trattativa in corso»). Ammesso in fase ordinaria sotto un governo IA, anche in crisi.
  - `Comando::Linea { misura: u32, si: Option<bool> }`: `None` torna al default. Rifiuta una misura sconosciuta. Valida su ogni forma: `previsione` usa la scelta per il partito del giocatore, e i suoi franchi tiratori sono `min(ft_max, ft_base + ft_distanza · |x_g − sostenuta| / 6)`, con `sostenuta` = la posizione nuova col sì, la vigente col no, `ft_base` senza posizione. Dopo il voto la percepita del giocatore scivola verso `sostenuta` di `scivolamento_percepita` (scelta 14).
  - `Comando::Sfiducia { dicastero: Option<String> }`: una per turno («una sfiducia è già presentata in questo turno»). Firme: seggi alla Camera dei partiti fuori dalla coalizione di governo, almeno `totale / 10` per eccesso, altrimenti «servono le firme di un decimo della Camera: {n} su {m}». Dicastero sconosciuto o in interim: rifiuto col nome. Sta in `Parlamento.sfiducia: Option<(Option<usize>, usize)>` (dicastero, promotore) e si vota nell'aula dopo referendum e firme, con `sfiducia(dicastero, Some(giocatore))`.
  - `pub fn Partita::anteprima_sfiducia(&self, dicastero: Option<&str>) -> Result<AnteprimaSfiducia, Errore>` con `pub struct AnteprimaSfiducia { pub firme: bool, pub linee: Vec<VoceLinea>, pub passa: bool }` (TS). Stessi rifiuti del comando tranne «già presentata»; non tocca nulla. È la stessa linea che il voto userà, se nulla cambia prima della chiusura.
  - `Comando::PresentaProposta { valori: Vec<ValoreProposto> }`: solo nel turno in cui il posto riservato è del giocatore (scelta 15), altrimenti «il posto riservato di questo turno è di {nome}»; una sola proposta del giocatore in calendario («il posto riservato è già occupato»); costituzionale rifiutata («una norma costituzionale non entra nel posto riservato»). Le regole di `bozza` valgono con `proponente = None` e `promotore = Some(giocatore)`. Il voto è a `t + 1`, con la linea. Sulla proposta respinta nessun costo, nessuna valence, solo il blocco; il promotore vota sì per default. Non occupa i tre posti ordinari.
  - `Comando::PromuoviReferendum { disposizione: String }`: `all_opposizione`, poi `promuovi_referendum(id del giocatore, disposizione)`.
  - `Dimettiti`, `ChiediGuida`, `RifiutaRibaltone`, `Allarga`: le loro guardie restano quelle dei Task 4-6.
  - `Vista.posto_opposizione: bool`; `Vista.alleati_avversari: Vec<VoceFascia>` sotto un governo IA, con `pub struct VoceFascia { pub partito: String, pub fascia: FasciaIntesa, pub causa_ultimatum: Option<CausaIntesa> }` e `pub enum FasciaIntesa { Solida, Incrinata, Ultimatum }`: `Ultimatum` con lo stato, `Incrinata` sotto `soglia_incrinata`, `Solida` altrimenti. La causa si vede solo in ultimatum (ticket 12, riga 19). `VoceMisura.promotore: Option<String>`, `VoceMisura.linea_giocatore: Option<bool>`.
  - Manopola `soglia_incrinata` 55,0 (min 40, max 90, `ticket = "12"`, assunzione del piano 7b).
  - Helper di test: `pub fn all_opposizione() -> Partita` in `tests/comune/mod.rs`: `fondata(1.5)`, due chiusure (voto e trattativa), con l'asserzione che il giocatore non governi (come `tests/governo_ia.rs:chi_perde_il_voto_va_all_opposizione`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/opposizione.rs
mod comune;
use comune::{all_opposizione, dati_base, fondata};
use sim_core::parlamento::{FormaMisura, ValoreProposto};
use sim_core::{Comando, Partita};

fn al_posto_riservato(p: &mut Partita) {
    for _ in 0..8 {
        if p.vista().posto_opposizione {
            return;
        }
        p.chiudi_turno();
    }
    panic!("il posto riservato non arriva mai al giocatore");
}

fn valori(v: &[(&str, &str)]) -> Vec<ValoreProposto> {
    v.iter().map(|(d, g)| ValoreProposto { disposizione: (*d).into(), gradino: (*g).into() }).collect()
}

#[test]
fn comandi_dell_opposizione_rifiutati_a_chi_governa() {
    for mut p in [Partita::nuova(dati_base(), 1), fondata(4.5)] {
        let prima = p.salva().unwrap();
        for c in [
            Comando::Sfiducia { dicastero: None },
            Comando::PresentaProposta { valori: valori(&[("licenziamento", "reintegro")]) },
            Comando::PromuoviReferendum { disposizione: "licenziamento".into() },
            Comando::Linea { misura: 0, si: Some(false) },
        ] {
            let e = p.comanda(c).unwrap_err();
            assert!(e.to_string().contains("opposizione"), "{e}");
        }
        assert_eq!(p.salva().unwrap(), prima);
    }
    let mut p = all_opposizione();
    let e = p.comanda(Comando::Dimettiti).unwrap_err();
    assert!(e.to_string().contains("opposizione"), "{e}");
}

#[test]
fn la_sfiducia_respinta_ricompatta_e_costa_valence() {
    let mut p = all_opposizione();
    let a = p.anteprima_sfiducia(None).unwrap();
    assert!(a.firme);
    let mut gemella = Partita::carica(&p.salva().unwrap()).unwrap();
    p.comanda(Comando::Sfiducia { dicastero: None }).unwrap();
    p.chiudi_turno();
    gemella.chiudi_turno();
    let e = p.vista().esiti.into_iter().find(|e| e.forma == FormaMisura::Sfiducia).unwrap();
    assert_eq!(e.approvata, a.passa); // ciò che l'anteprima mostrava
    if !a.passa {
        let g = p.giocatore();
        let dati = dati_base();
        let atteso = dati.manopola("valence_sfiducia_respinta") * (1.0 - dati.manopola("ritorno_valence"));
        let persa = gemella.partiti()[g].valence - p.partiti()[g].valence;
        assert!((persa - atteso).abs() < 1e-9, "{persa}");
    }
}

#[test]
fn la_proposta_dell_opposizione_va_al_voto_senza_costi() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta { valori: valori(&[("licenziamento", "reintegro")]) }).unwrap();
    let e = p.comanda(Comando::PresentaProposta { valori: valori(&[("tetto_contante", "1000")]) }).unwrap_err();
    assert!(e.to_string().contains("posto riservato"), "{e}");
    let m = &p.vista().calendario[0];
    assert_eq!(m.promotore.as_deref(), Some("giocatore"));
    assert!(m.emendamenti.is_empty());
    let pdc = p.governo().presidente;
    let v = p.partiti()[pdc].valence;
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.nome.contains("Licenziament")), "{:?}", p.vista().esiti);
    // Respinta o approvata, la proposta non tocca la valence del Presidente del Consiglio
    // oltre il ritorno verso la base.
    let r = dati_base().manopola("ritorno_valence");
    let base = p.partiti()[pdc].valence_base;
    let attesa = base + (v - base) * (1.0 - r) * (1.0 - r);
    assert!((p.partiti()[pdc].valence - attesa).abs() < 1e-9);
}

#[test]
fn la_linea_scelta_si_vede_e_si_vota() {
    let mut p = all_opposizione();
    al_posto_riservato(&mut p);
    p.comanda(Comando::PresentaProposta { valori: valori(&[("licenziamento", "reintegro")]) }).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Linea { misura: id, si: Some(false) }).unwrap();
    assert_eq!(p.vista().calendario[0].linea_giocatore, Some(false));
    p.chiudi_turno();
    p.chiudi_turno();
    let e = p.vista().esiti.into_iter().find(|e| e.misura == id).unwrap();
    assert!(!e.linee.iter().find(|l| l.partito == "giocatore").unwrap().si);
}

#[test]
fn il_giocatore_promuove_il_referendum() {
    let mut p = all_opposizione(); // turno 2, aprile-giugno 2027: fuori dai sei mesi
    p.comanda(Comando::PromuoviReferendum { disposizione: "licenziamento".into() }).unwrap();
    assert_eq!(p.vista().referendum[0].promotore.as_deref(), Some("giocatore"));
}

#[test]
fn l_intesa_avversaria_si_vede_in_fasce() {
    let p = all_opposizione();
    let v = p.vista();
    assert_eq!(v.alleati_avversari.len(), p.alleati().len());
    assert!(v.alleati.is_empty()); // nessuna cifra
}
```

Se `licenziamento`/`reintegro` non è presentabile all'opposizione col seme 7 (disposizione in esame o già al gradino), il test sceglie la prima disposizione referendabile del catalogo con un gradino diverso dal vigente e lo commenta. Il frammento «Licenziament» si adatta al nome della disposizione nei dati.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test opposizione`
Expected: FAIL, comandi assenti.

- [ ] **Step 3: Implementa guardia, linea, sfiducia, posto riservato e referendum**

La guardia di governo resta com'è: all'opposizione dice già «non governi: sei all'opposizione» (`comando.rs:104`), e `Dimettiti` passa da lì.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): comandi dell'opposizione (linea, sfiducia, posto riservato, referendum)"
```

### Task 8: Emendamenti degli alleati al bilancio

**Files:**
- Modify: `crates/sim-core/src/parlamento/emendamenti.rs:14-21` (`Attenuazione::Indicizza`), `:70-100` (testo), `:165-200` (`decidi_emendamento` sul bilancio)
- Modify: `crates/sim-core/src/partita.rs:932-937` (`presenta_bilancio` apre gli emendamenti), `:826-841` (`emendamento` sul bilancio)
- Modify: `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/bilancio_in_aula.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:87` (la legge di bilancio non conosce voto segreto e porta sempre la fiducia: la trattativa con gli alleati sta prima del voto).

**Dispatch:** `rust-engineer` · dopo il Task 7 · skill-gate: true.

**Interfaces:**
- Consumes: `Partita::indicizza` (`crates/sim-core/src/partita.rs:905`), `Voce::Dicastero` (`crates/sim-core/src/economia/legge_bilancio.rs:144`), `perdita_reale` (`:176`), `respingi_aperti` (`crates/sim-core/src/parlamento/emendamenti.rs:152`).
- Produces:
  - `Attenuazione::Indicizza`: sulle misure di bilancio l'indice della coppia è il dicastero. Testo «{nome del dicastero}: budget indicizzato all'inflazione»; motivo «{nome}: −{x}% reale», con una cifra decimale e la virgola.
  - In `presenta_bilancio`, col giocatore al governo, una richiesta per ogni alleato che tiene almeno un dicastero la cui voce perde più di `soglia_emendamento_bilancio` punti reali all'indice pubblicato; la proposta elenca quei dicasteri. Alla ripresentazione restano gli emendamenti già decisi e un accolto si riapplica al bilancio nuovo; quelli aperti si ricalcolano.
  - `Emendamento { misura, partito, accogli: true }` sul bilancio: `bilancio_presentato` diventa quello con le voci del partito indicizzate (`indicizza` voce per voce, poi `bilancio_ammesso`). Se il risultato non è ammesso, rifiuto col motivo del bilancio e partita identica. Respinto: costo e franchi tiratori come per le leggi (meccanismo esistente). Gli aperti a fine turno si respingono come oggi.
  - Manopola `soglia_emendamento_bilancio` 1,0 (punti reali; min 0, max 10; `ticket = "05"`, assunzione del piano 7b).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/bilancio_in_aula.rs (in coda)
fn al_bilancio_del_2028() -> Partita {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..4 {
        p.chiudi_turno(); // ottobre-dicembre 2027
    }
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
    p
}

#[test]
fn gli_alleati_chiedono_di_indicizzare_i_loro_dicasteri() {
    let p = al_bilancio_del_2028();
    let m = p.vista().calendario.into_iter().find(|m| m.forma == FormaMisura::Bilancio).unwrap();
    let e = m.emendamenti.iter().find(|e| e.partito == "centrodestra").unwrap();
    assert!(e.testo.contains("indicizzato"), "{}", e.testo);
}

#[test]
fn accolto_l_emendamento_la_voce_e_indicizzata() {
    let mut p = al_bilancio_del_2028();
    let id = p.vista().calendario.iter().find(|m| m.forma == FormaMisura::Bilancio).unwrap().id;
    let voce = |p: &Partita| p.economia().bilancio_presentato.as_ref().unwrap().dicasteri[6].spesa;
    let prima = voce(&p);
    p.comanda(Comando::Emendamento { misura: id, partito: "centrodestra".into(), accogli: true }).unwrap();
    assert!(voce(&p) > prima);
}

#[test]
fn un_bilancio_indicizzato_non_riceve_emendamenti() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let b = p.bozza_bilancio().unwrap();
    let b = p.indicizza(&b, &Voce::Tutto).unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
    let m = p.vista().calendario.into_iter().find(|m| m.forma == FormaMisura::Bilancio).unwrap();
    assert!(m.emendamenti.is_empty());
}
```

Se il campo della voce non si chiama `spesa`, il test usa il totale della voce del dicastero 6 (Ambiente ed Energia), come lo legge `riepilogo_bilancio`. Se la perdita reale della bozza a ottobre-dicembre 2027 col seme 1 sta sotto l'1%, il test abbassa `soglia_emendamento_bilancio` con `imposta` e lo commenta.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test bilancio_in_aula`
Expected: FAIL, nessun emendamento sul bilancio.

- [ ] **Step 3: Implementa richieste, accoglimento e ripresentazione**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): emendamenti degli alleati alla legge di bilancio"
```

### Task 9: Eredità e abbandono

**Files:**
- Create: `crates/sim-core/src/legislatura/eredita.rs` (`Eredita`, `ConfrontoIndicatore`)
- Modify: `crates/sim-core/src/legislatura/mod.rs:65-68` (`Fine::Abbandono`, TS)
- Modify: `crates/sim-core/src/partita.rs:79-101` (`turni_al_governo`, `leggi_del_giocatore`, `partenza`), `:223` (conteggio dei turni), `:630-649` e `:665-702` (leggi del giocatore), `abbandona`, `eredita`, `:2112` (controlli)
- Modify: `crates/sim-core/src/vista.rs` (`eredita`)
- Test: `crates/sim-core/tests/eredita.rs`

**Ricerca:** - nessun riferimento pertinente (ticket 12, riga 56, e 28, righe 29 e 35: contenuto dell'eredità, bozza, abbandono confermato).

**Dispatch:** `rust-engineer` · dopo il Task 8 · skill-gate: true.

**Interfaces:**
- Consumes: `Fine` (`crates/sim-core/src/legislatura/mod.rs:65`), `Disposizioni.voci[d].legge`, la `Vista` per i valori degli indicatori.
- Produces:
  - `pub enum Fine { SenzaSeggi, Abbandono }` (TS).
  - `pub struct Eredita { pub fine: Fine, pub bozza: bool, pub anni_al_governo: f64, pub leggi_in_vigore: Vec<String>, pub indicatori: Vec<ConfrontoIndicatore> }` e `pub struct ConfrontoIndicatore { pub id: String, pub nome: String, pub turno_zero: f64, pub fine: f64 }` (TS). `bozza` vale solo per `SenzaSeggi`. `leggi_in_vigore` = nomi delle disposizioni con `legge` in `leggi_del_giocatore`, nell'ordine dei dati. Gli `indicatori` sono quattro, con id `debito_pil` («Debito in % del PIL»), `spread` («Spread»), `disoccupazione` («Disoccupazione»), `inflazione` («Inflazione»).
  - `Partita` guadagna `turni_al_governo: u32` (+1 a ogni chiusura in cui `governa()` all'inizio), `leggi_del_giocatore: Vec<String>` (le stringhe `legge` scritte quando entra in vigore una misura del governo del giocatore, una sua proposta d'opposizione, o un confermativo di una sua legge) e `partenza: [f64; 4]` (i quattro indicatori al turno zero, scritti da `costruisci`). Una sola funzione privata `fn indicatori(&self) -> [f64; 4]` serve a `partenza` e alla fine.
  - `pub fn Partita::eredita(&self) -> Option<Eredita>`: `Some` solo a partita finita.
  - `pub fn Partita::abbandona(&mut self) -> Result<Eredita, Errore>`: a partita finita «La partita è finita», altrimenti `fine = Some(Abbandono)` e rende l'eredità.
  - `Vista.eredita: Option<Eredita>`.
  - `controlla_partiti` rifiuta `turni_al_governo` oltre `turno + 1` e `partenza` non finita.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/eredita.rs
mod comune;
use comune::{dati_base, presenta};
use sim_core::legislatura::Fine;
use sim_core::{Comando, Partita};

#[test]
fn abbandonare_produce_subito_l_eredita() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..4 {
        p.chiudi_turno();
    }
    let e = p.abbandona().unwrap();
    assert_eq!(e.fine, Fine::Abbandono);
    assert!(!e.bozza);
    assert!((e.anni_al_governo - 1.0).abs() < 1e-12);
    assert!(p.finita());
    assert_eq!(p.vista().eredita, Some(e));
    assert!(p.abbandona().is_err());
    assert!(p.comanda(Comando::Ferma { dicastero: "sanita".into() }).is_err());
}

#[test]
fn le_leggi_del_giocatore_in_vigore() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.misura == id && e.approvata));
    let nome = p.vista().disposizioni.iter().find(|d| d.id == "tetto_contante").unwrap().nome.clone();
    assert_eq!(p.abbandona().unwrap().leggi_in_vigore, vec![nome]);
}

#[test]
fn eredita_di_chi_non_ha_mai_governato() {
    // Come `senza_seggi_la_partita_finisce` del 7a (tests/voto_turno_zero.rs).
    let f = comune::imposta(comune::file_base(), "manopole", "[valence_partito_nuovo]\nvalore = ", "-3.0");
    let f = comune::imposta(f, "manopole", "[luna_di_miele]\nvalore = ", "0.0");
    let mut p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(), 7,
        &sim_core::legislatura::Fondazione { nome: "Partito nuovo".into(), posizione: 1.5 },
    ).unwrap();
    p.chiudi_turno();
    let e = p.eredita().unwrap();
    assert_eq!(e.fine, Fine::SenzaSeggi);
    assert!(e.bozza);
    assert_eq!(e.anni_al_governo, 0.0);
    assert!(e.leggi_in_vigore.is_empty());
    assert_eq!(e.indicatori.len(), 4);
    assert!(e.indicatori.iter().all(|i| i.turno_zero.is_finite() && i.fine.is_finite()));
    assert!(p.abbandona().is_err());
}

#[test]
fn al_turno_zero_gli_indicatori_non_cambiano() {
    let mut p = Partita::nuova(dati_base(), 1);
    let e = p.abbandona().unwrap();
    assert!(e.indicatori.iter().all(|i| i.turno_zero == i.fine), "{:?}", e.indicatori);
}
```

Se `tetto_contante`/`1000` non passa con la fiducia (alleato in ultimatum lontano), il test sceglie una disposizione vicina alla posizione della destra e lo commenta; la proprietà resta: la legge approvata del giocatore compare, nient'altro. Se `eredita_di_chi_non_ha_mai_governato` non finisce per seggi, usa la stessa via di riserva del test del 7a (`tests/voto_turno_zero.rs`, soglia di lista alzata).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test eredita`
Expected: FAIL, metodi assenti.

- [ ] **Step 3: Implementa eredità, abbandono e conteggi**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): eredità a fine partita e abbandono"
```

---

## Rimandato

- **Piano 8 (partiti IA):** condotta del governo avversario (programma, accoglimento degli emendamenti, fiducia, allargamento in crisi invece della sfiducia, voto anticipato), richieste degli alleati in ultimatum al suo bilancio, sfiducia dell'opposizione IA solo quando passa (spec riga 354), proposte-imbarazzo e referendum IA, vocazione identitaria che accelera il costo delle norme lontane (spec riga 333), referendum confermativo scelto dal giocatore con un quinto dei seggi (storia 97), crescita del malus d'opposizione nell'ultimo anno.
- **Piano 9 (campagna):** l'intesa che cala per le promesse su cui la linea dell'alleato sarebbe no; la disponibilità che legge le promesse.
- **Piano 10 (salvataggi):** riavvolgimenti, eredità confermata nell'albo, snapshot cancellati alla conferma.
- **Piano 11 (UI):** «avanti fino a novità» e le sue fermate (ticket 12, riga 14), schermate di crisi, sfiducia, ribaltone, opposizione ed eredità.
- **Piano 12 (bilanciamento):** tarare le manopole nuove dell'intesa, della sfiducia e degli emendamenti al bilancio contro i bersagli del costo del governare, e verificare che perdere apposta non convenga (ticket 29).
