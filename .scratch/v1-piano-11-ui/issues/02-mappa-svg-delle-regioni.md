Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Da quale fonte, con quale licenza e con quale semplificazione si ricava l'SVG delle 20 regioni per la mappa (storie 137-141)? Confini ISTAT generalizzati, Natural Earth (già in `docs/ricerca/estetica/mappe-e-globo.md`) o altro; peso del file a una risoluzione leggibile a 1280×800; isole minori; posto per i segni fissi di atti, cantieri ed emergenze; selettore accessibile da tastiera (una regione è un bersaglio cliccabile e focalizzabile); classi di un inchiostro leggibili dai daltonici.

## Answer

Pagina: [mappa-svg-delle-regioni](../../../docs/ricerca/estetica/mappa-svg-delle-regioni.md). Raccomandazione, da confermare nel prototipo della mappa:

- Fonte: confini ISTAT generalizzati al 1° gennaio 2026, CC BY 4.0, con le 20 regioni già pronte (`COD_REG`). Natural Earth per l'Italia ha 110 province, con le province sarde superate.
- Un comando mapshaper (`-filter-islands min-area=16km2 -simplify interval=1000 keep-shapes`, precisione 0,5): 38 KB (12 KB compressi) a 500 px di larghezza. L'SVG si genera una volta e va nel repo con l'attribuzione CC BY; join `COD_REG → id` di 20 righe (`dati/base/regioni.toml`). Una semplificazione in percentuale cancella Elba, Eolie e Lampedusa. I fili chiari fra regioni si coprono con `-innerlines`.
- Segni fissi sul punto interno (`-points inner`), non sul centroide, che per la Liguria cade in mare. Liguria, Valle d'Aosta e Molise hanno meno di 10 px di raggio libero: spostamento a mano o bollino unico.
- Selettore: `radiogroup` con tabindex mobile (una sola fermata di Tab), valore e segni nel nome accessibile, anello del fuoco ricopiato in coda con `<use>`, più la lista delle 20 regioni accanto (eccezione «equivalente» di WCAG 2.5.8 per le tre regioni sotto i 24 px).
- Colori: cinque classi a passi uguali di luminosità (L* ≥ 15), sul modello di `rampa()` del prototipo; `S_RAMP` va rifatta perché confonde le due classi più scure.
- Da provare nel prototipo: il Tab dentro gli elementi SVG in WKWebView. Le misure in pixel valgono a 500 px; la dimensione vera la decide [Ossatura e navigazione](07-ossatura-e-navigazione.md).
