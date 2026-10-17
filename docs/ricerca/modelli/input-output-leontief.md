---
tipo: modello
fonti: [fonti/wikipedia-input-output-model.md, fonti/eurostat-figaro-metadata.md, fonti/oecd-icio-dataset.md, fonti/wiod-2016-release.md]
aggiornato: 2026-10-16
---
# Input-output di Leontief: settori e catene

Una matrice dice quanto ogni settore compra dagli altri per produrre un euro. Con quella matrice uno shock su un settore (embargo sul gas, sciopero dei trasporti, dazio sull'acciaio) si propaga lungo le catene di fornitura. Per il nostro gioco è il livello settoriale sotto il PIL aggregato: rende visibile chi perde e chi guadagna.

## Equazioni chiave

- Quantità: $x = A x + d \Rightarrow x = (I - A)^{-1} d$, con $x$ produzione lorda per settore, $d$ domanda finale, $A$ coefficienti tecnici (fonti/wikipedia-input-output-model.md) [V]. $(I-A)^{-1}$ è l'inversa di Leontief.
- Condizione di Hawkins-Simon: se i minori principali di $I - A$ sono tutti positivi, la produzione richiesta è non negativa (fonti/wikipedia-input-output-model.md) [V]. In pratica vale quando ogni colonna di $A$ somma meno di 1, cioè ogni settore produce valore aggiunto [INF].
- Prezzi (duale): $p = (I - A^\top)^{-1} v$, con $v$ valore aggiunto per unità; un rincaro dell'energia si trasmette ai prezzi di tutti i settori a valle [INF, forma standard].
- Versione multi-paese: stessa algebra con $A$ a blocchi paese × settore; i blocchi fuori diagonale sono le importazioni intermedie [INF].

## Variabili di stato

$A$ (costante o aggiornata di rado), $d$ per paese e settore (viene dal blocco macro), $x$ e $p$ ricalcolati. Lo stato vero è solo $A$; il resto è funzione della domanda del turno.

## Stabilità numerica

- È un sistema lineare: nessuna dinamica, nessuna divergenza. Il rischio è solo $I - A$ quasi singolare se una colonna di $A$ si avvicina a 1 [INF].
- Misurato [M] con numpy 2.5.1 su Apple A18 Pro, matrice casuale con colonne che sommano 0,5:
  - $n = 64$ (un paese, 64 settori): risoluzione 0,017 ms;
  - $n = 2944$ (46 paesi × 64 settori): risoluzione 90 ms; con l'inversa precalcolata la moltiplicazione costa 1,1 ms, ma l'inversa occupa 69 MB.
  Comando `python3 -I /tmp/sp-bench/bench.py`, 2026-10-16; script usa e getta fuori dal repo.
- Una tavola FIGARO completa ha 50 regioni × 64 settori = 3.200 righe (fonti/eurostat-figaro-metadata.md), quindi un po' più grande del caso misurato [INF].

## Dati e implementazioni

- FIGARO (Eurostat-JRC): 27 Stati UE, 2 EFTA, 5 candidati, 15 partner principali e resto del mondo; 64 prodotti e industrie; milioni di euro a prezzi base; annuale dal 2010 con ritardo T-2, l'edizione 2026 copre 2010-2024 (fonti/eurostat-figaro-metadata.md) [V]. Licenza: quella dei dati Eurostat, vedi [[eurostat]].
- OECD ICIO edizione 2025: 80 economie più il resto del mondo, versione regolare ed estesa (fonti/oecd-icio-dataset.md) [V]. Licenza: vedi [[oecd]].
- WIOD 2016: 43 paesi più resto del mondo, 2000-2014, CC BY 4.0, non più aggiornato (fonti/wiod-2016-release.md) [V].
- Implementazione: nessuna libreria dedicata serve, basta un solver lineare denso (LU). Dettaglio in [[input-output-tables]].

## Cosa semplifichiamo

- Aggregare 64 settori in 10-15 macrosettori leggibili dal giocatore (energia, agricoltura, manifattura, edilizia, servizi pubblici…): la matrice scende a $n \approx 15$ per paese e il costo diventa trascurabile [INF].
- Paesi: modellare in dettaglio solo il paese giocato e i 10-20 partner principali, il resto del mondo come blocco unico [INF].
- $A$ fissa nel breve periodo (coefficienti tecnici stabili), aggiornata solo da eventi (innovazione, sanzioni che costringono a sostituire fornitori) [INF].
- Niente vincoli di capacità nel modello base: Leontief puro non razziona. Uno shock d'offerta (embargo) si rende con un tetto sulla produzione del settore e un ricalcolo della domanda soddisfatta [INF].

## Per il nostro gioco

- Prendiamo l'inversa di Leontief per il «chi soffre di uno shock» e il modello prezzi per l'inflazione importata settore per settore.
- La UI può mostrare la catena (gas → fertilizzanti → agricoltura → prezzi alimentari) direttamente dalle colonne dell'inversa.
- Aperto: se la tavola multi-paese gira a ogni turno o solo su eventi. Con l'aggregazione proposta il costo non decide; decide la leggibilità.

## Fonti

- **URL** https://en.wikipedia.org/wiki/Input%E2%80%93output_model · [V]
  **perché conta qui** formula, esempio numerico e condizione di Hawkins-Simon
  **cosa se ne prende** l'algebra di base e il caso di prova a due settori (fonti/wikipedia-input-output-model.md)
- **URL** https://ec.europa.eu/eurostat/cache/metadata/en/naio_10_fcp_esms.htm · [V]
  **perché conta qui** copertura e cadenza delle tavole FIGARO
  **cosa se ne prende** dimensione della matrice (50 regioni × 64) e anni disponibili (fonti/eurostat-figaro-metadata.md)
- **URL** https://www.oecd.org/en/data/datasets/inter-country-input-output-tables.html (snapshot Wayback 2026-10-06) · [V]
  **perché conta qui** l'alternativa a copertura mondiale
  **cosa se ne prende** 80 economie nell'edizione 2025 (fonti/oecd-icio-dataset.md)
- **URL** https://www.rug.nl/ggdc/valuechain/wiod/wiod-2016-release?lang=en · [V]
  **perché conta qui** tavola mondiale in CC BY 4.0
  **cosa se ne prende** solo come riserva: ferma al 2014 (fonti/wiod-2016-release.md)

## Collegamenti

- [[input-output-tables]]: le tavole, la licenza, cosa si ridistribuisce.
- [[commercio-modello-gravitazionale]]: i flussi bilaterali che le tavole multi-paese contengono.
- [[macro-stock-flow-consistent]]: la domanda finale $d$ arriva da lì.
- [[motore-economico-raccomandato]]
- [[produzione-industriale-e-risorse]]: come i grand strategy rappresentano la produzione.
- [[mercati-e-pops]]: i beni singoli di Victoria 3 che i macrosettori sostituiscono.
