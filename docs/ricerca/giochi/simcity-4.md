---
tipo: gioco
fonti: [fonti/simcity-wiki-sc4.md, fonti/simcity-wiki-ordinanze.md, fonti/simcity-wiki-gradimento-sindaco.md, fonti/simcity-wiki-consiglieri.md, fonti/steam-api-snapshot-gestionali.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# SimCity 4

City builder Maxis del 2003 (espansione Rush Hour), ancora giocato e moddato. Introduce la regione: più città adiacenti che il giocatore costruisce tutte e che si scambiano servizi e domanda. Per noi conta la regione come scala di governo, le ordinanze come forma più semplice di legge, il gradimento del sindaco e i consiglieri come canale di feedback. In più dimostra che una grafica pre-renderizzata invecchia bene su hardware minimo.

## Scheda

| Campo | Valore |
|---|---|
| Uscita | 2003; edizione Deluxe su Steam 20/07/2010, Windows/macOS (fonti/steam-api-snapshot-gestionali.md; fonti/simcity-wiki-sc4.md) |
| Recensioni Steam | 9.901, 82% positive (fonti/steam-api-snapshot-gestionali.md) |
| Requisiti minimi | Pentium III 500 MHz, 256 MB RAM, GPU 32 MB DirectX 7, modalità grafica software disponibile (fonti/steam-api-snapshot-gestionali.md) |

## Scala e loop

Modalità Dio (terreno), Sindaco (zone, strade, edifici, bilancio) e My Sim (un cittadino importato da The Sims che commenta la città) (fonti/simcity-wiki-sc4.md). Novità principale: la regione, dove si costruiscono e si collegano più città e si negoziano accordi con i vicini (fonti/simcity-wiki-sc4.md).

## Come modella la cosa

**Servizi locali e finanziati uno per uno.** Rispetto ai capitoli precedenti gli edifici civici non agiscono più su tutta la città ma su un'area vicina, e ogni edificio ha finanziamento e manutenzione regolabili (fonti/simcity-wiki-sc4.md).

**Ordinanze.** Interruttori con costo mensile, tranne quelle finanziarie, che rendono. Ogni ordinanza ha un effetto e un prezzo indiretto: il gioco d'azzardo legale porta entrate ma più crimine e meno desiderabilità; il risparmio energetico frena la crescita commerciale e industriale; la zona denuclearizzata dà una piccola spinta residenziale e smantella le centrali nucleari esistenti; la legge sull'aria pulita riduce l'inquinamento e la crescita industriale (fonti/simcity-wiki-ordinanze.md).

**Gradimento del sindaco.** Scala fino a −100. In SimCity 4 non si viene rimossi per impopolarità, solo se la cassa va troppo in rosso. Fattori a breve termine (tasse alzate, disastri ignorati, incendi, rivolte) e a lungo termine (edifici civici sottofinanziati, istruzione e sanità basse) (fonti/simcity-wiki-gradimento-sindaco.md).

**Consiglieri.** Un pannello di consiglieri per settore, con lo sfondo colorato secondo l'umore (verde = soddisfatto). In una crisi urgente compare un avviso con il link al luogo (fonti/simcity-wiki-consiglieri.md).

**Simulazione dei cittadini.** Nelle fonti catturate non c'è una descrizione ufficiale del modello interno (aggregato o per agente) [NON TROVATO]. Le community del gioco parlano di "cap" di domanda per tile che si alzano con parchi e collegamenti ai vicini (https://steamcommunity.com/app/24780/discussions/0/496880503057331341, non catturata) [INF: modello aggregato per lotto, coerente con le mod che riscrivono il simulatore di traffico].

**Grafica.** Gli edifici sono modelli 3D pre-renderizzati in sprite 2D per un angolo fisso; il traffico invece è in 3D (fonti/simcity-wiki-sc4.md).

## Realismo: cosa dicono i giocatori

Nel campione 2026 c'è una sola recensione negativa con parole chiave: meccaniche "rotte", opzioni ecologiche senza svantaggi, tasse che svuotano la città (fonti/steam-recensioni-gestionali.md). Il campione è piccolo; non va letto come tendenza.

## Prestazioni

Il caso migliore della lista: gira su hardware del 2000, anche in modalità software (fonti/steam-api-snapshot-gestionali.md).

## Per il nostro gioco

- **Prendiamo:** la regione di territori adiacenti che si scambiano servizi e domanda come scala intermedia fra città e paese; l'ordinanza come interruttore con costo e un solo effetto collaterale dichiarato; i consiglieri per settore con umore leggibile a colpo d'occhio; la grafica pre-renderizzata come scelta di leggerezza.
- **Evitiamo:** l'impopolarità senza conseguenze (in SimCity 4 decide solo la cassa).
- **Aperto:** come si comporta un modello di domanda aggregato con soglie per territorio (da verificare su fonte primaria).

## Collegamenti

- [[leggi-decreti-ed-editti]]: le ordinanze come caso minimo.
- [[territorio-e-urbanistica]]: la regione.
- [[consiglieri-e-informazione-imperfetta]]: pannello dei consiglieri.
- [[feedback-visivo-dello-stato]]: colore dell'umore del consigliere.
- [[simcity-2013]]: il seguito ad agenti.
