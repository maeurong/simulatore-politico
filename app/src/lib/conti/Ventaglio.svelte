<script lang="ts">
  // Il ventaglio di una grandezza prevista dal centro studi (prototipo 21): il centro per trimestre, la fascia di una e di due sigma.
  import { t, type Chiave } from "$lib/testi/t";
  import type { Trimestre } from "$lib/tipi/Trimestre";
  import type { VocePrevisione } from "$lib/tipi/VocePrevisione";

  let { previsione, k, W = 170, H = 58 }: { previsione: VocePrevisione[]; k: "pil" | "disoccupazione" | "inflazione" | "debito"; W?: number; H?: number } = $props();

  const c = $derived(previsione.map((p) => p[k].centro));
  const s = $derived(previsione.map((p) => p[k].sigma));
  const lo = $derived(Math.min(...c.map((v, i) => v - 2 * s[i])));
  const hi = $derived(Math.max(...c.map((v, i) => v + 2 * s[i])));
  const x = (i: number) => 8 + (i * (W - 16)) / Math.max(1, c.length - 1);
  const y = (v: number) => H - ((v - lo) / (hi - lo || 1)) * H;
  const banda = (m: number) => [...c.map((v, i) => `${x(i)},${y(v + m * s[i])}`), ...c.map((v, i) => `${x(i)},${y(v - m * s[i])}`).reverse()].join(" ");
  const breve = (d: Trimestre) => t(`conti.trim_${d.trimestre}` as Chiave, { anno: d.anno });
</script>

{#if c.length}
  <svg width="100%" height={H + 14} viewBox="0 0 {W} {H + 14}" aria-hidden="true">
    <polygon points={banda(2)} fill="var(--ink)" opacity=".08" />
    <polygon points={banda(1)} fill="var(--ink)" opacity=".18" />
    <polyline points={c.map((v, i) => `${x(i)},${y(v)}`).join(" ")} fill="none" stroke="var(--ink)" stroke-width="2" />
    {#each c as v, i (i)}<circle cx={x(i)} cy={y(v)} r="2.5" fill="var(--ink)" />{/each}
    <text x={x(0)} y={H + 12} font-size="10" fill="var(--grigio)">{breve(previsione[0].data)}</text>
    <text x={x(c.length - 1)} y={H + 12} font-size="10" text-anchor="end" fill="var(--grigio)">{breve(previsione[previsione.length - 1].data)}</text>
  </svg>
{/if}

<style>
  svg { display: block; overflow: visible }
</style>
