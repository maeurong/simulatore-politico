Type: grilling
Status: resolved
Blocked by: 02, 03
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Quali blocchi del modello economico entrano nella v1 (contabilità SFC, IS-Phillips senza Taylor perché i tassi sono della BCE, finanza pubblica, Leontief), e i settori produttivi ci sono (quanti) o il PIL resta aggregato? Punto di partenza: docs/ricerca/sintesi/motore-economico-raccomandato.md. Da correggere: docs/ricerca/modelli/banca-centrale-e-inflazione.md:39 presuppone una banca centrale nazionale.

Ingresso da Elettorato (06): il modello deve esporre disoccupazione e reddito disponibile per fascia di reddito (bassa, media, alta) e, dove possibile, per regione; sono gli indicatori degli interessi dei gruppi e muovono ogni turno le quote fra gruppi.

## Answer

Grilling con Mario del 17/10/2026, quattro giri, tutte le raccomandazioni accettate; una precisazione di Mario: ogni voce di spesa obbligatoria deve poter cambiare **regime**, non solo parametri (esempio: dal contributivo al retributivo). Termini nuovi in `CONTEXT.md`: spesa obbligatoria, regime, spread, PIL potenziale; settore produttivo precisato. Numeri per l'Italia in `docs/ricerca/modelli/economia-italiana-parametri.md` (ricerca lanciata durante il grilling).

Scelte fatte senza chiedere, perché non cambiano il prodotto: SFC a 4 settori (famiglie, imprese, Stato, estero) senza banche, con il controllo «la somma fa zero»; niente modello gravitazionale, l'estero è domanda esogena con shock ed elasticità dell'export ai prezzi relativi; l'impulso fiscale entra nella IS come termine proprio con moltiplicatore dalla letteratura; la spesa di ogni dicastero va in domanda finale con un vettore di settori fisso; i fondi di coesione sono un'entrata esogena, sospesa dalle sanzioni della PDE.

**Produzione.** Leontief nazionale a **10 settori** (NACE A*10), calibrato su FIGARO aggregato; matrice A fissa, cambiata solo dagli eventi; modello dei prezzi per l'inflazione importata. Ogni regione ha il suo mix di settori: il suo gap è la media dei gap di settore pesata col mix, così uno shock energetico colpisce le regioni in modo diverso senza regole scritte a mano.

**Congiuntura.** IS-Phillips di Rudebusch-Svensson trimestrale, backward-looking, coefficienti ricalibrati su Italia/area euro dove esistono (default RS, `docs/ricerca/modelli/macro-neokeynesiano-piccolo.md`). Ciclo da shock stocastici (domanda estera, energia, tasso BCE, produttività) estratti con **seme fisso** per partita, ampiezza dai residui storici italiani.

**BCE.** Fuori dal controllo del giocatore: regola di Taylor sull'area euro, in cui l'Italia pesa per la sua quota di PIL e il resto dell'area è esogeno con shock. Niente credibilità monetaria propria; aspettative d'inflazione ancorate al 2% con peso fisso.

**Spread e crisi del debito.** Spread **non lineare**: piatto finché debito, disavanzo e credibilità del MEF reggono, convesso oltre una soglia. Il **TPI** azzera il premio di sfiducia (non la parte da fondamentali) solo se valgono i suoi quattro criteri: una PDE con azione efficace riconosciuta li rispetta, un giudizio di «nessuna azione efficace» toglie la rete (`docs/ricerca/modelli/economia-italiana-parametri.md:44`, forma proposta a :117-119: quadratica di De Grauwe-Ji più premio di sfiducia legato agli eventi). Il tasso medio sul debito si aggiorna sulla quota che scade. Oltre una soglia le aste vanno deserte e il giocatore sceglie in un turno: **MES** con condizionalità (tetti di spesa imposti, misure obbligate, intesa degli alleati in calo), che apre l'**OMT** e schiaccia lo spread; oppure **ristrutturazione** (taglio del debito, perdite di reddito per i gruppi che detengono titoli, rating a terra con memoria lunga, fuori dal TPI). Uscita dall'euro fuori scope.

**Regole UE.** Forma del ticket 02 accettata: `tetto_spesa_netta[anno]`, `conto_controllo`, PDE come macchina a stati, clausole difesa ed energia, controllo annuale nel turno di primavera con proiezione visibile negli altri. Partenza: PDE sospesa non chiusa, tetti 1,6 / 1,9 / 1,7 / 1,5 per il 2026-2029. Il piano strutturale riveduto è una scelta del primo anno di un governo nuovo.

**Lavoro.** Okun **per regione**, con u* propria e coefficiente per macroarea dove la stima esiste. u* ha **isteresi** (sale dopo recessioni lunghe, scende piano) e la spostano con ritardo le riforme del lavoro e le politiche attive di Lavoro e Welfare. L'immigrazione resta nella demografia, non tocca u* nella v1.

**Reddito e prezzi.** Reddito disponibile calcolato per i **360 gruppi**: reddito primario di condizione e regione (salario privato dal mix settoriale, salario pubblico, reddito autonomo, pensione, sussidio) × moltiplicatore di fascia − imposte + trasferimenti. Niente 10 decili: un solo sistema di classi di reddito, quello dell'elettorato; Gini derivato dai gruppi con dispersione fissa interna. I prezzi dei 10 settori si combinano in **3 panieri**, uno per fascia: il reddito reale di ogni gruppo si deflaziona col suo.

**Fisco.** Nella legge di bilancio quattro imposte: IRPEF con i suoi scaglioni reali, IVA ordinaria, IRES, aliquota contributiva, ognuna con la sua elasticità del reddito imponibile (Laffer da ETI). Detrazioni, agevolazioni e regimi speciali sono leggi di riforma del catalogo, non manopole del bilancio.

**Spesa dei dicasteri.** Ogni dicastero ha una quota fissa di **salari** e una di **acquisti** nella spesa di funzionamento: tagliare i salari significa blocco del turnover (meno dipendenti pubblici, piano) e rinnovi sotto l'inflazione; tagliare gli acquisti pesa sul servizio e sui settori fornitori.

**Spesa obbligatoria.** Fuori dai dieci dicasteri: pensioni, sussidi (stabilizzatori automatici), interessi. Ogni voce ha un **regime** da un piccolo menu più i suoi parametri; una legge cambia regime o parametri, e una legge di riforma può istituire voci nuove (diritto permanente) che pesano ogni anno sul tetto di spesa netta.
- Pensioni: retributivo, contributivo (in vigore), quota base universale; parametri età, indicizzazione, aliquota contributiva, pensione minima.
- Sussidi: assicurativo (tipo NASpI) o universale; parametri importo, durata, soglia.
- Un cambio di regime vale solo per le **coorti future, pro rata** (come la riforma Dini del 1995); unica leva retroattiva l'indicizzazione delle pensioni in essere, che la giurisprudenza costituzionale limita (sent. 70/2015, da verificare).
- La colpa elettorale di pensioni e sussidi va al partito che regge Lavoro e Welfare, oltre alla quota fissa del Presidente del Consiglio (06).

**Pensioni e demografia.** Ripartizione su una piramide nazionale per coorti (ISTAT), che alimenta anche il passaggio annuale dei gruppi da lavoratori a pensionati (06).

**PIL potenziale.** Trend spostato poco e con ritardi di anni da infrastrutture, capitale umano, durata dei processi, prezzo dell'energia e demografia. Pesi provvisori al bilanciamento.

**Ordine di aggiornamento nel turno** (da `docs/ricerca/sintesi/motore-economico-raccomandato.md:37`, adattato): misure → finanza pubblica e spesa obbligatoria → domanda finale per settore → Leontief (produzione e prezzi) → IS/Phillips → Okun per regione → redditi dei gruppi e panieri → tasso BCE → SFC (stock e controllo di chiusura) → spread, rating, regole UE.

Effetti sulla mappa: graduano dalla nebbia «Crisi ed eventi», «Catalogo degli atti e delle leggi di riforma» (con i regimi di spesa obbligatoria) e «Tabella degli interessi»; nei punti di aggancio entrano banche e crisi bancaria, dazi ed embarghi, uscita dall'euro, immigrazione su u*. Corretta `docs/ricerca/modelli/banca-centrale-e-inflazione.md:39-41`.
