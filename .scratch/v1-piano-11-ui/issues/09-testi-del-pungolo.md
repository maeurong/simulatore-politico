Type: grilling
Status: resolved
Blocked by: 01, 03, esterno: spec del piano di core «esito e Vista completa»
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Da dove vengono occhiello serio e titolo spiritoso di ogni voce dell'esito, i motti dei ministri e le battute (storie 153-154, 159)? Catalogo di testi per tipo di voce con varianti a seme fisso, chi li scrive e quanti ne servono per la prima fetta, cosa succede a una voce senza testo, regola del tono grave (deciso dal modello) che li fa tacere, scherzo mai nei numeri.

Dal 10/11/2026 ([Dati senza piano](11-dati-senza-piano.md)): l'elenco dei tipi di voce dell'esito nasce nella spec del piano di core «esito e Vista completa». Questo ticket si prende quando quella spec esiste.

## Answer

Deciso con Mario l'11/11/2026 in tre giri. Parte dal catalogo dei 92 tipi di voce della spec di core «esito e Vista completa» (`docs/superpowers/specs/2026-11-11-esito-e-vista-completa.md`, sul branch `docs/piano-esito-vista-completa`, PR #50), dai testi del prototipo vincolante (`.scratch/v1-giocabile/prototipi/10-direzione-artistica-2.prototype.html:505-511`) e da [tono-giocoso-su-temi-seri](../../../docs/ricerca/estetica/tono-giocoso-su-temi-seri.md).

### Scelte di Mario

- **Titoli senza nomi.** Il titolo è generico per tipo di voce: niente ministri, partiti, dicasteri o regioni. Il nome sta già nell'occhiello, che è il `fatto` scritto dal core. Così non c'è grammatica da gestire (articoli delle regioni, nomi di partito scelti dal giocatore) e il titolo vale anche per le voci fuse.
- **Quanti: per frequenza.** Quattro varianti per la dozzina di tipi che tornano più spesso, due per gli altri che possono finire in evidenza, nessuna per i tipi sempre ordinari. Il registro resta neutro: fatto e causa, senza titoli.
- **Motti dal dicastero.** Quattro motti per dicastero, quaranta in tutto: lo scherzo è sul mestiere. Il ministro prende il suo con un hash del nome e lo tiene per tutta la partita; il successore ne ha un altro. È lo stesso meccanismo della [caricatura](05-caricature.md).
- **Il corsivo è tagliato dalla v1.** La riga che commenta il trimestre andrebbe composta da più fatti: o la scrive il core, o la UI si mette a fare regole. Nessuna storia la chiede.
- **Eventi titolati per famiglia.** `evento_nato` ed `evento_risolto` coprono tutti i 18 eventi dei dati e la voce non dice quale. **Rimando al piano di core «esito e Vista completa»**: la voce porta anche la famiglia dell'evento (`Famiglia`: situazione, emergenza, scandalo, shock, debito; `app/src/lib/tipi/Famiglia.ts`). I titoli di quei due tipi si scrivono per famiglia; un evento aggiunto nei dati trova già i suoi.
- **Chi scrive.** Un agente scrive tutte le varianti nel piano della fetta che porta la schermata e le passa da `scrittura-italiana`. Mario legge una tabella (tipo, occhiello d'esempio, titoli) e taglia o riscrive. Ciò che non approva resta senza titolo.
- **Regole di scrittura**, vincolo per chi scrive:
  1. Due tempi: frase secca, poi la coda. «Posato il primo binario. Il nastro, tagliato tre volte».
  2. Mai cifre, mai un fatto che l'occhiello non dice: il titolo non serve per decidere (storia 159).
  3. Terza persona. Il Pungolo non dà del tu e non tifa: punge chi governa, chiunque sia, e l'opposizione quando si muove. Gli stessi titoli valgono col giocatore al governo e all'opposizione.
  4. Il bersaglio è il palazzo, mai chi subisce (sfollati, disoccupati, pazienti).
  5. Mai destra o sinistra, mai nomi, tic o fatti di politici reali.
  6. Il titolo sta su una riga della colonna. Il motto è più corto, in prima persona, sul mestiere del dicastero.

### Scelte mie, confermate da Mario in blocco

- **Chiave del titolo: il tipo di voce**, più la famiglia per i due tipi d'evento. Niente chiave per criticità: all'opposizione la stessa criticità copre esiti opposti (`procedura_ue` è notevole se migliora col giocatore al governo, notevole se peggiora col giocatore all'opposizione). Sette tipi hanno due esiti (`evento_risolto`, `legge_respinta`, `referendum_votato`, `scissione`, `minaccia_di_scissione`, `procedura_ue`, `ministro_esce`): il loro titolo deve reggerli entrambi. È la settima regola di scrittura.
- **Voce senza titolo: resta l'occhiello, che sale a titolo.** È la forma del tono grave, quindi il meccanismo è uno solo. Vale per i tipi lasciati vuoti, per le varianti tagliate da Mario e per un dicastero dei dati senza motti (il ministro non ha motto).
- **Il conto.** Dei 92 tipi, 26 sono sempre ordinari. Cinque di questi salgono a notevoli quando parlano del partito del giocatore (`minaccia_rientrata`, `promessa_verificata`, `promesse_scadute`, `promessa_fatta`, `posizione_spostata`) e prendono due titoli anche loro. Con i due tipi d'evento divisi per cinque famiglie fanno 79 chiavi con titolo: circa 180 titoli, più 40 motti. La dozzina dei tipi frequenti si fissa nel piano, contando le voci in evidenza nelle partite automatiche della CLI (storia 164).
- **Ogni tipo ha la sua chiave nel file, anche vuota**, così `npm run check` segnala il tipo nuovo che il core aggiunge. I file sono quelli di [Testi esterni](03-testi-esterni.md): `pungolo.json` porta titoli, motti e note a biro.
- **Variante.** Titolo: `(hash(id della partita, chiave) + turno + posto fra le voci dello stesso tipo nella pagina) mod n`. Senza stato, uguale dopo un riavvolgimento, mai la stessa variante in due turni di fila. L'id della partita è il seme in esadecimale (`app/src-tauri/src/lib.rs:90`, `IndicePartita.id`): chiude l'aperto di Testi esterni sul seme che non arriva alla UI. Motto: `hash(nome del ministro, dicastero) mod n`.
- **Tono grave.** Resta la regola di [Ossatura e navigazione](07-ossatura-e-navigazione.md): turno grave finché un evento grave è aperto, prima pagina grave se una voce dell'esito ha `tono: grave`. Nel tono grave i testi del Pungolo non si mostrano: resta l'occhiello.
- **Quando si scrivono.** Nella fetta che porta la loro schermata: i motti con [Governo](18-governo.md), i titoli con [Prima pagina](16-prima-pagina.md). Quanti ne servono alla prima fetta lo dice [Fette](10-fette.md): se la prima fetta non porta la prima pagina, servono solo i motti.
- **Note a biro e didascalia della vignetta.** Le decide il ticket della loro schermata ([Schema dello Stato](29-schema-dello-stato.md), [Paese](23-paese.md), Prima pagina), con le stesse regole e nello stesso file.

### Esempi

Dal prototipo vincolante, già senza nomi:

| Occhiello (dal core) | Titolo |
|---|---|
| Piena del Po: argini a rischio in Emilia-Romagna | Il fiume chiede udienza, il governo cerca gli stivali |
| Senato: convertito il decreto Energia, 104 sì e 91 no | Qualcuno vota col cuore. Il cuore non firma |
| Partito il lotto 2 dell'alta velocità Salerno–Reggio | Posato il primo binario. Il nastro, tagliato tre volte |
| Spread a 171 punti, +14 | I mercati prendono appunti. Noi prendiamo tempo |

Due titoli del prototipo portano un nome («Morandi scrive le dimissioni. In bella copia, per ora») e vanno riscritti: «Dimissioni pronte. In bella copia, per ora». I motti del prototipo (`:524` e seguenti) valgono come primo motto di ciascun dicastero.

### Glossario

In `CONTEXT.md`: testi del Pungolo, occhiello, titolo, motto. «Registro» non si usa per i testi (è l'elenco del turno): i file sono i testi neutri e i testi del Pungolo.
