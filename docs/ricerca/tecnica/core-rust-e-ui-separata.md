---
tipo: tecnica
fonti: [fonti/gdext-github.md, fonti/godot-rust-book.md, fonti/godot-export-macos.md, fonti/tauri-what-is.md, fonti/tauri-linux-graphics.md, fonti/bevy-sixth-birthday.md, fonti/rand-book-reproducibility.md]
aggiornato: 2026-10-17
---
# Core Rust e UI separata (opzione da proporre)

> **Superata il 17/10/2026** da [[decisioni]] («stack e linguaggio del core»): Mario ha scelto Tauri 2.12 + core Rust. La direzione artistica da settimanale e il pubblico della v1, solo Mac, hanno tolto peso alla mappa con shader e al Deck.

Proposta da mettere sul tavolo con Mario, non decisione: la simulazione vive in una libreria Rust che non conosce la grafica; la UI e la mappa stanno in un frontend che legge lo stato e manda comandi. Il terreno regge Rust (parsing di dati, simulazione deterministica, binario da distribuire); il costo e' un confine fra due linguaggi e una build piu' complessa.

## Forma

- Crate `sim-core`: stato, tick, effetti, eventi, salvataggi; nessuna dipendenza da motore. Si testa con `cargo test` e si fa girare senza finestra per bilanciamento e CI [INF].
- Interfaccia stretta verso il frontend: `nuova_partita(seed, dati)`, `comando(c)`, `avanza(n_tick)`, `snapshot()`, `salva()`/`carica()` [INF].
- Il frontend non contiene logica di simulazione, solo presentazione [INF]. Dettagli del core in [[architettura-simulazione]].

## Tre frontend possibili

| Frontend | Come si collega | Pro | Contro |
| --- | --- | --- | --- |
| Godot + gdext | Libreria dinamica GDExtension; classi Rust chiamabili da GDScript con tipi sicuri (fonti/gdext-github.md) [V] | UI `Control` matura, mappa con shader, renderer OpenGL di ripiego (vedi [[godot]]) | gdext e' comunitario, indipendente da Godot (fonti/godot-rust-book.md) [V], v0.5.5 pre-1.0 [M]; su macOS serve l'entitlement "Disable Library Validation" per GDExtension (fonti/godot-export-macos.md) [V]; una libreria per piattaforma da compilare [INF] |
| Tauri | Core Rust nativo nel processo; UI web tramite comandi IPC serializzati [INF] | Tutto Rust lato backend, UI web per testo e grafici, binario minimo (fonti/tauri-what-is.md) [V] | Mappa WebGL fragile su Linux e Deck (fonti/tauri-linux-graphics.md) [V]; serializzazione dello stato a ogni aggiornamento [INF] |
| Bevy | Stesso processo, stesso linguaggio | Un solo linguaggio | UI immatura e API che cambiano a ogni release (fonti/bevy-sixth-birthday.md) [V] |

## Trade-off rispetto a scrivere tutto in GDScript

- Pro del core Rust: tipi forti su un modello con centinaia di variabili; test veloci senza motore; controllo esplicito di PRNG e interi (fonti/rand-book-reproducibility.md) [V per le regole del PRNG]; il core sopravvive a un cambio di frontend [INF].
- Contro: due linguaggi; confine FFI da mantenere; tempi di compilazione; contributori del repo pubblico devono avere Rust e il motore [INF]. Il modding in GDScript dentro il core diventa impossibile: lo scripting dei mod va integrato nel core (Rhai, Lua; vedi [[modding-e-dati-esterni]]) [INF].
- Via di mezzo: partire in GDScript con la stessa separazione (un autoload "simulazione" senza riferimenti ai nodi grafici) e portare in Rust solo se il profilo lo chiede [INF].

## Per il nostro gioco

- Raccomandazione (non decisione): core Rust + Godot via gdext, da proporre a Mario con il trade-off sopra; via di mezzo GDScript se la priorita' e' arrivare presto a un prototipo giocabile.
- Prima di decidere: uno spike che compili una GDExtension Rust minima per i tre OS e la notarizzi da CI Linux.

## Collegamenti

- [[godot]], [[tauri]], [[bevy]]: i tre frontend.
- [[architettura-simulazione]]: contenuto del core.
- [[confronto-stack]].

## Fonti

- **URL** https://github.com/godot-rust/gdext · [V]
  **perche' conta qui** come Rust entra in Godot
  **cosa se ne prende** classi Rust registrate e chiamate da GDScript
- **URL** https://godot-rust.github.io/book/ · [V]
  **perche' conta qui** rischio di manutenzione del ponte
  **cosa se ne prende** progetto comunitario indipendente da Godot
- **URL** https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_macos.html · [V]
  **perche' conta qui** firma di un'app con librerie native
  **cosa se ne prende** entitlement "Disable Library Validation"
- **URL** https://v2.tauri.app/start/ · [V]
  **perche' conta qui** frontend web alternativo
  **cosa se ne prende** core Rust e webview di sistema
- **URL** https://v2.tauri.app/develop/debug/linux-graphics/ · [V]
  **perche' conta qui** limite del frontend web sulla mappa
  **cosa se ne prende** WebGL lento in silenzio su Linux
- **URL** https://bevy.org/news/bevys-sixth-birthday/ · [V]
  **perche' conta qui** maturita' del frontend tutto-Rust
  **cosa se ne prende** editor e reattivita' UI non ancora consegnati
- **URL** https://rust-random.github.io/book/crate-reprod.html · [V]
  **perche' conta qui** vantaggio concreto del core Rust
  **cosa se ne prende** regole di riproducibilita' del PRNG
