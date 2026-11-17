<script lang="ts">
  // Il dettaglio della riga scelta, nel lato (storia 127; prototipo 22 e 36-scelte-11d-bilancio?marg=A): dicastero col margine a passi, scaglione, fondo.
  import { tick } from "svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Forte from "$lib/conti/Forte.svelte";
  import { reale } from "$lib/conti/conti";
  import { euro } from "$lib/governo/governo";
  import { nomeDi } from "$lib/partiti";
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import { daIndicizzare, euroSegnato, passoMargine } from "./bilancio";
  import { bozza, type Via } from "./bozza.svelte";

  // ferma: la presentata, il dettaglio si legge e basta (niente Indicizza né margine)
  let { vista, bilancio, riepilogo, ferma, scelta, torna }: {
    vista: Vista; bilancio: Bilancio; riepilogo: RiepilogoBilancio | null; ferma: boolean; scelta: string; torna: () => void;
  } = $props();

  const [tipo, id] = $derived(scelta.split(":"));
  const vecchio = $derived(vista.bilancio.anno);
  const perdita = (voce: string) => riepilogo?.perdita_reale.find((p) => p.voce === voce)?.perdita;
  let capo = $state<HTMLElement>();
  // «Indicizza» e «annulla» si scambiano il posto: se il fuoco cade col bottone premuto, va a «Torna»
  async function eTorna(f: () => unknown) {
    await f();
    await tick();
    if (document.activeElement === document.body) capo?.querySelector("button")?.focus();
  }
</script>

{#snippet cifra(x: number, z: number | undefined)}
  <b>{euro(x)}</b>{#if z !== undefined && z !== x}&nbsp;<span class="min muto">{t("bilancio.bozza_era", { euro: euro(z) })}</span>{/if}
{/snippet}

{#snippet realeDi(p: number | undefined)}
  {@const r = p === undefined ? null : reale(p)}
  {#if r}<span class={r.tono}>{r.testo}</span>{:else}<span class="muto">—</span>{/if}
{/snippet}

<div class="capo-lato" bind:this={capo}><Pulsante forma="link" onclick={torna}>{t("bilancio.torna")}</Pulsante></div>

{#if tipo === "dic"}
  {@const d = bilancio.dicasteri.find((x) => x.id === id)!}
  {@const z = bozza.base?.dicasteri.find((x) => x.id === id)}
  {@const nome = vista.dicasteri.find((x) => x.id === id)?.nome ?? id}
  {@const m = vista.ministeri.find((x) => x.dicastero === id)?.ministro}
  {@const p = perdita(id)}
  {@const voce = { tipo: "dicastero" as const, id }}
  {@const rif = bozza.rifiutoDi(voce)}
  <h4 class="tit">{nome}</h4>
  <span class="sec muto">{m ? `${m.nome} · ${nomeDi(vista, m.partito)}` : t("governo.senza_ministro")}</span>
  <dl class="dati">
    <dt>{t("bilancio.spesa_annua")}</dt><dd>{@render cifra(d.spesa, z?.spesa)}</dd>
    <dt>{t("bilancio.funzionamento")}</dt><dd>{t("bilancio.funz_cifre", { salari: euro(d.salari), acquisti: euro(d.acquisti) })}</dd>
    <dt>{t("bilancio.margine")}</dt><dd>{@render cifra(d.margine, z?.margine)}</dd>
    <dt>{t("bilancio.resto")}</dt><dd>{euro(d.spesa - d.salari - d.acquisti - d.margine)}</dd>
    <dt>{t("bilancio.reale")}</dt><dd>{@render realeDi(p)} {t("bilancio.dal_bilancio", { anno: vecchio })}</dd>
  </dl>
  {#if p !== undefined && daIndicizzare(p)}
    <p class="sec"><Forte testo={t("conti.dic_reale", { anno: vecchio })} vals={[t("conti.dic_perdita", { nome, perdita: reale(p).testo })]} /></p>
  {/if}
  {#if !ferma && bozza.indicizzato(voce)}
    <span class="riga"><span class="min bene">{t("bilancio.indicizzato")}</span>{#if bozza.accolto(id)}<span class="min muto">{t("bilancio.accolto_riapplica")}</span>{:else}<button type="button" class="link" onclick={() => eTorna(() => bozza.annulla(voce))}>{t("bilancio.annulla")}</button>{/if}</span>
  {:else if !ferma && p !== undefined && daIndicizzare(p)}
    {@const c = bozza.costi[id]}
    <span class="riga"><button type="button" class="link" onclick={() => eTorna(() => bozza.indicizza(voce))}>{t("bilancio.indicizza")}</button>
      <span class="min muto">{#if typeof c === "object"}{t("bilancio.costo_bozza", { c: euroSegnato(c.costo_spesa) })}{:else if c}{c}{/if}</span></span>
  {/if}
  {#if rif && !ferma}<Rifiuto testo={rif} />{/if}

  {#if z && !ferma}
    {@const passo = passoMargine(z)}
    {@const n = bozza.passi[id] ?? { funzionamento: 0, disavanzo: 0 }}
    <div class="grp"><span>{t("bilancio.allarga")}</span><span class="tondo">{t("bilancio.passo", { euro: euro(passo) })}</span></div>
    {#snippet via(v: Via, nomeVia: string)}
      <div class="via">
        <span>{nomeVia}</span>
        <span class="step">
          <button type="button" aria-label={t("bilancio.passo_meno", { via: nomeVia })} aria-disabled={n[v] ? undefined : "true"} onclick={() => bozza.allarga(id, v, -1)}>−</button>
          <span>{n[v] ? `+${euro(passo * n[v])}` : t("bilancio.passi_bozza")}</span>
          <button type="button" aria-label={t("bilancio.passo_piu", { via: nomeVia })} onclick={() => bozza.allarga(id, v, 1)}>+</button>
        </span>
      </div>
    {/snippet}
    {@render via("funzionamento", t("bilancio.via_funzionamento"))}
    {@render via("disavanzo", t("bilancio.via_disavanzo"))}
    <p class="min muto">{t("bilancio.nota_allarga")}</p>
  {/if}
{:else if tipo === "irpef"}
  {@const i = +id}
  {@const s = bilancio.aliquote.irpef[i]}
  {@const p = perdita(`scaglione ${i + 1}`)}
  <h4 class="tit">{t("bilancio.scaglione_titolo", { n: i + 1 })}</h4>
  <dl class="dati">
    <dt>{t("bilancio.aliquota")}</dt><dd><b>{percentuale(s.aliquota, 0)}</b></dd>
    <dt>{t("bilancio.soglia")}</dt><dd>{s.fino_a === null ? t("bilancio.soglia_ultimo") : t("bilancio.soglia_euro", { n: numero(s.fino_a) })}</dd>
    {#if s.fino_a !== null}<dt>{t("bilancio.soglia_reale")}</dt><dd>{@render realeDi(p)} {t("bilancio.dal_anno", { anno: vecchio })}</dd>{/if}
  </dl>
  {#if s.fino_a !== null && p !== undefined && daIndicizzare(p)}<p class="sec">{t("bilancio.soglia_ferma", { n: numero(s.fino_a) })}</p>{/if}
{:else if scelta === "fondo"}
  <h4 class="tit">{t("bilancio.fondo")}</h4>
  <dl class="dati">
    <dt>{bilancio.anno}</dt><dd><b>{euro(bilancio.fondo_riserva)}</b></dd>
    <dt>{t("bilancio.rimasto", { anno: vecchio })}</dt><dd>{euro(vista.bilancio.fondo_residuo)}</dd>
  </dl>
  <p class="sec">{t("bilancio.fondo_spiega")}</p>
  {#if vista.eventi.length}
    <h5 class="grp"><span>{t("conti.eventi_aperti")}</span></h5>
    {#each vista.eventi as e, i (i)}<div class="ev"><span>{e.nome}</span><b>{euro(e.costo)}</b></div>{/each}
  {/if}
{/if}

<style>
  .capo-lato { display: flex }
  .dati { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 3px 12px; font-size: var(--c-sec); align-items: baseline }
  .dati dt { color: var(--grigio) }
  .riga { display: flex; gap: 8px; align-items: baseline }
  .via { display: flex; gap: 8px; align-items: center; font-size: var(--c-sec) }
  .via > span:first-child { min-width: 120px }
  /* lo stepper del prototipo 36 (marg=A): − e + attorno ai passi aggiunti */
  .step { display: inline-flex; align-items: center; border: var(--bordo) }
  .step button { width: 22px; text-align: center; font-weight: 700; font-size: var(--c-lead) }
  .step button:not([aria-disabled="true"]):hover { background: var(--velo) }
  .step button[aria-disabled="true"] { color: var(--grigio); cursor: default }
  .step span { padding: 0 6px; font-weight: 700; border-inline: 1px solid var(--ink); white-space: nowrap }
  /* la cifra in maiuscoletto si leggerebbe «MLN» (Spesa: .tutto) */
  .tondo { letter-spacing: normal; text-transform: none; font-weight: 400 }
  .ev { display: flex; justify-content: space-between; gap: 8px; padding: 3px 0; border-bottom: var(--filo); font-size: var(--c-sec) }
</style>
