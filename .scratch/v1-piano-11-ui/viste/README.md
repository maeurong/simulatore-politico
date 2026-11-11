# Viste campione (ticket 06)

`partita.vista()` serializzata, una per stato, sui dati veri di `dati/base` (non `a_riposo`, casse IA
intatte). Generate da `genera/` (package Cargo a sé, `sim-core` per percorso, lockfile di root
potato da cargo alle sole crate usate, versioni identiche).

Codice: `crates/` e `dati/` identici a main `94087b9` (`git diff --stat main 06ad66f -- crates dati Cargo.lock`
vuoto); generate dal branch `docs/piano-11-mappa`, HEAD `06ad66f`. Rigenerate l'11/11/2026 dopo le PR
#51 (inflazione di fondo) e #52 (inflazione annua pubblicata, schema 17): la prima versione era su
main `a02f6ad`.

Rigenerare tutto (deterministico: due giri, stessi `shasum`):

    CARGO_TARGET_DIR=/Users/mario/GitHub/simulatore-politico/target cargo run --manifest-path /Users/mario/GitHub/simulatore-politico/.scratch/v1-piano-11-ui/viste/genera/Cargo.toml

Il programma prova i semi da 1 in su e prende il primo turno che soddisfa ogni stato; stampa su
stderr seme, turno, mosse e byte di ogni file, e `NON RAGGIUNTO: <file>` per quelli mancanti.

Tutti gli stati raggiunti **giocando**, nessuno forzato col salvataggio. Nome del partito
«Partito del giocatore».

| File | Seme | Pos. | Turno | Byte | Mosse |
|---|---|---|---|---|---|
| `01-turno-zero.json` | 4 | 4,5 | 0, 2026T4 | 102367 | `fonda`, nessun comando |
| `02-trattativa-incarico.json` | 4 | 4,5 | 1, 2027T1 | 111386 | `DichiaraCoalizione` coi partiti disponibili fra destra, centrodestra, estrema destra; chiudi |
| `03-governo-ordinario.json` | 4 | 4,5 | 2, 2027T2 | 110892 | come 02, poi chiudi (trattativa senza `ProponiSpartizione`: la spartizione di default) |
| `04-governo-bilancio.json` | 4 | 4,5 | 4, 2027T4 | 117843 | come 03, chiudi ×2, poi `PresentaBilancio` con `bozza_bilancio()`; Vista presa prima di chiudere |
| `04-governo-bilancio.bozza.json` | 4 | 4,5 | 4 | 2350 | `bozza_bilancio()` dello stesso turno (la Vista non la porta: ha solo `presentabile`/`presentato`) |
| `05-crisi.json` | 9 | 4,5 | 34, 2035T2 | 119988 | come 03, poi turni chiusi senza decisioni salvo `PresentaBilancio` con la bozza a ogni turno che la offre |
| `06-opposizione-governo-ia.json` | 1 | 1,5 | 2, 2027T2 | 114555 | `fonda`, nessuna coalizione, chiudi (voto), chiudi (trattativa IA) |
| `07-partita-finita-eredita.json` | 4 | 4,5 | 9, 2029T1 | 115377 | come 05 fino all'8° turno al governo, poi `abbandona()` |
| `08-tono-grave.json` | 4 | 4,5 | 7, 2028T3 | 116474 | come 05; «Emergenza incendi» in tono grave, al governo |

Semi 1-3 a 4,5: niente incarico al giocatore dopo il voto, scartati. 03 è il primo turno al
governo, fuori dal bilancio, con un evento aperto e almeno una `ministeri[].proposta` (qui 3).

## Stranezze notate

- `richieste` è vuota in tutti gli stati di governo: per `partita.rs:3263` sono le richieste
  degli alleati **in trattativa**, non dei ministri. Le richieste dei ministri stanno in
  `ministeri[].proposta` (03: infrastrutture, ambiente, esteri; 04: sei dicasteri).
- `programma` vuoto col giocatore Presidente del Consiglio, per costruzione (`vista.rs:91-93`); pieno
  (1 voce) solo nel 06.
- 05: crisi presa fuori dall'esercizio provvisorio apposta (`genera/src/main.rs:131-132`): in
  esercizio provvisorio la caduta è certa, `disponibilita` vuota, niente `Allarga`. Qui
  `disponibilita` ha 5 voci, `alleati` 2, un evento aperto («Scandalo su un leader», tono ordinario),
  seconda legislatura (`elezioni` 2). Col motore di prima era il seme 13, turno 26.
- 02: `alleati` vuoto in trattativa; c'è già un evento aperto in tono grave (emergenza
  idrogeologica), che al turno dopo (03) torna ordinario.
- 01: `coalizione` vuota (nessuna dichiarazione al turno zero), `statistica.revisione` `null`,
  `elezioni`/`mosse`/`eventi`/`calendario` vuoti, `turni_al_voto` 0, `bilancio.presentabile`
  false nonostante sia un ottobre-dicembre.
- 07: `eredita.fine` `abbandono`, `bozza` false, `anni_al_governo` 1,75, `leggi_in_vigore` vuota
  (nessuna legge fatta). `governa` false, `fase` resta `ordinaria`.
- Numeri non arrotondati nel JSON (`5.6000000000000005`, `139.33490824282669`): l'arrotondamento
  tocca alla UI.
- Inflazione: la `Stima` porta due misure. `inflazione_annua` (prezzi sullo stesso trimestre
  dell'anno prima) è quella da titolo, come la pubblica l'ISTAT; `inflazione` è la variazione del
  trimestre annualizzata, più rumorosa. Qui l'annua va da 4,2 (turno zero) a 0,71 (07) e tocca
  -0,13 nel 05 (2035T1); la trimestrale scende a -0,32 nello 08 mentre l'annua è 1,71. L'indicatore
  `inflazione` dell'eredità (07) è l'annua: 4,2 al turno zero, 0,75 alla fine. La deflazione a
  -4,33 della prima versione era del motore di prima della #51.
- `referendum`, `esiti_referendum` vuoti in tutti; `alleati_avversari` pieno solo nel 06 (4 voci).
