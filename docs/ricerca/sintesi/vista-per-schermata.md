---
tipo: sintesi
fonti: []
aggiornato: 2026-11-10
---
# Vista per schermata: cosa serve già il core, cosa manca al guscio, cosa non porta nessun piano

Inventario per il piano 11 (ticket `.scratch/v1-piano-11-ui/issues/01-inventario-vista-per-schermata.md`). Per ogni schermata della spec v1 (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:398`) dice quali campi della `Vista` la servono su `main`, quali metodi pubblici di `Partita` e quali varianti di `Comando` la muovono, quali comandi Tauri mancano e quali dati non esistono, con il piano che li produce. Conclusione in breve: il core copre già gran parte del turno di governo e di opposizione, ma il guscio ne espone quasi nulla, perché manca un comando `comanda`. **Esito del turno e registro non li porta nessun piano.**

Fotografia presa da `/Users/mario/GitHub/simulatore-politico`, branch `main`, HEAD `c56787d` (merge della #38), il 2026-11-01. Fonti: solo codice e documenti del repo, citati `file:riga`. Nessuna fonte esterna, quindi niente in `fonti/`. Tag: [V] letto in sessione, [M] misurato con un comando, [INF] inferenza.

## Correzioni al brief

- **8b è già su main.** Le PR #36 (`feat/v1-piano-8b-vita`), #37 (`feat/v1-piano-8b-movimento`) e #38 (`feat/v1-piano-8b-campagna`) risultano `MERGED` il 29/10/2026 [M: `gh pr list --state all --limit 8`]; `git log origin/main` ha in testa `c56787d … (#38)`, `631c0f9 … (#37)` e `8ef5d6f … (#36)` [M]. Quindi in questa pagina «8b» non è un rimando: i suoi dati sono su main e stanno nella colonna «Vista».
- **L'elenco delle schermate sta alla riga 398 della spec.** La 397 è la direzione «S · Il Settimanale».
- **`Vista.ts` è allineato a `vista.rs`.** Entrambi sono stati toccati per l'ultima volta in `c56787d` [M: `git log -1 -- <file>`], quindi qui si cita `crates/sim-core/src/vista.rs`.

## Il guscio oggi

- Il guscio registra cinque comandi: `nuova_partita`, `chiudi_turno`, `anteprima`, `anteprima_decreto` e `anteprima_accogli` (`app/src-tauri/src/lib.rs:104-110`) [V].
- **Non esiste un comando che passi un `Comando` al core.** `Partita::comanda` (`crates/sim-core/src/comando.rs:205`) è pubblico, ma il guscio non lo chiama mai. Senza di lui la UI non può applicare nessuna delle 28 varianti di `Comando` (`comando.rs:16-139`): Ferma, Dirotta, Accogli, PresentaLegge, Campagna, Sfiducia e tutte le altre [V]. La ricerca sul confine core/UI prevedeva proprio un `comando(c)` (`docs/ricerca/tecnica/core-rust-e-ui-separata.md:15`).
- **`nuova_partita` non fonda il partito del giocatore.** Chiama `Partita::nuova` (`lib.rs:36`), che crea «la partita di prova dal governo uscente»: il giocatore è il partito del Presidente del Consiglio (`partita.rs:134-138`). La partita vera nasce da `Partita::fonda` (`legislatura/fondazione.rs:106-112`), che il guscio non espone [V].
- **I tipi interi dicono `bigint` ma arrivano come `number`.** ts-rs traduce gli `i64` in `bigint` in 17 file di `app/src/lib/tipi/`, fra cui `Spesa`, `Bilancio`, `Decreto` e `VoceMinistero` [M: `grep -l bigint`]. Il JSON che arriva dal core porta invece numeri. C'è anche un problema in uscita: `JSON.stringify({euro: 1n})` lancia `TypeError: Do not know how to serialize a BigInt` [M, node v26.7.0]. Un `Comando::Campagna`, un `PresentaBilancio` o un `EmanaDecreto` di fondi costruiti con veri `bigint` fallirebbero quindi prima di arrivare al guscio, se `invoke` serializza gli argomenti in JSON [INF: `node_modules` non è installato e non ho potuto leggere `@tauri-apps/api`]. Gli importi stanno sotto 1e15 (`partita.rs:1323`) e dunque sotto 2^53: un `number` basta.
- **Due tipi d'ingresso non sono esportati in TypeScript.** `Fondazione` (`fondazione.rs:15-19`) e `Fascia` (`elettorato/mod.rs:71-77`) hanno `Deserialize` ma non `ts(export)`, e in `app/src/lib/tipi/` non hanno un file [M: `ls`]. La UI dovrebbe scriverne a mano la forma (`{nome, posizione}`, `"bassa"|"media"|"alta"`), oppure il core dovrebbe aggiungere due derive, che però sono una modifica a sim-core.

## Comandi Tauri passacarte da aggiungere (metodi già pubblici)

| Comando del guscio | Metodo del core | Schermate |
|---|---|---|
| `comanda(c: Comando) -> Vista` | `Partita::comanda` `comando.rs:205`, poi `vista()` `partita.rs:3336` | quasi tutte |
| `anteprima_fondazione() -> AnteprimaFondazione` | `Partita::anteprima_fondazione(dati, seme)` `fondazione.rs:141` | fondazione |
| `fonda(Fondazione) -> Vista` (sostituisce `nuova_partita`) | `Partita::fonda(dati, seme, f)` `fondazione.rs:112` | fondazione |
| `anteprima_sfiducia(dicastero?)` | `partita.rs:3195` | opposizione |
| `anteprima_campagna(spese)` | `partita.rs:1899` | campagna |
| `elettori_campione(regione, condizione, fascia)` | `partita.rs:2025` | mappa, scheda della regione |
| `bozza_bilancio()`, `indicizza(b, voce)`, `riepilogo_bilancio(b)` | `partita.rs:1316`, `:1324`, `:1334` | passaggio del bilancio |
| `riepilogo_presentato()` | `partita.rs:1341` (per l'opposizione: piano 8a, `2026-10-28-…-8a…md:466`) | aula, opposizione |
| `abbandona() -> Eredita` | `partita.rs:2116` | eredità (storia 152) |
| `salva` / `carica` | `partita.rs:3516`, `:3540` | partite in corso: è persistenza, piano 10 (spec `:391-393`) |

**Un nodo da sciogliere.** `anteprima_fondazione` e `fonda` devono ricevere **lo stesso seme**, perché l'anteprima è il sondaggio «della partita che `fonda` farebbe con `seme`» (`fondazione.rs:139-140`). Oggi il seme lo sceglie il guscio dall'orologio (`lib.rs:27-33`), e `AnteprimaFondazione` non lo riporta (`fondazione.rs:35-38`). Restano tre strade: (a) il guscio tiene in stato seme e dati in attesa, ma la decisione del 01/11 vuole comandi «senza logica né stato» (`.scratch/v1-piano-11-ui/map.md`, Notes); (b) la UI genera il seme e lo passa, ma un `u64` supera 2^53 e torna il problema del `bigint`; (c) il core aggiunge il seme ad `AnteprimaFondazione`, ma questo tocca sim-core. Va deciso [INF].

## Tabella: schermata → Vista → comandi e metodi → guscio mancante → dati mancanti → piano

Nella tabella «Vista» si riferisce ai campi di `Vista` in `crates/sim-core/src/vista.rs` (righe fra parentesi). «Comando» si riferisce alle varianti di `comando.rs:16-139`.

| Schermata (spec `:398`) | Campi della Vista che la servono | Comandi e metodi | Guscio mancante | Dati mancanti | Piano |
|---|---|---|---|---|---|
| **Partite in corso e albo** | nessuno; `elezioni` (`:79-80`) conservate tutte «per l'albo» (`2026-10-27-…-7a…md:804`, `:1289`) | `salva`/`carica` (`partita.rs:3516`, `:3540`) | file per partita, elenco, albo | elenco delle partite, salvataggi, albo con le serie dei tre indicatori sintetici (spec `:392`) | **10** (`…-piano-1-fondamenta.md:51`) |
| **Scelta dello scenario ed editor con riepilogo dal vivo** | nessuno | nessuno: il seme e lo scenario di base arrivano solo da `Partita::nuova`/`fonda` | tutto | cursori, riepilogo dal vivo, avvisi, scenario su misura (storie 2-7) | **10** (`…piano-1…:51`) |
| **Fondazione** (storie 8-9) | `AnteprimaFondazione { istogramma, posizioni[{posizione, quota_attesa, disponibilita}] }` (`fondazione.rs:24-38`) | `Partita::anteprima_fondazione` (`fondazione.rs:141`), `Partita::fonda` (`:112`) | `anteprima_fondazione`, `fonda`; nodo del seme (sopra) | tipo TS di `Fondazione` (sopra) | core pronto (piano 7a); il guscio spetta al piano 11 (`…7a…md:1290`) |
| **Esito come prima pagina** (storie 14-15, 101, 116, 129, 153) | solo frammenti: `mosse` delle IA nell'ultima chiusura (`:88-89`), con le spese di campagna come righe (`…8b…md:48`); `esiti` dei voti (`:62`); `esiti_referendum` (`:65`); `elezioni` (`:80`); `variazioni` di soddisfazione (`:540-541`) e d'intesa (`:171-172`) degli ultimi due turni; `statistica.revisione` (`:414`); `causa_dimissioni` (`:446-447`); `causa_ultimatum` (`:168-169`) | nessuno (sola lettura) | nessuno | **nessuna voce d'esito strutturata**: niente ordine della chiusura, niente causa per ogni voce, niente criticità, nessuna regola per le «7 in evidenza, prima le critiche» (spec `:390`); niente voce sul reddito sotto l'atteso (storia 116); niente esito dei singoli atti eseguiti | **nessuno** (vedi sotto) |
| **Cruscotto del turno** (storie 13, 17-21, 155) | `turno`, `data`, `voto`, `turni_al_voto` (`:22-31`); consenso da `sondaggio` (`:45`), `coalizione` (`:74`) e `alleati` (`:54`); maggioranza da `seggi` (`:53`) e `proiezione` (`:47`); conti da `spread_pb` (`:40`), che è il «conti» del glossario (`CONTEXT.md:358`); `fase`, `governa`, `crisi`, `dimissioni` (`:27-35`, `:84-87`) | `chiudi_turno` (`partita.rs:346`) | nessuno | serie storica dei tre indicatori (le caselle che avanzano): la Vista ha solo il valore corrente | serie: **10** (albo, spec `:392`) |
| **Listino-borsino dei ministri** (storie 25-38, 154) | `ministeri[]` (`:44`): `ministro`/`rosa` con capacità, posizione, obiettivo, soddisfazione, minaccia, causa, variazioni (`:527-541`); `agenda` con motivo, stato e `in_evidenza` (`:478-487`); `atti` con motto (`:491-502`); `cantieri` (`:516-522`); `margine_residuo`; `motivo_senza_agenda` (`:437-459`) | `Comando::Ferma`, `Dirotta`, `FermaCantiere`, `ChiediDimissioni`, `ScegliSuccessore` (`comando.rs:17-35`) | `comanda` | caricatura e motto del ministro (storia 154): `VoceMinistro` non li ha (`:527-541`), e i contenuti sono «da scrivere dopo la spec» (spec `:484`); anteprima del costo in soddisfazione di Ferma/Dirotta (storia 28); evento aperto nel dicastero come motivo di evidenza (`…piano-5…md:707`) | caricature: **nessuno** (contenuto); evidenza per evento: **9** |
| **Aula e calendario** (storie 54-69, 73, 75) | `calendario[]` `VoceMisura` con linee, forbice, emendamenti, `alleati_contro_fiducia`, `seggi` per la legge elettorale (`:211-230`); `esiti` (`:62`); `referendum` (`:64`, `:141-150`); `disposizioni` (`:51`, `:303-317`); `posto_riservato` (`:60`) | `Comando::PresentaLegge`, `Emendamento`, `PoniFiducia`, `EmanaDecreto`, `InvitaAstensione`, `Linea` (`comando.rs:52-70`, `:98-101`); `anteprima`, `anteprima_decreto` (`partita.rs:1520`, `:1554`) | `comanda` | posti liberi in calendario: `POSTI_ORDINARI = 3` è una costante (`parlamento/mod.rs:34`), non un campo; il decreto su disposizioni con evento aperto come innesco (`…piano-6…md:31`) | posti: **nessuno** (minore); innesco: **9** |
| **Proposte** (storie 30-31) | `ministeri[].proposta` `VoceProposta` con motivo e rinvio (`:458`, `:465-473`) | `Comando::Accogli`, `Rinvia`, `Respingi` (`comando.rs:36-44`); `anteprima_accogli` (`partita.rs:1573`, già nel guscio a `lib.rs:93-99`) | `comanda` | innesco «evento nel dicastero» (`…piano-5…md:26`) | **9** |
| **Eventi** (storie 130-136; tono grave spec `:400`) | nessuno | nessuno | — | eventi, opzioni, default, situazioni con soglia visibile, tono grave | **9** (`…piano-1…:50`) |
| **Passaggio del bilancio** (storie 39-53) | `bilancio` `VistaBilancio` con budget, aliquote, perdita reale, fondo, `presentabile`, `presentato`, `esercizio_provvisorio` (`:340-367`); il bilancio presentato come `VoceMisura` di forma `Bilancio`, con gli emendamenti degli alleati (`parlamento/mod.rs:45-53`, `partita.rs:1155`, `:1364`) | `bozza_bilancio`, `indicizza`, `riepilogo_bilancio` (`partita.rs:1316-1337`); `Comando::PresentaBilancio`, `Emendamento` | `comanda`, `bozza_bilancio`, `indicizza`, `riepilogo_bilancio` | stato della PDE, conto di controllo e debito/PIL: esistono nel core (`economia/regole_ue.rs:14-27`) ma non nella Vista (storia 50); il piano strutturale riveduto (storia 51), rimandato al 7 (`…piano-3…md:24`) e assente dal core [M: `grep -rn -i 'strutturale' crates/sim-core/src` → nessun risultato]; fondo di riserva speso dalle emergenze (`…piano-5…md:28`); aste deserte, MES e ristrutturazione (`…piano-3…md:24`) | PDE e debito nella Vista: **nessuno**; piano strutturale: **nessuno** (perso fra 3 e 7); emergenze e debito: **9** |
| **Anteprima della misura** (storie 55-56, 156) | `Anteprima` con costo, effetti a fasce, leve, gruppi, linee, forbice, seggi (`:237-248`) | `anteprima`, `anteprima_decreto`, `anteprima_accogli` (`partita.rs:1520-1573`) | nessuno: già esposti (`lib.rs:80-99`) | perimetro della disposizione territoriale da evidenziare (storia 141): c'è in `VoceDisposizione.perimetro` (`:315`) | — |
| **Coalizione pre-voto, trattativa e spartizione** (storie 77-87, 98) | `disponibilita` (`:68-71`), `coalizione` (`:72-74`), `richieste` (`:75-78`), `incarico` (`:81-82`, `:129-133`), `elezioni` (`:80`), `seggi` | `Comando::DichiaraCoalizione`, `ProponiSpartizione`, `Allarga`, `Dimettiti`, `ChiediGuida`, `RifiutaRibaltone` (`comando.rs:73-95`) | `comanda` | — | core pronto (7a/7b); il guscio spetta al piano 11 (`…7a…md:1290`, `…7b…md:1227`) |
| **Mappa con livelli, segni, scheda della regione e selettore** (storie 137-142) | consenso e affluenza per regione da `sondaggio.regioni` (`Incrocio`, `informazione/sondaggio.rs:23-33`; l'astensione come ultima quota [INF] per analogia con `Rilevazione`, `:14`); le tre grandezze regionali (`stock_infrastrutture`, `investimenti_privati`, `domanda_regionale`, `dati/base/grandezze.toml:131-153`) in `grandezze[].valori` (`:426-432`); segni da `agenda.regione` e `cantieri[].regione` (`:481`, `:518`); disposizioni territoriali con `perimetro` (`:315`); `Bersaglio::Regione` per il selettore (`legislatura/campagna.rs:21-24`) | `elettori_campione` (`partita.rs:2025`); `Comando::Dirotta {regione}`, `Campagna` | `comanda`, `elettori_campione` | **disoccupazione e reddito per regione** (livelli di `CONTEXT.md:382`): esistono nello stato vero (`economia/mod.rs:38`, `economia/lavoro.rs:11-14`) ma non nella Vista; **risultato del voto per regione** e seggi del Senato per regione (storia 138): `VotiPartito` è nazionale (`app/src/lib/tipi/VotiPartito.ts`); **gruppi della regione** (storia 140): il sondaggio ha solo i margini condizione, fascia e regione (`Sondaggio.ts`), non i 18 gruppi; **seggi per regione**; **cantieri finiti**, tolti alla fine dei lavori (`ministri/agenda.rs:347`); forbice del sondaggio per regione (c'è solo `campione`); pericolosità naturale ed emergenze | regioni, voto, gruppi, seggi, cantieri finiti: **nessuno**; pericolosità ed emergenze: **9** |
| **Schema dello Stato** (storia 157) | componibile da `ministeri`, `calendario`, `referendum`, `alleati`, `partiti[].al_governo` [INF] | — | `comanda` (se è anche interattivo) | nessun campo «chi tocca cosa nel turno» | **nessuno**; si ricava dalla Vista [INF] |
| **Registro con «torna al turno N»** (storie 16, 144) | nessuno persistente: `mosse` si svuota a ogni chiusura (`partita.rs:350`); `esiti` e `esiti_referendum` tengono due turni (`parlamento/mod.rs:1037-1043`); le variazioni tengono due turni (`:171`, `:540`) | `salva`/`carica` | riavvolgimento | registro completo di ogni turno passato; riavvolgimento | registro: **nessuno**; «torna al turno N»: **10** (`…piano-1…:51`, `…7b…md:1226`) |
| **Vista dell'opposizione** (storie 88-98, 104) | `alleati_avversari` in fasce (`:55-57`, `:179-193`); `programma` del governo IA (`:90-92`, `:109-114`); `mosse` (`:89`); agende nascoste per costruzione (`partita.rs:3382`); `posto_riservato` (`:60`); `referendum[].scelta` (`:138-149`) | `Comando::Linea`, `Sfiducia`, `PresentaProposta`, `PromuoviReferendum`, `Confermativo` (`comando.rs:98-121`); `anteprima_sfiducia` (`partita.rs:3195`); `riepilogo_presentato` (`:1341`) | `comanda`, `anteprima_sfiducia`, `riepilogo_presentato` | **«avanti fino a novità»** (storia 89): le fermate (voto su una misura che tocca l'elettorato del giocatore, ultimatum, crisi, finestra del referendum, turno prima del voto; spec `:387`) sono regole di gioco, ma nel core non c'è un predicato [M: `grep -rn -i 'novit' crates/` → nessun risultato], e il piano 7b le ha assegnate al piano 11 UI (`…7b…md:1227`) | **nessuno** nel core; vedi «Scoperte» |
| **Campagna** (storie 125-129) | `campagna { attiva, spese }` (`:94`, `:101-104`); `partiti[]` con `cassa`, `promesse`, dichiarata e percepita (`:380-406`) | `Comando::Riposiziona`, `Campagna`, `Prometti` (`comando.rs:125-138`); `anteprima_campagna` (`partita.rs:1899`) | `comanda`, `anteprima_campagna` | **minaccia di scissione** come stato: c'è in `Partito` (`fondazione.rs:73`) ma non in `VocePartito`; appare solo come riga in `mosse` nel turno in cui scatta (`partita/vita.rs:112-134`); acquisti d'informazione con la cassa (storie 123-124) | minaccia: **nessuno**; acquisti: **9** (`…8b…md:855`) |
| **Eredità** (storie 149-152) | `eredita` con fine, bozza, anni, leggi in vigore, indicatori (`:66-67`, `Eredita.ts`); `finita` (`:35`) | `abbandona` (`partita.rs:2116`) | `abbandona` | numero di riavvolgimenti; «torna a un turno» dalla bozza; conferma nell'albo | **10** (`…7b…md:1226`) |

## L'esito del turno e il registro (storie 14-16)

La domanda del ticket è se qualcosa produce le voci dell'esito con la causa e il registro. Su main non c'è niente, e nessun piano lo prevede:

- **Il core non ha un tipo «voce dell'esito».** La `Vista` (`vista.rs:21-95`) non ha un campo esito o registro. Ci sono frammenti con forme diverse: righe di testo (`MossaIa`, `:119-122`), esiti dei voti (`EsitoVoto`), variazioni per ministro e per alleato con una causa enumerata (`Causa`, `CausaIntesa`), la revisione della statistica. Nessuno porta l'ordine della chiusura, la criticità o un flag «in evidenza» [V].
- **La memoria è corta per costruzione.** `mosse` si svuota a ogni `chiudi_turno` (`partita.rs:114-115`, `:350`); gli esiti dei voti e dei referendum tengono due turni (`parlamento/mod.rs:1037-1043`); le variazioni tengono due turni (`vista.rs:171`, `:540`). Del turno N-3 non resta niente da consultare [V].
- **Ogni piano ha rimandato la questione, e la tabella non l'assegna a nessuno.** Piano 4: «il registro del turno nasce col piano che porta l'esito» (`…piano-4…md:30`). Piano 5: «La tabella dei piani non li assegna… la scelta delle 7 voci in evidenza resta aperta» (`…piano-5…md:32`). Il piano 6 ripete lo stesso (`…piano-6…md:34`). La tabella (`…piano-1-fondamenta.md:42-53`) non nomina esito né registro in nessuna riga: il 9 fa «situazioni, eventi puntuali, pericolosità naturale, acquisti d'informazione, centro studi» (`:50`), il 10 «editor…, riavvolgimento, bozza, più partite, albo» (`:51`) [V].
- **Il piano 11 non può prenderlo da sé.** «Prima le critiche» e «al massimo 7» sono una regola (spec `:390`), e la spec vuole che la UI non calcoli regole di gioco (`:247`) e che le regole di visibilità stiano nel core (`:248`). Comporre l'esito nella UI dai frammenti metterebbe la priorità nella UI e contraddirebbe la decisione del 01/11 di non toccare sim-core [V spec; INF conflitto].
- **Un aggancio possibile [INF].** Se il piano 10 salva uno snapshot a ogni apertura (spec `:391`), il guscio potrebbe ricostruire la `vista()` di ogni turno passato, e da lì i frammenti dell'esito di quel turno, senza stato nuovo nel core. Darebbe un registro grezzo, ma non l'ordine, la causa per voce e la criticità.

## Scoperte che cambiano le decisioni

1. **Esito del turno e registro senza piano.** Nessun campo li porta e nessun piano li assegna (sezione sopra). Va deciso chi li produce: un piano di core prima del piano 11 (un «9½»), un'estensione del 9 o del 10, oppure il piano 11 deroga alla regola «non tocca sim-core». È la voce «Esito del turno e registro senza fonte» della mappa, e questa ricerca la conferma.
2. **«Avanti fino a novità» assegnato alla UI ma regolato dal core.** Il piano 7b lo dà al piano 11 (`…7b…md:1227`), ma le sue fermate sono regole (spec `:387`) e una di esse, «una misura che tocca l'elettorato del giocatore», richiede calcoli che la Vista non espone. Senza un predicato nel core, il piano 11 dovrebbe inventare una regola.
3. **Il guscio non ha `comanda`.** Un solo comando passacarte apre 28 varianti e quasi tutte le schermate di governo e di opposizione. È la fetta zero del piano 11 e sblocca da sola cruscotto, borsino, aula, proposte, trattativa, opposizione e campagna.
4. **`nuova_partita` gioca la partita di prova, non quella vera.** Va sostituito da `anteprima_fondazione` + `fonda`, e prima va sciolto il nodo del seme condiviso (tre strade, ciascuna con un costo: stato nel guscio, `u64` in JS, campo nuovo nel core).
5. **La mappa ha solo due livelli su cinque.** Consenso e affluenza ci sono, dal sondaggio regionale. Disoccupazione e reddito per regione esistono nello stato vero ma non nella Vista, e nessun piano li porta; la pericolosità arriva col 9. Mancano anche il risultato del voto per regione, i seggi per regione, i 18 gruppi della scheda e i cantieri finiti. Il loro posto nella Vista dovrebbe passare anche dall'informazione imperfetta: una prima stima regionale e non lo stato vero (spec `:248`) [INF].
6. **Conti UE fuori dalla Vista, piano strutturale perso.** La PDE e il conto di controllo vivono in `RegoleUe` (`economia/regole_ue.rs:14-27`) senza un campo nella Vista (storia 50). Il piano strutturale riveduto (storia 51), rimandato dal 3 al 7, non c'è nel core.
7. **`bigint` nei tipi generati.** I 17 tipi con `i64` dicono `bigint`, ma i valori arrivano come `number`, e `JSON.stringify` rifiuta i `bigint`. Il piano 11 deve fissare una convenzione (trattarli come `number`) prima di costruire un `Comando` con importi.
8. **Rimandi di contenuto senza piano.** Le caricature e i motti dei ministri (storia 154) e le caricature dei leader (8a, `…8a…md:905`) sono contenuto (spec `:484`) senza una riga nella tabella. Al piano 11 servono dei segnaposto.

## Rimandi «Piano 11» già scritti

Raccolti con `grep -rn -i "piano 11" docs/superpowers/plans/ docs/superpowers/specs/` [M]:

- `…piano-1-fondamenta.md:26`: testi esternalizzati (storia 166); le etichette provvisorie si buttano.
- `…piano-1-fondamenta.md:52`, `:55`: riga della tabella; il piano 11 parte a fette dopo il 5.
- `…piano-2-riparto-elettorale.md:15`: il menu della legge elettorale come schermata, con i gradini spenti.
- `…piano-3-motore-economico.md:1060`: la riga `<p>` provvisoria.
- `…piano-7a…md:1290`: fondazione sopra `AnteprimaFondazione`, coalizione e trattativa, `EsitoReferendum.decaduto`.
- `…piano-7b…md:1227`: «avanti fino a novità», crisi, sfiducia, ribaltone, opposizione, eredità.
- `…piano-8a…md:466`: `riepilogo_presentato` all'opposizione; `:905`: caricature dei leader, programma, mosse IA, confermativo.
- `…piano-8b…md:66`, `:856`: campagna (bersagli, anteprima a fasce, promesse), casse dei rivali, righe di spesa nell'esito, minacce di scissione, nascite e scioglimenti sull'asse.

Nota sul menu della legge elettorale (piano 2, `:15`): i gradini sono in `VoceDisposizione.gradini` (`vista.rs:316`, `:329-334`), ma la regola «combinazioni impossibili spente» (storia 72) non ha un campo: `VoceGradino` porta solo `bloccato` per il referendum (`:326-334`). Anche lì la UI dovrebbe calcolare una regola → **nessuno** [INF].

## Per il nostro gioco

- **Prendiamo:** come prima fetta del piano 11, `comanda` più i passacarte della tabella. Le schermate di governo, aula, trattativa, opposizione e campagna hanno già i loro dati (tabella sopra).
- **Evitiamo:** comporre nella UI l'esito, le fermate di «avanti fino a novità», le combinazioni spente della legge elettorale e la forbice per regione. Sono regole, e la spec le vuole nel core (`:247-248`).
- **Aperto per Mario (raccomandazione, non decisione):** un piccolo piano di core «esito e registro», prima o accanto al 9, che porti nella Vista le voci ordinate con causa e criticità, la memoria per turno e il predicato delle fermate. Ci metterebbe anche i dati regionali mancanti della mappa, in forma di stima. Senza quel piano, le schermate dell'esito, del registro e della mappa restano scheletri.

## Collegamenti

- [[feedback-visivo-dello-stato]]: i sei canali e il rischio di notifiche; l'esito a 7 voci è la risposta della spec, ma non ha una fonte nel core.
- [[core-rust-e-ui-separata]]: l'interfaccia stretta prevista (`comando(c)`, `snapshot()`, `salva()`/`carica()`, riga 15) contro il guscio di oggi, che ha solo `nuova_partita` e `chiudi_turno`.
- [[decisioni]]: dove registrare chi produce esito e registro, una volta deciso.

## Aggiornamento del 2026-11-10: piani 9 e 10 su main

Aggiunta datata: il testo sopra resta la fotografia del 01/11 a `c56787d` e non è riscritto. Questa sezione è letta su `origin/main` a `8952a06` (#48), con i piani 9 (#44-#47) e 10 (#41-#43) fusi. Dove le due parti si contraddicono vale questa. Stessi tag: [V] letto, [M] misurato, [INF] inferenza.

### Rimandi chiusi

- **Eventi** (riga «Eventi» della tabella, rimando al 9): `Vista.eventi: Vec<VoceEvento>` con famiglia, causa, tono, regione, bersaglio, costo, opzioni e scelta (`app/src/lib/tipi/Vista.ts`, `VoceEvento.ts`); si risponde con `Comando::RispondiEvento` (`crates/sim-core/src/comando.rs:146`) [V]. Il tono grave arriva come `VoceEvento.tono`.
- **Situazioni**: `Vista.situazioni: Vec<VoceSituazione>` con misura, valore ufficiale, soglie di accensione e spegnimento, `accesa` (`VoceSituazione.ts`) [V].
- **Acquisti d'informazione** (riga «Campagna», rimando al 9): `Vista.acquisti: VistaAcquisti` con sondaggio privato, test di una misura, centro studi e prezzi (`VistaAcquisti.ts`); si compra con `Comando::Compra` (`comando.rs:47`) [V]. I dati ci sono, la schermata no.
- **Fondo di riserva speso dalle emergenze** (riga «Passaggio del bilancio»): `VistaBilancio.fondo_residuo` (`VistaBilancio.ts`) [V].
- **Partite in corso e albo** (rimando al 10): comandi del guscio `partite`, `riprendi`, `albo` (`app/src-tauri/src/lib.rs:240-318`), tipi `VocePartita`, `Testata`, `VoceAlbo`. `VoceAlbo.serie` porta i tre indicatori per turno (`PuntoSerie`, `crates/sim-core/src/salvataggi.rs:18-25`) [V].
- **Editor con riepilogo dal vivo** (rimando al 10): `scenari`, `salva_scenario`, `carica_scenario`, `riepilogo_scenario` (`lib.rs:320-358`); `RiepilogoScenario` porta spread, interessi, disavanzo, saldo delle disposizioni, una `PosizioneAttesa` per posizione della fondazione e gli `avvisi` [V].
- **Avvisi del riepilogo**: `Avviso` ha tre varianti, `senza_maggioranza`, `situazioni_accese`, `crisi_del_debito` (`app/src/lib/tipi/Avviso.ts`; le ultime due dalla #48) [V].
- **Riavvolgimento** (rimando al 10): comando `riavvolgi(turno)` (`lib.rs:286-293`); `Eredita.riavvolgimenti` e `VoceAlbo.riavvolgimenti` [V].
- **Eredità e abbandono**: `abbandona` e `conferma_eredita` nel guscio (`lib.rs:295-309`) [V].
- **`comanda`**: esiste (`lib.rs:277-284`) e apre le 30 varianti di `Comando` (erano 28: si aggiungono `Compra` e `RispondiEvento`) [M: conteggio delle varianti in `comando.rs:18-146`]. La scoperta 3 è chiusa.
- **Fondazione**: `fonda_partita(scenario, fondazione)` (`lib.rs:249-266`); `Fondazione` ora è esportato in TypeScript (`app/src/lib/tipi/Fondazione.ts`) [V]. La scoperta 4 è chiusa per metà, vedi sotto.

### Cosa manca ancora al guscio

Metodi pubblici del core senza comando Tauri [M: confronto fra `generate_handler!` a `lib.rs:405-423` e i `pub fn` di `partita.rs`]:

| Metodo | Dove | Schermata |
|---|---|---|
| `anteprima_sfiducia` | `partita.rs:3802` | opposizione |
| `anteprima_campagna` | `partita.rs:2422` | campagna |
| `elettori_campione` | `partita.rs:2548` | mappa, scheda della regione |
| `bozza_bilancio`, `indicizza`, `riepilogo_bilancio` | `partita.rs:1666`, `:1674`, `:1684` | passaggio del bilancio |
| `riepilogo_presentato` | `partita.rs:1691` | aula, opposizione |
| `serie` | `partita.rs:2600` | cruscotto: le caselle dei tre indicatori a partita in corso |
| `anteprima_fondazione` | `legislatura/fondazione.rs:142` | fondazione |

- **Il nodo del seme ha cambiato forma.** `fonda_partita` prende il seme dall'orologio (`lib.rs:257`) e non lo riceve dalla UI. Il riepilogo dello scenario è «deterministico e senza seme» (`crates/sim-core/src/riepilogo.rs:14-15`, `:116-118`) e dà già la quota attesa per posizione. Resta da decidere se la schermata della fondazione si nutre solo di `riepilogo_scenario`, rinunciando a istogramma e disponibilità di `AnteprimaFondazione`, o se serve ancora un passacarte per `anteprima_fondazione` con un seme che non è quello della partita [INF].
- **`Fascia` non è ancora esportato** in TypeScript [M: `app/src/lib/tipi/Fascia.ts` assente].
- **`bigint`**: i file generati che lo dicono sono 19, non più 17 [M: `grep -l bigint app/src/lib/tipi/*.ts | wc -l`]. La scoperta 7 resta aperta.

### Dati che mancano ancora, senza piano

Ricontrollati su `8952a06`:

- **Esito del turno con causa e registro** (storie 14-16): la Vista non ha un campo esito né registro [V: `Vista.ts`]. Il riavvolgimento c'è, il registro da cui scegliere il turno no.
- **«Avanti fino a novità»** (storia 89): nessun predicato nel core [M: `grep -rn -i novit crates/` trova solo un commento a `partita.rs:331`].
- **Dati regionali della mappa**: disoccupazione e reddito per regione assenti da `vista.rs` [M: `grep -n 'disoccupaz\|reddito' crates/sim-core/src/vista.rs` → nessun risultato]. Voto e seggi per regione, gruppi della scheda e cantieri finiti non ricontrollati in questa sessione: valgono come al 01/11.
- **PDE e conto di controllo nella Vista** (storia 50): assenti [M: `grep -n -i pde crates/sim-core/src/vista.rs` → nessun risultato].
- **Minaccia di scissione persistente** (storia 108): `VocePartito` non la porta (`vista.rs:446-461`) [V].
- **Piano strutturale riveduto** (storia 51): non ricontrollato, vale come al 01/11.

### Debiti nuovi lasciati dal piano 9

- **Stato del MES e della ristrutturazione fuori dalla Vista.** La scelta passa da un evento («Aste scoperte», `partita.rs:393-420`) e quindi dalla schermata degli eventi. Lo stato che ne segue vive in `Economia` (`crates/sim-core/src/economia/mod.rs:106-111`) e la Vista non lo porta [M: `grep -n -i 'mes\b\|ristruttur' crates/sim-core/src/vista.rs` → nessun risultato]: la UI non sa se un programma è in corso né fino a quando.
- **Clausola dell'energia**: l'effetto scrive `clausola_energia_fino_a` in `regole_ue`, quindi è nazionale (`partita.rs:4220-4227`, `economia/regole_ue.rs:31`; in una prima stesura questa riga diceva «sulla regione», corretto lo stesso giorno); nella Vista non c'è [M: `grep -n -i clausola crates/sim-core/src/vista.rs` → nessun risultato].
- **Bilancio ricondotto al tetto del MES senza avviso**: un bilancio presentato prima del programma «entra ricondotto» (`partita.rs:1646-1647`) e la Vista non lo segnala [V commento; INF assenza dell'avviso].
- **Campagna tagliata senza avviso** quando la cassa non basta dopo gli acquisti: riportato dalla sessione che ha chiuso il piano 9, non riletto qui nel codice.
- **Serie dei tre indicatori a partita in corso**: esiste (`Partita::serie`, `partita.rs:2600`) ma arriva alla UI solo dentro `VoceAlbo`, cioè a partita finita. È un passacarte mancante, non un dato mancante.

### Cosa cambia nelle conclusioni

- La «fetta zero» (scoperta 3) non è più `comanda`: è l'elenco dei passacarte qui sopra.
- L'esito e il registro restano la mancanza maggiore, e ora sono anche il posto naturale dei due avvisi mancanti (tetto del MES, campagna tagliata) [INF].
- Le schermate di eventi, situazioni, acquisti, partite, editor, albo ed eredità hanno i loro dati: si possono prototipare su Viste vere.
