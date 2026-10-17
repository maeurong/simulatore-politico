---
tipo: sistema
fonti: [fonti/democracy3-modding.md, fonti/cliffsblog-d4-three-party-happiness.md, fonti/cliffsblog-d4-fixed-income.md, fonti/wiki-democracy-series.md, fonti/steam-democracy4.md, fonti/steam-democracy4-voting-systems.md, fonti/vic3-wiki-interest-groups.md, fonti/vic3-dd57-journey.md, fonti/steam-political-process.md, fonti/steam-recensioni-meccaniche.md, fonti/steam-recensioni-political-process.md, fonti/steam-statecraft-corrupted.md, fonti/wiki-republic-the-revolution.md]
aggiornato: 2026-10-16
---
# Elettorato e gruppi di voto

Come i giochi rappresentano chi vota e perche'. Sono tre famiglie: elettori campionati con appartenenze multiple (Democracy), popolazioni aggregate divise fra gruppi d'interesse (Victoria 3), collegi con dati demografici e ideologia (The Political Process). Il punto debole ricorrente e' il passaggio dall'opinione al voto: soglie su un solo numero, oppure leggi che non spostano voti.

## Democracy: elettori campionati, voto a soglia

- **2.000 elettori simulati**; il file paese dice quante persone reali rappresenta ciascuno (fonti/democracy3-modding.md). D4 dichiara "thousands of virtual citizens" (fonti/steam-democracy4.md).
- **Appartenenze multiple**: ogni elettore e' in piu' gruppi (es. povero, conservatore, fumatore, patriota), quindi non si possono accontentare tutti (fonti/wiki-democracy-series.md).
- **Reddito per elettore** come neurone; il reddito di un elettore e' la somma pesata degli input dai neuroni di reddito dei suoi gruppi (fonti/cliffsblog-d4-fixed-income.md).
- **Dalla felicita' al voto**: e' una soglia. Con due partiti, implicitamente, vota per chi governa chi sta sopra il 50%. Con tre: 0-33% opposizione, 33-66% terzo partito, 66-100% giocatore. L'opposizione e' opposta al 100% su tutto, il terzo partito sta in mezzo, e il designer non intende modellare le posizioni dei partiti d'opposizione (fonti/cliffsblog-d4-three-party-happiness.md).
- Gli script di paese cambiano la composizione dei gruppi all'avvio; `apathy` e' un parametro del paese (fonti/democracy3-modding.md).
- Il DLC Voting Systems distingue elettorato e "likely voters" nei sondaggi (fonti/steam-democracy4-voting-systems.md).

## Victoria 3: pop, attrazione, forza politica

- Ogni pop distribuisce la propria forza politica fra i gruppi d'interesse in proporzione a un punteggio di attrazione; una parte puo' restare non allineata (fonti/vic3-wiki-interest-groups.md).
- La forza politica individuale dipende da ricchezza, accettazione culturale, e status o voto concessi dalle leggi. Un aristocratico ricco sotto oligarchia pesa "centinaia o migliaia di volte" un bracciante; i senza diritti pesano zero e passano dai movimenti politici (fonti/vic3-wiki-interest-groups.md).
- Clout a somma zero (totale 100%), con soglie di marginalizzazione (<4%) e di potere (>20%) e isteresi (fonti/vic3-wiki-interest-groups.md).
- Il compasso politico 3D iniziale e' stato abbandonato perche' "inadequate", e sostituito da "many dozens of ideologies" con posizioni su leggi specifiche (fonti/vic3-dd57-journey.md).
- Dettagli in [[victoria-3]] (pagina di altro agente).

## The Political Process: collegi da dati reali

- 50 stati e oltre 3.000 contee da dati reali; oltre 80 variabili modificabili (demografia, ideologia, affluenza, poverta') (fonti/steam-political-process.md).
- Le dichiarazioni in campagna muovono l'entusiasmo, che muove l'affluenza; una piattaforma estrema perde gli indipendenti, una troppo moderata perde le primarie (fonti/steam-political-process.md).
- Critica: posizioni binarie rosso/blu, nessuna sfumatura (fonti/steam-recensioni-political-process.md).

## Fallimenti segnalati dai giocatori

- [[lawgivers-2]]: le leggi approvate non spostano i voti; il consenso cala se non si fanno interviste e pubblicita' (fonti/steam-recensioni-meccaniche.md).
- [[democracy-4]] (D3): con tutto al massimo, elezioni con zero voti all'opposizione e zero astenuti (fonti/steam-recensioni-meccaniche.md).
- [[republic-the-revolution]]: ideologia come somma di tre misure (Force, Influence, Wealth), giudicata "rock, paper scissors" (fonti/wiki-republic-the-revolution.md).
- [[statecraft-simulator]] (Corrupted Democracy) fa della manipolazione demografica il gioco stesso: politicizzare gli apolitici, mettere i gruppi uno contro l'altro (fonti/steam-statecraft-corrupted.md).

## Per il nostro gioco

- **Prendere**: popolazione campionata (migliaia di agenti) con appartenenze multiple e reddito proprio, come Democracy; peso politico diverso per gruppo, come Victoria 3; collegi con dati, come The Political Process.
- **Cambiare**: preferenze su molte issue, non un solo scalare di felicita'. Il voto e' una scelta fra partiti con posizioni proprie (distanza dalle posizioni, giudizio sull'economia, fedelta' di partito) piu' un'affluenza separata. I modelli formali sono materia di altri agenti (scienza politica).
- **Garantire**: ogni legge sposta qualche voto, e il giocatore vede quali gruppi ha guadagnato o perso e perche'.
- **Aperto**: quanti agenti (2.000 come D3 o di piu') rispetto al costo di calcolo per turno; e se modellare la persuasione fra elettori.
- In tensione con [[cittadini-simulati]]: là il motore lavora per gruppi e l'individuo mostrato deriva dal gruppo, senza stato proprio; qui si propongono migliaia di elettori campione con reddito e appartenenze propri. Le fonti non decidono (vedi [[architettura-simulazione]]).
- Deciso il 17/10/2026 ([[decisioni]]): niente migliaia di elettori con stato proprio. 360 gruppi (condizione × fascia di reddito × regione) con un istogramma della posizione, voto logit con astensione, interessi materiali per condizione al posto delle molte issue. Con quali dati si tarano: [[dati-per-tarare-i-gruppi]].

## Collegamenti
- [[policy-e-effetti]]: da dove arrivano gli effetti sugli elettori.
- [[partiti-e-coalizioni]]: chi riceve i voti.
- [[campagna-elettorale]]: come si muovono i voti prima del voto.
- [[victoria-3]], [[tropico-6]]: modelli a gruppi e fazioni (pagine di altri agenti).
- [[voto-spaziale]]: la funzione elettore → voto proposta al posto della soglia.
- [[opinion-dynamics]]: come si muovono le posizioni degli elettori fra un voto e l'altro.
- [[voto-economico-e-retrospettivo]]: il giudizio sull'economia e il costo del governare.
- [[sondaggi-ed-errore]]: elettorato e votanti probabili nei sondaggi.
- [[media-informazione-e-propaganda]]: la salienza che pesa le issue.
- [[costo-politico-della-guerra]]: il rally e l'erosione del consenso, per gruppo.
