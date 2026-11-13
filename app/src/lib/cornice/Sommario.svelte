<script lang="ts">
  // Forma dai prototipi 07 (barrate e motivi) e 08 (cornice A · Gerenza).
  import { t } from "$lib/testi/t";
  import { SEZIONI, barrata, casa, etichetta, type Pagina } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import type { Vista } from "$lib/tipi/Vista";

  type Props = {
    vista: Vista;
    /** la pagina aperta, come nell'URL */
    sezione: string;
    impostazioni: boolean;
    onimpostazioni: () => void;
  };
  let { vista, sezione, impostazioni, onimpostazioni }: Props = $props();
  const id = $props.id();

  // la casa cambia nome con lo stato (07-ossatura.prototype.html:216): al secondo posto Trattativa o Eredità,
  // che porta alla pagina della casa e lì è la voce corrente
  const c = $derived(casa(vista));
  const voci = $derived(
    SEZIONI.map((s) => {
      const nome: Pagina = s === "cruscotto" ? c : s;
      return { nome, etichetta: etichetta(nome), motivo: barrata(s, vista) };
    }),
  );
</script>

<nav class="somm" aria-label={t("sommario.etichetta")}>
  {#each voci as v (v.nome)}
    {#if v.motivo}
      <span class="barrata">
        <button type="button" class="voce chiusa" aria-disabled="true" title={t(v.motivo)} aria-describedby="{id}-{v.nome}">{v.etichetta}</button>
        <span id="{id}-{v.nome}" class="perche nascosto">{t(v.motivo)}</span>
      </span>
    {:else}
      <a class="voce" class:qui={sezione === v.nome} href="/partita/{v.nome}" aria-current={sezione === v.nome ? "page" : undefined}>{v.etichetta}</a>
    {/if}
  {/each}
  <span class="fuori">
    <a href="/partita/schema" aria-current={sezione === "schema" ? "page" : undefined}
      ><span class="lungo">{t("sommario.schema")}</span><span class="corto">{t("sommario.schema_corto")}</span></a>
    <button type="button" id="apri-impostazioni" aria-expanded={impostazioni} onclick={onimpostazioni}>{t("sommario.impostazioni")}</button>
    <a href="/" onclick={partita.esci}>{t("sommario.edicola")}</a>
  </span>
</nav>

<style>
  .somm { display: flex; align-items: end; gap: 2px; margin-top: 2px }
  .barrata { position: relative; display: flex }
  .voce {
    padding: 4px 12px 5px; border: 1.5px solid transparent; border-bottom: 0;
    font-weight: 700; color: var(--grigio); white-space: nowrap; text-decoration: none;
  }
  .voce:hover { color: var(--ink) }
  .voce.qui { color: var(--ink); border-color: var(--ink); background: var(--carta); margin-bottom: -3px; padding-bottom: 8px }
  .voce.chiusa { text-decoration: line-through; font-weight: 400; cursor: help }
  .voce.chiusa:hover { color: var(--grigio) }

  /* il motivo, fuori dal nome del bottone: .nascosto ma letto (aria-describedby); al fuoco da tastiera compare
     sotto la voce, come nel prototipo */
  .chiusa:focus-visible + .perche {
    width: auto; height: auto; overflow: visible; clip-path: none; left: 0; top: calc(100% + 2px); z-index: 3;
    background: var(--ink); color: var(--carta); font-size: var(--c-min); font-weight: 400; padding: 3px 8px;
  }

  .fuori { margin-left: auto; display: flex; gap: 14px; padding-bottom: 5px }
  .fuori > * { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec); color: var(--ink) }
  .fuori > :hover { text-decoration-thickness: 2px }
  .fuori [aria-current="page"], .fuori [aria-expanded="true"] { font-weight: 700 }
  .corto { display: none }

  /* forma stretta (meno di 1150 px utili): voci a 13 px, «Schema» */
  :global(:root.stretta) .voce { padding-left: 7px; padding-right: 7px; font-size: var(--c-sec) }
  :global(:root.stretta) .lungo { display: none }
  :global(:root.stretta) .corto { display: inline }
</style>
