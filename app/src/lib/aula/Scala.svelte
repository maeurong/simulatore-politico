<script lang="ts">
  // I gradini di una disposizione: in vigore a fondo pieno, in bozza in giallo, in arrivo a tratteggio blu, bloccato a retino.
  // `spenti` (chiave `disposizione|gradino`): i gradini che il core non ammette, col motivo, tratteggiati, a retino se non sono quello in vigore.
  import { t, type Chiave } from "$lib/testi/t";
  import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";
  import { statoGradino, type Bozza } from "./bozza";

  let { d, bozza, onscegli, spenti = {} }: { d: VoceDisposizione; bozza: Bozza; onscegli: (gradino: string) => void; spenti?: Record<string, string> } = $props();
</script>

<span class="scala" role="group" aria-label={t("aula.gradini", { nome: d.nome })}>
  {#each d.gradini as gr (gr.id)}
    {@const s = statoGradino(d, bozza, gr)}
    {@const motivo = s === "in_bozza" || s === "bloccato" ? undefined : spenti[`${d.id}|${gr.id}`]}
    {@const stato = motivo !== undefined ? t("aula.gradino_spento", { motivo }) : t(`aula.gradino_${s}` as Chiave)}
    <button
      type="button"
      class="gr {s}"
      class:spento={motivo !== undefined}
      aria-pressed={s === "in_bozza"}
      aria-disabled={s === "bloccato" || motivo !== undefined ? "true" : undefined}
      aria-label={t("aula.gradino_stato", { nome: gr.nome, stato })}
      title={stato}
      onclick={() => s !== "bloccato" && motivo === undefined && onscegli(gr.id)}>{gr.nome}</button>
  {/each}
</span>

<style>
  .scala { display: flex; gap: 4px; flex-wrap: wrap }
  .gr { border: 1.5px solid var(--ink); padding: 2px 8px; font-size: var(--c-sec); white-space: nowrap; background: var(--carta) }
  .gr.in_vigore { background: var(--ink); color: var(--carta) }
  .gr.in_bozza { background: var(--giallo); color: var(--ink); box-shadow: inset 0 0 0 2px var(--ink) }
  .gr.in_arrivo { border-color: var(--biro); color: var(--biro); border-style: dashed }
  .gr.spento { border-style: dashed; cursor: not-allowed }
  .gr.spento:not(.in_vigore) { background: var(--retino); color: var(--grigio) }
  .gr.spento.in_vigore { opacity: .6 }
  .gr.bloccato { background: var(--retino); color: var(--grigio); border-style: dashed; cursor: not-allowed }
  :global(:root.grave) .gr.in_bozza { background: var(--carta); box-shadow: inset 0 0 0 3px var(--ink) }
</style>
