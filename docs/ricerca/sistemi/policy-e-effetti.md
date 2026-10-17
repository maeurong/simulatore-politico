---
tipo: sistema
fonti: [fonti/democracy4-modding.md, fonti/democracy4-mod-policies.md, fonti/democracy4-mod-simulation.md, fonti/democracy4-mod-situations.md, fonti/democracy3-modding.md, fonti/cliffsblog-2021-04.md, fonti/cliffsblog-d4-fixed-income.md, fonti/interview-cliff-2014.md, fonti/steam-political-process.md, fonti/vic3-wiki-interest-groups.md, fonti/steam-gps-2026.md, fonti/github-aidemocracy.md]
aggiornato: 2026-10-16
---
# Policy ed effetti: la rete policy → variabili → gruppi

Come i giochi traducono una decisione di governo in conseguenze. Il caso meglio documentato e' Democracy, che pubblica la sintassi delle formule nei file di modding; gli altri titoli si conoscono solo dalle descrizioni. Questa pagina e' il riferimento per progettare il nostro grafo di effetti.

## Democracy: oggetti, effetti, formule

**Oggetti.** Policy, valori di simulazione (statistiche), gruppi di voto, situazioni ed eventi sono tutti "oggetti" (neuroni) legati da "effetti" caricati da CSV in `data/simulation/` (fonti/democracy4-modding.md). Ogni neurone sta in [0,1] o in [-1,+1] (fonti/cliffsblog-d4-fixed-income.md).

**Sintassi di un effetto** (fonti/democracy4-modding.md):

```
[target],[value1][operator1]([value2][operator2][value3])[operator3][value4],[inertia]
Education,0.04+(0.04*x),4
Education,0.04+(0.04*x)*Technology,4
```

- `x` e' il valore dello slider della policy sorgente; si possono citare altri oggetti per nome (`Technology`, in [0,1]).
- Operatori: `+ - * / ^`; `^` serve a curvare le risposte.
- `inertia` = media sugli ultimi N valori: con 4 servono fino a 4 turni perche' una modifica arrivi a pieno effetto.
- Esempio: slider al 50% per 4 turni → `0.04 + 0.04*0.5 = 0.06`, cioe' +6% all'istruzione. Il valore effettivo scende se la policy non e' ancora implementata del tutto e si sposta con la competenza del ministro.
- Dal 2021 le equazioni accettano 5 variabili invece di 3 e termini come `(1.0-GDP)` (fonti/cliffsblog-2021-04.md).

**Policy** (`policies.csv`), colonne (fonti/democracy4-mod-policies.md):
- `Slider`: posizioni con nome, eventualmente discrete (`sliders.csv`).
- `Flags`: `UNCANCELLABLE` (sempre attiva, default 0.5) e `MULTIPLYINCOME` (gli input del gettito si moltiplicano invece di sommarsi).
- `Opposites`: policy incompatibili, cancellate in automatico.
- `Introduce/Cancel/Raise/Lower`: costo in capitale politico di ciascuna azione.
- `Department`: decide il ministro responsabile, e quindi l'efficienza.
- `PreReqs`: condizioni da `prereqs.txt` (es. `_prereq_has_coast`, `_prereq_royal_family`) (fonti/democracy4-modding.md).
- `MinCost/MaxCost`, `CostFunction` (default lineare `0+(1.0*x)`), `Cost Multiplier`: costo per turno, moltiplicato per un coefficiente paese e corretto per la competenza del ministro. Lo stesso schema vale per il gettito (`MinIncome/MaxIncome`, `IncomeFunction`).
- `Implementation`: turni per entrare a regime, corretti per la competenza.
- `nationalisation GDP percentage`: incasso o costo in caso di privatizzazione o nazionalizzazione.
- `#Effects`: da qui in poi, un effetto per cella.

**Valori di simulazione** (`simulation.csv`): `Default`, `Min/Max` (di solito 0 e 1), `Emotion` (colora la variazione come buona o cattiva; "Try to avoid any partisan value judgements"), poi due blocchi: input (effetti in entrata) e output (effetti in uscita) (fonti/democracy4-mod-simulation.md).

**Situazioni** (`situations.csv`): soglie di avvio e di arresto distinte (isteresi), costo e gettito per turno, input che le innescano e output mentre sono attive (fonti/democracy4-mod-situations.md).

**Per paese** (D3): `wealth_mod` scala costi e gettiti; `overrides/` riscrive o cancella (`DELETE`) singoli effetti solo per quel paese; `scripts/` crea "grudges" e cambia la composizione dei gruppi di voto all'avvio (fonti/democracy3-modding.md).

## Problemi documentati del modello Democracy

- **Saturazione**: il PIL arriva al 100% e si ferma. Ha 79 input e 80 output, e gli effetti sono troppo lineari (`GDP,0+(x*0.44)` per la produttivita'). La correzione e' curvare a mano (es. esponente 0.76), ma ogni ritocco sposta l'equilibrio di tutti i paesi: "Does this make Italy too easy? France too hard?" (fonti/cliffsblog-2021-04.md).
- **Solo percentuali**: gli effetti sul reddito erano tutti proporzionali, con benefici piu' alti per i ricchi. Correzione: neuroni a reddito fisso in parallelo (fonti/cliffsblog-d4-fixed-income.md).
- **Esplosione delle dipendenze**: ogni variabile nuova ne richiede altre 3-4 per restare in equilibrio (fonti/interview-cliff-2014.md).
- **Bilanciamento a mano**: "THOUSANDS of equations... They are not all works of art"; il designer scopre per caso link gia' presenti (fonti/cliffsblog-2021-04.md).

## Altri titoli

- **The Political Process**: oltre 80 tipi di legge su oltre 100 metriche, con catene esplicite (food stamps → nutrizione → diplomati → poverta' e criminalita' → gettito e spesa carceraria) (fonti/steam-political-process.md).
- **Victoria 3**: le leggi agiscono anche sull'approvazione dei gruppi d'interesse. Ogni gruppo ha una posizione da -2 a +2 su ogni legge, con somma limitata a ±5; una legge in discussione sposta l'approvazione di ±5 per gradino, e l'effetto decade in 5 anni dopo l'approvazione (fonti/vic3-wiki-interest-groups.md). Dettagli in [[victoria-3]].
- **Geo-Political Simulator**: oltre 600 dati per paese, ricalcolati in tempo reale; le formule non sono pubbliche (fonti/steam-gps-2026.md).
- **AI Democracy** (open source): stesso grafo di Democracy, archi colorati per segno e spessi per intensita'; effetti generati da GPT (fonti/github-aidemocracy.md).

## Per il nostro gioco

- **Prendere**: effetti dichiarativi in file di testo, con formula, inerzia e riferimenti ad altre variabili; isteresi sulle situazioni; override per paese; prerequisiti come flag del paese. Il modding viene gratis.
- **Cambiare**: variabili con unita' vere (moneta, persone, tassi) invece che tutto in [0,1]; effetti sia proporzionali sia assoluti fin dall'inizio; curve non lineari e saturazioni esplicite; l'economia come modello a stock e flussi a parte, collegato alla rete politica (vedi i modelli economici di altri agenti).
- **Rendere visibile**: per ogni variabile, il breakdown dei contributi in ingresso. E' cio' che i giocatori chiedono a [[power-and-revolution]] e [[rulers-of-nations]].
- **Aperto**: test automatici di bilanciamento su tutti i paesi a ogni modifica di formula. In Democracy il bilanciamento e' manuale e lo dichiarano un collo di bottiglia.

## Collegamenti
- [[democracy-4]]: il gioco da cui viene il modello.
- [[elettorato-e-gruppi-di-voto]]: dove finiscono gli effetti.
- [[ministri-e-gabinetto]]: competenza come moltiplicatore.
- [[victoria-3]]: leggi e approvazione dei gruppi d'interesse (pagina di altro agente).
- [[ip-e-giochi-di-riferimento]]: il formato si puo' riprendere, i dati no.
- [[ritardi-e-inerzia-delle-politiche]]: i tempi di risposta reali al posto dell'inerzia uguale per tutto.
- [[macro-stock-flow-consistent]]: il modello a stock e flussi che tiene l'economia fuori dalla rete in [0,1].
- [[voto-economico-e-retrospettivo]]: come l'economia arriva al voto.
- [[polarizzazione-e-fiducia-istituzionale]]: la fiducia come moltiplicatore dell'efficacia delle policy.
- [[cicli-politico-economici-e-luna-di-miele]]: la manovra pre-elettorale e il suo prezzo.
