<script lang="ts">
  // Le Camere nel dettaglio a lato (24-partiti.prototype.html): in carica e se si votasse oggi, a barre coi colori dei partiti.
  import { t, type Chiave } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import BarraSeggi from "$lib/componenti/BarraSeggi.svelte";
  import { RAMI, barreSeggi, maggioranza } from "$lib/componenti/seggi";
  import { TETTO_PARTITI } from "./schieramento";

  let { vista }: { vista: Vista } = $props();

  const sciolte = $derived(vista.fase === "sciolta");
  // a Camere sciolte i seggi in carica non ci sono, anche se la Vista ne porta ancora
  const carica = $derived(sciolte ? [] : barreSeggi(vista, vista.seggi));
  const oggi = $derived(barreSeggi(vista, vista.proiezione));
  const vivi = $derived(vista.partiti.filter((p) => !p.sciolto).length);
  const metaDi = (ramo: (typeof RAMI)[number]) => maggioranza((oggi.length ? oggi : carica).find((b) => b.ramo === ramo)?.totale ?? 0);
</script>

{#snippet gruppo(titolo: Chiave, nota: string, elenco: ReturnType<typeof barreSeggi>)}
  <h5 class="grp"><span>{t(titolo)}</span><span class="nota">{nota}</span></h5>
  {#each elenco as b (b.ramo)}
    <p class="ramo">{t(`edicola.${b.ramo}`)}</p>
    <BarraSeggi righe={b.righe} totale={b.totale} />
  {/each}
{/snippet}

<h4 class="tit">{t("partiti.camere")}</h4>
{#if sciolte}
  <h5 class="grp"><span>{t("partiti.in_carica")}</span><span class="nota">{t("partiti.sciolte")}</span></h5>
{:else}
  {@render gruppo("partiti.in_carica", t("partiti.elette", { anno: vista.elezioni[0]?.data.anno ?? "—" }), carica)}
{/if}
{#if oggi.length}
  {@render gruppo("partiti.se_oggi", t("partiti.dal_sondaggio"), oggi)}
{:else}
  <h5 class="grp"><span>{t("partiti.se_oggi")}</span></h5>
  <p class="sec muto">{t("paese.senza_proiezione")}</p>
{/if}
<h5 class="grp"><span>{t("partiti.su_dieci", { n: vivi, tetto: TETTO_PARTITI })}</span></h5>
{#if carica.length || oggi.length}<p class="min muto">{t("partiti.nota_filo", { camera: metaDi("camera"), senato: metaDi("senato"), tetto: TETTO_PARTITI })}</p>{/if}

<style>
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-sez) }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .nota { letter-spacing: 0; text-transform: none; font-weight: 400 }
  .ramo { font-size: var(--c-sec); font-weight: 700 }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
