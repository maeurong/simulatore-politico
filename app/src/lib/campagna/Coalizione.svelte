<script lang="ts">
  // Coalizione e cessioni (26-campagna.prototype.html, forma A): ogni passo è una dichiarazione, senza conferma.
  import { t, elenco } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { coloreDi } from "$lib/partiti";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Cessione } from "$lib/tipi/Cessione";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { PASSO_CESSIONE, cedi, dichiara, righeCoalizione, seggiAttesi } from "./campagna";

  let { vista }: { vista: Vista } = $props();
  const righe = $derived(righeCoalizione(vista));
  const attesi = $derived(seggiAttesi(vista));
  const chi = $derived(vista.coalizione.length ? elenco(vista.coalizione.map((id) => righe.find((r) => r.id === id)?.nome ?? id), ", ") : null);

  // le cessioni non sono nella Vista: vivono qui e ripartono da zero al rientro
  let cessioni = $state.raw<Cessione[]>([]);
  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ id: string; testo: string; vista: Vista | null } | null>(null);

  async function manda(id: string, comando: Comando) {
    if (partita.occupato || comando.tipo !== "dichiara_coalizione") return;
    rifiuto = null;
    try {
      if (await partita.comanda(comando)) cessioni = comando.cessioni;
    } catch (e) {
      rifiuto = { id, testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
  const seggiDi = (id: string) => cessioni.find((c) => c.partito === id)?.seggi ?? 0;
</script>

<section aria-label={t("campagna.coalizione")}>
  <p class="sec">
    <b>{chi ? t("campagna.corri_con", { chi }) : t("campagna.da_solo")}</b>
    {#if vista.governa}{t("campagna.resta_ultimo_voto")}{/if}
    {t("campagna.cambia_con_chi_c_e")}
  </p>
  <p class="min muto attesi">{attesi === null ? t("campagna.attesi_vuoto") : t("campagna.attesi", { n: attesi })}</p>
  {#each righe as r (r.id)}
    {@const seggi = seggiDi(r.id)}
    <div class="pr" class:dentro={r.dichiarato}>
      <i class="pdot" style:background={coloreDi(r.id) ?? "var(--retino)"}></i>
      <span class="n">{r.nome}{#if r.dichiarato} <span class="min muto">· {t("campagna.in_coalizione")}</span>{/if}</span>
      <span class="az">
        {#if r.dichiarato}
          <Pulsante forma="piccolo" aria-disabled={partita.occupato ? "true" : undefined} onclick={() => manda(r.id, dichiara(vista, cessioni, r.id, false))}>{t("campagna.togli")}</Pulsante>
        {:else if r.disponibile}
          <span class="si">{t("campagna.disponibile")}</span>
          <Pulsante forma="piccolo" aria-disabled={partita.occupato ? "true" : undefined} onclick={() => manda(r.id, dichiara(vista, cessioni, r.id, true))}>{t("campagna.aggiungi")}</Pulsante>
        {:else}
          <span class="no">{t("campagna.non_disponibile")}</span>
        {/if}
      </span>
      <span class="c"><i>{t("componenti.perche")}</i> {r.causa}</span>
      {#if !r.disponibile || seggi}
        <div class="ces">
          <span class="min muto">{t("campagna.cedi")}</span>
          <span class="step">
            <button type="button" aria-label={t("campagna.meno_seggi", { nome: r.nome })} aria-disabled={seggi === 0 || partita.occupato ? "true" : undefined}
              onclick={() => seggi > 0 && manda(r.id, cedi(vista, cessioni, r.id, seggi - PASSO_CESSIONE))}>−</button>
            <span>{seggi ? t("campagna.alla_camera", { n: seggi }) : t("campagna.nessuno")}</span>
            <button type="button" aria-label={t("campagna.piu_seggi", { nome: r.nome })} aria-disabled={partita.occupato ? "true" : undefined}
              onclick={() => manda(r.id, cedi(vista, cessioni, r.id, seggi + PASSO_CESSIONE))}>+</button>
          </span>
          {#if seggi && r.dichiarato}
            <div class="fatto">{t("campagna.entra", { n: seggi, causa: r.causa })}</div>
          {/if}
        </div>
      {/if}
      {#if rifiuto?.id === r.id && rifiuto.vista === partita.vista}<div class="rif"><Rifiuto testo={rifiuto.testo} /></div>{/if}
    </div>
  {/each}
  <p class="min muto">{t("campagna.cessioni_in_memoria")}</p>
</section>

<style>
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .attesi { margin: 2px 0 6px }
  .pr { display: grid; grid-template-columns: auto minmax(min-content, 1fr) auto; gap: 2px 8px; align-items: center; padding: 5px 2px 6px; border-top: var(--filo) }
  .pr.dentro { background: var(--velo) }
  .pdot { width: 10px; height: 10px; border: 1px solid var(--ink); flex: none; display: inline-block }
  .n { font-weight: 700; font-size: var(--c-sec); min-width: 0; display: flex; gap: 0 6px; flex-wrap: wrap; align-items: baseline }
  .az { display: flex; flex-wrap: wrap; justify-content: flex-end; gap: 2px 6px; align-items: center }
  /* forma stretta: «non disponibile» (nowrap) non entra accanto al nome e scende sotto */
  :global(:root.stretta) .az { grid-column: 2 / -1; justify-content: flex-start }
  .c { grid-column: 2 / -1; font-size: var(--c-min); color: var(--grigio) }
  .ces, .rif { grid-column: 2 / -1 }
  .ces { display: flex; gap: 4px 8px; align-items: center; flex-wrap: wrap }
  .si, .no { white-space: nowrap; font-weight: 700; font-size: var(--c-min); text-align: end }
  .si { color: var(--verde) }
  .no { color: var(--rosso) }
  .step { display: inline-flex; align-items: center; border: var(--bordo) }
  .step button { padding: 0 7px; font-weight: 700 }
  .step button[aria-disabled="true"] { color: var(--grigio); cursor: default }
  .step span { padding: 0 8px; border-inline: var(--bordo); font-weight: 700; font-size: var(--c-sec); min-width: 84px; text-align: center }
  .fatto { flex-basis: 100%; border: 1.5px solid var(--verde); color: var(--verde); padding: 4px 8px; font-size: var(--c-sec); font-weight: 700 }
  :global(:root.grave) .fatto { border-color: var(--ink); color: var(--ink) }
  .rif { border: 1.5px solid var(--rosso); padding: 4px 8px; margin: 2px 0 }
</style>
