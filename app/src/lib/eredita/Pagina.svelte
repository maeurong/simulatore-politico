<script lang="ts">
  // L'eredità in bozza dentro la partita: la Vista non porta lo scenario, lo dà l'indice della partita in corso.
  import { partita } from "$lib/partita.svelte";
  import type { Vista } from "$lib/tipi/Vista";
  import Eredita from "./Eredita.svelte";
  import { daVista } from "./eredita";

  let { vista }: { vista: Vista } = $props();
  let scenario = $state<string | null>(null);
  $effect(() => {
    let vivo = true;
    partita.indice().then((i) => vivo && (scenario = i?.testata.scenario ?? ""), () => vivo && (scenario = ""));
    return () => (vivo = false);
  });
  const nome = $derived(vista.partiti.find((p) => p.id === vista.giocatore)?.nome ?? "");
  const edizione = $derived(vista.eredita && scenario !== null ? daVista(vista, partita.serie, nome, scenario, partita.serie[0]?.data ?? vista.data) : null);
</script>

{#if edizione}<Eredita {edizione} azioni={vista.eredita?.bozza ?? false} />{/if}
