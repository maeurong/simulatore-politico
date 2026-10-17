---
tipo: modello
fonti: [fonti/poledna-2023-abm-forecasting.md, fonti/beforeit-arxiv-2502-13267.md, fonti/beforeit-docs.md, fonti/eurace-unibi-model.md, fonti/jmab-github.md]
aggiornato: 2026-10-16
---
# Modelli agent-based economici (Eurace, ABM macro) e loro costo

Invece di equazioni aggregate si simulano famiglie, imprese, banche e Stato come agenti con regole semplici; il PIL è la somma di quello che fanno. Oggi un ABM calibrato su dati nazionali prevede quanto un VAR o un DSGE. Per il nostro gioco la domanda è se vale il costo: in calcolo, in calibrazione e in leggibilità.

## Modelli di riferimento

- **Poledna, Miess, Hommes e Rabitsch (2023)**: ABM dell'Austria con tutti i settori istituzionali, milioni di agenti eterogenei, scala 1:1 (ogni agente è una persona fisica o giuridica, parametri da censimento e demografia d'impresa); compete con VAR e DSGE nelle previsioni fuori campione. Articolo open access CC BY (fonti/poledna-2023-abm-forecasting.md) [V].
- **BeforeIT.jl** (Banca d'Italia): implementazione in Julia di quel modello. Le aspettative degli agenti sono regole AR(1), i mercati funzionano per ricerca e abbinamento; il modello è calibrato su contabilità nazionale, dati settoriali, tavole input-output e demografia d'impresa (fonti/beforeit-arxiv-2502-13267.md) [V]. Contiene le parametrizzazioni di Austria e Italia (fonti/beforeit-docs.md) [V].
- **Eurace@Unibi** (Bielefeld): economia chiusa con struttura spaziale, pensata come quadro unico per analisi di politiche; sorgente scaricabile, eseguibile in una virtual appliance (fonti/eurace-unibi-model.md) [V]. Licenza del sorgente non indicata nella pagina [NON TROVATO].
- **JMAB** (Java): toolkit per ABM stock-flow consistent, configurato con Spring; testato con Java 6-7, ultimo push 2018-09-10 (fonti/jmab-github.md; `gh api repos/S120/jmab`) [V][M].

## Equazioni e stato

Non c'è un sistema chiuso da scrivere: lo stato è il vettore di tutti gli agenti (redditi, ricchezza, prezzi, scorte, debiti), aggiornato da una sequenza di funzioni per passo. In BeforeIT un'esecuzione è un ciclo su `step!`, e ogni passo è una serie di chiamate che aggiornano gli attributi di un tipo di agente (fonti/beforeit-arxiv-2502-13267.md) [V].

## Costo computazionale

- BeforeIT, Austria a scala 1:1000 (circa 8.000 agenti) e 1:1 (circa 8 milioni): a thread singolo è circa 17 volte più veloce del Matlab originale e 4 volte del C generato da Matlab sul modello piccolo; 35 e 10 volte sul modello grande. Benchmark su AMD Ryzen 5 5600H, 16 GB. Il multithreading per settore accelera in modo sub-lineare (fonti/beforeit-arxiv-2502-13267.md) [V].
- I tempi assoluti per passo sono solo in un grafico (fig. 3), non nel testo catturato [NON TROVATO].
- Gli ABM sono stocastici: servono insiemi di esecuzioni Monte Carlo per una previsione, e BeforeIT ha `ensemblerun` apposta (fonti/beforeit-arxiv-2502-13267.md) [V]. In un gioco, una sola esecuzione per turno è una singola realizzazione rumorosa [INF].
- La calibrazione di BeforeIT è ancora un «prototipo di ricerca non rifinito» secondo gli autori stessi (fonti/beforeit-arxiv-2502-13267.md) [V].

## Licenze

- BeforeIT.jl: il paper del 2025 dichiara AGPL-3.0 (fonti/beforeit-arxiv-2502-13267.md); la documentazione attuale dice Apache 2.0 (fonti/beforeit-docs.md). Il cambio è il commit `305c66a` del 2026-07-24, «update license from AGPLv3 to Apache License 2.0»; GitHub riporta oggi `Apache-2.0` (`gh api repos/bancaditalia/BeforeIT.jl`, HEAD `5a0ff17`, 2026-10-16) [M].
- Poledna et al. 2023: articolo CC BY (fonti/poledna-2023-abm-forecasting.md) [V].

## Cosa semplifichiamo

Se si usa, solo in forma ridotta [INF]:

- agenti come gruppi rappresentativi (10 decili di famiglie, 10-15 settori di imprese) invece che individui: torna di fatto a un modello aggregato con eterogeneità discreta;
- regole comportamentali di BeforeIT (aspettative AR(1)) riusabili nei blocchi aggregati senza simulare gli agenti;
- Julia non si incorpora facilmente in un motore di gioco: il modello andrebbe riscritto nel linguaggio del gioco, e la licenza Apache 2.0 ora lo permette [INF].

## Per il nostro gioco

- Raccomandazione: non usare un ABM completo come motore del turno. Il costo non è solo il calcolo: è la calibrazione (prototipo anche per gli autori), la variabilità fra esecuzioni e il fatto che il giocatore non può leggere la causa di un esito.
- Prendiamo le idee: settori eterogenei, aspettative adattive, contabilità SFC (vedi [[macro-stock-flow-consistent]]), calibrazione da tavole input-output (vedi [[input-output-leontief]]).
- Aperto: una modalità «analisi profonda» offline (scenario a fine mandato calcolato con un ABM) è possibile ma non necessaria.

## Fonti

- **URL** https://pure.iiasa.ac.at/18339/1/1-s2.0-S0014292122001891-main%20%281%29.pdf · [V]
  **perché conta qui** prova che un ABM calibrato prevede quanto i modelli standard
  **cosa se ne prende** struttura dei settori, scala 1:1, licenza CC BY dell'articolo (fonti/poledna-2023-abm-forecasting.md)
- **URL** https://arxiv.org/html/2502.13267v1 · [V]
  **perché conta qui** unica implementazione aperta industriale di un ABM macro calibrato
  **cosa se ne prende** architettura a passi, benchmark relativi, stato della calibrazione (fonti/beforeit-arxiv-2502-13267.md)
- **URL** https://docs.juliahub.com/General/BeforeIT/stable/ · [V]
  **perché conta qui** licenza corrente e parametrizzazioni disponibili
  **cosa se ne prende** Apache 2.0; Austria e Italia (fonti/beforeit-docs.md)
- **URL** https://www.uni-bielefeld.de/fakultaeten/wirtschaftswissenschaften/lehrbereiche/etace/eurace@unibi/the-eurace@unibi-model/ · [V]
  **perché conta qui** il modello europeo di riferimento per l'analisi di politiche
  **cosa se ne prende** solo come confronto: economia chiusa, licenza non dichiarata (fonti/eurace-unibi-model.md)
- **URL** https://github.com/S120/jmab · [V]
  **perché conta qui** AB-SFC aperto in Java
  **cosa se ne prende** solo come riferimento, progetto fermo (fonti/jmab-github.md)

## Collegamenti

- [[macro-stock-flow-consistent]]
- [[input-output-leontief]]
- [[motore-economico-raccomandato]]
- [[cittadini-simulati]]: il costo dell'agente individuale documentato nei city builder.
