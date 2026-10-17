---
tipo: dataset
fonti: [fonti/bdi-ibf-distribuzione-microdati.md]
aggiornato: 2026-10-17
---
# Banca d'Italia: Indagine sui bilanci delle famiglie (IBF)

Indagine campionaria della Banca d'Italia su redditi, ricchezza e lavoro delle famiglie italiane. I microdati sono gratuiti e contengono nello stesso record regione, condizione professionale e reddito: sono il miglior punto di partenza per ricostruire la tabella incrociata dei 360 gruppi. Sono però distribuiti solo a fini di ricerca.

## Contenuto

- Microdati in archivi annuali (`INDxx`) e in un archivio storico (versione 12.2), in CSV, SAS e Stata; ultima edizione pubblicata: indagine sul 2022, rilasciata il 27 ottobre 2025 (fonti/bdi-ibf-distribuzione-microdati.md:15, :21, :36-37) [V].
- Dal 2020 il disegno campionario è cambiato; per le analisi trasversali si usano i pesi `PESOFIT`/`PESOFIT2` dell'archivio `CARCOMxx` (fonti/bdi-ibf-distribuzione-microdati.md:23) [V].
- Archivio 2022, file `carcom22.csv`: 23.057 individui in 9.641 famiglie, 20.144 adulti; variabili `IREG` (20 regioni), `QUAL` (7 modalità), `SETT` (5 modalità), `AREA5`, `ETA`, `pesofit2` [M: `unzip ind22_ascii.zip` da https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/documenti/ind22_ascii.zip, sha256 dello zip fd31bb42…b675, poi conteggio con `csv.DictReader`, in sessione 2026-10-17].
- Adulti per regione nel campione 2022: da 79 (la regione più piccola) a 2.393 [M, stesso comando]. Con 360 gruppi fanno in media 56 adulti per gruppo, molti meno nelle regioni piccole.
- Codifica delle modalità: il codebook dell'archivio storico descrive la qualifica come dipendente (operaio, impiegato, quadro, dirigente), indipendente o «in condizione non professionale», e il settore con la voce «P.A., difesa, istruzione, sanità e altri servizi pubblici» [M: stringhe del file `documentazione_storico_versione_12.2.xlsx` dalla pagina Documentazione]. La codifica a 7 e 5 modalità dei file annuali 2022 **non è stata trovata** in un codebook dei file annuali [NON TROVATO]: va verificata prima dell'uso.
- Il settore distingue l'attività economica, non il datore di lavoro: un dipendente di una società privata nella sanità può finire nella stessa voce di un dipendente pubblico [INF].

## Licenza

- «I dati elementari anonimizzati dell'Indagine sono distribuiti esclusivamente a fini di ricerca»; l'uso è sotto la responsabilità dell'autore; citazione obbligatoria «Banca d'Italia, Indagine sui bilanci delle famiglie italiane» con la versione dell'archivio; i riferimenti dei lavori vanno inviati alla Banca (fonti/bdi-ibf-distribuzione-microdati.md:9, :11) [V].
- Nessuna licenza con nome (Creative Commons o simili) sulla pagina di distribuzione [NON TROVATO].

## Ridistribuibile in un repo pubblico?

- Microdati: **no**. La distribuzione è limitata alla ricerca e la pagina non prevede la ripubblicazione [INF dalla clausola «esclusivamente a fini di ricerca»].
- Quote dei gruppi ricostruite usando l'IBF come punto di partenza (seme) e ricalibrate su totali ISTAT e MEF: sono un prodotto derivato in cui il contributo dell'IBF è solo la struttura delle associazioni. Se possano stare nella cartella dati è da chiarire con la Banca [INF].

## Per il nostro gioco

- Seme della ricostruzione delle quote 20 × 6 × 3 con la ricalibratura iterativa (IPF) descritta in [[dati-per-tarare-i-gruppi]].
- Da evitare: usare l'IBF come unica fonte delle quote; il campione è troppo piccolo per 360 celle.
- Aperto: permesso della Banca d'Italia per le quote derivate; codebook dei file annuali.

## Collegamenti

- [[istat]], [[mef-statistiche-fiscali]]: i totali su cui si ricalibra.
- [[mercato-del-lavoro-e-disuguaglianza]]: i 10 decili su cui devono mapparsi le fasce di reddito.
- [[elettorato-e-gruppi-di-voto]]

## Riferimenti

- **URL** https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/index.html · [V]
  **perché conta qui** condizioni d'uso e file dei microdati IBF
  **cosa se ne prende** solo ricerca, citazione obbligatoria, pesi `PESOFIT2` per le analisi trasversali (fonti/bdi-ibf-distribuzione-microdati.md)
- **URL** https://www.bancaditalia.it/statistiche/tematiche/indagini-famiglie-imprese/bilanci-famiglie/distribuzione-microdati/documenti/ind22_ascii.zip · [M]
  **perché conta qui** misura diretta di cosa c'è nei microdati 2022
  **cosa se ne prende** presenza di `IREG`, `QUAL`, `SETT`; numerosità per regione
