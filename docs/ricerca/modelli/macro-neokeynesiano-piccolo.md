---
tipo: modello
fonti: [fonti/rudebusch-svensson-1999-policy-rules.md, fonti/fed-policy-rules.md, fonti/dynare-home.md]
aggiornato: 2026-10-17
---
# Macro neokeynesiano piccolo (curva IS, Phillips, regola di Taylor)

Tre equazioni bastano a legare domanda, inflazione e tasso d'interesse: una curva IS (la domanda scende quando il tasso reale sale), una curva di Phillips (l'inflazione sale quando l'economia gira sopra il potenziale) e una regola per il tasso. Per il nostro gioco è il blocco che fa muovere PIL, disoccupazione e inflazione in risposta alle scelte, con ritardi credibili.

## Due varianti

**Forward-looking (neokeynesiano da manuale)** [INF, forma standard non catturata in fonti/]:
$x_t = E_t x_{t+1} - \sigma (i_t - E_t \pi_{t+1} - r^n_t)$, $\pi_t = \beta E_t \pi_{t+1} + \kappa x_t$, più una regola per $i_t$. Le aspettative razionali impongono di risolvere il sistema in avanti (condizioni di Blanchard-Kahn) e la soluzione è unica solo se il tasso risponde più che uno a uno all'inflazione. Dynare fa questo lavoro, ma è un ambiente di stima, GPL v3 o successiva (fonti/dynare-home.md) [V].

**Backward-looking (Rudebusch e Svensson 1999)**, stimata su dati USA 1961-1996, trimestrale (fonti/rudebusch-svensson-1999-policy-rules.md) [V]:

- Phillips: $\pi_{t+1} = 0{,}70\,\pi_t - 0{,}10\,\pi_{t-1} + 0{,}28\,\pi_{t-2} + 0{,}12\,\pi_{t-3} + 0{,}14\,y_t + \varepsilon_{t+1}$
- IS: $y_{t+1} = 1{,}16\,y_t - 0{,}25\,y_{t-1} - 0{,}10\,(\bar\imath_t - \bar\pi_t) + \eta_{t+1}$

dove $y$ è l'output gap in punti percentuali, $\bar\imath$ e $\bar\pi$ sono le medie a quattro trimestri di tasso e inflazione. I coefficienti dell'inflazione ritardata sommano a uno (vincolo non rifiutato, p = 0,42): curva di Phillips verticale nel lungo periodo. Il rapporto di sacrificio è circa 3 (fonti/rudebusch-svensson-1999-policy-rules.md) [V].

**Regola di Taylor** (forma della Fed): $R_t = r^{LR} + \pi_t + 0{,}5(\pi_t - \pi^*) + 0{,}5(y_t - y^P_t)$, con varianti a peso doppio sul gap, con vincolo del limite inferiore e inerziale ($R_t = 0{,}85 R_{t-1} + 0{,}15[\dots]$) (fonti/fed-policy-rules.md) [V].

## Variabili di stato

Variante backward-looking: quattro ritardi di inflazione, due di output gap, quattro di tasso. Dieci numeri per paese. Nessuna aspettativa da risolvere: le aspettative sono adattive, implicite nei ritardi.

## Stabilità numerica

Misurato [M] con le equazioni RS sopra più la regola di Taylor della Fed, shock iniziale di +2 punti d'inflazione, 400 trimestri senza rumore:

- risposta all'inflazione $\phi_\pi = 0{,}5$ (cioè 1,5 in totale): scarto finale 8e-10, converge;
- $\phi_\pi = 0$ (tasso che segue l'inflazione uno a uno, tasso reale fermo): scarto finale 1,27, l'inflazione non torna più al target (radice unitaria);
- $\phi_\pi = -0{,}2$: scarto finale 113, diverge.

Costo: 0,004 ms per trimestre in Python puro. Comando `python3 -I /tmp/sp-bench/bench.py`, Python 3.14.7, Apple A18 Pro, 2026-10-16; script usa e getta fuori dal repo.

Conseguenza per il gioco: se il giocatore controlla la banca centrale e tiene i tassi bassi con inflazione alta, l'inflazione deriva. È il comportamento giusto, ed è il principio di Taylor reso giocabile [INF].

## Implementazioni aperte

- Dynare (GPL v3+) per stimare o verificare offline una variante forward-looking (fonti/dynare-home.md) [V]. La GPL non tocca il gioco se Dynare resta fuori dal binario [INF].
- Per RS non serve libreria: sono due equazioni lineari.

## Cosa semplifichiamo

- Si sceglie la variante backward-looking: niente solver di aspettative razionali, aggiornamento esplicito, nessun caso di indeterminatezza da gestire a runtime [INF].
- Il prezzo è la critica di Lucas: con aspettative adattive un cambio di regime (banca centrale nuova e credibile) non sposta subito le aspettative. Si compensa con una variabile di credibilità che pesa l'inflazione attesa fra target e passato (vedi [[banca-centrale-e-inflazione]]) [INF].
- I coefficienti USA vanno ricalibrati per paese; i valori RS fanno da default. Per l'Italia il primo default sono i coefficienti dell'area euro di Garnier-Wilhelmsen, in [[economia-italiana-parametri]].

## Per il nostro gioco

- Prendiamo RS come motore congiunturale: ritardi di 2-6 trimestri fra decisione ed effetto, che sono la sostanza del dilemma politico (paghi oggi, raccogli dopo le elezioni).
- Evitiamo il ciclo economico esogeno a sinusoide di Democracy 4 ([[democracy-4]]): qui il ciclo nasce dagli shock e dalla dinamica a due ritardi dell'IS.
- Aperto: frequenza del turno. RS è trimestrale; un turno mensile richiede di riscalare i coefficienti.

## Fonti

- **URL** https://www.nber.org/chapters/c7417.pdf · [V]
  **perché conta qui** modello IS-Phillips stimato, piccolo e senza aspettative razionali
  **cosa se ne prende** le due equazioni con i coefficienti, il vincolo di verticalità, il rapporto di sacrificio (fonti/rudebusch-svensson-1999-policy-rules.md)
- **URL** https://www.federalreserve.gov/monetarypolicy/policy-rules-and-how-policymakers-use-them.htm · [V]
  **perché conta qui** la regola di Taylor e le sue varianti nella forma che usa la Fed
  **cosa se ne prende** la regola di default della banca centrale non controllata dal giocatore (fonti/fed-policy-rules.md)
- **URL** https://www.dynare.org/ · [V]
  **perché conta qui** strumento standard per i modelli forward-looking
  **cosa se ne prende** licenza GPL v3+: solo uso offline, fuori dal binario (fonti/dynare-home.md)

## Collegamenti

- [[banca-centrale-e-inflazione]]: chi muove il tasso e con quale credibilità.
- [[mercato-del-lavoro-e-disuguaglianza]]: dall'output gap alla disoccupazione con Okun.
- [[macro-stock-flow-consistent]]: il blocco che tiene i conti mentre questo muove i prezzi.
- [[motore-economico-raccomandato]]
- [[tempo-e-ritmo]]: il passo del tick, che decide la frequenza dei coefficienti.
