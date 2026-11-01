---
tipo: estetica
fonti: [fonti/istat-confini-amministrativi-2026.md, fonti/istat-note-legali.md, fonti/naturalearth-admin1-province.md, fonti/geojson-italy-readme.md, fonti/mapshaper-riferimento-comandi.md, fonti/colorbrewer-schemi.md, fonti/colorbrewer-controllo-daltonismo.md, fonti/datawrapper-daltonismo-2.md, fonti/machado-2009-simulazione-cvd.md, fonti/w3c-wcag22-dimensione-bersaglio.md, fonti/w3c-wcag22-contrasto-non-testuale.md, fonti/w3c-wcag22-uso-del-colore.md, fonti/w3c-apg-radio-group.md, fonti/w3c-apg-interfaccia-tastiera.md, fonti/mdn-svg-tabindex.md, fonti/apple-wkpreferences-tabfocuseslinks.md, fonti/wry-0-57-0-wkwebview-mod-rs.md, fonti/w3c-svg2-ordine-di-rendering.md]
aggiornato: 2026-11-01
---
# Mappa SVG delle 20 regioni

Da dove viene, con quale licenza e in che forma arriva nell'app l'SVG delle 20 regioni per la mappa a tinte piatte del piano 11 (storie 137-141, decisioni nel ticket 27 della mappa v1). In breve: confini ISTAT generalizzati al 1° gennaio 2026 (CC BY 4.0), un solo comando mapshaper che pesa 38 KB (12 KB compressi) a 500 px di larghezza, isole sopra i 16 km² così Lampedusa resta, segni ancorati ai punti interni e non ai centroidi, regioni come gruppo di radio con tabindex mobile e una lista equivalente per le tre regioni troppo piccole, cinque classi di un inchiostro a passi uguali di luminosità.

Non ripete [[mappe-e-globo]] (Natural Earth, proiezioni, regole delle choropleth) né [[data-viz-cinematica]] (famiglie di scale, daltonismo rosso/verde): ci aggiunge sopra.

Tag: [V] fonte primaria letta · [M] misurato in sessione con comando · [INF] inferenza · [NON TROVATO].

Misure fatte il 01/11/2026 da `/Users/mario/GitHub/simulatore-politico`, branch `main`, HEAD `c56787d`, con `npx -y mapshaper@0.7.101` (rilasciata il 29/10/2026, `gh api repos/mbloch/mapshaper/releases/latest`) su `Limiti01012026_g.zip` (10.450.609 byte, sha256 `b011a590…a81d6`). Gli script di misura sono rimasti nello scratchpad della sessione, fuori dal repo; i comandi chiave sono riportati qui sotto, riga per riga.

## Fonte dei confini

### ISTAT, versione generalizzata 2026: la scelta
- ISTAT pubblica i confini aggiornati al 1° gennaio 2026 in due versioni, generalizzata e più dettagliata [V] (fonti/istat-confini-amministrativi-2026.md:8, :31). Formato shapefile, WGS84 UTM 32N, attributi in UTF-8 [V] (:16, :28-30).
- I dati «potrebbero essere ulteriormente revisionati» per variazioni non ancora pervenute [V] (:10). Per i confini regionali, che cambiano di rado e di pochi comuni, a 2 km per pixel non si vede [INF].
- Licenza: CC BY 4.0 su tutto il sito salvo diversa indicazione; si può modificare e ridistribuire anche a fini commerciali, con menzione, link alla licenza e indicazione delle modifiche, e senza misure tecnologiche che restringano gli altri [V] (fonti/istat-note-legali.md:31-42). Compatibile con un gioco gratuito open source sotto MIT: la geometria resta CC BY e porta la sua attribuzione, il codice resta MIT [INF]. Il repo ha già il precedente: `dati/base/regioni.toml:6-7` dichiara ISTAT CC BY 4.0 e rimanda alle modifiche.
- Lo strato regioni (`Reg01012026_g_WGS84.shp`) pesa 1.054.724 byte, 20 record, campi `COD_REG`, `COD_RIP`, `DEN_REG`, `Shape_Area`, `Shape_Leng` [M] (`mapshaper Reg01012026_g_WGS84.shp -info`). Esploso in parti fa 426 poligoni, di cui 386 sotto i 10 km² [M] (`-explode -each 'a=this.area/1e6' -filter 'a<10' -calc 'count()'`).
- `DEN_REG` è bilingue dove serve («Valle d'Aosta/Vallée d'Aoste», «Trentino-Alto Adige/Südtirol») [M]; il gioco usa `id` in snake case (`dati/base/regioni.toml:11`, `:20`). Serve una tabella `COD_REG → id` di 20 righe [INF].

### Natural Earth admin-1: no per l'Italia
- Pubblico dominio (vedi [[mappe-e-globo]]), ma per l'Italia lo strato admin-1 10m porta le **province**, non le regioni: 110 elementi con un campo `region`, che con `-dissolve region` torna a 20 [M] (`-filter 'adm0_a3=="ITA"' -dissolve region`, file `ne_10m_admin_1_states_provinces` versione 5.1.1). Nomi misti italiano/inglese («Sicily», «Apulia») e province sarde ferme a otto, cioè all'assetto prima delle riforme [M] (`-filter-fields name,type_en,region`).
- Natural Earth stessa lo dice: admin-1 è «il dataset più difficile da tenere aggiornato» [V] (fonti/naturalearth-admin1-province.md:33).
- A parità di pipeline il peso è simile (38.380 byte contro 43.983 di ISTAT, precisione 0,1) [M], quindi non c'è un guadagno che compensi nomi e date sbagliati. L'unico vantaggio è che non serve citare: con ISTAT basta una riga nei crediti [INF].

### geojson-italy (ex openpolis): comodo, ma di seconda mano
- `openpolis/geojson-italy` ora rimanda a `guglielmo/geojson-italy` [M] (`gh api repos/openpolis/geojson-italy --jq .full_name`). Deriva dalle edizioni generalizzate ISTAT, annata 1° gennaio 2026, CC BY 4.0 con l'obbligo di mantenere l'attribuzione ISTAT [V] (fonti/geojson-italy-readme.md:28, :258, :321-325). Il TopoJSON è già semplificato al 20% [V] (:59).
- Il prototipo della direzione artistica usava proprio questa fonte: 20 tracciati in 26.502 byte a 400×525 [M] (`.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html:15`, `:494`, `sed -n 494p | wc -c`).
- Stessi confini, un passaggio in più. Per un artefatto che si genera una volta conviene partire dallo zip ISTAT, che è la fonte primaria [INF].

## Semplificazione e peso

### Strumento: mapshaper
- Licenza MPL 2.0 [M] (`gh api repos/mbloch/mapshaper/contents/LICENSE`). Si usa solo in sviluppo con `npx` e non finisce nel binario; l'SVG prodotto non eredita la MPL [INF].
- Semplificazione di default: Visvalingam pesato [V] (fonti/mapshaper-riferimento-comandi.md:1521). Si può dosare in percentuale, in distanza (`interval=`) o per risoluzione d'uscita (`resolution=1000x800`) [V] (:1537-1539). `keep-shapes` impedisce che un elemento sparisca, ma di un multipoligono salva solo la parte più grande [V] (:1545). `-filter-islands min-area=` toglie gli anelli staccati sotto una soglia [V] (:803-807). Uscita SVG con `id-field=` per l'attributo `id` e `svg-data=` per gli attributi `data-*` [V] (:187, :233).
- **Trappola misurata:** la semplificazione in percentuale cancella le isole. All'1% restano 20 anelli, uno per regione: spariscono Elba, Eolie e Lampedusa [M] (`-simplify 1% keep-shapes`, conteggio dei `M ` nel path). Si semplifica per distanza, con la soglia delle isole dichiarata a parte.

### Peso misurato, larghezza 500 px (altezza 644, circa 2,0 km per px)

| Comando (dopo `-filter-fields DEN_REG`, uscita `format=svg width=500`) | byte | gzip -9 | anelli |
| --- | ---: | ---: | ---: |
| nessuna semplificazione, precisione 0,1 | 451.101 | 116.625 | 237 |
| `-simplify interval=500`, isole ≥ 16 km² | 84.664 | 27.519 | 37 |
| `-simplify interval=1000`, isole ≥ 16 km², precisione 0,1 | 43.983 | 14.943 | 35 |
| **idem, precisione 0,5** | **37.765** | **12.006** | **35** |
| idem, precisione 1 | 29.824 | 10.139 | 35 |
| `-simplify interval=2000`, isole ≥ 16 km² | 21.262 | 7.554 | 27 |
| `-simplify interval=4000` | 10.451 | 3.862 | 21 |

Tutte [M]. `resolution=500x644` dà lo stesso risultato di `interval=1000` (43.993 byte) [M].

- `interval=1000` vuol dire mezzo pixel CSS a 500 px, cioè un pixel fisico su Retina: il bordo non mostra spigoli [INF]. A 2000 si vedono i gradini e Lampedusa sparisce [M] (elenco delle parti con `-explode`).
- 38 KB dentro il bundle sono trascurabili rispetto a un'app Tauri; il peso non è il vincolo, lo è la leggibilità [INF].
- Il render di prova mostra sottili fili chiari fra regioni adiacenti: l'antialiasing di due bordi coincidenti [M] (render con `qlmanage -t` di `fin-0.5.svg`). Rimedio usuale: un tratto del colore dei confini sopra le regioni, oppure lo strato dei soli confini interni con `-innerlines` (11.822 byte) [V] (fonti/mapshaper-riferimento-comandi.md:895-897) [M].
- Proiezione: si tiene UTM 32N del dato ISTAT. Rispetto a una equivalente (LAEA Europa, EPSG:3035) l'area più gonfiata è la Puglia, +1,0%; Calabria +0,9, Basilicata +0,8, tutte le altre sotto lo 0,5% [M] (`-each 'utm=this.area' -proj +proj=laea … -each 'scarto=utm/laea-1'`). La regola «choropleth in proiezione equivalente» di [[mappe-e-globo]] è rispettata nei fatti senza riproiettare.

Comando raccomandato, da tenere accanto all'SVG generato:

```
npx -y mapshaper@0.7.101 Reg01012026_g_WGS84.shp \
  -filter-islands min-area=16km2 \
  -simplify interval=1000 keep-shapes \
  -filter-fields DEN_REG \
  -o regioni.svg format=svg width=500 precision=0.5 id-field=DEN_REG
```

## Isole minori

- Sopravvivono a `interval=1000` con soglia 16 km²: Elba, Pantelleria, Asinara, San Pietro, Ischia, Lipari, Salina, Vulcano, Capraia, La Maddalena, Giglio, Lampedusa, un'isola delle Egadi [M] (`-explode -each 'a=…, cx=…, cy=…'`, identificate per coordinate: [INF] sul nome). Ne restano fuori Capri, Procida, Ponza, Linosa e le Tremiti [INF, sotto soglia].
- A 2 km per pixel 1 px² vale 4 km²: Lampedusa (20 km²) è un punto di 2×2 px, Pantelleria circa 4,5×4,5, l'Elba circa 7,5×7,5 [INF, aritmetica].
- Soglia a 40 km²: 27 anelli e meno puntini, ma cadono Lampedusa e le Eolie [M]. Per un gioco con la situazione «ondata di sbarchi» (spec, storia 132) Lampedusa sulla mappa ha un senso [INF].
- Un'isola fa parte del tracciato della sua regione (`<path>` multiparte): cliccarla vuol dire cliccare la regione, e non diventa un bersaglio a sé [INF].
- Buchi: San Marino resta come foro fra Emilia-Romagna e Marche, visibile nel render; il Vaticano (0,44 km²) non si vede [M].

## Segni fissi: punti di etichetta, non centroidi

- mapshaper dà due ancore: `centroid`, il centroide dell'anello più grande, e `inner`, un punto interno lontano dai bordi [V] (fonti/mapshaper-riferimento-comandi.md:1157-1159).
- Misurato sui tracciati raccomandati, a 500 px: il centroide della **Liguria cade in mare** [M]. Distanza dal bordo in px, centroide → punto interno [M] (script Python sul GeoJSON di `-points centroid` / `-points inner`):

| Regione | centroide | interno |
| --- | ---: | ---: |
| Liguria | fuori | 7,5 |
| Valle d'Aosta | 9,1 | 9,2 |
| Molise | 6,1 | 9,8 |
| Calabria | 9,5 | 19,3 |
| Campania | 10,8 | 17,8 |
| Lazio | 11,3 | 16,8 |
| Puglia | 11,1 | 15,4 |
| Friuli-Venezia Giulia | 14,5 | 16,1 |
| Marche | 13,7 | 16,2 |
| le altre 11 | 13,7-27,7 | 16,2-28,1 |

- Lettura: un segno di 14 px di diametro sta dentro ogni regione tranne la Liguria, al limite; due o tre segni affiancati (atto, cantiere, emergenza) non ci stanno in Liguria, Valle d'Aosta e Molise [INF].
- Proposta [INF]: ancore calcolate una volta con `-points inner` e salvate accanto all'SVG (20 righe), più uno spostamento scritto a mano per le tre regioni piccole, verso il mare o con un filetto. Un solo bollino per regione con un numero, quando i segni sono più d'uno.
- I segni non devono rubare il clic alla regione (`pointer-events: none`) e il loro contenuto va nel nome accessibile della regione (sezione seguente) [INF].

## Regione cliccabile e focalizzabile

### Cosa dicono le fonti
- `tabindex` vale su qualunque elemento SVG ed è supportato da tutti i browser da gennaio 2017 [V] (fonti/mdn-svg-tabindex.md:5, :12).
- Scegliere una regione fra 20 è un gruppo di radio: contenitore `radiogroup` con etichetta, elementi `radio` con `aria-checked` ed etichetta propria [V] (fonti/w3c-apg-radio-group.md:17-20). Con il tabindex mobile entra nella sequenza del Tab solo l'elemento attivo (`tabindex="0"`), gli altri hanno `-1`; il browser porta in vista l'elemento che riceve il fuoco [V] (fonti/w3c-apg-interfaccia-tastiera.md:123, :132). Con 20 regioni vuol dire un solo passo di Tab invece di venti [INF].
- **Tauri su macOS.** `WKPreferences.tabFocusesLinks` decide se il Tab porta il fuoco su link e controlli dei moduli; Opzione inverte la preferenza [V] (fonti/apple-wkpreferences-tabfocuseslinks.md:9, :19). wry 0.57.0, la versione nel `Cargo.lock` del repo (`Cargo.lock:4228-4229`), la imposta a vero [V] (fonti/wry-0-57-0-wkwebview-mod-rs.md:522-524). Il Tab quindi raggiunge link e controlli come in Chrome; per gli elementi con `tabindex` non c'è una fonte che ne parli esplicitamente [INF], va provato nel prototipo.
- **Dimensione del bersaglio (WCAG 2.2, 2.5.8, livello AA).** Serve un quadrato di 24×24 px CSS interamente dentro il bersaglio, a qualunque zoom [V] (fonti/w3c-wcag22-dimensione-bersaglio.md:71, :89). L'eccezione per spaziatura non aiuta: le regioni confinano fra loro [V] (:23). Valgono due eccezioni. «Equivalente»: un altro controllo conforme fa la stessa cosa [V] (:64). «Essenziale»: nelle mappe la posizione è l'informazione, ma W3C raccomanda comunque un'alternativa [V] (:67).
- Misurato: Liguria (7,5 px), Valle d'Aosta (9,2) e Molise (9,8) hanno meno di 12 px di raggio libero, quindi nessun quadrato di 24 px ci entra: a 500 px di larghezza falliscono il criterio della dimensione [M + INF geometrica].
- **Fuoco visibile.** L'indicatore deve avere un contrasto di 3:1 con i colori adiacenti [V] (fonti/w3c-wcag22-contrasto-non-testuale.md:113). In SVG gli elementi si dipingono nell'ordine del documento, e non esiste uno z-index [V] (fonti/w3c-svg2-ordine-di-rendering.md:117): il contorno di una regione col fuoco finisce sotto le vicine dipinte dopo.

### Proposta [INF]
- Un `<g role="radiogroup" aria-label="Regioni">` con una regione per `<path>` (`role="radio"`, `aria-checked`, `tabindex` mobile); frecce nell'ordine dei codici ISTAT (da nord a sud); Invio/Spazio per scegliere. Lo stesso gruppo serve alla scheda al clic (la regione «scelta» è quella aperta) e al selettore (bersaglio, dirottamento, emergenza).
- Nome accessibile con valore e segni: «Calabria, disoccupazione 14,2%, classe 5 di 5, un cantiere, un'emergenza aperta». Così il colore non è l'unico veicolo dell'informazione (1.4.1) [V] (fonti/w3c-wcag22-uso-del-colore.md:19).
- Anello del fuoco: un `<use href="#…">` in fondo all'SVG ricopia la regione col fuoco sopra tutte le altre, con doppio tratto chiaro e scuro per contrastare con tutte e cinque le classi.
- Accanto alla mappa, la lista delle 20 regioni col valore del livello, ordinabile: fa da controllo equivalente per 2.5.8, da tabella dei dati e da via d'uscita per le tre regioni piccole. È l'idea «mappa e tabella come due viste dello stesso dato» già in [[data-viz-cinematica]].
- SVG in linea nel componente Svelte, non `<img>`: solo così ogni regione è un elemento del DOM.

## Classi di un inchiostro leggibili dai daltonici

- La regola è variare la **luminosità**: con colori che differiscono per luminosità, chi è daltonico li distingue («get it right in black & white») [V] (fonti/datawrapper-daltonismo-2.md:63-65).
- ColorBrewer: tutte e 18 le scale sequenziali hanno `blind:[1]`, cioè «adatta ai daltonici» per ogni numero di classi; fra le divergenti molte hanno 0 sopra le 4-5 classi [M] (`grep "'blind'"` su fonti/colorbrewer-schemi.md:38-56; il significato di 1 = «ok» in fonti/colorbrewer-controllo-daltonismo.md:211, :246-248). Una scala sequenziale a una tinta è sicura per costruzione, se i passi di luminosità sono abbastanza larghi [INF].
- Misura sulle rampe del prototipo e su due ColorBrewer a 5 classi: L* CIELAB per classe, passo minimo, contrasto WCAG fra classi adiacenti, ΔE76 minimo fra adiacenti sotto simulazione Machado 2009 a severità 1.0 (protanopia, deuteranopia, tritanopia; matrici in fonti/machado-2009-simulazione-cvd.md:246-260, applicate in RGB lineare) [M] (script Python nello scratchpad):

| Rampa | L* | ΔL* min | contrasto min | ΔE protan / deutan / tritan |
| --- | --- | ---: | ---: | --- |
| `S_RAMP` (prototipo, a mano) | 94 83 69 42 32 | 10 | 1,31 | 11 / 11 / 10 |
| `M_RAMP` (prototipo, a mano) | 88 74 61 47 29 | 13 | 1,46 | 13 / 14 / 15 |
| `rampa('#FFF8EC','#1E3F7A')` | 94 79 64 48 32 | 15 | 1,50 | 17 / 18 / 16 |
| `rampa('#EFE6D2','#1F2A44')` | 88 72 57 40 22 | 15 | 1,55 | 16 / 16 / 15 |
| ColorBrewer Blues 5 | 96 85 68 52 35 | 11 | 1,35 | 11 / 13 / 15 |
| ColorBrewer Greys 5 | 97 82 62 42 15 | 15 | 1,50 | 15 / 15 / 15 |

- Lettura [INF]: la `rampa()` del prototipo, interpolazione lineare in RGB fra carta e inchiostro a t = 0,06…0,94, dà già passi quasi uguali (15-16 L*) e regge alla simulazione meglio di Blues, che ColorBrewer segna come sicura. `S_RAMP`, scritta a mano, ha un salto di 27 fra la terza e la quarta classe e solo 10 fra le ultime due: le due classi più scure si confondono prima delle altre. Regola pratica: cinque classi a passi uguali di L*, almeno 15 l'uno, la più chiara distinta dalla carta.
- Il contrasto 3:1 fra classi adiacenti non è raggiungibile con cinque classi: servirebbe 3⁴ = 81:1, oltre il massimo di 21:1 [INF, aritmetica]. WCAG 1.4.11 esenta i gradienti che rappresentano una misura, come le heat map, e la grafica i cui valori sono presentati anche in altro modo [V] (fonti/w3c-wcag22-contrasto-non-testuale.md:277, :447). Il confine fra regioni (tratto d'inchiostro) e la lista dei valori coprono il resto [INF].
- Tono grave: la rampa grigia segue la stessa regola dei passi [INF].
- Livello «risultato del voto»: unica eccezione all'inchiostro unico (ticket 27). Le tinte di partito vanno scelte con luminosità diverse, come da [[data-viz-cinematica]], e il nome del partito va nel nome accessibile della regione [INF].

## Per il nostro gioco

- **Raccomandazione (non decisione):** ISTAT generalizzato 2026, un comando mapshaper (`-filter-islands min-area=16km2 -simplify interval=1000 keep-shapes`, precisione 0,5) → circa 38 KB, generato una volta e messo nel repo con la riga di attribuzione «Confini: ISTAT, unità amministrative a fini statistici al 1° gennaio 2026, versione generalizzata, CC BY 4.0; semplificati con mapshaper». Ancore dei segni da `-points inner`, con spostamento a mano per Liguria, Valle d'Aosta e Molise. Regioni come `radiogroup` con tabindex mobile più la lista equivalente. Classi a passi uguali di L* (≥ 15).
- **Evitiamo:** semplificazione in percentuale (cancella le isole); centroidi come ancore (Liguria in mare); 20 fermate di Tab; `S_RAMP` così com'è; Natural Earth per le regioni italiane.
- **Aperto:**
  - quanto è grande la mappa nell'ossatura (ticket 07). Le misure valgono a 500 px di larghezza: più grande, le tre regioni piccole migliorano; sotto i 500 px, `interval` va alzato;
  - dove vive lo script di generazione (forse accanto a `strumenti/taratura/`, che già produce `dati/base/regioni.toml`);
  - verificare nel prototipo che con WKWebView il Tab entri negli elementi SVG con `tabindex`.

## Riferimenti

- **URL** https://www.istat.it/notizia/confini-delle-unita-amministrative-a-fini-statistici-al-1-gennaio-2018-2/ · [V]
  **perché conta qui** fonte primaria dei confini regionali, edizione 1° gennaio 2026
  **cosa se ne prende** versione generalizzata, shapefile WGS84 UTM 32N, URL dello zip 2026, avviso di possibili revisioni
- **URL** https://www.istat.it/note-legali/ · [V] (fonte già catturata, `fonti/istat-note-legali.md`)
  **perché conta qui** licenza della geometria che distribuiamo
  **cosa se ne prende** CC BY 4.0: menzione, link alla licenza, indicazione delle modifiche, niente restrizioni aggiuntive
- **URL** https://www.istat.it/storage/cartografia/confini_amministrativi/generalizzati/2026/Limiti01012026_g.zip · [M]
  **perché conta qui** il dato su cui sono fatte tutte le misure
  **cosa se ne prende** 20 record, 426 parti, campi `COD_REG`/`DEN_REG`, sha256 `b011a590656c3a3ebc297fba80726a376aa843b6f164641cf6a4a990021a81d6`
- **URL** https://www.naturalearthdata.com/downloads/10m-cultural-vectors/10m-admin-1-states-provinces/ · [V]
  **perché conta qui** l'alternativa in pubblico dominio
  **cosa se ne prende** admin-1 è il dataset più difficile da tenere aggiornato; per l'Italia 110 province da dissolvere, misurato [M]
- **URL** https://github.com/guglielmo/geojson-italy · [V]
  **perché conta qui** fonte del prototipo, derivata da ISTAT
  **cosa se ne prende** stessa licenza CC BY 4.0, annata 2026, attribuzione ISTAT da mantenere; di seconda mano rispetto allo zip
- **URL** https://mapshaper.org/docs/reference.html · [V]
  **perché conta qui** strumento di semplificazione e di esportazione SVG
  **cosa se ne prende** `-filter-islands`, `-simplify interval=`/`keep-shapes`, `-points inner`, `-innerlines`, `id-field=`/`svg-data=`
- **URL** https://github.com/mbloch/mapshaper · [M]
  **perché conta qui** licenza e freschezza dello strumento
  **cosa se ne prende** MPL 2.0, v0.7.101 del 29/10/2026, solo dipendenza di sviluppo
- **URL** https://github.com/axismaps/colorbrewer/blob/master/colorbrewer_schemes.js · [V]
  **perché conta qui** riferimento delle scale sicure per i daltonici
  **cosa se ne prende** tutte le sequenziali con `blind:[1]`; Blues e Greys a 5 classi come termine di paragone
- **URL** https://github.com/axismaps/colorbrewer/blob/master/colorbrewer.js · [V]
  **perché conta qui** significato del flag `blind`
  **cosa se ne prende** 1 = «ok», 2 = «forse», 0 = «no»; un valore solo vale per ogni numero di classi
- **URL** https://www.datawrapper.de/blog/colorblindness-part2 · [V]
  **perché conta qui** regola operativa per scegliere i colori
  **cosa se ne prende** variare la luminosità, «get it right in black & white»
- **URL** https://www.inf.ufrgs.br/~oliveira/pubs_files/CVD_Simulation/CVD_Simulation.html · [V]
  **perché conta qui** modello di simulazione usato per misurare le rampe
  **cosa se ne prende** matrici Machado 2009 a severità 1.0 per protanopia, deuteranopia, tritanopia
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/target-size-minimum.html · [V]
  **perché conta qui** misura dei bersagli cliccabili
  **cosa se ne prende** quadrato di 24×24 px, eccezioni «equivalente» ed «essenziale» per le mappe; tre regioni sotto soglia a 500 px
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html · [V]
  **perché conta qui** contrasto delle classi e dell'anello del fuoco
  **cosa se ne prende** 3:1 per l'indicatore del fuoco; esenzione per gradienti che rappresentano una misura e per valori presentati altrove
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/use-of-color.html · [V]
  **perché conta qui** il colore della choropleth non può essere l'unico canale
  **cosa se ne prende** il valore va anche nel nome accessibile e nella lista
- **URL** https://www.w3.org/WAI/ARIA/apg/patterns/radio/ · [V]
  **perché conta qui** schema ARIA del selettore di regione
  **cosa se ne prende** `radiogroup`/`radio`, `aria-checked`, etichette
- **URL** https://www.w3.org/WAI/ARIA/apg/practices/keyboard-interface/ · [V]
  **perché conta qui** come tenere 20 regioni in una sola fermata di Tab
  **cosa se ne prende** tabindex mobile (`0` sull'attivo, `-1` sugli altri), scroll automatico in vista
- **URL** https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/tabindex · [V]
  **perché conta qui** focalizzabilità degli elementi SVG
  **cosa se ne prende** `tabindex` su qualunque elemento SVG, supportato ovunque da gennaio 2017
- **URL** https://developer.apple.com/documentation/webkit/wkpreferences/tabfocuseslinks · [V]
  **perché conta qui** comportamento del Tab nel webview di Tauri su macOS
  **cosa se ne prende** la preferenza governa link e controlli; Opzione la inverte
- **URL** https://raw.githubusercontent.com/tauri-apps/wry/wry-v0.57.0/src/wkwebview/mod.rs · [V] (fonte catturata da un'altra ricerca, `fonti/wry-0-57-0-wkwebview-mod-rs.md`)
  **perché conta qui** è la versione di wry nel `Cargo.lock` del repo
  **cosa se ne prende** wry imposta `tabFocusesLinks` a vero: il Tab raggiunge link e controlli
- **URL** https://www.w3.org/TR/SVG2/render.html · [V]
  **perché conta qui** disegno dell'anello del fuoco sopra le regioni vicine
  **cosa se ne prende** si dipinge nell'ordine del documento, niente z-index: serve una copia in coda

## Collegamenti

- [[mappe-e-globo]]: Natural Earth, proiezioni equivalenti, regole delle choropleth; qui le si applica all'Italia.
- [[data-viz-cinematica]]: famiglie di scale e daltonismo; mappa e tabella come due viste dello stesso dato.
- [[tauri]]: WKWebView su macOS, cioè dove gira la mappa.
- [[disposizioni-territoriali-al-turno-zero]]: perimetri da evidenziare sulla mappa nell'anteprima.
- [[decisioni]]: ticket 27, livelli, segni, scheda e selettore.
