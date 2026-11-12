---
tipo: tecnica
fonti: [fonti/wry-0-57-0-wkwebview-mod-rs.md, fonti/tauri-2-12-3-dati-del-webview-sorgente.md, fonti/tauri-cli-2-12-2-bundler-macos-app-rs.md, fonti/apple-wkwebsitedatastore.md, fonti/apple-wkwebsitedatastore-default.md, fonti/apple-wkwebsitedatastore-nonpersistent.md, fonti/apple-wkwebsitedatastore-init-foridentifier.md, fonti/webkit-2fbc5b5-websitedatastorecocoa-mm.md, fonti/webkit-2fbc5b5-resourceloadstatisticsstore-cpp.md, fonti/webkit-2fbc5b5-webresourceloadobserver-cpp.md, fonti/webkit-blog-full-third-party-cookie-blocking.md, fonti/webkit-blog-app-bound-domains.md, fonti/webkit-tracking-prevention.md, fonti/mdn-window-localstorage.md]
aggiornato: 2026-11-12
---
# localStorage in WKWebView per le impostazioni del giocatore

Dove tenere scala, velocità e interruttore del movimento (pochi byte di JSON) nell'app Tauri 2.12.3 su macOS 27. Risposta breve: `localStorage` della pagina regge fra un avvio e l'altro. Wry usa il data store persistente di default, e il tetto di ITP non scatta per un gioco che si usa cliccando. Restano due trappole: lo storage di sviluppo e quello di rilascio sono due depositi separati, su due cartelle diverse e con due origini diverse; e chi cambia `identifier`, nome del pacchetto cargo o porta di `devUrl` riparte da vuoto.

Ricerca svolta da `/Users/mario/GitHub/simulatore-politico`, branch `docs/piano-11-mappa`, HEAD `fd3ea2d`, il 12/11/2026. Versioni bloccate nel repo [M]: `tauri = "=2.12.3"` (`app/src-tauri/Cargo.toml:17`), `wry` 0.57.0 (`Cargo.lock:4230-4231`), `tauri-codegen` 2.7.4 (`Cargo.lock:3156-3157`), `@tauri-apps/cli` 2.12.2 (`app/package.json:23`). WebKit letto al commit `2fbc5b5` di `main` (12/11/2026), non al tag di Safari 27: vedi «Caveat».

## 1. Il data store che usa wry

- Wry sceglie il data store così [V] (fonti/wry-0-57-0-wkwebview-mod-rs.md:228-254, sorgente identico nel registro cargo locale `wry-0.57.0/src/wkwebview/mod.rs:221-247`):
  - `incognito = true` → `WKWebsiteDataStore::nonPersistentDataStore`;
  - `incognito = false` e `data_store_identifier` presente, con macOS ≥ 14 → `dataStoreForIdentifier(uuid)`;
  - altrimenti → `WKWebsiteDataStore::defaultDataStore`.
- Tauri passa `webview_attributes.incognito` a `with_incognito` (fonti/tauri-2-12-3-dati-del-webview-sorgente.md:14) e `data_store_identifier` solo se presente (:24-27) [V]. In config `incognito` è un `bool` con `#[serde(default)]`, quindi `false` se non scritto (:34-40) [V]. In `app/src-tauri/tauri.conf.json:13-19` la finestra non dichiara né `incognito` né `dataStoreIdentifier` [M]. Ne segue `defaultDataStore` [INF, dalle tre righe sopra].
- Per Apple il data store di default «stores data persistently to disk» (fonti/apple-wkwebsitedatastore-default.md:12), ed è quello che `WKWebViewConfiguration` usa se non si cambia nulla (fonti/apple-wkwebsitedatastore.md:31) [V]. Il non persistente tiene i dati «only in memory» (fonti/apple-wkwebsitedatastore-nonpersistent.md:26) [V].
- `dataDirectory` di Tauri su macOS non è supportato: va usato `dataStoreIdentifier`, da macOS 14 (fonti/tauri-2-12-3-dati-del-webview-sorgente.md:58, :71) [V]. Ci serve solo se un giorno si vuole separare lo storage di sviluppo da quello di rilascio dentro lo stesso processo; oggi no [INF].

## 2. Dove finisce su disco

- WebKit compone la cartella così: `~/Library/WebKit/<id>/WebsiteData/`, se il processo non è in sandbox (fonti/webkit-2fbc5b5-websitedatastorecocoa-mm.md:612-616) [V]. `<id>` è il `bundleIdentifier` del bundle principale; se manca, è il **nome del processo** (:132-142) [V].
- **Rilascio.** Il bundler della CLI 2.12.2 scrive `CFBundleIdentifier` = `identifier` nell'`Info.plist` del `.app` (fonti/tauri-cli-2-12-2-bundler-macos-app-rs.md:233-234) [V]. La doc di config lo dice esplicitamente: l'identifier serve per «the bundle ID and path to the webview data directory» (fonti/tauri-2-12-3-dati-del-webview-sorgente.md:82) [V]. Con `"identifier": "it.maeurong.simulatorepolitico"` (`app/src-tauri/tauri.conf.json:5`) il deposito sarà `~/Library/WebKit/it.maeurong.simulatorepolitico/WebsiteData/` [INF]. Non misurato: nel repo non c'è un `.app` costruito e quella cartella non esiste ancora (`ls ~/Library/WebKit/`, `find … -name "*.app"`, 12/11/2026) [M]. App non sandboxata, quindi niente `~/Library/Containers/…` ([[persistenza-dei-salvataggi]], riga 15).
- **Sviluppo.** In `tauri dev` il binario non sta in un bundle; `tauri-codegen` 2.7.4 incorpora un `Info.plist` con `CFBundleName` e le versioni, ma **senza** `CFBundleIdentifier` (fonti/tauri-2-12-3-dati-del-webview-sorgente.md:142-185) [V], e `app/src-tauri/` non ha un `Info.plist` proprio [M]. WebKit ripiega sul nome del processo, che è `app` (`app/src-tauri/Cargo.toml:2`). Misurato [M], `ls -la ~/Library/WebKit/` e `find ~/Library/WebKit/app -maxdepth 6`, 12/11/2026:
  - esiste `~/Library/WebKit/app/WebsiteData/` (creata il 22/10), con `Default/<hash>/<hash>/LocalStorage/localstorage.sqlite3` per ogni origine;
  - `xxd` sul file `origin` di una delle due origini dà `http` · `localhost` · `0x058c` = porta 1420, cioè il `devUrl` (`app/src-tauri/tauri.conf.json:8`). L'altra origine è `inspector-resource` (Web Inspector). La pagina dell'app non ha ancora scritto in `localStorage`: quella cartella contiene solo `origin`.
- `app_data_dir` invece segue sempre l'`identifier` della config, in sviluppo e in rilascio: `~/Library/Application Support/it.maeurong.simulatorepolitico/` esiste già e contiene `sviluppo` [M] ([[persistenza-dei-salvataggi]], righe 14-15).

## 3. ITP e il tetto dei 7 giorni

- Il tetto esiste ed è scritto per Safari: «after seven days of Safari use without user interaction on the site» (fonti/webkit-blog-full-third-party-cookie-blocking.md:65) [V], con `LocalStorage` fra gli storage colpiti (fonti/webkit-tracking-prevention.md:131-134) [V]. Le web app sulla schermata Home hanno un contatore loro, per giorni d'uso dell'app, e WebKit dichiara che la cancellazione dei dati del primo livello lì sarebbe «a serious bug» (fonti/webkit-blog-full-third-party-cookie-blocking.md:75-77) [V].
- **Vale anche nelle app.** Da macOS Big Sur ITP è «enabled by default in all `WKWebView` applications» (fonti/webkit-blog-app-bound-domains.md:151) [V]. Si vede sul Mac di Mario [M]: la copia di `~/Library/WebKit/app/WebsiteData/ResourceLoadStatistics/observations.db`, interrogata con `sqlite3 -readonly … "select … from ObservedDomains"` il 12/11/2026, contiene `localhost` con `hadUserInteraction = 1`, ultima interazione `1793647400` (02/11/2026 20:23), e due `OperatingDates`, 22/10 e 02/11. Le date coincidono con i due giorni in cui l'app di sviluppo è stata aperta.
- **Perché non scatta.** Nel codice (WebKit `2fbc5b5`):
  - fuori da Safari la modalità di rimozione resta `AllButCookies` (fonti/webkit-2fbc5b5-websitedatastorecocoa-mm.md:179-193) [V];
  - un dominio perde lo storage scrivibile da script se **non** ha un'interazione recente non scaduta (fonti/webkit-2fbc5b5-resourceloadstatisticsstore-cpp.md:2905-2925, :2886-2898) [V];
  - la finestra si misura in giorni d'uso, non di calendario: 30 per la regola ordinaria, 7 solo per i domini raggiunti con link decorati da un tracker (:81-82, :2915, :2122) [V], e un giorno d'uso si conta all'avvio dello store (:387) [V];
  - un clic, un tocco o un tasto sulla pagina principale registra l'interazione, per ogni schema che non sia `about:` o `file:` (fonti/webkit-2fbc5b5-webresourceloadobserver-cpp.md:379-402) [V]; lo scroll non conta (fonti/webkit-tracking-prevention.md:20) [V].

  Per perdere le impostazioni servirebbero quindi 30 giorni d'uso dell'app, quelli in cui l'app si apre, senza un solo clic sulla sua pagina. Un gioco a turni che si gioca cliccando non ci arriva [INF]. Le pause lunghe non contano: un mese di app chiusa non aggiunge giorni d'uso [INF, da :387 e :3385].
- La doc pubblica dice 7 giorni, il codice di oggi 30 giorni d'uso per il caso ordinario. Ragionare sui 7 giorni, più prudenti, non cambia il verdetto: anche 7 giorni d'uso senza un clic non capitano in un gioco a turni [INF].
- `localhost` non è esentato: l'esclusione `shouldIncludeLocalhost = isSafari` (fonti/webkit-2fbc5b5-websitedatastorecocoa-mm.md:232) vale solo per la classificazione dei tracker (`shouldSkip`, fonti/webkit-2fbc5b5-resourceloadstatisticsstore-cpp.md:403-408), non per la rimozione [V]. L'esenzione permanente esiste per gli app-bound domains e i domini gestiti (:2968, :820) [V]; aggiungere `WKAppBoundDomains` all'`Info.plist` bloccherebbe però l'iniezione di script da cui dipende l'IPC di Tauri (fonti/webkit-blog-app-bound-domains.md:29) [INF]. Non serve.
- `tauri://localhost` e `http://localhost:1420` hanno lo stesso host, quindi per ITP sono lo stesso dominio registrabile `localhost` [INF]: in rilascio vale lo stesso ragionamento, ma su un `observations.db` distinto, quello in `~/Library/WebKit/it.maeurong.simulatorepolitico/` [INF].

## 4. Cosa cambia fra sviluppo, rilascio e identifier

- **Origine.** In sviluppo la pagina è `devUrl`; in rilascio, su macOS, `tauri://localhost` (fonti/tauri-2-12-3-dati-del-webview-sorgente.md:98-101, :116-134) [V]. `localStorage` è per origine, e lo schema fa parte dell'origine (fonti/mdn-window-localstorage.md:16, :39) [V]. Le due origini, e i due depositi del § 2, non si vedono: le impostazioni scelte in sviluppo non arrivano nel `.app` [INF].
- **Identifier.** Cambiarlo sposta il deposito di rilascio in `~/Library/WebKit/<nuovo-id>/` e lascia indietro il vecchio [INF, § 2]. Sposta anche `app_data_dir`: su questo i due approcci sono pari.
- **Sviluppo.** Il deposito dipende dal nome del pacchetto cargo (`app`), non dall'identifier [M, § 2]. Cambiare `name` in `Cargo.toml` o la porta di `devUrl` azzera le impostazioni di sviluppo [INF]. Un altro binario non in bundle chiamato `app` condividerebbe la stessa cartella [INF].
- **«Cancella dati del sito»** non c'è in un'app Tauri, ma l'utente può cancellare `~/Library/WebKit/<id>/`, e `WKWebsiteDataStore.removeData` la svuota da codice [INF]. Non è un rischio della v1.

## 5. Confronto: file JSON nel guscio

Un `impostazioni.json` in `app_data_dir`, scritto con `std::fs` come i salvataggi ([[persistenza-dei-salvataggi]]), non dipende da ITP né dall'origine, ed è lo stesso file in sviluppo e in rilascio. In cambio chiede due comandi IPC, e il valore arriva dopo il primo disegno invece che in modo sincrono [INF].

## Per il nostro gioco

**Raccomandazione, non decisione.** `localStorage` va bene per scala, velocità e movimento: è persistente (default data store), è sincrono, quindi la scala si applica prima del primo disegno, e ITP non lo tocca finché il giocatore clicca. Due accorgimenti:

- una chiave sola con versione dentro (`{"v":1,…}`), e un default sensato quando manca: in sviluppo e in rilascio, o dopo un cambio di identifier, si riparte da vuoto, e i default vanno trattati come caso normale. Il valore iniziale di `spento` viene da `prefersReducedMotion` (`docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:85`);
- se un giorno le impostazioni diventano dati che costa perdere (profilo, scorciatoie personalizzate), si spostano nel file del § 5.

Resta aperto: provare il deposito di rilascio su un `.app` costruito (`ls ~/Library/WebKit/it.maeurong.simulatorepolitico/WebsiteData/Default/`) quando esiste il primo bundle.

## Caveat

- WebKit è letto su `main` al commit `2fbc5b5` (12/11/2026), non al tag di Safari 27 (WebKit.framework 22625.1.29.11.28, [[movimento-e-scala-in-wkwebview]]). Le costanti 30/7 e la logica di rimozione possono differire di poco dalla versione installata. Il comportamento osservato (ITP attivo, `localhost` con interazione registrata) è misurato sulla versione installata.
- Il percorso del deposito di rilascio è dedotto da sorgente e bundler, non misurato.
- [NON TROVATO] una issue di Tauri o di wry su `localStorage` perso fra un avvio e l'altro su macOS (due ricerche web, 12/11/2026).

## Riferimenti

- **URL** https://raw.githubusercontent.com/tauri-apps/wry/wry-v0.57.0/src/wkwebview/mod.rs · [V] (fonti/wry-0-57-0-wkwebview-mod-rs.md:228-254)
  **perché conta qui** è il punto in cui wry sceglie il data store del WKWebView
  **cosa se ne prende** `incognito` → non persistente, `data_store_identifier` → per UUID, altrimenti `defaultDataStore`
- **URL** registro cargo locale `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/` (tauri-runtime-wry 2.12.3, tauri-utils 2.10.2, tauri 2.12.3, tauri-codegen 2.7.4, wry 0.57.0) · [V] (fonti/tauri-2-12-3-dati-del-webview-sorgente.md)
  **perché conta qui** collega la config dell'app agli attributi di wry e all'origine della pagina
  **cosa se ne prende** `incognito` false di default; `dataDirectory` non supportato su macOS; origine `tauri://localhost` in rilascio e `devUrl` in sviluppo; `Info.plist` di sviluppo senza `CFBundleIdentifier`
- **URL** https://raw.githubusercontent.com/tauri-apps/tauri/%40tauri-apps%2Fcli-v2.12.2/crates/tauri-bundler/src/bundle/macos/app.rs · [V] (fonti/tauri-cli-2-12-2-bundler-macos-app-rs.md:233-234)
  **perché conta qui** decide il `bundleIdentifier` del `.app`, da cui WebKit ricava la cartella
  **cosa se ne prende** `CFBundleIdentifier` = `identifier` della config
- **URL** https://developer.apple.com/documentation/webkit/wkwebsitedatastore (con `default()`, `nonPersistent()`, `init(forIdentifier:)`) · [V] (fonti/apple-wkwebsitedatastore.md:31; fonti/apple-wkwebsitedatastore-default.md:12; fonti/apple-wkwebsitedatastore-nonpersistent.md:26; fonti/apple-wkwebsitedatastore-init-foridentifier.md:21)
  **perché conta qui** è la garanzia di Apple sulla persistenza del data store di default
  **cosa se ne prende** default = persistente su disco; non persistente = solo memoria; per identificatore = profilo persistente, da macOS 14
- **URL** https://raw.githubusercontent.com/WebKit/WebKit/2fbc5b5f4b86d36fd212c3b5207d70e6f8a97ec7/Source/WebKit/UIProcess/WebsiteData/Cocoa/WebsiteDataStoreCocoa.mm · [V] (fonti/webkit-2fbc5b5-websitedatastorecocoa-mm.md:132-142, :179-193, :232, :612-616)
  **perché conta qui** fissa la cartella su disco e i parametri di ITP per le app che non sono Safari
  **cosa se ne prende** `~/Library/WebKit/<bundle id o nome del processo>/WebsiteData/`; `AllButCookies` anche fuori da Safari; `localhost` escluso solo dalla classificazione
- **URL** https://raw.githubusercontent.com/WebKit/WebKit/2fbc5b5f4b86d36fd212c3b5207d70e6f8a97ec7/Source/WebKit/NetworkProcess/Classifier/ResourceLoadStatisticsStore.cpp · [V] (fonti/webkit-2fbc5b5-resourceloadstatisticsstore-cpp.md:81-82, :387, :403-408, :2886-2925, :2968-3003)
  **perché conta qui** è l'algoritmo che decide quando cancellare `localStorage`
  **cosa se ne prende** rimozione solo senza interazione recente; finestra di 30 giorni d'uso (7 per i link decorati); esenzione per app-bound domains
- **URL** https://raw.githubusercontent.com/WebKit/WebKit/2fbc5b5f4b86d36fd212c3b5207d70e6f8a97ec7/Source/WebKit/WebProcess/WebCoreSupport/WebResourceLoadObserver.cpp · [V] (fonti/webkit-2fbc5b5-webresourceloadobserver-cpp.md:379-402)
  **perché conta qui** dice se un clic nella pagina `tauri://` o `http://localhost` azzera il contatore
  **cosa se ne prende** l'interazione si registra per ogni schema tranne `about:` e `file:`
- **URL** https://webkit.org/blog/10218/full-third-party-cookie-blocking-and-more/ · [V] (fonti/webkit-blog-full-third-party-cookie-blocking.md:65, :75-77)
  **perché conta qui** è l'annuncio del tetto dei 7 giorni, quello che fa nascere il dubbio
  **cosa se ne prende** il tetto si conta in giorni d'uso, non di calendario, e non deve colpire il primo livello di una web app
- **URL** https://webkit.org/blog/10882/app-bound-domains/ · [V] (fonti/webkit-blog-app-bound-domains.md:29, :151)
  **perché conta qui** è la fonte primaria per ITP nelle app che incorporano WKWebView
  **cosa se ne prende** ITP attivo di default in ogni app WKWebView da macOS Big Sur; app-bound domains bloccano l'iniezione di script
- **URL** https://webkit.org/tracking-prevention/ · [V] (fonti/webkit-tracking-prevention.md:20, :131-134)
  **perché conta qui** è la doc attuale di ITP: elenca gli storage colpiti e cosa conta come interazione
  **cosa se ne prende** `LocalStorage` è nel tetto; clic, tocco e tasto contano, lo scroll no
- **URL** https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage · [V] (fonti/mdn-window-localstorage.md:16, :39)
  **perché conta qui** l'origine cambia fra sviluppo e rilascio
  **cosa se ne prende** storage per origine, schema compreso: `http://localhost:1420` e `tauri://localhost` sono due depositi
- **Misura** `ls -la ~/Library/WebKit/`; `xxd …/Default/<hash>/<hash>/origin`; `sqlite3 -readonly` su una copia di `observations.db` (12/11/2026, Mac di Mario) · [M]
  **perché conta qui** conferma sul WebKit installato cartella di sviluppo, origine e ITP attivo
  **cosa se ne prende** deposito di sviluppo in `~/Library/WebKit/app/`, origine `http://localhost:1420`, `localhost` con interazione registrata

## Collegamenti
- [[movimento-e-scala-in-wkwebview]]: definisce le tre impostazioni che qui si salvano.
- [[persistenza-dei-salvataggi]]: `app_data_dir` e `std::fs`, l'alternativa del § 5.
- [[tauri]]: webview di sistema, quindi WebKit di macOS.
