Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Quali indicatori, con quali pesi, formano gli interessi di ciascuna delle 6 condizioni (Elettorato, 06)? Candidati: reddito disponibile reale della fascia (deflazionato col suo paniere), disoccupazione regionale, pensioni e regime pensionistico, sussidi, indicatori di servizio dei dicasteri (sanità, scuola, trasporti, sicurezza, durata dei processi), salario pubblico e turnover per i dipendenti pubblici, pressione fiscale per fascia. Quanto è decisione e quanto contenuto da tarare?

Ingressi: Dicasteri (03) dà gli indicatori di servizio; Modello economico (07) gli indicatori per gruppo, regione e fascia; la colpa di pensioni e sussidi va a Lavoro e Welfare (07).

Ingresso da Catalogo atti e riforme (16): gli atti non toccano mai direttamente i gruppi, solo gli indicatori. Da decidere qui se qualche disposizione entra direttamente negli interessi di una condizione (licenziamento e salario minimo per i dipendenti privati, forfettario per gli autonomi) o arriva solo attraverso gli indicatori.

## Answer

Grilling con Mario del 17/10/2026, due giri e conferma finale. Raccomandazioni accettate tranne due cambi di Mario: la sicurezza entra anche negli interessi di dipendenti privati e autonomi, e la disoccupazione la paga Lavoro e Welfare invece del Presidente del Consiglio. Termini in `CONTEXT.md`: precisato interessi, nuovo giudizio.

Scelte fatte senza chiedere, perché di strumento: ogni indicatore si misura in deviazioni standard della sua variazione trimestrale storica, così grandezze in unità diverse si sommano; l'asimmetria si applica indicatore per indicatore, come in Nannestad-Paldam (`docs/ricerca/modelli/voto-economico-e-retrospettivo.md:13`); ogni indicatore si legge alla risoluzione che ha (disoccupazione per regione, servizi nazionali finché la nebbia «Mappa delle 20 regioni» non decide altro).

**Il denaro passa da una voce sola.** Il reddito disponibile reale della fascia, deflazionato col suo paniere (07), vale per tutte le condizioni e contiene ogni euro: salario, pensione, sussidio, imposte, prezzi. Pensioni, sussidi, pressione fiscale e salario pubblico non sono indicatori a sé, altrimenti lo stesso euro conterebbe due volte. Negli interessi entrano a parte solo cose che non sono denaro: disoccupazione regionale (come rischio, oltre al reddito perso), indicatori di servizio (03), occupazione pubblica.

**Giudizio cumulato con memoria.** A ogni turno ogni gruppo aggiorna il suo giudizio su ciascun partito di governo: `g ← λ·g + Σ_k w_k · f(Δ_k)`, con f asimmetrica (peggioramenti pesati più dei miglioramenti, ρ ≈ 1/3 come partenza) ed emivita della memoria intorno a 4 turni. Contano solo le variazioni, non i livelli. Un partito che entra al governo parte da zero; uno che resta si porta dietro il suo giudizio. Il costo del governare nasce da qui, senza regola scritta (`docs/ricerca/modelli/voto-economico-e-retrospettivo.md:46`).

**Tabella degli interessi.** Sei righe, una per condizione; la fascia conta già attraverso reddito, paniere e imposte. Pesi provvisori, ogni riga somma a 1:

| Condizione | Reddito reale | Indicatori che non sono denaro |
|---|---|---|
| Dipendente privato | 0,45 | disoccupazione regionale 0,25 · trasporti 0,10 · sanità 0,10 · sicurezza 0,10 |
| Dipendente pubblico | 0,50 | occupazione pubblica (turnover) 0,20 · sanità 0,15 · scuola 0,15 |
| Autonomo | 0,55 | durata dei processi 0,15 · sostegno alle imprese 0,15 · sicurezza 0,15 |
| Pensionato | 0,50 | sanità (liste d'attesa) 0,35 · sicurezza 0,15 |
| Disoccupato o inattivo | 0,35 | disoccupazione regionale 0,40 · copertura del welfare 0,25 |
| Studente | 0,30 | scuola 0,35 · disoccupazione regionale 0,35 (prospettive) |

Fuori da ogni riga: efficienza della riscossione, sicurezza energetica, prontezza della difesa. Arrivano ai gruppi solo attraverso reddito, paniere o posizione della misura.

**Nessuna disposizione diretta negli interessi.** Gli effetti materiali passano dagli indicatori, la parte simbolica dalla posizione della misura, che sposta la posizione percepita del partito (06). Ingressi per la spec del modello economico:
- l'evasione va ripartita per condizione, così la lotta all'evasione abbassa il reddito degli autonomi;
- il salario minimo entra nel reddito primario della fascia bassa dei dipendenti privati (e nell'occupazione, dalla sua tabella valore → effetto, 16);
- forfettario e licenziamento arrivano da imposte degli autonomi e da u*.

**Chi paga.** Resta la quota fissa al partito del Presidente del Consiglio (06); il resto, voce per voce:
- reddito primario (salari privati, redditi autonomi) e prezzi del paniere: solo il Presidente del Consiglio;
- disoccupazione, pensioni, sussidi, reddito minimo: Lavoro e Welfare;
- imposte, salario pubblico, occupazione pubblica: Economia e Finanze;
- ogni indicatore di servizio: il suo dicastero.

Conseguenza: Lavoro e Welfare, che già possiede 8 disposizioni (16), diventa il dicastero che porta più colpa, quindi il più pesante nella spartizione (05). Interno entra in tre righe.

**Decisione e contenuto.** Deciso qui: struttura, righe della tabella, responsabili. Provvisori, al bilanciamento: pesi numerici, memoria λ, asimmetria ρ, quota fissa del Presidente del Consiglio.

Effetti sulla mappa: nebbia «Costo del governare e bilanciamento» con pesi, λ, ρ e quota del Presidente del Consiglio; nessun ticket nuovo.
