Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Per ogni sistema escluso dalla v1, la spec deve riservargli un punto di aggancio (un campo nei dati, una famiglia di eventi, una fase del turno, una forma di misura) oppure niente, lasciando il lavoro allo sforzo che lo porterà dentro? Un aggancio costa adesso e vale solo se quel sistema arriva davvero.

Sistemi raccolti dai ticket risolti:
- esteri attivi, guerra, svolta autoritaria, attentati (Out of scope);
- giunte regionali come attori; Esteri e Difesa in due dicasteri; appoggio esterno e governo di minoranza (05);
- salienza mobile degli interessi; attrazione fra gruppi a confidenza limitata (06);
- banche e crisi bancaria; dazi ed embarghi sul commercio; immigrazione come offerta di lavoro su u* (07);
- rinvio presidenziale; Corte costituzionale; forma di governo e bicameralismo come norme costituzionali; sede deliberante e navetta (08);
- il giocatore come alleato minore in un governo altrui; congressi e sfide alla leadership del suo partito (12);
- fusioni fra partiti (18);
- collegi uninominali (e con loro l'impostazione Rosatellum), ballottaggio, preferenze (13);
- regista adattivo degli eventi; epidemia con modello SIR (15);
- campagna per i referendum; salienza mobile dei temi come canale della campagna (26, oltre al 06);
- uscita dall'euro: il motore separato in blocchi (07);
- migrazioni interne fra regioni, con la ripartizione dei seggi che segue la popolazione; indicatori di servizio regionali, che arrivano con le giunte (27);
- migrazioni dei salvataggi fra versioni dello schema; rami della stessa partita (28);
- aspettative estese ai servizi; salienza mobile delle aspettative fra gruppi; luna di miele per un leader nuovo del partito, con i congressi (29).

Gli altri ticket ancora aperti possono aggiungere voci: per questo arriva per ultimo.

## Answer

Grilling con Mario del 21/10/2026, tre giri più la conferma del quadro finale; una scelta contro la raccomandazione: i ruoli speciali dei dicasteri stanno nei dati, non nel codice. Termine nuovo in `CONTEXT.md`: ruolo del dicastero.

**Premessa: un aggancio non si compra in anticipo.** Un salvataggio di schema diverso si rifiuta, senza migrazioni (`28-salvataggi.md:33`). Il modding è solo dati (`11-stack-e-linguaggio-del-core.md:19`) e lo scenario su misura è un file di differenze (`21-editor-dello-scenario.md:41`). Un campo aggiunto dopo rompe quindi le stesse partite che romperebbe aggiunto adesso, e un campo riservato vuoto non protegge nulla.

**Criterio: solo cuciture.** Nessun campo vuoto, nessuna fase vuota, nessuna famiglia di eventi senza eventi. La spec chiede una cucitura solo quando tre condizioni valgono insieme: la v1 costruisce comunque quella parte, scriverla in forma generale costa quasi zero, scriverla fissa costringerebbe lo sforzo successivo a riscriverla invece di aggiungere. Scartati il «niente per nessuno», che fa pagare per intero anche le cuciture gratuite, e gli agganci riservati, che costano adesso per sistemi che forse non arriveranno.

**Le quattro cuciture.**
1. *Motore economico in blocchi separati*: già deciso in Modello economico della v1 (07). Ci si appoggiano uscita dall'euro, banche e crisi bancaria, dazi ed embarghi.
2. *Versione dello schema nel salvataggio*: già decisa in Salvataggi (28). Ci si appoggiano le migrazioni fra versioni e i rami della stessa partita.
3. *Seggi per regione calcolati dalla popolazione censita*, non scritti in una tabella. La v1 ne ha già bisogno: la norma costituzionale sul numero dei parlamentari (`08-iter-parlamentare.md:28`) cambia i seggi da ripartire, e premio e tetto sono già quote dei seggi (`13-legge-elettorale-come-misura.md:26`). Le migrazioni interne, quando arriveranno, muoveranno solo la popolazione.
4. *Dicasteri nei dati, ruoli compresi.* L'elenco dei dicasteri (nome, budget, atti) sta nei file di dati insieme ai **ruoli** che oggi il codice darebbe per nome: la colpa di disoccupazione e trasferimenti (Lavoro e Welfare), di imposte e impiego pubblico (Economia e Finanze), dei servizi (il dicastero che li eroga) da Tabella degli interessi (17), e la regione bersaglio degli atti (Infrastrutture e Trasporti, Sviluppo economico) da Mappa delle 20 regioni (27). Ogni ruolo va a un solo dicastero; un dicastero può averne più d'uno. Un file che lascia un ruolo senza dicastero, o lo assegna a due, si rifiuta al caricamento con il nome del ruolo, come il limite di taratura dello scenario (`21-editor-dello-scenario.md:38`). Reddito primario, prezzi e fondo di riserva restano del Presidente del Consiglio, che non è un dicastero, e non stanno nei dati. Ci si appoggia Esteri e Difesa in due dicasteri, che diventa un cambio di dati.

**Niente per tutto il resto.** Ogni altra voce si aggiunge senza riscrivere quello che la v1 costruisce: un blocco del motore, un passo in un ordine di chiusura già fisso (atti → aula → motore, 09), una famiglia di eventi, un caso nuovo accanto a quelli esistenti. Il caso più vicino a una cucitura è il giocatore come alleato minore, terzo ruolo accanto a governo e opposizione sullo stesso turno a tre fasi: resta a niente. Anche la salienza mobile resta a niente: i pesi degli interessi sono parametri per riga (`17-tabella-degli-interessi.md:26`) e diventarli stato è una modifica piccola. All'elenco si aggiungono due voci dei ticket chiusi, entrambe a niente: il rapporto di parte, cioè il ministro che piega i numeri (`25-informazione-imperfetta.md:32`), e lo scripting in sandbox (`11-stack-e-linguaggio-del-core.md:19`).

**Nella spec.** Una sezione «Fuori dalla v1»: una riga per sistema, con il ticket d'origine e la cucitura su cui si appoggia oppure «niente». Le quattro cuciture entrano anche nei requisiti della v1, perché sono vincoli su come si costruisce.

Effetti sulla mappa:
- nessun ticket nuovo; la mappa è risolta e il passo successivo è `/to-spec`;
- Out of scope: una riga che rimanda a questo ticket per l'elenco dei sistemi esclusi.
