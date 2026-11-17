<script lang="ts">
  // «Torna al turno N ›» con la conferma sul posto (25-registro.prototype.html, torna()): dice cosa si cancella, poi riapre la prima pagina ferma.
  import { untrack } from "svelte";
  import { goto } from "$app/navigation";
  import { comeErrore } from "$lib/guscio/chiamate";
  import { casa } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import { t } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { testoConferma, type Guasto, type Turno } from "./registro";

  let { turno, vista, turni }: { turno: number; vista: Vista; turni: (Turno | Guasto)[] } = $props();

  let aperta = $state(false);
  let rifiuto = $state<string | null>(null);
  // il numero lo sa l'indice della partita: letto quando la conferma si apre, e se non si legge la frase sul numero cade
  let riavvolgimenti = $state<number | null>(null);
  $effect(() => {
    if (!aperta) return;
    let vivo = true;
    untrack(() => partita.indice()).then(
      (i) => vivo && (riavvolgimenti = i?.riavvolgimenti ?? null),
      () => vivo && (riavvolgimenti = null),
    );
    return () => (vivo = false);
  });

  async function torna() {
    rifiuto = null;
    try {
      const nuova = await partita.riavvolgi(turno);
      // al turno zero la prima pagina è barrata: si apre la casa (lettura 3)
      if (nuova) await goto(nuova.turno === 0 ? `/partita/${casa(nuova)}` : "/partita/prima-pagina", { keepFocus: true });
    } catch (e) {
      rifiuto = comeErrore(e).testo;
    }
  }
</script>

<div class="riav">
  <Conferma bind:aperta etichetta={t("registro.torna", { n: turno })} conferma={t("registro.torna_si", { n: vista.turno - turno })}
    spiegazione={testoConferma(turno, vista, turni, riavvolgimenti)} spento={partita.occupato ? t("registro.in_corso") : undefined} onconferma={torna} />
  {#if rifiuto}<Rifiuto testo={rifiuto} />{/if}
</div>

<style>
  .riav { display: grid; gap: 4px; justify-items: end }
  /* la frase che dice cosa si cancella si legge prima di confermare: corpo pieno, in inchiostro, a tutta riga */
  .riav :global(.spieg) { flex-basis: 100%; font-size: var(--c-sec); color: var(--ink); max-width: 78ch }
</style>
