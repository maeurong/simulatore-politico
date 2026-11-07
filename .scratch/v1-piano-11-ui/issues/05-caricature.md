Type: research
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:research
Map: ../map.md

## Question

Come si producono le caricature di ministri, tecnici e leader inventati (storie 103, 154), coerenti con lo stile «Il Settimanale» e con `docs/ricerca/estetica/grafica-italiana-giocosa.md` (metodo per disegnare ministri inventati)? Quante ne servono (10 ministri più rose di 3 e tecnici, 7-10 leader che cambiano dopo una sconfitta), SVG procedurale a parti combinate col seme, disegno a mano, o nessuna caricatura con un ripiego tipografico; peso, licenze, regola che nessun volto somigli a un politico reale (storia 159), sparizione nel tono grave.

## Answer

Pagina: [caricature-di-personaggi-inventati](../../../docs/ricerca/estetica/caricature-di-personaggi-inventati.md). Raccomandazione, da validare nei prototipi:

- Kit SVG nostro, nessuna dipendenza: le parti del prototipo (`testa()` e `ACC` in `.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html:590-608`) estese a circa 30, combinate con un hash di `ruolo:partito:nome` calcolato nella UI (la Vista porta solo il nome). Il dicastero si legge dall'accessorio, lo stato dalla bocca. Una testa pesa 0,5-0,7 kB.
- Nessun capello, occhiale o barba nel kit: un volto geometrico senza lineamenti non può somigliare a una persona reale (Rhodes, Brennan e Carey 1987). La prova di riconoscimento si fa una volta sul campione.
- Rosa di 3: le collisioni dell'hash sono probabili, serve uno spostamento deterministico dentro il gruppo.
- Tono grave: al posto della caricatura, le iniziali in un cerchio d'inchiostro nello stesso riquadro.
- DiceBear 10 scartato: lo stile proprio si crea con Figma (non locale), il core pesa 47 kB gzip, gli stili hanno licenze diverse. Multiavatar escluso per la licenza.
- Aperti: il prototipo tiene la testa nel borsino anche nel tono grave (riga 712), mentre la spec (riga 400) fa sparire le illustrazioni → va in [Fondamenta visive](08-fondamenta-visive.md). Il bacino dei nomi (`dati/base/elettorato.toml:11-12`) genera «Matteo Ricci» e «Andrea Romano», politici reali (storia 159): è un difetto dei dati, fuori dal piano 11.
