---
tipo: sintesi
fonti: [fonti/godot-releases.md, fonti/godot-system-requirements.md, fonti/godot-export-macos.md, fonti/godot-i18n.md, fonti/godot-creating-applications.md, fonti/godot-license.md, fonti/bevy-0-20-release.md, fonti/bevy-sixth-birthday.md, fonti/tauri-releases.md, fonti/tauri-what-is.md, fonti/tauri-linux-graphics.md, fonti/webkit-safari-26-webgpu.md, fonti/electron-timelines.md, fonti/unity-pricing-updates.md, fonti/unity-6-system-requirements.md, fonti/defold-faq.md, fonti/slint-licensing.md, fonti/egui-github.md, fonti/raylib-github.md, fonti/love2d-github.md]
aggiornato: 2026-10-17
---
# Confronto degli stack

> **Superata il 17/10/2026** da [[decisioni]] («stack e linguaggio del core»): Mario ha scelto Tauri 2.12 + core Rust. La direzione artistica da settimanale e il pubblico della v1, solo Mac, hanno tolto peso alla mappa con shader e al Deck.

Tabella dei candidati sui criteri del brief, con la raccomandazione argomentata. Raccomandazione, non decisione: Godot 4.7 come motore, renderer Compatibility come base, e un core di simulazione separato, in Rust via gdext se Mario lo accetta. Seconda scelta Tauri, se uno spike dimostra che la mappa regge su Steam Deck.

## Tabella

Legenda: [V] fonte primaria letta, [M] misurato il 16/10/2026, [INF] inferenza, [NT] non trovato. I dettagli e le fonti di ogni cella sono nelle pagine collegate.

| Stack | Build Win / macOS (notarizzazione) / Linux | Peso runtime | RAM e GPU minime | UI densa e testo (i18n, a11y) | Shader ed effetti | Modding | Licenza | Maturita' al 2026 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| **Godot 4.7** ([[godot]]) | Tutte da un solo OS; notarizzazione anche da Linux con `rcodesign` [V] | Template 28-38 MB compressi, 73-109 MB estratti [M] | 2 GB; integrata Vulkan 1.0 o OpenGL 3.3 [V] | `Control` + container + temi; gettext/CSV, pseudolocalizzazione, BiDi [V]; AccessKit dalla 4.5 [V]; scala HiDPI manuale su Windows [V] | Linguaggio shader proprio; Compatibility limita alcune funzioni [V] | PCK/ZIP o file sciolti; PCK = codice arbitrario [V] | MIT [V] | 4.7.2 stabile, 4.8 beta; minor ogni 4-6 mesi [V] |
| **Bevy 0.20** ([[bevy]]) | Cargo per i tre OS; firma con strumenti generici [INF] | [NT] | wgpu: Vulkan, Metal, D3D12, GL 3.3 [V] | Widget headless e Feathers recenti; niente editor; reattivita' UI non convergente [V]; i18n [NT] | WESL; shader custom piu' complessi per ammissione del lead [V] | Da costruire [INF] | MIT/Apache [M/INF] | Pre-1.0, rompe API ogni 3-6 mesi [V] |
| **Tauri 2.12** ([[tauri]]) | Tre OS; notarizzazione via App Store Connect API [V] | App minima < 600 KB [V] | Dipende dalla webview di sistema [V] | Migliore per testo, tabelle, grafici, a11y [INF]; tre motori diversi [V] | WebGL2; WebGPU solo da macOS 26 e non su WebKitGTK [V/NT]; WebGL lento in silenzio su Linux [V] | Dati nel core Rust; JS dei mod = codice arbitrario [INF] | MIT/Apache [M/INF] | Stabile, release 10/10/2026 [V] |
| **Electron 44** ([[electron]]) | Tre OS; Forge firma e notarizza [V] | 123-159 MB zip [M] | [NT]; Chromium multiprocesso [INF] | Come Tauri, un solo motore [INF] | WebGL2 uniforme [INF] | Come Tauri [INF] | MIT [M] | Major ogni 8 settimane, supporto alle ultime 3 [V] |
| **Unity 6.3** ([[unity]]) | Tre OS; notarizzazione via Xcode/altool [V] | [NT] | Win10, macOS 12, Linux solo Ubuntu 22.04/24.04 [V] | UI Toolkit, Localization [NT] | Completo [INF] | Non nativo [INF] | Proprietaria; gratis fino a 200.000 $ [V] | Maturo; licenza gia' cambiata e ritirata [V] |
| **Defold 1.14** ([[altre-opzioni]]) | Tre OS [V] | [NT] | Ogni Linux 64 bit con driver aggiornati [V] | GUI da gioco, non da applicazione [INF] | Shader del motore [INF] | Lua [V] | Derivata da Apache 2.0, gratis [V] | Attivo [M] |
| **Slint / egui** ([[altre-opzioni]]) | Tre OS [V] | [NT] | [NT] | Slint dichiarativa; egui immediate mode con limiti di layout, AccessKit [V] | Mappa da fare a parte [INF] | Da costruire [INF] | Slint GPLv3/royalty-free/commerciale; egui MIT/Apache [V/M] | Attivi [M] |
| **raylib / LÖVE** ([[altre-opzioni]]) | Tre OS [V] | [NT] | OpenGL [V] | Nessun toolkit UI [V] | Shader GL [INF] | LÖVE: Lua [V] | zlib (raylib) [V] | LÖVE ferma al 2023 [V/M] |
| **Flutter** ([[altre-opzioni]]) | Tre OS nativi [V] | [NT] | [NT] | Ottima [INF] | Non e' un motore di gioco [INF] | Da costruire [INF] | [NT] | Versione [NT] |

## Raccomandazione argomentata

Raccomandazione per Mario, non decisione.

1. **Godot 4.7.x come motore.** E' l'unico candidato che unisce un toolkit UI maturo (pannelli, container, temi, i18n con pseudolocalizzazione, lettori di schermo), shader per la mappa e un renderer OpenGL 3.3 che tiene dentro le integrate vecchie e il Deck. Licenza MIT adatta a un repo pubblico; export e notarizzazione anche da un runner Linux; 28-38 MB compressi.
2. **Renderer Compatibility come base**, o Forward+/Mobile con ripiego automatico, purche' la mappa non dipenda da funzioni assenti in Compatibility. Scala della UI configurabile dal primo giorno, per Windows (niente scala automatica) e per i 9 px del Deck.
3. **Core di simulazione separato** ([[core-rust-e-ui-separata]]). In Rust via gdext e' l'opzione da proporre a Mario: tipi forti, test senza motore, PRNG e interi sotto controllo. Costo: due linguaggi e gdext ancora pre-1.0. Via di mezzo: stesso confine in GDScript, porting in Rust se il profilo lo chiede.
4. **Seconda scelta: Tauri**, se lo spike mostra che la UI web vale il rischio. Vince sul testo e sulle tabelle, perde sulla mappa: su Linux e quindi sul Deck WebGL puo' rallentare senza errori, e WebGPU non e' una base comune.
5. **Scartati per ora**: Electron (peso e aggiornamenti forzati, senza vantaggi rispetto a Tauri), Unity (licenza proprietaria e Linux ufficiale solo Ubuntu, in un progetto pubblico e moddabile), Bevy (UI immatura, niente editor, API che cambiano a ogni release: da rivalutare alla 1.0).

## Cosa misurare prima di decidere

- Uno spike Godot: una schermata densa (tabella, tre grafici, pannelli annidati) piu' la mappa con uno shader, su Intel UHD 620 a 1080p e su Steam Deck a 1280x800, con renderer Compatibility. Frame, RAM, leggibilita' a 9 px.
- Una GDExtension Rust minima compilata per Windows, macOS universale e Linux, firmata e notarizzata da CI Linux.
- Solo se si valuta Tauri: la stessa mappa in WebGL2 dentro WebKitGTK impacchettato, sul Deck.

## Collegamenti

- [[godot]], [[bevy]], [[tauri]], [[electron]], [[unity]], [[altre-opzioni]]: una pagina per stack.
- [[core-rust-e-ui-separata]], [[architettura-simulazione]], [[modding-e-dati-esterni]], [[distribuzione-e-aggiornamenti]], [[prestazioni-su-hardware-modesto]]: pagine trasversali.

## Fonti

Le fonti delle singole celle sono nelle pagine collegate, ciascuna con URL, motivo e cosa se ne prende. Qui le quattro che decidono la raccomandazione:

- **URL** https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_macos.html · [V]
  **perche' conta qui** notarizzazione senza un Mac in CI
  **cosa se ne prende** `rcodesign` da Linux con chiave App Store Connect
- **URL** https://docs.godotengine.org/en/stable/about/system_requirements.html · [V]
  **perche' conta qui** soglia hardware del candidato raccomandato
  **cosa se ne prende** 2 GB, OpenGL 3.3
- **URL** https://v2.tauri.app/develop/debug/linux-graphics/ · [V]
  **perche' conta qui** motivo per cui Tauri e' seconda scelta
  **cosa se ne prende** WebGL lento in silenzio su Linux, mappe e grafici citati
- **URL** https://bevy.org/news/bevys-sixth-birthday/ · [V]
  **perche' conta qui** motivo per cui Bevy e' scartato oggi
  **cosa se ne prende** editor non consegnato, UI reattiva non convergente
