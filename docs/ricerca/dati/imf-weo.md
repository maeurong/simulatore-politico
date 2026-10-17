---
tipo: dataset
fonti: [fonti/imf-copyright-terms.md]
aggiornato: 2026-10-16
---
# IMF: World Economic Outlook (WEO)

Database del Fondo Monetario con aggregati macro per paese e previsioni a medio termine, aggiornato due volte l'anno [INF: la pagina del database non è stata catturata, il sito IMF blocca i client automatici]. Per il nostro gioco sarebbe la fonte più comoda per debito pubblico, saldo di bilancio e previsioni, ma la licenza è la più restrittiva fra le grandi istituzioni.

## Licenza

I dati statistici IMF (WEO incluso) seguono «Special Terms for Statistical Data», non una licenza Creative Commons (fonti/imf-copyright-terms.md) [V]:

- gli utenti possono scaricare, estrarre, copiare, creare opere derivate, pubblicare, distribuire e usare i dati, con attribuzione all'IMF;
- per ogni potenziale riuso commerciale serve chiedere il permesso a copyright@imf.org;
- lo scaricamento massivo con strumenti automatici è vietato senza permesso esplicito;
- non è consentito usare contenuti o siti IMF per addestrare modelli linguistici.

Caveat: la pagina dei termini restituisce 403 ai client automatici e non ha uno snapshot Wayback; il testo è stato estratto con WebFetch come citazioni, non come pagina intera.

## Si può ridistribuire in un repo pubblico?

Per un repo gratuito e non commerciale la distribuzione con attribuzione è ammessa dai termini [V]. Se il gioco viene venduto, il riuso diventa commerciale e serve il permesso [INF]. Anche la raccolta automatica per costruire il dataset è vietata senza permesso [V].

In tensione con [[decisioni]]: là il WEO sta fra le fonti che «vietano la ridistribuzione indipendentemente dal prezzo». I termini speciali sui dati permettono invece di distribuire con attribuzione e chiedono il permesso solo per il riuso commerciale; richiamano però anche i termini generali (scaricamento non sistematico, uso personale e non commerciale) (fonti/imf-copyright-terms.md) [V]. La fonte non scioglie il punto. Il gioco è gratuito ([[decisioni]]); la raccomandazione sotto (WDI, OCSE, Eurostat al posto del WEO) non cambia.

## Per il nostro gioco

- Raccomandazione: non mettere dati WEO nel repo. Gli stessi aggregati (debito, saldo, PIL) esistono in [[world-bank-wdi]], [[oecd]] ed [[eurostat]] con licenze più chiare.
- Il WEO resta utile come controllo manuale degli ordini di grandezza durante la calibrazione [INF].

## Fonti

- **URL** https://www.imf.org/en/about/copyright-and-terms · [V]
  **perché conta qui** decide se il WEO può stare in un repo pubblico
  **cosa se ne prende** distribuzione con attribuzione sì, riuso commerciale solo con permesso, niente download massivo automatico (fonti/imf-copyright-terms.md)

## Collegamenti

- [[world-bank-wdi]], [[oecd]], [[eurostat]]: alternative aperte.
- [[finanza-pubblica]]: le grandezze che il WEO coprirebbe.
