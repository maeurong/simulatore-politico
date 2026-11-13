<script lang="ts">
  // Il dettaglio di una partita in corso (13-edicola.prototype.html:237-254).
  import { dataRelativa, t } from "$lib/testi/t";
  import type { VocePartita } from "$lib/tipi/VocePartita";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { azioneDi, periodo } from "./edicola";

  type Props = {
    voce: VocePartita;
    ora: Date;
    occupato: boolean;
    inVolo: boolean;
    /** il rifiuto del guscio nato da questa scheda */
    rifiuto: string | null;
    onriprendi: () => void;
    onabbandona: () => void;
  };
  let { voce, ora, occupato, inVolo, rifiuto, onriprendi, onabbandona }: Props = $props();

  const testata = $derived(voce.indice.testata);
  const azione = $derived(azioneDi(voce));
  const occhiello = $derived(
    t(azione === "eredita" ? "edicola.occhiello_bozza" : azione === "nessuna" ? "edicola.occhiello_altra_versione" : "edicola.occhiello_in_corso"),
  );
</script>

<div>
  <p class="occ">{occhiello}</p>
  <h2 class="tit">{testata.nome}</h2>
</div>
<dl class="dati">
  <dt>{t("edicola.campo_scenario")}</dt><dd>{testata.scenario}</dd>
  <dt>{t("edicola.campo_turno")}</dt><dd><b>{testata.turno}</b> · {periodo(testata.data)}</dd>
  <dt>{t("edicola.campo_salvata")}</dt><dd>{dataRelativa(voce.indice.ultimo_salvataggio, ora)}</dd>
  <dt>{t("edicola.campo_riavvolgimenti")}</dt><dd>{voce.indice.riavvolgimenti}</dd>
  <dt>{t("edicola.campo_schema")}</dt><dd>{testata.versione_schema}</dd>
</dl>
{#if azione === "nessuna"}
  <p class="sec male">{voce.motivo}.</p>
  <p class="min muto">{t("edicola.nota_altra_versione")}</p>
{:else if azione === "eredita"}
  <p class="sec">{t("edicola.nota_bozza")}</p>
{/if}
<div class="az">
  {#if azione === "nessuna"}
    <span class="min muto">{t("edicola.non_si_apre")}</span>
  {:else if inVolo}
    <span class="min muto" role="status">{t("edicola.abbandono_in_corso")}</span>
  {:else if azione === "eredita"}
    <Pulsante forma="bordo" aria-busy={occupato} aria-disabled={occupato ? "true" : undefined} onclick={onriprendi}>{t("edicola.apri_eredita")}</Pulsante>
  {:else}
    <Pulsante forma="chiude" aria-busy={occupato} aria-disabled={occupato ? "true" : undefined} onclick={onriprendi}>{t("edicola.riprendi")}</Pulsante>
    <Conferma forma="link" etichetta={t("edicola.abbandona")} conferma={t("edicola.conferma_abbandona")}
      spiegazione={t("edicola.nota_abbandono")} onconferma={onabbandona} />
  {/if}
</div>
{#if rifiuto}<Rifiuto testo={rifiuto} />{/if}

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .dati { display: grid; grid-template-columns: auto 1fr; gap: 3px 12px; font-size: var(--c-sec); align-items: baseline }
  .dati dt { color: var(--grigio) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .male { color: var(--rosso) }
  .az { display: flex; gap: 6px; flex-wrap: wrap; align-items: center }
  .az :global([aria-busy="true"]) { cursor: progress }
</style>
