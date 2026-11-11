Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

In che formato e come si caricano i testi dell'interfaccia esternalizzati (storia 166) in un'app Svelte 5 dentro Tauri, senza dipendenze se basta poco codice? Formati candidati (JSON, TOML, Fluent, ICU MessageFormat), plurali e numeri all'italiana (`Intl.PluralRules`, `Intl.NumberFormat` con cifre tabellari), testi con varianti a caso ma a seme fisso (titoli spiritosi del Pungolo), testi del tono grave separati da quelli giocosi, controllo che nessuna chiave manchi.

## Answer

Pagina: [testi-esterni-svelte](../../../docs/ricerca/tecnica/testi-esterni-svelte.md). Raccomandazione, da confermare nel piano:

- JSON importato staticamente da Vite in `app/src/lib/testi/`, un file per registro: `interfaccia.json` (neutro) e `pungolo.json` (titoli spiritosi, motti, battute), che i componenti del tono grave non importano. Nessuna dipendenza: svelte-i18n è ferma dal 2024, Paraglide solo se arrivano traduzioni.
- Una funzione `t` con chiave tipizzata (`keyof typeof`), segnaposto, plurale con `Intl.PluralRules("it")` (`one`/`many`/`other`, `many` ricade su `other`), variante con `hash(seme|chiave|turno) % n`. Il seme oggi non arriva alla UI: lo sceglie il guscio (`app/src-tauri/src/lib.rs:26-32`), salvataggi al piano 10.
- Numeri: `Intl.NumberFormat("it-IT", {useGrouping: "always"})`, perché di default 1234 resta senza punto (misurato in JavaScriptCore); `tabular-nums` in CSS.
- `npm run check` segnala le chiavi mancanti (TS2345, TS2741, misurato). Orfane e segnaposto richiedono un test: dipende dalla verifica della UI, ancora in nebbia.
- Aperti: il segnale del tono grave nella Vista (piano 9); `tnum` in Fraunces e Atkinson Next non verificato; genere dei ministri non nella Vista.
