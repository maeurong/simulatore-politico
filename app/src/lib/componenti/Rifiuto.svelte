<script lang="ts">
  let { testo }: { testo: string } = $props();
</script>

<span class="rifiuto" role="alert">{testo}</span>

<style>
  .rifiuto { font-size: var(--c-sec); font-weight: 700; color: var(--rosso) }
</style>
