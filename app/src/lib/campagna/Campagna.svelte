<script lang="ts">
  // Campagna, forma A · Tavolo (26-campagna.prototype.html): tre colonne e la cassa a lato. La terza colonna arriva col Task 7.
  import { untrack } from "svelte";
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio, comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { idDi, indiceDi } from "$lib/mappa/mappa";
  import { REGIONI } from "$lib/mappa/regioni";
  import SelettoreRegione from "$lib/mappa/SelettoreRegione.svelte";
  import type { Spesa } from "$lib/tipi/Spesa";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceAnteprimaCampagna } from "$lib/tipi/VoceAnteprimaCampagna";
  import Cassa from "./Cassa.svelte";
  import Coalizione from "./Coalizione.svelte";
  import Spese from "./Spese.svelte";
  import { aggiungiSpesa, quotaNelBersaglio, sessioneSpese } from "./campagna";

  let { vista }: { vista: Vista } = $props();

  // la bozza vive in memoria; all'apertura vale la Vista (la pagina si rimonta a ogni rientro)
  let spese = $state.raw<Spesa[]>(untrack(() => vista.campagna.spese));
  let voci = $state.raw<VoceAnteprimaCampagna[] | null>(null);
  // raw: il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ indice: number; testo: string; vista: Vista | null } | null>(null);
  let ultima = 0;
  let regione = $state(false);
  let scelta = $state<string | null>(null);

  /** false: la partita è occupata, il comando non parte e la sessione lo ripropone quando si libera */
  async function manda(inviate: Spesa[]) {
    if (partita.occupato) return false;
    try {
      await partita.comanda({ tipo: "campagna", spese: inviate });
    } catch (e) {
      rifiuto = { indice: ultima, testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
  const sessione = sessioneSpese(guscio.anteprimaCampagna, manda, (esito) => {
    voci = "voci" in esito ? esito.voci : null;
    rifiuto = "rifiuto" in esito ? { indice: ultima, testo: esito.rifiuto, vista: partita.vista } : null;
  });
  untrack(() => spese.length && sessione.cambia(spese, false));
  $effect(() => {
    if (!partita.occupato) sessione.riprova();
  });
  function modifica(nuove: Spesa[], indice: number) {
    ultima = indice;
    spese = nuove;
    sessione.cambia(nuove);
  }

  const valori = $derived(Object.fromEntries(vista.sondaggio.regioni.map((r, i) => [r.id, quotaNelBersaglio(vista, { regione: i }).pubblica])));
  const nomeScelta = $derived(REGIONI.find((r) => r.id === scelta)?.nome ?? "…");
  function spendi() {
    const i = scelta === null ? null : indiceDi(vista, scelta);
    if (i === null || partita.occupato) return;
    regione = false;
    scelta = null;
    modifica(aggiungiSpesa(spese, { regione: i }), spese.length);
    sessione.esci();
  }
</script>

<div class="tavolo">
  <div class="col"><h3 class="capo tit">{t("campagna.coalizione")}</h3><Coalizione {vista} /></div>
  <div class="col">
    <h3 class="capo tit">{t("campagna.spesa")}</h3>
    <Spese {vista} {spese} {voci} rifiuto={rifiuto && rifiuto.vista === partita.vista ? rifiuto : null} {modifica} esci={sessione.esci} apriRegione={() => (regione = true)} />
  </div>
  <div class="col"><h3 class="capo tit">{t("campagna.posizione")}</h3></div>
  <aside class="lato">
    {#if regione}
      <h3 class="capo tit">{t("campagna.regione_titolo")}</h3>
      <SelettoreRegione {valori} bind:scelta azione={t("campagna.spendi_in", { regione: nomeScelta })} onconferma={spendi} onannulla={() => (regione = false)} />
    {:else}
      <h3 class="capo tit">{t("campagna.cassa")}</h3>
      <Cassa {vista} {spese} />
    {/if}
  </aside>
</div>

<style>
  .tavolo { display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1.15fr) minmax(0, 1fr) var(--lato); min-height: 0 }
  .tavolo > * { min-width: 0 }
  .col { padding: 0 var(--canale) 12px 0 }
  .col + .col { border-left: var(--filo); padding-left: var(--canale) }
  .lato { border-left: var(--forte); padding-left: var(--canale) }
  .capo { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-riq); border-bottom: var(--doppio); padding-bottom: 3px; margin-bottom: 6px }
</style>
