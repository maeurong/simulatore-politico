<script lang="ts">
  // La tavola per poteri (37-scelte-11e-schema.prototype.html, svgB con riga · pettine · punta): la geometria sta in tavola.ts.
  // Ordine del Tab: ogni riga, poi i fili che muove. I tratti visibili stanno sotto i riquadri; sopra, una fascia trasparente per filo che prende clic e fuoco.
  // I tondi (piano 11f, lettura 8) stanno in cima, a metà del filo e sull'angolo della riga: si leggono, non si cliccano.
  import { t, type Chiave } from "$lib/testi/t";
  import type { Filo, Nodo } from "./schema";
  import { nomeFilo, nomeRiga, PASSO_TONDO, R_TONDO, stima, tavola, tondiSuiFili, type Misura, type Scelta } from "./tavola";

  /** `tutti`: i fili del trimestre, filtro o no, per i conteggi. */
  let { nodi, fili, tutti, tondi, scelta, pre, onscegli }: {
    nodi: Nodo[]; fili: Filo[]; tutti: Filo[]; tondi: { fili: Map<string, number[]>; nodi: Map<string, number[]> }; scelta: Scelta; pre: string; onscegli: (s: Scelta) => void;
  } = $props();

  let W = $state(0), H = $state(0);
  // i nomi si accorciano sulla larghezza vera del testo; a caratteri caricati la tavola si ridisegna
  let caratteri = $state(0);
  $effect(() => { document.fonts.ready.then(() => caratteri++); });
  const misura = $derived.by((): Misura => {
    void caratteri;
    const c = document.createElement("canvas").getContext("2d"), famiglia = getComputedStyle(document.documentElement).getPropertyValue("--txt");
    if (!c || !famiglia) return stima;
    return (s, px, grassetto) => { c.font = `${grassetto ? 700 : 400} ${px}px ${famiglia}`; return c.measureText(s).width; };
  });
  // ponytail: sotto i 200 px la tavola non si disegna; la pagina ha sempre più spazio (1280×800 a scala massima)
  const T = $derived(W > 200 && H > 200 ? tavola(nodi, fili, W, H, misura) : null);

  // in fuoco: il filo scelto coi suoi capi, o la riga scelta coi suoi fili e i loro capi
  const fuoco = $derived.by(() => {
    if (!scelta) return null;
    const L = scelta.tipo === "filo" ? fili.filter((f) => f.id === scelta.id) : fili.filter((f) => f.da === scelta.id || f.a === scelta.id);
    return new Set([...(scelta.tipo === "riga" ? [scelta.id] : []), ...L.flatMap((f) => [f.id, f.da, f.a])]);
  });
  const scelto = (tipo: "filo" | "riga", id: string) => scelta?.tipo === tipo && scelta.id === id;
  const tasto = (s: Scelta) => (e: KeyboardEvent) => {
    if (e.key === "Enter" || e.key === " ") { e.preventDefault(); onscegli(s); }
  };
</script>

<div class="tav" bind:clientWidth={W} bind:clientHeight={H}>
  {#if T}
    <svg class:fuoco={!!fuoco} width={W} height={T.altezza} viewBox="0 0 {W} {T.altezza}" role="group" aria-label={t("schema.tavola")}>
      <defs>
        {#each ["B", "R"] as c (c)}
          <marker id="{pre}-p{c}" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" markerUnits="userSpaceOnUse" orient="auto-start-reverse">
            <path d="M0,1 L10,5 L0,9 z" class="punta {c}" />
          </marker>
        {/each}
      </defs>
      {#each T.colonne as x, c (c)}
        <text class="col-t" {x} y="14">{t(`schema.col_${c}` as Chiave)}</text>
        <line class="col-l" x1={x} x2={x + T.cw} y1="20" y2="20" />
      {/each}
      <g aria-hidden="true">
        {#each T.tratti as { filo: f, d } (f.id)}
          <path class={["fi", { m: f.contrario, r: f.ritardo, c: f.cantiere, on: fuoco?.has(f.id) }]} {d} stroke-width={f.cantiere ? 1.5 : 2}
            marker-end="url(#{pre}-p{f.contrario ? 'R' : 'B'})" />
        {/each}
        {#each T.sbarre as s, i (i)}
          <path class="fi sbarra" d={s.d} stroke-width="3" marker-end={s.punta ? `url(#${pre}-pB)` : undefined} />
        {/each}
      </g>
      {#each T.righe as r (r.nodo.id)}
        {@const n = r.nodo}
        <g id="{pre}-n-{n.id}" class={["nd", n.tipo, { gov: n.alGoverno, tuo: n.tuo, spento: n.spento, scura: r.scura, on: fuoco?.has(n.id) }]}
          role="button" tabindex="0" aria-expanded={scelto("riga", n.id)} aria-controls="{pre}-det"
          aria-label={nomeRiga(n, tutti, tondi.nodi.get(n.id) ?? [])}
          onclick={() => onscegli({ tipo: "riga", id: n.id })} onkeydown={tasto({ tipo: "riga", id: n.id })}>
          <rect class="anello" x={r.x - 3} y={r.y - r.h / 2 - 3} width={r.w + 6} height={r.h + 6} rx={n.tipo === "presidente" ? r.h / 2 + 3 : n.tipo === "paese" || n.tipo === "situazione" ? 11 : 0} />
          <rect class="b" x={r.x} y={r.y - r.h / 2} width={r.w} height={r.h} rx={n.tipo === "presidente" ? r.h / 2 : n.tipo === "paese" || n.tipo === "situazione" ? 8 : 0} />
          <text class="t" x={r.x + 9} y={r.y - 3}>{r.nome}</text>
          <text class="s" x={r.x + 9} y={r.y + 12}>{r.sotto}</text>
          {#if r.etichetta}
            <text class={["et", { m: r.etichetta.contrario, scura: r.scura }]} x={r.x + r.w - 7} y={r.y + 12} text-anchor="end">{r.etichetta.testo}</text>
          {/if}
        </g>
        {#each r.uscenti as { filo: f, d } (f.id)}
          <path id="{pre}-f-{f.id}" class="fi-hit" {d} role="button" tabindex="0" aria-expanded={scelto("filo", f.id)} aria-controls="{pre}-det" aria-label={nomeFilo(f, tondi.fili.get(f.id) ?? [])}
            onclick={() => onscegli({ tipo: "filo", id: f.id })} onkeydown={tasto({ tipo: "filo", id: f.id })} />
        {/each}
      {/each}
      <g class="tondi" aria-hidden="true">
        {#each tondiSuiFili(T.tratti, tondi.fili, T.gap) as k (k.n)}
          <g class={["tnd", { m: k.m, on: fuoco?.has(k.filo) }]}><circle cx={k.x} cy={k.y} r={R_TONDO} /><text x={k.x} y={k.y + 4}>{k.n}</text></g>
        {/each}
        {#each T.righe as r (r.nodo.id)}
          {#each tondi.nodi.get(r.nodo.id) ?? [] as k, i (k)}
            {@const cx = r.x + r.w - 2 - i * PASSO_TONDO}
            <g class={["tnd", { on: fuoco?.has(r.nodo.id) }]}><circle {cx} cy={r.y - r.h / 2} r={R_TONDO} /><text x={cx} y={r.y - r.h / 2 + 4}>{k}</text></g>
          {/each}
        {/each}
      </g>
    </svg>
  {/if}
</div>

<style>
  .tav { min-width: 0; min-height: 0; overflow: hidden auto }
  svg { display: block }
  .col-t { font-family: var(--txt); font-size: 12px; letter-spacing: .1em; text-transform: uppercase; font-weight: 700; fill: var(--grigio) }
  .col-l { stroke: var(--grigio); stroke-width: 1 }

  .nd { cursor: pointer; outline: none }
  .nd text { font-family: var(--txt); fill: var(--ink) }
  .nd .t { font-size: 13px; font-weight: 700 }
  .nd .s { font-size: 12px; fill: var(--grigio) }
  .nd .b { fill: var(--carta); stroke: var(--ink); stroke-width: 1.5 }
  .nd.scura .b { fill: var(--ink) }
  .nd.scura .t { fill: var(--carta) }
  .nd.scura .s { fill: var(--carta); opacity: .85 }
  .nd.partito .b { stroke: var(--grigio) }
  .nd.partito.gov .b { stroke: var(--ink); stroke-width: 2 }
  .nd.partito.tuo .b { stroke: var(--rosso); stroke-width: 2 }
  .nd.paese .b { fill: var(--velo); stroke: var(--grigio) }
  .nd.situazione .b { stroke: var(--rosso); stroke-width: 2 }
  .nd.spento .b { stroke-dasharray: 4 3; stroke: var(--grigio); fill: var(--carta) }
  .nd.spento .t, .nd.spento .s { fill: var(--grigio); opacity: 1 }
  .anello { fill: none; stroke: none }
  .nd:focus-visible .anello { stroke: var(--biro); stroke-width: 2 }

  .fi { fill: none; stroke: var(--biro) }
  .fi.m { stroke: var(--rosso) }
  .fi.r { stroke-dasharray: 7 5 }
  .fi.c { stroke-dasharray: 2 4; stroke-linecap: round }
  .punta.B { fill: var(--biro) }
  .punta.R { fill: var(--rosso) }
  .fi-hit { fill: none; stroke: transparent; stroke-width: 12; cursor: pointer; outline: none }
  .fi-hit:focus-visible { stroke: var(--biro); stroke-opacity: .3 }

  /* l'etichetta corta del filo, nella riga di chi lo muove (decisione 1); gialla sui riquadri scuri, carta nel tono grave */
  .et { font-family: var(--txt); font-size: 12px; font-weight: 700; fill: var(--biro); paint-order: stroke; stroke: var(--carta); stroke-width: 4px; stroke-linejoin: round }
  .et.m { fill: var(--rosso) }
  .et.scura { fill: var(--giallo); stroke: none }
  :global(:root.grave) .et.scura { fill: var(--carta) }

  .tondi { pointer-events: none }
  .tnd circle { fill: var(--carta); stroke: var(--biro); stroke-width: 1.5 }
  .tnd text { font-family: var(--txt); font-size: 11px; font-weight: 700; fill: var(--biro); text-anchor: middle }
  .tnd.m circle { stroke: var(--rosso) }
  .tnd.m text { fill: var(--rosso) }

  .fuoco .nd, .fuoco .fi, .fuoco .tnd { opacity: .16 }
  .fuoco .fi.sbarra { opacity: .4 }
  .fuoco .on { opacity: 1 }
  .fuoco .fi.on { stroke-width: 3.5 }
</style>
