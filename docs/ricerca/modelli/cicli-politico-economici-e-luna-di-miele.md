---
tipo: modello
fonti: [fonti/nber-brender-drazen-2004.md, fonti/brender-drazen-2004-testo.md, fonti/louwerse-muller-2020-ciclo-elettorale.md, fonti/nannestad-paldam-1999-cost-of-ruling.md]
aggiornato: 2026-10-16
---
# Cicli politico-economici e luna di miele

Due regolarità legate al calendario elettorale. Il ciclo politico-economico: i governi spingono deficit e spesa prima del voto. La curva del consenso: il governo perde nella prima metà della legislatura e recupera poco. Per il gioco il primo è una tentazione da offrire al giocatore, con un prezzo; il secondo è la forma temporale da dare al consenso.

## Modelli

- **Ciclo opportunistico** (Nordhaus 1975): il governo stimola l'economia prima del voto e paga dopo, sfruttando elettori miopi [INF].
- **Ciclo partigiano** (Hibbs 1977; versione "razionale" di Alesina): sinistra e destra scelgono combinazioni diverse di inflazione e disoccupazione; con aspettative razionali l'effetto reale dura solo all'inizio del mandato [INF].
- **Ciclo di bilancio** (political budget cycle): il deficit sale nell'anno elettorale (fonti/brender-drazen-2004-testo.md) [V].
- **Curva del consenso**: perdita nella prima metà della legislatura, recupero parziale (fonti/louwerse-muller-2020-ciclo-elettorale.md) [V]. La "coalizione delle minoranze" di Mueller (1970) è la spiegazione classica: ogni decisione scontenta qualcuno e gli scontenti si sommano [INF]; Nannestad e Paldam la giudicano la meno difendibile e preferiscono l'asimmetria del malcontento, che però predice una perdita lineare (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V].

## Evidenza

- **Il ciclo di bilancio esiste solo nelle democrazie nuove**: nel campione ampio il deficit sale di circa 0,3 punti di PIL nell'anno elettorale; togliendo le democrazie nuove, nelle democrazie consolidate il ciclo sparisce (fonti/brender-drazen-2004-testo.md, fonti/nber-brender-drazen-2004.md) [V]. Altri studi riportati dagli autori (Shi-Svensson) trovano un calo dell'avanzo di 0,5-1 punto di PIL nell'anno elettorale (fonti/brender-drazen-2004-testo.md) [V].
- Gli autori collegano il risultato all'idea che gli elettori delle democrazie consolidate puniscano, invece di premiare, la manipolazione fiscale (fonti/nber-brender-drazen-2004.md) [V].
- **Curva del consenso**: 171 legislature in 22 democrazie parlamentari, intenzioni di voto: in media i partiti di governo perdono nella prima metà e recuperano al massimo in parte; la forma a U è più netta con governi monocolore e quando il premier controlla lo scioglimento; dagli anni 2000 il recupero è quasi assente (fonti/louwerse-muller-2020-ciclo-elettorale.md) [V].
- **Luna di miele**: uno studio su 271 leader in 40 paesi (Comparative Political Studies 2024) trova una luna di miele per i leader al primo mandato, più forte nei sistemi presidenziali e assente per i rieletti [INF, solo abstract nei risultati di ricerca].
- **Rally**: le crisi esterne alzano temporaneamente il consenso; trattato in [[costo-politico-della-guerra]].

## Parametri tipici

| Parametro | Valore | Fonte |
|---|---|---|
| aumento del deficit nell'anno elettorale, campione misto | ~0,3 punti di PIL | (fonti/brender-drazen-2004-testo.md) [V] |
| stesso, democrazie consolidate | non significativo | (fonti/nber-brender-drazen-2004.md) [V] |
| perdita del governo a fine legislatura | ~2,25 punti | (fonti/nannestad-paldam-1999-cost-of-ruling.md) [V] |
| forma | calo nella prima metà, recupero parziale; recupero quasi nullo dal 2000 | (fonti/louwerse-muller-2020-ciclo-elettorale.md) [V] |
| durata della luna di miele | [NON TROVATO] in fonti primarie catturate | |

## Costo computazionale

Una funzione del tempo dall'ultima elezione più un termine di bilancio: `O(1)` per tick [INF].

## Cosa semplifichiamo

- La curva del consenso come funzione del tempo (forma a U asimmetrica) sommata al voto economico, invece di farla emergere dagli elettori [INF]. Alternativa: farla emergere dall'asimmetria del malcontento di [[voto-economico-e-retrospettivo]], che però dà una retta.
- Nessuna distinzione fra spesa e tasse nel ciclo di bilancio [INF].

## Per il nostro gioco

- Offrire la leva "manovra pre-elettorale" (deficit in più nell'anno del voto) con effetto elettorale piccolo o negativo in una democrazia consolidata come l'Italia: è ciò che dice l'evidenza, e rompe il cliché da gioco "spendi prima del voto e vinci". In scenari di democrazie nuove l'effetto può essere positivo.
- Consenso del governo: calo nella prima metà della legislatura, recupero parziale; il recupero più forte se il premier controlla lo scioglimento anticipato. Da regolare con un parametro per epoca (prima e dopo il 2000).
- Aperto: chi gioca può sciogliere le camere? In Italia lo decide il presidente della Repubblica; la meccanica va decisa con il filone istituzioni.

## Collegamenti

- [[voto-economico-e-retrospettivo]]: costo del governare e funzione VP.
- [[costo-politico-della-guerra]]: effetto rally.
- [[policy-e-effetti]]: ritardi fra policy ed effetti nei giochi esistenti.
- [[sondaggi-ed-errore]]: la curva si vede attraverso i sondaggi.
- [[tempo-e-ritmo]]: calendario e arco della partita.

## Riferimenti

- **URL** https://www.nber.org/system/files/working_papers/w10539/w10539.pdf · [V]
  **perché conta qui** mostra che il ciclo di bilancio dipende dall'età della democrazia
  **cosa se ne prende** 0,3 punti di PIL e l'assenza di effetto nelle democrazie consolidate
- **URL** https://www.tomlouwerse.nl/publication/2018-muller-louwerse-psrm · [V]
  **perché conta qui** forma della curva del consenso su 22 democrazie parlamentari
  **cosa se ne prende** calo nella prima metà, recupero parziale, ruolo dello scioglimento
- **URL** https://repec.econ.au.dk/repec/afn/wp/99/wp99_9.pdf · [V]
  **perché conta qui** confronta le spiegazioni del calo di consenso
  **cosa se ne prende** perché scartare la "coalizione delle minoranze" come meccanismo
- **URL** https://journals.sagepub.com/doi/10.1177/00104140241237481 · [INF]
  **perché conta qui** luna di miele misurata su 271 leader
  **cosa se ne prende** luna di miele solo al primo mandato; da leggere per la durata
