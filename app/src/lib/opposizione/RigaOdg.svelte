<script lang="ts">
  // Riga dell'ordine del giorno all'opposizione (28-opposizione.prototype.html, forma A): segno, titolo, azioni a destra, sotto; i figli stanno sotto, a tutta larghezza.
  import type { Snippet } from "svelte";

  type Props = { titolo: string; sotto?: string; segno?: "" | "fatto" | "urg" | "info"; scelta?: boolean; onscegli?: () => void; azioni?: Snippet; children?: Snippet };
  let { titolo, sotto = "", segno = "", scelta = false, onscegli, azioni, children }: Props = $props();
</script>

<li class="r" class:sel={scelta}>
  <span class="seg {segno}" aria-hidden="true"></span>
  {#if onscegli}<button type="button" class="t" aria-pressed={scelta} onclick={onscegli}>{titolo}</button>{:else}<span class="t">{titolo}</span>{/if}
  {#if azioni}<span class="a" role="group" aria-label={titolo}>{@render azioni()}</span>{/if}
  {#if sotto}<span class="s">{sotto}</span>{/if}
  {#if children}<div class="fig">{@render children()}</div>{/if}
</li>

<style>
  .r { display: grid; grid-template-columns: 22px minmax(0, 1fr) auto; gap: 2px 10px; align-items: center; padding: 6px; border-bottom: var(--filo) }
  .r.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .seg { width: 14px; height: 14px; border: 1.5px solid var(--ink) }
  .seg.fatto { background: var(--ink) }
  .seg.info { border-style: dashed; border-color: var(--grigio) }
  .seg.urg { border-color: var(--rosso); background: var(--retino) }
  .t { font-weight: 700; line-height: 1.2; text-align: left }
  button.t:hover { text-decoration: underline; text-underline-offset: 2px }
  .a { grid-row: 1 / span 2; grid-column: 3; display: flex; gap: 4px 6px; align-items: center; justify-content: flex-end; flex-wrap: wrap }
  .s { grid-column: 2; font-size: var(--c-min); color: var(--grigio); line-height: 1.25 }
  .fig { grid-column: 2 / -1 }
</style>
