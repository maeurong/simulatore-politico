<script lang="ts">
  // La prima pagina, forma A · Giornale composto (16-prima-pagina.prototype.html): testata, voce di testa, tre colonne, piede.
  // Ferma (Riprendi) è composta; dopo una chiusura le voci si rivelano una alla volta, ognuna al suo posto (piano 11f, lettura 4).
  import { untrack } from "svelte";
  import { casa, statoDi } from "$lib/ossatura";
  import { fattori, impostazioni, sistema } from "$lib/impostazioni.svelte";
  import { partita } from "$lib/partita.svelte";
  import { chiaveDi, titoloDi } from "$lib/testi/pungolo";
  import { t, type Chiave } from "$lib/testi/t";
  import type { Vista } from "$lib/tipi/Vista";
  import { grave, ordineRivelazione, pagina } from "./esito";
  import { rivelazione } from "./rivelazione.svelte";
  import Testa from "./Testa.svelte";
  import Voce from "./Voce.svelte";

  /** `id` si passa solo dove la partita non c'è (le prove); altrimenti lo dà l'indice della partita, per il titolo scelto con l'hash. */
  let { vista, id: idDato }: { vista: Vista; id?: string } = $props();

  let idLetto = $state<string | null>(null);
  $effect(() => {
    if (idDato !== undefined) return;
    let vivo = true;
    untrack(() => partita.indice()).then((i) => vivo && (idLetto = i?.id ?? ""), () => vivo && (idLetto = ""));
    return () => (vivo = false);
  });
  const id = $derived(idDato ?? idLetto);

  const g = $derived(grave(vista));
  const pag = $derived(pagina(vista.esito));
  const ordine = $derived(ordineRivelazione(vista.esito));
  const rivelata = (i: number) => rivelazione.ferma || ordine.indexOf(i) < rivelazione.passo;
  // fra le voci dello stesso tipo, nella pagina: è il «posto» del titolo (spec:319); nel tono grave il titolo non c'è
  const titolo = (i: number) => {
    if (g || id === null) return null;
    const chiave = chiaveDi(vista.esito[i]);
    const posto = ordine.slice(0, ordine.indexOf(i)).filter((k) => chiaveDi(vista.esito[k]) === chiave).length;
    return titoloDi(id, vista.esito[i], vista.turno, posto);
  };
  const k = $derived(fattori(impostazioni, sistema.riduce).k);
  $effect(() => {
    if (rivelazione.ferma) return;
    if (k === 0) return rivelazione.salta();
    const timer = setInterval(() => rivelazione.avanza(), 1100 * k);
    return () => clearInterval(timer);
  });
  // uscire a metà chiude la rivelazione: i tracciati vanno al valore nuovo, e tornando la pagina è ferma
  $effect(() => () => rivelazione.chiudi());

  const stato = $derived(statoDi(vista));
  const [vai, destinazione] = $derived(
    stato === "finita" ? (["prima.vai_eredita", "eredita"] as const)
    : stato === "trattativa" ? (["prima.vai_trattativa", "trattativa"] as const)
    : (["prima.vai_cruscotto", casa(vista)] as const),
  );
  const periodo = (d: Vista["data"]) => t(`cornice.trimestre_${d.trimestre}` as Chiave, { anno: d.anno });
  // il trimestre chiuso è quello prima di quello nuovo
  const chiuso = $derived(vista.data.trimestre === 1 ? { anno: vista.data.anno - 1, trimestre: 4 } : { anno: vista.data.anno, trimestre: vista.data.trimestre - 1 });
  const fuori = $derived(pag.fuori);
</script>

{#if id !== null}
  {#key vista}
    <!-- chi monta con la pagina già ferma non anima; una Vista nuova rimonta il blocco e rilegge, e untrack tiene il valore di quel momento -->
    {@const nuova = untrack(() => !rivelazione.ferma)}
    <!-- il clic che salta ha la sua via da tastiera: il pulsante «Salta la rivelazione» -->
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="giornale" onclick={() => rivelazione.ferma || rivelazione.salta()}>
      <header class="testata">
        <div class="orecchio">
          {t("prima.esito")}<br /><b>{periodo(chiuso as Vista["data"])}</b><br />{t("prima.conteggio", { n: ordine.length, fuori })}
        </div>
        <div class="nome">{t(g ? "prima.esito" : "prima.testata")}</div>
        <div class="orecchio dx">{t("prima.numero", { turno: vista.turno })}<br /><b>{periodo(vista.data)}</b><br />{g ? "" : t("prima.motto")}</div>
      </header>
      <div class="sotto"><span>{t(g ? "prima.esito_grave" : "prima.sotto_sx")}</span><span>{periodo(vista.data)}</span><span>{t(g ? "prima.sotto_grave" : "prima.sotto_dx")}</span></div>

      {#if pag.testa === null}
        <p class="vuota">{t("prima.nessuna_notizia")}</p>
      {:else}
        <Testa {vista} voce={vista.esito[pag.testa]} titolo={titolo(pag.testa)} rivelata={rivelata(pag.testa)} {nuova} />
        {#if pag.colonne.length}
          <div class="colonne">
            {#each pag.colonne as i (i)}<Voce {vista} voce={vista.esito[i]} titolo={titolo(i)} rivelata={rivelata(i)} {nuova} />{/each}
          </div>
        {/if}
      {/if}

      <footer class="piede">
        <span class="sx">
          <a class="link" href="/partita/registro">{t(pag.critFuori ? "prima.altre_critiche" : "prima.altre", { n: fuori, k: pag.critFuori })}</a>
          {#if !rivelazione.ferma}<button class="salta" type="button" onclick={() => rivelazione.salta()}>{t("prima.salta")}</button>{/if}
        </span>
        <a class="btn pieno" href="/partita/{destinazione}">{t(vai)}</a>
      </footer>
    </div>
  {/key}
{/if}

<style>
  .giornale { display: grid; grid-template-columns: minmax(0, 1fr); align-content: start; min-width: 0; padding-top: 0 }
  .testata { display: grid; grid-template-columns: 1fr auto 1fr; align-items: end; padding-bottom: 3px }
  .nome { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 800; font-size: var(--c-test); line-height: 1.04; color: var(--rosso); text-align: center }
  :global(:root.grave) .nome { font-size: var(--c-pag); font-weight: 600; padding: 12px 0 4px; color: var(--ink) }
  .orecchio { font-size: var(--c-sec); line-height: 1.35; color: var(--grigio) }
  .orecchio b { color: var(--ink) }
  .orecchio.dx { text-align: right }
  .sotto {
    display: flex; justify-content: space-between; gap: 12px; font-size: var(--c-min); letter-spacing: .08em; text-transform: uppercase;
    border-top: 1px solid var(--ink); border-bottom: var(--doppio); padding: 3px 0; color: var(--grigio);
  }
  .colonne { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); grid-auto-rows: min-content; padding-top: 8px; row-gap: 14px }
  .colonne > :global(.voce) { padding: 2px 16px; border-right: var(--filo) }
  .colonne > :global(.voce:nth-child(3n + 1)) { padding-left: 0 }
  .colonne > :global(.voce:nth-child(3n)) { border-right: 0; padding-right: 0 }
  .vuota { color: var(--grigio); font-size: var(--c-sec); padding: 14px 0 }
  .piede { display: flex; justify-content: space-between; align-items: center; gap: 12px; border-top: var(--doppio); padding: 7px 0 4px; margin-top: 12px; position: sticky; bottom: 0; background: var(--carta); box-shadow: 0 var(--s4) 0 var(--carta) }
  .sx { display: flex; gap: 14px; align-items: center }
  .salta { font-size: var(--c-min); color: var(--grigio); text-decoration: underline; text-underline-offset: 2px }
  a { color: inherit }
  .btn { border: var(--forte); padding: 4px 11px; font-size: var(--c-corpo); font-weight: 700; white-space: nowrap }
  .btn.pieno { background: var(--ink); color: var(--carta) }
  .btn.pieno:hover { background: var(--carta); color: var(--ink) }
  .link { font-size: var(--c-sec); text-decoration: underline; text-underline-offset: 2px; white-space: nowrap }
  .link:hover { text-decoration-thickness: 2px }
</style>
