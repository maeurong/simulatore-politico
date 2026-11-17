<script lang="ts">
  // Il dettaglio a lato di una misura (storia 106): i due emicicli, la linea di ogni partito, i seggi di una legge elettorale, la regola
  // delle due deliberazioni, gli emendamenti degli alleati (Accogli · Respingi, diretti), la fiducia (conferma sul posto) e la linea del giocatore.
  // La legge di bilancio all'opposizione (ticket 22): le cifre in aula per dicastero e il riepilogo del presentato, perché il Bilancio è barrato.
  import { tick } from "svelte";
  import { t, numero, segnato, elenco } from "$lib/testi/t";
  import { nomeDi } from "$lib/partiti";
  import { inOrdine } from "$lib/partiti/schieramento";
  import { partita } from "$lib/partita.svelte";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceMisura } from "$lib/tipi/VoceMisura";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Emiciclo from "$lib/componenti/Emiciclo.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Linea from "./Linea.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Cartellino from "$lib/anteprima/Cartellino.svelte";
  import { chiLaPorta, fiduciaInCrisi, fiduciaPonibile, formaDi, lineaScelta, quandoMisura, slitta } from "./aula";
  import { motivoCrisi } from "$lib/cruscotto/cruscotto";
  import { reale } from "$lib/conti/conti";
  import { distanzaTetto, euroSegnato, riepilogoPresentato } from "$lib/bilancio/bilancio";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";

  let { vista, m, manda, rifiuto }: {
    vista: Vista; m: VoceMisura; manda: (c: Comando) => Promise<boolean>; rifiuto: { testo: string; tipo: Comando["tipo"] } | null;
  } = $props();

  // il cartellino sta al posto del dettaglio; Aula rimonta Misura a ogni misura e a ogni turno, quindi lì si richiude
  let anteprima = $state(false);
  let piede = $state<HTMLElement>();
  let daFuoco = $state(false);
  // il fuoco si sposta solo se stava sul bottone che sparisce: in WKWebView un clic non lo dà
  async function scambia(e: MouseEvent, apri: boolean) {
    const aveva = e.currentTarget === document.activeElement;
    daFuoco = aveva;
    anteprima = apri;
    if (aveva && !apri) {
      await tick();
      piede?.querySelector("button")?.focus({ preventScroll: true });
    }
  }

  const nome = (id: string) => nomeDi(vista, id);
  const partiti = $derived(inOrdine(vista));
  const linea = (id: string) => m.linee.find((l) => l.partito === id)?.si ?? false;
  const camera = $derived(vista.seggi.reduce((s, x) => s + x.camera, 0));
  const senato = $derived(vista.seggi.reduce((s, x) => s + x.senato, 0));
  const aperti = $derived(m.emendamenti.filter((e) => e.stato === "aperto"));
  const contro = $derived(m.alleati_contro_fiducia.length ? t("aula.fiducia_no", { nomi: elenco(m.alleati_contro_fiducia.map(nome), ", ") }) : t("aula.fiducia_nessuno_no"));
  let fiduciaAperta = $state(false);
  const occupato = $derived(partita.occupato ? t("campagna.comando_in_corso") : undefined);

  const delGoverno = $derived(m.forma === "bilancio" && !vista.governa ? vista.bilancio.in_aula : null);
  let conto = $state.raw<RiepilogoBilancio | string | null>(null);
  $effect(() => {
    const v = vista;
    if (delGoverno) void riepilogoPresentato().then((r) => { if (v === vista) conto = r; });
  });
</script>

{#if anteprima}
  <Cartellino {vista} misura={m} valori={m.valori} indietro={t("anteprima.torna_alla_misura")} ontorna={(e) => scambia(e, false)} {manda} {rifiuto} fuoco={daFuoco} />
{:else}
<span class="occ">{formaDi(m)}</span>
<h4 class="tit">{m.nome}</h4>
<p class="sec muto">{chiLaPorta(vista, m, nome)} · {quandoMisura(vista, m)}</p>

<div class="emicicli">
  <Emiciclo ramo="Camera" totale={camera} si={m.forbice.camera} maggioranza={m.forbice.maggioranza_camera} />
  <Emiciclo ramo="Senato" totale={senato} si={m.forbice.senato} maggioranza={m.forbice.maggioranza_senato} />
</div>
{#if m.forbice.in_bilico}<span class="chip rosso">{t("aula.in_bilico")}</span>{/if}

<dl class="dati">
  {#each partiti as p (p.id)}
    <dt>{p.nome}</dt><dd class:muto={!linea(p.id)}><b>{t(linea(p.id) ? "aula.si" : "aula.no")}</b></dd>
  {/each}
</dl>

{#if m.forma === "elettorale" && m.seggi.length}
  <div class="blocco">
    <span class="occ nero">{t("aula.seggi_titolo")}</span>
    <div class="sg">
      <span class="h">{t("aula.col_partito")}</span><span class="h n">{t("edicola.camera")}</span><span class="h n">{t("edicola.senato")}</span>
      {#each partiti as p (p.id)}
        {@const x = m.seggi.find((s) => s.partito === p.id)}
        <span>{p.nome}</span>
        <span class="n {(x?.camera ?? 0) > 0 ? 'bene' : (x?.camera ?? 0) < 0 ? 'male' : 'muto'}">{segnato(x?.camera ?? 0)}</span>
        <span class="n {(x?.senato ?? 0) > 0 ? 'bene' : (x?.senato ?? 0) < 0 ? 'male' : 'muto'}">{segnato(x?.senato ?? 0)}</span>
      {/each}
    </div>
  </div>
{/if}

{#if delGoverno}
  {@const b = delGoverno}
  <div class="blocco">
    <span class="occ nero">{t("bilancio.conto_governo")}</span>
    {#if conto && typeof conto === "object"}
      <p class="sec">{t("bilancio.giogo_spesa", { n: segnato(conto.crescita_spesa_netta, 1) })} · <span class="male">{t("bilancio.giogo_tetto", { n: numero(conto.tetto, 1) })}</span></p>
      <p class="sec">{distanzaTetto(conto)}</p>
      <dl class="dati">
        <dt>{t("bilancio.spesa_in_piu")}</dt><dd><b>{euroSegnato(conto.costo_spesa)}</b></dd>
        <dt>{t("bilancio.gettito_perso")}</dt><dd><b>{euroSegnato(conto.costo_entrate)}</b></dd>
      </dl>
    {:else if conto}<Rifiuto testo={conto} />{/if}
    <div class="bil">
      <span class="h">{t("bilancio.spesa_dicasteri")}</span><span class="h n">{b.anno}</span><span class="h n">{t("bilancio.reale")}</span>
      {#each b.dicasteri as d (d.id)}
        {@const p = conto && typeof conto === "object" ? conto.perdita_reale.find((x) => x.voce === d.id)?.perdita : undefined}
        {@const r = p === undefined ? null : reale(p)}
        <span>{vista.dicasteri.find((x) => x.id === d.id)?.nome ?? d.id}</span>
        <span class="n">{t("conti.miliardi", { n: numero(d.spesa / 1e9, 1) })}</span>
        <span class="n {r?.tono ?? 'muto'}">{r?.testo ?? "—"}</span>
      {/each}
    </div>
  </div>
{/if}

{#if m.forma === "costituzionale"}<p class="sec">{t("aula.regola_costituzionale")}</p>{/if}

{#if m.emendamenti.length}
  <div class="blocco">
    <span class="occ nero">{t("aula.emendamenti")}</span>
    {#each m.emendamenti as e (e.partito)}
      <div class="voce">
        <span><b>{nome(e.partito)}</b>: {e.testo}</span>
        {#if e.stato !== "aperto"}
          <span class="chip">{t(e.stato === "accolto" ? "aula.accolto" : "aula.respinto")}</span>
        {:else if vista.governa}
          <span class="az">
            <Pulsante forma="piccolo" aria-label={t("aula.accogli_di", { nome: nome(e.partito) })} onclick={() => manda({ tipo: "emendamento", misura: m.id, partito: e.partito, accogli: true })}>{t("aula.accogli")}</Pulsante>
            <Pulsante forma="piccolo" aria-label={t("aula.respingi_di", { nome: nome(e.partito) })} onclick={() => manda({ tipo: "emendamento", misura: m.id, partito: e.partito, accogli: false })}>{t("aula.respingi")}</Pulsante>
          </span>
        {/if}
        <small>{e.motivo}</small>
      </div>
    {/each}
    {#if aperti.length && vista.governa}<p class="min muto">{t(slitta(vista, m) ? "aula.emendamento_slitta" : "aula.emendamento_default")}</p>{/if}
    {#if rifiuto?.tipo === "emendamento"}<Rifiuto testo={rifiuto.testo} />{/if}
  </div>
{/if}

{#if m.fiducia}
  <div class="blocco"><span class="chip pieno">{t("aula.fiducia_posta")}</span></div>
{:else if fiduciaPonibile(vista, m) || fiduciaInCrisi(vista, m)}
  <div class="blocco">
    <div class="conf">
      <Conferma bind:aperta={fiduciaAperta} etichetta={t("aula.poni_fiducia")} conferma={t("aula.poni_fiducia_conferma")} spiegazione={contro} spento={motivoCrisi(vista) ?? occupato}
        onconferma={() => manda({ tipo: "poni_fiducia", misura: m.id })} />
      {#if !fiduciaAperta}<span class="sec" class:male={m.alleati_contro_fiducia.length > 0}><b>{contro}</b></span>{/if}
    </div>
    <span class="min muto">{t("aula.fiducia_costo")}</span>
    {#if rifiuto?.tipo === "poni_fiducia"}<Rifiuto testo={rifiuto.testo} />{/if}
  </div>
{/if}

{#if lineaScelta(vista, m) && m.costo_linea}
  <Linea {vista} misura={m} {occupato} onlinea={(si) => manda({ tipo: "linea", misura: m.id, si })}>
    {#if rifiuto?.tipo === "linea"}<Rifiuto testo={rifiuto.testo} />{/if}
  </Linea>
{/if}

<div bind:this={piede}><Pulsante forma="link" onclick={(e) => scambia(e, true)}>{t("aula.anteprima")}</Pulsante></div>
{/if}

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .occ.nero { color: var(--ink) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .bene { color: var(--verde) }
  .male { color: var(--rosso) }
  .emicicli { display: grid; grid-template-columns: 1.5fr 1fr; gap: 8px; align-items: end }
  .dati { display: grid; grid-template-columns: auto 1fr; gap: 3px 12px; font-size: var(--c-sec); align-items: center }
  .dati dt { color: var(--grigio) }
  .blocco { border-top: var(--filo); padding-top: 6px; display: grid; gap: 4px }
  .conf { display: flex; gap: 6px; align-items: center; flex-wrap: wrap }
  .chip { justify-self: start; display: inline-block; border: 1.5px solid var(--ink); padding: 0 6px; font-size: var(--c-min); font-weight: 700; white-space: nowrap; line-height: 1.4 }
  .chip.rosso { border-color: var(--rosso); color: var(--rosso) }
  .chip.pieno { background: var(--ink); color: var(--carta) }
  .voce { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 1px 10px; font-size: var(--c-sec) }
  .voce small { grid-column: 1 / -1; color: var(--grigio); font-size: var(--c-min) }
  .az { display: flex; gap: 6px }
  .sg { display: grid; grid-template-columns: minmax(0, 1fr) 60px 60px; gap: 2px 8px; font-size: var(--c-sec); align-items: center }
  .sg .h { font-size: var(--c-min); color: var(--grigio); text-transform: uppercase; letter-spacing: .06em }
  .sg .n { text-align: right; font-weight: 700 }
  .bil { display: grid; grid-template-columns: minmax(0, 1fr) auto auto; gap: 2px 12px; font-size: var(--c-sec); align-items: baseline }
  .bil .h { font-size: var(--c-min); color: var(--grigio); text-transform: uppercase; letter-spacing: .06em }
  .bil .n { text-align: right; white-space: nowrap }
</style>
