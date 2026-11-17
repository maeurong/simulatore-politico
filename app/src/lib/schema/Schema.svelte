<script lang="ts">
  // Schema dello Stato, B · Tavola per poteri (spec:402; 37-scelte-11e-schema e 29-schema). Tutti i fili all'apertura; tondi, filtro e battute arrivano con la 11f.
  // Invio (o clic) su una riga o un filo apre il dettaglio a lato; Esc lo chiude e rende il fuoco a chi l'ha aperto.
  import { tick } from "svelte";
  import { t } from "$lib/testi/t";
  import { periodoDi } from "$lib/aula/aula";
  import type { Vista } from "$lib/tipi/Vista";
  import { schema } from "./schema";
  import type { Scelta } from "./tavola";
  import Tavola from "./Tavola.svelte";
  import DettaglioSchema from "./DettaglioSchema.svelte";

  let { vista }: { vista: Vista } = $props();

  const pre = $props.id();
  const S = $derived(schema(vista));
  let scelta = $state<Scelta>(null);
  // una scelta che la Vista nuova non ha più si lascia cadere
  const aperta = $derived(scelta && (scelta.tipo === "filo" ? S.fili : S.nodi).some((x) => x.id === scelta!.id) ? scelta : null);

  async function chiudi() {
    const s = aperta;
    scelta = null;
    await tick();
    if (s) document.getElementById(`${pre}-${s.tipo === "filo" ? "f" : "n"}-${s.id}`)?.focus();
  }
  async function dalDettaglio(id: string) {
    scelta = { tipo: "filo", id };
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
  </div>
  <Tavola nodi={S.nodi} fili={S.fili} scelta={aperta} {pre} onscegli={(s) => (scelta = s)} />
  <aside class="lato">
    <DettaglioSchema {vista} nodi={S.nodi} fili={S.fili} scelta={aperta} id="{pre}-det" onfilo={dalDettaglio} onchiudi={chiudi} />
  </aside>
</div>

<style>
  .sch { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .sch > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: grid; gap: 2px; border-bottom: var(--doppio); padding-bottom: 5px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-pag) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start; min-height: 0; overflow: hidden auto; padding-bottom: 8px }
</style>
