<script lang="ts">
  // Paese, forma A · Atlante (23-paese.prototype.html): i livelli, la mappa e la lista; il dettaglio a lato tiene il sondaggio.
  import { t, percentuale } from "$lib/testi/t";
  import type { Chiave } from "$lib/testi/t";
  import { coalizioneDi, coloreDi } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import Mappa from "$lib/mappa/Mappa.svelte";
  import FormaSegno from "$lib/mappa/FormaSegno.svelte";
  import type { Segno } from "$lib/mappa/Mappa.svelte";
  import { classi, estremi } from "$lib/mappa/mappa";
  import Sondaggio from "./Sondaggio.svelte";
  import { LIVELLI, affluenzaPerRegione, consensoPerRegione, lista, livelloManca, segniPerRegione, type Livello } from "./paese";

  let { vista }: { vista: Vista } = $props();
  const pre = $props.id();

  let livello = $state<Livello>("consenso");
  let chi = $state("coalizione");
  let ordine = $state<"nome" | "valore">("nome");
  let scelta = $state<string | null>(null);

  const valori = $derived(livello === "affluenza" ? affluenzaPerRegione(vista) : consensoPerRegione(vista, chi === "coalizione" ? coalizioneDi(vista) : [chi]));
  const segni = $derived(segniPerRegione(vista));
  const righe = $derived(lista(valori, ordine));
  const classe = $derived(Object.fromEntries(righe.map((r, i) => [r.id, classi(righe.map((x) => x.valore))[i]])));
  const limiti = $derived(estremi(righe.map((r) => r.valore)));
  const tipi = $derived([...new Set(Object.values(segni).flat())] as Segno[]);
  const nomeLivello = (l: Livello) => t(`paese.livello_${l}` as Chiave);
  const pc = (x: number) => percentuale(x, 1);
</script>

<div class="atlante">
  <div class="col">
    <div class="livelli" role="group" aria-label={t("paese.livelli")}>
      {#each LIVELLI as l (l)}
        {@const manca = livelloManca(l)}
        {#if manca}
          <span class="barrata">
            <button type="button" class="chiusa" aria-disabled="true" title={t("paese.livello_manca", { fetta: manca })} aria-describedby="{pre}-{l}">{nomeLivello(l)}</button>
            <span id="{pre}-{l}" class="perche nascosto">{t("paese.livello_manca", { fetta: manca })}</span>
          </span>
        {:else}
          <button type="button" class:qui={livello === l} aria-pressed={livello === l} onclick={() => (livello = l)}>{nomeLivello(l)}</button>
        {/if}
      {/each}
      <span class="sub">
        {#if livello === "consenso"}
          <label>{t("paese.di")}
            <select bind:value={chi}>
              <option value="coalizione">{t("paese.coalizione")}</option>
              {#each vista.partiti.filter((p) => !p.sciolto) as p (p.id)}<option value={p.id}>{p.nome}</option>{/each}
            </select>
          </label>
          <span class="muto">· {t("paese.nota_consenso")}</span>
        {:else}<span class="muto">{t("paese.nota_affluenza")}</span>{/if}
      </span>
    </div>

    <div class="carta">
      <div class="mappa">
        <Mappa {valori} {segni} {scelta} ordine={righe.map((r) => r.id)} nome={t("mappa.regioni")} formato={pc} onscelta={(id) => (scelta = id)} />
      </div>
      <div class="lista" role="group" aria-label={t("mappa.regioni")}>
        <div class="h">
          <button type="button" class:on={ordine === "nome"} aria-pressed={ordine === "nome"} onclick={() => (ordine = "nome")}>{t("paese.col_regione")}</button>
          <button type="button" class="dx" class:on={ordine === "valore"} aria-pressed={ordine === "valore"} onclick={() => (ordine = "valore")}>{nomeLivello(livello)}</button>
          <span class="dx">{t("paese.col_segni")}</span>
        </div>
        {#each righe as r (r.id)}
          <button type="button" class="r" class:sel={scelta === r.id} aria-pressed={scelta === r.id} onclick={() => (scelta = r.id)}>
            <span class="n"><i style:background={classe[r.id] === null ? "var(--carta)" : `var(--classe-${classe[r.id]})`}></i>{r.nome}</span>
            <span class="x">{r.valore === null ? "—" : pc(r.valore)}</span>
            <span class="s">
              {#each segni[r.id] ?? [] as s, i (i)}
                <svg width="12" height="12" viewBox="-7 -7 14 14" aria-label={t(`mappa.segno_${s}`)} role="img"><FormaSegno {s} /></svg>
              {/each}
            </span>
          </button>
        {/each}
      </div>
    </div>

    <div class="legenda">
      {#if limiti}
        <span class="cl">{#each [0, 1, 2, 3, 4] as c (c)}<i style:background="var(--classe-{c})"></i>{/each}</span>
        <span>{pc(limiti.min)} · {t("paese.fino_a", { x: pc(limiti.max) })}</span>
      {:else}<span>{t("mappa.senza_dato")}</span>{/if}
      {#each tipi as s (s)}
        <span class="sg"><svg width="12" height="12" viewBox="-7 -7 14 14" aria-hidden="true"><FormaSegno {s} /></svg>{t(`mappa.segno_${s}`)}</span>
      {/each}
    </div>
  </div>

  <aside class="lato"><Sondaggio {vista} /></aside>
</div>

<style>
  .atlante { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); min-height: 0 }
  .atlante > * { min-width: 0 }
  .col { display: grid; gap: 8px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start }
  .muto { color: var(--grigio) }

  .livelli { display: flex; flex-wrap: wrap; align-items: end; border-bottom: var(--forte) }
  .livelli button { padding: 4px 11px; font-weight: 700; color: var(--grigio); border: 1.5px solid transparent; border-bottom: 0; font-size: var(--c-sec) }
  .livelli button:hover { color: var(--ink) }
  .livelli button.qui { color: var(--ink); border-color: var(--ink); background: var(--carta); margin-bottom: -2px; padding-bottom: 6px }
  .livelli button.chiusa { text-decoration: line-through; font-weight: 400; cursor: help }
  .livelli button.chiusa:hover { color: var(--grigio) }
  .barrata { position: relative; display: flex }
  .chiusa:focus-visible + .perche, .barrata:hover .perche {
    width: auto; height: auto; overflow: visible; clip-path: none; left: 0; top: calc(100% + 2px); z-index: 3;
    background: var(--ink); color: var(--carta); font-size: var(--c-min); font-weight: 400; padding: 3px 8px;
  }
  .sub { margin-left: auto; display: flex; gap: 6px; align-items: center; padding-bottom: 4px; font-size: var(--c-sec) }
  select { font: inherit; font-size: var(--c-sec); border: var(--bordo); background: var(--carta); padding: 1px 4px }

  .carta { display: grid; grid-template-columns: minmax(0, 1fr) minmax(250px, 290px); gap: 0 18px; align-items: start }
  .mappa { height: clamp(380px, 68vh, 640px) }
  .lista { display: grid; align-content: start; font-size: var(--c-sec) }
  .h, .r { display: grid; grid-template-columns: minmax(0, 1fr) 64px 58px; gap: 6px; align-items: center; padding: 2px 4px }
  .h { font-size: var(--c-min); color: var(--grigio); font-weight: 700; border-bottom: var(--bordo) }
  .h button.on { color: var(--ink); text-decoration: underline }
  .dx { text-align: right }
  .r { border-bottom: var(--filo); line-height: 1.25 }
  .r:hover { background: var(--velo) }
  .r.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .r .n { display: flex; gap: 6px; align-items: center; min-width: 0; overflow-wrap: normal }
  .r .n i { width: 10px; height: 10px; border: 1px solid var(--ink); flex: none }
  .x { text-align: right; font-weight: 700; white-space: nowrap }
  .s { display: flex; gap: 2px; justify-content: flex-end }

  .legenda { display: flex; gap: 10px; align-items: center; flex-wrap: wrap; font-size: var(--c-min); color: var(--grigio) }
  .cl { display: flex }
  .cl i { width: 34px; height: 10px; border: 1px solid var(--ink); border-right: 0 }
  .cl i:last-child { border-right: 1px solid var(--ink) }
  .sg { display: inline-flex; align-items: center; gap: 3px }
</style>
