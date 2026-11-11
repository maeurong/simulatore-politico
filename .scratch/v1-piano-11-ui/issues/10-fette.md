Type: grilling
Status: resolved
Blocked by: 01, 07
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come si divide il piano 11 in fette? Confini di ogni fetta (schermate e comandi del guscio che porta), ordine, prima fetta su ciò che la Vista ha già su main, criterio per cui una fetta diventa pronta quando un piano del core fonde i suoi dati, rimandi da scrivere nella spec per ciascuna.

## Answer

Deciso con Mario l'11/11/2026 in due giri. Parte da [Inventario](01-inventario-vista-per-schermata.md) (aggiornamento del 10/11, `docs/ricerca/sintesi/vista-per-schermata.md:116-175`), da [Ossatura e navigazione](07-ossatura-e-navigazione.md) e dalla spec del piano di core «esito e Vista completa» (`docs/superpowers/specs/2026-11-11-esito-e-vista-completa.md:16-21`, sul branch `docs/piano-esito-vista-completa`, PR #50). Su main la UI è un segnaposto di 54 righe (`app/src/routes/+page.svelte`): ogni fetta costruisce da zero.

### Regole

- **Taglio per percorso di gioco.** Ogni fetta lascia l'app giocabile un pezzo più in là, e si prova giocando. Il percorso parte dal giocatore vero (edicola, fondazione, turno zero), non dalla partita di prova di `nuova_partita` (`app/src-tauri/src/lib.rs:224`): il turno di governo arriva alla terza fetta.
- **Una fetta è un piano** di `superpowers:writing-plans`, chiamato 11a, 11b e così via, con 2-4 PR al suo interno.
- **Schermate a dati parziali subito**, con ciò che la Vista porta. Il posto di ciò che manca resta vuoto, e la fetta che lo riempie è scritta nella spec (colonna «posti lasciati» qui sotto). La UI lascia un posto vuoto ma non lo riempie deducendo.
- **Prima pagina e Registro, prima dell'esito:** «Chiudi il turno» apre il cruscotto del turno nuovo, e le due sezioni sono un segnaposto interno. Nessuna prima pagina provvisoria composta dai frammenti (`mosse`, `esiti`, variazioni): la scelta di cosa conta starebbe nella UI.
- **Pronta quando**, tutte e quattro: (1) i dati che usa sono su main; (2) i prototipi delle sue schermate sono chiusi; (3) le Viste campione sono rigenerate su quel main; (4) la fetta precedente è fusa, perché le fette scrivono negli stessi file (cornice, sommario, instradamento).
- **Unica eccezione all'ordine: 11f passa avanti** appena la PR 1 del core è su main, perché le basta 11b e scrive solo nelle sue sezioni. 11g no: riempie schermate di 11b-11e.
- **Passacarte per fetta:** ciascuna porta i comandi del guscio che chiamano le sue schermate. La forma del contratto (`bigint`, errori, CSP) resta a [Contratto del guscio](12-contratto-del-guscio.md); la prima fetta la applica a `comanda`.

### Le fette

| Fetta | Schermate | Passacarte nuovi | Dati | Posti lasciati, con rimando |
|---|---|---|---|---|
| **11a Avvio** | fondamenta (cornice, sommario con sezioni barrate, token, `t()`, scala e movimento, quattro componenti di base); [Edicola](13-edicola.md); [Scenario ed editor](14-scenario-ed-editor.md); [Fondazione](15-fondazione.md); ingresso nel turno zero con la cornice e «Chiudi il turno» | `serie` (tracciati della cornice); `anteprima_fondazione` se il Contratto del guscio la tiene | main | Prima pagina e Registro come segnaposto → 11f |
| **11b Al voto** | Cruscotto (evento in testa, tono grave, scelte del turno zero); [Campagna](26-campagna.md); [Paese](23-paese.md) (due livelli, sondaggio, selettore); [Partiti](24-partiti.md); [Trattativa e spartizione](27-trattativa-e-spartizione.md); kit delle caricature ([Disegno delle caricature](32-disegno-delle-caricature.md)). Tre PR: cruscotto e campagna; Paese e Partiti; trattativa con le caricature | `anteprima_campagna`, `elettori_campione` | main | livelli 3-5 e scheda completa della regione, minaccia di scissione → 11g |
| **11c Al governo** | [Cruscotto al governo](17-cruscotto-al-governo.md), anche in crisi; [Governo](18-governo.md); [Aula e calendario](19-aula-e-calendario.md); [Anteprima della misura](20-anteprima-della-misura.md); motti del Pungolo | nessuno (`comanda` e le anteprime ci sono già) | main | costo di Ferma e Dirotta, posti in calendario, combinazioni spente della legge elettorale → 11g |
| **11d I conti** | [Conti](21-conti.md); [Bilancio](22-bilancio.md) | `bozza_bilancio`, `indicizza`, `riepilogo_bilancio`, `riepilogo_presentato` | main | PDE, conto di controllo, MES, clausola dell'energia, redditi per fascia → 11g; piano strutturale riveduto → rimando agganciato (sotto) |
| **11e Opposizione e fine** | [Cruscotto all'opposizione](28-cruscotto-all-opposizione.md); [Eredità](30-eredita.md); [Schema dello Stato](29-schema-dello-stato.md) | `anteprima_sfiducia` | main | «Avanti fino a novità», «torna a un turno» dall'eredità → 11f |
| **11f L'esito** | [Prima pagina](16-prima-pagina.md); [Registro](25-registro.md) con «torna al turno N»; «Avanti fino a novità»; i due avvisi del piano 9; titoli del Pungolo ([Testi del Pungolo](09-testi-del-pungolo.md)) | `registro` (lo porta il piano di core) | PR 1 del core | nessuno |
| **11g Vista completa** | i pezzi mancanti di Paese, Conti, Partiti, Aula e Governo | `menu_elettorale`, `costo_dirotta` ([Contratto del guscio](12-contratto-del-guscio.md)) | PR 2 del core | nessuno |

Paese e Partiti stanno in 11b perché senza sondaggio e senza le casse dei rivali la campagna si gioca alla cieca. Conti e Bilancio fanno fetta a sé per non portare 11c a sei schermate.

**Piano strutturale riveduto** (storia 51): nessuna fetta sua. Il riquadro nel Bilancio lo riempie la prima fetta che si pianifica dopo che il piano di core strutturale è su main; 11g, se a quel punto c'è già.

### Scartato

Taglio per sezione del sommario; una fetta per PR; tutti i passacarte nella prima fetta; schermate parziali tenute ferme fino alla PR 2 del core; prima pagina provvisoria dai frammenti; 11a sulla partita di prova; ordine rigido anche per l'esito; fetta 11h per il piano strutturale.
