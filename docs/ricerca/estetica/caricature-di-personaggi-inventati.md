---
tipo: estetica
fonti: [fonti/dicebear-licenze.md, fonti/dicebear-js-library.md, fonti/dicebear-stile-figma.md, fonti/dicebear-schema-definizione.md, fonti/openpeeps-home.md, fonti/boring-avatars-readme.md, fonti/multiavatar-license.md, fonti/stonybrook-rhodes-brennan-carey-1987.md, fonti/wiki-it-matteo-ricci-politico.md, fonti/wiki-it-andrea-romano.md, fonti/inkscape-copying.md, fonti/gnu-gpl-faq.md]
aggiornato: 2026-11-01
---
# Caricature di personaggi inventati

Come produrre le caricature di ministri, tecnici e leader per «Il Settimanale» (storie 103, 154 e 159 della spec v1). Le opzioni sono tre: un kit di parti combinate da un seme, un set disegnato una volta, oppure nessuna caricatura con un ripiego tipografico. Il metodo per disegnare un ministro che non somigli a nessuno sta già in [[grafica-italiana-giocosa]] (righe 99-109) e qui non si ripete: questa pagina si occupa di produzione, numeri, peso, licenze e tono grave.

Tag: [V] fonte primaria letta · [M] misurato in sessione · [INF] inferenza · [NON TROVATO].

**Artefatti consultati** (repo `simulatore-politico`, ramo `main`, HEAD `c56787d`, 01/11/2026):
- spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md`: righe 65-66 (dimissioni e rosa di 3), 124 (tecnici), 152 (storia 103), 158 (al massimo dieci partiti), 224 (storia 154), 229 (storia 159), 352 (cambio del leader), 400 (tono grave), 484 (caricature come contenuto da scrivere);
- `CONTEXT.md:353-355` (tono grave);
- prototipo `.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html`: righe 522-534 (dieci ministri a macchietta), 590-601 (`ACC`, dieci accessori), 603-608 (`testa()`), 689 (vignetta nascosta nel tono grave), 712 (testa nel borsino), 790 (figurina), 947 (testa che diventa triangolo sotto 40);
- core: `crates/sim-core/src/ministri/mod.rs:215-233` (nome del ministro), `crates/sim-core/src/partita/ia.rs:45-55` (nome del leader), `dati/base/elettorato.toml:11-12` (12 nomi e 12 cognomi);
- Vista: `app/src/lib/tipi/VoceMinistro.ts` (`nome`, `partito`, `tecnico`, `soddisfazione`, `minaccia`), `app/src/lib/tipi/VocePartito.ts` (`id`, `leader`, vuoto per il partito del giocatore);
- `README.md:7` e `app/package.json:15`: codice MIT.

## Quante facce servono

- **Ministri:** 10 dicasteri (spec:277), tutti nel borsino a 40 px (prototipo:712) e nelle figurine a 62 px (prototipo:790) [V].
- **Rose:** a ogni sostituzione si sceglie fra 3 nomi del partito o 3 tecnici (spec:66, :330) [V]. Si vedono 3 facce dello **stesso dicastero** insieme, quindi con lo stesso accessorio [INF].
- **Tecnici:** possono entrare nella quota del partito del giocatore (spec:124), e nella Vista sono `tecnico: true` (VoceMinistro.ts) [V].
- **Leader:** uno per partito IA, mai per il partito del giocatore (VocePartito.ts, `leader` vuoto). I partiti sono al massimo dieci (spec:158), quindi i leader sono al massimo 9 [INF]. Il leader cambia dopo la perdita del governo o un calo al voto (spec:352), e i partiti nuovi o le scissioni portano un leader nuovo (`vita.rs:159`) [V].
- **A schermo insieme:** circa 10 ministri, 3 candidati e 7-9 leader, cioè una ventina di facce [INF].
- **In tutta la partita:** sono illimitate. Ogni nome viene estratto da 12 nomi per 12 cognomi, cioè **144 combinazioni** (`elettorato.toml:11-12`) [M: conteggio]. Il ministro non può avere il nome di un ministro in carica o in rosa (`ministri/mod.rs:215-233`) [V]; il leader nuovo deve solo essere diverso dal leader uscente (`ia.rs:52-54`) [V]. Due partiti possono quindi avere un leader con lo stesso nome, e un leader può chiamarsi come un ministro [INF].

**Conseguenza 1.** La Vista non porta un identificatore stabile del personaggio, solo il nome. Il seme della faccia, se non si tocca la Vista, si ricava dalla UI con un hash della stringa `ruolo:partito:nome`. È deterministico e sopravvive al salvataggio perché il nome fa parte dello stato [INF]. Senza `ruolo` e `partito`, due personaggi omonimi avrebbero la stessa faccia.

**Conseguenza 2.** Un set chiuso di ritratti, uno per personaggio, non copre una partita: i personaggi nascono durante il gioco. Qualunque opzione con un volto deve **combinare parti** [INF].

**Conseguenza 3: le collisioni.** Con un hash puro le facce si ripetono presto. Probabilità che almeno due facce su 23 coincidano [M: `1 - ∏(1 - i/N)`, python3, 01/11/2026]: 0,94 con N = 96 combinazioni, 0,36 con N = 576, 0,20 con N = 1152. Non conta però la collisione globale. Nel borsino l'accessorio del dicastero già distingue le dieci righe; fra i leader distingue il colore del partito. Le uniche collisioni da evitare sono **dentro un gruppo**, cioè i 3 candidati della stessa rosa: basta una regola deterministica che, a parità di parti, sposti il secondo e il terzo alla variante successiva [INF].

## Le tre opzioni

| | A · Kit di parti nostro, combinato col seme | A′ · Motore DiceBear | B · Set disegnato a mano, chiuso | C · Nessuna caricatura |
| --- | --- | --- | --- | --- |
| Facce distinte | identità = forma × occhi × tinta del volto × abito; per esempio 4 × 3 × 6 × 6 = 432 [INF]; bocca = stato, accessorio = ruolo | come A se lo stile è nostro; con uno stile esistente le combinazioni sono quelle dell'artista (Open Peeps dichiara «over 584,688») [V] | finite: non copre i personaggi generati in partita [INF] | nessuna; iniziali o sagoma uguale per tutti |
| Coerenza con le tinte piatte | piena: il prototipo è già scritto così (forma geometrica, contorno d'inchiostro, tinta piatta) [V prototipo:603-608] | con uno stile esistente bassa: Lorelei, Notionists e Open Peeps sono disegni a linea con capelli e lineamenti [INF dalla descrizione di Open Peeps, fonti/openpeeps-home.md:15, :37-51] | piena | piena, ma perde le storie 103 e 154 |
| Peso | testa inline 521-713 byte, kit sorgente 2.843 byte [M] | core: 33 file JS, 420 KB grezzi e 47 KB gzip in tutto [M]; uno stile da 37 KB (Lorelei) a 118 KB (Notionists) gzip [M] | 10-30 SVG da circa 1 KB l'uno [INF] | zero |
| Licenza | nostra, MIT come il resto del codice (README.md:7) [V] | codice MIT; ogni stile ha la sua licenza (vedi sotto) [V] | nostra | — |
| Rischio di somiglianza | minimo se il kit non ha capelli, occhiali o barbe [INF] | alto con gli stili esistenti, che hanno proprio capelli, occhiali e barbe [V fonti/openpeeps-home.md:37-51] | dipende dal disegno; la prova di riconoscimento si fa una volta | nullo |
| Tono grave | un ramo nella UI | idem | idem | non serve |

### A · Kit di parti nostro, combinato col seme

È il prototipo della direzione S, esteso. Oggi `testa()` combina 4 forme di testa (quadrato, cerchio, triangolo, mezzaluna), 2 occhi fissi, 3 bocche legate alla soddisfazione (normale, sotto 40, sotto 25), 10 accessori, una per dicastero, e due tinte (volto e abito) (prototipo:590-608) [V]. Le scelte sono scritte a mano nella tabella `AGENDA` (prototipo:523-534) [V]: per renderlo procedurale basta ricavare forma, occhi e tinte da un hash del nome invece che dalla tabella [INF].

- **Parti da disegnare una volta** [INF]: 4-6 forme di testa, 3 occhi (punti, ovali, semichiusi), 3 bocche (gli stati), 10 accessori di dicastero (ci sono già), un segno per il tecnico (per esempio un cartellino appuntato, non gli occhiali, che stanno nella lista nera), un segno per il leader (una coccarda nel colore del partito), 6 tinte del volto e 6 dell'abito. Sono circa 30 pezzi, contro i circa 110 disegni stimati per i ministri a pezzi animati in [[grafica-italiana-giocosa]]:115, che contava anche le pose.
- **Ruolo e stato non sono identità.** L'accessorio dice il dicastero (metodo Cipputi, [[grafica-italiana-giocosa]]:104) e la bocca e la forma che diventa triangolo dicono la soddisfazione (prototipo:947). L'hash sceglie solo le parti d'identità [INF].
- **Determinismo:** un hash della stringa (FNV-1a o simili, poche righe senza dipendenze) dà sempre la stessa faccia per lo stesso personaggio, su ogni macchina [INF].
- **Linea:** [[tono-giocoso-su-temi-seri]]:85 chiede «caricature a pennello con linea di spessore variabile»; il prototipo usa un contorno uniforme da 2,2 (prototipo:607). Il tratto a pennello si ottiene disegnando il contorno come forma piena invece che come `stroke` [INF]. Da decidere nel prototipo del borsino.

### A′ · Motore DiceBear (versione 10)

- **Che cos'è.** È una libreria che genera avatar SVG da uno stile e da un seme; lo stesso seme dà sempre lo stesso avatar (fonti/dicebear-js-library.md:56-64) [V], e ogni integrazione (JS, Rust, Python…) produce lo stesso avatar (fonti/dicebear-js-library.md:13) [V]. Si può limitare il generatore a certe varianti con una mappa di pesi: le varianti non elencate sono escluse (fonti/dicebear-js-library.md:251) [V]. Con `idRandomization` l'SVG smette di essere deterministico, perché il suffisso viene da `Math.random()` (fonti/dicebear-js-library.md:247) [V].
- **Uno stile nostro.** Dalla versione 10 uno stile è un file JSON dichiarativo, «no code, no functions» (fonti/dicebear-schema-definizione.md:15), con un elenco chiuso di elementi SVG ammessi (fonti/dicebear-schema-definizione.md:112) [V]. La via documentata per crearlo è il plugin per Figma (fonti/dicebear-stile-figma.md:11) [V]: Figma non è uno strumento locale, quindi viola il vincolo di produzione. Il JSON si può scrivere a mano, ma è lavoro in più rispetto alle poche righe di `testa()` [INF].
- **Versione.** Il pacchetto `@dicebear/core` è alla 10.7.0 e `@dicebear/styles` alla 10.6.0, entrambi modificati il 07/09/2026 [M: `npm view`, 01/11/2026]. Gli esempi scritti per la 9.x o prima, con import come `@dicebear/collection`, non valgono più [INF: la pagina JS attuale usa `@dicebear/styles/*.json`, fonti/dicebear-js-library.md].
- **Valutazione [INF].** Come motore aggiunge una dipendenza da 47 KB gzip per fare ciò che il prototipo fa in circa 15 righe. Con gli stili esistenti, invece, porta proprio i tratti che la lista nera esclude (capelli, occhiali, barbe) e un disegno a linea lontano dalle tinte piatte.

### B · Set disegnato a mano, chiuso

Un ritratto per personaggio va bene solo per un cast fisso, come i leader iniziali dei sette partiti, che la spec mette fra i contenuti da scrivere (spec:484) [V]. Tutti i personaggi nati in partita (rose, tecnici, leader dopo una sconfitta, partiti nuovi) restano scoperti, e il set ricade nell'opzione A [INF]. Il disegno a mano conviene per le **parti** del kit, non per le facce intere. Lo strumento locale e gratuito è Inkscape, sotto GPL 3 o successiva (fonti/inkscape-copying.md:19) [V]. La licenza del programma non copre ciò che ci si disegna: «the output of a program is not, in general, covered by the copyright on the code» (fonti/gnu-gpl-faq.md:338) [V]. Le parti restano MIT come il resto del codice [INF]. In alternativa le si scrive direttamente come path SVG, come nel prototipo (prototipo:590-601) [V].

### C · Nessuna caricatura: iniziali o sagoma

Non soddisfa le storie 103 («nome e caricatura») e 154 («borsino con caricatura») (spec:152, :224) [V]. Serve però come **ripiego nel tono grave** (vedi sotto) e come segnaposto nei prototipi finché il kit non è pronto [INF].

## Somiglianza a politici reali (storia 159)

- **Perché le teste geometriche sono sicure.** Nel generatore di Brennan un volto si descrive con 37 linee su 169 punti fissi, e la caricatura esagera tutte le differenze metriche fra quel volto e una norma; le caricature di volti noti si riconoscono più in fretta dei disegni fedeli, e la «best likeness» è la caricatura (fonti/stonybrook-rhodes-brennan-carey-1987.md:19) [V]. Riconoscere qualcuno richiede quindi le sue differenze dalla norma. Una testa fatta di una forma elementare, due punti e una linea non ha geometria del volto da cui misurarle, perciò può ricordare un politico solo attraverso segni esterni: capelli, occhiali, barba, abito, oggetto [INF]. È l'esempio di Altan in [[grafica-italiana-giocosa]]:106.
- **Regola di produzione che ne segue [INF]:** il kit non contiene **nessuna** acconciatura, occhiale o pelo, così la lista nera di [[grafica-italiana-giocosa]]:106 è garantita dalla costruzione e non deve essere ricontrollata a ogni volto generato. La prova di riconoscimento ([[grafica-italiana-giocosa]]:107) si fa una volta sul kit, mostrando a 3-5 persone un campione di facce, per esempio le prime 30 estratte da semi fissi, invece che personaggio per personaggio.
- **Scoperta fuori dal perimetro del ticket, ma pertinente alla 159: i nomi.** Il bacino di 12 nomi per 12 cognomi (`elettorato.toml:11-12`) genera nomi di politici italiani viventi. «Matteo Ricci» è europarlamentare del Partito Democratico dal 16 luglio 2024 ed è stato sindaco di Pesaro (fonti/wiki-it-matteo-ricci-politico.md:12-14) [V]; «Andrea Romano» è un politico ed ex deputato (fonti/wiki-it-andrea-romano.md:12) [V]. Le altre 142 combinazioni non sono state controllate [NON TROVATO]. Una faccia sicura non basta se il nome sotto la faccia è quello di un politico vero. Il rimedio, una lista di esclusione dei nomi o un bacino diverso, tocca i dati, non la UI.

## Tono grave

- Nel tono grave la spec fa sparire «testata, nome spiritoso, battute, motti e illustrazioni» (spec:400) e il glossario ripete «niente testata, battute, motti, illustrazioni» (CONTEXT.md:353-355) [V].
- **Il prototipo non è allineato.** La vignetta sparisce (prototipo:689, classe `solo-ord`) e anche il motto, ma la testa nel borsino resta, perché `testa(m,40)` non dipende dal registro (prototipo:712) [V]. Resta da decidere se la caricatura del borsino conti come «illustrazione» [INF].
- **Ripiego proposto [INF]:** nel tono grave la caricatura lascia il posto, nello stesso riquadro, alle iniziali in un cerchio d'inchiostro (opzione C). La riga non cambia misura, quindi non c'è salto di impaginazione, e il personaggio resta identificabile dal nome. In alternativa la stessa testa resta in solo inchiostro, senza tinta e senza accessorio. Decide Mario.
- **Accessibilità:** la caricatura è decorativa accanto al nome. DiceBear stesso emette `aria-hidden="true"` di default per questo caso (fonti/dicebear-js-library.md:267) [V], e il prototipo fa lo stesso (prototipo:607) [V].

## Licenze degli avatar esistenti

| Fonte | Codice | Disegni | Uso in un gioco gratuito open source |
| --- | --- | --- | --- |
| DiceBear | MIT (fonti/dicebear-licenze.md:9) [V] | ogni artista sceglie: 42 stili CC0, 14 CC BY 4.0, 1 MIT, 4 «Free for personal and commercial use» (fonti/dicebear-licenze.md:9, :65, :109-116) [V] | CC0: libero. CC BY 4.0: crediti, link alla licenza, indicazione delle modifiche (fonti/dicebear-licenze.md:72) [V]. Avataaars e Bottts hanno termini dell'autore, non una licenza standard: da evitare in un repo open source [INF]. Diritti di immagine e di pubblicità restano fuori da CC0 e CC BY (fonti/dicebear-licenze.md:17, :76) [V] |
| Open Peeps | — | CC0 (fonti/openpeeps-home.md:24) [V] | libero, ma ha capelli, occhiali e barbe (fonti/openpeeps-home.md:37-51) [V] |
| Boring Avatars | MIT (fonti/boring-avatars-readme.md:100) [V] | varianti astratte; `beam` ha un volto minimo (fonti/boring-avatars-readme.md:52) [V] | libero, ma non è una caricatura [INF] |
| Multiavatar | licenza propria v1.0 (fonti/multiavatar-license.md:17) [V] | vieta di ri-confezionare i disegni o farne un prodotto concorrente (fonti/multiavatar-license.md:41-44) [V] | da evitare: non è una licenza standard e ha restrizioni sui disegni [INF] |

## Per il nostro gioco

- **Prendiamo:** il kit del prototipo (forme, bocche di stato, accessori di dicastero) esteso a circa 30 parti, combinate da un hash di `ruolo:partito:nome`. Il ruolo si legge dall'oggetto, lo stato dalla bocca, l'identità dalle parti scelte dall'hash.
- **Evitiamo:** stili di avatar esistenti con capelli, occhiali o barbe; licenze non standard (Avataaars, Multiavatar); Figma come strumento di produzione; un ritratto per personaggio.
- **Raccomandazione (da validare, non decisione):** opzione A senza dipendenze, che è il prototipo con l'hash al posto della tabella. Le iniziali dell'opzione C diventano il ripiego del tono grave. Nessun capello, occhiale o barba nel kit; prova di riconoscimento fatta una volta sul campione. DiceBear non serve.
- **Aperto:** se la caricatura del borsino sparisce nel tono grave (prototipo:712 contro spec:400); il tratto a pennello o il contorno uniforme; i nomi del bacino che coincidono con politici veri (tocca i dati, non questa pagina); la regola anti-collisione dentro la rosa.

## Riferimenti

- **URL** https://www.dicebear.com/licenses/ · [V]
  **perché conta qui** il codice di DiceBear è MIT, ma ogni stile ha una licenza diversa
  **cosa se ne prende** la ripartizione 42 CC0, 14 CC BY 4.0, 1 MIT e 4 con termini dell'autore; gli obblighi del CC BY; diritti di immagine e pubblicità esclusi
- **URL** https://www.dicebear.com/how-to-use/js-library/ · [V]
  **perché conta qui** come DiceBear rende deterministico un avatar
  **cosa se ne prende** stesso seme, stessa faccia; esclusione delle varianti con la mappa di pesi; `idRandomization` rompe il determinismo; `aria-hidden` di default
- **URL** https://www.dicebear.com/guides/create-an-avatar-style-with-figma/ · [V]
  **perché conta qui** la via documentata per uno stile proprio
  **cosa se ne prende** passa da Figma, quindi viola il vincolo «solo strumenti locali»; versione 10.x
- **URL** https://www.dicebear.com/create-styles/definition-schema/ · [V]
  **perché conta qui** lo stile come JSON dichiarativo, scrivibile a mano
  **cosa se ne prende** nessun codice nello stile, elementi SVG ammessi in elenco chiuso: l'opzione A′ è possibile senza Figma, ma costa più di `testa()`
- **URL** https://www.openpeeps.com/ · [V]
  **perché conta qui** il set di parti combinabili più noto, in CC0
  **cosa se ne prende** l'ordine di grandezza delle combinazioni (584.688) e la prova che i set esistenti si basano su capelli, occhiali e barbe
- **URL** https://github.com/boringdesigners/boring-avatars · [V]
  **perché conta qui** avatar generati da un nome, licenza MIT, peso piccolo
  **cosa se ne prende** il confronto: varianti astratte, non caricature
- **URL** https://github.com/multiavatar/Multiavatar/blob/main/LICENSE · [V]
  **perché conta qui** generatore di avatar con licenza propria
  **cosa se ne prende** il divieto di ri-confezionare i disegni: da escludere
- **URL** https://researchconnect.stonybrook.edu/en/publications/identification-and-ratings-of-caricatures-implications-for-mental/ · [V, abstract]
  **perché conta qui** spiega su che cosa si regge il riconoscimento di un volto in caricatura
  **cosa se ne prende** la somiglianza nasce dalle differenze metriche rispetto a una norma, quindi le teste geometriche senza lineamenti la escludono per costruzione
- **URL** https://it.wikipedia.org/wiki/Matteo_Ricci_(politico) · [V, secondaria]
  **perché conta qui** un nome che il bacino del gioco può generare
  **cosa se ne prende** prova che la storia 159 è a rischio sui nomi, non sui volti
- **URL** https://it.wikipedia.org/wiki/Andrea_Romano · [V, secondaria]
  **perché conta qui** secondo nome del bacino che coincide con un politico
  **cosa se ne prende** conferma che la collisione non è un caso isolato
- **URL** https://gitlab.com/inkscape/inkscape/-/blob/master/COPYING · [V]
  **perché conta qui** l'editor vettoriale locale e gratuito per disegnare le parti
  **cosa se ne prende** i binari sono sotto GPL 3 o successiva
- **URL** https://www.gnu.org/licenses/gpl-faq.html#GPLOutput · [V]
  **perché conta qui** chiarisce se la GPL di Inkscape si estende ai disegni
  **cosa se ne prende** l'output di un programma non è coperto dalla licenza del programma: le parti restano MIT

## Collegamenti

- [[grafica-italiana-giocosa]]: metodo in cinque regole per il ministro inventato, che qui diventa una regola di produzione.
- [[tono-giocoso-su-temi-seri]]: due registri con soglia, caricatura da 32 px nel borsino, linea a pennello.
- [[soggetti-reali-vs-fittizi]]: articoli 96-97 della legge sul diritto d'autore, nessun volto né nome reale.
- [[decisioni]]: soglie di soddisfazione 40 e 25, che sono le bocche del kit.
