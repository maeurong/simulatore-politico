<script lang="ts">
  // La linea del giocatore su una misura: i tre bottoni, il default e il costo di ciascuna. Sta nel dettaglio dell'Aula e nel cruscotto all'opposizione.
  import type { Snippet } from "svelte";
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { costoDellaLinea } from "./linea";

  let { vista, misura, onlinea, occupato, children }: {
    vista: Vista; misura: VoceMisura; onlinea: (si: boolean | null) => void; occupato?: string; children?: Snippet;
  } = $props();

  const scelta = $derived(misura.linea_giocatore);
  const mia = $derived(misura.linee.find((l) => l.partito === vista.giocatore)?.si ?? false);
</script>

{#if misura.costo_linea}
  {@const c = misura.costo_linea}
  <div class="blocco">
    <span class="occ">{t("aula.tua_linea")}</span>
    <div class="linee">
      <Pulsante forma="piccolo" scelto={scelta === true} spento={occupato} onclick={() => onlinea(true)}>{t("aula.si_maiuscolo")}</Pulsante>
      <span class="min muto">{costoDellaLinea(vista, c.si)}</span>
      <Pulsante forma="piccolo" scelto={scelta === false} spento={occupato} onclick={() => onlinea(false)}>{t("aula.no_maiuscolo")}</Pulsante>
      <span class="min muto">{costoDellaLinea(vista, c.no)}</span>
      <Pulsante forma="piccolo" scelto={scelta === null} spento={occupato} onclick={() => onlinea(null)}>{scelta === null ? t("aula.come_da_posizione", { linea: t(mia ? "aula.si" : "aula.no") }) : t("aula.come_da_posizione_nuda")}</Pulsante>
      <span class="min muto">{costoDellaLinea(vista, c.default)}</span>
    </div>
    {@render children?.()}
  </div>
{/if}

<style>
  .blocco { border-top: var(--filo); padding-top: 6px; display: grid; gap: 4px }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--ink) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .linee { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 4px 8px; align-items: center }
</style>
