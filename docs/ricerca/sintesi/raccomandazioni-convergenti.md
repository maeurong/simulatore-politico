---
tipo: sintesi
fonti: []
aggiornato: 2026-10-16
---
# Raccomandazioni convergenti

Le raccomandazioni su cui più filoni della ricerca arrivano da soli alla stessa conclusione, ciascuna con le righe che la sostengono. Sotto, le domande di prodotto che restano aperte e che può chiudere solo Mario. Questa pagina non decide nulla: raccoglie e rimanda. Le decisioni già prese stanno in [[decisioni]].

Ogni rimando ha la forma `docs/ricerca/<cartella>/<pagina>.md:<riga>`; le affermazioni e le loro fonti stanno nella riga citata.

## Raccomandazioni convergenti

### 1. Popolazione aggregata, non un agente per cittadino

Gestionali, grand strategy, economia e scienza politica scartano l'agente per ogni cittadino: costa CPU, si calibra male e non si spiega al giocatore.

- docs/ricerca/sistemi/cittadini-simulati.md:35 e :36: aggregato come motore, individui come finestra; SimCity 2013 e Cities: Skylines II hanno dovuto ridurre agenti o mappa.
- docs/ricerca/sistemi/mercati-e-pops.md:55: pop come gruppi, decine o centinaia.
- docs/ricerca/giochi/stellaris.md:52: Paradox ha raggruppato i pop nella 4.0 per prestazioni.
- docs/ricerca/sistemi/logistica-e-trasporti.md:30 e docs/ricerca/giochi/cities-skylines.md:50: niente pathfinding per agente a scala paese.
- docs/ricerca/giochi/simcity-2013.md:42 e docs/ricerca/giochi/workers-and-resources-soviet-republic.md:46: causalità leggibile da aggregati spiegabili.
- docs/ricerca/modelli/modelli-agent-based-economici.md:43 e docs/ricerca/sintesi/motore-economico-raccomandato.md:33: niente ABM come motore del turno.
- docs/ricerca/modelli/mercato-del-lavoro-e-disuguaglianza.md:37: 10 decili invece di agenti.
- docs/ricerca/modelli/opinion-dynamics.md:51: rete sociale per gruppi.

Resta aperto un punto più fine: gli elettori campione (migliaia, alla Democracy) hanno uno stato proprio o derivano dal gruppo? Vedi la tensione in docs/ricerca/sistemi/cittadini-simulati.md:39 e docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:46, lasciata aperta anche in docs/ricerca/tecnica/architettura-simulazione.md:40. Deciso il 17/10/2026: derivano dal gruppo, senza stato proprio (docs/ricerca/sintesi/decisioni.md).

### 2. Grandezze in unità reali e conti che chiudono; interi per denaro, popolazione e voti

Il filone tecnico chiede interi o virgola fissa per riproducibilità; quello economico e quello dei giochi chiedono unità reali invece di valori in [0,1] e una contabilità che non crea né distrugge denaro. Le due richieste vanno nella stessa direzione: numeri esatti e verificabili.

- docs/ricerca/tecnica/architettura-simulazione.md:21 e :38: interi o virgola fissa per denaro, popolazione e voti; niente float nei conti pubblici.
- docs/ricerca/tecnica/core-rust-e-ui-separata.md:26 e docs/ricerca/sintesi/confronto-stack.md:32: PRNG e interi sotto controllo come argomento per il core separato.
- docs/ricerca/sistemi/policy-e-effetti.md:63: unità vere (moneta, persone, tassi) invece di tutto in [0,1].
- docs/ricerca/sintesi/motore-economico-raccomandato.md:64 e docs/ricerca/giochi/democracy-4.md:57: evitare la saturazione dei valori normalizzati.
- docs/ricerca/modelli/macro-stock-flow-consistent.md:35 e :53: la somma dei saldi fa zero a ogni turno; il debito è uno stock con interessi.
- docs/ricerca/modelli/finanza-pubblica.md:69: il default è uno stato raggiungibile.

Punto tecnico da verificare, non di prodotto: l'inversa di Leontief e le equazioni IS-Phillips sono scritte in virgola mobile nelle misure fatte finora; come si concilino con la regola degli interi va provato nello spike del core [INF].

### 3. Ritardi strutturali, diversi per sottosistema

Quasi ogni filone arriva a dire che una politica rende in tempi suoi, e che il ritardo deve nascere dalla struttura (stock, coorti, vintage) più che da un timer uguale per tutto.

- docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:41 e :42: stock per coorte o vintage; nessuna inerzia uguale per tutto.
- docs/ricerca/modelli/clima-ed-energia.md:43: stock di capitale con vita utile.
- docs/ricerca/modelli/istruzione-e-capitale-umano.md:19 e :36: capitale umano per coorte, niente effetti sul PIL nello stesso mandato.
- docs/ricerca/modelli/demografia-e-migrazioni.md:38: la fertilità risponde lentamente.
- docs/ricerca/modelli/finanza-pubblica.md:71: pensioni come pressione lenta e prevedibile.
- docs/ricerca/modelli/prontezza-e-spesa-per-la-difesa.md:40: tre orologi della difesa (mesi, 1-2 anni, 5-12 anni).
- docs/ricerca/modelli/macro-neokeynesiano-piccolo.md:53: 2-6 trimestri fra decisione ed effetto.
- docs/ricerca/modelli/burocrazia-corruzione-e-capacita-statale.md:31 e docs/ricerca/sintesi/motore-economico-raccomandato.md:35: capacità di attuazione che allunga i tempi.
- docs/ricerca/modelli/controinsurrezione-e-stabilita.md:34: circa sei anni dopo la svolta.
- docs/ricerca/sistemi/leggi-decreti-ed-editti.md:44 e docs/ricerca/giochi/fate-of-the-world.md:19: maturazione degli effetti, politiche pluriennali.

### 4. Informazione imperfetta: il giocatore vede rapporti, non lo stato vero

Scienza politica, militare, sanità e giochi convergono sulla catena stato vero → statistica rumorosa e in ritardo → rapporto di parte, con la possibilità di comprare informazione.

- docs/ricerca/sistemi/consiglieri-e-informazione-imperfetta.md:24 e :29: tre livelli, rumore strutturato che si impara a leggere.
- docs/ricerca/modelli/sondaggi-ed-errore.md:55: sondaggi con errore comune, d'istituto e di campionamento.
- docs/ricerca/modelli/epidemie-sir-seir.md:17: casi rilevati, non S/E/I/R.
- docs/ricerca/modelli/deterrenza-ed-escalation.md:32: lo scarto d'informazione come causa della guerra.
- docs/ricerca/modelli/costo-politico-della-guerra.md:31: probabilità di vittoria percepita, diversa da quella vera.
- docs/ricerca/modelli/lanchester-e-modelli-di-combattimento-aggregati.md:33: esito mostrato come distribuzione.
- docs/ricerca/modelli/burocrazia-corruzione-e-capacita-statale.md:22: percezioni in ritardo sulle riforme.
- docs/ricerca/giochi/workers-and-resources-soviet-republic.md:46 e docs/ricerca/giochi/this-is-the-police.md:19: l'informazione si paga.

### 5. Ogni numero spiega perché è cambiato

È la critica più ripetuta ai giochi del genere e la richiesta più ripetuta nelle pagine di modello.

- docs/ricerca/sistemi/policy-e-effetti.md:64: breakdown dei contributi per ogni variabile.
- docs/ricerca/giochi/power-and-revolution.md:38 e docs/ricerca/giochi/rulers-of-nations.md:24: leggibilità prima della quantità di dati.
- docs/ricerca/sintesi/motore-economico-raccomandato.md:43: blocchi con cause nominabili; un ABM non lo permette.
- docs/ricerca/sistemi/crisi-ed-eventi.md:38 e docs/ricerca/sistemi/personaggi-fazioni-e-stabilita-interna.md:55: ogni evento cita la causa, ogni soglia si vede prima.
- docs/ricerca/sistemi/ministri-e-gabinetto.md:29 e docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:44: perché un ministro si dimette, quali gruppi una legge ha spostato.
- docs/ricerca/giochi/suzerain.md:35 e docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:53: niente variabili decisive nascoste; tooltip annidati.

### 6. Modello e istituzioni come dati, non come codice

- docs/ricerca/sistemi/policy-e-effetti.md:62 e docs/ricerca/tecnica/modding-e-dati-esterni.md:31: effetti dichiarativi in file di testo, modding gratis.
- docs/ricerca/modelli/sistemi-elettorali.md:58: ogni sistema elettorale descritto come dati.
- docs/ricerca/sistemi/partiti-e-coalizioni.md:25 e docs/ricerca/giochi/power-and-revolution.md:37: regole istituzionali come parametri del paese.
- docs/ricerca/giochi/simcity-2013.md:41: regole dichiarative alla GlassBox.

### 7. Esiti probabilistici e soglie con isteresi, non gradini secchi

- docs/ricerca/modelli/voto-spaziale.md:41: voto logit, probabilità lisce invece delle soglie di Democracy.
- docs/ricerca/sistemi/parlamento-e-legislazione.md:36 e docs/ricerca/giochi/the-political-process.md:38: voto in aula probabilistico.
- docs/ricerca/sistemi/crisi-ed-eventi.md:35: crisi lente a soglia con isteresi.
- docs/ricerca/modelli/polarizzazione-e-fiducia-istituzionale.md:52 e docs/ricerca/modelli/finanza-pubblica.md:63: fiducia e rating crollano in fretta e risalgono piano.
- docs/ricerca/modelli/epidemie-sir-seir.md:37: compliance come stock che si consuma.

### 8. Pressione endogena: il governo perde consenso anche senza sbagliare

- docs/ricerca/modelli/voto-economico-e-retrospettivo.md:46: costo del governare, circa 2 punti a legislatura, concentrati nella prima metà.
- docs/ricerca/giochi/shadow-president.md:19: evitare l'equilibrio in cui non fare nulla è ottimale.
- docs/ricerca/modelli/finanza-pubblica.md:71: l'invecchiamento diventa debito se nessuno interviene.
- docs/ricerca/sistemi/campagna-elettorale.md:28: la campagna sposta voti al margine, conta il governo.

## Domande di prodotto aperte per Mario

Elencate, non decise. Ognuna rimanda alle righe dove la ricerca l'ha lasciata aperta.

1. **Arco della partita.** Un mandato (5 anni) o più legislature (20-30 anni)? La sconfitta elettorale chiude la partita? docs/ricerca/sistemi/tempo-e-ritmo.md:60; docs/ricerca/giochi/tropico-6.md:52 (evitare «sconfitta = game over») contro docs/ricerca/giochi/democracy-4.md:24; i benefici lenti chiedono più mandati o un bilancio di eredità: docs/ricerca/modelli/ritardi-e-inerzia-delle-politiche.md:38, docs/ricerca/modelli/istruzione-e-capitale-umano.md:37, docs/ricerca/modelli/controinsurrezione-e-stabilita.md:34. Deciso il 17/10/2026: più legislature, la sconfitta porta all'opposizione (docs/ricerca/sintesi/decisioni.md).
2. **Paese reale o fittizio, partiti reali o famiglie ideologiche.** La pagina legale raccomanda paesi e partiti fittizi (docs/ricerca/legale/soggetti-reali-vs-fittizi.md:43); altre pagine presuppongono l'Italia o partiti reali: docs/ricerca/dati/ches.md:28 (tensione registrata), docs/ricerca/modelli/sistemi-elettorali.md:59 (preset italiani), docs/ricerca/dati/input-output-tables.md:28 (paese giocabile europeo), docs/ricerca/modelli/alleanze-e-impegni.md:36 (NATO per nome), docs/ricerca/estetica/mappe-e-globo.md:20 (confini). Collegata: il paese è nell'euro? docs/ricerca/modelli/banca-centrale-e-inflazione.md:41.
3. **Direzione artistica sobria o spettacolare.** La sintesi estetica raccomanda la carta stampata, sobria, con la rete di Democracy (docs/ricerca/sintesi/direzioni-artistiche-candidate.md:71) e un solo effetto di firma (docs/ricerca/estetica/shader-ed-effetti-economici.md:61, docs/ricerca/estetica/movimento-e-feedback.md:42); il vincolo citato altrove è «grafica spettacolare ma leggera» (docs/ricerca/giochi/republic-the-revolution.md:22, docs/ricerca/tecnica/altre-opzioni.md:49); e The Political Process mostra un pubblico a cui la grafica importa poco (docs/ricerca/giochi/the-political-process.md:39). La scelta è già segnata come di Mario in docs/ricerca/sintesi/direzioni-artistiche-candidate.md:79.
4. **Tempo reale con pausa o turni, e passo del tick.** docs/ricerca/giochi/power-and-revolution.md:39, docs/ricerca/giochi/supreme-ruler.md:40, docs/ricerca/sistemi/tempo-e-ritmo.md:55 (tick giornaliero) contro docs/ricerca/modelli/macro-stock-flow-consistent.md:47 (trimestre); tensione registrata in docs/ricerca/sistemi/tempo-e-ritmo.md:61 e docs/ricerca/modelli/macro-stock-flow-consistent.md:48. Deciso il 17/10/2026: turni trimestrali discreti (docs/ricerca/sintesi/decisioni.md).
5. **Ruolo del giocatore.** Solo capo del governo, o anche opposizione e leader di un partito minore? docs/ricerca/giochi/power-and-revolution.md:37, docs/ricerca/modelli/formazione-di-coalizioni.md:51; quanto controllo sul proprio partito: docs/ricerca/sistemi/partiti-e-coalizioni.md:27; può sciogliere le camere: docs/ricerca/modelli/cicli-politico-economici-e-luna-di-miele.md:48.
6. **Guerra: piano operativo o solo leve politiche, e il nucleare.** docs/ricerca/sistemi/militare-e-guerra.md:75; docs/ricerca/modelli/deterrenza-ed-escalation.md:36.
7. **Profondità dei ministeri.** Sotto-gioco per dicastero o slider e moltiplicatori: docs/ricerca/sistemi/ministri-e-gabinetto.md:30. Indicatore sintetico della «salute del paese»: docs/ricerca/sistemi/feedback-visivo-dello-stato.md:47.
8. **Stack e linguaggio del core.** Godot con core separato, in Rust via gdext o prima in GDScript: docs/ricerca/sintesi/confronto-stack.md:28, docs/ricerca/tecnica/core-rust-e-ui-separata.md:32.
9. **Licenza del codice.** Open source deciso, licenza da scegliere: docs/ricerca/sintesi/decisioni.md:20. Nome del gioco dopo la verifica dei marchi: docs/ricerca/legale/ip-e-giochi-di-riferimento.md:36. Lingue al lancio: docs/ricerca/estetica/tipografia-e-font-aperti.md:60.

## Tensioni fra pagine ancora aperte

Registrate nelle pagine con «In tensione con»; nessuna fonte le scioglie.

- Elettori campione con stato proprio contro gruppi aggregati: docs/ricerca/sistemi/cittadini-simulati.md:39, docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:46.
- Passo del tick: docs/ricerca/sistemi/tempo-e-ritmo.md:61, docs/ricerca/modelli/macro-stock-flow-consistent.md:48.
- Partiti reali da CHES contro partiti fittizi: docs/ricerca/dati/ches.md:28, docs/ricerca/legale/soggetti-reali-vs-fittizi.md:44.
- IMF WEO fra le fonti escluse o fra quelle non commerciali: docs/ricerca/dati/imf-weo.md:25, docs/ricerca/sintesi/decisioni.md:16.
- Prezzi per bene alla Victoria 3 contro macrosettori di Leontief: docs/ricerca/giochi/victoria-3.md:57 contro docs/ricerca/sistemi/mercati-e-pops.md:58 (la pagina stessa lo lascia aperto).

## Per il nostro gioco

- Prendiamo: le otto convergenze sopra come vincoli di partenza del brainstorming, perché nessuna dipende da un filone solo.
- Evitiamo: trattare come decise le domande di prodotto; ognuna cambia più sistemi insieme.
- Aperto: le nove domande per Mario e le cinque tensioni.

## Collegamenti

- [[decisioni]]: le scelte già prese da Mario.
- [[motore-economico-raccomandato]], [[confronto-stack]], [[direzioni-artistiche-candidate]]: le tre sintesi di filone da cui vengono molte righe.
- [[cittadini-simulati]], [[elettorato-e-gruppi-di-voto]], [[tempo-e-ritmo]], [[ritardi-e-inerzia-delle-politiche]], [[consiglieri-e-informazione-imperfetta]]: le pagine trasversali su cui convergono più filoni.
- [[soggetti-reali-vs-fittizi]]: la domanda sul paese reale o fittizio.
