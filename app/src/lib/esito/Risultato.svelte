<script lang="ts">
  // Il risultato del voto al posto della vignetta (spec:408): affluenza, quota e seggi per partito; la coalizione col premio in nero, il giocatore in rosso.
  import { nomeDi } from "$lib/partiti";
  import { percentuale, t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";

  let { vista }: { vista: Vista } = $props();
  const voto = $derived(vista.elezioni.at(-1));
  const coalizione = $derived(voto?.premio?.split("+") ?? []);
  const righe = $derived(voto ? voto.voti.filter((r) => r.quota > 0.005).toSorted((a, b) => b.quota - a.quota) : []);
  const massimo = $derived(righe[0]?.quota ?? 1);
</script>

{#if voto}
  <figure class="ris">
    <div class="cap"><span>{t("prima.affluenza", { quota: percentuale(voto.affluenza) })}</span><span>{t("prima.camera_senato")}</span></div>
    {#each righe as r (r.partito)}
      <div class={["r", { coal: coalizione.includes(r.partito), tu: r.partito === vista.giocatore }]}>
        <span class="nm">{nomeDi(vista, r.partito)}</span>
        <span class="bar" aria-hidden="true"><i style:width="{(r.quota / massimo) * 100}%"></i></span>
        <span class="q">{percentuale(r.quota)}</span>
        <span class="sg">{r.camera} · {r.senato}</span>
      </div>
    {/each}
    <div class="cap fine"><span>{t("prima.coalizione_nera")}</span></div>
  </figure>
{/if}

<style>
  /* le righe condividono le colonne (subgrid): il nome più lungo dà la larghezza, il partito del giocatore non si taglia */
  .ris { display: grid; grid-template-columns: auto minmax(24px, 1fr) auto auto; gap: 3px 6px; font-size: var(--c-min); border: var(--forte); padding: 6px 8px; align-content: start }
  .r { display: grid; grid-column: 1 / -1; grid-template-columns: subgrid; align-items: center }
  .nm { white-space: nowrap }
  .tu .nm { font-weight: 700 }
  .bar { height: 9px; background: var(--velo); position: relative }
  .bar i { position: absolute; inset: 0 auto 0 0; background: var(--grigio) }
  .coal .bar i { background: var(--ink) }
  .tu .bar i { background: var(--rosso) }
  .q { text-align: right; font-weight: 700 }
  .sg { text-align: right; color: var(--grigio) }
  .cap { grid-column: 1 / -1; display: flex; justify-content: space-between; color: var(--grigio); border-bottom: var(--filo); padding-bottom: 2px; margin-bottom: 2px }
  .cap.fine { border-bottom: 0; border-top: var(--filo); padding: 2px 0 0; margin: 0 }
</style>
