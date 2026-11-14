<script lang="ts">
  // Partiti, forma A · Schieramento (24-partiti.prototype.html): l'asse, una scheda per partito; il dettaglio a lato tiene le Camere, o il partito scelto.
  import { tick } from "svelte";
  import { t, percentuale, type Chiave } from "$lib/testi/t";
  import { coloreDi, quotaVotanti } from "$lib/partiti";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VocePartito } from "$lib/tipi/VocePartito";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import Avvisi from "./Avvisi.svelte";
  import AsseDeiPartiti from "./AsseDeiPartiti.svelte";
  import Camere from "./Camere.svelte";
  import SchedaPartito from "./SchedaPartito.svelte";
  import { TETTO_PARTITI, avvisi, caselleFascia, cassaBreve, indoleBreve, inOrdine, perFila, seggiDi } from "./schieramento";

  let { vista }: { vista: Vista } = $props();

  const pre = $props.id();
  let scelta = $state<string | null>(null);
  const scelto = $derived(vista.partiti.find((p) => p.id === scelta));
  // un partito che sparisce dalla Vista non lascia la scelta appesa
  $effect(() => { if (scelta && !scelto) scelta = null; });
  async function indietro() {
    const id = scelta;
    scelta = null;
    await tick();
    document.getElementById(`${pre}-${id}`)?.focus();
  }

  const ordine = $derived(inOrdine(vista));
  const vivi = $derived(vista.partiti.filter((p) => !p.sciolto).length);
  const periodo = $derived(t(`cornice.trimestre_${vista.data.trimestre}` as Chiave, { anno: vista.data.anno }));
  const sondaggio = (p: VocePartito) => {
    const q = quotaVotanti(vista.sondaggio, vista.partiti.indexOf(p));
    return q === null ? "—" : percentuale(q, 1);
  };
  const seggi = (p: VocePartito) => {
    const c = seggiDi(vista, p.id).carica;
    return c ? `${c.camera} · ${c.senato}` : "—";
  };
  const alleato = (id: string) => vista.alleati.find((a) => a.partito === id);
  const avversario = (id: string) => vista.alleati_avversari.find((a) => a.partito === id);
</script>

<div class="par">
  <div class="col">
    <div class="capo">
      <h3 class="tit">{t("partiti.titolo")}</h3>
      <span class="sec muto">{t("partiti.testata", { periodo, n: vivi, tetto: TETTO_PARTITI })}</span>
    </div>
    <div class="asse">
      <AsseDeiPartiti partiti={ordine} giocatore={vista.giocatore} scelto={scelta} onscelta={(id) => (scelta = id)} />
      <p class="min muto">{t("partiti.asse_nota")}</p>
    </div>
    <div class="schede" style:--nc={perFila(ordine.length)}>
      {#each ordine as p (p.id)}
        {@const colore = coloreDi(p.id)}
        {@const tu = p.id === vista.giocatore}
        {@const a = alleato(p.id)}
        {@const f = avversario(p.id)}
        {@const indole = indoleBreve(p)}
        {@const lista = avvisi(vista, p)}
        <article class="sch" class:sciolto={p.sciolto} class:scelta={scelta === p.id}>
          <span class="filo" class:nato={colore === null} style:background={colore ?? undefined}></span>
          <span class="fz">{#if tu}<span class="tu" aria-hidden="true">{t("partiti.tu")}</span>{:else}<Faccia nome={p.leader} ruolo="leader" partito={p.id} misura={46} />{/if}</span>
          <h4 class="nm"><button type="button" id="{pre}-{p.id}" aria-pressed={scelta === p.id} onclick={() => (scelta = p.id)}>{tu ? t("partiti.tuo") : p.nome}</button></h4>
          <span class="ld">{tu ? t("partiti.tu_leader") : p.leader}</span>
          <span class="rr largo"><span class="muto">{t("partiti.sondaggio")}</span><b>{sondaggio(p)}</b></span>
          <span class="rr largo"><span class="muto">{t("partiti.seggi_cs")}</span><b>{seggi(p)}</b></span>
          <span class="rr largo"><span class="muto">{t("partiti.cassa")}</span><b>{cassaBreve(p.cassa)}</b></span>
          {#if !tu}<span class="ind largo">{t(indole[0])} · {t(indole[1])}</span>{/if}
          {#if a}
            <span class="largo ints"><Caselle piene={Math.round(a.intesa / 10)} su={10} rosso={a.ultimatum} nome={t("partiti.intesa", { n: Math.round(a.intesa) })} /></span>
          {:else if f}
            <span class="largo ints"><Caselle piene={caselleFascia(f.fascia)} su={3} rosso={f.fascia === "ultimatum"} nome={t("partiti.col_governo", { fascia: t(`partiti.fascia_${f.fascia}` as Chiave) })} /></span>
          {/if}
          {#if lista.length}<span class="largo"><Avvisi {lista} /></span>{/if}
        </article>
      {/each}
    </div>
  </div>
  <aside class="lato">{#if scelto}<SchedaPartito {vista} partito={scelto} {indietro} />{:else}<Camere {vista} />{/if}</aside>
</div>

<style>
  .par { display: grid; grid-template-columns: minmax(0, 1fr) var(--lato); gap: 0 var(--canale); min-height: 0 }
  .par > * { min-width: 0 }
  .col { display: grid; gap: 10px; align-content: start }
  .lato { border-left: var(--forte); padding-left: var(--canale); display: grid; gap: 8px; align-content: start }
  .muto { color: var(--grigio) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance }
  .capo { display: flex; align-items: baseline; justify-content: space-between; gap: 10px; border-bottom: var(--doppio); padding-bottom: 3px }
  .capo .tit { font-size: var(--c-sez) }
  .asse { border-bottom: var(--doppio); padding-bottom: 4px }

  .schede { display: grid; grid-template-columns: repeat(var(--nc), minmax(0, 1fr)); gap: 6px; align-items: start }
  .sch { position: relative; border: var(--bordo); padding: 0 8px 7px; display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 2px 8px; align-content: start; min-width: 0 }
  .sch > :not(.fz, .filo) { grid-column: 2 }
  .sch > .largo { grid-column: 1 / -1 }
  .filo { grid-column: 1 / -1; height: 5px; margin: 0 -8px 4px }
  .filo.nato { background: var(--retino), var(--carta); border-bottom: 1px solid var(--ink) }
  .fz { grid-row: 2 / span 2; grid-column: 1; align-self: start }
  .tu { display: inline-grid; place-items: center; width: 46px; height: 46px; border: 2px solid var(--biro); color: var(--biro); font: 800 16px var(--tit); flex: none }
  .nm button { font: inherit; text-align: left; overflow-wrap: normal }
  .nm button::after { content: ""; position: absolute; inset: 0 }
  .sch:hover, .sch.scelta { box-shadow: inset 0 0 0 1.5px var(--ink) }
  .nm { font-size: var(--c-sec); line-height: 1.15; font-weight: 700 }
  .sch.sciolto { border-style: dashed; border-color: var(--grigio); color: var(--grigio) }
  .sch.sciolto .nm { text-decoration: line-through }
  .ld { font-size: var(--c-min); color: var(--grigio) }
  .rr { display: flex; flex-wrap: wrap; justify-content: space-between; gap: 0 4px; font-size: var(--c-min) }
  .rr > * { white-space: nowrap }
  .rr b { margin-left: auto }
  .ints { font-size: var(--c-min); color: var(--grigio) }
  .ind { font-size: var(--c-min); color: var(--grigio); line-height: 1.25 }
  /* forma stretta: la faccia sta sopra e il nome ha tutta la scheda, così nessuna parola si spezza */
  :global(:root.stretta) .sch > :not(.filo) { grid-column: 1 / -1 }
  /* dieci caselle in una scheda stretta: più piccole di quelle del componente */
  .sch :global(.cas i) { width: 8px }
  .sch :global(.fascia) { flex-wrap: wrap; row-gap: 2px }
</style>
