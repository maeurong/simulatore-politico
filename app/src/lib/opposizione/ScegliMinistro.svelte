<script lang="ts">
  // «Scegli il ministro ›»: i dicasteri col ministro, sottolineati in rosso dove la sfiducia passerebbe (28-opposizione.prototype.html, `.dic`).
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import { dicasteriConMinistro, type Anteprime } from "./sfiducia";

  let { vista, scelto, anteprime, onscegli }: { vista: Vista; scelto: string | null; anteprime: Anteprime; onscegli: (dicastero: string) => void } = $props();
  const voci = $derived(dicasteriConMinistro(vista).map((id) => ({
    id, nome: vista.dicasteri.find((d) => d.id === id)?.nome ?? id, ministro: vista.ministeri.find((m) => m.dicastero === id)?.ministro?.nome ?? "",
  })));
</script>

<div class="dic" role="group" aria-label={t("opposizione.leva_sfiducia_ministro")}>
  {#each voci as d (d.id)}
    {@const passa = anteprime.get(d.id)?.passa}
    <button type="button" class:passa aria-pressed={scelto === d.id} title={d.ministro} onclick={() => onscegli(d.id)}>{d.nome}</button>
  {/each}
</div>
<p class="min muto">{t("opposizione.ministro_scegli_nota")}</p>

<style>
  .dic { display: flex; flex-wrap: wrap; gap: 4px }
  .dic button { border: 1.5px solid var(--ink); padding: 1px 7px; font-size: var(--c-min); font-weight: 700 }
  .dic button[aria-pressed="true"] { background: var(--ink); color: var(--carta) }
  .dic button.passa { box-shadow: inset 0 -3px 0 var(--rosso) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
