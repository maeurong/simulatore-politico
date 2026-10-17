---
tipo: dataset
fonti: [fonti/dataeuropa-eurobarometro-licenza.md, fonti/frontiers-torcal-christmann-2021-fiducia.md]
aggiornato: 2026-10-16
---
# Eurobarometro

Sondaggi della Commissione europea in tutti gli Stati membri, più volte l'anno dagli anni '70: fiducia nelle istituzioni nazionali ed europee, problemi più sentiti, giudizi sull'economia. Due canali con regole diverse: i volumi di tabelle della Commissione su data.europa.eu (licenza aperta) e i microdati distribuiti da GESIS.

## Contenuto

- Standard Eurobarometro con volumi di tabelle per paese, pubblicati dalla DG Comunicazione su data.europa.eu (es. EB 85, EB 92) (fonti/dataeuropa-eurobarometro-licenza.md) [V].
- Serie lunghe di fiducia in partiti e parlamento per paese, usate anche in studi longitudinali (es. Spagna 1997-2019) (fonti/frontiers-torcal-christmann-2021-fiducia.md) [V].

## Licenza

- Volumi recenti (es. EB 92): **CC BY 4.0** (fonti/dataeuropa-eurobarometro-licenza.md) [V].
- Volumi più vecchi (es. EB 85): "European Commission reuse notice", decisione 2011/833/UE, che consente il riuso con attribuzione (fonti/dataeuropa-eurobarometro-licenza.md) [V per l'etichetta; contenuto della decisione INF].
- Microdati tramite GESIS: download gratuito con registrazione, per ricerca e didattica, citazione obbligatoria [INF: la pagina GESIS risponde 403 ai client automatici].

## Ridistribuibile in un repo pubblico?

- Aggregati dai volumi della Commissione: **sì**, con attribuzione (fonti/dataeuropa-eurobarometro-licenza.md) [V].
- Microdati GESIS: **no** senza verificare le condizioni GESIS [INF].

## Per il nostro gioco

- Fonte consigliata per le serie di fiducia istituzionale da includere nel repo: licenza più aperta di [[ess]].
- Controllare la licenza volume per volume: cambia fra CC BY 4.0 e reuse notice.

## Collegamenti

- [[polarizzazione-e-fiducia-istituzionale]]: taratura della fiducia.
- [[ess]]: alternativa con microdati più ricchi ma licenza NC.
- [[eurostat]]: altra fonte della Commissione, pagina di un altro filone.

## Riferimenti

- **URL** https://data.europa.eu/api/hub/search/datasets/s2255_92_3_std92_eng · [V]
  **perché conta qui** campo licenza ufficiale dei volumi Eurobarometro
  **cosa se ne prende** CC BY 4.0 per i volumi recenti, reuse notice per i vecchi
- **URL** https://www.gesis.org/en/eurobarometer-data-service/search-data-access · [INF] (403 in sessione)
  **perché conta qui** canale dei microdati
  **cosa se ne prende** da verificare a mano prima di usare i microdati
