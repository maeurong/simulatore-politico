Type: grilling
Status: resolved
Blocked by: 20, 23
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Cosa cambia l'editor dello scenario su misura (`docs/ricerca/sintesi/decisioni.md:54`: «l'Italia di oggi più un editor dei parametri iniziali sullo stesso scenario»)? Quali grandezze del turno zero sono modificabili (debito, ricchezza, spread, disoccupazione, demografia, quote e posizioni dei gruppi, partiti, legge elettorale, disposizioni), con quali limiti, e come si tiene coerente lo stato quando se ne cambia una sola (per esempio: più debito senza toccare il disavanzo né gli interessi)? Si cambiano i valori uno per uno o con pochi cursori sintetici? Lo scenario modificato è un file di dati, come vuole il modding solo dati di Stack e linguaggio del core (11)?

Ingresso da Partenza della partita (20): uno scenario in cui nessuna coalizione col giocatore supera il 42% nei sondaggi attesi resta lecito, ma l'editor lo segnala con un avviso.

## Answer

Grilling con Mario del 18/10/2026, due giri, tutte le raccomandazioni accettate, quadro finale confermato. Termine nuovo in `CONTEXT.md`: scenario.

**Perimetro: struttura fissa, stato modificabile.** Lo «stesso scenario» della decisione del 16/10 (`docs/ricerca/sintesi/decisioni.md:54`) è la struttura. Restano fissi la data del turno zero (ottobre-dicembre 2026), le istituzioni, i 10 dicasteri, le 20 regioni, i 360 gruppi, i sette posti dei partiti IA con vocazioni e regole di movimento di default (18) e il governo uscente di centrodestra e destra (20). Si modificano lo stato economico e di finanza pubblica, la demografia, le posizioni dell'elettorato, la legge elettorale di partenza e i valori delle disposizioni.

**Coerenza: solo ingressi.** Lo scenario contiene solo grandezze indipendenti. Tutto ciò che il motore sa calcolare si ricalcola al caricamento con le formule del gioco: interessi = costo medio × debito, spread dalla curva non lineare (tarata perché lo scenario di base dia 79-89 pb), disavanzo, bilancio 2027 ricondotto al tetto UE con la regola di Partenza della partita (20), sondaggio, parlamento dal voto simulato. «Più debito con gli stessi interessi» non si può scrivere: per pagare meno interessi si abbassa il costo medio, che è un ingresso. La valence è un ingresso, tarata una volta sullo scenario di base sulle intenzioni di voto reali (20); nello scenario su misura il sondaggio di partenza è derivato, quindi il cursore di orientamento lo sposta davvero. La valence del singolo partito si cambia solo nel file.

**Forma: cursori nell'interfaccia, valori nel file.**

| Cursore | Muove (ingressi) | Si ricalcola (derivati) |
|---|---|---|
| Debito (% del PIL) | livello del debito, costo medio invariato | interessi, disavanzo, spread, bilancio 2027 |
| Ricchezza (PIL pro capite, ± %) | PIL, redditi dei gruppi, budget dei dicasteri in proporzione; scaglioni, soglie e salario minimo restano in euro | quote dei gruppi fra le fasce, gettito, disavanzo |
| Disoccupazione (punti) | tasso di ogni regione spostato degli stessi punti, divario invariato, insieme a u* | quote dei gruppi per condizione, NASpI, gettito |
| Inflazione di partenza | lo shock in corso (4,2% nello scenario di base) | tasso BCE (Taylor), panieri |
| Età della popolazione (età mediana ± anni) | piramide per coorti | quota di pensionati, spesa per pensioni, quote dei gruppi |
| Orientamento dell'elettorato (± fino a 1 punto) | istogrammi di tutti i gruppi spostati sull'asse | sondaggio, parlamento atteso |
| Menu legge elettorale | impostazioni pronte e gradini (13) | parlamento atteso |
| Menu disposizioni | gradini (16) | effetti sugli indicatori |

La disoccupazione sposta anche u*, altrimenti Okun e congiuntura la riportano al valore reale in pochi turni. La ricchezza tiene ferme le soglie in euro, l'unico modo in cui «più ricca» cambia la partita e non solo le cifre. Budget dei singoli dicasteri, aliquote, mix regionale e divario Nord-Sud si cambiano solo nel file di testo, senza interfaccia.

**Limiti e avvisi.**
- *Limite*: l'intervallo in cui il modello è tarato, dichiarato per ogni ingresso nel file base. Il cursore si ferma lì; un file scritto a mano fuori intervallo è rifiutato al caricamento con il nome del valore.
- *Avviso*, scenario lecito ma segnalato: nessuna coalizione col giocatore supera il 42% nei sondaggi attesi da qualche posto (20); oppure una situazione o una crisi è già accesa al turno zero (per esempio un debito che apre la crisi del debito al primo turno, 15).

**Sequenza e file.** Nuova partita → scelta dello scenario («Italia di oggi», oppure uno su misura nuovo o da file) → fondazione, con risultato atteso e coalizioni calcolati sullo scenario scelto → turno zero. Lo scenario su misura è un file di dati con un nome, fra i file del giocatore, coerente col modding solo dati (11); lo scenario di base è immutabile e si spedisce col gioco. Il seme non sta nello scenario, appartiene alla partita. L'eredità confronta gli indicatori con il turno zero dello scenario giocato. Scelta di strumento dichiarata: il file su misura contiene solo le differenze dalla base (valori dei cursori più le singole sostituzioni); i salvataggi restano snapshot, quindi un aggiornamento dei dati base non tocca le partite in corso.

**Disposizioni cambiate dall'editor.** Il valore nuovo si attribuisce a una legge del governo uscente del 2026, e il valore precedente diventa il valore dello scenario di base; per la legge elettorale, partire dal proporzionale al 3% vuol dire che abrogarla riporta alla 180. Cosa faccia davvero l'abrogazione lo decide Abrogazione e gradini di partenza (24).

**Riepilogo dal vivo.** A ogni cambio il core, puro e senza I/O (11), ricalcola e mostra consenso atteso per posto, maggioranza attesa, spread, interessi, disavanzo e avvisi. Cifre esatte, perché lo scenario lo scrive il giocatore e l'informazione imperfetta comincia con la partita; il sondaggio è quello atteso, senza rumore.

Effetti sulla mappa:
- nessun ticket nuovo;
- Abrogazione e gradini di partenza (24): riceve come ingresso la regola sulle disposizioni cambiate dall'editor;
- Costo del governare (nebbia): gli intervalli di taratura dei sei cursori;
- Salvataggi (nebbia): scenario su misura come file di differenze, seme nella partita;
- Mappa delle 20 regioni (nebbia): il divario Nord-Sud come possibile cursore.
