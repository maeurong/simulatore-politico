---
tipo: estetica
fonti: [fonti/zagnoli-about.md, fonti/salon-zagnoli-2012.md, fonti/pushingpixels-guasco-intervista.md, fonti/wiki-it-la-linea.md, fonti/sorrisi-bozzetto-2018.md, fonti/lambiek-bozzetto.md, fonti/wiki-en-mr-rossi.md, fonti/artribune-bozzetto-signor-rossi.md, fonti/wiki-it-ellekappa.md, fonti/gerdarntz-org-gerd-arntz.md, fonti/wiki-en-triadic-ballet.md, fonti/wiki-en-oskar-schlemmer.md, fonti/wiki-en-mr-men.md, fonti/wiki-en-miffy.md, fonti/wiki-en-barbapapa.md, fonti/wiki-en-charley-harper.md, fonti/iwataasks-wii-channels-4.md, fonti/wiki-en-mii.md, fonti/wiki-en-chernoff-face.md, fonti/dicebear-stile-thumbs.md]
aggiornato: 2026-11-12
---
# Riferimenti per le facce

Tredici sistemi di personaggi stilizzati, scelti perché sono belli da vedere e perché permettono di distinguere una figura dall'altra a colpo d'occhio. Servono da spunto per rifare le facce dei circa venti personaggi che stanno a schermo insieme (ministri, rose, leader), oggi disegnate dal segnaposto `faccia` che Mario ha giudicato «brutti da vedere» e «tutte uguali». Per ciascun riferimento la pagina dice quali leve usa per variare l'identità, che cosa lo rende piacevole invece che povero, se regge a 34 px e come fa a meno di capelli, occhiali e barbe. Chiude con quattro direzioni di stile dentro i sette inchiostri.

Tag: [V] fonte primaria letta · [M] misurato in sessione · [INF] inferenza · [NON TROVATO]. Le pagine di Wikipedia contano come fonti secondarie e sono segnate così.

**Artefatti consultati** (repo `simulatore-politico`, ramo `docs/piano-11-mappa`, HEAD `7d0a2e0`, 12/11/2026):
- prototipo `.scratch/v1-piano-11-ui/prototipi/08-fondamenta.prototype.html`: righe 48-50 (i sette inchiostri), 375-386 (`ACC`, gli oggetti dei dicasteri), 387-394 (`hash` e `faccia`), 422 (`faccia(x,34)` nelle righe dell'elenco dei ministri);
- [[caricature-di-personaggi-inventati]]: righe 46-53 (kit di circa 30 parti, hash di `ruolo:partito:nome`) e 70-73 (niente capelli, occhiali o barbe, per costruzione);
- [[grafica-italiana-giocosa]]: righe 99-109 (il metodo in cinque regole).

Non si rifà qui ciò che quelle due pagine coprono già: produzione, licenze, collisioni e il metodo di Munari, Luzzati, Quadratino, Cipputi e Altan.

## Perché il segnaposto stanca

Diagnosi [INF], costruita confrontando `faccia` con i riferimenti che seguono:

1. **Le tinte non sono i nostri inchiostri.** Il volto pesca da sei pastelli (`#E9B872`, `#D98E73`, `#9DB8A0`, `#C9A6C4`, `#E3C16F`, `#A8BFD6`) e il contorno è `#1b1b1b` (prototipo:391-394), mentre la pagina usa `#1F1A17` e sei inchiostri pieni (prototipo:48-50) [V]. La faccia sembra incollata sulla carta, non stampata. I sistemi che reggono, a cominciare da Bruna, usano pochi colori e sempre gli stessi (riferimento 8).
2. **Tutta la variazione sta dentro la testa, e gli occhi non si muovono mai.** Occhi sempre a `cx` 23 e 37 e a `cy` 26, bocca sempre in mezzo, busto sempre uguale (`#4a4d55`) (prototipo:392-394) [V]. Quattro forme per sei tinte danno 24 identità, ma tutte con la stessa espressione nella stessa posizione: a 34 px si leggono come un'unica faccia in cornici diverse. Chernoff, Thumbs e Mii mostrano che la posizione e l'inclinazione dei tratti sono leve forti quanto la forma (riferimenti 11-13).
3. **Manca la silhouette.** Contorno uniforme da 2,2 su ogni forma e busto identico: il profilo esterno della figura cambia poco. Nei Mr. Men e nei Barbapapà l'identità *è* la silhouette (riferimenti 7 e 9).
4. **L'oggetto compete con la testa.** L'oggetto del dicastero sta in basso a destra, sovrapposto (prototipo:375-386) [V]. Isotype lo mette dentro la sagoma, sul petto (riferimento 5).
5. **A 34 px i dettagli scendono sotto i 3 px** [M: aritmetica sul viewBox da 64 di prototipo:394, scala 34/64 = 0,53]: contorno 1,2 px, occhi di 2,8 px di diametro, 7,4 px fra un occhio e l'altro, testa larga circa 21 px. A quella scala si leggono solo la massa, la silhouette e il colore.

**Contrasto degli inchiostri** [M: luminanza relativa WCAG, python3, 12/11/2026]. Sulla carta `#F4ECD8`: inchiostro 14,6:1, biro 7,1:1, grigio 6,0:1, rosso 5,9:1, verde 5,4:1, **giallo 1,2:1**. Occhi d'inchiostro su una figura piena: su biro 2,1:1, su rosso 2,5:1, su verde 2,7:1, su giallo 11,9:1. Ne segue [INF]: una figura gialla senza contorno sparisce sulla carta, e gli occhi neri su un abito scuro scendono sotto la soglia di 3:1 che la WCAG chiede alla grafica non testuale (criterio 1.4.11, https://www.w3.org/WAI/WCAG22/Understanding/non-text-contrast.html, non catturata). Il rimedio di Thumbs è scegliere il colore degli occhi in contrasto con la forma (riferimento 13).

## Famiglia 1 · Illustrazione italiana e satira editoriale

### 1. Olimpia Zagnoli

- **Che cos'è.** Illustratrice di Milano, nata a Montecchio Emilia nel 1984, pubblicata su New York Times, New Yorker e Vogue Italia (fonti/zagnoli-about.md:13-15) [V]. La rivista Imprint descrive il suo lavoro come «a sorcery of rich color, economical shape and blank space» (fonti/salon-zagnoli-2012.md:19) [V]. Fra le sue influenze cita Matisse, Munari, Bass e Haring e definisce il suo stile «Graphic! Sixties! Europe!» (fonti/salon-zagnoli-2012.md:41) [V].
- **Leve d'identità** [INF dall'immagine]: la massa dei capelli come forma piatta che fa da cornice, il colore delle labbra, il collo e il colletto a motivo (scacchi, righe), il colore di fondo. Occhi spesso assenti o coperti.
- **Perché piace.** Pochi campi di colore pieno, senza contorno, accostati per contrasto. Il colore e la forma portano già un significato: «If you use a red, red is already a message. If you use a circle, that is something everyone can recognize» (fonti/salon-zagnoli-2012.md:43) [V].
- **A 34 px** [INF]: regge, perché tutto è massa e colore. Basta che due campi accostati non siano giallo e carta.
- **Capelli, occhiali, barbe.** Lo stile si regge sulla massa dei capelli, e nell'immagine scelta gli anelli olimpici fanno da occhiali. Per farne a meno, la massa che incornicia il volto diventa il bavero o il colletto dell'abito, oppure il fondo del riquadro [INF].
- **URL** https://www.olimpiazagnoli.com/about/ · [V]
  **immagine** https://oz.imbeard.com/wp-content/uploads/2025/10/OLIMPIADI26_DEFINITIVO-1463x2048.jpg
  **perché conta qui** è l'illustrazione italiana contemporanea più vicina alle tinte piatte del Settimanale, ed è bella senza contorno
  **cosa se ne prende** campi pieni accostati invece del contorno uniforme; il colletto a motivo come leva d'identità al posto dei capelli; un solo dettaglio forte per volto (le labbra rosse)

### 2. Riccardo Guasco

- **Che cos'è.** Illustratore e pittore di Alessandria. Nomina fra le sue fonti i futuristi, i suprematisti russi, i personaggi del Corriere dei Piccoli e i cartellonisti (Cappiello, Dudovich, Savignac), e cerca immagini «made up of few lines but full of expressive and emotional content» (fonti/pushingpixels-guasco-intervista.md:24) [V]. Nella serie dei ciclisti ha tolto la bicicletta e tutto il superfluo: restano «profiles speaking about struggle, noses stuck out» (fonti/pushingpixels-guasco-intervista.md:40) [V].
- **Leve d'identità** [INF dall'immagine]: profilo spezzato in piani (fronte, naso, mento come triangoli), lunghezza e angolo del naso, colore e forma del berretto, colletto della maglia.
- **Perché piace.** I volti sono costruiti con piani geometrici sfaccettati su carta calda, con tinte smorzate e un solo accento di colore (il berretto). Sono geometrici senza essere freddi.
- **A 34 px** [INF]: regge il profilo, perché la silhouette di naso e mento resta leggibile; si perdono le sfaccettature interne.
- **Capelli, occhiali, barbe.** Ne usa: baffi, ciuffi e berretti, perché i ciclisti sono persone vere (Coppi, Merckx). Per farne a meno si tiene il profilo a piani e al posto di baffi e ciuffi si mette un naso esagerato oltre ogni norma, con un colletto come costume [INF].
- **URL** https://www.pushing-pixels.org/2013/11/23/illustrators-at-work-interview-with-riccardo-guasco.html · [V]
  **immagine** https://www.pushing-pixels.org/wp-content/uploads/2013/11/cyclists.jpg
  **perché conta qui** è la versione italiana e futurista della «testa da forma elementare», ma portata sul profilo
  **cosa se ne prende** il naso come leva principale; volti costruiti con tre o quattro piani; un solo accento di colore per figura

### 3. Osvaldo Cavandoli · La Linea

- **Che cos'è.** Un omino che è parte della linea infinita su cui cammina, nato nel 1969 per il Carosello Lagostina (fonti/wiki-it-la-linea.md:12, :20) [V, secondaria]. Parla in grammelot e il colore dello sfondo dice il suo stato d'animo (fonti/wiki-it-la-linea.md:28) [V, secondaria].
- **Leve d'identità.** Il personaggio è uno solo, quindi non varia. Insegna però due cose: l'identità sta tutta nel profilo con il naso grande, e lo **stato** si può spostare fuori dal volto, sul fondo [INF].
- **Perché piace.** Un solo tratto bianco su fondo pieno: la povertà di mezzi diventa una regola dichiarata.
- **A 34 px** [INF]: il profilo regge come silhouette; il tratto unico, se sottile, scende sotto il pixel.
- **Capelli, occhiali, barbe.** Nessuno: profilo nudo con il naso.
- **URL** https://it.wikipedia.org/wiki/La_Linea_(personaggio) · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/it/7/79/La_Linea.png
  **perché conta qui** è la prova italiana che un profilo e un naso bastano a fare un personaggio memorabile
  **cosa se ne prende** lo stato espresso dal colore del fondo del riquadro (rosso, giallo, verde) invece che da una quarta bocca; il profilo come alternativa alla faccia frontale

### 4. Bruno Bozzetto · il Signor Rossi

- **Che cos'è.** È l'italiano medio di Bozzetto, nato nel 1960: Rossi è il cognome più diffuso, quindi «he could be anyone» (fonti/wiki-en-mr-rossi.md:14) [V, secondaria]. È anche un **caso d'allarme**. Bozzetto racconta: «in modo inconsapevole immaginai il signor Rossi con il nasone e i baffetti: era praticamente il ritratto del direttore del Festival» (fonti/sorrisi-bozzetto-2018.md:45) [V]. Lambiek ne fa il nome, Nino Zucchelli (fonti/lambiek-bozzetto.md:57) [V].
- **Leve d'identità** [INF dall'immagine]: proporzioni (piccolo, tondo), naso grande, baffetti, cappello, abito.
- **Perché piace.** La figura intera fa la battuta: sagoma bassa e rotonda, gesti ampi, pochi dettagli.
- **A 34 px** [INF]: regge per la sagoma tozza, non per il volto.
- **Capelli, occhiali, barbe.** Ne usa: baffetti e cappello, ed è proprio la combinazione di naso e baffi a fare il ritratto involontario. La lezione per noi è la regola 4 di [[grafica-italiana-giocosa]]:106: anche un disegno «da uomo comune» diventa qualcuno se porta una combinazione di tratti-firma [INF].
- **URL** https://www.lambiek.net/artists/b/bozzetto_bruno.htm · [V]
  **immagine** https://www.lambiek.net/artists/image/b/bozzetto_bruno/bozzetto_srrossi.jpg (anche https://www.artribune.com/wp-content/uploads/2024/02/Le-vacanze-del-signor-Rossi-1978-di-Bruno-Bozzetto.jpg)
  **perché conta qui** è una macchietta italiana, e insieme la prova che la somiglianza scappa anche a chi non la cerca
  **cosa se ne prende** la sagoma intera come leva (bassa, tonda, lunga); la conferma che la prova di riconoscimento di [[grafica-italiana-giocosa]]:107 serve davvero

**Nota senza immagine: Ellekappa.** Le sue vignette sono in bianco e nero, senza fondo, con due personaggi; i tratti sono «marcatamente pasciuti e curvilinei, con occhi sporgenti, nasi appuntiti e menti pronunciati» (fonti/wiki-it-ellekappa.md:26) [V, secondaria]. È un esempio di satira editoriale italiana che varia l'identità con naso, mento e occhi senza ricorrere a capelli o occhiali. Manca un URL d'immagine stabile [NON TROVATO], quindi Ellekappa non conta fra i tredici.

## Famiglia 2 · Personaggi geometrici e modernisti

### 5. Gerd Arntz · Isotype

- **Che cos'è.** Arntz disegnò circa 4000 pittogrammi per il sistema Isotype di Otto Neurath (fonti/gerdarntz-org-gerd-arntz.md:62) [V]. Il principio è la standardizzazione: un simbolo generico («uomo»), con «small variations or additions (such as a cap or standardized signs for various industries)», diventa specifico («operaio dell'acciaio») (fonti/gerdarntz-org-gerd-arntz.md:65) [V]. I simboli devono essere «instantly recognizable, without any distracting detail» (fonti/gerdarntz-org-gerd-arntz.md:74) [V]. Per ogni misura c'era una matrice di linoleum incisa a mano (fonti/gerdarntz-org-gerd-arntz.md:77) [V].
- **Leve d'identità.** Sagoma identica; varia il **segno sul petto** (ingranaggio, tazza, martelli, pane) e il **colore** della figura (fonti/gerdarntz-org-gerd-arntz.md:38-40, immagine «The generic symbol for 'man' acquires a specific content») [V]. Nell'immagine è rossa una figura sola su sei.
- **Perché piace.** Silhouette piena e compatta, nessun contorno, il segno ricavato in negativo dentro la sagoma. L'incisione nel linoleum dà ai bordi un leggero tremolio (fonti/gerdarntz-org-gerd-arntz.md:27) [V].
- **A 34 px.** L'immagine dell'archivio è alta 75 px [M: `sips`, 12/11/2026] e porta sei figure in 244 px di larghezza, circa 40 px per figura; il segno sul petto si legge [M: osservato]. A 34 px d'altezza [INF] il segno regge se è una forma sola, piena e chiara (una croce, un cerchio, una foglia), non un disegno con dettagli.
- **Capelli, occhiali, barbe.** Nessuno: la testa è un cerchio pieno.
- **URL** https://gerdarntz.org/content/gerd-arntz.html · [V]
  **immagine** https://gerdarntz.org/sites/gerdarntz.org/files/u2/Isotype-symbols.gif
  **perché conta qui** è proprio il nostro problema (molte figure uguali per ruolo, diverse per mestiere) risolto da un sistema che ha già i suoi pittogrammi per l'industria e la politica
  **cosa se ne prende** l'oggetto del dicastero va **dentro** la sagoma, sul petto, in negativo; sagoma piena senza contorno; un colore d'accento per la figura selezionata

### 6. Oskar Schlemmer · Triadisches Ballett

- **Che cos'è.** Il balletto del Bauhaus (prima nel 1922): 3 atti, 3 ballerini, 12 danze, 18 costumi, ogni atto con un suo colore di fondo (fonti/wiki-en-triadic-ballet.md:26) [V, secondaria]. Schlemmer pensava il corpo come geometria, «a cylinder for the neck, a circle for head and eyes», e chiamava i costumi «figurine» (fonti/wiki-en-triadic-ballet.md:28) [V, secondaria]. Considerava il movimento delle marionette superiore a quello umano (fonti/wiki-en-triadic-ballet.md:28) [V, secondaria].
- **Leve d'identità** [INF dall'immagine]: il busto come solido (campana, cono, disco, spirale), il collo come cilindro di lunghezza variabile, le spalle (sfere, gonnellini rigidi), la maschera del volto, il colore per figura.
- **Perché piace.** Ogni figura è una piccola architettura: forme pure, colori pieni, simmetria. Sono figure non umane che restano personaggi.
- **A 34 px** [INF]: regge molto bene, perché la sagoma del costume è l'identità.
- **Capelli, occhiali, barbe.** Nessuno: le teste sono maschere lisce, a volte con un elmo geometrico.
- **Diritti.** Schlemmer è morto il 13/4/1943 (fonti/wiki-en-oskar-schlemmer.md:12) [V, secondaria], quindi le figurine sono di pubblico dominio in Italia dal 1/1/2014 [INF: art. 25 L. 633/1941, vita più 70 anni, già usato in [[grafica-italiana-giocosa]]].
- **URL** https://en.wikipedia.org/wiki/Triadic_Ballet · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/commons/0/0c/Das_Triadische_Ballett%2C_Plakat_f%C3%BCr_eine_geplante_Auff%C3%BChrung_1924.jpg (costumi: https://upload.wikimedia.org/wikipedia/commons/6/68/Stuttgart_-_Neue_Staatsgalerie_%2835906597895%29.jpg)
  **perché conta qui** è la fonte modernista della «testa da forma elementare», ma applicata al corpo intero; ed è di pubblico dominio
  **cosa se ne prende** collo e spalle come leve d'identità; il busto come solido semplice; la figura come «figurina» (parola che il prototipo usa già)

### 7. Roger Hargreaves · Mr. Men

- **Che cos'è.** Dal 1971: ogni libro presenta un personaggio con un «single dominant personality trait», con illustrazioni «brightly coloured, boldly drawn» (fonti/wiki-en-mr-men.md:14) [V, secondaria], disegnate col pennarello Magic Marker (fonti/wiki-en-mr-men.md:48) [V, secondaria]. I personaggi sono 92 (fonti/wiki-en-mr-men.md:14) [V, secondaria].
- **Leve d'identità** [INF dall'immagine e dalla serie]: la silhouette dice il carattere (Mr. Tickle con le braccia lunghissime), un colore pieno per personaggio, un solo attributo (un cappello, un naso, una benda).
- **Perché piace.** Il tratto a pennarello, con il suo spessore irregolare, dà calore a forme elementari, e ogni sagoma è una battuta.
- **A 34 px** [INF]: regge, perché sono progettati per essere riconosciuti dalla sagoma e dal colore in copertina.
- **Capelli, occhiali, barbe.** Pochissimi nei Mr. Men; le Little Miss portano spesso fiocchi e acconciature. Anche un sistema così astratto ha ritratto una persona vera: nel 2006 nasce «Little Miss Stella», da Stella McCartney (fonti/wiki-en-mr-men.md:34) [V, secondaria]. Per noi la sagoma resta nuda, e il carattere va nella forma e non in un'acconciatura [INF].
- **URL** https://en.wikipedia.org/wiki/Mr._Men · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/en/4/4b/Mr._Tickle.jpg
  **perché conta qui** è la «macchietta» della regola 1 di [[grafica-italiana-giocosa]]:103 fatta sistema: un tratto di carattere, una sagoma, un colore
  **cosa se ne prende** la silhouette che traduce il tipo (il veterano tondo, il tecnico lungo e stretto, l'ambizioso a punta); il tratto a pennarello a spessore variabile

### 8. Dick Bruna · Miffy

- **Che cos'è.** Dal 1963 Miffy ha la forma stilizzata di oggi, «drawn in a graphic style, with minimalist black graphic lines»; Bruna usava solo nero, bianco, rosso, giallo, blu, verde, arancio, marrone e grigio (fonti/wiki-en-miffy.md:26) [V, secondaria]. Le fonti non concordano sulla lista esatta dei colori; la pagina ufficiale (https://www.miffy.com/special-colours) risponde 403 [NON TROVATO].
- **Leve d'identità** [INF]: gli amici di Miffy si distinguono per specie, cioè per la silhouette di testa e orecchie, e per il colore dell'abito. Il volto è sempre fatto degli stessi due punti e della stessa crocetta.
- **Perché piace.** Contorno nero spesso, di peso costante ma tracciato a mano; colori pieni scelti uno per uno e poi mai cambiati; posa rigorosamente frontale.
- **A 34 px** [INF]: è il riferimento che regge meglio, perché è progettato per bambini piccoli e per la stampa piccola.
- **Capelli, occhiali, barbe.** Nessuno. Le orecchie fanno il lavoro che altrove fanno i capelli.
- **URL** https://en.wikipedia.org/wiki/Miffy · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/en/3/30/Miffy%2C_by_Dick_Bruna.jpg
  **perché conta qui** mostra che, se cambiano silhouette e colore, un volto fisso e uguale per tutti non rende uguali i personaggi
  **cosa se ne prende** palette chiusa e fissa (i nostri sette inchiostri); un solo volto canonico; la sagoma della parte alta (orecchie, per noi copricapo o spalle) come leva d'identità

### 9. Annette Tison e Talus Taylor · Barbapapà

- **Che cos'è.** Creature a forma di goccia, con testa e braccia ma senza gambe, che cambiano forma a piacere; restano riconoscibili «by always retaining their faces and their distinctive colour» (fonti/wiki-en-barbapapa.md:24) [V, secondaria]. Ognuno dei sette figli ha un colore e un mestiere: lo sportivo rosso, lo scienziato blu, il naturalista giallo, la musicista verde (fonti/wiki-en-barbapapa.md:28) [V, secondaria].
- **Leve d'identità.** Colore (uno per personaggio), sagoma di base (i maschi più tondi in basso, le femmine più slanciate, fonti/wiki-en-barbapapa.md:24 [V, secondaria]), mestiere detto da un oggetto.
- **Perché piace.** Una forma morbida, un colore pieno, due occhi tondi: la famiglia sta tutta in una riga e si legge subito.
- **A 34 px** [INF]: regge, con la sagoma a goccia e il colore.
- **Capelli, occhiali, barbe.** Nessuno, tranne Barbabeau, che è «black and furry» (fonti/wiki-en-barbapapa.md:28) [V, secondaria]: la pelliccia è una texture, non un'acconciatura.
- **URL** https://en.wikipedia.org/wiki/Barbapapa · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/en/8/81/Barbapapa.jpg
  **perché conta qui** è il modello di «identità = colore + volto; stato = forma». Il prototipo fa già diventare triangolare la testa del ministro scontento ([[caricature-di-personaggi-inventati]]:15, che cita `10-direzione-artistica-2.prototype.html:947`)
  **cosa se ne prende** la forma può cambiare con la soddisfazione senza perdere l'identità, purché colore e volto restino fissi; un colore e un oggetto per mestiere

### 10. Charley Harper · «minimal realism»

- **Che cos'è.** Illustratore americano di natura: «I don't see the feathers in the wings, I just count the wings» (fonti/wiki-en-charley-harper.md:30) [V, secondaria]. Lui stesso chiamava il suo stile «minimal realism» (fonti/wiki-en-charley-harper.md:28) [V, secondaria].
- **Leve d'identità** [INF]: la specie è detta dalla geometria (triangoli, cerchi, semicerchi), l'occhio è un cerchio con un punto, la posa e i campi di colore sono pieni e tagliati netti.
- **Perché piace.** Non semplifica togliendo, sceglie: ogni forma porta un'informazione (l'ala, il becco), e i colori sono pochi ma accostati con cura.
- **A 34 px** [INF]: regge per i soggetti singoli; meno nelle composizioni a pattern.
- **Capelli, occhiali, barbe.** Non pertinente (animali).
- **URL** https://en.wikipedia.org/wiki/Charley_Harper · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/en/a/ae/Red-and-fed.jpg
  **perché conta qui** dà la regola per scegliere che cosa disegnare a 34 px: contare le ali, non le piume
  **cosa se ne prende** ogni faccia ha un solo dettaglio scelto (un naso, un colletto, un occhio a cerchio), il resto è massa; campi netti senza sfumature

## Famiglia 3 · Sistemi generativi e a parti

### 11. Nintendo · Mii

- **Che cos'è.** È l'editor di caricature del Wii, nato dal «Kokeshi concept» di Miyamoto, cioè le bambole di legno cilindriche (fonti/iwataasks-wii-channels-4.md:47) [V]. A proposito delle facce di tutto il mondo, Nogami spiega che chi giudica una somiglianza confronta il disegno con una «standard face» che ha in mente (fonti/iwataasks-wii-channels-4.md:115) [V]: è lo stesso meccanismo di Brennan già citato in [[caricature-di-personaggi-inventati]]:72. Il team ha tolto molte parti: «if there are hundreds of different options to choose from, you can be at a total loss» (fonti/iwataasks-wii-channels-4.md:133-137) [V]. Non cercavano il realismo, ma di cogliere «some of the character of that person» (fonti/iwataasks-wii-channels-4.md:167) [V].
- **Leve d'identità** [INF dall'immagine]: forma del volto, tipo, misura e **posizione** di occhi, naso e bocca, colore preferito sull'abito, altezza e corporatura (fonti/wiki-en-mii.md:34) [V, secondaria].
- **Perché piace.** La testa è sempre la stessa sfera morbida, il corpo varia per gioco (in Wii Sports niente braccia, mani sferiche fluttuanti, fonti/wiki-en-mii.md:80) [V, secondaria], e il colore dell'abito dice di chi è il personaggio.
- **A 34 px** [INF]: regge per il colore dell'abito e per la massa dei capelli; i lineamenti si perdono.
- **Capelli, occhiali, barbe.** Sono le leve principali, ed è proprio lì che il Mii diventa un sosia: allo staff di una filiale europea scappò un Mii troppo simile al capo, che si arrabbiò (fonti/iwataasks-wii-channels-4.md:149) [V]. Per noi è un **contro-esempio**: gli stessi cursori di posizione che aiutano a fare un sosia vanno tenuti stretti [INF].
- **URL** https://nintendo.com/en-gb/Iwata-Asks/Iwata-Asks-Wii/Iwata-Asks-Wii-Channels/4-Aiming-For-Worldwide-Mii-Communication/4-Aiming-For-Worldwide-Mii-Communication-213650.html · [V]
  **immagine** https://upload.wikimedia.org/wikipedia/en/0/07/Default_Miis.jpeg
  **perché conta qui** è un sistema a parti progettato per la somiglianza, cioè per l'opposto di ciò che serve a noi
  **cosa se ne prende** poche parti scelte bene, non centinaia; il colore dell'abito come firma; intervalli di posizione stretti, perché la posizione libera porta al ritratto

### 12. Herman Chernoff · le facce di Chernoff (1973)

- **Che cos'è.** Un metodo di visualizzazione in cui le parti del volto (occhi, orecchie, bocca, naso) codificano variabili con «shape, size, placement, and orientation» (fonti/wiki-en-chernoff-face.md:16) [V, secondaria]. Non tutte le parti pesano allo stesso modo: la misura degli occhi e l'inclinazione delle sopracciglia contano molto (fonti/wiki-en-chernoff-face.md:18) [V, secondaria]. Tufte nota che queste facce si rimpiccioliscono bene, leggibili fino a «0.05 square inches» (fonti/wiki-en-chernoff-face.md:22) [V, secondaria].
- **Leve d'identità.** Posizione, misura, forma e orientamento di ogni parte, ognuna legata a un numero.
- **Perché piace (e perché no).** È un ottimo catalogo di leve, ma senza un disegnatore le facce sono povere: contorni sottili, nessun colore, nessuna silhouette [INF]. È il nostro segnaposto in versione statistica.
- **A 34 px.** 0,05 pollici quadrati sono un quadrato di circa 21 px di lato a 96 dpi [M: √0,05 × 96, aritmetica]: secondo Tufte la faccia regge sotto i nostri 34 px. Il dato è di seconda mano, citato da Wikipedia [INF].
- **Capelli, occhiali, barbe.** Nessuno; le sopracciglia ci sono, e pesano molto.
- **URL** https://en.wikipedia.org/wiki/Chernoff_face · [V, secondaria]
  **immagine** https://upload.wikimedia.org/wikipedia/commons/1/17/Chernoff_faces_for_evaluations_of_US_judges.svg
  **perché conta qui** misura quali parti del volto si notano di più: misura degli occhi e inclinazione delle sopracciglia
  **cosa se ne prende** occhi di misura diversa e un sopracciglio inclinato come leve d'identità (o di stato) economiche; il limite: le leve senza disegno non bastano a fare il bello

### 13. DiceBear · stile Thumbs

- **Che cos'è.** Uno stile di avatar generato da un seme: «rounded thumb-shaped characters in a single solid color, with simple eyes and mouth on a tinted background», con licenza CC0 1.0 (fonti/dicebear-stile-thumbs.md:13, :21) [V].
- **Leve d'identità** [M: `curl` su `https://cdn.hopjs.net/npm/@dicebear/styles@10.6.0/dist/thumbs.min.json` e lettura delle chiavi `components` e `colors` con python3, 12/11/2026]: una sola forma di corpo e una di testa, 8 occhi, 5 bocche, 5 colori. La varietà viene soprattutto dalle trasformazioni: corpo e testa ruotano fra −20° e +20°, la testa si sposta di ±15 unità su 100, gli occhi di ±20 in orizzontale e ±10 in verticale. Il colore di occhi e bocca è nero o bianco, scelto per contrasto con la forma (`contrastTo: shape`).
- **Perché piace.** Con pochissime parti, l'inclinazione e lo spostamento danno a ogni figura una posa e un carattere: sembra che guardino da qualche parte.
- **A 34 px** [INF]: regge, con una sagoma unica a colore pieno e occhi grandi.
- **Capelli, occhiali, barbe.** Nessuno.
- **URL** https://www.dicebear.com/styles/thumbs/ · [V]
  **immagine** https://api.dicebear.com/10.x/thumbs/svg?seed=ministro-1
  **perché conta qui** mostra quanto rende la leva più economica di tutte (ruotare e spostare le parti) in un kit di parti a seme come il nostro
  **cosa se ne prende** inclinazione della testa e posizione degli occhi scelte dall'hash entro intervalli stretti; occhi color carta sugli abiti scuri e d'inchiostro sul giallo. Non serve la dipendenza: sono due attributi `transform` [INF]

## Tavola: le leve a confronto

| # | Riferimento | Silhouette | Proporzioni | Occhi (posizione, forma) | Naso | Collo e spalle | Colore | Abito o segno | Tratto | 34 px |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Zagnoli | massa dei capelli | — | spesso assenti | piccolo | colletto a motivo | campi pieni | colletto | senza contorno | sì [INF] |
| 2 | Guasco | profilo a piani | — | trattino | **forte** | colletto | un accento | berretto | piani, niente linea | profilo sì [INF] |
| 3 | Cavandoli | profilo | — | — | **forte** | — | lo stato sul fondo | — | linea unica | profilo sì [INF] |
| 4 | Bozzetto | figura bassa e tonda | **forte** | punti | forte | — | — | cappello | linea animata | sagoma sì [INF] |
| 5 | Arntz | identica | — | — | — | spalle uguali | un accento | **segno sul petto** | sagoma piena | sì [M parziale] |
| 6 | Schlemmer | **costume** | forte | maschera | — | **forte** | pieno | il costume | solidi | sì [INF] |
| 7 | Mr. Men | **forte** | forte | punti grandi | a volte | — | uno per figura | un attributo | pennarello | sì [INF] |
| 8 | Bruna | orecchie e testa | — | volto fisso | — | — | palette chiusa | abito | contorno spesso | sì [INF] |
| 9 | Barbapapà | goccia, cambia con lo stato | — | tondi | — | — | **uno per figura** | mestiere | morbido | sì [INF] |
| 10 | Harper | geometria della specie | — | cerchio e punto | — | — | pochi, netti | — | campi netti | singoli sì [INF] |
| 11 | Mii | volto | altezza e peso | **posizione** | sì | — | abito | colore preferito | morbido 3D | colore sì [INF] |
| 12 | Chernoff | volto | — | **misura, posizione** | sì | — | — | — | linea sottile | sì secondo Tufte |
| 13 | Thumbs | una | — | **posizione** | — | — | 5 | — | pieno | sì [INF] |

Elenco per la tavola da mostrare a Mario:

| # | Nome | Pagina | Immagine |
| --- | --- | --- | --- |
| 1 | Olimpia Zagnoli | https://www.olimpiazagnoli.com/about/ | https://oz.imbeard.com/wp-content/uploads/2025/10/OLIMPIADI26_DEFINITIVO-1463x2048.jpg |
| 2 | Riccardo Guasco, ciclisti | https://www.pushing-pixels.org/2013/11/23/illustrators-at-work-interview-with-riccardo-guasco.html | https://www.pushing-pixels.org/wp-content/uploads/2013/11/cyclists.jpg |
| 3 | Cavandoli, La Linea | https://it.wikipedia.org/wiki/La_Linea_(personaggio) | https://upload.wikimedia.org/wikipedia/it/7/79/La_Linea.png |
| 4 | Bozzetto, Signor Rossi | https://www.lambiek.net/artists/b/bozzetto_bruno.htm | https://www.lambiek.net/artists/image/b/bozzetto_bruno/bozzetto_srrossi.jpg |
| 5 | Gerd Arntz, Isotype | https://gerdarntz.org/content/gerd-arntz.html | https://gerdarntz.org/sites/gerdarntz.org/files/u2/Isotype-symbols.gif |
| 6 | Schlemmer, Triadisches Ballett | https://en.wikipedia.org/wiki/Triadic_Ballet | https://upload.wikimedia.org/wikipedia/commons/0/0c/Das_Triadische_Ballett%2C_Plakat_f%C3%BCr_eine_geplante_Auff%C3%BChrung_1924.jpg |
| 7 | Hargreaves, Mr. Men | https://en.wikipedia.org/wiki/Mr._Men | https://upload.wikimedia.org/wikipedia/en/4/4b/Mr._Tickle.jpg |
| 8 | Dick Bruna, Miffy | https://en.wikipedia.org/wiki/Miffy | https://upload.wikimedia.org/wikipedia/en/3/30/Miffy%2C_by_Dick_Bruna.jpg |
| 9 | Tison e Taylor, Barbapapà | https://en.wikipedia.org/wiki/Barbapapa | https://upload.wikimedia.org/wikipedia/en/8/81/Barbapapa.jpg |
| 10 | Charley Harper | https://en.wikipedia.org/wiki/Charley_Harper | https://upload.wikimedia.org/wikipedia/en/a/ae/Red-and-fed.jpg |
| 11 | Nintendo, Mii | https://nintendo.com/en-gb/Iwata-Asks/Iwata-Asks-Wii/Iwata-Asks-Wii-Channels/4-Aiming-For-Worldwide-Mii-Communication/4-Aiming-For-Worldwide-Mii-Communication-213650.html | https://upload.wikimedia.org/wikipedia/en/0/07/Default_Miis.jpeg |
| 12 | Facce di Chernoff | https://en.wikipedia.org/wiki/Chernoff_face | https://upload.wikimedia.org/wikipedia/commons/1/17/Chernoff_faces_for_evaluations_of_US_judges.svg |
| 13 | DiceBear Thumbs | https://www.dicebear.com/styles/thumbs/ | https://api.dicebear.com/10.x/thumbs/svg?seed=ministro-1 |

Tutti gli URL delle immagini rispondono 200 con un tipo immagine [M: `curl -I -L`, 12/11/2026]. Le immagini restano dei loro autori: servono per la tavola interna, non per entrare nel gioco [INF, coerente con [[ip-e-giochi-di-riferimento]]]. Solo Schlemmer è di pubblico dominio.

## Per il nostro gioco

Quattro direzioni, distinte fra loro. Ognuna resta dentro i sette inchiostri e fuori dalla lista nera. Ruolo e stato rimangono dove li mette già [[caricature-di-personaggi-inventati]]:51: il dicastero si legge dall'oggetto, la soddisfazione dalla bocca e dalla forma. Qui si decide solo **come si disegna l'identità**. Sono raccomandazioni da provare su un prototipo, non decisioni.

### A · Isotipo col segno sul petto

Ancorata ad **Arntz** (5), **Bruna** (8) e **Barbapapà** (9). Busto e testa sono una sola sagoma piena, senza contorno, come i pittogrammi Isotype; l'oggetto del dicastero si sposta dall'angolo in basso al **petto**, ricavato in carta dentro l'abito.
- **Leve d'identità:** colore dell'abito fra i cinque inchiostri scuri (inchiostro, grigio, rosso, verde, biro); larghezza delle spalle (stretta, media, larga); lunghezza del collo (nessuno, corto, lungo); forma della testa (le quattro di oggi). Fanno 5 × 3 × 3 × 4 = 180 combinazioni [M: aritmetica]. Il volto è carta, gli occhi sono d'inchiostro sul volto, quindi il contrasto è di 14,6:1.
- **Nella rosa** (stesso dicastero, stesso segno): decidono spalle, collo e testa.
- **Perché è bella:** è la più vicina a un giornale, sembra un timbro in pagina e regge benissimo a 34 px. Il rischio è che sia fredda; il calore viene dal bordo leggermente irregolare della sagoma, come nell'incisione su linoleum di Arntz [INF].
- Il giallo resta per il segno o per la figura selezionata, mai per l'abito: sulla carta ha un contrasto di 1,2:1.

### B · Macchiette di sagoma

Ancorata a **Mr. Men** (7), **Barbapapà** (9), **Thumbs** (13) e **Schlemmer** (6). Testa e corpo sono un'unica forma, e la silhouette traduce il tipo umano della regola 1 (il veterano tondo, il tecnico lungo e stretto, l'ambiziosa a punta, il notabile a pera). Occhi grandi, braccia corte facoltative, tratto a pennarello come forma piena di spessore variabile.
- **Leve d'identità:** silhouette (6 forme: goccia, pera, colonna, campana, uovo, cuneo); colore pieno fra rosso, verde, biro, giallo (con contorno d'inchiostro) e grigio; posizione e distanza degli occhi (3 varianti) e inclinazione della figura (−8°, 0°, +8°), scelte dall'hash come in Thumbs. Fanno 6 × 5 × 3 × 3 = 270 [M: aritmetica]. Occhi color carta sugli abiti scuri, d'inchiostro sul giallo.
- **Stato:** la sagoma si piega o si affloscia con la soddisfazione, mentre colore e occhi restano fissi (Barbapapà).
- **Perché è bella:** è la più giocosa e la più leggibile da lontano. Il rischio è che sembri un gioco per bambini; il registro del Settimanale si tiene con il tratto a pennarello e i colori d'inchiostro, non pastello [INF].

### C · Profilo al tratto

Ancorata a **Guasco** (2), **Cavandoli** (3) e **Bozzetto** (4), con Ellekappa come conferma testuale. Testa di profilo costruita a piani, con il naso come forma dominante e un colletto o un bavero come costume. Contorno come forma piena a spessore variabile, cioè il «pennello» chiesto in [[tono-giocoso-su-temi-seri]]:85.
- **Leve d'identità:** naso (5 forme geometriche: triangolo, uncino, goccia, patata, punta all'insù); mento (sfuggente, dritto, prominente); fronte (dritta, inclinata); colore del colletto fra cinque inchiostri. Fanno 5 × 3 × 2 × 5 = 150 [M: aritmetica]. L'occhio è un trattino o un punto, sempre alla stessa altezza.
- **Stato:** il colore del fondo del riquadro, come nella Linea, oppure la bocca.
- **Perché è bella:** è la più satirica e la più italiana, e ha il tono della vignetta da settimanale. **Il rischio di somiglianza è il più alto delle quattro:** profilo e naso sono proprio le differenze dalla norma che fanno riconoscere un volto (Brennan in [[caricature-di-personaggi-inventati]]:72; Nogami, fonti/iwataasks-wii-channels-4.md:115), e il Signor Rossi è nato ritratto senza volerlo. Si può fare solo con nasi esagerati oltre ogni naso reale, senza baffi, e con la prova di riconoscimento sul campione [INF].

### D · Figurino a solidi

Ancorata a **Schlemmer** (6), **Zagnoli** (1) e **Harper** (10). La figura è costruita con solidi piatti accostati, senza contorno: testa a cerchio, collo a cilindro, busto a campana o a cono, colletto a motivo. Un solo dettaglio forte per volto, secondo il «contare le ali» di Harper.
- **Leve d'identità:** busto (campana, cono, rettangolo, disco); collo (assente, corto, lungo); colletto a motivo (pieno, righe, scacchi, pois) in uno dei cinque inchiostri scuri; un dettaglio del volto (guance rosse, occhi a cerchio, bocca rossa). Fanno 4 × 3 × 4 × 5 × 3 = 720 [M: aritmetica].
- **Stato:** la bocca, oppure l'inclinazione del collo.
- **Perché è bella:** è la più elegante e la più «grafica italiana anni Sessanta» (Zagnoli che cita Munari). Il rischio è che, senza contorno, due campi vicini con poco contrasto si fondano: va verificata a 34 px con le coppie di inchiostri misurate sopra [INF].

### Valido per tutte: le leve di posizione

Da **Chernoff** (12), **Thumbs** (13) e **Mii** (11). Spostare gli occhi di pochi pixel e inclinare la testa di pochi gradi costa due attributi `transform` e dà più varietà percepita di una forma nuova [INF]. Due limiti. Gli intervalli vanno tenuti stretti, perché la libertà di posizione è ciò che trasforma un kit in un generatore di sosia (Mii, fonti/iwataasks-wii-channels-4.md:149). E pesano di più la misura degli occhi e l'inclinazione delle sopracciglia (fonti/wiki-en-chernoff-face.md:18): se si usano, le sopracciglia dicono lo stato, non l'identità, altrimenti un ministro sembra sempre arrabbiato [INF].

### Aperto

- Quale direzione provare per prima: decide Mario guardando la tavola. Un prototipo con le quattro direzioni affiancate, venti facce ciascuna a 34 e a 84 px, risponderebbe meglio di qualsiasi descrizione.
- Se il segno sul petto (direzione A) sostituisce l'oggetto in basso a destra anche a 84 px, dove c'è spazio per entrambi.
- Il tono grave: tutte e quattro le direzioni hanno una versione in solo inchiostro (sagoma piena o profilo), che può affiancare il ripiego delle iniziali di [[caricature-di-personaggi-inventati]].

## Collegamenti

- [[caricature-di-personaggi-inventati]]: produzione, numeri, collisioni e licenze del kit di parti; questa pagina decide come disegnare le parti.
- [[grafica-italiana-giocosa]]: il metodo in cinque regole, che le quattro direzioni rispettano; la regola 4 (lista nera) e la 5 (prova di riconoscimento) diventano decisive nella direzione C.
- [[tono-giocoso-su-temi-seri]]: la richiesta di caricature a pennello con linea di spessore variabile.
- [[ip-e-giochi-di-riferimento]]: perché si prende lo stile e non il personaggio.
- [[soggetti-reali-vs-fittizi]]: il vincolo legale dietro il caso del Signor Rossi.
