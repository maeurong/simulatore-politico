<script lang="ts">
  // Aula, forma D · Misto (19-aula.prototype.html): il calendario dei posti in testa, sotto quattro schede (In aula, Disposizioni, Legge elettorale,
  // Referendum), la misura scelta nel dettaglio a lato. Le ultime tre schede arrivano coi Task 8 e 9. La pagina sta ferma, il corpo e il dettaglio scorrono.
  import { tick, untrack } from "svelte";
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Calendario from "./Calendario.svelte";
  import InAula from "./InAula.svelte";
  import Misura from "./Misura.svelte";

  let { vista }: { vista: Vista } = $props();

  const SCHEDE = ["in_aula", "disposizioni", "elettorale", "referendum"] as const;
  let scheda = $state<(typeof SCHEDE)[number]>("in_aula");
  let scelta = $state<number | null>(null);
  // senza scelta: il bilancio nel suo turno (occupa il calendario), altrimenti la prima misura
  const corrente = $derived(vista.calendario.find((m) => m.id === scelta) ?? vista.calendario.find((m) => m.forma === "bilancio") ?? vista.calendario[0] ?? null);
  const numeri = $derived({
    in_aula: vista.calendario.length,
    disposizioni: vista.disposizioni.filter((d) => d.norma !== "elettorale").length,
    elettorale: null,
    referendum: vista.referendum.length || null,
  });

  // al turno nuovo il calendario è un altro: la scelta di prima non vale
  $effect.pre(() => {
    void vista.turno;
    untrack(() => (scelta = null));
  });

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiutato = $state.raw<{ testo: string; tipo: Comando["tipo"]; vista: Vista | null } | null>(null);
  const rifiuto = $derived(rifiutato?.vista === vista ? rifiutato : null);
  let lato: HTMLElement;
  async function manda(c: Comando): Promise<boolean> {
    if (partita.occupato) return false;
    rifiutato = null;
    const da = document.activeElement as HTMLElement | null;
    try {
      return (await partita.comanda(c)) !== null;
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), tipo: c.tipo, vista: partita.vista };
      return false;
    } finally {
      // il bottone premuto sparisce con la scelta, o si rimonta mentre la partita è occupata: il fuoco passa alla prossima azione del dettaglio, o al dettaglio
      await tick();
      if (da && !da.isConnected && document.activeElement === document.body) (lato.querySelector<HTMLElement>(".az button, .linee button, .conf button") ?? lato).focus();
    }
  }
</script>

<div class="aula">
  <div class="capo">
    <h3 class="tit">{t("aula.titolo")}</h3>
    <span class="sec muto">{t(vista.governa ? "aula.testata_governo" : "aula.testata_opposizione")}</span>
  </div>
  <div class="corpo">
    <Calendario {vista} scelta={corrente?.id ?? null} onscelta={(id) => (scelta = id)} />
    <div class="schede" role="group" aria-label={t("aula.schede")}>
      {#each SCHEDE as s (s)}
        <button type="button" class:qui={scheda === s} aria-pressed={scheda === s} onclick={() => (scheda = s)}>
          {t(`aula.scheda_${s}`)}{#if numeri[s] !== null}<small>{numeri[s]}</small>{/if}
        </button>
      {/each}
    </div>
    {#if scheda === "in_aula"}
      <InAula {vista} scelta={corrente?.id ?? null} onscelta={(id) => (scelta = id)} />
    {:else}
      <Segnaposto sezione={t(`aula.scheda_${scheda}`)} fetta="11c" />
    {/if}
  </div>
  <aside class="lato" bind:this={lato} tabindex="-1" aria-label={t("aula.dettaglio")}>
    {#if corrente}
      {#key `${vista.turno}:${corrente.id}`}
        <Misura {vista} m={corrente} {manda} {rifiuto} />
      {/key}
    {:else}
      <p class="sec muto">{t("aula.niente_in_esame")}</p>
    {/if}
  </aside>
</div>

<style>
  .aula { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); grid-template-rows: auto minmax(0, 1fr); gap: 10px var(--canale); height: 100%; min-height: 0 }
  .aula > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .corpo { min-height: 0; overflow: hidden auto; display: grid; gap: 10px; align-content: start }
  .schede { display: flex; gap: 2px; border-bottom: var(--forte) }
  .schede button { padding: 4px 12px; font-weight: 700; color: var(--grigio); border: 1.5px solid transparent; border-bottom: 0; white-space: nowrap }
  .schede button.qui { color: var(--ink); border-color: var(--ink); background: var(--carta); margin-bottom: -2px; padding-bottom: 6px }
  .schede small { font-weight: 400; margin-left: 4px }
  .lato { border-left: var(--forte); padding-left: var(--canale); min-height: 0; overflow: hidden auto; padding-bottom: 8px; display: grid; gap: 8px; align-content: start }
  .lato:focus-visible { outline-offset: -2px }
</style>
