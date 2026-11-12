Type: grilling
Status: open
Blocked by: -
Assignee: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Dove vivono le impostazioni del giocatore (scala con CSS `zoom`, velocità come fattore `--k`, interruttore del movimento sopra `prefers-reduced-motion`) e cosa sopravvive alla partita? Un file del guscio Tauri o la memoria del webview; per giocatore o per partita; cosa succede alla scala quando la finestra si stringe sotto il tetto dei 1024 px utili.

Il posto nella navigazione c'è già: una voce accanto a Schema dello Stato ed Edicola, e nell'edicola ([Ossatura e navigazione](07-ossatura-e-navigazione.md)). Scala e movimento li hanno decisi [Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md) e [Fondamenta visive](08-fondamenta-visive.md); entrano con la fetta 11a ([Fette](10-fette.md)), quindi la risposta serve prima che quel piano si scriva. Il guscio resta passacarte senza logica né stato di partita ([Contratto del guscio](12-contratto-del-guscio.md)).

## Answer
