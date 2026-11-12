Type: grilling
Status: resolved
Blocked by: -
Assignee: maeurong
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa controlla un piano del piano 11 oltre a `svelte-check`, prima di dirsi finito? Test di componente, un giro nel browser con la skill `run` sulle Viste campione, istantanee delle schermate, o una combinazione; e su quali stati (le Viste in `viste/`, rigenerabili con un comando, sono già una batteria di casi giocati).

Va deciso prima della spec, perché ogni fetta ([Fette](10-fette.md)) lo scrive nei suoi criteri di «pronta». I prototipi, dalle [Fondamenta visive](08-fondamenta-visive.md) in poi, contano tagli e scorrimento a 1280 e 1440: da decidere se quel controllo passa nel codice vero.

## Answer

Deciso con Mario il 12/11/2026 in tre giri di domande. Nessuna ricerca nuova: i fatti vengono dal codice, dai ticket già chiusi e dalla documentazione di Tauri su WebDriver e su `mockIPC` ([Tauri, mocking](https://v2.tauri.app/develop/tests/mocking/)). La seconda domanda dava l'app vera guidabile solo a screenshot; la documentazione offre anche WebdriverIO, e al terzo giro Mario ha confermato la scelta sapendolo.

### Cosa controlla una fetta prima di dirsi finita

Sei controlli, da scrivere nei criteri di ogni piano nato dalla spec ([Fette](10-fette.md)):

1. **`svelte-check` pulito e Vitest verde.** Vitest solo sulla logica pura, senza DOM: scale dei tracciati ([Scale dei tracciati](31-scale-dei-tracciati.md)), funzione `t` dei testi ([Testi esterni](03-testi-esterni.md)), hash delle caricature ([Caricature](05-caricature.md)), ordine delle voci in prima pagina ([Prima pagina](16-prima-pagina.md)). Niente test di componente. È l'unica dipendenza nuova, e usa la configurazione di Vite che c'è già. Oggi `app/package.json` ha solo `svelte-check`.
2. **Batteria nel browser sulle Viste campione.** L'app chiama il guscio con `invoke` (`app/src/routes/+page.svelte:2`), che in un browser normale fallisce. Un modulo caricato solo in sviluppo (`import.meta.env.DEV`) installa `mockIPC` di `@tauri-apps/api/mocks`, che sta già nella dipendenza `@tauri-apps/api`, e serve la Vista scelta con `?vista=<file>&stato=<chiave>` dalla cartella `viste/` com'è. I comandi che cambiano stato rispondono con la Vista «dopo» quando il campione ce l'ha, altrimenti con un rifiuto «non nel campione». Si provano tutti gli stati delle Viste che toccano le schermate della fetta, più il tono grave (`08-tono-grave.json`), a 1280×800 e 1440×900, ciascuno con scala 1 e con la scala massima (1,25 e 1,4). A 1280 con 1,25× restano meno di 1150 px utili e scatta la forma stretta della cornice ([Fondamenta visive](08-fondamenta-visive.md)), così anche quella è coperta. Le schermate delle fette precedenti non si rifanno: la cornice comune passa comunque in ogni stato.
3. **Zero tagli.** Il contatore dei prototipi (`prototipi/08-fondamenta.prototype.html:537`, gli elementi che escono dal loro riquadro) entra nel codice, attivo solo in sviluppo con `?tagli=1`, e non arriva nell'app rilasciata. Su tutta la batteria deve dare zero.
4. **Movimento e tastiera.** Con il movimento spento, e con «Come il sistema» sotto `prefers-reduced-motion`, nessuna animazione parte, tono grave compreso ([Impostazioni del giocatore](33-impostazioni-del-giocatore.md)). Ogni azione della schermata si raggiunge con Tab e si attiva da tastiera, conferme sul posto comprese. Si controlla nel browser con l'albero di accessibilità.
5. **Un giro giocato nell'app vera.** `tauri-driver` da solo non funziona su macOS, perché WKWebView non ha un driver ([Tauri, WebDriver](https://v2.tauri.app/develop/tests/webdriver/)). Le due vie che lo aggirano costano più del giro: `@wdio/tauri-service` vuole il plugin `tauri-plugin-wdio-webdriver` nel guscio e un albero di pacchetti npm, il fork di CrabNebula una chiave a pagamento. L'agente lancia `tauri dev` e, con computer-use, gioca dall'Edicola il percorso che la fetta aggiunge, poi prova Cmd+ / Cmd− e il movimento spento. Basta il giro dell'agente, con gli screenshot in PR: Mario guarda la PR. Al primo giro, in 11a, si verifica in WKWebView anche ciò che le Fondamenta e il [Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md) avevano lasciato aperto: `zoom`, `linear()`, i due caratteri dentro l'app.
6. **Screenshot come prova, non come confronto.** Gli screenshot delle schermate della fetta, alle due finestre, vanno nella PR. Non c'è confronto automatico con immagini di riferimento, perché ogni fetta cambia le schermate. Giudica la fedeltà il `craft-reviewer`, nel round di review prima del commit: confronta gli screenshot col prototipo scelto e coi token delle Fondamenta (nessun colore né misura scritti a mano), e gli scarti voluti si scrivono in PR. Se il revisore è pulito, la fedeltà vale.

### Scartato

Test di componente con testing-library; confronto pixel per pixel con Playwright; batteria nell'app vera con `@wdio/tauri-service`, per il plugin nel guscio e le dipendenze; nessun test; batteria su tutte le schermate a ogni fetta; solo il caso stretto (1280×800 a scala 1); giro nell'app vera fatto da Mario o seguito dal suo ok; fedeltà giudicata da Mario.

Nessun rimando al core, nessun passacarte: il modulo di sviluppo sta tutto nella UI.
