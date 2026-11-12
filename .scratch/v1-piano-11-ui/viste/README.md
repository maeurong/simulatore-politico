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
| `13-edicola.json` | 1-4, 9 | varie | — | 25496 | `partite()` e `albo()` del guscio rifatti con `testata()` e `voce_albo()`: cinque partite in corso coi nomi diversi, tre abbandonate nell'albo (vedi sotto) |
| `14-editor.json` | — | — | — | 204849 | `riepilogo()` di `sim-core::riepilogo` su `con_su_misura()`: intervalli dei sei cursori, leggi elettorali coi nove parametri, 46 disposizioni coi gradini (35 ordinarie, 2 costituzionali, 9 elettorali), otto scenari di prova e nove passi per cursore (vedi sotto) |
| `17-cruscotto.json` | 4, 1, 14 | 4,5 | varie | 673154 | stati del cruscotto al governo che mancavano, ciascuno con la sua `Vista` e il `come`: `ultimatum`, `esercizio_provvisorio`, `crisi_provvisorio` cercati giocando come il 05 (gli ultimi due senza mai presentare il bilancio), `dimissioni` e `scelte_fatte` dalla 03, più `anteprima_accogli_03` (vedi sotto) |
| `15-fondazione.json` | 4, 1, 9 | — | — | 32675 | `Partita::anteprima_fondazione` sull'Italia di oggi coi semi 4, 1, 9 e sull'«Elettorato a destra» (orientamento +1) col seme 4, più i partiti dello scenario coi posti (vedi sotto) |
| `19-aula.json` | 4, 2 | 4,5 / 1,5 | 2-4 | 838325 | stati dell'Aula che mancavano, ciascuno con `come` e `vista`: `piena` (il 03 con `PresentaLegge` della legge elettorale «proporzionale al 3%», della costituzionale più votata e della riforma più lontana dalla destra), `dopo_voto`, `seconda_deliberazione`, `dopo_elettorale` (chiusure successive, bilancio presentato), `opposizione_libero` e `opposizione_proposta` (seme 2 a 1,5, `PresentaProposta` e `Linea`), più `anteprima_elettorale`, `anteprima_riforma` e due rifiuti veri (vedi sotto) |
| `16-prima-pagina.json` | 4, 9, 1 | 4,5 / 1,5 | varie | 653911 | stati della prima pagina che mancavano, ciascuno con `come` e `vista`: `t3` (il 03 chiuso una volta), `zero_9` e `voto_9` (seme 9 prima e dopo il voto), `voto_opp` (seme 1 a 1,5 dopo il voto), e le corse di «avanti fino a novità» dal 06 in `corse`, con `prima` e `vista` di tre fermate: `fermata_4` (primo ultimatum), `fermata_19` (la corsa più lunga, 3 chiusure), `fermata_20` (campagna) (vedi sotto) |
| `20-anteprima.json` | 4, 7 | 4,5 | 2, 8 | 301982 | anteprime sul 03, ciascuna con `valori` e `anteprima` o `rifiuto`: riforma, tre territoriali (decontribuzione rifiutata perché già in esame, ZES, autonomia), due disposizioni, costituzionale, elettorale, proposta sul nucleare, `accogli`; `gradini` con ogni gradino di salario minimo, ZES, autonomia e numero dei parlamentari; `test` comprato su salario minimo e ZES con cassa prima e dopo e il rifiuto del secondo; `decreto` e `decreto_grave`, primo decreto su disposizioni ammesso da un evento aperto, con la sua Vista (vedi sotto) |
| `21-conti.json` | 4, 1, 25, 11 | 4,5 / 1,5 | 2-25 | 441110 | stati dei Conti, ciascuno con `come`, `cassa_prima`, `cassa_dopo`, `rifiuto_ripetuto`, `serie` e `vista` dopo `Compra { CentroStudi }`: `studi_03` (il 03), `studi_06` (il 06), `due_accese` (seme 4 turno 15), `vicina` (seme 1 turno 13, recessione al primo trimestre sotto), `spread_alto` (seme 25 turno 16, 205 pb), `senza_bilancio` (seme 11 turno 25, bilancio mai presentato) (vedi sotto) |
| `22-bilancio.json` | 4, 1 | 4,5 / 1,5 | 2-13 | 464346 | stati del Bilancio, ciascuno con `come`, `serie` e `vista`: `bozza_04` (il 03 chiuso due volte, prima di `PresentaBilancio`) con `bozza`, `riepilogo_bozza` e i `varianti` di `riepilogo_bilancio` (indicizza tutto, soglie, ogni dicastero da solo, Iva al 24%, primo scaglione al 21%, margine della Sanità dal funzionamento o in disavanzo, Iva al 95% rifiutata); `presentato` con `riepilogo_presentato` e `se_accogli` per alleato; `emendamenti_decisi` (Centrodestra accolto, Destra respinta); `ripresentato` (indicizzato per tutto); `provvisorio` (seme 1 turno 13, bilancio mai presentato); `opposizione` (il 06 fino a ottobre-dicembre, con il rifiuto di `riepilogo_bilancio`); `fuori_stagione` (la bozza del 04 sul 03, rifiutata) (vedi sotto) |
| `23-paese.json` | 4, 1 | 4,5 / 1,5 | 0, 2, 7 | 648759 | stati della sezione Paese: `governo` (il 03) e `opposizione` (il 06), ciascuno con `come`, `cassa_prima`, `cassa_dopo`, `rifiuto`, `rifiuto_ripetuto` e `vista` dopo `Compra { SondaggioPrivato }`, più `campione` (`elettori_campione` dei 360 gruppi a chiave `regione\|condizione\|fascia`), `regioni` (20 `VoceRegione` della spec di core, calcolate in `genera/`) ed `elezioni_regioni` (`null`); `zero` (seme 4 a 4,5, turno zero) con `campione` e `regioni`; `grave` (lo stato dell'08) con `regioni`; `effetto_disegno` della manopola (2) per il margine (vedi sotto) |

Semi 1-3 a 4,5: niente incarico al giocatore dopo il voto, scartati. 03 è il primo turno al
governo, fuori dal bilancio, con un evento aperto e almeno una `ministeri[].proposta` (qui 3).

## Stranezze notate

- 23: aggiunta dal ticket «Paese». `regioni` segue la tabella `VoceRegione` della spec di core «esito e Vista completa»
  (PR 2): disoccupazione vera della regione presa dalla partita al turno prima, reddito reale per elettore da
  `reddito_prima` pesato sulle quote, 18 quote condizione × fascia (sommano a 1 entro 2·10⁻¹⁶). Il controllo della spec
  tiene: la media pesata sulle forze di lavoro vale 5,693 nel 03 contro la riveduta pubblica 5,6925, 5,440 contro 5,4398
  nel 06, 5,791 contro 5,7915 nell'08. Al turno zero non c'è una chiusura prima: disoccupazione e reddito sono quelli di
  partenza (`reddito_prima` vale `reddito`). Nell'indice `totale` il sisma pesa poco, perché le sue probabilità a
  trimestre sono piccole accanto alle emergenze idrogeologiche e agli incendi: l'Abruzzo ha `sisma` 1 e `totale` 0,26;
  la regione più esposta è la Sicilia (incendi al tetto, 3,5 × 0,35 tagliato a 1), poi la Sardegna (sisma 0, totale
  0,81). Il campione è uguale prima e dopo l'acquisto: legge solo il sondaggio pubblico (`genera/` lo controlla). Il
  sondaggio privato costa 15.000 euro e ha 4.000 interviste, contro le 4.501 del pubblico sommate sulle regioni.
  `elezioni_regioni` è `null`: il voto parte dalle `intenzioni_vere()` prese dentro la chiusura dopo il motore e le
  consuma lì; prima e dopo la chiusura le intenzioni sono altre (la destra vale 0,259 prima, 0,227 dopo, 0,249 al voto,
  seme 4), e `voti_per_regione` e `riparto_con` sono `pub(crate)`. `cantiere_finito` NON RAGGIUNTO: le agende dei
  ministri si vedono solo al governo (02 e 06: nessuna agenda su 10 ministeri, cantieri visibili), quindi gli atti a un
  turno eseguiti nella chiusura della trattativa, dopo il voto, non si ricostruiscono dalle Viste. Rimandi al core.

- 22: aggiunta dal ticket «Bilancio». La perdita reale è uguale per tutti i dicasteri (1,0% nel 2027T4). Spostare
  il margine dal funzionamento lascia il riepilogo identico alla bozza. Ripresentando indicizzato per tutto dopo aver
  respinto la Destra, la sua intesa persa resta. Nell'esercizio provvisorio del seme 1 la bozza sta sul tetto (+1,49%
  contro 1,5%). La Vista non porta le cifre del bilancio presentato né i dicasteri di un emendamento: rimandi al core.

- 21: aggiunta dal ticket «Conti». La recessione è spenta con la crescita a −4,2% (seme 1, turno 13): è il primo trimestre
  sotto il −1% e ne servono due; la Vista non porta il conto. La perdita reale è uguale per tutti i dicasteri e cresce anche
  presentando la bozza a ogni bilancio (4,1% al seme 4, turno 15). Il debito sul PIL la Vista non lo porta: lo dice solo la
  previsione del centro studi (137% nel 03). Nel 06, all'opposizione, la perdita reale è negativa (deflazione).

- 20: aggiunta dal ticket «Anteprima della misura». `anteprima` rifiuta una misura già in esame («Sgravio contributivo nel
  Mezzogiorno è già in esame»: il Centro l'ha nel posto riservato), mentre il test della stessa bozza si compra. Una legge su
  due dicasteri è un rifiuto («devono stare nello stesso dicastero»). La fascia stimata non è monotona: salario minimo 7 €/h
  medio, 9 molto, 11 medio. Il primo decreto d'evento ammesso è sul codice degli appalti già al turno 2 del seme 4
  (emergenza idrogeologica); in tono grave al seme 7, turno 8. La Vista non dice quali norme un evento ammette:
  `genera/` le trova provando `anteprima_decreto`.

- 16: aggiunta dal ticket «Prima pagina». Le fermate sono calcolate in `genera/` con le regole della spec di core
  «esito e Vista completa» (sezione «Fermate»), tranne `voto`, che chiede la quota dei suoi elettori toccati: la porterà
  il core. All'opposizione la corsa si ferma quasi a ogni turno (posto riservato libero, finestra del referendum di
  luglio-settembre, ultimatum agli alleati del governo): la più lunga in 24 turni è 16→19. Nella chiusura del turno 18
  la Recessione si accende e il suo evento nasce insieme: due voci uguali, rimandate al core.

- 19: aggiunta dal ticket «Aula e calendario». Due rifiuti veri: una seconda costituzionale con quella in esame dà
  «calendario pieno»; `anteprima` all'opposizione dà «non governi: sei all'opposizione» (niente anteprima della proposta).
  La costituzionale entra con la prima deliberazione già votata a fine del turno di presentazione; i 300 e 150 passano alla
  prima (219 sì, sotto i due terzi) e cadono alla seconda: il referendum confermativo non arriva giocando (`referendum`:
  NON RAGGIUNTO). La legge elettorale approvata scrive «L. 1/2027» sulle disposizioni che cambia.

- 17: aggiunta dal ticket «Cruscotto al governo». Ultimatum ed esercizio provvisorio arrivano tutti e due al seme 1,
  turno 13 (2030T1), nella seconda legislatura (alleati sinistra, centrosinistra, centro); la crisi in esercizio
  provvisorio al seme 14, turno 37, con cinque ministeri in interim e `bilancio.presentabile` vero. `scelte_fatte` è la
  03 con `Rinvia` su Ambiente (`rinviata` diventa vera subito) e `RispondiEvento` «debito» (`scelta` 1). Il fondo di
  riserva rimasto è 0 nella 03 e nella 04, mentre il default degli eventi è «attingere al fondo».

- 15: aggiunta dal ticket «Fondazione». Un'anteprima costa circa 43 ms in release. Il sondaggio è rumoroso: a 1,5
  la quota attesa va dal 10,9% (seme 4) al 14,5% (seme 9), al posto medio 4,5 dal 19,2% al 23,3%. Il Centro non è mai
  disponibile, nemmeno a mezzo punto, con la causa «posizione troppo lontana dalla tua»: `Distanza` è il ripiego quando
  nessun termine spinge contro (`crates/sim-core/src/legislatura/coalizione.rs:326`).

- 14: aggiunta dal ticket «Scenario ed editor». Nessun comando del guscio porta ciò che qui sta sopra i riepiloghi
  (intervalli, leggi, disposizioni): `genera/` lo legge da `DatiGioco`. Scenari salvati: quattro delle prove, coi loro nomi.
  Un riepilogo costa circa 60-70 ms in release. Lungo i cursori, uno per volta: l'inflazione di partenza non muove
  niente del riepilogo; disoccupazione, età e ricchezza muovono solo il disavanzo; il debito muove spread e interessi
  (spread fermo a 60 fino a 135%, poi 586 a 160%, crisi del debito sopra 550); solo l'orientamento muove consenso e
  maggioranze. «Caro energia» è una situazione accesa anche nello scenario di base, quindi quell'avviso c'è sempre.
  Il regime delle pensioni ha saldo zero su ogni gradino: il suo effetto passa dalle leve, non dal saldo.

- 13: aggiunta dal ticket «Edicola». `ultimo_salvataggio` a orari fissi (dal 28/10 al 6/11/2026). **Forzati**: lo
  schema 16 di «Unione dei Pendolari» (col `motivo` che il guscio scriverebbe) e la `finita` di «Lista del Faro»,
  perché la fine senza seggi in tutte e due le Camere (`partita.rs:2891-2893`) non arriva coi semi 1-60 a 6,5.
  Nel margine dell'albo il turno zero vale −201 / −101: prima del voto le Camere della fondazione non hanno seggi
  della coalizione.

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
