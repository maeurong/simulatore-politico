---
tipo: dataset
fonti: [fonti/acled-terms.md, fonti/acled-eula.md, fonti/acled-content-usage.md]
aggiornato: 2026-10-16
---
# ACLED: Armed Conflict Location & Event Data

Dati disaggregati su eventi di conflitto e disordini (scontri, proteste, violenza contro civili), georeferenziati e aggiornati di continuo. Per granularità sarebbero perfetti per i disordini interni. La licenza però li rende **inutilizzabili** per un gioco con repo pubblico: niente ridistribuzione, niente uso commerciale senza licenza aziendale, solo derivati "trasformativi" non invertibili.

## Termini d'uso

- Tre documenti: End User License Agreement, Content Usage Terms, Attribution Policy. Tutti i contenuti appartengono ad ACLED (fonti/acled-terms.md).
- ACLED Analysis, Inc. è una società del Wisconsin (fonti/acled-eula.md).
- **Licenza concessa:** gratuita, non esclusiva, non trasferibile, non sublicenziabile, solo per **scopi non commerciali**. All'esterno si possono pubblicare solo materiali **trasformativi**, da cui non si possa ricostruire il dato. Non basta integrare, estrarre, riorganizzare o mostrare i dati in un proprio cruscotto (fonti/acled-eula.md).
- Gli enti commerciali non possono accedere senza una licenza aziendale; le amministrazioni pubbliche richiedono una licenza per il settore pubblico (fonti/acled-eula.md).
- Vietato dare accesso diretto ai contenuti, condividere le credenziali, fare scraping; si scarica solo via API o piattaforma (fonti/acled-eula.md).
- Vietato usare i dati per creare prodotti o dataset che facciano concorrenza o sostituiscano quelli di ACLED, o per benchmark di prodotti simili (fonti/acled-content-usage.md).
- Vietato addestrare o migliorare sistemi di IA/ML che sostituiscano ACLED o diano accesso ai suoi dati, anche per uso accademico o sperimentale (fonti/acled-eula.md, fonti/acled-content-usage.md).
- ACLED può sospendere la licenza o chiedere di ritirare materiali già pubblicati, a sua discrezione (fonti/acled-eula.md). I termini cambiano con un semplice aggiornamento della data "Last Updated" (fonti/acled-eula.md).

## Perché non lo usiamo

- Un repo pubblico con dati ACLED dentro equivale a dare accesso diretto ai contenuti: vietato [INF dalla lettura dell'EULA].
- Un gioco venduto è uso commerciale: serve la licenza aziendale [INF]. Il gioco è gratuito ([[decisioni]]), ma l'esclusione resta: la vieta già il divieto di ridistribuzione.
- Anche usarlo solo per tarare parametri, se poi i parametri finiscono pubblicati, richiede di dimostrare che sono trasformativi e non invertibili. Il rischio di ritiro a discrezione resta [INF].
- Vale anche per gli strumenti di sviluppo: niente pipeline che passino dati ACLED a modelli di IA [INF].

## Per il nostro gioco

- **Non usiamo** ACLED, nemmeno in sviluppo, salvo una licenza scritta.
- **Usiamo invece** UCDP (CC BY 4.0), che copre la violenza organizzata con eventi georeferenziati, ma non le proteste pacifiche (vedi [[ucdp]]).
- **Aperto:** per proteste e disordini senza morti serve un'altra fonte con licenza aperta. Da cercare con chi cura [[crisi-ed-eventi]] e [[criminalita-e-giustizia]].

## Collegamenti
- [[ucdp]]: il sostituto con licenza aperta.
- [[controinsurrezione-e-stabilita]]: dove i dati sugli eventi servirebbero.
- [[iiss-military-balance]]: l'altro dataset escluso per licenza.

## Riferimenti
- **URL** https://acleddata.com/eula · [V]
  **perché conta qui** è il contratto vincolante
  **cosa se ne prende** il divieto di ridistribuzione e di uso commerciale: ACLED escluso
- **URL** https://acleddata.com/contentusage · [V]
  **perché conta qui** i divieti espliciti (concorrenza, IA)
  **cosa se ne prende** l'esclusione anche dalle pipeline di sviluppo con IA
- **URL** https://acleddata.com/terms-of-use/ · [V]
  **perché conta qui** l'indice dei tre documenti e la titolarità
  **cosa se ne prende** solo il riferimento alla struttura dei termini
