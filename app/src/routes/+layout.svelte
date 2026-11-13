<script lang="ts">
  import "$lib/stile/fondamenta.css";
  import { avvisoScala, scorciatoia } from "$lib/impostazioni.svelte";

  let { children } = $props();
</script>

<svelte:window onkeydown={scorciatoia} />

{@render children()}

<!-- l'avviso breve delle scorciatoie della scala (prototipo 33, .toast) -->
<p class="avviso" role="status">{avvisoScala.testo}</p>

<style>
  .avviso { position: fixed; top: 110px; right: var(--marg); z-index: 5; background: var(--ink); color: var(--carta); font-size: var(--c-sec); padding: 5px 10px; pointer-events: none }
  .avviso:empty { display: none }
</style>
