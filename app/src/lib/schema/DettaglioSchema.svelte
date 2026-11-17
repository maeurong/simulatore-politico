<script lang="ts">
  // Il dettaglio a lato dello schema (29-schema.prototype.html, forma B): il filo o la riga scelti, poi «Come si legge» col numero dei fili.
  // Il campo `fonte` del filo è tecnico: al giocatore non si mostra.
  import { t } from "$lib/testi/t";
  import { apribile, etichetta } from "$lib/ossatura";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import type { Filo, Nodo } from "./schema";
  import { nomeNodo, type Scelta } from "./tavola";

  let { vista, nodi, fili, scelta, id, onfilo, onchiudi }: {
    vista: Vista; nodi: Nodo[]; fili: Filo[]; scelta: Scelta; id: string; onfilo: (id: string) => void; onchiudi: () => void;
  } = $props();

  const nodo = (x: string) => nodi.find((n) => n.id === x)!;
  const filo = $derived(scelta?.tipo === "filo" ? fili.find((f) => f.id === scelta.id) : undefined);
  const riga = $derived(scelta?.tipo === "riga" ? nodi.find((n) => n.id === scelta.id) : undefined);
  const suoi = $derived(riga ? fili.filter((f) => f.da === riga.id || f.a === riga.id) : []);
</script>

{#if filo}
  <section class="det" {id} tabindex="-1" aria-label={filo.frase}>
    <div class="occ">{nomeNodo(nodo(filo.da))} → {nomeNodo(nodo(filo.a))}</div>
    <p class="cosa">{filo.frase}</p>
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
          <li><button type="button" onclick={() => onfilo(f.id)} aria-label={f.frase}>
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

<h4 class="tit">{t("schema.come_si_legge")}</h4>
<p class="sec">{t("schema.n_fili", { n: fili.length })}</p>
<div class="leg" aria-hidden="true">
  <svg viewBox="0 0 34 14"><path class="fi" d="M2 7 H32" /></svg><span>{t("schema.leg_filo")}</span>
  <svg viewBox="0 0 34 14"><path class="fi m" d="M2 7 H32" /></svg><span>{t("schema.leg_contrario")}</span>
  <svg viewBox="0 0 34 14"><path class="fi r" d="M2 7 H32" /></svg><span>{t("schema.leg_ritardo")}</span>
  <svg viewBox="0 0 34 14"><path class="fi c" d="M2 7 H32" stroke-width="1.6" /></svg><span>{t("schema.leg_cantiere")}</span>
  <svg viewBox="0 0 34 14"><path class="fi" d="M2 7 H26" /><path class="punta" d="M24,3 L32,7 L24,11 z" /></svg><span>{t("schema.leg_punta")}</span>
  <svg viewBox="0 0 34 14"><path class="fi" d="M8 1 V13 M8 7 H32" stroke-width="2.5" /></svg><span>{t("schema.leg_sbarra")}</span>
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
  .fi { fill: none; stroke: var(--biro); stroke-width: 2.2 }
  .fi.m { stroke: var(--rosso) }
  .fi.r { stroke-dasharray: 7 5 }
  .fi.c { stroke-dasharray: 2 4; stroke-linecap: round }
  .punta { fill: var(--biro) }
</style>
