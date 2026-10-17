---
tipo: gioco
fonti: [fonti/hoi4-wiki-production.md, fonti/hoi4-wiki-supply.md, fonti/hoi4-wiki-land-battle.md, fonti/hoi4-wiki-government.md, fonti/hoi4-wiki-manpower.md, fonti/hoi4-wiki-battle-plan.md, fonti/hoi4-wiki-diplomacy.md, fonti/hoi4-wiki-construction.md, fonti/hoi4-wiki-land-doctrine.md, fonti/wikipedia-hearts-of-iron-iv.md, fonti/steam-hoi4.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Hearts of Iron IV

Grand strategy di Paradox sulla Seconda guerra mondiale, a tempo continuo. È il riferimento per come si modella un'economia di guerra con poche variabili leggibili: fabbriche come unità di capacità, efficienza che cresce col tempo, logistica come capacità di carico per provincia. Per noi conta il ministero della Difesa e quello dell'Industria, non la politica interna, che qui è sottile.

## Scala e loop

- Start il 1° gennaio 1936 o il 14 agosto 1939; si gioca qualunque nazione (fonti/wikipedia-hearts-of-iron-iv.md).
- Il combattimento terrestre si risolve a turni di un'ora (fonti/hoi4-wiki-land-battle.md). Il potere politico matura al giorno, +2 base (fonti/hoi4-wiki-government.md). Il gioco quindi gira a tick orario con aggregazioni giornaliere [INF].
- Loop: costruisci fabbriche civili → convertile in fabbriche militari e cantieri → produci equipaggiamento → riempi divisioni con manpower → disegna fronti e piani di battaglia → la IA esegue (fonti/hoi4-wiki-battle-plan.md).
- Recensioni Steam: 90% positive su 123.511, 78% sulle 467 degli ultimi 30 giorni al 2026-10-16 (fonti/steam-hoi4.md).

## Come modella l'industria

- Tre tipi di fabbrica. Civile: 4 di capacità al giorno (5 se alimentata), serve a costruire, commerciare e produrre beni di consumo. Militare: 3,5 al giorno (4,5 alimentata), moltiplicata per l'efficienza produttiva e per i modificatori di output, applicati indipendentemente. Cantiere: 2 al giorno (2,5), senza efficienza (fonti/hoi4-wiki-production.md).
- Esempio dal wiki: 4,5 × 0,5 × (1 + 0,1 − 0,218) = 1,9845 al giorno per fabbrica (fonti/hoi4-wiki-production.md).
- Efficienza: parte al 10%, tetto 50% senza modificatori. Crescita giornaliera = 0,001 × tetto² / efficienza corrente; tempo per arrivare a un livello t = 500 × (efficienza/tetto)², quindi al massimo 500 giorni per toccare il tetto (fonti/hoi4-wiki-production.md).
- Cambiare prodotto costa efficienza: si conserva il 90% (variante dello stesso modello), 70% (stesso telaio), 30%, 20%, 10% (qualunque altro cambio). Formula: efficienza nuova = CPE × (RET + (1 − RET) × BON), con il 10% base come pavimento (fonti/hoi4-wiki-production.md).
- Risorse strategiche: sette (fonti/hoi4-wiki-production.md). Non si accumulano, vanno dritte in produzione e l'eccesso si perde. La quota disponibile per l'interno dipende dalla legge sul commercio. Ogni unità mancante costa −5% di efficienza alla linea, fino a −100%, applicato prima alle linee a priorità più bassa (fonti/hoi4-wiki-production.md).
- Infrastruttura: +15% di risorse estratte per livello, +20% se nello stato c'è un hub o un porto collegato alla capitale (fonti/hoi4-wiki-production.md).
- Costi di costruzione per edificio (fabbrica civile 10.800, militare 7.200, cantiere 6.400, infrastruttura 6.000) in tabella (fonti/hoi4-wiki-construction.md).
- Licenze di produzione: costo in fabbriche civili che vanno al licenziante; −25% di output se la licenza viene dalla propria fazione, altro −10% se da fuori (fonti/hoi4-wiki-production.md).

## Come modella la logistica (dalla 1.11)

- Il rifornimento non si produce: in ogni provincia è una "capacità di supporto", un limite morbido di ammassamento. Tre fonti: hub, aerea, statale. Si applicano in quest'ordine: prima statale, poi hub, infine aerea (fonti/hoi4-wiki-supply.md).
- Capacità di un hub: ferrovia di livello 1 verso la capitale = 15, +5 per livello fino a 35; porto 8, +3 per livello fino a 35; l'hub della capitale vale 5 più il numero di fabbriche e cantieri (fonti/hoi4-wiki-supply.md).
- Il rifornimento cala con la distanza in province dall'hub, modulato da livello ferroviario, infrastruttura, terreno, meteo e motorizzazione, che consuma camion dalle scorte (fonti/hoi4-wiki-supply.md).
- Sotto il pieno rifornimento calano velocità, organizzazione, attacco (fino a −50% in difesa, −20% in attacco); sotto il 35% scatta l'attrito. Una divisione accumula fino al 150% come scorta, 2-3 giorni di autonomia. Isolata dalla capitale non riceve rinforzi anche se rifornita in loco (fonti/hoi4-wiki-supply.md).

## Come modella la battaglia

- Larghezza di fronte per terreno; i battaglioni in eccesso finiscono in riserva, ogni ora una riserva ha il 2% base di entrare (fonti/hoi4-wiki-land-battle.md).
- Numero di attacchi = round(attacco / 10), difese = round(difesa / 10). Probabilità di colpire 10% finché il bersaglio ha difese, 40% dopo. Danno: dado da 2 per i punti forza, da 4 per l'organizzazione, moltiplicati per 0,06 e 0,053. Il corazzato non perforato porta il dado d'organizzazione a 6 (fonti/hoi4-wiki-land-battle.md).
- Esempio del wiki: un corazzato con 1000 di attacco soft contro una fanteria da 500 di difesa fa circa 4,17 danni di organizzazione all'ora; la battaglia dura circa mezza giornata (fonti/hoi4-wiki-land-battle.md).
- Le dottrine sono alberi di bonus fissi (fonti/hoi4-wiki-land-doctrine.md).

## Stato interno

- Potere politico: +2 al giorno, accumulabile fra −500 e 2000; modulato dalla stabilità (−50% a 0%, +20% a 100%) (fonti/hoi4-wiki-government.md).
- Stabilità 0-100%: sopra il 50% dà fino a +20% di output industriale; sotto, fino a −50%. Essere in guerra toglie 20 punti; la popolarità dell'ideologia al potere ne dà fino a 15 (fonti/hoi4-wiki-government.md).
- Supporto alla guerra 0-100%: velocità di mobilitazione da +30% a −50%, limite di resa −30% al minimo. Guerra offensiva −20%, difensiva +20% (fonti/hoi4-wiki-government.md).
- Manpower: popolazione reclutabile per stato, rapporto fissato dalle leggi di coscrizione; l'addestramento è limitato al 75% del manpower al fronte o a 100.000 (fonti/hoi4-wiki-manpower.md).

## Cosa dicono i giocatori sul realismo

- Lodato: la scala della produzione e delle linee di rifornimento; "realistic supply line" con la sola riserva che tutto deve collegarsi alla capitale (fonti/steam-recensioni-grand-strategy.md).
- Lodato: il passaggio da HOI3 a battle planner e linee di produzione ha tolto la micro-gestione mantenendo profondità (fonti/steam-recensioni-grand-strategy.md).
- Criticato: politica interna appiattita in scelte binarie di focus; "map simulator"; le meccaniche aggiunte in dieci anni sono ostacoli attorno al "dipingere la mappa", non un simulatore di guerra più profondo (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: la fabbrica come unità di capacità con efficienza che cresce e si perde al cambio di produzione. È un modello di riconversione industriale che si spiega in una riga e produce scelte vere.
- Prendiamo: la logistica come capacità per nodo che cala con la distanza, non come merce. Si calcola su un grafo piccolo, si visualizza bene.
- Prendiamo: stabilità e supporto alla guerra come due indicatori distinti, con effetti lineari attorno al 50%.
- Evitiamo: il dado orario per battaglia. In un gestionale di governo il giocatore non vede le battaglie; basta un esito aggregato tipo [[lanchester-e-modelli-di-combattimento-aggregati]].
- Evitiamo: il focus tree scriptato, che i giocatori indicano come la parte meno realistica.
- Aperto: quanto della guerra delegare alla IA. HOI4 lo fa con i piani di battaglia; EU5 lo fa con l'automazione di interi sistemi ([[europa-universalis]]).

## Collegamenti

- [[militare-e-guerra]]: produzione, logistica, fronti nel dettaglio comparato.
- [[produzione-industriale-e-risorse]]: il modello fabbrica-efficienza confrontato con Victoria 3.
- [[logistica-e-trasporti]]: pagina di un altro filone sul trasporto civile.
- [[lanchester-e-modelli-di-combattimento-aggregati]]: alternativa aggregata al combattimento orario.
- [[supreme-ruler]]: l'altro modo di fare la guerra moderna, a battaglioni in tempo reale.

## Riferimenti

- **URL** https://hoi4.paradoxwikis.com/Production · [V] (snapshot Wayback 2026-08-28)
  **perche' conta qui** formule esplicite di output ed efficienza industriale
  **cosa se ne prende** crescita 0,001 × tetto²/eff e ritenzione al cambio di linea
- **URL** https://hoi4.paradoxwikis.com/Supply · [V] (snapshot 2026-08-28)
  **perche' conta qui** logistica come capacità per provincia, non merce
  **cosa se ne prende** capacità hub 15-35 per livello ferroviario, decadimento con la distanza, soglia 35% per l'attrito
- **URL** https://hoi4.paradoxwikis.com/Land_battle · [V] (snapshot 2026-09-01)
  **perche' conta qui** quanto costa una battaglia simulata a dado orario
  **cosa se ne prende** il contro-esempio: troppo fine per noi
- **URL** https://hoi4.paradoxwikis.com/War_support · [V] (pagina Government, snapshot 2026-05-19)
  **perche' conta qui** stabilità e supporto alla guerra come leve con effetti lineari
  **cosa se ne prende** soglia neutra al 50%, −20 punti di stabilità in guerra
- **URL** https://store.steampowered.com/appreviews/394360 · [V]
  **perche' conta qui** cosa i giocatori chiamano realistico o no
  **cosa se ne prende** logistica lodata, politica interna e focus tree criticati
