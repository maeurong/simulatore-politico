<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";

  let { children, ...resto }: HTMLAttributes<HTMLElement> & { children: Snippet } = $props();
</script>

<aside {...resto} class="dettaglio">{@render children()}</aside>

<style>
  .dettaglio {
    width: var(--lato); flex: none; min-height: 0; overflow: hidden auto;
    border: var(--bordo); padding: var(--s2) var(--s3);
    display: grid; gap: var(--s2); align-content: start;
  }
</style>
