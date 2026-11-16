# Piano 11d «I conti»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** il giocatore legge i conti dello Stato come una pagina economica e compone, presenta e difende in aula la legge di bilancio. La fetta porta due schermate (Conti e Bilancio), i quattro passacarte del bilancio e riempie subito, coi dati dello schema 19, i posti che la spec lasciava alla 11g.

**Architettura:**
- Ogni schermata è una cartella in `app/src/lib/<sezione>/`: un modulo `.ts` puro, che legge la Vista e decide cosa mostrare, e i componenti Svelte che lo disegnano. Il modulo puro è provato con Vitest sulle Viste campione.
- Il bilancio in composizione vive solo in memoria (spec:268). Ogni modifica passa dal core: `indicizza`, poi `riepilogo_bilancio` sul bilancio in memoria, con una richiesta in volo per volta (`ultimoVince`). Presentazione ed emendamenti passano da `partita.comanda`.
- Il guscio guadagna quattro passacarte verso metodi già pubblici del core: `bozza_bilancio`, `indicizza`, `riepilogo_bilancio`, `riepilogo_presentato`. `sim-core` non cambia.
- La Vista dello schema 19 porta già debito sul PIL, regole UE, MES, ristrutturazione, redditi per fascia, trimestri oltre la soglia, la soglia delle Aste scoperte, le cifre della legge in aula e i dicasteri di un emendamento: la fetta li usa (decisione 1).
- Il guscio finto impara a rispondere dai campioni 21 e 22, così ogni schermata si prova nel browser.

**Tecnologie:** quelle della 11c (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2, Rust). Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 118-130 (righe 164-178) e, delle storie della 11g, la 163 e la 164 (righe 220-221); i vincoli alle righe 248-371, la sezione della fetta alle righe 392-396, le conferme alla riga 453 e i test alle righe 455-476. La PR di questo piano corregge la spec dove le decisioni del 16/11 la cambiano (righe 354, 394, 395, 420). Chi esegue legge la spec e il ticket citato nel task.

**Riferimenti vincolanti per forma, ordine e testi:** i prototipi in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `21-conti.prototype.html`: forma A · Listino con le grandezze a lato; stati `ordinario`, `opposizione`, `accese`, `vicina`, `spread`, `senza`, `zero`, `crisi`, `grave`;
- `22-bilancio.prototype.html`: forma B · Bilancia; stati `bozza`, `presentato`, `decisi`, `ripresentato`, `provvisorio`, `opposizione` (barrata);
- `36-scelte-11d-conti.prototype.html`: i posti dello schema 19 nella colonna «L'economia» e nel dettaglio a lato (decisione 2); la fila «esempio inventato» ha valori finti, solo per la forma;
- `36-scelte-11d-bilancio.prototype.html` con `?marg=A`: il passo del margine (decisione 3).

I ticket che ogni task cita stanno in `.scratch/v1-piano-11-ui/issues/` (21 e 22). I testi dell'interfaccia si copiano dai prototipi. Un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR. Un numero che un prototipo segna «11g» ora viene dalla Vista o dai quattro passacarte.

**Da dove si parte:** la 11c fusa su main (le tre PR di `feat/11c-governo`, `feat/11c-aula-anteprima`, `feat/11c-cruscotto`, sopra la #68 e la #69). La 11c ha già aperto la sezione Bilancio (`app/src/lib/ossatura.ts:53`), la riga «Presenta la legge di bilancio» del cruscotto che rimanda a `/partita/bilancio` (`app/src/lib/cruscotto/cruscotto.ts:39-45`) e il formato delle cifre `euro` (`app/src/lib/governo/governo.ts:66-70`). Prima del Task 1 si rigenerano le Viste campione sul main con la 11c (spec:474).

## Vincoli globali

Valgono tutti i vincoli globali del piano 11c (`docs/superpowers/plans/2026-11-15-piano-11c-al-governo.md:29-43`), che richiamano quelli della 11a e della 11b. Qui quelli che la fetta tocca di più, e quelli nuovi.

- **La UI legge, il core decide** (spec:250). Lecito: confrontare due numeri della Vista (crescita e attese di una fascia, valore e soglia di una situazione), sottrarre la soglia dal valore per la frase della distanza, scrivere il funzionamento tagliato con le cifre della bozza e del bilancio in memoria (spec:395). Ogni costo, crescita della spesa netta, tetto e perdita reale viene dal core.
- **Soglie dalla Vista.** La linea delle Aste scoperte sul grafico dello spread è `Vista.soglie.crisi_debito_pb`; nessun 550 scritto nel codice della fetta. Le soglie delle situazioni sono `VoceSituazione.accensione` e `spegnimento`.
- **Il guscio aggiunge passacarte** verso metodi già pubblici della partita, senza logica né stato (spec:253). `sim-core` non cambia.
- **Conferme** (spec:453): «Presenta la legge di bilancio» è diretto, senza conferma; diretti anche Indicizza, i passi delle aliquote e del margine, Accogli · Respingi sugli emendamenti, «Modifica e ripresenta ›», «Compra» del centro studi.
- **Rifiuto del core accanto alla scelta**, col testo passato da `testoRifiuto`. Un bilancio non ammesso mostra il rifiuto al posto del giogo e spegne «Presenta» (storia 126).
- **Bozze solo in memoria** (spec:268): il bilancio in composizione resta finché il turno non cambia; Cmd+Q lo perde.
- **Tono grave**: le regole delle fondamenta; nei Conti e nel Bilancio non c'è niente che giri o oscilli.
- **Cargo da un worktree**: `--manifest-path <wt>/Cargo.toml`, `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi` e `CARGO_TARGET_DIR=<wt>/target`, tutti assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md:105`).

## Decisioni di Mario del 16/11/2026

Prese sui prototipi `36-scelte-11d-conti` e `36-scelte-11d-bilancio`, in una sola domanda.

1. **La 11d usa lo schema 19, tutti i posti.** Conti: debito sul PIL, regole UE (procedura, conto di controllo, prossimo controllo, coesione, margine per l'energia), MES, ristrutturazione, redditi per fascia, trimestri oltre la soglia, soglia delle Aste scoperte sul grafico dello spread. Bilancio: cifre della legge in aula (`VistaBilancio.in_aula`), costo «se accogli» di un emendamento (`VoceEmendamento.dicasteri`), stato del MES. MES, ristrutturazione, procedura aperta e margine per l'energia non stanno in nessuna Vista campione: il generatore li cerca giocando (Task 1); quelli che non trova si provano su una Vista copiata e ritoccata nel test, dichiarata come tale.
2. **Forma dei posti riempiti: come nel prototipo 36-conti.** Nella colonna «L'economia», sotto lo spread: «Debito» (debito sul PIL, poi le righe del MES e della ristrutturazione quando ci sono), «Regole UE» (procedura con da quanti turni, conto di controllo, prossimo controllo, coesione sospesa e margine per l'energia solo quando valgono), «Redditi reali per fascia» (reddito, crescita del trimestre, attese). Il dettaglio a lato al clic.
3. **Passo del margine: mezzo punto della spesa del dicastero**, con − e + per ciascuna delle due vie (dal funzionamento, in disavanzo); mai sotto il margine della bozza.

## Letture che la spec non fissa

Le ho decise scrivendo il piano. Mario le conferma col sì all'esecuzione, o le cambia prima.

1. **Il costo di «Indicizza»** è `costo_spesa` (e `costo_entrate` per le soglie) del riepilogo di `indicizza(bozza, voce)`: il costo sulla bozza, chiesto una volta all'apertura per le dieci voci, per tutto e per le soglie. Una voce già indicizzata nel bilancio in memoria dice «indicizzato» al posto del costo. Così la UI non sottrae due riepiloghi.
2. **«Dal funzionamento» toglie agli acquisti**, non ai salari (ticket 22, «acquisti −2,5 mld € sulla bozza»): tagliare i salari blocca il turnover (storia 42), un effetto diverso. Il passo è `Math.round(0.005 × spesa della bozza)`; il − toglie solo passi aggiunti.
3. **Aliquote a passi di un punto** (il prototipo 22, `title="−1 punto"`), su tutti gli scaglioni Irpef, Iva, Ires e contributi. La UI non mette limiti: un'aliquota fuori da 0-0,9 è il rifiuto del core al posto del giogo.
4. **«Modifica e ripresenta ›» riparte da `in_aula`**, cioè dal bilancio in aula con gli emendamenti accolti: sono le cifre che il giocatore ha davanti. Il core riapplica gli accolti alla nuova (`crates/sim-core/src/partita.rs:1959-1962`).
5. **Il costo «se accogli»** è il riepilogo di `in_aula` indicizzato, in sequenza, su ogni dicastero di `VoceEmendamento.dicasteri`. Il Task 7 controlla nel generatore che coincida col riepilogo del presentato dopo l'accoglimento.
6. **Sezione Bilancio barrata, due motivi**: all'opposizione «Non governi: il bilancio lo presenta il governo» (ticket 22); fuori stagione il testo del core (`FUORI_STAGIONE`, «fuori stagione: si presenta in ottobre-dicembre o nell'esercizio provvisorio»), copiato nella chiave: la Vista non porta la stringa e una chiamata per leggerla non serve.
7. **La crescita della spesa netta si chiama «stima»** (spec:396) e non porta un margine numerico: un ±0,4 scritto nella UI sarebbe un numero inventato.
8. **«Indicizzalo nella legge di bilancio ›»** dai Conti porta a `/partita/bilancio` con il dicastero scelto solo quando la sezione è aperta; altrimenti la frase dice quando si indicizza («nella legge di bilancio di ottobre-dicembre»).
9. **Deviazioni UE a parole, senza colore**: «0,27 punti sotto il percorso», «0,60 punti sopra il percorso». La procedura è rossa negli stati `aperta`, `inadempiente`, `intimata`, `sanzionata`.

## Punti da sorvegliare

I casi che la spec implica e che più facilmente rompono il gioco. Ognuno ha il suo test nel task indicato.

1. **Corsa dei riepiloghi.** Chi preme + tre volte in fretta fa partire tre `riepilogo_bilancio`: ne resta in volo uno per volta, l'ultimo vince, una risposta vecchia non copre la nuova; «Presenta» resta spento mentre un riepilogo è in volo. Task 8.
2. **Cifre negative.** `euro(-32882130)` oggi scrive «-32.882.130 €» (`governo.ts:66-70` guarda solo `c >= 1e6`): il costo sulla bozza va sotto zero quando la legge spende meno (opposizione nel 22: −32,9 mln). Serve il segno meno tipografico e le stesse unità del positivo. Task 2.
3. **Il secondo acquisto del centro studi** è un rifiuto sul posto col testo del core; due clic veloci fanno una chiamata sola. Task 4.
4. **Deflazione.** Una perdita reale negativa (nel 06, +1,0%) va in verde col più, nei Conti e nel Bilancio. Task 2 e Task 8.
5. **Esercizio provvisorio.** La bozza è dell'anno in corso, l'avviso è in testa, e la riga «se cade» dice le dimissioni, non un altro esercizio provvisorio. Task 9, sulla Vista `provvisorio` del 22 e su `senza_bilancio` del 21.

## Struttura dei file

```
app/src-tauri/src/lib.rs                    + bozza_bilancio, indicizza, riepilogo_bilancio, riepilogo_presentato
.scratch/v1-piano-11-ui/viste/genera/       + `prima` e quattro stati cercati nel 21; bilanci completi, passi del margine e «se accogli» nel 22
app/src/lib/guscio/chiamate.ts              + bozzaBilancio, indicizza, riepilogoBilancio, riepilogoPresentato
app/src/lib/guscio/finto.ts                 + risposte dai campioni 21 e 22
app/src/lib/acquisto.svelte.ts              acquistoSondaggio → acquisto(tipo)
app/src/lib/conti/                          conti.ts, Conti.svelte, ColonnaBilancio.svelte, ColonnaEconomia.svelte, Soglie.svelte, Righello.svelte, Spread.svelte, CentroStudi.svelte, DettaglioConti.svelte
app/src/lib/bilancio/                       bilancio.ts, bozza.svelte.ts, Bilancio.svelte, Giogo.svelte, Entrate.svelte, Spesa.svelte, DettaglioRiga.svelte, Presenta.svelte, Emendamenti.svelte
app/src/lib/governo/governo.ts              euro col segno
app/src/lib/ossatura.ts                     fettaDi dà null per Conti e Bilancio; due motivi per Bilancio barrata
app/src/lib/aula/Misura.svelte              all'opposizione, la legge di bilancio con le cifre in aula e il riepilogo del presentato
app/src/routes/partita/[sezione]/+page.svelte   monta Conti e Bilancio
docs/superpowers/verifiche/11d/             screenshot della fetta
```

I test Vitest stanno accanto al modulo che provano. Un file `.svelte` che supera le 250 righe si divide per riquadro.

## PR e ordine

- **PR 1, Conti** (`feat/11d-conti`): Task 1-5.
- **PR 2, Bilancio** (`feat/11d-bilancio`): Task 6-11.

La PR 2 parte da main dopo la fusione della PR 1. Dentro una PR i task vanno in sequenza, con queste eccezioni: 1 e 2 in parallelo (generatore e moduli puri, cartelle diverse); 6 e 7 in parallelo (guscio Rust e generatore, cartelle diverse).

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo; `rust-reviewer` in tutte e due (generatore nella PR 1, guscio nella PR 2). `craft-reviewer` riceve gli screenshot del task di verifica della PR e li confronta col prototipo e coi token.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta.

- **Dove ricerca e piano dicono cose diverse, vale il piano**: i prototipi e le decisioni del 16/11 sono venuti dopo le ricerche.
- **In un parallelo ogni subagente mette nel commit solo i file del proprio task**, nominandoli uno per uno in `git add`. Un rosso che nasce nella cartella dell'altro task si riporta a chi orchestra e resta com'è.

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `rust-engineer` | primo della PR 1, in parallelo col 2 | `rust-skills` |
| 2 | `frontend-engineer` | in parallelo con l'1 | salto dichiarato: moduli puri e guscio finto, nessuna interfaccia |
| 3 | `frontend-engineer` | dopo l'1 e il 2 | `impeccable`, `dataviz`, `apple-design` |
| 4 | `frontend-engineer` | dopo il 3 | `impeccable`, `dataviz`, `apple-design` |
| 5 | sessione principale | dopo il 4 | `scrittura-italiana` |
| 6 | `rust-engineer` | primo della PR 2, in parallelo col 7 | `rust-skills` |
| 7 | `rust-engineer` | in parallelo col 6 | `rust-skills` |
| 8 | `frontend-engineer` | dopo il 6 e il 7 | `impeccable`, `dataviz`, `apple-design` |
| 9 | `frontend-engineer` | dopo l'8 | `impeccable`, `dataviz`, `apple-design` |
| 10 | `frontend-engineer` | dopo il 9 | `impeccable`, `dataviz`, `apple-design` |
| 11 | sessione principale | dopo il 10 | `scrittura-italiana` |

«Almeno una fra» le skill elencate. Ogni brief nomina anche `skill-gate` e `caveman`; quelli dei subagenti che scrivono codice (tutti, fuori dalla sessione principale) anche `ponytail`.

## Dove va ogni storia

| Storia | Task |
|---|---|
| 118, 119, 120 | 3 |
| 121, 122, 123 | 4 |
| 163 (Conti) | 2, 3, 4 |
| 124, 125, 126 | 8 |
| 127, 128, 130 | 9 |
| 129, 164 | 10 |

---

### Task 1: generatore, stati dei Conti che mancano

**Subagente:** `rust-engineer`: il generatore è Rust.

**Sequenza:** primo della PR 1, in parallelo col Task 2. Il Task 3 aspetta il suo commit.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/modelli/regole-fiscali-ue.md:33`: la procedura si apre col disavanzo sopra il 3%: la strada per arrivarci giocando è un bilancio in disavanzo.
- `docs/ricerca/modelli/regole-fiscali-ue.md:20`: il conto di controllo somma le deviazioni dell'anno.

**Files:**
- Modify: `.scratch/v1-piano-11-ui/viste/genera/src/main.rs` (`conti`, righe 999-1050); `.scratch/v1-piano-11-ui/viste/README.md` (riga del 21 e nota «21» in «Stranezze notate»)
- Rigenerato: `.scratch/v1-piano-11-ui/viste/21-conti.json`

**Interfaces:**
- Consumes: `cerca_al_governo`, `fonda`, `governo_03`, `comanda` del generatore; `Partita::bozza_bilancio`, `Comando::PresentaBilancio`, `Comando::RispondiEvento`.
- Produces, in `21-conti.json`:
  - ogni stato guadagna `prima`: la Vista prima di `Compra { CentroStudi }` (il finto la serve all'apertura, Task 2);
  - `pde`: al governo con `regole_ue.procedura` in `aperta`, `inadempiente`, `intimata` o `sanzionata`;
  - `energia`: al governo con `regole_ue.margine_energia` non nullo, raggiunto rispondendo «clausola» a «Caro energia» dopo il 2028;
  - `mes` e `ristrutturazione`: al governo dopo «Aste scoperte», rispondendo con l'opzione `mes` o `ristrutturazione`; nello stato `mes` anche `rifiuto_presenta`, il rifiuto (o l'esito) di `PresentaBilancio` con la bozza indicizzata per tutto, se il turno è di bilancio.
  Ogni stato con `come`, `cassa_prima`, `cassa_dopo`, `rifiuto`, `rifiuto_ripetuto`, `serie`, `prima` e `vista`, come gli altri.

- [ ] **Step 1: aggiungi `prima`** a `compra` (la Vista presa prima del primo `Compra`).
- [ ] **Step 2: aggiungi la strategia in disavanzo.** Una variante di `cerca_al_governo` che a ogni bilancio presenta la bozza con tutte le aliquote abbassate di cinque punti e risponde a ogni evento con l'opzione indicata (o il default), semi 1-60, al più 40 turni. Le quattro cerche dell'interfaccia la usano; la nota del README dice la strategia per ciascuno stato.
- [ ] **Step 3: uno stato che non si raggiunge** stampa `NON RAGGIUNTO: <stato>` su stderr e manca dal file: il Task 2 lo prova con una Vista ritoccata e dichiarata. Non si forza nessun salvataggio.
- [ ] **Step 4: esegui e rigenera.** Run, uno per chiamata:
  - `CARGO_TARGET_DIR=<wt>/target cargo run --release --manifest-path <wt>/.scratch/v1-piano-11-ui/viste/genera/Cargo.toml`, due volte, con `shasum` dei file dopo ciascuna
  - `cargo clippy --manifest-path <wt>/.scratch/v1-piano-11-ui/viste/genera/Cargo.toml --all-targets -- -D warnings`
  - `git -C <wt> status --short`

  Expected: impronte uguali nei due giri; cambiano solo `21-conti.json`, il README e il generatore; clippy pulito. Nel report: quali dei quattro stati sono raggiunti, con seme e turno.
- [ ] **Step 5: commit.** `chore(viste): stati dei Conti con procedura, energia, MES e ristrutturazione`

---

### Task 2: moduli puri dei Conti, acquisto e guscio finto

**Subagente:** `frontend-engineer`.

**Sequenza:** in parallelo col Task 1: lavora in `app/src/lib/conti/`, `acquisto.svelte.ts`, `governo/governo.ts` e `guscio/finto.ts`. Le prove sugli stati nuovi del 21 si aggiungono quando il commit del Task 1 c'è; fino ad allora girano sulle Viste ritoccate.

**Skill-gate:** nessuna skill del ruolo ha il trigger su moduli puri e guscio finto senza interfaccia: il salto si dichiara nel report. `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:58`: il passaggio del bilancio nella Vista di allora; oggi lo schema 19 porta anche ciò che la riga dava per mancante.

**Files:**
- Create: `app/src/lib/conti/conti.ts`, `conti.test.ts`
- Modify: `app/src/lib/acquisto.svelte.ts`, `acquisto.svelte.test.ts` e i suoi chiamanti in Paese e Campagna; `app/src/lib/governo/governo.ts`, `governo.test.ts`; `app/src/lib/guscio/finto.ts`, `finto.test.ts`

**Interfaces:**
- Consumes: `Vista`, `VoceSituazione`, `VoceGrandezza`, `VistaRegoleUe`, `StatoPde`, `VistaDebito`, `RedditoFascia`, `VistaCentroStudi` (`app/src/lib/tipi/`); `numero`, `segnato` (`app/src/lib/testi/t.ts:19-37`).
- Produces:

```ts
// governo.ts: sostituisce euro; il negativo col segno meno tipografico e le stesse unità del positivo
export function euro(c: number): string;          // euro(-32882130) === "−33 mln €", come euro(32882130) === "33 mln €"
// acquisto.svelte.ts: sostituisce acquistoSondaggio
export function acquisto(tipo: "sondaggio_privato" | "centro_studi"): { readonly rifiuto: string | null; compra(): Promise<void> };
// conti.ts
/** La frase della distanza (ticket 21; decisione 1 per `oltre`). */
export function distanza(s: VoceSituazione): string;
/** La procedura in parole e se è rossa (lettura 9). */
export function procedura(u: VistaRegoleUe): { testo: string; male: boolean };
/** «0,27 punti sotto il percorso», «sul percorso», «0,60 punti sopra il percorso» (lettura 9). */
export function percorso(deviazione: number): string;
/** Le grandezze nazionali col valore; le regionali come «da–a» col rimando a Paese (ticket 21). */
export function grandezza(g: VoceGrandezza): { valore: number } | { da: number; a: number };
/** La perdita reale scritta: «−0,1%» rossa, «+1,0%» verde se negativa (deflazione), «0,0%» sotto 0,05. */
export function reale(p: number): { testo: string; tono: "male" | "bene" | "neutro" };
/** Sotto le attese: la crescita del trimestre minore delle attese. */
export const sottoAttese = (r: RedditoFascia): boolean => r.crescita < r.attesa;
```

La frase della distanza, con le cifre della spec e del prototipo 36:
- accesa: «accesa · si spegne sotto 5: mancano 7,5»;
- spenta con `oltre > 0`: «oltre la soglia da un trimestre: si accende al secondo di fila» (con `oltre` 2 «da due trimestri», con `trimestri` 3 «al terzo»);
- spenta oltre la soglia senza `oltre`: «oltre la soglia»;
- altrimenti: «si accende sotto 85: mancano 14,7».

**Guscio finto.** `installaFinto` carica sempre anche `21-`. Con `?vista=21-conti.json&stato=<s>` il finto serve `<s>.prima`. `rispondi` impara:

| Comando | Risposta |
|---|---|
| `comanda` con `compra` `centro_studi` | `<s>.vista`; se la Vista servita ha già `acquisti.centro_studi`, il rifiuto `<s>.rifiuto_ripetuto` |
| `serie` | `<s>.serie` |

Gli stati che il Task 1 non raggiunge si provano in `conti.test.ts` su una copia di `studi_03` ritoccata nel test (`procedura: "inadempiente"`, `mes` con due vincoli, `ristrutturazione`, `margine_energia`), con un commento che lo dichiara.

- [ ] **Step 1: scrivi i test che falliscono.**
  - `governo.test.ts`: `euro(-32882130)` uguale a `"−" + euro(32882130)`; `euro(-6197708677)` è «−6,2 mld €»; `euro(0)` è «0 €»;
  - `conti.test.ts`, `distanza`: su `21-conti.json["vicina"].vista` la Recessione (−4,2, `oltre` 1, `trimestri` 2) dà «oltre la soglia da un trimestre: si accende al secondo di fila»; su `studi_03` il Caro energia (12,5, accesa) dà una frase che comincia con «accesa · si spegne sotto»; la Criminalità del 03 (99,7) dà «si accende sotto 85: mancano 14,7»;
  - `procedura`: `sospesa` del 03 non è `male`; `inadempiente` sì; `percorso(-0.27)` è «0,27 punti sotto il percorso», `percorso(0)` «sul percorso»;
  - `reale(1.0)` è `{ testo: "−1,0%", tono: "male" }`, `reale(-1.0)` `{ testo: "+1,0%", tono: "bene" }` (punto da sorvegliare 4);
  - `grandezza`: le tre regionali del 03 danno `da`/`a`, le altre `valore`;
  - `acquisto.svelte.test.ts`: `acquisto("centro_studi").compra()` manda `{ tipo: "compra", acquisto: "centro_studi" }`; due `compra()` di fila mandano un comando solo (punto da sorvegliare 3); i test del sondaggio privato restano verdi;
  - `finto.test.ts`: `?vista=21-conti.json&stato=studi_03` serve una Vista senza `centro_studi`; `compra` la dà con `centro_studi`; un secondo `compra` lancia il rifiuto «già comprato questo turno: centro studi».
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.** `acquisto(tipo)` prende il posto di `acquistoSondaggio` in tutti i chiamanti.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): moduli dei Conti, acquisto del centro studi e finto del 21`

---

### Task 3: Conti, le tre colonne

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 1 e il Task 2. È il primo della fetta a modificare `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:46`: superficie leggibile in un'occhiata, il dettaglio a lato: è la forma A con le grandezze a lato.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:49`: cifre e tipografia da giornale come materiale primario.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:90`: i dettagli non salvano una superficie affollata: le 21 grandezze vanno nel lato.

**Files:**
- Create: `app/src/lib/conti/Conti.svelte`, `ColonnaBilancio.svelte`, `ColonnaEconomia.svelte`, `Soglie.svelte`, `Righello.svelte`, `Spread.svelte`
- Modify: `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 2 (`distanza`, `procedura`, `percorso`, `reale`, `sottoAttese`, `euro`); `partita.vista`, `partita.serie`; `Dettaglio`.
- Produces: `Conti.svelte` con `scelta: string | null` (la riga scelta, chiavi `dic:<id>`, `irpef:<i>`, `fondo`, `stat:<campo>`, `spread`, `sit:<i>`, `grandezze`, `debito`, `ue`, `mes`, `ristr`, `red:<fascia>`) che il Task 4 legge per il dettaglio a lato.

- `fettaDi("conti", …)` dà `null` in ogni stato: la pagina monta `Conti`.
- **Il bilancio** (storia 118, `21-conti.prototype.html` forma A): titolo «Il bilancio <anno>» con «in vigore» o «esercizio provvisorio»; i dieci dicasteri con spesa e perdita reale con la barretta; l'Irpef con aliquota e soglia reale per scaglione (l'ultimo senza soglia, «—»); Iva, Ires e contributi in una riga; il fondo di riserva rimasto sul totale con la barra.
- **L'economia** (storie 118, 119): la prima stima in quattro numeri grandi (PIL annualizzato, disoccupazione, `inflazione_annua`, disavanzo), col centro studi sotto ciascuno dopo l'acquisto; la revisione del trimestre prima, prima stima → riveduta con lo scarto, o «Nessuna revisione: è il primo trimestre»; lo spread col grafico di `partita.serie` e la linea a `soglie.crisi_debito_pb`; poi «Debito», «Regole UE», «Redditi reali per fascia» nella forma del prototipo 36 (decisione 2). Con la serie di un punto solo il grafico dice «La serie dello spread parte da questo turno.»
- **Le soglie** (storia 120): le sei situazioni col righello (accensione rossa, spegnimento tratteggiato, isteresi a retino, valore come pallino), il bollo «accesa», la frase di `distanza`; in fondo la riga «Le 21 grandezze ›».
- All'opposizione la pagina è la stessa. Al turno zero lo spread ha un punto solo e la revisione manca.

- [ ] **Step 1: scrivi i test che falliscono**, in `ossatura.test.ts`: `fettaDi("conti", s)` è `null` per tutti e sei gli stati.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `21-conti.prototype.html` e a `36-scelte-11d-conti.prototype.html`, su `/partita/conti?vista=21-conti.json&stato=` `studi_03`, `studi_06`, `due_accese`, `vicina`, `spread_alto`, `senza_bilancio` e gli stati del Task 1 raggiunti, poi `?vista=01-turno-zero.json`, `05-crisi.json`, `08-tono-grave.json`: le tre colonne a 1280×800; la frase della recessione in `vicina`; la deflazione verde in `studi_06`; la linea delle Aste scoperte. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Conti, bilancio, economia e soglie`

---

### Task 4: Conti, centro studi e dettaglio a lato

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 3: stessi `Conti.svelte` e cartella.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/modelli/informazione-a-pagamento.md:61`: il centro studi e il suo prezzo: si compra dove si legge il risultato.
- `docs/ricerca/modelli/informazione-a-pagamento.md:123`: la stima del centro studi toglie una parte dell'errore, non tutto: la pagina la mette accanto alla prima stima, mai al suo posto.

**Files:**
- Create: `app/src/lib/conti/CentroStudi.svelte`, `DettaglioConti.svelte`
- Modify: `app/src/lib/conti/Conti.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 2 (`acquisto`, `grandezza`, `percorso`, `procedura`); Task 3 (`scelta`); `apribile`, `barrata` (`app/src/lib/ossatura.ts`); `Rifiuto`.

- **Centro studi** (storia 122), nel lato finché nessuna riga è scelta: prima dell'acquisto cosa vende, «Compra · 150.000 €» (prezzo da `acquisti.prezzi`) e la cassa del partito; dopo, la stima del trimestre accanto alla prima stima e quattro ventagli a quattro trimestri (PIL, disoccupazione, inflazione, debito) con la fascia di una e di due sigma. Il secondo «Compra» mostra il rifiuto sul posto.
- **Dettaglio della riga**, con «‹ Torna al centro studi»: dicastero (spesa, salari, acquisti, margine, la frase «Sanità: −0,1% reale dal bilancio 2027» e «Indicizzalo nella legge di bilancio ›» secondo la lettura 8, storia 123); scaglione (quanto la soglia è erosa); situazione (righello grande, soglie, distanza, cosa fa accesa); grandezza; numero della stima (prima stima, centro studi, revisione); spread (la serie grande); fondo (gli eventi aperti col costo); «Le 21 grandezze» (l'elenco per gruppi, storia 121); debito (col ventaglio del debito se comprato); regole UE per esteso; MES coi vincoli; ristrutturazione; fascia di reddito. I testi del dettaglio dei posti nuovi si copiano dal prototipo 36.

- [ ] **Step 1: verifica nel browser**, accanto ai prototipi 21 e 36, su `?vista=21-conti.json&stato=studi_03`: «Compra» mostra stima e ventagli; il secondo «Compra» il rifiuto; due clic veloci una chiamata sola (punto da sorvegliare 3); ogni riga apre il suo dettaglio e «‹ Torna al centro studi» lo chiude; «Indicizzalo nella legge di bilancio ›» è un link sul 22 `bozza_04` e una frase sul 03; tutto raggiungibile da tastiera.
- [ ] **Step 2: test e tipi.** Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 3: commit.** `feat(app): centro studi e dettaglio dei Conti`

---

### Task 5: verifica della PR 1

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 4. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** (`npm --prefix <wt>/app run verifica:chrome` e `verifica:webkit`, `app/verifica/README.md`): `/partita/conti` su tutti gli stati del Task 3, Step 4, alle due finestre e alle due scale. Expected: uscita 0, tagli 0.
- [ ] **Step 2: tastiera, a mano**: le righe delle tre colonne, «Compra», il dettaglio e il ritorno.
- [ ] **Step 3: test e tipi.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`, `npm --prefix <wt>/app run build`. Expected: verdi, pulito.
- [ ] **Step 4: screenshot** in `docs/superpowers/verifiche/11d/`: Conti nei suoi stati alle due finestre; il centro studi comprato; un dettaglio per tipo di posto nuovo; il tono grave.
- [ ] **Step 5: corpo della PR**: cosa cambia, decisioni prese in esecuzione, «Per Mario», testi nuovi, verifiche, stati del Task 1 non raggiunti e come sono provati.

**Fine della PR 1.**

---

### Task 6: passacarte del bilancio

**Subagente:** `rust-engineer`: il task tocca `.rs`.

**Sequenza:** primo della PR 2, in parallelo col Task 7.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:58`: i tre metodi del bilancio e i comandi; i numeri di riga di allora sono cambiati, valgono quelli sotto.

**Files:**
- Modify: `app/src-tauri/src/lib.rs` (comandi e `generate_handler!`, riga 574)

**Interfaces:**
- Consumes: `Partita::bozza_bilancio(&self) -> Option<Bilancio>` (`crates/sim-core/src/partita.rs:1914`); `Partita::indicizza(&self, b: &Bilancio, voce: &Voce) -> Result<Bilancio, Errore>` (`:1922`); `Partita::riepilogo_bilancio(&self, b: &Bilancio) -> Result<RiepilogoBilancio, Errore>` (`:1932`); `Partita::riepilogo_presentato(&self) -> Option<RiepilogoBilancio>` (`:1939`); `sulla_partita` (`app/src-tauri/src/lib.rs:434-441`).
- Produces, i comandi, involucri di una riga su `sulla_partita` come `anteprima_accogli` (`lib.rs:503-509`):
  - `bozza_bilancio() -> Result<Option<Bilancio>, ErroreGuscio>`
  - `indicizza(bilancio: Bilancio, voce: Voce) -> Result<Bilancio, ErroreGuscio>`
  - `riepilogo_bilancio(bilancio: Bilancio) -> Result<RiepilogoBilancio, ErroreGuscio>`
  - `riepilogo_presentato() -> Result<Option<RiepilogoBilancio>, ErroreGuscio>`

- [ ] **Step 1: scrivi i test che falliscono**, in `lib.rs` `mod tests`, sulle funzioni interne come i test di `anteprima_accogli`:
  - i quattro senza partita: `Err(guasto(NESSUNA))`;
  - `la_bozza_e_quella_del_core` e `il_presentato_e_quello_del_core`: uguali al metodo del core sulla stessa partita;
  - `indicizza_tutto_e_quello_del_core`: con la bozza e `Voce::Tutto`, uguale a `Partita::indicizza`;
  - `riepilogo_fuori_stagione_e_un_rifiuto`: al turno zero, `Specie::Rifiuto` col testo del core.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml -p app`. Expected: errore di compilazione sui comandi che mancano.
- [ ] **Step 3: implementa** i quattro comandi e registrali.
- [ ] **Step 4: verifica.** Run, uno per chiamata:
  - `CARGO_TARGET_DIR=<wt>/target TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`
  - `cargo clippy --manifest-path <wt>/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
  - `git -C <wt> status --short`

  Expected: verdi, clippy pulito, nessun file in `app/src/lib/tipi/` cambiato.
- [ ] **Step 5: commit.** `feat(app): passacarte della legge di bilancio`

---

### Task 7: generatore, il Bilancio che manca

**Subagente:** `rust-engineer`.

**Sequenza:** in parallelo col Task 6: tocca solo il generatore e le Viste.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `.scratch/v1-piano-11-ui/viste/genera/src/main.rs` (`bilancio`, righe 1055-1179); `.scratch/v1-piano-11-ui/viste/README.md`
- Rigenerato: `.scratch/v1-piano-11-ui/viste/22-bilancio.json`

**Interfaces:**
- Produces, in `22-bilancio.json["bozza_04"]["varianti"]`:
  - `per_dicastero[i].bilancio`: il bilancio intero dopo `indicizza(bozza, Dicastero)`, accanto a `dicastero` che resta;
  - `margine_passo`: per la Sanità, `{ "funzionamento": { bilancio, riepilogo }, "disavanzo": { bilancio, riepilogo } }` con un passo di mezzo punto (`round(0,005 × spesa)`) dagli acquisti e in disavanzo (decisione 3, lettura 2);
  - `aliquote_passo`: `{ "<imposta>": { "meno": { bilancio, riepilogo }, "piu": … } }` per i tre scaglioni Irpef, Iva, Ires e contributi, un punto (lettura 3);
- in `22-bilancio.json["presentato"]`: `se_accogli_ui`, per alleato, il riepilogo di `in_aula` indicizzato in sequenza sui `dicasteri` del suo emendamento (lettura 5). Il generatore stampa `SCARTO <alleato>` se `costo_spesa` o `crescita_spesa_netta` differiscono da `se_accogli` di più di 1e-9.

- [ ] **Step 1: aggiungi le tre varianti e `se_accogli_ui`**, col controllo dello scarto.
- [ ] **Step 2: esegui e rigenera**, due giri con `shasum`, clippy del generatore, `git status`, come al Task 1, Step 4. Expected: nessuno `SCARTO`; impronte uguali; cambiano solo `22-bilancio.json`, il README e il generatore. Uno `SCARTO` ferma il task e si riporta a chi orchestra: la lettura 5 va rivista con Mario.
- [ ] **Step 3: commit.** `chore(viste): bilanci completi, passi e costo degli emendamenti nel 22`

---

### Task 8: Bilancio, bozza in memoria, giogo, entrate e spesa

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 6 e il Task 7. Modifica `ossatura.ts`, `ossatura.test.ts` e `+page.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/modelli/regole-fiscali-ue.md:8`: il solo vincolo è il tetto alla crescita della spesa netta: per questo il giogo sta in testa.
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:47`: ciò che costa ha peso visivo proporzionale, grande e centrale.

**Files:**
- Create: `app/src/lib/bilancio/bilancio.ts`, `bilancio.test.ts`, `bozza.svelte.ts`, `bozza.svelte.test.ts`, `Bilancio.svelte`, `Giogo.svelte`, `Entrate.svelte`, `Spesa.svelte`
- Modify: `app/src/lib/guscio/chiamate.ts`, `chiamate.test.ts`, `finto.ts`, `finto.test.ts`; `app/src/lib/ossatura.ts`, `ossatura.test.ts`; `app/src/routes/partita/[sezione]/+page.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: i comandi del Task 6; le varianti del Task 7; `ultimoVince` (`app/src/lib/editor/editor.svelte.ts:54`); `euro`, `reale` (Task 2); `Bilancio`, `Voce`, `RiepilogoBilancio`, `Aliquote`.
- Produces:

```ts
// chiamate.ts
guscio.bozzaBilancio(): Promise<Bilancio | null>
guscio.indicizza(bilancio: Bilancio, voce: Voce): Promise<Bilancio>
guscio.riepilogoBilancio(bilancio: Bilancio): Promise<RiepilogoBilancio>
guscio.riepilogoPresentato(): Promise<RiepilogoBilancio | null>
// bilancio.ts
export type Imposta = { tipo: "irpef"; indice: number } | { tipo: "iva" | "ires" | "contributi" };
/** Un punto in più o in meno sull'aliquota (lettura 3); non limita: i limiti li dice il core. */
export function ritocca(b: Bilancio, imposta: Imposta, verso: 1 | -1): Bilancio;
/** Il passo del margine: mezzo punto della spesa della bozza (decisione 3). */
export const passoMargine = (bozza: BudgetDicastero): number => Math.round(bozza.spesa * 0.005);
/** Applica n passi dalla via data, partendo dalla bozza: dagli acquisti (lettura 2) o in disavanzo (sulla spesa). */
export function allarga(b: Bilancio, bozza: Bilancio, dicastero: string, via: "funzionamento" | "disavanzo", passi: number): Bilancio;
/** Il funzionamento tagliato, sotto la barra: salari e acquisti del bilancio in memoria contro la bozza (spec:395); null se uguali. */
export function tagliato(b: BudgetDicastero, bozza: BudgetDicastero): { salari: number; acquisti: number } | null;
/** «dentro il tetto: restano 1,0 punti» o «oltre il tetto di 0,4 punti: il controllo di aprile-giugno lo rileva». */
export function distanzaTetto(r: RiepilogoBilancio): string;
// bozza.svelte.ts: il bilancio in composizione, in memoria
export const bozza: {
  readonly base: Bilancio | null;      // da bozzaBilancio(), una volta per turno
  readonly corrente: Bilancio | null;  // il bilancio in memoria
  readonly riepilogo: RiepilogoBilancio | null;
  readonly rifiuto: string | null;     // al posto del giogo (storia 126)
  readonly inVolo: boolean;
  readonly costi: Record<string, RiepilogoBilancio | string>;  // «tutto», «scaglioni», id del dicastero (lettura 1)
  apri(turno: number): Promise<void>;  // nuovo turno = bozza nuova
  cambia(b: Bilancio): void;           // ultimoVince su riepilogoBilancio
  indicizza(voce: Voce): Promise<void>;
};
```

- `fettaDi("bilancio", …)` dà `null`; `barrata("bilancio", v)` dà `barrata.bilancio_opposizione` se `!v.governa`, `barrata.bilancio_stagione` fuori stagione (lettura 6); le due chiavi in `interfaccia.json`.
- **Guscio finto.** `installaFinto` carica sempre anche `22-`. `bozza_bilancio`: `bozza_04.bozza` o `provvisorio.bozza`; `indicizza`: il `bilancio` della variante con la stessa voce; `riepilogo_bilancio`: il riepilogo della variante col bilancio uguale, `fuori_stagione.rifiuto` o `rifiuto_riepilogo` all'opposizione, altrimenti «non nel campione»; `riepilogo_presentato`: `riepilogo_presentato` dello stato; `comanda` con `presenta_bilancio` della bozza: `presentato.vista`; con `emendamento`: `emendamenti_decisi.vista` se è una delle due mosse giocate.
- **Il giogo** (storia 124): a tutta pagina, la crescita della spesa netta («stima», lettura 7) contro il tetto UE con la riga rossa e `distanzaTetto`; accanto, la spesa in più della bozza e il gettito perso. Con `rifiuto` il testo del core al posto del giogo.
- **Entrate e spesa a fronte** (storia 125): a sinistra le aliquote grandi con − e + (`ritocca`), le soglie reali degli scaglioni e «indicizza le soglie» col costo; Iva, Ires, contributi, il fondo di riserva. A destra i dieci dicasteri come barre (salari, acquisti, margine, resto = spesa − gli altri tre), la spesa, la perdita reale del riepilogo, «Indicizza» col costo (lettura 1); in testa «Indicizza tutto» col costo.

- [ ] **Step 1: scrivi i test che falliscono.**
  - `chiamate.test.ts`: quattro nomi di comando in più; `indicizza(b, { tipo: "tutto" })` chiama `indicizza` con `{ bilancio: b, voce: { tipo: "tutto" } }`.
  - `bilancio.test.ts`, sulla `bozza_04`: `passoMargine` della Sanità è 776.917.230 (0,5% di 155.383.445.952, arrotondato); `allarga(…, "sanita", "funzionamento", 1)` uguale a `varianti.margine_passo.funzionamento.bilancio`, e così `"disavanzo"`; `allarga(…, 0)` è la bozza; `ritocca` con `{ tipo: "iva" }` e `+1` porta l'Iva a 0,23 e uguaglia `aliquote_passo.iva.piu.bilancio`; `tagliato` della Sanità dopo un passo dal funzionamento dà acquisti −776.917.230 e salari 0; `distanzaTetto` del `riepilogo_bozza` è «dentro il tetto: restano 1,0 punti».
  - `bozza.svelte.test.ts`, con un guscio finto a promesse comandate: tre `cambia` di fila fanno partire due richieste (la prima e l'ultima), e il riepilogo finale è quello dell'ultima (punto da sorvegliare 1); `inVolo` è vero finché l'ultima non torna; un rifiuto va in `rifiuto` e svuota `riepilogo`; `apri` con un turno nuovo rifà `base` e svuota `corrente`.
  - `ossatura.test.ts`: `barrata("bilancio")` sul 06 è `barrata.bilancio_opposizione`, sul 03 `barrata.bilancio_stagione`, sul `bozza_04` `null`; `fettaDi("bilancio", "governo")` è `null`.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto a `22-bilancio.prototype.html`, su `/partita/bilancio?vista=22-bilancio.json&stato=bozza_04`: il giogo, i costi di «Indicizza» e «Indicizza tutto»; «Indicizza tutto» porta il giogo a +1,4% contro 1,7%; Iva + porta al riepilogo della variante; un secondo ritocco diverso dà «non nel campione» al posto del giogo, e «Presenta» si spegne. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): Bilancio, bozza in memoria, giogo, entrate e spesa`

---

### Task 9: Bilancio, dettaglio della riga, presentazione, esercizio provvisorio e MES

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 8: stessa cartella e `Bilancio.svelte`.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48`: la conseguenza prima della scelta: sotto «Presenta» cosa succede se cade.
- `docs/ricerca/sintesi/vista-per-schermata.md:167`: il bilancio ricondotto al tetto del MES senza avviso: nel riquadro del MES lo si dice.

**Files:**
- Create: `app/src/lib/bilancio/DettaglioRiga.svelte`, `Presenta.svelte`
- Modify: `app/src/lib/bilancio/Bilancio.svelte`, `bilancio.ts`, `bilancio.test.ts`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 8 (`bozza`, `allarga`, `passoMargine`, `tagliato`); `partita.comanda`, `testoRifiuto`, `Rifiuto`.
- Produces:

```ts
/** La riga «se cade»: esercizio provvisorio da gennaio, o le dimissioni se si è già in esercizio provvisorio (storia 128). */
export function seCade(v: Vista): string;
```

- **Dettaglio della riga** (storia 127), nel lato: dicastero (ministro, spesa, funzionamento, margine, resto, perdita reale, la frase della storia 41, «Indicizza», poi «Allarga il margine» con le due vie a passi, come `36-scelte-11d-bilancio.prototype.html?marg=A`, e sotto la barra la riga del funzionamento tagliato «acquisti −776,9 mln € sulla bozza · la qualità del servizio sanitario peggiora con ritardo», col nome dell'indicatore dalla Vista); scaglione (aliquota, soglia, soglia reale); fondo (la cifra, gli eventi aperti col costo).
- **Senza riga scelta** il lato tiene la presentazione: «Presenta la legge di bilancio ›» pieno e diretto, `{ tipo: "presenta_bilancio", bilancio: bozza.corrente ?? bozza.base }`, spento con il motivo mentre `bozza.inVolo`, con `bozza.rifiuto` o `partita.occupato`; sotto `seCade`. Il rifiuto del core sotto il bottone.
- **Esercizio provvisorio** (storia 130): l'avviso in testa «valgono le cifre del <anno>, nessuna misura con costo»; la bozza è dell'anno in corso (`base.anno`).
- **MES e piano strutturale**: con `v.debito.mes` il riquadro del MES dice fino a quando, i vincoli, e che un bilancio presentato prima entra ricondotto al tetto; senza MES non c'è. Il piano strutturale riveduto resta un riquadro a tratteggio col rimando al piano di core strutturale (spec:361).

- [ ] **Step 1: scrivi i test che falliscono**, in `bilancio.test.ts`: `seCade` sulla `bozza_04` parla di esercizio provvisorio da gennaio; su `provvisorio` parla di dimissioni (punto da sorvegliare 5).
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica nel browser**, accanto ai prototipi 22 e 36-bilancio: su `bozza_04` la Sanità, un passo dal funzionamento (giogo uguale alla bozza, riga del funzionamento tagliato), un passo in disavanzo (il riepilogo della variante); «Presenta» porta alla Vista `presentato`; due Invio veloci una chiamata sola. Su `provvisorio`: l'avviso, la bozza 2030, «se cade» con le dimissioni. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): dettaglio della riga, presentazione ed esercizio provvisorio del Bilancio`

---

### Task 10: Bilancio presentato, emendamenti e Aula all'opposizione

**Subagente:** `frontend-engineer`.

**Sequenza:** dopo il Task 9.

**Skill-gate:** almeno una fra `impeccable`, `dataviz` e `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:58`: il bilancio presentato è una `VoceMisura` di forma `Bilancio` con gli emendamenti degli alleati.

**Files:**
- Create: `app/src/lib/bilancio/Emendamenti.svelte`
- Modify: `app/src/lib/bilancio/Bilancio.svelte`, `Spesa.svelte`, `bilancio.ts`, `bilancio.test.ts`, `bozza.svelte.ts`; `app/src/lib/aula/Misura.svelte`; `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 8 e 9; `guscio.indicizza`, `guscio.riepilogoBilancio`, `guscio.riepilogoPresentato`; `Emiciclo`, la forbice di `VoceMisura`.
- Produces:

```ts
/** Il costo «se accogli»: in_aula indicizzato in sequenza sui dicasteri dell'emendamento, poi il riepilogo (lettura 5); un rifiuto resta testo. */
export function seAccogli(chiedi: { indicizza: typeof guscio.indicizza; riepilogo: typeof guscio.riepilogoBilancio }, inAula: Bilancio, e: VoceEmendamento): Promise<RiepilogoBilancio | string>;
/** La misura del bilancio in calendario, se c'è. */
export const misuraBilancio = (v: Vista): VoceMisura | undefined => v.calendario.find((m) => m.forma === "bilancio");
```

- **La presentata** (storia 129): le cifre sono quelle di `v.bilancio.in_aula`, la perdita reale e il giogo quelli di `riepilogoPresentato()`; il riquadro nero con la forbice di Camera e Senato e la maggioranza che serve; nel lato gli emendamenti con Accogli · Respingi (`{ tipo: "emendamento", misura, partito, accogli }`, diretti), il costo «se accogli» (spesa in più e spesa netta) e il default «Senza scelta: resta aperto, si vota il testo com'è»; un emendamento deciso dice com'è stato deciso. Sulle barre dei dicasteri chiesti, il foglietto dell'alleato.
- **«Modifica e ripresenta ›»** riapre la bozza in memoria da `in_aula` (lettura 4): `bozza.cambia(in_aula)` e la pagina torna alla forma della bozza fino alla nuova presentazione.
- **All'opposizione** la sezione è barrata (Task 8); il dettaglio della riga «Legge di bilancio» in `Misura.svelte` mostra le cifre di `in_aula` per dicastero e il riepilogo di `riepilogoPresentato()` (ticket 22).

- [ ] **Step 1: scrivi i test che falliscono**, in `bilancio.test.ts`: `seAccogli` su `presentato` con un `chiedi` finto che risponde dai campioni dà, per il Centrodestra, `costo_spesa` 981.850.113 e la crescita di `se_accogli.centrodestra`; con un `indicizza` che rifiuta il secondo dicastero dà il testo del rifiuto; le chiamate sono in sequenza; `misuraBilancio` su `presentato` ha l'id 3.
- [ ] **Step 2: esegui i test e verifica che falliscano.** Run: `npm --prefix <wt>/app test`. Expected: FAIL.
- [ ] **Step 3: implementa.** Il finto risponde a `indicizza` e `riepilogo_bilancio` su `in_aula` con i passi di `se_accogli_ui` (Task 7).
- [ ] **Step 4: verifica nel browser**, accanto a `22-bilancio.prototype.html`, sugli stati `presentato`, `emendamenti_decisi`, `ripresentato`, e `/partita/aula?vista=22-bilancio.json&stato=opposizione` con la riga «Legge di bilancio»: «se accogli: +981,9 mln € · spesa netta +0,8%»; Accogli porta a `emendamenti_decisi`; «Modifica e ripresenta ›» riapre la bozza con le cifre in aula. Run: `npm --prefix <wt>/app test` e `npm --prefix <wt>/app run check`. Expected: PASS, pulito.
- [ ] **Step 5: commit.** `feat(app): bilancio presentato, emendamenti e legge di bilancio in Aula`

---

### Task 11: verifica della fetta

**Subagente:** sessione principale.

**Sequenza:** dopo il Task 10. Chiude la PR 2 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 6).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** sul Bilancio in tutti gli stati dei Task 8-10, sull'Aula all'opposizione e, di nuovo, sui Conti e sul cruscotto con la riga del bilancio. Expected: uscita 0, tagli 0 (il prototipo 22 ne aveva 19 a 1280×800: la pagina vera no).
- [ ] **Step 2: tastiera, a mano**: − e + delle aliquote e del margine, Indicizza, Presenta, Accogli · Respingi, «Modifica e ripresenta ›».
- [ ] **Step 3: test e tipi**, come al Task 5, Step 3, più `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`.
- [ ] **Step 4: giro nell'app vera**, col telecomando (`app/verifica/README.md:63-98`). `npm --prefix <wt>/app run verifica:giro` va lanciato per primo e in background. Dall'edicola, una partita dell'Italia di oggi a 4,5 con la destra:
  1. al governo, Conti: compra il centro studi, il secondo acquisto rifiutato; apri il dettaglio del debito e delle regole UE;
  2. «Chiudi il turno» fino a ottobre-dicembre; dal cruscotto «Presenta la legge di bilancio ›»;
  3. Bilancio: «Indicizza» sulla Sanità, un + sull'Iva, un passo di margine dal funzionamento e uno in disavanzo, il giogo che risponde; «Presenta»;
  4. gli emendamenti col costo «se accogli»: uno accolto, uno respinto; «Modifica e ripresenta ›», indicizza tutto, ripresenta;
  5. «Chiudi il turno»: il voto del bilancio, poi i Conti dell'anno nuovo col bilancio in vigore.

  Poi i tagli nell'app vera su Conti e Bilancio (`?tagli=1`), a scala 1 e 1,25. A fine giro si rimettono le impostazioni, si cancella la cartella di verifica e si ripristina `src-tauri/Cargo.toml` (`README.md:94-98`).
- [ ] **Step 5: screenshot** del Bilancio nei suoi stati e dell'Aula all'opposizione in `docs/superpowers/verifiche/11d/`.
- [ ] **Step 6: corpo della PR 2.** Oltre alle sezioni solite: l'esito del giro; ciò che resta da provare a mano a Mario (il Tab reale, Cmd+Q con una bozza aperta).
- [ ] **Step 7: commit dei ritocchi della verifica**, se ce ne sono. `fix(app): ritocchi dalla verifica della 11d`

**Fine della PR 2 e della fetta.**
