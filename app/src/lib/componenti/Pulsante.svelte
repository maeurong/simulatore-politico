<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  type Props = Omit<HTMLButtonAttributes, "disabled"> & {
    forma: "chiude" | "bordo" | "piccolo" | "link";
    scelto?: boolean;
    distruttivo?: boolean;
    /** il motivo: il pulsante resta nel Tab e il motivo gli sta accanto */
    spento?: string;
    onclick?: (e: MouseEvent) => void;
    children: Snippet;
  };

  let { forma, scelto, distruttivo = false, spento, onclick, children, ...resto }: Props = $props();
  const id = $props.id();

  function clic(e: MouseEvent) {
    if (!spento) onclick?.(e);
  }
</script>

{#snippet pulsante()}
  <button
    {...resto}
    type="button"
    class={[forma === "link" ? "link" : ["btn", forma], { scelto, via: distruttivo }]}
    aria-pressed={scelto}
    aria-disabled={spento ? "true" : undefined}
    aria-describedby={spento ? `${id}-motivo` : resto["aria-describedby"]}
    onclick={clic}>{@render children()}</button>
{/snippet}

{#if spento}
  <span class="spento">{@render pulsante()}<span id="{id}-motivo" class="motivo">{spento}</span></span>
{:else}
  {@render pulsante()}
{/if}

<style>
  .btn {
    border: var(--forte); padding: 4px 11px; font-size: var(--c-corpo); font-weight: 700; background: var(--carta); white-space: nowrap;
    transition: transform calc(var(--t-press) * var(--k)) var(--e-work);
  }
  .btn:hover { background: var(--ink); color: var(--carta) }
  .btn:active { transform: scale(.97) }
  .chiude { background: var(--ink); color: var(--carta) }
  .chiude:hover { background: var(--carta); color: var(--ink) }
  .piccolo { padding: 1px 7px; font-size: var(--c-min); border-width: 1.5px }
  .btn.scelto { background: var(--giallo); color: var(--ink) }
  :global(:root.grave) .btn.scelto { background: var(--ink); color: var(--carta) }
  .btn[aria-disabled="true"] { border-style: dashed; border-color: var(--grigio); color: var(--grigio); cursor: default; background: var(--carta) }
  .btn[aria-disabled="true"]:active { transform: none }
  .btn.via:not([aria-disabled="true"]) { border-color: var(--rosso); color: var(--rosso) }
  .btn.via:not([aria-disabled="true"]):is(:hover, .chiude) { background: var(--rosso); color: var(--carta); border-color: var(--rosso) }

  .link { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec) }
  .link:hover { text-decoration-thickness: 2px }
  .link.via { color: var(--rosso) }
  .link[aria-disabled="true"] { color: var(--grigio); text-decoration-style: dashed; cursor: default }

  .spento { display: inline-flex; align-items: center; gap: 6px }
  .motivo { font-size: var(--c-min); color: var(--grigio) }
</style>
