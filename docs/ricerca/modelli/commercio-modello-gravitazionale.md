---
tipo: modello
fonti: [fonti/piermartini-yotov-2016-structural-gravity.md, fonti/head-mayer-2013-gravity-cookbook.md, fonti/wto-advanced-guide-gravity.md, fonti/cepii-gravity.md, fonti/cepii-baci.md]
aggiornato: 2026-10-16
---
# Commercio: modello gravitazionale, con dazi e accordi

Il commercio fra due paesi cresce con le loro dimensioni economiche e cala con i costi che li separano: distanza, dazi, confini, lingua. La versione strutturale aggiunge le «resistenze multilaterali»: un dazio fra A e B sposta commercio anche verso C. Per il nostro gioco è il modo di far contare dazi, sanzioni e accordi commerciali con numeri che un economista riconosce.

## Equazioni chiave

Sistema gravitazionale strutturale di Anderson e van Wincoop (2003), nella forma di Piermartini e Yotov (fonti/piermartini-yotov-2016-structural-gravity.md) [V]:

1. $X_{ij} = \dfrac{Y_i E_j}{Y} \left(\dfrac{t_{ij}}{P_j \Pi_i}\right)^{1-\sigma}$
2. $\Pi_i^{1-\sigma} = \sum_j \left(\dfrac{t_{ij}}{P_j}\right)^{1-\sigma} \dfrac{E_j}{Y}$
3. $P_j^{1-\sigma} = \sum_i \left(\dfrac{t_{ij}}{\Pi_i}\right)^{1-\sigma} \dfrac{Y_i}{Y}$

$X_{ij}$ esportazioni da $i$ a $j$, $Y_i$ produzione, $E_j$ spesa, $Y$ prodotto mondiale, $t_{ij}$ costi bilaterali, $\sigma > 1$ elasticità di sostituzione; $\Pi_i$ e $P_j$ resistenze multilaterali verso l'esterno e verso l'interno (fonti/piermartini-yotov-2016-structural-gravity.md) [V].

Come entrano i dazi: $\ln \tau_{ni}$ si misura come log di uno più il dazio ad valorem bilaterale (fonti/head-mayer-2013-gravity-cookbook.md) [V]. Quindi $t_{ij} = (1 + \text{dazio}_{ij}) \cdot \tau^{\text{altri}}_{ij}$, con gli altri costi da distanza, confine, lingua, accordi [INF].

## Parametri

Meta-analisi di Head e Mayer, 2.508 stime da 159 articoli (tab. 4; mediane: tutte le stime / solo gravità strutturale) (fonti/head-mayer-2013-gravity-cookbook.md) [V]:

| Variabile | tutte | strutturale |
|---|---|---|
| distanza | -0,89 | -1,14 |
| contiguità | 0,49 | 0,52 |
| lingua comune | 0,49 | 0,33 |
| legame coloniale | 0,91 | 0,84 |
| accordo commerciale (RTA/FTA) | 0,47 | 0,28 |
| UE | 0,23 | 0,19 |
| moneta comune | 0,87 | 0,98 |

Elasticità del commercio ai costi: stima preferita $\varepsilon = -5{,}03$, mediana delle stime da variazioni tariffarie con resistenze multilaterali controllate (fonti/head-mayer-2013-gravity-cookbook.md) [V]. Quindi $1 - \sigma \approx -5$, $\sigma \approx 6$. Un dazio del 10% riduce il flusso bilaterale di circa $1{,}10^{-5} - 1 \approx -38\%$ prima degli aggiustamenti generali [INF, aritmetica].

Gli autori avvertono che i coefficienti della tabella non sono effetti causali (endogeneità degli accordi) (fonti/head-mayer-2013-gravity-cookbook.md) [V]: per il gioco vanno bene come ordini di grandezza.

## Variabili di stato

Nessuna: dati $Y_i$, $E_j$ (dal blocco macro) e $t_{ij}$ (dalle scelte di politica commerciale), il sistema si risolve a ogni turno. Stato persistente solo la matrice dei costi bilaterali e la lista degli accordi in vigore.

## Stabilità numerica

- Le equazioni 2 e 3 si risolvono a punto fisso, alternando $\Pi$ e $P$ con una normalizzazione (il sistema è definito a meno di una costante) [INF].
- Misurato [M]: 190 paesi, $\sigma = 6$, costi casuali fra 1 e 2: 10 iterazioni per tolleranza 1e-10, 0,65 ms per soluzione (numpy 2.5.1, Apple A18 Pro). Comando `python3 -I /tmp/sp-bench/bench.py`, 2026-10-16; script usa e getta fuori dal repo.
- Partendo dalla soluzione del turno prima le iterazioni scendono ancora [INF].

## Implementazioni aperte e dati

- La guida WTO-UNCTAD sulla gravità strutturale si scarica gratis con esercizi e applicazioni (fonti/wto-advanced-guide-gravity.md) [V]. Il codice Stata del paper di Piermartini e Yotov è «disponibile su richiesta», non pubblico (fonti/piermartini-yotov-2016-structural-gravity.md) [V].
- La stima (PPML con effetti fissi, raccomandata da Piermartini e Yotov) è un lavoro offline; nel gioco servono solo i coefficienti [INF].
- Dati: CEPII Gravity (distanze, lingue, accordi, flussi) e BACI (flussi bilaterali per prodotto), entrambi Etalab 2.0 (fonti/cepii-gravity.md, fonti/cepii-baci.md) [V]. Vedi [[cepii-baci-e-gravity]].

## Cosa semplifichiamo

- Commercio aggregato per paese, oppure per i 10-15 macrosettori di [[input-output-leontief]], non per prodotto [INF].
- Equilibrio generale ridotto: si ricalcolano le resistenze ma $Y_i$ ed $E_j$ arrivano dal blocco macro del turno, senza un secondo giro di convergenza [INF].
- Le sanzioni sono dazi molto alti (o $t_{ij} \to \infty$, flusso zero) [INF].

## Per il nostro gioco

- Prendiamo il sistema strutturale con $\sigma \approx 6$ e i coefficienti mediani di Head e Mayer come default.
- Accordo commerciale nuovo: moltiplica $t_{ij}^{1-\sigma}$ per $e^{0{,}28}$ (circa +32% di commercio bilaterale, valore strutturale mediano) [INF, aritmetica].
- Effetto diversione: una guerra commerciale fra due grandi sposta flussi verso terzi tramite $\Pi$ e $P$. È un comportamento che il giocatore vede e che i giochi del genere di solito non hanno [INF].
- Aperto: quanti paesi simulare davvero. Il costo (sotto 1 ms per 190) non è il vincolo.

## Fonti

- **URL** https://www.ifo.de/DocDL/cesifo1_wp6009.pdf · [V]
  **perché conta qui** il sistema strutturale scritto per intero e le raccomandazioni di stima
  **cosa se ne prende** equazioni 1-3 e la definizione delle resistenze multilaterali (fonti/piermartini-yotov-2016-structural-gravity.md)
- **URL** https://www.cepii.fr/PDF_PUB/wp/2013/wp2013-27.pdf · [V]
  **perché conta qui** la meta-analisi dei coefficienti e la stima preferita dell'elasticità tariffaria
  **cosa se ne prende** tabella 4 e $\varepsilon = -5{,}03$ (fonti/head-mayer-2013-gravity-cookbook.md)
- **URL** https://www.wto.org/english/res_e/publications_e/advancedguide2016_e.htm · [V]
  **perché conta qui** manuale WTO-UNCTAD con esercizi gratuiti
  **cosa se ne prende** riferimento per chi calibra (fonti/wto-advanced-guide-gravity.md)
- **URL** https://www.cepii.fr/CEPII/en/bdd_modele/bdd_modele_item.asp?id=8 · [V]
  **perché conta qui** le variabili gravitazionali pronte, con licenza aperta
  **cosa se ne prende** distanze, contiguità, lingue, accordi (fonti/cepii-gravity.md)

## Collegamenti

- [[cepii-baci-e-gravity]], [[un-comtrade]]: i dati, e perché preferire BACI.
- [[input-output-leontief]]: il commercio intermedio dentro le catene.
- [[finanza-pubblica]]: i dazi sono anche gettito.
- [[motore-economico-raccomandato]]
- [[logistica-e-trasporti]]: rotte e corridoi interni che portano le merci ai porti.
- [[diplomazia-e-relazioni-internazionali]]: sanzioni e accordi come mosse diplomatiche.
