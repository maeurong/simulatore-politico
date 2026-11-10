Type: grilling
Status: open
Blocked by: -
Assignee: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Chi produce i dati che le schermate chiedono e che nessun piano porta ([vista-per-schermata](../../../docs/ricerca/sintesi/vista-per-schermata.md))? Esito del turno con causa, criticità e voci in evidenza, più il registro per turno (storie 14-16); il predicato delle fermate di «avanti fino a novità» (storia 89); disoccupazione, reddito, voto e seggi per regione, gruppi della scheda, cantieri finiti (storie 137-140); PDE e conto di controllo nella Vista (storia 50); piano strutturale riveduto (storia 51); minaccia di scissione persistente (storia 108). Per ciascuno: un piano di core nuovo prima o accanto al 9, dentro il 9 o il 10, oppure fuori dalla v1. La UI non calcola regole: nessuno di questi va inventato nel piano 11.

### Elenco al 10/11/2026

Riletto su main a `8952a06`, con i piani 9 e 10 fusi ([Inventario](01-inventario-vista-per-schermata.md), aggiornamento del 10/11). Le alternative «dentro il 9 o il 10» non ci sono più: per ogni voce resta un piano di core nuovo prima del piano 11, oppure fuori dalla v1.

Già nell'elenco e ancora senza fonte:

1. Esito del turno con causa, criticità e voci in evidenza (storie 14-15).
2. Registro per turno (storia 16). Il riavvolgimento c'è (`riavvolgi`), manca ciò da cui scegliere il turno.
3. Predicato delle fermate di «avanti fino a novità» (storia 89).
4. Disoccupazione, reddito, voto e seggi per regione, gruppi della scheda, cantieri finiti (storie 137-140).
5. PDE e conto di controllo nella Vista (storia 50).
6. Piano strutturale riveduto (storia 51).
7. Minaccia di scissione persistente (storia 108).

Debiti lasciati dal piano 9:

8. Avviso della campagna tagliata quando la cassa non basta dopo gli acquisti.
9. Avviso del bilancio ricondotto al tetto del MES (`crates/sim-core/src/partita.rs:1646-1647`).
10. Stato del MES e della ristrutturazione: in corso o no, fino a quando (`crates/sim-core/src/economia/mod.rs:106-111`). La scelta passa già dall'evento «Aste scoperte».
11. Clausola dell'energia per regione (`partita.rs:4220-4227`).

Fuori da questo ticket, perché i dati ci sono: gli acquisti d'informazione (`Vista.acquisti`) aspettano solo una schermata, da [Ossatura e navigazione](07-ossatura-e-navigazione.md); la serie dei tre indicatori a partita in corso (`Partita::serie`) aspetta un passacarte, da [Contratto del guscio](12-contratto-del-guscio.md).

Le voci 8 e 9 sono voci d'esito: se la 1 trova un piano, entrano lì.

## Answer
