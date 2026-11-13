<script lang="ts">
  // Barra dei seggi coi colori dei partiti (prototipo 27, «camere»). Il colore non basta mai: ogni tratto ha il nome nel
  // title e nell'elenco accanto (docs/ricerca/estetica/mappa-svg-delle-regioni.md:136).
  import { coloreDi } from "$lib/partiti";
  import { numero } from "$lib/testi/t";

  /** `segno`: i seggi della coalizione, un segno nero sotto la barra dal bordo sinistro. */
  let { righe, totale, segno }: { righe: { id: string; nome: string; seggi: number }[]; totale: number; segno?: number } = $props();

  const con = $derived(righe.filter((r) => r.seggi > 0));
  const meta = $derived(((Math.floor(totale / 2) + 1) / totale) * 100);
</script>

<div class="barra">
  <div class="stk" aria-hidden="true">
    {#each con as r (r.id)}
      <i title="{r.nome}: {numero(r.seggi)}" style:width="{(r.seggi / totale) * 100}%" style:background={coloreDi(r.id) ?? undefined} class:nato={coloreDi(r.id) === null}></i>
    {/each}
    <b class="meta" style:left="{meta}%"></b>
    {#if segno}<b class="segno" style:width="{(segno / totale) * 100}%"></b>{/if}
  </div>
  <ul class="elenco">
    {#each con as r (r.id)}
      <li><i class="sw" style:background={coloreDi(r.id) ?? undefined} class:nato={coloreDi(r.id) === null}></i>{r.nome} <b>{numero(r.seggi)}</b></li>
    {/each}
  </ul>
</div>

<style>
  .stk { display: flex; height: 16px; border: 1px solid var(--ink); position: relative; margin-bottom: 6px }
  .stk i { height: 100%; border-right: 1px solid var(--carta) }
  .meta { position: absolute; top: -4px; bottom: -4px; width: 2px; background: var(--rosso) }
  .segno { position: absolute; left: 0; bottom: -6px; height: 4px; background: var(--ink) }
  .nato { background: var(--retino), var(--carta) }
  .elenco { list-style: none; display: flex; flex-wrap: wrap; gap: 2px 14px; font-size: var(--c-min); color: var(--grigio) }
  .sw { display: inline-block; width: 10px; height: 10px; border: 1px solid var(--ink); margin-right: 5px; vertical-align: -1px }
  .elenco b { color: var(--ink) }
</style>
