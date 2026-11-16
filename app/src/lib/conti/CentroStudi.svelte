<script lang="ts">
  // Il centro studi nel lato (storia 122, prototipo 21): prima dell'acquisto cosa vende, il prezzo e la cassa; dopo, la stima accanto alla prima stima e i quattro ventagli.
  import { tick } from "svelte";
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import { periodo } from "$lib/edicola/edicola";
  import { euro } from "$lib/governo/governo";
  import { acquisto } from "$lib/acquisto.svelte";
  import { partita } from "$lib/partita.svelte";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Ventaglio from "./Ventaglio.svelte";

  let { vista }: { vista: Vista } = $props();

  const PREV = ["pil", "disoccupazione", "inflazione", "debito"] as const;
  const cs = $derived(vista.acquisti.centro_studi);
  const prezzo = $derived(vista.acquisti.prezzi.find((p) => p.acquisto === "centro_studi")?.euro);
  const cassa = $derived(vista.partiti.find((p) => p.id === vista.giocatore)?.cassa ?? 0);
  const prima = $derived(vista.statistica.prima_stima);
  const compra = acquisto("centro_studi");
  const occupato = $derived(partita.occupato ? "true" : undefined);
  let box = $state<HTMLElement>();
  // «Compra» sparisce con l'acquisto: il fuoco passa a «Compralo ancora» invece di cadere sul body
  async function compraQui() {
    await compra.compra();
    await tick();
    if (!box?.contains(document.activeElement)) box?.querySelector("button")?.focus();
  }
</script>

{#snippet forte(testo: string, vals: string[])}{#each testo.split("{b}") as pezzo, i (i)}{pezzo}{#if i < vals.length}<b>{vals[i]}</b>{/if}{/each}{/snippet}

<div class="studi" bind:this={box}>
  {#if cs}
    <div class="capo"><h4 class="tit">{t("conti.cs_titolo")}</h4><span class="sec muto">{t("conti.cs_comprato", { euro: euro(prezzo ?? 0) })}</span></div>
    <p class="sec">
      {@render forte(
        t("conti.cs_stima", { periodo: periodo(vista.statistica.trimestre), pil: segnato(prima.crescita_pil, 1), inflazione: numero(prima.inflazione_annua, 1), disoccupazione: numero(prima.disoccupazione, 1) }),
        [`${segnato(cs.stima.crescita_pil, 1)}%`, `${numero(cs.stima.inflazione_annua, 1)}%`, `${numero(cs.stima.disoccupazione, 1)}%`],
      )}
    </p>
    <div class="fan">
      {#each PREV as k (k)}
        {@const nota = t(`conti.prev_nota_${k}` as Chiave)}
        {@const f = cs.previsione.at(-1)?.[k]}
        <div>
          <p class="min muto"><b class="ink">{t(`conti.prev_${k}` as Chiave)}</b>{nota ? ` · ${nota}` : ""}</p>
          <Ventaglio previsione={cs.previsione} {k} />
          {#if f}
            {@const fine = cs.previsione[cs.previsione.length - 1].data}
            <p class="min">{@render forte(t("conti.prev_fine", { periodo: t(`conti.trim_${fine.trimestre}` as Chiave, { anno: fine.anno }), sigma: numero(f.sigma, 1) }), [`${numero(f.centro, 1)}%`])}</p>
          {/if}
        </div>
      {/each}
    </div>
    <p class="min muto">{t("conti.cs_fasce")}</p>
    <span><Pulsante forma="link" aria-disabled={occupato} onclick={compra.compra}>{t("conti.cs_ricompra")}</Pulsante></span>
  {:else}
    <h4 class="tit">{t("conti.cs_titolo")}</h4>
    <p class="sec">{t("conti.cs_vende")}</p>
    {#if prezzo !== undefined}
      <div class="prezzo">
        <Pulsante forma="bordo" aria-disabled={occupato} onclick={compraQui}>{t("conti.cs_compra", { euro: euro(prezzo) })}</Pulsante>
        <span class="sec muto">{t("conti.cs_cassa", { euro: `${numero(cassa)} €` })}</span>
      </div>
    {/if}
  {/if}
  {#if compra.rifiuto}<Rifiuto testo={compra.rifiuto} />{/if}
</div>

<style>
  .studi { border: var(--forte); padding: 10px 12px; display: grid; gap: 8px }
  .studi .tit { font-size: var(--c-riq) }
  .capo { margin: 0 }
  .prezzo { display: flex; align-items: center; gap: 10px; flex-wrap: wrap }
  .fan { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 8px 14px }
  .ink { color: var(--ink) }
</style>
