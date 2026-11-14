<script lang="ts">
  // La casa del turno in trattativa (ticket 27). Incarico al giocatore: risultato, spartizione e lato; guida e Capo dello Stato arrivano col Task 18.
  import { t, type Chiave } from "$lib/testi/t";
  import { etichetta } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Evento from "$lib/cruscotto/Evento.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Alleati from "./Alleati.svelte";
  import Risultato from "./Risultato.svelte";
  import Spartizione from "./Spartizione.svelte";
  import { sessioneSpartizione, type EsitoManda, type Stato } from "./trattativa";

  let { vista }: { vista: Vista } = $props();

  // la bozza vive in memoria: al rientro vale ciò che dice la Vista (spec:250). raw: il rifiuto vale per la Vista in cui è nato.
  let stato = $state.raw<Stato>({ s: { bozza: {}, rifiuto: null, esito: null }, vista: null });

  /** «occupato»: la sessione tiene la bozza pendente e la rimanda quando la partita si libera. */
  async function manda(comando: Parameters<typeof partita.comanda>[0]): Promise<EsitoManda> {
    if (partita.occupato) return "occupato";
    try {
      return (await partita.comanda(comando)) ? "ok" : "guasto";
    } catch (e) {
      return { rifiuto: testoRifiuto(comeErrore(e)) };
    }
  }
  const sessione = sessioneSpartizione(() => vista, manda, (s) => (stato = { s, vista: partita.vista }));
  $effect(() => {
    vista.richieste;
    sessione.aggiorna();
  });
  $effect(() => {
    if (!partita.occupato) sessione.riprova();
  });

  const periodo = $derived(t(`cornice.trimestre_${vista.data.trimestre}` as Chiave, { anno: vista.data.anno }));
</script>

{#if vista.incarico === "giocatore"}
  <div class="trt">
    <div class="capo">
      <h3 class="tit">{t("trattativa.titolo")}</h3>
      <span class="sec muto">{t("cornice.periodo_turno", { periodo, turno: vista.turno })} · {t("trattativa.incarico_a_te")}</span>
    </div>
    <div class="pag">
      {#if vista.eventi.length}<p class="evt">{t("trattativa.nessuno_risponde")}</p>{/if}
      {#each vista.eventi as evento, i (i)}<Evento {evento} indice={i} puoRispondere={false} />{/each}
      <Risultato {vista} />
      <Spartizione {vista} {stato} {sessione} />
    </div>
    <aside class="lato">
      <Alleati {vista} bozza={stato.s.bozza} {sessione} />
    </aside>
  </div>
{:else}
  <Segnaposto sezione={etichetta("trattativa")} fetta="11b" />
{/if}

<style>
  .trt { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 10px var(--canale); align-content: start }
  .trt > * { min-width: 0 }
  .capo { grid-column: 1 / -1; display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  h3.tit { font-size: var(--c-riq) }
  .pag { display: grid; gap: 10px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .evt { font-size: var(--c-sec); border-left: 3px solid var(--rosso); padding-left: 8px }
  :global(:root.grave) .evt { border-color: var(--ink) }
</style>
