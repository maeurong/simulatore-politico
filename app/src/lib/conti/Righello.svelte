<script lang="ts">
  // Il righello di una situazione (prototipo 21): accensione rossa, spegnimento tratteggiato, isteresi a retino, il valore come pallino.
  import type { VoceSituazione } from "$lib/tipi/VoceSituazione";

  let { s, W = 240, H = 22 }: { s: VoceSituazione; W?: number; H?: number } = $props();
  const id = $props.id();

  const scala = $derived.by(() => {
    const vs = [s.valore, s.accensione, s.spegnimento];
    const pad = Math.max((Math.max(...vs) - Math.min(...vs)) * 0.5, Math.abs(s.accensione) * 0.08, 1);
    return { lo: Math.min(...vs) - pad, hi: Math.max(...vs) + pad };
  });
  const x = (v: number) => ((v - scala.lo) / (scala.hi - scala.lo)) * W;
  const a = $derived(x(s.accensione));
  const b = $derived(x(s.spegnimento));
  // la zona accesa sta dalla parte dell'accensione: a destra se la situazione si accende salendo
  const zona = $derived(s.accensione > s.spegnimento ? [a, W] : [0, a]);
</script>

<svg width="100%" height={H} viewBox="0 0 {W} {H}" preserveAspectRatio="none" aria-hidden="true">
  <defs>
    <pattern id="{id}-retino" width="4" height="4" patternUnits="userSpaceOnUse" patternTransform="rotate(45)"><rect width="1" height="4" fill="var(--ink)" opacity=".35" /></pattern>
  </defs>
  <line x1="0" x2={W} y1={H / 2} y2={H / 2} stroke="var(--grigio)" stroke-width="1" />
  <rect x={zona[0]} y={H / 2 - 4} width={zona[1] - zona[0]} height="8" fill="var(--rosso)" opacity=".18" />
  <rect x={Math.min(a, b)} y={H / 2 - 4} width={Math.abs(a - b)} height="8" fill="url(#{id}-retino)" />
  <line x1={a} x2={a} y1="2" y2={H - 2} stroke="var(--rosso)" stroke-width="2" />
  <line x1={b} x2={b} y1="5" y2={H - 5} stroke="var(--ink)" stroke-width="1.5" stroke-dasharray="2 2" />
  <circle cx={x(s.valore)} cy={H / 2} r="5" fill={s.accesa ? "var(--rosso)" : "var(--ink)"} stroke="var(--carta)" stroke-width="1.5" />
</svg>

<style>
  svg { overflow: visible }
</style>
