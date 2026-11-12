Type: grilling
Status: resolved
Blocked by: -
Assignee: maeurong
Label: wayfinder:grilling
Map: ../map.md

## Question

Dove vivono le impostazioni del giocatore (scala con CSS `zoom`, velocità come fattore `--k`, interruttore del movimento sopra `prefers-reduced-motion`) e cosa sopravvive alla partita? Un file del guscio Tauri o la memoria del webview; per giocatore o per partita; cosa succede alla scala quando la finestra si stringe sotto il tetto dei 1024 px utili.

Il posto nella navigazione c'è già: una voce accanto a Schema dello Stato ed Edicola, e nell'edicola ([Ossatura e navigazione](07-ossatura-e-navigazione.md)). Scala e movimento li hanno decisi [Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md) e [Fondamenta visive](08-fondamenta-visive.md); entrano con la fetta 11a ([Fette](10-fette.md)), quindi la risposta serve prima che quel piano si scriva. Il guscio resta passacarte senza logica né stato di partita ([Contratto del guscio](12-contratto-del-guscio.md)).

## Answer

Deciso con Mario il 12/11/2026 in due giri e su un prototipo navigabile: [33-impostazioni.prototype.html](../prototipi/33-impostazioni.prototype.html), che si apre sulla forma decisa. Ricerca: [localstorage-in-wkwebview](../../../docs/ricerca/tecnica/localstorage-in-wkwebview.md).

### Dove vivono e cosa sopravvive

- **Per l'app, non per la partita.** Un solo insieme, uguale in edicola e in ogni partita. Sopravvive a tutto: a una partita nuova, all'abbandono, a Cmd+Q.
- **In `localStorage` del webview**, una chiave sola con un numero di versione dentro. Si legge in modo sincrono, quindi la scala è giusta già al primo disegno; un valore illeggibile o di versione ignota vale come assente, e si riparte dai valori di partenza. Il guscio non cambia: nessun comando nuovo, resta passacarte ([Contratto del guscio](12-contratto-del-guscio.md)).
- Regge fra un avvio e l'altro: wry usa il deposito persistente di Apple, e ITP cancella lo storage scritto da script solo dopo 30 giorni d'uso dell'app senza un clic sulla pagina (7 per la documentazione pubblica), cosa che in un gioco a click non succede. Sviluppo (`http://localhost:1420`, cartella `~/Library/WebKit/app/`) e rilascio (`tauri://localhost`, cartella dell'`identifier`) hanno due depositi separati: le impostazioni provate in sviluppo non passano alla build, e cambiare `identifier` le azzera.
- Scartato: un file JSON nell'`app_data_dir` scritto dal guscio. Sarebbe lo stesso in sviluppo e in rilascio, ma costa due comandi e arriva dopo il primo disegno, che sarebbe a 1× e poi salterebbe. Ci si torna se le impostazioni diventano dati che costa perdere.

### Cosa contengono

- **Scala**: gradini 1 · 1,1 · 1,25 · 1,4 · 1,6 · 1,8 · 2, partenza 1×. Cmd+ e Cmd− salgono e scendono di un gradino, Cmd0 torna a 1×. Fuori dalle Impostazioni la scorciatoia mostra per un secondo e mezzo, in alto a destra, «Scala 1,25× · Cmd0 torna a 1×».
- **Il tetto della finestra non tocca la scelta.** Si salva la scala scelta e si applica il minimo fra la scelta e la larghezza della finestra divisa per 1024 ([Fondamenta visive](08-fondamenta-visive.md)). Allargando la finestra torna la scelta. I gradini che la finestra non regge sono tratteggiati ma si possono scegliere; sotto la scala, in rosso: «Applicata 1,25×: la finestra è stretta. Allargala e torna 1,4×.» Anche l'avviso della scorciatoia dice il tetto quando c'è.
- **Velocità**: 1× · 2× · Istantaneo, cioè `k` a 1, 0,5 e 0 ([Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md)). Partenza 1×.
- **Movimento a tre posizioni**: Come il sistema, Acceso, Spento. Partenza «Come il sistema», che segue «Riduci movimento» di macOS anche se cambia a partita aperta e lo dice accanto («macOS non riduce il movimento: oggi è acceso»). Spento lascia le dissolvenze. Corregge la ricerca, che copiava la preferenza di sistema solo al primo avvio (`docs/ricerca/tecnica/movimento-e-scala-in-wkwebview.md:85`): così non c'è uno stato nascosto.
- **Torna ai valori di partenza**: un link in fondo, senza conferma (1×, velocità 1×, come il sistema).

### Forma: B · Lato

Le Impostazioni si aprono nel **dettaglio a lato**, sopra la sezione aperta: la pagina sotto resta in vista e cambia scala mentre si sceglie. Nel lato, in quest'ordine: Scala, Velocità, Movimento con le loro note, il ritorno ai valori di partenza, poi una **prova** (una carta che gira, dieci caselle che si riempiono, tre righe d'esito che entrano) per sentire velocità e movimento. Si chiude con «Chiudi» o con Esc. In edicola il lato prende il posto del dettaglio della partita. Scartate: A · Pagina (pagina intera come lo Schema dello Stato) e C · Striscia (tre comandi in fila sotto la cornice).

Nel prototipo, a 1024 px utili, i sette gradini vanno su due righe nel lato.

### Per la fetta 11a

Entrano con la fetta 11a ([Fette](10-fette.md)): la chiave in `localStorage`, la voce Impostazioni nella cornice e nell'edicola, le scorciatoie, `zoom` su `<html>` e `--k` aggiornati a ogni cambio e a ogni ridimensionamento della finestra. Nessun rimando al core, nessun passacarte.
