---
tipo: estetica
fonti: [fonti/ofl-faq.md, fonti/inter-rsms.md, fonti/ibm-plex-github.md, fonti/atkinson-hyperlegible-braille.md, fonti/noto-fonts-github.md, fonti/noto-cjk-github.md, fonti/cliffsblog-2021-04.md, fonti/msdfgen-github.md, fonti/steamworks-deck-compat.md]
aggiornato: 2026-10-16
---
# Tipografia e font aperti

In un gioco fatto di pannelli il carattere tipografico è la grafica più vista. Serve una famiglia leggibile a corpo piccolo, con cifre tabellari per le colonne di numeri, copertura multilingua e licenza SIL OFL, che permette di includere il font nel gioco anche venduto. Il costo nascosto sono le lingue CJK: un solo font cinese pesa decine di megabyte.

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Licenza OFL in un gioco

- Il bundling con software è esplicitamente permesso, sia della versione originale sia di una modificata; tra gli esempi elencati ci sono "giochi e software di intrattenimento" [V] (fonti/ofl-faq.md:47).
- Un font OFL si può vendere solo se è in bundle con software [V] (fonti/ofl-faq.md:57).
- In un'app si devono includere almeno copyright, avviso di licenza e testo della licenza; basta una menzione nella schermata "About" o nei crediti con il link al pacchetto [V] (fonti/ofl-faq.md:103).
- Un font incorporato in un documento può essere anche un sottoinsieme dei glifi [V] (fonti/ofl-faq.md:75).
- Se si modifica un font con "Reserved Font Name", la versione modificata deve cambiare nome [V] (fonti/ofl-faq.md:16, :99). Conta se si fa il subsetting o si converte in atlante: verificare l'RFN di ogni famiglia [INF].
- Le immagini create con il font (titoli, loghi) non ereditano la licenza [V] (fonti/ofl-faq.md:35).

## Candidate

| Famiglia | Perché | Lingue | Licenza |
| --- | --- | --- | --- |
| Inter | pensata per interfacce dense: x-height alta nelle taglie "text", ink trap; pesi 100-900; cifre tabellari `tnum`, zero barrato | oltre 2000 glifi, 147 lingue | OFL 1.1 [V] (fonti/inter-rsms.md:17-23, :64, :317-321) |
| IBM Plex | Sans, Serif, Mono, Condensed con corsivi veri: un sistema intero; progettata per ambienti UI | Sans: latino esteso, arabo, cinese tradizionale, cirillico, devanagari, greco, ebraico, giapponese, coreano, thai | OFL [V] (fonti/ibm-plex-github.md:19) |
| Atkinson Hyperlegible | lettere e numeri molto distinti, progettata per ipovedenti; gratuita anche per uso commerciale | latino | OFL 1.1 per la versione "Next" [M] (`gh api repos/googlefonts/atkinson-hyperlegible-next`) (fonti/atkinson-hyperlegible-braille.md:17-27) |
| Source Sans | "sans serif per ambienti di interfaccia utente" (descrizione del repository Adobe) | da verificare [NON TROVATO] | OFL 1.1 [M] (`gh api repos/adobe-fonts/source-sans`) |
| Noto | copertura di quasi tutte le scritture; un repository per scrittura | tutte | Noto CJK e Noto latin-greek-cyrillic OFL 1.1 [M] (`gh api repos/notofonts/noto-cjk`, `repos/notofonts/latin-greek-cyrillic`); il sito di distribuzione è Apache-2.0 [M]; altre scritture da verificare per repository (fonti/noto-fonts-github.md:17) |

Le cifre tabellari sono il requisito più sottovalutato: con cifre proporzionali una colonna di percentuali "balla" a ogni aggiornamento del turno [INF da fonti/inter-rsms.md:321].

## Il costo delle lingue CJK

- Dimensioni dei pacchetti Noto CJK, release correnti [M] (`gh api repos/notofonts/noto-cjk/releases`, 16/10/2026): Noto Sans SC 47,7 MB zip, Noto Sans JP 25,8 MB, Noto Sans KR 24,8 MB; il super-OTC di tutte le CJK 90,7 MB zip.
- Democracy 4 ha affrontato il problema delle lingue "con così tanti caratteri da non stare nella memoria delle texture" con pango e cairo [V] (fonti/cliffsblog-2021-04.md:198).
- Conseguenze [INF]: i font CJK si scaricano o installano solo con la lingua scelta; si usa un solo peso; si fa il subsetting ai caratteri effettivamente presenti nei testi del gioco; gli atlanti si generano al volo, non pre-cotti per tutti i glifi.

## Rendering

- Per testo nitido a qualunque zoom (etichette della mappa) si usano atlanti MSDF; `msdf-atlas-gen` è MIT [M] e genera atlanti di interi font [V] (fonti/msdfgen-github.md:20). Dipende dal motore: vedi [[godot]] e [[tauri]].

## Vincolo di leggibilità

- Steam Deck: carattere minimo di 9 px di altezza a 1280x800 ([[prestazioni-su-hardware-modesto]], fonti/steamworks-deck-compat.md) [V, cattura di altro agente]. Con Godot la scala HiDPI di sistema non viene letta su Windows: serve un'opzione di scala dell'interfaccia nel gioco ([[godot]]). Il font scelto va provato a 9-10 px reali, dove x-height alta e cifre distinte (Inter, Atkinson) pesano più dell'estetica [INF].
- Il text server di ripiego di Godot copre solo latino, greco e cirillico ([[godot]]): con lingue CJK o arabe serve quello avanzato [V, pagina di altro agente].

## Gerarchia consigliata [INF]

- Una famiglia sans per tutta l'interfaccia (Inter o IBM Plex Sans), cifre tabellari attive nei pannelli numerici.
- Un serif solo per titoli di giornale, discorsi e nomi di leggi (IBM Plex Serif tiene il sistema coerente), se la direzione è "editoriale".
- Un mono solo per dati grezzi o la vista "sala operativa", se serve.
- Noto come fallback per le scritture non coperte.
Tre famiglie sono il massimo; due è meglio.

## Per il nostro gioco

- **Prendiamo:** OFL come requisito; cifre tabellari; una famiglia principale con più stili (Plex è la più completa come sistema; Inter la più rifinita per UI densa).
- **Evitiamo:** font display "di carattere" nei pannelli; più di tre famiglie; pre-includere tutti i CJK nel pacchetto base.
- **Aperto:** quali lingue al lancio (decide il peso del pacchetto); verifica dell'RFN prima di fare subsetting.

## Riferimenti

- **URL** https://openfontlicense.org/ofl-faq/ · [V]
  **perché conta qui** regole d'uso della licenza dei font candidati
  **cosa se ne prende** bundling nei giochi permesso, obblighi di attribuzione, RFN sulle modifiche
- **URL** https://rsms.me/inter/ · [V]
  **perché conta qui** scheda ufficiale di Inter
  **cosa se ne prende** copertura, optical size, `tnum`, licenza
- **URL** https://github.com/IBM/plex · [V]
  **perché conta qui** repository ufficiale di IBM Plex
  **cosa se ne prende** sistema completo sans/serif/mono e copertura di scritture
- **URL** https://www.brailleinstitute.org/freefont/ · [V]
  **perché conta qui** font progettato per la leggibilità estrema
  **cosa se ne prende** candidato per numeri e testo piccolo, o per un'opzione di accessibilità
- **URL** https://github.com/notofonts/notofonts.github.io · [V]
  **perché conta qui** organizzazione del progetto Noto
  **cosa se ne prende** un repository per scrittura: si prende solo ciò che serve
- **URL** https://github.com/notofonts/noto-cjk · [V]
  **perché conta qui** distribuzione dei font CJK
  **cosa se ne prende** nomi dei pacchetti per lingua; pesi misurati via API
- **URL** https://positech.co.uk/cliffsblog/2021/04 (cattura di altro agente) · [V]
  **perché conta qui** esperienza diretta di un simulatore politico localizzato
  **cosa se ne prende** il CJK è un problema di memoria texture, risolto con pango/cairo
- **URL** https://github.com/Chlumsky/msdfgen · [V]
  **perché conta qui** generazione di atlanti per testo nitido
  **cosa se ne prende** MSDF per le etichette della mappa

## Collegamenti

- [[interfacce-di-giochi-gestionali]]: il testo come materiale primario dei pannelli.
- [[shader-ed-effetti-economici]]: MSDF per testo e icone.
- [[mappe-e-globo]]: etichette multilingua da Natural Earth.
- [[data-viz-cinematica]]: cifre tabellari nei grafici.
- [[prestazioni-su-hardware-modesto]], [[godot]]: corpo minimo e scala UI (altro agente).
