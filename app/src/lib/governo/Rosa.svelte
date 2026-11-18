<script lang="ts">
  // La rosa dei successori (storia 102): tre candidati con faccia, capacità, posizione e partito; il clic sceglie, senza scelta entra il primo (18-governo.prototype.html, funzione rosa).
  import { t, numero } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { chiaveDi, salti } from "$lib/caricature/caricatura";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMinistero } from "$lib/tipi/VoceMinistero";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import { posizioni } from "./governo";

  let { vista, min, manda }: { vista: Vista; min: VoceMinistero; manda: (c: Comando) => Promise<boolean> } = $props();

  const ruoloDi = (x: { tecnico: boolean }) => (x.tecnico ? "tecnico" : "ministro");
  const tua = $derived(min.rosa.length ? posizioni(vista, min.rosa[0]).tua : 0);
  const salto = $derived(salti(min.rosa.map(ruoloDi), min.rosa.map((x) => ({ partito: chiaveDi(vista, x.partito), nome: x.nome, genere: x.genere }))));
</script>

{#if min.rosa.length}
  <div class="sez"><span>{t("governo.rosa")}</span><span>{min.scelta === null ? "" : t("governo.rosa_scelto")}</span></div>
  <p class="min muto">{t("governo.rosa_nota")}{min.scelta === null ? ` ${t("governo.rosa_default", { nome: min.rosa[0].nome })}` : ""}</p>
  <div class="rosa">
    {#each min.rosa as x, i (x.nome)}
      <button type="button" class="cand" class:on={min.scelta === i} aria-pressed={min.scelta === i} onclick={() => manda({ tipo: "scegli_successore", dicastero: min.dicastero, indice: i })}>
        <Faccia nome={x.nome} genere={x.genere} ruolo={ruoloDi(x)} partito={chiaveDi(vista, x.partito)} salto={salto[i]} misura={46} />
        <span class="dati">
          <b>{x.nome}</b> <small>{nomeDi(vista, x.partito)}{x.tecnico ? ` · ${t("governo.rosa_tecnico")}` : ""}</small>
          <small>{t("governo.rosa_posizione", { n: numero(x.posizione, 1), tu: numero(tua, 1) })} · {t("governo.capacita_nome", { c: x.capacita })}</small>
        </span>
      </button>
    {/each}
  </div>
{/if}

<style>
  .sez { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px; margin-top: 4px; display: flex; justify-content: space-between; gap: 8px }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .rosa { display: grid; gap: 4px }
  .cand { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 10px; align-items: center; text-align: left; padding: 4px 6px; border: 1.5px solid var(--ink); background: var(--carta); font-size: var(--c-sec) }
  .cand:hover { background: var(--velo) }
  .cand.on { background: var(--giallo) }
  :global(:root.grave) .cand.on { background: var(--ink); color: var(--carta) }
  .dati { display: grid }
  small { font-size: var(--c-min); color: var(--grigio) }
  :global(:root.grave) .cand.on small { color: inherit }
</style>
