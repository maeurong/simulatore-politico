<script lang="ts">
  // La voce scelta, a lato (25-registro.prototype.html, latoCollegate): fatto, causa, soggetti, rimando, e le voci degli altri turni.
  import { periodo } from "$lib/edicola/edicola";
  import { SEZIONE_DI } from "$lib/esito/esito";
  import { apribile, etichetta } from "$lib/ossatura";
  import { t, type Chiave } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceEsito } from "$lib/tipi/VoceEsito";
  import { collegate, corregge, periodoDel, riveduta, soggetti, turnoDa, type Soggetto, type Turno } from "./registro";

  type Props = { vista: Vista; turni: Turno[]; scelta: { turno: number; i: number } | null; onvai: (turno: number, i: number) => void };
  let { vista, turni, scelta, onvai }: Props = $props();

  const turno = $derived(scelta ? turni.find((x) => x.turno === scelta.turno) : undefined);
  const voce = $derived(turno && scelta ? turno.voci[scelta.i] : undefined);

  const sezione = $derived(voce ? SEZIONE_DI[voce.tipo] : null);
  const gruppi = $derived.by(() => {
    if (!voce || !turno) return [];
    const per = new Map<Soggetto["tipo"], string[]>();
    for (const { soggetto, nome } of soggetti([turnoDa(turno.turno, [voce])], vista)) per.set(soggetto.tipo, [...(per.get(soggetto.tipo) ?? []), nome]);
    return [...per];
  });
  const prima = $derived(voce && turno ? corregge(voce, turno.turno, turni) : null);
  const dopo = $derived(voce && turno ? riveduta(voce, turno.turno, turni) : null);
  const altre = $derived(voce && turno ? collegate(voce, turno.turno, turni) : null);
  const senzaSoggetto = $derived(!!voce && gruppi.length === 0 && !prima && !dopo);

  const dove = (n: number, y: VoceEsito) => ({ n, i: turni.find((x) => x.turno === n)?.voci.indexOf(y) ?? -1 });
  const rimando = (n: number) => periodo(periodoDel(n, vista));
</script>

{#snippet rif(n: number, y: VoceEsito, etichettaRif: string)}
  <button type="button" class="rif" onclick={() => onvai(dove(n, y).n, dove(n, y).i)}>
    <span class="meta">{t("registro.collegata", { sezione: etichettaRif, n, periodo: rimando(n) })}</span>
    {y.fatto}
  </button>
{/snippet}

{#if !voce || !turno || !scelta}
  <p class="ph">{t("registro.scegli")}</p>
{:else}
  <div class="meta">{t("registro.voce_di", { n: turno.turno, periodo: rimando(turno.turno), i: scelta.i + 1, k: turno.voci.length })}</div>
  <p class="fatto">{voce.fatto}</p>
  <p class="causa"><i>{t("componenti.perche")}</i> {voce.causa}</p>
  {#if voce.dettagli.length}
    <ul class="det">{#each voce.dettagli as d, i (i)}<li>{d.testo}</li>{/each}</ul>
  {/if}
  <dl class="dati">
    <dt>{t("registro.criticita")}</dt><dd>{t(`registro.c_${voce.criticita}` as Chiave)}</dd>
    <dt>{t("registro.prima_pagina")}</dt><dd>{t(voce.in_evidenza ? "registro.si" : "registro.no")}</dd>
    {#if voce.tono === "grave"}<dt>{t("registro.tono")}</dt><dd>{t("registro.grave")}</dd>{/if}
    {#each gruppi as [tipo, nomi] (tipo)}<dt>{t(`registro.soggetto_${tipo}` as Chiave)}</dt><dd>{nomi.join(", ")}</dd>{/each}
  </dl>
  {#if sezione && apribile(sezione, vista)}
    <a class="btn" href="/partita/{sezione}">{t("prima.rimando", { sezione: etichetta(sezione) })}</a>
  {/if}

  {#if prima}<h4 class="h5">{t("registro.corregge")}</h4>{@render rif(turno.turno - 1, prima, t("registro.prima_stima"))}{/if}
  {#if dopo}<h4 class="h5">{t("registro.riveduta_da")}</h4>{@render rif(turno.turno + 1, dopo, t("registro.revisione"))}{/if}
  {#if altre && gruppi.length}
    <h4 class="h5">{t("registro.collegate", { n: altre.stesse.length })}</h4>
    {#each altre.stesse as [n, y] (`${n}:${y.tipo}:${y.fatto}`)}{@render rif(n, y, etichetta(SEZIONE_DI[y.tipo]))}{:else}<p class="sec muto">{t("registro.nessuna")}</p>{/each}
    {#if altre.nelleRighe.length}
      <h4 class="h5">{t("registro.nelle_righe", { n: altre.nelleRighe.length })}</h4>
      {#each altre.nelleRighe as [n, y] (`${n}:${y.tipo}:${y.fatto}`)}{@render rif(n, y, t("registro.voce_fusa"))}{/each}
    {/if}
  {/if}
  {#if senzaSoggetto}<p class="sec muto">{t("registro.senza_soggetto")}</p>{/if}
{/if}

<style>
  .meta { font-size: var(--c-min); color: var(--grigio) }
  .ph { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 7px 11px; font-size: var(--c-sec) }
  .fatto { font-family: var(--tit); font-variation-settings: var(--wonk); font-size: var(--c-riq); font-weight: 700; line-height: 1.2; margin: 4px 0 6px; text-wrap: pretty }
  :global(:root.grave) .fatto { font-weight: 600 }
  .causa { font-size: var(--c-sec); color: var(--grigio) }
  .det { list-style: none; font-size: var(--c-sec); margin-top: 8px; display: grid; gap: 3px; border-top: var(--filo); padding-top: 6px }
  .det li { padding-left: 10px; text-indent: -10px }
  .det li::before { content: "· " }
  .dati { display: grid; grid-template-columns: auto 1fr; gap: 3px 12px; font-size: var(--c-sec); margin-top: 10px }
  .dati dt { color: var(--grigio) }
  .btn { display: inline-block; margin-top: 10px; border: var(--forte); padding: 4px 11px; font-size: var(--c-corpo); font-weight: 700; color: inherit; text-decoration: none; white-space: nowrap }
  .btn:hover { background: var(--ink); color: var(--carta) }
  .h5 { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); font-weight: 700; margin: 14px 0 4px }
  .rif { display: block; width: 100%; text-align: left; border: var(--bordo); padding: 6px 9px; margin-top: 6px; font-size: var(--c-sec) }
  .rif:hover { background: var(--velo) }
  .rif .meta { display: block; margin-bottom: 2px }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
</style>
