---
tipo: sistema
fonti: [fonti/cs2-devdiary-servizi.md, fonti/cs2-steam-economia-2-0.md, fonti/cs1-wiki-politiche.md, fonti/simcity-wiki-sc4.md, fonti/simcity-wiki-gradimento-sindaco.md, fonti/anno1800-wiki-bisogni.md, fonti/anno117-wiki-popolazione.md, fonti/tropico-wiki-felicita.md, fonti/wr-wiki-cittadini.md, fonti/frostpunk-wiki-malcontento.md]
aggiornato: 2026-10-16
---
# Servizi pubblici

Come i gestionali modellano sanità, istruzione, sicurezza, rifiuti e simili: dove arriva il servizio, quanto costa, quanto rende, come lo vede il giocatore. È il modo in cui un ministero diventa tangibile sul territorio.

## Tre modelli di copertura

1. **Raggio + capacità + intensità (Cities: Skylines II).** Effetto passivo che si propaga lungo le strade: il raggio è in metri, la capacità dice quante persone raggiunge (in zone dense si esaurisce prima), l'intensità è piena nel raggio e cala al bordo. A questo si somma un effetto simulato: veicoli di pattuglia e cittadini che vanno al servizio. Il crimine, per esempio, si gestisce con il calo locale vicino al commissariato più le pattuglie in giro per la città (fonti/cs2-devdiary-servizi.md).
2. **Distanza su rete stradale con soddisfazione parziale (Anno).** Un bisogno "di edificio" è pieno se la casa sta entro N caselle stradali dal servizio, parziale oltre (fonti/anno1800-wiki-bisogni.md). In Anno 117 i servizi alimentano attributi della casa come salute (medici) e sicurezza antincendio (vigili) (fonti/anno117-wiki-popolazione.md).
3. **Visita fisica dell'individuo (Workers & Resources).** Il cittadino va dal medico a intervalli e vuole un servizio entro il suo raggio a piedi; se manca, compare una nota rossa sull'edificio ("Non riesco a vedere un medico") (fonti/wr-wiki-cittadini.md).

SimCity 4 sta in mezzo: gli edifici civici agiscono su un'area vicina e non più su tutta la città, con finanziamento regolabile edificio per edificio (fonti/simcity-wiki-sc4.md).

## Bilancio ed efficienza

- **Cursore per categoria.** In Cities: Skylines II il bilancio di un servizio alza o abbassa insieme manutenzione ed efficienza di tutti gli edifici di quella categoria. L'efficienza dipende anche da acqua, elettricità e dipendenti (numero, felicità, salute, istruzione) (fonti/cs2-devdiary-servizi.md).
- **Sottofinanziare costa nel lungo periodo.** In SimCity 4 gli edifici civici sottofinanziati e i livelli bassi di istruzione e sanità sono fattori lenti contro il gradimento del sindaco (fonti/simcity-wiki-gradimento-sindaco.md).
- **Le politiche di servizio scambiano costo e qualità.** In Cities: Skylines, "Istruzione a scopo di lucro" taglia del 50% la manutenzione delle scuole e abbassa la felicità; "Fiera del libro" raddoppia la manutenzione delle biblioteche per più felicità (fonti/cs1-wiki-politiche.md).
- **Importare servizi.** In Cities: Skylines II i servizi mancanti arrivano da città vicine; con Economy 2.0 l'import costa in proporzione alla popolazione ed è disattivato di default, mentre la manutenzione dei servizi è stata alzata "per dare più peso alle scelte" (fonti/cs2-steam-economia-2-0.md).

## Servizi e consenso

- In Tropico 6 sei delle otto voci della felicità sono servizi o beni (cibo, sanità, svago, fede, casa, lavoro), più libertà e sicurezza (fonti/tropico-wiki-felicita.md).
- In Frostpunk la mancanza di cure, il freddo e la fame alzano il malcontento, cioè il rischio di essere deposti (fonti/frostpunk-wiki-malcontento.md).
- In Cities: Skylines II i servizi non alzano più direttamente il valore del terreno: lo fanno attraverso la disponibilità a pagare affitti più alti (fonti/cs2-devdiary-servizi.md).

## Per il nostro gioco

- **Prendiamo:** copertura a raggio-capacità-intensità calcolata per territorio come modello di base, perché è economica, si disegna come mappa e regge l'aggregazione; un cursore di bilancio per ministero che sposta insieme costo ed efficienza; il personale del servizio (numero, salute, istruzione) come fattore di efficienza.
- **Evitiamo:** la visita fisica dell'individuo come unico canale (W&R); il servizio che aumenta il valore in modo diretto e automatico.
- **Aperto:** l'equivalente nazionale dell'"import di servizi" (convenzioni con il privato, accordi con l'estero).

## Collegamenti

- [[sanita-e-spesa-sanitaria]]: il ministero della sanità.
- [[feedback-visivo-dello-stato]]: mappe di copertura e note sugli edifici.
- [[cittadini-simulati]]: chi riceve il servizio.
- [[cities-skylines]], [[anno]], [[simcity-4]], [[workers-and-resources-soviet-republic]].
- [[istruzione-e-capitale-umano]]: il modello dell'istruzione dietro le scuole.
- [[criminalita-e-giustizia]]: il modello della sicurezza dietro i commissariati.
