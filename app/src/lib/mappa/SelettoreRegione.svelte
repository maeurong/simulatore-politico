<script lang="ts">
  // Il selettore del bersaglio: mappa piccola, lista a due colonne (l'equivalente da tastiera per le regioni sotto i 24 px), azione.
  import Pulsante from "$lib/componenti/Pulsante.svelte";
  import { percentuale, t } from "$lib/testi/t";
  import Mappa from "./Mappa.svelte";
  import { REGIONI } from "./regioni";

  let {
    valori, scelta = $bindable(null), azione, legenda, spento, formato = (x) => percentuale(x, 1), senzaAzioni = false, onconferma, onannulla,
  }: {
    valori: Record<string, number | null>;
    scelta: string | null;
    /** «Spendi in {regione}», già composta */
    azione: string;
    /** cosa dice la tinta, sotto la mappa */
    legenda?: string;
    /** il motivo per cui l'azione è spenta, oltre alla scelta mancante */
    spento?: string;
    formato?: (x: number) => string;
    /** chi lo usa tiene altrove i due pulsanti (una barra fissa in fondo al pannello) */
    senzaAzioni?: boolean;
    onconferma: () => void;
    onannulla: () => void;
  } = $props();

  const ordine = REGIONI.map((r) => r.id);
  const dato = (id: string) => valori[id] ?? null;
  const val = (id: string) => (dato(id) === null ? "—" : formato(dato(id)!));
</script>

<div class="sel">
  <Mappa {valori} {scelta} {ordine} nome={t("mappa.regioni")} piccola {formato} onscelta={(id) => (scelta = id)} />
  {#if legenda}<p class="min muto">{legenda}</p>{/if}
  <div class="lista">
    {#each REGIONI as r (r.id)}
      <button type="button" class="riga" class:sel={scelta === r.id} aria-pressed={scelta === r.id} onclick={() => (scelta = r.id)}>
        <span>{r.nome}</span><span class="x">{val(r.id)}</span>
      </button>
    {/each}
  </div>
  {#if scelta}<p class="sec"><b>{REGIONI.find((r) => r.id === scelta)?.nome}</b>: {val(scelta)}</p>{/if}
  {#if !senzaAzioni}<div class="az">
    <Pulsante forma="bordo" spento={scelta ? spento : t("mappa.scegli")} onclick={onconferma}>{azione}</Pulsante>
    <Pulsante forma="link" onclick={onannulla}>{t("componenti.annulla")}</Pulsante>
  </div>{/if}
</div>

<style>
  .sel { display: grid; gap: var(--s2); justify-items: start }
  .sec { font-size: var(--c-sec) }
  .min { font-size: var(--c-min) }
  .muto { color: var(--grigio) }
  .lista { display: grid; grid-template-columns: 1fr 1fr; gap: 0 var(--s4); width: 100%; font-size: var(--c-sec) }
  .riga { display: flex; justify-content: space-between; gap: var(--s2); padding: 2px var(--s1); border-top: var(--filo) }
  .riga:hover { background: var(--velo) }
  .riga.sel { background: var(--giallo); font-weight: 700 }
  :global(:root.grave) .riga.sel { background: var(--ink); color: var(--carta) }
  .x { font-variant-numeric: tabular-nums }
  .az { display: flex; gap: var(--s3); align-items: center }
</style>
