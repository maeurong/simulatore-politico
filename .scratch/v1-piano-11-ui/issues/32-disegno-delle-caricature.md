Type: prototype
Status: open
Blocked by: 05, 08
Assignee: -
Label: wayfinder:prototype
Map: ../map.md

## Question

Come sono disegnati ministri, tecnici e leader? L'11/11/2026, guardando il prototipo delle [Fondamenta visive](08-fondamenta-visive.md), Mario ha detto che la resa grafica dei ministri non gli piace. Non ha ancora detto perché: è la prima cosa da chiedergli, davanti al prototipo aperto.

Quello che ha visto è un segnaposto: le quattro forme elementari del prototipo vincolante (`.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html:590-608`), copiate pari pari in `prototipi/08-fondamenta.prototype.html`. Il kit di circa 30 parti raccomandato in [Caricature](05-caricature.md) non è mai stato disegnato.

Da decidere, su 3-5 stili a confronto: il tratto e quanto è caricatura; da cosa si riconosce una persona dall'altra; come si leggono il dicastero (oggi l'accessorio) e lo stato del ministro (oggi la bocca); come si distinguono ministro, tecnico e leader di partito; i colori, dentro i sette inchiostri delle fondamenta o fuori.

Vincoli già fissati, da non riaprire: il riquadro è di 34 px nella riga e 84 px nella carta del ministro, e lo stile deve reggere a 34 px; nel tono grave la caricatura lascia il posto alle iniziali; nessun volto somiglia a un politico reale (storia 159: per questo [Caricature](05-caricature.md) esclude capelli, occhiali e barbe, e se lo stile nuovo li vuole la prova di riconoscimento va rifatta); SVG nostro, combinato con un hash di ruolo, partito e nome, senza dipendenze; nella rosa di tre le facce non devono coincidere.

Metodo: subagente `graphic-designer` in due passaggi, prima le varianti con l'anteprima e poi la rifinitura di quella scelta. Le varianti si mostrano a Mario dentro la riga del borsino e la carta del ministro di `prototipi/08-fondamenta.prototype.html`, coi dieci ministri delle Viste 03 e 08, non su un foglio a parte. Si parte da `docs/ricerca/estetica/caricature-di-personaggi-inventati.md` e `docs/ricerca/estetica/grafica-italiana-giocosa.md` (metodo per disegnare ministri inventati).

## Answer
