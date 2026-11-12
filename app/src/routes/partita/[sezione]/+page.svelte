<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import { t, type Chiave } from "$lib/testi/t";
  import { SEZIONI, casa, fettaDi, statoDi, type Sezione } from "$lib/ossatura";
  import { partita } from "$lib/partita.svelte";
  import Cornice from "$lib/cornice/Cornice.svelte";
  import Impostazioni from "$lib/cornice/Impostazioni.svelte";
  import Segnaposto from "$lib/componenti/Segnaposto.svelte";

  type Pagina = Sezione | "trattativa" | "eredita" | "schema";
  const PAGINE: readonly string[] = [...SEZIONI, "trattativa", "eredita", "schema"];

  const sezione = $derived(page.params.sezione ?? "");
  const nota = $derived(PAGINE.includes(sezione) ? (sezione as Pagina) : null);

  // senza partita si torna all'edicola; un nome ignoto torna alla casa
  $effect(() => {
    if (!partita.vista) goto("/", { replaceState: true });
    else if (!nota) goto(`/partita/${casa(partita.vista)}`, { replaceState: true });
  });

  let impostazioni = $state(false);
  function chiudiImpostazioni() {
    impostazioni = false;
    document.getElementById("apri-impostazioni")?.focus();
  }
</script>

{#if partita.vista && nota}
  {@const fetta = fettaDi(nota, statoDi(partita.vista))}
  <div class="partita">
    <Cornice vista={partita.vista} {sezione} {impostazioni} onimpostazioni={() => (impostazioni ? chiudiImpostazioni() : (impostazioni = true))} />
    <main class="corpo" class:due={impostazioni}>
      <div>
        {#if fetta}<Segnaposto sezione={t(`sommario.${nota.replace("-", "_")}` as Chiave)} {fetta} />{/if}
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
</style>
