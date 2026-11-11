Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
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
11. Clausola dell'energia: fino a che anno vale (`partita.rs:4220-4227`, `crates/sim-core/src/economia/regole_ue.rs:31`). È nazionale, non per regione: corretto il 10/11 su segnalazione della sessione del piano di core.

Fuori da questo ticket, perché i dati ci sono: gli acquisti d'informazione (`Vista.acquisti`) aspettano solo una schermata, da [Ossatura e navigazione](07-ossatura-e-navigazione.md); la serie dei tre indicatori a partita in corso (`Partita::serie`) aspetta un passacarte, da [Contratto del guscio](12-contratto-del-guscio.md).

Le voci 8 e 9 sono voci d'esito: se la 1 trova un piano, entrano lì.

## Answer

Deciso con Mario il 10/11/2026. Nessuna delle 11 voci esce dalla v1 e nessuna si calcola nella UI.

- **Piano di core «esito e Vista completa»**, fuori dalla mappa, con il suo giro di brainstorming, spec e piano. Parte subito in una sessione a parte; la mappa continua in parallelo.
  - *PR della logica nuova*: esito del turno con voci nell'ordine della chiusura, causa e criticità (voce 1); registro per turno (2); fermate di «avanti fino a novità» (3); avvisi della campagna tagliata e del bilancio ricondotto al tetto del MES, come voci dell'esito (8, 9).
  - *PR delle esposizioni*, stato che il core ha già e la Vista non mostra: dati regionali della mappa (4), PDE e conto di controllo (5), minaccia di scissione (7), stato del MES e della ristrutturazione (10), clausola dell'energia (11).
- **Piano strutturale riveduto** (voce 6, storia 51): piano di core piccolo e suo, dopo quello dell'esito. È una regola nuova e non blocca la UI: la spec del piano 11 gli lascia il posto nel passaggio del bilancio con un rimando.

Forme fissate per le esposizioni:

- Disoccupazione e reddito per regione: **esatti, con un turno di ritardo**, come le grandezze regionali già nella Vista (`crates/sim-core/src/vista.rs:492-498`). Il dato esatto arriva nel turno in cui arriva la revisione nazionale, quindi non anticipa nulla.
- Gruppi della scheda della regione: per ciascuno dei 18 gruppi la **quota di elettori**, e al clic i tre elettori campione (`crates/sim-core/src/partita.rs:2543-2572`). Nessun consenso per gruppo: il sondaggio resta ai margini.
- Cantieri finiti: il segno resta **fino al voto successivo**. Oggi il core dimentica il cantiere nel turno in cui finisce (`crates/sim-core/src/ministri/agenda.rs:347`).
- Voto e seggi per regione: esatti (storia 120), quelli dell'ultimo voto.

Lasciato al brainstorming del piano di core:

- dove vive il registro (stato della partita o snapshot riletti dal guscio): scelta tecnica;
- cosa rende critica una voce e come si scelgono le sette in evidenza: decisione di Mario, da prendere lì;
- quattro voci minori dell'inventario del 01/11, non ricontrollate su `8952a06`, da verificare e prendere se mancano ancora: forbice del sondaggio per regione, combinazioni spente della legge elettorale (storia 72), posti liberi in calendario, costo in soddisfazione di Ferma e Dirotta (storia 28).

Conseguenze sulla mappa: [Testi del Pungolo](09-testi-del-pungolo.md) aspetta la spec del piano di core, che porta l'elenco dei tipi di voce. [Fette](10-fette.md) ordina le fette dell'esito, del registro, della mappa e dell'opposizione dietro quel piano. Termine nuovo nel glossario: «Fermata».

### Correzioni del 10/11/2026, dopo la chiusura

Dalla ricerca della sessione del piano di core «esito e Vista completa»:

- **La clausola dell'energia è nazionale.** Verificato qui: `clausola_energia_fino_a` è un campo di `regole_ue` (`crates/sim-core/src/economia/regole_ue.rs:31`), scritto a `crates/sim-core/src/partita.rs:4224-4225`. L'elenco diceva «per regione»: era un errore di lettura.
- **Seggi per regione**: valgono quelli del Senato (storia 138). La sessione del core riporta che i seggi della Camera per regione e lista non esistono, perché il riparto è nazionale; non verificato qui.
- Tre esposizioni chiedono stato nuovo e non solo un campo nella Vista, secondo la stessa sessione: disoccupazione regionale del turno prima, voti e seggi del Senato per regione, cantieri finiti.

