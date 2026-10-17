---
tipo: gioco
fonti: [fonti/vic3-wiki-market.md, fonti/vic3-wiki-pops.md, fonti/vic3-wiki-standard-of-living.md, fonti/vic3-wiki-goods.md, fonti/vic3-wiki-building.md, fonti/vic3-wiki-production-methods.md, fonti/vic3-wiki-laws.md, fonti/vic3-wiki-government.md, fonti/vic3-wiki-political-movement.md, fonti/vic3-wiki-diplomacy.md, fonti/vic3-wiki-diplomatic-play.md, fonti/vic3-wiki-war-estratto.md, fonti/vic3-dd09-mercati-nazionali.md, fonti/vic3-dd54-revisioni-commercio.md, fonti/wikipedia-victoria-3.md, fonti/steam-victoria3.md, fonti/steam-recensioni-realismo.md, fonti/vic3-wiki-interest-groups.md, fonti/vic3-dd57-journey.md, fonti/pcgamesn-victoria3-nested-tooltip.md]
aggiornato: 2026-10-16
---
# Victoria 3

Grand strategy economico-politico di Paradox, 1836-1936. È il titolo più vicino al nostro: economia a mercati con prezzi da domanda e offerta, popolazione in gruppi sociali che consumano e votano, leggi che passano per una procedura con esito incerto. È anche l'esempio più chiaro di un'economia simulata lodata e di una guerra simulata detestata nello stesso gioco.

## Scala e loop

- Copre il periodo 1836-1936 con oltre 100 paesi giocabili (fonti/wikipedia-victoria-3.md).
- Ritmo misto: i pop spendono il salario ogni settimana (fonti/vic3-wiki-standard-of-living.md); l'escalation diplomatica e l'avanzata al fronte crescono ogni giorno (fonti/vic3-wiki-diplomatic-play.md, fonti/vic3-wiki-war-estratto.md); le leggi avanzano per checkpoint di 100 giorni (fonti/vic3-wiki-laws.md).
- Loop: costruisci edifici con il settore edilizio → gli edifici assumono pop e comprano input sul mercato → i prezzi si muovono → i pop guadagnano, consumano e cambiano tenore di vita → il tenore muove lealtà e radicalismo → i gruppi d'interesse spingono o bloccano le leggi.
- Recensioni Steam: 70% positive su 20.655 in inglese; 69% sulle ultime 135 (fonti/steam-victoria3.md).

## Economia: mercati e prezzi

- Formula del prezzo: prezzo = base × [1 + 0,75 × clamp((BUY − SELL) / min(BUY, SELL), ±1)], quindi fra il 25% e il 175% della base (fonti/vic3-wiki-market.md).
- Ordini di acquisto e vendita vengono da consumo e produzione locali, come su una borsa merci (fonti/vic3-dd09-mercati-nazionali.md). Non esistono scorte di beni: il sistema rinuncia allo stockpile di Victoria 2 (fonti/vic3-dd09-mercati-nazionali.md).
- Il mercato non conserva il valore: tutti gli ordini di acquisto sono pagati e tutti quelli di vendita incassati anche quando non coincidono, e la differenza viene creata o distrutta dalla simulazione (fonti/vic3-wiki-market.md).
- Accesso al mercato = infrastruttura / uso dell'infrastruttura, al massimo 100%. Il prezzo locale = MAPI × prezzo di mercato + (1 − MAPI) × prezzo dello stato, con MAPI base 75% (fonti/vic3-wiki-market.md).
- Carenza quando gli ordini di acquisto sono almeno il doppio di quelli di vendita: −5% di throughput agli edifici che usano il bene, −1% al giorno fino a −50% (−75% se il bene manca del tutto), recupero di 1% al giorno (fonti/vic3-wiki-market.md).
- Commercio estero: vantaggio commerciale a somma zero, base 100, +2 per ogni punto percentuale di produzione mondiale nel mercato; monopolio fino a +20% sul prezzo mondiale (fonti/vic3-wiki-market.md). Le rotte commerciali in una versione precedente crescevano o calavano da sole ogni settimana secondo le condizioni di mercato (fonti/vic3-dd54-revisioni-commercio.md); la pagina wiki di agosto 2026 descrive invece un mercato mondiale con hub portuali, quindi il modello è cambiato almeno una volta [INF].

## Popolazione e tenore di vita

- Ogni pop ha cultura, religione, professione, luogo di lavoro, alfabetizzazione e ricchezza (fonti/vic3-wiki-pops.md).
- Tenore di vita da 1 a 99, in fasce da Destitute (1-4) a Opulent (60+) (fonti/vic3-wiki-standard-of-living.md).
- La ricchezza sale se il reddito settimanale supera il 102% del paniere successivo, a velocità 0,2 × differenza; scende se il reddito è sotto il 98% delle spese, a 0,1 × differenza (fonti/vic3-wiki-standard-of-living.md).
- Il tenore atteso dipende dallo strato sociale più un bonus da alfabetizzazione (massimo 5, +1 per ciascuna di cinque tecnologie sociali). Sotto l'atteso il pop si radicalizza (fonti/vic3-wiki-standard-of-living.md).
- La forza politica di un pop dipende dalla sua ricchezza e alimenta il peso (clout) dei gruppi d'interesse che sostiene (fonti/vic3-wiki-pops.md, fonti/vic3-wiki-interest-groups.md).

## Leggi e governo

- Una legge passa per tre fasi; a ogni checkpoint (100 giorni di base) esce Success, Advance, Debate o Stall. Tre battute d'arresto la fanno fallire, e non si ripropone per due anni (fonti/vic3-wiki-laws.md).
- Probabilità base = somma del clout dei gruppi al governo che la sostengono + sostegno dei movimenti + appoggio del sovrano (fonti/vic3-wiki-laws.md).
- Legittimità 0-100; sotto 25 il governo è illegittimo e passa solo leggi sostenute da movimenti (fonti/vic3-wiki-government.md).
- I movimenti politici sono entità separate dai gruppi d'interesse, nati per tenere stabile l'identità dei gruppi (fonti/vic3-dd57-journey.md, fonti/vic3-wiki-political-movement.md).

## Diplomazia e guerra

- Ogni guerra nasce da un diplomatic play: 20 giorni di mosse iniziali, 60 di manovre, 20 di conto alla rovescia; ogni azione mette in pausa l'escalation di 5 giorni (fonti/vic3-wiki-diplomatic-play.md).
- Le manovre disponibili dipendono dal rango (grande potenza 100) (fonti/vic3-wiki-diplomatic-play.md).
- Guerra a fronti: un fronte per coppia di teatri, avanza di uno stato alla volta; i generali ricevono ordini di avanzata o difesa e l'avanzata scatta a 100 punti di progresso (fonti/vic3-wiki-war-estratto.md).
- Supporto alla guerra da +100; capitolazione automatica a −100. Mobilitazione in 5-20 giorni (fonti/vic3-wiki-war-estratto.md).

## Cosa dicono i giocatori sul realismo

- Lodati: sistemi politico, economico e diplomatico "deep, but not overwhelming"; l'economia è il cuore del gioco (fonti/steam-recensioni-realismo.md).
- Criticata: la guerra. "100k vs 20k with the same stats doesn't provide the expected outcome"; battaglie automatiche "stupid"; marina confusa (fonti/steam-recensioni-realismo.md).
- Criticati: IA economica poco abile, rallentamento a fine partita, conseguenze delle azioni spiegate male (fonti/steam-recensioni-realismo.md).
- Per rendere leggibile l'economia Paradox ha adottato i tooltip annidati di Crusader Kings III (fonti/pcgamesn-victoria3-nested-tooltip.md).

## Per il nostro gioco

- Prendiamo: la formula del prezzo con clamp a ±75%. È una riga, è spiegabile in un tooltip e dà carenze ed eccessi credibili.
- Prendiamo: tenore di vita atteso contro reale come motore del radicalismo. È la variabile che collega economia e politica.
- Prendiamo: leggi con iter incerto a checkpoint. Si adatta a un parlamento vero (vedi [[parlamento-e-legislazione]]).
- Evitiamo: un mercato che crea o distrugge denaro. Un economista se ne accorge; la [[motore-economico-raccomandato]] propone contabilità stock-flow consistent proprio per evitarlo.
- Evitiamo: una guerra tattica che il giocatore non controlla ma di cui subisce esiti opachi. O si controlla, o è un esito aggregato spiegato.
- Aperto: quanti pop. Victoria 3 rallenta a fine partita; Stellaris ha raggruppato i pop proprio per prestazioni ([[stellaris]]).

## Collegamenti

- [[mercati-e-pops]]: il modello economico nel dettaglio.
- [[europa-universalis]]: EU5 importa pop e mercati di Victoria nel Rinascimento.
- [[elettorato-e-gruppi-di-voto]]: i gruppi d'interesse visti dal filone politico.
- [[parlamento-e-legislazione]]: iter delle leggi.
- [[motore-economico-raccomandato]]: perché non copiare il mercato non conservativo.

## Riferimenti

- **URL** https://vic3.paradoxwikis.com/Market · [V] (snapshot Wayback 2026-08-19)
  **perche' conta qui** formula del prezzo, accesso al mercato, carenze
  **cosa se ne prende** clamp ±75%, soglia di carenza al 2:1, penalità che cresce di 1% al giorno
- **URL** https://vic3.paradoxwikis.com/Standard_of_living · [V] (snapshot 2026-08-18)
  **perche' conta qui** collega reddito, consumo e radicalismo
  **cosa se ne prende** soglie 102%/98% e tenore atteso per strato
- **URL** https://vic3.paradoxwikis.com/Laws · [V] (snapshot 2026-08-19)
  **perche' conta qui** iter di una legge con esito incerto
  **cosa se ne prende** checkpoint di 100 giorni e tre esiti intermedi
- **URL** https://forum.paradoxplaza.com/forum/developer-diary/victoria-3-dev-diary-9-national-markets.1484917/ · [V]
  **perche' conta qui** intenzione di design dei mercati
  **cosa se ne prende** ordini come borsa merci, niente scorte
- **URL** https://vic3.paradoxwikis.com/War · [V] (estratto WebFetch, non pagina integrale)
  **perche' conta qui** guerra a fronti e capitolazione
  **cosa se ne prende** war support +100/−100 come contatore di resa
