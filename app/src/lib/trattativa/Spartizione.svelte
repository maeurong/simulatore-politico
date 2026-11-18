<script lang="ts">
  // La spartizione, forma A · Righe (27-trattativa.prototype.html): un dicastero per riga, a chi in bottoni; ogni clic manda la spartizione intera.
  import { t } from "$lib/testi/t";
  import { partita } from "$lib/partita.svelte";
  import { coloreDi, nomeDi, testoSu } from "$lib/partiti";
  import type { Comando } from "$lib/tipi/Comando";
  import type { Vista } from "$lib/tipi/Vista";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";
  import { aChi, proponi, type EsitoManda } from "./trattativa";

  let { vista, manda }: { vista: Vista; manda: (c: Comando) => Promise<EsitoManda> } = $props();

  // raw: il rifiuto vale per la Vista in cui è nato
  let rifiutato = $state.raw<{ dicastero: string; testo: string; vista: Vista | null } | null>(null);
  const rifiuto = $derived(rifiutato?.vista === partita.vista ? rifiutato : null);

  const chi = $derived(aChi(vista));
  const nome = (id: string | null) => (id === null ? t("trattativa.tecnico") : id === "giocatore" ? t("trattativa.tu") : nomeDi(vista, id));

  async function scegli(dicastero: string, a: string | null) {
    rifiutato = null;
    const e = await manda(proponi(vista, dicastero, a));
    if (typeof e === "object") rifiutato = { dicastero, testo: e.rifiuto, vista: partita.vista };
  }
</script>

<table class="rig">
  <thead><tr><th scope="col">{t("trattativa.col_dicastero")}</th><th scope="col">{t("trattativa.col_peso")}</th><th scope="col">{t("trattativa.col_a_chi")}</th></tr></thead>
  <tbody>
    {#each vista.spartizione as d (d.dicastero)}
      {@const peso = d.peso}
      {@const nomeD = vista.dicasteri.find((x) => x.id === d.dicastero)?.nome ?? d.dicastero}
      <tr>
        <th scope="row" class="d">{nomeD}</th>
        <td><span class="pesi" role="img" aria-label={t("trattativa.peso", { n: peso })} title={t("trattativa.peso", { n: peso })}>{#each { length: peso } as _, i (i)}<i></i>{/each}</span></td>
        <td>
          <span class="chi2" role="group" aria-label="{nomeD}: {t('trattativa.col_a_chi')}">
            {#each chi as a (a)}
              <button type="button" class="op" class:tec={a === null} class:nato={a !== null && coloreDi(a) === null} aria-pressed={d.partito === a} style:--pc={a === null ? undefined : (coloreDi(a) ?? undefined)} style:--tx={a === null ? undefined : testoSu(a)} onclick={() => scegli(d.dicastero, a)}>{nome(a)}</button>
            {/each}
          </span>
        </td>
      </tr>
      {#if rifiuto?.dicastero === d.dicastero}
        <tr><td colspan="3"><Rifiuto testo={rifiuto.testo} /></td></tr>
      {/if}
    {/each}
  </tbody>
</table>
<Segnaposto sezione={t("trattativa.ph_default")} fetta="11g" />

<style>
  .rig { width: 100%; border-collapse: collapse; font-size: var(--c-sec) }
  .rig th { font-size: var(--c-min); letter-spacing: .08em; text-transform: uppercase; color: var(--grigio); text-align: left; font-weight: 700; border-bottom: 1.5px solid var(--ink); padding: 2px 6px; white-space: nowrap }
  .rig tbody th.d { font-size: var(--c-sec); letter-spacing: 0; text-transform: none; color: var(--ink); border-bottom: var(--filo); padding: 4px 6px }
  .rig td { border-bottom: var(--filo); padding: 4px 6px; vertical-align: middle }
  .pesi { display: inline-flex; gap: 2px; vertical-align: middle }
  .pesi i { width: 7px; height: 7px; border-radius: 50%; background: var(--ink) }
  .chi2 { display: inline-flex; flex-wrap: wrap; gap: 3px }
  .op { border: 1.5px solid var(--ink); padding: 1px 8px; font-size: var(--c-min); font-weight: 700; background: var(--carta); white-space: nowrap }
  .op:hover { background: var(--velo) }
  .op[aria-pressed="true"] { color: var(--tx, var(--carta)); background: var(--pc, var(--ink)); border-color: var(--pc, var(--ink)) }
  :global(:root.grave) .op:not(.tec, .nato)[aria-pressed="true"] { color: var(--carta) }
  .op.tec[aria-pressed="true"] { background: var(--retino), var(--carta); color: var(--ink); border: 3px double var(--ink); padding: 0 6px }
  .op.nato[aria-pressed="true"] { background: var(--retino), var(--carta); color: var(--ink); border-color: var(--ink) }
</style>
