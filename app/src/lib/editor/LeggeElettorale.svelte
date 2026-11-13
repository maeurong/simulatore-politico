<script lang="ts">
  // Le regole del voto (14-editor.prototype.html:327-338): il menu della legge, il rifiuto del core sopra i nove
  // parametri, i parametri a bottoni col gradino della legge scelta, sotto la Costituzione.
  import { t, type Chiave } from "$lib/testi/t";
  import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
  import type { SuMisura } from "$lib/tipi/SuMisura";
  import Disposizioni from "./Disposizioni.svelte";
  import { base, cambiaLegge } from "./editor.svelte";

  type Props = {
    catalogo: CatalogoScenario;
    scenario: SuMisura | null;
    /** il testo del core, se l'ultimo scenario chiesto non regge */
    rifiuto: string | null;
    onscenario: (s: SuMisura) => void;
  };
  let { catalogo, scenario, rifiuto, onscenario }: Props = $props();

  const NOTE = ["legge_180", "proporzionale_3", "proporzionale_puro"];
  const nomeLegge = (n: string) => (NOTE.includes(n) ? t(`editor.${n}` as Chiave) : n);
  const legge = $derived(scenario?.cursori.legge_elettorale ?? catalogo.legge_di_base);
  const elettorali = $derived(catalogo.disposizioni.filter((d) => d.norma === "elettorale"));
  const costituzionali = $derived(catalogo.disposizioni.filter((d) => d.norma === "costituzionale"));
  const ritocchi = $derived(elettorali.filter((d) => scenario && d.id in scenario.disposizioni).length);
</script>

<div class="param">
  <label class="nm" for="legge-elettorale"><b>{t("editor.legge_elettorale")}</b><br /><span class="era">{t("editor.punto_di_partenza")}</span></label>
  <select id="legge-elettorale" value={legge}
    onchange={(e) => onscenario(cambiaLegge(scenario ?? base(catalogo), catalogo, e.currentTarget.value))}>
    {#each Object.keys(catalogo.leggi_elettorali) as n (n)}<option value={n}>{nomeLegge(n)}</option>{/each}
  </select>
</div>
{#if rifiuto}
  <div class="rifiuto" role="alert">{t("editor.legge_non_regge")}<span>{t("editor.riepilogo_fermo", { testo: rifiuto })}</span></div>
{/if}
<Disposizioni {catalogo} {scenario} voci={elettorali} {onscenario} />
{#if ritocchi}<p class="nota">{t("editor.ritocchi", { n: ritocchi, legge: nomeLegge(legge) })}</p>{/if}
<h3 class="occ">{t("editor.costituzione")}</h3>
<Disposizioni {catalogo} {scenario} voci={costituzionali} {onscenario} />

<style>
  .param { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 4px 12px; align-items: center; font-size: var(--c-sec); padding: 3px 0; border-bottom: var(--filo) }
  .era { font-size: var(--c-min); color: var(--grigio) }
  select { font: inherit; font-size: var(--c-sec); border: 1.5px solid var(--ink); background: var(--carta); padding: 1px 4px; color: var(--ink) }
  .rifiuto { border: 2px solid var(--rosso); color: var(--rosso); padding: 6px 10px; font-size: var(--c-sec); font-weight: 700; margin-top: 4px }
  .rifiuto span { display: block; font-weight: 400; color: var(--ink); margin-top: 2px }
  .nota { font-size: var(--c-min); color: var(--grigio) }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; margin-top: 8px }
</style>
