---
tipo: estetica
fonti: [fonti/robertson-2008-animation-trend.md, fonti/datawrapper-scale-colore.md, fonti/datawrapper-daltonismo.md, fonti/pudding-scrollytelling-responsive.md, fonti/owid-faq-licenza.md, fonti/owid-grapher-github.md]
aggiornato: 2026-10-16
---
# Data-viz cinematica

Un simulatore di governo mostra serie storiche, distribuzioni e mappe a ogni turno. Il giornalismo dei dati (NYT, The Pudding, Our World in Data, Datawrapper) ha già risolto come renderle belle e leggibili. Il punto chiave, misurato: l'animazione dei grafici piace e si capisce in fretta, ma fa sbagliare. Va usata per raccontare il passaggio da uno stato all'altro, non per farci analizzare i dati.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Animare o no

- Studio Microsoft Research (Robertson et al., 2008) sul modello Gapminder Trendalyzer: l'animazione è la tecnica più veloce in presentazione e i partecipanti la trovano "piacevole ed emozionante", ma porta a molti errori. Per l'analisi è la forma meno efficace: le tracce statiche sono più veloci e gli small multiples più accurati [V] (fonti/robertson-2008-animation-trend.md:13).
- Il lavoro di Heer e Robertson (2007) sulle transizioni animate fra grafici statistici è il riferimento per il "come" animare, ma la pagina non si è lasciata catturare (404 e fetch fallito) [NON TROVATO]. URL: https://vis.stanford.edu/papers/animated-transitions

**Regola che ne viene [INF]:** a fine turno si anima la *transizione* (la barra che cresce, la regione che cambia colore, una sola volta, con un'origine chiara); per confrontare nel tempo si dà una vista statica (tracce, small multiples) che il giocatore apre quando vuole.

## Colore

- Tre famiglie: categoriche (tinte diverse per categorie senza ordine), sequenziali (da chiaro a scuro per valori da basso ad alto), divergenti (due direzioni da un centro) [V] (fonti/datawrapper-scale-colore.md:52-95).
- In una choropleth la posizione è già codificata dalla geografia, quindi il colore va interamente al valore [V] (fonti/datawrapper-scale-colore.md:40).
- Tinte categoriche con luminosità diverse, così reggono anche in scala di grigi e per chi è daltonico [V] (fonti/datawrapper-scale-colore.md:66).
- Evidenziare una categoria e attenuare il resto vale in ogni tipo di scala [V] (fonti/datawrapper-scale-colore.md:111-113). Esempi citati: NYT (scala sequenziale a classi), The Guardian, Bloomberg [V] (fonti/datawrapper-scale-colore.md:77-79, :115-123).
- Daltonismo rosso/verde: 1 uomo su 12 e 1 donna su 200 di origine nordeuropea; su 500 persone, 24 potrebbero non leggere il grafico [V] (fonti/datawrapper-daltonismo.md:80). Il rosso/verde "buono/cattivo" è proprio la coppia da evitare in un gioco politico [INF].

## Ridurre all'essenziale

- The Pudding: progettare prima per lo schermo piccolo "costringe a ridurre l'esperienza all'osso, lasciando solo il necessario" [V] (fonti/pudding-scrollytelling-responsive.md:17). Vale anche per noi come esercizio: un pannello deve funzionare in una finestra piccola prima di riempire un 4K [INF].
- Lo scrollytelling (racconto a passi che fa avanzare il grafico) è la forma naturale del rapporto di fine turno: un passo per notizia, il grafico cambia sotto [INF].

## Our World in Data: cosa si può prendere

- I grafici OWID con logo e bollino CC BY si possono usare, riprodurre e distribuire citando la fonte [V] (fonti/owid-faq-licenza.md:54). I dati di terzi restano sotto le licenze dei fornitori (OMS, ONU, Banca Mondiale) [V] (fonti/owid-faq-licenza.md:76). I dati prodotti da OWID sono CC BY [V] (fonti/owid-faq-licenza.md:78).
- **Attenzione, cambiamento recente:** il codice del Grapher OWID "non è più disponibile con licenza MIT". Si può leggere e studiare, non copiare, modificare né distribuire [V] (fonti/owid-grapher-github.md:19). Chi lo ricorda come open source MIT ha un'informazione superata. Si riprende l'idea (grafico, mappa e tabella come tre viste dello stesso dato), non il codice.
- Il New York Times non ha una pagina di metodo catturata in questa ricerca; i suoi esempi arrivano tramite Datawrapper [NON TROVATO].

## Per il nostro gioco

- **Prendiamo:** animazione solo per le transizioni di stato; viste statiche per il confronto; scale di colore per tipo di dato; palette provata per il daltonismo; evidenziare/attenuare come gesto principale; il rapporto di fine turno a passi.
- **Evitiamo:** grafici che si animano in continuo (rumore, consumo, errori); rosso/verde come unica codifica; arcobaleni su dati ordinati; il cruscotto con dodici mini-grafici dello stesso peso, che è la "AI slop" da dashboard.
- **Aperto:** quanti tipi di grafico tenere. Meno sono, più il giocatore impara a leggerli [INF]. Per i dati da mostrare vedi le pagine in `dati/` di altri agenti.

## Riferimenti

- **URL** https://www.microsoft.com/en-us/research/publication/effectiveness-of-animation-in-trend-visualization/ · [V]
  **perché conta qui** misura empirica su animazione vs grafici statici
  **cosa se ne prende** animare per raccontare, non per analizzare
- **URL** https://vis.stanford.edu/papers/animated-transitions · [NON TROVATO]
  **perché conta qui** il riferimento classico su come animare le transizioni fra grafici
  **cosa se ne prende** da recuperare: principi di staging e congruenza delle transizioni
- **URL** https://blog.datawrapper.de/which-color-scale-to-use-in-data-vis/ · [V]
  **perché conta qui** guida pratica di chi fa grafici per le redazioni
  **cosa se ne prende** tipi di scala, choropleth, evidenziare/attenuare, luminosità diverse
- **URL** https://blog.datawrapper.de/colorblindness-part1/ · [V]
  **perché conta qui** numeri sul daltonismo e coppie da evitare
  **cosa se ne prende** 1 su 12 uomini; niente rosso/verde come codifica unica
- **URL** https://pudding.cool/process/responsive-scrollytelling/ · [V]
  **perché conta qui** metodo di The Pudding per racconti visivi a passi
  **cosa se ne prende** partire dal piccolo per tagliare; il racconto a passi
- **URL** https://ourworldindata.org/faqs · [V]
  **perché conta qui** licenza di grafici e dati OWID
  **cosa se ne prende** grafici e dati OWID CC BY con citazione; dati di terzi sotto le loro licenze
- **URL** https://github.com/owid/owid-grapher · [V]
  **perché conta qui** licenza attuale del codice
  **cosa se ne prende** il codice non è più MIT: si studia, non si copia

## Collegamenti

- [[mappe-e-globo]]: la choropleth nel dettaglio.
- [[movimento-e-feedback]]: tempi e ritmo delle transizioni.
- [[interfacce-di-giochi-gestionali]]: dove vivono i grafici.
- [[tipografia-e-font-aperti]]: cifre tabellari nei grafici.
