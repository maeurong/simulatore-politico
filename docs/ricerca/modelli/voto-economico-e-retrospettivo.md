---
tipo: modello
fonti: [fonti/nannestad-paldam-1999-cost-of-ruling.md, fonti/brender-drazen-2004-testo.md, fonti/louwerse-muller-2020-ciclo-elettorale.md]
aggiornato: 2026-10-21
---
# Voto economico e retrospettivo

Gli elettori giudicano il governo sui risultati passati, soprattutto economici: la "funzione VP" (vote and popularity function) lega crescita, disoccupazione e inflazione al voto per chi governa. Il fatto più solido non è l'effetto dell'economia ma il "costo del governare": in media chi governa perde voti comunque. Per il gioco è il ponte fra il modulo economico e il consenso.

## Equazioni

- **Funzione VP lineare**: `ΔV_gov = −κ + Σ_k α_k · ΔE_k + rumore`, dove ΔE_k sono variazioni di indicatori economici e κ è il costo del governare; se la funzione è calibrata, κ coincide con la costante (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V].
- **Asimmetria del malcontento** (grievance asymmetry): un peggioramento pesa più di un miglioramento uguale. Con `α_i = 2αρ/(1+ρ)` per i miglioramenti e `α_d = 2α/(1+ρ)` per i peggioramenti, ρ ∈ (0,1). Esempio degli autori: α = 2/3, ρ = 1/3 → α_i = 0,3, α_d = 1; due indicatori migliorano di un punto e due peggiorano di un punto, il governo perde (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V]. Questa asimmetria basta da sola a produrre il costo del governare osservato e predice una perdita lineare lungo la legislatura (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V].
- **Chiarezza della responsabilità** (Powell-Whitten 1993, Duch-Stevenson 2008): il termine α_k si riduce quando la responsabilità è diluita (coalizioni larghe, governi di minoranza, bicameralismo) [INF].

## Evidenza

- **Costo del governare**: il governo medio in una democrazia consolidata, in una legislatura normale, perde circa 2,25 punti percentuali di voto; deviazione standard di circa 4,5 punti nei sistemi parlamentari e circa il doppio in quelli presidenziali; il valore non dipende dal sistema elettorale, dalla struttura dei partiti o dalla dimensione del paese (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V]. Eccezioni: paesi molto piccoli, governi brevi, democrazie nuove, economie molto volatili (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V].
- **Effetto della crescita**: Fair, su elezioni presidenziali USA 1916-1976, trova che un punto in più di crescita nell'anno elettorale vale circa un punto di voto all'incumbent (riportato in fonti/brender-drazen-2004-testo.md) [V, fonte secondaria].
- **Instabilità**: gli stessi autori ammettono che la vera funzione VP "si è rivelata sfuggente" e che la solidità del costo del governare dipende dalla stabilità della funzione (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V]. Lewis-Beck e Paldam (Electoral Studies 19, 2000) scrivono che molte funzioni VP trovate negli anni sono poi sparite [INF, non catturata].
- **Retrospezione cieca, contestata**: Achen e Bartels sostengono che gli elettori puniscono i governi anche per eventi fuori dal loro controllo (attacchi di squali del 1916); Fowler e Hall (Journal of Politics 2018, doi:10.1086/699244) rianalizzano tutti gli attacchi mortali 1872-2012 e non trovano effetti sistematici [INF, PDF non scaricabile in sessione].
- **Ciclo dentro la legislatura**: i partiti di governo perdono soprattutto nella prima metà e recuperano solo in parte (fonti/louwerse-muller-2020-ciclo-elettorale.md) [V]. Dettagli in [[cicli-politico-economici-e-luna-di-miele]].

## Parametri tipici

| Parametro | Valore | Fonte |
|---|---|---|
| costo del governare κ | 2,25 punti a legislatura | (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V] |
| deviazione standard dell'esito del governo | 4,5 punti (parlamentare), ~9 (presidenziale) | (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V] |
| effetto crescita | ~1 punto di voto per punto di crescita (USA) | (fonti/brender-drazen-2004-testo.md) [V] |
| asimmetria ρ | ~1/3 nell'esempio degli autori | (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V] |
| peso dell'economia sul voto | circa un terzo della variazione (Lewis-Beck e Paldam 2000) | [INF] |

## Costo computazionale

Una somma pesata per partito di governo a ogni tick: `O(K)` con K indicatori. Se applicata per elettore (economia "egotropica" del proprio gruppo) diventa `O(N·K)`, sempre trascurabile [INF].

## Cosa semplifichiamo

- Elettore sociotropico (giudica l'economia nazionale) come regola base; quello egotropico (il proprio portafoglio) solo per gruppi sociali, se il modulo economico produce redditi per gruppo [INF].
- Nessun voto prospettico separato: l'asimmetria del malcontento lo rende in parte inutile per il gioco [INF].
- Attribuzione della responsabilità: un coefficiente unico per tipo di governo (monocolore, coalizione, minoranza) invece di un modello di percezione [INF].

## Per il nostro gioco

- Implementare il costo del governare come deriva negativa per legislatura (circa 2 punti a fine legislatura, ma concentrati nella prima metà e con recupero parziale, non lungo una retta: fonti/louwerse-muller-2020-ciclo-elettorale.md [V]; forma in [[cicli-politico-economici-e-luna-di-miele]]) più il termine economico con asimmetria: è il fatto empirico più robusto della pagina e dà al giocatore una pressione costante senza scriptarla.
- Il rumore attorno all'esito (deviazione standard 4,5 punti) va nel risultato elettorale, non nei sondaggi (quello dei sondaggi è in [[sondaggi-ed-errore]]).
- Gli shock esogeni (catastrofi) possono pesare sul governo ma l'evidenza è contesa: renderli modesti e tarabili.
- Deciso il 21/10/2026 ([[decisioni]]): niente deriva scritta. Il costo del governare emerge dall'asimmetria, dalle aspettative sul reddito reale (crescita degli ultimi ~8 turni) e dall'erosione nominale di budget e scaglioni; κ = 2,25 e σ = 4,5 diventano bersagli di taratura per una strategia media, non parametri.
- Aperto: indicatori da usare. Il modulo economico di un altro filone deve esporre crescita, disoccupazione, inflazione per gruppo di elettori.

## Collegamenti

- [[voto-spaziale]]: il termine economico entra nella valence del governo.
- [[cicli-politico-economici-e-luna-di-miele]]: andamento nel tempo e manipolazione pre-elettorale.
- [[policy-e-effetti]]: come i giochi esistenti collegano policy, economia e soddisfazione.
- [[costo-politico-della-guerra]]: un altro termine che sposta il consenso del governo.
- [[elettorato-e-gruppi-di-voto]]: i gruppi a cui applicare la versione egotropica.

## Riferimenti

- **URL** https://repec.econ.au.dk/repec/afn/wp/99/wp99_9.pdf · [V]
  **perché conta qui** misura il costo del governare su 19 paesi e lo spiega con l'asimmetria del malcontento
  **cosa se ne prende** κ = 2,25, deviazione standard 4,5 e la forma asimmetrica della funzione VP
- **URL** https://www.nber.org/system/files/working_papers/w10539/w10539.pdf · [V]
  **perché conta qui** riporta la stima di Fair sull'effetto della crescita
  **cosa se ne prende** l'ordine di grandezza "un punto di crescita, un punto di voto"
- **URL** https://www.tomlouwerse.nl/publication/2018-muller-louwerse-psrm · [V]
  **perché conta qui** forma temporale della perdita del governo fra un'elezione e l'altra
  **cosa se ne prende** perdita concentrata nella prima metà della legislatura
- **URL** https://www.andrewbenjaminhall.com/FowlerHall_Sharks.pdf · [INF] (download fallito in sessione)
  **perché conta qui** smonta la prova più citata di retrospezione cieca
  **cosa se ne prende** tenere bassi gli effetti elettorali degli eventi casuali
