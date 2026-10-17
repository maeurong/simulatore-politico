---
tipo: gioco
fonti: [fonti/democracy4-modding.md, fonti/democracy4-mod-policies.md, fonti/democracy4-mod-simulation.md, fonti/democracy4-mod-situations.md, fonti/democracy3-modding.md, fonti/cliffsblog-d4-fixed-income.md, fonti/cliffsblog-2021-04.md, fonti/cliffsblog-2020-05.md, fonti/cliffsblog-d4-three-party-happiness.md, fonti/interview-cliff-2014.md, fonti/democracy4-faq.md, fonti/democracy4-dlc.md, fonti/steam-democracy4.md, fonti/steam-democracy4-voting-systems.md, fonti/democracy3africa-site.md, fonti/steam-democracy3-africa.md, fonti/wiki-democracy-series.md, fonti/steam-api-snapshot.md, fonti/steam-recensioni-meccaniche.md, fonti/steam-recensioni-realismo.md, fonti/steam-discussion-d3-ministers.md, fonti/steam-discussion-d4-coalition.md]
aggiornato: 2026-10-16
---
# Democracy (serie: 1, 2, 3, 3 Africa, 4)

Simulatore di governo a turni di Positech (Cliff Harris): il giocatore e' capo del governo, regola policy con slider e cerca la rielezione. E' il riferimento diretto del nostro progetto: la sua rete policy → variabili → gruppi di voto e' documentata pubblicamente nei file di modding, e i suoi limiti (elezioni a soglia, opposizione finta, PIL saturato) sono misurati dal designer stesso. Si prende il metodo, si evitano i limiti.

## Scheda

| Titolo | Uscita | Piattaforme Steam (flag API) | Recensioni Steam |
|---|---|---|---|
| Democracy 4 | 13 gen 2022 (alpha agosto 2020) | windows | 6429, 82% Very Positive |
| Democracy 3 | 14 ott 2013 | windows, mac, linux | 7256, 74% Mostly Positive |
| Democracy 3 Africa | 12 apr 2016 | windows, mac | 409, 73% Mostly Positive |

Dati [M] da Steam Store API, 16/10/2026 (fonti/steam-api-snapshot.md). Democracy 1 e' del 2005, Democracy 2 del dicembre 2007 e usa nazioni fittizie (fonti/wiki-democracy-series.md). Democracy 3 Africa riusa base e gameplay di D3 con arte, eventi e dilemmi nuovi, tarati su temi come aiuti esteri condizionati, malnutrizione e acqua potabile (fonti/democracy3africa-site.md).

Motore in-house in C++ (Visual Studio), due programmatori; arte, audio e musica da terzi (fonti/democracy4-faq.md, fonti/wiki-democracy-series.md).

## Loop di gioco

Il giocatore spende **capitale politico** per introdurre, cancellare, alzare o abbassare una policy; ogni policy ha costi di capitale separati per le quattro azioni (fonti/democracy4-mod-policies.md). Le policy impiegano piu' turni per entrare a regime, e il numero di turni dipende dalla competenza del ministro responsabile (fonti/democracy4-mod-policies.md). Prima di ogni elezione il giocatore fa due promesse all'elettorato, verificate alle elezioni successive (fonti/wiki-democracy-series.md). La sconfitta elettorale (o l'assassinio) chiude la partita; il designer dichiara che la sfida vera e' costruire un paese di cui essere fieri (fonti/steam-democracy4.md).

## Il modello: una "rete neurale" artigianale

- Tutto e' un neurone: elettore, gruppo, policy, ministro, evento. Ogni neurone vale fra 0 e 1 o fra -1 e +1 (fonti/cliffsblog-d4-fixed-income.md).
- I neuroni sono legati da **effetti** caricati da CSV, non scritti nel codice: si modifica il file e si gioca subito la simulazione cambiata (fonti/democracy4-modding.md). Formule e sintassi in [[policy-e-effetti]].
- D3 simula **2.000 elettori**; il campo `population` del paese dice quante persone reali rappresenta ciascuno (fonti/democracy3-modding.md).
- Ogni elettore appartiene a piu' gruppi insieme (es. povero, conservatore, fumatore, patriota), quindi non si possono accontentare tutti (fonti/wiki-democracy-series.md). Dettaglio in [[elettorato-e-gruppi-di-voto]].
- Il ciclo economico globale e' una **sinusoide** esogena: il file paese dice solo da che punto della sinusoide si parte (fonti/democracy3-modding.md).
- Scala di grandezza: il PIL ha 79 input e 80 output, e il gioco contiene "THOUSANDS of equations" (fonti/cliffsblog-2021-04.md).

Novita' dichiarate da D4: report dei media, governi di coalizione, poteri d'emergenza, sistemi a tre partiti, inflazione, corruzione, QE, helicopter money, UBI (fonti/steam-democracy4.md). Il DLC Voting Systems aggiunge le regole elettorali come policy (voto obbligatorio, voter ID, finanziamento ai partiti, robocall), il passaggio maggioritario ↔ proporzionale, un tema unico su cui puntare la campagna, la scelta fra campagna onesta e campagna negativa, e il sondaggio sui soli "likely voters" (fonti/steam-democracy4-voting-systems.md).

## Dove il modello e' finto (dichiarato dal designer)

1. **Il voto e' una soglia sulla felicita'.** Con tre partiti vota per il giocatore solo chi ha felicita' ≥ 66%; 0-33% va all'opposizione, 33-66% al terzo partito. L'opposizione ha posizioni opposte al 100% su tutto, e il designer scrive: "I do NOT plan on actually modelling all the political stances of opposition parties" (fonti/cliffsblog-d4-three-party-happiness.md).
2. **Valori in [0,1] = saturazione.** I giocatori arrivano al PIL al 100% e non salgono oltre. La causa e' che ci sono pochi effetti che lo riducono e incrementi troppo lineari (es. `GDP,0+(x*0.44)` per la produttivita'). La cura proposta e' curvare a mano le formule (fonti/cliffsblog-2021-04.md). Un recensore nota il tetto del PIL a 1.000 miliardi (fonti/steam-recensioni-realismo.md).
3. **Effetti solo percentuali.** Fino al 2020 ogni beneficio era proporzionale al reddito: l'abbonamento al bus valeva 500 $ per la pensionata povera e 15.000 $ per l'ex gestore di hedge fund. Il rimedio e' stato un sistema "ombra" di neuroni a reddito fisso, ricucito a mano con quello vecchio (fonti/cliffsblog-d4-fixed-income.md).
4. **Debito senza default.** "Democracy 4 does not have an option to default on debt" (fonti/cliffsblog-2020-05.md). La recensione negativa piu' votata (73 utili, 146 ore) dice che il debito "has almost no real consequences": debito/PIL al 3.500-4.000% senza crisi (fonti/steam-recensioni-meccaniche.md).
5. **Ogni variabile ne trascina altre 3-4.** Esempio del designer: i prezzi delle case richiedono mutui, offerta di case, equita' negativa e mobilita' del lavoro, "and then the simulation can easily get too big to be fun" (fonti/interview-cliff-2014.md).

## Cosa lodano e cosa criticano i giocatori

- Lode: causa-effetto leggibile, liberta' ideologica, nessuna linea di partito imposta (fonti/steam-recensioni-meccaniche.md). La neutralita' e' anche dichiarata nella FAQ: "a game ABOUT politics, not a 'political' game" (fonti/democracy4-faq.md).
- Critica: c'e' una "correct" way fra socialdemocrazia e neoliberismo, e la pianificazione statale non e' praticabile (fonti/steam-recensioni-meccaniche.md). Gli eventi sono "stale" e di matrice statunitense anche giocando il Regno Unito; il PIL tende sempre a scendere (fonti/steam-recensioni-meccaniche.md).
- Critica: l'assassinio arriva anche in un paese prospero e sorvegliato, e sembra "rolling a dice" (D3) (fonti/steam-recensioni-meccaniche.md). La FAQ di D4 lo difende: e' la conseguenza di riforme troppo radicali (fonti/democracy4-faq.md).
- Critica: vincendo troppo il gioco si svuota. Elezioni con zero voti all'opposizione, nessuna nuova legge da fare, nessun modo di ritirarsi (D3) (fonti/steam-recensioni-meccaniche.md).
- Ministri (D3): si dimettono anche con il 70% dei voti, e il gioco non dice perche'. Per la community la lealta' dipende dal fare policy gradite ai loro gruppi (fonti/steam-discussion-d3-ministers.md).
- Richiesta ricorrente: trattative di coalizione e parlamento. Un altro giocatore si chiede se un parlamento che dice no renderebbe il gioco frustrante (fonti/steam-discussion-d4-coalition.md).

## Per il nostro gioco

- **Prendere**: il grafo di effetti dichiarativo, in file di testo moddabili, con inerzia per effetto; la popolazione campionata (migliaia di agenti con appartenenze multiple); il bilancio fra realismo e "too big to be fun".
- **Evitare**: valori tutti normalizzati in [0,1] per le grandezze economiche (saturano); voto come soglia su un solo scalare; opposizione senza posizioni; debito senza conseguenze; ciclo economico esogeno a sinusoide.
- **Aperto**: quanto rendere visibile il grafo. D4 mostra il breakdown per elettore, ed e' cosi' che il designer ha scoperto il bug dei redditi (fonti/cliffsblog-d4-fixed-income.md).
- Non copiare i CSV di Positech: vedi [[ip-e-giochi-di-riferimento]].

## Collegamenti
- [[policy-e-effetti]]: sintassi delle formule e struttura dei CSV.
- [[elettorato-e-gruppi-di-voto]]: 2.000 elettori, gruppi sovrapposti, voto a soglia.
- [[ministri-e-gabinetto]]: competenza e lealta' dei ministri.
- [[campagna-elettorale]]: DLC Voting Systems.
- [[lawgivers-2]]: l'alternativa centrata sul parlamento, citata dai giocatori come direzione per un "Democracy 5".
- [[victoria-3]]: modello a pop e gruppi d'interesse (pagina di altro agente).
- [[ip-e-giochi-di-riferimento]]: cosa non si puo' riprendere.
