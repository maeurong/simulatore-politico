# V1 · Piano 6: parlamento e misure — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Le misure normative dentro `sim-core`: 35 disposizioni ordinarie più le 9 della legge elettorale e le 2 costituzionali, con valore in vigore, legge d'origine, valore precedente, valore in arrivo ed effetti sul motore; l'aula col parlamento in carica, il calendario, le leggi di riforma presentate e votate con linee e franchi tiratori, emendamenti degli alleati, questione di fiducia, legge elettorale coi seggi attesi, norme costituzionali col referendum confermativo; decreti con conversione a fine turno, il voto della legge di bilancio, le proposte normative dei ministri, l'anteprima della misura e il referendum abrogativo.

**Architecture:** Due moduli nuovi in `sim-core`. `disposizioni/` porta il catalogo (decimo file di dati `dati/base/disposizioni.toml`, più le 9 elettorali generate dal menu di `elezioni.toml`), lo stato in partita e gli effetti: ogni cambio di valore diventa effetti di livello sulle grandezze del piano 5 e leve che il motore legge da `Misure`, al posto dei regimi che oggi stanno nello scenario. `parlamento/` porta seggi in carica, intesa degli alleati, misure in esame, voto, emendamenti, fiducia, decreti, referendum; il passo «aula» di `chiudi_turno` vota in ordine conversioni, bilancio, leggi, referendum. I comandi entrano dall'`enum Comando` esistente.

**Tech Stack:** Rust 1.98, workspace esistente (`sim-core`, `sim-cli`, `app/src-tauri`), nessuna dipendenza nuova; Python 3 con la sola libreria standard per il controllo del catalogo; Tauri 2.12.3.

**Spec:** [`docs/superpowers/specs/2026-10-21-v1-giocabile.md`](../specs/2026-10-21-v1-giocabile.md): storie 46 (riga 80), 54-69 (righe 91-106), 70-76 (righe 110-116), 95-97 (righe 141-143), 117 (riga 169), 142 (riga 206); «Modulo dati» (righe 268-270, schema della disposizione e valori di partenza); «Motore economico» (righe 298-303: redditi, fisco, spesa obbligatoria, territoriali); «Elettorato» (righe 312-313: colpa nel perimetro, posizione percepita); «Elezioni e riparto» (righe 321-322); «Parlamento» (righe 340-345); «Informazione» (riga 371, anteprima); «Turno e partita» (righe 385-386); «Testing Decisions» (riga 427); «Further Notes» (righe 484, 487). Scope dalla tabella dei piani: [`2026-10-21-v1-piano-1-fondamenta.md:47`](2026-10-21-v1-piano-1-fondamenta.md). Ticket: `.scratch/v1-giocabile/issues/08-iter-parlamentare.md`, `13-legge-elettorale-come-misura.md`, `16-catalogo-atti-e-riforme.md` con l'asset `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:21-53`, `22-valori-di-partenza-delle-disposizioni.md`, `24-abrogazione-e-gradini-di-partenza.md`, `27-mappa-delle-20-regioni.md:43-49`. Debiti lasciati dai piani precedenti: `2026-10-22-v1-piano-2-riparto-elettorale.md:15`, `2026-10-22-v1-piano-3-motore-economico.md:24`, `2026-10-22-v1-piano-4-elettorato-e-sondaggi.md:24`, `:27`, `2026-10-25-v1-piano-5-governo.md:25-31`.

## Tre PR

| PR | Branch | Base | Task | Contenuto |
|---|---|---|---|---|
| A · disposizioni | `feat/v1-piano-6-disposizioni` | `main` | 1-5 | catalogo, stato in partita, valori in arrivo, prerequisiti degli atti, effetti sul motore, disposizioni territoriali |
| B · leggi in aula | `feat/v1-piano-6-aula` | `feat/v1-piano-6-disposizioni` (impilata; dopo lo squash di A, `merge -s ours origin/main`, niente force-push) | 6-9 | parlamento in carica, intesa, calendario, voto con linee e franchi tiratori, posizione percepita, emendamenti, fiducia, legge elettorale, norme costituzionali e referendum confermativo |
| C · decreti, bilancio e referendum | `feat/v1-piano-6-decreti` | `feat/v1-piano-6-aula` (impilata, stessa regola) | 10-14 | decreti e conversione, Accogli come decreto, voto del bilancio, proposte normative dei ministri, anteprima, referendum abrogativo |

La PR A gira senza aula: le disposizioni cambiano solo coi valori in arrivo, che è anche come i test della A le muovono. La B porta le leggi; la C tutto ciò che chiude i debiti del piano 5 e il referendum.

## Confini con gli altri piani (scelte di questo piano)

- **Giocatore.** Come nel piano 5: il giocatore è il Presidente del Consiglio del governo uscente dello scenario, finché il piano 7 non porta fondazione e voto.
- **Parlamento in carica.** I seggi della legislatura uscente nascono in `Partita::nuova` dal riparto delle intenzioni vere del turno zero, con la legge di partenza e le coalizioni dello scenario, senza estrazioni. Il piano 7 li sostituisce col voto del turno zero e chiude l'aula in quel turno.
- **Intesa.** Esiste da qui, perché linee, franchi tiratori, emendamenti e fiducia la leggono: valore 0-100 per ogni alleato, base e ritorno verso la base, le quattro cause che nascono in questo piano (norma lontana, emendamento respinto, fiducia, seggi persi). Le altre cause della spec (riga 333), ultimatum come stato con la sua causa in evidenza, uscita sotto 25 e rimpasto sono del piano 7. Qui «in ultimatum» è solo la lettura `intesa < soglia_ultimatum`, che serve alla fiducia (storia 65).
- **Dimissioni del governo.** Fiducia respinta, e bilancio respinto anche nell'esercizio provvisorio, restano nell'esito del voto (`fiducia_respinta`, esercizio provvisorio che prosegue): il percorso delle dimissioni è del piano 7, come già deciso nel piano 5 per il bilancio.
- **Legislatura.** Durata e numero dei parlamentari sono disposizioni costituzionali con le loro leve; li legge il piano 7, che porta il voto, la fine della legislatura (decadenza delle leggi in esame, sblocco dei gradini abrogati, blocco dei referendum nell'ultimo anno, sospensione per scioglimento). Qui il numero dei parlamentari cambia solo la proiezione, e «vale dal voto successivo» è vero per costruzione: nessun voto avviene prima del piano 7.
- **Opposizione.** Il posto del calendario riservato all'opposizione esiste ed è sempre vuoto: le proposte dell'opposizione arrivano coi piani 7 (giocatore) e 8 (partiti IA). Il referendum abrogativo si apre con `Partita::promuovi_referendum`, che chiameranno il comando del piano 7 e i partiti IA del piano 8; il referendum confermativo lo chiedono già qui i partiti contrari, per regola.
- **Decreti.** Gli inneschi «evento aperto» e «crisi del debito» sono del piano 9: qui un decreto su disposizioni si rifiuta sempre, prima per materia esclusa (elettorale, costituzionale) e poi per mancanza di innesco; passano i decreti di fondi (Accogli e spostamento fra dicasteri).
- **Bilancio.** Gli emendamenti degli alleati al bilancio (storia 44) restano al piano 7, come dichiarato nel piano 5.
- **Vocazione.** Le cause d'intesa che pesano di più per un partito identitario (spec riga 333) e l'emendamento pesato per vocazione arrivano col piano 8; qui tutti gli alleati pesano uguale.
- **Esito e registro.** Come nel piano 5: la vista porta gli esiti dei voti degli ultimi due turni, gli esiti dei referendum e le variazioni d'intesa con la loro causa; la scelta delle 7 voci in evidenza resta aperta.
- **Testi del piano 5.** Il piano 6 non li riscrive: restano in attesa della revisione di Mario. I testi nuovi stanno nei task, ciascuno accanto alla regola che lo produce, e la PR C ne porta l'elenco per la stessa revisione.

## Scelte di prodotto

Prese con Mario il 26/10/2026:

1. **Accogli seguito da Ferma o da dimissioni chieste ritira il decreto**: niente speso, nessun voto di conversione, margine residuo riportato a prima; si paga solo il costo di Ferma o delle dimissioni. Dirotta su un altro atto tiene il decreto.
2. **Nel turno del bilancio le leggi slittano, i decreti no**: le conversioni si votano comunque; leggi ordinarie e norme costituzionali il cui voto cade nel turno slittano al turno dopo; si possono ancora presentare.

Prese dal piano, da confermare in review:

3. **Il partito del Presidente del Consiglio vota sì alle misure del governo**: la linea della spec (riga 340) vale per tutti gli altri; i suoi franchi tiratori crescono con la distanza come per gli altri.
4. **Bilancio e decreti di fondi non hanno posizione**: maggioranza sì, opposizione no, franchi tiratori dalla sola intesa.
5. **«Colpito nei suoi interessi»** (spec storia 63) vuol dire che la legge tocca una disposizione di un dicastero che l'alleato tiene: si legge in una riga e si calcola senza simulare.
6. **Emendamento ignorato = respinto**, come una proposta ignorata.
7. **Una legge di riforma tocca disposizioni di un solo dicastero**, così ha un solo ministro proponente; le disposizioni fuori dicastero (elettorali, autonomia) non hanno proponente.
8. **Anteprima sugli effetti nominali scritti nei dati**, come l'agenda del piano 5, mai su una simulazione dello stato vero: le fasce le sbaglia il ministro, non il motore.
9. **Referendum abrogativo deciso dalla distanza**, senza giudizio sul governo (la spec lo mette solo nel confermativo); sui quesiti elettorali, che non hanno posizione, i votanti si dividono a metà e conta solo il quorum.
10. **Disposizioni che nessuna grandezza legge** (cittadinanza, export di armi) muovono solo linee, emendamenti e posizione percepita.
11. **L'esercizio provvisorio occupa il calendario come ottobre-dicembre, se il bilancio è presentato**: è il turno del secondo voto sul bilancio. Senza bilancio presentato il calendario non è occupato, nemmeno al turno 0: le leggi si votano.
12. **Lo spostamento di fondi per decreto è ammesso anche nell'esercizio provvisorio**: ha saldo zero, non è una misura che costa denaro.
13. **Regimi e parametri della spesa obbligatoria lasciano lo scenario**: `regime_pensioni`, `regime_sussidi`, `eta_pensione`, `indicizzazione_pensioni` e la manopola `sussidio_universale` diventano leve delle disposizioni, una sola fonte di verità; `pensione_minima` resta nello scenario come livello in euro, e la disposizione lo moltiplica. «Per fasce» vale 0,95 dell'inflazione [assunzione del piano 6: media dei tre scaglioni della L. 160/2019, c. 478, pesata a occhio], dove il piano 3 metteva 1.
14. **Fiducia anche sul bilancio**: la spec (riga 342) la prevede su leggi e conversioni; il piano la estende alla legge di bilancio (Task 8 e 11), col voto a blocchi senza franchi tiratori.

## Global Constraints

Valgono tutti i vincoli dei piani 1, 3, 4 e 5 (`2026-10-21-v1-piano-1-fondamenta.md:13-26`, `2026-10-22-v1-piano-3-motore-economico.md:26-31`, `2026-10-22-v1-piano-4-elettorato-e-sondaggi.md:33-44`, `2026-10-25-v1-piano-5-governo.md:42-53`): italiano coi termini di `CONTEXT.md` (disposizione, disposizione territoriale, perimetro, legge di riforma, valore in arrivo, valore dopo l'abrogazione, decreto, legge, norma costituzionale, legge elettorale, anteprima, linea, franco tiratore, emendamento, questione di fiducia, calendario, esercizio provvisorio, referendum abrogativo, referendum confermativo, ministro proponente, intesa; mai «manopola» o «parametro» per una disposizione, mai «riforma» da sola, «pacchetto», «abrogazione per legge», «dissidente», «decreto-legge»); `sim-core` senza I/O e senza thread; solo `Vec`, array fissi e `BTreeMap`; euro in `i64`; dati in TOML con errori che nominano l'elemento colpevole. In più:

- Nessuna dipendenza nuova in nessun crate; `Cargo.lock` non cambia.
- I franchi tiratori escono da `Flusso::Parlamento = 2` (`crates/sim-core/src/caso.rs:12`), finora inusato: per ogni voto in aula, una uniforme per seggio, Camera e poi Senato, partiti nell'ordine dello scenario, anche sotto fiducia e anche per i seggi che non possono defezionare, così ogni voto consuma sempre lo stesso numero di estrazioni. Le fasce dell'anteprima escono da `Caso::estemporaneo` e non avanzano nessun flusso. Nessun'altra estrazione nuova.
- Un comando rifiutato lascia la partita identica: `salva()` prima e dopo uguali. Ogni rifiuto è `Errore::Comando` col nome della disposizione, del gradino, della misura, del partito o del dicastero in causa.
- Ogni valore provvisorio è una manopola con `valore`, `min`, `max`, `ticket`, aggiunta a `MANOPOLE_RICHIESTE` (`crates/sim-core/src/dati.rs:61`) dal task che la introduce; quelle senza fonte portano `# assunzione del piano 6, nessuna fonte in docs/ricerca`.
- La `Vista` non porta stato vero nuovo: intesa e linee degli alleati in cifre (il giocatore governa, spec riga 370), conteggi esatti dopo il voto, prima solo la forbice; nessun franco tiratore estratto prima del voto.
- `VERSIONE_SCHEMA` passa a `6` nel Task 3 e resta `6` per tutto il piano.
- Ogni task che tocca `vista.rs` o un tipo esportato rigenera i tipi con `cargo test -p sim-core` e committa `app/src/lib/tipi/`; `npm --prefix app run check` passa.
- Verifiche di ogni task Rust: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --check`.
- Turni del calendario nei test: turno 0 = ottobre-dicembre 2026, 1 = gennaio-marzo 2027, 3 = luglio-settembre 2027, 4 = ottobre-dicembre 2027 (bilancio del 2028), 5 = gennaio-marzo 2028, 6 = aprile-giugno 2028. Un voto «a fine turno t» avviene dentro la `chiudi_turno` che chiude il turno t.

## Review Focus

1. **Proponente che si dimette con la sua legge in esame**: la legge resta in calendario e si vota, i costi che toccherebbero il proponente cadono sull'interim (nessuno), nessun panic. Test `dimissioni_del_proponente_non_fanno_cadere_la_legge` nel Task 7.
2. **Legge approvata su una disposizione con un valore in arrivo**, e referendum che la abroga: il valore in arrivo si cancella e non arriva più. Test `legge_approvata_cancella_il_valore_in_arrivo` nel Task 7 e `abrogazione_cancella_il_valore_in_arrivo` nel Task 14.
3. **Salvataggio a metà turno con una legge presentata, un emendamento accolto, la fiducia posta e un decreto accolto** (storia 146): la partita ricaricata chiude il turno identica a quella mai salvata. Test `salvare_a_meta_turno_conserva_l_aula` nel Task 8, esteso nel Task 10 col decreto.
4. **Comando su una misura, un partito, una disposizione o un gradino che non esistono, già votati o fuori momento**: rifiuto col nome, partita invariata. Test `leggi_non_ammesse_rifiutate_col_nome` nel Task 6, `comandi_dell_aula_non_ammessi_rifiutati_col_nome` nel Task 8, estesi nei Task 10 e 14 coi comandi che aggiungono.
5. **Legge elettorale che diventa incoerente solo dopo** (un regime emendato che arriva quattro turni dopo, mentre un'altra legge ha cambiato le coalizioni): presentazione ed emendamento si controllano sullo stato con gli arrivi in corso; se comunque gli arrivi di un turno darebbero una legge elettorale non valida, si scartano tutti quelli elettorali di quel turno e le disposizioni restano com'erano, senza panic. Test d'unità `arrivo_che_rompe_la_legge_elettorale_si_scarta` in `disposizioni/stato.rs` nel Task 9: dal seam non ci si arriva, ed è proprio il caso che va coperto.

## Dispatch

PR A: tutto in sequenza, 1 → 2 → 3 → 4 → 5. Il Task 1 (`coder`, scrive solo `dati/base/disposizioni.toml`) viene prima perché i test comuni includono il file con `include_str!`; dal 2 in poi ogni task tocca `dati.rs` o `partita.rs` e consuma i tipi del precedente nell'ordine catalogo → stato → effetti → territoriali.

PR B: dopo la review della A, sul branch impilato, 6 → 7 → 8 → 9 in sequenza: tutti toccano `parlamento/` e `comando.rs`.

PR C: dopo la review della B, 10 → 11 → 12 → 13 → 14 in sequenza: tutti toccano `parlamento/`, `comando.rs` e `partita.rs`.

Ogni brief nomina `skill-gate`, `caveman` e, per chi scrive codice (`rust-engineer`, `coder`), `ponytail`; copia la riga **Ricerca** del task come riferimento. Il round pre-commit di fine PR aggiunge `rust-reviewer` (diff `.rs` in tutte e tre) e `spec-reviewer` (il piano esiste), in parallelo a `security-reviewer`, `code-reviewer`, `test-writer`, `craft-reviewer`. Il `craft-reviewer` della A legge nomi e gradini del catalogo, quello della C l'elenco dei testi per il giocatore. La PR A chiede la revisione di Mario sul contenuto di `dati/base/disposizioni.toml` (posizioni, saldi, effetti, spec riga 484) prima del merge.

---

## PR A · Disposizioni (Task 1-5)

### Task 1: Catalogo delle disposizioni

Contenuto, non codice: 35 disposizioni ordinarie e 2 costituzionali sotto la revisione di Mario nella PR. Le 9 elettorali non stanno nel file: le genera il Task 2 dal menu.

**Files:**
- Create: `dati/base/disposizioni.toml`

**Ricerca:** `docs/ricerca/sintesi/disposizioni-al-turno-zero.md:16-49` (valore vigente, legge d'origine, valore precedente), `:51-60` (correzioni ai gradini), `:62-80` (abrogazione); `.scratch/v1-giocabile/issues/24-abrogazione-e-gradini-di-partenza.md:23-45` (valore dopo l'abrogazione, esclusioni, gradini rinominati); `docs/ricerca/sintesi/disposizioni-territoriali-al-turno-zero.md:14-19`, `:74-79` (le tre territoriali).

**Dispatch:** `coder` · primo della PR A, nessuna dipendenza · skill-gate: true. Legge l'asset `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:21-53` e la tabella delle grandezze di `dati/base/grandezze.toml` più la riga nuova `regole_lavoro` (sotto).

**Interfaces:**
- Consumes: gli id delle grandezze di `dati/base/grandezze.toml` e la loro colonna `dicasteri`; la grandezza `regole_lavoro` che il Task 2 aggiunge al file delle grandezze (nazionale, `dicasteri = ["lavoro_welfare"]`, punti di disoccupazione strutturale tolti, iniziale 0, min −5, max 5); gli id dei dicasteri e delle regioni.
- Produces: `dati/base/disposizioni.toml`, 37 `[[disposizione]]` nell'ordine della tabella sotto, che il Task 2 carica. Campi nell'ordine `id, nome, dicastero, tipo, norma, referendabile, gruppi, perimetro, partenza, legge, precedente, dopo_abrogazione, in_arrivo, gradini`; `dicastero` assente = fuori dicastero; `perimetro` e `dopo_abrogazione` solo dove servono; `in_arrivo` sempre scritto, anche vuoto (`in_arrivo = []`), e su una riga sola anche con più arrivi, perché il test helper `con_arrivo` del Task 3 lo cerca e riscrive quella riga. Ogni gradino su una riga: `{ id, nome, posizione, saldo, leve, effetti }`, con `leve` ed `effetti` omessi se vuoti. `saldo` = costo annuo in % del PIL potenziale (positivo = spesa in più o entrate in meno) del gradino; il motore usa la differenza dal gradino di partenza, che quindi ha saldo 0. `effetti` = livello della grandezza col gradino in vigore, `{ grandezza, intensita, ritardo }`, sempre rispetto alla partenza (il gradino di partenza non ne ha).

Le leve, chiuse (il Task 2 rifiuta un nome che non è qui), e chi le legge:

| leva | unità | disposizione | letta da |
|---|---|---|---|
| `regime_pensioni` | 0 retributivo, 1 contributivo, 2 quota base universale | `regime_pensioni` | previdenza (Task 4) |
| `eta_pensione` | anni | `eta_pensionabile` | previdenza |
| `indicizzazione_pensioni` | quota dell'inflazione | `indicizzazione_pensioni` | previdenza |
| `pensione_minima` | × la pensione minima dello scenario | `pensione_minima` | previdenza |
| `sussidi` | × la spesa per disoccupato | `sussidi_disoccupazione` | previdenza |
| `reddito_minimo` | € annui per elettore dei disoccupati e inattivi di fascia bassa | `reddito_minimo` | redditi (Task 4) |
| `salario_minimo` | €/h, 0 = nessuno | `salario_minimo` | redditi |
| `cuneo` | punti del salario esenti per i dipendenti di fascia bassa e media | `cuneo_fiscale` | redditi |
| `forfettario` | × le imposte degli autonomi | `forfettario` | redditi |
| `accise` | punti sul prezzo dell'energia | `accise` | misure (Task 4) |
| `decontribuzione` | punti di disoccupazione strutturale tolti nel perimetro | `decontribuzione` | misure (Task 5) |
| `autonomia` | quota della spesa dei dicasteri che segue il gettito nel perimetro | `autonomia_differenziata` | misure (Task 5) |
| `durata_legislatura` | anni | `durata_legislatura` | piano 7 |
| `seggi_camera`, `seggi_senato` | seggi | `numero_parlamentari` | proiezione (Task 9), piano 7 |

Elenco, con gli id che i test usano (gradino di partenza con *). Le colonne «legge» e «precedente» vengono dalla ricerca, «canale» dice dove va l'effetto:

| id | dicastero | tipo | gradini (id) | precedente | referendabile | canale |
|---|---|---|---|---|---|---|
| `forfettario` | economia_finanze | parametro | `assente` · `65000` · `85000`* · `100000` | `65000` | no | leva `forfettario`, saldo |
| `cuneo_fiscale` | economia_finanze | parametro | `0` · `3` · `7`* | `0` | no | leva `cuneo`, saldo |
| `detrazione_edilizia` | economia_finanze | parametro | `0` · `36_50`* · `65` · `110` | `36_50` | no | `domanda_regionale` in tutte le regioni, saldo |
| `tetto_contante` | economia_finanze | parametro | `1000` · `3000` · `5000`* · `nessuno` | `1000` | sì | `efficienza_riscossione` |
| `incentivo_investimenti` | sviluppo_economico | parametro | `0` · `10` · `20` · `40`*; in arrivo `0` da ottobre-dicembre 2028 | `40` | no | `investimenti_privati` in tutte le regioni, saldo |
| `concorrenza_servizi` | sviluppo_economico | regime | `licenze_chiuse`* · `gare` · `libere` | `licenze_chiuse` | sì | `investimenti_privati`, `sostegno_imprese` |
| `regime_pensioni` | lavoro_welfare | regime | `retributivo` · `contributivo`* · `quota_base_universale` | `retributivo` | sì | leva `regime_pensioni` |
| `eta_pensionabile` | lavoro_welfare | parametro | `62` … `70`, un gradino per anno, `67`* | `67` | sì | leva `eta_pensione` |
| `indicizzazione_pensioni` | lavoro_welfare | parametro | `piena` · `per_fasce`* · `bloccata_sopra_soglia` | `bloccata_sopra_soglia` | sì | leva `indicizzazione_pensioni` (1,0 · 0,95 · 0,7) |
| `pensione_minima` | lavoro_welfare | parametro | `attuale`* · `piu_20` · `piu_50` | `attuale` | sì | leva `pensione_minima` (1,0 · 1,2 · 1,5) |
| `sussidi_disoccupazione` | lavoro_welfare | regime | `assicurativo`* · `esteso` · `universale` | `assicurativo` | sì | leva `sussidi` (1,0 · 1,3 · 1,6, il valore di oggi di `sussidio_universale`) |
| `reddito_minimo` | lavoro_welfare | regime | `non_istituito` · `categoriale`* · `universale`; `dopo_abrogazione = "non_istituito"` | `universale` | sì | leva `reddito_minimo`, saldo |
| `salario_minimo` | lavoro_welfare | parametro | `nessuno`* · `7` · `9` · `11` | `nessuno` | sì | leva `salario_minimo`, `regole_lavoro` |
| `licenziamento` | lavoro_welfare | regime | `reintegro` · `tutele_crescenti`* · `solo_indennizzo` | `reintegro` | sì | `regole_lavoro` |
| `decontribuzione` | lavoro_welfare | parametro | `0` · `15` · `20`* · `30`; perimetro Mezzogiorno; in arrivo `15` da gennaio-marzo 2029, `0` da gennaio-marzo 2030 | `20` | no | leva `decontribuzione`, saldo |
| `ticket` | sanita | parametro | `nessuno` · `attuale`* · `alto` | `attuale` | sì | `qualita_servizio_sanitario`, saldo |
| `accesso_medicina` | sanita | regime | `numero_chiuso` · `semestre_filtro`* · `libero` | `numero_chiuso` | sì | `qualita_servizio_sanitario` (ritardo 24-40), saldo |
| `privato_accreditato` | sanita | parametro | `ridotto` · `attuale`* · `ampio` | `attuale` | sì | `qualita_servizio_sanitario`, saldo |
| `obbligo_scolastico` | istruzione_ricerca | parametro | `16`* · `18` | `16` | sì | `capitale_umano`, saldo |
| `tasse_universitarie` | istruzione_ricerca | parametro | `gratuite` · `per_isee`* · `alte` | `per_isee` | sì | `capitale_umano`, saldo |
| `codice_appalti` | infrastrutture_trasporti | regime | `rigido` · `semplificato`* · `deroga_generale` | `rigido` | sì | `qualita_trasporti`, `stock_infrastrutture` |
| `concessioni_autostradali` | infrastrutture_trasporti | regime | `pubbliche` · `a_gara`* · `private` | `a_gara` | sì | `qualita_trasporti` |
| `nucleare` | ambiente_energia | regime | `vietato`* · `consentito`; in arrivo `consentito` da ottobre-dicembre 2027 | `vietato` | sì | `prezzo_energia`, `sicurezza_energetica` (ritardo lungo) |
| `accise` | ambiente_energia | parametro | `basse` · `attuali`* · `alte` | `attuali` | no | leva `accise`, saldo (negativo con `alte`) |
| `rinnovabili` | ambiente_energia | regime | `ordinarie`* · `accelerate` | `ordinarie` | sì | `prezzo_energia`, `sicurezza_energetica` |
| `decreto_flussi` | interno | parametro | `bassi` · `attuali`* · `alti` | `attuali` | sì | `immigrazione_irregolare` |
| `cittadinanza` | interno | regime | `ius_sanguinis`* · `ius_scholae` · `ius_soli_temperato` | `ius_sanguinis` | sì | nessuno (scelta 10) |
| `accoglienza` | interno | regime | `centri`* · `diffusa` | `centri` | sì | `immigrazione_irregolare`, `sicurezza` |
| `prescrizione` | giustizia | regime | `classica` · `bloccata` · `improcedibilita`* | `bloccata` | sì | `durata_processi` |
| `pene_alternative` | giustizia | parametro | `ristrette` · `attuali`* · `estese` | `attuali` | sì | `sovraffollamento_carceri` |
| `intercettazioni` | giustizia | parametro | `ampie` · `limitate`* | `ampie` | sì | `corruzione` |
| `leva_militare` | esteri_difesa | regime | `sospesa`* · `volontaria_rafforzata` · `obbligatoria` | `obbligatoria` | sì | `prontezza_difesa`, saldo |
| `export_armi` | esteri_difesa | regime | `restrittivo`* · `permissivo` | `restrittivo` | sì | nessuno (scelta 10) |
| `credito_zes` | sviluppo_economico | parametro | `0` · `0_75` · `1` · `2_3`* (miliardi l'anno); perimetro ZES unica; in arrivo `1` da gennaio-marzo 2027, `0_75` da gennaio-marzo 2028 | `2_3` | no | `investimenti_privati` nel perimetro, saldo |
| `autonomia_differenziata` | fuori | regime | `non_attuata`* · `prime_materie` · `intese_complete`; perimetro Liguria, Lombardia, Piemonte, Veneto; `legge = ""` | `non_attuata` | sì | leva `autonomia` (0 · 0,05 · 0,15) |
| `durata_legislatura` | fuori, costituzionale | parametro | `4` · `5`* · `6` | `5` | no | leva `durata_legislatura` |
| `numero_parlamentari` | fuori, costituzionale | parametro | `300_150` · `400_200`* · `630_315` | `630_315` | no | leve `seggi_camera`, `seggi_senato` |

Perimetri, scritti come tabella inline nell'ordine dell'elenco (`perimetro = { abruzzo = 1.0, molise = 1.0, … }`: i test sostituiscono il loro inizio): Mezzogiorno = `abruzzo, molise, campania, puglia, basilicata, calabria, sicilia, sardegna`, peso 1; ZES unica = il Mezzogiorno con `abruzzo = 0.5`, più `marche = 0.5` e `umbria = 0.5` (intensità ridotta, `disposizioni-territoriali-al-turno-zero.md:18`). Dove il valore precedente reale non è un gradino o non è noto, il gradino più vicino; se coincide col vigente la disposizione non è bersaglio di referendum finché la partita non la cambia (spec riga 270). `legge` è la citazione della ricerca, accorciata come nelle righe verbatim sotto.

Verbatim (i test vi si appoggiano):

```toml
[[disposizione]]
id = "eta_pensionabile"
nome = "Età pensionabile"
dicastero = "lavoro_welfare"
tipo = "parametro"
norma = "ordinaria"
referendabile = true
gruppi = ["pensionato", "dipendente_privato", "dipendente_pubblico", "autonomo"]
partenza = "67"
legge = "L. 199/2025, art. 1 c. 185"
precedente = "67"
in_arrivo = []
gradini = [
  { id = "62", nome = "62 anni", posizione = 2.0, saldo = 0.0, leve = { eta_pensione = 62.0 } },
  { id = "63", nome = "63 anni", posizione = 2.5, saldo = 0.0, leve = { eta_pensione = 63.0 } },
  { id = "64", nome = "64 anni", posizione = 3.0, saldo = 0.0, leve = { eta_pensione = 64.0 } },
  { id = "65", nome = "65 anni", posizione = 3.5, saldo = 0.0, leve = { eta_pensione = 65.0 } },
  { id = "66", nome = "66 anni", posizione = 4.0, saldo = 0.0, leve = { eta_pensione = 66.0 } },
  { id = "67", nome = "67 anni", posizione = 4.5, saldo = 0.0, leve = { eta_pensione = 67.0 } },
  { id = "68", nome = "68 anni", posizione = 5.0, saldo = 0.0, leve = { eta_pensione = 68.0 } },
  { id = "69", nome = "69 anni", posizione = 5.5, saldo = 0.0, leve = { eta_pensione = 69.0 } },
  { id = "70", nome = "70 anni", posizione = 6.0, saldo = 0.0, leve = { eta_pensione = 70.0 } },
]

[[disposizione]]
id = "licenziamento"
nome = "Licenziamento"
dicastero = "lavoro_welfare"
tipo = "regime"
norma = "ordinaria"
referendabile = true
gruppi = ["dipendente_privato"]
partenza = "tutele_crescenti"
legge = "D.Lgs. 23/2015, art. 3"
precedente = "reintegro"
in_arrivo = []
gradini = [
  { id = "reintegro", nome = "reintegro", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = "regole_lavoro", intensita = -0.3, ritardo = 4 }] },
  { id = "tutele_crescenti", nome = "tutele crescenti", posizione = 4.0, saldo = 0.0 },
  { id = "solo_indennizzo", nome = "solo indennizzo", posizione = 5.5, saldo = 0.0, effetti = [{ grandezza = "regole_lavoro", intensita = 0.3, ritardo = 4 }] },
]

[[disposizione]]
id = "reddito_minimo"
nome = "Reddito minimo"
dicastero = "lavoro_welfare"
tipo = "regime"
norma = "ordinaria"
referendabile = true
gruppi = ["disoccupato_inattivo"]
partenza = "categoriale"
legge = "DL 48/2023, artt. 1-13"
precedente = "universale"
dopo_abrogazione = "non_istituito"
in_arrivo = []
gradini = [
  { id = "non_istituito", nome = "non istituito", posizione = 5.5, saldo = -0.35, leve = { reddito_minimo = 0.0 } },
  { id = "categoriale", nome = "categoriale", posizione = 4.0, saldo = 0.0, leve = { reddito_minimo = 1200.0 } },
  { id = "universale", nome = "universale", posizione = 2.0, saldo = 0.3, leve = { reddito_minimo = 2400.0 } },
]

[[disposizione]]
id = "privato_accreditato"
nome = "Privato accreditato"
dicastero = "sanita"
tipo = "parametro"
norma = "ordinaria"
referendabile = true
gruppi = ["pensionato"]
partenza = "attuale"
legge = "L. 199/2025, art. 1 c. 400"
precedente = "attuale"
in_arrivo = []
gradini = [
  { id = "ridotto", nome = "ridotto", posizione = 2.0, saldo = -0.05, effetti = [{ grandezza = "qualita_servizio_sanitario", intensita = -2.0, ritardo = 2 }] },
  { id = "attuale", nome = "attuale", posizione = 4.0, saldo = 0.0 },
  { id = "ampio", nome = "ampio", posizione = 6.0, saldo = 0.05, effetti = [{ grandezza = "qualita_servizio_sanitario", intensita = 3.0, ritardo = 1 }] },
]

[[disposizione]]
id = "nucleare"
nome = "Nucleare"
dicastero = "ambiente_energia"
tipo = "regime"
norma = "ordinaria"
referendabile = true
gruppi = []
partenza = "vietato"
legge = "L. 169/2026, art. 1"
precedente = "vietato"
in_arrivo = [{ gradino = "consentito", anno = 2027, trimestre = 4 }]
gradini = [
  { id = "vietato", nome = "vietato", posizione = 3.0, saldo = 0.0 },
  { id = "consentito", nome = "consentito", posizione = 5.0, saldo = 0.0, effetti = [{ grandezza = "prezzo_energia", intensita = -3.0, ritardo = 40 }, { grandezza = "sicurezza_energetica", intensita = 4.0, ritardo = 40 }] },
]

[[disposizione]]
id = "numero_parlamentari"
nome = "Numero dei parlamentari"
tipo = "parametro"
norma = "costituzionale"
referendabile = false
gruppi = []
partenza = "400_200"
legge = "L. cost. 1/2020"
precedente = "630_315"
in_arrivo = []
gradini = [
  { id = "300_150", nome = "300 deputati e 150 senatori", posizione = 5.5, saldo = 0.0, leve = { seggi_camera = 300.0, seggi_senato = 150.0 } },
  { id = "400_200", nome = "400 deputati e 200 senatori", posizione = 4.5, saldo = 0.0, leve = { seggi_camera = 400.0, seggi_senato = 200.0 } },
  { id = "630_315", nome = "630 deputati e 315 senatori", posizione = 3.0, saldo = 0.0, leve = { seggi_camera = 630.0, seggi_senato = 315.0 } },
]
```

Il nucleare non ha precedente reale: «vietato» coincide col vigente, quindi un referendum sulla L. 169/2026 prima di ottobre 2027 cancella solo il valore in arrivo (spec riga 344). La decontribuzione in arrivo 2028 cambia il tetto mensile, non la percentuale: non è un gradino e non entra (semplificazione dichiarata).

Regole per le altre 31, che il Task 2 valida:
- `posizione` da 1.0 a 7.0, coerente con la scala dell'asset (sinistra più bassa); gradini ordinati per quantità hanno posizioni monotone;
- `saldo` fra −2 e +2 (% del PIL potenziale), 0 alla partenza; nessun saldo sulle disposizioni le cui leve il motore conta già nei conti (pensioni e sussidi), saldo sulle altre leve che pesano sui conti (cuneo, forfettario, reddito minimo, accise, decontribuzione) e su ogni disposizione che costa o rende senza leva;
- `effetti` sulle grandezze nazionali solo se hanno il dicastero della disposizione nella colonna `dicasteri` (per quelle fuori dicastero, nessun limite); sulle grandezze regionali, applicate sul perimetro o in tutte le regioni senza regione bersaglio, nessun limite di dicastero (così `detrazione_edilizia`, di economia_finanze, su `domanda_regionale`); intensità con la scala degli atti (indici 2 / 4 / 8 lieve, moderato, forte; `domanda_regionale` 0.3 / 0.6 / 1.0; `prezzo_energia` −1 / −2 / −4; `politiche_attive` e `regole_lavoro` 0.1 / 0.2 / 0.3), segno nel verso della grandezza; ritardo in turni, da 1 a 40;
- una leva sta in tutti i gradini della sua disposizione, col valore del gradino;
- leve senza valore nella tabella: `forfettario` 1,0 al gradino di partenza (moltiplica le imposte degli autonomi: sotto 1 coi gradini più larghi, sopra 1 con `assente`); `accise` 0 al gradino di partenza (punti sul prezzo dell'energia, negativi con `basse`); `decontribuzione` in punti di disoccupazione strutturale, 0 al gradino `0` e crescente col gradino, con la scala di `regole_lavoro`;
- `gruppi` sono le condizioni di `elettorato.toml` che la disposizione tocca, per l'anteprima;
- nomi dei gradini in minuscolo, come si leggono in una frase («67 anni», «ius scholae»); nessuna legge vera col suo nome nel `nome`, che resta generico.

- [ ] **Step 1: Scrivi `dati/base/disposizioni.toml`**

Intestazione: `# Disposizioni della v1 (ticket 16, 22, 24, 27). Valori di partenza, leggi e precedenti da docs/ricerca/sintesi/disposizioni-al-turno-zero.md e disposizioni-territoriali-al-turno-zero.md; posizioni, saldi ed effetti sono una bozza del piano 6 sotto revisione di Mario.`

- [ ] **Step 2: Controlla la forma del file**

Run: `python3 -I -c 'import tomllib;d=tomllib.load(open("dati/base/disposizioni.toml","rb"))["disposizione"];ids=[x["id"] for x in d];g=lambda x:[y["id"] for y in x["gradini"]];print(len(d),len(set(ids)),all(x["partenza"] in g(x) and x["precedente"] in g(x) for x in d),sum(not x["referendabile"] for x in d),sorted({k for x in d for y in x["gradini"] for k in y.get("leve",{})})==sorted(["regime_pensioni","eta_pensione","indicizzazione_pensioni","pensione_minima","sussidi","reddito_minimo","salario_minimo","cuneo","forfettario","accise","decontribuzione","autonomia","durata_legislatura","seggi_camera","seggi_senato"]))'`
Expected: `37 37 True 9 True`.

- [ ] **Step 3: Commit**

```bash
git add dati/base/disposizioni.toml
git commit -m "feat(dati): draft catalogue of the 37 dispositions"
```

---

### Task 2: Catalogo caricato e validato

**Files:**
- Create: `crates/sim-core/src/disposizioni/mod.rs`
- Modify: `crates/sim-core/src/dati.rs` (`FileDati.disposizioni`, `NOMI` a 10 voci, `DatiGioco::disposizioni()`, `disposizione()`, validazione, elettorali generate)
- Modify: `crates/sim-core/src/scenario.rs`, `dati/base/scenario.toml` (`legge_elettorale_precedente`)
- Modify: `crates/sim-core/src/elezioni/mod.rs` (`LeggeElettorale::valida` chiamata anche sul precedente)
- Modify: `crates/sim-core/src/errore.rs` (variante `Disposizioni`)
- Modify: `dati/base/grandezze.toml` (riga `regole_lavoro`), `crates/sim-core/tests/grandezze.rs` (21 grandezze)
- Modify: `crates/sim-core/src/lib.rs`, `crates/sim-core/tests/comune/mod.rs` (`include_str!` e ramo `"disposizioni"`)
- Test: `crates/sim-core/tests/disposizioni.rs`

**Ricerca:** `docs/ricerca/tecnica/modding-e-dati-esterni.md:14-15` (TOML, validazione all'avvio col nome); `docs/ricerca/modelli/sistemi-elettorali.md:12-14` (i parametri del menu); `.scratch/v1-giocabile/issues/13-legge-elettorale-come-misura.md:39-41` (le elettorali come disposizioni fuori dicastero, precedenti del Rosatellum).

**Dispatch:** `rust-engineer` · dopo il commit del Task 1 · skill-gate: true.

**Interfaces:**
- Consumes: `DatiElezioni.menu` e `impostazioni` (`crates/sim-core/src/elezioni/mod.rs:58-92`), `Scenario.legge_elettorale`, `DatiGioco::grandezze()`, `Condizione` (serde `snake_case`, `crates/sim-core/src/elettorato/mod.rs:29-37`).
- Produces:
  - `sim_core::disposizioni::DatoDisposizione { pub id: String, pub nome: String, pub dicastero: Option<String>, pub tipo: TipoDisposizione, pub norma: Norma, pub referendabile: bool, pub gruppi: Vec<Condizione>, pub perimetro: BTreeMap<String, f64>, pub partenza: String, pub legge: String, pub precedente: String, pub dopo_abrogazione: Option<String>, pub in_arrivo: Vec<Arrivo>, pub gradini: Vec<Gradino> }`; `Gradino { pub id: String, pub nome: String, pub posizione: Option<f64>, pub saldo: f64, pub leve: BTreeMap<String, f64>, pub effetti: Vec<EffettoDisposizione> }`; `EffettoDisposizione { pub grandezza: String, pub intensita: f64, pub ritardo: u32 }`; `Arrivo { pub gradino: String, pub anno: u16, pub trimestre: u8 }`; `TipoDisposizione { Parametro, Regime }`, `Norma { Ordinaria, Costituzionale, Elettorale }`, serde `snake_case`, `Copy`, TS esportati. Default serde: `perimetro`, `leve`, `effetti`, `gruppi` vuoti; `dicastero`, `dopo_abrogazione` assenti; `norma` mai scritta `elettorale` nel file (rifiutata al parsing, in `da_testi`: `{id}: norma elettorale solo dal menu`).
  - `DatoDisposizione::gradino(&self, id: &str) -> Option<usize>`; `DatoDisposizione::posizione(&self, g: usize) -> Option<f64>`.
  - `DatiGioco::disposizioni() -> &[DatoDisposizione]`: le 37 del file nel loro ordine, poi le 9 elettorali; `DatiGioco::disposizione(id: &str) -> Option<usize>`.
  - `pub const LEVE: [&str; 15]` in `disposizioni/mod.rs`, la colonna «leva» della tabella del Task 1.
  - `Scenario.legge_elettorale_precedente: LeggeElettorale`, in `scenario.toml` dopo la riga `coalizioni = [[…]]` (`dati/base/scenario.toml:12`) e prima di `[governo]`: subito dopo `legge_elettorale` la tabella nuova si prenderebbe la chiave `coalizioni` top-level che segue, e il TOML non si parserebbe più (chiave ripetuta):
    ```toml
    # Valori precedenti al turno zero: Rosatellum (L. 165/2017) senza collegi, ticket 13:41
    [legge_elettorale_precedente]
    soglia_lista_pb = 300
    coalizioni = true
    soglia_coalizione_pb = 1000
    ripescaggio = false
    premio = "nessuno"
    soglia_premio_pb = 4200
    taglia_premio_pb = 1750
    formula = "hare"
    ```
  - `Errore::Disposizioni(String)` → `disposizioni non valide: {0}`.

Le 9 elettorali, generate in `DatiGioco::da_testi` dopo il parsing, in quest'ordine: il rifiuto `{id}: norma elettorale solo dal menu` sulle voci del file, `elezioni.valida` sul menu, poi la generazione in coda alle 37. `norma = Elettorale`, `dicastero = None`, `referendabile = true`, `legge = "L. 180/2026"`, `posizione = None` e `saldo = 0` in ogni gradino, `gruppi`, `perimetro`, `in_arrivo` vuoti, partenza dall'impostazione dello scenario e precedente da `legge_elettorale_precedente`, gradini e nomi:

| id | nome | tipo | gradini (id → nome) |
|---|---|---|---|
| `soglia_lista` | Soglia di lista | parametro | menu `soglia_lista_pb`, id = il numero in punti base, nome «3%» |
| `coalizioni` | Coalizioni | regime | `no` → «non ammesse», `si` → «ammesse» |
| `soglia_coalizione` | Soglia di coalizione | parametro | menu `soglia_coalizione_pb` |
| `ripescaggio` | Ripescaggio | regime | `no`, `si` |
| `premio` | Premio di maggioranza | regime | `nessuno`, `lista` → «alla lista», `coalizione` → «alla coalizione» |
| `soglia_premio` | Soglia del premio | parametro | menu `soglia_premio_pb` |
| `taglia_premio` | Taglia del premio | parametro | menu `taglia_premio_pb` |
| `tetto` | Tetto ai seggi | parametro | menu `tetto` in ordine, id `{camera_pb}_{senato_pb}`, nome «55% alla Camera, 56,5% al Senato»; poi `nessuno` |
| `formula` | Formula di riparto | regime | `hare` → «quozienti Hare», `dhondt` → «D'Hondt» |

Controlli, in `DatiGioco::valida` dopo le grandezze e prima degli atti, con `self.elezioni.valida` anticipata prima di essi (oggi è in fondo, `crates/sim-core/src/dati.rs:546`). `valida` accetta le 9 elettorali generate: `Partita::carica` la riesegue sui dati del salvataggio (`crates/sim-core/src/partita.rs:575`), che le contengono. In quest'ordine, messaggi esatti dentro `Errore::Disposizioni`: id ripetuto (`disposizione ripetuta: {id}`); dicastero sconosciuto (`{id}: dicastero sconosciuto {d}`); 2-10 gradini, id distinti (`{id}: servono da 2 a 10 gradini distinti`); `partenza`, `precedente`, `dopo_abrogazione`, gradini di `in_arrivo` fra i gradini (`{id}: gradino sconosciuto {g}`); per gradino: `posizione` presente e finita in 1-7 tranne le elettorali (`{id}: posizione di {g} fuori da 1-7`), `saldo` finito in −2..2 (`{id}: saldo di {g} fuori intervallo`), leve fra `LEVE` (`{id}: leva sconosciuta {l}`) e finite, effetti su grandezze esistenti (`{id}: grandezza sconosciuta {g}`) con intensità finita e ritardo in 1-40 (`{id}: effetto su {g} fuori intervallo`) e, se la grandezza è nazionale, il dicastero nella colonna `dicasteri` della grandezza (`{id}: {g} non è nella riga del dicastero {d}`; le grandezze regionali non hanno limite di dicastero); poi, per disposizione, ogni leva in tutti i gradini (`{id}: leva {l} non è in tutti i gradini`); perimetro con regioni note (`{id}: regione sconosciuta {r}`) e pesi in (0, 1] (`{id}: peso di {r} fuori da (0, 1]`); arrivi dopo il turno zero, in ordine strettamente crescente, trimestre 1-4 (`{id}: arrivi fuori ordine`); norma costituzionale non referendabile e fuori dicastero (`{id}: norma costituzionale referendabile o con dicastero`); infine ogni nome di `LEVE` in esattamente una disposizione (`leva richiesta mancante: {l}`, `leva in due disposizioni: {l}`). La legge elettorale precedente passa `LeggeElettorale::valida` (`Errore::LeggeElettorale` col prefisso `legge elettorale precedente: `).

La riga nuova di `grandezze.toml`, dopo `corruzione`: `id = "regole_lavoro"`, `nome = "Regole del lavoro (punti di disoccupazione strutturale tolti)"`, `dinamica = "fissa"`, `verso = "sale"`, `regionale = false`, `iniziale = 0.0`, `min = -5.0`, `max = 5.0`, `dicasteri = ["lavoro_welfare"]`. Il motore la legge dal Task 4.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/disposizioni.rs
mod comune;
use comune::{dati_base, file_base, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::disposizioni::{Norma, TipoDisposizione};
use sim_core::Errore;

fn rifiuto(file: &str, da: &str, a: &str) -> String {
    let e = DatiGioco::da_testi(&sostituisci(file_base(), file, da, a)).unwrap_err();
    assert!(matches!(e, Errore::Disposizioni(_)), "{e}");
    e.to_string()
}

#[test]
fn catalogo_base_si_carica() {
    let d = dati_base();
    let tutte = d.disposizioni();
    assert_eq!(tutte.len(), 46);
    let conta = |n: Norma| tutte.iter().filter(|x| x.norma == n).count();
    assert_eq!((conta(Norma::Ordinaria), conta(Norma::Costituzionale), conta(Norma::Elettorale)), (35, 2, 9));
    let fiscali = ["forfettario", "cuneo_fiscale", "detrazione_edilizia", "accise", "incentivo_investimenti", "decontribuzione", "credito_zes"];
    for x in tutte {
        let atteso = !fiscali.contains(&x.id.as_str()) && x.norma != Norma::Costituzionale;
        assert_eq!(x.referendabile, atteso, "{}", x.id);
    }
    let n = &tutte[d.disposizione("nucleare").unwrap()];
    let a = &n.in_arrivo[0];
    assert_eq!((n.partenza.as_str(), a.gradino.as_str(), a.anno, a.trimestre), ("vietato", "consentito", 2027, 4));
    let r = &tutte[d.disposizione("reddito_minimo").unwrap()];
    assert_eq!(r.dopo_abrogazione.as_deref(), Some("non_istituito"));
    assert_eq!(tutte[d.disposizione("decontribuzione").unwrap()].perimetro.len(), 8);
}

#[test]
fn disposizioni_elettorali_dal_menu_e_dallo_scenario() {
    let d = dati_base();
    let s = &d.disposizioni()[d.disposizione("soglia_lista").unwrap()];
    let ids: Vec<&str> = s.gradini.iter().map(|g| g.id.as_str()).collect();
    assert_eq!(ids, ["0", "300", "400", "500"]);
    assert_eq!((s.partenza.as_str(), s.precedente.as_str(), s.legge.as_str()), ("300", "300", "L. 180/2026"));
    let p = &d.disposizioni()[d.disposizione("premio").unwrap()];
    assert_eq!((p.partenza.as_str(), p.precedente.as_str(), p.tipo), ("coalizione", "nessuno", TipoDisposizione::Regime));
    assert!(p.gradini.iter().all(|g| g.posizione.is_none()));
    let t = &d.disposizioni()[d.disposizione("tetto").unwrap()];
    assert_eq!((t.partenza.as_str(), t.precedente.as_str()), ("5500_5650", "nessuno"));
}

#[test]
fn gradino_sconosciuto_rifiutato_col_nome() {
    let m = rifiuto("disposizioni", "partenza = \"67\"", "partenza = \"66,5\"");
    assert!(m.contains("eta_pensionabile: gradino sconosciuto 66,5"), "{m}");
}

#[test]
fn leva_sconosciuta_rifiutata_col_nome() {
    let m = rifiuto("disposizioni", "leve = { eta_pensione = 62.0 }", "leve = { eta_pensionabile = 62.0 }");
    assert!(m.contains("eta_pensionabile: leva sconosciuta eta_pensionabile"), "{m}");
}

#[test]
fn effetto_fuori_dalla_riga_del_dicastero_rifiutato() {
    let m = rifiuto(
        "disposizioni",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"regole_lavoro\"",
        "nome = \"reintegro\", posizione = 2.0, saldo = 0.0, effetti = [{ grandezza = \"qualita_scuola\"",
    );
    assert!(m.contains("licenziamento: qualita_scuola non è nella riga del dicastero lavoro_welfare"), "{m}");
}

#[test]
fn regione_del_perimetro_sconosciuta_rifiutata() {
    let m = rifiuto("disposizioni", "perimetro = { abruzzo = 1.0", "perimetro = { abruzo = 1.0");
    assert!(m.contains("decontribuzione: regione sconosciuta abruzo"), "{m}");
}

#[test]
fn legge_elettorale_precedente_fuori_menu_rifiutata() {
    let f = sostituisci(file_base(), "scenario", "[legge_elettorale_precedente]\nsoglia_lista_pb = 300", "[legge_elettorale_precedente]\nsoglia_lista_pb = 250");
    let e = DatiGioco::da_testi(&f).unwrap_err();
    assert!(matches!(e, Errore::LeggeElettorale(_)) && e.to_string().contains("legge elettorale precedente: soglia_lista_pb"), "{e}");
}
```

Più due test d'unità in `disposizioni/mod.rs` su un catalogo costruito a mano: `leve_richieste_una_volta_sola` (una leva in due disposizioni, una leva mancante: i due messaggi) e `leva_in_un_gradino_solo_rifiutata`. La sostituzione `perimetro = { abruzzo = 1.0` assume il perimetro della decontribuzione scritto con `abruzzo` per primo, come nell'elenco del Task 1.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test disposizioni`
Expected: errore di compilazione, `sim_core::disposizioni` non esiste.

- [ ] **Step 3: Implementa `disposizioni/mod.rs`, le elettorali generate, la validazione e la riga di `regole_lavoro`**

`grandezze_base_si_caricano` passa a 21 grandezze.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add dati/base crates/sim-core
git commit -m "feat(core): load and validate the dispositions catalogue"
```

---

### Task 3: Le disposizioni nella partita

**Files:**
- Create: `crates/sim-core/src/disposizioni/stato.rs`
- Modify: `crates/sim-core/src/partita.rs` (campo `disposizioni`, `VERSIONE_SCHEMA = 6`, arrivi in apertura, `proiezione_seggi` sulla legge in vigore, `in_vigore()`, `legge_elettorale()`, `carica`)
- Modify: `crates/sim-core/src/atti.rs`, `dati/base/atti.toml` (prerequisito), `crates/sim-core/src/ministri/agenda.rs`, `ministri/proposte.rs` (atti col prerequisito mancante non pagabili), `crates/sim-core/src/ministri/mod.rs` (`Contesto.disposizioni`)
- Modify: `crates/sim-core/src/vista.rs` (`VoceDisposizione`, `VoceAtto.prerequisito_mancante`)
- Modify: `crates/sim-core/tests/comune/mod.rs` (`con_arrivo`)
- Test: `crates/sim-core/tests/stato_disposizioni.rs`

**Ricerca:** `docs/ricerca/sintesi/disposizioni-al-turno-zero.md:84-86` (nucleare in arrivo, incentivo che scade); `.scratch/v1-giocabile/issues/24-abrogazione-e-gradini-di-partenza.md:37` (valore in arrivo).

**Dispatch:** `rust-engineer` · dopo il Task 2 · skill-gate: true.

**Interfaces:**
- Consumes: `DatiGioco::disposizioni()`, `Trimestre`, `riparto` e `LeggeElettorale` (`crates/sim-core/src/partita.rs:434-463`), `Contesto` dei ministri (`crates/sim-core/src/ministri/mod.rs:113-124`).
- Produces:
  - `sim_core::disposizioni::StatoDisposizione { pub in_vigore: usize, pub precedente: usize, pub legge: String, pub in_arrivo: Vec<InArrivo>, pub bloccati: Vec<usize> }` (gradini per indice; `bloccati` = gradini abrogati da un referendum, Task 14); `InArrivo { pub gradino: usize, pub turno: u32, pub legge: String }` (gradino, turno d'apertura in cui arriva, legge con cui arriva); `Disposizioni { pub voci: Vec<StatoDisposizione> }`, `Clone, Debug, PartialEq, Serialize, Deserialize`.
  - `Disposizioni::iniziali(dati: &DatiGioco) -> Disposizioni`: dai dati, col turno di ogni arrivo = numero di trimestri fra il turno zero e `(anno, trimestre)` e la legge = la `legge` della disposizione.
  - `Disposizioni::cambia(&mut self, dati: &DatiGioco, d: usize, g: usize, turno: u32, legge: &str) -> Vec<EffettoAttivo>`: `precedente ← in_vigore`, `in_vigore ← g`, `legge ← legge`, `in_arrivo` svuotato; rende gli effetti (vuoti fino al Task 4, che li fa partire da `turno`). Nessun effetto se `g == in_vigore` (resta solo la legge nuova).
  - `Disposizioni::programma(&mut self, d: usize, g: usize, turno: u32, legge: &str)`: gli arrivi della disposizione `d` diventano uno solo, `g` al turno d'apertura `turno` con la legge `legge`; quelli già presenti, anche dai dati, si cancellano. Nessun effetto subito; la lista resta in ordine, quindi `controlla` e `salva()` restano validi. Lo usa il rinvio del Task 8.
  - `Disposizioni::arrivi(&mut self, dati: &DatiGioco, turno: u32) -> Vec<EffettoAttivo>`: ogni arrivo con turno ≤ `turno`, in ordine, come `cambia` con la legge dell'arrivo; l'arrivo esce dalla lista. Lo scarto degli arrivi elettorali non validi lo aggiunge il Task 9.
  - `Disposizioni::legge_elettorale(&self, dati: &DatiGioco) -> LeggeElettorale` (dai 9 gradini in vigore); `Disposizioni::posizione(&self, dati, d) -> Option<f64>` (del gradino in vigore); `Disposizioni::controlla(&self, dati) -> Result<(), Errore>`.
  - `Partita::disposizioni() -> &Disposizioni`; `Partita::in_vigore(id: &str) -> Option<&str>` (id del gradino); `Partita::legge_elettorale() -> LeggeElettorale`.
  - `Atto.prerequisito: Option<Prerequisito>`, `Prerequisito { pub disposizione: String, pub gradini: Vec<String> }`; validato in `atti::valida` (`{id}: prerequisito su disposizione sconosciuta {d}`, `{id}: prerequisito su gradino sconosciuto {g}`).
  - `Contesto.disposizioni: &'a Disposizioni` (tutti i costruttori di `Contesto` in `partita.rs`).
  - `sim_core::vista::VoceDisposizione { pub id: String, pub nome: String, pub dicastero: Option<String>, pub norma: Norma, pub tipo: TipoDisposizione, pub in_vigore: String, pub legge: String, pub precedente: String, pub in_arrivo: Vec<VoceArrivo>, pub referendabile: bool, pub perimetro: Vec<String>, pub gradini: Vec<VoceGradino> }`, `VoceArrivo { pub gradino: String, pub data: Trimestre }`, `VoceGradino { pub id: String, pub nome: String, pub posizione: Option<f64>, pub bloccato: bool }`; `Vista.disposizioni: Vec<VoceDisposizione>`, nell'ordine dei dati. `VoceAtto.prerequisito_mancante: Option<String>`.

Regole:
- **Arrivi**: in `chiudi_turno`, subito dopo `turno += 1` e prima di `ministri.apri`, così le agende vedono i prerequisiti nuovi; gli effetti resi entrano in `grandezze.effetti`.
- **Proiezione**: `proiezione_seggi` usa `self.legge_elettorale()` invece di `dati.legge_elettorale_iniziale()`; i seggi totali li sposta il Task 9.
- **Prerequisito** (spec riga 268): l'atto il cui prerequisito non è soddisfatto (gradino in vigore fuori da `gradini`) non è pagabile né eleggibile per l'agenda, il dirottamento o una proposta di fondi. `prerequisito_mancante = Some("serve {nome della disposizione}: {nomi dei gradini uniti da \" o \"}")`. In `atti.toml`, all'atto `sanita_convenzioni_privato` la riga `prerequisito = { disposizione = "privato_accreditato", gradini = ["attuale", "ampio"] }` come ultima riga dell'atto, dopo la chiusura di `effetti`: le sostituzioni dei test del piano 5 (`crates/sim-core/tests/proposte.rs`, `sanita_cara`) toccano il testo fra `costo` ed `effetti`.
- **Controllo del salvataggio**: tante voci quante disposizioni, gradini in vigore, precedenti, in arrivo e bloccati dentro i gradini, arrivi in ordine, la legge elettorale in vigore valida.

`con_arrivo` per i test, in `tests/comune/mod.rs`:

```rust
/// Nella disposizione `id`, la riga `in_arrivo = ...` diventa un solo arrivo `gradino` dal
/// trimestre dato: cambia una disposizione a un turno noto senza passare dall'aula.
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn con_arrivo(mut f: FileDati, id: &str, gradino: &str, anno: u16, trimestre: u8) -> FileDati {
    let testa = format!("id = \"{id}\"\n");
    let inizio = f.disposizioni.find(&testa).unwrap_or_else(|| panic!("{id} assente")) + testa.len();
    let riga = inizio + f.disposizioni[inizio..].find("in_arrivo = ").expect("in_arrivo");
    let fine = riga + f.disposizioni[riga..].find('\n').expect("fine riga");
    f.disposizioni.replace_range(riga..fine, &format!("in_arrivo = [{{ gradino = \"{gradino}\", anno = {anno}, trimestre = {trimestre} }}]"));
    f
}
```

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/stato_disposizioni.rs
mod comune;
use comune::{con_arrivo, dati_base, edita, file_base, sostituisci};
use sim_core::dati::DatiGioco;
use sim_core::disposizioni::Disposizioni;
use sim_core::{Errore, Partita};

#[test]
fn disposizioni_al_turno_zero_nella_vista() {
    let p = Partita::nuova(dati_base(), 1);
    let v = p.vista();
    assert_eq!(v.disposizioni.len(), 46);
    let e = v.disposizioni.iter().find(|d| d.id == "eta_pensionabile").unwrap();
    assert_eq!((e.in_vigore.as_str(), e.legge.as_str(), e.precedente.as_str()), ("67", "L. 199/2025, art. 1 c. 185", "67"));
    assert_eq!(p.in_vigore("nucleare"), Some("vietato"));
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert_eq!(n.in_arrivo[0].gradino, "consentito");
}

#[test]
fn valore_in_arrivo_arriva_al_suo_turno() {
    let mut p = Partita::nuova(dati_base(), 1);
    for _ in 0..3 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("nucleare"), Some("vietato"));
    p.chiudi_turno();
    assert_eq!(p.in_vigore("nucleare"), Some("consentito"));
    let v = p.vista();
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert_eq!((n.precedente.as_str(), n.legge.as_str()), ("vietato", "L. 169/2026, art. 1"));
    assert!(n.in_arrivo.is_empty());
}

#[test]
fn proiezione_con_la_legge_in_vigore() {
    let f = sostituisci(file_base(), "scenario", "legge_elettorale = \"legge_180\"", "legge_elettorale = \"proporzionale_puro\"");
    let d = DatiGioco::da_testi(&f).unwrap();
    let p = Partita::nuova(d.clone(), 1);
    assert_eq!(p.legge_elettorale(), d.elezioni().impostazioni["proporzionale_puro"]);
    assert!(p.proiezione_seggi().unwrap().premio.is_none());
}

#[test]
fn arrivo_scritto_nei_dati_cambia_la_disposizione() {
    let f = con_arrivo(file_base(), "eta_pensionabile", "70", 2027, 1);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("70"));
}

#[test]
fn arrivo_programmato_sostituisce_gli_altri_e_porta_la_sua_legge() {
    // Credito ZES: nei dati `1` al turno 1 e `0_75` al turno 5; l'arrivo programmato li sostituisce.
    let d = dati_base();
    let k = d.disposizione("credito_zes").unwrap();
    let g = |id: &str| d.disposizioni()[k].gradino(id).unwrap();
    let mut s = Disposizioni::iniziali(&d);
    s.programma(k, g("0"), 3, "L. 7/2027");
    assert_eq!(s.voci[k].in_arrivo.len(), 1);
    s.controlla(&d).unwrap();
    s.arrivi(&d, 2);
    assert_eq!(s.voci[k].in_vigore, g("2_3"));
    s.arrivi(&d, 3);
    assert_eq!((s.voci[k].in_vigore, s.voci[k].legge.as_str()), (g("0"), "L. 7/2027"));
    assert!(s.voci[k].in_arrivo.is_empty());
}

#[test]
fn atto_senza_prerequisito_non_va_in_agenda() {
    let f = sostituisci(file_base(), "disposizioni", "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"", "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"");
    for seme in 1..=5 {
        let v = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme).vista();
        let m = v.ministeri.iter().find(|m| m.dicastero == "sanita").unwrap();
        let c = m.atti.iter().find(|a| a.id == "sanita_convenzioni_privato").unwrap();
        assert!(!c.pagabile);
        assert_eq!(c.prerequisito_mancante.as_deref(), Some("serve Privato accreditato: attuale o ampio"));
        assert_ne!(m.agenda.as_ref().map(|a| a.atto.as_str()), Some("sanita_convenzioni_privato"));
    }
}

#[test]
fn salvataggio_con_disposizioni_guaste_illeggibile() {
    let p = Partita::nuova(dati_base(), 1);
    for guasto in [
        Box::new(|j: &mut serde_json::Value| { j["disposizioni"]["voci"].as_array_mut().unwrap().pop(); }) as Box<dyn Fn(&mut serde_json::Value)>,
        Box::new(|j: &mut serde_json::Value| j["disposizioni"]["voci"][0]["in_vigore"] = serde_json::json!(99)),
    ] {
        let e = edita(&p, |j| guasto(j)).unwrap_err();
        assert!(matches!(e, Errore::SalvataggioIlleggibile(_)), "{e}");
    }
}
```

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test stato_disposizioni`
Expected: errore di compilazione, `Vista.disposizioni` e `Partita::in_vigore` non esistono.

- [ ] **Step 3: Implementa stato, arrivi, proiezione, prerequisiti e vista**

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS; `app/src/lib/tipi/VoceDisposizione.ts` nuovo.

- [ ] **Step 5: Commit**

```bash
git add dati/base/atti.toml crates/sim-core app/src/lib/tipi
git commit -m "feat(core): dispositions in the game state, scheduled values and prerequisites"
```

---

### Task 4: Effetti delle disposizioni sul motore

**Files:**
- Create: `crates/sim-core/src/disposizioni/effetti.rs` (`Leve`, effetti di un cambio)
- Modify: `crates/sim-core/src/atti.rs` (`Forma::Livello`, rifiutata negli atti), `crates/sim-core/src/grandezze.rs` (contributo di livello, `EffettoAttivo.dicastero: Option<usize>`)
- Modify: `crates/sim-core/src/misure.rs` (`leve`, `leve_partenza`, accise, `regole_lavoro`)
- Modify: `crates/sim-core/src/economia/previdenza.rs`, `economia/mod.rs` (`Economia::iniziale` con le leve di partenza), `economia/finanza.rs` (`Spese.disposizioni`), `economia/regole_ue.rs` (spesa netta)
- Modify: `crates/sim-core/src/elettorato/redditi.rs` (cuneo, forfettario, salario minimo, reddito minimo)
- Modify: `crates/sim-core/src/scenario.rs`, `dati/base/scenario.toml` (via `regime_pensioni`, `regime_sussidi`, `eta_pensione`, `indicizzazione_pensioni`), `crates/sim-core/src/dati.rs` (via `sussidio_universale`, più `regole_lavoro` fra le grandezze richieste), `dati/base/manopole.toml`
- Modify: `crates/sim-core/tests/conti.rs` (i test sui regimi passano da `scenario` a `con_arrivo` o alla `partenza` in `disposizioni`)
- Test: `crates/sim-core/tests/effetti_disposizioni.rs`

**Ricerca:** `docs/ricerca/modelli/finanza-pubblica.md:34` (indicizzazione delle pensioni in essere), `:44` (la formula cambia la pensione media solo per le coorti future); `docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:17-18` (ritardo prima dell'effetto); spec righe 298 (salario minimo nel reddito primario della fascia bassa dei dipendenti privati), 300-301 (regimi per le coorti future, indicizzazione unica leva retroattiva).

**Dispatch:** `rust-engineer` · dopo il Task 3 · skill-gate: true. Cambia la forma di `Misure` e di `EffettoAttivo`: aggiorna ogni chiamante (`partita.rs`, `ministri/agenda.rs`, test d'economia).

**Interfaces:**
- Consumes: `Disposizioni::cambia` e `arrivi` del Task 3; `Gradino.leve`, `effetti`, `saldo`; `Economia::previdenza` (`crates/sim-core/src/economia/previdenza.rs:53-87`), `conti_del_trimestre` (`crates/sim-core/src/economia/finanza.rs:175-244`), `spesa_netta_trimestre` (`crates/sim-core/src/economia/regole_ue.rs:64-82`), `aggiorna_redditi` (`crates/sim-core/src/elettorato/redditi.rs:76-138`).
- Produces:
  - `sim_core::disposizioni::Leve { pub regime_pensioni: f64, pub eta_pensione: f64, pub indicizzazione_pensioni: f64, pub pensione_minima: f64, pub sussidi: f64, pub reddito_minimo: f64, pub salario_minimo: f64, pub cuneo: f64, pub forfettario: f64, pub accise: f64, pub decontribuzione: f64, pub autonomia: f64, pub durata_legislatura: f64, pub seggi_camera: f64, pub seggi_senato: f64 }`, `Clone, Debug, PartialEq, Serialize, Deserialize`; `Leve::partenza(dati: &DatiGioco) -> Leve`; `Disposizioni::leve(&self, dati: &DatiGioco) -> Leve`.
  - `RegimePensioni::da_leva(x: f64) -> RegimePensioni` (0, 1, 2 arrotondati; fuori = `Contributivo`).
  - `Misure.leve: Leve`, `Misure.leve_partenza: Leve`; `Misure::neutre` mette la partenza in entrambe; `Misure::da(…, disposizioni: &Disposizioni)`, a 8 parametri con `#[expect(clippy::too_many_arguments, reason = "…")]` (oggi 7, `crates/sim-core/src/misure.rs:82-90`).
  - `Misure.saldo_disposizioni: f64` = `Σ_d (saldo(in vigore) − saldo(partenza))`, % del PIL potenziale annuo.
  - `Forma::Livello`: `contributo` = `intensita` da `inizio` in poi, per sempre; mai scaduto (ramo `Livello` di `esaurito`, `crates/sim-core/src/grandezze.rs:85-94`, sempre `false`). Senza durata: la `durata` non conta (0.0) e il controllo `durata > 0` di `Grandezze::controlla` (`grandezze.rs:213-214`) vale solo per `Stock` e `Impulso`.
  - `EffettoAttivo.dicastero: Option<usize>` (`None` per le disposizioni fuori dicastero; il merito del piano 5 legge solo i `Some`).
  - `Spese.disposizioni: i64` (euro del trimestre, con segno), dentro il totale delle spese, la spesa primaria e la spesa netta.

Regole:
- **Effetti di un cambio** da `a` a `b`: per ogni grandezza toccata da `a` o da `b`, `Δ = I_b − I_a` (0 dove manca); se `Δ ≠ 0`, un `EffettoAttivo { forma: Livello, intensita: Δ, inizio: turno + ritardo di b (di a se b non tocca la grandezza), dicastero: indice del dicastero, regione }`, uno per regione del perimetro con `intensita = Δ · peso`, uno solo con `regione = None` senza perimetro. Nessun fattore di capacità: le disposizioni non le esegue un ministro.
- **Previdenza**: regime delle annate nuove `da_leva(leve.regime_pensioni)`, età `leve.eta_pensione`, indicizzazione `leve.indicizzazione_pensioni`, pensione minima `scenario.pensione_minima · leve.pensione_minima`, coefficiente dei sussidi `leve.sussidi`. `Economia::iniziale` e `avvia_previdenza` leggono le leve di partenza.
- **Redditi**, per gruppo, dopo l'aliquota effettiva di oggi, con `Δx = leve.x − leve_partenza.x`:
  - dipendenti privati e pubblici di fascia bassa e media: `imposte −= Δcuneo/100 · primario`;
  - autonomi: `imposte ·= leve.forfettario / leve_partenza.forfettario`;
  - dipendenti privati di fascia bassa: `primario += copertura_salario_minimo · max(0, leve.salario_minimo · ore_salario_minimo − primario)`;
  - disoccupati e inattivi di fascia bassa: `trasferimenti = max(0, trasferimenti + Δreddito_minimo)`;
  - imposte limitate a `[0, lordo]`.
- **Prezzi**: `Misure.prezzo_energia += Δaccise`.
- **Disoccupazione strutturale**: `disoccupazione_strutturale_r −= regole_lavoro` (come `politiche_attive`, `crates/sim-core/src/misure.rs`).
- **Conti**: `Spese.disposizioni = euro(saldo_disposizioni/100 · pil_potenziale_reale · livello_prezzi · MLD)`, cioè la quota trimestrale (il PIL potenziale del modello è trimestrale). `controlla_coerenza` lo aggiunge agli importi limitati.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `ore_salario_minimo` 1600 (1000-2200, "17"), `copertura_salario_minimo` 0.3 (0-1, "17"). Esce `sussidio_universale` (il suo valore è la leva `sussidi` del gradino `universale`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/effetti_disposizioni.rs
mod comune;
use comune::{con_arrivo, ferma_tutti, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::disposizioni::Leve;
use sim_core::elettorato::{Condizione, Fascia, condizione_di, fascia_di};
use sim_core::Partita;

fn gioca(f: FileDati, turni: usize) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..turni {
        ferma_tutti(&mut p);
        p.chiudi_turno();
    }
    p
}

fn media(p: &Partita, c: Condizione, f: Fascia, x: impl Fn(&sim_core::elettorato::Reddito) -> f64) -> f64 {
    let g = &p.elettorato().gruppi;
    let v: Vec<f64> = (0..g.len()).filter(|&i| condizione_di(i) == c && fascia_di(i) == f).map(|i| x(&g[i].reddito)).collect();
    v.iter().sum::<f64>() / v.len() as f64
}

#[test]
fn partenza_non_tocca_il_motore() {
    let d = DatiGioco::da_testi(&senza_shock()).unwrap();
    let mut p = Partita::nuova(d.clone(), 1);
    assert_eq!(p.misure().leve, Leve::partenza(&d));
    ferma_tutti(&mut p);
    p.chiudi_turno();
    // Il credito ZES arriva a 1 miliardo all'apertura del turno 1: fino a lì il saldo è zero.
    assert_eq!(p.economia().conti.spese.disposizioni, 0);
    for _ in 0..3 {
        ferma_tutti(&mut p);
        p.chiudi_turno();
        assert_eq!(p.misure().leve, Leve::partenza(&d));
    }
}

#[test]
fn eta_piu_alta_riduce_le_annate_nuove() {
    let alta = gioca(con_arrivo(senza_shock(), "eta_pensionabile", "70", 2027, 1), 9);
    let base = gioca(senza_shock(), 9);
    assert!(alta.economia().conti.spese.pensioni < base.economia().conti.spese.pensioni);
}

#[test]
fn regime_tocca_solo_le_annate_nuove() {
    let r = gioca(con_arrivo(senza_shock(), "regime_pensioni", "retributivo", 2027, 1), 5);
    let c = gioca(senza_shock(), 5);
    let annata = |p: &Partita, anno: u16| p.economia().pensioni.iter().find(|a| a.anno == anno).unwrap().importo;
    assert_eq!(annata(&r, 2026), annata(&c, 2026));
    assert!(annata(&r, 2027) > annata(&c, 2027));
}

#[test]
fn sussidi_universali_costano_di_piu() {
    let u = gioca(con_arrivo(senza_shock(), "sussidi_disoccupazione", "universale", 2027, 1), 2);
    let a = gioca(senza_shock(), 2);
    assert!(u.economia().conti.spese.sussidi > a.economia().conti.spese.sussidi);
}

#[test]
fn salario_minimo_alza_il_reddito_della_fascia_bassa_dei_dipendenti_privati() {
    let s = gioca(con_arrivo(senza_shock(), "salario_minimo", "9", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    let primario = |p: &Partita, f| media(p, Condizione::DipendentePrivato, f, |r| r.primario);
    let bassa = primario(&s, Fascia::Bassa) / primario(&b, Fascia::Bassa);
    let alta = primario(&s, Fascia::Alta) / primario(&b, Fascia::Alta);
    assert!(bassa > alta + 0.01, "{bassa} {alta}");
}

#[test]
fn cuneo_azzerato_alza_le_imposte_dei_dipendenti() {
    let z = gioca(con_arrivo(senza_shock(), "cuneo_fiscale", "0", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    let imposte = |p: &Partita| media(p, Condizione::DipendentePrivato, Fascia::Bassa, |r| r.imposte);
    assert!(imposte(&z) > imposte(&b));
}

#[test]
fn accise_alte_alzano_l_energia_e_portano_gettito() {
    let a = gioca(con_arrivo(senza_shock(), "accise", "alte", 2027, 1), 2);
    let b = gioca(senza_shock(), 2);
    assert!(a.misure().prezzo_energia > b.misure().prezzo_energia);
    assert!(a.economia().conti.spese.disposizioni < b.economia().conti.spese.disposizioni);
}

#[test]
fn effetto_di_livello_resta_finche_la_disposizione_non_cambia() {
    let f = con_arrivo(senza_shock(), "licenziamento", "solo_indennizzo", 2027, 1);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let mut serie = Vec::new();
    for _ in 0..20 {
        ferma_tutti(&mut p);
        p.chiudi_turno();
        serie.push(p.grandezza("regole_lavoro").unwrap());
    }
    // Arrivo all'apertura del turno 1, ritardo 4: vale dalla chiusura del turno 5.
    assert_eq!(serie[3], 0.0);
    assert!(serie[5..].iter().all(|&x| (x - 0.3).abs() < 1e-12), "{serie:?}");
}
```

Più test d'unità: in `grandezze.rs` `livello_resta_dopo_l_inizio` (0 prima, `I` a 0, 10 e 1000 turni, mai scaduto); in `disposizioni/effetti.rs` `cambio_rende_la_differenza_dei_livelli` (da `ridotto` a `ampio` di `privato_accreditato`: un effetto, intensità 5, ritardo 1) e `perimetro_divide_per_regione_col_peso`; in `atti.rs` `atto_di_livello_rifiutato` (`{id}: forma livello solo per le disposizioni`). In `tests/conti.rs` `con_regime_pensioni`, `regime_quota_base_universale_somma_la_quota` e il test dei sussidi passano a `sostituisci(…, "disposizioni", "partenza = \"contributivo\"", …)` e `"partenza = \"assicurativo\""`: i regimi non stanno più nello scenario.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test effetti_disposizioni`
Expected: errore di compilazione, `Misure.leve` e `Spese.disposizioni` non esistono.

- [ ] **Step 3: Implementa leve, livelli, conti e redditi; togli i regimi dallo scenario**

Il commento d'ordine sopra `Economia::passo` resta: le leve sono «misure». Se un test dei piani 3-5 fissa la spesa pensionistica dopo un gennaio, cambia per la scelta 13: si aggiorna citando quella riga del piano, non si tocca la leva.

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS, compresi i test d'economia ed elettorato dei piani 3-5.

- [ ] **Step 5: Commit**

```bash
git add crates dati/base app/src/lib/tipi
git commit -m "feat(core): dispositions reach the engine through levers, levels and balances"
```

---

### Task 5: Disposizioni territoriali

**Files:**
- Modify: `crates/sim-core/src/misure.rs` (decontribuzione, autonomia, merito nel perimetro)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/territoriali.rs`

**Ricerca:** `docs/ricerca/sintesi/disposizioni-territoriali-al-turno-zero.md:14-19` (valori, perimetri), `:50-65` (residuo fiscale: le quattro regioni del perimetro sono contributrici nette), `:74-79`; `.scratch/v1-giocabile/issues/27-mappa-delle-20-regioni.md:43-49`; spec riga 312 (merito nel perimetro al partito del dicastero).

**Dispatch:** `rust-engineer` · dopo il Task 4 · skill-gate: true. Chiude la PR A, poi il round di review.

**Interfaces:**
- Consumes: `Misure` e `Leve` del Task 4, `Misure.merito` del piano 5 (`crates/sim-core/src/misure.rs`), `DatoDisposizione.perimetro`, `Elettorato` (redditi e elettori dei gruppi).
- Produces: `Misure::da` riempie per ogni regione `r`:
  - decontribuzione: `disoccupazione_strutturale_r −= Δdecontribuzione · peso_r`, e `merito[r][d] −= Δdecontribuzione · peso_r` con `d` il dicastero della disposizione;
  - autonomia: con `q = Δautonomia`, `g_r` quota del reddito lordo dei gruppi della regione sul totale e `p_r` quota degli elettori, `domanda_regionale_r += elasticita_autonomia · q · (g_r/p_r − 1)` per `r` nel perimetro, e per le altre `−elasticita_autonomia · q · Σ_{s∈perimetro} p_s (g_s/p_s − 1) / (1 − Σ_{s∈perimetro} p_s)`: il totale pesato per elettori fa zero. Nessun merito: è fuori dicastero.
  - credito ZES: già coperto dagli effetti regionali del Task 4 su `investimenti_privati`, che portano `dicastero = Some(sviluppo_economico)` e quindi il merito del piano 5.

Manopola nuova: `elasticita_autonomia` 5.0 (0-20, "27"), `# assunzione del piano 6, nessuna fonte in docs/ricerca: punti di gap della regione per unità di quota trattenuta`.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/territoriali.rs
mod comune;
use comune::{con_arrivo, dati_base, ferma_tutti, imposta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::Partita;

/// 6 condizioni × 3 fasce: i gruppi di una regione sono contigui.
const PER_REGIONE: usize = 18;

fn coppia(f: FileDati, g: FileDati, turni: usize, mut ogni: impl FnMut(&Partita, &Partita)) {
    let (mut a, mut b) = (Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 3), Partita::nuova(DatiGioco::da_testi(&g).unwrap(), 3));
    for _ in 0..turni {
        ferma_tutti(&mut a);
        ferma_tutti(&mut b);
        a.chiudi_turno();
        b.chiudi_turno();
        ogni(&a, &b);
    }
}

fn u(p: &Partita, id: &str) -> f64 {
    p.economia().regioni.iter().find(|r| r.id == id).unwrap().disoccupazione
}

fn r(id: &str) -> usize {
    dati_base().regioni().iter().position(|x| x.id == id).unwrap()
}

#[test]
fn decontribuzione_abbassa_la_disoccupazione_nel_mezzogiorno() {
    let (mut calabria, mut lombardia) = (0.0, 0.0);
    coppia(con_arrivo(senza_shock(), "decontribuzione", "30", 2027, 1), senza_shock(), 8, |a, b| {
        calabria += u(a, "calabria") - u(b, "calabria");
        lombardia += u(a, "lombardia") - u(b, "lombardia");
    });
    assert!(calabria < 0.0);
    assert!(calabria.abs() > 10.0 * lombardia.abs());
}

#[test]
fn merito_della_decontribuzione_al_partito_del_dicastero() {
    let f = imposta(senza_shock(), "scenario", "lavoro_welfare = ", "\"centrodestra\"");
    let f = imposta(f, "manopole", "[quota_colpa_presidente]\nvalore = ", "0.0");
    let mut fine = (0.0, 0.0);
    coppia(con_arrivo(f.clone(), "decontribuzione", "30", 2027, 1), f, 6, |a, b| {
        let j = a.partiti().iter().position(|x| x.id == "centrodestra").unwrap();
        let media = |p: &Partita, reg: usize| {
            let g = &p.elettorato().gruppi[reg * PER_REGIONE..(reg + 1) * PER_REGIONE];
            g.iter().map(|x| x.giudizio[j]).sum::<f64>() / g.len() as f64
        };
        fine = (media(a, r("calabria")) - media(b, r("calabria")), media(a, r("lombardia")) - media(b, r("lombardia")));
    });
    assert!(fine.0 > 0.0 && fine.0 > 10.0 * fine.1.abs(), "{fine:?}");
}

#[test]
fn zes_pesa_meta_dove_il_perimetro_e_ridotto() {
    let mut ultimo = (0.0, 0.0, 0.0);
    let k = dati_base().grandezza("investimenti_privati").unwrap();
    coppia(senza_shock(), con_arrivo(senza_shock(), "credito_zes", "0", 2027, 1), 30, |a, b| {
        let d = |reg: &str| a.grandezze().valori[k][r(reg)] - b.grandezze().valori[k][r(reg)];
        ultimo = (d("calabria"), d("abruzzo"), d("lombardia"));
    });
    assert!(ultimo.0 > 0.0);
    assert!((ultimo.1 - 0.5 * ultimo.0).abs() < 1e-9 && ultimo.2 == 0.0, "{ultimo:?}");
}

#[test]
fn autonomia_sposta_domanda_verso_il_perimetro() {
    let f = con_arrivo(senza_shock(), "autonomia_differenziata", "intese_complete", 2027, 1);
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    // L'arrivo è all'apertura del turno 1: lo leggono le misure della chiusura del turno 1.
    for _ in 0..2 {
        ferma_tutti(&mut p);
        p.chiudi_turno();
    }
    let m = &p.misure().domanda_regionale;
    assert!(m[r("lombardia")] > 0.0 && m[r("sicilia")] < 0.0);
    let el = dati_base();
    let pesata: f64 = el.elettorato().regioni.iter().zip(m).map(|(x, d)| x.elettori as f64 * d).sum();
    assert!(pesata.abs() < 1e-6 * el.elettorato().regioni.iter().map(|x| x.elettori as f64).sum::<f64>());
}

#[test]
fn perimetro_nella_vista() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let a = v.disposizioni.iter().find(|d| d.id == "autonomia_differenziata").unwrap();
    assert_eq!(a.perimetro, ["liguria", "lombardia", "piemonte", "veneto"]);
}
```

Nel test della ZES la partita `b` anticipa a gennaio 2027 il gradino `0`, quella `a` tiene gli arrivi di base (`1` dal 2027, `0_75` dal 2028): la differenza su `investimenti_privati` è fatta solo di effetti di livello, quindi il rapporto fra Abruzzo e Calabria è esattamente il peso. Nel test dell'autonomia `domanda_regionale` la muove solo l'autonomia: gli atti sono fermati e nessun'altra disposizione cambia valore su quella grandezza nei primi due turni. `imposta` col prefisso `"lavoro_welfare = "` tocca la riga `[governo]` dello scenario.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test territoriali`
Expected: FAIL su decontribuzione, merito e autonomia (le leve non arrivano ancora alle misure); la ZES può già passare col Task 4.

- [ ] **Step 3: Implementa decontribuzione, autonomia e merito nel perimetro**

- [ ] **Step 4: Esegui i test, verifica che il core resti puro**

Run: `cargo test --workspace && npm --prefix app run check && grep -rnE "std::(fs|time|thread|net|env)|HashMap|StdRng|SmallRng" crates/sim-core/src`
Expected: PASS; nessuna riga dal `grep`.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml
git commit -m "feat(core): territorial dispositions with perimeter and credit to the ministry"
```

---

## PR B · Leggi in aula (Task 6-9)

### Task 6: Parlamento, intesa, presentazione e calendario

**Files:**
- Create: `crates/sim-core/src/parlamento/mod.rs` (`Parlamento`, `Misura`, calendario, presentazione), `crates/sim-core/src/parlamento/intesa.rs`
- Modify: `crates/sim-core/src/comando.rs` (`PresentaLegge`), `crates/sim-core/src/ministri/mod.rs` (`Causa::NormaControPosizione`), `crates/sim-core/src/ministri/agenda.rs` (`penalizza` da `pub(super)` a `pub(crate)`, perché `parlamento/` lo chiama)
- Modify: `crates/sim-core/src/partita.rs` (campi `parlamento`, `intesa`; seggi al turno zero; soglie dell'intesa; `carica`), `crates/sim-core/src/vista.rs`
- Modify: `crates/sim-core/src/lib.rs`, `dati/base/manopole.toml`, `crates/sim-core/tests/comune/mod.rs` (`presenta`)
- Test: `crates/sim-core/tests/calendario.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:27-33` (legge ordinaria, sedi, navetta rara), `:66` (sei mesi prima di ripresentare), `:151` (un posto su cinque all'opposizione); spec righe 340-343.

**Dispatch:** `rust-engineer` · primo della PR B, sul branch `feat/v1-piano-6-aula` dalla punta della A dopo la sua review · skill-gate: true.

**Interfaces:**
- Consumes: `riparto`, `Voti` (`crates/sim-core/src/elezioni/riparto.rs:16-60`), `Partita::intenzioni_vere`, `Disposizioni` del Task 3, `Ministri` e `penalizza` (`crates/sim-core/src/ministri/agenda.rs:198`).
- Produces:
  - `sim_core::parlamento::Parlamento { pub seggi_camera: Vec<u32>, pub seggi_senato: Vec<u32>, pub misure: Vec<Misura>, pub respinte: Vec<(usize, u32)>, pub prossima: u32, pub esiti: Vec<EsitoVoto> }` (`respinte` = disposizione e turno da cui si ripresenta); `Misura { pub id: u32, pub forma: FormaMisura, pub valori: Vec<(usize, usize)>, pub proponente: Option<usize>, pub presentata: u32, pub voto: u32, pub deliberazione: u8, pub fiducia: bool }` (`valori` = disposizione e gradino; `proponente` = dicastero; `voto` = turno nella cui chiusura si vota; `deliberazione` 1 o 2 per le costituzionali, 0 altrimenti); `FormaMisura { Riforma, Elettorale, Costituzionale, Conversione, Bilancio }`, `Copy`, serde `snake_case`, TS esportato. I Task 8 e 10 aggiungono campi a `Misura`.
  - `EsitoVoto { pub misura: u32, pub nome: String, pub forma: FormaMisura, pub data: Trimestre, pub deliberazione: u8, pub camera: Conteggio, pub senato: Conteggio, pub approvata: bool, pub fiducia: bool, pub fiducia_respinta: bool, pub linee: Vec<VoceLinea> }`, `Conteggio { pub si: u32, pub no: u32 }`, `VoceLinea { pub partito: String, pub si: bool }`, `Clone, Debug, PartialEq`, TS esportati: qui per `Parlamento.esiti` e `Vista.esiti`, li riempie il Task 7.
  - `sim_core::parlamento::Intesa { pub valori: Vec<f64>, pub variazioni: Vec<VariazioneIntesa> }`, `VariazioneIntesa { pub partito: usize, pub causa: CausaIntesa, pub delta: f64, pub turno: u32 }`, `CausaIntesa { NormaLontana, EmendamentoRespinto, Fiducia, SeggiPersi }` (le ultime tre le usano i Task 8 e 9), `Display`: «norma lontana dalla sua posizione», «emendamento respinto», «questione di fiducia», «seggi persi con la legge elettorale».
  - `Comando::PresentaLegge { valori: Vec<ValoreProposto> }`; `sim_core::parlamento::ValoreProposto { pub disposizione: String, pub gradino: String }`, TS esportato.
  - `Causa::NormaControPosizione` («norma firmata contro la sua posizione»).
  - `Partita::parlamento() -> &Parlamento`, `Partita::intesa() -> &Intesa`, `Partita::alleati() -> Vec<usize>` (al governo, escluso il partito del Presidente del Consiglio).
  - Vista: `Vista.seggi: Vec<SeggiPartito>` (il parlamento in carica), `Vista.alleati: Vec<VoceAlleato>`, `VoceAlleato { pub partito: String, pub intesa: f64, pub ultimatum: bool, pub causa_principale: Option<CausaIntesa>, pub variazioni: Vec<VariazioneIntesa> }`; `Vista.calendario: Vec<VoceMisura>`, `VoceMisura { pub id: u32, pub nome: String, pub forma: FormaMisura, pub proponente: Option<String>, pub valori: Vec<ValoreProposto>, pub voto: Trimestre, pub deliberazione: u8, pub fiducia: bool }` (i Task 7-9 aggiungono linee, forbice, emendamenti, seggi); `Vista.esiti: Vec<EsitoVoto>` (vuoto fino al Task 7).

Regole:
- **Parlamento in carica**: in `Partita::nuova`, `riparto(dati.elezioni(), legge di partenza, voti)` con `voti.per_regione[r][j] = round(elettori_r · quota vera di j nella regione)` dalle intenzioni vere (astensione esclusa) e le coalizioni dello scenario; seggi per partito = `totale()` di Camera e Senato. Nessuna estrazione.
- **Forma**, dalle disposizioni: tutte ordinarie → `Riforma`; tutte elettorali → `Elettorale`; tutte costituzionali → `Costituzionale`; miste → rifiuto (`norme di forma diversa nella stessa legge`).
- **Presentazione**, controlli in quest'ordine, rifiuti esatti: vuota (`legge vuota`); disposizione sconosciuta (`disposizione sconosciuta: {id}`); ripetuta (`disposizione ripetuta: {id}`); gradino sconosciuto (`gradino sconosciuto: {id} {g}`); forma; dicasteri diversi fra le ordinarie (`disposizioni di dicasteri diversi: {id}`); gradino già in vigore (`{id} è già a {nome del gradino}`); gradino bloccato (`gradino bloccato dal referendum: {id} {g}`); in esame (`{id} è già in esame`); respinta da poco (`{id} respinta: si ripresenta da {mese-mese anno}`); posto libero (`calendario pieno`). Il calendario ha `POSTI_ORDINARI = 3` (riforme ed elettorali) e `POSTI_COSTITUZIONALI = 1`; il posto dell'opposizione è `POSTI_OPPOSIZIONE = 1`, vuoto (Confini).
- **Voto**: riforma ed elettorale `voto = turno + 1`; costituzionale prima deliberazione `voto = turno`, la seconda la fissa il Task 9.
- **Proponente**: il dicastero delle disposizioni ordinarie; fuori dicastero nessuno. Se il dicastero ha un ministro, alla presentazione variazione `NormaControPosizione` di `−costo_firma · max(0, |pos_nuova − pos_ministro| − |pos_vigore − pos_ministro|)`, con `pos_nuova` e `pos_vigore` le medie delle posizioni dei gradini nuovi e in vigore.
- **Intesa**: `valori[j] = intesa_base` per ogni partito; si legge solo per gli alleati. In chiusura, nel passo delle soglie: `intesa += ritorno_intesa · (intesa_base − intesa)`, limitata a 0-100; variazioni tenute due turni come quelle dei ministri. `ultimatum = intesa < soglia_ultimatum`.
- **Nome della misura**: `"{nome disposizione}: {nome gradino}"` uniti da `"; "`.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `intesa_base` 60 (20-90, "05"), `ritorno_intesa` 0.1 (0-0.5, "05"), `soglia_ultimatum` 40 (20-60, "05"), `costo_firma` 10 (0-40, "16"), `turni_blocco_respinta` 2 (1-8, "08", `# iter-legislativo-italiano:66, sei mesi`).

`presenta` per i test, in `tests/comune/mod.rs`:

```rust
#[allow(
    dead_code,
    reason = "ogni crate di test usa solo una parte degli helper"
)]
pub fn presenta(p: &mut Partita, valori: &[(&str, &str)]) -> Result<(), Errore> {
    let valori = valori.iter().map(|(d, g)| ValoreProposto { disposizione: (*d).into(), gradino: (*g).into() }).collect();
    p.comanda(Comando::PresentaLegge { valori })
}
```

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/calendario.rs
mod comune;
use comune::{dati_base, presenta};
use sim_core::ministri::Causa;
use sim_core::parlamento::FormaMisura;
use sim_core::vista::SeggiPartito;
use sim_core::{Errore, Partita, Trimestre};

#[test]
fn parlamento_uscente_con_la_maggioranza() {
    let v = Partita::nuova(dati_base(), 1).vista();
    let somma = |f: fn(&SeggiPartito) -> u32| v.seggi.iter().map(f).sum::<u32>();
    assert_eq!((somma(|s| s.camera), somma(|s| s.senato)), (400, 200));
    let governo = |f: fn(&SeggiPartito) -> u32| {
        v.seggi.iter().filter(|s| s.partito == "destra" || s.partito == "centrodestra").map(f).sum::<u32>()
    };
    assert!(governo(|s| s.camera) > 200 && governo(|s| s.senato) > 100);
}

#[test]
fn legge_presentata_occupa_il_calendario_fino_al_voto() {
    let mut p = Partita::nuova(dati_base(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let v = p.vista();
    let m = &v.calendario[0];
    assert_eq!((m.forma, m.voto, m.proponente.as_deref()), (FormaMisura::Riforma, Trimestre { anno: 2027, trimestre: 1 }, Some("lavoro_welfare")));
    assert_eq!(m.nome, "Età pensionabile: 68 anni");
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    presenta(&mut p, &[("obbligo_scolastico", "18")]).unwrap();
    let e = presenta(&mut p, &[("leva_militare", "obbligatoria")]).unwrap_err();
    assert!(e.to_string().contains("calendario pieno"), "{e}");
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    assert_eq!(p.vista().calendario[3].forma, FormaMisura::Costituzionale);
}

#[test]
fn leggi_non_ammesse_rifiutate_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    for (valori, nome) in [
        (vec![("eta_pensionabile", "67")], "eta_pensionabile è già a 67 anni"),
        (vec![("eta_pensionabile", "71")], "gradino sconosciuto: eta_pensionabile 71"),
        (vec![("eta_pension", "68")], "disposizione sconosciuta: eta_pension"),
        (vec![("eta_pensionabile", "68"), ("ticket", "alto")], "disposizioni di dicasteri diversi: ticket"),
        (vec![("eta_pensionabile", "68"), ("eta_pensionabile", "69")], "disposizione ripetuta: eta_pensionabile"),
        (vec![("eta_pensionabile", "68"), ("soglia_lista", "0")], "norme di forma diversa"),
        (vec![], "legge vuota"),
    ] {
        let e = presenta(&mut p, &valori).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let e = presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap_err();
    assert!(e.to_string().contains("eta_pensionabile è già in esame"), "{e}");
}

#[test]
fn firma_contro_la_posizione_costa_soddisfazione() {
    let s = |gradino: &str| {
        let mut p = Partita::nuova(dati_base(), 2);
        presenta(&mut p, &[("eta_pensionabile", gradino)]).unwrap();
        let v = p.vista();
        let m = v.ministeri.iter().find(|m| m.dicastero == "lavoro_welfare").unwrap().ministro.clone().unwrap();
        (m.soddisfazione, m.variazioni.iter().any(|x| x.causa == Causa::NormaControPosizione))
    };
    let (lontana, causa) = s("62");
    assert!(causa && lontana < s("68").0);
}

#[test]
fn alleati_con_l_intesa_nella_vista() {
    let v = Partita::nuova(dati_base(), 1).vista();
    assert_eq!(v.alleati.len(), 1);
    let a = &v.alleati[0];
    assert_eq!((a.partito.as_str(), a.intesa, a.ultimatum), ("centrodestra", 60.0, false));
}
```

Il ministro del Lavoro è della destra (`dati/base/scenario.toml`, chiave `lavoro_welfare` di `[governo]`), con la posizione a 6 ± lo scarto: il gradino `62` (posizione 2) lo allontana più del `68` (5), per ogni scarto entro ±1,5.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test calendario`
Expected: errore di compilazione, `Comando::PresentaLegge` e `Vista.calendario` non esistono.

- [ ] **Step 3: Implementa `parlamento/`, intesa, presentazione e vista**

`Parlamento::controlla` e `Intesa::controlla` in `carica`: seggi uno per partito e coi totali dei rami, misure con indici di disposizione e gradino validi e proponente fra i dicasteri, intesa finita in 0-100 per ogni partito. Il commento d'ordine di `chiudi_turno` riceve dentro «soglie» il ritorno dell'intesa.

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS. Se `parlamento_uscente_con_la_maggioranza` fallisce, fermati e riporta i seggi: il riparto delle intenzioni vere non dà la maggioranza al governo uscente, e non si toccano i dati per farla tornare.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): sitting parliament, coalition agreement, bills and calendar"
```

---

### Task 7: Voto in aula, linee e franchi tiratori

**Files:**
- Create: `crates/sim-core/src/parlamento/voto.rs`
- Modify: `crates/sim-core/src/parlamento/mod.rs`, `crates/sim-core/src/partita.rs` (passo «aula»: leggi; posizione percepita), `crates/sim-core/src/vista.rs`, `crates/sim-core/src/ministri/mod.rs` (`Causa::LeggeRespinta`)
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/voto.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:166` (legge ordinaria su più turni), `:171` (bocciatura: due turni di blocco); `docs/ricerca/modelli/voto-spaziale.md:41` (distanza come base della scelta); spec righe 313 (posizione percepita), 340-341.

**Dispatch:** `rust-engineer` · dopo il Task 6 · skill-gate: true.

**Interfaces:**
- Consumes: `Parlamento`, `Misura`, `Intesa`, `EsitoVoto` del Task 6; `Disposizioni::cambia`; `Caso::uniforme(Flusso::Parlamento)`; `Partito.posizione_percepita` e `valence`.
- Produces:
  - `sim_core::parlamento::voto::linea(…) -> Vec<bool>` per partito, e `probabilita_franco(…) -> Vec<f64>`, usate dal voto, dalla vista e dall'anteprima del Task 13 (firma esatta a scelta del task, nello stesso modulo).
  - Gli `EsitoVoto` (tipo del Task 6) dei voti in aula, in `Parlamento.esiti` e `Vista.esiti`.
  - `VoceMisura` in più: `pub linee: Vec<VoceLinea>`, `pub forbice: Forbice`; `Forbice { pub camera: [u32; 2], pub senato: [u32; 2], pub maggioranza_camera: u32, pub maggioranza_senato: u32, pub in_bilico: bool }` (sì attesi ± 2σ, arrotondati e limitati ai seggi; `maggioranza_*` = sì necessari; `in_bilico` se una forbice scende sotto la sua maggioranza).
  - `Causa::LeggeRespinta` («legge respinta in aula»).

Regole:
- **Linea** di una misura con disposizioni ordinarie o costituzionali, per il partito `j` in posizione dichiarata `p`: partito del Presidente del Consiglio sì (scelta 3); altrimenti `s = |p − pos_vigore| − |p − pos_nuova|`, più `peso_intesa_linea · (intesa_j − 50)/50` se alleato, meno `malus_opposizione` se fuori dal governo; sì se `s > 0`. Linea elettorale: Task 9. Conversione e bilancio: Task 10 e 11.
- **Franchi tiratori**: `d_j = |p − pos_nuova|/6` (0 per le misure senza posizione); con linea sì `q = min(ft_max, ft_base + ft_distanza · d_j + [alleato] ft_intesa · max(0, 50 − intesa_j)/50)`; con linea no `q = min(ft_max, ft_base · (1 − d_j))`. Per ogni seggio una uniforme `u`; vota contro la linea se `u < q`. Sì contati per ramo.
- **Esito**: approvata se in ciascun ramo `sì > no` (tutti i seggi votano); per la seconda deliberazione costituzionale il Task 9.
- **Passo «aula»**, dopo l'esecuzione degli atti: le misure con `voto == turno`, in ordine di `id` (il Task 10 mette le conversioni prima, l'11 il bilancio). Approvata: per ogni valore `Disposizioni::cambia` con la legge `"L. {id}/{anno}"` e gli effetti in `grandezze.effetti`; la misura esce dal calendario. Respinta: esce, ogni disposizione in `respinte` fino a `turno + 1 + turni_blocco_respinta`; proponente `LeggeRespinta` di `−costo_legge_respinta`; partito del Presidente del Consiglio `valence −= valence_legge_respinta`. L'esito entra in testa a `esiti`: il più recente per primo (nello stesso turno, l'ultimo votato per primo); all'apertura si tengono quelli dei due turni prima.
- **Norma lontana**: approvata una misura con posizione, ogni alleato con `|p − pos_nuova| ≥ distanza_alleato` perde `costo_norma_lontana · (|p − pos_nuova| − distanza_alleato + 1)` d'intesa (`CausaIntesa::NormaLontana`).
- **Posizione percepita** (spec riga 313): dopo ogni voto su una misura con posizione, ogni partito con linea sì `posizione_percepita += scivolamento_percepita · (pos_nuova − posizione_percepita)`.
- **Dimissioni del proponente**: la misura resta; i costi che toccherebbero il proponente non toccano nessuno se il dicastero è in interim al momento del voto.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `peso_intesa_linea` 1.0 (0-3, "08"), `malus_opposizione` 1.0 (0-5, "08"), `ft_base` 0.02 (0-1, "08"), `ft_distanza` 0.15 (0-1, "08"), `ft_intesa` 0.1 (0-1, "08"), `ft_max` 0.5 (0-1, "08"), `costo_legge_respinta` 5 (0-20, "08"), `valence_legge_respinta` 0.05 (0-0.5, "08"), `distanza_alleato` 2.0 (1-4, "08", `# spec riga 101`), `costo_norma_lontana` 2 (0-10, "05"), `scivolamento_percepita` 0.05 (0-0.5, "06").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/voto.rs
mod comune;
use comune::{file_base, imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::ministri::Causa;
use sim_core::parlamento::FormaMisura;
use sim_core::{Comando, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn partita(f: FileDati, seme: u64) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme)
}

#[test]
fn legge_approvata_alla_fine_del_turno_dopo() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("67"));
    assert!(p.vista().esiti.is_empty());
    p.chiudi_turno();
    let v = p.vista();
    let e = &v.esiti[0];
    assert_eq!(e.forma, FormaMisura::Riforma);
    assert!(e.approvata && e.camera.si + e.camera.no == 400 && e.senato.si + e.senato.no == 200);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("68"));
    let d = v.disposizioni.iter().find(|d| d.id == "eta_pensionabile").unwrap();
    assert_eq!(d.precedente, "67");
    assert!(d.legge.starts_with("L. ") && d.legge.ends_with("/2027"), "{}", d.legge);
    assert!(v.calendario.is_empty());
}

#[test]
fn linee_seguono_la_distanza_senza_intesa_ne_malus() {
    let f = con(con(file_base(), "malus_opposizione", "0.0"), "peso_intesa_linea", "0.0");
    let mut p = partita(f, 1);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let v = p.vista();
    let si: Vec<&str> = v.calendario[0].linee.iter().filter(|l| l.si).map(|l| l.partito.as_str()).collect();
    assert_eq!(si, ["estrema_sinistra", "sinistra", "centrosinistra", "destra"]);
}

#[test]
fn conteggio_dentro_la_forbice() {
    let mut dentro = 0;
    for seme in 1..=20 {
        let mut p = partita(senza_shock(), seme);
        presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
        p.chiudi_turno();
        let [basso, alto] = p.vista().calendario[0].forbice.camera;
        p.chiudi_turno();
        let si = p.vista().esiti[0].camera.si;
        dentro += usize::from((basso..=alto).contains(&si));
    }
    assert!(dentro >= 17, "{dentro}");
}

#[test]
fn stesso_seme_stesso_voto() {
    let voto = |seme| {
        let mut p = partita(senza_shock(), seme);
        presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
        p.chiudi_turno();
        p.chiudi_turno();
        p.vista().esiti[0].clone()
    };
    assert_eq!(voto(4), voto(4));
}

#[test]
fn legge_respinta_si_ripresenta_dopo_due_turni() {
    let mut p = partita(con(con(senza_shock(), "ft_base", "0.9"), "ft_max", "1.0"), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    assert!(!v.esiti[0].approvata);
    let lw = v.ministeri.iter().find(|m| m.dicastero == "lavoro_welfare").unwrap();
    assert!(lw.ministro.as_ref().unwrap().variazioni.iter().any(|x| x.causa == Causa::LeggeRespinta));
    let e = presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap_err();
    assert!(e.to_string().contains("eta_pensionabile respinta: si ripresenta da"), "{e}");
    p.chiudi_turno();
    p.chiudi_turno();
    presenta(&mut p, &[("eta_pensionabile", "69")]).unwrap();
}

#[test]
fn votare_sposta_la_posizione_percepita() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let percepita = |id: &str| p.partiti().iter().find(|x| x.id == id).unwrap().posizione_percepita;
    assert!(percepita("estrema_sinistra") > 1.0 && percepita("destra") < 6.0);
    assert_eq!((percepita("centrosinistra"), percepita("centro")), (3.0, 4.0));
}

#[test]
fn legge_approvata_cancella_il_valore_in_arrivo() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("nucleare", "consentito")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    let v = p.vista();
    let n = v.disposizioni.iter().find(|d| d.id == "nucleare").unwrap();
    assert!(n.in_arrivo.is_empty() && n.in_vigore == "consentito");
}

#[test]
fn dimissioni_del_proponente_non_fanno_cadere_la_legge() {
    let mut p = partita(senza_shock(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.comanda(Comando::ChiediDimissioni { dicastero: "lavoro_welfare".into() }).unwrap();
    p.chiudi_turno();
    assert_eq!(p.vista().calendario.len(), 1);
    p.chiudi_turno();
    assert_eq!(p.vista().esiti.len(), 1);
}
```

Le posizioni di `eta_pensionabile` sono quelle verbatim del Task 1 (`62` a 2.0, `67` a 4.5): senza intesa né malus votano sì i partiti a 1, 2 e 3, il centro (4) no perché 67 gli è più vicino, la destra sì perché è del Presidente del Consiglio. Con il malus di partenza (1) invece il centrosinistra vota no (`1,5 − 1 − 1 < 0`) e l'estrema sinistra sì (`3,5 − 1 − 1 > 0`): per questo nel test della percepita si muovono estrema sinistra e destra, mentre centrosinistra e centro restano dove sono. Il nucleare `consentito` (5.0) contro `vietato` (3.0) ha la linea sì del centrodestra e della destra, che hanno la maggioranza (Task 6). Più test d'unità in `parlamento/voto.rs`: `franco_cresce_con_la_distanza_e_l_intesa_bassa`, `ft_max_limita_la_probabilita`.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test voto`
Expected: FAIL: `VoceMisura.linee` non esiste.

- [ ] **Step 3: Implementa linee, franchi tiratori, esito, effetti dell'approvazione e percepita**

Il commento d'ordine di `chiudi_turno` scrive dentro «aula»: conversioni → bilancio → leggi → referendum; qui solo le leggi.

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): floor votes with party lines, snipers and perceived position"
```

---

### Task 8: Emendamenti e questione di fiducia

**Files:**
- Create: `crates/sim-core/src/parlamento/emendamenti.rs`
- Modify: `crates/sim-core/src/parlamento/mod.rs`, `parlamento/voto.rs`, `crates/sim-core/src/comando.rs` (`Emendamento`, `PoniFiducia`), `crates/sim-core/src/partita.rs` (default in chiusura, emendamenti in apertura), `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/emendamenti.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:38-44` (fiducia: niente emendamenti, appello nominale, dimissioni per prassi), `:169`; `.scratch/v1-giocabile/issues/08-iter-parlamentare.md:20`, `:24` (emendamento e fiducia nel gioco); ticket 16:27 (metà strada sui parametri, rinvio sui regimi).

**Dispatch:** `rust-engineer` · dopo il Task 7 · skill-gate: true.

**Interfaces:**
- Consumes: linea e franchi tiratori del Task 7, `Intesa`, `TipoDisposizione`, `StatoDisposizione.in_arrivo`, `Disposizioni::programma(&mut self, d: usize, g: usize, turno: u32, legge: &str)` del Task 3 (il rinvio).
- Produces:
  - `Misura` in più: `pub emendamenti: Vec<RichiestaEmendamento>`, `pub emendamenti_aperti: bool`; `RichiestaEmendamento { pub partito: usize, pub proposta: Vec<(usize, Attenuazione)>, pub stato: StatoEmendamento, pub motivo: String }`; `Attenuazione { Gradino(usize), Rinvio }`; `StatoEmendamento { Aperto, Accolto, Respinto }`, `Copy`, TS esportato.
  - `Comando::Emendamento { misura: u32, partito: String, accogli: bool }`, `Comando::PoniFiducia { misura: u32 }`.
  - `VoceMisura` in più: `pub emendamenti: Vec<VoceEmendamento>`, `pub alleati_contro_fiducia: Vec<String>`; `VoceEmendamento { pub partito: String, pub testo: String, pub motivo: String, pub stato: StatoEmendamento }`.

Regole:
- **Quando**: all'apertura del turno in cui una riforma o una legge elettorale si vota (`voto == turno`), se `emendamenti_aperti == false` e senza fiducia, ogni alleato con `|p − pos_nuova| ≥ distanza_alleato`, o che tiene il dicastero delle disposizioni (scelta 5), chiede un emendamento; poi `emendamenti_aperti = true`. Sulla legge elettorale, che non ha posizione, lo chiede l'alleato che ci perde seggi (Task 9 porta il calcolo). Le costituzionali non ne hanno.
- **Testo**, per disposizione: `Parametro` con gradino in vigore `a` e nuovo `b`, `|a − b| ≥ 2`: il gradino `round_verso_a((a + b)/2)` (la metà, arrotondata verso il vigente); `Regime`, o parametro con `|a − b| = 1`: `Rinvio`. `testo`: `"{nome}: {nome del gradino}"` o `"{nome}: in vigore fra {turni_rinvio_regime} turni"`, uniti da `"; "`. `motivo`: `"distante {d:.1} punti"` o `"tocca il suo dicastero {nome}"`.
- **Accolto**: la misura prende il testo (il gradino a metà; per il rinvio il valore resta quello proposto e, approvato, diventa un arrivo invece di cambiare subito: `Disposizioni::programma(d, g, voto + turni_rinvio_regime, "L. {id}/{anno}")`, che sostituisce gli arrivi già in corso della disposizione e porta la legge della misura fino all'arrivo); quell'alleato vota sì e i suoi franchi tiratori hanno `q = ft_base`. **Respinto** (anche per default, primo passo della chiusura insieme alle proposte, tranne per le misure che slittano nel turno del bilancio, Task 11): intesa `−costo_emendamento_respinto` (`EmendamentoRespinto`); i suoi franchi tiratori su quella misura `q · (1 + ft_emendamento_respinto)`, sempre limitato a `ft_max`. Rifiuti: `misura sconosciuta: {id}`, `nessun emendamento di {partito} sulla misura {id}`, `emendamento già deciso: {partito}`.
- **Fiducia**: ammessa su riforme, leggi elettorali, conversioni (Task 10) e bilancio (Task 11); rifiuti `niente fiducia su una norma costituzionale: {id}`, `fiducia già posta: {id}`, `misura sconosciuta: {id}`. Effetti: `voto = turno`, emendamenti aperti scartati senza costo, `fiducia = true`; ogni alleato perde `costo_fiducia + costo_fiducia_distanza · |p − pos_nuova|/6` d'intesa (`Fiducia`), il doppio sulla legge elettorale. Voto a blocchi: maggioranza sì tranne gli alleati con `intesa < soglia_ultimatum` e `|p − pos_nuova| ≥ distanza_alleato`, che votano no; fuori dal governo no; nessun franco tiratore (le estrazioni si fanno comunque). Respinta: la misura cade senza blocco di due turni e `fiducia_respinta = true` nell'esito (Confini: le dimissioni sono del piano 7). La legge elettorale ha il suo voto finale: Task 9.
- **Vista**: `alleati_contro_fiducia` = gli alleati che con la fiducia voterebbero no, calcolato sempre (storia 65).

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `costo_emendamento_respinto` 5 (0-20, "08"), `ft_emendamento_respinto` 1.0 (0-3, "08"), `costo_fiducia` 3 (0-15, "08"), `costo_fiducia_distanza` 6 (0-20, "08"), `turni_rinvio_regime` 4 (1-8, "13", `# ticket 13:51, Italicum`).

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/emendamenti.rs
mod comune;
use comune::{imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{CausaIntesa, StatoEmendamento};
use sim_core::{Comando, Errore, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn al_voto(gradino: (&str, &str)) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[gradino]).unwrap();
    p.chiudi_turno();
    p
}

fn accogli(p: &mut Partita, accogli: bool) -> Result<(), Errore> {
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::Emendamento { misura: id, partito: "centrodestra".into(), accogli })
}

#[test]
fn alleato_lontano_chiede_un_emendamento_a_meta_strada() {
    let p = al_voto(("eta_pensionabile", "62"));
    let v = p.vista();
    let e = &v.calendario[0].emendamenti;
    assert_eq!(e.len(), 1);
    assert_eq!((e[0].partito.as_str(), e[0].testo.as_str(), e[0].stato), ("centrodestra", "Età pensionabile: 65 anni", StatoEmendamento::Aperto));
    assert!(e[0].motivo.starts_with("distante"));
}

#[test]
fn emendamento_accolto_attenua_e_compatta() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    accogli(&mut p, true).unwrap();
    let v = p.vista();
    assert!(v.calendario[0].linee.iter().any(|l| l.partito == "centrodestra" && l.si));
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("65"));
}

#[test]
fn emendamento_ignorato_vale_respinto_e_costa_intesa() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    p.chiudi_turno();
    let a = &p.vista().alleati[0];
    assert!(a.variazioni.iter().any(|x| x.causa == CausaIntesa::EmendamentoRespinto));
}

#[test]
fn regime_emendato_arriva_dopo_quattro_turni() {
    let mut p = al_voto(("licenziamento", "reintegro"));
    assert!(p.vista().calendario[0].emendamenti[0].testo.ends_with("in vigore fra 4 turni"));
    accogli(&mut p, true).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert_eq!(p.in_vigore("licenziamento"), Some("tutele_crescenti"));
    for _ in 0..4 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
}

#[test]
fn fiducia_vota_nel_turno_senza_franchi_tiratori() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    let maggioranza: u32 = p.vista().seggi.iter().filter(|s| s.partito == "destra" || s.partito == "centrodestra").map(|s| s.camera).sum();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti[0].fiducia && v.esiti[0].approvata);
    assert_eq!(v.esiti[0].camera.si, maggioranza);
    assert!(v.alleati[0].variazioni.iter().any(|x| x.causa == CausaIntesa::Fiducia));
}

#[test]
fn alleato_in_ultimatum_e_lontano_vota_no_alla_fiducia() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&con(senza_shock(), "intesa_base", "30.0")).unwrap(), 1);
    presenta(&mut p, &[("eta_pensionabile", "62")]).unwrap();
    let v = p.vista();
    assert_eq!(v.calendario[0].alleati_contro_fiducia, ["centrodestra"]);
    p.comanda(Comando::PoniFiducia { misura: v.calendario[0].id }).unwrap();
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert!(e.linee.iter().any(|l| l.partito == "centrodestra" && !l.si));
    // Voto a blocchi senza franchi tiratori: votano sì solo i seggi della destra.
    let destra = v.seggi.iter().find(|s| s.partito == "destra").unwrap();
    assert_eq!((e.camera.si, e.senato.si), (destra.camera, destra.senato));
    assert_eq!(e.approvata, destra.camera * 2 > 400 && destra.senato * 2 > 200);
    assert_eq!(e.fiducia_respinta, !e.approvata);
}

#[test]
fn niente_fiducia_su_una_norma_costituzionale() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    let id = p.vista().calendario[0].id;
    let e = p.comanda(Comando::PoniFiducia { misura: id }).unwrap_err();
    assert!(e.to_string().contains("niente fiducia su una norma costituzionale"), "{e}");
}

#[test]
fn salvare_a_meta_turno_conserva_l_aula() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    accogli(&mut p, true).unwrap();
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    let id = p.vista().calendario[1].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    let mut q = Partita::carica(&p.salva().unwrap()).unwrap();
    p.chiudi_turno();
    q.chiudi_turno();
    assert_eq!(p.salva().unwrap(), q.salva().unwrap());
}

#[test]
fn comandi_dell_aula_non_ammessi_rifiutati_col_nome() {
    let mut p = al_voto(("eta_pensionabile", "62"));
    let id = p.vista().calendario[0].id;
    let prima = p.salva().unwrap();
    for (c, nome) in [
        (Comando::Emendamento { misura: 99, partito: "centrodestra".into(), accogli: true }, "misura sconosciuta: 99"),
        (Comando::Emendamento { misura: id, partito: "sinistra".into(), accogli: true }, "nessun emendamento di sinistra"),
        (Comando::PoniFiducia { misura: 99 }, "misura sconosciuta: 99"),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    accogli(&mut p, true).unwrap();
    assert!(accogli(&mut p, false).unwrap_err().to_string().contains("emendamento già deciso: centrodestra"));
}
```

Il centrodestra sta a 5: `62` (posizione 2) dista 3 ≥ 2. Fra il gradino in vigore `67` (indice 5) e `62` (indice 0) la metà è 2,5, arrotondata verso il vigente: indice 3, `65`. Sul licenziamento il `reintegro` (2) dista 3 e il regime dà il rinvio. Con `intesa_base` a 30 il centrodestra parte sotto la soglia d'ultimatum.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test emendamenti`
Expected: errore di compilazione, `Comando::Emendamento` non esiste.

- [ ] **Step 3: Implementa emendamenti, default e fiducia**

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): coalition amendments and confidence votes"
```

---

### Task 9: Legge elettorale, norme costituzionali e referendum confermativo

**Files:**
- Create: `crates/sim-core/src/parlamento/regole.rs` (elettorale e costituzionale), `crates/sim-core/src/elettorato/referendum.rs`
- Modify: `crates/sim-core/src/parlamento/voto.rs`, `parlamento/mod.rs` (`Referendum`), `crates/sim-core/src/partita.rs` (proiezione col numero dei parlamentari, passo «aula»: referendum), `crates/sim-core/src/disposizioni/stato.rs` (arrivo elettorale non valido scartato), `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/norme.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:48-61` (voto segreto sulla legge elettorale solo alla Camera, fiducia sugli articoli con voto finale segreto), `:76-80` (art. 138: due deliberazioni, maggioranza assoluta, due terzi, un quinto chiede il referendum, nessun quorum), `:173`; `.scratch/v1-giocabile/issues/13-legge-elettorale-come-misura.md:45-56`.

**Dispatch:** `rust-engineer` · dopo il Task 8 · skill-gate: true. Chiude la PR B, poi il round di review.

**Interfaces:**
- Consumes: `Partita::proiezione_seggi` e `LeggeElettorale::valida`, voto ed emendamenti dei Task 7-8, `Leve.seggi_camera`/`seggi_senato`, `Elettorato` (posizioni dei gruppi, giudizio, astensione: `crates/sim-core/src/elettorato/voto.rs:53-85`).
- Produces:
  - `Elettorato::referendum(&self, dati: &DatiGioco, partiti: &[Partito], governo: &Governo, pos_si: Option<f64>, pos_no: Option<f64>, peso_governo: f64, invito: f64) -> Affluenza`, con `#[expect(clippy::too_many_arguments, reason = "…")]` (i partiti servono alla quota di governo del gruppo, dalle sue intenzioni); `Affluenza { pub si: f64, pub no: f64, pub astenuti: f64 }` in elettori. Per gruppo `i` e punto `x`: `u_si = −β(x − pos_si)² + peso_governo · γ · g_i`, `u_no = −β(x − pos_no)²` (termini di distanza a 0 se la posizione manca), `u_ast = astensione del gruppo + astensione_referendum + invito · quota di governo del gruppo`, con `g_i` il giudizio sul partito del Presidente del Consiglio; quote col softmax, pesate per `h_x` e per gli elettori.
  - `sim_core::parlamento::Referendum { pub tipo: TipoReferendum, pub misura: Option<u32>, pub disposizione: usize, pub gradino: usize, pub promotore: Option<usize>, pub voto: u32, pub firme: bool, pub invito_astensione: bool }`, `TipoReferendum { Confermativo, Abrogativo }`; `Parlamento.referendum: Option<Referendum>` (uno alla volta, Task 14) e `Parlamento.confermativi: Vec<Referendum>`.
  - `EsitoReferendum { pub tipo: TipoReferendum, pub nome: String, pub data: Trimestre, pub si: u64, pub no: u64, pub votanti: u64, pub aventi_diritto: u64, pub quorum: bool, pub approvato: bool }`; `Vista.esiti_referendum: Vec<EsitoReferendum>`, `Vista.referendum: Vec<VistaReferendum>`, `VistaReferendum { pub tipo: TipoReferendum, pub nome: String, pub voto: Trimestre, pub promotore: Option<String>, pub invito_astensione: bool }`.
  - `VoceMisura.seggi: Vec<VariazioneSeggi>`, `VariazioneSeggi { pub partito: String, pub camera: i32, pub senato: i32 }` (solo per le leggi elettorali).

Regole:
- **Legge elettorale** (spec righe 340, 75): alla presentazione la legge in vigore con i valori nuovi deve passare `LeggeElettorale::valida`, considerando anche gli arrivi elettorali in corso (`legge elettorale non valida: {messaggio}`, con `{messaggio}` la stringa dentro `Errore::LeggeElettorale`, non il suo `to_string()`, che aggiungerebbe «legge elettorale non ammessa: », `crates/sim-core/src/errore.rs:118`). `Δseggi_j` = seggi di Camera più Senato proiettati con la legge nuova meno quelli con la legge in vigore (proiezione sul sondaggio pubblico, coalizioni dell'ultimo voto, spec riga 322). Linea: partito del Presidente del Consiglio sì; altrimenti `s = Δseggi_j / seggi_per_punto_linea` più i termini d'intesa e malus del Task 7; sì se `s > 0`. Nessuna posizione: niente distanza nei franchi tiratori, niente percepita. Alla Camera il voto finale è segreto: `q` raddoppiato nei due sensi, limitato a `2 · ft_max`. Con la fiducia: prima il voto a blocchi del Task 8 (se respinto, `fiducia_respinta` e l'esito porta quei conteggi), poi il voto finale, con le linee del voto a blocchi (non quelle sui seggi attesi), che alla Camera ha i franchi tiratori raddoppiati e al Senato nessuno; l'esito porta i conteggi del voto finale. Bocciata al voto finale: cade la legge, `fiducia_respinta = false`. Approvata: ogni alleato con `Δseggi_j < 0` perde `costo_seggio_perso · |Δseggi_j|` d'intesa (`SeggiPersi`), anche se ha votato sì. Emendamento (Task 8): lo chiede l'alleato con `Δseggi_j < 0`.
- **Arrivi elettorali non validi**: in `Disposizioni::arrivi`, gli arrivi di un turno si applicano insieme; se la legge elettorale che ne risulta non passa `valida`, quelli elettorali del turno si scartano e le loro disposizioni restano.
- **Norma costituzionale**: prima deliberazione a fine del turno di presentazione (maggioranza semplice in ciascun ramo); se passa, `deliberazione = 2` e `voto = turno + 1`, a maggioranza assoluta (`sì > seggi/2` in ciascun ramo); niente emendamenti, niente fiducia. Respinta in una delle due: cade, blocco di due turni come le altre. Approvata la seconda con almeno i due terzi in entrambi i rami (`sì · 3 ≥ seggi · 2`): `Disposizioni::cambia` subito. Altrimenti, se i partiti fuori dal governo con linea no hanno almeno un quinto dei seggi in un ramo, un confermativo con `voto = turno + 1`; se no, `cambia` subito.
- **Referendum confermativo**: nella chiusura del suo turno, `Elettorato::referendum` con `pos_si` = posizione del gradino nuovo, `pos_no` = del vigente, `peso_governo = 1`, `invito = 0`; approvato se `si > no`, senza quorum; poi `cambia` o niente. Esito in `esiti_referendum`.
- **Proiezione**: i seggi totali di Camera e Senato vengono da `leve.seggi_camera` e `leve.seggi_senato` (una copia di `DatiElezioni` coi totali cambiati, `seggi_per_circoscrizione` regge ogni totale, `2026-10-22-v1-piano-2-riparto-elettorale.md:15`); il parlamento in carica non cambia fino al voto (piano 7).

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `seggi_per_punto_linea` 10 (1-50, "13"), `costo_seggio_perso` 0.5 (0-5, "13"), `astensione_referendum` 1.0 (−5..5, "12").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/norme.rs
mod comune;
use comune::{imposta, presenta, senza_shock, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{CausaIntesa, TipoReferendum};
use sim_core::vista::SeggiPartito;
use sim_core::{Comando, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

fn partita(f: FileDati) -> Partita {
    Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1)
}

#[test]
fn legge_elettorale_linea_sui_seggi_attesi() {
    let mut p = partita(con(con(senza_shock(), "malus_opposizione", "0.0"), "peso_intesa_linea", "0.0"));
    presenta(&mut p, &[("soglia_lista", "0")]).unwrap();
    let v = p.vista();
    let m = &v.calendario[0];
    for l in m.linee.iter().filter(|l| l.partito != "destra") {
        let s = m.seggi.iter().find(|s| s.partito == l.partito).unwrap();
        assert_eq!(l.si, s.camera + s.senato > 0, "{}", l.partito);
    }
}

#[test]
fn legge_elettorale_non_valida_rifiutata_col_motivo() {
    let mut p = partita(senza_shock());
    let e = presenta(&mut p, &[("coalizioni", "no")]).unwrap_err();
    assert!(e.to_string().contains("legge elettorale non valida: premio alla coalizione senza coalizioni"), "{e}");
}

#[test]
fn alleato_che_perde_seggi_paga_intesa_anche_se_vota_si() {
    let mut p = partita(senza_shock());
    presenta(&mut p, &[("premio", "nessuno")]).unwrap();
    let v = p.vista();
    let perde = v.calendario[0].seggi.iter().find(|s| s.partito == "centrodestra").unwrap();
    assert!(perde.camera + perde.senato < 0);
    p.comanda(Comando::PoniFiducia { misura: v.calendario[0].id }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti[0].approvata);
    assert!(p.vista().alleati[0].variazioni.iter().any(|x| x.causa == CausaIntesa::SeggiPersi));
}

#[test]
fn voto_finale_segreto_solo_alla_camera() {
    let mut p = partita(con(senza_shock(), "ft_base", "0.2"));
    presenta(&mut p, &[("soglia_lista", "400")]).unwrap();
    let v = p.vista();
    p.comanda(Comando::PoniFiducia { misura: v.calendario[0].id }).unwrap();
    let maggioranza = |f: fn(&SeggiPartito) -> u32| v.seggi.iter().filter(|s| s.partito == "destra" || s.partito == "centrodestra").map(f).sum::<u32>();
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert_eq!(e.senato.si, maggioranza(|s| s.senato));
    assert_ne!(e.camera.si, maggioranza(|s| s.camera));
}

#[test]
fn bocciatura_della_legge_elettorale_non_e_una_fiducia_respinta() {
    let mut p = partita(con(con(senza_shock(), "ft_base", "0.45"), "ft_max", "0.5"));
    presenta(&mut p, &[("soglia_lista", "400")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    let e = &p.vista().esiti[0];
    assert!(!e.approvata);
    assert!(!e.fiducia_respinta);
}

#[test]
fn norma_costituzionale_in_due_deliberazioni() {
    let mut p = partita(senza_shock());
    presenta(&mut p, &[("numero_parlamentari", "300_150")]).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert_eq!(v.esiti[0].deliberazione, 1);
    assert!(v.esiti[0].approvata);
    assert_eq!(v.calendario[0].deliberazione, 2);
    p.chiudi_turno();
    assert_eq!(p.vista().esiti[0].deliberazione, 2);
}

/// Il vigente all'estremo destro e il nuovo al centro: tutti tranne l'estrema destra preferiscono il nuovo.
fn costituzionale_condivisa() -> FileDati {
    let f = sostituisci(senza_shock(), "disposizioni", "nome = \"400 deputati e 200 senatori\", posizione = 4.5", "nome = \"400 deputati e 200 senatori\", posizione = 7.0");
    let f = sostituisci(f, "disposizioni", "nome = \"630 deputati e 315 senatori\", posizione = 3.0", "nome = \"630 deputati e 315 senatori\", posizione = 4.0");
    con(con(con(f, "malus_opposizione", "0.0"), "ft_base", "0.0"), "ft_distanza", "0.0")
}

#[test]
fn con_i_due_terzi_niente_referendum() {
    let mut p = partita(costituzionale_condivisa());
    presenta(&mut p, &[("numero_parlamentari", "630_315")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("630_315"));
    assert!(p.vista().referendum.is_empty());
    let e = p.proiezione_seggi().unwrap();
    assert_eq!(e.camera.iter().map(|s| s.totale()).sum::<u32>(), 630);
    assert_eq!(e.senato.iter().map(|s| s.totale()).sum::<u32>(), 315);
}

#[test]
fn senza_i_due_terzi_si_vota_il_referendum_confermativo() {
    let f = con(con(senza_shock(), "ft_base", "0.0"), "malus_opposizione", "5.0");
    let mut p = partita(f);
    presenta(&mut p, &[("numero_parlamentari", "300_150")]).unwrap();
    p.chiudi_turno();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti[0].approvata);
    assert_eq!(v.referendum[0].tipo, TipoReferendum::Confermativo);
    assert_eq!(p.in_vigore("numero_parlamentari"), Some("400_200"));
    p.chiudi_turno();
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(p.in_vigore("numero_parlamentari") == Some("300_150"), r.si > r.no);
}
```

Nella bocciatura il voto a blocchi passa (nessun franco tiratore, la maggioranza ha più di metà dei seggi, Task 6); al voto finale, con le linee del voto a blocchi, `ft_base` 0,45 dà alla Camera `q` = 0,9 nei due sensi, quindi i sì attesi sono 0,1 · maggioranza + 0,9 · opposizione, sotto 200 perché la maggioranza ha più di 200 seggi: la Camera boccia, il Senato (senza franchi tiratori) no. `300_150` (posizione 5,5) contro `400_200` (4,5) ha la linea sì del centrodestra (`0,5 − 0,5 + 0,2 > 0`), quindi la maggioranza la porta oltre le due deliberazioni. Con `malus_opposizione` a 5 nessun partito d'opposizione vota sì: la maggioranza non arriva ai due terzi (il governo da solo non li ha) e l'opposizione, che vota no con più di un quinto dei seggi, chiede il confermativo, che non ha quorum. Più test d'unità: in `elettorato/referendum.rs` `posizioni_uguali_dividono_a_meta` (con `peso_governo = 0`) e `invito_all_astensione_abbassa_i_votanti`; in `disposizioni/stato.rs` `arrivo_che_rompe_la_legge_elettorale_si_scarta` (Review Focus 5): uno stato col premio alla coalizione e un arrivo di `coalizioni = no` nello stesso turno di nessun altro, `arrivi` lo scarta, `coalizioni` resta `si`, `legge_elettorale` passa `valida`; con anche `premio = nessuno` nello stesso turno, entrambi arrivano.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test norme`
Expected: errore di compilazione, `VoceMisura.seggi` e `Vista.referendum` non esistono.

- [ ] **Step 3: Implementa legge elettorale, deliberazioni costituzionali, confermativo e proiezione**

- [ ] **Step 4: Esegui i test, rigenera i tipi, verifica che il core resti puro**

Run: `cargo test --workspace && npm --prefix app run check && grep -rnE "std::(fs|time|thread|net|env)|HashMap|StdRng|SmallRng" crates/sim-core/src`
Expected: PASS; nessuna riga dal `grep`.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): electoral law by expected seats, constitutional norms and confirmatory referendum"
```

---

## PR C · Decreti, bilancio e referendum (Task 10-14)

### Task 10: Decreti e conversione

**Files:**
- Create: `crates/sim-core/src/parlamento/decreti.rs`
- Modify: `crates/sim-core/src/economia/mod.rs` (`decreti: Vec<i64>` per dicastero, in domanda finale), `economia/finanza.rs` (somma nei conti, azzeramento dopo la domanda)
- Modify: `crates/sim-core/src/ministri/mod.rs`, `ministri/agenda.rs` (`Agenda.decreto`, ritiro su Ferma e dimissioni), `ministri/proposte.rs` (Accogli rende il dicastero)
- Modify: `crates/sim-core/src/comando.rs` (`EmanaDecreto`), `crates/sim-core/src/partita.rs` (conversioni prima nel passo «aula», `aggiungi_decreto`), `crates/sim-core/src/vista.rs`
- Modify: `crates/sim-core/tests/proposte.rs:85`, `:90`, `:413` (`decreti` per dicastero), `crates/sim-core/tests/agenda.rs:496-501` (i due guasti di `decreti` scrivono un vettore lungo quanto i dicasteri col valore guasto nella prima voce: lo scalare di oggi verrebbe rifiutato per il tipo e il test passerebbe a vuoto), `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/decreti.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:12-15` (vale subito, decade ex tunc, sessanta giorni insuperabili), `:20-23` (materie escluse, niente reiterazione), `:67` (decreto negato non si rinnova), `:163-164`.

**Dispatch:** `rust-engineer` · primo della PR C, sul branch `feat/v1-piano-6-decreti` dalla punta della B dopo la sua review · skill-gate: true.

**Interfaces:**
- Consumes: `Ministri::accogli` (`crates/sim-core/src/ministri/proposte.rs:88-126`), `Ministri::ferma` (`crates/sim-core/src/ministri/agenda.rs:366-383`), `Ministri::comanda` (`crates/sim-core/src/ministri/mod.rs:292-354`), `Partita::aggiungi_decreto` (`crates/sim-core/src/partita.rs:349-351`), il ciclo della domanda dei dicasteri in `Economia::passo` (`crates/sim-core/src/economia/mod.rs:233-241`), voto e fiducia dei Task 7-8.
- Produces:
  - `Economia.decreti: Vec<i64>`, un valore per dicastero; nei conti `spese.dicasteri` somma il totale; nel `passo` ogni decreto entra nella domanda finale col vettore del suo dicastero, come scostamento reale del trimestre (`decreti[d] / MLD / livello_prezzi`), e i decreti si azzerano dopo la domanda, non più in `finanza`.
  - `Agenda.decreto: i64` (l'importo del decreto di Accogli che l'ha messa in agenda, 0 altrimenti).
  - `Ministri::comanda(…) -> Result<EffettoComando, Errore>`, `EffettoComando { Nessuno, Decreto { dicastero: usize, importo: i64 }, Ritiro { dicastero: usize, importo: i64 } }`.
  - `Comando::EmanaDecreto { decreto: Decreto }`; `sim_core::parlamento::Decreto { Fondi { da: String, a: String, importo: i64 }, Disposizioni { valori: Vec<ValoreProposto> } }`, serde `tag = "tipo"`, TS esportato.
  - `Misura` in più: `pub fondi: Option<FondiDecreto>`; `FondiDecreto { Accolti { dicastero: usize, atto: usize, importo: i64 }, Spostati { da: usize, a: usize, importo: i64 } }`. `Parlamento.decaduti: Vec<FondiDecreto>` (firme dei decreti decaduti, per il divieto di ripetere).
  - `Causa::FondiTolti` («fondi spostati ad altro dicastero»), `Causa::DecretoDecaduto` («decreto non convertito»).

Regole:
- **Accogli** (piano 5) apre una misura `Conversione` con `voto = turno`, `proponente` = il dicastero, `fondi = Accolti`, e `Agenda.decreto = importo`. Rifiuto se un decreto con la stessa firma (dicastero e atto) è decaduto: `decreto decaduto non si ripete: {atto}`.
- **Ritiro** (scelta 1): `Ferma` sull'agenda col decreto, o `ChiediDimissioni` sul suo dicastero, ritira il decreto: `margine_residuo −= importo`, `economia.decreti[i] −= importo`, la conversione esce dal calendario, `Agenda.decreto = 0`; resta solo il costo del comando. `Dirotta` tiene il decreto.
- **Spostamento di fondi** (storia 46): `da ≠ a`, dicasteri noti, `0 < importo ≤ min(margine_residuo di da, margine annuo di da nel bilancio in vigore)`; il bilancio in vigore sposta `importo` da `margine` e `spesa` di `da` a quelli di `a`, i margini residui pure; il ministro di `da` ha `FondiTolti` di `−costo_taglio_budget · 100 · importo / spesa di da`. Si converte come Accogli. Ammesso anche nell'esercizio provvisorio (scelta 12). Rifiuti: `dicastero sconosciuto: {id}`, `stesso dicastero: {id}`, `importo non ammesso: {importo}`, `margine insufficiente in {da}`, `decreto decaduto non si ripete: {da} → {a}`.
- **Decreto su disposizioni**: prima la materia, poi l'innesco (Confini): elettorali e costituzionali `materia esclusa dal decreto: {id}`; altrimenti `decreto senza innesco: serve un evento aperto`. Il bilancio non è un contenuto possibile del tipo `Decreto`: il divieto vale per costruzione.
- **Conversione**, primo voto del passo «aula», anche nel turno del bilancio (scelta 2): linea maggioranza sì, opposizione no; franchi tiratori dalla sola intesa (scelta 4); fiducia ammessa. **Decaduta**: il denaro speso resta speso; uno spostamento torna indietro per `min(importo, margine residuo di a)`; il proponente ha `DecretoDecaduto` di `−costo_decreto_decaduto`; il partito del Presidente del Consiglio `valence −= valence_decreto_decaduto`; la firma va in `decaduti`.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `costo_decreto_decaduto` 10 (0-30, "08"), `valence_decreto_decaduto` 0.05 (0-0.5, "08").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/decreti.rs
mod comune;
use comune::{dati_base, ferma_tranne, file_base, imposta, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::{Decreto, FormaMisura, ValoreProposto};
use sim_core::{Comando, Errore, Partita};

/// La Sanità con i quattro atti a 9 quote, come in tests/proposte.rs: il ministro chiede fondi.
fn sanita_cara() -> FileDati {
    let effetto = "effetti = [\n  { grandezza = \"qualita_servizio_sanitario\", intensita = ";
    let mut f = file_base();
    for coda in ["8.0, ritardo = 1, forma = \"impulso\", durata = 2.0", "8.0, ritardo = 1, forma = \"impulso\", durata = 1.5", "4.0", "2.0"] {
        let costo = if coda == "2.0" { "costo = 0.5\n" } else { "costo = 1.0\n" };
        let da = format!("{costo}{effetto}{coda}");
        f = sostituisci(f, "atti", &da, &da.replacen(costo, "costo = 9.0\n", 1));
    }
    f
}

fn accolta(f: FileDati) -> (Partita, i64) {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    let importo = p.vista().ministeri[3].proposta.as_ref().unwrap().importo;
    p.comanda(Comando::Accogli { dicastero: "sanita".into() }).unwrap();
    (p, importo)
}

fn sposta(da: &str, a: &str, importo: i64) -> Comando {
    Comando::EmanaDecreto { decreto: Decreto::Fondi { da: da.into(), a: a.into(), importo } }
}

#[test]
fn accogli_apre_una_conversione_a_fine_turno() {
    let (mut p, importo) = accolta(sanita_cara());
    let v = p.vista();
    let m = v.calendario.iter().find(|m| m.forma == FormaMisura::Conversione).unwrap();
    assert_eq!(m.proponente.as_deref(), Some("sanita"));
    assert_eq!(p.economia().decreti[3], importo);
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.forma == FormaMisura::Conversione));
    assert_eq!(p.economia().decreti.iter().sum::<i64>(), 0);
}

#[test]
fn ferma_dopo_accogli_ritira_il_decreto() {
    let prima = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1).vista().ministeri[3].margine_residuo;
    let (mut p, _) = accolta(sanita_cara());
    p.comanda(Comando::Ferma { dicastero: "sanita".into() }).unwrap();
    assert_eq!(p.economia().decreti[3], 0);
    assert_eq!(p.vista().ministeri[3].margine_residuo, prima);
    assert!(p.vista().calendario.iter().all(|m| m.forma != FormaMisura::Conversione));
}

#[test]
fn dimissioni_dopo_accogli_ritirano_il_decreto() {
    let (mut p, _) = accolta(sanita_cara());
    p.comanda(Comando::ChiediDimissioni { dicastero: "sanita".into() }).unwrap();
    assert_eq!(p.economia().decreti[3], 0);
}

#[test]
fn dirotta_dopo_accogli_tiene_il_decreto() {
    let (mut p, importo) = accolta(sanita_cara());
    let v = p.vista();
    let m = &v.ministeri[3];
    // I quattro atti costano tutti 9 quote: il margine accolto per uno paga anche gli altri.
    let altro = m.atti.iter().find(|a| a.pagabile && a.id != m.agenda.as_ref().unwrap().atto).expect("un altro atto pagabile");
    p.comanda(Comando::Dirotta { dicastero: "sanita".into(), atto: Some(altro.id.clone()), regione: None }).unwrap();
    assert_eq!(p.economia().decreti[3], importo);
    assert!(p.vista().calendario.iter().any(|m| m.forma == FormaMisura::Conversione));
}

#[test]
fn decreto_accolto_entra_nella_domanda_del_dicastero() {
    let (mut a, _) = accolta(sanita_cara());
    let mut b = Partita::nuova(DatiGioco::da_testi(&sanita_cara()).unwrap(), 1);
    b.comanda(Comando::Respingi { dicastero: "sanita".into() }).unwrap();
    for p in [&mut a, &mut b] {
        // Fermare la Sanità in `a` ritirerebbe il decreto; in `b` non ha agenda.
        ferma_tranne(p, "sanita");
        p.chiudi_turno();
    }
    let d = dati_base();
    let st = d.struttura();
    // La Sanità non ha un vettore proprio (`dati/base/dicasteri.toml`): usa la spesa pubblica nazionale.
    let quote: Vec<f64> = match &d.dicasteri()[3].settori {
        Some(m) => st.settori.iter().map(|s| m.get(&s.id).copied().unwrap_or(0.0)).collect(),
        None => st.spesa_pubblica.clone(),
    };
    let k = (0..quote.len()).max_by(|&x, &y| quote[x].total_cmp(&quote[y])).unwrap();
    assert!(a.economia().gap_settori[k] > b.economia().gap_settori[k]);
}

#[test]
fn spostare_fondi_per_decreto_muove_margine_e_bilancio() {
    // Al turno zero il bilancio in vigore è quello del 2026 (`Economia.bilancio`).
    let mut p = Partita::nuova(dati_base(), 1);
    let (margine, spesa) = (p.economia().bilancio.dicasteri[8].margine, p.economia().bilancio.dicasteri[3].spesa);
    p.comanda(sposta("giustizia", "sanita", 1_000_000)).unwrap();
    assert_eq!(p.economia().bilancio.dicasteri[8].margine, margine - 1_000_000);
    assert_eq!(p.economia().bilancio.dicasteri[3].spesa, spesa + 1_000_000);
    assert!(p.vista().calendario.iter().any(|m| m.forma == FormaMisura::Conversione));
}

#[test]
fn decreti_non_ammessi_rifiutati_col_nome() {
    let mut p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    let disp = |d: &str, g: &str| Comando::EmanaDecreto { decreto: Decreto::Disposizioni { valori: vec![ValoreProposto { disposizione: d.into(), gradino: g.into() }] } };
    for (c, nome) in [
        (disp("soglia_lista", "0"), "materia esclusa dal decreto: soglia_lista"),
        (disp("numero_parlamentari", "630_315"), "materia esclusa dal decreto: numero_parlamentari"),
        (disp("eta_pensionabile", "68"), "decreto senza innesco"),
        (sposta("sanita", "sanita", 1), "stesso dicastero: sanita"),
        (sposta("sanita", "giustizia", 0), "importo non ammesso: 0"),
        (sposta("sanità", "giustizia", 1), "dicastero sconosciuto: sanità"),
    ] {
        let e = p.comanda(c).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn conversione_respinta_costa_e_non_si_ripete() {
    let f = imposta(imposta(sanita_cara(), "manopole", "[ft_base]\nvalore = ", "0.9"), "manopole", "[ft_max]\nvalore = ", "1.0");
    let (mut p, _) = accolta(f);
    // Bilancio presentato al turno 0: il turno 1 non è in esercizio provvisorio, dove ogni Accogli è rifiutato.
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
    p.chiudi_turno();
    assert!(p.vista().esiti.iter().any(|e| e.forma == FormaMisura::Conversione && !e.approvata));
    // Gli atti restano impagabili anche col margine di un anno: la proposta rinasce sullo stesso atto.
    assert!(p.vista().ministeri[3].proposta.is_some());
    let e = p.comanda(Comando::Accogli { dicastero: "sanita".into() }).unwrap_err();
    assert!(e.to_string().contains("decreto decaduto non si ripete"), "{e}");
}
```

`sanita_cara` copia l'helper di `crates/sim-core/tests/proposte.rs`, che si appoggia al testo verbatim della Sanità. L'indice 3 dei dicasteri è la Sanità, l'8 la Giustizia. Nel test della domanda le due partite divergono solo per il decreto: in `b` la proposta è respinta. `salvare_a_meta_turno_conserva_l_aula` (Task 8) si estende con uno spostamento di fondi per decreto (`sposta("giustizia", "sanita", 1_000_000)`) prima del salvataggio.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test decreti`
Expected: errore di compilazione, `Comando::EmanaDecreto` non esiste.

- [ ] **Step 3: Implementa decreti per dicastero, ritiro, spostamento e conversione**

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS, compresi i test delle proposte del piano 5 con `decreti` per dicastero.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): decrees with same-turn conversion, withdrawal and sector demand"
```

---

### Task 11: La legge di bilancio in aula

**Files:**
- Modify: `crates/sim-core/src/partita.rs` (passo «aula»: voto del bilancio; slittamento delle leggi nel turno del bilancio), `crates/sim-core/src/comando.rs` (`PresentaBilancio` apre la misura), `crates/sim-core/src/parlamento/mod.rs`, `parlamento/emendamenti.rs` (fiducia sulle leggi rifiutata nel turno del bilancio; default degli emendamenti delle leggi che slittano)
- Modify: `crates/sim-core/tests/decreti.rs` (fiducia sul bilancio in `conversione_respinta_costa_e_non_si_ripete`)
- Test: `crates/sim-core/tests/bilancio_in_aula.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:85-87` (presentazione entro il 20 ottobre, esercizio provvisorio fino a quattro mesi, bilancio mai in commissione né a voto segreto), `:174`; `2026-10-25-v1-piano-5-governo.md:25` (aula senza voto, da sostituire).

**Dispatch:** `rust-engineer` · dopo il Task 10 · skill-gate: true.

**Interfaces:**
- Consumes: `Partita::aula` del piano 5 (`crates/sim-core/src/partita.rs:255-271`), `presenta_bilancio` (`:309-313`), `bozza_bilancio`, voto e fiducia dei Task 7-8.
- Produces:
  - `PresentaBilancio` apre (o sostituisce) una misura `Bilancio` con `voto = turno`, senza proponente nel calendario ordinario; ripresentare tiene l'`id` e la fiducia già posta.
  - `Partita::turno_del_bilancio(&self) -> bool`: in calendario c'è una misura `Bilancio` con `voto == turno`, cioè un bilancio presentato in questo turno (ottobre-dicembre o esercizio provvisorio). Senza bilancio presentato è `false`, anche al turno 0 e nell'esercizio provvisorio.

Regole:
- **Voto** dopo le conversioni e prima delle leggi: linea maggioranza sì, opposizione no; franchi tiratori dalla sola intesa (scelta 4); fiducia ammessa. **Approvato**: il passo del piano 5 (margine residuo nell'esercizio provvisorio, costo del taglio, `bilancio_approvato`). **Respinto**: `bilancio_presentato` torna `None` e vale ciò che il piano 5 fa senza bilancio (esercizio provvisorio, poi l'anno con le cifre vecchie).
- **Calendario occupato** (scelta 2 e 11): `turno_del_bilancio()` si calcola una volta, all'inizio del passo «aula», prima del voto del bilancio (dopo, un bilancio approvato è uscito dal calendario e `bozza_bilancio()` rende `None`). Nel turno del bilancio le riforme, le leggi elettorali e le deliberazioni costituzionali con `voto == turno` passano a `voto = turno + 1` prima del voto e perdono la fiducia posta, senza costo. I loro emendamenti aperti slittano con loro: il primo passo della chiusura, che legge lo stesso `turno_del_bilancio()` (il bilancio non è ancora votato), non li respinge per default, e il default scatta nella chiusura del turno del nuovo voto. La fiducia su di esse è rifiutata solo se il bilancio è in calendario per quel turno (`il bilancio occupa il calendario: niente fiducia su {id} in questo turno`); presentare resta ammesso. Le conversioni si votano.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/bilancio_in_aula.rs
mod comune;
use comune::{imposta, presenta, senza_shock};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::FormaMisura;
use sim_core::{Comando, Partita, Trimestre};

fn al_turno(f: FileDati, turni: usize) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    for _ in 0..turni {
        p.chiudi_turno();
    }
    p
}

fn presenta_bozza(p: &mut Partita) {
    let b = p.bozza_bilancio().unwrap();
    p.comanda(Comando::PresentaBilancio { bilancio: b }).unwrap();
}

#[test]
fn bilancio_si_vota_e_passa_con_la_maggioranza() {
    let mut p = al_turno(senza_shock(), 4);
    presenta_bozza(&mut p);
    assert!(p.vista().calendario.iter().any(|m| m.forma == FormaMisura::Bilancio));
    p.chiudi_turno();
    let e = p.vista().esiti.into_iter().find(|e| e.forma == FormaMisura::Bilancio).unwrap();
    assert!(e.approvata);
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn bilancio_respinto_porta_l_esercizio_provvisorio() {
    let f = imposta(imposta(senza_shock(), "manopole", "[ft_base]\nvalore = ", "0.9"), "manopole", "[ft_max]\nvalore = ", "1.0");
    let mut p = al_turno(f, 4);
    presenta_bozza(&mut p);
    p.chiudi_turno();
    assert!(p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn fiducia_sul_bilancio_senza_franchi_tiratori() {
    let f = imposta(senza_shock(), "manopole", "[ft_base]\nvalore = ", "0.4");
    let mut p = al_turno(f, 4);
    presenta_bozza(&mut p);
    let id = p.vista().calendario.iter().find(|m| m.forma == FormaMisura::Bilancio).unwrap().id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert!(!p.vista().bilancio.esercizio_provvisorio);
}

#[test]
fn leggi_in_scadenza_slittano_nel_turno_del_bilancio() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    presenta_bozza(&mut p);
    let id = p.vista().calendario.iter().find(|m| m.forma == FormaMisura::Riforma).unwrap().id;
    let e = p.comanda(Comando::PoniFiducia { misura: id }).unwrap_err();
    assert!(e.to_string().contains("il bilancio occupa il calendario"), "{e}");
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti.iter().all(|e| e.forma != FormaMisura::Riforma));
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!(m.voto, Trimestre { anno: 2028, trimestre: 1 });
}

#[test]
fn senza_bilancio_presentato_le_leggi_si_votano() {
    // Turno 0 (ottobre-dicembre) e turno 1 (esercizio provvisorio), nessun bilancio presentato.
    let mut p = al_turno(senza_shock(), 0);
    assert!(p.bozza_bilancio().is_some() && !p.turno_del_bilancio());
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    presenta(&mut p, &[("ticket", "alto")]).unwrap();
    let id = p.vista().calendario[0].id;
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    p.chiudi_turno();
    assert_eq!(p.in_vigore("eta_pensionabile"), Some("68"));
    assert!(p.vista().bilancio.esercizio_provvisorio && !p.turno_del_bilancio());
    p.chiudi_turno();
    assert!(p.vista().calendario.is_empty());
}

#[test]
fn approvato_il_bilancio_le_leggi_slittano_lo_stesso() {
    let mut p = al_turno(senza_shock(), 3);
    presenta(&mut p, &[("eta_pensionabile", "68")]).unwrap();
    p.chiudi_turno();
    let id = p.vista().calendario.iter().find(|m| m.forma == FormaMisura::Riforma).unwrap().id;
    // Fiducia posta prima di presentare il bilancio: ammessa, e persa con lo slittamento.
    p.comanda(Comando::PoniFiducia { misura: id }).unwrap();
    presenta_bozza(&mut p);
    p.chiudi_turno();
    let v = p.vista();
    assert!(v.esiti.iter().any(|e| e.forma == FormaMisura::Bilancio && e.approvata));
    assert!(v.esiti.iter().all(|e| e.forma != FormaMisura::Riforma));
    let m = v.calendario.iter().find(|m| m.id == id).unwrap();
    assert_eq!((m.voto, m.fiducia), (Trimestre { anno: 2028, trimestre: 1 }, false));
}
```

I test del piano 5 che presentano un bilancio (`crates/sim-core/tests/bilancio.rs`) passano dal voto: con `ft_base` 0,02 e la maggioranza del Task 6 lo approvano. In `conversione_respinta_costa_e_non_si_ripete` (Task 10), dopo `PresentaBilancio`, si aggiunge la fiducia sulla misura `Bilancio` (`PoniFiducia` col suo `id`): con `ft_base` 0,9 il voto senza fiducia respingerebbe il bilancio e il turno 1 tornerebbe in esercizio provvisorio, dove Accogli è rifiutato prima del controllo sul decreto decaduto.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test bilancio_in_aula`
Expected: errore di compilazione, `Partita::turno_del_bilancio` non esiste.

- [ ] **Step 3: Implementa voto del bilancio, calendario occupato e fiducia**

- [ ] **Step 4: Esegui i test**

Run: `cargo test --workspace`
Expected: PASS, compresi i test del bilancio del piano 5.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core
git commit -m "feat(core): budget law floor vote occupying the calendar"
```

---

### Task 12: Proposte normative dei ministri

**Files:**
- Modify: `crates/sim-core/src/ministri/proposte.rs` (`TipoProposta`, inneschi 2 e 3, Accogli di una riforma, `controlla_proposte`), `crates/sim-core/src/ministri/mod.rs` (`Contesto.parlamento`, `EffettoComando::Riforma`), `crates/sim-core/src/comando.rs`, `crates/sim-core/src/partita.rs` (Accogli di una riforma passa da `PresentaLegge`), `crates/sim-core/src/vista.rs`
- Test: `crates/sim-core/tests/proposte_normative.rs`

**Ricerca:** - nessun riferimento pertinente (regole dei ticket 04 e 16: una proposta per ministro, disposizione più lontana di almeno 2 punti portata al valore più vicino, oppure lo sblocco del prerequisito; `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:38`).

**Dispatch:** `rust-engineer` · dopo il Task 11 · skill-gate: true.

**Interfaces:**
- Consumes: `Proposta` e `proponi` del piano 5 (`crates/sim-core/src/ministri/proposte.rs:15-73`), `Prerequisito` del Task 3, presentazione del Task 6, `EffettoComando` del Task 10, `Contesto` dei ministri (`crates/sim-core/src/ministri/mod.rs:113-124`).
- Produces:
  - `Proposta { pub tipo: TipoProposta, pub turni_in_sospeso: u32, pub rinviata: bool }`, `TipoProposta { Fondi { atto: usize, importo: i64 }, Riforma { disposizione: usize, gradino: usize } }`; il salvataggio controlla anche la nuova forma.
  - `Contesto.parlamento: &'a Parlamento` (tutti i costruttori di `Contesto` in `partita.rs`): l'innesco 3 legge le disposizioni in esame.
  - `EffettoComando::Riforma { dicastero: usize, disposizione: usize, gradino: usize }`: `Ministri::comanda` lo rende per Accogli di una proposta di riforma senza toccare la proposta; `Partita` presenta la legge e toglie la proposta solo se la presentazione passa.
  - `VoceProposta` in più: `pub disposizione: Option<String>`, `pub gradino: Option<String>`; `atto` e `importo` restano per le proposte di fondi (`importo = 0` per le riforme).

Regole:
- **Inneschi**, all'apertura dopo le agende, per un ministro in carica senza proposta, nell'ordine: (1) fondi, come nel piano 5; (2) sblocco: il suo atto con l'`o` più alto (`o > 0`) ha il prerequisito non soddisfatto → la disposizione del prerequisito al primo gradino ammesso; (3) distanza: fra le disposizioni ordinarie del suo dicastero non in esame e senza gradini bloccati, quella con `|pos_vigore − pos_ministro|` più alta, se `≥ distanza_alleato`, al gradino con la posizione più vicina alla sua (a parità quello più vicino al vigente). Motivi: `"sblocca {nome dell'atto}"`, `"{nome della disposizione}: lontana dalla sua posizione"`.
- **Riforme e agenda**: gli inneschi 2 e 3 valgono anche per un ministro con un'agenda; una proposta di riforma convive con l'agenda e ha importo 0. Il piano 5 lo impedisce oggi in due punti, che qui si limitano alle proposte di fondi: `proponi` scarta la proposta con un'agenda e fuori da `1..=LIMITE_EURO` (`crates/sim-core/src/ministri/proposte.rs:43-68`), `controlla_proposte` rifiuta una proposta con agenda o con l'importo fuori intervallo (`proposte.rs:150-157`).
- **Accogli** di una riforma = `PresentaLegge` con quella disposizione e quel gradino; il rifiuto della presentazione (calendario pieno, respinta da poco) è il rifiuto di Accogli e la proposta resta. Nessun costo di firma: la vuole lui. Nell'esercizio provvisorio resta rifiutato come ogni Accogli, dal controllo che scatta prima del dispatch (`crates/sim-core/src/ministri/mod.rs:308-312`). Rinvia, Respingi e il default come per i fondi.

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/proposte_normative.rs
mod comune;
use comune::{file_base, presenta, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::{Comando, Partita};

/// Licenziamento a reintegro (posizione 2): il ministro del Lavoro della destra (6 ± scarto) è lontano.
fn reintegro() -> FileDati {
    sostituisci(file_base(), "disposizioni", "partenza = \"tutele_crescenti\"", "partenza = \"reintegro\"")
}

/// La disposizione della proposta di riforma del ministro del Lavoro, se ne ha una.
fn proposta_di_riforma(p: &Partita) -> Option<String> {
    let v = p.vista();
    v.ministeri.iter().find(|m| m.dicastero == "lavoro_welfare")?.proposta.as_ref()?.disposizione.clone()
}

#[test]
fn ministro_lontano_da_una_disposizione_propone_una_riforma() {
    let mut trovata = false;
    for seme in 1..=20 {
        let v = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme).vista();
        let m = v.ministeri.iter().find(|m| m.dicastero == "lavoro_welfare").unwrap();
        if let Some(p) = m.proposta.as_ref().filter(|p| p.disposizione.is_some()) {
            assert!(p.motivo.ends_with("lontana dalla sua posizione"), "{}", p.motivo);
            assert_eq!(p.importo, 0);
            trovata = true;
        }
    }
    assert!(trovata);
}

#[test]
fn accogliere_la_proposta_presenta_la_legge() {
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme);
        let Some(d) = proposta_di_riforma(&p) else { continue };
        p.comanda(Comando::Accogli { dicastero: "lavoro_welfare".into() }).unwrap();
        assert_eq!(p.vista().calendario[0].valori[0].disposizione, d);
        trovata = true;
    }
    assert!(trovata);
}

#[test]
fn proposta_di_sblocco_del_prerequisito() {
    let f = sostituisci(file_base(), "disposizioni", "partenza = \"attuale\"\nlegge = \"L. 199/2025, art. 1 c. 400\"", "partenza = \"ridotto\"\nlegge = \"L. 199/2025, art. 1 c. 400\"");
    let mut trovata = false;
    for seme in 1..=20 {
        let v = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), seme).vista();
        let m = v.ministeri.iter().find(|m| m.dicastero == "sanita").unwrap();
        if let Some(p) = &m.proposta {
            if p.motivo.starts_with("sblocca") {
                assert_eq!((p.disposizione.as_deref(), p.gradino.as_deref()), (Some("privato_accreditato"), Some("attuale")));
                trovata = true;
            }
        }
    }
    assert!(trovata);
}

#[test]
fn calendario_pieno_rifiuta_accogli_e_tiene_la_proposta() {
    let mut trovata = false;
    for seme in 1..=20 {
        let mut p = Partita::nuova(DatiGioco::da_testi(&reintegro()).unwrap(), seme);
        if proposta_di_riforma(&p).is_none() {
            continue;
        }
        for (d, g) in [("ticket", "alto"), ("obbligo_scolastico", "18"), ("leva_militare", "obbligatoria")] {
            presenta(&mut p, &[(d, g)]).unwrap();
        }
        let e = p.comanda(Comando::Accogli { dicastero: "lavoro_welfare".into() }).unwrap_err();
        assert!(e.to_string().contains("calendario pieno"), "{e}");
        assert!(proposta_di_riforma(&p).is_some());
        trovata = true;
    }
    assert!(trovata);
}
```

Ogni test cicla sui semi 1-20 col flag `trovata`, come `proposta_di_sblocco_del_prerequisito`: col seme il ministro può avere prima una proposta di fondi (innesco 1, che ha la precedenza), e ogni test deve trovare almeno un seme con la proposta che cerca.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test proposte_normative`
Expected: errore di compilazione, `VoceProposta.disposizione` non esiste.

- [ ] **Step 3: Implementa i due inneschi e Accogli di una riforma**

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS, compresi i test delle proposte di fondi del piano 5.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core app/src/lib/tipi
git commit -m "feat(core): ministers propose reforms and prerequisite unlocks"
```

---

### Task 13: Anteprima della misura

**Files:**
- Create: `crates/sim-core/src/parlamento/anteprima.rs`
- Modify: `crates/sim-core/src/partita.rs` (`anteprima`), `crates/sim-core/src/vista.rs`, `dati/base/manopole.toml`
- Modify: `crates/sim-core/src/elettorato/mod.rs` (`Condizione` deriva `TS`, oggi no: `elettorato/mod.rs:28-29`)
- Test: `crates/sim-core/tests/anteprima.rs`

**Ricerca:** `docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:21` (la competenza del ministro corregge la stima), `:24` (tre livelli: stato vero, statistica, rapporto del consigliere); spec righe 371 (anteprima: costo esatto, effetti a fasce con l'errore che cala con la capacità), storia 55.

**Dispatch:** `rust-engineer` · dopo il Task 12 · skill-gate: true.

**Interfaces:**
- Consumes: validazione della presentazione (Task 6), linee e forbice (Task 7), seggi della legge elettorale (Task 9), `Gradino.effetti`, `saldo`, `leve`, `gruppi`; `Caso::estemporaneo` (`crates/sim-core/src/partita.rs:416-417`).
- Produces:
  - `Partita::anteprima(&self, valori: &[ValoreProposto]) -> Result<Anteprima, Errore>`: le stesse regole di `PresentaLegge` tranne il posto in calendario; non cambia la partita.
  - `Anteprima { pub nome: String, pub forma: FormaMisura, pub proponente: Option<String>, pub costo_annuo: i64, pub effetti: Vec<EffettoStimato>, pub leve: Vec<VoceLeva>, pub gruppi: Vec<Condizione>, pub linee: Vec<VoceLinea>, pub forbice: Forbice, pub seggi: Vec<VariazioneSeggi> }`; `EffettoStimato { pub grandezza: String, pub migliora: bool, pub fascia: Fascia3, pub ritardo: u32 }`; `Fascia3 { Poco, Medio, Molto }`; `VoceLeva { pub leva: String, pub da: f64, pub a: f64 }`; TS esportati. `Anteprima` e i tipi che contiene, compresi `Forbice` e `VariazioneSeggi`, derivano `Clone, Debug, PartialEq`: il test le confronta con `assert_eq!`.
  - `pub(crate) fn stima_fascia(vera: Fascia3, capacita: f64, u1: f64, u2: f64, errore: f64) -> Fascia3`, la regola della stima qui sotto, pura.

Regole:
- **Costo annuo** esatto: `Σ (saldo nuovo − saldo vigente)/100 ·` PIL potenziale annuo nominale, arrotondato all'euro.
- **Fascia vera** di un effetto: `r = |I_nuovo − I_vigente| / (max − min)` della grandezza; `Poco` sotto `soglia_fascia_poco`, `Medio` sotto `soglia_fascia_medio`, altrimenti `Molto`. `migliora` dal segno nel verso della grandezza; `ritardo` quello scritto.
- **Stima del ministro** (scelta 8): con `c` la capacità effettiva del proponente (3 senza proponente o in interim), due uniformi dell'estrattore estemporaneo con chiave `turno · 1_000_003 + Σ_k (k + 1) · (64 · disposizione_k + gradino_k)`; se la prima è `< errore_fascia · (5 − c)/4` la fascia si sposta di uno, in su o in giù secondo la seconda, limitata alle tre. Stessa misura, stesso turno, stessa stima.
- **Leve**: una voce per leva il cui valore cambia (`da` vigente, `a` nuovo). **Gruppi**: unione dei `gruppi` delle disposizioni. **Linee, forbice, seggi**: come in calendario.
- `VoceMisura` resta com'è (linee, forbice e seggi dei Task 7 e 9): `anteprima` riusa le stesse funzioni, non le duplica.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `errore_fascia` 0.4 (0-1, "25"), `soglia_fascia_poco` 0.02 (0-0.5, "25"), `soglia_fascia_medio` 0.05 (0-0.5, "25").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/anteprima.rs
mod comune;
use comune::dati_base;
use sim_core::parlamento::{FormaMisura, ValoreProposto};
use sim_core::Partita;

fn v(d: &str, g: &str) -> Vec<ValoreProposto> {
    vec![ValoreProposto { disposizione: d.into(), gradino: g.into() }]
}

#[test]
fn anteprima_da_costo_effetti_leve_e_linee() {
    let p = Partita::nuova(dati_base(), 1);
    let a = p.anteprima(&v("privato_accreditato", "ampio")).unwrap();
    assert_eq!((a.forma, a.proponente.as_deref()), (FormaMisura::Riforma, Some("sanita")));
    assert!(a.costo_annuo > 0);
    assert_eq!(a.effetti.len(), 1);
    assert!(a.effetti[0].migliora && a.effetti[0].ritardo == 1);
    assert_eq!(a.linee.len(), 7);
    let e = p.anteprima(&v("eta_pensionabile", "70")).unwrap();
    assert_eq!((e.leve[0].leva.as_str(), e.leve[0].da, e.leve[0].a), ("eta_pensione", 67.0, 70.0));
}

#[test]
fn anteprima_non_cambia_la_partita() {
    let p = Partita::nuova(dati_base(), 1);
    let prima = p.salva().unwrap();
    let a = p.anteprima(&v("privato_accreditato", "ampio")).unwrap();
    assert_eq!(a, p.anteprima(&v("privato_accreditato", "ampio")).unwrap());
    assert_eq!(p.salva().unwrap(), prima);
}

#[test]
fn anteprima_rifiuta_come_la_presentazione() {
    let p = Partita::nuova(dati_base(), 1);
    let e = p.anteprima(&v("eta_pensionabile", "67")).unwrap_err();
    assert!(e.to_string().contains("eta_pensionabile è già a 67 anni"), "{e}");
}

#[test]
fn anteprima_della_legge_elettorale_da_i_seggi() {
    let p = Partita::nuova(dati_base(), 1);
    let a = p.anteprima(&v("soglia_lista", "0")).unwrap();
    assert_eq!(a.seggi.len(), 7);
    assert!(a.effetti.is_empty() && a.costo_annuo == 0);
}
```

La capacità del ministro esce dal seme, quindi il legame fra capacità ed errore si fissa sulla funzione pura, con test d'unità in `parlamento/anteprima.rs`: `capacita_5_non_sbaglia_mai` (per ogni `u1`, la fascia resta la vera), `capacita_1_sbaglia_quando_u1_sta_sotto_l_errore` (`u1 = errore − ε` sposta, `u1 = errore + ε` no), `fascia_resta_fra_le_tre` (`Poco` spinta in giù resta `Poco`, `Molto` in su resta `Molto`), `u2_sceglie_il_verso` (`u2 < 0,5` in giù, altrimenti in su). La fascia vera di `ampio` su `qualita_servizio_sanitario` è `Poco` (3 su un intervallo di 200 fa 0,015).

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test anteprima`
Expected: errore di compilazione, `Partita::anteprima` non esiste.

- [ ] **Step 3: Implementa l'anteprima e la stima a fasce**

- [ ] **Step 4: Esegui i test, rigenera i tipi**

Run: `cargo test --workspace && npm --prefix app run check`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): measure preview with cost, estimated bands, lines and seats"
```

---

### Task 14: Referendum abrogativo

**Files:**
- Modify: `crates/sim-core/src/parlamento/mod.rs` (promozione, firme, voto), `crates/sim-core/src/partita.rs` (`promuovi_referendum`, passo «aula»), `crates/sim-core/src/comando.rs` (`InvitaAstensione`), `crates/sim-core/src/disposizioni/stato.rs` (gradini bloccati), `crates/sim-core/src/vista.rs`
- Modify: `dati/base/manopole.toml`
- Test: `crates/sim-core/tests/referendum.rs`

**Ricerca:** `docs/ricerca/modelli/iter-legislativo-italiano.md:131-137` (deposito entro il 30 settembre, voto fra aprile e giugno, quorum, blocchi elettorali, in turni), `:153`; `docs/ricerca/sintesi/disposizioni-al-turno-zero.md:62-80` (cosa torna dopo l'abrogazione); `.scratch/v1-giocabile/issues/24-abrogazione-e-gradini-di-partenza.md:25-35`.

**Dispatch:** `rust-engineer` · dopo il Task 13 · skill-gate: true. Chiude la PR C, poi il round di review; il `craft-reviewer` riceve anche l'elenco dei testi per il giocatore dei Task 6-14 (rifiuti, motivi, nomi delle cause), estratto con `grep -rhoE '"[^"]*\{[^"]*"' crates/sim-core/src/parlamento crates/sim-core/src/disposizioni`, per la revisione di Mario insieme a quelli del piano 5.

**Interfaces:**
- Consumes: `Referendum`, `Affluenza`, `Elettorato::referendum` del Task 9; `StatoDisposizione.bloccati`; `DatoDisposizione.dopo_abrogazione`, `referendabile`.
- Produces:
  - `Partita::promuovi_referendum(&mut self, promotore: &str, disposizione: &str) -> Result<(), Errore>`, pubblica: la chiameranno il comando del giocatore all'opposizione (piano 7) e i partiti IA (piano 8).
  - `Comando::InvitaAstensione`.
  - `Vista.referendum` porta anche l'abrogativo, con la fase: `VistaReferendum` in più `pub firme: bool` (`false` firme in raccolta, `true` indetto; il confermativo sempre `true`).

Regole:
- **Promozione**, rifiuti esatti in quest'ordine: promotore sconosciuto (`partito sconosciuto: {id}`) o al governo (`il promotore è al governo: {id}`); disposizione sconosciuta; non referendabile (`{id} non è referendabile`); senza legge, o col valore dopo l'abrogazione uguale al vigente e nessun arrivo da cancellare (`niente da abrogare: {id}`); elettorale che darebbe una legge non valida (`quesito inammissibile: {messaggio}`); un referendum abrogativo già in corso (`un referendum è già in corso`); fuori finestra, cioè dopo luglio-settembre (`fuori finestra: si chiede entro luglio-settembre`).
- **Fasi**: firme raccolte nella chiusura del turno di promozione (`firme = true`); voto nel turno aprile-giugno dell'anno dopo (`voto` = indice di quel turno).
- **Invito all'astensione**: ammesso con un abrogativo indetto, una volta (`nessun referendum da disertare`, `astensione già chiesta`).
- **Voto**, nel passo «aula» del suo turno: `Elettorato::referendum` con `pos_si` = posizione del gradino dopo l'abrogazione, `pos_no` = del vigente, `peso_governo = 0` (scelta 9), `invito = invito_astensione` se chiesto. Quorum: `votanti > aventi_diritto / 2`. Quorum e `si > no`: la disposizione va al valore dopo l'abrogazione con la legge `"referendum del {anno}"` (gli arrivi si cancellano), il gradino abrogato entra in `bloccati`, il partito del Presidente del Consiglio `valence −= valence_referendum_perso`. Senza quorum: il promotore `valence −= valence_referendum_fallito`. Esito in `esiti_referendum`.
- **Blocco**: `PresentaLegge` rifiuta già un gradino bloccato (Task 6); la vista lo segna in `VoceGradino.bloccato`. Lo sblocco a legislatura nuova è del piano 7.

Manopole nuove, `# assunzione del piano 6, nessuna fonte in docs/ricerca`: `valence_referendum_perso` 0.1 (0-0.5, "12"), `valence_referendum_fallito` 0.05 (0-0.5, "12"), `invito_astensione` 1.0 (0-5, "12").

- [ ] **Step 1: Scrivi i test che falliscono**

```rust
// crates/sim-core/tests/referendum.rs
mod comune;
use comune::{con_arrivo, imposta, presenta, senza_shock, sostituisci};
use sim_core::dati::{DatiGioco, FileDati};
use sim_core::parlamento::TipoReferendum;
use sim_core::{Comando, Errore, Partita};

fn con(f: FileDati, nome: &str, valore: &str) -> FileDati {
    imposta(f, "manopole", &format!("[{nome}]\nvalore = "), valore)
}

/// Le tutele crescenti all'estremo destro e il reintegro al centro: chi sta sotto 5,5 preferisce
/// l'abrogazione, cioè quasi tutto l'elettorato.
fn abrogazione_che_vince(astensione: &str) -> FileDati {
    let f = sostituisci(senza_shock(), "disposizioni", "nome = \"tutele crescenti\", posizione = 4.0", "nome = \"tutele crescenti\", posizione = 7.0");
    let f = sostituisci(f, "disposizioni", "nome = \"reintegro\", posizione = 2.0", "nome = \"reintegro\", posizione = 4.0");
    con(f, "astensione_referendum", astensione)
}

fn al_voto(f: FileDati) -> Partita {
    let mut p = Partita::nuova(DatiGioco::da_testi(&f).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "licenziamento").unwrap();
    // Turno 1 (gennaio-marzo 2027): firme in chiusura, voto ad aprile-giugno 2028 (turno 6).
    for _ in 0..6 {
        p.chiudi_turno();
    }
    p
}

#[test]
fn abrogazione_riporta_al_valore_dopo_l_abrogazione_e_blocca_il_gradino() {
    let mut p = al_voto(abrogazione_che_vince("-5.0"));
    let r = &p.vista().esiti_referendum[0];
    assert_eq!(r.tipo, TipoReferendum::Abrogativo);
    assert!(r.quorum && r.approvato);
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
    let e = presenta(&mut p, &[("licenziamento", "tutele_crescenti")]).unwrap_err();
    assert!(e.to_string().contains("gradino bloccato dal referendum: licenziamento tutele_crescenti"), "{e}");
}

#[test]
fn senza_quorum_paga_il_promotore() {
    let valence = |p: &Partita| p.partiti().iter().find(|x| x.id == "sinistra").unwrap().valence;
    let base = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let p = al_voto(abrogazione_che_vince("5.0"));
    assert!(!p.vista().esiti_referendum[0].quorum);
    assert_eq!(p.in_vigore("licenziamento"), Some("tutele_crescenti"));
    assert!(valence(&p) < valence(&base));
}

#[test]
fn invito_all_astensione_abbassa_l_affluenza() {
    let votanti = |invito: bool| {
        let mut p = Partita::nuova(DatiGioco::da_testi(&abrogazione_che_vince("0.0")).unwrap(), 1);
        p.chiudi_turno();
        p.promuovi_referendum("sinistra", "licenziamento").unwrap();
        for t in 0..6 {
            if invito && t == 5 {
                p.comanda(Comando::InvitaAstensione).unwrap();
            }
            p.chiudi_turno();
        }
        p.vista().esiti_referendum[0].votanti
    };
    assert!(votanti(true) < votanti(false));
}

#[test]
fn reddito_minimo_abrogato_diventa_non_istituito() {
    // Categoriale all'estremo destro: il «non istituito» (5,5) è più vicino a chi sta sotto 6,25.
    let f = sostituisci(senza_shock(), "disposizioni", "nome = \"categoriale\", posizione = 4.0", "nome = \"categoriale\", posizione = 7.0");
    let mut p = Partita::nuova(DatiGioco::da_testi(&con(f, "astensione_referendum", "-5.0")).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("estrema_destra", "reddito_minimo").unwrap();
    for _ in 0..6 {
        p.chiudi_turno();
    }
    assert!(p.vista().esiti_referendum[0].approvato);
    assert_eq!(p.in_vigore("reddito_minimo"), Some("non_istituito"));
}

#[test]
fn abrogazione_cancella_il_valore_in_arrivo() {
    let f = con_arrivo(abrogazione_che_vince("-5.0"), "licenziamento", "solo_indennizzo", 2029, 1);
    let mut p = al_voto(f);
    assert!(p.vista().esiti_referendum[0].approvato);
    // Dal turno 7 al 10: l'arrivo di gennaio-marzo 2029 (turno 9) non c'è più.
    for _ in 0..3 {
        p.chiudi_turno();
    }
    assert_eq!(p.in_vigore("licenziamento"), Some("reintegro"));
}

#[test]
fn arrivo_in_corso_rende_ammesso_il_quesito() {
    // Nucleare: vigente e precedente «vietato», ma «consentito» in arrivo a ottobre 2027.
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    p.chiudi_turno();
    p.promuovi_referendum("sinistra", "nucleare").unwrap();
}

#[test]
fn promozioni_non_ammesse_rifiutate_col_nome() {
    let mut p = Partita::nuova(DatiGioco::da_testi(&senza_shock()).unwrap(), 1);
    let prima = p.salva().unwrap();
    for ((promotore, disposizione), nome) in [
        (("destra", "licenziamento"), "il promotore è al governo: destra"),
        (("sinistra", "accise"), "accise non è referendabile"),
        (("sinistra", "eta_pensionabile"), "niente da abrogare: eta_pensionabile"),
        (("sinistra", "licenziamento"), "fuori finestra"),
    ] {
        let e = p.promuovi_referendum(promotore, disposizione).unwrap_err();
        assert!(matches!(e, Errore::Comando(_)) && e.to_string().contains(nome), "{e}");
    }
    assert_eq!(p.salva().unwrap(), prima);
    assert!(p.comanda(Comando::InvitaAstensione).is_err());
}
```

Il turno 0 è ottobre-dicembre, fuori finestra: per questo l'ultimo rifiuto arriva al turno 0 e `al_voto` promuove al turno 1, con voto ad aprile-giugno 2028 (turno 6, la sesta chiusura del ciclo). Il nucleare mostra il caso della spec (riga 344): vigente e precedente coincidono, ma l'arrivo in corso rende il quesito ammesso; nel gioco il voto cade dopo l'arrivo, quindi un sì colpirebbe `consentito`, già in vigore, e il gradino dopo l'abrogazione sarebbe `vietato`. Il valore in arrivo cancellato davvero dal voto lo fissa `abrogazione_cancella_il_valore_in_arrivo`, con un arrivo messo dopo il voto.

- [ ] **Step 2: Esegui i test e verifica che falliscano**

Run: `cargo test -p sim-core --test referendum`
Expected: errore di compilazione, `Partita::promuovi_referendum` non esiste.

- [ ] **Step 3: Implementa promozione, fasi, invito, voto, esito e blocco**

- [ ] **Step 4: Esegui tutta la suite, rigenera i tipi, verifica l'app**

Run: `cargo test --workspace && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check && npm --prefix app run check && npm --prefix app run build`
Expected: PASS; build di Vite completata coi tipi nuovi.

- [ ] **Step 5: Commit**

```bash
git add crates/sim-core dati/base/manopole.toml app/src/lib/tipi
git commit -m "feat(core): abrogative referendum with signatures, quorum and blocked step"
```
