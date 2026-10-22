# Spec: v1 giocabile del simulatore politico

Status: ready-for-agent
Label: ready-for-agent
Map: [map.md](../../../.scratch/v1-giocabile/map.md)
Origine: i 30 ticket risolti in [.scratch/v1-giocabile/issues/](../../../.scratch/v1-giocabile/issues/), il glossario [`CONTEXT.md`](../../../CONTEXT.md), le decisioni in [`docs/ricerca/sintesi/decisioni.md`](../../ricerca/sintesi/decisioni.md).

Ogni termine in grassetto o in corsivo che compare nel glossario ha il senso che gli dà `CONTEXT.md`. Fra parentesi, il numero del ticket che ha preso la decisione: il dettaglio, le fonti e i numeri stanno lì. Tutte le soglie, i pesi e i costi che la spec non dichiara reali sono provvisori e vanno al bilanciamento (29).

## Problem Statement

Mario vuole giocare a governare l'Italia di oggi: istituzioni, regole e dati reali, personaggi e partiti inventati, turni di tre mesi per più legislature. I giochi che esistono non danno questa esperienza. Democracy 4 ha un'opposizione finta, contraria al 100% su tutto, ed eventi senza causa; altri titoli fanno decidere la partita alla campagna invece che al governo; in quasi tutti i partiti giocano allo stesso modo, i ministri sono figurine e l'economia non ha conti che tornano. Nessuno ha la legge elettorale vigente, l'iter parlamentare italiano, le regole fiscali UE o il costo del governare come lo misura la letteratura.

Il gioco deve far sentire che governare costa: ogni misura ha perdenti che pesano più dei vincitori, i ministri hanno un'agenda propria, gli alleati hanno una pazienza che finisce, i numeri arrivano in ritardo e con errore. Perdere il governo non deve chiudere la partita: dall'opposizione si gioca ancora, e si può tornare.

## Solution

Un'applicazione desktop per il Mac (Tauri 2 con il core di simulazione in Rust e la UI in Svelte) in cui il giocatore fonda un partito, lo porta al voto del turno zero (ottobre-dicembre 2026) con la legge elettorale 180/2026 e, se vince, diventa Presidente del Consiglio di una coalizione. Ogni turno è un trimestre in tre fasi: apertura con l'esito del turno prima, decisioni in ordine libero, chiusura in ordine fisso. Dieci ministri inventati agiscono da soli nei loro dicasteri e il giocatore li ferma, li dirotta o ne accoglie le proposte; le misure passano dal parlamento con linee di partito e franchi tiratori; un motore economico trimestrale muove PIL, prezzi, lavoro, conti e spread; 360 gruppi di elettori giudicano il governo sui loro interessi e votano con un logit. Quando perde il governo (sfiducia, sconfitta, ribaltone) il giocatore passa all'opposizione con leve proprie e può tornare. La partita finisce solo quando il suo partito resta senza seggi, o per abbandono, con l'eredità.

Tutto è leggibile: ogni variazione mostra la sua causa nel modello, ogni avversario si muove con regole spiegate in una riga, ogni crisi lenta si vede arrivare. La grafica è quella di un settimanale satirico inventato, «Il Pungolo», con la cornice giocosa che tace quando il fatto è grave. Il caso è a seme fisso: stesso binario, stesso seme, stesse scelte, stessa partita.

## User Stories

### Nuova partita, scenario e fondazione

1. Come giocatore, voglio avviare una partita nuova dall'Italia di oggi ai dati reali, così che il paese che governo sia quello che conosco.
2. Come giocatore, voglio scegliere fra lo scenario di base e uno scenario su misura, nuovo o caricato da file, così che possa provare un'Italia diversa senza toccare la struttura del gioco.
3. Come giocatore, voglio sei cursori nell'editor (debito, ricchezza, disoccupazione, inflazione di partenza, età della popolazione, orientamento dell'elettorato) più i menu della legge elettorale e delle disposizioni, così che posso cambiare lo stato del paese senza scrivere file.
4. Come giocatore, voglio un riepilogo dal vivo con consenso atteso per posto, maggioranza attesa, spread, interessi, disavanzo e avvisi a ogni cambio di cursore, così che vedo subito cosa ho combinato.
5. Come giocatore, voglio che il cursore si fermi al limite dell'intervallo in cui il modello è tarato, così che non costruisco scenari in cui il motore dice cose senza senso.
6. Come giocatore, voglio un avviso quando nessuna coalizione con me supera il 42% atteso da qualche posto o quando una situazione o una crisi è già accesa al turno zero, così che so in che guaio mi metto senza che il gioco me lo vieti.
7. Come giocatore, voglio salvare lo scenario su misura con un nome fra i miei file, così che posso rigiocarlo.
8. Come giocatore, voglio dare un nome al mio partito e sceglierne la posizione su uno dei sei mezzi punti fra due partiti vicini (1,5 … 6,5), così che la scelta del posto sull'asse sia la prima decisione strategica.
9. Come giocatore, voglio vedere per ogni posto l'istogramma dell'elettorato, il risultato atteso e le coalizioni possibili con la loro disponibilità, letti sullo stesso sondaggio rumoroso del gioco, così che scelgo fra più elettori e meno concorrenza sapendo cosa rischio.
10. Come giocatore, voglio partire un turno prima del voto, col parlamento chiuso e il bilancio 2027 già approvato dal governo uscente, così che il primo turno è una campagna e non una trattativa già vinta.
11. Come giocatore, voglio che il mio partito nuovo abbia una luna di miele uguale da ogni posto (circa il 15% al posto medio), così che la vittoria è favorita ma non certa.
12. Come giocatore, voglio che il parlamento di partenza, i sondaggi e i giudizi escano tutti dal voto simulato dal motore, così che sono coerenti fra loro dal primo turno.

### Il turno

13. Come giocatore, voglio che ogni turno sia un trimestre in tre fasi (apertura, decisioni, chiusura), così che il ritmo è sempre lo stesso e lo imparo presto.
14. Come giocatore, voglio che l'apertura mi riveli l'esito del turno prima una voce alla volta, nell'ordine della chiusura e ciascuna con la sua causa, così che capisco cosa è successo e perché.
15. Come giocatore, voglio al massimo sette voci in evidenza nell'esito, prima le critiche, e il resto nel registro, così che non annego nelle notifiche.
16. Come giocatore, voglio consultare il registro completo di ogni turno passato, così che posso ricostruire una catena di cause.
17. Come giocatore, voglio decidere in ordine libero, così che il turno segue il mio ragionamento e non un modulo.
18. Come giocatore, voglio poter chiudere il turno in qualunque momento senza clic obbligati, con esiti di default per ogni decisione non presa, così che un turno tranquillo dura poco.
19. Come giocatore, voglio che nessuna finestra modale mi obblighi a rispondere, così che il gioco non mi interrompe.
20. Come giocatore, voglio che la chiusura esegua sempre atti dei ministri → voti d'aula → economia → elettorato → soglie → eventi del turno dopo, così che l'ordine delle conseguenze è prevedibile.
21. Come giocatore, voglio tre indicatori sintetici sempre a schermo (consenso, maggioranza, conti), uno per ogni modo di perdere il governo, più data e turni al voto, così che so sempre quanto sono vicino alla caduta.
22. Come giocatore, voglio che un turno ordinario duri 5-10 minuti e uno pesante (bilancio, elezioni, crisi) 20-30, così che una legislatura si gioca in 2-3 ore.
23. Come giocatore, voglio che nessuna moneta delle azioni limiti le mie mosse, ma solo calendario, budget, inneschi e costi in soddisfazione, intesa e valence, così che i limiti hanno una causa istituzionale.

### Ministri e dicasteri

24. Come giocatore, voglio dieci dicasteri (Economia e Finanze, Sviluppo economico, Lavoro e Welfare, Sanità, Istruzione e Ricerca, Infrastrutture e Trasporti, Ambiente ed Energia, Interno, Giustizia, Esteri e Difesa), ciascuno con un indicatore di servizio e le sue grandezze, così che governare ha una forma riconoscibile.
25. Come giocatore, voglio vedere ogni ministro con capacità, posizione, obiettivo e soddisfazione, così che capisco perché fa quello che fa.
26. Come giocatore, voglio che ogni ministro annunci a inizio turno l'atto in agenda col motivo in una riga, così che posso intervenire prima che lo esegua.
27. Come giocatore, voglio un listino dei dieci ministri in una tabella densa, con in evidenza solo le righe sopra soglia, contro la posizione del mio partito o legate a un evento aperto, così che guardo solo ciò che merita un clic.
28. Come giocatore, voglio fermare un atto (il margine resta) o dirottarlo su un altro dei quattro, pagando soddisfazione in proporzione a quanto l'atto scartato stava a cuore al ministro, così che il mio controllo ha un prezzo.
29. Come giocatore, voglio che un ministro con il margine esaurito non faccia atti, così che il budget conta.
30. Come giocatore, voglio che ogni ministro abbia al massimo una proposta normativa in sospeso, con tre inneschi (fondi che non bastano, evento nel suo dicastero, posizione lontana da una disposizione), così che le proposte arrivano per una ragione.
31. Come giocatore, voglio accogliere, rinviare una volta senza costo o respingere una proposta, e che una proposta ignorata valga come respinta dopo un turno, così che posso prendere tempo ma non all'infinito.
32. Come giocatore, voglio che ogni norma, anche mia, abbia un ministro proponente che firma e perde soddisfazione se la norma va contro la sua posizione, così che le mie riforme hanno un costo dentro il governo.
33. Come giocatore, voglio vedere la soddisfazione di ogni ministro con le sue cause, la minaccia di dimissioni sotto 40 e le dimissioni sotto 25 a fine turno, così che le crisi personali si vedono arrivare.
34. Come giocatore, voglio chiedere le dimissioni a un ministro (gratis con un tecnico, un colpo all'intesa del suo partito altrimenti), così che posso cambiare squadra a un prezzo chiaro.
35. Come giocatore, voglio scegliere il successore da una rosa di 3 nomi del partito, o fra 3 tecnici, con un turno di interim e uno di rodaggio, così che sostituire un ministro costa tempo.
36. Come giocatore, voglio che gli atti di Infrastrutture e Trasporti e di Sviluppo economico abbiano una regione bersaglio scelta dal ministro per bisogno e per voti del suo partito («Calabria: ci votano»), così che il clientelismo è una tentazione visibile.
37. Come giocatore, voglio dirottare l'atto, la regione o entrambi finché l'atto è in agenda, mai a lavori avviati, così che posso correggere una scelta clientelare.
38. Come giocatore, voglio che un cantiere fermato a metà perda quanto speso, così che avviare un'opera è un impegno.

### Bilancio e conti pubblici

39. Come giocatore, voglio presentare nel turno ottobre-dicembre la legge di bilancio con le aliquote delle quattro imposte (IRPEF a scaglioni, IVA, IRES, aliquota contributiva), il budget di ogni dicastero diviso in funzionamento e margine, il fondo di riserva e il saldo contro il tetto UE, così che una volta l'anno decido dove vanno i soldi.
40. Come giocatore, voglio che la bozza del bilancio proponga le cifre nominali dell'anno prima, con «indicizza all'inflazione» per tutto o per voce e il costo esatto, così che vedo che un bilancio fotocopia è un taglio reale.
41. Come giocatore, voglio vedere accanto a ogni budget e scaglione la perdita reale dall'ultimo aggiornamento («Sanità: −4% reale in un anno»), così che l'erosione dell'inflazione non mi sorprende.
42. Come giocatore, voglio che tagliare la spesa di funzionamento peggiori l'indicatore di servizio con ritardo, e che tagliarne i salari blocchi il turnover, così che i tagli hanno conseguenze materiali.
43. Come giocatore, voglio allargare il margine discrezionale (1,6% al turno zero) togliendo al funzionamento o facendo disavanzo, così che decido io quanto spazio dare ai ministri.
44. Come giocatore, voglio che gli alleati chiedano emendamenti al bilancio sotto forma di budget per i loro dicasteri, così che il bilancio è una trattativa.
45. Come giocatore, voglio che un bilancio non approvato porti a un turno di esercizio provvisorio (cifre dell'anno prima, nessuna misura con costo) e, se fallisce ancora, alle dimissioni, così che il bilancio è il voto più importante dell'anno.
46. Come giocatore, voglio spostare fondi durante l'anno solo per decreto, così che il bilancio vincola.
47. Come giocatore, voglio spendere il fondo di riserva nelle emergenze, e se finisce ricorrere a un decreto a debito, così che ho una prima risposta rapida e limitata.
48. Come giocatore, voglio cambiare per legge regime e parametri della spesa obbligatoria (pensioni: retributivo, contributivo, quota base universale; sussidi: assicurativo o universale) con effetto sulle coorti future pro rata, così che le riforme strutturali hanno i tempi veri.
49. Come giocatore, voglio istituire per legge voci nuove di spesa obbligatoria che pesano ogni anno sul tetto, così che un diritto permanente ha un costo permanente.
50. Come giocatore, voglio il controllo UE nel turno di primavera, con la proiezione del conto di controllo visibile negli altri turni, e la PDE come macchina a stati con ammende e sospensione dei fondi di coesione, così che le regole fiscali europee mordono.
51. Come giocatore, voglio presentare un piano strutturale riveduto nel primo anno di un governo nuovo, così che un governo che arriva può ridiscutere il percorso.
52. Come giocatore, voglio uno spread piatto finché i conti reggono e convesso oltre una soglia, con il TPI che azzera il premio di sfiducia solo se rispetto le regole UE, così che la disciplina ha un valore concreto.
53. Come giocatore, voglio che oltre una soglia le aste vadano deserte e io debba scegliere in un turno fra MES con condizionalità e ristrutturazione del debito, così che la crisi del debito è una scelta tragica e non un game over.

### Misure e parlamento

54. Come giocatore, voglio portare una misura in una delle quattro forme (atto del ministro, decreto, legge, norma costituzionale), così che lo strumento conta quanto il contenuto.
55. Come giocatore, voglio un'anteprima di ogni misura con costo esatto, effetti a fasce (poco, medio, molto) stimati dal ministro proponente, ritardo, gruppi colpiti, linee dei partiti e forbice del voto, così che decido sapendo quanto so.
56. Come giocatore, voglio che un ministro capace stimi meglio gli effetti, così che la capacità vale anche come affidabilità.
57. Come giocatore, voglio una legge di riforma che porti una o più disposizioni a un valore nuovo, compreso quello di prima, così che tornare indietro è una riforma come le altre.
58. Come giocatore, voglio vedere ogni disposizione col valore in vigore, la legge reale che l'ha fissato, il valore precedente e l'eventuale valore in arrivo, così che so da dove parto.
59. Come giocatore, voglio che un decreto sia ammesso solo con un evento aperto o per spostare fondi, valga subito e decada a fine turno se non convertito, senza poterlo ripetere, così che il decreto è uno strumento d'urgenza.
60. Come giocatore, voglio che una legge ordinaria si presenti nel turno t e si voti a fine t+1 in entrambe le Camere sullo stesso testo, così che legiferare richiede tempo.
61. Come giocatore, voglio un calendario con 3 posti per le leggi ordinarie e 1 per una costituzionale, i decreti in precedenza, la legge di bilancio che lo occupa tutto nel suo turno e un posto al trimestre riservato all'opposizione, così che devo scegliere le priorità.
62. Come giocatore, voglio vedere prima del voto la linea di ogni partito e la forbice dei franchi tiratori, e il conteggio esatto dopo, così che ogni voto è una scommessa informata.
63. Come giocatore, voglio che ogni alleato distante almeno 2 punti dalla misura, o colpito nei suoi interessi, chieda un emendamento (valore a metà strada, o entrata in vigore rinviata per un regime) e che respingerlo costi intesa e franchi tiratori, così che la coalizione annacqua le riforme.
64. Come giocatore, voglio porre la questione di fiducia su una legge o una conversione per far cadere emendamenti e franchi tiratori e risparmiare un turno, al costo dell'intesa degli alleati e col rischio di cadere, così che posso forzare a mio rischio.
65. Come giocatore, voglio sapere prima della fiducia quali alleati voterebbero no (in ultimatum e distanti almeno 2 punti), così che la scommessa è leggibile.
66. Come giocatore, voglio che una legge respinta si possa ripresentare solo dopo 2 turni, anche attenuata, così che una sconfitta in aula pesa.
67. Come giocatore, voglio che a fine legislatura decadano le leggi in esame, così che il tempo stringe.
68. Come giocatore, voglio proporre una norma costituzionale (durata della legislatura, numero dei parlamentari) con due deliberazioni a distanza di almeno un turno e il referendum confermativo se manco i due terzi, così che cambiare le regole del gioco è difficile come nella realtà.
69. Come giocatore, voglio che un cambio del numero dei parlamentari ripartisca i seggi per regione dalla popolazione censita, così che la riforma ha effetti coerenti.

### Legge elettorale

70. Come giocatore, voglio partire con la legge 180/2026 riprodotta esattamente (400/200 seggi, soglie di lista e coalizione, ripescaggio, premio 70/35 al 42% in entrambe le Camere, tetti 220/113), così che il voto è quello vero.
71. Come giocatore, voglio cambiare la legge elettorale da un menu a gradini (soglia di lista, coalizioni, soglia di coalizione, ripescaggio, premio, soglia del premio, taglia, tetto, formula) con le impostazioni pronte (180, proporzionale al 3%, proporzionale puro), così che posso piegare le regole a mio vantaggio.
72. Come giocatore, voglio che il menu spenga le combinazioni impossibili e imponga una soglia di almeno il 40% al premio, così che le regole restano costituzionalmente plausibili anche senza Corte.
73. Come giocatore, voglio vedere prima del voto la tabella dei seggi guadagnati e persi da ogni partito con la legge nuova, calcolata sul sondaggio pubblico, così che so chi voterà sì e perché.
74. Come giocatore, voglio che gli alleati che perdono seggi paghino intesa anche se votano sì, così che riscrivere le regole incrina la coalizione.
75. Come giocatore, voglio che sulla legge elettorale la fiducia copra gli articoli ma il voto finale alla Camera resti segreto con franchi tiratori raddoppiati, e che una bocciatura faccia cadere la legge ma non il governo, così che la prassi italiana è rispettata.
76. Come giocatore, voglio che la legge elettorale nuova valga dal voto successivo, così che non posso cambiarla a urne aperte.

### Coalizione, crisi, elezioni

77. Come giocatore, voglio dichiarare la coalizione nel turno prima del voto, vedendo la disponibilità di ogni partito con la sua causa, così che costruisco l'alleanza sui sondaggi.
78. Come giocatore, voglio cedere posti delle liste comuni per attirare un partito, pagandoli con seggi miei, così che ho una leva nella trattativa pre-elettorale.
79. Come giocatore, voglio che dopo il voto, se vinco, si apra la spartizione dei dieci dicasteri, con la richiesta di Gamson di ogni alleato in vista, così che so quanto devo dare.
80. Come giocatore, voglio proporre io la spartizione, con l'intesa di partenza di ogni alleato alzata o abbassata da quanto gli do rispetto alla richiesta, così che la generosità e l'avarizia hanno un prezzo.
81. Come giocatore, voglio nominare tecnici nella quota del mio partito, così che posso mettere competenza dove serve senza scontentare gli alleati.
82. Come giocatore, voglio che senza premio si tratti anche l'allargamento ad altri partiti, così che le leggi proporzionali cambiano il gioco.
83. Come giocatore, voglio vedere l'intesa di ogni alleato (0-100) con le sue cause, l'ultimatum sotto 40 con la causa principale e l'uscita sotto 25 a fine turno, così che le crisi di coalizione si vedono arrivare.
84. Come giocatore, voglio che l'uscita di un alleato porti a un rimpasto se ho ancora la maggioranza, o a un turno di crisi per allargare o dimettermi, così che una rottura non è automaticamente la fine.
85. Come giocatore, voglio che la sfiducia sia un voto palese a blocchi e deterministico, così che la caduta non dipende dal caso.
86. Come giocatore, voglio potermi dimettere e che il Capo dello Stato cerchi una maggioranza di partiti contigui senza di me (ribaltone) o sciolga le Camere, così che il voto anticipato è una scommessa sui sondaggi.
87. Come giocatore, voglio che il voto si tenga nella chiusura dell'ultimo turno della legislatura, col risultato e i seggi nell'apertura del turno dopo e il governo uscente in interim durante la trattativa, così che le elezioni hanno il loro ritmo.

### Opposizione

88. Come giocatore, voglio che perdere il governo mi porti all'opposizione e la partita continui, così che una sconfitta è un capitolo e non la fine.
89. Come giocatore all'opposizione, voglio lo stesso turno a tre fasi in 2-4 minuti, con «avanti fino a novità» che concatena le chiusure e si ferma quando serve una mia scelta, così che i turni vuoti scorrono.
90. Come giocatore all'opposizione, voglio vedere tutto ciò che è pubblico (indicatori, sondaggio, registro, misure del governo avversario in calendario con l'anteprima, intesa dei suoi alleati in tre fasce), ma non le agende dei suoi ministri prima dell'esecuzione, così che l'opposizione ha meno informazione del governo.
91. Come giocatore all'opposizione, voglio decidere la linea del mio partito misura per misura, sapendo che votare lontano dalla mia posizione mi costa franchi tiratori e sposta la mia posizione percepita, così che opporsi è una scelta e non un riflesso.
92. Come giocatore all'opposizione, voglio presentare la sfiducia al governo o a un ministro se ho un decimo dei seggi (anche con le firme di altri), sapendo prima quali alleati avversari la voterebbero, così che posso far cadere il governo quando è fragile.
93. Come giocatore all'opposizione, voglio che una sfiducia respinta ricompatti la maggioranza e mi costi un po' di valence, così che le sfiducie di bandiera hanno un prezzo.
94. Come giocatore all'opposizione, voglio portare una proposta di legge nel posto riservato, così che costringo la maggioranza a votare contro misure care ai suoi elettori.
95. Come giocatore all'opposizione, voglio chiedere un referendum abrogativo su una disposizione (una per quesito, firme in un turno, finestra annuale, quorum della maggioranza degli aventi diritto), così che posso cancellare una legge passando sopra al parlamento.
96. Come giocatore, voglio che il referendum abrogativo non si applichi alle sette disposizioni fiscali e contributive, al bilancio e all'amnistia, e che il gradino abrogato resti bloccato fino alla legislatura successiva, così che il referendum ha i limiti veri.
97. Come giocatore all'opposizione, voglio chiedere il referendum confermativo su una norma costituzionale approvata senza i due terzi, se ho un quinto dei seggi, così che posso fermare una riforma costituzionale.
98. Come giocatore all'opposizione, voglio tornare al governo col voto o con un ribaltone in cui il mio partito è il primo, o in cui chiedo la guida come condizione, così che ci sono due strade per tornare.
99. Come giocatore, voglio che la partita finisca solo quando il mio partito resta senza seggi in entrambe le Camere, o per abbandono, così che non c'è un termine arbitrario.

### Partiti IA e governo avversario

100. Come giocatore, voglio sette partiti inventati su sette posti dell'asse, ciascuno con vocazione e regola di movimento visibili, così che ogni avversario gioca in modo diverso e leggibile.
101. Come giocatore, voglio che ogni mossa di un partito IA sia spiegata in una riga nell'esito, così che capisco l'avversario invece di subirlo.
102. Come giocatore, voglio che un partito IA sposti la sua posizione dichiarata di mezzo punto al massimo nel turno prima del voto, senza scavalcare il vicino, così che i nomi sull'asse restano veri.
103. Come giocatore, voglio che ogni partito IA abbia un leader con nome e caricatura che porta la valence e cambia dopo una sconfitta, così che i partiti hanno un volto.
104. Come giocatore all'opposizione, voglio un governo avversario con un programma pubblico di disposizioni da cambiare, mandate in aula in quell'ordine, così che so cosa sta per fare.
105. Come giocatore all'opposizione, voglio che il governo avversario segua le mie stesse regole di coalizione, intesa e crisi e una condotta sola, non cadere, così che posso prevederlo e sfruttarne le debolezze.
106. Come giocatore al governo, voglio che i partiti IA all'opposizione presentino la sfiducia solo quando passerebbe, in apertura con il voto in chiusura, così che ho un turno per reagire e niente sfiducie di bandiera.
107. Come giocatore al governo, voglio che l'opposizione IA porti proposte-imbarazzo e referendum abrogativi che vincerebbero, così che anche governando subisco le leve simmetriche.
108. Come giocatore, voglio che nascano partiti di protesta in zone dell'asse scoperte e con astensione alta, e scissioni quando la posizione percepita di un partito si allontana di un punto dalla dichiarata (anche il mio), così che il sistema dei partiti si muove.
109. Come giocatore, voglio che un partito IA senza seggi per due elezioni di fila si sciolga e che i partiti non superino mai dieci, così che il sistema resta leggibile.

### Elettorato e sondaggi

110. Come giocatore, voglio che l'elettorato sia fatto di 360 gruppi (6 condizioni × 3 fasce di reddito × 20 regioni), ciascuno con la sua distribuzione sull'asse, così che le misure colpiscono persone riconoscibili.
111. Come giocatore, voglio un sondaggio pubblico gratuito a ogni apertura, da 3-4 istituti inventati, con incroci per condizione, fascia e regione e un errore che cresce nei sottocampioni piccoli, così che leggo l'elettorato senza vederne lo stato vero.
112. Come giocatore, voglio che i sondaggi condividano un errore comune che si rivela solo al voto, così che fidarsi ciecamente della media è un rischio.
113. Come giocatore, voglio cliccare su un gruppo e vedere tre elettori campione inventati, ricavati dal sondaggio, con il motivo principale del loro voto, così che i numeri hanno una faccia.
114. Come giocatore, voglio che il giudizio dei gruppi sul governo si accumuli turno dopo turno, pesi più i peggioramenti, sbiadisca col tempo e riparta da zero per un partito che entra al governo, così che governare logora e la memoria degli elettori è credibile.
115. Come giocatore, voglio che la colpa di ogni interesse vada a chi ne risponde (reddito primario e prezzi al Presidente del Consiglio, disoccupazione e trasferimenti a Lavoro e Welfare, imposte e impiego pubblico a Economia e Finanze, servizi al loro dicastero), così che la spartizione dei dicasteri è anche una scelta su chi si prende la colpa.
116. Come giocatore, voglio vedere accanto al reddito reale di ogni fascia la crescita attesa e una voce nell'esito quando delude («reddito +0,3%, atteso +1%: giudizio in calo»), così che capisco perché anni piatti dopo anni buoni costano consenso.
117. Come giocatore, voglio che la posizione percepita del mio partito scivoli verso le misure che firmo o voto, così che le mie scelte mi spostano sull'asse anche se non lo dichiaro.
118. Come giocatore, voglio che l'affluenza sia endogena, così che mobilitare e deludere contano.

### Informazione imperfetta e cassa del partito

119. Come giocatore, voglio ricevere i dati economici come prima stima con errore, riveduta il turno dopo con la revisione nel registro, così che governo con i numeri in ritardo come i governi veri.
120. Come giocatore, voglio indicatori di servizio esatti ma con un turno di ritardo, e spread, seggi, voti e cassa esatti, così che so quali numeri sono certi.
121. Come giocatore, voglio vedere quanto ogni grandezza è vicina alla soglia di una situazione, ma non la probabilità di scandali ed emergenze, così che le crisi lente si vedono e quelle improvvise no.
122. Come giocatore, voglio una cassa del partito alimentata dal 2 per mille, dalle donazioni e da una dote alla fondazione, che non va mai sotto zero, così che ho una risorsa mia separata dallo Stato.
123. Come giocatore, voglio comprare con la cassa, al massimo una volta per tipo a turno, un sondaggio privato, il test di una misura e il centro studi (stima migliore del trimestre e previsione a 4 turni con intervallo), così che posso pagare per sapere di più.
124. Come giocatore, voglio che ogni acquisto di informazione tolga denaro alla campagna, così che sapere e convincere competono.

### Campagna elettorale

125. Come giocatore, voglio fare campagna solo nel turno prima del voto (anche al turno zero e dopo uno scioglimento), con le stesse leve al governo e all'opposizione, così che la campagna sposta voti al margine e il governo resta il fattore principale.
126. Come giocatore, voglio riposizionarmi gratis di mezzo punto al massimo, sapendo che la percepita fa metà strada e che allontanarmi da essa avvicina la scissione, così che il riposizionamento è una scelta con un rischio.
127. Come giocatore, voglio spendere la cassa su fino a 3 bersagli (condizione o regione), ciascuno con una leva fra mobilitare, persuadere e convincere, con rendimenti decrescenti e un'anteprima a fasce, così che la campagna è un'allocazione e non un pulsante.
128. Come giocatore, voglio fare fino a 2 promesse su disposizioni, che mi danno persuasione subito e valence al voto dopo se mantenute (meno di quanta ne tolgono se mancate), così che promettere è un debito.
129. Come giocatore, voglio vedere le casse dei rivali in cifre e, nell'esito, quanto e dove hanno speso, così che la campagna degli altri non è un mistero.

### Eventi e situazioni

130. Come giocatore, voglio che ogni evento citi la sua causa nel modello (grandezza, valore e soglia, o livello di rischio), così che niente mi cade addosso senza ragione.
131. Come giocatore, voglio al massimo un evento puntuale nuovo per turno, mostrato solo in apertura, con 2-3 opzioni chiuse e un default meno irreversibile, così che gli eventi non soffocano il turno.
132. Come giocatore, voglio situazioni a soglia con isteresi (recessione, ondata di criminalità, collasso delle liste d'attesa, emergenza carceri, crisi energetica, ondata di sbarchi) che aprono un evento quando si accendono e poi pesano finché non scendono sotto la soglia di spegnimento, così che le crisi lente si combattono con atti e leggi ordinari.
133. Come giocatore, voglio emergenze naturali per regione (terremoti, alluvioni e frane, incendi e siccità) con rischio dai dati reali e gravità ridotta dalla prevenzione, così che investire in infrastrutture protegge.
134. Come giocatore, voglio che uno scandalo su un mio ministro mi faccia scegliere fra difenderlo (valence giù, sfiducia individuale possibile) e chiedergli le dimissioni, così che gli scandali sono dilemmi.
135. Come giocatore, voglio che gli shock economici oltre due deviazioni standard diventino eventi con la scelta fra aiuti per decreto e niente, e che lo shock energetico apra la clausola nazionale per l'energia, così che le crisi esterne chiedono una risposta.
136. Come giocatore, voglio che un evento con vittime si presenti in tono grave, senza testata, battute né illustrazioni, così che il gioco non scherza sulle tragedie.

### Mappa delle 20 regioni

137. Come giocatore, voglio una mappa SVG delle 20 regioni a tinte piatte con un livello alla volta (consenso, disoccupazione, reddito, affluenza, pericolosità naturale), così che leggo il paese a colpo d'occhio.
138. Come giocatore, voglio dopo il voto il livello «risultato del voto» col primo partito per regione e i seggi del Senato, così che vedo la geografia della vittoria.
139. Come giocatore, voglio segni fissi sulla mappa per atti e cantieri regionali in corso o finiti e per le emergenze aperte, così che vedo dove sto spendendo.
140. Come giocatore, voglio una scheda della regione al clic con gruppi, elettori campione, seggi, forbice del sondaggio e disposizioni territoriali in vigore, così che posso scendere nel dettaglio.
141. Come giocatore, voglio scegliere una regione sulla mappa quando una decisione la chiede (bersaglio di campagna, dirottamento, emergenza), e vedere il perimetro evidenziato nell'anteprima di una disposizione territoriale, così che la mappa è uno strumento e non un poster.
142. Come giocatore, voglio tre disposizioni territoriali (decontribuzione per il Mezzogiorno, credito d'imposta ZES unica, autonomia differenziata), così che il divario Nord-Sud è una leva politica.

### Salvataggi, eredità, albo

143. Come giocatore, voglio che il gioco salvi da solo all'apertura di ogni turno, così che non perdo mai la partita.
144. Come giocatore, voglio tornare all'apertura di un turno passato dal registro, cancellando i turni successivi, così che posso rimediare a un errore o a un bug di bilanciamento senza gestire slot.
145. Come giocatore, voglio che dopo un riavvolgimento le stesse scelte diano lo stesso esito, così che riavvolgere compra conoscenza e non fortuna.
146. Come giocatore, voglio chiudere l'app a metà turno e riprendere esattamente dalle scelte che avevo fatto, così che non devo finire un turno per smettere.
147. Come giocatore, voglio più partite in corso insieme, in una schermata con nome, scenario, turno e data dell'ultimo salvataggio, così che posso provare strade diverse.
148. Come giocatore, voglio che un salvataggio di una versione incompatibile sia rifiutato con un messaggio chiaro senza essere cancellato, così che so perché non si apre.
149. Come giocatore, voglio a fine partita un'eredità con anni al governo, leggi ancora in vigore, indicatori contro il turno zero e numero di riavvolgimenti, così che la partita ha un bilancio.
150. Come giocatore, voglio che dopo la sconfitta per seggi l'eredità resti in bozza con «torna a un turno» ancora possibile, così che l'ultimo voto perso è riavvolgibile come gli altri.
151. Come giocatore, voglio un albo delle partite concluse, leggibile anche quando una versione nuova non apre più i salvataggi, con la serie dei tre indicatori sintetici e i risultati di ogni elezione, così che conservo la storia delle mie partite.
152. Come giocatore, voglio abbandonare una partita con un comando confermato, che produce subito l'eredità, così che chiudere l'app non finisce mai una partita per sbaglio.

### Aspetto, leggibilità, accessibilità

153. Come giocatore, voglio l'esito del turno come prima pagina di «Il Pungolo», con occhiello serio col fatto e il numero e titolo spiritoso sotto, così che leggere le conseguenze è un piacere.
154. Come giocatore, voglio il listino dei ministri come un borsino con caricatura, voto con freccia e motto, e che fermare o dirottare giri la carta del ministro, così che i ministri sono personaggi.
155. Come giocatore, voglio i tre indicatori sintetici come tracciati a caselle che avanzano a scatti, così che i numeri chiave si leggono senza grafici.
156. Come giocatore, voglio l'anteprima della misura come un mobile appeso che oscilla solo a un mio gesto, mai in loop, così che il movimento comunica e non distrae.
157. Come giocatore, voglio uno schema dello Stato come vista secondaria (chi tocca cosa nel turno), così che vedo le connessioni fra istituzioni.
158. Come giocatore, voglio testo minimo di 12 px a scala 1×, una scala della UI regolabile, velocità 1×/2×/istantaneo e un interruttore del movimento, così che il gioco si legge e non stanca.
159. Come giocatore, voglio che lo scherzo stia nei testi e nei personaggi e mai nei numeri, e che nessun personaggio sia un politico reale, così che il gioco è giocoso ma onesto.

### Mario come autore dei dati e bilanciatore

160. Come autore dei dati, voglio catalogo degli atti, disposizioni, dicasteri con i loro ruoli, eventi, manopole e scenario in file di testo letti dal core, così che cambio il contenuto senza ricompilare.
161. Come autore dei dati, voglio che un file che lascia un ruolo dei dicasteri senza titolare o lo dà a due dicasteri sia rifiutato al caricamento col nome del ruolo, così che un errore nei dati non produce una partita silenziosamente sbagliata.
162. Come autore dei dati, voglio che ogni manopola abbia valore di partenza, intervallo e ticket d'origine, così che so da dove viene ogni numero da tarare.
163. Come autore dei dati, voglio che una modifica ai file valga solo per le partite nuove, perché ogni partita copia i suoi dati nel salvataggio, così che non rompo le partite in corso.
164. Come bilanciatore, voglio un binario CLI che gioca partite senza finestra con tre strategie automatiche (inerte, media, riformatrice) su molti semi, così che misuro il costo del governare contro i bersagli.
165. Come bilanciatore, voglio che lo stesso seme e gli stessi comandi diano la stessa partita sullo stesso binario, così che riproduco ogni bug e ogni regressione.
166. Come bilanciatore, voglio i testi del gioco esternalizzati dal codice, così che li correggo senza toccare la logica.

## Implementation Decisions

### Architettura (11)

- **Stack**: Tauri 2.12; core di simulazione in Rust; UI in Svelte 5 + TypeScript + Vite (template ufficiale di Tauri); mappa in SVG a tinte piatte, senza WebGL. Build solo per il Mac, versioni bloccate, nessuna CI di notarizzazione.
- **Core puro** (`sim-core`): nessun I/O, nessun thread. Il guscio Tauri legge e scrive i file e li passa al core come testo; il core resta compilabile in WASM, così web e altre piattaforme non sono escluse.
- **Confine core ↔ UI**: comandi tipizzati verso il core; una vista serializzata verso la UI una volta per fase del turno; tipi TypeScript generati dai tipi Rust. La UI non calcola regole di gioco: chiede al core anche anteprime e proiezioni.
- **Vista e stato vero**: il core tiene lo stato vero e consegna alla UI solo la vista del giocatore, già filtrata dall'informazione imperfetta (prime stime, sondaggi, fasce dell'intesa avversaria, agende avversarie nascoste). Lo stato vero è leggibile solo dal binario CLI, per il bilanciamento. Una regola di visibilità sta nel core, mai nella UI.
- **Secondo binario CLI** sullo stesso crate: partite senza finestra, strategie scriptate, molti semi in serie.
- **Numeri**: interi per euro, seggi, voti ed elettori; `f64` per Leontief, IS-Phillips e logit, arrotondati a interi quando toccano i conti pubblici. Riproducibilità garantita solo sullo stesso binario: stesso seme + stessi comandi = stessa partita.
- **Caso**: un seme fisso per partita, da cui nasce un flusso del generatore per sottosistema (economia, elettorato, parlamento, eventi, sondaggi, partiti IA), così un cambio in uno non sposta gli altri; lo stato fa parte dello snapshot. Ogni estrazione (franchi tiratori, eventi, shock, rumore dei sondaggi e delle scelte IA) passa da lì.
- **Modding solo dati**: tutto il contenuto e tutte le manopole stanno in file di testo leggibili a mano; il formato è una scelta di strumento del piano. Testi del gioco esternalizzati, solo in italiano.

### Le quattro cuciture (30)

Vincoli su come si costruisce la v1; sono le sole concessioni a sistemi futuri.

1. **Motore economico in blocchi separati**, con l'ordine di aggiornamento esplicito (sotto). Ci si appoggiano uscita dall'euro, banche, dazi ed embarghi.
2. **Versione dello schema nel salvataggio**, con rifiuto dei salvataggi di schema diverso.
3. **Seggi per regione calcolati dalla popolazione censita**, mai scritti in una tabella; premio e tetto espressi in quota dei seggi.
4. **Dicasteri nei dati, ruoli compresi.** L'elenco dei dicasteri (nome, budget, atti, disposizioni) sta nei file insieme ai **ruoli del dicastero** che il codice altrimenti darebbe per nome: colpa di disoccupazione e trasferimenti, colpa di imposte e impiego pubblico, colpa di ciascun indicatore di servizio. La scelta della regione bersaglio spetta a due dicasteri, quindi non è un ruolo unico ma un campo del dicastero nei dati (sì/no). Estensione dichiarata della stessa regola, non decisa nel ticket: anche il ministro che firma la legge di bilancio e la cui capacità pesa sulla credibilità, quindi sullo spread (03), è un ruolo nei dati invece di un nome nel codice. Ogni ruolo ha un solo dicastero; un dicastero può averne più d'uno. Un file che lascia un ruolo orfano o doppio si rifiuta al caricamento con il nome del ruolo. Reddito primario, prezzi e fondo di riserva restano del Presidente del Consiglio, che non è un dicastero, e non stanno nei dati.

Nessun altro campo, fase o famiglia di eventi riservati.

### Modulo dati

- Carica e valida: dicasteri e ruoli; catalogo dei 40 atti; 35 disposizioni di policy (32 dei dicasteri più 3 territoriali) e le disposizioni della legge elettorale; eventi; manopole; scenario di base; gruppi tarati; partiti e personaggi; testi.
- **Schema di un atto del ministro** (16, asset `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md`): nome e motto; dicastero e casella della griglia 2×2 (rapido o lento × sinistra o destra); posizione scritta a mano; forma puntuale o cantiere (N turni); costo in quote trimestrali del margine di riferimento; 1-2 effetti sulle grandezze della riga del dicastero (03) con intensità in unità vere, ritardo e forma del ritardo (impulso che decade con emivita, oppure stock con vita utile); regione bersaglio per gli 8 atti di Infrastrutture e Trasporti e Sviluppo economico; prerequisito facoltativo (una disposizione a un valore). Mai effetti diretti su posizioni, valence o intesa. La spesa va sempre in domanda finale col vettore di settori del dicastero.
- **Schema di una disposizione** (16, 22, 24, 27): nome; dicastero proprietario o «fuori dicastero» (legge elettorale, autonomia differenziata); gradini ordinati o regimi con nome, con «non istituita» per le voci nuove; scala valore → posizione; partenza (valore in vigore nel 2026, legge reale d'origine, valore precedente); **valore dopo l'abrogazione** di tipo *precedente* (default) o *fisso* (solo il reddito minimo → «non istituito» nella v1); referendabile sì/no (escluse le 7 fiscali e contributive: forfettario, cuneo fiscale, detrazione edilizia, accise, incentivo agli investimenti, decontribuzione per il Mezzogiorno, credito ZES unica); **valore in arrivo** facoltativo col turno da cui vale; perimetro per le territoriali; effetti dal motore dove la regola esiste (pensioni, sussidi, fisco, u*), altrimenti tabella valore → effetto; saldo annuo che pesa sul tetto UE.
- **Valori di partenza** dalla ricerca (22, 24): correzioni già decise (intercettazioni «limitate», concessioni autostradali «a gara», cuneo con precedente 0, incentivo agli investimenti 0/10/20/40* con valore in arrivo 0 dal turno ottobre-dicembre 2028, nucleare «vietato» con valore in arrivo «consentito» dal turno ottobre-dicembre 2027, rinnovabili «ordinarie*»/«accelerate»). Se il valore precedente reale non è un gradino o non è noto, si prende il gradino più vicino; se coincide col vigente, la disposizione non è bersaglio di referendum finché la partita non la cambia.
- **Manopole**: ogni valore provvisorio con valore di partenza, intervallo e ticket d'origine. κ (deriva del logoramento) esiste come manopola assente di partenza, da accendere solo se la taratura manca i bersagli.
- **Limiti di taratura**: ogni ingresso dello scenario dichiara l'intervallo in cui il modello è tarato; un file fuori intervallo si rifiuta col nome del valore.

### Modulo scenario (20, 21, 23)

- Lo scenario contiene **solo ingressi**; al caricamento il motore ricalcola i derivati (interessi = costo medio × debito, spread, disavanzo, bilancio 2027 ricondotto al tetto UE, sondaggio, parlamento dal voto simulato). La valence dei partiti è un ingresso.
- **Struttura fissa**: data del turno zero, istituzioni, 10 dicasteri, 20 regioni, 360 gruppi, sette posti dei partiti IA con vocazioni e regole di movimento di default, governo uscente di centrodestra e destra.
- **Stato base** al 2026T2 portato a inizio ottobre con gli indicatori mensili di luglio-settembre: debito 136,7% del PIL, spread 79-89 pb (la curva si tara per riprodurli), IPCA 4,2%, disoccupazione 5,6%; budget dei dicasteri da COFOG 2024 di tutte le AP, funzionamento = salari + acquisti (65% del totale), margine reale ~1,6%; FIGARO 2024 e mix regionale A*10 2023. Lo stato iniziale non è un equilibrio.
- **Cursori** e cosa ricalcolano: tabella del ticket 21. La disoccupazione sposta anche u*; la ricchezza tiene ferme le soglie in euro. Budget dei singoli dicasteri, aliquote, mix regionale e divario Nord-Sud solo nel file.
- **File su misura** = differenze dalla base (valori dei cursori più sostituzioni puntuali). Lo scenario di base è immutabile e si spedisce col gioco. Il seme appartiene alla partita, non allo scenario.
- **Disposizioni cambiate dall'editor**: il valore nuovo si attribuisce a una legge del governo uscente del 2026 e il precedente diventa il valore dello scenario di base.
- **Avvisi**: nessuna coalizione col giocatore sopra il 42% atteso da qualche posto; situazione o crisi già accesa al turno zero.
- **Riepilogo dal vivo** con cifre esatte e sondaggio atteso senza rumore: l'informazione imperfetta comincia con la partita.
- **Taratura dei partiti**: la valence dei sette partiti IA si tara una volta perché il sondaggio del turno zero riproduca le intenzioni di voto reali di ottobre 2026 sommate per posto (collocazioni CHES). La corrispondenza fra partiti veri e posti si fa offline e non entra nei dati del gioco.
- **Vincibilità**: lo scenario di base è tarato perché almeno una coalizione col giocatore superi il 42% atteso da ogni posto.

### Motore economico (07, 02, 23)

Blocchi separati, aggiornati in quest'ordine dentro il passo «motore» della chiusura: misure → finanza pubblica e spesa obbligatoria → domanda finale per settore → Leontief (produzione e prezzi) → IS/Phillips → Okun per regione → redditi dei gruppi e panieri → tasso BCE → SFC (stock e controllo di chiusura) → spread, rating, regole UE.

- **Produzione**: Leontief nazionale a 10 settori produttivi (NACE A*10) su FIGARO aggregato, matrice A fissa salvo eventi, modello dei prezzi per l'inflazione importata; ogni regione ha il suo mix e il suo gap è la media dei gap di settore pesata col mix.
- **Congiuntura**: IS-Phillips di Rudebusch-Svensson trimestrale, backward-looking, coefficienti ricalibrati su Italia/area euro dove esistono; impulso fiscale come termine proprio con moltiplicatore dalla letteratura; shock stocastici (domanda estera, energia, tasso BCE, produttività) col seme della partita, ampiezza dai residui storici italiani.
- **BCE**: regola di Taylor sull'area euro con l'Italia pesata per quota di PIL e il resto esogeno; aspettative d'inflazione ancorate al 2% con peso fisso.
- **SFC** a 4 settori (famiglie, imprese, Stato, estero), senza banche, con il controllo «la somma fa zero». Estero come domanda esogena con elasticità ai prezzi relativi. Fondi di coesione come entrata esogena sospendibile dalla PDE.
- **Spread**: quadratica di De Grauwe-Ji più premio di sfiducia; piatto finché debito, disavanzo e credibilità del ministro dell'Economia reggono, convesso oltre. TPI che azzera il premio di sfiducia (non i fondamentali) se valgono i suoi criteri; un giudizio di «nessuna azione efficace» lo toglie. Tasso medio aggiornato sulla quota che scade (vita media 7,8 anni). Oltre una soglia: aste deserte → evento con MES (default; condizionalità che impone valori di disposizioni e tetti di spesa, intesa degli alleati in calo, apre l'OMT) o ristrutturazione (taglio del debito, perdite di reddito per i gruppi che detengono titoli, rating a terra con memoria lunga, fuori dal TPI).
- **Regole UE**: `tetto_spesa_netta[anno]` (1,6 / 1,9 / 1,7 / 1,5 per il 2026-2029), conto di controllo con soglie 0,3% annuo e 0,6% cumulato, PDE come macchina a stati (partenza: sospesa, non chiusa), clausole nazionali per difesa ed energia, controllo annuale nel turno aprile-giugno, piano strutturale riveduto nel primo anno di un governo nuovo.
- **Lavoro**: Okun per regione con u* propria, coefficiente per macroarea dove la stima esiste; u* con isteresi, spostata con ritardo dalle riforme del lavoro, dalle politiche attive e dallo stock regionale di infrastrutture e investimenti.
- **Redditi e prezzi**: reddito disponibile per i 360 gruppi = reddito primario di condizione e regione × moltiplicatore di fascia − imposte + trasferimenti; tre panieri (uno per fascia) dai prezzi dei 10 settori; Gini derivato dai gruppi. L'evasione si ripartisce per condizione; il salario minimo entra nel reddito primario della fascia bassa dei dipendenti privati.
- **Fisco**: IRPEF coi suoi scaglioni reali, IVA ordinaria, IRES, aliquota contributiva, ognuna con la sua elasticità del reddito imponibile. Detrazioni e regimi speciali sono disposizioni, non manopole del bilancio.
- **Spesa dei dicasteri**: quota fissa di salari e di acquisti nel funzionamento; tagliare i salari blocca il turnover e frena i rinnovi, tagliare gli acquisti pesa sul servizio e sui settori fornitori. Erosione nominale: budget, fondo di riserva, scaglioni e soglie, salario minimo, pensione minima e soglie dei sussidi restano fermi finché una legge non li cambia; si indicizzano da sole solo le pensioni in essere.
- **Spesa obbligatoria**: pensioni (retributivo, contributivo in vigore, quota base universale; età, indicizzazione, aliquota, pensione minima), sussidi (assicurativo o universale; importo, durata, soglia), interessi. Cambio di regime solo per le coorti future, pro rata; unica leva retroattiva l'indicizzazione delle pensioni in essere. Piramide nazionale per coorti (ISTAT al 1/1/2026), che alimenta anche il passaggio annuale dei gruppi verso la pensione.
- **PIL potenziale**: trend spostato poco e con ritardi di anni da infrastrutture, capitale umano, durata dei processi, prezzo dell'energia, demografia.
- **Disposizioni territoriali** (27): decontribuzione (costo del lavoro dei dipendenti privati in forza nel Mezzogiorno), credito ZES (investimenti privati nelle 10 regioni, intensità ridotta in Abruzzo, Marche e Umbria), autonomia differenziata (le 4 regioni del perimetro trattengono gettito; la spesa per le materie trasferite segue il gettito della regione invece della popolazione).

### Elettorato (06, 14, 17, 29)

- **360 gruppi** con quota della popolazione e istogramma sui 7 punti dell'asse. Le quote si muovono ogni turno con lavoro e reddito (chi cambia gruppo porta con sé la sua distribuzione) e ogni anno con l'età.
- **Voto logit** per gruppo e punto dell'asse: −β·distanza² dalla posizione percepita del partito, valence, giudizio retrospettivo (solo per i partiti di governo), fedeltà al voto precedente; astensione come alternativa con utilità propria, quindi affluenza endogena.
- **Interessi**: il reddito disponibile reale della fascia, deflazionato col suo paniere, più gli indicatori che non sono denaro della tabella del ticket 17 (una riga per condizione, pesi che sommano a 1). Ogni indicatore si misura in deviazioni standard della sua variazione trimestrale storica e si legge alla risoluzione che ha.
- **Aspettativa**: il reddito reale si giudica contro la sua crescita media degli ultimi ~8 turni, alzata dalle promesse.
- **Giudizio**: `g ← λ·g + Σ_k w_k · f(Δ_k)` per ogni gruppo e partito di governo, f asimmetrica (ρ ≈ 1/3), emivita ~4 turni; da zero per chi entra al governo.
- **Colpa**: quota fissa al partito del Presidente del Consiglio; il resto per ruolo del dicastero (reddito primario e prezzi al Presidente del Consiglio; disoccupazione, pensioni, sussidi, reddito minimo al ruolo di Lavoro e Welfare; imposte, salario e occupazione pubblici al ruolo di Economia e Finanze; ogni indicatore di servizio al suo dicastero). Eccezione: nella regione bersaglio di un atto, e nel perimetro di una disposizione territoriale di dicastero, la variazione dovuta all'atto o alla disposizione va al partito del dicastero, che il motore conosce esatta perché l'atto entra come termine additivo sulla domanda della regione.
- **Posizioni**: ancora di Friedkin-Johnsen verso la posizione di partenza del gruppo, più spinte di eventi (criminalità e sbarchi, verso destra nella regione colpita) e di campagna, più rumore. **Posizione percepita** dei partiti che scivola verso le misure firmate o votate e fa metà strada quando la dichiarata si sposta.
- **Elettori campione**: tre per gruppo, a richiesta, generati col seme e ricavati dal sondaggio, mai dallo stato vero; il motivo mostrato è il termine che pesa di più nell'utilità stimata.

### Elezioni e riparto (01, 13, 20)

- **Riparto puro**: voti per lista e per regione + legge elettorale in vigore + seggi per regione → seggi per lista in ciascuna Camera. Deterministico, senza stato.
- **180/2026** come impostazione pronta: Camera 400 (8 Estero, 1 Valle d'Aosta, 7 Trentino-Alto Adige, 384 = 70 premio + 314 proporzionali), Senato 200 (4, 1, 6, 189 = 35 + 154); soglie di coalizione 10% con una lista al 3%, di lista 3%, ripescaggio della lista più votata sotto soglia in coalizione, al Senato in alternativa il 20% in una regione; premio 70/35 solo col 42% in entrambe le Camere allo stesso soggetto; tetti 220 (Camera, riparto nazionale) e 113 (Senato, riparto regionale con sottrazione dei seggi in eccesso). Premio e tetto in quota dei seggi.
- **Seggi premio fra gli alleati**: divisi nell'accordo pre-voto, di default in proporzione ai sondaggi; il giocatore può cedere posti delle liste comuni pagandoli con seggi propri.
- **Menu della legge elettorale** (13): disposizioni fuori dicastero con i gradini del ticket 13; fissi il Senato regionale e il premio unico; soglia del premio almeno 40%; combinazioni impossibili spente; impostazioni pronte 180, proporzionale al 3%, proporzionale puro. Valori precedenti al turno zero: quelli del Rosatellum.
- **Proiezione** dei seggi per la linea e per l'anteprima: sul sondaggio pubblico del trimestre, con le coalizioni dell'ultimo voto; i partiti nati dopo corrono da soli.
- Il **voto** si calcola nella chiusura, dopo il motore, col logit sui gruppi.

### Governo, ministri e coalizione (03, 04, 05)

- **Ministro**: capacità 1-5 (moltiplica l'intensità e divide il ritardo; ridotta nel rodaggio), posizione (partito più scarto personale; i tecnici solo la propria), obiettivo (un indicatore del dicastero a una soglia), soddisfazione 0-100 che torna piano verso una base. Cause di calo: atto fermato o dirottato, proposta respinta, norma firmata contro la propria posizione, budget tagliato, indicatore di servizio in calo; sale a obiettivo raggiunto. Minaccia sotto 40, dimissioni sotto 25 a fine turno.
- **Agenda**: fra gli atti che il margine paga, il punteggio più alto di avvicinamento all'obiettivo + vicinanza alla posizione + piccolo rumore (+ bisogno e voti pesati dalla vocazione per la regione bersaglio). Usa l'effetto nominale scritto, non una simulazione.
- **Proposte**: tre inneschi; per la riforma, la disposizione del dicastero più lontana dalla posizione del ministro portata al valore più vicino se la distanza è almeno 2, o lo sblocco del prerequisito di un atto utile all'obiettivo. Legge elettorale e autonomia differenziata le propongono solo il Presidente del Consiglio o il governo avversario, senza ministro proponente.
- **Successione**: rosa di 3 dal partito (o 3 tecnici, o passaggio a un partito), interim di un turno, rodaggio di un turno.
- **Coalizione pre-elettorale**: dichiarata nel turno prima del voto, obbligatoria con la 180, facoltativa senza premio né soglie di coalizione (impegno politico che costa intesa e sondaggi se rotto). Ogni partito entra nella coalizione col valore atteso più alto (seggi attesi, probabilità di governare, distanza, intesa), letto sugli stessi sondaggi rumorosi del giocatore.
- **Spartizione**: richiesta di Gamson `0,07 + 0,79 × quota dei seggi portati alla maggioranza`, in punti di peso dei dicasteri (provvisori: Economia 3, Interno 2, Esteri e Difesa 2, gli altri 1). Offerta sotto la richiesta → intesa di partenza più bassa, sopra → più alta. Tecnici nella quota del partito del giocatore, senza altro tetto. Solo i dieci dicasteri si spartiscono.
- **Intesa** 0-100 per alleato: cala con la soddisfazione media bassa dei suoi ministri, le dimissioni chieste a un suo ministro, le norme lontane dalla sua posizione (più in fretta per vocazione identitaria), la quota sotto la richiesta, il calo nei sondaggi al governo, la rottura dell'impegno, i seggi persi per una legge elettorale, le promesse su cui la sua linea sarebbe no; torna piano verso una base. Ultimatum sotto 40, uscita sotto 25 a fine turno.
- **Uscita** → dimissioni in blocco dei suoi ministri → rimpasto con Gamson ricalcolata se la maggioranza regge in entrambe le Camere, altrimenti crisi di un turno (ordinaria amministrazione) e sfiducia a fine turno se la maggioranza non torna.
- **Sfiducia** palese a blocchi e deterministica: la coalizione vota no tranne gli alleati in ultimatum che dalla caduta guadagnano di più; gli altri partiti sì. La sfiducia individuale ha le stesse firme e passa con la stessa condizione.
- **Dimissioni del giocatore**: il Capo dello Stato, istituzione a regole, cerca una maggioranza di partiti contigui senza il partito del giocatore in entrambe le Camere (ribaltone); altrimenti scioglie e si vota il turno dopo. Niente appoggio esterno né governo di minoranza.

### Parlamento (08, 12, 13, 24)

- **Linea** di ogni partito: per una legge di riforma, sì se il valore nuovo è più vicino alla sua posizione di quello in vigore; per la legge elettorale, sì se guadagna seggi attesi; più l'intesa per gli alleati e un malus fisso per l'opposizione (che cresce nell'ultimo anno di legislatura per i partiti IA). La posizione della misura è la media delle posizioni dei valori nuovi.
- **Franchi tiratori** estratti col seme, con probabilità crescente con la distanza e con l'intesa bassa; forbice visibile prima, conteggio esatto dopo; raddoppiati nei due sensi nel voto finale segreto della legge elettorale alla Camera.
- **Iter**: decreto con innesco (evento aperto, variazione di fondi; con crisi del debito aperta anche una riforma), mai su legge elettorale, norme costituzionali o bilancio; conversione a fine dello stesso turno, se decade cessano gli effetti normativi e il denaro speso resta speso, non si riemana. Legge ordinaria in 2 turni senza navetta. Fiducia su leggi e conversioni (non sulle costituzionali), che salta gli emendamenti, vota a blocchi e risparmia un turno; costo d'intesa doppio sulla legge elettorale; se respinta, il governo si dimette. Bilancio nel turno ottobre-dicembre, voto a fine turno, esercizio provvisorio, poi dimissioni. Norma costituzionale con art. 138 e referendum confermativo senza quorum.
- **Calendario**: 3 posti per le ordinarie, 1 per una costituzionale, decreti in precedenza, bilancio che occupa tutto, un posto al trimestre per l'opposizione a rotazione.
- **Referendum abrogativo**: una disposizione per quesito (la legge che ne ha fissato il valore in vigore); firme in un turno; un referendum pendente alla volta; richiesta entro il turno luglio-settembre, voto nel turno aprile-giugno dell'anno dopo; niente richieste nell'ultimo anno di legislatura; lo scioglimento sospende un referendum indetto; quorum della maggioranza degli aventi diritto con l'astensione endogena; il governo può invitare all'astensione. Esito: valore dopo l'abrogazione, eventuale valore in arrivo cancellato, gradino bloccato fino alla legislatura successiva per tutti, perdita di valence del governo (o del proponente senza quorum).
- **Capo dello Stato** promulga sempre; niente rinvio, niente Corte.

### Partiti IA (18, 26)

- Regole a punteggio che riusano le formule del giocatore (valore atteso, punteggio dell'agenda, linea), piccolo rumore col seme, una riga di spiegazione per ogni mossa; niente ricerca in avanti.
- **Vocazione** (identitaria agli estremi, governista al centro, mista per gli altri) e **regola di movimento** (Sticker agli estremi, Aggregator a sinistra e destra, Hunter a centrosinistra e centrodestra, Governator satisficing al centro), entrambe nei dati dello scenario.
- **Posizione dichiarata** mossa una volta a legislatura nel turno prima del voto, mezzo punto al massimo, mai oltre il vicino.
- **Leader**: porta la valence; cambia dopo la perdita del governo o un calo di almeno X punti al voto; il nuovo riparte dalla valence base con una breve luna di miele.
- **Governo avversario**: nasce col valore atteso, guidato dal partito più grande, spartizione = richiesta di Gamson esatta, niente tecnici. Programma pubblico alla nascita (le disposizioni più lontane dalla posizione della coalizione pesata per seggi, ciascuna al valore più vicino, in ordine di distanza), che riempie i 3 posti del calendario; può includere la legge elettorale se gli rende almeno N seggi. Bilancio: anno prima indicizzato e ricondotto al tetto UE più le richieste degli alleati in ultimatum. Eventi: sempre il default. Condotta: accoglie un emendamento se senza quell'alleato la forbice include la sconfitta o se l'alleato è sotto 40; pone la fiducia quando la forbice include la sconfitta e nessun alleato è in ultimatum; non ferma né dirotta mai le agende; in crisi allarga a un partito contiguo che ci guadagna, altrimenti si dimette. Voto anticipato se il valore atteso del voto supera quello del restare oltre un margine, mai nel primo anno né a meno di 2 turni dalla scadenza.
- **Opposizione IA contro il giocatore**: sfiducia solo quando passerebbe (presentata in apertura, votata in chiusura), sfiducia individuale sul ministro col giudizio peggiore fra i gruppi e mai dell'alleato in ultimatum; proposta-imbarazzo nel posto riservato (disposizione e valore che massimizzano i seggi della maggioranza la cui linea sarebbe sì); referendum abrogativo sulla legge la cui abrogazione vince nel sondaggio e raggiunge il quorum stimato, col margine più ampio.
- **Vita dei partiti**: partito di protesta (al massimo uno a legislatura, nel turno prima del voto, in un tratto con almeno X% degli elettori, lontano almeno 1 punto dal partito più vicino, astensione sopra la media; leader nuovo in luna di miele, Hunter, identitario, dote); scissione a 1 punto fra percepita e dichiarata con minaccia a mezzo punto, anche a metà legislatura, con una quota fissa dei seggi e della cassa, uscita dalla coalizione se il partito d'origine governava; scioglimento dopo due elezioni senza seggi; tetto di 10 partiti, niente fusioni.

### Eventi e situazioni (15, 19)

- **Situazioni** a soglia con isteresi, nazionali: le sei del ticket 15. Etichette sopra grandezze che il motore muove già, senza effetti materiali in più (salvo le spinte di posizione previste); aprono un evento solo nel turno in cui si accendono; soglia di accensione visibile accanto alla grandezza.
- **Eventi puntuali** con rischio per turno calcolato dallo stato ed estratto col seme; al massimo uno nuovo per turno (vince il più grave; uno shock escluso succede comunque nel motore e va nell'esito col suo default, uno scandalo o un'emergenza esclusi non succedono). Situazioni e crisi del debito fuori dal tetto. Nascono in chiusura, si mostrano in apertura.
- **Forma**: causa citata per nome sulle grandezze; 2-3 opzioni chiuse; default = la meno irreversibile; decreto ammesso solo su disposizioni e fondi dei dicasteri coinvolti (la crisi del debito apre anche le riforme). Danni solo attraverso le grandezze; valence mossa solo da scandali e gestione delle emergenze.
- **Famiglie**: situazioni dei dicasteri; shock economici oltre 2σ (aiuti per decreto a debito o niente; per l'energia anche la clausola nazionale); crisi del debito (MES di default, o ristrutturazione); emergenze naturali per regione (fondo di riserva fino a capienza di default, decreto a debito, nessun intervento con malus; tono grave con vittime); scandali (rischio uguale per tutti, crescente con la corruzione; bersaglio un ministro o un leader IA, mai il giocatore).
- **Pericolosità naturale**: sisma da zone DPC ed episodi CPTI15 (0,37 l'anno a scala nazionale; nel repo solo i conteggi derivati), idro dagli esposti IdroGEO con frequenza nazionale come manopola (4 l'anno), incendi da EFFIS concentrati nel terzo trimestre; danni ~4-6 miliardi l'anno; prevenzione con k = 0,85 per il sisma (DM 58/2017), k di design per idro e incendi.
- Le scadenze a data fissa (controllo UE, bilancio, voto, finestra referendaria) sono il calendario del turno, non eventi.

### Informazione e cassa del partito (25)

- Prima stima rumorosa (errore tarato sulle revisioni reali, circa 1 punto annuo sul PIL) di PIL, disoccupazione, inflazione e disavanzo, riveduta il turno dopo; indicatori di servizio esatti con un turno di ritardo; spread, seggi, voti d'aula e cassa esatti. Il giornale dell'esito riporta la statistica ufficiale.
- Sondaggio pubblico: 3-4 istituti con effetto istituto, bias comune estratto una volta per legislatura e mai mostrato, errore di campionamento; incroci per gruppo.
- Al governo soddisfazione e intesa in cifre con le soglie 40/25; all'opposizione l'intesa avversaria in tre fasce.
- Anteprima: costo esatto; effetti a fasce con errore che cala con la capacità del ministro proponente.
- Cassa: 2 per mille sui voti dell'ultima elezione, donazioni proporzionali all'intenzione di voto vera, dote alla fondazione; mai sotto zero. Acquisti, al massimo uno per tipo a turno: sondaggio privato (meno errore di campionamento e d'istituto, stesso bias comune), test di una misura (precisione dei gruppi colpiti), centro studi (stima migliore del trimestre chiuso e previsione a 4 turni di PIL, disoccupazione, inflazione e debito sul percorso in corso, con intervallo che si allarga). I partiti IA hanno la stessa cassa ma non comprano informazione.

### Campagna elettorale (26)

- Solo nel turno prima del voto (a scadenza, dopo uno scioglimento, al turno zero); niente campagna per i referendum.
- Apertura: spostamenti dei partiti IA e nascita del partito di protesta (non al turno zero). Decisioni: riposizionamento gratuito di mezzo punto (percepita a un quarto), coalizione, fino a 2 promesse, fino a 3 bersagli. Chiusura: si spendono tutte le casse, gira il motore, si vota.
- Bersaglio = una condizione o una regione, con una leva: mobilitare (meno astensione fra i vicini, per quel voto), persuadere (valence verso il bersaglio, per quel voto), convincere (posizioni spostate verso la dichiarata, permanente ma riassorbito dall'ancora). Rendimento con la radice della spesa; 1-3 punti nazionali a cassa piena.
- Promessa: una disposizione a un valore del catalogo; persuasione subito verso chi ne guadagna, intesa in calo per gli alleati con linea no; al voto dopo mantenuta (chiunque l'abbia portata lì) dà poca valence, mancata ne toglie di più; conta solo se il giocatore è stato Presidente del Consiglio per almeno 4 turni in quella legislatura. Alza l'aspettativa.
- Partiti IA: tutta la cassa sui 3 bersagli col miglior rendimento atteso sul sondaggio pubblico; solo gli identitari convincono; promesse = prime 2 disposizioni del programma.
- Casse dei rivali pubbliche; spese nell'esito, una riga per partito; nessun sondaggio fra spesa e voto.

### Turno e partita (09, 12, 20, 28, 29)

- **Fasi**: apertura (esito, eventi, agende, proposte, minacce, ultimatum, emendamenti, sondaggio) → decisioni in ordine libero → chiusura in ordine fisso (atti → aula → motore: economia poi elettorato → soglie → eventi del turno dopo; voto delle elezioni dopo il motore).
- **Default** delle decisioni obbligate: bilancio non presentato → esercizio provvisorio; crisi senza scelta → sfiducia; proposta ignorata → in sospeso un turno, poi respinta; evento → opzione di default; referendum confermativo non scelto → regola dell'08.
- **Varianti dello stesso scheletro**: bilancio (passaggio dentro le decisioni con bozza indicizzabile), crisi (ordinaria amministrazione), voto, esercizio provvisorio, turno zero (aula chiusa, campagna e voto), trattativa dopo il voto (governo uscente in interim), opposizione (2-4 minuti, «avanti fino a novità» che si ferma su: voto su una misura che tocca l'elettorato del giocatore, ultimatum nella maggioranza avversaria, crisi, finestra del referendum, turno prima del voto).
- **Calendario**: turno zero ottobre-dicembre 2026 con legislatura sciolta; gennaio-marzo 2027 spartizione; prima legge di bilancio del giocatore a fine 2027; legislatura di 5 anni modificabile per norma costituzionale.
- **Fine**: partito del giocatore senza seggi in entrambe le Camere (eredità in bozza, riavvolgibile) o abbandono (eredità subito).
- **Esito**: voci nell'ordine della chiusura, al massimo 7 in evidenza, prima le critiche, il resto nel registro; ogni voce con la sua causa.
- **Salvataggi**: snapshot automatico all'inizio di ogni apertura, tutti conservati per la partita; contenuto = stato completo, seme, stato del generatore, versione dello schema, copia dei dati con cui la partita è cominciata (catalogo, disposizioni, manopole, scenario), cassa, bias comune, promesse in sospeso, spinte di campagna. Riavvolgimento all'apertura di un turno passato che cancella il futuro; nessun ramo, nessuno slot; contatore dei riavvolgimenti nell'eredità, senza penalità. Bozza del turno alla chiusura dell'app, scartata da un riavvolgimento. Schema diverso → rifiuto con messaggio, file intatto, nessuna migrazione. Più partite in corso, un file ciascuna.
- **Albo delle partite**: per ogni partita conclusa un file piccolo e autonomo, leggibile con uno schema nuovo: eredità (anni al governo, leggi ancora in vigore, indicatori contro il turno zero dello scenario giocato), serie per turno dei tre indicatori sintetici, risultati di ogni elezione, numero di riavvolgimenti. Alla conferma dell'eredità gli snapshot si cancellano.
- **Persistenza** (guscio Tauri): JSON compresso con gzip nella cartella dati dell'app (`Application Support` su macOS); il bias è nel file ma non leggibile a colpo d'occhio.

### UI (10, 11, 27)

- Direzione **S · Il Settimanale**: dossier con rete, testata «Il Pungolo», occhiello serio col fatto e il numero e titolo spiritoso sotto, vignetta, note a biro; Fraunces per i titoli, Atkinson Next per il testo. Riferimento visivo: `.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html`.
- **Schermate**: partite in corso e albo; scelta dello scenario ed editor con riepilogo dal vivo; fondazione; esito come prima pagina; cruscotto del turno con i tre indicatori a caselle, listino-borsino dei ministri (carta che si gira su Ferma e Dirotta), aula e calendario, proposte, eventi; passaggio del bilancio; anteprima della misura come mobile appeso; trattativa e spartizione; mappa con livelli, segni, scheda della regione e selettore; schema dello Stato come vista secondaria; registro con «torna al turno N»; vista dell'opposizione; campagna; eredità.
- **Mappa**: cinque classi di un inchiostro; il risultato del voto col colore del partito è l'unica eccezione; resta fino al voto successivo.
- **Tono grave** deciso dal modello: spariscono testata, nome spiritoso, battute, motti e illustrazioni, niente rimbalzi né oscillazioni.
- **Accessibilità**: testo minimo 12 px a 1×, scala della UI regolabile dal primo giorno, velocità 1×/2×/istantaneo, interruttore del movimento.

### Pipeline di taratura, offline

Fuori dal binario del gioco; produce file di dati con la loro licenza in una cartella separata (decisione del 16/10).
- **Quote dei 360 gruppi**: IPF sui margini ISTAT (regione × condizione) e MEF (regione × classe × tipo di reddito), seme dai microdati IBF 2022 della Banca d'Italia; fasce sul reddito equivalente (proposta: bassa decili 1-3, media 4-7, alta 8-10), da confermare col motore economico.
- **Istogrammi della posizione** da ESS con la posizione come quarta dimensione dell'IPF; ITANES solo per stime offline (non ridistribuibile).
- **β e fedeltà** stimati offline su ITANES (logit condizionale; matrice di voto 2018 → 2022); un β su scala 0-10 si divide per 0,36 sulla scala a 7 punti. Affluenza 2022 per regione dal CSV della Camera.
- **Licenze**: ISTAT ed EFFIS CC BY 4.0; IdroGEO trattato come CC BY-SA 4.0; CPTI15 solo conteggi derivati; debito trimestrale da Eurostat invece dei PDF Banca d'Italia; dati che vietano la ridistribuzione esclusi.

## Testing Decisions

**Cosa fa un buon test.** Verifica il comportamento esterno a una seam, mai i dettagli interni: dati + scenario + seme + comandi in ingresso, vista, esito, registro o eredità in uscita. Un buon test sopravvive a un refactoring del motore che non cambia le regole. L'oracolo viene da fuori dal codice quando esiste (testo della legge, simulazione del dossier parlamentare, valori di partenza della ricerca, bersagli della letteratura); altrimenti è una regola della spec scritta come proprietà («la sfiducia è deterministica», «un ruolo orfano si rifiuta»).

**Seams** (ricavate dal ticket 11, confermate da Mario il 21/10/2026):

1. **API pubblica di `sim-core`**, la seam principale e quasi l'unica: caricare i dati, costruire una partita da scenario, fondazione e seme, inviare comandi, chiudere il turno, leggere vista, esito e registro, salvare e ricaricare. Ogni regola di gioco si testa da qui. Gli scenari su misura (file di differenze) sono il modo di mettere la partita nello stato che il test vuole, senza aprire il motore.
2. **Riparto voti → seggi** come funzione pura, l'unica seam interna in più: ha un oracolo esterno esatto (testo della 180/2026 e simulazione del dossier sul censimento 2021) e attraverso la seam 1 i voti non si fissano a mano.
3. **Binario CLI**, che sta sopra la seam 1 e non ne apre una nuova: gioca partite intere con le tre strategie (inerte, media, riformatrice) e misura i bersagli di bilanciamento.

**Cosa si testa, per modulo.**
- *Dati*: rifiuto al caricamento di un ruolo orfano o doppio e di un valore fuori dall'intervallo di taratura, ciascuno col nome nel messaggio.
- *Scenario*: lo scenario di base riproduce i valori di partenza (debito 136,7%, spread nella forbice 79-89 pb, inflazione 4,2%, margine ~1,6%); i cursori muovono gli ingressi e i derivati si ricalcolano; gli avvisi scattano nei casi del ticket 21.
- *Riproducibilità*: stesso seme e stessi comandi danno la stessa partita; salvare, ricaricare e chiudere il turno dà lo stesso esito che chiuderlo senza salvare; dopo un riavvolgimento le stesse scelte danno lo stesso esito.
- *Contabilità*: il controllo SFC «la somma fa zero» regge a ogni turno; i conti pubblici restano interi.
- *Elezioni*: riparto della 180 contro l'oracolo; premio solo col 42% in entrambe le Camere; tetti; ripescaggio; menu che rifiuta un premio con soglia sotto il 40%.
- *Parlamento e regole istituzionali*: sfiducia deterministica; decreto rifiutato senza innesco; legge ordinaria in 2 turni; fiducia respinta → dimissioni; esercizio provvisorio; referendum abrogativo che rispetta esclusioni, quorum, finestra, valore dopo l'abrogazione e blocco del gradino.
- *Soglie*: ministro sotto 25 si dimette, alleato sotto 25 esce, ultimatum e minacce sotto 40, situazioni con isteresi che non lampeggiano.
- *Informazione*: la vista non contiene mai lo stato vero (prima stima ≠ stato quando l'errore estratto non è nullo, bias comune mai presente, intesa avversaria solo in fasce, agende avversarie assenti prima dell'esecuzione).
- *Eventi*: al massimo un evento puntuale nuovo per turno; il default vale senza scelta.
- *Bilanciamento* (CLI, test di accettazione della spec, eseguiti dopo la costruzione su molti semi): la strategia media perde circa 2,25 punti a legislatura con deviazione standard intorno a 4,5; l'inerte perde più della media ed è rieletta meno spesso entro due legislature; la riformatrice paga nella prima metà e alla seconda elezione sta meglio dell'inerte. Si tarano le manopole, non le regole; κ si accende solo se i bersagli mancano.

**UI.** Nessuna suite automatica nella v1: la UI non contiene regole, quindi un test lì ripeterebbe quelli del core. Si verifica avviando l'app e guardando le schermate.

**Prior art.** Nessuna: il repo non ha ancora codice. I test del core sono i primi; la riproducibilità a seme fisso li rende test di regressione per costruzione.

## Out of Scope

### Fuori dalla v1 (30)

Una riga per sistema, con il ticket d'origine e la cucitura su cui si appoggia, oppure «niente»: niente vuol dire che lo sforzo successivo lo aggiunge senza riscrivere ciò che la v1 costruisce.

| Sistema | Origine | Cucitura |
|---|---|---|
| Guerra, esteri attivi, diplomazia, alleanze/unioni/federazioni | decisione del 16/10 | 4 (Esteri e Difesa divisi in due dicasteri con un cambio di dati); il resto niente |
| Dazi ed embarghi | 16/10, 07 | 1 (motore in blocchi) |
| Intelligence, finanziamento di gruppi e stati esteri | 16/10 | niente |
| Svolta autoritaria, attentati | 16/10 | niente |
| Ricerca tecnologica | 16/10 | niente |
| Fase da outsider (crescere per più turni prima del primo voto) | 16/10, 20 | niente |
| Uscita dall'euro | 07 | 1 |
| Banche e crisi bancaria | 07 | 1 |
| Immigrazione come offerta di lavoro su u* | 07 | niente |
| Giunte regionali come attori; indicatori di servizio regionali | 03, 27 | niente |
| Appoggio esterno e governo di minoranza | 05 | niente |
| Salienza mobile degli interessi, dei temi in campagna e delle aspettative | 06, 26, 29 | niente |
| Attrazione fra gruppi a confidenza limitata | 06 | niente |
| Rinvio presidenziale, Corte costituzionale | 08 | niente |
| Forma di governo e bicameralismo come norme costituzionali, sede deliberante, navetta, tetto ai decreti | 08 | niente |
| Giocatore come alleato minore in un governo altrui | 12 | niente |
| Congressi e sfide alla leadership, luna di miele per un leader nuovo del partito del giocatore | 12, 29 | niente |
| Fusioni fra partiti | 18 | niente |
| Collegi uninominali (e l'impostazione Rosatellum), ballottaggio, preferenze | 13 | niente |
| Regista adattivo degli eventi | 15 | niente |
| Epidemia con modello SIR | 15 | niente |
| Campagna per i referendum | 26 | niente |
| Migrazioni interne, con i seggi che seguono la popolazione | 27 | 3 (seggi dalla popolazione censita) |
| Migrazioni dei salvataggi fra versioni dello schema; rami della stessa partita | 28 | 2 (versione dello schema) |
| Aspettative estese ai servizi | 29 | niente |
| Rapporto di parte: il ministro che piega i numeri; giornale manipolabile | 25 | niente |
| Scripting in sandbox (Rhai o Lua) | 11 | niente |

### Altro fuori dalla v1

- Pubblicazione: store, marchio, nome del gioco, traduzioni. La v1 è per Mario, in italiano.
- Altre piattaforme (Windows, Linux, Steam Deck, web): non sono un requisito; il core puro compilabile in WASM non le esclude.
- Notarizzazione e CI di distribuzione.
- Scenari storici; cambi alla struttura dello scenario (data, istituzioni, dicasteri, regioni, gruppi, posti dei partiti).
- Stato vero visibile al giocatore.
- Parlamentari come personaggi; congressi; sfide interne al partito del giocatore.

## Further Notes

- **Contenuto da scrivere dopo la spec**, in file di dati sotto revisione di Mario: i 36 atti oltre ai 4 della Sanità (fra cui gli 8 regionali), le tabelle valore → effetto delle disposizioni (comprese le 3 territoriali), il catalogo degli eventi (una famiglia scritta per intero come modello, come la Sanità per gli atti), leader, nomi e caricature dei sette partiti, rose di ministri e tecnici, istituti di sondaggio, testi e motti. La Sanità scritta per intero sta in `.scratch/v1-giocabile/assets/16-catalogo-schema-e-sanita.md`.
- **Taratura dopo la costruzione**: tutte le manopole elencate nei ticket (soglie, pesi, costi, λ, ρ, β, quota di colpa del Presidente del Consiglio, finestra delle aspettative, rendimenti della campagna, prezzi dell'informazione, dote, errori di stima, luna di miele, soglie delle situazioni, rischi degli eventi, taglia del fondo di riserva 0,2-0,4 miliardi l'anno proposti, N seggi per la legge elettorale avversaria, X% del partito di protesta) si tarano contro i tre bersagli; nessuno spike prima, perché nessuna decisione aspetta un numero.
- **Durata dell'iter**: la legge ordinaria in 2 turni è più veloce del reale (3-4 turni); è un parametro di bilanciamento, non un errore.
- **Semplificazioni dichiarate** da non «correggere» nel piano: la delega sul nucleare come legge a effetto differito; il valore dopo l'abrogazione *precedente* anche per le novelle testuali; le gare dei balneari non separate dai taxi; il decreto flussi, il ticket e le tasse universitarie referendabili; la conversione dei decreti nello stesso turno.
- **Punti aperti per il piano, non di prodotto**: come trattare i seggi Estero (8 alla Camera, 4 al Senato), che l'elettorato a 20 regioni non vota (la ricerca non ha trovato la loro ripartizione); la quota retributiva/contributiva dello stock di pensioni [NON TROVATO in 23]; lo stanziamento del fondo di riserva e del Fondo emergenze; la siccità come situazione a soglia invece che emergenza puntuale (proposta in 19, il 15 la tiene fra le emergenze). Ognuno si chiude con un'assunzione dichiarata o con una ricerca puntuale, senza riaprire la mappa.
- **Primo passo del piano** (11): la prima compilazione Tauri + Rust con il crate `sim-core` e il binario CLI accanto.
- **Fonti**: ogni cifra reale citata qui ha la sua fonte nel ticket d'origine e nella pagina di `docs/ricerca/` che quel ticket cita. Le copie integrali delle fonti stanno solo in locale; nel repo ci sono i `.provenance.json`.
