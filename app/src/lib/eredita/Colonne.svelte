<script lang="ts">
  // Le tre colonne dell'ultima edizione: le elezioni, il Paese che lasci, le leggi che restano (30-eredita.prototype.html, .col3).
  // Le leggi portano l'etichetta solo dalla Vista (schema 19); riletta dall'albo, solo il nome.
  import Elezioni from "$lib/edicola/Elezioni.svelte";
  import Indicatori from "$lib/edicola/Indicatori.svelte";
  import { t } from "$lib/testi/t";
  import type { Edizione } from "./eredita";

  let { edizione }: { edizione: Edizione } = $props();
</script>

<div class="col3">
  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.le_elezioni")}</h3></div>
    <Elezioni elezioni={edizione.elezioni} nome={edizione.nome} />
  </section>

  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.il_paese")}</h3><span class="dx min muto">{t("eredita.contro_zero")}</span></div>
    <Indicatori indicatori={edizione.indicatori} />
  </section>

  <section>
    <div class="titsez"><h3 class="tit">{t("eredita.le_leggi")}</h3><span class="dx min muto">{edizione.leggi.length}</span></div>
    {#if edizione.leggi.length}
      <ul class="leggi">
        {#each edizione.leggi as l (l.nome)}
          <li><b>{l.nome}</b>{#if l.legge}<span class="min muto">{l.legge}</span>{/if}</li>
        {/each}
      </ul>
    {:else}
      <p class="sec muto">{t("eredita.nessuna_legge")}</p>
    {/if}
  </section>
</div>

<style>
  .col3 { --pad-ind: 8px; --el-primo-filo: 0; --el-primo-pad: 0; display: grid; grid-template-columns: minmax(0, 1fr) minmax(0, 1.3fr) minmax(0, .9fr); gap: 0 var(--canale) }
  .col3 > section { border-left: var(--filo); padding-left: var(--canale); min-width: 0 }
  .col3 > section:first-child { border-left: 0; padding-left: 0 }
  .titsez { display: flex; align-items: baseline; gap: 12px; border-bottom: var(--forte); padding-bottom: 4px; margin-bottom: 6px }
  .titsez .dx { margin-left: auto }
  .tit { font-family: var(--tit); font-variation-settings: var(--wonk); font-weight: 700; line-height: 1.1; text-wrap: balance; font-size: var(--c-riq) }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }

  .leggi { list-style: none; display: grid }
  .leggi li { padding: 5px 0; border-top: var(--filo); font-size: var(--c-sec); display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 2px 10px }
  .leggi li:first-child { border-top: 0 }
</style>
