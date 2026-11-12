---
tipo: tecnica
fonti: [fonti/bcd-easing-function.md, fonti/bcd-backface-visibility.md, fonti/bcd-transform-style.md, fonti/bcd-css-zoom.md, fonti/bcd-animation-api.md, fonti/mdn-linear-easing.md, fonti/mdn-backface-visibility.md, fonti/mdn-css-zoom.md, fonti/mdn-document-getanimations.md, fonti/mdn-animation-updateplaybackrate.md, fonti/mdn-transform.md, fonti/mdn-prefers-reduced-motion.md, fonti/webkit-safari-17-2.md, fonti/chrome-css-linear-easing.md, fonti/svelte-docs-motion.md, fonti/svelte-docs-transition.md, fonti/svelte-5-57-2-transitions-js.md, fonti/svelte-5-57-2-spring-js.md, fonti/tauri-api-2-12-3-webview-dts.md, fonti/tauri-2-12-3-zoom-hotkey-js.md, fonti/wry-0-57-0-wkwebview-mod-rs.md, fonti/apple-wkwebview-pagezoom.md, fonti/webkit-bug-27684.md, fonti/webkit-bug-264954.md, fonti/wcag22-resize-text.md, fonti/wcag22-animation-from-interactions-2026-11.md, fonti/csswg-mediaqueries-4.md, fonti/csswg-css-transitions-1.md, fonti/webdev-animations-guide.md]
aggiornato: 2026-11-01
---
# Movimento e scala della UI in WKWebView

Cosa regge il webview di Tauri su macOS per le storie 154-158 della spec v1 (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:224-228`, decisione di accessibilità alla riga 401): carta del ministro che si gira, mobile appeso che oscilla a un gesto, tracciati a caselle a scatti, esito rivelato una voce alla volta, velocità 1×/2×/istantaneo, interruttore del movimento, scala della UI con testo minimo di 12 px a 1×. Tutto si fa con CSS e con ciò che Svelte 5 ha già, senza librerie d'animazione. Due trappole: le transizioni di Svelte ignorano una regola CSS su `prefers-reduced-motion`, e `transform: scale` sfoca il testo in WebKit.

Ticket: `.scratch/v1-piano-11-ui/issues/04-movimento-e-scala-in-wkwebview.md`. Le pagine [[movimento-e-feedback]] (ritmo dell'esito, WCAG 2.3.3) e [[succo-per-interfacce-di-carte-e-numeri]] (durate e curve) restano il punto di partenza, e qui non si ripetono.

## Ambiente misurato

- Mac di Mario: macOS 27.0.1, Safari 27.0.1, WebKit.framework 22625.1.29.11.28, Apple A18 Pro, 8 GiB di RAM [M] (`sw_vers`; `defaults read /Applications/Safari.app/Contents/Info.plist CFBundleShortVersionString`; `defaults read /System/Library/Frameworks/WebKit.framework/Resources/Info.plist CFBundleVersion`; `sysctl -n machdep.cpu.brand_string hw.memsize`, 01/11/2026). È già un «Mac modesto»: la macchina Apple più piccola che c'è, con 8 GiB.
- Versioni bloccate nel repo [M]: `tauri = "=2.12.3"` (`app/src-tauri/Cargo.toml:15`), `wry` 0.57.0 e `tauri-utils` 2.10.2 (`Cargo.lock:4228-4229`, `:3245-3246`), `svelte` 5.57.2 e `@tauri-apps/api` 2.12.3 (`app/package-lock.json:1396-1397`, `app/package.json`).
- WKWebView usa il WebKit di sistema, quindi la sua versione segue Safari e macOS ([[tauri]]). La v1 è per Mario, perciò ogni funzione citata sotto c'è. Se un giorno l'app uscisse dal suo Mac, il minimo dichiarato da Tauri resterebbe macOS 10.13 (`tauri-utils-2.10.2/src/config.rs:694-695`, nel registro cargo locale) [M], cioè più in basso di quanto chiedono `linear()` e `pageZoom` (vedi sotto).

## 1. Cosa regge WKWebView

| Funzione | Safari minimo | Fonte |
|---|---|---|
| `transform-style: preserve-3d` | 9 senza prefisso; prima di Safari 17 `::before` e `::after` restavano fuori dal contesto 3D | [V] (fonti/bcd-transform-style.md:59) |
| `backface-visibility` | 15.4 senza prefisso, 5.1 con `-webkit-` | [V] (fonti/bcd-backface-visibility.md:61) |
| `cubic-bezier()` | 6 | [V] (fonti/bcd-easing-function.md:87) |
| `steps()` / parole chiave `jump-*` | 5.1 / 14 | [V] (fonti/bcd-easing-function.md:166, :207) |
| `linear()` con più punti | 17.2 | [V] (fonti/bcd-easing-function.md:123; fonti/webkit-safari-17-2.md:103) |
| Web Animations: `Animation`, `finish()`, `finished`, `playbackRate`, `updatePlaybackRate()` | 13.1 | [V] (fonti/bcd-animation-api.md:31, :315, :389, :632, :964) |
| `document.getAnimations()` | Baseline da settembre 2020 | [V] (fonti/mdn-document-getanimations.md:11) |
| `prefers-reduced-motion` | Baseline da gennaio 2020 | [V] (fonti/mdn-prefers-reduced-motion.md:16) |
| CSS `zoom` | 3.1; non animabile in Safari; valore `reset` tolto in 18.4 | [V] (fonti/bcd-css-zoom.md:72, :105) |
| `WKWebView.pageZoom` (cioè `setZoom` di Tauri) | macOS 11 | [V] (fonti/apple-wkwebview-pagezoom.md:25) |

Safari 17.2 era disponibile anche per macOS Monterey e Ventura (fonti/webkit-safari-17-2.md:602) [V]. Il pavimento reale per `linear()` è quindi «Monterey con Safari aggiornato», non una versione di macOS [INF].

### Carta che si gira (storia 154)

- Ricetta standard: contenitore con `perspective`, interno con `transform-style: preserve-3d` e `transform: rotateY(180deg)` in transizione, due facce con `backface-visibility: hidden`. La proprietà agisce solo sulle trasformazioni 3D (fonti/mdn-backface-visibility.md:18) [V]. Su Safari 27 non serve il prefisso `-webkit-` [V] (tabella).
- Durante la rotazione la carta è uno strato composto, disegnato una volta e poi ruotato dalla GPU [INF]. A rotazione finita torna piatta e il testo si ridisegna nitido [INF]. Il caso patologico dello strato ingrandito (sotto, «Scala») riguarda `scale`, non `rotateY`.
- Movimento spento: la stessa carta cambia faccia con una dissolvenza d'opacità invece della rotazione. Per WCAG un cambio d'opacità che non modifica dimensione, forma o posizione percepite non è «motion animation» (fonti/wcag22-animation-from-interactions-2026-11.md:68-70) [V]. È nuovo rispetto a [[movimento-e-feedback]], che cita 2.3.3 ma non questa esclusione. La stessa pagina ha però un'errata che toglie la sfocatura dall'esclusione (fonti/wcag22-animation-from-interactions-2026-11.md:71) [V]: niente `blur` nella variante ridotta.

### Molle e mobile appeso (storia 156)

Tre strade, in ordine di preferenza:

1. **Curva `linear()` su una sola animazione.** `linear()` accetta valori fuori da 0-1 (fonti/mdn-linear-easing.md:33) [V], quindi approssima rimbalzi e molle (fonti/chrome-css-linear-easing.md:21) [V]. Il generatore di Jake Archibald e Adam Argyle ha già i preset Spring e Bounce (fonti/chrome-css-linear-easing.md:130) [V]. Raddoppiando i punti la curva diventa liscia (fonti/chrome-css-linear-easing.md:101) [V]. Per il mobile: al gesto, `el.animate([{rotate: '6deg'}, {rotate: '0deg'}], {duration, easing: 'linear(…molla…)'})`. La curva supera 1 e torna, quindi il mobile oscilla oltre lo zero e si ferma [INF]. È una Web Animation: `getAnimations()` la vede e `playbackRate` la governa (§ 2).
2. **Funzione di easing JS dentro una transizione Svelte.** Svelte campiona `easing(t)` in keyframe a 60 al secondo, `n = ceil(durata / (1000/60))` (fonti/svelte-5-57-2-transitions-js.md:448) [V], e li passa a `element.animate` (fonti/svelte-5-57-2-transitions-js.md:483) [V]. Un `elasticOut` di `svelte/easing` diventa quindi una Web Animation senza bisogno di `linear()`. Una transizione da 240 ms produce 16 keyframe, una da 400 ms 25 [M] (stessa formula, calcolata in Python).
3. **`Spring` di `svelte/motion`** (dalla 5.8, fonti/svelte-docs-motion.md:22) [V]. Gira in un ciclo `requestAnimationFrame` sul thread principale (fonti/svelte-5-57-2-spring-js.md:259) [V] e si ferma quando è a riposo (fonti/svelte-5-57-2-spring-js.md:283-287) [V]. Ha `set(v, {instant: true})` (fonti/svelte-docs-motion.md:89; fonti/svelte-5-57-2-spring-js.md:308) [V], ma **nessuna scala del tempo**: il passo è il tempo reale, limitato a 1/30 di secondo (fonti/svelte-5-57-2-spring-js.md:262) [V]. Non segue la velocità 2× e `getAnimations()` non la vede [INF dal sorgente]. Serve solo se il mobile deve seguire un trascinamento, cosa che la storia 156 non chiede.

Tono grave (spec: «niente rimbalzi né oscillazioni»): basta riassegnare la variabile della curva con rimbalzo, per esempio `--e-pop` di [[succo-per-interfacce-di-carte-e-numeri]], a una curva senza superamento sotto `[data-tono=grave]` [INF].

### Tracciati a caselle a scatti (storia 155)

`steps(n, jump-end)` su una transizione di larghezza o di `transform` fa avanzare a scatti senza JS [V] (tabella: `steps()` da 5.1, `jump-*` da 14). In alternativa ogni casella è un elemento che si accende con `transition-delay: calc(var(--i) * var(--passo))` [INF]. Il ritardo scala insieme alla durata (§ 2).

### Esito rivelato a ritmo

- Il ritmo è già deciso (una voce alla volta, al massimo 7, [[movimento-e-feedback]]). Implementazione: `{#each voci as v, i}` con `in:fly={{ delay: i * passo * k, duration: base * k }}`. Le transizioni accettano parametri (fonti/svelte-docs-transition.md:53) [V] ed emettono `introend` per concatenare (fonti/svelte-docs-transition.md:158) [V].
- «Salta» col clic: `document.getAnimations().forEach(a => a.finish())`. Attenzione, Svelte esegue ogni transizione con ritardo in **due** Web Animations: un'animazione fittizia lunga quanto il ritardo, e solo al suo `onfinish` quella vera (fonti/svelte-5-57-2-transitions-js.md:422-424, :483-485) [V]. Un solo giro di `finish()` chiude i ritardi e fa partire le animazioni vere, non le salta [INF dal sorgente]. Per saltare davvero si porta il fattore `k` a 0 (§ 2) e poi si chiama `finish()`.
- Senza movimento ma a velocità 1× il ritmo resta, con le voci che compaiono in dissolvenza: l'opacità non è movimento per WCAG [V] (sopra) [INF sull'uso].

## 2. Velocità globale e interruttore del movimento

**Cosa non funziona da solo:**

- Una regola globale `@media (prefers-reduced-motion: reduce) { * { transition-duration: 0s; animation-duration: 0s } }` non tocca le transizioni di Svelte: lo dice la documentazione, perché sono guidate dalla Web Animations API e non da CSS (fonti/svelte-docs-transition.md:59) [V]. Svelte rimanda a `prefersReducedMotion` di `svelte/motion` (fonti/svelte-docs-transition.md:61) [V], disponibile dalla 5.7 (fonti/svelte-docs-motion.md:263) [V].
- `document.getAnimations()` restituisce solo le animazioni «currently in effect» (fonti/mdn-document-getanimations.md:16) [V]. Una manopola globale fatta solo con `playbackRate` (esempio MDN, fonti/mdn-document-getanimations.md:38) [V] perde ogni animazione che parte dopo [INF]. Serve solo per «salta» e per cambiare velocità a metà animazione. In quel caso va usato `updatePlaybackRate()`, che si sincronizza con l'animazione che gira fuori dal thread principale ed evita il salto (fonti/mdn-animation-updateplaybackrate.md:18) [V].
- Con durata 0 una transizione CSS non parte: la condizione è «combined duration greater than 0s» (fonti/csswg-css-transitions-1.md:199) [V]. Allora non arriva nemmeno `transitionend`, che scatta solo a transizione completata (fonti/csswg-css-transitions-1.md:361) [V]. Nessun codice deve aspettare `transitionend` [INF]. Le transizioni Svelte con durata 0 chiamano comunque il loro `onfinish` (fonti/svelte-5-57-2-transitions-js.md:483-485) [V].

**Cosa regge [INF], un solo fattore e due fonti di verità che lo leggono:**

```css
:root { --k: 1; }                       /* 1× = 1, 2× = 0.5, istantaneo = 0 */
.carta { transition: transform calc(400ms * var(--k)) var(--e-rare); }
```

```ts
// movimento.svelte.ts — un solo stato per CSS e JS
import { prefersReducedMotion } from 'svelte/motion';
export const mov = $state({ k: 1, spento: prefersReducedMotion.current });
export const dur = (ms: number) => ms * mov.k;    // per transizioni Svelte e el.animate()
$effect.root(() => $effect(() => document.documentElement.style.setProperty('--k', String(mov.k))));
```

- Velocità e movimento sono due assi distinti, come chiede la spec. `k` decide quanto dura, `spento` cosa si muove: rotazioni, traslazioni, scale e oscillazioni diventano dissolvenze o scompaiono. In CSS basta `:root[data-movimento=spento]` per riassegnare le variabili; in Svelte un `mov.spento ? fade : fly`.
- Il valore iniziale di `spento` viene da `prefersReducedMotion` (la preferenza di sistema), poi decide il giocatore. «Sopra `prefers-reduced-motion`» vuol dire questo [INF]. Su macOS la preferenza sta in Impostazioni di Sistema > Accessibilità (fonti/mdn-prefers-reduced-motion.md:50-51) [V]. MDN chiama Tahoe «macOS 25», ma Tahoe è macOS 26: refuso della fonte. Il 12/11/2026 il ticket 33 ha deciso diversamente: tre posizioni, e «Come il sistema» segue la preferenza anche dopo il primo avvio; le impostazioni stanno in `localStorage` ([[localstorage-in-wkwebview]]).
- `Spring` e `Tween` non leggono `--k`: si passa `dur()` a `Tween`, e per `Spring` `instant: mov.k === 0` [INF] (vedi § 1).

## 3. Scala della UI

| | `setZoom` di Tauri (`pageZoom`) | CSS `zoom` sulla radice | `font-size` sulla radice + tutto in `rem` | `transform: scale` |
|---|---|---|---|---|
| Cosa fa | Secondo Apple equivale a CSS `zoom` su tutta la pagina (fonti/apple-wkwebview-pagezoom.md:18) [V]; wry chiama `setPageZoom` (fonti/wry-0-57-0-wkwebview-mod-rs.md:965) [V] | Ingrandisce l'elemento e ricalcola il layout (fonti/mdn-css-zoom.md:16-20) [V] | Scala solo ciò che è in `rem` [INF] | Nessun ricalcolo di layout (fonti/mdn-css-zoom.md:20) [V] |
| Testo nitido | sì, si ridisegna alla nuova misura [INF] | sì [INF]; «texts don't behave the same at all zoom levels» (fonti/webkit-bug-264954.md:34) [V] | sì [INF] | **no** se lo strato è composto: WebKit lo disegna alla misura piccola e lo ingrandisce, bug aperto dal 2009 e ancora attivo il 28/10/2026 (fonti/webkit-bug-27684.md:10-16, :513) [V] |
| Layout | si riflette [INF] | si riflette [V] | si riflette, ma le media query in `rem`/`em` restano sul valore iniziale, non su quello della pagina (fonti/csswg-mediaqueries-4.md:63) [V] | resta a 1×, sborda; crea un contenitore per i figli `position: fixed` (fonti/mdn-transform.md:18) [V] |
| Costo d'integrazione | permesso `core:webview:allow-set-webview-zoom`, fuori da `core:webview:default` (fonti/tauri-api-2-12-3-webview-dts.md:453-456) [V]; la capability del repo ha solo `core:default` (`app/src-tauri/capabilities/default.json`) [M]; nessun `getZoom` nell'API [M] (`grep -n getZoom`, 0 risultati nel `.d.ts` 2.12.3); il valore va salvato e rimesso a ogni avvio [INF] | una riga, nessun permesso, uguale anche nel browser di `vite dev` [INF] | disciplina su ogni misura; bordi e immagini in `px` non scalano [INF] | da scartare |

- **`zoomHotkeysEnabled`** su macOS non è nativo: inietta uno script che a ⌘± chiama `set_webview_zoom` a passi del 20%, da 0,2 a 10 (fonti/tauri-api-2-12-3-webview-dts.md:619) [V]. Il suo contatore parte sempre da 1 (fonti/tauri-2-12-3-zoom-hotkey-js.md:14) [V], quindi non sa niente di uno zoom impostato dall'app e non lo ricorda fra un avvio e l'altro [INF dal sorgente]. Va lasciato spento; ⌘± si cattura in Svelte e scrive nello stesso stato della scala [INF].
- **CSS `zoom` e `pageZoom` sono la stessa cosa** secondo Apple [V] (sopra). Da verificare nel primo prototipo nel guscio [NON TROVATO in fonte]: se le media query e `innerWidth` reagiscono allo stesso modo nei due casi, e se le coordinate dei puntatori (`getBoundingClientRect`, `clientX`) restano coerenti sotto CSS `zoom` in WebKit. Conta per il mobile, se mai si trascina.
- **Testo minimo 12 px a 1×.** Con una scala che non scende sotto 1 il minimo regge per costruzione [INF]. Con lo zoom il minimo si scrive come `12px`, con l'approccio `rem` come `0.75rem` su base 16.
- **Quanto in alto.** WCAG 1.4.4 chiede testo ingrandibile fino al 200% senza perdite, a ogni passo intermedio (fonti/wcag22-resize-text.md:25, :41) [V]; una manopola dell'autore è una tecnica ammessa, G178 (fonti/wcag22-resize-text.md:89) [V]. Spazio CSS effettivo con la finestra minima 1280×800 [M] (aritmetica, finestra / scala): 1164×727 a 1,1; 1024×640 a 1,25; 853×533 a 1,5; 640×400 a 2. Il progetto a ~1440×900 della mappa diventa 1152×720 a 1,25 e 960×600 a 1,5.

## 4. Prestazioni su un Mac modesto

- Animare solo `transform` e `opacity`, che restano nella fase di composizione (fonti/webdev-animations-guide.md:252) [V]. `will-change` va usato con parsimonia (fonti/webdev-animations-guide.md:256) [V]: sulla carta lo si mette solo per la durata del giro [INF].
- Nelle transizioni Svelte conviene `css` a `tick`, perché le Web Animations possono girare fuori dal thread principale (fonti/svelte-docs-transition.md:114) [V].
- Memoria degli strati [M] (aritmetica, 4 byte per pixel a densità 2): una faccia di carta da 220×300 px CSS pesa 1,01 MiB, quindi 12 carte a due facce, tutte promosse insieme, fanno 24,2 MiB. Va bene su 8 GiB, purché le carte non restino promosse a riposo [INF].
- Nessuna animazione in loop: la spec la esclude già (storia 156) e il ridisegno continuo è il costo vero ([[movimento-e-feedback]], § Costo).

## Per il nostro gioco

**Raccomandazione (non decisione):**

- **Movimento**: CSS e Web Animations, nessuna libreria. Un fattore `k` (1, 0,5, 0) scritto in `--k` e letto da `dur()`; un booleano `spento` con valore iniziale da `prefersReducedMotion`. Carta con `preserve-3d` e dissolvenza come variante ridotta; mobile con `el.animate` e curva `linear()` a molla, che il tono grave sostituisce con una curva senza rimbalzo; caselle con `steps()`; esito con `delay: i * passo * k`. `Spring` solo se arriva un trascinamento.
- **Scala**: CSS `zoom` sull'elemento `<html>`, scala da 1 in su, salvata con le impostazioni del giocatore (dove vivono resta aperto nella mappa, «Impostazioni del giocatore»). `setZoom` di Tauri resta come ripiego se CSS `zoom` mostra difetti di coordinate in WKWebView. `transform: scale` escluso. `rem` ovunque si può tenere comunque per la tipografia, ma non serve come meccanismo di scala.
- **Da verificare nel primo prototipo nel guscio** (una sessione, nessun codice nuovo oltre il prototipo): media query e coordinate sotto CSS `zoom`; `prefers-reduced-motion` letto da WKWebView dentro Tauri (atteso sì, perché è lo stesso WebKit di Safari, ma non misurato) [INF]; fluidità della carta e del mobile sul Mac di Mario.
- **Aperto per Mario**: i gradini della scala (per esempio 1 / 1,1 / 1,25 / 1,5) e se il layout deve reggere 1024×640 effettivi, cioè 1,25 sulla finestra minima.

## Collegamenti

- [[movimento-e-feedback]]: ritmo dell'esito e WCAG 2.3.3; qui l'esclusione dell'opacità e l'implementazione.
- [[succo-per-interfacce-di-carte-e-numeri]]: durate e curve che il fattore `k` moltiplica.
- [[tauri]]: WKWebView legato a macOS.
- [[tipografia-e-font-aperti]]: il minimo di 12 px e le cifre tabellari.
- [[prestazioni-su-hardware-modesto]]: il quadro generale.

## Artefatti consultati

- Repo `simulatore-politico`, branch `main`, HEAD `c56787d`: `app/package.json`, `app/package-lock.json:1396-1397`, `app/src-tauri/Cargo.toml:15`, `app/src-tauri/tauri.conf.json`, `app/src-tauri/capabilities/default.json`, `Cargo.lock:3245-3246, 4228-4229`, spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md:224-228, 401`.
- Registro cargo locale: `tauri-2.12.3/src/webview/mod.rs:1115-1127, 2346-2360`, `tauri-2.12.3/src/manager/webview.rs:578-596`, `tauri-2.12.3/permissions/webview/autogenerated/reference.md:1-12` (insieme predefinito senza zoom), `tauri-utils-2.10.2/src/config.rs:694-695`. Sorgenti `svelte@5.57.2` e `@tauri-apps/api@2.12.3` scaricati dal registro npm nello scratchpad e confrontati con sha256 con le copie salvate in `fonti/`.
- `mdn/browser-compat-data` al commit `0efedff8c8b6f131ad029443056bc74e00b1aca2` (31/10/2026), letto con `jq`.

## Riferimenti

- **URL** https://raw.githubusercontent.com/mdn/browser-compat-data/0efedff8c8b6f131ad029443056bc74e00b1aca2/css/types/easing-function.json · [V]
  **perché conta qui** dati di compatibilità primari delle curve in Safari
  **cosa se ne prende** `linear()` da Safari 17.2, `steps()` da 5.1, `jump-*` da 14, `cubic-bezier()` da 6
- **URL** https://raw.githubusercontent.com/mdn/browser-compat-data/0efedff8c8b6f131ad029443056bc74e00b1aca2/css/properties/backface-visibility.json · [V]
  **perché conta qui** la carta che si gira ha bisogno di nascondere il retro
  **cosa se ne prende** senza prefisso da Safari 15.4, quindi niente `-webkit-` su Safari 27
- **URL** https://raw.githubusercontent.com/mdn/browser-compat-data/0efedff8c8b6f131ad029443056bc74e00b1aca2/css/properties/transform-style.json · [V]
  **perché conta qui** `preserve-3d` regge la carta a due facce
  **cosa se ne prende** da Safari 9; pseudo-elementi esclusi dal 3D prima di Safari 17: le facce vanno fatte con elementi veri
- **URL** https://raw.githubusercontent.com/mdn/browser-compat-data/0efedff8c8b6f131ad029443056bc74e00b1aca2/css/properties/zoom.json · [V]
  **perché conta qui** CSS `zoom` è il candidato per la scala della UI
  **cosa se ne prende** Safari da 3.1, non animabile, `reset` rimosso in 18.4
- **URL** https://raw.githubusercontent.com/mdn/browser-compat-data/0efedff8c8b6f131ad029443056bc74e00b1aca2/api/Animation.json · [V]
  **perché conta qui** velocità e «salta» passano dalla Web Animations API
  **cosa se ne prende** `finish()`, `finished`, `playbackRate`, `updatePlaybackRate()` da Safari 13.1
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/easing-function/linear · [V]
  **perché conta qui** è la curva con cui la molla si fa in CSS
  **cosa se ne prende** valori fuori da 0-1 ammessi, cioè il superamento che fa oscillare il mobile
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/backface-visibility · [V]
  **perché conta qui** semantica della faccia posteriore
  **cosa se ne prende** ha effetto solo con trasformazioni 3D
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/zoom · [V]
  **perché conta qui** confronto fra `zoom` e `scale()`
  **cosa se ne prende** `zoom` ricalcola il layout, `scale()` no
- **URL** https://developer.mozilla.org/en-US/docs/Web/API/Document/getAnimations · [V]
  **perché conta qui** è lo strumento proposto per la velocità globale
  **cosa se ne prende** vede solo le animazioni in corso: va bene per «salta», non come manopola globale
- **URL** https://developer.mozilla.org/en-US/docs/Web/API/Animation/updatePlaybackRate · [V]
  **perché conta qui** cambiare velocità mentre l'esito si sta rivelando
  **cosa se ne prende** `updatePlaybackRate()` invece di assegnare `playbackRate`, per evitare il salto
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transform · [V]
  **perché conta qui** effetti collaterali di `transform` sulla radice
  **cosa se ne prende** crea un contenitore per i figli `position: fixed`: un altro motivo per scartare `transform: scale` come scala
- **URL** https://webkit.org/blog/14787/webkit-features-in-safari-17-2/ · [V]
  **perché conta qui** fonte di prima mano su `linear()` in WebKit
  **cosa se ne prende** `linear()` in Safari 17.2, disponibile anche su Monterey e Ventura
- **URL** https://developer.chrome.com/docs/css-ui/css-linear-easing-function · [V]
  **perché conta qui** come fare molle e rimbalzi con `linear()`
  **cosa se ne prende** il generatore di Archibald e Argyle con il preset Spring; più punti, curva più liscia
- **URL** https://svelte.dev/docs/svelte/svelte-motion/llms.txt · [V]
  **perché conta qui** API di `Spring`, `Tween`, `prefersReducedMotion` in Svelte 5
  **cosa se ne prende** `Spring` dalla 5.8 con `instant`; `prefersReducedMotion` dalla 5.7 come valore iniziale dell'interruttore
- **URL** https://svelte.dev/docs/svelte/transition/llms.txt · [V]
  **perché conta qui** le transizioni di Svelte sono il veicolo principale del movimento
  **cosa se ne prende** guidate da Web Animations, quindi una regola CSS di movimento ridotto non le tocca; `css` invece di `tick`
- **URL** https://cdn.jsdelivr.net/npm/svelte@5.57.2/src/internal/client/dom/elements/transitions.js · [V]
  **perché conta qui** il sorgente della versione bloccata nel repo
  **cosa se ne prende** keyframe a 60 al secondo, transizione con ritardo in due animazioni (rilevante per «salta»)
- **URL** https://cdn.jsdelivr.net/npm/svelte@5.57.2/src/motion/spring.js · [V]
  **perché conta qui** capire se `Spring` segue la velocità globale
  **cosa se ne prende** ciclo `requestAnimationFrame` senza scala del tempo; solo `instant`
- **URL** https://unpkg.com/@tauri-apps/api@2.12.3/webview.d.ts · [V]
  **perché conta qui** API JS dello zoom nella versione del repo
  **cosa se ne prende** `setZoom` vuole un permesso fuori dal predefinito; nessun `getZoom`; `zoomHotkeysEnabled` è un polyfill su macOS
- **URL** https://raw.githubusercontent.com/tauri-apps/tauri/tauri-v2.12.3/crates/tauri/src/webview/scripts/zoom-hotkey.js · [V]
  **perché conta qui** come si comporta davvero lo zoom da tastiera di Tauri
  **cosa se ne prende** contatore che riparte da 1, passi del 20%, nessuna memoria: da lasciare spento
- **URL** https://raw.githubusercontent.com/tauri-apps/wry/wry-v0.57.0/src/wkwebview/mod.rs · [V]
  **perché conta qui** cosa fa `setZoom` su macOS sotto il cofano
  **cosa se ne prende** chiama `WKWebView.setPageZoom`, macOS 11+
- **URL** https://developer.apple.com/documentation/webkit/wkwebview/pagezoom · [V]
  **perché conta qui** semantica di `pageZoom` dichiarata da Apple
  **cosa se ne prende** equivale a CSS `zoom` su tutta la pagina: si può fare la stessa cosa in CSS senza permessi
- **URL** https://bugs.webkit.org/show_bug.cgi?id=27684 · [V]
  **perché conta qui** prova primaria che `transform: scale` sfoca in WebKit
  **cosa se ne prende** strato composto disegnato piccolo e ingrandito; aperto dal 2009, commentato il 28/10/2026
- **URL** https://bugs.webkit.org/show_bug.cgi?id=264954 · [V]
  **perché conta qui** caso di scala con strati figli, e il ripiego proposto
  **cosa se ne prende** `zoom` al posto di `scale` come ripiego, ma il testo non si comporta uguale a ogni livello
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html · [V]
  **perché conta qui** il criterio che misura una scala della UI
  **cosa se ne prende** 200% senza perdite a ogni passo intermedio; manopola dell'autore ammessa (G178)
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html · [V]
  **perché conta qui** decide che cosa può restare quando il movimento è spento
  **cosa se ne prende** l'opacità non è «motion animation», la sfocatura sì (errata): dissolvenze ammesse, sfocature no
- **URL** https://drafts.csswg.org/mediaqueries-4/ · [V]
  **perché conta qui** limite dell'approccio `rem` per la scala
  **cosa se ne prende** le unità relative nelle media query usano il valore iniziale, non il `font-size` della pagina
- **URL** https://drafts.csswg.org/css-transitions-1/ · [V]
  **perché conta qui** cosa succede a «istantaneo» con durate a 0
  **cosa se ne prende** durata 0 = nessuna transizione e nessun `transitionend`
- **URL** https://web.dev/articles/animations-guide · [V]
  **perché conta qui** regole di costo del movimento su hardware modesto
  **cosa se ne prende** solo `transform` e `opacity`; `will-change` con parsimonia
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion · [V] (cattura del 17/10, fonti/mdn-prefers-reduced-motion.md)
  **perché conta qui** valore iniziale dell'interruttore
  **cosa se ne prende** dove sta l'impostazione su macOS; Baseline da gennaio 2020
