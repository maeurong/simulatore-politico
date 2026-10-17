Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Con quali dati aperti, e con quali licenze, si dà a ciascuna delle 20 regioni un rischio per turno di emergenze naturali (Crisi ed eventi, 15)?
1. Pericolosità sismica per regione (INGV, classificazione sismica dei comuni, Protezione civile): una grandezza per regione, e la frequenza storica dei terremoti con danni o vittime.
2. Rischio idrogeologico (alluvioni e frane) per regione (ISPRA, rapporto sul dissesto idrogeologico): popolazione esposta e frequenza degli eventi.
3. Incendi e siccità per regione (EFFIS, ISPRA, Protezione civile): superficie bruciata annua, anni di siccità.
4. Costo storico delle emergenze (stati di emergenza deliberati dal Consiglio dei ministri, Fondo per le emergenze nazionali): ordine di grandezza per tarare gravità e fondo di riserva (03).
5. Esiste una stima di quanto la prevenzione (opere di difesa del suolo, adeguamento sismico) riduca i danni, abbastanza solida per un coefficiente?

Serve la forma minima per un rischio trimestrale per regione e per una gravità ridotta dalla prevenzione, non un modello di pericolosità.

## Answer

Ricerca del 17/10/2026 (researcher, AFK). Dettaglio, tabella per regione e licenze in `docs/ricerca/dati/pericolosita-naturale-regioni.md`, con 21 fonti in `docs/ricerca/fonti/`.

1. **Sisma.**
   - Grandezza per regione: quota dei comuni in zona 1 o 2 dal CSV DPC (maggio 2025), da 0% (Piemonte, Sardegna) a 100% (Calabria, Marche).
   - Frequenza: 45 episodi «regione-anno» con Mw ≥ 5,5 nel 1900-2020 (catalogo INGV CPTI15 v4.0), cioè 0,37 l'anno a scala nazionale. Si va dallo 0,0144 a trimestre dell'Abruzzo a zero per sei regioni. L'assegnazione degli episodi alle regioni è manuale e l'elenco è nella pagina.
2. **Alluvioni e frane.**
   - Esposti per regione dall'API IdroGEO di ISPRA: 6,8 milioni di abitanti in alluvione P2 e 1,28 milioni in frana P3-P4. Il peso idro di ogni regione è la media delle due quote nazionali: Emilia-Romagna 23%, Toscana 14%, Campania 12%.
   - La frequenza degli eventi non esiste in forma aperta, perché gli stati di emergenza sono solo PDF singoli: il numero nazionale di emergenze idro gravi all'anno resta una manopola, con valore iniziale 4.
3. **Incendi e siccità.**
   - Incendi: dall'API EFFIS, una media di 60.469 ha bruciati l'anno nel 2006-2025. La probabilità trimestrale di un incendio da almeno 500 ha va da 0,35 in Sicilia a zero, e l'87% di questi incendi cade nel terzo trimestre.
   - Siccità: dati solo per il 2022 (Sicilia −80,7%, Sardegna −73%, distretto del Po −66%). Una serie per regione non c'è.
4. **Costi.**
   - Circa 4 miliardi l'anno di danni da sismi e dissesto (Casa Italia 2017), circa 10 miliardi a episodio sismico con coda lunga.
   - 5,7 miliardi l'anno di danni da eventi climatici nel 2010-2024 (Eurostat `cli_iad_loss`).
   - Il Fondo per le emergenze nazionali è cresciuto di 100-400 milioni l'anno nel 2024-2026: copre la prima emergenza, non la ricostruzione. Questo regge l'opzione «decreto a debito» del ticket 15.
   - Non trovati: la dotazione consolidata del fondo e il costo degli incendi.
5. **Prevenzione.**
   - Sisma: c'è un appiglio normativo italiano, le classi di perdita attesa del DM 58/2017. Il salto dalla classe G alla B riduce la perdita attesa di circa l'85% (k = 0,85). È un modello di perdita convenzionale, non una misura dei danni evitati.
   - Alluvioni e incendi: esistono solo rapporti beneficio/costo statunitensi (NIBS 2019: 5-8:1 e 2-4:1). Lì k è una scelta di design.

**Licenze.** DPC, EFFIS e il sito ISPRA sono CC BY 4.0. Gli indicatori IdroGEO vanno trattati come CC BY-SA 4.0 per prudenza. Il catalogo CPTI15 è CC BY 4.0 ma «per scopi scientifici»: nel repo vanno solo i conteggi derivati.

Effetti sulla mappa:
- nebbia «Costo del governare e bilanciamento»:
  - il numero nazionale di emergenze idro all'anno (4 di partenza);
  - il minimo di rischio sismico per le regioni senza episodi;
  - k di prevenzione per idro e incendi;
  - i tre livelli di gravità;
  - la taglia del fondo di riserva (0,2-0,4 miliardi l'anno proposti);
- da decidere: la proposta del researcher di trattare la siccità come situazione a soglia invece che come emergenza puntuale. Nel ticket 15 la siccità sta fra le emergenze naturali.
