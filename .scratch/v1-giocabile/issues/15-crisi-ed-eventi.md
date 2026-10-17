Type: grilling
Status: open
Blocked by: -
Label: wayfinder:grilling
Map: ../map.md

## Question

Come nascono gli eventi nella v1: trigger su soglie del modello, regista adattivo che dosa la tensione, o entrambi? Quali famiglie di eventi entrano (shock economici già stocastici nel motore, crisi del debito con la scelta MES/ristrutturazione, emergenze pagate dal fondo di riserva, sfiducia individuale a un ministro, scandali che toccano la valence) e come ogni evento cita la sua causa dentro il modello (`docs/ricerca/sistemi/crisi-ed-eventi.md`)?

Ingressi: Modello economico (07) dà gli shock con seme fisso e la soglia delle aste deserte; Dicasteri (03) il fondo di riserva; Autonomia dei ministri (04) la sfiducia individuale come evento; Elettorato (06) valence e spinte degli eventi sulle posizioni dei gruppi; Giro del turno (09) i tempi: gli eventi nascono nell'ultimo passo della chiusura, si mostrano solo in apertura, si affrontano nello stesso turno (decreto, fondo di riserva) e, se ignorati, producono i loro effetti nella chiusura.

Ingresso da Opposizione (12): la sfiducia individuale a un ministro è anche una leva dell'opposizione, non solo un evento. Passa se la vota un alleato del governo in ultimatum che dalla caduta del ministro guadagna; i partiti IA all'opposizione la usano contro i ministri del giocatore. Resta da decidere qui se esiste anche come evento indipendente (scandalo che la innesca).

Ingresso da Catalogo atti e riforme (16): la condizionalità del MES si esprime come valori imposti a disposizioni (per esempio età pensionabile, indicizzazione). Un atto può avere una disposizione come prerequisito.

Ingresso da Partiti IA (18): il governo avversario risponde a ogni evento con la sua opzione di default (09), per decreto solo se l'evento lo consente; ogni evento deve quindi avere un default sensato per chiunque governi. All'opposizione i partiti IA presentano la sfiducia, anche individuale, solo quando passerebbe; il bersaglio di quella individuale è il ministro del dicastero col giudizio peggiore fra i gruppi. Una scissione a metà legislatura può aprire una crisi (05).
