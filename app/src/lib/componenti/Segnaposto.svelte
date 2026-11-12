<script lang="ts">
  import { t } from "$lib/testi/t";

  /** sezione: l'etichetta già maiuscola, com'è nel sommario */
  let { sezione, fetta }: { sezione: string; fetta: string } = $props();
</script>

<p class="ph">{t("segnaposto.arriva", { sezione, fetta })}</p>

<style>
  .ph { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 7px 11px; font-size: var(--c-sec) }
</style>
