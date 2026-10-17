---
tipo: tecnica
fonti: [fonti/raylib-github.md, fonti/defold-faq.md, fonti/defold-license.md, fonti/love2d-github.md, fonti/flutter-desktop.md, fonti/slint-github.md, fonti/slint-licensing.md, fonti/egui-github.md]
aggiornato: 2026-10-16
---
# Altre opzioni: raylib, Defold, LÖVE, Flutter, Slint, egui

Sei alternative minori, scartate o tenute come ripiego. Nessuna unisce UI densa matura e mappa con shader quanto Godot. Slint ed egui interessano solo come UI per un core Rust; Defold e' l'unico motore di gioco completo del gruppo.

## raylib

- Ultima release 6.2 del 15/10/2026 [M: `gh api repos/raysan5/raylib/releases/latest`, 16/10/2026].
- Libreria C "spartana": niente interfaccia grafica, niente editor; una sola finestra con un solo contesto OpenGL; ridimensionare o spostare la finestra ferma il ciclo di rendering sulle piattaforme che hanno finestre (fonti/raylib-github.md) [V].
- Licenza zlib/libpng, link statico anche in software chiuso (fonti/raylib-github.md) [V].
- Per noi: tutta la UI densa da scrivere a mano (raygui e' minimale) [INF]. Scartata.

## Defold

- Ultima release 1.14.0 del 12/10/2026 [M: `gh api repos/defold/defold/releases/latest`, 16/10/2026].
- Gratuito, senza royalty; licenza Defold derivata da Apache 2.0; posseduto da una fondazione (fonti/defold-faq.md, fonti/defold-license.md) [V].
- Logica in Lua 5.1/LuaJIT; supporto C# annunciato con costi di peso e prestazioni (fonti/defold-faq.md) [V].
- Runtime su macOS 12+, Windows, qualsiasi Linux 64 bit con driver aggiornati (fonti/defold-faq.md) [V].
- Per noi: motore leggero orientato al gioco 2D/mobile; il sistema GUI e' a nodi di gioco, non un toolkit di applicazione con container e temi [INF]. Ripiego possibile, non preferibile.

## LÖVE

- Ultima release 11.5 del 03/12/2023; il ramo `main` sviluppa la prossima major e "non va considerato stabile" (fonti/love2d-github.md) [V] [M: `gh api repos/love2d/love/releases`, 16/10/2026].
- Framework 2D in Lua per Windows, macOS, Linux, Android, iOS (fonti/love2d-github.md) [V]. Licenza: GitHub API restituisce `NOASSERTION` [M]; la zlib dichiarata dal progetto non e' nella cattura [NON TROVATO].
- Per noi: nessuna release da quasi tre anni, UI da zero. Scartata.

## Flutter (desktop)

- Compila app native Windows, macOS, Linux con plugin per desktop (fonti/flutter-desktop.md) [V].
- Ottimo per UI densa e testo; non e' un motore di gioco: la mappa animata si fa con canvas e shader di frammento, senza ciclo di gioco né pipeline di asset [INF]. Linguaggio Dart, lontano da Rust.
- Versione stabile corrente: [NON TROVATO] (le release non sono su GitHub).

## Slint

- Ultima release 1.18.1 del 21/09/2026 [M: `gh api repos/slint-ui/slint/releases/latest`, 16/10/2026].
- Toolkit GUI dichiarativo per Rust, C++, JS, Python; desktop, embedded, web (fonti/slint-github.md) [V].
- Tripla licenza a scelta: royalty-free (app proprietarie desktop/mobile/web gratis), GPLv3, commerciale (fonti/slint-licensing.md) [V]. Con un repo pubblico vanno bene sia GPLv3 sia royalty-free.
- Per noi: UI da applicazione, non da gioco; la mappa animata resta da fare a parte [INF].

## egui / eframe

- Ultima release 0.37.0 del 15/10/2026 [M: `gh api repos/emilk/egui/releases/latest`, 16/10/2026].
- Immediate mode in Rust, desktop e web tramite `eframe`, accessibilita' via AccessKit (fonti/egui-github.md) [V].
- Limite dichiarato: il layout complesso e' difficile e alcuni elementi si posizionano con un frame di ritardo (fonti/egui-github.md) [V].
- Per noi: ottimo per strumenti interni (editor di dati, debug della simulazione); aspetto da strumento, non da gioco "spettacolare" [INF].

## Per il nostro gioco

- Prendiamo: egui come possibile UI di debug del core Rust; Rhai o Lua come scripting (vedi [[modding-e-dati-esterni]]).
- Evitiamo: raylib e LÖVE come base.
- Aperto: nessuna di queste supera Godot o Tauri sui criteri del brief.

## Collegamenti

- [[confronto-stack]]: tabella completa.
- [[bevy]]: egui come UI sopra Bevy.
- [[core-rust-e-ui-separata]].

## Fonti

- **URL** https://github.com/raysan5/raylib · [V]
  **perche' conta qui** limiti di una libreria minimale
  **cosa se ne prende** finestra singola, niente UI, licenza zlib
- **URL** https://defold.com/faq/faq/ · [V]
  **perche' conta qui** piattaforme e linguaggio di Defold
  **cosa se ne prende** Lua 5.1/LuaJIT, runtime su ogni Linux 64 bit, gratuito
- **URL** https://defold.com/license/ · [V]
  **perche' conta qui** licenza
  **cosa se ne prende** derivata da Apache 2.0, uso commerciale libero
- **URL** https://github.com/love2d/love · [V]
  **perche' conta qui** maturita'
  **cosa se ne prende** `main` instabile, release ferma al 2023
- **URL** https://docs.flutter.dev/platform-integration/desktop · [V]
  **perche' conta qui** UI desktop non web
  **cosa se ne prende** build native per i tre OS
- **URL** https://github.com/slint-ui/slint · [V]
  **perche' conta qui** UI dichiarativa in Rust
  **cosa se ne prende** target desktop, embedded, web
- **URL** https://github.com/slint-ui/slint/blob/master/LICENSE.md · [V]
  **perche' conta qui** compatibilita' con repo pubblico
  **cosa se ne prende** royalty-free, GPLv3 o commerciale
- **URL** https://github.com/emilk/egui · [V]
  **perche' conta qui** UI immediate mode in Rust
  **cosa se ne prende** AccessKit, limiti di layout
