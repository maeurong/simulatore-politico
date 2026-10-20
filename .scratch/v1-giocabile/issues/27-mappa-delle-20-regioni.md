Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa varia davvero fra le 20 regioni nel modello, e cosa ne mostra la mappa? Oggi sono regionali il mix dei settori produttivi e l'Okun con isteresi (07), i 18 gruppi di ogni regione con affluenza e posizioni proprie (06, 14), il Senato su base regionale (13) e le emergenze naturali (15, 19); le situazioni dei dicasteri restano nazionali (15).

Da decidere:
- quali indicatori la mappa colora, e se ne sceglie uno alla volta;
- se gli indicatori di servizio (sanità, scuola, giustizia…) diventano regionali o restano nazionali;
- se un atto o un cantiere può avere una regione bersaglio (grandi opere visibili sulla mappa) o vale sempre per tutto il paese;
- se il divario Nord-Sud diventa un cursore dell'editor dello scenario o resta solo nel file (21).

Le regioni restano senza giunta (`CONTEXT.md`, Regione): le giunte come attori sono un punto di aggancio, non questa domanda.

## Answer

Grilling con Mario del 20/10/2026, cinque giri (l'ultimo dopo la ricerca, che ha corretto tre premesse), quadro finale confermato; tre scelte di Mario contro la raccomandazione: disposizioni con perimetro regionale, regione bersaglio per tutti gli atti di Infrastrutture e Trasporti e di Sviluppo economico, autonomia differenziata nella v1. Termini nuovi in `CONTEXT.md`: macroarea, regione bersaglio, disposizione territoriale, perimetro, livello; precisati regione e disposizione. Valori reali delle tre disposizioni nuove in `docs/ricerca/sintesi/disposizioni-territoriali-al-turno-zero.md`.

Due delle domande avevano già risposta: Dicasteri (03) dava la regione bersaglio a Infrastrutture e Trasporti e a Sviluppo economico, Editor dello scenario (21) teneva il divario Nord-Sud solo nel file. Confermate entrambe, la prima allargata.

**Cosa varia fra le regioni.** Resta regionale ciò che già lo era: reddito, disoccupazione con u* propria, mix dei settori, i 18 gruppi, seggi del Senato, pericolosità naturale. Lo diventano in modo esplicito lo stock di infrastrutture (che la prevenzione delle emergenze già leggeva per regione, 15) e gli investimenti privati. Restano nazionali:
- *gli indicatori di servizio e le situazioni dei dicasteri.* La spesa arriva per popolazione (03) e il giudizio si forma sulle variazioni (17): un indicatore regionale mosso dalla sola spesa nazionale si muoverebbe uguale ovunque, e uno scarto fisso non sposterebbe un voto. Il divario vive dove già si muove;
- *la popolazione fra le regioni:* invecchia con la demografia nazionale, nessuno si trasferisce, i seggi per regione restano quelli del censimento. Le migrazioni interne vanno ai punti di aggancio;
- *il divario Nord-Sud* si cambia solo nel file dello scenario (21).

**Regione bersaglio.** Tutti e 4 gli atti di Infrastrutture e Trasporti e di Sviluppo economico, 8 su 40, spendono e rendono su una regione:
- *atti rapidi:* impulso sulla domanda della regione che decade (occupazione locale subito);
- *atti lenti,* in forma di cantiere: stock della regione (infrastrutture o investimenti), che rafforza la prevenzione (15), abbassa la u* regionale e porta domanda alla regione invece di spalmarla per popolazione.
- *Scelta:* la fa il ministro, aggiungendo al punteggio dell'atto (04) un termine di bisogno (stock basso, disoccupazione alta) e uno dei voti del suo partito nella regione, pesato dalla vocazione: basso per un partito identitario, alto per uno governista (18). Il motivo sta nella riga dell'agenda («Calabria: ci votano»).
- *Dirottamento:* il Presidente del Consiglio dirotta l'atto, la regione o entrambi con la leva di 04, al costo di soddisfazione proporzionale al punteggio perso; solo finché l'atto è in agenda, mai a lavori avviati (fermare a metà perde quanto speso, 16).
- *Merito:* nella regione bersaglio la variazione dovuta all'atto va al partito del dicastero che l'ha fatto; il motore la conosce esatta, perché l'atto entra come termine additivo sulla domanda della regione. Altrove vale la regola di colpa di 17. Senza questa eccezione un ministro che sceglie la regione per i suoi voti lascerebbe il merito a Lavoro e Welfare.

**Disposizioni territoriali.** Valgono solo nel loro **perimetro**, un insieme di regioni scritto nella definizione e mai scelto da una legge; le macroaree sono le tre ISTAT (Nord, Centro, Mezzogiorno). Si aggiungono tre disposizioni, 35 in tutto:

| Disposizione | Proprietario | Gradini | Perimetro | Effetto |
|---|---|---|---|---|
| Decontribuzione per il Mezzogiorno | Lavoro e Welfare | 0 / 15 / 20* / 30% dei contributi del datore; in arrivo 20% con tetto più basso nel 2028, 15% nel 2029, poi 0 | Mezzogiorno (8 regioni) | costo del lavoro dei dipendenti privati in forza → occupazione nel perimetro |
| Credito d'imposta ZES unica | Sviluppo economico | stanziamento 0 / 1 / 2,3* miliardi l'anno; in arrivo 1 nel 2027 e 0,75 nel 2028 | ZES unica: Mezzogiorno più Marche e Umbria (10 regioni), intensità ridotta in Abruzzo, Marche e Umbria, dove le zone assistite coprono solo alcuni comuni | investimenti privati nel perimetro |
| Autonomia differenziata | fuori dicastero | non attuata* / prime materie (protezione civile, professioni, previdenza complementare) / intese complete (anche salute e coordinamento della finanza pubblica) | Liguria, Lombardia, Piemonte, Veneto | gettito trattenuto, quota crescente col gradino |

Valori reali e fonti in `docs/ricerca/sintesi/disposizioni-territoriali-al-turno-zero.md:16-19`. La ricerca ha corretto tre premesse del giro precedente:
- *la decontribuzione* vigente (L. 207/2024, c. 406-412) vale per i lavoratori già in forza, non per le assunzioni (`:16`); l'incentivo alle assunzioni è il Bonus ZES (DL 62/2026), scartato perché vale 26 milioni, invisibile a scala di regione (`:17`);
- *la ZES unica* comprende Marche e Umbria dal 20/11/2025 (L. 171/2025), quindi non coincide col Mezzogiorno; le aliquote (15-40%, più maggiorazioni per piccole e medie imprese) sono della Carta UE degli aiuti regionali fino al 2027, quindi la leva è lo stanziamento e le aliquote restano fisse (`:18`, `:33-34`);
- *l'autonomia* non ha un gradino «solo materie senza LEP»: le intese approvate in via definitiva dal Consiglio dei ministri il 13/10/2026 contengono la salute, che è una materia LEP, e i LEP non sono determinati (`:19`, `:44-45`). I gradini seguono le due intese reali; al turno zero mancano le leggi di approvazione.

Decontribuzione e credito ZES sono agevolazioni fiscali e contributive come il cuneo, quindi non referendabili (24): le disposizioni escluse dal referendum diventano sette.

- *Niente meccanismo generale* di valori per macroarea su disposizioni esistenti (gabbie salariali): moltiplicherebbe disposizioni e tabelle da tarare.
- *Autonomia differenziata senza giunte.* Agisce sul gettito: le regioni del perimetro trattengono la quota che copre le materie trasferite, e la spesa per quelle materie segue il gettito della regione invece della popolazione. Più domanda e reddito nel perimetro, meno nel resto, Mezzogiorno in testa: la perequazione cala, gli indicatori nazionali restano dove sono. La quota si tara sul residuo fiscale per regione: la serie CPT 2023 è aperta (CC BY 3.0 IT) e dà le quattro regioni del perimetro a +65,6 miliardi in tutto, ma il calcolo è ingenuo e va confrontato con uno studio pubblicato prima di tarare (`:50-72`). Come la legge elettorale, la propongono solo il Presidente del Consiglio o il governo avversario, e la colpa va al Presidente del Consiglio. Abrogabile per referendum come ogni disposizione non fiscale, con valore dopo l'abrogazione «non attuata»: l'inammissibilità reale del quesito del 2025 dipende dalla sentenza 192/2024, non da una regola da copiare.
- *Merito* delle due disposizioni di dicastero: al dicastero proprietario, nel perimetro, come per la regione bersaglio.

**Mappa.**
- *Livelli,* uno alla volta, a cinque classi di un inchiostro (10): consenso della coalizione dal sondaggio (di default, tinta sulla stima centrale), disoccupazione, reddito disponibile, affluenza, pericolosità naturale (nota, 25). Dopo il voto, il livello **risultato del voto**: primo partito per regione col colore del partito, unica eccezione all'inchiostro unico, e seggi del Senato; resta fino al voto successivo.
- *Segni fissi* sopra ogni livello: atti e cantieri regionali in corso o finiti, emergenze aperte.
- *Scheda della regione* al clic: gruppi, elettori campione, seggi, forbice del sondaggio (larga nelle regioni piccole), disposizioni territoriali in vigore.
- *Selettore:* quando una scelta chiede una regione (bersaglio di campagna, 26; dirottamento; emergenza) si sceglie sulla mappa. L'anteprima di una disposizione territoriale evidenzia il perimetro.

Effetti sulla mappa:
- nessun ticket nuovo;
- Costo del governare (29): peso dei voti per vocazione, impulso degli atti rapidi regionali, effetto dello stock su u*, costo del dirottamento della regione, quota di gettito trattenuto, intensità di sgravio e credito ZES;
- Punti di aggancio (30): migrazioni interne con seggi che seguono la popolazione; indicatori di servizio regionali con le giunte;
- Catalogo atti e riforme (16), contenuto dopo la spec: gli 8 atti regionali e le 3 disposizioni territoriali con le loro tabelle valore → effetto.
