---
tipo: gioco
fonti: [fonti/steam-command-modern-operations.md, fonti/wikipedia-command-mano.md, fonti/govuk-dstl-command-pe.md]
aggiornato: 2026-10-16
---
# Command: Modern Operations (e Command Professional Edition)

Simulatore operativo aria-mare-terra-spazio-cyber dal dopoguerra a oggi, su mappa globale, con un database di piattaforme reali. È lo stesso motore che la Difesa britannica ha acquistato nel 2024 per wargaming e analisi. Per noi è il polo opposto del nostro gioco: tutto dettaglio, nessuna politica. Ci insegna come mostrare l'incertezza dell'esito e dove fermarci col dettaglio.

## Scheda

- Sviluppo: Command Development Team (in origine Warfare Sims, studio greco); editore Matrix Games. *Command: Modern Air Naval Operations* esce il 24 settembre 2013; il seguito *Command: Modern Operations* il 14 novembre 2019, compatibile con gli scenari del predecessore (fonti/wikipedia-command-mano.md, fonti/steam-command-modern-operations.md).
- Steam: "Very Positive", 85% su 1.447 recensioni (16/10/2026); richiede l'accettazione di un EULA di terze parti; interfaccia solo in inglese (fonti/steam-command-modern-operations.md).
- Erede dichiarato della serie *Harpoon* (fonti/wikipedia-command-mano.md).

## Come modella

- Controllo operativo di unità su una mappa 2D della Terra, con missioni predefinite o ordini diretti ("vai qui, fai questo"). La scala va dallo scontro fra motovedette alla guerra globale; il solo limite è l'hardware (fonti/wikipedia-command-mano.md).
- Domini: aria, mare, subacqueo, terra, spazio (satelliti sulle orbite note), cyber (degrado o disattivazione di comunicazioni e sensori); guerra elettronica e C4ISR (fonti/steam-command-modern-operations.md, fonti/govuk-dstl-command-pe.md).
- Database vastissimo di piattaforme, costruito da fonti aperte. Nessuna esecuzione dello stesso scenario è uguale a un'altra: il caso è parte del modello (fonti/govuk-dstl-command-pe.md).
- Editor di scenari in tempo reale; scenari condivisi sullo Steam Workshop (fonti/wikipedia-command-mano.md).

## Edizione professionale

- Presentata nel 2015 per organizzazioni della difesa: accesso completo all'editing del database, multiplayer WEGO con arbitro, **modalità Monte Carlo** per l'analisi statistica, import/export dei dati, funzioni vendute a moduli. Tra i primi clienti BAE Systems (fonti/wikipedia-command-mano.md).
- Agosto 2024: il Defence Modelling and Simulation Office del comando strategico britannico firma un contratto iniziale di un anno per Command PE, per wargaming e analisi a livello operativo. Motivazione: nato come gioco commerciale, è facile da imparare (fonti/govuk-dstl-command-pe.md).

## Cosa prendiamo

- **Esito come distribuzione.** Il Monte Carlo professionale è la prova che anche gli analisti vogliono vedere la varianza, non un numero. Il nostro risolutore aggregato (vedi [[lanchester-e-modelli-di-combattimento-aggregati]]) deve mostrare un intervallo di esiti [INF].
- **Spazio e cyber come degrado di capacità**, non come teatri a sé: si tolgono sensori e comunicazioni, e la forza efficace cala. È un'astrazione leggera e coerente col nostro livello [INF].
- **Dal gioco commerciale all'uso professionale.** Un modello leggibile vale anche per gli esperti; la credibilità viene dalla trasparenza delle regole [INF].

## Cosa evitiamo

- Il dettaglio a piattaforma: il capo di governo non sposta fregate. Per noi l'unità minima è la capacità aggregata di teatro [INF].
- La dipendenza da un database di equipaggiamenti reali: ha gli stessi problemi di licenza di [[iiss-military-balance]] [INF].

## Collegamenti
- [[lanchester-e-modelli-di-combattimento-aggregati]]: la nostra alternativa aggregata.
- [[matrix-games]]: l'altro modo con cui la Difesa gioca, all'estremo opposto (narrativo).
- [[hedgemony]]: il livello strategico che Command non tocca.
- [[militare-e-guerra]]: la meccanica di guerra del nostro gioco.

## Riferimenti
- **URL** https://www.gov.uk/government/news/new-wargaming-and-analysis-software-for-defence · [V]
  **perché conta qui** prova primaria che il motore è adottato da una Difesa nazionale
  **cosa se ne prende** spazio e cyber come degrado; esito non deterministico
- **URL** https://en.wikipedia.org/wiki/Command:_Modern_Air_Naval_Operations · [V] (secondaria)
  **perché conta qui** storia della serie e funzioni dell'edizione professionale
  **cosa se ne prende** la modalità Monte Carlo come modello di presentazione dell'esito
- **URL** https://store.steampowered.com/app/1076160/Command_Modern_Operations/ · [V]
  **perché conta qui** scheda commerciale, ricezione, EULA
  **cosa se ne prende** il pubblico c'è anche per un simulatore molto tecnico (1.447 recensioni)
