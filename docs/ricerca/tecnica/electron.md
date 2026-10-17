---
tipo: tecnica
fonti: [fonti/electron-timelines.md, fonti/electron-code-signing.md, fonti/electron-performance.md]
aggiornato: 2026-10-16
---
# Electron

UI web con Chromium e Node.js impacchettati nell'app. Stesso motore su tutti gli OS, quindi niente incoerenze tra webview; in cambio ogni copia del gioco porta con se' un browser intero. Per un gioco che deve essere "leggero" e' la scelta piu' pesante del confronto.

## Stato al 2026

- Ultima stabile v44.8.0 pubblicata il 16/10/2026 [M: `gh api repos/electron/electron/releases/latest`, 16/10/2026].
- Una major ogni 8 settimane; supporto ufficiale solo alle ultime 3 stabili (fonti/electron-timelines.md) [V]. Ogni versione e' quindi supportata circa sei mesi: aggiornamenti obbligati e frequenti [INF].
- Licenza MIT [M: `gh api repos/electron/electron --jq .license.spdx_id`, 16/10/2026].

## Peso

- Runtime v44.8.0 compresso, prima del codice del gioco: Linux x64 123,4 MB, macOS arm64 130,6 MB, Windows x64 158,5 MB [M: dimensioni degli asset `electron-v44.8.0-*.zip` via `gh api`, 16/10/2026].

## Build e firma

- macOS: firma e notarizzazione integrate in Electron Forge (`osxSign`, `osxNotarize`) o in `@electron/packager` con `@electron/osx-sign` e `@electron/notarize`; alcune API di sistema funzionano bene solo con app firmata e notarizzata (fonti/electron-code-signing.md) [V].
- Windows: la doc indica Azure Artifact Signing (ex Trusted Signing) come l'opzione piu' economica, che elimina gli avvisi SmartScreen (fonti/electron-code-signing.md) [V]. Disponibilita' per sviluppatori individuali fuori da USA e Canada: [NON TROVATO] in fonte ufficiale; vedi [[distribuzione-e-aggiornamenti]].

## Prestazioni

- La guida ufficiale alle prestazioni e' qualitativa: misurare, profilare, caricare meno moduli, non bloccare il processo principale (fonti/electron-performance.md) [V]. Numeri ufficiali di RAM minima: [NON TROVATO].
- Chromium multiprocesso: piu' memoria di base di un motore di gioco nativo [INF].

## UI, shader, i18n, accessibilita'

- Gli stessi vantaggi della UI web descritti in [[tauri]] (testo, grafici, `Intl`, accessibilita'), con un solo motore da testare [INF].
- WebGL2 uniforme ovunque; WebGPU dipende da cosa Chromium abilita per piattaforma e driver [INF, non verificato su Linux].

## Modding

- Come Tauri: dati testuali; JS dei mod equivale a codice arbitrario con accesso a Node se non isolato [INF].

## Per il nostro gioco

- Prendiamo: niente di specifico che Tauri o Godot non diano gia'.
- Evitiamo: 120-160 MB di runtime e un aggiornamento di sicurezza ogni 8 settimane per un gioco offline e leggero.
- Aperto: nulla; resta come ripiego se Tauri fallisce solo per incoerenze delle webview.

## Collegamenti

- [[tauri]]: stessa UI web, binario di qualche MB.
- [[confronto-stack]].

## Fonti

- **URL** https://www.electronjs.org/docs/latest/tutorial/electron-timelines · [V]
  **perche' conta qui** costo di manutenzione nel tempo
  **cosa se ne prende** major ogni 8 settimane, supporto alle ultime 3
- **URL** https://www.electronjs.org/docs/latest/tutorial/code-signing · [V]
  **perche' conta qui** firma su macOS e Windows
  **cosa se ne prende** Forge con notarizzazione, Artifact Signing contro SmartScreen
- **URL** https://www.electronjs.org/docs/latest/tutorial/performance · [V]
  **perche' conta qui** costo su hardware modesto
  **cosa se ne prende** nessun numero ufficiale, solo pratiche di profilazione
