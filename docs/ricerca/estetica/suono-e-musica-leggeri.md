---
tipo: estetica
fonti: [fonti/minimetro-designingsound-vreeland.md, fonti/minimetro-wikipedia.md, fonti/minimotorways-miketendo-intervista.md, fonti/defcon-escapist-everybody-dies.md, fonti/godot-audiostreaminteractive.md, fonti/freesound-faq-licenze.md, fonti/kenney-support-licenza.md, fonti/jsfxr-github.md, fonti/dorfromantik-gamedeveloper-intervista.md]
aggiornato: 2026-10-16
---
# Suono e musica leggeri

Nei gestionali minimalisti più riusciti il suono non è un sottofondo: è un canale d'informazione e dà il tono. Mini Metro ha vinto un premio per l'audio con un sistema fatto di campioni brevi, attivati dagli eventi e messi a tempo. È anche la via più leggera: pochi secondi di campioni pesano meno di ore di brani registrati. Per gli asset aperti la regola è: CC0 o CC BY sì; NC solo se tenuto fuori dalla licenza del codice (vedi [[decisioni]]).

Tag: [V] fonte primaria letta · [M] misurato · [INF] inferenza · [NON TROVATO].

## Audio procedurale: Mini Metro

- Quasi tutto ciò che si sente sono campioni attivati in tempo reale dalle azioni del giocatore [V] (fonti/minimetro-designingsound-vreeland.md:16).
- Ogni città ha i suoi ritmi, armonie e suoni di motore; ogni linea ha un ritmo e una nota; la forma della rete decide che cosa suona [V] (fonti/minimetro-designingsound-vreeland.md:32).
- Il suono racconta lo stato del sistema: quanti passeggeri, quante linee, quanti treni, quali stazioni sono sovraccariche [V] (fonti/minimetro-designingsound-vreeland.md:86). L'obiettivo era sia informare sia accompagnare, con una qualità "minimale e ipnotica" [V] (fonti/minimetro-designingsound-vreeland.md:28).
- Tecnica: dopo alcuni tentativi falliti con FMOD, scelto G-Audio (plugin Unity), che fa girare un ciclo di aggiornamento a suddivisioni di battuta (quintine, sedicesimi) invece che a 60 fotogrammi al secondo [V] (fonti/minimetro-designingsound-vreeland.md:60).
- Produzione: suoni tonali con un sintetizzatore software; per i suoni di interfaccia centinaia di variazioni casuali di sinusoidi, renderizzate e usate come materiale di partenza [V] (fonti/minimetro-designingsound-vreeland.md:64).
- Riconoscimento: IGF 2016, "Excellence in Audio" [V] (fonti/minimetro-wikipedia.md:97). Mini Motorways riprende la formula: colonna sonora reattiva "che cresce con la città" [V] (fonti/minimotorways-miketendo-intervista.md:103).

## Il tono: DEFCON

- Musica e paesaggio sonoro rendono il gioco "cupo e tragico"; il recensore lo considera la parte migliore [V] (fonti/defcon-escapist-everybody-dies.md:45-47): archi eterei, sintetizzatori da guerra fredda, ambienti inquietanti, un allarme a ogni livello di DEFCON [V] (fonti/defcon-escapist-everybody-dies.md:47).
- Per un gioco con guerra, crisi ed elezioni il suono è il modo più economico di cambiare il tono di una schermata senza ridisegnarla [INF].

## Musica adattiva nel motore

- Godot ha un `AudioStreamInteractive`: clip e una tabella di transizioni; le transizioni possono aspettare la battuta e fare dissolvenze misurate in battute [V] (fonti/godot-audiostreaminteractive.md:13-19, :29). È un esempio di musica a stati (pace, crisi, guerra, elezioni) senza middleware [INF]. La scelta del motore è in [[godot]].
- FMOD e Wwise sono i middleware standard; le pagine di licenza non si sono lasciate catturare (pagina generata in JavaScript) [NON TROVATO]. URL: https://www.fmod.com/licensing. Prima di adottarli vanno lette le soglie di fatturato attuali.

## Asset aperti e licenze

- Freesound: ogni suono ha una licenza scelta dall'autore fra CC0, CC BY e CC BY-NC [V] (fonti/freesound-faq-licenze.md:57-61). Con CC0 si fa quasi tutto; con BY si deve citare l'autore [V] (fonti/freesound-faq-licenze.md:63). In un gioco venduto il BY-NC è escluso [INF]. Il gioco è gratuito ([[decisioni]]): il BY-NC entra solo se resta separato dal codice open source, come i dati NC [INF].
- Kenney: tutti gli asset delle pagine ufficiali sono CC0, usabili anche commercialmente, senza obbligo di attribuzione [V] (fonti/kenney-support-licenza.md:12-16).
- jsfxr: port HTML5 di sfxr per generare effetti sonori, usabile come libreria [V] (fonti/jsfxr-github.md:19-25); licenza Unlicense [M] (`gh api repos/chr15m/jsfxr`, 16/10/2026). Utile per suoni di interfaccia sintetici; in stile "8-bit" va trattato con cura per non sembrare un gioco arcade [INF].
- Il bundle GDC di Sonniss (effetti royalty-free) non si è lasciato catturare (403) [NON TROVATO]. URL: https://sonniss.com/gdc-bundle-license/

## Peso [INF]

Campioni brevi in Ogg Vorbis o Opus pesano decine o centinaia di kB l'uno; un set di campioni per un sistema alla Mini Metro sta in pochi MB. La musica registrata lineare pesa 1-2 MB al minuto. Mixare poche voci per volta è trascurabile per la CPU.

## Per il nostro gioco

- **Prendiamo:** suono legato agli stati della simulazione (tensione sociale, guerra, crescita) e agli eventi del turno; esiti del turno a tempo, come l'"Arpeggio" di Mini Metro (vedi [[movimento-e-feedback]]); un tono deciso per le crisi, alla DEFCON.
- **Evitiamo:** colonna sonora epica orchestrale in loop (pesa e stanca); asset NC mescolati al codice; suoni "giocosi" su eventi gravi, la stessa regola di Dorfromantik sull'atmosfera (fonti/dorfromantik-gamedeveloper-intervista.md:71).
- **Aperto:** middleware o audio del motore (tecnica); chi compone (un sistema procedurale chiede un compositore che sappia anche progettare regole, come Disasterpeace).

## Riferimenti

- **URL** https://designingsound.org/2016/02/18/the-programmed-music-of-mini-metro-interview-with-rich-vreeland-disasterpeace/ · [V]
  **perché conta qui** l'unica descrizione primaria di un sistema audio procedurale in un gestionale minimalista
  **cosa se ne prende** campioni brevi a tempo, suono come stato del sistema, strumenti usati
- **URL** https://en.wikipedia.org/wiki/Mini_Metro_(video_game) · [V]
  **perché conta qui** riconoscimento del risultato
  **cosa se ne prende** IGF Excellence in Audio 2016
- **URL** https://miketendo64.com/2022/08/14/interview-mini-motorways/ · [V]
  **perché conta qui** la formula riusata in un secondo gioco
  **cosa se ne prende** colonna sonora che cresce con il sistema
- **URL** https://www.escapistmagazine.com/everybody-dies-in-defcon/ · [V]
  **perché conta qui** tono tragico ottenuto con il suono in un gioco di mappa militare
  **cosa se ne prende** archi, sintetizzatori, allarmi come segnali di stato
- **URL** https://docs.godotengine.org/en/stable/classes/class_audiostreaminteractive.html · [V]
  **perché conta qui** musica a stati senza middleware, se il motore è Godot
  **cosa se ne prende** clip + tabella di transizioni a battuta
- **URL** https://freesound.org/help/faq/ · [V]
  **perché conta qui** licenze dell'archivio di suoni aperto più grande
  **cosa se ne prende** CC0/BY sì, BY-NC no
- **URL** https://kenney.nl/support · [V]
  **perché conta qui** asset CC0 pronti all'uso
  **cosa se ne prende** suoni di interfaccia senza obblighi
- **URL** https://github.com/chr15m/jsfxr · [V]
  **perché conta qui** generatore di effetti sonori libero
  **cosa se ne prende** suoni sintetici per prototipi e interfaccia
- **URL** https://www.fmod.com/licensing · [NON TROVATO]
  **perché conta qui** licenza del middleware audio più usato
  **cosa se ne prende** da recuperare a mano prima di decidere

## Collegamenti

- [[movimento-e-feedback]]: ritmo dell'esito del turno.
- [[riferimenti-minimalisti]]: Mini Metro e DEFCON per intero.
- [[direzioni-artistiche-candidate]]: quale suono per quale direzione.
- [[godot]]: motore candidato (altro agente).
