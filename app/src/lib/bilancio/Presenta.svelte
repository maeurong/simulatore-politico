<script lang="ts">
  // La presentazione nel lato, senza riga scelta (prototipo 22): «Presenta» diretto, sotto cosa succede se cade, poi il rifiuto del core.
  // Presentata: il riquadro nero con la forbice di Camera e Senato e «Modifica e ripresenta ›», che riapre la bozza dal bilancio in aula.
  import { tick } from "svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { partita } from "$lib/partita.svelte";
  import { t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import { seCade } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { vista, bilancio, misura, ferma }: { vista: Vista; bilancio: Bilancio; misura: VoceMisura | undefined; ferma: boolean } = $props();

  // nella presentata «Presenta» non c'è: un secondo Invio dopo la presentazione non trova il bottone
  const spento = $derived(
    partita.occupato ? t("campagna.comando_in_corso")
    : bozza.inVolo ? t("bilancio.presenta_in_volo")
    : bozza.rifiuto ? t(bozza.guasto ? "bilancio.presenta_guasto" : "bilancio.presenta_non_ammessa")
    : undefined,
  );
  let box = $state<HTMLElement>();

  async function ripresenta() {
    bozza.riapri(bilancio);
    await tick();
    box?.querySelector("button")?.focus();
  }
</script>

<div class="presenta" bind:this={box}>
  {#if ferma}
    {#if misura}
      {@const f = misura.forbice}
      <div class="esito">
        <b>{t("bilancio.presentata")}</b>
        <span>{t("bilancio.forbice_camera", { da: f.camera[0], a: f.camera[1], n: f.maggioranza_camera })}</span>
        <span>{t("bilancio.forbice_senato", { da: f.senato[0], a: f.senato[1], n: f.maggioranza_senato })}</span>
      </div>
    {/if}
    <p class="min muto">{t("bilancio.ripresenta_nota")}</p>
    <Pulsante forma="link" onclick={ripresenta}>{t("bilancio.ripresenta")}</Pulsante>
  {:else}
    <Pulsante forma="chiude" {spento} onclick={() => bozza.presenta((b) => partita.comanda({ tipo: "presenta_bilancio", bilancio: b }))}>{t("bilancio.presenta")}</Pulsante>
    <p class="min muto">{seCade(vista)}</p>
    {#if bozza.rifiutoPresenta}<Rifiuto testo={bozza.rifiutoPresenta} />{/if}
  {/if}
</div>

<style>
  .presenta { display: grid; gap: 6px; justify-items: start }
  /* il riquadro nero del prototipo 22 (.esito) */
  .esito { justify-self: stretch; background: var(--ink); color: var(--carta); padding: 6px 10px; font-size: var(--c-sec); display: grid; gap: 2px }
  .esito b { font-size: var(--c-lead) }
</style>
