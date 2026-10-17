Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come funzionano trattativa di coalizione e crisi di governo nella v1: come si calcolano le quote di dicasteri per partito (Gamson?), quanti tecnici si possono nominare, cosa accumula lo scontento di un alleato, quando un alleato esce, come si arriva alla sfiducia e cosa succede dopo (nuova maggioranza, elezioni anticipate)?

Ingresso da 04 (Autonomia dei ministri): la soddisfazione dei ministri di un partito è uno degli ingressi del suo scontento; le dimissioni chieste a un suo ministro sono un colpo diretto; i tecnici non toccano nessun partito. Qui si decidono i pesi.

## Answer

Grilling con Mario del 17/10/2026, cinque giri. Raccomandazioni accettate, tranne due cambi di Mario: la caduta del governo **non** è game over, perché il giocatore passa all'opposizione; i seggi premio dipendono dalla legge elettorale, che il giocatore può cambiare. La 180/2026 resta la legge di partenza, come in Legge elettorale vigente (01). Termini nuovi in `CONTEXT.md`: coalizione, richiesta, intesa, disponibilità, crisi, rimpasto, opposizione.

**Coalizione pre-elettorale.** La si dichiara un turno prima del voto e il giocatore è il candidato Presidente del Consiglio (la 180 vuole un candidato unico per coalizione, `docs/ricerca/modelli/sistemi-elettorali.md:69`). È una mossa strategica sui sondaggi. Ogni partito confronta il valore atteso delle coalizioni possibili sugli stessi sondaggi rumorosi del giocatore: seggi attesi, più probabilità di governare, meno distanza di posizione, meno intesa bassa se è già alleato. Entra in quella che gli rende di più. La sua **disponibilità** si mostra con la causa in una riga. Anche un alleato di governo può cambiare campo. I piccoli vicini al 3% sono attratti dal ripescaggio.
- **Con la 180** la dichiarazione è obbligatoria. I seggi premio, assegnati a liste comuni, si dividono nell'accordo pre-voto: per default in proporzione ai sondaggi, e il giocatore può cedere posti delle liste comuni per attirare un partito, pagandoli con seggi propri.
- **Con una legge senza premio né soglie di coalizione** la dichiarazione è facoltativa. Vale come impegno politico: romperla dopo il voto costa intesa e sondaggi.

**Dopo il voto.** Con il premio si spartiscono solo i dicasteri. Senza premio si tratta anche l'allargamento ad altri partiti. Se la coalizione non arriva alla maggioranza in entrambe le Camere, il giocatore va all'opposizione.

**Spartizione dei dieci dicasteri.** Propone il giocatore. Ogni alleato ha una **richiesta** di Gamson: `0,07 + 0,79 × quota dei seggi che porta alla maggioranza`, misurata in punti di peso dei dicasteri (`docs/ricerca/modelli/formazione-di-coalizioni.md:21`, :30). I pesi sono provvisori, per esempio Economia 3, Interno ed Esteri e Difesa 2, gli altri 1, perché quelli di Warwick-Druckman non sono catturati (:29). Con un'offerta sotto la richiesta l'alleato accetta ma parte con meno intesa; sopra la richiesta parte con di più. Non c'è nessun premio al formateur oltre la presidenza del consiglio (:20). I **tecnici** occupano dicasteri della quota del partito del giocatore, senza altro tetto.

**Intesa** (0-100, una per partito alleato), costruita come la soddisfazione del ministro (ticket 04).
- Scende con: bassa soddisfazione media dei suoi ministri, dimissioni chieste a un suo ministro (un colpo secco), norme approvate lontane dalla sua posizione, quota sotto la richiesta, calo nei sondaggi mentre sta al governo, rottura dell'impegno pre-elettorale.
- Torna piano verso una base, e ogni variazione si mostra con la sua causa.
- Sotto 40 l'alleato pone un ultimatum e indica la causa; sotto 25 a fine turno esce.

**Uscita di un alleato.** I suoi ministri si dimettono in blocco e i dicasteri vanno in interim (ticket 04).
- Se la coalizione ha ancora la maggioranza in entrambe le Camere: **rimpasto**, cioè i dicasteri liberati si ridistribuiscono con Gamson ricalcolata.
- Altrimenti si apre la **crisi**: un turno per allargare a un nuovo partito (con la sua richiesta) o per dimettersi. Senza maggioranza a fine turno, l'opposizione presenta la sfiducia.

**Sfiducia.** Voto palese a blocchi e deterministico, perché l'art. 94 Cost. vuole l'appello nominale (https://www.senato.it/istituzione/la-costituzione/parte-ii/titolo-iii/sezione-i/articolo-94). La coalizione vota no, tutti gli altri sì. Se passa in una Camera il governo cade e il giocatore va all'**opposizione**. La sfiducia a un singolo ministro (evento, ticket 04) passa solo se la vota anche un alleato con intesa sotto 40. Non c'è appoggio esterno né governo di minoranza nella v1.

**Dimissioni del giocatore ed elezioni anticipate.** Solo il giocatore le provoca, dimettendosi. Il Capo dello Stato è un'istituzione a regole, non un personaggio: controlla se senza il partito del giocatore esiste un'altra maggioranza, di partiti contigui sull'asse, in entrambe le Camere. Se esiste nasce quel governo (ribaltone) e il giocatore va all'opposizione. Altrimenti scioglie le Camere e si vota il turno dopo. Il voto anticipato diventa così una scommessa sui sondaggi (`docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:47`).

**Legge elettorale.** La 180 è quella di partenza. Il giocatore può cambiarla con una legge ordinaria (iter del ticket 08), scegliendo da un menu di parametri: soglia di lista, soglia di coalizione, premio e soglia per ottenerlo, tetto, coalizioni obbligatorie, a chi va il premio. La 180 è un'impostazione già pronta. La nuova legge vale dal voto successivo. Il dettaglio va nel ticket nuovo «Legge elettorale come misura».

Soglie, pesi e costi sono provvisori e vanno al bilanciamento. Effetti sulla mappa:
- destinazione corretta: ogni perdita del governo (sfiducia, sconfitta elettorale, ribaltone) porta all'opposizione;
- ticket nuovi: «Opposizione» e «Legge elettorale come misura»;
- la nebbia dei seggi premio è risolta qui;
- appoggio esterno e governo di minoranza vanno fra i punti di aggancio.
