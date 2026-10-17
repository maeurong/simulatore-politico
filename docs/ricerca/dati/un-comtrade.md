---
tipo: dataset
fonti: [fonti/comtrade-licence-2012.md, fonti/un-shop-comtrade-redissemination.md, fonti/cepii-baci.md]
aggiornato: 2026-10-16
---
# UN Comtrade

Database delle Nazioni Unite dei flussi commerciali bilaterali per prodotto, dichiarati dai paesi. È la materia prima da cui derivano quasi tutti i dataset di commercio, BACI compreso (fonti/cepii-baci.md) [V]. Per il nostro gioco serve come origine dei dati, non come file da mettere nel repo.

## Licenza

- Accordo d'uso (pagina datata 7 agosto 2012): copia, navigazione o scaricamento automatico, ridistribuzione, pubblicazione o sfruttamento commerciale sono vietati senza permesso scritto delle Nazioni Unite (fonti/comtrade-licence-2012.md) [V].
- Pagina attuale del negozio ONU: ridistribuire significa riusare i dati così come sono, senza trasformazione, su altre piattaforme; il prerequisito è un abbonamento Premium Pro attivo (fonti/un-shop-comtrade-redissemination.md) [V].

Caveat: l'accordo catturato è del 2012; la politica vigente su uncomtrade.org non è stata trovata come documento [NON TROVATO]. La pagina del negozio è la fonte più recente catturata.

## Si può ridistribuire in un repo pubblico?

No, non i dati grezzi. Parametri derivati (coefficienti stimati, quote aggregate calcolate da noi) sono una trasformazione e non «i dati così come sono» [INF: da confermare dal filone legale].

## Per il nostro gioco

- Raccomandazione: usare BACI del CEPII, che parte da Comtrade, riconcilia le dichiarazioni di import ed export ed è sotto Etalab 2.0 (vedi [[cepii-baci-e-gravity]]).

## Fonti

- **URL** https://comtrade.un.org/licenseagreement.html · [V]
  **perché conta qui** è il testo che vieta la ridistribuzione
  **cosa se ne prende** divieto di ridistribuzione senza permesso scritto (fonti/comtrade-licence-2012.md)
- **URL** https://shop.un.org/de/node/55 · [V]
  **perché conta qui** condizioni attuali di ridistribuzione
  **cosa se ne prende** ridistribuzione solo per abbonati Premium Pro (fonti/un-shop-comtrade-redissemination.md)
- **URL** https://www.cepii.fr/CEPII/en/bdd_modele/bdd_modele_item.asp?id=37 · [V]
  **perché conta qui** l'alternativa aperta derivata da Comtrade
  **cosa se ne prende** BACI sotto Etalab 2.0 (fonti/cepii-baci.md)

## Collegamenti

- [[cepii-baci-e-gravity]]
- [[commercio-modello-gravitazionale]]
