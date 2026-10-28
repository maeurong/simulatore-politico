# V1 · Piano 8a: governo avversario e opposizione IA — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** I partiti IA giocano. Ognuno ha una vocazione e un leader che porta la valence e cambia dopo una sconfitta. Il governo avversario sostituisce il segnaposto del 7a: dichiara un programma pubblico e lo manda in aula, accoglie le proposte dei suoi ministri, presenta il bilancio indicizzato entro il tetto UE, decide emendamenti e fiducia con una regola sola (ridurre il rischio di cadere), in crisi allarga o si dimette, e va al voto anticipato quando i sondaggi glielo consigliano. L'opposizione IA, contro chiunque governi, presenta la sfiducia solo quando passerebbe, porta proposte-imbarazzo nel posto riservato e chiede referendum abrogativi che vincerebbero. Il giocatore all'opposizione sceglie il referendum confermativo; nell'ultimo anno il malus d'opposizione dei partiti IA cresce. Ogni mossa IA ha la sua riga nella vista.

**Architecture:** Le decisioni IA stanno in un sottomodulo nuovo, `crates/sim-core/src/partita/ia.rs` (`mod ia;` in `partita.rs`), che legge i campi privati della partita come un figlio del modulo. Il governo IA non ha regole sue: passa dagli stessi metodi dei comandi del giocatore, attraverso `Partita::applica`, cioè `comanda` senza la guardia `di_governo` (`comando.rs:181-263`). Le regole pure stanno vicino alle loro simili: la vocazione in `partiti.rs`, il programma in `legislatura/programma.rs`, il valore di stare al governo in `legislatura/crisi.rs`, la stima del referendum sul sondaggio in `elettorato/referendum.rs`, l'indicizzazione parziale in `economia/legge_bilancio.rs`. Le mosse IA avvengono in due punti della chiusura: in coda, nell'apertura del turno nuovo (governo e opposizione), e prima del Capo dello Stato (voto anticipato), più il referendum dell'opposizione IA, chiesto in aula prima delle firme.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 97 (riga 140), 100-101 e 103-107 (righe 149-150, 152-156); «Governo, ministri e coalizione» (riga 328, agenda e regione bersaglio; riga 333, intesa); «Parlamento» (righe 340, 343-344: linea col malus dell'ultimo anno, calendario, referendum); «Partiti IA» (righe 349-354). Ticket: `.scratch/v1-giocabile/issues/18-partiti-ia.md` (righe 26-59: vocazione, leader, governo avversario, opposizione IA), `12-opposizione.md` (referendum confermativo, leve simmetriche), `27-mappa-delle-20-regioni.md:34` (voti pesati dalla vocazione). Rimandi: piano 7b `:1224`, piano 7a `:1288` (governo IA vero), piano 6 `:30` e `:33`, piano 5 `:30`. Il resto del piano 8 (vita dei partiti, coalizioni IA che si ricompongono, regola di movimento, cassa, campagna, promesse: storie 102, 108-109, 122, 125-129) è del piano 8b, che si scrive a 8a fuso.

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · tratti | `feat/v1-piano-8a-tratti` | `main` | 1-3 | schema 9, vocazione e leader nei dati, la vocazione dove pesa, cambio di leader, registro delle mosse IA |
| B · governo | `feat/v1-piano-8a-governo` | `feat/v1-piano-8a-tratti` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 4-6 | `applica`, programma, governo IA in apertura, bilancio IA, crisi e voto anticipato |
| C · opposizione | `feat/v1-piano-8a-opposizione` | `feat/v1-piano-8a-governo` (impilata, stessa regola) | 7-9 | sfiducia IA, linea del Presidente del Consiglio sulle proposte dell'opposizione, proposta-imbarazzo, malus dell'ultimo anno, referendum IA, confermativo del giocatore |

Questo file entra nel primo commit della PR A.

## Decisioni di Mario

Del 28/10/2026, sulle domande di questo piano (memoria `decisioni-piano-8`):

1. **Divisione.** 8a = governo avversario, opposizione IA, vocazione, leader, spiegazioni, confermativo (storia 97), malus dell'ultimo anno. 8b = vita dei partiti, coalizioni IA, movimento, cassa, campagna, promesse. Il «Piano 9 (campagna)» del Rimandato del 7b (riga 1225) è un errore: vale la tabella del piano 1 (riga 49).
2. **Voto anticipato.** Dopo le dimissioni per il voto anticipato il Capo dello Stato cerca la maggioranza contigua senza il partito del Presidente del Consiglio dimissionario: se c'è è un ribaltone (anche a favore del giocatore), altrimenti scioglie. Le altre cadute di un governo IA restano come nella scelta 11 del 7b (Task 6).
3. **Opposizione IA contro chiunque governi.** Sfiducia, proposta-imbarazzo e referendum valgono anche contro un governo IA, col giocatore all'opposizione insieme a loro (Task 7-9).
4. **Emendamento pesato per vocazione.** Pesa sul costo d'intesa del respinto; la soglia per chiederlo resta `distanza_alleato` (Task 2).

Restano valide le decisioni dei piani 6, 7a e 7b (memorie `decisioni-piano-6-aperte`, `decisioni-piano-7`, `decisioni-piano-7b`).

## Scelte del piano, da confermare in review

1. **`VERSIONE_SCHEMA` passa a 9** (Task 1). Lo schema 8 non è mai uscito, ma i salvataggi di prova del 7b esistono: con lo schema nuovo sono rifiutati col messaggio della cucitura 2 invece che con un errore di forma.
2. **Vocazione come α di Lehrer-Schumacher** (`abm-elettorali.md:35`): identitaria 0,75, mista 0,5, governista 0,25, tre manopole. `peso_politiche = α / 0,5`, `peso_poltrone = (1 − α) / 0,5`: un partito misto pesa 1 e 1, e si comporta esattamente come nel 7b.
3. **Dove pesa la vocazione.** Nel valore atteso (`Quadro::termini`) la distanza va per `peso_politiche`, seggi, governare e guida per `peso_poltrone`: cambiano insieme disponibilità, guida ceduta e voto sulla sfiducia, che leggono gli stessi termini. Norma lontana ed emendamento respinto costano per `peso_politiche` (decisione 4). I voti nella regione bersaglio pesano per `peso_poltrone` (ticket 27, riga 34: «basso per un partito identitario, alto per uno governista»).
4. **Leader.** Il nome sta nello scenario; il partito del giocatore ha il leader vuoto, perché il leader è il giocatore e non cambia mai. Un partito IA cambia leader quando era il partito del Presidente del Consiglio e il governo nuovo non lo comprende più (resta alleato: nessun cambio), o quando al voto perde almeno `calo_cambio_leader` di quota fra i votanti sull'elezione precedente; una volta sola per chiusura. Il nuovo nome si estrae da `nomi` e `cognomi` dell'elettorato con `Flusso::PartitiIa`, finora mai usato. La valence riparte da `valence_base + luna_di_miele_leader` e si spegne col ritorno della valence.
5. **Mosse IA.** `Partita.mosse` si svuota all'inizio della chiusura; ogni mossa IA della chiusura e dell'apertura che segue vi aggiunge una riga. La vista del turno le mostra tutte.
6. **Nascita del governo avversario.** Coalizione e guida restano quelle di oggi (incarico dal voto, poi Capo dello Stato, guida al primo per seggi); la spartizione resta `spartizione_di_default`, già la richiesta di Gamson coi dicasteri interi; niente tecnici. Le coalizioni IA scelte col valore atteso sono dell'8b.
7. **Programma.** Si calcola una volta alla nascita del governo IA e resta fisso. Posizione della coalizione: la media delle posizioni dichiarate pesata coi seggi di Camera più Senato. Per ogni disposizione con posizione, non costituzionale, il gradino non bloccato più vicino a quella posizione, se avvicina il vigente di almeno `soglia_programma`; in ordine di distanza decrescente del vigente. In coda, al più una voce elettorale: la disposizione elettorale e il gradino che rendono più seggi attesi (Camera più Senato) alla coalizione, se almeno `seggi_programma_elettorale`. Le costituzionali restano fuori: hanno il loro posto e due deliberazioni.
8. **Quando decide l'IA.** Il governo e l'opposizione IA decidono in apertura, dopo agende, proposte ed emendamenti, così il giocatore vede le mosse e ha il turno per reagire. Fanno eccezione il voto anticipato, deciso in chiusura prima del Capo dello Stato, e il referendum dell'opposizione IA, chiesto in aula prima delle firme se nel turno non ne è stato chiesto un altro: il giocatore ha la precedenza.
9. **Proposte dei ministri IA.** Il Presidente del Consiglio avversario le accoglie tutte, con le regole di `Accogli`, dopo il programma: un ministro scontento costa intesa, e la regola sola è ridurre il rischio di cadere. Quelle rifiutate dalle regole restano in sospeso e cadono come oggi.
10. **Bilancio IA.** Bozza con i budget dei dicasteri indicizzati (gli scaglioni no: le aliquote cambiano solo per una disposizione del programma); se la crescita della spesa netta (`RiepilogoBilancio::crescita_spesa_netta`) supera il tetto, l'indicizzazione si riduce della stessa quota per tutti, cercata per bisezione in 20 passi. Poi i dicasteri degli alleati in ultimatum si indicizzano per intero, anche oltre il tetto. Il bilancio IA non riceve emendamenti: le richieste degli alleati in ultimatum sono già dentro.
11. **Emendamenti e fiducia IA.** «Senza i voti di quell'alleato» = la previsione con la linea dell'alleato a no; «sotto 40» = sotto `soglia_ultimatum`. Prima gli emendamenti, poi la fiducia, sulle leggi del governo col voto nel turno, se la forbice è in bilico e nessun alleato è in ultimatum.
12. **Crisi IA.** Nell'apertura del turno di crisi il governo IA allarga a uno dei due partiti contigui alla coalizione (la fila per posizione dichiarata dei partiti con seggi, senza il giocatore, mai alleato minore) che le ridà la maggioranza e ci guadagna: il più vicino di posizione, a parità il più grande. Uno solo. Altrimenti si dimette. La guardia degli allargati in `carica` accetta anche la crisi sotto un governo IA.
13. **Voto anticipato.** Valore del voto per il partito del Presidente del Consiglio: `Quadro::valore` sul sondaggio pubblico, con le coalizioni dell'ultimo voto. Valore del restare: `valore_al_governo` coi seggi veri e la guida. Si dimette se il primo supera il secondo di `margine_voto_anticipato`. Mai nel primo anno (`turno < inizio + 4`), mai a meno di 2 turni dalla scadenza (`scadenza − turno < 2`), mai in crisi, con dimissioni già in corso o nel turno del voto.
14. **Sfiducia IA.** In apertura, al governo se passerebbe sullo stato dell'apertura; altrimenti individuale, se passerebbe, al ministro del partito col giudizio medio peggiore fra i gruppi (pesato per elettori), a parità il dicastero di peso maggiore, mai di un alleato in ultimatum né in interim. Il giudizio è per partito, non per dicastero: `ponytail:` nel codice, con la via d'uscita (la colpa per dicastero registrata da `Elettorato::giudizio`). Firme: i seggi alla Camera dei partiti IA fuori dal governo, almeno un decimo. Promotore: il partito IA d'opposizione con più seggi alla Camera. Il giocatore all'opposizione la vede nel calendario e nel turno non ne presenta un'altra.
15. **Linea del Presidente del Consiglio sulle proposte dell'opposizione.** Oggi il suo partito vota sì d'ufficio anche su quelle (`parlamento/voto.rs:54`), e la proposta-imbarazzo passerebbe sempre. Sulle proposte dell'opposizione segue il guadagno, come ogni altro partito. Il giocatore al governo sceglie la sua con `Linea`, solo sulle proposte dell'opposizione, coi franchi tiratori della scelta 14 del 7b: no costa franchi tiratori se la proposta gli è vicina, sì la fa passare e costa intesa agli alleati lontani.
16. **Proposta-imbarazzo.** Candidati: ogni coppia (disposizione, gradino) di riforma (né elettorale né costituzionale) ammessa da `Parlamento::bozza`, col guadagno del promotore positivo. Vince quella che massimizza i seggi (Camera più Senato) dei partiti di governo con la linea sì; a parità il guadagno del promotore, poi l'indice della disposizione e del gradino.
17. **Referendum IA.** Stima sul sondaggio del trimestre: le posizioni degli incroci regionali, l'astensione dei dati dei gruppi e la mobilitazione dei toccati, con la stessa formula di `Elettorato::referendum`. Fra le disposizioni che `Parlamento::abrogativo` ammette, escluse le elettorali, quelle dove il sì vince e i votanti superano la metà degli aventi diritto; vince il margine sì − no più ampio. Promotore: il partito IA all'opposizione, con seggi, che guadagna di più dall'abrogazione; nessuno ci guadagna, nessun referendum.
18. **Confermativo del giocatore (storia 97).** La costituzionale approvata senza i due terzi va al confermativo anche quando il quinto contro non c'è, se il giocatore è fuori dal governo e il suo partito da solo, o coi partiti contrari, fa un quinto di una Camera. Il campo nuovo `Referendum::richiesto` vale la regola dell'08 (il quinto contro). Nel turno dopo il giocatore lo cambia con `Confermativo { misura, chiedi }`. Chiedere si può se il suo partito, da solo o coi contrari, fa un quinto; rinunciare lascia `richiesto` al quinto contro senza di lui. Non richiesto, in chiusura la legge entra in vigore senza voto.
19. **Malus dell'ultimo anno.** `malus_opposizione · (1 + crescita_malus_ultimo_anno)` per i partiti IA fuori dal governo, quando `Legislatura::ultimo_anno(turno)`; il giocatore no (ha la sua linea).

## Global Constraints

Valgono tutti i vincoli del piano 7b (`2026-10-27-v1-piano-7b-intesa-crisi-e-opposizione.md:54-67`) e, attraverso quelli, dei piani 1-7a: italiano coi termini di `CONTEXT.md` (vocazione, leader, programma, governo avversario, intesa, ultimatum, crisi, sfiducia, opposizione; «posizione» sull'asse, mai «posto», che resta per il calendario); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano il colpevole; niente `serde(default)`. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Un solo flusso casuale in uso nuovo, `Flusso::PartitiIa`, per i nomi dei leader. Le regole IA non estraggono altro: il «piccolo rumore col seme» della spec riga 349 è già nel sondaggio che leggono e nelle agende dei ministri.
- Le regole IA leggono quello che legge il giocatore: sondaggio pubblico, seggi veri, previsioni e forbici d'aula, intesa dei propri alleati. Mai le intenzioni vere né il bias comune.
- Le mosse IA passano dalle stesse funzioni dei comandi del giocatore (`applica`, `presenta_legge`, `poni_fiducia`, `emendamento`, `promuovi_referendum`, `Parlamento::presenta_proposta`): nessuna regola duplicata. Una mossa rifiutata dalle regole non cambia nulla e non lascia righe.
- Ogni mossa IA lascia una riga in `Partita.mosse`; i testi sono elencati nella PR per la revisione di Mario, come nella #26.
- Ogni manopola nuova ha `valore`, `min`, `max`, `ticket`, entra in `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:65`) nel task che la introduce e porta la fonte o `# assunzione del piano 8a, nessuna fonte in docs/ricerca`.
- La `Vista` non porta stato vero nuovo: programma, vocazioni, leader e mosse sono pubblici; delle mosse si scrive il motivo, mai i valori attesi in cifre.
- `VERSIONE_SCHEMA` passa a `9` nel Task 1 e resta `9` per tutto il piano.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Partita di prova (`Partita::nuova`): sette partiti (0 estrema sinistra … 6 estrema destra), il giocatore è la destra (5) al governo col centrodestra (4); scadenza 19; turno 0 = ottobre-dicembre 2026. Partita vera: il giocatore è l'indice 7; `all_opposizione()` (`tests/comune/mod.rs:468`) lo mette all'opposizione di un governo IA al turno 2.

## Review Focus

1. **Governo IA in crisi nel turno del bilancio o nell'esercizio provvisorio**: ordinaria amministrazione anche per l'IA (decisione 2 del 7b), nessun bilancio presentato, una sola caduta, nessun panic. Test `il_governo_ia_in_crisi_non_presenta_il_bilancio` nel Task 6.
2. **Programma esaurito o calendario pieno** (voci già in vigore, bloccate dal referendum, respinte da poco, in esame, posto riservato occupato): nessuna presentazione, nessuna riga, nessun errore. Test `programma_esaurito_non_presenta_nulla` nel Task 5.
3. **Due sfiducie nello stesso turno** (quella IA presentata in apertura e il comando del giocatore; quella IA nel turno di crisi con la sfiducia automatica): una sola mozione votata, una sola caduta. Test `una_sola_sfiducia_per_turno` nel Task 7.
4. **Voto anticipato fuori finestra** (primo anno, a un turno dalla scadenza, in crisi, già dimissionario, nel turno del voto): nessuna dimissione. Test `niente_voto_anticipato_fuori_finestra` nel Task 6.
5. **Salvataggio con lo stato nuovo** (programma, mosse, `voto_anticipato`, `richiesto`, leader cambiato): si ricarica identico, e `carica` rifiuta un programma con una disposizione o un gradino fuori dai dati. Test `lo_stato_ia_sopravvive_al_salvataggio` nel Task 5.

## Dispatch

PR A: 1 → 2 → 3 in sequenza; tutti toccano `partiti.rs` o `partita.rs`.

PR B: dopo la review della A, sul branch impilato, 4 → 5 → 6 in sequenza: il 5 usa programma e `applica` del 4, il 6 i passi d'apertura del 5.

PR C: dopo la review della B, 7 → 8 → 9 in sequenza: tutti toccano `partita/ia.rs`, l'8 e il 9 anche `parlamento/`.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task. A fine PR, review a sei in parallelo: `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer` (legge i testi delle mosse e i messaggi elencati nella PR), `rust-reviewer` (il diff tocca `.rs`) e `spec-reviewer` (questo piano), con i findings di quest'ultimo in un report a parte. Poi un solo giro di correzioni. Le decisioni prese in esecuzione si elencano nella PR. Il merge lo decide Mario.

---

## PR A · Tratti (Task 1-3)

### Task 1: Vocazione e leader nei dati, schema 9

**Files:**
- Modify: `crates/sim-core/src/partiti.rs` (`DatoPartito`, `Partito`, `iniziali`, `valida`; nuovo `enum Vocazione`)
- Modify: `dati/base/scenario.toml:220-295` (sette `[[partito]]`), `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65` (`MANOPOLE_RICHIESTE`)
- Modify: `crates/sim-core/src/legislatura/fondazione.rs` (`partito_nuovo`), `crates/sim-core/src/partita.rs:37` (`VERSIONE_SCHEMA = 9`), `crates/sim-core/src/vista.rs:340` (`VocePartito`), `partita.rs` (`voci_partiti`)
- Test: `crates/sim-core/tests/partiti_ia.rs` (nuovo), test unitari in `partiti.rs`

**Ricerca:** `docs/ricerca/modelli/abm-elettorali.md:35` (α fra politiche e ufficio, default 0,5); `docs/ricerca/sistemi/partiti-e-coalizioni.md:17` (il difetto da evitare: «all political parties play identically»).

**Dispatch:** `rust-engineer` · primo della PR A · skill-gate: true.

**Interfaces:**
- Consumes: nulla di nuovo.
- Produces:
  - `pub enum Vocazione { Identitaria, Mista, Governista }` in `partiti.rs`, `Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize`, `serde(rename_all = "snake_case")`, esportato in TS; `impl Vocazione { pub fn peso_politiche(self, dati: &DatiGioco) -> f64; pub fn peso_poltrone(self, dati: &DatiGioco) -> f64 }`, con α dalle manopole `alfa_identitaria` (0,75, 0,5-1), `alfa_mista` (0,5, 0,25-0,75), `alfa_governista` (0,25, 0-0,5), fonte `abm-elettorali.md:35`, ticket `18`.
  - `DatoPartito { …, pub vocazione: Vocazione, pub leader: String }`; `Partito { …, pub vocazione: Vocazione, pub leader: String }`; il partito del giocatore nasce `Mista` col leader vuoto.
  - `VocePartito { …, pub vocazione: Vocazione, pub leader: String }`.
  - `partiti::valida` rifiuta il leader vuoto: «leader vuoto: {id}».
  - Scenario: vocazione identitaria per estrema sinistra ed estrema destra, governista per il centro, mista per gli altri (spec riga 350). Leader segnaposto, nella PR per la revisione di Mario: Anna Ferrucci, Luca Benedetti, Chiara Lombardo, Giorgio Valli, Silvia Moretti, Davide Ranieri, Bruno Castellani, nell'ordine dello scenario.
  - `VERSIONE_SCHEMA = 9`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/partiti_ia.rs
mod comune;
use comune::*;
use sim_core::partiti::Vocazione;

#[test]
fn ogni_partito_ha_vocazione_e_leader() {
    let v = fondata(4.5).vista();
    let voce = |id: &str| v.partiti.iter().find(|p| p.id == id).unwrap().clone();
    assert_eq!(voce("estrema_sinistra").vocazione, Vocazione::Identitaria);
    assert_eq!(voce("estrema_destra").vocazione, Vocazione::Identitaria);
    assert_eq!(voce("centro").vocazione, Vocazione::Governista);
    assert_eq!(voce("sinistra").vocazione, Vocazione::Mista);
    assert_eq!(voce("destra").leader, "Davide Ranieri");
    assert_eq!(voce("giocatore").leader, "");
}

#[test]
fn leader_vuoto_rifiutato_col_nome() {
    let f = sostituisci(file_base(), "scenario.toml", "leader = \"Giorgio Valli\"", "leader = \"\"");
    let e = sim_core::dati::DatiGioco::da_testi(&f).unwrap_err();
    assert!(e.to_string().contains("leader vuoto: centro"), "{e}");
}
```

Nel modulo di test di `partiti.rs`: `i_pesi_della_vocazione`, coi dati base `peso_politiche` 1,5 / 1 / 0,5 e `peso_poltrone` 0,5 / 1 / 1,5 per identitaria, mista, governista (tolleranza 1e-12).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test partiti_ia`
Expected: FAIL, `vocazione` e `leader` assenti.

- [ ] **Step 3: Implementa tipi, dati, validazione e schema**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/plans/2026-10-28-v1-piano-8a-governo-e-opposizione-ia.md crates dati app/src/lib/tipi
git commit -m "feat(core): vocazione e leader dei partiti, schema 9"
```

---

### Task 2: La vocazione dove pesa

**Files:**
- Modify: `crates/sim-core/src/legislatura/coalizione.rs:169-217` (`Quadro::termini`)
- Modify: `crates/sim-core/src/legislatura/crisi.rs:47-101` (`vota_la_caduta`, estratto in `valore_al_governo`)
- Modify: `crates/sim-core/src/partita.rs:930-941` (`norma_lontana`), `:988-993` (`emendamenti_respinti`)
- Modify: `crates/sim-core/src/ministri/agenda.rs:110-151` (`termini_regioni`)
- Test: moduli di test di `partita.rs`, `legislatura/crisi.rs`, `ministri/agenda.rs`

**Ricerca:** `docs/ricerca/modelli/abm-elettorali.md:35` (α); `docs/ricerca/modelli/formazione-di-coalizioni.md:24` (le regole orientate all'ufficio formano governi più spesso sovradimensionati).

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `Vocazione::peso_politiche`, `peso_poltrone`, `Partito.vocazione` (Task 1).
- Produces:
  - In `Quadro::termini` per il partito `j`: `Seggi`, `Governare` e `Guida` × `peso_poltrone(j)`; `Distanza` × `peso_politiche(j)`; `Intesa` invariata.
  - `pub(crate) fn valore_al_governo(q: &Quadro, j: usize, coalizione: &[usize], guida: bool) -> f64` in `legislatura/crisi.rs`, coi seggi veri di `q`: `poltrone · (peso_seggi_coalizione · quota + peso_governare + valore_guida · [guida]) − politiche · peso_distanza_coalizione · distanza(coalizione, j)`. `vota_la_caduta` la usa per il restare (con il termine dell'intesa, che resta suo) e per la maggioranza contigua; la usano i Task 6 e 8.
  - `norma_lontana` ed `emendamenti_respinti`: il delta di ogni alleato × `peso_politiche` della sua vocazione.
  - `termini_regioni`: `peso_voti_bersaglio × peso_poltrone` del partito del ministro.

- [ ] **Step 1: Scrivi i test che falliscono**

Nel modulo di test di `partita.rs`, con la partita di prova e il centrodestra (4) alleato:

```rust
#[test]
fn la_vocazione_pesa_norma_lontana_ed_emendamento() {
    let calo = |v: Vocazione, f: fn(&mut Partita)| {
        let mut p = Partita::nuova(dati_base(), 7);
        p.partiti[4].vocazione = v;
        let prima = p.intesa.valori[4];
        f(&mut p);
        prima - p.intesa.valori[4]
    };
    let norma = |p: &mut Partita| p.norma_lontana(1.0);
    let emendamento = |p: &mut Partita| p.emendamenti_respinti([4]);
    for f in [norma as fn(&mut Partita), emendamento] {
        let mista = calo(Vocazione::Mista, f);
        assert!(mista > 0.0);
        assert!((calo(Vocazione::Identitaria, f) - 1.5 * mista).abs() < 1e-9);
        assert!((calo(Vocazione::Governista, f) - 0.5 * mista).abs() < 1e-9);
    }
}

#[test]
fn la_vocazione_pesa_i_termini_del_valore_atteso() {
    let mut p = Partita::nuova(dati_base(), 7);
    let d = Dichiarazione::default();
    let termini = |p: &Partita| {
        let q = p.quadro();
        q.termini(&d, &q.seggi(&d), 0)
    };
    p.partiti[0].vocazione = Vocazione::Mista;
    let mista = termini(&p);
    p.partiti[0].vocazione = Vocazione::Identitaria;
    let identitaria = termini(&p);
    for ((c, x), (_, y)) in mista.iter().zip(&identitaria) {
        let atteso = match c {
            CausaDisponibilita::Distanza => 1.5 * x,
            CausaDisponibilita::Intesa => *x,
            _ => 0.5 * x,
        };
        assert!((y - atteso).abs() < 1e-9, "{c:?}");
    }
}
```

`termini`, `seggi` e `CausaDisponibilita` vanno resi visibili al modulo di test di `partita.rs` (`pub(crate)`), se non lo sono. In `ministri/agenda.rs`: `i_voti_nel_bersaglio_seguono_la_vocazione`, coi termini dei voti del partito governista 1,5 volte quelli dello stesso partito misto e quelli del bisogno uguali. In `legislatura/crisi.rs`: `valore_al_governo_coi_pesi`, che con un partito misto rende la stessa somma del vecchio restare senza intesa.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --lib vocazione`
Expected: FAIL, i pesi non ci sono.

- [ ] **Step 3: Implementa i pesi**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS. I test esistenti di coalizione, crisi e agenda cambiano solo dove toccano un partito identitario o governista (estremi e centro): ogni valore atteso aggiornato si scrive nel commento del test col motivo «vocazione, piano 8a».

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): la vocazione pesa valore atteso, intesa e regione bersaglio"
```

---

### Task 3: Cambio di leader e registro delle mosse IA

**Files:**
- Create: `crates/sim-core/src/partita/ia.rs` (`mod ia;` in `partita.rs`; qui il solo registro, poi crescerà)
- Modify: `crates/sim-core/src/partita.rs` (campo `mosse`, `chiudi_turno:269`, `vota_parlamento:1753`, `forma_governo:2156`, `vista:2730`, `carica:2905`)
- Modify: `crates/sim-core/src/vista.rs` (`MossaIa`, `Vista.mosse`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/partiti_ia.rs`

**Ricerca:** `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:22` (luna di miele dei leader al primo mandato); `docs/ricerca/modelli/abm-elettorali.md:24` (il carisma del leader fra i parametri calibrati da Laver-Sergenti).

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true.

**Interfaces:**
- Consumes: `Partito.leader` (Task 1).
- Produces:
  - `pub struct MossaIa { pub partito: String, pub testo: String }` in `vista.rs` (`partito` è l'id), `Clone, Debug, PartialEq, Serialize, Deserialize`, esportata in TS; `Partita.mosse: Vec<MossaIa>`, svuotato all'inizio di `chiudi_turno`; `Vista.mosse: Vec<MossaIa>`.
  - `fn mossa(&mut self, j: usize, testo: String)` in `partita/ia.rs`: aggiunge la riga col partito `j`.
  - `fn cambia_leader(&mut self, j: usize, causa: &str)` in `partita/ia.rs`: nome «{nome} {cognome}» estratto con `Flusso::PartitiIa` da `dati.elettorato().nomi` e `cognomi`, diverso dal leader uscente; `valence = valence_base + luna_di_miele_leader`; riga «Nuovo leader: {nome}, dopo {causa}». Mai sul partito del giocatore.
  - Inneschi: in `vota_parlamento`, ogni partito IA con quota fra i votanti scesa di almeno `calo_cambio_leader` sull'elezione precedente (causa «il calo di {x} punti al voto», un decimale, virgola); in `forma_governo`, il partito del Presidente del Consiglio uscente, se IA e fuori dalla coalizione nuova (causa «la perdita del governo»). Una volta sola per partito e per chiusura.
  - Manopole: `luna_di_miele_leader` (0,328125, 0-3, fonte `cicli-politico-economici-e-luna-di-miele.md:22`, ticket `18`); `calo_cambio_leader` (0,04, 0,01-0,15, assunzione del piano 8a, ticket `18`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/partiti_ia.rs (in coda)
/// Dopo il voto del turno zero, in trattativa con l'incarico IA a estrema sinistra, sinistra e
/// centrosinistra: alla formazione la destra, partito del Presidente del Consiglio uscente, è
/// fuori dal governo. `forma_governo` prende la coalizione dell'incarico IA senza contare i seggi.
fn sinistra_al_governo() -> Partita {
    let mut p = fondata(4.5);
    p.chiudi_turno(); // voto
    edita(&p, |j| {
        j["legislatura"]["incarico"] = serde_json::json!({ "Ia": { "coalizione": [0, 1, 2] } });
    })
    .unwrap()
}

#[test]
fn chi_perde_il_governo_cambia_leader() {
    let mut p = sinistra_al_governo();
    let destra = partito("destra");
    assert_eq!(p.governo().presidente, destra);
    let prima = p.partiti()[destra].clone();
    p.chiudi_turno(); // trattativa: governa la sinistra
    let dopo = &p.partiti()[destra];
    assert_ne!(dopo.leader, prima.leader);
    let luna = dati_base().manopola("luna_di_miele_leader");
    assert!((dopo.valence - (dopo.valence_base + luna)).abs() < 1e-9);
    assert!(p.vista().mosse.iter().any(|m| m.partito == "destra" && m.testo.contains("la perdita del governo")));
}

#[test]
fn il_giocatore_non_cambia_mai_leader() {
    let mut p = all_opposizione();
    let g = p.giocatore();
    for _ in 0..24 {
        p.chiudi_turno();
        assert_eq!(p.partiti()[g].leader, "");
    }
}

#[test]
fn le_mosse_si_azzerano_a_ogni_chiusura() {
    let mut p = sinistra_al_governo();
    p.chiudi_turno();
    assert!(!p.vista().mosse.is_empty());
    p.chiudi_turno();
    assert!(p.vista().mosse.iter().all(|m| !m.testo.contains("la perdita del governo")));
}

#[test]
fn chi_resta_alleato_non_cambia_leader() {
    let mut p = vinta(); // la destra, se disponibile, entra nella coalizione del giocatore
    let destra = partito("destra");
    let prima = p.partiti()[destra].leader.clone();
    p.chiudi_turno();
    if p.governo().al_governo(destra) {
        assert_eq!(p.partiti()[destra].leader, prima);
    }
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test partiti_ia`
Expected: FAIL, `mosse` assente.

- [ ] **Step 3: Implementa registro, inneschi e cambio**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): leader che cambia dopo una sconfitta e registro delle mosse IA"
```

---

## PR B · Governo avversario (Task 4-6)

### Task 4: `applica` e programma del governo avversario

**Files:**
- Modify: `crates/sim-core/src/comando.rs:181-263` (`comanda` diviso in guardie + `applica`)
- Create: `crates/sim-core/src/legislatura/programma.rs` (`mod programma;` e ri-esportazioni in `legislatura/mod.rs`)
- Modify: `crates/sim-core/src/partiti.rs` (`Governo.programma`), `crates/sim-core/src/partita.rs:2156` (`forma_governo`), `:2730` (`vista`), `:3146-3155` (`controlla_partiti`)
- Modify: `crates/sim-core/src/vista.rs` (`VoceProgramma`, `Vista.programma`), `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/governo_ia.rs`, modulo di test di `legislatura/programma.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:16` (Gamson, già nella spartizione di default), `:22` (il formateur prende la presidenza); `docs/ricerca/sistemi/partiti-e-coalizioni.md:25` (partiti IA con posizioni proprie su più temi).

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true.

**Interfaces:**
- Consumes: `Quadro`, `StatoAula::seggi_attesi` (`parlamento/regole.rs:168`), `Disposizioni`, `DatoDisposizione::posizione(gradino) -> Option<f64>`.
- Produces:
  - `pub(crate) fn applica(&mut self, c: Comando) -> Result<(), Errore>` in `comando.rs`: il corpo di `comanda` dopo `di_governo`, con `fuori_dalla_crisi` sui cinque comandi di sempre. `comanda` resta la guardia del giocatore e la chiama. I comandi non di governo non passano da `applica`: `Comando::Dimettiti` sì.
  - `Governo { …, pub programma: Vec<(usize, usize)> }` (disposizione, gradino), vuoto col giocatore Presidente del Consiglio; `partiti::iniziali` lo crea vuoto.
  - `pub(crate) fn programma(aula: &StatoAula, coalizione: &[usize], seggi: &[u32]) -> Vec<(usize, usize)>` in `legislatura/programma.rs`, con la regola della scelta 7. Manopole: `soglia_programma` (0,5, 0-2, assunzione del piano 8a, ticket `18`), `seggi_programma_elettorale` (15, 0-200, assunzione del piano 8a, ticket `18`).
  - `forma_governo`: col Presidente del Consiglio IA, `programma(…)` sulla coalizione e i seggi veri, e la riga «Programma di governo: {n} disposizioni, prima {nome}».
  - `pub struct VoceProgramma { pub disposizione: String, pub nome: String, pub gradino: String, pub nome_gradino: String }` (id e nomi dai dati); `Vista.programma: Vec<VoceProgramma>`.
  - `controlla_partiti`: «governo.programma con una disposizione o un gradino fuori dai dati, o non vuoto col giocatore Presidente del Consiglio».

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/governo_ia.rs (in coda)
#[test]
fn il_governo_ia_nasce_col_programma_pubblico() {
    let p = all_opposizione();
    let v = p.vista();
    assert!(!v.programma.is_empty());
    // In ordine di distanza decrescente del vigente dalla posizione della coalizione.
    let g = p.governo();
    let seggi: Vec<f64> = (0..p.partiti().len())
        .map(|j| f64::from(p.parlamento().seggi_camera[j] + p.parlamento().seggi_senato[j]))
        .collect();
    let pos = g.coalizione.iter().map(|&j| seggi[j] * p.partiti()[j].posizione_dichiarata).sum::<f64>()
        / g.coalizione.iter().map(|&j| seggi[j]).sum::<f64>();
    let dati = dati_base();
    let d = dati.disposizioni();
    let distanze: Vec<f64> = (g.programma.iter())
        .filter_map(|&(k, _)| d[k].posizione(p.disposizioni().voci[k].in_vigore))
        .map(|x| (x - pos).abs())
        .collect();
    assert!(distanze.windows(2).all(|w| w[0] >= w[1]), "{distanze:?}");
    assert!(v.mosse.iter().any(|m| m.testo.starts_with("Programma di governo")));
}

#[test]
fn col_giocatore_al_governo_nessun_programma() {
    assert!(al_governo().vista().programma.is_empty());
}
```

Nel modulo di test di `programma.rs`: `l_elettorale_entra_solo_oltre_n_seggi`, con un'aula in cui il gradino migliore dà 14 seggi attesi e poi 15 alla coalizione: fuori con 14, ultima voce con 15. Ancora nel modulo di `comando.rs`: `applica_non_guarda_chi_governa`, con `Comando::Dimettiti` applicato all'opposizione che mette le dimissioni al governo IA.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test governo_ia`
Expected: FAIL, `programma` assente.

- [ ] **Step 3: Implementa `applica`, `programma` e la vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): programma pubblico del governo avversario"
```

---

### Task 5: Il governo IA in apertura e il suo bilancio

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs` (`apertura_ia`, `governo_ia`, `bilancio_ia`)
- Modify: `crates/sim-core/src/partita.rs:277-289` (via il segnaposto), `:418-424` (coda dell'apertura)
- Modify: `crates/sim-core/src/economia/legge_bilancio.rs` (`indicizza_in_parte`)
- Test: `crates/sim-core/tests/governo_ia.rs`, modulo di test di `legge_bilancio.rs`

**Ricerca:** `docs/ricerca/modelli/regole-fiscali-ue.md:8` (il tetto annuo alla crescita della spesa netta è l'unico vincolo operativo), `:14` (cosa conta nella spesa netta); `docs/ricerca/modelli/iter-legislativo-italiano.md:36-44` (questione di fiducia: effetto sugli emendamenti, frequenza).

**Dispatch:** `rust-engineer` · dopo il Task 4 · skill-gate: true.

**Interfaces:**
- Consumes: `applica`, `Governo.programma` (Task 4); `mossa` (Task 3); `StatoAula::previsione`, `Parlamento::forbice` (`parlamento/mod.rs:901`), `economia::riepilogo` (`legge_bilancio.rs:363`), `Intesa.ultimatum`.
- Produces:
  - `pub(crate) fn indicizza_in_parte(b: &Bilancio, quota: f64, attuale: f64) -> Result<Bilancio, String>` in `legge_bilancio.rs`: le voci dei dicasteri × `1 + quota · (attuale / livello − 1)`, livello che segue come in `indicizza`; `quota = 1` coincide con `indicizza(b, &Voce::Tutto, …)` sui dicasteri.
  - `fn apertura_ia(&mut self)` in `partita/ia.rs`, chiamata in coda a `chiudi_turno`, a partita non finita e in fase ordinaria; qui chiama `governo_ia` se il Presidente del Consiglio non è il giocatore. I Task 6-8 vi aggiungono i loro passi.
  - `fn governo_ia(&mut self)`, fuori dalla crisi, nell'ordine:
    1. bilancio: in ottobre-dicembre o nell'esercizio provvisorio, con la bozza e senza bilancio presentato, `bilancio_ia()` e `applica(PresentaBilancio)`; riga «Bilancio {anno}: budget indicizzati al {q}% dell'inflazione, entro il tetto UE» più «, con le richieste di {alleati}» se ce ne sono;
    2. programma: per ogni voce non ancora in vigore, in ordine, `applica(PresentaLegge)` finché il calendario si riempie; le voci rifiutate dalle regole si saltano in silenzio; riga «Porta in aula: {nome della misura}»;
    3. proposte dei ministri: `applica(Accogli)` su ogni dicastero con una proposta; riga «Accoglie la proposta di {dicastero}» solo se passa;
    4. emendamenti aperti sulle misure del governo col voto nel turno: accolto (`applica(Emendamento)`) se l'alleato è sotto `soglia_ultimatum` («l'intesa di {alleato} è sotto 40») o se, con la sua linea a no, `forbice(previsione).in_bilico` («senza i voti di {alleato} la legge rischia»); altrimenti respinto («Respinge l'emendamento di {alleato}»);
    5. fiducia: su ogni misura del governo col voto nel turno, senza fiducia, se la forbice è in bilico e nessun alleato è in ultimatum, `applica(PoniFiducia)`; riga «Pone la fiducia su {misura}: la forbice include la sconfitta».
  - `fn bilancio_ia(&self) -> Option<Bilancio>`: scelta 10 del piano, con `indicizza_in_parte` e la bisezione su `quota` in 0-1, 20 passi.
  - `pub fn riepilogo_presentato(&self) -> Option<RiepilogoBilancio>` in `partita.rs`: il riepilogo del bilancio presentato contro la bozza, in sola lettura e per chiunque (il bilancio in aula è pubblico); `None` senza bilancio presentato o col riepilogo non calcolabile. Lo legge il test, e il piano 11 all'opposizione.
  - Il segnaposto del 7a (`partita.rs:277-289`) sparisce: il bilancio IA lo presenta l'apertura.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/governo_ia.rs (in coda)
/// Dopo la nascita il governo IA porta in aula le prime voci del programma, fino a tre.
#[test]
fn il_programma_va_in_aula_in_ordine() {
    let p = all_opposizione();
    let in_aula: Vec<usize> = (p.parlamento().misure.iter())
        .filter(|m| m.promotore.is_none() && m.valori.len() == 1)
        .map(|m| m.valori[0].0)
        .collect();
    let attese: Vec<usize> = p.governo().programma.iter().map(|v| v.0).take(in_aula.len()).collect();
    assert!(!in_aula.is_empty() && in_aula.len() <= 3);
    assert_eq!(in_aula, attese);
}

/// Il programma ridotto alla prima voce, scritta al gradino già in vigore: la presentazione la
/// rifiuta («gradino già in vigore»), e il governo non porta nulla in aula né lascia righe.
#[test]
fn programma_esaurito_non_presenta_nulla() {
    let p = all_opposizione();
    let k = p.governo().programma[0].0;
    let vigente = p.disposizioni().voci[k].in_vigore;
    let mut p = edita(&p, |j| {
        j["governo"]["programma"] = serde_json::json!([[k, vigente]]);
        j["parlamento"]["misure"] = serde_json::json!([]);
    })
    .unwrap();
    p.chiudi_turno();
    assert!(p.vista().mosse.iter().all(|m| !m.testo.starts_with("Porta in aula")));
    assert!(p.parlamento().misure.iter().all(|m| m.valori.iter().all(|v| v.0 != k)));
}

#[test]
fn il_bilancio_ia_sta_nel_tetto_senza_ultimatum() {
    // Avanza al primo ottobre-dicembre sotto il governo IA.
    let mut p = all_opposizione();
    while p.vista().data.trimestre != 4 {
        p.chiudi_turno();
    }
    assert!(p.intesa().ultimatum.iter().all(Option::is_none));
    let r = p.riepilogo_presentato().expect("bilancio IA presentato in apertura");
    assert!(r.crescita_spesa_netta <= r.tetto + 1e-6, "{r:?}");
    assert!(p.vista().mosse.iter().any(|m| m.testo.starts_with("Bilancio ")));
}

#[test]
fn lo_stato_ia_sopravvive_al_salvataggio() {
    let mut p = all_opposizione();
    p.chiudi_turno();
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
    let fuori = edita(&p, |j| j["governo"]["programma"] = serde_json::json!([[999, 0]]));
    assert!(fuori.is_err());
}
```

Se il seme 7 porta un ultimatum prima di ottobre-dicembre, il test lo azzera con `edita` all'apertura del turno prima. Nel modulo di test di `legge_bilancio.rs`: `indicizzare_in_parte_sta_fra_zero_e_tutto`, con quota 0 = bilancio invariato, quota 1 = `indicizza` dei dicasteri, quota 0,5 a metà, scaglioni intatti. Un test con l'alleato sotto 40 e un emendamento aperto verifica `Accolto` e la riga; uno col calendario e la forbice in bilico, senza ultimatum, verifica `fiducia` sulla misura e la riga.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test governo_ia`
Expected: FAIL, il governo IA non presenta nulla.

- [ ] **Step 3: Implementa `apertura_ia`, `governo_ia`, `bilancio_ia` e `indicizza_in_parte`**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS. Il test `il_bilancio_dell_ia_non_ha_emendamenti` (`partita.rs:3400`) resta vero; `il_governo_ia_presenta_il_bilancio` (`tests/governo_ia.rs:376`) si adatta al bilancio presentato in apertura invece che in chiusura.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): il governo avversario governa: programma, proposte, emendamenti, fiducia, bilancio"
```

---

### Task 6: Crisi e voto anticipato del governo IA

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs` (`crisi_ia`, `voto_anticipato`)
- Modify: `crates/sim-core/src/legislatura/mod.rs:46-60` (`Legislatura.voto_anticipato`), `crates/sim-core/src/partita.rs:2371-2399` (`capo_dello_stato`), `:376-379` (chiusura), `:3121-3129` (guardia degli allargati), le due costruzioni di `Legislatura` (`partita.rs:154`, `:173`)
- Test: `crates/sim-core/tests/governo_ia.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:13` (minima connessa vincente: la contigua del Capo dello Stato e il partner contiguo della crisi); `docs/ricerca/modelli/iter-legislativo-italiano.md:42-43` (dimissioni come prassi, sfiducia).

**Dispatch:** `rust-engineer` · dopo il Task 5 · skill-gate: true.

**Interfaces:**
- Consumes: `valore_al_governo` (Task 2), `apertura_ia`, `mossa` (Task 3, 5), `Quadro::valore`, `maggioranza`, `maggioranza_contigua`.
- Produces:
  - `fn crisi_ia(&mut self)`, chiamata da `apertura_ia` nel turno di crisi sotto un governo IA (fuori dal turno del voto): scelta 12 del piano. Allarga mettendo il partito in `legislatura.allargati` («Allarga la maggioranza a {partito}»), che `fine_della_crisi` già tratta; altrimenti `legislatura.dimissioni = true` («Si dimette: nessun partito contiguo ci guadagna»).
  - `Legislatura { …, pub voto_anticipato: bool }`, falso alla nascita e azzerato dal Capo dello Stato e dal voto.
  - `fn voto_anticipato(&mut self)` in chiusura, subito prima del passo delle dimissioni (`partita.rs:376-379`): scelta 13 del piano; se si dimette, `dimissioni = true`, `voto_anticipato = true`, riga «Si dimette per andare al voto: i sondaggi gli danno più del restare». Manopola `margine_voto_anticipato` (0,5, 0-4, assunzione del piano 8a, ticket `18`).
  - `capo_dello_stato`: con `voto_anticipato` l'escluso è il partito del Presidente del Consiglio dimissionario (decisione 2 di Mario); col giocatore Presidente del Consiglio resta lui; altrimenti nessuno, come oggi.
  - `controlla_partiti`: gli allargati sono ammessi anche nel turno di crisi sotto un governo IA; `voto_anticipato` solo con `dimissioni`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/governo_ia.rs (in coda)
/// L'alleato del governo IA senza il quale la coalizione perde la maggioranza esce sotto 25: il
/// turno dopo è di crisi. Se nessun alleato è decisivo, il test scrive i seggi con `con_seggi`.
fn in_crisi(mut p: Partita) -> Partita {
    let alleato = decisivo(&p);
    p = edita(&p, |j| j["intesa"]["valori"][alleato] = 10.0.into()).unwrap();
    p.chiudi_turno();
    assert!(p.vista().crisi);
    p
}

#[test]
fn in_crisi_il_governo_ia_allarga_o_si_dimette() {
    let p = in_crisi(all_opposizione());
    let l = p.legislatura();
    assert!(!l.allargati.is_empty() || l.dimissioni);
    assert!(!l.allargati.contains(&p.giocatore()));
    assert!(p.vista().mosse.iter().any(|m| m.testo.starts_with("Allarga") || m.testo.starts_with("Si dimette")));
}

#[test]
fn il_governo_ia_in_crisi_non_presenta_il_bilancio() {
    // La crisi cade in ottobre-dicembre: ordinaria amministrazione, niente bilancio.
    let mut p = all_opposizione();
    while p.vista().data.trimestre != 3 {
        p.chiudi_turno();
    }
    let mut p = in_crisi(p);
    assert_eq!(p.vista().data.trimestre, 4);
    assert!(p.economia().bilancio_presentato.is_none());
    p.chiudi_turno();
    assert!(p.salva().is_ok());
}

#[test]
fn col_voto_anticipato_il_capo_dello_stato_esclude_chi_si_dimette() {
    let p = all_opposizione();
    let pdc = p.governo().presidente;
    let mut p = edita(&p, |j| {
        j["legislatura"]["dimissioni"] = true.into();
        j["legislatura"]["voto_anticipato"] = true.into();
    })
    .unwrap();
    p.chiudi_turno();
    match &p.legislatura().incarico {
        Some(Incarico::Ia { coalizione } | Incarico::Guida { coalizione }) => assert!(!coalizione.contains(&pdc)),
        Some(Incarico::Giocatore) => assert!(!p.legislatura().maggioranza.as_ref().unwrap().contains(&pdc)),
        None => assert_eq!(p.legislatura().fase, Fase::Sciolta),
    }
}
```

`decisivo(&p)` è un helper locale: il primo alleato senza il quale `governo.coalizione` perde la maggioranza in una Camera. Nel modulo di test di `partita/ia.rs`, che vede i campi della partita:

- `niente_voto_anticipato_fuori_finestra`: la guardia `fn finestra_voto_anticipato(&self) -> bool` è falsa a `inizio + 3`, a `scadenza − 1`, nel turno di crisi, con `dimissioni` già vere e nel turno del voto; vera a `inizio + 4` e a `scadenza − 2`.
- `si_va_al_voto_quando_conviene`: sotto un governo IA, a `inizio + 6`, valence del partito del Presidente del Consiglio a 3 e degli altri a −3; dopo una chiusura (che rileva il sondaggio nuovo prima del voto anticipato) `voto_anticipato` e `dimissioni` sono veri, e la riga c'è. Con le valence di partenza resta in carica.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test governo_ia`
Expected: FAIL, `voto_anticipato` assente.

- [ ] **Step 3: Implementa crisi e voto anticipato**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): crisi e voto anticipato del governo avversario"
```

---

## PR C · Opposizione IA (Task 7-9)

### Task 7: La sfiducia dell'opposizione IA

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs` (`opposizione_ia`, `sfiducia_ia`)
- Modify: `crates/sim-core/src/partita.rs:2534-2608` (`voto_sfiducia`, `firme_sfiducia` per un insieme di firmatari)
- Test: `crates/sim-core/tests/opposizione_ia.rs` (nuovo)

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:43` (firme di un decimo, votazione per appello nominale), `:141-142` (sfiducia al singolo ministro con la stessa disciplina).

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true.

**Interfaces:**
- Consumes: `apertura_ia`, `mossa` (Task 3, 5); `voto_sfiducia(dicastero) -> Option<(Vec<bool>, Conteggio, Conteggio, bool)>`; `parlamento.sfiducia: Option<(Option<usize>, usize)>`.
- Produces:
  - `fn opposizione_ia(&mut self)`, chiamata da `apertura_ia` dopo il governo IA, chiunque governi (decisione 3 di Mario); qui chiama `sfiducia_ia`, il Task 8 vi aggiunge la proposta.
  - `fn sfiducia_ia(&mut self)`: scelta 14 del piano; con la mozione già presentata non fa nulla. Righe: «Presenta la sfiducia al governo: {alleato} in ultimatum la voterebbe» e «Presenta la sfiducia a {ministro del dicastero}, {dicastero}: {alleato} in ultimatum la voterebbe».
  - `firme_sfiducia` prende i firmatari: per il giocatore i partiti fuori dal governo come oggi, per l'IA i soli partiti IA fuori dal governo.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/opposizione_ia.rs
mod comune;
use comune::*;
use sim_core::comando::Comando;

/// Il giocatore governa col centrodestra; seggi scritti con `con_seggi` perché l'opposizione da
/// sola non abbia la metà in nessuna Camera e, col centrodestra, la superi alla Camera. Con
/// `ultimatum` il centrodestra è in ultimatum: vota sì sulla sfiducia ai ministri della destra.
fn al_governo_coi_seggi(ultimatum: bool) -> sim_core::partita::Partita {
    let p = seggi_al_limite(al_governo());
    let cd = partito("centrodestra");
    edita(&p, |j| {
        if ultimatum {
            j["intesa"]["valori"][cd] = 30.0.into();
            j["intesa"]["ultimatum"][cd] = "spartizione".into();
        }
    })
    .unwrap()
}

#[test]
fn l_opposizione_ia_sfiducia_solo_quando_passerebbe() {
    let mut p = al_governo_coi_seggi(true);
    chiudi_col_bilancio(&mut p);
    let (dicastero, promotore) = p.parlamento().sfiducia.expect("almeno l'individuale passa");
    assert_ne!(promotore, p.giocatore());
    if let Some(i) = dicastero {
        let m = p.ministri().dicasteri[i].ministro.as_ref().unwrap();
        assert_ne!(m.partito, partito("centrodestra"));
    }
    assert!(p.vista().mosse.iter().any(|m| m.testo.starts_with("Presenta la sfiducia")));
    let mut q = al_governo_coi_seggi(false);
    chiudi_col_bilancio(&mut q);
    assert!(q.parlamento().sfiducia.is_none(), "nessun alleato in ultimatum: niente sfiducia di bandiera");
}

#[test]
fn una_sola_sfiducia_per_turno() {
    let p = all_opposizione();
    let mut p = edita(&p, |j| j["parlamento"]["sfiducia"] = serde_json::json!([null, 0])).unwrap();
    let e = p.comanda(Comando::Sfiducia { dicastero: None }).unwrap_err();
    assert!(e.to_string().contains("già presentata"), "{e}");
}
```

`seggi_al_limite` è un helper locale che riscrive i seggi di `al_governo()` con `con_seggi` alle condizioni del commento, a totali invariati. Un terzo test, `la_sfiducia_ia_presentata_passa_se_nulla_cambia`, chiude il turno dopo senza comandi e trova l'esito della mozione approvato. Un quarto, `sotto_un_governo_ia_l_opposizione_ia_sfiducia`, ripete il primo con `all_opposizione()` e un alleato del governo IA in ultimatum: la mozione è di un partito IA, e il giocatore vota sì.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test opposizione_ia`
Expected: FAIL, nessuna sfiducia IA.

- [ ] **Step 3: Implementa la sfiducia IA**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): l'opposizione IA presenta la sfiducia quando passerebbe"
```

---

### Task 8: Proposta-imbarazzo, linea del Presidente del Consiglio e malus dell'ultimo anno

**Files:**
- Modify: `crates/sim-core/src/parlamento/voto.rs:49-65` (`linea`), `crates/sim-core/src/parlamento/mod.rs:296-386` (`previsione`), `StatoAula` (`ultimo_anno`)
- Modify: `crates/sim-core/src/partita.rs:2657-2664` (`Partita::linea`), ogni costruzione di `StatoAula` (`partita.rs:393`, `:1008`)
- Modify: `crates/sim-core/src/partita/ia.rs` (`proposta_ia`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/opposizione_ia.rs`, modulo di test di `parlamento/voto.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:117-127` (calendario riservato alle opposizioni, proposte che quasi mai passano, il caso tipico del salario minimo); `docs/ricerca/modelli/voto-spaziale.md:14` (voto per distanza, base della linea).

**Dispatch:** `rust-engineer` · dopo il Task 7 · skill-gate: true.

**Interfaces:**
- Consumes: `opposizione_ia`, `mossa`; `posto_riservato()`, `Parlamento::bozza`, `Parlamento::presenta_proposta`, `voto::guadagni`.
- Produces:
  - `pub fn linea(dati: &DatiGioco, governo: &Governo, intesa: &[f64], guadagno: &[f64], malus: &[f64], pdc_libero: bool) -> Vec<bool>`: `malus` per partito (zero al governo); con `pdc_libero` il Presidente del Consiglio vota sì solo se `guadagno > 0`. `previsione` passa `pdc_libero = m.promotore.is_some()` e il malus di `StatoAula::malus()`: `malus_opposizione`, × `(1 + crescita_malus_ultimo_anno)` per i partiti IA nell'ultimo anno (scelta 19).
  - `StatoAula { …, pub ultimo_anno: bool }`.
  - `Partita::linea`: ammessa anche al giocatore che governa, solo sulle proposte dell'opposizione; altrimenti «la linea del governo si sceglie solo sulle proposte dell'opposizione».
  - `fn proposta_ia(&mut self)`, chiamata da `opposizione_ia`: col posto riservato del turno a un partito IA e libero, la scelta 16 del piano, poi `Parlamento::presenta_proposta`. Riga «Porta nel posto riservato {nome della misura}: {n} seggi della maggioranza la voterebbero».
  - Manopola `crescita_malus_ultimo_anno` (0,5, 0-2, assunzione del piano 8a, ticket `18`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/opposizione_ia.rs (in coda)
#[test]
fn la_proposta_imbarazzo_piace_alla_maggioranza() {
    let mut p = al_governo();
    // Avanza finché il posto riservato tocca a un partito IA.
    let mut proposta = None;
    for _ in 0..8 {
        chiudi_col_bilancio(&mut p);
        proposta = p.parlamento().misure.iter().find(|m| m.promotore.is_some_and(|j| j != p.giocatore())).cloned();
        if proposta.is_some() {
            break;
        }
    }
    let m = proposta.expect("un turno del posto riservato a un partito IA");
    assert!(p.vista().mosse.iter().any(|x| x.testo.starts_with("Porta nel posto riservato")));
    // Almeno un partito di governo la voterebbe.
    let voce = p.vista().calendario.into_iter().find(|v| v.id == m.id).unwrap();
    let indice = |id: &str| p.partiti().iter().position(|x| x.id == id).unwrap();
    assert!(voce.linee.iter().any(|l| l.si && p.governo().al_governo(indice(&l.partito))));
}

#[test]
fn il_giocatore_al_governo_sceglie_la_linea_solo_sulle_proposte_dell_opposizione() {
    let mut p = al_governo();
    presenta(&mut p, &[("eta_pensionabile", "70")]).unwrap();
    let id = p.parlamento().misure.last().unwrap().id;
    let e = p.comanda(Comando::Linea { misura: id, si: Some(false) }).unwrap_err();
    assert!(e.to_string().contains("solo sulle proposte dell'opposizione"), "{e}");
}
```

Nel modulo di test di `parlamento/voto.rs`: `il_pdc_libero_segue_il_guadagno` (con `pdc_libero` e guadagno negativo il Presidente del Consiglio vota no, senza vota sì) e `il_malus_cresce_nell_ultimo_anno` (un partito IA d'opposizione con guadagno 1,2 vota sì col malus 1 e no col malus 1,5). La legge del secondo test è quella di `a_settanta_anni` (`partita.rs:3331`). Se il seme 7 non dà mai il posto a un partito IA in otto turni, il primo test lo dichiara e scrive con `edita` il turno giusto.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test opposizione_ia && cargo test -p sim-core --lib voto`
Expected: FAIL.

- [ ] **Step 3: Implementa linea, malus e proposta IA**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS. I test del 7b sulle proposte del giocatore all'opposizione (`tests/opposizione.rs:227`, `:272`) cambiano solo se il partito del Presidente del Consiglio IA, ora libero, vota diversamente: il valore nuovo si scrive col motivo «linea del Presidente del Consiglio, piano 8a».

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): proposta-imbarazzo dell'opposizione IA e linea libera del governo"
```

---

### Task 9: Referendum dell'opposizione IA e confermativo del giocatore

**Files:**
- Modify: `crates/sim-core/src/elettorato/referendum.rs` (`stima`)
- Modify: `crates/sim-core/src/partita/ia.rs` (`referendum_ia`), `crates/sim-core/src/partita.rs:485-506` (`aula`, prima delle firme), `:604-640` (`vota`, creazione del confermativo), `:779-820` (`vota_confermativi`)
- Modify: `crates/sim-core/src/parlamento/regole.rs:29` (`Referendum.richiesto`), `crates/sim-core/src/comando.rs` (`Comando::Confermativo`), `crates/sim-core/src/vista.rs` (`VistaReferendum`)
- Test: `crates/sim-core/tests/opposizione_ia.rs`, `crates/sim-core/tests/referendum.rs`, modulo di test di `elettorato/referendum.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:77` (un quinto dei membri di una Camera chiede il confermativo), `:131-135` (deposito, voto, blocchi dell'abrogativo); `docs/ricerca/modelli/sondaggi-ed-errore.md:55` (si decide sul sondaggio, che sbaglia, non sullo stato vero).

**Dispatch:** `rust-engineer` · dopo il Task 8 · skill-gate: true.

**Interfaces:**
- Consumes: `Quesito`, `Affluenza`, `toccati` (`elettorato/referendum.rs`); `promuovi_referendum`; `mossa`; `parlamento::quinto_contro`.
- Produces:
  - `pub(crate) fn stima(dati: &DatiGioco, sondaggio: &Sondaggio, quesito: &Quesito) -> Affluenza` in `elettorato/referendum.rs`: la formula di `Elettorato::referendum` (scelta 17 del piano) con le posizioni degli incroci regionali al posto dei gruppi, pesate per gli elettori della regione; per regione l'astensione dei dati e i toccati sono la media dei suoi gruppi. Senza giudizio sul governo (`peso_governo` dell'abrogativo è 0) e senza guadagno (le elettorali sono escluse).
  - `fn referendum_ia(&mut self)`, chiamata da `aula` prima di `r.firme = true`, se nel turno nessun referendum è stato chiesto: scelta 17 del piano, via `promuovi_referendum`. Riga «Chiede il referendum su {disposizione}: il sì vincerebbe col {x}% e il quorum».
  - `Referendum { …, pub richiesto: bool }`: vero sugli abrogativi; sul confermativo, il quinto contro (regola dell'08).
  - `Comando::Confermativo { misura: u32, chiedi: bool }`, con la guardia `all_opposizione`; ammesso nel turno dopo la seconda deliberazione, finché il referendum aspetta il voto. Rifiuti: «nessun confermativo da chiedere su questa misura»; «per chiederlo serve un quinto di una Camera, col tuo partito e i contrari». Con `chiedi = false`, `richiesto` = quinto contro senza il giocatore.
  - `vota_confermativi`: un confermativo non richiesto non si vota, e la legge entra in vigore come approvata, senza esito di referendum.
  - `VistaReferendum { …, pub richiesto: bool, pub scelta: bool }` (`scelta` = il giocatore può ancora chiederlo o rinunciarvi).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/opposizione_ia.rs (in coda)
#[test]
fn l_opposizione_ia_chiede_solo_referendum_vincenti() {
    let mut p = al_governo();
    let mut chiesto = None;
    for _ in 0..12 {
        chiudi_col_bilancio(&mut p);
        if let Some(r) = &p.parlamento().referendum
            && r.promotore != Some(p.giocatore())
        {
            chiesto = Some(r.clone());
            break;
        }
    }
    if let Some(r) = chiesto {
        assert!(p.vista().mosse.iter().any(|m| m.testo.starts_with("Chiede il referendum")));
        assert!(!p.governo().al_governo(r.promotore.unwrap()));
    }
}

// crates/sim-core/tests/referendum.rs (in coda)
#[test]
fn il_giocatore_rinuncia_al_confermativo() {
    let mut p = costituzionale_senza_due_terzi_all_opposizione();
    let id = p.parlamento().confermativi[0].misura.unwrap();
    p.comanda(Comando::Confermativo { misura: id, chiedi: false }).unwrap();
    p.chiudi_turno();
    assert!(p.parlamento().confermativi.is_empty());
    assert!(p.vista().esiti_referendum.is_empty());
}
```

`costituzionale_senza_due_terzi_all_opposizione()` è un helper locale: da `all_opposizione()` scrive con `edita` una costituzionale del governo IA in seconda deliberazione col voto nel turno, e seggi per cui passa senza i due terzi e col solo partito del giocatore contro, oltre un quinto; dopo la chiusura il confermativo aspetta il voto. Con lo stesso helper, `senza_scelta_vale_la_regola_dell_08` verifica il voto al turno dopo, e `chiedere_senza_un_quinto_e_rifiutato` il rifiuto col messaggio e la partita identica (`salva()` prima e dopo uguali). Nel modulo di test di `elettorato/referendum.rs`: `la_stima_sul_sondaggio_vero_e_vicina`, con un sondaggio senza errore costruito dalle intenzioni vere: sì e no stimati entro 3 punti da quelli di `Elettorato::referendum`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test opposizione_ia --test referendum`
Expected: FAIL, `Confermativo` assente.

- [ ] **Step 3: Implementa stima, referendum IA e confermativo**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): referendum dell'opposizione IA e confermativo scelto dal giocatore"
```

---

## Rimandato

- **Piano 8b (partiti e campagna):** partito di protesta, scissione (anche del partito del giocatore), scioglimento dopo due elezioni senza seggi, tetto di 10 partiti, e il numero di partiti che cambia a metà partita (vettori per partito, `controlla_partiti`); coalizioni IA che si ricompongono col valore atteso, quindi anche la nascita del governo avversario «col valore atteso» (scelta 6); regola di movimento nei dati e posizione dichiarata mossa nel turno prima del voto, riposizionamento del giocatore; cassa del partito (2 per mille, donazioni, dote); campagna con mobilitare, persuadere e convincere, anche dei partiti IA; promesse, con l'intesa che cala per le promesse su cui la linea dell'alleato sarebbe no e la disponibilità che le legge.
- **Piano 9 (eventi):** il governo IA sceglie sempre l'opzione di default degli eventi (spec riga 353); la colpa per dicastero registrata dal giudizio, se serve agli scandali, per la sfiducia individuale (scelta 14).
- **Piano 11 (UI):** caricature dei leader nel borsino, schermate del programma, delle mosse IA e del confermativo.
- **Piano 12 (bilanciamento):** α delle vocazioni, `calo_cambio_leader`, `soglia_programma`, `seggi_programma_elettorale`, `margine_voto_anticipato`, `crescita_malus_ultimo_anno` contro i bersagli del costo del governare (ticket 29, riga 26).
