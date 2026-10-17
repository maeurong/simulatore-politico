---
tipo: estetica
fonti: [fonti/naturalearth-termini-uso.md, fonti/naturalearth-admin0-paesi.md, fonti/naturalearth-politica-confini-contesi.md, fonti/osm-copyright-odbl.md, fonti/protomaps-pmtiles-docs.md, fonti/maplibre-globo-esempio.md, fonti/d3-geo-proiezioni.md, fonti/redblob-planet-generation.md, fonti/datawrapper-scale-colore.md, fonti/defcon-escapist-everybody-dies.md]
aggiornato: 2026-10-16
---
# Mappe e globo

La mappa del mondo è la superficie principale del gioco: choropleth per i dati, confini per la politica, rotte per commercio e militare. La base cartografica giusta esiste ed è libera: Natural Earth, pubblico dominio, confini de facto con varianti per punto di vista. Lo stile (piatto, globo, low-poly) è una scelta artistica che costa poco in tutti i casi, a patto di disegnare poligoni semplificati e non tile fotografiche.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Dati cartografici aperti

### Natural Earth: la base
- Tutte le versioni raster e vettoriali sono di pubblico dominio: uso libero anche commerciale, modifica di contenuto e stile, nessun permesso e nessun credito obbligatorio [V] (fonti/naturalearth-termini-uso.md:12-14).
- I nomi dei luoghi in più di 20 lingue vengono da Wikidata, CC0 [V] (fonti/naturalearth-termini-uso.md:66). Utile per la localizzazione delle etichette.
- Admin-0: 258 paesi (Groenlandia separata dalla Danimarca); confini de facto per default, varianti "point of view" per diverse decine di paesi [V] (fonti/naturalearth-admin0-paesi.md:14-16). "Fatevi il vostro mashup delle aree contese secondo la vostra prospettiva politica" [V] (fonti/naturalearth-admin0-paesi.md:32).
- Politica sui confini: de facto dal 2009; dalla versione 5 (dicembre 2021) supporto ufficiale a più punti di vista [V] (fonti/naturalearth-politica-confini-contesi.md:12-18). 512 codici admin-0, di cui 103 territori contesi [V] (fonti/naturalearth-politica-confini-contesi.md:26).
- **Staleness:** l'ultima release del repository `nvkelso/natural-earth-vector` è la v5.1.2 del 13 maggio 2022 [M] (`gh api repos/nvkelso/natural-earth-vector/releases/latest`, 16/10/2026). I confini cambiati dopo quella data vanno corretti a mano.
- Per un gioco con nazioni reali la scelta del punto di vista è anche una questione legale e di mercato: vedi le pagine in `legale/` di altri agenti.

### OpenStreetMap: no per la mappa globale
- ODbL: si può usare e adattare citando OSM, ma un database derivato si distribuisce solo con la stessa licenza [V] (fonti/osm-copyright-odbl.md:17-19). Per una mappa mondiale stilizzata Natural Earth basta e non ha vincoli [INF].

### Vector tiles offline
- PMTiles è un archivio a file singolo per piramidi di tile, letto con richieste HTTP Range [V] (fonti/protomaps-pmtiles-docs.md:11-16). MBTiles, basato su SQLite, è pensato per l'accesso su disco [V] (fonti/protomaps-pmtiles-docs.md:30). La specifica PMTiles è pubblico dominio/CC0, le implementazioni di riferimento BSD-3 [M] (`gh api repos/protomaps/PMTiles/contents/LICENSE`).
- Per un gioco offline le tile servono solo se si vuole zoom continuo dal mondo alla città. Con zoom a livelli fissi bastano poligoni semplificati caricati una volta [INF]. La scelta del formato tocca lo stack: vedi [[godot]] e [[tauri]].

## Proiezioni e globo

- d3-geo include, fra le altre, `equalEarth`, `naturalEarth1`, `orthographic`, `conicEqualArea` [M] (`gh api repos/d3/d3-geo/contents/src/projection`); licenza ISC (testo in `LICENSE`) [M].
- Per una choropleth serve una proiezione equivalente, che conservi le aree: con Mercator Russia e Canada pesano visivamente più di quanto valgano [INF]. Equal Earth è nata per questo; il sito non si è lasciato catturare (403) [NON TROVATO]. URL: https://equal-earth.com/
- Globo: MapLibre GL JS (BSD-3) [M] passa a globo con `map.setProjection({type: 'globe'})`; l'esempio ufficiale usa la v6.14.0 [V] (fonti/maplibre-globo-esempio.md:29). È un'opzione solo se lo stack è web ([[tauri]]), dove su Linux e Deck WebGL può rallentare senza errori: proprio la mappa è la parte più esposta [V, pagina di altro agente].
- Globo low-poly stilizzato: Red Blob Games genera mappe su sfera con punti, triangolazione di Delaunay e regioni di Voronoi; i punti perfettamente uniformi si sono rivelati inutili, serve un jitter casuale perché il risultato sia interessante [V] (fonti/redblob-planet-generation.md:14-37). Applicato a dati reali: si proiettano i poligoni Natural Earth su una sfera di celle e si colora per cella, con un effetto da "plastico" a costo di poche migliaia di triangoli [INF].

## Choropleth: regole

- La geografia codifica già la posizione, quindi il colore va tutto al valore [V] (fonti/datawrapper-scale-colore.md:40).
- Scala sequenziale per grandezze, divergente per scostamenti da un centro (es. crescita del PIL), categorica solo per variabili senza ordine (alleanze, partito al governo) [V] (fonti/datawrapper-scale-colore.md:52-95).
- Classi poche e stabili fra un turno e l'altro, così il cambio di colore significa davvero un cambio di valore [INF].
- Le nazioni piccole spariscono in una choropleth mondiale: servono simboli proporzionali o un riquadro di dettaglio [INF].

## Stili possibili e costo [INF]

| Stile | Riferimento | Costo GPU | Rischio |
| --- | --- | --- | --- |
| Piatto vettoriale, contorni luminosi | DEFCON (fonti/defcon-escapist-everybody-dies.md:45) | minimo: linee e un bagliore | cliché da sala comando |
| Mappa di carta, retini e inchiostro | CK3 a zoom massimo [NON TROVATO] | minimo: 2D, una texture di carta | sembrare una pagina web |
| Globo a celle low-poly | Red Blob (fonti/redblob-planet-generation.md) | basso: poche migliaia di triangoli, flat shading | perdere leggibilità dei confini |
| Rilievo 3D con terreno | Victoria 3, HOI | alto: heightmap, texture, ombre | fuori dall'obiettivo "leggero" |

## Per il nostro gioco

- **Prendiamo:** Natural Earth come base (pubblico dominio, nomi multilingua, varianti di confine); proiezione equivalente per la vista dati; un solo stile di mappa coerente con la direzione artistica.
- **Evitiamo:** OSM per la mappa globale (share-alike); Mercator per le choropleth; terreno 3D dettagliato.
- **Aperto:** punto di vista sui confini contesi (legale); vista piatta, globo o entrambe; tile o poligoni (tecnica).

## Riferimenti

- **URL** https://www.naturalearthdata.com/about/terms-of-use/ · [V]
  **perché conta qui** licenza della base cartografica
  **cosa se ne prende** pubblico dominio, nessun credito obbligatorio, nomi Wikidata CC0
- **URL** https://www.naturalearthdata.com/downloads/10m-cultural-vectors/10m-admin-0-countries/ · [V]
  **perché conta qui** il layer dei paesi che useremmo
  **cosa se ne prende** 258 paesi, de facto, varianti per punto di vista
- **URL** https://www.naturalearthdata.com/about/disputed-boundaries-policy/ · [V]
  **perché conta qui** politica editoriale sui confini contesi
  **cosa se ne prende** de facto di default, POV dalla v5, 103 territori contesi codificati
- **URL** https://www.openstreetmap.org/copyright · [V]
  **perché conta qui** licenza dell'alternativa più nota
  **cosa se ne prende** ODbL share-alike: da evitare per la mappa base
- **URL** https://docs.protomaps.com/pmtiles/ · [V]
  **perché conta qui** formato di tile a file singolo
  **cosa se ne prende** PMTiles vs MBTiles, se servisse lo zoom continuo
- **URL** https://maplibre.org/maplibre-gl-js/docs/examples/display-a-globe-with-a-vector-map/ · [V]
  **perché conta qui** globo vettoriale pronto, se lo stack è web
  **cosa se ne prende** una riga per passare da piatto a globo
- **URL** https://d3js.org/d3-geo/projection · [V]
  **perché conta qui** libreria di proiezioni di riferimento
  **cosa se ne prende** elenco delle proiezioni equivalenti disponibili
- **URL** https://www.redblobgames.com/x/1843-planet-generation/ · [V]
  **perché conta qui** tecnica per un globo a celle
  **cosa se ne prende** Delaunay/Voronoi su sfera, jitter dei punti
- **URL** https://equal-earth.com/ · [NON TROVATO]
  **perché conta qui** proiezione equivalente pensata per le mappe del mondo
  **cosa se ne prende** da recuperare: motivazioni della proiezione (sito 403)

## Collegamenti

- [[data-viz-cinematica]]: scale di colore e animazione delle transizioni.
- [[shader-ed-effetti-economici]]: contorni e bagliore della mappa.
- [[tipografia-e-font-aperti]]: etichette multilingua sulla mappa.
- [[direzioni-artistiche-candidate]]: quale stile di mappa per quale direzione.
- [[prestazioni-su-hardware-modesto]], [[godot]], [[tauri]]: hardware bersaglio e rischio WebGL (altro agente).
