---
tipo: tecnica
fonti: [fonti/steam-hw-survey.md, fonti/steam-deck-tech-specs.md, fonti/steamworks-deck-compat.md, fonti/godot-system-requirements.md, fonti/godot-renderers.md, fonti/godot-creating-applications.md, fonti/tauri-linux-graphics.md, fonti/wgpu-github.md]
aggiornato: 2026-10-16
---
# Prestazioni su hardware modesto

Quale hardware puntare e cosa ne segue per la UI densa e per la mappa animata. Il bersaglio realistico sono 8 GB di RAM, una GPU integrata Intel di qualche generazione fa e lo Steam Deck a 1280x800. Il collo di bottiglia vero e' la leggibilita' del testo a bassa risoluzione, piu' che la GPU.

## Il parco macchine (Steam, settembre 2026)

- RAM: 32 GB 42,2%, 16 GB 37,8%, 8 GB 6,5%, 12 GB 2,1%, 4 GB 1,2% (fonti/steam-hw-survey.md) [V]. Chi ha meno di 16 GB e' circa il 10% [INF dalla somma].
- Risoluzione principale: 1920x1080 al 47,9%; 1366x768 all'1,9%; 1280x800 allo 0,2% (fonti/steam-hw-survey.md) [V].
- Sistemi operativi: Windows 95,0%, Linux 3,05%, macOS 1,92% (fonti/steam-hw-survey.md) [V].
- Fra le GPU compaiono integrate Intel (Iris Xe, UHD 620, UHD 630, HD 520) e "Steam Deck GPU" allo 0,28% (fonti/steam-hw-survey.md) [V].

## Steam Deck

- APU AMD a 6 nm, GPU con 8 CU RDNA 2 a 1,6 GHz (1,6 TFLOPS FP32), consumo 4-15 W; 16 GB di LPDDR5 condivisi; schermo 1280x800; SteamOS 3 basato su Arch (fonti/steam-deck-tech-specs.md) [V].
- Requisito di verifica: carattere minimo **9 pixel di altezza a 1280x800**, leggibile a 30 cm (fonti/steamworks-deck-compat.md) [V]. Per una UI densa progettata a 1080p significa un'opzione di scala della UI e layout che si riorganizzano, non solo rimpiccioliti [INF].

## Godot

- Requisiti minimi di un progetto esportato: 2 GB di RAM, integrata con Vulkan 1.0 o OpenGL 3.3 per il renderer Compatibility (fonti/godot-system-requirements.md) [V].
- Compatibility e' il renderer per l'hardware piu' vecchio e modesto; dalla 4.4 c'e' un ripiego automatico verso di lui se manca RenderingDevice (fonti/godot-renderers.md) [V].
- Scala HiDPI automatica solo su macOS, Android e Linux Wayland: su Windows serve un'impostazione manuale (fonti/godot-creating-applications.md) [V].

## UI web (Tauri, Electron)

- Su Linux WebGL e canvas possono girare su un percorso lento senza alcun errore, e WebKitGTK nasconde il nome del renderer; la doc Tauri consiglia un ripiego non-WebGL e un'opzione utente (fonti/tauri-linux-graphics.md) [V]. Il Deck e' Linux.

## Rust nativo (Bevy, egui)

- wgpu copre Vulkan, Metal, D3D12 e GL 3.3/GLES 3.0 (fonti/wgpu-github.md) [V]; il ripiego GL tiene dentro le integrate vecchie [INF].

## Regole pratiche per la UI densa

- Ridisegnare solo cio' che cambia: la UI di un gioco a turni e' quasi sempre ferma tra un input e l'altro [INF].
- Limitare i frame quando la mappa non si anima (30 o 40 fps sul Deck) per batteria e calore, dati i 4-15 W dell'APU [INF].
- Grafici con poche migliaia di punti al massimo per schermata; serie lunghe aggregate dal core, non dal frontend [INF].
- Simulazione in un thread separato: un tick lungo non deve bloccare l'input [INF].
- Bersaglio da misurare: Intel UHD 620 con 8 GB, 1080p, e Steam Deck a 1280x800 [INF].

## Per il nostro gioco

- Prendiamo: Compatibility (o fallback automatico) come base grafica, scala UI configurabile da subito, bersagli di misura fissi.
- Evitiamo: effetti che esistono solo nel renderer avanzato; UI progettata solo per 1080p.
- Aperto: numeri reali di frame e RAM su UHD 620 e Deck con una schermata densa e la mappa animata. Da misurare in uno spike, non da stimare.

## Collegamenti

- [[godot]], [[tauri]]: dove nascono i limiti descritti.
- [[distribuzione-e-aggiornamenti]]: criteri di verifica Deck.
- [[confronto-stack]].

## Fonti

- **URL** https://store.steampowered.com/hwsurvey/ · [V]
  **perche' conta qui** l'hardware reale dei giocatori nel 2026
  **cosa se ne prende** RAM, risoluzioni, quote OS, presenza delle integrate
- **URL** https://www.steamdeck.com/en/tech · [V]
  **perche' conta qui** bersaglio portatile esplicito di Mario
  **cosa se ne prende** 8 CU RDNA 2, 16 GB condivisi, 1280x800, 4-15 W
- **URL** https://partner.steamgames.com/doc/steamdeck/compat · [V]
  **perche' conta qui** vincolo sulla dimensione del testo
  **cosa se ne prende** 9 px minimo a 1280x800
- **URL** https://docs.godotengine.org/en/stable/about/system_requirements.html · [V]
  **perche' conta qui** minimi del motore candidato
  **cosa se ne prende** 2 GB, Vulkan 1.0 o OpenGL 3.3
- **URL** https://docs.godotengine.org/en/stable/tutorials/rendering/renderers.html · [V]
  **perche' conta qui** scelta del renderer
  **cosa se ne prende** Compatibility e fallback automatico
- **URL** https://docs.godotengine.org/en/stable/tutorials/ui/creating_applications.html · [V]
  **perche' conta qui** scala della UI
  **cosa se ne prende** HiDPI manuale su Windows/X11
- **URL** https://v2.tauri.app/develop/debug/linux-graphics/ · [V]
  **perche' conta qui** rischio della UI web su Linux/Deck
  **cosa se ne prende** WebGL lento in silenzio
- **URL** https://github.com/gfx-rs/wgpu · [V]
  **perche' conta qui** copertura hardware dello stack Rust
  **cosa se ne prende** backend GL di ripiego
