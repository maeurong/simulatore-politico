---
tipo: estetica
fonti: [fonti/ameye-rendering-outlines.md, fonti/roystan-toon-shader.md, fonti/msdfgen-github.md, fonti/obradinn-setsideb-dither.md, fonti/townscaper-gamedeveloper-come-funziona.md, fonti/defcon-escapist-everybody-dies.md, fonti/godot-renderers.md]
aggiornato: 2026-10-16
---
# Shader ed effetti economici

Lo stile costa meno del realismo perché sostituisce luce fisica e texture con poche regole: contorni, due o tre bande di luce, un retino, un bagliore. Ogni tecnica qui sotto gira su GPU integrate se usata con misura. Il costo vero è nella coerenza: un effetto solo, applicato ovunque, vale più di cinque effetti diversi.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Contorni (outline)

Cinque tecniche, ciascuna con il suo compromesso fra prestazioni, resa e preparazione degli asset [V] (fonti/ameye-rendering-outlines.md:428):

| Tecnica | Come | Pro / contro | Fonte |
| --- | --- | --- | --- |
| Rim | prodotto scalare normale/vista nello shader dell'oggetto | semplice, funziona bene solo su oggetti sferici | (fonti/ameye-rendering-outlines.md:71) |
| Estrusione dei vertici | seconda passata con la mesh gonfiata | semplice, buona in clip space; problemi sugli spigoli vivi, si correggono con normali personalizzate | (fonti/ameye-rendering-outlines.md:222) |
| Buffer sfocato | silhouette in un buffer, sfocatura, composizione | morbido e luminoso, più costoso; si riduce il buffer prima della sfocatura | (fonti/ameye-rendering-outlines.md:242, :296) |
| Jump flood | campo di distanza calcolato a passi | contorni larghi a costo ragionevole | (fonti/ameye-rendering-outlines.md:300) |
| Edge detection | passata a schermo intero su discontinuità di profondità/normali (Roberts, Sobel) | un solo passaggio per tutta la scena, va calibrato | (fonti/ameye-rendering-outlines.md:310, :424) |

Per una mappa con confini e un diorama di edifici: edge detection a schermo intero se la scena è 3D, linee vettoriali disegnate direttamente se è 2D [INF].

## Toon shading

- Il toon (cel) shading fa sembrare piatte le superfici 3D. Il caso Breath of the Wild: due bande di luminosità, più rim e riflesso speculare; il rim solo sulle superfici illuminate dalla luce principale [V] (fonti/roystan-toon-shader.md:20-26).
- Il tutorial usa una sola luce direzionale che proietta e riceve ombre [V] (fonti/roystan-toon-shader.md:18). Una luce, un `step` sulla luminosità: è uno degli shader di illuminazione più economici che esistano [INF].

## Retino e dithering

- Return of the Obra Dinn: 3D in tempo reale ombreggiato in bianco e nero con dithering; il punto di forza è che il dithering è *stabile*, i punti non saltano e sembrano una texture applicata [V] (fonti/obradinn-setsideb-dither.md:18). Il post originale di Lucas Pope su TIGSource risponde 403 [NON TROVATO]. URL: https://forums.tigsource.com/index.php?topic=40832.msg1363742#msg1363742
- Per noi il retino è adatto a una direzione "carta stampata": choropleth a tratteggio, ritratti dei politici a mezzatinta [INF]. Il dithering stabile in movimento è difficile (è il lavoro documentato da Pope); su mappe e pannelli quasi fermi il problema quasi non si pone [INF].

## Testo e icone nitidi a ogni scala

- I campi di distanza con segno (SDF) rappresentano forme vettoriali e glifi come texture da cui ricostruirli in tempo reale; il metodo multicanale (MSDF) riproduce gli angoli vivi quasi perfettamente usando i tre canali colore [V] (fonti/msdfgen-github.md:13). Il metodo classico è il paper Valve del 2007, citato nello stesso README [V] (fonti/msdfgen-github.md:13).
- `msdfgen` e `msdf-atlas-gen` (atlanti di font) sono MIT [M] (`gh api repos/Chlumsky/msdfgen`, `repos/Chlumsky/msdf-atlas-gen`, 16/10/2026).
- Per noi: etichette della mappa e icone che restano nitide con qualunque zoom, con un solo atlante in memoria [INF]. Dipende dal motore: vedi [[godot]] e [[tauri]].

## Vincolo del renderer

- Il candidato motore è Godot con renderer Compatibility (OpenGL 3.3) ([[godot]], [[prestazioni-su-hardware-modesto]]). Compatibility supporta glow, SSAO e post-processing personalizzato con quad a schermo intero; non supporta i CompositorEffects (fonti/godot-renderers.md:116-122) [V, cattura di altro agente]. Quindi edge detection, retino e bagliore vanno scritti come shader su quad a schermo intero; toon e rim stanno nello shader del materiale. Lo stencil buffer, usato da Townscaper, è arrivato in Godot 4.5 ([[godot]]); il supporto in Compatibility non è verificato qui [NON TROVATO].
- Su Tauri gli stessi effetti passano da WebGL, che su Linux e Deck può rallentare senza errori ([[tauri]]).

## Trucchi di costo già visti

- Townscaper ritaglia le finestre con lo stencil buffer invece di modellarle [V] (fonti/townscaper-gamedeveloper-come-funziona.md:139) e mostra subito il blocco, il dettaglio dopo [V] (fonti/townscaper-gamedeveloper-come-funziona.md:129-131).
- DEFCON: il tono viene da bagliore e colore, cercati con molte iterazioni [V] (fonti/defcon-escapist-everybody-dies.md:45). Un bagliore (bloom) a risoluzione ridotta è una passata di post-processing [INF].

## Particelle e luce stilizzata [INF]

Nessuna fonte primaria catturata su questo punto. Indicazioni di mestiere, da verificare quando lo stack è scelto:
- Particelle come sprite istanziati, poche decine per effetto, solo su eventi (esplosione, festa, protesta), mai in continuo.
- Luce "dipinta": colore di ambiente e ombra scelti a mano per ora del giorno o stato del paese (crisi, prosperità), invece di illuminazione globale.
- Ombre finte (blob sotto gli oggetti) invece di shadow map, se la vista è dall'alto.

## Per il nostro gioco

- **Prendiamo:** al massimo un effetto di firma (contorno, retino o bagliore) più toon a due bande se c'è 3D; testo e icone in SDF/MSDF.
- **Evitiamo:** accumulare effetti (bloom + outline + vignetta + aberrazione cromatica = slop); illuminazione realistica.
- **Aperto:** quale effetto di firma, legato alla direzione in [[direzioni-artistiche-candidate]].

## Riferimenti

- **URL** https://ameye.dev/notes/rendering-outlines/ · [V]
  **perché conta qui** rassegna tecnica delle cinque tecniche di contorno con i compromessi
  **cosa se ne prende** quale outline scegliere secondo scena e budget
- **URL** https://roystan.net/articles/toon-shader/ · [V]
  **perché conta qui** tutorial di riferimento sul toon shading
  **cosa se ne prende** due bande + rim + speculare, una luce
- **URL** https://setsideb.com/?p=5672 · [V]
  **perché conta qui** sintesi del lavoro di Pope sul dithering stabile
  **cosa se ne prende** retino come stile 3D in tempo reale; la stabilità è il problema da risolvere
- **URL** https://forums.tigsource.com/index.php?topic=40832.msg1363742#msg1363742 · [NON TROVATO]
  **perché conta qui** fonte primaria di Pope
  **cosa se ne prende** da recuperare a mano (403)
- **URL** https://github.com/Chlumsky/msdfgen · [V]
  **perché conta qui** strumento MIT per testo e icone a distanza con segno
  **cosa se ne prende** MSDF per etichette e icone nitide a ogni zoom
- **URL** https://www.gamedeveloper.com/game-platforms/how-townscaper-works-a-story-four-games-in-the-making · [V]
  **perché conta qui** trucchi di produzione concreti
  **cosa se ne prende** stencil per i dettagli, decorazione differita
- **URL** https://www.escapistmagazine.com/everybody-dies-in-defcon/ · [V]
  **perché conta qui** il tono ottenuto con luce e colore
  **cosa se ne prende** un effetto di firma iterato a lungo

## Collegamenti

- [[riferimenti-minimalisti]]: i giochi da cui vengono questi effetti.
- [[mappe-e-globo]]: dove si applicano contorni e retini.
- [[tipografia-e-font-aperti]]: font da convertire in atlanti MSDF.
- [[direzioni-artistiche-candidate]]: quale effetto per quale direzione.
- [[godot]], [[prestazioni-su-hardware-modesto]], [[tauri]]: vincoli di renderer e hardware (altro agente).
