---
tipo: estetica
fonti: [fonti/steam-democracy4.md, fonti/democracy4-mod-policies.md, fonti/cliffsblog-2021-04.md, fonti/suzerain-avclub-intervista.md, fonti/steam-suzerain.md, fonti/steam-recensioni-meccaniche.md, fonti/frostpunk-xbox-console-radiale.md, fonti/frostpunk-playstation-console.md, fonti/frostpunk2-pcgamesn-ui.md, fonti/pcgamesn-victoria3-nested-tooltip.md, fonti/terrainvicta-gameluster-recensione.md, fonti/terrainvicta-turnbasedlovers-1-0.md, fonti/steam-terra-invicta.md, fonti/intothebreach-gamedeveloper-igf.md, fonti/intothebreach-pcgamer-preview.md]
aggiornato: 2026-10-16
---
# Interfacce di giochi gestionali

Nei giochi di governo l'interfaccia *è* il gioco: si passa più tempo nei pannelli che sulla mappa. Le fonti convergono su una regola: la complessità non è il problema, lo è il modo in cui si presenta. Funziona chi dà una forma riconoscibile all'informazione (radiale, rete, giornale) e la stratifica (tooltip annidati); annega chi la nasconde dietro icone minuscole o la lascia senza spiegazione.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Cosa funziona

### Democracy 4: lo schermo è il modello
- Un motore grafico vettoriale scritto apposta per un'interfaccia "più adattabile e più pulita" [V] (fonti/steam-democracy4.md:38).
- Ogni policy ha una "zona" dello schermo dove compare la sua icona, legata alla categoria e al ministro competente [V] (fonti/democracy4-mod-policies.md:34). La disposizione spaziale è un dato di design, non un'impaginazione.
- La rete di icone con le connessioni che si accendono al passaggio del mouse è la cifra della serie: lo si sa giocando, ma nessuna fonte catturata lo descrive [NON TROVATO].
- Localizzazione: il problema delle lingue con "così tanti caratteri da non stare nella memoria delle texture" risolto con pango e cairo [V] (fonti/cliffsblog-2021-04.md:198). Risoluzione più diffusa fra i giocatori: 1920 px [V] (fonti/cliffsblog-2021-04.md:164).

### Suzerain: il paese visto dal tavolo
- La "strategy view" con la mappa funziona perché il giocatore è seduto al tavolo e guarda il paese; da cittadino non funzionerebbe [V] (fonti/suzerain-avclub-intervista.md:27).
- Notiziari, rapporti sulle città, giornali diversi fanno sentire il paese "come una cosa viva" [V] (fonti/suzerain-avclub-intervista.md:31). Gli autori non volevano che fosse giocato come un gioco di numeri [V] (fonti/suzerain-avclub-intervista.md:39).
- Un recensore lo descrive come "romanzo interattivo, niente grafica 3D" e lo chiama capolavoro [V] (fonti/steam-recensioni-meccaniche.md:54). Requisiti comunque non minimi: 4 GB RAM, GTX serie 900 [V] (fonti/steam-suzerain.md:87-88).

### Frostpunk: una forma per tutto
- Il gioco ruota attorno a un generatore al centro di un cratere; strade, edifici e calore sono cerchi. L'interfaccia console riprende la stessa forma: hub circolare con cerchi più piccoli sul bordo, categorie di edifici come sottomenu circolari [V] (fonti/frostpunk-xbox-console-radiale.md:15-21).
- "La complessità del gameplay non è il problema. Il problema è come fornisci le opzioni ai giocatori": invece di semplificare, iterare e fare focus test [V] (fonti/frostpunk-playstation-console.md:21-22). Oltre 50 tipi di edificio raggruppati per funzione e per risorsa [V] (fonti/frostpunk-playstation-console.md:23).
- Frostpunk 2 ha rifatto HUD, menu di costruzione e albero delle idee prima del lancio per "chiarezza, leggibilità e look and feel" dopo la beta [V] (fonti/frostpunk2-pcgamesn-ui.md:19).

### Paradox: tooltip annidati
- Victoria 3 adotta i tooltip annidati di Crusader Kings III: ogni termine evidenziato apre un altro tooltip, all'infinito, "come avere la wiki ufficiale accanto al puntatore", anche per concetti base come pop, tasse sui dividendi, prezzo di mercato [V] (fonti/pcgamesn-victoria3-nested-tooltip.md:15-19).
- I dev diary originali (Victoria 3 #29 e #74, CK3 console #3, EU5 Tinto Talks #77) non sono catturabili: il forum e il sito Paradox rispondono 406 o con una pagina di verifica del browser [NON TROVATO]. URL: https://forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-29-user-experience.1506484/
- La "mappa di carta" di CK3 alla massima distanza di zoom (dev diary #2) è un riferimento importante per noi ma non verificato su fonte catturata [NON TROVATO]. URL: https://forum.paradoxplaza.com/forum/threads/ck3-dev-diary-2-the-medieval-map.1274052/

### Into the Breach: la regola della chiarezza
- Non è un gestionale, ma fissa la regola. Gli attacchi nemici sono annunciati prima; niente nebbia di guerra né percentuali di colpo [V] (fonti/intothebreach-gamedeveloper-igf.md:47; fonti/intothebreach-pcgamer-preview.md:29). "Quando fallisci un obiettivo, deve essere chiarissimo come o perché" [V] (fonti/intothebreach-gamedeveloper-igf.md:67).

## Cosa annega il giocatore

- **Terra Invicta.** "L'interfaccia è meno uno strumento e più uno stress test. L'informazione c'è, ma è sepolta, ridotta o sussurrata quando dovrebbe urlare"; cambi d'opinione pubblica che costano una nazione "nascosti dietro icone minuscole" [V] (fonti/terrainvicta-turnbasedlovers-1-0.md:63-65). Interfaccia "a volte affollata", però i tooltip ci sono ovunque al passaggio del mouse [V] (fonti/terrainvicta-gameluster-recensione.md:21). La 1.0 migliora "non semplificando i sistemi ma rispettando il tuo tempo" [V] (fonti/terrainvicta-turnbasedlovers-1-0.md:137). Pesa 30 GB [V] (fonti/steam-terra-invicta.md:93).
- **Geo-Political Simulator 2026.** Recensione negativa: "UI e mancanza di informazioni e documentazione: servono almeno i tooltip" [V] (fonti/steam-recensioni-meccaniche.md:82); un'altra lamenta "grafica datata e traballante" [V] (fonti/steam-recensioni-meccaniche.md:86).

## Regole che ne derivano [INF]

1. Una forma geometrica dominante, presa dal tema, che organizza menu e schermate (cerchio di Frostpunk, rete di Democracy).
2. Profondità a strati: superficie leggibile in un'occhiata, dettaglio raggiungibile con tooltip annidati; mai il dettaglio in superficie.
3. Ciò che costa al giocatore (perdere una nazione, una crisi) deve avere peso visivo proporzionale: grande, centrale, annunciato prima.
4. Le conseguenze si mostrano prima della conferma (Into the Breach), il perché dopo l'esito.
5. Testo e cifre sono materiale visivo primario: tipografia e impaginazione da giornale valgono più di cornici e gradienti. Vedi [[tipografia-e-font-aperti]].

## Per il nostro gioco

- **Prendiamo:** forma dominante unica; tooltip annidati; telegrafare le conseguenze; notizie e giornali come canale (Suzerain).
- **Evitiamo:** icone senza etichetta per dati critici; pannelli a griglia uniforme da dashboard, dove tutto pesa uguale; schermate che chiedono 30 GB per mostrare tabelle.
- **Aperto:** quale forma dominante. Dipende dalla direzione in [[direzioni-artistiche-candidate]]. Le pagine per gioco ([[democracy-4]], [[victoria-3]]) sono di altri agenti.

## Riferimenti

- **URL** https://store.steampowered.com/app/1410710/Democracy_4/ · [V]
  **perché conta qui** dichiarazione dello sviluppatore sul motore vettoriale dell'UI
  **cosa se ne prende** UI vettoriale = scalabile e pulita a qualunque risoluzione
- **URL** https://positech.co.uk/democracy4/mod_policies.html (cattura `democracy4-mod-policies`, di altro agente) · [V]
  **perché conta qui** mostra che la posizione delle icone è un campo del modello dati
  **cosa se ne prende** layout spaziale come dato, non come impaginazione
- **URL** https://positech.co.uk/cliffsblog/2021/04 (cattura `cliffsblog-2021-04`, di altro agente) · [V]
  **perché conta qui** costo reale della localizzazione in un gioco di testo
  **cosa se ne prende** pango/cairo per CJK; risoluzione 1920 px dominante
- **URL** https://www.avclub.com/suzerain-interview · [V]
  **perché conta qui** intervista primaria a Torpor Games sulla vista strategica e sui giornali
  **cosa se ne prende** il paese "vivo" attraverso notizie e rapporti, non numeri
- **URL** https://store.steampowered.com/app/1207650/Suzerain/ (cattura di altro agente) · [V]
  **perché conta qui** requisiti di sistema
  **cosa se ne prende** anche un gioco "di testo" chiede 4 GB e GTX 900 se è in 3D
- **URL** https://store.steampowered.com/appreviews/4021780 (cattura `steam-recensioni-meccaniche`, di altro agente) · [V]
  **perché conta qui** voce dei giocatori su UI mancante e grafica datata
  **cosa se ne prende** il tooltip è il minimo atteso
- **URL** https://news.xbox.com/en-us/?p=125332 · [V]
  **perché conta qui** il lead designer spiega l'interfaccia radiale
  **cosa se ne prende** la forma del gioco diventa la forma dell'interfaccia
- **URL** https://blog.playstation.com/archive/2019/10/10/find-out-how-frostpunk-console-editions-strategic-survival-gameplay-has-been-optimised-for-ps4 · [V]
  **perché conta qui** principio dichiarato su complessità e presentazione
  **cosa se ne prende** non semplificare il sistema, migliorare la presentazione; raggruppare per funzione
- **URL** https://www.pcgamesn.com/frostpunk-2/ui-improvements · [V]
  **perché conta qui** il seguito ha dovuto rifare l'UI dopo la beta
  **cosa se ne prende** la leggibilità va testata presto, costa rifarla tardi
- **URL** https://www.pcgamesn.com/victoria-3/nested-tooltip-system · [V]
  **perché conta qui** unica cattura riuscita sul sistema di tooltip annidati Paradox
  **cosa se ne prende** il meccanismo: termine evidenziato → tooltip → termine → tooltip
- **URL** https://gameluster.com/review-terra-invicta-mib-mvp/ · [V]
  **perché conta qui** recensione che bilancia affollamento e tooltip
  **cosa se ne prende** i tooltip non bastano se la superficie è affollata
- **URL** https://turnbasedlovers.com/review/terra-invicta-1-0-impressions/ · [V]
  **perché conta qui** critica più dura e verifica della 1.0
  **cosa se ne prende** l'anti-esempio: informazione critica dietro icone minuscole
- **URL** https://store.steampowered.com/app/1176470/Terra_Invicta/ · [V]
  **perché conta qui** requisiti
  **cosa se ne prende** 30 GB, 6 GB RAM: l'opposto del nostro obiettivo
- **URL** https://www.gamedeveloper.com/game-platforms/road-to-the-igf-subset-games-i-into-the-breach-i- · [V]
  **perché conta qui** intervista primaria a Subset Games
  **cosa se ne prende** telegrafare, e rendere chiaro il perché del fallimento
- **URL** https://www.pcgamer.com/into-the-breach-preview/ · [V]
  **perché conta qui** citazioni dirette degli autori sul determinismo
  **cosa se ne prende** niente nebbia né percentuali nascoste
- **URL** https://forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-29-user-experience.1506484/ · [NON TROVATO]
  **perché conta qui** dev diary primario sull'UX di Victoria 3
  **cosa se ne prende** da recuperare a mano: il forum blocca i client non browser

## Collegamenti

- [[riferimenti-minimalisti]]: lo stesso principio di taglio, con meno informazione.
- [[data-viz-cinematica]]: come mostrare numeri e serie nei pannelli.
- [[tipografia-e-font-aperti]]: il testo come materiale primario.
- [[movimento-e-feedback]]: telegrafare e rivelare l'esito di un turno.
- [[democracy-4]], [[victoria-3]]: pagine di gioco di altri agenti.
