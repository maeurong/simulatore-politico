<script lang="ts">
  // La banda in testa alla prima pagina dopo «avanti fino a novità»: quante chiusure, perché si è fermata, dove sono le pagine saltate.
  import { t } from "$lib/testi/t";
  import type { Fermata } from "$lib/tipi/Fermata";
  import type { Vista } from "$lib/tipi/Vista";
  import { fraseFermata } from "./esito";

  let { vista, corsa }: { vista: Vista; corsa: { chiusure: number; da: number; fermate: Fermata[] } } = $props();
</script>

<div class="banda" role="status">
  <!-- senza fermate (corsa interrotta da un guasto, partita finita) la banda dice solo le chiusure fatte; il guasto sta nella cornice -->
  {#if corsa.fermate.length}
    {t("prima.banda", { n: corsa.chiusure, da: corsa.da, fermate: corsa.fermate.map((f) => fraseFermata(f, vista)).join("; ") })}
  {:else}
    {t("prima.banda_senza_fermata", { n: corsa.chiusure, da: corsa.da })}
  {/if}
  <!-- il più vecchio dei saltati è il turno da cui si è partiti; con una sola chiusura non c'è niente di saltato -->
  {#if corsa.chiusure > 1}{t("prima.banda_registro")} <a href="/partita/registro?turno={corsa.da}">{t("prima.banda_turno", { n: corsa.da })}</a>{/if}
</div>

<style>
  .banda { background: var(--biro); color: var(--carta); padding: 4px 8px; margin-bottom: 8px; font-size: var(--c-sec); font-weight: 700; letter-spacing: .04em }
  :global(:root.grave) .banda { background: var(--ink) }
  a { color: inherit; text-decoration: underline; text-underline-offset: 2px }
  a:focus-visible { outline: 2px solid var(--carta); outline-offset: 2px }
</style>
