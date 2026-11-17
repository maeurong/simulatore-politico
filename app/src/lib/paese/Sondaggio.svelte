<script lang="ts">
  // Il sondaggio nel dettaglio a lato (23-paese.prototype.html, forma A): la media degli istituti, il privato, i seggi, gli incroci.
  import { t, numero, percentuale, type Chiave } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { acquisto } from "$lib/acquisto.svelte";
  import { coalizioneDi, coloreDi, quotaIncrocio } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import BarraSeggi from "$lib/componenti/BarraSeggi.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
    import { barreSeggi, maggioranza } from "$lib/componenti/seggi";
  import BarreQuote from "./BarreQuote.svelte";
  import { margineIstituti, margineTesto, media, quoteDi, sondaggioLetto } from "./paese";

  let { vista }: { vista: Vista } = $props();

  const letto = $derived(sondaggioLetto(vista));
  const pubblica = $derived(media(vista, vista.sondaggio));
  const privata = $derived(letto.privato ? media(vista, letto.sondaggio) : null);
  const privataDi = $derived(new Map(privata?.partiti.map((p) => [p.id, p.quota])));
  const massimo = $derived(Math.max(0.01, ...pubblica.partiti.map((p) => p.quota)));
  const prezzo = $derived(vista.acquisti.prezzi.find((p) => p.acquisto === "sondaggio_privato")?.euro);
  const cassa = $derived(vista.partiti.find((p) => p.id === vista.giocatore)?.cassa ?? 0);
  const compra = acquisto("sondaggio_privato");

  const coalizione = $derived(coalizioneDi(vista));
  const rami = $derived(
    barreSeggi(vista, vista.proiezione).map((b) => ({ ...b, n: b.righe.filter((r) => coalizione.includes(r.id)).reduce((a, r) => a + r.seggi, 0), meta: maggioranza(b.totale) })),
  );

  let scelto = $state<string | null>(null);
  const incrocio = $derived(letto.sondaggio.condizioni.find((c) => c.id === scelto));
  const quoteIncrocio = $derived(incrocio ? quoteDi(vista, incrocio) : []);
  const giocatore = $derived(vista.partiti.findIndex((p) => p.id === vista.giocatore));
  const picco = $derived(Math.max(0.0001, ...(incrocio?.posizione ?? [])));
  const pc = (x: number | null) => (x === null ? "—" : percentuale(x, 1));
</script>

{#snippet barra(id: string, quota: number, scala: number)}
  <span class="b"><i style:width="{(quota / scala) * 100}%" style:background={coloreDi(id) ?? "var(--retino)"}></i></span>
{/snippet}

{#if incrocio}
  <Pulsante forma="link" onclick={() => (scelto = null)}>‹ {t("paese.indietro")}</Pulsante>
  <h4 class="tit nome">{t(`condizione.${incrocio.id}` as Chiave)}</h4>
  <p class="sec">{t("paese.incrocio_campione", { n: numero(incrocio.campione) })} · {margineTesto(incrocio.margine)}{letto.privato ? ` · ${t("paese.dal_privato")}` : ""}</p>
  <BarreQuote righe={quoteIncrocio} giocatore={vista.giocatore} />
  <p class="sec">{t("paese.approvazione")} <b>{numero(incrocio.approvazione, 2)}</b> <span class="muto">{t("paese.approvazione_nota")}</span></p>
  <h5 class="grp"><span>{t("paese.collocazione")}</span><span>{t("paese.collocazione_asse")}</span></h5>
  <svg width="100%" height="44" viewBox="0 0 280 44" preserveAspectRatio="none" role="img" aria-label={t("paese.collocazione_aria")}>
    {#each incrocio.posizione as x, i (i)}
      <rect x={i * 40 + 4} y={36 - (x / picco) * 34} width="32" height={(x / picco) * 34} fill="var(--ink)" />
      <text x={i * 40 + 20} y="44" font-size="9" text-anchor="middle" fill="var(--grigio)">{i + 1}</text>
    {/each}
  </svg>
{:else}
  <h4 class="tit">{t("paese.sondaggio")}</h4>
  <div class="barre media" class:con-privato={privata}>
    <span class="n"></span><span></span>
    <span class="h">{t("paese.col_media")}<small>{margineIstituti(vista.sondaggio)}</small></span>{#if privata}<span class="h priv">{t("paese.col_privato")}<small>{margineIstituti(letto.sondaggio)}</small></span>{/if}
    {#each pubblica.partiti as p (p.id)}
      <span class="n" class:tu={p.id === vista.giocatore}>{p.nome}</span>
      {@render barra(p.id, p.quota, massimo)}
      <span class="q" class:tu={p.id === vista.giocatore}>{pc(p.quota)}</span>
      {#if privata}<span class="q priv">{pc(privataDi.get(p.id) ?? null)}</span>{/if}
    {/each}
    <span class="n muto">{t("paese.astensione")}</span><span></span>
    <span class="q muto">{pc(pubblica.astensione)}</span>{#if privata}<span class="q priv muto">{pc(privata.astensione)}</span>{/if}
  </div>
  <p class="min muto">{t("paese.nota_quote")}</p>

  <div class="priv-box">
    {#if letto.privato}
      <div class="capo"><b>{t("paese.privato_titolo")}</b><span class="sec muto">{t("paese.privato_comprato", { euro: numero(prezzo ?? 0) })}</span></div>
      <p class="sec">{t("paese.privato_corpo", { n: numero(letto.sondaggio.istituti[0]?.campione ?? 0) })}</p>
      <Pulsante forma="link" aria-disabled={partita.occupato ? "true" : undefined} onclick={compra.compra}>{t("paese.ricompra")}</Pulsante>
    {:else}
      <b>{t("paese.privato_titolo")}</b>
      <p class="sec">{t("paese.privato_spiega")}</p>
      {#if prezzo !== undefined}
        <div class="prezzo">
          <Pulsante forma="bordo" aria-disabled={partita.occupato ? "true" : undefined} onclick={compra.compra}>{t("paese.compra", { euro: numero(prezzo) })}</Pulsante>
          <span class="sec muto">{t("paese.cassa", { euro: numero(cassa) })}</span>
        </div>
      {/if}
    {/if}
    {#if compra.rifiuto}<Rifiuto testo={compra.rifiuto} />{/if}
  </div>

  <h5 class="grp"><span>{t("paese.seggi")}</span></h5>
  {#if rami.length}
    {#each rami as r (r.ramo)}
      <p class="sec"><b>{t(`edicola.${r.ramo}`)}</b> · {t("paese.seggi_coalizione", { n: r.n, tot: r.totale, meta: r.meta })}</p>
      <BarraSeggi righe={r.righe} totale={r.totale} segno={r.n} />
    {/each}
    <p class="min muto">{t("paese.seggi_nota")}</p>
  {:else}
    <p class="sec muto">{t("paese.senza_proiezione")}</p>
  {/if}

  <h5 class="grp"><span>{t("paese.incroci")}</span></h5>
  <table class="tab">
    <thead><tr><th>{t("paese.col_condizione")}</th><th>{t("paese.col_campione")}</th><th>{t("paese.col_tu")}</th></tr></thead>
    <tbody>
      {#each letto.sondaggio.condizioni as c (c.id)}
        <tr>
          <td><Pulsante forma="link" onclick={() => (scelto = c.id)}><span class="cond">{t(`condizione.${c.id}` as Chiave)}</span></Pulsante></td>
          <td class="muto">{numero(c.campione)}</td>
          <td><b>{pc(quotaIncrocio(c, giocatore))}</b></td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}

<style>
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-sez) }
  .nome::first-letter, .cond::first-letter { text-transform: uppercase }
  .cond { display: inline-block }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .barre { display: grid; grid-template-columns: minmax(0, 150px) minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; font-size: var(--c-sec) }
  .barre.con-privato { grid-template-columns: minmax(0, 150px) minmax(0, 1fr) auto auto }
  .n { overflow-wrap: anywhere }
  .b { display: block; height: 12px; position: relative }
  .b i { position: absolute; inset: 0 auto 0 0; border: 1px solid var(--ink) }
  .q { text-align: right; white-space: nowrap; font-variant-numeric: tabular-nums }
  .h { font-size: var(--c-min); font-weight: 700; color: var(--grigio); text-align: right }
  .h small { display: block; font-weight: 400 }
  .tu { font-weight: 700 }
  .priv { font-weight: 700; color: var(--biro); box-shadow: inset 1.5px 0 0 var(--biro); padding-left: 6px }
  .priv-box { border: var(--forte); padding: 8px 10px; display: grid; gap: 6px; justify-items: start }
  .capo { display: flex; justify-content: space-between; gap: 10px; width: 100% }
  .prezzo { display: flex; align-items: center; gap: 10px; flex-wrap: wrap }
  .tab { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .tab th { font-size: var(--c-min); text-align: right; color: var(--grigio); font-weight: 700; padding: 2px 5px; border-bottom: var(--bordo) }
  .tab td { padding: 2px 5px; border-bottom: var(--filo); text-align: right }
  .tab th:first-child, .tab td:first-child { text-align: left }
</style>
