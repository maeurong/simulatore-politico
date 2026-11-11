Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Per ogni schermata della spec v1 (decisioni UI, `docs/superpowers/specs/2026-10-21-v1-giocabile.md:397`), quali campi della Vista (`app/src/lib/tipi/Vista.ts`, `crates/sim-core/src/vista.rs`) e quali metodi pubblici della partita (`crates/sim-core/src/partita.rs`, `Comando` in `crates/sim-core/src/comando.rs`) la servono già su main? Quali comandi del guscio (`app/src-tauri/src/lib.rs:53-94`) mancano per raggiungerli? Cosa manca del tutto, e quale piano lo produce: 8b (PR #36/#37), 9, 10, o nessuno?

Da controllare in particolare:
- le voci dell'esito del turno con la loro causa e il registro (storie 14-16): c'è una fonte, o nessun piano le porta?
- fondazione e scelta del posto (storie 8-9, `AnteprimaFondazione`): raggiungibili dal guscio?
- i rimandi «Piano 11» già scritti nei piani 1, 3, 7a, 7b, 8a (`grep -rn "Piano 11" docs/superpowers/plans/`).

Uscita: una tabella schermata → campi → comandi → mancanti → piano, in `docs/ricerca/sintesi/`.

## Answer

Pagina: [vista-per-schermata](../../../docs/ricerca/sintesi/vista-per-schermata.md), letta su main a `c56787d` (8b già fuso, #36-#38).

- **Guscio**: manca `comanda` (`crates/sim-core/src/comando.rs:205`), che da solo apre le 28 varianti di `Comando`. Mancano anche, come passacarte verso metodi pubblici: `anteprima_fondazione`/`fonda`, `anteprima_sfiducia`, `anteprima_campagna`, `elettori_campione`, `bozza_bilancio`/`indicizza`/`riepilogo_bilancio`/`riepilogo_presentato`, `abbandona`. `nuova_partita` usa `Partita::nuova` (`app/src-tauri/src/lib.rs:35`), cioè la partita dal governo uscente, non la fondazione.
- **Senza piano**: esito del turno con causa e registro (storie 14-16; la Vista ha solo frammenti che durano 1-2 turni); il predicato di «avanti fino a novità» (il piano 7b lo dà al piano 11, ma è una regola); i dati regionali della mappa (3 livelli su 5, voto e seggi per regione, gruppi della scheda, cantieri finiti); PDE e conto di controllo nella Vista; piano strutturale riveduto; minaccia di scissione persistente; caricature e motti.
- **Piano 9**: eventi, situazioni, tono grave, pericolosità ed emergenze, acquisti d'informazione. **Piano 10**: partite e albo, editor, riavvolgimento, serie dei tre indicatori.
- **Contratto del guscio**: 17 tipi generati dicono `bigint` ma `JSON.stringify` non li serializza; `Fondazione` e `Fascia` non sono esportati in TypeScript; fondazione e anteprima devono condividere il seme, che `AnteprimaFondazione` non riporta.

## Aggiornamento del 10/11/2026

La risposta sopra è del 01/11, su main a `c56787d`. Riletta su `8952a06` (#48), con i piani 9 e 10 fusi; il dettaglio con le citazioni è nella sezione datata in coda a [vista-per-schermata](../../../docs/ricerca/sintesi/vista-per-schermata.md).

- **Rimandi chiusi.** Eventi e tono grave (`Vista.eventi`, `Comando::RispondiEvento`), situazioni (`Vista.situazioni`), acquisti d'informazione (`Vista.acquisti`, `Comando::Compra`), fondo di riserva residuo, partite e albo con la serie dei tre indicatori, editor con `riepilogo_scenario` e i tre avvisi (senza maggioranza, situazioni accese, crisi del debito), riavvolgimento (`riavvolgi`), abbandono ed eredità.
- **Guscio.** `comanda` c'è e apre 30 varianti; `fonda_partita` c'è e `Fondazione` è esportato. Mancano ancora i passacarte di `anteprima_sfiducia`, `anteprima_campagna`, `elettori_campione`, `bozza_bilancio`/`indicizza`/`riepilogo_bilancio`, `riepilogo_presentato`, più `serie` (i tre indicatori a partita in corso) e, se serve, `anteprima_fondazione`.
- **Contratto.** Il nodo del seme ha cambiato forma: il guscio lo sceglie dall'orologio e il riepilogo dello scenario è senza seme. `Fascia` resta non esportato; i tipi con `bigint` sono 19.
- **Senza piano, come prima.** Esito del turno con causa e registro, «avanti fino a novità», dati regionali della mappa, PDE e conto di controllo, piano strutturale, minaccia di scissione.
- **Senza piano, nuovi (debiti del piano 9).** Stato del MES e della ristrutturazione, clausola dell'energia, avviso del bilancio ricondotto al tetto del MES, avviso della campagna tagliata. Elenco in [Dati senza piano](11-dati-senza-piano.md).

