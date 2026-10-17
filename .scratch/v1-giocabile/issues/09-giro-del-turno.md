Type: grilling
Status: resolved
Blocked by: 03, 04
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa fa il giocatore in un turno trimestrale, in che ordine, e cosa vede alla fine: budget, proposte dei ministri, misure, sondaggi, eventi, esito del turno? Quanto dura un turno in minuti reali?

Ingresso da 04 (Autonomia dei ministri): a inizio turno ogni ministro annuncia l'atto in agenda (fermare / dirottare), le proposte normative in sospeso (accogli / rinvia / respingi) e le minacce di dimissioni; interim e rodaggio durano un turno.

Ingresso da Elettorato (06): il giocatore vede sondaggi con tre strati di errore e incroci per condizione, fascia di reddito e regione; a richiesta, tre elettori campione per gruppo, ricavati dal sondaggio.

## Answer

Grilling con Mario del 17/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Termini nuovi in `CONTEXT.md`: apertura, chiusura, esito del turno, registro, indicatore sintetico. Il turno è discreto e si chiude con un clic: il «tick giornaliero con pausa e velocità» di `docs/ricerca/sistemi/tempo-e-ritmo.md:55-56` è superato.

**Tre fasi.**
- **Apertura:** l'esito del turno precedente, poi eventi aperti, agenda dei 10 ministri, proposte in sospeso, minacce di dimissioni, ultimatum degli alleati, emendamenti alle leggi in voto, sondaggio del trimestre.
- **Decisioni**, in ordine libero: fermare o dirottare atti, accogliere/rinviare/respingere proposte, norme proprie, calendario, emendamenti, fiducia, fondo di riserva, dimissioni chieste, nomine.
- **Chiusura**, avviata dal giocatore, in ordine fisso: atti dei ministri → voti d'aula (conversioni, leggi al secondo turno, fiducia, sfiducia) → motore (economia, poi elettorato) → soglie (ministro sotto 25, alleato sotto 25) → eventi del turno dopo.

**Nessuna moneta delle azioni.** Niente capitale politico né punti azione: i limiti sono calendario a 3 posti, decreto solo con innesco, una proposta per ministro, budget, e i costi in soddisfazione, intesa e valence, ciascuno con la causa visibile.

**Turno senza clic.** Chiudere subito è sempre lecito: i ministri eseguono l'agenda, i voti già fissati si tengono. Default delle decisioni obbligate: bilancio non presentato → esercizio provvisorio (08); crisi senza scelta → sfiducia (05); proposta di un ministro ignorata → resta in sospeso un turno, poi vale come respinta, col suo costo in soddisfazione. Nessuna finestra modale obbligatoria.

**Eventi** solo in apertura, mai a metà turno; si affrontano nello stesso turno (decreto, fondo di riserva), altrimenti i loro effetti scattano in chiusura. Famiglie e origine al ticket 15.

**Sondaggi.** Uno pubblico per trimestre, gratuito, in apertura, con tutti gli incroci (condizione, fascia, regione) e l'errore che cresce nei sottocampioni piccoli (06). Sondaggi privati più precisi: nebbia «Informazione imperfetta».

**Durata obiettivo** (di design, non vincolo tecnico): 5-10 minuti un turno ordinario, 20-30 i turni pesanti (bilancio, elezioni, crisi), circa 2-3 ore a legislatura. Sotto i 3 minuti un turno ordinario è vuoto, sopra i 15 è sovraccarico. Motivo: le lamentele su turni vuoti e partite che si allungano (`docs/ricerca/sistemi/tempo-e-ritmo.md:49-50`).

**Esito del turno.** Voci nell'ordine della chiusura: voti d'aula col conteggio dei franchi tiratori, atti eseguiti, economia (indicatori sintetici con la variazione), sondaggio, variazioni di soddisfazione e intesa con causa, dimissioni e uscite, eventi nuovi. Al massimo 7 voci in evidenza, prima le critiche (`docs/ricerca/sistemi/feedback-visivo-dello-stato.md:36`); il resto nel **registro** del turno. Rivelate una alla volta (`docs/ricerca/estetica/movimento-e-feedback.md:21`). La forma visiva (prima pagina di giornale) va a Direzione artistica (10); il giornale manipolabile a «Informazione imperfetta».

**Indicatori sintetici**, uno per ogni modo di perdere il governo: consenso (intenzione di voto della coalizione, con forbice), maggioranza (seggi oltre la metà in ciascuna Camera, con gli alleati in ultimatum segnati), conti (spread). Più data e turni al voto. Niente indice composito di «salute del paese»; chiude l'aperto di `docs/ricerca/sistemi/feedback-visivo-dello-stato.md:47`.

**Agende.** Una tavola, una riga per ministro (atto, costo, motivo); in evidenza solo le righe che superano una soglia, vanno contro la posizione del partito del giocatore o toccano un evento aperto. Le altre non chiedono clic.

**Anteprima di una misura** prima della conferma (`docs/ricerca/estetica/movimento-e-feedback.md:23`): indicatori toccati con la direzione, ritardo in turni, gruppi colpiti, linee dei partiti e forbice del voto. Niente cifre esatte: precisione e fonte della stima vanno a «Informazione imperfetta».

**Turni speciali.**
- *Bilancio (ottobre-dicembre):* passaggio dentro le decisioni: schermata con aliquote delle 4 imposte, budget dei 10 dicasteri (funzionamento e margine), fondo di riserva e saldo rispetto al tetto UE → presentazione → emendamenti degli alleati come richieste di budget → accogli/respingi, fiducia → voto in chiusura. Calendario pieno; agende e decreti con innesco restano.
- *Crisi (05):* ordinaria amministrazione: agende con fermare/dirottare, solo decreti con innesco, leggi in calendario ferme un turno senza decadere, fondo di riserva disponibile; la trattativa per allargare occupa le decisioni.
- *Voto:* elezioni nella chiusura dell'ultimo turno della legislatura (o del turno dopo lo scioglimento), dopo il motore. L'apertura del turno dopo mostra risultato e seggi; la trattativa (05) occupa le decisioni; il governo uscente sbriga gli affari correnti (interim su tutti i dicasteri); ministri e agende del nuovo governo partono dal turno successivo.
- *Esercizio provvisorio:* come in 08.

**Ne escono:**
- Opposizione (12) si sblocca e eredita lo scheletro a tre fasi.
- Crisi ed eventi (15): gli eventi nascono in chiusura e si mostrano in apertura.
- Informazione imperfetta (nebbia): precisione dell'anteprima e chi la fornisce, giornale manipolabile, sondaggi privati.
- Costo del governare (nebbia): proposta ignorata che vale come respinta, tetto di 7 voci, tempi obiettivo per turno.
- Campagna elettorale (nebbia): il turno prima del voto.

