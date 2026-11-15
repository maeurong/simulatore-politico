<script lang="ts">
  // I cantieri del dicastero (storia 101): atto, regione, turni mancanti, rata, speso; fermarne uno costa gli euro già spesi e soddisfazione (18-governo.prototype.html, funzione cantieri).
  import { t, numero } from "$lib/testi/t";
  import { REGIONI } from "$lib/mappa/regioni";
  import type { Comando } from "$lib/tipi/Comando";
  import type { VoceMinistero } from "$lib/tipi/VoceMinistero";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import { partita } from "$lib/partita.svelte";
  import { euro } from "./governo";

  let { min, puoFermare, manda }: { min: VoceMinistero; puoFermare: boolean; manda: (c: Comando) => Promise<boolean> } = $props();
</script>

{#if min.cantieri.length}
  <div class="sez"><span>{t("governo.cantieri")}</span><span>{min.cantieri.length}</span></div>
  {#each min.cantieri as k (k.atto)}
    {@const punti = Math.round(-k.costo_ferma)}
    <div class="cant">
      <div class="sec"><b>{min.atti.find((x) => x.id === k.atto)?.nome ?? k.atto}</b>{#if k.regione} · {REGIONI.find((r) => r.id === k.regione)?.nome ?? k.regione}{/if}</div>
      <div class="min muto">{t("governo.cantiere_stato", { t: t("governo.cantiere_turni", { n: k.turni_mancanti }), rata: euro(k.rata), speso: euro(k.speso) })}</div>
      {#if puoFermare}
        <Conferma etichetta={t("governo.cantiere_ferma")} spento={partita.occupato ? t("trattativa.comando_in_corso") : undefined} conferma={t("governo.cantiere_conferma")}
          spiegazione={punti > 0 ? t("governo.cantiere_spiega", { speso: euro(k.speso), n: numero(punti) }) : t("governo.cantiere_spiega_gratis", { speso: euro(k.speso) })}
          onconferma={() => manda({ tipo: "ferma_cantiere", dicastero: min.dicastero, atto: k.atto })} />
      {/if}
    </div>
  {/each}
{/if}

<style>
  .sez { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px; margin-top: 4px; display: flex; justify-content: space-between; gap: 8px }
  .cant { display: grid; gap: 3px; justify-items: start }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
