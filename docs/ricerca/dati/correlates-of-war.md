---
tipo: dataset
fonti: [fonti/cow-datasets.md, fonti/cow-nmc.md, fonti/cow-wars.md, fonti/cow-nmc-v7-documentazione-fonti.md, fonti/cow-nmc-v7-documentazione-unesd.md]
aggiornato: 2026-10-16
---
# Correlates of War (COW)

Il progetto accademico più longevo sui conflitti fra stati, dal 1816. Per noi contano tre dataset: le capacità materiali (NMC, con l'indice CINC), le guerre, le alleanze formali. Si scaricano liberamente con richiesta di citazione, ma il sito **non dichiara una licenza**. Per un repo pubblico questo è un punto aperto, non un via libera.

## Dataset utili

- **National Material Capabilities v7.0 (1816-2022):** sei indicatori annui per ogni stato (popolazione totale e urbana, produzione di ferro e acciaio, consumo di energia, personale militare, spesa militare) e l'indice composito **CINC**. Formati csv, dta, txt (fonti/cow-nmc.md).
- **COW Wars v4.0 (1816-2007), Intra-State v5.1:** guerre interstatali, extra-statali, intrastatali, non statali. Lo stato partecipa a una guerra se subisce almeno 100 morti in battaglia o schiera almeno 1.000 soldati; inizio e fine si contano dalle battaglie, non dalle dichiarazioni (fonti/cow-wars.md). Dal luglio 2026 i dati sulle guerre sono ospitati da Paul Poast (University of Chicago) (fonti/cow-wars.md).
- **Formal Alliances v4.1 (1816-2012):** difesa reciproca, non aggressione, intesa. **MID v5.0 (1816-2014):** ogni minaccia, esibizione o uso della forza fra stati. **DCAD (1980-2010):** accordi bilaterali di cooperazione difensiva. Più contiguità, cambi territoriali, OIG, commercio 1870-2014 (fonti/cow-datasets.md).

## Da dove vengono i numeri militari

- Personale militare: ACDA (agenzia USA) per il 1961-1993, poi ACDA dove possibile, **integrata con The Military Balance (IISS)** (fonti/cow-nmc-v7-documentazione-fonti.md).
- Spesa militare URSS 1955-1963: stime SIPRI; dal 1963, ACDA (fonti/cow-nmc-v7-documentazione-fonti.md).
- Il consumo di energia viene dalla banca dati UN Energy Statistics, che ha una licenza: COW dichiara di **non poter ridistribuire** i dati grezzi ONU, e pubblica solo i valori derivati e il metodo (fonti/cow-nmc-v7-documentazione-unesd.md).

## Licenza e ridistribuibilità

- [NON TROVATO] Nessuna licenza esplicita sulle pagine del sito né nella documentazione NMC v7. C'è solo la richiesta di indicare la versione e citare l'articolo di riferimento (Singer, Bremer e Stuckey, 1972) (fonti/cow-nmc.md); per le guerre, Sarkees e Wayman, 2010 (fonti/cow-wars.md).
- Diversi dataset sono "hosted by" studiosi diversi (Gibler per le alleanze, Hensel per la contiguità, Barbieri per il commercio...) (fonti/cow-datasets.md): la titolarità è frammentata [INF].
- In NMC ci sono valori derivati da IISS e SIPRI, le cui licenze sono restrittive (vedi [[iiss-military-balance]], [[sipri]]). COW li pubblica come valori propri e documenta il trattamento dei dati ONU proprio perché ha un vincolo di licenza a monte (fonti/cow-nmc-v7-documentazione-unesd.md). Che i singoli valori numerici siano proteggibili è una questione giuridica: non la risolviamo qui [INF].

## Per il nostro gioco

- **Prendiamo** CINC come "potenza" di partenza dei paesi, perché è già normalizzato su tutto il sistema degli stati. Le sei componenti si mappano su variabili che il gioco ha comunque: popolazione, urbanizzazione, industria, energia, esercito, spesa [INF].
- **Prendiamo** le soglie COW (100 morti o 1.000 soldati per partecipare; guerra contata dalle battaglie) per decidere quando un paese di gioco "è in guerra" [INF].
- **Prendiamo** i dati su MID e alleanze come basi storiche per la frequenza delle crisi e degli impegni ([[deterrenza-ed-escalation]], [[alleanze-e-impegni]]). Servono per tarare, non vanno caricati [INF].
- **Evitiamo** di includere i file COW nel repo finché la licenza non è chiara. Si usano in sviluppo per tarare, e nei file di gioco vanno valori arrotondati [INF].
- **Aperto:** scrivere ai curatori (Poast per le guerre, il team NMC) per un permesso esplicito di ridistribuzione. Basso costo, alto valore.

## Collegamenti
- [[sipri]], [[iiss-military-balance]]: fonti a monte dei dati militari NMC.
- [[ucdp]]: l'alternativa con licenza aperta per i conflitti dal 1989.
- [[alleanze-e-impegni]], [[deterrenza-ed-escalation]]: dove questi dati tarano il modello.
- [[prontezza-e-spesa-per-la-difesa]]: spesa e personale.

## Riferimenti
- **URL** https://correlatesofwar.org/data-sets/national-material-capabilities/ · [V]
  **perché conta qui** l'unica serie lunga (1816-2022) di capacità nazionali con indice composito
  **cosa se ne prende** CINC come potenza di partenza; la richiesta di citazione
- **URL** https://correlatesofwar.org/wp-content/uploads/NMCv7.zip (NMC_Documentation_v7.pdf) · [V]
  **perché conta qui** mostra che una parte dei dati militari viene da IISS, ACDA e SIPRI
  **cosa se ne prende** la cautela sulla ridistribuzione e il precedente UNESD
- **URL** https://correlatesofwar.org/data-sets/cow-war/ · [V]
  **perché conta qui** definizioni operative di guerra e partecipazione
  **cosa se ne prende** le soglie (100 morti / 1.000 soldati) e l'inizio contato dalle battaglie
- **URL** https://correlatesofwar.org/data-sets/ · [V]
  **perché conta qui** catalogo completo, con chi ospita ogni dataset
  **cosa se ne prende** alleanze, MID, DCAD come fonti di taratura
