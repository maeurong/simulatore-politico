---
tipo: estetica
fonti: [fonti/balatro-pcgamer-jiggle.md, fonti/balatro-localthunk-timeline.md, fonti/balatro-gameinformer-intervista.md, fonti/balatro-rogueliker-intervista.md, fonti/balatro-steam-motion-sickness.md, fonti/balatro-steam-game-speed.md, fonti/papers-please-devlog-tig-2012-12.md, fonti/papers-please-devlog-mobile.md, fonti/inscryption-gamedeveloper-igf.md, fonti/cultist-thumbsticks-intervista.md, fonti/loophero-gamedeveloper-postmortem.md, fonti/slaythespire-gamedeveloper-igf.md, fonti/lbal-optimal-numbers.md, fonti/lbal-performance-improvement.md, fonti/carbon-motion.md, fonti/easings-net-yml.md, fonti/mdn-prefers-reduced-motion.md, fonti/mdn-font-variant-numeric.md, fonti/mdn-at-property.md, fonti/godot-tween-class.md, fonti/godot-displayserver-4-7.md, fonti/wikipedia-split-flap.md, fonti/wikipedia-it-panini.md, fonti/wcag22-contrast-minimum.md, fonti/steamworks-deck-compat.md]
aggiornato: 2026-10-17
---
# Succo per interfacce fatte di carte, menu e numeri

Come certi giochi rendono divertente un'interfaccia che in sostanza è un foglio di calcolo o un mazzo di carte, e quanto di quel succo regge in un turno di governo da 5-10 minuti. La pagina costruisce sopra [[movimento-e-feedback]], che copre già la definizione di juice, il suo limite, la rivelazione a ritmo e il criterio WCAG 2.3.3: qui non si ripete, si scende al livello delle ricette (durate, curve, ampiezze) e si chiude con tre stili candidati per il prototipo della direzione B con la rete di D ([[direzioni-artistiche-candidate]]).

Tag: [V] fonte primaria letta · [M] misurato in sessione, con il comando · [INF] inferenza · [NON TROVATO].

## Cosa dicono gli sviluppatori

### Balatro (LocalThunk, 2024)

- Il primo lavoro fatto alla ripresa del progetto, dopo tre mesi di pausa, fu «l'effetto succoso di oscillazione in rotazione e in scala quando passi il mouse sulle carte o quando segnano punti» [V] (fonti/balatro-pcgamer-jiggle.md:23, citazione di un post Bluesky di LocalThunk riportata da PC Gamer; il post originale non si è lasciato leggere dall'API pubblica, 403).
- Il sistema «CHIP X MULT» c'era fin dall'inizio, e la direzione del gioco venne dai video di Luck be a Landlord [V] (fonti/balatro-localthunk-timeline.md:32). L'effetto di fiamme sul punteggio per una mano molto buona fu un suggerimento di un amico, che LocalThunk prima detestò e poi accettò perché «si adattava perfettamente» [V] (fonti/balatro-localthunk-timeline.md:88).
- Il filtro CRT entrò per gusto, durante l'iterazione: «mi piace scrivere shader» [V] (fonti/balatro-gameinformer-intervista.md:141). I tooltip erano quasi assenti finché i giocatori della beta non mostrarono di averne bisogno [V] (fonti/balatro-gameinformer-intervista.md:109).
- Il tema del poker serve a far entrare il giocatore con immagini già note, «interessanti e tattili» [V] (fonti/balatro-rogueliker-intervista.md:31). Sul bilanciamento LocalThunk preferisce ciò che «sembra» dritto a ciò che lo è tecnicamente [V] (fonti/balatro-rogueliker-intervista.md:42).
- **Durate, curve e ampiezze del succo di Balatro non sono pubblicate** dallo sviluppatore [NON TROVATO]. Un'analisi di terzi molto citata (blakecrosley.com) riporta valori CSS precisi, ma non dice da dove vengano: non è stata catturata né usata.
- **Il rovescio, dai giocatori.** L'ondeggiare continuo delle carte e dello sfondo dà nausea a una parte dei giocatori; l'opzione «Screenshake» ferma l'oscillazione globale, ma le carte continuano a muoversi e a cambiare dimensione [V] (fonti/balatro-steam-motion-sickness.md:18-22, :93). Chi gioca da molte ore chiede animazioni più veloci; c'è un'opzione di velocità che arriva a 4 e c'è chi la trova ancora lenta [V] (fonti/balatro-steam-game-speed.md:17, :39, :63). Sono testimonianze di giocatori, non dati dello sviluppatore.

### Papers, Please (Lucas Pope, 2013)

- Il timbro come caso di studio. Trascinare il timbro e applicarlo al rilascio del mouse «non dà soddisfazione»: serve «un bel THUNK solido quando premi, non quando lasci» [V] (fonti/papers-please-devlog-tig-2012-12.md:397). Da qui la barra dei timbri fissa che scorre sopra la scrivania: si muovono i documenti, non i timbri [V] (fonti/papers-please-devlog-tig-2012-12.md:399).
- Timbrare è per Pope la parte più divertente e non va saltata, perché il gioco è fatto di procedura [V] (fonti/papers-please-devlog-tig-2012-12.md:560); «non ci sono abbastanza timbri nei giochi» [V] (fonti/papers-please-devlog-tig-2012-12.md:601).
- Con una palette limitata una dissolvenza che attraversa molte tinte intermedie «sembra fuori posto» [V] (fonti/papers-please-devlog-tig-2012-12.md:89).
- Sullo schermo piccolo la scrivania non regge: c'è «un conflitto di fondo fra leggibilità e spazio per disporre le cose», e nella versione per telefono Pope ha tolto scrivania e trascinamento per andare tutto sulla leggibilità [V] (fonti/papers-please-devlog-mobile.md:103, :106). Ha però tenuto il timbro alla pressione [V] (fonti/papers-please-devlog-mobile.md:260). La risoluzione del gioco desktop è 570x320, con pixel grandi [V] (fonti/papers-please-devlog-mobile.md:62).

### Luck be a Landlord (TrampolineTales, Godot)

- Numeri tondi: invece di «dà 1,25 monete» il simbolo dà «5 monete ogni 4 giri», perché i numeri in base dieci si leggono meglio [V] (fonti/lbal-optimal-numbers.md:27, :35).
- Costo nascosto in Godot: con molti oggetti a schermo il gioco scendeva a 1 FPS; la causa era il controllo del passaggio del mouse su ogni oggetto, anche fuori schermo, e il rimedio è stato trattare diversamente ciò che non si vede [V] (fonti/lbal-performance-improvement.md:21, :32, :42).

### Cultist Simulator (Weather Factory, 2018)

- Il tavolo con le carte nasce perché il prototipo «grey box» funzionava ma non aveva attrattiva visiva [V] (fonti/cultist-thumbsticks-intervista.md:25). Le carte permettono molti pezzi d'arte eleganti e minimi a basso costo [V] (fonti/cultist-thumbsticks-intervista.md:29).
- Ritmo: timer quasi tutti da 60 o 30 secondi, numeri facili da ricordare; il risultato è che «succede qualcosa ogni dieci-quindici secondi» [V] (fonti/cultist-thumbsticks-intervista.md:73, :75). La telecamera non esce dal tavolo [V] (fonti/cultist-thumbsticks-intervista.md:81). Un codice colore guida l'interfaccia senza essere rigido [V] (fonti/cultist-thumbsticks-intervista.md:33).

### Inscryption (Daniel Mullins, 2021)

- Gran parte del lavoro sulle carte fu «preoccuparsi di continuo della leggibilità»: alla bassa risoluzione i dettagli piccoli non si vedevano, quindi niente testo sulle carte oltre al nome e icone sopra il ritratto quando serve [V] (fonti/inscryption-gamedeveloper-igf.md:51).

### Loop Hero (Four Quarters, 2021)

- Pochi elementi, ciascuno rifinito [V] (fonti/loophero-gamedeveloper-postmortem.md:96). Il costo del succo nella fase finale: «una piccola animazione qui, un po' di movimento là, ridisegnare questo pannello»; un artista ha speso un giorno intero per uno shader appena visibile su una funzione che pochi conoscono [V] (fonti/loophero-gamedeveloper-postmortem.md:110).

### Slay the Spire (Mega Crit, 2019)

- Le combinazioni vanno premiate: la gioia è vedere il mazzo funzionare «come una macchina di Rube Goldberg ben oliata» [V] (fonti/slaythespire-gamedeveloper-igf.md:45-47). Nessuna fonte dello sviluppatore sui tempi delle animazioni [NON TROVATO].

### Senza fonte dello sviluppatore

Card Shark (Nerial), Potion Craft (niceplay) e Book of Hours (Weather Factory): nessuna intervista o postmortem degli sviluppatori sul movimento o sull'interfaccia trovata su Game Developer, GDC Vault o blog degli studi; solo recensioni [NON TROVATO].

## 1. Quali effetti fanno il lavoro

I giochi non pubblicano parametri, quindi le misure vengono da un sistema di design che li pubblica (IBM Carbon) e dalle curve standard di easings.net. Le ricette nella colonna di destra sono nostre [INF].

| Effetto | Chi lo usa | Parametri di riferimento | Ricetta proposta [INF] |
| --- | --- | --- | --- |
| Risposta alla pressione | Papers, Please (fonti/papers-please-devlog-tig-2012-12.md:397) | microinterazioni fra 90 e 120 ms, `ease-out` sull'input [V] (fonti/carbon-motion.md:128) | `scale(0.97)` su `pointerdown`, 70-90 ms |
| Inclinazione e scala al passaggio del mouse | Balatro (fonti/balatro-pcgamer-jiggle.md:23) | ampiezze non pubblicate [NON TROVATO] | `rotate(±1,5-2°) scale(1.03-1.05)`, 150 ms, curva con rimbalzo |
| Rimbalzo d'arrivo | curva easeOutBack, `cubic-bezier(0.34, 1.56, 0.64, 1)` [V] (fonti/easings-net-yml.md:105-106) | supera il bersaglio del 9,8% al 57% della durata e torna [M] (campionamento della curva di Bézier in Python, 10.001 punti) | timbri, figurine, delta che compaiono |
| Ingresso di un pannello | Carbon, entrata espressiva `cubic-bezier(0, 0, 0.3, 1)` [V] (fonti/carbon-motion.md:80) | 240 ms per espansioni e messaggi di sistema, 400 ms per espansioni grandi [V] (fonti/carbon-motion.md:97-102) | fascicoli e voci dell'esito, 240 ms, sfasati di 80-90 ms |
| Movimento di lavoro | Carbon, standard produttivo `cubic-bezier(0.2, 0, 0.38, 0.9)` [V] (fonti/carbon-motion.md:51) | 70-150 ms [V] (fonti/carbon-motion.md:97-99) | tutto ciò che si ripete molte volte per turno |
| Momenti rari | Carbon, standard espressivo `cubic-bezier(0.4, 0.14, 0.3, 1)` [V] (fonti/carbon-motion.md:52) | «riservare il movimento espressivo a momenti occasionali e importanti» [V] (fonti/carbon-motion.md:21) | prima pagina, fine legislatura, crisi |
| Effetto proporzionale al valore | Balatro, fiamme sul punteggio alto (fonti/balatro-localthunk-timeline.md:88) | durata proporzionale alla distanza percorsa o alla scala [V] (fonti/carbon-motion.md:85) | solo sulla voce più grande dell'esito, una per turno |
| Suono all'impatto | Papers, Please, il THUNK (fonti/papers-please-devlog-tig-2012-12.md:397); tabelloni a palette, il cui rumore metallico richiama l'attenzione [V] (fonti/wikipedia-split-flap.md:40) | — | un suono per gesto, non per elemento; vedi [[suono-e-musica-leggeri]] |

Regole che ne escono [INF]:
- Il feedback parte alla pressione, non al rilascio (Pope). Anche su Steam Deck con il touch.
- Due curve, non una: produttiva per i gesti frequenti, espressiva per i momenti rari, più una curva con rimbalzo usata solo all'arrivo di un oggetto piccolo.
- L'ampiezza segue l'importanza, ma la durata resta sotto i 400 ms tranne la prima pagina.

## 2. Come restano leggibili i numeri mentre si muovono

- **Cifre tabellari.** `font-variant-numeric: tabular-nums` attiva cifre tutte larghe uguali, che si allineano come in tabella [V] (fonti/mdn-font-variant-numeric.md:159). Misurato sui font candidati [M] (`venv/bin/python -I feat.py <font>` con fontTools sui file di `google/fonts` ramo `main`, 17/10/2026): IBM Plex Sans ha già le cifre di larghezza uguale senza attivare nulla; Atkinson Hyperlegible Next, Bricolage Grotesque, Barlow e Barlow Condensed hanno la funzione `tnum`; Fraunces e Big Shoulders non ce l'hanno, quindi solo per titoli.
- **Il numero sta fermo, cambia il contenuto.** Con cifre tabellari un conteggio non fa ballare la colonna (vedi [[tipografia-e-font-aperti]]). Il contenitore non si sposta e non scala mai sotto la dimensione di riposo; il «pop» è un ingrandimento breve e poi il ritorno [INF].
- **Conteggio breve, poi immobile.** Animare la transizione fra due stati e poi fermarsi, mai la serie (fonti/robertson-2008-animation-trend.md:13, già in [[movimento-e-feedback]]). Conteggio da 300 a 500 ms con easeOutQuint `cubic-bezier(0.22, 1, 0.36, 1)` [V] (fonti/easings-net-yml.md:59-60): la curva raggiunge il 90% del valore al 37% della durata [M] (stesso campionamento), quindi la cifra finale si legge per quasi due terzi del tempo [INF].
- **Delta separato dal valore.** Il valore nuovo si scrive subito, la differenza («+1,2») compare accanto come un bollino con il suo colore e la sua freccia: forma subito, dettaglio dopo, come in Townscaper ([[movimento-e-feedback]]) [INF].
- **Numeri tondi.** Come «5 monete ogni 4 giri» (fonti/lbal-optimal-numbers.md:35): a schermo percentuali senza decimali dove il decimale non cambia la decisione; lo spread in punti base interi [INF].
- **Poco testo sulle carte.** Inscryption ha tolto il testo dalle carte per la leggibilità (fonti/inscryption-gamedeveloper-igf.md:51): sulle schede dei ministri bastano nome, dicastero, un numero e un'icona; il resto nel tooltip [INF].
- **Nella mappa, niente dissolvenze fra tinte.** Una dissolvenza attraversa colori intermedi che sembrano classi che non esistono, lo stesso difetto che Pope vede con la palette limitata (fonti/papers-please-devlog-tig-2012-12.md:89). Il colore della regione si scambia di colpo e il movimento va sul contorno [INF].
- **Prototipo HTML.** `@property` registra una proprietà personalizzata tipizzata, che così si può animare in CSS senza JavaScript (fonti/mdn-at-property.md:66, :125) [V]; con `syntax: "<integer>"` e `counter()` si ottiene un conteggio di soli numeri interi [INF, da provare nei browser]. In Godot lo stesso effetto si fa con `tween_method` su una funzione che scrive il testo di una Label (fonti/godot-tween-class.md:564) [V].

## 3. Cosa costa in produzione

- **Tempo di rifinitura senza fondo.** Loop Hero: giorni interi per effetti che pochi vedono (fonti/loophero-gamedeveloper-postmortem.md:110). LocalThunk elenca suoni, edizioni speciali delle carte e altro fra le aggiunte che gonfiano il perimetro (fonti/balatro-localthunk-timeline.md:74) [V]. Da noi [INF]: un elenco chiuso di ricette (le sei del paragrafo 1), scritto prima, e nessun effetto fuori elenco.
- **Le carte sono economiche come arte.** Cultist Simulator le sceglie per tenere bassi i costi (fonti/cultist-thumbsticks-intervista.md:29) [V]. Una scheda ministro è un rettangolo con un ritratto e tre campi; il costo sta nei ritratti, non nel movimento [INF].
- **Prestazioni.** In Godot i Tween sono «oggetti leggeri» per animare via script (fonti/godot-tween-class.md:13) e non dipendono dal renderer, quindi funzionano anche con Compatibility [INF]. Il rischio misurato è un altro: il controllo del mouse su centinaia di elementi (fonti/lbal-performance-improvement.md:32). Con 10 schede, 20 regioni e una rete di qualche decina di nodi il numero resta piccolo [INF].
- **Accessibilità.** Il codice per spegnere il movimento è poco, ma va previsto per ogni ricetta. Carbon chiede di offrire sempre un'alternativa statica a ogni transizione di stato (fonti/carbon-motion.md:140) [V].

## 4. Cosa regge in un turno da 5-10 minuti

Durata obiettivo del turno ordinario 5-10 minuti, esito con al massimo 7 voci in evidenza e tre indicatori sintetici (consenso, maggioranza, spread) (docs/ricerca/sintesi/decisioni.md:121-122).

- **Bilancio del movimento [INF].** Esito: 7 voci × 240 ms con uno sfasamento di 90 ms fanno circa 0,8 s di ingressi, più un conteggio di 400-500 ms sui tre indicatori; con la prima pagina (400 ms) si resta sotto i 2 secondi, cioè meno dell'1% di un turno di 300-600 s. Il resto del movimento sono gesti da 70-150 ms. Il succo non mangia tempo; il rischio è la ripetizione.
- **La ripetizione stanca.** Un giocatore di Balatro scrive che «più gioco, più noto le animazioni lente» e chiede di poterle accelerare (fonti/balatro-steam-game-speed.md:17) [V]. Carbon: se il movimento viene notato spesso dall'utente medio, va ridotto o tolto (fonti/carbon-motion.md:136) [V]. Quindi [INF]: velocità del turno a tre livelli (1×, 2×, istantaneo); un clic o un tasto salta la rivelazione e porta tutto allo stato finale. In Godot: `set_speed_scale` per tween (fonti/godot-tween-class.md:413-415), `Engine.time_scale` per tutto il resto con `set_ignore_time_scale` dove serve (fonti/godot-tween-class.md:369), `custom_step` con un delta più lungo della durata per chiudere subito un'animazione (fonti/godot-tween-class.md:261) [V].
- **Niente movimento ambientale.** In Balatro è proprio l'ondeggiare continuo, non i singoli effetti, a dare nausea, anche sul televisore di casa (fonti/balatro-steam-motion-sickness.md:47, :93) [V]. Da noi lo schermo fermo resta fermo; si muove solo ciò che il giocatore tocca o ciò che è cambiato (coerente con [[movimento-e-feedback]]).
- **Ritmo di riferimento.** In Cultist Simulator qualcosa succede ogni 10-15 secondi ed è un ritmo «alto ma gestibile» (fonti/cultist-thumbsticks-intervista.md:75) [V]. In un turno di 5-10 minuti questo dà da 20 a 60 momenti in cui il gioco risponde: sono questi a meritare un suono e un piccolo rimbalzo, e non di più [INF].
- **Il gesto che si ripete di più merita il succo migliore** (Dorfromantik, in [[movimento-e-feedback]]): da noi sollevare e premere una scheda, timbrare una decisione, chiudere il turno.

## Vincoli del progetto

- **Steam Deck.** Il testo va letto a 30 cm; «il carattere più piccolo a schermo non deve scendere sotto i 9 pixel di altezza a 1280x800», e Valve raccomanda di puntare a 12 px e di permettere di regolare dimensione e contrasto del testo (fonti/steamworks-deck-compat.md:108) [V, cattura di altro agente]. Le altre pagine leggono la regola come «corpo minimo 9 px». Se «altezza del carattere» vuol dire l'altezza della x, il corpo minimo sale intorno a 17-18 px: il rapporto x/em misurato è 0,516 per IBM Plex Sans, 0,496 per Atkinson Hyperlegible Next, 0,506 per Barlow, 0,600 per Big Shoulders [M] (stesso comando con fontTools, `sxHeight / unitsPerEm`). Proposta [INF]: corpo minimo 12 px per etichette statiche, 16 px per tutto ciò che si anima o è un numero dell'esito.
- **Godot Compatibility, solo 2D.** Le ricette sono traslazione, rotazione, scala, opacità e scambio di colore: nessuna richiede shader [INF]. Il «flip» delle palette dello stile 3 si fa con `scale.y` su due metà, non con una vera rotazione 3D [INF].
- **Movimento spegnibile.** In CSS: `prefers-reduced-motion: reduce` legge la preferenza di sistema (fonti/mdn-prefers-reduced-motion.md:23) [V]; scala e panoramica di oggetti grandi sono fra i fattori scatenanti (fonti/mdn-prefers-reduced-motion.md:25) [V]. In Godot 4.7 `DisplayServer.accessibility_should_reduce_animation()` restituisce 1, 0 o -1 se lo stato è ignoto, ma è implementato solo su macOS e Windows (fonti/godot-displayserver-4-7.md:1794-1798) [V]: su SteamOS (Linux) la preferenza di sistema non arriva, quindi l'interruttore nel gioco è obbligatorio, non un ripiego [INF]. Una nota di repository indica che questi metodi stanno passando a un `AccessibilityServer` dedicato: da verificare alla versione usata [NON TROVATO in documentazione stabile].
- **Contrasto.** Almeno 4,5:1 per il testo normale, 3:1 per il testo grande; le soglie non si arrotondano (fonti/wcag22-contrast-minimum.md:27, :57) [V]. Le palette sotto sono calcolate con la formula di luminanza relativa WCAG [M] (`venv/bin/python -I contrast.py`, scratchpad di sessione).

## Tre stili candidati (da validare nel prototipo)

Tutti e tre restano dentro B con la rete di D: carta, inchiostro, prima pagina di giornale, listino dell'agenda, schema dello Stato come vista secondaria, mappa a tinte piatte. Cambia la metafora che porta il gioco. Ricette comuni a tutti [INF], basate sui valori Carbon ed easings.net citati sopra:

```css
:root {
  --t-press: 80ms;   --t-hover: 150ms;  --t-enter: 240ms;  --t-big: 400ms;
  --e-work:  cubic-bezier(0.2, 0, 0.38, 0.9);   /* Carbon standard produttivo */
  --e-rare:  cubic-bezier(0.4, 0.14, 0.3, 1);   /* Carbon standard espressivo */
  --e-in:    cubic-bezier(0, 0, 0.3, 1);        /* Carbon entrata espressiva */
  --e-pop:   cubic-bezier(0.34, 1.56, 0.64, 1); /* easeOutBack */
  --e-hit:   cubic-bezier(0.55, 0, 1, 0.45);    /* easeInCirc, verso l'impatto */
  --e-count: cubic-bezier(0.22, 1, 0.36, 1);    /* easeOutQuint */
}
.num { font-variant-numeric: tabular-nums; }
@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after { animation: none !important; transition: none !important; }
}
/* stesso effetto con l'interruttore del gioco: html[data-motion="off"] */
```

Nel prototipo l'interruttore del gioco va accanto alla media query, perché su Deck la preferenza di sistema non arriva a Godot.

### Stile 1: Scrivania del Consiglio

**Metafora:** il turno si gioca su una scrivania: fascicoli, un mazzo di schede dei ministri e due timbri.

**Palette** (rapporti di contrasto [M]):
| Ruolo | Hex | Uso e contrasto |
| --- | --- | --- |
| Carta | `#F4EBD9` | fondo dei fogli |
| Panno della scrivania | `#24453A` | fondo dello schermo; carta sopra 8,92:1 |
| Inchiostro | `#1E1B18` | testo; 14,47:1 su carta |
| Matita | `#5C5346` | testo secondario; 6,38:1 su carta |
| Timbro rosso | `#B3261E` | costi, rifiuti; 5,52:1 su carta, carta su rosso 5,52:1 |
| Timbro blu | `#1F4E8C` | approvazioni, fili della rete; 7,02:1 su carta |
| Evidenziatore | `#F2C14E` | fondo per evidenziare; inchiostro sopra 10,21:1 |

**Font (OFL, verificati nella cartella `ofl/` di google/fonts [M]):** titoli Fraunces (https://fonts.google.com/specimen/Fraunces, asse WONK e SOFT per un tono meno serio, niente `tnum`); testo IBM Plex Sans (https://fonts.google.com/specimen/IBM+Plex+Sans, cifre già tabellari); cifre e timbri IBM Plex Mono (https://fonts.google.com/specimen/IBM+Plex+Mono).

**Forme:** fogli ad angoli vivi; nessun contorno; ombra corta e dura a riposo (`0 1px 0 rgba(0,0,0,.25)`), ombra morbida solo quando un foglio si solleva; grana della carta come immagine statica ripetuta, non uno shader; timbri a doppio bordo con una rotazione fissa per ogni istanza, fra -6° e 6°.

**Ricette di movimento:**
| Innesco | Effetto | Durata | Curva | Ampiezza |
| --- | --- | --- | --- | --- |
| passaggio del mouse o focus su una scheda | si solleva e si inclina | 150 ms (uscita 110 ms con `--e-work`) | `--e-pop` | `translateY(-6px) rotate(-1.5deg) scale(1.03)` |
| `pointerdown` su scheda o pulsante | si schiaccia | 80 ms | `--e-work` | `scale(0.97)` |
| `pointerdown` su conferma di una misura | timbro: scende, colpisce, il foglio si schiaccia | 120 ms di discesa + 140 ms di assestamento | `--e-hit`, poi `--e-pop` | timbro da `scale(1.35)` e opacità 0 a `scale(1)`; foglio `scale(1.015, 0.985)` all'impatto |
| apertura del turno | i fascicoli entrano uno alla volta | 240 ms, sfasati di 80 ms, massimo 7 | `--e-in` | da `translateY(24px) rotate(1deg)` e opacità 0 |
| valore che cambia | conteggio, poi bollino del delta | 400 ms + 150 ms | `--e-count`, poi `--e-pop` | bollino da `scale(0.6)` a 1 |

**Le cinque superfici:**
- *Cruscotto del turno:* la scrivania vista dall'alto; consenso, maggioranza e spread su tre cartellini fermati da graffette; le misure in corso come fogli impilati in posizioni fisse.
- *Prima pagina dell'esito:* il giornale viene posato sulla scrivania (400 ms, `--e-rare`); titolo in Fraunces nero; le 7 voci come trafiletti che entrano in sequenza, ciascuno con un piccolo timbro di esito.
- *Listino dell'agenda:* le 10 schede dei ministri a ventaglio, come una mano di carte; fermare o dirottare una proposta è un timbro «FERMATA» o «DIROTTATA» sulla scheda.
- *Schema dello Stato:* una bacheca di sughero con spilli e fili in inchiostro blu (la rete di D); il nodo toccato da una decisione pulsa una volta.
- *Mappa a tinte piatte:* una carta d'Italia aperta sulla scrivania; al cambio di classe il colore si scambia di colpo e il contorno della regione si ispessisce di 2 px e torna in 400 ms.

**Riferimenti:** Papers, Please per il timbro alla pressione (fonti/papers-please-devlog-tig-2012-12.md:397-399); Cultist Simulator per il tavolo con la telecamera fissa (fonti/cultist-thumbsticks-intervista.md:81); Balatro per le carte che si inclinano (fonti/balatro-pcgamer-jiggle.md:23).

**Rischio principale:** lo spazio. Sul Deck una scrivania con fogli sovrapposti diventa illeggibile, ed è esattamente ciò che ha portato Pope a togliere la scrivania nella versione per telefono (fonti/papers-please-devlog-mobile.md:103-106). Rimedio [INF]: posizioni fisse, nessun trascinamento libero, un foglio aperto alla volta. Secondo rischio: il tono grigio e oppressivo di Papers, Please, che è l'opposto di ciò che Mario chiede.

### Stile 2: Album di figurine

**Metafora:** il governo è un album da completare: ministri, misure e regioni sono figurine da attaccare. Le figurine adesive in bustina sono una tradizione italiana, con un formato tipico di 4,9 × 6,5 cm (fonti/wikipedia-it-panini.md:66) [V].

**Palette** (rapporti di contrasto [M]):
| Ruolo | Hex | Uso e contrasto |
| --- | --- | --- |
| Carta dell'album | `#FFF8EC` | fondo |
| Inchiostro blu notte | `#1B1F3B` | testo; 15,23:1 su carta, 16,08:1 sul bordo bianco delle figurine |
| Testo secondario | `#5A5F7A` | 5,93:1 su carta |
| Rosso | `#C8102E` | titoli e voci negative; 5,57:1 su carta, bianco sopra 5,88:1 |
| Giallo bollino | `#FFC145` | fondo dei numeri; inchiostro sopra 9,93:1 |
| Verde acqua | `#7FD1B9` | fondo delle voci positive; inchiostro sopra 8,98:1 |
| Blu | `#2E5EAA` | fondo dei pulsanti; bianco sopra 6,37:1 |

**Font (OFL [M]):** titoli Bricolage Grotesque (https://fonts.google.com/specimen/Bricolage+Grotesque, ha `tnum`); testo Atkinson Hyperlegible Next (https://fonts.google.com/specimen/Atkinson+Hyperlegible+Next, ha `tnum`); cifre Atkinson Hyperlegible Mono (https://fonts.google.com/specimen/Atkinson+Hyperlegible+Mono).

**Forme:** figurine con angoli arrotondati di 6 px, bordo bianco di 4 px e ombra netta spostata (`2px 3px 0 rgba(27,31,59,.2)`), senza sfocatura; spazi vuoti dell'album a contorno tratteggiato con il numero progressivo; numeri dentro bollini tondi; nessuna texture.

**Ricette di movimento:**
| Innesco | Effetto | Durata | Curva | Ampiezza |
| --- | --- | --- | --- | --- |
| passaggio del mouse o focus | la figurina si alza | 150 ms | `--e-pop` | `rotate(±2deg)` alternato per indice, `scale(1.05)`, ombra da 3 a 6 px |
| assegnare o confermare (rilascio nello spazio, o pressione) | si attacca e si schiaccia | 200 ms | `--e-pop` | `scale(1.08)` → `scale(1.04, 0.96)` → 1; il contorno tratteggiato diventa pieno in 110 ms |
| apertura del turno (eventi) | la bustina si apre e le figurine escono a ventaglio | 240 ms, sfasate di 90 ms, massimo 7 | `--e-in` | dal centro della bustina, con rotazione finale fra -8° e 8° |
| numero che cambia | il bollino pulsa, le cifre scorrono | 200 ms + 150 ms | `--e-pop`, poi `--e-work` | bollino `scale(1.15)` e ritorno; cifra vecchia esce in alto di 8 px, la nuova entra dal basso |
| pagina completata (raro) | un riflesso attraversa la pagina, una volta | 700 ms | `--e-rare` | gradiente che scorre, opacità massima 0,3 |

**Le cinque superfici:**
- *Cruscotto del turno:* la pagina del trimestre; consenso, maggioranza e spread in tre bollini grandi; gli spazi per le misure da attaccare.
- *Prima pagina dell'esito:* il giornale in edicola, titolo in Bricolage Grotesque; le 7 voci come figurine in fila che si scoprono una alla volta.
- *Listino dell'agenda:* le 10 figurine dei ministri inventati, con sotto lo spazio della proposta; fermare una proposta gira la figurina sul retro (`scaleX` da 1 a 0 a 1, 240 ms).
- *Schema dello Stato:* una doppia pagina con le figurine collegate da linee stampate, numerate come in un album.
- *Mappa a tinte piatte:* la pagina «Regioni d'Italia» con le 20 regioni come figurine sagomate; al cambio di classe la regione si riattacca (`scale(1.04)` e ritorno in 180 ms) con il colore nuovo.

**Riferimenti:** Balatro per la carta come oggetto succoso (fonti/balatro-pcgamer-jiggle.md:23) e per le immagini note che fanno entrare nel gioco (fonti/balatro-rogueliker-intervista.md:31); Cultist Simulator per l'economia delle carte come arte (fonti/cultist-thumbsticks-intervista.md:29); la figurina adesiva italiana (fonti/wikipedia-it-panini.md:66).

**Rischio principale:** il tono infantile su crisi, guerre e repressione; una figurina che rimbalza su un disastro stona. Rimedio [INF]: le voci negative gravi entrano senza rimbalzo, con `--e-work`. Secondo rischio: evocare troppo da vicino un marchio registrato; si prende il formato, non nome, logo o impaginazione ([[ip-e-giochi-di-riferimento]]). Terzo: il retro delle figurine raddoppia gli asset.

### Stile 3: Tabellone a palette

**Metafora:** lo Stato è una stazione; ogni numero è un tabellone a palette che gira quando cambia. I tabelloni a palette si chiamavano spesso «Solari», dal costruttore di Udine (fonti/wikipedia-split-flap.md:26) [V], e il loro rumore metallico richiama l'attenzione quando l'informazione cambia (fonti/wikipedia-split-flap.md:40) [V].

**Palette** (rapporti di contrasto [M]):
| Ruolo | Hex | Uso e contrasto |
| --- | --- | --- |
| Carta | `#F2EFE6` | fondo delle pagine; è anche il colore delle cifre sulle palette, 13,16:1 sul tabellone |
| Tabellone | `#262626` | fondo delle palette |
| Inchiostro | `#1A1A1A` | testo su carta; 15,14:1 |
| Giallo segnale | `#F6C324` | intestazioni sul tabellone 9,19:1; inchiostro su giallo 10,57:1 |
| Rosso su carta | `#C8102E` | voci negative su carta; 5,12:1 |
| Rosso su tabellone | `#FF6B5E` | «in ritardo»; 5,42:1 sul tabellone |
| Verde su tabellone | `#7BD389` | «in orario»; 8,30:1 sul tabellone |

**Font (OFL [M]):** titoli Big Shoulders (https://fonts.google.com/specimen/Big+Shoulders, niente `tnum`, solo titoli; la vecchia «Big Shoulders Display» ora risponde 404 su Google Fonts [M] con `curl`); cifre sulle palette Barlow Condensed SemiBold (https://fonts.google.com/specimen/Barlow+Condensed, ha `tnum`); testo Barlow (https://fonts.google.com/specimen/Barlow, ha `tnum`).

**Forme:** palette rettangolari con angoli di 2 px e una fessura orizzontale a metà (linea di 1 px, nero al 40%); righe allineate a griglia come un tabellone; nessuna ombra morbida; nessuna texture. Il resto dell'interfaccia è carta stampata come in B.

**Ricette di movimento:**
| Innesco | Effetto | Durata | Curva | Ampiezza |
| --- | --- | --- | --- | --- |
| carattere che cambia | la palette gira: si chiude la metà alta vecchia, si apre la metà bassa nuova | 60 + 60 ms per giro, al massimo 2 giri intermedi per carattere (≤ 360 ms) | `--e-hit`, poi `--e-in` | `scaleY` da 1 a 0 e da 0 a 1 sulle due metà; cascata di 30 ms per carattere, da destra |
| riga aggiornata | lo sfondo della riga si accende una volta | 400 ms | `--e-work` | da giallo al 35% di opacità a 0; un solo lampo, mai a ripetizione |
| passaggio del mouse su una riga | sottolineatura che cresce | 150 ms | `--e-work` | 2 px gialli, da sinistra a destra |
| nuova proposta in agenda | la riga entra dall'alto e spinge le altre | 240 ms | `--e-in` | `translateY(-100%)` → 0 |
| suono | un «clac» per riga, non per carattere | — | — | — |

**Le cinque superfici:**
- *Cruscotto del turno:* un piccolo tabellone «arrivi» con consenso, maggioranza e spread come righe (valore, delta, stato).
- *Prima pagina dell'esito:* giornale su carta, titolo in Big Shoulders; i numeri chiave dentro l'articolo sono riquadri a palette che girano all'apertura.
- *Listino dell'agenda:* il tabellone delle partenze: 10 righe per 10 ministri, con dicastero al posto del binario, proposta al posto della destinazione, stato «in orario», «fermata» o «dirottata».
- *Schema dello Stato:* lo schema di una rete ferroviaria, linee e stazioni; la rete di D vi entra in modo naturale (Mini Metro è già un riferimento di D).
- *Mappa a tinte piatte:* tinte piatte ferme; il valore regionale appare in una mini-palette al passaggio del mouse.

**Riferimenti:** i tabelloni Solari (fonti/wikipedia-split-flap.md:26, :40); Luck be a Landlord e Balatro per il punteggio come spettacolo (fonti/balatro-localthunk-timeline.md:32); Carbon per la differenza fra movimento produttivo ed espressivo (fonti/carbon-motion.md:21).

**Rischio principale:** scivolare nella sala operativa (direzione A) e nella dashboard, cioè di nuovo «troppo serio». Secondo: decine di palette che girano insieme a ogni chiusura stancano, e il rumore diventa frastuono; per questo al massimo 2 giri per carattere e un suono per riga. Terzo: rosso e verde sul tabellone non bastano ai daltonici; serve sempre anche un'icona o una parola [INF].

## Per il nostro gioco

- **Prendiamo:** feedback alla pressione; due curve (produttiva ed espressiva) più un rimbalzo per gli arrivi piccoli; cifre tabellari e numeri tondi; valore subito e delta dopo; colori della mappa scambiati di colpo; velocità del turno a tre livelli con salto; interruttore del movimento nel gioco, perché su SteamOS Godot non legge la preferenza di sistema.
- **Evitiamo:** ondeggiamenti ambientali alla Balatro; dissolvenze fra tinte della mappa; testo sulle carte oltre a nome, dicastero e un numero; rimbalzi sulle voci gravi; effetti fuori da un elenco chiuso.
- **Aperto:** quale dei tre stili, da decidere sul prototipo; la lettura della regola dei 9 px di Valve (altezza della x o corpo); il suono di ogni ricetta ([[suono-e-musica-leggeri]]).

### Tensioni con altre pagine

- [[movimento-e-feedback]] propone «una sola curva di easing in tutto il gioco»; qui le curve sono due più il rimbalzo, sul modello di Carbon (fonti/carbon-motion.md:21, :51-52). La stessa pagina evita «rimbalzi su decisioni gravi»: resta valido, il rimbalzo qui va solo sui gesti e sulle voci non gravi.
- [[movimento-e-feedback]] raccomanda di rispettare la preferenza «riduci movimento» del sistema: su Steam Deck Godot 4.7 non la legge (fonti/godot-displayserver-4-7.md:1798), quindi l'opzione del gioco è il meccanismo principale.
- [[tipografia-e-font-aperti]] e [[direzioni-artistiche-candidate]] riportano «testo ≥ 9 px» sul Deck; la fonte parla del carattere più piccolo alto 9 px e raccomanda 12 px (fonti/steamworks-deck-compat.md:108).

## Riferimenti

- **URL** https://www.pcgamer.com/games/localthunk-gave-up-making-balatro-for-3-months-but-resumed-because-i-was-bored-but-the-internet-was-out-so-i-couldnt-play-rocket-league/ · [V]
  **perché conta qui** unica dichiarazione dello sviluppatore di Balatro sul suo succo, riportata testualmente
  **cosa se ne prende** l'oscillazione in rotazione e scala delle carte come primo lavoro, cioè come cuore del feel
- **URL** https://localthunk.com/blog/balatro-timeline-3aarh · [V]
  **perché conta qui** cronologia scritta dallo sviluppatore
  **cosa se ne prende** il punteggio CHIP × MULT, le fiamme sul punteggio alto, il perimetro che si gonfia con suoni ed effetti
- **URL** https://gameinformer.com/interview/2024/03/21/balatro-was-almost-called-joker-poker-and-other-details-from-its-creator · [V]
  **perché conta qui** intervista allo sviluppatore
  **cosa se ne prende** il CRT entrato per gusto; i tooltip aggiunti dopo la beta
- **URL** https://rogueliker.com/balatro-interview/ · [V]
  **perché conta qui** intervista allo sviluppatore
  **cosa se ne prende** immagini note e tattili come ingresso al gioco; tarare «a sensazione»
- **URL** https://steamcommunity.com/app/2379780/discussions/0/4201364375109001844/ · [V, testimonianze di giocatori]
  **perché conta qui** il costo del succo continuo sui giocatori sensibili al movimento
  **cosa se ne prende** niente movimento ambientale; l'interruttore deve fermare tutto, non solo lo screenshake
- **URL** https://steamcommunity.com/app/2379780/discussions/0/4346606879508270539/ · [V, testimonianze di giocatori]
  **perché conta qui** la stanchezza da ripetizione dopo molte ore
  **cosa se ne prende** velocità del turno regolabile e salto della rivelazione
- **URL** https://dukope.com/devlogs/papers-please/tig-01/ · [V]
  **perché conta qui** diario di sviluppo di Lucas Pope sul timbro e sulla palette
  **cosa se ne prende** feedback alla pressione, timbri fissi e documenti mobili, niente dissolvenze fra molte tinte
- **URL** https://dukope.com/devlogs/papers-please/mobile/ · [V]
  **perché conta qui** lo stesso gioco portato su uno schermo piccolo
  **cosa se ne prende** il conflitto fra leggibilità e scrivania, rischio principale dello stile 1
- **URL** https://www.gamedeveloper.com/design/how-game-jam-sacrifices-became-inscryption · [V]
  **perché conta qui** carte leggibili a bassa risoluzione
  **cosa se ne prende** sulle carte solo nome e icone, il resto altrove
- **URL** https://www.thumbsticks.com/interview-unseen-arts-behind-cultist-simulator/ · [V]
  **perché conta qui** perché un tavolo con carte, e con quale ritmo
  **cosa se ne prende** carte come arte economica, telecamera fissa, qualcosa ogni 10-15 secondi
- **URL** https://www.gamedeveloper.com/design/postmortem-loop-hero · [V]
  **perché conta qui** postmortem degli sviluppatori sul costo della rifinitura
  **cosa se ne prende** elenco chiuso di effetti, perché la rifinitura non ha fondo
- **URL** https://www.gamedeveloper.com/disciplines/road-to-the-igf-mega-crit-games-i-slay-the-spire-i- · [V]
  **perché conta qui** il piacere della combinazione che scatta
  **cosa se ne prende** l'esito del turno come catena di cause rivelata in ordine
- **URL** https://blog.trampolinetales.com/optimal-numbers/ · [V]
  **perché conta qui** leggibilità dei numeri in un gioco di soli numeri
  **cosa se ne prende** numeri tondi a schermo
- **URL** https://blog.trampolinetales.com/lbal-performance-improvement/ · [V]
  **perché conta qui** costo di prestazioni in un gioco di carte fatto con Godot
  **cosa se ne prende** il controllo del mouse su molti elementi costa; trattare a parte ciò che è fuori schermo
- **URL** https://carbondesignsystem.com/elements/motion/overview/ · [V]
  **perché conta qui** unico riferimento letto che pubblica durate e curve misurate
  **cosa se ne prende** i token 70-700 ms, le curve produttiva ed espressiva, la regola sul movimento troppo notato
- **URL** https://raw.githubusercontent.com/ai/easings.net/master/src/easings.yml · [V]
  **perché conta qui** valori CSS delle curve standard citate dalla documentazione di Godot
  **cosa se ne prende** easeOutBack, easeOutQuint, easeInCirc in `cubic-bezier`
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion · [V]
  **perché conta qui** il prototipo HTML deve rispettare la preferenza di sistema
  **cosa se ne prende** la media query e quali movimenti sono più a rischio
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/font-variant-numeric · [V]
  **perché conta qui** numeri che non ballano mentre cambiano
  **cosa se ne prende** `tabular-nums`
- **URL** https://developer.mozilla.org/en-US/docs/Web/CSS/@property · [V]
  **perché conta qui** animare un valore in CSS puro nel prototipo
  **cosa se ne prende** proprietà registrate e animabili, base del conteggio senza JavaScript
- **URL** https://docs.godotengine.org/en/stable/classes/class_tween.html · [V]
  **perché conta qui** l'API con cui le ricette passano dal prototipo a Godot
  **cosa se ne prende** `set_speed_scale`, `set_ignore_time_scale`, `custom_step`, `tween_method`
- **URL** https://docs.godotengine.org/en/stable/classes/class_displayserver.html · [V]
  **perché conta qui** lettura della preferenza «riduci animazioni» in Godot 4.7
  **cosa se ne prende** funziona solo su macOS e Windows: sul Deck serve l'interruttore nel gioco
- **URL** https://en.wikipedia.org/wiki/Split-flap_display · [V]
  **perché conta qui** fonda lo stile 3
  **cosa se ne prende** la metafora italiana dei tabelloni Solari e il suono che segnala l'aggiornamento
- **URL** https://it.wikipedia.org/wiki/Panini_(azienda) · [V]
  **perché conta qui** fonda lo stile 2
  **cosa se ne prende** la figurina adesiva in bustina e il suo formato
- **URL** https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum.html · [V]
  **perché conta qui** soglia di contrasto delle palette
  **cosa se ne prende** 4,5:1 per il testo normale, 3:1 per il grande, senza arrotondare
- **URL** https://partner.steamgames.com/doc/steamdeck/compat · [V, cattura di altro agente]
  **perché conta qui** vincolo di leggibilità sul Deck
  **cosa se ne prende** il carattere più piccolo alto almeno 9 px, consigliati 12 px
- **URL** https://fonts.google.com/specimen/Fraunces, https://fonts.google.com/specimen/IBM+Plex+Sans, https://fonts.google.com/specimen/IBM+Plex+Mono, https://fonts.google.com/specimen/Bricolage+Grotesque, https://fonts.google.com/specimen/Atkinson+Hyperlegible+Next, https://fonts.google.com/specimen/Atkinson+Hyperlegible+Mono, https://fonts.google.com/specimen/Big+Shoulders, https://fonts.google.com/specimen/Barlow+Condensed, https://fonts.google.com/specimen/Barlow · [M]
  **perché conta qui** font dei tre stili
  **cosa se ne prende** licenza OFL (cartella `ofl/` con `OFL.txt` in google/fonts) e presenza di `tnum`, misurate con fontTools
- **URL** card shark, potion craft, book of hours · [NON TROVATO]
  **perché conta qui** erano fra i candidati del brief
  **cosa se ne prende** nulla finché non si trova un'intervista o un postmortem degli sviluppatori

## Collegamenti

- [[movimento-e-feedback]]: la base di questa pagina (juice, limiti, rivelazione a ritmo, WCAG 2.3.3).
- [[direzioni-artistiche-candidate]]: la direzione B con la rete di D, dentro cui stanno i tre stili.
- [[tipografia-e-font-aperti]]: cifre tabellari, OFL, regola dei 9 px.
- [[suono-e-musica-leggeri]]: il suono di ogni ricetta.
- [[data-viz-cinematica]]: animare le transizioni, non le serie.
- [[mappe-e-globo]]: classi stabili e colore al valore sulla mappa.
- [[papers-please]]: la pagina del gioco.
- [[godot]], [[prestazioni-su-hardware-modesto]]: Compatibility e consumo sul Deck.
- [[ip-e-giochi-di-riferimento]]: cosa si può prendere da un formato riconoscibile.
