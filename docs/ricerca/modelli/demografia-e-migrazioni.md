---
tipo: modello
fonti: [fonti/un-wpp-2024-summary-pdf.md, fonti/un-wpp-2024-dataset.md, fonti/un-wpp-2024-methodology.md, fonti/wikipedia-population-projection.md, fonti/eurostat-population-ageing.md, fonti/meadows-leverage-points.md]
aggiornato: 2026-10-16
---
# Demografia e migrazioni: piramide, fertilità, invecchiamento, pensioni

La demografia è il sottosistema più prevedibile e più lento: chi avrà 65 anni nel 2050 è già nato. Il metodo standard è la proiezione per coorti e componenti. Un anno di gioco fa avanzare di un anno ogni coorte, toglie i morti, aggiunge nati e migranti netti.

## Il modello: coorti e componenti
- Le Nazioni Unite producono stime dal 1950 a oggi e proiezioni al 2100 per 237 paesi e aree; la revisione 2024 è la ventottesima (fonti/un-wpp-2024-dataset.md, fonti/un-wpp-2024-methodology.md).
- Componenti: si parte dalla popolazione stimata all'ultimo censimento e si proiettano nascite, morti e migrazioni (fonti/wikipedia-population-projection.md). [INF] La versione per coorti applica fertilità per età della madre, mortalità per età e sesso, migrazione netta per età; con 101 classi di età × 2 sessi il costo per paese per anno è trascurabile.
- Inerzia: la crescita passata, incorporata in una struttura per età giovane, continua a spingere la popolazione anche quando la fertilità scende; vale anche al contrario, per il declino (fonti/un-wpp-2024-summary-pdf.md).

## Numeri chiave (WPP 2024)
- Popolazione mondiale: 8,2 miliardi nel 2024, picco di circa 10,3 miliardi a metà degli anni 2080, 10,2 miliardi nel 2100; probabilità dell'80% che il picco cada in questo secolo (fonti/un-wpp-2024-summary-pdf.md).
- Fertilità mondiale: 2,25 figli per donna, contro 3,31 nel 1990; più di metà dei paesi è sotto la sostituzione (2,1); quasi un quinto, Italia compresa, è sotto 1,4 (fonti/un-wpp-2024-summary-pdf.md).
- Nei 24 paesi a fertilità ultra-bassa con popolazione già in calo, la probabilità di tornare a 2,1 entro 30 anni è dello 0,1% (fonti/un-wpp-2024-summary-pdf.md).
- Le politiche pro-natalità perdono efficacia nel tempo: entro la fine degli anni 2030 metà delle donne nei paesi già al picco sarà oltre l'età fertile (fonti/un-wpp-2024-summary-pdf.md).
- UE, 1/1/2025: età mediana 44,9 anni; 22,0% con 65 anni o più; indice di dipendenza degli anziani al 34,5%, cioè poco più di 3 persone in età da lavoro per ogni anziano (fonti/eurostat-population-ageing.md).

## Pensioni
[INF] Sistema a ripartizione: contributi = occupati × salario medio × aliquota; prestazioni = pensionati × pensione media. Il saldo dipende dall'indice di dipendenza, che il giocatore non può muovere in fretta. Leve: età di pensionamento (effetto in pochi anni), aliquota, indicizzazione, immigrazione in età da lavoro. Le formule del bilancio pubblico stanno nelle pagine del filone economia ([[finanza-pubblica]]).

## Variabili
- **Mosse**: assegni familiari e servizi per l'infanzia (fertilità, effetto piccolo e lento), quote e politiche migratorie, età pensionabile, cittadinanza.
- **Subite**: mortalità (sanità, epidemie), emigrazione dei giovani, struttura per età ereditata.

## Ritardi
- Una nascita in più oggi è un lavoratore fra 20 anni. Meadows cita proprio «il ritardo fra la nascita di un bambino e il momento in cui quel bambino può avere figli» come esempio di ritardo di sistema (fonti/meadows-leverage-points.md).

## Dati per calibrare
- [[un-population-prospects]]: piramide, fertilità, mortalità e migrazione per paese, 1950-2100.
- Eurostat per l'Europa (licenza già trattata dal filone legale e dati).

## Per il nostro gioco
- Prendere: la piramide come struttura dati portante, condivisa con istruzione, sanità, pensioni e criminalità.
- Evitare: fertilità che risponde in fretta agli incentivi (i dati WPP dicono il contrario).
- Aperto: le migrazioni sono l'unica leva demografica veloce; il loro costo politico va dal sistema elettorale.

## Collegamenti
- [[istruzione-e-capitale-umano]], [[sanita-e-spesa-sanitaria]], [[criminalita-e-giustizia]]: tutti leggono la piramide.
- [[ritardi-e-inerzia-delle-politiche]].
- [[cittadini-simulati]]: la popolazione aggregata nei gestionali.
