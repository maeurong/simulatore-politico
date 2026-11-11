---
tipo: sintesi
fonti: []
aggiornato: 2026-11-10
---
# Lo stato che il core ha già, voce per voce, per le esposizioni nella Vista

Per ogni dato che il piano «vista completa» vuole portare nella `Vista`: dove vive nello stato vero, con che tipo, come si calcola, chi lo legge oggi e se manca un pezzo da conservare nello snapshot. Serve a scrivere il piano senza riaprire il codice: tre voci chiedono stato nuovo (e quindi un salto di `VERSIONE_SCHEMA`), le altre sono letture o derivazioni di stato che c'è.

Provenienza: cwd `/Users/mario/GitHub/simulatore-politico/.claude/worktrees/piano-esito-vista-completa-316e4f`, branch `docs/piano-esito-vista-completa`, HEAD `8952a06` (= `origin/main`) [M: `git -C <worktree> log -1 --format='%H %s'`, `git -C <worktree> branch --show-current`]. Fonti: solo codice e documenti del repo, citati `file:riga`; i percorsi senza prefisso stanno sotto `crates/sim-core/src/`. Nessun `cargo` lanciato: niente è stato compilato né eseguito.

Etichette: [V] letto sul file alla riga citata; [M] misurato in sessione, col comando; [INF] deduzione da ciò che è stato letto.

## Regole comuni a tutte le voci

- **Lo snapshot è la struct `Partita`** (`partita.rs:97-135`), serializzata intera; `VERSIONE_SCHEMA = 15` (`partita.rs:52`) [V]. `carica` rifiuta ogni schema diverso con un confronto di uguaglianza (`partita.rs:4497-4502`) [V]: un campo aggiunto a una struct dello stato cambia la forma dello snapshot e chiede il salto a 16; senza salto, un salvataggio vecchio fallirebbe come `SalvataggioIlleggibile` invece che come `SchemaDiverso` [INF]. Un solo salto copre tutte le voci dello stesso piano [INF]. Precedente: la costante è cambiata nella PR A di ogni piano [M: `git -C <worktree> log --oneline -8 -G"VERSIONE_SCHEMA: u32 = " -- crates/sim-core/src/partita.rs`].
- **Peso**: lo snapshot sta intorno ai 640 KB in chiaro e 170 KB gzip, e pesa quasi tutto l'elettorato (391 KB) (`docs/ricerca/tecnica/persistenza-dei-salvataggi.md:78-80`) [V]. Tutto lo stato nuovo di questa pagina è nell'ordine di pochi KB: vettori di 20 regioni e, per ogni voto, 20 regioni × al più 10 partiti [INF].
- **L'albo non è toccato**: `ElezioneAlbo` è un tipo suo, copiato campo per campo da `EsitoElezioni` (`salvataggi.rs:70-80`, `partita.rs:2626-2637`) [V]. Aggiungere campi a `EsitoElezioni` non cambia il formato dell'albo, che ha regole proprie (`docs/ricerca/tecnica/persistenza-dei-salvataggi.md:94`) [INF].
- **Ritardo di un turno, com'è fatto oggi**: `Grandezze::avanza` copia `prima ← valori` e poi calcola i valori nuovi (`grandezze.rs:174`); la `Vista` legge `grandezze.prima` (`partita.rs:3978-3985`, commento a `vista.rs:38`) [V]. Dopo la chiusura del turno t il giocatore vede quindi i valori della chiusura t−1 [INF].
- **La `Vista` non ha un elenco delle regioni.** Le chiavi regionali oggi sono due: l'indice nell'ordine dei dati (`VoceGrandezza.valori`, `vista.rs:496-497`) e l'id in `Sondaggio.regioni[].id` (`informazione/sondaggio.rs:229-239`) [V]. `VoceEvento.regione` è il **nome**, non l'id (`vista.rs:137-138`, `partita.rs:4136-4139`) [V]. Nome, macroarea e pericolosità naturale stanno in `Regione` (`struttura.rs:38-49`), nei dati, mai nella `Vista` [V]. La UI oggi non nomina le regioni in nessun file [M: `grep -rnE "regioni|regione" app/src --include='*.svelte' --include='*.ts' --exclude-dir=tipi -l` → nessun risultato].
- **Il guscio Tauri chiama solo** `vista`, `comanda`, `chiudi_turno`, `anteprima`, `anteprima_decreto`, `anteprima_accogli`, `eredita`, `abbandona`, `salva`, `testata`, `voce_albo` [M: `grep -rnE "partita\.[a-z_]+\(|p\.[a-z_]+\(" app/src-tauri/src -o -h`]. Ogni altra funzione pubblica del core (`elettori_campione`, `anteprima_campagna`, `anteprima_sfiducia`, `riepilogo_bilancio`, `proiezione_seggi`) non arriva alla UI se non passa dalla `Vista` o da un comando nuovo del guscio [INF].

## 1. Disoccupazione e reddito per regione

**Disoccupazione.**
- Stato: `Economia.regioni: Vec<EconomiaRegione>` (`economia/mod.rs:56`), con `disoccupazione: f64`, `disoccupazione_strutturale: f64`, `gap: f64` per regione, nell'ordine dei dati (`economia/mod.rs:35-41`) [V]. La scrive `Economia::okun` a ogni passo (`economia/lavoro.rs:45-63`) [V].
- La nazionale è la media pesata sulle forze di lavoro: `Economia::disoccupazione_nazionale` → `media_regionale` (`economia/lavoro.rs:12-30`) [V].
- **Il valore della chiusura precedente non è conservato da nessuna parte.** `Elettorato.ultima_disoccupazione: Vec<f64>` (`elettorato/mod.rs:380-381`) è il valore del turno prima solo *dentro* la chiusura: `giudizio` lo confronta con quello nuovo (`elettorato/giudizio.rs:126`), poi `composizione` lo sovrascrive con quello corrente (`elettorato/composizione.rs:153`; l'ordine è scritto a `partita.rs:529`) [V]. A chiusura finita coincide con `economia.regioni[r].disoccupazione` [INF]. Per mostrarla «esatta con un turno di ritardo» serve conservare un vettore di 20 valori: **stato nuovo** [INF].
- Come arriva oggi la nazionale: `Statistica` (`informazione/statistica.rs:54-62`) tiene `prima_stima`, `vera` e `revisione`. `aggiorna` (`:79-116`) calcola `vera` dallo stato vero, la `prima_stima` come `vera + errore_stima_disoccupazione · z` (`:93`), e mette nella `revisione` la prima stima del trimestre prima accanto alla sua `riveduta`, che è il valore vero (`:111-115`, commento a `:43`) [V]. La `Vista` porta `prima_stima` e `revisione`, mai `vera` (`partita.rs:3995-3999`, `vista.rs:474-481`) [V].
- Coerenza col ritardo [INF]: la media pesata sulle forze di lavoro delle disoccupazioni regionali della chiusura t−1 è esattamente `revisione.riveduta.disoccupazione`, già pubblica. Mostrare invece i valori regionali correnti rivelerebbe `vera`, cioè scavalcherebbe l'errore della prima stima.
- Chi la legge oggi: `Stima::vera` (`informazione/statistica.rs:25`), `spesa_netta_trimestre` (`economia/regole_ue.rs:74-75`), `giudizio` e `composizione` (sopra), `termini_regioni` per il bisogno della regione bersaglio (`ministri/agenda.rs:119-125`), `indicatori` per l'eredità (`partita.rs:466-474`) [V].

**Reddito.** Non esiste una grandezza «reddito per regione»: va derivata dai gruppi [V: nessun campo regionale in `Economia` (`economia/mod.rs:44-112`) né in `Elettorato` (`elettorato/mod.rs:375-384`)].
- Stato: `Gruppo.reddito: Reddito` e `Gruppo.reddito_prima: Reddito` (`elettorato/mod.rs:366-367`), con `Reddito { primario, trasferimenti, imposte, paniere }` in euro annui nominali per elettore, `netto() = primario + trasferimenti − imposte`, `reale() = netto / paniere` (`elettorato/redditi.rs:10-29`) [V]. I gruppi di una regione sono i 18 consecutivi `r·18 .. (r+1)·18` (`elettorato/mod.rs:94-104`) e le loro `quota` sommano a 1 (`elettorato/mod.rs:504-511`) [V].
- Come nasce: `aggiorna_redditi` copia `reddito_prima ← reddito` (`elettorato/redditi.rs:109`) e ricalcola ogni gruppo dai dati del gruppo (`dato.primario`, `dato.trasferimenti`) scalati con indici **nazionali** (salario, salari pubblici, PIL, pensione, sussidio; `:111-125`) [V]. Il reddito regionale si muove quindi solo per gli indici nazionali e per la composizione dei gruppi, non per l'economia della regione [INF].
- Candidati, tutti somme sui 18 gruppi della regione pesate con `quota` [INF]:
  - A. reale per elettore: `Σ quota_i · reddito_i.reale()` (euro annui a prezzi del turno zero, perché `paniere` vale 1 al turno zero, `elettorato/redditi.rs:10-13`);
  - B. netto nominale per elettore: `Σ quota_i · reddito_i.netto()`;
  - C. reale per fascia: `Σ_c quota_{c,f} · reale_{c,f} / Σ_c quota_{c,f}`, tre valori per regione;
  - D. crescita reale trimestrale: A calcolato su `reddito` diviso A calcolato su `reddito_prima`, meno 1 (per gruppo è la formula di `elettorato/giudizio.rs:87`, e le ultime crescite stanno in `Gruppo.crescite`, `elettorato/mod.rs:371-372`);
  - E. primario lordo per elettore: `Σ quota_i · reddito_i.primario`.
- Valore della chiusura precedente: `reddito_prima` c'è, per gruppo [V]. Non c'è la `quota` della chiusura precedente: A-E «in ritardo» con i pesi di oggi non chiedono stato nuovo; con i pesi di allora sì [INF].
- Nessuna funzione aggrega oggi per regione. Leggono il reddito dei gruppi: `Elettorato::gini` (`elettorato/redditi.rs:181-201`, pesi `elettori`), `mobilita` (`elettorato/composizione.rs:167-173`), `giudizio` (`elettorato/giudizio.rs:77-124`) [V]. Nella `Vista` il reddito non compare [M: `grep -n reddito crates/sim-core/src/vista.rs` → nessun risultato, letto il file intero].

## 2. Voto e seggi per regione dell'ultimo voto

- `EsitoElezioni` (`legislatura/voto.rs:23-33`) conserva: `data`, `affluenza` nazionale, `voti: Vec<VotiPartito>` (`partito`, `quota` fra i votanti, `camera`, `senato` dopo le cessioni; `:13-21`), `premio: Option<String>`, `coalizioni: Vec<Vec<String>>` [V]. Sta in `Legislatura.elezioni` (`legislatura/mod.rs:69`), il più recente per primo, mai accorciato [M: `grep -rnE "\.elezioni\b" crates/sim-core/src` → un solo `insert(0, …)` a `partita.rs:2872`, nessun `truncate`/`retain`], ed è già nella `Vista` per intero (`partita.rs:4053`, `vista.rs:80-81`) [V].
- **I voti per regione si calcolano e si buttano.** `vota_parlamento` chiama `legislatura::voti_per_regione` (`partita.rs:2810`), che rende `BTreeMap<String, Vec<u64>>`: id della regione → voti per partito, senza astensione (`legislatura/voto.rs:52-73`) [V]. Ne ricava i totali per partito (`partita.rs:2811-2814`) e poi cede la mappa a `riparto_con` (`partita.rs:2816`) [V].
- **I seggi del Senato per regione si calcolano e si buttano.** `elezioni::riparto` rende `Esito` (`elezioni/riparto.rs:44-53`) con `senato_per_regione: BTreeMap<String, Vec<u32>>`, «seggi per lista in ogni regione, premio ed estero esclusi» (`:48-49`, riempita a `:604-650`) [V]. `vota_parlamento` usa solo i totali di `camera` e `senato` e il `premio` (`partita.rs:2817-2832`); `senato_per_regione`, `tetto_camera`, `tetto_senato` cadono [V].
- Invariante: per ogni lista, la somma sulle regioni di `senato_per_regione` è `proporzionali + speciali` (`crates/sim-core/tests/elezioni.rs:512-523`) [V]. Non comprende premio ed estero, e viene prima delle cessioni (`legislatura::cedi`, `partita.rs:2821`): la somma per regione non coincide in generale con `VotiPartito.senato` [INF].
- **La Camera per regione non esiste per lista.** Il riparto della Camera è nazionale (`elezioni/riparto.rs:548-584`); per circoscrizione esiste solo il numero di seggi dalla popolazione, `seggi_per_circoscrizione` → `SeggiCircoscrizione { id, regione, seggi, premio, uninominali }` (`elezioni/seggi.rs:5-12`, `:55-101`), senza le liste [V]. I seggi delle circoscrizioni speciali (Valle d'Aosta, Trentino-Alto Adige: `dati/base/elezioni.toml:147-157`, `:171-192`) confluiscono nel totale `speciali` della lista (`elezioni/riparto.rs:542-547`) [V].
- Affluenza per regione: non si calcola; si ricava dalle stesse intenzioni (`vero.per_gruppo`, ultima voce = astensione) e dagli elettori dei gruppi [INF da `legislatura/voto.rs:63-68`].
- Cosa andrebbe conservato [INF]: voti per regione e partito (da cui il primo partito), seggi del Senato per regione e partito, eventualmente l'affluenza per regione; il posto naturale è l'esito stesso, che è già stato, `Vista` e tipo esportato. **Stato nuovo.**
- Sulla proiezione corrente: `proiezione_seggi` (`partita.rs:2765-2774`) rende lo stesso `Esito`, con `senato_per_regione` calcolato sul sondaggio pubblico; la `Vista` ne tiene solo i totali (`partita.rs:3945-3954`) [V]. Seggi del Senato per regione «in proiezione» non chiedono stato [INF].

## 3. Gruppi della scheda della regione ed elettori campione

- Quota: `Gruppo.quota: f64` (`elettorato/mod.rs:359`), quota della regione, 18 per regione che sommano a 1; indice canonico `regione·18 + condizione·3 + fascia` (`indice_gruppo`, `elettorato/mod.rs:101-104`) [V]. Elettori del gruppo: `Elettorato::elettori` = quota × elettori della regione (`elettorato/mod.rs:387-390`) [V].
- Cambia nel tempo. `composizione` (`elettorato/composizione.rs:147-154`) gira a ogni chiusura: `lavoro` sposta quota fra privati, autonomi e disoccupati con la variazione della disoccupazione (`:156-165`, masse a `:16-43`), `mobilita` sposta fra fasce secondo la crescita del reddito (`:167-212`), `eta` solo a gennaio (pensionamenti, uscite, studenti; `:214-254`) [V]. La quota del turno zero resta nei dati (`DatoGruppo.quota`, `elettorato/mod.rs:143-144`) [V].
- È stato vero. Oggi ne trapela solo un aggregato: il `campione` di ogni incrocio del sondaggio è proporzionale agli elettori veri del sottoinsieme (`informazione/sondaggio.rs:186-187`) [V]. La quota per regione e gruppo non è esposta [INF].
- **`elettori_campione`**: `pub fn elettori_campione(&self, regione: &str, c: Condizione, f: Fascia) -> Result<Vec<ElettoreCampione>, Errore>` (`partita.rs:2548-2572`); `Errore::Elettorato` se la regione non c'è [V]. Rende tre elettori (`NUMERO = 3`, `informazione/campione.rs:14`) di tipo `ElettoreCampione { nome, eta: u8, posizione: u8, voto: Option<String>, motivo: Motivo }` (`informazione/campione.rs:26-35`; `voto` è l'id del partito, `None` l'astensione; `Motivo` = vicinanza, leader, governo, protesta, `:16-24`) [V].
- Legge solo il sondaggio pubblico, le `VocePartito` e i dati, mai lo stato vero (`informazione/campione.rs:1-2`, `:73-81`) [V]: non espone nulla che non sia già nella `Vista`, salvo l'estrazione. L'estrattore è estemporaneo, con chiave `turno · 1024 + indice del gruppo` (`partita.rs:2561-2562`): stessi tre elettori per tutto il turno, nessun flusso del `Caso` avanza [V].
- Non arriva alla UI: non è nella `Vista` e il guscio non la chiama [M: `grep -rn elettori_campione app/src-tauri/src` → nessun risultato]. `ElettoreCampione` e `Motivo` sono già esportati in TypeScript; `Fascia` (`elettorato/mod.rs:72-78`) no, a differenza di `Condizione` (`:35-37`) [V; M: `ls app/src/lib/tipi` → c'è `Fascia3.ts`, non `Fascia.ts`].
- Stato nuovo: nessuno.

## 4. Cantieri finiti e atti regionali eseguiti

- Stato: `StatoDicastero.cantieri: Vec<Cantiere>` (`ministri/mod.rs:147`), `Cantiere { atto: usize, regione: Option<usize>, turni_mancanti: u32, rata: i64, speso: i64 }` (`ministri/agenda.rs:43-51`) [V]. La `Vista` li porta in `VoceMinistero.cantieri` tramite `voci_governo` (`ministri/agenda.rs:611-619`), e solo quelli in corso [V].
- Dove finisce: in `Ministri::esegui`, primo passo della chiusura (`partita.rs:570`). Le rate si pagano e `turni_mancanti` scende (`ministri/agenda.rs:336-342`); per i cantieri a zero si registrano gli effetti (`:343-346`); `st.cantieri.retain(|f| f.turni_mancanti > 0)` li toglie (`:347`) [V].
- Cosa si sa in quel punto [V]: l'atto (`f.atto`, indice di catalogo: id, nome, dicastero), la regione (`f.regione`), lo speso totale con l'ultima rata (`f.speso`), la rata, il dicastero (`i`), la capacità del ministro (`c`), il turno e la data della chiusura (`cx.turno`, `cx.data`).
- Dopo il `retain` non resta traccia: gli effetti entrano in `grandezze.effetti` come `EffettoAttivo` (`grandezze.rs:83-97`), che ha dicastero e, solo per le grandezze regionali, regione (`ministri/agenda.rs:185-189`), ma né l'atto né il turno d'esecuzione (`inizio` è turno + ritardo, `:193`) [V]. **Stato nuovo.**
- Un cantiere fermato esce da un'altra porta: `ferma_cantiere` lo rimuove (`ministri/agenda.rs:491`) [V]. Non è un cantiere finito [INF].
- Cantieri con regione: solo i dicasteri con `regione_bersaglio`, cioè Sviluppo economico e Infrastrutture e Trasporti (`dati/base/dicasteri.toml:18-22`, `:62-66`); i loro atti a più turni sono `sviluppo_economico_distretti` (4), `infrastrutture_trasporti_ferrovie` (5), `infrastrutture_trasporti_grandi_opere` (8) (`dati/base/atti.toml:90-97`, `:318-325`, `:333-340`) [V]. Gli altri cantieri hanno `regione: None` [INF].
- **«Al voto successivo»**: il punto naturale è `vota_parlamento` (`partita.rs:2802-2893`), dove già si azzera ciò che vale per una legislatura: gradini bloccati (`:2804-2806`), campagna e turni da presidente (`:2862-2866`), dichiarazione, frange, protesta, riposizionamento (`:2874-2884`) [V].
- Trappola: `Ministri::nuovo_governo` ricostruisce ogni `StatoDicastero` da `default()` salvando solo `cantieri` e `margine_residuo` (`ministri/mod.rs:207-214`) [V]. Un elenco di cantieri finiti messo dentro `StatoDicastero` verrebbe svuotato alla nascita del governo, cioè un turno dopo il voto e a ogni ribaltone, se non lo si salva lì [INF].
- **Atti regionali eseguiti nel turno** (un turno solo, non cantieri): `esegui` prende l'agenda con `st.agenda.take()` (`ministri/agenda.rs:332`), paga e registra gli effetti (`:356-358`) [V]. Resta solo l'`EffettoAttivo`, senza atto; se l'atto tocca solo grandezze nazionali la regione si perde del tutto (`:185-189`) [V]. Prima dell'esecuzione l'atto e la regione sono in `VoceAgenda` (`vista.rs:541-553`), visibile a chi governa (`partita.rs:3987-3992`); all'apertura dopo c'è già l'agenda nuova [V]. Non lasciano traccia: **stato nuovo** anche per loro [INF].

## 5. PDE e conto di controllo

- Stato: `Economia.regole_ue: RegoleUe` (`economia/mod.rs:104-105`), con `pde: StatoPde`, `conto_controllo: f64` (% del PIL, mai sotto zero), `ultima_deviazione: f64` (% del PIL, già al netto della clausola), `turni_in_stato: u32`, `spesa_netta`, `pil_annuo`, `disavanzo_annuo: BTreeMap<u16, i64>` in euro per anno solare, `clausola_energia_fino_a: u16` (`economia/regole_ue.rs:13-32`) [V].
- Stati: `Nessuna`, `Aperta`, `Sospesa`, `Inadempiente`, `Intimata`, `Sanzionata` (`scenario.rs:19-28`); `e_inadempiente()` vale per gli ultimi tre (`:30-35`). Lo scenario parte da `sospesa` con conto 0,1 (`dati/base/scenario.toml:4`, `:195-196`) [V].
- Quando si aggiorna: `aggiorna_regole_ue` a ogni chiusura accumula spesa netta, PIL e disavanzo dell'anno e conta `turni_in_stato` (`economia/regole_ue.rs:111-126`); nel trimestre 2 gira `controllo_annuale` sull'anno prima (`:128-130`, `:143-191`) [V].
- Il controllo: `deviazione = (sn₁ − sn₂·(1 + tetto/100)) / pil₁ · 100 − clausola` (`:162-165`), `conto = max(0, conto + deviazione)` (`:166`), `rilevante = deviazione > 0,3 oppure conto > 0,6` (`:167-168`; manopole a `dati/base/manopole.toml:345-353`) [V]. Transizioni (`:171-186`) [V]: `Nessuna → Aperta` con disavanzo dell'anno sopra il 3%, o con debito sopra il 60%, disavanzo sopra lo 0,5% e deviazione rilevante; `Aperta`/`Sospesa → Inadempiente` se rilevante; `Aperta → Sospesa` altrimenti; `Sospesa → Nessuna` con disavanzo dell'anno e dell'anno mobile sotto il 3%; i tre stati d'inadempienza tornano a `Sospesa` se non rilevante. Fuori dal controllo (`:131-138`): `Inadempiente → Intimata` al primo trimestre 4, `Intimata → Sanzionata` dopo due turni nello stato.
- Ammende: `RegoleUe::ammende` (`:50-59`): `ammenda_pde` (0,2% del PIL) nel primo turno da `Inadempiente`, `ammenda_semestrale` (0,05%) ogni due turni da `Sanzionata` (`dati/base/manopole.toml:359-367`); entrano in `conti.spese.ammende` (`economia/finanza.rs:248`) [V].
- Fondi di coesione: finché `e_inadempiente()`, `sospensione_coesione` (0,25 punti di PIL) si toglie da `fondi_coesione_pil` (`economia/finanza.rs:225-239`, `dati/base/manopole.toml:373-374`) [V]. Nello stesso stato il TPI non azzera il premio di sfiducia (`economia/spread.rs:55-61`) [V].
- Debito/PIL: `debito_e_disavanzo_pil()` sugli ultimi quattro trimestri (`economia/spread.rs:21-27`) [V]. Nella `Vista` non c'è; il giocatore lo ha solo come forchetta del centro studi (`informazione/acquisti.rs:168`, `:203`) e, a fine partita, nell'eredità (`partita.rs:2687-2707`) [V]. Lo spread, che ne dipende, è esatto in `Vista` (`partita.rs:3968`) [V].
- **«Proiezione del conto di controllo negli altri turni»: non esiste.** Nessuna funzione la calcola [M: `grep -rnE "proiez|Proiez" crates/sim-core/src` → solo la proiezione dei seggi]. Fuori dal modulo, `regole_ue` è letto solo dal riepilogo del bilancio e scritto dalla clausola [M: `grep -rnE "regole_ue|StatoPde" partita.rs riepilogo.rs vista.rs informazione/acquisti.rs partita/ia.rs` → solo `partita.rs:4224`, `:5008-5015`]. Lo stato che serve c'è tutto: nessun campo nuovo nello snapshot [INF].
- Mattone più vicino: `economia::riepilogo` (`economia/legge_bilancio.rs:394-449`) calcola `crescita_spesa_netta` contro `tetto` annualizzando `regole_ue.spesa_netta` dell'anno sui trimestri chiusi (`:414-421`) e usando l'inflazione **pubblicata** per il resto (`:428-429`) [V]. È già mostrato al giocatore in `RiepilogoBilancio` (`:159-172`) [V].
- Vero o ufficiale: tutto il conto è costruito sullo stato vero. `spesa_netta_trimestre` (`economia/regole_ue.rs:73-90`) usa i conti veri del trimestre, la disoccupazione vera e quella strutturale, che non è mai mostrata; PIL e disavanzo dell'anno sono quelli veri (`:119-124`) [V]. Il giocatore conosce disavanzo, crescita e disoccupazione come prima stima con errore e, un turno dopo, come valore riveduto (`informazione/statistica.rs:91-96`, `:111-115`) [V].
- Cosa ne segue [INF]: l'esito del controllo (`pde`, `conto_controllo`, `ultima_deviazione`, turni nello stato, ammende, coesione sospesa) è un verdetto su un anno già chiuso e riveduto, quindi coerente con ciò che è pubblico. Una proiezione esatta sull'anno in corso includerebbe invece il trimestre appena chiuso, che il giocatore conosce solo per prima stima; il riepilogo del bilancio lo fa già in ottobre-dicembre.
- Fuori perimetro ma vicino: il «piano strutturale riveduto» della storia 51 (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:85`) non ha codice [M: `grep -rnE "piano_strutturale|piano strutturale" crates/sim-core/src` → nessun risultato].

## 6. Minaccia di scissione

- Stato: `Partito.minaccia: bool` (`partiti.rs:99-101`) [V]. È un booleano: non conserva né la causa né la distanza. `legislatura/fondazione.rs:74` lo inizializza soltanto a `false` nel partito nuovo [V].
- La distanza si ricava: `|posizione_percepita − posizione_dichiarata|` (`partita/vita.rs:121`), e le due posizioni sono già esatte in `VocePartito` (`vista.rs:449-450`) [V]. La causa è una sola per costruzione, quella distanza: è il testo della mossa, «la posizione percepita dista N punti dalla dichiarata» (`partita/vita.rs:130-133`) [V].
- Quando si accende e si spegne: solo in `scissioni()` (`partita/vita.rs:114-137`), in chiusura dopo l'aula (`partita.rs:580-581`), per ogni partito attivo, giocatore compreso. `p.minaccia = d >= minaccia_scissione` (`:127`): si accende a 0,5 e si spegne appena la distanza torna sotto (`dati/base/manopole.toml:1318-1319`). A `soglia_scissione` (1,0, `:1311-1312`) nasce la frangia e la minaccia torna `false` con la percepita sulla dichiarata (`partita/vita.rs:123`, `:167-168`); al tetto dei dieci partiti, o senza seggi per la frangia, resta minaccia (`:406-436`) [V].
- La riga nelle mosse esce una volta sola, quando la minaccia si accende (`partita/vita.rs:129-135`), e `mosse` si svuota a ogni chiusura (`partita.rs:550`): dal turno dopo la minaccia in corso non è più visibile [V]. Il flag non segue le mosse fatte dentro il turno (`Riposiziona`): si aggiorna solo alla chiusura [INF].
- Chi lo legge oggi: solo `scissioni()` [M: `grep -rnE "\.minaccia\b" crates/sim-core/src` → `partita/vita.rs:122`, `:127`, `:168` e i test]. Non è in `VocePartito` (`vista.rs:446-462`) [V].
- Stato nuovo: nessuno, se bastano il flag e la distanza [INF].

## 7. MES e ristrutturazione

- Stato: `Economia.programma_mes: Option<u32>`, il turno in cui il programma finisce (`economia/mod.rs:106-108`); `Economia.ristrutturazione: Option<Ristrutturazione>` con `turno: u32` del taglio e `premio: f64` in punti base (`:109-120`) [V]. I due non stanno mai insieme (`partita.rs:5045-5046`) [V].
- Da dove nascono: l'evento «Aste scoperte» si apre in `crisi_del_debito` con lo spread alla soglia (550 pb), senza programma in corso e senza ristrutturazione (`partita.rs:397-425`, `dati/base/manopole.toml:1535-1536`); le sue opzioni portano `EffettoOpzione::Mes` o `Ristrutturazione` (`partita.rs:4214-4219`) [V].
- **MES, durata**: `programma_del_mes` scrive `Some(turno + durata_mes)` (8 turni; `partita.rs:4250-4252`, `dati/base/manopole.toml:1542-1543`). È in corso finché `turno < fine` (`mes_in_corso`, `economia/mod.rs:228-230`) e il campo si azzera nel `motore` della chiusura in cui `turno >= fine` (`partita.rs:1068-1070`) [V].
- **MES, cosa vincola** [V]:
  - la condizionalità entra subito in vigore come legge «Memorandum del MES» (`partita.rs:4253-4262`): `eta_pensionabile` a `70` e `indicizzazione_pensioni` a `bloccata_sopra_soglia` (`dati/base/eventi.toml:666-672`);
  - ogni alleato perde `intesa_mes` (10) con causa `ProgrammaMes` (`partita.rs:4263-4266`, `parlamento/intesa.rs:28`);
  - `vincolo_mes` rifiuta leggi, anteprime, test, riforme accolte e decreti che portano quelle disposizioni a un altro gradino, col testo «vincolo del memorandum del MES, fino al turno N» (`partita.rs:4282-4299`; chiamate a `:1127`, `:1829`, `:1932`, `:2042`, `:2192`);
  - una legge già in esame entra in vigore saltando le disposizioni vincolate (`partita.rs:1184-1193`);
  - l'abrogativo su una disposizione vincolata decade senza voto (`partita.rs:1304-1310`);
  - tetto del bilancio: `oltre_il_tetto_mes` (`partita.rs:1741-1747`) ammette una crescita della spesa netta fino al maggiore fra il tetto UE dell'anno e la crescita della bozza; `tetto_mes` rifiuta oltre (`:1750-1759`, chiamato da `presenta_bilancio` a `:1727` e sugli emendamenti a `:1520`); l'IA riconduce il suo bilancio al tetto (`:1765-1792`);
  - spread: il premio di sfiducia resta a zero per tutta la durata (`economia/spread.rs:57-61`); la crisi del debito non si riapre (`partita.rs:403`).
- **Ristrutturazione**: `Economia::ristruttura` taglia il debito di `taglio_ristrutturazione` (0,5) e scrive `Ristrutturazione { turno, premio }` con `premio_ristrutturazione` (300 pb) (`economia/sfc.rs:52-73`, `dati/base/manopole.toml:1556-1579`) [V]. Il premio entra nello spread (`economia/spread.rs:42`) e si dimezza ogni 8 turni (`:17`, `:64-66`); sopra 50 pb tiene fuori il TPI (`:13`, `:55-56`) [V]. Per `turni_ristrutturazione` (8) turni dal taglio le fasce media e alta perdono `perdita_ristrutturazione` (5%) di reddito primario (`economia/mod.rs:122-128`, `elettorato/redditi.rs:100-107`) [V].
- Ristrutturazione, «fino a quando»: il campo resta finché il premio vale almeno 1 pb o la perdita dei redditi dura ancora (`partita.rs:1081-1085`); finché c'è, la crisi del debito non si riapre (`partita.rs:404-406`) [V]. Con 300 pb e l'emivita di 8 turni il premio scende sotto 1 pb dopo circa 66 turni [INF: 300 · 0,5^(n/8) < 1]. Non vincola né leggi né bilancio [INF: nessuna guardia la legge fuori da spread, redditi e crisi; M: `grep -rnE "ristrutturazione" crates/sim-core/src`].
- Nella `Vista`: né l'una né l'altra. Il giocatore le incontra solo nei testi di rifiuto e nelle cause dell'intesa [M: nessun campo in `vista.rs`, letto intero]. Stato nuovo: nessuno.

## 8. Clausola dell'energia

- **Non è per regione**: è un anno solo, nazionale. `RegoleUe.clausola_energia_fino_a: u16`, «ultimo anno solare per cui il giocatore ha chiesto il margine UE per l'energia», 0 se mai (`economia/regole_ue.rs:28-31`) [V].
- Chi la scrive: l'opzione «Chiedere il margine UE per l'energia per quest'anno, conta dal 2029» dell'evento `shock_energia` (`dati/base/eventi.toml:557-587`), con `EffettoOpzione::ClausolaEnergia` (`eventi/mod.rs:74-76`, ammesso solo sugli shock a `:474-476`). `risolvi_eventi` porta il campo all'anno corrente solo se l'anno è oltre `clausola_nazionale_fino_a` (`partita.rs:4220-4227`) [V].
- Cosa fa: nel controllo annuale dell'anno dopo toglie `clausola_energia_margine` (0,3 punti di PIL) dalla deviazione dell'anno coperto (`economia/regole_ue.rs:155-165`, `dati/base/manopole.toml:1528-1529`) [V]. Fino al 2028 compreso vale la clausola nazionale dello scenario, 0,6 punti, che la assorbe (`dati/base/scenario.toml:7`, `:201-202`; `economia/regole_ue.rs:155-156`) [V].
- «Attiva» significa quindi: l'anno corrente è coperto e il controllo della primavera dopo ne terrà conto [INF]. Nessun effetto su regioni, prezzi o grandezze [INF: gli unici lettori sono `controllo_annuale` e la validazione del salvataggio, `partita.rs:5008-5015`].
- Nella `Vista`: no. Stato nuovo: nessuno.

## 9. Quattro voci minori

**a. Forbice del sondaggio per regione: non esiste come dato.** `Incrocio { id, campione: u32, quote, posizione, approvazione }` (`informazione/sondaggio.rs:23-33`) porta il campione del sottoinsieme, non un margine [V]. Il campione è `round(n_tot · elettori del sottoinsieme / elettori totali)` (`:186-187`) [V]. L'errore di campionamento con cui la quota è estratta è `√(effetto_disegno · p(1−p) / n)` (`:67`), con `effetto_disegno` = 2 (`dati/base/manopole.toml:485-486`) [V]. In `Vista` ci sono `quote` e `campione` (`partita.rs:3993`); la manopola no, e nessuna funzione calcola un margine, nemmeno per il dato nazionale [M: `grep -rnic "forbice\|margine_errore\|intervallo" informazione/sondaggio.rs informazione/campione.rs informazione/mod.rs` → 0]. La forbice di campionamento non copre bias comune ed effetto d'istituto, che spostano le quote e restano nascosti (`informazione/sondaggio.rs:182-184`, `informazione/mod.rs:27-30`) [V]. Stato nuovo: nessuno.

**b. Combinazioni spente della legge elettorale: la regola c'è, la domanda «questo gradino è ammesso dato il resto» ha una funzione interna, nulla in `Vista`.**
- La regola è `LeggeElettorale::valida` (`elezioni/mod.rs:133-176`): ogni valore numerico è un gradino del menu, e il premio alla coalizione vuole le coalizioni attive, quello alla lista le vuole spente (`:165-175`) [V]. Il 40% non è un controllo sulla legge: è `Menu::valida`, che al caricamento dei dati rifiuta un menu con una soglia del premio sotto 4000 punti base (`:199-203`) [V]. Il menu del gioco non può quindi contenere quel gradino [INF].
- Dove si applica: `Parlamento::bozza` chiama `legge_non_valida` due volte, sui valori della legge e sugli stessi più gli arrivi elettorali in corso (`parlamento/mod.rs:769-777`); il rifiuto è «legge elettorale non applicabile: …» [V]. Passano da `bozza` `presenta_legge` (`partita.rs:1828-1837`), `anteprima` (`:1929-1934`), il test di una misura (`:1126-1129`) e la proposta dell'opposizione (`parlamento/mod.rs:649-657`) [V]. Le leggi in calendario rese non valide decadono all'apertura (`partita.rs:680-685`) [V].
- La funzione che risponde: `legge_non_valida(dati, disposizioni, valori: &[(usize, usize)]) -> Option<String>` (`parlamento/regole.rs:139-150`), `pub(crate)`, che compone la legge in vigore coi valori dati (`Disposizioni::legge_con`, `disposizioni/stato.rs:178-192`) e la valida [V]. Da fuori, la sola via è `Partita::anteprima(valori)`, che però porta con sé gli altri rifiuti (chi governa, crisi, MES, già in vigore, bloccato, in esame, respinta da poco; `partita.rs:1929-1934`, `parlamento/mod.rs:692-696`) e rende un testo [V].
- In `Vista`: `VoceGradino` ha solo `bloccato`, cioè abrogato da un referendum (`vista.rs:390-398`, `disposizioni/stato.rs:231-238`) [V]. Stato nuovo: nessuno.

**c. Posti liberi in calendario: lo stato c'è, il numero no.** `POSTI_ORDINARI = 3`, `POSTI_COSTITUZIONALI = 1`, `POSTI_OPPOSIZIONE = 1` (`parlamento/mod.rs:33-40`) [V]. `presenta` conta come occupati i posti ordinari le misure di forma `Riforma` o `Elettorale` senza `promotore`, e il posto costituzionale quelle `Costituzionale` senza `promotore`; al limite rifiuta con «calendario pieno» (`:585-602`) [V]. Conversioni dei decreti, bilancio e sfiducia non occupano posti [INF dallo stesso filtro]. Il posto riservato è libero se nessuna misura con `promotore` è stata presentata nel turno (`posto_riservato_libero`, `:668-674`) [V]. Nel turno del bilancio o di crisi le leggi col voto nel turno slittano, ma la presentazione non è chiusa da `presenta` (`partita.rs:567`, `:789-793`; in crisi la ferma `fuori_dalla_crisi`, `comando.rs:268-277`) [V]. `Vista.calendario` porta `forma` e `promotore` di ogni misura (`vista.rs:275-294`) e `posto_riservato: bool` (`vista.rs:59-61`), ma non le tre costanti né un conto dei posti; l'anteprima salta il controllo del calendario (`partita.rs:1924-1928`), quindi «pieno» si scopre solo presentando [V]. Stato nuovo: nessuno.

**d. Costo di Ferma e Dirotta prima di confermare: è deterministico, l'anteprima non esiste.**
- Ferma: `delta = −costo_fermare · attaccamento all'atto in agenda` (`ministri/agenda.rs:383-389`; `costo_fermare` = 15, `dati/base/manopole.toml:625-626`) [V].
- Dirotta: `delta = −costo_fermare · max(0, att(vecchio) − att(nuovo)) − costo_dirottare_regione · max(0, bersaglio(vecchia) − bersaglio(nuova))` (`ministri/agenda.rs:447-458`; seconda manopola = 2, `dati/base/manopole.toml:646-647`) [V].
- Gli ingredienti sono stato salvato nell'agenda: `Agenda.attaccamenti: Vec<f64>` per i 4 atti del dicastero («mai nella `Vista`») e `Agenda.bersaglio: Vec<f64>`, il punteggio di ogni regione (`ministri/agenda.rs:35-38`), scritti alla nascita dell'agenda senza il rumore della scelta (`:300`, `:318-319`) [V]. Nessuna estrazione al momento del comando [V]. La soddisfazione è poi limitata a 0-100 (`:208`): la variazione effettiva può essere più piccola del `delta` [V].
- Ferma Cantiere: stesso prezzo, con l'attaccamento ricalcolato al momento del comando (`ministri/agenda.rs:492-497`) [V].
- Effetti collaterali da non perdere in un'anteprima: Ferma ritira il decreto se l'agenda viene da un `Accogli` (`ministri/mod.rs:458-461`, `:514-525`); Dirotta sposta la conversione sull'atto nuovo (`comando.rs:302`) [V].
- Anteprime esistenti: `anteprima`, `anteprima_decreto`, `anteprima_accogli`, `anteprima_campagna`, `anteprima_sfiducia`; nessuna per Ferma o Dirotta [M: `grep -n "    pub fn " crates/sim-core/src/partita.rs`]. Mostrare il costo rende leggibili gli attaccamenti, che oggi sono nascosti [INF]. Stato nuovo: nessuno.

## Tabella riassuntiva

| Voce | Stato che c'è | Stato nuovo nello snapshot | Funzione che lo legge oggi |
|---|---|---|---|
| 1. Disoccupazione per regione, in ritardo | `Economia.regioni[r].disoccupazione` (corrente) | **sì**: il valore della chiusura precedente non è conservato | `disoccupazione_nazionale`, `Stima::vera`, `giudizio`, `composizione`, `termini_regioni` |
| 1. Reddito per regione | `Gruppo.reddito`, `Gruppo.reddito_prima`, `Gruppo.quota` | no coi pesi di oggi; sì se si vogliono i pesi della chiusura precedente | `Elettorato::gini`, `mobilita`, `giudizio` (nessuna aggrega per regione) |
| 2. Voti per regione dell'ultimo voto | calcolati in `vota_parlamento` e ceduti al riparto | **sì** | `legislatura::voti_per_regione` |
| 2. Seggi del Senato per regione | `Esito.senato_per_regione`, scartato | **sì** | `elezioni::riparto` (solo i test lo leggono) |
| 2. Seggi della Camera per regione e lista | non esiste (riparto nazionale) | non pertinente | `seggi_per_circoscrizione` dà solo i seggi totali |
| 3. Quota dei 18 gruppi per regione | `Gruppo.quota` | no | `Elettorato::elettori`, `sondaggio::rileva_con` |
| 3. Elettori campione | derivati dal sondaggio pubblico | no | `Partita::elettori_campione` (non chiamata dal guscio) |
| 4. Cantieri finiti | tolti a `ministri/agenda.rs:347` | **sì** | `Ministri::esegui` |
| 4. Atti regionali eseguiti | solo `EffettoAttivo`, senza atto | **sì** | `Ministri::esegui` |
| 5. PDE, conto, deviazione, ammende, coesione | `Economia.regole_ue` | no | `aggiorna_regole_ue`, `controllo_annuale`, `ammende`, `conti_del_trimestre` |
| 5. Proiezione del conto negli altri turni | non esiste; ingredienti in `regole_ue.spesa_netta` | no | più vicino: `economia::riepilogo` |
| 6. Minaccia di scissione | `Partito.minaccia: bool`, distanza dalle due posizioni | no | `Partita::scissioni` |
| 7. MES | `Economia.programma_mes: Option<u32>` | no | `mes_in_corso`, `vincolo_mes`, `oltre_il_tetto_mes` |
| 7. Ristrutturazione | `Economia.ristrutturazione: Option<Ristrutturazione>` | no | `spread_dallo_stato`, `aggiorna_redditi`, `crisi_del_debito` |
| 8. Clausola dell'energia | `RegoleUe.clausola_energia_fino_a: u16` (nazionale) | no | `controllo_annuale` |
| 9a. Forbice del sondaggio per regione | `Incrocio.campione`, `quote`; manopola `effetto_disegno` | no | `sondaggio::campiona` |
| 9b. Combinazioni elettorali ammesse | regola in `LeggeElettorale::valida` | no | `parlamento::legge_non_valida` |
| 9c. Posti liberi in calendario | `Parlamento.misure`, tre costanti | no | `Parlamento::presenta`, `posto_riservato_libero` |
| 9d. Costo di Ferma e Dirotta | `Agenda.attaccamenti`, `Agenda.bersaglio` | no | `Ministri::ferma`, `Ministri::dirotta` |

Voci che chiedono stato nuovo: tre (disoccupazione regionale della chiusura precedente; voti e seggi del Senato per regione dell'ultimo voto; cantieri finiti e atti regionali eseguiti), più il reddito regionale se lo si vuole coi pesi della chiusura precedente. Un solo salto di `VERSIONE_SCHEMA`, da 15 a 16, se entrano nello stesso piano [INF].

## Per il nostro gioco

**Prendiamo.**
- Il meccanismo del ritardo che c'è già: una copia «prima» fatta all'inizio del passo, come `grandezze.prima`. Per la disoccupazione regionale è l'unico modo di restare coerenti con la revisione nazionale già pubblica.
- I due risultati che il voto calcola e butta: `voti_per_regione` e `Esito.senato_per_regione`. Sono già del tipo giusto, per id di regione.
- `vota_parlamento` come punto in cui svuotare ciò che dura «fino al voto successivo»: è già lì che si azzera lo stato di legislatura.
- `legge_non_valida` per le combinazioni elettorali e le formule di `ferma` e `dirotta` per il costo: la regola è una sola e sta nel core, come chiede la spec (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:247-248`).
- `elettori_campione` così com'è: legge solo ciò che è pubblico.

**Evitiamo.**
- Esporre `economia.regioni[r].disoccupazione` corrente: rivela `Statistica.vera`.
- Mettere l'elenco dei cantieri finiti in `StatoDicastero` senza toccare `nuovo_governo`: lo svuoterebbe.
- Presentare i seggi del Senato per regione come se sommassero ai seggi del partito: mancano premio ed estero e vengono prima delle cessioni.
- Cercare i seggi della Camera per regione e lista: il riparto è nazionale, il dato non esiste.
- Trattare la clausola dell'energia come regionale: è un anno, per tutto il Paese.
- Far contare alla UI i posti del calendario o il margine del sondaggio con costanti sue: `POSTI_ORDINARI` e `effetto_disegno` stanno nel core e nei dati.

**Aperto.**
- Reddito per regione: quale dei cinque candidati, e se il ritardo vale anche per i pesi (`quota`).
- Quota dei gruppi nella scheda: esatta e corrente, in ritardo come le grandezze, o del turno zero. È stato vero che oggi trapela solo per aggregati.
- Proiezione del conto di controllo: esatta sullo stato vero (rivela il trimestre appena chiuso) o costruita sui valori pubblicati, come fa in parte il riepilogo del bilancio.
- Debito/PIL: oggi il giocatore non lo vede esatto durante la partita; la PDE lo usa per aprirsi.
- Costo di Ferma e Dirotta: mostrarlo per ogni alternativa rende leggibili gli attaccamenti del ministro, oggi nascosti.
- Forbice del sondaggio: solo campionamento (stretta, e più stretta dell'errore vero) o qualcosa che tenga conto degli altri due strati.
- Chiave delle regioni nella `Vista`: oggi indice in un punto, id in un altro, nome in un terzo.
- Minaccia di scissione: il flag si aggiorna solo in chiusura; un riposizionamento nel turno non lo muove.

## Collegamenti

- [[persistenza-dei-salvataggi]]: peso dello snapshot e regole del formato dell'albo, che fissano il costo dello stato nuovo.
- [[consiglieri-e-informazione-imperfetta]]: i tre livelli (stato vero, statistica ufficiale, rapporto) contro cui va letta ogni esposizione.
- [[sondaggi-ed-errore]]: i tre strati d'errore, per la forbice regionale.
- [[regole-fiscali-ue]]: conto di controllo, PDE e clausole, la fonte del modulo `regole_ue`.
- [[sistemi-elettorali]]: riparto nazionale della Camera e regionale del Senato, premio e tetti.
- [[vita-dei-partiti]]: soglie di minaccia e di scissione.
- [[situazioni-ed-eventi-al-turno-zero]]: aste, MES e ristrutturazione.
- [[disposizioni-territoriali-al-turno-zero]]: i perimetri regionali che la scheda della regione mostra.
- [[cursori-dello-scenario]]: l'altra pagina che mappa una funzione del prodotto sullo stato di `sim-core`.
