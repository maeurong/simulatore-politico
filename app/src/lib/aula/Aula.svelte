<script lang="ts">
  // Aula, forma D · Misto (19-aula.prototype.html): il calendario dei posti in testa, sotto quattro schede (In aula, Disposizioni, Legge elettorale,
  // Referendum), la misura scelta nel dettaglio a lato. Con Disposizioni e Legge elettorale il dettaglio a lato è la bozza. La pagina sta ferma, il corpo e il dettaglio scorrono.
  import { tick, untrack } from "svelte";
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import { riportaFuoco, rifiutoPer } from "./aula";
  import { bozzaDelComando, bozzaInvariata, cambia, rifiutoDellaBozza, type Bozza as TipoBozza } from "./bozza";
  import Bozza from "./Bozza.svelte";
  import Disposizioni from "./Disposizioni.svelte";
  import Elettorale from "./Elettorale.svelte";
  import Referendum from "./Referendum.svelte";
  import Calendario from "./Calendario.svelte";
  import InAula from "./InAula.svelte";
  import Misura from "./Misura.svelte";

  let { vista }: { vista: Vista } = $props();

  const SCHEDE = ["in_aula", "disposizioni", "elettorale", "referendum"] as const;
  let scheda = $state<(typeof SCHEDE)[number]>("in_aula");
  let scelta = $state<number | null>(null);
  // la legge in composizione e la riga aperta: solo in memoria, al rientro vale la Vista
  let bozza = $state<TipoBozza>([]);
  let rigaAperta = $state<string | null>(null);
  // senza scelta: il bilancio nel suo turno (occupa il calendario), altrimenti la prima misura
  const corrente = $derived(vista.calendario.find((m) => m.id === scelta) ?? vista.calendario.find((m) => m.forma === "bilancio") ?? vista.calendario[0] ?? null);
  const numeri = $derived({
    in_aula: vista.calendario.length,
    disposizioni: vista.disposizioni.filter((d) => d.norma !== "elettorale").length,
    elettorale: null,
    referendum: vista.referendum.length || null,
  });

  // al turno nuovo il calendario è un altro: la scelta di prima non vale. `turno` è un derived: una Vista nuova dello stesso turno
  // (dopo un comando) non fa rigirare l'effetto, e la misura scelta e il cartellino aperto restano.
  const turno = $derived(vista.turno);
  $effect.pre(() => {
    void turno;
    untrack(() => {
      scelta = null;
      bozza = [];
      rigaAperta = null;
    });
  });

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiutato = $state.raw<{ testo: string; tipo: Comando["tipo"]; misura: number | null; vista: Vista | null; bozza: string | null; disposizione: string | null } | null>(null);
  const rifiuto = $derived(rifiutoPer(rifiutato, vista, corrente?.id ?? null));
  let lato: HTMLElement;
  // `misura`: quella a cui il rifiuto si lega; la bozza non ne ha (null)
  async function manda(c: Comando, misura: number | null = corrente?.id ?? null): Promise<boolean> {
    if (partita.occupato) return false;
    rifiutato = null;
    // la misura della chiamata, non quella scelta quando arriva la risposta
    const da = lato.contains(document.activeElement) ? document.activeElement : null;
    try {
      const ok = (await partita.comanda(c)) !== null;
      await tick();
      if (ok) riportaFuoco(lato, da);
      return ok;
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), tipo: c.tipo, misura, vista: partita.vista, bozza: bozzaDelComando(c), disposizione: c.tipo === "promuovi_referendum" ? c.disposizione : null };
      return false;
    }
  }

  // un cambio della bozza che smonta il bottone col fuoco lo porta al dettaglio; la bozza va svuotata solo se il comando è riuscito
  async function nellaBozza(f: () => void) {
    const da = lato.contains(document.activeElement) ? document.activeElement : null;
    f();
    await tick();
    riportaFuoco(lato, da);
  }
  async function presenta(c: Comando): Promise<boolean> {
    const da = lato.contains(document.activeElement) ? document.activeElement : null;
    const ok = await manda(c, null);
    if (ok && bozzaInvariata(c, bozza)) {
      bozza = [];
      await tick();
      riportaFuoco(lato, da);
    }
    return ok;
  }
</script>

<div class="aula">
  <div class="capo">
    <h3 class="tit">{t("aula.titolo")}</h3>
    <span class="sec muto">{t(vista.governa ? "aula.testata_governo" : "aula.testata_opposizione")}</span>
  </div>
  <div class="corpo">
    <Calendario {vista} scelta={corrente?.id ?? null} onscelta={(id) => { scelta = id; scheda = "in_aula"; }} />
    <div class="schede" role="group" aria-label={t("aula.schede")}>
      {#each SCHEDE as s (s)}
        <button type="button" class:qui={scheda === s} aria-pressed={scheda === s} aria-label={numeri[s] !== null ? `${t(`aula.scheda_${s}`)}, ${numeri[s]}` : undefined} onclick={() => (scheda = s)}>
          {t(`aula.scheda_${s}`)}{#if numeri[s] !== null}<small>{numeri[s]}</small>{/if}
        </button>
      {/each}
    </div>
    {#if scheda === "in_aula"}
      <InAula {vista} scelta={corrente?.id ?? null} onscelta={(id) => (scelta = id)} />
    {:else if scheda === "disposizioni"}
      <Disposizioni {vista} {bozza} aperta={rigaAperta} onapri={(id) => (rigaAperta = id)} oncambia={(d, g) => (bozza = cambia(vista, bozza, d, g))}
        manda={(c) => manda(c, null)} rifiuto={rifiutoPer(rifiutato, vista, null)} />
    {:else if scheda === "elettorale"}
      <Elettorale {vista} {bozza} oncambia={(d, g) => (bozza = cambia(vista, bozza, d, g))} onbozza={(b) => (bozza = b)} />
    {:else}
      <Referendum {vista} manda={(c) => manda(c, null)} rifiuto={rifiutoPer(rifiutato, vista, null)} />
    {/if}
  </div>
  <aside class="lato" bind:this={lato} tabindex="-1" aria-label={t("aula.dettaglio")}>
    {#if scheda === "disposizioni" || scheda === "elettorale"}
      <Bozza
        {vista} {bozza} scheda={scheda} manda={presenta} rifiuto={rifiutoDellaBozza(rifiutato, vista, bozza)}
        ontoglie={(d, g) => nellaBozza(() => (bozza = cambia(vista, bozza, d, g)))} onsvuota={() => nellaBozza(() => (bozza = []))}
        onapri={(id) => { scelta = id; scheda = "in_aula"; }} />
    {:else if corrente}
      {#key `${vista.turno}:${corrente.id}`}
        <Misura {vista} m={corrente} {manda} {rifiuto} />
      {/key}
    {:else}
      <p class="sec muto">{t("aula.niente_in_esame")}</p>
    {/if}
  </aside>
</div>

<style>
  .aula { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .aula > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .corpo { min-height: 0; overflow: hidden auto; display: grid; gap: 10px; align-content: start }
  .schede { display: flex; gap: 2px; border-bottom: var(--forte) }
  .schede button { padding: 4px 12px; font-weight: 700; color: var(--grigio); border: 1.5px solid transparent; border-bottom: 0; white-space: nowrap }
  .schede button.qui { color: var(--ink); border-color: var(--ink); background: var(--carta); margin-bottom: -2px; padding-bottom: 6px }
  .schede small { font-weight: 400; margin-left: 4px }
  .lato { border-left: var(--forte); padding-left: var(--canale); min-height: 0; overflow: hidden auto; padding-bottom: 8px; display: grid; gap: 8px; align-content: start }
  .lato:focus-visible { outline-offset: -2px }
</style>
