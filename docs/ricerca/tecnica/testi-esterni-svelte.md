---
tipo: tecnica
fonti: [fonti/mdn-intl-pluralrules.md, fonti/mdn-intl-numberformat-costruttore.md, fonti/mdn-font-variant-numeric.md, fonti/mdn-math-random.md, fonti/vite-features-json.md, fonti/typescript-resolvejsonmodule.md, fonti/tauri-resources.md, fonti/github-svelte-i18n.md, fonti/github-typesafe-i18n.md, fonti/github-paraglide-js.md, fonti/svelte-cli-paraglide.md, fonti/github-wuchale.md, fonti/fluent-syntax-guide-selectors.md, fonti/unicode-messageformat-2.md, fonti/github-tc39-intl-messageformat.md]
aggiornato: 2026-11-01
---
# Testi esterni dell'interfaccia in Svelte

Come tenere fuori dal codice i testi della UI (storia 166) in un'app Svelte 5 + SvelteKit statico dentro Tauri 2, con una sola lingua. La piattaforma copre già plurali e numeri all'italiana (`Intl`), Vite importa il JSON e TypeScript ne ricava i tipi delle chiavi: con un file JSON importato e una funzione di una dozzina di righe una chiave mancante diventa un errore di `npm run check`. Le librerie i18n risolvono soprattutto il cambio di lingua, che la v1 non ha.

## Artefatti consultati

Repo `/Users/mario/GitHub/simulatore-politico`, branch `main`, HEAD `c56787d`.

- `app/package.json:11` (script `check` = `svelte-check`), `:16-18` (unica dipendenza runtime `@tauri-apps/api`).
- `app/tsconfig.json:8` (`resolveJsonModule: true`, già attivo).
- `app/src/routes/+page.svelte:7` (etichette scritte nel componente oggi).
- `app/src-tauri/src/lib.rs:22-25` (il guscio legge `dati/base` dalla cartella delle risorse), `:26-32` e `:60` (il guscio sceglie il seme dall'orologio).
- `app/src-tauri/tauri.conf.json` (`bundle.resources` copia `dati/base/`).
- `crates/sim-core/src/partita.rs:136` (`Partita::nuova(dati, seme)`; nessun metodo pubblico restituisce il seme).
- `dati/base/dicasteri.toml:8` e seguenti (`nome = "Economia e Finanze"`: i nomi delle entità vivono già nei dati del core).
- `docs/superpowers/specs/2026-10-21-v1-giocabile.md:239` (storia 166), `:400` (tono grave deciso dal modello; spariscono testata, nome spiritoso, battute, motti).
- `docs/superpowers/plans/2026-10-21-v1-piano-1-fondamenta.md:26` (testi esternalizzati rimandati al piano 11; messaggi di `Errore` restano nel codice).
- `.scratch/v1-piano-11-ui/map.md` (Out of scope: traduzioni; Vista e `sim-core` non si toccano se non serve).
- grep `seme|grave|tono|genere` in `app/src/lib/tipi/*.ts`: **0 risultati**. La Vista non porta né il seme, né il registro, né il genere dei ministri [M].

## Due tipi di testo, due posti

- **Nomi delle entità** (dicasteri, atti, disposizioni, regioni): stanno già nei TOML di `dati/base/` con il campo `nome` (`dati/base/dicasteri.toml:8`; `grep -c nome dati/base/*.toml` dà 155 in `disposizioni.toml`, 40 in `atti.toml`) e arrivano alla UI dentro la Vista [M]. Restano lì: sono dati di gioco, letti a runtime dal guscio (`lib.rs:22-25`).
- **Testi della cornice** (etichette, titoli di sezione, frasi dell'esito, titoli del Pungolo, motti): sono della UI e vanno in un file della UI [INF]. È questo il perimetro della domanda.

## Formato

| Formato | Plurali/selezione | Commenti | Nel webview | Tipi delle chiavi |
|---|---|---|---|---|
| JSON | a mano (oggetto `{one, many, other}`) | no | `import` nativo di Vite (fonti/vite-features-json.md:391-397) [V] | gratis con `resolveJsonModule`: TypeScript genera il tipo dalla forma del JSON (fonti/typescript-resolvejsonmodule.md:11) [V] |
| TOML | a mano | sì | serve un parser: `smol-toml` 1.9.1, 5,6 kB gzip [M], o un plugin Vite | solo con un generatore |
| Fluent (FTL) | select con varianti; numeri confrontati con la categoria CLDR; variante di default obbligatoria (fonti/fluent-syntax-guide-selectors.md:20-22) [V] | sì | `@fluent/bundle` 0.19.1, 4,1 kB gzip, ultima pubblicazione 02/04/2025 [M] | no, chiavi stringa |
| ICU MessageFormat 1 | `{n, plural, one {…} other {…}}`, `select` | no | `intl-messageformat` 12.1.4, 10,0 kB gzip [M] | no |
| MessageFormat 2 | standard Unicode (fonti/unicode-messageformat-2.md:15) [V] | sì | `messageformat` 4.1.0, 8,2 kB gzip [M]; `Intl.MessageFormat` nativo è allo **Stage 1** TC39 (fonti/github-tc39-intl-messageformat.md:19) [V] e manca sia in Node 26.7.0 sia in JavaScriptCore di macOS 27.0.1 (`typeof Intl.MessageFormat` → `undefined`) [M] | no |

Lettura [INF]: Fluent e MessageFormat valgono quando un traduttore deve esprimere grammatiche diverse senza toccare il codice. Con il solo italiano la loro forza (selezione dentro la stringa) costa una dipendenza e toglie il controllo statico delle chiavi. TOML sarebbe coerente con `dati/`, ma nel webview richiede un parser e perde i tipi gratuiti. JSON non ha commenti, ma è l'unico formato che Vite e TypeScript leggono senza aggiunte.

## Caricamento: webview o core

- **Import statico nel webview** (consigliato [INF]): `import testi from "$lib/testi/interfaccia.json"`. Vite lo mette nel bundle; con `tauri dev` la modifica si vede subito via HMR. Una correzione richiede un nuovo build, ma la logica non si tocca, che è quanto chiede la storia 166.
- **File di risorsa letto a runtime dal webview**: Tauri lo permette con `resolveResource` + `readTextFile`, ma serve il plugin `fs` con i permessi `fs:allow-read-text-file` e `fs:allow-resource-read-recursive` (fonti/tauri-resources.md:191-212) [V]. Costi [INF]: una dipendenza e un permesso nuovi, caricamento asincrono prima del primo disegno, chiavi non più controllate a build. L'unico vantaggio è correggere i testi nell'`.app` installata senza ricompilare, cosa che la v1 (solo Mario) non chiede.
- **Passati dal core o dal guscio**: il core dovrebbe leggere un altro TOML e la Vista portarlo, contro il vincolo della mappa («`sim-core` e la Vista non si toccano se non serve»); un comando del guscio che legge un file non è un passacarte verso un metodo della partita. In entrambi i casi le chiavi tornano stringhe non controllate [INF]. Scartato.

## Plurali e numeri all'italiana

Misurati in sessione con Node v26.7.0 e con il `jsc` di sistema (`/System/Library/Frameworks/JavaScriptCore.framework/Versions/Current/Helpers/jsc`, macOS 27.0.1, lo stesso motore del WKWebView di Tauri), script in scratchpad; stessi risultati in entrambi [M]:

- `new Intl.PluralRules("it").resolvedOptions().pluralCategories` → `["one","many","other"]`; `select`: 0→other, 1→one, 2→other, 1,5→other, 1000000→**many** [M]. `many` copre «1.000.000 **di** elettori»: la funzione dei plurali deve ricadere su `other` se la forma `many` manca. MDN elenca le sei categorie possibili e il metodo `select()` (fonti/mdn-intl-pluralrules.md:29) [V].
- Con le cifre decimali il plurale cambia: `new Intl.PluralRules("it",{minimumFractionDigits:1}).select(1)` → `other` («1,0 punti»), senza opzioni → `one` [M]. Le opzioni di formato vanno passate uguali a `PluralRules` e a `NumberFormat`; Fluent prescrive la stessa regola (fonti/fluent-syntax-guide-selectors.md:24) [V].
- Ordinali: `type:"ordinal"` → `many` per 8, 11, 80, 800, `other` per 1 e 7 [M]: è la categoria dell'apostrofo («l'8° turno», «l'11»). Torna utile per l'articolo davanti ai numeri.
- `new Intl.NumberFormat("it-IT").format(1234)` → **`1234`**, mentre 12345 → `12.345` [M]. In italiano il separatore delle migliaia scatta da cinque cifre: una colonna con 1234 e 12.345 ne avrebbe uno sì e uno no. Con `useGrouping:"always"` → `1.234` [M]; MDN: `"always"` mostra i separatori «anche se la locale preferisce altrimenti», il default è `"auto"` (fonti/mdn-intl-numberformat-costruttore.md:305-325) [V]. Nelle tabelle e nei cruscotti serve `"always"`.
- Altri formati misurati [M]: percentuale `31,7%` (con `style:"percent"`, `minimumFractionDigits:1`); euro `1234,50 €` (stesso problema delle migliaia: anche qui `useGrouping:"always"`); `notation:"compact"` → `2,5 Mld`, con `compactDisplay:"long"` → `2,5 miliardi`; `signDisplay:"exceptZero"` → `-1,3` e `0` per 0,04 arrotondato; `Intl.ListFormat("it")` → `Lega, FI e FdI`.
- Il segno meno prodotto è U+002D (trattino), non U+2212 [M]. In colonna è più stretto delle cifre tabellari; si può sostituire con `−` nella funzione di formato [INF].
- **Cifre tabellari**: non sono affare di `Intl` ma del CSS, `font-variant-numeric: tabular-nums` (OpenType `tnum`) (fonti/mdn-font-variant-numeric.md:159) [V]. Va acceso sui pannelli numerici (vedi [[tipografia-e-font-aperti]]). Che Atkinson Hyperlegible Next e Fraunces, i font della spec (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:397`), abbiano davvero `tnum`: [NON TROVATO] in questa sessione, da verificare sul file del font.
- I formattatori vanno creati una volta a livello di modulo, non a ogni render [INF].

## Varianti a caso con seme fisso (titoli del Pungolo)

- `Math.random()` non accetta un seme: «cannot be chosen or reset by the user» (fonti/mdn-math-random.md:21) [V]. Con `Math.random` un titolo cambierebbe a ogni nuovo disegno e nel registro («torna al turno N») [INF].
- Soluzione senza dipendenze [INF]: nel JSON la chiave ha un array di varianti; la scelta è `hash(seme|chiave|turno) % varianti.length`, con un hash di poche righe (FNV-1a a 32 bit). Funzione pura: stesso turno, stesso titolo, anche rileggendo il registro. Coerente con la storia 165 (stesso seme, stessa partita).
- **Da dove viene il seme**: la Vista non lo porta (grep vuoto, sopra) e `Partita` non ha un metodo pubblico che lo restituisca (`partita.rs:136`) [M]. Il guscio però lo sceglie da sé (`lib.rs:26-32`, `:60`) e può restituirlo accanto alla Vista senza toccare il core [INF]. Coi salvataggi (piano 10) il seme sta nell'istantanea (`partita.rs:3508`): rimando al piano 10. Ripiego senza seme: `hash(chiave|turno|anno)`, che però ripete gli stessi titoli in ogni partita.
- Si può anche fare scegliere la variante al core, che ha già un generatore ChaCha (`crates/sim-core/Cargo.toml:7`), ma il core dovrebbe conoscere i testi della UI [INF]. Scartato.

## Tono grave separato dal giocoso

- La spec dice cosa succede in registro grave: spariscono testata, nome spiritoso, battute, motti e illustrazioni (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:400`); la soglia è un evento del modello, non una scelta del singolo testo (estetica/tono-giocoso-su-temi-seri.md:47).
- Ne segue [INF] che il grave non chiede testi alternativi per ogni battuta: chiede che **le battute non compaiano**. Basta un file per registro: `interfaccia.json` (neutro, sempre visibile: occhiello col fatto e il numero, etichette) e `pungolo.json` (titoli spiritosi, motti, battute). I componenti del registro grave importano solo il primo, e il controllo è un import (vietato da un grep in un test o da una regola di lint), non una convenzione di chi scrive.
- I pochi testi che in grave cambiano invece di sparire (per esempio un titolo sobrio) vanno in un terzo file `grave.json`, che dichiara con `satisfies` di coprire le chiavi che sostituisce [INF].
- Da dove arriva il segnale «registro grave»: la Vista non lo porta (grep vuoto) [M]. Rimando al piano che produce gli eventi gravi (piano 9).

## Controllo delle chiavi mancanti

Misurato con `npx -p typescript@6.0.3 tsc --noEmit --strict --resolveJsonModule --module esnext --moduleResolution bundler` (la stessa versione di `app/package.json`) su una sonda in scratchpad [M]:

- chiave usata nel codice e assente nel JSON → `error TS2345: Argument of type '"turno.titolino"' is not assignable to parameter of type '"turno.titolo" | "esito.seggi" | "pungolo.titolo"'`;
- file di registro a cui manca una chiave richiesta → `error TS2741: Property '"esito.seggi"' is missing in type '{ "turno.titolo": string; }'`.

Quindi, con `t(chiave: keyof typeof testi)`, lo script `check` già presente (`app/package.json:11`) fallisce su ogni chiave sbagliata; che `svelte-check` lo faccia anche dentro i `.svelte` è [INF], perché usa lo stesso type checker. Restano fuori dal controllo statico [INF]:

- **chiavi orfane** (nel JSON e mai usate): un test che cerca ogni chiave nei sorgenti, poche righe;
- **segnaposto** (`{n}` scritto `{num}`): la funzione `t` può lanciare un errore in sviluppo se resta una graffa non sostituita;
- **forme dei plurali**: `satisfies Record<string, string | string[] | {other: string; one?: string; many?: string}>` sul JSON importato impone `other`.

Dove gira il test lo decide il ticket aperto «Verifica della UI» della mappa: oggi l'app non ha un test runner (`app/package.json`).

## Librerie Svelte correnti

Dati misurati il 01/11/2026: `npm view <pacchetto> version time dependencies peerDependencies`, `gh api repos/<repo>` (ultimo push, issue aperte), `https://bundlephobia.com/api/size?package=<pacchetto>` (gzip) [M].

| Libreria | Versione, ultima pubblicazione | Repo: ultimo push, issue aperte | Peso runtime | Note |
|---|---|---|---|---|
| svelte-i18n | 4.0.1, 21/10/2024 | 21/10/2024 (ultimo commit), 74 | 14,6 kB gzip, dipende da `intl-messageformat` | il README annuncia una rifattorizzazione «quando troverò il tempo» (fonti/github-svelte-i18n.md:11) [V]; basata sugli store (fonti/github-svelte-i18n.md:19) [V]. **Ferma da due anni.** |
| @inlang/paraglide-js | 2.29.3, 28/10/2026 | 30/10/2026, 18 | codice generato, funzioni per messaggio eliminabili dal bundler (fonti/github-paraglide-js.md:56, :64) [V]; pacchetto di build da 3,7 MB con `@inlang/sdk` e altre 9 dipendenze | integrazione i18n ufficiale di SvelteKit (fonti/github-paraglide-js.md:24) [V]; `sv add paraglide` aggiunge progetto inlang, plugin Vite, hook `reroute` e `handle` (fonti/svelte-cli-paraglide.md:15-21) [V]; chiavi tipizzate, plurali con `Intl.PluralRules` (fonti/github-paraglide-js.md:65, :270) [V]. **Attiva.** |
| typesafe-i18n | 5.27.1, 11/02/2026 | 22/03/2026, 42 | ~1 kB (fonti/github-typesafe-i18n.md:189-200) [V]; 1,3 kB gzip [M] | autore Ivan Hofer (1995-2023) (fonti/github-typesafe-i18n.md:19) [V]; repo passato a `codingcommons`. **Manutenzione a singhiozzo.** |
| wuchale | 0.26.7, 19/09/2026 | 26/10/2026, 19 | 217 kB di pacchetto; cataloghi come array | estrae il testo dai componenti senza chiavi, file PO (fonti/github-wuchale.md:20, :28) [V]. Pre-1.0. Il testo sorgente resta nel componente, l'opposto della storia 166 [INF]. |
| sveltekit-i18n | 3.4.1, 07/10/2026 | 20/10/2026, 18 | 40 kB di pacchetto | peer `svelte >=5` [M]. Attiva. |

Lettura [INF]: Paraglide è l'unica scelta da considerare se un giorno arrivano altre lingue. Oggi porterebbe routing per lingua, hook di server inutili in una SPA statica e un compilatore in più nel build, per ottenere ciò che `keyof typeof` dà gratis. svelte-i18n è da evitare (ferma, store invece di rune). typesafe-i18n è leggera ma senza manutentore stabile.

## Per il nostro gioco

**Raccomandazione [INF], non decisione:**

- JSON importato staticamente da `app/src/lib/testi/`, un file per registro (`interfaccia.json`, `pungolo.json`, eventuale `grave.json`); nomi delle entità lasciati nei TOML di `dati/`.
- Una funzione `t` in un modulo solo: chiave tipizzata, sostituzione `{nome}`, plurale con `Intl.PluralRules("it")` (`many` → `other`), varianti scelte con `hash(seme|chiave|turno)`. Formattatori `Intl.NumberFormat("it-IT", { useGrouping: "always" })` creati una volta; `tabular-nums` in CSS sui pannelli numerici.
- Controllo: `npm run check` per le chiavi mancanti (misurato); un test per orfane e segnaposto quando la mappa decide dove girano i test della UI.
- Nessuna dipendenza nuova. Si riapre se arrivano le traduzioni (allora Paraglide) o se servono accordi di genere dentro la frase che il solo oggetto di varianti non regge.

**Aperto:**

- Il seme alla UI: il guscio lo restituisce accanto alla Vista? Coi salvataggi, rimando al piano 10.
- Il segnale del registro grave nella Vista: rimando al piano 9.
- Genere dei ministri («il ministro» / «la ministra»): la Vista non ha il campo [M]; se arriva, stesso meccanismo dei plurali (oggetto `{f, m}`).
- `tnum` in Atkinson Hyperlegible Next e Fraunces: [NON TROVATO].

## Riferimenti

- **URL** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules · [V]
  **perché conta qui** API nativa dei plurali nel webview, senza librerie
  **cosa se ne prende** categorie CLDR e `select()`; per l'italiano `one`/`many`/`other` misurati in WebKit
- **URL** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat/NumberFormat · [V]
  **perché conta qui** numeri all'italiana nel cruscotto e nelle tabelle
  **cosa se ne prende** `useGrouping: "always"` contro il default che lascia 1234 senza punto
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric · [V] (cattura già presente)
  **perché conta qui** colonne di numeri che non ballano tra un turno e l'altro
  **cosa se ne prende** `tabular-nums` = OpenType `tnum`, in CSS e non in `Intl`
- **URL** https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Math/random · [V]
  **perché conta qui** varianti dei titoli che devono restare fisse
  **cosa se ne prende** il seme di `Math.random` non si sceglie: serve un hash proprio
- **URL** https://vite.dev/guide/features.html · [V]
  **perché conta qui** come il JSON entra nel bundle senza dipendenze
  **cosa se ne prende** import diretto del JSON, anche con import nominali
- **URL** https://www.typescriptlang.org/tsconfig/resolveJsonModule.html · [V]
  **perché conta qui** il controllo delle chiavi mancanti a costo zero
  **cosa se ne prende** il tipo generato dalla forma del JSON, base di `keyof typeof testi` (già attivo in `app/tsconfig.json:8`)
- **URL** https://v2.tauri.app/develop/resources/ · [V]
  **perché conta qui** alternativa di caricamento a runtime dal webview
  **cosa se ne prende** prezzo della via scartata: plugin `fs` e permessi `fs:allow-resource-read-recursive`
- **URL** https://github.com/kaisermann/svelte-i18n · [V]
  **perché conta qui** la libreria i18n Svelte più nota
  **cosa se ne prende** stato: ferma dal 21/10/2024, rifattorizzazione rimandata; da escludere
- **URL** https://github.com/ivanhofer/typesafe-i18n · [V]
  **perché conta qui** alternativa leggera e tipizzata
  **cosa se ne prende** ~1 kB, ma autore scomparso nel 2023 e manutenzione a singhiozzo
- **URL** https://github.com/opral/paraglide-js · [V]
  **perché conta qui** l'integrazione i18n ufficiale di SvelteKit, attiva
  **cosa se ne prende** il candidato se arrivano altre lingue; oggi costo di build sproporzionato
- **URL** https://svelte.dev/docs/cli/paraglide · [V]
  **perché conta qui** cosa installa davvero l'add-on ufficiale
  **cosa se ne prende** progetto inlang, plugin Vite, hook `reroute`/`handle`: pensati per il routing per lingua
- **URL** https://github.com/wuchalejs/wuchale · [V]
  **perché conta qui** i18n per Svelte a estrazione automatica, recente
  **cosa se ne prende** tiene il testo nei componenti: incompatibile con la storia 166
- **URL** https://projectfluent.org/fluent/guide/selectors.html · [V]
  **perché conta qui** il formato con selezione più espressivo fra i candidati
  **cosa se ne prende** due regole che valgono anche a mano: variante di default obbligatoria, stesse opzioni di formato per scegliere il plurale
- **URL** https://messageformat.unicode.org/ · [V]
  **perché conta qui** lo standard Unicode MF2, candidato formato
  **cosa se ne prende** esiste ed è uno standard, ma richiede una libreria (8,2 kB gzip misurati)
- **URL** https://github.com/tc39/proposal-intl-messageformat · [V]
  **perché conta qui** se MF2 diventerà nativo nel webview
  **cosa se ne prende** Stage 1: non conta per la v1; `Intl.MessageFormat` assente in WebKit (misurato)

## Collegamenti

- [[tono-giocoso-su-temi-seri]]: i due registri con soglia; qui diventano due file e un import vietato.
- [[tipografia-e-font-aperti]]: cifre tabellari; da verificare `tnum` nei font della spec.
- [[modding-e-dati-esterni]]: i dati di gioco in TOML; i testi della cornice restano fuori da quel livello.
- [[tauri]]: risorse e permessi del guscio.
