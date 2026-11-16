<script lang="ts">
  // La spesa a destra (storia 125): i dieci dicasteri come barre (salari, acquisti, margine, resto), la spesa, la perdita reale e «Indicizza» col costo; sotto la barra il funzionamento tagliato.
  // Presentata: niente da modificare; sulle barre dei dicasteri chiesti, il foglietto dell'alleato con Accogli · Respingi.
  import { euro } from "$lib/governo/governo";
  import { reale } from "$lib/conti/conti";
  import { numero, t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceEmendamento } from "$lib/tipi/VoceEmendamento";
  import { nomeDi } from "$lib/partiti";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { euroSegnato, rigaTagliato, tagliato } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { vista, bilancio, riepilogo, ferma, emendamenti, emenda, rifiutoEm, scelta = $bindable() }: {
    vista: Vista; bilancio: Bilancio; riepilogo: RiepilogoBilancio | null; ferma: boolean; emendamenti: VoceEmendamento[];
    emenda: (partito: string, accogli: boolean) => void; rifiutoEm: (partito: string) => string | null; scelta: string | null;
  } = $props();

  const nome = (id: string) => vista.dicasteri.find((d) => d.id === id)?.nome ?? id;
  const mx = $derived(Math.max(...bilancio.dicasteri.map((d) => d.spesa)));
  const w = (x: number) => `${(x / mx) * 100}%`;
  const tutto = $derived(bozza.costi.tutto);
  const indicatore = (id: string) => {
    const g = vista.dicasteri.find((d) => d.id === id)?.indicatore;
    return vista.grandezze.find((x) => x.id === g)?.nome ?? id;
  };
</script>

{#snippet costo(c: RiepilogoBilancio | string | undefined)}
  {#if typeof c === "object"}{euroSegnato(c.costo_spesa)}{:else if c}<span class="muto">{c}</span>{/if}
{/snippet}

<section>
  <div class="grp">
    <span>{t("bilancio.spesa_dicasteri")}</span>
    {#if !ferma && bozza.indicizzato({ tipo: "tutto" })}<span class="tutto"><span class="bene">{t("bilancio.indicizzato_tutto")}</span> · <button type="button" class="link" onclick={() => bozza.annulla({ tipo: "tutto" })}>{t("bilancio.annulla")}</button></span>
    {:else if !ferma}<span class="tutto"><button type="button" class="link" onclick={() => bozza.indicizza({ tipo: "tutto" })}>{t("bilancio.indicizza_tutto")}</button> {@render costo(tutto)}</span>{/if}
  </div>
  {#if !ferma && bozza.rifiutoDi({ tipo: "tutto" })}<Rifiuto testo={bozza.rifiutoDi({ tipo: "tutto" })!} />{/if}
  <div class="leg">
    <span><i style:background="var(--ink)"></i>{t("bilancio.salari")}</span><span><i style:background="var(--grigio)"></i>{t("bilancio.acquisti")}</span>
    <span><i style:background="var(--biro)"></i>{t("bilancio.margine")}</span><span><i class="ret"></i>{t("bilancio.resto")}</span>
    <span>{t("bilancio.reale_dal", { anno: vista.bilancio.anno })}</span>
  </div>
  {#each bilancio.dicasteri as d (d.id)}
    {@const resto = d.spesa - d.salari - d.acquisti - d.margine}
    {@const p = riepilogo?.perdita_reale.find((x) => x.voce === d.id)?.perdita}
    {@const r = p === undefined ? null : reale(p)}
    {@const voce = { tipo: "dicastero" as const, id: d.id }}
    {@const k = `dic:${d.id}`}
    {@const z = bozza.base?.dicasteri.find((x) => x.id === d.id)}
    {@const taglio = z ? tagliato(d, z) : null}
    {@const rif = bozza.rifiutoDi(voce)}
    {@const em = emendamenti.find((e) => e.dicasteri.includes(d.id))}
    <div class="barD" class:sel={scelta === k}>
      <!-- nome, barra e cifra sono il bottone che apre il dettaglio; «Indicizza» resta fuori, accanto -->
      <button type="button" class="apre" aria-pressed={scelta === k} onclick={() => (scelta = k)}>
        <span class="n">{nome(d.id)}</span>
        <!-- il margine ha un minimo visibile: anche piccolo resta il pezzo che si allarga -->
        <span class="sp" title={t("bilancio.barra", { salari: euro(d.salari), acquisti: euro(d.acquisti), margine: euro(d.margine), resto: euro(resto) })}>
          <i style:width={w(d.salari)} style:background="var(--ink)"></i><i style:width={w(d.acquisti)} style:background="var(--grigio)"></i><i
            style:width={w(Math.max(d.margine, mx * 0.004))} style:background="var(--biro)"></i><i class="ret" style:width={w(Math.max(resto, 0))}></i>
        </span>
        <span class="x">{t("conti.miliardi", { n: numero(d.spesa / 1e9, 1) })}</span>
      </button>
      <span class="az">
        {#if r}<span class={r.tono}>{r.testo}</span>{:else}<span class="muto">—</span>{/if}
        <!-- «Indicizza» solo con una perdita da recuperare: senza riepilogo o sotto 0,05 niente (prototipo 22:427) -->
        {#if !ferma && bozza.indicizzato(voce)}<span class="ind"><span class="min bene">{t("bilancio.indicizzato")}</span><button type="button" class="link" onclick={() => bozza.annulla(voce)}>{t("bilancio.annulla")}</button></span>
        {:else if !ferma && p !== undefined && p >= 0.05}<span class="ind"><button type="button" class="link" onclick={() => bozza.indicizza(voce)}>{t("bilancio.indicizza")}</button> <span class="min muto">{@render costo(bozza.costi[d.id])}</span></span>{/if}
      </span>
      {#if rif && !ferma}<span class="sotto"><Rifiuto testo={rif} /></span>{/if}
      {#if em}
        {@const chi = nomeDi(vista, em.partito)}
        <span class="sotto slip chiede">
          <span>{t("bilancio.chiede", { nome: chi })}{em.stato === "aperto" ? "" : ` · ${t(em.stato === "accolto" ? "aula.accolto" : "aula.respinto")}`}</span>
          {#if em.stato === "aperto"}
            <span class="ind">
              <button type="button" class="link" aria-label={t("aula.accogli_di", { nome: chi })} onclick={() => emenda(em.partito, true)}>{t("aula.accogli")}</button>
              <button type="button" class="link" aria-label={t("aula.respingi_di", { nome: chi })} onclick={() => emenda(em.partito, false)}>{t("aula.respingi")}</button>
            </span>
          {/if}
        </span>
        {#if rifiutoEm(em.partito)}<span class="sotto"><Rifiuto testo={rifiutoEm(em.partito)!} /></span>{/if}
      {/if}
      {#if taglio}<span class="sotto slip">{rigaTagliato(taglio, indicatore(d.id))}</span>{/if}
    </div>
  {/each}
</section>

<style>
  .leg { display: flex; flex-wrap: wrap; gap: 4px 12px; font-size: var(--c-min); color: var(--grigio) }
  .leg i { display: inline-block; width: 10px; height: 10px; margin-right: 4px; vertical-align: -1px }
  .ret { background: var(--retino); outline: 1px solid var(--grigio); outline-offset: -1px }
  .barD { display: grid; grid-template-columns: 150px minmax(0, 1fr) 64px 184px; gap: 2px 8px; align-items: center; padding: 3px 2px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .apre { grid-column: 1 / 4; display: grid; grid-template-columns: subgrid; align-items: center; text-align: left }
  .apre:hover .n { text-decoration: underline; text-underline-offset: 2px }
  .n { white-space: nowrap; overflow: hidden; text-overflow: ellipsis }
  .sotto { grid-column: 1 / -1; margin-left: 158px }
  /* il foglietto del funzionamento tagliato (prototipo 22: funz) */
  .slip { border: 1.5px solid var(--ink); padding: 1px 6px; font-size: var(--c-min); justify-self: start }
  /* il foglietto dell'alleato (prototipo 22: .barD .slip): rosso, nero nel tono grave */
  .chiede { justify-self: stretch; display: flex; gap: 8px; align-items: baseline; justify-content: space-between; border-color: var(--rosso); color: var(--rosso) }
  .chiede .link { font-size: var(--c-min) }
  :global(:root.grave) .chiede { border-color: var(--ink); color: var(--ink) }
  .sp { height: 12px; display: flex }
  .sp i { height: 100%; border-right: 1px solid var(--carta) }
  .x { text-align: right; font-weight: 700; white-space: nowrap }
  .az { display: grid; grid-template-columns: 46px auto; gap: 6px; justify-content: start; align-items: baseline; white-space: nowrap }
  .ind { display: inline-flex; gap: 6px; align-items: baseline }
  /* la cifra in maiuscoletto si leggerebbe «MLD»: il comando e il costo restano in tondo */
  .tutto { letter-spacing: normal; text-transform: none; font-weight: 400; color: var(--ink) }
  .tutto .link { font-size: var(--c-min); font-weight: 700 }
</style>
