---
tipo: dataset
fonti: [fonti/cses-imd-citazione.md, fonti/cses-indice-polarizzazione.md, fonti/itanes-2022-dataverse-termini.md]
aggiornato: 2026-10-17
---
# CSES (Comparative Study of Electoral Systems)

Moduli comuni inseriti negli studi elettorali nazionali dopo il voto: scelta di voto, collocazione di sé e dei partiti, giudizi sul governo, con le variabili sul sistema elettorale. È la base empirica di molti studi su voto spaziale, voto economico e polarizzazione. Gratuito con registrazione; licenza esplicita **non trovata**.

## Contenuto

- Integrated Module Dataset (IMD), versione del 27 febbraio 2024, doi:10.7804/cses.imd.2024-02-27; moduli 5 (2016-2021) e 6 (2021-2026) (fonti/cses-imd-citazione.md) [V].
- Italia 2022: la rilevazione post-elettorale ITANES usa il questionario del Modulo 6 CSES, N = 4696 (fonti/itanes-2022-dataverse-termini.md:79-81) [V]. Il dato italiano del Modulo 6 nasce quindi da [[itanes]], che ha termini d'uso espliciti; se la versione armonizzata CSES differisca da quella ITANES non è stato verificato [INF].
- Indice di polarizzazione dei sistemi partitici di Dalton per i moduli 1-4: posizioni percepite sinistra-destra pesate per voto, file Excel (fonti/cses-indice-polarizzazione.md) [V].

## Licenza

- Licenza dei dati CSES: [NON TROVATO] sulle pagine di download, registrazione e citazione del sito.
- Accesso: registrazione gratuita richiesta [INF, dalla struttura del sito di download].
- Citazione richiesta per l'IMD con la formula indicata (fonti/cses-imd-citazione.md) [V].
- Il file dell'indice di Dalton è dichiarato "contribuito al pubblico dominio" dagli autori (fonti/cses-indice-polarizzazione.md) [V].

## Ridistribuibile in un repo pubblico?

- Microdati CSES: **no** finché la licenza non è chiarita [INF].
- Indice di polarizzazione di Dalton: **sì**, è dichiarato di pubblico dominio (fonti/cses-indice-polarizzazione.md) [V].

## Per il nostro gioco

- Usare l'indice di Dalton come valore di taratura incluso nel repo.
- Usare i microdati offline per stimare i parametri del [[voto-spaziale]] (β, valence per gruppo) e includere solo i parametri stimati.

## Collegamenti

- [[voto-spaziale]], [[voto-economico-e-retrospettivo]], [[polarizzazione-e-fiducia-istituzionale]].
- [[itanes]]: la componente italiana del Modulo 6; [[dati-per-tarare-i-gruppi]]: stima di β per l'Italia.

## Riferimenti

- **URL** https://cses.org/data-download/cses-integrated-module-dataset-imd/funding-sources-and-how-to-cite-imd/ · [V]
  **perché conta qui** versione e citazione dell'IMD
  **cosa se ne prende** la versione da bloccare; nessuna licenza dichiarata
- **URL** https://cses.org/data-download/download-data-documentation/party-system-polarization-index-for-cses-modules-1-4/ · [V]
  **perché conta qui** indice già calcolato e in pubblico dominio
  **cosa se ne prende** valori di polarizzazione reali da includere
