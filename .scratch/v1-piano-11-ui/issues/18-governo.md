Type: prototype
Status: resolved
Blocked by: 06, 08, 32
Assignee: maeurong
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la sezione Governo? Il borsino dei dieci ministri (storia 154): caricatura, voto con freccia, motto; tabella densa con in evidenza solo le righe che meritano un clic (storia 27); l'atto in agenda col motivo (storia 26); Ferma e Dirotta che girano la carta (storie 28 e 37), con la regione bersaglio e il selettore (storia 36); a lato la carta del ministro con capacità, posizione, obiettivo, soddisfazione con le cause e minaccia di dimissioni (storie 25 e 33); richiesta di dimissioni, rosa dei successori, interim e rodaggio (storie 34-35); cantieri (storia 38); margine esaurito (storia 29); i dicasteri con indicatore di servizio e grandezze (storia 24). Variante in tono grave. La sezione è barrata quando il giocatore non governa.

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

Dall'11/11/2026 ([Testi del Pungolo](09-testi-del-pungolo.md)): il motto viene dal dicastero, quattro per ciascuno, scelto con un hash del nome del ministro; un dicastero senza motti lascia il ministro senza motto.

## Answer

Deciso con Mario il 12/11/2026 in un giro su un prototipo navigabile: [18-governo.prototype.html](../prototipi/18-governo.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati: la Vista 05 e [viste/18-governo.json](../viste/18-governo.json), che `genera/` aggiunge giocando: il 03 con Ferma, Dirotta sull'atto, Dirotta sulla regione e dimissioni chieste, comandati davvero; l'interim con la rosa, il successore in rodaggio, un tecnico dalla spartizione, i ministri scontenti (seme 4, turno 9) e il tono grave dell'08. I costi di Ferma, Dirotta e Ferma il cantiere sono misurati comandando una copia della partita, ministro per ministro e regione per regione: Ferma costa da 12 a 29 punti di soddisfazione nel 03, Dirotta sulla regione da 1 a 21.

### Forma: B · Figurine

Sotto la cornice, l'album a pagina col **dettaglio a lato**.

- **Dieci carte, cinque per due**, nell'ordine dei dicasteri dei dati. Ogni carta: il numero, lo stato (in preavviso, in rodaggio, tecnico, dimissioni chieste, interim), il **voto con la freccia**, la caricatura a 62 px, nome, dicastero e partito, il motto, l'atto in agenda con la regione e il costo, «Ferma −N» e «Dirotta ›». Il voto è la soddisfazione; la freccia somma le variazioni degli ultimi due turni, comandi del turno compresi.
- **In evidenza** (storia 27): un filo rosso in testa alla carta. Le altre restano uguali.
- **Senza agenda** la carta dice perché, in un riquadro tratteggiato: margine esaurito, interim, esercizio provvisorio.
- **Ferma e Dirotta girano la carta**: sul retro il bollo «Fermato» o «Dirottato», l'atto, il voto nuovo e «rigira». Web Animations, con la molla di [Movimento e scala in WKWebView](04-movimento-e-scala-in-wkwebview.md). Nel tono grave la carta non gira, la faccia diventa le iniziali e il motto sparisce.
- **La carta del ministro** nel dettaglio a lato: faccia a 84 px, capacità a pallini (nel rodaggio un pallino a retino), motto; la soddisfazione con la barra, le soglie di 25 e 40 e le cause; la posizione sull'asse accanto alla tua; l'obiettivo, cioè l'indicatore di servizio contro l'obiettivo del ministro; l'atto in agenda col perché dell'evidenza; il margine del dicastero; la proposta in sospeso col rimando al Cruscotto; i cantieri; le dimissioni.
- **Dirotta** apre a lato i quattro atti del dicastero: costo, turni, effetti, la soddisfazione che costa, oppure il rifiuto del core («margine insufficiente», «cantiere già in corso»). Per Sviluppo economico e Infrastrutture c'è anche la **regione bersaglio**: una mappa piccola in cinque classi di soddisfazione persa e la lista delle venti regioni col costo, come deciso in [Mappa SVG delle regioni](02-mappa-svg-delle-regioni.md). Atto e regione si scelgono insieme, poi «Dirotta · −N».
- **Interim**: la rosa dei tre successori nel dettaglio a lato, con faccia (regola della rosa di [Disegno delle caricature](32-disegno-delle-caricature.md)), capacità e posizione. Il default è scritto: senza scelta entra il primo, il più capace (`crates/sim-core/src/ministri/mod.rs:306`).
- **Il dettaglio a lato scorre da solo** quando la carta ha cantieri o Dirotta apre le venti regioni; l'album resta fermo. A 1280×800 l'album sta nella pagina in ogni stato giocato.

### Decisioni sui casi

- **Ferma diretto**, col costo scritto sul bottone («Ferma −18»), come Accogli e Respingi nel [Cruscotto al governo](17-cruscotto-al-governo.md). Il core non lo ritira (`crates/sim-core/src/ministri/agenda.rs:380-382`): il costo sul bottone basta a renderlo una scelta informata.
- **Due conferme sul posto in più**: «Chiedi le dimissioni», che dice il prezzo (gratis con un tecnico, 10 punti d'intesa a un alleato, niente col tuo partito, `crates/sim-core/src/partita.rs:1584-1595`), e «Ferma il cantiere», che dice quanti euro già spesi si perdono (storia 38). Con quelle dell'ossatura e «Dimettiti» sono sei.
- **Le grandezze del dicastero** (storia 24): l'indicatore di servizio sta nella carta; le altre si leggono negli effetti di Dirotta e in [Conti](21-conti.md), dove le 21 grandezze si aprono a lato.
- **Crisi**: la banda nera dell'ossatura; le carte in interim con la rosa, le altre come sono.
- **Motti**: uno per dicastero, dal prototipo vincolante, come primo dei quattro ([Testi del Pungolo](09-testi-del-pungolo.md)); gli altri si scrivono nella fetta.

### Guscio e dati

I comandi passano da `comanda` (`Ferma`, `Dirotta`, `FermaCantiere`, `ChiediDimissioni`, `ScegliSuccessore`); nessun passacarte nuovo. Il resto la UI lo legge dalla Vista, tranne quattro dati che oggi il prototipo rifà dai file del core. Sono **quattro rimandi al core, PR delle esposizioni, in 11g**:

- **I costi di Ferma e Dirotta**: `VoceAgenda.costo_ferma`, `VoceCantiere.costo_ferma` e `costo_dirotta` sono già nella spec del core (`docs/superpowers/specs/2026-11-11-esito-e-vista-completa.md:501-504`). Fino ad allora i bottoni restano senza numero.
- **L'indicatore di servizio di ogni dicastero**: `VoceDicastero` porta solo `id` e `nome` (`crates/sim-core/src/vista.rs:485-488`); l'indicatore sta in `dati/base/dicasteri.toml:9`.
- **Il perché dell'evidenza**: `VoceAgenda.in_evidenza` è un booleano (`crates/sim-core/src/vista.rs:552`) che unisce tre regole (minaccia, atto lontano dalla posizione del Presidente del Consiglio, evento nel dicastero: `crates/sim-core/src/ministri/agenda.rs:570-576`). La carta deve dire quale vale; il prototipo rifà la regola con le posizioni di `dati/base/atti.toml`.
- **Le soglie della soddisfazione (25 e 40) e il costo d'intesa delle dimissioni chieste**: sono manopole (`dati/base/manopole.toml:4-11`, `:1164-1168`). La Vista porta solo `minaccia`, e la barra, «si dimette a fine turno» e il prezzo delle dimissioni ne hanno bisogno.

Il genere dei ministri resta il rimando di [Disegno delle caricature](32-disegno-delle-caricature.md).

### Scartato

A · Listino (tabella densa da borsino, una riga per ministro); C · Per partito (una colonna per partito con l'intesa in testa); Ferma con la conferma sul posto; la regione con la sola lista.
