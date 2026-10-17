---
tipo: dataset
fonti: [fonti/camera-risultati-2022-csv.md, fonti/cise-volatilita-dataset.md]
aggiornato: 2026-10-17
---
# Risultati elettorali italiani: affluenza e volatilità

Dati ufficiali delle elezioni politiche (elettori, votanti, voti di lista) e indici aggregati di volatilità. Danno l'affluenza reale per regione, su cui si ricalibrano le stime per gruppo, e un tetto aggregato alla fedeltà di voto.

## Contenuto

- La Camera pubblica in CSV ed Excel i risultati delle politiche del 25 settembre 2022: riepilogo nazionale e un file per circoscrizione, con elettori iscritti, votanti, voti validi, bianche e nulle, cifre di coalizioni e liste (fonti/camera-risultati-2022-csv.md:10) [V].
- Riepilogo nazionale `ItaliaVotiListeCIRC.csv`: 46.253.014 elettori e 29.414.782 votanti, affluenza 63,6%, senza la Valle d'Aosta (collegio uninominale a parte) e senza l'Estero [M: `curl` di https://documenti.camera.it/leg19/risultatiElettorali2022/UGE/VotiListe/ItaliaVotiListeCIRC.csv, sha256 bbda02a9…7f3f, somma delle colonne «ELETTORI TOTALI» e «VOTANTI», circoscrizioni accorpate per regione, in sessione 2026-10-17].
- Affluenza 2022 per regione, stesso calcolo [M]:

| Regione | Affluenza | Regione | Affluenza |
|---|---|---|---|
| Calabria | 50,6% | Trentino-Alto Adige | 65,7% |
| Sardegna | 53,0% | Friuli-Venezia Giulia | 66,0% |
| Campania | 53,2% | Piemonte | 66,1% |
| Molise | 56,4% | Marche | 68,2% |
| Puglia | 56,5% | Umbria | 68,5% |
| Sicilia | 56,9% | Toscana | 69,4% |
| Basilicata | 58,6% | Lombardia | 69,8% |
| Lazio | 63,4% | Veneto | 70,0% |
| Abruzzo | 63,7% | Emilia-Romagna | 71,6% |
| Liguria | 63,9% | Valle d'Aosta | non nel file |

- Il DAIT del Ministero dell'Interno (Eligendo) pubblica open data elettorali dal 1946, ma nella pagina catturata non compare una licenza [NON TROVATO, pagina https://dait.interno.gov.it/elezioni/open-data letta in sessione, non salvata].
- Dataset CISE della volatilità elettorale (Emanuele 2015): elezioni per la camera bassa in 20 paesi dell'Europa occidentale, 1945-2015, 347 elezioni, con aggiornamenti successivi (fonti/cise-volatilita-dataset.md:15, :17) [V]. Il file di aggiornamento (8 luglio 2024) non si è scaricato in sessione: i valori italiani 2018 e 2022 non sono stati letti [NON TROVATO].

## Licenza

- Camera dei deputati: licenza dei file CSV **non trovata** sulla pagina dei risultati [NON TROVATO]. Sono risultati ufficiali di un'elezione; l'art. 5 della L. 633/1941 esclude dalla tutela i testi degli atti ufficiali dello Stato, ma che copra anche queste tabelle è un'inferenza da verificare con il filone legale [INF].
- CISE: formula di citazione indicata (fonti/cise-volatilita-dataset.md:19-21) [V]; licenza [NON TROVATO].

## Ridistribuibile in un repo pubblico?

- Venti percentuali di affluenza regionale sono fatti, non un'opera: si possono scrivere nella cartella dati citando la Camera [INF].
- I file completi della Camera: da chiarire prima di copiarli [INF].

## Per il nostro gioco

- L'affluenza regionale 2022 è il bersaglio su cui si ricalibra l'astensione dei gruppi: la media pesata dei gruppi di una regione deve ridare il valore ufficiale ([[dati-per-tarare-i-gruppi]]).
- I voti di lista per circoscrizione servono a controllare che i sette partiti inventati, tarati con [[ches]], ridiano un emiciclo plausibile [INF].

## Collegamenti

- [[sistemi-elettorali]]: i seggi si ripartiscono sulle cifre regionali.
- [[itanes]]: l'affluenza dichiarata nei sondaggi, da riportare a questi valori.
- [[sondaggi-ed-errore]]

## Riferimenti

- **URL** https://www.camera.it/deputati/elezioni/risultati-19-relazioni-csv-excel · [V]
  **perché conta qui** risultati ufficiali 2022 in CSV, verificati dalla Giunta delle elezioni
  **cosa se ne prende** elettori e votanti per circoscrizione, quindi l'affluenza per regione (fonti/camera-risultati-2022-csv.md)
- **URL** https://cise.luiss.it/cise/dataset-of-electoral-volatility-and-its-internal-components-in-western-europe-1945-2015/ · [V]
  **perché conta qui** volatilità aggregata: è un cambiamento netto, quindi un limite inferiore alla quota di elettori che cambiano voto
  **cosa se ne prende** citazione e struttura del dataset; valori italiani da leggere a mano (fonti/cise-volatilita-dataset.md)
