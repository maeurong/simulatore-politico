<script lang="ts">
  // Le quote per partito a barre, coi colori dei partiti e il nome sempre accanto (docs/ricerca/estetica/mappa-svg-delle-regioni.md:136).
  import { percentuale } from "$lib/testi/t";
  import { coloreDi } from "$lib/partiti";
  import type { QuotaPartito } from "./paese";

  let { righe, giocatore }: { righe: QuotaPartito[]; giocatore: string } = $props();

  const scala = $derived(Math.max(0.01, ...righe.map((r) => r.quota)));
</script>

<div class="barre">
  {#each righe as p (p.id)}
    <span class="n" class:tu={p.id === giocatore}>{p.nome}</span>
    <span class="b"><i style:width="{(p.quota / scala) * 100}%" style:background={coloreDi(p.id) ?? "var(--retino)"}></i></span>
    <span class="q" class:tu={p.id === giocatore}>{percentuale(p.quota, 1)}</span>
  {/each}
</div>

<style>
  .barre { display: grid; grid-template-columns: minmax(0, 150px) minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; font-size: var(--c-sec) }
  .n { overflow-wrap: anywhere }
  .b { display: block; height: 12px; position: relative }
  .b i { position: absolute; inset: 0 auto 0 0; border: 1px solid var(--ink) }
  .q { text-align: right; white-space: nowrap; font-variant-numeric: tabular-nums }
  .tu { font-weight: 700 }
</style>
