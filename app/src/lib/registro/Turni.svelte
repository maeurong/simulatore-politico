<script lang="ts">
  // La colonna dei turni (25-registro.prototype.html, forma D): il più recente in alto, i segni di criticità, tono grave e voto.
  import { periodo } from "$lib/edicola/edicola";
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import { periodoDel, type Guasto, type Turno } from "./registro";

  export type Riga = { turno: Turno | Guasto; passanti: number };
  type Props = { righe: Riga[]; scelto: number | undefined; filtrato: boolean; caricando: boolean; vista: Vista; onscelta: (turno: number) => void };
  let { righe, scelto, filtrato, caricando, vista, onscelta }: Props = $props();
</script>

<nav class="turni" aria-label={t("registro.colonna")}>
  <h3 class="tit">{t("registro.titolo")}</h3>
  {#each righe as { turno, passanti } (turno.turno)}
    <button type="button" class={["tn", { sel: turno.turno === scelto }]} aria-current={turno.turno === scelto ? "true" : undefined} onclick={() => onscelta(turno.turno)}>
      <b>{t("registro.turno", { n: turno.turno })}</b>
      {#if "voci" in turno}
        <span class="segni">
          {#if turno.critiche}<i class="mk" title={t("registro.c_critica")}></i><span class="nascosto">{t("registro.critiche", { n: turno.critiche })}</span>{/if}
          {#if turno.grave}<span class="grave">{t("registro.grave")}</span>{/if}
          {#if turno.voto}<span class="voto">{t("registro.voto")}</span>{/if}
        </span>
        <span class="meta">{periodo(periodoDel(turno.turno, vista))}</span>
        <span class="meta">{filtrato ? t("registro.voci_di", { k: passanti, n: turno.voci.length }) : t("registro.voci", { n: turno.voci.length })}</span>
      {:else}
        <span></span>
        <span class="meta">{periodo(periodoDel(turno.turno, vista))}</span>
        <span class="meta">{t("registro.non_si_legge")}</span>
      {/if}
    </button>
  {:else}
    {#if filtrato && !caricando}<p class="vuoto">{t("registro.nessun_turno")}</p>{/if}
  {/each}
  {#if caricando}<p class="vuoto" role="status">{t("registro.carico")}</p>{/if}
</nav>

<style>
  .turni { margin-left: calc(-1 * var(--s2)); border-right: var(--forte); min-height: 0; overflow: hidden auto; padding-bottom: 10px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq); padding: 0 var(--s4) 4px var(--s2); border-bottom: var(--doppio) }
  .tn { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 8px; width: 100%; padding: 7px var(--s4) 7px var(--s2); border-bottom: var(--filo); align-items: center; text-align: left }
  .tn:hover { background: var(--velo) }
  .tn.sel { background: var(--ink); color: var(--carta) }
  .tn.sel .meta { color: var(--carta) }
  .tn b { font-size: var(--c-corpo) }
  .segni { display: flex; gap: 4px; align-items: center; justify-content: end }
  .meta { font-size: var(--c-min); color: var(--grigio) }
  .mk { width: 9px; height: 9px; background: var(--rosso); flex: none }
  :global(:root.grave) .mk { background: var(--ink) }
  .tn.sel .mk { background: var(--carta) }
  .grave, .voto { font-size: var(--c-min); font-weight: 700; padding: 0 4px; white-space: nowrap }
  .grave { background: var(--ink); color: var(--carta) }
  .voto { border: 1.5px solid currentColor }
  .tn.sel .grave { background: var(--carta); color: var(--ink) }
  .vuoto { font-size: var(--c-sec); color: var(--grigio); padding: 8px var(--s4) 0 var(--s2) }
</style>
