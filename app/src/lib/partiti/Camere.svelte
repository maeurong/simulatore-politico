<script lang="ts">
  // Le Camere nel dettaglio a lato (24-partiti.prototype.html): in carica e se si votasse oggi, a barre coi colori dei partiti.
  import { t, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import type { SeggiPartito } from "$lib/tipi/SeggiPartito";
  import type { Vista } from "$lib/tipi/Vista";
  import BarraSeggi from "$lib/componenti/BarraSeggi.svelte";
  import { TETTO_PARTITI } from "./schieramento";

  let { vista }: { vista: Vista } = $props();

  const rami = ["camera", "senato"] as const;
  /** I seggi di un elenco, ramo per ramo; il totale è la somma delle voci, e a zero la barra non si disegna. */
  const barre = (elenco: SeggiPartito[] | null) =>
    rami.flatMap((ramo) => {
      const righe = (elenco ?? []).map((s) => ({ id: s.partito, nome: nomeDi(vista, s.partito), seggi: s[ramo] }));
      const totale = righe.reduce((a, r) => a + r.seggi, 0);
      return totale ? [{ ramo, righe, totale }] : [];
    });
  const sciolte = $derived(vista.fase === "sciolta");
  const carica = $derived(barre(vista.seggi));
  const oggi = $derived(barre(vista.proiezione));
  const vivi = $derived(vista.partiti.filter((p) => !p.sciolto).length);
  const maggioranza = (ramo: (typeof rami)[number]) => {
    const tot = barre(vista.proiezione).concat(barre(vista.seggi)).find((b) => b.ramo === ramo)?.totale ?? 0;
    return Math.floor(tot / 2) + 1;
  };
</script>

{#snippet gruppo(titolo: Chiave, nota: string, elenco: ReturnType<typeof barre>)}
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
{#if carica.length || oggi.length}<p class="min muto">{t("partiti.nota_filo", { camera: maggioranza("camera"), senato: maggioranza("senato"), tetto: TETTO_PARTITI })}</p>{/if}

<style>
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-sez) }
  .grp { display: flex; justify-content: space-between; gap: 8px; margin-top: 6px; padding-bottom: 2px; border-bottom: var(--bordo); font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .nota { letter-spacing: 0; text-transform: none; font-weight: 400 }
  .ramo { font-size: var(--c-sec); font-weight: 700 }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
