---
tipo: tecnica
fonti: [fonti/tauri-releases.md, fonti/tauri-what-is.md, fonti/tauri-webview-versions.md, fonti/tauri-app-size.md, fonti/tauri-sign-macos.md, fonti/tauri-prerequisites.md, fonti/tauri-linux-graphics.md, fonti/webkit-safari-26-webgpu.md, fonti/webkitgtk-2-52-highlights.md, fonti/webkitgtk-2-54-highlights.md, fonti/steamworks-linux-runtime.md]
aggiornato: 2026-10-16
---
# Tauri 2 (core Rust + UI web)

Applicazione desktop con backend Rust e interfaccia HTML/CSS/JS disegnata dalla webview del sistema operativo. Binario minuscolo e la migliore resa possibile per testo, tabelle e grafici. Il prezzo: tre motori web diversi su tre OS, e su Linux (quindi Steam Deck) la grafica WebGL della mappa e' il punto fragile.

## Stato al 2026

- `tauri` 2.12.3 e `tauri-cli` 2.12.2, entrambi del 10/10/2026 (fonti/tauri-releases.md) [V].
- Licenza: GitHub API rileva Apache-2.0 [M: `gh api repos/tauri-apps/tauri --jq .license.spdx_id`, 16/10/2026]; il doppio MIT/Apache e' la prassi del progetto [INF].

## Peso

- Un'app Tauri minima puo' stare sotto i 600 KB, perche' non include un motore browser (fonti/tauri-what-is.md) [V]. Profilo Cargo consigliato per ridurre ancora: `lto`, `opt-level = "s"`, `panic = "abort"`, `strip` (fonti/tauri-app-size.md) [V].

## Tre webview, tre comportamenti

- **Windows**: WebView2, basato su Chromium, si aggiorna da solo; preinstallato su Windows 11, sulle versioni precedenti lo installa l'installer di Tauri (fonti/tauri-webview-versions.md) [V].
- **macOS**: WKWebView, cioe' il WebKit del sistema: la versione dipende dalla versione di macOS dell'utente (fonti/tauri-webview-versions.md) [V].
- **Linux**: `webkit2gtk` (API 4.1), fornito dalla distribuzione (fonti/tauri-webview-versions.md, fonti/tauri-prerequisites.md) [V].

### WebGPU e WebGL

- WebKit ha attivato WebGPU in Safari 26.0 (15/09/2025) per macOS 26, iOS 26 e visionOS 26 (fonti/webkit-safari-26-webgpu.md) [V]. Su macOS piu' vecchi WKWebView non ha WebGPU [INF].
- WebKitGTK: le note delle versioni 2.52 (marzo 2026) e 2.54 (settembre 2026) non citano WebGPU (fonti/webkitgtk-2-52-highlights.md, fonti/webkitgtk-2-54-highlights.md) [V]; nessuna fonte trovata che lo dia attivo [NON TROVATO]. WebGPU quindi non e' una base comune: la mappa va scritta in WebGL2 o Canvas 2D [INF].
- Su Linux la stessa doc Tauri avverte: con GPU NVIDIA la finestra puo' restare bianca o sfarfallare (renderer DMABUF di WebKitGTK); **WebGL e canvas possono finire in silenzio su un percorso lento**, la creazione del contesto WebGL2 riesce anche se dietro c'e' un rasterizzatore software, e WebKitGTK maschera la stringa del renderer, cosi' non si sa cosa c'e' sotto. I sintomi citati riguardano proprio "mappe, grafici"; la raccomandazione e' un ripiego non-WebGL su Linux e un'opzione per l'utente (fonti/tauri-linux-graphics.md) [V].

### Steam Deck

- Lo Steam Linux Runtime include SDL, OpenAL, FreeType, Fontconfig; le altre dipendenze vanno ridistribuite con il gioco (fonti/steamworks-linux-runtime.md) [V]. WebKitGTK non e' nell'elenco: va impacchettato oppure si usa la build Windows sotto Proton, con WebView2 da far funzionare sotto Proton [INF, non verificato].

## Firma e notarizzazione macOS

- Certificato Developer ID dall'account Apple; notarizzazione con credenziali App Store Connect API o Apple ID tramite variabili d'ambiente durante `tauri build`; disponibile anche la firma ad-hoc, senza identita' Apple (fonti/tauri-sign-macos.md) [V].

## UI densa, i18n, accessibilita'

- HTML/CSS e' il terreno piu' maturo per testo lungo, tabelle, grafici (D3, ECharts e simili), font fallback, localizzazione (`Intl`) e accessibilita' (albero di accessibilita' della webview) [INF: conoscenza generale, nessuna fonte catturata]. E' il vero argomento a favore di Tauri per questo gioco.
- Incoerenze tra motori (CSS, font, prestazioni del canvas) da testare su tutte e tre le webview [INF].

## Modding

- Dati in file testuali letti dal core Rust; scripting nel core (Rhai, Lua) o in JS nella webview. JS dei mod nella webview equivale a codice arbitrario [INF]. Vedi [[modding-e-dati-esterni]].

## Per il nostro gioco

- Prendiamo: l'architettura core Rust separato dalla UI (vedi [[core-rust-e-ui-separata]]), valida anche se il frontend non e' Tauri.
- Evitiamo: WebGPU come requisito; una mappa che dipende da WebGL veloce su Linux senza ripiego.
- Aperto: una mappa Canvas 2D/WebGL2 regge su Steam Deck con WebKitGTK impacchettato? Va misurato prima di scegliere Tauri.

## Collegamenti

- [[electron]]: stessa UI web con Chromium uniforme, ma 120-160 MB.
- [[godot]]: alternativa con renderer proprio uguale su tutti gli OS.
- [[prestazioni-su-hardware-modesto]]: Steam Deck e integrate.
- [[confronto-stack]].

## Fonti

- **URL** https://v2.tauri.app/release/ · [V]
  **perche' conta qui** maturita' e attivita' del progetto
  **cosa se ne prende** 2.12.3 del 10/10/2026
- **URL** https://v2.tauri.app/start/ · [V]
  **perche' conta qui** peso dell'app
  **cosa se ne prende** app minima sotto 600 KB grazie alla webview di sistema
- **URL** https://v2.tauri.app/concept/size/ · [V]
  **perche' conta qui** come ridurre ancora il binario
  **cosa se ne prende** profilo Cargo di release
- **URL** https://v2.tauri.app/reference/webview-versions/ · [V]
  **perche' conta qui** i tre motori web e da cosa dipendono
  **cosa se ne prende** WebView2 evergreen, WKWebView legato a macOS, webkit2gtk di distro
- **URL** https://v2.tauri.app/start/prerequisites/ · [V]
  **perche' conta qui** dipendenze di build e runtime
  **cosa se ne prende** `libwebkit2gtk-4.1`, WebView2 su Windows
- **URL** https://v2.tauri.app/develop/debug/linux-graphics/ · [V]
  **perche' conta qui** il rischio piu' grave per la mappa animata
  **cosa se ne prende** WebGL lento in silenzio su Linux, schermo bianco su NVIDIA, ripiego non-WebGL
- **URL** https://v2.tauri.app/distribute/sign/macos/ · [V]
  **perche' conta qui** notarizzazione
  **cosa se ne prende** App Store Connect API o Apple ID, firma ad-hoc
- **URL** https://webkit.org/blog/17333/webkit-features-in-safari-26-0/ · [V]
  **perche' conta qui** WebGPU su macOS
  **cosa se ne prende** solo da Safari 26 / macOS 26
- **URL** https://webkitgtk.org/2026/03/18/webkitgtk-2.52-highlights.html · [V]
  **perche' conta qui** stato di WebKitGTK su Linux
  **cosa se ne prende** nessuna menzione di WebGPU
- **URL** https://webkitgtk.org/2026/09/16/webkitgtk-2.54-highlights.html · [V]
  **perche' conta qui** ultima versione di WebKitGTK
  **cosa se ne prende** nuovo compositore, ancora nessuna menzione di WebGPU
- **URL** https://partner.steamgames.com/doc/store/application/platforms/linux · [V]
  **perche' conta qui** cosa c'e' nel runtime Steam su Linux e Deck
  **cosa se ne prende** solo SDL, OpenAL, FreeType, Fontconfig inclusi; il resto si ridistribuisce
