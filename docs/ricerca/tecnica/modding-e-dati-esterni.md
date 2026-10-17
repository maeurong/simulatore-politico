---
tipo: tecnica
fonti: [fonti/democracy4-modding.md, fonti/steam-lawgivers2.md, fonti/mlua-github.md, fonti/rhai-book-safety.md, fonti/extism-github.md, fonti/godot-exporting-packs.md]
aggiornato: 2026-10-16
---
# Modding e dati esterni

Il contenuto del gioco (paesi, policy, effetti, eventi) va in file testuali fuori dal codice, come fa Democracy; lo scripting serve solo dove i dati non bastano e va eseguito in sandbox. Tre livelli: dati, script in sandbox, niente codice nativo.

## Livello 1: dati testuali

- Democracy 4 tiene quasi tutta la simulazione in file CSV nella cartella `data\simulation`: si modifica il file e si gioca subito il modello cambiato (fonti/democracy4-modding.md; vedi [[democracy-4]]) [V].
- Lawgivers II offre mod di paesi, leggi, eventi, parametri e ideologie con un editor di testo (fonti/steam-lawgivers2.md; vedi [[lawgivers-2]]) [V].
- Formato per noi: TOML o RON per entita' strutturate (policy, ministeri), CSV per tabelle numeriche lunghe (serie storiche), formule di effetto come espressioni in un mini-linguaggio valutato dal core [INF].
- Ogni file con versione di schema; validazione all'avvio con errori leggibili per il modder [INF].

## Livello 2: scripting in sandbox

- **Rhai**: linguaggio di scripting per Rust, in sandbox (un motore immutabile non puo' toccare l'ambiente se non e' permesso esplicitamente), garanzia "non va mai in panico", protezioni contro stack overflow, dati sovradimensionati e script che non terminano, supporto `Decimal`, parole chiave disattivabili, serializzazione serde (fonti/rhai-book-safety.md) [V]. Si presta come linguaggio delle formule [INF].
- **Lua via mlua**: binding Rust per Lua 5.1-5.5, LuaJIT e Luau, con serde e async; in modalita' standalone permette un runtime configurato per sicurezza (fonti/mlua-github.md) [V]. Lua e' il linguaggio di mod piu' noto ai giocatori [INF].
- **WASM via Extism**: pensato per eseguire codice non fidato degli utenti in sicurezza, con plugin scritti in molti linguaggi (fonti/extism-github.md) [V]. Piu' pesante da adottare per i modder [INF].

## Livello 3: niente codice nativo

- Godot carica a runtime pacchetti PCK o ZIP con scene, risorse e script; la doc avverte che un mod puo' contenere codice malevolo e propone pacchetti firmati con crittografia asimmetrica (fonti/godot-exporting-packs.md) [V]. Un PCK con GDScript e' quindi codice arbitrario [INF].
- Godot puo' anche leggere a runtime file sciolti non impacchettati (fonti/godot-exporting-packs.md) [V]: e' la via per i dati di livello 1.
- Con Tauri o Electron, JS dei mod nella webview e' codice arbitrario [INF].

## Per il nostro gioco

- Prendiamo: tutto il modello in file testuali leggibili come Democracy; formule in Rhai o in un mini-linguaggio di espressioni; eventi in Rhai o Lua.
- Evitiamo: mod come PCK con script o come codice nativo; formule scritte nel codice del gioco.
- Aperto: Rhai contro Luau (familiarita' dei modder contro integrazione con Rust); Steam Workshop o semplice cartella `mods/`.

## Collegamenti

- [[democracy-4]], [[lawgivers-2]]: modding per file testuali nei giochi di riferimento.
- [[architettura-simulazione]]: i dati caricati sono lo stato iniziale del core.
- [[godot]]: PCK e file sciolti.

## Fonti

- **URL** https://github.com/mlua-rs/mlua · [V]
  **perche' conta qui** Lua dentro un core Rust
  **cosa se ne prende** Lua 5.1-5.5, LuaJIT, Luau, serde
- **URL** https://rhai.rs/book/about/features.html · [V]
  **perche' conta qui** scripting sicuro per mod e formule
  **cosa se ne prende** sandbox, niente panic, limiti contro script infiniti, `Decimal`
- **URL** https://github.com/extism/extism · [V]
  **perche' conta qui** plugin non fidati in WASM
  **cosa se ne prende** esecuzione sicura di codice utente in piu' linguaggi
- **URL** https://docs.godotengine.org/en/stable/tutorials/export/exporting_pcks.html · [V]
  **perche' conta qui** rischi dei mod impacchettati in Godot
  **cosa se ne prende** codice malevolo nei PCK, firma, file sciolti a runtime
