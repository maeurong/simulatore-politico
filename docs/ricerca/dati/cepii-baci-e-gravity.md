---
tipo: dataset
fonti: [fonti/cepii-baci.md, fonti/cepii-gravity.md]
aggiornato: 2026-10-16
---
# CEPII: BACI e Gravity

Due dataset del CEPII. BACI: flussi commerciali bilaterali per prodotto, ricavati da Comtrade riconciliando le dichiarazioni di import ed export, che in pratica non coincidono quasi mai (fonti/cepii-baci.md) [V]. Gravity: le variabili del modello gravitazionale (distanze, contiguità, lingua, legami coloniali, accordi) più flussi bilaterali da FMI (DOTS), ONU (Comtrade) e BACI (fonti/cepii-gravity.md) [V]. Per il nostro gioco sono la via aperta ai dati del commercio.

## Licenza

Entrambi Etalab Open Licence 2.0 (fonti/cepii-baci.md, fonti/cepii-gravity.md) [V]: licenza aperta francese con obbligo di citare la fonte [INF, il testo della licenza non è stato catturato].

## Si può ridistribuire in un repo pubblico?

Sì, con attribuzione [V per la licenza]. Attenzione alla colonna dei flussi Comtrade dentro Gravity: è materiale di terzi, conviene escluderla e tenere solo i flussi BACI [INF].

## Per il nostro gioco

- Prendiamo da Gravity le variabili bilaterali che entrano in $t_{ij}$ e da BACI i flussi aggregati per calibrare il modello (vedi [[commercio-modello-gravitazionale]]).
- Sostituisce [[un-comtrade]] nel repo.

## Fonti

- **URL** https://www.cepii.fr/CEPII/en/bdd_modele/bdd_modele_item.asp?id=37 · [V]
  **perché conta qui** licenza e metodo di BACI
  **cosa se ne prende** Etalab 2.0, riconciliazione dei flussi Comtrade (fonti/cepii-baci.md)
- **URL** https://www.cepii.fr/CEPII/en/bdd_modele/bdd_modele_item.asp?id=8 · [V]
  **perché conta qui** variabili gravitazionali pronte
  **cosa se ne prende** Etalab 2.0, elenco variabili e fonti dei flussi (fonti/cepii-gravity.md)

## Collegamenti

- [[commercio-modello-gravitazionale]]
- [[un-comtrade]]
