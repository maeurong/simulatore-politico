<script lang="ts">
  // Un piccolo multiplo della voce d'albo (13-edicola.prototype.html:262-279): un asse, un inchiostro;
  // con due linee la seconda è tratteggiata, così si distinguono senza colore.
  import { t } from "$lib/testi/t";
  import { tracce } from "./edicola";

  type Linea = { nome?: string; valori: number[]; tratteggiata?: boolean };
  type Props = {
    titolo: string;
    linee: Linea[];
    formato: (v: number) => string;
    /** un'etichetta per punto, «turno 6, aprile–giugno 2028» */
    etichette: string[];
    /** gli indici dei punti in cui si è votato */
    voti: number[];
    /** lo zero entra nell'asse e, se cade dentro, si disegna */
    zero?: boolean;
  };
  let { titolo, linee, formato, etichette, voti, zero = false }: Props = $props();

  const ALTO = 58;
  const P = { l: 2, r: 52, t: 6, b: 14 };
  let largo = $state(0);
  const pianoL = $derived(Math.max(0, largo - P.l - P.r));
  const pianoA = ALTO - P.t - P.b;
  const tr = $derived(tracce(linee.map((l) => l.valori), pianoL, pianoA, zero));
  const x = (i: number) => P.l + tr.x(i);
  const y = (v: number) => P.t + tr.y(v);
  const ultimo = $derived(tr.n ? linee.map((l) => formato(l.valori[tr.n - 1])).join(" / ") : "");

  // i nomi delle linee a destra, distanziati quando le ultime quote si toccano
  const nomi = $derived.by(() => {
    if (linee.length < 2 || !tr.n) return [];
    const voci = linee.map((l) => ({ nome: l.nome ?? "", y: y(l.valori[tr.n - 1]) + 4 })).sort((a, b) => a.y - b.y);
    for (let i = 1; i < voci.length; i++) voci[i].y = Math.max(voci[i].y, voci[i - 1].y + 11);
    return voci;
  });

  let sopra = $state<number | null>(null);
  let largoTip = $state(0);
  function muovi(e: PointerEvent) {
    const r = (e.currentTarget as SVGSVGElement).getBoundingClientRect();
    const fx = r.width ? ((e.clientX - r.left) / r.width) * largo - P.l : 0;
    sopra = tr.n < 2 ? 0 : Math.min(tr.n - 1, Math.max(0, Math.round((fx / pianoL) * (tr.n - 1))));
  }
  const tip = $derived(
    sopra === null ? "" : `${etichette[sopra] ?? ""}: ${linee.map((l) => (l.nome ? `${l.nome} ` : "") + formato(l.valori[sopra!])).join(" · ")}`,
  );
</script>

<div class="gr" bind:clientWidth={largo}>
  <div class="k"><span>{titolo}</span><b>{ultimo}</b></div>
  {#if pianoL && tr.n}
    <svg width={largo} height={ALTO} viewBox="0 0 {largo} {ALTO}" role="img" aria-label={t("edicola.grafico_aria", { titolo })}
      onpointermove={muovi} onpointerleave={() => (sopra = null)}>
      {#each voti as i, j (j)}
        <line x1={x(i)} x2={x(i)} y1={P.t - 4} y2={ALTO - P.b} stroke="var(--grigio)" stroke-dasharray="2 2" />
        <text x={x(i) + 3} y={P.t + 4}>{t("edicola.voto")}</text>
      {/each}
      {#if zero && tr.lo < 0 && tr.hi > 0}
        <line x1={P.l} x2={P.l + pianoL} y1={y(0)} y2={y(0)} stroke="var(--ink)" stroke-width="1" />
        <text x={P.l + 2} y={y(0) - 3}>{t("edicola.meta_piu_uno")}</text>
      {/if}
      {#each linee as l, j (j)}
        <polyline fill="none" stroke="var(--ink)" stroke-width="2" stroke-linejoin="round" stroke-dasharray={l.tratteggiata ? "5 3" : undefined}
          points={l.valori.map((v, i) => `${x(i).toFixed(1)},${y(v).toFixed(1)}`).join(" ")} />
        <!-- un punto solo (abbandono al turno zero): la linea non si vede, il punto sì -->
        {#if tr.n === 1}<circle cx={x(0)} cy={y(l.valori[0])} r="2.5" fill="var(--ink)" />{/if}
      {/each}
      {#each nomi as n (n.nome)}<text class="nome" x={P.l + pianoL + 4} y={n.y}>{n.nome}</text>{/each}
      <text x={P.l} y={ALTO - 2}>{t("edicola.asse_turno", { n: 0 })}</text>
      {#if tr.n > 1}<text x={P.l + pianoL} y={ALTO - 2} text-anchor="end">{tr.n - 1}</text>{/if}
      {#if sopra !== null}<line x1={x(sopra)} x2={x(sopra)} y1={P.t} y2={ALTO - P.b} stroke="var(--rosso)" stroke-width="1" />{/if}
    </svg>
    {#if sopra !== null}
      <div class="tip" aria-hidden="true" bind:offsetWidth={largoTip} style:left="{Math.max(0, Math.min(x(sopra), largo - largoTip))}px">{tip}</div>
    {/if}
  {/if}
</div>

<style>
  .gr { position: relative }
  .k { font-size: var(--c-min); letter-spacing: .1em; text-transform: uppercase; color: var(--grigio); display: flex; justify-content: space-between; gap: 8px }
  .k b { color: var(--ink); letter-spacing: 0; text-transform: none; font-size: var(--c-sec); white-space: nowrap }
  svg { overflow: visible }
  text { font: var(--c-min) var(--txt); fill: var(--grigio) }
  text.nome { fill: var(--ink) }
  .tip { position: absolute; top: -6px; z-index: 4; background: var(--ink); color: var(--carta); font-size: var(--c-min); padding: 3px 7px; pointer-events: none; white-space: nowrap }
</style>
