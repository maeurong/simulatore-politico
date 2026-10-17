---
tipo: sistema
fonti: [fonti/hoi4-wiki-government.md, fonti/eu4-wiki-stability.md, fonti/eu4-wiki-estates.md, fonti/eu5-wiki-government.md, fonti/eu5-wiki-estate.md, fonti/eu5-wiki-parliament.md, fonti/vic3-wiki-government.md, fonti/vic3-wiki-political-movement.md, fonti/vic3-wiki-standard-of-living.md, fonti/vic3-wiki-pops.md, fonti/vic3-wiki-interest-groups.md, fonti/ck3-wiki-faction.md, fonti/ck3-wiki-council.md, fonti/ck3-wiki-character.md, fonti/ck3-wiki-attributes.md, fonti/stellaris-wiki-factions.md, fonti/terrainvicta-wiki-nations.md, fonti/wikipedia-shadow-empire.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Personaggi, fazioni e stabilità interna

Come i grand strategy misurano la tenuta interna di un regime: un indice di stabilità, gruppi organizzati con interessi propri, personaggi con opinioni, soglie oltre cui scattano crisi, colpi di stato o guerre civili. Per un simulatore di governo è il cuore del gioco; nei grand strategy è spesso un contorno della guerra, e i giocatori lo notano.

## Indici di stabilità a confronto

| Gioco | Scala | Effetti principali |
|---|---|---|
| HOI4 | 0-100%, neutro a 50 | sopra: fino a +20% output industriale; sotto: fino a −50%; −20 punti in guerra (fonti/hoi4-wiki-government.md) |
| EU4 | da −3 a +3 | a −3 niente dichiarazioni di guerra, pace forzata se il punteggio è sotto del 30%; si compra con punti amministrativi (fonti/eu4-wiki-stability.md) |
| EU5 | da −100 a +100, tende a 0 | per punto positivo soglie di ribellione più alte ed equilibrio di soddisfazione delle estati (fonti/eu5-wiki-government.md) |
| Victoria 3 | legittimità 0-100 | sotto 25 governo illegittimo, passa solo leggi sostenute da movimenti (fonti/vic3-wiki-government.md) |
| Terra Invicta | disordini 0-10, coesione, governo 0-10 | disordini alti tagliano il surplus e rendono probabili colpi di stato (fonti/terrainvicta-wiki-nations.md) |

## Gruppi organizzati

- **Estati** (EU4, EU5). EU4: influenza (0,5 per 1% di terre, massimo 40) e lealtà con equilibrio 30 (fonti/eu4-wiki-estates.md). EU5: potere percentuale dai pop e dai privilegi; soddisfazione sopra o sotto il 50% dà bonus o penalità; sotto il 25% l'estate non dà leve; ogni estate ha simpatie e antipatie di politica estera (fonti/eu5-wiki-estate.md).
- **Gruppi d'interesse e movimenti** (Victoria 3). Il peso dei gruppi viene dalla forza politica dei pop che li sostengono, legata alla ricchezza (fonti/vic3-wiki-interest-groups.md). I movimenti hanno un attivismo che si muove dell'1% a settimana verso l'obiettivo; sopra il 50% diventano ostinati e radicalizzano dallo 0,5% al 2,5% dei sostenitori al mese, sopra il 75% diventano rivoluzionari e riempiono una barra 0-100% che apre la guerra civile (fonti/vic3-wiki-political-movement.md).
- **Fazioni** (CK3, Stellaris). CK3: forza militare relativa dei membri e malcontento; sopra l'80% di forza il malcontento sale, a 100% ultimatum o guerra civile (fonti/ck3-wiki-faction.md). Stellaris: approvazione 0-100 con base 50, fasce di felicità da −40% a +10% (fonti/stellaris-wiki-factions.md).
- **Parlamento** (EU5): convocato ogni 5 anni, una questione per 180 giorni, passa secondo il potere delle estati favorevoli e contrarie (fonti/eu5-wiki-parliament.md).

## Personaggi

- CK3: ogni personaggio ha un'opinione di ogni altro; l'erede eredita il 25% dell'opinione positiva e il 50% di quella negativa dei vassalli verso il predecessore (fonti/ck3-wiki-character.md). Vassali potenti esclusi dal consiglio: −40 di opinione (fonti/ck3-wiki-faction.md).
- CK3: stress quando si agisce contro la propria personalità, crolli mentali alle soglie 100, 200, 300; terrore 0-100 che piega i meno audaci; legittimità che sale e scende con le azioni del sovrano (fonti/ck3-wiki-attributes.md).
- EU5: gabinetto con 2 seggi iniziali; sostituire un ministro costa 5 di stabilità; il capo di gabinetto dà +25% di efficienza, e se viene da un'estate quella guadagna potere (fonti/eu5-wiki-government.md).
- Shadow Empire: il personale ha competenze e personalità e può portare i generali alla guerra civile (fonti/wikipedia-shadow-empire.md).

## Soglie di crisi

- Terra Invicta, probabilità giornaliera di colpo di stato = ((disordini × 2 − coesione − governo) / 10 + scontento delle élite) / 6000; si controlla ogni quattro giorni (fonti/terrainvicta-wiki-nations.md).
- Terra Invicta: disordini di riposo = 10,5 − 1 per 10.000 di PIL pro capite − 1 per punto di coesione (fonti/terrainvicta-wiki-nations.md).
- Victoria 3: radicali da tenore di vita sotto l'atteso; la turbolenza di base è pari alla percentuale di radicali (fonti/vic3-wiki-standard-of-living.md, fonti/vic3-wiki-pops.md).
- HOI4: stabilità bassa in guerra può far scattare crisi fino alla guerra civile (fonti/hoi4-wiki-government.md).

## Cosa dicono i giocatori

- HOI4: politica interna "flattened... into a Bethesda-esque, two-option choice" (fonti/steam-recensioni-grand-strategy.md).
- CK3: lodato come "dynastic systems simulator"; criticato per modificatori nascosti di pochi punti (fonti/steam-recensioni-grand-strategy.md).
- Shadow Empire: ideologia bloccata, cambiarla a metà provoca ribellioni ingestibili (fonti/steam-recensioni-grand-strategy.md).
- Terra Invicta: cambi d'opinione decisivi nascosti in icone minuscole (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: separare legittimità del governo (Victoria 3), coesione sociale e disordini (Terra Invicta). Tre numeri diversi, tre leve diverse.
- Prendiamo: gruppi organizzati con potere proporzionale alla base sociale e soddisfazione con soglie, come estati EU5 e gruppi Victoria 3. Per noi sono sindacati, imprese, chiesa, forze armate, magistratura.
- Prendiamo: movimenti con attivismo che cresce lentamente e soglie (ostinato, rivoluzionario).
- Prendiamo: fazioni interne alla maggioranza alla CK3, con forza relativa e malcontento.
- Prendiamo: probabilità di colpo di stato come funzione esplicita di disordini, coesione e democrazia, da calibrare sui dati veri ([[controinsurrezione-e-stabilita]]).
- Evitiamo: crisi senza preavviso e cause invisibili; ogni soglia deve essere visibile prima di essere superata ([[feedback-visivo-dello-stato]]).
- Aperto: quanti personaggi con nome. Ministri e leader di partito sì; il resto come gruppi.

## Collegamenti

- [[crusader-kings-3]], [[victoria-3]], [[europa-universalis]], [[terra-invicta]], [[stellaris]], [[shadow-empire]], [[hearts-of-iron-4]].
- [[elettorato-e-gruppi-di-voto]]: il lato elettorale dei gruppi.
- [[partiti-e-coalizioni]]: fazioni dentro i partiti.
- [[ministri-e-gabinetto]]: gabinetto e deleghe.
- [[parlamento-e-legislazione]]: iter delle leggi.
- [[controinsurrezione-e-stabilita]]: modelli formali.
- [[polarizzazione-e-fiducia-istituzionale]]: coesione e fiducia.

## Riferimenti

- **URL** https://wiki.hoodedhorse.com/Terra_Invicta/Nations · [V] (snapshot 2026-10-12)
  **perche' conta qui** formule esplicite di colpo di stato e disordini
  **cosa se ne prende** struttura della funzione da calibrare
- **URL** https://vic3.paradoxwikis.com/Political_movement · [V] (snapshot 2026-08-22)
  **perche' conta qui** movimenti con attivismo e soglie
  **cosa se ne prende** 1% a settimana, soglie 50% e 75%, barra rivoluzionaria
- **URL** https://ck3.paradoxwikis.com/Faction · [V] (pagina Vassals, snapshot 2024-10-08)
  **perche' conta qui** fazioni che si coalizzano contro il capo
  **cosa se ne prende** forza relativa 80% e malcontento 100%
- **URL** https://eu5.paradoxwikis.com/Estate · [V]
  **perche' conta qui** gruppi sociali con potere, soddisfazione e politica estera
  **cosa se ne prende** soddisfazione con soglie 50% e 25%
