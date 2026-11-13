<script lang="ts">
  import type { Component } from "svelte";
  import { goto } from "$app/navigation";

  // import() dentro DEV: nel build il ramo cade e il componente di prova non diventa un chunk
  let Prova = $state<Component | null>(null);
  if (import.meta.env.DEV) import("$lib/sviluppo/ProvaComponenti.svelte").then((m) => (Prova = m.default));
  else goto("/", { replaceState: true });
</script>

{#if Prova}<Prova />{/if}
