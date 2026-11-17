<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { apribile, casa, etichetta, fettaDi, statoDi } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import Cruscotto from "$lib/cruscotto/Cruscotto.svelte";
  import Opposizione from "$lib/opposizione/Opposizione.svelte";
  import Governo from "$lib/governo/Governo.svelte";
  import Aula from "$lib/aula/Aula.svelte";
  import Conti from "$lib/conti/Conti.svelte";
  import Bilancio from "$lib/bilancio/Bilancio.svelte";
  import Campagna from "$lib/campagna/Campagna.svelte";
  import Paese from "$lib/paese/Paese.svelte";
  import Partiti from "$lib/partiti/Partiti.svelte";
  import Pagina from "$lib/eredita/Pagina.svelte";
  import Trattativa from "$lib/trattativa/Trattativa.svelte";
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
    <main class="corpo" class:due={impostazioni} class:fisso={aperta === "governo" || aperta === "aula" || aperta === "eredita" || (aperta === "cruscotto" && (risponde(partita.vista) || statoDi(partita.vista) === "opposizione"))}>
      <div class="pagina">
        {#if aperta}
          {@const fetta = fettaDi(aperta, statoDi(partita.vista))}
          {#if aperta === "cruscotto" && fetta === null && statoDi(partita.vista) === "opposizione"}
            <Opposizione vista={partita.vista} />
          {:else if aperta === "cruscotto" && fetta === null}
            <Cruscotto vista={partita.vista} />
          {:else if aperta === "governo" && fetta === null}
            <Governo vista={partita.vista} />
          {:else if aperta === "aula" && fetta === null}
            <Aula vista={partita.vista} />
          {:else if aperta === "conti" && fetta === null}
            <Conti vista={partita.vista} />
          {:else if aperta === "bilancio" && fetta === null}
            <Bilancio vista={partita.vista} />
          {:else if aperta === "campagna" && fetta === null}
            <Campagna vista={partita.vista} />
          {:else if aperta === "paese" && fetta === null}
            <Paese vista={partita.vista} />
          {:else if aperta === "partiti" && fetta === null}
            <Partiti vista={partita.vista} />
          {:else if aperta === "trattativa" && fetta === null}
            <!-- la bozza è dell'incarico: al cambio (giocatore, guida) si rimonta -->
            {#key partita.vista.incarico}<Trattativa vista={partita.vista} />{/key}
          {:else if aperta === "eredita" && fetta === null}
            <Pagina vista={partita.vista} />
          {:else if fetta}
            <Segnaposto sezione={etichetta(aperta)} {fetta} />
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
  /* Governo, Aula e Cruscotto al governo: la pagina sta nel corpo e non scorre; l'album (o l'ordine del giorno) e il dettaglio scorrono ciascuno per conto suo */
  .corpo.fisso { overflow: hidden; grid-template-rows: minmax(0, 1fr) }
  .corpo.fisso .pagina { grid-template-rows: minmax(0, 1fr); align-content: stretch; min-height: 0 }
  .pagina { display: grid; gap: 10px; align-content: start }
</style>
