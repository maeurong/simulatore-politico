Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Per ciascuna delle 30 disposizioni del Catalogo atti e riforme (16), elencate in `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:42-53`: qual è il valore in vigore nell'ottobre 2026, quale legge reale l'ha fissato (numero, data, articolo) e qual era il valore precedente, che serve all'abrogazione? Il gradino segnato con * nel catalogo corrisponde davvero al diritto vigente, o va corretto? Dove una disposizione è cambiata più volte di recente, quale legge abrogherebbe un referendum, e l'abrogazione riporta davvero al valore precedente?

Serve la tabella minima per il file di dati del turno zero, non una storia normativa.

## Answer

Ricerca del 17/10/2026, tabella e fonti in `docs/ricerca/sintesi/disposizioni-al-turno-zero.md`.

- **Valore vigente, legge d'origine, valore precedente** per ogni disposizione: tabella a `:16-49`, ogni riga con la sua fonte (Gazzetta Ufficiale, Normattiva, INPS, DPCM).
- **Le disposizioni sono 32, non 30**: la tabella del catalogo (`.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md:44-53`) ne elenca 32.
- **Gradini di partenza** (`:51-60`): 28 reggono. Da correggere:
  - intercettazioni: ampie* → limitate (L. 47/2025);
  - concessioni autostradali: private* → a gara* (L. 193/2024, art. 3);
  - credito d'imposta per gli investimenti: dal 2026 è un iperammortamento al 180% (L. 199/2025, c. 427), circa 40% del costo [INF];
  - nucleare: vietato fino al 14/10/2026, poi delega in vigore (L. 169/2026);
  - cuneo fiscale: 7* regge, ma il valore precedente è 0, non 3.
- **Abrogazione** (`:62-80`): per Corte cost. 13/2012 il referendum non fa rivivere le norme abrogate. Fa riespandere solo la regola generale quando cade una deroga (tutele crescenti → art. 18). Una novella testuale lascia il comma senza cifra (forfettario, contante, detrazione, accise), una norma a termine non torna (cuneo, Reddito di cittadinanza). L'abrogazione riporta davvero al valore precedente solo per età pensionabile e privato accreditato.
- [NON TROVATO]: accise effettive a ottobre 2026, aliquote del credito 4.0 nel 2025, alcuni valori precedenti (età prima del 2019, obbligo scolastico, prescrizione, export di armi).

Effetti sulla mappa: le scelte di prodotto che la ricerca apre (regola dell'abrogazione, nucleare, iperammortamento, gradini facoltativi) vanno nel ticket nuovo «Abrogazione e gradini di partenza» (24). Le correzioni fattuali entrano nel file di dati del turno zero.
