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
  /* il retino solo sotto il disegno, come nel 16: il riquadro del disegno si ferma ai 300 px della tela */
  .disegno { width: 100%; max-width: 300px; justify-self: center; background: var(--retino-vignetta) }
  .disegno :global(svg) { display: block; width: 100%; height: auto }
  figcaption { font-size: var(--c-sec); text-align: center; font-weight: 700 }
</style>
