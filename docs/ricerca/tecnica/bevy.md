---
tipo: tecnica
fonti: [fonti/bevy-news.md, fonti/bevy-0-20-release.md, fonti/bevy-sixth-birthday.md, fonti/bevy-egui-github.md, fonti/egui-github.md, fonti/wgpu-github.md, fonti/bevy-fixed-timestep-example.md]
aggiornato: 2026-10-16
---
# Bevy

Motore Rust interamente basato su ECS, open source, senza editor ufficiale. Ottimo per una simulazione scritta in Rust; debole proprio dove il nostro gioco pesa di piu', la UI densa. A ottobre 2026 e' alla 0.20 e rompe le API a ogni release.

## Stato al 2026

- Release recenti: 0.18 (13/01/2026), 0.19 (19/06/2026), 0.20 (08/10/2026) (fonti/bevy-news.md, fonti/bevy-0-20-release.md) [V]. Ogni release ha la sua guida di migrazione, per esempio "0.19 to 0.20 Migration Guide" (fonti/bevy-0-20-release.md) [V]: si aggiorna con lavoro manuale ogni 3-6 mesi [INF].
- Editor: nel post del sesto compleanno (10/08/2026) il project lead scrive che l'editor di base **non** e' stato consegnato e che lo annuncia "per l'anno prossimo" da molti anni; esiste un esperimento comunitario, Jackdaw (fonti/bevy-sixth-birthday.md) [V].
- Reattivita' della UI: esistono implementazioni comunitarie, ma "non abbiamo ancora trovato l'implementazione upstream corretta" (fonti/bevy-sixth-birthday.md) [V].
- Licenza: GitHub API rileva Apache-2.0 [M: `gh api repos/bevyengine/bevy --jq .license.spdx_id`, 16/10/2026]; il doppio MIT/Apache e' la prassi del progetto [INF].

## UI densa e testo

- `bevy_ui` ha oggi widget "headless" di base, il toolkit Feathers ("opinionated, editor-centric", pensato per strumenti di sviluppo) e la sintassi di scena BSN, che il lead indica come chiave della storia UI (fonti/bevy-sixth-birthday.md) [V].
- La 0.20 aggiunge widget Feathers (input colore, lista scorrevole, dropdown, menu), schede headless integrate con focus e accessibilita', unita' `Em`/`Rem`, temi contestuali, `InlineBox`/`InlineImage` per immagini nel testo (fonti/bevy-0-20-release.md) [V]. Sono mattoni appena arrivati: per tabelle, grafici e pannelli annidati si costruisce molto a mano [INF].
- Alternativa: `bevy_egui` 0.43.1 per Bevy 0.20, con supporto AccessKit (fonti/bevy-egui-github.md) [V]. egui e' immediate mode: il layout complesso e' difficile e certi elementi si posizionano con un frame di ritardo (fonti/egui-github.md) [V]. Aspetto da strumento, non da gioco, salvo molto lavoro di stile [INF].
- i18n: nessun sistema di localizzazione nelle fonti ufficiali lette [NON TROVATO]; esistono crate comunitari basati su Fluent [INF].

## Rendering, shader, hardware

- Bevy disegna con wgpu, che gira su Vulkan, Metal, D3D12 e OpenGL (GL 3.3+ o GLES 3.0+) (fonti/wgpu-github.md) [V]. Il backend GL copre le integrate vecchie [INF].
- La 0.20 adotta WESL (estensione di WGSL) per gli shader e permette materiali shader custom per gli sprite e materiali 2D estendibili (fonti/bevy-0-20-release.md) [V]. Il lead stesso ammette che il lavoro sul renderer ha reso piu' complesso lo shader custom lato utente (fonti/bevy-sixth-birthday.md) [V].
- RAM e GPU minime ufficiali: [NON TROVATO].

## Build e peso

- Binario Rust statico, nessun runtime esterno [INF]. Peso tipico: [NON TROVATO] in fonti ufficiali.
- Firma e notarizzazione macOS: nessuno strumento ufficiale Bevy; si usano gli strumenti generici (`codesign`/`notarytool` o `rcodesign`) [INF].

## Simulazione

- ECS nativo e schedule a timestep fisso (`FixedUpdate`) con esempio ufficiale che separa logica a passo fisso e rendering interpolato (fonti/bevy-fixed-timestep-example.md) [V]. Vedi [[architettura-simulazione]].

## Modding

- Nessun sistema di mod integrato; si aggiunge scripting via crate (mlua, Rhai) [INF]. Vedi [[modding-e-dati-esterni]].

## Per il nostro gioco

- Prendiamo: l'idea di schedule a passo fisso e l'ECS come riferimento per il core, se mai servisse.
- Evitiamo: Bevy come motore principale oggi. La UI densa e' la meta' del gioco, e Bevy non ha ancora editor, reattivita' UI stabile e API ferme.
- Aperto: rivalutare quando escono editor e 1.0.

## Collegamenti

- [[confronto-stack]]: posizione di Bevy nel confronto.
- [[core-rust-e-ui-separata]]: un core Rust puro non ha bisogno di Bevy.
- [[altre-opzioni]]: egui da solo, senza Bevy.

## Fonti

- **URL** https://bevy.org/news/ · [V]
  **perche' conta qui** cadenza delle release
  **cosa se ne prende** date di 0.18, 0.19, 0.20
- **URL** https://bevy.org/news/bevy-0-20/ · [V]
  **perche' conta qui** stato della UI all'ultima release
  **cosa se ne prende** widget Feathers, `Em`/`Rem`, WESL, guida di migrazione per release
- **URL** https://bevy.org/news/bevys-sixth-birthday/ · [V]
  **perche' conta qui** maturita' dichiarata dal project lead
  **cosa se ne prende** editor non consegnato, reattivita' UI non convergente
- **URL** https://github.com/mvlabat/bevy_egui · [V]
  **perche' conta qui** alternativa UI matura sopra Bevy
  **cosa se ne prende** versione 0.43.1 per Bevy 0.20, AccessKit
- **URL** https://github.com/emilk/egui · [V]
  **perche' conta qui** limiti dell'immediate mode per layout complessi
  **cosa se ne prende** ritardo di un frame nel layout, AccessKit
- **URL** https://github.com/gfx-rs/wgpu · [V]
  **perche' conta qui** backend grafici e copertura hardware
  **cosa se ne prende** Vulkan, Metal, D3D12, GL 3.3/GLES 3.0
- **URL** https://bevy.org/examples/movement/physics-in-fixed-timestep/ · [V]
  **perche' conta qui** separazione simulazione/rendering
  **cosa se ne prende** `FixedUpdate` e interpolazione
