<script lang="ts">
  import { tick } from "svelte";
  import { t } from "$lib/testi/t";
  import Pulsante from "./Pulsante.svelte";

  type Props = {
    etichetta: string;
    conferma: string;
    annulla?: string;
    /** cosa si perde, scritto accanto ai due pulsanti */
    spiegazione?: string;
    onconferma: () => void;
  };

  let { etichetta, conferma, annulla = t("componenti.annulla"), spiegazione, onconferma }: Props = $props();
  const id = $props.id();
  let aperta = $state(false);
  let radice: HTMLSpanElement;

  async function apri(stato: boolean) {
    aperta = stato;
    await tick();
    // chi conferma può smontare il componente: allora il fuoco lo decide lui
    radice?.querySelector("button")?.focus();
  }

  function tasto(e: KeyboardEvent) {
    if (aperta && e.key === "Escape") {
      e.preventDefault();
      apri(false);
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<span class="conferma" bind:this={radice} onkeydown={tasto}>
  {#if aperta}
    <Pulsante forma="chiude" distruttivo aria-describedby={spiegazione ? `${id}-spieg` : undefined}
      onclick={() => { onconferma(); apri(false); }}>{conferma}</Pulsante>
    <Pulsante forma="bordo" onclick={() => apri(false)}>{annulla}</Pulsante>
    {#if spiegazione}<span id="{id}-spieg" class="spieg">{spiegazione}</span>{/if}
  {:else}
    <Pulsante forma="bordo" distruttivo onclick={() => apri(true)}>{etichetta}</Pulsante>
  {/if}
</span>

<style>
  .conferma { display: inline-flex; align-items: center; gap: 6px; flex-wrap: wrap }
  .spieg { font-size: var(--c-min); color: var(--grigio) }
</style>
