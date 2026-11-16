<script lang="ts">
  // Lo spread per turno (prototipo 21) con la linea delle aste scoperte alla soglia della Vista; un punto solo non fa una linea.
  // la scritta della soglia sta a sinistra: a destra c'è l'ultimo punto, che con lo spread alto le passerebbe sopra
  import { numero, t } from "$lib/testi/t";
  import type { PuntoSerie } from "$lib/tipi/PuntoSerie";

  let { serie, soglia }: { serie: PuntoSerie[]; soglia: number } = $props();
  const W = 260, H = 56;

  const sr = $derived(serie.map((p) => p.spread_pb));
  // la soglia resta dentro il grafico anche con lo spread basso, con un po' d'aria sopra
  const cima = $derived(Math.max(soglia, ...sr) * 1.12);
  const x = (i: number) => (i / (sr.length - 1)) * W;
  const y = (v: number) => H - (v / cima) * H;
</script>

{#if sr.length < 2}
  <span class="sec muto">{t("conti.spread_da_qui")}</span>
{:else}
  <svg width="100%" height={H + 14} viewBox="0 -4 {W} {H + 18}" role="img" aria-label={t("conti.spread_aria")}>
    <line x1="0" x2={W} y1={y(soglia)} y2={y(soglia)} stroke="var(--rosso)" stroke-dasharray="4 3" />
    <text x="0" y={y(soglia) - 3} font-size="10" fill="var(--rosso)">{t("conti.aste", { n: numero(soglia) })}</text>
    <line x1="0" x2={W} y1={H} y2={H} stroke="var(--grigio)" />
    <polyline points={sr.map((v, i) => `${x(i)},${y(v)}`).join(" ")} fill="none" stroke="var(--ink)" stroke-width="2" />
    <circle cx={W} cy={y(sr[sr.length - 1])} r="3.5" fill="var(--ink)" />
    <text x="0" y={H + 12} font-size="10" fill="var(--grigio)">{t("edicola.asse_turno", { n: serie[0].turno })}</text>
    <text x={W} y={H + 12} text-anchor="end" font-size="10" fill="var(--grigio)">{t("edicola.asse_turno", { n: serie[serie.length - 1].turno })}</text>
  </svg>
{/if}

<style>
  svg { overflow: visible }
  .sec { display: block }
</style>
