<script lang="ts">
  // Conti, forma A · Listino (21-conti.prototype.html, posti dello schema 19 da 36-scelte-11d-conti): bilancio, economia, soglie; il lato legge `scelta`.
  import type { Vista } from "$lib/tipi/Vista";
  import ColonnaBilancio from "./ColonnaBilancio.svelte";
  import ColonnaEconomia from "./ColonnaEconomia.svelte";
  import Soglie from "./Soglie.svelte";

  let { vista }: { vista: Vista } = $props();
  /** La riga scelta: `dic:<id>`, `irpef:<i>`, `fondo`, `stat:<campo>`, `spread`, `sit:<i>`, `grandezze`, `debito`, `ue`, `mes`, `ristr`, `red:<fascia>`. */
  let scelta = $state<string | null>(null);
</script>

<div class="conti">
  <div class="listino">
    <ColonnaBilancio {vista} bind:scelta />
    <ColonnaEconomia {vista} bind:scelta />
    <Soglie {vista} bind:scelta />
  </div>
  <aside class="lato"></aside>
</div>

<style>
  .conti { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); align-items: start }
  .conti > * { min-width: 0 }
  .lato { border-left: var(--forte); padding-left: var(--canale); align-self: stretch; display: grid; gap: 8px; align-content: start }
  .listino { display: grid; grid-template-columns: minmax(0, 1.2fr) minmax(0, 1fr) minmax(0, 1fr); align-content: start }
  .listino > :global(section) { padding: 0 14px; border-left: var(--filo); display: grid; grid-template-columns: minmax(0, 1fr); gap: 6px; align-content: start; min-width: 0 }
  .listino > :global(section:first-child) { padding-left: 0; border-left: 0 }

  /* i pezzi comuni alle tre colonne (prototipo 21); :where() li tiene sotto le regole di ciascuna colonna */
  .listino :global(:where(.capo)) { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: 4px }
  .listino :global(:where(.tit)) { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  :global(:root.grave) .listino :global(:where(.tit)) { font-weight: 600 }
  .listino :global(:where(.grp)) {
    display: flex; justify-content: space-between; gap: 8px; margin-top: 8px; padding-bottom: 2px; border-bottom: var(--bordo);
    font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio);
  }
  .listino :global(:where(.r)) { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 0 8px; align-items: baseline; width: 100%; padding: 3px 4px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .listino :global(:where(.r:hover)) { background: var(--velo) }
  .listino :global(:where(.r.sel)) { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .listino :global(:where(.r .n)) { white-space: nowrap; overflow: hidden; text-overflow: ellipsis }
  .listino :global(:where(.r .x)) { white-space: nowrap; font-weight: 700; text-align: right }
  .listino :global(:where(.r .x small)), .listino :global(:where(.r .n small)) { font-size: inherit; font-weight: 400; color: var(--grigio) }
  .listino :global(:where(.r.due)) { grid-template-columns: minmax(0, 1fr) }
  .listino :global(:where(.r.due .x)) { text-align: left; white-space: normal }
  .listino :global(:where(.sec)) { font-size: var(--c-sec) }
  .listino :global(:where(.min)) { font-size: var(--c-min) }
  .listino :global(:where(.muto)) { color: var(--grigio) }
  .listino :global(:where(.male)) { color: var(--rosso) }
  .listino :global(:where(.bene)) { color: var(--verde) }
  .listino :global(:where(.chip)) { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .listino :global(:where(.chip.rosso)) { border-color: var(--rosso); color: var(--rosso) }
  .listino :global(:where(.chip.pieno)) { background: var(--rosso); border-color: var(--rosso); color: var(--carta) }
</style>
