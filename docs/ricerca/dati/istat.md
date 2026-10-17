---
tipo: dataset
fonti: [fonti/istat-note-legali.md]
aggiornato: 2026-10-17
---
# ISTAT

Istituto nazionale di statistica: conti nazionali, finanza pubblica, lavoro, prezzi, distribuzione dei redditi per l'Italia. Per il nostro gioco serve se l'Italia è un paese giocabile in dettaglio; per il resto Eurostat ha gli stessi dati armonizzati.

## Licenza

Salvo diversa indicazione, tutti i contenuti del sito sono sotto Creative Commons Attribuzione 4.0 (fonti/istat-note-legali.md, cattura di un altro agente) [V].

## Si può ridistribuire in un repo pubblico?

Sì, con attribuzione, controllando che la singola tavola non abbia un'indicazione diversa [V per la regola].

## Per il nostro gioco

- Dati di dettaglio per l'Italia (redditi per decile, spesa per funzione, entrate per imposta) dove Eurostat è più aggregato [INF].
- Se l'Italia non ha un trattamento speciale, basta [[eurostat]].
- L'Italia è l'unico paese giocabile ([[decisioni]]): ISTAT dà i margini per regione con cui si ricostruiscono le quote dei 360 gruppi dell'elettorato ([[dati-per-tarare-i-gruppi]]).

## Tavole per i gruppi dell'elettorato

Interrogate via SDMX (`https://esploradati.istat.it/SDMXWS/rest/data/IT1,<flusso>,1.0/all`) in sessione il 2026-10-17 [M]:
- `150_938_DF_DCCV_OCCUPATIT1_23`: occupati per posizione professionale (dipendenti, indipendenti) per regione e provincia; 2025: 18,93 milioni di dipendenti e 5,18 milioni di indipendenti [M].
- `151_929_DF_DCCV_DISOCCUPT1_6`: disoccupati 15-74 per regione; 2025: 1,58 milioni [M].
- `152_928_DF_DCCV_INATTIV1_9`: inattivi 15-64 e 15-74 per regione, senza distinguere pensionati e studenti [M].
- `32_38_DF_DCCV_QUINTOREDD_5`: famiglie per quinto di reddito equivalente × condizione del principale percettore (indipendente, dipendente, ritirato dal lavoro, in cerca, altra condizione), solo Italia [M].
- `32_292_DF_DCCV_REDNETFAMFONTERED_8`: reddito familiare medio e mediano per condizione del principale percettore, per le cinque ripartizioni [M].
- Nessuna tavola incrocia regione, condizione e reddito [NON TROVATO fra i nomi dei flussi del catalogo `dataflow/IT1`].

## Fonti

- **URL** https://www.istat.it/note-legali/ · [V]
  **perché conta qui** licenza dei dati ISTAT
  **cosa se ne prende** CC BY 4.0 salvo diversa indicazione (fonti/istat-note-legali.md, cattura di un altro agente)
- **URL** https://esploradati.istat.it/SDMXWS/rest/dataflow/IT1/all/latest · [M]
  **perché conta qui** catalogo dei flussi SDMX: dice quali incroci esistono e quali no
  **cosa se ne prende** i cinque flussi regionali e nazionali usati per i gruppi dell'elettorato

## Collegamenti

- [[eurostat]]
- [[mercato-del-lavoro-e-disuguaglianza]]
- [[dati-per-tarare-i-gruppi]]: uso dei margini regionali per i gruppi dell'elettorato.
- [[mef-statistiche-fiscali]], [[banca-italia-ibf]]: le altre fonti della ricostruzione.
