---
tipo: sistema
fonti: [fonti/hoi4-wiki-production.md, fonti/hoi4-wiki-construction.md, fonti/hoi4-wiki-government.md, fonti/vic3-wiki-building.md, fonti/vic3-wiki-production-methods.md, fonti/vic3-wiki-market.md, fonti/eu4-wiki-trade.md, fonti/eu4-wiki-development.md, fonti/eu5-wiki-economy.md, fonti/terrainvicta-wiki-gdp.md, fonti/terrainvicta-wiki-nations.md, fonti/stellaris-dd370-4-0-cambiamenti.md, fonti/wikipedia-supreme-ruler-ultimate.md, fonti/gamepretty-shadow-empire-logistica.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Produzione industriale e risorse

Come i grand strategy rappresentano la capacità produttiva di un paese, le materie prime e la crescita. Lo spettro va dalla fabbrica come unità astratta (HOI4) agli edifici con conto economico (Victoria 3) fino al PIL come unico numero (Terra Invicta). La scelta del livello decide quanto costa calcolare un turno e quanto il giocatore capisce.

## Tre livelli di astrazione

| Livello | Gioco | Unità | Costo di calcolo |
|---|---|---|---|
| PIL aggregato | Terra Invicta | PIL e PIL pro capite per nazione | minimo |
| Capacità industriale | HOI4 | fabbriche civili/militari, cantieri | basso |
| Edifici con conto economico | Victoria 3, EU5 | edifici con metodi di produzione, salari, profitti | alto |

Costo di calcolo: [INF], dedotto dal numero di entità; Stellaris documenta che il calcolo per entità pesa a fine partita (fonti/stellaris-dd370-4-0-cambiamenti.md).

## PIL aggregato (Terra Invicta)

- PIL iniziale dai dati reali dello scenario (fonti/terrainvicta-wiki-gdp.md).
- La crescita arriva dalla priorità Economia: +3 PIL pro capite per completamento (nazione da 50 milioni), +1,5 per regione petrolifera, mineraria o economica, +0,5 per punto di governo, +1 per punto di istruzione. Il clima e la disuguaglianza lo erodono (fonti/terrainvicta-wiki-gdp.md).
- Surplus investibile = (PIL in miliardi)^0,35 al mese (fonti/terrainvicta-wiki-gdp.md).
- Le regioni con risorse alzano la crescita ma anche la disuguaglianza (+0,0001 per regione a ogni completamento dell'Economia) (fonti/terrainvicta-wiki-nations.md).

## Capacità industriale (HOI4)

- Fabbrica civile 4/giorno (costruzione, beni di consumo, commercio), militare 3,5 × efficienza, cantiere 2 (fonti/hoi4-wiki-production.md).
- Costi di costruzione per edificio (fabbrica civile 10.800, militare 7.200) e livello massimo 20 per i tre tipi di fabbrica (fonti/hoi4-wiki-construction.md).
- Risorse strategiche senza scorte; −5% per unità mancante; infrastruttura +15% per livello; la legge sul commercio decide quanto resta per l'interno (fonti/hoi4-wiki-production.md).
- I beni di consumo sottraggono fabbriche civili (fonti/hoi4-wiki-production.md); la stabilità al 100% ne riduce il fattore del 20% (fonti/hoi4-wiki-government.md). È il trade-off burro/cannoni ridotto all'osso [INF].

## Edifici con conto economico (Victoria 3, EU5)

- Victoria 3: ogni edificio ha un metodo di produzione che fissa input, output e professioni; produttività per addetto, salari, dividendi; economie di scala +1% per livello nello stato fino a +20% (fonti/vic3-wiki-production-methods.md, fonti/vic3-wiki-building.md).
- Victoria 3: settore costruzioni di 10 punti base, edifici da 100 a 800 punti, coda governativa e privata separate (fonti/vic3-wiki-building.md).
- EU5: ricchezza di una località = profitti delle materie prime (RGO) + profitti potenziali degli edifici; moltiplicata per il controllo diventa base imponibile (fonti/eu5-wiki-economy.md).
- EU4 (contrasto): valore di una provincia = beni prodotti × prezzo del bene, senza catene di input (fonti/eu4-wiki-trade.md); sviluppo comprato con punti monarca a costo crescente (fonti/eu4-wiki-development.md).

## Economia mondiale condivisa

- Supreme Ruler: le nazioni IA usano lo stesso sistema economico del giocatore; se nessuno produce abbastanza petrolio tutti ne soffrono (fonti/wikipedia-supreme-ruler-ultimate.md).
- Il rovescio, riportato da un giocatore: in guerra mondiale i beni industriali spariscono dal mercato globale, nessuno può più costruire industria e le guerre si bloccano (fonti/steam-recensioni-grand-strategy.md).
- Victoria 3: carenza quando gli acquisti sono il doppio delle vendite, con penalità che si propagano a valle (fonti/vic3-wiki-market.md).

## Trasporto delle risorse

- Shadow Empire: alcune risorse viaggiano solo con capacità di trasporto (camion, treni), altre basta che siano collegate (fonti/gamepretty-shadow-empire-logistica.md).
- HOI4: le risorse estratte salgono del 20% se nello stato c'è un hub o porto collegato alla capitale (fonti/hoi4-wiki-production.md).
- Victoria 3: l'accesso al mercato cala se l'infrastruttura non regge l'uso (fonti/vic3-wiki-market.md).

## Per il nostro gioco

- Prendiamo: PIL e settori aggregati come livello principale (Terra Invicta + Leontief), con la capacità industriale di HOI4 come lente sul settore manifatturiero e sulla difesa.
- Prendiamo: infrastruttura come moltiplicatore della produzione e dell'accesso ai mercati.
- Prendiamo: il trade-off fra beni di consumo e produzione bellica come leva esplicita.
- Evitiamo: economia per edificio a livello mondiale. Victoria 3 e Stellaris documentano il costo; per un gioco leggero non serve.
- Evitiamo: lo stallo di Supreme Ruler. Ogni bene critico ha un pavimento (produzione domestica minima, sostituti, razionamento) e il test di una guerra mondiale fa parte della verifica del modello [INF].
- Aperto: quanti settori. La sintesi del filone economico misura 15 settori × 21 paesi in 0,33 ms ([[motore-economico-raccomandato]]).

## Collegamenti

- [[hearts-of-iron-4]], [[victoria-3]], [[europa-universalis]], [[terra-invicta]], [[supreme-ruler]], [[shadow-empire]].
- [[mercati-e-pops]]: il lato mercato.
- [[input-output-leontief]]: settori aggregati.
- [[motore-economico-raccomandato]]: dove la produzione entra nel nucleo macro.
- [[militare-e-guerra]]: produzione bellica.

## Riferimenti

- **URL** https://hoi4.paradoxwikis.com/Production · [V] (snapshot 2026-08-28)
  **perche' conta qui** capacità industriale con efficienza e risorse
  **cosa se ne prende** −5% per unità di risorsa mancante, infrastruttura +15%/livello
- **URL** https://wiki.hoodedhorse.com/Terra_Invicta/GDP · [V] (snapshot 2026-10-10)
  **perche' conta qui** PIL reale come stato e crescita per priorità
  **cosa se ne prende** crescita per completamento e surplus ^0,35
- **URL** https://vic3.paradoxwikis.com/Building · [V] (snapshot 2026-08-24)
  **perche' conta qui** livello per edificio e suo costo
  **cosa se ne prende** il limite superiore di dettaglio da non superare
- **URL** https://en.wikipedia.org/wiki/Supreme_Ruler_Ultimate · [V]
  **perche' conta qui** economia condivisa con la IA
  **cosa se ne prende** il caso di stallo globale da testare
