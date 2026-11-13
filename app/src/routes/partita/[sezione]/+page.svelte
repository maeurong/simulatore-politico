<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { apribile, casa, etichetta, fettaDi, statoDi } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import Cruscotto from "$lib/cruscotto/Cruscotto.svelte";
  import Evento from "$lib/cruscotto/Evento.svelte";
  import { risponde } from "$lib/cruscotto/cruscotto";
  import Cornice from "$lib/cornice/Cornice.svelte";
  import Impostazioni from "$lib/cornice/Impostazioni.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";

  const sezione = $derived(page.params.sezione ?? "");
  const aperta = $derived(partita.vista && apribile(sezione, partita.vista) ? sezione : null);

  // senza partita si torna all'edicola; un nome ignoto, una sezione barrata o una casa d'altro stato tornano alla casa
  $effect(() => {
    if (!partita.vista) goto("/", { replaceState: true });
    else if (!aperta) goto(`/partita/${casa(partita.vista)}`, { replaceState: true, keepFocus: true });
  });

  let impostazioni = $state(false);
  function chiudiImpostazioni() {
    impostazioni = false;
    document.getElementById("apri-impostazioni")?.focus();
  }
</script>

<!-- la cornice resta montata anche per il fotogramma in cui la sezione aperta non è più apribile: il fuoco non cade su body -->
{#if partita.vista}
  <div class="partita">
    <Cornice vista={partita.vista} {sezione} {impostazioni} onimpostazioni={() => (impostazioni ? chiudiImpostazioni() : (impostazioni = true))} />
    <main class="corpo" class:due={impostazioni}>
      <div class="pagina">
        {#if aperta}
          {@const fetta = fettaDi(aperta, statoDi(partita.vista))}
          {#if aperta === "cruscotto" && fetta === null}
            <Cruscotto vista={partita.vista} />
          {:else}
            <!-- governo, crisi, opposizione: l'evento in testa, il resto del cruscotto arriva con la sua fetta (lettura 1) -->
            {#if aperta === "cruscotto"}
              {#each partita.vista.eventi as evento, i (i)}<Evento {evento} indice={i} puoRispondere={risponde(partita.vista)} />{/each}
            {/if}
            {#if fetta}<Segnaposto sezione={etichetta(aperta)} {fetta} />{/if}
          {/if}
        {/if}
      </div>
      {#if impostazioni}<Impostazioni onchiudi={chiudiImpostazioni} />{/if}
    </main>
  </div>
{/if}

<style>
  /* la cornice resta ferma; il corpo scorre in verticale quando non basta, e niente scorre di lato (storia 40) */
  .partita { position: fixed; inset: 0; display: flex; flex-direction: column; overflow: hidden }
  .corpo {
    flex: 1; min-height: 0; min-width: 0; overflow: hidden auto; padding: 12px var(--marg) 14px;
    display: grid; grid-template-columns: minmax(0, 1fr); gap: 10px var(--canale); align-content: start;
  }
  .corpo.due { grid-template-columns: minmax(0, 1fr) var(--lato) }
  .corpo > * { min-width: 0 }
  .pagina { display: grid; gap: 10px; align-content: start }
</style>
