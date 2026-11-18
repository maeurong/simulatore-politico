<script lang="ts">
  // Profilo verso destra al tratto, figura campita di carta (prototipo 32, variante 5). Il tono grave lo gestisce Faccia, che
  // al posto del disegno mette le iniziali. Genere: le donne hanno ciglia, labbra, naso all'85% e, dai bit dell'hash, orecchino e cappello; assente = uomo. Dicastero: l'oggetto in alto a destra, solo se `dicastero` è un id noto (i leader hanno la coccarda).
  import { leveDi, type Leve, type Ruolo } from "./caricatura";
  import type { Genere } from "$lib/tipi/Genere";
  import { FANTASIE, OGGETTI, TRATTI, profilo } from "./kit";

  let { ruolo, partito, nome, genere, leve, misura, fondo = "carta", dicastero }: {
    ruolo: Ruolo; partito: string; nome: string; genere?: Genere; leve?: Leve; misura: 34 | 46 | 62 | 84; fondo?: "carta" | "giallo" | "rosso"; dicastero?: string;
  } = $props();

  const id = $props.id();
  const l = $derived(leve ?? leveDi(ruolo, partito, nome));
  const donna = $derived(genere === "donna");
  const p = $derived(profilo(l, donna));
  const fantasia = $derived(FANTASIE[l.colletto]);
</script>

<svg class={["car", fondo !== "carta" && fondo]} width={misura} height={misura} viewBox="0 0 64 64" role={nome ? "img" : undefined} aria-label={nome || undefined} aria-hidden={nome ? undefined : "true"}>
  {#if fondo !== "carta"}<rect width="64" height="64" class={fondo} />{/if}
  <path class="carta" d="M2 64C3 56 10 53 20 53L42 53C52 53 58 57 61 64Z" />
  <path class="carta" id="{id}-s" d={p.sagoma} />
  <clipPath id="{id}-c"><use href="#{id}-s" /></clipPath>
  <path class="terra" clip-path="url(#{id}-c)" d={p.ombre} />
  <path class="ink" transform="scale(.1)" d={p.pennino} />
  {#if donna}<path class="ink" transform={TRATTI.naso} d={p.naso} />{/if}
  <path class="filo" d={p.filo} /><path class="sottile fine" d="M27 30.2Q25.4 32 27.2 33.6" />
  {#if l.occhio}
    <path class="carta ink-l" d="M35.2 22.8Q38.4 19.4 41.2 21.3Q40.6 24.6 35.2 22.8Z" /><circle class="ink" cx="39.3" cy="22.2" r="1.45" />
  {:else}
    <circle class="carta ink-l" cx="38.2" cy="22.6" r="2.7" style:stroke-width="1.4" /><circle class="ink" cx="39.4" cy="22.6" r="1.45" />
  {/if}
  {#if donna}
    <path class="sottile ci" d={TRATTI.ciglia[l.occhio]} />
    <path class="rosso lc" d={TRATTI.labbra} /><path class="rosso li" d={TRATTI.labbra} /><path class="sottile bo" d={TRATTI.bocca} />
    {#if l.orecchino}<path class="sottile fine" d={TRATTI.orecchino.filo} /><path class="giallo ink-l" d={TRATTI.orecchino.gemma} />{/if}
  {:else}
    <path class="sottile" d="M44 39.4L40.3 39.8" />
  {/if}
  {#if ruolo === "tecnico"}
    <path class="coll grigio" d="M21 51L38 49L40.5 55.5L22 57Z" />
  {:else}
    <path class="coll" fill="url(#fantasia-{fantasia})" d="M21 51L38 49L40.5 55.5L22 57Z" />
  {/if}
  {#if donna && l.cappello}
    <defs><g id="{id}-h">{#each TRATTI.cappello as d}<path {d} />{/each}</g></defs>
    <use href="#{id}-h" class="ha" /><use href="#{id}-h" class="ink ink-l" />
    <path class="sottile na" d={TRATTI.nastro} />
  {/if}
  {#if ruolo === "leader"}
    <g transform="translate(55 55.5) scale(.95)"><circle r="8.4" class="carta" /><circle r="7.2" class="verde" /><circle r="4.6" class="carta" /><circle r="2.4" class="rosso" /></g>
  {:else if ruolo === "tecnico"}
    <g transform="translate(5 54.5)"><rect width="9.4" height="6.6" class="carta ink-l" style:stroke-width="1.2" /><rect x="1.8" y="1.8" width="5.8" height="1.8" class="biro" /><rect x="1.8" y="4.2" width="3.8" height="1.1" class="ink" /></g>
  {/if}
  {#if ruolo !== "leader" && dicastero && Object.hasOwn(OGGETTI, dicastero)}<g class="ogg" transform="translate(47.5 1.5) scale(.92)">{@html OGGETTI[dicastero]}</g>{/if}
</svg>

<style>
  /* :global dentro .car: gli elementi del disegno non portano la classe di Svelte, un centinaio di byte in meno per faccia */
  .car { display: block; flex: none }
  .car :global(.carta) { fill: var(--carta) } .car :global(.terra) { fill: var(--terra) } .car :global(.ink) { fill: var(--ink) }
  .car :global(.rosso) { fill: var(--rosso) } .car :global(.verde) { fill: var(--verde) } .car :global(.biro) { fill: var(--biro) } .car :global(.giallo) { fill: var(--giallo) }
  .car :global(.grigio) { fill: var(--grigio) }
  .car :global(.ink-l) { stroke: var(--ink); stroke-linejoin: round; stroke-width: 1.5 }
  .car :global(.filo), .car :global(.sottile) { fill: none; stroke: var(--ink); stroke-width: 1.4; stroke-linecap: round; stroke-linejoin: round }
  .car :global(.sottile) { stroke-width: 1.9 }
  .car :global(.sottile.fine) { stroke-width: 1.5 }
  .car :global(.sottile.ci) { stroke-width: 1.4 } .car :global(.sottile.bo) { stroke-width: 1.1 } .car :global(.sottile.na) { stroke: var(--carta); stroke-width: 2.2 }
  .car :global(.lc) { stroke: var(--carta); stroke-width: 2.6; stroke-linejoin: round } .car :global(.li) { stroke: var(--ink); stroke-width: .9; stroke-linejoin: round }
  .car :global(.ha) { fill: none; stroke: var(--carta); stroke-width: 3.6; stroke-linejoin: round }
  .car.giallo { --d: var(--giallo) } .car.rosso { --f: var(--carta); --d: var(--rosso) }
  .car { --f: var(--ink); --d: var(--carta) } .car :global(.ogg) { fill: var(--f); stroke-width: 1.5 } .car :global(.od) { fill: var(--d) }
  .car :global(.sf), .car :global(.sd) { fill: none; stroke-linecap: round } .car :global(.s14) { stroke-width: 1.4 } .car :global(.sf) { stroke: var(--f) } .car :global(.sd) { stroke: var(--d) }
  .car :global(.coll) { stroke: var(--ink); stroke-width: 1.8; stroke-linejoin: round }
</style>
