<script lang="ts">
  // La scheda della regione nel lato (23-paese.prototype.html, forma A): segni, sondaggio, i diciotto gruppi coi loro elettori, le disposizioni.
  import { t, numero, percentuale, type Chiave } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio } from "$lib/guscio/chiamate";
  import { barrata } from "$lib/ossatura";
  import { coloreDi, nomeDi } from "$lib/partiti";
  import { periodoDi } from "$lib/aula/aula";
  import { REGIONI } from "$lib/mappa/regioni";
  import FormaSegno from "$lib/mappa/FormaSegno.svelte";
  import type { Condizione } from "$lib/tipi/Condizione";
  import type { Fascia } from "$lib/tipi/Fascia";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import BarreQuote from "./BarreQuote.svelte";
  import { CONDIZIONI, FASCE, disposizioniDi, margineTesto, segniDi, sessioneElettori, sondaggioDi, sondaggioLetto, territorioDi, tonoGrave, votoPerRegione, type EsitoElettori } from "./paese";

  let { vista, regione, indietro }: { vista: Vista; regione: string; indietro: () => void } = $props();

  const nome = $derived(REGIONI.find((r) => r.id === regione)?.nome ?? regione);
  const segni = $derived(segniDi(vista, regione));
  const quote = $derived(sondaggioDi(vista, regione));
  const letto = $derived(sondaggioLetto(vista));
  const riga = $derived(letto.sondaggio.regioni.find((r) => r.id === regione));
  const territorio = $derived(territorioDi(vista, regione));
  const gruppi = $derived(vista.regioni.find((r) => r.id === regione)?.gruppi);
  const voto = $derived(votoPerRegione(vista)?.[regione]);
  const grave = $derived(tonoGrave(vista));
  const dispo = $derived(disposizioniDi(vista, regione));
  const aulaAperta = $derived(barrata("aula", vista) === null);

  const turno = $derived(vista.turno);
  let casella = $state<{ condizione: Condizione; fascia: Fascia } | null>(null);
  let esito = $state.raw<EsitoElettori | null>(null);
  const sessione = sessioneElettori(guscio.elettoriCampione, (e) => {
    esito = e;
    // un guasto va nella cornice; il rifiuto sta sotto la griglia
    if ("guasto" in e) partita.guasto = e.guasto;
  });
  $effect(() => {
    esito = null;
    if (casella) sessione.chiedi(regione, casella.condizione, casella.fascia, turno);
    else sessione.annulla();
    return sessione.annulla;
  });
  const scegli = (condizione: Condizione, fascia: Fascia) =>
    (casella = casella?.condizione === condizione && casella.fascia === fascia ? null : { condizione, fascia });

  const gradino = (d: (typeof dispo)[number], id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
</script>

<Pulsante forma="link" onclick={indietro}>‹ {t("paese.indietro")}</Pulsante>
<div class="capo"><h4 class="tit">{nome}</h4>{#if territorio}<span class="sec muto">{t(`paese.macro_${territorio.macroarea}` as Chiave)}</span>{/if}</div>
{#if segni.length}
  <ul class="elenco">
    {#each segni as s, i (i)}
      <li><svg width="12" height="12" viewBox="-7 -7 14 14" aria-hidden="true"><FormaSegno s={s.segno} /></svg>{s.testo}</li>
    {/each}
  </ul>
{/if}
{#if territorio}
  <dl class="dati">
    <dt>{t("paese.disoccupazione")}</dt><dd><b>{numero(territorio.disoccupazione, 1)}%</b> <span class="muto">· {t("paese.trimestre_prima")}</span></dd>
    <dt>{t("paese.reddito")}</dt><dd><b>{numero(territorio.reddito)} €</b> <span class="muto">{t("paese.reddito_nota")}</span></dd>
    <dt>{t("paese.pericolosita")}</dt>
    <dd>{#each ["sisma", "idro", "incendi"] as const as p, i (p)}{i ? " · " : ""}{t(`paese.tipo_${p}` as Chiave)} <b>{t(`paese.fascia_${territorio.pericolosita[p]}` as Chiave)}</b>{/each}</dd>
  </dl>
{/if}

<h5 class="grp"><span>{letto.privato ? t("paese.privato_titolo") : t("paese.scheda_sondaggio")}</span>{#if riga}<span>{t("paese.incrocio_campione", { n: numero(riga.campione) })} · {margineTesto(riga.margine)}</span>{/if}</h5>
{#if riga}<BarreQuote righe={quote} giocatore={vista.giocatore} />{:else}<p class="min muto">{t("mappa.senza_dato")}</p>{/if}

<h5 class="grp"><span>{t("paese.gruppi")}</span><span>{t("paese.gruppi_quota")}</span></h5>
<div class="gruppi">
  <span></span>
  {#each FASCE as f (f)}<span class="hh">{t(`paese.fascia_${f}` as Chiave)}</span>{/each}
  {#each CONDIZIONI as c, i (c)}
    <span class="rr">{t(`condizione.${c}` as Chiave)}</span>
    {#each FASCE as f, j (f)}
      {@const qui = casella?.condizione === c && casella.fascia === f}
      <button type="button" class:sel={qui} aria-pressed={qui} aria-label={t("paese.gruppo_aria", { condizione: t(`condizione.${c}` as Chiave), fascia: t(`paese.fascia_${f}` as Chiave) })} onclick={() => scegli(c, f)}>{gruppi ? percentuale(gruppi[i * 3 + j], 0) : "—"}</button>
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

<h5 class="grp"><span>{t("paese.senato")}</span><span>{voto ? t("paese.senato_voto", { periodo: periodoDi(vista, vista.elezioni.at(-1)!.data) }) : t("paese.senato_nessuno")}</span></h5>
{#if voto}
  <div class="legenda">
    {#each voto.senato.filter((x) => x.seggi) as x (x.partito)}<span class="sg">{#if !grave}<i class="sw" style:background={coloreDi(x.partito) ?? "var(--retino), var(--carta)"}></i>{/if}{nomeDi(vista, x.partito)} <b>{x.seggi}</b></span>{/each}
  </div>
  <p class="min muto">{t("paese.primo_partito", { nome: nomeDi(vista, voto.primo) })}</p>
{:else}
  <p class="min muto">{vista.elezioni.length ? t("mappa.senza_dato") : t("paese.senato_dopo")}</p>
{/if}

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
  .capo { display: flex; justify-content: space-between; align-items: baseline; gap: 8px }
  .sec { font-size: var(--c-sec) }
  .dati { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 2px 10px; font-size: var(--c-sec); margin: 4px 0 }
  .dati dt { color: var(--grigio) }
  .dati dd { margin: 0 }
  .legenda { display: flex; flex-wrap: wrap; gap: 2px 10px; font-size: var(--c-sec) }
  .sg { display: inline-flex; align-items: center; gap: 4px }
  .sw { width: 9px; height: 9px; border: 1px solid var(--ink); display: inline-block }
  .elenco { display: grid; gap: 3px; font-size: var(--c-sec); list-style: none; padding: 0 }
  .elenco li { display: flex; gap: 6px; align-items: baseline; border-bottom: var(--filo); padding: 2px 0 }
  .elenco svg { flex: none; align-self: center }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }

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
