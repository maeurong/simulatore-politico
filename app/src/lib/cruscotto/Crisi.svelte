<script lang="ts">
  // Il riquadro di testa del governo che cade (17-cruscotto.prototype.html, `crisi`): prima dell'evento. La conseguenza si legge prima della conferma
  // (docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48); perdere il governo porta all'opposizione, non alla fine (docs/ricerca/estetica/tabellone-e-carta-ritagliata.md:113).
  // L'allargamento riusa `fuori` e `allarga` della trattativa. La banda nera la mette già la cornice.
  import { tick, untrack } from "svelte";
  import { t, elenco } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import { allarga, fuori } from "$lib/trattativa/trattativa";
  import type { Vista } from "$lib/tipi/Vista";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { testa } from "./cruscotto";

  let { vista, tipo }: { vista: Vista; tipo: NonNullable<ReturnType<typeof testa>> } = $props();

  const disponibilita = $derived(fuori(vista));
  let spuntati = $state<string[]>([]);
  // al turno nuovo le spunte di prima non valgono; `turno` è un derived: una Vista nuova dello stesso turno non le tocca
  const turno = $derived(vista.turno);
  $effect.pre(() => {
    void turno;
    untrack(() => (spuntati = []));
  });
  const nomi = $derived(spuntati.flatMap((id) => disponibilita.find((x) => x.id === id)?.nome ?? []));
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const metti = (id: string) => (spuntati = spuntati.includes(id) ? spuntati.filter((x) => x !== id) : [...spuntati, id]);

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  async function manda(comando: Parameters<typeof partita.comanda>[0]) {
    if (partita.occupato) return;
    rifiuto = null;
    const da = document.activeElement;
    try {
      await partita.comanda(comando);
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
    // il fuoco passa al riquadro solo se il bottone su cui stava sparisce; mai a un'altra azione, dove un secondo Invio farebbe un altro comando
    await tick();
    if (da && !da.isConnected) radice?.focus({ preventScroll: true });
  }
  const chi = $derived(elenco(nomi, ", "));
  const id = $props.id();
  let radice = $state<HTMLElement>();
</script>

<section class="crisi" aria-labelledby="{id}-t" tabindex="-1" bind:this={radice}>
  {#if tipo === "dimissioni"}
    <div class="occ">{t("cruscotto.dimissioni_occ")}</div>
    <h2 class="tit" id="{id}-t">{t("cruscotto.dimissioni_tit")}</h2>
    <div class="sec muto">{t("cruscotto.dimissioni_sotto")}</div>
  {:else if tipo === "crisi_provvisorio"}
    <div class="occ">{t("cruscotto.crisi_prov_occ")}</div>
    <h2 class="tit" id="{id}-t">{t("cruscotto.crisi_prov_tit")}</h2>
    <div class="sec">{t("cruscotto.crisi_prov_sotto")}</div>
    <div class="min muto">{t("cruscotto.crisi_prov_resta")}</div>
  {:else}
    <div class="occ">{t("cruscotto.crisi_occ")}</div>
    <h2 class="tit" id="{id}-t">{t("cruscotto.crisi_tit")}</h2>
    <div class="sec muto">{t("cruscotto.crisi_sotto")}</div>
    <ul class="part">
      {#each disponibilita as x (x.id)}
        <li>
          {#if x.disponibile}
            <Pulsante forma="piccolo" scelto={spuntati.includes(x.id)} onclick={() => metti(x.id)}>{x.nome}</Pulsante>
            <span class="min">{x.causa}</span>
          {:else}
            <span class="no">{x.nome}</span>
            <span class="min muto">· {x.causa}</span>
          {/if}
        </li>
      {/each}
    </ul>
    <div class="az">
      {#if nomi.length}
        {#key spuntati.join("+")}
          <Conferma neutro etichetta={t("cruscotto.allarga_a", { chi })} conferma={t("trattativa.conferma_allarga")} spento={occupato}
            spiegazione={t("cruscotto.allarga_spiega", { n: nomi.length, chi })} onconferma={() => manda(allarga(spuntati))} />
        {/key}
      {:else}
        <Pulsante forma="bordo" spento={t("cruscotto.allarga_scegli")}>{t("cruscotto.allarga")}</Pulsante>
      {/if}
      <Conferma etichetta={t("cruscotto.dimettiti")} conferma={t("cruscotto.dimettiti_conferma")} spento={occupato}
        spiegazione={t("cruscotto.dimettiti_spiega")} onconferma={() => manda({ tipo: "dimettiti" })} />
    </div>
    {#if rifiuto?.vista === vista}<Rifiuto testo={rifiuto.testo} />{/if}
  {/if}
</section>

<style>
  .crisi:focus { outline: none }
  .crisi { border: 3px solid var(--rosso); padding: 10px 14px; display: grid; gap: 6px }
  :global(:root.grave) .crisi { border: var(--doppio) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .part { list-style: none; display: grid; gap: 4px }
  .part li { display: flex; gap: 4px 8px; align-items: center; flex-wrap: wrap }
  .no { border: 1.5px dashed var(--grigio); color: var(--grigio); padding: 2px 8px; font-size: var(--c-sec) }
  .az { display: flex; gap: 6px 14px; align-items: center; flex-wrap: wrap }
</style>
