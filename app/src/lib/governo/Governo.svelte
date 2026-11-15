<script lang="ts">
  // Governo, forma B · Figurine (18-governo.prototype.html): l'album dei dieci ministri sotto la cornice, la carta del ministro a lato.
  // L'album resta fermo, il dettaglio scorre da solo; senza scelta il dettaglio mostra il primo ministro in evidenza, o il primo dell'album.
  import { tick } from "svelte";
  import { t, type Chiave } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import CartaMinistro from "./CartaMinistro.svelte";
  import Dirotta from "./Dirotta.svelte";
  import Figurina from "./Figurina.svelte";
  import { figurine, type Figurina as Carta } from "./governo";

  let { vista }: { vista: Vista } = $props();

  const pre = $props.id();
  let scelta = $state<string | null>(null);
  const carte = $derived(figurine(vista));
  const evidenza = (f: Carta) => f.evidenza.length > 0 || f.stati.includes("preavviso");
  const corrente = $derived(carte.find((f) => f.dicastero === scelta) ?? carte.find((f) => f.ministro && evidenza(f)) ?? carte[0]);
  const grave = $derived(vista.eventi.some((e) => e.tono === "grave"));
  const periodo = $derived(t(`cornice.trimestre_${vista.data.trimestre}` as Chiave, { anno: vista.data.anno }));
  const n = $derived(carte.filter(evidenza).length);

  // Dirotta apre a lato al posto della carta; il comando va diretto. partita.occupato fa passare un solo clic per volta.
  let dirotta = $state<string | null>(null);
  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiutato = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  const rifiuto = $derived(rifiutato?.vista === vista ? rifiutato.testo : null);
  async function manda(c: Comando): Promise<boolean> {
    if (partita.occupato) return false;
    rifiutato = null;
    try {
      return (await partita.comanda(c)) !== null;
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
      return false;
    }
  }
  const sceglie = (d: string) => { scelta = d; dirotta = null; };
  async function chiudiDirotta() {
    const d = dirotta;
    dirotta = null;
    await tick();
    document.getElementById(`${pre}-${d}`)?.focus();
  }
</script>

<div class="gov">
  <div class="capo">
    <h3 class="tit">{t(grave ? "governo.titolo_grave" : "governo.titolo")}</h3>
    <span class="sec muto">{t(vista.crisi ? "governo.testata_crisi" : "governo.testata", { periodo, n })}</span>
  </div>
  <div class="album" role="group" aria-label={t("governo.album")}>
    {#each carte as f (f.dicastero)}
      <Figurina {vista} {f} {grave} id="{pre}-{f.dicastero}" scelta={corrente.dicastero === f.dicastero} onscelta={() => sceglie(f.dicastero)} {manda} ondirotta={() => (dirotta = f.dicastero)} />
    {/each}
  </div>
  <aside class="lato">
    {#if dirotta === corrente.dicastero && corrente.ministro && corrente.agenda?.stato === "annunciata"}
      <Dirotta {vista} f={corrente} {manda} {rifiuto} onchiudi={chiudiDirotta} />
    {:else}
      <CartaMinistro {vista} f={corrente} {grave} {manda} {rifiuto} ondirotta={() => (dirotta = corrente.dicastero)} />
    {/if}
  </aside>
</div>

<style>
  .gov { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .gov > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .album { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 10px; align-content: start; min-height: 0; overflow: hidden auto }
  .lato { border-left: var(--forte); padding-left: var(--canale); min-height: 0; overflow: hidden auto; padding-bottom: 8px; align-content: start }
</style>
