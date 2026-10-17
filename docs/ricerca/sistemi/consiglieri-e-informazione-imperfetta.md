---
tipo: sistema
fonti: [fonti/bea-gdp-reliability-2008.md, fonti/worldbank-wgi.md, fonti/ti-cpi-how-calculated.md, fonti/hoi4-wiki-intelligence-agency.md, fonti/wikipedia-this-is-the-police.md, fonti/wikipedia-not-for-broadcast.md, fonti/meadows-leverage-points.md, fonti/democracy4-mod-policies.md, fonti/wikipedia-papers-please.md]
aggiornato: 2026-10-16
---
# Consiglieri e informazione imperfetta

Un capo di governo non vede lo stato del paese. Vede statistiche in ritardo, rapporti di ministri con interessi propri, sondaggi rumorosi. Simulare questa distanza rende il gioco più verosimile; e serve anche al game design, perché permette di nascondere un modello complesso senza renderlo arbitrario.

## Il dato reale è rumoroso e in ritardo
- PIL USA: le revisioni assolute medie dalla prima stima all'ultima disponibile sono poco più di 1 punto percentuale nel tasso di crescita annuo (1983-2006). Prima del 1960 erano circa 3 punti (fonti/bea-gdp-reliability-2008.md).
- Le stime trimestrali correnti indovinano la direzione del cambiamento nel 98% dei casi, ma l'accelerazione o il rallentamento solo nel 76% (fonti/bea-gdp-reliability-2008.md). [INF] Il giocatore sa quasi sempre se l'economia cresce, ma non di quanto.
- Indicatori di governance e corruzione: percezioni che restano indietro rispetto alle riforme; variazioni piccole non significative (fonti/worldbank-wgi.md, fonti/ti-cpi-how-calculated.md).
- Meadows: chi riceve in ritardo l'informazione sullo stato del sistema supera il bersaglio e poi lo manca dall'altra parte (fonti/meadows-leverage-points.md). Con dati in ritardo il giocatore oscilla, come i governi veri.

## Come lo fanno i giochi
- **Hearts of Iron IV**: l'intelligence è divisa in quattro campi (civile/economico, esercito, marina, aviazione), ognuno potenziato da un dipartimento dell'agenzia (+25%); le operazioni si sbloccano solo con una rete di spie sufficiente nel paese bersaglio (fonti/hoi4-wiki-intelligence-agency.md). [NON TROVATO] in questa fonte: come il livello di intelligence degrada i numeri mostrati.
- **This Is the Police**: le idee politiche degli agenti sono ignote finché il giocatore non ne fa un informatore, a un costo in fiducia e denaro (fonti/wikipedia-this-is-the-police.md). L'informazione si compra.
- **Not For Broadcast**: il giocatore è la fonte imperfetta; seleziona cosa va in onda (fonti/wikipedia-not-for-broadcast.md).
- **Papers, Please**: il giocatore vede solo documenti, che possono essere falsi; deve confrontarli con regole che cambiano ogni giorno (fonti/wikipedia-papers-please.md).
- **Democracy 4**: la competenza del ministro corregge tempi e costi di attuazione (fonti/democracy4-mod-policies.md). [INF] Non abbiamo trovato una distorsione del dato mostrato: la verità resta visibile.

## Proposta [INF]
- **Tre livelli**: stato vero (nascosto) → statistica ufficiale (ritardo + rumore + revisioni; qualità legata alla capacità statale di [[burocrazia-corruzione-e-capacita-statale]]) → rapporto del consigliere (statistica + bias del ministro: un ministro che vuole il budget esagera i problemi, uno sotto accusa li minimizza).
- **Rumore tarato sul reale**: errore della prima stima ≈ 1 punto sul PIL, revisione nei trimestri seguenti (fonti/bea-gdp-reliability-2008.md).
- **Comprare informazione**: istituto di statistica, servizi di intelligence, sondaggi più grandi. Ogni spesa riduce rumore o ritardo.
- **Previsioni dei consiglieri** con intervallo di confidenza: è il modo di mostrare gli effetti ritardati di [[ritardi-e-inerzia-delle-politiche]] senza svelare il modello.
- **Fiducia nei consiglieri** come variabile: chi ha sbagliato previsioni pesa meno.
- Da evitare: rumore senza struttura. Il giocatore deve poter imparare quale fonte mente, e perché.

## Collegamenti
- [[ministri-e-gabinetto]] (altro filone): i ministri come fonti con interessi.
- [[media-informazione-e-propaganda]]: i media come seconda fonte, anch'essa distorta.
- [[crisi-ed-eventi]]: l'evento arriva come rapporto, non come verità.
- [[suzerain]], [[shadow-president]]: consiglieri nei giochi di governo (pagine di altri filoni).
- [[sondaggi-ed-errore]]: il sondaggio come statistica rumorosa, con l'errore misurato.
- [[epidemie-sir-seir]]: casi rilevati contro casi veri.
- [[deterrenza-ed-escalation]]: lo scarto d'informazione come causa della guerra.
