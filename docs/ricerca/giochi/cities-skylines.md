---
tipo: gioco
fonti: [fonti/cs1-wiki-politiche.md, fonti/cs1-wiki-distretti.md, fonti/cs1-wiki-valore-terreno.md, fonti/cs1-wiki-problemi.md, fonti/cs2-devdiary-cittadini.md, fonti/cs2-devdiary-traffico.md, fonti/cs2-devdiary-servizi.md, fonti/cs2-devdiary-economia.md, fonti/cs2-steam-dichiarazione-prestazioni.md, fonti/cs2-steam-city-corner-4-prestazioni.md, fonti/cs2-steam-economia-2-0.md, fonti/cs2-steam-passaggio-iceflake.md, fonti/cs2-performance-paavo.md, fonti/steam-api-snapshot-gestionali.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Cities: Skylines (I e II)

City builder di Colossal Order per Paradox: il primo (2015) è lo standard del genere, il secondo (2023) è il caso di studio di cosa costa simulare ogni cittadino e renderizzare ogni dettaglio. Per noi conta su tre fronti: politiche per distretto, servizi con copertura simulata, e il flop tecnico della II come avvertimento sulle prestazioni.

## Scheda

| | Cities: Skylines | Cities: Skylines II |
|---|---|---|
| Uscita | 10/03/2015, Win/macOS/Linux | 24/10/2023, solo Windows |
| Recensioni Steam | 291.057, 93% positive | 92.163, 56% (Mixed) |
| Requisiti minimi | i7 930, 8 GB, GTS 450 1 GB, 4 GB disco | i7-6700K / Ryzen 5 2600X, 8 GB, GTX 970 4 GB, 60 GB disco |

Dati: (fonti/steam-api-snapshot-gestionali.md). Dal 2026 lo sviluppo della II è passato da Colossal Order a Iceflake Studios (fonti/cs2-steam-passaggio-iceflake.md).

## Scala e loop

Zonizzazione residenziale/commerciale/industriale/uffici, strade, servizi, trasporti; la città cresce se domanda, servizi e traffico reggono. Il governo è il sindaco: bilancio, tasse, politiche.

## Come modella la cosa

**Politiche per distretto (I).** Il giocatore dipinge distretti con un pennello e assegna a ciascuno politiche diverse; il pannello del distretto mostra felicità, specializzazione, popolazione, livelli edilizi e politiche attive (fonti/cs1-wiki-distretti.md). Ogni politica ha effetto e costo: "Uso dell'acqua" −15% consumi; "Trasporto pubblico gratuito" più passeggeri e nessun incasso; "Istruzione a scopo di lucro" −50% manutenzione scuole e meno felicità; "Licenza di pesca" +5% tasse residenziali e +5% crimine (fonti/cs1-wiki-politiche.md).

**Servizi (II).** Ogni servizio ha un effetto passivo (in raggio lungo le strade) e uno simulato (veicoli di pattuglia, cittadini che vanno al servizio). L'effetto passivo dipende da tre parametri: raggio in metri lungo la strada, capacità (quante persone raggiunge: in zone dense si esaurisce prima) e intensità, piena nel raggio e calante al bordo (fonti/cs2-devdiary-servizi.md). Un cursore di bilancio per servizio alza o abbassa insieme manutenzione ed efficienza di tutti gli edifici della categoria; i servizi si possono importare da città vicine (fonti/cs2-devdiary-servizi.md). Nella II i servizi non alzano più direttamente il valore del terreno: lo fanno solo indirettamente, perché la domanda soddisfatta alza gli affitti (fonti/cs2-devdiary-servizi.md).

**Cittadini (II).** Agenti individuali con percorso di vita, età, istruzione su 5 livelli, felicità = benessere + salute. Gli adolescenti scelgono fra studio e lavoro confrontando il reddito atteso; criminalità come scelta del singolo con bersaglio a probabilità; malattia con probabilità legata alla salute (fonti/cs2-devdiary-cittadini.md).

**Traffico (I → II).** Nella I il percorso era per prossimità in linea d'aria; nella II è un costo composto da tempo, comfort, denaro e comportamento, pesato per età: gli adolescenti badano al prezzo, gli adulti al tempo, gli anziani al comfort (fonti/cs2-devdiary-traffico.md).

**Economia (II).** Famiglie, imprese, città e "investitori astratti" con sorgenti e pozzi di denaro; tasse per settore, anche negative (sussidi); risorse con prezzo, peso (costo di trasporto) e ingombro (fonti/cs2-devdiary-economia.md; fonti/cs2-steam-economia-2-0.md). Dopo le critiche, "Economy 2.0" (giugno 2024) ha tolto i sussidi governativi, alzato la manutenzione dei servizi e reso a pagamento l'import di servizi, con l'obiettivo dichiarato di "meno automatismi invisibili" (fonti/cs2-steam-economia-2-0.md).

## Il flop tecnico della II

- **Alla prima uscita.** Colossal Order ammette di non aver raggiunto l'obiettivo di prestazioni; fissa come obiettivo 30 fps stabili, perché in un city builder "la città che cresce diventa comunque vincolata dalla CPU", e consiglia di spegnere profondità di campo, volumetria e illuminazione globale (fonti/cs2-steam-dichiarazione-prestazioni.md).
- **L'analisi esterna.** Su RTX 3080 il gioco è vincolato dalla GPU, caso insolito per il genere. Lo stack è Unity 2022.3 + DOTS/ECS con HDRP e un'interfaccia in HTML/JS (Coherent Gameface). Molti modelli non hanno LOD e il culling è solo per frustum: un fotogramma elabora 121 milioni di vertici. I denti dei cittadini (13 dentature renderizzate in un fotogramma, zero pixel visibili) sono il sintomo, non la causa (fonti/cs2-performance-paavo.md).
- **Aprile 2026.** Gli sviluppatori dicono che il 40-60% dei triangoli inviati alla GPU viene scartato perché troppo piccolo, e che la simulazione cede quando la popolazione cresce, per colpa delle richieste di pathfinding. Fra i rimedi allo studio: ridurre il numero di pedoni "per riflettere la dimensione della città" e rendere più locali le scelte di acquisto. Annunciano anche uno strumento di benchmark (fonti/cs2-steam-city-corner-4-prestazioni.md).
- **Giocatori, 2026.** Media di 38 fps su i7-14700K + RTX 5070 Ti; "5 fps" su PC non potenti; "pittore di città": soldi e felicità senza conseguenze (fonti/steam-recensioni-gestionali.md).

## Realismo: cosa dicono i giocatori

La I resta il riferimento per traffico e logistica, e le mod (TM:PE) ne sono parte integrante (fonti/steam-recensioni-gestionali.md). Della II si loda la grafica e si critica la simulazione "vuota", senza posta in gioco (fonti/steam-recensioni-gestionali.md).

## Per il nostro gioco

- **Prendiamo:** politiche dipinte su un territorio, con effetto e costo locali; copertura dei servizi a raggio-capacità-intensità, che è economica da calcolare e leggibile come mappa; un cursore di bilancio per ministero che sposta insieme costo ed efficienza.
- **Evitiamo:** il motore grafico come prima voce di costo; l'agente individuale come unità di base del pathfinding su scala nazionale; gli aiuti automatici invisibili che tolgono posta in gioco (la II li ha dovuti togliere).
- **Aperto:** quale aggregato sostituisce il cittadino-agente a scala paese (vedi [[cittadini-simulati]]).

## Collegamenti

- [[servizi-pubblici]]: modello raggio/capacità/intensità.
- [[cittadini-simulati]]: il costo dell'agente individuale, misurato dagli stessi sviluppatori.
- [[logistica-e-trasporti]]: costo del percorso a quattro componenti.
- [[feedback-visivo-dello-stato]]: infoview e icone-problema.
- [[prestazioni-su-hardware-modesto]]: lezione della II.
- [[godot]]: alternativa di motore valutata dall'agente tecnico.
- [[architettura-simulazione]]: DOTS/ECS e il costo del pathfinding.
