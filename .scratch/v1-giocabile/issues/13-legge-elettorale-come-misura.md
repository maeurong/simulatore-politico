Type: grilling
Status: open
Blocked by: 08
Label: wayfinder:grilling
Map: ../map.md

## Question

Come si modella la legge elettorale come misura che il giocatore può cambiare? Quali parametri ha il menu (soglia di lista, soglia di coalizione, premio e soglia per ottenerlo, tetti, coalizioni obbligatorie, a chi va il premio; collegi uninominali sì o no?), quali combinazioni sono vietate, e come reagiscono alleati e opposizione a una riforma che li svantaggia? La 180/2026 è un preset (docs/ricerca/modelli/sistemi-elettorali.md:27-69).

Ingresso da Coalizione e crisi (05): si cambia con una legge ordinaria che segue l'iter del ticket 08 e vale dal voto successivo; la 180 è la legge di partenza.

Ingresso da Iter parlamentare (08): legge ordinaria in due turni (fiducia ammessa, costo d'intesa doppio); alla Camera voto segreto, con franchi tiratori raddoppiati nei due sensi; mai per decreto.
Da verificare sulla prassi (aperto da 08): alla Camera la fiducia sugli articoli della legge elettorale con voto finale segreto (Italicum 2015)? Se sì, i franchi tiratori sopravvivono alla fiducia nel voto finale (docs/ricerca/modelli/iter-legislativo-italiano.md:50).

Ingresso da Catalogo atti e riforme (16): le leggi di riforma sono cambi di valore di disposizioni, con linea contro lo status quo, emendamento a metà strada e abrogazione al valore precedente. Da valutare qui se soglie, premio e tetti diventano disposizioni senza dicastero con le stesse regole (suggerito, non deciso).

Ingresso da Partiti IA (18): il governo avversario dichiara alla nascita un programma pubblico di disposizioni, portate al valore più vicino alla posizione della sua coalizione. Se soglie, premio e tetti diventano disposizioni, da decidere qui se possono entrare nel programma, cioè se il governo avversario può cambiare la legge elettorale contro il giocatore. Il numero dei partiti ora varia (nascite e scissioni, tetto di 10), e le soglie di lista e di coalizione pesano su chi entra in parlamento.
