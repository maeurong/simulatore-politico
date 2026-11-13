<script lang="ts">
  // L'asse della forma A · Banco (15-fondazione.prototype.html:236-243): istogramma sui sette punti, i partiti sotto,
  // i sei mezzi punti come radio veri (una fermata di Tab, le frecce del sistema), nascosti sotto i loro tasti.
  import { numero, percentuale, t } from "$lib/testi/t";
  import type { AnteprimaFondazione } from "$lib/tipi/AnteprimaFondazione";
  import type { PartitoCatalogo } from "$lib/tipi/PartitoCatalogo";
  import { fraTesto } from "./fondazione";

  type Props = { anteprima: AnteprimaFondazione; partiti: PartitoCatalogo[]; posto: number | null; etichetta: string };
  let { anteprima, partiti, posto = $bindable(), etichetta }: Props = $props();

  const massimo = $derived(Math.max(...anteprima.istogramma));
  const ordinati = $derived([...partiti].sort((a, b) => a.posizione - b.posizione));
  // i due punti interi accanto al mezzo punto scelto
  const vicini = $derived(posto === null ? [] : [Math.floor(posto), Math.ceil(posto)]);
</script>

<div class="asse" role="radiogroup" aria-labelledby={etichetta}>
  {#each anteprima.istogramma as q, i (i)}
    <div class={["bar", { vic: vicini.includes(i + 1) }]} aria-hidden="true">
      <span class="q">{percentuale(q)}</span><i style:height="{(q / massimo) * 180}px"></i>
    </div>
  {/each}
  {#each anteprima.posizioni as p (p.posizione)}
    <label class={["slot", { qui: p.posizione === posto }]} style:left="{((p.posizione - 0.5) / 7) * 100}%">
      <input type="radio" class="nascosto" name="posto" value={p.posizione} bind:group={posto}
        aria-label="{numero(p.posizione, 1)}, {fraTesto(p.posizione, partiti)}: {percentuale(p.quota_attesa)}" />
      <span class="tasto" aria-hidden="true">{percentuale(p.quota_attesa)}<small>{numero(p.posizione, 1)}</small></span>
    </label>
  {/each}
</div>
<div class="sotto" aria-hidden="true">
  {#each ordinati as p (p.id)}<div><b>{p.nome}</b><span>{numero(p.posizione)}</span></div>{/each}
</div>
<p class="estremi"><span>{t("fondazione.estrema_sinistra")}</span><span>{t("fondazione.nota_asse")}</span><span>{t("fondazione.estrema_destra")}</span></p>

<style>
  .asse { position: relative; height: 230px; display: grid; grid-template-columns: repeat(7, minmax(0, 1fr)); align-items: end; border-bottom: var(--forte) }
  .bar { display: flex; flex-direction: column; justify-content: flex-end; align-items: center; height: 100%; padding: 0 18% }
  .bar i { display: block; width: 100%; background: var(--grigio); opacity: .55 }
  .bar.vic i { background: var(--ink); opacity: 1 }
  .bar .q { font-size: var(--c-min); color: var(--grigio); margin-bottom: 2px }
  .bar.vic .q { color: var(--ink); font-weight: 700 }

  .slot { position: absolute; top: 0; bottom: 0; width: 0; border-left: 1.5px dashed var(--grigio); cursor: pointer }
  .slot.qui { border-left: 2.5px solid var(--rosso) }
  .tasto {
    position: absolute; top: 0; transform: translateX(-50%); border: 1.5px solid var(--ink); background: var(--carta);
    padding: 2px 7px; font-size: var(--c-sec); font-weight: 700; white-space: nowrap; line-height: 1.15; text-align: center;
  }
  .tasto small { display: block; font-weight: 400; font-size: var(--c-min); color: var(--grigio) }
  .slot:hover .tasto { background: var(--velo) }
  .slot.qui .tasto { background: var(--rosso); border-color: var(--rosso); color: var(--carta) }
  .slot.qui .tasto small { color: var(--carta) }
  /* il radio è nascosto: il fuoco si vede sul suo tasto */
  .slot:has(:focus-visible) .tasto { outline: 2px solid var(--biro); outline-offset: 2px }

  .sotto { display: grid; grid-template-columns: repeat(7, minmax(0, 1fr)); text-align: center; font-size: var(--c-min); padding-top: 3px }
  /* le colonne sono alte quanto la più alta: il numero sta in fondo, e i sette restano in riga anche se un nome va a capo */
  .sotto div { display: flex; flex-direction: column }
  .sotto b { font-size: var(--c-sec) }
  .sotto span { color: var(--grigio); margin-top: auto }
  .estremi { display: flex; justify-content: space-between; gap: var(--s3); font-size: var(--c-min); color: var(--grigio); margin-top: 2px }
</style>
