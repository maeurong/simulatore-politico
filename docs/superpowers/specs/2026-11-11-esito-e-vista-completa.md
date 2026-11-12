# Piano di core «esito e Vista completa»

Spec del piano di core che sta fra il piano 10 e il piano 11 (UI «Il Settimanale»). Nasce dal ticket «Dati senza piano» della mappa del piano 11, chiuso con Mario il 10/11/2026: le schermate chiedono dati che nessun piano portava, e la UI non calcola regole (spec v1 `docs/superpowers/specs/2026-10-21-v1-giocabile.md:247-248`). Questo piano produce quei dati in `sim-core`.

Base: `origin/main` a `8952a06` (#48), poi riallineata ad `a02f6ad` (#49), che tocca solo `app/src-tauri/src/lib.rs`, e l’11/11/2026 a `94087b9` (#51, #52): main è allo schema 17, quindi le due PR lo portano a 18 e 19, e in `partita.rs` le righe dopo `:464` slittano di 2, quelle dopo `:2392` di 3. Ricerche lette prima di scrivere, entrambe sul codice a `8952a06`:

- `docs/ricerca/sintesi/fatti-della-chiusura.md`: i 135 fatti che una chiusura produce, nell'ordine di `chiudi_turno`. Qui sotto «fatto n» rimanda alla sua tabella.
- `docs/ricerca/sintesi/stato-per-le-esposizioni.md`: dove vive ogni dato da esporre.

## Perché

Oggi l'esito del turno non esiste come dato. Il core lascia frammenti con forme diverse (`mosse`, `esiti`, `esiti_referendum`, variazioni di soddisfazione e d'intesa, revisione della statistica), che durano uno o due turni, e una ventina di fatti non lascia nulla: l'atto eseguito, l'evento risolto col suo default, l'alleato che esce, il perché di un governo caduto (`fatti-della-chiusura.md:280-311`). Senza un elenco ordinato con causa e criticità, la prima pagina del Pungolo e il registro restano scheletri, e la regola «al massimo sette, prima le critiche» finirebbe nella UI.

Lo stesso vale per la mappa (due livelli su cinque), per i conti europei, per il MES e per «avanti fino a novità»: lo stato c'è, o la regola c'è, ma la Vista non li porta.

## Cosa consegna

Due PR, in quest'ordine.

1. **Logica nuova** (`feat/esito-del-turno`): l'esito del turno come elenco di voci, il registro per turno, le fermate, i due avvisi lasciati dal piano 9. Schema dei salvataggi da 17 a 18.
2. **Esposizioni** (`feat/vista-completa`): stato che il core ha già, o che calcola e butta, portato nella Vista. Schema da 18 a 19.

## Fuori da questo piano

- Il piano strutturale riveduto (storia 51): avrà un piano di core suo, dopo.
- I testi del Pungolo (titolo spiritoso, varianti, motti): ticket 09 della mappa, che parte dal catalogo dei tipi di voce di questa spec.
- Ogni schermata. Questo piano non tocca `app/src/`, salvo i tipi generati in `app/src/lib/tipi/`.
- I comandi passacarte del guscio per i metodi puri (`elettori_campione`, `menu_elettorale`, `costo_dirotta`, `anteprima_proposta`): restano al «Contratto del guscio» della mappa (ticket 12). Fa eccezione `registro`, che legge un salvataggio e sta qui.
- Il confronto fra l'ultimo sondaggio e il voto (fatto 83): non lo chiede nessuna storia.
- L'aspettativa «alzata dalle promesse»: glossario e spec v1 lo dicono (`CONTEXT.md:292-293`, spec v1 `:310`, `:379`), il motore non lo fa (`dati/base/manopole.toml:1394`). Questo piano non lo cambia e non scrive voci che citino le promesse come causa; il disaccordo resta un debito da sciogliere con Mario.

## Da evitare

- Una voce per singolo oggetto che cresce con la partita (`docs/ricerca/sistemi/feedback-visivo-dello-stato.md:46`): è il motivo della fusione delle voci gemelle.
- Ricostruire le voci a posteriori dallo stato: i fatti che si perdono sono già consumati quando la chiusura finisce. Ogni voce nasce nel punto del fatto.
- Rivelare lo stato vero: ogni esposizione rispetta l'informazione imperfetta (prime stime, un turno di ritardo, intesa avversaria in fasce).
- Regole nella UI: contare i posti del calendario, sommare costi, scegliere le sette voci.

## Decisioni di Mario

Del 10/11/2026, ticket «Dati senza piano»:

- Un solo piano di core con due PR, come sopra.
- Disoccupazione e reddito per regione: esatti, con un turno di ritardo.
- Gruppi della scheda della regione: la quota di elettori di ciascuno dei 18 gruppi, più i tre elettori campione al clic. Nessun consenso per gruppo.
- Cantieri finiti: il segno resta fino al voto successivo.
- Voto e seggi per regione: esatti, quelli dell'ultimo voto.

Del brainstorming di questo piano:

- **Criticità per tipo, tre livelli.** Ogni tipo di voce ha un livello fisso scritto in questa spec. Critica: tocca un modo di perdere o riprendere il governo (maggioranza, conti, consenso), oppure il gioco ha deciso al posto del giocatore (default), oppure ha tono grave. Notevole: riguarda il giocatore ma non mette a rischio il governo. Ordinaria: il resto. Nessuna soglia nuova: valgono quelle che il modello ha già.
- **Le sette.** Entrano prima le critiche, poi le notevoli, fino a sette. Le voci gemelle si fondono in una. L'apertura le rivela nell'ordine della chiusura, con le critiche segnate. Le critiche oltre la settima vanno nel registro, e la pagina dice quante sono.
- **La fermata sul voto.** Una misura tocca l'elettorato del giocatore quando almeno una quota fissa dei suoi elettori stimati sta nei gruppi toccati. La quota è una manopola, 30% per partire.
- **Le quattro voci minori** entrano tutte nella PR delle esposizioni: forbice del sondaggio per regione, combinazioni spente della legge elettorale, posti in calendario, costo di Ferma e Dirotta.
- **Redditi per fascia: subito, esatti.** La voce «reddito sotto l'atteso» e i redditi per fascia della Vista dicono il trimestre appena chiuso, senza il turno di ritardo delle regioni: la voce spiega il giudizio che si è mosso ora. Mario accetta che così trapeli la crescita reale esatta per fascia, e con essa un pezzo dell'inflazione vera.

Dell'11/11/2026, ticket «Testi del Pungolo» della mappa del piano 11:

- **La famiglia dell'evento nella voce.** `evento_risolto` ed `evento_nato` coprono tutti gli eventi dei dati e i titoli del Pungolo non portano nomi: la UI sceglie il titolo dal tipo e, per questi due, dalla famiglia. La voce la porta nel campo `famiglia`.

Del 12/11/2026, ticket «Anteprima della misura» della mappa del piano 11:

- **Anteprima di una misura in calendario** e **norme del decreto d'evento**: tutte e due nella PR delle esposizioni, nelle sezioni omonime qui sotto.

Dell'11/11/2026, ticket «Prima pagina» della mappa del piano 11:

- **Una situazione accesa, una voce sola.** Quando nella stessa chiusura una situazione si accende e nasce il suo evento, l'esito aveva due voci quasi uguali, tutte e due in evidenza: `situazione_accesa` e `evento_nato` («Recessione: crescita del PIL −2,9% annuo», giocato col seme 1 all'opposizione, turno 18). Tace la prima (sezione «Eventi nuovi e apertura»).

## Scelte mie, da ricontrollare leggendo

Scelte tecniche o di dettaglio prese scrivendo. Mario non le ha discusse: se una non va, si cambia qui prima dell'esecuzione.

1. **Dove vive il registro.** La partita tiene solo l'esito dell'ultima chiusura. Il registro del turno N si rilegge dal salvataggio del turno N+1 (sezione «Registro»).
2. **Testo nel core.** Ogni voce porta `fatto` e `causa` già scritti in italiano dal core, più un `tipo` enumerato. Il titolo spiritoso lo sceglierà la UI dal `tipo` e, per i due tipi d'evento, dalla `famiglia`. È lo stesso schema di `MossaIa.testo` e di `VoceEvento.causa`.
3. **`Vista.mosse` sparisce**: ogni mossa diventa una voce dell'esito con il suo partito. Nessuna schermata la legge: in `app/src`, nel guscio e nella CLI il nome compare solo nei tipi generati (`app/src/lib/tipi/Vista.ts`, `MossaIa.ts`).
4. **Il nome «criticità»**, non «livello»: nel glossario «livello» è già ciò che la mappa colora (`CONTEXT.md:381`).
5. **Precedenza fra le critiche** quando sono più di sette: tre classi (sezione «Le sette»).
6. **Fermate in più** rispetto alle cinque della spec v1: `incarico` (trattativa con incarico o guida al giocatore) e `posto_riservato` (il posto riservato del turno è suo e libero). Senza la prima il giocatore salterebbe il proprio ribaltone; senza la seconda perderebbe il turno di rotazione.
7. **La fermata sull'ultimatum e sulla crisi scatta quando nascono**, non a ogni turno in cui durano.
8. **La finestra del referendum** ferma in due casi: un confermativo su cui il giocatore può ancora scegliere, e l'ultimo turno utile dell'anno (luglio-settembre) per chiedere un abrogativo.
9. **La legge di bilancio e la sfiducia non fermano**: non hanno gruppi toccati nel modello.
10. **Storia 116.** La voce «reddito sotto l'atteso» entra nella PR 1, una per fascia di reddito, con una manopola nuova per lo scarto sotto cui tace. Il reddito reale per fascia con la crescita attesa entra nella Vista con la PR 2. Nessuno dei due era nell'elenco del ticket: li aggiungo perché l'esito senza quella voce lascia la storia 116 senza piano.
11. **Franchi tiratori contati.** `Conteggio` prende il numero esatto dei franchi tiratori: il glossario lo promette «dopo il voto» (`CONTEXT.md:141-142`) e oggi si ricava solo il saldo.
12. **Lo shock escluso dal tetto** di un evento a turno diventa una voce, come chiede la spec v1 (`:360`). Solo una voce, nessun effetto in più: il default dei cinque shock è «Non intervenire», senza effetti (`dati/base/eventi.toml:519-523`), e la serie si è già mossa nel motore.
13. **Reddito per regione** = reddito disponibile reale per elettore, in euro annui ai prezzi del turno zero. In ritardo di un turno sui redditi, pesato con le quote dei gruppi di oggi.
14. **Quote dei 18 gruppi**: quelle correnti, senza ritardo. Trapelano già dal campione degli incroci del sondaggio.
15. **Clausola dell'energia**: nazionale, non per regione. La premessa del ticket era sbagliata (`stato-per-le-esposizioni.md:119-125`).
16. **Seggi per regione**: solo quelli del Senato. Il riparto della Camera è nazionale e i seggi della Camera per regione e lista non esistono (`stato-per-le-esposizioni.md:51`). È ciò che chiede la storia 138.
17. **Proiezione del conto di controllo**: la deviazione che il prossimo controllo rileverebbe, calcolata sulla spesa netta accumulata (sezione «Regole UE»).
18. **Forbice del sondaggio** = margine dichiarato di campionamento, al massimo (p = 0,5), come lo dichiarano gli istituti veri. Non copre l'errore comune, che per decisione si rivela solo al voto.
19. **Elenco delle regioni e pericolosità naturale nella Vista**: la Vista oggi non ha un elenco delle regioni, e la pericolosità è un livello della mappa (storia 137) che sta solo nei dati. La Vista la porta come indici relativi da 0 a 1 (la regione più esposta vale 1), mai come probabilità: la mappa è nota, la probabilità per turno resta nascosta (`.scratch/v1-giocabile/issues/25-informazione-imperfetta.md:44`).
20. **Le voci di ogni turno non salgono.** La regola «il soggetto è il giocatore» non vale per i tre tipi che nascono a ogni chiusura (`sondaggio`, `revisione`, `entrate_di_cassa`): altrimenti occuperebbero due o tre dei sette posti in ogni turno.
21. **Il governo avversario che si dimette ha una voce sua**, `governo_dimissionario`. Oggi è una mossa scritta in apertura, coi suoi tre motivi; la caduta vera arriva alla chiusura dopo, con `governo_caduto`. Senza un tipo suo la riga sparirebbe, o la caduta comparirebbe in due esiti. Confermato da Mario l'11/11/2026.
22. **«Al governo» vuol dire Presidente del Consiglio.** Le colonne G e O del catalogo guardano se il giocatore presiede il governo, anche da uscente in trattativa: i suoi ministri eseguono ancora. La fermata `governo` guarda invece se può dare comandi di governo (`Vista.governa`).

## PR 1: logica nuova

### La voce dell'esito

`Vista.esito: Vec<VoceEsito>` è l'esito dell'ultima chiusura: tutte le voci, in evidenza e no, nell'ordine in cui la chiusura le ha prodotte. Al turno zero è vuoto. Sostituisce `Vista.mosse` e il tipo `MossaIa`.

Una voce porta:

| Campo | Tipo | Cosa dice |
|---|---|---|
| `tipo` | `TipoVoce` | enum senza dati, una variante per riga del catalogo sotto; in `snake_case` nel JSON |
| `criticita` | `Criticita` | `critica`, `notevole`, `ordinaria` |
| `precedenza` | `u8` | 3, 2 o 1 per le critiche (la sigla del catalogo), 0 per le altre; serve alla scelta delle sette |
| `in_evidenza` | `bool` | è una delle (al massimo) sette |
| `tono` | `Tono` | `grave` se la voce viene da un evento di tono grave, altrimenti `ordinario` |
| `fatto` | `String` | cosa è successo, col numero: l'occhiello serio |
| `causa` | `String` | perché; mai vuota |
| `partito` | `Option<String>` | id del partito di cui la voce parla |
| `dicastero` | `Option<String>` | id |
| `regione` | `Option<String>` | id |
| `misura` | `Option<u32>` | id della misura in calendario o votata |
| `famiglia` | `Option<Famiglia>` | la famiglia dell'evento (situazione, emergenza, scandalo, shock, debito), solo in `evento_risolto` ed `evento_nato`; vuota negli altri tipi |
| `dettagli` | `Vec<String>` | vuoto, salvo nelle voci fuse: una riga «fatto: causa» per ogni voce confluita |

Regole comuni:

- **Ordine.** L'elenco è nell'ordine della chiusura, cioè l'ordine dei passi di `chiudi_turno` (`crates/sim-core/src/partita.rs:546-725`). La scelta delle sette non lo cambia.
- **Dove nasce una voce.** Nel punto del fatto. Dove lo stato conserva già il fatto strutturato fino a fine chiusura (`EsitoVoto`, `EsitoReferendum`, `Variazione`, `VariazioneIntesa`, `Revisione`), la voce si scrive leggendo quello stato nello stesso passo.
- **Solo i fatti della chiusura.** Ciò che il giocatore fa durante il turno non è una voce: il «Ferma» lascia la sua variazione di soddisfazione al momento del comando, e la chiusura non la ripete. Per le variazioni di soddisfazione e d'intesa contano solo quelle nate dentro la chiusura.
- **Le mosse dell'apertura** (governo avversario che porta in aula, opposizione IA che presenta la sfiducia, emendamenti chiesti) nascono dopo che il turno è avanzato, ma stanno nello stesso esito: sono ciò che il giocatore trova aprendo il turno.
- **Mai i valori attesi in cifre** nelle voci dei partiti IA, come oggi per `MossaIa` (`crates/sim-core/src/vista.rs:180`).
- **Visibilità.** Una voce non dice ciò che la Vista nasconde: all'opposizione niente variazioni di soddisfazione dei ministri né d'intesa degli alleati del governo avversario.
- **Il soggetto è il giocatore.** Una voce il cui campo `partito` è il partito del giocatore non scende sotto `notevole`, qualunque cosa dica il catalogo. Fanno eccezione i tre tipi che nascono a ogni chiusura, `sondaggio`, `revisione` ed `entrate_di_cassa`, che restano ordinari. La regola si applica prima della fusione: una voce salita non si fonde con le gemelle rimaste ordinarie.

### Il catalogo dei tipi di voce

È l'elenco che serve al ticket 09 della mappa. Colonne: il tipo (il nome nel JSON); i fatti della ricerca che copre; cosa dice `fatto`; cosa dice `causa`; la criticità col giocatore al governo (G) e all'opposizione (O); se le gemelle si fondono (F).

Sigle: **C** critica, **N** notevole, **o** ordinaria, **–** non accade o non è visibile. Per le critiche il numero è la precedenza (3 governo e partita, 2 maggioranza e conti, 1 il resto).

**Eventi risolti**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `emergenza_lieve` | 2 | emergenza, regione, euro dal fondo di riserva e quota coperta | la pericolosità della regione per quel tipo di emergenza, a parole (alta, media, bassa); per la siccità la stagione | o | o | sì |
| `evento_risolto` | 3, 4, 5, 9, 10, 11 | evento, opzione applicata, «per default» se nessuno ha scelto, costo e chi lo paga (fondo o debito), valence guadagnata o persa; per lo scandalo: dimissioni o difesa; chiuso senza seguito se il bersaglio non c'è più | la causa dell'evento coi valori ufficiali, lo stesso testo di `VoceEvento.causa` | C1 se per default, N se scelta | N | no |
| `mes_avviato` | 6 | programma fino al trimestre X, disposizioni vincolate col loro gradino, intesa persa da ogni alleato | la scelta nelle «Aste scoperte», con spread e soglia | C2 | N | no |
| `ristrutturazione` | 7 | taglio del debito, premio sullo spread, perdita dei redditi delle fasce media e alta e per quanti turni | la scelta nelle «Aste scoperte» | C2 | N | no |
| `margine_energia` | 8 | margine UE per l'energia chiesto per l'anno | la scelta nell'evento dello shock | o | o | no |

**Default delle decisioni**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `proposta_respinta` | 12 | ministro, proposta, soddisfazione persa | in sospeso da due turni senza risposta | C1 | – | sì |
| `proposta_decaduta` | 13 | ministro, proposta, nessun costo | l'innesco che non vale più | o | – | sì |
| `emendamento_respinto` | 14 | alleato, misura, intesa persa | senza risposta prima del voto | C1 | – | sì |

**Atti dei ministri**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `atto_eseguito` | 18 | dicastero, atto, regione, euro spesi | il motivo dell'agenda, già scritto | o | o | sì |
| `atto_saltato` | 20 | atto annunciato e non eseguito | la spesa contro il margine residuo | N | o | sì |
| `cantiere_avviato` | 19 | atto, regione, rata, turni di lavori | il motivo dell'agenda | o | o | sì |
| `cantiere_fermo` | 16 | cantiere fermo un turno | la rata contro il margine residuo | N | o | sì |
| `cantiere_finito` | 17 | atto, regione, speso in tutto: gli effetti partono | la fine dei lavori dopo N turni | N | o | sì |

La rata pagata di un cantiere in corso (fatto 15) non è una voce: è un movimento continuo.

**Aula**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `leggi_slittate` | 21 | quante leggi passano al turno dopo, fiducia caduta | turno del bilancio, oppure crisi | o | o | no |
| `legge_approvata` | 28, 33 | nome, sì e no per ramo, franchi tiratori per ramo, fiducia; per la costituzionale quale deliberazione | le linee: chi ha votato sì e chi no | N | N | no |
| `legge_respinta` | 35 | come sopra, più disposizioni bloccate per N turni e valence persa dal partito del Presidente del Consiglio | le linee | C2 se è del governo, o se è una proposta dell'opposizione | N | no |
| `decreto_convertito` | 22 | nome, conteggi | le linee | o | o | no |
| `decreto_decaduto` | 23 | nome, conteggi, fondi tornati indietro, valence persa | le linee | C2 | N | no |
| `bilancio_approvato` | 24, 26 | conteggi, ministri col budget tagliato in termini reali | le linee | N | N | no |
| `bilancio_ricondotto` | 25 | crescita della spesa netta presentata, limite, quota tenuta dei budget presentati | il programma del MES è cominciato dopo la presentazione | C2 | N | no |
| `bilancio_respinto` | 27 | conteggi, valence persa, esercizio provvisorio da gennaio | le linee | C2 | N | no |
| `fiducia_respinta` | 36 | conteggi a blocchi: il governo è dimissionario | le linee | C3 | C3 | no |
| `sfiducia_approvata` | 46, 47, 98 | al governo o a un ministro; conteggi | le linee, con gli alleati in ultimatum che hanno votato sì | C3 | C3 | no |
| `sfiducia_respinta` | 46, 98 | conteggi, intesa risalita degli alleati, valence persa dal promotore | le linee | N | N | no |
| `sfiducia_scartata` | 48 | mozione tolta senza voto | governo già dimissionario, oppure dicastero in interim | o | o | no |
| `valore_rinviato` | 29 | disposizione, gradino, data d'arrivo | l'emendamento accolto | o | o | sì |
| `valore_non_applicato` | 30 | disposizione rimasta al valore di prima nonostante la legge approvata | il vincolo del memorandum del MES, fino a quando | N | o | sì |
| `misura_decaduta` | 38, 75, 101 | misura decaduta senza voto | legge elettorale non più valida (col motivo), fine della legislatura, oppure caduta del governo | N | o | sì |
| `confermativo_in_attesa` | 34 | norma costituzionale approvata senza i due terzi, data del voto, se è già richiesto | i conteggi della seconda deliberazione | N | N | no |
| `confermativo_senza_voto` | 40 | norma costituzionale in vigore senza referendum | nessuno ha chiesto il confermativo | o | o | no |
| `referendum_votato` | 39, 41, 42 | confermativo o abrogativo: sì, no, affluenza, quorum, esito, valence persa | l'affluenza contro il quorum, l'invito all'astensione | C2 se il governo perde (abrogativo approvato, confermativo bocciato), altrimenti N | N | no |
| `referendum_decaduto` | 43, 76 | referendum decaduto o sospeso senza voto | vincolo del memorandum, valore già quello del quesito, legge che non sarebbe più valida, scioglimento delle Camere | o | o | no |
| `referendum_chiesto` | 44 | promotore, disposizione, data del voto | la stima del promotore sul sondaggio: quota del sì e quorum | N | o | no |
| `firme_raccolte` | 45 | il referendum è indetto | – (la raccolta riesce sempre: «firme depositate nei termini») | N | o | no |

Norma lontana e seggi persi (fatti 31, 32) sono variazioni d'intesa e stanno in `intesa_variata`. La posizione percepita che scivola a ogni voto (fatto 37) non è una voce.

**Partiti e campagna**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `scissione` | 49 | partito, frangia nata, seggi e cassa portati via | distanza fra percepita e dichiarata, e la soglia | C2 se è il partito del giocatore o un partito di governo, altrimenti N | come G | no |
| `minaccia_di_scissione` | 50 | partito, distanza | la soglia di minaccia; se la scissione non è nata, perché (dieci partiti, frangia senza seggi) | C2 se è del giocatore, N se è di governo, altrimenti o | come G | no |
| `minaccia_rientrata` | – | partito | la distanza è tornata sotto la soglia | o | o | no |
| `promessa_verificata` | 52 | partito, promessa mantenuta o mancata, valence | la disposizione sta o non sta al gradino promesso | o | o | sì |
| `promesse_scadute` | 53 | partito, promesse tolte senza verifica | ha presieduto meno turni del minimo, o è sciolto | o | o | sì |
| `promessa_fatta` | 54 | partito, disposizione e gradino promessi | la prima voce del suo programma, o la sua posizione | o | o | sì |
| `spesa_di_campagna` | 55 | partito, euro, bersagli e leve | la cassa disponibile | N | N | sì |
| `campagna_tagliata` | 56 | euro chiesti, euro spesi, bersagli caduti | la cassa è scesa dopo la decisione: acquisti d'informazione per N euro, scissione, o tutti e due | C1 | C1 | no |

**Motore**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `mes_finito` | 57 | il programma è finito: i vincoli cadono | la durata del programma | N | o | no |
| `spread` | 59 | spread ora e variazione in punti base | – («chiusura del trimestre») | o | o | no |
| `controllo_ue` | 60 | deviazione della spesa netta, clausola applicata, conto di controllo dopo | la spesa netta dell'anno contro il tetto | N | o | no |
| `procedura_ue` | 61 | stato di prima e stato nuovo della procedura per disavanzo eccessivo | deviazione rilevante o no, disavanzo dell'anno | C2 se peggiora, N se migliora | N se peggiora, o se migliora | no |
| `ammenda_ue` | 62 | euro e % del PIL | lo stato della procedura | C2 | N | no |
| `ristrutturazione_finita` | 66 | premio rientrato e redditi tornati | – («premio sotto 1 punto base») | N | o | no |
| `reddito_deluso` | 67 | fascia, crescita reale del trimestre, crescita attesa: giudizio in calo | la media delle crescite degli ultimi N turni | N | N | sì |
| `sondaggio` | 69 | quota della coalizione del giocatore (o del suo partito) e variazione sul trimestre prima | – («sondaggio pubblico del trimestre») | o | o | no |
| `prima_stima` | 70 | PIL, inflazione, disoccupazione e disavanzo del trimestre, prima stima | – («prima stima, riveduta fra un turno») | o | o | no |
| `revisione` | 70 | per ogni grandezza la prima stima del trimestre prima e la riveduta | – («revisione della prima stima») | o | o | no |
| `situazione_accesa` | 71 | situazione, misura, valore ufficiale | la soglia di accensione | C1 | N | no |
| `situazione_spenta` | 72 | situazione, valore ufficiale | la soglia di spegnimento | N | o | no |
| `crisi_del_debito` | 73 | spread: si aprono le «Aste scoperte» | la soglia dello spread | C2 | N | no |
| `entrate_di_cassa` | 74 | partito, 2 per mille e donazioni del trimestre | i voti dell'ultima elezione | o | o | sì |

Le voci la cui causa è una frase fissa («–» più il testo fra parentesi) la portano comunque nel campo `causa`: una voce non ha mai la causa vuota.

**Voto**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `risultato_del_voto` | 78 | affluenza, quota e seggi del giocatore, primo partito, premio | – («elezioni politiche») | C3 | C3 | no |
| `incarico` | 79, 103 | a chi va l'incarico: al giocatore, al Capo dello Stato, guida | i seggi delle coalizioni, o la maggioranza contigua trovata dopo la caduta | C3 | C3 | no |
| `fine_della_partita` | 80 | il partito del giocatore è senza seggi nelle due Camere | i seggi del voto | C3 | C3 | no |
| `partito_sciolto` | 81, 82 | partito sciolto; coalizione caduta se ridotta a un solo partito | due elezioni senza seggi | N | N | sì |

**Soglie**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `soddisfazione_variata` | 23, 24, 35, 85, 86, 126 | ministro, variazione | la causa della variazione (`ministri::Causa`) | o | – | sì |
| `minaccia_di_dimissioni` | 88 | ministro sceso sotto la soglia di minaccia in questa chiusura | la causa peggiore fra le sue variazioni | C2 | – | sì |
| `ministro_esce` | 9, 47, 87, 94 | ministro, dicastero in interim | dimissioni per soddisfazione, dimissioni chieste, sfiduciato, uscita dal governo, rimpasto | C2, N se le dimissioni le ha chieste il giocatore o per rimpasto | o | sì |
| `intesa_variata` | 6, 21, 31, 32, 46, 84, 107, 108 | alleato, variazione | la causa della variazione (`CausaIntesa`) | o | – | sì |
| `ultimatum_posto` | 89, 111 | alleato in ultimatum | la causa peggiore fra le sue variazioni | C2 | C2 | no |
| `ultimatum_ritirato` | 90 | alleato | l'intesa è tornata sopra la soglia | N | N | no |

**Leader, uscite, crisi, governo**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `cambio_di_leader` | 91, 110 | partito, leader nuovo | calo di N punti al voto, oppure perdita del governo | N | N | sì |
| `frangia_esce` | 92 | la frangia lascia la maggioranza | nei sondaggi il voto le vale più del governo | C2 | C2 | no |
| `alleato_esce` | 93 | l'alleato lascia il governo, dicasteri in interim | intesa sotto la soglia d'uscita, con la causa dell'ultimatum | C2 | C2 | no |
| `rimpasto` | 94 | dicasteri che cambiano partito | l'uscita o l'allargamento che lo ha chiesto | N | o | no |
| `crisi_aperta` | 95, 99 | la coalizione non ha la maggioranza; se è una crisi riaperta | i seggi della coalizione; la sfiducia respinta, se riaperta | C3 | C3 | no |
| `coalizione_allargata` | 97, 127 | partiti entrati al governo | la crisi che ha chiesto l'allargamento | N | N | no |
| `governo_caduto` | 51, 96, 100 | il governo cade | l'origine: esercizio provvisorio, fiducia respinta, sfiducia, dimissioni del Presidente del Consiglio, crisi senza uscita, voto anticipato | C3 | C3 | no |
| `camere_sciolte` | 102, 112 | Camere sciolte, data del voto | nessuna maggioranza contigua, o nessun governo possibile in trattativa | C3 | C3 | no |
| `guida_rifiutata` | 104 | il partito più grande non cede la guida | il motivo, che il core scrive già | – | C3 | no |
| `governo_formato` | 105 | presidente, coalizione, spartizione; dopo il voto o per ribaltone | la maggioranza trovata e perché quella | C3 | C3 | no |
| `programma_dichiarato` | 106 | quante disposizioni, la prima | la nascita del governo avversario | – | N | no |
| `rottura_dell_impegno` | 108 | chi rompe con chi, valence e intesa perse | la coalizione dichiarata prima del voto | N | N | sì |

**Eventi nuovi e apertura**

| Tipo | Fatti | Il fatto dice | La causa dice | G | O | F |
|---|---|---|---|---|---|---|
| `evento_nato` | 113 | evento, regione o bersaglio, costo | la causa dell'evento coi valori ufficiali | C1 | N | no |
| `shock_assorbito` | 115 | shock escluso dal tetto di un evento a turno: la serie si è mossa, nessuno ha potuto scegliere e vale «Non intervenire» | la serie, l'ampiezza e la soglia | N | o | sì |
| `valore_in_vigore` | 116 | valore in arrivo entrato in vigore | la legge che lo aveva fissato | N | o | sì |
| `arrivo_scartato` | 117 | valore in arrivo della legge elettorale non applicato | la legge che ne uscirebbe non è valida, col motivo | o | o | sì |
| `posizione_spostata` | 119 | partito, da dove a dove | la sua regola di movimento | o | o | sì |
| `partito_di_protesta` | 120 | partito nato, posizione | elettori lontani dai partiti e astensione alta | N | N | no |
| `coalizione_ia` | 121 | chi entra in coalizione con chi, chi corre da solo | nel valore atteso ci guadagna | N | N | sì |
| `emendamento_chiesto` | 122 | alleato, misura, cosa chiede | distanza dalla sua posizione, dicastero toccato, seggi persi | N | – | sì |
| `misura_presentata` | 128, 129, 134 | chi porta in aula cosa: bilancio, voce del programma, proposta nel posto riservato | la voce del programma; per la proposta, quanti seggi di governo la voterebbero | N | N | sì |
| `proposta_accolta` | 130 | il governo avversario accoglie la proposta di un ministro | la proposta | – | o | sì |
| `emendamento_deciso` | 131 | il governo avversario accoglie o respinge un emendamento | il motivo, uno dei tre che il core scrive già | – | o | sì |
| `fiducia_posta` | 132 | il governo avversario pone la questione di fiducia su una misura | la forbice include la sconfitta | – | N | no |
| `sfiducia_presentata` | 133 | chi presenta la sfiducia, al governo o a quale ministro | la linea prevista, gli alleati in ultimatum che la voterebbero | C3 | N | no |
| `governo_dimissionario` | 127 | il governo avversario in crisi si dimette: cade alla chiusura del turno | il motivo, uno dei tre che il core scrive già | – | C3 | no |

**Situazione accesa ed evento nato.** Se nella stessa chiusura nasce l'evento di una situazione che si è accesa in quella chiusura, la voce `situazione_accesa` di quella situazione si toglie dall'esito: la causa di `evento_nato` porta già valore ufficiale e soglia. Lo fa il passo che scrive `evento_nato` (fatto 113), che sa da quale situazione viene l'evento. Se l'evento non nasce nella stessa chiusura (il tetto di un evento a turno, o una situazione senza evento), `situazione_accesa` resta. Decisione di Mario dell'11/11/2026 (ticket «Prima pagina»).

Interim e rosa dei successori, agende annunciate e proposte nate (fatti 123-125) non sono voci dell'esito: l'apertura le mostra già nelle loro schermate.

Il catalogo ha 92 tipi. Un fatto nuovo che un piano futuro aggiunge alla chiusura prende un tipo nuovo: `TipoVoce` è un'enum chiusa, e un tipo senza riga in questa tabella non compila (la criticità è un `match` esaustivo).

### Le sette

L'evidenza si compone alla fine della chiusura, sulle voci raccolte, in tre passi.

1. **Fusione.** Per ogni tipo fondibile, le voci dello stesso tipo e della stessa criticità diventano una voce sola, messa al posto della prima. `dettagli` porta una riga per ogni voce confluita, nell'ordine. `fatto` è il riassunto col conteggio («Tre ministri perdono soddisfazione»), `causa` la più frequente fra le confluite, o «più cause» se nessuna prevale. `partito`, `dicastero`, `regione` e `misura` restano solo se sono uguali in tutte. Una voce sola di un tipo fondibile resta com'è.
2. **Scelta.** Entrano le critiche. Se sono più di sette, restano le sette con la precedenza più alta (3, poi 2, poi 1); a parità, le prime nell'ordine della chiusura. Se sono meno di sette, i posti liberi vanno alle notevoli, le prime nell'ordine della chiusura. Le ordinarie non entrano mai: un turno quieto apre con meno di sette voci.
3. **Ordine.** Niente si sposta: l'elenco resta nell'ordine della chiusura, e `in_evidenza` segna le scelte.

Quante critiche sono rimaste fuori lo dice la pagina contando le voci critiche non in evidenza: è un conto, non una regola.

### Registro

- **La partita** ha un campo `esito: Vec<VoceEsito>` al posto di `mosse`: si svuota all'inizio di `chiudi_turno`, si riempie durante, si compone alla fine. È stato salvato, perché l'apertura lo deve ritrovare dopo un riavvio.
- **Il registro del turno N**, cioè le voci della chiusura del turno N, sta nel salvataggio del turno N+1, che il guscio scrive a ogni chiusura (`app/src-tauri/src/lib.rs:118-131`) e conserva per tutta la partita.
- **Il core** offre una funzione pura, `esito_da_salvataggio(json) -> Result<Vec<VoceEsito>, Errore>`, che legge dal testo del salvataggio solo la versione dello schema e l'esito, senza caricare la partita. Schema diverso: lo stesso errore di `carica`.
- **Il guscio** offre il comando `registro(turno) -> Vec<VoceEsito>`: legge il salvataggio del turno `turno + 1` dall'archivio e lo passa a quella funzione. Per un turno che la partita in memoria non ha ancora chiuso, errore, anche se sul disco quel salvataggio c'è già: dopo un guasto a metà il disco può restare un turno avanti (`app/src-tauri/src/lib.rs:114-117`).

Perché così: il salvataggio non cresce con la partita (con l'intero registro dentro, cento turni peserebbero in ogni file), il riavvolgimento resta coerente da solo (cancellare i turni dopo N cancella anche i loro registri), e i salvataggi di ogni turno ci sono già (`docs/ricerca/tecnica/persistenza-dei-salvataggi.md:79`). Il prezzo: la partita di prova, che non si salva, ha solo l'ultimo esito; e leggere molti turni di fila costa una lettura di file ciascuno.

### Fermate

`Vista.fermate: Vec<Fermata>` dice perché «avanti fino a novità» non può concatenare un'altra chiusura dal turno appena aperto. La UI ha una sola regola: continua finché la lista è vuota.

| Fermata | Quando |
|---|---|
| `governo` | il giocatore governa: ogni turno chiede le sue scelte |
| `fine` | la partita è finita |
| `voto` (con l'id della misura) | una misura in calendario col voto in questo turno tocca l'elettorato del giocatore |
| `ultimatum` (con l'id del partito) | nell'ultima chiusura un alleato del governo avversario ha posto un ultimatum |
| `crisi` | il governo avversario è entrato in crisi |
| `referendum` | c'è un confermativo su cui il giocatore può ancora scegliere, oppure è luglio-settembre, nessun referendum è in corso e il giocatore può ancora chiederne uno quest'anno |
| `campagna` | è il turno del voto: si vota in chiusura |
| `incarico` | trattativa con l'incarico, o la guida, al giocatore |
| `posto_riservato` | il posto riservato del turno è del giocatore, ed è libero |

Le prime due non sono fermate della spec v1: ci sono perché la UI non debba sapere altro che «lista vuota». Il turno di trattativa con l'incarico ai partiti IA non ferma: passa da solo, come vuole il ticket d'origine (`.scratch/v1-giocabile/issues/12-opposizione.md:27`).

**Quando una misura tocca l'elettorato del giocatore.** Si stima sul sondaggio pubblico, mai sullo stato vero.

- Gli elettori del giocatore in una condizione: campione dell'incrocio × quota del giocatore in quell'incrocio (`Sondaggio.condizioni`). Lo stesso per le regioni (`Sondaggio.regioni`).
- Una disposizione tocca le condizioni del suo elenco `gruppi`, e, se è territoriale, le regioni del suo perimetro col loro peso: la stessa definizione di `toccati` (`crates/sim-core/src/elettorato/referendum.rs:32-49`).
- Quota toccata da una disposizione = (elettori del giocatore nelle condizioni toccate / tutti i suoi elettori) × (elettori nel perimetro, pesati / tutti i suoi elettori); il secondo fattore vale 1 per una disposizione nazionale. Il sondaggio dà solo i margini, quindi i due fattori si moltiplicano.
- Quota toccata da una misura = la più alta fra le sue disposizioni.
- La misura tocca l'elettorato se la quota è almeno `fermata_quota_elettorato`, manopola nuova, 0,30.
- Un giocatore senza elettori nel sondaggio non ha fermate di questo tipo.

### I due avvisi del piano 9

Sono le voci `campagna_tagliata` e `bilancio_ricondotto`. Tutte e due vanno prese nel punto del fatto, perché i numeri oggi si scartano lì (`fatti-della-chiusura.md:338-360`).

- **Campagna tagliata.** In `spendi_campagne` il totale chiesto va sommato prima che le spese siano sovrascritte. Il motivo «acquisti» va letto prima che la chiusura azzeri gli acquisti del turno: la somma spesa in informazione nel turno si mette da parte all'inizio di `chiudi_turno`. La voce nasce solo per il partito del giocatore e solo se lo speso è minore del chiesto.
- **Bilancio ricondotto.** In `ricondotto_al_tetto_mes` crescita e limite, che `oltre_il_tetto_mes` rende già, e la quota tenuta finiscono nella voce. Vale per il governo in carica, del giocatore o avversario.

### Reddito sotto l'atteso (storia 116)

- `giudizio` calcola già, per ognuno dei 360 gruppi, la crescita reale del trimestre e l'aspettativa (`crates/sim-core/src/elettorato/giudizio.rs:82-87`). Deve renderle aggregate per fascia di reddito, su tutto il paese, pesate con gli elettori dei gruppi.
- Una voce `reddito_deluso` per ogni fascia in cui la crescita sta sotto l'attesa di almeno `scarto_reddito_deluso`, manopola nuova, 0,25 punti (crescite trimestrali, come nel motore).
- Il testo dà i due numeri e la conseguenza: «Reddito reale della fascia media +0,1% nel trimestre, atteso +0,5%: giudizio in calo».
- La voce non cita le promesse (vedi «Fuori da questo piano»).

### Altri cambi di stato della PR 1

- **Perché cade un governo.** `legislatura.dimissioni` passa da booleano a un'origine facoltativa, scritta nei sette punti in cui oggi si alza il flag (`fatti-della-chiusura.md`, fatto 100). Se due punti scattano nella stessa chiusura vale la prima origine. `Vista.dimissioni` resta un booleano.
- **Franchi tiratori.** `Conteggio` prende `franchi: u32`: i seggi che hanno votato contro la linea del loro partito, contati in `voto::conta`. Zero dove il voto non li estrae: sotto fiducia (salvo il voto finale della Camera sulla legge elettorale, che resta segreto: `crates/sim-core/src/parlamento/mod.rs:326-336`), nella sfiducia e nei voti a blocchi.
- **Shock escluso.** `estrai` rende anche gli shock usciti e scartati dal tetto, perché la chiusura ne scriva la voce. Nessun effetto si applica: il default degli shock non ne ha. Il numero e l'ordine delle estrazioni non cambiano, quindi i semi dei test restano validi.
- **`VERSIONE_SCHEMA` da 17 a 18.**

## PR 2: esposizioni

Tutto ciò che segue è nella Vista salvo dove è scritto «metodo». Dove serve stato nuovo è detto; il resto si legge da ciò che c'è.

### Regioni

`Vista.regioni: Vec<VoceRegione>`, una per regione nell'ordine dei dati.

| Campo | Cosa dice | Da dove |
|---|---|---|
| `id`, `nome`, `macroarea` | la chiave e il nome | dati (`crates/sim-core/src/struttura.rs:38-49`) |
| `pericolosita` | quattro indici da 0 a 1: `sisma`, `idro`, `incendi` e `totale` (vedi sotto) | dati (`crates/sim-core/src/struttura.rs:26-35`) |
| `disoccupazione` | %, esatta, della chiusura precedente | **stato nuovo**: una copia fatta prima che il motore la aggiorni, come `grandezze.prima` |
| `reddito` | euro annui reali per elettore, sui redditi della chiusura precedente | `Gruppo.reddito_prima` e `Gruppo.quota` |
| `gruppi` | 18 quote che sommano a 1, nell'ordine condizione × fascia | `Gruppo.quota`, corrente |

La media delle disoccupazioni regionali in ritardo, pesata sulle forze di lavoro, coincide con la disoccupazione riveduta già pubblica: è il controllo che l'esposizione non anticipa nulla.

**Gli indici di pericolosità.** I tre numeri dei dati non sono omogenei (una probabilità a trimestre, una quota nazionale, una frequenza), quindi non si mostrano grezzi. Per ogni tipo l'indice è il numero atteso di emergenze l'anno nella regione, calcolato con la formula dei candidati (`crates/sim-core/src/eventi/emergenze.rs:40-60`) sui quattro trimestri, diviso per quello della regione più esposta. `totale` fa lo stesso sulla somma dei tre tipi: è ciò che la mappa colora. La siccità non entra, perché il suo rischio è uguale in ogni regione. Dove una voce o una causa dice la pericolosità a parole, vale l'indice del tipo: alta da 2/3, media da 1/3, bassa sotto.

Altri cambi:

- `Fascia` si esporta in TypeScript: serve a chiamare `elettori_campione`.
- `Incrocio` e `Rilevazione` prendono `margine: f64`, il margine dichiarato a due deviazioni standard con p = 0,5: `2·√(effetto_disegno·0,25/campione)`. Vale per regioni, condizioni, fasce e istituti. Cambia la forma del sondaggio salvato.

### Voto per regione

`EsitoElezioni` prende `regioni: Vec<VotoRegione>`: per ogni regione l'id, i voti di ogni partito (stesso ordine di `voti`), i seggi del Senato di ogni partito, e l'id del primo partito. **Stato nuovo**: oggi `vota_parlamento` calcola voti e seggi per regione e li butta (`crates/sim-core/src/partita.rs:2810-2832`).

I seggi del Senato per regione non sommano ai seggi del partito: mancano il premio e l'estero, e vengono prima delle cessioni in coalizione. Il commento del campo lo dice. L'albo non cambia: copia in un tipo suo.

### Opere finite

`Vista.opere: Vec<VoceOpera>`: gli atti regionali eseguiti e i cantieri finiti dall'ultimo voto. Ogni voce: dicastero, atto, regione, turno, se era un cantiere, euro spesi. **Stato nuovo**, tenuto fuori da `StatoDicastero` (che `nuovo_governo` ricostruisce da zero, `crates/sim-core/src/ministri/mod.rs:207-214`) e svuotato in `vota_parlamento`. Entrano solo gli atti con una regione.

### Regole UE

`Vista.regole_ue`:

| Campo | Cosa dice |
|---|---|
| `procedura` | lo stato della procedura per disavanzo eccessivo (`StatoPde`, da esportare in TypeScript) |
| `turni_in_stato` | da quanti turni |
| `conto_controllo` | % del PIL, dopo l'ultimo controllo |
| `ultima_deviazione` | % del PIL, dell'ultimo controllo |
| `coesione_sospesa` | i fondi di coesione sono sospesi |
| `margine_energia` | l'anno coperto dal margine UE per l'energia, se chiesto e se conta |
| `proiezione` | anno, deviazione attesa e conto che ne uscirebbe |

Tutti tranne la proiezione sono il verdetto su un anno chiuso e riveduto, quindi già pubblico.

**La proiezione** è la deviazione che il prossimo controllo (chiusura di aprile-giugno) rileverebbe, con la stessa formula di `controllo_annuale` (`crates/sim-core/src/economia/regole_ue.rs:143-166`). L'anno da giudicare è quello prima del prossimo aprile-giugno da chiudere. Se il conto lo ha già intero, i numeri sono quelli: vale per un anno chiuso (si gioca gennaio-marzo o aprile-giugno) e per il 2026, che il turno zero scrive intero e che la chiusura di ottobre-dicembre 2026 non tocca (`crates/sim-core/src/economia/regole_ue.rs:92-107`, `:117-125`). Altrimenti si gioca luglio-settembre o ottobre-dicembre di un anno dopo il 2026, e l'anno in corso si annualizza sui trimestri già chiusi, due o tre (spesa netta e PIL accumulati × 4 / trimestri chiusi), contro la spesa netta dell'anno prima e il tetto dell'anno. La spesa netta è un conto pubblico, non una stima: usarla esatta non scavalca la prima stima.

### Debito

`Vista.debito`:

- `mes`: se c'è un programma, il trimestre in cui finisce e le disposizioni vincolate col loro gradino.
- `ristrutturazione`: se c'è, il trimestre del taglio, il premio di oggi sullo spread in punti base e il trimestre in cui finisce la perdita dei redditi (nessuno, se è già finita).

### Partiti

`VocePartito` prende `minaccia: bool`. La distanza è la differenza fra le due posizioni, che la Vista porta già.

### Calendario

`Vista.posti`: per i posti ordinari e per quello costituzionale, quanti sono e quanti sono occupati, contati come li conta `presenta` (`crates/sim-core/src/parlamento/mod.rs:585-602`). `posto_riservato` resta com'è.

### Legge elettorale

Metodo `menu_elettorale(valori) -> Vec<VoceMenu>`: per ogni gradino di ogni disposizione della legge elettorale, se è ammesso una volta messo nella bozza `valori` al posto del valore di quella disposizione, e altrimenti il motivo. Usa la regola che c'è, `legge_non_valida` (`crates/sim-core/src/parlamento/regole.rs:139-150`), sui valori in vigore e su quelli in arrivo. Non guarda chi governa, la crisi, il MES o il calendario: quelli li dice l'anteprima.

### Anteprima della proposta d'opposizione

Decisione di Mario dell'11/11/2026, ticket «Aula e calendario» della mappa del piano 11 (`.scratch/v1-piano-11-ui/issues/19-aula-e-calendario.md`). Oggi `Partita::anteprima` rifiuta chi non governa (`crates/sim-core/src/partita.rs:1930`): all'opposizione la proposta per il posto riservato si scrive senza costo, linee né forbice, che si vedono solo quando è già in calendario. La storia 55 chiede l'anteprima di ogni misura; la 94 vuole una proposta che costringa la maggioranza a votare contro i suoi elettori, e per sceglierla servono le linee e i gruppi toccati.

Metodo `anteprima_proposta(valori) -> Result<Anteprima, Errore>`: l'anteprima della proposta che `PresentaProposta` metterebbe nel posto riservato. Valgono le regole di quel comando tranne il posto riservato, così come `anteprima` salta il calendario pieno: la guardia dell'opposizione, poi `bozza_proposta` (`crates/sim-core/src/parlamento/mod.rs:649-666`), che rifiuta anche la norma costituzionale. Il resto è `parlamento::anteprima` come in `anteprima`, con la capacità neutra perché nessun ministro stima la proposta. La bozza porta già il promotore, quindi linee e forbice seguono le regole della proposta d'opposizione (`crates/sim-core/src/parlamento/mod.rs:295-297`). La partita non cambia.

### Anteprima di una misura in calendario

Decisione di Mario del 12/11/2026, ticket «Anteprima della misura» della mappa del piano 11 (`.scratch/v1-piano-11-ui/issues/20-anteprima-della-misura.md`). Una misura in calendario porta linee e forbice, ma non costo, effetti né gruppi toccati, e `Partita::anteprima` la rifiuta perché è già in esame (`crates/sim-core/src/parlamento/mod.rs:759-761`). Chi apre una misura in Aula non sa quanto costa né cosa fa.

`VoceMisura` prende i campi di `Anteprima` che non ha: `costo_annuo`, `effetti`, `leve`, `gruppi`, con lo stesso significato. Li calcola `parlamento::anteprima` (`crates/sim-core/src/parlamento/anteprima.rs:82-89`) sulla misura in calendario, con ciò che le passa `Partita::anteprima` (`crates/sim-core/src/partita.rs:1931-1946`): la capacità effettiva del ministro che oggi guida il dicastero proponente, neutra senza ministro e per una proposta d'opposizione, e il PIL potenziale annuo. La stima non si ricampiona di turno in turno, perché l'estrattore ha la chiave del turno di presentazione, della disposizione e del gradino. Cambia solo con la capacità: un ministro nuovo, o la fine del suo rodaggio (`crates/sim-core/src/ministri/mod.rs:125-132`). È la stima di chi siede oggi al dicastero. Nessuno stato nuovo, e `Partita::anteprima` continua a rifiutare la misura in esame.

### Norme del decreto d'evento

Decisione di Mario del 12/11/2026, stesso ticket. Il decreto innescato da un evento aperto vale solo su certe disposizioni: quelle di `norme`, o le ordinarie dei suoi `dicasteri` se `norme` è vuota (`crates/sim-core/src/eventi/mod.rs:101-106`); un evento del debito le ammette tutte (`:256-257`). `VoceEvento` non lo dice, e la UI dovrebbe provare `EmanaDecreto` su ogni disposizione per saperlo.

`VoceEvento.norme: Vec<String>`: le disposizioni (id) che il decreto innescato da quell'evento ammette, nell'ordine del catalogo. È la regola di `ammetti_disposizioni` (`crates/sim-core/src/parlamento/decreti.rs:115-139`) con quel solo evento aperto: solo norme ordinarie, ciascuna ammessa come disposizione o per il suo dicastero. Vuota se l'evento non ammette decreto. La regola resta una: il comando e la Vista usano la stessa funzione. Nessuno stato nuovo.

### Ferma e Dirotta

- `VoceAgenda.costo_ferma` e `VoceCantiere.costo_ferma`: la soddisfazione che il ministro perderebbe, con la formula di `ferma` e di `ferma_cantiere` (`crates/sim-core/src/ministri/agenda.rs:383-389`, `:492-497`).
- Metodo `costo_dirotta(dicastero, atto, regione) -> Result<f64, Errore>`: il costo di quel dirottamento, con la formula di `dirotta` (`:447-458`). Gli stessi rifiuti del comando.

I costi sono il `delta` della formula, prima del limite a 0-100 della soddisfazione. Mostrarli rende leggibile quanto il ministro tiene a ogni atto: è ciò che chiede la storia 28.

### Redditi per fascia (storia 116)

`Vista.redditi`, tre voci: fascia, reddito reale per elettore, crescita reale dell'ultimo trimestre, crescita attesa. Sono i numeri della voce `reddito_deluso`, esatti e del trimestre appena chiuso (decisione di Mario). **Stato nuovo**: l'aggregato che la PR 1 calcola in `giudizio` si conserva, perché l'attesa usata non si rifà dallo stato quando la finestra è piena (`fatti-della-chiusura.md:377`).

### Schema

`VERSIONE_SCHEMA` da 18 a 19, un solo salto per tutto lo stato nuovo della PR.

## Termini nuovi

In `CONTEXT.md`, con questa spec:

- **Voce** (dell'esito): una conseguenza della chiusura, col suo fatto e la sua causa.
- **Criticità**: quanto pesa una voce: critica, notevole, ordinaria. _Evita_: livello (che è quello della mappa), gravità, priorità.

«Fermata» è già nel glossario sul branch della mappa.

## Come si prova

Gli oracoli che il piano deve trasformare in test.

Esito:

- Al turno zero l'esito è vuoto. Dopo una chiusura ha almeno la prima stima e il sondaggio.
- Nessuna voce ha la causa vuota. Nessuna voce ordinaria è in evidenza. Mai più di sette in evidenza.
- `evento_risolto` ed `evento_nato` portano la famiglia del loro evento; nessun altro tipo la porta.
- Con nove critiche restano fuori le due di precedenza più bassa, e a parità le ultime nell'ordine della chiusura.
- Con due critiche e otto notevoli entrano le due critiche e le prime cinque notevoli.
- Dieci atti eseguiti in un turno sono una voce sola con dieci dettagli.
- Stesso seme e stessi comandi danno lo stesso esito, voce per voce.
- Ogni fatto della ricerca che il catalogo copre ha un test che lo fa accadere e trova la sua voce. I primi da scrivere sono quelli che oggi non lasciano traccia: evento risolto per default, alleato che esce, governo caduto con ognuna delle sei origini, situazione spenta, valore non applicato per il memorandum.
- Campagna tagliata: campagna pari alla cassa, poi un sondaggio privato; la voce porta il chiesto, lo speso e la somma degli acquisti. Senza taglio nessuna voce.
- Bilancio ricondotto: il caso di `il_bilancio_presentato_prima_del_mes_entra_ricondotto` (`crates/sim-core/tests/debito.rs:597-631`) produce la voce con crescita e limite.
- All'opposizione l'esito non contiene `soddisfazione_variata` né `intesa_variata`.

Registro:

- `esito_da_salvataggio` sul salvataggio del turno N+1 rende l'esito che la Vista mostrava all'apertura di N+1.
- Uno schema diverso dà l'errore di `carica`.
- Dopo un riavvolgimento al turno N, `registro(N)` è un errore finché il turno N non si richiude.

Fermate:

- Al governo la lista ha solo `governo`. A partita finita solo `fine`.
- All'opposizione, turno quieto: lista vuota.
- Una misura che tocca il 30% degli elettori stimati del giocatore ferma; al 29% no.
- L'ultimatum ferma nel turno in cui è posto e non nel successivo.

Esposizioni:

- La media pesata delle disoccupazioni regionali della Vista è uguale alla disoccupazione riveduta.
- I voti per regione sommano ai voti nazionali del partito; i seggi del Senato per regione sommano a proporzionali più speciali.
- Un cantiere finito resta in `opere` fino al voto successivo, anche attraverso un ribaltone, e poi sparisce.
- La proiezione di gennaio-marzo coincide con la deviazione che il controllo di aprile-giugno registra, se nel mezzo non cambia nulla. Al turno zero la proiezione sul 2026 non annualizza: usa l'anno già intero.
- Gli indici di pericolosità stanno fra 0 e 1, e per ogni tipo una regione vale 1.
- La voce `entrate_di_cassa` del partito del giocatore resta ordinaria e si fonde con le altre; in un turno quieto nessuna voce è in evidenza per la sola regola del soggetto.
- Uno shock escluso dal tetto lascia la voce `shock_assorbito` e nessun effetto: stesso stato, voce a parte, di una chiusura senza quello shock.
- `costo_ferma` è la variazione di soddisfazione che `Ferma` applica davvero, salvo il limite a 0-100.
- `menu_elettorale` segna non ammesso il premio alla coalizione con le coalizioni spente, col motivo.
- All'opposizione `anteprima_proposta` rende le linee e la forbice della misura che `PresentaProposta` mette in calendario nello stesso turno, e la partita non cambia.
- `anteprima_proposta` rifiuta al governo e con una norma costituzionale, come `PresentaProposta`; col posto riservato occupato non rifiuta.
- Una legge appena presentata ha in `VoceMisura` costo, effetti, leve e gruppi uguali a quelli dell'anteprima della stessa bozza nello stesso turno; al turno dopo, con lo stesso ministro fuori dal rodaggio, gli effetti non cambiano.
- Con un solo evento aperto, ogni disposizione di `VoceEvento.norme` passa `ammetti_disposizioni`, e ogni disposizione ordinaria fuori dalla lista è rifiutata come «decreto senza innesco». Un evento senza decreto ha la lista vuota.
- Un salvataggio con lo schema 17 è rifiutato con `SchemaDiverso`.

## Riferimenti

- `docs/ricerca/sintesi/fatti-della-chiusura.md`: l'inventario; `:280-311` i fatti senza traccia, `:338-360` i due avvisi, `:362-385` la storia 116.
- `docs/ricerca/sintesi/stato-per-le-esposizioni.md`: `:147-171` la tabella dello stato per voce.
- `docs/ricerca/sistemi/feedback-visivo-dello-stato.md:36`, `:46`: gerarchia delle notifiche per impatto, niente notifiche per singolo oggetto.
- `docs/ricerca/tecnica/persistenza-dei-salvataggi.md:78-85`: peso dei salvataggi.
- `.scratch/v1-giocabile/issues/09-giro-del-turno.md:35`: l'esito nel ticket d'origine.
- `.scratch/v1-giocabile/issues/12-opposizione.md:27`: «avanti fino a novità» nel ticket d'origine.
