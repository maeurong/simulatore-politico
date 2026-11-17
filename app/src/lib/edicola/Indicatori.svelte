<script lang="ts">
  // La tabella degli indicatori contro il turno zero. Una sola fonte per il dettaglio d'albo e l'eredità.
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import { numero, t } from "$lib/testi/t";
  import type { IndicatoreAlbo } from "$lib/tipi/IndicatoreAlbo";
  import { maleSe } from "./edicola";

  let { indicatori }: { indicatori: IndicatoreAlbo[] } = $props();
</script>

<table class="ind">
  <thead><tr><th>{t("edicola.indicatore")}</th><th class="r">{t("edicola.turno_zero")}</th><th class="r">{t("edicola.fine")}</th><th><span class="nascosto">{t("edicola.scarto")}</span></th></tr></thead>
  <tbody>
    {#each indicatori as i (i.id)}
      {@const decimali = i.id === "spread" ? 0 : 1}
      <tr>
        <td class="muto">{i.nome}</td>
        <td class="r">{numero(i.turno_zero, decimali)}</td>
        <td class="r"><b>{numero(i.fine, decimali)}</b></td>
        <!-- ponytail: lo scarto in colonna sua (13-edicola.prototype.html:115), col valore vuoto -->
        <td class="r"><NumeroFreccia valore={i.fine} prima={i.turno_zero} formato={() => ""} formatoVariazione={(x) => numero(x, decimali)} male={maleSe(i.id)} /></td>
      </tr>
    {/each}
  </tbody>
</table>

<style>
  .muto { color: var(--grigio) }
  .ind { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .ind th { font-size: var(--c-min); color: var(--grigio); letter-spacing: .06em; text-transform: uppercase; font-weight: 400; text-align: left; padding: 0 0 3px }
  .ind td { padding: 2px 0 }
  .ind td + td, .ind th + th { padding-left: var(--pad-ind, 12px) }
  .ind .r { text-align: right; white-space: nowrap }
  .ind th.r { white-space: normal }
</style>
