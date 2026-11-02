---
tipo: tecnica
fonti: [fonti/tauri-2-12-pathresolver.md, fonti/tauri-2-12-3-app-data-dir-sorgente.md, fonti/tauri-capabilities.md, fonti/tauri-plugin-fs.md, fonti/tauri-2-12-runevent.md, fonti/tauri-2-12-windowevent.md, fonti/tauri-2-12-app.md, fonti/tauri-2-12-3-uscita-sorgente.md, fonti/tao-0-37-1-macos-terminate.md, fonti/apple-nsapplication-terminate.md, fonti/apple-applicationshouldterminate.md, fonti/apple-applicationwillterminate.md, fonti/macos-man-2-rename.md, fonti/macos-man-2-fsync.md, fonti/rust-1-98-std-sync-all-apple.md, fonti/tempfile-namedtempfile.md, fonti/atomicwrites-0-4-4-src.md, fonti/flate2-1-1-10.md, fonti/flate2-1-1-10-gzencoder.md, fonti/flate2-1-1-10-cargo-toml.md, fonti/serde-container-attrs.md, fonti/serde-1-0-229-missing-field.md]
aggiornato: 2026-10-29
---
# Persistenza dei salvataggi (guscio Tauri 2.12, macOS)

Come il guscio scrive e rilegge salvataggi, bozza del turno e albo delle partite senza perdere dati, e quanto pesano. Il core resta senza I/O: `Partita::salva()` e `Partita::carica(&str)` danno e prendono una `String` JSON (`crates/sim-core/src/partita.rs:3141-3165`); cartella, compressione, scrittura e uscita dell'app sono lavoro del guscio (`app/src-tauri/src/lib.rs`). Basta `std` più `flate2`, che è già nel `Cargo.lock`; non serve nessun plugin.

Misure fatte da `docs/ricerca/` del worktree `docs-v1-piano-10`, branch `docs/v1-piano-10`, HEAD `95f18cb`, rustc 1.98.0, Mac17,5 (Apple A18 Pro), APFS su SSD.

## Dove: `app_data_dir`

- `PathResolver::app_data_dir()` risolve in `data_dir()/${bundle_identifier}`, a meno di un override in `app > appDirectoriesOverride`; su macOS `data_dir()` è `$HOME/Library/Application Support` (fonti/tauri-2-12-pathresolver.md) [V]. Nel sorgente 2.12.3 è `dirs::data_dir().join(identifier)` e non crea la cartella (fonti/tauri-2-12-3-app-data-dir-sorgente.md) [V]: serve `std::fs::create_dir_all` al primo uso [INF].
- Con `"identifier": "it.maeurong.simulatorepolitico"` (`app/src-tauri/tauri.conf.json:5`) il percorso reale è `~/Library/Application Support/it.maeurong.simulatorepolitico/` [INF, dalle due righe sopra]. L'app non è sandboxata (nessun entitlement in `tauri.conf.json`), quindi niente `~/Library/Containers/...` [INF].
- `tauri dev` e la build di release leggono lo stesso `identifier`, quindi scrivono **nella stessa cartella** [INF]. Mentre si costruisce, lo schema cambia spesso e un salvataggio di schema diverso viene rifiutato (`partita.rs:3165-3177`): le partite di prova finirebbero mescolate a quelle vere.

## Permessi: `std::fs` basta

- Tauri consente di default a tutte le finestre i comandi registrati dall'app con `invoke_handler`; le capabilities restringono i comandi dei plugin (fonti/tauri-capabilities.md) [V]. Il progetto ha solo `core:default` (`app/src-tauri/capabilities/default.json`).
- `tauri-plugin-fs` serve a dare il file system al **JavaScript** della webview, dentro uno scope (`$APPDATA/*`, ecc.) e con i comandi pericolosi bloccati di default (fonti/tauri-plugin-fs.md) [V]. Se scrive il lato Rust, dentro un comando `#[tauri::command]` o nell'handler di uscita, `std::fs` non passa da nessun permesso Tauri [INF]. Il plugin non serve, e non metterlo vuol dire una superficie in meno esposta alla webview.

## Chiusura dell'app e bozza del turno

Tre modi di uscire, tre percorsi nel sorgente di tauri 2.12.3 / tauri-runtime-wry 2.12.3 / tao 0.37.1:

| Come esce | Eventi che arrivano | Fonte |
|---|---|---|
| Chiusura dell'ultima finestra | `WindowEvent::CloseRequested` → `Destroyed` → `RunEvent::ExitRequested { code: None }` → `RunEvent::Exit` | fonti/tauri-2-12-3-uscita-sorgente.md [V] |
| `AppHandle::exit(code)` | `ExitRequested { code: Some(code) }` → `Exit` | fonti/tauri-2-12-3-uscita-sorgente.md, fonti/tauri-2-12-runevent.md [V] |
| **Cmd+Q / «Esci» del menu di default macOS** | **solo `RunEvent::Exit`**: niente `CloseRequested`, niente `ExitRequested` | catena qui sotto [V sul sorgente, INF sulla catena] |

La catena di Cmd+Q, un anello alla volta:
1. Il menu di default di macOS mette `PredefinedMenuItem::quit` (fonti/tauri-2-12-3-uscita-sorgente.md, `menu.rs:180-195`), e in muda 0.20.0 quella voce manda il selettore `terminate:` a `NSApp` (stesso file, `muda .../macos/mod.rs:944`) [V].
2. `terminate:` chiede al delegate `applicationShouldTerminate:`, poi, se la risposta è «termina ora», manda `willTerminateNotification`; la pulizia finale va in `applicationWillTerminate:`, e l'app termina quando quel metodo ritorna (fonti/apple-nsapplication-terminate.md, fonti/apple-applicationwillterminate.md) [V].
3. Il delegate di tao 0.37.1 registra `applicationWillTerminate:` ma **non** `applicationShouldTerminate:`, e in `applicationWillTerminate:` chiama `AppState::exit()`, che emette `Event::LoopDestroyed` in modo sincrono (fonti/tao-0-37-1-macos-terminate.md) [V]. Senza quel metodo vale la risposta di default, «termina ora» [INF: Apple documenta solo il valore di ritorno consigliato, fonti/apple-applicationshouldterminate.md].
4. tauri-runtime-wry traduce `LoopDestroyed` in `RunEvent::Exit`, e tauri chiama la callback utente prima di `cleanup_before_exit()` (fonti/tauri-2-12-3-uscita-sorgente.md) [V].

Conseguenze:
- **`RunEvent::Exit` è l'unico evento che arriva in tutti e tre i casi**, ed è sincrono sul thread principale: un salvataggio fatto lì finisce prima che il processo esca [INF dalla catena]. Cmd+Q non si può annullare né rinviare: tao non espone `applicationShouldTerminate`, e del resto nella v1 non serve (chiudere non finisce mai la partita, `.scratch/v1-giocabile/issues/28-salvataggi.md:29`).
- `applicationWillTerminate:` non viene chiamato in caso di *sudden termination* (fonti/apple-applicationwillterminate.md) [V]. Non ho verificato se il bundler di Tauri mette `NSSupportsSuddenTermination` nell'`Info.plist` [NON TROVATO]: va controllato con `plutil -p` sull'`.app` costruita.
- Oggi `run()` usa `Builder::run(context)` (`lib.rs:101-112`), che non accetta callback. Per l'handler serve `Builder::build(context)?.run(|app, evento| ...)`: `App::run` non ritorna mai e alla fine chiama `std::process::exit` (fonti/tauri-2-12-app.md) [V].
- Le decisioni del turno sono già nello stato Rust, perché i comandi del giocatore modificano `Partita` sul posto (`Partita::comanda(&mut self, c: Comando)`, `crates/sim-core/src/comando.rs:188`). La bozza quindi è `partita.salva()` dello stato in memoria: non c'è niente da chiedere alla webview, che a quel punto potrebbe non rispondere più [INF].
- Prima di salvare, l'handler di `Exit` prende il `Mutex` dello stato. Se in quel momento un `chiudi_turno` asincrono lo tiene occupato, l'handler aspetta che finisca, il che costa pochi millisecondi in release. Un lock avvelenato si recupera con `PoisonError::into_inner`, come in `avvia` (`lib.rs:39-41`) [INF].
- Un crash o un `kill -9` durante il turno perdono la bozza, mentre il salvataggio dell'apertura resta intatto [INF]. Se un giorno servisse di più, la bozza si può scrivere dopo ogni comando: con i tempi misurati sotto costa circa 25 ms in release. Non è richiesto, quindi non si fa.

## Scrittura atomica su APFS

- `rename(2)` su macOS garantisce che un'istanza di `new` esista sempre, anche se il sistema va in crash a metà dell'operazione (fonti/macos-man-2-rename.md) [V]. A un crash a metà resta quindi il file vecchio o quello nuovo, mai uno troncato, purché il nuovo sia già su disco prima del `rename` [INF].
- Su macOS `fsync()` porta i dati fino al disco, ma il disco può tenerli in cache e riordinarli; per l'integrità serve `F_FULLFSYNC` (fonti/macos-man-2-fsync.md) [V]. In Rust 1.98 `File::sync_all()` su Apple chiama proprio `fcntl(fd, F_FULLFSYNC)` (fonti/rust-1-98-std-sync-all-apple.md) [V]: con `std` non c'è niente di speciale da fare.
- Ricetta std: `File::create(dir/nome.tmp)`, poi `write_all`, `sync_all`, `rename(tmp, dir/nome)` e infine `File::open(dir)?.sync_all()`, per rendere durevole anche la voce di directory. Il temporaneo sta nella stessa cartella, perché `rename` non attraversa i file system [INF]. Misurata in sessione su un file da 175 KB: 6-10 ms, quasi tutti di `F_FULLFSYNC`; il `sync_all` sulla directory funziona su APFS senza errori [M: `cargo run --release -q --manifest-path <scratchpad>/misura-salvataggi/Cargo.toml -- 1 100`, colonna `scrivi_ms`].
- `tempfile` 3.27.0: `NamedTempFile::persist` sostituisce il file in modo atomico ma **non sincronizza né il contenuto né la cartella** (fonti/tempfile-namedtempfile.md) [V]. Il `sync_all` andrebbe chiamato a mano comunque, e il guadagno sulle 10 righe std è nullo.
- `atomicwrites` 0.4.4 fa `sync_all` del temporaneo e delle cartelle, ma scrive il temporaneo in una sottocartella casuale `.atomicwrite*` dentro la cartella di destinazione (fonti/atomicwrites-0-4-4-src.md) [V]. Fa la stessa cosa della ricetta std con una dipendenza nuova in più.
- Ai `*.tmp` lasciati da un crash ci pensa il guscio: all'avvio li ignora o li cancella [INF].

## Compressione: `flate2`

- `flate2` 1.1.10 è già nel lock, ci arriva via `png` → `ico`/`muda` → `tauri`, e il suo backend attivo è `miniz_oxide` 0.9.1 (`zlib-rs` sta nel lock ma non è attivato) [M: `cargo tree -p flate2 -e features` e `cargo tree -i zlib-rs --target all`, da HEAD `95f18cb`]. Il `miniz_oxide` 0.8.9 alla riga ~1770 del lock appartiene a `png`, non a `flate2`, che usa lo 0.9.1 (`Cargo.lock:775-783`).
- Il backend di default `rust_backend` usa `miniz_oxide`, un port di miniz.c che non richiede un compilatore C ed è solo Rust safe. La doc avverte che un giorno `rust_backend` potrebbe passare a `zlib-rs` (fonti/flate2-1-1-10.md) [V].
- Licenze: `flate2` e `crc32fast` MIT OR Apache-2.0, `miniz_oxide` MIT OR Zlib OR Apache-2.0, `adler2` 0BSD OR MIT OR Apache-2.0, `simd-adler32` MIT (fonti/flate2-1-1-10-cargo-toml.md) [V]. Tutte compatibili con un gioco chiuso [INF].
- API: `write::GzEncoder::new(Vec::new(), Compression::default())`, poi `write_all` e `finish()`, che restituisce il `Vec<u8>`. Senza `finish()` lo stream resta senza coda (fonti/flate2-1-1-10-gzencoder.md) [V]. Per leggere: `read::GzDecoder::new(file)` e `read_to_string`. `Compression::default()` vale 6 (fonti/flate2-1-1-10-cargo-toml.md) [V]. Il decoder confronta CRC32 e lunghezza della coda e dà errore se non tornano (stessa fonte, `gz/bufread.rs:326-336`) [V]: un file troncato o corrotto non arriva mai a `carica` come JSON plausibile.
- **Guscio o core?** La compressione non è I/O, e il vincolo del core («senza I/O e senza thread») non la escluderebbe. Va comunque nel guscio [INF]: il contratto del core resta `String` JSON, che i test confrontano a occhio (`architettura-simulazione.md:33`); `gzip` riguarda come il file è impacchettato, come la cartella e il nome; nel guscio `flate2` costa zero crate nuove, nel core sarebbe la prima dipendenza di I/O-formato; e il bias «non leggibile a colpo d'occhio» (`28-salvataggi.md:41`) lo dà già il gzip.

## Misure

Partita di prova: `Partita::nuova(dati/base, seme)`, `chiudi_turno()` ripetuto senza decisioni del giocatore, `salva()` a ogni apertura. Programma usa-e-getta in `/private/tmp/claude-501/-Users-mario-GitHub-simulatore-politico/808ec725-796d-4e4e-95f7-833295e53cda/scratchpad/misura-salvataggi/` (dipende da `sim-core` del worktree per path e da `flate2 = "=1.1.10"`), fuori dal worktree, che resta pulito. Per ogni turno: `salva`, gzip ai livelli 1, 6 (default) e 9, `GzDecoder` + `carica` con `assert!(riletta == p)`, scrittura atomica.

Dimensioni, seme 1 [M: `cargo run --release -q --manifest-path …/misura-salvataggi/Cargo.toml -- 1 100`, HEAD `95f18cb`]:

| Turno | In chiaro | gzip 1 | gzip 6 (default) | gzip 9 | Rapporto (6) |
|---|---|---|---|---|---|
| 0 | 418 363 B | 108 852 B | 91 326 B | 90 868 B | 4,6× |
| 20 | 617 446 B | 207 886 B | 163 046 B | 162 559 B | 3,8× |
| 40 | 641 887 B | 214 419 B | 168 066 B | 167 525 B | 3,8× |
| 100 | 645 456 B | 221 125 B | 174 564 B | 174 013 B | 3,7× |

- Seme 2 conferma: 419 149 / 91 515 B al turno 0, 619 141 / 163 563 B al 20, 644 986 / 168 088 B al 40 (in chiaro / gzip 6) [M: stesso comando, `-- 2 40`].
- Il peso cresce nei primi ~10 turni e poi si ferma intorno ai 640 KB in chiaro e ai 170 KB gzip: non c'è una storia che si accumula [M: tabella sopra].
- **Cento snapshot**: i 101 snapshot dei turni 0-100 fanno 63,2 MB in chiaro, **16,7 MB gzip 6** e 16,67 MB gzip 9 [M: riga `totale` dello stesso comando]. Una partita di 25 anni di gioco sta sotto i 17 MB.
- Chi pesa, al turno 40 (in chiaro / gzip 6): `elettorato` 391 KB / 118 KB, `dati` 162 KB / 32 KB, `grandezze` 37 KB / 2,7 KB, `informazione` 13,6 KB / 6,2 KB, il resto sotto i 10 KB [M: `cargo run --release -q --manifest-path …/misura-salvataggi/Cargo.toml --bin campi -- 40`]. La copia dei dati vale circa 32 KB per snapshot, cioè 3,2 MB su 100 (il 19%): tenerla una volta sola per partita richiederebbe di spezzare lo snapshot, e per 3 MB non vale la pena [INF]. L'elettorato comprime solo 3,3×, perché sono soprattutto `f64` [INF].
- Tempi in release, valori tipici del seme 1 [M: stesso comando]: `salva` 4-5 ms (comprende la rilettura di controllo), gzip 1 circa 3 ms, **gzip 6 14-15 ms**, gzip 9 21-23 ms, `GzDecoder` + `carica` circa 5 ms, scrittura atomica 6-10 ms. Per ogni apertura di turno: circa 25-30 ms fuori dal thread della finestra. Alcuni picchi isolati (198 ms al turno 79) vengono dal carico di altri processi.
- Tempi in debug (`tauri dev`) [M: `cargo run -q --manifest-path …/misura-salvataggi/Cargo.toml -- 1 3`]: `salva` 67-77 ms, gzip 6 122-203 ms, gzip 9 225-325 ms, `carica` 54-69 ms. Siamo intorno ai 250 ms: si sentono, ma non bloccano.
- Il livello 9 rispetto al 6 fa risparmiare lo 0,3% e costa il 50% di tempo in più; il livello 1 è 4-5 volte più veloce e pesa il 27% in più [M: tabella]. Il default va bene.
- Riscrivere un file unico con tutti i 101 snapshot (16,7 MB) con `F_FULLFSYNC` + `rename` costa 21-38 ms [M: `cargo run --release -q --manifest-path …/misura-salvataggi/Cargo.toml --bin unico`, 5 giri].
- Limite delle misure: la partita di prova non prende decisioni. Con atti, leggi e registro di un giocatore vero alcune sezioni crescono [INF], ma l'ordine di grandezza (centinaia di KB in chiaro, 100-200 KB gzip) è quello dell'elettorato, che non dipende dalle decisioni [INF].

## Albo leggibile con uno schema nuovo

L'albo è un formato diverso dallo snapshot e ha una regola opposta. Lo snapshot rifiuta qualsiasi schema diverso (`partita.rs:3165-3177`); l'albo deve restare leggibile anche dalle versioni future.

- serde ignora di default i campi sconosciuti nei formati autodescrittivi come JSON; serve `deny_unknown_fields` per rifiutarli (fonti/serde-container-attrs.md) [V]. Una versione **vecchia** legge quindi un albo **nuovo** che ha dei campi in più.
- Un campo `Option<T>` che manca nel file si deserializza come `None` **senza** `#[serde(default)]`: `MissingFieldDeserializer::deserialize_option` chiama `visit_none` (fonti/serde-1-0-229-missing-field.md) [V]. Una versione **nuova** legge quindi un albo **vecchio**, purché i campi aggiunti dopo siano `Option<T>`.
- Il vincolo «niente `serde(default)`» dei piani (per esempio `docs/superpowers/plans/2026-10-28-v1-piano-8a-governo-e-opposizione-ia.md:58`) per l'albo non è un ostacolo: la tolleranza viene da `Option<T>` e dal comportamento di default di serde, l'attributo non serve [INF]. Nota a margine: il codice ha già dei `#[serde(default)]` sulle strutture dei dati TOML (per esempio `crates/sim-core/src/disposizioni/mod.rs:79`, `atti.rs:79`), quindi il vincolo nella pratica riguarda i campi nuovi, non tutto il crate.
- Regole del formato [INF]: un campo `formato_albo: u32`, indipendente da `VERSIONE_SCHEMA`; i campi si aggiungono soltanto, mai rinominati, tolti o cambiati di tipo, e ogni campo aggiunto dopo la versione 1 è `Option<T>`; i valori si copiano per esteso (nomi delle leggi, dei partiti, dello scenario) invece di riferirsi a id dei dati, perché i dati possono cambiare; niente enum chiuse per valori che possono crescere: una variante sconosciuta fa fallire tutto il file, mentre un campo sconosciuto viene solo ignorato. Se un domani `formato_albo` cambiasse in modo incompatibile, la lettura ripiega su `serde_json::Value` e mostra i campi che trova.
- Formato: JSON **in chiaro**, `albo/<id>.json`, pochi KB [INF]: dentro non c'è il bias, e un file che resta leggibile da un editor di testo è leggibile anche fra dieci versioni. La struct sta nel core, accanto a `Eredita`, ed è pura; il guscio scrive.

## Organizzazione su disco

La spec dice «più partite in corso, un file ciascuna» (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:391`). Messa a confronto:

- **File unico per partita con 100 snapshot dentro.** Il tempo di riscrittura non è il problema: 21-38 ms per 16,7 MB [M, sopra]. Il problema è tutto il resto [INF]. Si riscrive tutto a ogni turno: sommando 1 + 2 + … + 101 snapshot da circa 165 KB vengono circa 850 MB scritti per partita, contro 16,7 MB. Serve un contenitore con un indice (gzip multi-membro, tar o zip, cioè una crate o un formato in più). Il riavvolgimento diventa una riscrittura invece di una cancellazione. Per leggere l'ultimo snapshot si apre tutto il file.
- **Una cartella per partita** [INF, raccomandata]:
  ```
  ~/Library/Application Support/it.maeurong.simulatorepolitico/
    partite/<id>/
      indice.json            nome, scenario, turno, data, versione_schema: piccolo, in chiaro, tollerante come l'albo
      turno-000.json.gz      un salvataggio per apertura, mai riscritto
      …
      turno-099.json.gz
      bozza.json.gz          solo se c'è un turno in corso con decisioni
    albo/<id>.json
  ```
  Ogni file si scrive una volta e in modo atomico. Il riavvolgimento a N cancella `bozza` e poi `turno-K` per K > N, dal più alto in giù: se un crash lo interrompe, la partita si riapre all'apertura di un turno fra N e quello di prima, il che non fa danni e si può ripetere. Elencare le partite costa un `read_dir` più un `indice.json` per partita.
- **`indice.json` serve** anche se il turno e la data si potrebbero ricavare dai file. La schermata delle partite in corso deve mostrare nome e scenario anche di una partita il cui schema non si apre più (rifiuto «con un messaggio chiaro, il file resta», `28-salvataggi.md:33`), e `carica` su uno schema diverso fallisce prima di arrivare al nome [INF].
- **Regola di ripresa** [INF]: si apre `bozza.json.gz` se esiste e il suo `turno` coincide con quello dell'ultimo `turno-K`, altrimenti `turno-K` e la bozza si cancella. Così si copre anche il crash fra la scrittura di `turno-(K+1)` e la cancellazione della bozza vecchia.

## Per il nostro gioco

- **Prendiamo:**
  - `app.path().app_data_dir()` più `create_dir_all`;
  - `std::fs` dal lato Rust, senza `tauri-plugin-fs` e senza capabilities nuove;
  - `flate2 = "=1.1.10"` (già nel lock) nel `Cargo.toml` del guscio, livello `Compression::default()`;
  - la ricetta atomica std (temporaneo nella stessa cartella, `sync_all`, `rename`, `sync_all` della cartella), senza `tempfile` né `atomicwrites`;
  - la bozza scritta nell'handler di `RunEvent::Exit`, l'unico evento comune a Cmd+Q, chiusura della finestra e `exit`, quindi `Builder::build(...)?.run(callback)` al posto di `Builder::run`;
  - una cartella per partita, con `indice.json`, un `turno-NNN.json.gz` per apertura e `bozza.json.gz`;
  - l'albo in JSON in chiaro, con un suo `formato_albo` e i campi nuovi solo `Option<T>`.
- **Evitiamo:** di mettere il salvataggio della bozza in `CloseRequested` o `ExitRequested`, che con Cmd+Q non arrivano; un file unico con dentro tutti gli snapshot; la compressione nel core; `deny_unknown_fields` nell'albo.
- **Aperto:**
  - Dove vive il contatore dei riavvolgimenti. Se sta nello snapshot, riavvolgere al turno N lo riporta al valore di N. Due strade: il guscio, dopo il riavvolgimento, carica N, chiama un metodo del core che incrementa il contatore e riscrive `turno-N`, così il caso resta intatto perché il generatore non viene toccato; oppure il contatore sta in `indice.json` e il core lo riceve per l'eredità.
  - In `tauri dev`, una sottocartella separata (per esempio con `cfg!(debug_assertions)`, oppure `appDirectoriesOverride`) per non mescolare le partite di prova con quelle vere.
  - Verificare `NSSupportsSuddenTermination` nell'`Info.plist` costruito, e provare a mano che Cmd+Q arrivi a `RunEvent::Exit`: basta un `eprintln!` nell'handler e Cmd+Q da `tauri dev`.

## Collegamenti

- [[tauri]]: lo stack del guscio; questa pagina ne precisa cartelle e uscita.
- [[architettura-simulazione]]: contenuto dello snapshot e scelta del formato testuale (riga 33).
- [[core-rust-e-ui-separata]]: perché il core resta senza I/O.
- [[decisioni]]: decisione del 20/10/2026 sui salvataggi.
- [[salvataggi-e-riavvolgimento]]: il lato di gioco (riavvolgimento, versioni) visto negli altri titoli, pagina parallela.

## Fonti

- **URL** https://docs.rs/tauri/2.12.3/tauri/path/struct.PathResolver.html · [V]
  **perche' conta qui** dove finiscono i salvataggi su macOS
  **cosa se ne prende** `app_data_dir` = `data_dir/${identifier}`, `data_dir` = `~/Library/Application Support`, override `appDirectoriesOverride`
- **URL** `~/.cargo/registry/.../tauri-2.12.3/src/path/` (desktop.rs:240-257, mod.rs:395-440) · [V]
  **perche' conta qui** se la cartella esiste già
  **cosa se ne prende** `dirs::data_dir().join(identifier)`, nessuna creazione: serve `create_dir_all`
- **URL** https://v2.tauri.app/security/capabilities/ · [V]
  **perche' conta qui** quali permessi chiedono i comandi del guscio
  **cosa se ne prende** i comandi dell'app sono consentiti di default a tutte le finestre; niente capability nuova
- **URL** https://v2.tauri.app/plugin/file-system/ · [V]
  **perche' conta qui** se serve `tauri-plugin-fs`
  **cosa se ne prende** il plugin espone il file system al JS con scope e permessi; dal lato Rust non serve
- **URL** https://docs.rs/tauri/2.12.3/tauri/enum.RunEvent.html · [V]
  **perche' conta qui** l'evento su cui salvare la bozza
  **cosa se ne prende** `Exit`, `ExitRequested { code, api }` con `code: None` per l'utente e `Some` per `AppHandle::exit`
- **URL** https://docs.rs/tauri/2.12.3/tauri/enum.WindowEvent.html · [V]
  **perche' conta qui** l'alternativa per la finestra
  **cosa se ne prende** `CloseRequested { api }`, che riguarda la finestra e non l'app
- **URL** https://docs.rs/tauri/2.12.3/tauri/struct.App.html · [V]
  **perche' conta qui** come agganciare la callback degli eventi
  **cosa se ne prende** `App::run(callback)` non ritorna e chiude con `std::process::exit`; `run_return` restituisce il codice
- **URL** `~/.cargo/registry/.../tauri-runtime-wry-2.12.3/src/lib.rs` (4131-4133, 4252-4270, 4297-4311), `tauri-2.12.3/src/app.rs:1449-1456`, `menu/menu.rs:180-195`, `muda-0.20.0/.../macos/mod.rs:940-946` · [V]
  **perche' conta qui** quali eventi arrivano davvero nei tre modi di uscire
  **cosa se ne prende** ultima finestra → `ExitRequested(None)` → `Exit`; `LoopDestroyed` → `Exit` prima di `cleanup_before_exit`; la voce Esci del menu di default = `terminate:`
- **URL** `~/.cargo/registry/.../tao-0.37.1/src/platform_impl/macos/` (app_delegate.rs:46-88, 124-134; app_state.rs:272-282) · [V]
  **perche' conta qui** cosa fa tao con Cmd+Q
  **cosa se ne prende** `applicationWillTerminate:` → `AppState::exit()` → `LoopDestroyed` sincrono; nessun `applicationShouldTerminate:`
- **URL** https://developer.apple.com/tutorials/data/documentation/appkit/nsapplication/terminate(_:).md · [V]
  **perche' conta qui** la sequenza di AppKit a Cmd+Q
  **cosa se ne prende** `applicationShouldTerminate:` → `willTerminateNotification`; pulizia in `applicationWillTerminate:`
- **URL** https://developer.apple.com/tutorials/data/documentation/appkit/nsapplicationdelegate/applicationshouldterminate(_:).md · [V]
  **perche' conta qui** se l'uscita si può rinviare
  **cosa se ne prende** solo implementando questo metodo, che tao non registra; Cmd+Q non si rinvia
- **URL** https://developer.apple.com/tutorials/data/documentation/appkit/nsapplicationdelegate/applicationwillterminate(_:).md · [V]
  **perche' conta qui** quanto è garantito il salvataggio all'uscita
  **cosa se ne prende** l'app termina quando il metodo ritorna; non viene chiamato in caso di sudden termination
- **URL** `man 2 rename` (macOS, Darwin 27.0.0) · [V]
  **perche' conta qui** l'atomicità della sostituzione del file
  **cosa se ne prende** `new` esiste sempre, anche con un crash a metà
- **URL** `man 2 fsync` (macOS, Darwin 27.0.0) · [V]
  **perche' conta qui** durabilità su disco Apple
  **cosa se ne prende** `fsync` non svuota la cache del disco; serve `F_FULLFSYNC`
- **URL** `rust-src` 1.98.0, `library/std/src/sys/fs/unix.rs:1410-1440` · [V]
  **perche' conta qui** se `std` basta per la durabilità
  **cosa se ne prende** `File::sync_all` su Apple = `fcntl(F_FULLFSYNC)`
- **URL** https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html (3.27.0) · [V]
  **perche' conta qui** l'alternativa crate alla ricetta std
  **cosa se ne prende** `persist` atomico ma senza sync del contenuto né della cartella
- **URL** https://docs.rs/atomicwrites/0.4.4/src/atomicwrites/lib.rs.html · [V]
  **perche' conta qui** l'altra alternativa crate
  **cosa se ne prende** fa `sync_all` del temporaneo e delle cartelle, con il temporaneo in una sottocartella `.atomicwrite*`; nulla in più della ricetta std
- **URL** https://docs.rs/flate2/1.1.10/flate2/ · [V]
  **perche' conta qui** il backend di compressione
  **cosa se ne prende** `rust_backend` = `miniz_oxide`, solo Rust safe, senza compilatore C; possibile passaggio futuro a `zlib-rs`
- **URL** https://docs.rs/flate2/1.1.10/flate2/write/struct.GzEncoder.html · [V]
  **perche' conta qui** l'API da usare
  **cosa se ne prende** `GzEncoder::new(w, level)`, `finish()` che restituisce il writer
- **URL** `~/.cargo/registry/.../flate2-1.1.10/` (Cargo.toml, src/lib.rs:264-268, src/gz/bufread.rs:326-336) e i `Cargo.toml` di miniz_oxide 0.9.1, crc32fast, adler2, simd-adler32 · [V]
  **perche' conta qui** licenze, default, integrità in lettura
  **cosa se ne prende** licenze MIT/Apache/Zlib/0BSD; `Compression::default()` = 6; CRC32 e lunghezza verificate dal decoder
- **URL** https://serde.rs/container-attrs.html · [V]
  **perche' conta qui** come una versione vecchia legge un albo nuovo
  **cosa se ne prende** campi sconosciuti ignorati di default in JSON
- **URL** `~/.cargo/registry/.../serde-1.0.229/src/private/de.rs:20-60` · [V]
  **perche' conta qui** come una versione nuova legge un albo vecchio senza `serde(default)`
  **cosa se ne prende** un campo `Option<T>` mancante diventa `None`
