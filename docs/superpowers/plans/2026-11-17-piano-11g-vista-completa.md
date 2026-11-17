# Piano 11g «Vista completa»: piano d'implementazione

> **Per chi esegue:** sotto-skill obbligatoria `superpowers:subagent-driven-development` (consigliata) o `superpowers:executing-plans`, un task alla volta. I passi usano le caselle (`- [ ]`).

**Obiettivo:** nessun posto vuoto resta nella UI. La fetta riempie i segnaposto «11g» di Paese, Partiti, Campagna, Trattativa, tracciati e caricature coi dati della Vista dello schema 19 e con due passacarte nuovi, e chiude il percorso di gioco: dopo questa fetta una partita si gioca dall'edicola alla fine.

**Architettura:**
- Nessuna schermata nuova. Ogni posto vive già in un componente della sua sezione (`app/src/lib/<sezione>/`) come `<Segnaposto fetta="11g" />`: il task lo sostituisce col dato, e la lettura della Vista va nel modulo `.ts` puro della sezione, provato con Vitest sulle Viste campione.
- Il guscio guadagna due passacarte verso metodi già pubblici del core: `anteprima_riposizionamento` (`crates/sim-core/src/partita.rs:3588`) e `anteprima_promesse` (`partita.rs:2825`). `sim-core` non cambia.
- Il generatore delle Viste campione impara tre cose, giocando: la Vista dopo `ProponiSpartizione`, le anteprime di riposizionamento e quelle delle promesse. Il guscio finto risponde da lì.
- Le caricature imparano il genere: il kit aggiunge ciglia, labbra, orecchino, cappello e il naso all'85%; `Faccia` e `Caricatura` ricevono `genere` dalla Vista.

**Tecnologie:** quelle della 11f (Svelte 5 con le runes, SvelteKit SPA, TypeScript, Vite, Vitest, Tauri 2, Rust). Nessuna dipendenza nuova.

**Spec:** [`docs/superpowers/specs/2026-11-12-v1-piano-11-ui.md`](../specs/2026-11-12-v1-piano-11-ui.md). Per questa fetta valgono le storie 158, 165, 167, 168 (righe 216-225; le 159-164 e 166 le hanno già chiuse 11c, 11d e 11e); i vincoli alle righe 248-371, le caricature alle righe 338-345, la sezione della fetta alla riga 421, i rimandi al core alle righe 423-450, le conferme alla riga 453, i test alle righe 455-476. La PR di questo piano corregge la spec dove le decisioni del 17/11 la cambiano (righe 342, 357, 379, 496).

**Riferimenti vincolanti per forma, ordine e testi**, in `.scratch/v1-piano-11-ui/prototipi/`, ciascuno aperto sulla forma decisa:
- `23-paese.prototype.html`, forma A · Atlante: linguette dei livelli, tipo della pericolosità (riga 376), legenda a cinque classi, scheda della regione (righe 455-475: territorio, gruppi con la quota, Senato e primo partito), margine;
- `24-partiti.prototype.html`, forma A: minaccia di scissione nell'asse coi fili ½ e 1 (righe 530-550), chip e conto in testa (righe 503, 621-622), frase della distanza nella scheda (riga 586);
- `26-campagna.prototype.html`, forma A · Tavolo: il metro della distanza con le soglie e l'anteprima dello spostamento (righe 414-440), l'intesa che perde ogni alleato;
- `27-trattativa.prototype.html`, forma A · Righe: pallino blu, alleati con l'intesa di partenza, lato «Il ribaltone» (riga 607);
- `31-tracciati.prototype.html`: lo stacco dello spread;
- `40-scelte-11g.prototype.html`: le donne nelle facce (tratti A, cappello i) e la spartizione in corso (A). È il riferimento per le decisioni 1-4.

I numeri che un prototipo segna «11g», «SIMULATE» o «rimando» ora vengono dalla Vista o dai passacarte. I testi si copiano dai prototipi; un testo che il prototipo non ha si scrive con le regole della spec (righe 313-322) e va nella tabella dei testi nuovi della PR.

**Da dove si parte:** main a `e7192a3`, con la 11f fusa (#82-#85). Le Viste campione sono rigenerate su questo main e coincidono con quelle versionate (25 file, nessuna differenza). I posti vuoti, tutti col segnaposto della 11g:
- Paese: `paese/paese.ts:20` (`livelloManca`), `paese.ts:47` (opere finite), `paese/SchedaRegione.svelte:58, 61, 65, 94`, `paese/Sondaggio.svelte:48`;
- Partiti: `partiti/AsseDeiPartiti.svelte:3`, `partiti/SchedaPartito.svelte:61, 72`;
- Campagna: `campagna/Posizione.svelte:50`, `campagna/Promesse.svelte:81`; testi `campagna.attesi_vuoto` e `campagna.entra_senato_vuoto` (`testi/interfaccia.json:538, 540`, usati in `campagna/Coalizione.svelte:42, 70`);
- Trattativa: `trattativa/Spartizione.svelte:50`, `trattativa/Alleati.svelte:94`, `trattativa/Trattativa.svelte:78`; peso scritto nella UI in `trattativa/trattativa.ts:9-12`;
- tracciati: `tracciati.ts:41-43, 118` e `componenti/Tracciato.svelte:11-13`;
- caricature: `caricature/Caricatura.svelte:3`.

## Vincoli globali

Valgono tutti i vincoli globali del piano 11e (`docs/superpowers/plans/2026-11-17-piano-11e-opposizione-e-fine.md:22-31`) e della 11f. Qui quelli che la fetta tocca di più.

- **La UI legge, il core decide** (spec:250). Lecito: scegliere la classe di un valore della Vista per colorare la mappa, cercare un partito per id, contare le voci. Ogni soglia viene da `Vista.soglie` o `Vista.campagna`; ogni peso da `VoceSpartizione.peso`; ogni anteprima dal core. Nessun numero delle regole scritto a mano.
- **Il guscio aggiunge passacarte** verso metodi già pubblici della partita, senza logica né stato (spec:253). `sim-core` non cambia.
- **Conferme** (spec:453): riposizionamento e promesse restano con la conferma sul posto; la spartizione, «Chiedi la guida» e «Torna alla spartizione di default» diretti.
- **Tono grave**: facce sostituite dalle iniziali (già in `Faccia.svelte`), colori dei partiti spenti nella mappa del voto, nessun rosso d'accento nei fili.
- **Viste campione**: si aggiungono campi solo giocando, nel generatore; due giri danno gli stessi `shasum`; il README delle Viste dice cosa è nuovo.
- **Cargo da un worktree**: `--manifest-path <wt>/Cargo.toml`, `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi` e `CARGO_TARGET_DIR=<wt>/target`, tutti assoluti. **npm**: `--prefix <wt>/app`. Un comando per chiamata, `git -C <wt>` al posto di `cd`.
- **Le batterie si lanciano a macchina scarica**: mai mentre `cargo` compila (`app/verifica/README.md`).
- **Main è protetto**: il controllo CI `test` deve passare prima di ogni squash; con «job was not acquired by Runner» si rilancia con `gh run rerun`.

## Decisioni di Mario del 17/11/2026

Prese sul prototipo `40-scelte-11g`, in una sola domanda.

1. **Donne nelle facce: A, come la spec.** A tutte ciglia, naso all'85% verso la radice (42,26) e labbra rosse; orecchino e cappello, ciascuno sì o no, dall'hash.
2. **Cappello: pieno d'inchiostro col nastro di carta.**
3. **Prova di riconoscimento sulle donne: passata** (102 facce senza nomi). Resta il debito della prova a 3-5 persone prima del rilascio.
4. **Spartizione in corso: A.** Le righe partono compilate da `Vista.spartizione`; ogni clic manda la spartizione intera; pallino blu dove `partito ≠ di_default`; «Torna alla spartizione di default» la rimette.

## Letture che la spec non fissa

Decise scrivendo il piano. Le cambio se Mario le cambia.

1. **Naso all'85% con una trasformazione, non con poligoni rifatti.** Il pennino è un poligono unico (`kit.ts:56-69`); il pezzo del naso si disegna a parte con `scale(.85)` intorno alla radice, e il filo e l'ombra del naso si scalano come nel prototipo 32 (`32-caricature.prototype.html:550-557`). Lo spessore del pennino sul naso cala del 15%: il prototipo 40 lo mostra già così.
2. **Orecchino e cappello dai bit che avanzano** dopo le sei leve (`floor(h / 864)`): le facce degli uomini non cambiano.
3. **La regola della rosa conta il genere** (chiave `naso:colletto:genere`, come `32-caricature.prototype.html:505-506, 586`, «864 per genere»): una donna non salta per far posto a un uomo con lo stesso naso. Una rosa mista può dare a un uomo la faccia di partenza dove prima saltava.
4. **Seggi ceduti al Senato: la riga si toglie.** Il core non li porta nella Vista; la metà per difetto sta solo nel commento di `Cessione` e la UI non riscrive regole. Il testo `campagna.entra_senato_vuoto` sparisce.
5. **«I tuoi attesi» senza proiezione**: il testo dice «nessuna proiezione dei seggi in questo turno», non più «arriva con la fetta 11g» (`seggiAttesi` è `null` solo senza proiezione, `campagna/campagna.ts:37`).
6. **Livello «Risultato del voto»**: la regione prende il colore del suo `primo` dell'ultimo voto (`EsitoElezioni.regioni[].primo`), con la legenda dei partiti; barrato «Nessun voto ancora» senza elezioni (prototipo 23, riga 301). Nel tono grave il retino al posto dei colori.
7. **Pericolosità**: cinque classi a passi di 0,2 sull'indice da 0 a 1 (prototipo 23, riga 317), con il selettore del tipo (tutte, terremoti, alluvioni e frane, incendi); nella scheda la parola «bassa», «media» o «alta» a terzi (riga 461).
8. **Disoccupazione e reddito**: cinque classi a passi uguali fra il minimo e il massimo delle venti regioni del turno (spec, storia 72). Il reddito in «mila €» (riga 314).
9. **Opere finite** come segno quadrato (`FormaSegno.svelte`, «opera») sulla mappa e nella scheda, da `Vista.opere`, col nome dell'atto e il trimestre della chiusura.
10. **Margine del sondaggio**: «± {m} punti» da `Rilevazione.margine` e `Incrocio.margine`, accanto al campione (prototipo 23, riga 18). È il margine dichiarato, non l'errore vero (`Incrocio.ts`).
11. **Spread**: lo stacco dello spread è `vista.soglie.crisi_debito_pb`; il tracciato perde il rimando. Il debito della 11a «`stacco` legato a `!tr.rimando`» si chiude qui: lo stacco si disegna quando la scala ce l'ha.
12. **Intesa di partenza**: barra a dieci caselle coi fili di `soglie.ultimatum` e `soglie.uscita`; «entra in ultimatum» sotto la prima, «può uscire» sotto la seconda (storia 86). Le stesse soglie nella scheda del partito alleato.
13. **Ribaltone**: lato «Il ribaltone» con i partiti di `Vista.ribaltone.coalizione` nell'ordine del core, i seggi alla Camera e al Senato dall'ultimo voto, il `primo` segnato «ha la guida».

## Punti da sorvegliare

Ognuno ha il suo test nel task indicato.

1. **Spartizione e allargamento.** Dopo `Allarga` il core rimette la spartizione al default (`partita.rs:3316`): la pagina deve mostrare la Vista nuova, non la scelta di prima, e il pallino blu sparisce. Task 11.
2. **Anteprima in ritardo.** Il giocatore cambia passo o promesse mentre un'anteprima è in volo: la risposta vecchia non scrive (`ultimoVince`, come `anteprima_campagna`). Un rifiuto del core (riposizionamento non ammesso, 4,5 contro 4,500000000000001, `difetti-core`) si mostra col suo testo. Task 7.
3. **Partito nato in partita senza riga nel voto.** Un `primo` o un `seggi_senato` di un partito che non ha colore (nato) usa il retino e il nome, e non rompe la legenda. Task 2 e Task 3.
4. **Genere mancante.** Il partito del giocatore ha `genere_leader: null` e non si disegna (già oggi); un nome senza genere (Vista vecchia) si disegna da uomo, senza errori. Task 12.
5. **Valori uguali in tutte le regioni.** Con minimo uguale al massimo (disoccupazione identica, turno zero) la mappa usa una classe sola e la legenda lo dice, senza divisioni per zero. Task 1.

## Struttura dei file

```
app/src-tauri/src/lib.rs                     + anteprima_riposizionamento, anteprima_promesse
app/src/lib/guscio/chiamate.ts               + anteprimaRiposizionamento, anteprimaPromesse
app/src/lib/guscio/finto.ts                  risposte nuove dai campioni 26 e 27
.scratch/v1-piano-11-ui/viste/genera/src/    main.rs (26), trattativa.rs (27); README.md
app/src/lib/paese/                           paese.ts, Paese.svelte, SchedaRegione.svelte, Sondaggio.svelte, Legenda dei livelli
app/src/lib/partiti/                         schieramento.ts, AsseDeiPartiti.svelte, SchedaPartito.svelte, Partiti.svelte
app/src/lib/campagna/                        campagna.ts, Posizione.svelte, Promesse.svelte, Coalizione.svelte
app/src/lib/trattativa/                      trattativa.ts, Spartizione.svelte, Alleati.svelte, Ribaltone.svelte, Trattativa.svelte
app/src/lib/tracciati.ts, componenti/Tracciato.svelte
app/src/lib/caricature/                      caricatura.ts, kit.ts, Caricatura.svelte; componenti/Faccia.svelte e i suoi chiamanti
app/src/lib/testi/interfaccia.json
docs/superpowers/verifiche/11g/              screenshot della fetta
```

## PR e ordine

- **PR 1, Paese** (`feat/11g-paese`): Task 1-4.
- **PR 2, Partiti, Campagna e tracciati** (`feat/11g-partiti-campagna`): Task 5-9.
- **PR 3, Trattativa e facce** (`feat/11g-trattativa-facce`): Task 10-14.

Le PR sono impilate: la 2 parte dalla 1, la 3 dalla 2 (memoria `pr-impilate-con-squash`). Dentro una PR i task vanno in sequenza, salvo: 5 e 6 in parallelo (guscio e generatore, file diversi); 10 parte dal ramo della PR 3 appena la 2 è chiusa.

Il round pre-commit di ogni PR: `code-reviewer`, `craft-reviewer`, `test-writer`, `security-reviewer` e `spec-reviewer` in parallelo; `rust-reviewer` nella PR 2 (guscio e generatore). `craft-reviewer` riceve gli screenshot della PR e li confronta col prototipo e coi token. Prima della PR 3: revisione finale dell'intera fetta (Task 13), con le correzioni dentro la PR 3.

### Dispatch

Chi orchestra copia nel brief di ogni task le sue righe «Skill-gate» e «Ricerca», così come sono scritte. Quando «Ricerca» porta `- nessun riferimento pertinente`, nel brief va quella riga, esatta. Dove ricerca e piano dicono cose diverse vale il piano. In un parallelo ogni subagente mette nel commit solo i file del proprio task, nominandoli uno per uno.

| Task | Subagente | Sequenza | Skill-gate |
|---|---|---|---|
| 1 | `frontend-engineer` | primo della PR 1 | salto dichiarato: modulo puro |
| 2 | `frontend-engineer` | dopo l'1 | `impeccable`, `dataviz`, `apple-design` |
| 3 | `frontend-engineer` | dopo il 2 | `impeccable`, `apple-design` |
| 4 | sessione principale | dopo il 3 | `scrittura-italiana` |
| 5 | `rust-engineer` | primo della PR 2, in parallelo col 6 | `rust-skills` |
| 6 | `rust-engineer` | in parallelo col 5 | `rust-skills` |
| 7 | `frontend-engineer` | dopo il 5 e il 6 | `impeccable`, `apple-design` |
| 8 | `frontend-engineer` | dopo il 7 | `impeccable`, `dataviz`, `apple-design` |
| 9 | sessione principale | dopo l'8 | `scrittura-italiana` |
| 10 | `frontend-engineer` | primo della PR 3 | `impeccable`, `apple-design` |
| 11 | `frontend-engineer` | dopo il 10 | `impeccable`, `apple-design` |
| 12 | `frontend-engineer` | dopo l'11 | `impeccable`, `apple-design` |
| 13 | `code-reviewer` (opus), poi chi corregge | dopo il 12 | `code-reviewer` |
| 14 | sessione principale | dopo il 13 | `scrittura-italiana` |

«Almeno una fra» le skill elencate. Ogni brief nomina anche `skill-gate` e `caveman`; quelli dei subagenti che scrivono codice anche `ponytail`.

## Dove va ogni storia

| Storia | Task |
|---|---|
| 158 | 1, 2, 3 |
| 165 | 5, 6, 7 |
| 167 | 6, 10, 11 |
| 168 | 8 (soglie dell'intesa, Aste scoperte), 12 (genere) |
| 70-74, 78, 81-83 (le parti lasciate dalla 11b) | 2, 3, 8 |

---

### Task 1: livelli della mappa e scheda, modulo puro

**Subagente:** `frontend-engineer`. **Sequenza:** primo della PR 1.

**Skill-gate:** salto dichiarato (modulo puro); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/index.md:62`: le unità territoriali e le grandezze che diventano mappe; vale il piano per classi e livelli.

**Files:**
- Modify: `app/src/lib/paese/paese.ts`
- Test: `app/src/lib/paese/paese.test.ts`

**Interfaces:**
- Consumes: `VoceRegione` (`tipi/VoceRegione.ts`), `EsitoElezioni.regioni` (`tipi/VotoRegione.ts`), `Vista.opere` (`tipi/VoceOpera.ts`).
- Produces:
  - `livelloManca(l: Livello, v: Vista): Chiave | null`: `null` se il livello ha dati; per `voto` senza `v.elezioni` la chiave `paese.voto_nessuno`. Firma nuova: aggiorna il chiamante in `Paese.svelte`.
  - `type Pericolo = "totale" | "sisma" | "idro" | "incendi"`.
  - `valoriDi(v: Vista, l: "disoccupazione" | "reddito" | "pericolosita", tipo?: Pericolo): Record<string, number>` per id di regione.
  - `classi(valori: number[], l: Livello): { soglie: number[]; classe: (x: number) => 0 | 1 | 2 | 3 | 4 }`: pericolosità a passi fissi di 0,2; disoccupazione e reddito a passi uguali fra minimo e massimo; minimo uguale al massimo = una classe sola (soglie `[]`, classe 0).
  - `votoPerRegione(v: Vista): Record<string, { primo: string; voti: number[]; senato: number[] }> | null` dall'ultima elezione; `null` senza elezioni.
  - `territorioDi(v: Vista, regione: string): { macroarea: Macroarea; disoccupazione: number; reddito: number; pericolosita: Record<Pericolo, "bassa" | "media" | "alta"> } | null` (terzi: sotto 1/3, sotto 2/3, sopra).
  - `vociSegni` aggiunge le opere: segno `opera`, testo `{atto} · {trimestre}` (`Vista.opere[].regione` è un id).

- [ ] **Step 1: test che falliscono** in `paese.test.ts`, sulle Viste campione:
  - 03 (`03-governo-ordinario.json`): `valoriDi(v,"disoccupazione")` ha 20 chiavi e la media pesata non serve: basta che `lombardia` sia `v.regioni.find(r=>r.id==="lombardia").disoccupazione`;
  - `classi([0.1,0.5,0.9],"pericolosita").classe(0.5) === 2`, `classe(1) === 4`, `classe(0) === 0`;
  - `classi([5,5,5],"disoccupazione")`: `soglie` vuote, `classe(5) === 0`;
  - 01 (turno zero): `livelloManca("voto", v) === "paese.voto_nessuno"`, `votoPerRegione(v) === null`;
  - 03: `votoPerRegione(v).lombardia.primo` uguale a `v.elezioni.at(-1).regioni.find(r=>r.regione==="lombardia").primo`;
  - una Vista con `opere` (la 05): `segniDi(v, v.opere[0].regione)` contiene un segno `opera`.
- [ ] **Step 2: verifica che falliscano.** Run: `npm --prefix <wt>/app test -- paese`. Expected: FAIL.
- [ ] **Step 3: implementa** in `paese.ts`. Togli il commento «In 11b: consenso e affluenza» e «Le opere finite arrivano in 11g».
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): livelli e territorio del Paese dalla Vista`

---

### Task 2: la mappa coi sei livelli e il margine del sondaggio

**Subagente:** `frontend-engineer`. **Sequenza:** dopo l'1.

**Skill-gate:** `impeccable`, `dataviz` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/paese/Paese.svelte`, `Sondaggio.svelte`, `app/src/lib/testi/interfaccia.json`

**Interfaces:**
- Consumes: Task 1.

Forma: prototipo 23 forma A. Le linguette non sono più barrate (salvo «Risultato del voto» senza elezioni, col motivo). Per la pericolosità il selettore del tipo accanto alle linguette (riga 376). Legenda a cinque classi a passi uguali; il voto colora col partito primo e mette la legenda dei partiti (lettura 6), il retino per un partito nato. La lista accanto ordina sul valore del livello (riga 314 per i formati). Il margine «± {m} punti» accanto al campione nella media degli istituti e nell'incrocio (lettura 10); il `Segnaposto` di `Sondaggio.svelte:48` sparisce.

- [ ] **Step 1: guarda la pagina** su `/partita/paese?vista=23-paese.json&stato=governo`, `&stato=opposizione`, `&stato=zero`, `&stato=grave`, `?vista=05-crisi.json`, a 1280×800 e 1440×900, su ogni livello. Expected: la forma del 23; nessun testo fuori dal suo riquadro; al turno zero il voto barrato con «Nessun voto ancora».
- [ ] **Step 2: tastiera, a mano**: le linguette e il selettore del tipo col Tab e con le frecce; un livello barrato si legge col fuoco.
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 4: commit.** `feat(app): mappa del Paese coi sei livelli e il margine`

---

### Task 3: la scheda della regione completa

**Subagente:** `frontend-engineer`. **Sequenza:** dopo il 2.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/paese/SchedaRegione.svelte`, `interfaccia.json`

**Interfaces:**
- Consumes: `territorioDi`, `votoPerRegione`, `segniDi` (Task 1).

Forma: prototipo 23, righe 455-475. Al posto dei quattro segnaposto (righe 58, 61, 65, 94): territorio (macroarea, disoccupazione «· trimestre prima», reddito «per elettore, l'anno», pericolosità per tipo a parole); il margine accanto al campione del sondaggio della regione; i diciotto gruppi coi bottoni che mostrano la quota (`VoceRegione.gruppi[i*3+j]`, condizione × fascia, già nell'ordine di `CONDIZIONI` e `FASCE`); «Senato» con i seggi per partito dell'ultimo voto e «Primo partito: …» (nessun voto: la riga lo dice). Le opere finite stanno fra i segni in testa (Task 1).

- [ ] **Step 1: guarda la scheda** di Lombardia, Molise e Valle d'Aosta nelle Viste del Task 2. Expected: diciotto quote che sommano al 100% a meno degli arrotondamenti; seggi del Senato e primo partito; nessun taglio.
- [ ] **Step 2: tastiera, a mano**: i diciotto bottoni col Tab, l'elettore campione si apre da tastiera come oggi.
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 4: commit.** `feat(app): scheda della regione completa`

---

### Task 4: verifica della PR 1

**Subagente:** sessione principale. **Sequenza:** dopo il 3. Chiude la PR 1.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 4).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** (`npm --prefix <wt>/app run verifica:chrome` e `verifica:webkit`) sulle pagine del Task 2 più `cruscotto=/partita/cruscotto?vista=08-tono-grave.json`. Expected: uscita 0, tagli 0, movimento e tastiera senza segnalazioni.
- [ ] **Step 2: screenshot** del Paese su ogni livello e della scheda in `docs/superpowers/verifiche/11g/`.
- [ ] **Step 3: round pre-commit** (sezione «PR e ordine») e correzioni.
- [ ] **Step 4: PR 1** con «Per Mario», testi nuovi in tabella, screenshot citati, ciò che resta da provare a mano. Merge squash a revisione pulita e CI verde.

**Fine della PR 1.**

---

### Task 5: passacarte delle anteprime della campagna

**Subagente:** `rust-engineer`. **Sequenza:** primo della PR 2, in parallelo col 6.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/sintesi/vista-per-schermata.md:33`: i passacarte di sola lettura sono involucri di una riga su `sulla_partita`.

**Files:**
- Modify: `app/src-tauri/src/lib.rs` (comandi e `generate_handler!`, riga 642); `app/src/lib/guscio/chiamate.ts` (oggetto `guscio`, riga 53)
- Test: `app/src-tauri/src/lib.rs` `mod tests`; `app/src/lib/guscio/chiamate.test.ts`

**Interfaces:**
- Consumes: `Partita::anteprima_riposizionamento(&self, posizione: f64) -> Result<AnteprimaRiposizionamento, Errore>` (`partita.rs:3588`); `Partita::anteprima_promesse(&self, promesse: &[(String, String)]) -> Result<Vec<VoceCostoPromessa>, Errore>` (`partita.rs:2825`); `sulla_partita` (`lib.rs:435`).
- Produces:
  - Rust: `anteprima_riposizionamento(posizione: f64)` e `anteprima_promesse(promesse: Vec<(String, String)>)`, come `anteprima_proposta` (`lib.rs:528-534, 620-626`).
  - TS: `guscio.anteprimaRiposizionamento(posizione: number) => Promise<AnteprimaRiposizionamento>` (comando `"anteprima_riposizionamento"`, `{ posizione }`); `guscio.anteprimaPromesse(promesse: [string, string][]) => Promise<VoceCostoPromessa[]>` (`"anteprima_promesse"`, `{ promesse }`).

- [ ] **Step 1: test che falliscono.** In `lib.rs`: `anteprima_riposizionamento_senza_partita_e_un_guasto`; `anteprima_riposizionamento_e_quella_del_core` (turno zero, passo +0,5, uguale al metodo del core); `anteprima_promesse_fuori_campagna_e_un_rifiuto` (`Specie::Rifiuto`); `anteprima_promesse_e_quella_del_core`. In `chiamate.test.ts` le due righe della tabella.
- [ ] **Step 2: verifica che falliscano.** Run: `TS_RS_EXPORT_DIR=<wt>/app/src/lib/tipi CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml -p app anteprima_` e `npm --prefix <wt>/app test -- chiamate`. Expected: FAIL.
- [ ] **Step 3: implementa.**
- [ ] **Step 4: verifica.** Gli stessi comandi, più `cargo clippy --manifest-path <wt>/Cargo.toml --workspace -- -D warnings`. Expected: verdi. Controlla che `git -C /Users/mario/GitHub/simulatore-politico status` non mostri tipi esportati nel checkout condiviso.
- [ ] **Step 5: commit.** `feat(guscio): anteprime di riposizionamento e promesse`

---

### Task 6: Viste campione con le anteprime e la spartizione in corso

**Subagente:** `rust-engineer`. **Sequenza:** in parallelo col 5.

**Skill-gate:** `rust-skills`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `.scratch/v1-piano-11-ui/viste/genera/src/main.rs` (`campagna`, riga 1730; riposizionamento righe 1819-1826, promesse 1832-1840, prove 1893-1920); `.scratch/v1-piano-11-ui/viste/genera/src/trattativa.rs` (`proposta`, riga 47); `.scratch/v1-piano-11-ui/viste/README.md`; i JSON rigenerati `26-campagna.json`, `27-trattativa.json`

**Interfaces:**
- Produces (campi nuovi, nessun valore esistente cambia):
  - `26-campagna.json`: `anteprime_riposizionamento` = per stato (`zero`, `governo_fine`, `opposizione_fine`) un oggetto con chiave la posizione in testo (`"4"`, `"4.25"`, …, come `riposiziona`) delle quattro posizioni di `posizioniVicine` (`campagna/campagna.ts:155`), valore `{ ok: AnteprimaRiposizionamento } | { rifiuto: string }`;
  - `26-campagna.json`: `anteprime_promesse` = per stato (`zero`, `governo_fine`) un oggetto con chiave le coppie unite da `|` e `=` (`"reddito_minimo=universale|eta_pensionabile=64"`), per una promessa sola e per le coppie già usate da `promesse` (righe 1832-1834), valore `{ ok: VoceCostoPromessa[] } | { rifiuto }`;
  - `27-trattativa.json`: ogni voce di `premio.proposte` guadagna `vista`, la Vista dopo il comando e prima della chiusura (la sua `spartizione` è la proposta).

- [ ] **Step 1: aggiungi i tre campi** chiamando i metodi del core sulla partita dello stato (una copia per chiamata, come `prova`).
- [ ] **Step 2: rigenera due volte** (`viste/README.md:13-15`, con `CARGO_TARGET_DIR=<wt>/target` e `--manifest-path <wt>/.scratch/…/genera/Cargo.toml`) e confronta gli `shasum`. Expected: stessi `shasum`; `git diff --stat` tocca solo 26 e 27 (e il README); i valori che c'erano restano uguali (`node` che confronta il JSON vecchio senza i campi nuovi).
- [ ] **Step 3: README** delle Viste: i campi nuovi nelle righe di 26 e 27 e «rigenerate il 17/11/2026 su main `e7192a3`».
- [ ] **Step 4: commit.** `chore(viste): anteprime della campagna e spartizione in corso`

---

### Task 7: Campagna, le due mosse a occhi aperti

**Subagente:** `frontend-engineer`. **Sequenza:** dopo il 5 e il 6.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/campagna/campagna.ts`, `Posizione.svelte`, `Promesse.svelte`, `Coalizione.svelte`; `app/src/lib/guscio/finto.ts`; `interfaccia.json`
- Test: `app/src/lib/campagna/campagna.test.ts`, `app/src/lib/guscio/finto.test.ts`

**Interfaces:**
- Consumes: `guscio.anteprimaRiposizionamento`, `guscio.anteprimaPromesse` (Task 5); campi del Task 6; `ultimoVince` (`editor/editor.svelte`).
- Produces:
  - `campagna.ts`: `distanzaDi(v: Vista): number` (percepita meno dichiarata del giocatore, in valore assoluto); `sogliaDi(d: number, c: VistaCampagna): "sotto" | "minaccia" | "scissione"` su `c.minaccia_scissione` e `c.soglia_scissione`;
  - `finto.ts`: `anteprima_riposizionamento` e `anteprima_promesse` rispondono dai campi del Task 6 dello stato chiesto, altrimenti il rifiuto «non nel campione: …».

Forma: prototipo 26, righe 414-440. **Posizione**: il metro della distanza con i fili di minaccia e scissione da `vista.campagna`; scelto un passo, `anteprimaRiposizionamento` dà la dichiarata e la percepita nuove e la distanza nuova sul metro; la conferma sul posto dice «la distanza sale/scende a {d}» e, oltre la soglia, che alla chiusura il partito si scinde. Un rifiuto del core sta sotto i passi. **Promesse**, al governo: a ogni scelta `anteprimaPromesse` e una riga per alleato «perde {n} di intesa» con le promesse su cui è contrario; nessuna perdita lo dice. **Coalizione**: letture 4 e 5.

- [ ] **Step 1: test che falliscono.** `campagna.test.ts`: `sogliaDi(0.6, {minaccia_scissione: .5, soglia_scissione: 1, …})` è `"minaccia"`, `sogliaDi(1, …)` è `"scissione"`; sul 26 `opposizione_fine` `distanzaDi` vale 0,61 a due decimali (README delle Viste). `finto.test.ts`: `anteprima_riposizionamento` col passo `"4.5"` di `zero` dà il valore del campione; con una posizione assente il rifiuto «non nel campione».
- [ ] **Step 2: verifica che falliscano.** Run: `npm --prefix <wt>/app test -- campagna finto`. Expected: FAIL.
- [ ] **Step 3: implementa**, poi guarda `/partita/campagna?vista=26-campagna.json&stato=zero`, `governo_fine`, `opposizione_fine` alle due finestre.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): anteprime di riposizionamento e promesse nella Campagna`

---

### Task 8: Partiti con la minaccia e le soglie, e lo stacco dello spread

**Subagente:** `frontend-engineer`. **Sequenza:** dopo il 7.

**Skill-gate:** `impeccable`, `dataviz` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/partiti/schieramento.ts`, `AsseDeiPartiti.svelte`, `SchedaPartito.svelte`, `Partiti.svelte`; `app/src/lib/tracciati.ts`, `app/src/lib/componenti/Tracciato.svelte`; `interfaccia.json`
- Test: `app/src/lib/partiti/schieramento.test.ts`, `app/src/lib/tracciati.test.ts`

**Interfaces:**
- Consumes: `VocePartito.minaccia`, `Vista.campagna.minaccia_scissione`, `soglia_scissione`, `Vista.soglie` (`ultimatum`, `uscita`, `crisi_debito_pb`).
- Produces: `avvisi` (`schieramento.ts:31`) aggiunge `"minaccia_scissione"` per `p.minaccia && !p.sciolto`; `fraseDistanza(p: VocePartito, c: VistaCampagna): Chiave` per la scheda (prototipo 24, riga 586); in `tracciati.ts` la scala dello spread ha `stacco: v.soglie.crisi_debito_pb` e il tracciato `rimando: null`.

Forma: prototipo 24. Nell'asse la freccia rossa per chi ha `minaccia` (non nel tono grave), i fili ½ e 1 dalla dichiarata verso la percepita alle distanze di `vista.campagna`; il chip «minaccia di scissione» e «· N in minaccia di scissione» in testa. Nella scheda la frase della distanza al posto di `partiti.ph_scissione`, e sulla barra dell'intesa i fili di `soglie.ultimatum` e `soglie.uscita` al posto di `partiti.ph_soglie` (`Caselle` ha già `filo`). Il tracciato dello spread disegna lo stacco della soglia; in `Tracciato.svelte` lo stacco dipende dalla scala, non da `!tr.rimando` (lettura 11). Togli il commento «arrivano con la 11g» da `AsseDeiPartiti.svelte:3` e da `schieramento.ts:31` (per la parte del governo IA resta il rimando al core).

- [ ] **Step 1: test che falliscono.** `schieramento.test.ts`: su una Vista del 05 con un partito in `minaccia`, `avvisi` contiene `"minaccia_scissione"`; uno sciolto no. `tracciati.test.ts`: sul 21 `spread_alto` il terzo tracciato ha `rimando === null` e `scala.stacco === v.soglie.crisi_debito_pb`.
- [ ] **Step 2: verifica che falliscano.** Run: `npm --prefix <wt>/app test -- schieramento tracciati`. Expected: FAIL.
- [ ] **Step 3: implementa**, poi guarda `/partita/partiti?vista=05-crisi.json`, `?vista=03-governo-ordinario.json`, `?vista=08-tono-grave.json`, e la cornice su `21-conti.json&stato=spread_alto`.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 5: commit.** `feat(app): minaccia di scissione, soglie dell'intesa e stacco dello spread`

---

### Task 9: verifica della PR 2

**Subagente:** sessione principale. **Sequenza:** dopo l'8. Chiude la PR 2.

**Skill-gate:** `scrittura-italiana` sul corpo della PR.

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** su Campagna (`26` stati `zero`, `governo_fine`, `opposizione_fine`, `cessione_che_convince`), Partiti (`03`, `05`, `08`), cornice con lo spread (`21-conti.json&stato=spread_alto`). Expected: uscita 0, tagli 0.
- [ ] **Step 2: test e tipi**, più `CARGO_TARGET_DIR=<wt>/target cargo test --manifest-path <wt>/Cargo.toml --workspace --no-fail-fast`.
- [ ] **Step 3: screenshot** in `docs/superpowers/verifiche/11g/`; round pre-commit con `rust-reviewer`; correzioni.
- [ ] **Step 4: PR 2** impilata sulla 1; dopo lo squash della 1, `merge -s ours` e CI verde; squash.

**Fine della PR 2.**

---

### Task 10: la spartizione in corso, modulo puro e guscio finto

**Subagente:** `frontend-engineer`. **Sequenza:** primo della PR 3.

**Skill-gate:** `impeccable` o `apple-design` (o salto dichiarato: modulo puro); `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/trattativa/trattativa.ts`, `app/src/lib/guscio/finto.ts` (`proponi_spartizione`, righe 231-235)
- Test: `app/src/lib/trattativa/trattativa.test.ts`, `finto.test.ts`

**Interfaces:**
- Consumes: `Vista.spartizione`, `Vista.richieste[].intesa_partenza`, `Vista.ribaltone`, `Vista.soglie`.
- Produces (sostituiscono `pesoDi`, `Bozza`, `completa`, `sessioneSpartizione`):
  - `proponi(v: Vista, dicastero: string, a: string | null): Comando`: la spartizione intera di `v.spartizione` con quel dicastero cambiato;
  - `predefinita(v: Vista): Comando`: la spartizione con ogni `partito = di_default`;
  - `cambiata(v: Vista): boolean`: almeno una riga con `partito !== di_default`;
  - `quota(v: Vista, a: string | null): { n: number; punti: number }` sui `peso` di `v.spartizione`;
  - `richiesteDi` aggiunge `intesa: number` (`intesa_partenza`) e `stato: "ok" | "ultimatum" | "uscita"` su `soglie.ultimatum` e `soglie.uscita`;
  - `ribaltoneDi(v: Vista): { id: string; nome: string; camera: number; senato: number; primo: boolean }[] | null`;
  - finto: `proponi_spartizione` risponde con la `vista` della proposta del campione (Task 6), altrimenti il rifiuto «non nel campione».

- [ ] **Step 1: test che falliscono** sul 27:
  - `premio.vista`: `cambiata` è falso; `quota(v,"giocatore").punti` uguale alla somma dei `peso` delle righe del giocatore;
  - `proponi(v,"economia_finanze","giocatore")` dà dieci voci, solo Economia cambiata;
  - la `vista` di `avara_destra`: `cambiata` vero, la Destra con `stato: "ultimatum"` (intesa 32,1);
  - `guida.vista`: `ribaltoneDi` dà Tu, Sinistra, Centrosinistra, Centro in quest'ordine, la Sinistra `primo`;
  - finto: `comanda` con la spartizione di `generosa_centrodestra` rende la sua `vista`.
- [ ] **Step 2: verifica che falliscano.** Run: `npm --prefix <wt>/app test -- trattativa finto`. Expected: FAIL.
- [ ] **Step 3: implementa.** Togli il codice morto della bozza; i test vecchi della sessione si cancellano con lei.
- [ ] **Step 4: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi. Perché `check` resti pulito, il task adatta i componenti della Trattativa alle firme nuove quanto basta a compilare (la forma nuova è del Task 11).
- [ ] **Step 5: commit.** `feat(app): spartizione in corso dal core`

---

### Task 11: Trattativa, righe, alleati e ribaltone

**Subagente:** `frontend-engineer`. **Sequenza:** dopo il 10.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- nessun riferimento pertinente

**Files:**
- Modify: `app/src/lib/trattativa/Spartizione.svelte`, `Alleati.svelte`, `Ribaltone.svelte`, `Trattativa.svelte`; `interfaccia.json`

**Interfaces:**
- Consumes: Task 10.

Forma: prototipo 40, scheda 2, variante A, e 27 forma A. Le righe compilate da `vista.spartizione`, i pesi da `peso`, il bottone premuto = `partito`; ogni clic manda `proponi` (una chiamata in volo per volta; un rifiuto sotto la riga cliccata); pallino blu sulle righe cambiate; «Torna alla spartizione di default» diretto, solo se `cambiata`. Negli alleati la barra dell'intesa di partenza coi fili delle soglie e «entra in ultimatum» o «può uscire». Dopo `Allarga` la pagina mostra la Vista nuova (punto da sorvegliare 1). Il ribaltone: il lato coi partiti, i seggi e chi ha la guida al posto del segnaposto di `Trattativa.svelte:78`.

- [ ] **Step 1: guarda la pagina** su `/partita/trattativa?vista=27-trattativa.json&stato=premio`, `senza_maggioranza`, `guida`, `capo_dello_stato`, e `?vista=02-trattativa-incarico.json`, alle due finestre; clicca le cinque proposte del campione. Expected: le intese del prototipo 40 (default 55/62, avara 55/32 con «entra in ultimatum»).
- [ ] **Step 2: tastiera, a mano**: i bottoni «a chi» col Tab, Invio manda; il link del default da tastiera.
- [ ] **Step 3: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 4: commit.** `feat(app): trattativa con la spartizione in corso e il ribaltone`

---

### Task 12: il genere nelle facce

**Subagente:** `frontend-engineer`. **Sequenza:** dopo l'11.

**Skill-gate:** `impeccable` o `apple-design`; `skill-gate`; `caveman`; `ponytail`.

**Ricerca:**
- `docs/ricerca/estetica/caricature-di-personaggi-inventati.md`: kit SVG di parti col seme, somiglianza esclusa per costruzione.

**Files:**
- Modify: `app/src/lib/caricature/caricatura.ts`, `kit.ts`, `Caricatura.svelte`; `app/src/lib/componenti/Faccia.svelte`; i chiamanti di `Faccia` (`cruscotto/DettaglioVoce.svelte:149`, `cruscotto/Cruscotto.svelte:81`, `governo/CartaMinistro.svelte:66, 142`, `governo/Dirotta.svelte:64`, `governo/Rosa.svelte:25`, `governo/Figurina.svelte:49, 79`, `partiti/Partiti.svelte:67`, `partiti/SchedaPartito.svelte:43`, `trattativa/Alleati.svelte:88`), `sviluppo/ProvaCaricature.svelte`
- Test: `app/src/lib/caricature/caricatura.test.ts`, `kit.test.ts`, `render.test.ts`

**Interfaces:**
- Consumes: `VoceMinistro.genere`, `VocePartito.genere_leader` (`tipi/Genere.ts`: `"donna" | "uomo"`).
- Produces:
  - `Leve` guadagna `orecchino: boolean; cappello: boolean`, dai bit dopo le sei leve (lettura 2): `leveDi` resta uguale sulle sei leve di prima;
  - `salti(ruolo, candidati: { partito: string; nome: string; genere?: Genere }[])` e `saltiDeiLeader` usano la chiave `naso:colletto:genere` (lettura 3);
  - `kit.ts`: `profilo(l: Leve, donna = false)` con naso all'85% (lettura 1) e `TRATTI` = tracciati di ciglia (due forme, una per occhio), labbra, orecchino, cappello e nastro, copiati dal prototipo 40;
  - `Caricatura` e `Faccia` hanno la prop `genere?: Genere` (assente = uomo).

Forma: prototipo 40, scheda 1, tratti A, cappello «i» (decisioni 1 e 2): labbra rosse col filo di carta, orecchino giallo, cappello pieno d'inchiostro col nastro di carta. Tolto il commento «fino alla 11g» da `Caricatura.svelte:3`.

- [ ] **Step 1: test che falliscono.**
  - `caricatura.test.ts`: per 600 chiavi le sei leve di `leveDi` sono quelle di oggi (istantanea presa prima della modifica); `orecchino` e `cappello` danno tutte e quattro le combinazioni su 600 chiavi; con due candidati di genere diverso e stesso naso e colletto nessuno salta; con due donne uguali la seconda salta.
  - `render.test.ts`: con `genere="donna"` il SVG contiene le labbra rosse (`class="rosso"` sulla bocca) e il naso scalato (`scale(.85)`); con `genere="uomo"` o senza, il SVG è uguale a quello di oggi; il peso di una faccia resta sotto il tetto del test che c'è.
- [ ] **Step 2: verifica che falliscano.** Run: `npm --prefix <wt>/app test -- caricatur render kit`. Expected: FAIL.
- [ ] **Step 3: implementa** e passa `genere` da ogni chiamante.
- [ ] **Step 4: guarda** Governo (`18-governo.json`, stati `scelte` e `scontenti`), Partiti (`03`), Trattativa (`27`, `premio`) e `/sviluppo/caricature` alle due finestre. Expected: le facce del prototipo 40; nel tono grave (`08`) le iniziali.
- [ ] **Step 5: verifica.** Run: `npm --prefix <wt>/app test`, `npm --prefix <wt>/app run check`. Expected: verdi, pulito.
- [ ] **Step 6: commit.** `feat(app): donne nelle caricature`

---

### Task 13: revisione finale della fetta

**Subagente:** `code-reviewer` su opus, sull'intero diff da `e7192a3` alla punta della PR 3; poi `frontend-engineer` per le correzioni.

**Sequenza:** dopo il 12, prima della PR 3.

**Skill-gate:** `code-reviewer`; `skill-gate`; `caveman` (e `ponytail` per chi corregge).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: revisione** contro questo piano e la spec: posti vuoti rimasti (`grep -rn "11g" app/src`, che deve dare solo commenti di storia), numeri scritti nella UI, regole rifatte, prove mancanti dei cinque punti da sorvegliare.
- [ ] **Step 2: correzioni** dei finding importanti e critici, con test; i minori che costano poco. I minori lasciati vanno nel corpo della PR 3.
- [ ] **Step 3: ri-revisione** fino a «pulito».
- [ ] **Step 4: commit.** `fix(app): correzioni della revisione finale della 11g`

---

### Task 14: verifica della PR 3 e giro giocato completo

**Subagente:** sessione principale. **Sequenza:** dopo il 13. Chiude la PR 3 e la fetta.

**Skill-gate:** `scrittura-italiana` sul corpo della PR (Step 5).

**Ricerca:**
- nessun riferimento pertinente

- [ ] **Step 1: batterie nei due motori** sulla Trattativa (stati del Task 11), sul Governo (`18`, `scelte`), sui Partiti (`03`), di nuovo su Paese e Campagna (uno stato ciascuno) e sul tono grave (`08`). Expected: uscita 0, tagli 0.
- [ ] **Step 2: test, tipi e Rust** come al Task 9, Step 2.
- [ ] **Step 3: giro giocato nell'app vera** col telecomando (`npm --prefix <wt>/app run verifica:giro`), una partita intera: edicola, scenario, fondazione dell'Italia di oggi a 4,5; turno zero con coalizione, campagna (anteprima del riposizionamento, promesse); il voto; la trattativa con una spartizione cambiata e tornata al default; al governo almeno un bilancio, una legge, Ferma o Dirotta; il Paese su ogni livello e una scheda; la Prima pagina e il Registro; poi «Chiudi il turno» fino alla fine della partita (o fino all'opposizione e «Avanti fino a novità» fino alla fine), l'Eredità, «Conferma l'eredità», l'edicola con la voce d'albo. Cmd+ e Cmd−, il movimento spento. Ogni intoppo che rompe il gioco è un difetto da correggere in questa PR. A fine giro si rimettono le impostazioni, si cancella la cartella di verifica e si ripristina `app/src-tauri/Cargo.toml`.
- [ ] **Step 4: screenshot** della Trattativa, delle facce e del giro in `docs/superpowers/verifiche/11g/`.
- [ ] **Step 5: PR 3** con «Per Mario», l'esito del giro, i minori rimasti, ciò che resta da provare a mano (Tab reale, resa a occhio delle facce a 34 px, prova a 3-5 persone). Merge squash a revisione pulita e CI verde.

**Fine della PR 3 e della fetta.**

## Rimandi al core

Nessuno nuovo: ogni posto della 11g ha il suo dato nella Vista. Restano aperti quelli delle fette prima, che la fetta non tocca: chi guida il governo uscente (e quindi Governo in trattativa sempre barrato), l'incarico dopo «Resta fuori» distinto da quello a un'altra coalizione, il costo della sfiducia respinta, l'etichetta delle leggi nella voce d'albo, la soglia delle firme, e i cinque difetti di `difetti-core-dalla-mappa-11`.
