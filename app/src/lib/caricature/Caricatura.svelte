<script lang="ts">
  // Profilo verso destra al tratto, figura campita di carta (prototipo 32, variante 5). Il tono grave lo gestisce Faccia, che
  // al posto del disegno mette le iniziali. Genere: fino alla 11g tutte senza segni femminili. Dicastero: dalla 11c.
  import { leveDi, type Leve, type Ruolo } from "./caricatura";
  import { COLLETTI, profilo } from "./kit";

  let { ruolo, partito, nome, leve, misura, fondo = "carta" }: {
    ruolo: Ruolo; partito: string; nome: string; leve?: Leve; misura: 34 | 46 | 62 | 84; fondo?: "carta" | "giallo" | "rosso";
  } = $props();

  const id = $props.id();
  const l = $derived(leve ?? leveDi(ruolo, partito, nome));
  const p = $derived(profilo(l));
  const colletto = $derived(ruolo === "tecnico" ? "grigio" : COLLETTI[l.colletto]);
</script>

<svg class="car" width={misura} height={misura} viewBox="0 0 64 64" role={nome ? "img" : undefined} aria-label={nome || undefined} aria-hidden={nome ? undefined : "true"}>
  {#if nome}<title>{nome}</title>{/if}
  {#if fondo !== "carta"}<rect width="64" height="64" class={fondo} />{/if}
  <path class="carta" d="M2 64C3 56 10 53 20 53L42 53C52 53 58 57 61 64Z" />
  <path class="carta" id="{id}-s" d={p.sagoma} />
  <clipPath id="{id}-c"><use href="#{id}-s" /></clipPath>
  <path class="terra" clip-path="url(#{id}-c)" d={p.ombre} />
  <path class="ink" transform="scale(.1)" d={p.pennino} />
  <path class="filo" d={p.filo} /><path class="sottile fine" d="M27 30.2Q25.4 32 27.2 33.6" />
  {#if l.occhio}
    <path class="carta ink-l" d="M35.2 22.8Q38.4 19.4 41.2 21.3Q40.6 24.6 35.2 22.8Z" /><circle class="ink" cx="39.3" cy="22.2" r="1.45" />
  {:else}
    <circle class="carta ink-l" cx="38.2" cy="22.6" r="2.7" style:stroke-width="1.4" /><circle class="ink" cx="39.4" cy="22.6" r="1.45" />
  {/if}
  <path class="sottile" d="M44 39.4L40.3 39.8" />
  <path class="coll {colletto}" d="M21 51L38 49L40.5 55.5L22 57Z" />
  {#if ruolo === "leader"}
    <g transform="translate(55 55.5) scale(.95)"><circle r="8.4" class="carta" /><circle r="7.2" class="verde" /><circle r="4.6" class="carta" /><circle r="2.4" class="rosso" /></g>
  {:else if ruolo === "tecnico"}
    <g transform="translate(5 54.5)"><rect width="9.4" height="6.6" class="carta ink-l" style:stroke-width="1.2" /><rect x="1.8" y="1.8" width="5.8" height="1.8" class="biro" /><rect x="1.8" y="4.2" width="3.8" height="1.1" class="ink" /></g>
  {/if}
</svg>

<style>
  /* :global dentro .car: gli elementi del disegno non portano la classe di Svelte, un centinaio di byte in meno per faccia */
  .car { display: block; flex: none }
  .car :global(.carta) { fill: var(--carta) } .car :global(.terra) { fill: var(--terra) } .car :global(.ink) { fill: var(--ink) }
  .car :global(.rosso) { fill: var(--rosso) } .car :global(.verde) { fill: var(--verde) } .car :global(.biro) { fill: var(--biro) } .car :global(.giallo) { fill: var(--giallo) }
  .car :global(.arancio) { fill: var(--arancio) } .car :global(.oliva) { fill: var(--oliva) } .car :global(.grigio) { fill: var(--grigio) }
  .car :global(.ink-l) { stroke: var(--ink); stroke-linejoin: round; stroke-width: 1.5 }
  .car :global(.filo), .car :global(.sottile) { fill: none; stroke: var(--ink); stroke-width: 1.4; stroke-linecap: round; stroke-linejoin: round }
  .car :global(.sottile) { stroke-width: 1.9 }
  .car :global(.sottile.fine) { stroke-width: 1.5 }
  .car :global(.coll) { stroke: var(--ink); stroke-width: 1.8; stroke-linejoin: round }
</style>
