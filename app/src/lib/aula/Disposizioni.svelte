<script lang="ts">
  // Le disposizioni in vigore (storia 107), righe chiuse per dicastero; i gradini si aprono solo sulla riga scelta o con un valore già in bozza:
  // in vigore a fondo pieno, in bozza in giallo, in arrivo a tratteggio blu, bloccato a retino.
  import { t, type Chiave } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { perDicastero, statoGradino, type Bozza } from "./bozza";

  let { vista, bozza, aperta, onapri, oncambia }: {
    vista: Vista; bozza: Bozza; aperta: string | null; onapri: (id: string | null) => void; oncambia: (disposizione: string, gradino: string) => void;
  } = $props();

  const gruppi = $derived(perDicastero(vista));
  const nomeGradino = (d: VoceDisposizione, id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
  const periodo = (d: { anno: number; trimestre: number }) => t(`cornice.trimestre_${d.trimestre}` as Chiave, { anno: d.anno }).toLowerCase();
  const inEsame = (d: VoceDisposizione) => vista.calendario.some((m) => m.valori.some((x) => x.disposizione === d.id));
</script>

<div class="disp" role="group" aria-label={t("aula.disposizioni")}>
  {#each gruppi as g (g.nome)}
    <div class="cg"><span>{g.nome}</span></div>
    {#each g.voci as d (d.id)}
      {@const forzata = bozza.some((x) => x.disposizione === d.id)}
      {@const aperto = aperta === d.id || forzata}
      <div class="dr" class:sel={aperto}>
        <span class="nm">{d.nome}<small>{d.legge}{d.precedente !== d.in_vigore ? ` · ${t("aula.disp_prima", { gradino: nomeGradino(d, d.precedente) })}` : ""}</small></span>
        {#if aperto}
          <span class="scala" role="group" aria-label={t("aula.gradini", { nome: d.nome })}>
            {#each d.gradini as gr (gr.id)}
              {@const s = statoGradino(d, bozza, gr)}
              <button
                type="button"
                class="gr {s}"
                aria-pressed={s === "in_bozza"}
                aria-disabled={s === "bloccato" ? "true" : undefined}
                aria-label={t("aula.gradino_stato", { nome: gr.nome, stato: t(`aula.gradino_${s}` as Chiave) })}
                title={t(`aula.gradino_${s}` as Chiave)}
                onclick={() => s !== "bloccato" && oncambia(d.id, gr.id)}>{gr.nome}</button>
            {/each}
          </span>
        {:else}
          <span><b>{nomeGradino(d, d.in_vigore)}</b>{#each d.in_arrivo as a (a.gradino)} <span class="arr">{t("aula.disp_arrivo", { gradino: nomeGradino(d, a.gradino), periodo: periodo(a.data) })}</span>{/each}</span>
        {/if}
        <span class="x">
          {#if inEsame(d)}<span class="chip">{t("aula.in_esame")}</span>{/if}
          {#if !forzata}<Pulsante forma="link" onclick={() => onapri(aperto ? null : d.id)}>{aperto ? t("aula.chiudi_riga") : t(vista.governa ? "aula.cambia" : "aula.proponi")}</Pulsante>{/if}
        </span>
      </div>
    {/each}
  {/each}
</div>

<style>
  .disp { display: grid; align-content: start }
  .cg { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; margin-top: 12px; padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  .cg:first-child { margin-top: 0 }
  .dr { display: grid; grid-template-columns: minmax(150px, 24%) minmax(0, 1fr) auto; gap: 2px 12px; align-items: center; padding: 5px 4px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .dr.sel { background: var(--velo) }
  .nm { font-weight: 700; line-height: 1.2 }
  .nm small { display: block; font-weight: 400; color: var(--grigio); font-size: var(--c-min) }
  .arr { color: var(--biro); font-weight: 700; white-space: nowrap; margin-left: 8px }
  .x { display: flex; gap: 8px; align-items: center; justify-content: flex-end; font-size: var(--c-min); white-space: nowrap }
  .chip { border: 1.5px solid var(--ink); padding: 0 6px; font-weight: 700; line-height: 1.4 }
  .scala { display: flex; gap: 4px; flex-wrap: wrap }
  .gr { border: 1.5px solid var(--ink); padding: 2px 8px; font-size: var(--c-sec); white-space: nowrap; background: var(--carta) }
  .gr.in_vigore { background: var(--ink); color: var(--carta) }
  .gr.in_bozza { background: var(--giallo); color: var(--ink); box-shadow: inset 0 0 0 2px var(--ink) }
  .gr.in_arrivo { border-color: var(--biro); color: var(--biro); border-style: dashed }
  .gr.bloccato { background: var(--retino); color: var(--grigio); border-style: dashed; cursor: not-allowed }
  :global(:root.grave) .gr.in_bozza { background: var(--carta); box-shadow: inset 0 0 0 3px var(--ink) }
</style>
