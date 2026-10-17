---
tipo: gioco
fonti: [fonti/glassbox-gamedeveloper-gdc2012.md, fonti/glassbox-willmott-talk.md, fonti/simcity2013-mcv-risposta-ia.md, fonti/simcity2013-pcgamer-dimensioni.md, fonti/simcity2013-wiki-gioco.md, fonti/simcity2013-wiki-interfaccia.md]
aggiornato: 2026-10-16
---
# SimCity (2013) e il motore GlassBox

Reboot Maxis/EA del 2013, costruito sul motore GlassBox, dove ogni Sim, veicolo e unità di risorsa è un agente. È il caso documentato di un gioco ad agenti che per stare nei limiti di calcolo ha dovuto rimpicciolire le città e svuotare gli agenti di identità persistente. È la lezione più diretta per scegliere fra agenti e aggregati. Titolo aggiunto da questo filone.

## Scheda

| Campo | Valore |
|---|---|
| Uscita | 2013, connessione sempre attiva obbligatoria al lancio (fonti/simcity2013-wiki-gioco.md) |
| Dimensione della città | 2 × 2 km, come le città medie di SimCity 4 (fonti/simcity2013-wiki-gioco.md; fonti/simcity2013-pcgamer-dimensioni.md) |
| Accoglienza | La più bassa della serie; server, problemi tecnici e online obbligatorio (fonti/simcity2013-wiki-gioco.md) |

## Come modella la cosa: GlassBox

Presentato alla GDC 2012 da Andrew Willmott (fonti/glassbox-willmott-talk.md). Quattro tipi di oggetto collegati da regole scritte negli script (fonti/glassbox-gamedeveloper-gdc2012.md):

- **Risorse** in contenitori a capacità (acqua, elettricità, cibo, persone).
- **Unità**: case, fabbriche, persone; lo stato di un'unità è un insieme di contenitori (una casa ha 20/30 di cibo e 4/10 persone).
- **Mappe**: risorse distribuite sul territorio (petrolio, inquinamento, valore del suolo, desiderabilità).
- **Globali**: contenitori della città (denaro, elettricità usata).
- **Regole**: i verbi; spostano o combinano risorse con ingressi, uscite e frequenza. Esempio: una regola di unità consuma Simoleon e materie prime, produce un prodotto e scarica 5 di inquinamento sulla mappa.
- **Agenti**: oggetti mobili che portano risorse da un'unità all'altra e non eseguono regole, per poterne tenere "10.000+" in movimento; una persona, un'auto, ma anche un'unità d'acqua in un tubo (fonti/glassbox-gamedeveloper-gdc2012.md).

## Cosa è andato storto

- **Città piccole per scelta di prestazioni.** Il direttore creativo Ocean Quigley: "Il gioco deve girare bene su computer normali", meglio città dense e dettagliate che estese e "inerti"; la dimensione è "questione di compromessi" (fonti/simcity2013-pcgamer-dimensioni.md).
- **Agenti senza identità.** Dopo l'uscita, il lead designer Stone Librande ammette che i Sim non possiedono una casa né hanno un impiego fisso, e che il gioco non ne traccia nome e genere. Persistono invece felicità, denaro, salute e istruzione. È stato fatto "per aumentare le prestazioni e avere più Sim", perché "il costo di quel livello di micro-dettaglio non rendeva più ricco il macro". Gli agenti prendevano sempre il percorso più breve, creando ingorghi "illogici" (fonti/simcity2013-mcv-risposta-ia.md).
- **Online obbligatorio.** Server instabili al lancio e regioni multigiocatore imposte; la modalità offline arriva con l'Update 10 del 2014 (fonti/simcity2013-wiki-gioco.md).

## Feedback visivo

Ogni strumento di costruzione apre il suo pannello con le mappe pertinenti: l'acqua mostra falda, inquinamento del suolo, germi e salute; la polizia mostra crimine e istruzione; il governo mostra densità, felicità e popolazione (fonti/simcity2013-wiki-interfaccia.md).

## Per il nostro gioco

- **Prendiamo:** il vocabolario di GlassBox (contenitori di risorse su unità, mappe e globali, regole dichiarative con ingressi e uscite) come forma possibile di un modello aggregato e moddabile; le mappe contestuali allo strumento in uso.
- **Evitiamo:** l'agente individuale come promessa di realismo, quando poi va svuotato di identità per stare nei tempi; il vincolo online.
- **Aperto:** se le regole in stile GlassBox (dati, non codice) possano descrivere politiche e ministeri come descrivono fabbriche.

## Collegamenti

- [[cittadini-simulati]]: il controesempio principale.
- [[simcity-4]]: il predecessore a regioni.
- [[cities-skylines]]: il successore spirituale nato dal flop.
- [[modding-e-dati-esterni]]: le regole come dati.
