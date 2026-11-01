# V1 · Piano 10: scenario e salvataggi — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Il giocatore sceglie fra l'Italia di oggi e uno scenario su misura, che costruisce con sei cursori e due menu guardando un riepilogo dal vivo e salva con un nome. Poi fonda il partito e gioca una partita che si salva da sola a ogni apertura e a ogni scelta, si riprende dopo la chiusura dell'app, si riavvolge all'apertura di un turno passato con lo stesso caso e finisce nell'albo delle partite. Più partite restano in corso insieme.

**Architecture:** Lo scenario su misura è un valore del core, `SuMisura`: cursori assoluti, cursori sintetici, legge elettorale, gradini delle disposizioni e sostituzioni puntuali. `DatiGioco::con_su_misura` lo applica ai dati base parsati e lo conserva dentro `DatiGioco`, quindi lo snapshot ne porta la copia. Il file su disco ne tiene solo le differenze dalla base (`SuMisura::in_toml`/`da_toml`). Il riepilogo dal vivo (`riepilogo::riepilogo`) costruisce per ogni posizione la partita fondata e legge un sondaggio atteso senza rumore (`Sondaggio::atteso`). Il core non fa I/O: dà lo snapshot (`salva`/`carica`), i tipi dell'indice e dell'albo, la serie degli indicatori sintetici e il contatore dei riavvolgimenti. Il guscio Tauri fa il resto: una cartella per partita in `app_data_dir`, scrittura atomica, gzip con `flate2`, bozza scritta a ogni comando e in `RunEvent::Exit`.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`); Tauri 2.12.3; una sola dipendenza diretta nuova, `flate2 = "=1.1.10"` nel guscio, già presente in `Cargo.lock` per via transitiva.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md):
- storie 1-7 (righe 26-32) e 143-152 (righe 210-219);
- «Modulo scenario» (righe 274-285);
- «Turno e partita» (righe 389, 391-393).

Ticket: `.scratch/v1-giocabile/issues/21-editor-dello-scenario.md` (tabella dei cursori, limiti e avvisi, disposizioni cambiate dall'editor) e `28-salvataggi.md` (modello, contenuto, versioni, albo, scelte di strumento).

Rimandi dei piani precedenti:
- piano 7a `:804` e `:1289`: l'albo legge `Legislatura.elezioni`;
- piano 7b `:52` e `:1226`: i riavvolgimenti li conta il guscio, l'eredità si conferma nell'albo, gli snapshot si cancellano alla conferma.

Sequenza dei piani: piano 1 `:51`.

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · scenario | `feat/v1-piano-10-scenario` | `main` | 0-5 | schema 11, `SuMisura` e file di differenze, i sei cursori, i due menu, riepilogo dal vivo e avvisi, `sim-cli --scenario` |
| B · salvataggi | `feat/v1-piano-10-salvataggi` | `feat/v1-piano-10-scenario` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 6-8 | serie degli indicatori sintetici, riavvolgimenti nell'eredità, tipi di indice e albo, test di completezza e riproducibilità |
| C · guscio | `feat/v1-piano-10-guscio` | `feat/v1-piano-10-salvataggi` (impilata, stessa regola) | 9-10 | archivio su disco nel guscio, comandi Tauri delle partite, degli scenari e dell'albo, bozza all'uscita |

Questo file entra nel primo commit della PR A, insieme agli aggiornamenti di `docs/ricerca/` di questo piano (tre pagine nuove, `tecnica/tauri.md`, `index.md`, `log.md`, i sidecar delle 61 fonti nuove).

## Decisioni di Mario

Del 01/11/2026, sulle domande di questo piano (memoria `decisioni-piano-10`), dopo le ricerche `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md`, `docs/ricerca/tecnica/persistenza-dei-salvataggi.md`, `docs/ricerca/sintesi/cursori-dello-scenario.md`:

1. **Le disposizioni costituzionali restano fuori dall'editor** (numero dei parlamentari, durata della legislatura: `Norma::Costituzionale`). Struttura fissa, spec riga 277. Task 4.
2. **L'avviso «situazione o crisi già accesa al turno zero» va al piano 9**, che porta le situazioni. Il piano 10 lascia il punto d'aggancio: `RiepilogoScenario.avvisi` è un `Vec<Avviso>`, e il piano 9 vi aggiunge una variante. Nessun avviso provvisorio. Task 5.
3. **Gli scenari su misura vivono nella cartella dell'app**, `app_data_dir/scenari/<nome>.toml`, elencati per nome. Niente `tauri-plugin-dialog`. Task 9.

Restano valide le decisioni dei piani 6-8b (memorie `decisioni-piano-6-aperte`, `decisioni-piano-7`, `decisioni-piano-7b`, `decisioni-piano-8`, `decisioni-piano-8b`).

## Scelte del piano, da confermare in review

Dichiarate a Mario il 01/11 prima delle domande, senza obiezioni:

1. **Una cartella per partita**, non «un file ciascuna» (spec riga 391): `partite/<id>/indice.json`, `turno-NNN.json.gz` (uno per apertura), `bozza.json.gz`. Così l'elenco legge l'indice senza aprire la partita, anche con uno schema diverso, e il riavvolgimento cancella file invece di riscriverne uno da 17 MB (`persistenza-dei-salvataggi.md:97-114`).
2. **La bozza si scrive a ogni comando accettato e in `RunEvent::Exit`**, non solo alla chiusura (`salvataggi-e-riavvolgimento.md`, «Per il nostro gioco» punto 4). Con Cmd+Q su macOS arriva solo `RunEvent::Exit` (`persistenza-dei-salvataggi.md:23-45`). Un crash costa al massimo l'ultima scelta.
3. **Il contatore dei riavvolgimenti fa testo nell'indice**, perché lo snapshot del turno N ne porta il valore vecchio. A ogni caricamento il guscio lo ricopia nella partita (`Partita::imposta_riavvolgimenti`), e così entra nell'eredità.
4. **Disposizioni dell'editor via `Disposizioni::cambia` al turno zero**, con l'etichetta della legge del governo uscente e il ritardo normale degli effetti. `partenza` resta il riferimento di saldo, leve ed effetti (`cursori-dello-scenario.md:96-110`). Riscriverla annullerebbe gli effetti: la ricerca lo dimostra su `effetti.rs:62-80`.
5. **Ricchezza.** Le fasce di reddito sono relative (`cursori-dello-scenario.md:37`), quindi nessun gruppo cambia fascia. «Più ricco» morde attraverso le soglie in euro: scaglioni IRPEF, pensione minima, salario e reddito minimo. «Quote dei gruppi fra le fasce» esce dai derivati del ticket 21.
6. **Età.** La piramide si inclina in modo esponenziale, come una popolazione stabile (`cursori-dello-scenario.md:60-62`); pensionati e spesa per pensioni ne derivano.
7. **Intervalli dei cursori** dalla ricerca (`cursori-dello-scenario.md:85-92`), provvisori fino al piano 12: debito 110-160% del PIL, ricchezza ±20%, disoccupazione 4,7-11,6%, inflazione 0-8%, età mediana da −5 a +3 anni, orientamento ±1.

Scelte di strumento di questo piano:

8. **`VERSIONE_SCHEMA` passa da 10 (8b, su main) a 11** nel Task 0 e resta 11 per tutto il piano.
21. **Ripresa** (`persistenza-dei-salvataggi.md:115`): la bozza vale solo se il suo turno coincide con quello dell'ultimo `turno-K` leggibile. Altrimenti, per esempio con un crash fra la scrittura di `turno-(K+1)` e la cancellazione della bozza, vince lo snapshot e la bozza vecchia si cancella.
22. **Gruppi spostati dai cursori** (disoccupazione, età): chi riceve quota ne mescola l'istogramma della posizione, pesato sulle masse, come nella partita (`Elettorato::sposta`, `composizione.rs:16-42`). Nei dati c'è solo la posizione; ancora, giudizio e voto precedente nascono da lì in `Elettorato::iniziale`.
23. **Maggioranza a Camere sciolte** (serie del turno zero e di ogni turno a Camere sciolte): la metà più uno si calcola sui seggi delle leve in vigore (`seggi_camera`, `seggi_senato`), non sui seggi assegnati, che sono zero.
24. **Riepilogo prima della campagna.** Il riepilogo legge le intenzioni del turno zero prima delle campagne IA, che con l'8b partono a cassa piena: la maggioranza attesa può discostarsi dal voto vero. Lo dice il testo dell'avviso.
9. **Un solo intervallo per grandezza.** I tre cursori assoluti (debito, disoccupazione, inflazione) scrivono gli ingressi `debito_pil`, `disoccupazione` e `inflazione` di `scenario.toml`, i cui intervalli si stringono ai valori della scelta 7. I tre sintetici (ricchezza, età, orientamento) hanno un `Ingresso` nuovo in `[cursori]` di `scenario.toml`, con valore 0. Le sostituzioni non possono nominare i tre ingressi dei cursori.
10. **Ordine d'applicazione fisso:** cursori assoluti coi loro derivati, poi ricchezza, età, orientamento, legge elettorale, gradini elettorali, sostituzioni. Le sostituzioni vengono per ultime e vincono. Le disposizioni ordinarie si applicano dopo, in `costruisci`, sopra `Disposizioni::iniziali`.
11. **Derivati dei cursori assoluti.**
    - Disoccupazione: `spesa_sussidi` scala con `u₁/u₀`, così il sussidio a testa resta fermo e la NASpI segue i disoccupati (`previdenza.rs:41`). Le quote dei gruppi si spostano una volta, con la logica di `Elettorato::lavoro` (`composizione.rs:76-100`), regione per regione, con Δ = u₁ − u₀.
    - Inflazione: `tasso_bce` sale di `(1 + taylor_inflazione) · quota_italia_area_euro · Δπ`, cioè lo spostamento dell'obiettivo della regola (`bce.rs:23-27`). Restano ferme l'inerzia e `inflazione_t2`/`inflazione_t3`: il cursore è lo shock in corso.
12. **Il regime pensionistico della prima annata resta quello di partenza** (`previdenza.rs:37`): l'annata 2026 è già in pensione. Un regime cambiato dall'editor vale dalla prima annata di gennaio, come una legge del 2026.
13. **Etichetta della legge dell'editor:** `ETICHETTA_GOVERNO_USCENTE = "Legge del governo uscente (2026)"`, mai della forma «L. n/anno» (`partita.rs:118-120`). Vale anche per le disposizioni elettorali quando l'editor cambia la legge elettorale: oggi la loro etichetta è la costante `LEGGE_ELETTORALE` (`disposizioni/mod.rs:46`).
14. **Avviso della coalizione vincibile, in seggi.** Per ogni posizione la coalizione possibile è il giocatore più i partiti IA disponibili con la dichiarazione vuota, letti sul sondaggio atteso. L'avviso scatta dove non ha la maggioranza assoluta in entrambe le Camere col riparto atteso. Con la legge 180 è il 42% della spec (riga 282), e vale anche con le altre leggi del menu.
15. **Riepilogo lento in debug:** `[profile.dev.package.sim-core] opt-level = 2` nel `Cargo.toml` della radice (`cursori-dello-scenario.md:169-176`). Il riepilogo costruisce una partita per posizione, circa 30 ms in release. La via più leggera della ricerca (§5, punti 1-4) va nel Rimandato.
16. **L'albo è JSON in chiaro** con il suo `formato_albo = 1`, separato da `VERSIONE_SCHEMA`. I campi si aggiungono soltanto, e in un formato nuovo solo come `Option`; niente enum chiuse; i valori sono copiati per esteso (`persistenza-dei-salvataggi.md:87-95`). Serde ignora da sé i campi sconosciuti, quindi niente `serde(default)` e niente `deny_unknown_fields`.
17. **In debug il guscio lavora in `app_data_dir/sviluppo/`**, per non mescolare le partite di `tauri dev` con quelle vere (`persistenza-dei-salvataggi.md:128-130`).
18. **Il riavvolgimento va all'apertura di un turno N ≤ turno corrente.** N uguale al turno corrente scarta la bozza: si ricomincia il turno. Anche così conta come riavvolgimento.
19. **Abbandono e conferma dell'eredità** scrivono `albo/<id>.json` e poi cancellano la cartella della partita (spec riga 392). Nessuna conferma nel core: confermare è un'azione del guscio.
20. **La partita di prova** (`nuova_partita`, `Partita::nuova`) resta senza salvataggi, per la pagina provvisoria `app/src/routes/+page.svelte` fino al piano 11.

## Global Constraints

Valgono tutti i vincoli del piano 8a (`2026-10-28-v1-piano-8a-governo-e-opposizione-ia.md:56-70`) e, attraverso quelli, dei piani 1-7b:
- italiano coi termini di `CONTEXT.md` (scenario, partita, salvataggio, riavvolgimento, albo delle partite, eredità, indicatore sintetico; «posizione» sull'asse, mai «posto», che resta per il calendario);
- `sim-core` senza I/O e senza thread;
- solo `Vec`, array fissi e `BTreeMap`;
- euro in `i64`;
- dati in TOML con errori che nominano il colpevole;
- niente `serde(default)` nei tipi dello snapshot.

In più:

- **Dipendenze.** Una sola nuova: `flate2 = "=1.1.10"` in `app/src-tauri/Cargo.toml`. In `Cargo.lock` cambia solo la lista delle dipendenze di `app`, nessuna versione nuova. Niente `tempfile`, `atomicwrites`, `tauri-plugin-fs` o `tauri-plugin-dialog`.
- **`VERSIONE_SCHEMA` passa a `11` nel Task 0.**
- **Base: `main` a `c56787d`, con l'8b fuso** (#36, #37, #38; `VERSIONE_SCHEMA = 10`). Le righe di `partita.rs`, `dati.rs` e `scenario.rs` citate in questo piano sono di `95f18cb`, prima dell'8b: si cerca per nome di funzione o di tipo (tabella degli spostamenti nell'annotazione dell'architect). Se `main` si muove durante l'esecuzione, ogni branch si riallinea con un merge da `origin/main`, mai rebase né force-push; in `carica` si tengono tutte le validazioni; `VERSIONE_SCHEMA` resta 11.
- **Partiti che nascono e si sciolgono (8b).** `partiti.len()` cambia durante la partita: serie, albo e riepilogo lo reggono, e i partiti sono sempre copiati per id o per nome, mai per indice, in ogni formato che esce dal core.
- **File di differenze.** Il file TOML dello scenario su misura passa da un tipo suo (`FileSuMisura`, con `deny_unknown_fields`). Lì i campi mancanti sono le differenze non scritte, quindi `Option` o `serde(default)`, come negli altri file di dati. `SuMisura`, che va nello snapshot, ha tutti i campi pieni.
- **Validazione contro la base.** I valori di uno scenario su misura si controllano sugli intervalli dei dati di base, mai su intervalli scritti nel file di differenze.
- **Tipi esportati.** Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa. Le schermate (editor, elenco delle partite, albo, «torna al turno N») sono del piano 11.
- **Testi.** I testi nuovi (avvisi, errori, etichetta della legge, nome «Italia di oggi») si elencano nella PR, per la revisione di Mario.
- **Verifiche di ogni task:** `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- **Partite dei test.** Partita di prova (`Partita::nuova`): sette partiti, il giocatore è la destra (5) al governo; turno 0 = ottobre-dicembre 2026. Partita vera: `comune::fondata(posizione)` (`tests/comune/mod.rs:319`).

## Review Focus

1. **File di differenze ostile.** Casi:
   - nome di campo o di ingresso sconosciuto;
   - valore fuori intervallo o non finito;
   - disposizione costituzionale;
   - gradino o impostazione elettorale inesistente;
   - sostituzione di un ingresso che è un cursore.

   Rifiuto con il nome del colpevole e nessun panic. Test `su_misura_ostile_rifiutato_col_nome` nel Task 0.
2. **Nomi e id che arrivano dalla UI** (nome di scenario, id di partita). Casi: `/`, `\`, `..`, punto iniziale, vuoto, oltre 60 caratteri, caratteri di controllo. Rifiuto prima di toccare il disco, così nessun percorso esce dalla cartella dell'app. Test `nome_fuori_cartella_rifiutato` nel Task 9.
3. **Scrittura interrotta o file rovinato.** Restano un `.tmp` orfano, una bozza troncata o uno snapshot illeggibile. La ripresa ripiega sull'ultimo snapshot leggibile, l'elenco ignora i `.tmp` e nessun file rifiutato viene riscritto. Test `bozza_rovinata_ripiega_sullo_snapshot` nel Task 9.
4. **Partita di uno schema diverso.** L'elenco la mostra non apribile, con un messaggio che nomina le due versioni. La ripresa la rifiuta e i suoi file restano identici byte per byte. Un file senza `versione_schema` è «illeggibile», non «schema 0». Test `schema_diverso_elencato_e_intatto` nel Task 9 e `senza_versione_e_illeggibile` nel Task 6.
5. **Stesse scelte dopo il riavvolgimento.** Stessi comandi nello stesso ordine danno la stessa partita, bit per bit, anche con comandi che estraggono dal caso. Test `stesse_scelte_stesso_esito_dopo_il_ricaricamento` nel Task 8.

## Dispatch

> **Annotazione dell'architect, 2026-11-01.** Letta da worktree `docs-v1-piano-10`, branch `docs/v1-piano-10`, HEAD `95f18cb`, confrontata con `origin/main` = `c56787d`. Righe **Ricerca** verificate sulle pagine; tre correzioni nei Task 7, 9 e 10. Assegnazione: `rust-engineer` ovunque, confermata (ogni task tocca `.rs`); skill-gate true ovunque, nessuno step meccanico.
>
> **L'8b è già fuso.** `origin/main` contiene #36, #37 e #38 (`c56787d feat(core): piano 8b, campagna e promesse (PR C) (#38)`), con `VERSIONE_SCHEMA = 10` (`partita.rs:41` su main). Il «Coordinamento con l'8b» dei Global Constraints descrive PR ancora aperte: il rischio di conflitto fra branch non c'è più, ne restano due conseguenze.
> 1. **Le righe del piano sono quelle di `95f18cb`.** La PR A parte da `main` e le trova spostate: `partita.rs` ha +818 righe (`VERSIONE_SCHEMA` 39→41, `struct Partita` 85→87, `costruisci` 131→143, `chiudi_turno` 279→346, `eredita_di` 1815→2082, `quadro` 2007→2296, `coalizione_del_giocatore` 2247→2589, `salva` 3141→3516, `carica` 3165→3540), `dati.rs` +21 (`DatiGioco` 314→335, `da_testi` 507→528), `scenario.rs` +2 (`ingressi` 90→92). Restano dove sono: `disposizioni/*`, `elettorato/composizione.rs`, `Sondaggio` (`sondaggio.rs:37-42`), `Quadro` (`coalizione.rs:118-136`), `caso.rs`, `app/src-tauri/src/lib.rs`. Ogni brief dice all'agente di cercare per nome, non per numero, e la PR non riporta righe di `95f18cb` come se fossero di main.
> 2. **`docs/ricerca/index.md` e `log.md`.** Sono modificati nel worktree, non committati, e l'8b li ha toccati su main (+3 e +6 righe). Prima del primo commit della PR A serve un merge di `origin/main` in `docs/v1-piano-10`, o la copia dei file sul branch nuovo; altrimenti il conflitto arriva nel commit che porta il piano.
>
> Altri punti di contatto con l'8b, da tenere nei brief:
> - **Partiti che nascono e si sciolgono.** `partiti.len()` cambia durante la partita e `Partito.sciolto` esiste (`coalizione.rs:243-247` su main). `PuntoSerie`, `ElezioneAlbo.voti` (copiati per nome, va bene) e `coalizione_del_giocatore` lo devono reggere.
> - **Le casse IA del turno zero.** Le casse IA nascono piene in `costruisci` (`partita.rs:169` su main), e il voto del turno zero sconta le campagne IA. Il riepilogo legge il sondaggio prima della campagna, quindi la «maggioranza attesa» può discostarsi dal voto vero. `comune::fondata` azzera le casse IA proprio per questo motivo (`tests/comune/mod.rs:314-334` su main). Il Task 5 lo dichiara nel testo dell'avviso o nella PR.
> - **Il partito del giocatore ha più campi** (`partito_nuovo`, ora `pub(crate)` in `fondazione.rs:57` su main): il riepilogo lo riusa senza ricostruirlo.
> - Su main c'è `quote_fra_i_votanti(&Intenzioni)` (`partita.rs:120-126`): calcola la quota attesa del Task 5 senza scriverne una seconda.
>
> **Sequencing, file per file.** Confermato in sequenza dentro ogni PR, con un'eccezione possibile fra B e C.
> - *PR A.* 0-4 scrivono tutti `su_misura.rs` e `tests/su_misura.rs`; 0 e 5 scrivono entrambi `sim-core/src/lib.rs`. Il 5 testa l'orientamento (Task 2) e `proporzionale_puro` (Task 4), e il suo `saldo_disposizioni_pil` presuppone il Task 4. Niente in parallelo.
> - *PR B.* 6 e 7 scrivono `salvataggi.rs`, `partita.rs` e `tests/salvataggi.rs`. L'8 scrive solo `tests/salvataggi.rs`, ma se un test fallisce diventa un fix in `partita.rs`, e deve vedere i campi nuovi del 6 nello snapshot. Niente in parallelo.
> - *Task 9 contro 6-8.* Il 9 consuma `Testata` e `riavvolgimenti` (Task 6), `VoceAlbo` e `voce_albo` (Task 7) e aggiunge `IndicePartita` a `salvataggi.rs`, quindi non parte prima della fine del 7. Con l'**8** non condivide file (`archivio.rs`, `app/src-tauri/Cargo.toml`, `Cargo.lock` e `salvataggi.rs` contro `tests/salvataggi.rs`, più l'eventuale fix in `partita.rs`). 8 ‖ 9 si può fare sui file, ma apre il branch C prima della review della B, e la review può cambiare `Testata`/`VoceAlbo`, che il 9 consuma. Raccomandazione: resta in sequenza; il guadagno è un solo task.
> - *PR C.* Il 10 chiama l'`Archivio` del 9 e scrive `lib.rs` del guscio. In sequenza.
>
> **Firme da correggere in esecuzione** (elencate nel task; le scelte di Mario e del piano restano come sono):
> - Task 0: `Scenario::ingressi` è privato e dà `&Ingresso` (`scenario.rs:90-94`), e i campi di `DatiGioco` sono privati (`dati.rs:314-325`). Le sostituzioni e i Task 1-4 hanno bisogno di accessori `pub(crate)` mutabili, oppure di `con_su_misura` scritta in `dati.rs`. Manca anche `scenario.rs` fra i file, per `CursoriBase`.
> - Task 1: `sposta_lavoro(quote: &mut [f64], …)` non può lasciare `Elettorato::lavoro` identico nei risultati. `Elettorato::sposta` mescola anche `posizione`, `ancora`, `giudizio` e `voto_precedente` del gruppo che riceve (`composizione.rs:16-42`), e `ripartisci` pesa con le quote dei dati (`composizione.rs:56-64`).
> - Task 3: estrarre la logica di `composizione.rs:149-190` significa modificare `composizione.rs`, che manca fra i file. Il problema del mescolamento è lo stesso del Task 1.
> - Task 5: il `Quadro` di `Partita` è privato (`partita.rs:2007-2024`). Per leggerlo sul sondaggio atteso serve un `pub(crate) fn` in `partita.rs`, che manca fra i file. Quanto ai «seggi della coalizione col riparto atteso», `Quadro::seggi` li ripartisce secondo la `Dichiarazione` (`coalizione.rs:141-162`): con la dichiarazione vuota il giocatore corre da solo e il premio della 180 va calcolato su un'altra coalizione.
> - Task 9: `riprendi` («la bozza se si legge») si discosta dalla regola di ripresa della ricerca. Lì la bozza vale solo se il suo turno coincide con l'ultimo `turno-K` (`persistenza-dei-salvataggi.md:115`). Dettagli nel task.
>
> **Risolti il 01/11 dal coordinatore**, nel piano:
> - ripresa: scelta 21, Task 9;
> - mescolamento dei gruppi spostati: scelta 22, firma `spostamenti_lavoro` nel Task 1;
> - maggioranza a Camere sciolte: scelta 23;
> - riepilogo prima della campagna: scelta 24, testo dell'avviso;
> - dichiarazione dei seggi attesi e `Quadro` sul sondaggio atteso: Task 5;
> - accessori e file mancanti: Task 0, 3 e 5;
> - `State<DatiGioco>` e creazione in `.setup`: Task 10;
> - regola di formato di indice e testata: Task 9;
> - Global Constraints riscritti sull'8b fuso.
>
> 8 e 9 restano in sequenza.
>
> **Casi limite dalla storia del repo** (`git log --all --format=%s | grep -i '^fix'`): la classe che si ripete è quella delle correzioni dopo la review a sei e dei controlli di `carica` (`fix(core): testi, carica e test del governo IA`, `fix(core): eredità su debito dell'anno mobile e turni da premier`). Nel Task 6 vanno quindi sotto `carica` anche i campi nuovi: serie con turni crescenti da 0 al turno corrente e valori finiti. Il Task 7 confronta l'eredità dell'albo con `Partita::eredita`, invece di ricalcolarla.

PR A: 0 → 1 → 2 → 3 → 4 → 5 in sequenza. Tutti toccano `su_misura.rs`; 1-3 anche `scenario.toml` ed `elettorato`; il 5 legge tutto.

PR B: dopo la review della A, sul branch impilato, 6 → 7 → 8 in sequenza. Il 7 usa la serie e il contatore del 6; l'8 gioca le partite del 6 e del 7.

PR C: dopo la review della B, 9 → 10 in sequenza. Il 10 chiama l'archivio del 9.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task.

A fine PR, review a sei in parallelo:
- `security-reviewer`: input esterno dal file di differenze e dalla UI, percorsi su disco;
- `code-reviewer`;
- `test-writer`;
- `craft-reviewer`: legge i testi elencati nella PR;
- `rust-reviewer`: il diff tocca `.rs`;
- `spec-reviewer`: questo piano, con i findings in un report a parte.

Poi un solo giro di correzioni. Le decisioni prese in esecuzione si elencano nella PR. Il merge lo decide Mario.

---

## PR A · Scenario (Task 0-5)

### Task 0: Schema 11 e file di differenze

**Files:**
- Create: `crates/sim-core/src/su_misura.rs`
- Modify: `crates/sim-core/src/lib.rs` (modulo e riesportazioni), `crates/sim-core/src/scenario.rs` (`CursoriBase`, accessori `pub(crate)` per le sostituzioni per nome), `crates/sim-core/src/dati.rs:314-325` (`DatiGioco` con `su_misura`, accessori `pub(crate)` mutabili per i Task 1-4), `crates/sim-core/src/partita.rs:39` (`VERSIONE_SCHEMA = 11`), `crates/sim-core/src/errore.rs` (variante `SuMisura`), `dati/base/scenario.toml` (tabella `[cursori]`; intervalli di `debito_pil` 110-160, `disoccupazione` 4,7-11,6, `inflazione` 0-8)
- Test: `crates/sim-core/tests/su_misura.rs`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:114` — caricamento di oggi, dove inserire le differenze
- `docs/ricerca/sintesi/cursori-dello-scenario.md:141` — validazione del file di differenze
- `docs/ricerca/sintesi/cursori-dello-scenario.md:85` — intervalli dei cursori

**Dispatch:** `rust-engineer` · primo della PR A · skill-gate: true. Nota dell'architect: anche `scenario.rs` va modificato, per `CursoriBase` dentro la macro `scenario!` e per gli accessori. `Scenario::ingressi` è privato e immutabile (`scenario.rs:90-94`), i campi di `DatiGioco` sono privati (`dati.rs:314-325`). Questo task lascia accessori `pub(crate)` mutabili, o scrive `con_su_misura` in `dati.rs`, così i Task 1-4 non riaprono `dati.rs` ogni volta. Da decidere qui: se `ingressi()` elenca anche i tre `CursoriBase`, le sostituzioni li escludono come `NOMI_CURSORI_ASSOLUTI` (scelta 9). Righe di `95f18cb`: su main, `dati.rs` +21 e `partita.rs:41`.

**Interfaces:**
- Consumes: `DatiGioco::da_testi` (`dati.rs:507`), `Scenario::ingressi` (`scenario.rs:90-94`), `Ingresso` (`scenario.rs:10-16`), `Errore::ScenarioFuoriIntervallo`.
- Produces:
  - `pub struct Cursori { pub debito_pil: f64, pub disoccupazione: f64, pub inflazione: f64, pub ricchezza: f64, pub eta_mediana: f64, pub orientamento: f64, pub legge_elettorale: String }`
  - `pub struct SuMisura { pub nome: String, pub cursori: Cursori, pub disposizioni: BTreeMap<String, String>, pub sostituzioni: BTreeMap<String, f64> }`, con `Serialize`, `Deserialize` e TS export
  - `SuMisura::base(dati: &DatiGioco) -> SuMisura` (nome `"Italia di oggi"`, valori della base)
  - `SuMisura::da_toml(testo: &str, base: &DatiGioco) -> Result<SuMisura, Errore>`
  - `SuMisura::in_toml(&self, base: &DatiGioco) -> String`: solo le differenze, e `da_toml(in_toml(s)) == s`
  - `DatiGioco::con_su_misura(&self, s: &SuMisura) -> Result<DatiGioco, Errore>`: clone, applica, `valida`
  - `DatiGioco::su_misura(&self) -> Option<&SuMisura>` e `DatiGioco::nome_scenario(&self) -> &str`
  - `Errore::SuMisura(String)`, che si stampa come «scenario su misura non valido: {m}»
  - `pub(crate) const NOMI_CURSORI_ASSOLUTI: [&str; 3] = ["debito_pil", "disoccupazione", "inflazione"]`

  In questo task `con_su_misura` applica i tre cursori assoluti (solo l'ingresso, senza derivati), la legge elettorale come nome e le sostituzioni. I cursori sintetici diversi da 0 e le disposizioni si validano ma non si applicano ancora: arrivano coi Task 1-4.

- [ ] **Step 1: Scrivi i test che falliscono**

In `tests/su_misura.rs`:

```rust
#[test]
fn la_base_non_cambia_i_dati() {
    let base = comune::dati_base();
    let d = base.con_su_misura(&SuMisura::base(&base)).unwrap();
    assert_eq!(d.scenario(), base.scenario());
    assert_eq!(d.nome_scenario(), "Italia di oggi");
}

#[test]
fn andata_e_ritorno_del_file() {
    let base = comune::dati_base();
    let mut s = SuMisura::base(&base);
    s.nome = "Italia indebitata".into();
    s.cursori.debito_pil = 150.0;
    s.sostituzioni.insert("costo_medio_debito".into(), 3.2);
    let testo = s.in_toml(&base);
    assert!(!testo.contains("ricchezza"), "solo differenze: {testo}");
    assert_eq!(SuMisura::da_toml(&testo, &base).unwrap(), s);
}

#[test]
fn sostituzione_e_cursore_assoluto_arrivano_allo_scenario() {
    // debito_pil = 150 e costo_medio_debito = 3,2: lo scenario dei dati li porta
}

#[test]
fn su_misura_ostile_rifiutato_col_nome() {
    // Ogni caso: errore il cui testo contiene il nome indicato, mai panic.
    // campo sconosciuto in [cursori] ("pil")                      -> "pil"
    // sostituzione sconosciuta ("debito")                          -> "debito"
    // sostituzione di un cursore assoluto ("debito_pil")           -> "debito_pil"
    // debito_pil = 170 (fuori 110-160)                             -> "debito_pil"
    // ricchezza = 25 (fuori ±20)                                   -> "ricchezza"
    // orientamento = nan                                           -> "orientamento"
    // legge_elettorale = "maggioritario"                           -> "maggioritario"
    // disposizioni: numero_parlamentari = "400"  (costituzionale)  -> "numero_parlamentari"
    // disposizioni: cuneo_fiscale = "99" (gradino inesistente)     -> "cuneo_fiscale"
    // disposizioni: inesistente = "0"                              -> "inesistente"
}
```

E in `tests/partita.rs`: `salva` contiene `"versione_schema":11`.

- [ ] **Step 2: Verifica che falliscano**

Run: `cargo test -p sim-core --test su_misura`
Expected: FAIL, `SuMisura` non esiste.

- [ ] **Step 3: Implementa**

`FileSuMisura` è privato, con `#[serde(deny_unknown_fields)]` su ogni tabella:

```toml
nome = "…"
[cursori]        # tutti facoltativi
[disposizioni]   # id disposizione = id gradino, facoltativa
[sostituzioni]   # nome come in Scenario::ingressi, facoltativa
```

`da_toml` lo riempie dalla base e chiama la stessa validazione di `con_su_misura`.

La validazione usa sempre la base:
- i cursori assoluti sugli `Ingresso` della base;
- i sintetici su `[cursori]` della base;
- le sostituzioni sui nomi di `Scenario::ingressi`, meno `NOMI_CURSORI_ASSOLUTI`; il valore lo controlla `valida` alla fine;
- le disposizioni contro il catalogo (`DatoDisposizione::gradino`), con rifiuto di `Norma::Costituzionale`.

`[cursori]` in `scenario.toml` contiene `ricchezza`, `eta_mediana` e `orientamento`, ciascuno `Ingresso` con `valore = 0`. Per ognuno, la fonte e l'intervallo sono quelli della scelta 7, con il commento `# cursori-dello-scenario:88-92`. Il campo nuovo di `Scenario` si chiama `cursori: CursoriBase { ricchezza: Ingresso, eta_mediana: Ingresso, orientamento: Ingresso }`, e `Scenario::valida` controlla anche questi.

- [ ] **Step 4: Verifica**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS. I test che oggi mettono `debito_pil`, `disoccupazione` o `inflazione` fuori dai nuovi intervalli vanno corretti, ed elencati nella PR.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/scenario.toml app/src/lib/tipi docs
git commit -m "feat(core): scenario su misura come differenze dalla base (piano 10, Task 0)"
```

### Task 1: Derivati dei tre cursori assoluti

**Files:**
- Modify: `crates/sim-core/src/su_misura.rs`, `crates/sim-core/src/elettorato/composizione.rs:76-100` (estrai lo spostamento per regione)
- Test: `crates/sim-core/tests/su_misura.rs`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:46` — NASpI a spesa fissa e quote dei gruppi ferme
- `docs/ricerca/sintesi/cursori-dello-scenario.md:52` — tasso BCE fermo con l'inflazione di partenza

**Dispatch:** `rust-engineer` · dopo il Task 0 · skill-gate: true. Nota dell'architect: con la firma `sposta_lavoro(quote: &mut [f64], …)` `Elettorato::lavoro` cambia risultato, perché `Elettorato::sposta` mescola anche `posizione`, `ancora`, `giudizio` e `voto_precedente` del gruppo che riceve (`composizione.rs:16-42`). Una forma che regge: una funzione pura che dà gli spostamenti `(da, a, massa)` per regione, che `lavoro` applica con `sposta`/`ripartisci` e `su_misura` sulle quote dei dati. Copia le quote prima di passarle: `ripartisci` legge le quote dei dati come pesi (`composizione.rs:56-64`). Va deciso, e scritto nella PR, se nei dati il gruppo che riceve mescola la `posizione` come nella partita.

**Interfaces:**
- Consumes: `con_su_misura` del Task 0.
- Produces: nessun nome pubblico nuovo. `pub(crate) fn spostamenti_lavoro(quote: &[f64], dati: &DatiElettorato, regione: usize, delta: f64) -> Vec<(usize, usize, f64)>` restituisce le masse da spostare (`da`, `a`, `massa`) senza applicarle. `Elettorato::lavoro` le applica con `sposta`/`ripartisci`, che mescolano posizione, ancora, giudizio e voto precedente; `su_misura` le applica sui `DatoGruppo`, mescolando la sola posizione (scelta 22). Nella partita, `ripartisci` pesa coi dati (`composizione.rs:56-64`): la funzione pura riceve quelle quote, così `lavoro` resta identico nei risultati.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn la_disoccupazione_sposta_naspi_e_gruppi() {
    // disoccupazione 8,0 contro 5,6: spesa_sussidi × 8,0/5,6 (tolleranza 1e-9);
    // Partita::nuova sui dati nuovi: sussidio_per_disoccupato uguale alla base entro 1 €;
    // quota dei DisoccupatoInattivo di ogni regione più alta della base.
}

#[test]
fn l_inflazione_sposta_il_tasso_bce() {
    // inflazione 6,0 contro 4,2:
    // tasso_bce = base + (1 + taylor_inflazione) · quota_italia_area_euro · 1,8
}

#[test]
fn il_debito_muove_interessi_e_spread() {
    // debito_pil 150: con Economia::iniziale spread e interessi più alti della base
}
```

- [ ] **Step 2: Verifica che falliscano**

Run: `cargo test -p sim-core --test su_misura`
Expected: FAIL sui primi due.

- [ ] **Step 3: Implementa i derivati nell'ordine della scelta 10**

`Elettorato::lavoro` chiama `sposta_lavoro` e resta identico nei risultati: lo verifica `tests/elettorato.rs` invariato.

- [ ] **Step 4: Verifica.** Stessi tre comandi del Task 0. Expected: PASS.

- [ ] **Step 5: Commit** `feat(core): derivati dei cursori assoluti (piano 10, Task 1)`

### Task 2: Ricchezza e orientamento

**Files:**
- Modify: `crates/sim-core/src/su_misura.rs`
- Test: `crates/sim-core/tests/su_misura.rs`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:39` — cosa scalare e come far mordere le soglie in euro
- `docs/ricerca/sintesi/cursori-dello-scenario.md:68` — spostamento frazionario dell'istogramma

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `Elettorato::aliquota_media_irpef` (`elettorato/mod.rs:412`).
- Produces: nessun nome pubblico nuovo.

**Ricchezza x %, k = 1 + x/100:**
- scalano per k `pil_trimestrale`, `spesa_pensioni`, `spesa_sussidi`, `altre_prestazioni`, e `primario` e `trasferimenti` dei 360 gruppi;
- restano fermi scaglioni, pensione minima, gradini delle disposizioni e manopole (`fondo_riserva` compreso);
- `irpef_pil` scala per `t(k)/t(1)`, con `t` = `aliquota_media_irpef` sugli scaglioni fermi, calcolata su due `Elettorato::iniziale` costruiti sui dati scalati e su quelli di base;
- le aliquote di fascia di `elettorato.toml` scalano allo stesso modo, ciascuna col rapporto delle aliquote medie IRPEF dei gruppi della sua fascia.

**Orientamento x ∈ [−1, 1]:** su ogni istogramma a 7 punti, per x ≥ 0 il punto k tiene (1−x) della massa e ne passa x al punto k+1; per x < 0, simmetrico verso k−1. La massa che uscirebbe oltre 7 o sotto 1 resta al bordo, e la somma resta 1.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn la_ricchezza_scala_le_spese_in_euro_e_fa_mordere_gli_scaglioni() {
    // ricchezza +10: pil_trimestrale e spesa_pensioni × 1,1 (entro 1e-9); scaglioni invariati;
    // irpef_pil (quota del PIL) più alto della base: con gli scaglioni fermi t(1,1) > t(1)
    assert!(nuovo.irpef_pil.valore > base.irpef_pil.valore);
}

#[test]
fn l_orientamento_sposta_la_media_e_conserva_la_massa() {
    // orientamento +0,5: per ogni gruppo somma = 1 entro 1e-12;
    // media nazionale pesata (elettori × quota) +0,473 ± 0,005 (cursori-dello-scenario:69)
}
```

- [ ] **Step 2: Verifica che falliscano.** Run: `cargo test -p sim-core --test su_misura`. Expected: FAIL.

- [ ] **Step 3: Implementa** le due trasformazioni nell'ordine della scelta 10.

- [ ] **Step 4: Verifica.** Stessi tre comandi. Expected: PASS.

- [ ] **Step 5: Commit** `feat(core): cursori di ricchezza e orientamento (piano 10, Task 2)`

### Task 3: Età della popolazione

**Files:**
- Modify: `crates/sim-core/src/su_misura.rs`, `crates/sim-core/src/elettorato/composizione.rs:149-190` (spostamenti verso i pensionati estratti come nel Task 1)
- Test: `crates/sim-core/tests/su_misura.rs`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:60` — inclinazione esponenziale di popolazione stabile
- `docs/ricerca/sintesi/cursori-dello-scenario.md:62` — pensionati, spesa e quote dei gruppi derivati

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true. Nota dell'architect: estrarre la logica di `composizione.rs:149-190` significa modificare `elettorato/composizione.rs`, che manca nei Files. Quella logica è il flusso annuale verso la pensione (`tasso` dalle annate di `e.pensioni`), non uno spostamento dato un Δ: l'estrazione segue la forma scelta nel Task 1, mescolamento compreso.

**Interfaces:**
- Consumes: `Struttura::valida_piramide` (`struttura.rs:202`); età di pensione dalle leve di partenza.
- Produces: nessun nome pubblico nuovo.

**Metodo:**
- La piramide diventa c'(a) ∝ c(a)·e^(−k·a), rinormalizzata al totale di base.
- k si trova per bisezione in [−0,02, 0,02], fino a mediana = base ± N entro 0,01 anni.
- Con `r` = quota oltre l'età di pensione, nuova su base: `pensionati` e `spesa_pensioni` scalano per `r`, con importo medio fermo.
- Le quote dei gruppi `Pensionato` di ogni regione si spostano con la logica di `composizione.rs:149-190`, estratta come nel Task 1.

L'ordine con la ricchezza è indifferente, perché i prodotti commutano.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn l_eta_centra_la_mediana_e_conserva_il_totale() {
    // eta_mediana +2: mediana 51,34 ± 0,01; totale invariato entro 1 persona;
    // quota 67+ 24,6% ± 0,2 (cursori-dello-scenario:61); pensionati e spesa_pensioni più alti
}

#[test]
fn eta_e_ricchezza_commutano() {
    // eta +2 con ricchezza +10 dà la stessa spesa_pensioni dell'ordine inverso entro 1e-6
}
```

- [ ] **Step 2: Verifica che falliscano.** Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi. Expected: PASS.

- [ ] **Step 5: Commit** `feat(core): cursore dell'età della popolazione (piano 10, Task 3)`

### Task 4: Menu della legge elettorale e delle disposizioni

**Files:**
- Modify: `crates/sim-core/src/su_misura.rs`, `crates/sim-core/src/dati.rs:391-410` e `:534-546` (rigenerare le elettorali), `crates/sim-core/src/disposizioni/mod.rs:46` e `:266-295` (etichetta come parametro), `crates/sim-core/src/partita.rs:128-240` (`costruisci` applica le disposizioni ordinarie)
- Test: `crates/sim-core/tests/su_misura.rs`, `crates/sim-core/tests/eredita.rs`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:75` — legge elettorale costante, precedente fisso, gradini senza posto
- `docs/ricerca/sintesi/cursori-dello-scenario.md:105` — `Disposizioni::cambia` al turno zero e cose da sistemare

**Dispatch:** `rust-engineer` · dopo il Task 3 · skill-gate: true. Nota dell'architect: `Disposizioni::cambia` vuole indici (`d: usize, g: usize`, `stato.rs:112-123`), quindi id e gradino si convertono con `DatoDisposizione::gradino`. In `costruisci`, `grandezze` è un `let` immutabile e `Disposizioni::iniziali` arriva dopo `Elettorato::iniziale` e `Informazione::nuova` (`partita.rs:131-150` a `95f18cb`): al turno zero redditi e sondaggio vedono ancora i gradini di base, coerente col ritardo normale della scelta 4. `cambia` deve stare prima di `Ministri::iniziali`, che legge `&disposizioni`.

**Interfaces:**
- Consumes: `Disposizioni::cambia` (`disposizioni/stato.rs:112-123`); `disposizioni::elettorali` (`disposizioni/mod.rs`); `Errore::SuMisura`.
- Produces:
  - `pub const ETICHETTA_GOVERNO_USCENTE: &str = "Legge del governo uscente (2026)"` in `su_misura.rs`;
  - `disposizioni::elettorali(menu, iniziale, precedente, legge: &str)`, con il parametro nuovo.

**Legge elettorale diversa da quella di base:**
- `scenario.legge_elettorale` prende il nome nuovo;
- `legge_elettorale_precedente` prende i valori dell'impostazione di base (spec riga 281: abrogarla riporta alla 180);
- le nove disposizioni elettorali si rigenerano con l'etichetta del governo uscente;
- i gradini elettorali di `SuMisura.disposizioni` diventano la loro `partenza`, il che è sicuro per le elettorali (saldo, leve ed effetti nulli: `disposizioni/mod.rs:286-294`).

**Disposizioni ordinarie:** in `costruisci`, dopo `Disposizioni::iniziali`, per ognuna di `dati.su_misura().disposizioni` con gradino diverso dal vigente si chiama `Disposizioni::cambia(dati, d, g, 0, ETICHETTA_GOVERNO_USCENTE, &mut grandezze.effetti)`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn il_proporzionale_dell_editor_ha_la_180_come_precedente() {
    // legge_elettorale = "proporzionale_3": la disposizione della soglia in vigore è quella
    // del proporzionale, il precedente è la 180, l'etichetta è ETICHETTA_GOVERNO_USCENTE
}

#[test]
fn la_disposizione_dell_editor_ha_effetto() {
    // cuneo_fiscale = "0" contro "7": dopo Partita::fonda, in_vigore "0", precedente "7",
    // legge ETICHETTA_GOVERNO_USCENTE, Disposizioni::saldo diverso da zero
}
```

In `tests/eredita.rs`, `la_legge_del_governo_uscente_non_e_del_giocatore`: dopo l'abbandono, la disposizione cambiata dall'editor non sta in `leggi_in_vigore`.

- [ ] **Step 2: Verifica che falliscano.** Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi. Expected: PASS; i test elettorali esistenti restano verdi con `LEGGE_ELETTORALE` passata esplicitamente.

- [ ] **Step 5: Commit** `feat(core): menu della legge elettorale e delle disposizioni (piano 10, Task 4)`

### Task 5: Riepilogo dal vivo, avvisi e CLI

**Files:**
- Create: `crates/sim-core/src/riepilogo.rs`
- Modify: `crates/sim-core/src/informazione/sondaggio.rs` (`atteso`), `crates/sim-core/src/legislatura/fondazione.rs` (TS export di `Fondazione`), `crates/sim-core/src/lib.rs`, `crates/sim-core/src/partita.rs` (`Quadro` sul sondaggio atteso, accanto a `quadro`), `crates/sim-cli/src/main.rs` (`--scenario <file>`), `Cargo.toml` della radice (profilo dev di `sim-core`)
- Test: `crates/sim-core/tests/riepilogo.rs`, test esistenti di `sim-cli`

**Ricerca:**
- `docs/ricerca/sintesi/cursori-dello-scenario.md:146` — cosa serve al riepilogo e da dove si prende
- `docs/ricerca/sintesi/cursori-dello-scenario.md:169` — costo misurato in release e in debug

**Dispatch:** `rust-engineer` · ultimo della PR A · skill-gate: true. Nota dell'architect: il `Quadro` di `Partita` è privato (`partita.rs:2007-2024`). Per costruirlo sul sondaggio atteso serve un `pub(crate) fn` in `partita.rs`, che manca nei Files, per esempio un `quadro` che prende il `&Sondaggio`. `Sondaggio::atteso` si ricava da `rileva` (`sondaggio.rs:75-181`) togliendo `caso`, `bias` ed `effetti`; anche `approvazione` resta senza estrazione. Per «i seggi della coalizione» `Quadro::seggi` ripartisce secondo la `Dichiarazione` (`coalizione.rs:141-162`). Con quella vuota il giocatore corre da solo, quindi il riepilogo passa una `Dichiarazione` con la coalizione e lo dichiara nella PR. Su main ci sono `partito_nuovo` `pub(crate)` e `quote_fra_i_votanti`: si riusano. Le campagne IA del turno zero (8b) non entrano nel riepilogo, e questo va scritto fra i limiti.

**Interfaces:**
- Consumes: `Partita::costruisci`, `partito_nuovo`, `posizioni` (`fondazione.rs:45`); `Quadro` (`legislatura/coalizione.rs:122`); `DatiGioco::con_su_misura`.
- Produces:
  - `pub(crate) fn Sondaggio::atteso(dati, el, vero, presidente) -> Sondaggio`: un istituto, quote = intenzioni vere, incroci veri, nessun bias, nessuna estrazione;
  - `pub fn riepilogo(dati: &DatiGioco) -> RiepilogoScenario`, deterministico e senza seme;
  - `pub struct RiepilogoScenario { pub spread_pb: i32, pub interessi_pil: f64, pub disavanzo_pil: f64, pub saldo_disposizioni_pil: f64, pub posizioni: Vec<PosizioneAttesa>, pub avvisi: Vec<Avviso> }`;
  - `pub struct PosizioneAttesa { pub posizione: f64, pub quota_attesa: f64, pub coalizione: Vec<String>, pub quota_coalizione: f64, pub seggi_camera: u32, pub seggi_senato: u32, pub maggioranza: bool }`;
  - `#[serde(tag = "tipo")] pub enum Avviso { SenzaMaggioranza { posizioni: Vec<f64> } }`.

  Tutto con TS export.

Definizioni:
- Le quote sono fra i votanti.
- `coalizione` è il giocatore più i partiti IA che `Quadro::disponibilita` dà disponibili con la dichiarazione vuota, sul sondaggio atteso (scelta 14), in ordine d'indice; ne fanno parte anche i loro id.
- I seggi sono quelli della coalizione col riparto atteso: `Quadro::seggi` con una `Dichiarazione` che porta i membri IA di `coalizione`, così il premio va alla coalizione dell'avviso e non al giocatore da solo. Il `Quadro` sul sondaggio atteso si prende con un `pub(crate) fn` nuovo in `partita.rs`, accanto a `quadro`; la quota attesa con `quote_fra_i_votanti` di main.
- `maggioranza` vale se la coalizione supera la metà dei seggi in entrambe le Camere.
- `saldo_disposizioni_pil` = `Disposizioni::saldo` dopo le disposizioni dell'editor, perché il disavanzo del turno zero non lo contiene ancora.

Testo dell'avviso (per la revisione di Mario): «Da {posizioni} nessuna coalizione con te ha la maggioranza attesa in entrambe le Camere, prima della campagna».

`sim-cli --scenario <file.toml>` applica `SuMisura::da_toml` ai dati prima di giocare. Un errore esce con codice 1 e il messaggio.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn il_riepilogo_della_base_e_vincibile_ovunque() {
    let r = riepilogo(&comune::dati_base());
    assert_eq!(r.posizioni.len(), 6);
    assert!(r.avvisi.is_empty(), "spec riga 285: vincibilità della base");
    assert_eq!(r.spread_pb, /* spread della partita di prova al turno 0, arrotondato */);
}

#[test]
fn il_riepilogo_non_ha_rumore() {
    // due chiamate uguali, e la quota attesa = intenzioni vere della partita fondata
    // a quella posizione, pesate sui votanti, entro 1e-12
}

#[test]
fn un_elettorato_spostato_accende_l_avviso() {
    // orientamento −1 e legge "proporzionale_puro": almeno una posizione in avvisi
    // (se non basta, il test sceglie il caso estremo dentro gli intervalli e lo dichiara)
}
```

In `sim-cli`: `scenario_ostile_esce_con_errore` (file con `pil = 1` in `[cursori]`, codice d'uscita 1, il messaggio contiene «pil»).

- [ ] **Step 2: Verifica che falliscano.** Expected: FAIL.

- [ ] **Step 3: Implementa.** Una partita per posizione, come `anteprima_fondazione`; il commento `ponytail:` nomina la via leggera di `cursori-dello-scenario.md:171-175`. Nel `Cargo.toml` della radice:

```toml
[profile.dev.package.sim-core]
opt-level = 2
```

- [ ] **Step 4: Verifica.** Stessi tre comandi, più `npm --prefix app run check`. Expected: PASS. Nella PR: il tempo di `riepilogo` sui dati base in release e in debug, con comando, branch e HEAD.

- [ ] **Step 5: Commit** `feat(core): riepilogo dal vivo dello scenario (piano 10, Task 5)`

---

## PR B · Salvataggi nel core (Task 6-8)

### Task 6: Serie, riavvolgimenti, testata

**Files:**
- Create: `crates/sim-core/src/salvataggi.rs`
- Modify: `crates/sim-core/src/partita.rs:85-115` (campi `serie`, `riavvolgimenti`), `:240` circa (primo punto alla fine di `costruisci`), `:279` (punto in fondo a `chiudi_turno`), `:1815-1845` (`eredita_di`), `:3165-3172` (`carica`), `crates/sim-core/src/legislatura/eredita.rs:17-25`
- Test: `crates/sim-core/tests/salvataggi.rs`, `crates/sim-core/tests/eredita.rs`

**Ricerca:**
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:129` — il contatore non può vivere solo nello snapshot
- `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md:65` — testata stabile e `unwrap_or(0)` da distinguere

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true. Nota dell'architect: le righe sono di `95f18cb`; su main `coalizione_del_giocatore` sta a `partita.rs:2589`, `chiudi_turno` a `:346` (un solo `return`, all'inizio, quindi il punto in fondo si raggiunge sempre) e `carica` a `:3540`. `coalizione_del_giocatore` è privato: il punto della serie si calcola in `partita.rs`, mentre in `salvataggi.rs` stanno solo i tipi. Al turno zero della partita vera le Camere sono sciolte, coi seggi a zero e `legislatura.totali = (0, 0)`: va definito da dove viene «la metà più uno», dai totali dei dati o da quelli dell'ultimo voto, e cosa vale `maggioranza_*` a Camere sciolte. Con l'8b fuso la coalizione può contenere partiti sciolti. Casi limite da `git log`: sotto `carica` vanno anche serie e riavvolgimenti, cioè turni crescenti da 0 al turno corrente e valori finiti.

**Interfaces:**
- Consumes: `Partita::coalizione_del_giocatore` (`partita.rs:2247`), `informazione.sondaggio`, `parlamento`, `economia.spread`.
- Produces, in `salvataggi.rs`:
  - `pub struct PuntoSerie { pub turno: u32, pub data: Trimestre, pub consenso: f64, pub maggioranza_camera: i32, pub maggioranza_senato: i32, pub spread_pb: i32 }`, dove `consenso` è la quota fra i votanti della coalizione del giocatore nel sondaggio pubblico (media degli istituti) e `maggioranza_*` sono i seggi della coalizione meno la metà più uno della Camera;
  - `pub struct Testata { pub nome: String, pub scenario: String, pub turno: u32, pub data: Trimestre, pub versione_schema: u32, pub finita: bool }`, dove `nome` è il nome del partito del giocatore.
- Produces, in `Partita`:
  - `pub fn serie(&self) -> &[PuntoSerie]`
  - `pub fn testata(&self) -> Testata`
  - `pub fn imposta_riavvolgimenti(&mut self, n: u32)`
  - `pub fn riavvolgimenti(&self) -> u32`
- Produces, altrove: `Eredita.riavvolgimenti: u32`.

`carica` tratta un `versione_schema` mancante o non intero come `SalvataggioIlleggibile("manca la versione dello schema")`, non come schema 0.

Il punto della serie si aggiunge all'apertura di ogni turno: in fondo a `costruisci` (turno 0) e in fondo a `chiudi_turno`, se la partita non è finita.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn un_punto_per_apertura() {
    let mut p = comune::fondata(3.5);
    assert_eq!(p.serie().len(), 1);
    for _ in 0..3 { p.chiudi_turno(); }
    assert_eq!(p.serie().iter().map(|s| s.turno).collect::<Vec<_>>(), [0, 1, 2, 3]);
}

#[test]
fn i_riavvolgimenti_entrano_nell_eredita() {
    let mut p = comune::fondata(3.5);
    p.imposta_riavvolgimenti(3);
    assert_eq!(p.abbandona().unwrap().riavvolgimenti, 3);
}

#[test]
fn senza_versione_e_illeggibile() {
    // salva, togli "versione_schema" col serde_json::Value, carica:
    // SalvataggioIlleggibile il cui testo contiene "versione"
}

#[test]
fn la_testata_regge_un_salvataggio() {
    // testata prima di salva == testata dopo carica; nome = nome del partito fondato
}
```

- [ ] **Step 2: Verifica che falliscano.** Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi, più `npm --prefix app run check`. Expected: PASS.

- [ ] **Step 5: Commit** `feat(core): serie degli indicatori, riavvolgimenti e testata (piano 10, Task 6)`

### Task 7: La voce dell'albo

**Files:**
- Modify: `crates/sim-core/src/salvataggi.rs`, `crates/sim-core/src/partita.rs` (`voce_albo`)
- Test: `crates/sim-core/tests/salvataggi.rs`

**Ricerca:**
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:92` — albo tollerante senza `serde(default)`
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:94` — regole del formato: `formato_albo`, campi solo aggiunti, valori copiati per esteso, niente enum chiuse
- `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md:66` — l'albo deve essere elencabile da solo

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true. Nota dell'architect: aggiunta la riga `:94`, perché è quella che governa la forma di `VoceAlbo`; la `:92` copre solo la lettura. `VoceAlbo.indicatori` riusa `ConfrontoIndicatore` (`eredita.rs:6-14`), che è un tipo dell'eredità: se un giorno cambia, cambia anche il formato dell'albo. Una copia sua nell'albo costa quattro campi; la scelta va scritta nella PR. Nel test `serie.len() == turni + 1`, `turni` va definito: con `Fine::SenzaSeggi` la chiusura del voto non aggiunge il punto. I campi di `VoceAlbo` che coincidono con l'eredità (`anni_al_governo`, `leggi_in_vigore`, `indicatori`) vengono da `eredita_di`, mai ricalcolati: è la classe di `fix(core): eredità su debito dell'anno mobile e turni da premier`.

**Interfaces:**
- Consumes: `Eredita`, `PuntoSerie`, `Legislatura.elezioni` (`EsitoElezioni`, `legislatura/voto.rs:27-33`).
- Produces:
  - `pub const FORMATO_ALBO: u32 = 1`
  - `pub struct VoceAlbo { pub formato_albo: u32, pub nome: String, pub scenario: String, pub inizio: Trimestre, pub fine: Trimestre, pub turni: u32, pub esito: String, pub anni_al_governo: f64, pub leggi_in_vigore: Vec<String>, pub indicatori: Vec<ConfrontoIndicatore>, pub serie: Vec<PuntoSerie>, pub elezioni: Vec<ElezioneAlbo>, pub riavvolgimenti: u32 }`
  - `pub struct ElezioneAlbo { pub data: Trimestre, pub affluenza: f64, pub voti: Vec<(String, f64)>, pub seggi_giocatore: (u32, u32) }`: nome del partito e quota fra i votanti, copiati per esteso
  - `Partita::voce_albo(&self) -> Option<VoceAlbo>`, `None` se la partita non è finita

`esito` è testo: «Senza seggi» o «Abbandono», mai l'enum `Fine`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn la_voce_dell_albo_regge_un_formato_futuro() {
    let mut p = comune::fondata(3.5);
    p.chiudi_turno();
    p.abbandona().unwrap();
    let v = p.voce_albo().unwrap();
    let mut j = serde_json::to_value(&v).unwrap();
    j["campo_del_futuro"] = serde_json::json!({"a": 1});
    assert_eq!(serde_json::from_value::<VoceAlbo>(j).unwrap(), v);
}

#[test]
fn l_albo_porta_elezioni_e_serie() {
    // partita fondata giocata oltre il voto del turno zero, poi abbandono:
    // elezioni.len() == legislatura().elezioni.len(); serie.len() == aperture giocate
    // (turno 0 compreso; la chiusura che finisce la partita non aggiunge il punto);
    // eredità dell'albo uguale a Partita::eredita, non ricalcolata
}

#[test]
fn nessuna_voce_a_partita_in_corso() {
    assert!(comune::fondata(3.5).voce_albo().is_none());
}
```

- [ ] **Step 2: Verifica che falliscano.** Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi, più `npm --prefix app run check`. Expected: PASS.

- [ ] **Step 5: Commit** `feat(core): voce dell'albo delle partite (piano 10, Task 7)`

### Task 8: Completezza dello snapshot e stesso caso

**Files:**
- Test: `crates/sim-core/tests/salvataggi.rs`

**Ricerca:**
- `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md:63` — test di completezza dello snapshot
- `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md:62` — stessa azione, stesso esito

**Dispatch:** `rust-engineer` · ultimo della PR B · skill-gate: true. Solo test: se uno fallisce, il task diventa un fix con la causa nella PR. Nota dell'architect: i flussi del caso sono per sottosistema e stanno nello snapshot (`caso.rs:7-18`), quindi il secondo test misura la riproducibilità, non la introduce. Con l'8b fuso, i 40 turni del primo test possono passare per scissioni e proteste, cioè per `partiti.len()` che cambia. La PR dice se almeno un seme ne passa una: se nessuno ci passa, un seme che lo fa.

**Interfaces:**
- Consumes: `Partita::salva`, `Partita::carica`, `Partita::comanda`, `comune::fondata`, `comune::presenta`.
- Produces: nessuna API.

- [ ] **Step 1: Scrivi i test**

```rust
#[test]
fn ricaricare_a_ogni_apertura_non_cambia_la_partita() {
    // semi 1..=5, 40 turni: partita A chiusa di fila; partita B salvata e ricaricata a ogni
    // apertura; a ogni turno A.salva() == B.salva()
}

#[test]
fn stesse_scelte_stesso_esito_dopo_il_ricaricamento() {
    // partita vera al primo turno di governo (come i test di presenta); snapshot s; comandi c1..cn
    // fra cui una legge con franchi tiratori e un decreto; chiudi. Poi carica(s), stessi comandi
    // nello stesso ordine, chiudi: salva() uguali
}

#[test]
fn la_bozza_a_meta_turno_riprende_le_scelte() {
    // comandi a metà turno, salva (la bozza), carica: calendario e bilancio presentato uguali,
    // e chiudi_turno dà lo stesso esito della partita mai salvata
}
```

- [ ] **Step 2: Esegui.** Run: `cargo test -p sim-core --test salvataggi`. Expected: PASS. Se un test fallisce, la causa sta in uno stato fuori da `Partita` o saltato da serde: si corregge, e la causa va nella PR.

- [ ] **Step 3: Verifica.** Stessi tre comandi. Expected: PASS.

- [ ] **Step 4: Commit** `test(core): completezza dello snapshot e stesso caso (piano 10, Task 8)`

---

## PR C · Guscio (Task 9-10)

### Task 9: Archivio su disco

**Files:**
- Create: `app/src-tauri/src/archivio.rs`
- Modify: `app/src-tauri/Cargo.toml` (`flate2 = "=1.1.10"`), `crates/sim-core/src/salvataggi.rs` (`IndicePartita`)
- Test: in `archivio.rs`, `#[cfg(test)]`, con una cartella temporanea `std::env::temp_dir()/sim-<nome del test>-<pid>` cancellata a fine test

**Ricerca:**
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:51` — scrittura atomica su APFS, ricetta std
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:97` — una cartella per partita
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:115` — regola di ripresa: la bozza vale solo se è dell'ultimo snapshot
- `docs/ricerca/sistemi/salvataggi-e-riavvolgimento.md:64` — bozza separata, snapshot mai riscritto

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true. Nota dell'architect: `:47` era l'intestazione della sezione, ora punta alla ricetta (`:51`). Aggiunta la `:115`, perché `riprendi` («la bozza se si legge») se ne discosta. `snapshot` scrive `turno-(K+1)` e poi toglie la bozza: se l'app cade fra i due passi resta una bozza del turno K leggibile, e `riprendi` riaprirebbe il turno K mentre esiste `turno-(K+1)`. Regola della ricerca: la bozza si apre solo se il suo `turno` coincide con l'ultimo `turno-K`, altrimenti si apre `turno-K` e la bozza si cancella. La correzione tocca la sola descrizione di `riprendi` e aggiunge un caso a `bozza_rovinata_ripiega_sullo_snapshot`, quello della bozza vecchia accanto a uno snapshot più nuovo. La decide Mario in review: è una scelta del piano. `IndicePartita` è tollerante come l'albo (`persistenza-dei-salvataggi.md:106`), quindi la regola di `formato_albo` (campi solo aggiunti, i nuovi `Option`) vale anche per `formato_indice` e per `Testata`, che ci sta dentro.

**Interfaces:**
- Consumes: `Partita::{salva, carica, testata, voce_albo, imposta_riavvolgimenti, riavvolgimenti}`, `VERSIONE_SCHEMA`, `SuMisura::{da_toml, in_toml}`.
- Produces, in `sim_core::salvataggi` (tipo puro, con TS export):
  - `pub struct IndicePartita { pub formato_indice: u32, pub id: String, pub testata: Testata, pub ultimo_salvataggio: u64, pub riavvolgimenti: u32 }`, con `ultimo_salvataggio` in secondi Unix
  - `pub struct VocePartita { pub indice: IndicePartita, pub apribile: bool, pub motivo: Option<String> }`
  - `IndicePartita` e `Testata` seguono la regola dell'albo (scelta 16): `formato_indice = 1`, campi solo aggiunti, i nuovi come `Option`, così l'elenco legge anche gli indici scritti da versioni diverse
- Produces, in `archivio.rs` (`pub(crate)`), tutto con errori `String` in italiano che nominano il file:
  - `struct Archivio { radice: PathBuf }`
  - `Archivio::nuovo(radice: PathBuf) -> Result<Archivio, String>`: `create_dir_all` di `partite/`, `albo/`, `scenari/`
  - `fn crea(&self, p: &Partita, id: &str) -> Result<(), String>`: cartella, `turno-000.json.gz`, indice
  - `fn snapshot(&self, id: &str, p: &Partita) -> Result<(), String>`: `turno-NNN.json.gz` del turno, toglie la bozza, aggiorna l'indice
  - `fn bozza(&self, id: &str, p: &Partita) -> Result<(), String>`
  - `fn riprendi(&self, id: &str) -> Result<Partita, String>`: la bozza se si legge e il suo turno è quello dell'ultimo `turno-K` leggibile (scelta 21), altrimenti quello snapshot; sempre `imposta_riavvolgimenti(indice.riavvolgimenti)`
  - `fn riavvolgi(&self, id: &str, turno: u32) -> Result<Partita, String>`: richiede `turno-N` esistente e N ≤ turno dell'indice; cancella `turno-M` per M > N e la bozza, poi incrementa e scrive l'indice
  - `fn elenca(&self) -> Vec<VocePartita>`: legge solo gli indici, ignora `.tmp` e cartelle senza indice; `apribile = versione_schema == VERSIONE_SCHEMA`, con `motivo` che nomina le due versioni
  - `fn chiudi_nell_albo(&self, id: &str, v: &VoceAlbo) -> Result<(), String>`: `albo/<id>.json` in chiaro, poi cancella la cartella della partita
  - `fn albo(&self) -> Vec<VoceAlbo>`: salta, senza fermarsi, i file che non si leggono
  - `fn scenari(&self) -> Vec<String>`
  - `fn salva_scenario(&self, s: &SuMisura, base: &DatiGioco) -> Result<(), String>`
  - `fn carica_scenario(&self, nome: &str, base: &DatiGioco) -> Result<SuMisura, String>`
  - `fn valida_nome(nome: &str) -> Result<(), String>`: lunghezza 1-60, niente `/`, `\`, `..`, punto iniziale, caratteri di controllo. La usano gli id di partita (16 cifre esadecimali) e i nomi di scenario.

Scrittura atomica: file `.tmp` nella stessa cartella, `write_all`, `sync_all`, `rename`, poi `sync_all` della cartella. gzip con `flate2::write::GzEncoder` e `Compression::default()`; lettura con `GzDecoder`. Testi d'errore nella PR.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn crea_snapshot_riprendi() { /* fonda, crea, 2 chiusure con snapshot: riprendi == partita al turno 2 */ }

#[test]
fn riavvolgi_cancella_il_futuro_e_conta() {
    // 3 snapshot, riavvolgi a 1: esistono turno-000 e turno-001, niente bozza,
    // indice.riavvolgimenti == 1, partita.riavvolgimenti() == 1; riavvolgi a 5: errore
}

#[test]
fn bozza_rovinata_ripiega_sullo_snapshot() {
    // bozza troncata a metà e un turno-002.json.gz.tmp orfano: riprendi dà lo snapshot del turno,
    // elenca mostra una sola partita
}

#[test]
fn schema_diverso_elencato_e_intatto() {
    // indice con versione_schema 10: apribile false, motivo con "10" e "11";
    // riprendi errore; byte di tutti i file uguali a prima
}

#[test]
fn nome_fuori_cartella_rifiutato() {
    for n in ["../x", "a/b", "a\\b", ".nascosto", "", &"x".repeat(61), "a\u{7}b"] {
        assert!(Archivio::valida_nome(n).is_err(), "{n:?}");
    }
}

#[test]
fn albo_e_cartella_cancellata() { /* abbandona, chiudi_nell_albo: albo() ha 1 voce, cartella sparita */ }

#[test]
fn scenario_andata_e_ritorno() { /* salva_scenario, scenari() lo elenca, carica_scenario == s */ }
```

- [ ] **Step 2: Verifica che falliscano.** Run: `cargo test -p app archivio`. Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi. Expected: PASS. In `Cargo.lock` cambia solo la lista delle dipendenze di `app` (`git diff Cargo.lock`).

- [ ] **Step 5: Commit** `feat(app): archivio delle partite su disco (piano 10, Task 9)`

### Task 10: Comandi Tauri e bozza all'uscita

**Files:**
- Modify: `app/src-tauri/src/lib.rs` (stato, comandi nuovi, `run` con `build().run(callback)`)
- Test: in `lib.rs`, `#[cfg(test)]`, sulle funzioni sotto i comandi, come quelle di oggi (`lib.rs:114-165`)

**Ricerca:**
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:40` — Cmd+Q dà solo `RunEvent::Exit`
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:124` — `Builder::build(...)?.run(callback)`
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:44` — nell'handler di `Exit` il `Mutex` si prende anche avvelenato (`PoisonError::into_inner`)

**Dispatch:** `rust-engineer` · ultimo della PR C · skill-gate: true. Nota dell'architect: aggiunta la riga `:44`. `app_data_dir` c'è solo dopo `build`, quindi l'`Archivio` entra in `manage` dentro `.setup(|app| …)`, non al posto dello `Stato::default()` di oggi (`lib.rs:101-112`). `DatiBase` non è definito altrove: se sono i `DatiGioco` di base, letti una volta e mai scritti, bastano `manage(DatiGioco)` e `State<'_, DatiGioco>`, senza `Mutex` (ladder: niente lock su un valore che non cambia).

**Interfaces:**
- Consumes: `Archivio` del Task 9; `riepilogo`, `SuMisura`, `Fondazione`, `Partita::{fonda, comanda, chiudi_turno, abbandona, voce_albo, vista}`.
- Produces: lo stato diventa `Mutex<Option<InCorso>>`, con `struct InCorso { id: Option<String>, partita: Partita }`; `id` vale `None` per la partita di prova. In più, `State<DatiGioco>` coi dati di base parsati una volta (immutabili, senza `Mutex`) e `State<Archivio>`, entrambi creati in `.setup`, perché `app_data_dir` e `resource_dir` esistono solo dopo `build`.

Comandi Tauri nuovi (tutti `async`, `Result<_, String>`):

| Comando | Firma | Effetto |
|---|---|---|
| `partite` | `() -> Vec<VocePartita>` | |
| `fonda_partita` | `(scenario: Option<SuMisura>, fondazione: Fondazione) -> Vista` | id = seme in esadecimale a 16 cifre; `Archivio::crea` |
| `riprendi` | `(id: String) -> Vista` | |
| `comanda` | `(comando: Comando) -> Vista` | con `id`, `Archivio::bozza` dopo ogni comando accettato |
| `riavvolgi` | `(turno: u32) -> Vista` | |
| `abbandona` | `() -> Eredita` | `voce_albo` → `chiudi_nell_albo` |
| `conferma_eredita` | `() -> ()` | solo a partita finita con bozza d'eredità |
| `albo` | `() -> Vec<VoceAlbo>` | |
| `scenari` | `() -> Vec<String>` | |
| `salva_scenario` | `(scenario: SuMisura) -> ()` | |
| `carica_scenario` | `(nome: String) -> SuMisura` | |
| `riepilogo_scenario` | `(scenario: Option<SuMisura>) -> RiepilogoScenario` | |

`chiudi_turno` esistente: con `id`, `Archivio::snapshot` dopo la chiusura. `nuova_partita` resta com'è, partita di prova senza salvataggi.

Radice: `app.path().app_data_dir()`, più `sviluppo/` se `cfg!(debug_assertions)`.

`RunEvent::Exit`: con una partita in corso con `id` e non finita, `Archivio::bozza`. Un errore va su `eprintln!` e non blocca l'uscita.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
#[test]
fn fonda_comanda_chiudi_riprendi() {
    // su un Archivio temporaneo: fonda, un comando, la bozza esiste; chiudi, la bozza non c'è
    // e c'è turno-001; riprendi dà la stessa Vista
}

#[test]
fn abbandona_scrive_l_albo_e_libera_lo_stato() { /* albo() 1 voce, partite() vuoto, stato None */ }

#[test]
fn riepilogo_della_base_senza_scenario() { /* 6 posizioni */ }
```

- [ ] **Step 2: Verifica che falliscano.** Run: `cargo test -p app`. Expected: FAIL.

- [ ] **Step 3: Implementa.**

- [ ] **Step 4: Verifica.** Stessi tre comandi, più `npm --prefix app run check`. Expected: PASS.

- [ ] **Step 5: Prova a mano, da riportare nella PR**

`npm --prefix app run tauri dev`, poi un comando dalla console del webview: `invoke("fonda_partita", …)`, poi `invoke("comanda", …)`. Poi Cmd+Q. La bozza dev'essere in `~/Library/Application Support/it.maeurong.simulatorepolitico/sviluppo/partite/<id>/bozza.json.gz`, con l'orario del Cmd+Q. Si controlla anche che l'`Info.plist` costruito non abbia `NSSupportsSuddenTermination` (`persistenza-dei-salvataggi.md:131`). Se la bozza non c'è, la PR lo dice e lascia il salvataggio a ogni comando come difesa.

- [ ] **Step 6: Commit** `feat(app): comandi delle partite, degli scenari e dell'albo (piano 10, Task 10)`

---

## Rimandato

- **Riepilogo leggero:** un'economia per stato e, per ogni posizione, solo elettorato e voto (`cursori-dello-scenario.md:171-175`). Serve se i 30 ms in release pesano nella schermata.
- **Avviso «situazione o crisi accesa al turno zero»:** piano 9 (decisione 2).
- **`anteprima_fondazione` come comando Tauri**, sul sondaggio rumoroso della storia 9: piano 11, con la schermata.
- **Schermate:** editor coi cursori, elenco delle partite, «torna al turno N» nel registro, conferma del riavvolgimento con il numero di turni persi, eredità e albo. Tutte del piano 11.
- **Registro dei turni passati (storia 16):** gli snapshot lo contengono (`Partita.mosse` di ogni apertura). Leggerlo è del piano 11.
- **Bisezione dell'età** su piramidi molto diverse dalla base: oggi l'intervallo ±0,02 copre −5/+3 anni (`cursori-dello-scenario.md:61`).
- **Fuori dal punto tarato:** spread piatto sotto il 135,5% di debito e accumulo ai bordi dell'orientamento a ±1. Al bilanciamento (piano 12).
