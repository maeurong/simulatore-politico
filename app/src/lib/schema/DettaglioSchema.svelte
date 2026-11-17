<script lang="ts">
  // Il dettaglio a lato dello schema (29-schema.prototype.html, forma B): il filo o la riga scelti, «Le notizie del trimestre», poi «Come si legge» col numero dei fili.
  // Il campo `fonte` del filo è tecnico: al giocatore non si mostra. La battuta a biro solo qui, sul filo col tondo, e tace nel tono grave (decisione 2).
  import { t } from "$lib/testi/t";
  import { apribile, etichetta } from "$lib/ossatura";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { grave } from "$lib/esito/esito";
  import { battutaDi } from "$lib/testi/pungolo";
  import type { Filo, Nodo, tondiDi } from "./schema";
  import { nomeNodo, type Scelta } from "./tavola";

  let { vista, nodi, fili, tondi, scelta, id, onapri, onchiudi }: {
    vista: Vista; nodi: Nodo[]; fili: Filo[]; tondi: ReturnType<typeof tondiDi>; scelta: Scelta; id: string; onapri: (s: Scelta) => void; onchiudi: () => void;
  } = $props();

  const nodo = (x: string) => nodi.find((n) => n.id === x)!;
  const filo = $derived(scelta?.tipo === "filo" ? fili.find((f) => f.id === scelta.id) : undefined);
  const riga = $derived(scelta?.tipo === "riga" ? nodi.find((n) => n.id === scelta.id) : undefined);
  const suoi = $derived(riga ? fili.filter((f) => f.da === riga.id || f.a === riga.id) : []);
  const numeri = $derived(filo ? (tondi.fili.get(filo.id) ?? []) : []);
  const battuta = $derived(filo && numeri.length && !grave(vista) ? battutaDi(filo.tipo) : null);
  const haTondi = $derived(tondi.fili.size + tondi.nodi.size > 0);
</script>

{#if filo}
  <section class="det" {id} tabindex="-1" aria-label={filo.frase}>
    <div class="occ">{nomeNodo(nodo(filo.da))} → {nomeNodo(nodo(filo.a))}</div>
    <p class="cosa">{filo.frase}</p>
    {#if numeri.length}
      <p class="sec">{t("schema.in_prima_pagina")} {#each numeri as k (k)}<span class={["tn", { m: filo.contrario }]}>{k}</span> {/each}</p>
    {/if}
    {#if battuta}<p class="battuta">{t("schema.battuta", { battuta })}</p>{/if}
    <div class="az">
      {#if apribile(filo.sezione, vista)}<a class="rimando" href="/partita/{filo.sezione}">{etichetta(filo.sezione)} ›</a>{/if}
      <Pulsante forma="link" onclick={onchiudi}>{t("schema.chiudi")}</Pulsante>
    </div>
  </section>
{:else if riga}
  <section class="det" {id} tabindex="-1" aria-label={nomeNodo(riga)}>
    <div class="occ">{riga.sotto}</div>
    <h4 class="tit">{nomeNodo(riga)}</h4>
    {#if suoi.length}
      <ul class="fl">
        {#each suoi as f (f.id)}
          <li><button type="button" onclick={() => onapri({ tipo: "filo", id: f.id })}>
            <span class="ver" class:male={f.contrario}>{f.da === riga.id ? "→" : "←"} {nomeNodo(nodo(f.da === riga.id ? f.a : f.da))}</span>
            <span>{f.etichetta}</span>
          </button></li>
        {/each}
      </ul>
    {:else}
      <p class="sec muto">{t("schema.nessun_filo")}</p>
    {/if}
    <div class="az"><Pulsante forma="link" onclick={onchiudi}>{t("schema.chiudi")}</Pulsante></div>
  </section>
{/if}

{#if tondi.elenco.length}
  <h4 class="tit">{t("schema.notizie")}</h4>
  <ol class="voci">
    {#each tondi.elenco as { n, voce, posto } (n)}
      <li>
        {#if posto}
          {@const f = fili.find((x) => x.id === posto)}
          <!-- il tondo ha il colore del suo filo, come sulla tavola -->
          <button type="button" onclick={() => onapri({ tipo: f ? "filo" : "riga", id: posto })}>
            <span class={["tn", { m: f?.contrario }]}>{n}</span><span>{voce.fatto}</span>
          </button>
        {:else}
          <span class="no"><span class="tn">{n}</span><span>{voce.fatto} <span class="muto">· {t("schema.senza_posto")}</span></span></span>
        {/if}
      </li>
    {/each}
  </ol>
{/if}

<h4 class="tit">{t("schema.come_si_legge")}</h4>
<p class="sec">{t("schema.n_fili", { n: fili.length })}</p>
<div class="leg" aria-hidden="true">
  <svg viewBox="0 0 34 14"><path class="fi" d="M2 7 H32" /></svg><span>{t("schema.leg_filo")}</span>
  <svg viewBox="0 0 34 14"><path class="fi m" d="M2 7 H32" /></svg><span>{t("schema.leg_contrario")}</span>
  <svg viewBox="0 0 34 14"><path class="fi r" d="M2 7 H32" /></svg><span>{t("schema.leg_ritardo")}</span>
  <svg viewBox="0 0 34 14"><path class="fi c" d="M2 7 H32" /></svg><span>{t("schema.leg_cantiere")}</span>
  <svg viewBox="0 0 34 14"><path class="fi" d="M2 7 H26" /><path class="punta" d="M24,3 L32,7 L24,11 z" /></svg><span>{t("schema.leg_punta")}</span>
  <svg viewBox="0 0 34 14"><path class="fi sb" d="M8 1 V13 M8 7 H32" /></svg><span>{t("schema.leg_sbarra")}</span>
  {#if haTondi}<span class="tn"></span><span>{t("schema.leg_tondo")}</span>{/if}
</div>

<style>
  .det { display: grid; gap: 6px; padding-bottom: 10px; border-bottom: var(--filo); outline: none }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .cosa { font-size: var(--c-lead); font-weight: 700; line-height: 1.25 }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .az { display: flex; gap: 14px; align-items: baseline }
  .rimando { font-weight: 700; text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec) }
  .fl { list-style: none; display: grid }
  .fl button { display: grid; grid-template-columns: auto 1fr; gap: 6px; width: 100%; text-align: left; padding: 4px 0; border-top: var(--filo); font-size: var(--c-sec) }
  .fl button:hover { background: var(--velo) }
  .ver { font-weight: 700; white-space: nowrap }
  .male { color: var(--rosso) }
  .leg { display: grid; grid-template-columns: 34px 1fr; gap: 3px 8px; font-size: var(--c-min); align-items: center }
  .leg svg { width: 34px; height: 14px }
  /* gli spessori della tavola: filo 2, cantiere 1,5, sbarra 3 */
  .fi { fill: none; stroke: var(--biro); stroke-width: 2 }
  .fi.m { stroke: var(--rosso) }
  .fi.r { stroke-dasharray: 7 5 }
  .fi.c { stroke-dasharray: 2 4; stroke-linecap: round; stroke-width: 1.5 }
  .fi.sb { stroke-width: 3 }
  .punta { fill: var(--biro) }
  .tn { display: inline-grid; place-items: center; width: 20px; height: 20px; border-radius: 50%; border: 1.5px solid var(--biro); color: var(--biro); font-size: var(--c-min); font-weight: 700; background: var(--carta); vertical-align: middle }
  .tn.m { border-color: var(--rosso); color: var(--rosso) }
  .leg .tn { justify-self: center; width: 14px; height: 14px }
  .battuta { font-size: var(--c-sec); color: var(--biro) }
  .voci { list-style: none; display: grid; gap: 4px; font-size: var(--c-sec) }
  .voci button, .voci .no { display: grid; grid-template-columns: 20px 1fr; gap: 6px; align-items: start; width: 100%; text-align: left }
  .voci button:hover { background: var(--velo) }
  .voci .no .tn { border-style: dashed; border-color: var(--grigio); color: var(--grigio) }
</style>
