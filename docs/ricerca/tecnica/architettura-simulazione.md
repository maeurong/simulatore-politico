---
tipo: tecnica
fonti: [fonti/gaffer-fix-your-timestep.md, fonti/gaffer-floating-point-determinism.md, fonti/rand-book-reproducibility.md, fonti/bevy-fixed-timestep-example.md, fonti/democracy4-modding.md, fonti/democracy3-modding.md]
aggiornato: 2026-10-16
---
# Architettura della simulazione

Come separare la simulazione dalla grafica, renderla riproducibile con un seed e salvarla. La raccomandazione che segue vale per qualunque motore: simulazione a tick in un modulo che non conosce il rendering, stato in tabelle, numeri economici interi, PRNG con algoritmo esplicito.

## Simulazione separata dal rendering

- Schema classico: la simulazione avanza a passo fisso, il rendering disegna quando puo' e interpola tra due stati; un accumulatore di tempo decide quanti passi fare per frame (fonti/gaffer-fix-your-timestep.md) [V].
- Bevy offre lo stesso schema con lo schedule `FixedUpdate` e l'interpolazione visiva, al costo di un frame di ritardo visivo (fonti/bevy-fixed-timestep-example.md) [V].
- Per un gioco politico il tick e' di calendario (giorno, settimana, mese) e il giocatore controlla la velocita'; la UI legge solo snapshot dello stato e manda comandi [INF]. Ne segue che la simulazione puo' girare in un thread o processo separato e senza finestra, utile per test, bilanciamento e CI [INF].

## Determinismo e seed

- I float danno risultati identici solo con **stesso compilatore, stessa architettura e accorgimenti** specifici; aspettarsi risultati uguali tra compilatori, architetture o tra build debug e release e' ingenuo (fonti/gaffer-floating-point-determinism.md) [V].
- Riproducibilita' dei numeri casuali in Rust: `StdRng` e `SmallRng` sono dichiarati **non portabili** (possono cambiare tra piattaforme e versioni); gli algoritmi nominati sono riproducibili tra piattaforme e patch release, ma le minor possono cambiare i valori (fonti/rand-book-reproducibility.md) [V].
- Sempre dalla stessa fonte: mai campionare `usize` direttamente; le distribuzioni con `f32`/`f64` possono non essere portabili per via delle funzioni trascendenti (fonti/rand-book-reproducibility.md) [V].
- Quindi: PRNG con algoritmo esplicito (per esempio ChaCha o Xoshiro da crate dedicato) e versione bloccata; denaro, popolazione e voti in interi o virgola fissa; float solo per grandezze che non devono essere riprodotte bit per bit [INF].
- Un seed per partita, salvato nel salvataggio; un flusso PRNG per sottosistema (economia, eventi, elezioni) per evitare che un cambio in uno sposti tutti gli altri [INF].

## ECS o tabelle

- Democracy usa una rete di "neuroni" (policy, variabili, gruppi) legati da effetti scritti in CSV, e D3 simula 2.000 elettori (fonti/democracy4-modding.md, fonti/democracy3-modding.md; vedi [[democracy-4]]) [V].
- Ordini di grandezza del nostro stato: centinaia di policy e variabili, decine di gruppi, regioni e ministeri, migliaia di elettori campione [INF]. Sono dati **relazionali**: tabelle indicizzate per ID (vettori di struct o colonne) con un grafo di dipendenze bastano. Un ECS rende quando ci sono molte entita' eterogenee che cambiano composizione; qui serve poco [INF].
- Ordine di valutazione esplicito e stabile (ordine topologico del grafo degli effetti), mai l'ordine di iterazione di una hash map [INF].

## Salvataggi

- Salvataggio = snapshot completo dello stato piu' seed, versione dello schema e versione dei dati di gioco (mod compresi) [INF].
- Formato testuale (JSON o RON) leggibile e confrontabile nei test; compressione opzionale [INF].
- Il replay da seed piu' comandi del giocatore funziona solo a parita' di binario, per quanto detto sui float; utile per debug e test di regressione, non come formato di salvataggio [INF].

## Per il nostro gioco

- Prendiamo: tick fisso disaccoppiato, PRNG nominato e versionato, interi per l'economia, tabelle invece di ECS, snapshot versionati.
- Evitiamo: logica di simulazione dentro gli script della UI; `StdRng`; float nei conti pubblici.
- Aperto: granularita' del tick (giorno o settimana) e se gli elettori sono campioni o gruppi aggregati: decisioni di design, non di stack. Deciso il 17/10/2026 ([[decisioni]]): gruppi aggregati, 360 con istogramma della posizione.

## Collegamenti

- [[core-rust-e-ui-separata]]: dove vive questo modulo.
- [[democracy-4]]: modello a rete di effetti caricati da CSV.
- [[modding-e-dati-esterni]]: lo stato iniziale e le formule vengono da file.
- [[bevy]]: `FixedUpdate`.

## Fonti

- **URL** https://gafferongames.com/post/fix_your_timestep/ · [V]
  **perche' conta qui** separare il passo della simulazione dal frame
  **cosa se ne prende** accumulatore e interpolazione
- **URL** https://gafferongames.com/post/floating_point_determinism/ · [V]
  **perche' conta qui** limiti della riproducibilita' con i float
  **cosa se ne prende** stesso binario e stessa architettura come condizione minima
- **URL** https://rust-random.github.io/book/crate-reprod.html · [V]
  **perche' conta qui** seed riproducibili in Rust
  **cosa se ne prende** `StdRng`/`SmallRng` non portabili, niente `usize`, float non portabili
- **URL** https://bevy.org/examples/movement/physics-in-fixed-timestep/ · [V]
  **perche' conta qui** implementazione concreta del passo fisso
  **cosa se ne prende** `FixedUpdate` e il ritardo visivo di un frame
