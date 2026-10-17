---
tipo: sistema
fonti: [fonti/cs1-wiki-distretti.md, fonti/cs1-wiki-politiche.md, fonti/cs1-wiki-valore-terreno.md, fonti/cs2-devdiary-servizi.md, fonti/cs2-devdiary-economia.md, fonti/cs2-steam-economia-2-0.md, fonti/simcity-wiki-sc4.md, fonti/simcity2013-wiki-gioco.md, fonti/simcity2013-pcgamer-dimensioni.md, fonti/glassbox-gamedeveloper-gdc2012.md, fonti/anno117-wiki-gioco.md, fonti/anno1800-wiki-bisogni.md, fonti/frostpunk2-steam-dopo-beta.md, fonti/wr-wiki-cittadini.md, fonti/ostriv-michet-blog.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Territorio e urbanistica

Come i gestionali dividono lo spazio governato (zone, distretti, regioni, isole) e quali grandezze ci mettono sopra (valore del terreno, desiderabilità, inquinamento). Per un simulatore di governo il territorio è la superficie su cui le politiche diventano visibili. Qui conta scegliere l'unità territoriale e le mappe di grandezze.

## Unità di territorio nel filone

| Unità | Gioco | Cosa ci si attacca |
|---|---|---|
| Zona (R/C/I/uffici) | Cities: Skylines, SimCity | Tipo di sviluppo e densità; la domanda per zona guida la crescita (fonti/cs2-steam-economia-2-0.md) |
| Distretto dipinto | Cities: Skylines | Politiche, specializzazione (foresta, agricoltura, minerali, petrolio, turismo), statistiche proprie (fonti/cs1-wiki-distretti.md) |
| Area di industria specializzata | Cities: Skylines II | Estrazione di materie prime, disegnata con uno strumento simile ai distretti (fonti/cs2-devdiary-economia.md) |
| Distretto funzionale a dimensione fissa | Frostpunk 2 | Si costruisce l'intero distretto, non il singolo edificio (fonti/frostpunk2-steam-dopo-beta.md) |
| Città in una regione | SimCity 4 | Più città adiacenti costruite dal giocatore, con accordi fra vicini (fonti/simcity-wiki-sc4.md) |
| Isola, provincia | Anno 1800/117 | Magazzino unico, fertilità, ceti; due province interdipendenti in Anno 117 (fonti/anno1800-wiki-bisogni.md; fonti/anno117-wiki-gioco.md) |
| Mappa libera senza griglia | Ostriv | Posizionamento e rotazione liberi, sentieri dove la gente cammina (fonti/ostriv-michet-blog.md) |

## Grandezze sul territorio

- **Mappe di GlassBox**: valore del suolo, desiderabilità, inquinamento, risorse estraibili sono "mappe" che le regole leggono e scrivono (es. una fabbrica scarica 5 di inquinamento sulla mappa) (fonti/glassbox-gamedeveloper-gdc2012.md).
- **Valore del terreno**: in Cities: Skylines sale con servizi, decorazioni e assenza di inquinamento e alza la felicità locale; la mappa di calore va dal verde (alto) al blu (basso) (fonti/cs1-wiki-valore-terreno.md). Nella II i servizi agiscono solo in modo indiretto, attraverso la disponibilità a pagare l'affitto (fonti/cs2-devdiary-servizi.md).
- **Localizzazione delle imprese**: in Cities: Skylines II l'industria sceglie dove insediarsi in base ai costi di trasporto e all'ingombro: le produzioni che occupano spazio cercano terreni a basso valore (fonti/cs2-devdiary-economia.md).
- **Raggi di accessibilità**: in Workers & Resources il raggio a piedi di 480 m disegna di fatto i quartieri, e ogni edificio mostra in sovrapposizione i propri raggiungibili (fonti/wr-wiki-cittadini.md).

## Il limite di dimensione

SimCity 2013 ha città di 2 × 2 km, pari alle città medie di SimCity 4, per scelta di prestazioni (fonti/simcity2013-wiki-gioco.md; fonti/simcity2013-pcgamer-dimensioni.md). La regione di SimCity 4 è la risposta opposta: tante città piccole collegate (fonti/simcity-wiki-sc4.md).

## Cosa dicono i giocatori

Della II di Cities si critica la zonizzazione rigida "sensibile agli angoli" e la mancanza di edifici d'angolo realistici (fonti/steam-recensioni-gestionali.md). Dei distretti a dimensione fissa di Frostpunk 2 si dice che tolgono controllo (fonti/steam-recensioni-gestionali.md).

## Per il nostro gioco

- **Prendiamo:** gerarchia nazione → regione → provincia/distretto, dove ogni livello è un contenitore di aggregati e un bersaglio di politiche (distretti di Cities: Skylines, regione di SimCity 4); grandezze territoriali come mappe che le regole leggono e scrivono (GlassBox); specializzazione economica del territorio come conseguenza di costi e risorse (Cities: Skylines II).
- **Evitiamo:** zonizzazione edificio per edificio: a scala paese il giocatore non pianifica isolati.
- **Aperto:** la granularità minima della mappa (provincia? collegio elettorale?), da decidere insieme a [[elettorato-e-gruppi-di-voto]].

## Collegamenti

- [[servizi-pubblici]]: la copertura vive sul territorio.
- [[logistica-e-trasporti]]: nodi e corridoi.
- [[feedback-visivo-dello-stato]]: mappe di calore.
- [[cities-skylines]], [[simcity-4]], [[simcity-2013]], [[anno]], [[ostriv]].
- [[sistemi-elettorali]]: i collegi come possibile unità minima della mappa.
