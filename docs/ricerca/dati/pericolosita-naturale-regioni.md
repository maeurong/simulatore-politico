---
tipo: dataset
fonti: [fonti/dpc-classificazione-sismica.md, fonti/dpc-classificazione-sismica-csv-2025-05.md, fonti/dpc-note-legali.md, fonti/ingv-cpti15-download-licenza.md, fonti/ingv-cpti15-v4-estratto-mw55-1900.md, fonti/ispra-idrogeo-indicatori-regioni-2026-10.md, fonti/ispra-dissesto-2024-executive-summary.md, fonti/ispra-note-legali.md, fonti/ispra-siccita-disponibilita-idrica-2023.md, fonti/effis-burntareas-it-aggregato-regioni.md, fonti/effis-data-license.md, fonti/ispra-analisi-effis-2024.md, fonti/ispra-analisi-effis-2025.md, fonti/eurostat-cli-iad-loss-it.md, fonti/eea-italia-perdite-eventi-climatici-2025.md, fonti/casaitalia-sintesi-rapporto-2017.md, fonti/camera-temi-terremoti-2026.md, fonti/mit-linee-guida-classificazione-rischio-sismico.md, fonti/mit-sismabonus-slide-2017.md, fonti/nibs-mitigation-saves-2019-pagina.md, fonti/nibs-mitigation-saves-2019-overview.md]
aggiornato: 2026-10-17
---
# Pericolosità naturale per regione

Dati aperti per dare alle 20 regioni un rischio trimestrale di emergenza naturale (terremoti, alluvioni e frane, incendi e siccità), una gravità e un ordine di grandezza del costo per tarare il fondo di riserva. È l'ingresso della M degli eventi puntuali di [[crisi-ed-eventi]]. Questa pagina non è un modello di pericolosità: dà solo la forma minima.

## Licenze

| Fonte | Licenza | Ridistribuibile nel repo |
|---|---|---|
| DPC, classificazione sismica dei comuni | CC BY 4.0, con la dicitura «Fonte: Dipartimento della Protezione Civile-Presidenza del Consiglio dei Ministri» (fonti/dpc-note-legali.md) [V] | sì, con attribuzione |
| INGV, catalogo CPTI15 v4.0 | CC BY 4.0, ma la stessa pagina dice anche «può essere utilizzato per scopi scientifici» (fonti/ingv-cpti15-download-licenza.md) [V] | sì per la licenza; la clausola sugli scopi scientifici è ambigua: meglio tenere nel repo solo i conteggi derivati [INF] |
| ISPRA, indicatori IdroGEO | sito ISPRA CC BY 4.0 salvo diversa indicazione (fonti/ispra-note-legali.md) [V]; le schede INSPIRE degli indicatori di rischio frane dichiarano CC BY-SA 4.0 [NON VERIFICATO: pagina INSPIRE non catturabile, redirect] | sì; per prudenza trattarli come BY-SA |
| EFFIS (JRC, Copernicus) | CC BY 4.0 (fonti/effis-data-license.md) [V] | sì, con attribuzione e indicazione delle modifiche |
| Eurostat `cli_iad_loss` | licenza Eurostat già in wiki ([[eurostat]]) | sì |

## 1. Terremoti

- **Pericolosità.** Dal 2003 ogni comune sta in una delle quattro zone. Zona 1 è la più pericolosa, con ag fra 0,25 e 0,35 g al 10% in 50 anni; zona 4 la meno pericolosa, con ag ≤ 0,05 g. Alcune regioni usano sottozone (2A, 3S...) (fonti/dpc-classificazione-sismica.md) [V]. Il DPC pubblica il CSV comune per comune, aggiornato a maggio 2025 (fonti/dpc-classificazione-sismica-csv-2025-05.md) [V].
- **Grandezza per regione:** quota dei comuni in zona 1 o 2, calcolata dal CSV in colonna 2 della tabella [M: `csv.DictReader` sul CSV DPC, zona = primo carattere di `ZONA_SISMICA`]. La quota è sui comuni, non sulla popolazione.
- **Frequenza storica.** Dal CPTI15 v4.0 (copre fino al 2020) ho estratto i 70 eventi con Mw ≥ 5,5 dal 1900, sezione `MA` (fonti/ingv-cpti15-v4-estratto-mw55-1900.md) [M]. Li ho assegnati a mano alla regione dell'epicentro e fusi in episodi «regione-anno»: per esempio la sequenza del 2016 conta una volta per Lazio, Marche e Umbria, e il 2017 per l'Abruzzo [INF: assegnazione manuale]. Ho escluso gli epicentri esteri (Slovenia, Vallese) e quelli in mare aperto (Tirreno meridionale 1941, 1980 e 2002; Ionio 1932; Mar Ligure 1963). Ho tenuto quelli sotto costa con danni a terra (Golfo di Patti 1978, Sicilia sud-orientale 1990). Messina 1908 ha l'epicentro sul lato calabrese ed è contato per entrambe le regioni. Totale: 45 episodi in 121 anni, cioè 0,37 all'anno a scala nazionale [M].
- Mw ≥ 5,5 è la soglia del «danno con vittime»: è una scelta mia [INF]. Lascia fuori eventi piccoli ma letali, come Ischia 2017.

## 2. Alluvioni e frane

- Dal rapporto ISPRA 2024: 1,28 milioni di abitanti a rischio frane in aree P3-P4 (dati 2024) e 6,8 milioni a rischio alluvioni nello scenario P2 (tempo di ritorno 100-200 anni). Le regioni più esposte sono Emilia-Romagna, Toscana, Veneto, Campania, Lombardia e Liguria (fonti/ispra-dissesto-2024-executive-summary.md) [V].
- I valori regionali vengono dall'API pubblica di IdroGEO, `https://idrogeo.isprambiente.it/api/pir/regioni/<1..20>`, interrogata il 17/10/2026: campi `pop_idr_p2`, `popidp2_p`, `popfr_p3p4`, `popfrp3p4p` (fonti/ispra-idrogeo-indicatori-regioni-2026-10.md) [M]. I totali nazionali dell'API (`pop_idr_p2` = 6.818.375, `popfr_p3p4` = 1.284.960) coincidono con il rapporto [M].
- **Peso idro della regione** w = ½ × quota nazionale della popolazione in P2 alluvioni + ½ × quota nazionale della popolazione in frana P3-P4 [M nel calcolo, INF nella scelta della media]. La media smorza l'Emilia-Romagna, che da sola ha il 39,8% della popolazione nazionale in P2.
- **Frequenza degli eventi:** [NON TROVATO]. Non esiste un dataset aperto degli stati di emergenza deliberati dal Consiglio dei ministri. Il DPC pubblica le delibere solo come PDF singoli, e fra i repo GitHub `pcm-dpc` non c'è un elenco [M: `api.github.com/orgs/pcm-dpc/repos`]. Il numero nazionale di eventi idro gravi all'anno resta quindi una manopola da tarare (vedi sotto).

## 3. Incendi e siccità

- **Incendi.** Ho scaricato dall'API EFFIS `burntareas/current?country=IT` tutti i 15.592 poligoni bruciati, con data dal 2000-06-01 al 2026-10-15. La regione l'ho presa dalla provincia, o dal comune, tramite il CSV del DPC; ho aggregato sul periodo 2006-2025 (fonti/effis-burntareas-it-aggregato-regioni.md) [M].
  - Controllo: il 2024 aggregato fa 50.802 ha, contro i 51.424 di ISPRA (fonti/ispra-analisi-effis-2024.md); il 2025 fa 96.539 ha, contro i 965,39 km² di ISPRA (fonti/ispra-analisi-effis-2025.md) [M].
  - Media nazionale 2006-2025: 60.469 ha l'anno; massimo 159.537 ha nel 2021 [M].
  - Degli incendi ≥ 500 ha del 2006-2025, 317 su 364 (87%) cadono nel terzo trimestre [M].
- **Siccità.** Nel 2022 circa il 20% del territorio era in siccità estrema e circa il 40% in siccità severa o moderata. Per persistenza il 2022 è il terzo anno peggiore, dopo il 1990 e il 2002. I deficit massimi del 2022: Sicilia −80,7%, Sardegna −73%, distretto del Po −66% (fonti/ispra-siccita-disponibilita-idrica-2023.md) [V]. Una serie per regione degli anni di siccità: [NON TROVATO]. Il modello BIGBANG di ISPRA lavora per distretto idrografico, e non l'ho interrogato.

## Tabella per regione

Colonne: (a) quota % dei comuni in zona 1 o 2; (b) episodi sismici Mw ≥ 5,5 nel 1900-2020; (c) probabilità trimestrale di sisma = 1 − exp(−b/121/4); (d) % della popolazione regionale in P2 alluvioni; (e) % in frana P3-P4; (f) peso idro w, in %; (g) ettari bruciati medi annui 2006-2025; (h) quota dei trimestri 2006-2025 con almeno un incendio ≥ 500 ha.

| Regione | (a) | (b) | (c) | (d) | (e) | (f) | (g) | (h) |
|---|---|---|---|---|---|---|---|---|
| Piemonte | 0,0 | 0 | — | 4,9 | 1,81 | 4,6 | 740 | 0,025 |
| Valle d'Aosta | 0,0 | 0 | — | 9,1 | 10,90 | 0,6 | 10 | 0 |
| Lombardia | 3,8 | 0 | — | 4,4 | 0,44 | 4,8 | 231 | 0,013 |
| Trentino-Alto Adige | 1,4 | 0 | — | 18,0 | 2,46 | 2,4 | 16 | 0 |
| Veneto | 45,7 | 1 | 0,0021 | 11,7 | 0,13 | 4,4 | 81 | 0,025 |
| Friuli-Venezia Giulia | 65,1 | 2 | 0,0041 | 9,9 | 0,33 | 1,0 | 131 | 0,025 |
| Liguria | 17,5 | 0 | — | 17,4 | 7,01 | 6,1 | 446 | 0,037 |
| Emilia-Romagna | 33,0 | 4 | 0,0082 | 62,5 | 1,86 | 23,1 | 43 | 0 |
| Toscana | 32,6 | 4 | 0,0082 | 25,5 | 4,96 | 13,9 | 469 | 0,037 |
| Umbria | 80,4 | 4 | 0,0082 | 7,2 | 2,10 | 1,2 | 130 | 0 |
| Marche | 100,0 | 3 | 0,0062 | 5,2 | 1,91 | 1,7 | 190 | 0,013 |
| Lazio | 79,9 | 2 | 0,0041 | 3,2 | 1,50 | 4,6 | 2.630 | 0,062 |
| Abruzzo | 91,1 | 7 | 0,0144 | 7,2 | 5,07 | 3,2 | 1.414 | 0,037 |
| Molise | 93,4 | 1 | 0,0021 | 2,3 | 5,93 | 0,7 | 150 | 0,013 |
| Campania | 88,7 | 4 | 0,0082 | 5,1 | 4,69 | 12,4 | 3.384 | 0,075 |
| Puglia | 26,5 | 1 | 0,0021 | 3,4 | 1,61 | 3,4 | 2.428 | 0,125 |
| Basilicata | 88,5 | 2 | 0,0041 | 1,1 | 7,00 | 1,5 | 1.141 | 0,037 |
| Calabria | 100,0 | 6 | 0,0123 | 12,8 | 2,78 | 3,8 | 10.535 | 0,150 |
| Sicilia | 91,3 | 4 | 0,0082 | 2,6 | 1,93 | 4,6 | 28.352 | 0,350 |
| Sardegna | 0,0 (tutta zona 4) | 0 | — | 7,5 | 1,30 | 1,7 | 7.947 | 0,263 |

Fonti delle colonne: (a) fonti/dpc-classificazione-sismica-csv-2025-05.md; (b) e (c) fonti/ingv-cpti15-v4-estratto-mw55-1900.md, con assegnazione manuale [INF]; (d), (e) e (f) fonti/ispra-idrogeo-indicatori-regioni-2026-10.md. Le percentuali (d) ed (e) sono i campi `popidp2_p` e `popfrp3p4p` dell'API, che usano come base la popolazione del censimento 2011. (g) e (h) vengono da fonti/effis-burntareas-it-aggregato-regioni.md. Tutte le colonne sono [M].

Episodi sismici assegnati (b) [INF]:
- Abruzzo: 1904, 1915, 1933, 1950, 1984, 2009, 2017
- Calabria: 1905, 1907, 1908, 1913, 1928, 1947
- Campania: 1910, 1930, 1962, 1980
- Toscana: 1914, 1917, 1919, 1920
- Emilia-Romagna: 1916, 1918, 1971, 2012
- Umbria: 1979, 1984, 1997, 2016
- Sicilia: 1908, 1968, 1978, 1990
- Marche: 1930, 1943, 2016
- Lazio: 1916, 2016
- Friuli-Venezia Giulia: 1928, 1976
- Basilicata: 1990, 1998
- Veneto: 1936
- Puglia: 1948
- Molise: 2002

## 4. Costi: ordine di grandezza

- **Totale storico.** Negli ultimi 70 anni (fino al 2016-17) i danni da eventi idrogeologici e sismici sono stimati in circa 290 miliardi di euro, cioè circa 4 miliardi l'anno, in crescita, con oltre 10.000 vittime. La stima Ance/Cresme per il 1944-2013, a prezzi 2013, dà 256 miliardi: 188,5 per i sismi e 67,5 per il dissesto idrogeologico (fonti/casaitalia-sintesi-rapporto-2017.md) [V, che riporta una stima di terzi].
  - Sismi: 188,5 / 70 ≈ 2,7 miliardi l'anno [INF].
  - Diviso per i 19 episodi Mw ≥ 5,5 del 1944-2013 della tabella, fa circa 10 miliardi per episodio. La distribuzione è molto asimmetrica, dominata da Friuli 1976 e Irpinia 1980 [INF].
- **Eventi climatici** (alluvioni, tempeste, siccità, incendi, ondate di calore). Eurostat `cli_iad_loss` per l'Italia, in milioni di euro a prezzi costanti (fonti/eurostat-cli-iad-loss-it.md) [M]:
  - totale 1980-2024: 145.229;
  - media 1980-2024: 3.227 l'anno; media 2010-2024: 5.701 l'anno; media trentennale al 2024: 4.024;
  - picchi: 18.112 nel 2022, 16.072 nel 2023, 13.558 nel 2017.

  La suddivisione per tipo di evento è vuota per l'Italia [M]. L'alluvione dell'Emilia-Romagna del 2023 da sola: 8,5 miliardi e 17 vittime (fonti/eea-italia-perdite-eventi-climatici-2025.md) [V]. La stessa pagina EEA scrive anche «1,2 miliardi fra il 1990 e il 2023», un valore incompatibile con la serie Eurostat: probabile refuso, da non usare.
- **Fondo per le emergenze nazionali** (art. 44 del d.lgs. 1/2018). Incrementi: +130 milioni nel 2024, +71,8 milioni l'anno nel 2024 e nel 2025, +200 milioni nel 2024 (d.l. 113/2024), +100 milioni l'anno nel 2025 e nel 2026 (d.l. 39/2024) (fonti/camera-temi-terremoti-2026.md) [V]. Sono incrementi, non la dotazione totale [NON TROVATO: dotazione annua consolidata].
- **Prevenzione già spesa.** ReNDiS conta quasi 26.000 interventi contro il dissesto, finanziati per 19,2 miliardi in 25 anni, circa 0,77 miliardi l'anno (fonti/ispra-dissesto-2024-executive-summary.md) [V].
- **Ricostruzione** fuori dal fondo: per esempio 1,328 miliardi di contributo aggiuntivo al solo sisma 2016 nella legge di bilancio 2026 (fonti/camera-temi-terremoti-2026.md) [V].

## 5. Prevenzione: esiste un coefficiente?

- **Sisma: sì, come convenzione normativa italiana.** Le linee guida del DM 58/2017 classificano gli edifici da A+ a G secondo la PAM, la perdita annua media attesa in % del costo di ricostruzione (fonti/mit-linee-guida-classificazione-rischio-sismico.md, copia del testo ospitata da certifico.com perché il PDF del MIT è una scansione) [V]:
  - A+ ≤ 0,5%; A ≤ 1,0%; B ≤ 1,5%; C ≤ 2,5%; D ≤ 3,5%; E ≤ 4,5%; F ≤ 7,5%; G > 7,5%;
  - un edificio nuovo al minimo delle NTC ha PAM 1,13% (classe B).

  Portare un edificio da G a B riduce quindi la perdita attesa di circa l'85% (1 − 1,13/7,5) [INF]. Il Sismabonus premia ogni classe guadagnata: detrazione al 70% per una classe, all'80% per due o più (fonti/mit-sismabonus-slide-2017.md) [V]. Attenzione: è un modello di perdita convenzionale, non una misura ex post dei danni evitati.
- **Alluvioni, frane, incendi: solo rapporti beneficio/costo statunitensi.** NIBS, *Natural Hazard Mitigation Saves* 2019, dà medie nazionali USA (fonti/nibs-mitigation-saves-2019-overview.md, fonti/nibs-mitigation-saves-2019-pagina.md) [V]:
  - alluvione fluviale: 5:1 – 8:1, secondo la misura;
  - terremoto: 3:1 (sussidi federali) fino a 13:1 (retrofit degli edifici);
  - incendio nell'interfaccia urbano-foresta: 2:1 – 4:1;
  - sussidi federali in complesso: 6:1 (27 miliardi di dollari spesi, 160 risparmiati).

  Sono € risparmiati per € speso sull'intera vita dell'opera, non una % di riduzione del danno per turno, e sono dati USA [INF: trasferibilità all'Italia non verificata]. Una stima italiana della riduzione dei danni da alluvione per € di difesa del suolo: [NON TROVATO].

## Per il nostro gioco

Proposta minima [INF, da decidere nel ticket 15]:

- **Rischio trimestrale** per regione r e tipo h: p = 1 − exp(−λ(r,h)/4), da cui la M alla Paradox di [[crisi-ed-eventi]].
  - **Sisma:** λ = episodi(b)/121. Per le regioni a zero con comuni in zona 1-3 (Piemonte, Valle d'Aosta, Lombardia, Trentino-Alto Adige, Liguria) serve un minimo, per esempio 0,5/121, cioè p ≈ 0,001 a trimestre. Sardegna: 0. I conteggi sono piccoli e rumorosi; per smussarli si può fondere (b) con (a).
  - **Alluvioni e frane:** λ = N_idro × w(r). N_idro, il numero nazionale di emergenze idro gravi all'anno, è una **manopola**: il dato non esiste aperto. Valore iniziale proposto: 4, da tarare finché la perdita media simulata torna intorno ai ~1-5 miliardi l'anno di Casa Italia ed Eurostat. Stagione: più probabile nel quarto trimestre [INF, non misurato].
  - **Incendi:** la colonna (h) è già una probabilità trimestrale media. Meglio concentrarla nel terzo trimestre, dove cade l'87% dei grandi incendi: p_Q3 ≈ min(1; 3,5 × h), gli altri trimestri ≈ 0.
  - **Siccità:** non è un evento puntuale ma una situazione a soglia con isteresi, come in [[crisi-ed-eventi]]. Si accende sugli anni secchi, a scala nazionale o di distretto; Sicilia, Sardegna e Po sono le aree più colpite nel 2022.
- **Gravità**, in euro di danno:
  - sisma: circa 10 miliardi in media per episodio, con coda lunga. Basta un moltiplicatore casuale su tre livelli (lieve, grave, catastrofico) con media 1;
  - idro: lo stesso ordine di grandezza dell'Emilia-Romagna 2023 per l'evento grave, cioè 8,5 miliardi;
  - incendio: costo [NON TROVATO]; resta solo la superficie.

  La gravità si moltiplica per (1 − k × prevenzione), con prevenzione ∈ [0, 1] presa dallo stock di infrastrutture e dalla sicurezza ambientale della regione:
  - k_sisma = 0,85, il salto G → B del DM 58: l'unico coefficiente con un appiglio italiano;
  - per idro e incendi, k va dichiarato come scelta di design. Si può ancorare al fatto che un rapporto beneficio/costo di 4-8:1 rende la prevenzione conveniente, ma non dice di quanto riduca il danno a turno.
- **Fondo di riserva:** gli incrementi del Fondo per le emergenze nazionali nel 2024-2026 vanno da 100 a 400 milioni l'anno, contro un danno atteso medio di circa 4-8 miliardi l'anno (sismi più eventi climatici). Il fondo copre la prima emergenza, mai la ricostruzione. Questo regge l'opzione «decreto a debito» del ticket 15. Taglia di partenza proposta: 0,2-0,4 miliardi l'anno [INF].
- **Da evitare:** la quota (a) e il peso (f) pesati sui comuni o sulla popolazione 2011 vanno bene per pesi relativi, non come probabilità assolute.

## Collegamenti
- [[crisi-ed-eventi]]: il rischio trimestrale è l'ingresso della M degli eventi puntuali; la siccità è una situazione a soglia.
- [[decisioni]]: decisione del 17/10/2026 sulle emergenze naturali (ticket 15).
- [[istat]]: popolazione per pesare la quota (a) sugli abitanti, se serve.
- [[eurostat]]: licenza di `cli_iad_loss`.
- [[servizi-pubblici]]: stock di infrastrutture e sicurezza ambientale come prevenzione.
- [[territorio-e-urbanistica]]: le regioni come unità della mappa.
