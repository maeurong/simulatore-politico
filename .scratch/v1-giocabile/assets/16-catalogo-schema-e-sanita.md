# Catalogo delle misure: schema, disposizioni, Sanità d'esempio

Asset del ticket [Catalogo atti e riforme](../issues/16-catalogo-atti-e-riforme.md). Numeri e scale sono provvisori e vanno al bilanciamento; i valori di partenza (*) al ticket di scenario, che li verifica sulle leggi vere.

## Schema di un atto del ministro

| Campo | Contenuto |
|---|---|
| nome, motto | nome dell'atto; una riga nel tono di Il Settimanale (10), che non tocca mai i numeri |
| dicastero, casella | uno dei 10; casella della griglia: rapido o lento × sinistra o destra |
| posizione | punto sull'asse a 7, scritto a mano, coerente col lato della casella |
| forma | puntuale (costa nel turno in cui si esegue) o cantiere (impegna il margine per N turni, rende a lavori finiti, fermato a metà perde quanto speso) |
| costo | in quote trimestrali del margine discrezionale di riferimento |
| effetti | 1-2 grandezze della riga del dicastero (tabella 03), ciascuna con intensità in unità vere, ritardo in turni e forma del ritardo: impulso che decade (con emivita) o stock con vita utile. La capacità del ministro moltiplica l'intensità e divide il ritardo (04) |
| spesa | sempre in domanda finale col vettore di settori del dicastero (07); nessun campo da scrivere |
| regione bersaglio | solo Infrastrutture e Trasporti, Sviluppo economico (03) |
| prerequisito | facoltativo: una disposizione a un certo valore |

Mai effetti diretti su posizioni degli elettori, valence o intesa: ai gruppi si arriva solo dagli indicatori (06). Il punteggio con cui il ministro sceglie (04) usa l'effetto nominale scritto qui, non una simulazione.

## Schema di una disposizione

| Campo | Contenuto |
|---|---|
| nome, dicastero | ogni disposizione appartiene a un dicastero, il cui ministro è il proponente |
| valori | gradini ordinati (parametro) o regimi con nome; «non istituita» per le voci nuove |
| posizione per valore | scala valore → punto sull'asse |
| partenza | valore in vigore nel 2026, legge reale che l'ha fissato, valore precedente (serve all'abrogazione) |
| effetti | dal motore, dove la regola esiste già in 07 (pensioni, sussidi, fisco, u*); altrimenti tabella valore → effetto sulle grandezze, con intensità, ritardo e forma come per gli atti |
| saldo | costo o gettito annuo, che pesa sul tetto di spesa netta |
| decorrenza | i regimi di spesa obbligatoria valgono per le coorti future, pro rata; unica leva retroattiva l'indicizzazione delle pensioni in essere |

Regole generiche, valide per tutte:
- **Legge di riforma** = una o più disposizioni portate a un valore nuovo; posizione della misura = media delle posizioni dei valori nuovi.
- **Linea**: sì se il valore nuovo è più vicino alla posizione del partito di quello in vigore, più intesa e malus d'opposizione (08).
- **Emendamento**: su un parametro il valore a metà strada (arrotondato al gradino), su un regime l'entrata in vigore rinviata di un anno.
- **Abrogazione** (anche per referendum, 12): riporta al valore precedente.
- **Proposta del ministro** (innesco 3 di 04): la disposizione del suo dicastero più lontana dalla sua posizione, portata al valore più vicino, se la distanza è almeno 2; oppure lo sblocco del prerequisito di un atto che il suo obiettivo richiede.

## Le 30 disposizioni della v1

| Dicastero | Disposizioni (valori; * partenza) |
|---|---|
| Economia e Finanze | forfettario degli autonomi (0 / 65k / 85k* / 100k €); cuneo fiscale (0 / 3 / 7* punti); detrazione edilizia (0 / 36-50%* / 65% / 110%); tetto al contante (1k / 3k / 5k* / nessuno) |
| Sviluppo economico | credito d'imposta per gli investimenti (0 / 10%* / 20% / 40%); concorrenza nei servizi: taxi, balneari, commercio (licenze chiuse* / gare / libere) |
| Lavoro e Welfare | regime delle pensioni (retributivo / contributivo* / quota base universale); età pensionabile (62…70, 67*); indicizzazione delle pensioni in essere (piena / per fasce* / bloccata sopra soglia); pensione minima (attuale* / +20% / +50%); sussidi di disoccupazione (assicurativo* / esteso / universale); reddito minimo (non istituito / categoriale* / universale); salario minimo legale (nessuno* / 7 / 9 / 11 €/h); licenziamento (reintegro / tutele crescenti* / solo indennizzo) |
| Sanità | ticket (nessuno / attuale* / alto); accesso a Medicina (numero chiuso / semestre filtro* / libero); privato accreditato (ridotto / attuale* / ampio) |
| Istruzione e Ricerca | obbligo scolastico (16* / 18 anni); tasse universitarie (gratuite / per ISEE* / alte) |
| Infrastrutture e Trasporti | codice degli appalti (rigido / semplificato* / deroga generale); concessioni autostradali (pubbliche / private*) |
| Ambiente ed Energia | nucleare (vietato* / consentito); accise sui carburanti (basse / attuali* / alte con carbon tax); autorizzazioni per le rinnovabili (lente* / rapide) |
| Interno | ingressi per lavoro, decreto flussi (bassi / attuali* / alti); cittadinanza (ius sanguinis* / ius scholae / ius soli temperato); accoglienza (centri* / diffusa) |
| Giustizia | prescrizione (classica / bloccata dopo il primo grado / improcedibilità*); pene alternative al carcere (ristrette / attuali* / estese); intercettazioni (ampie* / limitate) |
| Esteri e Difesa | leva (sospesa* / volontaria rafforzata / obbligatoria); export di armi (restrittivo* / permissivo) |

Lavoro e Welfare ne ha 8: è il dicastero più conteso nella spartizione (05) e quello che prende più colpa (06).

## Sanità, scritta per intero

Grandezze della riga (03): indicatore di servizio «qualità del servizio sanitario» (liste d'attesa), speranza di vita (lenta).

### Atti

| Casella | Atto | Pos. | Forma, costo | Effetti |
|---|---|---|---|---|
| rapido, sinistra | Piano straordinario sulle liste d'attesa negli ospedali pubblici (prestazioni aggiuntive del personale) | 3 | puntuale, 1 quota | liste d'attesa ↓ forte, ritardo 1, impulso che decade in ~4 turni |
| rapido, destra | Convenzioni col privato accreditato per smaltire le liste | 5 | puntuale, 1 quota; prerequisito: privato accreditato almeno «attuale» | liste d'attesa ↓ forte, ritardo 1, impulso che decade in ~3 turni |
| lento, sinistra | Case di comunità e medicina territoriale | 3 | cantiere, 4 turni × 1 quota | liste d'attesa ↓ moderato, stock con vita utile ~40 turni; speranza di vita ↑ lieve, ritardo ~12 turni |
| lento, destra | Accorpamento delle ASL e premi di risultato ai primari | 5 | puntuale, ½ quota | liste d'attesa ↓ lieve, ritardo 4, stock con vita utile ~20 turni; spesa di funzionamento −2% (efficienza) |

### Disposizioni

| Disposizione | Valori → posizione | Effetti |
|---|---|---|
| ticket | nessuno 2 / attuale* 4 / alto 6 | gettito; liste d'attesa (domanda ridotta) ↓ lieve col ticket alto; reddito reale della fascia bassa ↓ |
| accesso a Medicina | libero 2 / semestre filtro* 4 / numero chiuso 5 | stock di medici fra ~24-40 turni, che sposta le liste d'attesa; costo universitario annuo con l'accesso libero |
| privato accreditato | ridotto 2 / attuale* 4 / ampio 6 | liste d'attesa ↓ rapido con «ampio», spesa sanitaria ↑; prerequisito delle convenzioni |
