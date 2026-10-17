---
tipo: estetica
fonti: [fonti/minimetro-wikipedia.md, fonti/minimetro-designingsound-vreeland.md, fonti/steam-mini-metro.md, fonti/minimotorways-miketendo-intervista.md, fonti/steam-mini-motorways.md, fonti/dorfromantik-gamedeveloper-intervista.md, fonti/dorfromantik-digitaltrends-intervista.md, fonti/steam-dorfromantik.md, fonti/townscaper-gamedeveloper-come-funziona.md, fonti/steam-townscaper.md, fonti/badnorth-80lv-wfc-diorami.md, fonti/steam-bad-north.md, fonti/islanders-gameworldobserver-intervista.md, fonti/steam-islanders.md, fonti/defcon-escapist-everybody-dies.md, fonti/steam-defcon.md]
aggiornato: 2026-10-16
---
# Riferimenti minimalisti

I gestionali leggeri che restano in mente tolgono invece di aggiungere: una sola metafora visiva presa dal tema (mappa della metro, bunker NORAD, plastico di paese), forme piatte, palette stretta, suono legato al sistema. Costano poco perché non hanno texture fotografiche, luci fisiche o migliaia di asset unici. Per noi contano due cose: il metodo per scegliere la metafora e i numeri che dicono quanto poco hardware chiedono.

Tag: [V] fonte primaria letta · [M] misurato in sessione · [INF] inferenza · [NON TROVATO].

## Mini Metro (Dinosaur Polo Club, 2015)

- **Memorabile.** Lo stile riprende le mappe di trasporto moderne [V] (fonti/minimetro-wikipedia.md:22). Stazioni e passeggeri sono forme geometriche: il triangolo va alla stazione triangolare [V] (fonti/minimetro-wikipedia.md:26). Il suono è parte del design: quasi tutto è fatto di campioni suonati in tempo reale dalle azioni del giocatore, ogni linea ha un ritmo e una nota [V] (fonti/minimetro-designingsound-vreeland.md:16, :32). Il compositore cercava "unità fra forma, movimento e suono" [V] (fonti/minimetro-designingsound-vreeland.md:36). Premio IGF 2016 per l'audio, candidature per arte visiva e design [V] (fonti/minimetro-wikipedia.md:97).
- **Perché costa poco.** Requisiti minimi: 2 GB di RAM, Shader Model 4.0, 350 MB su disco [V] (fonti/steam-mini-metro.md:69-72). Geometria 2D piatta, poche primitive, nessuna texture [INF].
- **Cosa si riprende.** L'astrazione come scelta di leggibilità, non come ripiego. I limiti numerici dichiarati (7 linee, 4 locomotive, 4 carrozze) che rendono leggibili sia lo schermo sia il suono [V] (fonti/minimetro-designingsound-vreeland.md:72). Modalità daltonici e notte di serie [V] (fonti/steam-mini-metro.md:52). Non si riprendono la grammatica delle linee colorate a 45° né le forme delle stazioni: sono la firma del gioco.

## Mini Motorways (Dinosaur Polo Club, 2021)

- **Memorabile.** Lo stile minimalista era deciso in partenza; la discussione è stata sul *livello* di astrazione: fra forme pure alla Mini Metro e mappe turistiche con edifici singoli, scelta una via di mezzo [V] (fonti/minimotorways-miketendo-intervista.md:51-53). Palette selezionabile per mappa, incluse daltonici e notte; colonna sonora reattiva di Disasterpeace [V] (fonti/minimotorways-miketendo-intervista.md:102-103).
- **Perché costa poco.** Shader Model 4.0, 350 MB [V] (fonti/steam-mini-motorways.md:65-67).
- **Cosa si riprende.** La domanda giusta da farsi in pre-produzione: non "che stile", ma "quanto astratto". Per un simulatore verosimile la risposta sta più vicino alla mappa illustrata che alla forma pura [INF].

## Dorfromantik (Toukana Interactive, 2021-22)

- **Memorabile.** Obiettivo dichiarato: un'interpretazione "ideale e armonica" del paesaggio reale, "con tutta la sua bellezza e nessuno dei suoi difetti" [V] (fonti/dorfromantik-gamedeveloper-intervista.md:77). Forme di alberi e case prese dall'arte naïf, colori e texture dalla pittura impressionista (le sfumature dei campi di grano) [V] (fonti/dorfromantik-digitaltrends-intervista.md:47). Uniformità tale che anche una tessera piazzata male sembra voluta [V] (fonti/dorfromantik-digitaltrends-intervista.md:45). Uno dei quattro pilastri era "Minimalism and Simplicity"; via i timer [V] (fonti/dorfromantik-digitaltrends-intervista.md:35).
- **Perché costa poco.** Gira su Intel HD 520 con 3 GB di RAM e 650 MB [V] (fonti/steam-dorfromantik.md:94-96). Unity, asset in Blender, animazioni con DOTween [V] (fonti/dorfromantik-gamedeveloper-intervista.md:43). Tessere modulari riusate: il costo è nel tileset, non nella scena [INF].
- **Cosa si riprende.** La regola "l'interazione più frequente deve essere la più soddisfacente", con il limite che nessun suono o animazione rompa l'atmosfera [V] (fonti/dorfromantik-gamedeveloper-intervista.md:67, :71). Vedi [[movimento-e-feedback]].

## Townscaper (Oskar Stålberg, 2020)

- **Memorabile.** Griglia irregolare di quadrilateri, scelta per avere geometria organica come nelle città vere [V] (fonti/townscaper-gamedeveloper-come-funziona.md:105). Generazione a tre strati: griglia irregolare, Wave Function Collapse per scegliere i pezzi, marching cubes per adattarli alla griglia [V] (fonti/townscaper-gamedeveloper-come-funziona.md:123-125). "Voglio che le forme grandi siano prevedibili, le piccole possono variare molto" [V] (fonti/townscaper-gamedeveloper-come-funziona.md:149). L'astrazione lascia al giocatore proiettare il dettaglio che immagina [V] (fonti/townscaper-gamedeveloper-come-funziona.md:159).
- **Perché costa poco.** Il blocco compare subito, la decorazione arriva dopo: il lavoro pesante è spalmato nel tempo [V] (fonti/townscaper-gamedeveloper-come-funziona.md:129-131). Le finestre sono ritagliate con lo stencil buffer invece che modellate [V] (fonti/townscaper-gamedeveloper-come-funziona.md:139). Requisiti di sistema non estratti dalla cattura Steam [NON TROVATO] (fonti/steam-townscaper.md).
- **Cosa si riprende.** Forme grandi prevedibili, dettaglio piccolo variabile: vale anche per una mappa politica (confini netti, decorazione regionale procedurale) [INF].

## Bad North (Plausible Concept, 2018)

- **Memorabile.** Isole-diorama assemblate con Wave Function Collapse da tileset fatti a mano [V] (fonti/badnorth-80lv-wfc-diorami.md:27-33). Oltre 300 tessere modellate in Maya e portate in Unity [V] (fonti/townscaper-gamedeveloper-come-funziona.md:75).
- **Perché costa poco.** Scena piccola e chiusa (un'isola), inquadratura fissa, pochi materiali [INF]. Requisiti non estratti dalla cattura [NON TROVATO] (fonti/steam-bad-north.md).
- **Cosa si riprende.** Il diorama come cornice: una porzione di mondo isolata e leggibile, utile per la vista ravvicinata di una regione o di una battaglia [INF].

## Islanders (GrizzlyGames, 2019)

- **Memorabile.** Tre persone, quattro mesi; a ogni decisione la domanda "possiamo farlo più semplice?" [V] (fonti/islanders-gameworldobserver-intervista.md:115-121). Tagliato il ciclo giorno/notte per semplificare [V] (fonti/islanders-gameworldobserver-intervista.md:127).
- **Perché costa poco.** 2 GB di RAM e 200 MB su disco; GPU minima indicata GTX 950, quindi non è il più leggero del gruppo [V] (fonti/steam-islanders.md:95-97).
- **Cosa si riprende.** Il metodo di taglio, più dell'aspetto.

## DEFCON (Introversion, 2006)

- **Memorabile.** "Volevamo un look semplice e stilizzato... che ti sentissi un generale sepolto nel NORAD"; molte iterazioni per arrivare al bagliore e al tono giusti; musica e paesaggio sonoro rendono il gioco "cupo e tragico" [V] (fonti/defcon-escapist-everybody-dies.md:45). Anni dopo la grafica vettoriale è descritta come "senza tempo" [V] (fonti/defcon-escapist-everybody-dies.md:41).
- **Perché costa poco.** 1 GB di RAM, 64 MB di memoria video [V] (fonti/steam-defcon.md:47-48). Linee vettoriali su fondo scuro: nessuna texture, il bagliore è un solo passaggio di post-processing [INF].
- **Cosa si riprende.** È il riferimento più vicino al nostro tema: mappa del mondo, militare, decisioni con conseguenze tragiche. Si prende il principio (il tono viene da luce e suono, non dal dettaglio). Non si prende la combinazione vettori luminosi su nero, che è sua ed è anche un cliché da "sala comando".

## Per il nostro gioco

- **Prendiamo:** una metafora visiva unica, derivata dal tema e decisa presto; il livello di astrazione scelto esplicitamente (Mini Motorways); forme grandi prevedibili e dettaglio piccolo variabile (Townscaper); il taglio sistematico (Islanders); il suono come canale di informazione (Mini Metro, DEFCON).
- **Evitiamo:** copiare la firma visiva di uno di questi titoli; il tono "cozy" applicato a guerra e repressione, che stona [INF].
- **Aperto:** quale metafora. Le candidate sono in [[direzioni-artistiche-candidate]].

## Riferimenti

- **URL** https://en.wikipedia.org/wiki/Mini_Metro_(video_game) · [V]
  **perché conta qui** storia e riconoscimenti di Mini Metro, fonte secondaria ma con rimandi primari
  **cosa se ne prende** stile da mappa di trasporto, forme come identità, IGF audio 2016
- **URL** https://designingsound.org/2016/02/18/the-programmed-music-of-mini-metro-interview-with-rich-vreeland-disasterpeace/ · [V]
  **perché conta qui** intervista primaria al compositore che ha progettato il sistema audio
  **cosa se ne prende** unità forma-movimento-suono; limiti numerici che reggono la leggibilità
- **URL** https://store.steampowered.com/app/287980/Mini_Metro/ · [V]
  **perché conta qui** requisiti di sistema dichiarati dallo sviluppatore
  **cosa se ne prende** 2 GB RAM, SM 4.0, 350 MB; modalità daltonici e notte
- **URL** https://miketendo64.com/2022/08/14/interview-mini-motorways/ · [V]
  **perché conta qui** intervista allo studio sulla scelta del livello di astrazione
  **cosa se ne prende** la domanda "quanto astratto" come decisione di pre-produzione
- **URL** https://store.steampowered.com/app/1127500/Mini_Motorways/ · [V]
  **perché conta qui** requisiti di sistema
  **cosa se ne prende** SM 4.0, 350 MB
- **URL** https://www.gamedeveloper.com/disciplines/sparking-joy-through-tile-placement-in-idyllic-village-builder-i-dorfromantik-i- · [V]
  **perché conta qui** intervista primaria a Toukana su direzione artistica e feedback
  **cosa se ne prende** paesaggio idealizzato; interazione più frequente = più soddisfacente; strumenti (Unity, Blender, DOTween)
- **URL** https://www.digitaltrends.com/gaming/dorfromantik-interview/ · [V]
  **perché conta qui** citazioni dirette dell'artista sui riferimenti pittorici
  **cosa se ne prende** naïf per le forme, impressionismo per il colore; pilastro minimalismo
- **URL** https://store.steampowered.com/app/1455840/Dorfromantik/ · [V]
  **perché conta qui** requisiti di sistema di un 3D stilizzato
  **cosa se ne prende** Intel HD 520, 3 GB RAM, 650 MB: soglia raggiungibile per un 3D low-poly
- **URL** https://www.gamedeveloper.com/game-platforms/how-townscaper-works-a-story-four-games-in-the-making · [V]
  **perché conta qui** analisi tecnica con citazioni di Stålberg
  **cosa se ne prende** griglia irregolare, decorazione differita, stencil per le finestre, forme grandi prevedibili
- **URL** https://store.steampowered.com/app/1291340/Townscaper/ · [V]
  **perché conta qui** scheda del prodotto
  **cosa se ne prende** nulla sui requisiti: la cattura non li contiene
- **URL** https://80.lv/articles/using-wave-function-collapse-algorithm-for-dioramas · [V]
  **perché conta qui** descrizione del talk di Stålberg su Bad North
  **cosa se ne prende** diorama da tileset fatto a mano con WFC
- **URL** https://store.steampowered.com/app/688420/Bad_North_Jotunn_Edition/ · [V]
  **perché conta qui** scheda del prodotto
  **cosa se ne prende** nulla sui requisiti: la cattura non li contiene
- **URL** https://gameworldobserver.com/2019/06/14/islanders · [V]
  **perché conta qui** intervista all'artista co-fondatore
  **cosa se ne prende** il metodo "possiamo farlo più semplice?"
- **URL** https://store.steampowered.com/app/1046030/ISLANDERS/ · [V]
  **perché conta qui** requisiti di sistema
  **cosa se ne prende** 2 GB RAM, 200 MB, GTX 950
- **URL** https://www.escapistmagazine.com/everybody-dies-in-defcon/ · [V]
  **perché conta qui** citazione diretta di Mark Morris sulla direzione artistica
  **cosa se ne prende** look stilizzato, bagliore e tono iterati, suono che dà la tragedia
- **URL** https://store.steampowered.com/app/1520/DEFCON/ · [V]
  **perché conta qui** requisiti di sistema
  **cosa se ne prende** 1 GB RAM, 64 MB video

## Collegamenti

- [[interfacce-di-giochi-gestionali]]: come gli stessi principi reggono (o no) sotto molta informazione.
- [[movimento-e-feedback]]: la regola Dorfromantik applicata a un gioco a turni.
- [[suono-e-musica-leggeri]]: il sistema audio di Mini Metro nel dettaglio.
- [[shader-ed-effetti-economici]]: le tecniche dietro diorama e vettori luminosi.
- [[direzioni-artistiche-candidate]]: dove questi riferimenti diventano proposte.
