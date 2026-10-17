---
tipo: modello
fonti: [fonti/sfcr-readme.md, fonti/sfcr-cran.md, fonti/jmab-github.md, fonti/ecb-eb-box-2019-02-i-g.md]
aggiornato: 2026-10-16
---
# Macro stock-flow consistent (Godley-Lavoie, modelli SIM e PC)

Famiglia di modelli in cui ogni flusso parte da un settore e arriva a un altro, e ogni stock è la somma dei flussi passati. Il disavanzo dello Stato è per costruzione l'avanzo del settore privato. Per il nostro gioco è la contabilità di base: chiude il bilancio fra famiglie, imprese, Stato, banca centrale ed estero, così il debito non può sparire né crescere senza conseguenze.

## Equazioni chiave

Modello SIM (Godley e Lavoie 2007, cap. 3), nella forma del pacchetto R `sfcr` (fonti/sfcr-readme.md) [V]:

- tasse: $T = \theta \, W N$; reddito disponibile: $YD = W N - T$
- consumo: $C = \alpha_1 YD + \alpha_2 H_{t-1}$
- moneta delle famiglie: $H_h = H_{h,t-1} + YD - C$
- produzione: $Y = C + G$, occupazione $N = Y / W$
- moneta emessa dallo Stato: $H_s = H_{s,t-1} + G - T$

Valori d'esempio del README: $G = 20$, $W = 1$, $\alpha_1 = 0{,}6$, $\alpha_2 = 0{,}4$, $\theta = 0{,}2$ (fonti/sfcr-readme.md) [V].

Risolvendo il sistema per periodo si ottiene una forma chiusa, senza solver: $Y_t = \dfrac{G_t + \alpha_2 H_{t-1}}{1 - \alpha_1 (1-\theta)}$ [INF, algebra sulle equazioni sopra]. Lo stato stazionario è $Y^* = G/\theta$: la spesa pubblica si ripaga in tasse solo quando il reddito è salito abbastanza [INF].

Modello PC (cap. 4): aggiunge i titoli di Stato, un tasso d'interesse fissato dalla banca centrale e una scelta di portafoglio fra moneta e titoli. Il README di `sfcr` rimanda ai notebook che replicano i modelli del libro (fonti/sfcr-readme.md) [V]; le equazioni di PC non sono state catturate qui [NON TROVATO in fonti/].

## Variabili di stato

Una per stock: $H$ (moneta/ricchezza delle famiglie) in SIM; in PC si aggiungono i titoli in mano a famiglie e banca centrale. Tutto il resto (reddito, consumo, tasse) è flusso ricalcolato a ogni turno. I parametri ($\alpha_1$, $\alpha_2$, $\theta$, $G$) sono le leve del giocatore o costanti calibrate.

## Stabilità numerica

- Sostituendo, $H_t = \left(1 - \dfrac{\alpha_2 \theta}{1 - \alpha_1(1-\theta)}\right) H_{t-1} + \text{cost.}$: converge se il fattore sta fra -1 e 1. Con i valori del README il fattore è 0,846 per periodo [INF].
- Misurato [M]: iterando la forma chiusa con i parametri del README, il primo periodo dà $Y = 38{,}46$ e il secondo $47{,}93$, come le colonne `Ns` del README di `sfcr` (38,5 e 47,9, fonti/sfcr-readme.md); a 60 periodi $Y = 100{,}00$ e $H = 80{,}00$, cioè lo stato stazionario $G/\theta$. Il costo per periodo è trascurabile (poche operazioni in virgola mobile). Comando: `python3 -I /tmp/sp-bench/bench.py`, Python 3.14.7, numpy 2.5.1, Apple A18 Pro, 2026-10-16. Lo script era usa e getta, fuori dal repo.
- Nei modelli più grandi le equazioni simultanee non si risolvono a mano: `sfcr` ordina il sistema come grafo (DAG) e usa un solver di Broyden per i blocchi ciclici (fonti/sfcr-readme.md) [V]. Per un gioco conviene scrivere il modello in modo che ogni blocco sia ricorsivo o abbia la sua forma chiusa [INF].
- Il controllo di coerenza è gratuito: a ogni turno la somma dei saldi settoriali deve fare zero. Se non lo fa c'è un bug [INF].

## Implementazioni aperte

- `sfcr` (R), licenza MIT (fonti/sfcr-cran.md) [V]. Utile per prototipare e calibrare fuori dal gioco, non per girare dentro.
- JMAB (Java), toolkit per modelli agent-based stock-flow consistent; il README contiene un testo di licenza in stile MIT, ma GitHub non riconosce la licenza (`NOASSERTION`) e l'ultimo push è del 2018-09-10 (fonti/jmab-github.md; `gh api repos/S120/jmab`) [V][M].
- Il README di `sfcr` cita `PKSFC` (R) e `pysolve3` (Python) come fonti d'ispirazione (fonti/sfcr-readme.md); le loro licenze non sono state verificate [NON TROVATO].

## Cosa semplifichiamo

- Niente portafoglio completo alla Tobin con decine di attività: bastano moneta, titoli di Stato e un credito bancario aggregato [INF].
- Prezzi e salari non stanno dentro SFC puro (SIM è a prezzi fissi): inflazione e tassi arrivano dal blocco [[macro-neokeynesiano-piccolo]] [INF].
- Un turno = un trimestre. I parametri comportamentali si calibrano una volta sui dati, non si stimano a runtime.
- In tensione con [[tempo-e-ritmo]]: là si propone un tick giornaliero con economia settimanale o mensile, sullo schema dei Paradox. Le fonti non decidono; se il passo economico scende sotto il trimestre vanno riscalati i parametri calibrati.

## Per il nostro gioco

- Prendiamo la matrice dei saldi settoriali come scheletro del motore: ogni azione del giocatore (spesa, tasse, emissione di debito) passa da una riga della matrice.
- È la cura diretta al «debito senza conseguenze» di Democracy 4 (debito/PIL al 3.500-4.000% senza crisi, vedi [[democracy-4]]): in SFC il debito è uno stock che paga interessi e quegli interessi diventano reddito di qualcuno.
- Resta aperto: quanti settori istituzionali esporre in UI (5 è il minimo: famiglie, imprese, banche, Stato, estero).

## Fonti

- **URL** https://cran.r-project.org/web/packages/sfcr/readme/README.html · [V]
  **perché conta qui** equazioni del modello SIM, parametri d'esempio e traiettoria numerica di riferimento
  **cosa se ne prende** il sistema SIM e i valori per verificare l'implementazione (fonti/sfcr-readme.md)
- **URL** https://cran.r-project.org/package=sfcr · [V]
  **perché conta qui** licenza del pacchetto di riferimento
  **cosa se ne prende** MIT: si può leggere e riusare liberamente per la calibrazione (fonti/sfcr-cran.md)
- **URL** https://github.com/S120/jmab · [V]
  **perché conta qui** unica implementazione aperta di AB-SFC trovata in Java
  **cosa se ne prende** solo come riferimento architetturale: è ferma al 2018 e la licenza non è dichiarata in modo standard (fonti/jmab-github.md)
- **URL** https://www.ecb.europa.eu/press/economic-bulletin/focus/2019/html/ecb.ebbox201902_06~0c96ee6f7c.en.html · [V]
  **perché conta qui** lega stock di debito, saldo primario e differenziale tasso-crescita
  **cosa se ne prende** la dinamica del debito che il blocco SFC deve riprodurre (fonti/ecb-eb-box-2019-02-i-g.md)

## Collegamenti

- [[finanza-pubblica]]: il debito pubblico è uno stock di questa matrice.
- [[banca-centrale-e-inflazione]]: il tasso del modello PC lo fissa la banca centrale.
- [[modelli-agent-based-economici]]: AB-SFC è l'estensione ad agenti della stessa contabilità.
- [[motore-economico-raccomandato]]: dove SFC diventa lo scheletro del motore.
- [[democracy-4]]: il difetto che SFC corregge.
- [[mercati-e-pops]]: il mercato di Victoria 3 che crea e distrugge denaro: il difetto che la matrice dei saldi impedisce.
- [[tempo-e-ritmo]]: il passo del tick, in tensione con il turno trimestrale.
- [[policy-e-effetti]]: la rete di Democracy che questo modello affianca per la parte economica.
