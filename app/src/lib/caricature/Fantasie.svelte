<script lang="ts">
  // Le sei fantasie del colletto (prototipo 35, pat0-pat5), in inchiostro su carta; montato una volta in +layout.svelte.
  // Fuori schermo e non display:none, che in alcuni browser spegne i pattern. Ordine e nomi: FANTASIE in kit.ts.
</script>

<svg width="0" height="0" style="position:absolute" aria-hidden="true" focusable="false">
  <defs>
    <pattern id="fantasia-pieno" width="4" height="4" patternUnits="userSpaceOnUse"><rect width="4" height="4" class="i" /></pattern>
    <pattern id="fantasia-vuoto" width="4" height="4" patternUnits="userSpaceOnUse"><rect width="4" height="4" class="c" /></pattern>
    <pattern id="fantasia-righe" width="2.6" height="2.6" patternUnits="userSpaceOnUse"><rect width="2.6" height="2.6" class="c" /><rect width="2.6" height="1.2" class="i" /></pattern>
    <pattern id="fantasia-pois" width="3" height="3" patternUnits="userSpaceOnUse"><rect width="3" height="3" class="c" /><circle cx="1.5" cy="1.5" r=".8" class="i" /></pattern>
    <pattern id="fantasia-quadretti" width="3.4" height="3.4" patternUnits="userSpaceOnUse"><rect width="3.4" height="3.4" class="i" /><rect width="1.7" height="1.7" class="c" /><rect x="1.7" y="1.7" width="1.7" height="1.7" class="c" /></pattern>
    <pattern id="fantasia-tratteggio" width="2.4" height="2.4" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><rect width="2.4" height="2.4" class="c" /><rect width=".9" height="2.4" class="i" /></pattern>
  </defs>
</svg>

<style>
  .i { fill: var(--ink) }
  .c { fill: var(--carta) }
</style>
