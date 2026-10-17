---
tipo: modello
fonti: [fonti/wikipedia-compartmental-models.md, fonti/steam-plague-inc-the-cure.md, fonti/cdc-blog-plague-inc-2013.md, fonti/democracy4-mod-situations.md, fonti/who-gho-home.md]
aggiornato: 2026-10-16
---
# Epidemie: modelli SIR e SEIR

I modelli a compartimenti dividono la popolazione in stati (Suscettibili, Esposti, Infetti, Rimossi) e spostano persone fra gli stati con tassi fissi. Sono il motore minimo di un'epidemia credibile: poche equazioni, un solo numero che il giocatore capisce (R), una curva che sale e scende da sola.

## Il modello
- SIR: tre compartimenti, due parametri. β è il tasso di contagio, γ quello di guarigione; il numero di riproduzione di base è R0 = β/γ (fonti/wikipedia-compartmental-models.md). Se R0 · S/N < 1 l'epidemia non decolla, a prescindere dalla taglia iniziale dei suscettibili (fonti/wikipedia-compartmental-models.md).
- SEIR: aggiunge il compartimento E per il periodo di latenza, in cui l'individuo è infetto ma non ancora contagioso (fonti/wikipedia-compartmental-models.md). Serve per avere il ritardo fra il primo caso e l'esplosione dei numeri.
- Varianti: SIS (nessuna immunità), SIRS (immunità temporanea), con nascite e morti, MSEIR con immunità passiva (fonti/wikipedia-compartmental-models.md).
- R0 misura la pressione di controllo; R_t, quello in tempo reale, è il numero che le autorità seguono durante l'emergenza (fonti/wikipedia-compartmental-models.md).

## Esecuzione
[INF] Un SEIR per regione, integrato con Eulero a passo giornaliero e accoppiato fra regioni da una matrice di mobilità, costa poche operazioni per regione al giorno: trascurabile anche con centinaia di regioni. Il giocatore non vede S, E, I, R: vede casi rilevati, che sono una frazione ritardata di I (vedi [[consiglieri-e-informazione-imperfetta]]).

## Variabili
- **Mosse dal giocatore** (agiscono su β, sulla rilevazione o sul sistema sanitario): lockdown, tracciamento dei contatti, chiusura delle frontiere, igiene, preparazione degli ospedali, sussidi di cassa integrazione. Sono le leve elencate da Plague Inc: The Cure (fonti/steam-plague-inc-the-cure.md).
- **Subite**: caratteristiche del patogeno (β, latenza, letalità), densità e mobilità, età della popolazione, saturazione ospedaliera.
- **Il punto politico**: «la gente non rispetta misure di quarantena mal disegnate»; il consenso si costruisce con politiche economiche di sostegno (fonti/steam-plague-inc-the-cure.md). La compliance è una variabile di stato, non una costante.

## Ritardi
- Latenza (E→I) e durata dell'infezione (1/γ) sono i ritardi biologici (fonti/wikipedia-compartmental-models.md).
- [INF] Fra misura e casi osservati passano latenza + tempo di test e notifica: il giocatore vede l'effetto di un lockdown dopo una o due settimane di gioco, e deve decidere prima.

## Dati per calibrare
- [[who-gho]]: mortalità, copertura vaccinale, capacità IHR per le emergenze sanitarie (fonti/who-gho-home.md).
- [[our-world-in-data]]: serie storiche COVID e malattie infettive, con la licenza del fornitore originale.

## Giochi che lo fanno bene
- [[plague-inc]]: il giocatore è il patogeno; l'autore ha costruito i modelli con ricerca online, senza formazione medica, puntando a «sembrare realistico» (fonti/cdc-blog-plague-inc-2013.md).
- [[democracy-4]]: le crisi sono «situazioni» con soglie distinte di avvio e di arresto (fonti/democracy4-mod-situations.md).

## Per il nostro gioco
- Prendere: SEIR per regione, R_t esposto come cruscotto, compliance come stock che si consuma con la durata delle restrizioni e si ricarica con i sostegni economici.
- Evitare: il gioco-patogeno di Plague Inc; da noi il giocatore è il governo, come in The Cure.
- Aperto: quante regioni (dipende dalla mappa, vedi [[ritardi-e-inerzia-delle-politiche]] per il passo temporale).

## Collegamenti
- [[sanita-e-spesa-sanitaria]]: la capacità ospedaliera fissa la letalità effettiva.
- [[crisi-ed-eventi]]: l'epidemia come evento emergente, non scriptato.
- [[consiglieri-e-informazione-imperfetta]]: casi rilevati contro casi veri.
- [[media-informazione-e-propaganda]]: fiducia e disinformazione spostano la compliance.
