---
tipo: dataset
fonti: [fonti/vdem-dataset.md]
aggiornato: 2026-10-16
---
# V-Dem (Varieties of Democracy)

Indicatori della qualità democratica per paese e anno, codificati da esperti e aggregati con un modello di misura bayesiano. Serve a dare a ogni paese simulato valori iniziali credibili per istituzioni, libertà, corruzione, polarizzazione. Licenza **CC BY-SA 4.0**: uso commerciale consentito, derivati con la stessa licenza.

## Contenuto

- Versione 16, marzo 2026; formati Stata, CSV, R, SPSS (fonti/vdem-dataset.md) [V].
- Paese-anno "Full+Others": 531 indicatori V-Dem, 251 indici, 62 indicatori da altre fonti; "Core": 5 indici di democrazia, 93 sotto-indici, 179 indicatori (fonti/vdem-dataset.md) [V].
- Versioni paese-data e a livello di singolo codificatore (fonti/vdem-dataset.md) [V].
- I punteggi latenti stanno di solito fra −4 e 4; i valori non si confrontano fra versioni diverse del dataset (fonti/vdem-dataset.md) [V].

## Licenza

- **CC BY-SA 4.0**: uso, adattamento e condivisione liberi anche commerciali, con attribuzione e con i prodotti derivati distribuiti con la stessa licenza (fonti/vdem-dataset.md) [V].
- Citazione richiesta per la v16: Coppedge et al. (fonti/vdem-dataset.md) [V].

## Ridistribuibile in un repo pubblico?

**Sì**, con attribuzione e mettendo in CC BY-SA i file di dati derivati (fonti/vdem-dataset.md) [V]. Tenerli in una cartella separata con la propria licenza evita di estendere lo share-alike al codice [INF]. È la strada decisa da Mario: codice open source, dati in una cartella separata ([[decisioni]]).

## Per il nostro gioco

- Fonte principale per lo stato istituzionale iniziale dei paesi (corruzione, indipendenza giudiziaria, libertà di stampa) e per gli scenari storici.
- Bloccare la versione (v16) nel repo: i valori cambiano fra versioni.

## Collegamenti

- [[polarizzazione-e-fiducia-istituzionale]]: corruzione e polarizzazione come stato iniziale.
- [[cicli-politico-economici-e-luna-di-miele]]: distinguere democrazie nuove e consolidate.

## Riferimenti

- **URL** https://www.v-dem.net/data/the-v-dem-dataset/ · [V]
  **perché conta qui** licenza ufficiale, versioni e contenuto del dataset
  **cosa se ne prende** CC BY-SA 4.0 con uso commerciale; v16; non confrontabilità fra versioni
