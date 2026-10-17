# Knowledge base del progetto: schema

Wiki locale mantenuta da agenti, secondo lo schema LLM Wiki di Karpathy: le fonti grezze si compilano **una volta** in pagine collegate, e la conoscenza si accumula invece di essere riscoperta a ogni domanda. Gli umani leggono la wiki (anche da Obsidian); gli agenti la scrivono e la tengono coerente.

## Tre strati

1. **`fonti/`: grezzo, immutabile.** Una fonte = `fonti/<slug>.md` (estratta con `defuddle`, frontmatter originale) + `fonti/<slug>.provenance.json` (`derived_from.path` = URL, `sha256`, `tool`, `extracted`). Mai modificata dopo la scrittura: se la pagina cambia, nuova fonte con nuovo slug.
2. **Wiki: pagine compilate**, in cartelle per tipo:
   - `giochi/`: un gioco (o una serie) per pagina.
   - `sistemi/`: un sistema di gioco trasversale ai titoli (elettorato, militare, commercio, logistica…).
   - `modelli/`: modelli accademici e formali utilizzabili nella simulazione.
   - `dati/`: dataset per calibrare, con la licenza.
   - `tecnica/`: stack, architettura della simulazione, prestazioni, distribuzione.
   - `estetica/`: direzione artistica, data-viz, riferimenti visivi leggeri.
   - `legale/`: IP, licenze, soggetti reali.
   - `sintesi/`: confronti e risposte a domande, archiviati come pagine.
3. **Questo file: lo schema.** Si cambia solo su decisione esplicita, non durante un ingest.

## Pagina wiki

Nome file `kebab-case.md`. Struttura:

```markdown
---
tipo: gioco | sistema | modello | dataset | tecnica | estetica | legale | sintesi
fonti: [fonti/slug-a.md, fonti/slug-b.md]
aggiornato: YYYY-MM-DD
---
# Titolo

Sintesi in 2-4 righe: cos'è e perché conta per il nostro gioco.

## <sezioni libere>
Ogni affermazione fattuale chiude con la sua fonte: `(fonti/slug.md)` oppure l'URL se non catturata.

## Per il nostro gioco
Cosa prendiamo, cosa evitiamo, cosa resta aperto.

## Collegamenti
- [[altra-pagina]]: perché è collegata.
```

I collegamenti sono `[[nome-file-senza-estensione]]`. Un link a una pagina che non esiste ancora è lecito: segna una pagina da scrivere.

## Operazioni

- **Ingest.** Cattura la fonte in `fonti/`, poi aggiorna **ogni** pagina che tocca (di solito 3-10), non solo quella nuova. Aggiorna `index.md` e aggiungi una voce a `log.md`.
- **Query.** Si risponde leggendo prima `index.md`, poi le pagine. Una risposta che vale la pena tenere diventa una pagina in `sintesi/`.
- **Lint.** Cerca contraddizioni fra pagine, affermazioni senza fonte, pagine orfane (nessun link entrante), link a pagine mai scritte, pagine superate da fonti più nuove. Registra l'esito in `log.md`.

## `index.md` e `log.md`

- `index.md`: catalogo per cartella, una riga per pagina, `- [[pagina]]: sintesi in una riga`.
- `log.md`: solo in aggiunta, voci in ordine cronologico, intestazione fissa `## [YYYY-MM-DD] ingest|query|lint | titolo` (così `grep "^## \[" log.md` le elenca).

## Ingest in parallelo

Quando più agenti lavorano insieme, ognuno scrive **solo** le pagine wiki assegnate nel suo brief e le proprie fonti in `fonti/`. Prima di creare una fonte controlla che lo slug sia libero. `index.md` e `log.md` li aggiorna chi coordina, a partire dalle righe che ogni agente restituisce. Per una pagina di un altro agente: link `[[...]]` e nota nel resoconto, senza modificarla.
