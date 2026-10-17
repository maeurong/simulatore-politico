---
tipo: tecnica
fonti: [fonti/godot-releases.md, fonti/godot-4-7-release.md, fonti/godot-4-5-release.md, fonti/godot-4-5-beta1.md, fonti/godot-system-requirements.md, fonti/godot-renderers.md, fonti/godot-export-macos.md, fonti/godot-license.md, fonti/godot-i18n.md, fonti/godot-optimizing-size.md, fonti/godot-creating-applications.md, fonti/godot-exporting-packs.md, fonti/godot-rust-book.md, fonti/gdext-github.md]
aggiornato: 2026-10-16
---
# Godot 4.x

Motore open source MIT con un sistema di UI maturo (nodi `Control`, container, temi), tre renderer di cui uno OpenGL 3.3 per l'hardware vecchio, export verso Windows, macOS e Linux da qualsiasi sistema operativo. Per un gioco che e' soprattutto pannelli, grafici e testo con una mappa animata e' il candidato che copre piu' requisiti con meno lavoro di base.

## Stato al 2026

- Ultima stabile: 4.7.2; la 4.7 e' uscita il 18 giugno 2026; la 4.8 e' in beta1 (fonti/godot-releases.md) [V]. Gli asset della 4.7.2 sono stati pubblicati il 18/08/2026 [M: `gh api repos/godotengine/godot-builds/releases/tags/4.7.2-stable --jq .published_at`, 16/10/2026].
- Cadenza: una minor ogni 4-6 mesi circa (4.6 a gennaio 2026, 4.7 a giugno 2026) [INF dalle date in fonti/godot-releases.md].
- Licenza MIT: uso libero anche commerciale; l'unico obbligo e' riportare la nota di copyright e la licenza, per esempio con un link nei crediti. Il gioco resta di chi lo fa (fonti/godot-license.md) [V].

## Rendering e hardware minimo

- Tre renderer: Forward+ e Mobile (Vulkan, Direct3D 12 o Metal) e Compatibility (OpenGL 3.3 / ES 3.0), quest'ultimo "per l'hardware desktop e mobile piu' vecchio o modesto" (fonti/godot-renderers.md) [V].
- Dalla 4.4, se Vulkan manca il motore prova Direct3D 12 e, se manca anche quello, ripiega su Compatibility; il progetto gira ma puo' apparire diverso (fonti/godot-renderers.md) [V]. Conseguenza: gli effetti della mappa vanno pensati perche' reggano anche su Compatibility.
- Requisiti minimi di un progetto esportato "semplice": 2 GB di RAM, 150 MB di disco, GPU integrata con Vulkan 1.0 (esempio Intel HD 510) oppure OpenGL 3.3 per Compatibility; Windows 10, macOS 10.13 (Compatibility) o 10.15/13 (Vulkan/Metal), Linux uscito dopo il 2018 (fonti/godot-system-requirements.md) [V]. La pagina avverte che i requisiti reali dipendono dal progetto e raccomanda test su hardware modesto e un menu di opzioni grafiche (stessa fonte) [V].
- 4.7 aggiunge l'uscita HDR su Windows, macOS e Linux (Wayland) (fonti/godot-4-7-release.md) [V]; la 4.5 il buffer stencil (fonti/godot-4-5-release.md) [V].

## Peso del binario

- Template di export 4.7.2 misurati dal pacchetto ufficiale: Linux x86_64 73,5 MB (28,3 MB compresso), Windows x86_64 109,3 MB (37,9 MB compresso), `macos.zip` universale 123,6 MB [M: lettura della directory centrale di `Godot_v4.7.2-stable_export_templates.tpz` via richiesta HTTP a intervallo, 16/10/2026]. Il pacchetto del gioco aggiunge i propri asset.
- Riduzioni possibili ricompilando il template: `disable_3d=yes` toglie circa il 15%; il text server di ripiego (solo latino, greco, cirillico) sostituisce quello avanzato; lo stripping dei simboli e' gia' fatto nelle build ufficiali (fonti/godot-optimizing-size.md) [V].
- Steam comprime da se' i file; lo ZIP ottimizzato serve solo per itch.io e GitHub Releases (fonti/godot-optimizing-size.md) [V].
- 4.7 introduce il download dei template per singola piattaforma, invece del pacchetto intero da 1,3 GB (fonti/godot-4-7-release.md) [V].

## Build Windows, macOS, Linux

- Si esporta verso le tre piattaforme da un solo sistema. Un `.app` esportato da Windows perde il bit eseguibile: conviene esportare in `.zip` (fonti/godot-export-macos.md) [V].
- Firma e notarizzazione macOS: da macOS con `codesign` e `notarytool`; **da Linux o Windows con `rcodesign`** e una chiave API di App Store Connect, staple compreso. Serve comunque un certificato Developer ID (fonti/godot-export-macos.md) [V]. Quindi la pipeline di rilascio puo' girare su un runner Linux.
- Le estensioni GDExtension (per esempio un core Rust) richiedono l'entitlement "Disable Library Validation" quando si usano add-on GDExtension o firma ad-hoc (fonti/godot-export-macos.md) [V].
- Linux: esiste una BaseApp Godot per Flathub (fonti/godot-creating-applications.md) [V].

## UI densa e testo

- Godot e' usato anche per applicazioni non di gioco (Material Maker, Pixelorama), con finestre multiple e dialoghi file nativi (fonti/godot-creating-applications.md) [V].
- Scala HiDPI: il fattore di scala del sistema viene letto solo su macOS, Android e Linux Wayland; **su Windows e X11 serve un'opzione di scala manuale nel gioco** (fonti/godot-creating-applications.md) [V]. Va prevista comunque, anche per la leggibilita' su Steam Deck (vedi [[prestazioni-su-hardware-modesto]]).
- 4.5: `FoldableContainer` per pannelli richiudibili e `DPITexture` (SVG rasterizzati alla scala corrente) (fonti/godot-4-5-beta1.md) [V]: due mattoni diretti per una UI densa che deve scalare.
- 4.7: trasformazione di offset per i nodi `Control` dentro i container, puramente visiva per default, utile ad animare pannelli senza rompere il layout (fonti/godot-4-7-release.md) [V].

### i18n

- Traduzioni con gettext o CSV; **pseudolocalizzazione** integrata per verificare che la UI regga stringhe piu' lunghe (fonti/godot-i18n.md) [V].
- Testo bidirezionale e specchiatura della UI per le lingue RTL automatici; i dati ICU per le lingue senza spazi (circa 4 MB) vanno inclusi a mano nell'export (fonti/godot-i18n.md) [V].

### Accessibilita'

- Dalla 4.5 Godot integra AccessKit per i lettori di schermo sui nodi `Control` (fonti/godot-4-5-release.md, fonti/godot-4-5-beta1.md) [V]. Si attiva da solo se rileva un lettore di schermo; per un livello d'uso buono servono etichette `accessibility_name`/`accessibility_description` e test con NVDA, VoiceOver, Orca (fonti/godot-creating-applications.md) [V].
- L'annuncio della 4.5 beta1 dice il supporto "integrato nel core" e completo su tutte le piattaforme desktop (fonti/godot-4-5-beta1.md) [V]; una sintesi di ricerca sulle note finali della 4.5 lo descrive come sperimentale, ma quella parte della pagina 4.5 si carica via JS e non e' nella cattura [NON TROVATO]. Lo stato alla 4.7 va verificato a mano con NVDA e Orca.

## Shader ed effetti

- Linguaggio shader proprio, simile a GLSL; la 4.7 aggiunge le anteprime inline nell'editor degli shader testuali e i gradienti conici in `GradientTexture2D` (fonti/godot-4-7-release.md) [V].
- La tabella delle funzioni per renderer elenca cosa manca in Compatibility (fonti/godot-renderers.md) [V]: per una mappa 2D con shader di canvas il limite pratico e' basso [INF].

## Modding

- Caricamento a runtime di pacchetti PCK o ZIP per patch, DLC e mod; la doc avverte che un mod puo' contenere codice malevolo e suggerisce di firmare i pacchetti con crittografia asimmetrica (fonti/godot-exporting-packs.md) [V].
- In alternativa si leggono a runtime file sciolti (dati testuali) senza passare dai PCK (fonti/godot-exporting-packs.md) [V]. Dettagli in [[modding-e-dati-esterni]].

## Rust con GDExtension

- godot-rust (gdext) e' il binding Rust per Godot 4 via GDExtension; progetto **della comunita'**, mantenuto indipendentemente da Godot ma in contatto con gli sviluppatori del motore (fonti/godot-rust-book.md) [V].
- Rust e GDScript si mescolano nello stesso progetto, e le API Rust si chiamano da GDScript in modo tipizzato (fonti/gdext-github.md) [V].
- Ultima release v0.5.5 del 09/08/2026, quindi ancora pre-1.0 [M: `gh api repos/godot-rust/gdext/releases --jq '.[0]'`, 16/10/2026].

## Per il nostro gioco

- Prendiamo: UI `Control` con temi, pseudolocalizzazione, AccessKit, export e notarizzazione da CI Linux, renderer Compatibility come base per integrate e Steam Deck.
- Evitiamo: affidarsi alla scala automatica su Windows (non c'e'); effetti che esistono solo in Forward+.
- Aperto: misurare la UI densa su Intel UHD 620 e Steam Deck; maturita' reale di AccessKit nella 4.7; se il core va in Rust via gdext (vedi [[core-rust-e-ui-separata]]).

## Collegamenti

- [[confronto-stack]]: la tabella che mette Godot accanto alle alternative.
- [[core-rust-e-ui-separata]]: Godot come frontend di un core Rust.
- [[prestazioni-su-hardware-modesto]]: renderer Compatibility e target hardware.
- [[distribuzione-e-aggiornamenti]]: Steam, itch.io, Flathub, firma.
- [[modding-e-dati-esterni]]: PCK, file testuali, scripting.

## Fonti

- **URL** https://godotengine.org/download/archive/ · [V]
  **perche' conta qui** date e stato delle versioni al 2026
  **cosa se ne prende** 4.7.2 stabile, 4.8 in beta, cadenza delle minor
- **URL** https://godotengine.org/releases/4.7/ · [V]
  **perche' conta qui** novita' della versione da cui partiremmo
  **cosa se ne prende** HDR, offset dei `Control`, download template per piattaforma
- **URL** https://godotengine.org/releases/4.5/ · [V]
  **perche' conta qui** introduce i lettori di schermo
  **cosa se ne prende** AccessKit sui `Control`, stencil buffer
- **URL** https://godotengine.org/article/dev-snapshot-godot-4-5-beta-1/ · [V]
  **perche' conta qui** dettaglio dell'integrazione accessibilita' e fine supporto Windows 7/8.1
  **cosa se ne prende** accessibilita' nel core, Windows 10 come minimo
- **URL** https://docs.godotengine.org/en/stable/about/system_requirements.html · [V]
  **perche' conta qui** requisiti minimi dell'eseguibile esportato
  **cosa se ne prende** 2 GB RAM, Intel HD 510 o OpenGL 3.3, OS minimi
- **URL** https://docs.godotengine.org/en/stable/tutorials/rendering/renderers.html · [V]
  **perche' conta qui** scelta del renderer per hardware modesto
  **cosa se ne prende** Compatibility come base, catena di fallback dalla 4.4
- **URL** https://docs.godotengine.org/en/stable/tutorials/export/exporting_for_macos.html · [V]
  **perche' conta qui** firma e notarizzazione macOS
  **cosa se ne prende** `rcodesign` da Linux, entitlement per GDExtension
- **URL** https://godotengine.org/license/ · [V]
  **perche' conta qui** compatibilita' con un repo pubblico e un gioco venduto
  **cosa se ne prende** MIT, obbligo di nota nei crediti
- **URL** https://docs.godotengine.org/en/stable/tutorials/i18n/internationalizing_games.html · [V]
  **perche' conta qui** i18n della UI densa
  **cosa se ne prende** gettext/CSV, pseudolocalizzazione, BiDi, dati ICU opzionali
- **URL** https://docs.godotengine.org/en/stable/engine_details/development/compiling/optimizing_for_size.html · [V]
  **perche' conta qui** peso del binario
  **cosa se ne prende** `disable_3d` (-15%), text server di ripiego, ZIP solo fuori da Steam
- **URL** https://docs.godotengine.org/en/stable/tutorials/ui/creating_applications.html · [V]
  **perche' conta qui** Godot per interfacce dense, non solo giochi
  **cosa se ne prende** scala HiDPI manuale su Windows/X11, lettori di schermo, BaseApp Flathub
- **URL** https://docs.godotengine.org/en/stable/tutorials/export/exporting_pcks.html · [V]
  **perche' conta qui** modding via pacchetti
  **cosa se ne prende** PCK/ZIP a runtime, rischio codice malevolo, firma dei pacchetti
- **URL** https://godot-rust.github.io/book/ · [V]
  **perche' conta qui** affidabilita' del ponte Rust
  **cosa se ne prende** progetto comunitario indipendente da Godot
- **URL** https://github.com/godot-rust/gdext · [V]
  **perche' conta qui** come si integra un core Rust
  **cosa se ne prende** classi Rust chiamabili da GDScript, tipi sicuri
