---
tipo: sintesi
fonti: [fonti/sfcr-readme.md, fonti/rudebusch-svensson-1999-policy-rules.md, fonti/fed-policy-rules.md, fonti/head-mayer-2013-gravity-cookbook.md, fonti/piermartini-yotov-2016-structural-gravity.md, fonti/saez-slemrod-giertz-2009-eti.md, fonti/laubach-2003-deficits-interest.md, fonti/ball-leigh-loungani-2013-okun.md, fonti/beforeit-arxiv-2502-13267.md, fonti/eurostat-figaro-metadata.md]
aggiornato: 2026-10-17
---
# Motore economico raccomandato

Confronto fra i modelli di `modelli/` per decidere cosa combinare in un motore che gira a ogni turno in pochi millisecondi, offline, e che un economista trovi credibile. La risposta breve: un nucleo aggregato a blocchi, con contabilità stock-flow consistent, dinamica IS-Phillips backward-looking, commercio gravitazionale, input-output aggregato e finanza pubblica con elasticità misurate. Niente ABM completo.

## Confronto

| Blocco | Modello | Costo per turno | Stato persistente | Credibilità | Implementazione aperta |
|---|---|---|---|---|---|
| Contabilità | SFC (SIM/PC) | trascurabile [M] | stock per settore (moneta, titoli, debito) | alta: identità contabili | `sfcr` MIT, solo offline |
| Congiuntura | Rudebusch-Svensson + Taylor | 0,004 ms per paese in Python [M] | 10 numeri per paese | media-alta: stimato, rapporto di sacrificio ~3 | nessuna serve |
| Lavoro | Okun per paese | trascurabile | $u^*$ | alta: coefficienti stabili | nessuna serve |
| Distribuzione | 10 decili, Gini derivato | trascurabile | redditi per decile | media | nessuna serve |
| Settori | Leontief aggregato | 0,003 ms a 15 settori; 0,33 ms a 315 (15 settori × 21 paesi) [M] | matrice $A$ | alta | nessuna serve |
| Commercio | gravità strutturale | 0,65 ms per 190 paesi, 10 iterazioni [M] | costi bilaterali, accordi | alta: $\varepsilon \approx -5$ | codice degli autori non pubblico |
| Finanza pubblica | debito, ETI, spread, rating | trascurabile | debito, tasso medio, rating | media-alta | nessuna serve |
| ABM completo | BeforeIT | [NON TROVATO in assoluto]; 8 milioni di agenti a scala 1:1 | milioni di agenti | alta ma stocastico | Apache 2.0 dal 2026-07-24 |

Misure: `python3 -I /tmp/sp-bench/bench.py` e un one-liner per le matrici 15/315/960, Python 3.14.7, numpy 2.5.1, Apple A18 Pro, 2026-10-16, script usa e getta fuori dal repo. La somma dei blocchi raccomandati sta sotto i 2 ms per turno in Python con numpy; in un linguaggio compilato sarà più bassa [INF].

## Raccomandazione (non decisione)

1. **Scheletro SFC a 5 settori** (famiglie, imprese, banche, Stato, estero) per paese giocato; i partner hanno solo PIL, prezzi e commercio. Ogni scelta del giocatore passa da una riga della matrice dei saldi, e il controllo «la somma fa zero» trova i bug gratis ([[macro-stock-flow-consistent]]).
2. **Rudebusch-Svensson trimestrale** per output gap e inflazione, con regola di Taylor inerziale per la banca centrale indipendente e una variabile di credibilità per le aspettative ([[macro-neokeynesiano-piccolo]], [[banca-centrale-e-inflazione]]). Niente aspettative razionali: niente solver, niente indeterminatezza a runtime.
3. **Okun per paese e 10 decili** per disoccupazione e disuguaglianza ([[mercato-del-lavoro-e-disuguaglianza]]).
4. **Leontief a 10-15 macrosettori** per paese giocato e partner principali, calibrato su FIGARO, per gli shock settoriali e l'inflazione importata ([[input-output-leontief]]).
5. **Gravità strutturale** con $\sigma \approx 6$ e coefficienti mediani di Head e Mayer per dazi, accordi e sanzioni; dati CEPII ([[commercio-modello-gravitazionale]]).
6. **Finanza pubblica** con dinamica del debito, curva di Laffer da ETI 0,12-0,4, spread di base da Laubach e rating da Cantor e Packer; il default è uno stato raggiungibile. Le pensioni a ripartizione leggono occupati e pensionati dalla piramide di [[demografia-e-migrazioni]] e il loro saldo entra nel saldo primario ([[finanza-pubblica]]).
7. **ABM**: no come motore del turno. Al massimo uno scenario offline ([[modelli-agent-based-economici]]).

**Decisione per la v1** (17/10/2026, [[decisioni]]): presi i punti 1 (a 4 settori, senza banche), 2 (senza Taylor nazionale: tasso della BCE), 4 (Leontief nazionale a 10 settori), 6 (con spread non lineare per l'euro) e 7; il punto 3 perde i decili a favore delle fasce di reddito dell'elettorato; il punto 5 resta fuori, l'estero è domanda esogena.

Due moltiplicatori trasversali, da altri filoni, si applicano prima che una scelta del giocatore entri nei blocchi [INF]: la capacità di attuazione del ministero scala l'effetto e allunga i tempi ([[burocrazia-corruzione-e-capacita-statale]]), e le riforme strutturali (pensioni, lavoro, fisco) entrano a regime in anni secondo [[ritardi-e-inerzia-delle-politiche]]. Il motore economico ha già i suoi ritardi interni (2-6 trimestri nella IS-Phillips); quelli di attuazione si sommano a monte.

Ordine di aggiornamento in un turno [INF]: politiche del giocatore → finanza pubblica (gettito, spesa) → domanda finale → Leontief (produzione per settore) → gravità (commercio) → IS/Phillips (gap, inflazione) → Okun (disoccupazione) → decili (redditi, Gini) → banca centrale (tasso) → SFC (aggiornamento degli stock e controllo di chiusura) → spread e rating.

## Cosa costa

- **Calcolo**: non è il vincolo, sotto i 2 ms misurati.
- **Calibrazione**: è il costo vero. Ogni paese giocabile chiede valori iniziali per SFC, RS, Okun, $A$ e fiscalità. I dati aperti bastano (vedi sotto), ma serve uno script di estrazione e aggregazione [INF].
- **Leggibilità**: ogni blocco ha cause e coefficienti nominabili, quindi la UI può spiegare perché il PIL è sceso. Un ABM non lo permette.

## Dati per calibrare, e cosa entra nel repo

| Dataset | Licenza | Nel repo pubblico |
|---|---|---|
| [[world-bank-wdi]] | CC BY 4.0 + clausola controversie, eccezioni di terzi | sì, filtrando gli indicatori di terzi |
| [[oecd]] | riuso anche commerciale con attribuzione, salvo dati di terzi | sì |
| [[eurostat]] | riuso con citazione | sì |
| [[istat]] | CC BY 4.0 | sì |
| [[penn-world-table]] | CC BY 4.0 | sì |
| [[cepii-baci-e-gravity]] | Etalab 2.0 | sì |
| [[input-output-tables]] (FIGARO, ICIO, WIOD) | come Eurostat / OCSE / CC BY 4.0 | sì, meglio in forma aggregata |
| [[imf-weo]] | termini IMF: riuso commerciale solo con permesso, no download massivo | no, raccomandato |
| [[un-comtrade]] | ridistribuzione solo per abbonati Premium Pro | no, usare BACI |

## Rischi e punti aperti

- Coefficienti stimati su USA (RS, Laubach): vanno ricalibrati per paesi europei, e lo spread non lineare dell'eurozona non è coperto dalle fonti catturate [NON TROVATO]. Per l'Italia chiusi il 17/10/2026: IS-Phillips dell'area euro, spread quadratico e criteri del TPI in [[economia-italiana-parametri]].
- Frequenza del turno: RS è trimestrale; un turno mensile cambia i coefficienti.
- I parametri derivati da dati non ridistribuibili (Comtrade, WEO) sono una zona grigia da far confermare al filone legale.
- Democracy 4 mostra i difetti da evitare: valori normalizzati in [0,1] che saturano, debito senza conseguenze, ciclo a sinusoide esogena ([[democracy-4]]). Il motore proposto li evita per costruzione: grandezze in unità reali, debito come stock con interessi, ciclo generato dagli shock.

## Fonti

- **URL** https://cran.r-project.org/web/packages/sfcr/readme/README.html · [V]
  **perché conta qui** riferimento numerico per lo scheletro SFC
  **cosa se ne prende** equazioni SIM e traiettoria per i test (fonti/sfcr-readme.md)
- **URL** https://www.nber.org/chapters/c7417.pdf · [V]
  **perché conta qui** il blocco congiunturale raccomandato
  **cosa se ne prende** IS e Phillips con coefficienti (fonti/rudebusch-svensson-1999-policy-rules.md)
- **URL** https://www.cepii.fr/PDF_PUB/wp/2013/wp2013-27.pdf · [V]
  **perché conta qui** parametri del commercio
  **cosa se ne prende** $\varepsilon = -5{,}03$ e mediane dei coefficienti (fonti/head-mayer-2013-gravity-cookbook.md)
- **URL** https://www.nber.org/system/files/working_papers/w15012/w15012.pdf · [V]
  **perché conta qui** curva di Laffer con numeri della letteratura
  **cosa se ne prende** $\tau^* = 1/(1+ae)$, $e$ fra 0,12 e 0,4 (fonti/saez-slemrod-giertz-2009-eti.md)
- **URL** https://arxiv.org/html/2502.13267v1 · [V]
  **perché conta qui** l'alternativa ABM e il suo costo
  **cosa se ne prende** calibrazione ancora prototipale, stocasticità, benchmark relativi (fonti/beforeit-arxiv-2502-13267.md)

## Collegamenti

- Tutte le pagine di `modelli/` e `dati/` di questo filone, linkate sopra.
- [[demografia-e-migrazioni]]: la piramide che alimenta pensioni e forza lavoro.
- [[burocrazia-corruzione-e-capacita-statale]]: la capacità di attuazione come moltiplicatore degli effetti.
- [[ritardi-e-inerzia-delle-politiche]]: i tempi di risposta delle riforme.
- [[democracy-4]]: il riferimento da superare.
- [[victoria-3]]: l'altro gioco con un'economia a settori (pagina di un altro filone, se esiste).
