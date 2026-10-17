---
tipo: sistema
fonti: [fonti/cs2-devdiary-traffico.md, fonti/cs2-devdiary-cittadini.md, fonti/cs2-devdiary-servizi.md, fonti/cs1-wiki-problemi.md, fonti/cs1-wiki-valore-terreno.md, fonti/cs1-wiki-distretti.md, fonti/simcity2013-wiki-interfaccia.md, fonti/simcity-wiki-consiglieri.md, fonti/anno1800-wiki-bisogni.md, fonti/anno1800-wiki-giornali.md, fonti/anno1800-wiki-rotte-commerciali.md, fonti/wr-wiki-cittadini.md, fonti/frostpunk-wiki-speranza.md, fonti/frostpunk-wiki-malcontento.md, fonti/frostpunk2-steam-dopo-beta.md, fonti/tropico6-wiki-discorso-elettorale.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-17
---
# Feedback visivo dello stato

Come il giocatore vede a colpo d'occhio la salute di città o paese. Nel filone ricorrono sei canali: mappe tematiche, icone sui luoghi, indicatori sintetici, pannelli per luogo, voci di individui e un riassunto periodico. Per noi conta combinarli senza trasformare il gioco in una gestione di notifiche.

## I sei canali

1. **Mappe tematiche (infoview).**
   - Cities: Skylines II colora le strade per flusso e volume, con gli ingorghi in rosso (fonti/cs2-devdiary-traffico.md), e ha una vista sanità con salute per edificio e media cittadina (fonti/cs2-devdiary-cittadini.md).
   - Cities: Skylines mostra il valore del terreno come mappa di calore dal verde al blu (fonti/cs1-wiki-valore-terreno.md).
   - SimCity 2013 apre con ogni strumento le mappe pertinenti: per l'acqua falda, inquinamento del suolo, germi e salute; per la polizia crimine e istruzione (fonti/simcity2013-wiki-interfaccia.md).
2. **Icone sui luoghi.**
   - Cities: Skylines mette una bolla pulsante sopra l'edificio con il problema (acqua, lavoratori, rifiuti), in due stadi di gravità; con più problemi le icone ruotano (fonti/cs1-wiki-problemi.md).
   - Workers & Resources mostra note rosse nel pannello dell'edificio ("temperatura troppo bassa", "non riesco a vedere un medico") (fonti/wr-wiki-cittadini.md).
3. **Indicatori sintetici sempre visibili.**
   - Frostpunk: speranza e malcontento, che decidono sconfitta e deposizione (fonti/frostpunk-wiki-speranza.md; fonti/frostpunk-wiki-malcontento.md).
   - SimCity 4: consiglieri con sfondo colorato per umore, verde se soddisfatti (fonti/simcity-wiki-consiglieri.md).
4. **Pannelli per luogo e per gruppo.**
   - Distretto di Cities: felicità, specializzazione, popolazione, livelli, politiche (fonti/cs1-wiki-distretti.md).
   - Casa di Anno: barre dei bisogni (vuote, blu se parziali, verdi se piene) e un menu di statistiche con le medie per isola (fonti/anno1800-wiki-bisogni.md).
   - Rotte di Anno: scorte e tendenza di ogni bene sopra le isole della mappa (fonti/anno1800-wiki-rotte-commerciali.md).
5. **Voci di individui.**
   - Cities: Skylines II ha Chirper, con messaggi legati a eventi reali i cui "mi piace" indicano quanto il tema pesa, e un diario per seguire singoli cittadini (fonti/cs2-devdiary-cittadini.md).
   - Frostpunk 2 ha aggiunto le "Zoom Stories" dopo che la beta aveva fatto perdere il legame con gli individui (fonti/frostpunk2-steam-dopo-beta.md).
   - Workers & Resources permette di aprire il singolo cittadino con preferenze e tempo di viaggio (fonti/wr-wiki-cittadini.md).
6. **Riassunto periodico.**
   - Anno 1800 pubblica un giornale di tre articoli sugli eventi recenti, ciascuno a ±5 di felicità, manipolabile con la propaganda (fonti/anno1800-wiki-giornali.md).
   - Il discorso elettorale di Tropico 6 è un momento di bilancio sui temi della felicità (fonti/tropico6-wiki-discorso-elettorale.md).

## Gerarchia delle notifiche

Workers & Resources raggruppa le notifiche per impatto (informative, di avviso, critiche) e distingue quelle sul sistema da quelle sulle preferenze dei cittadini (fonti/wr-wiki-cittadini.md). In Anno 1800 le notifiche di fine partita ("rotta con grave problema", "nave sotto attacco") sono la critica principale (fonti/steam-recensioni-gestionali.md).

## Per il nostro gioco

- **Prendiamo:**
  - due o tre indicatori sintetici sempre a schermo (sul modello di speranza e malcontento);
  - mappe tematiche aperte dal ministero o dallo strumento in uso, non da un menu separato (SimCity 2013);
  - icone di problema sul territorio con due livelli di gravità (Cities: Skylines);
  - un giornale periodico che riassume e che il governo può manipolare a costo (Anno);
  - voci campionate di cittadini legate a eventi reali (Chirper).
- **Evitiamo:** notifiche per singolo oggetto che crescono con la partita.
- **Aperto:** quale indicatore sintetico rappresenta la "salute del paese" (consenso? stabilità? entrambi?). Deciso il 17/10/2026: tre indicatori, uno per ogni modo di perdere il governo (consenso, maggioranza, spread), nessun indice composito (docs/ricerca/sintesi/decisioni.md).

## Collegamenti

- [[consiglieri-e-informazione-imperfetta]]: chi riporta lo stato, e quanto è affidabile.
- [[cittadini-simulati]]: le voci individuali come finestra sull'aggregato.
- [[territorio-e-urbanistica]]: le mappe di calore.
- [[crisi-ed-eventi]]: le notifiche critiche.
- [[interfacce-di-giochi-gestionali]]: la stessa domanda dal lato della direzione artistica (pagina dell'agente estetica).
- [[data-viz-cinematica]]: come disegnare gli indicatori sintetici.
- [[movimento-e-feedback]]: animazione delle icone di problema.
