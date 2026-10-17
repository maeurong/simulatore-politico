Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Quali sono i 40 atti del ministro (4 per dicastero) e le leggi di riforma della v1, ciascuno con effetti sulle grandezze del modello, costo, ritardo, posizione sull'asse e il punteggio con cui il ministro lo sceglie (Autonomia dei ministri, 04)? Fra le leggi di riforma: i regimi e i parametri della spesa obbligatoria (pensioni retributivo / contributivo / quota base universale; sussidi assicurativo / universale; voci nuove come diritti permanenti), detrazioni e agevolazioni fiscali, riforme del lavoro su u*. Quanto del catalogo va deciso con Mario e quanto è contenuto da scrivere?

Ingressi: Dicasteri (03) dà la tabella delle grandezze per dicastero; Modello economico (07) le grandezze che esistono davvero (10 settori, quattro stock del PIL potenziale, salari/acquisti, spesa obbligatoria).

## Answer

Grilling con Mario del 17/10/2026, tre giri, tutte le raccomandazioni accettate, quadro finale confermato. Schema, elenco e Sanità d'esempio in [16-catalogo-schema-e-sanita.md](../assets/16-catalogo-schema-e-sanita.md). Termini nuovi in `CONTEXT.md`: cantiere, disposizione, legge di riforma; precisati atto del ministro e linea. Il catalogo è soprattutto grammatica: meno effetti scritti a mano, meno bilanciamento manuale, il collo di bottiglia dichiarato da Democracy (`docs/ricerca/sistemi/policy-e-effetti.md:51`).

Scelte fatte senza chiedere, perché di dettaglio: posizione degli atti scritta a mano, scala valore → posizione per ogni disposizione, media delle posizioni per una legge su più disposizioni; voci nuove (diritti permanenti, 07) come disposizioni che partono «non istituite»; il punteggio del ministro (04) usa l'effetto nominale scritto, non una simulazione; costo degli atti in quote trimestrali del margine di riferimento.

**Atti del ministro.** 40, quattro per dicastero in una griglia 2×2 velocità × posizione: un atto rapido e visibile e uno lento e strutturale per ciascun lato dell'asse. Così ogni dicastero porta il dilemma fra ritardo e consenso (`docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:36`) e la posizione del ministro cambia davvero la scelta.
- *Forma:* puntuale (costa nel turno in cui si esegue) o **cantiere** (impegna il margine per N turni, rende a lavori finiti, fermato a metà perde quanto speso: grandi opere e investimenti lenti).
- *Effetti:* 1-2 grandezze della riga del dicastero (03), in unità vere, con ritardo e forma del ritardo: impulso che decade o stock con vita utile (`ritardi-e-inerzia-delle-politiche.md:16-18`), mai media mobile uguale per tutti. La capacità moltiplica l'intensità e divide il ritardo (04). La spesa va sempre in domanda finale col vettore del dicastero (07). Mai effetti diretti su posizioni, valence o intesa.
- *Prerequisito* facoltativo: una disposizione a un certo valore (le convenzioni col privato chiedono il privato accreditato almeno «attuale»).

**Leggi di riforma = disposizioni.** Nessuna riforma con effetti scritti a parte: ogni dicastero possiede alcune **disposizioni** (regime o parametro con un valore in vigore) e una legge di riforma ne porta una o più a un valore nuovo. Effetti dal motore dove la regola esiste già (pensioni, sussidi, fisco, u*: 07), altrimenti da una tabella valore → effetto. Regole generiche:
- *Linea* (precisa 08 per le sole leggi di riforma): un partito vota sì se il valore nuovo è più vicino alla sua posizione di quello in vigore, più intesa e malus d'opposizione; la posizione della misura, per la posizione percepita (06), è quella del valore nuovo.
- *Emendamento:* valore a metà strada sui parametri; entrata in vigore rinviata di un anno sui regimi.
- *Abrogazione,* anche per referendum (12): ritorno al valore precedente.
- *Turno zero:* valori reali 2026, ciascuno con la legge reale che l'ha fissato e il valore precedente, quindi abrogabili anche le leggi vere (Jobs Act → reintegro).
- *Proposta del ministro* (innesco 3 di 04): la disposizione del suo dicastero più lontana dalla sua posizione, al valore più vicino, se la distanza è almeno 2; oppure lo sblocco del prerequisito di un atto che serve al suo obiettivo.

**Elenco:** 30 disposizioni approvate (tabella nell'asset). Lavoro e Welfare ne ha 8, quindi è il dicastero più conteso nella spartizione (05) e quello che si prende più colpa (06).

**Confine fra decisione e contenuto.** Deciso qui: schema di atto e disposizione, elenco delle 30 con i loro valori, Sanità scritta per intero come modello (4 atti, 3 disposizioni). Contenuto da scrivere dopo la spec, in un file di dati sotto revisione di Mario: gli altri 36 atti e le tabelle valore → effetto. I numeri vanno al bilanciamento, i valori di partenza al ticket di scenario.

Effetti sulla mappa:
- si sblocca Partiti IA (18): il governo avversario porta leggi di riforma come cambi di disposizioni;
- Legge elettorale come misura (13): soglie, premio e tetti possono essere disposizioni senza dicastero, con le stesse regole di linea, emendamento e abrogazione (suggerito, non deciso);
- Crisi ed eventi (15): la condizionalità del MES impone valori di disposizioni;
- Tabella degli interessi (17): decidere se qualche disposizione entra direttamente negli interessi (licenziamento, salario minimo) o solo attraverso gli indicatori;
- nebbia «Scenario iniziale»: verificare valori di partenza e leggi d'origine delle 30 disposizioni; nebbia «Costo del governare e bilanciamento»: costi e intensità degli atti, scale delle disposizioni.
