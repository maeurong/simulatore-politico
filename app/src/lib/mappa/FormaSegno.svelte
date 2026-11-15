<script lang="ts">
  // La forma di un segno, centrata sull'origine: triangolo per il cantiere, cerchio per l'atto, quadrato per l'opera, croce rossa per l'emergenza.
  import type { Segno } from "./Mappa.svelte";

  let { s }: { s: Segno } = $props();
</script>

{#if s === "cantiere"}<path d="M0,-6L6,5L-6,5Z" fill="var(--ink)" stroke="var(--carta)" stroke-width="1.2" />
{:else if s === "atto"}<circle r="5" fill="var(--ink)" stroke="var(--carta)" stroke-width="1.2" />
{:else if s === "opera"}<rect x="-5" y="-5" width="10" height="10" fill="var(--ink)" stroke="var(--carta)" stroke-width="1.2" />
{:else}<circle r="7" fill="var(--rosso)" stroke="var(--carta)" stroke-width="1.2" /><path d="M-3.5,0H3.5M0,-3.5V3.5" stroke="var(--carta)" stroke-width="2" />{/if}
