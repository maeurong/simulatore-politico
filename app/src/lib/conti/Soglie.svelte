<script lang="ts">
  // Le soglie (storia 120): le sei situazioni col righello e la distanza; le grandezze stanno nel lato, qui solo il rimando.
  import { numero, t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Righello from "./Righello.svelte";
  import { distanza } from "./conti";

  let { vista, scelta = $bindable() }: { vista: Vista; scelta: string | null } = $props();
  const accese = $derived(vista.situazioni.filter((s) => s.accesa).length);
</script>

<section>
  <div class="capo"><h3 class="tit">{t("conti.soglie")}</h3><span class="sec muto">{t("conti.accese", { n: accese })}</span></div>
  {#each vista.situazioni as s, i (i)}
    {@const k = `sit:${i}`}
    <button type="button" class="sit" class:on={s.accesa} class:sel={scelta === k} aria-pressed={scelta === k} onclick={() => (scelta = k)}>
      <span class="n">{s.nome}</span>
      <span class="v">{#if s.accesa}<span class="chip pieno">{t("conti.chip_accesa")}</span>{/if} <b>{numero(s.valore, 1)}{s.misura === "crescita_pil" ? "%" : ""}</b></span>
      <span class="riga"><Righello {s} /></span>
      <span class="riga min muto">{distanza(s)}</span>
    </button>
  {/each}
  <button type="button" class="r" class:sel={scelta === "grandezze"} aria-pressed={scelta === "grandezze"} onclick={() => (scelta = "grandezze")}>
    <span class="n"><b>{t("conti.grandezze", { n: vista.grandezze.length })}</b> <small>{t("conti.grandezze_nota")}</small></span><span class="x">›</span>
  </button>
</section>

<style>
  .sit { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; width: 100%; padding: 5px 4px; border-bottom: var(--filo) }
  .sit:hover { background: var(--velo) }
  .sit.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .n { font-weight: 700; font-size: var(--c-sec) }
  .on .n { color: var(--rosso) }
  .v { white-space: nowrap }
  .riga { grid-column: 1 / -1; display: block }
  .r { margin-top: 6px }
</style>
