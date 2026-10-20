Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come si salva e si riprende una partita? Il core è puro e riproducibile sullo stesso binario con seme fisso (11), lo scenario su misura è un file di differenze dalla base e il seme appartiene alla partita (21), e la partita finisce con l'eredità (12).

Da decidere:
- salvataggio automatico a ogni turno, slot manuali, o entrambi;
- se si può tornare a un turno passato, e quindi rigiocare un voto perso, oppure la partita va solo avanti;
- cosa contiene il salvataggio: stato completo, oppure scenario + seme + elenco delle scelte da rigiocare;
- cosa succede a un salvataggio quando cambia la versione del gioco o dei dati (modding solo dati, 11);
- dove finiscono l'eredità e le partite concluse.

Lo stato della partita comprende anche la cassa del partito e il bias comune dei sondaggi estratto per la legislatura (25): un salvataggio non deve rivelarlo. Ne fanno parte anche le promesse in sospeso di ogni partito, da verificare al voto successivo, e le spinte di posizione della campagna che l'ancora non ha ancora riassorbito (26).

## Answer

Grilling con Mario del 20/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Restano fermi: snapshot e non replay, riproducibilità sullo stesso binario (11), seme nella partita e partite in corso intatte quando cambiano i dati base (21), eredità a fine partita (12). Contenuto dello snapshot come in `docs/ricerca/tecnica/architettura-simulazione.md:32`. Termini nuovi in `CONTEXT.md`: partita, salvataggio, riavvolgimento, albo delle partite.

**Modello: riavvolgimento, una storia sola.** All'inizio dell'apertura di ogni turno si scrive da solo uno snapshot, e restano tutti per l'intera partita (un centinaio, costo trascurabile). Dal registro «torna al turno N» riporta all'apertura di N e cancella i turni successivi: niente rami, niente slot manuali. Scartati il solo avanti (ironman), troppo duro per una partita di più legislature e per una v1 che avrà bug di bilanciamento, e gli slot liberi, che trasformano ogni voto in un tentativo da ripetere.

**Stesso caso.** Lo snapshot contiene lo stato del generatore casuale: dopo un riavvolgimento, stesse scelte danno lo stesso esito (franchi tiratori, eventi, shock). Riavvolgere compra conoscenza, cioè il bias dei sondaggi e i conteggi visti, non fortuna. L'eredità riporta «riavvolta N volte», senza penalità: un costo dentro la partita non avrebbe una causa nel modello (15).

**Dentro il turno.** Chiudere l'app salva le scelte del turno in corso come bozza, e si riprende esattamente da lì; lo snapshot ufficiale resta quello dell'apertura, e un riavvolgimento scarta la bozza. Abbandonare è un comando a sé, confermato e definitivo, e produce l'eredità (12). Chiudere non finisce mai la partita.

**Contenuto.** Stato completo, seme, stato del generatore, versione dello schema e una copia dei dati con cui la partita è cominciata: catalogo, disposizioni, manopole, scenario. Una modifica ai file di dati (modding solo dati, 11) vale solo per le partite nuove. Lo stato comprende anche la cassa del partito e il bias comune dei sondaggi della legislatura (25), le promesse in sospeso e le spinte di posizione della campagna (26). Il bias non si mostra mai nel gioco.

**Versioni.** Un salvataggio di uno schema diverso si rifiuta con un messaggio chiaro; il file resta, l'eredità resta leggibile. Nessuna migrazione nella v1: durante la costruzione lo schema cambia di continuo e una migrazione per ogni cambio costa più della partita che salva.

**Fine e albo.** Alla fine per seggi (12) l'eredità appare come bozza, con «torna a un turno» ancora possibile: è un voto perso come gli altri. Alla conferma l'eredità si scrive nell'**albo delle partite** e gli snapshot si cancellano; l'abbandono, già confermato, ci va subito. Per ogni partita l'albo tiene un file piccolo e autonomo, leggibile anche con uno schema nuovo: l'eredità di 12 (anni al governo, leggi ancora in vigore, indicatori contro il turno zero), la serie per turno dei tre indicatori sintetici, i risultati di ogni elezione e il numero di riavvolgimenti.

**Partite in corso.** Più d'una insieme, ciascuna un file a sé, in una schermata con nome, scenario, turno e data dell'ultimo salvataggio. Una partita nuova non sostituisce le altre.

**Scelte di strumento, dichiarate e non chieste**
- JSON compresso con gzip, nella cartella dati dell'app (`~/Library/Application Support/` su macOS); il guscio Tauri legge e scrive, il core resta senza I/O (11).
- Il bias è nel file ma non leggibile a colpo d'occhio: chi decomprime il file bara sapendo di barare, e il giocatore della v1 è Mario.

Effetti sulla mappa:
- nessun ticket nuovo;
- Punti di aggancio (30): migrazioni dei salvataggi fra versioni dello schema; rami della stessa partita;
- Costo del governare (29): nessuna manopola nuova.
