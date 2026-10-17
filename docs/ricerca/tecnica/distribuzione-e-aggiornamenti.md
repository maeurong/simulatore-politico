---
tipo: tecnica
fonti: [fonti/steamworks-steam-direct.md, fonti/steamworks-platforms.md, fonti/steamworks-deck-compat.md, fonti/steamworks-linux-runtime.md, fonti/itch-butler.md, fonti/github-releases-limits.md, fonti/flathub-requirements.md, fonti/godot-creating-applications.md, fonti/apple-developer-program.md, fonti/electron-code-signing.md, fonti/ms-artifact-signing-overview.md, fonti/ms-artifact-signing.md, fonti/godot-optimizing-size.md]
aggiornato: 2026-10-16
---
# Distribuzione e aggiornamenti

Steam, itch.io, GitHub Releases e Flathub, con i costi fissi di firma. Steam richiede la notarizzazione macOS ma non la firma Windows; per i download diretti senza firma Windows l'utente vede l'avviso SmartScreen.

## Steam

- 100 $ per prodotto, non rimborsabili ma recuperati dopo 1.000 $ di ricavo lordo rettificato; 30 giorni di attesa tra pagamento e uscita (fonti/steamworks-steam-direct.md) [V].
- macOS: dal 14/10/2019 Steam richiede app macOS a 64 bit e **notarizzate** da Apple; Steam non e' compatibile con l'entitlement sandbox (fonti/steamworks-platforms.md) [V].
- Linux: si sceglie una versione dello Steam Linux Runtime per branch; il runtime include SDL, OpenAL, FreeType, Fontconfig, il resto si ridistribuisce (fonti/steamworks-linux-runtime.md) [V].
- Steam Deck e Steam Machine: criteri di verifica comuni (controller, testo, launcher). Per il Deck: testo leggibile a 30 cm, **il carattere piu' piccolo mai sotto i 9 pixel di altezza a 1280x800**; glifi del controller corretti; tastiera a schermo per l'input di testo; nessun avviso "hardware non supportato"; launcher sconsigliati (fonti/steamworks-deck-compat.md) [V].
- Se esiste una build Linux nativa la verifica parte da quella; se fallisce si prova la build Windows sotto Proton (fonti/steamworks-deck-compat.md) [V].
- Steam comprime da se' i file, non serve impacchettare in ZIP (fonti/godot-optimizing-size.md) [V]. Aggiornamenti: gestiti da Steam [INF].

## itch.io

- `butler` carica le build, copia build da Steam a itch.io e genera patch applicabili offline; open source MIT, basato sulla specifica aperta Wharf; integrabile in pipeline automatiche (fonti/itch-butler.md) [V].
- Aggiornamenti differenziali via app itch [INF].

## GitHub Releases

- Fino a 1.000 asset per release, ogni file sotto i 2 GiB, nessun limite di dimensione totale ne' di banda (fonti/github-releases-limits.md) [V]. Adatto a un repo pubblico; nessun aggiornamento automatico senza un updater nel gioco [INF].

## Flathub

- Requisiti di sottomissione: niente software a fine vita con permessi di rete invasivi, niente gioco d'azzardo con denaro reale, ID dell'app stabile (fonti/flathub-requirements.md) [V].
- Per Godot esiste una BaseApp Flathub, con Pixelorama come esempio (fonti/godot-creating-applications.md) [V].

## Firma: costi fissi

- **Apple**: Apple Developer Program a 99 USD l'anno, necessario per il certificato Developer ID e la notarizzazione (fonti/apple-developer-program.md) [V]. Obbligatorio per Steam su macOS (sopra).
- **Windows**: la doc Electron indica Azure Artifact Signing come l'opzione piu' economica e quella che elimina gli avvisi SmartScreen (fonti/electron-code-signing.md) [V]. La pagina Microsoft descrive il servizio (fonti/ms-artifact-signing-overview.md) [V]; la pagina prezzi riporta i piani Basic (5.000 firme/mese) e Premium (100.000) ma le cifre non compaiono nella cattura (fonti/ms-artifact-signing.md) [V/NON TROVATO] e l'idoneita' per un privato residente in Italia non e' confermata da fonte ufficiale: una risposta Microsoft Q&A, generata da IA, limita i privati a USA e Canada [NON TROVATO in fonte ufficiale].
- Su Steam e itch.io la firma Authenticode non e' un requisito nelle fonti lette [INF].

## Pesi indicativi del solo runtime

- Godot 4.7.2: template Windows 37,9 MB compresso, Linux 28,3 MB compresso [M, vedi [[godot]]].
- Electron 44.8.0: 123-159 MB zip [M, vedi [[electron]]].
- Tauri: app minima sotto 600 KB [V, vedi [[tauri]]].

## Per il nostro gioco

- Prendiamo: Steam + itch.io per i giocatori, GitHub Releases per il repo pubblico, Flathub come canale Linux opzionale; nessun launcher; gioco interamente offline.
- Evitiamo: launcher separati (contro i criteri Deck); dipendenze Linux non incluse nel runtime Steam.
- Aperto: firma Windows per i download diretti (costo e idoneita'); il conto dei 99 USD/anno Apple va messo in budget dal primo rilascio macOS.

## Collegamenti

- [[godot]], [[tauri]], [[electron]]: pesi e pipeline di firma per stack.
- [[prestazioni-su-hardware-modesto]]: requisiti Deck sul testo.

## Fonti

- **URL** https://partner.steamgames.com/steamdirect · [V]
  **perche' conta qui** costo d'ingresso su Steam
  **cosa se ne prende** 100 $ recuperabili, 30 giorni di attesa
- **URL** https://partner.steamgames.com/doc/store/application/platforms · [V]
  **perche' conta qui** requisiti per piattaforma su Steam
  **cosa se ne prende** notarizzazione macOS obbligatoria, niente sandbox
- **URL** https://partner.steamgames.com/doc/steamdeck/compat · [V]
  **perche' conta qui** criteri Deck che vincolano la UI
  **cosa se ne prende** testo minimo 9 px a 1280x800, tastiera a schermo, niente launcher
- **URL** https://partner.steamgames.com/doc/store/application/platforms/linux · [V]
  **perche' conta qui** dipendenze Linux su Steam
  **cosa se ne prende** librerie incluse nel runtime e obbligo di ridistribuire le altre
- **URL** https://itch.io/docs/butler/ · [V]
  **perche' conta qui** pubblicazione automatica su itch.io
  **cosa se ne prende** upload, patch offline, integrazione in CI
- **URL** https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases · [V]
  **perche' conta qui** distribuzione dal repo pubblico
  **cosa se ne prende** 2 GiB per file, banda illimitata
- **URL** https://docs.flathub.org/docs/for-app-authors/requirements · [V]
  **perche' conta qui** canale Linux fuori da Steam
  **cosa se ne prende** regole di sottomissione
- **URL** https://developer.apple.com/programs/whats-included/ · [V]
  **perche' conta qui** costo della notarizzazione
  **cosa se ne prende** 99 USD l'anno
- **URL** https://www.electronjs.org/docs/latest/tutorial/code-signing · [V]
  **perche' conta qui** opzione di firma Windows piu' economica
  **cosa se ne prende** Artifact Signing contro SmartScreen
- **URL** https://azure.microsoft.com/en-us/pricing/details/artifact-signing/ · [V]
  **perche' conta qui** costo della firma Windows
  **cosa se ne prende** piani e quote; cifre non catturate
- **URL** https://learn.microsoft.com/en-us/azure/artifact-signing/overview · [V]
  **perche' conta qui** servizio di firma Microsoft
  **cosa se ne prende** descrizione del servizio; prezzo e idoneita' da verificare
