---
tipo: modello
fonti: [fonti/shirani-mehr-2018-bias-varianza-sondaggi.md, fonti/nature-jennings-wlezien-2018.md, fonti/soton-jennings-wlezien-2018-comunicato.md, fonti/agcom-delibera-121-18-sondaggi.md, fonti/steam-democracy4-voting-systems.md]
aggiornato: 2026-10-16
---
# Sondaggi ed errore

Come generare sondaggi credibili a partire dallo stato vero della simulazione. Il punto chiave: l'errore reale dei sondaggi è circa il doppio del margine dichiarato, e una parte è comune a tutti gli istituti (bias della singola elezione), quindi non si cancella facendo la media. Per il gioco il sondaggio è la finestra principale sull'opinione: deve sbagliare come sbagliano quelli veri, né di più né di meno.

## Modello generativo

Per l'istituto k, il partito j, la data t, con quota vera `p_j(t)` dalla simulazione:

`sondaggio = p_j(t) + b_j(e) + h_jk + ε`, con `ε ~ N(0, d · p(1−p)/n)`

- `b_j(e)`: bias comune a tutti i sondaggi di quell'elezione, estratto una volta per legislatura e rivelato solo dal voto. Shirani-Mehr e coautori lo stimano esplicitamente come componente di livello elezione, che può variare linearmente nel tempo (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- `h_jk`: effetto istituto (house effect), persistente [INF: non stimato nelle fonti catturate].
- `d`: effetto del disegno (design effect) > 1: la varianza di livello elezione è più alta di quella del campionamento casuale semplice anche per pesature e disegni complessi (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- Più partiti: estrarre il vettore e rinormalizzare a 100, oppure una Dirichlet-multinomiale [INF].

## Evidenza

- 4221 sondaggi su 608 elezioni statali USA 1998-2014, ultime tre settimane: errore quadratico medio ≈ 3,5 punti, circa il doppio di quanto implicano i margini d'errore dichiarati; bias assoluto medio di elezione ≈ 2 punti (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- Per tipo di elezione: RMSE 3,7 (senato) e 3,9 (governatore) contro 2,0-2,1 attesi dal campionamento casuale; 2,5 per le presidenziali statali; bias assoluto ≈ 2 punti per senato e governatori, ≈ 1 per le presidenziali (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- L'errore cala avvicinandosi al voto ma si stabilizza nell'ultimo mese: a una settimana dal voto è simile che a un mese. Gli autori lo attribuiscono a errori di copertura e di non risposta, non a cambi d'opinione (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- Il bias non cresce nel tempo nonostante il crollo dei tassi di risposta (dal 36% del 1998 al 9% del 2012) (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- Oltre 30.000 sondaggi nazionali, 351 elezioni, 45 paesi, 1942-2017: gli errori seguono un andamento strutturato lungo la campagna e i sondaggi recenti non vanno peggio del solito (fonti/nature-jennings-wlezien-2018.md) [V]. Errore assoluto medio 2,1 punti negli anni '40-'50, simile nei '60-'70, 2 punti dal 2000; più basso nei sistemi proporzionali e nelle presidenziali; più alto per i partiti più grandi (fonti/soton-jennings-wlezien-2018-comunicato.md) [V].

## Regole italiane utili

Nei quindici giorni prima del voto è vietato rendere pubblici sondaggi sull'esito e sugli orientamenti di voto (art. 8, legge 28/2000, citato in fonti/agcom-delibera-121-18-sondaggi.md) [V]. Il regolamento AGCOM 256/10/CSP estende il divieto fino alla chiusura dei seggi (fonti/agcom-delibera-121-18-sondaggi.md) [V].

## Parametri tipici

| Parametro | Valore | Fonte |
|---|---|---|
| errore assoluto medio, ultima settimana | ~2 punti | (fonti/soton-jennings-wlezien-2018-comunicato.md) [V] |
| RMSE totale | ~3,5 punti, 2× il margine dichiarato | (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V] |
| bias comune di elezione, valore assoluto medio | 1-2 punti | (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V] |
| campione n | 1000 tipico | [INF] |
| effetto istituto | ±1 punto | [INF] |
| periodo di silenzio Italia | 15 giorni | (fonti/agcom-delibera-121-18-sondaggi.md) [V] |

## Costo computazionale

Un sondaggio è `O(P)` estrazioni gaussiane; anche cento istituti a settimana sono irrilevanti [INF]. Se invece si campionano davvero n elettori-agenti, il costo è `O(n)` per sondaggio e dà gratis l'errore di campionamento [INF].

## Cosa semplifichiamo

- Nessun modello di affluenza separato: il bias comune di elezione assorbe l'incertezza su chi va a votare, che gli autori indicano come il problema più difficile (fonti/shirani-mehr-2018-bias-varianza-sondaggi.md) [V].
- Nessuna dinamica di "herding" fra istituti [INF].

## Per il nostro gioco

- Generare i sondaggi dallo stato vero con tre strati di errore (comune, istituto, campionamento): il giocatore che fa la media degli istituti migliora ma non elimina l'errore, come nella realtà. Il bias comune si rivela la notte elettorale: è il colpo di scena "i sondaggi avevano sbagliato" senza scriptarlo.
- Errore più grande per i partiti grandi, più piccolo nei sistemi proporzionali.
- Silenzio elettorale di 15 giorni in Italia: in quel periodo il giocatore resta al buio. Sondaggi privati del partito possono restare disponibili: è una scelta di design da decidere.
- Democracy 4 ha aggiunto con un DLC la distinzione fra elettorato e "likely voters" nei sondaggi (fonti/steam-democracy4-voting-systems.md) [V]: conferma che il tema interessa ai giocatori.
- Proiezioni in seggi: passare i sondaggi nel motore di [[sistemi-elettorali]].

## Collegamenti

- [[voto-spaziale]], [[voto-economico-e-retrospettivo]]: producono le quote vere p_j(t).
- [[sistemi-elettorali]]: proiezione da voti a seggi.
- [[elettorato-e-gruppi-di-voto]], [[democracy-4]]: sondaggi nei giochi esistenti.
- [[campagna-elettorale]]: i sondaggi guidano le scelte di campagna.
- [[consiglieri-e-informazione-imperfetta]]: il sondaggio dentro la catena stato vero → statistica → rapporto.

## Riferimenti

- **URL** https://5harad.com/papers/polling-errors.pdf · [V]
  **perché conta qui** scompone l'errore dei sondaggi in bias comune e varianza, con stime numeriche
  **cosa se ne prende** la struttura a strati del generatore e i valori 3,5 / 2 punti
- **URL** https://www.nature.com/articles/s41562-018-0315-6 · [V]
  **perché conta qui** confronto su 45 paesi e 75 anni, non solo USA
  **cosa se ne prende** errore medio ~2 punti, stabile nel tempo, più basso col proporzionale
- **URL** https://www.southampton.ac.uk/news/2018/03/opinion-poll-errors.page · [V]
  **perché conta qui** riporta i numeri dell'articolo Nature (il PDF dell'autore è bloccato)
  **cosa se ne prende** 2,1 e 2 punti di errore medio, errore più alto per i partiti grandi
- **URL** https://www.agcom.it/sites/default/files/migration/delibera/Delibera%20121-18-CONS.pdf · [V]
  **perché conta qui** cita testualmente il divieto di pubblicazione dei sondaggi in Italia
  **cosa se ne prende** la regola dei 15 giorni come meccanica
