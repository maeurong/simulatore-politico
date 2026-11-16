<script lang="ts">
  // Le entrate a sinistra (storia 125): aliquote grandi con − e +, la soglia reale degli scaglioni con «indicizza le soglie», il fondo di riserva.
  import { euro } from "$lib/governo/governo";
  import { reale } from "$lib/conti/conti";
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import { euroSegnato, ritocca, type Imposta } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { bilancio }: { bilancio: Bilancio } = $props();

  const perdita = (voce: string) => bozza.riepilogo?.perdita_reale.find((p) => p.voce === voce)?.perdita;
  const costoSoglie = $derived(bozza.costi.scaglioni);
  const passo = (imposta: Imposta, verso: 1 | -1) => bozza.cambia(ritocca(bilancio, imposta, verso));
  const altre = ["iva", "ires", "contributi"] as const;
</script>

{#snippet stepper(imposta: Imposta, nome: string, aliquota: number)}
  <span class="step">
    <button type="button" aria-label={t("bilancio.meno_punto", { imposta: nome })} onclick={() => passo(imposta, -1)}>−</button>
    <span>{percentuale(aliquota, 0)}</span>
    <button type="button" aria-label={t("bilancio.piu_punto", { imposta: nome })} onclick={() => passo(imposta, 1)}>+</button>
  </span>
{/snippet}

<section>
  <div class="grp"><span>{t("bilancio.entrate")}</span><span>{t("bilancio.aliquote_anno", { anno: bilancio.anno })}</span></div>
  {#each bilancio.aliquote.irpef as s, i (i)}
    {@const nome = s.fino_a === null ? t("bilancio.irpef_oltre") : t("bilancio.irpef_fino", { n: numero(s.fino_a / 1000) })}
    <div class="imp">
      <span>{nome}</span>
      <span class="v">{@render stepper({ tipo: "irpef", indice: i }, nome, s.aliquota)}</span>
      {#if s.fino_a !== null}
        {@const p = perdita(`scaglione ${i + 1}`)}
        {@const r = p === undefined ? null : reale(p)}
        <span class="d">
          {t("bilancio.soglia")} {#if r}<span class={r.tono}>{r.testo}</span>{:else}<span class="muto">—</span>{/if} {t("bilancio.reale")}
          {#if i === 0}
            · {#if bozza.indicizzato({ tipo: "scaglioni" })}<span class="bene">{t("bilancio.soglie_indicizzate")}</span>
            {:else}<button type="button" class="link" onclick={() => bozza.indicizza({ tipo: "scaglioni" })}>{t("bilancio.indicizza_soglie")}</button>
              {#if typeof costoSoglie === "object"}{euroSegnato(-costoSoglie.costo_entrate)}{:else if costoSoglie}<span class="muto">{costoSoglie}</span>{/if}{/if}
          {/if}
        </span>
      {/if}
    </div>
  {/each}
  {#each altre as k (k)}
    <div class="imp"><span>{t(`bilancio.${k}`)}</span><span class="v">{@render stepper({ tipo: k }, t(`bilancio.${k}`), bilancio.aliquote[k])}</span></div>
  {/each}
  <div class="imp"><span>{t("bilancio.fondo")}</span><span class="v fondo">{euro(bilancio.fondo_riserva)}</span></div>
</section>

<style>
  .imp { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; border-top: var(--bordo); padding: 4px 0 }
  .imp .v { font-family: var(--tit); font-variation-settings: var(--wonk); font-size: var(--c-sez); font-weight: 800 }
  .imp .v.fondo { font-size: var(--c-lead) }
  .imp .d { grid-column: 1 / -1; font-size: var(--c-min); color: var(--grigio) }
  .d .link { font-size: var(--c-min) }
  .step { display: inline-flex; align-items: center; border: var(--bordo) }
  .step button { width: 22px; text-align: center; font-weight: 700; font-family: var(--txt); font-size: var(--c-lead) }
  .step button:hover { background: var(--velo) }
  .step span { padding: 0 6px; border-inline: 1px solid var(--ink) }
</style>
