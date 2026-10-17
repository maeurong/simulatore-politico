---
tipo: modello
fonti: [fonti/ball-leigh-loungani-2013-okun.md, fonti/wikipedia-gini-coefficient.md, fonti/rudebusch-svensson-1999-policy-rules.md, fonti/worldbank-terms-datasets.md]
aggiornato: 2026-10-17
---
# Mercato del lavoro e disuguaglianza: Okun, Gini, distribuzione dei redditi

La disoccupazione segue l'output gap con un coefficiente che varia da paese a paese (legge di Okun). La distribuzione dei redditi si descrive con pochi parametri, e il Gini ne esce in forma chiusa. Per il nostro gioco sono due cose: il numero che gli elettori sentono (disoccupazione) e la forma della società su cui agiscono tasse e trasferimenti.

## Equazioni chiave

**Legge di Okun** (in forma di gap): $u_t - u^*_t = \beta\,(y_t - y^*_t)$. Coefficiente stimato: -0,4 / -0,5 per gli USA con $R^2$ intorno a 0,8, stabile nel tempo e anche durante la Grande Recessione; -0,15 in Giappone, -0,45 negli USA, -0,85 in Spagna. Le differenze riflettono istituzioni del lavoro (impiego a vita in Giappone, contratti temporanei in Spagna) e non sono spiegate dalla protezione dell'impiego (fonti/ball-leigh-loungani-2013-okun.md) [V]. Nella stessa fonte l'Italia vale -0,254 (-0,358 dopo il 1995); i coefficienti delle 20 regioni sono in [[economia-italiana-parametri]].

**Salari e prezzi**: la Phillips di Rudebusch-Svensson usa l'output gap, e Okun lo traduce in disoccupazione; non serve un'equazione salariale a parte per il motore base (fonti/rudebusch-svensson-1999-policy-rules.md) [V per la Phillips, INF per la scelta].

**Distribuzione log-normale**: se il reddito è log-normale con deviazione standard dei logaritmi $\sigma$, allora $G = \operatorname{erf}(\sigma/2) = 2\Phi(\sigma/\sqrt2) - 1$ (fonti/wikipedia-gini-coefficient.md) [V]. Al contrario $\sigma = 2\,\operatorname{erf}^{-1}(G)$: dal Gini pubblicato si ricostruisce tutta la distribuzione.
Curva di Lorenz della log-normale: $L(p) = \Phi(\Phi^{-1}(p) - \sigma)$, quindi la quota di reddito di ogni decile è chiusa [INF, risultato standard non catturato].

**Gini da gruppi discreti** (decili o quintili): $G = 1 - \sum_k (X_k - X_{k-1})(Y_k + Y_{k-1})$ con $X$ quota cumulata di popolazione e $Y$ quota cumulata di reddito [INF, formula di Brown standard].

## Variabili di stato

- Tasso di disoccupazione e disoccupazione strutturale $u^*$ (si muove lentamente, isteresi dopo recessioni lunghe) [INF].
- Reddito medio di 10 decili (o 5 quintili) prima e dopo tasse e trasferimenti. Il Gini è derivato, non uno stato.

## Stabilità numerica

Okun è una relazione statica: nessuna dinamica propria. Il rischio è solo di uscire dal dominio ($u < 0$); va limitato con un minimo frizionale [INF]. La distribuzione per decili è un vettore aggiornato con tassi di crescita per gruppo: stabile per costruzione, purché le quote vengano rinormalizzate a ogni turno [INF].

## Dati

- Gini e quote per decile: World Development Indicators e Poverty and Inequality Platform della Banca Mondiale, CC BY 4.0 di default (fonti/worldbank-terms-datasets.md) [V]; vedi [[world-bank-wdi]].
- World Inequality Database (wid.world): la licenza dei dati non è stata trovata su fonte primaria [NON TROVATO].

## Cosa semplifichiamo

- 10 decili invece di agenti individuali: abbastanza per Gini, povertà relativa e gruppi di voto per reddito [INF].
- Una sola $u^*$ per paese, niente flussi lordi assunzioni-licenziamenti, niente curva di Beveridge [INF].
- Salario minimo, contrattazione, riforme del lavoro agiscono su $u^*$ e sul coefficiente di Okun, non con micro-meccanismi [INF].

## Per il nostro gioco

- Prendiamo Okun con coefficiente per paese (Giappone -0,15, Spagna -0,85: la stessa recessione pesa in modo molto diverso).
- Prendiamo il Gini come numero derivato dai 10 decili: ogni politica fiscale sposta il reddito di qualche decile e il Gini si aggiorna da solo, senza formule dedicate.
- I decili sono il ponte verso l'elettorato: un gruppo di voto può essere legato al reddito di un decile (vedi [[democracy-4]] per il modello a gruppi). Aperto: chi definisce i gruppi di voto (altro filone).

## Fonti

- **URL** https://www.nber.org/system/files/working_papers/w18668/w18668.pdf · [V]
  **perché conta qui** stima recente e comparata del coefficiente di Okun
  **cosa se ne prende** i coefficienti per paese e la loro stabilità (fonti/ball-leigh-loungani-2013-okun.md)
- **URL** https://en.wikipedia.org/wiki/Gini_coefficient · [V]
  **perché conta qui** formula chiusa del Gini per la log-normale
  **cosa se ne prende** $G = \operatorname{erf}(\sigma/2)$ per ricostruire la distribuzione dal Gini (fonti/wikipedia-gini-coefficient.md)
- **URL** https://www.worldbank.org/ext/en/legal/terms-conditions/datasets · [V]
  **perché conta qui** licenza dei dati di disuguaglianza della Banca Mondiale
  **cosa se ne prende** CC BY 4.0 di default, con eccezioni per dati di terzi (fonti/worldbank-terms-datasets.md)

## Collegamenti

- [[macro-neokeynesiano-piccolo]]: l'output gap che Okun traduce.
- [[finanza-pubblica]]: tasse e trasferimenti che spostano i decili.
- [[world-bank-wdi]]: dati di calibrazione.
- [[our-world-in-data]]: ripubblica i dati WID sulla disuguaglianza (pagina di un altro filone).
- [[motore-economico-raccomandato]]
- [[mercati-e-pops]]: salari e tenore di vita dei pop di Victoria 3.
