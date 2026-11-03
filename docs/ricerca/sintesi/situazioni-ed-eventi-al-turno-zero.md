---
tipo: sintesi
fonti: [fonti/eurostat-namq-10-gdp-it-qoq-2026-10.md, fonti/viminale-dossier-ferragosto-2026.md, fonti/istat-delitti-denunciati-2008.md, fonti/istat-bes-2026-nota-stampa.md, fonti/giustizia-detenuti-capienze-2026-09-30.md, fonti/giustizia-detenuti-presenti-1991-2026.md, fonti/giustizia-interrogazione-benzoni-risposta-2026-07-07.md, fonti/cedu-torreggiani-2013-testo.md, fonti/worldbank-pink-sheet-2026-10.md, fonti/worldbank-cmo-mensile-gas-europa-brent-2026-10.md, fonti/viminale-cruscotto-sbarchi-2026-10-28.md, fonti/sole24ore-33-ministri-dimessi-dal-2001.md, fonti/ilpost-2013-06-24-idem-dimissioni.md, fonti/skytg24-2014-01-26-de-girolamo-dimissioni.md, fonti/espresso-2026-03-25-dimissioni-santanche.md, fonti/quifinanza-2026-santanche-dimessa.md, fonti/ilpost-2024-04-02-sfiducia-individuale.md, fonti/sole24ore-sfiducia-79-tentativi.md, fonti/ti-cpi-italia-2025.md, fonti/ti-cpi-how-calculated.md, fonti/ispra-siccita-disponibilita-idrica-2023.md, fonti/dpc-delibera-2017-06-22-crisi-idrica-parma-piacenza.md, fonti/dpc-delibera-2022-07-04-deficit-idrico.md, fonti/dpc-delibera-2024-05-06-deficit-idrico-sicilia.md, fonti/camera-temi-terremoti-2026.md, fonti/camera-temi-alluvione-emilia-romagna-2023.md, fonti/blitz-2011-10-btp-10-anni-6-06.md, fonti/blitz-2011-11-29-asta-btp-7-56.md, fonti/linkiesta-2011-12-aste-titoli-di-stato.md, fonti/bdi-qef-128-2012-premi-rischio-sovrano.md, fonti/esm-guideline-precautionary-assistance.md, fonti/ecb-pr-2012-09-06-omt.md, fonti/zettelmeyer-trebesch-gulati-2013-greek-autopsy.md]
aggiornato: 2026-10-29
---
# Situazioni ed eventi al turno zero

I numeri reali su cui tarare le sei situazioni a soglia, gli scandali, la siccità, i fondi per le emergenze, la crisi del debito e gli shock economici della v1 (piano 9; spec righe 357-364, decisione del 17/10 in [[decisioni]]). Per ogni situazione: valore di ottobre 2026 o l'ultimo disponibile, picchi recenti con data, variabilità trimestrale, posizione dei picchi su un indice a 100 = oggi. Le soglie restano proposte: le decide il brainstorming.

Tag: [V] fonte primaria letta; [M] misurato in questa sessione con un comando (convenzione di questa base, vedi [[sondaggi-ottobre-2026]]); [INF] inferenza; [NON TROVATO]. Dove un numero viene dalla memoria e non da una fonte letta, è scritto «da memoria, non verificato». Le stampe dei calcoli [M] sono nel testo; gli script non sono conservati nel repo.

## 0. Premesse sul motore (lette il 29/10/2026)

- Le grandezze dei dicasteri sono indici a 100 al turno zero, con `min = 0` e `max = 200` (`dati/base/grandezze.toml:1-3`, `:193-223`). `sovraffollamento_carceri`, `immigrazione_irregolare` e `corruzione` hanno `dinamica = "fissa"`: senza una deriva propria si muovono solo con atti e disposizioni.
- `prezzo_energia` è uno scarto in % sugli input importati, a 0 al turno zero (`grandezze.toml:159-168`), e lo stato iniziale dell'energia è 0 «perché lo shock in corso sta già nell'inflazione» (`dati/base/scenario.toml:208-212`).
- Gli shock sono cinque estrazioni normali per turno, `normale × sigma` (`crates/sim-core/src/economia/mod.rs:218-224`). Sigma attuali: domanda 0,4 punti di gap, produttività 0,2, energia 3,0 (con persistenza 0,7), domanda estera 1,0 (persistenza 0,7), BCE 0,1 (`dati/base/manopole.toml:93-140`). Tutte sono «assunzione del piano 3», con i residui storici [NON TROVATO].
- Una legislatura dura `durata_legislatura` anni, cioè 4 turni l'anno (`crates/sim-core/src/partita.rs:1999`); con 5 anni sono 20 turni [INF].

## 1. Le sei situazioni

### 1.1 Recessione

- **Oggi.** PIL in volume, variazione sul trimestre precedente: +0,3% nel 2026T1, +0,2% nel 2026T2; nessun trimestre negativo dal 2022T4 (fonti/eurostat-namq-10-gdp-it-qoq-2026-10.md, aggiornamento del 26/10/2026) [M]. Il 2026T3 preliminare non era ancora uscito al 29/10.
- **Definizione.** La convenzione giornalistica è la «recessione tecnica», due trimestri consecutivi negativi [INF: convenzione, nessuna fonte normativa]. Con questa regola, dal 2000 la serie Eurostat attuale dà sei episodi [M]:

| episodio | trimestri | calo cumulato (somma delle variazioni) |
|---|---|---|
| 2001T2-2001T4 | 3 | -0,7 |
| 2003T1-2003T3 | 3 | -0,8 |
| 2008T2-2009T2 | 5 | -7,7 |
| 2011T2-2013T1 | 8 | -5,5 |
| 2013T4-2014T1 | 2 | -0,2 |
| 2019T3-2020T2 | 4 | -18,9 |

- **Revisioni.** La recessione tecnica di fine 2018, annunciata all'epoca, non compare nella serie riveduta: oggi l'unico trimestre negativo del 2018 è il T1 (stessa fonte) [M]. L'annuncio di allora è da memoria, non verificato. Per il gioco conta una cosa: con la prima stima rumorosa ([[consiglieri-e-informazione-imperfetta]]) una recessione può accendersi sul dato ufficiale e sparire alla revisione.
- **Variabilità.** Deviazione standard della crescita trimestrale: 0,49 punti nel 2010-2019 (media 0,07); 2,05 punti sull'intero 2000-2026, per via del 2020 [M].
- **Accesa al turno zero:** no.

### 1.2 Ondata di criminalità

- **Oggi.** Delitti denunciati: 2.399.347 nel 2024 e 2.357.105 nel 2025 (-1,8%); primo semestre 2026: 1.068.240, contro 1.186.268 del primo semestre 2025 (-10%). Omicidi volontari 163 → 138, furti 493.975 → 437.800, rapine 14.275 → 12.515 (fonti/viminale-dossier-ferragosto-2026.md, p. 5) [V].
- **Storia.** 2.709.888 delitti nel 2008, -7,6% sul 2007; secondo l'ISTAT i livelli, saliti fino al 1991, sono poi rimasti «sostanzialmente stabili» con andamento altalenante (fonti/istat-delitti-denunciati-2008.md) [V]. Il 2007 vale quindi circa 2,93 milioni [M: 2.709.888 / 0,924]. Su un indice a 100 = 2025, il 2007 sta a circa 124 [M]. Serie annuale completa 2004-2024 [NON TROVATO in sessione]: il servizio SDMX dell'ISTAT non ha risposto.
- **Cautela di misura.** Il conteggio dipende anche dalla propensione a denunciare e dalle modifiche normative (fonti/istat-delitti-denunciati-2008.md) [V]: il gioco può leggerlo come «sicurezza percepita».
- **Variabilità.** Fra un anno e l'altro gli scarti stanno sotto il 10%, con un solo dato semestrale a -10% [INF, dai pochi punti letti]. La grandezza del motore più vicina è `sicurezza`, un servizio con verso «sale» (`grandezze.toml:82-91`).
- **Accesa al turno zero:** no, il dato è in calo.

### 1.3 Collasso delle liste d'attesa

- **Oggi.** Nel 2025 il 9,6% delle persone ha rinunciato ad almeno una visita o un accertamento di cui aveva bisogno. Il dato è stabile sul 2024 ma «in forte crescita rispetto al 2019». Per area: Nord da 9,2% a 8,7%, Centro 10,7%, Mezzogiorno 10,3% (fonti/istat-bes-2026-nota-stampa.md) [V]. Per il solo motivo delle liste d'attesa la stampa riporta il 6,5% della popolazione nel 2025 e il 6,8% nel 2024 (Quotidiano Sanità e Rapporto annuale ISTAT 2025, citati in ricerca, non catturati) [INF].
- **Storia.** Valore 2019: circa 6,3% secondo fonti secondarie [non verificato]. Con questo valore il 2019 starebbe a circa 66 su un indice a 100 = 2025 [INF].
- **Variabilità.** È un indicatore annuale da indagine campionaria (Aspetti della vita quotidiana) [INF]: niente variabilità trimestrale misurabile. La grandezza del motore è `qualita_servizio_sanitario`, un servizio (`grandezze.toml:38-47`).
- **Accesa al turno zero:** dipende dalla soglia. Il valore è al massimo della serie letta. Una soglia «peggio di oggi di X%» la lascia spenta; una soglia ancorata al livello pre-pandemia la accende subito [INF].

### 1.4 Emergenza carceri

- **Oggi.** Al 30/09/2026: 66.063 detenuti presenti (2.968 donne, 20.856 stranieri) su 51.319 posti di capienza regolamentare, cioè un affollamento del 128,7% [M: 66.063 / 51.319]. La capienza è calcolata con 9 m² per detenuto più 5 per gli altri (fonti/giustizia-detenuti-capienze-2026-09-30.md) [V]. Sui posti **disponibili**, al 2-3 giugno 2026: 64.776 detenuti, 51.269 posti regolamentari di cui 46.312 disponibili, indice di affollamento 139,86%; 18.307 detenuti stavano in spazi fra 3 e 4 m² (fonti/giustizia-interrogazione-benzoni-risposta-2026-07-07.md) [V].
- **Picco e Torreggiani.** Massimo della serie semestrale: 68.258 detenuti al 30/06/2010 (fonti/giustizia-detenuti-presenti-1991-2026.md) [V]. Nella sentenza Torreggiani (CEDU, 8/1/2013) la Corte riporta 67.961 detenuti nel 2010 per 45.000 posti, un sovraffollamento nazionale del 151%; al 13/4/2012, 66.585 detenuti e 148% (fonti/cedu-torreggiani-2013-testo.md, §§ 23 e 29) [V]. Su un indice ad affollamento con 100 = 128,7% di oggi, il 151% vale circa 117 [M]. Contando le persone, il picco del 2010 sta a 103 [M: 68.258 / 66.063].
- **Variabilità e tendenza.** Variazione semestrale della popolazione detenuta: +1,76% di media nel 2021-2026, deviazione standard 1,07 punti [M], cioè circa +0,9% a trimestre. I salti vengono tutti dalle norme e dagli shock: indulto del 2006 -45% in un semestre (da 61.264 a 39.005), COVID nel primo semestre 2020 -12,6%, misure seguite a Torreggiani nel 2014 -7/-8% a semestre (stessa fonte) [M]. L'indulto come causa del 2006 è da memoria, non verificato.
- **Conseguenza per una grandezza «fissa».** Con capienza ferma, il tasso cresce di circa il 3,5% l'anno e arriva al livello Torreggiani (indice ~117) in circa 4,5 anni, cioè verso la fine di una legislatura [M: ln(1,173)/0,035]. Misurato sui posti disponibili (139,9% → 151%) ci arriva in circa 2 anni [M].
- **Accesa al turno zero:** è la più vicina. Con una soglia di accensione all'indice di oggi o poco sopra è già accesa; con la soglia al livello Torreggiani è spenta ma ci arriva da sola entro la legislatura, purché la grandezza abbia la deriva misurata [INF].

### 1.5 Crisi energetica

- **Oggi.** Gas europeo (TTF): media del 2026T3 a 21,53 $/MMBtu, settembre 25,42, contro 10,26 nel 2025T4, cioè 2,1 volte [M]. Brent: 116,8 $/barile a settembre 2026, 97,1 di media nel T3, contro 63,7 nel 2025T4 (fonti/worldbank-pink-sheet-2026-10.md) [V].
- **Causa.** La stampa la lega al conflitto con l'Iran, iniziato a febbraio 2026, e ai flussi di Hormuz; il TTF era a circa 89 €/MWh al 22/10, +177% da febbraio, secondo dati della Commissione citati da soldionline.it [INF: secondaria, non catturata].
- **Picco del 2022.** TTF medio a 60,16 $/MMBtu nel 2022T3, 70,04 ad agosto 2022 (fonti/worldbank-cmo-mensile-gas-europa-brent-2026-10.md) [V]: 2,8 volte il 2026T3, 5,9 volte il 2025T4, 12,5 volte la media del 2019 [M]. PUN medio 2022 circa 304 €/MWh, da memoria, non verificato: il sito del GME carica i dati via script e non è stato leggibile.
- **Variabilità.** Deviazione standard della variazione logaritmica trimestrale: TTF 0,137 nel 2010-2019 e 0,224 sull'intero 2000-2026; Brent 0,126 e 0,155 [M]. Le code sono grasse: 19 trimestri su 106 superano due deviazioni standard del decennio calmo (2009, 2016, 2019-2024, 2026T1 e T3) [M]. La sigma del motore (3 punti sugli input importati) non è confrontabile direttamente: è un'altra unità.
- **Accesa al turno zero:** nel mondo reale sì, la crisi è in corso. Nel motore no per costruzione, perché lo scarto parte da 0 e lo shock in corso sta solo nell'inflazione (`scenario.toml:208-212`). Il brainstorming deve scegliere fra due strade: ancorare la soglia a un livello pre-shock, oppure partire con la situazione accesa per scenario [INF].

### 1.6 Ondata di sbarchi

- **Oggi.** Dal 1/1 al 28/10/2026 sono sbarcate 26.192 persone, contro 58.273 nello stesso periodo del 2025 e 55.049 nel 2024 [V]. Mesi del 2026: da 1.457 (gennaio) a 3.329 (settembre) (fonti/viminale-cruscotto-sbarchi-2026-10-28.md) [V]. Ultimi 12 mesi (ottobre 2025 - settembre 2026): 38.199 [M].
- **Serie annuale:** 2014: 170.100; 2015: 153.842; **2016: 181.436**; 2017: 119.369; 2018: 23.370; 2019: 11.471; 2020: 34.154; 2021: 67.477; 2022: 105.131; **2023: 157.651**; 2024: 66.617; 2025: 66.316 (stessa fonte) [V].
- **Sull'indice a 100 = ultimi 12 mesi:** 2016 = 475, 2023 = 413, 2024-2025 = 174, 2019 = 30 [M]. Le due ondate storiche stanno **oltre il `max = 200`** della grandezza (`grandezze.toml:208-213`): con questa scala il motore non può rappresentarle [INF].
- **Variabilità.** Deviazione standard della variazione logaritmica annuale 2015-2025: 0,74, con un -1,63 nel 2018 [M]. Trimestrale 2024T1-2026T3: 0,50, stagionalità compresa [M]. È di gran lunga la più volatile delle sei.
- **Accesa al turno zero:** no, il dato è dimezzato sul 2025.

### Sintesi delle sei

| situazione | grandezza più vicina | oggi | picco recente (data) | picco su indice 100 = oggi | variabilità | accesa al turno zero |
|---|---|---|---|---|---|---|
| recessione | PIL (motore) | +0,2% t/t (2026T2) | -12,6% t/t (2020T2); -7,7 cumulato 2008-09 | n/a (soglia sul segno) | σ 0,49 pt a trimestre (2010-19) | no |
| criminalità | `sicurezza` | 2,36 mln di delitti (2025); -10% nel I sem. 2026 | ~2,93 mln (2007) | ~124 | <10% annuo | no |
| liste d'attesa | `qualita_servizio_sanitario` | 9,6% di rinunce (2025) | 9,6-9,9% (2024-25, massimo letto) | 100 (oggi è il picco) | solo annuale | dipende dalla soglia |
| carceri | `sovraffollamento_carceri` | 128,7% regolamentare; 139,9% sui disponibili | 151% (2010) | ~117 (tasso), 103 (persone) | +0,9% a trimestre, σ ~0,75 | quasi; da sola entro la legislatura |
| energia | `prezzo_energia` (scarto) | TTF 21,5 $ (2026T3), ×2,1 sul 2025T4 | TTF 60,2 $ (2022T3) | ×2,8 su oggi | σ log 0,14-0,22 a trimestre | reale sì, motore no |
| sbarchi | `immigrazione_irregolare` | 38.199 in 12 mesi | 181.436 (2016), 157.651 (2023) | 475, 413 (oltre max 200) | σ log 0,50 trim., 0,74 annua | no |

Le colonne «picco su indice» e «variabilità» sono [M] dalle fonti sopra; la colonna «grandezza» è un'associazione [INF] da `grandezze.toml`, non una decisione.

### Deriva e rumore per le tre grandezze «fisse» [INF]

- `sovraffollamento_carceri`: deriva di circa +0,9% a trimestre (2021-2026), rumore piccolo (σ ~0,75%). I salti grossi li danno solo atti e disposizioni (indulti, pene alternative).
- `immigrazione_irregolare`: deriva nulla nel lungo periodo, rumore grande (σ log 0,5 a trimestre) e code grasse: un passo aleatorio in logaritmo, con ritorno lento verso un livello. Va riscalato il `max`.
- `corruzione`: quasi ferma. Il CPI italiano è a 53/100 nel 2025 (52° su 182, -1 sul 2024) (fonti/ti-cpi-italia-2025.md) [V]. Transparency avverte che le variazioni piccole non sono significative e che la corruzione di un anno compare nelle fonti anche un anno o più dopo (fonti/ti-cpi-how-calculated.md) [V]. Serie italiana 2012-2025 (circa 42 → 56 → 53), da memoria, non verificata: il grafico della pagina non si estrae. Rumore trimestrale ≤1% e una deriva lenta [INF].

## 2. Scandali

**Ministri dimessi per scandali o inchieste, 2008-2026** (esclusi i passaggi ad altri incarichi e le crisi di coalizione):

| legislatura | governo | ministro (anno) | fonte |
|---|---|---|---|
| XVI (2008-2013) | Berlusconi IV | Claudio Scajola (maggio 2010), Aldo Brancher (2010) | fonti/sole24ore-33-ministri-dimessi-dal-2001.md [V] |
| XVI | Monti | nessuno | [INF: nessun caso nelle fonti lette] |
| XVII (2013-2018) | Letta | Josefa Idem (giugno 2013, ICI non pagata); Nunzia De Girolamo (gennaio 2014, inchiesta sull'ASL di Benevento, con una sfiducia M5S già calendarizzata) | fonti/ilpost-2013-06-24-idem-dimissioni.md, fonti/skytg24-2014-01-26-de-girolamo-dimissioni.md [V] |
| XVII | Renzi | Maurizio Lupi (2015, Grandi opere), Federica Guidi (aprile 2016, Tempa Rossa) | fonti/sole24ore-33-ministri-dimessi-dal-2001.md [V] |
| XVII | Gentiloni | nessuno (Lotti supera la sfiducia del 15/3/2017) | fonti/ilpost-2024-04-02-sfiducia-individuale.md [V] |
| XVIII (2018-2022) | Conte I, Conte II, Draghi | nessun ministro. Le uscite del Conte II (Fioramonti, Bonetti, Bellanova) sono politiche | fonti/sole24ore-33-ministri-dimessi-dal-2001.md [V]. Sottosegretari usciti (Siri, Rixi, Durigon): da memoria, non verificato |
| XIX (2022-) | Meloni | Gennaro Sangiuliano (settembre 2024, caso Boccia); Daniela Santanchè (marzo 2026, inchieste Visibilia, dopo la richiesta pubblica della premier). Con lei lasciano il sottosegretario Delmastro e la capa di gabinetto Bartolozzi | fonti/sole24ore-33-ministri-dimessi-dal-2001.md, fonti/espresso-2026-03-25-dimissioni-santanche.md, fonti/quifinanza-2026-santanche-dimessa.md [V] |

- **Frequenza:** 8 ministri in circa 18,5 anni, cioè 2 per legislatura (0, 2 o 4 per legislatura completa) e circa 0,11 a trimestre [M: 8 / 74 trimestri]. Per ogni motivo, compresi quelli personali, dal 2001 se ne sono andati 33 ministri su 278 nominati, il 12% (fonti/sole24ore-33-ministri-dimessi-dal-2001.md) [V].
- **Il caso del 2026 è politico.** Santanchè aveva superato più sfiducie: Senato il 26/7/2023 (fonti/ilpost-2024-04-02-sfiducia-individuale.md) [V], poi due mozioni M5S nell'aprile e nel luglio 2024, entrambe respinte, e una nuova coppia a febbraio 2025 (fonti/sole24ore-sfiducia-79-tentativi.md) [V]. Si dimette solo dopo la richiesta della premier, seguita alla sconfitta del referendum sulla giustizia (fonti/quifinanza-2026-santanche-dimessa.md) [V]. Lo scandalo dà il bersaglio; la caduta la decide la forza del governo [INF].
- **Sfiducia individuale.** Dal 1984: 82 mozioni presentate, 39 votate, una sola approvata (Mancuso, Senato, 19/10/1995, presentata dalla stessa maggioranza) (fonti/ilpost-2024-04-02-sfiducia-individuale.md, aprile 2024) [V]. Il Sole 24 Ore, a febbraio 2025, conta 80 mozioni dal 1983 e non più di 36 votate (fonti/sole24ore-sfiducia-79-tentativi.md) [V]: le due serie differiscono per data e criterio. Il picco è la XVII legislatura, con 34 mozioni, 30 del M5S (fonti/ilpost-2024-04-02-sfiducia-individuale.md) [V]. Su quelle votate passa circa il 3% [M: 1/37].
- **Legame con la corruzione percepita: [NON TROVATO].** Nessuna fonte letta collega il CPI (o un altro indice) alla frequenza delle dimissioni dei ministri. Il CPI è a sua volta un'aggregazione di percezioni di esperti, con ritardo (fonti/ti-cpi-how-calculated.md) [V]: un legame CPI → scandali rischia di essere causalità rovesciata [INF]. Chang, Golden e Hill (2010, World Politics) studiano le accuse giudiziarie ai deputati italiani, ma il testo è a pagamento e non è stato letto. La licenza CC BY-ND del CPI resta quella già annotata in [[burocrazia-corruzione-e-capacita-statale]].

## 3. Siccità: situazione o emergenza

**Dati.**
- 2017: il CdM del 22/6/2017 dichiara per 180 giorni lo stato di emergenza per la crisi idropotabile nelle province di Parma e Piacenza, con 8,65 milioni dal Fondo per le emergenze nazionali (fonti/dpc-delibera-2017-06-22-crisi-idrica-parma-piacenza.md) [V]. Seguono l'estensione ad altre province dell'Emilia-Romagna (15/9/2017) e lo stato di emergenza nel Lazio (7/8/2017) [INF: delibere viste in ricerca, non catturate].
- 2022: il CdM del 4/7/2022 dichiara fino al 31/12/2022 lo stato di emergenza nei bacini del Po e delle Alpi orientali (Emilia-Romagna, Friuli-Venezia Giulia, Lombardia, Piemonte, Veneto), con 36,5 milioni dal Fondo: 10,9 all'Emilia-Romagna, 4,2 al Friuli-Venezia Giulia, 9,0 alla Lombardia, 7,6 al Piemonte, il resto al Veneto (fonti/dpc-delibera-2022-07-04-deficit-idrico.md) [V]. L'estensione a Lazio, Liguria, Toscana, Marche e Umbria e la proroga al 31/12/2023 [INF: delibere viste in ricerca, non catturate]. Per ISPRA il 2022 ha avuto circa il 20% del territorio in siccità estrema ed è il terzo anno peggiore dopo il 1990 e il 2002 (fonti/ispra-siccita-disponibilita-idrica-2023.md) [V].
- 2024: il CdM del 6/5/2024 dichiara per 12 mesi lo stato di emergenza per il deficit idrico in Sicilia, con 20 milioni dal Fondo (fonti/dpc-delibera-2024-05-06-deficit-idrico-sicilia.md) [V]. La proroga di 12 mesi del maggio 2025 [INF: secondaria].
- **Danni:** una stima ufficiale [NON TROVATO]; le cifre circolanti vengono dalle associazioni agricole. **Durata:** 6-18 mesi di stato di emergenza per episodio [M, dalle delibere lette e dalle proroghe].

**Argomenti** [INF, da decidere nel brainstorming]:
- *A favore della situazione a soglia:* l'esordio è graduale, la durata va da molti trimestri ad anni e c'è un fine naturale. Il pattern accensione/spegnimento con isteresi gli si adatta.
- *Contro:* la spec vuole le situazioni come etichette sopra grandezze **che il motore muove già** (riga 357), e nessuna grandezza misura acqua o precipitazioni (`grandezze.toml`). Una situazione sulla siccità chiederebbe una grandezza nuova. Le situazioni sono poi nazionali per decisione del 20/10 ([[decisioni]]), mentre ogni siccità reale è regionale o di distretto (Po 2022, Sicilia 2024). E la risposta dello Stato è la stessa delle altre emergenze: delibera, prima tranche dal Fondo, commissario.
- **Proposta:** emergenza puntuale regionale con effetto prolungato. Il rischio stagionale cade nel secondo e terzo trimestre, l'effetto dura 2-6 turni, e usa lo stesso meccanismo (fondo di riserva / decreto / nulla) delle altre emergenze. Resta situazione solo se il piano aggiunge una grandezza idrica.

## 4. Fondo per le emergenze nazionali e fondo di riserva

- **Dotazione annua consolidata del Fondo: [NON TROVATO].** Si conoscono solo gli incrementi: +130 milioni nel 2024, +71,8 milioni l'anno nel 2024-2025, +200 milioni nel 2024 (d.l. 113/2024), +100 milioni l'anno nel 2025-2026 (d.l. 39/2024) (fonti/camera-temi-terremoti-2026.md) [V]; già in [[pericolosita-naturale-regioni]].
- **Utilizzo per evento:** la prima delibera mette poche decine di milioni. Siccità 2017: 8,65; siccità 2022: 36,5; Sicilia 2024: 20 (§3) [V]. Alluvione dell'Emilia-Romagna 2023: primo stanziamento «del tutto preliminare» di 10 milioni con la delibera del 4/5/2023 (fonti/camera-temi-alluvione-emilia-romagna-2023.md) [V].
- **Decreti a debito:** per l'alluvione del 2023 il governo aveva stanziato **4,7 miliardi** a dicembre 2024 (2,7 per le infrastrutture pubbliche, circa 1,9 di rimborsi a famiglie e imprese, di cui 0,7 in credito d'imposta), attraverso il d.l. 61/2023 e i successivi (fonti/camera-temi-alluvione-emilia-romagna-2023.md, comunicato del Commissario del 27/12/2024) [V]. Il rapporto fra prima tranche del Fondo e totale è circa 1:470 [M: 4.700 / 10].
- **Fondo di riserva per le spese impreviste** (art. 28 della L. 196/2009): stanziamento 2025-2026 [NON TROVATO]. Un fondo da 350 milioni per il solo 2026, istituito dalla L. 199/2025 c. 555 e distinto dal Fondo emergenze, risulta da una fonte secondaria [non verificato]. La taglia di 0,2-0,4 miliardi l'anno proposta nella spec è nell'ordine degli incrementi del Fondo emergenze [INF].

## 5. Crisi del debito

Già nella base ([[economia-italiana-parametri]], §1-2): massimo dello spread a 5,5 punti con il rendimento al 7,3% nel novembre 2011; TPI, OMT e i loro criteri. Qui si aggiunge il resto.

- **Le aste del 2011 non sono andate deserte.** 28/10/2011: BTP decennale al 6,06%, con 3,793 miliardi di domanda per al massimo 3 offerti; BTP luglio 2014 al 4,93%, con il rapporto domanda/offerta sceso a 1,324 da 1,365 (fonti/blitz-2011-10-btp-10-anni-6-06.md, dati Banca d'Italia) [V, fonte di stampa]. 29/11/2011: triennale al 7,89%, il massimo dal 1996, e decennale al 7,56%, collocato per l'intero massimo di 2,5 miliardi (fonti/blitz-2011-11-29-asta-btp-7-56.md) [V, stampa]. BOT a sei mesi al 6,504% a novembre, con copertura 1,4; a fine dicembre, dopo l'LTRO della BCE, 3,251% con copertura 1,7 (fonti/linkiesta-2011-12-aste-titoli-di-stato.md) [V, stampa]. Un'asta italiana scoperta nel 2011-2012 [NON TROVATO]. Lo stress si è visto nei rendimenti oltre il 7% e nella copertura fra 1,3 e 1,4, non nella mancanza di domanda [INF].
- **Conseguenza per il motore.** La «soglia delle aste deserte» (ticket 07) non ha un precedente italiano. Un innesco realistico è un rendimento o uno spread oltre una soglia, con la copertura che cala, nella zona di novembre 2011 [INF].
- **MES, linee precauzionali** (linee guida in vigore, testo pre-riforma). PCCL: solo per paesi con situazione «fondamentalmente sana», sulla base di sei criteri: (a) rispetto del Patto di stabilità, ammesso anche chi è in PDE purché rispetti le raccomandazioni; (b) debito sostenibile; (c) impegni della procedura per squilibri; (d) storia di accesso ai mercati a condizioni ragionevoli; (e) posizione esterna sostenibile; (f) assenza di crisi bancarie sistemiche. ECCL: per chi non rispetta alcuni criteri ma resta «sound», con misure correttive concordate. Entrambe durano un anno, rinnovabili due volte per sei mesi, e si erogano con prestiti o acquisti sul primario; c'è sorveglianza rafforzata della Commissione (fonti/esm-guideline-precautionary-assistance.md, artt. 2-5) [V]. L'OMT richiede un programma EFSF/MES pieno o una ECCL con acquisti sul primario (fonti/ecb-pr-2012-09-06-omt.md) [V]. Con il debito al 137%, il criterio (b) sembra difficile: la via più probabile è l'ECCL [INF]. Il trattato riformato non è in vigore perché l'Italia non l'ha ratificato (Camera, dicembre 2023); lo stato al 2026 [non verificato].
- **Grecia 2012 (PSI), il solo caso di ristrutturazione nell'area euro con titoli di Stato.** Scambio di marzo-aprile 2012 per circa 200 miliardi, con adesione del 96,9% (199,2 miliardi; 6,4 di resistenti). Per ogni titolo vecchio: 15% del nominale in titoli EFSF a 1-2 anni più 31,5% in 20 titoli nuovi 2023-2042, con cedole fra il 2 e il 4,3%, più un titolo legato al PIL. Il taglio nominale vale quindi il 53,5% [M: 100 − 15 − 31,5]; quello aggregato in valore attuale il 58-65%; la riduzione del debito il 44-53% del PIL. Per i titoli di diritto greco, l'86% del totale, servirono clausole d'azione collettiva retroattive. BCE e banche centrali nazionali furono esentate; le banche greche, fra i maggiori detentori, andarono ricapitalizzate. Nove mesi dopo, un riacquisto di oltre metà dei titoli nuovi (fonti/zettelmeyer-trebesch-gulati-2013-greek-autopsy.md, abstract e §§ 2-3) [V]. Cipro 2013 colpì i depositi, non i titoli di Stato (da memoria, non verificato).
- **Chi detiene il debito italiano oggi:** [NON TROVATO in sessione]. Serve per tarare il costo interno di una ristrutturazione (banche e famiglie italiane).

## 6. Shock oltre 2σ: il conto

**Ipotesi dichiarate** [INF]: cinque shock gaussiani indipendenti per turno, con la soglia su |innovazione| > 2σ della propria manopola; 20 turni per legislatura. Il conto non dipende dal valore delle sigma: sopra 2σ c'è la stessa probabilità qualunque sia la sigma.

- P(|z| > 2) = 0,0455. Per una serie: **0,91 eventi a legislatura**. Per cinque serie: **4,55**, o **2,28** se contano solo gli shock sfavorevoli (un lato). P(almeno uno in legislatura): 0,99 con due lati, 0,90 con uno [M].
- **Confronto storico, PIL.** Dal 2000 al 2026T2 (106 trimestri, 5,3 legislature) i trimestri sotto la media di oltre 2σ del 2010-2019 sono 6: 2008T3, 2008T4, 2009T1, 2012T1, 2020T1, 2020T2. Sono raggruppati in **3 episodi** (fonti/eurostat-namq-10-gdp-it-qoq-2026-10.md) [M]. Con σ calcolata sull'intero campione restano solo i tre trimestri del 2020.
- **Confronto storico, energia.** Il TTF supera 2σ del decennio calmo in 19 trimestri su 106; quelli in rialzo cadono in 2020T3-2021T4, 2022T3 e 2026T1-T3 (§1.5) [M].
- **Lettura.** Gli shock grandi distinti dal 2000 sono circa cinque: 2008-09, 2011-12, 2020, 2021-22 (energia), 2026 (energia). Fa circa **1 per legislatura** [INF, conteggio di episodi]. Il motore, con estrazioni indipendenti e senza persistenza delle innovazioni, ne produce 2-4,5 a legislatura, per lo più isolati e di un turno. Sono più frequenti e più piccoli di quelli veri, che sono rari, a grappolo e con code molto più grasse (σ del PIL 2,05 sull'intero campione contro 0,49 nel decennio calmo) [INF].
- **Leve possibili per il brainstorming** [INF]: soglia a 2,5σ: P(|z| > 2,5) = 0,0124, cioè circa 1,2 eventi a legislatura su cinque serie, 0,6 contando un solo lato [M]; oppure shock con code grasse (t di Student) e persistenza, così che l'evento nasca alla prima innovazione oltre soglia e il grappolo resti un evento solo.

## Per il nostro gioco

Raccomandazioni, non decisioni:
- **Carceri ed energia sono i due casi in cui il turno zero tocca la soglia.** Per le carceri, la deriva misurata (+0,9% a trimestre) basta a portarle alla soglia Torreggiani entro la legislatura. Per l'energia va scelto se lo shock in corso è stato iniziale.
- **Le tre grandezze «fisse» hanno bisogno di una dinamica esogena** con i parametri del §1. Senza, la situazione delle carceri non si accende mai da sola, e gli sbarchi non hanno né rumore né scala: il `max = 200` sta sotto i picchi del 2016 e del 2023.
- **Scandali:** un tasso di base di circa 0,1 dimissioni a trimestre è un buon ancoraggio per «rischio uguale per tutti»; la crescita con la corruzione resta una manopola, non un dato.
- **Siccità** come emergenza regionale prolungata (§3); **aste deserte** sostituite da una soglia di rendimento (§5); **shock** a 2,5σ o con code grasse (§6).

## Riferimenti

- **URL** https://ec.europa.eu/eurostat/api/dissemination/statistics/1.0/data/namq_10_gdp?geo=IT&unit=CLV_PCH_PRE&s_adj=SCA&na_item=B1GQ&sinceTimePeriod=2000-Q1 · [V]
  **perché conta qui** serie ufficiale del PIL trimestrale italiano, riveduta al 26/10/2026
  **cosa se ne prende** gli episodi di recessione tecnica dal 2000, la σ trimestrale, il conto dei trimestri oltre 2σ (fonti/eurostat-namq-10-gdp-it-qoq-2026-10.md)
- **URL** https://www.interno.gov.it/sites/default/files/2026-08/dossier_viminale_2026.pdf · [V]
  **perché conta qui** dato più recente sui delitti denunciati, anche semestrale
  **cosa se ne prende** 2.399.347 (2024), 2.357.105 (2025), -10% nel I sem. 2026 e i reati di allarme sociale (fonti/viminale-dossier-ferragosto-2026.md)
- **URL** https://www.istat.it/tavole-di-dati/delitti-denunciati-dalle-forze-di-polizia-allautorita-giudiziaria-anno-2008/ · [V]
  **perché conta qui** punto storico ISTAT, e la cautela sulla misura
  **cosa se ne prende** 2.709.888 delitti nel 2008 (-7,6%), quindi il 2007 a ~2,93 milioni; la dipendenza dalla propensione a denunciare (fonti/istat-delitti-denunciati-2008.md)
- **URL** https://www.istat.it/wp-content/uploads/2026/10/Nota-stampa-BES-2026.pdf · [V]
  **perché conta qui** indicatore ufficiale delle rinunce alle cure, cioè le liste d'attesa
  **cosa se ne prende** 9,6% nel 2025, stabile sul 2024, in forte crescita sul 2019, valori per area (fonti/istat-bes-2026-nota-stampa.md)
- **URL** https://www.giustizia.it/giustizia/it/mg_1_14_1.page?contentId=SST1522607 · [V]
  **perché conta qui** valore delle carceri al turno zero
  **cosa se ne prende** 66.063 presenti su 51.319 posti al 30/09/2026 e il criterio dei 9+5 m² (fonti/giustizia-detenuti-capienze-2026-09-30.md)
- **URL** https://www.giustizia.it/giustizia/it/mg_1_14_1.page?contentId=SST165666 · [V]
  **perché conta qui** serie semestrale 1991-2026 dei detenuti presenti
  **cosa se ne prende** il picco di 68.258 (30/06/2010), la tendenza 2021-2026, i salti del 2006, del 2014 e del 2020 (fonti/giustizia-detenuti-presenti-1991-2026.md)
- **URL** https://www.giustizia.it/cmsresources/cms/documents/interrogazione_benzoni_1_risposta_7lug2026.pdf · [V]
  **perché conta qui** affollamento ufficiale sui posti disponibili, più alto di quello regolamentare
  **cosa se ne prende** 46.312 posti disponibili e indice del 139,86% a giugno 2026; 18.307 detenuti in 3-4 m² (fonti/giustizia-interrogazione-benzoni-risposta-2026-07-07.md)
- **URL** https://giurcost.org/casi_scelti/CEDU/CEDU08-01-13-43517.htm · [V]
  **perché conta qui** testo della sentenza pilota Torreggiani (copia di Consulta Online: HUDOC non era leggibile senza script)
  **cosa se ne prende** 151% nel 2010 (67.961 detenuti per 45.000 posti) e 148% ad aprile 2012, cioè la soglia storica (fonti/cedu-torreggiani-2013-testo.md, §§ 23, 29)
- **URL** https://thedocs.worldbank.org/en/doc/74e8be41ceb20fa0da750cda2f6b9e4e-0050012026/related/CMO-Pink-Sheet-October-2026.pdf · [V]
  **perché conta qui** prezzi dell'energia aggiornati a settembre 2026; licenza del Pink Sheet non verificata (i WDI sono CC BY 4.0, vedi [[world-bank-wdi]])
  **cosa se ne prende** TTF e Brent annuali 2023-2025, trimestrali e mensili 2025-2026 (fonti/worldbank-pink-sheet-2026-10.md)
- **URL** https://thedocs.worldbank.org/en/doc/74e8be41ceb20fa0da750cda2f6b9e4e-0050012026/related/CMO-Historical-Data-Monthly.xlsx · [V]
  **perché conta qui** serie mensile lunga per i picchi e la variabilità
  **cosa se ne prende** TTF 70,04 $ ad agosto 2022, σ log trimestrale di TTF e Brent, il conto dei trimestri oltre 2σ (fonti/worldbank-cmo-mensile-gas-europa-brent-2026-10.md)
- **URL** https://libertaciviliimmigrazione.dlci.interno.gov.it/sites/default/files/2026-10/Cruscotto%20statistico%20giornaliero%2028-10-2026.pdf · [V]
  **perché conta qui** dato ufficiale degli sbarchi al 28/10/2026, con la serie annuale e i mesi
  **cosa se ne prende** 26.192 nel 2026, la serie 2014-2025 (picchi 2016 e 2023), i mesi 2024-2026 (fonti/viminale-cruscotto-sbarchi-2026-10-28.md)
- **URL** https://en.ilsole24ore.com/art/caso-santanche-ruggiero-sangiuliano-ecco-33-ministri-che-si-sono-dimessi-2001-AGNMJ2yC · [V]
  **perché conta qui** elenco delle dimissioni dei ministri dal 2001 (fonte di stampa)
  **cosa se ne prende** 33 su 278 (12%); Scajola, Brancher, Lupi, Guidi, Sangiuliano; le uscite politiche del Conte II (fonti/sole24ore-33-ministri-dimessi-dal-2001.md)
- **URL** https://www.ilpost.it/2013/06/24/live-josefa-idem-dimissioni/ · [V]
  **perché conta qui** dimissioni Idem, governo Letta, che manca nel pezzo del Sole
  **cosa se ne prende** data e causa (fonti/ilpost-2013-06-24-idem-dimissioni.md)
- **URL** https://tg24.sky.it/politica/2014/01/26/governo_dimissioni_ministro_nunzia_de_girolamo · [V]
  **perché conta qui** dimissioni De Girolamo, governo Letta
  **cosa se ne prende** data, interim a Letta, sfiducia M5S già calendarizzata (fonti/skytg24-2014-01-26-de-girolamo-dimissioni.md)
- **URL** https://lespresso.it/c/politica/2026/3/25/dimissioni-santanche-meloni-giusi-bartolozzi-nordio-sfiducia/60866 · [V]
  **perché conta qui** caso Santanchè del marzo 2026
  **cosa se ne prende** la richiesta pubblica della premier del 24/3, le mozioni dell'opposizione, l'assenza di un potere di revoca (fonti/espresso-2026-03-25-dimissioni-santanche.md)
- **URL** https://quifinanza.it/politica/santanche-dimissioni-revoca/963397/ · [V]
  **perché conta qui** conferma delle dimissioni e del contesto
  **cosa se ne prende** dimissioni dopo Delmastro e Bartolozzi, legate alla sconfitta al referendum sulla giustizia (fonti/quifinanza-2026-santanche-dimessa.md)
- **URL** https://www.ilpost.it/2024/04/02/salvini-santanche-mozione-di-sfiducia/ · [V]
  **perché conta qui** statistica storica della sfiducia individuale
  **cosa se ne prende** 82 presentate, 39 votate, 1 approvata; le 34 della XVII; Lotti 2017 (fonti/ilpost-2024-04-02-sfiducia-individuale.md)
- **URL** https://en.ilsole24ore.com/art/caso-santanche-ecco-cos-e-mozione-sfiducia-40-anni-falliti-79-tentativi-80-AGH6VkoC · [V]
  **perché conta qui** conteggio alternativo, più recente (febbraio 2025)
  **cosa se ne prende** 80 mozioni dal 1983, non più di 36 votate (fonti/sole24ore-sfiducia-79-tentativi.md)
- **URL** https://www.transparency.org/en/countries/italy · [V]
  **perché conta qui** valore del CPI italiano, l'ingresso del rischio di scandalo (CC BY-ND)
  **cosa se ne prende** 53/100 nel 2025, 52° su 182, -1 sul 2024 (fonti/ti-cpi-italia-2025.md)
- **URL** https://protezionecivile.regione.emilia-romagna.it/piani-sicurezza-interventi-urgenti/ordinanze-piani-e-atti-correlati-dal-2008/crisi-idrica-pr-pc-ord-468-del-2017/00_dichiarazionestatoemergenza_DCM_22617_GU_156_6717.pdf/@@download/file/00_dichiarazione%20stato%20emergenza_DCM_22-6-17_GU_156_6-7-17.pdf · [V]
  **perché conta qui** siccità 2017, con la prima tranche del Fondo
  **cosa se ne prende** 180 giorni e 8,65 milioni per Parma e Piacenza (fonti/dpc-delibera-2017-06-22-crisi-idrica-parma-piacenza.md)
- **URL** https://www.protezionecivile.gov.it/static/29ed648477104f3c3ff3f3004c12bf78/delibera-del-4-luglio-2022-crisi-idrica-2022-po-alpi-orientali-reg-lomb-fvg-er-ven-pie.pdf · [V]
  **perché conta qui** siccità 2022, la più grave recente
  **cosa se ne prende** cinque regioni, 36,5 milioni ripartiti, scadenza 31/12/2022 (fonti/dpc-delibera-2022-07-04-deficit-idrico.md)
- **URL** https://www.protezionecivile.gov.it/static/ca917ec264dd4f171bad3c236317ecf3/delibera-cdm-del-06-maggio-2024-eme-deficit-idrico-sicilia.pdf · [V]
  **perché conta qui** siccità regionale del 2024
  **cosa se ne prende** 12 mesi e 20 milioni per la Sicilia (fonti/dpc-delibera-2024-05-06-deficit-idrico-sicilia.md)
- **URL** https://temi.camera.it/leg19/post/l-alluvione-e-le-frane-in-emilia-romagna-2023-maggio.html · [V]
  **perché conta qui** costo reale di un'emergenza grande contro la prima tranche del Fondo
  **cosa se ne prende** 10 milioni iniziali e 4,7 miliardi stanziati a fine 2024, con la ripartizione (fonti/camera-temi-alluvione-emilia-romagna-2023.md)
- **URL** https://archivio.blitzquotidiano.it/economia/il-rendimento-dei-titoli-di-stato-btp-a-10-anni-sfonda-il-6-999332 · [V]
  **perché conta qui** asta di ottobre 2011 (stampa su dati Banca d'Italia)
  **cosa se ne prende** decennale al 6,06%, domanda 3,793 miliardi su 3, copertura 1,324 sul 2014 (fonti/blitz-2011-10-btp-10-anni-6-06.md)
- **URL** https://archivio.blitzquotidiano.it/economia/asta-btp-3-anni-10-anni-rendimenti-spread-1034366 · [V]
  **perché conta qui** l'asta peggiore della crisi
  **cosa se ne prende** triennale al 7,89%, decennale al 7,56% collocato per intero (fonti/blitz-2011-11-29-asta-btp-7-56.md)
- **URL** https://www.linkiesta.it/2011/12/titoli-di-stato-litalia-respira-ma-i-veri-rischi-sono-a-gennaio/ · [V]
  **perché conta qui** effetto dell'LTRO sulle aste
  **cosa se ne prende** BOT dal 6,504% al 3,251%, copertura da 1,4 a 1,7 (fonti/linkiesta-2011-12-aste-titoli-di-stato.md)
- **URL** https://www.esm.europa.eu/system/files/document/esm_guideline_on_precautionary_financial_assistance.pdf · [V]
  **perché conta qui** condizioni d'accesso al MES precauzionale, la porta dell'OMT
  **cosa se ne prende** i sei criteri della PCCL, la regola dell'ECCL, le durate (fonti/esm-guideline-precautionary-assistance.md)
- **URL** https://www.economic-policy.org/wp-content/uploads/2013/04/zettelmeyer.pdf · [V]
  **perché conta qui** analisi di riferimento della ristrutturazione greca
  **cosa se ne prende** adesione del 96,9%, menu 15% + 31,5%, taglio del 58-65% in valore attuale, clausole retroattive, esenzione della BCE (fonti/zettelmeyer-trebesch-gulati-2013-greek-autopsy.md)
- Fonti già nella base, riusate: fonti/camera-temi-terremoti-2026.md (incrementi del Fondo), fonti/ispra-siccita-disponibilita-idrica-2023.md (siccità 2022), fonti/ti-cpi-how-calculated.md (ritardo e licenza del CPI), fonti/bdi-qef-128-2012-premi-rischio-sovrano.md e fonti/ecb-pr-2012-09-06-omt.md (spread 2011 e OMT), con i tre campi nelle pagine che le hanno catturate.

## Collegamenti

- [[crisi-ed-eventi]]: i meccanismi (soglia con isteresi, rischio per turno) che questi numeri tarano.
- [[pericolosita-naturale-regioni]]: rischio e costi delle emergenze naturali; qui si aggiungono gli usi del Fondo.
- [[economia-italiana-parametri]]: spread, TPI e OMT; qui le aste, il MES e il PSI greco.
- [[criminalita-e-giustizia]]: carceri e Torreggiani.
- [[burocrazia-corruzione-e-capacita-statale]]: CPI e licenza.
- [[iter-legislativo-italiano]]: la sfiducia individuale come procedura.
- [[consiglieri-e-informazione-imperfetta]]: prima stima e revisione, che possono accendere e spegnere una recessione.
- [[stato-del-paese-al-turno-zero]]: i livelli macro di partenza.
- [[decisioni]]: decisione del 17/10/2026 sugli eventi (ticket 15).
