Type: grilling
Status: resolved
Blocked by: 25, 26
Assignee: Mario
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

## Answer

Grilling con Mario del 20-21/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Fatti in `docs/ricerca/modelli/voto-economico-e-retrospettivo.md` (costo del governare ~2,25 punti a legislatura, deviazione standard 4,5, asimmetria del malcontento: `:13`, `:18`) e `docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md` (calo nella prima metà, recupero parziale: `:21`). Termini nuovi in `CONTEXT.md`: costo del governare, aspettativa, luna di miele.

**Il problema.** Con l'asimmetria del giudizio (17) chi perde da una misura pesa circa tre volte chi ci guadagna: ogni redistribuzione costa consenso, e un governo che non fa nulla paga solo gli shock. Senza un prezzo dell'inerzia, l'inerzia vince.

**Bersagli di bilanciamento**, test di accettazione della spec, verificati con partite simulate dal binario CLI (11) su tre strategie automatiche: *inerte* (chiude ogni turno senza clic, bilancio dell'anno prima), *media* (accetta le agende, bilancio in equilibrio), *riformatrice* (due o tre leggi di riforma per legislatura).
1. La media perde circa 2,25 punti a legislatura, deviazione standard intorno a 4,5.
2. L'inerte perde più della media ed è rieletta meno spesso, entro due legislature.
3. La riformatrice paga nella prima metà e alla seconda elezione sta meglio dell'inerte: le riforme costano subito e rendono dopo.

**Logoramento senza regola scritta.** Nessuna deriva κ legata al calendario: sarebbe una causa fuori dal modello, come il regista adattivo escluso in 15. Resta la via d'uscita, una manopola κ da aggiungere solo se la taratura manca i bersagli. Le fonti, oltre a giudizio asimmetrico, situazioni, invecchiamento e debito già decisi:
- **Aspettative.** Il reddito reale della fascia si giudica contro la sua crescita media degli ultimi ~8 turni, non contro lo zero: dopo anni buoni un anno piatto è un peggioramento. Le promesse (26) alzano l'asticella. Solo sul reddito reale, che contiene già ogni euro (17); i servizi hanno l'erosione.
- **Erosione nominale.** Restano fermi finché una legge non li cambia: budget dei dicasteri, fondo di riserva, scaglioni e soglie delle imposte (drenaggio fiscale), salario minimo, pensione minima, soglie dei sussidi. Si indicizzano da sole solo le pensioni in essere, col parametro del regime (07). La bozza della legge di bilancio (09) propone le cifre nominali dell'anno prima, con «indicizza all'inflazione» per tutto o per voce e il costo esatto in anteprima. Bilancio fotocopia ed esercizio provvisorio (08) sono quindi tagli reali e tasse più alte, con l'inflazione al 4,2% del turno zero (23).

**Forma a U.** Emerge senza funzione del tempo: la luna di miele che si spegne fa il calo della prima metà, la campagna (26) il recupero. Luna di miele solo al primo governo del giocatore, come nell'evidenza sui leader al primo mandato; niente al ritorno dall'opposizione né dopo un ribaltone, dove il giudizio che riparte da zero (17) premia già.

**Margine discrezionale vero.** 1,6% al turno zero (23). Il giocatore lo allarga nella legge di bilancio togliendo alla spesa di funzionamento (servizi che peggiorano) o facendo disavanzo (tetto UE, spread). Gli atti restano leve mirate sugli indicatori di servizio; le leve grosse sono bilancio e leggi di riforma.

**Visibilità, con la causa (09).** Crescita attesa accanto al reddito reale di ogni fascia; voce nell'esito («reddito +0,3%, atteso +1%: giudizio in calo»); perdita reale dall'ultimo aggiornamento accanto a ogni budget e scaglione («Sanità: −4% reale in un anno»). Resta imperfetta la prima stima del reddito (25).

**Manopole.** Tutti i valori provvisori elencati sopra, più la finestra delle aspettative (~8 turni) e κ (assente di partenza), stanno nel file dei dati con valore di partenza, intervallo e ticket d'origine (modding solo dati, 11). Taratura dopo la costruzione, contro i tre bersagli; nessuno spike adesso, perché nessuna decisione della mappa aspetta un numero.

**Scelta di strumento, dichiarata e non chiesta:** le tre strategie sono comandi scriptati nel binario CLI, sopra lo stesso core.

Effetti sulla mappa:
- nessun ticket nuovo;
- Punti di aggancio (30): aspettative estese ai servizi; salienza mobile delle aspettative fra gruppi; luna di miele per un leader nuovo del partito, se arrivano i congressi (12).
