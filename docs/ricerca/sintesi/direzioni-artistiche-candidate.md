---
tipo: sintesi
fonti: [fonti/defcon-escapist-everybody-dies.md, fonti/steam-defcon.md, fonti/suzerain-avclub-intervista.md, fonti/obradinn-setsideb-dither.md, fonti/datawrapper-scale-colore.md, fonti/dorfromantik-gamedeveloper-intervista.md, fonti/steam-dorfromantik.md, fonti/townscaper-gamedeveloper-come-funziona.md, fonti/minimetro-wikipedia.md, fonti/minimetro-designingsound-vreeland.md, fonti/steam-mini-metro.md, fonti/democracy4-mod-policies.md, fonti/steam-democracy4.md, fonti/frostpunk-xbox-console-radiale.md, fonti/minimotorways-miketendo-intervista.md, fonti/islanders-gameworldobserver-intervista.md, fonti/godot-renderers.md, fonti/steamworks-deck-compat.md, fonti/tauri-linux-graphics.md]
aggiornato: 2026-10-16
---
# Direzioni artistiche candidate

Quattro direzioni distinte per un simulatore di governo che sia bello senza pesare. Ognuna nasce da una metafora presa dal tema, come Mini Metro dalla mappa della metro e DEFCON dal bunker NORAD. Tutte stanno sotto la soglia hardware dei riferimenti (da 64 MB di memoria video per DEFCON a Intel HD 520 per Dorfromantik). Cambiano il tono, il costo di produzione degli asset e il rischio di scivolare nel generico.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO]. Costi e rischi sono stime [INF] costruite sui riferimenti citati.

## Vincoli tecnici (dalle pagine `tecnica/`)

- Bersaglio hardware: Intel UHD 620 con 8 GB a 1080p e Steam Deck a 1280x800 ([[prestazioni-su-hardware-modesto]]); sul Deck il testo deve avere almeno 9 px di altezza (fonti/steamworks-deck-compat.md) [V, cattura di altro agente].
- Candidato motore in testa: Godot con renderer Compatibility (OpenGL 3.3) ([[godot]]). Nella tabella ufficiale delle funzioni, Compatibility supporta glow, SSAO, nebbia di profondità e post-processing personalizzato con un quad a schermo intero; non supporta i CompositorEffects, i riflessi in screen space né la nebbia volumetrica (fonti/godot-renderers.md:112-122) [V, cattura di altro agente]. Conseguenza: bagliore (A), edge detection e retino come passata a schermo intero (B, C) e toon a bande (C) restano fattibili; niente che dipenda da Forward+.
- Seconda scelta: Tauri ([[tauri]]). Su Linux e Steam Deck WebGL e canvas possono finire su un percorso lento senza errori (fonti/tauri-linux-graphics.md) [V, cattura di altro agente]: colpisce proprio mappe e grafici animati, quindi pesa di più sulle direzioni con mappa animata (A, C) che su quelle quasi statiche (B, D) [INF].
- Una build senza 3D (`disable_3d`) toglie circa il 15% del template Godot ([[godot]]): vale per A, B, D, non per C [INF].

## A. Sala operativa

**Metafora:** il giocatore è nella stanza delle decisioni; il mondo è un tracciato su uno schermo.
- **Riferimenti:** DEFCON, look "semplice e stilizzato", bagliore e tono iterati, suono tragico (fonti/defcon-escapist-everybody-dies.md:45) [V].
- **Forma:** mappa vettoriale a contorni su fondo scuro, un colore d'accento per fazione, testo mono per i dati, globo a filo.
- **Suono:** ambient sintetico, allarmi come segnali di stato.
- **Costo GPU/CPU:** minimo; DEFCON chiede 64 MB di memoria video (fonti/steam-defcon.md:48) [V]. **Produzione:** bassa, quasi tutto è dato più stile.
- **Rischio:** alto di cliché ("hacker", "war room", tema scuro da dashboard). Freddo sui ministeri sociali (scuola, sanità). Difficile da distinguere da DEFCON stesso.

## B. Carta stampata

**Metafora:** il paese si legge sui giornali, nei dossier e sulle carte geografiche stampate.
- **Riferimenti:** Suzerain, dove notizie, rapporti e giornali rendono il paese "vivo" (fonti/suzerain-avclub-intervista.md:31) [V]; il giornalismo dei dati per grafici e choropleth (fonti/datawrapper-scale-colore.md:40-95) [V]; il retino stabile di Obra Dinn per i ritratti (fonti/obradinn-setsideb-dither.md:18) [V]; la mappa di carta di CK3 [NON TROVATO].
- **Forma:** carta, inchiostro e due o tre colori tipografici; choropleth a retino o tinte piatte; titoli in serif, pannelli in sans con cifre tabellari; ritratti a mezzatinta; la fine del turno come prima pagina.
- **Suono:** carta, macchine da scrivere e telex, musica da camera a stati.
- **Costo GPU/CPU:** minimo, tutto 2D. **Produzione:** media; il lavoro è impaginazione e tipografia, molto controllabile.
- **Rischio:** troppo testo; somigliare a un sito di notizie; il retino può ridurre la leggibilità delle mappe se usato ovunque. Il militare chiede un registro diverso (mappe di stato maggiore, che restano comunque "carta").

## C. Plastico

**Metafora:** il paese è un modellino su un tavolo; il giocatore lo guarda dall'alto e lo vede cambiare.
- **Riferimenti:** Dorfromantik, paesaggio "ideale e armonico" (fonti/dorfromantik-gamedeveloper-intervista.md:77) [V]; Townscaper, forme grandi prevedibili e dettaglio piccolo variabile (fonti/townscaper-gamedeveloper-come-funziona.md:149) [V]; Bad North e Islanders.
- **Forma:** regioni low-poly con toon a due bande e contorno; città, fabbriche e basi che crescono come pezzi; colori idealizzati; globo a celle per la vista mondiale.
- **Suono:** ambiente del paesaggio, musica acustica.
- **Costo GPU/CPU:** basso ma non minimo; Dorfromantik gira su Intel HD 520 con 3 GB (fonti/steam-dorfromantik.md:94-96) [V]. **Produzione:** alta: servono kit modulari 3D per edifici, infrastrutture, unità militari.
- **Rischio:** tono "cozy" che stona con guerra, repressione e crisi; somigliare a Townscaper o Dorfromantik; i dati (choropleth) faticano su un terreno colorato.

## D. Diagramma di sistema

**Metafora:** lo Stato come schema di funzionamento: nodi, flussi, linee.
- **Riferimenti:** Mini Metro, stile da mappa di trasporto (fonti/minimetro-wikipedia.md:22) [V] e suono generato dalla rete (fonti/minimetro-designingsound-vreeland.md:32) [V]; Democracy 4, icone collocate per zona e connesse (fonti/democracy4-mod-policies.md:34; fonti/steam-democracy4.md:38) [V]; Frostpunk, una forma che organizza tutto (fonti/frostpunk-xbox-console-radiale.md:17) [V].
- **Forma:** policy, ministeri e gruppi sociali come nodi; commercio, bilancio e catene di comando come linee che si ispessiscono; geometria piatta e una palette per categoria.
- **Suono:** procedurale, ogni flusso ha un ritmo.
- **Costo GPU/CPU:** minimo; Mini Metro chiede 2 GB e Shader Model 4.0 (fonti/steam-mini-metro.md:69-72) [V]. **Produzione:** bassa negli asset, alta nel design dei diagrammi.
- **Rischio:** l'astrazione toglie verosimiglianza, che è la promessa del gioco; la mappa geografica non si riduce a un diagramma; il passo da "diagramma" a "dashboard di nodi" è breve.

## Confronto

| | A Sala operativa | B Carta stampata | C Plastico | D Diagramma |
| --- | --- | --- | --- | --- |
| GPU/CPU | minimo | minimo | basso | minimo |
| Su Compatibility (Godot) | sì (glow) | sì (quad a schermo intero) | sì (toon + edge detection), 3D obbligatorio | sì |
| Su Tauri/Linux | rischio: mappa animata in WebGL | ok: quasi statico, Canvas 2D | rischio alto: 3D in WebGL | ok |
| Testo ≥ 9 px su Deck | facile (mono a corpo fisso) | da progettare: molto testo | facile | da progettare: etichette dei nodi |
| Produzione asset | bassa | media | alta | bassa |
| Verosimiglianza | media | alta | media | bassa |
| Tutti i ministeri | debole sul sociale | buona | buona | buona |
| Militare | ottimo | buono | buono (unità come pezzi) | debole |
| Rischio "generico/slop" | alto | medio | medio | alto |

## Raccomandazione (da validare, non decisione)

**B come base, con la vista di rete di D per lo schermo delle policy.** B è l'unica direzione che regge insieme verosimiglianza, tutti i ministeri e costo minimo, e trasforma la densità di testo e numeri, che il gioco ha comunque, in materiale estetico invece di nasconderla. D dà la forma dominante allo schermo che è il cuore del genere (la rete causa-effetto di Democracy), purché sia disegnata con lo stesso inchiostro di B e non come grafo da dashboard. A e C restano registri locali possibili (A per la vista militare notturna, C per la vista ravvicinata di una regione) solo se il prototipo dimostra che non spezzano l'unità; altrimenti si tagliano, come Islanders taglia (fonti/islanders-gameworldobserver-intervista.md:121) [V].

Prima di decidere, una prova economica [INF]: lo stesso schermo (mappa d'Europa con una choropleth, un pannello policy, la notizia di fine turno) disegnato in B e in B+D, giudicato su leggibilità e sovrapposizioni a 1080p su UHD 620 e a 1280x800 sul Deck (testo ≥ 9 px). B e D sono anche le direzioni più tolleranti verso i vincoli tecnici: quasi statiche, nessuna dipendenza dal 3D o da WebGL veloce.

## Per il nostro gioco

- **Prendiamo:** una metafora sola come fonte di forma, colore, movimento e suono.
- **Evitiamo:** mescolare direzioni senza prova; il tema scuro con accenti al neon come scelta di default.
- **Aperto:** scelta della direzione (Mario); prototipo di confronto; verifica legale sui ritratti di politici reali e sui confini (`legale/`).

## Riferimenti

Ogni fonte è già descritta con i tre campi nella pagina di dettaglio: [[riferimenti-minimalisti]] (DEFCON, Dorfromantik, Townscaper, Mini Metro, Islanders), [[interfacce-di-giochi-gestionali]] (Suzerain, Democracy 4, Frostpunk), [[data-viz-cinematica]] (Datawrapper), [[shader-ed-effetti-economici]] (Obra Dinn). Qui servono a questo:
- **URL** https://www.escapistmagazine.com/everybody-dies-in-defcon/ · [V]
  **perché conta qui** fonda la direzione A
  **cosa se ne prende** tono da luce e suono, non da dettaglio
- **URL** https://www.avclub.com/suzerain-interview · [V]
  **perché conta qui** fonda la direzione B
  **cosa se ne prende** il paese raccontato da giornali e rapporti
- **URL** https://www.gamedeveloper.com/disciplines/sparking-joy-through-tile-placement-in-idyllic-village-builder-i-dorfromantik-i- · [V]
  **perché conta qui** fonda la direzione C
  **cosa se ne prende** paesaggio idealizzato, coerenza fra pezzi
- **URL** https://designingsound.org/2016/02/18/the-programmed-music-of-mini-metro-interview-with-rich-vreeland-disasterpeace/ · [V]
  **perché conta qui** fonda la direzione D
  **cosa se ne prende** unità fra forma, movimento e suono generati dal sistema
- **URL** https://store.steampowered.com/app/1520/DEFCON/, https://store.steampowered.com/app/1455840/Dorfromantik/, https://store.steampowered.com/app/287980/Mini_Metro/ · [V]
  **perché conta qui** soglie hardware delle direzioni
  **cosa se ne prende** il costo GPU/CPU in tabella

## Collegamenti

- [[riferimenti-minimalisti]], [[interfacce-di-giochi-gestionali]], [[data-viz-cinematica]], [[mappe-e-globo]], [[shader-ed-effetti-economici]], [[movimento-e-feedback]], [[tipografia-e-font-aperti]], [[suono-e-musica-leggeri]]: le pagine da cui viene ogni direzione.
- [[democracy-4]], [[victoria-3]]: pagine di gioco di altri agenti.
- [[prestazioni-su-hardware-modesto]], [[godot]], [[tauri]]: vincoli tecnici (pagine di altro agente).
