---
tipo: estetica
fonti: [fonti/juice-gamedeveloper-2013.md, fonti/dont-juice-gamedeveloper.md, fonti/dorfromantik-gamedeveloper-intervista.md, fonti/minimetro-designingsound-vreeland.md, fonti/townscaper-gamedeveloper-come-funziona.md, fonti/intothebreach-gamedeveloper-igf.md, fonti/wcag22-animation-from-interactions.md, fonti/robertson-2008-animation-trend.md]
aggiornato: 2026-10-17
---
# Movimento e feedback

Il "juice" nasce nei giochi d'azione: cose che oscillano, rimbalzano e fanno piccoli rumori. In un gioco lento a turni lo stesso principio serve a un'altra cosa: far sentire il peso di una decisione e rendere leggibile l'esito. Le fonti indicano un limite preciso: il movimento deve servire l'atmosfera e l'informazione, altrimenti costa immersione. E deve potersi spegnere.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Juice, e il suo limite

- Jonasson e Purho (GDC Europe 2012): "le cose succose oscillano, schizzano, rimbalzano e fanno piccoli rumori carini; è un termine ombrello per ciò che rende un gioco più soddisfacente al tocco" [V] (fonti/juice-gamedeveloper-2013.md:14).
- Folmer Kelly (GDC Europe 2014), in risposta: aggiungere rifinitura rende un gioco più vivo ma fa perdere immersione; troppa attenzione all'effetto, poca al contesto [V] (fonti/dont-juice-gamedeveloper.md:14).
- Dorfromantik risolve la tensione: l'interazione fatta più spesso deve essere la più soddisfacente, con piccole animazioni e suoni; e nessun suono o animazione deve disturbare la calma [V] (fonti/dorfromantik-gamedeveloper-intervista.md:67, :71).

## Pattern per un gioco a turni

1. **Gesto frequente curato.** Da noi i gesti frequenti sono regolare uno slider di policy, aprire un tooltip, confermare il turno: sono questi a meritare la rifinitura, non le schermate rare [INF da Dorfromantik].
2. **Esiti sfasati a ritmo.** In Mini Metro i passeggeri comparivano a grappoli; il compositore ha proposto di sfasarli in modo ritmico (modulo "Arpeggio") e la proposta è entrata nel gioco [V] (fonti/minimetro-designingsound-vreeland.md:68). A fine turno: le conseguenze si rivelano una dopo l'altra, a tempo, invece che tutte insieme [INF].
3. **Forma subito, dettaglio dopo.** Townscaper mostra il blocco subito e lo decora un attimo dopo [V] (fonti/townscaper-gamedeveloper-come-funziona.md:131). Da noi: il numero cambia subito, il grafico e la mappa lo seguono [INF].
4. **Telegrafare.** In Into the Breach ogni attacco nemico è annunciato e il fallimento deve essere chiaro nel come e nel perché [V] (fonti/intothebreach-gamedeveloper-igf.md:55, :67). Da noi: anteprima dell'effetto di una policy (frecce, delta previsti) prima della conferma [INF].
5. **Animare transizioni, non dati.** L'animazione dei trend fa sbagliare l'analisi [V] (fonti/robertson-2008-animation-trend.md:13): si anima il passaggio fra due stati, poi si lascia fermo. Vedi [[data-viz-cinematica]].

## Accessibilità del movimento

- WCAG 2.2, criterio 2.3.3: le animazioni scatenate da un'interazione devono potersi disattivare, salvo quelle essenziali all'informazione [V] (fonti/wcag22-animation-from-interactions.md:25-27). Per chi ha disturbi vestibolari le reazioni vanno da nausea a emicrania [V] (fonti/wcag22-animation-from-interactions.md:37). Rimedi: evitare animazioni inutili, dare un comando per spegnerle, rispettare l'impostazione "riduci movimento" del sistema operativo [V] (fonti/wcag22-animation-from-interactions.md:39).
- È un criterio per il web, ma il problema fisico è lo stesso in un gioco [INF]. Un'opzione "movimento ridotto" che rispetta anche la preferenza di sistema è un costo piccolo.

## Tempi ed easing

Le linee guida Apple sul movimento non si sono lasciate catturare (pagina vuota per l'estrattore) [NON TROVATO]. URL: https://developer.apple.com/design/human-interface-guidelines/motion. Indicazioni di mestiere [INF]: transizioni di pannello brevi; nessuna animazione blocca l'input; ogni animazione si interrompe se il giocatore agisce; una sola curva di easing in tutto il gioco, così il movimento diventa parte dell'identità.

## Costo

Tween di posizione, scala e opacità su pochi elementi sono quasi gratis su CPU e GPU [INF]. Il costo vero è il ridisegno continuo: un'interfaccia ferma che si ridisegna solo quando qualcosa cambia consuma pochissimo, ed è il caso di un gioco a turni [INF]. Sul Deck conviene limitare i frame quando nulla si anima, per batteria e calore ([[prestazioni-su-hardware-modesto]]).

## Per il nostro gioco

- **Prendiamo:** rifinitura concentrata sui gesti frequenti; rivelazione ritmica dell'esito del turno; anteprima delle conseguenze; opzione movimento ridotto.
- **Evitiamo:** screenshake, particelle e rimbalzi su decisioni gravi (una guerra non "rimbalza"); animazioni che bloccano l'input; schermate che si muovono da sole.
- **Aperto:** il ritmo del turno e il legame con il suono, vedi [[suono-e-musica-leggeri]]. Deciso il 17/10/2026 per il ritmo: esito rivelato una voce alla volta in apertura, al massimo 7 voci in evidenza (docs/ricerca/sintesi/decisioni.md); il suono resta aperto.

## Riferimenti

- **URL** https://www.gamedeveloper.com/design/video-is-your-game-juicy-enough- · [V]
  **perché conta qui** resoconto del talk che ha definito il "juice"
  **cosa se ne prende** la definizione e il principio massimo effetto per minimo input
- **URL** https://www.gamedeveloper.com/design/video-indies-resist-the-urge-to-juice-it-or-lose-it- · [V]
  **perché conta qui** la critica al juice, utile per un gioco serio
  **cosa se ne prende** il juice costa immersione se ignora il contesto
- **URL** https://www.gamedeveloper.com/disciplines/sparking-joy-through-tile-placement-in-idyllic-village-builder-i-dorfromantik-i- · [V]
  **perché conta qui** regola applicata in un gioco calmo
  **cosa se ne prende** curare il gesto più frequente, senza rompere l'atmosfera
- **URL** https://designingsound.org/2016/02/18/the-programmed-music-of-mini-metro-interview-with-rich-vreeland-disasterpeace/ · [V]
  **perché conta qui** il modulo "Arpeggio" che sfasa eventi a ritmo
  **cosa se ne prende** rivelare l'esito del turno a tempo
- **URL** https://www.gamedeveloper.com/game-platforms/how-townscaper-works-a-story-four-games-in-the-making · [V]
  **perché conta qui** feedback immediato con dettaglio differito
  **cosa se ne prende** forma subito, dettaglio dopo
- **URL** https://www.gamedeveloper.com/game-platforms/road-to-the-igf-subset-games-i-into-the-breach-i- · [V]
  **perché conta qui** telegrafare come principio di design
  **cosa se ne prende** anteprima delle conseguenze, perché del fallimento chiaro
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html · [V]
  **perché conta qui** norma di accessibilità sul movimento
  **cosa se ne prende** animazioni non essenziali disattivabili; rispettare "riduci movimento"
- **URL** https://www.microsoft.com/en-us/research/publication/effectiveness-of-animation-in-trend-visualization/ · [V]
  **perché conta qui** limite misurato dell'animazione dei dati
  **cosa se ne prende** animare transizioni, non serie
- **URL** https://developer.apple.com/design/human-interface-guidelines/motion · [NON TROVATO]
  **perché conta qui** linee guida di piattaforma su durata e scopo del movimento
  **cosa se ne prende** da recuperare (pagina generata in JavaScript)

## Collegamenti

- [[data-viz-cinematica]]: animare le transizioni dei grafici.
- [[suono-e-musica-leggeri]]: suono e ritmo dell'esito.
- [[interfacce-di-giochi-gestionali]]: telegrafare nei pannelli.
- [[riferimenti-minimalisti]]: Dorfromantik e Mini Metro per intero.
- [[prestazioni-su-hardware-modesto]]: frame limitati e consumo (altro agente).
