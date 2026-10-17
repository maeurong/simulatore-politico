---
tipo: dataset
fonti: [fonti/parlgov-about.md, fonti/parlgov-data-info.md, fonti/parlgov-dataverse-2024-licenza.md]
aggiornato: 2026-10-16
---
# ParlGov

Base dati su partiti, elezioni e governi delle democrazie UE e OCSE. È la fonte per tarare la [[formazione-di-coalizioni]] (chi governa con chi, quanto durano i governi) e per i risultati elettorali storici. Il rilascio 2024 è in **CC0**: si può includere nel repo.

## Contenuto

- 37 paesi, circa 1700 partiti, 1000 elezioni (9800 risultati), 1600 governi (4000 partecipazioni di partito), dal 1900 al 2023 (fonti/parlgov-about.md) [V].
- Versione di sviluppo (4 novembre 2023) scaricabile come CSV, Excel, SQLite, con codebook (fonti/parlgov-data-info.md) [V].
- Rilascio stabile 2024 su Harvard Dataverse, doi:10.7910/DVN/2VZ5ZC (fonti/parlgov-about.md) [V].

## Licenza

- Rilascio 2024 su Dataverse: **CC0 1.0** (fonti/parlgov-dataverse-2024-licenza.md) [V].
- Licenza della versione di sviluppo sul sito: [NON TROVATO] nelle pagine lette; usare il rilascio Dataverse.
- Citazione: Döring e Manow 2024, ParlGov Release 2024 (fonti/parlgov-about.md) [V].

## Ridistribuibile in un repo pubblico?

**Sì**, il rilascio 2024 (CC0) (fonti/parlgov-dataverse-2024-licenza.md) [V]. Citare comunque la fonte per correttezza.

## Per il nostro gioco

- Includere le tabelle di governi ed elezioni italiane (ed europee) come dati di partenza e per i test: durata dei governi, composizione delle coalizioni, risultati.
- Stimare i pesi del logit di formazione delle coalizioni sui governi europei.

## Collegamenti

- [[formazione-di-coalizioni]]: taratura.
- [[sistemi-elettorali]]: risultati storici per testare il motore di riparto.
- [[ches]]: posizioni dei partiti, collegabili tramite id di partito.

## Riferimenti

- **URL** https://dataverse.harvard.edu/api/datasets/:persistentId/?persistentId=doi:10.7910/DVN/2VZ5ZC · [V]
  **perché conta qui** campo licenza ufficiale del rilascio 2024
  **cosa se ne prende** CC0: inclusione nel repo consentita
- **URL** https://parlgov.org/about/ · [V]
  **perché conta qui** copertura del database
  **cosa se ne prende** 37 paesi, 1600 governi, 1900-2023
- **URL** https://parlgov.org/data-info/ · [V]
  **perché conta qui** formati disponibili
  **cosa se ne prende** SQLite e CSV pronti da importare
