Type: grilling
Status: resolved
Blocked by: 08
Assignee: maeurong
Label: wayfinder:grilling
Map: ../map.md

## Question

Che scala e che soglia di pericolo ha ciascuno dei tre tracciati a caselle, e da dove vengono i numeri? [Fondamenta visive](08-fondamenta-visive.md) ha fissato la forma: dieci caselle, a sinistra la caduta, uno stacco dopo la zona di pericolo. Le scale del prototipo sono proposte: consenso dal 25 al 75% a 5 punti per casella, pericolo sotto il 40; maggioranza sul margine più stretto fra le due Camere, da −9 a +21 seggi a 3 per casella, pericolo sotto +3; conti sullo spread rovesciato, da 500 a 0 punti base a 50 per casella, pericolo sopra 350.

Da decidere: gli estremi e il passo di ogni tracciato; se la soglia è un numero fisso della UI o una manopola del core che la Vista deve portare (in quel caso è un rimando, non un dato della UI); come si mostra un valore fuori scala (spread a 1.000, margine a −116 nella Vista 05); se la maggioranza mostra la Camera più debole o tutte e due.

Si parte dalle manopole in `dati/base` e da come il core fa cadere un governo per ciascuna delle tre vie; il passacarte `serie` (`crates/sim-core/src/partita.rs:2600`) porta già consenso, margini e spread per turno. Una scelta che riguarda cosa si vede va a Mario sul prototipo `prototipi/08-fondamenta.prototype.html`, non a parole.

### Premesse dal 12/11/2026

[Cruscotto all'opposizione](28-cruscotto-all-opposizione.md) ha deciso cosa mostrano i tracciati quando il giocatore non governa: il consenso del suo partito; «per farlo cadere» al posto della maggioranza (i sì che mancano alla sfiducia al governo, dall'anteprima, il minimo fra le due Camere; si riempie quando il governo è fragile); al turno zero i seggi attesi dalla proiezione contro la metà più uno della coalizione dichiarata. Servono scala e soglia anche per questi due. Sulla scala di prova del consenso, dal 25 al 75%, un partito d'opposizione fra il 5 e l'11% resta sempre a zero caselle.

### Premessa dal ticket 30 (12/11/2026)

[Eredità](30-eredita.md): a partita finita i tracciati restano fermi in grigio all'ultimo voto (il consenso è la quota presa, i seggi zero), senza pericolo acceso. Servono scala e soglia anche per questo stato, o la scelta di mostrarli vuoti.

## Answer

Deciso con Mario il 12/11/2026 in due giri: il primo a parole, sul significato della soglia e della scala; il secondo su un prototipo navigabile, [31-tracciati.prototype.html](../prototipi/31-tracciati.prototype.html), con otto stati: sette dalle Viste campione (03, 08, 05, 06 con [viste/28-opposizione.json](../viste/28-opposizione.json), 01, [viste/30-eredita.json](../viste/30-eredita.json)) e uno di prova dichiarato, lo spread a 1.000 pb. Si apre sulle varianti scelte.

### Da dove vengono i numeri

Dal core, letto prima di decidere:

- **Maggioranza:** la crisi si apre quando la coalizione perde la maggioranza in una qualunque delle due Camere (`crates/sim-core/src/partita.rs:3462-3467`); con un margine da zero in su la maggioranza tiene.
- **Conti:** lo spread sta fra 60 e 1.000 pb (`spread_minimo`, `spread_massimo` in `dati/base/manopole.toml:317-329`); le Aste scoperte scattano a `soglia_crisi_debito`, 550 pb (`manopole.toml:1542`, `partita.rs:397-425`), soglia compresa.
- **Consenso:** nessuna regola fa cadere il governo per il consenso; pesa al voto. La Legge 180 dà il premio dal 42% e taglia le liste sotto il 3% (`dati/base/elezioni.toml:286-295`): tutte e due sono disposizioni che la Vista porta già.

### Regola comune

- **Lo stacco segna una regola del core**, non un preavviso: a sinistra la regola è scattata, o scatta alla chiusura del turno. Nessun numero inventato dalla UI; quanto manca lo dicono il numero e la variazione accanto.
- **Sempre dopo la terza casella**, in tutti i tracciati. Il passo per casella è fisso e gli estremi si ancorano alla soglia: se la legge elettorale cambia la soglia, la scala scorre.
- **Una casella è piena se il valore ne raggiunge il limite basso**: un valore esattamente alla soglia sta fuori dal pericolo dove la regola lo salva (margine 0, consenso al 42%, partito al 3%) e dentro dove la regola scatta già (spread a 550).
- **Fuori scala, il segno:** un ◂ prima della prima casella quando il valore va oltre la caduta, rosso se è il lato del pericolo; un ▸ grigio dopo l'ultima quando va oltre il pieno. Il numero accanto dice il valore vero.

### I tracciati

| Tracciato | Quando | Passo | Scala | Stacco | Rosso |
|---|---|---|---|---|---|
| Consenso della coalizione | al governo | 4 punti | 30→70% con la Legge 180 | soglia del premio della legge in vigore; 50% con una legge senza premio | quando la `proiezione` non dà alla coalizione la metà più uno in una delle due Camere |
| Consenso del partito | all'opposizione, al turno zero | 1 punto | 0→10% | soglia di lista della legge in vigore (3%) | sotto lo stacco: fuori dal Parlamento |
| Maggioranza | al governo | 3 seggi | −9→+21 | 0, sulla Camera più debole, col ramo scritto accanto («+14 Senato») | sotto lo stacco |
| Seggi attesi | al turno zero | 3 seggi | −9→+21 | 0, come la maggioranza, dalla `proiezione` contro la metà più uno della coalizione dichiarata | sotto lo stacco |
| Per farlo cadere | all'opposizione | 15 seggi | 150→0 sì che mancano, pieno quando la sfiducia passa | nessuno | mai: per chi è all'opposizione la caduta del governo non è un pericolo |
| Spread, rovesciato | sempre | 75 pb | 775→25 | 550 pb | da 550 in su |

- **Il consenso al governo ha stacco e rosso indipendenti.** Nella Vista 03 tre partiti al 38,5% stanno sotto il premio, ma la proiezione dà 225 seggi su 400: con il rosso al premio un governo che sta bene sarebbe rosso dal secondo turno. Lo stacco dice dove sta il premio, il rosso dice «se si votasse oggi perderesti». Nella Vista 08 (30,1%, 165 seggi) si accende.
- **Senza proiezione** (`null`, solo se il riparto rifiuta i voti) il consenso al governo non si accende: assunzione proposta nel riepilogo e confermata da Mario.
- **Il partito al 13,4% del turno zero** riempie la scala e prende il ▸: la scala del partito misura la distanza dalla soglia di lista, non la forza.
- **A partita finita** i tracciati restano fermi in grigio all'ultimo voto, con le stesse scale e senza rosso ([Eredità](30-eredita.md)): il 2,4% sta sotto lo stacco e dice da solo perché la partita è finita.
- **Tono grave:** niente di nuovo; il rosso diventa inchiostro come ovunque ([Fondamenta visive](08-fondamenta-visive.md)).

### Dati e rimandi

- Soglia del premio, soglia di lista e premio sì o no: dalle disposizioni della legge elettorale in vigore (`Vista.disposizioni`, `soglia_premio`, `soglia_lista`, `premio`). Proiezione: `Vista.proiezione`. Margini: `Vista.seggi`. Sì che mancano: dall'anteprima della sfiducia ([Cruscotto all'opposizione](28-cruscotto-all-opposizione.md)).
- **Un rimando al core, PR delle esposizioni, in 11g:** durante la partita la Vista non porta `soglia_crisi_debito`. La porta solo l'avviso dello scenario (`Avviso.crisi_del_debito.soglia_pb`) e la causa dell'evento quando scatta. Fino ad allora lo stacco dello spread non ha un dato da cui leggersi.

### Scartato

Il preavviso prima della regola (la zona di pericolo del prototipo delle fondamenta, margine +3 e spread 350); estremi fissi per tracciato; il rosso del consenso al premio; il valore fuori scala senza segno; le due Camere su due file sottili; «per farlo cadere» a 3 seggi per casella, vuoto finché il governo non è quasi caduto; i tracciati vuoti a partita finita.
