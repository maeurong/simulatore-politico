<script lang="ts">
  // Bilancio, forma B · Bilancia (22-bilancio.prototype.html): il giogo in testa, entrate e spesa a fronte; il lato arriva col Task 9.
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Giogo from "./Giogo.svelte";
  import Entrate from "./Entrate.svelte";
  import Spesa from "./Spesa.svelte";
  import { bozza } from "./bozza.svelte";

  let { vista }: { vista: Vista } = $props();

  $effect(() => void bozza.apri(vista.turno));
  const bilancio = $derived(bozza.corrente ?? bozza.base);
</script>

<div class="bilancio">
  <div class="foglio">
    {#if bilancio}
      <div class="capo">
        <h3 class="tit">{t("bilancio.titolo_bozza", { anno: bilancio.anno })}</h3>
        <span class="sec muto">{t("bilancio.sotto_bozza", { anno: vista.bilancio.anno })}</span>
      </div>
      <Giogo riepilogo={bozza.riepilogo} rifiuto={bozza.rifiuto} inVolo={bozza.inVolo} />
      <div class="fronte">
        <Entrate {bilancio} />
        <Spesa {vista} {bilancio} />
      </div>
    {:else if bozza.rifiuto}
      <Giogo riepilogo={null} rifiuto={bozza.rifiuto} inVolo={false} />
    {:else if bozza.base === null && !bozza.inVolo}
      <p class="sec muto">{t("bilancio.senza_bozza")}</p>
    {/if}
  </div>
  <aside class="lato">
    <p class="min muto">{t("bilancio.nota_giogo")}</p>
  </aside>
</div>

<style>
  .bilancio { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); align-items: start }
  .bilancio > * { min-width: 0 }
  .foglio { display: grid; gap: 8px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); align-self: stretch; display: grid; gap: 8px; align-content: start }
  .fronte { display: grid; grid-template-columns: minmax(0, .8fr) minmax(0, 1.4fr); gap: 0 var(--canale) }
  .fronte > :global(section) { min-width: 0; display: grid; gap: 6px; align-content: start }
  .fronte > :global(section + section) { border-left: var(--filo); padding-left: var(--canale) }

  /* i pezzi comuni a giogo, entrate e spesa (prototipo 22); :where() li tiene sotto le regole di ciascun pezzo */
  .bilancio :global(:where(.capo)) { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .bilancio :global(:where(.tit)) { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  :global(:root.grave) .bilancio :global(:where(.tit)) { font-weight: 600 }
  .bilancio :global(:where(.grp)) {
    display: flex; justify-content: space-between; gap: 8px; padding-bottom: 2px; border-bottom: var(--bordo);
    font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio)
  }
  .bilancio :global(:where(.sec)) { font-size: var(--c-sec) }
  .bilancio :global(:where(.min)) { font-size: var(--c-min) }
  .bilancio :global(:where(.muto)) { color: var(--grigio) }
  .bilancio :global(:where(.male)) { color: var(--rosso) }
  .bilancio :global(:where(.bene)) { color: var(--verde) }
  .bilancio :global(:where(.link)) { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec) }
  .bilancio :global(:where(.link:hover)) { text-decoration-thickness: 2px }
</style>
