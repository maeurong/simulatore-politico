# Mappa: piano 11, UI «Il Settimanale»

Label: wayfinder:map

## Destination

Spec del piano 11 in `docs/superpowers/specs/`, divisa in fette ordinate sulla Vista: ogni fetta pronta per `superpowers:writing-plans` quando su main ci sono i suoi dati, la prima su ciò che la Vista porta già oggi. Copre tutte le schermate della [spec v1](../../docs/superpowers/specs/2026-10-21-v1-giocabile.md) (storie 1-166, decisioni UI alle righe 395-402); per quelle i cui dati non sono su main fissa composizione, posto nella navigazione e pezzi del sistema visivo, senza inventare campi della Vista (decisione del 01/11/2026).

## Notes

- Dominio: UI Svelte 5 + TypeScript sopra la Vista generata (`app/src/lib/tipi/`), dentro il guscio Tauri 2 (`app/src-tauri/src/lib.rs`). Glossario in `CONTEXT.md`: usare quei termini (esito del turno, registro, tono grave, indicatore sintetico, livello, anteprima…).
- Vincolo: `sim-core` e la Vista non si toccano se non serve. Un dato che la Vista non porta è un **rimando** al piano che lo produce (i piani 8b, 9 e 10 sono su main dal 10/11, a `8952a06`: i dati di [Dati senza piano](issues/11-dati-senza-piano.md) li portano il piano di core «esito e Vista completa» e quello del piano strutturale), mai stato nuovo aggiunto dalla UI. Il guscio Tauri sì: comandi passacarte verso metodi già pubblici della partita, senza logica né stato (decisione del 01/11/2026).
- Riferimento visivo vincolante: `.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html`; direzione decisa in [Direzione artistica](../v1-giocabile/issues/10-direzione-artistica.md). Mappa decisa in [Mappa delle 20 regioni](../v1-giocabile/issues/27-mappa-delle-20-regioni.md).
- Finestra: laptop ridimensionabile, progetto a ~1440×900, minimo ~1280×800; la scala della UI copre il resto.
- Prototipi: **uno per schermata**, HITL, Mario sceglie fra varianti. Usa e getta, HTML statico in `.scratch/v1-piano-11-ui/prototipi/`, nutriti con Viste vere (ticket [Viste campione](issues/06-viste-campione.md)). Le schermate sono le 18 di [Ossatura e navigazione](issues/07-ossatura-e-navigazione.md), ticket da 13 a 30, tutti dietro Viste campione e [Fondamenta visive](issues/08-fondamenta-visive.md). **Le proposte grafiche si mostrano, non si descrivono**: anche in un ticket di grilling, una scelta che riguarda cosa si vede va a Mario su un prototipo aperto nel browser (11/11/2026).
- Skill per ogni sessione: `grilling` + `domain-modeling`; `research` per i ticket research (subagente `researcher`, scrive in `docs/ricerca/` secondo `docs/ricerca/CLAUDE.md`, non su branch `research/*`); `prototype` per i prototipi, con `impeccable`/`dataviz` dove servono.
- Knowledge base: si parte da `docs/ricerca/index.md`, sezione estetica (data-viz, movimento, tipografia, mappe, grafica italiana giocosa, tono). Ogni decisione che vale la pena tenere va anche in `docs/ricerca/sintesi/decisioni.md`.
- Tracker: local-markdown in `.scratch/v1-piano-11-ui/`.
- Pianificazione, non esecuzione: la mappa produce decisioni; il codice lo scrivono i piani nati dalla spec.

## Decisions so far

<!-- una riga per ticket chiuso: gist + link -->

- [Inventario Vista per schermata](issues/01-inventario-vista-per-schermata.md): il guscio manca di `comanda` e di 10 passacarte (fondazione in testa); esito del turno e registro, «avanti fino a novità», 3 livelli regionali della mappa e PDE non li porta nessun piano; eventi al 9, partite/editor/riavvolgimento al 10; `bigint` non serializzabile. **Aggiornato il 10/11** con i piani 9 e 10 su main: `comanda`, fondazione, eventi, situazioni, acquisti, partite, editor, albo e riavvolgimento ci sono; restano 7 passacarte e, senza piano, anche lo stato del MES, la clausola dell'energia e due avvisi.
- [Dati senza piano](issues/11-dati-senza-piano.md): niente esce dalla v1 e niente si calcola nella UI. Un piano di core «esito e Vista completa», fuori dalla mappa e avviato subito, porta esito, registro, fermate e i due avvisi in una PR, le otto esposizioni in un'altra; il piano strutturale ha un piano suo, dopo. Dati regionali esatti con un turno di ritardo, gruppi come quota più elettori campione, cantieri finiti fino al voto successivo.
- [Testi esterni](issues/03-testi-esterni.md): JSON importato da Vite, un file per registro (neutro e Pungolo), funzione `t` tipizzata con `Intl` e varianti a seme; nessuna libreria; `useGrouping: "always"`; il seme non arriva ancora alla UI.
- [Caricature](issues/05-caricature.md): kit SVG nostro di circa 30 parti senza capelli, occhiali né barbe, combinate con un hash di ruolo, partito e nome; iniziali nel tono grave; DiceBear scartato. Il bacino dei nomi genera politici reali: difetto dei dati, fuori mappa.
- [Movimento e scala in WKWebView](issues/04-movimento-e-scala-in-wkwebview.md): CSS e Web Animations senza librerie, velocità come fattore `--k`, interruttore sopra `prefers-reduced-motion` (le transizioni Svelte ignorano il CSS); carta 3D, molla con `linear()`, caselle a `steps()`; scala con CSS `zoom`, `transform: scale` escluso.
- [Mappa SVG delle regioni](issues/02-mappa-svg-delle-regioni.md): ISTAT generalizzato 2026 (CC BY 4.0), un comando mapshaper per 38 KB a 500 px con le isole sopra 16 km²; segni sul punto interno; selettore `radiogroup` con la lista delle 20 regioni accanto; cinque classi a passi uguali di luminosità.
- [Ossatura e navigazione](issues/07-ossatura-e-navigazione.md): cornice fissa coi tre indicatori (anche sulla prima pagina, come striscia), sommario di dieci sezioni barrate col motivo quando non valgono (Prima pagina, Cruscotto, Governo, Aula, Conti, Bilancio, Paese, Partiti, Campagna, Registro), dettaglio a lato e conferme sul posto, mai modali; cruscotto corto con l'evento e le scelte aperte; tono grave a tutto il turno; edicola all'avvio; acquisti dove si leggono. Elenco definitivo: 18 schermate da prototipare, un ticket ciascuna.

## Not yet specified

- **Impostazioni del giocatore**: dove vivono scala (CSS `zoom`), velocità e movimento (file del guscio o memoria del webview) e se sopravvivono alla partita. Il posto nella navigazione c'è già: una voce accanto a Schema dello Stato ed Edicola, e nell'edicola ([Ossatura e navigazione](issues/07-ossatura-e-navigazione.md)).
- **Verifica della UI**: cosa controlla un piano del piano 11 oltre a `svelte-check` (test di componente, giro nel browser con la skill `run`, istantanee). Da decidere prima della spec, dopo le fondamenta.

## Out of scope

- Implementazione: la mappa si chiude con la spec, il codice lo scrivono i piani.
- Modifiche a `sim-core` e alla Vista: ogni dato mancante è un rimando al piano che lo produce.
- Suono e musica: la spec v1 non li chiede.
- Nomi del bacino che coincidono con politici reali («Matteo Ricci», «Andrea Romano», da `dati/base/elettorato.toml:11-12`): storia 159, ma è un difetto dei dati del core, non della UI ([Caricature](issues/05-caricature.md)).
- Traduzioni e piattaforme diverse dal Mac: la v1 è per Mario ([Direzione artistica](../v1-giocabile/issues/10-direzione-artistica.md), mappa v1 Out of scope).
