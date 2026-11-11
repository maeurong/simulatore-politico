---
tipo: sintesi
fonti: []
aggiornato: 2026-11-10
---
# Fatti della chiusura: inventario per l'esito del turno

Tutto ciò che una chiusura di turno produce oggi in `sim-core`, un fatto per riga, nell'ordine in cui `chiudi_turno` lo fa accadere: chi lo produce, dove ne resta traccia, quale causa il codice ha in mano in quel punto e chi lo subisce. Serve al piano che introduce l'esito del turno (spec `docs/superpowers/specs/2026-10-21-v1-giocabile.md:390`, storie 14-16 alle righe `:42-44`, glossario `CONTEXT.md:345-351`): oggi un tipo «voce dell'esito» non esiste, esistono frammenti. È un inventario: non propone il tipo Rust e non assegna la criticità.

Letto su: cwd `/Users/mario/GitHub/simulatore-politico/.claude/worktrees/piano-esito-vista-completa-316e4f`, branch `docs/piano-esito-vista-completa`, HEAD `8952a068ccaa432cf706f7679f61e11fd90e81a9` [M: `git -C <worktree> log -1 --format='%H %s'`]. Nessuna fonte esterna: solo codice e documenti del repo, citati `file:riga`. `cargo` non è stato lanciato: niente è misurato eseguendo il gioco.

Tag: [V] letto in questa sessione alla riga citata; [M] misurato con il comando scritto accanto; [INF] inferenza da ciò che è stato letto. **Ogni riga della tabella è [V] alle righe che cita, salvo dove è scritto [INF].**

Abbreviazioni dei percorsi, tutti sotto `crates/sim-core/src/`: `p.rs` = `partita.rs`; `ia.rs` = `partita/ia.rs`; `vita.rs` = `partita/vita.rs`; `v.rs` = `vista.rs`; `min/` = `ministri/`; `parl/` = `parlamento/`; `ev/` = `eventi/`; `eco/` = `economia/`; `el/` = `elettorato/`; `leg/` = `legislatura/`; `disp/` = `disposizioni/`; `info/` = `informazione/`.

Abbreviazioni delle tracce:

- **M** = riga in `Partita.mosse` (`p.rs:126-127`), che passa nella `Vista.mosse` (`v.rs:89-90`, `p.rs:4062`) e si svuota alla chiusura dopo (`p.rs:550`): vive un turno.
- **EV** = `EsitoVoto` (`parl/mod.rs:94-108`) in `Parlamento.esiti`, nella `Vista.esiti` (`v.rs:63`); **ER** = `EsitoReferendum` (`parl/regole.rs:50-61`) nella `Vista.esiti_referendum` (`v.rs:66`). Restano gli esiti dei due turni prima (`parl/mod.rs:1044-1050`).
- **VM** = `Variazione { causa, delta, turno }` del ministro (`min/mod.rs:99-105`), nella `VoceMinistro.variazioni` (`v.rs:606-607`); esce all'apertura del secondo turno dopo (`min/mod.rs:292-293`). La Vista la mostra solo finché il ministro è in carica (`min/mod.rs:571`).
- **VI** = `VariazioneIntesa { partito, causa, delta, turno }` (`parl/intesa.rs:53-58`), nella `VoceAlleato.variazioni` (`v.rs:235-236`) solo col giocatore Presidente del Consiglio (`p.rs:4430-4433`); esce alla chiusura dopo (`parl/intesa.rs:140`).
- **nessuna** = il fatto si perde: resta al più un numero cambiato, senza dire perché.

«Governo» nell'ultima colonna è il governo in carica, del giocatore o avversario; dove conta la differenza è scritto.

## Premesse verificate

- L'ordine della chiusura è nel commento a `p.rs:520-545` e `chiudi_turno` sta a `p.rs:546-725`: confermato [V]. Il commento è indietro sul codice in due punti: mette la «nascita degli eventi» dentro il motore, prima del voto (`p.rs:531`), mentre gli eventi puntuali nascono a `p.rs:675`, dopo `forma_governo` (prima del voto nascono solo gli eventi delle situazioni e le «Aste scoperte», `p.rs:625-636`); e non nomina né l'azzeramento degli acquisti (`p.rs:551`) né `incassa` (`p.rs:637`) [V].
- `self.mosse` si svuota a `p.rs:550` ed è `Vec<MossaIa>` (`v.rs:180-186`): confermato [V]. Non porta solo i fatti dei partiti IA: `spendi_campagne`, `verifica_promesse` e `scissioni` scrivono una riga anche per il partito del giocatore (`p.rs:2303-2320`, `p.rs:2515-2534`, `vita.rs:117-135`; il test `p.rs:6146-6155` legge la riga del giocatore) [V]. Le chiamate a `self.mossa(` sono 25: 3 in `partita.rs`, 17 in `ia.rs`, 5 in `vita.rs` [M: `grep -c "self\.mossa(" crates/sim-core/src/partita.rs crates/sim-core/src/partita/ia.rs crates/sim-core/src/partita/vita.rs`].
- I frammenti nella Vista stanno dove il brief li mette: `esiti` `v.rs:63`, `esiti_referendum` `v.rs:66`, `elezioni` `v.rs:81`, `statistica.revisione` `v.rs:480`, `VoceMinistro.variazioni` `v.rs:606-607`, `VoceAlleato.variazioni` `v.rs:233-236`, `causa_dimissioni` `v.rs:513` [V].
- Nessun tipo per la voce dell'esito né per il registro [M: `grep -rn -E "VoceEsito|struct Registro|enum Criticita" crates --include="*.rs"`, nessun risultato]. «Registro» nel codice è solo il nome a parole di `mosse` (`ia.rs:1`, `ia.rs:23`) [V].
- `verifica_promesse` sta in `p.rs:2511`, non in `vita.rs`, `ia.rs` o `legislatura/`; `vota_confermativi` e `vota_abrogativo` stanno a `p.rs:1234` e `p.rs:1294`, fuori dall'intervallo `788-1055` [V].

## I fatti, nell'ordine della chiusura

A partita finita la chiusura non fa nulla (`p.rs:547-549`). A Camere sciolte l'aula vota solo i confermativi (`p.rs:572-574`).

### A. Avvio (`p.rs:550-551`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 1 | `chiudi_turno`, `p.rs:550-551` | Le mosse del turno prima si svuotano; l'informazione comprata nel turno scade | nessuna: `mosse.clear()`, `acquisti = Acquistati::default()`; la `Vista.acquisti` torna vuota (`v.rs:101`) | nessuna | giocatore |

### B. Eventi aperti risolti (`risolvi_eventi`, `p.rs:4157-4246`, chiamata a `p.rs:552`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 2 | `p.rs:4162-4168`, `spendi` `p.rs:4375-4398` | Emergenza lieve del turno prima pagata dal fondo di riserva | `eco.fondo_speso` fino alla chiusura del quarto trimestre (`eco/mod.rs:292-294`), visibile come `VistaBilancio.fondo_residuo` (`v.rs:413-414`); l'emergenza non è mai stata un evento per il giocatore (`ev/mod.rs:243-245`) | `EventoAperto` intero: evento, regione, costo, `Causa::Rischio { livello }`; la quota pagata che `spendi` rende | regione colpita; governo (fondo) |
| 3 | `p.rs:4170-4245`, `Eventi::risolvi` `ev/mod.rs:304-316` | Evento risolto, con l'opzione scelta o col default | nessuna: `aperti` si svuota (`ev/mod.rs:308`) e l'opzione applicata non si scrive da nessuna parte; restano gli effetti sulle grandezze, senza origine (`dicastero: None`, `p.rs:4192-4203`) | `EventoAperto` (`ev/mod.rs:214-228`): causa strutturata, regione, bersaglio, persona, tono, costo, `scelta` (`None` = default); `DatoOpzione` con id, nome, `default`, effetti (`ev/mod.rs:86-92`) | governo (sotto un governo IA vale sempre il default, `ev/mod.rs:89`, `p.rs:4411-4413`); paese |
| 4 | `p.rs:4206-4213`, `spendi` `p.rs:4375-4398` | Fondo di riserva speso per l'evento, o spesa a debito | fondo: `fondo_speso`, quindi `fondo_residuo`; debito: `eco.decreti[d]`, azzerato dal passo dell'economia (`eco/mod.rs:291`) | costo, pagato, quota = pagato / costo, residuo del fondo prima (`p.rs:4383-4384`) | governo; paese (conti) |
| 5 | `p.rs:4232-4244`, `valence_evento` `p.rs:4355-4370` | Emergenza: risposta premiata, o emergenza ignorata punita | solo `Partito.valence`, nella `VocePartito.valence` (`v.rs:451`): il numero cambia senza causa | delta = `valence_emergenza_risposta` × quota, oppure −`valence_emergenza_ignorata`; partito colpito | partito del Presidente del Consiglio |
| 6 | `p.rs:4214`, `programma_del_mes` `p.rs:4250-4267` | Programma del MES avviato: condizionalità in vigore | `eco.programma_mes = Some(fine)` (`eco/mod.rs:106-108`), che la Vista non porta (`v.rs:22-102`); le disposizioni della condizionalità con `legge = "Memorandum del MES"` nella `VoceDisposizione` (`v.rs:373-375`); VI `ProgrammaMes` per ogni alleato | `durata_mes`, turno di fine, elenco (disposizione, gradino) della condizionalità (`p.rs:4270-4277`), `intesa_mes`; l'evento «Aste scoperte» con `Causa::Spread { pb, soglia }` | governo, alleati, paese |
| 7 | `p.rs:4215-4219`, `ristruttura` `eco/sfc.rs:52` | Ristrutturazione del debito | `eco.ristrutturazione = Some { turno, premio }` (`eco/mod.rs:116-120`), non nella Vista; debito e spread cambiati | `taglio_ristrutturazione`, `premio_ristrutturazione` | paese, fasce media e alta (`el/redditi.rs:100-107`); governo |
| 8 | `p.rs:4221-4226` | Margine UE per l'energia chiesto per l'anno | `regole_ue.clausola_energia_fino_a` (`eco/regole_ue.rs:28-31`), non nella Vista | anno; fuori dalla clausola nazionale | governo |
| 9 | `risolvi_scandalo` `p.rs:4319-4342` | Scandalo: il ministro si dimette (dimissioni date ora o già chieste) | `dimissioni_chieste`, poi il ministro esce al passo K con `causa_dimissioni = DimissioniChieste`; VI `DimissioniChieste` se è di un alleato (`comando.rs:315-317`, `p.rs:1584-1593`); valence senza causa | evento, bersaglio, persona alla nascita, `valence_dimissioni_scandalo` | ministro, suo partito |
| 10 | `p.rs:4343-4350` | Scandalo: ministro o leader difeso | valence senza causa; `Ministro.sotto_scandalo = turni_scandalo` (`min/mod.rs:117-119`), che la `VoceMinistro` non porta (`v.rs:593-608`) | `valence_scandalo`, `turni_scandalo` | partito del bersaglio, anche un partito IA all'opposizione |
| 11 | `p.rs:4320-4324` | Scandalo chiuso in silenzio: il bersaglio non c'è più o è cambiato | nessuna | persona alla nascita contro chi occupa il posto ora | nessuno |

### C. Default delle decisioni (`p.rs:564-569`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 12 | `decidi_proposte` `min/proposte.rs:278-292` | Proposta del ministro respinta per default | VM `PropostaRespinta`: la stessa causa del `Respingi` esplicito (`min/proposte.rs:369-370`), le due non si distinguono | la proposta (fondi con atto e importo, o riforma), `turni_in_sospeso`, `rinviata`, `costo_respingere`; la proposta è già tolta (`min/proposte.rs:288`) | ministro |
| 13 | `min/proposte.rs:283-284` | Proposta di riforma decaduta senza costo: il suo innesco non vale più | nessuna | quale condizione di `tiene` è caduta (`min/proposte.rs:61-80`), non registrata | ministro |
| 14 | `respingi_aperti` `parl/emendamenti.rs:183-194`, `emendamenti_respinti` `p.rs:1525-1530` | Emendamento respinto per default | VI `EmendamentoRespinto`, la stessa causa del respinto esplicito (`p.rs:1496`); stato `Respinto` sull'emendamento finché la misura è in calendario | l'alleato e il costo (`costo_emendamento_respinto` × peso delle politiche, `p.rs:1533-1535`); a `emendamenti_respinti` arriva solo l'indice dell'alleato, non la misura | alleato |

### D. Atti dei ministri (`Ministri::esegui`, `min/agenda.rs:327-370`, chiamata a `p.rs:570-571`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 15 | `min/agenda.rs:336-342` | Rata di un cantiere pagata | `Cantiere.speso` e `turni_mancanti`, nella `VoceCantiere` (`v.rs:582-588`) | rata, margine residuo | dicastero |
| 16 | `min/agenda.rs:337` (ramo falso) | Cantiere fermo un turno: il margine non copre la rata | nessuna: `turni_mancanti` resta com'era | rata e margine residuo | dicastero |
| 17 | `min/agenda.rs:343-347` | Cantiere finito: gli effetti partono | il cantiere sparisce da `cantieri`; gli `EffettoAttivo` (con dicastero e regione) non passano nella Vista | atto, regione, capacità effettiva del ministro | regione o paese; merito al partito del dicastero |
| 18 | `min/agenda.rs:348-358` | Atto del ministro eseguito (atto di un turno) | nessuna: l'agenda è presa a `min/agenda.rs:332` e non torna; cala `margine_residuo` | `Agenda` (`min/agenda.rs:29-41`): atto, regione, stato, motivo già scritto, decreto; spesa; capacità | regione o paese |
| 19 | `min/agenda.rs:359-367` | Cantiere avviato | `cantieri.push`, nella `VoceCantiere` | come la 18 | dicastero |
| 20 | `min/agenda.rs:348-355` | Atto annunciato e non eseguito: fermato, dimissioni chieste, o margine insufficiente | in chiusura nessuna (il «Ferma» ha lasciato una VM `AttoFermato` al momento del comando; gli altri due casi niente) | stato dell'agenda, `dimissioni_chieste`, spesa contro margine | ministro |

In interim non si esegue nulla e i cantieri restano fermi (`min/agenda.rs:333-335`): è uno stato, non un fatto.

### E. Aula (`aula`, `p.rs:788-819`, chiamata a `p.rs:576`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 21 | `p.rs:789-793`, `slitta_leggi` `parl/mod.rs:866-877` | Leggi slittate al turno dopo (turno del bilancio o di crisi); la questione di fiducia cade e il suo costo torna | `Misura.voto` + 1, nella `VoceMisura.voto` (`v.rs:283`); VI `Fiducia` col segno più (`p.rs:2133-2137`) | `occupato` = turno del bilancio o crisi (`p.rs:567`), senza dire quale dei due | governo, alleati |
| 22 | `vota_forma` `p.rs:828-836`, `vota` `p.rs:870-1019` | Decreto convertito | EV con forma `Conversione`, conteggi e linee; un decreto su disposizioni entra in vigore come una legge | conteggi di Camera e Senato, linea di ogni partito, fiducia | governo |
| 23 | `p.rs:979-1000`, `decreto_decaduto` `p.rs:2222-2234` | Decreto decaduto | EV respinta; VM `DecretoDecaduto` al proponente; valence del Presidente del Consiglio senza causa (`p.rs:993-997`); `parlamento.decaduti`, che resta (la firma non si ripete); fondi spostati tornati indietro | conteggi, linee; `FondiDecreto` con dicasteri e importo; quanto torna = min(importo, residuo, margine annuo) | governo, ministro proponente |
| 24 | `p.rs:911-914`, `approva_bilancio` `p.rs:1648-1661` | Legge di bilancio approvata | EV con forma `Bilancio`; `eco.bilancio_approvato`; VM `BudgetTagliato` per ogni ministro con perdita reale positiva (`p.rs:1658-1659`, `min/agenda.rs:248-252`) | perdite reali in % per dicastero (`p.rs:728-733`), `costo_taglio_budget` | governo, ministri |
| 25 | `ricondotto_al_tetto_mes` `p.rs:1765-1792`, chiamata a `p.rs:1652` | Bilancio approvato ma ricondotto al tetto del MES | nessuna: vedi «I due avvisi del piano 9» | crescita proposta e limite, quota tenuta, bilancio prima e dopo: calcolati e scartati | governo |
| 26 | `p.rs:1653-1657` | Bilancio approvato durante l'esercizio provvisorio: margini residui a tre quarti | `margine_residuo` di ogni dicastero | margine annuo del bilancio | governo |
| 27 | `p.rs:915`, `p.rs:969-997` | Legge di bilancio respinta: il presentato cade | EV respinta; `eco.bilancio_presentato = None`; valence del Presidente del Consiglio senza causa; a gennaio esercizio provvisorio | conteggi, linee | governo |
| 28 | `p.rs:935-968`, `entra_in_vigore` `p.rs:1174-1209` | Legge approvata, valori in vigore | EV; `VoceDisposizione.in_vigore`, `legge`, `precedente` (`v.rs:373-375`); `leggi_del_giocatore`, che resta per l'eredità (`p.rs:122-123`, `p.rs:1180-1182`) | conteggi dei due rami, linea per partito, fiducia. I franchi tiratori non sono scritti: dai conteggi e dalle linee si ricava solo il saldo, perché un seggio può votare contro la linea nei due versi (`parl/voto.rs:126-139`) [INF] | paese; governo, o l'opposizione che l'ha promossa |
| 29 | `p.rs:1194-1199` | Valore rinviato da un emendamento accolto: diventa valore in arrivo | `VoceDisposizione.in_arrivo` con gradino e data (`v.rs:376`, `v.rs:385-388`) | emendamento accolto, `turni_rinvio_regime` | paese |
| 30 | `p.rs:1186-1193` | Valore di una legge approvata non applicato: è vincolato dal memorandum del MES | nessuna: la legge risulta approvata, la disposizione resta dov'era | fine del programma, elenco della condizionalità | governo o promotore |
| 31 | `p.rs:962-967`, `norma_lontana` `p.rs:1447-1465` | Norma lontana: l'alleato perde intesa | VI `NormaLontana` | distanza fra dichiarata e norma, soglia `distanza_alleato`, costo, peso delle politiche: resta solo il delta | alleato |
| 32 | `p.rs:968`, `seggi_persi` `p.rs:1220-1229` | Seggi attesi persi con la legge elettorale | VI `SeggiPersi` | Δseggi di Camera più Senato, `costo_seggio_perso` | alleato |
| 33 | `p.rs:920`, `p.rs:1014-1018` | Norma costituzionale: prima deliberazione approvata, torna in calendario | EV con `deliberazione = 1`; la misura di nuovo in calendario con `deliberazione = 2` | conteggi | governo |
| 34 | `p.rs:938-956` | Norma costituzionale approvata senza i due terzi: referendum confermativo in attesa | `parlamento.confermativi`, nella `Vista.referendum` con `richiesto` e `scelta` (`v.rs:203-214`); l'EV della seconda deliberazione | `due_terzi`, `richiesto` (un quinto contro), `chiedibile` | governo, opposizione |
| 35 | `p.rs:969-997` | Legge respinta: le sue disposizioni restano bloccate | EV; `parlamento.respinte` (disposizione, turno da cui si ripresenta), non nella Vista; VM `LeggeRespinta` al proponente; valence del Presidente del Consiglio senza causa (non per la proposta dell'opposizione, `p.rs:992-997`) | conteggi, linee, `turni_blocco_respinta` | governo o promotore, ministro proponente |
| 36 | `p.rs:894`, `p.rs:970-974` | Questione di fiducia respinta: il governo è dimissionario | EV con `fiducia_respinta = true`; `legislatura.dimissioni`, consumato a `p.rs:667` | conteggi a blocchi | governo |
| 37 | `p.rs:1002-1012` | Posizione percepita scivolata col voto | `VocePartito.posizione_percepita` (`v.rs:450`): solo il valore corrente | `scivolamento_percepita`, posizione sostenuta (nuova col sì, vigente col no) | ogni partito che vota sì; il giocatore a ogni voto |
| 38 | `decade` `p.rs:1024-1051`, chiamata a `p.rs:877` e in apertura a `p.rs:681-685` | Legge elettorale decaduta senza voto | EV con `decaduta` = testo col motivo | solo testo: il motivo reso da `legge_non_valida` (`p.rs:1028`) | governo o promotore |
| 39 | `vota_confermativi` `p.rs:1234-1280` | Referendum confermativo votato | ER con sì, no, votanti, aventi diritto, approvato; se approvato valori in vigore e `leggi_del_giocatore` | affluenza per esito | paese, governo |
| 40 | `p.rs:1242`, `p.rs:1259-1275` | Confermativo non richiesto: la norma entra in vigore senza voto e senza esito | solo la `VoceDisposizione` | `richiesto = false` | paese |
| 41 | `vota_abrogativo` `p.rs:1294-1390` (rami `1360-1381`) | Referendum abrogativo approvato | ER; disposizione al valore dopo l'abrogazione con legge «referendum del AAAA»; gradino in `bloccati`, nel `VoceGradino.bloccato` (`v.rs:397`); valence di ogni partito di governo senza causa | affluenza, quorum, invito all'astensione, `valence_referendum_perso` | governo, paese |
| 42 | `p.rs:1358`, `p.rs:1382-1388` | Referendum abrogativo fallito | ER; senza quorum il promotore perde valence, senza causa | affluenza, quorum, `valence_referendum_fallito` | promotore |
| 43 | `p.rs:1304-1336` | Referendum abrogativo decaduto senza voto | ER con `decaduto` = uno di tre testi | solo testo (vincolo del memorandum con la sua fine, valore già quello del quesito, legge che non sarebbe più valida) | promotore |
| 44 | `referendum_ia` `ia.rs:633-694`, chiamata a `p.rs:800-804` | Referendum abrogativo chiesto dall'opposizione IA | M «Chiede il referendum su …»; `parlamento.referendum`, nella `Vista.referendum` | stima sul sondaggio (sì, no, astenuti), margine, guadagno del promotore: nel testo solo la quota del sì e il quorum | governo |
| 45 | `p.rs:805-808` | Firme dell'abrogativo raccolte | `Referendum.firme`, nella `VistaReferendum.firme` (`v.rs:210`) | nessuna | promotore, governo |
| 46 | `p.rs:811-818`, `sfiducia` `p.rs:3667-3716`, `voto_sfiducia` `p.rs:3720-3750` | Sfiducia al governo votata | EV con forma `Sfiducia`, nome, conteggi a blocchi, linee. Approvata: dimissioni. Respinta: VI `SfiduciaRespinta` col segno più per ogni alleato, valence del promotore senza causa | linea di ogni partito (gli alleati in ultimatum che votano sì), conteggi | governo; promotore |
| 47 | `p.rs:3695-3703` | Sfiducia individuale approvata: il ministro esce | EV; `causa_dimissioni = Sfiduciato`; VI `DimissioniChieste` se è di un alleato | come la 46; il ministro era un tecnico o no | ministro, suo partito |
| 48 | `p.rs:811-816`, `p.rs:3670-3672` | Mozione di sfiducia scartata senza voto: governo già dimissionario, o dicastero in interim | nessuna: la mozione è tolta con `take` | il motivo dello scarto | promotore |

### F. Scissioni (`scissioni`, `vita.rs:114-137`, chiamata a `p.rs:580-581`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 49 | `scissione` `vita.rs:146-222` | Scissione: nasce la frangia | M «Scissione: nasce … con N deputati e M senatori, sulla posizione …» sul partito d'origine; partito nuovo; seggi; `legislatura.frange` fino al voto | distanza fra percepita e dichiarata (non nel testo), `soglia_scissione`, `quota_scissione`, seggi e cassa presi | il partito, anche quello del giocatore; il governo, se il partito ne fa parte |
| 50 | `vita.rs:126-135` | Minaccia di scissione | M «Minaccia di scissione: la posizione percepita dista … punti dalla dichiarata», una sola finché la distanza non rientra; `Partito.minaccia`, che la `VocePartito` non porta (`v.rs:446-462`) | distanza, `minaccia_scissione`; se la scissione non è nata per il tetto dei dieci partiti o perché la frangia non avrebbe seggi (`vita.rs:123`, `vita.rs:153-158`) il testo non lo dice | il partito, anche quello del giocatore |

### G. Esercizio provvisorio (`p.rs:582-588`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 51 | `p.rs:584-588` | Governo dimissionario dopo un turno di esercizio provvisorio | `legislatura.dimissioni`, consumato a `p.rs:667`; nel turno la `VistaBilancio.esercizio_provvisorio` era vera (`v.rs:419-420`) | `esercizio_provvisorio(data)` (`eco/finanza.rs:283-286`) | governo |

### H. Turno del voto, prima del motore (`p.rs:589-595`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 52 | `verifica_promesse` `p.rs:2511-2537` | Promessa mantenuta o mancata | M «Promessa mantenuta: …» o «Promessa mancata: …», anche per il giocatore; valence | la disposizione sta o non sta al gradino promesso; `valence_promessa_mantenuta`, `valence_promessa_mancata` | il partito che ha presieduto il governo |
| 53 | `p.rs:2517-2520` | Promesse scadute senza verifica: il partito non ha presieduto abbastanza turni, o è sciolto | nessuna: le promesse sono tolte con `take` | `turni_da_presidente` contro `turni_promessa` | il partito |
| 54 | `campagna_ia` `ia.rs:146-196`, `promesse_ia` `ia.rs:202-229` | Campagna e promesse dei partiti IA decise | `Partito.campagna` fino al voto; M «Promette …»; `promesse_campagna`, nelle `VocePromessa` (`v.rs:460-472`) | rese per bersaglio e leva, non conservate | partiti IA |
| 55 | `spendi_campagne` `p.rs:2302-2322` | Spesa di campagna pagata | M «Spende N euro: persuade …, mobilita …, convince …» per ogni partito con una campagna, giocatore compreso (`leg/campagna.rs:296-308`); cassa | spese rimaste (bersaglio, leva, euro), speso | ogni partito |
| 56 | `p.rs:2308-2313` | Campagna tagliata: la cassa non basta | nessuna distinta: vedi «I due avvisi del piano 9» | cassa, spese chieste e spese rimaste: in mano nel ciclo, non scritte | il partito; nei fatti il giocatore |

### I. Motore (`p.rs:596-637`; `motore` `p.rs:1056-1097`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 57 | `p.rs:1068-1070` | Programma del MES finito | `programma_mes = None`; nessuna voce, e la Vista non lo porta nemmeno finché dura | turno contro fine | governo |
| 58 | `finanza` `eco/finanza.rs:291-296` | Il bilancio approvato entra in vigore a gennaio | `eco.bilancio`, nella `VistaBilancio.anno` (`v.rs:405-406`) | anno | governo |
| 59 | `passo_con` `eco/mod.rs:258-340` | Passo dell'economia: PIL, prezzi, lavoro, conti, debito, spread | stato dell'economia; `Vista.spread_pb` (`v.rs:40-41`); `storico` di 8 trimestri (`eco/mod.rs:92-93`) | le cinque innovazioni (`p.rs:597`), usate solo per gli shock; nessuna scomposizione delle variazioni | paese |
| 60 | `controllo_annuale` `eco/regole_ue.rs:143-191` (solo ad aprile-giugno) | Controllo annuale delle regole UE: deviazione della spesa netta e conto di controllo | `regole_ue.ultima_deviazione`, `conto_controllo`, non nella Vista | deviazione, clausola, tetto, conto, `rilevante`, disavanzo dell'anno, debito | governo, paese |
| 61 | `aggiorna_regole_ue` `eco/regole_ue.rs:111-140` | Procedura per disavanzo eccessivo: cambio di stato (aperta, sospesa, inadempiente, intimata, sanzionata, chiusa) | `regole_ue.pde`, `turni_in_stato`, non nella Vista | come la 60 | governo, paese |
| 62 | `ammende` `eco/regole_ue.rs:50-59`, letta in `eco/finanza.rs:248` | Ammenda della procedura pagata | `conti.spese.ammende` del trimestre, non nella Vista | % del PIL annuo, stato della procedura | paese (conti) |
| 63 | `aggiorna_spread` `eco/spread.rs:52-67` | Lo scudo della BCE (TPI) vale o non vale più: il premio di sfiducia resta nello spread o si azzera | `eco.premio_sfiducia`, dentro lo spread; è uno stato, non c'è un passaggio registrato [INF] | procedura inadempiente, premio della ristrutturazione oltre 50 pb, programma del MES in corso | paese |
| 64 | `avanza` `grandezze.rs:165-167`, chiamata a `p.rs:1078` | Indicatori di servizio e grandezze avanzano | `valori`; `prima`, nella `Vista.grandezze` con un turno di ritardo (`v.rs:38-39`) | effetti attivi, deriva | paese |
| 65 | `aggiorna_redditi` `el/redditi.rs:78`, chiamata a `p.rs:1079-1080` | Redditi dei gruppi aggiornati | `Gruppo.reddito` e `reddito_prima` (`el/mod.rs:366-367`), non nella Vista | componenti del reddito per gruppo | gruppi |
| 66 | `p.rs:1081-1085` | Ristrutturazione finita: premio sotto 1 pb e perdita dei redditi conclusa | `ristrutturazione = None`; nessuna voce | premio, turni | paese |
| 67 | `giudizio` `el/giudizio.rs:28-147`, chiamata a `p.rs:1086-1093` | Giudizio dei gruppi aggiornato: reddito reale contro l'aspettativa, disoccupazione, servizi | `Gruppo.giudizio` (solo il valore nuovo) e `crescite`; niente variazione, niente Vista: vedi «Storia 116» | crescita, aspettativa, componenti con il responsabile: locali al ciclo | partiti di governo |
| 68 | `p.rs:598`, `p.rs:602-607` | Il «convincere» della campagna sposta le posizioni; ogni situazione accesa con la spinta le sposta a destra | nessuna | numero di situazioni accese con la spinta × `spinta_situazione` | elettorato |
| 69 | `p.rs:608-615` | Sondaggio pubblico del trimestre rilevato | `informazione.sondaggio`, nella `Vista.sondaggio` (`v.rs:46`), sostituito a ogni chiusura | nessuna | tutti |
| 70 | `Statistica::aggiorna` `info/statistica.rs:79-116`, chiamata a `p.rs:616-622` | Prima stima del trimestre pubblicata; revisione della stima precedente | `prima_stima` e `revisione { trimestre, prima_stima, riveduta }`, nella `VistaStatistica` (`v.rs:477-481`); la revisione dura una chiusura, poi è sovrascritta | strutturata: le quattro grandezze, prima e rivedute | giocatore (informazione) |
| 71 | `situazioni::aggiorna` e `apri` `ev/situazioni.rs:38-96`, chiamate a `p.rs:625-635` | Situazione accesa: si apre il suo evento | `eventi.accese[i]`, nella `VoceSituazione.accesa` (`v.rs:128-135`); `EventoAperto`, nella `VoceEvento` con la causa in testo (`v.rs:141-151`) | `Causa::Soglia { misura, valore, soglia }`: l'innesco legge il vero, la causa porta il valore ufficiale (`p.rs:627-628`) | paese, governo |
| 72 | `ev/situazioni.rs:44-60` | Situazione spenta | solo il flag `accese[i]`; nessun evento e nessuna riga: `aggiorna` rende solo le accese (`ev/situazioni.rs:57-62`) | misura e soglia di spegnimento, in mano in `aggiorna` | paese, governo |
| 73 | `crisi_del_debito` `p.rs:397-425`, chiamata a `p.rs:636` | Crisi del debito: si aprono le «Aste scoperte» | `EventoAperto` con tono grave, nella `VoceEvento` | `Causa::Spread { pb, soglia }` | governo, paese |
| 74 | `incassa` `p.rs:2275-2289`, chiamata a `p.rs:637` | Entrate della cassa: 2 per mille e donazioni | `Partito.cassa`, nella `VocePartito.cassa` (`v.rs:458-459`) | entrate per partito, non conservate | ogni partito |

Nel passo I ci sono anche i ritorni continui (giudizio che sbiadisce, composizione dell'elettorato a `p.rs:1094-1095`): muovono numeri, non sono fatti con un prima e un dopo.

### J. Voto (`vota_parlamento`, `p.rs:2802-2893`, chiamata a `p.rs:638-640`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 75 | `p.rs:2803`, `decadi_tutto` `parl/mod.rs:1015-1024` | Misure in calendario decadute per la fine della legislatura | EV con `decaduta` = «Decaduta per la fine della legislatura» | solo testo | governo, opposizione |
| 76 | `parl/mod.rs:1025-1041` | Referendum abrogativo sospeso per lo scioglimento | ER con `decaduto` = «Sospeso per lo scioglimento delle Camere» | solo testo | promotore |
| 77 | `p.rs:2804-2806`, `parl/mod.rs:1024` | Gradini bloccati da un referendum sbloccati; leggi respinte dimenticate | `VoceGradino.bloccato` torna falso; nessuna voce | nessuna | tutti |
| 78 | `p.rs:2807-2873` | Risultato del voto | `EsitoElezioni` (`leg/voto.rs:27-33`): affluenza, quota e seggi di ogni partito, premio, coalizioni; in `legislatura.elezioni`, che non si accorcia mai (misura sotto la tabella), nella `Vista.elezioni` (`v.rs:80-81`); seggi nella `Vista.seggi` | i numeri del voto; nessun perché | tutti |
| 79 | `p.rs:2871`, `p.rs:2887` | Incarico assegnato, si apre la trattativa | `legislatura.incarico`, nella `Vista.incarico` come giocatore, Capo dello Stato o guida (`v.rs:190-197`): la coalizione dell'incarico IA non passa; `Vista.fase` | coalizioni, seggi | tutti |
| 80 | `p.rs:2888-2891` | Partita finita: il giocatore senza seggi nelle due Camere | `fine = SenzaSeggi`, nella `Vista.finita` e nell'eredità (`v.rs:36`, `v.rs:68`) | seggi | giocatore |
| 81 | `scioglimenti` `vita.rs:258-296`, chiamata a `p.rs:2892` | Partito IA sciolto dopo due elezioni senza seggi | M «Si scioglie dopo 2 elezioni senza seggi»; `Partito.sciolto`, nella `VocePartito.sciolto` | `senza_seggi` | partito IA |
| 82 | `vita.rs:286-295` | Coalizione dell'ultimo voto caduta perché ridotta a un solo partito; lo sciolto esce dall'incarico IA | `legislatura.coalizioni`; nessuna voce | elenco degli sciolti | partiti della coalizione |
| 83 | `p.rs:2857-2859`, `p.rs:2867` | L'errore comune dei sondaggi si rivela col voto ed è riestratto | nessuna: non c'è un confronto calcolato fra l'ultimo sondaggio e il voto | sondaggio pubblico e intenzioni vere, tutti e due in mano | giocatore (informazione) |

`legislatura.elezioni` non si accorcia in nessun punto del codice [M: `grep -rn -E "elezioni\.(truncate|retain|drain|pop|remove|clear)" crates --include="*.rs"`, nessun risultato].

### K. Soglie (`p.rs:641-648`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 84 | `pressioni_intesa` `p.rs:1602-1640` | Pressioni di fine turno sull'intesa: ministri scontenti, quota dei dicasteri sotto la richiesta, calo nei sondaggi | VI `MinistriScontenti`, `QuotaScoperta`, `SondaggiInCalo`, solo se il delta non è zero | media della soddisfazione contro `soglia_minaccia`; richiesta e offerta; quota al voto e quota nel sondaggio: resta solo il delta | alleati |
| 85 | `soddisfazione::passo` `min/soddisfazione.rs:34-41`, da `Ministri::soglie` `min/mod.rs:269-284` | Indicatore di servizio in calo: il ministro perde soddisfazione | VM `IndicatoreInCalo` | calo, `scala_servizio`, `costo_calo_servizio` | ministro |
| 86 | `min/soddisfazione.rs:42-52` | Obiettivo raggiunto: premio, e l'obiettivo si sposta | VM `ObiettivoRaggiunto`; `VoceMinistro.obiettivo` (`v.rs:599`) | valore, obiettivo, `premio_obiettivo` | ministro |
| 87 | `min/soddisfazione.rs:58-67` | Ministro si dimette (soddisfazione sotto la soglia), o esce per le dimissioni chieste | `causa_dimissioni` finché dura l'interim (`min/mod.rs:306-314`), nella `VoceMinistero.causa_dimissioni` (`v.rs:512-513`); le sue variazioni restano nello stato ma la Vista in interim non le mostra | enum `Causa`: la peggiore delle variazioni, o `DimissioniChieste`; soddisfazione e `soglia_dimissioni` non conservate | governo |
| 88 | `min/mod.rs:550`, `min/mod.rs:560` | Minaccia di dimissioni | calcolata nella Vista (`minaccia` = soddisfazione sotto `soglia_minaccia`, `v.rs:604`): è uno stato, niente dice «da questo turno»; `causa_principale` (`v.rs:605`) | soddisfazione, soglia, causa peggiore | governo |
| 89 | `Intesa::soglie` `parl/intesa.rs:138-154`, `ultimatum_di` `parl/intesa.rs:116-132` | Ultimatum posto | `Intesa.ultimatum[j] = Some(causa)` finché dura; nella `VoceAlleato.causa_ultimatum` (`v.rs:232-233`) o, sotto un governo IA, nella `VoceFascia.causa_ultimatum` (`v.rs:243-247`) | enum `CausaIntesa`: la peggiore delle variazioni, `NormaLontana` se non ce ne sono; intesa e `soglia_ultimatum` | governo, alleato |
| 90 | `parl/intesa.rs:122-123` | Ultimatum ritirato | `ultimatum[j] = None`; nessuna voce | intesa tornata alla soglia o sopra | governo, alleato |

Nello stesso passo ci sono i ritorni verso la base di soddisfazione, intesa e valence (`min/soddisfazione.rs:54-56`, `parl/intesa.rs:146-152`, `p.rs:645-648`): continui, senza traccia propria.

### L. Cambio di leader dopo il voto (`p.rs:649-652`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 91 | `leader_dopo_il_voto` `ia.rs:94-104`, `cambia_leader` `ia.rs:55-78` | Cambio di leader per il calo al voto | M «Nuovo leader: …, dopo il calo di N punti percentuali al voto»; `Partito.leader`; valence alla base più la luna di miele | calo in punti, `calo_cambio_leader` | partito IA |

### M. Uscite dal governo (`p.rs:653-660`, fuori dal turno del voto e senza dimissioni)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 92 | `frange_in_uscita` `vita.rs:231-252` | Una frangia di governo esce dalla maggioranza | M «Esce dalla maggioranza: nei sondaggi il voto gli vale più che restare al governo» | valore del voto contro valore al governo più `margine_voto_anticipato`: mai in cifre, per scelta (`v.rs:180`) | governo |
| 93 | `uscite` `p.rs:3439-3466`; chi esce lo dice `parl/intesa.rs:141-145` | Un alleato esce dal governo: intesa sotto `soglia_uscita` | `governo.coalizione`; l'ultimatum cade (`p.rs:3447-3449`); i suoi dicasteri in interim con `causa_dimissioni = UscitaDalGoverno`. Nessuna riga, nemmeno sotto un governo IA; un alleato senza dicasteri esce senza lasciare altro | intesa, soglia, causa dell'ultimatum appena azzerata | governo |
| 94 | `rimpasto` `p.rs:3471-3492` | Rimpasto | `governo.dicasteri`; `causa_dimissioni = Rimpasto` sui dicasteri che cambiano partito | spartizione di default al netto dei dicasteri tenuti | governo, ministri |
| 95 | `p.rs:3463-3465` | Crisi aperta: la coalizione rimasta non ha la maggioranza | `legislatura.crisi = Some(turno + 1)`, nella `Vista.crisi` del turno dopo (`v.rs:86-88`) | seggi della coalizione | governo |

### N. Voto anticipato del governo IA (`p.rs:661-662`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 96 | `voto_anticipato` `ia.rs:852-867` | Il governo IA si dimette puntando al voto | M «Si dimette puntando al voto: nei sondaggi il voto gli vale più che restare al governo»; `dimissioni` e `voto_anticipato`, consumati nella stessa chiusura | valore del voto, valore del restare, margine: mai in cifre | governo avversario; tutti |

### O. Fine della crisi (`fine_della_crisi`, `p.rs:3501-3529`, chiamata a `p.rs:663-665`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 97 | `p.rs:3519-3528` | Coalizione allargata: gli allargati entrano al governo, rimpasto | `governo.coalizione`; intesa alla base; `Rimpasto`; VI `RotturaImpegno`. Sotto un governo IA la riga «Allarga la coalizione a …» è dell'apertura prima ed è già stata svuotata a `p.rs:550` | allargati, seggi | governo |
| 98 | `p.rs:3511-3518` | Sfiducia automatica a fine crisi | EV «Mozione di sfiducia al governo»; approvata: dimissioni | linee, conteggi | governo |
| 99 | `p.rs:3514-3516` | Crisi riaperta: sfiducia respinta, o già votata nel turno | `legislatura.crisi = Some(turno + 1)` | esito della sfiducia | governo |

### P. Capo dello Stato (`capo_dello_stato`, `p.rs:3550-3571`, chiamata a `p.rs:666-669`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 100 | `p.rs:667` | Il governo cade | `legislatura.dimissioni` è un booleano e si consuma qui: perché è caduto non si conserva. Le origini sono sette: `p.rs:587` (esercizio provvisorio), `p.rs:973` (fiducia respinta), `p.rs:3533` (comando «Dimettiti»), `p.rs:3694` (sfiducia), `ia.rs:708` e `ia.rs:762` (crisi del governo IA), `ia.rs:861` (voto anticipato). Nel turno del voto le dimissioni si azzerano senza effetto | l'origine, nota solo nel punto in cui il flag si alza | governo; tutti |
| 101 | `p.rs:3551-3553`, `decadi_calendario` `parl/mod.rs:977-1007` | Le misure in calendario del governo caduto decadono | EV con `decaduta` = «Decaduta per la caduta del governo» | solo testo | governo caduto |
| 102 | `p.rs:3556-3559` | Camere sciolte: nessuna maggioranza contigua | `fase = Sciolta`, nella `Vista.fase` (`v.rs:27-28`) | seggi, partito escluso | tutti |
| 103 | `p.rs:3560-3570` | Incarico dopo la caduta: ai partiti IA, al giocatore, o «guida» | `legislatura.incarico` e `maggioranza`; nella `Vista.incarico` | coalizione trovata, primo per seggi | tutti |

### Q. Governo nuovo (`forma_governo`, `p.rs:3293-3404`, chiamata a `p.rs:670-672`, in trattativa e fuori dal turno del voto)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 104 | `p.rs:3296`, `chiedi_guida` `p.rs:3618-3645` | Guida del ribaltone chiesta d'ufficio: ceduta o no | ceduta: `incarico = Giocatore`. Rifiutata: l'errore con la causa («… non ti cede la guida: …», `p.rs:3635-3639`) è scartato da `let _ =` | la causa del rifiuto, già in testo | giocatore |
| 105 | `p.rs:3300-3404` | Governo formato, dopo il voto o per ribaltone: i due casi non si distinguono | `Governo` (presidente, dicasteri, coalizione, programma); ministri tutti nuovi e in rodaggio (`min/mod.rs:207-219`). M «Forma il governo con …: la maggioranza che gli vale di più» solo nel ramo `maggioranza_ia` (`p.rs:3309`, `p.rs:3329-3331`, `ia.rs:818-831`): nessuna riga con l'incarico IA dato dal voto o dal Capo dello Stato, con la contigua di ripiego, col governo del giocatore | coalizione, presidente, spartizione; se la maggioranza è stata scelta per valore | tutti |
| 106 | `p.rs:3366-3388` | Programma del governo avversario dichiarato | `governo.programma`, nella `Vista.programma` (`v.rs:91-93`); M «Programma di governo: N disposizioni, la prima è …» | programma intero | opposizione |
| 107 | `p.rs:3342-3351` | Spartizione: intesa di ogni alleato sopra o sotto la base secondo la quota avuta | VI `Spartizione` (solo col giocatore presidente) | offerta meno richiesta, `intesa_per_punto` | alleati |
| 108 | `paga_rotture` `p.rs:3420-3433` | Rottura dell'impegno pre-elettorale | VI `RotturaImpegno`; valence senza causa | chi rompe, con chi; `valence_rottura_impegno`, `intesa_rottura_impegno` | chi rompe, anche il giocatore |
| 109 | `giudizio_da_zero` `p.rs:3407-3415` | Il giudizio riparte da zero per chi entra al governo | nessuna | elenco degli entranti | partiti entranti |
| 110 | `p.rs:3390-3395` | Cambio di leader per la perdita del governo | M «Nuovo leader: …, dopo la perdita del governo» | presidente uscente fuori dalla coalizione nuova | partito IA uscente |
| 111 | `p.rs:3396-3397` | Ultimatum alla nascita del governo, per l'alleato che entra sotto la soglia | come la 89 | come la 89 | governo, alleato |
| 112 | `p.rs:3319-3327` | Camere sciolte: in trattativa nessun governo possibile | `fase = Sciolta` | nessuna coalizione trovata | tutti |

### R. Eventi puntuali del turno dopo (`nascono_i_puntuali`, `p.rs:341-391`, chiamata a `p.rs:675`)

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 113 | `p.rs:361-390` | Evento puntuale nato: emergenza grave o con vittime, scandalo, shock | `eventi.aperti`, nella `VoceEvento` con nome, famiglia, causa in testo, tono, regione, bersaglio, costo, opzioni; il danno dell'emergenza è già negli effetti (`p.rs:376-381`) | `Causa::Rischio { livello }` o `Causa::Shock { serie, sigma, soglia }` (`ev/rischio.rs:114-116`, `ev/rischio.rs:151-155`) | governo, regione, ministro o leader bersaglio |
| 114 | `p.rs:382-389` | Emergenza lieve nata | `eventi.lievi`, nascosta al giocatore; danno negli effetti | come la 113 | regione |
| 115 | `estrai` `ev/rischio.rs:179-188` | Candidato uscito ma escluso dal tetto di un evento a turno | nessuna: `estrai` rende solo il vincitore. Per lo shock la spec chiede altro: «uno shock escluso succede comunque nel motore e va nell'esito col suo default» (spec `:360`) | il `Candidato` con causa e costo, in mano in `candidati` | paese |

### S. Apertura del turno nuovo (`p.rs:676-724`, dopo `turno + 1`)

Da qui `self.turno` è già quello nuovo: le variazioni e le righe nate in questo passo portano il turno dopo, ma stanno nello stesso `mosse` della chiusura.

| # | Funzione, `file:riga` | Fatto | Traccia oggi | Causa in mano | Chi lo subisce |
|---|---|---|---|---|---|
| 116 | `arrivi` `disp/stato.rs:154-171`, chiamata a `p.rs:678` | Valore in arrivo entrato in vigore | `VoceDisposizione.in_vigore`, `legge`, `precedente`; l'arrivo esce da `in_arrivo`; nessuna voce | `InArrivo { gradino, turno, legge }` | paese |
| 117 | `disp/stato.rs:162-167` | Arrivi della legge elettorale scartati: la legge che ne uscirebbe non è valida | nessuna | l'errore di `valida`, non letto | paese |
| 118 | `Parlamento::apri` `parl/mod.rs:1044-1050`, chiamata a `p.rs:679` | Gli esiti più vecchi di due turni escono | EV ed ER tolti | data | giocatore |
| 119 | `movimenti_ia` `ia.rs:356-407`, da `apertura_campagna` `ia.rs:126-133`, chiamata a `p.rs:687` | Nel turno del voto un partito IA sposta la posizione dichiarata | M «Sposta la posizione da X a Y: …» col motivo della regola; `posizione_dichiarata` | regola di movimento, guadagno o perdita all'ultimo voto | partito IA |
| 120 | `protesta` `vita.rs:47-108` | Nasce il partito di protesta | M «Nasce …, partito di protesta in posizione N: elettori lontani dai partiti e astensione alta»; partito nuovo; `legislatura.protesta` | punto dell'asse, quota di elettori, astensione, media: nel testo solo il punto | tutti |
| 121 | `coalizioni_ia` `ia.rs:254-352` | Coalizioni IA prima del voto: un partito entra in coalizione o corre da solo | M «Entra in coalizione con …» o «Corre da solo: nel valore atteso ci guadagna»; `legislatura.coalizioni_ia` | valori attesi: mai in cifre | partiti IA |
| 122 | `apri_emendamenti` `parl/emendamenti.rs:120-163`, chiamata a `p.rs:688-700` | Un alleato chiede un emendamento | `RichiestaEmendamento` aperta, nella `VoceEmendamento` con testo e motivo (`v.rs:345-350`) | solo testo: «a N punti dalla sua posizione», «tocca il suo dicastero …», seggi persi | governo |
| 123 | `Ministri::apri` `min/mod.rs:289-318`, chiamata a `p.rs:715` | Interim: nasce la rosa dei successori; il turno dopo entra il successore, in rodaggio | `rosa`, nella `VoceMinistero.rosa`; ministro nuovo con `rodaggio`; `causa_dimissioni` azzerata | scelta nella rosa | governo |
| 124 | `apri_agende` `min/agenda.rs:217-243`, `agenda_di` `min/agenda.rs:281-322` | Agenda annunciata, o nessuna agenda | `Agenda`, nella `VoceAgenda` con motivo in testo (`v.rs:544-553`); `motivo_senza_agenda` (`v.rs:516-517`) | punteggi dell'atto e della regione, ridotti a un testo | governo |
| 125 | `proponi` `min/proposte.rs:231-274` | Proposta del ministro nata, ricalcolata o decaduta | `StatoDicastero.proposta`, nella `VoceProposta` con motivo (`v.rs:531-539`); la decaduta non lascia nulla | innesco (evento, fondi, sblocco, distanza) | governo |
| 126 | `p.rs:716-720` | Anno nuovo in esercizio provvisorio: i ministri pagano il taglio reale; niente agende né proposte | VM `BudgetTagliato` col turno nuovo; `VistaBilancio.esercizio_provvisorio` | perdite reali per dicastero | governo, ministri |
| 127 | `crisi_ia` `ia.rs:705-772`, da `apertura_ia` `ia.rs:110-121`, chiamata a `p.rs:721` | Governo IA in crisi: allarga la coalizione o si dimette | M «Allarga la coalizione a …», o «Si dimette: …» con uno di tre motivi; `allargati` o `dimissioni` | fila dei partiti, vicini utili, intesa sotto soglia | governo avversario |
| 128 | `governo_ia` `ia.rs:877-883`, testo da `bilancio_ia` `ia.rs:1035-1087` | Il governo IA presenta il bilancio | M «Bilancio AAAA: budget indicizzati per il N% dell'inflazione, entro o oltre il tetto UE, con le richieste di …»; misura in calendario | quota indicizzata, crescita della spesa netta e tetto, alleati in ultimatum | opposizione |
| 129 | `ia.rs:885-919` | Il governo IA porta in aula una voce del programma | M «Porta in aula: …»; misura in calendario | voce del programma; le voci scartate per non portare un alleato all'ultimatum non lasciano righe | opposizione |
| 130 | `ia.rs:920-931` | Il governo IA accoglie la proposta di un ministro | M «Accoglie la proposta del dicastero …» | proposta | governo avversario |
| 131 | `ia.rs:932-976` | Il governo IA accoglie o respinge un emendamento | M «Accoglie l'emendamento di … su …: motivo» o «Respinge l'emendamento di … su …»; VI `EmendamentoRespinto`, che il giocatore all'opposizione non vede | uno di tre motivi, in testo | alleato del governo avversario |
| 132 | `ia.rs:977-1001` | Il governo IA pone la questione di fiducia | M «Pone la questione di fiducia su …: la forbice include la sconfitta»; `Misura.fiducia`; VI `Fiducia` | forbice in bilico | alleati |
| 133 | `sfiducia_ia` `ia.rs:518-605` | L'opposizione IA presenta la sfiducia, al governo o a un ministro | M «Presenta la sfiducia al governo: …» o «Presenta la sfiducia a …, …: …»; `parlamento.sfiducia`, che la Vista non porta (`v.rs:22-102`): la mozione in attesa si vede solo dalla riga | linea prevista, alleati in ultimatum che la voterebbero, ministro sotto scandalo o col giudizio peggiore | governo |
| 134 | `proposta_ia` `ia.rs:435-507` | L'opposizione IA porta una proposta nel posto riservato | M «Porta nel posto riservato …; N seggi … la voterebbero»; misura in calendario con `promotore` | seggi di governo che voterebbero sì, guadagno del promotore | governo |
| 135 | `registra_punto` `p.rs:481-518`, chiamata a `p.rs:722-724` | Punto della serie dei tre indicatori sintetici | `serie`, che resta per tutta la partita (`p.rs:130-131`, `p.rs:2599-2602`) e non passa nella Vista | consenso, margini di Camera e Senato, spread | giocatore |

Le righe della tabella sono 135 [M: `grep -c -E '^\| [0-9]+ \|' docs/ricerca/sintesi/fatti-della-chiusura.md`].

## Fatti senza traccia

Quattro modi di perdere un fatto, in ordine di gravità per chi deve scrivere le voci.

**1. Non resta nulla.** Il dato che descrive il fatto è consumato nella funzione che lo produce.

- Evento risolto e opzione applicata, default compreso (3); scandalo chiuso in silenzio (11).
- Proposta di riforma decaduta (13); promesse scadute senza verifica (53).
- Atto del ministro eseguito (18), atto annunciato e non eseguito (20), cantiere fermo per margine (16). L'agenda è l'unico posto dove sta l'atto, e `esegui` la toglie per prima cosa (`min/agenda.rs:332`).
- Bilancio ricondotto al tetto del MES (25) e campagna tagliata (56): i due avvisi del piano 9, sotto.
- Valore non applicato per il vincolo del memorandum (30); arrivi elettorali scartati (117).
- Confermativo non richiesto (40): l'unica norma che cambia senza un esito.
- Mozione di sfiducia scartata (48).
- Situazione spenta (72): l'accensione apre un evento, lo spegnimento niente.
- Candidato escluso dal tetto (115), che per lo shock contraddice la spec `:360`.
- Guida non ceduta nella richiesta d'ufficio (104): la causa era già scritta in italiano.
- Giudizio da zero (109), spinta a destra delle situazioni accese (68), errore dei sondaggi rivelato dal voto (83).

**2. Resta uno stato che la Vista non porta.** Il fatto è leggibile nel core ma non arriva alla UI: programma del MES in corso e sua fine (6, 57), ristrutturazione e sua fine (7, 66), margine UE per l'energia (8), procedura per disavanzo eccessivo, controllo annuale e ammende (60, 61, 62), ministro sotto scandalo (10), minaccia di scissione come flag (50), leggi respinte e bloccate (35), sfiducia presentata e in attesa (133), redditi e giudizio dei gruppi (65, 67), serie degli indicatori (135). [V: `v.rs:22-102` non ha campi per nessuno di questi.]

**3. Resta solo un numero cambiato, senza causa.** La valence è il caso tipico: fuori dai test cambia in dieci punti (`p.rs:647`, `996`, `1379`, `1387`, `2532`, `3427`, `3711`, `4368`; `ia.rs:76`; `vita.rs:174`) [M: `grep -rn -E "valence (\+|-)?= " crates/sim-core/src/partita.rs crates/sim-core/src/partita/ia.rs crates/sim-core/src/partita/vita.rs`, tolte le righe dei test] e solo tre hanno una riga in `mosse` accanto (promessa, cambio di leader, scissione). Senza causa restano: legge respinta e decreto decaduto (23, 27, 35), referendum perso o fallito (41, 42), sfiducia respinta al promotore (46), rottura dell'impegno (108), eventi (5, 9, 10). Lo stesso vale per la posizione percepita (37), la cassa (74) e il fondo di riserva speso per un'emergenza lieve (2).

**4. Resta, ma dura poco o confonde due fatti.**

- Tutto ciò che sta in `mosse` vive un turno (`p.rs:550`): 25 punti di scrittura. Il registro «di ogni turno passato» della storia 16 (spec `:44`) oggi non ha dati da cui leggere oltre il turno prima [INF].
- EV ed ER vivono due turni (`parl/mod.rs:1044-1050`); le variazioni di soddisfazione due turni, quelle d'intesa fino alla chiusura dopo; la revisione della statistica una chiusura.
- Per tutta la partita restano solo: `legislatura.elezioni` (78), `serie` (135), `leggi_del_giocatore` (28), `parlamento.decaduti` (23).
- Il governo cade (100) senza che si salvi quale delle sette origini lo ha fatto cadere.
- Respinto per default e respinto per scelta sono la stessa causa, per le proposte (12) e per gli emendamenti (14).
- Governo formato dopo il voto e ribaltone sono lo stesso fatto nel codice (105); la riga del formatore c'è in un ramo solo.
- Un alleato che esce (93) non lascia una riga: si vede dai suoi dicasteri in interim, se ne aveva.
- Le dimissioni di un ministro (87) tengono la causa finché dura l'interim, poi si azzera (`min/mod.rs:313`).

## Cause già strutturate e cause solo testuali

**Strutturate e già serializzate.**

- `ministri::Causa`, 15 varianti (`min/mod.rs:59-75`), con il testo italiano nel `Display` (`min/mod.rs:77-97`). Accompagna ogni `Variazione` e le dimissioni. Copre i fatti 12, 20, 23, 24, 35, 47, 85, 86, 87, 93, 94, 126.
- `CausaIntesa`, 13 varianti (`parl/intesa.rs:15-29`), testo nel `Display` (`parl/intesa.rs:31-49`). Accompagna ogni `VariazioneIntesa` e l'ultimatum. Copre i fatti 6, 9, 14, 21, 31, 32, 46, 84, 89, 107, 108, 131, 132.
- `eventi::Causa`, 4 varianti con i valori letti (`ev/mod.rs:141-160`): `Soglia { misura, valore, soglia }`, `Rischio { livello }`, `Shock { serie, sigma, soglia }`, `Spread { pb, soglia }`. È strutturata nello stato, ma nella Vista passa solo come testo: `causa: a.causa.testo(&self.dati)` (`p.rs:4134`, `v.rs:144`).
- `EsitoVoto`, `EsitoReferendum`, `EsitoElezioni`, `Revisione`, `VoceSituazione`: numeri completi (conteggi, linee, affluenza, quorum, prima stima e riveduta, valore e soglie), nessun perché oltre ai numeri.

**Solo testo.**

- `MossaIa.testo` (`v.rs:180-186`): 25 punti di scrittura, una trentina di frasi. La causa è dentro la frase («…: la forbice include la sconfitta», «…: la sua intesa è sotto 40», «dopo il calo di 3,2 punti percentuali al voto»). Per regola non porta mai i valori attesi in cifre (`v.rs:180`).
- `EsitoVoto.decaduta` (`parl/mod.rs:106-107`): tre origini, tutte in testo (`p.rs:1044-1047`, `p.rs:3551`, `parl/mod.rs:1021`).
- `EsitoReferendum.decaduto` (`parl/regole.rs:60`): quattro testi (`p.rs:1307-1322`, `parl/mod.rs:1037`).
- `VoceEvento.causa`, `VoceEmendamento.motivo`, `VoceAgenda.motivo`, `VoceProposta.motivo`, `motivo_senza_agenda`.

**In mano nel punto del fatto e poi scartate.**

- I numeri dietro ogni delta: distanza e soglia (31), Δseggi (32), richiesta e offerta, media della soddisfazione, quota al voto e nel sondaggio (84), calo dell'indicatore (85). Si salva il delta, non gli addendi.
- Il valore al momento in cui una soglia è superata: soddisfazione alle dimissioni (87), intesa all'uscita e all'ultimatum (89, 93), distanza alla scissione (49). Fanno eccezione `Causa::Soglia`, `Causa::Spread` e la riga della minaccia di scissione, che il valore lo portano.
- I franchi tiratori: il voto estrae seggio per seggio (`parl/voto.rs:126-139`) e rende solo sì e no. Dalle linee e dai seggi si ricava il saldo, non il numero; e se nella stessa chiusura una scissione sposta i seggi (`vita.rs:179-190`, dopo l'aula) nemmeno quello [INF].
- L'opzione di un evento e il fatto che fosse il default (3).
- La crescita proposta e il limite del MES (25); le spese di campagna chieste (56).
- Crescita del reddito reale, aspettativa e contributo al giudizio (67).

## I due avvisi del piano 9

### Campagna tagliata

- **Dove.** `spendi_campagne`, `p.rs:2302-2322`, chiamata a `p.rs:594` nel solo turno del voto, dopo le scissioni e prima del motore. Per ogni partito non sciolto: `resto = p.cassa`; poi `retain_mut` sulle spese con `s.euro = s.euro.min(resto); resto -= s.euro; s.euro > 0` (`p.rs:2308-2313`). Le spese si tagliano in ordine: la prima intera, quella che non ci sta tutta ridotta al resto, le successive tolte [V].
- **Perché la cassa può non bastare.** Il comando «Campagna» controlla che il totale stia nella cassa di quel momento (`controlla_campagna`, `p.rs:2430-2438`). Dopo, la cassa può calare per due strade: un acquisto d'informazione, che controlla solo che la cassa copra il suo prezzo e non guarda la campagna già decisa (`compra`, `p.rs:2344-2353`, `p.rs:2399-2401`); una scissione del partito del giocatore nella stessa chiusura, che porta via `quota_scissione` della cassa (`vita.rs:165-166`), prima della spesa (`p.rs:581`, `p.rs:590-594`) [V]. Il piano 9 l'aveva previsto: «un acquisto nel turno del voto toglie denaro alla campagna senza codice nuovo» (`docs/superpowers/plans/2026-11-01-v1-piano-9-eventi-e-informazione.md:74`) [V].
- **Numeri in mano nel ciclo.** La cassa prima (`p.cassa`); ogni spesa chiesta, che però è sovrascritta sul posto (`s.euro = …`): il totale chiesto va sommato prima del `retain_mut` o è perso; le spese rimaste con bersaglio, leva ed euro; `speso = p.cassa - resto` (`p.rs:2317`) [V].
- **Cosa non è più in mano.** Il motivo del calo. Gli acquisti del turno sono stati azzerati all'inizio della chiusura (`p.rs:551`), 43 righe prima della spesa: a `p.rs:594` il core non sa più se e cosa il giocatore ha comprato [V]. La scissione invece è avvenuta poco sopra e ha lasciato la sua riga in `mosse` [V].
- **Traccia oggi.** La riga «Spende N euro: …» porta lo speso vero e i bersagli rimasti (`leg/campagna.rs:296-308`); non dice che era stato chiesto di più né quali bersagli sono caduti. Se non resta nulla da spendere la riga non c'è proprio (`p.rs:2314-2316`) [V].
- **Test.** `campagna_oltre_la_cassa_si_taglia`, `p.rs:6131-6156`: cassa 1.500.000 imposta a mano contro tre spese da 1.000.000, attesa la riga «Spende 1.500.000 euro: persuade A e B»; il suo commento attribuisce il calo alla scissione (`p.rs:6128-6129`). La strada dell'acquisto è in `crates/sim-core/tests/acquisti.rs:93-116` (`l_acquisto_toglie_alla_campagna`): campagna pari alla cassa di 3.000.000, sondaggio privato comprato dopo, riga «Spende 2.985.000 euro»; con la cassa pari al prezzo nessuna riga [V].
- **Chi.** Il ciclo vale per ogni partito. Per i partiti IA il taglio non dovrebbe scattare: `campagna_ia` divide esattamente la cassa (`ia.rs:183`) nella stessa chiusura, una riga prima della spesa (`p.rs:593-594`) [INF].

### Bilancio ricondotto al tetto del MES

- **Dove.** `ricondotto_al_tetto_mes`, `p.rs:1765-1792`. Un solo chiamante: `approva_bilancio`, `p.rs:1648-1661` (riga `1652`), a sua volta chiamata da `vota` quando la legge di bilancio è approvata (`p.rs:911-914`) [V; i chiamanti: M `grep -rn -E "ricondotto_al_tetto_mes|approva_bilancio" crates/sim-core/src`].
- **Quando succede.** Col programma in corso un bilancio oltre il tetto non si presenta nemmeno: `tetto_mes` lo rifiuta a `presenta_bilancio` (`p.rs:1727`) e all'emendamento accolto (`p.rs:1520`), con un messaggio che porta limite e crescita proposta (`p.rs:1750-1759`). Il caso nasce quando il programma comincia fra la presentazione e il voto, cioè nella stessa chiusura: `risolvi_eventi` avvia il MES a `p.rs:552` (opzione dell'evento «Aste scoperte», `p.rs:4214`), l'aula vota il bilancio dopo, a `p.rs:576`. Il commento alla funzione lo dice: «presentato prima del programma» (`p.rs:1645-1647`) [V]. Test: `il_bilancio_presentato_prima_del_mes_entra_ricondotto`, `crates/sim-core/tests/debito.rs:597-631` [V].
- **Numeri in mano.**
  - `oltre_il_tetto_mes(&b)` (`p.rs:1741-1747`) rende `(crescita, limite)`: la crescita della spesa netta del bilancio presentato e il limite, che è il maggiore fra il tetto UE del riepilogo e la crescita della sola bozza. A `p.rs:1768` il risultato è usato solo come `.is_some()`: i due numeri sono calcolati e non legati a un nome [V].
  - La bozza (`p.rs:1766-1767`), il bilancio presentato `b`, il bilancio che ne esce; `resta`, la quota di `b` che rimane (bisezione in 20 passi, `p.rs:1789-1791`); il caso limite in cui torna tutta la bozza perché nemmeno i dicasteri della bozza bastano (`p.rs:1786-1788`) [V].
  - Verso la bozza tornano solo le voci dei dicasteri (spesa, salari, acquisti, margine, `p.rs:1776-1781`); aliquote e fondo di riserva restano quelli presentati, salvo nel caso limite [V].
  - La fine del programma, `economia.programma_mes` [V].
- **Cosa non resta.** `approva_bilancio` toglie il presentato con `take` (`p.rs:1649`) e salva solo il ricondotto (`p.rs:1660`): dopo la chiusura la differenza fra ciò che il giocatore ha presentato e ciò che è in vigore non si ricostruisce. L'`EsitoVoto` del bilancio dice «approvata» con i conteggi (`p.rs:921-934`), nessuna riga in `mosse`, nessun campo nella Vista [V].
- **Chi.** Il governo in carica, del giocatore o avversario: la funzione non guarda chi presiede [V].

## Storia 116

La storia chiede due cose (spec `:168`): accanto al reddito reale di ogni fascia la crescita attesa, e una voce nell'esito quando delude, con l'esempio «reddito +0,3%, atteso +1%: giudizio in calo».

**Dove stanno i tre ingredienti.**

- **Reddito reale.** Per gruppo, non per fascia: `Gruppo.reddito` e `Gruppo.reddito_prima` (`el/mod.rs:366-367`), con `Reddito::reale()` = netto diviso l'indice del paniere della fascia (`el/redditi.rs:22-29`). Li scrive `aggiorna_redditi` (`el/redditi.rs:78`, `reddito_prima ← reddito` a `el/redditi.rs:109`), chiamata dal motore a `p.rs:1079-1080` [V]. I gruppi sono 360, con indice regione × 18 + condizione × 3 + fascia (`el/mod.rs:95`, `el/mod.rs:102-104`) [V].
- **Aspettativa.** Non è un campo: è una variabile locale di `giudizio`, la media di `Gruppo.crescite` letta prima di accodare la crescita del turno (`el/giudizio.rs:82-86`). `crescite` è nello stato (`el/mod.rs:371-372`), lunga al più `finestra_aspettative` turni, 8 nei dati (`dati/base/manopole.toml:422-426`), con tetto a 16 (`el/mod.rs:33`) [V].
- **Variazione del giudizio.** Non si salva: `*gp = lambda * *gp + fattore * reddito + …` (`el/giudizio.rs:139-143`) sovrascrive il valore. Non restano né il giudizio di prima né la parte dovuta al reddito [V].

**Il confronto esiste, ma è locale.** `giudizio` calcola la crescita reale trimestrale del gruppo, `100 × (reale ora / reale prima − 1)` (`el/giudizio.rs:87`), la accoda (`el/giudizio.rs:88-94`) e mette l'aspettativa nelle componenti del reddito come `prezzi − aspettativa` (`el/giudizio.rs:117`); la somma delle quattro componenti, divisa per `scala_reddito`, è `dy`, il cui segno sceglie l'asimmetria (`el/giudizio.rs:124-125`) [V]. Quindi «reddito contro atteso» è già calcolato per ognuno dei 360 gruppi a ogni chiusura, ma non esce dalla funzione: va esposto, non inventato [INF].

**Cosa manca, in ordine.**

1. **L'aggregato per fascia.** Nessuna funzione rende reddito reale, crescita o aspettativa per fascia. L'unico aggregato sul reddito reale è il Gini, pesato con gli elettori dei gruppi (`el/redditi.rs:180-201`; i pesi sono `Elettorato::elettori`, `el/mod.rs:387-389`) [V]. Aggregare su 20 regioni e 6 condizioni è da scrivere, e come pesare è una scelta [INF].
2. **L'aspettativa dopo la chiusura.** Dallo stato salvato si può rifare (media di `crescite` senza l'ultima) solo finché la finestra non è piena: quando lo è, la chiusura scarta il valore più vecchio (`el/giudizio.rs:93-94`), che faceva parte della media usata [INF]. Per avere i numeri esatti bisogna prenderli dentro `giudizio`.
3. **La variazione del giudizio.** Va presa nello stesso punto, prima della sovrascrittura.
4. **La Vista.** Non porta redditi, aspettative né giudizio (`v.rs:22-102`): anche la prima metà della storia («accanto al reddito reale di ogni fascia la crescita attesa») è da fare [V].
5. **La soglia del «delude».** Nel codice non c'è: il giudizio reagisce a ogni scarto, senza un livello sotto cui tacere [V].

**Due avvertenze sui documenti.**

- Il glossario e la spec dicono che l'aspettativa è «alzata dalle promesse» (`CONTEXT.md:292-293`; spec `:310`, `:379`). Nel codice le promesse non entrano: l'aspettativa è la sola media delle crescite (`el/giudizio.rs:82-86`), e la manopola delle promesse lo annota come scelta: «niente aspettativa» (`dati/base/manopole.toml:1394`) [V]. Una voce che citasse le promesse come causa direbbe una cosa che il motore non fa.
- L'unità. `crescite` è in punti di crescita reale **trimestrale** (`el/mod.rs:371`); l'esempio della storia («+0,3%, atteso +1%») non dice se è trimestrale o annuo [V].

## Per il nostro gioco

- **Prendiamo:**
  - l'ordine di `chiudi_turno` come ordine delle voci: i passi da A a S sono già fissi e commentati (`p.rs:520-725`), e ogni fatto ha un punto preciso del codice in cui nasce;
  - le tre enum di causa che esistono già (`ministri::Causa`, `CausaIntesa`, `eventi::Causa`), serializzate e con il testo italiano pronto per le prime due;
  - gli esiti già strutturati (`EsitoVoto`, `EsitoReferendum`, `EsitoElezioni`, `Revisione`) come corpo delle voci dell'aula, dei referendum, del voto e della statistica;
  - la colonna «chi lo subisce» come base per raggruppare per impatto, nel senso della gerarchia a tre livelli di [[feedback-visivo-dello-stato]] (`docs/ricerca/sistemi/feedback-visivo-dello-stato.md:36`): l'inventario dice chi è colpito, non quanto.
- **Evitiamo:**
  - voci per singolo oggetto che crescono con la partita (`docs/ricerca/sistemi/feedback-visivo-dello-stato.md:46`). I candidati a moltiplicarsi, in questo inventario: una voce per ogni variazione di soddisfazione o d'intesa (fino a dieci ministri e a tre pressioni per alleato, a ogni turno: 84-86), una per rata di cantiere (15), una per gruppo o per regione nella storia 116 (67), una per ogni riga del governo avversario in apertura (128-132);
  - ricostruire le voci a posteriori dalla Vista o dallo stato: i fatti del gruppo 1 di «Fatti senza traccia» sono già consumati quando la chiusura finisce, e fra questi ci sono i due avvisi del piano 9;
  - affidare la causa a una frase: `mosse`, `decaduta`, `decaduto` e i motivi sono testo, e un testo non si ordina né si filtra;
  - citare come causa ciò che il motore non fa (le promesse nell'aspettativa).
- **Aperto** (decisioni di prodotto, non prese qui):
  - la criticità di ognuno dei 135 fatti, e quali restano fuori dall'esito perché sono movimenti continui (59, 64, 65, 69, 74, i ritorni verso la base);
  - se un fatto che colpisce il governo avversario o un suo alleato è una voce per il giocatore all'opposizione, e con quanto dettaglio: la storia 90 gli dà l'intesa in tre fasce e non le agende prima dell'esecuzione (spec `:136`), ma oggi sotto un governo IA non si vede nemmeno un alleato che esce (93) o un evento risolto col default (3);
  - se le mosse dell'apertura (119-134), che nascono dopo `turno + 1`, appartengono all'esito del turno chiuso o all'apertura del turno nuovo: oggi stanno nello stesso vettore;
  - lo shock escluso dal tetto (115): la spec `:360` lo vuole nell'esito, il codice non lo registra;
  - gli stati che la Vista non porta (MES, ristrutturazione, procedura UE, sfiducia in attesa, ministro sotto scandalo): se entrano nell'esito come voci, nella Vista come stato, o in tutti e due;
  - per la storia 116: come si aggrega per fascia, sotto quale scarto la voce tace, in che unità si mostra la crescita, e che cosa fare della frase «alzata dalle promesse» del glossario;
  - quanto dura il registro e da dove legge: oggi oltre il turno prima restano solo elezioni, serie degli indicatori, leggi del giocatore e decreti decaduti.

## Collegamenti

- [[feedback-visivo-dello-stato]]: gerarchia delle notifiche per impatto e divieto di notifiche per singolo oggetto; è il riferimento che governa questo inventario.
- [[movimento-e-feedback]]: gli esiti del turno rivelati a ritmo, una voce alla volta.
- [[crisi-ed-eventi]]: situazioni con isteresi ed eventi, i passi B, I e R della tabella.
- [[consiglieri-e-informazione-imperfetta]]: prima stima e revisione (fatto 70), errore dei sondaggi (83).
- [[iter-legislativo-italiano]]: le regole dell'aula dietro i fatti 21-48.
- [[vita-dei-partiti]]: scissione, protesta, scioglimento (49, 50, 81, 120).
- [[regole-fiscali-ue]]: la procedura per disavanzo eccessivo, che oggi non ha traccia nella Vista (60-62).
- [[voto-economico-e-retrospettivo]]: il giudizio e l'asimmetria dietro la storia 116.
- [[salvataggi-e-riavvolgimento]]: il registro con «torna al turno N» e i salvataggi per turno.
- [[situazioni-ed-eventi-al-turno-zero]]: le soglie delle situazioni e la crisi del debito.
- [[cursori-dello-scenario]]: l'altra sintesi fatta leggendo il codice riga per riga, stesso metodo.
- [[decisioni]]: dove andranno le scelte sulla criticità e sulle domande aperte.
