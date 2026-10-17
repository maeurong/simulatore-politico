---
tipo: gioco
fonti: [fonti/eu4-wiki-monarch-power.md, fonti/eu4-wiki-trade.md, fonti/eu4-wiki-stability.md, fonti/eu4-wiki-land-warfare.md, fonti/eu4-wiki-manpower.md, fonti/eu4-wiki-estates.md, fonti/eu4-wiki-diplomacy.md, fonti/eu4-wiki-development.md, fonti/eu4-wiki-economy.md, fonti/eu5-wiki-europa-universalis-v.md, fonti/eu5-wiki-population.md, fonti/eu5-wiki-market.md, fonti/eu5-wiki-economy.md, fonti/eu5-wiki-estate.md, fonti/eu5-wiki-warfare.md, fonti/eu5-wiki-military.md, fonti/eu5-wiki-country.md, fonti/eu5-wiki-government.md, fonti/eu5-wiki-parliament.md, fonti/wikipedia-europa-universalis-iv.md, fonti/wikipedia-europa-universalis-v.md, fonti/steam-eu4.md, fonti/steam-eu5.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Europa Universalis (IV e V)

La serie Paradox sull'età moderna. EU4 (2013) è il modello astratto: tre valute di "potere del monarca" pagano tutto. EU5 (novembre 2025) lo butta via e passa a popolazione, mercati, controllo territoriale ed estati, cioè al modello di Victoria portato nel Trecento. Il passaggio fra i due è il confronto più utile che abbiamo fra astrazione giocabile e simulazione credibile, con le reazioni dei giocatori a entrambi.

## Scala e loop

- EU4: scenari dall'11 novembre 1444 fino all'ultimo, 21 settembre 1792 (fonti/wikipedia-europa-universalis-iv.md). La data di fine non è nelle fonti catturate [NON TROVATO].
- EU5: dal 1° aprile 1337 al 1° gennaio 1837; quasi tutto si può automatizzare e lasciare alla IA, per concentrarsi su commercio o guerra (fonti/wikipedia-europa-universalis-v.md). Uscito il 4 novembre 2025, sviluppato da Paradox Tinto (fonti/eu5-wiki-europa-universalis-v.md).
- Ritmo: EU4 genera il potere del monarca all'inizio di ogni mese e risolve le battaglie in fasi di 3 giorni (fonti/eu4-wiki-monarch-power.md, fonti/eu4-wiki-land-warfare.md). EU5 calcola nascite e attrito mensili (fonti/eu5-wiki-population.md, fonti/eu5-wiki-military.md).
- Recensioni Steam: EU4 87% positive su 47.690, ma 28% sulle ultime 595 (fonti/steam-eu4.md). EU5 "Mixed", 19.862 positive e 9.996 negative in tutte le lingue [M] (`curl appreviews/3450310?json=1&language=all&num_per_page=0`, 2026-10-16).

## EU4: il modello astratto

- Potere del monarca in tre tipi (amministrativo, diplomatico, militare), da 2 a 20 al mese per tipo: +3 base, fino a +6 dal sovrano, +5 dal consigliere e altre voci minori; tetto 999 per tipo (fonti/eu4-wiki-monarch-power.md). Paga tecnologie, idee, stabilità, sviluppo, azioni militari.
- Stabilità da −3 a +3. A −3 non si può dichiarare guerra e si deve accettare una pace che valga almeno il 30% sotto il punteggio di guerra (fonti/eu4-wiki-stability.md).
- Manpower: 250 uomini per punto di sviluppo militare; ricarica da zero in 10 anni, cioè manpower massimo × (1 + velocità) / 120 al mese (fonti/eu4-wiki-manpower.md).
- Sviluppo: costo che sale del 3% ogni 10 punti oltre il decimo (fonti/eu4-wiki-development.md).
- Commercio: nodi collegati in un grafo a senso unico; il valore commerciale di una provincia = beni prodotti × prezzo; il potere commerciale relativo in ogni nodo decide chi trattiene e chi devia il valore (fonti/eu4-wiki-trade.md).
- Estati: influenza e lealtà; l'influenza sale di 0,5 per ogni 1% di terre controllate, fino a 40; la lealtà tende a un equilibrio di 30 (fonti/eu4-wiki-estates.md).
- Battaglia: fasi alterne di fuoco e urto di 3 giorni, un dado per lato a ogni fase; perdite base = 15 + 5 × (dado + bonus + differenza fra comandanti + pip offensivi − pip difensivi − terreno) (fonti/eu4-wiki-land-warfare.md).

## EU5: il modello simulativo

- Popolazione per località: ogni mese nascono pop × crescita / 12. La crescita è spinta da prosperità, cibo a buon prezzo, terra libera, scorte alimentari; frenata da carestia (−2,5%), saccheggio, stanchezza di guerra (−0,03% per punto) (fonti/eu5-wiki-population.md).
- Controllo: la capitale irradia prossimità (100%), i governatori l'80%, e cala con la distanza via terra e via mare. Controllo = prossimità × 0,75 + bonus (nucleo +20, città +5, edifici) − insoddisfazione dei pop. Esempio del wiki: 45 × 0,75 + 20 + 5 + 5 − 13 = 50 (fonti/eu5-wiki-country.md).
- Fisco: base imponibile = ricchezza della località × controllo; la base si divide fra le estati in proporzione al loro peso e ognuna è tassata con la propria aliquota (fonti/eu5-wiki-economy.md).
- Prezzi: tendono a un prezzo obiettivo = base × min((1 + R × stabilità prezzi) / (R + stabilità prezzi), massimo), con R = (1 + offerta effettiva) / (1 + domanda effettiva) (fonti/eu5-wiki-market.md). Il prezzo si muove verso l'obiettivo, non lo raggiunge di colpo.
- Commercio: profitto = prezzo import × quantità × efficienza − prezzo export × quantità × modificatori − manutenzione − pedaggi (fonti/eu5-wiki-market.md).
- Estati: potere percentuale distribuito per numero di pop e privilegi; soddisfazione sopra il 50% dà bonus, sotto penalità; sotto il 25% niente leve da quella estate (fonti/eu5-wiki-estate.md).
- Parlamento: convocabile ogni 5 anni, la questione si discute 180 giorni e passa secondo il potere delle estati a favore e contro (fonti/eu5-wiki-parliament.md).
- Gabinetto: 2 seggi iniziali; sostituire un ministro costa 5 di stabilità; il capo di gabinetto alza l'efficienza del 25% (fonti/eu5-wiki-government.md).
- Guerra: ogni punto di stanchezza costa −2% di morale, −1% di efficienza produttiva e commerciale, −0,03% di crescita; cala di 0,1 al mese in pace (fonti/eu5-wiki-warfare.md). Le leve sottraggono popolazione quando si alzano e la restituiscono quando si sciolgono (fonti/eu5-wiki-military.md).

## Cosa dicono i giocatori sul realismo

- EU4: "the economy (though very shallow and not how it behaved in real life - if you want that go and play EU5, it is way more realistic)" (fonti/steam-recensioni-grand-strategy.md).
- EU4: profondità che diventa "managing spreadsheets and waiting for numbers to change"; prestazioni che crollano verso il Seicento (fonti/steam-recensioni-grand-strategy.md).
- EU5, la recensione più votata (863 utili): "It's a nice simulator, but not a good game... what I seek here is an agency" (fonti/steam-recensioni-grand-strategy.md).
- EU5: "Europa Victoria Kings X", troppi sistemi senza un fuoco (fonti/steam-recensioni-grand-strategy.md).
- EU5 positivo: abbandona il "mana" per "living systems: people, infrastructure, politics, and trade"; "more of an actual simulation" (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo da EU5: base imponibile = ricchezza × controllo, con il controllo che cala con la distanza dalla capitale. È un modello compatto di capacità statale; si collega a [[burocrazia-corruzione-e-capacita-statale]].
- Prendiamo da EU5: prezzi che tendono a un obiettivo invece di saltarci. Dà inerzia senza codice in più.
- Prendiamo da EU5: l'automazione per sistema. Un governo vero delega; il giocatore sceglie su cosa mettere le mani.
- Evitiamo: il "mana" di EU4 come valuta universale. I giocatori lo vedono come astrazione da gioco da tavolo.
- Evitiamo: il rischio EU5 di simulatore senza agenzia. Ogni sistema deve offrire al giocatore una decisione leggibile, non solo un numero da guardare crescere.
- Aperto: quanta granularità territoriale. EU5 ragiona per località; per un governo moderno bastano probabilmente le regioni [INF].

## Collegamenti

- [[victoria-3]]: la fonte del modello a pop e mercati di EU5.
- [[mercati-e-pops]]: confronto fra le formule di prezzo.
- [[personaggi-fazioni-e-stabilita-interna]]: estati, parlamento, stabilità.
- [[parlamento-e-legislazione]]: pagina del filone politico.
- [[tempo-e-ritmo]]: tick mensile contro giornaliero.

## Riferimenti

- **URL** https://eu5.paradoxwikis.com/Country · [V] (catturata direttamente 2026-10-16)
  **perche' conta qui** controllo territoriale calcolato dalla prossimità alla capitale
  **cosa se ne prende** formula del controllo e suo uso come moltiplicatore fiscale
- **URL** https://eu5.paradoxwikis.com/Market · [V]
  **perche' conta qui** prezzo obiettivo con inerzia e profitto delle rotte
  **cosa se ne prende** prezzo che converge invece di saltare
- **URL** https://eu4.paradoxwikis.com/Monarch_power · [V] (snapshot Wayback 2026-09-29)
  **perche' conta qui** il modello astratto da cui EU5 si è allontanato
  **cosa se ne prende** il contro-esempio: valuta unica che paga tutto
- **URL** https://en.wikipedia.org/wiki/Europa_Universalis_V · [V]
  **perche' conta qui** date e automazione dei sistemi
  **cosa se ne prende** delega per sistema alla IA
- **URL** https://store.steampowered.com/appreviews/3450310 · [V]
  **perche' conta qui** reazione a un simulatore molto profondo
  **cosa se ne prende** "nice simulator, but not a good game": la profondità senza agenzia non basta
