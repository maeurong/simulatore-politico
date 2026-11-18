<script lang="ts">
  // Fascia: poche classi con un nome (prototipo 08, «fascia»). Le caselle sono decorazione: il nome accanto è il testo.
  /** `piene` su `su` caselle; `filo`: il filo rosso alla posizione `filo` (in caselle, anche frazionaria), o a ognuna di quelle elencate; `rosso`: sotto la soglia. */
  let { piene, su, filo, rosso = false, nome }: { piene: number; su: number; filo?: number | number[]; rosso?: boolean; nome: string } = $props();
</script>

<span class="fascia">
  <span class="cas" class:inp={rosso} aria-hidden="true">
    {#each { length: su }, i (i)}<i data-tagli-ignora class:on={i < piene}></i>{/each}
    {#each [filo ?? []].flat() as f, j (j)}<b data-tagli-ignora style:left="{f * 14 - 1}px"></b>{/each}
  </span>
  {nome}
</span>

<style>
  .fascia { display: inline-flex; align-items: center; gap: 6px; white-space: nowrap }
  .cas { position: relative; display: flex; gap: 2px; color: var(--ink) }
  .cas.inp { color: var(--rosso) }
  .cas i { position: relative; width: 12px; height: 8px; border: 1.5px solid currentColor; flex: none }
  .cas i.on { background: currentColor }
  .cas b { position: absolute; top: -4px; bottom: -4px; width: 2px; background: var(--rosso) }
</style>
