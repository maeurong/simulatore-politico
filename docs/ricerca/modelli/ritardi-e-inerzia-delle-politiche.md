---
tipo: modello
fonti: [fonti/meadows-leverage-points.md, fonti/democracy4-modding.md, fonti/democracy3-modding.md, fonti/democracy4-mod-policies.md, fonti/en-roads-guide-background.md, fonti/worldbank-hci-methodology-wps8593.md, fonti/worldbank-wgi.md, fonti/un-wpp-2024-summary-pdf.md, fonti/bea-gdp-reliability-2008.md, fonti/wikipedia-fate-of-the-world.md]
aggiornato: 2026-10-16
---
# Ritardi e inerzia delle politiche (pagina trasversale)

Una riforma della scuola rende in anni, non in turni. Questa pagina raccoglie i tempi di risposta dei sottosistemi e i tre meccanismi con cui un gioco li rappresenta. È la decisione di progetto che più separa un simulatore verosimile da un gioco di slider.

## Perché contano
- I ritardi nei cicli di retroazione sono cause comuni di oscillazione: chi corregge un sistema ricevendo informazioni in ritardo supera il bersaglio in un verso e poi nell'altro (fonti/meadows-leverage-points.md).
- Contano rispetto alla velocità di cambiamento: troppo corti, il sistema reagisce in eccesso; troppo lunghi, oscilla o collassa (fonti/meadows-leverage-points.md).
- Sono raramente modificabili («le cose impiegano il tempo che impiegano») (fonti/meadows-leverage-points.md). Per il gioco: il giocatore non accorcia i ritardi, li anticipa.

## Tre meccanismi, dal più semplice
1. **Media mobile (inerzia)**. Democracy 4: ogni effetto ha un'inerzia N e vale la media degli ultimi N valori dello slider. Inerzia 4 significa fino a 4 turni per arrivare all'effetto pieno (fonti/democracy4-modding.md). Economico; però tratta tutti i ritardi come rampe lineari.
2. **Decadimento esponenziale**. Democracy 3, i «grudges»: un effetto moltiplicato ogni turno per un fattore (es. 0,9) che svanisce gradualmente (fonti/democracy3-modding.md). Adatto a shock e memoria dell'opinione.
3. **Stock con tempi di permanenza**. En-ROADS: il capitale energetico si rinnova solo quando il vecchio va in pensione; circa il 6% delle infrastrutture cambia ogni anno (fonti/en-roads-guide-background.md). Lo HCI: la forza lavoro si rinnova coorte per coorte in circa 50 anni (fonti/worldbank-hci-methodology-wps8593.md). Più costoso, ma il ritardo nasce dalla fisica del sistema e non da un parametro arbitrario.
- In più, il **tempo di attuazione**: Democracy 4 impiega N turni per attuare o cancellare una policy, corretti per la competenza del ministro (fonti/democracy4-mod-policies.md). Vedi [[burocrazia-corruzione-e-capacita-statale]].

## Tabella dei tempi di risposta
| Sottosistema | Ritardo tipico | Fonte |
|---|---|---|
| Capacità energetica fossile | decenni (ritiro ~3%/anno) | fonti/en-roads-guide-background.md |
| Auto, edifici, industria | 10-15 anni di vita utile | fonti/en-roads-guide-background.md |
| Temperatura dopo le emissioni | anni o decenni (inerzia oceanica) | fonti/en-roads-guide-background.md |
| Istruzione → produttività | ~50 anni per l'effetto pieno | fonti/worldbank-hci-methodology-wps8593.md |
| Fertilità → forza lavoro | ~20 anni | fonti/meadows-leverage-points.md (esempio qualitativo); [INF] il numero |
| Politiche pro-natalità | efficacia in calo con l'invecchiamento | fonti/un-wpp-2024-summary-pdf.md |
| Percezione di governance/corruzione | un anno o più dopo i fatti | fonti/worldbank-wgi.md |
| Dati macro (PIL) | stime riviste di ~1 punto in media | fonti/bea-gdp-reliability-2008.md |
| Epidemie | giorni o settimane (latenza, test) | [[epidemie-sir-seir]] |
| Salienza mediatica | giorni, decade in fretta | [[media-informazione-e-propaganda]] |

## Il problema di game design
- La Banca Mondiale lo scrive per l'istruzione: i benefici richiedono tempo e «non sono molto visibili agli elettori», ed è per questo che i politici li trascurano (fonti/worldbank-hci-methodology-wps8593.md). Se il gioco è fedele, il giocatore razionale fa lo stesso.
- Fate of the World ha turni di 5 anni e politiche che bloccano fondi per più turni (fonti/wikipedia-fate-of-the-world.md): con un passo così lungo il ritardo resta leggibile.
- [INF] Tre contromisure: (a) indicatori anticipatori visibili subito (punteggi dei test, cantieri avviati, capacità installata); (b) la proiezione del consigliere («fra 10 anni…»), con l'incertezza di [[consiglieri-e-informazione-imperfetta]]; (c) un bilancio di eredità a fine partita, o la possibilità di giocare più mandati e vedere i frutti.

## Per il nostro gioco
- Raccomandazione [INF]: passo base mensile (o trimestrale) per l'economia e le crisi, e stock per coorte/vintage per energia, istruzione e demografia; media mobile solo dove manca una struttura fisica.
- Evitare: un'inerzia uguale per tutto. Il realismo sta nelle differenze fra i ritardi.

## Collegamenti
- [[voto-economico-e-retrospettivo]] (altro filone): se l'elettore giudica solo il passato recente, i ritardi lunghi puniscono chi investe.
- [[policy-e-effetti]] (altro filone): dove l'inerzia entra nel grafo delle politiche.
- Tutte le pagine di modello di questo filone; [[democracy-4]] per il meccanismo di inerzia; [[fate-of-the-world]].
- [[leggi-decreti-ed-editti]]: la maturazione degli editti di Tropico come forma di ritardo.
- [[tempo-e-ritmo]]: le cadenze dei sottosistemi nei grand strategy.
- [[crisi-ed-eventi]]: il decadimento dei grudges e le crisi a soglia.
