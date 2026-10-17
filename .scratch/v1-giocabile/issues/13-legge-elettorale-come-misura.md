Type: grilling
Status: resolved
Blocked by: 08
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come si modella la legge elettorale come misura che il giocatore può cambiare? Quali parametri ha il menu (soglia di lista, soglia di coalizione, premio e soglia per ottenerlo, tetti, coalizioni obbligatorie, a chi va il premio; collegi uninominali sì o no?), quali combinazioni sono vietate, e come reagiscono alleati e opposizione a una riforma che li svantaggia? La 180/2026 è un preset (docs/ricerca/modelli/sistemi-elettorali.md:27-69).

Ingresso da Coalizione e crisi (05): si cambia con una legge ordinaria che segue l'iter del ticket 08 e vale dal voto successivo; la 180 è la legge di partenza.

Ingresso da Iter parlamentare (08): legge ordinaria in due turni (fiducia ammessa, costo d'intesa doppio); alla Camera voto segreto, con franchi tiratori raddoppiati nei due sensi; mai per decreto.
Da verificare sulla prassi (aperto da 08): alla Camera la fiducia sugli articoli della legge elettorale con voto finale segreto (Italicum 2015)? Se sì, i franchi tiratori sopravvivono alla fiducia nel voto finale (docs/ricerca/modelli/iter-legislativo-italiano.md:50).

Ingresso da Catalogo atti e riforme (16): le leggi di riforma sono cambi di valore di disposizioni, con linea contro lo status quo, emendamento a metà strada e abrogazione al valore precedente. Da valutare qui se soglie, premio e tetti diventano disposizioni senza dicastero con le stesse regole (suggerito, non deciso).

Ingresso da Partiti IA (18): il governo avversario dichiara alla nascita un programma pubblico di disposizioni, portate al valore più vicino alla posizione della sua coalizione. Se soglie, premio e tetti diventano disposizioni, da decidere qui se possono entrare nel programma, cioè se il governo avversario può cambiare la legge elettorale contro il giocatore. Il numero dei partiti ora varia (nascite e scissioni, tetto di 10), e le soglie di lista e di coalizione pesano su chi entra in parlamento.

## Answer

Grilling con Mario del 17/10/2026, tre giri, tutte le raccomandazioni accettate, quadro finale confermato. La prassi su fiducia e voto segreto è verificata sui resoconti della Camera (`docs/ricerca/modelli/iter-legislativo-italiano.md`, §5). Termini precisati in `CONTEXT.md`: legge elettorale, disposizione (non sempre di un dicastero), linea (regola dei seggi), questione di fiducia (eccezione elettorale).

Scelte fatte senza chiedere, perché di dettaglio:
- premio e tetto espressi in quota dei seggi, così la 180 resta identica a 400/200 e sopravvive a una norma costituzionale sul numero dei parlamentari;
- nessuna finestra di blocco prima del voto, come nella prassi italiana (Rosatellum approvato 5 mesi prima del voto del 2018);
- la proiezione usa le coalizioni dell'ultimo voto, e i partiti nati dopo corrono da soli;
- a pari seggi vale la linea ordinaria (08);
- le combinazioni impossibili sono spente nel menu.

**Menu a gradini.**
- *Parametri:* soglia di lista 0 · 3 · 4 · 5%; coalizioni ammesse sì/no, con soglia di coalizione 0 · 10% e ripescaggio sì/no; premio nessuno · alla lista · alla coalizione, con soglia per ottenerlo 40 · 42 · 45% e taglia circa 9 · 18% dei seggi; tetto 55 · 60% · nessuno; formula quozienti Hare · D'Hondt (`docs/ricerca/modelli/sistemi-elettorali.md:12-14`).
- *Fissi:* Senato su base regionale (art. 57 Cost.); premio unico per le due Camere, come nella 180 (`sistemi-elettorali.md:39`).
- *Paletti della Corte nel menu:* il premio richiede sempre una soglia di almeno il 40% (Corte cost. 1/2014 e 35/2017). In v1 la Corte non c'è (08), quindi il paletto sta nel menu.
- *Esclusi:* collegi uninominali (l'elettorato non ha geografia sotto la regione), ballottaggio, preferenze. Vanno ai punti di aggancio.
- *Impostazioni pronte:* 180/2026, proporzionale con soglia 3%, proporzionale puro.

**Forma: disposizioni fuori dicastero.** Ogni parametro è una disposizione del gruppo «legge elettorale», fuori da ogni dicastero, con le regole di 16:
- valore in vigore e valore precedente;
- abrogazione disposizione per disposizione, anche per referendum (12), che riporta al valore precedente. Al turno zero i valori precedenti sono quelli del Rosatellum (premio nessuno, tetto nessuno, ripescaggio no). Così non serve la reviviscenza dell'intera legge, che la Corte esclude (sent. 13/2012).

La propone solo il Presidente del Consiglio, senza ministro proponente; nessun ministro la mette in agenda (04). Vale dal voto successivo.

**Linea sui seggi attesi.**
- *Voto:* ogni partito proietta i seggi con la legge nuova e con quella in vigore sul sondaggio pubblico del trimestre, e vota sì se ci guadagna. Gli alleati pesano anche l'intesa, come sempre.
- *Costo per gli alleati:* un alleato che perde seggi paga intesa in proporzione, anche se vota sì.
- *Nessun costo diretto di consenso.* La contromossa dell'opposizione è il referendum abrogativo (12).
- *Anteprima:* il giocatore vede prima la tabella dei seggi guadagnati e persi da ogni partito. È l'anteprima della misura, e la sua precisione segue la nebbia «Informazione imperfetta».

**Emendamento.** Sui gradini ordinali è il gradino di mezzo. Sui parametri a scelta (a chi va il premio, coalizioni sì/no, formula) l'emendamento rinvia l'efficacia di 4 turni: se si vota prima, vale il valore in vigore. È la regola dei regimi di 16; precedente reale: l'Italicum, approvato nel 2015 e applicabile dal 1/7/2016.

**Fiducia e voto segreto.** Alla Camera la fiducia si può porre anche sulla legge elettorale, perché lì il voto segreto è solo su richiesta. Copre gli articoli, ma il voto finale resta segreto (decisione Boldrini del 28/4/2015, art. 116 c. 2). Nella realtà è andata così in tutti i casi: Italicum 334-61, Rosatellum 375-215, 180/2026 227-164. Senza fiducia, nella prima lettura della 180, un emendamento del governo è caduto 187-188. Nel gioco:
- la fiducia fa cadere gli emendamenti e risparmia un turno, al costo d'intesa doppio (08);
- il voto finale alla Camera resta segreto, con franchi tiratori raddoppiati e forbice visibile prima;
- se il voto finale boccia la legge, cade la legge ma non il governo.

**Governo avversario.** Simmetrico (12, 18): mette la legge elettorale nel programma se alla nascita la proiezione gli dà almeno N seggi in più, oppure la aggiunge più tardi, quando il sondaggio gli apre quel guadagno. La minaccia è visibile al giocatore. Valgono le stesse regole di linea, emendamento e fiducia.

Effetti sulla mappa:
- Iter parlamentare (08): chiusa la verifica aperta sulla fiducia, con la prassi documentata;
- nebbia «Costo del governare e bilanciamento»: N seggi del governo avversario, costo d'intesa per seggio perso, taglie esatte del premio;
- nebbia «Informazione imperfetta»: precisione della tabella dei seggi guadagnati e persi, calcolata sul sondaggio pubblico;
- punti di aggancio: collegi uninominali (quindi l'impostazione Rosatellum), ballottaggio, preferenze;
- nebbia «Campagna elettorale»: la scelta della coalizione pre-elettorale (05) ora dipende anche da soglie e premio scelti dal giocatore.
