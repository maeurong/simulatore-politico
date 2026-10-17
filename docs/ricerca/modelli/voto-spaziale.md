---
tipo: modello
fonti: [fonti/lewis-king-1999-direzionale-prossimita.md, fonti/gking-lewis-king-1999.md, fonti/schofield-2010-valenza-convergenza.md, fonti/cses-indice-polarizzazione.md]
aggiornato: 2026-10-16
---
# Voto spaziale: prossimità, direzionale, valence

Elettori e partiti stanno in uno spazio di policy (1-3 assi); ogni elettore dà a ogni partito un'utilità che dipende dalla distanza, e vota di conseguenza. È il nucleo più economico per trasformare posizioni in voti, e il modello "valence" spiega perché i partiti non convergono tutti al centro. Per il gioco è la funzione elettore→voto su cui si appoggiano opinion dynamics, sondaggi e coalizioni.

## Equazioni

- **Prossimità** (Downs, Hinich-Enelow): `U_ij = α − β (v_i − c_ij)²`, con β > 0; massima quando elettore e partito coincidono. Variante a distanza assoluta ("city block") al posto di quella quadratica (fonti/lewis-king-1999-direzionale-prossimita.md) [V].
- **Direzionale** (Rabinowitz-Macdonald 1989): con il punto neutro a 0, `U_ij = α + β (v_i · c_ij)`. Premia i partiti "dalla propria parte" e tanto più quanto sono netti; nella teoria completa c'è una "regione di accettabilità" oltre la quale gli estremisti vengono penalizzati (fonti/lewis-king-1999-direzionale-prossimita.md) [V].
- **Probabilistico con valence** (Schofield): `u_ij = λ_j − β ‖x_i − z_j‖² + ε_ij`, con ε di tipo Gumbel, quindi le probabilità di voto sono un logit multinomiale. λ_j è la valence: competenza, leadership, tratti percepiti, indipendenti dalla posizione (fonti/schofield-2010-valenza-convergenza.md) [V].
- **Coefficiente di convergenza**: `c = 2β (1 − 2ρ₁) σ²`, con ρ₁ la quota del partito a valence più bassa quando tutti stanno al centro e σ² la varianza totale degli elettori sugli assi. Condizione necessaria perché il "centro" sia equilibrio: `c < w` (w = numero di assi); sufficiente in 2D: `c < 1`. Se fallisce, il partito più debole si sposta verso le ali (fonti/schofield-2010-valenza-convergenza.md) [V].

## Evidenza

- **Contro il confronto prossimità/direzionale**: Lewis e King mostrano che i test empirici fra i due modelli si riducono a scegliere un'assunzione statistica non verificabile con i dati disponibili, e che cambiando l'assunzione cambiano le conclusioni (fonti/gking-lewis-king-1999.md) [V]. Implicazione: nessuno dei due è "il vero"; si sceglie per comportamento di gioco.
- **A favore della valence**: le stime su Israele, Turchia, USA e Regno Unito indicano che la convergenza al centro è un caso raro; nei dati USA 2008 e UK il modello puramente elettorale predice posizioni vicine al centro, e la divergenza osservata si spiega aggiungendo la valence degli attivisti (fonti/schofield-2010-valenza-convergenza.md) [V].
- Molte stime della teoria formale assumono la prossimità (fonti/lewis-king-1999-direzionale-prossimita.md) [V].

## Parametri tipici

- β stimato: circa 0,85 negli USA 2008 (modello spaziale puro, logit condizionale) e circa 0,27 nel Regno Unito 1979; i valori dipendono dalla scala degli assi e non si trasferiscono tali e quali (fonti/schofield-2010-valenza-convergenza.md) [V].
- Scala: le posizioni dei partiti si prendono da CHES (sinistra-destra 0-10, GAL-TAN) o da Manifesto; quelle percepite dagli elettori da CSES (fonti/cses-indice-polarizzazione.md) [V].
- Numero di assi: 2 è il compromesso usato da Schofield e da Laver-Sergenti [INF].

## Costo computazionale

`O(N·P·D)` per elezione (N elettori-campione, P partiti, D assi). Con 10.000 elettori, 10 partiti, 2 assi sono 200.000 operazioni a turno: trascurabile [INF]. Il logit aggiunge un `exp` per coppia elettore-partito [INF].

## Cosa semplifichiamo

- Assi fissi e uguali per tutti (gli elettori reali pesano gli assi in modo diverso) [INF].
- Valence come scalare per partito e non per elettore; la valence per gruppo sociale (età, religione, reddito) è il primo raffinamento, ed è quello che Schofield chiama modello "joint" (fonti/schofield-2010-valenza-convergenza.md) [V].
- Nessun voto strategico: lo aggiungiamo solo dove il sistema elettorale lo rende visibile (collegi uninominali, soglie) [INF].

## Per il nostro gioco

- Prendere il logit con valence come regola base: posizione = scelte di policy, valence = scandali, leader, competenza percepita, campagne. Dà probabilità lisce, quindi sondaggi e risultati non saltano a gradini come le soglie di Democracy (vedi [[elettorato-e-gruppi-di-voto]]).
- Il coefficiente c è un indicatore di progettazione: se gli avversari IA convergono tutti al centro, la varianza degli elettori o le differenze di valence sono troppo basse.
- Direzionale come opzione per elettori "identitari" (un sottoinsieme), non come regola globale: l'evidenza non permette di preferirlo.

## Collegamenti

- [[voto-economico-e-retrospettivo]]: l'economia entra come termine aggiuntivo nella valence del governo.
- [[opinion-dynamics]]: muove le posizioni v_i nel tempo.
- [[abm-elettorali]]: regole con cui i partiti spostano c_j.
- [[polarizzazione-e-fiducia-istituzionale]]: la varianza σ² è la polarizzazione ideologica.
- [[elettorato-e-gruppi-di-voto]]: come lo fanno i giochi esistenti.
- [[ches]], [[cses]], [[manifesto-project]]: da dove prendere le posizioni.
- [[partiti-e-coalizioni]]: come i giochi esistenti posizionano (o non posizionano) i partiti avversari.
- [[cittadini-simulati]]: agenti campione o gruppi: chi porta le posizioni v_i.

## Riferimenti

- **URL** https://gking.harvard.edu/files/spatial.pdf · [V]
  **perché conta qui** formalizza prossimità e direzionale nella stessa notazione e mostra che l'evidenza empirica non sceglie fra i due
  **cosa se ne prende** le due funzioni di utilità e la decisione di non trattare il direzionale come regola globale
- **URL** https://emis.muni.cz/journals/HOA/IJMMS/Volume2010/562813.pdf · [V]
  **perché conta qui** modello logit con valence, coefficiente di convergenza, stime di β (open access CC BY)
  **cosa se ne prende** la regola di voto base, c come indicatore di taratura, ordine di grandezza di β
- **URL** https://cses.org/data-download/download-data-documentation/party-system-polarization-index-for-cses-modules-1-4/ · [V]
  **perché conta qui** posizioni dei partiti percepite dagli elettori, pesate per voto
  **cosa se ne prende** la fonte per tarare le posizioni iniziali e la varianza
- Schofield 2007, "The Mean Voter Theorem", Review of Economic Studies 74(3) · [INF]
  **perché conta qui** è il teorema originale; letto solo tramite l'applicazione del 2010
  **cosa se ne prende** nulla in più: la versione 2010 basta per implementare
