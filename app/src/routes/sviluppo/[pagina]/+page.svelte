<script lang="ts">
  import type { Component } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  // Il campione delle caricature (/sviluppo/caricature) sta sotto un segmento dinamico, non in una cartella sua: il manifesto
  // delle rotte del build elenca i percorsi, e così nel build non resta nemmeno il nome della pagina di prova.
  // import() dentro DEV: nel build il ramo cade e il componente di prova non diventa un chunk
  let Prova = $state<Component | null>(null);
  if (import.meta.env.DEV && page.params.pagina === "caricature") import("$lib/sviluppo/ProvaCaricature.svelte").then((m) => (Prova = m.default));
  else goto("/", { replaceState: true });
</script>

{#if Prova}<Prova />{/if}
