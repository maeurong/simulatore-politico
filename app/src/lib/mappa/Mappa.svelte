<script module lang="ts">
  export type Segno = "cantiere" | "atto" | "opera" | "emergenza";
</script>

<script lang="ts">
  // Le venti regioni: un radiogroup SVG con una sola fermata di Tab (ricerca: mappa-svg-delle-regioni.md:112-115).
  // Il colore non porta da solo il valore: classe e segni stanno nel nome accessibile.
  import { numero, t } from "$lib/testi/t";
  import { REGIONI, TELA } from "./regioni";
  import { accanto, classi } from "./mappa";
  import FormaSegno from "./FormaSegno.svelte";

  let {
    valori, segni = {}, scelta, ordine, nome, piccola = false, solaLettura = false, formato = (x) => numero(x, 1), classe: classeFuori, tinta, etichetta, onscelta,
  }: {
    valori: Record<string, number | null>;
    segni?: Record<string, Segno[]>;
    scelta: string | null;
    ordine: string[];
    nome: string;
    piccola?: boolean;
    solaLettura?: boolean;
    formato?: (x: number, id: string) => string;
    /** Le classi già decise (pericolosità a passi fissi); senza, passi uguali fra il minimo e il massimo dei valori. */
    classe?: Record<string, 0 | 1 | 2 | 3 | 4 | null>;
    /** Un colore per regione che vince sulla classe (il voto); null = retino. */
    tinta?: Record<string, string | null>;
    /** Il valore a parole di una regione, al posto di «valore, classe n di 5». */
    etichetta?: (id: string) => string;
    onscelta: (id: string) => void;
  } = $props();

  const pre = $props.id();
  const classe = $derived(classeFuori ?? Object.fromEntries(ordine.map((id, i) => [id, classi(ordine.map((x) => valori[x] ?? null))[i]])));
  // mobile: la fermata di Tab sta sul fuoco, o sulla scelta, o sulla prima
  let fuoco = $state<string | null>(null);
  let daTastiera = $state(false);
  const fermata = $derived(fuoco ?? scelta ?? ordine[0]);
  let gruppo: SVGGElement;

  const nomeAccessibile = (id: string, nomeReg: string) => {
    const x = valori[id] ?? null, c = classe[id];
    const dove = etichetta ? etichetta(id) : x === null || c === null ? t("mappa.senza_dato") : `${formato(x, id)}, ${t("mappa.classe", { n: c + 1 })}`;
    return [nomeReg, dove, ...(segni[id] ?? []).map((s) => t(`mappa.segno_${s}`))].join(", ");
  };

  function tasto(e: KeyboardEvent, id: string) {
    const verso = e.key === "ArrowRight" || e.key === "ArrowDown" ? 1 : e.key === "ArrowLeft" || e.key === "ArrowUp" ? -1 : 0;
    if (verso) {
      e.preventDefault();
      gruppo.querySelector<SVGElement>(`[data-reg="${accanto(id, ordine, verso)}"]`)?.focus();
    } else if ((e.key === " " || e.key === "Enter") && !solaLettura) {
      e.preventDefault();
      onscelta(id);
    }
  }
  const x0 = (n: number, i: number) => (i - (n - 1) / 2) * 13;
</script>

<svg viewBox="0 0 {TELA.w} {TELA.h}" class:piccola role="radiogroup" aria-label={nome} aria-readonly={solaLettura || undefined}>
  <defs>
    <pattern id="{pre}-tr" width="5" height="5" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
      <rect width="5" height="5" fill="var(--carta)" /><rect width="2" height="5" fill="var(--ink)" />
    </pattern>
  </defs>
  <g bind:this={gruppo}>
    {#each REGIONI as r (r.id)}
      {@const c = classe[r.id]}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <path
        id="{pre}-{r.id}" class="reg" class:sola={solaLettura} d={r.d} data-reg={r.id}
        style:--riempi={tinta ? (tinta[r.id] ?? `url(#${pre}-tr)`) : c === null || c === undefined ? `url(#${pre}-tr)` : `var(--classe-${c})`}
        role="radio" aria-checked={scelta === r.id} aria-label={nomeAccessibile(r.id, r.nome)}
        tabindex={fermata === r.id ? 0 : -1}
        onkeydown={(e) => tasto(e, r.id)}
        onclick={() => !solaLettura && onscelta(r.id)}
        onfocus={(e) => { fuoco = r.id; daTastiera = e.currentTarget.matches(":focus-visible"); }}
        onblur={() => (daTastiera = false)}
      ></path>
    {/each}
  </g>
  <!-- gli anelli in coda: in SVG non c'è z-index, e il contorno di una regione finirebbe sotto le vicine; i segni ancora dopo, perché l'alone non li copra -->
  {#if scelta}
    <use href="#{pre}-{scelta}" class="alone" /><use href="#{pre}-{scelta}" class="anello" />
  {/if}
  {#if fuoco && daTastiera}
    <use href="#{pre}-{fuoco}" class="alone" /><use href="#{pre}-{fuoco}" class="fuoco" />
  {/if}
  <g aria-hidden="true" class="segni">
    {#each REGIONI as r (r.id)}
      {@const l = segni[r.id] ?? []}
      {#each l as s, i (i)}
        <g transform="translate({r.ancora[0] + x0(l.length, i)} {r.ancora[1]})" class="segno"><FormaSegno {s} /></g>
      {/each}
    {/each}
  </g>
</svg>

<style>
  svg { width: 100%; height: 100% }
  svg.piccola { height: 300px; width: auto }
  /* fill, stroke e spessore passano da variabili: <use> le ridefinisce e la copia dell'anello le legge */
  .reg { fill: var(--ov, var(--riempi)); stroke: var(--tr, var(--ink)); stroke-width: var(--sp, 1px); vector-effect: non-scaling-stroke; cursor: pointer }
  .reg:hover { --sp: 2.5px }
  .reg.sola { cursor: default }
  .reg:focus { outline: none }
  .segni { pointer-events: none }
  use { --ov: none; pointer-events: none }
  .alone { --tr: var(--carta); --sp: 6px }
  .anello { --tr: var(--biro); --sp: 3.5px }
  .fuoco { --tr: var(--ink); --sp: 2px; stroke-dasharray: 4 3 }
</style>
