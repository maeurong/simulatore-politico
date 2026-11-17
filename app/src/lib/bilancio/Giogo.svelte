<script lang="ts">
  // Il giogo (storia 124): la stima della crescita della spesa netta contro il tetto UE, a tutta pagina; accanto il costo sulla bozza.
  import Rifiuto from "$lib/componenti/Rifiuto.svelte";
  import { numero, segnato, t } from "$lib/testi/t";
  import type { RiepilogoBilancio } from "$lib/tipi/RiepilogoBilancio";
  import { distanzaTetto, euroSegnato } from "./bilancio";

  let { riepilogo, rifiuto, inVolo }: { riepilogo: RiepilogoBilancio | null; rifiuto: string | null; inVolo: boolean } = $props();

  const W = 800, H = 34;
  // la scala del prototipo 22: il tetto a 1/1,6 della larghezza, la barra si allunga se lo supera; lo zero entra se la spesa cala
  const g = $derived.by(() => {
    if (!riepilogo) return null;
    const c = riepilogo.crescita_spesa_netta, tetto = riepilogo.tetto;
    const hi = Math.max(tetto * 1.6, c * 1.15, 1), lo = Math.min(0, c * 1.15);
    const x = (v: number) => ((v - lo) / (hi - lo)) * W;
    return { a: x(tetto), z: x(0), c: x(c), oltre: c > tetto };
  });
</script>

<div class="giogo" class:vecchio={inVolo}>
  {#if rifiuto}
    <div class="rif"><Rifiuto testo={rifiuto} /></div>
  {:else if riepilogo && g}
    <div>
      <svg width="100%" height={H} viewBox="0 0 {W} {H}" preserveAspectRatio="none" aria-hidden="true">
        <line x1="0" x2={W} y1={H / 2} y2={H / 2} stroke="var(--grigio)" vector-effect="non-scaling-stroke" />
        <rect x={g.a} y={H / 2 - 5} width={W - g.a} height="10" fill="var(--rosso)" opacity=".18" />
        <rect x={Math.min(g.z, g.c)} y={H / 2 - 3} width={Math.abs(g.c - g.z)} height="6" fill={g.oltre ? "var(--rosso)" : "var(--ink)"} />
        <line x1={g.a} x2={g.a} y1="1" y2={H - 1} stroke="var(--rosso)" stroke-width="2" vector-effect="non-scaling-stroke" />
        <line x1={g.z} x2={g.z} y1="6" y2={H - 6} stroke="var(--ink)" vector-effect="non-scaling-stroke" />
      </svg>
      <div class="l"><span>{t("bilancio.giogo_spesa", { n: segnato(riepilogo.crescita_spesa_netta, 1) })}</span><span class="male">{t("bilancio.giogo_tetto", { n: numero(riepilogo.tetto, 1) })}</span></div>
      <span class="sec">{distanzaTetto(riepilogo)}</span>
    </div>
    <div class="costi">
      <div><div class="k">{t("bilancio.spesa_in_piu")}</div><div class="v">{euroSegnato(riepilogo.costo_spesa)}</div></div>
      <div><div class="k">{t("bilancio.gettito_perso")}</div><div class="v">{euroSegnato(riepilogo.costo_entrate)}</div></div>
    </div>
  {/if}
</div>

<style>
  /* il rifiuto sta dove stava il giogo, alla stessa altezza: entrate e spesa sotto non saltano */
  .giogo { border-bottom: var(--doppio); padding-bottom: 8px; display: grid; grid-template-columns: minmax(0, 1fr) auto; gap: 4px 20px; align-items: end; min-height: 80px }
  .giogo.vecchio > * { opacity: .55 }
  .l { display: flex; justify-content: space-between; gap: 8px; font-size: var(--c-min); color: var(--grigio) }
  .costi { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 4px 10px; min-width: 300px }
  .k { font-size: var(--c-min); color: var(--grigio) }
  .v { font-weight: 700; font-size: var(--c-lead) }
  .rif { grid-column: 1 / -1; align-self: center; border-left: 3px solid var(--rosso); padding: 2px 8px }
</style>
