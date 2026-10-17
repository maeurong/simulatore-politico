Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Con quali dati aperti, e con quale ultima annata coerente, si fissa lo stato del paese al turno zero per il motore del Modello economico della v1 (07)?
1. Grandezze macro e di finanza pubblica: PIL e componenti, debito, disavanzo, spesa per interessi e vita media del debito, spread, inflazione, disoccupazione nazionale e regionale; ultimo trimestre o anno disponibile a ottobre 2026 e fonte.
2. Budget dei 10 dicasteri (Dicasteri della v1, 03): la legge di bilancio 2026 (o il bilancio dello Stato per missioni e programmi) si può ricondurre ai 10 dicasteri? Quale quota è spesa di funzionamento e quale margine discrezionale? Il fondo di riserva reale quanto vale?
3. Spesa obbligatoria: pensioni (spesa, numero, quota contributiva e retributiva), sussidi di disoccupazione, reddito minimo (assegno di inclusione), con la piramide per coorti.
4. Tavola input-output a 10 settori e mix regionale: annata più recente pubblicata da ISTAT o Eurostat.
5. Le date sono compatibili fra loro? Se le annate non coincidono, quale trimestre conviene come turno zero?

Licenza di ogni fonte. Punto di partenza: `docs/ricerca/modelli/economia-italiana-parametri.md`, che dà i coefficienti ma non i livelli di partenza.

## Answer

Ricerca del 17/10/2026, dettaglio, fonti e licenze in `docs/ricerca/sintesi/stato-del-paese-al-turno-zero.md`.

1. **Macro e finanza pubblica** (`:12-59`): tutti i livelli arrivano almeno al 2026T2. Debito al 136,7% del PIL a fine 2025, dopo la revisione ISTAT del 22/9; deficit -3,1% nel 2025; interessi al 3,85% del PIL con vita media di 7,8 anni; spread 79 pb nel T2 (89 a settembre); IPCA 4,2% a settembre; disoccupazione 5,6%, per regione solo l'annuale 2025.
2. **Budget dei dicasteri** (`:61-110`): la legge di bilancio 2026 si riconduce ai 10 dicasteri, ma non va usata come budget, perché il bilancio dello Stato è quasi tutto trasferimenti (la Salute vale 2,6 miliardi contro i 141,5 della spesa sanitaria delle AP). Proposta [INF]: COFOG 2024 di tutte le AP, con il funzionamento pari a salari più acquisti (65% del totale, dal 19% di Sviluppo economico al 93% della Sanità). Il margine discrezionale reale, misurato sulla manovra, è l'1,6%. Fondo di riserva e Fondo emergenze nazionali: stanziamento [NON TROVATO].
3. **Spesa obbligatoria** (`:112-130`): pensioni 342,5 miliardi nel 2025 (16,39 milioni di pensionati), NASpI 17,93 miliardi, ADI 835 mila nuclei; piramide ISTAT al 1/1/2026. La quota retributiva/contributiva dello stock è [NON TROVATO].
4. **Input-output** (`:132-141`): FIGARO 2024, tavola nazionale 2022, mix regionale A*10 2023.
5. **Date** (`:143-167`): i livelli arrivano al 2026T2, strutture e quote possono essere più vecchie e si scalano. La ricerca raccomandava il turno 1 nel 2026T3. **Riconciliazione, assunzione del coordinatore:** resta il calendario di Partenza della partita (20), col turno zero a ottobre-dicembre 2026 perché la 180 vale dal 24/10. Lo stato base è al 2026T2, portato a inizio ottobre con gli indicatori mensili di luglio-settembre trattati come dati osservati. Lo stato iniziale non è un equilibrio: cade nello shock energetico 2026, con inflazione sopra l'obiettivo.

Licenze: ISTAT CC BY 4.0, Eurostat riuso con citazione; PDF della Banca d'Italia solo per uso personale, quindi per il repo il debito trimestrale Eurostat; RGS riuso non commerciale; MEF e PDF INPS [NON TROVATO].

Effetti sulla mappa:
- Editor dello scenario (21) si sblocca per la parte dati: le grandezze modificabili e le loro fonti sono nella tabella della pagina;
- Costo del governare (nebbia): un margine discrezionale reale dell'1,6% del budget rende i 40 atti piccoli in proporzione; va deciso nel bilanciamento se il gioco lo allarga;
- per il turno zero vale il 136,7%: il 137,1% di `docs/ricerca/modelli/regole-fiscali-ue.md:47` è la cifra della Commissione del 3 giugno, precedente alla revisione ISTAT, e lì resta corretto come citazione.
