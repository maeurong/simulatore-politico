Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Con quali dati aperti, e con quali licenze, si tarano i 360 gruppi dell'elettorato (Elettorato, 06)?
1. Quote di popolazione con diritto di voto per regione × condizione (dipendente privato, dipendente pubblico, autonomo, pensionato, disoccupato o inattivo, studente) × fascia di reddito (bassa, media, alta): esiste la tabella incrociata (ISTAT, Eurostat, Banca d'Italia IBF) o va ricostruita da marginali?
2. Posizione di partenza: autocollocazione sinistra-destra per regione, condizione e reddito (ESS, CSES, ITANES, Eurobarometro), abbastanza fitta da dare un istogramma su 7 punti per gruppo, o da ricostruire con un modello?
3. Valori di partenza per β del logit spaziale, fedeltà al voto precedente e affluenza per gruppo in Italia.
Ogni dato con URL, licenza e compatibilità con la cartella dati separata (`docs/ricerca/sintesi/decisioni.md`, 16/10).

## Answer

Ricerca del 17/10/2026, dettaglio e fonti in `docs/ricerca/sintesi/dati-per-tarare-i-gruppi.md`.

1. **Quote dei 360 gruppi**: la tabella regione × condizione × fascia non è pubblicata [NON TROVATO] (`:12`). Ci sono i margini: ISTAT per regione (dipendenti, indipendenti, disoccupati, inattivi), MEF per regione × classe × tipo di reddito (CC BY 3.0 IT), ISTAT EU-SILC nazionale per condizione × quinto. Seme: microdati IBF 2022 della Banca d'Italia. Metodo: IPF sui margini [INF] (`:56-62`). Proposta, non decisa: fasce sul reddito equivalente, bassa = decili 1-3, media 4-7, alta 8-10 (`:66`).
2. **Istogrammi della posizione**: nessuna indagine abbastanza fitta; ITANES 2022 ha tutte le variabili ma circa 17 casi per gruppo. Stessa IPF con la posizione come quarta dimensione [INF]. Licenze: ITANES vieta di ridistribuire i microdati, quindi gli istogrammi della cartella dati si costruiscono da ESS (CC BY-NC-SA) e ITANES resta per stime offline (`:95`).
3. **β, fedeltà, affluenza**: β e fedeltà italiani [NON TROVATO], da stimare offline su ITANES (logit condizionale; matrice voto 2018 → 2022). Un β su scala 0-10 va diviso per 0,36 sulla scala a 7 punti (`:99-104`). Affluenza 2022 per regione misurata dal CSV della Camera, dal 50,6% (Calabria) al 71,6% (Emilia-Romagna); per condizione solo da sondaggio, da ricalibrare.

Effetto sulla mappa: lo Scenario iniziale (nebbia) eredita il metodo di taratura; la scelta delle fasce sui decili va confermata con il Modello economico (07), che le usa per reddito e panieri.
