---
tipo: gioco
fonti: [fonti/tropico6-wiki-costituzione.md, fonti/tropico6-wiki-editti.md, fonti/tropico-wiki-elezioni.md, fonti/tropico6-wiki-discorso-elettorale.md, fonti/tropico-wiki-felicita.md, fonti/steam-tropico6.md, fonti/steam-api-snapshot-gestionali.md]
aggiornato: 2026-10-16
---
# Tropico 6

City builder con presidente-dittatore (Limbic, 2019) su un'isola caraibica attraverso quattro ere. È il gestionale che più somiglia a un simulatore politico: ogni cittadino vota, appartiene a fazioni, e il giocatore governa con editti, costituzione e discorsi elettorali. Conta per noi perché mostra come rendere tangibili leggi ed elezioni dentro una città visibile, e dove il tono satirico copre semplificazioni forti.

## Scheda

| Campo | Valore |
|---|---|
| Uscita | 29/03/2019, Windows/macOS/Linux (fonti/steam-api-snapshot-gestionali.md) |
| Recensioni Steam | 29.611, 88% positive (fonti/steam-api-snapshot-gestionali.md) |
| Requisiti minimi | CPU 3 GHz (i3-2000), 8 GB RAM, GPU dedicata 2 GB (GTX 750) (fonti/steam-api-snapshot-gestionali.md) |

## Scala e loop

Un'isola, una popolazione di cittadini simulati uno per uno: ciascuno ha felicità, fazioni e voto personale. Il loop alterna costruzione (produzione, servizi, turismo) e politica (fazioni, elezioni, superpotenze). Le elezioni si perdono: sconfitta = fine partita (fonti/tropico-wiki-elezioni.md; fonti/tropico6-wiki-discorso-elettorale.md).

## Come modella il governo

**Felicità.** In Tropico 6 la felicità del cittadino è composta da otto voci: cibo, sanità, svago, fede, abitazione, lavoro, libertà, sicurezza dal crimine. Il cittadino infelice emigra, vota contro, protesta, si unisce ai ribelli (fonti/tropico-wiki-felicita.md).

**Elezioni.** Si vota a ogni elezione su base individuale: felicità personale più rapporti delle proprie fazioni col presidente. Lo sfidante è un cittadino con alta leadership (fonti/tropico-wiki-elezioni.md; pagina di serie, non specifica di Tropico 6). Le elezioni sono chieste ogni 10 anni; si possono rinviare, ma il rinvio pesa sull'approvazione di ogni elettore (fonti/tropico6-wiki-discorso-elettorale.md). Se si perde, il tribunale elettorale offre di truccare il voto, con costo in rapporti con gli Intellettuali e in "aspettative democratiche" (fonti/tropico-wiki-elezioni.md).

**Discorso elettorale.** Quattro mosse componibili: riconoscere un problema (gli scontenti di quel problema non ne tengono conto al voto), lodare una fazione (+10 di rapporto), attaccare una superpotenza, promettere un miglioramento; la promessa mancata si paga all'elezione successiva (fonti/tropico-wiki-elezioni.md; fonti/tropico6-wiki-discorso-elettorale.md).

**Editti.** Leggi a tempo con costo una tantum o ricorrente, effetti positivi pagati con effetti negativi, e un livello che cresce (0-2 stelle) finché restano attivi (fonti/tropico6-wiki-editti.md). Esempi con numeri:
- Legge marziale: 7.500 $, annulla e impedisce le elezioni, libertà −35/−30/−25 per livello, +15% probabilità di diventare ribelli, militaristi +30, tutte le altre fazioni −15 (fonti/tropico6-wiki-editti.md).
- Elezioni anticipate: 2.000 $, voto fra 12 mesi, senza discorso (fonti/tropico6-wiki-editti.md).
- Taglio delle tasse: 5 $ per occupato, approvazione di tutti in su, cooldown 5 anni (fonti/tropico6-wiki-editti.md).
- Patrimoniale: +5/6/7 $ al mese per adulto ricco, capitalisti −20, comunisti +15 (fonti/tropico6-wiki-editti.md).
- Divieto di assemblea: chiude le proteste, libertà −15, rischio ribelli +15, 1 $ per cittadino al mese (fonti/tropico6-wiki-editti.md).

**Costituzione.** Scelte permanenti a paragrafi, sbloccate dopo l'indipendenza, quattro per era (due subito, due con la ricerca) (fonti/tropico6-wiki-costituzione.md). Ogni paragrafo ha alternative mutuamente esclusive con effetti numerici: diritto di voto (universale, solo benestanti con +10 rischio ribelli fra i poveri, voto palese per i dipendenti pubblici), forze armate (stato pacifista: −75% minaccia ribelle e caserme disattivate), religione (teocrazia: −10 libertà, chiese che arrestano; stato ateo: +50% efficienza istruzione e ricerca), lavoro, ecologia (zero emissioni: −75% inquinamento, −10% efficienza industriale) (fonti/tropico6-wiki-costituzione.md).

**Fazioni.** Otto fazioni in coppie opposte (capitalisti/comunisti, conservatori/intellettuali, religiosi/militaristi, industriali/ambientalisti); quasi ogni editto o paragrafo sposta due fazioni in direzioni opposte (fonti/tropico6-wiki-editti.md; fonti/tropico6-wiki-costituzione.md) [INF: lo schema a coppie si ricava dagli effetti elencati; la pagina wiki delle fazioni di Tropico 6 è uno stub].

## Realismo: cosa dicono i giocatori

Nella stampa citata sullo store prevale "city builder con personalità e umorismo" più che "simulazione profonda" (fonti/steam-tropico6.md). Il campione di recensioni filtrate per realismo di Tropico 6 è nella fonte dell'agente dei giochi politici, `fonti/steam-recensioni-realismo.md` [rimando, non riletto qui].

## Prestazioni

Requisiti minimi modesti per il 2019 (8 GB, GPU 2 GB). Dati di frame rate su hardware debole [NON TROVATO].

## Per il nostro gioco

- **Prendiamo:** l'editto come oggetto con costo, durata, livello che matura e doppio effetto su due gruppi opposti; la costituzione come scelta strutturale lenta, separata dall'editto rapido; il discorso elettorale come mossa di agenda ("riconosci il problema e quel problema non pesa sul voto"); il voto individuale che somma felicità personale e lealtà di gruppo.
- **Evitiamo:** sconfitta elettorale = game over; i numeri tondi (+10, −15) che rendono leggibile ma trasparente la manipolazione; il trucco elettorale offerto come pulsante.
- **Aperto:** quante fazioni reggono la leggibilità (Tropico ne usa otto, a coppie).

## Collegamenti

- [[leggi-decreti-ed-editti]]: editti e costituzione a confronto con libro delle leggi di Frostpunk e ordinanze di SimCity.
- [[cittadini-simulati]]: Tropico è il caso "ogni cittadino vota".
- [[elettorato-e-gruppi-di-voto]]: fazioni e voto individuale.
- [[democracy-4]]: l'altro modello di policy a costo ricorrente, senza città.
- [[campagna-elettorale]]: discorso e promesse.
