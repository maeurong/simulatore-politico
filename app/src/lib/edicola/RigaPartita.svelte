<script lang="ts">
  // Una riga di «In corso» (13-edicola.prototype.html:319-323): solo ciò che porta la testata.
  import { dataRelativa, t } from "$lib/testi/t";
  import type { VocePartita } from "$lib/tipi/VocePartita";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { azioneDi, periodo } from "./edicola";

  type Props = {
    voce: VocePartita;
    /** la più recente: più grande, coi pulsanti pieni */
    prima: boolean;
    scelta: boolean;
    ora: Date;
    /** una chiamata del guscio è in volo: il secondo clic non parte */
    occupato: boolean;
    /** l'abbandono di questa partita è in volo */
    inVolo: boolean;
    onscegli: () => void;
    onriprendi: () => void;
    onabbandona: () => void;
  };
  let { voce, prima, scelta, ora, occupato, inVolo, onscegli, onriprendi, onabbandona }: Props = $props();

  const testata = $derived(voce.indice.testata);
  const azione = $derived(azioneDi(voce));
  const [primaDelTurno, dopoIlTurno] = $derived(
    t("edicola.riga", { scenario: testata.scenario, periodo: periodo(testata.data), quando: dataRelativa(voce.indice.ultimo_salvataggio, ora) }).split("{turno}"),
  );
</script>

<!-- il clic su tutta la riga è una comodità del puntatore: da tastiera si sceglie col nome, che è un pulsante -->
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class={["rp", { prima, spenta: azione === "nessuna", sel: scelta }]} onclick={onscegli}>
  <span class="n"
    ><button type="button" class="scegli" aria-current={scelta ? "true" : undefined}>{testata.nome}</button
    >{#if prima}<span class="occ">{t("edicola.ultima_giocata")}</span>{/if}</span>
  <span class="azr">
    {#if azione === "nessuna"}
      <span class="min muto">{t("edicola.non_si_apre")}</span>
    {:else if inVolo}
      <span class="min muto" role="status">{t("edicola.abbandono_in_corso")}</span>
    {:else if azione === "eredita"}
      <Pulsante forma={prima ? "bordo" : "piccolo"} aria-busy={occupato} onclick={onriprendi}>{t("edicola.apri_eredita")}</Pulsante>
    {:else}
      <span class:pieno={!prima}
        ><Pulsante forma={prima ? "chiude" : "piccolo"} aria-busy={occupato} onclick={onriprendi}>{t("edicola.riprendi")}</Pulsante></span>
      <Conferma forma="link" etichetta={t("edicola.abbandona")} conferma={t("edicola.conferma_abbandona")} onconferma={onabbandona} />
    {/if}
  </span>
  <span class="m">
    {#if azione === "nessuna"}
      <span class="male">{voce.motivo}</span>
    {:else}
      {#if azione === "eredita"}{t("edicola.senza_seggi_bozza")}{" · "}{/if}{primaDelTurno}<b>{testata.turno}</b>{dopoIlTurno}{#if voce.indice.riavvolgimenti > 0}
        · {t("edicola.riavvolgimenti", { n: voce.indice.riavvolgimenti })}{/if}
    {/if}
  </span>
</div>

<style>
  .rp { position: relative; display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 14px; padding: 8px; border-bottom: var(--filo); cursor: pointer; align-items: center }
  .rp:hover { background: var(--velo) }
  .rp.sel { background: var(--velo); box-shadow: inset 0 0 0 1.5px var(--ink) }
  .n { font-family: var(--tit); font-weight: 700; font-size: var(--c-riq); line-height: 1.15 }
  .prima .n { font-size: var(--c-sez) }
  .scegli { text-align: left }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--rosso); vertical-align: .2em; margin-left: 6px }
  .m { font-size: var(--c-sec); color: var(--grigio) }
  .m b { color: var(--ink); font-weight: 700 }
  .azr { display: flex; gap: 8px; align-items: center; grid-row: span 2 }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .male { color: var(--rosso) }

  /* il retino del token, attenuato: la riga resta leggibile */
  .spenta .n { color: var(--grigio) }
  .spenta::before { content: ""; position: absolute; inset: 0; background: var(--retino); opacity: .18; pointer-events: none }

  /* nelle righe sotto la prima «Riprendi ›» è piccolo ma pieno, come nel prototipo (.btn.pieno.p) */
  .pieno :global(.btn) { background: var(--ink); color: var(--carta) }
  .pieno :global(.btn:hover) { background: var(--carta); color: var(--ink) }
  .azr :global([aria-busy="true"]) { cursor: progress }
</style>
