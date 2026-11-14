<script lang="ts">
  // La spartizione, forma A · Righe (27-trattativa.prototype.html): un dicastero per riga, a chi in bottoni; a bozza completa parte il comando.
  import { untrack } from "svelte";
  import { t, numero } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { coloreDi, nomeDi } from "$lib/partiti";
  import { comeErrore, testoRifiuto } from "$lib/guscio/chiamate";
  import type { Vista } from "$lib/tipi/Vista";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import { aChi, pesoDi, proponi, type Bozza } from "./trattativa";

  let { vista }: { vista: Vista } = $props();

  // la bozza vive in memoria: al rientro vale ciò che dice la Vista (spec:250)
  let bozza = $state.raw<Bozza>({});
  // raw: il rifiuto vale per la Vista in cui è nato, e sta sotto la riga cliccata
  let rifiuto = $state.raw<{ dicastero: string; testo: string; vista: Vista | null } | null>(null);
  let ultimo = "";
  let pendente = false;

  const assegnati = $derived(vista.dicasteri.filter((d) => bozza[d.id] !== undefined).length);
  const chi = $derived(aChi(vista));
  const nome = (id: string | null) => (id === null ? t("trattativa.tecnico") : id === "giocatore" ? t("trattativa.tu") : nomeDi(vista, id));

  /** Un comando solo: se la partita è occupata la bozza resta pendente e riparte quando si libera. */
  async function invia() {
    pendente = false;
    const comando = proponi(vista, bozza);
    if (!comando) return;
    if (partita.occupato) {
      pendente = true;
      return;
    }
    try {
      await partita.comanda(comando);
    } catch (e) {
      rifiuto = { dicastero: ultimo, testo: testoRifiuto(comeErrore(e)), vista: partita.vista };
    }
  }
  $effect(() => {
    if (!partita.occupato && untrack(() => pendente)) untrack(invia);
  });

  function scegli(dicastero: string, a: string | null) {
    if (bozza[dicastero] === a && partita.occupato) return;
    rifiuto = null;
    ultimo = dicastero;
    bozza = { ...bozza, [dicastero]: a };
    invia();
  }
</script>

<p class="min muto" aria-live="polite">{assegnati === vista.dicasteri.length ? t("trattativa.proposta_in_corso") : t("trattativa.assegnati", { n: assegnati, tot: vista.dicasteri.length })}</p>
<table class="rig">
  <thead><tr><th scope="col">{t("trattativa.col_dicastero")}</th><th scope="col">{t("trattativa.col_peso")}</th><th scope="col">{t("trattativa.col_a_chi")}</th></tr></thead>
  <tbody>
    {#each vista.dicasteri as d (d.id)}
      {@const peso = pesoDi(d.id)}
      <tr>
        <th scope="row" class="d">{d.nome}</th>
        <td><span class="pesi" role="img" aria-label={t("trattativa.peso", { n: peso })} title={t("trattativa.peso", { n: peso })}>{#each { length: peso } as _, i (i)}<i></i>{/each}</span></td>
        <td>
          <span class="chi2" role="group" aria-label="{d.nome}: {t('trattativa.col_a_chi')}">
            {#each chi as a (a)}
              <button type="button" class="op" class:tec={a === null} class:nato={a !== null && coloreDi(a) === null} aria-pressed={bozza[d.id] === a} style:--pc={a === null ? undefined : (coloreDi(a) ?? undefined)} onclick={() => scegli(d.id, a)}>{nome(a)}</button>
            {/each}
          </span>
        </td>
      </tr>
      {#if rifiuto?.dicastero === d.id && rifiuto.vista === partita.vista}
        <tr><td colspan="3"><Rifiuto testo={rifiuto.testo} /></td></tr>
      {/if}
    {/each}
  </tbody>
</table>
<div class="tot">
  <span class="min muto">{t("trattativa.punti_alleati")}</span>
  {#each vista.richieste as r (r.partito)}
    <span><i class="sw" style:background={coloreDi(r.partito) ?? undefined} class:nato={coloreDi(r.partito) === null}></i>{nomeDi(vista, r.partito)} <b>{numero(r.offerta)}</b> <span class="min muto">{t("trattativa.chiede", { x: numero(r.richiesta, 1) })}</span></span>
  {/each}
</div>
<p class="min muto">{t("trattativa.nota")}</p>
<Segnaposto sezione={t("trattativa.ph_default")} fetta="11g" />

<style>
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .rig { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .rig th { font-size: var(--c-min); letter-spacing: .08em; text-transform: uppercase; color: var(--grigio); text-align: left; font-weight: 700; border-bottom: 1.5px solid var(--ink); padding: 2px 6px; white-space: nowrap }
  .rig tbody th.d { font-size: var(--c-sec); letter-spacing: 0; text-transform: none; color: var(--ink); border-bottom: var(--filo); padding: 4px 6px }
  .rig td { border-bottom: var(--filo); padding: 4px 6px; vertical-align: middle }
  .pesi { display: inline-flex; gap: 2px; vertical-align: middle }
  .pesi i { width: 7px; height: 7px; border-radius: 50%; background: var(--ink) }
  .chi2 { display: inline-flex; flex-wrap: wrap; gap: 3px }
  .op { border: 1.5px solid var(--ink); padding: 1px 8px; font-size: var(--c-min); font-weight: 700; background: var(--carta); white-space: nowrap }
  .op:hover { background: var(--velo) }
  .op[aria-pressed="true"] { color: var(--carta); background: var(--pc, var(--ink)); border-color: var(--pc, var(--ink)) }
  .op.tec[aria-pressed="true"] { background: var(--retino), var(--carta); color: var(--ink); border: 3px double var(--ink); padding: 0 6px }
  .op.nato[aria-pressed="true"] { background: var(--retino), var(--carta); color: var(--ink); border-color: var(--ink) }
  .tot { display: flex; gap: 6px 16px; font-size: var(--c-sec); flex-wrap: wrap; align-items: baseline }
  .tot b { font-size: var(--c-num) }
  .sw { display: inline-block; width: 10px; height: 10px; border: 1px solid var(--ink); margin-right: 5px; vertical-align: -1px }
  .sw.nato { background: var(--retino), var(--carta) }
</style>
