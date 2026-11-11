Type: grilling
Status: open
Blocked by: 08
Assignee: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Che scala e che soglia di pericolo ha ciascuno dei tre tracciati a caselle, e da dove vengono i numeri? [Fondamenta visive](08-fondamenta-visive.md) ha fissato la forma: dieci caselle, a sinistra la caduta, uno stacco dopo la zona di pericolo. Le scale del prototipo sono proposte: consenso dal 25 al 75% a 5 punti per casella, pericolo sotto il 40; maggioranza sul margine più stretto fra le due Camere, da −9 a +21 seggi a 3 per casella, pericolo sotto +3; conti sullo spread rovesciato, da 500 a 0 punti base a 50 per casella, pericolo sopra 350.

Da decidere: gli estremi e il passo di ogni tracciato; se la soglia è un numero fisso della UI o una manopola del core che la Vista deve portare (in quel caso è un rimando, non un dato della UI); come si mostra un valore fuori scala (spread a 1.000, margine a −116 nella Vista 05); se la maggioranza mostra la Camera più debole o tutte e due.

Si parte dalle manopole in `dati/base` e da come il core fa cadere un governo per ciascuna delle tre vie; il passacarte `serie` (`crates/sim-core/src/partita.rs:2600`) porta già consenso, margini e spread per turno. Una scelta che riguarda cosa si vede va a Mario sul prototipo `prototipi/08-fondamenta.prototype.html`, non a parole.

## Answer
