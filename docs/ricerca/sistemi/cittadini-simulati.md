---
tipo: sistema
fonti: [fonti/cs2-devdiary-cittadini.md, fonti/cs2-devdiary-traffico.md, fonti/cs2-steam-city-corner-4-prestazioni.md, fonti/cs2-steam-dichiarazione-prestazioni.md, fonti/cs2-performance-paavo.md, fonti/glassbox-gamedeveloper-gdc2012.md, fonti/simcity2013-mcv-risposta-ia.md, fonti/simcity2013-pcgamer-dimensioni.md, fonti/wr-wiki-cittadini.md, fonti/anno1800-wiki-bisogni.md, fonti/anno117-wiki-popolazione.md, fonti/frostpunk2-wiki-delegati.md, fonti/frostpunk2-steam-dopo-beta.md, fonti/frostpunk2-pcgamer-fazioni.md, fonti/tropico-wiki-elezioni.md, fonti/ostriv-michet-blog.md, fonti/ostriv-faq.md, fonti/steam-recensioni-gestionali.md, fonti/steam-api-snapshot-gestionali.md]
aggiornato: 2026-10-16
---
# Cittadini simulati: agenti singoli o aggregati

Come i gestionali rappresentano la popolazione: un agente per persona, un agente senza identità, un aggregato per abitazione o per gruppo. Ogni scelta ha un costo di prestazioni e un costo di leggibilità documentati. Per un simulatore di governo a scala nazionale è la decisione architetturale numero uno.

## Lo spettro, con i casi documentati

| Modello | Gioco | Cosa si simula | Costo dichiarato o osservato |
|---|---|---|---|
| Aggregato per abitazione | Anno 1800/117 | Ogni casa ha capienza, bisogni soddisfatti in percentuale, attributi; il consumo dipende dalla capienza massima, non dagli abitanti presenti (fonti/anno1800-wiki-bisogni.md; fonti/anno117-wiki-popolazione.md) | Requisiti di Anno 117 alti per la grafica, non per la popolazione (fonti/steam-api-snapshot-gestionali.md) [INF] |
| Aggregato per gruppo + volti | Frostpunk 2 | Comunità e fazioni con opinione; 100 delegati con nome, mestiere e biografia generati (fonti/frostpunk2-pcgamer-fazioni.md; fonti/frostpunk2-wiki-delegati.md) | Perdita di legame con gli individui segnalata in beta, poi corretta con le "Zoom Stories" (fonti/frostpunk2-steam-dopo-beta.md) |
| Agenti senza identità | SimCity 2013 | Agenti semplici (10.000+) che portano risorse fra unità e non eseguono regole; niente casa o lavoro fissi, nessun nome (fonti/glassbox-gamedeveloper-gdc2012.md; fonti/simcity2013-mcv-risposta-ia.md) | Città limitate a 2 × 2 km "perché il gioco deve girare su computer normali" (fonti/simcity2013-pcgamer-dimensioni.md) |
| Agenti con identità | Cities: Skylines II | Percorso di vita, età, istruzione, felicità, scelta studio/lavoro, crimine, pathfinding pesato per età (fonti/cs2-devdiary-cittadini.md; fonti/cs2-devdiary-traffico.md) | La simulazione cede con la crescita della popolazione per le richieste di pathfinding; i rimedi allo studio riducono i pedoni (fonti/cs2-steam-city-corner-4-prestazioni.md) |
| Ogni cittadino, ogni passo | Workers & Resources | Nome, età, preferenze, raggio a piedi di 480 m, viaggi a tempo massimo (fonti/wr-wiki-cittadini.md) | Micro-gestione degli spostamenti e rallentamenti a fine partita, secondo i giocatori (fonti/steam-recensioni-gestionali.md) |
| Ogni cittadino vota | Tropico 6 | Felicità personale + rapporti di fazione decidono il voto (fonti/tropico-wiki-elezioni.md) | [NON TROVATO] |
| Famiglie individuali, motore su misura | Ostriv | Famiglie con denaro, tasse per professione (fonti/ostriv-michet-blog.md) | 2 GB di RAM; "con la crescita della città servono più risorse" (fonti/steam-api-snapshot-gestionali.md; fonti/ostriv-faq.md) |

## Cosa dicono gli sviluppatori sul costo

- **Maxis, 2013**: casa e lavoro fissi tolti "per aumentare le prestazioni e avere più Sim"; "il costo di quel micro-dettaglio non rendeva più ricco il macro" (fonti/simcity2013-mcv-risposta-ia.md).
- **Colossal Order, 2023**: in un city builder la città che cresce "diventa inevitabilmente vincolata dalla CPU"; la simulazione è tarata su 30 fps (fonti/cs2-steam-dichiarazione-prestazioni.md). La II non ha più limiti fissi agli agenti, che c'erano nella I; "gli unici limiti sono quelli dell'hardware" (fonti/cs2-devdiary-traffico.md).
- **Iceflake/Paradox, aprile 2026**: i problemi di simulazione "emergono solo oltre una certa dimensione della città" e dipendono dal pathfinding dei cittadini. Le soluzioni allo studio sono acquisti più locali e meno pedoni "per riflettere la dimensione della città" (fonti/cs2-steam-city-corner-4-prestazioni.md).
- Sul lato GPU l'agente costa anche da disegnare: modelli dei personaggi senza LOD in Cities: Skylines II (fonti/cs2-performance-paavo.md).

## Cosa dicono i giocatori

L'agente dà causalità leggibile: in W&R "c'è sempre una ragione simulata" mentre Skylines "aggiusta per bilanciamento o prestazioni". Ma dà anche comportamenti "folli" se non lo si sorveglia (fonti/steam-recensioni-gestionali.md). L'aggregato toglie attaccamento: in Frostpunk 2 la beta l'ha mostrato (fonti/frostpunk2-steam-dopo-beta.md), e le recensioni lamentano crisi "senza spiegazione" (fonti/steam-recensioni-gestionali.md).

## Per il nostro gioco

**Raccomandazione (da confermare, non decisa):** modello aggregato come motore e individui come finestra.
- Il motore lavora per gruppi (fascia sociale × territorio, con stato e bisogni come contenitori in stile GlassBox o case di Anno), perché a scala nazionale nessun gioco del filone regge l'agente individuale e i due che ci hanno provato (SimCity 2013, Cities: Skylines II) hanno dovuto ridurre gli agenti o la mappa.
- La rappresentazione mostra individui campionati dal gruppo (i delegati con biografia di Frostpunk 2, il percorso di vita e Chirper di Cities: Skylines II, la tomba visitata di Ostriv) per dare volti e causalità senza simularli tutti.
- L'individuo campionato deve essere coerente con l'aggregato: i suoi bisogni e il suo voto si derivano dai valori del gruppo, non da una simulazione propria [INF].
- In tensione con [[elettorato-e-gruppi-di-voto]]: là si propongono migliaia di elettori campione con stato proprio (reddito, appartenenze, posizione mossa da [[opinion-dynamics]] e votata con [[voto-spaziale]]); qui l'individuo campionato non ha stato proprio e deriva dal gruppo. Le fonti non decidono: è la domanda lasciata aperta in [[architettura-simulazione]].
- Deciso il 17/10/2026 ([[decisioni]]): gruppi aggregati come motore; l'elettore campione non ha stato proprio e si ricava dal sondaggio.

## Collegamenti

- [[elettorato-e-gruppi-di-voto]]: i gruppi aggregati come elettori.
- [[prestazioni-su-hardware-modesto]]: costo CPU della simulazione.
- [[feedback-visivo-dello-stato]]: individui come finestra sullo stato.
- [[simcity-2013]], [[cities-skylines]], [[workers-and-resources-soviet-republic]], [[anno]], [[frostpunk]], [[ostriv]], [[tropico-6]].
- [[architettura-simulazione]]: dove l'aggregato vive nel motore (pagina dell'agente tecnico).
- [[opinion-dynamics]]: la dinamica delle opinioni gira su gruppi o su campioni, secondo come si scioglie la tensione sopra.
- [[modelli-agent-based-economici]]: lo stesso dilemma agente/aggregato sul lato economico, con i costi di BeforeIT.
- [[demografia-e-migrazioni]]: la piramide per coorti, l'aggregato demografico su cui poggiano i gruppi.
