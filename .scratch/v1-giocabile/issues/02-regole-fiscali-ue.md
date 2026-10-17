Type: research
Status: resolved
Blocked by: -
Label: wayfinder:research
Map: ../map.md

## Question

Quali regole fiscali UE vincolano il bilancio italiano nel 2026 (governance economica riformata nel 2024: piano strutturale di bilancio, traiettoria della spesa netta, procedura per disavanzo eccessivo, clausole di salvaguardia), e cosa succede in concreto a chi le viola? Serve la forma minima per modellarle come vincolo e come fonte di eventi nella v1.

La knowledge base non le copre: docs/ricerca/modelli/finanza-pubblica.md tratta debito, spread e rating, non le regole UE.

## Answer

Un solo vincolo operativo dal 2024: **tetto annuo alla crescita della spesa netta**, fissato dal Consiglio sul piano strutturale di medio termine (4-5 anni; un governo nuovo può presentarne uno riveduto). Dettaglio e fonti in `docs/ricerca/modelli/regole-fiscali-ue.md`.

- Spesa netta: esclusi interessi, fondi UE, componenti cicliche e una tantum; un taglio di tasse conta come spesa.
- Salvaguardie: debito -1 punto l'anno sopra il 90%; deficit strutturale verso l'1,5%.
- **Conto di controllo**: scostamenti cumulati, soglie 0,3% annuo e 0,6% cumulato.
- **PDE**: si apre con deficit sopra il 3% o con conto sforato (debito sopra il 60%); richiede azione efficace entro 6 mesi e almeno 0,5% di PIL di aggiustamento l'anno; senza azione efficace scattano un'ammenda dello 0,2% del PIL, la sospensione dei fondi di coesione e poi 0,05% a semestre. Si chiude con deficit sotto il 3% a consuntivo e nelle previsioni.
- Clausole: generale (un anno, prorogabile) e nazionale (difesa fino all'1,5% del PIL fino al 2028, energia dal 2026).
- **Italia oggi**: PDE aperta il 26/7/2024 e sospesa il 3/6/2026, non chiusa; tetti di spesa netta 1,6% / 1,9% / 1,7% / 1,5% dal 2026 al 2029; clausola per difesa ed energia raccomandata dalla Commissione l'8/10/2026, adozione del Consiglio non verificata.

Forma proposta per la v1 (raccomandazione del researcher, da decidere nel 07): stato `tetto_spesa_netta[anno]`, `conto_controllo`, macchina a stati della PDE, clausole; il controllo avviene una volta l'anno nel turno del pacchetto di primavera, negli altri turni si vede la proiezione; eventi a data fissa.

Da segnalare al 07: la spesa per la difesa italiana vale 1,3% del PIL in classificazione UE (COFOG) e 2,01% in misura NATO, che è quella usata in `docs/ricerca/modelli/prontezza-e-spesa-per-la-difesa.md`. Non trovati: l'adozione della clausola da parte del Consiglio, se un'ammenda sia mai stata applicata, l'art. 123 TFUE.
