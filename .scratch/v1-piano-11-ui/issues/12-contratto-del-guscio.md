Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Com'è fatto il contratto fra UI e guscio Tauri? Elenco dei comandi passacarte (`comanda` più quelli dell'[inventario](01-inventario-vista-per-schermata.md)); i 17 tipi generati come `bigint` che `JSON.stringify` non serializza (convenzione in ingresso e in uscita); `Fondazione` e `Fascia` non esportati in TypeScript; il seme condiviso fra `anteprima_fondazione` e `fonda` (stato nel guscio, `u64` da JS o campo nel core); come la UI riceve gli errori del core. Decisione tecnica: si risolve con Mario solo dove tocca il core.

### Premesse al 10/11/2026

Il piano 10 ha cambiato il guscio ([Inventario](01-inventario-vista-per-schermata.md), aggiornamento del 10/11): `comanda` e `fonda_partita` esistono, `Fondazione` è esportato, i tipi con `bigint` sono 19, il seme lo sceglie il guscio dall'orologio e il riepilogo dello scenario è senza seme. Restano da decidere i passacarte mancanti (anteprime di sfiducia e campagna, elettori campione, bilancio, `serie`, forse `anteprima_fondazione`), `Fascia`, la convenzione sui `bigint` e gli errori. Due voci in più: `nuova_partita` (`app/src-tauri/src/lib.rs:224`) apre ancora la partita di prova senza fondazione e senza salvataggi, accanto a `fonda_partita`, e va deciso se la UI la tiene; la politica di sicurezza dei contenuti è spenta (`"csp": null`, `app/src-tauri/tauri.conf.json:21`), rimandata dalla #43 alla UI del piano 11.

Dall'11/11/2026 ([Fette](10-fette.md)): nessuna fetta usa la partita di prova, perché il percorso parte da edicola e fondazione. Ogni fetta porta i passacarte delle sue schermate; questo ticket ne fissa l'elenco e la forma, e la fetta 11a la applica a `comanda`.

## Answer

Deciso con Mario l'11/11/2026 in un giro, sul guscio di main a `894274d` (`app/src-tauri/src/lib.rs`) e sulla spec del piano di core «esito e Vista completa» (branch `docs/piano-esito-vista-completa`, PR #50). Il core non cambia, salvo una riga d'esportazione su `Fascia`.

### Forma del contratto

- **`bigint` → `number`**: `TS_RS_LARGE_INT = "number"` in `.cargo/config.toml`, accanto a `TS_RS_EXPORT_DIR`; ts-rs 12 lo prevede (`ts-rs-12.0.1/src/lib.rs:80`). I 19 tipi si rigenerano, il codice del core no. Gli importi sono euro interi (`crates/sim-core/src/vista.rs:305`) e i conteggi elettori: tutti molto sotto 2⁵³, in ingresso e in uscita.
- **`Fascia`**: la esporta il Task 17 della PR 2 del core (spec, `:369`). Se 11b arriva prima, mette lei la riga `#[cfg_attr(test, derive(TS), ts(export))]` su `Fascia` (`crates/sim-core/src/elettorato/mod.rs:74`): solo esportazione, niente a runtime. Chi arriva dopo la trova fatta.
- **Errori in due specie**, forma `{ specie: "rifiuto" | "guasto", testo }`, distinte dal guscio. **Rifiuto**: ogni `sim_core::Errore` da `comanda`, `fonda_partita`, anteprime, `indicizza`, `riepilogo_bilancio`, `elettori_campione`; si mostra sul posto, accanto alla scelta. **Guasto**: il resto (disco dell'archivio, lock avvelenato, «nessuna partita in corso», salvataggio illeggibile); sta nella cornice e dice che la partita è rimasta com'era, come il guscio garantisce. Tipo TS scritto nel modulo delle chiamate: il guscio non usa ts-rs.
- **Seme della fondazione nel guscio**: la storia 9 vuole l'anteprima sullo stesso sondaggio del gioco. `anteprima_fondazione` sceglie il seme dall'orologio e lo tiene (un `Option<u64>` accanto alla partita in corso); `fonda_partita` lo riusa, e senza anteprima prima lo prende dall'orologio come oggi (`lib.rs:264`). Vale fino alla fondazione o all'anteprima successiva; la UI passa alle due chiamate lo stesso scenario. `riepilogo_scenario` resta all'editor: è atteso, senza seme, e non porta istogramma né disponibilità.
- **Bozze della UI solo in memoria**: bilancio in composizione, valori della legge, spese della campagna. Cmd+Q le perde; ciò che è già comandato è salvato dal guscio.
- **Chiamate da un modulo tipizzato** sopra `invoke`, una funzione per comando, coi tipi generati.
- **`nuova_partita` esce** dal `generate_handler!` (`lib.rs:231`, `:412`) in 11a: nessuna fetta la usa.
- **CSP accesa in 11a**: `default-src 'self'`, `style-src 'self' 'unsafe-inline'` per le transizioni di Svelte; gli script del guscio li firma Tauri.

### Passacarte per fetta

| Fetta | Passacarte |
|---|---|
| 11a | `serie`, `anteprima_fondazione` (col seme sopra); forma del contratto applicata a `comanda` e ai comandi che ci sono; `catalogo_scenario`, verso una funzione nuova del core: lo aggiunge [Scenario ed editor](14-scenario-ed-editor.md) |
| 11b | `anteprima_campagna`, `elettori_campione` |
| 11c | nessuno |
| 11d | `bozza_bilancio`, `indicizza`, `riepilogo_bilancio`, `riepilogo_presentato` |
| 11e | `anteprima_sfiducia` |
| 11f | `registro`, che porta il piano di core |
| 11g | `menu_elettorale`, `costo_dirotta`: li lascia a questo ticket la spec del core (`:28`); corregge il «nessuno» di [Fette](10-fette.md) |

### Scartato

Seme nella UI o in un campo del core; tipo `Fascia` a mano o elettori campione rimandati a 11g; errore come testo solo; bozze nella memoria del webview; `#[ts(type)]` campo per campo al posto della variabile d'ambiente.
