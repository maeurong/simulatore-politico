---
tipo: gioco
fonti: [fonti/wikipedia-supreme-ruler-2020.md, fonti/wikipedia-supreme-ruler-ultimate.md, fonti/steam-supreme-ruler-ultimate.md, fonti/steam-supreme-ruler-2030.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Supreme Ruler (Ultimate, 2030)

Serie di BattleGoat Studios: il mondo reale, qualunque paese, economia completa e guerra a battaglioni, in tempo reale con pausa. È il precedente più diretto del nostro progetto per ambizione (governare un paese moderno in tutte le sue parti) e anche per i difetti: un'economia globale condivisa con la IA che si inceppa, un'interfaccia a centinaia di cursori, ministri che aiutano ma non bastano. Le meccaniche sono documentate poco fuori dal gioco.

## Scala e loop

- Supreme Ruler 2020 (2008): oltre 250 regioni giocabili; il giocatore controlla economia, forze armate, ricerca, spesa pubblica, spionaggio con satelliti e diplomazia (fonti/wikipedia-supreme-ruler-2020.md).
- Tempo reale con pausa e velocità variabile; unità militari a livello di battaglione, controllabili singolarmente o in gruppo (fonti/wikipedia-supreme-ruler-2020.md, fonti/wikipedia-supreme-ruler-ultimate.md).
- Supreme Ruler Ultimate (2014) raccoglie i titoli precedenti; sandbox con avvio dal 1914 al 2020, alcuni con divisioni sub-nazionali (stati USA, regioni europee) (fonti/wikipedia-supreme-ruler-ultimate.md). Una recensione citata dallo store parla di partite dal 1936 al 2070 e oltre con una sola nazione (fonti/steam-supreme-ruler-ultimate.md).
- Supreme Ruler 2030: versione aggiornata e rimasterizzata, motore a 64 bit DirectX 11, fino a 15 giocatori in rete (fonti/steam-supreme-ruler-2030.md).
- Recensioni Steam, tutte le lingue: Ultimate "Mostly Positive", 968 positive e 290 negative; 2030 "Mixed", 361 e 196 [M] (`curl appreviews/<appid>?json=1&language=all&num_per_page=0`, 2026-10-16).

## Come modella lo stato

- Economia dettagliata con statistiche su sotto e sovra-occupazione e risorse naturali; le nazioni IA usano lo stesso sistema del giocatore, quindi se nessuno produce abbastanza petrolio tutti ne soffrono (fonti/wikipedia-supreme-ruler-ultimate.md).
- "Manage all aspects of your country, right down to financial and supply chain needs. Tanks cost money and people expect to be fed." Se non si soddisfano i bisogni della popolazione gli oppositori rimuovono il giocatore dal potere o intere regioni dichiarano l'indipendenza (fonti/steam-supreme-ruler-2030.md).
- Ministri di gabinetto con un sistema di priorità e una casella di posta interna al gioco; il giocatore sceglie quanto delegare (fonti/wikipedia-supreme-ruler-2020.md, fonti/steam-supreme-ruler-2030.md).
- Meteo integrato che influenza le battaglie; commercio di armamenti come fonte di entrate (fonti/steam-supreme-ruler-2030.md).
- Formule di approvazione, spesa sociale, PIL: non documentate in fonti pubbliche trovate [NON TROVATO]. Il manuale ufficiale non è stato reperito.

## Cosa dicono i giocatori sul realismo

- "The economy is broken. As soon as a world war starts, industrial goods dry up... This shortage is global": nessuno può più costruire industria né importare, e le guerre si bloccano (fonti/steam-recensioni-grand-strategy.md).
- 2030: relazioni internazionali irrealistiche, la IA ignora i patti di mutua difesa, le sanzioni non hanno quasi effetto (fonti/steam-recensioni-grand-strategy.md).
- Interfaccia: "Windows proliferate like metastasies. Sliders—dozens, hundreds"; si può regolare il budget dell'istruzione di una provincia canadese (fonti/steam-recensioni-grand-strategy.md).
- Lodato: la sensazione di scala da superpotenza; "the king of this genre... the best geo political game out there" (fonti/steam-recensioni-grand-strategy.md).
- Diplomazia giudicata povera e senza alleanze multi-nazione (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: la stessa economia per giocatore e IA. È onesto e fa sentire il mondo condiviso.
- Prendiamo: ministri con priorità come strumento di delega, già indicato anche da [[europa-universalis]].
- Evitiamo: un mercato globale senza ammortizzatori. Lo stallo descritto dai giocatori è il caso limite da testare: in guerra mondiale la domanda di beni industriali non deve azzerare la capacità di costruire. Serve un pavimento o una sostituzione (produzione domestica, razionamento) [INF].
- Evitiamo: centinaia di cursori. Il problema non è la profondità ma l'assenza di livelli: aggregato per default, dettaglio a richiesta.
- Aperto: il tempo reale. Supreme Ruler è un RTS con pausa; per un governo il ritmo di [[tempo-e-ritmo]] è una scelta da fare.

## Collegamenti

- [[power-and-revolution]]: l'altro simulatore geopolitico del mondo reale, nel filone dei giochi politici.
- [[hearts-of-iron-4]]: guerra moderna con astrazione più alta.
- [[produzione-industriale-e-risorse]]: il caso dello stallo industriale.
- [[ministri-e-gabinetto]]: delega ai ministri.

## Riferimenti

- **URL** https://en.wikipedia.org/wiki/Supreme_Ruler_Ultimate · [V]
  **perche' conta qui** unica descrizione pubblica del modello economico condiviso con la IA
  **cosa se ne prende** stessa economia per tutti, scarsità globale
- **URL** https://store.steampowered.com/app/2093410/ · [V]
  **perche' conta qui** dichiarazione di design dell'editore
  **cosa se ne prende** bisogni della popolazione che causano caduta del governo o secessioni
- **URL** https://store.steampowered.com/appreviews/314980 · [V]
  **perche' conta qui** il caso di stallo dell'economia globale in guerra
  **cosa se ne prende** il test di regressione da prevedere
- **URL** https://en.wikipedia.org/wiki/Supreme_Ruler_2020 · [V]
  **perche' conta qui** ministri con priorità e posta interna
  **cosa se ne prende** la delega come meccanica
