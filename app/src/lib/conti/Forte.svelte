<script lang="ts">
  // Un testo con i segnaposto `{b}` riempiti, in ordine, dai valori in grassetto.
  let { testo, vals }: { testo: string; vals: string[] } = $props();
</script>

{#each testo.split("{b}") as pezzo, i (i)}{pezzo}{#if i < vals.length}<b>{vals[i]}</b>{/if}{/each}
