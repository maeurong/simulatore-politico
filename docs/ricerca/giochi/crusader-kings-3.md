---
tipo: gioco
fonti: [fonti/ck3-wiki-faction.md, fonti/ck3-wiki-council.md, fonti/ck3-wiki-character.md, fonti/ck3-wiki-attributes.md, fonti/ck3-wiki-warfare.md, fonti/ck3-wiki-army.md, fonti/wikipedia-crusader-kings-iii.md, fonti/steam-ck3.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Crusader Kings III

Grand strategy dinastico di Paradox sul Medioevo. Il soggetto non è uno stato ma un personaggio: lo stato è la somma delle opinioni dei vassalli verso di lui. Per noi conta come modello di politica fatta di persone, con fazioni che si coalizzano contro il capo, un consiglio di ministri con compiti, e stress e legittimità del leader.

## Scala e loop

- Start nel 867, 1066 o 1178; alla morte o deposizione si continua con l'erede (fonti/wikipedia-crusader-kings-iii.md). La data di fine non è nelle fonti catturate [NON TROVATO].
- Loop: governa il dominio diretto → tieni contenti i vassalli che danno tasse e leve → usa il consiglio → combina matrimoni ed eredità → fai guerre per titoli → sopravvivi alle fazioni.
- Recensioni Steam: 90% positive su 50.263; 76% sulle ultime 367 (fonti/steam-ck3.md).

## Come modella lo stato

- I vassalli danno tasse e leve secondo il tipo di governo: feudale per contratto, tribale secondo la fama del sovrano, clan secondo l'opinione, teocrazia secondo la devozione, repubblica sempre 20% di tasse e 10% di leve (fonti/ck3-wiki-faction.md).
- Limite di vassalli diretti; oltre il limite, −5% su tasse e leve per ogni vassallo in eccesso, fino a −95% (fonti/ck3-wiki-faction.md).
- Vassalli potenti: quelli con più tasse e leve si aspettano un seggio in consiglio, altrimenti −40 di opinione verso il sovrano (fonti/ck3-wiki-faction.md).
- Opinione: ogni personaggio ha un'opinione di ogni altro, che decide il comportamento della IA. Alla morte del sovrano l'erede eredita il 25% dell'opinione positiva e il 50% di quella negativa (fonti/ck3-wiki-character.md).

## Fazioni

- Le fazioni si formano contro il sovrano per uno scopo; crescono finché il problema non è risolto e consegnano un ultimatum (fonti/ck3-wiki-faction.md).
- Due valori: potere militare (rapporto fra la forza combinata dei membri e quella del sovrano) e malcontento. Sopra l'80% di potere il malcontento sale, sotto scende. A 100% di malcontento parte l'ultimatum; se il sovrano accetta perde 20 di terrore, se rifiuta è guerra civile (fonti/ck3-wiki-faction.md).
- Non si unisce chi ha almeno 80 di opinione del sovrano, chi è amico o alleato, chi è sotto ricatto forte o terrorizzato (fonti/ck3-wiki-faction.md).

## Consiglio e personaggio

- Consiglio: un maestro delle spie per tutti, più quattro seggi regolari (cancelliere, tesoriere, maresciallo, consigliere per gli affari religiosi). Ogni consigliere ha un compito; cambiarlo azzera il progresso, tranne per alcuni compiti territoriali (fonti/ck3-wiki-council.md).
- Stress: sale quando il personaggio agisce contro i propri tratti di personalità; superare per la prima volta 100, 200 o 300 di stress provoca un crollo mentale di livello 1-3 (fonti/ck3-wiki-attributes.md).
- Terrore (dread) 0-100: rende intimiditi o terrorizzati i personaggi meno audaci, così le azioni diplomatiche riescono anche con opinione negativa (fonti/ck3-wiki-attributes.md).
- Legittimità del sovrano: cresce creando titoli, tenendo corte, con decisioni; cala perdendo titoli, accettando richieste delle fazioni, con atti tirannici e guerre perse (fonti/ck3-wiki-attributes.md).

## Guerra

- Punteggio di guerra da −100 a +100, a somma zero; si guadagna con battaglie, assedi, cattura del sovrano nemico (+100) o dell'erede (fonti/ck3-wiki-warfare.md).
- Il danno di un esercito si distribuisce sulle truppe nemiche in proporzione alla loro forza: le leve prendono il grosso delle perdite, poi uomini d'arme, infine cavalieri; +5% di danno per punto di vantaggio (fonti/ck3-wiki-army.md).

## Cosa dicono i giocatori sul realismo

- Lodato: "a dynastic systems simulator disguised as a strategy game", con scale micro (personalità, matrimoni, stress) e macro (istituzioni, legittimità, sviluppo) insieme (fonti/steam-recensioni-grand-strategy.md).
- Lodato: "one of the most realistic political strategy games", centrato sul governare più che sulla battaglia (fonti/steam-recensioni-grand-strategy.md).
- Criticato: il ciclo è "clicking through endless nested menus, spending arbitrary points to make some hidden modifier go from +3% to +5%" (fonti/steam-recensioni-grand-strategy.md).
- Criticato: sistemi aggiunti "bloated and gamey"; il gioco diventa troppo facile col tempo (fonti/steam-recensioni-grand-strategy.md).
- Riconosciuto: semplifica di proposito il realismo della guerra medievale per non intralciare la politica di corte (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: fazioni con due numeri, forza relativa e malcontento, e una soglia che fa scattare l'ultimatum. Si applica a correnti di partito, alleati di coalizione, forze armate.
- Prendiamo: ministri con un compito assegnato e un'aspettativa di seggio per chi conta. Si incrocia con [[ministri-e-gabinetto]].
- Prendiamo: l'opinione che si eredita in modo asimmetrico (il negativo pesa il doppio). Vale per un nuovo premier che eredita il partito.
- Evitiamo: modificatori nascosti a pochi punti percentuali che il giocatore non legge.
- Aperto: quanto personalizzare i personaggi. CK3 ne simula migliaia; a noi servono decine di figure (ministri, leader di partito, generali) [INF].

## Collegamenti

- [[personaggi-fazioni-e-stabilita-interna]]: il confronto delle fazioni fra titoli.
- [[ministri-e-gabinetto]]: pagina del filone politico.
- [[consiglieri-e-informazione-imperfetta]]: come i consiglieri filtrano l'informazione.
- [[shadow-empire]]: consiglio di personaggi in un wargame.

## Riferimenti

- **URL** https://ck3.paradoxwikis.com/Faction · [V] (pagina Vassals, snapshot Wayback 2024-10-08)
  **perche' conta qui** fazioni con soglia di potere e malcontento
  **cosa se ne prende** soglia 80% di forza relativa, ultimatum a 100% di malcontento
- **URL** https://ck3.paradoxwikis.com/Council · [V] (snapshot 2026-06-24)
  **perche' conta qui** ministri con compiti e progressi
  **cosa se ne prende** un compito per ministro, progresso perso al cambio
- **URL** https://ck3.paradoxwikis.com/Opinion · [V] (pagina Character, snapshot 2026-09-18)
  **perche' conta qui** opinione come motore della IA politica
  **cosa se ne prende** eredità asimmetrica dell'opinione
- **URL** https://store.steampowered.com/appreviews/1158310 · [V]
  **perche' conta qui** dove i giocatori vedono realismo e dove arbitrarietà
  **cosa se ne prende** evitare i modificatori nascosti
