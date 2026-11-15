<script lang="ts">
  // Cruscotto del turno zero, forma A · Ordine del giorno (17-cruscotto.prototype.html): gli eventi in testa, poi le due scelte del voto.
  import { t, elenco, numero } from "$lib/testi/t";
  import { etichetta } from "$lib/ossatura";
  import type { Vista } from "$lib/tipi/Vista";
  import Evento from "./Evento.svelte";
  import Riga from "./Riga.svelte";
  import { righeDelVoto, risponde } from "./cruscotto";

  let { vista }: { vista: Vista } = $props();
  const voto = $derived(righeDelVoto(vista));
  const { dichiarati, disponibili } = $derived(voto.coalizione);
  const { spese, euro } = $derived(voto.campagna);
  const stato = $derived(
    dichiarati.length
      ? elenco(dichiarati, ", ")
      : `${t("cruscotto.coalizione_nessuna")} · ${t("cruscotto.disponibili", { nomi: disponibili.length ? elenco(disponibili, ", ") : t("cruscotto.nessun_disponibile") })}`,
  );
  const rimando = $derived(`${etichetta("campagna")} ›`);
</script>

{#each vista.eventi as evento, i (i)}<Evento {evento} indice={i} puoRispondere={risponde(vista)} />{/each}
<section class="odg">
  <h2 class="grp">{t("cruscotto.da_decidere")}</h2>
  <ul>
    <Riga titolo={t("cruscotto.coalizione")} {stato} {rimando} href="/partita/campagna" fatta={dichiarati.length > 0} urgente />
    <Riga
      titolo={t("cruscotto.campagna")}
      stato={spese ? t("cruscotto.spese", { n: spese, euro: numero(euro) }) : t("cruscotto.nessuna_spesa")}
      {rimando} href="/partita/campagna" fatta={spese > 0} />
  </ul>
</section>

<style>
  .grp { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
</style>
