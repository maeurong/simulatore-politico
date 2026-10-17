---
tipo: dataset
fonti: [fonti/sipri-milex.md, fonti/sipri-armstransfers.md, fonti/sipri-terms.md]
aggiornato: 2026-10-16
---
# SIPRI: spesa militare e trasferimenti di armi

Due database dello Stockholm International Peace Research Institute: la spesa militare per paese dal 1949 e i trasferimenti di grandi armamenti convenzionali dal 1950. Sono il riferimento mondiale per calibrare i bilanci della difesa e i flussi di armi. La licenza però **non** permette di ridistribuirli dentro il gioco senza autorizzazione.

## Military Expenditure Database

- Serie coerenti di spesa militare per paese, 1949-2025, aggiornate ogni anno; l'aggiornamento può rivedere anche gli anni passati (fonti/sipri-milex.md).
- Valori in valuta locale corrente (per anno fiscale e solare), in dollari costanti 2024 e correnti, in % del PIL, pro capite e in % della spesa pubblica (fonti/sipri-milex.md).
- Copertura: per la maggior parte dei paesi indipendenti, almeno dalla fine degli anni Cinquanta. Il totale mondiale parte dal 1988, perché mancano i dati dell'URSS. Solo fonti aperte (fonti/sipri-milex.md).
- File: `SIPRI-Milex-data-1949-2025_v1.2.xlsx`, rivisto il 27 aprile 2026. DOI 10.55163/CQGC9685 (fonti/sipri-milex.md).

## Arms Transfers Database

- Tutti i trasferimenti di grandi armamenti convenzionali dal 1950 all'ultimo anno solare completo; aggiornato il 9 marzo 2026 con i dati 1950-2025, che sostituiscono tutti i precedenti (fonti/sipri-armstransfers.md).
- Risponde a domande come: chi fornisce chi, come cambiano le relazioni fornitore-cliente, da dove prendono le armi i paesi in conflitto (fonti/sipri-armstransfers.md).

## Licenza e ridistribuibilità

- **Fair use** solo se valgono *entrambe* le condizioni: uso non commerciale (critica, commento, cronaca, insegnamento, ricerca) **e** riproduzione di meno del 10% di un dataset pubblicato (fonti/sipri-terms.md).
- Ogni riproduzione dei dati, in qualunque mezzo, richiede autorizzazione, salvo il fair use. Senza permesso si possono usare per tesi, articoli accademici e giornalistici, blog, grafici, purché sotto il 10% e citando SIPRI (fonti/sipri-terms.md).
- **Uso commerciale** = attività per guadagno, vantaggio d'affari o promozione, anche indiretta, inclusi strumenti interni. Richiede licenza e di norma una royalty; SIPRI valuta caso per caso (fonti/sipri-terms.md).
- Permessi oltre il fair use: richiesta scritta con nome, affiliazione e scopo; credito obbligatorio "Information from the Stockholm International Peace Research Institute (SIPRI)..." con URL o DOI (fonti/sipri-terms.md).
- Legge applicabile: svedese (fonti/sipri-terms.md).

## Per il nostro gioco

- **Non si ridistribuisce** il dataset nel repo né nel gioco: anche gratis, oltre il 10% serve un permesso. Se il gioco è venduto, serve una licenza commerciale per qualsiasi uso [INF dalla lettura dei termini].
- **Si può** usare SIPRI in sviluppo per tarare i parametri. Resta però un dubbio, che è un'interpretazione e non un parere legale: se i valori iniziali per paese derivati da SIPRI finiscono nei file di gioco, sono una "riproduzione"? Con pochi paesi per un solo anno si resta sotto il 10%, ma l'uso commerciale scatta comunque se il gioco è venduto [INF].
- **Alternativa ridistribuibile:** i dati NATO (tabelle pubbliche 2014-2025, vedi [[prontezza-e-spesa-per-la-difesa]]) per i paesi NATO, e la spesa militare di COW NMC per lo storico (vedi [[correlates-of-war]]). Per il resto del mondo, valori arrotondati e dichiarati "ispirati a" [INF].
- **Aperto:** scrivere a SIPRI per un permesso esplicito (progetto, repo pubblico, eventuale vendita). Costo: un'email. Da girare al filone legale (`legale/`). Il gioco è gratuito ([[decisioni]]): cade l'ipotesi della vendita, resta il limite del 10% oltre il quale serve il permesso.

## Collegamenti
- [[prontezza-e-spesa-per-la-difesa]]: dove la spesa entra nel modello.
- [[correlates-of-war]]: alternativa storica per spesa e personale.
- [[iiss-military-balance]]: l'altra fonte di riferimento, ancora più chiusa.
- [[commercio-modello-gravitazionale]]: i trasferimenti di armi come voce del commercio estero.

## Riferimenti
- **URL** https://www.sipri.org/about/terms-and-conditions · [V]
  **perché conta qui** decide se i dati possono stare nel gioco
  **cosa se ne prende** la doppia condizione (non commerciale e meno del 10%) e l'obbligo di licenza per l'uso commerciale
- **URL** https://www.sipri.org/databases/milex · [V]
  **perché conta qui** copertura, unità e versione del dataset di spesa
  **cosa se ne prende** le serie in %PIL e in dollari costanti come riferimento di taratura
- **URL** https://www.sipri.org/databases/armstransfers · [V]
  **perché conta qui** l'unica serie lunga sui flussi di armi
  **cosa se ne prende** la struttura fornitore → cliente per il commercio di armi
