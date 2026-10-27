# V1 · Piano 7a: turno zero, voto e governo — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** La partita vera comincia in `sim-core`: il giocatore fonda il partito su uno dei sei mezzi punti, dichiara la coalizione nel turno zero e va al voto a Camere sciolte; dal voto escono parlamento, trattativa e governo (il suo, con spartizione di Gamson e tecnici, oppure uno IA che lo manda all'opposizione); la legislatura ha durata e numero dei parlamentari dalle disposizioni, scade, decade e torna al voto. Prima di tutto, la coda del piano 6: referendum ritarati, valence che torna, abrogativo elettorale sui seggi attesi, anteprima dei decreti.

**Architecture:** Un modulo nuovo, `legislatura/`: fase della legislatura (Camere sciolte, trattativa, ordinaria), scadenza, coalizioni dell'ultimo voto, dichiarazione e disponibilità, voto, spartizione, formazione del governo. `Partita` guadagna l'indice del partito del giocatore e un secondo costruttore, `Partita::fonda`, per la partita vera; `Partita::nuova` resta la partita di prova dal governo uscente, su cui girano tutti i test dei piani 1-6. Il voto delle elezioni è un passo nuovo di `chiudi_turno`, dopo il motore e prima delle soglie; i comandi nuovi entrano dall'`enum Comando` esistente, con una guardia unica in `Partita::comanda` per chi governa.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 8-12 (righe 33-37), 67 (riga 104), 77-82 (righe 120-125), 87 (riga 130), 99 (riga 145); «Modulo scenario» (righe 284-285, taratura e vincibilità); «Elezioni e riparto» (righe 320-323, seggi premio, proiezione, voto); «Governo, ministri e coalizione» (righe 331-332, coalizione pre-elettorale e spartizione); «Parlamento» (riga 344, referendum abrogativo); «Turno e partita» (righe 385, 388). Scope dalla tabella dei piani: [`2026-10-21-v1-piano-1-fondamenta.md:48`](2026-10-21-v1-piano-1-fondamenta.md). Ticket: `.scratch/v1-giocabile/issues/20-partenza-della-partita.md` (fondazione, calendario, parlamento di partenza, vincibilità), `05-coalizione-e-crisi.md` (coalizione pre-elettorale, dopo il voto, spartizione), `29-costo-del-governare.md:50` (luna di miele solo al primo governo). Debiti del piano 6: `2026-10-26-v1-piano-6-parlamento-e-misure.md:25-32`, sezioni «Rimandato» delle PR #25 e #26. Decisioni di Mario del 27/10/2026: memoria `decisioni-piano-6-aperte` e le tre risposte riportate sotto.

## Perché due piani

Lo scope della riga 7 della tabella non entra in un piano solo: il piano 6 aveva 14 task, questo ne avrebbe più di 20. La rotta però è chiara (spec, ticket 05 e 20), quindi niente wayfinder: si divide in due piani, ciascuno scritto quando il precedente è fuso, come vuole la tabella dei piani.

- **7a (questo):** coda del piano 6, fondazione, coalizione pre-elettorale, voto, trattativa e formazione del governo, legislatura. Produce una partita giocabile dal turno zero al voto successivo, con il giocatore al governo o all'opposizione.
- **7b:** intesa completa (le altre cause della spec riga 333, ultimatum come stato, uscita sotto 25, rimpasto), crisi, sfiducia, dimissioni (volontarie, dopo una fiducia respinta, dopo il bilancio respinto anche in esercizio provvisorio), ribaltone e scioglimento anticipato, comandi dell'opposizione (linea per misura, sfiducia, proposte nel posto riservato, `Partita::promuovi_referendum`), emendamenti degli alleati al bilancio (storia 44), ritorno al governo con la guida chiesta come condizione, eredità e abbandono.

Il taglio è diverso da quello proposto («7a turno zero, voto e legislatura; 7b coalizione, crisi e opposizione») per una ragione sola: con la 180 il voto non si tiene senza una coalizione dichiarata (ticket 05), e senza formazione del governo il voto non produce una partita che continua. La coalizione pre-elettorale e la spartizione stanno quindi qui.

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · coda del piano 6 | `feat/v1-piano-7-coda` | `main` | 1-4 | quorum ritarati, valence che torna, abrogativo elettorale sui seggi attesi e quesito spostato da una legge, anteprima dei decreti |
| B · fondazione e voto | `feat/v1-piano-7-voto` | `feat/v1-piano-7-coda` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 5-8 | giocatore e fondazione, luna di miele e vincibilità, coalizione pre-elettorale, voto |
| C · governo e legislatura | `feat/v1-piano-7-governo` | `feat/v1-piano-7-voto` (impilata, stessa regola) | 9-11 | spartizione e tecnici, allargamento e governo IA, scadenza e fine della legislatura |

## Decisioni di Mario

Del 27/10/2026, prima del piano (memoria `decisioni-piano-6-aperte`):

1. **Ritaratura del turno zero.** Diventa: niente ritaratura delle valence (risposta del 27/10, sotto). Il voto del turno zero non dà una maggioranza al governo uscente, e il motivo sta nei bersagli: i sondaggi di ottobre 2026 sommati per posto danno posti 2+3 = 43,135% dei votanti contro posti 5+6 = 41,42% (`strumenti/taratura/bersagli.toml:3-12`; il posto 2 somma PD e M5S, Futuro Nazionale è al posto 7, fuori dal governo uscente). La taratura del piano 4 li riproduce: per l'affluenza di 0,636 fanno il 27,4% e il 26,3% degli aventi diritto misurati. Ritarare le valence romperebbe la riproduzione dei sondaggi.
2. **Quorum dei referendum.** Il confermativo verso il 50% circa di affluenza; l'abrogativo raggiunge il quorum solo sui temi molto sentiti (Task 1).
3. **Abrogazione approvata.** Ogni partito della coalizione di governo perde il valore pieno di `valence_referendum_perso` (Task 1).
4. **Abrogativo elettorale.** Gli elettori votano sui seggi attesi, come in aula (Task 3).
5. **Anteprima dei decreti.** Decreto di fondi e conversione, come dice la storia 55 (Task 4).
6. **Tarature.** Nel confermativo la distanza pesa più del giudizio sul governo (Task 1); `valence_legge_respinta` smette di accumularsi senza ritorno (Task 2).

Del 27/10/2026, sulle domande di questo piano:

7. **Parlamento al turno zero.** Niente ritaratura; al turno zero della partita vera le Camere sono sciolte e la tabella `[parlamento_uscente]` non entra. Il task di taratura diventa luna di miele e vincibilità (Task 6), che nella sequenza viene dopo la fondazione (Task 5), perché tara il partito del giocatore.
8. **«Temi molto sentiti» = elettori toccati.** L'astensione dell'abrogativo cala per i gruppi che la disposizione tocca (`gruppi` e `perimetro` del catalogo): il quorum dipende da quanti sono colpiti, non da quanto la cosa divide (Task 1). Conseguenza da sapere: le disposizioni con `gruppi` vuoti nel catalogo (le nove elettorali, nucleare, cittadinanza, prescrizione e le altre di giustizia e interno) non raggiungono mai il quorum; per cambiarlo basta dare loro dei gruppi in `dati/base/disposizioni.toml`.
9. **La guida pesa.** Un partito che guiderebbe la propria coalizione mette in conto, nella disponibilità, la presidenza che perde entrando in quella del giocatore (Task 7). Il giocatore resta sempre il candidato Presidente del Consiglio della sua coalizione (ticket 05).

## Scelte del piano, da confermare in review

1. **`Partita::nuova` resta, come partita di prova dal governo uscente.** Il giocatore vi è il partito del Presidente del Consiglio uscente, l'aula è aperta e i seggi vengono da `[parlamento_uscente]` (i seggi reali del 2022). Serve a tutti i test dei piani 1-6, che assumono un giocatore al governo dal turno zero, e alle strategie del bilanciamento (piano 12). La partita vera parte da `Partita::fonda`, a Camere sciolte e senza tabella. **La tabella quindi non sparisce dai dati**: esce dalla partita vera, resta per quella di prova. L'alternativa, riscrivere i test dei piani 5-6 sopra un voto vinto, cambierebbe una quarantina di file senza cambiare il gioco.
2. **La luna di miele e ogni perdita di valence tornano verso la valence di base con un solo ritorno per turno** (`ritorno_valence`). Così `valence_legge_respinta` smette di accumularsi (decisione 6), e la luna di miele si spegne nella prima metà della legislatura (`docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:32`). Durata e decadimento della luna di miele vanno al bilanciamento (ticket 20).
3. **Disponibilità come valore atteso a cinque termini**, letto sulla proiezione del sondaggio pubblico (Task 7): seggi attesi, probabilità di governare (logistica sul margine di maggioranza proiettato), distanza media dai membri, guida, intesa bassa con un alleato in carica. La causa mostrata è il termine che pesa di più nella differenza fra le due alternative.
4. **Le coalizioni IA al voto sono quelle dello scenario meno i partiti entrati nella coalizione del giocatore**; chi resta senza coalizione corre da solo. Le coalizioni IA che si ricompongono fra loro sono del piano 8 (vita dei partiti).
5. **Cessione di posti**: `n` seggi della Camera e `n / 2` (per difetto) del Senato passano dal giocatore all'alleato dopo il riparto, mai oltre i seggi del giocatore. La disponibilità li vede come seggi attesi in più.
6. **Spartizione di default**, se il giocatore non propone, e spartizione dei governi IA: dicasteri in ordine di `peso_spartizione` decrescente, ciascuno al partito con la richiesta scoperta più grande; il partito che guida ha come richiesta il resto. La stessa funzione serve al rimpasto del 7b.
7. **Tecnici**: posizione `4 + scarto_ministri · z` (nessun partito a cui ancorarla), capacità da 3 a 5. Contano nella quota del partito del giocatore; la loro intesa non esiste.
8. **Il governo nasce alla chiusura del turno di trattativa**, con ministri tutti nuovi in rodaggio; il governo uscente regge l'interim nel turno di trattativa, in cui l'aula non vota leggi nuove e i comandi di governo sono rifiutati a tutti.
9. **Senza maggioranza dopo il voto**: incarico al giocatore se la sua coalizione è la prima per seggi alla Camera, con un turno di trattativa per allargare; poi il Capo dello Stato cerca la maggioranza contigua più piccola senza il partito del giocatore; senza nemmeno quella, scioglie e si vota a fine del turno dopo.
10. **Governo IA, segnaposto fino al piano 8**: le proposte dei suoi ministri vanno ignorate (quindi respinte dopo un turno, regola esistente); nel turno di ottobre-dicembre presenta da sé la bozza della legge di bilancio, se c'è.
11. **Durata della legislatura e numero dei parlamentari si leggono al voto**: una norma costituzionale approvata a metà legislatura vale dalla successiva.
12. **A ogni voto**, per scadenza o scioglimento: le leggi in esame decadono, i gradini abrogati si sbloccano, le leggi respinte tornano presentabili, un abrogativo indetto e non ancora votato è sospeso. I referendum confermativi in attesa si votano comunque.

## Global Constraints

Valgono tutti i vincoli dei piani 1, 3, 4, 5 e 6 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`, `2026-10-22-v1-piano-3-motore-economico.md:26-31`, `2026-10-22-v1-piano-4-elettorato-e-sondaggi.md:33-44`, `2026-10-25-v1-piano-5-governo.md:42-53`, `2026-10-26-v1-piano-6-parlamento-e-misure.md:61-73`): italiano coi termini di `CONTEXT.md` (turno zero, fondazione, luna di miele, coalizione, disponibilità, richiesta, spartizione, intesa, opposizione, legislatura, tecnico; mai «manopola» per una disposizione, mai «tick», «round», «elezione» per il voto del turno); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Nessun flusso casuale nuovo. Il voto delle elezioni è deterministico (quote vere per gruppo × elettori, arrotondate); il bias comune si riestrae al voto da `Flusso::Sondaggi`, un normale per alternativa nell'ordine dei partiti e poi l'astensione; i ministri del governo nuovo da `Flusso::Governo`, dicastero per dicastero nell'ordine dei dati. La disponibilità e l'anteprima della fondazione non estraggono nulla.
- Il partito del giocatore, nella partita vera, è l'ultimo dei partiti (indice = numero dei partiti dello scenario), con id `giocatore`; uno scenario con un partito di id `giocatore` è rifiutato.
- Un comando rifiutato lascia la partita identica: `salva()` prima e dopo uguali. Ogni rifiuto è `Errore::Comando` col nome del partito, del dicastero, del posto o della fase in causa; `Partita::fonda` rifiuta con `Errore::Fondazione`.
- Ogni valore provvisorio è una manopola con `valore`, `min`, `max`, `ticket`, aggiunta a `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:65`) dal task che la introduce; quelle senza fonte portano `# assunzione del piano 7, nessuna fonte in docs/ricerca`.
- La `Vista` non porta stato vero nuovo: disponibilità e richieste come esito della regola e causa, mai i valori attesi in cifre; il risultato del voto in cifre esatte dopo il voto.
- `VERSIONE_SCHEMA` passa a `7` nel Task 1 e resta `7` per tutto il piano.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task Rust: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Turni nei test: nella partita vera turno 0 = ottobre-dicembre 2026 (Camere sciolte, voto nella sua chiusura), 1 = gennaio-marzo 2027 (trattativa, governo alla sua chiusura), 2 = aprile-giugno 2027 (primo turno ordinario), scadenza 20 = ottobre-dicembre 2031. Nella partita di prova la legislatura va dal turno 0 alla scadenza 19.
- Seme dei test della partita vera: `7`, posto `4.5` salvo dove il test dice altro.

## Review Focus

1. **Fondazione con nome vuoto o di soli spazi, posto che non è un mezzo punto fra due partiti vicini, posto non finito**: `Partita::fonda` rifiuta col nome o col posto, nessun panic. Test `fondazione_non_valida_rifiutata_col_motivo` nel Task 5.
2. **Comando di governo dato da chi non governa** (turno zero, trattativa, opposizione) **o a partita finita**: rifiuto con la fase o il motivo, partita identica. Test `comandi_di_governo_rifiutati_a_chi_non_governa` nel Task 5, esteso nel Task 10 con l'opposizione.
3. **Partito del giocatore senza seggi dopo il voto** (sotto soglia, da solo): la partita finisce, `chiudi_turno` non fa nulla, i comandi sono rifiutati, nessuna divisione per zero nelle richieste. Test `senza_seggi_la_partita_finisce` nel Task 8.
4. **Salvataggio a metà del turno zero con la coalizione dichiarata, e a metà della trattativa con la spartizione proposta** (storia 146): la partita ricaricata chiude il turno identica a quella mai salvata. Test `salvare_con_la_coalizione_dichiarata` nel Task 7 e `salvare_a_meta_trattativa` nel Task 9.
5. **Nessuna maggioranza, nemmeno contigua, per due voti di fila**: scioglimento, voto al turno dopo e di nuovo, senza ciclo infinito né panic; la partita resta giocabile. Test `due_scioglimenti_di_fila` nel Task 10.

## Dispatch

PR A: 1 → 2 → 3 → 4 in sequenza. Tutti toccano `partita.rs`; 1 e 3 toccano anche `elettorato/referendum.rs`.

PR B: dopo la review della A, sul branch impilato, 5 → 6 → 7 → 8 in sequenza: il 6 tara il partito che il 5 crea, il 7 e l'8 consumano `Legislatura` e le coalizioni.

PR C: dopo la review della B, 9 → 10 → 11 in sequenza: tutti toccano `legislatura/` e `chiudi_turno`.

Ogni brief nomina `skill-gate`, `caveman` e, per chi scrive codice (`rust-engineer`), `ponytail`; copia la riga **Ricerca** del task come riferimento. Il round pre-commit di fine PR aggiunge `rust-reviewer` (diff `.rs` in tutte e tre) e `spec-reviewer` (il piano esiste), in parallelo a `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer`; il `craft-reviewer` legge i testi per il giocatore elencati nella PR. Ogni PR porta l'elenco dei testi nuovi per la revisione di Mario, come la #26.

---

## PR A · Coda del piano 6 (Task 1-4)

### Task 1: Quorum ritarati e coalizione che paga l'abrogazione

**Files:**
- Modify: `crates/sim-core/src/elettorato/referendum.rs:16-75` (quesito con mobilitazione, astensione per tipo)
- Modify: `crates/sim-core/src/partita.rs:30` (`VERSIONE_SCHEMA = 7`), `:559-593` (confermativo), `:603-673` (abrogativo)
- Modify: `crates/sim-core/src/dati.rs:65` (manopole), `dati/base/manopole.toml`
- Modify: `crates/sim-core/tests/referendum.rs`, `crates/sim-core/tests/norme.rs` (usano `astensione_referendum`)
- Test: `crates/sim-core/tests/referendum.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:77` (confermativo senza quorum, abrogativo con la maggioranza degli aventi diritto); `:134` (quorum); `docs/ricerca/modelli/sondaggi-ed-errore.md:50` (l'astensione non ha un modello separato: la mobilitazione è una nostra assunzione).

**Dispatch:** `rust-engineer` · primo della PR A, nessuna dipendenza · skill-gate: true. Le tre manopole si tarano con un giro di prova sul seme 1 dei test; i valori trovati entrano in `manopole.toml` con l'intervallo.

**Interfaces:**
- Consumes: `Elettorato::referendum` (`crates/sim-core/src/elettorato/referendum.rs:31`), `DatoDisposizione::gruppi` e `perimetro` (`crates/sim-core/src/disposizioni/mod.rs:108-112`), `Governo::al_governo`.
- Produces:
  - `Quesito` guadagna `astensione: f64` (la base del tipo) e `toccati: Vec<f64>`: per gruppo, quota toccata in 0-1 (vuoto = nessuno). `u_ast` del gruppo `i` = `astensione(i) + quesito.astensione − mobilitazione_toccati · toccati[i] + invito · quota di governo`.
  - `pub(crate) fn toccati(dati: &DatiGioco, d: usize) -> Vec<f64>` in `elettorato/referendum.rs`: 1 se la condizione del gruppo è fra i `gruppi` della disposizione, per il peso della regione nel `perimetro` (perimetro vuoto = peso 1); 0 altrimenti.
  - Manopole: `astensione_confermativo` e `astensione_abrogativo` al posto di `astensione_referendum`; `mobilitazione_toccati`; `peso_governo_confermativo` al posto del letterale `1.0` di `partita.rs:572`.
  - `vota_abrogativo`: con l'abrogazione approvata, ogni partito con `governo.al_governo(j)` perde `valence_referendum_perso` intero.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/referendum.rs (in coda; `al_voto` promuove su licenziamento)
fn al_voto_su(f: FileDati, disposizione: &str) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", disposizione).unwrap();
    for _ in 0..6 {
        p.chiudi_turno();
    }
    p
}

fn affluenza(p: &Partita) -> f64 {
    let e = &p.vista().esiti_referendum[0];
    e.votanti as f64 / e.aventi_diritto as f64
}

#[test]
fn l_abrogativo_passa_il_quorum_solo_sui_temi_che_toccano_molti() {
    // Regime e età delle pensioni toccano quattro condizioni su sei.
    assert!(al_voto_su(senza_shock(), "eta_pensionabile").vista().esiti_referendum[0].quorum);
    for d in ["licenziamento", "tetto_contante", "nucleare"] {
        let p = al_voto_su(senza_shock(), d);
        assert!(!p.vista().esiti_referendum[0].quorum, "{d}: {}", affluenza(&p));
    }
}

#[test]
fn senza_gruppi_toccati_l_affluenza_dell_abrogativo_resta_bassa() {
    let p = al_voto_su(senza_shock(), "nucleare");
    assert!(affluenza(&p) < 0.35, "{}", affluenza(&p));
}

#[test]
fn abrogazione_approvata_costa_a_tutta_la_coalizione() {
    let prima = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let p = al_voto(abrogazione_che_vince("-5.0"));
    let persa = |id: &str| {
        let v = |q: &Partita| q.partiti().iter().find(|x| x.id == id).unwrap().valence;
        v(&prima) - v(&p)
    };
    let dati = DatiGioco::da_testi(&senza_shock()).unwrap();
    let pieno = dati.manopola("valence_referendum_perso");
    for id in ["destra", "centrodestra"] {
        assert!((persa(id) - pieno).abs() < 1e-9, "{id}: {}", persa(id));
    }
    assert!(persa("sinistra").abs() < 1e-9);
}
```

```rust
// crates/sim-core/tests/norme.rs (in coda): il confermativo, con `costituzionale_doppia` (riga 286)
#[test]
fn il_confermativo_porta_alle_urne_circa_meta_degli_elettori() {
    let p = costituzionale_doppia("5.0", "0.3");
    let e = &p.vista().esiti_referendum[0];
    let a = e.votanti as f64 / e.aventi_diritto as f64;
    assert!((0.45..=0.55).contains(&a), "{a}");
}

#[test]
fn nel_confermativo_la_distanza_pesa_piu_del_giudizio() {
    // Gradini nuovi a 6,0, lontani dalla maggioranza degli elettori, col peso del giudizio dei dati:
    // vince il no anche con un governo giudicato bene.
    let peso = comune::dati_base().manopola("peso_giudizio").to_string();
    assert!(!costituzionale_doppia("6.0", &peso).vista().esiti_referendum[0].approvato);
}
```

Nel test della distanza, se il giudizio sul governo al turno del confermativo è negativo col seme del file, il test lo porta a `+1,0` in ogni gruppo con `edita` prima del voto, così il caso è quello che conta: giudizio buono, norma lontana.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test referendum --test norme`
Expected: FAIL; i test esistenti con `astensione_referendum` falliscono al caricamento (manopola sconosciuta) finché non passano a `astensione_abrogativo`.

- [ ] **Step 3: Implementa**

`Quesito` e `toccati` come nelle Interfaces; i due voti passano la base del loro tipo, il confermativo `toccati` vuoto. Tarare sul seme 1: `astensione_confermativo` finché l'affluenza del confermativo sta in 0,45-0,55; `astensione_abrogativo` e `mobilitazione_toccati` finché `eta_pensionabile` passa il quorum e le altre tre no, con almeno 3 punti di margine su entrambi i lati del 50%; `peso_governo_confermativo` sotto 1 finché il secondo test del confermativo passa. Nel commento delle tre manopole, l'affluenza misurata. `VERSIONE_SCHEMA = 7`.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml
git commit -m "feat(core): quorum dei referendum ritarati, abrogazione a carico della coalizione"
```

### Task 2: La valence torna verso la base

**Files:**
- Modify: `crates/sim-core/src/partiti.rs:25-32` (`Partito.valence_base`), `:60-83` (`iniziali`)
- Modify: `crates/sim-core/src/partita.rs:151-245` (ritorno nelle soglie), `:1424-1447` (controllo)
- Modify: `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/partita.rs`

**Ricerca:** `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:32` (calo nella prima metà, recupero parziale).

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `DatoPartito::valence` (`crates/sim-core/src/partiti.rs:15`).
- Produces:
  - `Partito::valence_base: f64`, al turno zero uguale a `valence`.
  - Nelle soglie di `chiudi_turno`, dopo `intesa.soglie`: per ogni partito `valence += ritorno_valence · (valence_base − valence)`.
  - Manopola `ritorno_valence` (valore 0,1, min 0, max 1; assunzione del piano 7, nessuna fonte in docs/ricerca).
  - `controlla_partiti` rifiuta `valence_base` non finita.

- [ ] **Step 1: Scrivi il test che fallisce**

```rust
// crates/sim-core/tests/partita.rs (in coda)
#[test]
fn la_valence_persa_torna_verso_la_base() {
    let dati = dati_base();
    let ritorno = dati.manopola("ritorno_valence");
    let mut p = Partita::nuova(dati, 3);
    let base = p.partiti()[p.governo().presidente].valence_base;
    let colpita = edita(&p, |j| {
        let pdc = j["governo"]["presidente"].as_u64().unwrap() as usize;
        j["partiti"][pdc]["valence"] = serde_json::json!(base - 1.0);
    })
    .unwrap();
    p = colpita;
    p.chiudi_turno();
    let v = p.partiti()[p.governo().presidente].valence;
    assert!((v - (base - 1.0 + ritorno)).abs() < 1e-9, "{v}");
    for _ in 0..40 {
        p.chiudi_turno();
    }
    let v = p.partiti()[p.governo().presidente].valence;
    assert!((v - base).abs() < 0.05, "{v}");
}
```

- [ ] **Step 2: Esegui e verifica che fallisca**

Run: `cargo test -p sim-core --test partita la_valence_persa_torna_verso_la_base`
Expected: FAIL, campo `valence_base` assente.

- [ ] **Step 3: Implementa `valence_base` e il ritorno**

I test che confrontano una valence dopo una perdita (`tests/referendum.rs`, `tests/bilancio_in_aula.rs`, `tests/decreti.rs`) la leggono dopo le soglie dello stesso turno: la perdita attesa diventa `perdita · (1 − ritorno_valence)`. Aggiornarli così, non con tolleranze più larghe.

- [ ] **Step 4: Esegui e verifica che passi**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml
git commit -m "feat(core): valence che torna verso la base"
```

### Task 3: Abrogativo elettorale sui seggi attesi e quesito spostato da una legge

**Files:**
- Modify: `crates/sim-core/src/elettorato/referendum.rs` (guadagno per partito nel sì)
- Modify: `crates/sim-core/src/partita.rs:603-673` (`vota_abrogativo`)
- Modify: `crates/sim-core/src/parlamento/regole.rs:149-172` (seggi attesi riusati)
- Test: `crates/sim-core/tests/referendum.rs`

**Ricerca:** `docs/ricerca/sintesi/disposizioni-al-turno-zero.md:64-68` (cosa produce l'abrogazione della legge che ha fissato il valore in vigore), `:72` (la legge da abrogare cambia con la catena delle leggi recenti); Corte cost. 68/1978 [NON TROVATO in docs/ricerca: la regola sotto è un'assunzione del piano].

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true.

**Interfaces:**
- Consumes: `StatoAula::seggi_attesi` (`crates/sim-core/src/parlamento/regole.rs:152`), manopola `seggi_per_punto_linea`, `Quesito` del Task 1.
- Produces:
  - `Quesito` guadagna `guadagno: Vec<f64>`, uno per partito (vuoto = nessuno). Con `guadagno` non vuoto, `u_si` del gruppo al punto `x` è `Σ_j v_j · guadagno_j`, con `v_j` le intenzioni del gruppo, e `u_no = 0`; le posizioni non contano. Sui quesiti elettorali `guadagno_j = (Δseggi Camera + Δseggi Senato)_j / seggi_per_punto_linea`, con `Δ` = seggi attesi con l'abrogazione meno seggi con la legge in vigore.
  - In `vota_abrogativo`, prima del voto: se il gradino in vigore della disposizione è già quello dopo l'abrogazione, il referendum decade senza voto con `decaduto = "Decaduto senza voto: una legge ha già abrogato la disposizione"`, nessuna valence, nessun blocco. Se una legge ha cambiato il gradino in vigore dopo la promozione con un valore diverso, il quesito si trasferisce sul gradino nuovo: il titolo dell'esito è il gradino in vigore al voto (già oggi), e il gradino bloccato è quello.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/referendum.rs (in coda)
#[test]
fn sull_abrogativo_elettorale_gli_elettori_votano_i_seggi() {
    // Il ripescaggio è abrogabile; i partiti che ne guadagnano sono votati dalla maggioranza.
    let p = al_voto_su(con(senza_shock(), "astensione_abrogativo", "-5.0"), "ripescaggio");
    let e = &p.vista().esiti_referendum[0];
    assert!(e.si != e.no, "{e:?}"); // prima del piano 7 finivano pari
}

#[test]
fn una_legge_che_abroga_prima_del_voto_fa_decadere_il_quesito() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    presenta(&mut p, &[("licenziamento", "reintegro")]).unwrap();
    for _ in 0..6 {
        p.chiudi_turno();
    }
    let e = &p.vista().esiti_referendum[0];
    assert_eq!(e.decaduto.as_deref(), Some("Decaduto senza voto: una legge ha già abrogato la disposizione"));
    assert!(!e.quorum && !e.approvato);
}
```

`ripescaggio` non sta in `disposizioni.toml`: la genera il codice, referendabile e con `gruppi` vuoti (`disposizioni/mod.rs:226`, `:277-278`), per questo il test abbassa `astensione_abrogativo`. Se la legge su `licenziamento` non passa in aula col seme 1, il test usa `con_arrivo` per portare il gradino `reintegro` al turno 3.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test referendum`
Expected: FAIL sui due test nuovi.

- [ ] **Step 3: Implementa il guadagno nel quesito e la regola del quesito spostato**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(core): abrogativo elettorale sui seggi attesi, quesito spostato da una legge"
```

### Task 4: Anteprima dei decreti

**Files:**
- Modify: `crates/sim-core/src/parlamento/anteprima.rs:49` (effetti dell'atto per i fondi accolti)
- Modify: `crates/sim-core/src/partita.rs:845-860` (`anteprima`), nuove `anteprima_decreto` e `anteprima_accogli`
- Modify: `app/src-tauri/src/lib.rs` (comando Tauri dell'anteprima, rimandato dalla #26)
- Test: `crates/sim-core/tests/decreti.rs`

**Ricerca:** `docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:24` (tre livelli: stato vero, statistica, rapporto del consigliere); spec riga 371 (effetti a fasce con l'errore che cala con la capacità del ministro proponente).

**Dispatch:** `rust-engineer` · dopo il Task 3 · skill-gate: true.

**Interfaces:**
- Consumes: `parlamento::anteprima` (`crates/sim-core/src/parlamento/anteprima.rs:49`), `Decreto` e `FondiDecreto` (`crates/sim-core/src/parlamento/decreti.rs:18-45`), la proposta di fondi del ministro (`StatoDicastero::proposta`).
- Produces:
  - `pub fn Partita::anteprima_decreto(&self, decreto: &Decreto) -> Result<Anteprima, Errore>`: `Disposizioni` come l'anteprima di una legge, con `forma = Conversione`; `Fondi` con `forma = Conversione`, `costo_annuo = 0` (saldo zero), nessun effetto, linee e forbice della conversione (maggioranza sì, opposizione no, franchi tiratori dalla sola intesa). Rifiuta come `EmanaDecreto` (materia esclusa, innesco mancante, importo oltre il margine), senza toccare nulla.
  - `pub fn Partita::anteprima_accogli(&self, dicastero: &str) -> Result<Anteprima, Errore>`: per una proposta di fondi aperta del dicastero, il decreto che `Accogli` produrrebbe: `costo_annuo` = l'importo, gli effetti nominali dell'atto con le fasce stimate dal ministro del dicastero, linee e forbice della conversione. Rifiuta col nome del dicastero se non c'è una proposta di fondi.
  - Comando Tauri `anteprima` (legge) e `anteprima_decreto`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/decreti.rs (in coda)
#[test]
fn anteprima_dello_spostamento_di_fondi() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    let margine = v.ministeri.iter().find(|m| m.dicastero == "giustizia").unwrap().margine_residuo;
    let d = Decreto::Fondi { da: "giustizia".into(), a: "sanita".into(), importo: margine / 2 };
    let a = p.anteprima_decreto(&d).unwrap();
    assert_eq!(a.forma, FormaMisura::Conversione);
    assert_eq!(a.costo_annuo, 0);
    assert!(a.effetti.is_empty());
    assert!(a.linee.iter().all(|l| l.si == p.governo().al_governo(indice_partito(&p, &l.partito))));
}

#[test]
fn anteprima_dell_accogli_mostra_l_atto_e_il_costo() {
    let p = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1); // la Sanità chiede fondi
    let a = p.anteprima_accogli("sanita").unwrap();
    assert!(a.costo_annuo > 0);
    assert!(!a.effetti.is_empty());
    let prima = p.salva().unwrap();
    assert!(p.anteprima_accogli("giustizia").unwrap_err().to_string().contains("Giustizia"));
    assert_eq!(p.salva().unwrap(), prima);
}
```

`indice_partito` è un helper locale: posizione dell'id in `p.partiti()`. `sanita_cara()` è l'helper locale di `tests/decreti.rs:8`; la proposta di fondi della Sanità c'è già al turno zero (`tests/decreti.rs:20`).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test decreti`
Expected: FAIL, metodi assenti.

- [ ] **Step 3: Implementa le due anteprime e i comandi Tauri**

L'anteprima resta sugli effetti nominali scritti nei dati (scelta 8 del piano 6) e non avanza nessun flusso: la chiave dell'estrattore estemporaneo per i fondi accolti è turno, dicastero e atto, con lo stesso `OFFSET_CHIAVE` di `anteprima.rs:15`.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src-tauri app/src/lib/tipi
git commit -m "feat(core): anteprima dei decreti di fondi e della conversione"
```

---

## PR B · Fondazione e voto (Task 5-8)

### Task 5: Il giocatore e la fondazione

**Files:**
- Create: `crates/sim-core/src/legislatura/mod.rs` (`Fase`, `Legislatura`)
- Create: `crates/sim-core/src/legislatura/fondazione.rs` (`Fondazione`, posti, anteprima)
- Modify: `crates/sim-core/src/lib.rs` (`pub mod legislatura;`)
- Modify: `crates/sim-core/src/partita.rs:76-149` (campi `giocatore`, `legislatura`, `fine`; `fonda`), `:1424-1447` (controlli)
- Modify: `crates/sim-core/src/partiti.rs:93` (`valida`: id `giocatore` riservato)
- Modify: `crates/sim-core/src/comando.rs:78` (guardia unica)
- Modify: `crates/sim-core/src/errore.rs` (`Errore::Fondazione`)
- Modify: `crates/sim-core/src/vista.rs:19-45` (giocatore, fase, turni al voto)
- Modify: `crates/sim-cli/src/main.rs:16-63` (`--posto`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`, `dati/base/scenario.toml:23` (commento della tabella)
- Test: `crates/sim-core/tests/fondazione.rs`, `crates/sim-core/tests/comune/mod.rs` (helper `fondata`)

**Ricerca:** `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:22` (luna di miele per i leader al primo mandato).

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true. Legge `.scratch/v1-giocabile/issues/20-partenza-della-partita.md` (sezioni «Fondazione», «Calendario», «Governo uscente»).

**Interfaces:**
- Consumes: `Partita::nuova` (`crates/sim-core/src/partita.rs:96`), `partiti::iniziali` (`crates/sim-core/src/partiti.rs:60`), `Partito::valence_base` (Task 2).
- Produces:
  - `pub struct Fondazione { pub nome: String, pub posto: f64 }` (`Clone, Debug, PartialEq, Serialize, Deserialize`).
  - `pub fn posti(dati: &DatiGioco) -> Vec<f64>`: i punti medi fra partiti dello scenario consecutivi sull'asse, crescenti (con lo scenario di base 1,5 … 6,5).
  - `pub enum Fase { Sciolta, Trattativa, Ordinaria }` (`Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize`, esportato in TS): `Sciolta` = Camere sciolte, campagna, voto nella chiusura; `Trattativa` = il turno dopo il voto; `Ordinaria` = governo in carica.
  - `pub struct Legislatura { pub inizio: u32, pub scadenza: u32, pub fase: Fase, pub coalizioni: Vec<Vec<usize>> }` con `pub fn turno_di_voto(&self, turno: u32) -> bool` (= `fase == Sciolta || turno == scadenza`) e `pub fn ultimo_anno(&self, turno: u32) -> bool` (= `turno + 4 > scadenza`). `coalizioni` = coalizioni dell'ultimo voto, indici di partito; alla nascita quelle dello scenario.
  - `pub enum Fine { SenzaSeggi }`.
  - `legislatura/mod.rs` ri-esporta con `pub use` i tipi pubblici dei sottomoduli di questo task e dei seguenti: i test li importano da `sim_core::legislatura` (`Fondazione`, `posti`, `Cessione`, `Incarico`, `Assegnazione`).
  - `Partita` guadagna `giocatore: usize`, `legislatura: Legislatura`, `fine: Option<Fine>`; accessori `pub fn giocatore(&self) -> usize`, `pub fn legislatura(&self) -> &Legislatura`, `pub fn finita(&self) -> bool`, `pub fn governa(&self) -> bool` (= `fine.is_none() && fase == Ordinaria && governo.presidente == giocatore`).
  - `pub fn Partita::fonda(dati: DatiGioco, seme: u64, f: &Fondazione) -> Result<Partita, Errore>`: rifiuta `Errore::Fondazione` con nome vuoto dopo `trim` o più lungo di 40 caratteri, con posto non finito o non entro 1e-9 da uno dei `posti(dati)`. Costruisce la partita come `nuova` ma con il partito `Partito { id: "giocatore", nome: f.nome.trim(), posizione_dichiarata: f.posto, posizione_percepita: f.posto, valence: valence_partito_nuovo + luna_di_miele, valence_base: valence_partito_nuovo }` in coda ai partiti, prima di elettorato e informazione (così ogni vettore per partito nasce della misura giusta); governo uscente dello scenario; `giocatore` = indice del partito nuovo; seggi tutti a zero (Camere sciolte, `Parlamento::uscente` non si chiama); `Legislatura { inizio: 0, scadenza: 0, fase: Sciolta, coalizioni: scenario }`.
  - `Partita::nuova` (partita di prova): `giocatore = governo.presidente`, `Legislatura { inizio: 0, scadenza: 4 · durata − 1, fase: Ordinaria, coalizioni: scenario }` con `durata` = leva `durata_legislatura` in vigore (19 con lo scenario di base).
  - `pub fn Partita::anteprima_fondazione(dati: &DatiGioco, seme: u64) -> AnteprimaFondazione`, con `pub struct AnteprimaFondazione { pub istogramma: [f64; PUNTI], pub posti: Vec<Posto> }` e `pub struct Posto { pub posizione: f64, pub quota_attesa: f64 }` (esportati in TS). Per ogni posto: la partita che `fonda` farebbe con quel seme e un nome segnaposto; `quota_attesa` = media sugli istituti del sondaggio del turno zero della quota del giocatore fra i votanti. `istogramma` = media delle posizioni degli incroci regionali del sondaggio, pesata con gli elettori della regione. Stesso seme, stesso sondaggio della partita che si fonda: è la storia 9.
  - Guardia in `Partita::comanda`: a partita finita ogni comando è rifiutato con `"La partita è finita"`; i comandi di governo (tutti quelli esistenti) chiedono `governa()`, altrimenti `"Comando di governo: il giocatore non governa (Camere sciolte)"`, `"… (trattativa in corso)"` o `"… (all'opposizione)"`.
  - In `chiudi_turno`: a partita finita non fa nulla; a Camere sciolte `aula` non vota (non c'è nulla in calendario: nessuno può presentare).
  - Manopole: `luna_di_miele` (valore provvisorio 1,0, min 0, max 3; lo tara il Task 6), `valence_partito_nuovo` (0,0, min −3, max 3). Entrambe con `ticket = "20"`.
  - `Vista` guadagna `giocatore: String` (id), `fase: Fase`, `voto: Trimestre` (la data del turno del prossimo voto), `turni_al_voto: u32`, `governa: bool`, `finita: bool`.
  - CLI: `--posto <x>` usa `Partita::fonda` col nome `Partito del giocatore`; senza, `Partita::nuova` come oggi.
  - `controlla_partiti`: partiti = scenario + 1 se `giocatore` è l'ultimo indice e l'id è `giocatore`, altrimenti partiti = scenario e `giocatore < n`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/fondazione.rs
mod comune;
use comune::{dati_base, edita, fondata};
use sim_core::legislatura::{Fase, Fondazione, posti};
use sim_core::{Comando, Errore, Partita};

#[test]
fn sei_posti_fra_i_sette_partiti() {
    assert_eq!(posti(&dati_base()), vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5]);
}

#[test]
fn la_partita_vera_comincia_a_camere_sciolte() {
    let p = fondata(4.5);
    assert_eq!(p.partiti().len(), 8);
    assert_eq!(p.giocatore(), 7);
    assert_eq!(p.partiti()[7].id, "giocatore");
    assert_eq!(p.legislatura().fase, Fase::Sciolta);
    assert!(!p.governa());
    assert_eq!(p.parlamento().seggi_camera, vec![0; 8]);
    assert_eq!(p.governo().presidente, 5); // destra, il governo uscente
    let dati = dati_base();
    let atteso = dati.manopola("valence_partito_nuovo") + dati.manopola("luna_di_miele");
    assert!((p.partiti()[7].valence - atteso).abs() < 1e-12);
    assert!(p.vista().turni_al_voto == 0);
}

#[test]
fn fondazione_non_valida_rifiutata_col_motivo() {
    for (nome, posto, motivo) in [
        ("  ", 4.5, "nome"),
        ("Partito", 4.0, "4"),
        ("Partito", 0.5, "0.5"),
        ("Partito", f64::NAN, "posto"),
    ] {
        let f = Fondazione { nome: nome.into(), posto };
        match Partita::fonda(dati_base(), 7, &f) {
            Err(Errore::Fondazione(m)) => assert!(m.contains(motivo), "{m}"),
            altro => panic!("{nome:?} {posto}: {altro:?}"),
        }
    }
}

#[test]
fn comandi_di_governo_rifiutati_a_chi_non_governa() {
    let mut p = fondata(4.5);
    let prima = p.salva().unwrap();
    let e = p.comanda(Comando::Ferma { dicastero: "sanita".into() }).unwrap_err();
    assert!(e.to_string().contains("Camere sciolte"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn l_anteprima_della_fondazione_e_il_sondaggio_della_partita() {
    let a = Partita::anteprima_fondazione(&dati_base(), 7);
    assert_eq!(a.posti.len(), 6);
    let p = fondata(4.5);
    let v = p.vista();
    let q = &a.posti[3];
    assert_eq!(q.posizione, 4.5);
    let atteso = v.sondaggio.istituti.iter()
        .map(|r| r.quote[7] / (1.0 - r.quote[8]))
        .sum::<f64>() / v.sondaggio.istituti.len() as f64;
    assert!((q.quota_attesa - atteso).abs() < 1e-12);
    assert!((a.istogramma.iter().sum::<f64>() - 1.0).abs() < 1e-9);
}

#[test]
fn salvataggio_della_partita_vera_si_ricarica() {
    let p = fondata(2.5);
    let q = Partita::carica(&p.salva().unwrap()).unwrap();
    assert_eq!(p, q);
    // Un giocatore fuori dai partiti è rifiutato.
    assert!(edita(&p, |j| j["giocatore"] = serde_json::json!(99)).is_err());
}
```

```rust
// crates/sim-core/tests/comune/mod.rs (in coda)
/// La partita vera col seme 7, fondata al posto `posto`.
#[allow(dead_code, reason = "ogni crate di test usa solo una parte degli helper")]
pub fn fondata(posto: f64) -> Partita {
    let f = sim_core::legislatura::Fondazione { nome: "Partito nuovo".into(), posto };
    Partita::fonda(dati_base(), 7, &f).unwrap()
}
```

In `crates/sim-cli/tests/cli.rs`: `--posto 4.5` esce con codice 0 e il JSON ha `"giocatore": "giocatore"`; `--posto 4` esce con codice 1 e un messaggio che nomina il posto.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test --workspace`
Expected: FAIL, modulo `legislatura` assente.

- [ ] **Step 3: Implementa modulo, costruttore, guardia, vista e CLI**

La guardia sta in un punto solo, in testa a `Partita::comanda` (`crates/sim-core/src/comando.rs:78`): ogni variante esistente di `Comando` è di governo. Il commento della tabella `[parlamento_uscente]` in `dati/base/scenario.toml:23` diventa: «Parlamento della legislatura uscente, solo per la partita di prova dal governo uscente (`Partita::nuova`): la partita vera parte a Camere sciolte (piano 7a, scelta 1)».

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS; i test dei piani 1-6 passano senza modifiche, perché girano su `Partita::nuova`.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core crates/sim-cli dati/base app/src/lib/tipi
git commit -m "feat(core): fondazione del partito e partita a Camere sciolte"
```

### Task 6: Luna di miele e vincibilità

**Files:**
- Modify: `crates/sim-cli/src/tara.rs:134-305` (passo finale sulla luna di miele)
- Modify: `dati/base/manopole.toml` (`luna_di_miele` tarata)
- Test: `crates/sim-core/tests/fondazione.rs`, `crates/sim-cli/tests/cli.rs`

**Ricerca:** `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:22` (luna di miele per i leader al primo mandato); ticket 20, sezioni «Peso del partito nuovo» e «Vincibilità».

**Dispatch:** `rust-engineer` · dopo il Task 5 · skill-gate: true. Nel report: la quota attesa del giocatore per ogni posto e la coalizione contigua migliore per posto, con la sua quota.

**Interfaces:**
- Consumes: `Partita::fonda`, `posti`, `Partita::intenzioni_vere` (`crates/sim-core/src/partita.rs:1085`), `riscrivi` di `tara.rs:87`.
- Produces:
  - `sim-cli tara`, dopo le valence dei partiti IA (tarate sulla partita di prova, sette partiti, come oggi): bisezione su `luna_di_miele` in `[0, 3]` finché la media sui sei posti della quota vera del giocatore fra i votanti (`intenzioni_vere().nazionali`, turno zero, seme 0) è 0,15 ± 0,002; riscrive `valore` di `[luna_di_miele]` in `manopole.toml` e lo stampa. Bersaglio 0,15 nel codice, con la riga della spec.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/fondazione.rs (in coda)
fn quote_vere(p: &Partita) -> Vec<f64> {
    let v = p.intenzioni_vere().nazionali;
    let votanti = 1.0 - v[v.len() - 1];
    v[..v.len() - 1].iter().map(|q| q / votanti).collect()
}

#[test]
fn luna_di_miele_al_quindici_per_cento_del_posto_medio() {
    let media = posti(&dati_base()).iter()
        .map(|&x| quote_vere(&fondata(x))[7])
        .sum::<f64>() / 6.0;
    assert!((media - 0.15).abs() < 0.005, "{media}");
}

#[test]
fn vincibile_da_ogni_posto() {
    // Esiste un blocco di partiti contigui sull'asse, col giocatore, sopra il 42% dei votanti.
    for x in posti(&dati_base()) {
        let p = fondata(x);
        let q = quote_vere(&p);
        let mut ordine: Vec<usize> = (0..q.len()).collect();
        ordine.sort_by(|&a, &b| p.partiti()[a].posizione_dichiarata.total_cmp(&p.partiti()[b].posizione_dichiarata));
        let g = ordine.iter().position(|&j| j == 7).unwrap();
        let migliore = (0..=g).flat_map(|i| (g..ordine.len()).map(move |k| (i, k)))
            .map(|(i, k)| ordine[i..=k].iter().map(|&j| q[j]).sum::<f64>())
            .fold(0.0, f64::max);
        assert!(migliore > 0.42, "posto {x}: {migliore}");
    }
}
```

`crates/sim-cli/tests/cli.rs`: `tara` sui dati di base stampa `luna_di_miele` e, rilanciato sui dati appena riscritti, la lascia entro 0,002.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test fondazione luna_di_miele_al_quindici_per_cento_del_posto_medio`
Expected: FAIL con la `luna_di_miele` provvisoria di 1,0 (o PASS per caso: allora il test resta e il passo di `tara` si scrive comunque).

- [ ] **Step 3: Implementa il passo di `tara` ed eseguilo**

Run: `cargo run -p sim-cli -- tara --dati dati/base --bersagli strumenti/taratura/bersagli.toml`
Il commento di `[luna_di_miele]` dice «tarata da sim-cli tara: media dei sei posti al 15% fra i votanti (spec riga 36)». Se `vincibile_da_ogni_posto` fallisce dopo la taratura, fermarsi e riportarlo: la vincibilità è un bersaglio della spec (riga 285) e la scelta di come recuperarla è di Mario.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-cli dati/base/manopole.toml crates/sim-core/tests
git commit -m "feat(cli): taratura della luna di miele, vincibilità da ogni posto"
```

### Task 7: Coalizione pre-elettorale

**Files:**
- Create: `crates/sim-core/src/legislatura/coalizione.rs`
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.dichiarazione`)
- Modify: `crates/sim-core/src/parlamento/regole.rs:82-117` (`proietta` con le coalizioni passate)
- Modify: `crates/sim-core/src/partita.rs:1161-1170` (`proiezione_seggi` sulle coalizioni dell'ultimo voto), nuova `disponibilita`
- Modify: `crates/sim-core/src/comando.rs` (`DichiaraCoalizione`)
- Modify: `crates/sim-core/src/legislatura/fondazione.rs` (`Posto.disponibilita`)
- Modify: `crates/sim-core/src/vista.rs`, `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/coalizione.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:39` (premio alle liste circoscrizionali della coalizione nel suo insieme); `docs/ricerca/modelli/formazione-di-coalizioni.md:34` (coalizioni dichiarate prima del voto con un unico candidato premier), `:48` (punteggio di una coalizione: vincente, distanza ideologica, partito più grande dentro).

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true. Legge `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md`, sezione «Coalizione pre-elettorale».

**Interfaces:**
- Consumes: `proietta` (`crates/sim-core/src/parlamento/regole.rs:82`), `Legislatura` (Task 5), `Intesa::valori`.
- Produces:
  - `proietta(dati, partiti, sondaggio, legge, leve, coalizioni: &[Vec<usize>])`: le coalizioni passate da chi chiama, non più lette dallo scenario. `StatoAula` e `Partita::proiezione_seggi` passano `legislatura.coalizioni`; nel turno zero della partita vera il giocatore non vi compare e corre da solo (spec riga 322).
  - `pub struct Cessione { pub partito: String, pub seggi: u32 }` (comando, TS).
  - `pub struct Dichiarazione { pub partiti: Vec<usize>, pub cessioni: Vec<(usize, u32)> }`: i membri senza il giocatore; `Legislatura.dichiarazione: Option<Dichiarazione>`, cancellata dal voto.
  - `pub(crate) fn coalizioni_al_voto(ultime: &[Vec<usize>], giocatore: usize, d: Option<&Dichiarazione>) -> Vec<Vec<usize>>`: prima la coalizione del giocatore (lui più i membri dichiarati; senza dichiarazione, la sua coalizione dell'ultimo voto, o lui da solo), poi le coalizioni dell'ultimo voto senza il giocatore e i suoi membri, tolte quelle rimaste di un partito solo (scelta 4). `ultime` = `legislatura.coalizioni`, alla nascita quelle dello scenario.
  - `pub enum CausaDisponibilita { Seggi, Governare, Distanza, Guida, Intesa }` con `Display` in una riga: «più seggi da solo» / «con te», «più probabilità di governare altrove» / «con te», «troppo lontano dalla tua posizione» / «vicino», «guiderebbe la propria coalizione», «intesa bassa con te»; `pub struct VoceDisponibilita { pub partito: String, pub disponibile: bool, pub causa: String }` (TS).
  - Valore atteso per il partito `j` di una disposizione delle coalizioni `c` (la sua coalizione `k` in `c`), sulla proiezione del sondaggio pubblico con la legge e i seggi in vigore e le cessioni applicate come al voto:
    `V = peso_seggi_coalizione · s_j / S + peso_governare · P_k − peso_distanza_coalizione · d_jk + valore_guida · [j guida k] − peso_intesa_coalizione · max(0, intesa_base − intesa_j) / 100 · [j alleato del giocatore in carica e k è del giocatore]`
    con `s_j` seggi di `j` alla Camera più al Senato, `S` seggi dei due rami, `P_k = σ(min(m_c, m_s) / scala_maggioranza)` (`σ` logistica, `m` = seggi di `k` meno la maggioranza del ramo, in quota dei seggi del ramo), `d_jk` distanza media di `j` dagli altri membri di `k` (0 da solo), «guida» = primo per quota nel sondaggio fra i membri, e nella coalizione del giocatore guida sempre il giocatore.
  - `pub(crate) fn disponibilita(...) -> Vec<VoceDisponibilita>`, una voce per partito IA (gli altri partiti, giocatore escluso): disponibile se `V` nella coalizione del giocatore con `j` aggiunto (e le cessioni dichiarate) supera `V` nella disposizione senza; causa = il termine con lo scarto più grande fra le due, col verso che decide.
  - `Comando::DichiaraCoalizione { partiti: Vec<String>, cessioni: Vec<Cessione> }`: ammesso solo nel turno di voto, dal giocatore, a partita non finita (fuori dalla guardia di governo del Task 5). Rifiuta col nome: partito sconosciuto, ripetuto, il giocatore stesso, cessione a un non membro, cessioni oltre i seggi attesi del giocatore, e ogni membro non disponibile con la coalizione intera («Centro non entra: troppo lontano dalla tua posizione»). Sostituisce la dichiarazione precedente; `partiti` vuoto = da solo.
  - `pub fn Partita::disponibilita(&self) -> Vec<VoceDisponibilita>` sulla dichiarazione in corso; `Vista.disponibilita` nel turno di voto, vuota altrove; `Vista.coalizione: Vec<String>` (membri dichiarati).
  - `Posto` guadagna `disponibilita: Vec<VoceDisponibilita>`: per ogni posto, la disponibilità con la coalizione vuota.
  - Manopole (tutte `ticket = "05"`, assunzione del piano 7): `peso_seggi_coalizione` 4,0; `peso_governare` 2,0; `peso_distanza_coalizione` 1,0; `valore_guida` 1,0 (max 10, il valore di `la_guida_ceduta_trattiene_il_grande`); `peso_intesa_coalizione` 2,0; `scala_maggioranza` 0,05.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/coalizione.rs
mod comune;
use comune::fondata;
use sim_core::legislatura::Cessione;
use sim_core::{Comando, Partita};

fn dichiara(p: &mut Partita, partiti: &[&str]) -> Result<(), sim_core::Errore> {
    p.comanda(Comando::DichiaraCoalizione {
        partiti: partiti.iter().map(|s| (*s).into()).collect(),
        cessioni: Vec::new(),
    })
}

#[test]
fn un_vicino_disponibile_entra_un_lontano_no() {
    let mut p = fondata(6.5);
    let v = p.disponibilita();
    let estrema = v.iter().find(|x| x.partito == "estrema_destra").unwrap();
    assert!(estrema.disponibile, "{estrema:?}");
    dichiara(&mut p, &["estrema_destra"]).unwrap();
    let prima = p.salva().unwrap();
    let e = dichiara(&mut p, &["estrema_sinistra"]).unwrap_err();
    assert!(e.to_string().contains("Estrema sinistra"), "{e}");
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn la_guida_ceduta_trattiene_il_grande() {
    // Destra guida centrodestra e destra. Col valore della guida al massimo non entra, e la causa
    // è la guida; centrodestra, che non guida, non cambia.
    let f = comune::imposta(comune::file_base(), "manopole", "[valore_guida]\nvalore = ", "10.0");
    let p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(),
        7,
        &sim_core::legislatura::Fondazione { nome: "Partito nuovo".into(), posto: 5.5 },
    )
    .unwrap();
    let v = p.disponibilita();
    let voce = |id: &str| v.iter().find(|x| x.partito == id).unwrap();
    assert!(!voce("destra").disponibile);
    assert!(voce("destra").causa.contains("guiderebbe"), "{:?}", voce("destra"));
    let base = fondata(5.5).disponibilita();
    let prima = base.iter().find(|x| x.partito == "centrodestra").unwrap();
    assert_eq!(voce("centrodestra").disponibile, prima.disponibile);
}

#[test]
fn cedere_posti_attira_un_partito() {
    let mut p = fondata(4.5);
    let cessioni = |n| vec![Cessione { partito: "centro".into(), seggi: n }];
    let rifiuto = p.comanda(Comando::DichiaraCoalizione { partiti: vec!["centro".into()], cessioni: cessioni(0) });
    if rifiuto.is_err() {
        p.comanda(Comando::DichiaraCoalizione { partiti: vec!["centro".into()], cessioni: cessioni(30) }).unwrap();
    }
    assert_eq!(p.vista().coalizione, vec!["centro".to_string()]);
}

#[test]
fn fuori_dal_turno_di_voto_non_si_dichiara() {
    let mut p = Partita::nuova(comune::dati_base(), 1); // partita di prova, scadenza 19
    let e = dichiara(&mut p, &[]).unwrap_err();
    assert!(e.to_string().contains("turno del voto"), "{e}");
}

#[test]
fn salvare_con_la_coalizione_dichiarata() {
    let mut a = fondata(6.5);
    dichiara(&mut a, &["estrema_destra"]).unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}
```

Se al posto 4,5 il centro accetta anche senza cessioni, `cedere_posti_attira_un_partito` lo verifica comunque (dichiarazione riuscita); il caso che conta è che con 30 seggi ceduti entri. Se con 30 non basta col seme 7, il test sale fino a 60 e riporta il numero nel report.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test coalizione`
Expected: FAIL, comando e tipi assenti.

- [ ] **Step 3: Implementa proiezione con coalizioni, valore atteso, disponibilità e comando**

La disponibilità rifà il riparto per ogni alternativa: due proiezioni per partito IA, nessuna cache (ponytail: 14 riparti a vista; si memorizza quando il profilo lo chiede).

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): coalizione pre-elettorale con disponibilità e cessioni"
```

### Task 8: Il voto

**Files:**
- Create: `crates/sim-core/src/legislatura/voto.rs`
- Modify: `crates/sim-core/src/partita.rs:151-245` (passo del voto dopo il motore), `:1424-1447` (seggi contro il parlamento eletto)
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.elezioni`, `totali`, `incarico`)
- Modify: `crates/sim-core/src/informazione/mod.rs:21-75` (`riestrai_bias`)
- Modify: `crates/sim-core/src/vista.rs`
- Test: `crates/sim-core/tests/voto_turno_zero.rs`

**Ricerca:** `docs/ricerca/modelli/sistemi-elettorali.md:39-46` (premio, riparto di Camera e Senato); `docs/ricerca/modelli/sondaggi-ed-errore.md:50` (bias comune di elezione), `:55` (il bias comune si rivela la notte elettorale).

**Dispatch:** `rust-engineer` · dopo il Task 7 · skill-gate: true.

**Interfaces:**
- Consumes: `Intenzioni::calcola` (`crates/sim-core/src/elettorato/voto.rs:19`), `elezioni::riparto` (`crates/sim-core/src/elezioni/riparto.rs:665`), `coalizioni_al_voto` (Task 7), leve `seggi_camera` e `seggi_senato`.
- Produces:
  - `pub(crate) fn voti_per_regione(dati: &DatiGioco, el: &Elettorato, v: &Intenzioni) -> BTreeMap<String, Vec<u64>>`: per regione e partito `Σ_gruppi della regione round(elettori_i · v_ij)`, senza l'astensione.
  - `pub struct EsitoElezioni { pub data: Trimestre, pub affluenza: f64, pub voti: Vec<VotiPartito>, pub premio: Option<String>, pub coalizioni: Vec<Vec<String>> }` e `pub struct VotiPartito { pub partito: String, pub quota: f64, pub camera: u32, pub senato: u32 }` (TS); `quota` fra i votanti; `premio` = id dei membri di chi l'ha preso, uniti da `+`.
  - `Legislatura` guadagna `elezioni: Vec<EsitoElezioni>` (la più recente per prima, tutte conservate per l'albo del piano 10), `totali: (u32, u32)` (seggi dei rami al voto) e `incarico: Option<Incarico>`.
  - `pub enum Incarico { Giocatore, Ia { coalizione: Vec<usize> } }` (`Clone, Debug, PartialEq, Serialize, Deserialize`: il Task 9 lo confronta con `assert_eq!`): la coalizione del giocatore ha la maggioranza assoluta in entrambe le Camere → `Giocatore`; una coalizione IA ce l'ha → `Ia`; nessuna → `None` (lo tratta il Task 10).
  - Passo del voto in `chiudi_turno`, se `legislatura.turno_di_voto(turno)`, dopo `informazione.statistica.aggiorna` e prima delle soglie: intenzioni vere; coalizioni al voto; riparto con la legge in vigore e i seggi dei rami dalle leve in vigore; cessioni applicate (scelta 5); seggi nuovi in `parlamento`; `voto_precedente` di ogni gruppo = le sue intenzioni; bias comune riestratto (`Informazione::riestrai_bias(&mut self, dati, caso, n_partiti)`, un normale per alternativa); esito in testa a `elezioni`; `coalizioni` = coalizioni al voto; `dichiarazione = None`; `inizio = turno + 1`; `scadenza = turno + 4 · durata` (leva in vigore al voto, scelta 11); `fase = Trattativa`; incarico. Se il partito del giocatore ha zero seggi in entrambe le Camere, `fine = Some(SenzaSeggi)`. Se il riparto rifiuta i voti, i seggi restano quelli di prima e l'esito porta la coalizione e le quote senza seggi (non deve succedere: la legge in vigore è sempre valida, piano 6 Task 9).
  - Nel turno di trattativa l'aula non vota leggi nuove (nessuno può presentarle: Task 5); il governo resta quello uscente fino al Task 9.
  - `Vista.elezioni: Vec<EsitoElezioni>`; `Vista.seggi` come oggi, ora dal voto.
  - `controlla_partiti` (o un `controlla_legislatura` accanto): fuori dal turno zero della partita vera, la somma dei seggi di ogni ramo è quella di `totali`; `scadenza ≥ inizio`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/voto_turno_zero.rs
mod comune;
use comune::{edita, fondata};
use sim_core::legislatura::Fase;
use sim_core::{Comando, Partita};

fn votata(posto: f64, alleati: &[&str]) -> Partita {
    let mut p = fondata(posto);
    p.comanda(Comando::DichiaraCoalizione {
        partiti: alleati.iter().map(|s| (*s).into()).collect(),
        cessioni: Vec::new(),
    }).unwrap();
    p.chiudi_turno();
    p
}

#[test]
fn il_voto_del_turno_zero_elegge_il_parlamento() {
    let p = votata(6.5, &["estrema_destra"]);
    let l = p.legislatura();
    assert_eq!(l.fase, Fase::Trattativa);
    assert_eq!((l.inizio, l.scadenza), (1, 20));
    assert_eq!(p.parlamento().seggi_camera.iter().sum::<u32>(), 400);
    assert_eq!(p.parlamento().seggi_senato.iter().sum::<u32>(), 200);
    assert!(p.parlamento().seggi_camera[7] > 0);
    let e = &p.vista().elezioni[0];
    assert_eq!(e.voti.len(), 8);
    assert!((e.voti.iter().map(|v| v.quota).sum::<f64>() - 1.0).abs() < 1e-9);
    assert_eq!(e.coalizioni[0], vec!["giocatore".to_string(), "estrema_destra".into()]);
}

#[test]
fn dopo_il_voto_l_intenzione_parte_dal_voto() {
    let p = votata(4.5, &[]);
    let v = p.intenzioni_vere();
    let e = &p.vista().elezioni[0];
    // Nessun turno è passato dal voto: l'intenzione del turno dopo è vicina al voto.
    assert!((v.nazionali[7] / (1.0 - v.nazionali[8]) - e.voti[7].quota).abs() < 0.02);
}

#[test]
fn senza_seggi_la_partita_finisce() {
    // Luna di miele azzerata e valence del partito nuovo molto bassa: sotto il 3% da solo.
    let f = comune::imposta(comune::file_base(), "manopole", "[valence_partito_nuovo]\nvalore = ", "-3.0");
    let f = comune::imposta(f, "manopole", "[luna_di_miele]\nvalore = ", "0.0");
    let mut p = Partita::fonda(
        sim_core::dati::DatiGioco::da_testi(&f).unwrap(), 7,
        &sim_core::legislatura::Fondazione { nome: "Partito nuovo".into(), posto: 1.5 },
    ).unwrap();
    p.chiudi_turno();
    assert!(p.finita());
    let prima = p.salva().unwrap();
    p.chiudi_turno();
    assert_eq!(p.salva().unwrap(), prima);
    assert!(p.comanda(Comando::DichiaraCoalizione { partiti: vec![], cessioni: vec![] }).is_err());
}

#[test]
fn seggi_che_non_tornano_col_parlamento_eletto_rifiutati() {
    let p = votata(6.5, &["estrema_destra"]);
    assert!(edita(&p, |j| j["parlamento"]["seggi_camera"][0] = serde_json::json!(999)).is_err());
}
```

Se `valence_partito_nuovo = -3,0` è fuori dall'intervallo della manopola, il test usa il minimo dell'intervallo e, se non basta a stare sotto soglia, alza `soglia_lista` con `imposta` sulla legge 180 di `elezioni.toml`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test voto_turno_zero`
Expected: FAIL, il voto non c'è.

- [ ] **Step 3: Implementa il voto**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(core): il voto nella chiusura del turno, parlamento eletto"
```

---

## PR C · Governo e legislatura (Task 9-11)

### Task 9: Spartizione, tecnici e governo nuovo

**Files:**
- Create: `crates/sim-core/src/legislatura/spartizione.rs`
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.proposta`)
- Modify: `crates/sim-core/src/ministri/mod.rs:101-110` (`Ministro.tecnico`), `:175-226` (`nuovo_governo`, tecnici in `genera`)
- Modify: `crates/sim-core/src/parlamento/intesa.rs:13-30` (`CausaIntesa::Spartizione`)
- Modify: `crates/sim-core/src/partiti.rs:35-58` (`Governo.coalizione`)
- Modify: `crates/sim-core/src/partita.rs` (formazione alla chiusura della trattativa), `crates/sim-core/src/comando.rs` (`ProponiSpartizione`)
- Modify: `crates/sim-core/src/elettorato/mod.rs:345-353` (giudizio azzerato per chi entra)
- Modify: `crates/sim-core/src/vista.rs`, `crates/sim-core/src/dati.rs:65`, `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/spartizione.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:16` (legge di Gamson); `:20-22` (nessun premio al formateur oltre la presidenza); `:29-30` (pesi provvisori, intercetta 0,07 e pendenza 0,79).

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true. Legge `.scratch/v1-giocabile/issues/05-coalizione-e-crisi.md`, sezioni «Dopo il voto» e «Spartizione».

**Interfaces:**
- Consumes: `Incarico` (Task 8), `Dicastero::peso_spartizione` (`crates/sim-core/src/dati.rs:256`), `Ministri::genera` (`crates/sim-core/src/ministri/mod.rs:192`), `Intesa::varia`.
- Produces:
  - `pub(crate) fn richiesta(dati: &DatiGioco, seggi_j: u32, seggi_maggioranza: u32) -> f64`: `(gamson_intercetta + gamson_pendenza · seggi_j / seggi_maggioranza) · Σ peso_spartizione`, seggi di Camera più Senato; 0 se `seggi_maggioranza = 0`.
  - `pub(crate) fn spartizione_di_default(dati, coalizione: &[usize], guida: usize, seggi: &[u32]) -> Vec<usize>`: scelta 6, un partito per dicastero nell'ordine dei dati; parità sciolte per indice di partito.
  - `pub struct Assegnazione { pub dicastero: String, pub partito: Option<String> }` (`None` = tecnico; TS); `pub struct VoceRichiesta { pub partito: String, pub richiesta: f64, pub offerta: f64 }` (TS).
  - `Legislatura.proposta: Option<Vec<Option<usize>>>`, un posto per dicastero (`None` = tecnico).
  - `Comando::ProponiSpartizione { dicasteri: Vec<Assegnazione> }`: ammesso in trattativa con `Incarico::Giocatore`. Rifiuta col nome: dicastero sconosciuto, ripetuto o mancante (servono tutti e dieci), partito fuori dalla coalizione. Sostituisce la proposta precedente.
  - `pub fn Partita::richieste(&self) -> Vec<VoceRichiesta>`: una voce per alleato, offerta = punti di peso dei dicasteri che la proposta in corso gli dà (0 senza proposta); `Vista.richieste` in trattativa.
  - `Governo` guadagna `coalizione: Vec<usize>` (i partiti della maggioranza, presidente compreso) e `al_governo(p)` vale anche per un membro della coalizione senza dicasteri: un alleato a cui la proposta non dà nulla resta in maggioranza, con l'intesa che paga l'offerta. Nella partita di prova `coalizione` = i partiti che tengono un dicastero più il presidente (`crates/sim-core/src/partiti.rs:42`, `iniziali` a `:60`).
  - Formazione alla chiusura del turno di trattativa, dopo le soglie: con `Incarico::Giocatore`, la proposta (o quella di default) diventa `Governo { presidente: giocatore, dicasteri, coalizione }`, coi tecnici al partito del giocatore; ogni alleato parte con intesa `intesa_base + intesa_per_punto · (offerta − richiesta)`, limitata a 0-100, con la variazione `CausaIntesa::Spartizione`. Con `Incarico::Ia`: governo col primo partito per seggi della coalizione e la spartizione di default. Poi: ministri tutti nuovi (`Ministri::nuovo_governo(&mut self, cx, caso, tecnici: &[bool])`, rodaggio, rose vuote, agende riaperte all'apertura); giudizio azzerato in ogni gruppo per i partiti che entrano al governo (spec riga 312, «da zero per chi entra al governo»); `fase = Ordinaria`; `incarico = None`; `proposta = None`.
  - `Ministro.tecnico: bool`; con `tecnico`, `genera` usa la scelta 7.
  - `CausaIntesa::Spartizione` con `Display` «quota dei dicasteri rispetto alla richiesta».
  - Manopole: `gamson_intercetta` 0,07 e `gamson_pendenza` 0,79 (`ticket = "05"`, fonte `formazione-di-coalizioni:30`); `intesa_per_punto` 10,0 (assunzione del piano 7).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/spartizione.rs
mod comune;
use comune::fondata;
use sim_core::legislatura::{Assegnazione, Fase, Incarico};
use sim_core::{Comando, Partita};

/// Al posto 6,5 con destra, centrodestra ed estrema destra: con la 180 la coalizione prende il
/// premio col seme 7 (se non lo prende, il test lo dice e va scelto un altro posto o seme).
fn vinta() -> Partita {
    let mut p = fondata(6.5);
    let alleati: Vec<String> = p.disponibilita().into_iter()
        .filter(|v| v.disponibile && ["destra", "centrodestra", "estrema_destra"].contains(&v.partito.as_str()))
        .map(|v| v.partito).collect();
    p.comanda(Comando::DichiaraCoalizione { partiti: alleati, cessioni: vec![] }).unwrap();
    p.chiudi_turno();
    assert_eq!(p.legislatura().incarico, Some(Incarico::Giocatore), "{:?}", p.vista().elezioni[0]);
    p
}

fn tutto_a(p: &Partita, partito: Option<&str>) -> Vec<Assegnazione> {
    p.vista().dicasteri.iter()
        .map(|d| Assegnazione { dicastero: d.id.clone(), partito: partito.map(Into::into) })
        .collect()
}

#[test]
fn la_richiesta_segue_gamson() {
    let p = vinta();
    let r = p.richieste();
    assert!(!r.is_empty());
    let pesi = 14.0; // 3 + 2 + 2 + 7 · 1
    for v in &r {
        let j = p.partiti().iter().position(|x| x.id == v.partito).unwrap();
        let s = |x: &[u32]| x[j] as f64;
        let pl = p.parlamento();
        let maggioranza: f64 = p.legislatura().coalizioni[0].iter()
            .map(|&k| (pl.seggi_camera[k] + pl.seggi_senato[k]) as f64).sum();
        let attesa = (0.07 + 0.79 * (s(&pl.seggi_camera) + s(&pl.seggi_senato)) / maggioranza) * pesi;
        assert!((v.richiesta - attesa).abs() < 1e-9, "{v:?}");
    }
}

#[test]
fn l_avarizia_costa_intesa_la_generosita_la_alza() {
    let mut avara = vinta();
    avara.comanda(Comando::ProponiSpartizione { dicasteri: tutto_a(&avara, Some("giocatore")) }).unwrap();
    avara.chiudi_turno();
    let mut di_default = vinta();
    di_default.chiudi_turno();
    for &j in &avara.alleati() {
        assert!(avara.intesa().valori[j] < di_default.intesa().valori[j]);
    }
    assert_eq!(avara.legislatura().fase, Fase::Ordinaria);
    assert!(avara.governa());
}

#[test]
fn i_tecnici_stanno_nella_quota_del_giocatore() {
    let mut p = vinta();
    p.comanda(Comando::ProponiSpartizione { dicasteri: tutto_a(&p, None) }).unwrap();
    p.chiudi_turno();
    let g = p.giocatore();
    assert!(p.governo().dicasteri.iter().all(|&k| k == g));
    assert!(p.ministri().dicasteri.iter().all(|s| s.ministro.as_ref().unwrap().tecnico));
}

#[test]
fn proposte_non_valide_rifiutate_col_nome() {
    let mut p = vinta();
    let prima = p.salva().unwrap();
    let mut a = tutto_a(&p, Some("giocatore"));
    a.pop();
    assert!(p.comanda(Comando::ProponiSpartizione { dicasteri: a }).unwrap_err().to_string().contains("Esteri"));
    let a = tutto_a(&p, Some("sinistra"));
    assert!(p.comanda(Comando::ProponiSpartizione { dicasteri: a }).unwrap_err().to_string().contains("Sinistra"));
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn chi_entra_al_governo_e_giudicato_da_zero() {
    let p = vinta();
    let g = p.giocatore();
    // Un giudizio residuo sul partito del giocatore, come da un governo passato.
    let mut p = edita(&p, |j| {
        for x in j["elettorato"]["gruppi"].as_array_mut().unwrap() {
            x["giudizio"][g] = serde_json::json!(1.0);
        }
    })
    .unwrap();
    p.chiudi_turno();
    let json: serde_json::Value = serde_json::from_str(&p.salva().unwrap()).unwrap();
    assert!(json["elettorato"]["gruppi"].as_array().unwrap().iter()
        .all(|x| x["giudizio"][g].as_f64().unwrap() == 0.0));
}

#[test]
fn salvare_a_meta_trattativa() {
    let mut a = vinta();
    a.comanda(Comando::ProponiSpartizione { dicasteri: tutto_a(&a, Some("giocatore")) }).unwrap();
    let mut b = Partita::carica(&a.salva().unwrap()).unwrap();
    a.chiudi_turno();
    b.chiudi_turno();
    assert_eq!(a.salva().unwrap(), b.salva().unwrap());
}
```

`chi_entra_al_governo_e_giudicato_da_zero` regge perché nel turno di trattativa il motore non accumula giudizio sul partito del giocatore (non è al governo) e la formazione viene dopo le soglie. `use comune::edita;` in testa al file. Se il nome del dicastero mancante non è «Esteri e Difesa» (ultimo nell'ordine dei dati), l'asserzione usa il nome dell'ultimo di `vista().dicasteri`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test spartizione`
Expected: FAIL.

- [ ] **Step 3: Implementa richiesta, proposta, formazione e tecnici**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): spartizione di Gamson, tecnici, governo nuovo"
```

### Task 10: Allargamento, governo IA e scioglimento

**Files:**
- Modify: `crates/sim-core/src/legislatura/spartizione.rs` (`maggioranza`, `maggioranza_contigua`)
- Modify: `crates/sim-core/src/legislatura/coalizione.rs` (disponibilità dopo il voto)
- Modify: `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.allargati`)
- Modify: `crates/sim-core/src/partita.rs` (esito della trattativa, segnaposto del governo IA), `crates/sim-core/src/comando.rs` (`Allarga`)
- Modify: `crates/sim-core/src/vista.rs`
- Test: `crates/sim-core/tests/governo_ia.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:13` (coalizione minima connessa vincente); `:49` (formateur con la presidenza).

**Dispatch:** `rust-engineer` · dopo il Task 9 · skill-gate: true.

**Interfaces:**
- Consumes: `Incarico`, `richiesta`, `spartizione_di_default`, `Ministri::nuovo_governo` (Task 9), il valore atteso del Task 7.
- Produces:
  - `pub(crate) fn maggioranza(seggi_c: &[u32], seggi_s: &[u32], coalizione: &[usize]) -> bool`: più della metà dei seggi in entrambe le Camere.
  - `pub(crate) fn maggioranza_contigua(partiti: &[Partito], seggi_c: &[u32], seggi_s: &[u32], escluso: Option<usize>) -> Option<Vec<usize>>`: fra le finestre di partiti consecutivi per posizione dichiarata, senza `escluso`, quelle con la maggioranza; la più piccola per numero di partiti, poi la più grande per seggi, poi la più a sinistra. Il 7b la riusa per il ribaltone.
  - Senza incarico al voto (Task 8): se la coalizione del giocatore è la prima per seggi alla Camera, `Incarico::Giocatore` anche senza maggioranza, e nel turno di trattativa vale `Comando::Allarga { partito: String }`: il partito entra se la sua disponibilità dopo il voto è positiva, cioè se il valore atteso del Task 7, con i seggi veri al posto di quelli proiettati e `P = 1` con la maggioranza raggiunta (0 senza), supera quello di restarne fuori. Rifiuta col nome e con la causa; un partito entrato resta fino alla formazione, in `Legislatura.allargati: Vec<usize>` (svuotato alla formazione e allo scioglimento). Non si scrive in `coalizioni`, che resta il record dell'ultimo voto e il default della dichiarazione successiva. Altrimenti l'incarico passa al Capo dello Stato.
  - Alla chiusura della trattativa: coalizione del giocatore con la maggioranza → Task 9; altrimenti `maggioranza_contigua(…, Some(giocatore))` → governo IA col primo partito per seggi e la spartizione di default; altrimenti scioglimento: `fase = Sciolta`, il voto si tiene nella chiusura del turno dopo con le coalizioni dell'ultimo voto e la dichiarazione da rifare.
  - `Vista.incarico: Option<String>` («giocatore», «Capo dello Stato»), `Vista.disponibilita` anche in trattativa con incarico al giocatore.
  - Segnaposto del governo IA (scelta 10): se il giocatore non governa in fase ordinaria, all'inizio di `chiudi_turno` le proposte aperte restano ignorate (regola esistente) e, nel turno di ottobre-dicembre senza bilancio presentato, il governo presenta `bozza_bilancio()` se esiste. `// ponytail: governo IA minimo, il piano 8 lo sostituisce`.
  - Guardia del Task 5: all'opposizione il motivo è «all'opposizione».

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/governo_ia.rs
mod comune;
use comune::{fondata, imposta, file_base};
use sim_core::legislatura::{Fase, Fondazione};
use sim_core::{Comando, Partita};

/// Proporzionale puro (`dati/base/elezioni.toml:307`): nessun premio, coalizioni facoltative.
fn proporzionale(f: sim_core::dati::FileDati, posto: f64) -> Partita {
    let f = imposta(f, "scenario", "legge_elettorale = \"", "proporzionale_puro\"");
    Partita::fonda(sim_core::dati::DatiGioco::da_testi(&f).unwrap(), 7,
        &Fondazione { nome: "Partito nuovo".into(), posto }).unwrap()
}

#[test]
fn chi_perde_il_voto_va_all_opposizione() {
    let mut p = fondata(1.5); // da solo all'estrema sinistra
    p.chiudi_turno(); // voto
    p.chiudi_turno(); // trattativa
    assert_eq!(p.legislatura().fase, Fase::Ordinaria);
    assert!(!p.governa());
    assert!(!p.governo().al_governo(p.giocatore()));
    let e = p.comanda(Comando::Ferma { dicastero: "sanita".into() }).unwrap_err();
    assert!(e.to_string().contains("all'opposizione"), "{e}");
}

#[test]
fn senza_premio_si_allarga() {
    let mut p = proporzionale(file_base(), 3.5);
    p.chiudi_turno();
    if p.vista().incarico.as_deref() == Some("giocatore") {
        let fuori: Vec<String> = p.disponibilita().into_iter().filter(|v| v.disponibile).map(|v| v.partito).collect();
        for x in fuori {
            p.comanda(Comando::Allarga { partito: x }).unwrap();
        }
    }
    p.chiudi_turno();
    // Un governo c'è, del giocatore o del Capo dello Stato, oppure le Camere sono sciolte.
    let l = p.legislatura();
    assert!(l.fase == Fase::Ordinaria || l.fase == Fase::Sciolta);
}

#[test]
fn due_scioglimenti_di_fila() {
    // Proporzionale puro, valence dei tre partiti IA più grandi al minimo: nessuna maggioranza
    // contigua senza il giocatore, e la coalizione del giocatore, da solo, non ce l'ha.
    let mut f = file_base();
    for id in ["sinistra", "destra", "centrosinistra"] {
        f = comune::sostituisci_valence(f, id, -3.0);
    }
    let mut p = proporzionale(f, 3.5);
    for _ in 0..6 {
        p.chiudi_turno();
    }
    assert!(p.vista().elezioni.len() >= 2);
    assert!(Partita::carica(&p.salva().unwrap()).is_ok());
}

#[test]
fn il_governo_ia_presenta_il_bilancio() {
    let mut p = fondata(1.5);
    for _ in 0..4 {
        p.chiudi_turno(); // voto, trattativa, aprile-giugno, luglio-settembre 2027
    }
    // Ottobre-dicembre 2027: il bilancio del 2028 lo presenta il governo IA.
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.forma == sim_core::parlamento::FormaMisura::Bilancio));
}
```

Test d'unità in `legislatura/spartizione.rs`, `maggioranza_contigua_minima`: sette partiti alle posizioni 1-7, seggi scritti a mano (Camera 400, Senato 200); la finestra di due partiti con la maggioranza vince su quella di tre; con `escluso` dentro l'unica finestra possibile, `None`; a parità di numero, la finestra con più seggi.

`sostituisci_valence(f, id, v)` è un helper nuovo di `tests/comune/mod.rs`: nella sezione `[[partito]]` di id `id` dello scenario, la riga `valore = …` sotto `[partito.valence]` diventa `v` (stessa tecnica di `con_ingresso`, `tests/comune/mod.rs:68`). Se col seme 7 una maggioranza contigua esiste comunque, il test abbassa anche la valence di `centrodestra` e lo commenta. Se `chi_perde_il_voto_va_all_opposizione` dà l'incarico al giocatore, il test abbassa `valence_partito_nuovo` al minimo.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test governo_ia`
Expected: FAIL.

- [ ] **Step 3: Implementa allargamento, Capo dello Stato, scioglimento e segnaposto**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(core): allargamento, governo IA e scioglimento senza maggioranza"
```

### Task 11: Scadenza e fine della legislatura

**Files:**
- Modify: `crates/sim-core/src/legislatura/voto.rs` (fine della legislatura al voto)
- Modify: `crates/sim-core/src/partita.rs:820-835` (`promuovi_referendum` nell'ultimo anno)
- Modify: `crates/sim-core/src/parlamento/mod.rs` (`decadi_tutto`)
- Modify: `crates/sim-core/src/vista.rs`
- Test: `crates/sim-core/tests/legislatura.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:80` (a fine legislatura i progetti non conclusi decadono); `:135` (nessuna richiesta nell'anno prima della scadenza, referendum indetto sospeso dallo scioglimento); `:153` (arma di metà legislatura).

**Dispatch:** `rust-engineer` · dopo il Task 10 · skill-gate: true.

**Interfaces:**
- Consumes: passo del voto (Task 8), `Legislatura::ultimo_anno` (Task 5), `Disposizioni.voci[d].bloccati` (`crates/sim-core/src/disposizioni/stato.rs:27`), `Parlamento` (`crates/sim-core/src/parlamento/mod.rs:113-124`).
- Produces:
  - Al voto, prima del riparto (scelta 12): `pub(crate) fn Parlamento::decadi_tutto(&mut self, dati: &DatiGioco, data: Trimestre)`: ogni misura in calendario esce con un esito `decaduta = "Decaduta per la fine della legislatura"`, senza voto né costi né blocco; `respinte` svuotata; l'abrogativo indetto e non ancora votato esce con un esito `decaduto = "Sospeso per lo scioglimento delle Camere"`, nessuna valence; i confermativi restano. Poi `bloccati` svuotati in ogni disposizione.
  - `promuovi_referendum` rifiuta nell'ultimo anno: «Nessuna richiesta di referendum nell'ultimo anno della legislatura».
  - La partita di prova vota alla chiusura del turno 19 come la vera alla chiusura del turno 20: il turno di voto ammette `DichiaraCoalizione`; senza dichiarazione valgono le coalizioni dell'ultimo voto (`coalizioni_al_voto`, Task 7), quindi nella partita di prova la destra corre ancora col centrodestra.
  - `Vista.voto` e `Vista.turni_al_voto` leggono `scadenza` (Task 5); nel turno di voto `turni_al_voto = 0`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/legislatura.rs
mod comune;
use comune::{dati_base, presenta, senza_shock};
use sim_core::dati::DatiGioco;
use sim_core::legislatura::Fase;
use sim_core::Partita;

fn al_turno(p: &mut Partita, t: u32) {
    while p.vista().turno < t {
        p.chiudi_turno();
    }
}

#[test]
fn a_fine_legislatura_le_leggi_in_esame_decadono_e_i_gradini_si_sbloccano() {
    let p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    // Un gradino bloccato a mano, come dopo un referendum.
    let k = dati_base().disposizione("licenziamento").unwrap();
    let mut p = comune::edita(&p, |j| {
        j["disposizioni"]["voci"][k]["bloccati"] = serde_json::json!([0]);
    }).unwrap();
    al_turno(&mut p, 19);
    presenta(&mut p, &[("tetto_contante", "1000")]).unwrap();
    p.chiudi_turno(); // voto
    let v = p.vista();
    assert!(v.calendario.is_empty());
    assert!(v.esiti.iter().any(|e| e.decaduta.as_deref() == Some("Decaduta per la fine della legislatura")));
    assert!(v.disposizioni.iter().all(|d| d.gradini.iter().all(|g| !g.bloccato)));
    assert_eq!(p.legislatura().fase, Fase::Trattativa);
}

#[test]
fn nessun_referendum_nell_ultimo_anno() {
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 16);
    let e = p.promuovi_referendum("sinistra", "licenziamento").unwrap_err();
    assert!(e.to_string().contains("ultimo anno"), "{e}");
}

#[test]
fn la_durata_cambiata_vale_dalla_legislatura_dopo() {
    // `durata_legislatura` a 4 anni dal turno 3: la scadenza in corso resta 19, la prossima è
    // 19 + 4 · 4 = 35.
    let f = comune::con_arrivo(comune::file_base(), "durata_legislatura", "4", 2027, 3);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    al_turno(&mut p, 4);
    assert_eq!(p.in_vigore("durata_legislatura"), Some("4"));
    assert_eq!(p.legislatura().scadenza, 19);
    al_turno(&mut p, 20);
    assert_eq!(p.legislatura().scadenza, 35);
}
```

`con_arrivo` scrive l'arrivo sulla riga `in_arrivo` della disposizione (`tests/comune/mod.rs:115`); se il catalogo rifiuta un arrivo su una norma costituzionale, il test cambia il gradino con `edita` sul salvataggio al turno 3.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test legislatura`
Expected: FAIL.

- [ ] **Step 3: Implementa fine della legislatura e blocco dell'ultimo anno**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(core): scadenza della legislatura, decadenza e blocchi dei referendum"
```

---

## Rimandato

- **Piano 7b:** tutto l'elenco di «Perché due piani»; in più la rottura dell'impegno pre-elettorale (un partito che lascia una coalizione IA per allargare quella del giocatore paga intesa e sondaggi, ticket 05), l'intesa «quota sotto la richiesta» come causa che dura, la disponibilità che legge le promesse.
- **Piano 8:** governo IA vero (proposte, bilancio, agenda del Presidente del Consiglio avversario) al posto del segnaposto del Task 10; coalizioni IA che si ricompongono fra loro; partito di protesta con la sua luna di miele.
- **Piano 10:** l'albo legge `Legislatura.elezioni`.
- **Piano 11:** schermata della fondazione sopra `AnteprimaFondazione`, schermate di coalizione e trattativa; `EsitoReferendum.decaduto` mostrato.
- **Piano 12:** tarare le manopole della disponibilità e del ritorno della valence contro i bersagli del costo del governare.
