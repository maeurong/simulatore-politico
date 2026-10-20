Type: grilling
Status: open
Blocked by: 25, 26
Label: wayfinder:grilling
Map: ../map.md

## Question

Come si evita che non fare nulla sia la strategia migliore? Serve una perdita di consenso endogena (logoramento di chi governa, aspettative che crescono, situazioni che si accendono da sole), e quanto deve pesare rispetto al giudizio retrospettivo sugli interessi (17)?

Due domande collegate:
- il margine discrezionale reale è circa l'1,6% del budget dei dicasteri (23): il gioco lo allarga perché gli atti contino, o lo lascia vero e fa pesare di più le leggi di riforma?
- i valori provvisori elencati sotto: la spec li fissa come manopole con un valore di partenza e un intervallo, da tarare dopo la costruzione con partite simulate, oppure qualcuno va fissato prima con uno spike usa e getta?

Valori provvisori raccolti dai ticket risolti:
- ministro (04): soglie 40/25, costi di fermare, dirottare e respingere;
- alleato (05): intesa 40/25, pesi dei dicasteri nella richiesta, costo della rottura dell'impegno pre-elettorale;
- iter (08): tasso dei franchi tiratori, malus d'opposizione, costo d'intesa della fiducia e degli emendamenti respinti, costo di valence di un voto perso, legge ordinaria in 2 turni contro i 3-4 reali;
- turno (09): proposta ignorata che vale come respinta dopo un turno, tetto di 7 voci nell'esito, 5-10 minuti per turno ordinario e 20-30 per quelli pesanti;
- opposizione (12): perché non convenga perdere apposta, ricompattamento e costo di valence di una sfiducia respinta, costo del quorum mancato, franchi tiratori quando il giocatore vota lontano dalla propria posizione, 2-4 minuti per turno;
- legge elettorale (13): guadagno in seggi che spinge il governo avversario a cambiarla, costo d'intesa per seggio perso da un alleato, taglie esatte del premio;
- eventi (15): soglie di accensione e spegnimento delle sei situazioni, rischi per turno di scandali ed emergenze, soglia di 2σ degli shock, premio e malus di valence della gestione, ampiezza delle spinte di posizione, gravità che fa scattare il tono grave;
- catalogo (16): costi e intensità degli atti, scale valore → posizione delle disposizioni;
- interessi (17): pesi della tabella per condizione, memoria λ del giudizio, asimmetria ρ (≈ 1/3 di partenza), quota fissa di colpa del Presidente del Consiglio;
- partiti IA (18): soglia di elettori per la nascita di un partito di protesta, quota di seggi della scissione, soglia del cambio di leader, margine del voto anticipato, crescita del malus d'opposizione nell'ultimo anno, peso dei livelli di vocazione;
- emergenze naturali (19): emergenze idro gravi all'anno (4 di partenza), rischio sismico minimo per le regioni senza episodi, k di prevenzione per idro e incendi, tre livelli di gravità, fondo di riserva (0,2-0,4 miliardi l'anno), siccità come situazione o emergenza puntuale;
- partenza (20): durata e decadimento della luna di miele del partito del giocatore;
- editor (21): intervalli di taratura dei sei cursori dello scenario su misura;
- informazione (25): prezzi di sondaggio privato, test di una misura e centro studi; taglia della dote; peso di 2 per mille e donazioni; numero di istituti; errore della prima stima e della previsione;
- abrogazione (24): intensità dell'incentivo agli investimenti in % del costo, posizioni dei gradini rinominati;
- campagna (26): rendimento di ciascuna leva (mobilitare, persuadere, convincere), scala di 1-3 punti a cassa piena, premio e malus di valence delle promesse, costo d'intesa di una promessa lontana, dote del partito di protesta. Le promesse sono una prima forma di aspettative che crescono.
- mappa delle regioni (27): peso dei voti del partito nella scelta della regione bersaglio per ciascuna vocazione, intensità e decadimento dell'impulso degli atti rapidi regionali, effetto dello stock regionale su u*, costo di soddisfazione del dirottamento di una regione, quota di gettito trattenuta per gradino dell'autonomia differenziata, intensità dello sgravio per il Mezzogiorno e del credito ZES.
