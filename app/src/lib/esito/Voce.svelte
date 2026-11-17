<script lang="ts">
  // Una voce della prima pagina (16-prima-pagina.prototype.html: occhiello, titolo, causa, righe fuse, rimando).
  // In attesa occupa il suo posto e non si vede: la rivelazione non sposta niente.
  import { apribile, casa, etichetta } from "$lib/ossatura";
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceEsito } from "$lib/tipi/VoceEsito";
  import { eventoAperto, SEZIONE_DI } from "./esito";

  type Props = { vista: Vista; voce: VoceEsito; titolo: string | null; testa?: boolean; rivelata: boolean; nuova: boolean };
  let { vista, voce, titolo, testa = false, rivelata, nuova }: Props = $props();

  const critica = $derived(voce.criticita === "critica");
  const sezione = $derived(SEZIONE_DI[voce.tipo]);
  const aperto = $derived(eventoAperto(vista, voce));
  // al governo e in trattativa l'evento si decide qui, all'opposizione lo decide il governo
  const decido = $derived(vista.governa || vista.fase === "trattativa");
</script>

<article class={["voce", { testa, attesa: !rivelata, nuova: rivelata && nuova }]}>
  {#if titolo}
    <div class={["occh", { crit: critica }]}>{#if critica}<span class="nascosto">{t("prima.critica")}: </span>{/if}{voce.fatto}</div>
    <h2 class="tit">{titolo}</h2>
  {:else}
    <h2 class={["tit", { crit: critica }]}>{#if critica}<span class="nascosto">{t("prima.critica")}: </span>{/if}{voce.fatto}</h2>
  {/if}
  <p class="causa"><i>{t("prima.perche")}</i> {voce.causa}</p>
  {#if voce.dettagli.length}
    <ul class="det">{#each voce.dettagli as d, i (i)}<li>{d.testo}</li>{/each}</ul>
  {/if}
  <div>
    {#if aperto}
      {#if decido}<a class="aperto" href="/partita/{casa(vista)}">{t("prima.si_decide_sul_cruscotto")}</a>{:else}<span class="aperto">{t("prima.lo_decide_il_governo")}</span>{/if}
    {:else if apribile(sezione, vista)}
      <a class="rim" href="/partita/{sezione}">{t("prima.rimando", { sezione: etichetta(sezione) })}</a>
    {/if}
  </div>
</article>

<style>
  .voce { display: grid; gap: 3px; align-content: start; min-width: 0 }
  .attesa { visibility: hidden }
  .nuova { animation: entra calc(520ms * var(--k)) var(--e-work) both }
  @keyframes entra { from { opacity: 0 } to { opacity: 1 } }
  /* col movimento acceso la voce sale anche di qualche pixel: spento resta la sola dissolvenza (spec, storia 55) */
  :global(:root:not([data-movimento="spento"], .grave)) .nuova { animation-name: sale }
  @keyframes sale { from { opacity: 0; transform: translateY(10px) } to { opacity: 1; transform: none } }

  .occh { font-size: var(--c-sec); font-weight: 700; line-height: 1.3 }
  .occh.crit { color: var(--rosso) }
  .occh.crit::before { content: ""; display: inline-block; width: .6em; height: .6em; background: var(--rosso); margin-right: .35em; vertical-align: .04em }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  :global(:root.grave) .tit { font-weight: 600; font-size: var(--c-riq) }
  .tit.crit::before { content: ""; display: inline-block; width: .42em; height: .42em; background: var(--ink); margin-right: .3em; vertical-align: .1em }
  .causa { font-size: var(--c-min); color: var(--grigio); line-height: 1.3 }
  .det { list-style: none; font-size: var(--c-min); color: var(--grigio); margin-top: 3px; display: grid; gap: 1px }
  .det li::before { content: "· " }
  a { color: inherit }
  .rim { font-size: var(--c-min); font-weight: 700; text-decoration: underline; text-underline-offset: 2px; white-space: nowrap }
  .aperto { display: inline-block; font-size: var(--c-min); font-weight: 700; border: 1.5px solid var(--ink); padding: 0 5px; margin-top: 4px }
  a.aperto:hover { background: var(--ink); color: var(--carta) }

  .testa .occh { font-size: var(--c-lead) }
  .testa .tit { font-size: var(--c-apre); line-height: 1.04; margin: 3px 0 6px }
  :global(:root.grave) .testa .tit { font-size: var(--c-pag) }
  .testa .causa { font-size: var(--c-sec) }
</style>
