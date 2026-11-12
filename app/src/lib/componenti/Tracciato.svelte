<script lang="ts">
  // Forma dal prototipo 31 (.scratch/v1-piano-11-ui/prototipi/31-tracciati.prototype.html), varianti di base.
  import { caselle, type Tracciato } from "$lib/tracciati";
  import { numero, t } from "$lib/testi/t";
  import NumeroFreccia from "./NumeroFreccia.svelte";

  /** Le caselle sono larghe --cw (9 px); chi le allarga ad almeno 12 px passa anche --ret: var(--retino). */
  let { tracciato: tr }: { tracciato: Tracciato } = $props();

  const pos = $derived(tr.valore !== null && tr.scala ? caselle(tr.valore, tr.scala) : null);
  // col valore il rimando è dello stacco (lo spread fino a 11g): caselle piene, senza tacche dello stacco
  const stacco = $derived(!tr.rimando);
  const rimando = $derived(tr.rimando ? t(tr.valore === null ? "tracciato.arriva" : "tracciato.stacco_arriva", { fetta: tr.rimando }) : undefined);
  const decimali = $derived(tr.nome.startsWith("tracciato.consenso") ? 1 : 0);
  const accanto = $derived(tr.ramo ?? (tr.nome === "tracciato.conti" ? t("tracciato.pb") : null));
</script>

<div class="trk" title={rimando}>
  <span class="k">{t(tr.nome)}</span>
  {#if pos && tr.valore !== null}
    <span class="cas" class:inp={tr.rosso} class:fermo={tr.grigio} aria-hidden="true">
      <b class="oltre" class:su={!tr.rosso}>{pos.sotto ? "◂" : ""}</b>
      {#each { length: 10 }, i (i)}
        <i data-tagli-ignora class:per={stacco && i < 3 && !tr.grigio} class:sog={stacco && i === 2} class:on={i < pos.piene} style:--i={i}></i>
      {/each}
      <b class="oltre su">{pos.sopra ? "▸" : ""}</b>
    </span>
    <span class="v" class:fermo={tr.grigio}>
      <NumeroFreccia
        valore={tr.valore}
        prima={tr.prima ?? undefined}
        formato={() => tr.testo}
        formatoVariazione={(x) => numero(x, decimali)}
        male={tr.scala?.rovesciata ? "sale" : "scende"} />
      {#if accanto}<small>{accanto}</small>{/if}{#if rimando}<span class="nascosto" data-tagli-ignora>{rimando}</span>{/if}
    </span>
  {:else}
    <span class="cas vuoto" aria-hidden="true"><b class="oltre"></b>{#each { length: 10 }, i (i)}<i data-tagli-ignora></i>{/each}<b class="oltre"></b></span>
    <span class="v fermo">{tr.testo}{#if rimando}<span class="nascosto" data-tagli-ignora>{rimando}</span>{/if}</span>
  {/if}
</div>

<style>
  .trk { display: grid; grid-template-columns: auto auto; gap: 0 8px; align-items: center; padding: 0 12px; border-left: var(--filo) }
  .trk:last-child { border-right: var(--filo) }
  .k { grid-column: 1 / -1; font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); white-space: nowrap }
  .v { display: flex; align-items: first baseline; font-size: var(--c-num); font-weight: 700; line-height: 1.1; white-space: nowrap }
  /* la variazione sotto il numero */
  .v :global(.nf) { display: inline-grid; justify-items: start }
  .v :global(.dl) { margin-left: 0 }
  .v small { font-size: var(--c-min); font-weight: 400; color: var(--grigio); margin-left: 3px }
  .v.fermo { color: var(--grigio) }

  .cas { display: flex; gap: 2px; align-items: center; color: var(--ink) }
  .cas i { position: relative; width: var(--cw, 9px); height: var(--ch, 14px); border: 1.5px solid var(--ink); flex: none }
  .cas i.per { background: var(--ret, none) }
  .cas i.sog { margin-right: 4px }
  /* il pieno copre anche il bordo: si accende con l'opacità, a scatti, da sinistra; si spegne da destra */
  .cas i::after {
    content: ""; position: absolute; inset: -1.5px; background: currentColor; opacity: 0;
    transition: opacity calc(var(--t-casella) * var(--k)) steps(1, jump-end) calc((9 - var(--i)) * var(--t-casella) * var(--k));
  }
  .cas i.on::after { opacity: 1; transition-delay: calc(var(--i) * var(--t-casella) * var(--k)) }
  :global(:root[data-movimento="spento"]) .cas i::after { transition-duration: 0s; transition-delay: 0s }
  .cas.inp { color: var(--rosso) }
  .cas.fermo { color: var(--grigio) }
  .cas.fermo i { border-color: var(--grigio) }
  .cas.vuoto i { border-style: dashed; border-color: var(--grigio) }
  .oltre { font-size: var(--c-min); line-height: 1; color: var(--rosso); width: 8px; text-align: center }
  .oltre.su { color: var(--grigio) }
</style>
