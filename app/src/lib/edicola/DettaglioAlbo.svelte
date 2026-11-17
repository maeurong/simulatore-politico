<script lang="ts">
  // Il dettaglio di una voce d'albo (13-edicola.prototype.html:255-306; «Leggi l'eredità ›» dal ticket 30).
  import { numero, segnato, t, type Chiave } from "$lib/testi/t";
  import type { Trimestre } from "$lib/tipi/Trimestre";
  import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Elezioni from "./Elezioni.svelte";
  import Grafico from "./Grafico.svelte";
  import Indicatori from "./Indicatori.svelte";
  import { anniAlGoverno, esitoTesto, periodo } from "./edicola";

  let { voce, onleggi }: { voce: VoceAlbo; onleggi: () => void } = $props();

  const Periodo = (d: Trimestre) => t(`cornice.trimestre_${d.trimestre}` as Chiave, { anno: d.anno });
  const S = $derived(voce.serie);
  const etichette = $derived(S.map((p) => t("edicola.punto", { turno: p.turno, periodo: periodo(p.data) })));
  const voti = $derived(
    voce.elezioni.map((e) => S.findIndex((p) => p.data.anno === e.data.anno && p.data.trimestre === e.data.trimestre)).filter((i) => i >= 0),
  );
  const pb = (v: number) => `${numero(v)} ${t("tracciato.pb")}`;
  // la durata al governo non va a capo: il testo si spezza sul segnaposto
  const [primaDelGoverno, dopoIlGoverno] = $derived(
    t("edicola.arco", { inizio: Periodo(voce.inizio), fine: periodo(voce.fine), turni: t("edicola.turni", { n: voce.turni }) }).split("{governo}"),
  );
</script>

<div>
  <p class="occ">{t("edicola.occhiello_albo", { scenario: voce.scenario })}</p>
  <h2 class="tit">{voce.nome}</h2>
  <p class="sec muto">
    {primaDelGoverno}<span class="nw">{anniAlGoverno(voce.anni_al_governo)}</span>{dopoIlGoverno}
  </p>
</div>
<div class="esito">
  <span class="bollo" class:ink={voce.esito !== "Senza seggi"}>{esitoTesto(voce.esito)}</span>
  <span class="sec">
    {voce.leggi_in_vigore.length ? t("edicola.leggi_in_vigore", { n: voce.leggi_in_vigore.length }) : t("edicola.nessuna_legge")}
    · {t("edicola.riavvolgimenti", { n: voce.riavvolgimenti })}
  </span>
</div>
<div class="leggi">
  <Pulsante forma="bordo" onclick={onleggi}>{t("edicola.leggi_eredita")}</Pulsante>
</div>

<div>
  <h3 class="sotto">{t("edicola.contro_il_turno_zero")}</h3>
  <Indicatori indicatori={voce.indicatori} />
</div>

<div class="blocco">
  <h3 class="sotto">{t("edicola.tre_indicatori")}</h3>
  {#if S.length < 2}
    <p class="sec muto">{t("edicola.serie_corta")}</p>
  {:else}
    <Grafico titolo={t("edicola.grafico_consenso")} linee={[{ valori: S.map((p) => p.consenso * 100) }]} formato={(v) => `${numero(v, 1)}%`} {etichette} {voti} />
    <Grafico titolo={t("edicola.grafico_margine")} zero formato={(v) => segnato(v)} {etichette} {voti}
      linee={[{ nome: t("edicola.camera"), valori: S.map((p) => p.margine_camera) }, { nome: t("edicola.senato"), valori: S.map((p) => p.margine_senato), tratteggiata: true }]} />
    <Grafico titolo={t("edicola.grafico_spread")} linee={[{ valori: S.map((p) => p.spread_pb) }]} formato={pb} {etichette} {voti} />
  {/if}
</div>

{#if voce.elezioni.length}
  <div class="blocco">
    <h3 class="sotto">{t("edicola.elezioni")}</h3>
    <Elezioni elezioni={voce.elezioni} nome={voce.nome} />
  </div>
{/if}

<style>
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  .sotto { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; font-size: var(--c-riq); margin-bottom: 4px }
  .sec { font-size: var(--c-sec) }
  .muto { color: var(--grigio) }
  .esito { display: flex; gap: 14px; align-items: center }
  .bollo {
    display: inline-block; border: var(--doppio); border-color: var(--rosso); color: var(--rosso); font-weight: 700; letter-spacing: .12em;
    text-transform: uppercase; padding: 1px 9px; transform: rotate(-5deg); font-size: var(--c-min); white-space: nowrap;
  }
  .bollo.ink { border-color: var(--ink); color: var(--ink) }
  .blocco { display: grid; gap: 8px }

  /* inline-block: WebKit non va a capo davanti a un nowrap in riga e lo lascia sporgere */
  .nw { display: inline-block; white-space: nowrap }
</style>
