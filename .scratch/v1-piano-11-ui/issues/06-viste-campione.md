Type: task
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:task
Map: ../map.md

## Question

Produrre Viste vere in JSON per nutrire i prototipi: turno zero prima del voto, trattativa con l'incarico al giocatore, governo in carica in un turno ordinario e nel turno del bilancio, crisi, opposizione sotto un governo IA, partita finita con l'eredità. Da `sim-cli` o da un test che serializza la Vista; salvate in `.scratch/v1-piano-11-ui/viste/`, una per stato, con seme e comandi che la riproducono. Nessuna modifica a `sim-core` né alla Vista: se uno stato non si raggiunge da main, annotarlo.

## Answer

Fatto l'11/11/2026. Otto Viste in [`../viste/`](../viste/README.md), tutte raggiunte **giocando** sui dati veri di `dati/base` (nessuna forzata dal salvataggio, casse IA intatte), codice e dati identici a main `a02f6ad`. Il generatore è [`../viste/genera/`](../viste/genera/src/main.rs): package Cargo a sé con `sim-core` per percorso, niente di toccato in `crates/`, `app/`, `dati/`. Rigenerato una seconda volta dal thread principale: stessi `shasum`.

| File | Stato | Seme | Posizione | Turno |
|---|---|---|---|---|
| `01-turno-zero.json` | Camere sciolte, prima del voto | 4 | 4,5 | 0, 2026T4 |
| `02-trattativa-incarico.json` | trattativa, incarico al giocatore | 4 | 4,5 | 1, 2027T1 |
| `03-governo-ordinario.json` | al governo, evento aperto, 3 proposte dei ministri | 4 | 4,5 | 2, 2027T2 |
| `04-governo-bilancio.json` + `.bozza.json` | al governo nel turno del bilancio, bilancio presentato | 4 | 4,5 | 4, 2027T4 |
| `05-crisi.json` | crisi fuori dall'esercizio provvisorio, 5 voci di disponibilità | 9 | 4,5 | 34, 2035T2 |
| `06-opposizione-governo-ia.json` | opposizione sotto un governo IA | 1 | 1,5 | 2, 2027T2 |
| `07-partita-finita-eredita.json` | partita finita per abbandono, eredità in Vista | 4 | 4,5 | 9, 2029T1 |
| `08-tono-grave.json` | al governo con «Emergenza incendi» in tono grave | 4 | 4,5 | 7, 2028T3 |

Comando unico e mosse di ogni file nel [README](../viste/README.md).

Fatti che servono ai ticket dopo:

- **La Vista non basta a tutte le schermate.** La bozza del bilancio non è nella Vista (`bilancio` ha solo `presentabile`/`presentato`): sta in `04-governo-bilancio.bozza.json`. Edicola, Scenario ed editor, Fondazione e Anteprima della misura leggono metodi fuori dalla Vista (indice delle partite e albo, riepilogo dello scenario, anteprima della fondazione, `anteprima`, `riepilogo_bilancio`): la sessione che prototipa quella schermata aggiunge una funzione a `genera/` e rigenera, senza ticket nuovo.
- **`Vista.richieste` non sono le richieste dei ministri**: sono quelle degli alleati in trattativa (`crates/sim-core/src/partita.rs:3263`). Le proposte dei ministri stanno in `ministeri[].proposta`.
- **`programma` è vuoto quando il giocatore è Presidente del Consiglio**, per costruzione (`crates/sim-core/src/vista.rs:91`); pieno solo all'opposizione.
- **Il tono grave arriva già in trattativa**: nel 02 c'è un'emergenza idrogeologica aperta in tono grave.
- **Partita finita**: `fase` resta `ordinaria`, `governa` false; lo stato si legge da `finita` ed `eredita`, non dalla fase.
- **Numeri non arrotondati** (`5.6000000000000005`): arrotonda la UI ([Testi esterni](03-testi-esterni.md), `Intl`).
- `referendum` ed `esiti_referendum` vuoti in tutte le Viste: nessuno stato campione li mostra; servirà una Vista in più a chi prototipa i referendum in Aula.

Fuori dalla mappa, da guardare nel core: coi turni chiusi senza decisioni l'inflazione della prima stima scendeva da 4,2 (turno zero) a −4,33 (2028T4, file 07) ed era già −0,51 al 2027T1 nel 06.

**Aggiornamento dell'11/11/2026.** Il core è stato corretto e le otto Viste rigenerate su main `94087b9` (tabella sopra aggiornata: la crisi ora è il seme 9, turno 34). Due fatti nuovi per i ticket dopo:

- **L'inflazione da titolo è `statistica.prima_stima.inflazione_annua`** (prezzi sullo stesso trimestre dell'anno prima), non `inflazione`, che è la variazione del trimestre annualizzata e resta nella `Stima` accanto. Nelle Viste l'annua va da 4,2 a 0,71; la trimestrale scende fino a −0,32. L'indicatore `inflazione` dell'eredità è l'annua.
- **La crescita della spesa netta del riepilogo di bilancio è una stima**: su 30 semi dista in media 0,3-0,4 punti da quella vera, contro un tetto di 1,5-1,7. Chi prototipa la schermata del bilancio decide come mostrarla; non è ancora deciso.
