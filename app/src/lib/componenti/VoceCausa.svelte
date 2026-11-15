<script lang="ts">
  // Voce con causa (prototipo 08): il fatto in neretto, sotto il perché in grigio.
  import { t } from "$lib/testi/t";
  let { fatto, causa }: { fatto: string; causa?: string } = $props();
</script>

<div class="voce">
  <b>{fatto}</b>
  {#if causa}<small><i>{t("componenti.perche")}</i> {causa}</small>{/if}
</div>

<style>
  .voce { display: grid; gap: 1px; padding: 6px 0; border-top: var(--filo); font-size: var(--c-sec) }
  .voce:first-child { border-top: 0 }
  small { color: var(--grigio); font-size: var(--c-min) }
</style>
