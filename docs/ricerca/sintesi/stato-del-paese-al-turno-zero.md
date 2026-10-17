---
tipo: sintesi
fonti: [fonti/mef-dpfp-2026.md, fonti/istat-cet-trimestrali-q2-2026-revisione.md, fonti/istat-conto-ap-q2-2026.md, fonti/bdi-fpi-2026-10-15.md, fonti/bdi-copyright.md, fonti/senato-camera-dossier-lb2026-vol5.md, fonti/rgs-bilancio-in-breve-2026-2028.md, fonti/rgs-copyright.md, fonti/inps-osservatorio-pensioni-2026.md, fonti/inps-rapporto-annuale-xxv-2026.md, fonti/inps-adi-sfl-infografica-2026-07.md, fonti/inps-open-data.md, fonti/istat-note-legali.md, fonti/eurostat-licence-policy-pdf.md, fonti/eurostat-figaro-metadata.md, fonti/istat-cet-2022-2024-comunicato.md, fonti/istat-pil-territoriale-prel-2025.md]
aggiornato: 2026-10-17
---
# Stato del paese al turno zero

I livelli di partenza con cui il motore della v1 (ticket 07) parte dall'Italia di oggi: macro e finanza pubblica, budget dei dieci dicasteri, spesa obbligatoria, struttura input-output e mix regionale. Ogni valore ha data, fonte e licenza. I coefficienti stanno in [[economia-italiana-parametri]]; qui ci sono solo i livelli. Raccomandazione in fondo: il turno zero è il **secondo trimestre 2026**, l'ultimo in cui conti trimestrali, conto delle AP e lavoro coincidono.

Le interrogazioni [M] sono state fatte il 2026-10-17 sull'API Eurostat (`https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/<dataset>?geo=IT&...`, risposta JSON-stat letta con `python3 -I`) e sul servizio SDMX ISTAT (`https://esploradati.istat.it/SDMXWS/rest/data/IT1,<flusso>,1.0/<chiave>`). Per ogni dataset è indicata la data di aggiornamento letta nella risposta.

## 1. Macro e finanza pubblica

| Grandezza | Valore | Periodo | Fonte | Licenza |
|---|---|---|---|---|
| PIL nominale, anno | 2.265,0 mld € | 2025 | fonti/mef-dpfp-2026.md:2816 (conti ISTAT del 22/9/2026) [V] | ISTAT CC BY 4.0 |
| PIL reale | +0,6% (2025); +0,2% t/t e +1,0% a/a nel T2 2026; acquisita 2026 +0,8% | 2026T2 | fonti/istat-cet-trimestrali-q2-2026-revisione.md:20-26 [V] | ISTAT CC BY 4.0 |
| PIL nominale trimestrale (destag.) | 576,9 mld € | 2026T2 | [M] `namq_10_gdp`, `CP_MEUR`, `SCA`, `B1GQ`, aggiornato 2026-10-16 | Eurostat, riuso con citazione |
| Consumi delle famiglie e ISP | 339,0 mld € | 2026T2 | [M] `namq_10_gdp`, `P31_S14_S15` | Eurostat |
| Consumi della PA | 104,9 mld € | 2026T2 | [M] `namq_10_gdp`, `P3_S13` | Eurostat |
| Investimenti fissi lordi | 130,5 mld € | 2026T2 | [M] `namq_10_gdp`, `P51G` | Eurostat |
| Esportazioni / importazioni | 190,1 / 185,7 mld € | 2026T2 | [M] `namq_10_gdp`, `P6`, `P7` | Eurostat |
| Debito delle AP, livello | 3.097,1 mld € (136,7% del PIL) | fine 2025 | fonti/bdi-fpi-2026-10-15.md:93-95; fonti/mef-dpfp-2026.md:4619-4620 [V] | BdI: solo uso personale (fonti/bdi-copyright.md:1) |
| Debito delle AP, mensile | 3.208,5 mld € (giugno), 3.200,8 mld € (agosto), provvisori | 2026 | fonti/bdi-fpi-2026-10-15.md:493, 497 [V] | BdI: solo uso personale |
| Debito/PIL trimestrale | 138,9% | 2026T1 | [M] `gov_10q_ggdebt`, aggiornato 2026-07-21 | Eurostat |
| Debito/PIL a fine T2 | circa 139,9% | 2026T2 | [M] debito di giugno su somma dei PIL trimestrali 2025T3-2026T2; Eurostat pubblicherà il dato ufficiale con la prossima notifica trimestrale [INF] | derivato |
| Indebitamento netto | -3,1% del PIL (2025); -2,0% nel T2 2026 (dato grezzo); -2,9% previsto per il 2026 | 2025, 2026T2 | fonti/mef-dpfp-2026.md:3527-3528; fonti/istat-conto-ap-q2-2026.md:22 [V] | ISTAT CC BY 4.0 |
| Saldo primario | +0,8% del PIL (2025); +3,0% nel T2 2026 | 2025, 2026T2 | fonti/mef-dpfp-2026.md:3523-3524; fonti/istat-conto-ap-q2-2026.md:22 [V] | ISTAT CC BY 4.0 |
| Pressione fiscale | 42,9% (2025); 43,5% nel T2 2026 | 2025, 2026T2 | fonti/mef-dpfp-2026.md:3521; fonti/istat-conto-ap-q2-2026.md:24 [V] | ISTAT CC BY 4.0 |
| Spesa per interessi | 87,2 mld € (3,85% del PIL) nel 2025; 96,9 mld previsti nel 2026 | 2025 | fonti/mef-dpfp-2026.md:3500, 4903 [V] | MEF: [NON TROVATO] |
| Costo medio del debito | 2,81% | 2025 | [M] interessi 87.158 / debito 3.097.093, `python3 -I -c` | derivato |
| Vita media residua del debito delle AP | 7,8 anni | agosto 2026 | fonti/bdi-fpi-2026-10-15.md:746, 807 [V] | BdI: solo uso personale |
| Rendimento BTP decennale | 3,80% (media T2 2026); 4,36% a settembre | 2026 | [M] `irt_lt_mcby_m`, aggiornato 2026-10-13 | Eurostat |
| Spread BTP-Bund decennale | 79 pb (media T2 2026); 83 pb (media T3); 89 pb a settembre | 2026 | [M] stesso dataset, IT meno DE, medie mensili | Eurostat |
| Tasso BCE sui depositi | 2,50% dopo il rialzo di 25 pb di settembre | settembre 2026 | fonti/mef-dpfp-2026.md:917-918 [V] | MEF: [NON TROVATO] |
| Inflazione IPCA | 3,0% (media T2 2026); 4,2% a settembre; 3,4% media T3 | 2026 | [M] `prc_hicp_minr`, `RCH_A`, aggiornato 2026-10-16; coerente con fonti/mef-dpfp-2026.md:1989-1992 [V] | Eurostat |
| Disoccupazione nazionale | 6,1% (2025); 5,6% nel T2 2026; 6,2% ad agosto (mensile destag.) | 2025-2026 | fonti/mef-dpfp-2026.md:1457-1459 [V]; [M] `une_rt_m`, aggiornato 2026-10-16 | ISTAT / Eurostat |
| Occupati | 24,4 milioni, massimo storico | 2026T2 | fonti/mef-dpfp-2026.md:1446-1450 [V] | ISTAT CC BY 4.0 |
| Gettito: imposte sul reddito delle persone fisiche / società / IVA | 273,1 / 60,7 / 150,4 mld € | 2025 | [M] `gov_10a_taxag`, `D51A_C1`, `D51B_C2`, `D211`, settore `S13_S212`, aggiornato 2026-07-21 (precede la revisione ISTAT di settembre) | Eurostat |
| Contributi sociali effettivi | 301,4 mld € | 2025 | fonti/mef-dpfp-2026.md:3515 [V] | ISTAT CC BY 4.0 |
| Popolazione residente | 58.942.828 | 1° gennaio 2026 (stima) | [M] SDMX ISTAT `22_289_DF_DCIS_POPRES1_1`, chiave `A.IT.JAN.9.TOTAL.99`, stato `e` | ISTAT CC BY 4.0 |

Due avvertenze. Il debito 2025 al 137,1% citato in [[regole-fiscali-ue]] è precedente alla revisione ISTAT del 22 settembre 2026: con il PIL rivisto il rapporto scende al 136,7% (fonti/bdi-fpi-2026-10-15.md:93-95) [V]. La disoccupazione mensile (6,0% a giugno, 6,2% ad agosto) e quella trimestrale (5,6% nel T2) vengono da due elaborazioni diverse della stessa indagine e non vanno mescolate: per il turno zero vale la trimestrale [INF].

**Disoccupazione per regione** (15-74 anni, 2025, %) [M] `lfst_r_lfu3rt`, aggiornato 2026-09-10, Eurostat:

| Nord | % | Centro | % | Mezzogiorno | % |
|---|---|---|---|---|---|
| Piemonte | 6,0 | Toscana | 4,3 | Abruzzo | 6,5 |
| Valle d'Aosta | 3,9 | Umbria | 4,6 | Molise | 6,2 |
| Liguria | 4,7 | Marche | 5,1 | Campania | 13,9 |
| Lombardia | 3,0 | Lazio | 5,5 | Puglia | 10,0 |
| Bolzano | 1,8 | | | Basilicata | 5,3 |
| Trento | 2,2 | | | Calabria | 9,8 |
| Veneto | 3,4 | | | Sicilia | 12,2 |
| Friuli-Venezia Giulia | 4,6 | | | Sardegna | 9,3 |
| Emilia-Romagna | 4,1 | | | Italia | 6,1 |

Il Trentino-Alto Adige compare come due province autonome (livello NUTS 2); per le 20 regioni del gioco serve la media pesata sulle forze di lavoro [INF]. Un dato regionale trimestrale per il 2026 non è stato interrogato [NON TROVATO].

## 2. Budget dei dieci dicasteri

### Il bilancio dello Stato si riconduce ai dieci dicasteri, ma non misura la loro spesa

La legge di bilancio 2026 (L. 199/2025) autorizza 923,1 miliardi di spese finali, divise fra 16 ministeri (fonti/senato-camera-dossier-lb2026-vol5.md:186-228) [V]. Accorpati come nel ticket 03 [M, somme con `python3 -I -c`, il totale torna a 923.116 milioni]:

| Dicastero | Ministeri | Spese finali 2026 (mln €) | Quota |
|---|---|---|---|
| Economia e Finanze | MEF | 548.030 | 59,4% |
| Lavoro e Welfare | Lavoro | 186.214 | 20,2% |
| Istruzione e Ricerca | Istruzione e merito, Università, Cultura | 74.917 | 8,1% |
| Esteri e Difesa | Esteri, Difesa | 36.250 | 3,9% |
| Interno | Interno | 30.329 | 3,3% |
| Infrastrutture e Trasporti | MIT | 18.547 | 2,0% |
| Sviluppo economico | Imprese, Agricoltura, Turismo | 11.402 | 1,2% |
| Giustizia | Giustizia | 11.404 | 1,2% |
| Ambiente ed Energia | MASE | 3.444 | 0,4% |
| Sanità | Salute | 2.579 | 0,3% |

La tabella non serve come budget dei dicasteri. Il MEF contiene gli interessi (108,9 miliardi), il finanziamento della sanità regionale (oltre 93,2 miliardi nella missione 3) e i trasferimenti agli enti territoriali; il Lavoro contiene i trasferimenti all'INPS (missione 25, «Politiche previdenziali», 112,7 miliardi) (fonti/senato-camera-dossier-lb2026-vol5.md:1567-1597, 2777-2860) [V]. Il bilancio dello Stato è soprattutto un bilancio di trasferimenti: redditi da lavoro e consumi intermedi fanno 130,0 miliardi su 923,1, il 14,1%, mentre i trasferimenti correnti ad altre AP sono 367,9 miliardi (fonti/senato-camera-dossier-lb2026-vol5.md:1574-1577) [V; quota M]. La sanità si vede solo così: il ministero della Salute ha 2,6 miliardi, la spesa sanitaria delle AP è 141,5 miliardi nel 2025 (fonti/mef-dpfp-2026.md:3502) [V].

### Proposta: COFOG delle amministrazioni pubbliche, con le due quote

La classificazione per funzione (COFOG) della spesa di tutte le AP, incrociata con la natura economica, dà sia il budget di ogni dicastero sia le due quote del ticket 03 [INF, scelta di metodo]: è consolidata (Stato, Regioni, Comuni, enti di previdenza) e coerente con la regola delle Regioni passive, a cui la spesa nazionale arriva per popolazione ([[decisioni]]). Ultimo anno disponibile: 2024 [M] `gov_10a_exp`, `S13`, `MIO_EUR`, aggiornato 2026-09-16, Eurostat.

Riconduzione proposta [INF], con la spesa obbligatoria tolta prima: pensioni e altre prestazioni sociali in denaro (COFOG 10, voce `D62`, 445,7 miliardi) e interessi (COFOG 01.7, 87,8 miliardi).

| Dicastero | Divisioni COFOG | Spesa 2024 (mld €) | Salari D1 | Acquisti P2 + D632 | Funzionamento | Resto |
|---|---|---|---|---|---|---|
| Sanità | 07 | 146,1 | 44,6 | 90,8 | 135,4 (93%) | 10,7 |
| Istruzione e Ricerca | 09, 08, 01.4 | 120,7 | 70,2 | 18,7 | 89,0 (74%) | 31,7 |
| Economia e Finanze | 01 senza 01.2, 01.4, 01.7 | 68,4 | 23,4 | 17,7 | 41,1 (60%) | 27,2 |
| Infrastrutture e Trasporti | 04.5, 06 | 68,8 | 5,0 | 9,9 | 14,9 (22%) | 53,9 |
| Sviluppo economico | 04 senza 04.3, 04.5 | 47,3 | 4,4 | 4,5 | 8,9 (19%) | 38,4 |
| Ambiente ed Energia | 04.3, 05 | 33,1 | 2,3 | 12,9 | 15,2 (46%) | 17,9 |
| Esteri e Difesa | 02, 01.2 | 29,8 | 15,7 | 2,2 | 17,9 (60%) | 11,8 |
| Interno | 03 senza 03.3, 03.4 | 27,4 | 18,9 | 4,8 | 23,7 (87%) | 3,7 |
| Lavoro e Welfare | 10 senza D62 | 22,4 | 4,8 | 13,1 | 17,9 (80%) | 4,5 |
| Giustizia | 03.3, 03.4 | 11,7 | 7,8 | 2,6 | 10,3 (88%) | 1,4 |
| **Dieci dicasteri** | | **575,6** | | | **374,4 (65%)** | **201,2** |

[M, `python3 -I -c` sulla risposta `gov_10a_exp`; la somma dei dicasteri più D62 della divisione 10 più la divisione 01.7 torna a 1.109,2 miliardi, la spesa totale delle AP 2024.] Qui «funzionamento» vale redditi da lavoro dipendente (D1) più consumi intermedi (P2) più prestazioni sociali in natura acquistate sul mercato (D632, cioè farmaceutica e medicina convenzionata nella sanità); «resto» raccoglie investimenti, contributi alla produzione, trasferimenti correnti e in conto capitale [INF, definizione proposta]. D1 e P2 sono le due quote fisse di salari e acquisti che il ticket 07 dà a ogni dicastero.

Il «resto» non è tutto margine discrezionale: ci stanno i contributi agli investimenti già promessi da leggi pluriennali e i sussidi energetici (11,8 miliardi di contributi alla produzione nella sola divisione 04.3) [V per il dato COFOG, INF per la lettura]. Il margine che il parlamento muove davvero in un anno si misura meglio sulla manovra: 14,6 miliardi di spese finali nel 2026, l'1,6% del bilancio dello Stato, di cui 5,5 con la Sezione II e 9,1 con la Sezione I (fonti/senato-camera-dossier-lb2026-vol5.md:160-183) [V; quota M]. La legge di contabilità distingue oneri inderogabili, fattori legislativi e spese di adeguamento al fabbisogno, e solo le ultime sono quantificate «tenendo conto delle esigenze delle amministrazioni» (fonti/senato-camera-dossier-lb2026-vol5.md:453-473) [V]; la ripartizione 2026 della spesa fra le tre nature sta negli allegati agli stati di previsione e non è stata trovata in forma aggregata [NON TROVATO].

Proposta per il gioco [INF]: budget di partenza = colonna «Spesa 2024» scalata al PIL del turno zero; spesa di funzionamento = D1 + P2 (+ D632); margine discrezionale = una quota del «resto» tarata al bilanciamento, con l'1,6% della manovra come ordine di grandezza della parte che si sposta ogni anno.

### Fondo di riserva

Nel bilancio 2026 il programma 33.2 «Fondi di riserva e speciali» vale 5.333,3 milioni (fonti/senato-camera-dossier-lb2026-vol5.md:4896) [V]: è un contenitore che raccoglie più fondi, e lo stanziamento del solo fondo di riserva per le spese impreviste non è stato trovato [NON TROVATO]. La missione 8 «Soccorso civile», dove sta la Protezione civile, vale 6.576 milioni, con 872 milioni aggiunti dalla manovra (fonti/senato-camera-dossier-lb2026-vol5.md:2781) [V]. La dotazione 2026 del Fondo per le emergenze nazionali non è stata trovata su fonte primaria [NON TROVATO]. Per il gioco il fondo di riserva resta una manopola da tarare, sotto i 5,3 miliardi del programma 33.2 [INF].

## 3. Spesa obbligatoria

| Grandezza | Valore | Periodo | Fonte | Licenza |
|---|---|---|---|---|
| Spesa per pensioni nel conto delle AP | 342,5 mld € (2025); 351,8 previsti nel 2026 | 2025 | fonti/mef-dpfp-2026.md:3496 [V] | ISTAT CC BY 4.0 |
| Altre prestazioni sociali in denaro | 116,3 mld € | 2025 | fonti/mef-dpfp-2026.md:3497 [V] | ISTAT CC BY 4.0 |
| Pensioni INPS vigenti | 21.257.999, per 353,5 mld € annui; 79,2% previdenziali (325,0 mld), 20,8% assistenziali (28,5 mld) | 1° gennaio 2026 | fonti/inps-osservatorio-pensioni-2026.md:19-21 [V] | INPS: licenza dei PDF e delle notizie [NON TROVATO] |
| Pensionati | 16.388.149, reddito pensionistico 370,7 mld € (provvisorio); 15.794.919 con almeno una prestazione INPS | 2025 | fonti/inps-rapporto-annuale-xxv-2026.md:12196, 12201 [V] | INPS: [NON TROVATO] |
| Età media dei pensionati | 74,3 anni (71,9 uomini, 76,3 donne) | 1° gennaio 2026 | fonti/inps-osservatorio-pensioni-2026.md:33 [V] | INPS |
| Nuove pensioni liquidate | 1.540.943 (-1,8%), 18,6 mld € annualizzati | 2025 | fonti/inps-osservatorio-pensioni-2026.md:39-41 [V] | INPS |
| Quota retributiva / mista / contributiva dello stock | non trovata | | [NON TROVATO] nell'Osservatorio e nel XXV Rapporto annuale | |
| NASpI | 2,845 milioni di beneficiari; spesa 17,93 mld € con i contributi figurativi; entrate 7,27 mld; saldo -10,66 mld | 2025 | fonti/inps-rapporto-annuale-xxv-2026.md:6108, 6123-6127 [V] | INPS |
| Spesa per disoccupazione (COFOG 10.5, tutte le AP) | 20,3 mld €, di cui 19,9 in prestazioni in denaro | 2024 | [M] `gov_10a_exp`, `GF1005` | Eurostat |
| Assegno di inclusione | 835.078 nuclei (1,96 milioni di persone) con almeno una mensilità, importo medio mensile 754 € | 2025 | fonti/inps-rapporto-annuale-xxv-2026.md:11265 [V] | INPS |
| Assegno di inclusione, mese | 640.997 nuclei, 680 € medi (marzo 2026); per caratteristica del nucleo a giugno 2026 | 2026 | fonti/inps-rapporto-annuale-xxv-2026.md:11277, 11291; fonti/inps-adi-sfl-infografica-2026-07.md:54-72 [V] | INPS |

Il totale mensile di giugno 2026 (690 mila nuclei, 672 euro) circola nelle agenzie del 3 agosto 2026 come dato INPS, ma il comunicato primario non è stato catturato [NON TROVATO su fonte primaria]. Un ordine di grandezza della spesa annua ADI: 835 mila nuclei × 754 € × 12 ≈ 7,6 miliardi è un tetto, perché non tutti i nuclei ricevono dodici mensilità [INF].

**Piramide per coorti** [M] SDMX ISTAT `22_289_DF_DCIS_POPRES1_1`, chiave `A.IT.JAN.9..99`, 1° gennaio 2026, età singole da 0 a 99 più la classe 100 e oltre (24.710 persone), stato `e` (stima): 0-14 anni 11,6%; 15-24 10,1%; 25-44 22,4%; 45-64 30,8%; 65-79 17,3%; 80 e oltre 7,8%; 67 e oltre (età di vecchiaia) 22,4%. La serie per età singola e sesso è la base della piramide del ticket 07; la stessa interrogazione con `SEX=1` e `SEX=2` dà i due lati [INF, non interrogata per sesso].

## 4. Tavola input-output e mix regionale

| Pezzo | Ultima annata | Fonte | Licenza |
|---|---|---|---|
| FIGARO (64 prodotti, multi-paese, prezzi correnti) | 2024 (edizione 2026, 2010-2024); a prezzi dell'anno precedente attesa per dicembre 2026 | fonti/eurostat-figaro-metadata.md:8, 68 [V] | Eurostat, riuso con citazione e dichiarazione delle modifiche |
| Tavola simmetrica nazionale Eurostat per l'Italia | 2022 (la serie arriva al 2024 come asse temporale, i valori si fermano al 2022) | [M] `naio_10_cp1700`, `CPA_A01`×`CPA_A01`, aggiornato 2026-09-30 | Eurostat |
| Valore aggiunto regionale in A*10 | 2023; il 2024 solo a 6 branche | [[economia-italiana-parametri]] (sez. 5); fonti/istat-cet-2022-2024-comunicato.md [V] | ISTAT CC BY 4.0 |
| Occupazione regionale per branca | 2024, stesse 42 branche | [[economia-italiana-parametri]] (sez. 5) [M] | ISTAT CC BY 4.0 |

La matrice A a 10 settori si ricava da FIGARO 2024 aggregando i 64 prodotti in A*10, come stabilito nel ticket 07; la tavola nazionale del 2022 serve da controllo [INF]. Il mix regionale si fissa sulle quote di valore aggiunto A*10 del 2023 e si porta al turno zero con la crescita per macroarea della stima preliminare 2025 (fonti/istat-pil-territoriale-prel-2025.md) [INF].

## 5. Le date sono compatibili? Quale trimestre è il turno zero

| Blocco | Ultimo dato a metà ottobre 2026 |
|---|---|
| Conti trimestrali (PIL e componenti) | 2026T2, coerente con la revisione annuale del 22/9 (fonti/istat-cet-trimestrali-q2-2026-revisione.md:18) |
| Conto trimestrale delle AP | 2026T2 (fonti/istat-conto-ap-q2-2026.md:22) |
| Debito | agosto 2026 (Banca d'Italia, provvisorio); 2026T1 su Eurostat |
| Disoccupazione | 2026T2 trimestrale; agosto 2026 mensile; regioni al 2025 |
| Prezzi, spread, tasso BCE | settembre 2026 |
| Conto annuale delle AP | 2025 (notifica del 30 settembre) |
| Legge di bilancio | 2026, anno in corso |
| COFOG | 2024 |
| Pensioni, NASpI, ADI | 1° gennaio 2026; 2025; marzo-giugno 2026 |
| Popolazione | 1° gennaio 2026 |
| Input-output | FIGARO 2024; nazionale 2022 |
| Mix regionale | 2023 (A*10) |

Le annate non coincidono, ma non devono: i dati si dividono in due famiglie [INF].

- **Livelli di flusso e di stock**, che il motore aggiorna a ogni turno (PIL, componenti, deficit, debito, disoccupazione, inflazione, spread): arrivano tutti almeno al 2026T2.
- **Strutture e quote**, che il motore tiene fisse o fa muovere lentamente (matrice A, mix regionale, quote COFOG dei dicasteri, salari e acquisti, pensioni per coorte): bastano annate più vecchie, scalate ai livelli del turno zero.

**Raccomandazione [INF]: turno zero = 2026T2.** È l'ultimo trimestre in cui conti trimestrali, conto delle AP e mercato del lavoro escono insieme e coerenti con la stessa revisione ISTAT. Il debito di fine giugno (3.208,5 miliardi) c'è già. Il 2026T3 avrà il PIL preliminare a fine ottobre e il conto delle AP solo a gennaio 2027 [INF dal calendario abituale ISTAT, non verificato]: aspettarlo vuol dire partire con un trimestre in cui metà dei blocchi è stimata. Gli indicatori mensili di luglio-settembre (inflazione al 4,2%, spread a 89 pb, BCE al 2,50%, disoccupazione mensile in salita) entrano come primo shock del turno 1, non come stato iniziale. Il turno 1 è così il 2026T3, e il turno ottobre-dicembre della legge di bilancio arriva al turno 2.

> **Nota del 17/10/2026, ticket «Partenza della partita» (20):** il calendario di gioco resta un altro. Il turno zero è ottobre-dicembre 2026, perché la legge 180/2026 vale dal 24/10 e il primo voto deve cadere dopo. Di questa raccomandazione resta l'annata dei dati: stato base al 2026T2, portato all'inizio di ottobre con gli indicatori mensili di luglio-settembre trattati come dati osservati, non come shock del turno 1.

Un'avvertenza sul contesto: il turno zero cade dentro lo shock energetico seguito alla guerra in Medio Oriente di febbraio 2026, con l'IPCA dal 1,0% di gennaio al 4,2% di settembre e la BCE che rialza (fonti/mef-dpfp-2026.md:907-918) [V]. Lo stato iniziale non è un equilibrio: l'IS-Phillips parte con l'inflazione sopra l'obiettivo e il ciclo in rallentamento nell'industria (-0,6% del valore aggiunto nel T2, fonti/istat-cet-trimestrali-q2-2026-revisione.md:30) [V].

## Licenze, in sintesi

- **ISTAT**: CC BY 4.0 salvo diversa indicazione (fonti/istat-note-legali.md) [V]. Copre conti nazionali, conto delle AP, popolazione, conti territoriali.
- **Eurostat**: riuso con citazione, anche commerciale, con l'obbligo di dichiarare le modifiche (fonti/eurostat-licence-policy-pdf.md) [V]. È la strada per mettere nel repo debito, deficit, COFOG, IPCA, rendimenti e FIGARO.
- **Banca d'Italia**: le pubblicazioni del sito sono per solo uso personale; fanno eccezione gli open data su dati.gov.it, in CC BY 4.0 (fonti/bdi-copyright.md:1-3) [V]. Il debito mensile e la vita media si usano per tarare, ma nel repo conviene tenere il debito trimestrale Eurostat [INF].
- **MEF (DPFP)**: licenza del sito non trovata [NON TROVATO]. Quasi tutti i suoi numeri di consuntivo sono dati ISTAT, che si possono prendere dalla fonte ISTAT [INF].
- **RGS**: riuso senza fini di lucro con citazione; a fini di lucro serve un permesso scritto (fonti/rgs-copyright.md) [V]. Il gioco è gratuito ([[decisioni]]).
- **Dossier di Senato e Camera**: «i contenuti originali possono essere riprodotti, nel rispetto della legge, a condizione che sia citata la fonte» (fonti/senato-camera-dossier-lb2026-vol5.md:44-45) [V].
- **INPS**: gli open data seguono la licenza indicata in ciascuna scheda, con rinvio alla IODL 2.0 (fonti/inps-open-data.md:23) [V]; per rapporti, osservatori in PDF e notizie una licenza non è indicata [NON TROVATO].

## Per il nostro gioco

- Prendiamo: livelli al 2026T2 da ISTAT ed Eurostat; budget dei dicasteri da COFOG 2024 con le due quote D1 e P2; spesa obbligatoria da conto delle AP 2025 e INPS; matrice A da FIGARO 2024; mix regionale da A*10 2023.
- Evitiamo: il bilancio dello Stato per ministero come budget dei dicasteri, perché metà della sanità, delle pensioni e degli enti locali passa da trasferimenti del MEF e del Lavoro.
- Aperti: quota retributiva e contributiva dello stock di pensioni; ripartizione 2026 fra oneri inderogabili, fattori legislativi e fabbisogno; stanziamento del fondo per le spese impreviste e del Fondo per le emergenze nazionali; disoccupazione regionale trimestrale 2026.

## Fonti

- **URL** https://www.mef.gov.it/export/sites/MEF/inevidenza/2026/article_00390/DPFP26.pdf · [V]
  **perché conta qui** unico documento che mette insieme conto delle AP 2025 rivisto, quadro macro e tassi del 2026
  **cosa se ne prende** PIL 2025 e componenti, conto delle AP (pensioni, interessi, spesa sanitaria, entrate), BCE al 2,50%, lavoro e prezzi del T2 (fonti/mef-dpfp-2026.md, cattura di un altro agente)
- **URL** https://www.istat.it/comunicato-stampa/conti-economici-trimestrali-revisione-serie-ii-trimestre-2026/ · [V]
  **perché conta qui** ultimo trimestre dei conti nazionali coerente con la revisione di settembre
  **cosa se ne prende** PIL T2 2026 +0,2% t/t, +1,0% a/a, acquisita 0,8%, valore aggiunto per macrosettore (fonti/istat-cet-trimestrali-q2-2026-revisione.md)
- **URL** https://www.istat.it/?p=136588 · [V]
  **perché conta qui** deficit e saldo primario trimestrali per lo stesso trimestre del turno zero
  **cosa se ne prende** indebitamento -2,0%, primario +3,0%, pressione fiscale 43,5%, propensione al risparmio 6,7% (fonti/istat-conto-ap-q2-2026.md)
- **URL** https://www.bancaditalia.it/pubblicazioni/finanza-pubblica/2026-finanza-pubblica/statistiche_FPI_20261015.pdf · [V]
  **perché conta qui** debito mensile più recente e vita media residua, rivisti con la notifica del 30 settembre
  **cosa se ne prende** debito 2025 a 3.097,1 miliardi (136,7%), giugno e agosto 2026, vita media 7,8 anni (fonti/bdi-fpi-2026-10-15.md)
- **URL** https://www.bancaditalia.it/footer/copyright/index.html · [V]
  **perché conta qui** decide se i numeri della Banca d'Italia possono entrare nel repo
  **cosa se ne prende** solo uso personale, eccetto gli open data in CC BY 4.0 (fonti/bdi-copyright.md)
- **URL** https://www.senato.it/service/PDF/PDFServer/BGT/1493538.pdf · [V]
  **perché conta qui** legge di bilancio 2026 per ministero, missione e categoria economica
  **cosa se ne prende** spese finali per ministero, tabella delle missioni, categorie economiche, manovra di 14,6 miliardi, programma 33.2 dei fondi di riserva (fonti/senato-camera-dossier-lb2026-vol5.md)
- **URL** https://www.rgs.mef.gov.it/_Documenti/VERSIONE-I/attivita_istituzionali/formazione_e_gestione_del_bilancio/bilancio_di_previsione/bilancio_in_breve/2026/Il-bilancio-dello-Stato-2026-2028-in-breve.pdf · [V]
  **perché conta qui** quadro generale riassuntivo ufficiale RGS, controllo del dossier parlamentare
  **cosa se ne prende** spese finali 923.117 milioni, interessi 108.908, saldo netto da finanziare -154,3 miliardi (fonti/rgs-bilancio-in-breve-2026-2028.md:279-300)
- **URL** https://www.inps.it/it/it/inps-comunica/notizie/dettaglio-news-page.news.2026.03.online-l-osservatorio-sulle-pensioni-erogate-dall-istituto.html · [V]
  **perché conta qui** stock delle pensioni al 1° gennaio 2026, per la prima volta con i dipendenti pubblici
  **cosa se ne prende** 21,26 milioni di pensioni, 353,5 miliardi, previdenziali e assistenziali, età media (fonti/inps-osservatorio-pensioni-2026.md)
- **URL** https://www.inps.it/content/dam/inps-site/pdf/dati-analisi-bilanci/rapporti-annuali/xxv-rapporto-annuale/Rapporto_annuale_XXV_2026.pdf · [V]
  **perché conta qui** consuntivo 2025 di NASpI, ADI e pensionati
  **cosa se ne prende** NASpI 2,845 milioni e 17,93 miliardi; ADI 835 mila nuclei e 754 euro; 16,39 milioni di pensionati (fonti/inps-rapporto-annuale-xxv-2026.md)
- **URL** https://www.inps.it/content/dam/inps-site/pdf/dati-analisi-bilanci/osservatori-statistici/osservatorio-adi-sfl/Infografiche_ADI_SFL_2026_07.pdf · [V]
  **perché conta qui** dato ADI più recente per regione e tipo di nucleo
  **cosa se ne prende** nuclei e importi di giugno 2026 per caratteristica, cumulato per regione (fonti/inps-adi-sfl-infografica-2026-07.md)
- **URL** https://www.inps.it/it/it/dati-e-bilanci/open-data.html · [V]
  **perché conta qui** licenza dei dati INPS
  **cosa se ne prende** licenza per scheda di dataset, rinvio alla IODL 2.0 (fonti/inps-open-data.md)
- **URL** https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/ (dataset `namq_10_gdp`, `gov_10q_ggdebt`, `gov_10q_ggnfa`, `gov_10a_exp`, `gov_10a_taxag`, `prc_hicp_minr`, `une_rt_m`, `lfst_r_lfu3rt`, `irt_lt_mcby_m`, `naio_10_cp1700`) · [M]
  **perché conta qui** livelli trimestrali, COFOG per natura economica, disoccupazione regionale, rendimenti, con una licenza che permette il repo
  **cosa se ne prende** PIL e componenti T2 2026, debito T1, COFOG 2024 per dicastero, spread, IPCA, disoccupazione per regione
- **URL** https://esploradati.istat.it/SDMXWS/rest/data/IT1,22_289_DF_DCIS_POPRES1_1,1.0/A.IT.JAN.9..99?startPeriod=2026 · [M]
  **perché conta qui** piramide per coorti al 1° gennaio 2026
  **cosa se ne prende** popolazione totale ed età singole (fonti: nessuna cattura, interrogazione ripetibile)
- **URL** https://www.rgs.mef.gov.it/VERSIONE-I/copyright/index.html · [V]
  **perché conta qui** licenza dei documenti RGS
  **cosa se ne prende** riuso non commerciale con citazione (fonti/rgs-copyright.md, cattura di un altro agente)

## Collegamenti

- [[economia-italiana-parametri]]: i coefficienti; questa pagina dà i livelli a cui applicarli.
- [[regole-fiscali-ue]]: stato della PDE e tetti di spesa netta; il debito 2025 lì è prima della revisione di settembre.
- [[motore-economico-raccomandato]]: l'ordine di aggiornamento che questi livelli inizializzano.
- [[input-output-leontief]], [[input-output-tables]]: FIGARO 2024 come base della matrice a 10 settori.
- [[finanza-pubblica]]: dinamica del debito con il costo medio del 2,8% e la vita media di 7,8 anni.
- [[demografia-e-migrazioni]]: la piramide al 1° gennaio 2026.
- [[mercato-del-lavoro-e-disuguaglianza]]: disoccupazione per regione come u di partenza.
- [[istat]], [[eurostat]]: i flussi interrogati.
- [[disposizioni-al-turno-zero]]: pagina parallela sullo stato iniziale degli altri sistemi.
- [[decisioni]]: dieci dicasteri e due quote di budget (16/10/2026).
