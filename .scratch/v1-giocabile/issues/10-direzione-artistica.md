Type: prototype
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

La direzione B+D (carta stampata + diagramma di sistema, docs/ricerca/sintesi/direzioni-artistiche-candidate.md:71) regge su una schermata vera del gioco? Prototipo usa e getta di una schermata (cruscotto del turno o mappa delle regioni) da giudicare a occhio.

## Verdetto parziale (17/10/2026)

- Direzione: **B · Dossier con rete** (B+D), prototipo `prototipi/10-direzione-artistica.prototype.html`.
- Da rubare: da A l'esito del turno come prima pagina di giornale, in una schermata propria prima del cruscotto, e il listino dell'agenda (tutti e 10 i ministri in una tabella densa); da C lo schema dello Stato come vista secondaria (chi tocca cosa nel turno).
- Mappa: choropleth a tinte piatte, cinque classi di un inchiostro; il retino resta per ritratti e illustrazioni.
- **Ancora aperto, il tono.** Mario: «così è troppo serio, rimane comunque un gioco»; deve restare semplice e comprensibile. Giro di ricerca su riferimenti giocosi, poi secondo prototipo con le scelte qui sopra e gli stili nuovi. Il ticket si chiude sul verdetto di quel prototipo.

## Answer

Verdetto sul secondo prototipo (`prototipi/10-direzione-artistica-2.prototype.html`, 17/10/2026), quattro stili giocosi sulle scelte del verdetto parziale:

- **Stile: S · Il Settimanale.** Il dossier con rete diventa un settimanale satirico inventato (testata «Il Pungolo»): occhiello serio col fatto e il numero, titolo spiritoso sotto; vignetta; note a biro sulla rete e sullo schema dello Stato. Fraunces per i titoli, Atkinson Next per il testo (docs/ricerca/estetica/tono-giocoso-su-temi-seri.md:79).
- **Da rubare:**
  - da S stesso, il borsino dei ministri: caricatura, voto con freccia e motto in corsivo;
  - da F, Ferma e Dirotta girano la carta del ministro sul retro;
  - da P, i tre indicatori sintetici come tracciati a caselle, con blocchetti che avanzano a scatti;
  - da M, l'anteprima della misura come un mobile appeso, che oscilla smorzato solo a un gesto del giocatore e mai in loop.
- **Tono grave:** la cornice giocosa tace. Spariscono testata, nome spiritoso, battute, motti e illustrazioni; resta una pagina neutra col fatto, solo inchiostro, senza rimbalzi né oscillazioni.
- Restano le regole della ricerca: lo scherzo sta nei testi e nei personaggi, mai nei numeri; i ministri sono macchiette di tipi sociali, mai politici reali; velocità 1×/2×/istantaneo e interruttore del movimento nel gioco.
- **Aperto, per la tecnica:** la regola dei 9 px sul Deck. Misurando il corpo, il prototipo sta a 12 px; misurando la x-height servirebbero circa 17 px.
