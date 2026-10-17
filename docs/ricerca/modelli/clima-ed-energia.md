---
tipo: modello
fonti: [fonti/pnas-dice-2023.md, fonti/nordhaus-dice-rice.md, fonti/en-roads-guide-background.md, fonti/en-roads-guide-about.md, fonti/en-roads-guide-structure.md, fonti/en-roads-guide-carbonprice.md, fonti/en-roads-user-guide.md, fonti/owid-energy-data-github.md, fonti/steam-fate-of-the-world.md]
aggiornato: 2026-10-16
---
# Clima ed energia: DICE, En-ROADS, mix energetico

Due modelli di riferimento. DICE è un modello di crescita ottima con danni climatici: pochi stock, risolto per ottimizzazione. En-ROADS è un modello a dinamica dei sistemi con leve politiche e ritardi espliciti. Per un gioco serve la struttura di En-ROADS con le funzioni di danno di DICE.

## DICE (Nordhaus, Barrage)
- È il modello integrato clima-economia più usato; calcola il costo sociale del carbonio (SCC) per il governo USA e altri (fonti/pnas-dice-2023.md).
- Struttura: modello di Ramsey con un settore, produzione Cobb-Douglas in capitale e lavoro, output ridotto da danni Ω e costi di abbattimento Λ (fonti/pnas-dice-2023.md).
- Danni quadratici nell'aumento di temperatura dall'era preindustriale: 3,1% del PIL a +3 °C, 7,0% a +4,5 °C; la funzione non è affidabile sopra +4 °C (fonti/pnas-dice-2023.md).
- Abbattimento: costo come polinomio del tasso di controllo μ; azzerare le emissioni costa l'11% del PIL nel 2020, in calo dell'1,7% annuo (fonti/pnas-dice-2023.md).
- Ciclo del carbonio DFAIR a quattro serbatoi (fonti/pnas-dice-2023.md).
- Risultati: SCC di 66 $/tCO2 nel 2020 in scenario base; +1,5 °C non raggiungibile coi vincoli del modello; la politica attuale vale un prezzo implicito di circa 6 $/tCO2 (fonti/pnas-dice-2023.md).
- Codice: GAMS ed Excel per DICE-2016 dal sito dell'autore (fonti/nordhaus-dice-rice.md). Nessuna licenza dichiarata sulla pagina: [NON TROVATO], quindi si reimplementano le equazioni pubblicate, non si copia il codice.

## En-ROADS (Climate Interactive, MIT Sloan)
- Simulatore di scenari che sintetizza la letteratura su tempi di ritardo, tassi di apprendimento, elasticità di prezzo e crescita storica delle fonti (fonti/en-roads-guide-about.md).
- Pensato per workshop e giochi di ruolo politici (fonti/en-roads-user-guide.md).
- Ritardo principale, rinnovo dello stock di capitale: solo il 6% circa delle infrastrutture energetiche mondiali cambia ogni anno; centrali e raffinerie durano 30 anni e più; il fossile si ritira a circa il 3% l'anno (fonti/en-roads-guide-background.md).
- Il capitale che consuma energia (auto, edifici, industria) dura 10-15 anni: si possono vendere auto elettriche da subito, ma la flotta cambia in decenni (fonti/en-roads-guide-background.md).
- Inerzia climatica: con la CO2 stabilizzata la temperatura sale ancora per anni, perché l'oceano raggiunge tardi l'equilibrio termico (fonti/en-roads-guide-background.md).
- Leve: prezzo del carbonio, efficienza energetica, elettrificazione, sussidi o tasse per fonte, rimozione del carbonio (fonti/en-roads-guide-background.md, fonti/en-roads-guide-carbonprice.md).
- Motori esogeni: popolazione e PIL pro capite spingono le emissioni, comprese quelle di agricoltura e uso del suolo (fonti/en-roads-guide-structure.md).

## Mix energetico
- [INF] Uno stock di capacità per fonte (carbone, gas, petrolio, nucleare, idro, rinnovabili) con vita utile e costo livellato; le nuove quote si decidono solo sulla capacità che si ritira più la domanda nuova. È lo schema di En-ROADS ridotto all'osso (fonti/en-roads-guide-background.md).
- Dati: dataset energetico di OWID, che aggrega Energy Institute Statistical Review, EIA ed Ember (fonti/owid-energy-data-github.md); la licenza va controllata fonte per fonte (vedi [[our-world-in-data]]).

## Variabili
- **Mosse**: prezzo del carbonio, standard di efficienza, sussidi per fonte, divieti (es. nuovo carbone), investimento pubblico in ricerca.
- **Subite**: temperatura (globale, quindi decisa anche dagli altri paesi), prezzi internazionali dell'energia, danni da eventi estremi.

## Ritardi
- Capacità energetica: decenni. Flotte e edifici: 10-15 anni. Temperatura: decenni dopo le emissioni (fonti/en-roads-guide-background.md).

## Giochi
- [[fate-of-the-world]]: turni di 5 anni, carte politiche che bloccano fondi per più turni, scienza del clima di Myles Allen (Oxford) (fonti/steam-fate-of-the-world.md).

## Per il nostro gioco
- Prendere: stock di capitale con vita utile (il ritardo nasce dalla struttura e non serve un timer); danno quadratico alla DICE come costo annuo sul PIL.
- Evitare: l'ottimizzazione intertemporale di DICE nel ciclo di gioco. Si esegue in avanti, non si risolve.
- Aperto: il clima è globale. Il nostro paese pesa poco sulla temperatura, quindi la leva climatica in gioco è soprattutto adattamento, energia e diplomazia.

## Collegamenti
- [[ritardi-e-inerzia-delle-politiche]]: il rinnovo dello stock di capitale come ritardo strutturale.
- [[crisi-ed-eventi]]: eventi estremi la cui frequenza cresce con la temperatura.
