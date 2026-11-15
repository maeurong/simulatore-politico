<script lang="ts">
  // La legge elettorale (storia 109): in testa le impostazioni pronte del catalogo, che riempiono la bozza; sotto le nove disposizioni coi gradini
  // tutti aperti. A ogni bozza il core dice quali gradini non regge (`menu_elettorale`): una richiesta in volo per volta, vince l'ultima.
  import { onMount, untrack } from "svelte";
  import { t, type Chiave } from "$lib/testi/t";
  import { guscio, testoRifiuto } from "$lib/guscio/chiamate";
  import { ultimoVince } from "$lib/editor/editor.svelte";
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceDisposizione } from "$lib/tipi/VoceDisposizione";
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import type { Bozza } from "./bozza";
  import { conLegge, eLegge, spenti, type LeggePronta } from "./elettorale";
  import Scala from "./Scala.svelte";

  let { vista, bozza, oncambia, onbozza }: {
    vista: Vista; bozza: Bozza; oncambia: (disposizione: string, gradino: string) => void; onbozza: (b: Bozza) => void;
  } = $props();

  const NOTE = ["legge_180", "proporzionale_3", "proporzionale_puro"];
  const nomeLegge = (n: string) => (NOTE.includes(n) ? t(`editor.${n}` as Chiave) : n);
  const disposizioni = $derived(vista.disposizioni.filter((d) => d.norma === "elettorale"));
  const nomeGradino = (d: VoceDisposizione, id: string) => d.gradini.find((g) => g.id === id)?.nome ?? id;
  const soglia = $derived(disposizioni.find((d) => d.id === "soglia_premio")?.gradini[0]?.nome);

  let leggi = $state.raw<Record<string, LeggePronta>>({});
  let errore = $state<string | null>(null);
  onMount(() => {
    guscio.catalogoScenario().then((c) => (leggi = c.leggi_elettorali), (e) => (errore = testoRifiuto(e)));
  });
  // una pronta già composta dalla bozza la toglie, un'altra prende il posto della parte elettorale
  const scegli = (legge: LeggePronta) => onbozza(conLegge(vista, bozza, eLegge(vista, bozza, legge) ? {} : legge));

  let spento = $state.raw<Record<string, string>>({});
  let ultima = "";
  const chiedi = ultimoVince(
    (b: Bozza) => guscio.menuElettorale(b),
    (r, b) => {
      if (JSON.stringify(b) !== ultima) return;
      if ("specie" in r) {
        spento = {};
        errore = testoRifiuto(r);
      } else {
        spento = spenti(r);
        errore = null;
      }
    },
  );
  $effect(() => {
    const b = bozza;
    ultima = JSON.stringify(b);
    untrack(() => chiedi(b));
  });
</script>

<div class="el">
  <div class="pronte" role="group" aria-label={t("aula.pronte_gruppo")}>
    <span class="sec muto">{t("aula.pronte")}</span>
    {#each Object.entries(leggi) as [n, legge] (n)}
      <Pulsante forma="piccolo" scelto={eLegge(vista, bozza, legge)} onclick={() => scegli(legge)}>{nomeLegge(n)}</Pulsante>
    {/each}
  </div>
  {#if errore}<Rifiuto testo={errore} />{/if}
  <div class="tb" role="group" aria-label={t("aula.elettorali")}>
    {#each disposizioni as d (d.id)}
      <div class="dr">
        <span class="nm">{d.nome}<small>{d.legge}{d.precedente !== d.in_vigore ? ` · ${t("aula.disp_prima", { gradino: nomeGradino(d, d.precedente) })}` : ""}</small></span>
        <Scala {d} {bozza} onscegli={(g) => oncambia(d.id, g)} spenti={spento} />
      </div>
    {/each}
  </div>
  {#if soglia}<p class="min muto">{t("aula.elettorale_nota", { soglia })}</p>{/if}
</div>

<style>
  .el { display: grid; gap: 6px; align-content: start }
  .pronte { display: flex; gap: 6px; align-items: center; flex-wrap: wrap }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .tb { display: grid }
  .dr { display: grid; grid-template-columns: minmax(150px, 28%) minmax(0, 1fr); gap: 2px 12px; align-items: center; padding: 5px 4px; border-bottom: var(--filo); font-size: var(--c-sec) }
  .nm { font-weight: 700; line-height: 1.2 }
  .nm small { display: block; font-weight: 400; color: var(--grigio); font-size: var(--c-min) }
</style>
