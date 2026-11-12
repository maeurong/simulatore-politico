Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta l'anteprima della misura, il mobile appeso (storia 156)? Costo esatto, effetti a fasce, ritardo, gruppi colpiti, linee dei partiti e forbice del voto (storia 55); il perimetro evidenziato per una disposizione territoriale (storia 141); l'oscillazione smorzata solo a un gesto del giocatore, mai in loop; il test della misura comprato con la cassa (storia 123, `VistaTest`) e come cambia ciò che si legge. Si apre a lato: in Aula per una legge, sul cruscotto per una proposta e per il decreto di un evento. Varianti per le quattro forme della misura e per il tono grave.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario il 12/11/2026 in due giri su un prototipo navigabile: [20-anteprima.prototype.html](../prototipi/20-anteprima.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Ospite: l'Aula in forma D ([Aula e calendario](19-aula-e-calendario.md)) e un cruscotto ridotto da [Cruscotto al governo](17-cruscotto-al-governo.md). Dati: [viste/20-anteprima.json](../viste/20-anteprima.json), che `genera/` aggiunge giocando sul 03 (seme 4): anteprime vere di salario minimo a 7, 9 e 11 €/h, ZES a 0, 0,75 e 2,3 miliardi, autonomia differenziata, numero dei parlamentari, salario minimo con le pensioni, proporzionale al 3%, proposta del ministro sul nucleare, decreto di `Accogli`; il decreto d'evento sul codice degli appalti (seme 4 turno 2, e seme 7 turno 8 in tono grave); il test comprato su salario minimo e ZES; tre rifiuti veri.

### Forma: D · Cartellino col prima e dopo

Nata dal primo giro: Mario ha scelto A, ma voleva vedere il prima e dopo di C.

- **Si apre nel dettaglio a lato**, alla sua larghezza, al posto della bozza o della riga, con «‹ Torna alla bozza» (o alla misura, o alla riga). Si apre da «Anteprima della misura ›» sotto la bozza e nel dettaglio di una misura in Aula, e da «Anteprima ›» sulla riga di una proposta o del decreto d'evento nel cruscotto.
- **Un cartellino appeso a uno spillo**: un foglio solo, col filo in testa. Dentro, in ordine: forma e titolo; la tabella **cosa cambia · ora · con la misura** (una riga per disposizione, poi la spesa all'anno, «saldo zero» se non cambia i conti, o l'importo per un decreto di fondi); gli **effetti stimati** con quando («subito», «dopo 4 turni», «dopo 40 turni · 10 anni») e quanto (tre caselle, migliora o peggiora); chi ha fatto la stima e la sua capacità; i gruppi toccati col bottone del test; dove vale; il voto con l'emiciclo di Camera e Senato e le linee; i seggi per una legge elettorale; una riga sulla forma (deliberazioni, conversione, rinvio).
- **Il mobile oscilla intero, solo a un gesto**: aprire l'anteprima, cambiare un gradino della bozza con l'anteprima aperta, comprare il test. Oscillazione smorzata con le Web Animations, attorno allo spillo, mai in loop; la durata segue la velocità (a 2× è la metà), e col movimento spento o nel tono grave non c'è.
- **Il test della misura** sta sotto i gruppi toccati: «Compra il test · 20.000 €», con la cassa del partito accanto. Prima del test si legge chi la misura tocca, non come reagirà; dopo, al posto dei gruppi, una tabella delle sei condizioni con la reazione al governo fra due turni (a favore, contro, indifferenti, con la fascia) e i gruppi toccati segnati. Il secondo test del turno è un rifiuto sul posto col testo del core («già comprato questo turno: test di una misura»).
- **Il perimetro** di una disposizione territoriale si legge su una mappa delle regioni con quelle dove vale piene, e l'elenco dei nomi accanto. Nell'app la mappa ISTAT di [Mappa SVG delle regioni](02-mappa-svg-delle-regioni.md); il prototipo usa tessere.
- **Tono grave**: lo stesso cartellino con le regole delle fondamenta, fermo.

### Decisioni sui casi

- **Le quattro forme** stanno nello stesso cartellino; cambiano l'occhiello e la riga in fondo. Una costituzionale dice le due deliberazioni e i due terzi (non ha effetti sulle grandezze: «cambia le istituzioni»), un'elettorale aggiunge i seggi in più o in meno per partito, un decreto dice che vale subito e decade se non convertito, con la forbice sulla conversione. La proposta di un ministro dice che, accolta, entra in calendario come legge.
- **La stima è del ministro, e lo si dice**: «Stima di Luca Romano, ministro Ambiente ed Energia: capacità 1 su 5». La UI non scrive quanto spesso sbaglia, perché verrebbe da una manopola che la Vista non porta. Senza ministro proponente (autonomia, costituzionale, elettorale) la stima è neutra e lo dice.
- **Una bozza che il core rifiuta** si legge sul posto col testo del core, e col rimando alla misura in esame se c'è: la decontribuzione al 30% nel 03 è «già in esame» perché il Centro l'ha portata nel posto riservato al 15%.
- **All'opposizione** resta il rifiuto del core fino ad `anteprima_proposta` in 11g, già deciso in [Aula e calendario](19-aula-e-calendario.md).

### Rimandi

- **Anteprima di una misura in calendario — al core.** Una misura in esame porta linee e forbice (`VoceMisura`) ma non costo, effetti né gruppi, e `Partita::anteprima` la rifiuta («è già in esame»). Mario sceglie il rimando: `VoceMisura` porta costo, effetti e gruppi come `Anteprima`, voce minore della PR delle esposizioni del piano di core «esito e Vista completa»; la UI la mostra in 11g. Fino ad allora, «Anteprima della misura ›» su una misura in calendario mostra il rifiuto.
- **Norme del decreto d'evento — al core.** Un evento aperto ammette un decreto su certe disposizioni (`DatoEvento.norme` e `dicasteri`, `crates/sim-core/src/eventi/mod.rs:101-106`), ma `VoceEvento` non le porta. Mario sceglie il rimando: `VoceEvento` porta le disposizioni che il suo decreto ammette, e la riga dell'evento nel cruscotto le offre pronte, con «Emana il decreto» e «Anteprima ›». Stessa PR delle esposizioni, stessa fetta.
- Tutti e due scritti il 12/11 nella spec di core «esito e Vista completa» (sezioni «Anteprima di una misura in calendario» e «Norme del decreto d'evento», con gli oracoli) e nel suo piano, Task 16b.

### Guscio e dati

Nessun passacarte nuovo: `anteprima`, `anteprima_decreto` e `anteprima_accogli` il guscio li ha già (`app/src-tauri/src/lib.rs:379-392`); il test passa da `comanda` con `Compra { TestMisura }`, e si legge in `Vista.acquisti.test_misura`. Il perimetro si legge da `Vista.disposizioni[].perimetro` per le disposizioni della bozza, i nomi dei ministri e la capacità da `Vista.ministeri`.

### Stranezze notate

- La stima del ministro non è monotona: salario minimo a 7 €/h «medio», a 9 «molto», a 11 «medio» sulle regole del lavoro (Simone Ricci, capacità 2). È il rumore voluto della capacità, e si vede cambiando gradino.
- Il test della ZES a 2,3 miliardi dà gli autonomi, i soli toccati, «contro» il governo, poco; quello del salario minimo dà i dipendenti privati, i soli toccati, «contro», molto. Il test misura il giudizio sui partiti di governo, non il beneficio.
- Il test si compra anche su una misura già in esame (`bozza_test` non guarda il calendario), mentre l'anteprima la rifiuta.
- La legge su due disposizioni deve stare in un dicastero solo: salario minimo e ZES insieme sono un rifiuto del core. Salario minimo con le pensioni passa.
- Nel 03 il fondo di riserva è 0 € e l'evento costa 1 miliardo; nel tono grave (seme 7) l'emergenza costa 5 miliardi.

### Scartato

A da sola (il cartellino con gli effetti a elenco: mancava il prima e dopo); B · Mobile a fili (il ritardo come lunghezza del filo; con quattro pezzi il dettaglio è troppo stretto, e 40 turni non stanno in scala); C da sola (la tabella ferma, con le sole righe cambiate che si muovono); il dettaglio che si allarga a metà pagina; il decreto d'evento a prova e rifiuto su ogni disposizione; l'anteprima di una misura in calendario solo prima di presentarla.
