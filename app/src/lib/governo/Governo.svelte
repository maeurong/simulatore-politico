<script lang="ts">
  // Governo, forma B · Figurine (18-governo.prototype.html): l'album dei dieci ministri sotto la cornice, la carta del ministro a lato.
  // L'album resta fermo, il dettaglio scorre da solo; senza scelta il dettaglio mostra il primo ministro in evidenza, o il primo dell'album.
  import { tick, untrack } from "svelte";
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
    const da = document.activeElement as HTMLElement | null;
    const carta = da?.closest(".g3d");
    try {
      const ok = (await partita.comanda(c)) !== null;
      if (ok && (c.tipo === "ferma" || c.tipo === "dirotta")) {
        gira = c.dicastero;
        // un nuovo atto fermato dopo un «rigira» deve rigirare di nuovo
        volte[c.dicastero] = false;
        // il bottone premuto sparisce con l'atto e il fronte diventa inert: il fuoco passa al «rigira» della stessa carta
        await tick();
        if (da && !da.isConnected) carta?.querySelector<HTMLElement>(".retro .link")?.focus();
        // le carte lo leggono appena montate o al cambio: poi si spegne, così un rimontaggio non rigira
        setTimeout(() => (gira = null), 200);
      }
      return ok;
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
      return false;
    }
  }
  // Le carte girate «rigirate» da chi ha premuto «rigira»; e il dicastero la cui carta gira ora, dopo Ferma o Dirotta riusciti.
  let volte = $state<Record<string, boolean>>({});
  let gira = $state<string | null>(null);
  // Al turno nuovo Dirotta aperta e le carte rigirate non valgono più: l'agenda è un'altra e le carte tornano a leggere agenda.stato.
  $effect.pre(() => {
    void vista.turno;
    untrack(() => { dirotta = null; volte = {}; });
  });
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
    <span class="sec muto">{t("governo.testata", { periodo, n })}</span>
  </div>
  <div class="album" role="group" aria-label={t("governo.album")}>
    {#each carte as f (f.dicastero)}
      <Figurina {vista} {f} {grave} id="{pre}-{f.dicastero}" scelta={corrente.dicastero === f.dicastero} onscelta={() => sceglie(f.dicastero)} {manda} ondirotta={() => (dirotta = f.dicastero)} anima={gira === f.dicastero} volta={!!volte[f.dicastero]} onrigira={() => (volte[f.dicastero] = true)} />
    {/each}
  </div>
  <aside class="lato">
    {#if dirotta === corrente.dicastero && corrente.ministro && corrente.agenda?.stato === "annunciata"}
      {#key corrente.dicastero}
        <Dirotta {vista} f={corrente} {manda} {rifiuto} onchiudi={chiudiDirotta} />
      {/key}
    {:else}
      <CartaMinistro {vista} f={corrente} {grave} {manda} {rifiuto} ondirotta={() => (dirotta = corrente.dicastero)}
        anima={gira === corrente.dicastero} volta={!!volte[corrente.dicastero]} onrigira={() => (volte[corrente.dicastero] = true)} />
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
  .lato { border-left: var(--forte); padding-left: var(--canale); min-height: 0; overflow: hidden auto; padding-bottom: 8px; scroll-padding-bottom: 80px; align-content: start }
</style>
