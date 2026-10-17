---
tipo: dataset
fonti: [fonti/ess-licence.md, fonti/ess-r11-questionario.md]
aggiornato: 2026-10-17
---
# European Social Survey (ESS)

Indagine biennale a campione su atteggiamenti, valori e comportamenti politici in Europa, dal 2002 (round 1) al round 11 (2023), più il panel online CRONOS. Utile per distribuzioni di elettori (autocollocazione, fiducia, temi) per paese. Licenza **CC BY-NC-SA 4.0** sui dati: non commerciale e share-alike.

## Contenuto

- Round 1 (2002) - round 11 (2023), file integrati per round; moduli tematici a rotazione (es. round 10: democrazia; round 6: democrazia e benessere); l'Italia manca nel round 2 (fonti/ess-licence.md) [V].
- Panel CRONOS-2 (2021-23) e CRONOS-3 (2024-26), con moduli su populismo, tecnocrazia, fiducia istituzionale (fonti/ess-licence.md) [V].
- Domande stabili su autocollocazione sinistra-destra e fiducia in parlamento, partiti, politici [INF].
- Round 11, questionario sorgente: autocollocazione 0-10 (fonti/ess-r11-questionario.md:514); voto alle ultime elezioni nazionali (:372); attività principale (lavoro, studio, disoccupazione, pensione, :3103); dipendente o autonomo (:3214); tipo di organizzazione, con amministrazione centrale o locale e altro settore pubblico distinti dall'impresa privata (:3364); reddito familiare netto in 10 categorie (:3530) [V]. Che le categorie siano decili nazionali e che il file dati contenga la regione è [INF]. Bastano a ricavare le sei condizioni dei gruppi dell'elettorato ([[dati-per-tarare-i-gruppi]]).

## Licenza

- Dati: **CC BY-NC-SA 4.0**; documentazione: CC BY-SA 4.0 (fonti/ess-licence.md) [V].
- Dati disponibili senza restrizioni per scopi non commerciali; "commerciale" si giudica dall'attività, non dal tipo di ente (fonti/ess-licence.md) [V].
- Condizioni: citare ESS ERIC, dire che i dati si scaricano gratis dal Data Portal, non far credere a un'affiliazione (fonti/ess-licence.md) [V].
- ESS ERIC **raccomanda di non pubblicare i dataset su siti esterni** e di linkare il Data Portal; se si fa, indicare la versione e, se modificati, "Adapted from ESS Round X" (fonti/ess-licence.md) [V].

## Ridistribuibile in un repo pubblico?

**Solo se il gioco è non commerciale, e con la stessa licenza sui dati derivati**, contro la raccomandazione di ESS (fonti/ess-licence.md) [V]. Se il gioco sarà venduto, anche aggregati derivati (es. media della fiducia per paese) ricadono nel vincolo NC come materiale adattato [INF, da verificare con il filone legale]. Il gioco è gratuito e i dati stanno in una cartella separata con la propria licenza ([[decisioni]]): la condizione non commerciale è soddisfatta; restano lo share-alike sui derivati e la raccomandazione ESS di linkare il Data Portal.

## Per il nostro gioco

- Usarlo per la progettazione e la taratura offline; includere nel repo solo parametri del modello (non tabelle derivate) oppure preferire fonti CC BY come [[eurobarometro]] per le serie di fiducia.
- Deciso: gioco gratuito, dati in una cartella separata ([[decisioni]]). I derivati ESS possono entrare nella cartella dati in CC BY-NC-SA 4.0.
- Fonte consigliata per gli istogrammi della posizione da mettere nella cartella dati, al posto di [[itanes]] che ne vieta la ridistribuzione ([[dati-per-tarare-i-gruppi]]).

## Collegamenti

- [[polarizzazione-e-fiducia-istituzionale]]: fiducia e autocollocazione.
- [[opinion-dynamics]]: distribuzioni iniziali degli elettori.
- [[eurobarometro]]: alternativa con licenza più aperta per gli aggregati.

## Riferimenti

- **URL** https://www.europeansocialsurvey.org/node/58 · [V]
  **perché conta qui** condizioni d'uso ufficiali e raccomandazione contro la pubblicazione esterna
  **cosa se ne prende** CC BY-NC-SA 4.0, vincolo non commerciale, formula di citazione
- **URL** https://www.europeansocialsurvey.org/sites/default/files/2024-01/ESS%20Round%2011%20Source%20Questionnaire_FINAL_Alert%2004.pdf · [V]
  **perché conta qui** variabili disponibili per i gruppi dell'elettorato
  **cosa se ne prende** scala 0-10, attività, settore pubblico, reddito in 10 categorie, voto (fonti/ess-r11-questionario.md)
