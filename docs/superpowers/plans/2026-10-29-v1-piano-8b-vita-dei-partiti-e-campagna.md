# V1 · Piano 8b: vita dei partiti e campagna — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Il sistema dei partiti si muove. Nel turno prima del voto ogni partito IA sposta la posizione dichiarata con la sua regola di movimento, il giocatore si riposiziona di mezzo punto, può nascere un partito di protesta e le coalizioni IA si ricompongono col valore atteso. Quando la posizione percepita si allontana di un punto dalla dichiarata un partito si scinde, anche quello del giocatore; un partito IA senza seggi per due elezioni si scioglie; i partiti attivi non superano mai dieci. Ogni partito ha una cassa (2 per mille, donazioni, dote) e la spende in campagna su fino a tre bersagli, con le leve mobilitare, persuadere e convincere; fa fino a due promesse, che al voto dopo danno o tolgono valence. Il governo avversario nasce col valore atteso del formatore.

**Architecture:** Il numero dei partiti cambia senza togliere elementi dai vettori: un partito sciolto resta al suo indice con `sciolto = true`, fuori dal voto (utilità −∞), dalle coalizioni e dalle mosse. Un partito nuovo entra in coda con `Partita::aggiungi_partito`, l'unico punto che allunga tutti i vettori per partito (elettorato, informazione, parlamento, intesa), con la voce nuova inserita prima dell'astensione dove c'è. Le regole pure stanno vicino alle loro simili: regola di movimento in `legislatura/movimento.rs`, cassa in `legislatura/cassa.rs`, bersagli, leve e spinte in `legislatura/campagna.rs`; le decisioni IA della campagna in `partita/ia.rs` (scelte di coalizione, spesa) e nel sottomodulo nuovo `partita/vita.rs` (protesta, scissione, scioglimento). La campagna ha un punto d'ingresso in apertura, `apertura_campagna`, chiamato nella chiusura quando il turno nuovo è di voto (mai al turno zero), dopo arrivi e decadenze e prima di emendamenti, rose e `apertura_ia` (`partita.rs:396-428`), così lo stato dell'aula del turno del voto vede posizioni e coalizioni nuove; la spesa avviene nella chiusura del turno del voto, prima del motore.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 102 (riga 151), 108-109 (righe 157-158), 122 (riga 177), 125-129 (righe 183-187); «Partiti IA» (righe 350-351 regola di movimento e posizione dichiarata, 355 vita dei partiti); «Informazione e cassa del partito» (riga 372); «Campagna elettorale» (righe 376-381). Ticket: `.scratch/v1-giocabile/issues/18-partiti-ia.md` (riga 12 coalizioni sul valore atteso, righe 61-65 vita dei partiti, riga 70 riposizionamento), `26-campagna-elettorale.md` (righe 24-56), `25-informazione-imperfetta.md` (cassa), `28-salvataggi.md` (cassa, promesse in sospeso). Rimandi: piano 8a `:901-906` (Rimandato, perimetro di questo piano), scelta 6 dell'8a (`:41`, governo avversario col valore atteso). Gli acquisti d'informazione (storie 123-124) sono del piano 9 (tabella del piano 1, riga 49).

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · vita | `feat/v1-piano-8b-vita` | `main` | 0-3 | rilievi dell'8a, schema 10, regola di movimento nei dati, partiti che entrano ed escono a metà partita, cassa, scioglimento e tetto |
| B · movimento | `feat/v1-piano-8b-movimento` | `feat/v1-piano-8b-vita` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 4-7 | apertura della campagna, movimento IA e riposizionamento, protesta, scissione, coalizioni IA e governo avversario col valore atteso |
| C · campagna | `feat/v1-piano-8b-campagna` | `feat/v1-piano-8b-movimento` (impilata, stessa regola) | 8-10 | spinte di campagna, comando e anteprima, campagna IA, promesse |

Questo file entra nel primo commit della PR A, insieme agli aggiornamenti di `docs/ricerca/` di questo piano.

## Decisioni di Mario

Del 29/10/2026, sulle domande di questo piano (memoria `decisioni-piano-8b`), dopo le ricerche `docs/ricerca/modelli/vita-dei-partiti.md`, `finanziamento-dei-partiti.md`, `effetti-della-campagna-e-promesse.md`:

1. **Scala della campagna: 0,5-1,5 punti nazionali a cassa piena** (la spec, riga 378, diceva 1-3; `effetti-della-campagna-e-promesse.md:51`). A parità di spesa persuadere ≥ mobilitare > convincere (`:55`). Task 8.
2. **Frangia di un partito di governo: decide col valore atteso.** Nasce in maggioranza e resta finché le conviene; esce, con la crisi se la maggioranza non regge, quando il valore atteso del voto supera quello del restare (la spec, riga 355, la faceva uscire subito; `vita-dei-partiti.md:61-67`: nessuna frangia italiana 2010-2026 è uscita subito). Task 6.
3. **Promesse come nella spec, senza «alza l'aspettativa»** (spec riga 379; Markwat 2023 in `effetti-della-campagna-e-promesse.md:44`): si valutano solo se il giocatore è stato Presidente del Consiglio per almeno 4 turni in quella legislatura. Task 10.

Restano valide le decisioni dei piani 6, 7a, 7b e 8a (memorie `decisioni-piano-6-aperte`, `decisioni-piano-7`, `decisioni-piano-7b`, `decisioni-piano-8`).

## Scelte del piano, da confermare in review

1. **`VERSIONE_SCHEMA` passa a 10** (Task 1) e resta 10 per tutto il piano; i campi nuovi dei task successivi entrano senza alzarla, come nell'8a.
2. **Scioglimento come flag.** Un partito sciolto resta nei vettori al suo indice: nessun `usize` salvato cambia significato. Utilità −∞ nel voto, quota zero nel sondaggio, nessun seggio, fuori da coalizioni, fila, campagna e mosse; nella vista resta con `sciolto: true`, perché gli esiti passati lo citano per id. Il tetto di 10 conta i partiti attivi.
3. **Partiti nuovi.** `aggiungi_partito` li mette in coda: dopo una nascita il giocatore non è più l'ultimo indice, e `controlla_partiti` smette di assumerlo. Id `nuovo_<indice>`; nome estratto da `nomi_partiti` (lista nuova dello scenario, testi nella PR per la revisione di Mario) con `Flusso::PartitiIa`, leader estratto come in `cambia_leader` (`partita/ia.rs:30-57`). Bias e effetti d'istituto del partito nuovo si estraggono da `Flusso::Sondaggi`: la sequenza delle estrazioni cambia solo nelle partite dove nasce un partito.
4. **Regola di movimento** (spec riga 350, ticket 18): Sticker fermo; Aggregator verso il baricentro di chi l'ha votato all'ultimo voto, letto sugli incroci regionali del sondaggio pubblico come la stima del referendum IA (scelta 17 dell'8a, `elettorato/referendum.rs:127-150`); Hunter ripete il passo se all'ultimo voto ha guadagnato quota sul voto prima, altrimenti lo inverte, e senza due voti alle spalle va verso il centro (4); Governator fermo se è al governo, altrimenti verso la posizione del governo (media delle dichiarate della coalizione pesata coi seggi, scelta 7 dell'8a) senza superarla. Passo massimo `passo_movimento` = 0,5 (`vita-dei-partiti.md:122`); mai oltre la posizione del vicino attivo (uguale ammessa), mai fuori da 1-7. Il partito del giocatore ha regola `Sticker` e si muove solo col comando. I partiti si muovono in ordine d'indice, ciascuno sulle posizioni già mosse.
5. **Percepita a metà strada.** Ogni spostamento della dichiarata, IA o del giocatore, sposta la percepita della metà nella stessa direzione (ticket 26, riga 32). Oggi la dichiarata non si muove mai fuori dai test.
6. **Protesta.** Nell'apertura della campagna, al più una a legislatura (`Legislatura.protesta`, azzerata al voto), con meno di 10 attivi. Candidati: i 7 punti dell'asse. Un punto `k` è scoperto se la sua quota nazionale di elettori (istogrammi pesati con gli elettori) è almeno `soglia_protesta` = 10% (`vita-dei-partiti.md:111`), se dista almeno `distanza_protesta` = 1 dalla dichiarata del partito attivo più vicino (`:112`), e se l'astensione vera dei suoi elettori supera quella nazionale (`:113`). Nasce sul punto scoperto con la quota più alta (a parità il più basso): Hunter, identitario, valence `valence_partito_nuovo + luna_di_miele` come il partito fondato, cassa `dote_partito_nuovo`, nessun seggio fino al voto, nessun voto precedente (gli elettori arrivano col logit).
7. **Scissione.** Controllo in chiusura dopo l'aula (dove la percepita scivola coi voti, `partita.rs:705-715`) e prima del motore, per tutti i partiti attivi, giocatore compreso. Distanza percepita − dichiarata ≥ `soglia_minaccia` = 0,5: una riga di minaccia, una volta finché la distanza non torna sotto. ≥ `soglia_scissione` = 1 con meno di 10 attivi: nasce la frangia sulla percepita dell'origine (decisione 2), con vocazione e regola dell'origine, leader nuovo in luna di miele; prende `quota_scissione` = 12% (`vita-dei-partiti.md:117`, arrotondato) dei seggi di Camera e Senato, della cassa, e del voto precedente di ogni gruppo, e copia il giudizio dell'origine. L'origine riparte con la percepita sulla dichiarata. Con 10 attivi resta la minaccia. Il giocatore resta leader del suo partito, allo stesso indice.
8. **Frangia di governo** (decisione 2). Entra nella coalizione, senza dicasteri, con l'intesa dell'origine. In chiusura, nel punto e nella finestra delle uscite degli alleati (fase ordinaria, senza dimissioni, fuori dal turno del voto, `partita.rs:371-374`), e fuori dalla crisi, la frangia della legislatura in corso (`Legislatura.frange`) si aggiunge a chi esce se `Quadro::valore` sul sondaggio pubblico supera `valore_al_governo` coi seggi veri di `margine_voto_anticipato` (lo stesso confronto del voto anticipato, `partita/ia.rs:458-476`); l'uscita passa da `uscite` (`partita.rs:2481-2506`): rimpasto con la maggioranza, crisi senza. La regola vale solo per le frange: gli alleati IA di sempre restano quelli dell'8a.
9. **Scioglimento.** Dopo ogni voto un partito IA attivo senza seggi in entrambe le Camere incrementa `senza_seggi`, altrimenti lo azzera; a 2 si scioglie (riga «Si scioglie dopo due elezioni senza seggi») ed esce dalle coalizioni dell'ultimo voto. Il giocatore no: senza seggi la partita finisce già.
10. **Cassa** (euro in `i64`, mai sotto zero). Entrate a ogni chiusura dopo il motore, per ogni partito attivo: 2 per mille = `due_per_mille_turno` × quota fra i votanti all'ultimo voto (abbinata per id; zero per chi non c'era), donazioni = `donazioni_turno` × quota vera fra i votanti; il totale per `quota_libera_entrate` = 0,1, la parte che resta dopo il funzionamento (`finanziamento-dei-partiti.md:90-92`, `:95`: Forza Italia spende in campagna circa un decimo dei ricavi di una legislatura). Cassa iniziale dei partiti dello scenario: `turni_cassa_iniziale` = 20 turni d'entrate calcolate al turno zero con l'intenzione vera iniziale al posto dell'ultimo voto. Il partito fondato parte con `dote_partito_nuovo`. Le regole della spec restano (2 per mille sui voti, donazioni sull'intenzione vera): le donazioni sono anche il segnale dell'intenzione vera per chi legge le casse. Il coefficiente per partito del 2 per mille (`finanziamento-dei-partiti.md:84`) va nel Rimandato.
11. **Coalizioni IA pre-voto.** Nell'apertura della campagna, dopo movimenti e protesta. Si parte dalle coalizioni dell'ultimo voto senza i membri di quella del giocatore (che decide il giocatore con la sua dichiarazione) e senza gli sciolti. Fino a 10 giri; in ogni giro ogni partito IA libero, in ordine di quota nel sondaggio, valuta: restare, correre da solo, entrare in una coalizione adiacente nella fila per dichiarata (nessun partito attivo fra lui e il membro più vicino). Valuta con `Quadro::valore` su un quadro le cui `ultime` sono le coalizioni candidate, con la dichiarazione corrente del giocatore. Si muove se ci guadagna più di `margine_coalizione`, e per entrare servono tutti i membri che non ci perdono. Il risultato va in `Legislatura.coalizioni_ia`, letta da `coalizioni_correnti` e dal quadro della disponibilità nel turno del voto, azzerata al voto; `legislatura.coalizioni` resta il patto dell'ultimo voto, per le rotture d'impegno. Una riga per ogni cambio.
12. **Governo avversario col valore atteso** (scelta 6 dell'8a). Quando dopo il voto nessuna coalizione ha l'incarico, il formatore (il partito IA attivo con più seggi) sceglie fra le maggioranze contigue che lo comprendono, nella fila dei partiti con seggi senza il giocatore, quella che gli vale di più (`valore_al_governo` con la guida), purché ogni membro ci guadagni (`valore_al_governo > 0`); a parità la meno numerosa in seggi, poi gli indici. Nessuna: Camere sciolte, come oggi. Il Capo dello Stato resta la contigua a regole.
13. **Bersagli e spinte.** Un bersaglio è una condizione (6) o una regione (20); fino a 3 per partito, distinti, una leva ciascuno. Forza = `forza_<leva>` × √(euro / `spesa_riferimento`), con `spesa_riferimento` = 3.000.000 € (`finanziamento-dei-partiti.md:92`). Persuadere: valence in più nei gruppi del bersaglio. Mobilitare: utilità dell'astensione più bassa, nei gruppi del bersaglio, ai punti entro 1 dalla dichiarata del partito. Valgono solo per le intenzioni del turno del voto, cioè finché `campagna` non si svuota col voto. Convincere: nella chiusura, prima dell'ancora, la quota `forza_convincere × √(…)` dell'istogramma di ogni gruppo del bersaglio passa al punto più vicino alla dichiarata; l'ancora la riassorbe. Le tre forze si tarano col test di scala del Task 8; valori e misura vanno nella PR.
14. **Spesa.** Si spende nella chiusura del turno del voto, prima del motore. Il giocatore spende quanto ha assegnato e il resto resta in cassa; i partiti IA spendono tutto. Una riga per partito nel registro (`Partita.mosse`), giocatore compreso: «Spende N euro: persuade …, mobilita …».
15. **Anteprima della campagna.** Costo esatto; per ogni bersaglio l'intensità `Poco`/`Medio`/`Molto` secondo √(euro / `spesa_riferimento`) sotto 1/3, sotto 2/3 o oltre, con la causa in una riga. Non legge lo stato vero.
16. **Campagna IA.** Candidati: ogni bersaglio con la leva che rende di più. Solo gli identitari convincono. Rendimento per euro letto sull'incrocio del sondaggio pubblico (quota `q` del partito fra i votanti, astensione `a`, peso `w` = elettori del bersaglio): persuadere `w·q·(1−q)·forza_persuadere`, mobilitare `w·a·q·forza_mobilitare`, convincere `w·q·(1−q)·forza_convincere`. Si scelgono i 3 bersagli migliori; la cassa si divide in proporzione al quadrato dei rendimenti, l'ottimo per rendimenti a radice, col resto dell'arrotondamento al primo. `ponytail:` è una stima al primo ordine del logit; la via d'uscita è simulare il voto sul sondaggio.
17. **Promesse.** Fino a 2 per campagna, disposizione e gradino diverso dal vigente; un solo comando per campagna. Persuasione subito: `persuasione_promessa` come spinta di valence verso il partito nei gruppi la cui posizione media è più vicina al gradino promesso che al vigente (`DatoDisposizione::posizione`, `disposizioni/mod.rs:129`); nessun effetto per le disposizioni senza posizione. Intesa: col giocatore al governo, ogni alleato la cui linea sulla promessa sarebbe no (`StatoAula::linea` su `guadagni`, `parlamento/voto.rs:35-73`) perde `intesa_promessa × peso_politiche`, con la causa nuova `CausaIntesa::Promessa`. Al voto la promessa diventa in sospeso; al voto dopo, prima delle intenzioni, è mantenuta se la disposizione sta al gradino promesso, chiunque l'abbia portata lì: `+valence_promessa_mantenuta` o `−valence_promessa_mancata` (il doppio, `effetti-della-campagna-e-promesse.md:61`), solo se il partito ha presieduto il governo per almeno 4 turni in quella legislatura (`Partito.turni_da_presidente`). Promesse IA: le prime 2 voci del programma, quello del governo se il partito lo presiede, altrimenti `legislatura::programma` sulla coalizione `[j]`; nessun effetto sull'intesa.
18. **Turno zero.** Niente movimenti, protesta né coalizioni IA (ticket 26, riga 30): `apertura_campagna` non gira. Il giocatore spende la dote, può promettere e può riposizionarsi; i partiti IA spendono la cassa iniziale.

## Global Constraints

Valgono tutti i vincoli del piano 8a (`2026-10-28-v1-piano-8a-governo-e-opposizione-ia.md:56-70`) e, attraverso quelli, dei piani 1-7b: italiano coi termini di `CONTEXT.md` (partito di protesta, scissione, cassa del partito, campagna elettorale, bersaglio, promessa, regola di movimento, posizione percepita; «posizione» sull'asse, mai «posto», che resta per il calendario); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano il colpevole; niente `serde(default)`. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Flussi casuali: `Flusso::PartitiIa` per nomi di partiti e leader nuovi, `Flusso::Sondaggi` per bias ed effetti d'istituto del partito nuovo. Nessun flusso nuovo.
- Le regole IA leggono quello che legge il giocatore: sondaggio pubblico, seggi veri, casse (pubbliche), intesa dei propri alleati. Gli eventi del mondo (protesta, scissione, scioglimento, entrate) leggono lo stato vero.
- Le mosse del giocatore passano da comandi nuovi in `comando.rs`: `Riposiziona`, `Campagna`, `Prometti`. Un comando rifiutato non cambia nulla.
- Ogni mossa IA e ogni evento della vita dei partiti lascia una riga in `Partita.mosse`; i testi vanno elencati nella PR per la revisione di Mario.
- Ogni manopola nuova ha `valore`, `min`, `max`, `ticket` ed entra in `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:65`) nel task che la introduce, con la fonte o con `# assunzione del piano 8b, nessuna fonte in docs/ricerca`. Le cifre in euro sono manopole `f64`, arrotondate a `i64` all'uso.
- La `Vista` non porta stato vero nuovo: casse, regole, promesse e spese sono pubbliche; delle mosse si scrive il motivo, mai i valori attesi in cifre.
- `VERSIONE_SCHEMA` passa a `10` nel Task 1.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa. Le schermate (campagna, casse, promesse) sono del piano 11.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Partita di prova (`Partita::nuova`): sette partiti (0 estrema sinistra … 6 estrema destra), il giocatore è la destra (5) al governo col centrodestra (4); scadenza 19, quindi la campagna è il turno 19; turno 0 = ottobre-dicembre 2026. Partita vera: il giocatore è l'indice 7 finché non nasce un partito.

## Review Focus

1. **Partito nato o sciolto, poi salvataggio**: `carica` ridà la stessa partita, e sondaggio, bias, effetti d'istituto, seggi, intesa e voto precedente hanno tutti n+1 o n voci. Test `aggiungi_partito_e_salvataggio` nel Task 1 e `la_scissione_sopravvive_al_salvataggio` nel Task 6.
2. **Tetto dei dieci**: con dieci attivi non nasce la protesta e la scissione resta una minaccia, senza panic. Test `al_tetto_nessuna_nascita` nel Task 5 e `al_tetto_la_scissione_resta_minaccia` nel Task 6.
3. **Frangia fuori finestra**: in crisi, con dimissioni in corso o nel turno del voto la frangia non esce, e un'uscita non apre due crisi. Test `la_frangia_non_esce_fuori_finestra` nel Task 6.
4. **Comandi di campagna rifiutati** (fuori dal turno del voto, più di 3 bersagli, bersaglio ripetuto, spesa oltre la cassa, spesa non positiva, terza promessa, gradino già in vigore): la partita resta identica (`salva` prima e dopo). Test `campagna_rifiutata_non_cambia_nulla` nel Task 8 e `promessa_rifiutata_non_cambia_nulla` nel Task 10.
5. **Sciolto dentro uno stato che lo cita** (coalizione dell'ultimo voto, promesse in sospeso, intesa): il voto dopo non lo conta, le coalizioni restano valide per il riparto, nessun indice orfano. Test `lo_sciolto_esce_dalle_coalizioni` nel Task 3.

## Dispatch

PR A: 0 → 1 → 2 → 3 in sequenza. Lo 0 tocca `partita/ia.rs` e i test; 1, 2 e 3 toccano tutti `partiti.rs` e `partita.rs`.

PR B: dopo la review della A, sul branch impilato, 4 → 5 → 6 → 7 in sequenza. Il 4 crea `apertura_campagna`, che 5 e 7 estendono; il 6 usa `aggiungi_partito` e la percepita del 4; il 7 legge le posizioni mosse e i partiti nati.

PR C: dopo la review della B, 8 → 9 → 10 in sequenza: il 9 sceglie le spese col tipo dell'8, il 10 aggiunge le spinte delle promesse al costruttore dell'8.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task. A fine PR, review a sei in parallelo: `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer` (legge i testi delle mosse e i messaggi d'errore elencati nella PR), `rust-reviewer` (il diff tocca `.rs`) e `spec-reviewer` (questo piano), con i findings di quest'ultimo in un report a parte. Poi un solo giro di correzioni. Le decisioni prese in esecuzione si elencano nella PR. Il merge lo decide Mario.

---

## PR A · Vita (Task 0-3)

### Task 0: Rilievi aperti dell'8a

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs:378-438` (`crisi_ia`)
- Modify: `crates/sim-core/tests/governo_ia.rs:663-685` (`SI_DIMETTE`, `in_crisi_il_governo_ia_non_riprende_chi_e_uscito`), `crates/sim-core/tests/legislatura.rs:174-193` (`ogni_caduta_ha_la_sua_causa`)

**Ricerca:**
- nessun riferimento pertinente

**Dispatch:** `rust-engineer` · primo della PR A · skill-gate: true.

**Interfaces:**
- Consumes: nulla di nuovo.
- Produces: in `crisi_ia`, terza riga di dimissioni. Se nessun vicino passa il filtro ma almeno uno darebbe maggioranza e guadagno ed è scartato solo per l'intesa sotto `soglia_ultimatum`: «Si dimette: il solo partito contiguo che gli ridà la maggioranza ha rotto con lui». Le altre due righe restano.

- [ ] **Step 1: Scrivi i test che falliscono**

In `tests/governo_ia.rs`, la costante nuova `SI_DIMETTE_ROTTURA` col testo sopra. `in_crisi_il_governo_ia_non_riprende_chi_e_uscito` deve trovare `SI_DIMETTE_ROTTURA` e non `SI_DIMETTE`. Un test nuovo, `senza_vicini_il_motivo_resta_nessun_partito`: crisi IA con l'intesa del vicino alla base e la maggioranza impossibile anche con lui (`con_seggi`); deve trovare `SI_DIMETTE`.

In `tests/legislatura.rs`, `ogni_caduta_ha_la_sua_causa` diventa un controllo vero:

```rust
let mut casi = 0;
// … stesso ciclo, sui semi 1..=20, fino al turno 41 …
// cause ammesse: sfiducia approvata nel turno, crisi aperta nel turno, dimissioni dell'IA
// (una mossa che comincia con "Si dimette") nel turno, esercizio provvisorio.
casi += 1;
assert!(sfiducia || crisi || dimissioni || provvisorio, "seme {seme}, turno {}: nessuna causa", turno + 1);
// … fine ciclo …
assert!(casi > 0, "nessuna caduta in 20 semi: il test non controlla nulla");
```

Se nessun seme fra 1 e 20 produce una caduta, si costruisce lo stato con `edita` (un alleato sotto `soglia_uscita`) e il commento dice perché.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test governo_ia --test legislatura`
Expected: FAIL su `in_crisi_il_governo_ia_non_riprende_chi_e_uscito` (riga vecchia).

- [ ] **Step 3: Implementa la terza riga in `crisi_ia`**

Il filtro si divide in due: `utili` (maggioranza e guadagno) e, fra questi, `scelti` (intesa non sotto soglia). Se `scelti` è vuoto e `utili` no, la riga nuova.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/plans/2026-10-29-v1-piano-8b-vita-dei-partiti-e-campagna.md docs/ricerca crates
git commit -m "fix(core): motivo vero delle dimissioni in crisi, cadute controllate davvero"
```

---

### Task 1: Partiti che entrano ed escono, schema 10

**Files:**
- Modify: `crates/sim-core/src/partiti.rs` (nuovo `enum Regola`; `DatoPartito`, `Partito`, `iniziali`, `valida`; `pub const MASSIMO_PARTITI: usize = 10`)
- Modify: `dati/base/scenario.toml:220-309` (`regola` nei sette partiti; lista `nomi_partiti`), `crates/sim-core/src/scenario.rs` (`nomi_partiti`)
- Modify: `crates/sim-core/src/partita.rs:39` (`VERSIONE_SCHEMA = 10`), `:132-242` (`costruisci`), `:3284-3477` (`controlla_partiti`); nuovo `aggiungi_partito`
- Modify: `crates/sim-core/src/elettorato/voto.rs:53-80` (`utilita`: sciolto a −∞), `crates/sim-core/src/elettorato/mod.rs` (`Elettorato::aggiungi_partito`), `crates/sim-core/src/informazione/mod.rs` (`Informazione::aggiungi_partito`), `crates/sim-core/src/parlamento/mod.rs`, `crates/sim-core/src/parlamento/intesa.rs`
- Modify: `crates/sim-core/src/partita/ia.rs:59-68` (`calo` abbinato per id, via il `ponytail:`), `crates/sim-core/src/legislatura/fondazione.rs:56-68` (`partito_nuovo`), `crates/sim-core/src/vista.rs:369` (`VocePartito`)
- Test: test unitari in `partita.rs`, `partiti.rs`, `elettorato/voto.rs`; `crates/sim-core/tests/partiti_ia.rs`

**Ricerca:** `docs/ricerca/modelli/abm-elettorali.md:14-18` (Sticker, Aggregator, Hunter, Governator e Satisficing Governator); `docs/ricerca/modelli/vita-dei-partiti.md:121` (tetto di 10: liste con seggi 9-14, NEV 3,5-6,4).

**Dispatch:** `rust-engineer` · dopo il Task 0 · skill-gate: true.

**Interfaces:**
- Consumes: nulla di nuovo.
- Produces:
  - `pub enum Regola { Sticker, Aggregator, Hunter, Governator }` in `partiti.rs`, `Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize`, `serde(rename_all = "snake_case")`, esportato in TS.
  - `DatoPartito { …, pub regola: Regola }`; `Partito { …, pub regola: Regola, pub sciolto: bool }`. Scenario: sticker per estrema sinistra ed estrema destra, aggregator per sinistra e destra, hunter per centrosinistra e centrodestra, governator per il centro (spec riga 350). Il partito fondato nasce `Sticker`.
  - `Scenario.nomi_partiti: Vec<String>` (almeno 8; `valida` rifiuta la lista vuota e un nome uguale a quello di un partito dello scenario: «nome di partito nuovo già usato: {nome}»).
  - `pub(crate) fn aggiungi_partito(&mut self, p: Partito) -> usize` su `Partita`: in coda; giudizio 0 e voto precedente 0 (inserito prima dell'astensione) in ogni gruppo; bias e un effetto per istituto estratti da `Flusso::Sondaggi`, quota 0 in ogni rilevazione e incrocio, entrambi prima dell'astensione; seggi 0 in entrambe le Camere; intesa `intesa_base` senza ultimatum. Restituisce l'indice.
  - `pub(crate) fn attivi(&self) -> usize` su `Partita`: i partiti non sciolti.
  - `fn nome_nuovo(&mut self, gia: &[String], lista: &[String]) -> Option<String>`, estratto da `cambia_leader`: lo usano leader e partiti nuovi.
  - `utilita`: −∞ per un partito sciolto.
  - `calo(j)` abbina i voti delle due elezioni per id.
  - `controlla_partiti`: le prime voci sono i partiti dello scenario nell'ordine, poi al più il giocatore fondato (`ID_GIOCATORE`), poi solo id `nuovo_<indice>`; id unici; `giocatore < n`; al più `MASSIMO_PARTITI` attivi; il giocatore mai sciolto. Errori col colpevole: «partito in più non valido: {id}», «più di 10 partiti attivi», «il partito del giocatore non può essere sciolto».
  - `VocePartito { …, pub regola: Regola, pub sciolto: bool }`.
  - `VERSIONE_SCHEMA = 10`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// partita.rs, mod tests
#[test]
fn aggiungi_partito_e_salvataggio() {
    let mut p = Partita::nuova(dati_base(), 1);
    let mut nuovo = p.partiti[2].clone();
    nuovo.id = "nuovo_7".into();
    let j = p.aggiungi_partito(nuovo);
    assert_eq!(j, 7);
    assert_eq!(p.giocatore, 5);
    assert!(p.elettorato.gruppi.iter().all(|g| g.voto_precedente.len() == 9));
    assert_eq!(p.informazione.bias.len(), 9);
    assert_eq!(Partita::carica(&p.salva().unwrap()).unwrap(), p);
    p.chiudi_turno();
    assert!(p.informazione.controlla(&p.dati, 8).is_ok()); // n+1 voci in ogni rilevazione
    assert!(p.intenzioni_vere().nazionali[7] > 0.0);
}

#[test]
fn sciolto_non_prende_voti() {
    let mut p = Partita::nuova(dati_base(), 1);
    p.partiti[0].sciolto = true;
    let v = p.intenzioni_vere();
    assert_eq!(v.nazionali[0], 0.0);
    assert!((v.nazionali.iter().sum::<f64>() - 1.0).abs() < 1e-9);
}

#[test]
fn calo_abbina_per_id() { /* due esiti con un partito in più nel secondo: il calo del centro è quello del centro */ }
```

In `tests/partiti_ia.rs`: `ogni_partito_ha_la_sua_regola` (le quattro regole sui sette partiti, `Sticker` per il fondato) e `carica_rifiuta_un_partito_in_piu_non_valido` (via `edita`, un partito in coda con id `intruso`).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core aggiungi_partito sciolto_non calo_abbina && cargo test -p sim-core --test partiti_ia`
Expected: FAIL, `aggiungi_partito`, `sciolto` e `regola` assenti.

- [ ] **Step 3: Implementa tipi, dati, `aggiungi_partito`, controlli e schema**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): partiti che entrano ed escono, regola di movimento, schema 10"
```

---

### Task 2: La cassa del partito

**Files:**
- Create: `crates/sim-core/src/legislatura/cassa.rs` (`mod cassa;` in `legislatura/mod.rs`)
- Modify: `crates/sim-core/src/partiti.rs` (`Partito.cassa`), `crates/sim-core/src/partita.rs` (`costruisci`, `chiudi_turno` dopo il motore), `crates/sim-core/src/legislatura/fondazione.rs` (dote), `crates/sim-core/src/vista.rs:369` (`VocePartito.cassa`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/cassa.rs` (nuovo), test unitari in `cassa.rs`

**Ricerca:** `docs/ricerca/modelli/finanziamento-dei-partiti.md:82` (montante del 2 per mille, 32.584.199 € l'anno), `:89` (donazioni detratte, circa 22 milioni l'anno), `:90-92` (ricavi e spesa di campagna), `:95` (una legislatura di entrate vale dieci volte la campagna: la quota libera), `:91` (dote 0,5-1,5 milioni).

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `Partita::attivi`, `Partito.sciolto` (Task 1).
- Produces:
  - `Partito.cassa: i64`; `VocePartito.cassa: i64`.
  - `pub(crate) fn entrate(dati: &DatiGioco, ultimo_voto: &[f64], intenzione: &[f64]) -> Vec<i64>` in `legislatura/cassa.rs`: per partito `round((due_per_mille_turno · ultimo_voto[j] + donazioni_turno · intenzione[j]) · quota_libera_entrate)`; quote fra i votanti, una voce per partito; zero per gli sciolti (quota zero).
  - Manopole: `due_per_mille_turno` 8.146.050 (2.000.000-12.000.000, fonte `finanziamento-dei-partiti.md:82`, un quarto del montante annuo), `donazioni_turno` 5.500.000 (1.000.000-10.000.000, `:89`), `quota_libera_entrate` 0,1 (0,02-0,3, `:90-92`, `:95`), `dote_partito_nuovo` 1.000.000 (500.000-1.500.000, `:91`), `turni_cassa_iniziale` 20 (4-40, assunzione del piano 8b), tutte ticket `25`.
  - Scelta 10 del piano: entrate a ogni chiusura dopo il motore, cassa iniziale, dote del fondato.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/cassa.rs
#[test]
fn il_partito_fondato_parte_con_la_dote() {
    assert_eq!(fondata(4.5).vista().partiti.iter().find(|p| p.id == "giocatore").unwrap().cassa, 1_000_000);
}

#[test]
fn le_entrate_seguono_voti_e_intenzioni() {
    let mut p = Partita::nuova(dati_base(), 1);
    chiudi_col_bilancio(&mut p); // voto del turno zero: c'è un ultimo voto
    let prima: Vec<i64> = p.partiti().iter().map(|x| x.cassa).collect();
    chiudi_col_bilancio(&mut p);
    let delta: Vec<i64> = p.partiti().iter().zip(&prima).map(|(x, c)| x.cassa - c).collect();
    assert!(delta.iter().all(|&d| d >= 0));
    // il primo per quota all'ultimo voto incassa più dell'ultimo per quota
}

#[test]
fn le_casse_iniziali_dello_scenario_sono_venti_turni() { /* cassa ≈ 20 × entrate del turno zero, entro l'arrotondamento */ }
```

Nel modulo di test di `cassa.rs`: `entrate_a_quota_zero_sono_zero` e `entrate_esatte` (un partito al 50% dei voti e al 50% delle intenzioni: `round((8_146_050 · 0,5 + 5_500_000 · 0,5) · 0,1)` = 682.303).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test cassa`
Expected: FAIL, `cassa` assente.

- [ ] **Step 3: Implementa `entrate`, cassa iniziale, dote ed entrate in chiusura**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): cassa del partito, 2 per mille, donazioni e dote"
```

---

### Task 3: Scioglimento e tetto

**Files:**
- Create: `crates/sim-core/src/partita/vita.rs` (`mod vita;` in `partita.rs`)
- Modify: `crates/sim-core/src/partiti.rs` (`Partito.senza_seggi`), `crates/sim-core/src/partita.rs:1926-2005` (`vota_parlamento`), `:2342` (`forma_governo`, nessuno sciolto nella fila)
- Test: `crates/sim-core/tests/vita_dei_partiti.rs` (nuovo)

**Ricerca:** `docs/ricerca/modelli/vita-dei-partiti.md:120` (due elezioni senza seggi: i «flop» spariscono dopo la seconda o terza elezione), `:73` (scioglimento e morte dei partiti).

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true.

**Architect:** `vota_parlamento` fissa `legislatura.incarico` dalle coalizioni del voto (`partita.rs:1988`) prima della coda: `scioglimenti` toglie lo sciolto anche da `Incarico::Ia { coalizione }`, che `forma_governo` prende senza filtri (`:2356`); e `leader_dopo_il_voto` (`partita/ia.rs:73`), che gira dopo, salta gli sciolti.

**Interfaces:**
- Consumes: `Partito.sciolto`, `Partita::attivi`, `MASSIMO_PARTITI` (Task 1).
- Produces:
  - `Partito.senza_seggi: u8`.
  - `pub(super) fn scioglimenti(&mut self)` in `partita/vita.rs`, chiamata in coda a `vota_parlamento` (scelta 9): mossa «Si scioglie dopo due elezioni senza seggi»; lo sciolto esce da `legislatura.coalizioni`, e le coalizioni rimaste di un solo partito cadono.
  - `legislatura::maggioranza_contigua` e la fila di `crisi_ia` saltano gli sciolti (hanno zero seggi: basta la condizione sui seggi, verificata dal test).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/vita_dei_partiti.rs
#[test]
fn due_elezioni_senza_seggi_sciolgono() {
    // estrema sinistra a valence −10 (sostituisci_valence): zero voti, zero seggi.
    // Dopo il voto del turno zero senza_seggi = 1, dopo quello del turno 20 sciolto,
    // con la mossa; il voto dopo le dà quota zero.
}

#[test]
fn una_elezione_con_seggi_azzera_il_conto() { /* senza_seggi = 1 via edita, poi un voto con seggi: 0 */ }

#[test]
fn lo_sciolto_esce_dalle_coalizioni() {
    // edita: lo sciolto in una coalizione a due dell'ultimo voto e con intesa;
    // dopo lo scioglimento la coalizione cade, il riparto del voto dopo non fallisce,
    // carica accetta il salvataggio.
}

#[test]
fn il_giocatore_non_si_scioglie() { /* senza seggi la partita finisce con Fine::SenzaSeggi, il partito resta attivo */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test vita_dei_partiti`
Expected: FAIL.

- [ ] **Step 3: Implementa `senza_seggi` e `scioglimenti`**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): scioglimento dopo due elezioni senza seggi"
```

---

## PR B · Movimento (Task 4-7)

### Task 4: Apertura della campagna, movimento e riposizionamento

**Files:**
- Create: `crates/sim-core/src/legislatura/movimento.rs`
- Modify: `crates/sim-core/src/partiti.rs` (`Partito.ultimo_passo`), `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.riposizionato`), `crates/sim-core/src/partita.rs` (`chiudi_turno`: `apertura_campagna` prima di `apertura_ia`; nuovo `riposiziona`), `crates/sim-core/src/partita/ia.rs` (`apertura_campagna`, `movimenti_ia`), `crates/sim-core/src/comando.rs` (`Comando::Riposiziona`)
- Test: test unitari in `movimento.rs`; `crates/sim-core/tests/campagna.rs` (nuovo)

**Ricerca:** `docs/ricerca/modelli/abm-elettorali.md:14-18` (le quattro regole); `docs/ricerca/modelli/vita-dei-partiti.md:101` (regola di movimento: Adams 2004 non trova la reazione al voto su cui poggia Hunter), `:122` (passo di 0,5 a legislatura: CHES, 90° percentile 0,98 punti).

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true.

**Interfaces:**
- Consumes: `Regola`, `Partito.sciolto` (Task 1).
- Produces:
  - `pub(crate) struct Contesto { pub centro: f64, pub baricentro: Option<f64>, pub guadagno: Option<bool>, pub ultimo_passo: f64, pub al_governo: bool, pub posizione_governo: f64 }` e `pub(crate) fn passo(r: Regola, x: f64, c: &Contesto, massimo: f64) -> f64` in `legislatura/movimento.rs` (scelta 4); `pub(crate) fn limita(x: f64, passo: f64, altri: &[f64]) -> f64`: la posizione nuova, mai oltre il primo attivo nella direzione del passo, in 1-7.
  - `Partito.ultimo_passo: f64`; `Legislatura.riposizionato: bool`, azzerato al voto.
  - `pub(super) fn apertura_campagna(&mut self)` su `Partita`, chiamata in `chiudi_turno` dopo il ciclo delle decadenze e prima di `apri_emendamenti` (`partita.rs:396-409`), se il turno nuovo è di voto e la partita non è finita (mai al turno zero): per ora `movimenti_ia`, con una riga per partito mosso: «Sposta la posizione da 3,0 a 3,5: verso i suoi elettori» (Aggregator), «…: dopo un guadagno al voto ripete lo spostamento» / «…: dopo una perdita al voto inverte lo spostamento» / «…: alla prima legislatura va verso il centro» (Hunter), «…: verso la posizione del governo» (Governator).
  - `Comando::Riposiziona { posizione: f64 }` → `Partita::riposiziona(posizione: f64) -> Result<(), Errore>`: dichiarata nuova e percepita a metà strada (scelta 5). Errori: «Il riposizionamento si decide solo nel turno della campagna», «Già riposizionato in questa campagna», «Al massimo mezzo punto dalla posizione dichiarata», «Non si scavalca {nome}», «Posizione fuori da 1-7».
  - Manopola `passo_movimento` 0,5 (0,25-1, `vita-dei-partiti.md:122`, ticket `18`).

- [ ] **Step 1: Scrivi i test che falliscono**

Nel modulo di test di `movimento.rs`, un test per regola: Sticker fermo; Aggregator a 2,0 col baricentro 2,3 va a 2,3, col baricentro 4 va a 2,5; Hunter con `guadagno = Some(true)` e `ultimo_passo = −0,5` va di −0,5, con `Some(false)` di +0,5, con `None` verso 4; Governator al governo fermo, fuori a 5,0 col governo a 4,8 va a 4,8. `limita`: da 3,0 con +0,5 e un vicino a 3,2 si ferma a 3,2.

```rust
// tests/campagna.rs
#[test]
fn i_partiti_ia_si_muovono_nel_turno_prima_del_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    al_turno(&mut p, 18);
    let prima = p.partiti().to_vec();
    chiudi_col_bilancio(&mut p); // il turno 19 è di voto: apertura della campagna
    for (a, b) in prima.iter().zip(p.partiti()) {
        let d = b.posizione_dichiarata - a.posizione_dichiarata;
        assert!(d.abs() <= 0.5 + 1e-12);
        assert!((b.posizione_percepita - a.posizione_percepita - d / 2.0).abs() < 0.05); // scivolamento dei voti a parte
        if a.regola == Regola::Sticker { assert_eq!(d, 0.0); }
    }
}

#[test]
fn niente_movimenti_al_turno_zero() { /* turno 0: dichiarate dello scenario, nessuna riga */ }

#[test]
fn riposizionamento_del_giocatore() {
    // al turno 19: +0,5 ok, percepita +0,25; un secondo comando rifiutato; +0,6 rifiutato;
    // oltre il vicino rifiutato col nome; al turno 18 rifiutato; al turno 0 della partita fondata ammesso. Ogni rifiuto lascia salva() identico.
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core movimento && cargo test -p sim-core --test campagna`
Expected: FAIL.

- [ ] **Step 3: Implementa `movimento.rs`, `apertura_campagna`, `movimenti_ia`, `riposiziona`**

Il baricentro dell'Aggregator: media dei punti dell'asse pesata con la quota del partito negli incroci regionali del sondaggio pubblico e con gli elettori di ogni regione, come in `elettorato/referendum.rs:127-150`. `guadagno` di Hunter: quota fra i votanti dell'ultimo voto contro quella del voto prima, per id; `None` se il partito non c'era in entrambi.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): regola di movimento IA e riposizionamento del giocatore"
```

---

### Task 5: Il partito di protesta

**Files:**
- Modify: `crates/sim-core/src/partita/vita.rs` (`protesta`), `crates/sim-core/src/partita/ia.rs` (`apertura_campagna` la chiama dopo i movimenti), `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.protesta`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/vita_dei_partiti.rs`, test unitari in `vita.rs`

**Ricerca:** `docs/ricerca/modelli/vita-dei-partiti.md:111` (10% nel tratto scoperto, Laver-Schilperoord), `:112` (1 punto dal più vicino), `:113` (astensione sopra la media, Tavits), `:114` (al più uno a legislatura), `:116` (luna di miele del partito nuovo).

**Dispatch:** `rust-engineer` · dopo il Task 4 · skill-gate: true.

**Interfaces:**
- Consumes: `aggiungi_partito`, `nome_nuovo`, `attivi` (Task 1); `dote_partito_nuovo` (Task 2); `apertura_campagna` (Task 4).
- Produces:
  - `pub(crate) fn punto_di_protesta(quote: &[f64; 7], astensione: &[f64; 7], media: f64, dichiarate: &[f64], soglia: f64, distanza: f64) -> Option<usize>` in `partita/vita.rs`, pura (scelta 6).
  - `Legislatura.protesta: bool`, azzerata al voto.
  - `fn protesta(&mut self)`: riga «Nasce {nome}, partito di protesta in posizione {k}: elettori lontani dai partiti e astensione alta».
  - Manopole: `soglia_protesta` 0,1 (0,075-0,15, `vita-dei-partiti.md:111`), `distanza_protesta` 1 (0,5-2, `:112`), ticket `18`.

- [ ] **Step 1: Scrivi i test che falliscono**

Nel modulo di test di `vita.rs`: `punto_di_protesta` sceglie il punto scoperto con la quota più alta; scarta un punto con astensione sotto la media, uno a 0,5 dal partito più vicino, uno sotto soglia; `None` senza candidati.

```rust
// tests/vita_dei_partiti.rs
#[test]
fn nasce_la_protesta_dove_l_asse_e_scoperto() {
    // scenario con i partiti di destra spostati verso il centro (sostituisci su scenario.toml)
    // e soglia_protesta bassa: al turno 19 nasce un partito, id "nuovo_7", posizione ≥ 5,
    // Hunter, identitario, cassa = dote, zero seggi, la riga nella vista. Al voto prende voti.
}

#[test]
fn al_tetto_nessuna_nascita() { /* tre partiti aggiunti con aggiungi_partito (10 attivi): nessuna protesta */ }

#[test]
fn una_sola_protesta_a_legislatura() { /* protesta = true via edita: nessuna nascita */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test vita_dei_partiti && cargo test -p sim-core punto_di_protesta`
Expected: FAIL.

- [ ] **Step 3: Implementa `punto_di_protesta` e `protesta`**

Quota del punto `k`: Σ_i elettori_i · h_ik / Σ elettori. Astensione del punto: Σ_i elettori_i · h_ik · p_astensione(i, k) / Σ_i elettori_i · h_ik, con `p_astensione` dal softmax dell'utilità al punto `k` senza fedeltà; media = astensione nazionale delle intenzioni vere.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): partito di protesta nel turno della campagna"
```

---

### Task 6: La scissione

**Files:**
- Modify: `crates/sim-core/src/partita/vita.rs` (`scissioni`, `frange_in_uscita`), `crates/sim-core/src/partiti.rs` (`Partito.minaccia`), `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.frange`), `crates/sim-core/src/partita.rs` (`chiudi_turno`: `scissioni` dopo l'aula, `frange_in_uscita` unite a `escono` prima di `uscite`, `:371-374`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/vita_dei_partiti.rs`

**Ricerca:** `docs/ricerca/modelli/vita-dei-partiti.md:46-55` (scissioni alla Camera 2010-2026: quote, origine al governo), `:61-67` (nessuna uscita immediata dalla maggioranza), `:117` (quota 12%), `:118` (cassa in proporzione ai seggi), `:119` (soglia 1 e minaccia 0,5: astrazione di gioco).

**Dispatch:** `rust-engineer` · dopo il Task 5 · skill-gate: true.

**Architect:** `apertura_ia` chiama il governo IA solo se il Presidente del Consiglio non è il giocatore (`partita/ia.rs:90-95`), ma la frangia del test esce da un governo del giocatore: `frange_ia` sta fuori da quel ramo. `uscite` è scritta per la chiusura, prima che il turno avanzi (`partita.rs:373`): chiamata in apertura apre la crisi un turno più tardi (`crisi = turno + 1` sul turno già avanzato, `:2502`) e dimette ministri dopo `ministri.apri` (`:423`). Due vie da scegliere in review: `frange_ia` segna l'uscita in apertura e la chiusura la passa a `uscite` insieme a `escono`, oppure il test fissa il turno di crisi voluto. **Risolta:** la frangia decide in chiusura, accanto a `uscite` (scelta 8, `frange_in_uscita`).

**Interfaces:**
- Consumes: `aggiungi_partito`, `nome_nuovo`, `attivi` (Task 1); `Partito.cassa` (Task 2); `uscite` (`partita.rs:2481`), `Quadro::valore` (`legislatura/coalizione.rs:287`), `valore_al_governo` (`legislatura/crisi.rs:35`), col confronto di `voto_anticipato` (`partita/ia.rs:459-476`).
- Produces:
  - `Partito.minaccia: bool`; `Legislatura.frange: Vec<usize>`, azzerata al voto.
  - `pub(super) fn scissioni(&mut self)` (scelta 7). Righe: «Minaccia di scissione: la posizione percepita dista {d} dalla dichiarata»; «Scissione: nasce {nome} con {c} deputati e {s} senatori, sulla posizione {x}». Sul partito d'origine.
  - `pub(super) fn frange_in_uscita(&mut self) -> Vec<usize>` in `partita/vita.rs` (scelta 8), con qualunque Presidente del Consiglio; le frange restituite si uniscono a `escono` e passano da `uscite`. Riga «Esce dalla maggioranza: nei sondaggi il voto gli vale più che restare al governo».
  - Manopole: `soglia_scissione` 1 (0,5-2), `soglia_minaccia` 0,5 (0,25-1), `quota_scissione` 0,12 (0,05-0,3, `vita-dei-partiti.md:117`), ticket `18`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/vita_dei_partiti.rs
#[test]
fn a_un_punto_il_partito_si_scinde() {
    // edita: percepita del centrodestra (al governo col giocatore) = dichiarata + 1,0.
    // Dopo la chiusura: partito "nuovo_7" sulla vecchia percepita, 12% dei seggi
    // (somma dei seggi invariata), 12% della cassa (somma invariata), nella coalizione di governo;
    // percepita dell'origine = dichiarata; la riga nella vista.
}

#[test]
fn a_mezzo_punto_solo_la_minaccia() { /* distanza 0,6: una riga di minaccia, nessuna nascita; al turno dopo nessuna riga ripetuta */ }

#[test]
fn si_scinde_anche_il_partito_del_giocatore() { /* giocatore stesso indice, leader vuoto, la frangia è IA */ }

#[test]
fn al_tetto_la_scissione_resta_minaccia() { /* 10 attivi: nessuna nascita, la minaccia sì */ }

#[test]
fn la_frangia_esce_quando_le_conviene() {
    // margine_voto_anticipato molto negativo (sostituisci): alla prima chiusura dopo la scissione la frangia esce,
    // la coalizione la perde, rimpasto o crisi secondo la maggioranza.
}

#[test]
fn la_frangia_non_esce_fuori_finestra() { /* in crisi, con dimissioni, nel turno del voto: resta; mai due crisi */ }

#[test]
fn la_scissione_sopravvive_al_salvataggio() { /* carica(salva()) == p dopo la scissione */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test vita_dei_partiti`
Expected: FAIL.

- [ ] **Step 3: Implementa `scissioni` e `frange_in_uscita`**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): scissione con minaccia, frangia che decide col valore atteso"
```

---

### Task 7: Coalizioni IA e governo avversario col valore atteso

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs` (`coalizioni_ia`, chiamata da `apertura_campagna` dopo la protesta; `maggioranza_ia`), `crates/sim-core/src/legislatura/mod.rs` (`Legislatura.coalizioni_ia`), `crates/sim-core/src/partita.rs:1911-1918` (`coalizioni_correnti`), `:2007-2024` (`quadro`: `ultime` nel turno del voto), `:2342-2370` (`forma_governo`: `maggioranza_ia` al posto di `maggioranza_contigua`), `:3284` (`controlla_partiti`: indici di `coalizioni_ia`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/coalizione.rs`, `crates/sim-core/tests/governo_ia.rs`

**Ricerca:** `docs/ricerca/modelli/formazione-di-coalizioni.md:13` (minima connessa vincente), `:22` (il formateur prende la presidenza); `docs/ricerca/modelli/abm-elettorali.md:35` (α fra politiche e ufficio, già nei termini del valore atteso).

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true.

**Interfaces:**
- Consumes: `Quadro` e `Quadro::valore` (`legislatura/coalizione.rs:122-136`, `:287`), `coalizioni_al_voto` (`:74-94`), `valore_al_governo`, `dichiarazione_corrente` (`partita.rs:2166`).
- Produces:
  - `Legislatura.coalizioni_ia: Option<Vec<Vec<usize>>>`, azzerata al voto.
  - `fn coalizioni_ia(&mut self)` (scelta 11). Righe: «Entra in coalizione con {nomi}: nel valore atteso ci guadagna», «Corre da solo: nel valore atteso ci guadagna».
  - `coalizioni_correnti` nel turno del voto: `coalizioni_al_voto(coalizioni_ia.unwrap_or(ultime), …)`; `quadro()` nel turno del voto ha `ultime` = le stesse.
  - `fn maggioranza_ia(&self) -> Option<Vec<usize>>` (scelta 12), riga del formatore «Forma il governo con {nomi}: la maggioranza che gli vale di più».
  - Manopola `margine_coalizione` 0,01 (0-0,1, assunzione del piano 8b, ticket `18`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/coalizione.rs
#[test]
fn le_coalizioni_ia_sono_un_equilibrio() {
    // al turno 19: per ogni partito IA libero, nessuna mossa singola (solo, o in una coalizione
    // adiacente con l'assenso dei membri) gli fa guadagnare più di margine_coalizione.
}

#[test]
fn le_coalizioni_ia_non_toccano_il_patto_dell_ultimo_voto() { /* legislatura.coalizioni invariata fino al voto */ }

#[test]
fn senza_campagna_restano_le_coalizioni_dell_ultimo_voto() { /* turno 0: coalizioni dello scenario */ }

// tests/governo_ia.rs
#[test]
fn il_formatore_sceglie_la_maggioranza_che_gli_vale_di_piu() {
    // in_trattativa senza incarico, due maggioranze contigue col primo per seggi: governa con
    // quella del valore più alto, e ogni membro ci guadagna.
}
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test coalizione --test governo_ia`
Expected: FAIL.

- [ ] **Step 3: Implementa `coalizioni_ia` e `maggioranza_ia`**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS. Le partite di prova esistenti possono cambiare esito dopo il primo voto: i test che fissano coalizioni o governi dopo un voto vanno riletti, non allentati; ogni test cambiato va nella PR col perché.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): coalizioni IA e governo avversario col valore atteso"
```

---

## PR C · Campagna (Task 8-10)

### Task 8: Spinte di campagna, comando e anteprima

**Files:**
- Create: `crates/sim-core/src/legislatura/campagna.rs`
- Modify: `crates/sim-core/src/partiti.rs` (`Partito.campagna`), `crates/sim-core/src/elettorato/voto.rs:19-137` (`Intenzioni::calcola`, `utilita`, `transizione`, `intenzioni` con `Option<&Spinte>`), `:166-180` (convincere prima dell'ancora), `crates/sim-core/src/partita.rs` (`intenzioni_vere`, spesa in `chiudi_turno` prima del motore, `campagna`, `anteprima_campagna`), `crates/sim-core/src/comando.rs` (`Comando::Campagna`), `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/campagna.rs`, test unitari in `campagna.rs`

**Ricerca:** `docs/ricerca/modelli/effetti-della-campagna-e-promesse.md:51` (0,5-1,5 punti a cassa piena, decisione 1), `:52` (radice della spesa), `:53-55` (mobilitare, persuadere, ordine delle leve), `:56-57` (convincere riassorbito), `:58` (persuasione solo per quel voto); `docs/ricerca/modelli/finanziamento-dei-partiti.md:92` (spesa di riferimento 2-3,5 milioni); `docs/ricerca/sistemi/campagna-elettorale.md:28` (la campagna sposta voti al margine).

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true.

**Interfaces:**
- Consumes: `Partito.cassa` (Task 2), `apertura_campagna` (Task 4).
- Produces:
  - In `legislatura/campagna.rs`, tutti `Serialize, Deserialize`, esportati in TS: `enum Bersaglio { Condizione(Condizione), Regione(usize) }`; `enum Leva { Mobilitare, Persuadere, Convincere }`; `struct Spesa { pub bersaglio: Bersaglio, pub leva: Leva, pub euro: i64 }`; `enum Intensita { Poco, Medio, Molto }`; `struct VoceAnteprimaCampagna { pub bersaglio: Bersaglio, pub leva: Leva, pub euro: i64, pub intensita: Intensita, pub causa: String }`.
  - `pub struct Spinte { pub valence: Vec<Vec<f64>>, pub mobilita: Vec<Vec<f64>> }` (gruppo × partito) e `pub(crate) fn spinte(dati: &DatiGioco, partiti: &[Partito], n_gruppi: usize) -> Spinte` (scelta 13); `pub(crate) fn convinci(dati, partiti, el: &mut Elettorato)`.
  - `Partito.campagna: Vec<Spesa>`, svuotata dopo il voto.
  - `Comando::Campagna { spese: Vec<Spesa> }` → `Partita::campagna(spese) -> Result<(), Errore>`; sostituisce la campagna già scelta. Errori: «La campagna si decide solo nel turno prima del voto», «Al massimo 3 bersagli», «Bersaglio ripetuto: {nome}», «Spesa non positiva», «Spesa oltre la cassa: {cassa} euro», «Regione sconosciuta».
  - `pub fn anteprima_campagna(&self, spese: &[Spesa]) -> Result<Vec<VoceAnteprimaCampagna>, Errore>` (scelta 15), con gli stessi rifiuti del comando.
  - Spesa in chiusura (scelta 14), riga «Spende {euro} euro: persuade {bersagli}, mobilita {bersagli}, convince {bersagli}».
  - Vista: `VistaCampagna { pub attiva: bool, pub spese: Vec<Spesa> }` per il giocatore.
  - Manopole: `spesa_riferimento` 3.000.000 (2.000.000-3.500.000, `finanziamento-dei-partiti.md:92`), `forza_persuadere`, `forza_mobilitare`, `forza_convincere`, tarate dal test di scala (`effetti-della-campagna-e-promesse.md:51-55`), ticket `26`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/campagna.rs
fn quota_al_voto(spese: &[Spesa]) -> f64 { /* turno 19, cassa del giocatore = spesa_riferimento,
    campagne IA vuote, campagna(spese), chiusura: quota del giocatore nell'esito */ }

#[test]
fn la_campagna_a_cassa_piena_vale_da_mezzo_a_un_punto_e_mezzo() {
    // tre regioni più popolose, persuadere, un terzo della spesa_riferimento ciascuna.
    let guadagno = quota_al_voto(&persuasione) - quota_al_voto(&[]);
    assert!((0.005..=0.015).contains(&guadagno), "{guadagno}");
}

#[test]
fn le_leve_a_pari_spesa() {
    // stessa spesa sugli stessi bersagli: persuadere ≥ mobilitare > convincere (al voto).
}

#[test]
fn convincere_dura_oltre_il_voto() { /* la massa spostata verso la dichiarata è ancora > 0 un turno dopo, e meno del turno prima */ }

#[test]
fn campagna_rifiutata_non_cambia_nulla() {
    // fuori dal turno, 4 bersagli, bersaglio ripetuto, oltre la cassa, euro 0:
    // ogni errore col suo testo, salva() identico.
}

#[test]
fn anteprima_a_fasce() { /* euro = spesa_riferimento/16 → Poco; /4 → Medio; = → Molto; nessun cambio di stato */ }

#[test]
fn spese_al_turno_zero() { /* la partita fondata spende la dote al turno 0: cassa a 1.000.000 − spesa dopo il voto, più le entrate */ }
```

Nel modulo di test di `campagna.rs`: `spinte` con una spesa di persuasione su una condizione scrive la valence solo nei 60 gruppi di quella condizione (20 regioni × 3 fasce) e solo per quel partito.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test campagna`
Expected: FAIL.

- [ ] **Step 3: Implementa tipi, spinte, comando, anteprima e spesa; tara le tre forze col test di scala e scrivi valori e misura nella PR**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): campagna su tre bersagli con mobilitare, persuadere, convincere"
```

---

### Task 9: La campagna dei partiti IA

**Files:**
- Modify: `crates/sim-core/src/partita/ia.rs` (`campagna_ia`, chiamata in chiusura del turno del voto prima della spesa; `fn riparto_spesa(cassa: i64, rendimenti: &[f64]) -> Vec<i64>`), `crates/sim-core/src/partita.rs` (`chiudi_turno`)
- Test: `crates/sim-core/tests/campagna.rs`; test unitario di `riparto_spesa` nel modulo di test di `partita/ia.rs`

**Ricerca:** `docs/ricerca/modelli/effetti-della-campagna-e-promesse.md:52` (radice: divisione ottima in proporzione al quadrato dei rendimenti), `:53-55` (dove rende ogni leva); `docs/ricerca/modelli/sondaggi-ed-errore.md:55` (si decide sul sondaggio, che sbaglia).

**Dispatch:** `rust-engineer` · dopo il Task 8 · skill-gate: true.

**Interfaces:**
- Consumes: `Spesa`, `Bersaglio`, `Leva`, spesa in chiusura (Task 8).
- Produces: `fn campagna_ia(&mut self)` (scelta 16): per ogni partito IA attivo con cassa positiva, `campagna` = 3 spese che sommano la cassa. Turno zero compreso; la protesta spende la dote.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn i_partiti_ia_spendono_tutta_la_cassa() { /* al voto la spesa di ogni partito IA = la sua cassa prima del voto; tre bersagli distinti */ }

#[test]
fn solo_gli_identitari_convincono() { /* nessuna spesa Convincere da un partito non identitario */ }

#[test]
fn la_divisione_segue_il_quadrato_dei_rendimenti() { /* test unitario sulla funzione di riparto: rendimenti 1, 2, 3 → 1/14, 4/14, 9/14, resto al primo */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test campagna`
Expected: FAIL.

- [ ] **Step 3: Implementa `campagna_ia`**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): campagna dei partiti IA sul sondaggio pubblico"
```

---

### Task 10: Le promesse

**Files:**
- Modify: `crates/sim-core/src/partiti.rs` (`Partito.promesse_campagna`, `Partito.promesse`, `Partito.turni_da_presidente`), `crates/sim-core/src/legislatura/campagna.rs` (spinte delle promesse), `crates/sim-core/src/parlamento/intesa.rs:15-40` (`CausaIntesa::Promessa`, «promessa contraria alla sua linea»), `crates/sim-core/src/partita.rs` (`prometti`, `verifica_promesse` prima delle intenzioni nella chiusura del voto, contatore in `chiudi_turno`, `controlla_partiti`), `crates/sim-core/src/partita/ia.rs` (`campagna_ia` aggiunge le promesse IA), `crates/sim-core/src/comando.rs` (`Comando::Prometti`), `crates/sim-core/src/vista.rs` (`VocePartito.promesse`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/promesse.rs` (nuovo)

**Ricerca:** `docs/ricerca/modelli/effetti-della-campagna-e-promesse.md:59` (57% delle promesse mantenute dalle coalizioni italiane), `:60` (peso al voto del mantenimento), `:61` (la mancata pesa 1,5-2 volte la mantenuta), `:44` (Markwat: niente effetto della delusione delle aspettative, decisione 3).

**Dispatch:** `rust-engineer` · dopo il Task 9 · skill-gate: true.

**Architect:** `spinte(dati, partiti, n_gruppi)` del Task 8 non riceve né le posizioni dei gruppi (`Elettorato`) né i gradini in vigore (`Disposizioni`), che servono alla persuasione delle promesse: questo task allarga la firma e aggiorna i chiamanti del Task 8 (`intenzioni_vere` e l'anteprima).

**Interfaces:**
- Consumes: `spinte` (Task 8), `campagna_ia` (Task 9), `StatoAula::linea` e `guadagni` (`parlamento/voto.rs:35-73`), `peso_politiche` (`partita.rs:1006`), `legislatura::programma`.
- Produces:
  - `Partito.promesse_campagna: Vec<(usize, usize)>`, `Partito.promesse: Vec<(usize, usize)>` (in sospeso dal voto prima), `Partito.turni_da_presidente: u32` (azzerato al voto).
  - `Comando::Prometti { promesse: Vec<(String, String)> }` (disposizione, gradino) → `Partita::prometti` (scelta 17). Errori: «Le promesse si fanno solo nel turno prima del voto», «Al massimo 2 promesse», «Promesse già fatte in questa campagna», «Disposizione sconosciuta: {id}», «Gradino sconosciuto: {id}», «Già in vigore: {nome}», «Promessa ripetuta: {nome}».
  - `CausaIntesa::Promessa`.
  - `fn verifica_promesse(&mut self)`: righe «Promessa mantenuta: {nome}» / «Promessa mancata: {nome}» sul partito.
  - `VocePartito.promesse: Vec<VocePromessa { pub disposizione: String, pub gradino: String, pub in_sospeso: bool }>` (nomi del catalogo).
  - Manopole: `persuasione_promessa`, `intesa_promessa`, `valence_promessa_mantenuta`, `valence_promessa_mancata` = 2 × la mantenuta (`effetti-della-campagna-e-promesse.md:61`), `turni_promessa` 4 (spec riga 379), ticket `26`; le prime tre con `# assunzione del piano 8b`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/promesse.rs
#[test]
fn promessa_mantenuta_da_valence() {
    // edita: promessa in sospeso sul gradino in vigore, turni_da_presidente = 4; al voto la valence
    // del giocatore sale di valence_promessa_mantenuta rispetto alla stessa partita senza promessa.
}

#[test]
fn promessa_mancata_toglie_di_piu() { /* gradino diverso dal vigente: −valence_promessa_mancata, e |mancata| > |mantenuta| */ }

#[test]
fn senza_quattro_turni_la_promessa_non_conta() { /* turni_da_presidente = 3: valence invariata, nessuna riga */ }

#[test]
fn la_promessa_costa_intesa_agli_alleati_con_linea_no() {
    // al governo, turno 19: una promessa lontana dal centrodestra gli toglie intesa con causa
    // Promessa; una vicina no.
}

#[test]
fn la_promessa_persuade_chi_ne_guadagna() { /* al voto la quota del giocatore con la promessa ≥ senza */ }

#[test]
fn promesse_ia_dal_programma() { /* il governo IA promette le prime 2 voci del suo programma */ }

#[test]
fn promessa_rifiutata_non_cambia_nulla() { /* ogni errore col suo testo, salva() identico */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test promesse`
Expected: FAIL.

- [ ] **Step 3: Implementa promesse, verifica, contatore, promesse IA e vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): promesse di campagna con verifica al voto dopo"
```

---

## Rimandato

- **Piano 9 (eventi e informazione):** acquisti d'informazione con la cassa (sondaggio privato, test di una misura, centro studi; storie 123-124), che tolgono denaro alla campagna.
- **Piano 11 (UI):** schermate della campagna (bersagli, anteprima a fasce, promesse), casse dei rivali, righe di spesa nell'esito, minacce di scissione, nascite e scioglimenti sull'asse.
- **Piano 12 (bilanciamento):** `passo_movimento`, `soglia_protesta`, `distanza_protesta`, `soglia_scissione`, `soglia_minaccia`, `quota_scissione`, `margine_coalizione`, le entrate e la quota libera, le tre forze della campagna, i pesi delle promesse, contro i bersagli del costo del governare (ticket 29). Dalla ricerca: un coefficiente per partito sul 2 per mille (scarto reale 0,3-2,8, `finanziamento-dei-partiti.md:84`); donazioni legate anche ai seggi e più alte negli anni di voto (`:97`); la deriva al centro degli Hunter nati in periferia, che spegne la protesta in un paio di legislature (`vita-dei-partiti.md:133`).
- **Non nella v1:** fusioni fra partiti (ticket 18, riga 65; Ibenskas 2016 ne conta circa una ogni tre legislature); scambi di posizione fra vicini (il 14,6% delle coppie nei dati CHES); promesse che alzano l'aspettativa (decisione 3).
- **Rilievi dell'8a non presi nel Task 0:** la guardia del programma che non somma le voci dello stesso turno (`partita/ia.rs:228-236`) e due rami senza test (`partita.rs:2309-2313`, `:3284`) dalla #34; le sfiducie individuali raddoppiate, la riga della proposta senza test per «nessun seggio» e «1 seggio», `rinascerebbe_uguale` che copia la regola dell'incarico, le imprecisioni di `CONTEXT.md` e «dei partiti alleati» col giocatore che governa da solo dalla #35. Restano nelle PR, per il bilanciamento o per un giro di pulizia.
