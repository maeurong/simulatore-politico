---
tipo: modello
fonti: [fonti/cgd-capability-traps-wp234.md, fonti/worldbank-wgi.md, fonti/ti-cpi-how-calculated.md, fonti/ti-cpi-2025.md, fonti/democracy4-mod-policies.md, fonti/wikipedia-papers-please.md, fonti/wikipedia-this-is-the-police.md]
aggiornato: 2026-10-16
---
# Burocrazia, corruzione e capacità statale

La capacità statale è il moltiplicatore fra ciò che il giocatore decide e ciò che accade. Una legge firmata non è una legge applicata. Questa pagina propone una variabile, la capacità di attuazione, che filtra ogni politica, e la corruzione come perdita su quel filtro.

## Il modello concettuale (Pritchett, Woolcock, Andrews 2010)
- Molti paesi sono in «trappole di capacità»: la capacità di attuazione è bassa e migliora lentamente, se migliora. Al ritmo attuale Haiti, Afghanistan o Liberia impiegherebbero secoli per raggiungere Singapore (fonti/cgd-capability-traps-wp234.md).
- Due tecniche di fallimento. **Mimetismo isomorfico**: adottare la forma delle istituzioni dei paesi funzionanti, che maschera la mancanza di funzione. **Carico prematuro**: chiedere al sistema più di quanto regge, con il risultato che la capacità si indebolisce o crolla (fonti/cgd-capability-traps-wp234.md).
- La confusione fra forma e funzione è uno degli errori più diffusi dello sviluppo (fonti/cgd-capability-traps-wp234.md).

## Come entra nel gioco
- Democracy 4 lo fa già in piccolo: i turni di attuazione di una policy e il suo costo sono corretti per la competenza del ministro (fonti/democracy4-mod-policies.md).
- [INF] Proposta: `capacità` (0-1) per ministero. Effetto reale di una politica = effetto nominale × capacità. Tempo di attuazione = base / capacità. Il carico prematuro diventa una regola: se il giocatore lancia più riforme di quante il ministero ne regga, la capacità scende. La capacità cresce lentamente con stipendi pubblici, formazione e digitalizzazione, e cala con le epurazioni politiche e le nomine clientelari.
- [INF] Corruzione: frazione della spesa che si perde (per ministero) più una tassa sugli investimenti privati. Cresce con i controlli deboli e con la libertà di stampa bassa; si riduce lentamente.

## Misurare (e i limiti delle misure)
- Worldwide Governance Indicators (Banca Mondiale): sei dimensioni (voce e responsabilità, stabilità politica, efficacia del governo, qualità della regolazione, stato di diritto, controllo della corruzione), da 35 fonti di percezione, oltre 200 economie, 1996-2025 (fonti/worldbank-wgi.md).
- Avvertenza della Banca Mondiale: sono stime, e le percezioni pubbliche possono restare indietro rispetto alle riforme vere (fonti/worldbank-wgi.md). È un ritardo da modellare.
- Indice di percezione della corruzione (Transparency International): 182 paesi, scala 0-100, almeno 3 di 13 fonti per paese; la media globale 2025 è scesa a 42 (fonti/ti-cpi-2025.md, fonti/ti-cpi-how-calculated.md). Le variazioni piccole non sono significative; la corruzione di un anno può comparire nelle fonti un anno o più dopo (fonti/ti-cpi-how-calculated.md).
- **Licenza CPI: CC BY-ND 4.0**, nessuna opera derivata (fonti/ti-cpi-how-calculated.md). [INF] Si possono usare i numeri per calibrare in privato; ridistribuire nel gioco valori rielaborati è a rischio. Da girare al filone legale.

## Variabili
- **Mosse**: stipendi e assunzioni pubbliche, digitalizzazione, autorità anticorruzione, protezione dei whistleblower, nomine (merito o fedeltà), numero di riforme simultanee.
- **Subite**: capacità ereditata, corruzione sistemica, sabotaggio delle burocrazie ostili.

## Ritardi
- Capacità: decenni per costruirla, mesi per distruggerla (fonti/cgd-capability-traps-wp234.md; l'asimmetria è [INF] dal concetto di carico prematuro).
- Percezione della corruzione: indietro di uno o più anni rispetto ai fatti (fonti/ti-cpi-how-calculated.md, fonti/worldbank-wgi.md).

## Giochi
- [[papers-please]]: la burocrazia vista dal basso, regole che cambiano ogni giorno, mazzette (fonti/wikipedia-papers-please.md).
- [[this-is-the-police]]: corruzione e mafia dentro l'apparato (fonti/wikipedia-this-is-the-police.md).

## Per il nostro gioco
- Prendere: capacità per ministero come moltiplicatore universale; carico prematuro come limite al numero di riforme.
- Evitare: corruzione come semplice slider.

## Collegamenti
- [[ministri-e-gabinetto]]: competenza e lealtà dei ministri (pagina di altro filone).
- [[consiglieri-e-informazione-imperfetta]]: una burocrazia debole produce anche dati peggiori.
- [[ritardi-e-inerzia-delle-politiche]].
