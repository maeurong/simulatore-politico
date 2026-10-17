---
tipo: legale
fonti: [fonti/copyright-office-games.md, fonti/wiki-tetris-v-xio.md, fonti/wiki-sas-v-wpl.md, fonti/democracy4-modding.md, fonti/steam-democracy4.md, fonti/democracy4-faq.md, fonti/steam-lawgivers2.md, fonti/github-aidemocracy.md]
aggiornato: 2026-10-16
---
# IP e giochi di riferimento: cosa si puo' riprendere

Cosa possiamo prendere da Democracy 4, Lawgivers II e simili in un repo pubblico. In sintesi: idee, regole e meccaniche sono libere; testi, arte, icone, dati compilati e nomi-marchio no; il "look and feel" copiato in blocco e' rischioso. Questa pagina non e' una consulenza legale: prima della pubblicazione serve il parere di un professionista.

## Principi (fonti primarie e casi)

- **USA, Copyright Office**: "The idea for a game is not protected by copyright. The name or title given to the game and the methods for playing it are not protected by copyright." Sono invece proteggibili il testo che descrive le regole e l'arte grafica (fonti/copyright-office-games.md). [V]
- **USA, Tetris Holding v. Xio (D.N.J. 2012)**: il gameplay di Tetris non e' proteggibile (blocchi che cadono, rotazione, linee), ma la combinazione degli elementi espressivi si', e Mino li aveva copiati tutti senza altro scopo che "to avoid the difficult task of developing its own take on a known idea". Anche colori e stile dei pezzi nel marketing creavano confusione sul trade dress. Il caso e' vincolante solo nel distretto, ma e' citato come riferimento per il "look and feel" (fonti/wiki-tetris-v-xio.md). [V, via Wikipedia: fonte secondaria]
- **UE, SAS Institute v WPL (C-406/10, 2012)**: "Neither the functionality of a computer program nor the programming language and the format of data files... constitute a form of expression of that program and are not protected by copyright" (fonti/wiki-sas-v-wpl.md). [V, via Wikipedia; sentenza CGUE non catturata: https://eur-lex.europa.eu/legal-content/EN/TXT/?uri=CELEX:62010CJ0406]

## Applicazione ai giochi di riferimento

| Elemento | Si puo' riprendere? | Base |
|---|---|---|
| Idea "policy → variabili → gruppi di voto" con slider ed effetti | si', e' un'idea e un metodo di gioco | copyright-office-games; SAS |
| Sintassi `target,formula,inertia` in CSV | si' come formato e funzionalita' (UE) [INF] | SAS |
| I CSV di Positech (policy, testi, equazioni e valori compilati) | **no**: espressione e, nell'UE, possibile diritto sui generis sulle banche dati [INF; direttiva 96/9/CE non catturata: https://eur-lex.europa.eu/eli/dir/1996/9/oj] | inferenza |
| Testi di policy, eventi, dilemmi | no: espressione letteraria | copyright-office-games |
| Icone, ritratti, layout a cerchi di D4 | no; e il layout complessivo copiato in blocco e' il rischio "look and feel" | Tetris v. Xio |
| Nome "Lawgivers" | no: "Lawgivers is a trademark of Damian Bernardi" | fonti/steam-lawgivers2.md |
| Nome "Democracy" come titolo | titolo non proteggibile da copyright (USA); stato del marchio [NON TROVATO], da evitare comunque | copyright-office-games |

- Il permesso di modding di Positech ("all our data is in text files that you can edit yourself... share your changes with other players") vale dentro il loro gioco (Steam Workshop, cartella mods), non come licenza per riusare i dati altrove [INF] (fonti/steam-democracy4.md, fonti/democracy4-modding.md).
- Le icone dei mod D4 sono SVG fatti dal modder (fonti/democracy4-modding.md): ogni asset nostro va creato da zero o preso con una licenza compatibile.
- AI Democracy (MIT) e' un esempio di clone dell'idea di Democracy con codice e dati propri (fonti/github-aidemocracy.md).

## Per il nostro gioco

- **Raccomandazione** (non decisione): riprendere liberamente le meccaniche documentate in [[policy-e-effetti]]; scrivere da zero formule, valori, testi e icone, con calibrazione da dataset aperti (vedi le pagine dati di altri agenti); niente nomi o layout che richiamino D4 o Lawgivers.
- Tenere nel repo una nota di provenienza per ogni dato di calibrazione (fonte + licenza), sullo stesso schema di `fonti/`. Deciso: i dati stanno in una cartella separata dal codice, ciascuno con la propria licenza ([[decisioni]]).
- **Aperto**: verifica dei marchi (EUIPO/USPTO) prima di scegliere il nome del gioco.

## Collegamenti
- [[soggetti-reali-vs-fittizi]]: l'altro rischio legale (persone e partiti reali).
- [[democracy-4]], [[lawgivers-2]]: i giochi di riferimento.
- [[policy-e-effetti]]: cosa riprendiamo come meccanica.
- [[decisioni]]: codice open source e dati in una cartella separata.
