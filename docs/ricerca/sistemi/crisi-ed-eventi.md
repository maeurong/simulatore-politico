---
tipo: sistema
fonti: [fonti/democracy4-mod-situations.md, fonti/democracy3-modding.md, fonti/hoi4-wiki-event-modding.md, fonti/rimworld-wiki-ai-storytellers.md, fonti/democracy4-faq.md, fonti/wikipedia-plague-inc.md, fonti/wikipedia-papers-please.md]
aggiornato: 2026-10-17
---
# Crisi ed eventi: scriptati o emergenti

Un evento è credibile quando ha una causa nello stato del mondo. I giochi studiati usano quattro meccanismi, combinabili: situazioni a soglia, dilemmi pesati, eventi a trigger con tempo medio, regia adattiva. Proposta in fondo.

## 1. Situazioni a soglia con isteresi (Democracy 4)
- Una «situazione» (crisi sanitaria, ondata di crimine, boom…) è una riga di CSV con ingressi: la somma degli effetti in ingresso la accende sopra una soglia di avvio e la spegne sotto una soglia di arresto distinta (fonti/democracy4-mod-situations.md).
- Mentre è attiva ha costi e ricavi per turno e produce effetti su elettori, statistiche e altre situazioni (fonti/democracy4-mod-situations.md).
- [INF] Le due soglie distinte danno isteresi: una crisi non lampeggia on/off attorno al valore critico. È il meccanismo più economico per crisi emergenti.

## 2. Dilemmi pesati dallo stato (Democracy 3)
- Ogni dilemma ha influenze in ingresso che ne calcolano la probabilità a ogni turno; parte quello in cima, se nello stesso turno non è già partito un evento (fonti/democracy3-modding.md).
- Esempio: probabilità = 0,8 − 0,6 × Salute: più sanità, meno probabilità del dilemma; c'è anche un'influenza casuale esplicita (fonti/democracy3-modding.md).
- Le due opzioni eseguono script; tipicamente creano un «grudge» con effetto e decadimento (fonti/democracy3-modding.md).

## 3. Trigger + tempo medio (Hearts of Iron IV, motore Paradox)
- L'evento ha un blocco `trigger` (condizioni) controllato ogni 20 giorni; se è vero, si passa al controllo giornaliero del `mean_time_to_happen`. Con M giorni, la probabilità giornaliera è 1 − 2^(−1/M): M è un tempo mediano (fonti/hoi4-wiki-event-modding.md).
- M si modifica con fattori condizionali (es. `factor = 0.2` se una condizione vale) (fonti/hoi4-wiki-event-modding.md).
- Altri attributi utili: `fire_only_once`, `is_triggered_only` (solo da altri eventi o decisioni), `timeout_days` (scelta automatica se il giocatore non decide), eventi `hidden` per la logica interna, `news_event` per la notizia mostrata a tutti (fonti/hoi4-wiki-event-modding.md).

## 4. Regista adattivo (RimWorld)
- L'«AI storyteller» decide frequenza e tipo degli eventi in base a ricchezza della colonia, numero di coloni, morti o feriti recenti e tempo dall'ultimo evento maggiore (fonti/rimworld-wiki-ai-storytellers.md).
- Tre personalità: Cassandra (tensione crescente classica), Phoebe (lunghe pause fra i disastri), Randy (casuale, a volte ingiusto) (fonti/rimworld-wiki-ai-storytellers.md).
- Adattamento: un punteggio cresce col tempo quando il giocatore va bene e cala quando subisce danni; alza la difficoltà (fonti/rimworld-wiki-ai-storytellers.md).

## Eventi scriptati
- Papers, Please e Not For Broadcast usano una trama scriptata con molti finali; Papers, Please ne ha 20, più modalità infinite randomizzate (fonti/wikipedia-papers-please.md).
- Democracy 4 difende la plausibilità dell'evento estremo (l'assassinio) legandolo a cause nel modello: cambi radicali troppo veloci, sicurezza scarsa, gruppi terroristici attivi (fonti/democracy4-faq.md). È il criterio di credibilità.

## Proposta [INF]
- Crisi lente (recessione, ondata di criminalità, epidemia, siccità) = situazioni a soglia con isteresi, alimentate dai modelli di [[epidemie-sir-seir]], [[clima-ed-energia]], [[criminalita-e-giustizia]].
- Eventi puntuali (scandali, disastri, attentati, dimissioni) = trigger + tempo mediano alla Paradox, con M funzione dello stato (corruzione alta → M degli scandali più breve).
- Un regista leggero, alla RimWorld, che allunga M dopo una crisi grave e lo accorcia dopo lunghe calme, con preset per il giocatore.
- Ogni evento cita la sua causa nel rapporto del consigliere ([[consiglieri-e-informazione-imperfetta]]).

Decisione del 17/10/2026 ([[decisioni]], ticket wayfinder 15): nella v1 entrano i primi due punti; il regista adattivo resta fuori, perché è una causa esterna al modello, e lo sostituisce un tetto di un evento puntuale nuovo per turno.

## Collegamenti
- [[democracy-4]]: situazioni e dilemmi.
- [[plague-inc]]: l'evento dirompente sul mondo vero (il gioco fu ritirato in Cina nel febbraio 2020) (fonti/wikipedia-plague-inc.md).
- [[ritardi-e-inerzia-delle-politiche]]: decadimento dei grudges.
- [[media-informazione-e-propaganda]]: gli eventi spostano la salienza.
- [[deterrenza-ed-escalation]]: la crisi internazionale come scala di escalation.
