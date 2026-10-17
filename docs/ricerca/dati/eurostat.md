---
tipo: dataset
fonti: [fonti/eurostat-copyright-notice.md, fonti/eurostat-licence-policy-pdf.md, fonti/eurostat-figaro-metadata.md]
aggiornato: 2026-10-16
---
# Eurostat

Ufficio statistico dell'UE: conti nazionali e di settore, finanza pubblica (debito e disavanzo secondo le regole UE), lavoro, prezzi, commercio, tavole input-output FIGARO (fonti/eurostat-figaro-metadata.md) [V]. Per il nostro gioco è la fonte armonizzata per i paesi europei, Italia compresa.

## Licenza

- Il riuso di dati statistici, metadati e pubblicazioni è autorizzato per scopi commerciali e non commerciali, citando la fonte; la politica attua la Decisione della Commissione del 12 dicembre 2011. I contenuti editoriali del sito sono CC BY 4.0 (fonti/eurostat-copyright-notice.md, cattura di un altro agente) [V].
- I principi generali possono avere condizioni specifiche nei singoli avvisi di copyright (fonti/eurostat-copyright-notice.md) [V].
- Policy di riuso (voce Statistics Explained «Copyright/licence policy», PDF del 08/07/2021): riuso autorizzato con citazione, anche per diffusione commerciale. Non si ridiffondono a scopo commerciale: dati di fonti diverse da Eurostat, dati di paesi fuori da UE, EFTA e candidati, alcuni dati di commercio EFTA e austriaci, loghi, documenti con avviso restrittivo. Chi adatta o modifica i dati deve dichiararlo in modo visibile (fonti/eurostat-licence-policy-pdf.md) [V].
- La vecchia cattura `eurostat-licence-policy.md` contiene la pagina principale di Statistics Explained, perché la pagina HTML reindirizza lì: non va citata.

## Si può ridistribuire in un repo pubblico?

Sì, citando la fonte. Le tavole FIGARO sono prodotte da Eurostat e dal JRC (fonti/eurostat-figaro-metadata.md): rientrano nella stessa politica di riuso [INF, nessuna clausola specifica trovata nei metadati FIGARO]. Con il gioco gratuito ([[decisioni]]) le eccezioni commerciali non scattano; resta l'obbligo di dichiarare le modifiche, per esempio l'aggregazione a 10-15 settori (fonti/eurostat-licence-policy-pdf.md) [V].

## Per il nostro gioco

- Prendiamo i saldi settoriali dei conti finanziari (per lo scheletro di [[macro-stock-flow-consistent]]), debito e disavanzo per [[finanza-pubblica]], FIGARO per [[input-output-leontief]].

## Fonti

- **URL** https://ec.europa.eu/eurostat/help/copyright-notice · [V]
  **perché conta qui** regola di riuso dei dati Eurostat
  **cosa se ne prende** riuso anche commerciale con citazione della fonte (fonti/eurostat-copyright-notice.md, cattura di un altro agente)
- **URL** https://ec.europa.eu/eurostat/statistics-explained/SEPDF/cache/2059.pdf · [V]
  **perché conta qui** è la policy di riuso vera; la pagina HTML reindirizza alla home di Statistics Explained
  **cosa se ne prende** le eccezioni al riuso commerciale e l'obbligo di dichiarare le modifiche (fonti/eurostat-licence-policy-pdf.md)
- **URL** https://ec.europa.eu/eurostat/cache/metadata/en/naio_10_fcp_esms.htm · [V]
  **perché conta qui** chi produce FIGARO e cosa copre
  **cosa se ne prende** Eurostat-JRC, 50 regioni × 64 settori, 2010 a T-2 (fonti/eurostat-figaro-metadata.md)

## Collegamenti

- [[istat]]: il dato nazionale italiano.
- [[input-output-tables]]
