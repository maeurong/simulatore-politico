---
tipo: gioco
fonti: [fonti/wr-wiki-cittadini.md, fonti/steam-api-snapshot-gestionali.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Workers & Resources: Soviet Republic

City builder e gestionale logistico di 3Division (1.0 nel giugno 2024): si pianifica una repubblica socialista dove ogni merce, ogni lavoratore e ogni veicolo vanno spostati davvero. È l'estremo "tutto simulato" del genere. Per noi è la misura di cosa costa, in giocabilità e in prestazioni, l'agente individuale che deve camminare fino al lavoro.

## Scheda

| Campo | Valore |
|---|---|
| Uscita 1.0 | 20/06/2024, solo Windows (fonti/steam-api-snapshot-gestionali.md) |
| Recensioni Steam | 27.643, 91% positive (fonti/steam-api-snapshot-gestionali.md) |
| Requisiti minimi | Pentium G3250 dual core, 6 GB RAM, GTX 650 2 GB, 9 GB disco (fonti/steam-api-snapshot-gestionali.md) |

## Scala e loop

Una repubblica su una mappa con più città, industria pesante, import/export in rubli e dollari. Il loop è logistico: estrarre, trasformare, trasportare e distribuire, e portare i lavoratori al lavoro (fonti/wr-wiki-cittadini.md) [INF: la valuta doppia si ricava dagli inviti di immigrati pagati in rubli o in dollari].

## Come modella la cosa

**Ogni cittadino è simulato** con nome, età, istruzione e preferenze; ha una routine di lavoro (o studio), tempo libero e viaggio (fonti/wr-wiki-cittadini.md).

**Demografia.** La popolazione cresce per inviti a pagamento (10 immigrati sovietici in rubli; 5 esperti laureati a 2,5 volte il prezzo; 10 immigrati non istruiti dal terzo mondo in dollari), per nascite e per trasferimento fra edifici. Si invecchia di 12 anni per ogni anno di gioco, si muore verso i 70-80 anni e si fugge quando la felicità tocca un minimo critico (fonti/wr-wiki-cittadini.md).

**Mobilità.** Il cittadino preferisce camminare: con percorsi al 100% di velocità il raggio massimo è 480 m, che scende a 240 m al 50%. Oltre quel raggio prova auto o trasporto pubblico; dopo 4 ore su un singolo mezzo "si teletrasporta a casa"; oltre 5 ore di viaggio al giorno perde felicità (fonti/wr-wiki-cittadini.md). I lavoratori non hanno un posto fisso: cercano un lavoro libero nel raggio a piedi, altrimenti vanno alla fermata (fonti/wr-wiki-cittadini.md).

**Statistiche personali.** La produttività va da un minimo del 30% a oltre il 100%. La felicità, che pesa più di tutto, scende sotto i bisogni non soddisfatti; a 0% il cittadino fugge subito. La salute dipende da inquinamento, acqua, temperatura e cibo e regola nascite e morti (fonti/wr-wiki-cittadini.md). Il fabbisogno alimentare è di 1,5 kg di cibo per periodo libero (fonti/wr-wiki-cittadini.md).

**Lealtà.** È una statistica politica: alza la produttività e attutisce i bisogni non soddisfatti. Si alza con monumenti vicini agli edifici più frequentati, fino al 50%, e oltre con la propaganda radio. Per conoscerla il giocatore deve mandare la polizia segreta a spiare; sotto il 30% riduce la felicità (fonti/wr-wiki-cittadini.md).

**Domande.** Le attività del tempo libero (cultura, sport, religione…) si scelgono a caso, pesate per preferenza; ogni domanda soddisfatta alza felicità e preferenza, ogni domanda mancata le abbassa (fonti/wr-wiki-cittadini.md).

## Realismo: cosa dicono i giocatori

Il pregio citato più spesso: "c'è sempre una ragione simulata per tutto", a differenza di Skylines "dove la simulazione aggiusta per bilanciamento o prestazioni". Il prezzo è un micromanagement di ogni spostamento: la stessa recensione dice che senza controllo i cittadini "si comportano in modi folli". Si criticano anche il raggio a piedi di 400 m "da disabili", i viaggi senza ritorno e l'interfaccia. Fra i difetti torna la prestazione a fine partita (fonti/steam-recensioni-gestionali.md).

## Prestazioni

Requisiti minimi bassi, ma i giocatori segnalano rallentamenti con le repubbliche grandi (fonti/steam-recensioni-gestionali.md). Una misura sistematica [NON TROVATO].

## Per il nostro gioco

- **Prendiamo:** la lealtà come statistica distinta dalla felicità, misurabile solo a costo (polizia segreta = informazione imperfetta); la causalità leggibile ("c'è una ragione per tutto"), da ottenere con aggregati spiegabili invece che con agenti; le note rosse per edificio come diagnosi.
- **Evitiamo:** il realismo che obbliga a micro-gestire gli spostamenti; i teletrasporti di emergenza che rompono la verosimiglianza proprio dove l'agente avrebbe dovuto darla.
- **Aperto:** la popolazione per inviti a pagamento come leva di immigrazione.

## Collegamenti

- [[cittadini-simulati]]: il caso estremo dell'agente individuale.
- [[logistica-e-trasporti]]: tutto si sposta fisicamente.
- [[consiglieri-e-informazione-imperfetta]]: la lealtà si conosce solo spiando.
- [[feedback-visivo-dello-stato]]: note rosse per edificio, notifiche a tre livelli.
