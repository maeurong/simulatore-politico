---
tipo: gioco
fonti: [fonti/github-ricerca-progetti-politici.md, fonti/github-aidemocracy.md]
aggiornato: 2026-10-16
---
# Progetti open source di simulazione politica

Non esiste un progetto open source maturo di simulazione di governo da cui partire o da forkare. La ricerca su GitHub restituisce progetti con 0-17 stelle, spesso senza licenza. Per noi questo vuol dire che un repo pubblico ben fatto avrebbe poca concorrenza diretta, e che il codice va scritto da zero.

## Ricerca (16/10/2026)

Comando [M]: `gh search repos "<query>" --sort stars --limit 6` per cinque query ("political simulation game", "democracy simulator game", "election simulation game", "parliament simulator", "government simulation game") (fonti/github-ricerca-progetti-politici.md).

- Il risultato con piu' stelle e' WJNCT55555/Liberty-Unquenched (17 stelle, MIT): sim politico storico sulla Seconda Repubblica spagnola (fonti/github-ricerca-progetti-politici.md).
- eskalon/ProjektGG: 12 stelle, Apache-2.0, sim economico-politico medievale, attivo a ottobre 2026 (fonti/github-ricerca-progetti-politici.md).
- "democracy simulator game": nessun risultato (fonti/github-ricerca-progetti-politici.md).
- Gli altri risultati sono progetti scolastici o prototipi, molti senza licenza (cioe' non riutilizzabili) (fonti/github-ricerca-progetti-politici.md).

## AI Democracy (cosmin-novac/aidemocracy)

- Clone web di [[democracy-4]]: nodi di stato, nodi policy e nodi elettore collegati da linee verdi/rosse con spessore pari alla forza dell'effetto; policy aggiunte e rimosse spendendo crediti politici (fonti/github-aidemocracy.md).
- Le nuove policy e i loro effetti sono generati da GPT: serve una chiave OpenAI, quindi richiede la rete; e' stato scritto "without ANY manual coding" (fonti/github-aidemocracy.md).
- Licenza MIT, 2 stelle, JavaScript, ultimo push giugno 2026 [M] (fonti/github-ricerca-progetti-politici.md).

## Per il nostro gioco

- Niente da riusare come base. La visualizzazione a grafo di AI Democracy (spessore e colore degli archi) e' un'idea UI da tenere presente.
- Effetti generati da LLM: incompatibili con il vincolo offline e con la verosimiglianza (nessuna calibrazione).

## Collegamenti
- [[democracy-4]]: il modello imitato.
- [[policy-e-effetti]]: il grafo di effetti.
- [[titoli-politici-minori]].
