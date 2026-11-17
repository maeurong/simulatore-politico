# Piano 11f · I testi del Pungolo, da tagliare

Qui stanno i testi del Pungolo della fetta 11f: i titoli della prima pagina, le didascalie delle vignette e le battute a biro dello schema dello Stato. Li ha scritti un agente; li tieni, li tagli o li riscrivi tu. Ciò che tagli resta senza testo: il titolo lascia il posto all'occhiello, la didascalia e la battuta tacciono (ticket 09, spec:321). Nel tono grave nessuno di questi testi si vede.

## Le sette regole

1. Due tempi: frase secca, poi la coda.
2. Mai cifre, mai un fatto che l'occhiello non dice.
3. Terza persona: il Pungolo non dà del tu e non tifa, vale col giocatore al governo e all'opposizione.
4. Il bersaglio è il palazzo, mai chi subisce.
5. Mai destra o sinistra, mai politici reali, niente nomi di partiti, ministri, dicasteri o regioni.
6. Una riga di colonna: al massimo 55 caratteri, spazi compresi (45 per le battute a biro).
7. I sette tipi con due esiti opposti hanno titoli che reggono tutti e due.

## Come tagliare

Nella colonna «Mario»: vuota = tieni, `x` = taglia, oppure scrivi il testo nuovo. L'occhiello d'esempio viene da `.scratch/v1-piano-11-ui/chiavi-11f.json`, accorciato a circa 120 caratteri (i «…» segnano i tagli). Dove il JSON non ne aveva uno, l'ho ricavato dalla `format!` del core: è segnato «(ricavato dal codice)» e i valori che dipendono dalla partita stanno fra ‹ ›.

## Titoli

| Chiave | Occhiello d'esempio | # | Titolo | Mario |
|---|---|---|---|---|
| `evento_risolto\|situazione` | Recessione, «Non intervenire» per default | 1 | Caso chiuso. Il comunicato era pronto da giorni | |
| `evento_risolto\|situazione` |  | 2 | Scelta fatta. Ora si cerca chi l’ha fatta | |
| `evento_risolto\|emergenza` | Emergenza idrogeologica in Emilia-Romagna, «Attingere al fondo di riserva» per default; 305.700.000 euro dal fondo di riserva … | 1 | Emergenza archiviata. Gli stivali tornano nell’armadio | |
| `evento_risolto\|emergenza` |  | 2 | Il palazzo decide. Il territorio prende nota | |
| `evento_risolto\|emergenza` |  | 3 | Pratica d’urgenza evasa. Con calma, s’intende | |
| `evento_risolto\|emergenza` |  | 4 | Fine dell’emergenza. Comincia quella dei comunicati | |
| `evento_risolto\|scandalo` | Scandalo su un ministro (Giulia Ferri), «Difenderlo» per default (ricavato dal codice) | 1 | Lo scandalo si chiude. L’armadio, per prudenza, no | |
| `evento_risolto\|scandalo` |  | 2 | Si volta pagina. Senza leggerla, se possibile | |
| `evento_risolto\|shock` | Rincaro improvviso dell'energia, «Chiedere il margine UE per l'energia per quest'anno, conta dal 2029» | 1 | Lo shock passa. Il governo giura di averlo previsto | |
| `evento_risolto\|shock` |  | 2 | Risposta allo shock. Con calma, e per iscritto | |
| `evento_risolto\|debito` | Aste scoperte, «Chiedere il programma del MES» | 1 | Il debito ha una risposta. I mercati, una domanda nuova | |
| `evento_risolto\|debito` |  | 2 | Decisione sul debito. Il palazzo trattiene il fiato | |
| `mes_avviato` | Programma del MES di 8 trimestri, finisce a aprile 2032; vincolati Età pensionabile: 70 anni; … | 1 | I conti hanno un tutore. Il governo, un alibi | |
| `mes_avviato` |  | 2 | Firmato il memorandum. Il governo ha un correttore | |
| `ristrutturazione` | Debito tagliato del 50%, 1.983.575.523.475 euro; premio di 300 pb sullo spread; redditi delle fasce media e alta giù … | 1 | Il debito si accorcia. La credibilità, pure | |
| `ristrutturazione` |  | 2 | Taglio netto al debito. I mercati si segnano la data | |
| `proposta_respinta` | 3 proposte respinte | 1 | Proposte respinte. Il cestino ringrazia | |
| `proposta_respinta` |  | 2 | Il ministro propone. Il cassetto dispone | |
| `emendamento_respinto` | 3 emendamenti respinti | 1 | Emendamenti respinti. Il testo si sente perfetto | |
| `emendamento_respinto` |  | 2 | Bocciate le modifiche. I proponenti fingono sorpresa | |
| `atto_saltato` | Il dicastero Ambiente ed Energia non esegue «Riempimento accelerato delle riserve di gas», annunciato in agenda | 1 | Annunciato, poi saltato. L’agenda era a matita | |
| `atto_saltato` |  | 2 | L’atto salta. L’annuncio, invece, resta agli atti | |
| `cantiere_fermo` | 6 cantieri fermi | 1 | Cantieri fermi. Le gru si godono il panorama | |
| `cantiere_fermo` |  | 2 | Lavori sospesi. Il nastro tagliato resta per terra | |
| `cantiere_fermo` |  | 3 | Il cantiere si ferma. Il cartello dei lavori resiste | |
| `cantiere_fermo` |  | 4 | Ruspe ferme. Avanzano a pieno ritmo le scuse | |
| `cantiere_finito` | 2 cantieri finiti | 1 | Cantiere finito. Si cerca un ministro per la foto | |
| `cantiere_finito` |  | 2 | Lavori conclusi. Lo stupore è generale | |
| `legge_approvata` | Approvata «Sgravio contributivo nel Mezzogiorno: 15%»; Camera …; Senato … | 1 | La legge passa. Ora tocca scoprire cosa dice | |
| `legge_approvata` |  | 2 | Votata e approvata. Per applicarla, ripassare | |
| `legge_approvata` |  | 3 | Il Parlamento dice sì. Qualcuno, a voce bassa | |
| `legge_approvata` |  | 4 | Nuova legge approvata. Vecchie abitudini in ufficio | |
| `legge_respinta` | Respinta «Concorrenza nei servizi: mercato libero»; Camera …; Senato …; disposizioni bloccate per 2 turni | 1 | La legge non passa. Qualcuno festeggia, qualcuno conta | |
| `legge_respinta` |  | 2 | Bocciata in aula. Il testo torna da dove è venuto | |
| `legge_respinta` |  | 3 | Respinta. Gli autori la trovavano bellissima | |
| `legge_respinta` |  | 4 | L’aula dice no. Il comunicato era scritto per il sì | |
| `decreto_decaduto` | Decade «Decreto: fondi a Economia e Finanze per Incrocio delle banche dati fiscali»; Camera …; Senato … | 1 | Il decreto decade. Era urgente, ma senza fretta | |
| `decreto_decaduto` |  | 2 | Decreto scaduto. Andava consumato entro la data | |
| `bilancio_approvato` | Approvata la legge di bilancio; Camera …; Senato …; budget tagliato in termini reali a Economia e Finanze (-7,8%), … | 1 | Il bilancio passa. I conti, sulla parola | |
| `bilancio_approvato` |  | 2 | Bilancio approvato. Le tabelle restano un mistero | |
| `bilancio_approvato` |  | 3 | I conti quadrano. A guardarli da lontano | |
| `bilancio_approvato` |  | 4 | Sì alla manovra. Ogni ministro giura di aver vinto | |
| `bilancio_ricondotto` | Legge di bilancio ricondotta al tetto del MES: la spesa netta presentata cresceva del ‹x›%, il limite è del ‹y›%; … (ricavato dal codice) | 1 | Bilancio ricondotto al tetto. Il tetto non si discute | |
| `bilancio_ricondotto` |  | 2 | La manovra cala di taglia. Il sarto è il MES | |
| `bilancio_respinto` | Respinta la legge di bilancio; ‹voti di Camera e Senato› (ricavato dal codice) | 1 | Bocciato il bilancio. Si ricomincia dalla prima riga | |
| `bilancio_respinto` |  | 2 | La manovra non passa. I conti restano senza padrone | |
| `fiducia_respinta` | Respinta la fiducia su «Decreto: fondi a Infrastrutture e Trasporti per Manutenzione straordinaria delle strade locali» (ricavato dal codice) | 1 | Fiducia negata. Il governo prende atto, e il cappotto | |
| `fiducia_respinta` |  | 2 | Il governo chiede fiducia. L’aula chiede il conto | |
| `sfiducia_approvata` | Approvata la sfiducia individuale a Fabio Rossi (Sviluppo economico); Camera …; Senato …; il dicastero va in interim | 1 | Passa la sfiducia. Si libera qualche poltrona | |
| `sfiducia_approvata` |  | 2 | L’aula ha deciso. Il trasloco può cominciare | |
| `sfiducia_approvata` |  | 3 | Voto di sfiducia. Gli scatoloni erano già pronti | |
| `sfiducia_approvata` |  | 4 | La fiducia è finita. Restano i ringraziamenti di rito | |
| `sfiducia_respinta` | Respinta la sfiducia al governo; Camera 60 sì e 340 no; Senato 28 sì e 172 no; valence -0,10 a Partito del giocatore | 1 | Respinta la sfiducia. Il governo sopravvive a sé stesso | |
| `sfiducia_respinta` |  | 2 | La sfiducia non passa. Gli scatoloni tornano in cantina | |
| `valore_non_applicato` | Età pensionabile resta a ‹vigente›, non a ‹nuovo›, nonostante la legge approvata (ricavato dal codice) | 1 | Legge approvata, valore fermo. Decide il memorandum | |
| `valore_non_applicato` |  | 2 | Votata, non applicata. Il calendario lo tiene il MES | |
| `misura_decaduta` | 3 misure decadute | 1 | Misure decadute. Gli uffici archiviano senza lutto | |
| `misura_decaduta` |  | 2 | Cade la misura. Nessuno se ne ricorda il nome | |
| `confermativo_in_attesa` | Approvata senza i due terzi «‹legge›»; referendum confermativo a ‹mese anno›, non ancora richiesto (ricavato dal codice) | 1 | Riforma votata. L’ultima parola agli elettori | |
| `confermativo_in_attesa` |  | 2 | Costituzione ritoccata. In attesa del collaudo | |
| `referendum_votato` | Referendum abrogativo su «Tetto al contante: 5.000 €» fallito senza quorum; … affluenza del 26,8% | 1 | Il quesito è sciolto. Le dichiarazioni no | |
| `referendum_votato` |  | 2 | Il quesito ha la sua risposta. I partiti, le loro scuse | |
| `referendum_chiesto` | Chiede il referendum su Età pensionabile, si vota a aprile 2031 | 1 | Chiesto il referendum. Il palazzo scopre gli elettori | |
| `referendum_chiesto` |  | 2 | Referendum in vista. I partiti ripassano il quesito | |
| `firme_raccolte` | Indetto il referendum abrogativo su Età pensionabile, si vota a aprile 2031 | 1 | Firme depositate. Il palazzo conta le penne | |
| `firme_raccolte` |  | 2 | Le firme ci sono. Il palazzo comincia a leggerle | |
| `scissione` | Scissione in Partito del giocatore: nasce Lista dei Territori, sulla posizione 2,5, con 4 deputati, 2 senatori … | 1 | Nasce un partito nuovo. Lo stesso, ma in due | |
| `scissione` |  | 2 | Scissione. Ognuno porta via le sue sedie | |
| `minaccia_di_scissione` | Minaccia di scissione in Partito del giocatore, a 0,5 punti dalla posizione dichiarata | 1 | Aria di scissione. Si contano i fedeli | |
| `minaccia_di_scissione` |  | 2 | Il partito litiga. Per ora a porte chiuse | |
| `minaccia_rientrata` | Rientra la minaccia di scissione in Partito del giocatore (ricavato dal codice) | 1 | Scissione rientrata. Si torna a litigare in famiglia | |
| `minaccia_rientrata` |  | 2 | Pace fatta nel partito. Fino al prossimo congresso | |
| `promessa_verificata` | 2 promesse verificate | 1 | Promesse alla verifica. Qualcuno ha la memoria lunga | |
| `promessa_verificata` |  | 2 | Si controllano le promesse. Gli archivi non perdonano | |
| `promesse_scadute` | 6 promesse scadute | 1 | Promesse scadute. Nessuno ricorda di averle fatte | |
| `promesse_scadute` |  | 2 | Le promesse scadono. I manifesti restano attaccati | |
| `promessa_fatta` | 7 promesse fatte | 1 | Nuove promesse. La carta, intanto, regge tutto | |
| `promessa_fatta` |  | 2 | Promessa fatta. Il calendario si fa serio | |
| `spesa_di_campagna` | 7 spese di campagna | 1 | Si spende per la campagna. I manifesti sorridono | |
| `spesa_di_campagna` |  | 2 | Spese di campagna. Ogni sorriso ha il suo preventivo | |
| `campagna_tagliata` | Campagna tagliata: chiesti 1.000.000 euro, spesi 985.000 | 1 | Campagna ridimensionata. Il sorriso del manifesto, pure | |
| `campagna_tagliata` |  | 2 | Si taglia la campagna. I volantini a mezzo servizio | |
| `mes_finito` | Finito il programma del MES: cadono i vincoli del memorandum (ricavato dal codice) | 1 | Finisce il programma. Il governo ritrova le chiavi | |
| `mes_finito` |  | 2 | Via il memorandum. Ora le colpe sono tutte di casa | |
| `controllo_ue` | Controllo UE sul 2027: spesa netta sotto il percorso di 0,5 punti di PIL, al netto della clausola … | 1 | L’Europa fa i conti. Il governo, gli scongiuri | |
| `controllo_ue` |  | 2 | Arriva la pagella europea. Si legge in silenzio | |
| `procedura_ue` | Chiusa la procedura per disavanzo eccessivo, che era sospesa | 1 | Procedura europea in movimento. Si rilegge la lettera | |
| `procedura_ue` |  | 2 | L’Europa aggiorna la pratica. Il governo, il comunicato | |
| `ammenda_ue` | Pagata un'ammenda UE di ‹importo› euro, pari a ‹quota›% del PIL (ricavato dal codice) | 1 | Arriva la multa europea. La colpa è di chi c’era prima | |
| `ammenda_ue` |  | 2 | Ammenda pagata. Lo sconto non era previsto | |
| `ristrutturazione_finita` | Finita la ristrutturazione del debito: il premio sullo spread è rientrato e i redditi delle fasce media e alta non calano più (ricavato dal codice) | 1 | Finita la cura del debito. Il paziente chiede il conto | |
| `ristrutturazione_finita` |  | 2 | Lavori finiti sul debito. Si smonta l’impalcatura | |
| `reddito_deluso` | 3 fasce di reddito deluse | 1 | Redditi delusi. Il governo parla di ripresa percepita | |
| `reddito_deluso` |  | 2 | Il portafoglio protesta. Il palazzo cambia grafico | |
| `reddito_deluso` |  | 3 | Attese tradite. Il ministero prepara una slide | |
| `reddito_deluso` |  | 4 | Promessa la crescita. Consegnata la spiegazione | |
| `situazione_accesa` | Si accende la situazione «Caro energia»: scarto dei prezzi dell'energia a ‹valore› (ricavato dal codice) | 1 | Si accende la spia. Il palazzo cerca l’interruttore | |
| `situazione_accesa` |  | 2 | Allarme acceso. Prima riunione: chi l’ha acceso | |
| `situazione_spenta` | Si spegne la situazione «Caro energia»: scarto dei prezzi dell'energia a 3,9 | 1 | Spia spenta. Il merito trova molti padri | |
| `situazione_spenta` |  | 2 | Si spegne l’allarme. Il palazzo si riprende la scena | |
| `crisi_del_debito` | Spread a ‹pb› pb: si aprono le «Aste scoperte» (ricavato dal codice) | 1 | Lo spread bussa. Il palazzo finge di non essere in casa | |
| `crisi_del_debito` |  | 2 | Mercati in fuga. Il governo cerca l’uscita di sicurezza | |
| `risultato_del_voto` | Elezioni politiche: affluenza del 71,4%; Partito del giocatore al 16,4% con 76 seggi alla Camera …; primo partito Sinistra … | 1 | Gli elettori hanno parlato. I partiti traducono | |
| `risultato_del_voto` |  | 2 | Spoglio finito. Comincia quello dei leader | |
| `incarico` | Incarico di formare il governo a Partito del giocatore | 1 | Arriva l’incarico. Cominciano le telefonate | |
| `incarico` |  | 2 | Incarico conferito. Il totoministri apre le scommesse | |
| `incarico` |  | 3 | C’è l’incaricato. Ora servono gli amici | |
| `incarico` |  | 4 | Incarico in tasca. La squadra ancora no | |
| `fine_della_partita` | Partito del giocatore resta senza seggi alla Camera e al Senato: la partita finisce | 1 | Il partito esce di scena. Il guardaroba resta aperto | |
| `fine_della_partita` |  | 2 | Seggi azzerati. Il partito si dà alla memorialistica | |
| `partito_sciolto` | Si scioglie | 1 | Il partito chiude. Le bandiere vanno in soffitta | |
| `partito_sciolto` |  | 2 | Si scioglie il partito. Restano gli ex, numerosi | |
| `minaccia_di_dimissioni` | 3 ministri minacciano le dimissioni | 1 | Dimissioni pronte. In bella copia, per ora | |
| `minaccia_di_dimissioni` |  | 2 | Ministri sul piede di partenza. Il piede resta fermo | |
| `ministro_esce` | 6 ministri escono | 1 | Si svuotano scrivanie. Le targhe sulle porte si svitano | |
| `ministro_esce` |  | 2 | Si esce dal governo. Le auto blu restano in cortile | |
| `ultimatum_posto` | Sinistra pone l'ultimatum al governo | 1 | Ultimatum al governo. La pazienza ha una scadenza | |
| `ultimatum_posto` |  | 2 | L’alleato alza la voce. Il governo abbassa la sua | |
| `ultimatum_posto` |  | 3 | Ultimatum sul tavolo. Il tavolo traballa | |
| `ultimatum_posto` |  | 4 | L’alleato mette le condizioni. Per iscritto, in rosso | |
| `ultimatum_ritirato` | Centrodestra ritira l'ultimatum al governo | 1 | Ritirato l’ultimatum. L’offesa resta in archivio | |
| `ultimatum_ritirato` |  | 2 | L’alleato torna a sorridere. A denti stretti | |
| `cambio_di_leader` | Nuovo leader: Anna Barbieri | 1 | Cambia la guida. In sede si sostituiscono i ritratti | |
| `cambio_di_leader` |  | 2 | Il partito cambia faccia. Lo specchio è lo stesso | |
| `frangia_esce` | Esce dalla maggioranza (ricavato dal codice) | 1 | Esce una frangia. La maggioranza si accorcia | |
| `frangia_esce` |  | 2 | Esce un pezzo di maggioranza. Il resto finge di niente | |
| `alleato_esce` | Sinistra lascia il governo; dicasteri in interim: Economia e Finanze, Sviluppo economico, Istruzione e Ricerca, Interno | 1 | L’alleato se ne va. Lascia le chiavi e le colpe | |
| `alleato_esce` |  | 2 | Fuori un alleato. Il governo ricalcola il percorso | |
| `rimpasto` | Rimpasto: Sanità a ‹partito›, Interno a ‹partito› (ricavato dal codice) | 1 | Rimpasto. Stesso governo, scrivanie diverse | |
| `rimpasto` |  | 2 | Si rimescolano le carte. Il mazzo è sempre quello | |
| `crisi_aperta` | Crisi di governo: senza Sinistra la coalizione non ha la maggioranza | 1 | Crisi di governo. Il corridoio diventa la sala grande | |
| `crisi_aperta` |  | 2 | Crisi aperta. Comincia la stagione dei caffè lunghi | |
| `coalizione_allargata` | Entra al governo ‹partito› (o: Allarga la coalizione a ‹partito›) (ricavato dal codice) | 1 | La coalizione si allarga. Si aggiungono sedie al tavolo | |
| `coalizione_allargata` |  | 2 | Nuovi alleati a bordo. Il programma si stringe | |
| `governo_caduto` | Cade il governo di Sinistra | 1 | Cade il governo. Il portone resta aperto al prossimo | |
| `governo_caduto` |  | 2 | Il governo cade. Restano in piedi gli uscieri | |
| `camere_sciolte` | Camere sciolte, si vota a ‹mese anno› (ricavato dal codice) | 1 | Camere sciolte. I parlamentari riscoprono il collegio | |
| `camere_sciolte` |  | 2 | Si torna al voto. Le promesse tornano in vetrina | |
| `guida_rifiutata` | ‹partito› non ti cede la guida del ribaltone (ricavato dal codice) | 1 | Ribaltone in sospeso. Nessuno cede il volante | |
| `guida_rifiutata` |  | 2 | Si cambia governo, forse. Il capotavola non si tocca | |
| `governo_formato` | Nasce il governo di Partito del giocatore dopo il voto, con Centrodestra e Destra; dicasteri: … | 1 | Nasce il governo. Il giuramento regge fino a sera | |
| `governo_formato` |  | 2 | Governo nuovo. Le auto blu sono state lavate | |
| `governo_formato` |  | 3 | Ministri nominati. Ognuno cerca il suo ufficio | |
| `governo_formato` |  | 4 | La squadra è fatta. Resta da trovare il campo | |
| `programma_dichiarato` | Programma di governo: 1 disposizione, Sgravio contributivo nel Mezzogiorno | 1 | Ecco il programma. Le scadenze, a parte | |
| `programma_dichiarato` |  | 2 | Il governo dice cosa farà. Il quando, a voce | |
| `rottura_dell_impegno` | 5 rotture dell'impegno | 1 | Impegno rotto. La colla era elettorale | |
| `rottura_dell_impegno` |  | 2 | Patti traditi. Il palazzo parla di aggiustamenti | |
| `evento_nato\|situazione` | Recessione | 1 | Nuovo problema sul tavolo. Il tavolo si allunga | |
| `evento_nato\|situazione` |  | 2 | Il paese segnala un guasto. Il governo cerca il manuale | |
| `evento_nato\|emergenza` | Emergenza idrogeologica in Emilia-Romagna, costo dell'intervento 5.000.000.000 euro | 1 | Emergenza sul territorio. A palazzo, l’unità di crisi | |
| `evento_nato\|emergenza` |  | 2 | Arriva l’emergenza. Il conto arriva subito dopo | |
| `evento_nato\|emergenza` |  | 3 | Il territorio chiede aiuto. Il governo, un preventivo | |
| `evento_nato\|emergenza` |  | 4 | Emergenza in corso. Si sceglie la giacca a vento | |
| `evento_nato\|scandalo` | Scandalo su un leader (Bruno Castellani, Estrema destra) | 1 | Scoppia lo scandalo. Torna di moda il garantismo | |
| `evento_nato\|scandalo` |  | 2 | Un caso imbarazzante. Le smentite si scrivono a matita | |
| `evento_nato\|shock` | Crollo della domanda interna (ricavato dal codice) | 1 | Arriva lo shock. Il governo lo chiama congiunturale | |
| `evento_nato\|shock` |  | 2 | Colpo all’economia. Si cerca un colpevole lontano | |
| `evento_nato\|debito` | Aste scoperte (ricavato dal codice) | 1 | Le aste vanno deserte. Il governo rifà il listino | |
| `evento_nato\|debito` |  | 2 | Titoli senza compratori. Il palazzo abbassa la voce | |
| `shock_assorbito` | Crollo della domanda estera non diventa un evento: ne nasce uno solo a turno. La serie si muove comunque … | 1 | Lo shock passa di lato. Nessuno lo annuncia | |
| `shock_assorbito` |  | 2 | Uno shock in meno in pagina. I conti lo sentono uguale | |
| `valore_in_vigore` | Credito d'imposta nella ZES unica passa a 1 miliardo: entra in vigore il valore in arrivo | 1 | Entra in vigore. Qualcuno se n’era dimenticato | |
| `valore_in_vigore` |  | 2 | La norma diventa vera. Sorpresa negli uffici | |
| `posizione_spostata` | 4 posizioni spostate | 1 | Il partito si sposta. Di poco, per non farsi vedere | |
| `posizione_spostata` |  | 2 | Cambio di posizione. La coerenza resta a casa | |
| `partito_di_protesta` | Nasce ‹nome›, partito di protesta in posizione ‹k› (ricavato dal codice) | 1 | Nasce un partito di protesta. Gli altri protestano | |
| `partito_di_protesta` |  | 2 | Protesta con simbolo. Il palazzo chiude le finestre | |
| `coalizione_ia` | 2 coalizioni fra partiti IA | 1 | Fidanzamento politico. Le nozze si vedranno alle urne | |
| `coalizione_ia` |  | 2 | Patto fra rivali. Si giurano fedeltà fino al voto | |
| `emendamento_chiesto` | 2 emendamenti chiesti | 1 | Chiesti emendamenti. Ogni virgola ha il suo padrino | |
| `emendamento_chiesto` |  | 2 | Il testo non piace a tutti. Arrivano i ritocchi | |
| `misura_presentata` | Porta nel posto riservato Sgravio contributivo nel Mezzogiorno: 15% | 1 | Nuovo testo in aula. La relazione illustra poco | |
| `misura_presentata` |  | 2 | Il testo arriva in aula. I banchi fanno finta di niente | |
| `misura_presentata` |  | 3 | Depositato il testo. La tipografia ringrazia | |
| `misura_presentata` |  | 4 | Una misura in calendario. Il calendario sospira | |
| `fiducia_posta` | Pone la questione di fiducia su Decreto: fondi a Infrastrutture e Trasporti per Manutenzione straordinaria … | 1 | Questione di fiducia. Gli emendamenti restano fuori | |
| `fiducia_posta` |  | 2 | Fiducia sul testo. Chi dissente, voti sì lo stesso | |
| `sfiducia_presentata` | Presenta la sfiducia a Filippo Barbieri, Interno | 1 | Mozione di sfiducia. L’aula si scalda | |
| `sfiducia_presentata` |  | 2 | Sfiducia depositata. Il destinatario conta gli amici | |
| `governo_dimissionario` | Si dimette: il governo cade alla chiusura del turno (ricavato dal codice) | 1 | Governo dimissionario. Gli affari correnti corrono | |
| `governo_dimissionario` |  | 2 | Il governo getta la spugna. Il ring resta occupato | |

## Didascalie

Una per tipo, sotto la scena che il tipo condivide con altri. Le dice qualcuno del palazzo.

| Tipo | Scena | Didascalia | Mario |
|---|---|---|---|
| `sfiducia_approvata` | Aula | «Ma non avevamo contato?» | |
| `bilancio_approvato` | Aula | «Torna tutto. Basta non fare la somma» | |
| `legge_respinta` | Aula | «Bellissima legge. Peccato per il voto» | |
| `sfiducia_presentata` | Aula | «Una mozione? Niente pausa caffè, oggi» | |
| `legge_approvata` | Aula | «Passata. Adesso qualcuno la legga» | |
| `governo_formato` | Giuramento | «Lo giuro. Per i dettagli, c’è il gabinetto» | |
| `evento_risolto\|emergenza` | Emergenza | «Gli stivali li restituisco lunedì» | |
| `evento_nato\|emergenza` | Emergenza | «Qualcuno ha visto il fondo di riserva?» | |
| `ultimatum_posto` | Lettera | «Ci scrivono gli alleati. Di solito telefonano» | |
| `minaccia_di_dimissioni` | Lettera | «Firmo, ma a matita» | |
| `minaccia_di_scissione` | Lettera | «Ce ne andiamo. Appena troviamo il nome» | |
| `ministro_esce` | Lettera | «Le chiavi le lascio. L’auto blu la riporto domani» | |
| `evento_risolto\|situazione` | Cassa | «Problema risolto. Lo dice la ricevuta» | |
| `reddito_deluso` | Cassa | «La ripresa c’è. È solo timida» | |

## Battute a biro

Una per tipo di filo (`app/src/lib/schema/schema.ts:17-19`), nel dettaglio a lato. Minuscola iniziale, senza punto, al massimo 45 caratteri.

| Filo | Battuta | Mario |
|---|---|---|
| `maggioranza` | la maggioranza si conta, poi si spera | |
| `agenda` | promesso in agenda, a matita | |
| `cantiere` | la gru c’è, il cantiere si vedrà | |
| `proposta` | sale in ufficio, scende in cassetto | |
| `minaccia` | la porta è socchiusa, apposta | |
| `dimissioni` | chieste con garbo, attese con ansia | |
| `intesa` | l’amicizia si misura in sondaggi | |
| `misura` | in fila per l’aula, col numerino | |
| `legge` | la navetta non ha orari | |
| `posto` | posto riservato, cartellino in vista | |
| `sfiducia` | qualcuno affila la penna | |
| `evento` | il paese bussa alla porta | |
| `situazione` | la spia è accesa, il palazzo guarda altrove | |
| `spread` | i mercati non votano, ma contano | |

## Da sapere prima di tagliare

- Tre chiavi non si vedranno mai con la regola di oggi. Le «Aste scoperte» si aprono in tono grave (`crates/sim-core/src/partita.rs:446`) e la voce `crisi_del_debito` è sempre grave (`crates/sim-core/src/partita/esito.rs:892`); `evento_risolto|debito` prende il tono dell'evento aperto (`partita/esito.rs:310`). `evento_nato|debito` oggi non nasce: le Aste passano da `voce_crisi` (`crates/sim-core/src/partita.rs:438`), non da `voce_nato`. Ho scritto i titoli lo stesso, perché il file vuole una chiave per tipo; puoi tagliarli senza perdere niente.
- `guida_rifiutata`: l'occhiello del core dà del tu («non ti cede la guida»). Il titolo resta in terza persona.
- `minaccia_di_dimissioni` n. 1 è la riscrittura già decisa nel ticket 09 («Dimissioni pronte. In bella copia, per ora»).
- `sfiducia_approvata` e `sfiducia_presentata` valgono sia per la sfiducia al governo sia per quella al singolo ministro: i titoli non dicono quale.

## Conteggio

- Chiavi: 79 (12 da quattro varianti, 67 da due)
- Titoli: 182
- Didascalie: 14
- Battute a biro: 14
- Occhielli ricavati dal codice: 21
