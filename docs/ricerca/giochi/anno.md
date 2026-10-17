---
tipo: gioco
fonti: [fonti/anno1800-wiki-bisogni.md, fonti/anno1800-wiki-rotte-commerciali.md, fonti/anno1800-wiki-commercio.md, fonti/anno1800-wiki-giornali.md, fonti/anno117-wiki-popolazione.md, fonti/anno117-wiki-gioco.md, fonti/steam-api-snapshot-gestionali.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Anno (1800 e 117: Pax Romana)

Serie di city builder economici di Ubisoft Mainz: isole, catene produttive, rotte commerciali, popolazione divisa in ceti che chiedono beni sempre più complessi. Per noi conta il commercio come sistema visibile e manovrabile (rotte, navi, magazzini) e la popolazione modellata per abitazione invece che per individuo, con formule documentate.

## Scheda

| | Anno 1800 | Anno 117: Pax Romana |
|---|---|---|
| Uscita | 16/04/2019, Windows | 12/11/2025, Windows |
| Recensioni Steam | 32.661, 81% positive | 12.988, 69% (Mixed) |
| Requisiti minimi | i5 3470, 8 GB, GTX 670 2 GB, 60 GB | i7-7700 / Ryzen 5 1600, 16 GB, GTX 1660 6 GB, DX12, AVX2, SSD, 117 GB |

Dati: (fonti/steam-api-snapshot-gestionali.md).

## Scala e loop

Si fonda su un'isola, si soddisfano i bisogni del ceto più basso, le case salgono di ceto e chiedono beni nuovi, che spesso richiedono fertilità presenti solo su altre isole: da qui espansione e commercio (fonti/anno1800-wiki-bisogni.md; fonti/anno117-wiki-popolazione.md). Anno 117 ha due province interdipendenti, Lazio e Britannia, con beni producibili in una sola delle due (fonti/anno117-wiki-gioco.md).

## Come modella la cosa

**Popolazione per abitazione, non per individuo.** I bisogni "di beni" sono soddisfatti se il bene è nel magazzino dell'isola e la casa è collegata via strada al porto; quelli "di edificio" se la casa è entro un certo numero di caselle stradali dall'edificio, con soddisfazione parziale oltre (fonti/anno1800-wiki-bisogni.md). Il consumo di una casa dipende dalla sua capienza massima, non dagli abitanti presenti (fonti/anno1800-wiki-bisogni.md):

consumo = C · (1 + R) · (1 + N) · (1 + B)

dove C è il consumo base (es. 0,0004166667 t/s di pesce per i contadini), R l'aumento di capienza, N lo sconto da propaganda sul giornale, B i bonus da oggetti del municipio (fonti/anno1800-wiki-bisogni.md). I benefici (abitanti, reddito, felicità) scalano in modo proporzionale alla percentuale di soddisfazione (fonti/anno1800-wiki-bisogni.md).

**Anno 117: bisogni per categoria.** Per salire di ceto basta un bisogno per categoria (Cibo, Moda, Servizio pubblico…), con punti maggiori per beni più complessi; ogni casa ha otto attributi (reddito, popolazione, felicità, sicurezza antincendio, salute, fede, prestigio, conoscenza) (fonti/anno117-wiki-popolazione.md).

**Rotte commerciali.** Ogni rotta collega più isole e trasporta più beni con più navi; gli slot utilizzabili sono quelli delle navi assegnate. Le opzioni per tappa: attendi le merci, getta il carico, attendi lo scarico (fonti/anno1800-wiki-rotte-commerciali.md). La wiki documenta tre architetture che i giocatori costruiscono: punto a punto, circolare, hub and spoke (fonti/anno1800-wiki-rotte-commerciali.md). Il commercio con gli avversari IA richiede un accordo commerciale e migliora la reputazione diplomatica in proporzione al valore scambiato (fonti/anno1800-wiki-commercio.md).

**Giornale e propaganda.** A intervalli esce un giornale con tre articoli sugli eventi dell'impero, ciascuno a +5, −5 o 0 di felicità globale. Con punti influenza si sostituiscono gli articoli: "Ipnosi" vale +5/+10/+15 di felicità al costo di 30/45/75 influenza, "Austerità" −10/−20/−30% di consumi, "Legge marziale" −10/−25/−50% di rischio rivolte. L'abuso di propaganda abbassa la felicità e provoca rivolte contro la censura (fonti/anno1800-wiki-giornali.md).

## Realismo: cosa dicono i giocatori

Si lodano le catene produttive che "si costruiscono una sull'altra" e i problemi logistici come piacere del gioco. Si critica il diluvio di notifiche a fine partita ("rotta con grave problema", "nave sotto attacco"), che riduce il gioco a reagire agli avvisi (fonti/steam-recensioni-gestionali.md). Di Anno 117 i giocatori citano soprattutto Ubisoft Connect, i DLC e i problemi tecnici; un recensore non tiene 60 fps stabili su RTX 4090 (fonti/steam-recensioni-gestionali.md).

## Prestazioni

Il salto dei requisiti fra 1800 e 117 è netto: si passa a 16 GB di RAM, una GPU da 6 GB, AVX2 e SSD obbligatori e 117 GB su disco (fonti/steam-api-snapshot-gestionali.md).

## Per il nostro gioco

- **Prendiamo:** il commercio come rotte disegnate dal giocatore con capacità e regole di carico, con un magazzino aggregato per territorio; la popolazione per ceto e per abitazione con consumo da formula semplice e benefici proporzionali alla soddisfazione; il giornale come canale che riassume lo stato e come leva di propaganda a costo crescente e con rischio.
- **Evitiamo:** la gestione delle notifiche che a fine partita diventa il gioco; i requisiti di Anno 117.
- **Aperto:** quale aggregato territoriale fa da "isola" nel nostro gioco (regione? provincia?).

## Collegamenti

- [[logistica-e-trasporti]]: rotte punto a punto, circolari, hub.
- [[cittadini-simulati]]: il modello "per abitazione".
- [[feedback-visivo-dello-stato]]: giornale, barre di soddisfazione, statistiche per isola.
- [[commercio-modello-gravitazionale]]: commercio con i rivali e reputazione.
