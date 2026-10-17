---
tipo: dataset
fonti: [fonti/iiss-military-balance-2022-colophon.md, fonti/iiss-military-balance-plus-flyer-2017.md, fonti/cow-nmc-v7-documentazione-fonti.md]
aggiornato: 2026-10-16
---
# IISS The Military Balance

La valutazione annuale delle capacità militari e dell'economia della difesa dell'International Institute for Strategic Studies, pubblicata dal 1959: forze, equipaggiamenti, bilanci, schieramenti per oltre 170 paesi. È la fonte che ogni wargame vorrebbe usare per gli ordini di battaglia. **Non la possiamo usare**: è un'opera protetta, con tutti i diritti riservati, e un prodotto in abbonamento a pagamento senza diritti di ridistribuzione.

## Cos'è

- Pubblicazione annuale dal 1959, "the annual assessment of global military capabilities and defence economics" (fonti/iiss-military-balance-plus-flyer-2017.md).
- Edizione a stampa pubblicata da Routledge Journals (Taylor & Francis), ISSN 0459-7222; l'edizione online è inclusa negli abbonamenti istituzionali (fonti/iiss-military-balance-2022-colophon.md).
- **Military Balance+** (dal 2017): database online con forze di 171 paesi e di alcuni gruppi armati non statali, inventari di equipaggiamento, bilanci, procurement, schieramenti (fonti/iiss-military-balance-plus-flyer-2017.md).

## Perché non si può usare

1. **Tutti i diritti riservati.** "No part of this publication may be reproduced, stored, transmitted, or disseminated, in any form, or by any means, without prior written permission from Taylor & Francis" (fonti/iiss-military-balance-2022-colophon.md). Vale per il libro e quindi per le tabelle che contiene.
2. **Accesso a pagamento e per utente.** Military Balance+ si vende con licenze aziendali (minimo cinque utenti, prezzo su richiesta) o abbonamenti individuali (£695 l'anno al lancio, 2017), che danno a un singolo utente la possibilità di consultare e analizzare (fonti/iiss-military-balance-plus-flyer-2017.md). Nessuna delle due prevede la ridistribuzione [INF].
3. **I termini d'uso online non sono leggibili senza browser.** [NON TROVATO] La pagina https://www.iiss.org/terms-and-conditions/ si carica solo con JavaScript; né defuddle né WebFetch ne hanno estratto il testo. Va letta a mano prima di qualsiasi richiesta di permesso.
4. **Contaminazione indiretta.** Il personale militare di COW NMC dopo il 1993 è integrato con dati del Military Balance (fonti/cow-nmc-v7-documentazione-fonti.md). Usare COW non elimina del tutto la questione, ma la sposta su un dataset accademico che li pubblica come valori propri [INF].

## Per il nostro gioco

- **Non usiamo** IISS né come file né come fonte di numeri copiati negli ordini di battaglia.
- **Usiamo invece** i dati NATO (personale per paese 2014-2025, tabella 7, vedi [[prontezza-e-spesa-per-la-difesa]]), i dati ufficiali dei ministeri della difesa nazionali, e COW NMC per lo storico (vedi [[correlates-of-war]]) [INF].
- **Evitiamo** l'equipaggiamento reale per paese, modello per modello. Il gioco ragiona per capacità aggregate (forza efficace, prontezza), che è anche la scala giusta per un capo di governo; vedi [[lanchester-e-modelli-di-combattimento-aggregati]] [INF].
- **Aperto:** se un giorno servissero ordini di battaglia realistici, c'è il database di Command, costruito da fonti aperte ma proprietario (vedi [[command-modern-operations]]): andrebbe licenziato anche quello [INF].

## Collegamenti
- [[correlates-of-war]]: alternativa storica, in parte derivata da IISS.
- [[sipri]]: l'altra fonte di riferimento, con fair use limitato.
- [[acled]]: l'altro dataset escluso per licenza.
- [[prontezza-e-spesa-per-la-difesa]]: i dati NATO come sostituto.

## Riferimenti
- **URL** https://api.pageplace.de/preview/DT0400.9781000619720_A43060657/preview-9781000619720_A43060657.pdf (colophon, The Military Balance 2022) · [V]
  **perché conta qui** la clausola di copyright scritta dall'editore stesso
  **cosa se ne prende** "all rights reserved" e permessi solo scritti da Taylor & Francis: escluso
- **URL** https://egmontinstitute.be/app/uploads/2017/01/170227-flyer-Military-Balance-Plus-online.pdf · [V] (2017, prezzi forse superati)
  **perché conta qui** modello di licenza del database online
  **cosa se ne prende** accesso per utente a pagamento, nessun diritto di ridistribuzione
- **URL** https://www.iiss.org/terms-and-conditions/ · [NON TROVATO] (pagina solo JavaScript)
  **perché conta qui** sarebbero i termini vigenti del sito
  **cosa se ne prende** nulla finché non viene letta a mano
