---
tipo: gioco
fonti: [fonti/stellaris-wiki-resources.md, fonti/stellaris-wiki-pops.md, fonti/stellaris-wiki-factions.md, fonti/stellaris-wiki-planetary-management.md, fonti/stellaris-wiki-diplomacy.md, fonti/stellaris-wiki-galactic-community.md, fonti/stellaris-wiki-warfare.md, fonti/stellaris-dd366-4-0-pop-groups.md, fonti/stellaris-dd370-4-0-cambiamenti.md, fonti/wikipedia-stellaris.md, fonti/steam-stellaris.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Stellaris

4X/grand strategy spaziale di Paradox, in tempo reale con pausa. Non è verosimile per costruzione, ma serve per due cose: un parlamento sovranazionale a voto pesato (la Galactic Community, un'ONU giocabile) e la lezione più documentata del genere sulle prestazioni: la popolazione simulata per individuo ha rallentato il gioco per anni, e nel 2025 Paradox l'ha raggruppata.

## Scala e loop

- Inizia il 1° gennaio 2200; partita standard di 300 anni in tre fasi da 100 (inizio, metà, fine) che fanno scattare eventi diversi (fonti/wikipedia-stellaris.md).
- Economia su sei risorse principali (crediti energetici, minerali, cibo, beni di consumo, leghe e commercio), più risorse strategiche (fonti/wikipedia-stellaris.md, fonti/stellaris-wiki-resources.md). Il valore commerciale si converte ogni mese (fonti/stellaris-wiki-resources.md). **Superato**: la cattura è del settembre 2024, precedente alla 4.0, che ha ridotto il commercio a risorsa standard (fonti/stellaris-dd366-4-0-pop-groups.md) [V].
- Loop: esplora → colonizza → costruisci distretti → i pop riempiono lavori → produci risorse → espanditi con diplomazia o guerra.
- Recensioni Steam: 86% positive su 76.745; 68% sulle ultime 351 (fonti/steam-stellaris.md).

## Popolazione: dai pop ai gruppi (4.0, 2025)

- Il sistema Pop e Lavori della 2.2 "ha sempre avuto implicazioni pesanti di prestazioni a fine partita" (fonti/stellaris-dd366-4-0-pop-groups.md).
- Con la 4.0 i pop si raggruppano per specie, strato ed etica; i gruppi producono forza lavoro che riempie (anche in parte) i lavori. La scala cambia: ciò che prima toccava 1 pop ora ne tocca 100 (fonti/stellaris-dd366-4-0-pop-groups.md).
- Ogni pop genera 1 unità di forza lavoro di default; "invece di tracciare i singoli pop tracciamo la forza lavoro che riempie i lavori", per ridurre i calcoli mensili (fonti/stellaris-dd370-4-0-cambiamenti.md).
- Anche il commercio, con calcoli di percorso continui, aveva "un impatto sproporzionato sulle prestazioni rispetto al beneficio": ridotto a risorsa semplice (fonti/stellaris-dd366-4-0-pop-groups.md).
- Le pagine wiki su pop e risorse catturate qui sono in parte precedenti alla 4.0 (snapshot 2024-09 per le risorse); i dettagli numerici del vecchio sistema non valgono più [INF].

## Fazioni e stabilità

- Approvazione della fazione 0-100, base 50, mossa dalle questioni soddisfatte o ignorate. Dà felicità ai pop membri: da −40% (0-19) a +10% (80-100) (fonti/stellaris-wiki-factions.md).
- Le fazioni producono unità in proporzione al potere politico dei membri e all'approvazione (fonti/stellaris-wiki-factions.md).
- Leve del governo: abbracciare una fazione sposta l'etica dell'impero verso di lei (5000 unità, +10% approvazione a lei, −35% alle altre per 10 anni); sopprimerla toglie il 50% di approvazione e spinge i pop a uscirne (fonti/stellaris-wiki-factions.md).
- La felicità dei pianeti dipende dalle amenità, con effetto non lineare: la mancanza pesa più dell'eccesso (fonti/stellaris-wiki-planetary-management.md).

## Diplomazia: la Galactic Community

- Si forma quando un impero ha contatti con almeno il 70% degli altri (fonti/stellaris-wiki-galactic-community.md).
- Il Senato alterna recesso e sessione. Alla fine del recesso va in sessione la risoluzione con più sostegno; in sessione ogni impero vota a favore, contro o si astiene con il proprio peso diplomatico, e vince il lato con il peso totale maggiore (fonti/stellaris-wiki-galactic-community.md).
- Il peso diplomatico viene da flotta, economia, tecnologia e popolazione (fonti/stellaris-wiki-galactic-community.md). I membri del Consiglio galattico possono dichiarare una misura d'emergenza ogni 20 anni; le sanzioni colpiscono chi viola la legge galattica (fonti/stellaris-wiki-galactic-community.md).

## Guerra

- Ogni belligerante dichiara obiettivi con un costo di "accettazione della resa"; ci si arriva con stanchezza di guerra, occupazione e rapporto di flotte. Una guerra può finire senza vincitori (fonti/stellaris-wiki-warfare.md).
- Resa: si applica l'obiettivo del vincitore e i territori rivendicati passano di mano. Status quo: ognuno tiene i sistemi occupati su cui ha rivendicazioni (fonti/stellaris-wiki-warfare.md).

## Cosa dicono i giocatori

- Criticato: le patch 4.0 e 4.3 riducono flotte e rendite per prestazioni e bilanciamento; "if a feature causes problems, just remove it" (fonti/steam-recensioni-grand-strategy.md).
- Criticato: "it's not at all complex... It's just VOLUMINOUS": acquisisci risorse, spendi risorse, ricerca per averne di più (fonti/steam-recensioni-grand-strategy.md).
- Lodato: "a very good simulation of diplomacy, empire building, resource and population management" (fonti/steam-recensioni-grand-strategy.md).
- Criticati da molti: prezzi dei DLC, prestazioni di fine partita, raccolta dati (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: popolazione in gruppi aggregati (classe, regione, orientamento) e non in individui. È la stessa conclusione che Paradox ha raggiunto dopo anni di problemi; per un gioco "leggero" si parte da lì.
- Prendiamo: un'assemblea internazionale a voto pesato con potere derivato da economia, forze armate e popolazione. Si adatta a ONU, UE, NATO.
- Prendiamo: approvazione delle fazioni 0-100 con effetto a fasce sulla soddisfazione.
- Evitiamo: sistemi calcolati per entità e per percorso a ogni tick se il beneficio non si vede.
- Aperto: l'ambientazione spaziale non ci serve; la pagina resta per le due lezioni sopra.

## Collegamenti

- [[diplomazia-e-relazioni-internazionali]]: confronto con HOI4 e Victoria 3.
- [[personaggi-fazioni-e-stabilita-interna]]: fazioni a confronto.
- [[mercati-e-pops]]: pop individuali contro gruppi.
- [[prestazioni-su-hardware-modesto]]: pagina del filone tecnico.

## Riferimenti

- **URL** https://forum.paradoxplaza.com/forum/developer-diary/stellaris-dev-diary-366-announcing-stellaris-4-0.1726042/ · [V] (snapshot Wayback)
  **perche' conta qui** prova di prima mano che i pop individuali costano troppo
  **cosa se ne prende** pop in gruppi e commercio come risorsa semplice
- **URL** https://forum.paradoxplaza.com/forum/developer-diary/stellaris-dev-diary-370-4-0-changes-part-4.1728047/ · [V] (snapshot Wayback)
  **perche' conta qui** dettaglio della forza lavoro
  **cosa se ne prende** 1 pop = 1 unità di forza lavoro, meno calcoli mensili
- **URL** https://stellaris.paradoxwikis.com/Galactic_community · [V] (snapshot 2026-08-13)
  **perche' conta qui** parlamento sovranazionale giocabile
  **cosa se ne prende** voto pesato e ciclo recesso-sessione
- **URL** https://stellaris.paradoxwikis.com/Factions · [V] (snapshot 2026-01-26)
  **perche' conta qui** approvazione con effetti a fasce
  **cosa se ne prende** base 50, fasce da −40% a +10%
