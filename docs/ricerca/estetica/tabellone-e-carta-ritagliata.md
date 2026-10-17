---
tipo: estetica
fonti: [fonti/twilight-struggle-pocketgamer-playdek.md, fonti/twilight-struggle-regole-deluxe-gmt.md, fonti/paxpamir-wehrle-building-the-box.md, fonti/root-moregamesplease-ferrin.md, fonti/lara-croft-go-gamesbeat-routon.md, fonti/cultist-simulator-itch-intervista.md, fonti/cultist-simulator-mcv-when-we-made.md, fonti/cultist-simulator-gamesbeat-gdc2018.md, fonti/iwata-paper-mario-3ds-1.md, fonti/iwata-paper-mario-3ds-2.md, fonti/iwata-paper-mario-3ds-3.md, fonti/iwata-paper-mario-3ds-4.md, fonti/tearaway-gameinformer-gdc2013.md, fonti/paper-trail-exputer-hoffman.md, fonti/lumino-city-mcv-carta-cartone.md, fonti/godot-styleboxflat.md, fonti/mdn-drop-shadow.md, fonti/mdn-feturbulence.md]
aggiornato: 2026-10-17
---
# Tabellone e carta ritagliata

Due metafore vicine alla carta stampata della direzione B, che possono renderla giocosa senza perdere la leggibilità. Il **tabellone** mette tutto lo stato del gioco sul tavolo: tracciati, segnalini, carte in fila. La **carta ritagliata** dà calore con strati piatti e ombre finte, senza 3D. Questa pagina raccoglie cosa ne dicono gli autori (interviste, diari di sviluppo, regolamenti), cosa costa farlo in 2D su Godot Compatibility e dove le due metafore stonano con crisi, spread e sfiducia. Chiude con tre stili candidati pronti per un prototipo HTML.

Tag: [V] fonte primaria letta · [M] misurato in sessione · [INF] inferenza · [NON TROVATO].

**Perimetro.** Mini Metro, Dorfromantik, Townscaper, Bad North, Islanders e DEFCON sono in [[riferimenti-minimalisti]] e qui non si ripetono. La pagina [[tono-giocoso-su-temi-seri]], scritta in parallelo da un altro agente, propone già uno stile da gioco da tavolo, «La Scatola di Stato» (plancia, pedine, gioco dell'oca per l'iter): gli stili candidati di questa pagina sono costruiti per **non** sovrapporsi a quello (vedi la sezione dedicata).

## Il tabellone

### Twilight Struggle (GMT, 2005; edizione digitale Playdek)

- **Lo stato sta sui tracciati.** La preparazione della partita mette un segnalino su ciascun tracciato: corsa allo spazio, operazioni militari di ciascuna superpotenza, registro dei turni, DEFCON (si parte da 5) e punti vittoria [V] (fonti/twilight-struggle-regole-deluxe-gmt.md:188-193). Ogni colpo di stato in un paese chiave abbassa il DEFCON di un livello verso la guerra nucleare [V] (fonti/twilight-struggle-regole-deluxe-gmt.md:414): la tensione globale è una casella, e la si vede scendere.
- **Le carte portano la storia.** Ogni carta è un evento storico con effetti modellati su quelli reali: «Iron Lady» dà influenza all'URSS in Argentina e le toglie quella nel Regno Unito [V] (fonti/twilight-struggle-pocketgamer-playdek.md:28-30). L'intreccio degli effetti è il fascino del gioco e anche ciò che rende difficile scriverne l'IA [V] (fonti/twilight-struggle-pocketgamer-playdek.md:32).
- **Il passaggio allo schermo.** Playdek vuole che il giocatore senta di usare «un gioco da tavolo digitale» e non di toccare uno schermo [V] (fonti/twilight-struggle-pocketgamer-playdek.md:40). Una «Expert UI» opzionale condensa tutta l'informazione in una schermata sola, ma, ammette lo sviluppatore, travolgerebbe i nuovi giocatori; il gioco applica le regole al posto del giocatore [V] (fonti/twilight-struggle-pocketgamer-playdek.md:42, :50).
- **Cosa se ne prende.** Indicatori come tracciati a caselle con zone di pericolo, sul modello del DEFCON; mai tutto in una schermata sola per chi comincia.

### Pax Pamir, seconda edizione (Cole Wehrle, Wehrlegig)

- **Il tono dichiarato.** «Pax Pamir è un gioco sulla politica e sulla guerra», presentato con sobrietà; può essere anche buffo, ma la comicità sta nei colpi di scena assurdi e nei piani che falliscono in modo tragico [V] (fonti/paxpamir-wehrle-building-the-box.md:58).
- **Astrazione dei pezzi.** Mappa e pezzi seguono la tradizione dei giochi non rappresentativi come il Pachisi e gli scacchi; fanno da contrappunto alle carte, illustrate con stampe d'epoca [V] (fonti/paxpamir-wehrle-building-the-box.md:60, :64).
- **La carta come fondo.** Le carte hanno una texture di carta marrone chiaro: secondo l'autore è un fondo facile da contrastare, che fa «saltare fuori» i colori [V] (fonti/paxpamir-wehrle-building-the-box.md:64).
- **Contro lo scheumorfismo.** Gli ornamenti che imitano oggetti antichi o fotografici, nella grafica dei giochi da tavolo, sono spesso usati senza pensarci e invecchiano male; la prima edizione ne era piena (semi come sigilli di ceralacca, località stampate su un finto foglietto) [V] (fonti/paxpamir-wehrle-building-the-box.md:70-72). Nella seconda edizione Wehrle abbandona le icone smussate per uno stile semipiatto, con un contorno irregolare che suggerisce il disegno a mano «senza enfatizzarlo» [V] (fonti/paxpamir-wehrle-building-the-box.md:74).
- **Colore e gravità.** Per la scatola un colore chiaro non avrebbe trasmesso la serietà del gioco, quindi la scelta è caduta su un viola scuro, che nel gioco è anche il colore del seme politico [V] (fonti/paxpamir-wehrle-building-the-box.md:88).
- **Cosa se ne prende.** Il registro «sobrio con esiti assurdi», che è esattamente il giocoso che serve a un simulatore di governo; contorni appena irregolari, niente finti oggetti antichi.

### Root (Leder Games; illustrazioni di Kyle Ferrin)

- **Guerra senza storia reale.** Root nasce come gioco di guerra non legato a un contesto storico preciso, quindi con regole del mondo inventate e animali antropomorfi [V] (fonti/root-moregamesplease-ferrin.md:94).
- **Leggibilità a colpo d'occhio.** I giocatori scelgono la fazione per tema ed estetica, non per la meccanica; l'illustrazione deve dare «informazione a colpo d'occhio attraverso la comunicazione visiva» [V] (fonti/root-moregamesplease-ferrin.md:72). La «presenza al tavolo» conta quanto poter leggere lo stato della partita [V] (fonti/root-moregamesplease-ferrin.md:114).
- **Mestiere.** Texture della carta e linea pesante portano a valori chiari, per non perdere il dettaglio; la palette della plancia è campionata da fotografie di fogliame autunnale [V] (fonti/root-moregamesplease-ferrin.md:106, :108). Attenzione ai daltonici come compromesso che vale sempre la pena [V] (fonti/root-moregamesplease-ferrin.md:116). Già in Vast i goblin dovevano sembrare «più dispettosi che violenti» per restare un gioco per famiglie [V] (fonti/root-moregamesplease-ferrin.md:62).
- **Cosa se ne prende.** La forma carina governa il tono di un tema violento, ma a una condizione: la violenza resta nel sistema, non nel disegno. Colori presi da una fonte reale e coerente, non da una ruota cromatica.

### Hitman GO e Lara Croft GO (Square Enix Montréal)

- **Il gioco da tavolo come oggetto.** Hitman doveva essere «elegante e di classe», da cui l'idea di farne un gioco da tavolo raffinato, «l'oggetto che metti su uno scaffale» [V] (fonti/lara-croft-go-gamesbeat-routon.md:54-56). Il prototipo era fatto di pezzetti di carta mossi a mano a ogni turno [V] (fonti/lara-croft-go-gamesbeat-routon.md:62). La squadra era di circa 10 persone [V] (fonti/lara-croft-go-gamesbeat-routon.md:16).
- **Niente tutorial, tutto evidente.** «Non ci piacciono i tutorial»: molto lavoro perché tutto fosse impossibile da non vedere [V] (fonti/lara-croft-go-gamesbeat-routon.md:94). Per rendere chiaro un contatore di turni (leva e porta) un grafico ha preparato una matrice 10×10 con cento visualizzazioni possibili, provate una alla volta sui giocatori [V] (fonti/lara-croft-go-gamesbeat-routon.md:114-116).
- **Quando la metafora non regge.** Per Lara Croft hanno abbandonato l'estetica da gioco da tavolo: lei doveva essere animata, e il tabellone «non aveva senso» [V] (fonti/lara-croft-go-gamesbeat-routon.md:132).
- **Cosa se ne prende.** Il prototipo di carta come primo test; il contatore di un indicatore va iterato come un pezzo di design a sé; la metafora si sceglie sul soggetto, non sulla moda.

### Cultist Simulator (Weather Factory): il tavolo di carte

- **Un tavolo, una meccanica.** L'unica meccanica è combinare carte per ottenerne di nuove, e ogni combinazione fa avanzare la storia [V] (fonti/cultist-simulator-itch-intervista.md:26; fonti/cultist-simulator-mcv-when-we-made.md:38). Il giocatore «impara la lingua del gioco», compreso il significato della combinazione di colori; se qualcosa si rompe nell'interfaccia, si rompe il divertimento [V] (fonti/cultist-simulator-mcv-when-we-made.md:58, :62).
- **Le persone come gettoni.** Se tutto è una carta, gli esseri umani diventano intercambiabili con gli spiccioli o un foglietto di appunti: «un modo solitario e crudele di controllare il mondo» [V] (fonti/cultist-simulator-itch-intervista.md:28). Bevan lo rivendica: trattare le persone come oggetti «fa un po' orrore», ed è voluto [V] (fonti/cultist-simulator-gamesbeat-gdc2018.md:20).
- **Rompere la metafora apposta.** Il tavolo deve diventare così onnipresente da sparire; poi, in un momento raro, il tavolo scompare e lascia posto a una mappa ultraterrena [V] (fonti/cultist-simulator-gamesbeat-gdc2018.md:34-36).
- **Costo.** Riusare il tavolo per città o epoche nuove costa relativamente poco, «soprattutto i dorsi delle carte»; l'obiettivo dichiarato è tenere bassi i costi [V] (fonti/cultist-simulator-gamesbeat-gdc2018.md:42). Sviluppato in circa 11 mesi [V] (fonti/cultist-simulator-mcv-when-we-made.md:48).

## La carta ritagliata

### Paper Mario: Sticker Star (Intelligent Systems, Nintendo 3DS)

- **Origine dello stile.** Nel 1997 un giovane designer disegna un'immagine «di poligoni 3D» con l'atmosfera di un libro illustrato: sfondi e personaggi sottili come la carta. L'idea era usare la potenza del 3D per esaltare l'aspetto 2D [V] (fonti/iwata-paper-mario-3ds-1.md:74, :78, :86).
- **La forma segue la funzione.** Secondo Miyamoto il design «alla Mario» nasce dalla funzione: un personaggio con le spine fa male se lo calpesti [V] (fonti/iwata-paper-mario-3ds-2.md:56-60).
- **Il gesto frequente regolato a fotogrammi.** Staccare un adesivo «si fa di continuo», quindi Miyamoto lo voleva più soddisfacente e più rapido: è stato regolato fotogramma per fotogramma, e la sensazione cambia con la dimensione dell'adesivo [V] (fonti/iwata-paper-mario-3ds-3.md:40, :44).
- **Una grammatica dei danni.** Per le battaglie il team si chiede come si danneggia la carta: piegarla, bagnarla, bruciarla, attaccarci il nastro adesivo e strapparlo, fissarla con una puntina [V] (fonti/iwata-paper-mario-3ds-4.md:28, :36).
- **Cosa se ne prende.** La metafora della carta ha già un vocabolario per gli stati negativi (bagnato, piegato, strappato, bruciato). Il gesto più frequente si rifinisce al fotogramma.

### Tearaway (Media Molecule)

- **La mano visibile.** «Non mi piacciono le cose digitali, mi piace vedere la mano di qualcun altro», dice Rex Crowle [V] (fonti/tearaway-gameinformer-gdc2013.md:12). La carta è stata scelta per creare «un'intersezione più giocosa fra il mondo del gioco e il mondo reale» [V] (fonti/tearaway-gameinformer-gdc2013.md:18).
- **Il vincolo che concentra.** Far passare la carta per ogni decisione ha aiutato a concentrarsi sul nucleo; il team ha riprodotto come la carta si piega, si flette e si strappa [V] (fonti/tearaway-gameinformer-gdc2013.md:24). I giocatori ricevono modelli da stampare e costruire, «come trofei fisici» [V] (fonti/tearaway-gameinformer-gdc2013.md:26).

### Paper Trail (Newfangled Games)

- **Prima la leggibilità, poi il colore.** Il meccanismo di piegatura imponeva di distinguere senza ambiguità le zone calpestabili da quelle no; questa distinzione ha retto il design di ogni ambiente, e solo dopo è arrivata l'identità visiva con una palette dedicata [V] (fonti/paper-trail-exputer-hoffman.md:60-62).
- **Cosa hanno tolto.** Rotazione e capovolgimento del mondo, perché i giocatori li trovavano disorientanti [V] (fonti/paper-trail-exputer-hoffman.md:44). Hanno aggiunto un tasto per vedere il retro del foglio e un sistema di suggerimenti [V] (fonti/paper-trail-exputer-hoffman.md:82).
- **Riuso.** Una meccanica ridisegnata con altre grafiche e animazioni sembra nuova [V] (fonti/paper-trail-exputer-hoffman.md:70).

### Lumino City e Kami (State of Play): il costo della carta vera

- Lumino City è fatto di carta, cartone e colla: ogni livello è un set fisico costruito a mano, e i set formano una città-modello alta circa tre metri [V] (fonti/lumino-city-mcv-carta-cartone.md:10). Le riprese hanno richiesto un braccio robotico da cinema a 2.500 sterline al giorno, per 150 inquadrature, e dopo le riprese sono serviti altri due anni di sviluppo [V] (fonti/lumino-city-mcv-carta-cartone.md:24-28, :32). La protagonista è animata in Flash con fotografie di carta colorata, più grana e sfocatura via codice per farla stare nella scena [V] (fonti/lumino-city-mcv-carta-cartone.md:34, :38).
- Nel frattempo lo stesso studio ha fatto Kami, che ha finanziato la chiusura di Lumino City [V] (fonti/lumino-city-mcv-carta-cartone.md:42-44).
- **Cosa se ne prende.** La carta fotografata è il limite alto, fuori portata. Il percorso praticabile è la carta disegnata a tinte piatte [INF].

## Le quattro domande

### 1. Come il tabellone rende leggibile un sistema complesso

- **Ogni grandezza ha un posto fisso e un segnalino.** In Twilight Struggle lo stato globale è la posizione di cinque segnalini su cinque tracciati (fonti/twilight-struggle-regole-deluxe-gmt.md:188-193) [V]. Il giocatore legge lo stato guardando *dove* sta il segnalino, prima ancora del numero [INF]. Per noi coincide con i tre indicatori sintetici sempre a schermo, cioè consenso, maggioranza e spread (docs/ricerca/sintesi/decisioni.md:122).
- **Le soglie si vedono come zone del tracciato.** Il DEFCON va da 5 a 1, e un'azione specifica lo fa scendere (fonti/twilight-struggle-regole-deluxe-gmt.md:414) [V]. Lo spread come tracciato a zone (calma, attenzione, tensione, crisi) rende la soglia un luogo, non un numero da ricordare [INF].
- **Le carte in fila sono un'agenda.** In Twilight Struggle ogni turno porta una mano di carte-evento (fonti/twilight-struggle-pocketgamer-playdek.md:28) [V]: le opzioni diventano oggetti contabili [INF]. Dieci ministri con un atto ciascuno si leggono bene come dieci carte [INF].
- **Il limite.** Mettere tutto su una schermata travolge chi comincia (fonti/twilight-struggle-pocketgamer-playdek.md:42) [V]. Sul tavolo sta lo *stato*; le regole le applica il gioco (fonti/twilight-struggle-pocketgamer-playdek.md:50) [V], e i dettagli stanno nei pannelli che si aprono [INF].

### 2. Come la carta ritagliata dà calore senza 3D

- **La mano visibile** (fonti/tearaway-gameinformer-gdc2013.md:12) [V] e il contorno appena irregolare, non enfatizzato (fonti/paxpamir-wehrle-building-the-box.md:74) [V]: l'imperfezione piccola e coerente dà calore, quella grande diventa decorazione [INF].
- **Strati piatti, profondità finta.** Paper Mario usa il 3D per sembrare 2D (fonti/iwata-paper-mario-3ds-1.md:86) [V]. Noi possiamo fare il contrario, cioè un 2D con due o tre strati e un'ombra dura che dice «questo pezzo sta sopra» [INF].
- **Un fondo di carta che fa risaltare i colori** (fonti/paxpamir-wehrle-building-the-box.md:64) [V] e valori chiari per non perdere la linea (fonti/root-moregamesplease-ferrin.md:106) [V].
- **Il gesto tattile.** Staccare, incollare, piegare: Paper Mario rifinisce al fotogramma il gesto più frequente (fonti/iwata-paper-mario-3ds-3.md:40) [V], la stessa regola di Dorfromantik già in [[movimento-e-feedback]].

### 3. Cosa si fa in 2D con produzione bassa o media

- **Godot Compatibility regge il 2D.** La tabella ufficiale dà il 2D come pienamente supportato e Compatibility come «di solito sufficiente» per il 2D (fonti/godot-renderers.md:78) [V, cattura di altro agente]. Manca però l'MSAA 2D (fonti/godot-renderers.md:131) [V, cattura di altro agente]: i bordi dei pezzi ritagliati vanno antialiasati per altra via [INF].
- **Pannelli con ombra senza texture.** `StyleBoxFlat` disegna bordi arrotondati, antialiasing, ombre e inclinazione senza texture (fonti/godot-styleboxflat.md:14) [V]; l'ombra ha offset e dimensione in pixel (fonti/godot-styleboxflat.md:277, :286) [V], e l'antialiasing aggiunge un anello sfumato ai bordi (fonti/godot-styleboxflat.md:81) [V]. Per un'ombra dura da pezzo ritagliato, la via più semplice è un secondo pannello pieno, scuro e spostato, dietro al primo [INF].
- **Nel prototipo HTML.** `filter: drop-shadow()` segue la forma (il canale alfa) del pezzo, mentre `box-shadow` segue il riquadro (fonti/mdn-drop-shadow.md:43-45) [V]: è quello che serve per i ritagli irregolari. `feTurbulence` sintetizza una texture di rumore (fonti/mdn-feturbulence.md:17) [V], utile per la grana della carta o per i bordi strappati con uno spostamento. In Godot conviene invece una texture di rumore statica, generata una volta sola [INF].
- **Produzione bassa:** palette, pannelli, tracciati, segnalini geometrici, carte con dorso unico. Cultist Simulator riusa lo stesso tavolo e cambia i dorsi delle carte (fonti/cultist-simulator-gamesbeat-gdc2018.md:42) [V].
- **Produzione media:** 20 sagome di regione ritagliate (si ricavano dai confini), una decina di illustrazioni piatte per i dicasteri, ritratti-sagoma dei ministri inventati [INF].
- **Fuori portata:** carta vera fotografata, come in Lumino City (fonti/lumino-city-mcv-carta-cartone.md:24-32) [V].
- **Testo.** Su Steam Deck il carattere più piccolo non deve scendere sotto i 9 px di altezza a 1280×800, e Valve consiglia di puntare a 12 px (fonti/steamworks-deck-compat.md:108) [V, cattura di altro agente]. Rotazioni e texture vanno quindi tenute **fuori** dal testo dei dati [INF].

### 4. Dove le metafore stonano con crisi, spread e sfiducia

- **Le persone come pedine.** Cultist Simulator dichiara che trattare le persone come gettoni è crudele e voluto (fonti/cultist-simulator-itch-intervista.md:28; fonti/cultist-simulator-gamesbeat-gdc2018.md:20) [V]. Da noi seggi e voti possono essere segnalini, perché sono già unità contabili. I gruppi sociali e chi viene colpito da una crisi, invece, non vanno trattati come pedine da scartare [INF], in linea con la regola «lo scherzo non colpisce le vittime» di [[tono-giocoso-su-temi-seri]].
- **Il dado suggerisce il caso.** Il modello ha rumore piccolo (docs/ricerca/sintesi/decisioni.md:68), ma gli esiti hanno cause. Il dado va usato al massimo come icona statica del margine d'errore del sondaggio, mai come animazione che «decide» [INF]. Concorda con la Scatola di Stato, che esclude i dadi che rotolano.
- **Il carino copre il grave.** Root regge la guerra con animali carini perché la violenza sta nel sistema e il disegno resta «dispettoso più che violento» (fonti/root-moregamesplease-ferrin.md:62) [V]. Lo spread a 400 punti, invece, non deve rimbalzare. Il registro serio c'è già: per Pamir il colore chiaro non comunicava serietà (fonti/paxpamir-wehrle-building-the-box.md:88) [V], quindi nelle crisi la palette scurisce e il movimento si ferma [INF].
- **Dentro la metafora, prima di romperla.** Paper Mario ha un vocabolario dei danni alla carta (fonti/iwata-paper-mario-3ds-4.md:28) [V]. Una crisi può essere carta bagnata o stropicciata (texture statica) senza uscire dal mondo [INF]. Per il momento più grave (la sfiducia, la crisi del debito) Cultist Simulator mostra l'altra strada: far sparire il tavolo (fonti/cultist-simulator-gamesbeat-gdc2018.md:36) [V]. Da noi: la plancia si toglie e resta il testo nudo della notizia [INF].
- **La metafora non si adatta a tutto.** Lara Croft GO ha abbandonato il tabellone perché non funzionava per quel personaggio (fonti/lara-croft-go-gamesbeat-routon.md:132) [V]. Per noi il punto debole è il militare [INF], lo stesso segnalato per B in [[direzioni-artistiche-candidate]].
- **Sfiducia e opposizione.** Perdere il governo porta all'opposizione, non al game over (docs/ricerca/sintesi/decisioni.md:77). Sul tabellone è un cambio di lato del tavolo, non una pedina eliminata [INF].

## Tre stili candidati

I tre stili restano dentro **B · Dossier con rete**: carta stampata più diagramma di sistema, prima pagina dell'esito, listino dell'agenda dei 10 ministri, schema dello Stato come vista secondaria, mappa delle regioni a tinte piatte. Si distinguono dalla «Scatola di Stato» di [[tono-giocoso-su-temi-seri]], che usa angoli tondi da gioco in scatola, Bricolage Grotesque con Recursive e il gioco dell'oca per l'iter. Contrasti [M]: rapporto WCAG calcolato in sessione con uno script (formula della luminanza relativa, verificata su nero/bianco = 21:1). Licenze [M]: `gh api repos/google/fonts/contents/ofl/<famiglia>/OFL.txt`, nessun Reserved Font Name nell'intestazione delle famiglie scelte.

### 1. Plancia a tracciati

**Metafora:** il governo come un wargame politico da scaffale: l'Italia è il tabellone, ai bordi corrono i tracciati, sopra si muovono blocchetti di legno.

- **Palette:** cartone della plancia `#EFE6D2` (fondo dei pannelli); inchiostro blu notte `#1F2A44` (testo, 11,49:1); noce del tavolo `#5C3A21` (cornice e fondo esterno, testo color cartone sopra a 8,13:1); rosso blocchetto `#B3261E` (allarme, testo su cartone a 5,27:1); verde blocchetto `#246B4D` (esito favorevole, 5,15:1); ocra segnalino `#E3A72F` (solo come fondo, inchiostro sopra a 6,68:1); grigio caselle `#8C8270` (filetti dei tracciati, non testo, 3,05:1). Mappa: `#C9D8C5` `#E8C4B8` `#F1D9A0` `#BFD0E0` `#D8C3DE`, con inchiostro sopra da 8,68 a 10,30:1.
- **Font:** titoli Zilla Slab 700, come il dorso di una scatola (https://fonts.google.com/specimen/Zilla+Slab); testo Chivo (https://fonts.google.com/specimen/Chivo); cifre Chivo Mono (https://fonts.google.com/specimen/Chivo+Mono).
- **Forme:** angoli vivi sulla plancia, raggio di 2 px sulle carte. Contorni in inchiostro da 1,5 px, con un tremolio appena visibile sui pezzi illustrati (un path SVG con seme fisso; dritti sui dati). Caselle dei tracciati da 28×28 px. I blocchetti sono quadrati da 18 px con `box-shadow: 0 2px 0 rgb(0 0 0 / .25), inset 0 1px 0 rgb(255 255 255 / .35)`. Carte con grana `feTurbulence` a opacità 0,06. Niente smussi né finta ceralacca.
- **Movimento e succo:** il blocchetto avanza di casella in casella (80 ms a casella, 600 ms al massimo) con un «toc» di legno all'arrivo. Lo spread sale a scatti contabili. La carta evento si posa sul tavolo con l'ombra che si accorcia da 6 a 2 px in 180 ms. Con movimento ridotto il blocchetto compare direttamente nella casella.
- **Cruscotto del turno:** la plancia stessa. Consenso sul tracciato superiore (0-100, caselle da 5); maggioranza come tracciato dei seggi con la soglia marcata; spread come tracciato a quattro zone, sul modello del DEFCON; il trimestre sul registro dei turni con segnalino ocra.
- **Prima pagina dell'esito:** il «bollettino» stampato sul retro di una grande carta evento: al massimo 7 voci numerate, ognuna con il blocchetto che si è mosso e di quante caselle.
- **Listino dell'agenda:** 10 carte-ministro in fila lungo il bordo della plancia, come il mercato di un gioco di carte: atto, punteggio in Chivo Mono e due gettoni, Ferma e Dirotta, da posare sopra.
- **Schema dello Stato:** una rete di rotte fra caselle-istituzione (Consiglio dei ministri, Camera, Senato, Quirinale…). Gli atti in transito sono blocchetti sulla rotta e lo spessore della rotta è il flusso. È una rete, non il percorso lineare di un gioco dell'oca.
- **Mappa a tinte piatte:** 20 regioni come aree del tabellone, con cinque tinte, confini in inchiostro da 1 px e numero della regione nel cerchietto; i blocchetti sopra segnano opere ed eventi.
- **Riferimenti:** Twilight Struggle (tracciati, DEFCON, carte-evento), Pax Pamir (sobrietà, stile semipiatto, carta come fondo), Root (presenza al tavolo, colore da una fonte reale), Playdek (il limite dell'Expert UI).
- **Rischio principale:** ricadere nel «troppo serio» da wargame, con troppi tracciati. Si tiene a tre tracciati fissi; il resto va nei pannelli.

### 2. Ritaglio a strati

**Metafora:** lo Stato come un collage di cartoncini colorati, ritagliati e incollati a strati; più un pezzo sta in alto, più conta adesso.

- **Palette:** piano di lavoro `#E9E2D3` (fondo); foglio bianco `#FFFFFF` (pannelli di testo); inchiostro `#222222` (15,91:1 sul bianco, 12,34:1 sul piano); arancio forbice `#A8380C` (accento, testo a 6,49:1 sul bianco e 5,03:1 sul piano; testo bianco sopra a 6,49:1); blu cobalto `#2F4EA1` (secondario e link, 7,71:1 sul bianco); ombra `rgb(34 34 34 / .28)`. Cartoncini per mappa e carte: salvia `#8DB596`, rosa `#F2A7A0`, giallo `#F6D365`, azzurro `#9DC3E6`, albicocca `#F08A5D`, con inchiostro sopra da 6,44 a 10,93:1.
- **Font:** titoli Fraunces con asse SOFT a 100 e WONK attivo, solo da 20 px in su (https://fonts.google.com/specimen/Fraunces); testo Atkinson Hyperlegible Next (https://fonts.google.com/specimen/Atkinson+Hyperlegible+Next); cifre Atkinson Hyperlegible Mono (https://fonts.google.com/specimen/Atkinson+Hyperlegible+Mono).
- **Forme:** pezzi con angoli vivi «a forbice», poligoni da 5-8 vertici appena fuori squadra, ruotati di ±0,5-1,5°. Ombra dura senza sfocatura: `filter: drop-shadow(2px 3px 0 rgb(34 34 34 / .28))`. Al massimo tre strati. Bordo strappato (maschera SVG con `feTurbulence` e `feDisplacementMap`) solo sui pezzi decorativi. **I dati stanno sempre su foglio bianco dritto, a 0°.** In Godot: `Polygon2D` per il ritaglio più un duplicato scuro spostato per l'ombra, bordi antialiasati (niente MSAA 2D).
- **Movimento e succo:** il gesto frequente è incollare. Alla conferma di un atto il pezzo scende sul suo strato, l'ombra passa da 6 a 2 px e c'è una schiacciata minima (scala da 1,03 a 1, 160 ms), regolata come l'adesivo di Paper Mario. Alla selezione di una regione il pezzo si solleva (ombra da 2 a 5 px, −2 px in verticale, 120 ms). A fine turno i ritagli si incollano uno alla volta. Con movimento ridotto spariscono rotazioni e sollevamenti.
- **Cruscotto del turno:** tre cartellini ritagliati in alto (consenso, maggioranza, spread), con la cifra grande in Atkinson Mono; il trimestre su un'etichetta pinzata. Il cartellino che peggiora sale in cima alla pila.
- **Prima pagina dell'esito:** un collage di ritagli di giornale su un foglio bianco: testata in Fraunces, al massimo 7 ritagli incollati in ordine, il titolo spiritoso sul ritaglio e il fatto con il numero sempre dritto.
- **Listino dell'agenda:** 10 strisce di cartoncino, una per dicastero e del suo colore, impilate come un blocchetto. Ferma piega la striscia a metà; Dirotta la taglia e la riattacca all'altro atto.
- **Schema dello Stato:** nodi come cartellini ritagliati, collegati da strisce di nastro di carta il cui spessore è il flusso. Niente spago rosso da bacheca del complotto.
- **Mappa a tinte piatte:** 20 regioni di cartoncino, incastrate con 1 px di fessura che lascia vedere il piano sotto; la regione selezionata si alza.
- **Riferimenti:** Paper Mario (2D sottile, gesto regolato al fotogramma, vocabolario dei danni alla carta), Tearaway (la mano visibile), Paper Trail (prima la leggibilità, poi il colore), Lumino City e Kami (dove sta il limite di costo).
- **Rischio principale:** il collage diventa caos e i numeri si perdono, oppure scivola nel «lavoretto delle elementari» che infantilizza la politica. La regola che lo tiene in piedi: rotazione e strappo mai sui dati.

### 3. Teatrino di carta

**Metafora:** Palazzo Chigi come un teatrino di carta: quinte piatte una dietro l'altra, ministri come sagome di cartone sul piedistallo; a ogni turno si apre il sipario.

- **Palette:** fondale `#F6EBDD`; inchiostro seppia `#2A1E17` (13,76:1); rosso sipario `#9E2B25` (come fondo con testo color fondale, e come testo, 6,31:1); blu quinta `#23395B` (fondo con testo color fondale, 9,87:1); oro cornice `#D9A441` (fondo, inchiostro sopra a 7,20:1); verde `#8FB592` (fondo, 7,09:1); seppia chiaro `#6B5B4E` (note secondarie, 5,52:1). Mappa: `#E9B7A6` `#A9C6DE` `#8FB592` `#D9A441` `#F6EBDD`, con inchiostro sopra da 7,09 a 13,76:1.
- **Font:** titoli Young Serif (https://fonts.google.com/specimen/Young+Serif); testo Figtree (https://fonts.google.com/specimen/Figtree); cifre JetBrains Mono (https://fonts.google.com/specimen/JetBrains+Mono).
- **Forme:** quattro piani fissi (fondale, quinta, sagome, proscenio) con una parallasse minima di 2-6 px quando si muove il cursore o lo stick. Sagome con un margine bianco di 3 px, come il taglio di una fustella, e ombra dura sul piano dietro; una linguetta-piedistallo sotto ogni sagoma; boccascena in oro piatto. In CSS sono livelli con `transform`; in Godot nodi su livelli separati.
- **Movimento e succo:** il sipario si apre solo in apertura di turno (400 ms). Quando annuncia l'atto, la sagoma del ministro entra di lato e si ferma con un dondolio sul piedistallo (da 2° a 0°, 300 ms). Ferma gira la sagoma sul retro bianco (200 ms); Dirotta scambia il cartiglio. In crisi il sipario resta socchiuso, la luce cala (tinta più scura) e nulla dondola.
- **Cruscotto del turno:** il proscenio, con consenso, maggioranza e spread su cartelli di scena appesi in alto e il trimestre sul cartellone; al centro, come fondale, la mappa o lo schema.
- **Prima pagina dell'esito:** a sipario chiuso, la locandina dell'edicola appesa al sipario, con al massimo 7 voci rivelate una alla volta.
- **Listino dell'agenda:** le 10 sagome in fila sul proscenio come una compagnia teatrale, ognuna con il cartiglio dell'atto e il punteggio.
- **Schema dello Stato:** nella versione economica, la rete disegnata sul fondale. Nella versione ricca, istituzioni come facciate piatte collegate da passerelle su cui camminano le sagomine degli atti.
- **Mappa a tinte piatte:** l'Italia come quinta ritagliata davanti al mare blu, regioni a tinte, opere come piccole sagome su piedistallo.
- **Riferimenti:** Hitman GO e Lara Croft GO (il diorama come oggetto da scaffale, nessun tutorial, il contatore iterato), Paper Mario (il libro illustrato a strati sottili), Tearaway.
- **Rischio principale:** il costo, perché servono 10 sagome di ministro più le varianti, e la lettura cinica che la politica sia una recita di marionette. È lo stile più vicino al «troppo giocoso».

## Per il nostro gioco

- **Prendiamo:** gli indicatori come tracciati con zone (Twilight Struggle); il registro «sobrio con esiti assurdi» (Pax Pamir); la carta come fondo che fa risaltare il colore; il gesto frequente regolato al fotogramma (Paper Mario); prima la leggibilità, poi il colore (Paper Trail); il vocabolario dei danni alla carta per gli stati di crisi; la scelta di togliere la metafora nel momento più grave (Cultist Simulator).
- **Evitiamo:** lo scheumorfismo antiquario (ceralacca, smussi, finta carta invecchiata); tutto in una schermata; persone e gruppi sociali come pedine da scartare; dadi animati; rotazioni e texture sul testo dei dati; la carta vera fotografata.
- **Aperto:** quale stile prototipare, confrontandolo con la Scatola di Stato e con il Settimanale di [[tono-giocoso-su-temi-seri]]; come si legge la soglia dei 9 px (corpo o altezza del carattere) sul Deck; come si antialiasano i ritagli in Compatibility senza MSAA 2D; resa del militare.

## Riferimenti

- **URL** https://www.gmtgames.com/living_rules/TS_Rules_Deluxe.pdf · [V]
  **perché conta qui** regolamento ufficiale di un gioco politico in cui lo stato globale è fatto di segnalini sui tracciati
  **cosa se ne prende** i cinque tracciati della preparazione; il DEFCON che scende a ogni colpo di stato, modello per lo spread a zone
- **URL** https://www.pocketgamer.com/twilight-struggle/talking-twilight-struggle-the-best-board-game-youve-never-heard-of/ · [V]
  **perché conta qui** intervista a Playdek e ai designer sul passaggio del tabellone allo schermo
  **cosa se ne prende** il «gioco da tavolo digitale»; il limite dell'Expert UI; le regole applicate dal gioco; carte modellate su eventi reali
- **URL** https://web.archive.org/web/20250418062850id_/https://wehrlegig.com/blogs/essays/building-the-box · [V]
  **perché conta qui** diario di sviluppo di Cole Wehrle sulla grafica di un gioco politico e di guerra (sito originale con limite di richieste, letto via Wayback)
  **cosa se ne prende** sobrietà con esiti assurdi; pezzi astratti; carta come fondo; no allo scheumorfismo; contorno irregolare non enfatizzato; colore scuro per la gravità
- **URL** https://www.moregamesplease.com/art-in-boardgames/2018/3/15/kyle-ferrin-art-in-board-games-31 · [V]
  **perché conta qui** intervista all'illustratore di Root, un gioco di guerra carino
  **cosa se ne prende** informazione a colpo d'occhio; presenza al tavolo; palette da fonte reale; daltonici; dispettoso più che violento
- **URL** https://gamesbeat.com/lara-croft-go-developer-on-turning-an-action-blockbuster-into-a-turn-based-mobile-game/ · [V]
  **perché conta qui** intervista ad Antoine Routon su Hitman GO e Lara Croft GO, il gioco da tavolo come estetica digitale
  **cosa se ne prende** oggetto da scaffale; prototipo di carta; nessun tutorial; il contatore iterato in cento varianti; quando abbandonare il tabellone
- **URL** https://itch.io/blog/37131/the-internet-is-not-a-kind-place-for-human-brains-an-interview-with-the-masterminds-behind-cultist-simulator · [V]
  **perché conta qui** gli autori spiegano il tavolo di carte come metafora e il suo costo morale
  **cosa se ne prende** una sola meccanica; le persone come gettoni, «crudele»
- **URL** https://mcvuk.com/development-news/when-we-made-cultist-simulator/ · [V]
  **perché conta qui** retrospettiva di produzione di Cultist Simulator
  **cosa se ne prende** la lingua del gioco, colori compresi; l'interfaccia rotta rompe il divertimento; 11 mesi di sviluppo
- **URL** https://gamesbeat.com/cultist-simulator-is-how-sunless-seas-creators-play-with-cards-and-reality/ · [V]
  **perché conta qui** intervista GDC 2018 a Kennedy e Bevan sulla metafora del tavolo
  **cosa se ne prende** trattare le persone come oggetti è voluto; la metafora rotta apposta in un momento raro; riuso del tavolo e dei dorsi per tenere bassi i costi
- **URL** https://iwataasks.nintendo.com/interviews/3ds/papermario/0/1/ · [V]
  **perché conta qui** origine dello stile «sottile come la carta» raccontata dagli autori
  **cosa se ne prende** libro illustrato; il 3D usato per sembrare 2D
- **URL** https://iwataasks.nintendo.com/interviews/3ds/papermario/0/2/ · [V]
  **perché conta qui** principio di Miyamoto sul design dei personaggi
  **cosa se ne prende** la forma segue la funzione (le spine fanno male)
- **URL** https://iwataasks.nintendo.com/interviews/3ds/papermario/0/3/ · [V]
  **perché conta qui** rifinitura del gesto più frequente
  **cosa se ne prende** l'adesivo che si stacca regolato fotogramma per fotogramma, diverso per dimensione
- **URL** https://iwataasks.nintendo.com/interviews/3ds/papermario/0/4/ · [V]
  **perché conta qui** come il team ha generato idee dalla carta come materiale
  **cosa se ne prende** il vocabolario dei danni (piegare, bagnare, bruciare, strappare il nastro) per gli stati di crisi
- **URL** https://www.gameinformer.com/b/news/archive/2013/03/28/media-molecule-talks-about-crafting-the-world-of-tearaway.aspx · [V]
  **perché conta qui** resoconto del talk GDC 2013 di Rex Crowle, con citazioni dirette
  **cosa se ne prende** la mano visibile; la carta come vincolo che concentra; il mondo di gioco che incontra quello reale
- **URL** https://exputer.com/interviews/paper-trail-storytelling-puzzle-blend/ · [V]
  **perché conta qui** intervista al direttore creativo di un gioco 2D recente costruito sulla carta piegata
  **cosa se ne prende** prima la leggibilità (calpestabile o no), poi la palette; tolta la rotazione perché disorienta; riuso con nuove grafiche
- **URL** https://mcvuk.com/development-news/crafting-the-lumino-city · [V]
  **perché conta qui** costo reale della carta fisica fotografata
  **cosa se ne prende** il limite alto, fuori portata; Kami come alternativa piatta dello stesso studio
- **URL** https://docs.godotengine.org/en/stable/classes/class_styleboxflat.html · [V]
  **perché conta qui** l'API 2D di Godot per pannelli con ombra senza texture
  **cosa se ne prende** ombra con offset e dimensione in pixel, antialiasing, inclinazione
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/filter-function/drop-shadow · [V]
  **perché conta qui** l'ombra del prototipo HTML
  **cosa se ne prende** `drop-shadow()` segue il canale alfa, `box-shadow` il riquadro
- **URL** https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/feTurbulence · [V]
  **perché conta qui** grana della carta e bordi strappati nel prototipo
  **cosa se ne prende** rumore di Perlin come filtro SVG
- **URL** https://fonts.google.com (famiglie Zilla Slab, Chivo, Chivo Mono, Fraunces, Atkinson Hyperlegible Next e Mono, Young Serif, Figtree, JetBrains Mono) · [M]
  **perché conta qui** font degli stili candidati
  **cosa se ne prende** licenza OFL 1.1 senza Reserved Font Name, verificata con `gh api repos/google/fonts/contents/ofl/<famiglia>/OFL.txt`
- **URL** https://boardgamegeek.com/thread/1864168 · [NON TROVATO]
  **perché conta qui** diario di sviluppo di Root di Cole Wehrle, sulle plance di fazione
  **cosa se ne prende** da recuperare: non aperto in sessione
- **URL** https://www.youtube.com/watch?v=0a9Rh997RPo · [NON TROVATO]
  **perché conta qui** postmortem GDC 2015 di Hitman GO (Daniel Lutz)
  **cosa se ne prende** da recuperare: è un video, non trascritto

Non coperti per scelta di tempo: Wingspan e Through the Ages digitali, Gorogoa, Paperback, Hegemony (gioco da tavolo sulle classi sociali con un tracciato delle politiche; nessuna fonte primaria catturata) [NON TROVATO].

## Collegamenti

- [[direzioni-artistiche-candidate]]: la direzione B, di cui questi stili sono varianti giocose.
- [[tono-giocoso-su-temi-seri]]: la Scatola di Stato, il Settimanale e l'Ufficio Protocollo (altro agente); regola dello scherzo che non colpisce le vittime.
- [[movimento-e-feedback]]: gesto frequente curato, esito a ritmo, niente rimbalzi sulle decisioni gravi.
- [[tipografia-e-font-aperti]]: OFL, cifre tabellari, al massimo tre famiglie.
- [[riferimenti-minimalisti]]: Bad North e il diorama, già coperti.
- [[mappe-e-globo]]: confini delle 20 regioni da cui ricavare le sagome.
- [[decisioni]]: tre indicatori sintetici, 7 voci, agenda con Ferma e Dirotta.
- [[godot]], [[prestazioni-su-hardware-modesto]]: vincoli del renderer e del Deck (altro agente).
