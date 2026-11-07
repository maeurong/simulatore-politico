Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Cosa regge il webview di Tauri su macOS (WKWebView) per il movimento e la scala della UI (storie 154-158)? Carta del ministro che si gira (CSS 3D), mobile appeso che oscilla solo a un gesto (molla), tracciati a caselle che avanzano a scatti, esito rivelato una voce alla volta; velocità 1×/2×/istantaneo e interruttore del movimento sopra `prefers-reduced-motion`; scala della UI regolabile (zoom del webview, `rem` sulla radice o trasformazione) senza rompere il testo minimo di 12 px a 1×. Parte da `docs/ricerca/estetica/movimento-e-feedback.md` e `docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md`.

## Answer

Pagina: [movimento-e-scala-in-wkwebview](../../../docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md). Sul Mac di Mario (macOS 27.0.1, Safari 27) WKWebView regge tutto ciò che serve. Raccomandazione, da verificare nel primo prototipo dentro il guscio:

- Movimento con CSS e Web Animations, nessuna libreria. Velocità come fattore `k` (1 / 0,5 / 0) scritto in `--k` e letto da una `dur()`; interruttore `spento` che parte da `prefersReducedMotion`. Le transizioni Svelte passano dalla Web Animations API, quindi una regola CSS su `prefers-reduced-motion` non le tocca; «salta» = `k` a 0, poi `finish()` su `getAnimations()`. A movimento spento restano le dissolvenze (per WCAG l'opacità non è movimento).
- Carta: `preserve-3d`, con la dissolvenza come variante ridotta. Mobile: `el.animate` con una curva `linear()` a molla, e il tono grave la sostituisce con una curva senza rimbalzo. Caselle: `steps()`. Esito: ritardo `i * passo * k`. `Spring` di `svelte/motion` non segue il 2×: solo per un eventuale trascinamento.
- Scala: CSS `zoom` su `<html>`, da 1 in su, salvato con le impostazioni del giocatore. Equivale al `setZoom` di Tauri, che però chiede un permesso assente dalla capability del repo. `transform: scale` escluso, perché sfoca il testo (bug WebKit 27684).
- Animare solo `transform` e `opacity`, `will-change` solo durante il giro: 12 carte a due facce, promosse insieme, occupano circa 24 MiB.
- Da misurare nel guscio: media query e coordinate del puntatore sotto CSS `zoom`, `prefers-reduced-motion` letto da WKWebView.
