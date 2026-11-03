# V1 · Piano 9: eventi e informazione — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Il paese si muove anche senza il giocatore. Sei situazioni a soglia con isteresi (recessione, ondata di criminalità, collasso delle liste d'attesa, emergenza carceri, crisi energetica, ondata di sbarchi) si accendono sulle grandezze del motore e aprono un evento; ogni turno può nascere al massimo un evento puntuale (emergenza naturale per regione, scandalo, shock economico oltre 2,5σ), con 2-3 opzioni chiuse e un default; la crisi del debito apre la scelta fra MES e ristrutturazione. Ogni evento cita la sua causa nel modello, apre il decreto sui dicasteri coinvolti, dà un innesco alle proposte dei ministri; il governo IA sceglie sempre il default. Con la cassa del partito il giocatore compra un sondaggio privato, il test di una misura e il centro studi, e ogni acquisto toglie denaro alla campagna.

**Architecture:** Un modulo nuovo `eventi/` tiene catalogo, stato e regole pure: `eventi/mod.rs` (tipi, `Eventi` nello stato della partita, risoluzione delle opzioni), `eventi/situazioni.rs` (isteresi), `eventi/rischio.rs` (candidati del turno, estrazioni con `Flusso::Eventi`, tetto), `eventi/emergenze.rs` (pericolosità per regione, gravità, prevenzione). I dati stanno in un file nuovo `dati/base/eventi.toml` (undicesimo di `FileDati`); la pericolosità in `regioni.toml`. Nella chiusura gli eventi aperti si risolvono per primi (scelta del giocatore o default), le situazioni si aggiornano dopo il motore, gli eventi del turno dopo nascono alla fine, prima di `turno += 1` (spec riga 385: «soglie → eventi del turno dopo»). L'informazione a pagamento sta in `informazione/acquisti.rs`, con un comando nuovo `Compra`; il centro studi fa girare una copia del motore senza shock.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 47 (riga 81), 53 (riga 88), 59 (riga 96), 119-124 (righe 174-179), 130-136 (righe 191-197); «Eventi e situazioni» (righe 357-364); «Informazione e cassa del partito» (righe 366-372); governo avversario e default (riga 353); iter del decreto (riga 342); punti aperti (riga 488). Ticket: `.scratch/v1-giocabile/issues/15-crisi-ed-eventi.md` (righe 30-73), `19-pericolosita-naturale-per-regione.md` (righe 21-50), `25-informazione-imperfetta.md` (righe 44-53). Rimandi dei piani precedenti: piano 4 `:28` (spinte di posizione degli eventi), piano 5 `:26`, `:28`, `:38`, `:707`, `:1398` (innesco evento delle proposte, fondo di riserva speso dalle emergenze, tre grandezze per situazioni e scandali, ministro in evidenza per evento aperto), piano 6 `:31` (inneschi del decreto), piano 8a `:904` (governo IA sempre col default; colpa per gli scandali), piano 8b `:855` (acquisti che tolgono denaro alla campagna).

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md` e `docs/ricerca/modelli/informazione-a-pagamento.md`, scritte per questo piano; più `docs/ricerca/dati/pericolosita-naturale-regioni.md`, `docs/ricerca/sistemi/crisi-ed-eventi.md`, `docs/ricerca/modelli/economia-italiana-parametri.md`.

## Quando si esegue

Non prima che il piano 8b (PR #36, #37 e la PR C su `feat/v1-piano-8b-campagna`) sia su `main`. Le premesse sul codice di questo piano sono lette su `feat/v1-piano-8b-campagna` a `d1b6f62`. Prima del Task 1:

1. riallinea `docs/v1-piano-9` a `main` (merge, niente force-push);
2. ricontrolla ogni `file:riga` citato in **Files**, **Architect** e nelle scelte: dove una riga si è spostata, correggi il piano nello stesso commit del Task 1; dove una premessa non regge più, fermati e chiedi a Mario;
3. leggi `VERSIONE_SCHEMA` su `main` e usa quel valore + 1 (11 se il piano 10 non è ancora fuso, 12 se lo è).

**Piani 10 e 11 in parallelo.** Il piano 10 si esegue insieme a questo su `feat/v1-piano-10-scenario`, già a schema 11, e tocca `partita.rs`, `dati.rs` e `scenario.rs`. Regola di Mario: chi mergia per secondo alza `VERSIONE_SCHEMA`. Se il 10 o l'11 arriva su `main` prima di questa PR, il riallineamento (merge da `origin/main`, mai rebase né force-push) la porta a 12 e lo dichiara nella PR.

**Architect (01/11):** la condizione è già vera. `origin/main` è a `c56787d` (PR C dell'8b, #38) e il suo albero coincide con `feat/v1-piano-8b-campagna`, oggi a `c95ad57`, non più a `d1b6f62` (`git diff --stat origin/main feat/v1-piano-8b-campagna` vuoto). `VERSIONE_SCHEMA` su `main` = 10 (`partita.rs:41`), quindi 11. Le righe citate sotto sono rilette su quell'albero; dove erano sbagliate, la correzione sta nella riga **Architect** del task.

## Quattro PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · eventi | `feat/v1-piano-9-eventi` | `main` | 1-4 | catalogo e stato degli eventi, schema, comando e default, inneschi del decreto e delle proposte, grandezze che si muovono da sole, turno zero coi valori veri, situazioni |
| B · emergenze | `feat/v1-piano-9-emergenze` | `feat/v1-piano-9-eventi` (impilata; dopo lo squash della base, `merge -s ours origin/main`, niente force-push) | 5-7 | rischio e tetto, emergenze naturali, fondo di riserva e decreto a debito, scandali |
| C · debito | `feat/v1-piano-9-debito` | `feat/v1-piano-9-emergenze` (impilata, stessa regola) | 8-9 | shock oltre 2,5σ, crisi del debito con MES e ristrutturazione |
| D · informazione | `feat/v1-piano-9-informazione` | `feat/v1-piano-9-debito` (impilata, stessa regola) | 10-12 | acquisti con la cassa, sondaggio privato, test di una misura, centro studi |

Questo file entra nel primo commit della PR A, insieme agli aggiornamenti di `docs/ricerca/` del branch `docs/v1-piano-9`.

## Decisioni di Mario

Del 29/10/2026, sulle domande di questo piano (memoria `decisioni-piano-9`), più la 5 del 01/11, dopo le ricerche `situazioni-ed-eventi-al-turno-zero.md` e `informazione-a-pagamento.md`:

1. **Turno zero coi valori veri.** Lo scarto dell'energia parte dai prezzi di settembre 2026 e la crisi energetica è accesa al turno zero; le carceri partono dal 128,7% e arrivano da sole al livello Torreggiani (`situazioni-ed-eventi-al-turno-zero.md:54-68`). Task 3.
2. **Deriva e rumore veri** per carceri, sbarchi e corruzione (`:91-95`), con la scala degli sbarchi allargata oltre 475 (`:70-76`). Task 3.
3. **Shock-evento oltre 2,5σ**, non 2σ come la spec (riga 362): circa 1,2 a legislatura sulle cinque serie, vicino alla storia (`:146-154`). Nessun cambio al motore. Task 8.
4. **Prezzi veri dell'informazione:** sondaggio privato 15.000 €, test di una misura 20.000 €, centro studi 150.000 € a turno (`informazione-a-pagamento.md:116-125`). Task 10.
5. **Energia nell'unità del motore** (01/11, dopo l'annotazione dell'architect): scarto 14 sul costo delle importazioni, già nei prezzi del turno zero, soglie riscalate; la crisi rientra in circa 3 trimestri. Task 3, scelte 5 e 9.

Restano valide le decisioni dei piani 6-8b (memorie `decisioni-piano-6-aperte`, `decisioni-piano-7`, `decisioni-piano-7b`, `decisioni-piano-8`, `decisioni-piano-8b`).

## Scelte del piano, da confermare in review

1. **Un modulo, un file di dati.** Eventi e situazioni in `eventi.toml`; `FileDati` passa a 11 nomi. Il guscio Tauri e il CLI leggono il file nuovo come gli altri.
2. **Ordine nella chiusura.** (a) In testa, dopo `mosse.clear()`: si risolvono gli eventi aperti (opzione scelta o default) e si svuota `aperti`. (b) Dopo il motore e prima delle soglie dei ministri: si aggiornano le situazioni sui valori del turno. (c) In coda, prima di `self.turno += 1`: nascono gli eventi del turno dopo (situazioni appena accese, crisi del debito, un evento puntuale al più). Gli eventi nati in (c) si vedono nell'apertura e si risolvono alla chiusura successiva.

   **Architect (01/11), verificato contro il commento d'ordine di `chiudi_turno` (`partita.rs:321-345`, corpo `:346-520`): nessuna invariante scritta si rompe.** (a) Dopo `mosse.clear()` (`:350`) le righe del registro della risoluzione sopravvivono; prima verrebbero cancellate. La risoluzione non estrae nulla. Le spese (fondo, debito) entrano nel passo dello stesso trimestre via `Economia.decreti`, che il passo azzera dopo l'uso (`economia/mod.rs:240-250`). Svuotare `aperti` prima dell'aula non tocca la conversione del decreto innescato: `ammetti` si chiama solo all'emanazione e all'anteprima (`partita.rs:1557`, `:1750`), la conversione non ricontrolla l'innesco. (b) Va subito dopo `statistica.aggiorna` (`:432-438`): il commento chiede che `giudizio` stia prima di `composizione`, e le situazioni non leggono nessuno dei due. (c) Dopo `capo_dello_stato` e `forma_governo` (`:469-474`), prima di `turno += 1` (`:475`): i bersagli degli scandali sono i ministri del governo appena formato, il che è giusto. Gli eventi nascono prima di `ministri.apri` (`:513`), quindi le proposte con innesco evento del Task 2 li vedono nella stessa apertura. Due cose da scrivere: il commento d'ordine, che è la fonte di verità dell'ordine, prende i tre punti nuovi nello stesso commit; e a partita finita (`:347-349`) gli eventi nati nell'ultima chiusura restano aperti senza risoluzione, quindi `RispondiEvento` si rifiuta (Task 1).
3. **Un evento aperto vale un turno.** Si sceglie un'opzione col comando `RispondiEvento`, e la si può cambiare fino alla chiusura; senza scelta vale il default. Le situazioni restano accese come stato; il loro evento no.
4. **Situazioni nazionali** (decisione del 20/10, spec riga 359), definite nei dati: misura, verso, soglia di accensione, soglia di spegnimento, evento da aprire, spinta di posizione. Le misure sono una grandezza per id, più due misure del motore: `crescita_pil` (variazione trimestrale vera del PIL reale) ed `energia` (`Economia.energia`). La recessione si accende con due trimestri consecutivi di crescita vera sotto zero e si spegne con due consecutivi sopra zero (`situazioni-ed-eventi-al-turno-zero.md:21-37`): un contatore nella situazione. Le situazioni leggono lo stato vero; la vista mostra valore ufficiale, soglia e stato.
5. **Soglie di partenza** (manopole in `eventi.toml`, al bilanciamento), dalla tabella `situazioni-ed-eventi-al-turno-zero.md:78-89`: carceri accese a 117 (Torreggiani 151% su 128,7%), spente a 110; sbarchi accesi a 250, spenti a 175; criminalità su `sicurezza` accesa sotto 85, spenta sopra 92 (il 2007 a ~124 delitti); liste d'attesa su `qualita_servizio_sanitario` accese sotto 90, spente sopra 95 (oggi è il massimo letto delle rinunce); energia accesa da 10, spenta sotto 5, nell'unità del motore (scelta 9).
6. **Al turno zero le situazioni partono accese senza evento** se la misura è già oltre la soglia: la crisi c'è già, il governo uscente l'ha ereditata. Oggi vale solo per l'energia. L'avviso dell'editor (storia 6, riga 31) è del piano 10: qui `Partita::situazioni` lo rende leggibile.
7. **Spinte di posizione** solo da criminalità e sbarchi accese (ticket 15, riga 50): a ogni chiusura, prima dell'ancora, la quota `spinta_situazione` dell'istogramma di ogni gruppo passa di un punto verso destra (dal punto k al k+1, il 7 resta). Nazionali come le situazioni: ogni gruppo, non solo una regione. `spinta_situazione` = 0,002 per situazione accesa, assunzione del piano 9 (nessuna fonte in `docs/ricerca`).
8. **Deriva delle grandezze fisse** in una lista nuova `[[deriva]]` di `grandezze.toml` (grandezza, `tasso`, `sigma`, `ritorno`, `logaritmica`), senza toccare le righe esistenti. Sulla base, prima degli effetti: lineare `b ← b·(1+tasso) + ritorno·(iniziale−b) + sigma·z`; logaritmica lo stesso in ln. Valori: carceri tasso 0,009, sigma 0,75, ritorno 0 (`:54-60`); sbarchi logaritmica, tasso 0, sigma 0,3, ritorno 0,1 (σ annua 0,74 senza stagionalità, `:70-76`); corruzione tasso 0, sigma 0,5, ritorno 0,05 (`:91-95`). `immigrazione_irregolare` passa a `max = 600`. Estrazioni da `Flusso::Eventi`, nell'ordine della lista, a ogni chiusura.
9. **Energia al turno zero** = 14 (`scenario.toml`, chiave `energia`), già nei prezzi del turno zero (decisione 5). Nel motore `energia` è lo scarto del costo di **tutte** le importazioni (`economia/mod.rs:377-384`), non il prezzo del gas: lo scarto dell'energia, circa +80% (media fra TTF ×2,1 e Brent ×1,52 del 2026T3 sul 2025T4, `situazioni-ed-eventi-al-turno-zero.md:62-68`), pesato con la quota dell'energia nelle importazioni (0,15-0,2, stima dell'architect su `economia.toml:37`, assunzione del piano 9) dà circa 14. `Economia::iniziale` calcola `prezzi_settori(dati, s.energia)` invece di 0 (`economia/mod.rs:150`): l'IPCA del 4,2% dello scenario resta lo shock già nei prezzi, e il primo passo è un rientro (inflazione importata negativa). Con `persistenza_energia` 0,7 la crisi si spegne in circa 3 trimestri senza shock. Se `l_energia_rientra` fallisce, fermarsi e chiedere.
10. **Candidati e tetto.** A ogni nascita si estraggono sempre, nello stesso ordine, tutte le uniformi di tutti i candidati (scandalo, poi emergenze per regione e tipo in ordine di indice, poi gli shock), anche quando uno è già uscito: le estrazioni non dipendono dagli esiti. Fra i candidati usciti vince quello con `gravita` più alta (dato dell'evento; per le emergenze, del livello estratto), a parità il primo nell'ordine. Gli esclusi: lo shock succede comunque nel motore e va nel registro col default; scandalo ed emergenza non succedono (ticket 15, riga 25). Situazioni e crisi del debito sono fuori dal tetto.
11. **Opzioni come dati.** Un'opzione ha nome, `default` sì/no, e una lista di effetti di tipo chiuso: `Grandezza` (come gli effetti degli atti: grandezza, intensità, ritardo, forma, durata, regione dell'evento se c'è), `Valence` (al partito del governo o a quello del bersaglio), `FondoRiserva` (spende fino a capienza il costo dell'evento), `Debito` (decreto a debito del costo dell'evento), `Dimissioni` (del ministro bersaglio), `ClausolaEnergia`, `Mes`, `Ristrutturazione`. Esattamente un default per evento, 2-3 opzioni (un evento senza scelta, lo scandalo di un leader IA, ne ha una sola, il default). `valida` rifiuta il resto col nome dell'evento.
12. **Decreto innescato.** Un evento aperto con `decreto = true` ammette il decreto sulle disposizioni ordinarie dei suoi `dicasteri`; la crisi del debito su tutte le ordinarie (spec riga 342). La spesa a debito non è un comando: è l'opzione «Decreto a debito» degli eventi con `debito = true` (effetto `Debito`), spesa per intero nel turno attraverso `Economia.decreti` (decisione d'esecuzione del 01/11, sotto il Task 6). Le regole del piano 6 restano: conversione nello stesso turno, se decade cessano gli effetti normativi e il denaro speso resta speso, nessuna ripetizione.
13. **Governo IA:** non risponde mai, quindi vale sempre il default (spec riga 353). Il giocatore all'opposizione vede gli eventi ma non risponde (comando rifiutato).
14. **Emergenze naturali** (Task 6): quattro tipi (sisma, idro, incendi, siccità) per regione. Probabilità trimestrale dalla tabella `pericolosita-naturale-regioni.md:42-86`: sisma colonna degli episodi a trimestre, con un minimo `sisma_minimo` per le regioni senza episodi; idro `emergenze_idro_anno` / 4 × peso idro della regione; incendi `min(1; 3,5·h)` nel terzo trimestre e zero negli altri (`:124`). Siccità come emergenza regionale prolungata (`situazioni-ed-eventi-al-turno-zero.md:116-127`, punto aperto della spec riga 488): `siccita_anno` = 0,4 episodi l'anno, solo nel secondo e terzo trimestre, regione estratta col peso della quota agricola (`mix.A × forze_lavoro`), effetto per 4 turni. `ponytail:` il peso agricolo è un'approssimazione; la via d'uscita è una serie per regione, che la ricerca non ha trovato.
15. **Gravità e prevenzione.** Livello estratto fra lieve, grave e con vittime (probabilità per tipo in `eventi.toml`); danno in punti d'indice su `stock_infrastrutture` e `domanda_regionale` della regione, per livello, moltiplicato per `1 − k·p`, con `p = clamp((stock_infrastrutture_regione − 100)/100, 0, 1)` e `k` = 0,85 per il sisma (DM 58/2017, `pericolosita-naturale-regioni.md:102-116`), 0,5 per idro, incendi e siccità (scelta di design, stessa pagina). Costo dell'evento in euro per livello (lieve 0,1 miliardi, grave 1, con vittime 5; `:87-100`, Emilia-Romagna 2023 a 4,7 miliardi). Il ripristino recupera la quota `min(1, speso/costo)` del danno sullo stock. Con vittime l'evento ha `tono = Grave` (storia 136).
16. **Fondo di riserva.** Il default delle emergenze spende `min(costo, fondo residuo)`; il fondo residuo cala nell'anno e torna allo stanziamento con il bilancio nuovo. Se il fondo è vuoto, il default resta il fondo (spesa 0, ripristino 0) e il giocatore può scegliere il decreto a debito: il fondo copre la prima emergenza, mai la ricostruzione (`pericolosita-naturale-regioni.md:135`).
17. **Valence della gestione** (ticket 15, riga 50): risposta (fondo con spesa positiva o debito) `+valence_emergenza_risposta` = 0,02 al partito del Presidente del Consiglio; nessun intervento `−valence_emergenza_ignorata` = 0,05. Assunzioni del piano 9, nessuna fonte in `docs/ricerca`.
18. **Scandali** (Task 7). Rischio per turno `scandalo_base × corruzione/100`, con `scandalo_base` = 0,11 (8 ministri in 74 trimestri, `situazioni-ed-eventi-al-turno-zero.md:97-114`). Bersaglio estratto uniforme fra i ministri in carica e i leader dei partiti IA attivi, mai il giocatore. Su un ministro: «Difenderlo» (default: `−valence_scandalo` = 0,05 al suo partito, ministro sotto scandalo per `turni_scandalo` = 4) o «Chiedergli le dimissioni» (la regola di `ChiediDimissioni` del piano 5: gratis con un tecnico, un colpo all'intesa altrimenti). Su un leader IA: un'opzione sola, `−valence_scandalo` al suo partito. L'opposizione IA, quando la sfiducia individuale passerebbe, sceglie il ministro sotto scandalo prima di quello col giudizio peggiore (8a, `partita/ia.rs:578-592`). Il legame CPI → scandali non ha fonte (`:110`): la formula è un'assunzione dichiarata.
19. **Shock oltre 2,5σ** (Task 8, decisione 3). Il motore salva le cinque innovazioni z del turno (`economia/mod.rs:219-224`). Diventa candidato solo lo shock sfavorevole oltre `soglia_shock` = 2,5 (domanda e domanda estera e produttività negative, energia e tasso BCE positivi); quello favorevole va nel registro come riga. Opzioni: «Niente» (default), «Aiuti per decreto a debito» (costo `aiuti_shock` = 0,3% del PIL annuo, in domanda finale col vettore del dicastero coinvolto); per l'energia anche «Attiva la clausola nazionale per l'energia».
20. **Crisi del debito** (Task 9). Nessuna asta italiana scoperta nel 2011 (`situazioni-ed-eventi-al-turno-zero.md:136-145`): l'innesco è lo spread oltre `soglia_crisi_debito` = 550 pb (massimo di novembre 2011, `economia-italiana-parametri.md:27`), fuori dal tetto, una sola crisi aperta alla volta. L'evento si chiama «Aste scoperte». Opzioni: «Programma del MES» (default): valori imposti a disposizioni elencati in `eventi.toml` (scritti per intero nel task) che entrano subito, intesa di ogni alleato `−intesa_mes`, premio di sfiducia azzerato e OMT attivo finché il programma dura (`durata_mes` = 8 turni, un anno più i due rinnovi di sei mesi, `:136-145`); «Ristrutturazione»: debito × (1 − `taglio_ristrutturazione`) con 0,5 (PSI greco, 53,5% nominale), perdita di reddito per `turni_ristrutturazione` = 8 alle fasce media e alta di tutte le condizioni (i detentori, [NON TROVATO] la ripartizione vera, assunzione), premio `premio_ristrutturazione` sullo spread che si dimezza ogni 8 turni, fuori dal TPI finché il premio non scende sotto 50 pb.
21. **Acquisti** (Task 10-12). Comando `Compra(Acquisto)`, al massimo uno per tipo a turno, pagato subito dalla cassa del giocatore, rifiutato se la cassa non basta. La spesa di campagna già si taglia in ordine se la cassa cala dopo il comando (`partita.rs:1857-1880`): un acquisto nel turno del voto toglie denaro alla campagna senza codice nuovo, e l'anteprima della campagna lo mostra perché legge la cassa. I risultati valgono per il turno in cui si comprano e stanno nella vista.
22. **Sondaggio privato.** Stesso generatore del pubblico, con campione `moltiplicatore_campione_privato` = 4 volte quello di un istituto ed effetto istituto per `riduzione_istituto_privato` = 0,5, stesso bias comune (spec riga 372). La riduzione dell'effetto istituto non ha fonte (`informazione-a-pagamento.md:38-41`): assunzione del piano 9. Estrazioni con `Caso::estemporaneo` e una chiave propria del turno, che non consuma: comprarlo non rilancia i dadi del sondaggio pubblico né della prima stima (decisione d'esecuzione del 01/11, sotto il Task 10).
23. **Test di una misura.** Sulla stessa bozza dell'anteprima (`ValoreProposto`), dà per ogni condizione la reazione stimata (fascia e segno) con errore `σ = errore_test / √(quota del gruppo)`, letto dall'effetto vero sui gruppi; i gruppi piccoli restano rumorosi (`informazione-a-pagamento.md:54-59`).
24. **Centro studi.** Stima del trimestre chiuso con errore `quota_errore_centro_studi` = 0,6 volte quello della prima stima (assunzione del piano 9, `:98-103`) e previsione a 4 turni di PIL, disoccupazione, inflazione e debito: centro sul percorso deterministico dallo stato vero (misure in vigore e in arrivo, nessuna misura nuova, shock a zero), intervallo ±1σ dalla tabella della Commissione per l'Italia (crescita annua 1,15 → 1,84; inflazione 0,86 → 2,28; disoccupazione 1,06 → 1,47; debito 3 punti a 4 turni; interpolati linearmente fra il turno 1 e il 4, `:75-96`). Nessun bias d'ottimismo: resta in Rimandato.

## Global Constraints

Valgono tutti i vincoli del piano 8b (`2026-10-29-v1-piano-8b-vita-dei-partiti-e-campagna.md:54-68`) e, attraverso quelli, dei piani 1-8a: italiano coi termini di `CONTEXT.md` (evento, situazione, prima stima, anteprima, cassa del partito, previsione; «crisi» resta quella di governo); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano il colpevole; niente `serde(default)`. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- Flussi casuali: `Flusso::Eventi` per deriva delle grandezze, candidati, livelli, bersagli e regioni; `Flusso::Elettorato` mai. Nessun flusso nuovo. Sondaggio privato, test di una misura e centro studi estraggono con la stessa regola dell'anteprima (caso letto, non consumato: `parlamento/anteprima.rs:53-60` riceve `&Caso`).
- Gli eventi del mondo leggono lo stato vero; la vista mostra la causa con valori ufficiali e soglie, mai le probabilità (storia 121, riga 176: si vede quanto una grandezza è vicina alla soglia, non il rischio di scandali ed emergenze).
- Ogni evento cita la sua causa per nome: grandezza, valore e soglia; oppure il livello del rischio (`Poco`/`Medio`/`Molto`), mai la probabilità in cifre.
- Le mosse del giocatore passano da comandi nuovi in `comando.rs`: `RispondiEvento`, `Compra`. Un comando rifiutato non cambia nulla.
- Ogni manopola nuova ha `valore`, `min`, `max`, `ticket` (`15`, `19` o `25`) ed entra in `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:65`) nel task che la introduce, con la fonte o con `# assunzione del piano 9, nessuna fonte in docs/ricerca`.
- Testi di eventi, opzioni e righe del registro elencati nella PR per la revisione di Mario; gli eventi in tono grave senza battute (storia 136, riga 197).
- `VERSIONE_SCHEMA` = valore su `main` + 1, nel Task 1; i campi nuovi dei task successivi entrano senza alzarla.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa. Le schermate (eventi, situazioni, mappa della pericolosità, acquisti) sono del piano 11.
- Verifiche di ogni task: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Partita di prova come nell'8b (`Partita::nuova`: giocatore destra, indice 5, al governo col centrodestra; scadenza 19; turno 0 = ottobre-dicembre 2026). Per mettere la partita nello stato voluto: `edita` (`tests/comune/mod.rs:257`) su `eventi`, `grandezze`, `economia`.

## Review Focus

1. **Determinismo con eventi:** salvare e ricaricare a metà turno con un evento aperto e un'opzione scelta, poi chiudere, dà lo stesso esito che chiudere senza salvare; e cambiare la scelta di un evento non sposta le estrazioni di nessun altro flusso. Test `evento_aperto_sopravvive_al_salvataggio` nel Task 1 e `la_scelta_non_sposta_le_estrazioni` nel Task 5.
2. **Situazione che lampeggia:** una grandezza che oscilla fra spegnimento e accensione non apre due eventi. Test `la_situazione_non_lampeggia` nel Task 4.
3. **Comandi rifiutati** (evento inesistente, opzione sconosciuta, risposta all'opposizione, secondo acquisto dello stesso tipo, acquisto oltre la cassa, acquisto a partita finita): la partita resta identica (`salva` prima e dopo). Test `risposta_rifiutata_non_cambia_nulla` nel Task 1, `acquisto_rifiutato_non_cambia_nulla` nel Task 10.
4. **Fondo vuoto e cassa a zero:** un'emergenza col fondo di riserva esaurito non porta il fondo sotto zero né va in panic; un acquisto nel turno del voto che svuota la cassa lascia la campagna tagliata, mai negativa. Test `il_fondo_non_va_sotto_zero` nel Task 6 e `l_acquisto_toglie_alla_campagna` nel Task 10.
5. **Bersagli che spariscono:** lo scandalo su un ministro che si dimette nella stessa chiusura, o su un leader di un partito che si scioglie, non lascia indici orfani; la crisi del debito non si riapre mentre il MES è in corso. Test `bersaglio_sparito_nessun_panic` nel Task 7 e `una_crisi_del_debito_alla_volta` nel Task 9.

## Dispatch

PR A: 1 → 2 → 3 → 4 in sequenza. L'1 crea tipi e stato che tutti usano; il 2 tocca `parlamento/decreti.rs` e `ministri/`; il 3 tocca `grandezze.rs` e lo scenario; il 4 legge le grandezze del 3.

PR B: dopo la review della A, 5 → 6 → 7 in sequenza: il 6 e il 7 aggiungono candidati al generatore del 5.

PR C: dopo la review della B, 8 → 9: l'8 usa il decreto a debito del 6; il 9 legge lo spread e apre il decreto sulle riforme.

PR D: dopo la review della C, 10 → 11 → 12: tutti toccano `informazione/acquisti.rs` e `comando.rs`.

**Architect (01/11), sequenza e parallelo.** Un solo parallelo regge sui file: **2 ∥ 3** dopo l'1. Il 3 tocca `grandezze.rs`, `grandezze.toml`, `scenario.toml`, `tests/comune/mod.rs` (`senza_shock`) e in `partita.rs` una riga sola, la chiamata di `avanza` (`:408-409`). Il 2 tocca `parlamento/`, `ministri/` e in `partita.rs` `emana_decreto`, `anteprima_decreto` e i cinque `Contesto`. Si fa in due worktree e si fonde il 3 dopo il 2. Il guadagno è piccolo e il 3 aspetta comunque la decisione sull'energia: se quella arriva tardi, il 2 parte da solo e il 3 lo segue senza perdere nulla. Tutto il resto resta in sequenza: 4/5/6/7/9 si contendono `eventi.toml`, `manopole.toml` e la coda di `chiudi_turno`; 10/11/12 `acquisti.rs`, `comando.rs` e `vista.rs`. Dipendenza nuova fra PR: il seam `passo_con` nasce nell'8 e lo usa il 12 (era implicita).

**Architect (01/11), task troppo grandi per una review sola** (proposta, il piano non è riscritto): **Task 2** → 2a decreto su disposizioni innescato (`ammetti_disposizioni`, conversione con valori, anteprima), 2b proposte con innesco evento, evidenza e governo IA. Il 2a è il pezzo che il piano non vedeva. **Task 6** → 6a pericolosità, candidati, gravità e fondo; 6b decreto a debito, solo se la decisione sul doppione `Debito` lo tiene (riga Architect del Task 6). **Task 9** → 9a crisi e programma del MES; 9b ristrutturazione (debito, SFC, redditi, premio sullo spread): sono due meccanismi indipendenti, e un reviewer può accettare l'uno e respingere l'altro. **Task 5** è sottile (una funzione pura e un generatore vuoto); si può fondere nel 6a.

Ogni task va a `rust-engineer`. Il brief nomina `skill-gate`, `caveman` e `ponytail`, e copia la riga **Ricerca** del task. Skill-gate: true per tutti e dodici. Nessun task è meccanico: anche il 5, il più piccolo, fissa una regola di determinismo. A fine PR, review a sei in parallelo: `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer` (legge testi degli eventi, righe del registro e messaggi d'errore elencati nella PR), `rust-reviewer` (il diff tocca `.rs`) e `spec-reviewer` (questo piano), con i findings di quest'ultimo in un report a parte. Poi un solo giro di correzioni. Le decisioni prese in esecuzione si elencano nella PR. Il merge lo decide Mario.

---

## PR A · Eventi (Task 1-4)

### Task 1: Catalogo, stato e risposta agli eventi

**Files:**
- Create: `crates/sim-core/src/eventi/mod.rs` (`mod eventi;` in `lib.rs`), `dati/base/eventi.toml`
- Modify: `crates/sim-core/src/dati.rs:17-61` (`FileDati` con `eventi`, `NOMI` a 11, `leggi`), `:65` (`MANOPOLE_RICHIESTE`), `:375` (il letterale `FileDati` dei test unitari); i letterali dei test: `tests/comune/mod.rs:10-22` (`file_base`) e `:118-131` (`testo_di`, ramo `"eventi"`), `tests/dati.rs:12`, `:127`, `:143`, `:155`, `:166`, `tests/elezioni.rs:132`. Guscio e CLI non cambiano: leggono con `FileDati::leggi` (`app/src-tauri/src/lib.rs:23`, `crates/sim-cli/src/main.rs:72`, `tara.rs:221`).
- Modify: `crates/sim-core/src/partita.rs:41` (`VERSIONE_SCHEMA`), `:87-116` (`Partita.eventi`), `:346` (`chiudi_turno`: risoluzione in testa)
- Modify: `crates/sim-core/src/comando.rs` (`RispondiEvento`), `crates/sim-core/src/vista.rs:21-65` (`Vista.eventi`)
- Test: `crates/sim-core/tests/eventi.rs` (nuovo), unitari in `eventi/mod.rs`

**Ricerca:** `docs/ricerca/sistemi/crisi-ed-eventi.md:16-18` (dilemmi a due opzioni con probabilità dalle cause), `:23` (`timeout_days`: scelta automatica se il giocatore non decide, cioè il default); `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:78-89` (le sei situazioni e le loro misure).

**Dispatch:** `rust-engineer` · primo della PR A · skill-gate: true.

**Architect:** i tre tipi riusati esistono, con due avvertenze. `Fascia3` (`vista.rs:250-257`, `Poco`/`Medio`/`Molto`) deriva solo `Serialize`: dentro `Causa`, che sta nello snapshot, serve anche `Deserialize`, da aggiungere alla derive. `Verso` (`grandezze.rs:25-41`) in `grandezze` vuol dire «il verso in cui la grandezza migliora»; qui vuol dire «il verso in cui si accende»: per le carceri il primo è `Scende` e il secondo `Sale`. Va scritto nel commento di `DatoSituazione.verso`, se no chi legge i dati li confonde. `Forma` (`atti.rs:21-29`) ha anche `Livello`, «solo per le disposizioni»: `valida` lo rifiuta negli effetti di un'opzione. Il salvataggio: le partite caricate passano da `controlla_coerenza` (`partita.rs:3997`); `Eventi` ci entra con indici di evento, opzione, regione e bersaglio dentro i dati e `accese`/`contatori` lunghi quanto le situazioni (la classe di difetti ricorrente del repo è la carica: `91e56ca`). `RispondiEvento` si rifiuta anche a partita finita: gli eventi nati nell'ultima chiusura restano aperti per sempre (`chiudi_turno` esce subito se `finita()`, `partita.rs:347-349`). `EffettoOpzione::Valence { delta }` non dice a chi: la regola implicita (il partito del bersaglio se c'è, altrimenti quello del Presidente del Consiglio) va scritta nel commento della variante.

**Interfaces:**
- Consumes: `Flusso::Eventi` (`caso.rs:13`), finora senza estrazioni.
- Produces:
  - In `eventi/mod.rs`: `pub enum Famiglia { Situazione, Emergenza, Scandalo, Shock, Debito }`; `pub enum Tono { Ordinario, Grave }`; `pub enum EffettoOpzione { Grandezza { grandezza: String, intensita: f64, ritardo: u32, forma: Forma, durata: f64 }, Valence { delta: f64 }, FondoRiserva, Debito, Dimissioni, ClausolaEnergia, Mes, Ristrutturazione }` (serde `tag = "tipo"`, `rename_all = "snake_case"`; `Forma` è quella degli atti); `pub struct DatoOpzione { pub id: String, pub nome: String, pub default: bool, pub effetti: Vec<EffettoOpzione> }`; `pub struct DatoEvento { pub id: String, pub famiglia: Famiglia, pub nome: String, pub gravita: u8, pub dicasteri: Vec<String>, pub decreto: bool, pub debito: bool, pub opzioni: Vec<DatoOpzione> }`; `pub struct DatoSituazione { pub id: String, pub nome: String, pub misura: String, pub verso: Verso, pub accensione: f64, pub spegnimento: f64, pub evento: String, pub spinta: bool }` (`Verso` = quello di `grandezze`: `Sale` accende sopra, `Scende` sotto).
  - `pub enum Causa { Soglia { misura: String, valore: f64, soglia: f64 }, Rischio { livello: Fascia3 }, Shock { serie: String, sigma: f64 }, Spread { pb: i32, soglia: i32 } }`.
  - `pub struct EventoAperto { pub evento: usize, pub turno: u32, pub causa: Causa, pub regione: Option<usize>, pub bersaglio: Option<Bersaglio>, pub tono: Tono, pub costo: i64, pub scelta: Option<usize> }`; `pub enum Bersaglio { Ministro { dicastero: usize }, Leader { partito: usize } }`.
  - `pub struct Eventi { pub aperti: Vec<EventoAperto>, pub accese: Vec<bool>, pub contatori: Vec<u8> }` in `Partita.eventi`; `fn default_di(&self, dati) -> usize`; `pub(crate) fn risolvi(&mut self, ...)` che restituisce gli effetti da applicare per evento (gli effetti `Grandezza` vanno in `grandezze.effetti`; gli altri tipi li applicano i task che li introducono: fino ad allora `valida` li rifiuta nei dati, così il Task 1 consegna solo `Grandezza` e `Valence`).
  - `Comando::RispondiEvento { evento: usize, opzione: String }`: indice in `aperti`. Errori: «nessun evento aperto con indice {n}», «opzione sconosciuta per {nome evento}: {id}», «solo il governo risponde agli eventi».
  - `DatiGioco::eventi()`, `DatiGioco::situazioni()`, `DatiGioco::evento(id) -> Option<usize>`. `valida`: id unici; 1 default esatto; 1-3 opzioni, e 1 solo se l'evento non ha scelta; dicasteri e grandezze noti; `accensione` oltre `spegnimento` nel verso; evento della situazione esistente. Messaggi col nome dell'evento o della situazione.
  - `Vista.eventi: Vec<VoceEvento>` con `nome`, `famiglia`, `causa` (testo), `tono`, `regione`, `bersaglio` (nome), `opzioni` (id, nome, default), `scelta`.
  - `VERSIONE_SCHEMA` = valore su `main` + 1.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/eventi.rs
#[test]
fn senza_scelta_vale_il_default() {
    let mut p = con_evento_di_prova(); // edita: un evento aperto con default "nulla" e opzione "agisci" con un effetto Grandezza
    let prima = grandezza(&p, "sicurezza");
    chiudi_col_bilancio(&mut p);
    assert!(p.vista().eventi.is_empty());
    assert_eq!(grandezza(&p, "sicurezza"), prima); // default senza effetti
}

#[test]
fn la_scelta_applica_l_opzione() { /* RispondiEvento { evento: 0, opzione: "agisci" }: alla chiusura l'effetto entra in grandezze */ }

#[test]
fn risposta_rifiutata_non_cambia_nulla() { /* indice 9, opzione "boh", giocatore all'opposizione: Err con i tre messaggi, salva() identico prima e dopo */ }

#[test]
fn evento_aperto_sopravvive_al_salvataggio() {
    let mut p = con_evento_di_prova();
    p.esegui(Comando::RispondiEvento { evento: 0, opzione: "agisci".into() }).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    chiudi_col_bilancio(&mut p);
    chiudi_col_bilancio(&mut q);
    assert_eq!(p, q);
}
```

In `tests/dati.rs`: `evento_senza_default_rifiutato_col_nome`, `evento_con_due_default_rifiutato_col_nome`, `situazione_con_soglie_rovesciate_rifiutata_col_nome`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test eventi --test dati`
Expected: FAIL, `eventi` e `RispondiEvento` assenti.

- [ ] **Step 3: Implementa tipi, file di dati, stato, comando, risoluzione in testa alla chiusura, vista e schema**

`eventi.toml` nasce con le sei situazioni e i loro sei eventi (scelte 4-5): per ciascuno «Ordinaria amministrazione» (default, nessun effetto) e un'opzione attiva con un effetto `Grandezza` sulla sua misura, `decreto = true` coi dicasteri della misura. Testi nella PR.

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add docs/superpowers/plans/2026-11-01-v1-piano-9-eventi-e-informazione.md docs/ricerca crates dati app
git commit -m "feat(core): catalogo e stato degli eventi, risposta col default"
```

---

### Task 2: Inneschi del decreto e delle proposte, governo IA

**Files:**
- Modify: `crates/sim-core/src/parlamento/decreti.rs:106-131` (`ammetti`: innesco dagli eventi aperti)
- Modify: `crates/sim-core/src/ministri/proposte.rs:19-40` (innesco evento), `crates/sim-core/src/ministri/agenda.rs:570` (`in_evidenza` con evento aperto nel dicastero)
- Modify: `crates/sim-core/src/partita.rs` (chiamate di `ammetti` e `Contesto` con gli eventi aperti)
- Test: `crates/sim-core/tests/decreti.rs`, `crates/sim-core/tests/ministri.rs`, `crates/sim-core/tests/governo_ia.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:12-14` (decreto d'urgenza: presupposti, conversione, perdita di efficacia ex tunc), `:20` (materie escluse e divieto di rinnovare il decreto non convertito).

**Dispatch:** `rust-engineer` · dopo il Task 1 · skill-gate: true.

**Architect:** il commento di `ammetti` (`decreti.rs:106-107`) e il ritorno fisso «decreto senza innesco» (`:129`, non `:130`) vanno sostituiti: l'innesco è l'insieme dei dicasteri degli eventi aperti con `decreto = true` (più tutte le ordinarie con la crisi del debito, Task 9).

**Architect (01/11), premessa incompleta:** togliere il rifiuto non basta, perché a valle il decreto su disposizioni non esiste. `ammetti` restituisce `(da, a, importo)`, la tupla di uno spostamento di fondi (`decreti.rs:113-118`). `emana_decreto` (`partita.rs:1746-1761`) e `anteprima_decreto` (`:1554-1565`) sanno costruire solo `FondiDecreto::Spostati`. `FondiDecreto` (`decreti.rs:33-45`) non ha una variante con valori. Il task deve quindi costruire anche il decreto su disposizioni. La via più corta: un controllo separato, `ammetti_disposizioni(dati, inneschi, valori) -> Result<Vec<(usize, usize)>, Errore>`; `ammetti` dei fondi resta com'è e cambia solo la dispatch in `emana_decreto` e `anteprima_decreto`. La conversione è una `Misura` di forma `Conversione` con `valori` pieni e `fondi: None` (`parlamento/mod.rs:808-823` oggi mette `valori: Vec::new()`; il proponente è il dicastero della prima disposizione). Così l'approvazione passa da `entra_in_vigore` (`partita.rs:848`) come una legge, e la decadenza non applica nulla: «cessano gli effetti normativi» viene gratis. «Nessuna ripetizione» viene dal blocco delle respinte, che copre già `m.valori` delle conversioni (`partita.rs:770`). Il prezzo: il decreto entra alla conversione, nella stessa chiusura, non al comando; nel trimestre è la stessa cosa. Due righe da aggiornare: il commento «Bilancio e conversioni non hanno valori» (`partita.rs:851`), che diventa falso, e la condizione di `leggi_del_giocatore` subito sotto, che conterebbe il decreto convertito come legge del giocatore (decidere e scriverlo nella PR). Le proposte: `TipoProposta` (`ministri/proposte.rs:26-38`) è `Copy` e una variante nuova `Evento { evento }` perderebbe disposizione e gradino. Meglio un campo `evento: Option<usize>` accanto a `sblocca` in `Riforma`, con l'indice del **catalogo** (`DatoEvento`), non di `aperti`: `aperti` si svuota a ogni chiusura e la proposta può restare in sospeso. `in_evidenza` è un campo di `VoceAgenda` (`ministri/agenda.rs:562-571`), cioè dell'atto in agenda: senza agenda non c'è niente da evidenziare. Va bene così, ma il test `evento_nel_dicastero_mette_in_evidenza` deve partire da un dicastero con agenda. `Contesto` si costruisce in cinque punti (`partita.rs:234`, `:500`, `:542`, `:563`; `ministri/agenda.rs:649`): ognuno riceve gli inneschi.

**Interfaces:**
- Consumes: `Eventi.aperti`, `DatoEvento.{decreto, dicasteri}` (Task 1).
- Produces:
  - `pub(crate) struct Inneschi { pub dicasteri: Vec<usize>, pub tutte: bool }` in `eventi/mod.rs`, con `Eventi::inneschi(&self, dati) -> Inneschi`.
  - `ammetti(dati, decaduti, spostabili, inneschi, decreto)`: un `Decreto::Disposizioni` passa se ogni disposizione è ordinaria e il suo dicastero sta in `inneschi.dicasteri` (o `tutte`); errore «decreto senza innesco: nessun evento aperto per {nome disposizione}». Il decreto sui fondi resta com'è.
  - Proposte: innesco «evento nel suo dicastero» (spec storia 30, ticket 04): un ministro senza proposta in sospeso, con un evento aperto nel suo dicastero che ammette il decreto, propone la disposizione del dicastero più vicina alla sua posizione fra quelle che la misura dell'evento migliora (stessa scelta dell'innesco 3, `proposte.rs:120`, ristretta al verso utile); `TipoProposta` guadagna la causa `Evento { evento: usize }`.
  - `in_evidenza`: vero anche con un evento aperto nel dicastero del ministro.

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/decreti.rs`: `decreto_su_disposizione_ammesso_con_evento_aperto` (evento di prova sul dicastero dell'Interno, decreto su una disposizione dell'Interno: Ok), `decreto_fuori_dai_dicasteri_dell_evento_rifiutato` (stesso evento, disposizione della Sanità: Err col nome), `decreto_su_disposizione_senza_evento_rifiutato` (il test di oggi, messaggio nuovo).
`tests/ministri.rs`: `evento_nel_dicastero_da_una_proposta`, `evento_nel_dicastero_mette_in_evidenza`.
`tests/governo_ia.rs`: `il_governo_ia_prende_il_default` (giocatore all'opposizione, evento aperto: `RispondiEvento` rifiutato, alla chiusura vale il default).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test decreti --test ministri --test governo_ia`
Expected: FAIL.

- [ ] **Step 3: Implementa `Inneschi`, il controllo in `ammetti`, l'innesco delle proposte e l'evidenza**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates app/src/lib/tipi
git commit -m "feat(core): l'evento aperto innesca decreto e proposte"
```

---

### Task 3: Grandezze che si muovono da sole, turno zero coi valori veri

**Files:**
- Modify: `crates/sim-core/src/grandezze.rs:141-185` (`avanza` con la deriva, firma con `&mut Caso`), `dati/base/grandezze.toml` (`[[deriva]]`; `immigrazione_irregolare` `max = 600`, `:204-213`)
- Modify: `dati/base/scenario.toml:208-212` (`energia` = 14 con la fonte), `crates/sim-core/src/economia/mod.rs:150` (prezzi del turno zero con lo scarto), `crates/sim-core/tests/economia.rs:13-20` (`senza_shock_i_prezzi_dei_settori_restano_a_uno` con l'energia a 0 via dati, motivo nel commit), `crates/sim-core/src/partita.rs` (chiamata di `avanza`)
- Test: `crates/sim-core/tests/grandezze.rs`, `crates/sim-core/tests/economia.rs`

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:54-60` (carceri: +0,9% a trimestre, σ ~0,75), `:62-68` (energia: TTF ×2,1, Brent 116,8 $; ma `:67-68` avverte che lo scarto del motore è un'altra unità e che lo shock in corso sta già nell'inflazione), `:70-76` (sbarchi: σ log 0,50 trimestrale con stagionalità, 0,74 annua; 2016 = 475), `:91-95` (deriva e rumore delle tre grandezze fisse).

**Dispatch:** `rust-engineer` · dopo il Task 1 (non dipende dai tipi del 2; vedi «Dispatch») · skill-gate: true · energia decisa da Mario il 01/11 (decisione 5, scelta 9).

**Architect:** oggi `Dinamica::Fissa` riporta la base a `iniziale` a ogni chiusura (`grandezze.rs:152-155`): per le grandezze con una `[[deriva]]` la base evolve invece dal valore precedente, ed è già nello snapshot (`basi`). Le estrazioni avvengono anche nelle partite di prova esistenti: i test che fissano valori di quelle tre grandezze dopo una chiusura vanno aggiornati, con il motivo nel commit.

**Architect (01/11), energia a 80, da decidere prima del dispatch:** il test `l_energia_rientra` quasi certamente fallisce sull'inflazione, e il motivo si legge nel codice. (1) `Economia::iniziale` mette `energia = 80` ma calcola `prezzi_settori` con 0 (`economia/mod.rs:148-152`): al turno zero lo scarto non è nei prezzi, e i test del turno zero passano per costruzione. (2) Al primo passo `energia ← 0,7·80 + 3z ≈ 56` (`mod.rs:226`, `persistenza_energia` 0,7 in `manopole.toml:114-118`), e i prezzi saltano tutti in un trimestre da 0 a 56 (`mod.rs:255-257`). (3) `energia` scala **tutti** gli input importati (`mod.rs:376-384`, vettore `importazioni` di `economia.toml:37`), non solo l'energia. Il contenuto d'importazione del paniere, pesato con `consumi` (`:40`), è circa 7% diretto e 10-12% con Leontief (stima a mano, non misurata). (4) `inflazione_importata = 400·(ΔP/P)` entra nella Phillips con coefficiente 1 (`congiuntura.rs:35`), e `phillips_1` = 1,18 (`manopole.toml:44-45`) la trascina nei trimestri dopo. Ordine di grandezza: circa +25 punti d'inflazione annualizzata nel primo trimestre, e un IPCA del primo anno sopra il 15%, contro la soglia del 6% del test. La ricerca lo dice già (`situazioni-ed-eventi-al-turno-zero.md:67-68`): la sigma del motore «è un'altra unità». La media TTF/Brent (×2,1, ×1,52) è un prezzo dell'energia, mentre `energia` è lo scarto di tutte le importazioni. Altre due rotture prevedibili: `senza_shock_i_prezzi_dei_settori_restano_a_uno` (`tests/economia.rs:13-20`) fallisce, perché `senza_shock` azzera solo le `sigma_` e lo scarto di partenza resta. E dopo il rientro la soglia di 50 non si raggiunge più (deviazione stazionaria `3/√(1−0,49)` ≈ 4,2): la crisi energetica esiste solo al turno zero e con lo shock oltre 2,5σ del Task 8. Strade, la scelta è di Mario: (a) riscalare lo scarto sulla quota dell'energia nelle importazioni (circa 80 × 0,15-0,2 ≈ 12-16) e le soglie con lui; (b) tenere `energia` a 0 e accendere la situazione per scenario, contro la decisione 1; (c) mettere lo scarto anche nei prezzi del turno zero (`prezzi_settori(dati, s.energia)`), così il primo passo dà un rientro, cioè inflazione importata negativa, e l'inflazione 4,2 dello scenario regge come «shock già nei prezzi». **Risolta il 01/11:** strade (a) e (c) insieme, decisione 5 di Mario.

**Architect (01/11), firme e test:** `avanza` con `caso: &mut Caso` chiude la strada al Task 12, che fa avanzare copie senza toccare il caso. Firma più economica: `caso: Option<&mut Caso>`, `None` = deriva senza rumore; lo stesso parametro serve al passo del motore (Task 8). La deriva lavora sulla base, che oggi per le `Fissa` non ha clamp (`grandezze.rs:152-155`): la base va limitata a `[min, max]` dopo il passo, se no l'indice lineare delle carceri cresce senza tetto sotto il valore che resta a 200. `senza_shock` (`tests/comune/mod.rs:38-61`) azzera solo le manopole `sigma_`: le `sigma` di `[[deriva]]` stanno in `grandezze.toml` e restano. Si estende `senza_shock` alle righe `sigma = ` di `grandezze.toml`, così i test deterministici esistenti restano deterministici.

**Interfaces:**
- Consumes: nulla di nuovo.
- Produces:
  - `pub struct DatoDeriva { pub grandezza: String, pub tasso: f64, pub sigma: f64, pub ritorno: f64, pub logaritmica: bool }`; `DatiGioco::derive()`. `valida`: grandezza nota e `fissa`, sigma ≥ 0, ritorno in 0-1; errore col nome.
  - `Grandezze::avanza(&mut self, dati, caso: &mut Caso, e: &Economia, turno: u32)`: una normale da `Flusso::Eventi` per deriva, nell'ordine della lista, a ogni chiamata (scelta 8).
  - Scenario: `energia.valore = 14.0`, `fonte = "situazioni-ed-eventi-al-turno-zero:62-68, scarto dell'energia +80% per la quota dell'energia nelle importazioni (0,15-0,2), assunzione del piano 9; scostamento % del costo delle importazioni"`. `Economia::iniziale`: `prezzi_settori(dati, s.energia.valore)`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/grandezze.rs
#[test]
fn le_carceri_crescono_da_sole() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..20 { chiudi_col_bilancio(&mut p); }
    let v = grandezza(&p, "sovraffollamento_carceri");
    assert!(v > 112.0 && v < 125.0, "{v}"); // 1,009^20 ≈ 1,196, meno il rumore
}

#[test]
fn gli_sbarchi_tornano_verso_cento() { /* 200 semi, dopo 40 chiusure la mediana sta fra 60 e 160 e nessun valore oltre 600 */ }

#[test]
fn la_deriva_non_tocca_gli_altri_flussi() { /* stesso seme: il sondaggio del turno 1 è identico con e senza le righe [[deriva]] (dati edit) */ }
```

`tests/economia.rs`: `il_turno_zero_resta_tarato` (il test di scenario esistente su debito, spread, inflazione 4,2 e margine passa con l'energia a 14 nei prezzi) e `l_energia_rientra` (con `senza_shock`, dopo 4 chiusure `energia < 5`, la situazione è spenta e l'inflazione IPCA del primo anno sta fra 0 e il 4,2% di partenza: il rientro abbassa i prezzi). Se `l_energia_rientra` fallisce, fermarsi e chiedere a Mario (scelta 9).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test grandezze --test economia`
Expected: FAIL su `le_carceri_crescono_da_sole`.

- [ ] **Step 3: Implementa `[[deriva]]`, `avanza` col caso, scala degli sbarchi ed energia di partenza**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati
git commit -m "feat(core): carceri, sbarchi e corruzione si muovono da soli; energia vera al turno zero"
```

---

### Task 4: Le sei situazioni

**Files:**
- Create: `crates/sim-core/src/eventi/situazioni.rs`
- Modify: `crates/sim-core/src/partita.rs` (`chiudi_turno`: aggiornamento dopo il motore, nascita degli eventi in coda; `costruisci`: accese al turno zero senza evento; `pub fn situazioni`)
- Modify: `crates/sim-core/src/elettorato/mod.rs` (`spingi_a_destra`, chiamato prima dell'ancora), `crates/sim-core/src/vista.rs` (`Vista.situazioni`)
- Modify: `dati/base/eventi.toml` (soglie della scelta 5), `dati/base/manopole.toml` (`spinta_situazione`)
- Test: `crates/sim-core/tests/situazioni.rs` (nuovo)

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:21-37` (recessione tecnica, sei episodi dal 2000), `:39-52` (criminalità e liste d'attesa), `:78-89` (tabella delle sei: picchi e indice a 100 = oggi); `docs/ricerca/sistemi/crisi-ed-eventi.md:10-13` (isteresi alla Democracy 4).

**Dispatch:** `rust-engineer` · dopo il Task 3 · skill-gate: true.

**Architect (01/11):** tre premesse da correggere. (1) Il valore nazionale di una grandezza regionale esiste già, `Grandezze::nazionale` (`grandezze.rs:187-190`, `media` a `:245-252`), ed è pesato sulle **forze di lavoro**, non sulla popolazione. Si riusa quello, e l'interfaccia sotto va letta così. (2) La crescita vera del trimestre esiste già: `informazione.statistica.vera.crescita_pil`, annualizzata (`informazione/statistica.rs:87-88`), calcolata dopo il motore. Per la recessione conta il segno, quindi niente calcolo nuovo; la prima stima della vista è `statistica.prima_stima.crescita_pil`. (3) Il test `due_trimestri_in_calo_accendono_la_recessione` non può «editare due shock di domanda»: le innovazioni non sono nello stato (lo diventano solo al Task 8, e il passo le riscrive). `aggiorna(dati, eventi, misure)` è pura: la recessione si prova lì, con un vettore di misure scritto a mano; l'integrazione resta sui test delle carceri. Posizione nella chiusura: l'aggiornamento (b) va subito dopo `statistica.aggiorna` (`partita.rs:432-438`), in coda al blocco del motore. La spinta va prima di `elettorato.posizioni` (`partita.rs:423`; `posizioni` sta in `elettorato/voto.rs:184`, non in `mod.rs`), quindi legge le situazioni accese alla chiusura **precedente**: è voluto, va scritto nel commento d'ordine di `chiudi_turno`.

**Interfaces:**
- Consumes: `Eventi.{accese, contatori}`, `DatoSituazione` (Task 1); `Grandezze` con deriva (Task 3).
- Produces:
  - `pub(crate) fn aggiorna(dati, eventi: &mut Eventi, misure: &[f64]) -> Vec<usize>` in `situazioni.rs`: le situazioni appena accese. Una misura per situazione, nell'ordine dei dati: valore della grandezza nazionale (media delle regioni pesata con la popolazione se regionale), `crescita_pil` o `energia`. Recessione: contatore dei trimestri di segno coerente (scelta 4).
  - La nascita in coda alla chiusura apre l'evento della situazione appena accesa con `Causa::Soglia { misura, valore, soglia: accensione }`, fuori dal tetto.
  - `Elettorato::spingi_a_destra(&mut self, quota: f64)`: per ogni gruppo, da 6 a 1 sui punti, `quota` della massa del punto k passa al k+1.
  - `pub fn situazioni(&self) -> Vec<VoceSituazione>` su `Partita` e `Vista.situazioni`: `nome`, `misura`, `valore` (ufficiale: per le grandezze di servizio quello col turno di ritardo, per `crescita_pil` la prima stima), `accensione`, `spegnimento`, `accesa`.
  - Manopola `spinta_situazione` 0,002 (0-0,01, ticket `15`, assunzione del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/situazioni.rs
#[test]
fn la_crisi_energetica_e_accesa_al_turno_zero_senza_evento() {
    let p = Partita::nuova(dati_base(), 1);
    assert!(p.situazioni().iter().any(|s| s.misura == "energia" && s.accesa));
    assert!(p.vista().eventi.is_empty());
}

#[test]
fn le_carceri_oltre_la_soglia_aprono_un_evento() { /* edita la base a 118: dopo una chiusura accesa e un evento con causa Soglia { misura "sovraffollamento_carceri", soglia 117 } */ }

#[test]
fn la_situazione_non_lampeggia() { /* 118 → 113 → 118: un solo evento, sempre accesa; sotto 110 si spegne */ }

#[test]
fn due_trimestri_in_calo_accendono_la_recessione() { /* edita due shock di domanda negativi: accesa al secondo trimestre negativo, non al primo */ }

#[test]
fn gli_sbarchi_spingono_a_destra() { /* sbarchi accesi: dopo una chiusura la posizione media di ogni gruppo è più a destra che con gli sbarchi spenti, a parità di seme */ }
```

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test situazioni`
Expected: FAIL.

- [ ] **Step 3: Implementa `aggiorna`, la nascita in coda, le accese al turno zero, la spinta e la vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): sei situazioni a soglia con isteresi"
```

---

## PR B · Emergenze e scandali (Task 5-7)

### Task 5: Rischio e tetto degli eventi puntuali

**Files:**
- Create: `crates/sim-core/src/eventi/rischio.rs`
- Modify: `crates/sim-core/src/partita.rs` (nascita in coda: dopo le situazioni, il generatore dei puntuali)
- Test: `crates/sim-core/tests/eventi.rs`

**Ricerca:** `docs/ricerca/sistemi/crisi-ed-eventi.md:20-21` (trigger con tempo mediano alla Paradox), `:36` (eventi puntuali con M funzione dello stato), `:40` (niente regista adattivo, tetto di un puntuale a turno).

**Dispatch:** `rust-engineer` · primo della PR B · skill-gate: true.

**Architect (01/11):** con il generatore vuoto, `al_massimo_un_puntuale_per_turno` non ha da dove prendere i «candidati di prova» in una partita intera: nessun seam li inietta, e aggiungerne uno solo per il test è un seam ipotetico. Il tetto si prova sui test unitari di `estrai`, che è pura; il test d'integrazione passa al Task 6, con la pericolosità alzata via dati. La scelta 10 («le estrazioni non dipendono dagli esiti») va tenuta anche per gli shock del Task 8: i candidati di shock sono sempre cinque, con `p` 0 o 1, così il numero di uniformi estratte resta fisso.

**Interfaces:**
- Consumes: `Eventi`, `EventoAperto`, `Causa::Rischio` (Task 1).
- Produces:
  - `pub(crate) struct Candidato { pub evento: usize, pub p: f64, pub gravita: u8, pub regione: Option<usize>, pub bersaglio: Option<Bersaglio>, pub causa: Causa, pub costo: i64, pub tono: Tono }`.
  - `pub(crate) fn estrai(caso: &mut Caso, candidati: &[Candidato]) -> (Option<usize>, Vec<usize>)` in `rischio.rs`: una uniforme da `Flusso::Eventi` per candidato, sempre, nell'ordine; restituisce il vincitore (gravità più alta, a parità il primo) e gli altri usciti. Pura, testata da sola.
  - `fn livello(p: f64) -> Fascia3`: `Poco` sotto 0,05, `Medio` sotto 0,2, `Molto` oltre. È il «livello di rischio» citato nella causa.
  - Il generatore in `Partita` raccoglie i candidati delle famiglie che i Task 6-8 aggiungono; in questo task la lista è vuota tranne nei test.

- [ ] **Step 1: Scrivi i test che falliscono**

Nel modulo di test di `rischio.rs`: `vince_il_piu_grave`, `a_parita_vince_il_primo`, `estrae_sempre_tutti` (tre candidati con p = 1, 0, 1: il caso avanza di tre estrazioni in ogni caso). In `tests/eventi.rs`: `al_massimo_un_puntuale_per_turno` (100 semi × 20 chiusure con candidati di prova a p alta: mai due puntuali nati nello stesso turno) e `la_scelta_non_sposta_le_estrazioni` (stesso seme, due scelte diverse sullo stesso evento: sondaggi e candidati estratti al turno dopo identici).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core rischio && cargo test -p sim-core --test eventi`
Expected: FAIL.

- [ ] **Step 3: Implementa `Candidato`, `estrai`, `livello` e il generatore**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates
git commit -m "feat(core): rischio degli eventi puntuali e tetto di uno a turno"
```

---

### Task 6: Emergenze naturali, fondo di riserva e decreto a debito

**Files:**
- Create: `crates/sim-core/src/eventi/emergenze.rs`
- Modify: `dati/base/regioni.toml` (`pericolosita = { sisma, idro, incendi }` per regione), `crates/sim-core/src/struttura.rs` o il tipo della regione (campo e validazione), `dati/base/eventi.toml` (quattro emergenze con livelli, opzioni fondo/debito/nulla), `dati/base/manopole.toml`
- Modify: `crates/sim-core/src/economia/mod.rs:78-80`, `:240-250` (`Economia.decreti` per fondo e debito; `Economia.fondo_speso`, azzerato al cambio d'anno)
- Modify: `crates/sim-core/src/vista.rs` (`VoceRegione.pericolosita` o equivalente; `Vista.eventi` col tono)
- Test: `crates/sim-core/tests/emergenze.rs` (nuovo), `crates/sim-core/tests/decreti.rs`

**Ricerca:** `docs/ricerca/dati/pericolosita-naturale-regioni.md:42-86` (tabella per regione: sisma a trimestre, peso idro, incendi), `:87-100` (costi: 4-6 miliardi l'anno, ~10 miliardi a episodio sismico), `:102-116` (k = 0,85 per il sisma, DM 58/2017), `:124` (incendi nel terzo trimestre), `:135` (il fondo copre la prima emergenza, mai la ricostruzione); `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:116-134` (siccità come emergenza regionale, prima tranche del Fondo 10-40 milioni, Emilia-Romagna 4,7 miliardi per decreto).

**Dispatch:** `rust-engineer` · dopo il Task 5 · skill-gate: true.

**Architect:** il fondo di riserva oggi è uno stanziamento (`economia/finanza.rs:140`, `fondo_riserva` da manopola) con il suo peso sul tetto UE, che nessuno spende (piano 5 `:1398`). Va verificato come entra nel saldo prima di decidere se la spesa dal fondo è domanda nuova o riallocazione: la regola del piano è che spendere il fondo non aggiunge disavanzo oltre lo stanziato, il debito sì.

**Architect (01/11), verificato, la regola sopra non regge:** lo stanziamento non entra mai nel saldo. La spesa del trimestre è `spesa_totale / 4 + Σ decreti` (`economia/finanza.rs:242`), senza `fondo_riserva`; la spesa netta del controllo UE si calcola da quei conti (`economia/regole_ue.rs:68-84`). Il fondo pesa solo nel riepilogo della legge di bilancio, come spesa annua contro il tetto (`economia/legge_bilancio.rs:403`), cioè sul tetto **previsto**, non su quello controllato. Ne segue che spendere dal fondo è domanda nuova e disavanzo nuovo, esattamente come il debito, e conta nella spesa netta vera. Il fondo dà due cose sole: un tetto alla spesa senza passare dall'aula, e il fatto di essere già stato messo in conto nel tetto del bilancio. La via più corta per tutte e due le spese è il vettore che esiste già: `Economia.decreti`, euro per dicastero spesi per intero nel trimestre, fuori dal bilancio annuo, già in domanda finale col vettore del dicastero e già a disavanzo (`economia/mod.rs:78-80`, `:240-250`). Quindi `misure.rs` non si tocca. La quota regionale va come effetto `Grandezza` su `domanda_regionale` della regione, il canale che il danno usa già. **Doppione da sciogliere (decisione):** `EffettoOpzione::Debito` (risolto in testa alla chiusura, senza aula) e `Decreto::Debito` + `FondiDecreto::Debito` (comando, poi conversione in aula) fanno la stessa cosa per due strade. Se l'opzione «Decreto a debito» spende da sola, il comando e la variante di conversione sono codice in più; se invece deve passare dall'aula, l'opzione si riduce a «apre il decreto», cioè `debito = true` sull'evento, e l'effetto `Debito` non serve. Una delle due, non entrambe. Altri tre punti. `VoceRegione` non esiste (`vista.rs` non ha voci per regione): la pericolosità è un dato statico e la sua mappa è del piano 11, quindi la vista della pericolosità si rimanda e qui resta solo `Vista.eventi` col tono. `fondo_residuo` dentro `Bilancio` finirebbe anche nella legge di bilancio proposta e nei tipi esportati (`finanza.rs:29-37`, `legge_bilancio.rs:357`): basta `Economia.fondo_speso: i64`, azzerato al cambio d'anno, con residuo = `bilancio.fondo_riserva − fondo_speso`. Il danno si applica alla nascita (coda della chiusura), il ripristino alla risoluzione (testa della successiva): se tutti e due stanno alla risoluzione, «Nessun intervento» e «Fondo» si confrontano su un danno che nessuno ha visto.

**Decisione d'esecuzione (01/11, thread principale):** resta solo l'effetto `Debito` dell'opzione, che spende da `Economia.decreti` senza aula; niente `Decreto::Debito` né `FondiDecreto::Debito`. Il decreto dell'evento resta quello sulle disposizioni (Task 2). `fondo_speso` su `Economia`, vista della pericolosità al piano 11, danno alla nascita e ripristino alla risoluzione, come sopra. `ponytail:` la spesa a debito dell'opzione non passa dalla conversione; se il bilanciamento vuole il rischio d'aula, si torna al comando.

**Interfaces:**
- Consumes: `Candidato`, `estrai` (Task 5); `EffettoOpzione::{FondoRiserva, Debito}` (Task 1, qui ammessi da `valida`).
- Produces:
  - `pub struct Pericolosita { pub sisma: f64, pub idro: f64, pub incendi: f64 }` per regione (probabilità trimestrali, peso idro come quota nazionale); `valida`: in 0-1, pesi idro che sommano a 1 ± 0,01.
  - `pub(crate) fn candidati(dati, data: Trimestre, grandezze: &Grandezze, caso: &mut Caso) -> Vec<Candidato>` in `emergenze.rs` (scelte 14-15): per regione e tipo, con il livello estratto da `Flusso::Eventi` per ogni candidato (sempre); siccità dopo i tre tipi.
  - Effetto `Debito` dell'opzione «Decreto a debito»: il costo dell'evento entra in `Economia.decreti` sul primo dicastero dell'evento, speso per intero nel turno, a disavanzo; la quota regionale come effetto `Grandezza` su `domanda_regionale`. Effetto `FondoRiserva`: stessa strada, fino al residuo `bilancio.fondo_riserva − fondo_speso`.
  - `Bilancio.fondo_residuo: i64` (stanziamento a inizio anno, cala con le emergenze, mai sotto zero).
  - Manopole (ticket `19`): `sisma_minimo` 0,001, `emergenze_idro_anno` 4 (`pericolosita-naturale-regioni.md:123`, valore iniziale proposto; la frequenza vera è [NON TROVATO] a `:32`: va marcata come manopola da tarare), `siccita_anno` 0,4, `k_prevenzione_sisma` 0,85, `k_prevenzione_altre` 0,5, `costo_emergenza_lieve` 1e8, `costo_emergenza_grave` 1e9, `costo_emergenza_vittime` 5e9, `valence_emergenza_risposta` 0,02, `valence_emergenza_ignorata` 0,05 (le ultime due e `siccita_anno` assunzioni del piano 9).
  - Opzioni delle emergenze: «Fondo di riserva» (default, `FondoRiserva` + ripristino), «Decreto a debito» (`Debito` del costo + ripristino pieno), «Nessun intervento» (`Valence` negativa). Con vittime `tono = Grave`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// tests/emergenze.rs
#[test]
fn gli_incendi_cadono_d_estate() { /* 300 semi × 8 chiusure: ogni emergenza d'incendio nata ha la data nel terzo trimestre */ }

#[test]
fn la_prevenzione_riduce_il_danno() { /* stessa emergenza di prova in una regione con stock 100 e con stock 200: danno sullo stock del secondo = (1 − 0,85) del primo per il sisma */ }

#[test]
fn il_fondo_paga_e_non_va_sotto_zero() { /* fondo residuo 50 milioni, emergenza grave (1 miliardo) col default: residuo 0, ripristino 5%, nessun panic; seconda emergenza: spesa 0 */ }

#[test]
fn con_vittime_il_tono_e_grave() { /* livello vittime: VoceEvento.tono == Grave */ }
```

`tests/emergenze.rs`: `il_decreto_a_debito_alza_il_disavanzo` (opzione «Decreto a debito»: il disavanzo del turno sale del costo entro l'arrotondamento, il fondo residuo non cambia).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test emergenze --test decreti`
Expected: FAIL.

- [ ] **Step 3: Implementa pericolosità, candidati, gravità e prevenzione, fondo residuo, decreto a debito, opzioni e vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): emergenze naturali per regione, fondo di riserva e decreto a debito"
```

---

### Task 7: Scandali

**Files:**
- Modify: `crates/sim-core/src/eventi/rischio.rs` (candidato scandalo), `dati/base/eventi.toml` (scandalo su un ministro, scandalo su un leader), `dati/base/manopole.toml`
- Modify: `crates/sim-core/src/ministri/mod.rs:107` (`Ministro.sotto_scandalo: u32`), `crates/sim-core/src/partita/ia.rs:578-592` (`individuale`: prima il ministro sotto scandalo), `crates/sim-core/src/partita.rs` (`EffettoOpzione::Dimissioni` con la regola di `ChiediDimissioni`)
- Test: `crates/sim-core/tests/scandali.rs` (nuovo), `crates/sim-core/tests/opposizione_ia.rs`

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:97-114` (otto ministri dimessi 2008-2026, 0,11 a trimestre; sfiducia individuale: 1 approvata su 39 votate; legame col CPI non trovato).

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true.

**Architect (01/11):** `MinistroInCarica` non esiste: il tipo è `Ministro` (`ministri/mod.rs:106-116`), dentro `StatoDicastero.ministro: Option<Ministro>` (`:132-134`). Il campo va lì, e `Ministro` è anche il tipo delle rose: un candidato in rosa ha `sotto_scandalo` = 0. `ChiediDimissioni` non è una funzione sola: il comando passa da `ministri.comanda` (`ministri/mod.rs:400-425`, che toglie il ministro e può rifiutare) e poi da `dimissioni_chieste` (`partita.rs:1235-1245`, il colpo all'intesa se il ministro è di un alleato e non è tecnico; `comando.rs:281-302`). L'effetto `Dimissioni` riusa le due chiamate. Se `comanda` rifiuta (`ammesso`, `ministri/mod.rs:417-422`), l'evento si chiude senza effetto con una riga nel registro, come per il bersaglio sparito; mai un `Err` che esca dalla chiusura. Risolto in testa alla chiusura, il dicastero resta in interim per tutta la chiusura: il giocatore nomina al turno dopo. Va scritto nella PR. La sfiducia IA: `individuale` (`partita/ia.rs:576-592`) prende **un** ministro, quello col giudizio peggiore, e controlla solo su lui che la sfiducia passi (`motivo(Some(i))?`). «Prima il ministro sotto scandalo» quindi è una chiave in più in testa al `min_by` (`sotto_scandalo > 0` prima del giudizio), non un secondo giro.

**Interfaces:**
- Consumes: `Candidato`, `Bersaglio`, `EffettoOpzione::{Valence, Dimissioni}` (Task 1, 5).
- Produces:
  - Candidato scandalo (scelta 18): `p = scandalo_base × corruzione / 100`, bersaglio estratto da `Flusso::Eventi` (sempre, anche se lo scandalo non esce) fra ministri in carica, in ordine di dicastero, poi leader dei partiti IA attivi, in ordine d'indice.
  - `Ministro.sotto_scandalo: u32` (`ministri/mod.rs:107`; turni che restano; 0 = no), che cala a ogni chiusura.
  - Nella risoluzione: bersaglio che non c'è più (ministro dimesso, partito sciolto) → l'evento si chiude senza effetti, una riga nel registro.
  - Manopole (ticket `15`): `scandalo_base` 0,11 (fonte `:97-114`), `valence_scandalo` 0,05, `turni_scandalo` 4 (assunzioni del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/scandali.rs`: `difenderlo_costa_valence` (default: valence del partito del ministro −0,05 alla chiusura, `sotto_scandalo` = 4), `le_dimissioni_seguono_la_regola_del_piano_5` (opzione dimissioni su un tecnico: nessun colpo all'intesa; su un politico: colpo come `ChiediDimissioni`), `mai_il_giocatore` (1.000 estrazioni con la corruzione a 200: nessun bersaglio è il giocatore), `bersaglio_sparito_nessun_panic`, `scandalo_del_leader_ia_senza_scelta` (una sola opzione, `RispondiEvento` con un altro id rifiutato).
`tests/opposizione_ia.rs`: `la_sfiducia_individuale_va_al_ministro_sotto_scandalo` (con la sfiducia che passerebbe su entrambi, l'IA sceglie quello sotto scandalo anche se non ha il giudizio peggiore).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test scandali --test opposizione_ia`
Expected: FAIL.

- [ ] **Step 3: Implementa il candidato, lo stato del ministro, la risoluzione e il bersaglio della sfiducia IA**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): scandali su ministri e leader, bersaglio della sfiducia individuale"
```

---

## PR C · Shock e crisi del debito (Task 8-9)

### Task 8: Shock economici oltre 2,5σ

**Files:**
- Modify: `crates/sim-core/src/economia/mod.rs:58`, `:219-224` (`Economia.innovazioni: [f64; 5]`, salvate nel passo), `crates/sim-core/src/economia/regole_ue.rs:150-158` (attivazione della clausola per l'energia)
- Modify: `crates/sim-core/src/eventi/rischio.rs` (candidati shock), `dati/base/eventi.toml` (cinque eventi di shock), `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/shock.rs` (nuovo)

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:146-154` (2σ = 0,91 eventi a legislatura per serie; 2,5σ = 1,24 sulle cinque; circa 1 episodio vero a legislatura).

**Dispatch:** `rust-engineer` · primo della PR C · skill-gate: true.

**Architect (01/11):** (1) Il test `shock_oltre_soglia_apre_un_evento` non può «editare le innovazioni»: il passo le riscrive nella stessa chiusura, prima della nascita (`economia/mod.rs:219-224`), quindi l'edit si perde. I candidati di shock si calcolano con una funzione pura `candidati_shock(innovazioni: &[f64; 5], soglia) -> [Candidato; 5]`, sempre cinque con `p` 0 o 1 (vedi Task 5), e la soglia si prova lì. (2) Questo è il task che tocca il passo, quindi qui si apre anche il seam che serve al Task 12: `passo` estrae le cinque normali e chiama `passo_con(dati, data, misure, z: [f64; 5])`, che le salva in `innovazioni` e fa il resto. Il centro studi chiama `passo_con` con `[0.0; 5]`. Due adapter (partita e previsione), quindi un seam vero. (3) `ClausolaEnergia` non ha una «regola esistente» da attivare. `clausola_nazionale_fino_a` è un dato dello scenario, immutabile (`scenario.toml:7`, 2028; letto in `regole_ue.rs:150-154`), e il margine `clausola_nazionale_margine` = 0,6 **contiene già** 0,3 per l'energia (`scenario.toml:198-202`). Serve uno stato nuovo in `RegoleUe` (l'anno fino a cui vale la clausola per l'energia), letto in `controllo_annuale`. Fino al 2028 la clausola energetica è già dentro quella nazionale: attivarla nel 2027-2028 conterebbe due volte lo 0,3. Proposta: vale solo per gli anni dopo `clausola_nazionale_fino_a`, margine 0,3 dalla stessa fonte; altrimenti l'opzione nel 2027-2028 non fa nulla e lo dice. Da confermare in review.

**Interfaces:**
- Consumes: `Candidato` (Task 5), `EffettoOpzione::Debito` (Task 6), `EffettoOpzione::ClausolaEnergia` (Task 1).
- Produces:
  - `Economia.innovazioni: [f64; 5]`: le normali estratte nel passo, prima di moltiplicarle per la sigma, nell'ordine domanda, produttività, energia, domanda estera, BCE. Nessuna estrazione nuova.
  - Candidati shock (scelta 19): `p = 1` per ogni serie con innovazione sfavorevole oltre `soglia_shock`, `Causa::Shock { serie, sigma: |z| }`; quelli favorevoli una riga del registro («Shock favorevole: …»). Lo shock escluso dal tetto va nel registro col default.
  - Dicasteri coinvolti: Sviluppo economico (domanda, produttività, domanda estera), Ambiente ed energia (energia), Economia e finanze (BCE).
  - `ClausolaEnergia`: attiva la clausola nazionale per l'energia per l'anno in corso, secondo la regola esistente di `regole_ue.rs` (`clausola_nazionale_fino_a`).
  - Manopole (ticket `15`): `soglia_shock` 2,5 (decisione 3), `aiuti_shock` 0,003 del PIL annuo (assunzione del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/shock.rs`: `shock_oltre_soglia_apre_un_evento` (edita le innovazioni del motore con una domanda a −3: candidato uscito, causa con sigma 3), `shock_sotto_soglia_niente` (−2,4: nessun evento), `shock_favorevole_solo_nel_registro`, `frequenza_vicina_alla_storia` (500 semi × 20 chiusure senza altri candidati: media degli eventi di shock fra 0,3 e 1,0 a legislatura, il lato sfavorevole di 1,24), `la_clausola_per_l_energia_allarga_il_margine`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test shock`
Expected: FAIL.

- [ ] **Step 3: Implementa le innovazioni salvate, i candidati, gli eventi e la clausola**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): shock economici oltre 2,5 sigma come eventi"
```

---

### Task 9: Crisi del debito, MES e ristrutturazione

**Files:**
- Modify: `crates/sim-core/src/economia/mod.rs:82-100` (`programma_mes`, `ristrutturazione`), `crates/sim-core/src/economia/spread.rs:24-35` (`spread_dallo_stato`: premio della ristrutturazione), `:37-47` (`aggiorna_spread`: TPI e OMT col MES), `crates/sim-core/src/economia/sfc.rs:25-57` (stock dopo il taglio), `crates/sim-core/src/elettorato/redditi.rs` (perdita delle fasce media e alta)
- Modify: `crates/sim-core/src/eventi/mod.rs` (`Inneschi.tutte` con la crisi aperta), `crates/sim-core/src/partita.rs` (nascita fuori dal tetto, effetti `Mes` e `Ristrutturazione`), `dati/base/eventi.toml` (evento «Aste scoperte» e condizionalità), `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/debito.rs` (nuovo)

**Ricerca:** `docs/ricerca/sintesi/situazioni-ed-eventi-al-turno-zero.md:136-145` (nessuna asta scoperta nel 2011; PCCL ed ECCL; PSI greco: 53,5% nominale, adesione 96,9%); `docs/ricerca/modelli/economia-italiana-parametri.md:27` (massimo di novembre 2011: 5,5 punti), `:46-48` (TPI e OMT, OMT solo con un programma del MES).

**Dispatch:** `rust-engineer` · dopo il Task 8 · skill-gate: true.

**Architect (01/11):** `spread.rs` ha 48 righe. Il `:117-118` della riga **Files** originale era il rimando alla ricerca nel commento di testa (`spread.rs:2`), non codice; corretto sopra. Premessa che manca e che rompe la carica: il debito è anche uno stock della contabilità SFC. Al turno zero lo Stato lo deve tutto alle famiglie (`economia/sfc.rs:33-45`), e un salvataggio si rifiuta se `sfc.stock.stato != −debito` o se la somma degli stock non è zero (`partita.rs:4089-4095`; test `tests/conti.rs:183-196`). `debito × (1 − taglio)` da solo produce un salvataggio illeggibile. Il taglio va applicato insieme a `debito`, `sfc.stock.stato` (+taglio) e allo stock di chi detiene il debito (`famiglie`, e `estero` se nel frattempo ne ha), nella stessa proporzione. Così la «perdita delle fasce media e alta» ha anche un appiglio contabile: nel modello i detentori sono le famiglie. Oggi `premio_sfiducia` lo dà solo lo scenario (`[premio_sfiducia]`, 0 in `scenario.toml`; azzerato col TPI già al turno zero, `economia/mod.rs:191-194`) e lo azzera il TPI se la PDE non è inadempiente (`spread.rs:41-45`): «premio azzerato col MES» vale già in quasi tutti gli stati. Il pezzo nuovo vero è l'OMT fuori dai criteri del TPI, cioè il ramo inadempiente.

**Interfaces:**
- Consumes: `Inneschi` (Task 2), `Causa::Spread` (Task 1).
- Produces:
  - Nascita (scelta 20): se `spread ≥ soglia_crisi_debito`, nessuna crisi aperta e nessun programma in corso, l'evento «Aste scoperte» con `Causa::Spread`, fuori dal tetto.
  - `Economia.programma_mes: Option<u32>` (turno di fine) e `Economia.ristrutturazione: Option<Ristrutturazione { turno: u32, premio: f64 }>`.
  - `Mes`: valori delle disposizioni della condizionalità (in `eventi.toml`, `[[condizionalita]]` disposizione e gradino; scritti nel task: età pensionabile e indicizzazione delle pensioni al gradino più restrittivo, più un tetto alla crescita dei budget dei dicasteri; testi e scelte nella PR) che entrano subito come valori in vigore della legge «Memorandum del MES»; `−intesa_mes` a ogni alleato; con il programma in corso il premio di sfiducia è zero e l'OMT vale anche fuori dai criteri del TPI.
  - `Ristrutturazione`: `debito × (1 − taglio_ristrutturazione)`; reddito delle fasce media e alta `× (1 − perdita_ristrutturazione)` per `turni_ristrutturazione`; premio sullo spread `premio_ristrutturazione` che si dimezza ogni 8 turni; fuori dal TPI finché il premio supera 50 pb.
  - Con l'evento aperto `Inneschi.tutte = true`.
  - Manopole (ticket `07`/`15`): `soglia_crisi_debito` 550 pb (fonte `economia-italiana-parametri.md:27`), `durata_mes` 8 (fonte `:136-145`), `intesa_mes` 10, `taglio_ristrutturazione` 0,5 (fonte `:136-145`), `perdita_ristrutturazione` 0,05, `turni_ristrutturazione` 8, `premio_ristrutturazione` 300 pb (le ultime quattro tranne il taglio: assunzioni del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/debito.rs`: `spread_oltre_soglia_apre_le_aste_scoperte`, `il_mes_e_il_default` (alla chiusura senza scelta: disposizioni della condizionalità in vigore, premio di sfiducia 0, intesa degli alleati in calo), `la_ristrutturazione_taglia_il_debito` (debito dopo = metà entro l'arrotondamento; spread col premio; reddito della fascia alta in calo), `una_crisi_del_debito_alla_volta` (programma in corso e spread ancora sopra soglia: nessun evento nuovo), `con_la_crisi_aperta_il_decreto_apre_le_riforme` (decreto su una disposizione di qualunque dicastero: Ok).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test debito`
Expected: FAIL.

- [ ] **Step 3: Implementa l'innesco, il programma del MES, la ristrutturazione e l'innesco delle riforme**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): crisi del debito con MES o ristrutturazione"
```

---

## PR D · Informazione (Task 10-12)

### Task 10: Acquisti con la cassa e sondaggio privato

**Files:**
- Create: `crates/sim-core/src/informazione/acquisti.rs`
- Modify: `crates/sim-core/src/comando.rs` (`Compra`), `crates/sim-core/src/partita.rs` (`acquisti` del turno, azzerati in chiusura), `crates/sim-core/src/informazione/sondaggio.rs:77` (`rileva` con campione e riduzione d'istituto), `crates/sim-core/src/vista.rs` (`Vista.acquisti`)
- Modify: `dati/base/manopole.toml`, `crates/sim-core/src/dati.rs:65`
- Test: `crates/sim-core/tests/acquisti.rs` (nuovo)

**Ricerca:** `docs/ricerca/modelli/informazione-a-pagamento.md:21-41` (prezzo: 10-20 mila € per 1.000-1.500 interviste; nessuna fonte per la riduzione dell'effetto istituto), `:116-125` (prezzi raccomandati sulla scala della cassa).

**Dispatch:** `rust-engineer` · primo della PR D · skill-gate: true.

**Architect (01/11):** il sondaggio privato estratto da `Flusso::Sondaggi` **al comando**, a metà turno, fa avanzare lo stesso flusso da cui alla chiusura escono il sondaggio pubblico (`informazione/sondaggio.rs:93`) e la prima stima (`informazione/statistica.rs:89-90`). Comprarlo cambia quindi il rumore del sondaggio pubblico e della statistica del turno: è un modo di rilanciare i dadi, a pagamento. Il salvataggio resta deterministico, perché il caso sta nello snapshot, ma il vincolo «cambiare una scelta non sposta le estrazioni di altri flussi» del Review Focus 1 qui cade. Soluzione dentro le regole esistenti: l'estrattore estemporaneo che non consuma (`caso.rs:76-84`, `Caso::estemporaneo(chiave)`, quello dell'anteprima), con una chiave propria del turno. Il vincolo globale «`Flusso::Sondaggi` per il sondaggio privato» va riletto così; è una decisione di Mario (vedi report). `rileva` (`sondaggio.rs:77-85`) prende `caso: &mut Caso` e fa tutti gli istituti più gli incroci: per il privato con la chiave estemporanea serve un `z` passato da fuori. `campiona` lo prende già come `&mut impl FnMut() -> f64` (`sondaggio.rs:55-61`), quindi il cambio è una riga di firma.

**Decisione d'esecuzione (01/11, thread principale):** `Caso::estemporaneo` con una chiave propria (turno, `"sondaggio_privato"`); `rileva` riceve lo `z` da fuori. Il vincolo globale è aggiornato.

**Interfaces:**
- Consumes: `Partito.cassa` (8b), `spendi_campagne` (`partita.rs:1857-1880`).
- Produces:
  - `pub enum Acquisto { SondaggioPrivato, TestMisura { valori: Vec<ValoreProposto> }, CentroStudi }` in `acquisti.rs`; `Comando::Compra(Acquisto)`. Errori: «già comprato questo turno: {tipo}», «cassa insufficiente: servono {euro}», «partita finita».
  - `Partita.acquisti: Vec<Acquistato>` del turno, con il risultato; azzerati a ogni chiusura; nello snapshot.
  - `sondaggio::rileva(..., campione: f64, riduzione_istituto: f64)`: il pubblico passa 1 e 1; il privato `moltiplicatore_campione_privato` e `riduzione_istituto_privato`.
  - `Vista.acquisti: VistaAcquisti { sondaggio_privato: Option<Sondaggio>, test: Option<VistaTest>, centro_studi: Option<VistaCentroStudi>, prezzi: [i64; 3] }` (`VistaTest` e `VistaCentroStudi` nei Task 11-12).
  - Manopole (ticket `25`): `prezzo_sondaggio_privato` 15.000 (fonte `:21-41`, decisione 4), `prezzo_test_misura` 20.000 (`:43-59`), `prezzo_centro_studi` 150.000 (`:61-73`), `moltiplicatore_campione_privato` 4, `riduzione_istituto_privato` 0,5 (assunzioni del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/acquisti.rs`: `il_sondaggio_privato_costa_e_sbaglia_meno` (cassa −15.000; su 200 semi l'errore medio assoluto del privato sulle intenzioni vere è minore di quello del pubblico, e il bias comune è lo stesso), `acquisto_rifiutato_non_cambia_nulla` (secondo acquisto, cassa insufficiente, partita finita: Err, `salva` identico), `l_acquisto_toglie_alla_campagna` (turno del voto, campagna assegnata pari alla cassa, poi `Compra`: alla chiusura la campagna spende cassa − 15.000, mai negativa), `gli_acquisti_finiscono_col_turno`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test acquisti`
Expected: FAIL.

- [ ] **Step 3: Implementa `Acquisto`, `Compra`, il sondaggio privato e la vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): acquisti con la cassa del partito, sondaggio privato"
```

---

### Task 11: Test di una misura

**Files:**
- Modify: `crates/sim-core/src/informazione/acquisti.rs` (`VistaTest`), `crates/sim-core/src/parlamento/anteprima.rs:53-133` (effetto vero per condizione, letto e non mostrato), `crates/sim-core/src/partita.rs:1520` (`anteprima` riusata dal test)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/acquisti.rs`

**Ricerca:** `docs/ricerca/modelli/informazione-a-pagamento.md:43-59` (message testing; ±13,9 punti con 100 per braccio, ±14 per un gruppo al 10% su 2.000 interviste; effetti poco eterogenei fra gruppi, Coppock 2018).

**Dispatch:** `rust-engineer` · dopo il Task 10 · skill-gate: true.

**Architect (01/11):** «estratta come nell'anteprima» vuol dire `caso.estemporaneo(chiave)` (`parlamento/anteprima.rs:50-52`, `caso.rs:76-84`), che non consuma. La chiave però deve essere diversa da quella dell'anteprima (turno, disposizione, gradino): con la stessa chiave l'errore del test coinciderebbe col rumore di `errore_fascia` dell'anteprima gratuita, e il test non direbbe niente di nuovo. Basta un offset fisso nella chiave, scritto in un commento.

**Interfaces:**
- Consumes: `Acquisto::TestMisura` (Task 10), la bozza dell'anteprima (`parlamento.bozza`, `partita.rs:1523`).
- Produces:
  - `pub struct VistaTest { pub misura: String, pub reazioni: Vec<ReazioneStimata> }`; `pub struct ReazioneStimata { pub condizione: Condizione, pub segno: i8, pub fascia: Fascia3 }`.
  - La reazione vera di una condizione è la variazione dell'utilità media dei suoi gruppi verso il governo dovuta alla misura (stessi termini della linea, letti dallo stato vero); quella stimata le aggiunge una normale con `σ = errore_test / √(quota della condizione)`, estratta come nell'anteprima (scelta 23). Mai il valore in cifre nella vista.
  - Rifiuto come l'anteprima (stessi errori) prima di pagare.
  - Manopola `errore_test` 0,05 (ticket `25`, fonte `:54-59` per la forma, valore assunzione del piano 9).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/acquisti.rs`: `il_test_indovina_il_segno_dei_gruppi_grandi` (misura che colpisce i pensionati: su 200 semi il segno stimato è giusto in almeno il 90% per la condizione più grande, e meno spesso per la più piccola), `test_su_misura_non_ammessa_rifiutato_senza_pagare`.

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test acquisti`
Expected: FAIL.

- [ ] **Step 3: Implementa reazione vera, stima e vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): test di una misura sui gruppi colpiti"
```

---

### Task 12: Centro studi

**Files:**
- Modify: `crates/sim-core/src/informazione/acquisti.rs` (`VistaCentroStudi`), `crates/sim-core/src/informazione/statistica.rs:79` (stima migliore del trimestre chiuso)
- Modify: `crates/sim-core/src/economia/mod.rs` (passo senza shock per la previsione, sulla copia), `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/acquisti.rs`

**Ricerca:** `docs/ricerca/modelli/informazione-a-pagamento.md:61-73` (budget dei centri studi, Magna Carta 325 mila € nel 2024), `:75-96` (RMSE della Commissione per l'Italia: PIL 1,15 → 1,84, inflazione 0,86 → 2,28, disoccupazione 1,06 → 1,47, saldo 1,27 → 1,63; sono errori sulla crescita annua: per la trimestrale annualizzata `:123` dà circa 2 → 3), `:98-103` (revisione della prima stima: 0,271 punti trimestrali; nessuna fonte su un previsore che batte la prima stima).

**Dispatch:** `rust-engineer` · dopo il Task 11 · skill-gate: true.

**Architect:** la previsione gira su copie di `Economia`, `Grandezze`, `Misure` e `Disposizioni` con i valori in arrivo, senza toccare il caso della partita: il passo del motore estrae gli shock da `Caso` (`economia/mod.rs:219`), quindi serve un passo con gli shock a zero (parametro o variante), non un `Caso` copiato. Il costo per acquisto è quattro passi del motore: sotto i 2 ms per turno del bersaglio.

**Architect (01/11):** verificato, il passo prende `caso: &mut Caso` e ne estrae solo le cinque normali (`economia/mod.rs:208-224`). Il seam si apre al Task 8 (`passo_con(…, z: [f64; 5])`), qui si chiama con `[0.0; 5]`. Lo stesso vale per `Grandezze::avanza`, che dal Task 3 estrae la deriva: con `caso: Option<&mut Caso>` la previsione passa `None`. Senza questi due seam il task deve toccare di nuovo motore e grandezze. Ogni passo della previsione rifà `Misure::da` (`partita.rs:395-404`) sulle copie, e prima gli arrivi delle disposizioni (`Disposizioni::arrivi`, `partita.rs:477`); niente atti dei ministri («nessuna misura nuova»). Unità della forchetta del PIL: le RMSE 1,15 → 1,84 sono sulla crescita **annua**; per la crescita trimestrale annualizzata la ricerca dà circa 2 → 3 punti, piatta oltre h = 2 (`informazione-a-pagamento.md:123`). `VocePrevisione.pil` va definita (crescita sull'anno mobile, per stare con le RMSE scelte) e scritta nel commento del campo. Il test `senza_shock_la_previsione_e_esatta` con il solo `senza_shock` non è esatto: alla chiusura vera i ministri eseguono atti e la deriva estrae ancora. Servono `ferma_tutti` (`tests/comune/mod.rs:248`), la `sigma` della deriva a zero (Task 3) e nessun evento che spenda. `ponytail:` `Forchetta { centro, meno, piu }` è simmetrica per costruzione (±1σ): basta `{ centro, sigma }`.

**Interfaces:**
- Consumes: `Acquisto::CentroStudi` (Task 10).
- Produces:
  - `pub struct VistaCentroStudi { pub stima: Stima, pub previsione: Vec<VocePrevisione> }`; `pub struct VocePrevisione { pub data: Trimestre, pub pil: Forchetta, pub disoccupazione: Forchetta, pub inflazione: Forchetta, pub debito: Forchetta }`; `pub struct Forchetta { pub centro: f64, pub meno: f64, pub piu: f64 }`.
  - Stima del trimestre chiuso: valore vero + errore della prima stima già estratto × `quota_errore_centro_studi` (nessuna estrazione nuova: la stima migliore è una frazione dello stesso errore, così prima stima e stima del centro studi non si contraddicono a caso).
  - Previsione (scelta 24): quattro turni, centro deterministico, ±1σ interpolato fra le manopole `rmse_<grandezza>_1` e `rmse_<grandezza>_4`.
  - Manopole (ticket `25`): `quota_errore_centro_studi` 0,6 (assunzione), `rmse_pil_1` 1,15, `rmse_pil_4` 1,84, `rmse_inflazione_1` 0,86, `rmse_inflazione_4` 2,28, `rmse_disoccupazione_1` 1,06, `rmse_disoccupazione_4` 1,47 (fonte `:75-96`), `rmse_debito_1` 1,0, `rmse_debito_4` 3,0 ([NON TROVATO] in fonte, `:87`, assunzione).

- [ ] **Step 1: Scrivi i test che falliscono**

`tests/acquisti.rs`: `la_stima_del_centro_studi_e_piu_vicina` (|stima − vero| = 0,6 × |prima stima − vero| esatto), `la_forchetta_si_allarga` (per ogni grandezza `piu − meno` cresce dal turno 1 al 4), `la_previsione_non_tocca_la_partita` (`salva` prima e dopo l'acquisto identico tranne cassa e acquisti; la chiusura dopo dà lo stesso esito di una partita senza acquisto), `senza_shock_la_previsione_e_esatta` (manopole sigma a zero via dati: il centro del turno 1 coincide col valore vero alla chiusura).

- [ ] **Step 2: Esegui e verifica che falliscano**

Run: `cargo test -p sim-core --test acquisti`
Expected: FAIL.

- [ ] **Step 3: Implementa la stima, il passo senza shock, la previsione e la vista**

- [ ] **Step 4: Esegui e verifica che passino**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates dati app/src/lib/tipi
git commit -m "feat(core): centro studi con stima e previsione a quattro turni"
```

---

## Rimandato

- **Piano 10 (scenario):** l'avviso dell'editor «situazione o crisi già accesa al turno zero» (storia 6) legge `Partita::situazioni`; i cursori che spostano l'energia o le carceri ne cambiano l'accensione.
- **Piano 11 (UI):** schermate degli eventi in apertura (tono grave senza testata), soglie accanto alle grandezze, livello «pericolosità naturale» della mappa e segni delle emergenze aperte (storie 137, 139), acquisti.
- **Piano 12 (bilanciamento):** tutte le manopole nuove, in particolare soglie delle situazioni, `spinta_situazione`, deriva e rumore degli sbarchi, rischio degli scandali, costi delle emergenze, valence della gestione, prezzi dell'informazione; e il confronto fra 2,5σ e code grasse con persistenza (decisione 3).
- **Ottimismo del centro studi** (`informazione-a-pagamento.md:116-125`): un errore medio non nullo come quello dei previsori veri, se il bilanciamento lo chiede.
- **Detentori del debito italiano** ([NON TROVATO], `situazioni-ed-eventi-al-turno-zero.md:145`): la ripartizione vera della perdita della ristrutturazione fra le fasce.
- **Regista adattivo ed epidemia (SIR):** fuori dalla v1 (spec righe 464-465).
