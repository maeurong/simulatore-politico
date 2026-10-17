---
tipo: sistema
fonti: [fonti/vic3-wiki-market.md, fonti/vic3-wiki-pops.md, fonti/vic3-wiki-standard-of-living.md, fonti/vic3-wiki-goods.md, fonti/vic3-wiki-building.md, fonti/vic3-wiki-production-methods.md, fonti/vic3-dd09-mercati-nazionali.md, fonti/vic3-dd54-revisioni-commercio.md, fonti/eu5-wiki-market.md, fonti/eu5-wiki-population.md, fonti/eu5-wiki-economy.md, fonti/stellaris-dd366-4-0-pop-groups.md, fonti/stellaris-dd370-4-0-cambiamenti.md, fonti/steam-recensioni-realismo.md, fonti/steam-recensioni-grand-strategy.md]
aggiornato: 2026-10-16
---
# Mercati e pops: il modello economico di Victoria 3

Il modello di Victoria 3 ricostruito pezzo per pezzo dalle fonti, con le varianti di EU5 e la correzione di Stellaris sulle prestazioni. È il candidato più ovvio per il livello "micro" della nostra economia: chi produce, chi consuma, chi guadagna. Il modello ha un difetto strutturale (crea e distrugge denaro) che un'economia credibile non può ereditare.

## Le quattro entità

1. **Pop**: gruppo di persone con stessa cultura, religione, professione e luogo di lavoro; porta ricchezza e alfabetizzazione (fonti/vic3-wiki-pops.md).
2. **Edificio**: impiega pop secondo il metodo di produzione scelto, compra input e vende output. Il metodo di produzione decide beni in ingresso e uscita, professioni impiegate e chi riceve il profitto (fonti/vic3-wiki-production-methods.md).
3. **Mercato**: insieme di stati collegati alla capitale di mercato, dove si incontrano ordini d'acquisto e di vendita (fonti/vic3-wiki-market.md).
4. **Stato**: governa leggi, tasse, sussidi e il settore delle costruzioni.

## Il ciclo settimanale

- Gli edifici producono e vendono; consumo e produzione locali diventano ordini sul mercato, "come ordini su una borsa merci" (fonti/vic3-dd09-mercati-nazionali.md).
- Prezzo = base × [1 + 0,75 × clamp((BUY − SELL) / min(BUY, SELL), ±1)] (fonti/vic3-wiki-market.md).
- Prezzo locale = MAPI × prezzo di mercato + (1 − MAPI) × prezzo dello stato; accesso al mercato = infrastruttura / uso dell'infrastruttura (fonti/vic3-wiki-market.md).
- Produttività di un edificio = profitto per addetto all'anno prima dei salari; bilancio settimanale = (produttività − salario medio annuo) × addetti / 52 (fonti/vic3-wiki-building.md).
- Se l'edificio è in utile, dal 25% al 50% del profitto va nelle sue riserve e il resto ai proprietari come dividendi; i proprietari possono essere lo stato, i lavoratori o edifici di proprietà (aristocratici o finanziari) (fonti/vic3-wiki-building.md).
- Economie di scala: +1% di throughput per ogni livello dello stesso edificio nello stato, fino a +20% di base (fonti/vic3-wiki-building.md).
- I pop spendono il salario settimanale sul paniere che dà il tenore di vita più alto che possono permettersi; la ricchezza sale o scende con soglie al 102% e al 98% (fonti/vic3-wiki-standard-of-living.md).
- Il settore costruzioni ha 10 punti base; ogni lunedì i punti si assegnano alla coda; il costo di un edificio va da 100 a 800 (fonti/vic3-wiki-building.md).

## Disequilibri

- Eccesso d'offerta: buono per chi compra; i produttori non fanno profitto, non alzano i salari, licenziano; i licenziati diventano radicali (fonti/vic3-wiki-market.md).
- Carenza (acquisti ≥ 2× vendite): throughput −5% poi −1% al giorno fino a −50% o −75%; si propaga a valle sugli edifici che usano il bene (fonti/vic3-wiki-market.md).
- Nessuna scorta: Victoria 3 ha tolto lo stockpile di Victoria 2, in cui ogni unità veniva prodotta, venduta, conservata e consumata (fonti/vic3-dd09-mercati-nazionali.md).
- **Il mercato non conserva il valore.** Compratori e venditori pagano e incassano ai rispettivi ordini anche se non coincidono; la differenza è creata o distrutta dalla simulazione. Esempio del wiki: 100 ordini d'acquisto e 120 di vendita di legno a 17 creano 340 sterline dal nulla (fonti/vic3-wiki-market.md).

## Commercio fra mercati

- Una versione precedente: rotte commerciali che partono a livello 1 e crescono o calano ogni settimana secondo le condizioni di mercato; dazi calcolati sul prezzo base (fonti/vic3-dd54-revisioni-commercio.md).
- Versione documentata ad agosto 2026: mercato mondiale raggiungibile dai porti, hub per area, vantaggio commerciale a somma zero e bonus di monopolio fino al 20% (fonti/vic3-wiki-market.md). Il commercio è cambiato almeno due volte dal lancio [INF].

## Varianti negli altri titoli

- EU5: prezzo che converge verso un obiettivo, prezzo obiettivo = base × min((1 + R × s) / (R + s), massimo) con R = (1 + offerta) / (1 + domanda) e s = stabilità dei prezzi legata all'epoca (fonti/eu5-wiki-market.md). Dà inerzia ai prezzi.
- EU5: i pop crescono ogni mese di pop × crescita / 12, con crescita legata a prosperità, prezzo del cibo e terra libera (fonti/eu5-wiki-population.md). La fiscalità moltiplica la ricchezza per il controllo territoriale (fonti/eu5-wiki-economy.md).
- Stellaris 4.0: pop raggruppati per specie, strato ed etica; scala ×100; forza lavoro che riempie i lavori; fatto per tagliare calcoli mensili (fonti/stellaris-dd366-4-0-pop-groups.md, fonti/stellaris-dd370-4-0-cambiamenti.md).

## Cosa dicono i giocatori

- L'economia è la parte lodata di Victoria 3; i difetti citati sono IA economica debole e lentezza a fine partita (fonti/steam-recensioni-realismo.md).
- Su EU5, che porta lo stesso modello nel Trecento, un giocatore scrive "EU5 is designed more like Victoria 2/3, more of an actual simulation"; un altro "a nice simulator, but not a good game" (fonti/steam-recensioni-grand-strategy.md).

## Per il nostro gioco

- Prendiamo: la formula del prezzo con clamp e l'accesso al mercato da infrastruttura. Costano poco e generano carenze leggibili.
- Prendiamo: produttività per addetto e riparto fra riserve e dividendi; collega impresa, lavoro e reddito.
- Prendiamo: pop come gruppi aggregati (classe × regione × settore), mai individui; decine o centinaia, non milioni.
- Prendiamo da EU5: prezzi con inerzia.
- Evitiamo: creazione e distruzione di denaro nel mercato. Il nostro mercato deve chiudere i conti (ciò che un compratore paga lo incassa qualcuno, il resto è scorta o importazione), come chiede [[macro-stock-flow-consistent]].
- Aperto: se tenere un livello di mercati per beni (micro) sotto il nucleo macro aggregato di [[motore-economico-raccomandato]], o se il micro sia un costo che il gioco "leggero" non si può permettere. Raccomandazione: settori aggregati alla Leontief ([[input-output-leontief]]) al posto dei beni singoli [INF].

## Collegamenti

- [[victoria-3]], [[europa-universalis]], [[stellaris]].
- [[motore-economico-raccomandato]]: sintesi del filone economico.
- [[macro-stock-flow-consistent]]: contabilità che chiude.
- [[input-output-leontief]]: settori al posto dei beni.
- [[mercato-del-lavoro-e-disuguaglianza]]: salari e decili.
- [[cittadini-simulati]]: come altri titoli rappresentano la popolazione.
- [[produzione-industriale-e-risorse]]: lato offerta.

## Riferimenti

- **URL** https://vic3.paradoxwikis.com/Market · [V] (snapshot 2026-08-19)
  **perche' conta qui** formula del prezzo e prova della non conservazione del valore
  **cosa se ne prende** clamp ±75%; il difetto da non ereditare
- **URL** https://vic3.paradoxwikis.com/Building · [V] (snapshot 2026-08-24)
  **perche' conta qui** conto economico di un edificio
  **cosa se ne prende** produttività per addetto, riserve 25-50%, economie di scala +1%/livello
- **URL** https://forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-9-national-markets.1484917/ · [V]
  **perche' conta qui** scelta di design esplicita di togliere le scorte
  **cosa se ne prende** ordini come borsa merci
- **URL** https://eu5.paradoxwikis.com/Market · [V]
  **perche' conta qui** variante con inerzia dei prezzi
  **cosa se ne prende** prezzo obiettivo e convergenza
