Type: prototype
Status: resolved
Blocked by: 07, 04
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Quali sono le fondamenta visive di tutte le schermate, ricavate da `.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html`? Token di colore e inchiostri, Fraunces e Atkinson Next con cifre tabellari, griglia e densità a 1440×900, testata «Il Pungolo», componenti di base (numero con freccia, fascia, voce con causa, pulsante d'azione), cornice del tono grave, scala della UI. La caricatura conta come illustrazione nel tono grave? Il prototipo la tiene nel borsino (riga 712), la spec fa sparire le illustrazioni (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:400`); la ricerca propone le iniziali in un cerchio come ripiego ([Caricature](05-caricature.md)). Varianti da confrontare su una cornice vuota dell'ossatura.

### Premesse dall'11/11/2026

[Ossatura e navigazione](07-ossatura-e-navigazione.md) ha fissato la cornice su cui confrontare le varianti: testata piccola, stato del giocatore, data e turni al voto, tre indicatori a caselle, «Chiudi il turno» (con «Avanti fino a novità» sopra, all'opposizione); sotto, il sommario di dieci sezioni con le voci barrate e, a destra, Schema dello Stato, Impostazioni ed Edicola; sulla prima pagina la stessa cornice fa da striscia sopra la testata grande; in crisi una banda nera sotto. Il tono grave vale per tutto il turno, cornice compresa. La cornice non ha un prototipo suo: si decide qui. Forma di partenza in `.scratch/v1-piano-11-ui/prototipi/07-ossatura.prototype.html`, dove a 1280×800 la cornice entra a fatica: larghezze da ricontrollare.

## Answer

Deciso con Mario l'11/11/2026 su un prototipo navigabile: [08-fondamenta.prototype.html](../prototipi/08-fondamenta.prototype.html). Si apre sulla forma decisa; le alternative restano dagli interruttori della barra e dai parametri scritti in testa al file. I token sono le variabili CSS in cima al foglio di stile del prototipo: sono loro la fonte, questa pagina li riassume. Dati delle [Viste campione](06-viste-campione.md) 01, 03, 05, 06 e 08.

### Scelte di Mario

- **Cornice: A · Gerenza.** Due righe, 104 px a scala 1×. Riga 1: testatina «Il Pungolo» (27 px), stato del giocatore su tre righe (stato, periodo e turno, turni al voto), i tre tracciati al centro separati da filetti, a destra «Chiudi il turno ›» pieno con sotto «N scelte al default ›»; all'opposizione «Avanti fino a novità ››» sta sopra. Riga 2: il sommario; la sezione corrente è una linguetta che buca il filetto doppio, la barrata dice il motivo al passaggio del puntatore e al fuoco della tastiera; a destra Schema dello Stato, Impostazioni, Edicola. Sulla prima pagina è la stessa cornice senza testatina. In crisi la banda nera sta sotto.
- **Tracciato: si riempie.** Dieci caselle piene fino al valore, **a sinistra la caduta** per tutti e tre (lo spread è rovesciato). Uno stacco dopo la zona di pericolo segna la soglia anche senza colore; dentro la zona le caselle piene diventano rosse; il retino della zona si vede solo dove la casella è larga almeno 12 px. Senza valore (maggioranza a Camere sciolte) le caselle sono tratteggiate. Accanto il numero a 19 px, sotto la variazione.
- **Caricatura nel tono grave: le iniziali** in un cerchio d'inchiostro, nello stesso riquadro. La caricatura conta come illustrazione e sparisce, come dice la spec (riga 400); il prototipo vincolante è superato su questo punto.
- **A 1440×900 più contenuto**, non caratteri più grandi: colonne fluide, dettaglio a lato più largo. Chi vuole più grande usa la scala.

### Token

| Inchiostro | Valore | Ruolo | Contrasto sulla carta |
|---|---|---|---|
| `--carta` | `#F4ECD8` | il foglio, unico fondo | — |
| `--ink` | `#1F1A17` | testo, filetti forti, pieni | 14,6 |
| `--grigio` | `#5E5750` | testo secondario, filetti sottili | 6,0 |
| `--rosso` | `#B3122B` | testata, ciò che va male, il critico | 5,9 |
| `--verde` | `#2E6B45` | ciò che va bene | 5,4 |
| `--biro` | `#1D4E89` | note a biro, motti, rimandi allo schema, anello del fuoco | 7,1 |
| `--giallo` | `#FFD23F` | evidenziatore: solo fondo sotto l'inchiostro, mai testo né filetto | 1,2 |

Più `--velo` (inchiostro al 6%: riga scelta, passaggio del puntatore) e `--retino` (tratteggio a 45°: zona di pericolo, vignette). Filetti: 1 px grigio fra le righe, 1,5 px per riquadri e caselle, 2 px per i pulsanti, doppio da 3 px per testa e piede di pagina, tratteggio per ciò che è spento. Nessun angolo arrotondato, nessuna ombra, nessuna sfumatura. Spazio a passi di 4 px.

### Caratteri

Fraunces (`SOFT 100, WONK 1`, peso 700, 800 per la testata) per testata e titoli; Atkinson Hyperlegible Next per tutto il resto, con `tabular-nums lining-nums` sulla radice. Scala, in px a 1×: 12 minimo (etichette, occhiello in maiuscolo spaziato, cause), 13 secondario, 14 corpo, 16 attacco, 19 numero dei tracciati, 18 titolo di riquadro, 23 titolo di sezione, 30 titolo di pagina, 44 titolo d'apertura, 72 testata. Niente sotto i 12.

### Griglia, densità, scala

- Margine di pagina 24 px, canale 20 px, dodici colonne fluide. Il dettaglio a lato è largo `clamp(300px, 30%, 440px)`.
- Riga di tabella densa: 43 px, due righe di testo. I dieci ministri col titolo occupano 470 px dei 696 che restano sotto la cornice a 1280×800.
- Il corpo di una sezione scorre in verticale quando non basta; la cornice non scorre mai e niente scorre di lato.
- Scala con CSS `zoom` ([Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md)), **mai sotto 1024 px di larghezza utile**: a 1280 si ferma a 1,25×, a 1440 a 1,4×. Sotto 1150 px utili la cornice passa alla forma stretta: via la testatina, «Schema» al posto di «Schema dello Stato», voci del sommario a 13 px.

### Componenti di base

- **Numero con freccia**: la freccia (un triangolo disegnato, non un carattere) dice il verso, il colore il giudizio: lo spread che sale è rosso. Sotto soglia la variazione va in negativo (carta su rosso), così il critico si legge anche senza colore e nel tono grave.
- **Fascia**: un valore dato come una fra poche classi con un nome (capacità buona, rischio alto, intesa media), con le stesse caselle del tracciato in piccolo.
- **Voce con causa**: il fatto in neretto col suo numero, sotto «perché:» e la causa in grigio a 12 px. La forma lunga, per la prima pagina, è occhiello col fatto, titolo e causa.
- **Pulsante d'azione**: pieno per l'azione che chiude (uno per schermata), a bordo per le altre, piccolo nelle righe, sottolineato per i rimandi. Scelto: fondo giallo. Spento: tratteggiato e **sempre col motivo scritto accanto**. Ciò che distrugge: bordo rosso, e al clic diventa due pulsanti sul posto.
- **Riquadro** a 1,5 px per il dettaglio a lato; **segnaposto** tratteggiato solo nei prototipi.

### Tono grave

Una classe sola sulla radice: rosso, verde e biro diventano inchiostro, il giallo sparisce, Fraunces perde `SOFT` e `WONK` e scende a peso 600, spariscono testatina, corsivo, motti, vignetta e bollo, le caricature diventano iniziali. Il «scelto» diventa pieno d'inchiostro. Nessuna banda in più: quella nera è della crisi.

### Verificato, e non

Verificato nel pannello Browser: i contrasti qui sopra; le cifre di Atkinson Next sono tabellari («1111» e «8888» larghe uguali); nessun taglio per la forma A in 150 combinazioni di pagina, stato, finestra, scala e tracciato (il contatore «tagli» della barra del prototipo). Non verificato: tutto questo dentro WKWebView, come chiede il ticket 04. Da portare nella spec: i due caratteri vanno dentro l'app, il prototipo li prende da Google Fonts.

### Scartato

B · Colonna (tutto a sinistra in 214 px: il corpo perde larghezza e a 1,25× la colonna scorre); C · Testa e piede (sommario in testa, indicatori e chiusura in fondo); la striscia su una riga sola, scartata prima di mostrarla perché chiede circa 1.300 px e a 1280 ce ne sono 1.232; il tracciato a pedina; la caricatura in grigio o tolta; la pagina ingrandita a 1440; la scala 1,5× a 1280.

### Aperto

- Scale e soglie dei tre tracciati: quelle del prototipo sono proposte, non dati del core. Ticket [Scale dei tracciati](31-scale-dei-tracciati.md).
- Cosa mostrano i tre indicatori quando il giocatore non governa: aggiunto alle premesse di [Cruscotto all'opposizione](28-cruscotto-all-opposizione.md).

Termini nuovi nel glossario (`CONTEXT.md`, «Interfaccia»): tracciato, fascia.
