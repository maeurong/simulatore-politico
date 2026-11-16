<script lang="ts">
  // La spesa a destra (storia 125): i dieci dicasteri come barre (salari, acquisti, margine, resto), la spesa, la perdita reale e «Indicizza» col costo.
  import { euro } from "$lib/governo/governo";
  import { reale } from "$lib/conti/conti";
  import { numero, t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import { euroSegnato } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { vista, bilancio }: { vista: Vista; bilancio: Bilancio } = $props();

  const nome = (id: string) => vista.dicasteri.find((d) => d.id === id)?.nome ?? id;
  const mx = $derived(Math.max(...bilancio.dicasteri.map((d) => d.spesa)));
  const w = (x: number) => `${(x / mx) * 100}%`;
  const tutto = $derived(bozza.costi.tutto);
</script>

{#snippet costo(c: RiepilogoBilancio | string | undefined)}
  {#if typeof c === "object"}{euroSegnato(c.costo_spesa)}{:else if c}<span class="muto">{c}</span>{/if}
{/snippet}

<section>
  <div class="grp">
    <span>{t("bilancio.spesa_dicasteri")}</span>
    {#if bozza.indicizzato({ tipo: "tutto" })}<span class="bene">{t("bilancio.indicizzato_tutto")}</span>
    {:else}<span class="tutto"><button type="button" class="link" onclick={() => bozza.indicizza({ tipo: "tutto" })}>{t("bilancio.indicizza_tutto")}</button> {@render costo(tutto)}</span>{/if}
  </div>
  <div class="leg">
    <span><i style:background="var(--ink)"></i>{t("bilancio.salari")}</span><span><i style:background="var(--grigio)"></i>{t("bilancio.acquisti")}</span>
    <span><i style:background="var(--biro)"></i>{t("bilancio.margine")}</span><span><i class="ret"></i>{t("bilancio.resto")}</span>
    <span>{t("bilancio.reale_dal", { anno: vista.bilancio.anno })}</span>
  </div>
  {#each bilancio.dicasteri as d (d.id)}
    {@const resto = d.spesa - d.salari - d.acquisti - d.margine}
    {@const p = bozza.riepilogo?.perdita_reale.find((x) => x.voce === d.id)?.perdita}
    {@const r = p === undefined ? null : reale(p)}
    {@const voce = { tipo: "dicastero" as const, id: d.id }}
    <div class="barD">
      <span class="n">{nome(d.id)}</span>
      <!-- il margine ha un minimo visibile: anche piccolo resta il pezzo che si allarga -->
      <span class="sp" title={t("bilancio.barra", { salari: euro(d.salari), acquisti: euro(d.acquisti), margine: euro(d.margine), resto: euro(resto) })}>
        <i style:width={w(d.salari)} style:background="var(--ink)"></i><i style:width={w(d.acquisti)} style:background="var(--grigio)"></i><i
          style:width={w(Math.max(d.margine, mx * 0.004))} style:background="var(--biro)"></i><i class="ret" style:width={w(Math.max(resto, 0))}></i>
      </span>
      <span class="x">{t("conti.miliardi", { n: numero(d.spesa / 1e9, 1) })}</span>
      <span class="az">
        {#if r}<span class={r.tono}>{r.testo}</span>{:else}<span class="muto">—</span>{/if}
        {#if bozza.indicizzato(voce)}<span class="min bene">{t("bilancio.indicizzato")}</span>
        {:else}<span class="ind"><button type="button" class="link" onclick={() => bozza.indicizza(voce)}>{t("bilancio.indicizza")}</button> <span class="min muto">{@render costo(bozza.costi[d.id])}</span></span>{/if}
      </span>
    </div>
  {/each}
</section>

<style>
  .leg { display: flex; flex-wrap: wrap; gap: 4px 12px; font-size: var(--c-min); color: var(--grigio) }
  .leg i { display: inline-block; width: 10px; height: 10px; margin-right: 4px; vertical-align: -1px }
  .ret { background: var(--retino); outline: 1px solid var(--grigio); outline-offset: -1px }
  .barD { display: grid; grid-template-columns: 150px minmax(0, 1fr) 64px 184px; gap: 2px 8px; align-items: center; padding: 3px 2px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .n { white-space: nowrap; overflow: hidden; text-overflow: ellipsis }
  .sp { height: 12px; display: flex }
  .sp i { height: 100%; border-right: 1px solid var(--carta) }
  .x { text-align: right; font-weight: 700; white-space: nowrap }
  .az { display: grid; grid-template-columns: 46px auto; gap: 6px; justify-content: start; align-items: baseline; white-space: nowrap }
  .ind { display: inline-flex; gap: 6px; align-items: baseline }
  /* la cifra in maiuscoletto si leggerebbe «MLD»: il comando e il costo restano in tondo */
  .tutto { letter-spacing: normal; text-transform: none; font-weight: 400; color: var(--ink) }
  .tutto .link { font-size: var(--c-min); font-weight: 700 }
</style>
