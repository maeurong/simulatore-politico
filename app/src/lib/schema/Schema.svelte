<script lang="ts">
  // Schema dello Stato, B · Tavola per poteri (spec:402; 37-scelte-11e-schema e 29-schema). Tutti i fili all'apertura; coi tondi, il filtro «Solo quelli in prima pagina» (11f).
  // Invio (o clic) su una riga o un filo apre il dettaglio a lato; Esc lo chiude e rende il fuoco a chi l'ha aperto.
  import { tick } from "svelte";
  import { t } from "$lib/testi/t";
  import { periodoDi } from "$lib/aula/aula";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { schema, soloInPrimaPagina, tondiDi } from "./schema";
  import type { Scelta } from "./tavola";
  import Tavola from "./Tavola.svelte";
  import DettaglioSchema from "./DettaglioSchema.svelte";

  let { vista }: { vista: Vista } = $props();

  const pre = $props.id();
  const tutto = $derived(schema(vista));
  const T = $derived(tondiDi(vista, tutto));
  const filtra = $derived(T.fili.size + T.nodi.size > 0);
  let solo = $state(false);
  // lo schema si apre su tutti i fili (ticket 29): a turno nuovo il filtro si spegne
  $effect(() => {
    void vista.turno;
    solo = false;
  });
  const S = $derived(solo && filtra ? soloInPrimaPagina(tutto, T) : tutto);
  let scelta = $state<Scelta>(null);
  // una scelta che la Vista nuova (o il filtro) non ha più si lascia cadere
  const aperta = $derived(scelta && (scelta.tipo === "filo" ? S.fili : S.nodi).some((x) => x.id === scelta!.id) ? scelta : null);
  // e si azzera: se il filo ricompare un turno dopo, il dettaglio non si riapre da solo
  $effect(() => {
    if (!aperta) scelta = null;
  });

  // chi ha aperto il dettaglio dall'elenco delle notizie: Esc gli rende il fuoco; dalla tavola il fuoco torna alla riga o al filo
  let apritore: HTMLElement | null = null;
  async function chiudi() {
    const s = aperta, da = apritore;
    scelta = null;
    apritore = null;
    await tick();
    if (da?.isConnected) da.focus();
    else if (s) document.getElementById(`${pre}-${s.tipo === "filo" ? "f" : "n"}-${s.id}`)?.focus();
  }
  async function dalDettaglio(s: Scelta, da?: HTMLElement) {
    scelta = s;
    apritore = da ?? null;
    await tick();
    document.getElementById(`${pre}-det`)?.focus();
  }
  function tasto(e: KeyboardEvent) {
    if (e.key === "Escape" && !e.defaultPrevented && aperta) { e.preventDefault(); chiudi(); }
  }
</script>

<svelte:window onkeydown={tasto} />

<div class="sch">
  <div class="capo">
    <h3 class="tit">{t("schema.testa", { turno: vista.turno })}</h3>
    <span class="sec muto">{t("schema.sottotitolo", { periodo: periodoDi(vista, vista.data) })}</span>
    {#if filtra}
      <div class="filtro">
        <Pulsante forma="piccolo" scelto={!solo} onclick={() => (solo = false)}>{t("schema.tutti_i_fili", { n: tutto.fili.length })}</Pulsante>
        <Pulsante forma="piccolo" scelto={solo} onclick={() => (solo = true)}>{t("schema.solo_prima", { n: T.fili.size })}</Pulsante>
      </div>
    {/if}
  </div>
  <Tavola nodi={S.nodi} fili={S.fili} tutti={tutto.fili} tondi={T} scelta={aperta} {pre} onscegli={(s) => ((scelta = s), (apritore = null))} />
  <aside class="lato">
    <DettaglioSchema {vista} nodi={S.nodi} fili={S.fili} tutti={tutto.fili} tondi={T} scelta={aperta} id="{pre}-det" onapri={dalDettaglio} onchiudi={chiudi} />
  </aside>
</div>

<style>
  .sch { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .sch > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: grid; grid-template-columns: 1fr auto; gap: 2px var(--canale); border-bottom: var(--doppio); padding-bottom: 5px }
  .capo > span { grid-column: 1 }
  .filtro { grid-column: 2; grid-row: 1 / span 2; align-self: end; display: flex; gap: 8px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-pag) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start; min-height: 0; overflow: hidden auto; padding-bottom: 8px }
</style>
