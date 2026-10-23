---
tipo: dataset
fonti: []
aggiornato: 2026-10-23
---
# Sondaggi nazionali di ottobre 2026, sommati sui 7 posti

Media delle intenzioni di voto nazionali di ottobre 2026 per partito reale, ricondotta ai 7 posti dell'asse sinistra-destra con la collocazione CHES 2024. Serve alla taratura della valence dei sette partiti al turno zero (spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md:284-285`). Il vettore che il gioco usa davvero è nella sezione «Bersagli adottati»; le altre tabelle sono il materiale da cui nasce e le alternative scartate.

**Copertura.** Ottobre 2026 è coperto fino al 20 ottobre (dati letti il 23 ottobre 2026): 19 sondaggi di 11 istituti. Il mese non è chiuso: restano fuori le rilevazioni dal 21 al 31 ottobre.

## Bersagli adottati

Sono i valori di `strumenti/taratura/bersagli.toml`, letti da `sim-cli tara`. Medie grezze in percento, nell'ordine dei posti 1-7, somma 100.

| Posto | Quota | Composizione |
|---|---|---|
| 1 | 3,165 | metà di AVS |
| 2 | 36,015 | PD 20,33, M5S 12,52, metà di AVS 3,165 |
| 3 | 7,12 | Azione 3,16, Italia Viva 2,43, +Europa 1,53 |
| 4 | 5,02 | «altri», unico posto senza partito |
| 5 | 8,42 | Forza Italia 7,27, Noi Moderati 1,15 |
| 6 | 33,00 | Fratelli d'Italia 27,29, Lega 5,71 |
| 7 | 7,26 | Futuro Nazionale |

**Regola della mappa: troncamento, non arrotondamento.** Dal valore `lrgen` di CHES si prende la parte intera (le classi sono [k, k+1), come in un istogramma) e poi si applica il taglio in sette posti: `{0,1}->1, {2,3}->2, 4->3, 5->4, 6->5, {7,8}->6, {9,10}->7`. Il motivo viene dallo scenario di base. La spec fissa un «governo uscente di centrodestra e destra» (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:277`) e lo scenario lo traduce nei posti 5 e 6: presidente `destra`, due dicasteri `centrodestra`, coalizioni `[["sinistra","centrosinistra"],["centrodestra","destra"]]` (`dati/base/scenario.toml:12` e `:15-17`, con i posti numerati nello stesso file alle righe 212-280: estrema_sinistra 1, sinistra 2, centrosinistra 3, centro 4, centrodestra 5, destra 6, estrema_destra 7) [V]. Per mettere Forza Italia e Noi Moderati al posto 5 e la Lega al 6, come nel governo reale, serve il troncamento: l'arrotondamento le spostava di uno verso destra.

**Confronto con l'arrotondamento** (mappa usata in una versione precedente di questa pagina e di `bersagli.toml`, ora scartata). Cambiano tre partiti:

| Partito | `lrgen` | Arrotondato | Troncato |
|---|---|---|---|
| Forza Italia | 6,7333 | 7, posto 6 | 6, posto 5 |
| Noi Moderati [INF] | stesso intero di FI | posto 6 | posto 5 |
| Lega | 8,6667 | 9, posto 7 | 8, posto 6 |

Vettore con l'arrotondamento (stesse regole su AVS e «altri», questi ultimi divisi fra i posti 4 e 5 perché il 5 era vuoto): 3,165 / 36,015 / 7,12 / 2,51 / 2,51 / 35,71 / 12,97. Con quella mappa il governo uscente (posti 5-6) valeva soltanto il 38,2% (2,51 + 35,71) contro il 43,1% di sinistra e centrosinistra, e la Lega, partito di governo, cadeva fuori dalla coalizione al posto 7 [M: somma delle colonne della tabella, da `bersagli.toml` della versione precedente]. Con il troncamento le coalizioni dello scenario valgono 43,135 (posti 2-3) contro 41,42 (posti 5-6), cioè 1,7 punti di scarto invece di 4,9 [INF: somma dei bersagli sopra].

**Scelta su AVS.** AVS è una lista sola nel sondaggio ma due partiti in CHES: Sinistra Italiana (`lrgen` 1,2308, posto 1) ed Europa Verde (2,1429, posto 2). Scelta: la media grezza 6,33 si divide a metà, 3,165 al posto 1 e 3,165 al posto 2. Con la media dei due valori (1,6868) il troncamento darebbe 1 e tutta AVS andrebbe al posto 1; lo si è scartato perché i due partiti, a voti 2022 uguali, hanno posizioni diverse e la divisione rispetta entrambe. Il posto 1 resta così occupato.

**Scelta sugli «altri».** Gli «altri» (100 − 94,98 = 5,02, vedi sotto) vanno tutti al posto 4, l'unico che nessun partito reale occupa con questa mappa. Nessun posto resta a zero, perché il logit del gioco non può dare quota 0 a un partito. Gli «altri» non sono una quota rinormalizzata: i bersagli sono medie grezze, e per questo la somma dei partiti è 94,98 e non 100.

## Quote rinormalizzate sugli 11 partiti (non usate come bersaglio)

Quote rinormalizzate sui votanti degli 11 partiti (somma = 100), con la mappa per arrotondamento. Media degli 11 istituti, ciascuno prima mediato sui suoi sondaggi del mese. Restano come controllo e per il confronto con altre medie.

| Posto | Partiti (collocazione arrotondata) | Quota |
|---|---|---|
| 1 | nessuno con `lrgen` 0-1 (AVS cade al 2) | 0,0% (3,3% se AVS si divide) |
| 2 | PD, M5S, AVS | 41,2% |
| 3 | Azione, +Europa, Italia Viva [INF] | 7,5% |
| 4 | nessuno (`lrgen` 5) | 0,0% |
| 5 | nessuno (`lrgen` 6) | 0,0% |
| 6 | FdI, Forza Italia, Noi Moderati [INF] | 37,6% |
| 7 | Lega, Futuro Nazionale [INF] | 13,7% |
| astensione e indecisi | non nelle tabelle degli istituti | [NON TROVATO] |

Variante «AVS divisa» (metà al posto 1 e metà al 2): 3,3 / 37,9 / 7,5 / 0 / 0 / 37,6 / 13,7 [M].

Controllo incrociato con la Supermedia YouTrend/Agi del 22 ottobre (FdI 27,9, PD 20,3, M5S 12,1, Lega 5,6, FI 7,3, Azione 3,3, IV 2,4, AVS 6,5, +E 1,4, NM 1,1, FN 6,8), stessa mappa e stessa rinormalizzazione: 0 / 41,1 / 7,5 / 0 / 0 / 38,3 / 13,1. Scarto massimo 0,7 punti sulla stima sopra (posto 6: 38,3 contro 37,6) [M]. Media semplice sui 19 sondaggi, senza mediare prima per istituto: 41,2 / 7,5 / 37,7 / 13,7 [M].

## Media per partito

Media semplice sugli 11 istituti, punti di intenzione di voto sul totale dei rispondenti con preferenza, prima della rinormalizzazione.

**Come si rifà.** [M] Rifatto il 23 ottobre 2026 con uno script Python locale che legge il wikitext della revisione 1381481066 e dà gli stessi numeri della tabella. Lo script non è conservato nel repo (stava in `/tmp`): a lungo termine l'appiglio è l'ingresso, che si riscarica uguale all'URL `https://en.wikipedia.org/w/index.php?title=Opinion_polling_for_the_next_Italian_general_election&oldid=1381481066&action=raw` (691.148 byte, sha256 `b98af384…12c6`, verificato di nuovo il 23 ottobre 2026). Il procedimento: dalle righe della tabella nazionale con data di fine campo in ottobre 2026 si estraggono le 11 colonne dei partiti (compresa Futuro Nazionale), si media per istituto, poi si media sugli istituti. Noto non pubblica +Europa: la sua media su +E è sugli altri 10 istituti.

| Partito | Media | Quota rinormalizzata |
|---|---|---|
| Fratelli d'Italia | 27,29 | 28,73 |
| PD | 20,33 | 21,40 |
| M5S | 12,52 | 13,18 |
| Forza Italia | 7,27 | 7,65 |
| Futuro Nazionale | 7,26 | 7,65 |
| AVS | 6,33 | 6,66 |
| Lega | 5,71 | 6,01 |
| Azione | 3,16 | 3,33 |
| Italia Viva | 2,43 | 2,56 |
| +Europa | 1,53 | 1,62 |
| Noi Moderati | 1,15 | 1,22 |
| somma degli 11 | 94,98 | 100 |

## Istituti inclusi

Criterio: rilevazioni con fine campo in ottobre 2026 (comprende tre rilevazioni che iniziano il 30 settembre e finiscono il 1-5 ottobre). Esclusi per scelta: le medie di altri (YouTrend/Agi, BiDiMedia, Doppia Cifra, Cassandra, PolitPro), usate solo come controllo; i sondaggi sulle primarie del campo largo; le simulazioni di seggi.

| Istituto | Rilevazioni | Campione |
|---|---|---|
| SWG | 3 (30 set-5 ott, 7-12 ott, 14-19 ott) | 1.200 |
| Only Numbers | 3 (30 set-1 ott, 7-8 ott, 14-15 ott) | 800 |
| Termometro Politico | 3 (30 set-1 ott, 7-8 ott, 13-15 ott) | 2.300-2.500 |
| EMG | 2 (5-6 ott, 19-20 ott) | 1.000 |
| Eumetra | 2 (7-8 ott, 13-14 ott) | 800 |
| BiDiMedia | 1 (17-19 ott) | 1.000 |
| Lab21 | 1 (9 ott) | 754 |
| Sigma Piave | 1 (5-8 ott) | 800 |
| Quorum | 1 (5-6 ott) | 805 |
| Piepoli | 1 (5 ott) | 500 |
| Noto | 1 (6 ott) | non indicato; senza +Europa |

I numeri per sondaggio non si copiano qui: sono nella pagina Wikipedia citata, alla revisione indicata.

## «Altri» e indecisi

- Gli istituti pubblicano quote già riportate a 100 sui rispondenti con preferenza; gli indecisi e gli astenuti non compaiono nelle tabelle lette [M].
- Gli 11 partiti sommano il 94,98%. Il restante 5,02% è «altri»: Sud chiama Nord (entrato nel centrodestra il 10 ottobre), Partito Liberal Democratico, Ora!, Avanti-PSI, Progetto Civico Italia, Più Uno e le liste minori [M].
- Esito: nei bersagli gli «altri» restano nel conto (medie grezze) e vanno tutti al posto 4, vedi «Bersagli adottati». La rinormalizzazione sugli 11 partiti (divisione per 94,98), equivalente a distribuire gli «altri» in proporzione ai grandi, è documentata sopra ma non è quella adottata.
- Astensione e indecisi: **[NON TROVATO]** nelle tabelle consultate; una ricerca sulla quota SWG di ottobre 2026 non l'ha restituita. Il gioco ha una propria costante d'astensione (`strumenti/taratura/elettorato.py:79`, 0,081, tarata a mano sull'affluenza 2022) [V], e `sim-cli tara` riscrive le astensioni per regione; la taratura confronta quindi le quote fra votanti, non fra elettori.

## Corrispondenza partito -> posto

CHES 2024, file `CHES_2024_final_v2.csv`, Italia = codice paese 8, colonna `lrgen` [M: letto offline in sessione, vedi Riferimenti]. Regola adottata: troncamento all'intero, poi taglio del piano (vedi «Bersagli adottati»).

| Partito | `lrgen` grezzo | Intero (troncato) | Posto | Tag |
|---|---|---|---|---|
| Fratelli d'Italia (FDI) | 8,4667 | 8 | 6 | [M] |
| Lega | 8,6667 | 8 | 6 | [M] |
| Forza Italia (FI) | 6,7333 | 6 | 5 | [M] |
| PD | 2,9333 | 2 | 2 | [M] |
| M5S (nel file «MS5») | 3,3333 | 3 | 2 | [M] |
| Azione (A) | 4,4286 | 4 | 3 | [M] |
| +Europa (+E) | 4,3333 | 4 | 3 | [M] |
| Sinistra Italiana (SI) | 1,2308 | 1 | 1 | [M] |
| Europa Verde (EV) | 2,1429 | 2 | 2 | [M] |
| AVS (lista unica SI + EV) | media dei due 1,6868, voti 2022 uguali 1,8 e 1,8 | 1 sulla media | metà 1, metà 2 | [INF] |
| Italia Viva | assente da CHES 2024 | | 3 | [INF] |
| Noi Moderati | assente | | 5 | [INF] |
| Futuro Nazionale | assente (nata il 6 febbraio 2026) | | 7 | [INF] |

Motivazioni dei tre [INF] e dell'AVS:

- **AVS**: scelta di dividerla a metà fra i posti 1 e 2, vedi «Bersagli adottati».
- **Italia Viva** al posto 3: nel 2022 correva con Azione (lista unica «Terzo Polo», CHES 2024 ha solo Azione); centrista liberale, accanto ad Azione (4,43) e +Europa (4,33) [INF].
- **Noi Moderati** al posto 5: è nel centrodestra con FI, fonte democristiana; lo stesso intero di FI (6 col troncamento) la porta al posto 5. Pesa 1,15 punti, quindi l'errore massimo è piccolo [INF].
- **Futuro Nazionale** al posto 7: nata da una scissione dalla Lega il 6 febbraio 2026, ideologia ultranazionalista, euroscettica e di destra radicale, più a destra della Lega (`lrgen` 8,67, che col troncamento dà il posto 6) [INF su Wikipedia «National Future», letta il 23 ottobre 2026]. Se la si mettesse al posto 6 (come la Lega) i posti 6 e 7 diventerebbero 40,26 e 0: è la scelta con più peso nel risultato (7,26 punti) e ha la conseguenza di svuotare il posto 7.

## Sensibilità e punti fragili

Col troncamento le soglie che contano sono i confini di classe del taglio (2, 4, 5, 6, 7, 9). Distanze dal confine più vicino, in punti di `lrgen`:

- **Europa Verde** (2,1429) sta a 0,14 sopra il 2: scendendo sotto passerebbe al posto 1 e AVS intera sarebbe al posto 1. È il valore più vicino a un confine, ma sposta al massimo 3,165 punti fra i posti 1 e 2.
- **Forza Italia** (6,7333) sta a 0,27 sotto il 7: se un'altra release del file la portasse a 7 passerebbe al posto 6, con Noi Moderati al seguito (8,42 punti dal 5 al 6).
- **Lega** (8,6667) sta a 0,33 sotto il 9: a 9 passerebbe al posto 7 (5,71 punti dal 6 al 7).
- **+Europa** (4,3333) sta a 0,33 sopra il 4: scendendo sotto passerebbe al posto 2.
- **FdI** (8,4667) non è più fragile: dista 0,53 dal 9 e 1,47 dal 7. Con l'arrotondamento era a 0,03 dalla soglia di 8,5.
- Il valore 2024 qui usato è `CHES_2024_final_v2.csv`: un'altra release può spostare più partiti insieme.
- **Posti vuoti**: con questa mappa nessun partito ha `lrgen` 5, quindi il posto 4 è senza partiti reali e prende gli «altri». Il posto 1 è pieno solo grazie alla metà di AVS. Il gioco ha 7 partiti IA, uno per posto: la taratura da sondaggi reali dà quote non nulle a tutti, ma ai posti 1 e 4 la quota è una scelta di costruzione, non una misura [INF].
- La media di 11 istituti sposta le quote per posto di meno di 0,2 punti rispetto alla media semplice dei 19 sondaggi [M].

## Coalizioni e governo ricondotti ai posti

Lo scenario di base ha `coalizioni = [["sinistra","centrosinistra"],["centrodestra","destra"]]`, cioè i posti 2-3 e 5-6 (`dati/base/scenario.toml:12`), e un governo con presidente `destra`, otto dicasteri `destra` e due `centrodestra`, cioè i posti 6 e 5 (`dati/base/scenario.toml:15-17`) [V]. Una versione precedente di questa pagina dava per scontati i posti 1-2 e 6-7 senza aver letto lo scenario: era sbagliato, come ha rilevato la revisione del codice.

Con la mappa per troncamento:

- **Centrodestra 2022** (FdI, Lega, FI, Noi Moderati): posti 6, 6, 5, 5. Tutti dentro {5,6}: la riconduzione regge per intero [M su CHES per FdI, Lega e FI; INF per Noi Moderati].
- **Centrosinistra 2022** (PD, AVS, +Europa, Impegno Civico): PD al posto 2, +Europa al 3, entrambi dentro {2,3}; AVS per metà al 2 e per metà al 1; Impegno Civico non è in CHES. Regge tutto tranne la metà di AVS al posto 1 [M su CHES].
- **M5S** corse da solo nel 2022 e sta al posto 2: dentro la coalizione dei posti, fuori dall'alleanza 2022 [M].
- **Azione-Italia Viva** (Terzo Polo 2022) sta al posto 3, dentro {2,3} per posizione ma fuori da ogni alleanza [M].
- **Campo largo 2026** (PD, M5S, AVS, IV, +E): posti 2, 2, 1-2, 3, 3. I posti 2 e 3 valgono insieme 43,135 punti, che comprendono Azione (3,16) fuori dal campo largo; il posto 1 aggiunge 3,165.
- **Centrodestra 2026** (FdI, Lega, FI, NM): tutto in {5,6}, 41,42 punti. Futuro Nazionale (7,26) sta al posto 7, fuori da coalizione e governo. Il posto 4 (5,02, «altri») non ha coalizione [INF: somme dei bersagli].

## Per il nostro gioco

- Il vettore dei sette bersagli è fissato in `strumenti/taratura/bersagli.toml` e rimane identico finché non cambia la fonte; `sim-cli tara` lo usa per riscrivere le valence dei sette partiti in `dati/base/scenario.toml` e le astensioni per regione.
- La corrispondenza fra partiti veri e posti si fa offline e non entra nei dati del gioco, come chiede la spec (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:284`).
- Esito delle questioni lasciate aperte nella prima stesura: AVS si divide a metà fra i posti 1 e 2; gli «altri» vanno al posto 4; l'astensione resta [NON TROVATO] e si gestisce con la costante del gioco; il troncamento è la regola di mappa.
- Da aggiornare quando esce un mese nuovo di sondaggi: ricalcolare le medie con la procedura di «Media per partito» e rilanciare `sim-cli tara`. Da rivedere se esce una nuova release CHES: i valori vicini ai confini sono elencati nella sezione «Sensibilità».
- Restano aperti: il posizionamento di Futuro Nazionale (nessun dato CHES) e la quota di astensione dei sondaggi.

## Licenza

Dati di partenza: tabelle dei sondaggi su Wikipedia, testo in CC BY-SA 4.0 [INF: licenza generale del progetto, non riletta pagina per pagina]. Nel repo non è copiata nessuna tabella: solo le undici medie e le somme per posto, valori derivati con la revisione citata. CHES: nessuna licenza dichiarata, quindi si tengono nel repo i valori derivati (undici `lrgen` e gli interi) e non il file, come in [[ches]].

## Collegamenti

- [[ches]]: fonte delle collocazioni `lrgen` e vincolo di non ridistribuire il file.
- [[risultati-elettorali-italia]]: l'altro bersaglio di `sim-cli tara`, l'affluenza 2022 per regione.
- [[itanes]]: fonte delle stime di beta e fedeltà con cui si tarano le stesse valence.

## Riferimenti

- **URL** https://en.wikipedia.org/w/index.php?title=Opinion_polling_for_the_next_Italian_general_election&oldid=1381481066&action=raw · [M]
  **perché conta qui** raccolta di tutti i sondaggi nazionali di intenzione di voto per le prossime politiche, con le quote dei partiti per ogni istituto (SWG, Only Numbers, Termometro Politico, EMG, Eumetra, BiDiMedia, Lab21, Sigma Piave, Quorum, Piepoli, Noto) e le medie di YouTrend/Agi, BiDiMedia, Doppia Cifra, PolitPro
  **cosa se ne prende** le quote per partito dei 19 sondaggi di ottobre 2026 e la Supermedia del 22 ottobre per il controllo; nessuna tabella copiata nel repo. Wikitext della revisione 1381481066 del 2026-10-22T20:22:36Z, 691.148 byte, sha256 `b98af3840c1e02b25a9bc40cfc7efda31fd57f27f6fbf5ebefe38d0bffe212c6`, riscaricato con `curl` e verificato lo stesso 2026-10-23. Non salvato in `fonti/`: sono tabelle che la regola del repo esclude; l'URL con `oldid` basta a riottenerlo.
- **URL** https://www.chesdata.eu/ches-europe · [V]
  **perché conta qui** pagina del dataset CHES con il file di dati 2024 (`CHES_2024_final_v2.csv`, github.com/chesdata/chesdata.github.io/releases/download/ches-europe/) e il codebook; nessuna licenza dichiarata
  **cosa se ne prende** la colonna `lrgen` dei partiti italiani (codice paese 8) per la tabella di corrispondenza; il file CSV (99.668 byte, sha256 `1c1ec0532afa2a0a13317122cbbe40eb9ff35425191892d1fff24fbef6acc6a8`) è stato letto fuori dal repo e non vi è stato copiato, come da `docs/ricerca/dati/ches.md:18-29` [M]
- **URL** https://en.wikipedia.org/wiki/National_Future · [V]
  **perché conta qui** unica fonte letta su Futuro Nazionale (nata il 6 febbraio 2026, ideologia, rapporto con la Lega), partito senza dati CHES
  **cosa se ne prende** la motivazione [INF] che lo mette al posto 7; pagina letta tramite WebFetch il 2026-10-23, non salvata in `fonti/` (sintesi dello strumento, non testo integrale)
- **URL** https://www.agi.it/agiprima/supermedia-agiyoutrend-bene-fdi-fn-cala-csinistra-avanti-agi:sur:39656065/ · [M]
  **perché conta qui** Supermedia YouTrend/Agi del 22 ottobre 2026, media indipendente di istituti
  **cosa se ne prende** controllo della stima: stessa mappa e rinormalizzazione danno 0 / 41,1 / 7,5 / 0 / 0 / 38,3 / 13,1; letta solo come riga della tabella Wikipedia, la pagina Agi non è stata aperta
- **URL** docs/ricerca/dati/ches.md:18-29 (file del repo) · [V]
  **perché conta qui** licenza e uso di CHES già valutati dal progetto
  **cosa se ne prende** vincolo: file CHES solo offline, in repo vanno solo i valori derivati (qui: undici valori di `lrgen` e gli interi)
- **URL** dati/base/scenario.toml:12 e :15-17, docs/superpowers/specs/2026-10-21-v1-giocabile.md:277 (file del repo) · [V]
  **perché conta qui** definiscono coalizioni e governo uscente dello scenario, cioè quali posti formano un blocco
  **cosa se ne prende** i posti 2-3 e 5-6 come coalizioni, e la ragione del troncamento al posto dell'arrotondamento
