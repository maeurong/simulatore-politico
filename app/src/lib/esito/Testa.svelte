<script lang="ts">
  // La voce di testa; a lato il risultato a barre nella pagina del voto (spec:408), la vignetta se il tipo ha una scena; altrimenti a tutta larghezza.
  import type { Vista } from "$lib/tipi/Vista";
  import type { VoceEsito } from "$lib/tipi/VoceEsito";
  import { scenaDi } from "./esito";
  import Risultato from "./Risultato.svelte";
  import Vignetta from "./Vignetta.svelte";
  import Voce from "./Voce.svelte";

  type Props = { vista: Vista; voce: VoceEsito; titolo: string | null; rivelata: boolean; nuova: boolean };
  let { vista, voce, titolo, rivelata, nuova }: Props = $props();
  const risultato = $derived(voce.tipo === "risultato_del_voto" && vista.elezioni.length > 0);
  const scena = $derived(scenaDi(vista, voce));
</script>

<div class={["apertura", { sola: !risultato && !scena }]}>
  <Voce {vista} {voce} {titolo} {rivelata} {nuova} testa />
  {#if risultato || scena}
    <div class={["lato", { attesa: !rivelata, nuova: rivelata && nuova }]}>
      {#if risultato}<Risultato {vista} />{:else if scena}<Vignetta {scena} {voce} />{/if}
    </div>
  {/if}
</div>

<style>
  .apertura { display: grid; grid-template-columns: minmax(0, 1fr) 30%; gap: 24px; padding: 10px 0 9px; border-bottom: 1px solid var(--ink) }
  .apertura.sola { grid-template-columns: minmax(0, 1fr) }
  .lato { min-width: 0 }
  .attesa { visibility: hidden }
  .nuova { animation: entra calc(520ms * var(--k)) var(--e-work) both }
  @keyframes entra { from { opacity: 0 } to { opacity: 1 } }
</style>
