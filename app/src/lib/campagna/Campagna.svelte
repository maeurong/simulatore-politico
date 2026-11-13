<script lang="ts">
  // Campagna, forma A · Tavolo (26-campagna.prototype.html): tre colonne e la cassa a lato. Spesa, posizione e cassa arrivano coi Task 6 e 7.
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Coalizione from "./Coalizione.svelte";

  let { vista }: { vista: Vista } = $props();
</script>

<div class="tavolo">
  <div class="col"><h3 class="capo tit">{t("campagna.coalizione")}</h3><Coalizione {vista} /></div>
  <div class="col"><h3 class="capo tit">{t("campagna.spesa")}</h3></div>
  <div class="col"><h3 class="capo tit">{t("campagna.posizione")}</h3></div>
  <aside class="lato"><h3 class="capo tit">{t("campagna.cassa")}</h3></aside>
</div>

<style>
  .tavolo { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1.15fr) minmax(0, 1fr) var(--lato); min-height: 0 }
  .tavolo > * { min-width: 0 }
  .col { padding: 0 var(--canale) 12px 0 }
  .col + .col { border-left: var(--filo); padding-left: var(--canale) }
  .lato { border-left: var(--forte); padding-left: var(--canale) }
  .capo { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-riq); border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: 6px }
</style>
