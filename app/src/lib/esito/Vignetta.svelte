<script lang="ts">
  // La vignetta della testa, riquadro del prototipo 16: la scena disegnata e la didascalia del tipo (piano 11f, decisione 1).
  // Gli SVG stanno inline perché currentColor e var(--carta) dentro <img> non arrivano; sono file nostri e statici.
  import { didascaliaDi } from "$lib/testi/pungolo";
  import type { VoceEsito } from "$lib/tipi/VoceEsito";
  import type { Scena } from "./esito";
  import aula from "./scene/aula.svg?raw";
  import cassa from "./scene/cassa.svg?raw";
  import emergenza from "./scene/emergenza.svg?raw";
  import giuramento from "./scene/giuramento.svg?raw";
  import lettera from "./scene/lettera.svg?raw";

  const SVG: Record<Scena, string> = { aula, cassa, emergenza, giuramento, lettera };
  let { scena, voce }: { scena: Scena; voce: VoceEsito } = $props();
</script>

<figure class="vignetta">
  <div class="disegno" aria-hidden="true">{@html SVG[scena]}</div>
  <figcaption>{didascaliaDi(voce)}</figcaption>
</figure>

<style>
  .vignetta { border: var(--forte); padding: 6px 8px; display: grid; gap: 4px; align-content: start; color: var(--ink) }
  /* il retino del 16 (riga 479): righe a 45° a un ottavo d'inchiostro, solo sotto il disegno */
  .disegno { background: repeating-linear-gradient(45deg, transparent 0 4.8px, rgb(31 26 23 / .12) 4.8px 6px) }
  .disegno :global(svg) { display: block; width: 100%; height: auto; max-height: 150px }
  figcaption { font-size: var(--c-sec); text-align: center; font-weight: 700 }
</style>
