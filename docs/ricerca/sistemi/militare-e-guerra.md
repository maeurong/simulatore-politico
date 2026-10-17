---
tipo: sistema
fonti: [fonti/hoi4-wiki-production.md, fonti/hoi4-wiki-supply.md, fonti/hoi4-wiki-land-battle.md, fonti/hoi4-wiki-manpower.md, fonti/hoi4-wiki-battle-plan.md, fonti/hoi4-wiki-land-doctrine.md, fonti/hoi4-wiki-government.md, fonti/vic3-wiki-war-estratto.md, fonti/eu4-wiki-manpower.md, fonti/eu4-wiki-land-warfare.md, fonti/eu5-wiki-military.md, fonti/eu5-wiki-warfare.md, fonti/ck3-wiki-army.md, fonti/ck3-wiki-warfare.md, fonti/stellaris-wiki-warfare.md, fonti/terrainvicta-wiki-gdp.md, fonti/gamepretty-shadow-empire-logistica.md, fonti/steam-recensioni-grand-strategy.md, fonti/steam-recensioni-realismo.md]
aggiornato: 2026-10-16
---
# Militare e guerra

Come i grand strategy modellano produzione bellica, logistica, fronti, manpower e dottrina, e come decidono quando una guerra finisce. Il confronto serve a scegliere a che livello di dettaglio il nostro ministero della Difesa deve funzionare: il giocatore governa un paese, non comanda divisioni.

## Produzione

| Gioco | Unità di produzione | Cosa la limita |
|---|---|---|
| HOI4 | fabbrica militare, 3,5/giorno × efficienza (10-50%+) | risorse strategiche (−5% per unità mancante), riconversione che fa perdere efficienza (fonti/hoi4-wiki-production.md) |
| Victoria 3 | edifici delle unità che consumano beni | le carenze di beni riducono il rifornimento delle formazioni (fonti/vic3-wiki-war-estratto.md) |
| EU5 | leve tratte dalla popolazione | alzare leve toglie popolazione, scioglierle la restituisce (fonti/eu5-wiki-military.md) |
| Terra Invicta | priorità "costruisci esercito" pagata in IP | ogni esercito e flotta riduce gli IP futuri (fonti/terrainvicta-wiki-gdp.md) |

- La curva di efficienza di HOI4 (crescita 0,001 × tetto² / efficienza al giorno, al massimo 500 giorni per il tetto) è il modo più semplice trovato per rendere costosa la riconversione industriale (fonti/hoi4-wiki-production.md).
- Il costo ricorrente delle forze armate è esplicito in Terra Invicta: tenerle sottrae budget, non solo costruirle (fonti/terrainvicta-wiki-gdp.md).

## Logistica

- HOI4: rifornimento come capacità per provincia, da hub collegati per ferrovia alla capitale (15-35 per livello), decrescente con la distanza; sotto il 35% scatta l'attrito; isolati dalla capitale niente rinforzi (fonti/hoi4-wiki-supply.md).
- Victoria 3: rifornimento sotto il 95% toglie la stessa percentuale di organizzazione massima; il raid sui convogli riduce l'efficienza (fonti/vic3-wiki-war-estratto.md).
- EU5: attrito mensile se l'esercito è senza cibo o se il peso delle unità supera il limite di rifornimento della località; inverno, terreno, clima (fonti/eu5-wiki-military.md).
- Shadow Empire: risorse che richiedono capacità di trasporto (cibo, munizioni, manpower, macchinari) contro risorse che basta collegare (fonti/gamepretty-shadow-empire-logistica.md).
- Giocatori: la logistica di HOI4 è citata come la parte più realistica, con la riserva che tutto debba collegarsi alla capitale (fonti/steam-recensioni-grand-strategy.md).

## Fronti e comando

- HOI4: il giocatore disegna linea del fronte e linea offensiva; la IA esegue il piano e accumula un bonus di pianificazione (fonti/hoi4-wiki-battle-plan.md).
- Victoria 3: fronti generati dai confini fra teatri, avanzata di uno stato alla volta; il giocatore assegna generali con ordini di avanzata o difesa, e l'avanzata scatta a 100 punti di progresso (fonti/vic3-wiki-war-estratto.md).
- Giocatori su Victoria 3: "100k vs 20k with the same stats doesn't provide the expected outcome"; la guerra è il sistema più criticato del gioco (fonti/steam-recensioni-realismo.md).

## Risoluzione del combattimento

| Gioco | Granularità | Meccanica |
|---|---|---|
| HOI4 | turni di un'ora | attacchi contro difese, 10% o 40% di colpire, dadi da 2 e 4 (fonti/hoi4-wiki-land-battle.md) |
| EU4 | fasi di 3 giorni fuoco/urto | un dado per lato a fase; perdite base = 15 + 5 × (dado + bonus + ...) (fonti/eu4-wiki-land-warfare.md) |
| CK3 | per battaglia | danno totale distribuito sulle truppe in proporzione alla forza; +5% per punto di vantaggio (fonti/ck3-wiki-army.md) |
| Victoria 3 | per tick | minimo 5 perdite per unità per tick; morti fra 0,1% e 0,5% (fonti/vic3-wiki-war-estratto.md) |

Nessuno dei titoli usa le equazioni di Lanchester in forma esplicita nelle fonti trovate [NON TROVATO]; la pagina [[lanchester-e-modelli-di-combattimento-aggregati]] le propone come alternativa aggregata.

## Manpower

- HOI4: popolazione reclutabile per stato secondo la legge di coscrizione; i territori non nucleo contribuiscono pochissimo (−98%) (fonti/hoi4-wiki-manpower.md).
- EU4: 250 uomini per punto di sviluppo militare; si ricostituisce da zero in 10 anni (fonti/eu4-wiki-manpower.md).
- Victoria 3: coscritti attivati armata per armata durante crisi o guerra, dal bacino fissato dalle leggi sull'esercito e dalla popolazione; −50% negli stati non incorporati (fonti/vic3-wiki-war-estratto.md).
- EU5: le leve sottraggono popolazione reale, quindi la guerra costa crescita demografica (fonti/eu5-wiki-military.md).

## Dottrina

- HOI4: alberi di dottrina con bonus fissi a combattimento e organizzazione (fonti/hoi4-wiki-land-doctrine.md). Gli altri titoli la incorporano in tecnologie o tratti dei comandanti [INF].

## Fine della guerra e costo interno

| Gioco | Contatore | Soglia |
|---|---|---|
| Victoria 3 | war support da +100 | capitolazione automatica a −100; occupazione >90% dà +10 a settimana di esaurimento (fonti/vic3-wiki-war-estratto.md) |
| EU5 | stanchezza di guerra | ogni punto: −2% morale, −1% produzione e commercio, −0,03% crescita; −0,1 al mese in pace (fonti/eu5-wiki-warfare.md) |
| CK3 | war score −100/+100, somma zero | catturare il sovrano nemico vale +100 (fonti/ck3-wiki-warfare.md) |
| Stellaris | accettazione della resa | stanchezza, occupazione, rapporto di flotte; possibile status quo (fonti/stellaris-wiki-warfare.md) |
| HOI4 | supporto alla guerra | −30% limite di resa al minimo; guerra offensiva −20, difensiva +20 (fonti/hoi4-wiki-government.md) |

## Per il nostro gioco

- Prendiamo: produzione bellica come capacità industriale con costo di riconversione (HOI4) e costo ricorrente che toglie budget (Terra Invicta).
- Prendiamo: logistica come vincolo aggregato per regione, alimentato dall'infrastruttura, non come minigioco.
- Prendiamo: un contatore di consenso alla guerra separato dalla stabilità, con soglie che costringono alla pace. È anche la cerniera con il voto: si collega a [[costo-politico-della-guerra]].
- Prendiamo: manpower legato alla popolazione e alle leggi di leva, con costo demografico (EU5).
- Evitiamo: il combattimento tattico a dado. Il giocatore è un governo; riceve esiti aggregati spiegati (rapporto di forze, rifornimento, morale).
- Aperto: dare al giocatore un "piano" come in HOI4 (obiettivi, priorità di fronte) o solo leve politiche (budget, leva, alleanze). Decisione di prodotto.

## Collegamenti

- [[hearts-of-iron-4]], [[victoria-3]], [[europa-universalis]], [[crusader-kings-3]], [[stellaris]], [[supreme-ruler]], [[terra-invicta]], [[shadow-empire]].
- [[lanchester-e-modelli-di-combattimento-aggregati]]: modello aggregato del combattimento (filone militare).
- [[costo-politico-della-guerra]]: effetto delle perdite sul consenso.
- [[prontezza-e-spesa-per-la-difesa]]: spesa e prontezza delle forze armate.
- [[deterrenza-ed-escalation]]: quando una crisi diventa guerra.
- [[logistica-e-trasporti]]: trasporto civile.
- [[produzione-industriale-e-risorse]]: la base industriale.
- [[controinsurrezione-e-stabilita]]: la guerra interna e le missioni di stabilizzazione.

## Riferimenti

- **URL** https://hoi4.paradoxwikis.com/Supply · [V] (snapshot 2026-08-28)
  **perche' conta qui** logistica come capacità per nodo
  **cosa se ne prende** capacità che cala con la distanza e soglia d'attrito
- **URL** https://vic3.paradoxwikis.com/War · [V] (estratto WebFetch)
  **perche' conta qui** fronti, mobilitazione, capitolazione a contatore
  **cosa se ne prende** war support +100/−100 e soglie di occupazione
- **URL** https://eu5.paradoxwikis.com/Warfare · [V]
  **perche' conta qui** costo della guerra su economia e demografia
  **cosa se ne prende** stanchezza con effetti per punto
- **URL** https://eu4.paradoxwikis.com/Land_warfare · [V] (snapshot 2026-07-21)
  **perche' conta qui** esempio di formula di perdite
  **cosa se ne prende** il contro-esempio di granularità
