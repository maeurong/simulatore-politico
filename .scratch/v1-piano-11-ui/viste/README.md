# Viste campione (ticket 06)

`partita.vista()` serializzata, una per stato, sui dati veri di `dati/base` (non `a_riposo`, casse IA
intatte). Generate da `genera/` (package Cargo a sé, `sim-core` per percorso, lockfile di root
potato da cargo alle sole crate usate, versioni identiche).

Codice: `crates/` e `dati/` identici a main `a02f6ad` (`git diff --stat a02f6ad ad5eafd -- crates dati Cargo.lock`
vuoto); generate dal branch `docs/piano-11-mappa`, HEAD `ad5eafd`.

Rigenerare tutto (deterministico: due giri, stessi `shasum`):

    CARGO_TARGET_DIR=/Users/mario/GitHub/simulatore-politico/target cargo run --manifest-path /Users/mario/GitHub/simulatore-politico/.scratch/v1-piano-11-ui/viste/genera/Cargo.toml

Il programma prova i semi da 1 in su e prende il primo turno che soddisfa ogni stato; stampa su
stderr seme, turno, mosse e byte di ogni file, e `NON RAGGIUNTO: <file>` per quelli mancanti.

Tutti gli stati raggiunti **giocando**, nessuno forzato col salvataggio. Nome del partito
«Partito del giocatore».

| File | Seme | Pos. | Turno | Byte | Mosse |
|---|---|---|---|---|---|
| `01-turno-zero.json` | 4 | 4,5 | 0, 2026T4 | 102336 | `fonda`, nessun comando |
| `02-trattativa-incarico.json` | 4 | 4,5 | 1, 2027T1 | 111274 | `DichiaraCoalizione` coi partiti disponibili fra destra, centrodestra, estrema destra; chiudi |
| `03-governo-ordinario.json` | 4 | 4,5 | 2, 2027T2 | 110777 | come 02, poi chiudi (trattativa senza `ProponiSpartizione`: la spartizione di default) |
| `04-governo-bilancio.json` | 4 | 4,5 | 4, 2027T4 | 116805 | come 03, chiudi ×2, poi `PresentaBilancio` con `bozza_bilancio()`; Vista presa prima di chiudere |
| `04-governo-bilancio.bozza.json` | 4 | 4,5 | 4 | 2350 | `bozza_bilancio()` dello stesso turno (la Vista non la porta: ha solo `presentabile`/`presentato`) |
| `05-crisi.json` | 13 | 4,5 | 26, 2033T2 | 119289 | come 03, poi turni chiusi senza decisioni salvo `PresentaBilancio` con la bozza a ogni turno che la offre |
| `06-opposizione-governo-ia.json` | 1 | 1,5 | 2, 2027T2 | 114444 | `fonda`, nessuna coalizione, chiudi (voto), chiudi (trattativa IA) |
| `07-partita-finita-eredita.json` | 4 | 4,5 | 9, 2029T1 | 113281 | come 05 fino all'8° turno al governo, poi `abbandona()` |
| `08-tono-grave.json` | 4 | 4,5 | 7, 2028T3 | 116376 | come 05; «Emergenza incendi» in tono grave, al governo |

Semi 1-3 a 4,5: niente incarico al giocatore dopo il voto, scartati. 03 è il primo turno al
governo, fuori dal bilancio, con un evento aperto e almeno una `ministeri[].proposta` (qui 3).

## Stranezze notate

- `richieste` è vuota in tutti gli stati di governo: per `partita.rs:3263` sono le richieste
  degli alleati **in trattativa**, non dei ministri. Le richieste dei ministri stanno in
  `ministeri[].proposta` (03: infrastrutture, ambiente, esteri; 04: sei dicasteri).
- `programma` vuoto col giocatore Presidente del Consiglio, per costruzione (`vista.rs:91-93`); pieno
  (1 voce) solo nel 06.
- 05: crisi presa fuori dall'esercizio provvisorio apposta. Il primo caso trovato (seme 9, turno 33,
  2035T1) era in esercizio provvisorio: caduta certa, `disponibilita` vuota, niente `Allarga`. Qui
  `disponibilita` ha 6 voci, `alleati` 1, `eventi` vuoto, seconda legislatura (`elezioni` 2).
- 02: `alleati` vuoto in trattativa; c'è già un evento aperto in tono grave (emergenza
  idrogeologica), che al turno dopo (03) torna ordinario.
- 01: `coalizione` vuota (nessuna dichiarazione al turno zero), `statistica.revisione` `null`,
  `elezioni`/`mosse`/`eventi`/`calendario` vuoti, `turni_al_voto` 0, `bilancio.presentabile`
  false nonostante sia un ottobre-dicembre.
- 07: `eredita.fine` `abbandono`, `bozza` false, `anni_al_governo` 1,75, `leggi_in_vigore` vuota
  (nessuna legge fatta). `governa` false, `fase` resta `ordinaria`.
- Numeri non arrotondati nel JSON (`5.6000000000000005`, `139.33490824282669`): l'arrotondamento
  tocca alla UI.
- Deflazione: `statistica.prima_stima.inflazione` va da 4,2 (turno zero) a -4,33 (07, 2028T4);
  -0,51 già al 2027T1 nel 06. Turni senza decisioni, ma va guardato: l'inflazione in eredità
  (07) è -4,17.
- `referendum`, `esiti_referendum` vuoti in tutti; `alleati_avversari` pieno solo nel 06 (4 voci).
