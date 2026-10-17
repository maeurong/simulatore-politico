Type: grilling
Status: resolved
Blocked by: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Quali 8-10 dicasteri accorpati ha la v1, e cosa governa ciascuno (quali grandezze del modello muove, quali atti del ministro può fare)? Quali ministeri reali confluiscono in quale dicastero?

## Answer

Grilling con Mario del 16/10/2026, due round, tutte le raccomandazioni accettate. Termini nuovi in `CONTEXT.md`: spesa di funzionamento, margine discrezionale, indicatore di servizio, legge di bilancio, fondo di riserva.

**Dieci dicasteri** (fra parentesi i ministeri reali che vi confluiscono, assetto 2022):

| Dicastero | Ministeri reali | Indicatore di servizio | Grandezze spostate dagli atti |
|---|---|---|---|
| Economia e Finanze | MEF | efficienza della riscossione | evasione → gettito; credibilità → spread |
| Sviluppo economico | Imprese e Made in Italy, Agricoltura, Turismo | sostegno alle imprese | investimenti privati, export |
| Lavoro e Welfare | Lavoro e Politiche sociali | copertura del welfare | povertà, spesa pensionistica, occupazione (politiche attive) |
| Sanità | Salute | qualità del servizio sanitario (liste d'attesa) | speranza di vita (lenta) |
| Istruzione e Ricerca | Istruzione e Merito, Università e Ricerca, Cultura | qualità della scuola | abbandono scolastico; capitale umano → produttività (molto lenta) |
| Infrastrutture e Trasporti | Infrastrutture e Trasporti | qualità dei trasporti | stock di infrastrutture (grandi opere) |
| Ambiente ed Energia | Ambiente e Sicurezza energetica | sicurezza energetica | prezzo dell'energia, emissioni |
| Interno | Interno | sicurezza (criminalità) | immigrazione irregolare, integrazione |
| Giustizia | Giustizia | durata dei processi | sovraffollamento carceri, corruzione |
| Esteri e Difesa | Esteri, Difesa | prontezza della difesa | spesa per la difesa sul PIL rispetto all'obiettivo NATO; credito con l'UE |

Esteri e Difesa è un solo dicastero perché nella v1 gli esteri sono solo vincoli e shock; si divide quando arriva la guerra (punto di aggancio). Le grandezze macro (PIL, occupazione, inflazione) restano al ticket 07, che riceve questa tabella come ingresso.

**Budget.**
- Aliquote e budget per dicastero si fissano una volta l'anno con la **legge di bilancio** (turno ottobre-dicembre), proposta dal Presidente del Consiglio con il ministro dell'Economia e votata dal parlamento. Durante l'anno i fondi si spostano solo per decreto. Il ministro dell'Economia ha atti propri (riscossione e lotta all'evasione, gestione del debito, dismissioni); la sua capacità pesa sulla credibilità, cioè sullo spread.
- Il budget di un dicastero ha due quote: la **spesa di funzionamento**, che tiene in piedi l'indicatore di servizio (un taglio lo fa peggiorare con ritardo, senza bisogno di alcun atto), e il **margine discrezionale**, che il ministro spende in autonomia sugli atti.
- **4 atti del ministro per dicastero**, 40 in tutto nella v1.
- **Fondo di riserva**: piccolo, fissato nella legge di bilancio, fuori dai dicasteri, speso solo dal Presidente del Consiglio nelle emergenze (Protezione civile); se finisce, servono un decreto e altro debito.

**Regioni.** Sono passive: unità della mappa con i loro indicatori, nessuna giunta. La spesa nazionale vi arriva per popolazione. Solo gli atti di Infrastrutture e Trasporti e di Sviluppo economico hanno una Regione bersaglio.

**Coalizione.** Nella trattativa si spartiscono solo i 10 dicasteri: niente vicepresidenze, ministri senza portafoglio o sottosegretari nella v1.

Effetti sulla mappa: si sblocca il ticket 04. Al fog si aggiunge il catalogo dei 40 atti; le giunte regionali vanno fra i punti di aggancio, il fondo di riserva in crisi ed eventi.
