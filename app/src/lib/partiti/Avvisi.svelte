<script lang="ts">
  // Gli avvisi di un partito, uguali nella scheda e nel dettaglio a lato: l'ultimatum e la minaccia di scissione sono rossi, sciolto e senza seggi sono tratteggiati.
  import { t, type Chiave } from "$lib/testi/t";

  let { lista }: { lista: Chiave[] } = $props();
</script>

{#if lista.length}
  <span class="avvisi">{#each lista as k (k)}<span class="chip" class:rosso={k === "avviso.ultimatum" || k === "avviso.ultimatum_governo" || k === "avviso.minaccia_scissione"} class:tr={k === "avviso.sciolto" || k === "avviso.senza_seggi"}>{t(k)}</span>{/each}</span>
{/if}

<style>
  .avvisi { display: flex; flex-wrap: wrap; gap: 3px }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .chip.tr { border-style: dashed; border-color: var(--grigio); color: var(--grigio) }
</style>
