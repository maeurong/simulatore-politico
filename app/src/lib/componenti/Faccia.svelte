<script lang="ts">
  // Con `ruolo` e un nome disegna la caricatura; senza, o nel tono grave (:root.grave), le iniziali nel cerchio d'inchiostro,
  // nello stesso riquadro. Il tono grave si legge dalla classe sulla radice, come il resto del CSS: nessun movimento, nessun accento.
  // Il nome sta sempre scritto accanto: la faccia è nascosta ai lettori di schermo. `partito` è il nome visualizzato del partito,
  // chiave dell'hash come nel prototipo 32 (32-caricature.prototype.html:588), non l'id.
  import { iniziali } from "$lib/partiti";
  import Caricatura from "$lib/caricature/Caricatura.svelte";
  import type { Ruolo } from "$lib/caricature/caricatura";
  let { nome, misura, ruolo, partito = "" }: { nome: string; misura: 34 | 46 | 62 | 84; ruolo?: Ruolo; partito?: string } = $props();
  const disegna = $derived(ruolo !== undefined && nome !== "");
</script>

{#if disegna}<span class="disegno" aria-hidden="true"><Caricatura ruolo={ruolo!} {partito} {nome} {misura} /></span>{/if}
<span class="iniz" class:solo={!disegna} style:width="{misura}px" style:height="{misura}px" style:font-size="{Math.max(12, Math.round(misura * 0.36))}px" aria-hidden="true">{iniziali(nome)}</span>

<style>
  .disegno { display: inline-flex; flex: none }
  .iniz { display: none; place-items: center; flex: none; border: 1.5px solid var(--ink); border-radius: 50%; font-weight: 700; letter-spacing: .02em }
  .iniz.solo, :global(:root.grave) .iniz { display: inline-grid }
  :global(:root.grave) .disegno { display: none }
</style>
