---
tipo: sistema
fonti: [fonti/hoi4-wiki-land-battle.md, fonti/hoi4-wiki-government.md, fonti/hoi4-wiki-production.md, fonti/vic3-wiki-standard-of-living.md, fonti/vic3-wiki-diplomatic-play.md, fonti/vic3-wiki-laws.md, fonti/vic3-wiki-building.md, fonti/vic3-wiki-political-movement.md, fonti/eu4-wiki-monarch-power.md, fonti/eu4-wiki-land-warfare.md, fonti/eu4-wiki-manpower.md, fonti/eu5-wiki-population.md, fonti/eu5-wiki-military.md, fonti/eu5-wiki-parliament.md, fonti/terrainvicta-wiki-gdp.md, fonti/terrainvicta-wiki-nations.md, fonti/stellaris-dd370-4-0-cambiamenti.md, fonti/wikipedia-hearts-of-iron-iv.md, fonti/wikipedia-victoria-3.md, fonti/wikipedia-europa-universalis-iv.md, fonti/wikipedia-europa-universalis-v.md, fonti/wikipedia-crusader-kings-iii.md, fonti/wikipedia-stellaris.md, fonti/wikipedia-supreme-ruler-ultimate.md, fonti/wikipedia-shadow-empire.md, fonti/steam-recensioni-grand-strategy.md, fonti/steam-recensioni-realismo.md]
aggiornato: 2026-10-17
---
# Tempo e ritmo

Come i grand strategy scandiscono il tempo: il tick della simulazione, la cadenza dei vari sottosistemi, pausa e velocità, l'arco storico di una partita. Tutti i titoli Paradox usano un calendario continuo con pausa, ma ogni sottosistema gira a una cadenza diversa. È la decisione di design che il filone tecnico lascia aperta (giorno o settimana).

## Arco storico e lunghezza nominale

| Gioco | Inizio | Fine o durata | Fonte |
|---|---|---|---|
| Hearts of Iron IV | 1° gennaio 1936 o 14 agosto 1939 | [NON TROVATO] | (fonti/wikipedia-hearts-of-iron-iv.md) |
| Victoria 3 | 1836 | 1936 | (fonti/wikipedia-victoria-3.md) |
| Europa Universalis IV | 11 novembre 1444 (ultimo scenario 1792) | [NON TROVATO] | (fonti/wikipedia-europa-universalis-iv.md) |
| Europa Universalis V | 1° aprile 1337 | 1° gennaio 1837 | (fonti/wikipedia-europa-universalis-v.md) |
| Crusader Kings III | 867, 1066 o 1178 | [NON TROVATO]; si prosegue con l'erede | (fonti/wikipedia-crusader-kings-iii.md) |
| Stellaris | 1° gennaio 2200 | 300 anni in tre fasi da 100 | (fonti/wikipedia-stellaris.md) |
| Supreme Ruler Ultimate | sandbox dal 1914 al 2020 | aperto | (fonti/wikipedia-supreme-ruler-ultimate.md) |
| Shadow Empire | turni | fino alla conquista | (fonti/wikipedia-shadow-empire.md) |

Quante ore reali dura una partita non è documentato da fonti primarie [NON TROVATO].

## Cadenze interne

| Cadenza | Esempi |
|---|---|
| Ora | battaglia terrestre HOI4 a turni di un'ora (fonti/hoi4-wiki-land-battle.md) |
| Giorno | potere politico HOI4 +2/giorno (fonti/hoi4-wiki-government.md); crescita dell'efficienza industriale HOI4 (fonti/hoi4-wiki-production.md); escalation dei diplomatic play di Victoria 3 (fonti/vic3-wiki-diplomatic-play.md) |
| 3 giorni | fasi fuoco/urto delle battaglie EU4 (fonti/eu4-wiki-land-warfare.md) |
| 4 giorni | controllo di colpi di stato e rivoluzioni in Terra Invicta (fonti/terrainvicta-wiki-nations.md) |
| Settimana | spesa e ricchezza dei pop di Victoria 3 (fonti/vic3-wiki-standard-of-living.md); punti costruzione ogni lunedì (fonti/vic3-wiki-building.md); attivismo dei movimenti ±1% (fonti/vic3-wiki-political-movement.md) |
| Mese | potere del monarca EU4 (fonti/eu4-wiki-monarch-power.md); nascite e attrito EU5 (fonti/eu5-wiki-population.md, fonti/eu5-wiki-military.md); surplus di investimento Terra Invicta (fonti/terrainvicta-wiki-gdp.md); calcoli economici di Stellaris (fonti/stellaris-dd370-4-0-cambiamenti.md) |
| 100 giorni | checkpoint delle leggi di Victoria 3 (fonti/vic3-wiki-laws.md) |
| Anni | manpower EU4 si ricostituisce in 10 anni (fonti/eu4-wiki-manpower.md); parlamento EU5 ogni 5 anni, una questione dibattuta 180 giorni (fonti/eu5-wiki-parliament.md) |

## Pausa e velocità

- Supreme Ruler è in tempo reale con pausa e velocità regolabile (fonti/wikipedia-supreme-ruler-ultimate.md); Stellaris è in tempo reale (fonti/wikipedia-stellaris.md). Che tutti i Paradox abbiano pausa e velocità è noto ma non è scritto nelle fonti catturate [INF].
- EU5 permette di automatizzare quasi ogni sistema, così il giocatore sceglie dove passare il tempo (fonti/wikipedia-europa-universalis-v.md).
- Shadow Empire è a turni (fonti/wikipedia-shadow-empire.md).

## Cosa dicono i giocatori sul ritmo

- EU5: "the performance and speed of ticks is surprisingly fast", ma il gioco resta "shallow, non-strategic... clicking simulator" per lo stesso recensore (fonti/steam-recensioni-grand-strategy.md).
- EU4: "performance gradually falls off a cliff. By the 1600s, the game can become painfully slow" (fonti/steam-recensioni-grand-strategy.md).
- Victoria 3: "waiting about 5 minutes at a time for buildings to complete"; "Game becomes really slow in the end-game" (fonti/steam-recensioni-realismo.md).
- HOI4: "you enjoy playing until 1942 and then you realise you wasted 8 hours" (fonti/steam-recensioni-grand-strategy.md).
- Terra Invicta: "You do almost nothing then wait to do almost nothing again" (fonti/steam-recensioni-grand-strategy.md).
- Supreme Ruler: "Crazy Slow and I have put in over 300 hours", detto come lode (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: un calendario unico con tick giornaliero e sottosistemi a cadenze proprie (economia settimanale o mensile, politica mensile, leggi a checkpoint, elezioni a scadenza). È lo schema comune a tutti i Paradox e si sposa con la [[architettura-simulazione]] a tick fisso. Superato il 17/10/2026: il turno è un trimestre discreto, chiuso dal giocatore, con apertura, decisioni e chiusura (docs/ricerca/sintesi/decisioni.md).
- Prendiamo: pausa e quattro-cinque velocità; il giocatore decide il ritmo. Superato il 17/10/2026: niente tempo reale, quindi niente pausa né velocità (docs/ricerca/sintesi/decisioni.md).
- Prendiamo: processi lunghi (leggi, crisi, mobilitazione) con fasi visibili, così l'attesa è informativa e non vuota.
- Evitiamo: il rallentamento di fine partita. Il costo per tick deve restare costante al crescere della partita, cosa che EU4 e Victoria 3 non garantiscono.
- Evitiamo: tempi morti senza decisioni; i giocatori di Terra Invicta e Victoria 3 li citano come difetto.
- Aperto: l'arco della partita. Un mandato (5 anni) o più legislature (20-30 anni)? Decisione di prodotto da porre a Mario. Deciso il 17/10/2026: più legislature, e la sconfitta porta all'opposizione; obiettivo di 5-10 minuti per turno ordinario, circa 2-3 ore a legislatura (docs/ricerca/sintesi/decisioni.md).
- In tensione con [[macro-stock-flow-consistent]]: là «un turno = un trimestre», e i coefficienti di [[macro-neokeynesiano-piccolo]] sono stimati su trimestri; qui tick giornaliero con economia settimanale o mensile. [[ritardi-e-inerzia-delle-politiche]] propone un passo mensile o trimestrale. Le fonti non decidono; un passo più fine obbliga a riscalare i coefficienti. Chiusa il 17/10/2026 a favore del trimestre, senza riscalare i coefficienti (docs/ricerca/sintesi/decisioni.md).

## Collegamenti

- [[architettura-simulazione]]: tick fisso e snapshot.
- [[prestazioni-su-hardware-modesto]]: costo per tick.
- [[ritardi-e-inerzia-delle-politiche]]: quanto tempo serve a una politica per avere effetto.
- [[cicli-politico-economici-e-luna-di-miele]]: il calendario elettorale.
- [[hearts-of-iron-4]], [[victoria-3]], [[europa-universalis]], [[crusader-kings-3]], [[stellaris]], [[terra-invicta]], [[supreme-ruler]], [[shadow-empire]].
- [[macro-neokeynesiano-piccolo]]: coefficienti trimestrali da riscalare se il passo cambia.
- [[macro-stock-flow-consistent]]: il turno trimestrale del motore economico.

## Riferimenti

- **URL** https://vic3.paradoxwikis.com/Laws · [V] (snapshot 2026-08-19)
  **perche' conta qui** processo lungo con fasi e checkpoint
  **cosa se ne prende** checkpoint di 100 giorni
- **URL** https://en.wikipedia.org/wiki/Europa_Universalis_V · [V]
  **perche' conta qui** arco di 500 anni e automazione per sistema
  **cosa se ne prende** delega come gestione del tempo del giocatore
- **URL** https://en.wikipedia.org/wiki/Stellaris_(video_game) · [V]
  **perche' conta qui** partita divisa in fasi con eventi diversi
  **cosa se ne prende** tre fasi da 100 anni come struttura narrativa
- **URL** https://store.steampowered.com/appreviews/236850 · [V]
  **perche' conta qui** prestazioni che crollano col procedere della partita
  **cosa se ne prende** costo per tick costante come requisito
