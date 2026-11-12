<script lang="ts">
  // Cornice A · Gerenza dal prototipo 08 (.scratch/v1-piano-11-ui/prototipi/08-fondamenta.prototype.html), testi dal 07.
  import { t, type Chiave } from "$lib/testi/t";
  import { statoDi, type StatoGiocatore } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import { tracciatiDi } from "$lib/tracciati";
  import { comeErrore } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Tracciato from "$lib/componenti/Tracciato.svelte";
  import Sommario from "./Sommario.svelte";

  type Props = { vista: Vista; sezione: string; impostazioni: boolean; onimpostazioni: () => void };
  let { vista, sezione, impostazioni, onimpostazioni }: Props = $props();

  const STATO: Record<StatoGiocatore, Chiave> = {
    turno_zero: "stato.zero", trattativa: "stato.trattativa", governo: "stato.governo",
    crisi: "stato.crisi", opposizione: "stato.opposizione", finita: "stato.fine",
  };
  const stato = $derived(statoDi(vista));
  const nome = $derived(
    stato !== "governo" ? STATO[stato] : vista.turni_al_voto === 0 ? "stato.voto" : vista.data.trimestre === 4 ? "stato.bilancio" : "stato.governo",
  );
  const periodo = $derived(t(`cornice.trimestre_${vista.data.trimestre}` as Chiave, { anno: vista.data.anno }));
  const tre = $derived(tracciatiDi(vista, partita.serie));
  // finché un evento grave è aperto tutto il turno è grave (storia 169): tacciono colori e testatina
  const grave = $derived(vista.eventi.some((e) => e.tono === "grave"));
  $effect(() => {
    document.documentElement.classList.toggle("grave", grave);
    return () => document.documentElement.classList.remove("grave");
  });
  // sulla prima pagina la testata grande c'è già
  const testatina = $derived(sezione !== "prima-pagina" && !grave);

  // raw: la Vista dentro si confronta per identità; il rifiuto vale per la Vista che l'ha causato, con una nuova sparisce
  let rifiuto = $state.raw<{ testo: string; vista: Vista } | null>(null);
  async function chiudi() {
    rifiuto = null;
    try {
      await partita.chiudiTurno();
    } catch (e) {
      rifiuto = { testo: comeErrore(e).testo, vista };
    }
  }
</script>

<header class="cornice">
  <div class="c1">
    <div class="c-sx">
      {#if testatina}<span class="nomino">{t("cornice.nomino")}</span>{/if}
      <span class="chi">
        <b>{t(nome)}</b><br />{t("cornice.periodo_turno", { periodo, turno: vista.turno })}
        {#if !vista.finita}<br /><b>{vista.turni_al_voto === 0 ? t("cornice.si_vota_a_fine_turno") : t("cornice.turni_al_voto", { n: vista.turni_al_voto })}</b>{/if}
      </span>
    </div>
    <div class="trks">
      {#each tre as tracciato (tracciato.nome)}<Tracciato {tracciato} />{/each}
    </div>
    <div class="c-dx">
      {#if vista.finita}
        <span class="nessuno">{t("cornice.nessun_turno")}</span>
      {:else}
        <Pulsante forma="chiude" aria-busy={partita.occupato} onclick={chiudi}>{t("cornice.chiudi_il_turno")}</Pulsante>
        {#if rifiuto?.vista === vista}<Rifiuto testo={rifiuto.testo} />{/if}
      {/if}
    </div>
  </div>
  <Sommario {vista} {sezione} {impostazioni} {onimpostazioni} />
</header>
{#if stato === "crisi"}<div class="banda">{t("cornice.banda_crisi")}</div>{/if}
{#if partita.guasto}<p class="guasto" role="alert">{partita.guasto}</p>{/if}

<style>
  /* 104 px a 1× in ogni stato: la riga di sopra prende ciò che avanza, il sommario resta sul filetto */
  .cornice { flex: none; min-width: 0; min-height: 104px; display: grid; grid-template-rows: minmax(0, 1fr) auto; padding: 8px var(--marg) 0; border-bottom: var(--doppio) }
  .c1 { display: grid; grid-template-columns: auto minmax(0, 1fr) auto; align-items: center; gap: 18px; padding-bottom: 6px }
  .c-sx { display: flex; align-items: center; gap: 12px }
  /* 27 px: la misura del prototipo 08, fra --c-sez e --c-pag */
  .nomino { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; font-size: 27px; color: var(--rosso); line-height: 1; white-space: nowrap }
  .chi { font-size: var(--c-sec); line-height: 1.3; color: var(--grigio); white-space: nowrap }
  .chi b { color: var(--ink) }
  .trks { display: flex; justify-content: center; min-width: 0 }
  /* il rifiuto sta sotto il pulsante, fuori dal flusso: non sposta né il pulsante né i tracciati */
  .c-dx { position: relative; display: grid; justify-items: end }
  .c-dx :global(.rifiuto) { position: absolute; top: 100%; right: 0; z-index: 3; width: max-content; max-width: 44ch; background: var(--carta); text-align: right }
  .nessuno { font-size: var(--c-sec); color: var(--grigio); white-space: nowrap }

  .banda { flex: none; background: var(--ink); color: var(--carta); padding: 4px var(--marg); font-size: var(--c-sec); font-weight: 700; letter-spacing: .04em }
  .guasto { flex: none; padding: 4px var(--marg); border-bottom: var(--filo); font-size: var(--c-sec); font-weight: 700; color: var(--rosso) }

  /* forma stretta (meno di 1150 px utili): via la testatina; la variazione resta sotto il numero, in linea non entra */
  :global(:root.stretta) .nomino { display: none }
  :global(:root.stretta) .c1 { gap: 10px }
  /* 4 px e non gli 8 del prototipo: i tracciati veri sono più larghi (◂ ▸, pb, il ramo) e a 1024 px utili non entravano */
  :global(:root.stretta) .trks :global(.trk) { padding: 0 4px }
</style>
