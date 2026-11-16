<script lang="ts">
  // Conti, forma A · Listino (21-conti.prototype.html, posti dello schema 19 da 36-scelte-11d-conti): bilancio, economia, soglie; il lato legge `scelta`.
  import type { Vista } from "$lib/tipi/Vista";
  import ColonnaBilancio from "./ColonnaBilancio.svelte";
  import ColonnaEconomia from "./ColonnaEconomia.svelte";
  import Soglie from "./Soglie.svelte";
  import CentroStudi from "./CentroStudi.svelte";
  import DettaglioConti from "./DettaglioConti.svelte";
  import { tick } from "svelte";
  import { esiste } from "./conti";

  let { vista }: { vista: Vista } = $props();
  /** La riga scelta: `dic:<id>`, `irpef:<i>`, `fondo`, `stat:<campo>`, `spread`, `sit:<i>`, `grandezze`, `gr:<id>`, `debito`, `ue`, `mes`, `ristr`, `red:<fascia>`. */
  let scelta = $state<string | null>(null);
  // una Vista nuova senza la riga scelta (il MES finito) riporta il lato al centro studi
  $effect(() => { if (scelta && !esiste(vista, scelta)) scelta = null; });
  let listino = $state<HTMLElement>();
  let lato = $state<HTMLElement>();
  async function torna() {
    const da = listino?.querySelector<HTMLElement>('[aria-pressed="true"]');
    scelta = null;
    await tick();
    (da ?? lato?.querySelector("button"))?.focus();
  }
</script>

<div class="conti">
  <div class="listino" bind:this={listino}>
    <ColonnaBilancio {vista} bind:scelta />
    <ColonnaEconomia {vista} bind:scelta />
    <Soglie {vista} bind:scelta />
  </div>
  <aside class="lato" bind:this={lato}>
    {#if scelta && esiste(vista, scelta)}<DettaglioConti {vista} bind:scelta {torna} />{:else}<CentroStudi {vista} />{/if}
  </aside>
</div>

<style>
  .conti { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); align-items: start }
  .conti > * { min-width: 0 }
  .lato { border-left: var(--forte); padding-left: var(--canale); align-self: stretch; display: grid; gap: 8px; align-content: start }
  .listino { display: grid; grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr) minmax(0, 1fr); align-content: start }
  .listino > :global(section) { padding: 0 14px; border-left: var(--filo); display: grid; grid-template-columns: minmax(0, 1fr); gap: 6px; align-content: start; min-width: 0 }
  .listino > :global(section:first-child) { padding-left: 0; border-left: 0 }

  /* i pezzi comuni alle tre colonne e al lato (prototipo 21); :where() li tiene sotto le regole di ciascuna colonna */
  .conti :global(:where(.capo)) { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: 4px }
  .conti :global(:where(.tit)) { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  :global(:root.grave) .conti :global(:where(.tit)) { font-weight: 600 }
  .conti :global(:where(.grp)) {
    display: flex; justify-content: space-between; gap: 8px; margin-top: 8px; padding-bottom: 2px; border-bottom: var(--bordo);
    font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio);
  }
  .conti :global(:where(.r)) { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 0 8px; align-items: baseline; width: 100%; padding: 3px 4px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .conti :global(:where(.r:hover)) { background: var(--velo) }
  .conti :global(:where(.r.sel)) { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .conti :global(:where(.r .n)) { white-space: nowrap; overflow: hidden; text-overflow: ellipsis }
  .conti :global(:where(.r .x)) { white-space: nowrap; font-weight: 700; text-align: right }
  .conti :global(:where(.r .x small)), .conti :global(:where(.r .n small)) { font-size: inherit; font-weight: 400; color: var(--grigio) }
  .conti :global(:where(.r.due)) { grid-template-columns: minmax(0, 1fr) }
  .conti :global(:where(.r.due .x)) { text-align: left; white-space: normal }
  .conti :global(:where(.sec)) { font-size: var(--c-sec) }
  .conti :global(:where(.min)) { font-size: var(--c-min) }
  .conti :global(:where(.muto)) { color: var(--grigio) }
  .conti :global(:where(.male)) { color: var(--rosso) }
  .conti :global(:where(.bene)) { color: var(--verde) }
  .conti :global(:where(.chip)) { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .conti :global(:where(.chip.rosso)) { border-color: var(--rosso); color: var(--rosso) }
  .conti :global(:where(.chip.pieno)) { background: var(--rosso); border-color: var(--rosso); color: var(--carta) }
  .conti :global(:where(.dati)) { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 3px 12px; font-size: var(--c-sec); align-items: baseline }
  .conti :global(:where(.dati dt)) { color: var(--grigio) }
  .conti :global(:where(a.link)) { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec) }
  .conti :global(:where(a.link:hover)) { text-decoration-thickness: 2px }
  .lato :global(:where(.link)) { justify-self: start }
  .lato :global(:where(h4.tit)) { font-size: var(--c-sez) }
</style>
