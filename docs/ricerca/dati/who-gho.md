---
tipo: dataset
fonti: [fonti/who-gho-home.md, fonti/who-copyright.md, fonti/who-data-policy.md, fonti/who-ghed.md]
aggiornato: 2026-10-16
---
# WHO Global Health Observatory (GHO)

Portale statistico dell'OMS: oltre 1000 indicatori sanitari per 194 Stati membri (fonti/who-gho-home.md). Per noi: aspettativa di vita, mortalità per causa, copertura vaccinale, fattori di rischio, capacità di risposta alle emergenze.

## Contenuto
- Temi: copertura sanitaria universale, emergenze sanitarie (capacità di rilevare e rispondere), salute e benessere (mortalità, malattie, tabacco e sostanze) (fonti/who-gho-home.md).
- API disponibili; i dati COVID-19 non passano dalle API GHO ma da HDX (fonti/who-gho-home.md).
- Spesa sanitaria: la fonte vera è il Global Health Expenditure Database (195 paesi dal 2000, dati 2023 nell'aggiornamento 2025), che alimenta anche GHO e World Bank WDI (fonti/who-ghed.md).

## Licenza e ridistribuibilità
- Le pubblicazioni OMS sono **CC BY-NC-SA 3.0 IGO**: riuso libero ma **non commerciale**, con attribuzione e stessa licenza per gli adattamenti (fonti/who-copyright.md).
- L'uso commerciale richiede un permesso, e serve un permesso anche per inserire materiali OMS in «prodotti e servizi di database elettronici» (fonti/who-copyright.md).
- La policy sui dati prevede che l'OMS renda disponibili i dati degli Stati membri «per uso non commerciale e senza scopo di lucro, a fini di sanità pubblica» (fonti/who-data-policy.md).
- [NON TROVATO] in fonte primaria: una licenza dichiarata sulla pagina GHO per i dati. Un record di metadati della Banca Mondiale (Data360, fonte secondaria non catturata) indica CC BY-NC-SA 3.0 IGO.
- **Conclusione [INF]**: si può usare per calibrare i parametri. Non si ridistribuisce dentro il gioco se il gioco sarà venduto, o se la licenza del repo permette usi commerciali, senza un permesso OMS. Ricavare costanti di modello (es. una curva età-mortalità) è meno rischioso che spedire la tabella; la valutazione spetta al filone legale. Deciso: gioco gratuito e dati in una cartella separata con la propria licenza ([[decisioni]]), quindi il vincolo NC è rispettato; restano il permesso per i «prodotti e servizi di database elettronici» e lo share-alike sugli adattamenti [INF].

## Per il nostro gioco
- Usare per: mortalità per età (con [[un-population-prospects]]), letalità di riferimento delle epidemie, spesa sanitaria per paese.
- Preferire, dove esiste, la stessa serie da una fonte con licenza aperta (WPP per la mortalità).

## Collegamenti
- [[sanita-e-spesa-sanitaria]], [[epidemie-sir-seir]], [[our-world-in-data]] (che ridistribuisce dati OMS sotto la licenza OMS).
- [[modding-e-dati-esterni]] (filone tecnico): come i dati entrano nel gioco.
