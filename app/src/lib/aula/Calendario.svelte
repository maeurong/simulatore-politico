<script lang="ts">
  // La fila dei posti in testa all'Aula (storia 104): ogni posto dice la misura e quando si vota; un posto libero lo dice.
  // I posti vengono da Vista.posti (calendario()); nel turno del bilancio la fila è un posto solo.
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import { calendario, quando, type Posto } from "./aula";

  let { vista, scelta, onscelta }: { vista: Vista; scelta: number | null; onscelta: (id: number) => void } = $props();

  const posti = $derived(calendario(vista));
  const bilancio = $derived(posti[0]?.tipo === "bilancio");

  const etichetta = (p: Posto) =>
    p.tipo === "ordinaria" ? t("aula.posto_ordinaria", { numero: p.numero ?? 1, di: p.di ?? 1 }) : t(`aula.posto_${p.tipo}`);
  function libero(p: Posto): string {
    if (p.tipo === "decreti") return t("aula.libero_decreti");
    if (p.tipo === "opposizione") return t(vista.posto_riservato ? "aula.libero_riservato" : vista.governa ? "aula.libero_nessuna_proposta" : "aula.libero_occupato");
    return t(vista.governa ? "aula.libero_scrivi" : "aula.libero");
  }
</script>

<div class="posti" style:--n={posti.length} role="group" aria-label={t("aula.calendario")}>
  {#each posti as p, i (i)}
    {#if p.misura}
      <button type="button" class="posto" class:sel={scelta === p.misura.id} class:mio={p.misura.promotore === "giocatore"} class:bil={p.tipo === "bilancio"}
        aria-pressed={scelta === p.misura.id} onclick={() => onscelta(p.misura!.id)}>
        <span class="k">{etichetta(p)}</span>
        <b>{p.misura.nome}</b>
        <span class="s">{quando(vista, p.misura.voto, true)}</span>
      </button>
    {:else}
      <div class="posto libero" class:mio={p.tipo === "opposizione" && vista.posto_riservato}>
        <span class="k">{etichetta(p)}</span>
        <b>{libero(p)}</b>
      </div>
    {/if}
  {/each}
</div>
{#if bilancio}<p class="min muto nota">{t("aula.bilancio_nota")}</p>{/if}

<style>
  .posti { display: grid; grid-template-columns: repeat(var(--n, 6), minmax(0, 1fr)); gap: 6px }
  .posto { border: var(--bordo); padding: 5px 8px; display: grid; gap: 2px; align-content: start; min-height: 64px; min-width: 0; text-align: left }
  button.posto { cursor: pointer }
  button.posto:hover { background: var(--velo) }
  .posto.sel { box-shadow: inset 0 0 0 1.5px var(--ink); background: var(--velo) }
  .posto .k { font-size: var(--c-min); letter-spacing: .08em; text-transform: uppercase; color: var(--grigio); font-weight: 700; white-space: nowrap; overflow: hidden; text-overflow: ellipsis }
  .posto b { font-size: var(--c-sec); line-height: 1.2; overflow: hidden; display: -webkit-box; line-clamp: 2; -webkit-line-clamp: 2; -webkit-box-orient: vertical }
  .posto .s { font-size: var(--c-min); color: var(--grigio) }
  .posto.libero { border-style: dashed; border-color: var(--grigio) }
  .posto.libero b { color: var(--grigio); font-weight: 400 }
  .posto.mio { border-color: var(--rosso) }
  .posto.bil { box-shadow: inset 0 -3px 0 var(--rosso) }
  .posto.bil.sel { box-shadow: inset 0 0 0 1.5px var(--ink), inset 0 -3px 0 var(--rosso) }
  .muto { color: var(--grigio) }
  .min { font-size: var(--c-min) }
  .nota { margin-top: 3px }
</style>
