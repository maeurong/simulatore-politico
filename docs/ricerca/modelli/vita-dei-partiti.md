---
tipo: modello
fonti: [fonti/laver-schilperoord-2007-nascita-partiti.md, fonti/laver-sergenti-netlogo-cap7.md, fonti/tavits-2006-ingresso-nuovi-partiti-abstract.md, fonti/tavits-2008-nuovi-partiti-abstract.md, fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md, fonti/istat-annuario-2013-cap10-elezioni.md, fonti/camera-dossier-dl149-2013-finanziamento-partiti.md, fonti/normattiva-dl149-2013-art10.md, fonti/ibenskas-sikk-cambiamento-partiti-cee.md, fonti/ibenskas-2016-fusioni-partiti.md, fonti/lse-ceron-2013-fazioni-scissioni.md, fonti/camera-sparql-scissioni-gruppi.md, fonti/ilpost-2017-10-04-mdp-esce-maggioranza.md, fonti/skytg24-2021-01-13-italia-viva-ritira-ministre.md, fonti/ilpost-2022-06-22-scissione-m5s.md, fonti/lapresse-2026-02-06-futuro-nazionale-deputati.md, fonti/open-2026-06-06-futuro-nazionale-ingressi.md, fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md, fonti/parlgov-view-election-csv.md, fonti/ches-trend-1999-2024-csv.md, fonti/adams-2004-cambio-posizioni-abstract.md]
aggiornato: 2026-10-29
---
# Vita dei partiti: nascita, scissione, scioglimento, movimento

Evidenze e parametri per le regole della spec v1 sulla vita dei partiti (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:350-351` e `:355`): partito di protesta, scissione, scioglimento, tetto al numero dei partiti, regola e passo di movimento della posizione dichiarata. Per ogni regola la pagina dice che cosa la regge, che cosa la contraddice e quale valore si può proporre. Le scelte restano a chi decide il gioco: qui ci sono proposte, non decisioni.

Nota sull'asse: nello scenario le posizioni vanno da 1.0 a 7.0 (`dati/base/scenario.toml:223` e `:236`, dalla radice del repo), e la mappa dai dati CHES (`lrgen` 0-10) ai sette posti è un troncamento con classi di larghezza diversa: i posti 3, 4 e 5 valgono un punto CHES ciascuno, gli altri due ([[sondaggi-ottobre-2026]], `docs/ricerca/dati/sondaggi-ottobre-2026.md:26`) [V]. Mezzo punto di gioco vale quindi da 0,5 a 1 punto CHES secondo il posto [INF].

## Nascita di partiti nuovi e di protesta

### Il modello Laver-Schilperoord-Sergenti

- Un cittadino diventa leader di un nuovo partito con probabilità proporzionale alla propria insoddisfazione relativa, cioè alla distanza dal partito più vicino aggiornata nel tempo: `p_c = β · D*_ct / media(D*_ct)` (fonti/laver-schilperoord-2007-nascita-partiti.md:116-119) [V].
- La soglia di nascita coincide con quella di sopravvivenza: un cittadino non fonda un partito che non raggiungerebbe la soglia `T_s` (fonti/laver-schilperoord-2007-nascita-partiti.md:112) [V]. Esperimenti con `α = 0,5`, `β = 0,0001`, `T_t = 10` tick (fonti/laver-schilperoord-2007-nascita-partiti.md:130) [V].
- Soglia e numero di partiti a regime: 20% del voto dà 3-4 partiti, 15% ne dà 4-5, 10% ne dà 6-7, 7,5% ne dà 8-10, 5% ne dà 12-15. Gli autori ne ricavano che un sistema reale con sei o sette partiti ha una soglia di sopravvivenza di fatto intorno al 10% (fonti/laver-schilperoord-2007-nascita-partiti.md:132) [V].
- Dove nascono: con partiti Hunter le nascite avvengono alla periferia dello spazio, e i nuovi partiti poi si spostano verso il centro; con partiti Aggregator nascono verso il centro (fonti/laver-schilperoord-2007-nascita-partiti.md:166, :172) [V].
- Nel codice NetLogo del cap. 7 di Laver e Sergenti a ogni elezione si estrae **un solo** punto dello spazio, che genera un partito con probabilità `misery-beta · misery · 1000`: al massimo una nascita per elezione (fonti/laver-sergenti-netlogo-cap7.md:394-397) [V]. Il nuovo partito nasce nel punto degli elettori scontenti con una regola estratta a caso (fonti/laver-sergenti-netlogo-cap7.md:1004) [V] e con una fitness iniziale pari a `1 / numero di partiti` (fonti/laver-sergenti-netlogo-cap7.md:412) [V].
- Valori degli esperimenti nel codice: memoria dell'insoddisfazione 0,5, sensibilità della nascita 0,9, memoria della fitness 0,5, soglia di sopravvivenza 0,1 della quota di voto, 20 tick di campagna per elezione (fonti/laver-sergenti-netlogo-cap7.md:1351-1355) [V].

### Evidenza empirica

- Tavits 2006, 22 paesi OCSE 1960-2002: i nuovi partiti sono più frequenti quando le istituzioni sono permissive (costo d'ingresso basso), quando il corporativismo è debole (beneficio alto) e quando i partiti in carica hanno storie elettorali brevi (fonti/tavits-2006-ingresso-nuovi-partiti-abstract.md:15) [V, solo abstract].
- Tavits 2008, 15 democrazie dell'Est 1990-2004: l'ingresso è più probabile con costi bassi, benefici alti e viabilità percepita alta; il **voto** ai nuovi partiti dipende dalla delusione verso i partiti esistenti (fonti/tavits-2008-nuovi-partiti-abstract.md:18) [V, solo abstract].
- Hug 2001 (22 democrazie sviluppate): i temi nuovi o trascurati spingono la nascita, filtrati da accesso alle schede, finanziamento pubblico e sistema elettorale [INF: solo descrizione dell'editore nei risultati di ricerca; testo non accessibile].
- Emanuele e Sikk 2018, 20 paesi dell'Europa occidentale, 344 elezioni dal 1945: i partiti «genuinamente nuovi» superano l'1% in 98 elezioni; la loro somma arriva al 3% in 41 e al 10% in 10 (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:199-201) [V]. L'Italia è il paese dove pesano di più: 17,5% di media nell'ultimo decennio (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:186-187) [V].
- Traiettorie dopo il debutto (127 partiti): «flop» 53,5%, che entrano col 2,4% medio e scompaiono di solito dopo la seconda o la terza elezione; «contender» 20,5%; «flat» 13,4%; «explosive» 11%; «meteoric» 1,6% (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:506-512, :526-528) [V]. M5S è classificato «explosive»: 25,6% al debutto, 32,7% alla seconda elezione (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:482) [V].
- Italia 2013: affluenza alla Camera 72,2% contro 78,1% nel 2008 (circoscrizione Estero inclusa) (fonti/istat-annuario-2013-cap10-elezioni.md:30-32) [V]. Il nuovo partito più votato nasce quindi in un'elezione con astensione in forte crescita, ma la crescita dell'astensione non è stata riassorbita [INF].
- Dote: un partito nuovo che nasce fuori dal parlamento non accede al 2 per mille. Serve almeno un candidato eletto col proprio simbolo all'ultima elezione di Camera, Senato o Parlamento europeo (fonti/normattiva-dl149-2013-art10.md:15; già nel dossier del 2013, fonti/camera-dossier-dl149-2013-finanziamento-partiti.md:606) [V], oppure un gruppo parlamentare o una componente del Misto che dichiari di farvi riferimento (fonti/normattiva-dl149-2013-art10.md:16-19) [V]. Il partito di protesta parte quindi senza fondi pubblici e la sua dote viene da fondatori e privati [INF]; ammontare tipico [NON TROVATO]. Ordini di grandezza della cassa in [[finanziamento-dei-partiti]].

## Scissioni

### Cause

- Laver-Schilperoord rinviano esplicitamente scissioni e fusioni a un modello di contrattazione interna non ancora scritto (fonti/laver-schilperoord-2007-nascita-partiti.md:116) [V]: le estensioni Laver-Sergenti non danno regole per le scissioni [INF].
- Ceron (fazioni dei partiti italiani 1946-2011, sintesi dell'autore): la scissione dipende dalla forza della minaccia della minoranza e dalla disponibilità del leader al compromesso; queste a loro volta dipendono da distribuzione di cariche, politiche e voti, lealtà, regole interne, sistema elettorale (le liste bloccate favoriscono le uscite) e competitività del sistema. Se l'unità serve, il leader compra la minoranza; se serve meno, la lascia andare (fonti/lse-ceron-2013-fazioni-scissioni.md:27-29, :33) [V, blog dell'autore; l'articolo BJPS non è scaricabile].
- Ibenskas e Sikk, 11 paesi dell'Europa centro-orientale 1990-2015: 154 scissioni, nel 19% delle coppie partito-legislatura; i partiti **più grandi** si scindono più spesso; servono altre variabili, come i disaccordi interni sulle politiche (fonti/ibenskas-sikk-cambiamento-partiti-cee.md:304-305, :586-594) [V]. Il dato riguarda democrazie giovani, dove le scissioni sono più frequenti che in Occidente (stessa fonte, :177) [V].
- Nessuna fonte trovata usa la distanza fra posizione **percepita** e **dichiarata** come causa: la letteratura misura la distanza fra fazioni o fra leader e minoranza [NON TROVATO].

### Quanta parte del gruppo se ne va: casi italiani alla Camera

Misurato sull'endpoint SPARQL della Camera: fondatori del nuovo gruppo il giorno della nascita, divisi per il gruppo d'origine, e consistenza del gruppo d'origine il giorno prima (fonti/camera-sparql-scissioni-gruppi.md, query e risposte integrali) [M].

| Scissione | Da | Fondatori dal partito d'origine | Gruppo d'origine il giorno prima | Quota | Origine al governo? |
|---|---|---|---|---|---|
| Futuro e libertà, 30/7/2010 | PdL | 33 (:55) | 271 (:134) | 12,2% | sì |
| MDP, 28/2/2017 | PD | 18, più 16 da SI-SEL (:17-19) | 303 (:77) | 5,9% | sì |
| Italia Viva, 19/9/2019 | PD | 24 (:31) | 111 (:100) | 21,6% | sì |
| Insieme per il futuro, 21/6/2022 | M5S | 50 (:44) | 155 (:115) | 32,3% | sì |
| Italia Viva-Il Centro, 20/11/2023 | gruppo Azione-IV | 9 (:66) | 21 (:155) | 42,9% | no |
| Futuro Nazionale, 6/2/2026 | Lega | 2 (:192-193) | 61 (:171) | 3,3% | sì |

- Quota mediana sulle cinque scissioni di un partito vero (esclusa la separazione del gruppo comune Azione-IV): 12,2%; intervallo 3,3-32,3% [M, dalla tabella].
- Il gruppo scissionista cresce dopo la nascita: Futuro Nazionale passa da 2 deputati a 8 in quattro mesi, con ingressi da Lega e Forza Italia (fonti/open-2026-06-06-futuro-nazionale-ingressi.md:15) [V].
- Quota di cassa: [NON TROVATO]. Un partito nato da una scissione che forma un gruppo o una componente del Misto può chiedere il 2 per mille subito, senza attendere un'elezione (fonti/normattiva-dl149-2013-art10.md:16-19) [V]: le risorse pubbliche seguono i parlamentari che escono, attraverso le scelte dei contribuenti degli anni successivi, non una spartizione del patrimonio [INF].

### Se il partito d'origine governa

Quattro scissioni da partiti di governo alla Camera, e in nessun caso il nuovo gruppo esce subito dalla maggioranza:

- **MDP** (2017) esce dalla maggioranza Gentiloni sette mesi dopo, il 3 ottobre 2017, con le dimissioni del suo unico sottosegretario (fonti/ilpost-2017-10-04-mdp-esce-maggioranza.md:15) [V].
- **Italia Viva** (2019) resta nel governo Conte II per sedici mesi e ritira le ministre il 13 gennaio 2021, aprendo la crisi (fonti/skytg24-2021-01-13-italia-viva-ritira-ministre.md:13) [V].
- **Futuro e libertà** (2010): la scissione contribuisce alla caduta del governo Berlusconi del 2011 (fonti/lse-ceron-2013-fazioni-scissioni.md:17) [V]; data dell'uscita formale dalla maggioranza [NON TROVATO in fonte catturata].
- **Insieme per il futuro** (2022) resta nel governo con il ministro degli Esteri Di Maio; il rischio per il governo viene dal partito d'origine (fonti/ilpost-2022-06-22-scissione-m5s.md:17, :33) [V].
- **Futuro Nazionale** (2026): i deputati vengono dalla maggioranza (fonti/open-2026-06-06-futuro-nazionale-ingressi.md:15) e passano al Misto (fonti/lapresse-2026-02-06-futuro-nazionale-deputati.md:9) [V]; posizione verso il governo [NON TROVATO].

### Fusioni

- Ibenskas, 24 democrazie europee dal dopoguerra: 94 fusioni, in media una ogni tre legislature; si fondono partiti che devono superare una soglia o diventare uno dei due-tre partiti centrali per i governi, e la distanza ideologica è un costo (fonti/ibenskas-2016-fusioni-partiti.md:15-17, :46-47) [V]. La stessa fonte cita l'Italia dagli anni Novanta come esempio di sistema rifatto anche dalle fusioni (:31) [V].

## Scioglimento e morte dei partiti

- Bolleyer, Ibenskas e Bischoff 2019: 144 partiti nuovi in 17 democrazie consolidate, nati dal 1968; un partito è morto quando smette per sempre di presentare candidati (fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md:821) [V]. Ne muoiono 64, il 44%, entro il 2011 (:831) [V]; la vita media dei morti è 13 anni, con deviazione standard di 8,4 (:867) [V].
- Perdere i seggi non coincide con la morte: 23 partiti sopravvivono più di cinque anni dopo aver perso la rappresentanza (11 ancora attivi nel 2011), e 17 partiti mai entrati in parlamento sopravvivono più di cinque anni (fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md:851-853) [V].
- I seggi proteggono: passare da nessun seggio al 4,4% dei seggi riduce il rischio di morte del 38% (fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md:1178-1180) [V]. I partiti fondati da parlamentari (cioè quasi sempre le scissioni) sono più protetti nei primi sei anni e più fragili dopo i dodici: a 23,5 anni il loro rischio è più di sei volte quello degli altri (:1093-1103) [V].
- Emanuele e Sikk: la traiettoria più comune dei partiti nuovi è il «flop», che scompare dopo la seconda o la terza elezione (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:526-528) [V].
- Beyens, Lucardie e Deschouwer 2016 (Paesi Bassi e Belgio, 30 partiti nuovi con seggi 1950-2003) usano una definizione di scomparsa legata ai risultati (citati in fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md:293) [V]; le loro cifre [NON TROVATO: articolo non accessibile].

## Quanti partiti

Misurato su ParlGov (versione di sviluppo, Camera, righe italiane: fonti/parlgov-view-election-csv.md:5959-6041). Numero effettivo `1/Σ s_i²` su voti (NEV) e seggi (NES); «con seggi» conta anche le voci aggregate di ParlGov («one seat», «others»); «≥3%» conta le liste sopra la soglia di ammissione [M].

| Elezione | Liste con seggi | Liste ≥3% | NEV | NES |
|---|---|---|---|---|
| 2008 | 9 | 6 | 3,50 | 3,08 |
| 2013 | 13 | 6 | 5,00 | 3,51 |
| 2018 | 14 | 6 | 4,85 | 4,32 |
| 2022 | 12 | 7 | 6,39 | 5,64 |

- Gruppi alla Camera il 19 novembre 2023: otto gruppi più il Misto; dal giorno dopo nove, con Italia Viva-Il Centro (fonti/camera-sparql-scissioni-gruppi.md:142-160, :58-66) [M]. Nel 2008 la lista che ParlGov registra come PRC (Sinistra Arcobaleno) prende il 3,16% e nessun seggio (fonti/parlgov-view-election-csv.md:5959-6041) [M]; la ragione è la soglia del 4% per le liste fuori coalizione della legge di allora [INF].
- Con la lettura di Laver-Schilperoord, sei o sette partiti rilevanti corrispondono a una soglia di sopravvivenza di fatto intorno al 10%, molto più alta della soglia legale del 3% (fonti/laver-schilperoord-2007-nascita-partiti.md:132; soglia legale in [[sistemi-elettorali]]) [INF].

## Regola di movimento

- **Passo nel modello.** Nel codice Laver-Sergenti gli Hunter e gli Aggregator hanno passi di 0,25, 0,5, 1, 2 e 4 unità per tick (fonti/laver-sergenti-netlogo-cap7.md:89, :97, :123), con elettori distribuiti con deviazione standard 10 (:1341) e 20 tick di campagna fra un'elezione e l'altra (:463, :1355) [V]. Lo spostamento massimo fra due elezioni va quindi da 0,5 a 8 deviazioni standard dell'elettorato, e il tick non corrisponde a un tempo del calendario: i passi del modello non sono tarati su dati di posizione [INF].
- **Variazioni osservate.** CHES 1999-2024, stesso partito in due ondate successive (ogni 3-5 anni, circa una legislatura): 959 coppie in Europa, |Δ`lrgen`| media 0,45, mediana 0,34, 75° percentile 0,62, 90° percentile 0,98 sulla scala 0-10; solo il 19% supera 0,71 e il 9% supera 1. Italia: 55 coppie, media 0,46, mediana 0,36, 90° percentile 1,00 [M].
- **Nei posti del gioco.** Con la mappa di troncamento, fra un'ondata e la successiva il 71% delle coppie resta nello stesso posto, il 27% si sposta di un posto e l'1,7% di due o più; in Italia 73% fermi e 27% di un posto [M].
- **Scavalcamenti.** Fra due ondate il 14,6% delle coppie di partiti vicini nello stesso paese si scambia l'ordine sull'asse (769 coppie); in Italia il 16,7% (7 su 42: Fratelli d'Italia e Lega si scambiano fra 2014 e 2019 e di nuovo fra 2019 e 2024) [M].
- **Che cosa muove i partiti.** Sui programmi elettorali di otto paesi dell'Europa occidentale i partiti si spostano quando l'opinione pubblica si allontana chiaramente dalla loro posizione, mentre non si trova alcuna reazione ai risultati delle elezioni precedenti (fonti/adams-2004-cambio-posizioni-abstract.md:15) [V, solo abstract]. Hunter fa esattamente questo: ripete o inverte il passo secondo i voti guadagnati (fonti/laver-sergenti-netlogo-cap7.md:217) [V].

Comandi delle misure [M], tutti dal 29/10/2026, su file catturati in `fonti/`: (1) CHES: `python3 -I` sul CSV `ches-trend-1999-2024-csv` (sha256 del CSV originale `254384ab…a033`), |Δ`lrgen`| fra ondate consecutive in cui compare lo stesso `party_id`, posti con la mappa `{0,1}->1, {2,3}->2, 4->3, 5->4, 6->5, {7,8}->6, {9,10}->7`; scavalcamenti contati sulle coppie adiacenti per `lrgen` nell'ondata t che risultano invertite nell'ondata t+1. (2) ParlGov: `python3 -I` sul CSV `parlgov-view-election-csv` (sha256 `4b318c36…ac64`), righe `ITA`, `election_type = parliament`. (3) Camera: query SPARQL riportate integralmente in `fonti/camera-sparql-scissioni-gruppi.md`.

## Per il nostro gioco

### Parametri proposti

| Parametro della spec | Valore proposto | Fonte e grado di appoggio |
|---|---|---|
| X% degli elettori nel tratto scoperto (protesta) | **10%**, con intervallo di prova 7,5-15% | Laver-Schilperoord: soglia di nascita = soglia di sopravvivenza, e 10% corrisponde a sistemi di 6-7 partiti come quello italiano (fonti/laver-schilperoord-2007-nascita-partiti.md:112, :132) [V]; il passaggio dal modello al gioco è [INF] |
| distanza minima dal partito più vicino | 1 punto, come in spec | coerente con la nascita dove l'insoddisfazione (distanza dal partito più vicino) è massima (fonti/laver-schilperoord-2007-nascita-partiti.md:116-119) [V]; il valore 1 è [INF] |
| astensione sopra la media | come in spec | Tavits 2008: il voto ai nuovi dipende dalla delusione (fonti/tavits-2008-nuovi-partiti-abstract.md:18) [V]; Italia 2013, affluenza −5,9 punti (fonti/istat-annuario-2013-cap10-elezioni.md:30-32) [V] |
| al massimo un partito di protesta a legislatura | come in spec | Laver-Sergenti: al massimo una nascita per elezione (fonti/laver-sergenti-netlogo-cap7.md:394-397) [V]; nell'Europa occidentale i nuovi partiti superano insieme il 3% nel 12% delle elezioni (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:199-201) [V]: le condizioni devono renderla rara |
| dote del partito di protesta | piccola, privata: per esempio la cassa del partito più piccolo in parlamento | niente 2 per mille senza eletti né gruppo (fonti/normattiva-dl149-2013-art10.md:15-19) [V]; ammontare [NON TROVATO], il valore è una scelta di gioco |
| luna di miele del partito nuovo | bonus di valence fino alla prima elezione, che poi decade | il 53,5% dei nuovi partiti («flop») cala dopo il debutto, il 31,5% («explosive» e «contender») cresce (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:506-512) [V]; durata [NON TROVATO], vedi [[cicli-politico-economici-e-luna-di-miele]] |
| quota dei seggi che segue la scissione | **12%** (mediana), intervallo 5-30% | casi alla Camera 2010-2026 (fonti/camera-sparql-scissioni-gruppi.md) [M] |
| quota della cassa che segue la scissione | uguale alla quota dei seggi | spartizione [NON TROVATO]; lo scissionista con un gruppo accede subito al 2 per mille (fonti/normattiva-dl149-2013-art10.md:16-19) [V]; proporzione [INF] |
| soglia di scissione (percepita − dichiarata) | 1 punto, minaccia a 0,5, come in spec | nessuna fonte usa questa misura [NON TROVATO]; resta un'astrazione di gioco |
| scioglimento | due elezioni senza seggi, come in spec | in linea con i «flop» che spariscono dopo la seconda o terza elezione (fonti/emanuele-sikk-2018-traiettorie-nuovi-partiti.md:526-528) [V]; più severo della morte organizzativa (fonti/bolleyer-ibenskas-bischoff-2019-morte-partiti.md:851-853) [V] |
| tetto al numero dei partiti | 10, come in spec | liste ≥3%: 6-7; liste con seggi: 9-14 comprese le micro-liste; NEV 3,5-6,4 (ParlGov 2008-2022) [M] |
| passo massimo della dichiarata | 0,5 a legislatura, come in spec | in punti CHES vale 0,5-1,0; il 90° percentile osservato è 0,98 e solo il 19% delle variazioni supera 0,71 (CHES) [M] |
| passo nel modello Laver-Sergenti | non trasferibile | passi da 0,025 a 0,4 deviazioni standard per tick, senza tempo di calendario (fonti/laver-sergenti-netlogo-cap7.md:97, :1341, :1355) [V] |

### Dove i dati contraddicono la spec

1. **Uscita dalla coalizione** se il partito d'origine governava: in nessuno dei quattro casi italiani documentati il gruppo nuovo esce subito. MDP esce dopo sette mesi, Italia Viva dopo sedici, Insieme per il futuro resta, Futuro e libertà contribuisce alla caduta del governo più tardi (vedi «Se il partito d'origine governa»). Una regola più fedele terrebbe il partito nuovo nella maggioranza come alleato fragile, che può uscire in seguito [INF].
2. **Quota fissa dei seggi:** la quota osservata varia da 3% a 32%, e il gruppo nuovo continua a crescere nei mesi successivi (Futuro Nazionale da 2 a 8 deputati). Una quota fissa va bene come media, non come regola del singolo caso [M].
3. **Causa della scissione:** la distanza fra percepita e dichiarata non ha riscontro in letteratura; contano la forza della minoranza, i premi di carica, le liste bloccate (Ceron) e la grandezza del partito (Ibenskas-Sikk). Quando l'unità serve, il leader compra la minoranza invece di lasciarla uscire (fonti/lse-ceron-2013-fazioni-scissioni.md:29) [V]; un partito di governo con una maggioranza stretta è il caso tipico [INF]. La spec non ne tiene conto.
4. **Niente fusioni:** in Europa avviene una fusione ogni tre legislature circa (fonti/ibenskas-2016-fusioni-partiti.md:46-47) [V]. Escluderle è una semplificazione consapevole, che con scissioni e nascite attive fa salire il numero dei partiti nel tempo senza un meccanismo che lo riduca, salvo lo scioglimento [INF].
5. **Mai oltre il vicino:** fra due ondate CHES un partito vicino su sette scavalca l'altro, in Italia uno su sei [M]. Il divieto elimina una dinamica reale (Fratelli d'Italia e Lega si sono scambiati di posto due volte).
6. **Hunter come regola di movimento:** sui programmi elettorali non c'è traccia di reazione ai risultati delle elezioni precedenti, che è proprio il segnale di Hunter (fonti/adams-2004-cambio-posizioni-abstract.md:15) [V, solo abstract]. Una regola che segue lo spostamento degli elettori ha più appoggio empirico [INF].
7. **Partito di protesta Hunter:** in Laver-Schilperoord gli Hunter nati in periferia si spostano verso il centro (fonti/laver-schilperoord-2007-nascita-partiti.md:172) [V]. Il passo massimo di mezzo punto rallenta la deriva ma non la toglie: dopo due legislature il partito di protesta può non essere più tale [INF].

### Aperto

- La durata della luna di miele, del leader e del partito nuovo, resta senza fonte primaria.
- La spartizione della cassa nelle scissioni non è documentata: servirebbero i rendiconti dei partiti depositati presso la Commissione di garanzia.
- Le cifre di Beyens et al. 2016 e l'articolo BJPS di Ceron 2015 non sono stati letti per esteso (accesso negato dai repository).

## Collegamenti

- [[abm-elettorali]]: regole Sticker/Aggregator/Hunter/Governator e parametri Laver-Sergenti; questa pagina ne aggiunge nascita e morte.
- [[partiti-e-coalizioni]]: le scissioni nei giochi esistenti.
- [[formazione-di-coalizioni]]: dove si decide se lo scissionista resta in maggioranza.
- [[sistemi-elettorali]]: soglia del 3% e numero effettivo di partiti.
- [[cicli-politico-economici-e-luna-di-miele]]: luna di miele del leader.
- [[finanziamento-dei-partiti]]: 2 per mille e cassa, utili per dote e quota di cassa nelle scissioni.
- [[ches]], [[parlgov]]: i dataset usati per le misure.
- [[decisioni]]: le scelte di prodotto del 18/10 che questa pagina mette alla prova.

## Riferimenti

- **URL** https://pmc.ncbi.nlm.nih.gov/articles/PMC2440781/ · [V]
  **perché conta qui** è l'articolo che formalizza nascita e morte endogena dei partiti nel filone Laver, con i parametri usati
  **cosa se ne prende** soglia di nascita uguale alla soglia di sopravvivenza, la tabella soglia → numero di partiti, il 10% per sistemi di 6-7 partiti, la nascita degli Hunter in periferia
- **URL** https://modelingcommons.org/browse/one_model/3371 (file `Party_competition_Ch7.nlogo`) · [V]
  **perché conta qui** è il codice ufficiale del cap. 7 di Laver e Sergenti, con valori e regole esatte
  **cosa se ne prende** al massimo una nascita per elezione, soglia 0,1, memoria 0,5, passi 0,25-4 su deviazione standard 10, 20 tick per elezione
- **URL** https://profiles.wustl.edu/en/publications/party-system-change-testing-a-model-of-new-party-entry/ · [V, abstract]
  **perché conta qui** modello di ingresso strategico testato su 22 paesi OCSE
  **cosa se ne prende** costi d'ingresso, benefici e probabilità di voti come condizioni della nascita
- **URL** https://ideas.repec.org/a/cup/bjposi/v38y2008i01p113-133_00.html · [V, abstract]
  **perché conta qui** lega il voto ai nuovi partiti alla delusione verso quelli esistenti
  **cosa se ne prende** l'appoggio alla condizione di astensione e scontento della spec
- **URL** https://discovery.ucl.ac.uk/10179760/1/Emanuele%20%26%20Sikk%20ECPR%202018.pdf · [V]
  **perché conta qui** traiettorie dei partiti nuovi in 20 paesi europei, con l'Italia e il M5S
  **cosa se ne prende** frequenza dei nuovi partiti, quota dei «flop» e scomparsa dopo la seconda o terza elezione
- **URL** https://www.istat.it/it/files//2013/12/Cap_10.pdf · [V]
  **perché conta qui** affluenza ufficiale 2008 e 2013 alla nascita elettorale del M5S
  **cosa se ne prende** il calo di 5,9 punti di affluenza nel 2013
- **URL** https://documenti.camera.it/leg17/dossier/Testi/d13149.htm · [V]
  **perché conta qui** regole di accesso dei partiti al 2 per mille
  **cosa se ne prende** senza eletti niente fondi pubblici, quindi la dote del partito nuovo è privata
- **URL** https://www.normattiva.it/uri-res/N2Ls?urn:nir:stato:decreto.legge:2013-12-28;149~art10 · [V] (fonte catturata da un altro agente per [[finanziamento-dei-partiti]])
  **perché conta qui** testo vigente dei requisiti per il 2 per mille
  **cosa se ne prende** un partito con un gruppo o una componente del Misto accede al 2 per mille anche senza eletti col proprio simbolo: conta per la cassa delle scissioni
- **URL** https://blogs.lse.ac.uk/europpblog/2013/07/09/factions-and-fissions-in-the-aftermath-of-the-2013-elections-why-italian-politics-never-gets-boring-2 · [V]
  **perché conta qui** sintesi dell'autore del modello di scissione delle fazioni italiane
  **cosa se ne prende** cause della scissione (forza della minoranza, premi, liste bloccate) e l'esempio di Futuro e libertà
- **URL** https://eprints.soton.ac.uk/397775/1/pre-publication.pdf · [V]
  **perché conta qui** conteggio sistematico delle scissioni in 11 democrazie
  **cosa se ne prende** frequenza (19% delle coppie partito-legislatura) e il legame con la grandezza del partito
- **URL** https://eprints.soton.ac.uk/386722/1/683685.pdf · [V]
  **perché conta qui** l'unico studio comparato sulle fusioni
  **cosa se ne prende** una fusione ogni tre legislature, contro il «niente fusioni» della spec
- **URL** https://dati.camera.it/sparql · [M]
  **perché conta qui** fonte ufficiale delle adesioni ai gruppi della Camera, giorno per giorno
  **cosa se ne prende** le quote dei sei casi di scissione 2010-2026 e la crescita successiva dei gruppi
- **URL** https://www.ilpost.it/2017/10/04/mdp-maggioranza-pisapia/ · [V]
  **perché conta qui** data e modo dell'uscita di MDP dalla maggioranza
  **cosa se ne prende** i sette mesi fra scissione e uscita
- **URL** https://tg24.sky.it/politica/2021/01/13/crisi-governo-conte-cdm · [V]
  **perché conta qui** data del ritiro delle ministre di Italia Viva
  **cosa se ne prende** i sedici mesi fra scissione e uscita dal governo
- **URL** https://www.ilpost.it/2022/06/22/scissione-m5s/ · [V]
  **perché conta qui** consistenza di Insieme per il futuro e rapporto col governo Draghi
  **cosa se ne prende** lo scissionista che resta al governo mentre il partito d'origine traballa
- **URL** https://www.lapresse.it/politica/2026/02/06/vannacci-sasso-e-ziello-lasciano-il-gruppo-della-lega-alla-camera/ · [V]
  **perché conta qui** primi deputati di Futuro Nazionale e motivi dell'uscita
  **cosa se ne prende** il caso del 2026: due deputati della Lega al Misto
- **URL** https://www.open.online/2026/06/06/futuro-nazionale-si-allarga-nuovi-ingressi-partito-vannacci/ · [V]
  **perché conta qui** crescita di Futuro Nazionale nei mesi dopo la nascita
  **cosa se ne prende** da 2 a 8 deputati in quattro mesi, contro la quota fissa e immediata della spec
- **URL** https://rucforsk.ruc.dk/ws/files/67183258/EPSR_Bolleyer_Party_Survival_epsr_Revision_R1_Final_july_18.pdf · [V]
  **perché conta qui** analisi di sopravvivenza di 144 partiti nuovi in 17 democrazie
  **cosa se ne prende** 44% morti, vita media 13 anni, sopravvivenza senza seggi, effetto dei seggi e delle scissioni sul rischio
- **URL** https://www.parlgov.org/data/parlgov-development_csv-utf-8/view_election.csv · [M]
  **perché conta qui** risultati delle elezioni italiane in voti e seggi per lista
  **cosa se ne prende** liste con seggi, liste sopra il 3% e numero effettivo 2008-2022 per il tetto di 10
- **URL** https://github.com/chesdata/chesdata.github.io/releases/download/ches-trend/1999-2024_CHES_dataset_meansV2.csv · [M]
  **perché conta qui** posizioni dei partiti europei in sette ondate, con lo stesso codice di partito
  **cosa se ne prende** distribuzione dei cambi di posizione fra ondate e frequenza degli scavalcamenti
- **URL** https://repository.essex.ac.uk/10107/ · [V, abstract]
  **perché conta qui** test empirico di che cosa sposta le posizioni dei partiti
  **cosa se ne prende** nessuna reazione ai risultati elettorali passati, contro la logica di Hunter
