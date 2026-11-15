<script lang="ts">
  // La bozza della legge nel dettaglio a lato (storia 108): i valori scelti con «ora …» accanto, l'anteprima (una in volo per volta, vince l'ultima),
  // il bottone della forma, il rifiuto del core sul posto col rimando alla misura in esame se il testo la nomina. «Anteprima della misura ›» apre
  // il cartellino al posto della bozza, come Misura.svelte: cambiato un gradino, oscilla.
  import { tick, untrack } from "svelte";
  import { t, numero, elenco } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { inOrdine } from "$lib/partiti/schieramento";
  import { partita } from "$lib/partita.svelte";
  import { guscio, testoRifiuto } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import { euro } from "$lib/governo/governo";
  import type { Anteprima } from "$lib/tipi/Anteprima";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Cartellino from "$lib/anteprima/Cartellino.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { comandoBozza, etichettaBozza, forma, misuraNominata, type Bozza } from "./bozza";

  let { vista, bozza, manda, rifiuto, ontoglie, onsvuota, onapri }: {
    vista: Vista; bozza: Bozza; manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null;
    ontoglie: (disposizione: string, gradino: string) => void; onsvuota: () => void; onapri: (misura: number) => void;
  } = $props();

  const f = $derived(forma(vista, bozza));
  const etichetta = $derived(etichettaBozza(vista, bozza));
  const voci = $derived(bozza.flatMap((x) => {
    const d = vista.disposizioni.find((y) => y.id === x.disposizione);
    return d ? [{ d, nuovo: d.gradini.find((g) => g.id === x.gradino)?.nome ?? x.gradino, ora: d.gradini.find((g) => g.id === d.in_vigore)?.nome ?? d.in_vigore }] : [];
  }));
  const partiti = $derived(inOrdine(vista));
  const grandezza = (id: string) => vista.grandezze.find((g) => g.id === id)?.nome ?? id;
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);
  const nominata = $derived(rifiuto && (rifiuto.tipo === "presenta_legge" || rifiuto.tipo === "presenta_proposta") ? misuraNominata(vista, rifiuto.testo) : null);

  // una anteprima in volo per volta: chi cambia gradino in fretta lascia l'ultima, e una risposta vecchia non copre la nuova
  let ante = $state.raw<Anteprima | null>(null);
  let errore = $state<string | null>(null);
  let inCalcolo = $state(false);
  let ultima = "";
  const chiedi = ultimoVince(
    (b: Bozza) => (vista.governa ? guscio.anteprima(b) : guscio.anteprimaProposta(b)),
    (r, b) => {
      if (JSON.stringify(b) !== ultima) return;
      inCalcolo = false;
      if ("specie" in r) {
        ante = null;
        errore = r.specie === "rifiuto" ? testoRifiuto(r) : r.testo;
      } else {
        ante = r;
        errore = null;
      }
    },
  );
  $effect(() => {
    const b = bozza;
    ultima = JSON.stringify(b);
    untrack(() => {
      if (!b.length) {
        ante = null;
        errore = null;
        inCalcolo = false;
        return;
      }
      inCalcolo = true;
      chiedi(b);
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

  const dice = (si: boolean) => elenco(partiti.filter((p) => ante?.linee.some((l) => l.partito === p.id && l.si === si)).map((p) => p.nome), ", ") || t("aula.nessuno");
  const effetto = (e: Anteprima["effetti"][number]) =>
    t("aula.ante_effetto", {
      nome: grandezza(e.grandezza), fascia: e.fascia, verso: t(e.migliora ? "governo.effetto_meglio" : "governo.effetto_peggio"),
      quando: e.ritardo ? t("governo.effetto_fra", { n: e.ritardo }) : t("governo.effetto_subito"),
    });
  const ramo = ([da, a]: [number, number], n: number) => t("aula.ante_forbice", { da: numero(da), a: numero(a), n: numero(n) });
</script>

{#if f && etichetta && apertoCartellino && ante}
  <Cartellino {vista} misura={ante} valori={bozza} indietro={t("anteprima.torna_alla_bozza")} ontorna={(e) => scambia(e, false)} {manda} {rifiuto} fuoco={daFuoco} />
{:else if !f || !etichetta}
  <div class="bozza vuota sec">{t("aula.bozza_vuota", { azione: t(vista.governa ? "aula.cambia" : "aula.proponi") })}</div>
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
      {:else if ante}
        <dl class="dati">
          <dt>{t("aula.ante_costo")}</dt><dd><b>{euro(ante.costo_annuo)}</b></dd>
          <dt>{t("aula.ante_effetti")}</dt>
          <dd class="sec">{#each ante.effetti as e (e.grandezza)}<span class="riga">{effetto(e)}</span>{:else}—{/each}</dd>
          <dt>{t("aula.ante_linee")}</dt><dd class="sec">{t("aula.ante_linee_dice", { si: dice(true), no: dice(false) })}</dd>
          <dt>{t("edicola.camera")}</dt><dd class="sec">{ramo(ante.forbice.camera, ante.forbice.maggioranza_camera)}</dd>
          <dt>{t("edicola.senato")}</dt><dd class="sec">{ramo(ante.forbice.senato, ante.forbice.maggioranza_senato)}</dd>
        </dl>
        {#if ante.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}
        <div bind:this={piede}><Pulsante forma="link" onclick={(e) => scambia(e, true)}>{t("aula.anteprima")}</Pulsante></div>
      {:else}
        <p class="sec muto">{t("aula.ante_in_calcolo")}</p>
      {/if}
    </div>
    <div class="conf">
      <Pulsante forma="chiude" spento={occupato} onclick={() => manda(comandoBozza(vista, bozza))}>{t(etichetta)}</Pulsante>
      <Pulsante forma="link" onclick={onsvuota}>{t("aula.svuota")}</Pulsante>
    </div>
    {#if rifiuto && (rifiuto.tipo === "presenta_legge" || rifiuto.tipo === "presenta_proposta")}
      <Rifiuto testo={rifiuto.testo} />
      {#if nominata}<Pulsante forma="link" onclick={() => onapri(nominata.id)}>{t("aula.apri_misura", { nome: nominata.nome })}</Pulsante>{/if}
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
