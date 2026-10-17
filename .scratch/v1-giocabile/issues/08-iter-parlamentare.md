Type: grilling
Status: resolved
Blocked by: 01
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come passano le misure normative nella v1: decreto (durata, conversione in legge, decadenza), legge (iter, voto probabilistico, emendamenti sì o no), norma costituzionale (maggioranze, referendum)? Cosa decide il giocatore in ciascun passaggio?

## Answer

Grilling con Mario del 17/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Fatti istituzionali in `docs/ricerca/modelli/iter-legislativo-italiano.md` (ricerca della stessa sessione). Termini nuovi in `CONTEXT.md`: linea, franco tiratore, emendamento, calendario, questione di fiducia, esercizio provvisorio, referendum confermativo; precisati decreto e norma costituzionale.

**Voto in aula, per tutte le forme.** Ogni partito ha una **linea** (sì/no) calcolata dalla distanza fra la posizione della misura e la sua, più l'intesa per gli alleati e un malus fisso per l'opposizione, che vota sì solo a misure molto vicine. Una parte dei suoi parlamentari defeziona (**franchi tiratori**) con probabilità crescente con la distanza e con l'intesa bassa, estratta col seme della partita. Prima del voto il giocatore vede le linee e una forbice; il conteggio esatto arriva dopo. Nessun parlamentare personaggio. La sfiducia resta deterministica (05). Sulla legge elettorale alla Camera, dove venti deputati possono chiedere lo scrutinio segreto, i franchi tiratori raddoppiano nei due sensi; al Senato no (iter-legislativo-italiano.md:48-50).

**Decreto.** Ammesso solo con un innesco di necessità e urgenza: un evento aperto o una variazione di fondi durante l'anno. Mai su legge elettorale, norme costituzionali o bilancio (l. 400/1988 art. 15, iter-legislativo-italiano.md:20-23). Con una crisi del debito aperta passa anche una riforma. Vale subito e la conversione si vota a fine dello stesso turno: 60 giorni contro i 43-44 medi reali (iter-legislativo-italiano.md:12-16, 92). Se decade, gli effetti normativi cessano (aliquote e regimi tornano come prima), il denaro speso resta speso; costi in soddisfazione del ministro proponente e in valence. Non si riemana (Corte cost. 360/1996): poi solo per legge. Il giocatore decide se emanarlo e se porre la fiducia sulla conversione.

**Legge ordinaria.** Presentazione e commissione nel turno t, voto delle due Camere sullo stesso testo a fine t+1, senza navetta. Il dato reale è più lento: 272-333 giorni medi per le leggi governative, cioè 3-4 turni (iter-legislativo-italiano.md:90). I 2 turni sono un parametro di bilanciamento. Prima del voto ogni alleato distante almeno 2 punti dalla misura, o colpito nei suoi interessi, presenta un **emendamento**, cioè un'attenuazione: se il giocatore la accoglie il partito vota compatto, se la respinge paga in intesa e i franchi tiratori aumentano. Il **calendario** ha 3 posti per le ordinarie e 1 per una costituzionale; i decreti hanno la precedenza.
- *Legge respinta:* decade e si ripresenta dopo 2 turni, i sei mesi dei regolamenti (iter-legislativo-italiano.md:54-55), anche attenuata. Il ministro proponente perde soddisfazione e c'è un costo di valence; se c'è un innesco resta il decreto.
- *Fine legislatura:* decadono tutte le leggi in esame.

**Questione di fiducia.** Sulle leggi e sulle conversioni, non sulle norme costituzionali. Gli emendamenti cadono, il voto è a blocchi senza franchi tiratori e avviene alla fine del turno di presentazione, quindi risparmia un turno. Costa intesa a ogni alleato, di più se la misura è lontana dalla sua posizione; sulla legge elettorale il costo è doppio. Un alleato vota no solo se è in ultimatum (intesa sotto 40) e la misura dista almeno 2 punti: lo si vede prima. Se la fiducia è respinta il governo si dimette, come vuole la prassi, non un obbligo (art. 94 c.4, iter-legislativo-italiano.md:38-44), e si segue il percorso delle dimissioni (05).

**Legge di bilancio.** Si presenta a inizio turno ottobre-dicembre e si vota a fine dello stesso turno; occupa tutto il calendario e ammette la fiducia. Gli emendamenti degli alleati sono richieste di budget per i dicasteri della loro quota. Se non passa, un turno di **esercizio provvisorio** (cifre dell'anno prima, nessuna misura con costo; art. 81 c.5, massimo 4 mesi, iter-legislativo-italiano.md:73-76); se non passa nemmeno allora, il governo si dimette.

**Norma costituzionale.** Nella v1: durata della legislatura e numero dei parlamentari. Art. 138 fedele (iter-legislativo-italiano.md:64-69): prima deliberazione a fine turno t, seconda a fine t+1 o dopo, a maggioranza assoluta. Senza i 2/3 in una Camera, l'opposizione chiede il **referendum confermativo** se ha 1/5 dei seggi ed è contraria. Si tiene nel turno dopo, senza quorum, e lo vota l'elettorato dei gruppi (06): distanza dalla misura più giudizio sul governo. Niente fiducia; decade a fine legislatura.

**Capo dello Stato.** Promulga sempre. Nella v1 niente rinvio (art. 74) e niente Corte costituzionale.

**Cosa decide il giocatore.** Quale misura portare e in che forma (decreto solo con un innesco); cosa mettere in calendario; accogliere o respingere gli emendamenti; porre la fiducia; dopo un voto perso, ripresentare, attenuare o passare al decreto; per una norma costituzionale, cercare i 2/3 o rischiare il referendum.

**Ne escono:**
- Legge elettorale come misura (13): eredita voto segreto e fiducia. Da verificare sulla prassi: fiducia sugli articoli con voto finale segreto (Italicum 2015)? Se sì, i franchi tiratori sopravvivono alla fiducia nel voto finale. La pagina lo segna come inferenza aperta (iter-legislativo-italiano.md:50).
- Opposizione (12): referendum abrogativo (art. 75) e leggi proposte dall'opposizione.
- Bilanciamento (nebbia): tasso dei franchi tiratori, malus d'opposizione, costi della fiducia e degli emendamenti, durata della legge ordinaria (2 turni contro 3-4 reali).
- Punti di aggancio (nebbia): rinvio presidenziale, Corte costituzionale, forma di governo e bicameralismo come norme costituzionali, sede deliberante, navetta, tetto ai decreti per turno (iter-legislativo-italiano.md:105-116, proposte non adottate).
