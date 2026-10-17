---
tipo: modello
fonti: [fonti/druckman-warwick-2006-portfolio.md, fonti/plos-lehrer-schumacher-2018.md, fonti/parlgov-about.md, fonti/parlgov-dataverse-2024-licenza.md, fonti/camera-dossier-ac2822-legge-elettorale-2026.md]
aggiornato: 2026-10-16
---
# Formazione di coalizioni

Chi governa dopo un voto senza maggioranza, e come si dividono i ministeri. Tre strati: teorie su quali coalizioni si formano (minima vincente, connessa, Laver-Shepsle), modelli di contrattazione (Baron-Ferejohn) e la regolarità empirica sulla spartizione (legge di Gamson). Per il gioco è il passaggio fra risultato elettorale e governo, e la fonte di gran parte del "dramma" parlamentare.

## Regole e modelli

- **Minima vincente** (Riker 1962): si formano coalizioni che perdono la maggioranza togliendo un qualsiasi partito; variante "minimum winning" con il minor numero di seggi [INF].
- **Minima connessa vincente** (Axelrod 1970) e **minimo raggio ideologico** (Leiserson): fra le coalizioni vincenti, quelle di partiti contigui sull'asse sinistra-destra o con la minor distanza fra gli estremi [INF].
- **Allocazione dei portafogli** (Laver-Shepsle 1996): ogni ministro attua la posizione del proprio partito nel suo dicastero; un governo è stabile se nessuna maggioranza preferisce un'altra assegnazione di dicasteri; esistono "partiti forti" che stanno in ogni governo di equilibrio [INF].
- **Baron-Ferejohn** (APSR 1989, regola chiusa): a turno un proponente estratto a caso offre una spartizione, che passa a maggioranza; con n membri uguali e sconto δ, il proponente compra (n−1)/2 voti offrendo a ciascuno il loro valore di continuazione δ/n, e tiene `1 − δ(n−1)/(2n)`. Predice accordo immediato, coalizione minima vincente e forte premio al proponente [INF, derivazione standard].
- **Legge di Gamson**: ogni partito di coalizione ottiene una quota di ministeri proporzionale alla quota di seggi che porta alla maggioranza (fonti/druckman-warwick-2006-portfolio.md) [V].

## Evidenza

- **Gamson regge, i modelli a proponente no**: su 14 paesi dell'Europa occidentale, pesando i ministeri per importanza (indagine fra esperti), le quote di portafoglio rispecchiano le quote di seggi; il formateur non estrae il premio previsto dai modelli di contrattazione (fonti/druckman-warwick-2006-portfolio.md) [V].
- Regressione quota portafogli su quota seggi: r = 0,943; intercetta 0,069, pendenza 0,793, entrambe diverse da 0 e 1: i partiti piccoli stanno sopra la bisettrice, cioè ricevono un po' più del proporzionale (fonti/druckman-warwick-2006-portfolio.md) [V].
- Il formateur quasi sempre prende la presidenza del consiglio: il vantaggio è qualitativo più che quantitativo (fonti/druckman-warwick-2006-portfolio.md) [V].
- **Chi entra al governo**: lo strumento standard è il logit condizionale su tutte le coalizioni possibili (Martin-Stevenson, AJPS 45: 33-50, 2001), che stima il peso di essere minimi vincenti, del partito più grande, della distanza ideologica, del governo uscente; Glasgow, Golder e Golder (Political Analysis 20(2), 2012) mostrano che l'assunzione IIA del modello è violata [INF, articoli non catturati].
- In un ABM con formazione dei governi, le regole orientate all'ufficio formano governi più coesi ma più spesso sovradimensionati (fonti/plos-lehrer-schumacher-2018.md) [V].

## Parametri e dati

- ParlGov: circa 1600 governi (4000 partecipazioni di partito), 1000 elezioni, 1700 partiti, 37 democrazie UE/OCSE, 1900-2023 (fonti/parlgov-about.md) [V], rilascio 2024 in CC0 (fonti/parlgov-dataverse-2024-licenza.md) [V]. Da qui si stimano durata delle trattative, frequenza di governi di minoranza e sovradimensionati per paese [INF].
- Pesi di salienza dei ministeri: esistono da indagine fra esperti per 14 paesi (Warwick-Druckman 2001) (fonti/druckman-warwick-2006-portfolio.md) [V]; il dataset non è catturato [NON TROVATO].
- Pendenza di Gamson 0,79 e intercetta 0,07 come default per la spartizione (fonti/druckman-warwick-2006-portfolio.md) [V].

## Caso italiano 2026

La legge elettorale approvata in via definitiva dalla Camera l'8 ottobre 2026 (A.C. 2822-B) assegna un premio fisso di 70 seggi alla Camera e 35 al Senato alla lista o coalizione prima, solo se supera il 42% dei voti validi in entrambe le Camere; altrimenti tutto torna proporzionale (fonti/camera-dossier-ac2822-legge-elettorale-2026.md) [V]. Le coalizioni vanno dichiarate prima del voto con un unico candidato premier indicato nel programma (fonti/camera-dossier-ac2822-legge-elettorale-2026.md) [V]. Quindi in Italia la formazione post-elettorale serve solo quando il premio non scatta. Dettagli in [[sistemi-elettorali]].

## Costo computazionale

Enumerare tutte le coalizioni di P partiti costa `2^P`: con 12 partiti sono 4096 sottoinsiemi, ciascuno valutato in `O(P)`: trascurabile. Un logit condizionale su queste alternative è una softmax su 4096 punteggi [INF]. Baron-Ferejohn in forma chiusa è `O(1)` [INF].

## Cosa semplifichiamo

- Un solo asse ideologico per contiguità e distanza; le politiche dei ministeri dal secondo asse in poi [INF].
- Contrattazione in un solo round con probabilità di fallimento crescente col tempo, invece di un gioco ripetuto [INF].
- Nessun appoggio esterno strutturato: governo di minoranza come esito possibile con una probabilità tarata su ParlGov [INF].

## Per il nostro gioco

- Scegliere la coalizione con un logit su tutte le coalizioni possibili, punteggio = vincente (+), minima (+), distanza ideologica (−), partito più grande dentro (+), incumbent (+). I pesi si tarano su ParlGov.
- Spartire i ministeri con Gamson (pendenza ~0,8, bonus ai piccoli), dando al formateur la presidenza del consiglio. Evitare il premio al proponente di Baron-Ferejohn: l'evidenza lo smentisce.
- Rendere visibile la trattativa (durata in giorni, ministeri chiesti) è più interessante del modello stesso: ParlGov dà le durate reali.
- Aperto: chi gioca come leader di un partito minore deve poter negoziare i ministeri; serve una regola che parta da Gamson e si sposti col potere di ricatto.

## Collegamenti

- [[sistemi-elettorali]]: produce i seggi da cui parte tutto; in Italia il premio di maggioranza.
- [[abm-elettorali]]: regole Governator e risultati sulla rappresentanza.
- [[voto-spaziale]]: distanze ideologiche fra partiti.
- [[parlgov]]: dati per tarare.
- [[partiti-e-coalizioni]]: come i giochi esistenti trattano coalizioni e trattative.
- [[ministri-e-gabinetto]]: dove finiscono i ministeri spartiti con Gamson.
- [[parlamento-e-legislazione]]: la maggioranza che ne esce deve votare le leggi.

## Riferimenti

- **URL** https://faculty.wcas.northwestern.edu/jnd260/pub/Druckman%20Warwick%20EJPR%202006.pdf · [V]
  **perché conta qui** test più forte della legge di Gamson con ministeri pesati per importanza
  **cosa se ne prende** pendenza 0,79, intercetta 0,07, nessun premio al formateur
- **URL** https://parlgov.org/about/ · [V]
  **perché conta qui** base dati su governi, partiti ed elezioni di 37 democrazie
  **cosa se ne prende** i dati per tarare chi governa, quanto dura la trattativa, quanti governi di minoranza
- **URL** https://documenti.camera.it/leg19/dossier/Pdf/AC0469f.pdf · [V]
  **perché conta qui** regole della nuova legge elettorale italiana con premio e coalizioni pre-elettorali
  **cosa se ne prende** soglia 42%, premio 70/35, candidato premier di coalizione
- **URL** https://pmc.ncbi.nlm.nih.gov/articles/PMC5796695/ · [V]
  **perché conta qui** ABM con formazione dei governi
  **cosa se ne prende** l'effetto delle motivazioni di ufficio sulla forma dei governi
