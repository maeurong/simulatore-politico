<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import type { Vista } from "$lib/tipi/Vista";

  // Schermata provvisoria fino al piano 11: solo la Vista, nessuna logica di gioco.
  const trimestri = ["gennaio-marzo", "aprile-giugno", "luglio-settembre", "ottobre-dicembre"];

  let vista = $state<Vista | null>(null);
  let errore = $state("");

  async function chiama(comando: "nuova_partita" | "chiudi_turno") {
    try {
      vista = await invoke<Vista>(comando);
      errore = "";
    } catch (e) {
      errore = String(e);
    }
  }

  onMount(() => chiama("nuova_partita"));
</script>

<main>
  {#if vista}
    <h1>Turno {vista.turno} · {trimestri[vista.data.trimestre - 1]} {vista.data.anno}</h1>
    <ul>
      {#each vista.dicasteri as dicastero (dicastero.id)}
        <li>{dicastero.nome}</li>
      {/each}
    </ul>
    <button onclick={() => chiama("chiudi_turno")}>Chiudi turno</button>
  {/if}
  {#if errore}
    <p role="alert">{errore}</p>
  {/if}
</main>

<style>
  main {
    font: 16px system-ui, sans-serif;
    padding: 1rem 2rem;
  }
</style>
