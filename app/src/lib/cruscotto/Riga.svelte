<script lang="ts">
  // Riga dell'ordine del giorno (17-cruscotto.prototype.html, forma A): casella di stato, titolo, rimando, stato sotto.
  type Props = { titolo: string; stato: string; rimando: string; href: string; fatta: boolean; urgente?: boolean };
  let { titolo, stato, rimando, href, fatta, urgente = false }: Props = $props();
</script>

<li class="r">
  <span class="seg" class:fatto={fatta} class:urg={urgente && !fatta} aria-hidden="true"></span>
  <span class="t">{titolo}</span>
  <a class="link" {href} aria-label="{rimando.replace(/\s*›$/, '')}: {titolo}">{rimando}</a>
  <span class="s">{stato}</span>
</li>

<style>
  .r { display: grid; grid-template-columns: 22px minmax(0, 1fr) auto; gap: 2px 10px; align-items: center; padding: 6px; border-bottom: var(--filo) }
  .seg { width: 14px; height: 14px; border: 1.5px solid var(--ink) }
  .seg.fatto { background: var(--ink) }
  .seg.urg { border-color: var(--rosso); background: var(--retino) }
  .t { font-weight: 700; line-height: 1.2 }
  .link { color: inherit; text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec); white-space: nowrap }
  .link:hover { text-decoration-thickness: 2px }
  .s { grid-column: 2 / -1; font-size: var(--c-min); color: var(--grigio); line-height: 1.25 }
</style>
