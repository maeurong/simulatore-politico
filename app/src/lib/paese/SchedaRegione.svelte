<script lang="ts">
  // La scheda della regione nel lato (23-paese.prototype.html, forma A): segni, sondaggio, i diciotto gruppi coi loro elettori, le disposizioni.
  import { t, numero, percentuale, type Chiave } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio } from "$lib/guscio/chiamate";
  import { barrata } from "$lib/ossatura";
  import { coloreDi, nomeDi } from "$lib/partiti";
  import { REGIONI } from "$lib/mappa/regioni";
  import FormaSegno from "$lib/mappa/FormaSegno.svelte";
  import type { Condizione } from "$lib/tipi/Condizione";
  import type { Fascia } from "$lib/tipi/Fascia";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import { CONDIZIONI, FASCE, disposizioniDi, segniDi, sessioneElettori, sondaggioDi, sondaggioLetto, type EsitoElettori } from "./paese";

  let { vista, regione, indietro }: { vista: Vista; regione: string; indietro: () => void } = $props();

  const nome = $derived(REGIONI.find((r) => r.id === regione)?.nome ?? regione);
  const segni = $derived(segniDi(vista, regione));
  const quote = $derived(sondaggioDi(vista, regione));
  const massimo = $derived(Math.max(0.01, ...quote.map((q) => q.quota)));
  const letto = $derived(sondaggioLetto(vista));
  const campione = $derived(letto.sondaggio.regioni.find((r) => r.id === regione)?.campione ?? 0);
  const dispo = $derived(disposizioniDi(vista, regione));
  const aulaAperta = $derived(barrata("aula", vista) === null);
  const pc = (x: number) => percentuale(x, 1);

  let casella = $state<{ condizione: Condizione; fascia: Fascia } | null>(null);
  let esito = $state.raw<EsitoElettori | null>(null);
  const sessione = sessioneElettori(guscio.elettoriCampione, (e) => {
    esito = e;
    // un guasto va nella cornice; il rifiuto sta sotto la griglia
    if ("guasto" in e) partita.guasto = e.guasto;
  });
  $effect(() => {
    esito = null;
    if (casella) sessione.chiedi(regione, casella.condizione, casella.fascia);
    else sessione.annulla();
    return sessione.annulla;
  });
  const scegli = (condizione: Condizione, fascia: Fascia) =>
    (casella = casella?.condizione === condizione && casella.fascia === fascia ? null : { condizione, fascia });

  const gradino = (d: (typeof dispo)[number], id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
</script>

<Pulsante forma="link" onclick={indietro}>‹ {t("paese.indietro")}</Pulsante>
<h4 class="tit">{nome}</h4>
{#if segni.length}
  <ul class="elenco">
    {#each segni as s, i (i)}
      <li><svg width="12" height="12" viewBox="-7 -7 14 14" aria-hidden="true"><FormaSegno s={s.segno} /></svg>{s.testo}</li>
    {/each}
  </ul>
{/if}
<Segnaposto sezione={t("paese.ph_territorio")} fetta="11g" />

<h5 class="grp"><span>{letto.privato ? t("paese.privato_titolo") : t("paese.scheda_sondaggio")}</span><span>{t("paese.incrocio_campione", { n: numero(campione) })}</span></h5>
<Segnaposto sezione={t("paese.margine")} fetta="11g" />
<div class="barre">
  {#each quote as p (p.id)}
    <span class="n" class:tu={p.id === vista.giocatore}>{p.nome}</span>
    <span class="b"><i style:width="{(p.quota / massimo) * 100}%" style:background={coloreDi(p.id) ?? "var(--retino)"}></i></span>
    <span class="q" class:tu={p.id === vista.giocatore}>{pc(p.quota)}</span>
  {/each}
</div>

<h5 class="grp"><span>{t("paese.gruppi")}</span></h5>
<Segnaposto sezione={t("paese.gruppi_quota")} fetta="11g" />
<div class="gruppi">
  <span></span>
  {#each FASCE as f (f)}<span class="hh">{t(`paese.fascia_${f}` as Chiave)}</span>{/each}
  {#each CONDIZIONI as c (c)}
    <span class="rr">{t(`condizione.${c}` as Chiave)}</span>
    {#each FASCE as f (f)}
      {@const qui = casella?.condizione === c && casella.fascia === f}
      <button type="button" class:sel={qui} aria-pressed={qui} aria-label={t("paese.gruppo_aria", { condizione: t(`condizione.${c}` as Chiave), fascia: t(`paese.fascia_${f}` as Chiave) })} onclick={() => scegli(c, f)}>—</button>
    {/each}
  {/each}
</div>
{#if !casella}
  <p class="min muto">{t("paese.gruppi_aiuto")}</p>
{:else if esito && "elettori" in esito}
  <div class="elettori">
    {#each esito.elettori as e, i (i)}
      <div class="e">
        <span class="tit eta">{e.eta}</span>
        <span><b>{e.nome}</b> <span class="vt">· {e.voto ? t("paese.vota", { partito: nomeDi(vista, e.voto) }) : t("paese.non_vota")}</span></span>
        <span class="mt">{t(e.voto ? "paese.perche" : "paese.non_va", { motivo: t(`paese.motivo_${e.motivo}` as Chiave) })} · {t("paese.si_colloca", { n: e.posizione })}</span>
      </div>
    {/each}
    <p class="min muto">{t("paese.inventati")}</p>
  </div>
{:else if esito && "rifiuto" in esito}
  <Rifiuto testo={esito.rifiuto} />
{/if}

<Segnaposto sezione={t("paese.ph_senato")} fetta="11g" />

<h5 class="grp"><span>{t("paese.dispo")}</span><span>{dispo.length}</span></h5>
{#if dispo.length}
  <div class="dispo">
    {#each dispo as d (d.id)}
      <div class="d">
        <span>{d.nome}</span><b>{gradino(d, d.in_vigore)}</b>
        <span class="m">
          {d.legge}{d.in_arrivo.length ? ` · ${t("paese.dispo_in_arrivo", { gradini: d.in_arrivo.map((a) => t("paese.dispo_dal", { nome: gradino(d, a.gradino), anno: a.data.anno })).join(", ") })}` : ""} ·
          {#if aulaAperta}<a class="aula" href="/partita/aula">{t("paese.in_aula")}</a>{:else}{t("paese.in_aula")}{/if}
        </span>
      </div>
    {/each}
  </div>
{:else}
  <p class="min muto">{t("paese.dispo_nessuna")}</p>
{/if}

<style>
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-sez) }
  .elenco { display: grid; gap: 3px; font-size: var(--c-sec); list-style: none; padding: 0 }
  .elenco li { display: flex; gap: 6px; align-items: baseline; border-bottom: var(--filo); padding: 2px 0 }
  .elenco svg { flex: none; align-self: center }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .barre { display: grid; grid-template-columns: minmax(0, 120px) minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; font-size: var(--c-sec) }
  .n { overflow-wrap: anywhere }
  .b { display: block; height: 12px; position: relative }
  .b i { position: absolute; inset: 0 auto 0 0; border: 1px solid var(--ink) }
  .q { text-align: right; white-space: nowrap; font-variant-numeric: tabular-nums }
  .tu { font-weight: 700 }

  .gruppi { display: grid; grid-template-columns: minmax(0, 1fr) repeat(3, 52px); gap: 1px; font-size: var(--c-sec) }
  .hh { font-size: var(--c-min); color: var(--grigio); font-weight: 700; text-align: center; border-bottom: 1.5px solid var(--ink) }
  .rr { overflow-wrap: anywhere; padding: 1px 0 }
  .rr::first-letter { text-transform: uppercase }
  .gruppi button { text-align: center; border: var(--filo); padding: 1px 0; font-weight: 700 }
  .gruppi button:hover { background: var(--velo) }
  .gruppi button.sel { background: var(--ink); color: var(--carta) }
  .elettori { display: grid; gap: 5px }
  .e { display: grid; grid-template-columns: 34px minmax(0, 1fr); gap: 0 8px; align-items: center; border-bottom: var(--filo); padding-bottom: 4px }
  .eta { text-align: center; font-size: var(--c-riq) }
  .vt { font-size: var(--c-sec) }
  .mt { font-size: var(--c-min); color: var(--grigio); grid-column: 2 }

  .dispo { display: grid; gap: 3px }
  .d { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 0 8px; font-size: var(--c-sec); border-bottom: var(--filo); padding: 2px 0 }
  .m { grid-column: 1 / -1; font-size: var(--c-min); color: var(--grigio) }
  .aula { color: inherit; text-decoration: underline; text-underline-offset: 2px }
</style>
