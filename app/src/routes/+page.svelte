<script lang="ts">
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";

  // ponytail: l'Edicola vera arriva col Task 12; qui solo la strada verso la partita seminata dal finto
</script>

<main class="provvisoria">
  {#if partita.vista}<a href="/partita/cruscotto">{t("sommario.cruscotto")} ›</a>{:else}<p>{t("edicola.nessuna_in_corso")}</p>{/if}
</main>

<style>
  .provvisoria { padding: var(--s3) var(--marg) }
  a { text-decoration: underline; text-underline-offset: 2px }
</style>
