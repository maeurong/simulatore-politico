---
tipo: gioco
fonti: [fonti/terrainvicta-wiki-nations.md, fonti/terrainvicta-wiki-gdp.md, fonti/terrainvicta-wiki-economy-priority.md, fonti/terrainvicta-wiki-military-priority.md, fonti/terrainvicta-wiki-public-opinion.md, fonti/terrainvicta-wiki-full-unification.md, fonti/wikipedia-terra-invicta.md, fonti/steam-terra-invicta.md, fonti/steam-recensioni-grand-strategy.md, fonti/terrainvicta-turnbasedlovers-1-0.md, fonti/terrainvicta-gameluster-recensione.md]
aggiornato: 2026-10-16
---
# Terra Invicta

Grand strategy di Pavonis Interactive (gli autori della mod Long War per XCOM), pubblicato da Hooded Horse. Il pretesto è un'invasione aliena; la parte che ci interessa è la Terra: nazioni reali con PIL, popolazione, disuguaglianza, coesione, disordini e un indice di democrazia, governate da chi ne controlla i "punti di controllo" e spende il surplus su priorità di investimento. È la cosa più vicina che esiste a un bilancio dello stato giocabile con formule pubbliche.

## Scala e loop

- Il giocatore guida una di sette fazioni con ideologie diverse; il gioco è diviso fra la simulazione geopolitica della Terra e la colonizzazione e guerra spaziale (fonti/wikipedia-terra-invicta.md).
- Scenari reali 2003, 2022, 2026 con PIL iniziali presi dai dati veri (fonti/terrainvicta-wiki-gdp.md).
- Ritmo: i punti di investimento sono mensili; i disordini si muovono di 0,25 al mese verso il valore di riposo; colpi di stato e rivoluzioni si controllano ogni quattro giorni (fonti/terrainvicta-wiki-gdp.md, fonti/terrainvicta-wiki-nations.md).
- Loop terrestre: i consiglieri (agenti) conquistano punti di controllo nelle nazioni → chi controlla i punti sceglie le priorità di spesa → le priorità muovono PIL, democrazia, coesione, disordini, forze armate → si federano le nazioni.
- Recensioni Steam, tutte le lingue: "Very Positive", 8.012 positive e 1.974 negative [M] (`curl appreviews/1176470?json=1&language=all&num_per_page=0`, 2026-10-16).

## Come modella la nazione

- Punti di investimento (IP) mensili = (PIL in miliardi)^0,35, ridotti dal numero di eserciti e flotte e dai disordini alti (fonti/terrainvicta-wiki-gdp.md).
- Le priorità (economia, welfare, ambiente, conoscenza, governo, unità, militare, oppressione, bottino e altre) sono secchi che si riempiono di IP; a secchio pieno scatta l'effetto, poi si ricomincia. Ogni detentore di punto di controllo assegna pesi 0-3 alle priorità (fonti/terrainvicta-wiki-nations.md).
- Bonus diversità: investire su più priorità di costruzione nazionale aumenta la resa delle altre (economia 50%, le altre 20%, proporzionale ai pesi) (fonti/terrainvicta-wiki-nations.md).
- Priorità Economia (nazione da 50 milioni): +3 PIL pro capite, +1,5 per regione petrolifera, mineraria o economica, +0,5 per punto di Governo, +1 per punto di Istruzione; per popolazioni diverse si scala (fonti/terrainvicta-wiki-gdp.md).
- Disuguaglianza 1-9: Economia la alza di 0,00015 per completamento, Bottino (corruzione) di 0,0025, Welfare la abbassa di 0,005. Oltre 9 l'eccesso diventa perdita di coesione e aumento dei disordini (fonti/terrainvicta-wiki-nations.md).
- Governo 0-10, da dittatura totalitaria a democrazia ideale: alto aumenta ricerca, riduce corruzione, aiuta l'unificazione; basso rende più efficaci oppressione ed esercito contro i disordini (fonti/terrainvicta-wiki-nations.md).
- Disordini 0-10, valore di riposo = 10,5 − 1 per ogni 10.000 di PIL pro capite − 1 per punto di coesione (più altri termini) (fonti/terrainvicta-wiki-nations.md).
- Colpo di stato: probabilità giornaliera = ((disordini × 2 − coesione − governo) / 10 + scontento delle élite) / 6000 (fonti/terrainvicta-wiki-nations.md).
- Numero di punti di controllo = max(1, min(round(PIL^0,18 / 1,09), 6)): le grandi economie sono più difficili da prendere (fonti/terrainvicta-wiki-nations.md).
- Opinione pubblica: per nazione, una percentuale per ciascuna delle sette ideologie; alimenta l'influenza della fazione e facilita le missioni dove la gente è d'accordo (fonti/terrainvicta-wiki-public-opinion.md).
- Unificazione pacifica: le nazioni hanno rivendicazioni e possono fondersi in blocchi (fonti/terrainvicta-wiki-full-unification.md).

## Cosa dicono i giocatori sul realismo

- Lodato: la simulazione della Terra traccia PIL, economie nazionali, disuguaglianza, clima e opinione pubblica, "detailed, sometimes eerily so" (fonti/terrainvicta-turnbasedlovers-1-0.md).
- Criticato: "Nations often feel inert unless a faction pokes them... Economic catastrophes rarely spill across borders. The model is impressive, but not always convincing" (fonti/terrainvicta-turnbasedlovers-1-0.md).
- Criticato: variazioni d'opinione che fanno perdere una nazione "hide behind tiny icons"; guerre civili quasi senza preavviso (fonti/terrainvicta-turnbasedlovers-1-0.md).
- Criticato: "a grinding micromanagement simulator", consiglieri da rimandare in missione all'infinito; "the design philosophy wildly swings between hardcore realism and illogical gameyness" (fonti/steam-recensioni-grand-strategy.md).
- Criticato: la IA delle nazioni "doesn't know what it's doing", le nazioni crescono lentamente senza intervento (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: il surplus economico come budget discrezionale allocato su priorità che si riempiono e scattano. È un modo leggibile di trasformare la legge di bilancio in gameplay, e si mappa sui ministeri.
- Prendiamo: le variabili nazionali compatte e le formule pubblicate (democrazia 0-10, coesione, disordini, disuguaglianza) come punto di partenza da calibrare con i dati veri.
- Prendiamo: il trade-off welfare/economia/bottino sulla disuguaglianza, con l'eccesso che si riversa sui disordini.
- Evitiamo: nazioni inerti e crisi che non attraversano i confini. Il commercio deve propagare gli shock ([[commercio-modello-gravitazionale]]).
- Evitiamo: segnali critici nascosti in icone minuscole.
- Aperto: governare uno stato dall'interno (noi) contro manovrarne molti dall'esterno (Terra Invicta). Le formule restano utili, il punto di vista cambia.

## Collegamenti

- [[personaggi-fazioni-e-stabilita-interna]]: disordini, coesione, colpi di stato.
- [[produzione-industriale-e-risorse]]: PIL e priorità come produzione aggregata.
- [[burocrazia-corruzione-e-capacita-statale]]: corruzione e bottino.
- [[ministri-e-gabinetto]]: priorità come portafogli ministeriali.

## Riferimenti

- **URL** https://wiki.hoodedhorse.com/Terra_Invicta/Nations · [V] (wiki ufficiale, snapshot Wayback 2026-10-12)
  **perche' conta qui** formule pubbliche di disordini, coesione, disuguaglianza, colpi di stato
  **cosa se ne prende** le variabili nazionali compatte e le loro interazioni
- **URL** https://wiki.hoodedhorse.com/Terra_Invicta/GDP · [V] (snapshot 2026-10-10)
  **perche' conta qui** surplus dal PIL con esponente 0,35
  **cosa se ne prende** budget discrezionale che scala meno che linearmente col PIL
- **URL** https://turnbasedlovers.com/review/terra-invicta-1-0-impressions/ · [V] (fonte di un altro agente)
  **perche' conta qui** critica del realismo della simulazione terrestre
  **cosa se ne prende** shock che devono attraversare i confini
- **URL** https://store.steampowered.com/appreviews/1176470 · [V]
  **perche' conta qui** micro-gestione come costo del realismo
  **cosa se ne prende** limitare le azioni ripetitive del giocatore
