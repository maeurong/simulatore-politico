---
tipo: modello
fonti: [fonti/wikipedia-lanchester-laws.md, fonti/rand-aggregation-3-1-rule.md, fonti/govuk-dstl-command-pe.md, fonti/wikipedia-command-mano.md]
aggiornato: 2026-10-16
---
# Lanchester e modelli di combattimento aggregati

Le equazioni di Lanchester (1916) sono il modo più semplice e più usato per risolvere uno scontro fra due forze con due numeri ciascuna: quanti sono e quanto colpiscono. A un capo di governo non serve la tattica. Gli serve una stima credibile di "quanto costa e quanto dura" una campagna, e un modello aggregato gliela dà in pochi millisecondi. La meccanica di gioco del fronte è di [[militare-e-guerra]]; qui sta solo il modello.

## Le due leggi

- **Legge lineare.** Vale per il corpo a corpo (ogni soldato ne affronta uno) e per il fuoco non mirato su un'area: le perdite dipendono dalla densità dei bersagli oltre che dal numero di armi (fonti/wikipedia-lanchester-laws.md).
- **Legge quadratica (N²).** Vale per il fuoco mirato a distanza:
  `dA/dt = −β·B`, `dB/dt = −α·A`, con A e B gli effettivi e α, β il potere offensivo per unità. La forza di un esercito cresce col **quadrato** del numero: per compensare un'inferiorità numerica N serve una superiorità qualitativa N² (fonti/wikipedia-lanchester-laws.md).
- **Via di mezzo.** Nella guerra moderna, dove valgono in parte entrambe, si usa spesso un esponente 1,5 (fonti/wikipedia-lanchester-laws.md).
- **Parametri di Helmbold.** Due indici storici, calcolati dalle forze iniziali e finali: intensità λ (quanto è sanguinosa la battaglia) e vantaggio del difensore μ. Il valore non dipende dal fatto che la legge quadratica sia vera (fonti/wikipedia-lanchester-laws.md).

## Limiti dichiarati

- La legge quadratica prevede attrito e perdite. Non vale per eserciti interi, dove lo schieramento tattico tiene fuori dal fuoco una parte delle truppe; non vale per mitragliatrici, artiglieria non guidata, armi nucleari; non vale se le perdite sono istantanee (fonti/wikipedia-lanchester-laws.md).
- Il modello a salve (missili) è discreto e include la difesa attiva (intercettori), che Lanchester ignora. Per navi e missili è il modello giusto (fonti/wikipedia-lanchester-laws.md).
- Aggregare non è gratis. Davis (RAND, 1995) mostra che se il combattimento nei sottosettori segue la legge quadratica, a livello di teatro può valere un'altra legge, o nessuna: dipende dal rapporto fra i tempi di informazione, decisione, manovra e durata dello sfondamento. Anche la "regola del 3:1" per l'attaccante vale a certi livelli e non ad altri (fonti/rand-aggregation-3-1-rule.md).
- Fit storici esistono (Gettysburg, Battaglia d'Inghilterra, Kursk), ma sono ricostruzioni a posteriori (fonti/wikipedia-lanchester-laws.md).

## Il polo opposto: simulazione a piattaforma

Command PE simula ogni ingaggio aereo e navale su un vasto database di piattaforme costruito da fonti aperte; nessuna esecuzione è uguale a un'altra, e la versione professionale ha una modalità Monte Carlo per l'analisi statistica (fonti/govuk-dstl-command-pe.md, fonti/wikipedia-command-mano.md). È la risposta "esatta" che un gioco di governo non può permettersi a ogni turno.

## Per il nostro gioco

- **Prendiamo** Lanchester come risolutore di campagna: per ogni teatro, una forza efficace per parte (effettivi × qualità × prontezza × logistica) e un'equazione con esponente fra 1 e 2. Dà al giocatore la lezione giusta: la qualità costa, ma non sostituisce la massa in modo lineare [INF].
- **Prendiamo** λ e μ di Helmbold come parametri di tuning leggibili: "guerra di logoramento" e "difensore trincerato" diventano due manopole [INF].
- **Evitiamo** di presentare il risultato come certo. L'esito va mostrato come distribuzione (pochi campioni Monte Carlo per turno costano niente) con la sua incertezza, come fa Command PE in modalità professionale [INF].
- **Aperto:** il modello a salve per il dominio navale e missilistico, se il gioco lo modella a parte. Da decidere con [[militare-e-guerra]].

## Collegamenti
- [[militare-e-guerra]]: la meccanica di fronte che consuma questo risolutore.
- [[prontezza-e-spesa-per-la-difesa]]: da dove vengono effettivi, qualità e prontezza.
- [[costo-politico-della-guerra]]: le perdite che il risolutore produce diventano costo politico.
- [[command-modern-operations]]: l'esempio di simulazione a piattaforma.

## Riferimenti
- **URL** https://en.wikipedia.org/wiki/Lanchester%27s_laws · [V] (secondaria)
  **perché conta qui** formule, limiti ed esponente 1,5 in un posto solo
  **cosa se ne prende** le due equazioni, la regola N², i parametri di Helmbold come manopole
- **URL** https://www.rand.org/pubs/monograph_reports/MR638.html · [V]
  **perché conta qui** dice quando la legge aggregata a livello teatro regge e quando no
  **cosa se ne prende** cautela sull'aggregazione e sulla regola 3:1; esponente come parametro, non costante
- **URL** https://www.gov.uk/government/news/new-wargaming-and-analysis-software-for-defence · [V]
  **perché conta qui** l'alternativa ad alta risoluzione, in uso alla Difesa britannica
  **cosa se ne prende** l'esito come distribuzione, non come numero
