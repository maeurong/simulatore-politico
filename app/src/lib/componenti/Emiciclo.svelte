<script lang="ts">
  // L'emiciclo di una Camera (prototipo 19, forma D): seggi pieni per i sì sicuri, a retino fino al tetto della forbice, vuoti gli altri;
  // la riga rossa sulla maggioranza. Il nome accessibile dice la stessa frase della didascalia.
  import { t, numero } from "$lib/testi/t";

  let { totale, si, maggioranza, ramo }: { totale: number; si: [number, number]; maggioranza: number; ramo: "Camera" | "Senato" } = $props();

  const id = $props.id();
  const W = 560;
  const file = $derived(ramo === "Camera" ? 9 : 6);
  const raggio = $derived(ramo === "Camera" ? W / 90 : W / 62);
  const frase = $derived(t("aula.emiciclo", { ramo, si: si[0] === si[1] ? numero(si[0]) : `${numero(si[0])}–${numero(si[1])}`, n: numero(maggioranza) }));

  // i seggi su file concentriche, ordinati dall'angolo di sinistra a quello di destra
  const seggi = $derived.by(() => {
    const r0 = W * 0.2, r1 = W * 0.48;
    const raggi = Array.from({ length: file }, (_, i) => r0 + ((r1 - r0) * i) / (file - 1));
    const somma = raggi.reduce((a, b) => a + b, 0);
    const fuori: { a: number; x: number; y: number }[] = [];
    let resto = totale;
    raggi.forEach((r, i) => {
      const n = i === file - 1 ? resto : Math.round((totale * r) / somma);
      resto -= n;
      for (let j = 0; j < n; j++) {
        const a = Math.PI * (1 - (j + 0.5) / n);
        fuori.push({ a, x: W / 2 + r * Math.cos(a), y: W * 0.5 - r * Math.sin(a) });
      }
    });
    return fuori.sort((p, q) => q.a - p.a);
  });
  const am = $derived(Math.PI * (1 - maggioranza / totale));
  const riga = $derived({
    x1: W / 2 + W * 0.2 * 0.8 * Math.cos(am), y1: W * 0.5 - W * 0.2 * 0.8 * Math.sin(am),
    x2: W / 2 + (W * 0.48 + raggio * 2) * Math.cos(am), y2: W * 0.5 - (W * 0.48 + raggio * 2) * Math.sin(am),
  });
</script>

<figure class="emi">
  <svg viewBox="0 0 {W} {W * 0.56}" width="100%" role="img" aria-label={frase}>
    <defs>
      <pattern id="{id}-r" width="4" height="4" patternUnits="userSpaceOnUse" patternTransform="rotate(45)">
        <rect width="4" height="4" fill="var(--carta)" /><rect width="1.4" height="4" fill="var(--ink)" />
      </pattern>
    </defs>
    {#each seggi as s, i (i)}
      <circle cx={s.x.toFixed(1)} cy={s.y.toFixed(1)} r={raggio.toFixed(1)} stroke="var(--ink)" stroke-width="1"
        fill={i < si[0] ? "var(--ink)" : i < si[1] ? `url(#${id}-r)` : "var(--carta)"} />
    {/each}
    <line x1={riga.x1} y1={riga.y1} x2={riga.x2} y2={riga.y2} stroke="var(--rosso)" stroke-width="2.5" />
  </svg>
  <figcaption class="sec" aria-hidden="true">{frase}</figcaption>
</figure>

<style>
  .emi { min-width: 0 }
  figcaption { text-align: center; font-size: var(--c-sec) }
</style>
