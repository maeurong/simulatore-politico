---
tipo: sistema
fonti: [fonti/cliffsblog-d4-three-party-happiness.md, fonti/steam-democracy4.md, fonti/steam-discussion-d4-coalition.md, fonti/steam-lawgivers2.md, fonti/itch-lawgivers2.md, fonti/steam-gps-2026.md, fonti/vic3-wiki-interest-groups.md, fonti/vic3-dd57-journey.md, fonti/steam-political-process.md, fonti/steam-recensioni-meccaniche.md, fonti/steam-statecraft-corrupted.md]
aggiornato: 2026-10-16
---
# Partiti e coalizioni

Come i giochi modellano i partiti avversari e la formazione dei governi. E' l'area piu' debole del genere. Democracy non modella le posizioni dell'opposizione; Lawgivers II ed Eversim dichiarano coalizioni e sfiducia, ma la loro logica non e' documentata; Victoria 3 tratta i partiti come alleanze di gruppi d'interesse. La trattativa di coalizione e' una richiesta esplicita dei giocatori.

## Cosa fanno i giochi

- **Democracy 4**: dichiara "coalition governments" e "three-party systems" (fonti/steam-democracy4.md). L'opposizione e' l'opposto al 100% delle posizioni del giocatore e il terzo partito sta in mezzo, quindi e' sempre moderato e raccoglie voti ovunque. Con piu' del 33% si e' primo partito "in a coalition". Il designer non modellera' le posizioni dei partiti avversari ("HUGE undertaking"), e propone una correzione a mano delle soglie (0-40 / 40-66 / 66-100) come "small party disadvantage" (fonti/cliffsblog-d4-three-party-happiness.md).
- **Lawgivers II**: governo da soli o in coalizione, sopravvivenza alle mozioni di sfiducia (fonti/steam-lawgivers2.md). Se nel partito convivono troppe posizioni, alcuni politici escono o fondano un partito nuovo (fonti/itch-lawgivers2.md).
- **Geo-Political Simulator 2026**: l'IA dei partiti forma e scioglie coalizioni (anche quella del giocatore), sparisce, cambia leader, fonda partiti; un parametro regola quanto sono attivi i partiti IA. Sono modellate sfiducia costruttiva, fiducia sulla legge, scioglimento e soglia di sbarramento (fonti/steam-gps-2026.md).
- **Victoria 3**: i partiti sono alleanze di gruppi d'interesse, solo nei paesi con elezioni. Un gruppo con almeno il 3% di clout puo' entrare in un partito e, dentro un partito, diventa marginalizzato solo sotto il 2% (fonti/vic3-wiki-interest-groups.md). Lo strato partiti/fazioni e' stato tolto durante lo sviluppo perche' confondeva, poi reintrodotto solo per le democrazie (fonti/vic3-dd57-journey.md). Vedi [[victoria-3]].
- **The Political Process**: solo Democratici e Repubblicani. Ci sono leader di partito e di caucus che influenzano il voto dei colleghi, e un presidente del comitato elettorale che recluta candidati, ne regola le posizioni e sfida i membri sgraditi alle primarie (fonti/steam-political-process.md).
- **Statecraft: Corrupted Democracy**: alleanze con gruppi radicali e organizzazioni ombra (fonti/steam-statecraft-corrupted.md). Critica: "all political parties play identically" (fonti/steam-recensioni-meccaniche.md).

## Cosa chiedono i giocatori

- Per Democracy: "actual coalition talks, parliaments". Un altro giocatore obietta che un parlamento che dice no puo' bloccare la visione del giocatore e frustrarlo. La risposta in thread: e' un livello tattico in piu', non un ostacolo (fonti/steam-discussion-d4-coalition.md).

## Per il nostro gioco

- **Prendere**: partiti IA con posizioni proprie su piu' issue, che competono per gli stessi elettori; regole istituzionali (sbarramento, sfiducia costruttiva) come parametri del paese; scissioni quando un partito e' troppo eterogeneo.
- **Costruire** (nessuno lo fa bene): la trattativa di coalizione. Programma comune su issue, ripartizione dei ministeri, rischio di rottura legato alla distanza fra i partner. I modelli formali di formazione delle coalizioni sono materia di altri agenti.
- **Aperto**: quanto lasciare all'IA la vita interna del proprio partito (Lawgivers II la fa gestire al giocatore, e i giocatori se ne lamentano; fonti/steam-recensioni-meccaniche.md).

## Collegamenti
- [[elettorato-e-gruppi-di-voto]]: da dove vengono i voti dei partiti.
- [[parlamento-e-legislazione]]: maggioranze e voti di fiducia.
- [[ministri-e-gabinetto]]: ripartizione dei ministeri.
- [[democracy-4]], [[lawgivers-2]], [[power-and-revolution]], [[victoria-3]].
- [[formazione-di-coalizioni]]: il modello formale della trattativa: logit sulle coalizioni, legge di Gamson per i ministeri.
- [[abm-elettorali]]: regole Sticker/Aggregator/Hunter/Governator per i partiti IA.
- [[voto-spaziale]]: le posizioni dei partiti IA nello spazio di policy.
