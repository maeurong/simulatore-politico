---
tipo: modello
fonti: [fonti/arxiv-sistemi-elettorali-mondo-2016.md, fonti/arxiv-seat-product-2019.md, fonti/camera-sistema-elettorale-leg19.md, fonti/camera-dossier-collegi-2022.md, fonti/camera-dossier-ac2822-legge-elettorale-2026.md, fonti/camera-legge-elettorale-approvazione-2026.md, fonti/camera-legge-elettorale-provvedimento-leg19.md, fonti/gu-legge-180-2026-scheda.md, fonti/gu-legge-180-2026-art1.md, fonti/gu-legge-180-2026-art2.md, fonti/gu-legge-180-2026-art3.md, fonti/gu-legge-180-2026-art6.md, fonti/gu-legge-180-2026-art9.md, fonti/normattiva-dpr361-1957-vigenza-2026-10-24.md, fonti/steam-democracy4-voting-systems.md]
aggiornato: 2026-10-16
---
# Sistemi elettorali

Formule che trasformano voti in seggi: divisori (D'Hondt, Sainte-Laguë), quozienti e resti (Hare), maggioritario uninominale, sistemi misti, premi di maggioranza. Sono deterministiche ed economiche, e cambiano la politica più di molte policy: per il gioco sono un ingranaggio da implementare esattamente, con l'Italia come caso guida. Attenzione: dal 24 ottobre 2026 vale la legge 180/2026, non più il Rosatellum.

## Formule

- **Divisori (medie più alte)**: i voti di ogni lista si dividono per una serie di divisori e i seggi vanno ai quozienti più alti. D'Hondt: 1, 2, 3, 4…; Sainte-Laguë: 1, 3, 5, 7…; Sainte-Laguë modificato: 1,4, 3, 5, 7…; Imperiali: 2, 3, 4, 5…; danese: 1, 4, 7, 10… (fonti/arxiv-sistemi-elettorali-mondo-2016.md) [V].
- **Quozienti e maggiori resti**: quota (Hare = voti/seggi; Droop; Imperiali), seggi interi per quoziente, i restanti ai resti più alti [INF per le definizioni delle quote; l'uso di Hare in Italia è in fonti/camera-dossier-collegi-2022.md].
- **Favore ai grandi partiti**, dal più al meno favorevole: resti con quota Imperiali, D'Hondt, STV, resti con quota Droop, Sainte-Laguë modificato, resti con quota Hare e Sainte-Laguë, metodo danese (fonti/arxiv-sistemi-elettorali-mondo-2016.md) [V].
- **Maggioritario uninominale**: eletto il più votato nel collegio (fonti/camera-dossier-collegi-2022.md) [V].
- **Indici**: numero effettivo di partiti `N = 1/Σ s_i²` (Laakso-Taagepera 1979); disproporzionalità di Gallagher `LSq = √(½ Σ (v_i − s_i)²)` [INF; la pagina del sito di Gallagher al Trinity College risponde 404].
- **Modello del prodotto dei seggi** (Taagepera-Shugart): numero di partiti rilevanti `n ≈ (m·s)^(1/4)`, con m magnitudine media dei collegi e s seggi totali (fonti/arxiv-seat-product-2019.md) [V]; per il numero effettivo di partiti in seggi la versione Li-Shugart 2016 usa l'esponente 1/6 [INF].

## Italia fino al 2026: Rosatellum (legge 165/2017)

- 3/8 dei seggi in collegi uninominali maggioritari a turno unico, 5/8 in collegi plurinominali proporzionali con soglie (fonti/camera-sistema-elettorale-leg19.md) [V].
- Dopo la riduzione dei parlamentari (l. cost. 1/2020, d.lgs. 177/2020): Camera 147 uninominali + 245 proporzionali (in 49 collegi plurinominali) + 8 estero; Senato 74 + 122 (26 plurinominali) + 4 estero (fonti/camera-dossier-collegi-2022.md) [V].
- Riparto proporzionale con quozienti interi e maggiori resti, a livello nazionale per la Camera e regionale per il Senato (fonti/camera-dossier-collegi-2022.md) [V].
- Soglie: 3% per le liste, 10% per le coalizioni; i voti delle liste coalizzate sotto l'1% non contano per la coalizione; al Senato basta anche il 20% in una regione (fonti/camera-dossier-collegi-2022.md) [V].
- Voto unico: vale per il candidato uninominale e per la lista (fonti/camera-dossier-collegi-2022.md) [V].

## Italia dal 24 ottobre 2026: legge 180/2026

**Stato.** Legge 9 ottobre 2026, n. 180, «Disposizioni in materia di elezioni della Camera dei deputati e del Senato della Repubblica», pubblicata nella GU Serie generale n. 235 del 9 ottobre 2026, in vigore dal **24 ottobre 2026** (fonti/gu-legge-180-2026-scheda.md:28) [V]. Promulgata da Mattarella il 9 ottobre 2026, il giorno dopo l'approvazione definitiva della Camera (227 sì, 164 no) (fonti/gu-legge-180-2026-art9.md:22; fonti/camera-legge-elettorale-approvazione-2026.md) [V]. Pubblicata senza note per urgenza; la ripubblicazione con note è annunciata nella GU del 23 ottobre 2026 (fonti/gu-legge-180-2026-art1.md:1242) [V]. È una legge di novella: modifica il DPR 361/1957 (Camera, art. 1), il d.lgs. 533/1993 (Senato, art. 2) e la legge 459/2001 (Estero, art. 6). Normattiva al 16/10/2026 non ha ancora consolidato il DPR 361/1957 (ultimo aggiornamento all'atto del 17/05/2025, anche chiedendo la vigenza al 24/10/2026) (fonti/normattiva-dpr361-1957-vigenza-2026-10-24.md:17) [V]: il testo coordinato degli artt. 83 e 83-bis DPR 361 e 16-bis e 17 d.lgs. 533 si ricava oggi applicando le novelle al testo Rosatellum. Il testo pubblicato coincide nei punti chiave (42%, 70/35, 220/113, soglie, abolizione della regola dell'1%) con quello descritto dal dossier del 16/09/2026 (fonti/camera-dossier-ac2822-legge-elettorale-2026.md) [V].

**Seggi.** Camera 400 = 8 Estero + 1 Valle d'Aosta + 7 Trentino-Alto Adige + 384 nelle altre 26 circoscrizioni; con premio, i 384 si dividono in 70 di premio e 314 proporzionali (fonti/camera-dossier-ac2822-legge-elettorale-2026.md:845) [V]. Senato 200 = 4 Estero + 1 Valle d'Aosta + 6 Trentino-Alto Adige + 189 nelle altre 18 regioni, con premio 35 + 154 (fonti/camera-dossier-ac2822-legge-elettorale-2026.md:6296, :7190) [V]. I collegi plurinominali restano i 49 (Camera) e 26 (Senato) del d.lgs. 177/2020; i collegi uninominali spariscono fuori da Valle d'Aosta e Trentino-Alto Adige (fonti/gu-legge-180-2026-art1.md:32; fonti/gu-legge-180-2026-art2.md:25) [V].

**Soglie** (uguali nelle due Camere, salvo la deroga regionale del Senato), calcolate sui voti nazionali comprese Valle d'Aosta e Trentino-Alto Adige (fonti/gu-legge-180-2026-art1.md:754) [V]:
- coalizione ammessa con almeno il 10% nazionale e almeno una lista coalizzata al 3% (Camera: o una lista di minoranza linguistica al 20% nella sua regione; Senato: o una lista al 20% in una regione) (fonti/camera-dossier-ac2822-legge-elettorale-2026.md:4331, :8435) [V];
- lista ammessa con il 3% nazionale, coalizzata o no, anche se la sua coalizione resta sotto il 10%; al Senato basta anche il 20% in una regione (fonti/gu-legge-180-2026-art1.md:551; fonti/gu-legge-180-2026-art2.md:260; per le liste di coalizioni sotto soglia fonti/camera-dossier-ac2822-legge-elettorale-2026.md:4331) [V];
- **ripescaggio**: per ogni coalizione ammessa entra anche la lista coalizzata più votata fra quelle sotto soglia (fonti/gu-legge-180-2026-art1.md:560) [V];
- **abolita la regola dell'1%**: la cifra della coalizione somma i voti di tutte le liste coalizzate (fonti/gu-legge-180-2026-art1.md:529; fonti/camera-dossier-ac2822-legge-elettorale-2026.md:4259) [V].

**Premio.** Va alla lista o coalizione con la maggiore cifra nazionale fra quelle ammesse, se ha almeno il 42% dei voti validi nazionali **e** se al Senato la prima è la lista o coalizione collegata con essa (collegamento dichiarato al deposito del contrassegno) e ha anch'essa almeno il 42%; altrimenti nessun premio in nessuna delle due Camere: 70 seggi alla Camera, 35 al Senato (fonti/gu-legge-180-2026-art1.md:573; fonti/gu-legge-180-2026-art2.md:273) [V]. Se si vota per una sola Camera il premio non si assegna (fonti/gu-legge-180-2026-art1.md:733; fonti/gu-legge-180-2026-art2.md:395) [V]. I seggi premio vanno ai candidati di **liste circoscrizionali** presentate dalla coalizione nel suo insieme, non alle singole liste (fonti/gu-legge-180-2026-art1.md:200) [V].

**Riparto Camera** (art. 83 DPR 361 come novellato):
1. riparto nazionale dei 314 seggi (con premio) o dei 384 (senza) fra coalizioni e liste ammesse, sulle cifre nazionali **senza** i voti di Valle d'Aosta e Trentino-Alto Adige, con quozienti interi e maggiori resti come nel Rosatellum (fonti/gu-legge-180-2026-art1.md:533, :603; metodo: fonti/camera-dossier-collegi-2022.md) [V];
2. **tetto 220**: se il vincitore, sommando premio, quota proporzionale e seggi vinti in Valle d'Aosta e Trentino-Alto Adige (Estero escluso), supera 220, riceve nel proporzionale `150 − seggi VdA/TAA`; gli altri si dividono `314 − quel numero` con quoziente intero e maggiori resti (fonti/gu-legge-180-2026-art1.md:614, :656) [V];
3. riparto interno alla coalizione fra le liste ammesse (3% e ripescata), poi distribuzione nelle circoscrizioni e nei collegi con il correttore eccedentarie/deficitarie del Rosatellum; nel caso del tetto la distribuzione usa indici separati per gruppo di maggioranza e di minoranza, troncati alla sesta cifra decimale (fonti/gu-legge-180-2026-art1.md:689, :810) [V].

**Riparto Senato** (art. 16-bis d.lgs. 533 come novellato): riparto **regione per regione** (154 o 189 seggi) con quozienti interi e maggiori resti sulle cifre regionali (fonti/gu-legge-180-2026-art2.md:302, :343) [V]. **Tetto 113**: se il vincitore supera 113 (premio 35 + proporzionale + seggi VdA/TAA), gli si tolgono seggi finché resta a 78 fuori dal premio, prima nelle regioni in cui ha preso seggi con i resti più bassi (graduatoria nazionale crescente), poi una per regione dove ha la percentuale regionale più bassa, mai sotto un seggio per regione; ogni seggio tolto va, nella stessa regione, all'altra forza con il maggior resto non usato (fonti/gu-legge-180-2026-art2.md:352) [V]. Poi riparto interno alla coalizione su base regionale (fonti/gu-legge-180-2026-art2.md:434) [V].

**Riparto fra circoscrizioni.** Seggi per circoscrizione e quota di premio si fissano con il DPR di convocazione dei comizi in base ai dati del censimento permanente; i 70 (35) seggi premio si ripartiscono escluse Valle d'Aosta e Trentino-Alto Adige (fonti/gu-legge-180-2026-art1.md:81; fonti/gu-legge-180-2026-art2.md:46) [V]. Il dossier li simula sui dati del censimento 2021 con quozienti interi e maggiori resti (fonti/camera-dossier-ac2822-legge-elettorale-2026.md:2162, :6910, :7190) [V]; sono numeri validi finché un nuovo DPR sul censimento non li cambia [INF]:

| Camera (totale/premio/plurinominali con premio) | | Senato (totale/premio/plurinominali con premio) |
|---|---|---|
| Piemonte 1 15/3/12 · Piemonte 2 14/2/12 | | Piemonte 14/3/11 |
| Lombardia 1 27/5/22 · Lomb. 2 14/2/12 · Lomb. 3 15/3/12 · Lomb. 4 11/2/9 | | Lombardia 32/6/26 |
| Veneto 1 13/2/11 · Veneto 2 19/4/15 | | Veneto 16/3/13 |
| Friuli-V. G. 8/1/7 · Liguria 10/2/8 · Emilia-Romagna 29/5/24 | | FVG 4/1/3 · Liguria 5/1/4 · Emilia-R. 14/3/11 |
| Toscana 24/4/20 · Umbria 6/1/5 · Marche 10/2/8 | | Toscana 12/2/10 · Umbria 3/1/2 · Marche 5/1/4 |
| Lazio 1 25/5/20 · Lazio 2 13/2/11 · Abruzzo 8/2/6 · Molise 2/0/2 | | Lazio 18/3/15 · Abruzzo 4/1/3 · Molise 2/0/2 |
| Campania 1 20/4/16 · Campania 2 17/3/14 · Puglia 26/5/21 | | Campania 18/3/15 · Puglia 13/2/11 |
| Basilicata 4/1/3 · Calabria 12/2/10 | | Basilicata 3/0/3 · Calabria 6/1/5 |
| Sicilia 1 15/3/12 · Sicilia 2 17/3/14 · Sardegna 10/2/8 | | Sicilia 15/3/12 · Sardegna 5/1/4 |
| **Totale 384/70/314** | | **Totale 189/35/154** |

**Valle d'Aosta e Trentino-Alto Adige.** Camera: Valle d'Aosta un collegio uninominale; Trentino-Alto Adige 4 uninominali (eletto il più votato, a parità il più giovane) + 3 seggi proporzionali in un unico collegio plurinominale, ripartiti con quozienti interi e maggiori resti fra le coalizioni e liste ammesse a livello nazionale, più le liste di minoranza linguistica al 20% nella circoscrizione (fonti/gu-legge-180-2026-art1.md:58, :1161) [V]. Senato: Valle d'Aosta un uninominale, Trentino-Alto Adige 6 uninominali e nessun seggio proporzionale; l'art. 3 ridisegna i collegi di Bolzano, Merano e Bressanone (fonti/gu-legge-180-2026-art2.md:67; fonti/gu-legge-180-2026-art3.md:15) [V]. I voti di queste circoscrizioni contano solo per soglie, primo posto e 42%; i loro seggi contano per il tetto (fonti/gu-legge-180-2026-art1.md:58) [V].

**Estero.** Camera: le ripartizioni passano da 4 a 2 (Europa; Americhe, Africa, Asia, Oceania e Antartide); Senato: i 4 senatori si eleggono nell'intera circoscrizione Estero (fonti/gu-legge-180-2026-art6.md:19; fonti/camera-dossier-ac2822-legge-elettorale-2026.md:10027) [V]. Seggi esteri esclusi dai tetti 220/113 (fonti/gu-legge-180-2026-art1.md:50) [V].

**Altro.** Fino a tre preferenze, di genere diverso, per i non capolista; capolista eletto per primo; liste di 7 candidati in ordine alternato di genere; nome del candidato presidente del consiglio dichiarato, unico per coalizione (fonti/gu-legge-180-2026-art1.md:114, :136, :229) [V].

**Non trovato.** Testo coordinato ufficiale (Normattiva non aggiornata; GU con note attesa il 23/10/2026) [NON TROVATO]. DPR con i seggi reali per circoscrizione: si emana solo alla convocazione dei comizi [NON TROVATO]. Divisione degli 8 deputati esteri fra le due nuove ripartizioni [NON TROVATO]. A quale partito della coalizione appartengano i seggi premio: la legge li dà a candidati di liste circoscrizionali comuni, e la composizione dei gruppi dipende da chi viene candidato [INF].

## Evidenza

- Il numero di partiti cresce con la magnitudine dei collegi in modo concavo (legge di Duverger generalizzata), per effetto meccanico della formula e per effetto psicologico su partiti ed elettori (fonti/arxiv-seat-product-2019.md) [V].
- Il modello del prodotto dei seggi ha alternative più precise per i sistemi D'Hondt, testate su elezioni di più paesi e su centinaia di elezioni locali polacche (fonti/arxiv-seat-product-2019.md) [V].
- Le formule sono definite per legge: qui l'evidenza serve solo per stimare gli effetti indiretti (frammentazione, voto strategico), non le formule stesse [INF].

## Costo computazionale

D'Hondt e Sainte-Laguë con S seggi e P liste: `O(S log P)` con una coda di priorità, oppure `O(S·P)` ingenuo. Hare: `O(P log P)` per ordinare i resti. Il Rosatellum annida più riparti (nazionale → circoscrizione → collegio, più gli slittamenti) ma resta sotto il millisecondo [INF]. La complessità è nel codice, non nel calcolo: il riparto italiano ha molte regole di correzione che vanno testate su risultati ufficiali [INF].

## Cosa semplifichiamo

- Voto estero: seggi fissi assegnati con le stesse quote nazionali [INF].
- Trentino-Alto Adige e Valle d'Aosta: regole speciali omesse o semplificate [INF].
- Pluricandidature, preferenze e alternanza di genere: irrilevanti per i seggi di partito, utili solo se il gioco modella i singoli eletti [INF].

## Per il nostro gioco

- Implementare un motore di riparto generico (divisori, quote, soglie, premi, livelli annidati) e descrivere ogni sistema come dati. Così il giocatore può riformare la legge elettorale, una delle leve più forti e meno sfruttate dai giochi (vedi [[democracy-4]]: il DLC Voting Systems permette di passare fra uninominale e proporzionale e aggiunge policy su chi vota e sul finanziamento delle campagne, fonti/steam-democracy4-voting-systems.md).
- Fornire tre preset italiani: Rosatellum (2017-2026), legge 2026, proporzionale puro. Il confronto fra i tre è già un contenuto di gioco.
- Calcolare e mostrare N effettivo e indice di Gallagher a ogni elezione: due numeri che spiegano al giocatore cosa ha fatto la sua riforma.
- Test: riprodurre le elezioni politiche 2018 e 2022 dai voti ufficiali come oracolo del motore.

## Collegamenti

- [[formazione-di-coalizioni]]: cosa succede quando nessuno ha la maggioranza.
- [[voto-spaziale]]: il voto strategico nei collegi uninominali parte da qui.
- [[sondaggi-ed-errore]]: le proiezioni in seggi dai sondaggi usano lo stesso motore.
- [[democracy-4]], [[lawgivers-2]]: come i giochi esistenti trattano i sistemi elettorali.
- [[territorio-e-urbanistica]]: la granularità della mappa, da decidere insieme ai collegi.

## Riferimenti

- **URL** https://arxiv.org/pdf/1605.01343 · [V]
  **perché conta qui** rassegna dei metodi di riparto con divisori e ordinamento per favore ai grandi partiti
  **cosa se ne prende** le serie di divisori e quale metodo favorisce chi
- **URL** https://arxiv.org/pdf/1909.12036 · [V]
  **perché conta qui** modello del prodotto dei seggi e alternativa per D'Hondt
  **cosa se ne prende** n ≈ (m·s)^(1/4) per prevedere la frammentazione prodotta da una riforma
- **URL** https://www.camera.it/temiap/documentazione/temi/pdf/1104098.pdf · [V]
  **perché conta qui** descrizione ufficiale del Rosatellum dopo la riduzione dei parlamentari
  **cosa se ne prende** numeri di seggi, soglie, regola dell'1%, livelli di riparto
- **URL** https://documenti.camera.it/leg19/dossier/Pdf/AC0469f.pdf · [V]
  **perché conta qui** commento ufficiale al testo poi pubblicato, con le tabelle dei seggi simulate sul censimento 2021
  **cosa se ne prende** conti 384/314 e 189/154, seggi e premio per circoscrizione, lettura delle soglie e dell'abolizione dell'1%
- **URL** https://comunicazione.camera.it/archivio-prima-pagina/19-62971 · [V]
  **perché conta qui** conferma l'approvazione definitiva dell'8 ottobre 2026
  **cosa se ne prende** la data e il voto finale
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaDettaglioAtto/originario?atto.dataPubblicazioneGazzetta=2026-10-09&atto.codiceRedazionale=26G00201&elenco30giorni=false · [V]
  **perché conta qui** prova primaria di pubblicazione: legge 180/2026, GU n. 235 del 9/10/2026
  **cosa se ne prende** numero, data e entrata in vigore (24/10/2026) da cui parte il preset «legge 2026»
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaArticolo?art.versione=1&art.idGruppo=0&art.flagTipoArticolo=0&art.codiceRedazionale=26G00201&art.idSottoArticolo=1&art.idSottoArticolo1=10&art.dataPubblicazioneGazzetta=2026-10-09&art.progressivo=0&art.idArticolo=1 · [V]
  **perché conta qui** testo ufficiale delle regole di riparto della Camera (artt. 1-3, 83, 83-bis, 93-septies DPR 361)
  **cosa se ne prende** soglie, ripescaggio, verifica del 42%, tetto 220 con 150/314, riparto in Trentino-Alto Adige
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaArticolo?art.versione=1&art.idGruppo=0&art.flagTipoArticolo=0&art.codiceRedazionale=26G00201&art.idSottoArticolo=1&art.idSottoArticolo1=10&art.dataPubblicazioneGazzetta=2026-10-09&art.progressivo=0&art.idArticolo=2 · [V]
  **perché conta qui** testo ufficiale del riparto del Senato, che è regionale e ha un tetto diverso da quello della Camera
  **cosa se ne prende** riparto regionale, tetto 113 con soglia 78 e algoritmo di sottrazione dei seggi, deroga del 20% regionale
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaArticolo?art.versione=1&art.idGruppo=0&art.flagTipoArticolo=0&art.codiceRedazionale=26G00201&art.idSottoArticolo=1&art.idSottoArticolo1=10&art.dataPubblicazioneGazzetta=2026-10-09&art.progressivo=0&art.idArticolo=3 · [V]
  **perché conta qui** ridisegno dei collegi senatoriali di Bolzano, Merano e Bressanone
  **cosa se ne prende** i 6 uninominali del Trentino-Alto Adige al Senato restano, con confini nuovi
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaArticolo?art.versione=1&art.idGruppo=0&art.flagTipoArticolo=0&art.codiceRedazionale=26G00201&art.idSottoArticolo=1&art.idSottoArticolo1=10&art.dataPubblicazioneGazzetta=2026-10-09&art.progressivo=0&art.idArticolo=6 · [V]
  **perché conta qui** cambia la struttura della circoscrizione Estero
  **cosa se ne prende** 2 ripartizioni per la Camera, collegio unico per il Senato
- **URL** https://www.gazzettaufficiale.it/atto/serie_generale/caricaArticolo?art.versione=1&art.idGruppo=0&art.flagTipoArticolo=0&art.codiceRedazionale=26G00201&art.idSottoArticolo=1&art.idSottoArticolo1=10&art.dataPubblicazioneGazzetta=2026-10-09&art.progressivo=0&art.idArticolo=9 · [V]
  **perché conta qui** formula di promulgazione con data e firma
  **cosa se ne prende** promulgazione del 9/10/2026, che chiude il [NON TROVATO] precedente
- **URL** https://www.normattiva.it/uri-res/N2Ls?urn:nir:presidente.repubblica:decreto:1957-03-30;361~art83!vig=2026-10-24 · [V]
  **perché conta qui** mostra che il testo coordinato del DPR 361/1957 non è ancora aggiornato
  **cosa se ne prende** il motivo per cui il motore va scritto dalle novelle e ricontrollato sul testo coordinato quando esce
