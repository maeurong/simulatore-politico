---
tipo: dataset
fonti: []
aggiornato: 2026-10-23
---
# Sondaggi nazionali di ottobre 2026, sommati sui 7 posti

Media delle intenzioni di voto nazionali di ottobre 2026 per partito reale, ricondotta ai 7 posti dell'asse sinistra-destra con la collocazione CHES 2024. Serve alla taratura della valence dei sette partiti al turno zero (spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md:284-285`). Nessuna premessa sul codice del gioco.

**Copertura.** Ottobre 2026 è coperto fino al 20 ottobre (dati letti il 23 ottobre 2026): 19 sondaggi di 11 istituti. Il mese non è chiuso: restano fuori le rilevazioni dal 21 al 31 ottobre.

## Risultato

Quote rinormalizzate sui votanti dei 7 posti (somma = 100). Media dei 11 istituti, ciascuno prima mediato su suoi sondaggi del mese.

| Posto | Partiti (collocazione) | Quota |
|---|---|---|
| 1 | nessuno con `lrgen` 0-1 (AVS cade al 2, vedi sotto) | 0,0% (3,3% se AVS si divide) |
| 2 | PD, M5S, AVS | 41,2% |
| 3 | Azione, +Europa, Italia Viva [INF] | 7,5% |
| 4 | nessuno (`lrgen` 5) | 0,0% |
| 5 | nessuno (`lrgen` 6) | 0,0% |
| 6 | FdI, Forza Italia, Noi Moderati [INF] | 37,6% |
| 7 | Lega, Futuro Nazionale [INF] | 13,7% |
| astensione e indecisi | non nelle tabelle degli istituti | [NON TROVATO] |

Variante «AVS divisa» (metà a posto 1 e metà a posto 2): 3,3 / 38,0 / 7,5 / 0 / 0 / 37,6 / 13,7.

Controllo incrociato con la Supermedia YouTrend/Agi del 22 ottobre (FdI 27,9, PD 20,3, M5S 12,1, Lega 5,6, FI 7,3, Azione 3,3, IV 2,4, AVS 6,5, +E 1,4, NM 1,1, FN 6,8), stessa mappa e stessa rinormalizzazione: 0 / 41,1 / 7,5 / 0 / 0 / 38,3 / 13,1 [M]. Scarto massimo 0,8 punti sulla stima di sopra. Media semplice sui 19 sondaggi (senza mediare prima per istituto): 41,2 / 7,5 / 37,7 / 13,7 [M].

## Media per partito

Media semplice sugli 11 istituti, punti di intenzione di voto sul totale dei rispondenti con preferenza, prima della rinormalizzazione [M: `/tmp/w2/calc.py` su wikitext di Wikipedia, vedi Riferimenti].

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
| Noto | 1 (6 ott) | non indicato; senza +Europa, la sua media su +E è calcolata sugli altri 10 istituti |

I numeri per sondaggio non si copiano qui: sono nella pagina Wikipedia citata, alla revisione indicata.

## «Altri» e indecisi

- Gli istituti pubblicano quote già riportate a 100 sui rispondenti con preferenza; gli indecisi e gli astenuti non compaiono nelle tabelle lette [M].
- Gli 11 partiti sommano il 94,98%. Il restante 5,02% è «altri»: Sud chiama Nord (entrato nel centrodestra il 10 ottobre), Partito Liberal Democratico, Ora!, Avanti-PSI, Progetto Civico Italia, Più Uno e le liste minori [M].
- Scelta: **«altri» esclusi e quote rinormalizzate sugli 11 partiti** (divisione per 94,98). È equivalente a distribuire «altri» in proporzione ai grandi. Se invece gli «altri» finissero tutti a un solo posto (caso estremo, 5 punti) la quota di quel posto salirebbe di meno di 5 punti [INF].
- Astensione e indecisi: **[NON TROVATO]** nelle tabelle consultate; una ricerca sulla quota SWG di ottobre 2026 non l'ha restituita. Il gioco ha una costante d'astensione propria (commit `67a812e`, non letta qui), quindi la taratura confronta le quote fra votanti.

## Corrispondenza partito -> posto

CHES 2024, file `CHES_2024_final_v2.csv`, Italia = codice paese 8, colonna `lrgen` [M: letto offline in sessione, vedi Riferimenti]. Arrotondamento all'intero a metà verso l'alto, poi taglio del piano: `{0,1}->1, {2,3}->2, 4->3, 5->4, 6->5, {7,8}->6, {9,10}->7`.

| Partito | `lrgen` grezzo | Intero | Posto | Tag |
|---|---|---|---|---|
| Fratelli d'Italia (FDI) | 8,4667 | 8 | 6 | [M] |
| Lega | 8,6667 | 9 | 7 | [M] |
| Forza Italia (FI) | 6,7333 | 7 | 6 | [M] |
| PD | 2,9333 | 3 | 2 | [M] |
| M5S (nel file «MS5») | 3,3333 | 3 | 2 | [M] |
| Azione (A) | 4,4286 | 4 | 3 | [M] |
| +Europa (+E) | 4,3333 | 4 | 3 | [M] |
| Sinistra Italiana (SI) | 1,2308 | 1 | 1 | [M] |
| Europa Verde (EV) | 2,1429 | 2 | 2 | [M] |
| AVS (lista unica SI + EV) | 1,6868 (media dei due, voti 2022 uguali 1,8 e 1,8) | 2 | 2 | [INF] |
| Italia Viva | assente da CHES 2024 | | 3 | [INF] |
| Noi Moderati | assente | | 6 | [INF] |
| Futuro Nazionale | assente (nata il 6 febbraio 2026) | | 7 | [INF] |

Motivazioni dei tre [INF] e dell'AVS:

- **AVS** è una lista sola nel sondaggio ma due partiti in CHES. La media semplice dei due (voti 2022 pari) dà 1,69 -> 2 -> posto 2. Se invece si pesa metà e metà per partito, mezza AVS (3,3 punti rinormalizzati) va al posto 1. Va deciso dal controller: il posto 1 resta vuoto o no.
- **Italia Viva** al posto 3: nel 2022 correva con Azione (lista unica «Terzo Polo», CHES 2024 ha solo Azione); centrista liberale, accanto ad Azione (4,43) e +Europa (4,33) [INF].
- **Noi Moderati** al posto 6: è nel centrodestra con FI, fonte democristiana; lo stesso intero di FI (7) la porta al posto 6. Pesa 1,2%, quindi l'errore massimo è piccolo [INF].
- **Futuro Nazionale** al posto 7: nata da una scissione dalla Lega il 6 febbraio 2026, ideologia ultranazionalista, euroscettica e di destra radicale, più a destra della Lega (`lrgen` 8,67 -> 9, posto 7) [INF su Wikipedia «National Future», letta il 23 ottobre 2026]. Se la si mettesse al posto 6 (come FdI) i posti 6 e 7 diventerebbero 45,3 e 6,0: è la scelta con più peso nel risultato (7,65 punti).

## Sensibilità e punti fragili

- **FdI a 8,4667**: a 0,03 dalla soglia di 8,5. Con un arrotondamento a metà in su su un valore appena diverso (per esempio un'altra release del file CHES), FdI passerebbe da 8 a 9 e dal posto 6 al posto 7: i posti 6 e 7 diventerebbero 8,9 e 42,4. Il valore 2024 qui usato è `CHES_2024_final_v2.csv`.
- **Posti 1, 4 e 5 vuoti** con questa mappa (posto 1 pieno solo con AVS divisa). Il centro (4-5) non ha partiti nelle intenzioni di voto: Azione e +Europa stanno a 4 (posto 3), nessun partito sta a 5 o 6 di `lrgen`. Il gioco ha 7 partiti IA, uno per posto: la taratura da sondaggi reali non può dare quote a tre di essi [INF]. Il controller deve dichiarare come tratta i posti vuoti.
- La media di 11 istituti sposta le quote per posto di meno di 0,2 punti rispetto alla media semplice dei 19 sondaggi [M].

## Coalizioni del 2022 ricondotte ai posti

Il gioco ha `[["sinistra","centrosinistra"],["centrodestra","destra"]]`. Assumendo che i nomi siano i posti 1-2 e 6-7 in ordine [INF, non letto nel codice]:

- **Centrodestra 2022** (FdI, Lega, FI, Noi Moderati): posti 6, 7, 6, 6. Tutti dentro {6,7}: **la riconduzione regge** per intero [M su CHES per FdI/Lega/FI; INF per Noi Moderati].
- **Centrosinistra 2022** (PD, AVS, +Europa, Impegno Civico): PD 2 e AVS 2 sono dentro {1,2}; +Europa è al posto 3 (4,33 -> 4); Impegno Civico non è in CHES. **Regge in parte**: +Europa, 2,83% dei voti 2022 nel file CHES contro 25,47% del blocco misurato, cioè l'11% del blocco, cade fuori. Il resto sta nei posti 1-2 [M].
- **M5S** corse da solo nel 2022 e sta al posto 2: dentro «sinistra» come posto, fuori dalla coalizione 2022 [M].
- **Azione-Italia Viva** (Terzo Polo 2022, 7,78% in CHES) sta al posto 3, fuori da entrambe le coalizioni [M].
- **Campo largo 2026** (PD, M5S, AVS, IV, +E): posti 2, 2, 2, 3, 3; dentro {1,2} il 41,2% su 45,4% dell'insieme. Il **centrodestra 2026** (FdI, Lega, FI, NM) sta tutto in {6,7}: 43,6% senza Futuro Nazionale, 51,3% con (la quota di FN cade in {6,7}). Il posto 3 (7,5%) non ha coalizione: è l'unico che il giocatore al centro può prendere a entrambe [INF].

## Riferimenti

- **URL** https://en.wikipedia.org/wiki/Opinion_polling_for_the_next_Italian_general_election · [M]
  **perché conta qui** raccolta di tutti i sondaggi nazionali di intenzione di voto per le prossime politiche, con le quote dei partiti per ogni istituto (SWG, Only Numbers, Termometro Politico, EMG, Eumetra, BiDiMedia, Lab21, Sigma Piave, Quorum, Piepoli, Noto) e le medie di YouTrend/Agi, BiDiMedia, Doppia Cifra, PolitPro
  **cosa se ne prende** le quote per partito dei 19 sondaggi di ottobre 2026 e la Supermedia del 22 ottobre per il controllo; nessuna tabella copiata nel repo. Letto come wikitext (`action=raw`), revisione 1381481066 del 2026-10-22T20:22:36Z, 691.148 byte, sha256 `b98af3840c1e02b25a9bc40cfc7efda31fd57f27f6fbf5ebefe38d0bffe212c6`, estratto con `python3 -I` in `/tmp` il 2026-10-23. Non salvato in `fonti/`: sono tabelle che la regola del repo esclude.
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
  **perché conta qui** licenza e uso di CHES già valutati dal progetto (`docs/ricerca/dati/ches.md:18-29`)
  **cosa se ne prende** vincolo: file CHES solo offline, in repo vanno solo i valori derivati (qui: 11 valori di `lrgen` e gli interi)
