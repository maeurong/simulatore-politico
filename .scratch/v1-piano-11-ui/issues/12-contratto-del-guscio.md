Type: grilling
Status: open
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
