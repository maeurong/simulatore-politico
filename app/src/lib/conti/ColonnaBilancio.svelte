<script lang="ts">
  // Il bilancio in vigore (storia 118): spesa e perdita reale per dicastero, Irpef con la soglia reale, le altre aliquote, il fondo di riserva.
  import { numero, percentuale, t } from "$lib/testi/t";
  import { euro } from "$lib/governo/governo";
  import type { Vista } from "$lib/tipi/Vista";
  import { reale } from "./conti";

  let { vista, scelta = $bindable() }: { vista: Vista; scelta: string | null } = $props();

  const b = $derived(vista.bilancio);
  const nome = (id: string) => vista.dicasteri.find((d) => d.id === id)?.nome ?? id;
  // la barretta più lunga è 22 px: su 5 punti, o sulla perdita più grande, dicasteri e scaglioni, se la supera (prototipo 21)
  const scala = $derived(Math.max(5, ...[...b.dicasteri.map((d) => d.perdita_reale), ...b.perdita_reale_scaglioni].map(Math.abs)));
  const resto = $derived(b.fondo_riserva > 0 ? b.fondo_residuo / b.fondo_riserva : 0);
  const pc = (x: number) => percentuale(x, 0);
</script>

{#snippet perdita(p: number)}
  {@const r = reale(p)}
  {@const w = (Math.abs(p) / scala) * 22}
  <!-- sotto 2 px la barretta sembra un difetto di disegno: resta la cifra colorata -->
  <span class={r.tono}>{r.testo}</span>{#if r.tono !== "neutro" && w >= 2}<i class="pb {r.tono}" style:width="{w}px"></i>{/if}
{/snippet}

<section>
  <div class="capo">
    <h3 class="tit">{t("conti.bilancio", { anno: b.anno })}</h3>
    <span class="sec muto">{t(b.esercizio_provvisorio ? "conti.provvisorio" : "conti.in_vigore")}</span>
  </div>

  <div class="tab">
    <div class="h"><span>{t("conti.col_dicastero")}</span><span>{t("conti.col_spesa")}</span><span>{t("conti.col_reale")}</span></div>
    {#each b.dicasteri as d (d.id)}
      {@const k = `dic:${d.id}`}
      <button type="button" class="r" class:sel={scelta === k} aria-pressed={scelta === k} onclick={() => (scelta = k)}>
        <span class="n">{nome(d.id)}</span><span class="v">{t("conti.miliardi", { n: numero(d.spesa / 1e9, 1) })}</span><span class="v">{@render perdita(d.perdita_reale)}</span>
      </button>
    {/each}
  </div>

  <div class="tab">
    <div class="h"><span>{t("conti.col_irpef")}</span><span>{t("conti.col_aliquota")}</span><span>{t("conti.col_soglia")}</span></div>
    {#each b.aliquote.irpef as s, i (i)}
      {@const k = `irpef:${i}`}
      {@const p = b.perdita_reale_scaglioni[i]}
      <button type="button" class="r" class:sel={scelta === k} aria-pressed={scelta === k} onclick={() => (scelta = k)}>
        <span class="n">{s.fino_a === null ? t("conti.oltre") : t("conti.fino_a", { n: numero(s.fino_a) })}</span>
        <span class="v">{pc(s.aliquota)}</span>
        <span class="v">{#if p === undefined}<span class="muto">—</span>{:else}{@render perdita(p)}{/if}</span>
      </button>
    {/each}
  </div>
  <p class="sec muto">{t("conti.aliquote", { iva: pc(b.aliquote.iva), ires: pc(b.aliquote.ires), contributi: pc(b.aliquote.contributi) })}</p>

  <button type="button" class="r fondo" class:sel={scelta === "fondo"} aria-pressed={scelta === "fondo"} onclick={() => (scelta = "fondo")}>
    <span class="n"><b>{t("conti.fondo")}</b> · {b.anno}</span><span class="x">{euro(b.fondo_residuo)}</span>
    <span class="min muto riga">{t("conti.fondo_rimasti", { euro: euro(b.fondo_riserva) })}</span>
    <span class="barra riga"><i style:width="{resto * 100}%"></i></span>
  </button>
</section>

<style>
  .tab { display: grid; font-size: var(--c-sec) }
  .tab .h, .tab .r { grid-template-columns: minmax(0, 1fr) auto 76px }
  .h { display: grid; gap: 0 8px; padding: 2px 4px; border-bottom: var(--bordo); font-size: var(--c-min); font-weight: 700; color: var(--grigio) }
  .h span + span { text-align: right }
  .v { white-space: nowrap; text-align: right }
  .pb { display: inline-block; height: 7px; margin-left: 5px; vertical-align: middle; background: var(--rosso) }
  .pb.bene { background: var(--verde) }
  .fondo { margin-top: 4px }
  .riga { grid-column: 1 / -1 }
  .barra { display: block; height: 6px; margin-top: 3px; border: var(--filo); position: relative }
  .barra i { position: absolute; inset: 0 auto 0 0; background: var(--ink) }
</style>
