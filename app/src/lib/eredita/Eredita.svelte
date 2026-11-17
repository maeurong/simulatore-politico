<script lang="ts">
  // L'ultima edizione, forma A (30-eredita.prototype.html): testa, tre colonne, tre grafici, il dettaglio a lato.
  // Con `azioni` è la bozza dentro la partita; senza, la stessa pagina riletta dall'albo.
  import { periodo } from "$lib/edicola/edicola";
  import Grafico from "$lib/edicola/Grafico.svelte";
  import { numero, segnato, t } from "$lib/testi/t";
  import Bozza from "./Bozza.svelte";
  import Colonne from "./Colonne.svelte";
  import Testa from "./Testa.svelte";
  import type { Edizione } from "./eredita";

  let { edizione, azioni }: { edizione: Edizione; azioni: boolean } = $props();

  const S = $derived(edizione.serie);
  const etichette = $derived(S.map((p) => t("edicola.punto", { turno: p.turno, periodo: periodo(p.data) })));
  const voti = $derived(
    edizione.elezioni.map((e) => S.findIndex((p) => p.data.anno === e.data.anno && p.data.trimestre === e.data.trimestre)).filter((i) => i >= 0),
  );
</script>

<div class="cr">
  <div class="pag">
    <Testa {edizione} />
    <Colonne {edizione} />
    {#if S.length < 2}
      <p class="sec muto">{t("edicola.serie_corta")}</p>
    {:else}
      <div class="gr3">
        <Grafico titolo={t("edicola.grafico_consenso")} linee={[{ valori: S.map((p) => p.consenso * 100) }]} formato={(v) => `${numero(v, 1)}%`} {etichette} {voti} />
        <Grafico titolo={t("edicola.grafico_margine")} zero formato={(v) => segnato(v)} {etichette} {voti}
          linee={[{ nome: t("edicola.camera"), valori: S.map((p) => p.margine_camera) }, { nome: t("edicola.senato"), valori: S.map((p) => p.margine_senato), tratteggiata: true }]} />
        <Grafico titolo={t("edicola.grafico_spread")} linee={[{ valori: S.map((p) => p.spread_pb) }]} formato={(v) => `${numero(v)} ${t("tracciato.pb")}`} {etichette} {voti} />
      </div>
    {/if}
  </div>
  <aside class="lato" aria-label={t("edicola.dove")}>
    {#if azioni}
      <Bozza {edizione} />
    {:else}
      <div>
        <p class="occ">{t("edicola.occhiello_albo", { scenario: edizione.scenario })}</p>
        <h3 class="tit">{edizione.nome}</h3>
      </div>
      <p class="sec muto">{t("eredita.albo_nota")}</p>
    {/if}
  </aside>
</div>

<style>
  .cr { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .pag { display: grid; gap: 12px; align-content: start; min-width: 0; min-height: 0; overflow: hidden auto }
  .gr3 { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: var(--canale); border-top: var(--doppio); padding-top: 8px }
  .lato { border: var(--bordo); padding: 12px 14px; display: grid; gap: 12px; align-content: start; min-height: 0; min-width: 0; overflow: hidden auto }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
</style>
