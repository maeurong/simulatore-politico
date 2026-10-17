---
tipo: tecnica
fonti: [fonti/unity-pricing-updates.md, fonti/unity-cancel-runtime-fee.md, fonti/unity-6-system-requirements.md, fonti/unity-macos-notarization.md, fonti/itch-lawgivers2.md]
aggiornato: 2026-10-16
---
# Unity 6

Motore commerciale a codice chiuso; Lawgivers II e' fatto con Unity (fonti/itch-lawgivers2.md). Tecnicamente copre tutto, ma la licenza proprietaria, la storia recente dei prezzi e il codice chiuso stridono con un repo pubblico e un gioco moddabile.

## Stato al 2026

- Documentazione corrente: Unity 6.3 (`6000.3`) (fonti/unity-6-system-requirements.md) [V].
- La "runtime fee" e' stata cancellata a settembre 2024: ritorno al modello ad abbonamento per posto, con l'impegno a valutare aumenti di prezzo solo una volta l'anno (fonti/unity-cancel-runtime-fee.md) [V].

## Licenza e costi

- Personal gratuito fino a 200.000 $ di ricavi e finanziamenti annui; con Unity 6 lo splash screen e' opzionale (fonti/unity-pricing-updates.md) [V].
- Pro: 2.310 $ l'anno per posto dal 12 gennaio 2026 (+5%), obbligatorio sopra i 200.000 $ (fonti/unity-pricing-updates.md) [V].
- Havok Physics esce dai piani a partire da Unity 6.3 (fonti/unity-pricing-updates.md) [V]: esempio di come il perimetro dell'offerta cambi da una versione all'altra.
- Editor e runtime proprietari: chi clona il repo pubblico deve installare Unity e accettarne i termini [INF].

## Build e hardware minimo del gioco esportato

- Windows 10 21H1 o successivo, GPU DX10/DX11/DX12 o Vulkan; macOS 12 Monterey o successivo, GPU Metal; **Linux: solo Ubuntu 22.04 e 24.04 ufficiali**, OpenGL 3.2+ o Vulkan, GNOME su X11 o Wayland (fonti/unity-6-system-requirements.md) [V]. SteamOS non e' nell'elenco [INF dall'elenco].
- Wayland per il Linux desktop e' ancora sperimentale (fonti/unity-6-system-requirements.md) [V].
- Firma e notarizzazione macOS documentate, con Xcode o `altool` (fonti/unity-macos-notarization.md) [V]. Da Linux: [NON TROVATO].
- Peso di un build vuoto: [NON TROVATO] in fonte ufficiale.

## UI, i18n, accessibilita', shader

- UI Toolkit, pacchetto Localization e modulo di accessibilita' esistono, ma non sono stati verificati su fonte in questa ricerca [NON TROVATO].
- Shader Graph e URP coprono gli effetti richiesti [INF].

## Modding

- Il modding di un gioco Unity passa di solito per file dati esterni o per strumenti di terze parti che caricano assembly; senza supporto ufficiale del motore [INF]. Lawgivers II offre mod con editor di testo su paesi, leggi, eventi (vedi [[lawgivers-2]]).

## Rust

- Un core Rust si collega come plugin nativo via ABI C [INF]; nessun binding ufficiale.

## Per il nostro gioco

- Prendiamo: la conferma, da Lawgivers II, che un gioco del genere si regge su un motore generalista.
- Evitiamo: dipendere da termini commerciali che il fornitore ha gia' cambiato una volta in corsa, in un progetto pubblico.
- Aperto: niente che giustifichi Unity rispetto a Godot per questo genere.

## Collegamenti

- [[lawgivers-2]]: gioco politico fatto con Unity.
- [[godot]]: l'alternativa open source con gli stessi punti di forza per la UI.
- [[confronto-stack]].

## Fonti

- **URL** https://unity.com/products/pricing-updates · [V]
  **perche' conta qui** costi e soglie al 2026
  **cosa se ne prende** Personal gratis fino a 200.000 $, Pro 2.310 $/anno, splash opzionale
- **URL** https://unity.com/blog/unity-is-canceling-the-runtime-fee · [V]
  **perche' conta qui** rischio di cambi di licenza
  **cosa se ne prende** runtime fee cancellata, aumenti solo annuali
- **URL** https://docs.unity3d.com/6000.3/Documentation/Manual/system-requirements.html · [V]
  **perche' conta qui** piattaforme e GPU supportate
  **cosa se ne prende** Linux ufficiale solo Ubuntu 22.04/24.04, Wayland sperimentale
- **URL** https://docs.unity3d.com/6000.3/Documentation/Manual/macos-building-notarization.html · [V]
  **perche' conta qui** notarizzazione
  **cosa se ne prende** percorso Xcode/altool, nessun percorso da Linux documentato
