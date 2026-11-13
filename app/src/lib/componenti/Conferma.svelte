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
    /** la forma del primo pulsante, prima della conferma: il link rosso dell'edicola (13-edicola.prototype.html:235) */
    forma?: "bordo" | "link";
    /** il primo pulsante a riposo nero, non rosso: il rosso resta ai due della conferma (14-editor.prototype.html:299, 377) */
    neutro?: boolean;
    /** aperta: legabile, per accendere insieme più conferme della stessa cosa; non passata, la tiene il componente */
    aperta?: boolean;
    onconferma: () => void;
  };

  let { etichetta, conferma, annulla = t("componenti.annulla"), spiegazione, forma = "bordo", neutro = false, aperta = $bindable(false), onconferma }: Props =
    $props();
  const id = $props.id();
  let radice: HTMLSpanElement;

  async function apri(stato: boolean) {
    aperta = stato;
    await tick();
    // chi conferma può smontare il componente: allora il fuoco lo decide lui
    radice?.querySelector("button")?.focus();
  }

  function tasto(e: KeyboardEvent) {
    if (aperta && e.key === "Escape") {
      // un Esc annulla la conferma e basta: il pannello o l'editor che la contiene resta aperto
      e.preventDefault();
      e.stopPropagation();
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
    <Pulsante {forma} distruttivo={!neutro} onclick={() => apri(true)}>{etichetta}</Pulsante>
  {/if}
</span>

<style>
  .conferma { display: inline-flex; align-items: center; gap: 6px; flex-wrap: wrap }
  .spieg { font-size: var(--c-min); color: var(--grigio) }
</style>
