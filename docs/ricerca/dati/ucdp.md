---
tipo: dataset
fonti: [fonti/ucdp-downloads.md, fonti/ucdp-definitions.md]
aggiornato: 2026-10-16
---
# UCDP: Uppsala Conflict Data Program

Il maggiore fornitore di dati sulla violenza organizzata e la raccolta sulle guerre civili attiva da più tempo: oltre 40 anni. La sua definizione di conflitto armato è lo standard internazionale. È l'**unico** dei dataset su conflitti e difesa di questo filone con una licenza aperta e ridistribuibile: CC BY 4.0.

## Licenza e ridistribuibilità

- Tutti i dataset sono gratuiti e sotto **CC BY 4.0**: si possono usare e ridistribuire citando le pubblicazioni indicate per ciascun dataset (fonti/ucdp-downloads.md).
- Accesso anche via API JSON per uso programmatico; versioni storiche archiviate (fonti/ucdp-downloads.md).
- Dalla versione 17.1 gli ID di conflitti, attori e diadi sono cambiati e non sono compatibili con le versioni precedenti; ci sono tabelle di conversione (fonti/ucdp-downloads.md).

## Definizioni operative

- **Conflitto armato (state-based):** incompatibilità su governo e/o territorio, uso della forza fra due parti di cui almeno una è un governo, con almeno **25 morti in battaglia** in un anno solare (fonti/ucdp-definitions.md).
- **Non-state:** nessuna delle parti è un governo. **One-sided:** un gruppo organizzato è attivo se uccide deliberatamente almeno 25 civili in un anno (fonti/ucdp-definitions.md).
- **Intensità:** *minor* 25-999 morti in battaglia per anno, *war* 1.000 o più (fonti/ucdp-definitions.md).
- I morti in battaglia si contano con moderazione: ogni morte va verificata, i dati non affidabili si scartano (fonti/ucdp-definitions.md). Le stime sono quindi prudenti [INF].
- Un conflitto termina quando scende sotto la soglia dei 25 (fonti/ucdp-definitions.md).

## Dataset di contorno utili

Accordi di pace 1975-2021 (v22.2) con i testi completi; sostegno esterno (ESD); violenza elettorale letale (DECO, 1989-2017); conflitti in città (CACE, estensione dell'evento georeferenziato GED); attacchi ai peacekeeper (PAR) (fonti/ucdp-downloads.md).

## Per il nostro gioco

- **Prendiamo** le soglie UCDP come stati di un conflitto di gioco: pace → conflitto minore (≥25/anno) → guerra (≥1.000/anno). La terminologia è quella ufficiale, e il giocatore la ritrova nelle notizie vere [INF].
- **Prendiamo** i tre tipi di violenza (fra stati, fra gruppi non statali, contro i civili) come tre generatori di eventi distinti, utili in [[controinsurrezione-e-stabilita]] [INF].
- **Prendiamo** il dataset accordi di pace e quello sul sostegno esterno per tarare quanto durano i conflitti e quanto spesso intervengono attori esterni [INF].
- **Si può ridistribuire** un estratto UCDP dentro il gioco o il repo, con attribuzione nei crediti e la citazione richiesta. È la base dati di default per tutto ciò che riguarda i conflitti dal 1989 in poi [INF dalla licenza].
- **Aperto:** quale versione congelare (GED o UCDP/PRIO annuale) e dove tenere l'attribuzione nell'interfaccia. Da coordinare con [[militare-e-guerra]] e `legale/`.

## Collegamenti
- [[acled]]: dati sugli eventi più fini ma non ridistribuibili; UCDP li sostituisce.
- [[correlates-of-war]]: serie più lunga (dal 1816) ma senza licenza dichiarata.
- [[controinsurrezione-e-stabilita]]: soglie e tipi di violenza.
- [[deterrenza-ed-escalation]]: dal conflitto minore alla guerra.

## Riferimenti
- **URL** https://ucdp.uu.se/downloads/ · [V]
  **perché conta qui** dichiara la licenza CC BY 4.0 per tutti i dataset
  **cosa se ne prende** il permesso di ridistribuire con citazione; l'API JSON per l'estrazione
- **URL** https://www.uu.se/en/department/peace-and-conflict-research/research/ucdp/ucdp-definitions · [V]
  **perché conta qui** le definizioni standard di conflitto, intensità, morti in battaglia
  **cosa se ne prende** le soglie 25 / 1.000 e i tre tipi di violenza come stati ed eventi di gioco
