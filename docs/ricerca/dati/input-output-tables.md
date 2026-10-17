---
tipo: dataset
fonti: [fonti/eurostat-figaro-metadata.md, fonti/oecd-icio-dataset.md, fonti/wiod-2016-release.md, fonti/eurostat-copyright-notice.md, fonti/oecd-terms-conditions.md]
aggiornato: 2026-10-16
---
# Tavole input-output multi-paese (FIGARO, OECD ICIO, WIOD)

Tre tavole che dicono chi compra cosa da chi, per settore e per paese. Servono a calibrare la matrice $A$ di [[input-output-leontief]] e il commercio intermedio. Per il nostro gioco la scelta è fra copertura (ICIO), aggiornamento e licenza (FIGARO) e storia lunga (WIOD).

## Confronto

| | FIGARO (Eurostat-JRC) | OECD ICIO | WIOD 2016 |
|---|---|---|---|
| paesi | 27 UE, 2 EFTA, 5 candidati, 15 partner, resto del mondo | 80 economie e resto del mondo (ed. 2025, revisione di metà gennaio 2026) | 43 paesi e resto del mondo |
| settori | 64 | [NON TROVATO nella cattura] | 56 (ISIC Rev. 4) [non nella pagina catturata, da ricerca web] |
| anni | 2010 a T-2 (ed. 2026: 2010-2024) | dal 1995 (archivi scaricabili) | 2000-2014 |
| licenza | riuso Eurostat con citazione | dati OCSE, riuso anche commerciale con attribuzione | CC BY 4.0 |
| fonti | fonti/eurostat-figaro-metadata.md, fonti/eurostat-copyright-notice.md | fonti/oecd-icio-dataset.md, fonti/oecd-terms-conditions.md | fonti/wiod-2016-release.md |

FIGARO è in milioni di euro a prezzi base, a prezzi correnti; le tavole a prezzi dell'anno precedente sono attese per dicembre 2026 (fonti/eurostat-figaro-metadata.md) [V]. WIOD è in milioni di dollari correnti e include un modello del resto del mondo (fonti/wiod-2016-release.md) [V]. Gli archivi ICIO partono dal 1995 (link «1995-1999» nella pagina) (fonti/oecd-icio-dataset.md) [V].

## Si può ridistribuire in un repo pubblico?

Tutte e tre sì, con attribuzione [V per le licenze]. Una tavola intera pesa molto: conviene versionare nel repo solo la versione aggregata a 10-15 settori, e lo script che la ricava dall'originale [INF].

## Per il nostro gioco

- Raccomandazione: FIGARO per l'Europa (aggiornata ogni anno, licenza semplice, paese giocabile probabile europeo), ICIO per il resto del mondo se serve. WIOD solo come riserva storica: è ferma al 2014.
- Aggregare a 10-15 macrosettori prima di entrare nel gioco (vedi [[input-output-leontief]]).

## Fonti

- **URL** https://ec.europa.eu/eurostat/cache/metadata/en/naio_10_fcp_esms.htm · [V]
  **perché conta qui** copertura e cadenza di FIGARO
  **cosa se ne prende** dimensioni, anni, unità di misura (fonti/eurostat-figaro-metadata.md)
- **URL** https://www.oecd.org/en/data/datasets/inter-country-input-output-tables.html (snapshot Wayback 2026-10-06) · [V]
  **perché conta qui** l'alternativa mondiale
  **cosa se ne prende** 80 economie, edizione 2025 (fonti/oecd-icio-dataset.md)
- **URL** https://www.rug.nl/ggdc/valuechain/wiod/wiod-2016-release?lang=en · [V]
  **perché conta qui** l'unica delle tre con licenza CC BY esplicita sulla pagina del dataset
  **cosa se ne prende** CC BY 4.0, 2000-2014 (fonti/wiod-2016-release.md)

## Collegamenti

- [[input-output-leontief]]
- [[eurostat]], [[oecd]]
