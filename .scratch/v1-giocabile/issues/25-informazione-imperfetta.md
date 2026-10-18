Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa vede il giocatore dello stato vero del paese, e con quale errore? La v1 promette «informazione imperfetta» (`docs/ricerca/sintesi/decisioni.md:51`) e la ricerca propone tre livelli, stato vero → statistica ufficiale rumorosa e in ritardo → rapporto di parte, con l'informazione che si compra (`docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:24-29`). Ma il giro del turno ha escluso una moneta delle azioni (09), quindi «comprare» informazione non ha ancora un prezzo.

Da decidere:
- quali grandezze arrivano esatte, quali rumorose, quali in ritardo e rivedute dopo (economia, indicatori di servizio, spread, seggi);
- chi le riferisce: statistica neutra, ministri con il loro interesse, giornale;
- precisione e fonte dell'anteprima di una misura: direzione, ritardo e gruppi già visibili, cifre no (09);
- sondaggi: il pubblico del trimestre a tre strati d'errore (`docs/ricerca/modelli/sondaggi-ed-errore.md:55`), sondaggi privati, silenzio elettorale;
- se il giornale dell'esito è manipolabile dal governo (09);
- precisione della tabella dei seggi guadagnati e persi di una legge elettorale (13);
- quanto si legge dell'intesa della maggioranza avversaria oltre le tre fasce (12);
- quanto si vede del rischio di un evento puntuale prima che accada (15).

Ingressi:
- Giro del turno (09): un sondaggio pubblico gratuito a trimestre, nessuna moneta delle azioni, esito come prima pagina.
- Autonomia dei ministri (04): capacità 1-5, obiettivo e posizione propri.
- Elettorato (06): elettori campione ricavati dal sondaggio, non dallo stato vero.
- Partiti IA (18): votano e trattano sui seggi attesi dal sondaggio pubblico.

## Answer

Grilling con Mario del 18/10/2026, quattro giri. Raccomandazioni accettate tranne due scelte di Mario: l'informazione si compra con una **cassa del partito** (risorsa nuova, invece di nessun acquisto) e il centro studi vende **anche previsioni**. Termini nuovi in `CONTEXT.md`: sondaggio, prima stima, anteprima, cassa del partito, previsione.

**Due livelli, nessun bias.** Il giocatore vede la statistica ufficiale, mai lo stato vero né un rapporto di parte: il terzo livello della ricerca, il ministro che piega i numeri (`docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:24`), resta fuori.
- Economia (PIL, disoccupazione, inflazione, disavanzo): **prima stima** rumorosa, con errore tarato sulle revisioni reali (circa 1 punto annuo sul PIL, `:11`), riveduta il turno dopo; la revisione va nel registro.
- Indicatori di servizio: esatti, con un turno di ritardo.
- Spread, seggi, voti d'aula, conti della cassa: esatti.
- Il giornale dell'esito riporta la statistica ufficiale e non è manipolabile.

**Sondaggio pubblico del trimestre**: 3-4 istituti inventati, ciascuno col suo effetto istituto, più il bias comune estratto una volta per legislatura e rivelato solo dal voto, più l'errore di campionamento (`docs/ricerca/modelli/sondaggi-ed-errore.md:55`); incroci per gruppo. Fare la media degli istituti aiuta, ma non toglie il bias comune. Il silenzio elettorale non conta, perché il sondaggio esce in apertura, prima del voto in chiusura.

**Attori.** Al governo, soddisfazione dei ministri e intesa degli alleati in cifre, con le soglie 40/25 a schermo. All'opposizione l'intesa della maggioranza avversaria resta in tre fasce (12), nient'altro.

**Anteprima di una misura.** Costo in denaro esatto (è contabilità). Effetti sulle grandezze in fasce (poco, medio, molto), stimati dal ministro proponente con un errore che cala con la sua capacità: un ministro capace non è solo più efficace, ma anche più affidabile nel prevedere. Direzione, ritardo, gruppi colpiti, linee e forbice restano come in Giro del turno (09). La tabella dei seggi di una legge elettorale (13) è la proiezione sul sondaggio pubblico: esatta come calcolo, ma porta l'errore del sondaggio, e i partiti IA votano su quella.

**Rischio degli eventi.** Di ogni situazione si vede quanto la grandezza è vicina alla soglia di accensione; la pericolosità naturale per regione (19) è una mappa nota; la probabilità per turno di emergenze e scandali resta nascosta.

**Cassa del partito**, risorsa nuova del partito, non del governo.
- Entrate: 2 per mille proporzionale ai voti dell'ultima elezione, donazioni proporzionali all'intenzione di voto vera, dote alla fondazione (al turno zero il partito non ha voti). Cifre tarate sul 2 per mille reale.
- Non va sotto zero.
- In informazione compra, al massimo una volta per tipo a turno:
  - **sondaggio privato**: campione più grande, meno errore di campionamento e d'istituto, stesso bias comune; incroci per gruppo e regione;
  - **test di una misura**: la reazione stimata dei gruppi, che rende più precisa la parte «gruppi colpiti» dell'anteprima;
  - **centro studi**: una stima del trimestre chiuso più precisa della prima stima, più una **previsione** a 4 turni di PIL, disoccupazione, inflazione e debito, con intervallo che si allarga con l'orizzonte e tarato sugli errori reali delle previsioni. Si calcola sul percorso in corso: misure in vigore e in arrivo, nessuna misura nuova, shock futuri a zero. L'effetto di una misura proposta resta all'anteprima a fasce.
- I partiti IA hanno la stessa cassa e le stesse entrate, ma non comprano informazione: decidono sul sondaggio pubblico (18). La spendono solo in campagna.

Effetti sulla mappa:
- Campagna elettorale (26): la cassa esiste, e la campagna è il suo secondo uso, per il giocatore e per i partiti IA; il sondaggio privato resta disponibile nel turno del voto.
- Costo del governare (29): prezzi dei tre acquisti, taglia della dote, peso di 2 per mille e donazioni, numero di istituti, errore della prima stima e della previsione.
- Salvataggi (28): la cassa entra nello stato della partita.
