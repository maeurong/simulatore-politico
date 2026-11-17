<script lang="ts">
  // Il dettaglio a lato della bozza (30-eredita.prototype.html, azioniBozza('voto')): torna a prima del voto o conferma, entrambi sul posto.
  import { goto } from "$app/navigation";
  import { comeErrore } from "$lib/guscio/chiamate";
  import { periodo } from "$lib/edicola/edicola";
  import { partita } from "$lib/partita.svelte";
  import { t } from "$lib/testi/t";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { Edizione } from "./eredita";

  let { edizione }: { edizione: Edizione } = $props();
  // il turno della chiusura che ha finito la partita: da lì si torna a prima del voto
  const turno = $derived(edizione.turni - 1);
  const voto = $derived(periodo(edizione.elezioni[0]?.data ?? edizione.fine));
  let aperta = $state<"torna" | "conferma" | null>(null);
  let rifiuto = $state<string | null>(null);

  async function torna() {
    rifiuto = null;
    try {
      await partita.riavvolgi(turno);
    } catch (e) {
      rifiuto = comeErrore(e).testo;
    }
  }

  // il nome va nell'indirizzo: l'edicola sceglie la voce nuova dell'albo come dopo l'abbandono
  async function conferma() {
    rifiuto = null;
    try {
      if (await partita.confermaEredita()) await goto(`/?albo=nuova&nome=${encodeURIComponent(edizione.nome)}`);
    } catch (e) {
      rifiuto = comeErrore(e).testo;
    }
  }
</script>

<div>
  <p class="occ">{t("eredita.bozza_occhiello")}</p>
  <h3 class="tit">{t("eredita.bozza_titolo")}</h3>
</div>
<p class="sec">{t("eredita.bozza_nota")}</p>
<div class="az">
  <Conferma etichetta={t("eredita.torna", { n: turno })} conferma={t("eredita.torna_si", { n: turno })} neutro
    spiegazione={t("eredita.torna_cancella", { n: turno, voto })}
    bind:aperta={() => aperta === "torna", (v) => (aperta = v ? "torna" : aperta === "torna" ? null : aperta)} onconferma={torna} />
  <Conferma etichetta={t("eredita.conferma")} conferma={t("eredita.conferma_si")}
    spiegazione={t("eredita.conferma_cancella")}
    bind:aperta={() => aperta === "conferma", (v) => (aperta = v ? "conferma" : aperta === "conferma" ? null : aperta)} onconferma={conferma} />
</div>
{#if rifiuto}<Rifiuto testo={rifiuto} />{/if}

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .az { display: grid; gap: 10px; justify-items: start }
</style>
