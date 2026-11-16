<script lang="ts">
  // Una carta a due facce che gira con la molla (docs/ricerca/estetica/succo-per-interfacce-di-carte-e-numeri.md:204): il retro compare quando `girata`.
  // La rotazione parte al gesto (`anima`, vera solo dopo Ferma o Dirotta riusciti) o a «rigira»; nel resto la carta nasce già nello stato giusto, anche al rientro.
  // Col movimento spento il retro compare senza rotazione (classe `piatta`); nel tono grave chi usa il componente non passa `conRetro`.
  import type { Snippet } from "svelte";
  import { fattori, impostazioni, sistema } from "$lib/impostazioni.svelte";
  import { ruota } from "./governo";

  let { girata, anima, conRetro, chiave, fronte, retro }: { girata: boolean; anima: boolean; conRetro: boolean; chiave: string; fronte: Snippet; retro: Snippet } = $props();

  const moto = $derived(fattori(impostazioni, sistema.riduce));
  let giro = $state<HTMLElement>();
  let prima: { c: string; g: boolean } | null = null;
  let fatta = false;
  let corrente: Animation | undefined;
  $effect(() => {
    const c = chiave, g = girata, a = anima;
    if (giro) {
      if (g && a && !fatta) {
        fatta = true;
        corrente?.cancel();
        corrente = ruota(giro, 1, moto);
      } else if (prima && prima.c === c && prima.g && !g) {
        corrente?.cancel();
        corrente = ruota(giro, -1, moto);
      }
    }
    if (!a) fatta = false;
    prima = { c, g };
  });
</script>

<div class="g3d" class:piatta={!moto.muove}>
  <div class="giro" class:dietro={girata} bind:this={giro}>
    <div class="fronte" inert={girata}>{@render fronte()}</div>
    {#if conRetro}<div class="retro" inert={!girata}>{@render retro()}</div>{/if}
  </div>
</div>

<style>
  .g3d { perspective: 900px; height: 100% }
  .giro { position: relative; transform-style: preserve-3d; height: 100% }
  .giro.dietro { transform: rotateY(180deg) }
  .fronte, .retro { backface-visibility: hidden; -webkit-backface-visibility: hidden; display: grid }
  .fronte { height: 100% }
  .retro { position: absolute; inset: 0; transform: rotateY(180deg) }
  .piatta .giro.dietro { transform: none }
  .piatta .retro { transform: none }
  .piatta .giro.dietro .fronte, .piatta .giro:not(.dietro) .retro { visibility: hidden }
</style>
