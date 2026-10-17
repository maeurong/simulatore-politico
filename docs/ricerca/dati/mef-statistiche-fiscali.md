---
tipo: dataset
fonti: [fonti/mef-finanze-open-data-dichiarazioni.md, fonti/mef-finanze-open-data-pensionati.md]
aggiornato: 2026-10-17
---
# MEF, Dipartimento delle Finanze: open data delle dichiarazioni dei redditi

Statistiche sulle dichiarazioni IRPEF pubblicate ogni anno dal Dipartimento delle Finanze, in CSV, per regione, classe di reddito, tipologia di reddito, comune. Sono conteggi completi, non campionari, con licenza aperta: il miglior dato per il legame fra regione, fonte del reddito (lavoro dipendente, pensione, lavoro autonomo) e livello di reddito.

## Contenuto

- Catalogo «Open Data Dichiarazioni»: per l'anno d'imposta 2024 (pubblicato il 23 aprile 2026) c'è la tavola «Tipologia di reddito» con classificazione «Classi di reddito e Regione», con frequenza e ammontare per reddito da lavoro dipendente, da pensione, da lavoro autonomo e altri (fonti/mef-finanze-open-data-dichiarazioni.md:51-73) [V].
- Tavole per i soli pensionati e per i soli lavoratori dipendenti, classificate per regione e, separatamente, per classe di reddito complessivo (fonti/mef-finanze-open-data-pensionati.md:51-63, :111-123) [V].
- File `REG_tipo_reddito_2025.csv`: 748 righe, 22 unità territoriali per 34 classi di reddito complessivo; 42.837.946 contribuenti, di cui 24.127.859 con reddito da lavoro dipendente, 14.527.380 con reddito da pensione, 512.317 con reddito da lavoro autonomo [M: `curl` di https://www1.finanze.gov.it/finanze/analisi_stat/public/v_4_0_0/contenuti/REG_tipo_reddito_2025.csv, sha256 7a31be66…0643, somma delle colonne «Frequenza», in sessione 2026-10-17].
- Le 22 unità territoriali sono probabilmente le 20 regioni con Trentino-Alto Adige diviso nelle due province autonome, più una voce residua [INF, non verificato riga per riga].

## Limiti per i nostri gruppi

- Le tipologie non si escludono: chi ha sia salario sia pensione compare in tutte e due le frequenze [INF dalla struttura del file].
- I lavoratori autonomi sono sottostimati: 512 mila con reddito da lavoro autonomo contro oltre 5 milioni di occupati indipendenti ISTAT [M, confronto con [[istat]]]. Il resto sta nei redditi d'impresa o nei regimi forfettari, fuori dal reddito complessivo IRPEF [INF].
- Reddito individuale e imponibile, non familiare equivalente: le fasce di reddito dei gruppi, se definite sul reddito familiare, non coincidono con queste classi [INF].
- Chi non presenta dichiarazione e non ha redditi (molti studenti, parte degli inattivi) manca [INF].

## Licenza

- Ogni tavola del catalogo porta l'icona «Licenza Creative Commons» con link a **CC BY 3.0 Italia** (fonti/mef-finanze-open-data-dichiarazioni.md:65-67; fonti/mef-finanze-open-data-pensionati.md) [V].

## Ridistribuibile in un repo pubblico?

Sì, con attribuzione al Dipartimento delle Finanze e indicazione delle modifiche (CC BY 3.0 IT) [V per la licenza].

## Per il nostro gioco

- Margine regione × classe di reddito, separato per dipendenti e pensionati, nella ricostruzione delle quote dei gruppi ([[dati-per-tarare-i-gruppi]]).
- Da evitare: usarlo per gli autonomi senza correggere per i regimi fuori IRPEF.
- Aperto: come tradurre classi di reddito individuale in fasce di reddito familiare (via [[banca-italia-ibf]], che ha entrambe).

## Collegamenti

- [[istat]]: occupati, inattivi e disoccupati per regione.
- [[banca-italia-ibf]]: il seme della ricostruzione.
- [[mercato-del-lavoro-e-disuguaglianza]]: i decili di reddito dell'economia.

## Riferimenti

- **URL** https://www1.finanze.gov.it/finanze/analisi_stat/public/index.php?opendata=yes · [V]
  **perché conta qui** catalogo ufficiale delle tavole IRPEF con la licenza di ciascuna
  **cosa se ne prende** tavola tipologia di reddito × classe × regione, licenza CC BY 3.0 IT (fonti/mef-finanze-open-data-dichiarazioni.md)
- **URL** https://www1.finanze.gov.it/finanze/analisi_stat/public/index.php?search_contrib=PFPEN&opendata=yes · [V]
  **perché conta qui** tavole dei soli pensionati
  **cosa se ne prende** pensionati per regione e per classe di reddito (fonti/mef-finanze-open-data-pensionati.md)
