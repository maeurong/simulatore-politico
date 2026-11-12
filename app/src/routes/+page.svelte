<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { Vista } from "$lib/tipi/Vista";

  // Schermata provvisoria fino al piano 11: solo la Vista, nessuna logica di gioco.
  const trimestri = ["gennaio-marzo", "aprile-giugno", "luglio-settembre", "ottobre-dicembre"];

  let vista = $state<Vista | null>(null);
  let errore = $state("");
  let occupato = $state(false);

  async function chiama(comando: "chiudi_turno") {
    occupato = true;
    try {
      vista = await invoke<Vista>(comando);
      errore = "";
    } catch (e) {
      errore = String(e);
    } finally {
      occupato = false;
    }
  }
</script>

<main>
  {#if vista}
    <h1>Turno {vista.turno} · {trimestri[vista.data.trimestre - 1]} {vista.data.anno}</h1>
    <p>Spread {vista.spread_pb} <abbr title="punti base">pb</abbr></p>
    <ul>
      {#each vista.dicasteri as dicastero (dicastero.id)}
        <li>{dicastero.nome}</li>
      {/each}
    </ul>
    <button onclick={() => chiama("chiudi_turno")} disabled={occupato}>Chiudi turno</button>
  {:else}
    <p>Nessuna partita in corso.</p>
  {/if}
  {#if errore}
    <p role="alert">{errore}</p>
  {/if}
</main>

<style>
  main {
    font: 1rem system-ui, sans-serif;
    padding: 1rem 2rem;
  }
</style>
