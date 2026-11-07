Type: grilling
Status: open
Blocked by: -
Assignee: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Com'è fatto il contratto fra UI e guscio Tauri? Elenco dei comandi passacarte (`comanda` più quelli dell'[inventario](01-inventario-vista-per-schermata.md)); i 17 tipi generati come `bigint` che `JSON.stringify` non serializza (convenzione in ingresso e in uscita); `Fondazione` e `Fascia` non esportati in TypeScript; il seme condiviso fra `anteprima_fondazione` e `fonda` (stato nel guscio, `u64` da JS o campo nel core); come la UI riceve gli errori del core. Decisione tecnica: si risolve con Mario solo dove tocca il core.

## Answer
