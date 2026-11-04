---
tipo: sintesi
fonti: [fonti/princeton-rodriguez-stable-populations.md, fonti/civ7-update-check-in-2025-06-10.md, fonti/civ7-patch-1-2-4-1-2025-08-27.md, fonti/frostpunk2-gamepressure-difficolta.md, fonti/terrainvicta-gamespress-patch11-2023.md, fonti/cliffsblog-2013-too-easy.md]
aggiornato: 2026-10-29
---
# Cursori dello scenario: cablaggio sul codice e costo del riepilogo

Come si collegano i sei cursori e i due menu dell'editor dello scenario (ticket 21, spec righe 27-32 e 274-285) al motore di oggi: quali ingressi muovono, dove il motore li legge, cosa ricalcola `Partita::costruisci` e cosa manca. Poi gli intervalli, le disposizioni cambiate dall'editor, il file di differenze, il costo misurato del riepilogo dal vivo e un confronto breve con quattro giochi. Le decisioni del 18/10 (`.scratch/v1-giocabile/issues/21-editor-dello-scenario.md`, `docs/ricerca/sintesi/decisioni.md:247-253`) non si rimettono in discussione: qui si dice dove il codice le regge e dove no.

Tag: [V] letto (codice in questa sessione, o fonte primaria); [M] misurato con comando; [INF] inferenza; [NON TROVATO]. Codice letto su worktree `docs-v1-piano-10`, branch `docs/v1-piano-10`, HEAD `95f18cb`.

## Premesse del brief verificate

- `scenario.rs:10-16` `Ingresso { valore, min, max, fonte }`, `scenario.rs:99-130` gli ingressi della macro, `scenario.rs:147-158` `Scenario::valida` che rifiuta col nome: confermato [V].
- `partita.rs:132-242` `costruisci` chiama `Economia::iniziale`, `Grandezze::iniziali`, `Elettorato::iniziale`, `Elettorato::avvia_voto`, `Informazione::nuova`, `Disposizioni::iniziali`, `Ministri::iniziali`: confermato, con `avvia_voto` (riga 146) e `Ministri::iniziali` (righe 199-214) in più rispetto al brief [V].
- `fondazione.rs:130-151` `anteprima_fondazione` costruisce una partita intera per posizione, commento `ponytail:` alle righe 136-138: confermato [V]. Precisazione che conta: legge la quota dal **sondaggio rumoroso** (`fondazione.rs:141,146`, `quota_fra_i_votanti` sugli istituti), mentre il riepilogo dell'editor vuole il sondaggio atteso senza rumore (spec riga 283). Quindi il riepilogo non può essere `anteprima_fondazione` così com'è [V].
- `struttura.rs:202` `valida_piramide`: confermato; la piramide sta in `economia.toml:46-58` (101 età), non in `scenario.toml` [V].

## 1. I sei cursori e i due menu sul codice di oggi

Tre cursori sono già un ingresso dello scenario con il suo intervallo (debito, disoccupazione, inflazione); tre sono trasformazioni sintetiche di più file (ricchezza, età, orientamento) che oggi non esistono in nessuna forma [V, dalla lettura che segue].

### Debito (% del PIL)

- **Muove**: `debito_pil` (`scenario.toml:46-50`, 136,7, intervallo 60-200). Costo medio invariato: `costo_medio_debito` (`scenario.toml:52-56`) [V].
- **Letto da**: `economia/mod.rs:118` (debito in euro = `debito_pil` × 4 trimestri di PIL), `economia/mod.rs:166` (tasso medio = costo medio), `finanza.rs:246` (interessi = tasso medio × debito / 4), `spread.rs:24-35` (quadratica su debito e disavanzo dell'anno mobile) [V].
- **Ricalcola `costruisci`**: interessi, disavanzo, spread al turno zero (`economia/mod.rs:178-195`). Bilancio 2027: `finanza.rs:174-178` lo scala con min(inflazione, tetto), quindi non dipende dal debito [V].
- **Misura** [M, comando in §5, binario `derivati`]: con debito 100 / 120 / 130 / 135,5 / 140 / 150 / 160 / 170 / 180% lo spread vale 60 / 60 / 60 / 60,8 / 148,9 / 358,2 / 585,6 / 831,0 / 1000 pb, gli interessi 2,81 / 3,37 / 3,65 / 3,81 / 3,93 / 4,21 / 4,50 / 4,78 / 5,06% del PIL, il disavanzo 1,91 / 2,47 / 2,75 / 2,90 / 3,03 / 3,31 / 3,59 / 3,87 / 4,15%. Base: 83,9 pb, 3,84%, 2,94%. La curva (`manopole.toml:289-318`: s0 −883, β₁ −0,0523, β₂ 0,0009, minimo 60, massimo 1000) è piatta sul minimo sotto il 135,5% e tocca il tetto di 1000 pb verso il 177%: è tarata solo intorno al punto base [M+INF].
- **Manca**: niente per il cablaggio. Manca il significato fuori dal punto tarato (sotto 135% lo spread non risponde più al cursore) [INF].

### Ricchezza (PIL pro capite, ± %)

- **Muove oggi, se si scala solo `pil_trimestrale`** (`scenario.toml:40-44`, 576,9, intervallo 400-800): budget dei dicasteri in proporzione, perché `spesa_pil`, `salari_pil` e `acquisti_pil` sono in % del PIL (`dicasteri.toml:13`, `finanza.rs:118-141`); tutte le entrate in proporzione, perché sono quote del PIL potenziale (`finanza.rs:193-195, 236-239`) [V].
- **Non si muove e dovrebbe** [V]: le spese in miliardi di euro restano ferme, cioè `spesa_pensioni` (`scenario.toml:166-170`, letta in `previdenza.rs:39`), `spesa_sussidi` (`scenario.toml:178-182`, `previdenza.rs:41`), `altre_prestazioni` (`scenario.toml:184-188`, `finanza.rs:245`); i redditi dei gruppi restano quelli in euro di `elettorato.toml` (`DatoGruppo.primario`/`trasferimenti`, `elettorato/mod.rs:133-142`), perché al turno zero gli indici di base coincidono con l'economia appena costruita e il rapporto vale 1 (`redditi.rs:93-110`, `elettorato/mod.rs:382-406`).
- **Misura** [M]: `pil_trimestrale` 461,5 (−20%) dà disavanzo **8,01%** del PIL e spread 95,9 pb; 692,3 (+20%) dà **−0,44%**. È l'effetto delle spese in euro rimaste ferme, non della ricchezza.
- **Soglie in euro che restano ferme** [V]: scaglioni IRPEF `scenario.toml:34-38` (28.000 e 50.000 €); `pensione_minima` `scenario.toml:172-176` (7.800 €, letta in `previdenza.rs:66`); salario minimo e reddito minimo come leve delle disposizioni (`disposizioni.toml:231-245`, 7 e 9 €/h; `disposizioni.toml:212-227`, 1.200 e 2.400 €), lette in `redditi.rs:112-117`.
- **Premessa del ticket che non regge** [V]: il ticket 21 (riga 27) mette fra i derivati «quote dei gruppi fra le fasce». Le fasce del modello sono **relative**: decili 1-3, 4-7, 8-10 della distribuzione nazionale (`strumenti/taratura/elettorato.py:67`, `SOGLIE_FASCE = (0.3, 0.7)`). Con tutti i redditi scalati dello stesso fattore nessuno cambia fascia: non c'è una soglia in euro fra le fasce che possa restare ferma.
- **Anche il gettito non si muove** [V]: l'IRPEF è `irpef_pil` × PIL potenziale × `fattore_irpef` (`finanza.rs:198-199`), con `fattore_irpef = t/t0` (`misure.rs:199-200`) e `t0` ricalcolato sui redditi del turno zero (`elettorato/mod.rs:406`); le imposte dei gruppi sono aliquota di fascia × lordo (`redditi.rs:119, 146`, `elettorato.toml:8`). Al turno zero entrambi i rapporti valgono 1 qualunque sia il livello dei redditi, quindi gli scaglioni fermi in euro non mordono.
- **Serve** [INF]: (a) scalare per k = 1 + x/100 anche `spesa_pensioni`, `spesa_sussidi`, `altre_prestazioni` e i `primario`/`trasferimenti` dei 360 gruppi, lasciando ferme le soglie elencate sopra; (b) perché le soglie mordano, ricalcolare al caricamento `irpef_pil` × t(k)/t(1), con t l'aliquota media IRPEF sugli scaglioni fermi e sui redditi scalati (la funzione c'è già: `Elettorato::aliquota_media_irpef`, `elettorato/mod.rs:412-421`), e allo stesso modo le aliquote di fascia di `elettorato.toml:8`. Senza (b) «più ricca» cambia solo le cifre, proprio quello che il ticket voleva evitare (riga 35). `fondo_riserva` (`manopole.toml:758`, miliardi) va scalato o no: scelta del piano.

### Disoccupazione (punti)

- **Muove**: `disoccupazione` (`scenario.toml:94-98`, 5,6, intervallo 3-15) [V].
- **Letto da**: `economia/mod.rs:112-116` calcola lo spostamento fra il dato nazionale e la media pesata di `disoccupazione_2025` di `regioni.toml`; `economia/mod.rs:139-143` lo applica a **ogni regione** e mette `disoccupazione_strutturale = u`. **u\* è `EconomiaRegione.disoccupazione_strutturale`** (`economia/mod.rs:39`), aggiornata dall'isteresi in `lavoro.rs:57-61`. Il ticket chiede esattamente questo: oggi è già cablato [V].
- **Misura base** [M, `python3 -I` su `regioni.toml`]: media pesata 2025 6,148, spostamento −0,548; Trentino-Alto Adige parte da 1,45, Campania da 13,35. Il pavimento regionale è 0,5 (`lavoro.rs:8`).
- **Non si ricalcola e dovrebbe** [V+M]: (a) la spesa NASpI: `sussidio_per_disoccupato = spesa_sussidi / disoccupati` (`previdenza.rs:41`), quindi con più disoccupati la spesa resta 17,93 mld e scende il sussidio a testa: 12.493 € alla base, 6.031 € con disoccupazione 11,6%, 14.885 € con 4,7% [M]. Va invertito: sussidio a testa fermo, spesa = sussidio × disoccupati. (b) Le quote dei gruppi per condizione: al turno zero vengono da `elettorato.toml` senza correzioni (`elettorato/mod.rs:390-392`), e `Elettorato::lavoro` (`composizione.rs:76-100`) sposta quota fra occupati e disoccupati solo sullo scarto da `ultima_disoccupazione`, che parte già spostata (`elettorato/mod.rs:385`). Il cursore non muove nessun gruppo. Rimedio: applicare una volta, al caricamento, lo stesso spostamento di `composizione.rs:86-97` con Δ = cursore − base. (c) Il gettito: le entrate dipendono dal gap, non dalla disoccupazione (`finanza.rs:193-195`), quindi il disavanzo misurato non cambia (2,94% a 4,7, 8,0 e 11,6) [M]. Il «gettito» fra i derivati del ticket non si ricalcola, e nel modello di oggi non deve.

### Inflazione di partenza

- **Muove**: `inflazione` (`scenario.toml:70-74`, 4,2, intervallo −2-12). `inflazione_t3` e `inflazione_t2` (3,4 e 3,0) sono storia per Phillips (`economia/mod.rs:124-133`) [V].
- **Letto da**: `economia/mod.rs:125` (inflazione corrente), `:154` (`area_euro.inflazione_resto` = la stessa inflazione: il cursore muove anche il resto dell'area euro), `:158-160` (tasso reale), `finanza.rs:177` (bilancio 2027 cresce con min(inflazione, tetto)) [V].
- **Non si ricalcola**: il tasso BCE. È un ingresso (`tasso_bce`, `scenario.toml:64-68`) e la regola di Taylor gira solo nel passo (`economia/mod.rs:286`). Misura: tasso BCE 2,50 con inflazione 2,0 e 8,0 [M]. I panieri valgono 1 al turno zero per costruzione (`redditi.rs:10-13`) [V].
- **Serve** [INF]: decidere se il tasso BCE di partenza deriva dalla Taylor al caricamento (coerente con «solo ingressi», ticket 21 riga 29) o resta un ingresso che la BCE corregge dal primo turno. E decidere se il cursore sposta solo il dato di settembre (lo shock) o anche T2 e T3. Poiché `inflazione_resto` segue il cursore, un'inflazione alta di partenza è uno shock **europeo**: la BCE reagirà forte. Il testo del cursore deve dirlo.

### Età della popolazione (età mediana ± anni)

- **Muove**: `piramide` di `economia.toml:46-58`, più `pensionati` (`scenario.toml:160-164`, 16.388.149) e `spesa_pensioni`, che oggi sono ingressi indipendenti dalla piramide [V].
- **Letto da**: la piramide entra solo in due punti: `misure.rs:58-66` (crescita della forza lavoro 15-66 per la crescita potenziale, `misure.rs:159-161`) e `previdenza.rs:64` (la coorte che va in pensione ogni gennaio). Pensionati e spesa del turno zero vengono dallo scenario (`previdenza.rs:32-42`); la quota di pensionati fra gli elettori viene da `elettorato.toml` [V].
- **Misura** [M, `python3 -I` su `economia.toml`]: totale 58.942.828, età mediana 49,34, quota 67+ 22,40%, 15-66 65,97%, 0-17 14,57%.
- **Metodo**: inclinazione esponenziale di popolazione stabile. Nella popolazione stabile di Lotka la struttura per età è c(a) = b·e^(−ra)·L(a), con b = 1/Σ e^(−ra)·L(a) (fonti/princeton-rodriguez-stable-populations.md:361-370) [V]. A mortalità ferma, cambiare il tasso di crescita di Δr moltiplica la struttura per e^(−Δr·a): si applica alla piramide vera c'(a) ∝ c(a)·e^(−k·a), si rinormalizza al totale e si cerca k per bisezione finché la mediana vale base ± N. Applicarlo a una piramide non stabile come quella italiana è un'analogia, non il teorema [INF].
- **Confronto misurato** [M]: per −5 / −2 / +2 / +5 anni l'inclinazione chiede k = +0,0067 / +0,0029 / −0,0031 / −0,0083 per anno e dà 67+ = 18,0 / 20,5 / 24,6 / 28,4%, 15-66 = 67,1 / 66,6 / 65,1 / 63,3%. La traslazione delle coorti di N anni (alternativa ingenua) non centra la mediana (45,50 invece di 44,34 a −5; 53,21 invece di 54,34 a +5), sposta il picco del baby boom e quindi il calendario dei pensionamenti (`previdenza.rs:64`), e ai bordi deve inventare coorti a 0 anni o ammucchiarle a 100+ (66.712 persone a +5 contro 24.710). L'inclinazione conserva il totale per costruzione, non ha problemi di bordo e lascia la forma del baby boom dove sta.
- **Serve** [INF]: derivare `pensionati` e `spesa_pensioni` dal rapporto fra la quota sopra l'età di pensione della piramide nuova e quella della base (importo medio fermo), e spostare quota verso i gruppi `pensionato` di ogni regione con la stessa logica di `composizione.rs:149-190`. Gli elettori per regione (`elettorato.toml`) restano fermi: lo spostamento della quota 18+ rispetto al totale è un effetto del secondo ordine.

### Orientamento dell'elettorato (± fino a 1 punto)

- **Muove**: `posizione: [f64; 7]` di ciascuno dei 360 gruppi (`elettorato/mod.rs:139`, `elettorato.toml`) [V].
- **Letto da**: `Elettorato::iniziale` copia la posizione anche nell'`ancora` (`elettorato/mod.rs:392-393`), quindi spostare il dato sposta tutte e due e il ritorno all'ancora (`voto.rs:163-176`) non lo annulla. Il voto è logit per punto dell'istogramma (`voto.rs:53-113`); `avvia_voto` mette il voto precedente sulla stazionaria (`voto.rs:179-188`): il sondaggio di partenza si ricalcola da sé [V].
- **Metodo** [INF]: interpolazione lineare della massa. Per x ≥ 0 ogni punto k tiene (1−x) della sua massa e passa x al punto k+1 (simmetrico per x < 0); la massa che uscirebbe oltre 7 (o sotto 1) resta al bordo. La somma resta 1 e `valida_gruppi` (`elettorato/mod.rs:248-253`) passa.
- **Misura** [M, `python3 -I` su `elettorato.toml`, istogramma nazionale pesato per elettori × quota]: base [0,047 0,194 0,124 0,239 0,116 0,226 0,054], media 4,075. Spostamento effettivo della media per x = −1 / −0,5 / −0,25 / +0,25 / +0,5 / +1: −0,953 / −0,476 / −0,238 / +0,237 / +0,473 / +0,946. Ai bordi la massa si accumula: con x = +1 il punto 7 passa dal 5,4% al 27,9%, con x = −1 il punto 1 dal 4,7% al 24,2%. La perdita di spostamento è piccola (≤ 5,4%), ma con ±1 i due partiti estremi ricevono un bacino molto più largo: da guardare al bilanciamento.

### Menu legge elettorale

- **Muove**: `legge_elettorale` (`scenario.toml:8`, nome di un'impostazione di `elezioni.toml:286-315`: `legge_180`, `proporzionale_3`, `proporzionale_puro`) e i gradini delle nove disposizioni elettorali [V].
- **Letto da**: `dati.rs:391-410` e `dati.rs:541-546` costruiscono le nove disposizioni elettorali con `partenza` = impostazione dello scenario e `precedente` = `legge_elettorale_precedente` (`scenario.toml:13-21`, Rosatellum); `disposizioni/mod.rs:266-295` [V].
- **Manca**: (a) la legge è una costante, `LEGGE_ELETTORALE = "L. 180/2026"` (`disposizioni/mod.rs:46`, usata a riga 281): se l'editor parte dal proporzionale al 3%, l'etichetta resta «L. 180/2026», che è falso. (b) La regola della spec (riga 281: il precedente diventa il valore di base) vuole `legge_elettorale_precedente` = valori di `legge_180` quando l'editor cambia la legge; oggi il precedente è il Rosatellum fisso. (c) I gradini singoli (non un'impostazione pronta) non hanno un posto nello scenario: `legge_elettorale` è un nome. Le disposizioni elettorali hanno saldo 0, nessuna leva e nessun effetto (`disposizioni/mod.rs:286-294`), quindi per loro riscrivere la partenza nei dati è sicuro [V].

### Menu disposizioni

Vedi §3: qui c'è la trappola più grossa di tutto l'editor.

## 2. Intervalli di taratura

Gli intervalli di `scenario.toml` sono dichiarati provvisori (`scenario.toml:1`) e sono larghi: limiti di plausibilità, non la validità del modello [V]. Proposta per i cursori, con l'assunzione dichiarata:

| Cursore | Intervallo proposto | Da dove viene |
|---|---|---|
| Debito | 110-160% del PIL | Sotto 135,5% lo spread resta sul minimo di 60 pb, oltre 160% supera i 585 pb e verso 177% tocca il tetto di 1000 [M, §1]. Il limite alto lascia accendere l'avviso di crisi senza uscire dalla curva [INF] |
| Ricchezza | ±20% sul PIL pro capite | Resta dentro `pil_trimestrale` 400-800 (`scenario.toml:42-43`) e dentro gli intervalli delle spese in euro scalate (`spesa_pensioni` 351,8 × 1,2 = 422 < 500, `scenario.toml:168-169`) [V]; ±20% è una scelta, non una taratura [INF] |
| Disoccupazione | da −0,9 a +6 punti (nazionale 4,7-11,6%) | Il basso viene dal pavimento regionale 0,5 (`lavoro.rs:8`): il Trentino-Alto Adige parte da 1,45 e oltre −0,95 il divario non resterebbe invariato [M]. L'alto porta la Campania a 19,4%: una scelta [INF]; il picco storico nazionale non è nelle fonti di `docs/ricerca` [NON TROVATO] |
| Inflazione di partenza | 0-8% | Dentro `inflazione` −2-12 (`scenario.toml:72-73`); Phillips e Taylor sono ricalibrate su Italia e area euro (spec, motore economico) e non sono verificate su inflazioni a due cifre [INF] |
| Età mediana | da −5 a +3 anni (44,3-52,3) | Fuori da ±5 l'inclinazione chiede |k| > 0,008 l'anno e la quota 67+ esce dal 18-28% [M]; il limite alto è più stretto perché l'età di pensione e la copertura sono tarate sul 2026 [INF] |
| Orientamento | ±1 punto (ticket) | Spostamento effettivo ≥ 0,946 a ±1 [M] |

Gli intervalli dei tre cursori sintetici non hanno un ingresso in cui stare: servono campi nuovi nei dati di base (§4).

## 3. Disposizioni cambiate dall'editor

**Come sono rappresentate oggi** [V]:
- Nei dati, ogni disposizione ha `partenza`, `legge`, `precedente`, `in_arrivo` (`disposizioni/mod.rs:113-118`; per esempio `disposizioni.toml:30-32`, cuneo con partenza «7», legge «L. 207/2024, art. 1 c. 4-9», precedente «0»).
- Nella partita, `StatoDisposizione { in_vigore, precedente, legge, in_arrivo, bloccati }` (`stato.rs:22-28`), costruita da `Disposizioni::iniziali` (`stato.rs:64-81`).
- **`partenza` ha due ruoli**: è il gradino in vigore al turno zero **e** il riferimento da cui il motore misura ogni effetto. Il saldo è Σ(saldo in vigore − saldo di partenza) (`effetti.rs:74-80`); le leve si leggono come scarto da `Leve::partenza` (`effetti.rs:62-64`) in `misure.rs:106` (decontribuzione, autonomia, accise) e in `redditi.rs:79` (cuneo, salario minimo, reddito minimo, forfettario); gli effetti sulle grandezze nascono solo da un cambio di gradino (`effetti.rs:109-159`).

**Conseguenza** [V+INF]: se l'editor riscrive `partenza` nei dati, la disposizione cambiata non fa nulla. Saldo, leve ed effetti valgono zero perché il riferimento si è spostato con lei: «cuneo 0» nell'editor lascerebbe redditi e disavanzo uguali a «cuneo 7». Il ticket chiede invece «effetti sugli indicatori» (riga 33).

**Cosa serve** [INF]: tenere `partenza` come riferimento di taratura (il valore dello scenario di base) e applicare il valore dell'editor come una legge al turno zero. Esiste già: `Disposizioni::cambia(dati, d, g, 0, legge, &mut grandezze.effetti)` (`stato.rs:112-123`) mette `precedente = in_vigore`, cioè il valore di base (`stato.rs:103`), che è proprio la regola della spec (riga 281). Scrive la legge, cancella gli arrivi (`stato.rs:122`) ed emette gli effetti col loro ritardo. Restano da fare:
1. un posto nello scenario per i valori dell'editor (per esempio `disposizioni = { cuneo_fiscale = "0" }`) che `costruisci` applica dopo `Disposizioni::iniziali`;
2. `previdenza.rs:37` prende il regime della prima annata da `Leve::partenza`: con un regime pensionistico cambiato dall'editor va letto dalle leve in vigore;
3. gli effetti di livello partono a turno 0 + ritardo (`effetti.rs:146`): una legge «del 2026» li vedrebbe arrivare durante la partita, non già in vigore. Il piano sceglie fra ritardo 0 al turno zero e ritardo pieno;
4. l'etichetta: `etichetta_legge` scrive «L. {id}/{anno}» per le leggi del giocatore (`partita.rs:118-120`) e l'eredità le confronta con le leggi in vigore. Un'etichetta «L. n/2026» dell'editor potrebbe collidere con una legge del giocatore approvata nel 2026 (il turno zero è il quarto trimestre 2026, `partita.rs:50-53`). Serve un'etichetta che non sia mai di quella forma;
5. le disposizioni costituzionali (per esempio `numero_parlamentari`, letto dalla validazione del parlamento uscente in `parlamento/mod.rs:139-145`): stanno o no nel menu.

## 4. File su misura come differenze dalla base

**Caricamento di oggi** [V]: `FileDati::leggi` legge dieci testi per nome fisso (`dati.rs:31-61`); `DatiGioco::da_testi` li parsa (`dati.rs:507-533`), rifiuta disposizioni elettorali nel file (`:534-539`), aggiunge le nove elettorali (`:540-546`) e chiama `valida` (`:560`), che controlla gli ingressi dello scenario (`:707`, `scenario.rs:147-158` con `Errore::ScenarioFuoriIntervallo { nome, valore, min, max }`). La partita salva `dati` per intero (`partita.rs:88`): il salvataggio è già uno snapshot, quindi un aggiornamento dei dati base non tocca le partite in corso, come vuole il ticket (riga 41).

**Dove inserire le differenze** [INF]: fra il parse e la validazione, cioè fra `dati.rs:533` e `dati.rs:540`, con una funzione nuova (per esempio `DatiGioco::da_testi_con(file, differenze)`) che parsa il file di differenze, controlla i valori **contro gli intervalli del file base** (mai contro intervalli scritti nel file di differenze, sennò il limite si aggira), applica le trasformazioni dei cursori e poi le sostituzioni puntuali, e lascia a `valida` il controllo finale degli ingressi toccati. L'ordine fisso conta per i cursori che toccano le stesse grandezze: ricchezza ed età scalano entrambe la spesa per pensioni (sono prodotti, quindi commutano; la sostituzione puntuale viene dopo e vince).

**Formato proposto** [INF]:

```toml
# Scenario su misura: solo differenze dallo scenario di base.
nome = "Italia indebitata"

[cursori]
debito_pil = 150.0        # % del PIL, valore assoluto (è l'ingresso)
disoccupazione = 8.0      # % nazionale, valore assoluto (è l'ingresso)
inflazione = 6.0          # % annua, valore assoluto (è l'ingresso)
ricchezza = 10.0          # ± % sul PIL pro capite di base
eta_mediana = 2.0         # ± anni sull'età mediana di base
orientamento = -0.5       # ± punti sull'asse 1-7
legge_elettorale = "proporzionale_3"

[disposizioni]            # id della disposizione = id del gradino
cuneo_fiscale = "0"

[sostituzioni]            # nome dell'ingresso come in Scenario::ingressi
costo_medio_debito = 3.2
"valence di destra" = 0.5
```

**Validazione** [INF]: `#[serde(deny_unknown_fields)]` sulle tabelle, così un nome sbagliato è rifiutato col suo nome invece di essere ignorato; le sostituzioni accettano solo i nomi di `Scenario::ingressi` (`scenario.rs:90-94`, che già nomina «valence di {id}»); fuori intervallo, lo stesso `ScenarioFuoriIntervallo` di oggi. I tre cursori sintetici hanno bisogno di un intervallo dichiarato nei dati di base, per esempio una tabella `[cursore.ricchezza] min/max/fonte` in `scenario.toml`, con lo stesso tipo `Ingresso` e valore 0. Ogni gradino delle `[disposizioni]` va controllato contro il catalogo (`DatoDisposizione::gradino`, `disposizioni/mod.rs:124-126`).

## 5. Riepilogo dal vivo: cosa serve e quanto costa

**Cosa serve, con cosa c'è** [V]:
- *Consenso atteso per posizione, senza rumore*: `Intenzioni::calcola` (`voto.rs:19-41`) dà le intenzioni vere; dopo `avvia_voto` le intenzioni del turno zero coincidono col voto precedente (`voto.rs:179-180`), quindi basta pesare `voto_precedente` con gli elettori. Non serve `Informazione::nuova` (`informazione/mod.rs:32-71`), che aggiunge proprio il rumore da togliere.
- *Maggioranza attesa*: `elezioni::riparto` (`riparto.rs:665`) sui voti per regione (`legislatura/voto.rs:46-66`) con la coalizione del giocatore; la disponibilità dei partiti IA è `Partita::disponibilita` (`partita.rs:2077`).
- *Spread, interessi, disavanzo*: tutti in `Economia::iniziale` (`economia/mod.rs:109-196`), che non dipende dalla posizione del giocatore: si calcola una volta per stato dell'editor.
- *Avviso 42%*: dalle quote attese delle coalizioni possibili, per ognuna delle sei posizioni (`fondazione.rs:45-52`).
- *Avviso «situazione o crisi accesa»* [V]: le situazioni del ticket 15 non esistono nel codice: `grep -rn -i situazion crates/sim-core/src` trova solo `VistaStatistica`, e nessuna soglia di aste deserte o di crisi del debito. Oggi si possono segnalare solo indicatori: stato della PDE (`scenario.toml:4`), disavanzo oltre il 3% (`spread.rs:31`), spread vicino al tetto. L'avviso vero aspetta il piano delle situazioni.

**Misura** [M]. Da `/Users/mario/GitHub/simulatore-politico/.claude/worktrees/docs-v1-piano-10`, branch `docs/v1-piano-10`, HEAD `95f18cb`. Crate di misura esterno al repo, con dipendenza a percorso su `crates/sim-core`, nello scratchpad della sessione; il lockfile è risolto da crates.io, quindi le versioni dei crate non fissati con `=` possono differire da `Cargo.lock` del repo. Comando: `CARGO_TARGET_DIR=<scratchpad>/target cargo run -q [--release] --bin misura --manifest-path <scratchpad>/misura/Cargo.toml -- <worktree>/dati/base N`. Macchina Apple A18 Pro, 6 core, rustc 1.98.0 (Homebrew). Mediane, dati base, seme 1:

| Passo | release (N=30) | debug (N=5) |
|---|---|---|
| `DatiGioco::da_testi` (parse + valida) | 1,53 ms | 28,3 ms |
| `DatiGioco::clone` | 0,05 ms | 0,21 ms |
| `Economia::iniziale` | 0,004 ms | 0,05 ms |
| `Elettorato::iniziale` | 0,02 ms | 0,37 ms |
| `Elettorato::avvia_voto` | 2,27 ms | 41,9 ms |
| `Intenzioni::calcola` | 1,57 ms | 17,1 ms |
| `elezioni::riparto` | 0,025 ms | 0,23 ms |
| `Partita::nuova` (= `costruisci` senza giocatore) | 4,02 ms | 62,9 ms |
| `Partita::fonda` a 3,5 | 4,51 ms | 70,3 ms |
| `Partita::anteprima_fondazione` (6 posizioni) | 29,0 ms (N=10) | 445 ms |

Il primo giro release, con load average 8-10 per gli altri due ricercatori che compilavano in parallelo, ha dato 65,3 ms per `anteprima_fondazione` e 10,3 ms per `Partita::nuova`. Il secondo e il terzo, con load ~2,5, i valori in tabella. Le cifre valgono come ordine di grandezza, non come benchmark.

**Lettura** [M+INF]: in release un cambio di cursore che rifà tutto (`da_testi` + `anteprima_fondazione`) costa ~31 ms, sotto la soglia di 100 ms del brief. In debug costa ~470 ms, oltre la soglia; succede con `cargo tauri dev` se il core non è ottimizzato, e nessun `[profile]` lo ottimizza oggi (`Cargo.toml` della radice e di `app/src-tauri` letti: nessuna sezione profile) [V]. Dentro `costruisci`, `avvia_voto` + `Intenzioni::calcola` pesano ~3,8 ms su 4,0, cioè il 95% [M].

**Cosa costruire di meno** [INF], per il riepilogo:
1. niente `Informazione::nuova`, niente `Ministri::iniziali`, niente `Grandezze`: servono alla partita, non al riepilogo;
2. `Economia::iniziale` una volta per stato, non una per posizione;
3. per posizione solo `Elettorato::iniziale` + `avvia_voto` + somma di `voto_precedente` (niente `Intenzioni::calcola`): ~2,3 ms × 6 ≈ 14 ms in release, ~255 ms in debug;
4. parse dei dati base una volta all'apertura dell'editor, poi differenze applicate su un `clone` (0,05 ms) invece di rifare `da_testi` (1,5 ms);
5. se il debug resta lento: `[profile.dev.package.sim-core] opt-level = 2` nel `Cargo.toml` della radice (scelta di strumento, non di prodotto).

## 6. Confronto esterno: cosa mostrano i giochi mentre si muove un cursore

- **Civilization VII**: l'aggiornamento 1.2.2 ha aggiunto opzioni avanzate di partenza (percorsi d'eredità on/off, scelta delle crisi, difficoltà dell'IA per sistema: economia, combattimento, espansione) (fonti/civ7-update-check-in-2025-06-10.md:39-48) [V]. Con la patch 1.2.4.1 le impostazioni avanzate della difficoltà sono sempre visibili e la difficoltà passa a «Custom» appena si cambia un'impostazione singola; alcune impostazioni sono «sticky» fra una partita e l'altra (fonti/civ7-patch-1-2-4-1-2025-08-27.md:21-23, 33-37) [V]. Nessuna anteprima dell'esito.
- **Frostpunk 2**: quattro preset (Citizen, Officer, Steward, Captain) e quattro cursori separati (economia, meteo, Frostland, società), che non si cambiano più a partita iniziata (fonti/frostpunk2-gamepressure-difficolta.md:18-38, 44). Guida di terzi, non fonte di 11 bit studios [V, secondaria]. Nessuna anteprima dell'esito.
- **Terra Invicta**: la patch 11 (agosto 2023) ha aggiunto composizione iniziale del consiglio, opzioni di difficoltà e cursori di durata (velocità della ricerca, resa minerarie, progressione aliena) (fonti/terrainvicta-gamespress-patch11-2023.md) [V, comunicato dell'editore letto tramite WebFetch]. Il comunicato non dice di un riepilogo dal vivo [NON TROVATO].
- **Democracy (Positech)**: c'è un cursore di difficoltà alla partenza, così poco visibile che il designer lo cita fra le cause del «troppo facile» (fonti/cliffsblog-2013-too-easy.md:7; post del 2013, era Democracy 3) [V]; il paese di partenza è un file di dati (vedi [[democracy-4]], righe 29-32).
- **Victoria 3**: le «game rules» non sono state catturate (wiki Paradox irraggiungibile da defuddle) [NON TROVATO].

Schema comune [INF]: preset con nome più cursori per asse, e il preset diventa «personalizzato» al primo ritocco; nessuno dei quattro mostra un'anteprima numerica degli effetti. Il riepilogo dal vivo con spread, disavanzo e consenso atteso per posizione non ha un precedente trovato: costa poco (§5) ed è il nostro tratto distintivo.

## Riferimenti

- **URL** https://grodri.github.io/demography/stablepop · [V]
  **perche' conta qui** dà la struttura per età della popolazione stabile, c(a) = b·e^(−ra)·L(a) (righe 361-370)
  **cosa se ne prende** il metodo dell'inclinazione e^(−k·a) per il cursore dell'età, con k cercato per bisezione sulla mediana
- **URL** https://civilization.2k.com/civ-vii/news/civ-vii-update-check-in-jun-10/ · [V]
  **perche' conta qui** è un'impostazione avanzata di partenza con assi separati, da fonte primaria (righe 39-48)
  **cosa se ne prende** cursori per sistema dietro un preset, non valori uno per uno
- **URL** https://civilization.2k.com/civ-vii/game-update-notes/2025-aug-27-patch-1-2-4-1/ · [V]
  **perche' conta qui** il preset passa a «Custom» al primo ritocco, e le opzioni avanzate nascoste erano un problema (righe 21-23, 37)
  **cosa se ne prende** «Italia di oggi» che diventa «su misura» al primo cursore mosso, con i cursori sempre visibili
- **URL** https://www.gamepressure.com/frostpunk-2/difficulty-levels/zf115ec · [V, secondaria]
  **perche' conta qui** preset più quattro cursori, bloccati a partita iniziata (righe 18-38, 44)
  **cosa se ne prende** conferma dello schema preset + assi; lo scenario si fissa alla fondazione, come da ticket
- **URL** https://gamespress.com/Terra-Invicta-August-update-and-sale · [V, via WebFetch]
  **perche' conta qui** è un gioco di strategia geopolitica con cursori di partenza sul ritmo e sul consiglio iniziale
  **cosa se ne prende** nessun riepilogo documentato: il nostro sarebbe una differenza
- **URL** https://www.positech.co.uk/cliffsblog/?p=2599 · [V]
  **perche' conta qui** il cursore di difficoltà di Democracy che i giocatori non vedono (riga 7)
  **cosa se ne prende** i cursori dell'editor stanno in vista, non dietro un «avanzate»
- **URL** `crates/sim-core/src/disposizioni/effetti.rs:62-80` · [V]
  **perche' conta qui** saldo e leve sono scarti da `partenza`
  **cosa se ne prende** l'editor non può riscrivere `partenza`: applica il valore con `Disposizioni::cambia` al turno zero
- **URL** `crates/sim-core/src/economia/mod.rs:112-143` · [V]
  **perche' conta qui** la disoccupazione nazionale sposta già ogni regione e u*
  **cosa se ne prende** il cursore della disoccupazione è l'ingresso; restano da fare NASpI e quote dei gruppi
- **URL** `crates/sim-core/src/economia/previdenza.rs:41` · [V]
  **perche' conta qui** il sussidio a testa deriva dalla spesa fissa
  **cosa se ne prende** invertire: sussidio a testa fermo e spesa derivata, sennò il cursore abbassa il sussidio
- **URL** `strumenti/taratura/elettorato.py:67` · [V]
  **perche' conta qui** le fasce sono decili relativi
  **cosa se ne prende** «quote fra le fasce» esce dai derivati della ricchezza; le soglie in euro mordono solo su IRPEF, pensione minima, salario e reddito minimo
- **URL** `crates/sim-core/src/legislatura/fondazione.rs:136-146` · [V]
  **perche' conta qui** l'anteprima costruisce sei partite e legge il sondaggio rumoroso
  **cosa se ne prende** il riepilogo è una funzione nuova e più leggera, senza `Informazione`

## Per il nostro gioco

Raccomandazioni (sono raccomandazioni, non decisioni):

1. **Tre cursori sono ingressi, tre sono trasformazioni.** Debito, disoccupazione e inflazione scrivono `debito_pil`, `disoccupazione`, `inflazione`, coi loro intervalli. Ricchezza, età e orientamento sono funzioni di trasformazione con un intervallo nuovo nei dati di base.
2. **Disposizioni: `partenza` resta il riferimento di taratura**, e il valore dell'editor passa per `Disposizioni::cambia` al turno zero. Altrimenti il menu non ha effetti (§3).
3. **Ricchezza**: scalare anche le spese in euro e i redditi dei gruppi, ricalcolare `irpef_pil` e le aliquote di fascia con gli scaglioni fermi; togliere «quote fra le fasce» dai derivati del ticket.
4. **Disoccupazione**: sussidio a testa fermo, spesa derivata; spostamento delle quote occupati → disoccupati applicato una volta al caricamento.
5. **Età**: inclinazione esponenziale della piramide, con pensionati e spesa pensionistica derivati dalla piramide.
6. **Riepilogo**: funzione nuova, un'economia per stato e per posizione solo elettorato + `avvia_voto` (~14 ms release); i dati base si parsano una volta sola.
7. **File di differenze** controllato contro gli intervalli della base, `deny_unknown_fields`, applicato fra parse e validazione.

Scelte del piano (di strumento, le decide il piano dichiarando l'assunzione):
- tasso BCE di partenza da Taylor o ingresso; il cursore dell'inflazione sposta solo settembre o anche T2-T3;
- ritardo degli effetti delle disposizioni dell'editor (0 o pieno);
- etichetta della legge del governo uscente, che non collida con «L. n/anno»;
- `fondo_riserva` scalato o no con la ricchezza;
- `opt-level` del core in debug.

Da chiedere a Mario (cambiano il prodotto):
- se le disposizioni costituzionali (numero dei parlamentari) stanno nel menu dell'editor;
- se nel menu della legge elettorale si scelgono i gradini uno per uno o solo le tre impostazioni pronte;
- l'avviso «crisi accesa» finché le situazioni non esistono: rinviarlo al piano delle situazioni o mostrare intanto indicatori grezzi (PDE, disavanzo oltre il 3%, spread oltre una soglia).

## Collegamenti

- [[stato-del-paese-al-turno-zero]]: i livelli di partenza e la piramide ISTAT.
- [[decisioni]]: la voce del 18/10 sull'editor dello scenario.
- [[disposizioni-al-turno-zero]]: valori di partenza, leggi d'origine e precedenti.
- [[economia-italiana-parametri]]: curva dello spread e coefficienti dietro le misure di §1.
- [[democracy-4]]: il paese di partenza come file di dati.
