---
tipo: modello
fonti: [fonti/ilpost-2022-costi-sondaggi-politici.md, fonti/intotheminds-prezzo-sondaggi-opinione.md, fonti/arera-2023-gara-cati-offerta-economica.md, fonti/fatto-2026-sondaggio-chigi-tecne.md, fonti/gonews-2020-open-sondaggi-renzi.md, fonti/fivethirtyeight-2014-partisan-senate-polls.md, fonti/coppock-leeper-mullinix-2018-pnas.md, fonti/magnacarta-bilancio-2024.md, fonti/openpolis-mappatura-think-tank.md, fonti/upb-rendiconto-2019.md, fonti/upb-focus-1-2022-accuratezza-previsioni.md, fonti/ec-dp027-2016-accuratezza-previsioni.md, fonti/heinisch-2024-iwh-previsioni-trimestrali.md, fonti/ocpi-2025-revisioni-pil.md, fonti/istat-2017-scheda-revisioni-pil.md]
aggiornato: 2026-10-29
---
# Informazione a pagamento

Prezzi ed errori reali dei tre acquisti d'informazione che la cassa del partito può fare (spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md:372`): sondaggio privato, test di una misura, centro studi. La pagina non rifà gli errori già in gioco (manopole `errore_stima_*`, `sigma_istituto`, `sigma_bias_comune`; modulo `crates/sim-core/src/informazione/mod.rs`): dice quanto costa ridurli nel mondo reale e di quanto si riducono. I prezzi sono in euro, sulla scala della cassa del piano 8b ([[finanziamento-dei-partiti]], branch `feat/v1-piano-8b-campagna`, righe 82-95).

## Scala della cassa, per confronto [M]

Dalle manopole del piano 8b (`feat/v1-piano-8b-campagna:dati/base/manopole.toml:1254-1280`): 2 per mille 8.146.050 € a turno e donazioni 5.500.000 € a turno, in tutto il paese; `quota_libera_entrate` = 0,1. Un partito che prende la quota s delle entrate ha libera ogni turno 0,1 × s × 13,65 milioni [M, `python3` in sessione]:

| quota del partito | entrate a turno | parte libera a turno |
|---|---|---|
| 5% | 682.000 € | 68.000 € |
| 10% | 1.365.000 € | 136.000 € |
| 20% | 2.729.000 € | 273.000 € |
| 30% | 4.094.000 € | 409.000 € |

## 1. Sondaggio privato

### Prezzo
- CATI/CAMI (telefono, fisso più cellulare): costa all'istituto circa 5-6 € a intervista; al cliente si fattura circa il doppio. Per 1.000 interviste, quindi, il prezzo è di circa 10.000-12.000 € (Paolo Natale, Statale di Milano, in fonti/ilpost-2022-costi-sondaggi-politici.md:21) [V giornalistico].
- CAWI (online, da un panel di circa 100.000 persone: si estraggono 2.000-3.000 nominativi per avere circa 1.000 risposte): il costo «arriva a malapena a 1.000-2.000 euro per sondaggio» (fonti/ilpost-2022-costi-sondaggi-politici.md:23-25) [V giornalistico]. Attenzione: è il costo per l'istituto, non il prezzo al cliente.
- Interviste di persona: 20-25 € a intervista (fonti/ilpost-2022-costi-sondaggi-politici.md:15) [V giornalistico].
- Un listino di un istituto di ricerche di mercato (non politico) indica CAWI da 5.000 € con media 7.500 € (8.500 € con l'affitto di un panel), CATI da 5.000 € con media 12.000 €, e 20-100 € per intervista CATI riuscita secondo il bersaglio (fonti/intotheminds-prezzo-sondaggi-opinione.md:53-71) [V, fonte commerciale].
- Gara pubblica ARERA del 2023: base d'asta di 4.000 € IVA esclusa per un lotto aggiuntivo di 300 interviste CATI (fonti/arera-2023-gara-cati-offerta-economica.md:47) [V]. Vuol dire 13,3 € per intervista in più su un'indagine già impostata [M]: è il prezzo marginale di un sovracampionamento.
- Committenza pubblica: tre rilevazioni affidate da Palazzo Chigi a Tecnè in due mesi, per 120.000 €, sul referendum sulla giustizia del 2026 (titolo e occhiello in fonti/fatto-2026-sondaggio-chigi-tecne.md:3-5) [V giornalistico; il corpo è a pagamento, quindi numero di interviste e metodo restano ignoti]. Fanno circa 40.000 € a rilevazione [M].
- Spesa di un'area politica: secondo le informative della Guardia di Finanza, la fondazione Open avrebbe pagato oltre 130.000 € di sondaggi per le campagne di Renzi del 2012-2013 (fonti/gonews-2020-open-sondaggi-renzi.md:5) [V giornalistico; è un'accusa, non un fatto accertato].
- [NON TROVATO] Una voce «sondaggi» nei rendiconti dei partiti: la legge non chiede il dettaglio delle spese, e i sondaggi finiscono fra le consulenze o i servizi. Il dato «il PD spende circa 500.000 € l'anno» (attribuito a Report) è emerso in ricerca senza una fonte primaria rintracciabile, e non va usato.
- [NON TROVATO] Prezzi su ANAC o MEPA per un sondaggio politico nazionale da 1.000 interviste. Le gare trovate (ISTAT, Consip, università) riguardano indagini statistiche molto più grandi.

### Sovracampionamento per regione o condizione [M, INF]
- Errore di campionamento (casuale semplice, p = 0,5), su una quota: con n = 800 l'IC95 è ±3,5 punti; con 1.000 ±3,1; con 1.500 ±2,5; con 2.000 ±2,2; con 3.000 ±1,8; con 5.000 ±1,4 [M, `python3`: 1,96·√(0,25/n)]. L'errore reale vale circa il doppio ([[sondaggi-ed-errore]]).
- Una regione con 500 interviste ha un IC95 di ±4,4 punti [M]. Venti regioni da 500 fanno 10.000 interviste: circa 15.000-75.000 € in CAWI e 100.000-130.000 € in CATI, ai prezzi per intervista indicati sopra [INF].

### Errore d'istituto in un sondaggio commissionato
- [NON TROVATO] Nessuno studio misura se un sondaggio commissionato per uso interno abbia un effetto istituto (house effect) minore di quelli pubblicati. Questi sondaggi per definizione non escono, e non c'è un risultato contro cui confrontarli.
- L'evidenza disponibile va semmai nella direzione opposta per i sondaggi di parte **resi pubblici**. Sui 147 sondaggi di parte per il Senato USA pubblicati dal 2002, la distorsione media è di 4,25 punti a favore del committente; sale a 6,64 punti quando il candidato del committente perde, e scende a 0,98 quando vince (fonti/fivethirtyeight-2014-partisan-senate-polls.md:7-17) [V]. Per la Camera FiveThirtyEight riporta circa 6 punti (stessa fonte, :5) [V]. La causa più citata è la selezione di ciò che si pubblica (stessa fonte) [V].
- [INF] Il vantaggio documentabile di un sondaggio privato sta quindi in tre cose: un campione più grande, le domande e gli incroci scelti dal committente, e l'assenza di selezione su cosa pubblicare. Una riduzione dell'effetto istituto non è documentata. Se il gioco la toglie, è una scelta di design, non un dato.

## 2. Test di una misura

### Cosa corrisponde nel mondo reale [INF]
- **Esperimento in un sondaggio** (survey experiment, split-sample): metà campione legge la misura in una formulazione, metà in un'altra (o non la legge), e si confronta la reazione per gruppo. È il «message testing» delle campagne.
- **Conjoint**: profili di misure con attributi variati a caso. Serve per scomporre gli effetti, ed è più costoso in potenza statistica (vedi sotto).
- **Focus group**: qualitativo, 6-10 persone. Non dà una stima per gruppo: dà argomenti e reazioni.

### Prezzo
- Un esperimento in un sondaggio costa quanto un CAWI della stessa taglia (vedi §1): da 1.000-2.000 € (costo per l'istituto) a 5.000-8.500 € (listino) per circa 1.000 interviste; la randomizzazione costa poco in più [INF].
- Focus group: 3.000-5.000 € a gruppo, sala, reclutamento, compensi e analisi compresi, secondo lo stesso istituto di ricerche di mercato (pagina non catturata: https://www.intotheminds.com/blog/it/ricerca-di-mercato-quanto-costa/, letta solo nell'estratto della ricerca) [INF, non verificato su pagina salvata].

### Precisione dell'effetto su un sottogruppo [M]
- Errore standard della differenza fra due bracci di n persone ciascuno, su un esito sì/no con p = 0,5: √(2·0,25/n). Con n = 100 l'IC95 è ±13,9 punti; con 150 ±11,3; con 250 ±8,8; con 500 ±6,2; con 1.000 ±4,4 [M, `python3`].
- In un esperimento da 2.000 interviste divise in due bracci, un gruppo che pesa il 10% della popolazione ha 100 persone per braccio, quindi ±13,9 punti; uno che pesa il 25% ne ha 250, quindi ±8,8 punti [M]. La reazione stimata di un gruppo piccolo è quasi rumore.
- L'evidenza empirica dice anche che, nei classici esperimenti di persuasione, l'eterogeneità fra gruppi è bassa. Su 27 esperimenti replicati (101.745 risposte) gli effetti per sottogruppo (CATE) stanno «strettamente raggruppati» attorno all'effetto medio, e le differenze fra gruppi, quando ci sono, sono piccole (fonti/coppock-leeper-mullinix-2018-pnas.md:5, :75, :93) [V]. Le dimensioni dei campioni di quegli studi stanno fra 280 e 3.277 (stessa fonte, tabella 1) [V].
- Conjoint: gli autori di un'analisi di potenza (Stefanelli e Lukac, 2020; Schuessler e Freitag, 2020) avvertono che l'eterogeneità e le interazioni richiedono campioni molto grandi [NON TROVATO in testo integrale: solo abstract letti in ricerca, nessuna pagina catturata].
- [INF] Per il gioco: un test reale dice bene la **direzione** della reazione di un gruppo grande e male l'**entità** per un gruppo piccolo. Se il gioco rende «più precisa la parte gruppi colpiti» (spec :372), l'errore del test dovrebbe scendere con il peso del gruppo, come 1/√(quota).

## 3. Centro studi

### Budget delle strutture vicine ai partiti
- Fondazione Magna Carta (area di centrodestra), bilancio 2024: proventi 325.490 €, oneri 321.654 €; personale e collaboratori 66.233 €, convegni e seminari 42.522 €; contratti con enti pubblici 59.064 € (fonti/magnacarta-bilancio-2024.md:183-286) [V]. Fanno circa 81.000 € a trimestre [M].
- Mappatura Openpolis dei think tank e delle fondazioni politiche (106 attive): solo il 29,25% pubblica il bilancio. Nel 2018 sette avevano entrate oltre 1 milione: in testa la Fondazione Eni Enrico Mattei (oltre 6 milioni) e Aspen Institute Italia (5,7 milioni); Italia decide 766.000 €, di cui 656.000 da soci; l'Associazione Rousseau circa 1 milione, versato soprattutto dagli eletti M5S (fonti/openpolis-mappatura-think-tank.md:74, :92, :112) [V]. La legge chiede il bilancio, ma non il dettaglio delle spese (stessa fonte, :128) [V].
- [NON TROVATO] I bilanci di Italianieuropei e Farefuturo: i siti non hanno una pagina di trasparenza leggibile (Farefuturo risponde 404, Italianieuropei mostra una pagina senza documenti, verificato con `curl` in sessione).

### Previsore indipendente
- Ufficio parlamentare di bilancio, 2019: dotazione di 6.000.000 € l'anno, spese correnti impegnate per 4.813.952 €, 24 persone in servizio su 40 previste (fonti/upb-rendiconto-2019.md:70, :139, :244) [V]. Fanno 1,5 milioni a trimestre: è il costo di un previsore pubblico completo [M].
- [NON TROVATO] Il costo di un singolo incarico (una previsione commissionata a Prometeia, REF, CER o Oxford Economics): nessun listino pubblico trovato.

### Lettura [INF]
Un centro studi «di partito» costa 0,3-1 milioni l'anno, cioè 80.000-250.000 € a trimestre; un previsore di livello istituzionale ne costa circa 1,5 milioni a trimestre. Confrontati con la parte libera della cassa (tabella sopra), il primo vale da metà a tutta la parte libera di un partito medio; il secondo non è alla portata di nessun partito.

## 4. Errore di previsione per l'Italia

### Annuale, Commissione europea, 1969-2014 [V]
Le previsioni dell'anno in corso escono in primavera (maggio, quindi per circa tre trimestri ancora da vedere); quelle dell'anno dopo escono in autunno (novembre, quindi per quattro-sette trimestri). Il risultato con cui si confrontano è il dato di un anno dopo, non quello finale (fonti/ec-dp027-2016-accuratezza-previsioni.md:389-394). Righe dell'Italia, tabelle A1.1-A1.4 (stessa fonte, :3068-3208):

| grandezza | anno in corso MAE / RMSE | anno dopo MAE / RMSE | rapporto RMSE [M] |
|---|---|---|---|
| PIL in volume | 0,85 / 1,15 | 1,35 / 1,84 | 1,6 |
| deflatore dei consumi (inflazione) | 0,60 / 0,86 | 1,32 / 2,28 | 2,7 |
| saldo delle AP (% del PIL) | 0,86 / 1,27 | 1,26 / 1,63 | 1,3 |
| tasso di disoccupazione | 0,69 / 1,06 | 0,98 / 1,47 | 1,4 |

- L'Italia ha una distorsione ottimista sul PIL: errore medio +0,45 sull'anno in corso e +0,83 sull'anno dopo (stessa fonte, :3080) [V]. Nel 2000-2014 è l'unico paese con una distorsione significativa (stessa fonte, :1320) [V].
- La Commissione è accurata quanto OCSE, FMI e Consensus, e più del mercato; per Italia e Finlandia, però, i suoi errori risultano maggiori di quelli OCSE (stessa fonte, abstract e :2506) [V].
- [NON TROVATO] Il debito: le tabelle non lo riportano. [INF] L'errore sul rapporto debito/PIL viene per lo più dal PIL nominale (circa 1,35 × l'errore sulla crescita nominale) più l'errore sul saldo: a un anno sono circa 3 punti di PIL. È una stima grezza, da tarare.

### Trimestrale, Commissione europea, 2000-2019 [V]
- PIL italiano trimestre su trimestre: crescita media 0,075%, deviazione standard 0,686 (fonti/heinisch-2024-iwh-previsioni-trimestrali.md:435) [V]. In tassi annualizzati (×4) fanno 0,3% e 2,7 punti [M].
- Rispetto alla media storica, l'RMSE della previsione della Commissione per l'Italia vale 0,745 nel trimestre corrente (h = 0, significativo), 0,945 a h = 1, e da 1,02 a 1,22 da h = 2 a h = 8 (fonti/heinisch-2024-iwh-previsioni-trimestrali.md:853-861) [V]. **Oltre il trimestre in corso, la Commissione non batte la semplice media storica.** Contro Consensus Economics il rapporto è vicino a 1 a tutti gli orizzonti (stessa fonte, :929) [V].
- L'Italia è il solo paese con una distorsione verso l'alto significativa a tutti gli orizzonti, fra +0,16 e +0,51 punti trimestrali (stessa fonte, :657 e tabella 2) [V].
- [M] Ne segue una forchetta realistica sul PIL trimestrale annualizzato: circa ±2 punti (1σ) sul trimestre in corso, poi circa 2,7-3 punti da due trimestri avanti, cioè l'ampiezza della variabilità della crescita stessa. La forchetta si allarga molto fra h = 0 e h = 2, poi resta quasi piatta.
- [INF] Previsione sulla crescita trimestrale annualizzata o sulla media di quattro trimestri? La scelta cambia la forchetta di un fattore circa 2: le cifre annuali della tabella sopra sono più strette, perché la media annua smorza il rumore dei singoli trimestri.

### Prima stima e trimestre chiuso [V, M]
- ISTAT, 2000-2017: la stima preliminare (45 giorni dopo la fine del trimestre) viene rivista dalla prima stima dei conti trimestrali (70 giorni) in media di 0,05 punti in valore assoluto sulla variazione congiunturale (fonti/istat-2017-scheda-revisioni-pil.md:5-7) [V]. Nel breve, quindi, la revisione è minima.
- Dalla prima pubblicazione al dato corrente del 2024, invece, la revisione assoluta media del PIL italiano trimestre su trimestre è 0,271 punti, con deviazione standard 0,352 (fonti/heinisch-2024-iwh-previsioni-trimestrali.md:435) [V]. Annualizzata vale circa 1,1 punti [M]: conferma il valore della manopola `errore_stima_pil` = 1,0 (`dati/base/manopole.toml:499-502` su main).
- Le revisioni ISTAT sono per lo più al rialzo: dal 2014 il livello finale del PIL in volume supera la prima stima dello 0,4% in media (1,7% in termini nominali); sulla crescita reale annua, +0,2 punti nel 2014-2020 e +1,4 nel 2021-22 (fonti/ocpi-2025-revisioni-pil.md:3, :51) [V].
- [NON TROVATO] Nessuno studio mostra un previsore privato che batta la prima stima ISTAT sul trimestre chiuso. [INF] Una «stima migliore del trimestre chiuso» ha due appigli reali: la distorsione al ribasso documentata, che si può correggere, e le informazioni che arrivano dopo la stima preliminare. Il guadagno plausibile è una parte dell'errore di 0,27 punti trimestrali (circa 1 annualizzato), non la sua eliminazione.
- [NON TROVATO] Revisioni della prima stima di inflazione, disoccupazione e disavanzo. Le manopole `errore_stima_inflazione` (0,2), `errore_stima_disoccupazione` (0,2) ed `errore_stima_disavanzo` (0,3) restano un'assunzione del piano 4.

### Banca d'Italia, UPB, FMI
- UPB, previsioni del MEF: RMSE del PIL dell'ordine di 2,5 punti oltre l'anno in corso nel 2000-2014, sceso a circa 0,5 sul PIL reale nel 2014-2019. Dal campione sono esclusi 2008, 2012 e 2020 (fonti/upb-focus-1-2022-accuratezza-previsioni.md:234-245, nota 8) [V]. Il numero basso vale quindi per anni tranquilli; in una partita con shock somiglia di più alla tabella della Commissione.
- [NON TROVATO] Una valutazione ex post della Banca d'Italia con RMSE per orizzonte sull'Italia (l'Occasional Paper 257 del 2015 riguarda l'area dell'euro). [NON TROVATO] Gli errori del WEO del FMI per l'Italia: non estratti; la Commissione li giudica comparabili ai propri (fonti/ec-dp027-2016-accuratezza-previsioni.md, abstract) [V].

## 5. Contro-esempi dai giochi
- **This Is the Police**: le idee degli agenti si scoprono facendone informatori, a un costo in fiducia e denaro ([[this-is-the-police]], fonti/wikipedia-this-is-the-police.md) [V, già nella base].
- **Hearts of Iron IV**: l'informazione si compra con un'agenzia d'intelligence a dipartimenti (+25% per campo), non a singolo acquisto ([[hearts-of-iron-4]], fonti/hoi4-wiki-intelligence-agency.md) [V, già nella base].
- **Democracy 4**: i sondaggi sono gratuiti e sempre visibili; il DLC aggiunge il sondaggio sui soli «likely voters» ([[democracy-4]], fonti/steam-democracy4-voting-systems.md) [V, già nella base].
- **The Political Process**: i sondaggi sono fra le azioni di campagna, accanto a raccolta fondi e comizi ([[the-political-process]], fonti/steam-political-process.md) [V, già nella base]; il costo non è descritto.
- [NON TROVATO] Political Machine 2024 e Suzerain: nessuna fonte rapida su sondaggi a pagamento.

## Per il nostro gioco

**Raccomandazioni, non decisioni** (le scelte restano al brainstorming del piano 9):

- **Prezzi sulla scala reale.** Sondaggio privato nazionale da 1.000-1.500 interviste: 10.000-20.000 €; con incroci regionali (10.000 interviste) 50.000-130.000 €. Test di una misura: 10.000-25.000 €. Centro studi: 80.000-250.000 € a turno. Contro la parte libera di un partito al 20% (273.000 € a turno): il sondaggio vale il 4-7%, il test il 4-9%, il centro studi il 30-90%. [INF] Se il centro studi deve competere con la campagna e non essere un acquisto fisso, la fascia alta è quella giusta.
- **Sondaggio privato.** L'errore di campionamento che cala con √n si calcola, ed è difendibile. Il calo dell'errore d'istituto non ha fonte: se lo si tiene, va marcato in `manopole.toml` come assunzione del piano 9. Un'alternativa documentabile [INF] è lasciare a `sigma_istituto` lo stesso valore e dare al sondaggio privato solo un n più grande e incroci più fini.
- **Test di una misura.** Errore della reazione di un gruppo proporzionale a 1/√(quota del gruppo × n del test), con l'effetto vero poco eterogeneo fra gruppi (Coppock e coautori). I gruppi piccoli restano rumorosi anche dopo il test.
- **Centro studi, stima del trimestre chiuso.** Togliere una parte dell'errore di `errore_stima_pil`, non tutto: per esempio un errore residuo pari al 50-70% di quello della prima stima [INF, nessuna fonte numerica]. Per inflazione, disoccupazione e disavanzo manca anche la base.
- **Centro studi, previsione a 4 turni.** Forchetta tarata sulla tabella della Commissione: per la crescita annua, 1σ ≈ 1,15 a uno-due turni e 1,84 a quattro; inflazione 0,86 → 2,28; disoccupazione 1,06 → 1,47; saldo 1,27 → 1,63; debito circa 3 punti a quattro turni [INF]. Se si prevede la crescita trimestrale annualizzata, invece, la forchetta va da circa 2 a circa 3 punti e oltre h = 2 resta piatta (Heinisch). Da tenere: per l'Italia anche i previsori ufficiali sono stati ottimisti di 0,4-0,8 punti sul PIL.
- **Aperto.** Se il centro studi debba avere un suo errore medio non nullo (ottimismo), come quelli veri. È un realismo che il giocatore può imparare a correggere, ma resta una scelta di design.

## Collegamenti
- [[consiglieri-e-informazione-imperfetta]]: i tre livelli e «l'informazione migliore si paga»; questa pagina mette i prezzi.
- [[sondaggi-ed-errore]]: il generatore a tre strati che il sondaggio privato modifica.
- [[finanziamento-dei-partiti]]: la scala della cassa (pagina sul branch del piano 8b).
- [[economia-italiana-parametri]]: le grandezze che il centro studi prevede.
- [[campagna-elettorale]]: l'altro uso della cassa.
- [[this-is-the-police]], [[hearts-of-iron-4]], [[democracy-4]], [[the-political-process]]: come i giochi prezzano l'informazione.

## Riferimenti

- **URL** https://www.ilpost.it/2022/08/02/costi-sondaggi-politici/ · [V]
  **perché conta qui** unico prezzo per intervista di un sondaggio politico italiano, da un docente di metodi
  **cosa se ne prende** CATI 5-6 € di costo, circa il doppio al cliente; CAWI 1.000-2.000 € a sondaggio; interviste di persona 20-25 €
- **URL** https://www.intotheminds.com/blog/it/opinione-sondaggi-prezzo/ · [V]
  **perché conta qui** listino commerciale con prezzi minimi e medi per metodo
  **cosa se ne prende** CAWI 5.000-7.500 €, CATI 5.000-12.000 €: la fascia alta del prezzo al cliente
- **URL** https://www.arera.it/fileadmin/allegati/gare/2023/96884287B4%20Tabella%20n.%201%20Integrazione%20offerta%20economica_signed.pdf · [V]
  **perché conta qui** prezzo pubblico di un lotto aggiuntivo di interviste CATI
  **cosa se ne prende** 13,3 € per intervista: il costo marginale del sovracampionamento
- **URL** https://www.ilfattoquotidiano.it/in-edicola/articoli/2026/02/18/referendum-giustizia-il-sondaggio-di-palazzo-chigi-sulle-urne-costo-120-mila-euro/8295580/ · [V]
  **perché conta qui** committenza politica reale e recente, con importo
  **cosa se ne prende** 3 rilevazioni per 120.000 €, circa 40.000 € l'una: tetto superiore di un sondaggio commissionato (corpo dell'articolo non letto)
- **URL** https://www.gonews.it/2020/11/13/sviluppi-su-inchiesta-open-pagati-a-renzi-sondaggi-pubblicita-e-book-foto/ · [V]
  **perché conta qui** spesa in sondaggi di un'area politica su due campagne
  **cosa se ne prende** oltre 130.000 € in due campagne (accusa, non accertamento): ordine di grandezza della spesa di una corrente
- **URL** https://web.archive.org/web/2022id_/https://fivethirtyeight.com/features/how-partisan-are-partisan-senate-polls/ · [V]
  **perché conta qui** unica misura trovata dell'effetto del committente su un sondaggio
  **cosa se ne prende** +4,25 punti verso il committente nei sondaggi di parte pubblicati: la riduzione dell'effetto istituto nei sondaggi privati non ha fonte
- **URL** https://pmc.ncbi.nlm.nih.gov/articles/PMC6298071/ · [V]
  **perché conta qui** misura quanto varia fra gruppi l'effetto di un messaggio in 27 esperimenti
  **cosa se ne prende** effetti per gruppo vicini alla media: il test rende precisa soprattutto la direzione, e i gruppi piccoli restano rumorosi
- **URL** https://fondazionemagnacarta.it/wp-content/uploads/2025/06/BILANCIO-FMC-2024-per-sito.pdf · [V]
  **perché conta qui** bilancio completo di una fondazione politica, con le voci
  **cosa se ne prende** 325.000 € l'anno, circa 81.000 € a trimestre: la fascia bassa del centro studi
- **URL** https://www.openpolis.it/esercizi/la-mappatura-dei-think-tank-quante-sono-cosa-fanno-e-quanto-sono-trasparenti/ · [V]
  **perché conta qui** censimento dei bilanci di think tank e fondazioni politiche italiane
  **cosa se ne prende** sette strutture oltre 1 milione, Italia decide 766.000 €: la fascia alta del centro studi
- **URL** https://www.upbilancio.it/wp-content/uploads/2020/06/Rendiconto-gestione-finanziaria-e-relazione-illustrativa-2019.pdf · [V]
  **perché conta qui** costo di un previsore pubblico indipendente
  **cosa se ne prende** 6 milioni l'anno e 24 persone: il tetto oltre cui un centro studi non è più di partito
- **URL** https://www.upbilancio.it/wp-content/uploads/2022/01/Focus-no_1_2022_EN.pdf · [V]
  **perché conta qui** accuratezza delle previsioni ufficiali italiane prima e dopo l'UPB
  **cosa se ne prende** RMSE del PIL 2,5 punti prima del 2014 e circa 0,5 dopo, su anni tranquilli: l'errore dipende dal periodo
- **URL** https://economy-finance.ec.europa.eu/document/download/f092a1d8-1a46-4a63-8392-37808195ea23_en?filename=dp027_en.pdf · [V]
  **perché conta qui** MAE e RMSE per l'Italia di PIL, inflazione, saldo e disoccupazione, anno in corso e anno dopo
  **cosa se ne prende** la tabella che tara la forchetta del centro studi e il suo allargamento (×1,3-2,7)
- **URL** https://www.iwh-halle.de/fileadmin/user_upload/iwh-discussion-paper_2024_22_Heinisch.pdf · [V]
  **perché conta qui** errori trimestrali per orizzonte da 0 a 8 trimestri, e revisioni dalla prima pubblicazione, con l'Italia
  **cosa se ne prende** oltre h = 1 la previsione non batte la media storica; revisione assoluta media 0,271 trimestrale, circa 1 annualizzato
- **URL** https://osservatoriocpi.unicatt.it/pubblicazioni/2025/i-primi-dati-istat-sottostimano-sistematicamente-il-pil.html · [V]
  **perché conta qui** mostra che la prima stima ISTAT è distorta al ribasso
  **cosa se ne prende** il margine reale per una «stima migliore del trimestre chiuso»: correggere la distorsione
- **URL** https://www.istat.it/it/files//2017/06/Scheda_revisioni_I-2017.pdf · [V]
  **perché conta qui** revisione fra stima preliminare e prima stima trimestrale
  **cosa se ne prende** 0,05 punti: la revisione a 25 giorni è minima, il grosso arriva anni dopo
