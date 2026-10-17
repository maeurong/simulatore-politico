Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Con quale stack si costruisce la v1: Godot 4.7 con core di simulazione separato (docs/ricerca/sintesi/confronto-stack.md:28), e il core in Rust via gdext o in GDScript (docs/ricerca/tecnica/core-rust-e-ui-separata.md:32)? Rust va proposto con il trade-off e deciso da Mario.

## Answer

Grilling con Mario del 17/10/2026, due giri, tutte le raccomandazioni accettate. Si scosta dalla raccomandazione della ricerca (Godot + gdext, `docs/ricerca/sintesi/confronto-stack.md:30-33`), che pesava la mappa con shader e la tenuta su Steam Deck e Linux. Dopo la Direzione artistica (10) la UI è un settimanale da impaginare e la mappa è a tinte piatte; il pubblico della v1 è Mario sul Mac. Quindi conta di più il testo, e il punto debole di Tauri (WebGL su Linux, `docs/ricerca/tecnica/core-rust-e-ui-separata.md:21`) non entra in gioco.

**Deciso da Mario**
- **Stack: Tauri 2.12 + core di simulazione in Rust.** UI in HTML/CSS/TypeScript nella webview; mappa = SVG delle 20 regioni a tinte piatte, senza WebGL. Rust proposto con il trade-off (secondo linguaggio, compilazioni lente, un confine da mantenere, contro tipi forti, test senza finestra, PRNG e interi sotto controllo, partite simulate in serie per il bilanciamento) e accettato.
- **Numeri e riproducibilità:** interi per euro, seggi, voti ed elettori; `f64` per Leontief, IS-Phillips e logit, arrotondati a interi quando toccano i conti pubblici (`docs/ricerca/tecnica/architettura-simulazione.md:21`). Stesso seed + stessi comandi = stessa partita sullo stesso binario, per debug e test di regressione. I salvataggi sono snapshot, non replay; niente identità fra sistemi operativi o versioni.
- **Modding v1: solo dati.** Catalogo, disposizioni e parametri in file di testo letti dal core. Lo scripting in sandbox (Rhai o Lua, `docs/ricerca/tecnica/modding-e-dati-esterni.md:31-33`) va fra i punti di aggancio.
- **Corpo del testo:** chiusa la regola dei 9 px del Deck rimasta aperta in Direzione artistica. Testo minimo 12 px a scala 1×, scala della UI regolabile dal primo giorno; il Deck resta possibile ma non è un vincolo.

**Scelte di strumento, dichiarate e non chieste**
- UI con **Svelte 5 + TypeScript + Vite**, il template ufficiale di Tauri.
- **Core puro:** il crate `sim-core` non fa I/O e non usa thread. I file li legge il guscio Tauri e li passa al core, che così resta compilabile in WASM: web e altre piattaforme non sono escluse.
- Confine come in `docs/ricerca/tecnica/core-rust-e-ui-separata.md:13`: comandi tipizzati verso il core, `snapshot` serializzato verso la UI una volta per fase del turno, tipi TypeScript generati dal Rust.
- Un secondo binario CLI sullo stesso crate, per le partite senza finestra (bilanciamento e test).
- Build solo per il Mac, versioni bloccate, nessuna CI di notarizzazione nella v1.

**Effetti sulla mappa:** si chiude la nebbia «Spike tecnico del core», perché chiedeva interi o float, e la risposta è sopra. Nessun ticket di spike: nessuna decisione lo aspetta. La prima compilazione Tauri + Rust diventa il primo passo del piano.
