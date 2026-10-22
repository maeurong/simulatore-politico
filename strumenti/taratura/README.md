# Taratura offline dei dati strutturali

`economia.py` scarica tavole e serie da Eurostat e ISTAT e scrive i dati di struttura del motore economico. Gira fuori dal binario del gioco: nel repo stanno solo i risultati aggregati e lo script che li ricava.

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
| Bonaventura (2020), Okun regionale, tab. 2 | coefficienti `b` copiati da `docs/ricerca/modelli/economia-italiana-parametri.md:56-65` | n/d | solo coefficienti citati |

Endpoint: `https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/<dataset>` (JSON-stat) e `https://esploradati.istat.it/SDMXWS/rest/data/IT1,<flusso>,1.0/<chiave>` (SDMX-CSV). Il servizio ISTAT limita le interrogazioni al minuto e a volte risponde con un errore del server: lo script aspetta 15 secondi dopo ogni chiamata e riprova fino a tre volte, un minuto dopo l'errore.

## Licenze

- **Eurostat**: riuso consentito, anche commerciale, con citazione della fonte e dichiarazione delle modifiche (`docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:174`). Fonte: Eurostat, dataset elencati sopra. Le modifiche sono nella sezione seguente.
- **ISTAT**: CC BY 4.0 (`docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md:173`). Fonte: Istat, dataset elencati sopra.

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

## Elettorato (provvisorio)

```
python3 -I strumenti/taratura/elettorato.py            # scrive dati/base/elettorato.toml
python3 -I strumenti/taratura/elettorato.py --selftest # solo invarianti, nessuna scrittura
```

Legge solo `regioni.toml`, `elezioni.toml`, `scenario.toml`, `economia.toml` e costanti citate; nessuna rete. Scrive 20 regioni, 360 gruppi (ordine canonico), interessi, panieri, istituti e nomi. Tutti i valori sono provvisori e la PR B sostituisce i corpi delle funzioni con le fonti vere.

- Assunzione: elettori proporzionali alla popolazione censita delle circoscrizioni della Camera, totale 46.350.000 (46.253.014 della Camera 2022 più la Valle d'Aosta).
- Assunzione: dipendenti pubblici 17% dei dipendenti (RGS non aperto); studenti 6% degli elettori.
- Assunzione: fasce per condizione dai quinti di reddito e dalle fasce 3-4-3; quota bassa x 1,3 nel Mezzogiorno, x 0,85 al Nord.
- Assunzione: forze di lavoro oltre gli elettori (stranieri, minorenni occupati). Il residuo dei disoccupati o inattivi è tagliato a 0 e le quote sono persone del gruppo / somma delle persone della regione. Taglio in valle_d_aosta (185), trentino_alto_adige (18.891), emilia_romagna (29.816). La PR B lo sostituisce con l'IPF.
- Assunzione: redditi per elettore (primario per condizione, moltiplicatori di fascia 0,55 / 1 / 1,9 e di macroarea 1,1 / 1 / 0,8); trasferimenti da `spesa_pensioni` e `spesa_sussidi`.
- Assunzione: astensione = `ln((1-a)/a) + 0,57` con a l'affluenza 2022 per regione; Valle d'Aosta = affluenza nazionale 63,6%.
- Segnaposto: istogrammi uguali per tutti (la PR B li ricava da ESS); tre panieri uguali al vettore `consumi`; quattro istituti; dodici nomi e cognomi; pesi degli interessi del ticket 17 (provvisori).
