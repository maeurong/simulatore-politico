---
tipo: modello
fonti: [fonti/worldbank-hci-methodology-wps8593.md, fonti/wikipedia-human-capital-index.md, fonti/oecd-pisa-data.md, fonti/democracy4-modding.md]
aggiornato: 2026-10-16
---
# Istruzione e capitale umano

Il capitale umano è lo stock di competenze della forza lavoro. Cresce solo quando entrano coorti istruite meglio ed escono quelle vecchie, quindi è il caso più netto di politica che rende in decenni. Il riferimento operativo è lo Human Capital Index (HCI) della Banca Mondiale, che traduce scuola e salute in produttività futura.

## Il modello (HCI, Kraay 2018)
- HCI = sopravvivenza × scuola × salute. Misura la produttività da adulto di un bambino nato oggi rispetto a un riferimento di istruzione completa e salute piena; va da 0 a 1 (fonti/worldbank-hci-methodology-wps8593.md).
- Scuola: anni attesi di scuola entro i 18 anni, corretti per la qualità con i punteggi armonizzati dei test, divisi per un riferimento di 625 (fonti/worldbank-hci-methodology-wps8593.md).
- Conversione in produttività: un anno di scuola in più alza il reddito di circa l'8%; 9 anni contro 14 valgono il 40% di produttività in meno (fonti/worldbank-hci-methodology-wps8593.md).
- Salute: tasso di sopravvivenza adulta 15-60 anni e assenza di arresto della crescita (stunting) sotto i 5 anni (fonti/wikipedia-human-capital-index.md).
- Dall'indice al PIL: con HCI = x, il PIL pro capite di lungo periodo con istruzione e salute piene è circa 1/x volte quello dello status quo; se servono 50 anni, un raddoppio vale 1,4 punti di crescita l'anno (fonti/worldbank-hci-methodology-wps8593.md).
- Valori: dallo 0,3 dei paesi più poveri allo 0,9 dei migliori (fonti/worldbank-hci-methodology-wps8593.md); Singapore 0,88 in testa (fonti/wikipedia-human-capital-index.md).

## Esecuzione
[INF] Stock di capitale umano per coorte di età (si appoggia sulla piramide di [[demografia-e-migrazioni]]). Ogni anno la coorte che compie 18 anni entra con l'HCI corrente; la produttività aggregata è la media pesata sulle coorti in età da lavoro. Il ritardo nasce dalla struttura: nessun timer da tarare.

## Variabili
- **Mosse**: spesa per studente, anni di obbligo, rapporto studenti/insegnanti, salari degli insegnanti, programmi per la prima infanzia, università e borse di studio.
- **Subite**: qualità misurata (test), abbandono, fuga di cervelli (migrazione), demografia delle classi.

## Ritardi
- Il paper lo dice in apertura: i benefici «richiedono tempo per materializzarsi e non sono molto visibili agli elettori», ed è per questo che i politici investono poco (fonti/worldbank-hci-methodology-wps8593.md). È la tensione da mettere al centro del gioco.
- Test di qualità: 1-3 anni dopo una riforma. Ingresso nel lavoro: 10-20 anni. Rinnovo della forza lavoro: circa 50 anni (fonti/worldbank-hci-methodology-wps8593.md). [INF] per le prime due cifre.
- Democracy 4, per confronto, dà ai sussidi all'istruzione degli adulti un'inerzia di 4 turni (fonti/democracy4-modding.md): troppo veloce per una riforma della scuola.

## Dati per calibrare
- [[pisa-oecd]]: risultati PISA 2025 e microdati 2000-2025 (fonti/oecd-pisa-data.md).
- HCI della Banca Mondiale, per paese.

## Per il nostro gioco
- Prendere: la catena HCI (spesa → anni e qualità → produttività) e lo stock per coorte.
- Evitare: effetti dell'istruzione sul PIL nello stesso mandato.
- Aperto: come rendere visibile al giocatore un beneficio che arriva dopo la sua uscita. Un indicatore anticipatore (punteggi dei test) e un'eredità valutata a fine partita (vedi [[ritardi-e-inerzia-delle-politiche]]).

## Collegamenti
- [[demografia-e-migrazioni]]: coorti, fuga di cervelli.
- [[sanita-e-spesa-sanitaria]]: la componente salute dell'HCI.
- [[ritardi-e-inerzia-delle-politiche]]: il caso tipo del ritardo lungo.
- [[servizi-pubblici]]: copertura e bilancio dei servizi nei gestionali.
