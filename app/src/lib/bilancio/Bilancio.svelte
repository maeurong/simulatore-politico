<script lang="ts">
  // Bilancio, forma B · Bilancia (22-bilancio.prototype.html): il giogo in testa, entrate e spesa a fronte; a lato la presentazione o il dettaglio della riga.
  // Presentata (storia 129): le cifre in aula e il riepilogo del presentato, a lato gli emendamenti degli alleati e «Modifica e ripresenta».
  import { tick, untrack } from "svelte";
  import { page } from "$app/state";
  import Forte from "$lib/conti/Forte.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { partita } from "$lib/partita.svelte";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Giogo from "./Giogo.svelte";
  import Entrate from "./Entrate.svelte";
  import Spesa from "./Spesa.svelte";
  import DettaglioRiga from "./DettaglioRiga.svelte";
  import Presenta from "./Presenta.svelte";
  import Emendamenti from "./Emendamenti.svelte";
  import { daUrl, misuraBilancio, presentata, riepilogoPresentato } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { vista }: { vista: Vista } = $props();

  $effect(() => void bozza.apri(vista.turno));
  const ferma = $derived(presentata(vista, bozza.riaperta));
  const bilancio = $derived(ferma ? vista.bilancio.in_aula : (bozza.corrente ?? bozza.base));
  const misura = $derived(misuraBilancio(vista));

  // il riepilogo del presentato lo dice il core; una Vista nuova (un emendamento deciso) lo richiede
  let presentato = $state.raw<{ vista: Vista; r: RiepilogoBilancio | string | null } | null>(null);
  $effect(() => {
    const v = vista;
    if (v.bilancio.presentato) void riepilogoPresentato().then((r) => (presentato = { vista: v, r }));
  });
  const rp = $derived(presentato?.vista === vista ? presentato.r : null);
  const riepilogo = $derived(ferma ? (typeof rp === "object" ? rp : null) : bozza.riepilogo);

  // Accogli · Respingi, diretti; il rifiuto vale per la Vista in cui è nato, accanto all'emendamento
  let rifiutato = $state.raw<{ partito: string; testo: string; vista: Vista } | null>(null);
  const rifiutoEm = (p: string) => (rifiutato?.vista === vista && rifiutato.partito === p ? rifiutato.testo : null);
  async function emenda(partito: string, accogli: boolean) {
    if (!misura || partita.occupato) return;
    const da = document.activeElement;
    rifiutato = null;
    try {
      await partita.comanda({ tipo: "emendamento", misura: misura.id, partito, accogli });
      await tick();
      // il bottone premuto (nel lato o sul foglietto) sparisce con l'emendamento deciso: il fuoco va al lato
      if (da && da !== document.body && !da.isConnected) lato?.querySelector<HTMLElement>("button")?.focus({ preventScroll: true });
    } catch (e) {
      rifiutato = { partito, testo: testoRifiuto(comeErrore(e)), vista };
    }
  }
  const provvisorio = $derived(vista.bilancio.esercizio_provvisorio);

  /** La riga scelta: `dic:<id>`, `irpef:<i>`, `fondo`; dal link «Indicizzalo» dei Conti arriva `?dicastero=<id>`. */
  // solo all'apertura: dopo, la riga la sceglie il giocatore
  let scelta = $state<string | null>(untrack(() => daUrl(page.url.searchParams.get("dicastero"), vista)));
  const esiste = (k: string, b: NonNullable<typeof bilancio>) => {
    const [tipo, id] = k.split(":");
    return k === "fondo" || (tipo === "dic" && b.dicasteri.some((d) => d.id === id)) || (tipo === "irpef" && +id < b.aliquote.irpef.length);
  };
  let fronte = $state<HTMLElement>();
  let lato = $state<HTMLElement>();
  async function torna() {
    const da = fronte?.querySelector<HTMLElement>('[aria-pressed="true"]');
    scelta = null;
    await tick();
    (da ?? lato?.querySelector("button"))?.focus();
  }
</script>

<div class="bilancio">
  <div class="foglio">
    {#if bilancio}
      <div class="capo">
        <h3 class="tit">{t(ferma ? "bilancio.titolo_presentata" : provvisorio ? "bilancio.titolo_provvisorio" : "bilancio.titolo_bozza", { anno: bilancio.anno })}</h3>
        <span class="sec muto">{ferma ? t("bilancio.sotto_presentata") : t("bilancio.sotto_bozza", { anno: vista.bilancio.anno })}</span>
      </div>
      {#if provvisorio && !ferma}
        <p class="avviso"><Forte testo={t("bilancio.avviso_provvisorio", { anno: bilancio.anno, vecchio: vista.bilancio.anno })} vals={[t("bilancio.esercizio_provvisorio")]} /></p>
      {/if}
      {#if ferma}
        <Giogo {riepilogo} rifiuto={typeof rp === "string" ? rp : null} inVolo={false} />
      {:else}
        <Giogo {riepilogo} rifiuto={bozza.rifiuto} inVolo={bozza.inVolo} />
      {/if}
      <div class="fronte" bind:this={fronte}>
        <Entrate {vista} {bilancio} {riepilogo} {ferma} bind:scelta />
        <Spesa {vista} {bilancio} {riepilogo} {ferma} emendamenti={ferma ? (misura?.emendamenti ?? []) : []} {emenda} {rifiutoEm} bind:scelta />
      </div>
    {:else if bozza.rifiuto}
      <Giogo riepilogo={null} rifiuto={bozza.rifiuto} inVolo={false} />
    {:else if bozza.base === null && !bozza.inVolo}
      <p class="sec muto">{t("bilancio.senza_bozza")}</p>
    {/if}
  </div>
  <aside class="lato" bind:this={lato}>
    {#if bilancio && scelta && esiste(scelta, bilancio)}
      <DettaglioRiga {vista} {bilancio} {riepilogo} {ferma} {scelta} {torna} />
    {:else if bilancio}
      <!-- un solo Presenta per le due forme: dopo «Modifica e ripresenta» il fuoco passa al suo bottone -->
      {#if ferma}<Emendamenti {vista} misura={misura ?? null} inAula={bilancio} {emenda} {rifiutoEm} />{/if}
      <Presenta {vista} {bilancio} {misura} {ferma} />
      {#if !ferma}<p class="min muto">{t("bilancio.nota_giogo")}</p>{/if}
    {/if}
  </aside>
</div>

<style>
  .bilancio { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); align-items: start }
  .bilancio > * { min-width: 0 }
  .foglio { display: grid; gap: 8px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); align-self: stretch; display: grid; gap: 8px; align-content: start }
  .fronte { display: grid; grid-template-columns: minmax(0, .8fr) minmax(0, 1.4fr); gap: 0 var(--canale) }
  .fronte > :global(section) { min-width: 0; display: grid; gap: 6px; align-content: start }
  .fronte > :global(section + section) { border-left: var(--filo); padding-left: var(--canale) }
  /* l'avviso del prototipo 22: filo rosso a sinistra, niente fondo */
  .avviso { border-left: 3px solid var(--rosso); padding: 3px 8px; font-size: var(--c-sec) }

  /* i pezzi comuni a giogo, entrate, spesa e lato (prototipo 22); :where() li tiene sotto le regole di ciascun pezzo */
  .bilancio :global(:where(.capo)) { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .bilancio :global(:where(.tit)) { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  :global(:root.grave) .bilancio :global(:where(.tit)) { font-weight: 600 }
  .bilancio :global(:where(.grp)) {
    display: flex; justify-content: space-between; gap: 8px; padding-bottom: 2px; border-bottom: var(--bordo);
    font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio)
  }
  .bilancio :global(:where(.sec)) { font-size: var(--c-sec) }
  .bilancio :global(:where(.min)) { font-size: var(--c-min) }
  .bilancio :global(:where(.muto)) { color: var(--grigio) }
  .bilancio :global(:where(.male)) { color: var(--rosso) }
  .bilancio :global(:where(.bene)) { color: var(--verde) }
  .bilancio :global(:where(.link)) { text-decoration: underline; text-underline-offset: 2px; font-size: var(--c-sec) }
  .bilancio :global(:where(.link:hover)) { text-decoration-thickness: 2px }
  /* la riga scelta (prototipo 22: .barD.sel, .imp.sel) */
  .bilancio :global(:where(.sel)) { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .bilancio :global(:where(.ph)) { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 7px 11px; font-size: var(--c-sec) }
  .bilancio :global(:where(.ph b)) { color: var(--ink) }
  .lato :global(:where(.link)) { justify-self: start }
  .lato :global(:where(h4.tit)) { font-size: var(--c-sez) }
</style>
