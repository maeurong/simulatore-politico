---
tipo: modello
fonti: [fonti/jasss-hegselmann-krause-2002-testo.md, fonti/jasss-hegselmann-krause-2002.md, fonti/castellano-2009-fisica-dinamiche-sociali.md, fonti/arxiv-fernandez-gracia-2014-voter-model.md, fonti/jasss-flache-2017-influenza-sociale.md]
aggiornato: 2026-10-16
---
# Opinion dynamics: DeGroot, bounded confidence, voter model

Modelli di come le opinioni si spostano per influenza reciproca fra agenti. Tre famiglie: medie pesate (DeGroot, Friedkin-Johnsen), confidenza limitata (Deffuant, Hegselmann-Krause) e copia di un vicino (voter model). Per il gioco servono a muovere le posizioni degli elettori fra un'elezione e l'altra, e a far nascere consenso, polarizzazione o frammentazione senza scriverli a mano.

## Equazioni

- **DeGroot (modello classico)**: `x(t+1) = A x(t)`, con A matrice stocastica (righe non negative che sommano a 1): ogni agente fa la media pesata delle opinioni altrui (fonti/jasss-hegselmann-krause-2002-testo.md) [V]. Converge al consenso se il grafo è connesso; il disaccordo persistente richiede ingredienti in più (fonti/jasss-flache-2017-influenza-sociale.md) [V].
- **Friedkin-Johnsen**: `x(t+1) = G x(0) + (I − G) A x(t)`, con g_i ∈ [0,1] l'attaccamento all'opinione iniziale. Con G = 0 torna DeGroot (fonti/jasss-hegselmann-krause-2002-testo.md) [V]. È il modo più semplice di avere "zoccoli duri".
- **Deffuant-Weisbuch** (incontri a coppie): se `|x_i − x_j| < ε`, allora `x_i ← x_i + μ (x_j − x_i)` e simmetrico per j; altrimenti nulla. μ ∈ [0, 1/2] regola la velocità, ε il numero di gruppi finali (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V].
- **Hegselmann-Krause** (riunioni): ogni agente prende la media di tutti i vicini entro ε: `x_i(t+1) = Σ_{j:|x_i−x_j|<ε} a_ij x_j / Σ_{j:|x_i−x_j|<ε} a_ij` (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V].
- **Voter model**: a ogni passo un agente a caso copia l'opinione (discreta) di un vicino a caso (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V]. Variante con rumore: con probabilità legata a D l'agente cambia per motivi esterni (media, deriva) invece di copiare (fonti/arxiv-fernandez-gracia-2014-voter-model.md) [V].

## Risultati utili

- **Deffuant**: consenso completo per ε > 1/2 su grafi completi, reticoli, grafi casuali e scale-free; per ε più piccolo il numero di cluster finali è circa `1/(2ε)` (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V]. La media globale delle opinioni si conserva (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V].
- **HK**: soglia di consenso ε_c ≈ 0,2 su grafi completi, 1/2 su grafi a grado medio costante; i tempi di convergenza esplodono vicino alle soglie di biforcazione dei cluster (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V].
- **Voter model**: su sistema finito arriva sempre al consenso; tempo medio `T_N ~ N` sul grafo completo, `N²` in 1D, `N ln N` in 2D (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V]. Senza rumore, quindi, un elettorato simulato diventa monocolore: serve il rumore.

## Evidenza

- **A favore (voter model con rumore)**: con influenza sociale rumorosa e pendolarismo casa-lavoro, il modello riproduce la dispersione dei voti per contea nelle presidenziali USA 1980-2012 (deviazione standard ≈ 0,11, stazionaria) e la correlazione spaziale che decade col logaritmo della distanza; partendo dal 2000 predice il 2004 con l'80% dei rapporti modello/dato fra 0,6 e 1,5 a livello di contea (fonti/arxiv-fernandez-gracia-2014-voter-model.md) [V].
- **Contro (bounded confidence e affini)**: la rassegna di Flache e coautori (fra cui Deffuant e Lorenz) conclude che la letteratura agent-based "non può ancora offrire spiegazioni e previsioni affidabili" per dinamiche concrete, e che manca calibrazione e validazione sui dati (fonti/jasss-flache-2017-influenza-sociale.md) [V].
- Lo stesso modello che descrive bene un contesto può fallire in un altro (faccia a faccia contro online) (fonti/jasss-flache-2017-influenza-sociale.md) [V].

## Parametri tipici

| Parametro | Valore | Fonte |
|---|---|---|
| ε (Deffuant, opinioni in [0,1]) | > 0,5 consenso; 0,1 → ~5 cluster | (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V] |
| μ (Deffuant) | 0-0,5; incide sulla velocità, poco sui cluster se non piccolo | (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V] |
| ε_c (HK, grafo completo) | ≈ 0,2 | (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V] |
| rumore D (voter model USA) | ≈ 0,03, con 2,5 passi Monte Carlo l'anno | (fonti/arxiv-fernandez-gracia-2014-voter-model.md) [V] |
| g_i (Friedkin-Johnsen) | da tarare; nessun valore generale trovato | [NON TROVATO] |

## Costo computazionale

- DeGroot / FJ: `O(archi)` per passo con matrice sparsa [INF].
- Deffuant: `O(1)` per incontro; N incontri a passo → `O(N)` [INF].
- HK: ingenuo `O(N²)` per passo; in una dimensione su grafo completo basta ordinare le opinioni e usare finestre scorrevoli, `O(N log N)` [INF]. Castellano nota che il costo di HK è il motivo per cui è stato poco studiato (fonti/castellano-2009-fisica-dinamiche-sociali.md) [V].
- Voter model: `O(1)` per aggiornamento [INF].

Con 10⁴-10⁵ elettori-campione tutti e quattro stanno in pochi millisecondi a tick; il vincolo vero è il tempo di convergenza, non il costo per passo [INF].

## Cosa semplifichiamo

- Rete sociale: grafo per gruppi (classe, territorio, età) invece di reti individuali [INF].
- Media e partiti come agenti con ε ampio che influenzano molti nodi [INF].
- Nessun meccanismo di repulsione (influenza negativa): genera polarizzazione ma la rassegna Flache lo considera non ancora validato [INF da fonti/jasss-flache-2017-influenza-sociale.md].

## Per il nostro gioco

- Base consigliata: Friedkin-Johnsen (zoccolo duro) + Deffuant fra gruppi + rumore. È lineare dove serve, dà cluster e polarizzazione quando ε scende, ed è economico.
- ε come leva di gioco: polarizzazione mediatica, social, crisi riducono ε; istruzione e fiducia lo alzano. Da documentare come scelta di design, non come fatto empirico.
- Il voter model rumoroso è l'unico con una validazione su dati elettorali reali: utile per la geografia del voto (correlazione fra territori vicini).
- Non promettere "previsioni": l'evidenza dice che questi modelli generano pattern plausibili, non traiettorie esatte.

## Collegamenti

- [[voto-spaziale]]: le opinioni x_i sono le posizioni v_i degli elettori.
- [[abm-elettorali]]: dove queste regole girano insieme ai partiti.
- [[polarizzazione-e-fiducia-istituzionale]]: misura il risultato delle dinamiche.
- [[elettorato-e-gruppi-di-voto]]: i gruppi dei giochi esistenti come nodi del grafo.
- [[cittadini-simulati]]: agenti singoli o aggregati: chi porta le opinioni nel motore.

## Riferimenti

- **URL** https://jasss.soc.surrey.ac.uk/5/3/2/2.pdf · [V]
  **perché conta qui** definisce in una notazione unica DeGroot, Friedkin-Johnsen e HK
  **cosa se ne prende** le equazioni da implementare
- **URL** https://arxiv.org/pdf/0710.3256 · [V]
  **perché conta qui** rassegna con soglie di consenso, numero di cluster e tempi di convergenza
  **cosa se ne prende** ε_c, la regola 1/(2ε), le leggi di scala di T_N
- **URL** https://ar5iv.labs.arxiv.org/html/1309.1131 · [V]
  **perché conta qui** unica validazione su dati elettorali reali fra i modelli di questa pagina
  **cosa se ne prende** voter model con rumore, D ≈ 0,03, criterio di taratura sulla dispersione territoriale
- **URL** https://www.jasss.org/20/4/2.html · [V]
  **perché conta qui** rassegna critica scritta dagli autori stessi dei modelli
  **cosa se ne prende** il limite: pattern plausibili sì, previsioni no
