---
tipo: modello
fonti: [fonti/ecb-eb-box-2019-02-i-g.md, fonti/ecb-eb-box-2024-05-ageing-report.md, fonti/ec-ageing-report-2024-fiche-francia.md, fonti/saez-slemrod-giertz-2009-eti.md, fonti/laubach-2003-deficits-interest.md, fonti/nyfed-cantor-packer-1996.md]
aggiornato: 2026-10-17
---
# Finanza pubblica: bilancio, debito, spread, rating, gettito, pensioni

Il bilancio dello Stato è un'identità contabile; il debito segue un'equazione nota; lo spread e il rating reagiscono a debito e disavanzo con elasticità misurate; il gettito risponde all'aliquota secondo l'elasticità del reddito imponibile. Per il nostro gioco è il blocco che punisce o premia le scelte di bilancio, e che rende possibile una crisi del debito.

## Equazioni chiave

**Dinamica del debito** (rapporto debito/PIL $b$, saldo primario $pb$, tasso implicito $i$, crescita nominale $g$, aggiustamenti stock-flussi $sfa$) [INF, forma standard]:
$b_t = \dfrac{1 + i_t}{1 + g_t}\, b_{t-1} - pb_t + sfa_t$

La BCE la legge così: se $i - g$ è positivo serve un avanzo primario per stabilizzare il debito, tanto più grande quanto più alto è il debito di partenza; con $i - g$ negativo il debito può stabilizzarsi anche con disavanzi primari, ma non se sono troppo grandi. Per le economie mature $i - g$ si è aggirato storicamente intorno a un punto percentuale (fonti/ecb-eb-box-2019-02-i-g.md) [V].

**Gettito e curva di Laffer**. Elasticità del reddito imponibile $e$ rispetto a $(1-\tau)$. Aliquota che massimizza il gettito sullo scaglione più alto (fonti/saez-slemrod-giertz-2009-eti.md) [V]:
$\tau^* = \dfrac{1}{1 + a \cdot e}$
con $a$ parametro della coda paretiana dei redditi. Esempio degli autori: $e = 0{,}5$, $a = 1{,}6$ danno $\tau^* = 55{,}6\%$. Le stime di lungo periodo più affidabili di $e$ stanno fra 0,12 e 0,4, simili negli altri paesi; l'elasticità è più alta per i redditi elevati con più spazi di elusione e dipende dall'ampiezza della base imponibile (fonti/saez-slemrod-giertz-2009-eti.md) [V].

Base imponibile nel gioco: $Z(\tau) = Z_0 \left(\dfrac{1-\tau}{1-\tau_0}\right)^{e}$, gettito $R = \tau \cdot Z(\tau)$ [INF, definizione dell'elasticità applicata]. Con $e$ fra 0,12 e 0,4 e un'aliquota unica ($a = 1$), $\tau^*$ va dal 71% all'89%: le aliquote realistiche stanno quasi sempre sul ramo crescente della curva [INF, aritmetica].

**Spread / tasso a lungo**. Su dati USA, un punto in più di disavanzo/PIL atteso alza il tasso forward a 10 anni fra 5 anni di 20-29 punti base (tipico 22); un punto in più di debito/PIL atteso lo alza di 3-4 punti base (fonti/laubach-2003-deficits-interest.md) [V].

**Rating sovrano**. Sei fattori spiegano gran parte dei rating di Moody's e S&P: reddito pro capite, crescita del PIL, inflazione, debito estero, livello di sviluppo, storia di default. I rating influenzano i rendimenti, soprattutto sotto l'investment grade, anche al netto dell'informazione pubblica (fonti/nyfed-cantor-packer-1996.md) [V].

## Pensioni a ripartizione

I contributi di chi lavora oggi pagano le pensioni di oggi. Lo schema si collega alla piramide per coorti di [[demografia-e-migrazioni]], che fornisce $N_{a,t}$, la popolazione per età $a$.

- Occupati e pensionati dalla piramide [INF]: $E_t = \sum_{a=20}^{64} N_{a,t}\, e_{a,t}$ con $e_a$ tasso di occupazione per età, che si muove con la disoccupazione di [[mercato-del-lavoro-e-disuguaglianza]]; $P_t = \sum_{a \ge \bar a_t} N_{a,t}\, c_{a,t}$ con $\bar a_t$ età di pensionamento e $c_a$ copertura (quota di persone di età $a$ che riceve una pensione).
- Saldo del sistema [INF]: $S_t = \tau^c_t\, w_t\, E_t - \bar b_t\, P_t$, con $\tau^c$ aliquota contributiva, $w$ salario medio, $\bar b$ pensione media. Se $S_t < 0$ lo Stato copre il buco con un trasferimento, che entra nel saldo primario $pb_t$ della dinamica del debito.
- Aliquota di pareggio [INF, algebra]: $\tau^{c*} = \dfrac{\bar b}{w} \cdot \dfrac{P}{E}$, cioè tasso di sostituzione medio per indice di dipendenza del sistema. Se il rapporto pensionati/occupati sale da 0,7 a 0,8, a parità di generosità l'aliquota di pareggio sale di un settimo.
- Indicizzazione [INF]: $\bar b_{t+1} = \bar b_t \,(1 + k_\pi \pi_t + k_w g^w_t)$. Con indicizzazione ai soli prezzi ($k_w = 0$) il rapporto fra pensione e salario scende quando la produttività cresce. È la leva che nelle proiezioni UE riduce di più la spesa: il rapporto prestazioni/salari dipende da quanto e quanto in fretta le pensioni seguono inflazione e produttività (fonti/ecb-eb-box-2024-05-ageing-report.md) [V].
- Rendimento implicito: nel lungo periodo un sistema a ripartizione «rende» quanto cresce la massa salariale, cioè occupazione più salari reali [INF, risultato standard di Samuelson e Aaron, non catturato].

**Scomposizione ufficiale UE.** L'Ageing Report divide la spesa pensionistica in quattro fattori: indice di dipendenza degli anziani, tasso di copertura (pensionati su popolazione con 65 anni o più), rapporto prestazioni/salari e un effetto mercato del lavoro (occupazione, ore lavorate, partecipazione degli anziani) (fonti/ecb-eb-box-2024-05-ageing-report.md) [V]. Da qui al 2070 l'indice di dipendenza spinge la spesa in su di 6,2 punti di PIL in media; prestazioni (-2,9), copertura (-1,3) e mercato del lavoro (-1,1) compensano in parte. L'effetto mercato del lavoro pesa di più in Italia: -2,8 punti al 2070 (fonti/ecb-eb-box-2024-05-ageing-report.md) [V]. La forma moltiplicativa esatta della scomposizione è in una figura non estraibile come testo (fonti/ec-ageing-report-2024-fiche-francia.md, fig. 4) [NON TROVATO come testo].

**Ordini di grandezza (Francia, Ageing Report 2024)** (fonti/ec-ageing-report-2024-fiche-francia.md) [V]:
- 2022: 20,3 milioni di pensionati e 29,0 milioni di occupati, rapporto 0,7; 14,4 milioni di persone con 65 anni o più e 37,6 milioni fra 20 e 64 anni, rapporto 0,4. Al 2070 i due rapporti salgono a 0,8 e 0,6 (tab. 11).
- Contributi pensionistici pubblici all'11,1% del PIL nel 2022: 5,8 a carico dei datori, 3,7 dei lavoratori, 1,6 dallo Stato (tab. 16).
- La riforma del 2023 alza l'età minima da 62 a 64 anni di 3 mesi all'anno, per coorte di nascita (chi è nato nel 1962 va in pensione a 62 anni e 6 mesi, chi è nato nel 1968 a 64). È un esempio di riforma che entra a regime in anni, non in un turno: vedi [[ritardi-e-inerzia-delle-politiche]].

Leve del giocatore: età di pensionamento (agisce su $c_a$, introdotta per coorte), aliquota $\tau^c$, regola d'indicizzazione ($k_\pi$, $k_w$), formula di calcolo per i nuovi pensionati (retributiva o contributiva, che cambia $\bar b$ solo per le coorti future) [INF].

## Variabili di stato

Debito (stock, per scadenza media se si vuole il rifinanziamento), tasso medio pagato sul debito (si muove lentamente verso il tasso di mercato man mano che il debito si rinnova), rating (scala a gradini), storia di default (booleana con memoria). Per le pensioni: pensione media (una per coorte di pensionamento, se si vuole che le riforme tocchino solo i nuovi pensionati), età di pensionamento per coorte, aliquota contributiva, regola d'indicizzazione [INF].

## Stabilità numerica

- La dinamica del debito è esplosiva quando $i > g$ e l'avanzo primario non basta: è un comportamento voluto, è la crisi del debito [INF].
- Il pericolo è il circuito spread → tasso → disavanzo → spread: con un premio al rischio lineare forte il circuito diverge in pochi turni. Va smorzato: il tasso medio sul debito si aggiorna solo sulla quota che scade nel turno [INF].
- Laubach misura effetti lineari e piccoli su un paese che si finanzia nella propria valuta; per un paese senza banca centrale propria (eurozona) gli spread reagiscono in modo non lineare oltre una soglia. Questo pezzo non è coperto dalle fonti catturate [NON TROVATO]: va calibrato a parte. Chiuso il 17/10/2026: forma quadratica di De Grauwe-Ji sull'area euro e criteri del TPI in [[economia-italiana-parametri]].

## Implementazioni aperte

Nessuna libreria serve: sono identità e poche funzioni chiuse. Il DSA dell'FMI (framework di sostenibilità del debito) è il riferimento professionale, ma non è stato catturato [NON TROVATO].

## Cosa semplifichiamo

- Un solo tasso medio sul debito e una scadenza media, non la curva dei titoli [INF].
- Il rating come funzione a gradini di un punteggio lineare nei sei fattori di Cantor e Packer, con isteresi (si scende in fretta, si risale piano) [INF].
- Gettito: un'elasticità per tipo d'imposta (reddito, consumi, società, contributi) più la sensibilità al ciclo; l'elasticità ciclica non è stata verificata su fonte primaria (pagina OCSE bloccata) [NON TROVATO]. Chiuso il 17/10/2026 con il metodo comune UE: semi-elasticità del saldo 0,544 per l'Italia ed elasticità per imposta in [[economia-italiana-parametri]].

## Per il nostro gioco

- Prendiamo l'equazione del debito, $\tau^*$ e l'ETI per la curva di Laffer, Laubach per lo spread di base e Cantor e Packer per il rating.
- Default possibile: Democracy 4 non ha l'opzione di default e il debito «non ha quasi conseguenze reali» ([[democracy-4]]). Qui il default è uno stato raggiungibile, con memoria nel rating.
- Il giocatore che alza l'aliquota massima al 90% vede il gettito scendere: la curva di Laffer diventa una lezione giocabile, con i numeri della letteratura invece di una forma inventata.
- Pensioni: il buco del sistema a ripartizione entra nel saldo primario, quindi l'invecchiamento della piramide diventa debito se il giocatore non tocca età, aliquota o indicizzazione. È la pressione lenta e prevedibile che dà senso alle riforme impopolari.
- Aperto: la soglia non lineare degli spread per i paesi dell'eurozona.

## Fonti

- **URL** https://www.ecb.europa.eu/press/economic-bulletin/focus/2019/html/ecb.ebbox201902_06~0c96ee6f7c.en.html · [V]
  **perché conta qui** lettura ufficiale del differenziale tasso-crescita e dell'avanzo che stabilizza il debito
  **cosa se ne prende** la logica della dinamica del debito e l'ordine di grandezza di $i - g$ (fonti/ecb-eb-box-2019-02-i-g.md)
- **URL** https://www.nber.org/system/files/working_papers/w15012/w15012.pdf · [V]
  **perché conta qui** rassegna di riferimento sull'elasticità del reddito imponibile
  **cosa se ne prende** $\tau^* = 1/(1+a e)$ e l'intervallo 0,12-0,4 (fonti/saez-slemrod-giertz-2009-eti.md)
- **URL** https://www.federalreserve.gov/pubs/feds/2003/200312/revision/200312pap.pdf · [V]
  **perché conta qui** effetto misurato di debito e disavanzo sui tassi
  **cosa se ne prende** 3-4 pb per punto di debito, circa 22 pb per punto di disavanzo (fonti/laubach-2003-deficits-interest.md)
- **URL** https://www.newyorkfed.org/research/epr/96v02n2/9610cant.html · [V]
  **perché conta qui** determinanti dei rating sovrani
  **cosa se ne prende** i sei fattori del punteggio di rating (fonti/nyfed-cantor-packer-1996.md)

- **URL** https://www.ecb.europa.eu/press/economic-bulletin/focus/2024/html/ecb.ebbox202405_08~5f9531042f.en.html · [V]
  **perché conta qui** lettura BCE dell'Ageing Report 2024 con la scomposizione della spesa pensionistica
  **cosa se ne prende** i quattro fattori e il loro peso al 2070 (fonti/ecb-eb-box-2024-05-ageing-report.md)
- **URL** https://economy-finance.ec.europa.eu/document/download/e412927a-ea31-406d-bb6c-c925914123e9_en · [V]
  **perché conta qui** scheda paese ufficiale con i numeri di un sistema a ripartizione reale
  **cosa se ne prende** pensionati, occupati, contributi in % del PIL e la riforma per coorte del 2023 (fonti/ec-ageing-report-2024-fiche-francia.md)

## Collegamenti
- [[regole-fiscali-ue]]: il vincolo europeo sulla spesa netta e la procedura per disavanzo eccessivo, che questa pagina non copre.

- [[demografia-e-migrazioni]]: la piramide per coorti da cui escono occupati e pensionati.
- [[ritardi-e-inerzia-delle-politiche]]: le riforme delle pensioni entrano a regime per coorte, in anni.
- [[macro-stock-flow-consistent]]: il debito come stock della matrice settoriale.
- [[banca-centrale-e-inflazione]]: il tasso di policy è il pavimento del tasso sul debito.
- [[mercato-del-lavoro-e-disuguaglianza]]: le tasse cambiano la distribuzione dei redditi.
- [[motore-economico-raccomandato]]
- [[democracy-4]]
