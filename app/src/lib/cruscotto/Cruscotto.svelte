<script lang="ts">
  // Cruscotto, forma A · Ordine del giorno (17-cruscotto.prototype.html): gli eventi in testa; al turno zero le due scelte del voto, al governo i gruppi con il conto al default.
  import { tick } from "svelte";
  import { t, elenco, numero, type Chiave } from "$lib/testi/t";
  import { etichetta } from "$lib/ossatura";
  import type { Vista } from "$lib/tipi/Vista";
  import Evento from "./Evento.svelte";
  import Riga from "./Riga.svelte";
  import Voce from "./Voce.svelte";
  import { ordineDelGiorno, righeDelVoto, risponde, type Gruppo } from "./cruscotto";

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

  const governo = $derived(risponde(vista));
  const odg = $derived(ordineDelGiorno(vista));
  const GRUPPI: [Gruppo, Chiave][] = [
    ["da_decidere", "cruscotto.da_decidere"], ["proposte", "cruscotto.proposte"], ["emendamenti", "cruscotto.emendamenti"],
    ["si_vota", "cruscotto.si_vota"], ["occhio", "cruscotto.occhio"],
  ];
  const gruppi = $derived(GRUPPI.map(([g, titolo]) => ({ titolo, voci: odg.filter((x) => x.gruppo === g) })).filter((g) => g.voci.length));
  let corpo: HTMLElement | undefined = $state();
  // se la voce su cui stava il fuoco sparisce con la sua risposta, il fuoco passa alla prima azione che resta; altrimenti non si tocca
  async function riporta(da: Element | null) {
    await tick();
    if (!da || da.isConnected) return;
    (corpo?.querySelector<HTMLElement>('.a button:not([aria-disabled="true"]), .a a') ?? corpo)?.focus({ preventScroll: true });
  }
</script>

{#each vista.eventi as evento, i (i)}<Evento {vista} {evento} indice={i} puoRispondere={governo} />{/each}
{#if governo}
  <div class="odg" bind:this={corpo} tabindex="-1">
    {#each gruppi as g (g.titolo)}
      {@const n = g.voci.filter((x) => x.alDefault).length}
      <section>
        <h2 class="grp"><span>{t(g.titolo)}</span>{#if n}<span>{t("cruscotto.al_default", { n })}</span>{/if}</h2>
        <ul>{#each g.voci as voce (voce.chiave)}<Voce {voce} {vista} {riporta} />{/each}</ul>
      </section>
    {/each}
    {#if !gruppi.length && !vista.eventi.length}<p class="muto">{t("cruscotto.nessuna_scelta")}</p>{/if}
  </div>
{:else}
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
{/if}

<style>
  .odg { outline: none }
  div.odg { display: grid; gap: 10px }
  .muto { color: var(--grigio) }
  .grp { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); padding-bottom: 2px; border-bottom: 1.5px solid var(--ink) }
  ul { list-style: none }
</style>
