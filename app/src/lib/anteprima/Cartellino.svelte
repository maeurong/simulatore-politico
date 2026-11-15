<script lang="ts">
  // Il cartellino (ticket 20, forma D): un foglio appeso a uno spillo nel dettaglio a lato, col prima e dopo. Oscilla solo a un gesto:
  // all'apertura, al cambio di gradino (`valori`) e all'acquisto del test; mai in loop, mai nel tono grave o col movimento spento.
  import { untrack } from "svelte";
  import { numero, segnato, t, elenco, type Chiave } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { inOrdine } from "$lib/partiti/schieramento";
  import { partita } from "$lib/partita.svelte";
  import { fattori, impostazioni, sistema } from "$lib/impostazioni.svelte";
  import { formaDi } from "$lib/aula/aula";
  import { euro } from "$lib/governo/governo";
  import { REGIONI } from "$lib/mappa/regioni";
  import Mappa from "$lib/mappa/Mappa.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Fascia3 } from "$lib/tipi/Fascia3";
  import type { ValoreProposto } from "$lib/tipi/ValoreProposto";
  import type { Vista } from "$lib/tipi/Vista";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import Emiciclo from "$lib/componenti/Emiciclo.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { cosaCambia, perimetro, quando, stimatore, type Misurabile } from "./anteprima";
  import { oscilla } from "./oscilla";

  let { vista, misura, valori, indietro, ontorna, manda, rifiuto, fuoco = false }: {
    vista: Vista; misura: Misurabile; valori: ValoreProposto[]; indietro: string; ontorna: (e: MouseEvent) => void;
    manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null; fuoco?: boolean;
  } = $props();

  const nome = (id: string) => nomeDi(vista, id);
  const righe = $derived(cosaCambia(vista, valori));
  const grandezza = (id: string) => (vista.grandezze.find((g) => g.id === id)?.nome ?? id).replace(/ \(.*\)$/, "");
  const fascia = (f: Fascia3) => t(`anteprima.fascia_${f}` as Chiave);
  const piene = (f: Fascia3) => ({ poco: 1, medio: 2, molto: 3 })[f];
  const spesa = $derived(misura.costo_annuo === 0 ? t("anteprima.saldo_zero") : `${misura.costo_annuo > 0 ? "+" : "−"}${euro(Math.abs(misura.costo_annuo))}`);
  const chi = $derived(stimatore(vista, misura.proponente));
  const regioni = $derived(perimetro(vista, valori));
  const prezzo = $derived(vista.acquisti.prezzi.find((p) => p.acquisto === "test_misura")?.euro);
  const cassa = $derived(vista.partiti.find((p) => p.id === vista.giocatore)?.cassa ?? 0);
  const euroEsatti = (c: number) => `${numero(c)} €`;
  const test = $derived(vista.acquisti.test_misura?.misura === misura.nome ? vista.acquisti.test_misura : null);
  const camera = $derived(vista.seggi.reduce((s, x) => s + x.camera, 0));
  const senato = $derived(vista.seggi.reduce((s, x) => s + x.senato, 0));
  const partiti = $derived(inOrdine(vista));
  const votano = (si: boolean) => elenco(partiti.filter((p) => (misura.linee.find((l) => l.partito === p.id)?.si ?? false) === si).map((p) => p.nome), ", ");
  const nota = $derived(misura.forma === "costituzionale" ? t("aula.regola_costituzionale") : misura.forma === "conversione" ? t("anteprima.nota_decreto") : misura.forma === "elettorale" ? t("anteprima.nota_elettorale") : "");
  const vuoto = $derived(t(misura.forma === "elettorale" ? "anteprima.nessun_effetto_elettorale" : misura.forma === "costituzionale" ? "anteprima.nessun_effetto_costituzionale" : "anteprima.nessun_effetto"));
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  const moto = $derived(fattori(impostazioni, sistema.riduce));
  let foglio = $state<HTMLElement>();
  let corrente: Animation | null = null;
  let prima: string | null = null;
  $effect(() => {
    const chiave = `${JSON.stringify(valori)}|${test ? "test" : ""}`;
    untrack(() => {
      if (foglio && chiave !== prima) {
        corrente?.cancel();
        corrente = oscilla(foglio, moto.muove ? moto.k : 0);
      }
      if (prima === null && fuoco) foglio?.querySelector("button")?.focus({ preventScroll: true });
      prima = chiave;
    });
  });
</script>

<div class="cart" bind:this={foglio}>
  <i class="spillo" aria-hidden="true"></i>
  <div class="capo"><Pulsante forma="link" onclick={ontorna}>{indietro}</Pulsante></div>
  <span class="occ">{t("anteprima.occhiello", { forma: formaDi({ forma: misura.forma, deliberazione: "deliberazione" in misura ? misura.deliberazione : 1 }) })}</span>
  <h4 class="tit">{misura.nome}</h4>

  <table class="led">
    <thead><tr><th scope="col">{t("anteprima.cosa_cambia")}</th><th scope="col">{t("anteprima.ora")}</th><th scope="col">{t("anteprima.con_la_misura")}</th></tr></thead>
    <tbody>
      {#each righe as r (r.disposizione)}
        <tr><td>{r.disposizione}</td><td class="muto">{r.ora}</td><td class="n">{r.con}</td></tr>
      {/each}
      <tr><td>{t("anteprima.spesa")}</td><td class="muto">—</td><td class="n">{spesa}</td></tr>
    </tbody>
  </table>

  {#if misura.effetti.length}
    <table class="led">
      <thead><tr><th scope="col">{t("anteprima.effetti")}</th><th scope="col">{t("anteprima.quando")}</th></tr></thead>
      <tbody>
        {#each misura.effetti as e (e.grandezza)}
          <tr>
            <td class={e.migliora ? "bene" : "male"}>
              <span class="ink">{grandezza(e.grandezza)}</span>
              <Caselle piene={piene(e.fascia)} su={3} rosso={!e.migliora} nome={t(e.migliora ? "anteprima.migliora" : "anteprima.peggiora", { fascia: fascia(e.fascia) })} />
            </td>
            <td class="muto">{quando(e.ritardo)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {:else}
    <p class="sec muto">{vuoto}</p>
  {/if}
  <p class="min muto">{chi ? t("anteprima.stima_di", { nome: chi.nome, dicastero: chi.dicastero, capacita: chi.capacita }) : t("anteprima.stima_neutra")}</p>

  {#if test}
    <table class="led">
      <thead><tr><th scope="col" colspan="3" class="tito">{t("anteprima.test_titolo")}</th></tr>
        <tr><th scope="col">{t("anteprima.test_gruppo")}</th><th scope="col">{t("anteprima.test_toccato")}</th><th scope="col">{t("anteprima.test_reazione")}</th></tr></thead>
      <tbody>
        {#each test.reazioni as r (r.condizione)}
          <tr>
            <td>{t(`condizione.${r.condizione}` as Chiave)}</td><td class="muto">{misura.gruppi.includes(r.condizione) ? t("aula.si") : ""}</td>
            <td class="n {r.segno > 0 ? 'bene' : r.segno < 0 ? 'male' : 'muto'}">
              {#if r.segno}<Caselle piene={piene(r.fascia)} su={3} rosso={r.segno < 0} nome={t(r.segno > 0 ? "anteprima.a_favore" : "anteprima.contro")} />{:else}{t("anteprima.indifferenti")}{/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
    {#if prezzo !== undefined}<p class="min muto">{t("anteprima.test_nota", { prezzo: euroEsatti(prezzo), cassa: euroEsatti(cassa) })}</p>{/if}
  {:else}
    <div class="blocco">
      <span class="occ nero">{t("anteprima.gruppi")}</span>
      <div>{#each misura.gruppi as g (g)}<span class="chip">{t(`condizione.${g}` as Chiave)}</span>{:else}<span class="sec muto">{t("anteprima.gruppi_nessuno")}</span>{/each}</div>
      <p class="min muto">{t("anteprima.gruppi_nota")}</p>
      {#if prezzo !== undefined}
      <div class="conf">
        <Pulsante forma="piccolo" spento={occupato} onclick={() => manda({ tipo: "compra", acquisto: { test_misura: { valori } } })}>{t("anteprima.compra_test", { prezzo: euroEsatti(prezzo) })}</Pulsante>
        <span class="min muto">{t("anteprima.cassa_test", { cassa: euroEsatti(cassa) })}</span>
      </div>
      {/if}
      {#if rifiuto?.tipo === "compra"}<Rifiuto testo={rifiuto.testo} />{/if}
    </div>
  {/if}

  {#if regioni.length}
    <div class="dove">
      <div class="mappa"><Mappa valori={Object.fromEntries(REGIONI.map((r) => [r.id, regioni.includes(r.id) ? 1 : 0]))} ordine={REGIONI.map((r) => r.id)} scelta={null} nome={t("anteprima.mappa")} piccola solaLettura
        formato={(x) => t(x ? "anteprima.vale" : "anteprima.non_vale")} onscelta={() => {}} /></div>
      <div>
        <span class="occ nero">{t("anteprima.dove", { n: regioni.length })}</span>
        <p class="sec">{elenco(regioni.map((id) => REGIONI.find((r) => r.id === id)?.nome ?? id), ", ")}</p>
      </div>
    </div>
  {/if}

  <div class="blocco">
    <span class="occ nero">{t("anteprima.voto")}</span>
    <div class="emicicli">
      <Emiciclo ramo="Camera" totale={camera} si={misura.forbice.camera} maggioranza={misura.forbice.maggioranza_camera} />
      <Emiciclo ramo="Senato" totale={senato} si={misura.forbice.senato} maggioranza={misura.forbice.maggioranza_senato} />
    </div>
    {#if misura.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
    <p class="sec">
      {#if votano(true)}{t("anteprima.linea_si", { nomi: votano(true) })}{/if}
      {#if votano(false)}{t("anteprima.linea_no", { nomi: votano(false) })}{/if}
    </p>
  </div>

  {#if misura.forma === "elettorale" && misura.seggi.length}
    <div class="blocco">
      <span class="occ nero">{t("aula.seggi_titolo")}</span>
      <div class="sg">
        <span class="h">{t("aula.col_partito")}</span><span class="h n">{t("edicola.camera")}</span><span class="h n">{t("edicola.senato")}</span>
        {#each partiti as p (p.id)}
          {@const x = misura.seggi.find((s) => s.partito === p.id)}
          <span>{p.nome}</span>
          <span class="n {(x?.camera ?? 0) > 0 ? 'bene' : (x?.camera ?? 0) < 0 ? 'male' : 'muto'}">{segnato(x?.camera ?? 0)}</span>
          <span class="n {(x?.senato ?? 0) > 0 ? 'bene' : (x?.senato ?? 0) < 0 ? 'male' : 'muto'}">{segnato(x?.senato ?? 0)}</span>
        {/each}
      </div>
    </div>
  {/if}
  {#if nota}<p class="min muto">{nota}</p>{/if}
</div>

<style>
  .cart { position: relative; border: var(--forte); background: var(--carta); padding: 14px 14px 12px; margin: 30px 10px 0; display: grid; grid-template-columns: minmax(0, 1fr); gap: 9px; align-content: start; min-width: 0;
    transform-origin: 50% -20px; box-shadow: 3px 4px 0 rgb(31 26 23 / .16) }
  .cart::before { content: ""; position: absolute; left: 50%; top: -20px; width: 1.5px; height: 20px; background: var(--ink) }
  .spillo { position: absolute; left: calc(50% - 6px); top: -27px; width: 13px; height: 13px; border-radius: 50%; background: var(--rosso); border: 1.5px solid var(--ink) }
  :global(:root.grave) .cart { box-shadow: none }
  .capo { justify-self: start }
  .occ { overflow-wrap: anywhere; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .occ.nero { color: var(--ink) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .bene { color: var(--verde) }
  .male { color: var(--rosso) }
  .led { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .led th { text-align: left; font-size: var(--c-min); letter-spacing: .08em; text-transform: uppercase; color: var(--grigio); font-weight: 700; padding: 8px 6px 2px 0; border-bottom: var(--bordo) }
  .led th.tito { color: var(--ink) }
  .led th:nth-child(n + 2), .led td:nth-child(n + 2) { text-align: right }
  .led th[colspan] { text-align: left }
  .led td { overflow-wrap: anywhere; padding: 4px 6px 4px 0; border-top: var(--filo); vertical-align: top }
  .led td.n { font-weight: 700 }
  .led td .ink { display: block; color: var(--ink); font-weight: 400 }
  .led td :global(.fascia) { font-weight: 700 }
  .blocco { border-top: var(--filo); padding-top: 6px; display: grid; gap: 4px }
  .conf { display: flex; gap: 6px; align-items: center; flex-wrap: wrap }
  .chip { display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; margin: 0 4px 2px 0; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip { justify-self: start }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .emicicli { display: grid; grid-template-columns: 1.5fr 1fr; gap: 8px; align-items: end }
  .dove { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 12px; align-items: start; border-top: var(--filo); padding-top: 6px }
  .mappa :global(svg.piccola) { height: 170px }
  .sg { display: grid; grid-template-columns: minmax(0, 1fr) 60px 60px; gap: 2px 8px; font-size: var(--c-sec); align-items: center }
  .sg .h { font-size: var(--c-min); color: var(--grigio); text-transform: uppercase; letter-spacing: .06em }
  .sg .n { text-align: right; font-weight: 700 }
</style>
