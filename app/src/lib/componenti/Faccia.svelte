<script lang="ts">
  // Per ora le iniziali nel cerchio d'inchiostro; il disegno arriva col Task 15 (caricature).
  import { iniziali } from "$lib/partiti";
  let { nome, misura }: { nome: string; misura: 34 | 46 | 62 | 84 } = $props();
</script>

<span class="iniz" style:width="{misura}px" style:height="{misura}px" style:font-size="{Math.max(12, Math.round(misura * 0.36))}px" aria-hidden="true">{iniziali(nome)}</span>

<style>
  .iniz { display: inline-grid; place-items: center; flex: none; border: 1.5px solid var(--ink); border-radius: 50%; font-weight: 700; letter-spacing: .02em }
</style>
