Type: prototype
Status: resolved
Blocked by: 06, 08
Assignee: Mario
Label: wayfinder:prototype
Map: ../map.md

## Question

Com'è fatta la scelta dello scenario e l'editor? Scenario di base o su misura, nuovo o caricato da file (storia 2); sei cursori più i menu della legge elettorale e delle disposizioni (storia 3), col cursore che si ferma al limite di taratura (storia 5); riepilogo dal vivo con consenso atteso per posto, maggioranza attesa, spread, interessi, disavanzo e i tre avvisi (storie 4 e 6, `RiepilogoScenario`, `Avviso`); salvataggio con un nome (storia 7). Pagina senza cornice, con «‹ Edicola».

Posto nell'ossatura e regole comuni (cornice, sommario, dettaglio a lato, conferme sul posto, tono grave a tutto il turno): [Ossatura e navigazione](07-ossatura-e-navigazione.md). Un prototipo HTML usa e getta in `.scratch/v1-piano-11-ui/prototipi/`, nutrito con le [Viste campione](06-viste-campione.md), Mario sceglie fra varianti.

## Answer

Deciso con Mario l'11/11/2026 in un giro su un prototipo navigabile: [14-editor.prototype.html](../prototipi/14-editor.prototype.html), che si apre sulla forma decisa; le scartate restano raggiungibili dalla barra in basso. Dati da `genera/` ([viste/14-editor.json](../viste/14-editor.json)): `riepilogo()` vero su otto scenari di prova e su nove passi per cursore; fra un passo e l'altro, o con più cursori mossi, il prototipo approssima e lo dichiara.

### Forma: A · Banco

Pagina senza cornice, come l'edicola: «‹ Edicola», «Il Pungolo · Nuova partita» e, a destra, lo stato dello scenario («Italia di oggi», «Su misura: nuovo, senza nome · non salvato»). Sotto, le dodici colonne col **dettaglio a lato**: a sinistra l'editor che scorre, a destra il riepilogo fermo.

- **Scenario**: una fila di bottoni, «Italia di oggi», poi gli scenari salvati, poi «+ Nuovo su misura». L'Italia di oggi non si cambia: toccare un cursore apre uno scenario su misura nuovo, senza nome.
- **Il Paese al turno zero**: i sei cursori in due colonne. Valore grande a destra con l'unità, estremi della taratura ai lati, una tacca rossa sul valore di oggi; il cursore mosso ha il filo giallo a sinistra. Al limite dell'intervallo il cursore si ferma e dice «fermo al limite della taratura» (storia 5).
- **Le regole del voto**: menu della legge elettorale (Legge 180, proporzionale al 3%, proporzionale puro) e sotto i nove parametri come bottoni affiancati, il valore della legge scelta segnato. Un parametro ritoccato dice cosa direbbe la legge; cambiare legge toglie i ritocchi. Sotto, la Costituzione: durata della legislatura e numero dei parlamentari.
- **Le leggi in vigore**: le 35 disposizioni ordinarie per dicastero (l'autonomia differenziata sotto «Regioni»), bottoni affiancati fino a quattro gradini e un menu oltre; il gradino di oggi col pallino. Una disposizione cambiata dice il valore di oggi e il suo saldo.
- **Riepilogo dal vivo** a lato (storie 4 e 6): avvisi in testa, poi spread, interessi e disavanzo con lo scarto da oggi, poi le disposizioni cambiate come peso a parte («il disavanzo del turno zero non le contiene ancora», `saldo_disposizioni_pil`), poi i sei posti: quota vostra, quota della coalizione, seggi di Camera e Senato, maggioranza, e i partner della coalizione. Le righe senza maggioranza sono a retino rosso.
- **Avvisi**: «senza maggioranza» coi posti, crisi del debito con spread e soglia, in rosso; le situazioni accese con un filo grigio, perché «Caro energia» è accesa anche nell'Italia di oggi e l'avviso c'è sempre.
- **Salva e fonda**, in fondo al dettaglio: nome e «Salva» (storia 7); un nome già usato diventa sul posto «Conferma: sostituisci «…»» e «Annulla». Salvare non serve per giocare. «Fonda il partito ›» porta alla [Fondazione](15-fondazione.md) con lo scenario com'è.

### Casi

- **Legge elettorale che non sta in piedi** (`riepilogo_scenario` rifiuta, per esempio premio alla coalizione senza coalizioni): rifiuto sul posto sopra i parametri col messaggio del core, il riepilogo resta all'ultimo valido e si spegne, «Fonda il partito» è spento ([Contratto del guscio](12-contratto-del-guscio.md): errori in due specie).
- **Uscita con modifiche non salvate**: «‹ Edicola» diventa sul posto «Esci comunque» e «Resta». Le bozze vivono solo in memoria.
- **Inflazione di partenza**: il cursore resta, con una riga sotto: non cambia il riepilogo, pesa dal primo trimestre sul tasso della BCE e sul bilancio dell'anno dopo (`crates/sim-core/src/su_misura.rs:295-301`, `crates/sim-core/src/economia/finanza.rs:177`).
- **Scenari da file** (storia 2): solo quelli della cartella del gioco, l'elenco di `scenari`. Niente finestra per aprire un file qualsiasi.

### Guscio e dati

- **Catalogo dell'editor dal core.** Nessun comando porta alla UI intervalli dei cursori, leggi elettorali coi parametri e disposizioni coi gradini; stanno in `DatiGioco` (`scenario()`, `elezioni().impostazioni`, `disposizioni()`). Una funzione nuova di sola lettura in `sim-core`, accanto a `riepilogo` (`crates/sim-core/src/riepilogo.rs:118`), col suo tipo esportato in TypeScript, e il passacarte `catalogo_scenario`. Li porta la fetta 11a nella sua prima PR: legge dati già caricati e non aggiunge stato, quindi niente piano di core a parte (assunzione dichiarata; deroga al «pronta con dati su main» di [Fette](10-fette.md)). È l'unico tocco al core del piano 11 fin qui. Per legge elettorale servono i gradini dei nove parametri: `genera/` li ricava da `con_su_misura`.
- **Riepilogo a ogni movimento**: circa 60-70 ms in release. Una chiamata in volo per volta, l'ultima vince; durante il trascinamento si ridisegna solo il riepilogo.
- Numeri del formato `Intl` di `t()` ([Testi esterni](03-testi-esterni.md)); il meno è il segno meno.

### Scartato

B · Numero zero (il riepilogo come prima pagina di prova, cursori in un banco di regia in basso); C · Fascicolo (cinque passi in un sommario, striscia fissa dei numeri chiave); catalogo dentro `riepilogo_scenario` o composto dal guscio; togliere il cursore dell'inflazione o chiedere al core di farlo vedere nel riepilogo; «Apri un file…» con la finestra di sistema.
