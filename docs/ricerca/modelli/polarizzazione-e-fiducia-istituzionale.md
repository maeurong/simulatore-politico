---
tipo: modello
fonti: [fonti/cses-indice-polarizzazione.md, fonti/castellano-2009-fisica-dinamiche-sociali.md, fonti/frontiers-torcal-christmann-2021-fiducia.md, fonti/vdem-dataset.md, fonti/dataeuropa-eurobarometro-licenza.md]
aggiornato: 2026-10-16
---
# Polarizzazione e fiducia istituzionale

Due variabili di stato lente che condizionano tutto il resto. La polarizzazione ideologica misura quanto sono lontani i partiti (pesati per voto); quella affettiva quanto gli elettori detestano gli altri partiti. La fiducia nelle istituzioni dipende soprattutto da corruzione e capacità di risposta, meno dall'economia, e quando crolla risale lentamente. Per il gioco sono indicatori da mostrare e moltiplicatori da applicare.

## Misure

- **Polarizzazione ideologica (Dalton)**: dispersione dei partiti sull'asse sinistra-destra, con le posizioni medie percepite dagli elettori pesate per la quota di voto (fonti/cses-indice-polarizzazione.md) [V]. Formula: `PI = √( Σ_j v_j · ((x_j − x̄)/5)² )`, con x su scala 0-10 e v_j quota di voto; vale 0 se tutti i partiti coincidono e 10 se stanno tutti agli estremi [INF, formula da fonti secondarie; Dalton 2008, Comparative Political Studies 41: 899-920].
- **Polarizzazione affettiva**: differenza fra simpatia per il proprio partito e antipatia per gli altri, su scale piace/non piace; Reiljan (European Journal of Political Research 2020) propone un indice per sistemi multipartitici, Wagner (Electoral Studies 69, doi:10.1016/j.electstud.2020.102199) misure individuali su 51 paesi e 166 elezioni CSES [INF, abstract nei risultati di ricerca].
- **Fiducia istituzionale**: quota di chi dichiara fiducia in parlamento, governo, partiti, magistratura; domande standard di Eurobarometro ed ESS [INF].

## Dinamica

- **Polarizzazione dalle dinamiche d'opinione**: nei modelli a confidenza limitata la soglia ε governa il numero di gruppi: sopra 1/2 consenso, sotto circa `1/(2ε)` cluster (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V]. Ridurre ε nel tempo (media frammentati, crisi) produce polarizzazione senza scriverla [INF].
- **Fiducia**: Spagna 1997-2019: percezioni di corruzione e di scarsa capacità di risposta pesano molto, l'economia meno; la Grande Recessione fa crollare la fiducia in partiti e parlamento fra 2008 e 2012, ma la fiducia resta bassa anche dopo la recessione per gli scandali e la scarsa risposta; lo studio usa i campioni spagnoli di Eurobarometro e un panel del 2015 (fonti/frontiers-torcal-christmann-2021-fiducia.md) [V].
- Studi comparati su dati ESS ed EVS trovano che corruzione, sistema elettorale e passato di regime spiegano le differenze fra paesi, mentre l'economia conta poco una volta controllata la corruzione [INF, non catturati].

Forma minima che ne segue: `F(t+1) = F(t) + a·(F* − F(t))`, con un bersaglio `F* = f(corruzione, risposta, economia)` e una velocità `a` più bassa in salita che in discesa [INF, scelta di design coerente con l'isteresi spagnola].

## Evidenza

- La misura di Dalton è disponibile per i moduli CSES 1-4, calcolata da Dalton e "contribuita al pubblico dominio" (fonti/cses-indice-polarizzazione.md) [V].
- V-Dem v16 (marzo 2026) contiene 531 indicatori, fra cui misure stimate da esperti con un modello di misura bayesiano; i valori si confrontano solo dentro la stessa versione (fonti/vdem-dataset.md) [V]. La presenza di indicatori di polarizzazione specifici (es. v2cacamps) non è verificata in sessione [INF].
- I volumi dello Standard Eurobarometro sono pubblicati su data.europa.eu in CC BY 4.0 (fonti/dataeuropa-eurobarometro-licenza.md) [V]: serie di fiducia per paese utilizzabili per tarare F.

## Parametri tipici

| Parametro | Valore | Fonte |
|---|---|---|
| PI Dalton | 0-10 | [INF] |
| soglia di consenso ε (Deffuant) | 0,5 | (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V] |
| livelli di fiducia nel parlamento per paese | da Eurobarometro | (fonti/dataeuropa-eurobarometro-licenza.md) [V per la licenza; valori non estratti] |
| velocità di recupero della fiducia | [NON TROVATO] come parametro; solo evidenza qualitativa di lentezza | (fonti/frontiers-torcal-christmann-2021-fiducia.md) [V] |

## Costo computazionale

PI: `O(P)` per elezione. Polarizzazione affettiva per gruppo: `O(G·P)`. Fiducia: `O(1)` per istituzione a tick [INF].

## Cosa semplifichiamo

- Un solo asse per la polarizzazione ideologica (sinistra-destra) [INF].
- Polarizzazione affettiva come distanza ideologica più un termine di identità di gruppo, invece di simulare le emozioni [INF].
- Fiducia come scalare per istituzione e non per gruppo sociale nella prima versione [INF].

## Per il nostro gioco

- Calcolare PI a ogni elezione e mostrarlo: è un numero solo, ha letteratura dietro e si confronta con i valori reali CSES.
- Usare la fiducia come moltiplicatore: bassa fiducia riduce l'efficacia delle policy che chiedono collaborazione (fisco, sanità pubblica) e alza il voto ai partiti anti-sistema. Isteresi: crollo rapido con scandali, recupero lento.
- Corruzione e capacità di risposta come leve principali della fiducia, l'economia come leva secondaria: è il contrario dell'intuizione comune ed è ciò che dicono i dati.
- Aperto: modellare l'astensione come funzione della fiducia.

## Collegamenti

- [[opinion-dynamics]]: genera la polarizzazione.
- [[voto-spaziale]]: la varianza degli elettori entra nel coefficiente di convergenza.
- [[cses]], [[v-dem]], [[eurobarometro]], [[ess]]: dati per tarare.
- [[policy-e-effetti]]: dove applicare il moltiplicatore di fiducia.
- [[personaggi-fazioni-e-stabilita-interna]]: legittimità, coesione e disordini nei grand strategy.

## Riferimenti

- **URL** https://cses.org/data-download/download-data-documentation/party-system-polarization-index-for-cses-modules-1-4/ · [V]
  **perché conta qui** indice di polarizzazione di Dalton già calcolato, in pubblico dominio
  **cosa se ne prende** definizione dell'indice e valori reali per tarare
- **URL** https://www.frontiersin.org/journals/political-science/articles/10.3389/fpos.2021.676672/pdf · [V]
  **perché conta qui** serie lunga sulla fiducia in un paese europeo, con cause separate
  **cosa se ne prende** corruzione e risposta come leve principali, isteresi dopo la crisi
- **URL** https://arxiv.org/pdf/0710.3256 · [V]
  **perché conta qui** legame fra soglia di confidenza e numero di gruppi d'opinione
  **cosa se ne prende** ε come manopola della polarizzazione
- **URL** https://data.europa.eu/api/hub/search/datasets/s2255_92_3_std92_eng · [V]
  **perché conta qui** licenza dei volumi Eurobarometro con le serie di fiducia
  **cosa se ne prende** la possibilità di includere serie aggregate nel repo con attribuzione
