---
tipo: modello
fonti: [fonti/degrauwe-ji-2013-self-fulfilling-crises.md, fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, fonti/bdi-boleco-4-2018-spread.md, fonti/bdi-boleco-3-2022-spread.md, fonti/ecb-pr-2022-07-21-tpi.md, fonti/ecb-pr-2012-09-06-omt.md, fonti/ecb-op-372-2025-tpi-omt.md, fonti/ecb-lagarde-econ-2025-12-03-tpi.md, fonti/ball-leigh-loungani-2013-okun.md, fonti/econbull-bonaventura-2020-okun-regioni.md, fonti/ecb-wp-546-garnier-wilhelmsen-2005.md, fonti/ijcb-riggi-santoro-2015-phillips-italia.md, fonti/istat-cet-2022-2024-comunicato.md, fonti/istat-pil-territoriale-prel-2025.md, fonti/istat-note-legali.md, fonti/ade-irpef-aliquote-2026.md, fonti/ade-iva-norme-aliquote.md, fonti/ade-ires-infogen.md, fonti/ec-dp-098-2019-semi-elasticita.md, fonti/ecb-capital-key-2026.md]
aggiornato: 2026-10-17
---
# Parametri dell'economia italiana

I numeri per tarare sull'Italia il motore della v1: spread BTP-Bund non lineare, TPI e OMT come freni condizionati, Okun nazionale e regionale, IS e Phillips stimate sull'area euro, dettaglio settoriale dei conti territoriali ISTAT, fisco 2026 con l'elasticità ciclica del bilancio, peso dell'Italia nell'area euro. Sostituisce i coefficienti USA dove la letteratura offre un numero europeo o italiano, e dice dove non lo offre.

## 1. Spread BTP-Bund: non lineare nel debito, con episodi sganciati dai fondamentali

**Forma quadratica stimata sull'area euro.** De Grauwe e Ji regrediscono lo spread decennale verso il Bund su debito/PIL, debito/PIL al quadrato, crescita, saldo corrente cumulato, cambio reale ed effetti fissi paese, su 10 paesi dell'area euro (fra cui l'Italia), trimestrale 2000Q1-2011Q3 (fonti/degrauwe-ji-2013-self-fulfilling-crises.md, tab. 1 ed eq. 2) [V]. Coefficienti, spread in punti percentuali e debito in punti di PIL [INF, dall'intestazione «(%)» della tab. 5]:

- lineare: 0,0744 per punto di debito (R² 0,67);
- quadratica: -0,0523 sul debito (non significativo) e +0,0009 sul quadrato (significativo all'1%), R² 0,80. Nel testo estratto il segno meno è un carattere di controllo `\x04` (fonti/degrauwe-ji-2013-self-fulfilling-crises.md, tab. 1) [V, segno letto sul carattere].

Effetto marginale implicito $\partial s/\partial b = -0{,}0523 + 0{,}0018\,b$ [INF, derivata]: 5,6 pb per punto di debito al 60%, 12,8 al 100%, 16,4 al 120%, 19,1 al 135%, 21,8 al 150% [M, `python3 -I -c` con i due coefficienti, 2026-10-17]. Il confronto è con i 3-4 pb di Laubach sugli USA ([[finanza-pubblica]]).

**Il paese con la propria valuta non paga lo stesso prezzo.** Sui 14 paesi «stand-alone» (Regno Unito, USA, Giappone, Svezia…) debito e spazio fiscale non hanno effetto significativo sullo spread (0,0105, n.s.); nell'area euro il coefficiente sul debito passa da 0,0034 (n.s.) prima della crisi a 0,1157 dopo il 2008 (fonti/degrauwe-ji-2013-self-fulfilling-crises.md, tabb. 2-3) [V]. Con dummy temporali, dal 2010Q4 al 2011Q3 compaiono effetti positivi e significativi non spiegati dai fondamentali, sia nel centro sia nella periferia (fonti/degrauwe-ji-2013-self-fulfilling-crises.md, par. 4.3) [V]: gli autori li leggono come crisi autoavverate.

**Stime della Banca d'Italia sullo spread «giustificato».** Con la stessa forma quadratica (più interazioni per paese), la relazione debito-spread è convessa in tutti i paesi tranne il Belgio; per l'Italia nel 2012Q1 il valore coerente con i fondamentali è circa 210 pb contro 380 osservati (fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, par. 4.2) [V]. Sul decennale, le varie stime danno fra 80 e 408 pb, la maggior parte intorno a 200, contro circa 450 di fine agosto 2012; la Banca attribuisce lo scarto soprattutto al timore di rottura dell'euro (fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, sintesi e tab. 1) [V]. Dalla letteratura citata: 1 punto di disavanzo in più (rispetto alla Germania) vale in media 2,4 pb di spread (Barrios et al. 2009) (fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, par. 3) [V].

**Episodi.**

| Episodio | Spread decennale | Fonte |
|---|---|---|
| Novembre 2011 | massimo di 5,5 punti percentuali, rendimento 7,3% | fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, par. 2 [V] |
| Maggio 2010 - giugno 2011 | circa 150 pb in media | fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, par. 2 [V] |
| Agosto 2012 (prima dell'OMT) | circa 450 pb osservati contro circa 200 coerenti con i fondamentali | fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, sintesi [V] |
| Metà ottobre 2018 | oltre 300 pb, +70 pb da fine giugno; rendimento 3,58% | fonti/bdi-boleco-4-2018-spread.md, par. 2.8 [V] |
| 14-15 giugno 2022 | circa 242 pb (da 154 a inizio aprile); 194 l'8 luglio | fonti/bdi-boleco-3-2022-spread.md, par. 2.8 e riquadro [V] |

Nel 2022 la Banca d'Italia stimava coerente con i fondamentali uno spread sotto i 150 pb e non giustificati livelli oltre i 200, con un modello panel su debito/PIL, attese di crescita e d'inflazione e disoccupazione (fonti/bdi-boleco-3-2022-spread.md, riquadro, fig. B) [V]. Il massimo del 2018 su fonte primaria non è stato trovato: il valore circolato in stampa (circa 326 pb il 20 novembre) non è verificato [NON TROVATO].

## 2. TPI e OMT: chi può essere aiutato, a quali condizioni

**TPI (21 luglio 2022).** Acquisti sul secondario di titoli pubblici con vita residua fra 1 e 10 anni, senza limiti ex ante, contro dinamiche di mercato «non giustificate dai fondamentali del paese»; l'attivazione richiede una valutazione complessiva degli indicatori di mercato e di trasmissione, dei criteri di ammissibilità e della proporzionalità; gli acquisti finiscono quando la trasmissione migliora in modo durevole o quando le tensioni risultano dovute ai fondamentali (fonti/ecb-pr-2022-07-21-tpi.md) [V]. I quattro criteri, cumulativi, testuali:

1. rispetto del quadro fiscale UE: «not being subject to an excessive deficit procedure (EDP), or not being assessed as having failed to take effective action in response to an EU Council recommendation under Article 126(7) TFEU»;
2. assenza di squilibri macroeconomici gravi: non essere in procedura per squilibri eccessivi, o non aver mancato l'azione correttiva raccomandata ai sensi dell'art. 121(4) TFUE;
3. sostenibilità fiscale, valutata anche con le analisi di sostenibilità del debito di Commissione, MES, FMI e BCE;
4. politiche macroeconomiche sane: rispetto degli impegni del PNRR e delle raccomandazioni specifiche per paese in campo fiscale (fonti/ecb-pr-2022-07-21-tpi.md) [V].

I criteri sono «un input» della decisione e possono essere adattati ai rischi del momento (fonti/ecb-pr-2022-07-21-tpi.md) [V]. Lettura per l'Italia: il criterio 1 è alternativo («or»), quindi un paese in PDE ma con azione efficace riconosciuta lo soddisfa alla lettera; l'Italia oggi è in PDE sospesa per azione efficace ([[regole-fiscali-ue]]) [INF, dal testo]. Un giudizio di «nessuna azione efficace» (passo 126.8 della PDE) la escluderebbe [INF].

**Mai attivato.** «Neither OMTs nor the TPI have been activated to date» (fonti/ecb-op-372-2025-tpi-omt.md, giugno 2025) [V]; Lagarde al Parlamento europeo il 3 dicembre 2025: TPI e OMT «available in the toolbox and ready to go, but never used» (fonti/ecb-lagarde-econ-2025-12-03-tpi.md) [V]. Una conferma primaria successiva a dicembre 2025 non è stata trovata [NON TROVATO]. Effetto dell'annuncio: secondo lo staff dell'Eurosistema il TPI ha in parte isolato gli spread dei paesi ad alto debito (oltre il 90%: Grecia, Italia, Francia, Spagna, Belgio, Portogallo) dagli shock di politica monetaria restrittiva (fonti/ecb-op-372-2025-tpi-omt.md, grafico 10) [V].

**OMT (6 settembre 2012).** Condizione necessaria: condizionalità «rigorosa ed efficace» legata a un programma EFSF/MES, completo o precauzionale (ECCL) purché preveda acquisti sul primario, con coinvolgimento dell'FMI; acquisti concentrati su titoli da 1 a 3 anni, nessun limite quantitativo ex ante, trattamento pari passu, liquidità interamente sterilizzata; stop in caso di mancato rispetto del programma (fonti/ecb-pr-2012-09-06-omt.md) [V]. Differenza chiave: il TPI si attiva a discrezione della BCE, l'OMT solo dopo che il paese ha chiesto il sostegno del MES (fonti/ecb-op-372-2025-tpi-omt.md) [V].

## 3. Okun: Italia e regioni

**Italia, forma in gap** $u - u^* = \beta (y - y^*)$, annuale 1980-2011, filtro HP con λ = 100: $\beta = -0{,}254$ (errore standard 0,067), R² corretto 0,29, uno dei due più bassi fra i 20 paesi insieme all'Austria (fonti/ball-leigh-loungani-2013-okun.md, tab. 5) [V]. Con coefficiente spezzato: -0,142 (n.s.) prima del 1995, -0,358 dopo (fonti/ball-leigh-loungani-2013-okun.md, tab. 6) [V]. Per confronto, nella stessa tabella: Francia e Germania -0,367, Spagna -0,852.

**Regioni, forma in differenze** $\Delta u_t = a + b\,\dot y_t$ (variazione del tasso di disoccupazione sulla crescita del PIL reale), annuale 1995-2015 (fonti/econbull-bonaventura-2020-okun-regioni.md, eq. 1 e tab. 2) [V]. Coefficiente $b$ negativo in tutte le 20 regioni, significativo in 18 (non in Valle d'Aosta e Calabria):

| Nord | b | Centro | b | Mezzogiorno | b |
|---|---|---|---|---|---|
| Piemonte | -0,239 | Toscana | -0,281 | Abruzzo | -0,238 |
| Valle d'Aosta | -0,059 (n.s.) | Umbria | -0,345 | Molise | -0,305 |
| Liguria | -0,262 | Marche | -0,295 (-0,323 con AR(1)) | Campania | -0,369 |
| Lombardia | -0,220 | Lazio | -0,391 | Puglia | -0,403 (-0,448 con AR(1)) |
| Trentino-Alto Adige | -0,141 | | | Basilicata | -0,180 |
| Veneto | -0,198 | | | Calabria | -0,342 (n.s.) |
| Friuli-Venezia Giulia | -0,172 | | | Sicilia | -0,419 |
| Emilia-Romagna | -0,222 | | | Sardegna | -0,251 |

(fonti/econbull-bonaventura-2020-okun-regioni.md, tab. 2) [V]. Medie semplici per macroarea: Nord -0,19, Centro -0,33, Mezzogiorno -0,31 [M, media aritmetica non ponderata dei coefficienti della tab. 2, `python3 -I -c`, 2026-10-17]. Le R² regionali vanno da 0,04 (Valle d'Aosta) a 0,74 (Umbria): nel Sud la relazione è più forte in pendenza ma più rumorosa (fonti/econbull-bonaventura-2020-okun-regioni.md, tab. 2) [V]. Al Sud il coefficiente femminile è spesso non significativo (fonti/econbull-bonaventura-2020-okun-regioni.md, abstract) [V].

Le due stime non sono intercambiabili: la nazionale è in gap, la regionale in differenze; nella forma in differenze $b$ include anche la crescita potenziale nella costante $a$ [INF].

## 4. IS e Phillips: coefficienti per l'area euro e per l'Italia

**Area euro, forma Laubach-Williams** (backward-looking, trimestrale 1963q1-2004q1), la più vicina a Rudebusch-Svensson fra quelle trovate (fonti/ecb-wp-546-garnier-wilhelmsen-2005.md, eq. 1-2 e tab. 1) [V]:

- IS: $\tilde y_t = 0{,}70\,\tilde y_{t-1} + 0{,}14\,\tilde y_{t-2} + a_r(\tilde r_{t-1} + \tilde r_{t-2}) + \varepsilon$, con $a_r = -0{,}056$ (t = 2,42), dove $\tilde r$ è lo scarto del tasso reale dal tasso naturale e i due ritardi hanno lo stesso coefficiente;
- Phillips: $\pi_t = 1{,}18\,\pi_{t-1} - 0{,}28\,\bar\pi_{t-2,4} + 0{,}10\,\bar\pi_{t-5,8} + 0{,}051\,\tilde y_{t-1} + \varepsilon$, con $\bar\pi$ medie dei ritardi indicati e somma dei coefficienti sull'inflazione imposta a uno (vincolo non rifiutato);
- nella stessa stima per gli USA: $a_r = -0{,}18$ e pendenza 0,103. Secondo gli autori la somma dei coefficienti autoregressivi dell'IS sta fra 0,83 e 0,93 in tutti i paesi, la sensibilità al tasso reale è più debole nell'area euro e la pendenza della Phillips è leggermente più forte che negli USA e in Germania (fonti/ecb-wp-546-garnier-wilhelmsen-2005.md, par. 4.3) [V]. La tabella però dà all'area euro una pendenza (0,051) più alta di quella tedesca (0,041) ma più bassa di quella USA (0,103): sul confronto con gli USA testo e tabella non coincidono, e qui vale la tabella (fonti/ecb-wp-546-garnier-wilhelmsen-2005.md, tab. 1) [V].

Confronto con RS ([[macro-neokeynesiano-piccolo]]): persistenza dell'IS 0,84 contro 0,91, sensibilità al tasso reale circa metà (-0,056 per ciascun ritardo contro -0,10 sulla media annua), pendenza della Phillips 0,05 contro 0,14 [INF, confronto fra specifiche non identiche: RS usa medie a quattro trimestri di tasso e inflazione].

**Italia, Phillips appiattita dopo l'euro.** Phillips ibrida stimata con metodi bayesiani su un modello di equilibrio generale: pendenza rispetto al costo marginale reale da 0,74 (1981-1998) a 0,05 (1999-2012); persistenza intrinseca (peso dell'inflazione passata) da 0,45 a 0,18; durata media dei prezzi da 1,7 a 2,9 trimestri (fonti/ijcb-riggi-santoro-2015-phillips-italia.md, discussione delle stime a posteriori, pp. 180-181) [V]. La pendenza è sul costo marginale, non sull'output gap: indica la direzione (crollo dopo il 1999), non un numero da trapiantare in RS [INF].

Una IS backward-looking stimata sulla sola Italia, con coefficienti comparabili a RS, non è stata trovata su fonte primaria [NON TROVATO]. Peersman-Smets (1999) e Gerlach-Smets (1999) stimano IS e Phillips backward-looking per l'area euro ma non sono stati letti (testo non accessibile) [NON TROVATO].

## 5. Conti economici territoriali ISTAT

- **Ultima diffusione**: definitivi 2022, semi-definitivi 2023, preliminari 2024, coerenti con i conti nazionali di settembre 2025; su IstatData per regione PIL, valore aggiunto, redditi da lavoro dipendente, occupazione, investimenti, consumi e reddito disponibile; per provincia PIL, valore aggiunto e occupazione (fonti/istat-cet-2022-2024-comunicato.md) [V]. Edizione dei dati `2025M12` [M, sotto]. Il 2024 è stimato con un approccio econometrico basato su indicatori, soggetto a revisioni sostanziali (fonti/istat-cet-2022-2024-comunicato.md) [V].
- **Stima preliminare 2025**, solo per macroarea (Nord-ovest, Nord-est, Centro, Mezzogiorno) e per sei comparti: PIL +0,5% in Italia, +0,6% nel Mezzogiorno; occupati +1,1% in Italia, +1,5% nel Mezzogiorno; revisioni «anche ampie» possibili (fonti/istat-pil-territoriale-prel-2025.md) [V].
- **Dettaglio per branca** [M, `curl` sul servizio SDMX `https://esploradati.istat.it/SDMXWS/rest/`, 2026-10-17]:
  - valore aggiunto regionale, flusso `93_1227_DF_DCCN_TNA1_1`: 42 codici NACE Rev. 2, cioè le sezioni A-T, la manifattura in 9 gruppi (C10T12 … C31T33), agricoltura e pesca separate, più gli aggregati della classificazione A*10 (A, BTE, F, GTI, J, K, L, M_N, OTQ, RTU) e a 6 branche (A, BTE, F, GTJ, KTN, OTU); prezzi correnti, prezzi dell'anno precedente, valori concatenati 2020; dal 1995 al 2024; territori 139 (Italia, ripartizioni, regioni, province) (`availableconstraint/93_1227_DF_DCCN_TNA1_1`);
  - per il 2023 Campania ha tutti i 42 codici (`data/.../A.ITF3.....V...2025M12?startPeriod=2023`); per il 2024 la Lombardia ha solo le 6 branche e il totale (`...A.ITC4.....V...2025M12?startPeriod=2024`): **l'A*10 regionale arriva al 2023, il 2024 è a 6 branche**;
  - occupazione regionale, flusso `93_379_DF_DCCN_OCCTSEC2010_1`: stesse 42 branche, regioni e ripartizioni (non province), in occupati, posizioni lavorative, ore e unità di lavoro (ULA), regolari e irregolari, dipendenti e indipendenti, 1995-2024 (`availableconstraint/93_379_DF_DCCN_OCCTSEC2010_1`);
  - non verificato se il dettaglio a 42 branche esiste anche per le province (richiesta scaduta) [NON TROVATO].
- **Licenza**: CC BY 4.0 salvo diversa indicazione (fonti/istat-note-legali.md) [V]; vedi [[istat]].

## 6. Fisco 2026 ed elasticità ciclica del bilancio

**IRPEF 2026** (art. 11 TUIR come modificato dall'art. 1, c. 3, L. 199/2025): 23% fino a 28.000 €, 33% da 28.000 a 50.000 €, 43% oltre; imposta sui redditi oltre 50.000 € = 13.700 € + 43% dell'eccedenza; per redditi complessivi oltre 200.000 € il beneficio del taglio dal 35 al 33% è sterilizzato (art. 1, cc. 3-4) (fonti/ade-irpef-aliquote-2026.md, pagina aggiornata al 13/01/2026) [V]. La tabella principale della pagina mostra ancora il 35%, l'avviso sotto la corregge [V]. Detrazioni da lavoro e sterilizzazione nel dettaglio: non catturate [NON TROVATO].

**IVA**: ordinaria 22%; ridotte 4%, 5%, 10% per i beni delle parti II, II-bis e III della tabella A del DPR 633/1972 (fonti/ade-iva-norme-aliquote.md) [V]. La pagina è aggiornata al 12/12/2020: le aliquote non sono cambiate per quanto risulta, ma singoli beni sono stati spostati fra aliquote dal 2024 [INF, da notizie non catturate].

**IRES**: 24%; solo per il 2025, 20% per le imprese che accantonano almeno l'80% degli utili 2024 e ne investono una parte, con vincoli occupazionali (L. 207/2024, cc. 436-444) (fonti/ade-ires-infogen.md, pagina aggiornata al 13/07/2026) [V]. Per il 2026 la pagina non indica aliquote ridotte [INF, assenza nel testo].

**Elasticità ciclica (metodo comune UE).** Per l'Italia la semi-elasticità del saldo di bilancio rispetto all'output gap è 0,544: un punto di output gap negativo peggiora il saldo di circa 0,54 punti di PIL (aggiornamento 2018; era 0,539 nel 2014) (fonti/ec-dp-098-2019-semi-elasticita.md, tabb. I.3 e IV.3) [V]. Scomposizione italiana: elasticità delle entrate totali 1,05 e delle spese -0,05; entrate 46,76% e spese 49,96% del PIL; semi-elasticità delle entrate 0,022 e delle spese -0,522 (fonti/ec-dp-098-2019-semi-elasticita.md, tab. I.3) [V]. Elasticità per imposta rispetto all'output gap: IRPEF e redditi 1,46; società 3,07; contributi sociali 0,58; imposte indirette 1,10; spesa per disoccupazione -2,29; resto della spesa 0 (fonti/ec-dp-098-2019-semi-elasticita.md, tab. I.1) [V]. Un aggiornamento della Commissione successivo al 2019 non è stato trovato [NON TROVATO].

Nel gioco [INF, dalla definizione]: gettito di un'imposta $R_k = R_{k,0}\,(1 + \varepsilon_k \cdot gap/100)$ sopra la base strutturale, con $\varepsilon_k$ dalla tab. I.1; l'elasticità del reddito imponibile all'aliquota ([[finanza-pubblica]]) resta un canale separato.

## 7. Peso dell'Italia nell'area euro

- **PIL**: 2.265,0 miliardi di euro nel 2025 su 15.968,9 dell'area euro a 21 paesi, cioè il **14,2%**; 14,3% sull'area a 20; 14,4% nel 2024 sull'area a 21 [M, API Eurostat `nama_10_gdp`, `unit=CP_MEUR`, `na_item=B1GQ`, aggiornamento del dataset 2026-10-15, interrogata il 2026-10-17].
- **Capitale BCE**: quota della Banca d'Italia nel capitale sottoscritto (tutte le banche centrali UE) 13,0993%; **chiave dell'Eurosistema** (solo area euro, con la Bulgaria dal 1° gennaio 2026) 15,8307%, contro 16,0201% al 1° gennaio 2024; la chiave pesa per metà popolazione e per metà PIL e si aggiorna ogni 5 anni (fonti/ecb-capital-key-2026.md) [V].

Per la regola di Taylor della BCE nel gioco, il peso giusto dell'Italia nell'inflazione e nel gap dell'area è la quota di PIL (o di consumi armonizzati), non la chiave di capitale: questa guida gli acquisti dei programmi BCE [INF, dalla funzione della chiave descritta in fonti/ecb-capital-key-2026.md].

## Per il nostro gioco

- **Spread**: $s = s_0 + \max(0,\,-0{,}0523\,b + 0{,}0009\,b^2) + \kappa_t$, con $b$ debito/PIL atteso e $\kappa_t$ un premio di sfiducia che scatta con eventi politici e decade, tarato sui due casi in cui lo spread osservato ha superato di 150-250 pb quello «giustificato» (2012, 2022) [INF]. Con il debito italiano intorno al 137% la pendenza marginale è circa 19 pb per punto di debito, cinque volte Laubach [M sopra].
- **Tetto del TPI**: se i quattro criteri valgono, la BCE riporta $\kappa_t$ verso zero ma non tocca la parte da fondamentali (gli acquisti si fermano se le tensioni dipendono dai fondamentali). Un «nessuna azione efficace» in PDE, una procedura per squilibri o una DSA negativa tolgono il tetto [INF, dai criteri].
- **OMT**: solo dopo una richiesta al MES con programma e condizionalità; è l'evento-scelta già fissato in [[decisioni]] (MES con OMT o ristrutturazione).
- **Okun per regione**: coefficienti della tab. 2 nella forma in differenze, con $u^*$ regionale che ha isteresi; per la coerenza col nazionale (-0,25 in gap) serve una ricalibrazione della media [INF].
- **Congiuntura**: IS e Phillips dell'area euro di Garnier-Wilhelmsen per l'Italia come primo default, con la pendenza della Phillips bassa (0,05) coerente con l'appiattimento italiano dopo il 1999 [INF].
- **Settori**: i 10 settori della v1 combaciano con l'A*10 ISTAT regionale fino al 2023; il 2024 regionale esiste solo a 6 branche [M].
- **Fisco**: scaglioni IRPEF 2026, IVA 22/10/5/4, IRES 24, più le elasticità cicliche della Commissione.
- Aperti: IS stimata sull'Italia; massimo dello spread 2018 su fonte primaria; detrazioni IRPEF 2026; semi-elasticità aggiornate dopo il 2019.

## Fonti

- **URL** https://eprints.lse.ac.uk/49648/1/__lse.ac.uk_storage_LIBRARY_Secondary_libfile_shared_repository_Content_De%20Grauwe,%20P_Self-fulfilling%20crises_De%20Grauwe_Self-fulfilling%20crises_2014.pdf · [V]
  **perché conta qui** unica stima trovata della forma quadratica debito-spread per l'area euro, con il confronto coi paesi con valuta propria
  **cosa se ne prende** coefficienti lineare e quadratico, assenza d'effetto fuori dall'euro, salto post-2008 (fonti/degrauwe-ji-2013-self-fulfilling-crises.md)
- **URL** https://www.bancaditalia.it/pubblicazioni/qef/2012-0128/QEF_128.pdf · [V]
  **perché conta qui** stime della Banca d'Italia dello spread coerente coi fondamentali per l'Italia, con la forma non lineare
  **cosa se ne prende** 210 contro 380 pb nel 2012Q1, la tabella delle stime, i massimi del novembre 2011 (fonti/bdi-qef-128-2012-premi-rischio-sovrano.md)
- **URL** https://www.bancaditalia.it/pubblicazioni/bollettino-economico/2018-4/boleco-4-2018.pdf · [V]
  **perché conta qui** episodio 2018, lo spread da incertezza sulle politiche economiche
  **cosa se ne prende** oltre 300 pb a metà ottobre 2018, +70 da fine giugno (fonti/bdi-boleco-4-2018-spread.md, pp. 38-40)
- **URL** https://www.bancaditalia.it/pubblicazioni/bollettino-economico/2022-3/boleco-3-2022.pdf · [V]
  **perché conta qui** episodio 2022 alla vigilia del TPI, con la soglia dei fondamentali
  **cosa se ne prende** 242 pb a metà giugno, 194 l'8 luglio, fondamentali sotto 150 e oltre 200 non giustificato (fonti/bdi-boleco-3-2022-spread.md, pp. 47-50)
- **URL** https://www.ecb.europa.eu/press/pr/date/2022/html/ecb.pr220721~973e6e7273.en.html · [V]
  **perché conta qui** testo ufficiale del TPI
  **cosa se ne prende** i quattro criteri cumulativi, scadenze 1-10 anni, condizioni di stop (fonti/ecb-pr-2022-07-21-tpi.md)
- **URL** https://www.ecb.europa.eu/press/pr/date/2012/html/pr120906_1.en.html · [V]
  **perché conta qui** testo ufficiale dell'OMT
  **cosa se ne prende** condizionalità MES, scadenze 1-3 anni, sterilizzazione (fonti/ecb-pr-2012-09-06-omt.md)
- **URL** https://www.ecb.europa.eu/pub/pdf/scpops/ecb.op372.en.pdf · [V]
  **perché conta qui** valutazione BCE del 2025 su TPI e OMT
  **cosa se ne prende** mai attivati al giugno 2025; il TPI ha isolato in parte gli spread dei paesi ad alto debito (fonti/ecb-op-372-2025-tpi-omt.md, pp. 32-35)
- **URL** https://www.ecb.europa.eu/press/key/date/2025/html/ecb.sp251203_2~9b0c3c074f.en.pdf · [V]
  **perché conta qui** conferma più recente trovata che TPI e OMT non sono mai stati usati
  **cosa se ne prende** «ready to go, but never used», 3 dicembre 2025 (fonti/ecb-lagarde-econ-2025-12-03-tpi.md, pp. 1-6)
- **URL** https://www.nber.org/system/files/working_papers/w18668/w18668.pdf · [V]
  **perché conta qui** Okun nazionale per l'Italia, già catturato e non riportato in [[mercato-del-lavoro-e-disuguaglianza]]
  **cosa se ne prende** β = -0,254, prima e dopo il 1995 (fonti/ball-leigh-loungani-2013-okun.md, tabb. 5-6)
- **URL** http://www.accessecon.com/Pubs/EB/2020/Volume40/EB-20-V40-I4-P243.pdf · [V]
  **perché conta qui** unica stima trovata di Okun per tutte le 20 regioni
  **cosa se ne prende** i 20 coefficienti b con significatività e R² (fonti/econbull-bonaventura-2020-okun-regioni.md)
- **URL** https://www.ecb.europa.eu/pub/pdf/scpwps/ecbwp546.pdf · [V]
  **perché conta qui** IS e Phillips backward-looking per l'area euro nella forma più vicina a RS
  **cosa se ne prende** coefficienti dell'IS (0,70, 0,14, -0,056) e della Phillips (1,18, -0,28, 0,10, 0,051) (fonti/ecb-wp-546-garnier-wilhelmsen-2005.md, tab. 1)
- **URL** https://www.ijcb.org/sites/default/files/journal/v11n2/ijcb-v11n2-slope-and-persistence-italian-phillips-curve.pdf · [V]
  **perché conta qui** stima della Banca d'Italia della Phillips italiana prima e dopo l'euro
  **cosa se ne prende** pendenza da 0,74 a 0,05, persistenza da 0,45 a 0,18 (fonti/ijcb-riggi-santoro-2015-phillips-italia.md)
- **URL** https://www.istat.it/comunicato-stampa/conti-economici-territoriali-2022-2024/ · [V]
  **perché conta qui** ultima diffusione dei conti territoriali, con gli anni e lo stato delle stime
  **cosa se ne prende** 2022 definitivo, 2023 semi-definitivo, 2024 preliminare; variabili per regione e provincia (fonti/istat-cet-2022-2024-comunicato.md)
- **URL** https://www.istat.it/comunicato-stampa/stima-preliminare-del-pil-e-delloccupazione-territoriale-anno-2025/ · [V]
  **perché conta qui** il dato territoriale più recente, solo per macroarea
  **cosa se ne prende** crescita di PIL e occupati 2025 per macroarea e comparto (fonti/istat-pil-territoriale-prel-2025.md)
- **URL** https://www.istat.it/note-legali/ · [V]
  **perché conta qui** licenza dei dati territoriali
  **cosa se ne prende** CC BY 4.0 salvo diversa indicazione (fonti/istat-note-legali.md, cattura di un altro agente)
- **URL** https://www.agenziaentrate.gov.it/portale/imposta-sul-reddito-delle-persone-fisiche-irpef-/aliquote-e-calcolo-dell-irpef · [V]
  **perché conta qui** scaglioni IRPEF vigenti dopo la legge di bilancio 2026
  **cosa se ne prende** 23/33/43 con soglie 28.000 e 50.000, sterilizzazione oltre 200.000 (fonti/ade-irpef-aliquote-2026.md)
- **URL** https://www.agenziaentrate.gov.it/portale/iva-regole-generali-aliquote-esenzioni-pagamento/norme-generali-e-aliquote · [V]
  **perché conta qui** aliquote IVA
  **cosa se ne prende** 22% ordinaria, 4/5/10 ridotte (fonti/ade-iva-norme-aliquote.md)
- **URL** https://www.agenziaentrate.gov.it/portale/imposta-sui-redditi-societa-ires/infogen-imposta-sui-redditi-societa-ires · [V]
  **perché conta qui** aliquota IRES
  **cosa se ne prende** 24%, 20% condizionato solo per il 2025 (fonti/ade-ires-infogen.md)
- **URL** https://economy-finance.ec.europa.eu/document/download/5850437c-99c8-4205-9207-66878050f2cf_en?filename=dp098_en.pdf · [V]
  **perché conta qui** semi-elasticità ufficiali UE del bilancio al ciclo, paese per paese
  **cosa se ne prende** Italia 0,544 e le elasticità per imposta (fonti/ec-dp-098-2019-semi-elasticita.md, tabb. I.1, I.3, IV.3)
- **URL** https://www.ecb.europa.eu/ecb/orga/capital/html/index.en.html · [V]
  **perché conta qui** peso dell'Italia nel capitale BCE dopo l'ingresso della Bulgaria
  **cosa se ne prende** 13,0993% del capitale, 15,8307% della chiave dell'Eurosistema (fonti/ecb-capital-key-2026.md)

## Collegamenti

- [[finanza-pubblica]]: lo spread non lineare e le elasticità cicliche chiudono due suoi [NON TROVATO].
- [[regole-fiscali-ue]]: lo stato della PDE decide il criterio 1 del TPI.
- [[macro-neokeynesiano-piccolo]]: coefficienti dell'area euro da mettere accanto a RS.
- [[mercato-del-lavoro-e-disuguaglianza]]: Okun italiano e regionale.
- [[banca-centrale-e-inflazione]]: il peso dell'Italia nella regola della BCE.
- [[istat]]: i flussi SDMX dei conti territoriali.
- [[input-output-leontief]]: i 10 settori della v1 contro l'A*10 regionale.
- [[motore-economico-raccomandato]]: i coefficienti USA da ricalibrare.
- [[decisioni]]: le scelte del 17/10/2026 che questa pagina rifornisce di numeri.
