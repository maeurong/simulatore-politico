---
tipo: dataset
fonti: [fonti/unodc-data-portal-home.md, fonti/unodc-legal.md, fonti/un-terms-of-use.md]
aggiornato: 2026-10-16
---
# UNODC: dati su criminalità, giustizia, droghe, corruzione

Il portale dati dell'Ufficio ONU contro la droga e il crimine (data.unodc.org, prima dataunodc.un.org) raccoglie: omicidi intenzionali, crimini violenti e sessuali, popolazione carceraria, funzionamento della giustizia, corruzione e reati economici, droghe, traffico di armi (fonti/unodc-data-portal-home.md). I vecchi URL dataunodc.un.org reindirizzano alla nuova home [M] (`curl -sL -w '%{url_effective}' https://dataunodc.un.org/en/node/651`, 16/10/2026).

## Licenza e ridistribuibilità
- Il link «Terms and Conditions» nel piè di pagina del portale punta a https://www.unodc.org/unodc/legal.html [M] (`curl -sL https://data.unodc.org/ | grep -oiE '<a[^>]*>[^<]*Terms and Conditions'`, 16/10/2026; defuddle toglie il piè di pagina dalla cattura).
- Quei termini concedono di scaricare e copiare i materiali **solo per uso personale e non commerciale, senza diritto di rivendere, ridistribuire o creare opere derivate**, salvo restrizioni più specifiche (fonti/unodc-legal.md); è lo stesso testo dei termini ONU generali (fonti/un-terms-of-use.md).
- [NON TROVATO] in fonte primaria catturata: la clausola di libero riuso con citazione che una ricerca web attribuisce a una vecchia pagina dataUNODC in cinese (non verificata, il portale è stato migrato).
- **Conclusione [INF]**: i dati UNODC non si ridistribuiscono nel gioco. Si usano per calibrare (ordini di grandezza dei tassi di omicidio e detenzione) e si cita la fonte. Prima di un uso diverso serve una richiesta a UNODC.

## Per il nostro gioco
- Tassi di omicidio e detenzione per paese come obiettivi di calibrazione del modello di [[criminalita-e-giustizia]].

## Collegamenti
- [[criminalita-e-giustizia]], [[burocrazia-corruzione-e-capacita-statale]].
- [[modding-e-dati-esterni]] (filone tecnico): come i dati entrano nel gioco.
