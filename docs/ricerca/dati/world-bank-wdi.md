---
tipo: dataset
fonti: [fonti/worldbank-wdi-about.md, fonti/worldbank-terms-datasets.md, fonti/worldbank-public-licenses.md]
aggiornato: 2026-10-16
---
# World Bank: World Development Indicators (WDI)

Raccolta principale della Banca Mondiale di dati comparabili fra paesi: 1.400 serie per 217 economie e oltre 40 gruppi di paesi, molte dal 1960 circa (fonti/worldbank-wdi-about.md) [V]. Per il nostro gioco è la base di calibrazione più larga con licenza aperta: PIL, popolazione, spesa pubblica, commercio, Gini, povertà.

## Licenza

- Salvo diversa indicazione, i dataset sono CC BY 4.0 con un'aggiunta vincolante: controversie risolte con mediazione e poi arbitrato (fonti/worldbank-terms-datasets.md) [V].
- Attribuzione nella forma «The World Bank: Dataset name: Data source (if known)», da ripetere nelle sottolicenze (fonti/worldbank-terms-datasets.md) [V].
- Alcuni dataset e indicatori vengono da terzi e possono non essere ridistribuibili senza il loro consenso; le condizioni stanno nei metadati del singolo indicatore (fonti/worldbank-terms-datasets.md) [V].
- Il catalogo dati elenca le licenze possibili: CC BY 4.0 di default per i dataset prodotti dalla Banca, con la clausola di mediazione e arbitrato; ODbL (share-alike) quando lo impone il fornitore; Microdata Research License, che vieta di ridistribuire; licenze esterne o personalizzate indicate nei metadati (fonti/worldbank-public-licenses.md) [V].

## Si può ridistribuire in un repo pubblico?

Sì per gli indicatori prodotti dalla Banca Mondiale, con attribuzione e con il testo dell'aggiunta sulle controversie. Prima di includere un indicatore va letto il campo licenza dei suoi metadati: quelli di fonte terza sono l'eccezione da filtrare [INF].

## Per il nostro gioco

- Prendiamo i valori iniziali per paese (PIL, debito, spesa per funzione, quote commerciali, Gini) e le serie storiche per verificare che il motore riproduca ordini di grandezza credibili.
- Un file di attribuzione nel repo, generato dallo script di estrazione, con un'entrata per indicatore [INF].

## Fonti

- **URL** https://datatopics.worldbank.org/world-development-indicators/ · [V]
  **perché conta qui** copertura del dataset
  **cosa se ne prende** 1.400 indicatori, 217 economie (fonti/worldbank-wdi-about.md)
- **URL** https://www.worldbank.org/ext/en/legal/terms-conditions/datasets · [V]
  **perché conta qui** la licenza che decide se i dati entrano nel repo
  **cosa se ne prende** CC BY 4.0 con clausola sulle controversie ed eccezione per dati di terzi (fonti/worldbank-terms-datasets.md)
- **URL** https://datacatalog.worldbank.org/public-licenses · [V]
  **perché conta qui** dice quali altre licenze può portare un indicatore oltre a CC BY 4.0
  **cosa se ne prende** ODbL e Microdata Research License come casi da filtrare prima di includere un dato (fonti/worldbank-public-licenses.md)

## Collegamenti

- [[mercato-del-lavoro-e-disuguaglianza]]: Gini e decili.
- [[motore-economico-raccomandato]]: dove si decide quali dati servono.
