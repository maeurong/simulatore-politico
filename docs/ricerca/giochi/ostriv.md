---
tipo: gioco
fonti: [fonti/ostriv-faq.md, fonti/ostriv-gog-changelog.md, fonti/ostriv-michet-blog.md, fonti/steam-api-snapshot-gestionali.md, fonti/steam-recensioni-gestionali.md]
aggiornato: 2026-10-16
---
# Ostriv

City builder storico (Ucraina, XVIII secolo) di un solo sviluppatore, Yevhen8, in accesso anticipato dal 2020. Simula famiglie che comprano, mangiano, si ammalano e vengono tassate per professione, su un motore C++ proprio che chiede 2 GB di RAM. È la prova che un gioco con cittadini individuali e una grafica curata può restare leggero se il motore è fatto su misura.

## Scheda

| Campo | Valore |
|---|---|
| Uscita | 19/03/2020 (accesso anticipato), Windows (fonti/steam-api-snapshot-gestionali.md) |
| Recensioni Steam | 7.717, 95% (Overwhelmingly Positive) (fonti/steam-api-snapshot-gestionali.md) |
| Requisiti minimi | quad core, 2 GB RAM, GPU con OpenGL 4.3, 600 MB disco; "con la crescita della città servono più risorse" (fonti/steam-api-snapshot-gestionali.md) |
| Tecnologia | C++17 senza motori di terze parti, OpenGL, FMOD, Freetype, LibPNG; editor di modelli interno (fonti/ostriv-faq.md) |

## Scala e loop

Un villaggio che diventa città, a posizionamento libero senza griglia, con percorsi pedonali che si formano dove la gente cammina e catene produttive lunghe: il vetro richiede vetreria, cava di sabbia e a monte miniere che a loro volta chiedono carrelli di ferro (fonti/ostriv-michet-blog.md).

## Come modella la cosa

- **Famiglie con denaro proprio** che comprano beni; nessuna classe sociale codificata: il ricco compra di più ma non chiede beni diversi. Le tasse si regolano per professione, e il giocatore deve tassare a parte le professioni ricche perché accumulano tutto il denaro della città (fonti/ostriv-michet-blog.md).
- **Stagioni**: d'inverno fattorie e pesca si fermano, alla semina i cittadini lasciano il lavoro per andare nei campi (fonti/ostriv-michet-blog.md).
- **Fedeltà che si accende con la popolazione**: infortuni e malattie compaiono solo verso i 200 abitanti (un resoconto di stampa sull'Alpha 5 parla di 250, https://mezha.ua/en/2023/03/20/the-ukrainian-urban-planning-strategy-ostriv-has-received-an-alpha-5-update/, non catturato); a 400 abitanti senza taverna rifornita smette l'immigrazione (fonti/ostriv-michet-blog.md).
- Lo sviluppo procede per alpha senza date fisse; il changelog ufficiale è ripubblicato sul forum GOG (fonti/ostriv-gog-changelog.md; fonti/ostriv-faq.md).

## Realismo: cosa dicono i giocatori

Si lodano i dettagli che rendono visibile la vita delle famiglie: i parenti che visitano la tomba di un anziano, le famiglie che emigrano piuttosto che morire di fame, le strade che spiegano i percorsi (fonti/steam-recensioni-gestionali.md). Il blogger critica le soglie fisse di popolazione come pressione "molto dura e irrealistica" accanto a pressioni morbide (fonti/ostriv-michet-blog.md).

## Prestazioni

Il più leggero fra i titoli moderni della lista: 2 GB di RAM, 600 MB di disco (fonti/steam-api-snapshot-gestionali.md). Le FAQ suggeriscono di spegnere il post-processing sulle GPU Intel integrate (fonti/ostriv-faq.md).

## Per il nostro gioco

- **Prendiamo:** tasse per categoria professionale come leva fiscale concreta; stagionalità che mette sotto stress lavoro e produzione; dettagli narrativi a costo zero (la tomba visitata) per legare il giocatore agli individui; il motore su misura come prova che la leggerezza è una scelta d'architettura.
- **Evitiamo:** soglie di popolazione rigide che accendono sistemi interi senza spiegazione.
- **Aperto:** la "fedeltà che cresce con la scala" come tecnica, purché motivata e annunciata al giocatore.

## Collegamenti

- [[cittadini-simulati]]: famiglie individuali a basso costo.
- [[prestazioni-su-hardware-modesto]]: motore proprio, 2 GB.
- [[territorio-e-urbanistica]]: posizionamento libero e sentieri.
