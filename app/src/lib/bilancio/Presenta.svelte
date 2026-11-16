<script lang="ts">
  // La presentazione nel lato, senza riga scelta (prototipo 22): «Presenta» diretto, sotto cosa succede se cade, poi il rifiuto del core.
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { partita } from "$lib/partita.svelte";
  import { t } from "$lib/testi/t";
  import type { Bilancio } from "$lib/tipi/Bilancio";
  import type { Vista } from "$lib/tipi/Vista";
  import { seCade } from "./bilancio";
  import { bozza } from "./bozza.svelte";

  let { vista, bilancio }: { vista: Vista; bilancio: Bilancio } = $props();

  // raw: il rifiuto vale per il bilancio che l'ha avuto, confrontato per identità
  let rifiutato = $state.raw<{ testo: string; bilancio: Bilancio } | null>(null);
  const rifiuto = $derived(rifiutato?.bilancio === bilancio ? rifiutato.testo : null);
  const spento = $derived(
    // già presentata: un secondo Invio non la ripresenta uguale; «Modifica e ripresenta» arriva con la forma della presentata
    vista.bilancio.presentato ? t("bilancio.presentata") : partita.occupato ? t("campagna.comando_in_corso") : bozza.inVolo ? t("bilancio.presenta_in_volo") : bozza.rifiuto ? t("bilancio.presenta_non_ammessa") : undefined,
  );

  async function presenta() {
    const b = bilancio;
    rifiutato = null;
    try {
      await partita.comanda({ tipo: "presenta_bilancio", bilancio: b });
    } catch (e) {
      rifiutato = { testo: testoRifiuto(comeErrore(e)), bilancio: b };
    }
  }
</script>

<div class="presenta">
  <Pulsante forma="chiude" {spento} onclick={presenta}>{t("bilancio.presenta")}</Pulsante>
  <p class="min muto">{seCade(vista)}</p>
  {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
</div>

<style>
  .presenta { display: grid; gap: 6px; justify-items: start }
</style>
