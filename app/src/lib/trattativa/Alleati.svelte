<script lang="ts">
  // Il dettaglio a lato (27-trattativa.prototype.html, «lAllarga» e «lAlleati»): l'allargamento in testa, poi ogni alleato con ciò che chiede.
  import { t, numero, elenco } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { coloreDi } from "$lib/partiti";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Caselle from "$lib/componenti/Caselle.svelte";
  import Conferma from "$lib/componenti/Conferma.svelte";
  import Faccia from "$lib/componenti/Faccia.svelte";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import { allarga, fuori, haLaMaggioranza, pesoDi, quota, richiesteDi, type Bozza, type Sessione } from "./trattativa";

  let { vista, bozza, fatta, sessione }: { vista: Vista; bozza: Bozza; fatta: boolean; sessione: Sessione } = $props();

  const maggioranza = $derived(haLaMaggioranza(vista));
  const fuoriCoalizione = $derived(fuori(vista));
  const disponibili = $derived(fuoriCoalizione.filter((x) => x.disponibile));
  const alleati = $derived(richiesteDi(vista));
  const totale = $derived(vista.dicasteri.reduce((s, d) => s + pesoDi(d.id), 0));
  const mio = $derived(quota(vista, bozza, "giocatore"));
  const tecnici = $derived(quota(vista, bozza, null));
  const leader = (id: string) => vista.partiti.find((p) => p.id === id)?.leader ?? "";
  const spento = $derived(partita.occupato ? t("trattativa.comando_in_corso") : undefined);

  // raw: il rifiuto vale per la Vista in cui è nato
  let rifiuto = $state.raw<{ chi: string; testo: string; vista: Vista | null } | null>(null);
  let radice = $state<HTMLElement>();

  /** Riuscito, la Vista nuova ha alleati nuovi: la bozza riparte. La conferma sparisce con la sua riga: qui il fuoco passa al riquadro degli alleati. */
  async function manda(ids: string[]) {
    if (partita.occupato) return;
    rifiuto = null;
    try {
      if (await partita.comanda(allarga(ids))) {
        sessione.svuota();
        radice?.focus();
      }
    } catch (e) {
      rifiuto = { chi: ids.join("+"), testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
  const chi = (ids: string[]) => elenco(ids.map((id) => fuoriCoalizione.find((x) => x.id === id)?.nome ?? id), ", ");
  const motivo = (ids: string[]) => (rifiuto?.chi === ids.join("+") && rifiuto.vista === partita.vista ? rifiuto.testo : null);
</script>

{#if maggioranza ? disponibili.length : fuoriCoalizione.length}
  <section class="blocco" aria-labelledby="all-t">
    <h4 class="sez-l" id="all-t">{t("trattativa.allargare")}</h4>
    {#if maggioranza}
      <p class="sec">{t("trattativa.puoi_allargare", { n: disponibili.length, chi: elenco(disponibili.map((x) => x.nome), ", ") })}</p>
    {:else}
      <p class="rif">{t("trattativa.senza_maggioranza")}</p>
      {#each fuoriCoalizione as x (x.id)}
        <div class="fu" class:no={!x.disponibile}>
          <span>
            <i class="sw" style:background={coloreDi(x.id) ?? undefined} class:nato={coloreDi(x.id) === null}></i><b>{x.nome}</b>
            <span class="min">{t("trattativa.seggi_fuori", { camera: x.camera, senato: x.senato })}</span><br />
            <span class="min" class:muto={!x.disponibile}>{x.disponibile ? t("trattativa.disponibile") : t("trattativa.non_entra")}: {x.causa}</span>
          </span>
          {#if x.disponibile}
            <Conferma neutro etichetta={t("trattativa.allarga")} conferma={t("trattativa.conferma_allarga")} {spento}
              spiegazione={t("trattativa.spiega_allarga", { n: 1, chi: x.nome })} onconferma={() => manda([x.id])} />
          {/if}
          {#if motivo([x.id])}<span class="largo"><Rifiuto testo={motivo([x.id])!} /></span>{/if}
        </div>
      {/each}
      {#if disponibili.length > 1}
        {@const ids = disponibili.map((x) => x.id)}
        <div class="az">
          <Conferma neutro etichetta={t(disponibili.length === 2 ? "trattativa.allarga_tutti_due" : "trattativa.allarga_tutti")} conferma={t("trattativa.conferma_allarga")} {spento}
            spiegazione={t("trattativa.spiega_allarga", { n: ids.length, chi: chi(ids) })} onconferma={() => manda(ids)} />
          {#if motivo(ids)}<Rifiuto testo={motivo(ids)!} />{/if}
        </div>
      {/if}
    {/if}
  </section>
{/if}

<section class="blocco" aria-labelledby="ali-t" tabindex="-1" bind:this={radice}>
  <h4 class="sez-l" id="ali-t">{t("trattativa.gli_alleati")}</h4>
  {#each alleati as a (a.id)}
    <div class="all" style:--pc={coloreDi(a.id) ?? undefined} class:nato={coloreDi(a.id) === null}>
      <span class="fz"><Faccia nome={leader(a.id)} ruolo="leader" partito={a.nome} misura={34} /></span>
      <span><b>{a.nome}</b> <span class="min muto">{leader(a.id)}</span></span>
      <span class="sec">{t("trattativa.chiede_offri", { x: numero(a.richiesta, 1), o: numero(a.offerta) })}</span>
      <span class="largo"><Caselle piene={a.offerta} su={totale} filo={Math.ceil(a.richiesta)} rosso={a.scarto < 0} nome="" /></span>
    </div>
  {/each}
  <Segnaposto sezione={t("trattativa.ph_intesa")} fetta="11g" />
</section>

<section class="blocco" aria-labelledby="mio-t">
  <h4 class="sez-l" id="mio-t">{t("trattativa.il_tuo_partito")}</h4>
  {#if Object.keys(bozza).length}
    <p class="sec">
      <b>{t("trattativa.tuoi_dicasteri", { n: mio.n })}</b> · {t("trattativa.tuoi_punti", { n: mio.punti, tot: totale })}
      {#if tecnici.n}<br /><span class="min muto">{t("trattativa.dicasteri_tecnici", { n: tecnici.n })}</span>{/if}
    </p>
    <div class="az">
      <Pulsante forma="piccolo" {spento} onclick={() => sessione.svuota()}>{t("trattativa.svuota")}</Pulsante>
    </div>
  {/if}
  <p class="min muto">{t(fatta ? "trattativa.svuota_nota" : "trattativa.nota")}</p>
</section>

<style>
  .blocco { display: grid; gap: 6px; align-content: start }
  .blocco:focus { outline: none }
  .sez-l { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; color: var(--grigio); border-bottom: 1.5px solid var(--ink); padding-bottom: 2px }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .rif { border: 1.5px solid var(--rosso); color: var(--rosso); padding: 5px 8px; font-size: var(--c-sec); font-weight: 700 }
  .fu { border: var(--filo); padding: 5px 7px; display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 8px; align-items: center; font-size: var(--c-sec) }
  .fu.no { color: var(--grigio) }
  .fu .largo { grid-column: 1 / -1 }
  .az { display: flex; flex-wrap: wrap; gap: 4px 8px; align-items: center }
  .sw { display: inline-block; width: 10px; height: 10px; border: 1px solid var(--ink); margin-right: 5px; vertical-align: -1px }
  .sw.nato { background: var(--retino), var(--carta) }
  .all { border: var(--bordo); border-top: 5px solid var(--pc, var(--ink)); padding: 6px 8px; display: grid; grid-template-columns: auto minmax(0, 1fr); gap: 3px 8px }
  .all.nato { border-top-color: var(--ink) }
  .all > :not(.fz, .largo) { grid-column: 2 }
  .all .fz { grid-row: 1 / span 2; display: inline-flex }
  .all .largo { grid-column: 1 / -1 }
</style>
