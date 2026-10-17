---
tipo: modello
fonti: [fonti/ecb-strategy-statement-2025.md, fonti/fed-policy-rules.md, fonti/rudebusch-svensson-1999-policy-rules.md]
aggiornato: 2026-10-17
---
# Banca centrale e inflazione

La banca centrale fissa il tasso di policy per tenere l'inflazione al target; l'inflazione risponde al tasso con ritardo, tramite domanda e aspettative. Per il nostro gioco serve decidere chi muove il tasso (il giocatore o una banca centrale indipendente) e cosa succede quando la regola viene forzata.

## Equazioni chiave

- **Obiettivo.** La BCE punta al 2% nel medio periodo, in modo simmetrico: scostamenti sopra e sotto sono ugualmente indesiderati. Davanti a deviazioni ampie e persistenti serve una risposta forte o prolungata, per non disancorare le aspettative. Lo strumento principale sono i tassi di policy; vicino al limite inferiore si usano altri strumenti (rifinanziamenti a lungo termine, acquisti di titoli, tassi negativi, forward guidance). Strategia aggiornata a giugno 2025 (fonti/ecb-strategy-statement-2025.md) [V].
- **Regola.** Taylor e varianti nella forma della Fed: $R_t = r^{LR} + \pi_t + 0{,}5(\pi_t - \pi^*) + 0{,}5(y_t - y^P_t)$; regola con limite inferiore $R = \max\{R^{BA} - Z_t, ELB\}$, dove $Z_t$ accumula lo stimolo mancato; regola inerziale con 0,85 sul tasso precedente. La Fed le consulta senza seguirle in modo meccanico (fonti/fed-policy-rules.md) [V].
- **Trasmissione.** Curva di Phillips accelerazionista e IS con tasso reale medio a quattro trimestri, da [[macro-neokeynesiano-piccolo]] (fonti/rudebusch-svensson-1999-policy-rules.md) [V].
- **Aspettative e credibilità** [INF, proposta di gioco]: $\pi^e_t = c_t\,\pi^* + (1 - c_t)\,\bar\pi_{t-1}$, con credibilità $c_t \in [0,1]$ che sale quando l'inflazione resta vicina al target e crolla quando il governo forza la banca centrale. Sostituisce $\pi_t$ con $\pi^e_t$ nella Phillips di RS.

## Variabili di stato

Tasso di policy (e il suo ritardo, per la regola inerziale), credibilità $c$, stock di titoli acquistati (se si modella il QE), accumulo $Z$ per la regola con limite inferiore.

## Stabilità numerica

- Con RS e la regola di Taylor della Fed l'inflazione torna al target; con risposta all'inflazione nulla resta lontana, con risposta negativa diverge (misure in [[macro-neokeynesiano-piccolo]]) [M].
- Il limite inferiore introduce una non linearità: in deflazione il tasso reale sale da solo e la recessione si avvita. È realistico e va lasciato, ma con un'uscita per il giocatore (acquisti di titoli, politica fiscale) [INF].
- La credibilità va aggiornata lentamente (media mobile), altrimenti oscilla di turno in turno [INF].

## Implementazioni aperte

Nessuna libreria necessaria. Le regole sono una riga ciascuna.

## Cosa semplifichiamo

- Un solo tasso di policy; il tasso sul debito e sui prestiti ne deriva con uno spread (vedi [[finanza-pubblica]]) [INF].
- Il cambio entra solo come canale dell'inflazione importata, non come mercato a sé (non coperto da fonti catturate) [NON TROVATO].
- Moneta come quantità: si ignora. Il tasso è lo strumento, come nella strategia BCE [INF].

## Per il nostro gioco

- Default: banca centrale indipendente che segue la regola di Taylor con inerzia. Il giocatore può nominarne il governatore o toglierle l'indipendenza: in quel caso controlla il tasso, ma la credibilità scende. **Superato per la v1** (decisione del 17/10/2026, [[decisioni]]): l'Italia è nell'euro, il tasso lo fissa la BCE con una regola di Taylor sull'area euro in cui l'Italia pesa per la sua quota di PIL; niente governatore da nominare e niente credibilità monetaria propria. La punizione dei mercati passa dallo spread ([[finanza-pubblica]]).
- Prendiamo il target simmetrico al 2% e il limite inferiore. Il giocatore che stampa moneta per finanziare il disavanzo ottiene inflazione con ritardo, non subito: la tentazione resta reale.
- Chiuso il 17/10/2026: la v1 modella il paese senza banca centrale propria (membro dell'euro), con spread non lineare e TPI condizionato alle regole UE ([[decisioni]]).

## Fonti

- **URL** https://www.ecb.europa.eu/mopo/strategy/strategy-review/ecb.strategyreview202506_strategy_statement.en.html · [V]
  **perché conta qui** obiettivo e strumenti di una banca centrale reale, aggiornati al 2025
  **cosa se ne prende** target simmetrico al 2%, limite inferiore, elenco degli strumenti (fonti/ecb-strategy-statement-2025.md)
- **URL** https://www.federalreserve.gov/monetarypolicy/policy-rules-and-how-policymakers-use-them.htm · [V]
  **perché conta qui** le regole con i loro coefficienti
  **cosa se ne prende** Taylor, balanced-approach, regola con ELB e inerziale (fonti/fed-policy-rules.md)
- **URL** https://www.nber.org/chapters/c7417.pdf · [V]
  **perché conta qui** canale di trasmissione stimato
  **cosa se ne prende** la Phillips accelerazionista su cui si innesta la credibilità (fonti/rudebusch-svensson-1999-policy-rules.md)

## Collegamenti

- [[macro-neokeynesiano-piccolo]]: IS e Phillips.
- [[finanza-pubblica]]: tasso di policy e costo del debito.
- [[motore-economico-raccomandato]]
