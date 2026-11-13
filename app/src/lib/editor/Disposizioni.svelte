<script lang="ts">
  // Righe di disposizioni coi gradini (14-editor.prototype.html:324-346): bottoni affiancati fino a quattro gradini,
  // oltre un menu; il pallino sul gradino senza ritocco. Raggruppate per dicastero se `gruppi`.
  import { numero, segnato, t } from "$lib/testi/t";
  import type { CatalogoScenario } from "$lib/tipi/CatalogoScenario";
  import type { SuMisura } from "$lib/tipi/SuMisura";
  import type { VoceCatalogo } from "$lib/tipi/VoceCatalogo";
  import { diPartenza, gradino, scegli } from "./editor.svelte";

  type Props = {
    catalogo: CatalogoScenario;
    scenario: SuMisura | null;
    voci: VoceCatalogo[];
    gruppi?: boolean;
    /** spento (salvataggio in volo): i comandi restano nel Tab e non cambiano niente */
    fermo?: boolean;
    onscenario: (s: SuMisura) => void;
  };
  let { catalogo, scenario, voci, gruppi = false, fermo = false, onscenario }: Props = $props();

  const elenco = $derived.by(() => {
    if (!gruppi) return [{ nome: null as string | null, voci }];
    const m = new Map<string, VoceCatalogo[]>();
    for (const d of voci) {
      const g = d.dicastero ?? t("editor.regioni");
      m.set(g, [...(m.get(g) ?? []), d]);
    }
    return [...m].map(([nome, voci]) => ({ nome, voci }));
  });

  const nomeDi = (d: VoceCatalogo, id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
  const saldoDi = (d: VoceCatalogo, id: string) => d.gradini.find((g) => g.id === id)?.saldo ?? 0;

  /** Cosa dice la riga ritoccata: per le elettorali il gradino della legge scelta, per le altre quello di oggi e il saldo. */
  function era(d: VoceCatalogo, g: string, p: string): string {
    if (d.norma === "elettorale") return t("editor.la_legge_dice", { gradino: nomeDi(d, p) });
    const s = saldoDi(d, g) - saldoDi(d, p);
    if (!/[1-9]/.test(numero(Math.abs(s), 2))) return t("editor.oggi_gradino", { gradino: nomeDi(d, p) });
    return t("editor.oggi_gradino_saldo", { gradino: nomeDi(d, p), saldo: segnato(s, 2) });
  }
  const partenzaDetta = (d: VoceCatalogo) => t(d.norma === "elettorale" ? "editor.della_legge" : "editor.in_vigore_oggi");
</script>

{#each elenco as gr (gr.nome)}
  <div class="gruppo">
    {#if gr.nome}<h3 class="occ">{gr.nome}</h3>{/if}
    {#each gr.voci as d (d.id)}
      {@const g = gradino(scenario, catalogo, d)}
      {@const p = diPartenza(scenario, catalogo, d)}
      <div class={["param", { mosso: g !== p }]}>
        <span class="nm">{d.nome}{#if g !== p}{" "}<span class="era">{era(d, g, p)}</span>{/if}</span>
        {#if d.gradini.length > 4}
          <select aria-label={d.nome} value={g} aria-disabled={fermo || undefined}
            onchange={(e) => (fermo ? (e.currentTarget.value = g) : onscenario(scegli(scenario, catalogo, d, e.currentTarget.value)))}>
            {#each d.gradini as x (x.id)}<option value={x.id}>{x.nome}{x.id === p ? " •" : ""}</option>{/each}
          </select>
        {:else}
          <span class="seg" role="group" aria-label={d.nome}>
            {#each d.gradini as x (x.id)}
              <button type="button" class={{ part: x.id === p }} aria-pressed={x.id === g} aria-disabled={fermo || undefined}
                onclick={() => fermo || onscenario(scegli(scenario, catalogo, d, x.id))}>{x.nome}{#if x.id === p}<span class="nascosto">, {partenzaDetta(d)}</span>{/if}</button>
            {/each}
          </span>
        {/if}
      </div>
    {/each}
  </div>
{/each}

<style>
  .gruppo { display: grid }
  .occ { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; font-weight: 700; padding: 6px 0 2px; border-bottom: var(--bordo) }
  .param { display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 4px 12px; align-items: center; font-size: var(--c-sec); padding: 3px 0; border-bottom: var(--filo) }
  .param.mosso { background: linear-gradient(90deg, var(--velo), transparent) }
  .era { font-size: var(--c-min); color: var(--grigio) }
  .seg { display: inline-flex; flex-wrap: wrap; justify-content: flex-end; border: 1.5px solid var(--ink); min-width: 0 }
  .seg button { padding: 1px 8px; font-size: var(--c-min); font-weight: 700; border-left: 1.5px solid var(--ink); white-space: nowrap }
  .seg button:first-child { border-left: 0 }
  .seg button:hover { background: var(--velo) }
  .seg button[aria-disabled="true"], select[aria-disabled="true"] { opacity: .5; cursor: default }
  .seg button[aria-pressed="true"] { background: var(--ink); color: var(--carta) }
  .seg button.part::after { content: "•"; color: var(--rosso); margin-left: 3px }
  .seg button[aria-pressed="true"].part::after { color: var(--giallo) }
  select { font: inherit; font-size: var(--c-sec); border: 1.5px solid var(--ink); background: var(--carta); padding: 1px 4px; color: var(--ink) }
</style>
