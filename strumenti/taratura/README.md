# Taratura offline dei dati strutturali

`economia.py` scarica tavole e serie da Eurostat e ISTAT e scrive i dati di struttura del motore economico. Gira fuori dal binario del gioco: nel repo stanno solo i risultati aggregati e lo script che li ricava. `elettorato.py` scrive l'elettorato: elettori e quote dei gruppi dalle fonti, il resto provvisorio. Vedi «Elettorato» in fondo.

```
python3 -I strumenti/taratura/economia.py
```

Solo libreria standard di Python (3.11 o successivo). Lo script controlla le invarianti con `assert` prima di scrivere: se una fallisce, i file restano come sono.

## Cosa scrive

| File | Campi | Fonte |
|---|---|---|
| `dati/base/economia.toml` | `settori`, `a`, `importazioni`, `valore_aggiunto`, `consumi`, `investimenti`, `esportazioni`, `spesa_pubblica`, `anno_tavola` | FIGARO 2024 |
| `dati/base/economia.toml` | `piramide` | ISTAT, residenti al 1° gennaio 2026 |
| `dati/base/regioni.toml` | 20 regioni con `forze_lavoro`, `disoccupazione_2025`, `okun`, `mix` | Eurostat LFS, ISTAT conti territoriali, Bonaventura (2020) |
| `dati/base/dicasteri.toml` | `spesa_pil`, `salari_pil`, `acquisti_pil` per dicastero, `settori` per Infrastrutture e Trasporti | Eurostat COFOG e conti nazionali |
| `dati/base/elettorato.toml` | tutto il file, da `elettorato.py` | `elettori` e `quota`: Camera 2022, ISTAT, MEF, IBF; `primario` di dipendenti e autonomi: MEF; `posizione`: ESS round 11 (`--ess`, CC BY-NC-SA 4.0); il resto provvisorio (vedi «Elettorato») |

In `dicasteri.toml` lo script riscrive solo questi campi e le due righe di commento `# [taratura]`; il resto del file è a mano.

## Fonti

| Dataset | Interrogazione | Aggiornamento letto nella risposta | Licenza |
|---|---|---|---|
| Eurostat `naio_10_fcp_ip4`, FIGARO, tavola inter-paese prodotto per prodotto ai prezzi base, milioni di euro | `c_dest=IT&time=2024` e `c_orig=IT&time=2024` | 2026-07-20 | riuso con citazione, modifiche dichiarate (vedi Licenze) |
| Eurostat `gov_10a_exp`, spesa delle AP per funzione (COFOG) | `geo=IT&sector=S13&unit=MIO_EUR&time=2024&na_item=TE,D1,P2,D632,D62` | 2026-10-21 | riuso con citazione, modifiche dichiarate (vedi Licenze) |
| Eurostat `nama_10_gdp`, PIL ai prezzi correnti | `geo=IT&unit=CP_MEUR&na_item=B1GQ&time=2024` | 2026-10-21 | riuso con citazione, modifiche dichiarate (vedi Licenze) |
| Eurostat `lfst_r_lfu3rt`, tasso di disoccupazione per regione NUTS 2 | `time=2025&sex=T&age=Y15-74&isced11=TOTAL` | 2026-09-10 | riuso con citazione, modifiche dichiarate (vedi Licenze) |
| Eurostat `lfst_r_lfp2act`, forze di lavoro per regione NUTS 2, migliaia | `time=2025&sex=T&age=Y15-74` | 2026-09-10 | riuso con citazione, modifiche dichiarate (vedi Licenze) |
| ISTAT `93_1227_DF_DCCN_TNA1_1`, valore aggiunto per branca e regione ai prezzi correnti | chiave `A.<20 regioni>.B1G_B_W2_S1.<settori produttivi (A*10) e totale>...V...2025M12`, anno 2023 | edizione 2025M12 | ISTAT, CC BY 4.0 |
| ISTAT `22_289_DF_DCIS_POPRES1_1`, popolazione residente per età | chiave `A.IT.JAN.9..99`, anno 2026 (stima) | n/d | ISTAT, CC BY 4.0 |
| Camera dei deputati `ItaliaVotiListeCIRC.csv`, elettori iscritti delle politiche 2022 | `https://documenti.camera.it/leg19/risultatiElettorali2022/UGE/VotiListe/ItaliaVotiListeCIRC.csv`, colonna `ELETTORI TOTALI`, circoscrizioni sommate per regione | n/d (sha256 `bbda02a9…` in `docs/ricerca/dati/risultati-elettorali-italia.md:13`) | [NON TROVATO]: usati come fatti |
| ISTAT `150_938_DF_DCCV_OCCUPATIT1_23`, occupati per posizione professionale | chiave `A.............` (tutte le dimensioni libere), anno 2025; `POSIZ_PROF` 1 dipendenti, 2 indipendenti | n/d | ISTAT, CC BY 4.0 |
| ISTAT `151_929_DF_DCCV_DISOCCUPT1_6`, disoccupati | tutto il flusso, anno 2025; `SEX=9`, `LABPROF_STATUS_A=99`, 15-74 anni | n/d | ISTAT, CC BY 4.0 |
| ISTAT `152_928_DF_DCCV_INATTIV1_9`, inattivi | tutto il flusso, anno 2025; `SEX=9`, `AGE=Y15-74`, `LABPROF_STATUS_A=IN` | n/d | ISTAT, CC BY 4.0 |
| ISTAT `32_38_DF_DCCV_QUINTOREDD_5`, famiglie per quinto di reddito equivalente e condizione del principale percettore | tutto il flusso, anno 2024; `EDU_LEV_MAIN_EARN=99`; 1A indipendente, 1B dipendente, 2A ritirato, 2B in cerca, 2C altra non professionale | n/d | ISTAT, CC BY 4.0 |
| MEF Finanze `REG_tipo_reddito_2025.csv`, dichiarazioni, anno d'imposta 2024: contribuenti per classe di reddito complessivo, regione e tipo di reddito | `https://www1.finanze.gov.it/finanze/analisi_stat/public/v_4_0_0/contenuti/REG_tipo_reddito_2025.csv`, sha256 `7a31be66…0643` | n/d | CC BY 3.0 IT |
| Banca d'Italia IBF 2022, `ind22_ascii.zip` (`CSV/carcom22.csv`, `CSV/rfam22.csv`) | `https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/documenti/ind22_ascii.zip`, sha256 `fd31bb42…b675` | n/d | microdati solo per ricerca: restano nella cache, nel repo solo l'aggregato |
| Bonaventura (2020), Okun regionale, tab. 2 | coefficienti `b` copiati da `docs/ricerca/modelli/economia-italiana-parametri.md:56-65` | n/d | solo coefficienti citati |

Endpoint: `https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/<dataset>` (JSON-stat) e `https://esploradati.istat.it/SDMXWS/rest/data/IT1,<flusso>,1.0/<chiave>` (SDMX-CSV). Il servizio ISTAT limita le interrogazioni al minuto e a volte risponde con un errore del server: lo script aspetta 15 secondi dopo ogni chiamata e riprova fino a tre volte, un minuto dopo l'errore.

## Licenze

- **Eurostat**: riuso consentito, anche commerciale, con citazione della fonte e dichiarazione delle modifiche (`docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:174`). Fonte: Eurostat, dataset elencati sopra. Le modifiche sono nella sezione seguente.
- **ISTAT**: CC BY 4.0 (`docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:173`). Fonte: Istat, dataset elencati sopra.
- **MEF, Dipartimento delle Finanze**: CC BY 3.0 IT, con attribuzione al Dipartimento delle Finanze e indicazione delle modifiche (`docs/ricerca/dati/mef-statistiche-fiscali.md`, sezione Licenza). Fonte: MEF, Dipartimento delle Finanze, open data delle dichiarazioni, anno d'imposta 2024.
- **Camera dei deputati**: licenza del CSV [NON TROVATO] (`docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md:34`); nel repo restano solo gli elettori per regione, come fatti [INF]. Fonte: Camera dei deputati, risultati delle politiche 2022.
- **ESS ERIC, European Social Survey round 11**: CC BY-NC-SA 4.0 sui dati (non commerciale, share-alike, citazione di ESS ERIC, dati gratuiti dal Data Portal) (`docs/ricerca/dati/ess.md:21`). Microdati fuori dal repo; le posizioni dei gruppi sono derivati, sotto la stessa licenza, dichiarata nell'intestazione di `elettorato.toml`.
- **Banca d'Italia, IBF 2022**: microdati distribuiti solo per ricerca, citazione obbligatoria. Non entrano nel repo: restano nella cache; il repo contiene la quota di ogni gruppo, un aggregato che li mescola con le altre fonti. La compatibilità di quest'uso con le condizioni IBF [da chiarire] (`dati-per-tarare-i-gruppi.md:129`, «quote derivate da chiarire»). Fonte: Banca d'Italia, Indagine sui bilanci delle famiglie italiane 2022.

## Modifiche fatte ai dati

Nessun file contiene i dati originali: salvo la piramide, i valori sono elaborati come segue.

**Popolazione.**
- `piramide` = residenti per età singola da ISTAT, 0..99 più «100 e oltre»; nessuna elaborazione oltre al taglio a intero.

**Tavola input-output (FIGARO 2024).**
- I 64 prodotti CPA sono aggregati nei 10 settori A*10 per somma, secondo la sezione del codice: A; B-E → BTE; F; G-I → GTI; J; K; L; M-N → MN; O-Q → OTQ; R-U → RTU.
- La produzione di ogni settore è la somma della colonna italiana: input intermedi italiani ed esteri più le voci di valore aggiunto (`D1`, `B2A3G`, `D29X39`) e le imposte nette sui prodotti (`D21X31`). Lo script controlla che coincida con la somma della riga (impieghi italiani ed esteri dei prodotti italiani).
- `a[i][j]`: input dal settore italiano `i` diviso per la produzione del settore `j`. `importazioni[j]`: input intermedi dai 50 paesi e aree diversi dall'Italia, divisi per la produzione di `j`. `valore_aggiunto[j] = 1 − Σ_i a[i][j] − importazioni[j]`, quindi comprende le imposte nette sui prodotti.
- Vettori di domanda finale, solo prodotti di origine italiana: `consumi` = colonne `P3_S14` + `P3_S15` dell'Italia, `investimenti` = `P51G` (variazione delle scorte `P5M` esclusa), `spesa_pubblica` = `P3_S13`, `esportazioni` = tutti gli impieghi esteri, intermedi e finali, dei prodotti italiani (equivalente di `P6` nella tavola inter-paese; gli acquisti diretti dei non residenti, `OP_NRES`, non hanno un prodotto e restano fuori).
- Coefficienti arrotondati a 6 decimali; ogni vettore di quote è normalizzato a somma 1, arrotondato a 6 decimali e corretto sull'ultima voce.

**Regioni.**
- Forze di lavoro arrotondate a 1 decimale, disoccupazione a 2. Le regioni diverse dal Trentino-Alto Adige hanno i valori Eurostat invariati.
- Trentino-Alto Adige da due province: forze di lavoro = somma delle province di Bolzano (`ITH1`) e Trento (`ITH2`); disoccupazione = media dei due tassi pesata sulle forze di lavoro, arrotondata a 2 decimali. Il valore aggiunto viene dal codice ISTAT della regione (`ITDA`).
- `mix`: valore aggiunto 2023 dei 10 settori A*10 (ISTAT `M_N` = `MN`) diviso per la loro somma, arrotondato come sopra. Lo script controlla che la somma dei 10 settori coincida col totale `_T`.

**Dicasteri.**
- Spesa delle AP 2024 per divisione COFOG ricondotta ai dieci dicasteri come in `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:88-99`: spesa = `TE − D62` (senza pensioni e altre prestazioni sociali in denaro, che stanno solo nella divisione 10) e senza la divisione 01.7 (interessi); salari = `D1`; acquisti = `P2 + D632`.
- Ogni valore è diviso per il PIL 2024 (`nama_10_gdp`, 2.210,6 miliardi) e moltiplicato per 100, arrotondato a 4 decimali.
- `settori` di Infrastrutture e Trasporti = vettore `investimenti` [assunzione del piano 3: le opere vanno in costruzioni e beni d'investimento].

## Scelte e ripieghi

- FIGARO 2024 è filtrabile per paese (`naio_10_fcp_ip4` copre dal 2022): la tavola simmetrica nazionale `naio_10_cp1700` del 2022, prevista come ripiego, non serve. `anno_tavola = 2024`.
- I valori COFOG sono quelli della risposta, non quelli arrotondati a un decimale della tabella di ricerca: le dieci spese coincidono con la tabella al decimale, la somma vale 26,0394% del PIL contro il 26,0382% di 575,6 miliardi.

## Elettorato

```
python3 -I strumenti/taratura/elettorato.py [--senza-ibf] [--cache DIR] [--ess FILE]   # scrive dati/base/elettorato.toml
python3 -I strumenti/taratura/elettorato.py --selftest [--cache DIR]      # IPF sintetico, invarianti e confronto col file committato
```

Legge `regioni.toml`, `elezioni.toml`, `scenario.toml`, `economia.toml`, scarica le fonti sopra e scrive 20 regioni, 360 gruppi (ordine canonico), interessi, panieri, istituti e nomi. Ogni fonte ha la sua cartella nella cache (`--cache`, predefinita `~/.cache/simulatore-politico-taratura/`), fuori dal repo; un file già scaricato non si riscarica, quindi la seconda esecuzione e il `--selftest` sono offline. Gli archivi sono dati: i CSV si leggono, lo zip IBF si legge in memoria per nome di file. Il servizio ISTAT va a 15 s fra le chiamate (riuso degli helper di `economia.py`).

**Quote dei gruppi (IPF 20 x 6 x 3).** Margini: (a) elettori per regione; (b) regione x condizione; (c) regione x fascia; (d) condizione x fascia nazionale. Il seme è la tabella IBF 2022 degli adulti pesati con `pesofit2`; le celle vuote (4 su 360) valgono lo 0,1% del prodotto dei margini. Con `--senza-ibf` il seme è l'indipendenza (prodotto dei margini). L'IPF si ferma quando ogni margine torna entro 1e-12 (relativo), ben dentro lo 0,1% chiesto.

- **(a) elettori**: somma di «ELETTORI TOTALI» per regione, 46.253.014 in tutto, 19 regioni. La Valle d'Aosta (collegio uninominale a parte, non nel file) è stimata dalla popolazione di `elezioni.toml`: 96.861 elettori, in proporzione agli elettori delle altre regioni. Il totale è 46.349.875; il selftest lo esige uguale a elettori del CSV + Valle d'Aosta.
- **(b) condizione**: dipendenti e indipendenti (occupati 2025, 15-74 anni) da ISTAT; **pubblico/privato: 17% dei dipendenti pubblico, assunzione** (RGS non aperto). Pensionati = persone con reddito da pensione, MEF a.i. 2024 (Trentino-Alto Adige = Trento + Bolzano). Studenti = 6% degli elettori, assunzione. Disoccupati o inattivi = disoccupati + inattivi 15-74 − pensionati − studenti, con minimo i disoccupati. Il minimo scatta in 11 regioni: piemonte, valle_d_aosta, liguria, lombardia, trentino_alto_adige, veneto, friuli_venezia_giulia, emilia_romagna, toscana, umbria, marche (i pensionati MEF includono chi lavora e chi ha più di 74 anni, gli inattivi no). `persone()` le elenca su stderr a ogni esecuzione, con lo scarto fra somma delle condizioni ed elettori (somma degli scarti positivi 613.363 persone, il 23/10/2026). Ogni regione è poi scalata ai suoi elettori: ISTAT conta i residenti non cittadini, le quote valgono per l'elettore.
- **(c) fascia**: le 34 classi MEF di reddito complessivo individuale diventano bassa, media o alta dal percentile nazionale del loro punto medio (sotto il 30% bassa, sopra il 70% alta: decili 3-4-3, `dati-per-tarare-i-gruppi.md:66`); per regione, quota dei contribuenti per fascia. È reddito individuale imponibile, non familiare equivalente (limite dichiarato nella ricerca). Poi si ricalibra (IPF 20 x 3) su righe = elettori e colonne = totali per fascia di (d), perché (c) e (d) coincidano.
- **(d) condizione x fascia**: quinti ISTAT 2024 per condizione del principale percettore, tradotti in fasce con metà del secondo e del quarto quinto (bassa = Q1 + Q2/2, media = Q2/2 + Q3 + Q4/2, alta = Q4/2 + Q5) e normalizzati a 1 (le percentuali ISTAT sommano 99,9 o 100,1). Dipendente privato e pubblico: dipendente; pensionato: ritirato dal lavoro; studente: altra condizione non professionale; disoccupato o inattivo: media di «in cerca» e «altra non professionale» col peso dei disoccupati sugli inattivi più disoccupati (7,9%).
- **Seme**: IBF `carcom22` (adulti, ETA >= 18) con il reddito familiare netto `Y` di `rfam22` diviso per la radice dei componenti; fasce sul percentile nazionale pesato come in (c). Condizione da `QUAL`: 1-3 dipendente (17% pubblico), 4-5 autonomo, 6 pensionato, 7 studente se sotto i 30 anni, altrimenti disoccupato o inattivo. **Il codebook dei file annuali non è stato trovato** (`docs/ricerca/dati/banca-italia-ibf.md:19`): la lettura di `QUAL` (conteggi e età coerenti: ritirati con età media 74) e l'ordine di `IREG` (quello ISTAT, confermato dal gradiente del reddito equivalente) sono assunzioni. Il settore `SETT` non entra.

**Modifiche ai dati.** Margini ISTAT e MEF sommati e riscalati agli elettori della Camera; classi di reddito aggregate in tre fasce; seme IBF aggregato in 360 celle e riscalato; il file contiene solo la quota di ogni gruppo e gli elettori per regione, nessun microdato. Lo script scrive `elettori` e `quota` di ogni gruppo, a 6 decimali con l'ultima voce corretta.

**Ripieghi.** Nessuna fonte ha fallito: tutte rispondono e le dimensioni citate esistono (verificato il 23/10/2026). Se IBF non fosse raggiungibile: `--senza-ibf`. Se mancasse il MEF, i pensionati non avrebbero ripiego (il margine (b) e (c) dipendono dal file).

**Posizioni (IPF 20 x 6 x 3 x 7, con `--ess FILE`).** Quarta dimensione: la posizione a 7 punti. Margini: (r, c, f) = elettori dei gruppi sopra; (c, f, p) nazionale = elettori di (c, f) per la distribuzione ESS per posizione di (c, f); (r, p) = elettori della regione per la distribuzione ESS della regione, accorpata alla macroarea sotto 100 casi, ricalibrata sui totali per posizione di (c, f, p) (IPF 20 x 7). Le distribuzioni sono lisciate verso la nazionale con 5 pseudo-casi (assunzione: nessuna cella ESS resta vuota). Taglio: 0-1 → 1, 2-3 → 2, 4 → 3, 5 → 4, 6 → 5, 7-8 → 6, 9-10 → 7; chi non si colloca (77, 88, 99) è escluso da tutti i margini. Colonne obbligatorie del CSV del Data Portal (se ne manca una, errore col nome): `essround` (si tiene solo 11), `cntry`, `lrscale`, `mainact`, `emplrel`, `tporgwk`, `hinctnta`, `region` (NUTS 2), `anweight`; il peso è normalizzato a somma = numero di righe, così 5 pseudo-casi e soglia dei 100 casi hanno lo stesso senso. Condizione: `mainact` 2 studente, 6 pensionato, 3-5 e 7-9 disoccupato o inattivo; 1 con `emplrel` 2-3 autonomo, con `emplrel` 1 dipendente, pubblico se `tporgwk` 1-2 (amministrazione o altro settore pubblico), privato se 3-5; un dipendente con `tporgwk` mancante o speciale non ha condizione (contato e stampato, resta solo nei margini della posizione). Lo script fallisce se meno del 90% dei casi italiani con `lrscale` valido ha regione mappata, fascia o condizione; i codici `region` non riconosciuti sono contati e riportati. Fascia: `hinctnta` 1-3 bassa, 4-7 media, 8-10 alta (assunzione: le 10 categorie sono decili nazionali, `ess.md:21`). Senza `--ess` le posizioni restano il segnaposto, uguale per tutti i 360 gruppi.

- **Licenza dei derivati ESS: CC BY-NC-SA 4.0** (ESS ERIC, European Social Survey round 11, https://www.europeansocialsurvey.org; i dati si scaricano gratis dal Data Portal). Il file `elettorato.toml` la dichiara nell'intestazione quando le posizioni vengono da ESS; i derivati restano share-alike e non commerciali. I microdati ESS non entrano mai nel repo: il file passato con `--ess` sta nella cache, fuori dal repo.
- **Stato al 23/10/2026: il file ESS non è stato scaricato.** Il Data Portal chiede registrazione e accesso; non è stato aggirato. Le posizioni nel file committato sono il segnaposto. Il percorso `--ess` è provato solo su CSV sintetici (selftest in memoria; esecuzione intera su un CSV di 2.800 righe), mai su dati ESS veri.
- **Passi per Mario** (la cache durevole è `~/.cache/simulatore-politico-taratura/`, non la cartella del job):
  1. Registrarsi (gratis) su https://ess.sikt.no e scaricare ESS round 11 in CSV, con le 9 colonne sopra (Italia o integrato).
  2. Salvare il file fuori dal repo: `~/.cache/simulatore-politico-taratura/ess/ESS11.csv`.
  3. Rigenerare: `python3 -I strumenti/taratura/elettorato.py --cache ~/.cache/simulatore-politico-taratura --ess ~/.cache/simulatore-politico-taratura/ess/ESS11.csv`.
  4. Verificare con gli stessi argomenti: `python3 -I strumenti/taratura/elettorato.py --selftest --cache ~/.cache/simulatore-politico-taratura --ess ~/.cache/simulatore-politico-taratura/ess/ESS11.csv`, poi `cargo test --workspace`, poi commit del `elettorato.toml` rigenerato. (Il selftest confronta col file committato solo con la stessa cache e lo stesso `--ess` con cui è stato generato.)

**Redditi (MEF).** Il primario di dipendenti (pubblici e privati insieme: il MEF non li distingue) e autonomi è il reddito medio per percettore della regione e della fascia, `Ammontare / Frequenza` delle classi di reddito complessivo ricondotte a fascia come in (c): dipendente = «Reddito da lavoro dipendente e assimilati»; autonomo = lavoro autonomo + imprenditore in contabilità ordinaria + semplificata. Dove una regione non ha percettori in una fascia vale la media nazionale della fascia. È reddito per percettore, non per elettore del gruppo, e ha un bias di composizione: la fascia si assegna sul reddito complessivo del contribuente, che mescola più tipi (chi ha lavoro dipendente e pensione, o altri redditi, ricade nella fascia per la somma), quindi la media di un tipo in una fascia non è il reddito di chi ha quella condizione (limite dichiarato). Pensionati: primario 0 e trasferimenti da `spesa_pensioni`. Resta provvisorio per disoccupati/inattivi e studenti (4.000 e 3.000 euro per moltiplicatori di fascia e macroarea) e per i trasferimenti.

**Panieri.** Interrogazione ISTAT della spesa per quinto: **non individuata**. Il catalogo ISTAT (`categoryscheme/IT1`) elenca la categoria `DCCV_SPEMEFAM_CONSDISTR_2` («Distribuzione delle spese per consumi», Coicop 2018), ma l'elenco dei flussi (`dataflow/IT1`) non risponde (connessione chiusa) e l'identificatore del flusso non è stato trovato. Resta il ripiego: tre panieri uguali al vettore `consumi`, così il paniere del turno zero vale 1 per ogni fascia (prezzi iniziali 1, quote che sommano a 1; `cargo test --workspace` verde).

**Provvisorio.** Posizioni senza `--ess`; redditi di disoccupati/inattivi e studenti e trasferimenti (moltiplicatori di fascia 0,55 / 1 / 1,9 e di macroarea 1,1 / 1 / 0,8); tre panieri uguali al vettore `consumi`; quattro istituti; dodici nomi e cognomi; pesi degli interessi del ticket 17.

- Assunzione: astensione = `ln((1-a)/a) + 0,081` con a l'affluenza 2022 per regione; Valle d'Aosta = affluenza nazionale 63,6%. La costante è tarata a mano coi parametri di default, fedeltà inclusa: la fedeltà al voto precedente sposta la stazionaria verso le alternative grandi, astensione compresa, e con lo 0,57 calcolato senza fedeltà l'affluenza del turno zero usciva 0,47 invece di 0,63 (`elettorato.py:58-60`). `sim-cli tara` (Task 12) la sostituisce.
- Il `--selftest` esegue per prima la parte offline (IPF sintetico 2 x 2 x 2: margini ritrovati entro 1e-9; una cella a zero resta zero se il margine lo consente; un margine che vuole massa dove il seme è vuoto fallisce; taglio 0-10 → 7 punti su tutti gli 11 valori; IPF a 4 dimensioni sintetico; un CSV ESS sintetico in memoria passato da `ess_righe`: righe 77/88/99, round 10 e altri paesi scartati, un peso 2 che sposta il margine (c, f, p), margini ricalcolati a parte, regioni sotto i 100 casi uguali alla macroarea, CSV guasti rifiutati con messaggio), poi scarica o legge la cache e controlla la somma degli elettori (CSV + Valle d'Aosta, esatta), che le quote arrotondate riproducano i margini (b) e (d) entro lo 0,1%, l'affluenza implicita fra 0,5 e 0,8 e che `dati/base/elettorato.toml` sia l'uscita dello script. Camera, IBF e MEF hanno lo sha256 verificato a ogni lettura (errore col nome del file); ISTAT no, perché cambia a ogni edizione: **il confronto col file committato vale solo con la cache fissata** (la stessa con cui è stato generato), altrimenti un aggiornamento ISTAT lo fa fallire. Il confronto esclude le righe `astensione = ` di `[[regione]]`, che sono di `sim-cli tara` (Task 12).

## Stima di beta e fedeltà (`stima_voto.py`)

Logit condizionale per massima verosimiglianza (Newton-Raphson, solo libreria standard) sul post-elettorale ITANES 2022: `U_k = −β₁₀·(x−z_k)² + λ_k + φ·[k = voto 2018]`, 8 partiti più astensione (λ = 0). `x` è l'autocollocazione 0-10 (`Q19`), `z_k` la media delle collocazioni del partito (`Q18a-h`), voto 2022 da `voto_post`/`Q10LHa`, voto 2018 da `Q14a`/`Q14b`. Stessa forma di `crates/sim-core/src/elettorato/voto.rs`. Il voto 2018 passa all'erede 2022 per famiglia politica (tabella `ERede18` nello script: Liberi e Uguali → AVS; Potere al popolo, altre liste e scheda bianca senza erede, quindi senza termine di fedeltà; astensione 2018 → astensione 2022). Scrive in `manopole.toml` `beta_distanza = β₁₀/0,36` e `fedelta = φ`, con gli errori standard nel commento, e avvisa se il valore esce da `min`/`max`.

```
python3 -I strumenti/taratura/stima_voto.py --selftest
python3 -I strumenti/taratura/stima_voto.py [--itanes CARTELLA] [--solo-stima] [--sensibilita]
```

- **Dati.** I file `Itanes2022_POST_release 01_weighted.tab` (Dataverse UNIMI, `doi:10.13130/RD_UNIMI/JV77WR`, file 119566, senza login né guestbook) stanno in `~/.cache/simulatore-politico-taratura/itanes/`, fuori dal repo: i termini ITANES vietano di ridistribuirli. Nel repo restano solo i parametri.
- **Selftest.** Dati sintetici con β e φ noti: errore < 5%, valore vero entro 4 errori standard, copertura a 2 ES su 40 repliche, scrittura del toml, cartella senza dati che esce con messaggio. Non serve la cartella ITANES.
- **Non fatto.** Il pacchetto `nopp` (CSES Italia 2006, formato R `.rda`) non è letto: la libreria standard non legge `.rda` e il banco di prova sintetico copre il codice.
- **Letture.** φ è un tetto (il ricordo del 2018 si sposta verso il voto 2022). I pesi di campionamento non sono usati. Chi non si colloca (965 su 4.696, astensione 30% contro 12%) è scartato nella stima base: `--sensibilita` mostra cosa cambia. Gli errori standard del toml sono quelli dell'informazione osservata, più stretti della variazione fra le specificazioni.
