---
tipo: modello
fonti: [fonti/jasss-review-laver-sergenti.md, fonti/plos-lehrer-schumacher-2018.md, fonti/arxiv-fernandez-gracia-2014-voter-model.md, fonti/jasss-flache-2017-influenza-sociale.md]
aggiornato: 2026-10-16
---
# ABM elettorali

Modelli ad agenti in cui partiti ed elettori si muovono in uno spazio di policy con regole semplici, senza ottimizzazione globale. Il riferimento è il modello di Laver (2005) sviluppato in Laver e Sergenti (2011), esteso con la formazione dei governi da Lehrer e Schumacher (2018). Per il gioco forniscono le regole di comportamento dei partiti IA, già studiate e tarate su sistemi partitici europei.

## Regole

Impianto base: elettori con punti ideali fissi in 2D, distribuiti in modo normale, votano il partito più vicino; sistema perfettamente proporzionale; a ogni turno i partiti applicano una regola, si vota, i partiti leggono il risultato (fonti/plos-lehrer-schumacher-2018.md) [V].

- **Sticker**: non si muove mai. Partiti di nicchia molto coesi (fonti/plos-lehrer-schumacher-2018.md) [V].
- **Aggregator**: si sposta sul baricentro dei propri elettori (fonti/jasss-review-laver-sergenti.md) [V].
- **Hunter**: se l'ultimo spostamento ha portato voti lo ripete nella stessa direzione, altrimenti prova una direzione a caso; è un hill-climbing (fonti/plos-lehrer-schumacher-2018.md) [V].
- **Predator**: si sposta verso il partito più grande (Laver 2005, APSR 99(2): 263-281) [INF, abstract letto solo nei risultati di ricerca].
- **Governator** (Lehrer-Schumacher): si sposta verso la posizione del governo senza superarla; **Satisficing Governator**: resta fermo se è al governo, altrimenti come Governator (fonti/plos-lehrer-schumacher-2018.md) [V].
- Estensioni Laver-Sergenti: nascita e morte endogena dei partiti, parametri di velocità di adattamento ed esplorazione/sfruttamento (fonti/jasss-review-laver-sergenti.md) [V].

## Evidenza

- Con elettori distribuiti in modo simmetrico gli Aggregator rappresentano meglio gli elettori; con distribuzione asimmetrica più partiti Hunter migliorano la rappresentanza; il successo delle regole vote-seeking dipende molto dai parametri (fonti/jasss-review-laver-sergenti.md) [V].
- Il libro calibra il modello sui sistemi partitici europei 1989-2002 e ne ricava distribuzioni di parametri (quanto un partito usa regole vote-seeking, carisma del leader, posizione dei nuovi partiti) (fonti/jasss-review-laver-sergenti.md) [V].
- Con la formazione dei governi, gli Aggregator producono anche la miglior corrispondenza fra politica del governo e preferenze degli elettori; più partiti riducono la distanza fra elettori e partito più vicino ("system misery") qualunque sia la regola; i Governator formano più spesso maggioranze sovradimensionate (fonti/plos-lehrer-schumacher-2018.md) [V].
- Nelle simulazioni evolutive nessuna regola domina: quota mediana di ciascuna ≈ 20%, al massimo metà dei partiti con la stessa regola (fonti/plos-lehrer-schumacher-2018.md) [V].
- Limite generale: la rassegna Flache rileva poca validazione empirica dei modelli ad agenti di influenza sociale (fonti/jasss-flache-2017-influenza-sociale.md) [V]. Il voter model rumoroso è un'eccezione, validato sul voto per contea USA (fonti/arxiv-fernandez-gracia-2014-voter-model.md) [V].

## Parametri tipici

| Parametro | Valore usato | Fonte |
|---|---|---|
| dimensioni dello spazio | 2 | (fonti/plos-lehrer-schumacher-2018.md) [V] |
| distribuzione elettori | normale, punti fissi | (fonti/plos-lehrer-schumacher-2018.md) [V] |
| peso policy vs ufficio α | 0 (solo ufficio) - 1 (solo policy); default 0,5 | (fonti/plos-lehrer-schumacher-2018.md) [V] |
| fattore di sconto | 0,5 nelle figure | (fonti/plos-lehrer-schumacher-2018.md) [V] |
| numero di partiti | variabile; 5 nelle figure principali | (fonti/plos-lehrer-schumacher-2018.md) [V] |

## Costo computazionale

- Per turno: assegnare N elettori al partito più vicino `O(N·P)` più l'aggiornamento dei P partiti; il baricentro per Aggregator è `O(N)` [INF].
- Il costo che pesa è statistico: Lehrer-Schumacher trattano il modello come catena di Markov e mediano 500 catene dopo il burn-in per ogni configurazione (fonti/plos-lehrer-schumacher-2018.md) [V]. Il gioco non ne ha bisogno: gira una sola traiettoria.
- Il libro rende disponibile il codice NetLogo dei modelli (fonti/jasss-review-laver-sergenti.md) [V].

## Cosa semplifichiamo

- Elettori fissi nei modelli Laver: noi li muoviamo con [[opinion-dynamics]] [INF].
- Voto sincero al partito più vicino: noi usiamo il logit con valence di [[voto-spaziale]] [INF].
- Proporzionale perfetto: noi passiamo il voto dal sistema elettorale reale ([[sistemi-elettorali]]) [INF].

## Per il nostro gioco

- I partiti IA possono usare direttamente queste regole: sono poche righe ciascuna, hanno nomi leggibili e risultati noti. Un mix di Sticker (estremi, identitari), Aggregator (partiti di militanti), Hunter (partiti personali/pigliatutto) e Governator (centristi da coalizione) produce un sistema partitico credibile.
- La regola di ogni partito IA può essere un tratto visibile al giocatore.
- La nascita di nuovi partiti in posizioni scoperte (estensione Laver-Sergenti) è una meccanica pronta per frammentazione e partiti di protesta.

## Collegamenti

- [[voto-spaziale]], [[opinion-dynamics]]: lato elettori.
- [[formazione-di-coalizioni]]: Lehrer-Schumacher aggiungono la formazione del governo.
- [[partiti-e-coalizioni]]: come i giochi esistenti modellano i partiti avversari (Democracy non ne modella le posizioni).
- [[lawgivers-2]], [[democracy-4]]: confronto con i partiti nei giochi esistenti.

## Riferimenti

- **URL** https://www.jasss.org/15/3/reviews/5.html · [V]
  **perché conta qui** riassume regole, risultati e calibrazione europea del libro Laver-Sergenti
  **cosa se ne prende** l'elenco di regole Sticker/Aggregator/Hunter e il fatto che esista una calibrazione 1989-2002
- **URL** https://pmc.ncbi.nlm.nih.gov/articles/PMC5796695/ · [V]
  **perché conta qui** estende il modello alla formazione dei governi, open access, con parametri espliciti
  **cosa se ne prende** le regole Governator, i parametri α e sconto, i risultati sulla rappresentanza
- **URL** https://ar5iv.labs.arxiv.org/html/1309.1131 · [V]
  **perché conta qui** l'unico ABM elettorale di questa pagina validato su dati
  **cosa se ne prende** la parte elettori con rumore e mobilità
- **URL** https://resolve.cambridge.org/core/journals/american-political-science-review/article/abs/policy-and-the-dynamics-of-political-competition/89669AF3A2D7CA5196C72272B4AEE3BC · [INF] (Laver 2005, APSR 99(2): 263-281; solo abstract visto nei risultati di ricerca)
  **perché conta qui** articolo originale con la regola Predator e l'applicazione a serie di sondaggi reali
  **cosa se ne prende** la regola Predator; da leggere se la si implementa
