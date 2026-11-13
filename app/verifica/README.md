# Verifica della UI

Ogni fetta della UI si chiude con una batteria (`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`, «Cosa controlla una fetta prima di dirsi finita»). Qui stanno gli strumenti che la eseguono: contano i tagli e controllano la forma stretta a 1280×800 e 1440×900, a scala 1 e alla massima; provano il movimento spento e la tastiera a 1440×900, a scala 1. Sono nati durante la fetta 11a, non hanno dipendenze e non entrano né nel build né nel controllo dei tipi: vivono fuori da `src/`. I loro test (`*.test.mjs`) girano con `npm test`.

I comandi si lanciano da `app/`.

| Comando | Dove gira | Cosa misura |
|---|---|---|
| `npm run verifica:chrome` | Chrome headless | tagli, forma stretta, errori di console, movimento, tastiera, screenshot |
| `npm run verifica:webkit` | WebKit di sistema, in una WKWebView fuori schermo | tagli, forma stretta, fotografia |
| `npm run verifica:giro` | l'app vera, col guscio vero | quello che gli chiedi, un passo alla volta |

L'app gira su WebKit, quindi Chrome da solo non basta: nella 11a la batteria in WebKit ha trovato tre difetti che Chrome non mostrava. Uno per tutti: sotto `zoom`, WebKit confronta una `@container` in px con la larghezza ingrandita.

## Le due batterie

Serve il server di sviluppo acceso in un altro terminale (`npm run dev`). Le pagine si passano come `nome=percorso`; il nome finisce nei nomi dei file e ha solo lettere, cifre, `-` e `_`, e il percorso comincia con `/`: altrimenti la batteria stampa l'uso ed esce con 2.

```bash
npm run verifica:chrome -- 'edicola=/?vista=13-edicola.json' scenario=/scenario
```

```bash
npm run verifica:webkit -- 'edicola=/?vista=13-edicola.json' scenario=/scenario
```

Le Viste campione stanno in `.scratch/v1-piano-11-ui/viste/` e si scelgono con `?vista=<file>`; `&stato=<chiave>` sceglie uno stato dentro il file. Si passano tutti gli stati che toccano le schermate della fetta, più il tono grave (`08-tono-grave.json`). `?tagli=1` lo aggiunge la batteria.

Ogni pagina passa per quattro misure: 1280×800 a scala 1 e 1,25, 1440×900 a scala 1 e 1,4. Una misura vale se la pagina è rimasta quella chiesta, la scala è applicata e i tagli sono zero. Se una non vale, la riga dice perché (`← TAGLI`, `← SCALA NON APPLICATA`, `← ALTROVE`: una Vista che non esiste rimanda all'edicola) e la batteria esce con 1. Esce con 2, senza misurare, se il server non risponde, se è quello di un altro checkout o se una pagina dà errore. In Chrome anche due controlli a 1440×900 portano a 1: `← MOVIMENTO`, se col movimento spento (o con «Come il sistema» sotto il movimento ridotto) parte l'animazione di una proprietà diversa da `opacity`, e `← TASTIERA`, se il Tab non raggiunge un elemento visibile. Porta a 1 anche `← REGISTRO MUTO`, se il registro delle animazioni non risponde: lo script iniettato non ha girato, e uno zero letto così non prova niente. Il resto si legge sulle righe: `err` e `lato=true` non cambiano il codice d'uscita.

**In Chrome** una riga per misura, poi il movimento e la tastiera a 1440×900:

```
edicola 1280x800 x1.25: tagli=0 classi=[stretta] zoom=1.25 lato=false err=0
edicola movimento spento: animazioni=4 tipi=[opacity] err=0
edicola movimento sistema+reduce: animazioni=4 tipi=[opacity] err=0
edicola tastiera 19/19 err=0
```

- `classi` sono quelle della radice: `stretta` a 1280 con 1,25× e a 1440 con 1,4×, `grave` sulle Viste del tono grave.
- `lato=true` vuol dire che la pagina scorre di lato.
- `animazioni` conta ogni animazione partita dopo il caricamento e dopo aver toccato ciascuno dei primi 40 pulsanti, link e campi della pagina: pulsanti e campi li clicca, ai link dà solo il fuoco, per non lasciare la pagina. `tipi` sono le proprietà animate. Le registra uno script iniettato a ogni documento: ascolta `transitionrun` e `animationstart` e avvolge `Element.prototype.animate`. La misura si ripete prima col movimento spento, poi con «Come il sistema» sotto il movimento ridotto (`sistema+reduce`). Le dissolvenze di opacità restano ammesse (spec, storia 55); qualunque altra proprietà dà `← MOVIMENTO`. Per sapere quale elemento fa partire l'animazione: `node verifica/anim-chi.mjs /scenario spento`; campiona a istanti, quindi un'animazione breve che il registro ha contato può non comparirgli. Per il controllo al contrario, cioè che col movimento acceso un'animazione parta davvero e lo zero non sia a vuoto: `node verifica/anim-accese.mjs '/?vista=13-edicola.json' Impostazioni 'Prova ›'`. Legge lo stesso registro della batteria. Clicca in fila i pulsanti che contengono quei testi e misura dopo l'ultimo, col movimento acceso e poi spento; esce con 1 se acceso non parte nessuna animazione. Il conto può restare uguale nei due modi, perché spento una rotazione diventa una dissolvenza: la differenza si legge nei `tipi`, `transform` da una parte e solo `opacity` dall'altra. È la regola della spec (storia 55 e punto 4): col movimento spento restano solo le dissolvenze di opacità, e una transizione di colore o di ombra conta come movimento.
- `tastiera 19/19` conta gli elementi visibili raggiunti dal Tab; quelli mancati vengono elencati e danno `← TASTIERA`. Li conta soltanto: non li attiva e non legge l'albero di accessibilità. La spec (punto 4) chiede anche che ogni azione si attivi da tastiera, conferme sul posto comprese: questo si prova a mano nel browser e si scrive nella PR della fetta.

Pulsanti e campi la batteria li clicca davvero. Nel browser risponde il guscio finto, quindi non tocca nessun salvataggio.

**In WebKit** una riga per misura: `t` i tagli, `s` le classi della radice, `z` la scala applicata, `dove` il percorso d'arrivo, `lato` lo scorrimento di lato, `chi` i primi elementi tagliati.

```
scenario 1280x800 x1.25 {"t":0,"s":"stretta","z":1.25,"dove":"/scenario","lato":false,"chi":[]}
```

Al primo uso compila `wk.swift` con `swiftc` (servono i Command Line Tools di Xcode). Movimento e tastiera qui non si misurano. Per una misura sola, con un'espressione tua, si lancia `verifica/uscita/wk` a mano: l'uso sta in testa a `wk.swift`.

Screenshot, fotografie ed `esiti.json` finiscono in `verifica/uscita/chrome` e `verifica/uscita/webkit`, che git ignora. Quelli delle schermate della fetta si committano in `docs/superpowers/verifiche/<fetta>/` e il corpo della PR li cita (spec, punto 6): così ha fatto la 11a.

| Variabile | A cosa serve | Se manca |
|---|---|---|
| `VERIFICA_BASE` | indirizzo del server di sviluppo | `http://127.0.0.1:1420` |
| `VERIFICA_USCITA` | radice degli esiti; ogni strumento ci mette la sua sottocartella | `verifica/uscita` |
| `CHROME` | eseguibile di Chrome | quello in `/Applications` |

## Il giro nell'app vera

`computer-use` non vede l'app di sviluppo: `tauri dev` lancia un binario senza bundle. Il giro la pilota dall'interno. Un plugin di Vite, caricato solo da questo comando, aggancia alla pagina un telecomando (`giro/giro.js`) che esegue i passi messi in coda e risponde con l'esito. Nessun file dell'app cambia.

Prima si spegne `npm run dev`: il giro avvia il suo Vite sulla stessa porta 1420. Poi, in un terminale, parte l'app:

```bash
npm run verifica:giro
```

Il primo avvio compila il guscio da zero: su questo Mac, una dozzina di minuti. A finestra aperta, da un altro terminale si mandano i passi, uno o più per volta:

```bash
npm run verifica:comando -- '{"passo":"stato"}' '{"passo":"clicca","testo":"Impostazioni"}'
```

Il primo passo è sempre `stato`: annota `ls`, il valore delle impostazioni salvate, che a fine giro va rimesso. `NESSUNA RISPOSTA` vuol dire che il telecomando non è agganciato: il passo resta in coda e parte quando l'app arriva. Con un passo senza risposta, rifiutato, in errore o che non trova l'elemento (`NON TROVATO`), `verifica:comando` esce con 1. Se è il telecomando a rifiutare il comando (403 o 405), lo dice subito, senza aspettare.

Un passo è un oggetto JSON: `passo`, poi chi cercare (`sel`, un selettore, oppure `testo`, il testo che l'elemento contiene; `n` sceglie fra più trovati) e `attesa`, i millisecondi da aspettare dopo il passo (500 se manca).

| Passo | Cosa fa |
|---|---|
| `stato` | URL, finestra, scala, classi, movimento, caratteri caricati, tagli, impostazioni salvate (`ls`), avvisi di console e violazioni della CSP |
| `testo`, `elenco`, `rett`, `stile` | leggono il testo, una lista di elementi, il riquadro, lo stile calcolato |
| `clicca`, `scrivi`, `fuoco` | clic, valore in un campo o in un menu (`valore`), fuoco |
| `tasto` | un `keydown` sintetico (`key`, `code`, `meta`): arriva alle scorciatoie della pagina, non al sistema |
| `vai`, `ricarica` | navigano; `vai` accetta solo percorsi dell'app (`url` che comincia con `/`) |
| `rimetti` | riscrive le impostazioni salvate (`ls`); senza `ls` le cancella |

Per contare i tagli nell'app vera si naviga con `{"passo":"vai","url":"/scenario?tagli=1"}` e si legge `tagli` in `stato`. Ogni esito finisce anche in `verifica/uscita/giro/diario.jsonl`.

A fine giro, nell'ordine:

1. Si rimette `ls` com'era all'inizio: `{"passo":"rimetti","ls":{"versione":1,"scala":1,"velocita":"1","movimento":"sistema"}}`, coi valori letti da `stato`. Se all'inizio era `null`, `{"passo":"rimetti"}`. La pagina tiene in memoria il valore di prima finché non si ricarica: `{"passo":"ricarica"}` e un ultimo `stato` confermano che `ls` è rimasto quello rimesso.
2. Si chiude l'app e si cancella a mano la cartella di verifica: `rm -r ~/Library/Application\ Support/it.maeurong.simulatorepolitico.verifica`.
3. Si ripristina `src-tauri/Cargo.toml`, di cui `tauri dev` riscrive due righe: `git checkout -- src-tauri/Cargo.toml`. Non si committano.

## Fatti d'ambiente

1. **`CARGO_TARGET_DIR`.** Se `~/.cargo/config.toml` sposta la cartella di compilazione con un `target-dir` globale, `tauri dev` va in panico all'avvio («Failed to setup app: unknown path»): Tauri cerca le risorse sotto `target/debug` del repo. Su questo Mac quella riga è stata tolta il 13/11/2026. `verifica:giro` imposta comunque la variabile da sé; dove un `target-dir` globale c'è, chi lancia `tauri dev` a mano passa `CARGO_TARGET_DIR=<repo>/target`.
2. **L'identificativo di verifica.** Il giro parte con `it.maeurong.simulatorepolitico.verifica` (`giro/tauri.verifica.json`), quindi salva in una cartella sua sotto `~/Library/Application Support/`. I salvataggi di sviluppo di Mario, in `it.maeurong.simulatorepolitico/`, non si toccano.
3. **Il `localStorage` delle impostazioni è condiviso.** Scala, velocità e movimento stanno in `~/Library/WebKit/app`, che non dipende dall'identificativo: il giro e l'app di sviluppo di Mario leggono lo stesso valore. Per questo `ls` si annota all'inizio e si rimette alla fine.
4. **Le batterie non si lanciano mentre `cargo` compila.** Con una compilazione in corso, per esempio il primo `verifica:giro` o un `cargo test`, la macchina può essere così carica che Chrome non apre il canale di debug o `wk` non risponde in tempo. Gli strumenti escono con 2 e nominano il carico come sospetto: si aspetta la fine della compilazione e si rilancia.
5. **Con una cartella di compilazione unica due worktree si scambiano i binari dei test.** Fino al 13/11/2026 `~/.cargo/config.toml` mandava tutti i checkout nella stessa cartella (`target-dir`): un `cargo test` lanciato da un worktree ha eseguito i test di un altro (1468 test invece di 1444, con nomi che nel sorgente non c'erano). La riga è stata tolta e ogni checkout compila nella propria `target/`. Se un `target-dir` globale torna, chi lancia `cargo test` da un worktree passa `CARGO_TARGET_DIR=<worktree>/target`; un conto dei test che cambia senza modifiche a Rust è questo scambio.

## Cosa non coprono

Queste prove chiedono mani e occhi, e vanno scritte nella PR per Mario:

- **L'attivazione da tastiera.** La batteria conta gli elementi che il Tab raggiunge. Non preme Invio o Spazio su nessuno, non prova le conferme sul posto e non legge l'albero di accessibilità: attivazione e conferme si provano a mano nel browser.
- **Il Tab reale.** La tastiera è provata solo in Chrome. Nell'app vera il telecomando manda eventi sintetici, che non spostano il fuoco.
- **Il trascinamento col puntatore.** `scrivi` assegna il valore a un cursore e lancia `input` e `change`: il gesto non c'è.
- **Cmd+Q.** `tasto` non arriva al menu del sistema: l'uscita dall'app e ciò che il guscio fa in quel momento restano da provare.
- **La CSP nel rilascio.** In `tauri dev` la pagina arriva dal server di Vite, con uno script in linea che la CSP di `tauri.conf.json` vieterebbe: lì non è applicata. `stato` riporta le violazioni che vede, ma solo un build le prova davvero.
- **La resa a occhio nell'app vera.** Un agente non può fotografare la finestra: `computer-use` non la vede. La guarda Mario. Le fotografie della batteria in WebKit vengono dallo stesso motore, non dalla finestra dell'app.
