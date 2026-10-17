---
tipo: dataset
fonti: [fonti/itanes-2022-dataverse-termini.md, fonti/itanes-2022-questionario-pre.md, fonti/itanes-2022-questionario-post.md]
aggiornato: 2026-10-17
---
# ITANES (Italian National Election Studies)

Lo studio elettorale nazionale italiano: un'indagine prima e una dopo ogni elezione politica, con l'autocollocazione sinistra-destra, la collocazione dei partiti, il voto attuale e quello precedente, e una condizione professionale che distingue il dipendente pubblico da quello privato. È l'unica fonte trovata che ha tutte e tre le dimensioni dei nostri gruppi (regione, condizione, reddito) insieme alla posizione politica. I microdati però non si possono ridistribuire.

## Contenuto (edizione 2022)

- Due rilevazioni con disegno panel, realizzate da SWG: pre-elettorale a campioni giornalieri dal 5 al 24 settembre 2022, N = 6250; post-elettorale dal 12 al 15 ottobre, N = 4696, con il questionario del Modulo 6 CSES (fonti/itanes-2022-dataverse-termini.md:72, :80, :81) [V].
- Campione dalla community online di SWG, stratificato per età, genere e area geografica sulla popolazione maggiorenne (fonti/itanes-2022-dataverse-termini.md:74) [V]. Un campione online non è probabilistico: le stime per gruppi piccoli vanno trattate con cautela [INF].
- Autocollocazione su una scala 0-10, con le risposte «non mi collocherei» e «non saprei» (fonti/itanes-2022-questionario-pre.md:515) [V]; nel post-elettorale anche la collocazione dei partiti sulla stessa scala (fonti/itanes-2022-questionario-post.md:683, :702) [V].
- Condizione: lavoro sì/no, dipendente o autonomo, settore pubblico o privato per i dipendenti, poi pensionato, studente, disoccupato per chi non lavora (fonti/itanes-2022-questionario-pre.md:830, :849, :901-904) [V]. Le sei condizioni dei gruppi si ricavano direttamente.
- Reddito: nel pre solo percezione («il reddito del suo nucleo consente di vivere…», fonti/itanes-2022-questionario-pre.md:88); nel post il reddito annuo familiare in 11 classi, da meno di 9.990 € a oltre 100.000 € (fonti/itanes-2022-questionario-post.md:1013) [V].
- Voto precedente: il voto alla Camera del 2018, nel pre e nel post (fonti/itanes-2022-questionario-pre.md:662; fonti/itanes-2022-questionario-post.md:573) [V].
- Regione di residenza fra le variabili di campionamento (fonti/itanes-2022-questionario-post.md:112) [V].

## Licenza

- Il campo «termini d'uso» dichiara CC0 «con i seguenti termini aggiuntivi o modificati» (fonti/itanes-2022-dataverse-termini.md:14) [V]. I termini aggiuntivi prevalgono:
  - dati liberi solo per scopi senza fini di lucro (:18);
  - i dati non vanno condivisi con altri né messi su siti esterni senza permesso esplicito; se modificati, citare «Adapted from ITANES 2022 (Vezzoni et al. 2023), release number X» (:20);
  - si possono pubblicare solo in forma riassunta, come negli articoli scientifici; vietato pubblicare casi singoli (:24);
  - i dati non si trasferiscono a terzi fuori dal progetto di ricerca (:26) [V].
- File scaricabili dal Dataverse dell'Università di Milano, nessuno ad accesso ristretto (fonti/itanes-2022-dataverse-termini.md:84) [V].

## Ridistribuibile in un repo pubblico?

- Microdati: **no**, vietato esplicitamente (fonti/itanes-2022-dataverse-termini.md:20, :26) [V].
- Aggregati derivati (istogrammi per gruppo, coefficienti stimati): rientrano probabilmente nella «forma riassunta» ammessa (:24), ma 2.520 quote per 360 gruppi sono più vicine a una tabella di dati che a una sintesi da articolo [INF]. Prima di metterli nella cartella dati conviene chiedere il permesso agli autori, oppure tenere nel repo solo i parametri stimati.

## Per il nostro gioco

- Fonte principale per stimare offline β del logit spaziale, la fedeltà al voto precedente e le differenze di posizione fra condizioni e fasce di reddito. Dettaglio del metodo in [[dati-per-tarare-i-gruppi]].
- Da evitare: copiare nel repo le distribuzioni per gruppo così come escono dai microdati senza permesso.
- Aperto: chiedere a ITANES il permesso per gli istogrammi aggregati; verificare se l'edizione 2027 sarà disponibile in tempo.

## Collegamenti

- [[cses]]: il post-elettorale 2022 è la componente italiana del Modulo 6 CSES.
- [[ess]]: alternativa con licenza CC BY-NC-SA, campione probabilistico ma più piccolo per l'Italia.
- [[voto-spaziale]]: il modello di cui ITANES dà i dati per stimare β.
- [[elettorato-e-gruppi-di-voto]]: i 360 gruppi da tarare.

## Riferimenti

- **URL** https://doi.org/10.13130/RD_UNIMI/JV77WR · [V]
  **perché conta qui** unico studio italiano con posizione, voto, voto precedente, regione, condizione con pubblico/privato e reddito
  **cosa se ne prende** le variabili per stimare istogrammi, β e fedeltà; i termini che vietano la ridistribuzione dei microdati (fonti/itanes-2022-dataverse-termini.md)
- **URL** https://dataverse.unimi.it/api/access/datafile/119545 · [V]
  **perché conta qui** questionario pre-elettorale 2022: formulazione delle domande
  **cosa se ne prende** scala 0-10, condizione con settore pubblico, voto 2018 (fonti/itanes-2022-questionario-pre.md)
- **URL** https://dataverse.unimi.it/api/access/datafile/119546 · [V]
  **perché conta qui** questionario post-elettorale 2022 (CSES Modulo 6)
  **cosa se ne prende** reddito familiare in 11 classi, collocazione dei partiti, regione (fonti/itanes-2022-questionario-post.md)
