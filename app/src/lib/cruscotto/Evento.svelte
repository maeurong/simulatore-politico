<script lang="ts">
  // Evento aperto (17-cruscotto.prototype.html, forma A): costa al giocatore, quindi sta in testa e pesa (docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:47).
  import { t } from "$lib/testi/t";
  import { euro } from "$lib/governo/governo";
  import { partita } from "$lib/partita.svelte";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceEvento } from "$lib/tipi/VoceEvento";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { defaultDi, fondoBasta, motivoCrisi, opzioneUnica } from "./cruscotto";

  // `norma`/`onnorma`: il decreto d'evento scelto, che il dettaglio a lato mostra coi suoi gradini
  type Props = { vista: Vista; evento: VoceEvento; indice: number; puoRispondere: boolean; norma?: string | null; onnorma?: (id: string) => void };
  let { vista, evento, indice, puoRispondere, norma = null, onnorma }: Props = $props();
  const id = $props.id();

  const occhiello = $derived([evento.famiglia, evento.regione, evento.tono === "grave" ? t("evento.tono_grave") : null].filter(Boolean).join(" · "));
  // la quota coperta la dice il core: qui solo quanto resta nel fondo, in rosso se non basta
  const fondo = $derived(puoRispondere && evento.costo > 0);
  const norme = $derived(evento.norme.map((id) => ({ id, nome: vista.disposizioni.find((d) => d.id === id)?.nome ?? id })));
  const crisi = $derived(motivoCrisi(vista));
  const unica = $derived(opzioneUnica(evento));
  const scelta = $derived(evento.scelta === null ? null : evento.opzioni[evento.scelta]);
  // la conseguenza si legge prima della scelta (docs/ricerca/estetica/interfacce-di-giochi-gestionali.md:48)
  const conseguenza = $derived(
    unica ? t("evento.opzione_unica", { nome: evento.opzioni[0].nome })
    : scelta ? t("evento.scelto", { nome: scelta.nome })
    : t("evento.senza_scelta", { nome: defaultDi(evento)?.nome ?? "" }),
  );

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ testo: string; vista: Vista | null } | null>(null);
  async function rispondi(opzione: string) {
    if (partita.occupato) return;
    rifiuto = null;
    try {
      await partita.comanda({ tipo: "rispondi_evento", evento: indice, opzione });
    } catch (e) {
      rifiuto = { testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
</script>

<section class="evento" aria-label={t("evento.aperto")}>
  <div class="occ">{occhiello}</div>
  <div class="capo">
    <h2 class="tit">{evento.nome}{evento.costo ? `: ${euro(evento.costo)}` : ""}</h2>
    {#if fondo}<span class="min muto" class:male={!fondoBasta(vista, evento)}>{t("cruscotto.fondo", { euro: euro(vista.bilancio.fondo_residuo) })}</span>{/if}
  </div>
  <div class="sec muto"><i>{t("componenti.perche")}</i> {evento.causa}</div>
  {#if puoRispondere && onnorma && norme.length}
    <div class="norme sec muto">{t("cruscotto.norme")}
      {#each norme as n (n.id)}<Pulsante forma="piccolo" scelto={norma === n.id} aria-disabled={crisi ? "true" : undefined} aria-describedby={crisi ? `${id}-crisi` : undefined} onclick={() => !crisi && onnorma(n.id)}>{n.nome}</Pulsante>{/each}
      {#if crisi}<span id="{id}-crisi" class="min">{crisi}</span>{/if}
    </div>
  {/if}
  {#if !unica}
    <div class="opz">
      {#each evento.opzioni as o, i (o.id)}
        {#if puoRispondere}
          <Pulsante forma="bordo" scelto={evento.scelta === i} aria-disabled={partita.occupato ? "true" : undefined} onclick={() => rispondi(o.id)}>
            {o.nome}{#if o.default} <small>{t("evento.default")}</small>{/if}
          </Pulsante>
        {:else}
          <span class="chip" class:scelto={evento.scelta === i}>{o.nome}{#if o.default} <small>{t("evento.default")}</small>{/if}</span>
        {/if}
      {/each}
    </div>
  {/if}
  {#if rifiuto?.vista === partita.vista}<Rifiuto testo={rifiuto.testo} />{/if}
  <div class="min muto">{conseguenza}</div>
</section>

<style>
  .evento { border: var(--forte); padding: 10px 14px; display: grid; gap: 6px }
  :global(:root.grave) .evento { border: var(--doppio) }
  .capo { display: flex; flex-wrap: wrap; align-items: baseline; gap: 2px 14px }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso) }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-sez) }
  :global(:root.grave) .tit { font-weight: 600 }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .norme { display: flex; gap: 4px 6px; align-items: center; flex-wrap: wrap }
  .male { color: var(--rosso); font-weight: 700 }
  .opz { display: flex; gap: 6px; flex-wrap: wrap; align-items: center }
  .opz :global(.btn small), .chip small { font-weight: 400; font-size: var(--c-min); margin-left: 5px }
  .opz :global(.btn.scelto small), .opz :global(.btn:hover small) { color: inherit }
  /* in sola lettura: niente bordo pieno, non si confonde coi bottoni */
  .chip { border: 1px dashed var(--grigio); padding: 0 6px; font-size: var(--c-sec); line-height: 1.5; cursor: default }
  .chip.scelto { background: var(--giallo) }
  :global(:root.grave) .chip.scelto { background: var(--ink); color: var(--carta) }
</style>
