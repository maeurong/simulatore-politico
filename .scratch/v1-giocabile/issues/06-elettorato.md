Type: grilling
Status: resolved
Blocked by: -
Assignee: Mario
Label: wayfinder:grilling
Map: ../map.md

## Question

Come è fatto l'elettorato della v1: gruppi aggregati con stato proprio, elettori campione derivati dai gruppi, o entrambi (tensione aperta in docs/ricerca/sistemi/cittadini-simulati.md:39 e docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:46)? Quanti gruppi, definiti da cosa (reddito, età, regione, ideologia), e come l'opinione diventa voto?

## Answer

Grilling con Mario del 17/10/2026, tre giri, tutte le raccomandazioni accettate; un cambio di Mario: tre fasce di reddito invece di due. Scioglie la tensione fra `docs/ricerca/sistemi/cittadini-simulati.md:39` e `docs/ricerca/sistemi/elettorato-e-gruppi-di-voto.md:46` a favore dell'aggregato, come la raccomandazione convergente 1 (`docs/ricerca/sintesi/raccomandazioni-convergenti.md:14`). Termini nuovi in `CONTEXT.md`: gruppo, condizione, fascia di reddito, interessi, elettore campione; posizione estesa agli elettori.

**Gruppi come motore.** L'elettorato è fatto di 360 **gruppi**: 6 condizioni (dipendente privato, dipendente pubblico, autonomo, pensionato, disoccupato o inattivo, studente) × 3 fasce di reddito (bassa, media, alta) × 20 regioni. Le regioni servono perché con la 180 il Senato riparte i seggi regione per regione e il tetto 113 lavora sulle cifre regionali (`docs/ricerca/modelli/sistemi-elettorali.md:46`). La posizione politica non definisce il gruppo: dentro ogni gruppo è un istogramma sui 7 punti dell'asse (7 quote, 2.520 numeri in tutto), così un gruppo spaccato in due resta visibile.

**Elettori campione come finestra.** Nessuno stato proprio: tre persone inventate, a richiesta, sul gruppo cliccato, generate con seme fisso. Si ricavano da ciò che dice il sondaggio, non dallo stato vero, altrimenti bucherebbero l'informazione imperfetta. Il motivo mostrato è il termine che pesa di più nell'utilità stimata.

**Dall'opinione al voto: logit** (`docs/ricerca/modelli/voto-spaziale.md:41`). Per ogni gruppo e punto dell'asse, l'utilità di un partito somma:
- distanza: `−β·(distanza)²` dalla posizione percepita del partito;
- valence del partito (leader, scandali, campagna);
- giudizio retrospettivo sugli interessi del gruppo, solo per i partiti di governo, più severo sui peggioramenti (`docs/ricerca/modelli/voto-economico-e-retrospettivo.md:46`);
- fedeltà al voto precedente.
L'astensione è un'alternativa del logit con utilità propria, legata alla fiducia: l'affluenza è endogena.

**Chi paga il giudizio.** Una quota fissa va al partito del Presidente del Consiglio, il resto ai partiti che reggono i dicasteri legati agli interessi toccati: la sanità che peggiora pesa su chi ha la Sanità. La spartizione dei dicasteri (ticket 05) diventa anche una scelta su chi si prende la colpa.

**Interessi.** Tabella fissa condizione → indicatori (per il pensionato: pensioni e sanità), più il reddito disponibile della fascia, che vale per tutti. Niente salienza mobile nella v1: arriva con gli eventi, punto di aggancio.

**Una sola dimensione posizionale.** Lo stesso asse a 7 punti dei partiti; le issue entrano come interessi materiali, non come assi. Ogni misura ha una posizione sull'asse ed effetti sugli indicatori.

**Posizione percepita.** La posizione dichiarata di un partito resta fissa; quella percepita di un partito di governo scivola piano verso le misure che firma (ministro proponente) o vota. Così la posizione di una misura sposta voti, e anche il partito del giocatore può spostarsi dal centro senza averlo deciso.

**Dinamica delle posizioni degli elettori.** Ancora di Friedkin-Johnsen verso la posizione di partenza del gruppo, più spinte di eventi e campagna e un po' di rumore (`docs/ricerca/modelli/opinion-dynamics.md:13`). Niente attrazione fra gruppi a confidenza limitata: la validazione manca (`docs/ricerca/modelli/opinion-dynamics.md:27`); resta un punto di aggancio.

**Composizione mobile.** Ogni turno le quote dei gruppi si muovono con lavoro e reddito: chi cambia gruppo porta con sé la sua distribuzione di posizioni. Ogni anno si muovono con l'età (lavoratori verso pensionati, studenti verso il lavoro).

**Cosa vede il giocatore.** Mai lo stato vero: sondaggi con tre strati di errore (`docs/ricerca/modelli/sondaggi-ed-errore.md:55`), con incroci per condizione, fascia e regione, il cui errore cresce quanto più è piccolo il sottocampione.

Parametri (β, ancora, fedeltà, quota di colpa del Presidente del Consiglio, pesi degli interessi) provvisori, al bilanciamento. Effetti sulla mappa:
- ticket nuovo research «Dati per tarare i gruppi»;
- ingresso al Modello economico (07): disoccupazione e reddito per fascia e per regione, gli indicatori degli interessi;
- ingresso al Giro del turno (09): sondaggi e incroci, elettori campione a richiesta;
- nebbia nuova: campagna elettorale; punti di aggancio: salienza mobile, attrazione fra gruppi.
