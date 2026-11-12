Type: prototype
Status: resolved
Blocked by: 05, 08
Assignee: maeurong
Label: wayfinder:prototype
Map: ../map.md

## Question

Come sono disegnati ministri, tecnici e leader? L'11/11/2026, guardando il prototipo delle [Fondamenta visive](08-fondamenta-visive.md), Mario ha detto che la resa grafica dei ministri non gli piace. Non ha ancora detto perché: è la prima cosa da chiedergli, davanti al prototipo aperto.

Quello che ha visto è un segnaposto: le quattro forme elementari del prototipo vincolante (`.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html:590-608`), copiate pari pari in `prototipi/08-fondamenta.prototype.html`. Il kit di circa 30 parti raccomandato in [Caricature](05-caricature.md) non è mai stato disegnato.

Da decidere, su 3-5 stili a confronto: il tratto e quanto è caricatura; da cosa si riconosce una persona dall'altra; come si leggono il dicastero (oggi l'accessorio) e lo stato del ministro (oggi la bocca); come si distinguono ministro, tecnico e leader di partito; i colori, dentro i sette inchiostri delle fondamenta o fuori.

Vincoli già fissati, da non riaprire: il riquadro è di 34 px nella riga e 84 px nella carta del ministro, e lo stile deve reggere a 34 px; nel tono grave la caricatura lascia il posto alle iniziali; nessun volto somiglia a un politico reale (storia 159: per questo [Caricature](05-caricature.md) esclude capelli, occhiali e barbe, e se lo stile nuovo li vuole la prova di riconoscimento va rifatta); SVG nostro, combinato con un hash di ruolo, partito e nome, senza dipendenze; nella rosa di tre le facce non devono coincidere.

Metodo: subagente `graphic-designer` in due passaggi, prima le varianti con l'anteprima e poi la rifinitura di quella scelta. Le varianti si mostrano a Mario dentro la riga del borsino e la carta del ministro di `prototipi/08-fondamenta.prototype.html`, coi dieci ministri delle Viste 03 e 08, non su un foglio a parte. Si parte da `docs/ricerca/estetica/caricature-di-personaggi-inventati.md` e `docs/ricerca/estetica/grafica-italiana-giocosa.md` (metodo per disegnare ministri inventati).

## Answer

Deciso con Mario il 12/11/2026 in quattro giri: il perché del no davanti al prototipo delle fondamenta; una tavola di tredici riferimenti in quattro direzioni, [32-riferimenti.prototype.html](../prototipi/32-riferimenti.prototype.html), dalla ricerca [riferimenti-per-le-facce](../../../docs/ricerca/estetica/riferimenti-per-le-facce.md); quattro varianti della direzione scelta; la rifinitura. Tutto in [32-caricature.prototype.html](../prototipi/32-caricature.prototype.html), che si apre con `?f=5`.

### Perché il segnaposto non andava

Mario: l'aspetto astratto va bene, ma le facce erano «proprio brutte da vedere» e «tutte uguali». La ricerca ne ha trovato le cause: sei pastelli fuori dai sette inchiostri; occhi e bocca sempre nello stesso punto, quindi a 34 px dieci ministri sembravano una faccia sola; nessuna silhouette; l'oggetto del dicastero sovrapposto al mento.

### Lo stile: profilo al tratto, alla Guasco

Direzione C della ricerca, con Riccardo Guasco come riferimento principale («mi piacciono moltissimo i disegni di Guasco»). È la variante 5, cioè la Linea rifinita:

- **Profilo verso destra**, disegnato a **pennino piatto**: lo spessore cambia con la direzione, le estremità del tratto si assottigliano, due pennellate si staccano alla radice del naso. Il tratto è sempre d'inchiostro.
- **Figura campita di carta**, che sui fondi colorati si stacca come un ritaglio. I piani alla Guasco (sotto la mascella, sotto il naso, l'orecchio) sono in terra chiara.
- **Due inchiostri sovrapposti** ammessi solo nelle facce, come una stampa a due passaggi: terra chiara (giallo e rosso), colletto arancio (giallo e rosso), colletto oliva (giallo e biro). Contrasto del tratto su ciascuno fra 4,9:1 e 10,3:1.
- **Leve d'identità** scelte dall'hash di `ruolo:partito:nome`: naso 6, mento 3, fronte 2, nuca 2, occhio 2 (tondo o a mandorla), colletto 6. Sono 864 combinazioni per genere; sul bacino, 8 coppie uguali su 10.296.
- **Dicastero:** un oggetto in un solo inchiostro in alto a destra. **Soddisfazione:** il fondo del riquadro, carta, giallo sotto 40 e rosso sotto 25. È il segnale più forte a 34 px.
- **Ministro, tecnico, leader:** il tecnico ha il cartellino e l'abito grigio, il leader la coccarda tricolore.
- **Lista nera rispettata:** teste calve, niente capelli, chignon, occhiali, barbe, baffi, collane di perle o fiocchi al collo.

### Donne

Mario l'ha notato sulle quattro varianti: le donne non avevano tratti femminili. Il problema si è risolto subito, perché è una leva d'identità e non ha una soluzione ovvia senza capelli.

- Tutte le donne hanno le **ciglia** (tre, piegate all'indietro) e il naso dell'hash ridotto **all'85%**. Senza la riduzione, naso a uncino e labbra rosse davano la strega.
- **Labbra rosse** sempre; **orecchino** a rombo giallo e **cappello a tesa larga** nel colore del colletto, sì o no secondo l'hash. Così le donne si distinguono anche fra loro. A 34 px il cappello si legge bene, le labbra come un punto rosso, l'orecchino appena.
- Gli uomini restano senza ciglia, labbra e cappello.

### Rosa

Se un candidato ripete naso e colletto di chi è già nella rosa, passa alla combinazione successiva dell'hash, finché ne trova una libera. Naso e colletto sono le due leve che si leggono a 34 px. È una regola della UI e il core non cambia.

### Per la fetta che porta i ministri

- **Peso:** il pennino del prototipo è fatto di quadrilateri misurati sul tracciato, circa 15 KB per faccia. In produzione si precalcola un poligono unico per ogni parte del kit.
- **Prova di riconoscimento:** il campione va mostrato senza nomi a 3-5 persone (metodo in `docs/ricerca/estetica/grafica-italiana-giocosa.md:107`) prima di fondere la fetta. Il designer ha già tolto il mento prominente, che con la testa calva e alta ricordava Mussolini.
- **Tono grave:** non cambia. Restano le iniziali ([Fondamenta visive](08-fondamenta-visive.md)).

### Dati e rimandi

- **Un rimando al core, PR delle esposizioni, in 11g: il genere del personaggio.** La Vista non lo porta. Il prototipo lo ricava da una lista di nomi di battesimo: i sei femminili del bacino (`dati/base/elettorato.toml:11`) più Silvia, leader iniziale (`dati/base/scenario.toml:307`). Una lista copiata nella UI smette di valere senza dare errori appena i dati cambiano, quindi la fonte è un campo `genere` per ministri, candidati e leader. Fino ad allora le donne si disegnano come gli uomini: è un vuoto assegnato a 11g, come gli altri ([Fette](10-fette.md)).

### Scartato

Le direzioni A (isotipo), B (macchiette di sagoma) e D (figurino a solidi); le varianti 1 (piani senza contorno, soddisfazione dalla postura, debole a 34 px), 3 (il cappello come segno del dicastero, solo 216 combinazioni) e 4 (sagoma piena, spilla illeggibile a 34 px); il tratto a spessore costante della variante 2, troppo vicino alla Linea di Cavandoli; il naso pieno arancio; un solo segno femminile per tutte; tutti e tre i segni su ogni donna, che farebbe del cappello una divisa.
