---
tipo: sistema
fonti: [fonti/intothebreach-wiki-reset-turn.md, fonti/intothebreach-steam-reset-at-will.md, fonti/intothebreach-steam-reset-at-will-p2.md, fonti/intothebreach-v1-2-82.md, fonti/xcom-wiki-ironman.md, fonti/xcom-wiki-second-wave.md, fonti/xcom-wiki-random-vs-seed.md, fonti/fireemblem-wiki-turn-rewind.md, fonti/slaythespire-gist-reroll-salvataggio.md, fonti/slaythespire-neowsletter-2025-10.md, fonti/slaythespire2-v0-100-0.md, fonti/slaythespire2-v0-107-1.md, fonti/slaythespire2-v0-108-0.md, fonti/ftl-wiki-game-patches.md, fonti/civ7-update-1-1-1.md, fonti/civ7-update-1-2-0.md, fonti/civ7-patch-notes-2025-03-04.md, fonti/civ7-steam-autosave-cancellati.md, fonti/civ6-wiki-hall-of-fame.md, fonti/civ-wiki-hall-of-fame.md, fonti/civ6-wiki-update-2019-02.md, fonti/civ4-civfanatics-preview-hall-of-fame.md, fonti/civ4-apolyton-hall-of-fame.md, fonti/frostpunk2-patch-1-1-0.md, fonti/frostpunk-patch-1-6-2.md, fonti/stellaris-4-3-1-hotfix.md, fonti/stellaris-dd413-version-mismatch.md, fonti/stellaris-4-5-cygnus-rilascio.md, fonti/factorio-forum-map-version-higher.md, fonti/ck3-update-1-16-0-1.md, fonti/democracy4-build-1-130.md, fonti/democracy4-2023-02-18.md, fonti/democracy4-build-1-20.md]
aggiornato: 2026-10-29
---
# Salvataggi e riavvolgimento

Come i giochi a turni salvano da soli, tornano a un turno passato, riprendono a metà turno, elencano più partite, conservano le partite finite e rifiutano i file di un'altra versione. Serve al piano 10 per eseguire le decisioni già prese nel ticket 28 (`.scratch/v1-giocabile/issues/28-salvataggi.md`) e nella spec (`docs/superpowers/specs/2026-10-21-v1-giocabile.md:391-392`), che questa pagina non rimette in discussione. Contenuto dello snapshot: [[architettura-simulazione]] riga 32.

Tag: [V] fonte primaria letta e catturata · [M] misurato in sessione · [INF] inferenza · [NON TROVATO]. Le discussioni dei giocatori (Steam, forum) valgono solo come segnale di lamentela e sono dichiarate tali.

## 1. Riavvolgimento senza slot

- **Into the Breach.** Una volta per battaglia si riavvolge il turno: le unità tornano dove erano e possono agire di nuovo; un pilota (Isaac Jones) ne dà un secondo (fonti/intothebreach-wiki-reset-turn.md:7-11) [V]. Il riavvolgimento copre solo il turno in corso, non quelli già chiusi [V, stessa fonte].
- **Fire Emblem, dal 2017 in ogni capitolo** (Mila's Turnwheel, Divine Pulse, Time Crystal): si torna a un punto qualsiasi dei turni precedenti, fino a 50 turni indietro in *Three Houses*, con cariche limitate (fonti/fireemblem-wiki-turn-rewind.md:13-33) [V]. In *Shadow Dragon* (riedizione del trentennale) lo fa l'emulatore: **uno stato all'inizio di ogni turno, se ne carica uno e quelli più recenti si cancellano** (fonti/fireemblem-wiki-turn-rewind.md:41) [V]. È il nostro modello quasi alla lettera, salvo che lì gli stati sono 19 e i più vecchi si scartano.
- **Civilization VII.** Autosalvataggi a rotazione: l'impostazione «Autosaves Amount» arriva fino a 100 dalla 1.2.0 (fonti/civ7-update-1-2-0.md:240) [V]; un bug della 1.0 smetteva di crearli superato un certo limite (fonti/civ7-patch-notes-2025-03-04.md:343-349) [V]; dalla 1.1.1 «Continua» carica il salvataggio manuale se è più recente dell'ultimo automatico (fonti/civ7-update-1-1-1.md:288) [V]. Segnale dei giocatori: al passaggio d'età gli autosalvataggi dell'età precedente spariscono e chi vuole tornare indietro di poco non può («Why? That is such a strange patronizing design choice», fonti/civ7-steam-autosave-cancellati.md:18-19) [forum].
- **Civilization VI.** Il default di 10 autosalvataggi, uno per turno, viene da discussioni di giocatori, non da una fonte ufficiale [NON TROVATO come fonte primaria].
- **Paradox, ironman.** In EU4: salvataggio automatico ogni tre mesi di gioco e all'uscita, un solo file più una copia di riserva, niente caricamento a partita in corso. Letto su https://eu4.paradoxwikis.com/Ironman con WebFetch, **non catturato** perché il sito risponde con una pagina anti-bot anche nelle copie della Wayback Machine. XCOM fa lo stesso: un solo file a salvataggio automatico, «Save and Quit» al posto di «Salva» e «Carica» (fonti/xcom-wiki-ironman.md:7) [V]. Varianti nate dai giocatori: l'«honestman», cioè la disciplina dell'ironman senza la modalità attiva, per paura di file corrotti e bug; il «bronzeman», che vieta i salvataggi durante la missione e la lascia ricominciare dall'inizio (fonti/xcom-wiki-ironman.md:13-14) [V]. Il bronzeman è la forma più vicina al nostro riavvolgimento a inizio turno.
- **Frostpunk 2.** Dalla 1.1.0 l'autosalvataggio si spegne o si allunga a 50, 75, 100 o 150 settimane (fonti/frostpunk2-patch-1-1-0.md:143-144) [V]. Una funzione «torna a un punto» [NON TROVATO]: si ricarica un salvataggio.
- **Come si presenta la cancellazione del futuro.** Nessuna fonte primaria descrive il testo o la conferma che un gioco mostra prima di cancellare i turni successivi [NON TROVATO]. Nell'emulatore di Fire Emblem la cancellazione avviene in silenzio (fonti/fireemblem-wiki-turn-rewind.md:41) [V]; in *Three Houses* e in *Engage* il riavvolgimento si offre da solo al game over, e il giocatore può rifiutarlo (fonti/fireemblem-wiki-turn-rewind.md:35, 51) [V].

## 2. Stesso caso dopo il riavvolgimento

- **XCOM: Enemy Unknown e XCOM 2.** Seme e indice del generatore stanno nel salvataggio, «per vincolare i giocatori alle loro scelte»: **la stessa azione dà lo stesso esito, un'azione diversa ne dà uno diverso**, perché ogni azione fa avanzare la sequenza (fonti/xcom-wiki-random-vs-seed.md:31, 42) [V]. Il seme fisso vale anche per XCOM 2 (fonti/xcom-wiki-ironman.md:15) [V]. Restano fuori dal seme alcuni sorteggi, che un caricamento cambia: mappa della missione, richieste dei paesi, reclute (fonti/xcom-wiki-random-vs-seed.md:57-59) [V].
- **Correzione al brief:** l'opzione «Save Scum», che azzera il seme a ogni caricamento, è di *XCOM: Enemy Within* (Second Wave, 2013), non di XCOM 2 (fonti/xcom-wiki-second-wave.md:76; fonti/xcom-wiki-ironman.md:15) [V]. Firaxis l'ha resa facoltativa: il default è il seme fisso.
- **Fire Emblem.** Il riavvolgimento di norma non cambia la sequenza del generatore: stesse azioni, stessi esiti (fonti/fireemblem-wiki-turn-rewind.md:19, 37) [V].
- **Into the Breach.** La difesa della rete elettrica è un tiro casuale «incorporato»: dopo un reset il giocatore sa già se scatterà, e c'è chi colpisce i propri edifici per scoprirlo e poi resetta (fonti/intothebreach-steam-reset-at-will-p2.md:30) [forum]. Un altro giocatore osserva che per lasciare intatta la difficoltà con reset illimitati servirebbe «tornare all'ultimo evento casuale» (fonti/intothebreach-steam-reset-at-will-p2.md:128) [forum]. Chi chiede reset illimitati riceve risposte che difendono il limite come fonte di attenzione (fonti/intothebreach-steam-reset-at-will.md:12, 141) [forum]. **Riavvolgere compra conoscenza, non fortuna: è proprio l'effetto descritto nel ticket 28.**
- **Slay the Spire.** Un generatore per sottosistema (`cardRng`, `potionRng`, `eventRng`…), tutti dal seme della run; il salvataggio tiene per ciascuno un **contatore** e lo stato si ricostruisce ripetendo il generatore quel numero di volte (fonti/slaythespire-gist-reroll-salvataggio.md:49) [V, analisi del codice decompilato]. I bug noti nascono da stato **non salvato**: la stanza corrente, le iscrizioni già viste nell'atto, un contatore scritto sbagliato. Dopo un salva-ed-esci la stanza «?» si risolve in modo diverso (fonti/slaythespire-gist-reroll-salvataggio.md:11, 14-41) [V]. Stessa famiglia in *Slay the Spire 2*: una reliquia predeterminata ridiventava casuale dopo salva-ed-esci (fonti/slaythespire2-v0-100-0.md:327) [V].
- **Cosa dicono gli sviluppatori.** Mega Crit sul ricaricare l'inizio del piano: «Yeah you can still save scum. Play how you want» (fonti/slaythespire-neowsletter-2025-10.md:46-47) [V]. *Slay the Spire 2* conta i ricaricamenti nel salvataggio (`SerializableSave.NumReloads`, fonti/slaythespire2-v0-108-0.md:298) [V]: un precedente del nostro «riavvolta N volte».

## 3. Ripresa a metà turno e chiusura brusca

- **FTL, Advanced Edition (1.5.4).** «Save & Quit anywhere: mid-battle or even mid-event»; anche chiudere la finestra salva. Dopo un crash si riprende «di solito un salto prima» (fonti/ftl-wiki-game-patches.md:171-172) [V]. Seguono le correzioni: un evento salvato male che ricaricato dava un altro esito (riga 137); uscire durante un salto lasciava un file inaffidabile (riga 142); il blocco riprendendo un salvataggio fatto a metà salto (riga 77); salva-e-continua usato per incassare due volte le ricompense (riga 50); un profilo azzerato dopo una schermata blu (riga 37) [V].
- **Slay the Spire.** Si salva all'ingresso della stanza: chi esce a metà combattimento riprende dall'inizio del piano, con lo stesso caso (fonti/slaythespire-neowsletter-2025-10.md:46; fonti/slaythespire-gist-reroll-salvataggio.md:54) [V]. Le scelte fatte e non ancora chiuse dentro il combattimento si perdono; il combattimento riparte uguale [INF dalle due fonti].
- **Slay the Spire 2, 2026.** «Fixed save files being lost or corrupted after a PC crash, blue screen, or power outage» e operazioni di salvataggio rese «fault-tolerant» (fonti/slaythespire2-v0-107-1.md:359-360) [V]; ritenta se un antivirus o la sincronizzazione nel cloud bloccano un attimo il file (fonti/slaythespire2-v0-108-0.md:180) [V]. Into the Breach 1.2.82: «Improved save file backup system to prevent data loss» (fonti/intothebreach-v1-2-82.md:10) [V]. Democracy 4: un salvataggio fallito e corrotto mandava in crash i salvataggi successivi (fonti/democracy4-2023-02-18.md:15) [V].
- **Cosa resta della bozza dopo un crash.** Nessuna fonte dice se un gioco conserva le scelte non risolte dopo una chiusura brusca. FTL e Slay the Spire ripiegano sull'ultimo punto stabile (un salto, l'ingresso della stanza): la bozza si perde e il turno resta [INF].

## 4. Schermata delle partite in corso

- **Campi documentati.** Democracy 4 mostra data e ora del salvataggio: una correzione della 1.20 ne sistema il formato a due cifre (fonti/democracy4-build-1-20.md:16) [V]. Stellaris segna con un'icona i salvataggi incompatibili direttamente nell'elenco (fonti/stellaris-4-3-1-hotfix.md:13) [V]. Civ VI tiene nella cronologia dell'albo leader, civiltà, vittoria o sconfitta, velocità, dimensione della mappa, numero di turni, era di partenza e «last day and time you played» (fonti/civ6-wiki-hall-of-fame.md:17-19) [V].
- **Miniature e campi della schermata di carico** di Civ, CK3 e Frostpunk: [NON TROVATO] in fonti primarie.
- **L'elenco legge tutti i file della cartella, anche quelli che non apre.** In CK3 1.16.0.1 un salvataggio di una versione vecchia nella cartella mandava in crash il gioco (fonti/ck3-update-1-16-0-1.md:14) [V]. In Democracy 4 bastava un salvataggio di un paese moddato poi disinstallato per mandare in crash la schermata di carico (fonti/democracy4-build-1-130.md:20) [V]. Civ VII apre un riquadro con il contenuto mancante invece di caricare a metà (fonti/civ7-update-1-2-0.md:180) [V].

## 5. Albo delle partite concluse

- **Civilization IV.** La Hall of Fame conserva il replay di ogni partita, con difficoltà, velocità, vittoria, era di partenza, dimensione del mondo, data di fine e punteggio (fonti/civ4-civfanatics-preview-hall-of-fame.md:20) [V]. **Non sopravvive alle espansioni**: passando a *Warlords* l'albo riparte da zero, e i giocatori lo ricompongono spostando i replay e rinominandone l'estensione (fonti/civ4-apolyton-hall-of-fame.md:42, 98-102) [forum].
- **Civilization VI.** La Hall of Fame arriva con l'aggiornamento di febbraio 2019 («victory and defeat tracking with detailed statistics and graphs», fonti/civ6-wiki-update-2019-02.md:39) [V]. Ha due sezioni: totali per vittoria e per leader, e la cronologia delle ultime partite, separate per regolamento, scenario e sfida (fonti/civ6-wiki-hall-of-fame.md:11-19) [V]. Un file a sé, da copiare per conservarlo (fonti/civ-wiki-hall-of-fame.md:7) [V].
- **Civilization VII** non ha una Hall of Fame (fonti/civ-wiki-hall-of-fame.md:42-48) [V].
- **Democracy 4, Frostpunk:** un albo o uno storico delle partite finite [NON TROVATO].

## 6. Salvataggi di versione incompatibile

- **Factorio.** Il messaggio nomina entrambe le versioni: «Map version (1.1.71-0) cannot be loaded because it is higher than the game version (1.1.70-1)». Il file resta intatto: con la versione giusta si carica (fonti/factorio-forum-map-version-higher.md:20, 67) [V, risposta dello staff].
- **Stellaris 4.3.1.** Il tasto «Resume» del launcher caricava salvataggi incompatibili «potentially ruining them», ironman compresi: ora porta al menu con un errore «Version Mismatch» che invita a tornare alla versione giusta (fonti/stellaris-4-3-1-hotfix.md:13; fonti/stellaris-dd413-version-mismatch.md:10) [V]. Stellaris 4.5: «Once a save has been opened in 4.5, it can't be opened again in 4.4. Roll back first, then play», con l'invito a copiare la cartella prima (fonti/stellaris-4-5-cygnus-rilascio.md:28-37) [V].
- **Frostpunk 1.6.2.** Un edificio nuovo vale solo nelle partite nuove, «not available on previous saves» (fonti/frostpunk-patch-1-6-2.md:28) [V]: la stessa regola che il ticket 28 applica alle modifiche ai dati.

## Per il nostro gioco

Le decisioni del ticket 28 reggono contro tutti i precedenti. Le cose che li precisano:

1. **Il riavvolgimento ha un precedente esatto.** Lo stato all'inizio di ogni turno, la cancellazione dei più recenti al caricamento (emulatore di Fire Emblem), più la regola di XCOM per cui stessa azione dà stesso esito. Prendiamo il modello; evitiamo il limite a 19 stati e la rotazione di Civ, che toglie proprio il punto a cui il giocatore vuole tornare (fonti/civ7-steam-autosave-cancellati.md:18) [forum]. La conferma che dice quanti turni si cancellano non ha precedenti documentati [NON TROVATO]: la scriviamo noi, con il turno di arrivo e il numero di turni persi [INF].
2. **Un flusso del generatore per sottosistema, nello snapshot.** Con un flusso solo, un decreto diverso sposta tutti i tiri successivi: i franchi tiratori di un voto che il giocatore non ha toccato cambierebbero dopo il riavvolgimento. XCOM lo accetta («un'azione diversa dà un esito diverso»); Slay the Spire lo evita con un generatore per sottosistema (fonti/slaythespire-gist-reroll-salvataggio.md:49) [V]. Per noi va scelto in modo esplicito e testato. [[architettura-simulazione]]:22 lo suggerisce già [INF].
3. **Test di completezza dello snapshot.** `salva()` rilegge e confronta (`crates/sim-core/src/partita.rs:3141-3156`), ma non vede lo stato che vive fuori da `Partita` o che serde salta. È proprio la classe di bug di Slay the Spire e FTL: stanza corrente, iscrizioni viste, evento salvato male. Proposta: un test che gioca N turni di fila e li rigioca salvando e ricaricando a ogni apertura, e pretende la stessa partita, bit per bit [INF].
4. **La bozza si scrive a ogni scelta, non solo alla chiusura.** FTL salva anche quando si chiude la finestra; dopo un crash ripiega un passo indietro (fonti/ftl-wiki-game-patches.md:171-172) [V]. Slay the Spire 2 ha dovuto correggere file persi dopo crash e blocchi da antivirus (fonti/slaythespire2-v0-107-1.md:359; fonti/slaythespire2-v0-108-0.md:180) [V]. Per noi: bozza e snapshot in file separati, scrittura atomica (file temporaneo e poi rinomina), lo snapshot dell'apertura mai riscritto. Un crash costa al massimo l'ultima scelta; il turno resta [INF]. Il ticket non parla del crash: va scritto nel piano.
5. **Elenco e «Continua» leggono una testata, non la partita.** Il CK3 e il Democracy 4 delle fonti andavano in crash per un solo file vecchio o orfano, e Stellaris rovinava gli ironman caricandoli da «Resume» (sezioni 4 e 6). Proposta: ogni file di partita espone in testa, in forma stabile fra gli schemi, nome, scenario, turno, data dell'ultimo salvataggio e versione dello schema. L'elenco mostra anche i file d'altro schema, con un segno e il messaggio, senza caricarli; «Continua» controlla lo schema prima di aprire. Un file rifiutato non si riscrive mai, e nemmeno ci si salva sopra [INF]. Il messaggio nomina entrambe le versioni, come Factorio: `Errore::SchemaDiverso { trovata, attesa }` (`partita.rs:3165-3180`) ha già i due numeri. Un file senza il campo risulta schema 0 (`unwrap_or(0)`, `partita.rs:3172`): il messaggio deve distinguere «file illeggibile» da «schema 0» [INF].
6. **L'albo deve essere elencabile da solo.** Il ticket ci mette eredità, serie dei tre indicatori, elezioni e riavvolgimenti. `Eredita` (`crates/sim-core/src/legislatura/eredita.rs:17-25`) non contiene nome, scenario, date né numero di turni, che Civ VI mostra in cronologia (fonti/civ6-wiki-hall-of-fame.md:17-19) [V]. Vanno nel file dell'albo, con una sua versione separata da `VERSIONE_SCHEMA`, campi solo aggiunti e mai tolti. È la lezione di Civ IV, il cui albo si azzerava a ogni espansione (fonti/civ4-apolyton-hall-of-fame.md:42) [forum]. Civ VII l'ha tolto del tutto (fonti/civ-wiki-hall-of-fame.md:48) [V].
7. **Il contatore senza penalità ha un precedente.** `NumReloads` in Slay the Spire 2 e il «play how you want» di Mega Crit (sezione 2) [V].

Evitiamo: lo slot unico con una sola copia di riserva (ironman EU4); il seme rimescolato al caricamento (Save Scum di Enemy Within), che il ticket esclude; il «Continua» che carica alla cieca.

Aperto: la conferma del riavvolgimento e l'aspetto dell'elenco (miniatura sì o no) restano scelte di UI senza precedenti trovati.

## Collegamenti

- [[persistenza-dei-salvataggi]]: come scrivere i file (scrittura atomica, gzip, cartella dati, chiusura dell'app); pagina di un altro ricercatore del piano 10.
- [[architettura-simulazione]]: contenuto dello snapshot, replay solo a parità di binario, decisione del 20/10.
- [[decisioni]]: registro delle scelte del ticket 28.
- [[tempo-e-ritmo]]: cadenza dei turni, quindi quanti snapshot per partita.
- [[democracy-4]], [[frostpunk]], [[crusader-kings-3]], [[stellaris]]: le pagine dei titoli citati.
- [[modding-e-dati-esterni]]: dati copiati nel salvataggio, modifiche solo per le partite nuove.

## Riferimenti

- **URL** https://intothebreach.fandom.com/wiki/Reset_Turn · [V]
  **perche' conta qui** riavvolgimento di un turno senza slot, con limite
  **cosa se ne prende** reset a inizio turno, una volta per battaglia; il nostro è illimitato ma a turni chiusi
- **URL** https://steamcommunity.com/app/590380/discussions/0/2650805212053351822/ · [forum, segnale]
  **perche' conta qui** giocatori che chiedono reset illimitati e chi difende il limite
  **cosa se ne prende** il limite visto come fonte di attenzione; nessuna risposta dello sviluppatore
- **URL** https://steamcommunity.com/app/590380/discussions/0/2650805212053351822/?ctp=2 · [forum, segnale]
  **perche' conta qui** il tiro della difesa della rete resta uguale dopo il reset
  **cosa se ne prende** riavvolgere compra conoscenza del caso: conferma l'effetto atteso nel ticket 28
- **URL** https://store.steampowered.com/news/app/590380/view/4621277624709592573 · [V]
  **perche' conta qui** Into the Breach 1.2.82 rafforza le copie di riserva
  **cosa se ne prende** la perdita del file è un rischio reale anche in giochi maturi
- **URL** https://xcom.fandom.com/wiki/Ironman · [V]
  **perche' conta qui** ironman, honestman, bronzeman e seme fisso in XCOM e XCOM 2
  **cosa se ne prende** il bronzeman (ripartire dall'inizio missione) è il parente del nostro riavvolgimento
- **URL** https://xcom.fandom.com/wiki/Second_Wave · [V]
  **perche' conta qui** l'opzione Save Scum, in Enemy Within, rimescola il seme al caricamento
  **cosa se ne prende** correzione del brief: non è di XCOM 2; il default resta il seme fisso
- **URL** https://xcom.fandom.com/wiki/Random_vs_Seed_(XCOM:_Enemy_Unknown) · [V]
  **perche' conta qui** seme e indice nel salvataggio; cosa resta fuori dal seme
  **cosa se ne prende** stessa azione stesso esito, azione diversa esito diverso: decidere i flussi per sottosistema
- **URL** https://fireemblemwiki.org/wiki/Turn_rewind · [V]
  **perche' conta qui** riavvolgimento a turni passati in quattro giochi, compreso lo stato a ogni inizio turno
  **cosa se ne prende** il modello «uno stato per turno, i successivi si cancellano», RNG invariato
- **URL** https://gist.github.com/luckytyphlosion/e1c1fd8b445e7df406ad4bc480f9dcdd · [V, analisi del codice]
  **perche' conta qui** generatori per sottosistema con contatore nel salvataggio, e i bug dello stato non salvato
  **cosa se ne prende** flussi separati e test salva-ricarica contro partita continua
- **URL** https://store.steampowered.com/news/app/646570/view/1813674388948758 · [V]
  **perche' conta qui** lo sviluppatore sul save scumming
  **cosa se ne prende** nessuna penalità per chi ricarica: coerente con «riavvolta N volte» senza costo
- **URL** https://store.steampowered.com/news/app/2868840/view/1827626365756213 · [V]
  **perche' conta qui** Slay the Spire 2 v0.100: esito che cambia dopo salva-ed-esci
  **cosa se ne prende** un altro caso di stato non salvato; serve il test di completezza
- **URL** https://store.steampowered.com/news/app/2868840/view/1835871199305790 · [V]
  **perche' conta qui** Slay the Spire 2 v0.107.1: file persi dopo crash e interruzioni di corrente
  **cosa se ne prende** scrittura atomica e tollerante agli errori della bozza e dello snapshot
- **URL** https://store.steampowered.com/news/app/2868840/view/1836506165569491 · [V]
  **perche' conta qui** Slay the Spire 2 v0.108: ritenta se antivirus o cloud bloccano il file; `NumReloads` nel salvataggio
  **cosa se ne prende** ritentare la scrittura; il contatore dei ricaricamenti ha un precedente
- **URL** https://ftl.fandom.com/wiki/Game_patches · [V]
  **perche' conta qui** salva-ed-esci ovunque, salvataggio alla chiusura della finestra, ripresa dopo crash
  **cosa se ne prende** bozza scritta a ogni scelta; dopo un crash si ripiega sull'ultimo punto stabile
- **URL** https://civilization.2k.com/civ-vii/game-update-notes/2025-mar-25-patch-1-1-1/ · [V]
  **perche' conta qui** «Continua» carica il salvataggio più recente, manuale o automatico
  **cosa se ne prende** «Continua» va alla partita con il salvataggio più recente
- **URL** https://store.steampowered.com/news/app/1295660/view/1797185861800349 · [V]
  **perche' conta qui** Civ VII 1.2.0: autosalvataggi fino a 100; riquadro sul contenuto mancante
  **cosa se ne prende** ordine di grandezza dei salvataggi tenuti; rifiutare con un messaggio invece di caricare a metà
- **URL** https://support.civilization.com/hc/en-us/articles/39070074668691-Civilization-VII-Patch-Notes-March-4-2025 · [V]
  **perche' conta qui** autosalvataggi che smettono di crearsi oltre un limite
  **cosa se ne prende** un tetto ai file salvati è un punto di rottura da testare
- **URL** https://steamcommunity.com/app/1295660/discussions/0/570371162116255204/ · [forum, segnale]
  **perche' conta qui** autosalvataggi cancellati al cambio d'età
  **cosa se ne prende** conservare tutti gli snapshot della partita, come deciso
- **URL** https://civilization.fandom.com/wiki/Hall_of_Fame_(Civ6) · [V]
  **perche' conta qui** campi della cronologia e dei totali dell'albo di Civ VI
  **cosa se ne prende** nome, scenario, date e turni nel file dell'albo
- **URL** https://civilization.fandom.com/wiki/Hall_of_Fame · [V]
  **perche' conta qui** albo come file a sé nella serie; assente in Civ VII
  **cosa se ne prende** albo in file separati dai salvataggi
- **URL** https://civilization.fandom.com/wiki/Civilization_VI_February_2019_Update · [V]
  **perche' conta qui** data d'introduzione dell'albo di Civ VI, con statistiche e grafici
  **cosa se ne prende** la serie per turno degli indicatori ha un precedente nei grafici dell'albo
- **URL** https://civfanatics.com/civ4/preview/filling_the_gap.php · [V, anteprima del 2005]
  **perche' conta qui** campi dell'albo di Civ IV e replay conservati
  **cosa se ne prende** l'elenco dei campi per una partita finita
- **URL** https://apolyton.net/forum/civilization-iv/civilization-iv-general/200416-hall-of-fame?p=6730729 · [forum, segnale]
  **perche' conta qui** albo azzerato al passaggio a un'espansione
  **cosa se ne prende** l'albo ha una versione sua e campi solo aggiunti
- **URL** https://store.steampowered.com/news/app/1601580/view/6212244583190849562 · [V]
  **perche' conta qui** Frostpunk 2: autosalvataggio regolabile, nessun «torna a un punto»
  **cosa se ne prende** solo un caso negativo per il riavvolgimento
- **URL** https://store.steampowered.com/news/app/323190/view/4613399405957636887 · [V]
  **perche' conta qui** contenuto nuovo solo nelle partite nuove
  **cosa se ne prende** conferma la regola del ticket 28 sui dati copiati nel salvataggio
- **URL** https://store.steampowered.com/news/app/281990/view/1827626365753371 · [V]
  **perche' conta qui** Stellaris 4.3.1: «Resume» non carica più salvataggi incompatibili; icona nell'elenco
  **cosa se ne prende** «Continua» controlla lo schema; l'elenco segna i file incompatibili
- **URL** https://store.steampowered.com/news/app/281990/view/1827626365770287 · [V]
  **perche' conta qui** errore «Version Mismatch» e salvataggi ironman rovinati
  **cosa se ne prende** il testo del rifiuto spiega cosa fare; il file non si tocca
- **URL** https://store.steampowered.com/news/app/281990/view/1844115010503624 · [V]
  **perche' conta qui** un salvataggio aperto nella versione nuova non si riapre in quella vecchia
  **cosa se ne prende** mai riscrivere un file d'altro schema
- **URL** https://forums.factorio.com/viewtopic.php?p=576959 · [V, risposta dello staff]
  **perche' conta qui** testo esatto del rifiuto, con le due versioni
  **cosa se ne prende** il formato del messaggio: versione trovata e versione attesa
- **URL** https://store.steampowered.com/news/app/1158310/view/1797820624627359 · [V]
  **perche' conta qui** CK3 in crash per un salvataggio vecchio nella cartella
  **cosa se ne prende** l'elenco legge solo una testata stabile e regge i file d'altro schema
- **URL** https://store.steampowered.com/news/app/1410710/view/3884878205210296241 · [V]
  **perche' conta qui** Democracy 4: schermata di carico in crash per un salvataggio orfano di mod
  **cosa se ne prende** l'elenco non crolla mai per un file
- **URL** https://store.steampowered.com/news/app/1410710/view/5059268099243682927 · [V]
  **perche' conta qui** un salvataggio fallito rompeva i successivi
  **cosa se ne prende** un file per partita, scrittura atomica
- **URL** https://store.steampowered.com/news/app/1410710/view/4483864038816780470 · [V]
  **perche' conta qui** data e ora mostrate nell'elenco dei salvataggi
  **cosa se ne prende** «data dell'ultimo salvataggio» fra i campi dell'elenco, con formato fisso
- **URL** https://eu4.paradoxwikis.com/Ironman · [V, letto con WebFetch, non catturato]
  **perche' conta qui** ironman di EU4: salvataggio ogni tre mesi e all'uscita, un file più una copia
  **cosa se ne prende** il modello scartato dal ticket 28 (solo avanti), descritto per contrasto
