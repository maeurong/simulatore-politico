<script module lang="ts">
  /** Riuscito il comando la pagina si rimonta (l'incarico cambia): il fuoco lo raccoglie la testa della pagina nuova. */
  export const passaggio = { fuoco: false };
</script>

<script lang="ts">
  // Le pagine della trattativa a incarico non del giocatore (27-trattativa.prototype.html, «guida» e «capo»): il ribaltone da decidere, o l'incarico ad altri.
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Risultato from "./Risultato.svelte";

  let { vista }: { vista: Vista } = $props();

  const guida = $derived(vista.incarico === "guida");
  const spento = $derived(partita.occupato ? t("trattativa.comando_in_corso") : undefined);

  // raw: il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);

  async function manda(tipo: "chiedi_guida" | "rifiuta_ribaltone") {
    if (partita.occupato) return;
    rifiuto = null;
    passaggio.fuoco = true;
    try {
      if (!(await partita.comanda({ tipo }))) passaggio.fuoco = false;
    } catch (e) {
      passaggio.fuoco = false;
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
</script>

<div class="gui">
  <p class="occ">{t(guida ? "ribaltone.occhiello" : "ribaltone.occhiello_altri")}</p>
  <p class="grande">{t(guida ? "ribaltone.titolo" : "ribaltone.titolo_altri")}</p>
  <p>{t(guida ? "ribaltone.testo" : "ribaltone.testo_altri")}</p>
  <Risultato {vista} />
  {#if guida}
    <div class="az">
      <Pulsante forma="chiude" {spento} onclick={() => manda("chiedi_guida")}>{t("ribaltone.chiedi")}</Pulsante>
      <Conferma etichetta={t("ribaltone.resta_fuori")} conferma={t("ribaltone.conferma_fuori")} {spento}
        spiegazione={t("ribaltone.spiega_fuori")} onconferma={() => manda("rifiuta_ribaltone")} />
    </div>
    {#if rifiuto && rifiuto.vista === partita.vista}<Rifiuto testo={rifiuto.testo} />{/if}
    <p class="min muto">{t("ribaltone.senza_scelta")}</p>
  {/if}
</div>

<style>
  .gui { display: grid; gap: 10px; max-width: 760px }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio) }
  .grande { font-family: var(--tit); font-variation-settings: var(--wonk); font-size: var(--c-pag); font-weight: 700; line-height: 1.1; text-wrap: balance }
  .az { display: flex; flex-wrap: wrap; gap: 6px 10px; align-items: center }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
</style>
