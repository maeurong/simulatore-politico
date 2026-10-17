---
tipo: modello
fonti: [fonti/who-ghed.md, fonti/wikipedia-baumol-cost-disease.md, fonti/wikipedia-preston-curve.md, fonti/steam-plague-inc-the-cure.md, fonti/eurostat-population-ageing.md]
aggiornato: 2026-10-16
---
# Sanità e spesa sanitaria

La sanità nel gioco è due cose: una voce di spesa che cresce da sola più del PIL e un produttore di salute (aspettativa di vita, capacità di reggere un'epidemia). Bastano tre relazioni empiriche: Preston, Baumol, invecchiamento.

## Relazioni
- **Curva di Preston**: relazione trasversale fra reddito pro capite e aspettativa di vita alla nascita. Ripida nei paesi poveri, piatta in quelli ricchi (fonti/wikipedia-preston-curve.md). Nel gioco: oltre una certa soglia di reddito, la spesa sanitaria compra pochi anni di vita.
- **Malattia dei costi di Baumol**: nei settori con poca crescita di produttività i salari salgono comunque, perché devono competere con quelli dei settori produttivi; i servizi diventano più cari rispetto ai beni (fonti/wikipedia-baumol-cost-disease.md). Sanità e istruzione sono i casi classici: a parità di servizio la spesa sale ogni anno.
- **Invecchiamento**: nell'UE il 22,0% della popolazione ha 65 anni o più (1/1/2025) (fonti/eurostat-population-ageing.md). [INF] La spesa pro capite cresce con l'età, quindi la piramide di [[demografia-e-migrazioni]] spinge la spesa.

## Modello proposto
[INF] Spesa sanitaria = popolazione per età × costo per età × indice Baumol (cresce con i salari reali) × copertura pubblica. L'output è una "capacità" (posti letto, personale) che fissa la letalità effettiva in [[epidemie-sir-seir]] e la componente salute di [[istruzione-e-capitale-umano]].

## Variabili
- **Mosse**: quota pubblica della spesa, ticket, posti letto e personale formato, prevenzione (tabacco, alcol), prezzi dei farmaci, liste d'attesa.
- **Subite**: invecchiamento, crescita dei costi alla Baumol, epidemie, emigrazione di medici.

## Ritardi
- [INF] Posti letto: anni (edilizia). Medici: un ciclo di formazione, circa un decennio. Prevenzione: decenni sulla mortalità. Ticket: effetto immediato sul bilancio, ritardato sulla salute.

## Dati per calibrare
- WHO Global Health Expenditure Database: spesa comparabile per 195 paesi dal 2000, per fonte (governo, famiglie, donatori) e per funzione; aggiornamento 2025 con dati 2023; è anche la fonte dei dati di spesa di World Bank WDI e GHO (fonti/who-ghed.md).
- [[who-gho]] per aspettativa di vita e mortalità.

## Giochi
- [[plague-inc]] (modalità The Cure): preparare gli ospedali prima che si saturino è una leva esplicita (fonti/steam-plague-inc-the-cure.md).

## Per il nostro gioco
- Prendere: la spesa che sale da sola (Baumol più invecchiamento) come pressione di bilancio costante; capacità ospedaliera come stock che serve nelle crisi.
- Evitare: la salute come semplice funzione lineare della spesa (Preston dice di no).

## Collegamenti
- [[demografia-e-migrazioni]], [[epidemie-sir-seir]], [[istruzione-e-capitale-umano]], [[ritardi-e-inerzia-delle-politiche]].
- [[servizi-pubblici]]: come i gestionali rendono la copertura dei servizi sul territorio.
