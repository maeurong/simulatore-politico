<script lang="ts">
  // La bozza della legge nel dettaglio a lato (storia 108): i valori scelti con «ora …» accanto, l'anteprima (una in volo per volta, vince l'ultima),
  // il bottone della forma, il rifiuto del core sul posto col rimando alla misura in esame se il testo la nomina. «Anteprima della misura ›» apre
  // il cartellino al posto della bozza, come Misura.svelte: cambiato un gradino, oscilla.
  import { tick, untrack } from "svelte";
  import { t, numero } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { guscio } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import type { Anteprima } from "$lib/tipi/Anteprima";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Cartellino from "$lib/anteprima/Cartellino.svelte";
  import { forbice, rigaEffetto, spesa, votano } from "$lib/anteprima/anteprima";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { riportaFuoco } from "./aula";
  import { motivoCrisi } from "$lib/cruscotto/cruscotto";
  import { comandoBozza, etichettaBozza, forma, leggiAnteprima, misuraNominata, numeraRichieste, type Bozza } from "./bozza";

  let { vista, bozza, scheda, manda, rifiuto, ontoglie, onsvuota, onapri }: {
    vista: Vista; bozza: Bozza; scheda: "disposizioni" | "elettorale"; manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null;
    ontoglie: (disposizione: string, gradino: string) => void; onsvuota: () => void; onapri: (misura: number) => void;
  } = $props();

  const f = $derived(forma(vista, bozza));
  const etichetta = $derived(etichettaBozza(vista, bozza));
  const voci = $derived(bozza.flatMap((x) => {
    const d = vista.disposizioni.find((y) => y.id === x.disposizione);
    return d ? [{ d, nuovo: d.gradini.find((g) => g.id === x.gradino)?.nome ?? x.gradino, ora: d.gradini.find((g) => g.id === d.in_vigore)?.nome ?? d.in_vigore }] : [];
  }));
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : motivoCrisi(vista));
  const nominata = $derived(rifiuto && (rifiuto.tipo === "presenta_legge" || rifiuto.tipo === "presenta_proposta") ? misuraNominata(vista, rifiuto.testo) : null);

  // una anteprima in volo per volta: chi cambia gradino in fretta lascia l'ultima, e una risposta vecchia non copre la nuova
  let ante = $state.raw<Anteprima | null>(null);
  // la bozza a cui `ante` risponde: il cartellino mostra quella, non la bozza già cambiata che aspetta la sua risposta
  let anteDi = $state.raw<Bozza>([]);
  let errore = $state<string | null>(null);
  let inCalcolo = $state(false);
  const richieste = numeraRichieste();
  const nominataAnte = $derived(errore ? misuraNominata(vista, errore) : null);
  const chiedi = ultimoVince(
    ({ b }: { b: Bozza; n: number }) => (vista.governa ? guscio.anteprima(b) : guscio.anteprimaProposta(b)),
    async (r, { b, n }) => {
      if (!richieste.vale(n)) return;
      inCalcolo = false;
      const letta = leggiAnteprima(r);
      ({ ante, errore } = letta);
      if (letta.guasto) partita.guasto = letta.guasto;
      anteDi = b;
      // un rifiuto chiude il cartellino e lo azzera: la correzione dopo non lo riapre; il fuoco, se stava lì, passa al dettaglio
      if (!ante && apertoCartellino) {
        const da = document.activeElement;
        const lato = da?.closest<HTMLElement>("aside");
        apertoCartellino = false;
        await tick();
        if (lato && da && !da.isConnected) riportaFuoco(lato, da);
      }
    },
  );
  $effect(() => {
    const b = bozza;
    void vista; // una Vista nuova (un comando riuscito) cambia ciò che il core risponde, anche a bozza ferma
    const n = richieste.nuova();
    untrack(() => {
      if (!b.length) {
        ante = null;
        errore = null;
        inCalcolo = false;
        return;
      }
      inCalcolo = true;
      chiedi({ b, n });
    });
  });

  // il cartellino vale finché c'è una bozza con la sua anteprima; svuotata la bozza si richiude
  let apertoCartellino = $state(false);
  let piede = $state<HTMLElement>();
  let daFuoco = $state(false);
  $effect(() => {
    if (!bozza.length) apertoCartellino = false;
  });
  // il fuoco si sposta solo se stava sul bottone che sparisce: in WKWebView un clic non lo dà
  async function scambia(e: MouseEvent, apri: boolean) {
    const aveva = e.currentTarget === document.activeElement;
    daFuoco = aveva;
    apertoCartellino = apri;
    if (aveva && !apri) {
      await tick();
      piede?.querySelector("button")?.focus({ preventScroll: true });
    }
  }

  const ramo = (si: [number, number], n: number) => t("aula.ante_forbice", { si: forbice(si), n: numero(n) });
  const senzaChevron = (chiave: "aula.cambia" | "aula.proponi") => t(chiave).replace(/ ›$/, "");
</script>

{#snippet rimando(m: VoceMisura)}
  <Pulsante forma="link" onclick={() => onapri(m.id)}>{t("aula.apri_misura", { nome: m.nome })}</Pulsante>
{/snippet}

{#if f && etichetta && apertoCartellino && ante}
  <Cartellino {vista} misura={ante} valori={anteDi} {inCalcolo} indietro={t("anteprima.torna_alla_bozza")} ontorna={(e) => scambia(e, false)} {manda} {rifiuto} fuoco={daFuoco} />
{:else if !f || !etichetta}
  <div class="bozza vuota sec">{scheda === "elettorale" ? t("aula.bozza_vuota_elettorale") : t("aula.bozza_vuota", { azione: senzaChevron(vista.governa ? "aula.cambia" : "aula.proponi") })}</div>
{:else}
  <section class="bozza" aria-label={t("aula.bozza")}>
    <span class="occ">{t(`aula.bozza_${f}`)}</span>
    {#each voci as v (v.d.id)}
      <div class="voce">
        <span>{v.d.nome}: <b>{v.nuovo}</b> <span class="muto">{t("aula.bozza_ora", { gradino: v.ora })}</span></span>
        <Pulsante forma="link" onclick={() => ontoglie(v.d.id, bozza.find((x) => x.disposizione === v.d.id)!.gradino)}>{t("aula.togli")}</Pulsante>
      </div>
    {/each}
    <div class="ante" class:vecchia={inCalcolo && ante} aria-busy={inCalcolo}>
      {#if errore}
        <Rifiuto testo={errore} />
        {#if nominataAnte}{@render rimando(nominataAnte)}{/if}
      {:else if ante}
        <dl class="dati">
          <dt>{t("anteprima.spesa")}</dt><dd><b>{spesa(ante.costo_annuo)}</b></dd>
          <dt>{t("anteprima.effetti")}</dt>
          <dd class="sec">{#each ante.effetti as e (e.grandezza)}<span class="riga">{rigaEffetto(vista, e)}</span>{:else}—{/each}</dd>
          <dt>{t("anteprima.voto")}</dt>
          <dd class="sec">{#if votano(vista, ante.linee, true)}<span class="riga">{t("anteprima.linea_si", { nomi: votano(vista, ante.linee, true) })}</span>{/if}{#if votano(vista, ante.linee, false)}<span class="riga">{t("anteprima.linea_no", { nomi: votano(vista, ante.linee, false) })}</span>{/if}</dd>
          <dt>{t("edicola.camera")}</dt><dd class="sec">{ramo(ante.forbice.camera, ante.forbice.maggioranza_camera)}</dd>
          <dt>{t("edicola.senato")}</dt><dd class="sec">{ramo(ante.forbice.senato, ante.forbice.maggioranza_senato)}</dd>
        </dl>
        {#if ante.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
        <div bind:this={piede}><Pulsante forma="link" onclick={(e) => scambia(e, true)}>{t("aula.anteprima")}</Pulsante></div>
      {:else if inCalcolo}
        <p class="sec muto">{t("aula.ante_in_calcolo")}</p>
      {/if}
    </div>
    <div class="conf">
      <Pulsante forma="chiude" spento={occupato} onclick={() => manda(comandoBozza(vista, bozza))}>{t(etichetta)}</Pulsante>
      <Pulsante forma="link" onclick={onsvuota}>{t("aula.svuota")}</Pulsante>
    </div>
    {#if rifiuto && (rifiuto.tipo === "presenta_legge" || rifiuto.tipo === "presenta_proposta")}
      <Rifiuto testo={rifiuto.testo} />
      {#if nominata}{@render rimando(nominata)}{/if}
    {/if}
  </section>
{/if}

<style>
  .bozza { border: var(--forte); padding: 8px 10px; display: grid; gap: 6px; background: var(--carta) }
  .bozza.vuota { border-style: dashed; border-color: var(--grigio); color: var(--grigio) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--ink) }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .voce { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 8px; font-size: var(--c-sec); align-items: baseline }
  .ante { display: grid; gap: 4px; border-top: var(--filo); padding-top: 6px }
  .ante.vecchia { opacity: .5 }
  .dati { display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 3px 12px; font-size: var(--c-sec) }
  .dati dt { color: var(--grigio) }
  .riga { display: block }
  .chip { justify-self: start; border: 1.5px solid var(--rosso); color: var(--rosso); padding: 0 6px; font-size: var(--c-min); font-weight: 700; line-height: 1.4 }
  .conf { display: flex; gap: 10px; align-items: center; flex-wrap: wrap }
</style>
