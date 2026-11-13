<script lang="ts">
  // Il dettaglio di una voce d'albo (13-edicola.prototype.html:255-306; «Leggi l'eredità ›» dal ticket 30).
  import { numero, percentuale, segnato, t, type Chiave } from "$lib/testi/t";
  import { etichetta } from "$lib/ossatura";
  import type { Trimestre } from "$lib/tipi/Trimestre";
  import type { VoceAlbo } from "$lib/tipi/VoceAlbo";
  import NumeroFreccia from "$lib/componenti/NumeroFreccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import Grafico from "./Grafico.svelte";
  import { anniAlGoverno, esitoTesto, maleSe, periodo } from "./edicola";

  let { voce }: { voce: VoceAlbo } = $props();

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
  let leggi = $state(false);
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
  <Pulsante forma="bordo" aria-expanded={leggi} onclick={() => (leggi = true)}>{t("edicola.leggi_eredita")}</Pulsante>
  {#if leggi}<Segnaposto sezione={etichetta("eredita")} fetta="11e" />{/if}
</div>

<div>
  <h3 class="sotto">{t("edicola.contro_il_turno_zero")}</h3>
  <table class="ind">
    <thead><tr><th>{t("edicola.indicatore")}</th><th class="r">{t("edicola.turno_zero")}</th><th class="r">{t("edicola.fine")}</th><th><span class="nascosto">{t("edicola.scarto")}</span></th></tr></thead>
    <tbody>
      {#each voce.indicatori as i (i.id)}
        {@const decimali = i.id === "spread" ? 0 : 1}
        <tr>
          <td class="muto">{i.nome}</td>
          <td class="r">{numero(i.turno_zero, decimali)}</td>
          <td class="r"><b>{numero(i.fine, decimali)}</b></td>
          <!-- ponytail: lo scarto in colonna sua (13-edicola.prototype.html:115), col valore vuoto -->
          <td class="r"><NumeroFreccia valore={i.fine} prima={i.turno_zero} formato={() => ""} formatoVariazione={(x) => numero(x, decimali)} male={maleSe(i.id)} /></td>
        </tr>
      {/each}
    </tbody>
  </table>
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
    {#each voce.elezioni as e, k (k)}
      {@const tot = e.voti.reduce((s, v) => s + v[1], 0) || 1}
      {@const mio = e.voti.find((v) => v[0] === voce.nome)}
      {@const [primaQuota, dopoQuota] = t("edicola.esito_giocatore", { nome: voce.nome, camera: e.seggi_giocatore[0], senato: e.seggi_giocatore[1] }).split("{quota}")}
      <div class="el">
        <p class="sec"><b>{t("edicola.voto_di", { periodo: periodo(e.data) })}</b> <span class="muto">{t("edicola.affluenza", { quota: percentuale(e.affluenza) })}</span></p>
        <div class="barra" role="img" aria-label={e.voti.map(([n, q]) => `${n} ${percentuale(q)}`).join(", ")}>
          {#each e.voti as [n, q], j (j)}<i class:mio={n === voce.nome} style:flex={q / tot} title="{n} {percentuale(q)}"></i>{/each}
        </div>
        <div class="etich" aria-hidden="true">
          {#each e.voti as [n, q], j (j)}<span class:mio={n === voce.nome} style:flex={q / tot} data-tagli-ignora={n === voce.nome ? undefined : ""}>{q > 0.07 ? `${n.split(" ").pop()} ${numero(q * 100)}%` : ""}</span>{/each}
        </div>
        <p class="sec">{primaQuota}<b>{mio ? percentuale(mio[1]) : t("edicola.assente")}</b>{dopoQuota}</p>
      </div>
    {/each}
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
  .leggi { display: grid; gap: 6px; justify-items: start }
  .blocco { display: grid; gap: 8px }

  .ind { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .ind th { font-size: var(--c-min); color: var(--grigio); letter-spacing: .06em; text-transform: uppercase; font-weight: 400; text-align: left; padding: 0 0 3px }
  .ind td { padding: 2px 0 }
  .ind td + td, .ind th + th { padding-left: 12px }
  .ind .r { text-align: right; white-space: nowrap }

  .el { display: grid; gap: 4px; padding-top: 6px; border-top: var(--filo) }
  .barra { display: flex; gap: 2px; height: 16px }
  .barra i { background: var(--grigio); min-width: 0 }
  .barra i:nth-child(even) { background: var(--ink) }
  .barra i.mio { background: var(--rosso) }
  /* contain: le etichette, che ci stiano o no, non allargano il dettaglio */
  .etich { display: flex; gap: 2px; font-size: var(--c-min); color: var(--grigio); contain: inline-size }
  /* un'etichetta che non ci sta non si mostra: va a capo dietro un primo elemento vuoto e l'altezza di una riga la taglia */
  /* 1.35em è l'interlinea della radice: il ripiego per un WebKit senza lh */
  .etich span { min-width: 0; display: flex; flex-wrap: wrap; height: 1.35em; height: 1lh; overflow: hidden; white-space: nowrap }
  .etich span::before { content: ""; height: 1.35em; height: 1lh }
  /* quella del giocatore non si stringe */
  .etich span.mio { min-width: max-content; color: var(--rosso); font-weight: 700 }
  /* inline-block: WebKit non va a capo davanti a un nowrap in riga e lo lascia sporgere */
  .nw { display: inline-block; white-space: nowrap }
</style>
