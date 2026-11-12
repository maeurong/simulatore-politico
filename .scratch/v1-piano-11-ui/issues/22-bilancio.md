Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatto il passaggio del bilancio? La bozza con le cifre nominali dell'anno prima e «indicizza all'inflazione» per tutto o per voce, col costo esatto (storia 40); le aliquote delle quattro imposte, il budget di ogni dicastero diviso in funzionamento e margine, il fondo di riserva e il saldo contro il tetto UE (storie 39 e 43), con la perdita reale accanto (storia 41); gli emendamenti degli alleati (storia 44); la presentazione e il riepilogo del bilancio presentato; l'esercizio provvisorio (storia 45). Il piano strutturale riveduto (storia 51) e l'avviso del bilancio ricondotto al tetto del MES hanno qui il loro posto, con un rimando ai piani di core. Sezione barrata fuori da ottobre-dicembre e quando il giocatore non governa.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [22-bilancio.prototype.html](../prototipi/22-bilancio.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: [viste/22-bilancio.json](../viste/22-bilancio.json), che `genera/` aggiunge giocando. La bozza del 04 (seme 4, 2027T4) prima di presentarla, coi riepiloghi veri di `riepilogo_bilancio`: per la bozza, per «indicizza» su tutto, sulle soglie Irpef e su ogni dicastero da solo, per l'Iva al 24%, il primo scaglione al 21%, il margine della Sanità raddoppiato dal funzionamento o in disavanzo, e il rifiuto dell'Iva al 95%. Poi il bilancio presentato con gli emendamenti di Centrodestra e Destra e il costo di accoglierli, gli emendamenti decisi (uno accolto e uno respinto), il bilancio ripresentato indicizzato per tutto, l'esercizio provvisorio (seme 1, 2030T1), l'opposizione in ottobre-dicembre e il rifiuto fuori stagione.

### Forma: B · Bilancia

- **In testa, a tutta pagina, il giogo:** la crescita della spesa netta sull'anno prima contro il tetto UE dell'anno, la riga rossa sul tetto e la frase della distanza («dentro il tetto: restano 1,0 punti»). Accanto, il costo sulla bozza: spesa in più e gettito perso.
- **Sotto, entrate e spesa a fronte.** A sinistra le aliquote in numeri grandi, coi tasti − e + sul posto: gli scaglioni Irpef con la soglia reale e «indicizza le soglie» col gettito che costa, poi Iva, Ires, contributi e il fondo di riserva. A destra i dieci dicasteri come barre divise in salari, acquisti, margine e resto, con la spesa annua, la perdita reale e «Indicizza» col suo costo; in testa alla colonna «Indicizza tutto» col costo.
- **Il dettaglio a lato** è la riga scelta. Dicastero: ministro, spesa, salari e acquisti, margine, resto, perdita reale, la frase della storia 41 («Sanità: −1,0% reale dal bilancio 2027»), «Indicizza» e «Allarga il margine» dal funzionamento o in disavanzo. Scaglione: aliquota, soglia, soglia reale. Fondo: la cifra e gli eventi aperti col loro costo. Senza riga scelta, il lato tiene gli emendamenti e la presentazione.
- **Le modifiche** passano dal core a ogni passo: `indicizza`, poi `riepilogo_bilancio` sul bilancio in memoria. Un bilancio non ammesso mostra il rifiuto del core al posto del giogo, e «Presenta» si spegne.
- **«Presenta la legge di bilancio ›»** diretto, senza conferma: si può ripresentare nel turno. Sotto, cosa succede se cade: esercizio provvisorio da gennaio, o le dimissioni se si è già in esercizio provvisorio.
- **Dopo la presentazione** la pagina diventa la presentata: il riquadro nero con la forbice di Camera e Senato e la maggioranza che serve, gli emendamenti a lato con Accogli · Respingi, il costo «se accogli» e il default scritto («Senza scelta: resta aperto, si vota il testo com'è», come in [Aula e calendario](19-aula-e-calendario.md)). Sulle barre, il foglietto dell'alleato che chiede l'indicizzazione di quel dicastero. «Modifica e ripresenta ›» riapre la bozza in memoria; la nuova sostituisce la presentata e gli emendamenti accolti si riapplicano.
- **Esercizio provvisorio:** una riga d'avviso in testa («valgono le cifre del 2029, nessuna misura con costo»), e la bozza è dell'anno in corso.
- **Piano strutturale riveduto e programma del MES:** due riquadri a tratteggio in fondo alle entrate, col rimando. Il piano strutturale lo riempie la prima fetta dopo il piano di core strutturale ([Fette](10-fette.md)); col MES in corso la presentazione oltre il tetto è un rifiuto sul posto col testo del core, e lo stato del MES lo porta la PR delle esposizioni in 11g.
- **Tono grave:** le regole delle fondamenta.

### Decisioni sui casi

- **Funzionamento tagliato:** allargare il margine togliendo al funzionamento non cambia il riepilogo del core: costo zero, perdita reale uguale alla bozza. Mario sceglie che la UI lo scriva da sé, senza rimando: sotto la barra del dicastero «acquisti −2,5 mld € sulla bozza · la qualità del servizio sanitario peggiora con ritardo». Le due cifre sono quelle della bozza e quelle del bilancio in memoria; la regola è quella della storia 42 e del glossario.
- **All'opposizione la sezione resta barrata** («Non governi: il bilancio lo presenta il governo»), come in [Ossatura e navigazione](07-ossatura-e-navigazione.md). Il riepilogo del bilancio del governo, che `riepilogo_presentato` dà a chiunque, va nel dettaglio dell'Aula sulla riga «Legge di bilancio», dove l'opposizione sceglie la linea. `riepilogo_bilancio` all'opposizione è un rifiuto («non governi: sei all'opposizione»).
- **Sezione barrata fuori stagione:** il core rifiuta la bozza fuori da ottobre-dicembre e dall'esercizio provvisorio («fuori stagione: si presenta in ottobre-dicembre o nell'esercizio provvisorio»), e il sommario lo dice.

### Rimandi

- **Cifre del bilancio in aula — al core.** La Vista ha il bilancio in vigore e i booleani `presentabile` e `presentato`, non le cifre della legge presentata. Accolto l'emendamento di un alleato, il core indicizza i suoi dicasteri e la UI non sa più quanto va a ciascuno: ha solo il riepilogo. Mario sceglie il rimando: `VistaBilancio.in_aula: Option<Bilancio>`, pubblico come il riepilogo. Fino ad allora la pagina della presentata mostra le cifre mandate dalla UI e la perdita reale del riepilogo.
- **Dicasteri chiesti da un emendamento — al core.** Per dire «se accogli: +981,9 mln € · spesa netta +0,8%» prima della scelta, la UI chiama `indicizza` sui dicasteri chiesti e poi `riepilogo_bilancio`. Il core li tiene nella richiesta, ma `VoceEmendamento` porta solo testo e motivo. Mario sceglie il rimando: `VoceEmendamento.dicasteri`, gli id, vuoto sugli emendamenti delle leggi. Fino ad allora l'emendamento si decide senza il costo, come in Aula.
- Tutti e due scritti il 12/11 nella spec di core «esito e Vista completa» (sezione «Bilancio in aula») e nel suo piano (Task 16d), PR delle esposizioni, in 11g.

### Guscio e dati

Nessun passacarte nuovo: `bozza_bilancio`, `indicizza`, `riepilogo_bilancio` e `riepilogo_presentato` sono già di 11d ([Contratto del guscio](12-contratto-del-guscio.md)); presentazione ed emendamenti passano da `comanda` (`PresentaBilancio`, `Emendamento`). Il costo «se accogli» è `riepilogo_bilancio` su `in_aula` indicizzato sui `dicasteri` dell'emendamento, tutti e due dai rimandi. Il bilancio in composizione vive solo in memoria, come deciso nel Contratto del guscio.

### Stranezze notate

- La perdita reale è la stessa per tutti i dicasteri (1,0% nel 2027T4): la bozza copia le cifre del bilancio in vigore, tutte allo stesso livello dei prezzi.
- Ripresentando il bilancio indicizzato per tutto dopo aver respinto la Destra, l'intesa persa resta (da 59,9 a 54,9) anche se ora i suoi dicasteri sono in pari.
- Nell'esercizio provvisorio del seme 1 (2030T1) la bozza sta sul tetto: spesa netta +1,49% contro 1,5%.
- Indicizzare tutto costa 6,2 mld di spesa e 1,3 mld di gettito, e porta la spesa netta da +0,7% a +1,4% contro un tetto di 1,7%: nel 2027 si può.

### Scartato

A · Prospetto (un foglio di ragioneria unico con in vigore e proposta affiancati: il più fitto, ma il saldo sta solo a lato); C · Iter (la legge per passi: guida, ma nasconde la tabella dietro i passi); il bilancio del governo in lettura all'opposizione.
