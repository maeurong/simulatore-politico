<script lang="ts">
  import { t } from "$lib/testi/t";

  /** sezione: l'etichetta già maiuscola, com'è nel sommario */
  let { sezione, fetta }: { sezione: string; fetta: string } = $props();
  // il nome in neretto e in --ink, il resto in grigio (08-fondamenta.prototype.html:141-142)
  const [prima, dopo] = $derived(t("segnaposto.arriva", { fetta }).split("{sezione}"));
</script>

<p class="ph">{prima}<b>{sezione}</b>{dopo}</p>

<style>
  .ph { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 7px 11px; font-size: var(--c-sec) }
  .ph b { color: var(--ink) }
</style>
