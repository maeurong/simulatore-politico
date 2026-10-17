---
tipo: dataset
fonti: [fonti/manifesto-terms.md, fonti/manifesto-about.md]
aggiornato: 2026-10-16
---
# Manifesto Project (MARPOR)

Analisi del contenuto dei programmi elettorali: per ogni partito e ogni elezione, la quota di frasi dedicata a ciascuna categoria di policy. Serve a posizionare i partiti su assi multipli e a tarare le loro agende. È gratuito da consultare, ma i termini **vietano la ridistribuzione** senza autorizzazione scritta.

## Contenuto

- Dataset principale: oltre 1400 partiti, dal 1945 a oggi, oltre 67 paesi; aggiornato una volta l'anno; accessibile via pacchetto R manifestoR e API (fonti/manifesto-about.md) [V].
- Corpus: oltre 3200 programmi leggibili a macchina, più di 2000 annotati a livello di quasi-frase, oltre 2 milioni di quasi-frasi (fonti/manifesto-about.md) [V].
- Modelli linguistici manifestoberta addestrati su circa 1,9 milioni di frasi annotate (fonti/manifesto-about.md) [V].
- Indice sinistra-destra "RILE" e 56 categorie di codifica [INF].

## Licenza

- I dati sono forniti per la ricerca scientifica (fonti/manifesto-terms.md) [V].
- **Ridistribuzione vietata** salvo autorizzazione scritta del Manifesto Project; se autorizzata, vanno inclusi tutti i file di accompagnamento e i termini d'uso (fonti/manifesto-terms.md) [V].
- Obbligo di citare, di comunicare le pubblicazioni che usano i dati e di inviarne una copia (fonti/manifesto-terms.md) [V].
- I dati vanno conservati in modo che se ne riconosca la provenienza, con i termini d'uso nelle cartelle (fonti/manifesto-terms.md) [V].

## Ridistribuibile in un repo pubblico?

**No**, non i file del dataset (fonti/manifesto-terms.md) [V]. Usare valori derivati a mano (es. posizione di un partito su un asse, arrotondata) è una zona grigia contrattuale da verificare con il filone legale [INF].

## Per il nostro gioco

- Usarlo in fase di progettazione per scegliere assi e agende tipiche delle famiglie di partito, non come file incluso nel gioco.
- Se servono posizioni dei partiti nel gioco, preferire [[ches]] (licenza da chiarire) o chiedere l'autorizzazione scritta al Manifesto Project.

## Collegamenti

- [[voto-spaziale]]: posizioni dei partiti sugli assi.
- [[ches]]: alternativa con posizioni stimate da esperti.
- [[soggetti-reali-vs-fittizi]]: partiti reali o fittizi.

## Riferimenti

- **URL** https://manifesto-project.wzb.eu/information/documents/terms_of_use · [V]
  **perché conta qui** termini d'uso che vietano la ridistribuzione
  **cosa se ne prende** il divieto di includere il dataset nel repo
- **URL** https://manifesto-project.wzb.eu/information/documents/information · [V]
  **perché conta qui** descrive copertura e prodotti
  **cosa se ne prende** dimensione del dataset e del corpus
