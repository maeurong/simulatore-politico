---
tipo: sintesi
fonti: [fonti/bdi-ibf-distribuzione-microdati.md, fonti/mef-finanze-open-data-dichiarazioni.md, fonti/mef-finanze-open-data-pensionati.md, fonti/rgs-copyright.md, fonti/itanes-2022-dataverse-termini.md, fonti/itanes-2022-questionario-pre.md, fonti/itanes-2022-questionario-post.md, fonti/ess-r11-questionario.md, fonti/ess-licence.md, fonti/istat-note-legali.md, fonti/camera-risultati-2022-csv.md, fonti/cise-volatilita-dataset.md, fonti/schofield-2010-valenza-convergenza.md]
aggiornato: 2026-10-17
---
# Dati per tarare i gruppi dell'elettorato

Con quali dati aperti si tarano i 360 gruppi dell'elettorato (6 condizioni × 3 fasce di reddito × 20 regioni, decisione del 17/10 in [[decisioni]]): quote di popolazione, istogramma della posizione sui 7 punti, β del logit, fedeltà e affluenza. In breve: nessuna fonte pubblica dà la tabella incrociata né gli istogrammi a quella finezza. Esistono però tutti i margini, con licenze compatibili con la cartella dati separata, e due microdati (Banca d'Italia e ITANES) che danno la struttura delle associazioni. Il metodo più semplice è la ricalibratura iterativa sui margini (IPF, detta anche raking).

## Risposta breve

1. **Quote regione × condizione × fascia**: la tabella incrociata non è pubblicata [NON TROVATO]. Ci sono i margini: occupati dipendenti e indipendenti, disoccupati e inattivi per regione (ISTAT), contribuenti per regione × classe di reddito × tipo di reddito (MEF), condizione × quinto di reddito a livello nazionale (ISTAT). Si ricostruisce con IPF partendo dai microdati della Banca d'Italia [INF].
2. **Istogrammi della posizione**: nessuna indagine è abbastanza fitta. ITANES 2022 ha tutte le variabili ma 6.250 casi, cioè circa 17 per gruppo e 2-3 per punto dell'asse. Si stimano i margini (regione × posizione; condizione × fascia × posizione) e si ricompone con la stessa IPF [INF].
3. **β, fedeltà, affluenza**: nessun β italiano trovato su fonte primaria accessibile [NON TROVATO]; si stima offline da ITANES con un logit condizionale. Fedeltà per gruppo [NON TROVATO], ricavabile dalla matrice voto 2018 → voto 2022 di ITANES. Affluenza: valore ufficiale per regione misurato (da 50,6% in Calabria a 71,6% in Emilia-Romagna); per condizione solo da sondaggio, da ricalibrare sui valori ufficiali [M e INF].

## Tabella dei dati

Granularità: R = regione, C = condizione, F = reddito. «Cartella dati» = si può copiare nella cartella dati separata secondo [[decisioni]] (16/10).

| Dato | Fonte | Licenza | R | C | F | Cartella dati | Cosa ne prendiamo |
|---|---|---|---|---|---|---|---|
| Occupati per posizione (dipendenti, indipendenti), 2024-2025 | ISTAT RCFL, flusso `150_938_DF_DCCV_OCCUPATIT1_23` [M] | CC BY 4.0 [V] | sì, anche province | dipendente / indipendente | no | sì | margine R × lavoratori |
| Disoccupati 15-74 | ISTAT, `151_929_DF_DCCV_DISOCCUPT1_6` [M] | CC BY 4.0 [V] | sì | disoccupati | no | sì | margine R × disoccupato |
| Inattivi 15-64 e 15-74 | ISTAT, `152_928_DF_DCCV_INATTIV1_9` [M] | CC BY 4.0 [V] | sì | inattivi, senza distinguere pensionato e studente | no | sì | margine R × inattivi (va diviso) |
| Famiglie per quinto di reddito equivalente × condizione del principale percettore, 2024 | ISTAT EU-SILC, `32_38_DF_DCCV_QUINTOREDD_5` [M] | CC BY 4.0 [V] | no, solo Italia | 5 modalità | quinti | sì | margine C × F nazionale |
| Reddito familiare medio e mediano per condizione del principale percettore | ISTAT EU-SILC, `32_292_DF_DCCV_REDNETFAMFONTERED_8` [M] | CC BY 4.0 [V] | 5 ripartizioni | 5 modalità | medio/mediano | sì | controllo per macroarea |
| Contribuenti per classe di reddito × regione × tipo di reddito (dipendente, pensione, autonomo), a.i. 2024 | MEF Finanze, open data dichiarazioni | CC BY 3.0 IT [V] | sì | come tipo di reddito, non esclusivo | 34 classi, individuale | sì | margine R × F per dipendenti e pensionati |
| Pensionati per regione; per classe di reddito | MEF Finanze, tavole Pensionati | CC BY 3.0 IT [V] | sì | pensionati | sì (separato) | sì | margine R × pensionato |
| Dipendenti pubblici per regione | RGS, Conto annuale | uso a fini di lucro solo con permesso scritto; citazione sempre obbligatoria [V] | sì [INF: sito non raggiunto in sessione] | pubblico | no | sì per un gioco gratuito, con citazione [INF] | divide i dipendenti fra pubblico e privato |
| Microdati IBF 2022 | Banca d'Italia | solo ricerca, citazione obbligatoria [V] | sì (`IREG`) [M] | qualifica, settore [M] | individuale e familiare | no [INF] | seme dell'IPF |
| ITANES 2022 pre (N=6.250) e post (N=4.696) | UNIMI Dataverse | senza fini di lucro; vietato condividere o pubblicare i dati; aggregati solo «in forma riassunta» [V] | sì | tutte e 6, pubblico/privato compreso [V] | familiare in 11 classi (post) [V] | microdati no; aggregati da chiedere [INF] | istogrammi, β, fedeltà, affluenza dichiarata |
| ESS round 11 (e precedenti) | ESS ERIC | CC BY-NC-SA 4.0 [V] | sì [INF] | attività principale, dipendente/autonomo, tipo di organizzazione (amministrazione, altro settore pubblico, privato) [V] | reddito familiare in 10 categorie [V], decili [INF] | sì, derivati in CC BY-NC-SA | secondo campione per istogrammi e affluenza dichiarata |
| CSES Modulo 6, Italia 2022 | CSES; la componente italiana è il post ITANES [V] | [NON TROVATO] | come ITANES | come ITANES | come ITANES | no | nulla in più di ITANES [INF] |
| Elettori e votanti 2022 per circoscrizione | Camera dei deputati, CSV | [NON TROVATO] | sì (19 regioni, VdA a parte) [M] | no | no | i 20 valori sì, come fatti [INF] | totale per regione e affluenza bersaglio |
| Volatilità elettorale 1945-2015 e aggiornamenti | CISE, Emanuele (2015) | [NON TROVATO] | no | no | no | da verificare | controllo aggregato della fedeltà |

Dettaglio di ogni dataset nelle pagine [[istat]], [[mef-statistiche-fiscali]], [[banca-italia-ibf]], [[itanes]], [[ess]], [[cses]], [[risultati-elettorali-italia]].

## 1. Quote dei gruppi: regione × condizione × fascia

### Cosa c'è

- La tabella 20 × 6 × 3 non è pubblicata da ISTAT, Banca d'Italia o MEF [NON TROVATO]. Le tavole ISTAT che incrociano condizione e reddito sono nazionali (quinto × condizione) o per ripartizione (reddito mediano × condizione) [M: query SDMX su `esploradati.istat.it/SDMXWS/rest/data/IT1,<flusso>,1.0/all`, in sessione 2026-10-17].
- Esempio del margine condizione × reddito (famiglie per quinto, percentuali di riga, 2024): indipendente 23,2% nel primo quinto e 30,6% nel quinto; dipendente 13,8% e 20,5%; ritirato dal lavoro 14,1% e 21,3%; in cerca di occupazione 57,6% e 4,8% [M, flusso `32_38_DF_DCCV_QUINTOREDD_5`]. Gli autonomi sono i più polarizzati: per loro una sola fascia media sarebbe sbagliata.
- Occupati 2025: 18,93 milioni di dipendenti e 5,18 milioni di indipendenti; in Lombardia 3,66 e 0,90 milioni [M, flusso `150_938_DF_DCCV_OCCUPATIT1_23`]. Disoccupati 15-74 nel 2025: 1,58 milioni [M, flusso `151_929_DF_DCCV_DISOCCUPT1_6`].
- MEF, anno d'imposta 2024: 42,8 milioni di contribuenti, di cui 24,1 con redditi da lavoro dipendente e 14,5 con redditi da pensione, per regione e 34 classi di reddito [M, dettaglio in [[mef-statistiche-fiscali]]].
- Microdati IBF 2022: 20.144 adulti con regione, qualifica, settore e reddito nello stesso record; media 56 adulti per gruppo, 79 nella regione meno rappresentata [M, dettaglio in [[banca-italia-ibf]]].

### Buchi e attriti

- Nessuna fonte pubblica separa i dipendenti pubblici da quelli privati per regione, salvo il Conto annuale RGS, che non si è riusciti ad aprire in sessione [INF].
- Le tavole regionali ISTAT degli inattivi non separano pensionati e studenti e si fermano a 74 anni [M]. I pensionati vengono meglio dal MEF; gli studenti maggiorenni restano il residuo degli inattivi sotto i 30 anni [INF].
- Le classi MEF sono di reddito individuale imponibile; gli autonomi in regime forfettario non ci sono [INF, vedi [[mef-statistiche-fiscali]]].
- Lavoratori stranieri senza cittadinanza contano nelle forze di lavoro ma non votano: le quote vanno ristrette agli elettori [INF]. ISTAT pubblica le stesse tavole per cittadinanza (dimensione `CITIZENSHIP` dei flussi) [M].

### Metodo: IPF sui margini [INF]

Il metodo più semplice che usa tutto quello che c'è:
1. **Seme**: tabella 20 × 6 × 3 dai microdati IBF 2022, pesati con `PESOFIT2`, se serve uniti al 2020. Le celle vuote prendono un valore piccolo proporzionale al prodotto dei margini (indipendenza), così l'IPF può muoverle.
2. **Margini bersaglio**: (a) elettori per regione, dai risultati 2022 della Camera o dalla popolazione maggiorenne con cittadinanza; (b) regione × condizione, da ISTAT (dipendenti, indipendenti, disoccupati, inattivi), MEF (pensionati) e RGS (quota pubblica dei dipendenti); (c) regione × fascia, dalle classi MEF tradotte in fasce; (d) condizione × fascia nazionale, dai quinti ISTAT EU-SILC.
3. **IPF**: si scala a turno la tabella perché rispetti ciascun margine, fino a che tutti tornano entro lo 0,1%. Con 360 celle il calcolo è istantaneo. È l'algoritmo classico di ricalibratura delle tabelle di contingenza; la sua convergenza su margini coerenti è un risultato noto della statistica [INF, nessuna fonte catturata in sessione].
4. **Controllo**: la tabella finale deve ridare le distribuzioni IBF per macroarea entro l'errore campionario [INF].

### Fasce e decili

Le fasce devono mapparsi sui 10 decili dell'economia (`docs/ricerca/modelli/mercato-del-lavoro-e-disuguaglianza.md:37`). **Raccomandazione**, non decisione: fasce sul reddito familiare equivalente, bassa = decili 1-3, media = 4-7, alta = 8-10. Così ESS (10 categorie di reddito familiare) e i quinti ISTAT si convertono senza riclassificare, e la fascia media pesa il 40% della popolazione, le altre due il 30% ciascuna [INF].

## 2. Posizione di partenza: istogrammi a 7 punti

### Cosa c'è

- **ITANES 2022**: autocollocazione 0-10 con «non mi collocherei» e «non saprei» (fonti/itanes-2022-questionario-pre.md:515) [V]; condizione con settore pubblico (:849) e pensionato, studente, disoccupato (:901-904) [V]; reddito familiare in 11 classi nel post (fonti/itanes-2022-questionario-post.md:1013) [V]; regione (:112) [V]. N = 6.250 nel pre (fonti/itanes-2022-dataverse-termini.md:72) [V].
- **ESS round 11**: scala 0-10 (fonti/ess-r11-questionario.md:514) [V]; attività principale (lavoro, studio, disoccupazione, pensione, :3103) [V]; dipendente o autonomo (:3214) [V]; tipo di organizzazione, con amministrazione centrale o locale e altro settore pubblico distinti dall'impresa privata (:3364) [V]; reddito familiare netto in 10 categorie (:3530) [V]. Che le 10 categorie siano decili nazionali e che il file dati abbia la regione è [INF]. Numerosità italiana del round 11 [NON TROVATO] (le pagine INAPP e GESIS rispondono 403 ai client automatici).
- Eurobarometro: non serve qui. Non distingue pubblico e privato e non ha il reddito, e i microdati GESIS non sono stati verificati ([[eurobarometro]]) [INF].

### Perché non bastano da soli

Con 6.250 casi su 360 gruppi si hanno in media 17 casi per gruppo, 2-3 per punto dell'asse; nelle regioni piccole spesso zero [INF, aritmetica]. Un istogramma per gruppo stimato direttamente sarebbe quasi tutto rumore.

### Metodo: stessa IPF, con la posizione come quarta dimensione [INF]

Tabella regione × condizione × fascia × posizione (20 × 6 × 3 × 7 = 2.520 celle), con margini:
- totali dei gruppi, dalla risposta 1;
- condizione × fascia × posizione, nazionale: 18 combinazioni con circa 350 casi ciascuna in ITANES;
- regione × posizione: circa 310 casi per regione in media; le regioni sotto i 100 casi si accorpano alla macroarea [INF].

Il risultato equivale a un modello log-lineare senza interazione a tre fra regione, condizione e posizione. Che una pensionata calabrese stia a sinistra quanto la media dei pensionati, corretta per la Calabria, è un'ipotesi accettabile per una partenza; se il gioco mostra stranezze si passa a una regressione multilivello con poststratificazione (MRP) [INF]. Con ESS e ITANES uniti i margini sono più stabili, ma vanno armonizzati prima (diversa modalità di intervista, diversi anni) [INF].

### Dalla scala 0-10 ai 7 punti

**Raccomandazione**: {0,1} → estrema sinistra, {2,3} → sinistra, 4 → centrosinistra, 5 → centro, 6 → centrodestra, {7,8} → destra, {9,10} → estrema destra [INF]. Il valore 5 raccoglie molti indecisi; chi risponde «non mi collocherei» o «non saprei» va trattato a parte, per esempio al centro con più astensione, perché è la quota con la partecipazione più bassa [INF, da verificare sui dati]. Il taglio si controlla a posteriori: con le posizioni dei partiti tarate su [[ches]], il voto simulato deve ridare il 2022 entro qualche punto.

### Licenza degli istogrammi

Costruiti da ITANES, sono aggregati dei microdati: i termini ammettono la pubblicazione «in forma riassunta» ma vietano di rendere disponibili i dati (fonti/itanes-2022-dataverse-termini.md:20, :24) [V]. **Raccomandazione**: costruire gli istogrammi da mettere nella cartella dati con ESS (CC BY-NC-SA 4.0, ammessa da [[decisioni]]); usare ITANES solo per stime offline e controlli, finché ITANES non concede il permesso [INF].

## 3. β, fedeltà e affluenza

### β del logit spaziale

- Valore italiano: [NON TROVATO] su fonte primaria accessibile. Il capitolo di Schofield e Sened sulle elezioni italiane 1992-1996 (*Multiparty Democracy*, 2006) usa un modello di questo tipo, ma la pagina Cambridge risponde 403 [NON TROVATO].
- Ordini di grandezza già in wiki: circa 0,85 negli USA 2008 e circa 0,27 nel Regno Unito 1979, dipendenti dalla scala (`docs/ricerca/modelli/voto-spaziale.md:25`, fonti/schofield-2010-valenza-convergenza.md) [V].
- **Stima offline [INF]**: logit condizionale sul post ITANES 2022 (CSES Modulo 6), con autocollocazione (fonti/itanes-2022-questionario-post.md:683), collocazione dei partiti (:702) e voto; valence come costante per partito. Per un primo test del codice c'è il pacchetto R `nopp` (archiviato da CRAN il 6/10/2025, GPL ≥ 2): dati CSES Italia 2006, 438 rispondenti, cinque partiti, distanza quadratica già calcolata [V: file `man/italy2006.Rd` e `DESCRIPTION` di `nopp_1.1.2.tar.gz` da https://cran.r-project.org/src/contrib/Archive/nopp/, letti in sessione, non salvati in `fonti/`].
- **Cambio di scala** [INF, aritmetica]: se la posizione a 7 punti è circa `1 + 0,6·x` con x sulla scala 0-10, le distanze si accorciano di 0,6 e le distanze al quadrato di 0,36. Quindi `β₇ = β₁₀ / 0,36 ≈ 2,8 · β₁₀`. Senza questa correzione un β preso dalla letteratura su scala 0-10 rende gli elettori 2,8 volte meno sensibili alla posizione di quanto stimato.

### Fedeltà al voto precedente

- Valore per gruppo: [NON TROVATO].
- **Stima offline [INF]**: ITANES chiede il voto alla Camera del 2018 nel pre (fonti/itanes-2022-questionario-pre.md:662) e nel post (fonti/itanes-2022-questionario-post.md:573) [V]. Da qui la matrice di transizione 2018 → 2022 per condizione e macroarea, e il termine di fedeltà del logit si tara perché la simulazione la riproduca. Il ricordo del voto passato tende a spostarsi verso il voto attuale, quindi la fedeltà stimata è un tetto [INF].
- **Controllo aggregato**: l'indice di Pedersen è un cambiamento netto, quindi un limite inferiore alla quota di chi cambia voto. Il dataset CISE lo dà per elezione (fonti/cise-volatilita-dataset.md:15) [V]; i valori 2018 e 2022 non sono stati letti in sessione [NON TROVATO].
- Il parametro di ancoraggio di Friedkin-Johnsen resta senza valore generale (`docs/ricerca/modelli/opinion-dynamics.md:38`). Il panel ITANES reintervista le stesse persone a una distanza fra due settimane e mezzo e sei settimane (pre dal 5 al 24 settembre, post dal 12 al 15 ottobre, fonti/itanes-2022-dataverse-termini.md:70-80): dà la stabilità dell'autocollocazione su un orizzonte breve, un limite inferiore per l'ancora su un turno di tre mesi [INF].

### Affluenza

- **Per regione, ufficiale**: politiche 2022, 63,6% su 46,25 milioni di elettori (Valle d'Aosta ed Estero esclusi), da 50,6% in Calabria a 71,6% in Emilia-Romagna [M, tabella completa in [[risultati-elettorali-italia]]].
- **Per condizione e fascia**: nessun dato ufficiale [NON TROVATO]; le liste sezionali registrano chi ha votato, non la sua professione [INF]. Solo dichiarato: ITANES e ESS (domanda sul voto alle ultime politiche, fonti/ess-r11-questionario.md:372) [V]. Nei sondaggi l'affluenza dichiarata supera quella reale [INF].
- **Metodo [INF]**: l'utilità dell'astensione si tara in due passi. Prima le differenze fra condizioni e fasce dai sondaggi, come rapporti; poi una costante per regione tale che la media pesata dei gruppi ridia l'affluenza ufficiale della regione.

## Licenze e cartella dati separata

Regola: dati in una cartella separata, ogni dataset con la sua licenza; ammessi gli NC, esclusi quelli che vietano la ridistribuzione (`docs/ricerca/sintesi/decisioni.md:19-20`).

| Fonte | Nella cartella dati? |
|---|---|
| ISTAT | sì, CC BY 4.0, salvo indicazioni diverse sulla singola tavola (fonti/istat-note-legali.md) [V] |
| MEF Finanze | sì, CC BY 3.0 IT (fonti/mef-finanze-open-data-dichiarazioni.md:67) [V] |
| RGS Conto annuale | il divieto riguarda i «fini di lucro» (fonti/rgs-copyright.md:15) [V]; un gioco gratuito ne è fuori, con citazione obbligatoria [INF] |
| ESS | sì, CC BY-NC-SA 4.0 sui derivati (fonti/ess-licence.md) [V] |
| Banca d'Italia IBF | microdati no; quote derivate da chiarire [INF] |
| ITANES | microdati no (fonti/itanes-2022-dataverse-termini.md:20, :26) [V]; aggregati da chiedere |
| Camera, CISE | licenza [NON TROVATO]; i 20 valori di affluenza come fatti [INF] |

## Per il nostro gioco

- **Prendiamo**: margini ISTAT e MEF nella cartella dati, perché sono aperti e completi; IPF come unico strumento per quote e istogrammi; affluenza regionale ufficiale come bersaglio; β e fedeltà stimati offline su ITANES, con solo i parametri nel repo.
- **Evitiamo**: copiare nel repo microdati IBF o ITANES; stimare un istogramma per gruppo direttamente da 17 casi; prendere un β dalla letteratura senza correggere la scala.
- **Aperto**: permesso ITANES per gli aggregati; codebook dei file annuali IBF; apertura del Conto annuale RGS per i dipendenti pubblici regionali; valori italiani di volatilità; definizione delle fasce (raccomandazione 3-4-3 sui decili, da confermare con il ticket Modello economico).
- La proposta di [[elettorato-e-gruppi-di-voto]] (`docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:42`) di campionare migliaia di agenti con reddito proprio è superata dalla decisione del 17/10 sui gruppi. Questa pagina dà i dati per i gruppi; gli elettori campione si generano dalle quote, senza dati propri.

## Collegamenti

- [[elettorato-e-gruppi-di-voto]]: i gruppi da tarare.
- [[voto-spaziale]]: β e valence.
- [[opinion-dynamics]]: l'ancora della posizione di partenza.
- [[mercato-del-lavoro-e-disuguaglianza]]: i 10 decili su cui si mappano le fasce.
- [[sistemi-elettorali]]: perché servono le 20 regioni.
- [[sondaggi-ed-errore]]: gli stessi incroci che il giocatore vede nei sondaggi.
- [[istat]], [[mef-statistiche-fiscali]], [[banca-italia-ibf]], [[itanes]], [[ess]], [[cses]], [[risultati-elettorali-italia]]: le pagine dei dataset.

## Riferimenti

- **URL** https://esploradati.istat.it/SDMXWS/rest/data/IT1,150_938_DF_DCCV_OCCUPATIT1_23,1.0/all · [M]
  **perché conta qui** occupati dipendenti e indipendenti per regione e provincia
  **cosa se ne prende** il margine regione × lavoratori dipendenti e autonomi
- **URL** https://esploradati.istat.it/SDMXWS/rest/data/IT1,152_928_DF_DCCV_INATTIV1_9,1.0/all · [M]
  **perché conta qui** inattivi 15-74 per regione
  **cosa se ne prende** il margine regione × inattivi, da dividere fra pensionati e studenti
- **URL** https://esploradati.istat.it/SDMXWS/rest/data/IT1,151_929_DF_DCCV_DISOCCUPT1_6,1.0/all · [M]
  **perché conta qui** disoccupati per regione
  **cosa se ne prende** il margine regione × disoccupati
- **URL** https://esploradati.istat.it/SDMXWS/rest/data/IT1,32_38_DF_DCCV_QUINTOREDD_5,1.0/all · [M]
  **perché conta qui** unico incrocio ufficiale condizione × reddito trovato
  **cosa se ne prende** il margine nazionale condizione × fascia
- **URL** https://www1.finanze.gov.it/finanze/analisi_stat/public/index.php?opendata=yes · [V]
  **perché conta qui** contribuenti per regione × classe di reddito × tipo di reddito, CC BY 3.0 IT
  **cosa se ne prende** il margine regione × fascia per dipendenti e pensionati (fonti/mef-finanze-open-data-dichiarazioni.md)
- **URL** https://www1.finanze.gov.it/finanze/analisi_stat/public/index.php?search_contrib=PFPEN&opendata=yes · [V]
  **perché conta qui** pensionati per regione
  **cosa se ne prende** il margine regione × pensionato (fonti/mef-finanze-open-data-pensionati.md)
- **URL** https://www.rgs.mef.gov.it/VERSIONE-I/copyright/index.html · [V]
  **perché conta qui** condizioni di riuso del Conto annuale RGS
  **cosa se ne prende** riuso a fini di lucro solo con permesso, citazione sempre (fonti/rgs-copyright.md)
- **URL** https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/index.html · [V]
  **perché conta qui** microdati con regione, condizione e reddito nello stesso record
  **cosa se ne prende** il seme dell'IPF e il vincolo «solo ricerca» (fonti/bdi-ibf-distribuzione-microdati.md)
- **URL** https://doi.org/10.13130/RD_UNIMI/JV77WR · [V]
  **perché conta qui** l'unico studio italiano con posizione, voto, voto 2018 e le sei condizioni
  **cosa se ne prende** margini degli istogrammi, β, fedeltà; termini che vietano di ridistribuire i microdati (fonti/itanes-2022-dataverse-termini.md, fonti/itanes-2022-questionario-pre.md, fonti/itanes-2022-questionario-post.md)
- **URL** https://www.europeansocialsurvey.org/sites/default/files/2024-01/ESS%20Round%2011%20Source%20Questionnaire_FINAL_Alert%2004.pdf · [V]
  **perché conta qui** variabili ESS utili ai gruppi, con licenza compatibile con la cartella dati
  **cosa se ne prende** scala 0-10, attività principale, settore pubblico, reddito in 10 categorie, voto (fonti/ess-r11-questionario.md)
- **URL** https://www.camera.it/deputati/elezioni/risultati-19-relazioni-csv-excel · [V]
  **perché conta qui** elettori e votanti ufficiali 2022
  **cosa se ne prende** l'affluenza regionale su cui si ricalibra l'astensione (fonti/camera-risultati-2022-csv.md)
- **URL** https://cise.luiss.it/cise/dataset-of-electoral-volatility-and-its-internal-components-in-western-europe-1945-2015/ · [V]
  **perché conta qui** volatilità aggregata come controllo della fedeltà
  **cosa se ne prende** struttura e citazione; valori italiani da leggere (fonti/cise-volatilita-dataset.md)
- **URL** https://cran.r-project.org/src/contrib/Archive/nopp/ · [V]
  **perché conta qui** dati CSES Italia 2006 già pronti per un logit spaziale, in un pacchetto GPL
  **cosa se ne prende** banco di prova per il codice di stima di β (file letti in sessione, non salvati in `fonti/`)
