---
tipo: sistema
fonti: [fonti/anno1800-wiki-rotte-commerciali.md, fonti/anno1800-wiki-bisogni.md, fonti/anno117-wiki-gioco.md, fonti/cs2-devdiary-traffico.md, fonti/cs2-devdiary-economia.md, fonti/cs2-steam-city-corner-4-prestazioni.md, fonti/wr-wiki-cittadini.md, fonti/glassbox-gamedeveloper-gdc2012.md, fonti/simcity2013-mcv-risposta-ia.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Logistica e trasporti

Come i gestionali spostano merci e persone: rotte disegnate dal giocatore, costo del percorso calcolato per agente, trasporto fisico di ogni unità. Per un gioco a scala nazionale conta capire quale livello di dettaglio rende la logistica una leva di governo (infrastrutture, commercio) senza diventare micro-gestione o un pozzo di CPU.

## Tre approcci

**1. Rotte tra nodi aggregati (Anno).** Ogni isola ha un magazzino unico. Le case sono servite se collegate via strada al porto (fonti/anno1800-wiki-bisogni.md). Fra isole si spostano merci con rotte multi-tappa e multi-bene, assegnando navi le cui stive limitano gli slot disponibili, con regole per tappa (aspetta il carico, getta il carico, aspetta lo scarico) (fonti/anno1800-wiki-rotte-commerciali.md). Tempi misurati fra regioni: 4-6,5 minuti per una nave, vuota o carica (fonti/anno1800-wiki-rotte-commerciali.md). In Anno 117 due province interdipendenti, con beni che si producono in una sola delle due, forzano il commercio interno (fonti/anno117-wiki-gioco.md).

**2. Costo del percorso per agente (Cities: Skylines II).** Il percorso minimizza un costo a quattro componenti: tempo, comfort, denaro (carburante, parcheggio, biglietto), comportamento (propensione a manovre rischiose). Il peso dipende dall'età: adolescenti → denaro, adulti → tempo, anziani → comfort (fonti/cs2-devdiary-traffico.md). I veicoli di servizio si assegnano guardando anche dove saranno dopo l'incarico in corso (fonti/cs2-devdiary-traffico.md). Le merci costano in proporzione a peso e distanza: le risorse pesanti conviene produrle in città o spostarle su treno o nave (fonti/cs2-devdiary-economia.md).

**3. Trasporto fisico di tutto (Workers & Resources).** Ogni cittadino cammina fino a 480 m, poi prova auto o mezzi pubblici; dopo 4 ore su un mezzo torna a casa d'ufficio (fonti/wr-wiki-cittadini.md). In GlassBox anche l'acqua nei tubi è un agente (fonti/glassbox-gamedeveloper-gdc2012.md).

## Il costo del pathfinding

- Cities: Skylines II, 2026: la simulazione rallenta con la popolazione proprio per le richieste di pathfinding; alcuni bug (cittadini bloccati negli edifici, troppe biciclette) le avevano moltiplicate (fonti/cs2-steam-city-corner-4-prestazioni.md).
- SimCity 2013: gli agenti prendevano sempre il percorso più breve, con ingorghi "inevitabili e illogici", corretti dopo l'uscita con una ritaratura (fonti/simcity2013-mcv-risposta-ia.md).

## Cosa dicono i giocatori

La logistica di Anno è il piacere del gioco ("un problema logistico dopo l'altro"), ma a fine partita le notifiche di rotta e di attacco si accumulano fino a sostituire la pianificazione (fonti/steam-recensioni-gestionali.md). Di Workers & Resources si criticano i viaggi senza ritorno e il bilanciamento manuale dei lavoratori per destinazione (fonti/steam-recensioni-gestionali.md). Di Cities: Skylines si loda il traffico come sfida di progettazione (fonti/steam-recensioni-gestionali.md).

## Per il nostro gioco

- **Prendiamo:** grafo di nodi aggregati (regioni, porti, hub) con magazzini, e rotte o corridoi con capacità, che il giocatore finanzia e vede. È il modello di Anno portato sulle infrastrutture nazionali. Il costo per unità di merce va pesato per massa e distanza (Cities: Skylines II), così la geografia economica emerge da sola.
- **Evitiamo:** pathfinding per singolo agente su scala paese (il collo di bottiglia dichiarato da Cities: Skylines II); notifiche per ogni rotta.
- **Aperto:** i tempi di trasporto vanno tenuti come ritardo nelle filiere o ridotti a un costo?

## Collegamenti

- [[commercio-modello-gravitazionale]]: le rotte verso altri paesi.
- [[territorio-e-urbanistica]]: dove stanno i nodi.
- [[cittadini-simulati]]: perché non un agente per persona.
- [[anno]], [[cities-skylines]], [[workers-and-resources-soviet-republic]], [[simcity-2013]].
